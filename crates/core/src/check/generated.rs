//! The generated indexes are current.
//!
//! A generated file that drifts is worse than no generated file: thaum's `bumping-rules` skill
//! calls the rule index, which thaum's rules extension generates, *the work list* for a
//! release, and a stale work list decides what a renumbering breaks.
//!
//! **The comparison is against a string.** Nothing here writes the file and restores it, so a
//! run that dies half-way leaves the tree exactly as it found it.

use std::path::Path;

use crate::extension::Generated;
use crate::finding::Finding;
use crate::index;
use crate::manifest::Manifest;
use crate::model::Model;

use super::Inputs;

/// Compare each generated file with what it would be generated as now: every file an
/// extension generates, then one index per file-register instance.
///
/// An extension's files arrive already rendered, because only the extension can render them;
/// the file-register indexes are rendered here, so a caller handing in no extension still has
/// every listing compared.
pub(crate) fn check(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    extension_files: &[Generated],
) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in extension_files {
        compare(&file.rel, &file.text, inputs, file.action, &mut findings);
    }
    for (rel, expected) in index::file_register_indexes(model, manifest, inputs.directories) {
        compare(
            &rel,
            &expected,
            inputs,
            &format!(
                "run `{} index`; the listing is a function of the entries beside it, \
                 and a hand edit is what this reports",
                manifest.command()
            ),
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
    fn a_run_with_no_extension_still_judges_the_file_register_indexes() {
        use crate::manifest::Manifest;
        use std::collections::{HashMap, HashSet};
        use std::path::PathBuf;

        let text = "[project]\nname = \"a-project\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let committed = HashMap::new();
        let configs = HashMap::new();
        let present: HashSet<PathBuf> = HashSet::new();
        let directories: HashSet<PathBuf> = [PathBuf::from("docs/open-issues")].into();
        let inputs = Inputs {
            committed: &committed,
            configs: &configs,
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
        };
        let found: Vec<String> = check(&model, &manifest, &inputs, &[])
            .iter()
            .map(|f| f.location())
            .collect();
        // The issue instance's index, and nothing an extension would have generated.
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
