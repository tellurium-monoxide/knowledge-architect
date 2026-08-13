//! Every project-relative path named in a document exists.
//!
//! Slugs were checked and paths were not, so a rename once left nine references to a deleted
//! file sitting in live documents while every checker reported green. A pointer with no
//! enforcement is a guess.

use crate::finding::Finding;
use crate::model::Model;
use crate::scan::Observation;

use super::Inputs;

pub fn check(model: &Model, inputs: &Inputs) -> (Vec<Finding>, usize) {
    let mut out = Vec::new();
    let mut seen = 0;
    for doc in model.documents() {
        for (line, reference) in doc.observations_of(|o| match o {
            Observation::PathRef(p) => Some(p.as_str()),
            _ => None,
        }) {
            // An absolute path is not this project's to resolve, and a space means the
            // backticks held prose rather than a path.
            if reference.starts_with('/') || reference.contains(' ') {
                continue;
            }
            seen += 1;
            // Relative to the naming document first, then to the project root — a document
            // points at a sibling far more often than at the root.
            let beside = doc
                .rel
                .parent()
                .map(|d| normalise(&d.join(reference)))
                .unwrap_or_default();
            let from_root = normalise(std::path::Path::new(reference));
            if inputs.present.contains(&beside) || inputs.present.contains(&from_root) {
                continue;
            }
            out.push(Finding::at(
                &doc.rel,
                line,
                format!("`{reference}` does not exist"),
                "repair the pointer, or delete it; a path that does not resolve is a guess",
            ));
        }
    }
    (out, seen)
}

/// Resolve `.` and `..` textually, so a parent reference beside a document in one
/// directory names a file in its sibling.
fn normalise(path: &std::path::Path) -> std::path::PathBuf {
    let mut out = std::path::PathBuf::new();
    for part in path.components() {
        match part {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn a_parent_reference_resolves_against_the_naming_document() {
        // Built from parts: a path literal in a walked file is a reference this project's
        // own check would then have to resolve.
        const NAME: &str = "a.md";
        let with_parent = format!("docs/design/../rules/{NAME}");
        assert_eq!(
            normalise(Path::new(&with_parent)),
            Path::new(&format!("docs/rules/{NAME}"))
        );
        assert_eq!(normalise(Path::new(&format!("./{NAME}"))), Path::new(NAME));
    }
}
