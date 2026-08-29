//! Every component the project declares carries the documents a component carries.
//!
//! A component is a directory with its own README, its own scoped `CLAUDE.md`, its own three
//! documents under `docs/`, and one design home, and the project root is one of them. What
//! that buys is one home per component for each kind of statement: what it does, what a
//! session inside it must know, what was decided, what lost, what is outstanding, and what
//! would reopen the decisions.
//!
//! **A missing document is a finding rather than an absence.** The trackers among them are what
//! `cargo knowledge outstanding` reads, so one that is not there makes the report under-count
//! what is open — and the report is what a session consults before diagnosing anything. The
//! others fail the same way more slowly: a component with no design home has its
//! design recorded wherever the last session happened to put it.
//!
//! **The design home has two accepted shapes, and a component carries exactly one.** The
//! single file `docs/design.md`, or the directory `docs/design/` whose `README.md` is the
//! head: an introduction, and a table of contents naming every subdocument as a backticked
//! path. Both present would give a decision two candidate homes, which is the failure the
//! one-home rule exists to prevent; the table of contents is checked because a subdocument
//! nobody lists is a design home nobody finds.
//!
//! **A tracker outside every component is declared one by one, and checked the same way.** Some
//! directories carry outstanding state and nothing else a component carries, so
//! `additional-trackers` names those files directly. Each owes the same two things: it exists,
//! and it is named as a tracker — the report splits its totals on that name, so a file called
//! anything else joins them under whichever half its name happens to fall in.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use crate::finding::Finding;
use crate::manifest::{
    Component, Components, Manifest, COMPONENT_DOCUMENTS, COMPONENT_TRACKERS, DESIGN_DIR,
    DESIGN_FILE, DESIGN_README, MANIFEST_NAME,
};
use crate::model::Model;
use crate::scan::{self, Observation};

use super::Inputs;

/// What the check looked at: the components, and the trackers declared outside them.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub components: usize,
    pub additional_trackers: usize,
}

/// The findings, and what was looked at.
pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
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
        design_home(&mut out, component, model, &components, inputs);
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

    // **Every path the manifest declares is checked to exist.** A row naming a deleted file is
    // silent in both directions: nobody is told it is dead, and a file later created at that
    // path inherits what the row grants. That matters most for `exempt-files`, which
    // `check::regime::run` reads as well as the lint, so a stale row there can exempt a
    // document from the whole citation regime without anyone deciding to.
    //
    // **Every declaration is checkable because `.gitignore` carries the rest.** The walk prunes
    // what git does not track, so a manifest never names build output or a directory made on
    // demand — the class that cannot be checked, since a fresh clone has none of it.
    //
    // Existence, never file-ness: `skip-dirs` names directories, `exclude` names either, and
    // `docs/rules/past` is a declared skip that is legitimately empty in a fresh checkout.
    let walk = manifest.walk();
    let declared: [(&str, &Vec<std::path::PathBuf>); 4] = [
        ("[walk] skip-dirs", &walk.skip_dirs),
        ("[walk] skip-files", &walk.skip_files),
        ("[walk] exclude", &walk.exclude),
        ("[lint] exempt-files", &manifest.lint().exempt_files),
    ];
    for (list, paths) in declared {
        for path in paths {
            if inputs.present.contains(path) {
                continue;
            }
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "`{}` is declared in {list} and does not exist",
                    path.display()
                ),
                "delete the row, or restore what it names; a declaration nothing checks \
                 silently covers whatever is created at that path next. What git ignores is \
                 pruned by the walk and is never declared here",
            ));
        }
    }

    let counts = Counts {
        components: components.all().len(),
        additional_trackers: additional.len(),
    };
    (out, counts)
}

/// Exactly one design home, and a directory home's README names every subdocument.
fn design_home(
    out: &mut Vec<Finding>,
    component: &Component,
    model: &Model,
    components: &Components,
    inputs: &Inputs,
) {
    let file = component.document(DESIGN_FILE);
    let dir = component.document(DESIGN_DIR);
    let readme = component.document(DESIGN_README);
    match (
        inputs.present.contains(&file),
        inputs.present.contains(&dir),
    ) {
        (true, false) => {}
        (false, false) => out.push(Finding::in_file(
            &file,
            format!("the component `{}` carries no design home", component.name),
            "create docs/design.md, or docs/design/ with a README.md; every component \
             records its design in exactly one of the two",
        )),
        (true, true) => out.push(Finding::in_file(
            &dir,
            format!(
                "the component `{}` carries both `{}` and `{}`",
                component.name,
                file.display(),
                dir.display()
            ),
            "keep exactly one design home; with two, a decision lands in either and the \
             reader who finds the other acts on half the design",
        )),
        (false, true) => {
            // Without a head nothing can list the subdocuments, so the per-subdocument
            // findings would bury the one repair that fixes them all.
            if !inputs.present.contains(&readme) {
                out.push(Finding::in_file(
                    &readme,
                    format!("the design directory `{}` has no README.md", dir.display()),
                    "create it; the README is the directory home's head — an introduction, \
                     and a table of contents naming every subdocument as a backticked path",
                ));
                return;
            }
            let listed = listed_in(model, &readme, components);
            let mut subdocuments: Vec<&PathBuf> = inputs
                .present
                .iter()
                .filter(|p| {
                    p.starts_with(&dir) && **p != readme && p.extension().is_some_and(|e| e == "md")
                })
                .collect();
            subdocuments.sort();
            for subdocument in subdocuments {
                if !listed.contains(subdocument) {
                    out.push(Finding::in_file(
                        &readme,
                        format!(
                            "`{}` is not named in this README's table of contents",
                            subdocument.display()
                        ),
                        "name it as a backticked path, or delete the subdocument; a design \
                         subdocument nobody lists is a home nobody finds",
                    ));
                }
            }
        }
    }
}

/// Every path the README's backticked references resolve to.
///
/// Resolution mirrors `check::paths`: a bare reference resolves from the project root, a
/// `<component>@path` reference against that component's directory. A reference the paths
/// check would report — an unknown component, an `@`-ignored path — lists nothing here,
/// so a table of contents is made of checkable pointers or it does not count.
fn listed_in(model: &Model, readme: &PathBuf, components: &Components) -> HashSet<PathBuf> {
    let Some(doc) = model.documents().iter().find(|d| d.rel == *readme) else {
        return HashSet::new();
    };
    doc.observations
        .iter()
        .filter_map(|l| match &l.what {
            Observation::PathRef { component, path } => match component {
                None => Some(super::paths::normalise(std::path::Path::new(path))),
                Some(named) => components
                    .by_name(named)
                    .map(|c| super::paths::normalise(&c.path.join(path))),
            },
            _ => None,
        })
        .collect()
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
        declaring_full(components, additional, "[]", "[]", "[]", "[]")
    }

    /// The same, with every path list the manifest can declare spelled out.
    fn declaring_full(
        components: &str,
        additional: &str,
        skip_dirs: &str,
        skip_files: &str,
        exclude: &str,
        exempt: &str,
    ) -> Manifest {
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = [{components}]\n\
             additional-trackers = [{additional}]\n\n\
             [walk]\nskip-dirs = {skip_dirs}\nskip-files = {skip_files}\n\
             exclude = {exclude}\n\n\
             [lint]\nexempt-files = {exempt}\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The findings against a listing of what exists.
    fn findings(manifest: &Manifest, present: &[&str]) -> Vec<String> {
        findings_over(manifest, present, Model::from_documents(Vec::new()))
    }

    /// The same, with documents in the model — what the table-of-contents check reads.
    fn findings_over(manifest: &Manifest, present: &[&str], model: Model) -> Vec<String> {
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
        check(&model, manifest, &inputs)
            .0
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    /// Every document a component carries, under `dir`, with the file-shaped design home.
    fn all_of(dir: &str) -> Vec<String> {
        COMPONENT_DOCUMENTS
            .iter()
            .copied()
            .chain([DESIGN_FILE])
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
    fn a_declared_path_that_does_not_exist_is_reported_from_every_list() {
        // All four, because every one of them is now checkable: `.gitignore` carries what
        // cannot be — build output and directories made on demand — so a manifest names only
        // paths git tracks.
        let manifest = declaring_full(
            "",
            "",
            "[\"gone/dir\"]",
            "[\"gone/skipped.md\"]",
            "[\"gone/excluded\"]",
            "[\"gone/exempt.md\"]",
        );
        let present: Vec<String> = all_of("");
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 4, "one per list: {found:#?}");
        for name in [
            "gone/dir",
            "gone/skipped.md",
            "gone/excluded",
            "gone/exempt.md",
        ] {
            assert!(
                found.iter().any(|f| f.contains(name)),
                "{name} is unreported: {found:#?}"
            );
        }
    }

    #[test]
    fn a_declared_directory_that_exists_passes_even_when_it_holds_nothing() {
        // `docs/rules/past` is a declared skip that is legitimately empty in a fresh checkout,
        // so the test is existence and never file-ness.
        let manifest = declaring_full("", "", "[\"empty/dir\"]", "[]", "[]", "[]");
        let mut present: Vec<String> = all_of("");
        present.push("empty/dir".to_string());
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        assert_eq!(findings(&manifest, &present), Vec::<String>::new());
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
        let counts = check(&Model::from_documents(Vec::new()), &manifest, &inputs).1;
        assert_eq!(counts.components, 3);
        assert_eq!(counts.additional_trackers, 0);
    }

    // Interpolated, never spelled out: this tool's own source is walked, so a backticked
    // path written literally in a fixture is a real reference to a file this repository
    // does not have.
    const SUB_A: &str = "docs/design/one-subject.md";
    const SUB_B: &str = "docs/design/another-subject.md";

    /// The five shared documents without the design file, under `dir`.
    fn without_design(dir: &str) -> Vec<String> {
        all_of(dir)
            .into_iter()
            .filter(|p| !p.ends_with(DESIGN_FILE))
            .collect()
    }

    #[test]
    fn a_directory_design_home_with_a_readme_passes() {
        let manifest = declaring("");
        let mut present = without_design("");
        present.push(DESIGN_DIR.to_string());
        present.push(DESIGN_README.to_string());
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        assert_eq!(findings(&manifest, &present), Vec::<String>::new());
    }

    #[test]
    fn a_component_with_no_design_home_is_one_finding() {
        let manifest = declaring("");
        let present = without_design("");
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].starts_with(DESIGN_FILE), "{found:#?}");
        assert!(found[0].contains("no design home"), "{found:#?}");
    }

    #[test]
    fn both_design_homes_is_one_finding_naming_both() {
        // Two homes give a decision two candidate places to land, which is the failure the
        // one-home rule exists to prevent — so the finding names both, and the repair is a
        // choice rather than a creation.
        let manifest = declaring("");
        let mut present = all_of("");
        present.push(DESIGN_DIR.to_string());
        present.push(DESIGN_README.to_string());
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains(DESIGN_FILE) && found[0].contains("carries both"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_design_directory_without_a_readme_is_one_finding() {
        // One finding and not one per subdocument: without a head nothing can list them, so
        // the single repair is the README and the rest would bury it.
        let manifest = declaring("");
        let mut present = without_design("");
        present.push(DESIGN_DIR.to_string());
        present.push(SUB_A.to_string());
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no README.md"), "{found:#?}");
    }

    #[test]
    fn a_subdocument_the_readme_does_not_name_is_reported() {
        // One listed as a bare root-stem path, one not listed at all, and a non-markdown
        // asset that owes no listing.
        let manifest = declaring("");
        let mut present = without_design("");
        for p in [
            DESIGN_DIR,
            DESIGN_README,
            SUB_A,
            SUB_B,
            "docs/design/sketch.svg",
        ] {
            present.push(p.to_string());
        }
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let model = Model::from_documents(vec![(
            PathBuf::from(DESIGN_README),
            format!("# design\n\n- `{SUB_A}`\n"),
        )]);
        let found = findings_over(&manifest, &present, model);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].starts_with(DESIGN_README), "{found:#?}");
        assert!(
            found[0].contains(SUB_B) && found[0].contains("not named"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_subdocument_listed_in_the_component_relative_form_counts() {
        // The README may point at its sibling either way `check::paths` accepts, so the
        // resolution here has to agree with that check's.
        let part = "parts/a-part";
        let manifest = declaring(&format!("\"{part}\""));
        let mut present = all_of("");
        present.push(part.to_string());
        present.extend(without_design(part));
        for p in [DESIGN_DIR, DESIGN_README, SUB_A] {
            present.push(format!("{part}/{p}"));
        }
        let present: Vec<&str> = present.iter().map(String::as_str).collect();
        let model = Model::from_documents(vec![(
            PathBuf::from(format!("{part}/{DESIGN_README}")),
            format!("# design\n\n- `a-part@{SUB_A}`\n"),
        )]);
        assert_eq!(
            findings_over(&manifest, &present, model),
            Vec::<String>::new()
        );
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
