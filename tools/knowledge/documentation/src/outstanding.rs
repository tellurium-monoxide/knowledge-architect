//! What is outstanding, across every tracker in the project.
//!
//! Asking otherwise takes one file open per component, and the count grows with every one that
//! is added. The set of tracker files comes from `knowledge.toml [project]`: every component
//! carries the same documents, and two of them are its trackers. The entries come from reading
//! those files at run time, so no count is stored anywhere and none can go stale.

use std::path::PathBuf;

use crate::manifest::Manifest;
use crate::model::Model;

/// One tracker entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub file: PathBuf,
    pub title: String,
    pub kind: Kind,
    pub body: String,
    /// Whether it sits in a file the project calls its issues tracker, which is what decides
    /// the totals — not the kind, which describes the entry rather than the file.
    pub is_issue: bool,
}

/// What an entry is: the tag its own title carries, or a tripwire.
///
/// **The label is derived from the tag as written**, not matched against a list. The kinds are
/// deliberately not a closed set — an entry written with a kind nobody anticipated is intended
/// — and matching against a list meant such an entry fell through to the literal `tripwire`,
/// which is the one wrong answer available because it is also a real kind. A reader scanning
/// the issues then met a row labelled `tripwire` inside an issues file and had to work out
/// which of the two was meant.
///
/// `tripwire` is now reserved for an entry whose body says when it fires, and nothing else can
/// produce that label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// It states when it fires: a hypothesis about a future failure.
    Tripwire,
    /// Whatever its title says it is.
    Tagged(String),
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `pad` rather than `write_str`: a Display impl that writes directly ignores the
        // width, and this is printed in a column.
        match self {
            Kind::Tripwire => f.pad("tripwire"),
            Kind::Tagged(k) => f.pad(k),
        }
    }
}

/// Every tracker file in the project, whether or not it holds an entry.
///
/// An empty tracker is a well-formed tracker, and the count says how many exist rather than
/// how many have something in them — the difference between the two is itself worth seeing.
pub fn tracker_files(model: &Model, manifest: &Manifest) -> Vec<PathBuf> {
    let present: std::collections::HashSet<&PathBuf> =
        model.documents().iter().map(|d| &d.rel).collect();
    let mut out: Vec<PathBuf> = manifest
        .tracker_paths()
        .into_iter()
        .filter(|p| present.contains(p))
        .collect();
    out.sort();
    out
}

/// Every entry in every tracker the project declares.
pub fn entries(model: &Model, manifest: &Manifest) -> Vec<Entry> {
    // The components' own paths, not a filename match over the whole tree. A file called
    // `open-issues.md` in a directory that is not a component is not a tracker, and counting it
    // would report an entry against a total the project never claimed.
    let registered: std::collections::HashSet<PathBuf> =
        manifest.tracker_paths().into_iter().collect();
    let mut out = Vec::new();
    for doc in model.documents() {
        if !registered.contains(&doc.rel) {
            continue;
        }
        let Some(name) = doc.rel.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let is_issue = name == "open-issues.md";
        for (title, body) in sections(&doc.text) {
            // A heading alone does not make an entry: a tracker may carry a grouping header or
            // a note, and counting those inflates the total this exists to make trustworthy.
            // An entry is recognised by the required fields its own skill mandates — a
            // tripwire states when it fires, an issue states what it is — so the test is the
            // one a reader applies.
            let tagged = tag_of(&title);
            let fires = body.lines().any(states_when_it_fires);
            if !fires && tagged.is_none() && !body.contains("**What.**") {
                continue;
            }
            // The title's own tag is the label wherever there is one. `tripwire` is what a
            // body that states when it fires produces, and nothing else can produce it — which
            // is the whole of the fix: a kind nobody anticipated used to land there.
            let kind = match (&tagged, fires) {
                (Some(k), _) => Kind::Tagged(k.clone()),
                (None, true) => Kind::Tripwire,
                (None, false) => Kind::Tagged("issue".to_string()),
            };
            out.push(Entry {
                file: doc.rel.clone(),
                title: strip_tag(&title),
                kind,
                body,
                is_issue,
            });
        }
    }
    out
}

/// Whether this line is a tripwire's firing field: a bold label opening with `Fires when`,
/// optionally qualified — `**Fires when:**`, `**Fires when (the bound):**` — whose bold span
/// closes with a colon.
///
/// Anchored to the line's start, not a substring anywhere: an entry ABOUT tripwires quotes the
/// phrase in its prose, and a substring test reads that entry as a tripwire — which is how the
/// first draft of the anchoring fix mislabelled the very entry recording the defect it was
/// fixing. Requiring the span to close with `:` is what keeps a bold `**Fires when**` opening
/// an ordinary sentence from counting as a field.
///
/// **The qualifier is admitted in any shape**, because the exact-prefix match this replaces
/// dropped a real entry whole: a two-clause tripwire tells its clauses apart inside the label —
/// `**Fires when (the bound):**` beside `**Fires when (the key):**` — and matching only the
/// unqualified form left it out of the listing and the count, unenumerated for every session
/// that reads the trackers through this tool as instructed.
fn states_when_it_fires(line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix("**Fires when") else {
        return false;
    };
    rest.find("**")
        .is_some_and(|end| rest[..end].ends_with(':'))
}

/// `(title, body)` for each second- or third-level heading.
fn sections(text: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut body = String::new();
    let mut title: Option<String> = None;
    for line in text.lines() {
        let heading = line
            .strip_prefix("### ")
            .or_else(|| line.strip_prefix("## "));
        match heading {
            Some(h) => {
                if let Some(t) = title.take() {
                    out.push((t, std::mem::take(&mut body)));
                }
                title = Some(h.trim().to_string());
            }
            None => {
                if title.is_some() {
                    body.push_str(line);
                    body.push('\n');
                }
            }
        }
    }
    if let Some(t) = title {
        out.push((t, body));
    }
    out
}

/// The kind tag a title ends with, as `` `kind` ``, whatever word it holds.
///
/// A tag is one lowercase word, which is what separates it from the backticked paths, slugs
/// and code spans a title may also end with.
fn tag_of(title: &str) -> Option<String> {
    let trimmed = title.trim_end();
    let inner = trimmed.strip_suffix('`')?;
    let at = inner.rfind('`')?;
    let kind = &inner[at + 1..];
    let word = !kind.is_empty() && kind.chars().all(|c| c.is_ascii_lowercase() || c == '-');
    word.then(|| kind.to_string())
}

/// The title without its kind tag.
fn strip_tag(title: &str) -> String {
    match tag_of(title) {
        Some(kind) => title
            .trim_end()
            .strip_suffix(&format!("`{kind}`"))
            .unwrap_or(title)
            .trim_end()
            .to_string(),
        None => title.trim_end().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tagged_title_gives_up_its_kind_and_loses_the_tag() {
        assert_eq!(
            tag_of("Something is wrong `defect`").as_deref(),
            Some("defect")
        );
        assert_eq!(
            strip_tag("Something is wrong `defect`"),
            "Something is wrong"
        );
        assert_eq!(tag_of("A plain heading"), None);
        assert_eq!(strip_tag("A plain heading"), "A plain heading");
    }

    #[test]
    fn a_kind_nobody_anticipated_is_read_as_itself() {
        // The recorded defect, closed. `todo` is a kind the skill's table names and the old
        // list omitted, and any future kind behaves the same way: the label is the tag as
        // written, and the title loses it.
        assert_eq!(tag_of("Work left undone `todo`").as_deref(), Some("todo"));
        assert_eq!(strip_tag("Work left undone `todo`"), "Work left undone");
        assert_eq!(
            tag_of("Something `newly-invented`").as_deref(),
            Some("newly-invented")
        );
    }

    #[test]
    fn a_backticked_span_that_is_not_a_kind_is_not_a_tag() {
        // Titles routinely end with a path, a slug or an identifier in backticks. The slug
        // and the path are written inline: the checker reads no string literal of its own
        // source, per `knowledge#checker-source-literals-are-data`.
        assert_eq!(tag_of("The subject of `outstanding.py`"), None);
        assert_eq!(tag_of("Guarding `#some-slug`"), None);
        assert_eq!(tag_of("A title ending in `CamelCase`"), None);
    }

    #[test]
    fn tripwire_is_reserved_for_a_body_that_says_when_it_fires() {
        assert_eq!(Kind::Tripwire.to_string(), "tripwire");
        assert_eq!(Kind::Tagged("todo".into()).to_string(), "todo");
        // Printed in a column, so the label pads.
        assert_eq!(
            format!("{:<10}|", Kind::Tagged("todo".into())),
            "todo      |"
        );
    }

    #[test]
    fn a_firing_field_is_recognised_with_or_without_a_qualifier() {
        assert!(states_when_it_fires(
            "**Fires when:** a reader treats the bound as a guarantee"
        ));
        // The recorded defect, closed: the two-clause entry's own labels, which the
        // exact-prefix match this repairs dropped from the listing entirely. The mutation the
        // qualified cases discriminate is restoring `starts_with("**Fires when:**")`.
        assert!(states_when_it_fires(
            "**Fires when (the bound):** a reader or a document treats the engine's catalog bound"
        ));
        assert!(states_when_it_fires(
            "  **Fires when (the key):** a second pool exists"
        ));
        // Prose about the field is not the field: quoted mid-line, or a bold span opening an
        // ordinary sentence rather than closing a label with a colon.
        assert!(!states_when_it_fires(
            "an entry states **Fires when:** and a response"
        ));
        assert!(!states_when_it_fires(
            "**Fires when** is the field a tripwire states"
        ));
    }

    #[test]
    fn a_heading_alone_is_not_an_entry() {
        let text = "## A grouping header\n\nJust prose.\n\n## A real one `defect`\n\n**What.** It broke.\n";
        let found = sections(text);
        assert_eq!(found.len(), 2);
        assert!(tag_of(&found[0].0).is_none() && !found[0].1.contains("**What.**"));
        assert!(tag_of(&found[1].0).is_some());
    }

    #[test]
    fn both_heading_depths_open_an_entry() {
        let text = "## Two\n\nbody one\n\n### Three\n\nbody two\n";
        let found = sections(text);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].0, "Two");
        assert_eq!(found[1].0, "Three");
    }
}
