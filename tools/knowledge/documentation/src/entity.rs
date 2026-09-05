//! The entity table: everything a document can cite, and the one resolver every check reads.
//!
//! An entity has a **kind**, an **anchor**, an **id** and a **definition site**. The table is
//! built once from the model, and a reference `` `<kind>@<anchor>@<id>` `` is resolved against
//! it. Before this table five checks held five notions of a name; the argument is
//! `knowledge#one-entity-table`.
//!
//! **An anchor is a named directory that carries registers.** Today every anchor is a
//! component, carrying the three built-in heading registers; a location — a directory carrying
//! a declared subset — is the same shape with a different register list and home base, which is
//! why `Anchor` carries both as data rather than deriving them from a component.
//!
//! **The `path` kind is resolved against the tree, not the table.** Its ids are paths, its
//! anchors are the same anchors plus two reserved words, and the check that resolves it needs
//! the survey. What this module gives it is the candidate rule, the segmentation and the anchor
//! lookup, so one grammar has one reader.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::finding::Finding;
use crate::manifest::{Manifest, COMPONENT_DOCUMENTS};
use crate::model::Model;
use crate::scan::{Observation, SlugSite};

/// What a reference names, in its first segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// A decision about how a component is built.
    Design,
    /// What a component is for.
    Goal,
    /// Evidence that would flip a decision.
    Tripwire,
    /// A file or directory. Defined by the tree itself.
    Path,
}

impl Kind {
    /// Every kind, in the order findings list them.
    pub const ALL: [Kind; 4] = [Kind::Design, Kind::Goal, Kind::Tripwire, Kind::Path];

    /// The three heading registers, each with the basename of its home under the anchor's
    /// home base: `<dir>.md` or `<dir>/`.
    pub const HEADING_REGISTERS: [(Kind, &'static str); 3] = [
        (Kind::Design, "design"),
        (Kind::Goal, "goals"),
        (Kind::Tripwire, "tripwires"),
    ];

    /// The word a reference spells.
    pub fn name(self) -> &'static str {
        match self {
            Kind::Design => "design",
            Kind::Goal => "goal",
            Kind::Tripwire => "tripwire",
            Kind::Path => "path",
        }
    }

    pub fn parse(word: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.name() == word)
    }

    /// The basename of this kind's heading-register home, or `None` for `path`.
    pub fn register_dir(self) -> Option<&'static str> {
        Kind::HEADING_REGISTERS
            .iter()
            .find(|(k, _)| *k == self)
            .map(|(_, d)| *d)
    }

    /// The kind names, comma-separated, as a finding lists them.
    pub fn listed() -> String {
        Kind::ALL
            .iter()
            .map(|k| k.name())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The reserved anchor word for a path deliberately not resolvable in this tree.
///
/// A declared component may not take this name, which `check::components` asserts.
pub const ESCAPE_ANCHOR: &str = "elsewhere";

/// The reserved anchor for every component's own copy of a path.
pub const EVERY_ANCHOR: &str = "*";

/// The shape an anchor name must have for a reference to be able to name it.
///
/// A name a reference cannot spell is one every pointer at it misses silently, so the check
/// over the declaration asks this rather than assuming it.
static ANCHOR_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\.?[A-Za-z0-9][A-Za-z0-9._-]*$").unwrap());

/// Can a reference name an anchor called this.
pub fn is_anchor_name(name: &str) -> bool {
    ANCHOR_NAME.is_match(name)
}

/// The two shapes of a heading register's home, for one anchor and one kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Home {
    /// The single-file shape.
    pub file: PathBuf,
    /// The directory shape, whose subdocuments hold the definitions.
    pub dir: PathBuf,
    /// The head of the directory shape. It defines nothing.
    pub readme: PathBuf,
}

/// A named directory carrying registers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    /// The one word a reference names it by.
    pub name: String,
    /// Project-relative, and empty for the component at the root.
    pub path: PathBuf,
    /// Where its heading-register homes sit: `<path>/docs` for a component.
    pub home_base: PathBuf,
    /// The heading registers it carries. `path` is carried by every anchor and is not listed.
    pub registers: Vec<Kind>,
}

impl Anchor {
    /// A component: every built-in register, homes under `docs/`.
    pub fn component(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            home_base: path.join("docs"),
            registers: Kind::HEADING_REGISTERS.iter().map(|(k, _)| *k).collect(),
        }
    }

    /// Whether this is the component at the project root.
    pub fn is_root(&self) -> bool {
        self.path.as_os_str().is_empty()
    }

    /// Whether a reference of this kind may anchor here.
    pub fn carries(&self, kind: Kind) -> bool {
        kind == Kind::Path || self.registers.contains(&kind)
    }

    /// The home of one heading register here, or `None` where this anchor does not carry it.
    pub fn home(&self, kind: Kind) -> Option<Home> {
        if !self.carries(kind) {
            return None;
        }
        let dir = kind.register_dir()?;
        Some(Home {
            file: self.home_base.join(format!("{dir}.md")),
            dir: self.home_base.join(dir),
            readme: self.home_base.join(dir).join("README.md"),
        })
    }
}

/// Every anchor of a project: the component at the root first, then the declared ones.
#[derive(Clone, Debug)]
pub struct Anchors(Vec<Anchor>);

impl Anchors {
    /// The anchors a manifest declares: its components, each carrying every built-in register.
    pub fn of(manifest: &Manifest) -> Self {
        Self(
            manifest
                .components()
                .all()
                .iter()
                .map(|c| Anchor::component(&c.name, &c.path))
                .collect(),
        )
    }

    /// Anchors stated directly, for a test that needs a register list no component has.
    pub fn from_list(anchors: Vec<Anchor>) -> Self {
        Self(anchors)
    }

    pub fn all(&self) -> &[Anchor] {
        &self.0
    }

    /// The anchor a document belongs to: the deepest whose path holds it, and the root where
    /// none does. The root's path is empty and is a prefix of every path, which is what makes
    /// it the fallback rather than a case.
    pub fn owning(&self, rel: &Path) -> &Anchor {
        self.0
            .iter()
            .filter(|a| rel.starts_with(&a.path))
            .max_by_key(|a| a.path.components().count())
            .expect("the anchor at the root is a prefix of every path")
    }

    /// The anchor a reference names, or `None` where nothing declares that name.
    pub fn by_name(&self, name: &str) -> Option<&Anchor> {
        self.0.iter().find(|a| a.name == name)
    }

    /// Every anchor name, comma-separated, as a finding lists them.
    pub fn listed(&self) -> String {
        self.0
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Whether `word` could stand in the anchor position of some reference: a declared
    /// anchor, or one of the two words reserved for `path`.
    pub fn is_anchor_word(&self, word: &str) -> bool {
        word == EVERY_ANCHOR || word == ESCAPE_ANCHOR || self.by_name(word).is_some()
    }

    /// Whether a component-relative path is a required document name in one of its shapes,
    /// and whether that shape is a directory: what the generic anchor `path@*@<path>` accepts
    /// whether or not any component carries it yet.
    ///
    /// The compiled-in document set, and every heading register's file, directory and README:
    /// naming a shape no component uses yet is legitimate.
    pub fn required_kind(path: &str) -> Option<bool> {
        if COMPONENT_DOCUMENTS.contains(&path) {
            return Some(false);
        }
        for (_, dir) in Kind::HEADING_REGISTERS {
            if path == format!("docs/{dir}.md") || path == format!("docs/{dir}/README.md") {
                return Some(false);
            }
            if path == format!("docs/{dir}") {
                return Some(true);
            }
        }
        None
    }
}

/// Where something was written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    pub file: PathBuf,
    pub line: u32,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file.display(), self.line)
    }
}

/// What a backticked `@` span is, under the candidate rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Candidate<'a> {
    /// A well-formed reference: kind, anchor word as written, id as written.
    Reference {
        kind: Kind,
        anchor: &'a str,
        id: &'a str,
    },
    /// A known kind followed by the wrong number of segments, or an empty one.
    Malformed { why: &'static str },
    /// The head is an anchor, or a reserved anchor, where a kind goes: the old form.
    AnchorInKindPosition { head: &'a str },
    /// The head is neither a kind nor an anchor. Not a reference; silent.
    NotOne,
}

/// The candidate rule and the segmentation, over one backticked span holding an `@`.
///
/// **Which spans are candidates.** The head — the text before the first `@` — is a known kind,
/// a declared anchor or a reserved anchor; anything else is not a reference and is silent, so
/// an email address or a git remote in backticks reports nothing. A typo inside the kind is
/// silent for the same reason, which `knowledge@docs/tripwires.md` guards.
///
/// **Segmentation.** Every kind but `path` takes exactly three segments; `path` takes an anchor
/// and then everything after the second `@` as its id, so a path may hold an `@`. An empty
/// segment is malformed in either shape.
pub fn candidate<'a>(span: &'a str, anchors: &Anchors) -> Candidate<'a> {
    let Some((head, rest)) = span.split_once('@') else {
        return Candidate::NotOne;
    };
    // An empty head in front of a path shape is the retired `@` escape, or a kind that
    // was never typed; either way a pointer that would otherwise leave every check. A
    // slashless `@word` is an annotation or a handle and stays silent.
    if head.is_empty() && rest.contains('/') {
        return Candidate::Malformed {
            why: "the kind segment is empty",
        };
    }
    let Some(kind) = Kind::parse(head) else {
        if anchors.is_anchor_word(head) {
            return Candidate::AnchorInKindPosition { head };
        }
        return Candidate::NotOne;
    };
    let Some((anchor, id)) = rest.split_once('@') else {
        return Candidate::Malformed {
            why: "two segments; a reference has three",
        };
    };
    if anchor.is_empty() {
        return Candidate::Malformed {
            why: "the anchor segment is empty",
        };
    }
    if id.is_empty() {
        return Candidate::Malformed {
            why: "the id segment is empty",
        };
    }
    if kind != Kind::Path && id.contains('@') {
        return Candidate::Malformed {
            why: "four or more segments; a reference has three",
        };
    }
    Candidate::Reference { kind, anchor, id }
}

/// Why a reference to a table kind resolves to nothing, or that it resolves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    Resolved,
    /// Nothing declares that anchor name.
    UnknownAnchor,
    /// The anchor exists and carries no register of that kind; the anchors that do.
    AnchorLacksRegister {
        carriers: Vec<String>,
    },
    /// The anchor carries the register and defines no such id.
    Undefined,
}

/// The table: every entity defined in the walk, keyed by kind, anchor and id.
#[derive(Debug, Default)]
pub struct Entities {
    defined: BTreeMap<(Kind, String, String), Vec<Site>>,
    /// Misplaced and duplicate definitions, found while building.
    findings: Vec<Finding>,
}

impl Entities {
    /// Read every slug definition out of the model and file it under its anchor and register.
    ///
    /// A definition is owned by where its document sits — the deepest anchor whose path holds
    /// it — and by which of that anchor's register homes the document is. A slug outside every
    /// home, or at a heading level the grammar does not accept, or at the head of a plain line,
    /// defines nothing and is reported as misplaced.
    pub fn build(model: &Model, anchors: &Anchors) -> Self {
        let mut out = Self::default();
        for doc in model.documents() {
            let owner = anchors.owning(&doc.rel);
            for l in &doc.observations {
                let Observation::SlugDef { id, site } = &l.what else {
                    continue;
                };
                let at = Site {
                    file: doc.rel.clone(),
                    line: l.line,
                };
                let misplaced = |why: String| {
                    Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`##{id}` is written at {why} and defines nothing"),
                        format!(
                            "a slug is defined at the end of a level-two or level-three \
                             heading, or in a table cell, inside the {} home of its anchor; \
                             move it there, or delete it",
                            Kind::HEADING_REGISTERS
                                .iter()
                                .map(|(_, d)| *d)
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )
                };
                match site {
                    SlugSite::Heading(level) if !(2..=3).contains(level) => {
                        out.findings
                            .push(misplaced(format!("a level-{level} heading")));
                        continue;
                    }
                    SlugSite::LineHead => {
                        out.findings
                            .push(misplaced("the head of a plain line".to_string()));
                        continue;
                    }
                    SlugSite::Inline => {
                        out.findings
                            .push(misplaced("the middle of a line".to_string()));
                        continue;
                    }
                    _ => {}
                }
                match Self::register_of(owner, &doc.rel) {
                    Some(Ok(kind)) => out
                        .defined
                        .entry((kind, owner.name.clone(), id.clone()))
                        .or_default()
                        .push(at),
                    Some(Err(dir)) => out.findings.push(misplaced(format!(
                        "the README of the `{}` directory home",
                        dir.display()
                    ))),
                    None => out.findings.push(misplaced(format!(
                        "`{}`, which is no register home of `{}`",
                        doc.rel.display(),
                        owner.name
                    ))),
                }
            }
        }
        for ((kind, anchor, id), sites) in &out.defined {
            if sites.len() < 2 {
                continue;
            }
            // One finding at each site, each naming the others, so the reader who opens
            // either copy is told the other exists.
            for site in sites {
                let others: Vec<String> = sites
                    .iter()
                    .filter(|s| *s != site)
                    .map(Site::to_string)
                    .collect();
                out.findings.push(Finding::at(
                    &site.file,
                    site.line,
                    format!(
                        "`{kind}@{anchor}@{id}` is also defined at {}",
                        others.join(", ")
                    ),
                    "keep one definition; a reference resolves to exactly one entity",
                ));
            }
        }
        out
    }

    /// Which register home of `owner` holds `rel`: `Ok(kind)` for the file home or a
    /// subdocument of the directory home, `Err(dir)` for the directory home's README, `None`
    /// for a file that is no home.
    fn register_of(owner: &Anchor, rel: &Path) -> Option<Result<Kind, PathBuf>> {
        for kind in &owner.registers {
            let home = owner.home(*kind)?;
            if rel == home.file {
                return Some(Ok(*kind));
            }
            if rel == home.readme {
                return Some(Err(home.dir));
            }
            if rel.starts_with(&home.dir) {
                return Some(Ok(*kind));
            }
        }
        None
    }

    /// Resolve a reference to a table kind. `path` is not this table's to resolve.
    pub fn resolve(&self, anchors: &Anchors, kind: Kind, anchor: &str, id: &str) -> Resolution {
        let Some(a) = anchors.by_name(anchor) else {
            return Resolution::UnknownAnchor;
        };
        if !a.carries(kind) {
            return Resolution::AnchorLacksRegister {
                carriers: anchors
                    .all()
                    .iter()
                    .filter(|a| a.carries(kind))
                    .map(|a| a.name.clone())
                    .collect(),
            };
        }
        if self
            .defined
            .contains_key(&(kind, anchor.to_string(), id.to_string()))
        {
            Resolution::Resolved
        } else {
            Resolution::Undefined
        }
    }

    /// How many distinct entities the table holds.
    pub fn len(&self) -> usize {
        self.defined.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defined.is_empty()
    }

    /// The misplaced and duplicate definitions found while building the table.
    pub fn definition_findings(&self) -> &[Finding] {
        &self.findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `knowledge#checker-source-literals-are-data`.

    /// A project whose root component is `a-project` with one component under `parts/`.
    fn anchors() -> Anchors {
        let text = "[project]\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n";
        Anchors::of(&Manifest::parse(Path::new("/nowhere"), text).expect("a declaration"))
    }

    fn table(docs: Vec<(&str, &str)>) -> Entities {
        let model = Model::from_documents(
            docs.into_iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        Entities::build(&model, &anchors())
    }

    fn findings(docs: Vec<(&str, &str)>) -> Vec<String> {
        table(docs)
            .definition_findings()
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    #[test]
    fn a_slug_in_each_register_home_defines_an_entity_of_that_kind() {
        // The three heading registers, file shape, each defining the same word: three
        // entities, because the register is part of the key.
        let e = table(vec![
            ("docs/design.md", "### A decision `##same`\n"),
            ("docs/goals.md", "## A goal `##same`\n"),
            ("docs/tripwires.md", "## Guarding it `##same`\n"),
        ]);
        assert_eq!(e.len(), 3);
        assert!(e.definition_findings().is_empty(), "{:#?}", e.findings);
        let a = anchors();
        for kind in [Kind::Design, Kind::Goal, Kind::Tripwire] {
            assert_eq!(
                e.resolve(&a, kind, "a-project", "same"),
                Resolution::Resolved,
                "{kind}"
            );
        }
        assert_eq!(
            e.resolve(&a, Kind::Design, "a-project", "other"),
            Resolution::Undefined
        );
    }

    #[test]
    fn a_directory_home_defines_in_its_subdocuments_and_not_in_its_readme() {
        // Both shapes, for a register other than design, so the two-shape rule is shown to
        // generalise. The README is the head and defines nothing.
        let e = table(vec![
            ("docs/goals/one.md", "## A goal `##in-a-subdocument`\n"),
            ("docs/goals/README.md", "## A goal `##in-the-readme`\n"),
        ]);
        assert_eq!(e.len(), 1);
        let found = findings(vec![(
            "docs/goals/README.md",
            "## A goal `##in-the-readme`\n",
        )]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("the README of the `docs/goals` directory home"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_slug_outside_every_home_is_misplaced_and_defines_nothing() {
        // Mutation checked: with the `None` arm pushing no finding, the count is zero.
        let e = table(vec![("notes/a.md", "### A decision `##stray`\n")]);
        assert_eq!(e.len(), 0, "a misplaced slug defines nothing");
        let found = findings(vec![("notes/a.md", "### A decision `##stray`\n")]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("notes/a.md:1") && found[0].contains("no register home"),
            "{found:#?}"
        );
        // A file merely NAMED like a home, in a directory that is no anchor's docs/.
        let found = findings(vec![("notes/docs/design.md", "### A decision `##stray`\n")]);
        assert_eq!(found.len(), 1, "{found:#?}");
    }

    #[test]
    fn a_slug_at_the_wrong_heading_level_or_at_a_line_head_is_misplaced_even_in_a_home() {
        for (line, why) in [
            ("# A title `##deep`\n", "level-1"),
            ("#### A deep heading `##deep`\n", "level-4"),
            ("##### A deeper heading `##deep`\n", "level-5"),
            ("`##deep` — **The statement.**\n", "head of a plain line"),
            ("as `##deep` records\n", "middle of a line"),
        ] {
            let e = table(vec![("docs/design.md", line)]);
            assert_eq!(e.len(), 0, "{line:?} must define nothing");
            let found = findings(vec![("docs/design.md", line)]);
            assert_eq!(found.len(), 1, "{line:?}: {found:#?}");
            assert!(found[0].contains(why), "{line:?}: {found:#?}");
        }
    }

    #[test]
    fn a_definition_belongs_to_the_deepest_anchor_holding_its_document() {
        let e = table(vec![
            ("docs/design.md", "### A decision `##word`\n"),
            ("parts/a-part/docs/design.md", "### A decision `##word`\n"),
        ]);
        assert_eq!(e.len(), 2, "two anchors, two entities");
        assert!(e.definition_findings().is_empty());
        let a = anchors();
        assert_eq!(
            e.resolve(&a, Kind::Design, "a-part", "word"),
            Resolution::Resolved
        );
        assert_eq!(
            e.resolve(&a, Kind::Design, "a-project", "word"),
            Resolution::Resolved
        );
    }

    #[test]
    fn two_definitions_in_one_instance_are_a_finding_at_each_site() {
        // Mutation checked: iterating `sites.iter().take(1)` leaves one finding, and the
        // assertion on two fails.
        let found = findings(vec![
            ("docs/design.md", "### One `##twice`\n\n### Two `##twice`\n"),
            ("docs/goals.md", "## A goal `##twice`\n"),
        ]);
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found[0].starts_with("docs/design.md:1") && found[0].contains("docs/design.md:3"),
            "{found:#?}"
        );
        assert!(
            found[1].starts_with("docs/design.md:3") && found[1].contains("docs/design.md:1"),
            "{found:#?}"
        );
        assert!(
            found.iter().all(|f| f.contains("`design@a-project@twice`")),
            "the finding names the entity in the grammar: {found:#?}"
        );
    }

    #[test]
    fn a_misplaced_definition_does_not_count_toward_a_duplicate() {
        // It defines nothing, so it cannot collide with the one that does.
        let found = findings(vec![
            ("docs/design.md", "### One `##once`\n"),
            ("notes/a.md", "### One `##once`\n"),
        ]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("defines nothing"), "{found:#?}");
    }

    #[test]
    fn the_candidate_rule_admits_a_kind_or_an_anchor_at_the_head_and_nothing_else() {
        let a = anchors();
        assert_eq!(
            candidate("design@a-project@x-y", &a),
            Candidate::Reference {
                kind: Kind::Design,
                anchor: "a-project",
                id: "x-y"
            }
        );
        // The old forms: a component, the generic and the escape anchor in kind position.
        for span in ["a-project@docs/x.md", "*@docs/x.md", "elsewhere@x"] {
            assert!(
                matches!(candidate(span, &a), Candidate::AnchorInKindPosition { .. }),
                "{span}"
            );
        }
        // An email, a remote, a placeholder-free typo in the kind, an annotation: silent.
        for span in [
            "user@example.test",
            "git@host:x/y.git",
            "desing@a-project@x",
            "@Test",
        ] {
            assert_eq!(candidate(span, &a), Candidate::NotOne, "{span}");
        }
        // An empty head in front of a path shape is a pointer that would otherwise leave
        // every check: the retired `@` escape, or a kind never typed.
        assert!(
            matches!(candidate("@docs/x.md", &a), Candidate::Malformed { .. }),
            "{:?}",
            candidate("@docs/x.md", &a)
        );
    }

    #[test]
    fn segmentation_takes_three_segments_except_that_a_path_may_hold_an_at_sign() {
        // Mutation checked: dropping the `kind != Kind::Path` guard makes the path case
        // malformed and the first assertion fails.
        let a = anchors();
        assert_eq!(
            candidate("path@a-project@notes/a@b.md", &a),
            Candidate::Reference {
                kind: Kind::Path,
                anchor: "a-project",
                id: "notes/a@b.md"
            }
        );
        for span in [
            "design@a-project",
            "design@a-project@x@y",
            "design@@x",
            "design@a-project@",
            "path@a-project",
            "path@@x",
            "path@a-project@",
        ] {
            assert!(
                matches!(candidate(span, &a), Candidate::Malformed { .. }),
                "{span}"
            );
        }
    }

    #[test]
    fn each_way_a_reference_resolves_to_nothing_is_told_apart() {
        // The fourth arm needs an anchor that exists and lacks the register, which no
        // component has: the anchor is stated directly, the shape a location will take.
        let root = Anchor::component("a-project", Path::new(""));
        let bare = Anchor {
            name: "bare".to_string(),
            path: PathBuf::from("bare"),
            home_base: PathBuf::from("bare"),
            registers: vec![Kind::Tripwire],
        };
        let a = Anchors::from_list(vec![root, bare]);
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "### A decision `##word`\n".to_string(),
        )]);
        let e = Entities::build(&model, &a);
        assert_eq!(
            e.resolve(&a, Kind::Design, "nowhere", "word"),
            Resolution::UnknownAnchor
        );
        assert_eq!(
            e.resolve(&a, Kind::Design, "bare", "word"),
            Resolution::AnchorLacksRegister {
                carriers: vec!["a-project".to_string()]
            }
        );
        assert_eq!(
            e.resolve(&a, Kind::Design, "a-project", "other"),
            Resolution::Undefined
        );
        assert_eq!(
            e.resolve(&a, Kind::Design, "a-project", "word"),
            Resolution::Resolved
        );
        // Every anchor carries `path`, whatever its register list.
        assert!(a.by_name("bare").unwrap().carries(Kind::Path));
    }

    #[test]
    fn the_required_document_set_carries_each_shape_with_its_kind() {
        // The compiled-in files, and each heading register's file, directory and README.
        assert_eq!(Anchors::required_kind("README.md"), Some(false));
        assert_eq!(Anchors::required_kind("docs/tripwires.md"), Some(false));
        assert_eq!(Anchors::required_kind("docs/goals.md"), Some(false));
        assert_eq!(Anchors::required_kind("docs/goals"), Some(true));
        assert_eq!(Anchors::required_kind("docs/goals/README.md"), Some(false));
        assert_eq!(Anchors::required_kind("docs/design"), Some(true));
        assert_eq!(Anchors::required_kind("docs/design/one.md"), None);
        assert_eq!(Anchors::required_kind("src/lib.rs"), None);
    }

    #[test]
    fn an_anchor_name_is_spellable_exactly_when_the_pattern_accepts_it() {
        for (name, ok) in [
            ("a-tool", true),
            ("an.engine", true),
            (".claude", true),
            ("an_engine", true),
            ("9lives", true),
            ("-leading-dash", false),
            ("has/slash", false),
            ("has space", false),
            ("", false),
        ] {
            assert_eq!(is_anchor_name(name), ok, "{name:?}");
        }
    }
}
