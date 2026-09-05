//! `knowledge.toml` — what a project declares must stay conformant, and what is exempt.
//!
//! Nothing about any particular repository is compiled into this tool. Every list a check
//! reads comes from here, so the same binary checks this repository and a mock project under
//! `knowledge@tests/projects/` with no special case anywhere, and a path that should not be checked has
//! to say so in one file with a reason beside it.
//!
//! The file's presence is also what makes a directory a project root. That is one mechanism
//! rather than two: a directory either declares itself a project or it does not, and the tool
//! refuses to run outside one instead of guessing a root from its own location.
//!
//! **Four registers are compiled in and the rest are declared.** `design`, `goal`, `tripwire`
//! and `issue` are what the word component means here, so a project neither adds nor removes
//! them; `[registers.<name>]` declares further ones, and `[locations.<name>]` names a
//! directory that carries a subset of them. The argument is `knowledge#registers-are-declared`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The file that both marks a project root and declares its conformance surface.
pub const MANIFEST_NAME: &str = "knowledge.toml";

/// The documents every component carries beside its registers, relative to its own directory.
///
/// Compiled in, unlike every other list a check reads. WHICH components a project has is that
/// project's own knowledge and is declared in `[project]`; WHAT a component is, is this tool's
/// definition of one. A project free to declare its own set could be conformant with anything,
/// which is the same as being checked against nothing.
///
/// The register homes are not in this list. Each is a register's own storage, in whichever
/// shape that register accepts, and `check::registers` asserts them from the register list.
pub const COMPONENT_DOCUMENTS: [&str; 3] =
    ["README.md", "CLAUDE.md", "docs/rejected-alternatives.md"];

/// The name of the built-in issue register, which is the one register that accepts a key.
pub const ISSUE_REGISTER: &str = "issue";

/// The built-in heading register of evidence that would flip a decision, by the name a
/// reference spells in kind position.
pub const TRIPWIRE_REGISTER: &str = "tripwire";

/// The issue register's compiled kind list. Closed: an unknown kind is a finding naming it.
pub const ISSUE_KINDS: [&str; 6] = [
    "defect",
    "observation",
    "question",
    "todo",
    "deferred",
    "design",
];

/// The level-three subsections an issue owes under `## Details`, in order.
pub const ISSUE_SUBSECTIONS: [&str; 3] = ["What", "Why it matters", "What would close it"];

/// The same for a `deferred` entry, which states its trigger where the others state closure.
pub const DEFERRED_SUBSECTIONS: [&str; 3] = ["What", "Why it matters", "Trigger"];

/// Where a register keeps its entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Entries are headings carrying a slug, in `<dir>.md` or in `<dir>/` behind a README.
    Heading,
    /// Entries are files `<id>.md` under `<dir>/`, beside a README and a generated index.
    File,
}

impl Shape {
    pub fn name(self) -> &'static str {
        match self {
            Shape::Heading => "heading",
            Shape::File => "file",
        }
    }
}

/// Which anchors carry a register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Every component carries it.
    Component,
    /// Only the locations that name it carry it.
    OptIn,
}

impl Scope {
    pub fn name(self) -> &'static str {
        match self {
            Scope::Component => "component",
            Scope::OptIn => "opt-in",
        }
    }
}

/// One register: a kind together with its storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    /// The word a reference spells in kind position.
    pub name: String,
    pub scope: Scope,
    pub shape: Shape,
    /// The basename of the home under the anchor's home base.
    pub dir: String,
    /// File shape: the level-two headings every entry carries, in order.
    pub sections: Vec<String>,
    /// File shape: each frontmatter key an entry carries, with its closed value set.
    pub metadata: Vec<(String, Vec<String>)>,
    /// The issue register alone: the kinds an entry may declare.
    pub kinds: Vec<String>,
    /// Whether this register is one of the four the tool compiles in.
    pub built_in: bool,
}

impl Register {
    /// The subsections an entry of this register owes under its last section, given the
    /// value of its first metadata key. Empty for every register but `issue`.
    pub fn owed_subsections(&self, kind: Option<&str>) -> &'static [&'static str] {
        if self.name != ISSUE_REGISTER {
            return &[];
        }
        match kind {
            Some("deferred") => &DEFERRED_SUBSECTIONS,
            _ => &ISSUE_SUBSECTIONS,
        }
    }
}

/// Every register of a project: the four built in, then the declared ones by name.
#[derive(Debug, Clone)]
pub struct Registers(Vec<Register>);

impl Registers {
    pub fn all(&self) -> &[Register] {
        &self.0
    }

    pub fn by_name(&self, name: &str) -> Option<&Register> {
        self.0.iter().find(|r| r.name == name)
    }

    /// Every register name, comma-separated, as a finding lists them.
    pub fn listed(&self) -> String {
        self.0
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The registers every component carries.
    pub fn component_scoped(&self) -> impl Iterator<Item = &Register> {
        self.0.iter().filter(|r| r.scope == Scope::Component)
    }

    /// The four compiled-in registers, before any declaration is read.
    fn built_in() -> Vec<Register> {
        let heading = |name: &str, dir: &str| Register {
            name: name.to_string(),
            scope: Scope::Component,
            shape: Shape::Heading,
            dir: dir.to_string(),
            sections: Vec::new(),
            metadata: Vec::new(),
            kinds: Vec::new(),
            built_in: true,
        };
        vec![
            heading("design", "design"),
            heading("goal", "goals"),
            heading(TRIPWIRE_REGISTER, "tripwires"),
            Register {
                name: ISSUE_REGISTER.to_string(),
                scope: Scope::Component,
                shape: Shape::File,
                dir: "open-issues".to_string(),
                sections: vec!["Summary".to_string(), "Details".to_string()],
                metadata: vec![(
                    "kind".to_string(),
                    ISSUE_KINDS.iter().map(|k| k.to_string()).collect(),
                )],
                kinds: ISSUE_KINDS.iter().map(|k| k.to_string()).collect(),
                built_in: true,
            },
        ]
    }
}

/// A register as the manifest declares it.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct RegisterDecl {
    pub scope: Option<String>,
    pub shape: Option<String>,
    pub dir: Option<String>,
    #[serde(default)]
    pub sections: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, MetadataDecl>,
    pub kinds: Option<Vec<String>>,
}

/// One frontmatter key an entry must carry, and the values it accepts.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct MetadataDecl {
    pub values: Vec<String>,
}

/// A named directory carrying a declared subset of the registers.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct LocationDecl {
    /// Project-relative.
    pub path: PathBuf,
    /// The registers it carries, by name.
    pub registers: Vec<String>,
}

/// What the project is called, and which directories are its components.
///
/// The project itself is a component — the one at the root — and it is not listed here: it
/// exists whatever this says, and it is named by `name` rather than by a basename it has none
/// of.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Project {
    /// What a reference names the component at the root by.
    pub name: String,
    /// One project-relative directory per component below the root, in declaration order.
    pub components: Vec<PathBuf>,
}

/// One component: a directory carrying its own documents and every component register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    /// The one word a reference names it by: the basename of its path, or the project's
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
/// of a declared one is derived from its path.
#[derive(Debug, Clone)]
pub struct Components(Vec<Component>);

impl Components {
    pub fn all(&self) -> &[Component] {
        &self.0
    }

    /// The component a document belongs to: the deepest declared path that holds it, and the
    /// component at the root where none does.
    pub fn owning(&self, rel: &Path) -> &Component {
        self.0
            .iter()
            .filter(|c| rel.starts_with(&c.path))
            .max_by_key(|c| c.path.components().count())
            .expect("the component at the root is a prefix of every path")
    }

    /// The component a reference names, or `None` where nothing declares that name.
    pub fn by_name(&self, name: &str) -> Option<&Component> {
        self.0.iter().find(|c| c.name == name)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Walk {
    /// Directories skipped by their project-relative PATH.
    ///
    /// **Not by name.** A bare name matched at any depth, so a component-local `past/` or
    /// `build/` was skipped by the walk AND dropped from the inverse assertion — invisible in
    /// both directions, which is where a fabricated quote goes unread.
    pub skip_dirs: Vec<PathBuf>,
    /// Files skipped by their project-relative PATH.
    ///
    /// **Not by name.** A bare name matched anywhere in the tree, so declaring the two
    /// generated indexes exempted every file called `index.md` — from the walk AND from the
    /// inverse assertion that an unwalked file may not name a rule. A fabricated rule quote in
    /// a hand-written index elsewhere in the tree was read by nothing and reported by nothing.
    pub skip_files: Vec<PathBuf>,
    /// Project-relative paths skipped by location rather than by name.
    #[serde(default)]
    pub exclude: Vec<PathBuf>,
}

#[cfg(test)]
impl Walk {
    /// A walk configuration for a unit test. It is not this project's — a test that cares
    /// about this project's declaration reads the real manifest.
    pub fn sample() -> Self {
        Self {
            skip_dirs: vec![PathBuf::from(".git")],
            skip_files: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Lint {
    /// Files exempt from the missing-marker lint, by their project-relative PATH.
    ///
    /// **Not by name.** A bare name matched anywhere, so a second file with the same basename
    /// inherited an exemption argued for one document. Quotes in these files are still
    /// verified.
    #[serde(default)]
    pub exempt_files: Vec<PathBuf>,
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
struct Declared {
    project: Project,
    #[serde(default)]
    locations: BTreeMap<String, LocationDecl>,
    #[serde(default)]
    registers: BTreeMap<String, RegisterDecl>,
    walk: Walk,
    lint: Lint,
    rules: Rules,
}

/// A project: where its root is, and what it declares.
#[derive(Debug, Clone)]
pub struct Manifest {
    root: PathBuf,
    declared: Declared,
    registers: Registers,
    /// What the register declarations got wrong, in the words `check::registers` reports.
    ///
    /// Held rather than returned, because a declaration this tool refuses to act on still has
    /// to produce a run: a manifest that would not load reports nothing at all, and nothing at
    /// all is what a session reads as conformance.
    register_complaints: Vec<String>,
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
    ///
    /// The ignore rules are not read here and are not this tool's to parse: git answers what it
    /// ignores, per `knowledge#git-supplies-the-walk`.
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
        retired_keys(text)?;
        let declared: Declared = toml::from_str(text).map_err(|e| e.to_string())?;
        let (registers, register_complaints) = build_registers(&declared.registers);
        Ok(Self {
            root: root.to_path_buf(),
            declared,
            registers,
            register_complaints,
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

    /// Every register this project has, the four built in first.
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// What the register declarations got wrong, for `check::registers` to report.
    pub fn register_complaints(&self) -> &[String] {
        &self.register_complaints
    }

    /// The locations this project declares, by name.
    pub fn locations(&self) -> &BTreeMap<String, LocationDecl> {
        &self.declared.locations
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

/// Refuse a manifest still written in the retired grammar, naming what replaces each key.
///
/// `deny_unknown_fields` would refuse both anyway, and its message names the key and nothing
/// else. A manifest is edited by hand once per project, so the one moment either key is met is
/// the moment the reader needs the replacement named.
fn retired_keys(text: &str) -> Result<(), String> {
    let value: toml::Value = toml::from_str(text).map_err(|e| e.to_string())?;
    if value.get("interpretations").is_some() {
        return Err("[interpretations] is retired: declare the register in \
                    [registers.<name>] with `scope`, `shape`, `dir`, `sections` and \
                    `metadata`, and give it a home with [locations.<name>]"
            .to_string());
    }
    if value
        .get("project")
        .and_then(|p| p.get("additional-trackers"))
        .is_some()
    {
        return Err(
            "[project] additional-trackers is retired: declare the directory as \
                    [locations.<name>] with `path` and the `registers` it carries"
                .to_string(),
        );
    }
    Ok(())
}

/// Fold the declarations into the compiled-in registers, and say what each one got wrong.
fn build_registers(declared: &BTreeMap<String, RegisterDecl>) -> (Registers, Vec<String>) {
    let mut out = Registers::built_in();
    let mut complaints = Vec::new();
    for (name, decl) in declared {
        let built_in = out.iter().position(|r| r.name == *name);
        if let Some(at) = built_in {
            // A built-in register's storage is what the word component means, so a
            // declaration may extend only what is extensible about it. `issue` extends its
            // kind list; the three heading registers extend nothing.
            let mut wrong: Vec<&str> = Vec::new();
            if decl.scope.is_some() {
                wrong.push("scope");
            }
            if decl.shape.is_some() {
                wrong.push("shape");
            }
            if decl.dir.is_some() {
                wrong.push("dir");
            }
            if !decl.sections.is_empty() {
                wrong.push("sections");
            }
            if !decl.metadata.is_empty() {
                wrong.push("metadata");
            }
            if !wrong.is_empty() {
                complaints.push(format!(
                    "[registers.{name}] sets {} on a built-in register",
                    wrong.join(", ")
                ));
            }
            match &decl.kinds {
                Some(kinds) if *name == ISSUE_REGISTER => {
                    out[at].kinds = kinds.clone();
                    out[at].metadata = vec![("kind".to_string(), kinds.clone())];
                }
                Some(_) => complaints.push(format!(
                    "[registers.{name}] sets kinds, which only [registers.{ISSUE_REGISTER}] takes"
                )),
                None => {}
            }
            continue;
        }
        if decl.kinds.is_some() {
            complaints.push(format!(
                "[registers.{name}] sets kinds, which only [registers.{ISSUE_REGISTER}] takes"
            ));
        }
        let scope = match decl.scope.as_deref() {
            Some("component") => Scope::Component,
            Some("opt-in") => Scope::OptIn,
            other => {
                complaints.push(format!(
                    "[registers.{name}] declares scope {}; a declared register takes \
                     `component` or `opt-in`",
                    other.unwrap_or("nothing")
                ));
                Scope::OptIn
            }
        };
        let shape = match decl.shape.as_deref() {
            Some("heading") => Shape::Heading,
            Some("file") => Shape::File,
            other => {
                complaints.push(format!(
                    "[registers.{name}] declares shape {}; a declared register takes \
                     `heading` or `file`",
                    other.unwrap_or("nothing")
                ));
                Shape::File
            }
        };
        if shape == Shape::Heading && (!decl.sections.is_empty() || !decl.metadata.is_empty()) {
            complaints.push(format!(
                "[registers.{name}] declares sections or metadata on a heading register, \
                 which has neither"
            ));
        }
        out.push(Register {
            name: name.clone(),
            scope,
            shape,
            dir: decl.dir.clone().unwrap_or_else(|| name.clone()),
            sections: decl.sections.clone(),
            metadata: decl
                .metadata
                .iter()
                .map(|(k, v)| (k.clone(), v.values.clone()))
                .collect(),
            kinds: Vec::new(),
            built_in: false,
        });
    }
    (Registers(out), complaints)
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

    /// A declaration with the components named, parsed from text.
    ///
    /// Written out rather than taken from a checkout: what is under test is what the file
    /// says, and the paths below need not exist for that.
    fn declaring(components: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = [{components}]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    #[test]
    fn this_repository_declares_a_readable_manifest() {
        let m = Manifest::load(&this_project()).expect("knowledge.toml");
        assert!(m.rules_tree().text().is_file());
        assert!(m.registers().by_name("design").is_some());
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
    fn the_four_built_in_registers_exist_before_anything_is_declared() {
        let m = declaring("");
        let names: Vec<&str> = m
            .registers()
            .all()
            .iter()
            .map(|r| r.name.as_str())
            .collect();
        assert_eq!(names, vec!["design", "goal", "tripwire", "issue"]);
        let issue = m.registers().by_name("issue").expect("the issue register");
        assert_eq!(issue.shape, Shape::File);
        assert_eq!(issue.dir, "open-issues");
        assert_eq!(issue.sections, vec!["Summary", "Details"]);
        assert_eq!(issue.kinds, ISSUE_KINDS);
        assert_eq!(
            m.registers()
                .by_name("tripwire")
                .expect("the tripwire register")
                .shape,
            Shape::Heading
        );
        assert!(m.register_complaints().is_empty());
    }

    /// A register declaration, folded into the built-in four.
    fn with_registers(body: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = []\n\n\
             {body}\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    #[test]
    fn a_declared_register_carries_its_scope_shape_dir_sections_and_metadata() {
        let m = with_registers(
            "[registers.interpretation]\nscope = \"opt-in\"\nshape = \"file\"\n\
             dir = \"interpretations\"\nsections = [\"Rules\", \"Reading\"]\n\n\
             [registers.interpretation.metadata.status]\nvalues = [\"settled\", \"ambiguous\"]\n",
        );
        let r = m
            .registers()
            .by_name("interpretation")
            .expect("the declared register");
        assert_eq!((r.scope, r.shape), (Scope::OptIn, Shape::File));
        assert_eq!(r.dir, "interpretations");
        assert_eq!(r.sections, vec!["Rules", "Reading"]);
        assert_eq!(
            r.metadata,
            vec![(
                "status".to_string(),
                vec!["settled".to_string(), "ambiguous".to_string()]
            )]
        );
        assert!(!r.built_in);
        assert!(
            m.register_complaints().is_empty(),
            "{:#?}",
            m.register_complaints()
        );
    }

    #[test]
    fn a_declared_register_with_no_scope_or_no_shape_is_a_complaint() {
        let m = with_registers("[registers.reading]\nshape = \"file\"\n");
        assert_eq!(m.register_complaints().len(), 1);
        assert!(
            m.register_complaints()[0].contains("scope"),
            "{:?}",
            m.register_complaints()
        );
        let m = with_registers("[registers.reading]\nscope = \"opt-in\"\n");
        assert!(
            m.register_complaints()[0].contains("shape"),
            "{:?}",
            m.register_complaints()
        );
    }

    #[test]
    fn a_built_in_register_accepts_kinds_on_issue_and_nothing_anywhere_else() {
        // The one extensible thing about a built-in, and the refusals beside it.
        let m = with_registers("[registers.issue]\nkinds = [\"defect\", \"todo\"]\n");
        assert!(
            m.register_complaints().is_empty(),
            "{:?}",
            m.register_complaints()
        );
        assert_eq!(
            m.registers().by_name("issue").expect("issue").kinds,
            vec!["defect", "todo"]
        );
        // Each of the five in turn: a declaration the tool then discards is configuration
        // that looks applied and is not, so each owes its own complaint.
        for (row, key) in [
            ("scope = \"opt-in\"", "scope"),
            ("shape = \"heading\"", "shape"),
            ("dir = \"issues\"", "dir"),
            ("sections = [\"One\"]", "sections"),
            (
                "[registers.issue.metadata.theme]\nvalues = [\"a\"]",
                "metadata",
            ),
        ] {
            let m = with_registers(&format!("[registers.issue]\n{row}\n"));
            assert_eq!(m.register_complaints().len(), 1, "{key}");
            assert!(
                m.register_complaints()[0].contains(key),
                "{key}: {:?}",
                m.register_complaints()
            );
        }
        let m = with_registers("[registers.issue]\ndir = \"issues\"\n");
        // The compiled storage stands whatever the declaration said.
        assert_eq!(
            m.registers().by_name("issue").expect("issue").dir,
            "open-issues"
        );
        assert_eq!(
            m.registers().by_name("issue").expect("issue").shape,
            Shape::File
        );
        let m = with_registers("[registers.design]\nkinds = [\"a\"]\n");
        assert_eq!(m.register_complaints().len(), 1);
        assert!(
            m.register_complaints()[0].contains("kinds"),
            "{:?}",
            m.register_complaints()
        );
    }

    #[test]
    fn a_declared_register_defaults_its_directory_to_its_own_name() {
        let m = with_registers("[registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\n");
        assert_eq!(
            m.registers().by_name("reading").expect("reading").dir,
            "reading"
        );
    }

    #[test]
    fn a_location_declares_its_path_and_the_registers_it_carries() {
        let m = with_registers(
            "[locations.agent-config]\npath = \".claude\"\nregisters = [\"issue\"]\n",
        );
        let l = m.locations().get("agent-config").expect("the location");
        assert_eq!(l.path, PathBuf::from(".claude"));
        assert_eq!(l.registers, vec!["issue"]);
    }

    #[test]
    fn the_two_retired_keys_are_refused_by_name_with_their_replacement() {
        // `deny_unknown_fields` refuses both anyway and names neither replacement. A manifest
        // is migrated once, and the message is the whole of what the migrator gets.
        let base = "[project]\nname = \"a\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        let e = Manifest::parse(
            Path::new("/nowhere"),
            &format!("{base}\n[interpretations]\ndir = \"i\"\nconcerns = []\n"),
        )
        .expect_err("the retired table");
        assert!(
            e.contains("[registers.") && e.contains("[locations."),
            "{e}"
        );
        let with_trackers = base.replace(
            "components = []",
            "components = []\nadditional-trackers = [\"x/open-issues.md\"]",
        );
        let e =
            Manifest::parse(Path::new("/nowhere"), &with_trackers).expect_err("the retired key");
        assert!(e.contains("[locations."), "{e}");
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
