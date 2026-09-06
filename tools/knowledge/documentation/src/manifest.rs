//! `knowledge.toml` — what a project declares must stay conformant, and what is exempt.
//!
//! Nothing about any particular repository is compiled into this tool. Every list a check
//! reads comes from here, so the same binary checks this repository and a mock project under
//! `path@knowledge@tests/projects/` with no special case anywhere, and a path that should not be checked has
//! to say so in one file with a reason beside it.
//!
//! The file's presence is also what makes a directory a project root. That is one mechanism
//! rather than two: a directory either declares itself a project or it does not, and the tool
//! refuses to run outside one instead of guessing a root from its own location.
//!
//! **Four registers are compiled in and the rest are declared.** `design`, `goal`, `tripwire`
//! and `issue` are what the word component means here, so a project neither adds nor removes
//! them; `[registers.<name>]` declares further ones, and `[locations.<name>]` names a
//! directory that carries a subset of them. The argument is `design@knowledge@registers-are-declared`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::finding::Finding;

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
    /// Every declaration this tool refused, in the shape `check` reports.
    ///
    /// **A refused declaration is absent from the configuration**, so nothing acts on it: a
    /// refused register is not a register, a refused anchor is not an anchor, a refused row is
    /// not in its list. Held rather than returned, because a manifest that would not load
    /// reports nothing at all, and nothing at all is what a session reads as conformance. The
    /// one table with no default is `[rules]`, whose refusal fails the load instead.
    complaints: Vec<Finding>,
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
    /// ignores, per `design@knowledge@git-supplies-the-walk`.
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
        let mut declared: Declared = toml::from_str(text).map_err(|e| e.to_string())?;
        let mut complaints = Vec::new();
        let mut registers = build_registers(&declared.registers, &mut complaints);
        resolve_registers(&mut registers, &mut complaints);
        normalise_paths(&mut declared, &mut complaints)?;
        resolve_anchors(&mut declared, &registers, &mut complaints);
        Ok(Self {
            root: root.to_path_buf(),
            declared,
            registers,
            complaints,
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

    /// Every declaration the tool refused, for `check` to report first.
    pub fn complaints(&self) -> &[Finding] {
        &self.complaints
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
/// Every path the manifest declares, spelled the way git lists it.
///
/// **A declared path is compared against git's listing, and git spells a path with no `.`
/// segment, no `..` segment and no leading `/`.** A row spelled otherwise matched nothing, so
/// a location declared with `path = "./docs"` was reported absent while `docs` existed, and
/// a `..` segment was read by the nesting check as one more component of the path. A `.`
/// segment is dropped, so a dot-spelled and a plain `docs` are one declaration; a `..`
/// segment and an absolute path are refused, because what they name depends on where the
/// manifest sits rather than on the tree. A refused row leaves its list and is reported, so it
/// is acted on by nothing.
///
/// **A refused `[rules]` path fails the load.** The corpus has no list to drop a row from and
/// no default to stand in, and a path kept as spelled was joined and read: `index` once wrote
/// through a `..` corpus directory into a sibling tree.
fn normalise_paths(declared: &mut Declared, complaints: &mut Vec<Finding>) -> Result<(), String> {
    let project = &mut declared.project;
    normalise_list("[project] components", &mut project.components, complaints);
    declared.locations.retain(|name, decl| {
        normalise_one(
            &format!("[locations.{name}] path"),
            &mut decl.path,
            complaints,
        )
    });
    let walk = &mut declared.walk;
    normalise_list("[walk] skip-dirs", &mut walk.skip_dirs, complaints);
    normalise_list("[walk] skip-files", &mut walk.skip_files, complaints);
    normalise_list("[walk] exclude", &mut walk.exclude, complaints);
    normalise_list(
        "[lint] exempt-files",
        &mut declared.lint.exempt_files,
        complaints,
    );
    let rules = &mut declared.rules;
    for (key, path) in [
        ("dir", &mut rules.dir),
        ("text", &mut rules.text),
        ("version", &mut rules.version),
        ("past", &mut rules.past),
        ("manifest", &mut rules.manifest),
    ] {
        let mut refused = Vec::new();
        if !normalise_one(&format!("[rules] {key}"), path, &mut refused) {
            return Err(format!(
                "{}; the corpus has no default to stand in for it",
                refused[0].what
            ));
        }
    }
    Ok(())
}

/// Normalise every path of one list, dropping the rows that are refused.
fn normalise_list(list: &str, paths: &mut Vec<PathBuf>, complaints: &mut Vec<Finding>) {
    paths.retain_mut(|path| normalise_one(list, path, complaints));
}

/// Normalise one path in place, or record why it is refused and say so with `false`.
fn normalise_one(list: &str, path: &mut PathBuf, complaints: &mut Vec<Finding>) -> bool {
    use std::path::Component as Segment;
    let mut out = PathBuf::new();
    for segment in path.components() {
        let why = match segment {
            Segment::CurDir => continue,
            Segment::Normal(name) => {
                out.push(name);
                continue;
            }
            Segment::ParentDir => "spells a `..` segment",
            Segment::RootDir | Segment::Prefix(_) => "is not project-relative",
        };
        complaints.push(Finding::in_file(
            MANIFEST_NAME,
            format!("`{}` in {list} {why}", path.display()),
            "spell the path the way git lists it, relative to the manifest's directory with \
             no `..` and no leading `/`; the row is acted on by nothing until then",
        ));
        return false;
    }
    *path = out;
    true
}

/// A declared register is one a reference can name, at a home inside its anchor.
///
/// A register's name is what a reference spells in kind position, so a name outside the id
/// grammar is a register nothing can point at, and `path` is a name the resolver answers
/// before it ever reaches the register list. Its home is `<home base>/<dir>`, so a `dir` that
/// is not one plain segment puts the home somewhere the anchor does not reach, and a `dir`
/// spelling a compiled document makes one file both the document and the home. Each is a
/// complaint, and the register is not declared. The built-in four are named and placed by
/// this tool and are never refused here.
fn resolve_registers(registers: &mut Registers, complaints: &mut Vec<Finding>) {
    registers.0.retain(|register| {
        if register.built_in {
            return true;
        }
        let refusal = if register.name == crate::entity::PATH_KIND {
            Some((
                format!(
                    "`{}` is the reserved kind of a file or directory",
                    register.name
                ),
                "rename the register; a reference whose kind segment is this word resolves \
                 against the tree and never reaches the register",
            ))
        } else if !crate::entity::is_entity_id(&register.name) {
            Some((
                format!(
                    "`{}` cannot be spelled in a reference's kind segment",
                    register.name
                ),
                "name it in lower-case words joined by hyphens; a register nothing can point \
                 at is one every pointer misses in silence",
            ))
        } else if !crate::entity::is_entity_id(&register.dir) {
            Some((
                format!(
                    "the {} register's directory `{}` is not one plain segment",
                    register.name, register.dir
                ),
                "name it in lower-case words joined by hyphens; a `/` or a `..` puts the \
                 home outside the anchor, and the real one is then read by nothing",
            ))
        } else if register.scope == Scope::Component
            && COMPONENT_DOCUMENTS.contains(&format!("docs/{}.md", register.dir).as_str())
        {
            // Every heading register has the file shape, so the `.md` is what collides; the
            // compiled documents are all files, so the directory shape collides with none.
            Some((
                format!(
                    "the {} register's directory `{}` makes its home the compiled document \
                     `docs/{}.md`",
                    register.name, register.dir, register.dir
                ),
                "give the register another directory; a compiled document is what every \
                 component carries, and a file that is also a register home means two things",
            ))
        } else {
            None
        };
        match refusal {
            Some((what, action)) => {
                complaints.push(Finding::in_file(MANIFEST_NAME, what, action));
                false
            }
            None => true,
        }
    });
    // Two component registers at one directory: the first one asked would answer for every
    // entry in the shared home, and every reference of the other kind would dangle for ever
    // with nothing said. The later-declared one is refused; the built-in four come first.
    let mut taken: Vec<(String, String)> = Vec::new();
    registers.0.retain(|register| {
        if register.scope != Scope::Component {
            return true;
        }
        if let Some((other, _)) = taken.iter().find(|(_, dir)| *dir == register.dir) {
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "the {} and {other} registers share the home directory `{}`",
                    register.name, register.dir
                ),
                "give each register a directory of its own; one home answers for one \
                 register, and the other's references resolve to nothing",
            ));
            return false;
        }
        taken.push((register.name.clone(), register.dir.clone()));
        true
    });
}

/// One anchor as the declaration names it, before it is accepted.
struct Candidate {
    name: String,
    path: PathBuf,
    is_component: bool,
    /// Every register directory the anchor would carry, with the register's name.
    homes: Vec<(String, PathBuf)>,
    /// Where the declaration sits, as a complaint names it.
    declared_at: String,
}

impl Candidate {
    fn depth(&self) -> usize {
        self.path.components().count()
    }

    fn kind(&self) -> &'static str {
        if self.is_component {
            "component"
        } else {
            "location"
        }
    }
}

/// No two anchors give one path two meanings, and every anchor is one a reference can name.
///
/// An anchor is a directory, and its register homes sit under it. The deepest anchor owns
/// every document under its path, per `design@knowledge@every-path-names-its-anchor`, so an
/// anchor inside another's directory is the ordinary case — this repository's two locations
/// both sit inside the root component. What that rule cannot absorb is refused here, per
/// `design@knowledge@anchors-are-components-and-locations`, and **the refused anchor is not an
/// anchor**: it owns nothing, carries nothing and is asserted against nothing, so the
/// consequences of the declaration are not reported as defects of the tree.
///
/// Candidates are taken shallowest first, a component before a location at equal depth,
/// declaration order among components and name order among locations after that — a
/// location's declaration order is not kept, the table being keyed by name — and each is
/// judged against the anchors already accepted. So of two anchors that collide, the deeper
/// one, or the later one in that order, is the one refused:
///
/// - **a name no reference can spell, or a reserved one, or one an accepted anchor already
///   has**: every pointer at it would miss or read as the other.
/// - **the root's path, spelled by a component or a location**: the root is a component and
///   carries every register already.
/// - **a component inside a location**: it nests a full register set inside a partial one.
/// - **the same path as an accepted anchor**: nothing decides which owns the documents under it.
/// - **inside, or at, an accepted anchor's register directory**: every file under it would
///   be read as an entry or a subdocument of that register.
/// - **holding an accepted anchor's register directory**: being deeper, it would own every
///   document in that home, and each slug defined there would be misplaced.
fn resolve_anchors(declared: &mut Declared, registers: &Registers, complaints: &mut Vec<Finding>) {
    let component_homes = |path: &Path| -> Vec<(String, PathBuf)> {
        registers
            .component_scoped()
            .map(|r| (r.name.clone(), path.join("docs").join(&r.dir)))
            .collect()
    };
    let mut candidates: Vec<Candidate> = vec![Candidate {
        name: declared.project.name.clone(),
        path: PathBuf::new(),
        is_component: true,
        homes: component_homes(Path::new("")),
        declared_at: "[project] name".to_string(),
    }];
    for path in &declared.project.components {
        candidates.push(Candidate {
            name: path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default(),
            path: path.clone(),
            is_component: true,
            homes: component_homes(path),
            declared_at: format!("[project] components row `{}`", path.display()),
        });
    }
    for (name, decl) in &mut declared.locations {
        // A location carries the registers it names, and a name that is no register is a
        // complaint against the row rather than a register the location silently lacks.
        let mut dirs: Vec<(String, String)> = Vec::new();
        decl.registers.retain(|register| {
            let Some(known) = registers.by_name(register) else {
                complaints.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!("[locations.{name}] carries `{register}`, which is no register"),
                    format!(
                        "declare it in [registers.{register}], or name one of {}",
                        registers.listed()
                    ),
                ));
                return false;
            };
            // Two of its registers at one directory: the same collision the component
            // registers are refused for, per location.
            if let Some((other, _)) = dirs.iter().find(|(_, dir)| *dir == known.dir) {
                complaints.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!(
                        "[locations.{name}] carries the {} and {other} registers at one \
                         directory, `{}`",
                        known.name, known.dir
                    ),
                    "give each register a directory of its own; one home answers for one \
                     register, and the other's references resolve to nothing",
                ));
                return false;
            }
            dirs.push((known.name.clone(), known.dir.clone()));
            true
        });
        candidates.push(Candidate {
            name: name.clone(),
            path: decl.path.clone(),
            is_component: false,
            homes: decl
                .registers
                .iter()
                .filter_map(|r| registers.by_name(r))
                .map(|r| (r.name.clone(), decl.path.join(&r.dir)))
                .collect(),
            declared_at: format!("[locations.{name}]"),
        });
    }
    // Stable, so among equals the order the candidates were pushed in decides: components in
    // declaration order, then locations in name order.
    candidates.sort_by_key(|c| (c.depth(), !c.is_component));

    let mut accepted: Vec<Candidate> = Vec::new();
    for candidate in candidates {
        let mut refuse = |what: String, action: &str| {
            complaints.push(Finding::in_file(MANIFEST_NAME, what, action));
        };
        if !crate::entity::is_anchor_name(&candidate.name) {
            refuse(
                format!(
                    "{} gives the anchor the name `{}`, which cannot be spelled in a reference",
                    candidate.declared_at, candidate.name
                ),
                "name it in letters, digits, `.`, `-` and `_`; an anchor nothing can point \
                 at is one every pointer misses in silence",
            );
            continue;
        }
        // The reserved anchors are compiled in, so an anchor wearing one could never be the
        // target of a path reference: every pointer at it would read as the reserved meaning.
        if candidate.name == crate::entity::ESCAPE_ANCHOR
            || candidate.name == crate::entity::EVERY_ANCHOR
        {
            refuse(
                format!(
                    "{} gives the anchor the reserved name `{}`",
                    candidate.declared_at, candidate.name
                ),
                "rename it; this word is reserved by the path kind",
            );
            continue;
        }
        if let Some(other) = accepted.iter().find(|a| a.name == candidate.name) {
            refuse(
                format!(
                    "`{}` names 2 anchors: {} and {}",
                    candidate.name,
                    place(other),
                    place(&candidate)
                ),
                "rename or move one; a reference names its anchor by that one word, so two \
                 of them cannot share it",
            );
            continue;
        }
        if candidate.path.as_os_str().is_empty() && !accepted.is_empty() {
            refuse(
                format!("{} names the project root", candidate.declared_at),
                "give it a directory of its own; the root is a component, and it carries \
                 every register already",
            );
            continue;
        }
        let collision = accepted
            .iter()
            .find_map(|outer| collides(outer, &candidate));
        if let Some((what, action)) = collision {
            refuse(what, action);
            continue;
        }
        accepted.push(candidate);
    }

    declared
        .project
        .components
        .retain(|path| accepted.iter().any(|a| a.is_component && a.path == *path));
    declared
        .locations
        .retain(|name, _| accepted.iter().any(|a| !a.is_component && a.name == *name));
}

/// How a complaint names an anchor's place: its path, or the root.
fn place(candidate: &Candidate) -> String {
    if candidate.path.as_os_str().is_empty() {
        "the project root".to_string()
    } else {
        format!("`{}`", candidate.path.display())
    }
}

/// Why `inner`, which is at least as deep as `outer`, cannot be an anchor beside it.
fn collides(outer: &Candidate, inner: &Candidate) -> Option<(String, &'static str)> {
    if !inner.path.starts_with(&outer.path) {
        return None;
    }
    if inner.path == outer.path {
        return Some((
            format!(
                "two anchors sit at `{}`: `{}`, `{}`",
                inner.path.display(),
                outer.name,
                inner.name
            ),
            "move one; the deepest anchor owns every document under its path, and at one \
             path nothing decides which of the two that is",
        ));
    }
    if inner.is_component && !outer.is_component {
        return Some((
            format!(
                "the component `{}` sits inside the location `{}`",
                inner.name, outer.name
            ),
            "move one out of the other; a location carries a subset of the registers and a \
             component carries them all, so nesting gives one document two homes",
        ));
    }
    let mut held: Vec<&str> = Vec::new();
    for (register, home) in &outer.homes {
        if inner.path.starts_with(home) {
            return Some((
                format!(
                    "the {} `{}` sits inside the {register} register's directory `{}` of `{}`",
                    inner.kind(),
                    inner.name,
                    home.display(),
                    outer.name
                ),
                "move it out; the directory is the register's home in one shape and reserved \
                 in the other, and every file under the anchor would be read as an entry or a \
                 subdocument of it",
            ));
        }
        if home.starts_with(&inner.path) {
            held.push(register);
        }
    }
    if held.is_empty() {
        return None;
    }
    Some((
        format!(
            "the {} `{}` at `{}` holds the {} of `{}`",
            inner.kind(),
            inner.name,
            inner.path.display(),
            list_homes(&held),
            outer.name
        ),
        "move it out; the deepest anchor owns every document under its path, so each slug \
         defined in those homes would be misplaced against an anchor that carries no such \
         register",
    ))
}

/// `design home`, `design and goal homes`, `design, goal and issue homes`.
fn list_homes(registers: &[&str]) -> String {
    match registers {
        [one] => format!("{one} home"),
        [head @ .., last] => format!("{} and {last} homes", head.join(", ")),
        [] => String::new(),
    }
}

/// Fold the declarations into the compiled-in registers, and say what each one got wrong.
fn build_registers(
    declared: &BTreeMap<String, RegisterDecl>,
    complaints: &mut Vec<Finding>,
) -> Registers {
    let mut out = Registers::built_in();
    let mut complaints = Complaints(complaints);
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
    Registers(out)
}

/// The register complaints, each carrying the one action every one of them shares.
struct Complaints<'a>(&'a mut Vec<Finding>);

impl Complaints<'_> {
    fn push(&mut self, what: String) {
        self.0.push(Finding::in_file(
            MANIFEST_NAME,
            what,
            "a declared register states its scope and its shape; a built-in register's \
             storage is what the word component means and is not declared",
        ));
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

    /// A declaration with the components named, parsed from text.
    ///
    /// Written out rather than taken from a checkout: what is under test is what the file
    /// says, and the paths below need not exist for that.
    fn declaring(components: &str) -> Manifest {
        declaring_full(components, "", "[]", "[]", "[]", "[]")
    }

    /// The statement of each complaint, in order.
    fn whats(m: &Manifest) -> Vec<String> {
        m.complaints().iter().map(|f| f.what.clone()).collect()
    }

    #[test]
    fn the_homes_a_complaint_lists_read_as_prose_at_every_count() {
        assert_eq!(list_homes(&["design"]), "design home");
        assert_eq!(list_homes(&["design", "goal"]), "design and goal homes");
        assert_eq!(
            list_homes(&["design", "goal", "issue"]),
            "design, goal and issue homes"
        );
    }

    /// The same, with every path list the manifest can declare spelled out, and `extra`
    /// holding whole tables.
    fn declaring_full(
        components: &str,
        extra: &str,
        skip_dirs: &str,
        skip_files: &str,
        exclude: &str,
        exempt: &str,
    ) -> Manifest {
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = [{components}]\n\n\
             {extra}\
             [walk]\nskip-dirs = {skip_dirs}\nskip-files = {skip_files}\n\
             exclude = {exclude}\n\n\
             [lint]\nexempt-files = {exempt}\n\n\
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
    fn a_dot_segment_is_dropped_from_every_declared_path() {
        // A dot-spelled `docs` and a plain one are one declaration: git lists neither a `.`
        // segment nor a trailing separator, and a row spelled with one matched nothing.
        let m = declaring_full(
            "\"./crates/./an-engine/\"",
            "[locations.papers]\npath = \"./docs/plans\"\nregisters = [\"issue\"]\n\n\
             [locations.here]\npath = \".\"\nregisters = [\"issue\"]\n\n",
            "[\"./build\"]",
            "[\"docs/./index.md\"]",
            "[\"./vendor/\"]",
            "[\"./notes/x.md\"]",
        );
        // `.` is the root, spelled the way the root component spells it, and the root is a
        // component already: the one complaint, and the location is no anchor.
        assert_eq!(
            whats(&m),
            vec!["[locations.here] names the project root".to_string()]
        );
        let components = m.components();
        assert_eq!(components.all()[1].path, PathBuf::from("crates/an-engine"));
        assert_eq!(components.all()[1].name, "an-engine");
        assert_eq!(m.locations()["papers"].path, PathBuf::from("docs/plans"));
        assert!(!m.locations().contains_key("here"));
        assert_eq!(m.walk().skip_dirs, vec![PathBuf::from("build")]);
        assert_eq!(m.walk().skip_files, vec![PathBuf::from("docs/index.md")]);
        assert_eq!(m.walk().exclude, vec![PathBuf::from("vendor")]);
        assert_eq!(m.lint().exempt_files, vec![PathBuf::from("notes/x.md")]);
    }

    #[test]
    fn the_refused_anchor_is_the_deeper_one_whatever_the_declaration_order() {
        // `a` is declared first and sits inside `b`'s issue directory. Judged in declaration
        // order `a` would be accepted and `b` refused for holding `a`'s home; judged
        // shallowest first, `b` stands and `a` is the one refused.
        let m = declaring_full(
            "",
            "[locations.a]\npath = \"notes/open-issues/x\"\nregisters = [\"issue\"]\n\n\
             [locations.b]\npath = \"notes\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        assert_eq!(whats(&m).len(), 1, "{:?}", whats(&m));
        assert!(
            whats(&m)[0].contains("the location `a` sits inside the issue register's directory"),
            "{:?}",
            whats(&m)
        );
        assert!(m.locations().contains_key("b"));
        assert!(!m.locations().contains_key("a"));
    }

    #[test]
    fn a_refused_register_is_no_register_and_a_location_naming_it_says_so() {
        let m = declaring_full(
            "",
            "[registers.path]\nscope = \"opt-in\"\nshape = \"file\"\n\n\
             [registers.Notes]\nscope = \"opt-in\"\nshape = \"file\"\n\n\
             [locations.papers]\npath = \"papers\"\nregisters = [\"path\", \"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let whats = whats(&m);
        assert!(
            whats
                .iter()
                .any(|w| w.contains("`path` is the reserved kind")),
            "{whats:?}"
        );
        assert!(
            whats
                .iter()
                .any(|w| w.contains("`Notes` cannot be spelled in a reference's kind segment")),
            "{whats:?}"
        );
        assert!(
            whats
                .iter()
                .any(|w| w.contains("[locations.papers] carries `path`, which is no register")),
            "{whats:?}"
        );
        assert!(m.registers().by_name("path").is_none());
        assert!(m.registers().by_name("Notes").is_none());
        assert_eq!(m.locations()["papers"].registers, vec!["issue".to_string()]);
    }

    #[test]
    fn a_location_carrying_two_registers_at_one_directory_keeps_the_first_named() {
        let m = declaring_full(
            "",
            "[registers.note]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"open-issues\"\n\n\
             [locations.papers]\npath = \"papers\"\nregisters = [\"issue\", \"note\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        assert_eq!(whats(&m).len(), 1, "{:?}", whats(&m));
        assert!(
            whats(&m)[0].contains(
                "[locations.papers] carries the note and issue registers at one directory, \
                 `open-issues`"
            ),
            "{:?}",
            whats(&m)
        );
        assert_eq!(m.locations()["papers"].registers, vec!["issue".to_string()]);
    }

    #[test]
    fn two_anchors_sharing_a_name_keep_the_first_declared() {
        let m = declaring("\"a/widget\", \"b/widget\"");
        assert_eq!(whats(&m).len(), 1, "{:?}", whats(&m));
        assert!(
            whats(&m)[0].contains("`widget` names 2 anchors: `a/widget` and `b/widget`"),
            "{:?}",
            whats(&m)
        );
        let components = m.components();
        let paths: Vec<&Path> = components.all().iter().map(|c| c.path.as_path()).collect();
        assert_eq!(paths, vec![Path::new(""), Path::new("a/widget")]);
    }

    #[test]
    fn a_parent_segment_or_an_absolute_path_is_a_complaint_and_the_row_is_dropped() {
        // What such a row names depends on where the manifest sits, not on the tree, and
        // the nesting check would read `..` as one more segment of the path.
        let m = declaring_full(
            "\"crates/../crates/an-engine\", \"tools/a-tool\"",
            "[locations.up]\npath = \"docs/../docs\"\nregisters = [\"issue\"]\n\n",
            "[\"/build\"]",
            "[\"../x.md\"]",
            "[\"/vendor\"]",
            "[\"../y.md\"]",
        );
        let complaints = whats(&m);
        assert_eq!(complaints.len(), 6, "{complaints:#?}");
        for label in ["[walk] skip-files", "[walk] exclude", "[lint] exempt-files"] {
            assert!(
                complaints.iter().any(|c| c.contains(label)),
                "{label}: {complaints:#?}"
            );
        }
        assert!(
            complaints[0].contains("`crates/../crates/an-engine` in [project] components")
                && complaints[0].contains("`..` segment"),
            "{complaints:#?}"
        );
        assert!(
            complaints[1].contains("in [locations.up] path"),
            "{complaints:#?}"
        );
        assert!(
            complaints[2].contains("`/build` in [walk] skip-dirs is not project-relative"),
            "{complaints:#?}"
        );
        let components = m.components();
        let names: Vec<&str> = components.all().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["a-project", "a-tool"],
            "the refused row is gone"
        );
        assert!(m.locations().is_empty());
        assert!(m.walk().skip_dirs.is_empty());
    }

    #[test]
    fn a_refused_rules_path_fails_the_load_and_a_dot_spelled_one_is_normalised() {
        // The corpus has no default to stand in for a refused row, and a path kept as
        // spelled was joined and read: `index` once wrote through a `..` corpus directory.
        // Every one of the five rows fails the same way.
        let rules = |dir: &str, text: &str, version: &str, past: &str, manifest: &str| {
            format!(
                "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = []\n\
                 skip-files = []\nexclude = []\n\n[lint]\nexempt-files = []\n\n[rules]\n\
                 dir = \"{dir}\"\ntext = \"{text}\"\nbody-starts-at = 0\nversion = \"{version}\"\n\
                 past = \"{past}\"\nmanifest = \"{manifest}\"\n"
            )
        };
        for (key, text) in [
            ("dir", rules("../corpus", "t", "v", "p", "m")),
            ("text", rules("corpus", "/t", "v", "p", "m")),
            ("version", rules("corpus", "t", "../v", "p", "m")),
            ("past", rules("corpus", "t", "v", "../p", "m")),
            ("manifest", rules("corpus", "t", "v", "p", "/m")),
        ] {
            let e = Manifest::parse(Path::new("/nowhere"), &text).expect_err(key);
            assert!(e.contains(&format!("in [rules] {key}")), "{key}: {e}");
            assert!(e.contains("no default"), "{key}: {e}");
        }
        let m = Manifest::parse(
            Path::new("/nowhere"),
            &rules("./corpus", "./t", "v", "p", "m"),
        )
        .expect("a declaration");
        assert!(whats(&m).is_empty(), "{:?}", whats(&m));
        assert_eq!(m.rules().dir, PathBuf::from("corpus"));
        assert_eq!(m.rules().text, PathBuf::from("t"));
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
        assert!(whats(&m).is_empty());
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
        assert!(whats(&m).is_empty(), "{:#?}", whats(&m));
    }

    #[test]
    fn a_declared_register_with_no_scope_or_no_shape_is_a_complaint() {
        let m = with_registers("[registers.reading]\nshape = \"file\"\n");
        assert_eq!(whats(&m).len(), 1);
        assert!(whats(&m)[0].contains("scope"), "{:?}", whats(&m));
        let m = with_registers("[registers.reading]\nscope = \"opt-in\"\n");
        assert!(whats(&m)[0].contains("shape"), "{:?}", whats(&m));
    }

    #[test]
    fn a_built_in_register_accepts_kinds_on_issue_and_nothing_anywhere_else() {
        // The one extensible thing about a built-in, and the refusals beside it.
        let m = with_registers("[registers.issue]\nkinds = [\"defect\", \"todo\"]\n");
        assert!(whats(&m).is_empty(), "{:?}", whats(&m));
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
            assert_eq!(whats(&m).len(), 1, "{key}");
            assert!(whats(&m)[0].contains(key), "{key}: {:?}", whats(&m));
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
        assert_eq!(whats(&m).len(), 1);
        assert!(whats(&m)[0].contains("kinds"), "{:?}", whats(&m));
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
