//! Every slug anchor referenced somewhere is defined exactly once, in the component named.
//!
//! A slug is DEFINED where it opens a decision and REFERENCED anywhere else. A reference with
//! no definition is a pointer into nothing, which the head-and-commit split makes cheap to
//! create and expensive to notice; two definitions mean a rename left one behind, and the
//! reader who finds the stale one acts on it.
//!
//! **A slug is unique inside its component, not across the project.** Two components may each
//! decide something they call the same word, and a reference says which one it means by naming
//! it: `` `<component>#<slug>` ``. That makes three ways for a reference to resolve to nothing
//! rather than one, and each has its own repair — the component is not declared, the component
//! is declared and does not define that slug, or the reference names no component at all.

use std::collections::BTreeMap;

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::scan::Observation;

/// Where something was written: the document, and the line in it.
type Place = (String, u32);

pub fn check(model: &Model, manifest: &Manifest) -> (Vec<Finding>, (usize, usize)) {
    let components = manifest.components();

    let mut out = Vec::new();

    // Keyed by the component that owns the definition, which is where the document sits rather
    // than anything the line says.
    let mut defined: BTreeMap<(&str, &str), Vec<Place>> = BTreeMap::new();
    let mut referenced: BTreeMap<(Option<&str>, &str), Vec<Place>> = BTreeMap::new();
    for doc in model.documents() {
        let name = doc.rel.display().to_string();
        let owner = components.owning(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::SlugDef(s) => {
                    // The design home's two shapes, per `check::components`: the single file,
                    // or a subdocument of the directory. The directory's README is the head
                    // and the table of contents, not a decision's home. Matched against the
                    // owning component's own paths rather than by filename suffix — a suffix
                    // match accepted a slug in any file whose NAME ends in `design.md`, a
                    // plan document included.
                    let file = owner.document(crate::manifest::DESIGN_FILE);
                    let dir = owner.document(crate::manifest::DESIGN_DIR);
                    let readme = owner.document(crate::manifest::DESIGN_README);
                    let in_home =
                        doc.rel == file || (doc.rel.starts_with(&dir) && doc.rel != readme);
                    if !in_home {
                        out.push(Finding::at(
                            name.clone(),
                            l.line,
                            format!("`#{s}` defined outside its component's design home"),
                            "write it in docs/design.md of its component, or in a \
                             subdocument of its docs/design/ directory",
                        ))
                    }
                    defined
                        .entry((owner.name.as_str(), s.as_str()))
                        .or_default()
                        .push((name.clone(), l.line))
                }
                Observation::SlugRef { component, slug } => referenced
                    .entry((component.as_deref(), slug.as_str()))
                    .or_default()
                    .push((name.clone(), l.line)),
                _ => {}
            }
        }
    }
    for ((component, slug), where_) in &referenced {
        let (file, line) = &where_[0];
        match component {
            // Naming no component resolves to nothing: the same word may open a decision in
            // any number of them, so there is no component to look in and no way to guess one.
            None => out.push(Finding::at(
                file,
                *line,
                format!("`#{slug}` names no component"),
                "write it as `<component>#slug`, naming the component that defines it; \
                 knowledge.toml [project] is what declares them",
            )),
            Some(named) if components.by_name(named).is_none() => out.push(Finding::at(
                file,
                *line,
                format!("`{named}#{slug}` names `{named}`, which is no component of this project"),
                "name a component of knowledge.toml [project], by the basename of its path, \
                 or the project itself by its name",
            )),
            Some(named) if !defined.contains_key(&(named, slug)) => out.push(Finding::at(
                file,
                *line,
                format!("`{named}#{slug}` is referenced and `{named}` does not define it"),
                "define it where the decision is made, or name the component that holds it",
            )),
            Some(_) => {}
        }
    }
    for ((component, slug), where_) in &defined {
        if where_.len() > 1 {
            let all: Vec<String> = where_.iter().map(|(f, l)| format!("{f}:{l}")).collect();
            let (file, line) = &where_[0];
            out.push(Finding::at(
                file,
                *line,
                format!(
                    "`{component}#{slug}` is defined {} times: {}",
                    where_.len(),
                    all.join(", ")
                ),
                "delete all but one; a reference must resolve to exactly one anchor",
            ));
        }
    }
    (out, (defined.len(), referenced.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `knowledge#checker-source-literals-are-data`.

    /// A project whose root component is `a-project` and which declares one component under `parts/`.
    fn manifest() -> Manifest {
        let text = "[project]\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n";
        Manifest::parse(Path::new("/nowhere"), text).expect("a declaration")
    }

    fn findings(docs: Vec<(&str, String)>) -> Vec<String> {
        let docs = docs
            .into_iter()
            .map(|(p, text)| (PathBuf::from(p), text))
            .collect();
        let model = Model::from_documents(docs);
        check(&model, &manifest())
            .0
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    /// A design-home line defining the slug `a-decision`.
    fn head() -> String {
        "### `##a-decision` — **The decision.**\n".to_string()
    }

    #[test]
    fn a_reference_resolves_against_the_component_it_names() {
        // Both directions across the boundary, and a definition is owned by where its document
        // sits rather than by anything the line says.
        let found = findings(vec![
            (
                "docs/design.md",
                format!("{}\nIt rests on `a-part#a-decision`.\n", head()),
            ),
            (
                "parts/a-part/docs/design.md",
                format!("{}\nIt rests on `a-project#a-decision`.\n", head()),
            ),
        ]);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn the_same_slug_in_two_components_is_two_decisions_and_not_a_duplicate() {
        // What qualifying a reference buys: two components may each decide something they call
        // the same word, and nothing has to invent a second word for one of them.
        let found = findings(vec![
            ("docs/design.md", head()),
            ("parts/a-part/docs/design.md", head()),
        ]);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn the_same_slug_twice_in_one_component_is_reported_once_naming_both_places() {
        // Two documents of one component, so the duplicate is a property of the component
        // rather than of a file — which is the case a per-file check would miss.
        let found = findings(vec![
            ("docs/design.md", head()),
            ("docs/rejected-alternatives.md", head()),
        ]);
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found[0].contains("`#a-decision` defined outside its component's design home"),
            "{found:#?}"
        );
        assert!(
            found[1].contains("`a-project#a-decision` is defined 2 times"),
            "{found:#?}"
        );
        assert!(
            found[1].contains("docs/design.md:1")
                && found[1].contains("docs/rejected-alternatives.md:1"),
            "the finding must name both places: {found:#?}"
        );
    }

    #[test]
    fn each_way_a_reference_resolves_to_nothing_is_reported_as_the_repair_it_needs() {
        let found = findings(vec![
            ("docs/design.md", head()),
            (
                "docs/open-issues.md",
                "Naming none: `#a-decision`.\n\
                 Naming a component that is not declared: `nothing-declares-this#a-decision`.\n\
                 Naming one that does not define it: `a-part#a-decision`.\n"
                    .to_string(),
            ),
        ]);
        assert_eq!(found.len(), 3, "{found:#?}");
        let at = |line: u32, needle: &str| {
            let prefix = format!("docs/open-issues.md:{line}  ");
            assert!(
                found
                    .iter()
                    .any(|f| f.starts_with(&prefix) && f.contains(needle)),
                "expected {needle:?} at line {line}: {found:#?}"
            );
        };
        at(1, "names no component");
        at(2, "which is no component of this project");
        at(3, "does not define it");
    }

    #[test]
    fn a_subdocument_of_the_design_directory_is_a_definition_home() {
        // Both components at once, and a cross-reference between them, so ownership is by
        // the document's place under the component rather than by any one path shape.
        let found = findings(vec![
            (
                "docs/design/one-subject.md",
                format!("{}\nIt rests on `a-part#a-decision`.\n", head()),
            ),
            (
                "parts/a-part/docs/design/one-subject.md",
                format!("{}\nIt rests on `a-project#a-decision`.\n", head()),
            ),
        ]);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn the_design_directorys_readme_is_not_a_definition_home() {
        // The README is the head and the table of contents. A decision recorded there
        // competes with the subdocuments as a home, which is what the split exists to end.
        let found = findings(vec![("docs/design/README.md", head())]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("outside its component's design home"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_file_merely_named_like_a_design_document_is_not_a_definition_home() {
        // The rule matches the owning component's own paths, never a filename suffix: a
        // suffix match accepted a slug in any file whose name ends in `design.md`, which a
        // plan document's can.
        let found = findings(vec![("notes/a-plan-design.md", head())]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("notes/a-plan-design.md:1"),
            "{found:#?}"
        );
        assert!(
            found[0].contains("outside its component's design home"),
            "{found:#?}"
        );
    }

    #[test]
    fn the_file_home_is_matched_exactly_and_not_by_path_suffix() {
        // The fixture path ends with the home's whole path, component by component. The
        // match is against the owning component's own document, so the suffix shape is as
        // foreign as any other file.
        let found = findings(vec![("notes/docs/design.md", head())]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("outside its component's design home"),
            "{found:#?}"
        );
    }

    #[test]
    fn the_counts_are_of_distinct_anchors_and_distinct_pointers() {
        // Not of occurrences: a slug pointed at from five documents is one pointer as far as
        // the report is concerned, and counting the mentions would make the two numbers read
        // as a coverage ratio they are not.
        let docs = vec![
            (PathBuf::from("docs/design.md"), head()),
            (
                PathBuf::from("docs/open-issues.md"),
                "`a-project#a-decision` and again `a-project#a-decision`.\n".to_string(),
            ),
            (
                PathBuf::from("README.md"),
                "`a-project#a-decision` a third time.\n".to_string(),
            ),
        ];
        let model = Model::from_documents(docs);
        let (found, (defined, referenced)) = check(&model, &manifest());
        assert!(found.is_empty(), "{found:#?}");
        assert_eq!((defined, referenced), (1, 1));
    }
}
