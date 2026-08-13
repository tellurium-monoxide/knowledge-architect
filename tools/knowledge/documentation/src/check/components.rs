//! Every component the project declares carries the documents a component carries.
//!
//! A component is a directory with its own README, its own scoped `CLAUDE.md` and its own four
//! documents under `docs/`, and the project root is one of them. What that buys is one home per
//! component for each kind of statement: what it does, what a session inside it must know, what
//! was decided, what lost, what is outstanding, and what would reopen the decisions.
//!
//! **A missing document is a finding rather than an absence.** The trackers among them are what
//! `cargo knowledge outstanding` reads, so one that is not there makes the report under-count
//! what is open — and the report is what a session consults before diagnosing anything. The
//! other four fail the same way more slowly: a component with no docs/design.md has its
//! design recorded wherever the last session happened to put it.
//!
//! **A tracker outside every component is declared one by one, and checked the same way.** Some
//! directories carry outstanding state and nothing else a component carries, so
//! `additional-trackers` names those files directly. Each owes the same two things: it exists,
//! and it is named as a tracker — the report splits its totals on that name, so a file called
//! anything else joins them under whichever half its name happens to fall in.

use std::collections::BTreeMap;

use crate::finding::Finding;
use crate::manifest::{Manifest, COMPONENT_DOCUMENTS, COMPONENT_TRACKERS, MANIFEST_NAME};
use crate::scan;

use super::Inputs;

/// What the check looked at: the components, and the trackers declared outside them.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub components: usize,
    pub additional_trackers: usize,
}

/// The findings, and what was looked at.
pub fn check(manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
    let components = manifest.components();
    let mut out = Vec::new();

    // The declaration first: a name that is ambiguous or that no reference can spell breaks
    // every pointer at that component, which is a worse failure than a missing document and
    // one a reader would otherwise meet as a dangling reference somewhere else entirely.
    let mut by_name: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for component in components.all() {
        by_name
            .entry(component.name.as_str())
            .or_default()
            .push(where_it_is(component));
    }
    for (name, places) in &by_name {
        if places.len() > 1 {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "`{name}` names {} components: {}",
                    places.len(),
                    places.join(", ")
                ),
                "rename or move one; a slug reference names its component by that one word, \
                 so two of them cannot share it",
            ));
        }
        if !scan::is_component_name(name) {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!("`{name}` cannot be spelled in a slug reference"),
                "name it in letters, digits, `.`, `-` and `_`; a component nothing can point \
                 at is one every pointer misses in silence",
            ));
        }
    }

    for component in components.all() {
        // A component directory that is not there is ONE finding. Six, one per document it
        // does not carry, would bury the single fact that explains all of them.
        if !component.is_root() && !inputs.present.contains(&component.path) {
            out.push(Finding::in_file(
                &component.path,
                format!(
                    "`{}` is declared a component and does not exist",
                    component.path.display()
                ),
                "create it, or stop listing it in knowledge.toml [project] components",
            ));
            continue;
        }
        for name in COMPONENT_DOCUMENTS {
            let path = component.document(name);
            if !inputs.present.contains(&path) {
                out.push(Finding::in_file(
                    &path,
                    format!("the component `{}` carries no {name}", component.name),
                    "create it, or stop listing the component in knowledge.toml [project]; \
                     every component carries the same documents",
                ));
            }
        }
    }
    // A tracker belonging to no component. It is declared as a file rather than as a directory
    // because that directory carries nothing else a component carries.
    let additional = &manifest.project().additional_trackers;
    for path in additional {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        // The report splits its totals on this name, and every component tracker is one of
        // these two. A file named anything else is counted under whichever half its own name
        // falls in, silently — so the name is asserted where it is declared.
        if !COMPONENT_TRACKERS.iter().any(|t| t.ends_with(&name)) {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!("`{}` is declared a tracker and is not named as one", path.display()),
                "name it as one of the tracker files a component carries, or stop listing it                  in knowledge.toml [project] additional-trackers",
            ));
        }
        if !inputs.present.contains(path) {
            out.push(Finding::in_file(
                path,
                format!(
                    "`{}` is declared an additional tracker and does not exist",
                    path.display()
                ),
                "create it, or stop listing it in knowledge.toml [project] additional-trackers",
            ));
        }
    }

    let counts = Counts {
        components: components.all().len(),
        additional_trackers: additional.len(),
    };
    (out, counts)
}

/// How a finding names a component's place: its path, or the root.
fn where_it_is(component: &crate::manifest::Component) -> String {
    if component.is_root() {
        "the project root".to_string()
    } else {
        component.path.display().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::citations::Release;
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};

    /// A manifest declaring `components`, against a root nothing reads.
    fn declaring(components: &str) -> Manifest {
        declaring_with(components, "")
    }

    /// The same, also declaring trackers that belong to no component.
    fn declaring_with(components: &str, additional: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = [{components}]\n\
             additional-trackers = [{additional}]\n\n\
             [walk]\nsuffixes = [\"md\"]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The findings against a listing of what exists.
    fn findings(manifest: &Manifest, present: &[&str]) -> Vec<String> {
        let releases: HashMap<Option<String>, Release> = HashMap::new();
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let outside = Vec::new();
        let inputs = Inputs {
            releases: &releases,
            pinned: "",
            committed: &committed,
            present: &present,
            outside: &outside,
        };
        check(manifest, &inputs)
            .0
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    /// Every document a component carries, under `dir`.
    fn all_of(dir: &str) -> Vec<String> {
        COMPONENT_DOCUMENTS
            .iter()
            .map(|n| {
                if dir.is_empty() {
                    n.to_string()
                } else {
                    format!("{dir}/{n}")
                }
            })
            .collect()
    }

    #[test]
    fn a_project_carrying_every_document_of_every_component_reports_nothing() {
        let manifest = declaring("\"parts/a-part\"");
        let mut present = all_of("");
        present.push("parts/a-part".to_string());
        present.extend(all_of("parts/a-part"));
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        assert_eq!(findings(&manifest, &present), Vec::<String>::new());
    }

    #[test]
    fn each_missing_document_is_one_finding_naming_the_path_it_belongs_at() {
        // The root component is one, and is checked exactly like a declared one.
        let manifest = declaring("");
        let present: Vec<String> = all_of("")
            .into_iter()
            .filter(|p| p != "docs/tripwires.md")
            .collect();
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].starts_with("docs/tripwires.md  "), "{found:#?}");
        assert!(found[0].contains("a-project"), "{found:#?}");
    }

    #[test]
    fn a_component_directory_that_is_not_there_is_one_finding_and_not_six() {
        // One cause, one finding. Six would bury the fact that explains all of them.
        let manifest = declaring("\"parts/a-part\"");
        let present: Vec<String> = all_of("");
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
    }

    #[test]
    fn two_components_sharing_a_name_are_reported_against_the_declaration() {
        // Both paths end in the same basename, so a reference naming it resolves to whichever
        // was declared first — silently, and to the wrong decision half the time.
        let manifest = declaring("\"crates/a-part\", \"tools/a-part\"");
        let mut present = all_of("");
        for dir in ["crates/a-part", "tools/a-part"] {
            present.push(dir.to_string());
            present.extend(all_of(dir));
        }
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].starts_with("knowledge.toml  "), "{found:#?}");
        assert!(
            found[0].contains("crates/a-part") && found[0].contains("tools/a-part"),
            "the finding must name both places: {found:#?}"
        );
    }

    #[test]
    fn a_component_name_no_reference_can_spell_is_reported() {
        // The failure is silent otherwise: every pointer at it is scanned as no reference at
        // all, so nothing resolves them and nothing reports them.
        let manifest = declaring("\"parts/a part\"");
        let mut present = all_of("");
        present.push("parts/a part".to_string());
        present.extend(all_of("parts/a part"));
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("cannot be spelled"), "{found:#?}");
    }

    #[test]
    fn the_count_is_the_components_looked_at_including_the_root() {
        let manifest = declaring("\"parts/a-part\", \"parts/another\"");
        let releases: HashMap<Option<String>, Release> = HashMap::new();
        let committed = HashMap::new();
        let present = HashSet::new();
        let outside = Vec::new();
        let inputs = Inputs {
            releases: &releases,
            pinned: "",
            committed: &committed,
            present: &present,
            outside: &outside,
        };
        let counts = check(&manifest, &inputs).1;
        assert_eq!(counts.components, 3);
        assert_eq!(counts.additional_trackers, 0);
    }

    #[test]
    fn a_declared_tracker_outside_every_component_is_checked_where_it_is_declared() {
        // `.claude/` is the case: it carries what is open about the agent configuration and
        // nothing else a component carries, so the file is named directly.
        let manifest = declaring_with("", "\"a-directory/open-issues.md\"");
        let mut present = all_of("");
        present.push("a-directory/open-issues.md".to_string());
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        assert_eq!(findings(&manifest, &present), Vec::<String>::new());
    }

    #[test]
    fn a_declared_tracker_that_does_not_exist_is_reported_at_the_path_declared() {
        // The failure it prevents is silent: `outstanding` reads the declared paths, so a file
        // that is not there makes the report under-count what is open rather than fail.
        let manifest = declaring_with("", "\"a-directory/open-issues.md\"");
        let present = all_of("");
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("a-directory/open-issues.md  "),
            "{found:#?}"
        );
        assert!(found[0].contains("does not exist"), "{found:#?}");
    }

    #[test]
    fn a_declared_tracker_not_named_as_one_is_reported() {
        // `outstanding` splits its totals on the filename, so a tracker named anything else
        // joins one half or the other by accident and nothing says which.
        let manifest = declaring_with("", "\"a-directory/notes.md\"");
        let mut present = all_of("");
        present.push("a-directory/notes.md".to_string());
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].starts_with("knowledge.toml  "), "{found:#?}");
        assert!(found[0].contains("is not named as one"), "{found:#?}");
    }
}
