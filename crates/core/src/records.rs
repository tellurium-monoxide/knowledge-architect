//! What the recorded entries hold, for the commands that print them.
//!
//! `show`, `issues` and `tripwires` all ask the same three questions of an entity: where it is
//! defined, what it says, and what points at it. This module answers them over the entity table
//! `path@core@src/entity.rs` builds, so a listing reads the same definition sites
//! every check resolves against and no command holds a second notion of what an entry is.
//!
//! **A file register's entry is its file; a heading register's entry is its section.** That is
//! the whole of the difference between the two shapes here: the body of the first is the file's
//! text, and the body of the second is the heading line through to the next heading at or above
//! its level. A Directory register's entry, a milestone, is its README's file.
//!
//! Nothing here reads the filesystem or spawns a process. The last-change column a listing
//! prints comes from `git::last_changed`, which the binary calls beside these; every function
//! here is a pure function of the model, so a test states its project as text.

use std::collections::BTreeMap;
use std::path::Path;

use crate::entity::{candidate, Anchors, Candidate, Entities, Kind, Site};
use crate::manifest::{Registers, Shape, ISSUE_REGISTER, TRIPWIRE_REGISTER};
use crate::model::{Document, Model};
use crate::scan::Observation;

/// One recorded entry, as a row and as a body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Record {
    pub anchor: String,
    pub id: String,
    /// The entry's title: a file entry's level-one heading, a heading entry's heading text
    /// without its slug. The id where the entry carries neither.
    pub title: String,
    /// Where the entry is defined.
    pub site: Site,
    /// The group subdirectory of a file entry, `None` at an instance's top level and for
    /// every heading entry.
    pub group: Option<String>,
    /// The entry whole: a file's text, or a heading's section.
    pub body: String,
    /// The value of the register's first declared metadata key, where the entry carries one.
    ///
    /// `None` covers both an entry that declares no value and one whose frontmatter was
    /// refused, which are the same thing to a listing and each a `registers` finding.
    pub metadata: Option<String>,
    /// Every reference to a guarded kind the entry's body carries, as written, in the order
    /// they appear. [`guarded`] says which kinds those are.
    pub guards: Vec<String>,
}

/// Whether a reference of this kind names something a tripwire guards.
///
/// A tripwire guards what is recorded as settled: a decision, a goal, or the entry of a
/// register a project declares, such as a reading of a specification. Every register kind but
/// two, then. An `issue` reference in a tripwire names what its response opens, and a
/// `tripwire` reference names another guard; neither is guarded. `path` is no register.
pub(crate) fn guarded(registers: &Registers, kind: &str) -> bool {
    kind != ISSUE_REGISTER && kind != TRIPWIRE_REGISTER && registers.by_name(kind).is_some()
}

/// Every entry of one register kind, in `(anchor, id)` order.
///
/// An entity whose definition site is in no walked document is skipped: the table holds it
/// because something defined it, and a row with no body would say the register holds an entry
/// nobody can read.
pub(crate) fn records(
    model: &Model,
    anchors: &Anchors,
    entities: &Entities,
    kind: &Kind,
) -> Vec<Record> {
    let by_path: BTreeMap<&Path, &Document> = model
        .documents()
        .iter()
        .map(|d| (d.rel.as_path(), d))
        .collect();
    let mut out = Vec::new();
    for (anchor_name, id, sites) in entities.of_kind(kind) {
        // The first site: a second definition of one id is a finding of its own, and a listing
        // that printed two rows for one entity would hide it behind a duplicate.
        let Some(site) = sites.first() else { continue };
        let Some(doc) = by_path.get(site.file.as_path()) else {
            continue;
        };
        let shape = anchors
            .by_name(anchor_name)
            .and_then(|a| anchors.home(a, kind))
            .map(|home| home.shape);
        let record = match shape {
            Some(Shape::File) => file_record(anchors, kind, anchor_name, id, site, doc),
            // A Directory entry is its README, whole; the directory is the entry, not a group.
            Some(Shape::Directory) => Record {
                group: None,
                ..file_record(anchors, kind, anchor_name, id, site, doc)
            },
            _ => heading_record(anchors, anchor_name, id, site, doc),
        };
        out.push(record);
    }
    out
}

/// A file register's entry: the file whole.
fn file_record(
    anchors: &Anchors,
    kind: &Kind,
    anchor: &str,
    id: &str,
    site: &Site,
    doc: &Document,
) -> Record {
    let dir = anchors
        .by_name(anchor)
        .and_then(|a| anchors.home(a, kind))
        .map(|home| home.dir);
    let group = dir.and_then(|dir| {
        let inside = doc.rel.strip_prefix(&dir).ok()?;
        let parts: Vec<_> = inside.components().collect();
        (parts.len() > 1).then(|| parts[0].as_os_str().to_string_lossy().into_owned())
    });
    let key = anchors
        .registers()
        .by_name(kind.name())
        .and_then(|r| r.metadata.first())
        .map(|(k, _)| k.clone());
    Record {
        anchor: anchor.to_string(),
        id: id.to_string(),
        title: heading_text(doc, 1).unwrap_or_else(|| id.to_string()),
        site: site.clone(),
        group,
        body: doc.text.clone(),
        metadata: key.and_then(|k| value_of(doc, &k)),
        guards: guards(anchors.registers(), doc, 1, u32::MAX),
    }
}

/// A heading register's entry: the heading's own section.
fn heading_record(
    anchors: &Anchors,
    anchor: &str,
    id: &str,
    site: &Site,
    doc: &Document,
) -> Record {
    let (from, to) = section(doc, site.line);
    Record {
        anchor: anchor.to_string(),
        id: id.to_string(),
        title: strip_slug(&heading_at(doc, from).unwrap_or_else(|| id.to_string())),
        site: site.clone(),
        group: None,
        body: lines(doc, from, to),
        metadata: None,
        guards: guards(anchors.registers(), doc, from, to),
    }
}

impl Record {
    /// The row a listing prints for a tripwire: the references it carries, joined.
    pub(crate) fn guarding(&self) -> String {
        if self.guards.is_empty() {
            "-".to_string()
        } else {
            self.guards.join(" ")
        }
    }

    /// Whether the entry's id or title holds `needle`, case-insensitively.
    pub(crate) fn matches(&self, needle: &str) -> bool {
        let needle = needle.to_lowercase();
        self.id.to_lowercase().contains(&needle) || self.title.to_lowercase().contains(&needle)
    }
}

/// Every site that references one entity, in document and line order.
///
/// The reference grammar's own resolver decides what is a reference, so a span this reports is
/// one the `references` family judged, and a span it passes over is one that family called
/// silent.
pub(crate) fn inbound(
    model: &Model,
    anchors: &Anchors,
    kind: &Kind,
    anchor: &str,
    id: &str,
) -> Vec<Site> {
    let mut out = Vec::new();
    for doc in model.documents() {
        for l in &doc.observations {
            let Observation::Span(span) = &l.what else {
                continue;
            };
            if let Candidate::Reference {
                kind: k,
                anchor: a,
                id: i,
            } = candidate(span, anchors)
            {
                // A path's id is compared without its trailing slash: `x` and `x/` name one
                // target, and the slash is a kind claim the check judges.
                let same = if kind.takes_a_path() {
                    i.trim_end_matches('/') == id.trim_end_matches('/')
                } else {
                    i == id
                };
                // A planned citation of a path is one a conversion edits once the file exists,
                // so `show` on the path lists it too, per `design@core@planned-path-form`.
                let kind_matches = &k == kind || (kind.is_path() && k.is_planned());
                if kind_matches && a == anchor && same {
                    out.push(Site {
                        file: doc.rel.clone(),
                        line: l.line,
                    });
                }
            }
        }
    }
    out.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    out
}

/// The inclusive line range of the section holding `line`: the nearest heading at or above it,
/// through to the next heading at the same level or shallower.
///
/// An entry is its heading and the body under it, so the section rather than the line is what
/// a reader is shown.
fn section(doc: &Document, line: u32) -> (u32, u32) {
    let mut headings: Vec<(u32, u8)> = doc
        .observations
        .iter()
        .filter_map(|l| match &l.what {
            Observation::Heading { level, .. } => Some((l.line, *level)),
            _ => None,
        })
        .collect();
    headings.sort();
    let last = doc.text.lines().count() as u32;
    let Some(&(from, level)) = headings.iter().rev().find(|(at, _)| *at <= line) else {
        return (1, last);
    };
    let to = headings
        .iter()
        .find(|(at, l)| *at > from && *l <= level)
        .map(|(at, _)| at.saturating_sub(1))
        .unwrap_or(last);
    (from, to)
}

/// The text of lines `from..=to`, one-based.
fn lines(doc: &Document, from: u32, to: u32) -> String {
    doc.text
        .lines()
        .skip(from.saturating_sub(1) as usize)
        .take((to.saturating_sub(from) + 1) as usize)
        .collect::<Vec<_>>()
        .join("\n")
}

/// The text of the first heading of `level` in the document.
fn heading_text(doc: &Document, level: u8) -> Option<String> {
    doc.observations.iter().find_map(|l| match &l.what {
        Observation::Heading { level: n, text } if *n == level => Some(text.clone()),
        _ => None,
    })
}

/// The text of the heading on `line`, where one is there.
fn heading_at(doc: &Document, line: u32) -> Option<String> {
    doc.observations.iter().find_map(|l| match &l.what {
        Observation::Heading { text, .. } if l.line == line => Some(text.clone()),
        _ => None,
    })
}

/// A heading's text without the slug that ends it.
///
/// The statement comes first and the slug last, so the title is everything before the last
/// backticked span opening with `##`.
fn strip_slug(text: &str) -> String {
    let trimmed = text.trim_end();
    let Some(inner) = trimmed.strip_suffix('`') else {
        return trimmed.to_string();
    };
    let Some(at) = inner.rfind('`') else {
        return trimmed.to_string();
    };
    if !inner[at + 1..].starts_with("##") {
        return trimmed.to_string();
    }
    inner[..at].trim_end().to_string()
}

/// One frontmatter value of a document, where the block parsed and carries the key.
fn value_of(doc: &Document, key: &str) -> Option<String> {
    doc.parsed
        .frontmatter
        .as_ref()?
        .as_ref()
        .ok()?
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
}

/// Every span of a [`guarded`] kind written between `from` and `to`, as written.
///
/// Read off the scanner's spans rather than off the text, so a rule about what is a reference
/// lives in one place.
fn guards(registers: &Registers, doc: &Document, from: u32, to: u32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for l in &doc.observations {
        if l.line < from || l.line > to {
            continue;
        }
        let Observation::Span(span) = &l.what else {
            continue;
        };
        let kind = span.split('@').next().unwrap_or_default();
        if guarded(registers, kind) && !out.contains(span) {
            out.push(span.clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Manifest;
    use std::path::PathBuf;

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `design@core@checker-source-literals-are-data`.

    /// A project whose root component is `a-project`, with one location carrying the issue
    /// register and a declared `reading` register.
    fn anchors() -> Anchors {
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"issue\", \"reading\"]\n\n\
             [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\
             sections = [\"Reading\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        Anchors::declared(&Manifest::parse(Path::new("/nowhere"), text).expect("a declaration"))
    }

    fn rows(docs: Vec<(&str, &str)>, kind: &str) -> Vec<Record> {
        let anchors = anchors();
        let model = Model::from_documents(
            docs.into_iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        let entities = Entities::build(&model, &anchors);
        records(&model, &anchors, &entities, &Kind::new(kind))
    }

    /// The claim: a milestone's record is its README whole, under no group, whatever headings
    /// it holds. Mutation checked: the Directory arm of `records` removed, which reads the
    /// README as a heading section and stops at its second level-one heading.
    #[test]
    fn a_milestone_entry_is_its_readme_whole() {
        let text = "# A milestone\n\nIts plan.\n\n# A second top heading\n\nMore of it.\n";
        let docs = [("docs/plans/milestones/m/README.md", text)];
        let tree: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n",
        )
        .expect("a declaration");
        let anchors = Anchors::of(&manifest, &tree);
        let model = Model::from_documents(
            docs.iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        let entities = Entities::build(&model, &anchors);
        let found = records(&model, &anchors, &entities, &Kind::new("milestone"));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert_eq!(
            (found[0].anchor.as_str(), found[0].id.as_str()),
            ("plans", "m")
        );
        assert_eq!(found[0].title, "A milestone");
        assert_eq!(found[0].group, None);
        assert_eq!(found[0].body, text);
    }

    #[test]
    fn a_file_entry_is_the_file_whole_and_carries_its_own_metadata() {
        let text = "---\nkind: defect\n---\n# A broken thing\n\n## Summary\n\nIt broke.\n";
        let found = rows(vec![("notes/open-issues/a-broken-thing.md", text)], "issue");
        assert_eq!(found.len(), 1, "{found:#?}");
        assert_eq!(found[0].anchor, "notes");
        assert_eq!(found[0].id, "a-broken-thing");
        assert_eq!(found[0].title, "A broken thing");
        assert_eq!(found[0].metadata.as_deref(), Some("defect"));
        assert_eq!(found[0].group, None);
        assert_eq!(found[0].body, text, "the file whole, frontmatter included");
    }

    #[test]
    fn an_entry_in_a_group_names_it_and_a_refused_block_leaves_the_kind_unread() {
        let two_kinds = "---\nkind: defect\nkind: todo\n---\n# Two values for one thing\n";
        let found = rows(
            vec![("notes/open-issues/a-group/two-kinds.md", two_kinds)],
            "issue",
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert_eq!(found[0].group.as_deref(), Some("a-group"));
        // The block was refused, so there is no value to print. A listing showing the first of
        // two would be choosing between them, which is what the refusal exists to avoid.
        assert_eq!(found[0].metadata, None);
    }

    #[test]
    fn a_heading_entry_is_its_own_section_and_loses_the_slug_from_its_title() {
        let home = "# Tripwires\n\n\
                    ## The first one `##first`\n\n\
                    **Fires when:** it fires, guarding `design@a-project@a-decision`, and see \
                    `path@a-project@docs/design.md` and `issue@notes@a-thing`.\n\n\
                    **Response:** reopen `design@a-project@a-decision`, and read \
                    `design@a-project@another-one`, `reading@notes@a-reading`, \
                    `goal@a-project@a-goal` and `tripwire@a-project@second`.\n\n\
                    ## The second one `##second`\n\n\
                    **Fires when:** something else.\n";
        let found = rows(vec![("docs/tripwires.md", home)], "tripwire");
        assert_eq!(found.len(), 2, "{found:#?}");
        let first = found.iter().find(|r| r.id == "first").expect("the first");
        assert_eq!(first.title, "The first one");
        assert_eq!(first.site.line, 3);
        // The section stops at the next heading of the same level, so the second entry's body
        // is not in the first's.
        assert!(first.body.contains("it fires"), "{}", first.body);
        assert!(!first.body.contains("something else"), "{}", first.body);
        assert!(!first.body.contains("# Tripwires"), "{}", first.body);
        // Every guarded kind, a declared register's included: a path, an issue and a tripwire
        // reference sit beside them and are not what this tripwire guards. The first is written
        // twice in the section and appears once, in the order the entry names them.
        assert_eq!(
            first.guards,
            vec![
                "design@a-project@a-decision".to_string(),
                "design@a-project@another-one".to_string(),
                "reading@notes@a-reading".to_string(),
                "goal@a-project@a-goal".to_string(),
            ]
        );
        assert_eq!(
            first.guarding(),
            "design@a-project@a-decision design@a-project@another-one \
             reading@notes@a-reading goal@a-project@a-goal"
        );
        let second = found.iter().find(|r| r.id == "second").expect("the second");
        assert!(second.guards.is_empty(), "{second:#?}");
        assert_eq!(second.guarding(), "-");
    }

    #[test]
    fn inbound_names_every_site_that_references_the_entity_and_nothing_else() {
        let anchors = anchors();
        let model = Model::from_documents(vec![
            (
                PathBuf::from("docs/design.md"),
                "# Decisions\n\n## A decision `##a-decision`\n\nIt holds.\n".to_string(),
            ),
            (
                PathBuf::from("docs/tripwires.md"),
                "# Tripwires\n\n## One `##one`\n\nGuarding `design@a-project@a-decision`.\n"
                    .to_string(),
            ),
            (
                PathBuf::from("README.md"),
                // Four references: the one asked for, and one differing in each segment of the
                // key. Each of the last three is one part of the key, so a resolver dropping
                // any comparison picks one of them up.
                "See `design@a-project@a-decision`, `design@a-project@another`, \
                 `design@notes@a-decision` and `tripwire@a-project@a-decision`.\n"
                    .to_string(),
            ),
        ]);
        let sites = inbound(
            &model,
            &anchors,
            &Kind::new("design"),
            "a-project",
            "a-decision",
        );
        assert_eq!(
            sites.iter().map(Site::to_string).collect::<Vec<_>>(),
            vec!["README.md:1", "docs/tripwires.md:5"]
        );
        // One occurrence on that line and not four: the other three are other entities.
        assert_eq!(
            sites
                .iter()
                .filter(|s| s.file.ends_with("README.md"))
                .count(),
            1
        );
        // The definition site is not an inbound reference.
        assert!(!sites.iter().any(|s| s.file.ends_with("design.md")));
    }

    #[test]
    fn a_title_keeps_every_backticked_span_that_is_not_its_slug() {
        assert_eq!(strip_slug("A statement `##the-slug`"), "A statement");
        assert_eq!(
            strip_slug("A statement about `code`"),
            "A statement about `code`"
        );
        assert_eq!(strip_slug("A plain statement"), "A plain statement");
        assert_eq!(strip_slug("`##only-a-slug`"), "");
    }

    #[test]
    fn a_needle_matches_the_id_or_the_title_in_either_case() {
        let found = rows(
            vec![(
                "notes/open-issues/a-broken-thing.md",
                "---\nkind: defect\n---\n# A Broken Thing\n",
            )],
            "issue",
        );
        assert!(found[0].matches("BROKEN"));
        assert!(found[0].matches("a-broken"));
        assert!(
            !found[0].matches("defect"),
            "the kind is a column, not the text"
        );
    }
}
