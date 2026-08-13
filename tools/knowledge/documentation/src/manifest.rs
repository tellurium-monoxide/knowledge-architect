//! `knowledge.toml` — what a project declares must stay conformant, and what is exempt.
//!
//! Nothing about any particular repository is compiled into this tool. Every list a check
//! reads comes from here, so the same binary checks this repository and a mock project under
//! `tests/projects/` with no special case anywhere, and a path that should not be checked has
//! to say so in one file with a reason beside it.
//!
//! The file's presence is also what makes a directory a project root. That is one mechanism
//! rather than two: a directory either declares itself a project or it does not, and the tool
//! refuses to run outside one instead of guessing a root from its own location.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The file that both marks a project root and declares its conformance surface.
pub const MANIFEST_NAME: &str = "knowledge.toml";

/// The documents every component carries, relative to the component's own directory.
///
/// Compiled in, unlike every other list a check reads. WHICH components a project has is that
/// project's own knowledge and is declared in `[project]`; WHAT a component is, is this tool's
/// definition of one. A project free to declare its own set could be conformant with anything,
/// which is the same as being checked against nothing.
pub const COMPONENT_DOCUMENTS: [&str; 6] = [
    "README.md",
    "CLAUDE.md",
    "docs/design.md",
    "docs/rejected-alternatives.md",
    "docs/open-issues.md",
    "docs/tripwires.md",
];

/// The component documents that carry outstanding state, which `outstanding` reads.
///
/// A subset of `COMPONENT_DOCUMENTS`, asserted as one by a test beside it: a tracker that is
/// not a document every component carries would be read by the report and checked by nothing.
pub const COMPONENT_TRACKERS: [&str; 2] = ["docs/open-issues.md", "docs/tripwires.md"];

/// What the project is called, and which directories are its components.
///
/// The project itself is a component — the one at the root — and it is not listed here: it
/// exists whatever this says, and it is named by `name` rather than by a basename it has none
/// of.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Project {
    /// What a slug reference names the component at the root by.
    pub name: String,
    /// One project-relative directory per component below the root, in declaration order.
    pub components: Vec<PathBuf>,
    /// Tracker files that belong to no component, each a project-relative path to the file.
    ///
    /// A directory may carry outstanding state without being a component — `.claude/` holds
    /// what is open about the agent configuration, and there is no library or binary there to
    /// have a README, a design or a rejected-alternatives document. Listing the file directly
    /// is what lets `outstanding` read it while nothing asks that directory for the rest.
    #[serde(default)]
    pub additional_trackers: Vec<PathBuf>,
}

/// One component: a directory carrying its own documents and its own trackers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    /// The one word a slug reference names it by: the basename of its path, or the project's
    /// name for the component at the root.
    pub name: String,
    /// Project-relative, and empty for the component at the root.
    pub path: PathBuf,
}

impl Component {
    /// Whether this is the component at the project root.
    pub fn is_root(&self) -> bool {
        self.path.as_os_str().is_empty()
    }

    /// The project-relative path of one of the documents this component carries.
    pub fn document(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

/// Every component of a project: the one at the root first, then the declared ones in
/// declaration order.
///
/// Built rather than stored, because the root component is not declared anywhere and the name
/// of a declared one is derived from its path. Two lookups are all any check needs: which
/// component a document belongs to, and which component a reference names.
#[derive(Debug, Clone)]
pub struct Components(Vec<Component>);

impl Components {
    pub fn all(&self) -> &[Component] {
        &self.0
    }

    /// The component a document belongs to: the deepest declared path that holds it, and the
    /// component at the root where none does.
    ///
    /// Deepest rather than first, so a component nested inside another owns its own documents.
    /// The root component's path is empty and is therefore a prefix of every path, which is
    /// what makes it the fallback rather than a case.
    pub fn owning(&self, rel: &Path) -> &Component {
        self.0
            .iter()
            .filter(|c| rel.starts_with(&c.path))
            .max_by_key(|c| c.path.components().count())
            .expect("the component at the root is a prefix of every path")
    }

    /// The component a reference names, or `None` where nothing declares that name.
    ///
    /// The first match, so a name declared twice resolves to the first of them. That the name
    /// is ambiguous at all is reported on its own, by the check over the declaration.
    pub fn by_name(&self, name: &str) -> Option<&Component> {
        self.0.iter().find(|c| c.name == name)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Walk {
    pub suffixes: Vec<String>,
    pub skip_dirs: Vec<String>,
    pub skip_files: Vec<String>,
    /// Project-relative paths skipped by location rather than by name.
    #[serde(default)]
    pub exclude: Vec<PathBuf>,
}

#[cfg(test)]
impl Walk {
    /// A walk configuration for a unit test that only needs to know which suffixes carry
    /// comment leaders. It is not this project's — a test that cares about this project's
    /// declaration reads the real manifest.
    pub fn sample() -> Self {
        Self {
            suffixes: ["md", "rs", "py", "sh", "toml"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            skip_dirs: vec![".git".to_string()],
            skip_files: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Lint {
    /// Exempt from the missing-marker lint only. Quotes in these files are still verified.
    #[serde(default)]
    pub exempt_files: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Rules {
    pub dir: PathBuf,
    pub text: PathBuf,
    pub body_starts_at: usize,
    pub version: PathBuf,
    pub past: PathBuf,
    pub manifest: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Interpretations {
    pub dir: PathBuf,
    /// The concerns the register is partitioned by, one file each.
    ///
    /// Listed rather than derived from anything. A partition read off another artifact
    /// cannot diverge from it without editing that artifact, and here that artifact is the
    /// frozen survey. The check compares this list against the directory in both directions,
    /// so a file with no entry and an entry with no file are both failures.
    pub concerns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct Declared {
    project: Project,
    walk: Walk,
    lint: Lint,
    rules: Rules,
    interpretations: Interpretations,
}

/// A project: where its root is, and what it declares.
#[derive(Debug, Clone)]
pub struct Manifest {
    root: PathBuf,
    declared: Declared,
}

impl Manifest {
    /// The nearest project root at or above `start`, and its manifest.
    ///
    /// Walking up rather than reading a compiled-in path is what lets the tool be pointed at
    /// a mock project: the mock carries its own manifest and is therefore its own root.
    pub fn find(start: &Path) -> Result<Self, String> {
        let root = start
            .ancestors()
            .find(|dir| dir.join(MANIFEST_NAME).is_file())
            .ok_or_else(|| {
                format!(
                    "no {MANIFEST_NAME} at or above {}\n       \
                     a project declares its conformance surface in that file; \
                     run this from inside one",
                    start.display()
                )
            })?;
        Self::load(root)
    }

    /// Read a manifest from a known root.
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join(MANIFEST_NAME);
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::parse(root, &text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Parse a declaration from text, against a root that is not read.
    ///
    /// The half of `load` that touches no filesystem, so a test over what a project declares
    /// needs no checkout — the same reason a check is a pure function over the model.
    pub fn parse(root: &Path, text: &str) -> Result<Self, String> {
        let declared: Declared = toml::from_str(text).map_err(|e| e.to_string())?;
        Ok(Self {
            root: root.to_path_buf(),
            declared,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn project(&self) -> &Project {
        &self.declared.project
    }

    pub fn walk(&self) -> &Walk {
        &self.declared.walk
    }

    pub fn lint(&self) -> &Lint {
        &self.declared.lint
    }

    pub fn rules(&self) -> &Rules {
        &self.declared.rules
    }

    pub fn interpretations(&self) -> &Interpretations {
        &self.declared.interpretations
    }

    /// Every component, the one at the root first.
    pub fn components(&self) -> Components {
        let mut all = vec![Component {
            name: self.declared.project.name.clone(),
            path: PathBuf::new(),
        }];
        all.extend(self.declared.project.components.iter().map(|path| {
            Component {
                // The basename, which is what a reference spells. A path with no basename — `.`,
                // or an empty string — yields an empty name, which the check over the declaration
                // reports as a name no reference can be written with.
                name: path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                path: path.clone(),
            }
        }));
        Components(all)
    }

    /// The project-relative path of every tracker file in the project.
    ///
    /// Two of every component's documents are its trackers, and `additional-trackers` names
    /// the ones that belong to no component. Nothing is discovered by filename: a file called
    /// `open-issues.md` that is neither is not a tracker, so nothing counts it and nothing
    /// reports it.
    pub fn tracker_paths(&self) -> Vec<PathBuf> {
        let components = self.components();
        let mut out: Vec<PathBuf> = components
            .all()
            .iter()
            .flat_map(|c| COMPONENT_TRACKERS.iter().map(|n| c.document(n)))
            .collect();
        // Deduplicated, keeping the first of each: a file a component already carries and that
        // is also listed as an additional tracker would otherwise be counted twice in the
        // report's total, which is the one number this list exists to make trustworthy.
        for extra in &self.declared.project.additional_trackers {
            if !out.contains(extra) {
                out.push(extra.clone());
            }
        }
        out
    }

    /// The rules corpus as `rules::Tree` needs it, with every path already resolved.
    ///
    /// The `rules` library is handed explicit paths rather than the manifest, so it keeps
    /// knowing nothing about how a project is laid out or where its declaration lives.
    pub fn rules_tree(&self) -> rules::Tree {
        let dir = self.root.join(&self.declared.rules.dir);
        rules::Tree::new(
            &self.root,
            dir.join(&self.declared.rules.text),
            dir.join(&self.declared.rules.version),
            dir.join(&self.declared.rules.past),
            dir.join(&self.declared.rules.manifest),
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// The manifest of the checkout these tests were compiled from.
    ///
    /// `CARGO_MANIFEST_DIR` is legitimate here and nowhere else: it is baked in at compile
    /// time, which is wrong for a binary and exactly right for a test that only ever runs
    /// against the tree it was built from.
    pub fn this_project() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .expect("documentation/ sits three levels below the project root")
            .to_path_buf()
    }

    /// A declaration with two components below the root, parsed from text.
    ///
    /// Written out rather than taken from a checkout: what is under test is what the file
    /// says, and the paths below need not exist for that.
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

    #[test]
    fn this_repository_declares_a_readable_manifest() {
        let m = Manifest::load(&this_project()).expect("knowledge.toml");
        assert!(m.walk().suffixes.contains(&"md".to_string()));
        assert!(m
            .tracker_paths()
            .iter()
            .any(|p| p.ends_with("open-issues.md")));
        assert!(m.rules_tree().text().is_file());
    }

    #[test]
    fn the_component_at_the_root_exists_whether_or_not_anything_is_declared() {
        let m = declaring("");
        let components = m.components();
        assert_eq!(components.all().len(), 1);
        let root = &components.all()[0];
        assert!(root.is_root());
        // Named by the project, because it has no basename to be named by.
        assert_eq!(root.name, "a-project");
    }

    #[test]
    fn a_declared_component_is_named_by_the_basename_of_its_path() {
        let m = declaring("\"crates/an-engine\", \"tools/a-tool\"");
        let components = m.components();
        let names: Vec<&str> = components.all().iter().map(|c| c.name.as_str()).collect();
        // The root first, then declaration order.
        assert_eq!(names, vec!["a-project", "an-engine", "a-tool"]);
    }

    #[test]
    fn a_document_belongs_to_the_deepest_component_that_holds_it() {
        // Nested on purpose: first-match would give the outer component the inner one's
        // documents, and every slug defined inside the inner one would then answer to the
        // wrong name.
        let m = declaring("\"crates\", \"crates/an-engine\"");
        let components = m.components();
        let owner = |p: &str| components.owning(Path::new(p)).name.clone();
        assert_eq!(owner("crates/an-engine/docs/design.md"), "an-engine");
        assert_eq!(owner("crates/other/docs/design.md"), "crates");
        // Under no declared component: the root's, which is the fallback rather than a case.
        assert_eq!(owner("docs/design.md"), "a-project");
        assert_eq!(owner("README.md"), "a-project");
    }

    #[test]
    fn a_component_is_found_by_the_name_a_reference_spells() {
        let m = declaring("\"crates/an-engine\"");
        let components = m.components();
        assert_eq!(
            components.by_name("an-engine").map(|c| c.path.clone()),
            Some(PathBuf::from("crates/an-engine"))
        );
        assert!(components.by_name("a-project").is_some_and(|c| c.is_root()));
        assert!(
            components.by_name("crates").is_none(),
            "the path is not the name"
        );
    }

    #[test]
    fn a_tracker_belonging_to_no_component_is_listed_where_it_is_declared() {
        let m = declaring_with(
            "\"crates/an-engine\"",
            "\"a-directory/open-issues.md\", \"docs/open-issues.md\"",
        );
        let paths: Vec<String> = m
            .tracker_paths()
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        assert!(
            paths.contains(&"a-directory/open-issues.md".to_string()),
            "{paths:#?}"
        );
        // The second one is already the root component's own, and is listed once rather than
        // twice: the report's total counts this list.
        assert_eq!(
            paths.iter().filter(|p| *p == "docs/open-issues.md").count(),
            1,
            "{paths:#?}"
        );
    }

    #[test]
    fn every_tracker_is_one_of_the_documents_a_component_carries() {
        // A tracker outside that set would be read by `outstanding` and checked by nothing.
        for tracker in COMPONENT_TRACKERS {
            assert!(COMPONENT_DOCUMENTS.contains(&tracker), "{tracker}");
        }
        let m = declaring("\"crates/an-engine\"");
        let paths: Vec<String> = m
            .tracker_paths()
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        assert_eq!(
            paths,
            vec![
                "docs/open-issues.md",
                "docs/tripwires.md",
                "crates/an-engine/docs/open-issues.md",
                "crates/an-engine/docs/tripwires.md",
            ]
        );
    }

    #[test]
    fn the_walk_up_finds_the_root_from_below_it() {
        let root = this_project();
        let deep = root.join("tools/knowledge/documentation/src");
        assert_eq!(Manifest::find(&deep).expect("a manifest").root(), root);
    }

    #[test]
    fn a_directory_outside_any_project_is_an_error_and_says_what_is_missing() {
        let e = Manifest::find(Path::new("/")).expect_err("no project at /");
        assert!(e.contains(MANIFEST_NAME), "{e}");
    }

    #[test]
    fn an_unknown_key_is_rejected_rather_than_ignored() {
        // A misspelled key that parsed and did nothing would be an exclusion silently not
        // applied, which is the failure mode this whole file exists to make impossible.
        let dir = std::env::temp_dir().join("knowledge-manifest-unknown-key");
        std::fs::create_dir_all(&dir).expect("a temp dir");
        std::fs::write(dir.join(MANIFEST_NAME), "[walk]\nsuffix = []\n").expect("write");
        let e = Manifest::load(&dir).expect_err("unknown key");
        assert!(e.contains("suffix"), "{e}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
