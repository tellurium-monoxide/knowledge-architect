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
    // Keyed by the component that owns the definition, which is where the document sits rather
    // than anything the line says.
    let mut defined: BTreeMap<(&str, &str), Vec<Place>> = BTreeMap::new();
    let mut referenced: BTreeMap<(Option<&str>, &str), Vec<Place>> = BTreeMap::new();
    for doc in model.documents() {
        let name = doc.rel.display().to_string();
        let owner = components.owning(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::SlugDef(s) => defined
                    .entry((owner.name.as_str(), s.as_str()))
                    .or_default()
                    .push((name.clone(), l.line)),
                Observation::SlugRef { component, slug } => referenced
                    .entry((component.as_deref(), slug.as_str()))
                    .or_default()
                    .push((name.clone(), l.line)),
                _ => {}
            }
        }
    }

    let mut out = Vec::new();
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
    use crate::manifest::Walk;
    use std::path::{Path, PathBuf};

    // Interpolated, never spelled out: this tool's own source is walked, so a slug written
    // literally here defines a real anchor and a reference here is a real pointer.
    const SLUG: &str = "a-decision";
    const ROOT: &str = "a-project";
    const PART: &str = "a-part";

    /// A project whose root component is `a-project` and which declares `parts/a-part`.
    fn manifest() -> Manifest {
        let text = format!(
            "[project]\nname = \"{ROOT}\"\ncomponents = [\"parts/{PART}\"]\n\n\
             [walk]\nsuffixes = [\"md\"]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    fn findings(docs: Vec<(&str, String)>) -> Vec<String> {
        let docs = docs
            .into_iter()
            .map(|(p, text)| (PathBuf::from(p), text))
            .collect();
        let model = Model::from_documents(docs, &Walk::sample());
        check(&model, &manifest())
            .0
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    fn head(slug: &str) -> String {
        format!("`##{slug}` — **The decision.**\n")
    }

    #[test]
    fn a_reference_resolves_against_the_component_it_names() {
        // Both directions across the boundary, and a definition is owned by where its document
        // sits rather than by anything the line says.
        let found = findings(vec![
            (
                "docs/design.md",
                format!("{}\nIt rests on `{PART}#{SLUG}`.\n", head(SLUG)),
            ),
            (
                &format!("parts/{PART}/docs/design.md"),
                format!("{}\nIt rests on `{ROOT}#{SLUG}`.\n", head(SLUG)),
            ),
        ]);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn the_same_slug_in_two_components_is_two_decisions_and_not_a_duplicate() {
        // What qualifying a reference buys: two components may each decide something they call
        // the same word, and nothing has to invent a second word for one of them.
        let found = findings(vec![
            ("docs/design.md", head(SLUG)),
            (&format!("parts/{PART}/docs/design.md"), head(SLUG)),
        ]);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn the_same_slug_twice_in_one_component_is_reported_once_naming_both_places() {
        // Two documents of one component, so the duplicate is a property of the component
        // rather than of a file — which is the case a per-file check would miss.
        let found = findings(vec![
            ("docs/design.md", head(SLUG)),
            ("docs/rejected_alternatives.md", head(SLUG)),
        ]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains(&format!("`{ROOT}#{SLUG}` is defined 2 times")),
            "{found:#?}"
        );
        assert!(
            found[0].contains("docs/design.md:1")
                && found[0].contains("docs/rejected_alternatives.md:1"),
            "the finding must name both places: {found:#?}"
        );
    }

    #[test]
    fn each_way_a_reference_resolves_to_nothing_is_reported_as_the_repair_it_needs() {
        let found = findings(vec![
            ("docs/design.md", head(SLUG)),
            (
                "docs/open-issues.md",
                format!(
                    "Naming none: `#{SLUG}`.\n\
                     Naming a component that is not declared: `nothing-declares-this#{SLUG}`.\n\
                     Naming one that does not define it: `{PART}#{SLUG}`.\n"
                ),
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
    fn the_counts_are_of_distinct_anchors_and_distinct_pointers() {
        // Not of occurrences: a slug pointed at from five documents is one pointer as far as
        // the report is concerned, and counting the mentions would make the two numbers read
        // as a coverage ratio they are not.
        let docs = vec![
            (PathBuf::from("docs/design.md"), head(SLUG)),
            (
                PathBuf::from("docs/open-issues.md"),
                format!("`{ROOT}#{SLUG}` and again `{ROOT}#{SLUG}`.\n"),
            ),
            (
                PathBuf::from("README.md"),
                format!("`{ROOT}#{SLUG}` a third time.\n"),
            ),
        ];
        let model = Model::from_documents(docs, &Walk::sample());
        let (found, (defined, referenced)) = check(&model, &manifest());
        assert!(found.is_empty(), "{found:#?}");
        assert_eq!((defined, referenced), (1, 1));
    }
}
