//! The generated indexes are current.
//!
//! A generated file that drifts is worse than no generated file: `bumping-rules` calls the
//! rule index *the work list* for a release, and a stale work list decides what a renumbering
//! breaks.
//!
//! **The comparison is against a string.** Nothing here writes the file and restores it, so a
//! run that dies half-way leaves the tree exactly as it found it.

use std::path::Path;

use crate::finding::Finding;
use crate::index;
use crate::manifest::Manifest;
use crate::model::Model;

use super::Inputs;

/// Compare each generated file with what it would be generated as now.
///
/// **The rule index is gated on the vendored release and the file-register indexes are not.**
/// The binary resolves the release before it calls this family, so no run of `check` reaches here
/// without one; what the gate buys is that a caller assembling its own `Inputs` — every test here
/// does — cannot silence the whole family by handing it no release, which would report the
/// listings current without comparing a byte.
pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Vec<Finding> {
    let mut findings = Vec::new();

    if let Some(vendored) = inputs.releases.get(&None) {
        compare(
            &manifest.rules().dir.join("index.md"),
            &index::rule_index(model, manifest, &vendored.rules, inputs.pinned),
            inputs,
            "regenerate it and read the diff: it is the work list a release bump reads",
            &mut findings,
        );
    }
    for (rel, expected) in index::file_register_indexes(model, manifest, inputs.directories) {
        compare(
            &rel,
            &expected,
            inputs,
            "run `cargo knowledge index`; the listing is a function of the entries beside it, \
             and a hand edit is what this reports",
            &mut findings,
        );
    }
    findings
}

fn compare(path: &Path, expected: &str, inputs: &Inputs, action: &str, out: &mut Vec<Finding>) {
    let committed = inputs.committed.get(path).map(String::as_str);
    match committed {
        None => out.push(Finding::in_file(
            path,
            "the generated file is missing",
            action,
        )),
        Some(current) if current != expected => {
            let line = first_difference(current, expected);
            out.push(Finding::at(
                path,
                line,
                "the generated file is out of date".to_string(),
                action,
            ));
        }
        Some(_) => {}
    }
}

/// The first line where the committed file and the regenerated one disagree.
///
/// A file-level *this is stale* sends a reader to diff the whole thing; a line sends them to
/// the change. Both files are generated, so the first disagreement is where the content
/// actually moved.
fn first_difference(a: &str, b: &str) -> u32 {
    let mut n = 1;
    for (x, y) in a.lines().zip(b.lines()) {
        if x != y {
            return n;
        }
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_with_no_release_still_judges_the_file_register_indexes() {
        use crate::manifest::Manifest;
        use std::collections::{HashMap, HashSet};
        use std::path::PathBuf;

        let text = "[project]\nname = \"a-project\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let releases = HashMap::new();
        let committed = HashMap::new();
        let configs = HashMap::new();
        let present: HashSet<PathBuf> = HashSet::new();
        let directories: HashSet<PathBuf> = [PathBuf::from("docs/open-issues")].into();
        let inputs = Inputs {
            releases: &releases,
            pinned: "20200101",
            committed: &committed,
            configs: &configs,
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
        };
        let found: Vec<String> = check(&model, &manifest, &inputs)
            .iter()
            .map(|f| f.location())
            .collect();
        // The issue instance's index, and not the rule index, which has no release to render.
        assert_eq!(found, vec!["docs/open-issues/index.md".to_string()]);
    }

    #[test]
    fn the_first_differing_line_is_reported() {
        assert_eq!(first_difference("a\nb\nc", "a\nB\nc"), 2);
        // A file that is a prefix of the other differs at the line past the shorter one.
        assert_eq!(first_difference("a\nb", "a\nb\nc"), 3);
        assert_eq!(first_difference("same", "same"), 2);
    }
}
