//! Every project-relative path named in a document exists.
//!
//! Slugs were checked and paths were not, so a rename once left nine references to a deleted
//! file sitting in live documents while every checker reported green. A pointer with no
//! enforcement is a guess.

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::scan::Observation;

use super::Inputs;

pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, usize) {
    let components = manifest.components();
    let mut out = Vec::new();
    let mut seen = 0;
    for doc in model.documents() {
        for l in &doc.observations {
            if let Observation::PathRef {
                component,
                path: reference,
            } = &l.what
            {
                let line = l.line;
                // An absolute path is not this project's to resolve, and a space means the
                // backticks held prose rather than a path.
                if reference.starts_with('/') || reference.contains(' ') {
                    continue;
                }
                seen += 1;

                match component {
                    None => {
                        let from_root = normalise(std::path::Path::new(reference));
                        // Allow old style ref pointing from root of repo.
                        if inputs.present.contains(&from_root) {
                            continue;
                        }
                        out.push(Finding::at(
                                &doc.rel,
                                line,
                                format!("`{reference}` found as an old path reference, now only allowed for existing paths that stem from repo root."),
                                "migrate to the new syntax: `<component>@path/from/component/file.md`",
                            ));
                    }

                    Some(component) => {
                        let component_root = components.by_name(component);
                        match component_root {
                            None => {
                                out.push(Finding::at(
                        &doc.rel,
                        line,
                        format!("`{component}` does not exist"),
                        "repair the pointer, or delete it; a path should point to an existing component, or the project",
                    ));
                            }
                            Some(cr) => {
                                // Relative to the component_root, `.` and `..` resolved the
                                // same way `check::components` resolves a design README's
                                // links, so a pointer means one thing whichever check reads it.
                                let beside = normalise(&cr.path.join(reference));
                                let from_root = normalise(std::path::Path::new(reference));
                                if inputs.present.contains(&beside)
                                    || (cr.is_root() && inputs.present.contains(&from_root))
                                {
                                    continue;
                                }
                                let p = cr.path.clone().into_os_string().into_string().unwrap();
                                out.push(Finding::at(
                        &doc.rel,
                        line,
                        format!("`{reference}` does not exist in component {component} at {p}"),
                        "repair the pointer, or delete it; a path that does not resolve is a guess",
                    ));
                            }
                        }
                    }
                }
            }
        }
    }
    (out, seen)
}

/// Resolve `.` and `..` textually, so a parent reference beside a document in one
/// directory names a file in its sibling.
///
/// `check::components` resolves a design README's links with the same rule,
/// so a pointer means one thing whichever check reads it.
pub(crate) fn normalise(path: &std::path::Path) -> std::path::PathBuf {
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
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};

    #[test]
    fn a_component_relative_reference_resolves_dots_against_the_component() {
        // The same textual resolution as the root form and as `check::components`' reading
        // of a design README, so a pointer means one thing whichever check reads it.
        let part = "parts/a-part";
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = [\"{part}\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n"
        );
        let manifest = Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration");
        let dotted = "notes/other/../real/a.md";
        let model = Model::from_documents(vec![(
            PathBuf::from("README.md"),
            format!("See `a-part@{dotted}`.\n"),
        )]);
        let releases = HashMap::new();
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = [PathBuf::from(format!("{part}/notes/real/a.md"))].into();
        let directories = crate::check::testing::implied_directories(&present);
        let outside = Vec::new();
        let inputs = Inputs {
            releases: &releases,
            pinned: "",
            committed: &committed,
            present: &present,
            directories: &directories,
            outside: &outside,
        };
        let (found, seen) = check(&model, &manifest, &inputs);
        let found: Vec<String> = found.iter().map(|f| f.to_string()).collect();
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(seen, 1, "the reference was looked at");
    }

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
