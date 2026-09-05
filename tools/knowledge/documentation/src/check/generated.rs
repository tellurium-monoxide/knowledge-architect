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
/// A run that could not resolve the release still judges every index whose generator reads the
/// model alone, because a family that reported nothing would say the listings are current.
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
    fn the_first_differing_line_is_reported() {
        assert_eq!(first_difference("a\nb\nc", "a\nB\nc"), 2);
        // A file that is a prefix of the other differs at the line past the shorter one.
        assert_eq!(first_difference("a\nb", "a\nb\nc"), 3);
        assert_eq!(first_difference("same", "same"), 2);
    }
}
