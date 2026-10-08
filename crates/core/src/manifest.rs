//! `knowledge-architect.toml` — what a project declares must stay conformant, and what is exempt.
//!
//! Nothing about any particular repository is compiled into this tool. Every list a check
//! reads comes from here, so the same binary checks this repository and a mock project under
//! `path@core@tests/projects/` with no special case anywhere, and a path that should not be checked has
//! to say so in one file with a reason beside it.
//!
//! The file's presence is also what makes a directory a project root. That is one mechanism
//! rather than two: a directory either declares itself a project or it does not, and the tool
//! refuses to run outside one instead of guessing a root from its own location.
//!
//! **Ten registers are compiled in and the rest are declared.** `design`, `goal`, `tripwire`
//! and `issue` are what the word component means here, so a project neither adds nor removes
//! them; `spec` and `milestone` are the plan documents, carried by the plan anchors,
//! which the tool constructs at [`PLANS_DIR`]. `[registers.<name>]` declares further ones, and
//! `[locations.<name>]` names a directory that carries a subset of them. The argument is
//! `design@core@registers-are-declared`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::finding::Finding;

/// The file that both marks a project root and declares its conformance surface.
pub const MANIFEST_NAME: &str = "knowledge-architect.toml";

/// The documents a component carries beside its registers, relative to its own directory; which
/// of them a project owes is [`Manifest::required_documents`].
///
/// Compiled in, unlike every other list a check reads. WHICH components a project has is that
/// project's own knowledge and is declared in `[project]`; WHAT a component is, is this tool's
/// definition of one. A project free to declare its own set could be conformant with anything,
/// which is the same as being checked against nothing.
///
/// The register homes are not in this list. Each is a register's own storage, in whichever
/// shape that register accepts, and `check::registers` asserts them from the register list.
pub(crate) const COMPONENT_DOCUMENTS: [&str; 3] =
    ["README.md", "CLAUDE.md", "docs/rejected-alternatives.md"];

/// The document of `COMPONENT_DOCUMENTS` that holds agent instructions, required only while the
/// project declares the `claude` harness, per `design@core@agents-table`.
pub(crate) const AGENT_DOCUMENT: &str = "CLAUDE.md";

/// The command a project runs the checker by when its manifest declares none: the name of the
/// binary this crate installs, per `design@core@declared-command`.
pub(crate) const DEFAULT_COMMAND: &str = "klarch";

/// The agent harnesses a manifest may declare, per `design@core@agents-table`.
pub(crate) const HARNESSES: [&str; 1] = ["claude"];

/// The prefix that owns a namespace in a project's agent configuration, per
/// `design@core@owned-namespace-check`. Every skill directory and agent file whose name starts
/// with it, and every file under .claude/knowledge-architect/, belongs to the installer.
pub(crate) const OWNED_PREFIX: &str = "knowledge-architect-";

/// The name of the built-in issue register, which is the one register that accepts a key.
pub(crate) const ISSUE_REGISTER: &str = "issue";

/// The built-in heading register of evidence that would flip a decision, by the name a
/// reference spells in kind position.
pub(crate) const TRIPWIRE_REGISTER: &str = "tripwire";

/// The built-in register of plan documents: a spec file, or a slice spec inside a milestone.
pub(crate) const SPEC_REGISTER: &str = "spec";

/// The built-in register of milestones, whose entries are directories.
pub(crate) const MILESTONE_REGISTER: &str = "milestone";

/// Where the tool constructs the anchor `plans`, relative to the project root.
///
/// Fixed rather than declared: a built-in register's storage is what the tool defines, and a
/// declared plans directory would be the only declared path in that family.
pub(crate) const PLANS_DIR: &str = "docs/plans";

/// The home of the `spec` register under [`PLANS_DIR`].
pub(crate) const SPECS_HOME: &str = "specs";

/// The home of the `milestone` register under [`PLANS_DIR`].
pub(crate) const MILESTONES_HOME: &str = "milestones";

/// The item register of a plan's threads.
pub(crate) const THREAD_REGISTER: &str = "thread";

/// The item register of a plan's arguments.
pub(crate) const ARGUMENT_REGISTER: &str = "argument";

/// The item register of a plan's criteria.
pub(crate) const CRITERION_REGISTER: &str = "criterion";

/// The item register of a plan's acceptance criteria.
pub(crate) const ACCEPTANCE_REGISTER: &str = "acceptance";

/// The four item registers: the name a reference spells, the word the Undefined repair names
/// as the home, and the level-two section of a plan document whose level-three headings are
/// its entries.
pub(crate) const ITEM_REGISTERS: [(&str, &str, &str); 4] = [
    (THREAD_REGISTER, "threads", "Threads"),
    (ARGUMENT_REGISTER, "arguments", "Arguments"),
    (CRITERION_REGISTER, "criteria", "Criteria"),
    (
        ACCEPTANCE_REGISTER,
        "acceptance-criteria",
        "Acceptance criteria",
    ),
];

/// The level-two sections a spec of specs/ and a milestone's README owe, in order: the plan
/// document's sections, as the plans layout fixes them.
pub(crate) const PLAN_SECTIONS: [&str; 20] = [
    "Status and audience",
    "How the work is done",
    "Names",
    "What the work is",
    "What is already decided",
    "Criteria",
    "Threads",
    "Arguments",
    "New names, in one place",
    "Decided design",
    "Mapping tables",
    "Losing alternatives",
    "Readings",
    "Premortem",
    "Acceptance criteria",
    "Implementation sequence",
    "Order rationale",
    "Defaults awaiting the owner",
    "Harvest",
    "Later consequences",
];

/// The level-two sections a slice spec owes, in order. A slice's fixtures are owed only where the
/// Component drives its tests with authored content, which no check can read, so they are not
/// checked.
pub(crate) const SLICE_SECTIONS: [&str; 5] = [
    "Builds",
    "Claims",
    "Audit subjects",
    "Fails alone on",
    "Premises that expire",
];

/// The issue register's compiled kind list. Closed: an unknown kind is a finding naming it.
pub(crate) const ISSUE_KINDS: [&str; 6] = [
    "defect",
    "observation",
    "question",
    "todo",
    "deferred",
    "design",
];

/// The level-three subsections an issue owes under `## Details`, in order.
pub(crate) const ISSUE_SUBSECTIONS: [&str; 3] = ["What", "Why it matters", "What would close it"];

/// The same for a `deferred` entry, which states its trigger where the others state closure.
pub(crate) const DEFERRED_SUBSECTIONS: [&str; 3] = ["What", "Why it matters", "Trigger"];

/// Where a register keeps its entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    /// Entries are headings at the register's level carrying a slug, in `<dir>.md` or in
    /// `<dir>/` behind a README.
    Heading,
    /// Entries are files `<id>.md` under `<dir>/`, beside a README and a generated index.
    File,
    /// Entries are directories `<id>/` under `<dir>/`, each holding a `README.md`, beside a
    /// README and a generated index. Only `milestone` has it: each entry is an anchor of its own.
    Directory,
    /// Entries are level-three headings carrying a slug, under the level-two section the register
    /// names, in the documents of a plan anchor. Only the four item registers have it.
    Section,
}

/// Which anchors carry a register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Scope {
    /// Every component carries it.
    Component,
    /// Only the locations that name it carry it.
    OptIn,
}

/// One register: a kind together with its storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Register {
    /// The word a reference spells in kind position.
    pub name: String,
    pub scope: Scope,
    pub shape: Shape,
    /// The basename of the home under the anchor's home base.
    pub dir: String,
    /// Heading and Section shapes: the one heading level every entry sits at. `None` for the
    /// file and directory shapes.
    pub level: Option<u8>,
    /// File shape: the level-two headings every entry carries, in order.
    pub sections: Vec<String>,
    /// File shape: each frontmatter key an entry carries, with its closed value set.
    pub metadata: Vec<(String, Vec<String>)>,
    /// The issue register alone: the kinds an entry may declare.
    pub kinds: Vec<String>,
    /// Section shape: the level-two section whose level-three headings are its entries.
    pub section: Option<String>,
    /// Whether this register is one of those the tool compiles in.
    pub built_in: bool,
}

impl Register {
    /// The subsections an entry of this register owes under its last section, given the
    /// value of its first metadata key. Empty for every register but `issue`.
    pub(crate) fn owed_subsections(&self, kind: Option<&str>) -> &'static [&'static str] {
        if self.name != ISSUE_REGISTER {
            return &[];
        }
        match kind {
            Some("deferred") => &DEFERRED_SUBSECTIONS,
            _ => &ISSUE_SUBSECTIONS,
        }
    }
}

/// Every register of a project: the built-in ones, then the declared ones by name.
#[derive(Debug, Clone)]
pub(crate) struct Registers(Vec<Register>);

impl Registers {
    pub(crate) fn all(&self) -> &[Register] {
        &self.0
    }

    pub(crate) fn by_name(&self, name: &str) -> Option<&Register> {
        self.0.iter().find(|r| r.name == name)
    }

    /// Every register a declaration may name, comma-separated, as a finding lists them: all
    /// but the plan registers, which the plan anchors alone carry.
    pub(crate) fn listed(&self) -> String {
        self.0
            .iter()
            .filter(|r| !Self::is_plan_register(&r.name))
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The registers every component carries.
    pub(crate) fn component_scoped(&self) -> impl Iterator<Item = &Register> {
        self.0.iter().filter(|r| r.scope == Scope::Component)
    }

    /// The compiled-in registers, before any declaration is read.
    fn built_in() -> Vec<Register> {
        let heading = |name: &str, dir: &str, level: u8| Register {
            name: name.to_string(),
            scope: Scope::Component,
            shape: Shape::Heading,
            dir: dir.to_string(),
            level: Some(level),
            sections: Vec::new(),
            metadata: Vec::new(),
            kinds: Vec::new(),
            section: None,
            built_in: true,
        };
        let mut out = vec![
            heading("design", "design", 3),
            heading("goal", "goals", 2),
            heading(TRIPWIRE_REGISTER, "tripwires", 2),
            Register {
                name: ISSUE_REGISTER.to_string(),
                scope: Scope::Component,
                shape: Shape::File,
                dir: "open-issues".to_string(),
                level: None,
                sections: vec!["Summary".to_string(), "Details".to_string()],
                metadata: vec![(
                    "kind".to_string(),
                    ISSUE_KINDS.iter().map(|k| k.to_string()).collect(),
                )],
                kinds: ISSUE_KINDS.iter().map(|k| k.to_string()).collect(),
                section: None,
                built_in: true,
            },
        ];
        // The plan registers are opt-in: the anchors the tool constructs name them, and no
        // declared anchor may. A plan document owes the plan sections; a slice spec owes its own
        // list, which `Anchor::sections_of` gives at a milestone anchor.
        out.push(plan(SPEC_REGISTER, Shape::File, SPECS_HOME, None));
        out.push(plan(
            MILESTONE_REGISTER,
            Shape::Directory,
            MILESTONES_HOME,
            None,
        ));
        for (name, dir, section) in ITEM_REGISTERS {
            out.push(plan(name, Shape::Section, dir, Some(section)));
        }
        out
    }

    /// Whether `name` is a register only the anchors the tool constructs carry: `spec`,
    /// `milestone` and the four item registers.
    pub(crate) fn is_plan_register(name: &str) -> bool {
        name == SPEC_REGISTER
            || name == MILESTONE_REGISTER
            || ITEM_REGISTERS.iter().any(|(n, _, _)| *n == name)
    }
}

/// One of the plan registers, which no declaration can change. `section` is an item register's
/// section, and makes its entries level-three headings.
fn plan(name: &str, shape: Shape, dir: &str, section: Option<&str>) -> Register {
    Register {
        name: name.to_string(),
        scope: Scope::OptIn,
        shape,
        dir: dir.to_string(),
        level: section.map(|_| 3),
        sections: if section.is_some() {
            Vec::new()
        } else {
            PLAN_SECTIONS.iter().map(|s| s.to_string()).collect()
        },
        metadata: Vec::new(),
        kinds: Vec::new(),
        section: section.map(str::to_string),
        built_in: true,
    }
}

/// A register as the manifest declares it.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct RegisterDecl {
    pub scope: Option<String>,
    pub shape: Option<String>,
    pub dir: Option<String>,
    pub level: Option<i64>,
    #[serde(default)]
    pub sections: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, MetadataDecl>,
    pub kinds: Option<Vec<String>>,
}

/// One frontmatter key an entry must carry, and the values it accepts.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct MetadataDecl {
    pub values: Vec<String>,
}

/// A named directory carrying a declared subset of the registers.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct LocationDecl {
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
pub(crate) struct Project {
    /// What a reference names the component at the root by.
    pub name: String,
    /// One project-relative directory per component below the root, in declaration order.
    pub components: Vec<PathBuf>,
    /// The command this project runs the checker by, printed in messages, in generated index
    /// headers and in the installed skills. [`DEFAULT_COMMAND`] when absent.
    #[serde(default)]
    pub command: Option<String>,
    /// The version of the checker the project pins, as written; [`resolve_checker`] reads it
    /// into a [`Pin`]. Optional here so that an absent key is a complaint with a repair rather
    /// than a deserialisation error.
    #[serde(default)]
    pub checker_version: Option<String>,
}

/// What `[project] checker-version` says, per `design@core@installed-binary-version-check`.
///
/// A version is the core library's version the project runs. The two sentinels are claims
/// about the tree that the running binary's build confirms or refuses: a mock project inside a
/// library directory, and the tree the checker is built from. Which build confirms which claim
/// is `cli::refuse_another_version`'s; a manifest only parsed is never confirmed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Pin {
    /// `"<MAJOR.MINOR.PATCH>"`, compared as text with the binary's version.
    Version(String),
    /// `"fixture"`.
    Fixture,
    /// `"self"`; `Self` is a Rust keyword.
    OwnBuild,
}

/// Whether a value is a version as the key takes one: three decimal integers that each fit in a
/// `u64`, dot-separated, with no leading zero except a lone `0`, and no pre-release or build
/// suffix. A part too large for a `u64` is refused, so every pin compares as three integers.
pub(crate) fn is_plain_version(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.bytes().all(|b| b.is_ascii_digit())
                && (p.len() == 1 || !p.starts_with('0'))
                && p.parse::<u64>().is_ok()
        })
}

/// The `[agents]` table: which agent harnesses the project serves.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct AgentsDecl {
    /// The harnesses, each one of [`HARNESSES`]. Empty declares no agent configuration.
    pub harness: Vec<String>,
}

/// The `[commits]` table: what `commits` judges beyond the rules every message is held to.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct CommitsDecl {
    /// Refuse a message or a document that cites a commit of the judged range by its SHA,
    /// per `design@core@branch-shas-are-refused`. Off when absent.
    #[serde(default)]
    pub refuse_branch_shas: bool,
}

/// One component: a directory carrying its own documents and every component register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Component {
    /// The one word a reference names it by: the basename of its path, or the project's
    /// name for the component at the root.
    pub name: String,
    /// Project-relative, and empty for the component at the root.
    pub path: PathBuf,
}

impl Component {
    /// Whether this is the component at the project root.
    #[cfg(test)]
    pub(crate) fn is_root(&self) -> bool {
        self.path.as_os_str().is_empty()
    }
}

/// Every component of a project: the one at the root first, then the declared ones in
/// declaration order.
///
/// Built rather than stored, because the root component is not declared anywhere and the name
/// of a declared one is derived from its path.
#[derive(Debug, Clone)]
pub(crate) struct Components(Vec<Component>);

impl Components {
    pub(crate) fn all(&self) -> &[Component] {
        &self.0
    }

    /// The component a document belongs to: the deepest declared path that holds it, and the
    /// component at the root where none does.
    #[cfg(test)]
    pub(crate) fn owning(&self, rel: &Path) -> &Component {
        self.0
            .iter()
            .filter(|c| rel.starts_with(&c.path))
            .max_by_key(|c| c.path.components().count())
            .expect("the component at the root is a prefix of every path")
    }

    /// The component a reference names, or `None` where nothing declares that name.
    #[cfg(test)]
    pub(crate) fn by_name(&self, name: &str) -> Option<&Component> {
        self.0.iter().find(|c| c.name == name)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct Walk {
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
    pub(crate) fn sample() -> Self {
        Self {
            skip_dirs: vec![PathBuf::from(".git")],
            skip_files: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

/// What the manifest declares, as written.
///
/// **Every other top-level table is kept, as written, for an extension to claim**, per
/// `design@core@an-extension-claims-its-manifest-tables`. That is why this struct refuses no
/// unknown field: `serde(flatten)` cannot be combined with that refusal, and phase 1 refuses a
/// table no registered extension claims instead, so a misspelt table is still reported.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Declared {
    project: Project,
    #[serde(default)]
    locations: BTreeMap<String, LocationDecl>,
    #[serde(default)]
    registers: BTreeMap<String, RegisterDecl>,
    walk: Walk,
    #[serde(default)]
    agents: Option<AgentsDecl>,
    #[serde(default)]
    commits: Option<CommitsDecl>,
    #[serde(flatten)]
    tables: BTreeMap<String, toml::Value>,
}

/// A project: where its root is, and what it declares.
#[derive(Debug, Clone)]
pub struct Manifest {
    root: PathBuf,
    declared: Declared,
    registers: Registers,
    /// Whether the anchor `plans` the tool constructs at [`PLANS_DIR`] was accepted beside the
    /// declared anchors. `false` only when a declaration collides with it, which is a complaint.
    plans: bool,
    /// Every declaration this tool refused, in the shape `check` reports.
    ///
    /// **A refused declaration is absent from the configuration**, so nothing acts on it: a
    /// refused register is not a register, a refused anchor is not an anchor, a refused row is
    /// not in its list. Held rather than returned, because a manifest that would not load
    /// reports nothing at all, and nothing at all is what a session reads as conformance. An
    /// extension's complaints about its own tables join them when it is configured.
    complaints: Vec<Finding>,
    /// The paths each extension declares, by the label a finding names them with, asserted to
    /// exist in phase 2 with the core's own declared paths.
    extension_paths: Vec<(String, Vec<PathBuf>)>,
    /// The files each extension generates, which the walk leaves out and the `generated` check
    /// reads as committed.
    extension_generated: Vec<PathBuf>,
    /// What `[project] checker-version` says; `None` only when the key is absent or holds none
    /// of the three forms, which is then a complaint.
    pin: Option<Pin>,
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
    /// ignores, per `design@core@git-supplies-the-walk`.
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
        let mut declared: Declared = toml::from_str(text).map_err(|e| e.to_string())?;
        let mut complaints = Vec::new();
        let mut registers = build_registers(&declared.registers, &mut complaints);
        resolve_registers(&mut registers, &mut complaints);
        normalise_paths(&mut declared, &mut complaints)?;
        let plans = resolve_anchors(&mut declared, &registers, &mut complaints);
        resolve_agents(&mut declared, &mut complaints);
        resolve_command(&mut declared, &mut complaints);
        let pin = resolve_checker(&declared, &mut complaints);
        Ok(Self {
            root: root.to_path_buf(),
            declared,
            registers,
            plans,
            complaints,
            extension_paths: Vec::new(),
            extension_generated: Vec::new(),
            pin,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn walk(&self) -> &Walk {
        &self.declared.walk
    }

    /// What `[project] checker-version` says, or `None` while its complaint stands.
    pub(crate) fn pin(&self) -> Option<&Pin> {
        self.pin.as_ref()
    }

    /// Whether the complaint about `[project] checker-version` stands, and its text.
    pub(crate) fn pin_complaint(&self) -> Option<&Finding> {
        self.complaints
            .iter()
            .find(|f| f.what.starts_with("[project] checker-version"))
    }

    /// The command this project runs the checker by, per `design@core@declared-command`.
    pub fn command(&self) -> &str {
        self.declared
            .project
            .command
            .as_deref()
            .unwrap_or(DEFAULT_COMMAND)
    }

    /// Whether `commits` refuses a citation of a commit of its range by SHA: off unless the
    /// `[commits]` table turns it on, per `design@core@branch-shas-are-refused`.
    pub(crate) fn refuses_branch_shas(&self) -> bool {
        self.declared
            .commits
            .as_ref()
            .is_some_and(|c| c.refuse_branch_shas)
    }

    /// Whether the project serves the `claude` harness: the default when no `[agents]` table
    /// is declared, per `design@core@agents-table`.
    pub(crate) fn serves_claude(&self) -> bool {
        match &self.declared.agents {
            None => true,
            Some(agents) => agents.harness.iter().any(|h| h == "claude"),
        }
    }

    /// The documents every component is required to carry: [`COMPONENT_DOCUMENTS`], without
    /// [`AGENT_DOCUMENT`] when the project serves no harness that reads it.
    pub(crate) fn required_documents(&self) -> Vec<&'static str> {
        COMPONENT_DOCUMENTS
            .iter()
            .copied()
            .filter(|d| *d != AGENT_DOCUMENT || self.serves_claude())
            .collect()
    }

    /// Whether a project-relative path belongs to the installer's namespace, per
    /// `design@core@owned-namespace-check`. Nothing does when the project serves no harness.
    pub(crate) fn owned(&self, rel: &Path) -> bool {
        self.serves_claude() && owned_path(rel)
    }

    /// A top-level table the core does not own, as written, for the extension that claims it.
    pub fn table(&self, name: &str) -> Option<&toml::Value> {
        self.declared.tables.get(name)
    }

    /// The names of every top-level table the core does not own.
    pub(crate) fn extension_tables(&self) -> impl Iterator<Item = &str> {
        self.declared.tables.keys().map(String::as_str)
    }

    /// The paths each configured extension declares, by the label a finding names them with.
    pub(crate) fn extension_paths(&self) -> &[(String, Vec<PathBuf>)] {
        &self.extension_paths
    }

    /// The files each configured extension generates.
    pub(crate) fn extension_generated(&self) -> &[PathBuf] {
        &self.extension_generated
    }

    /// Record what configuring the extensions produced: complaints, declared paths, generated
    /// files. Only `extension::configure` calls this.
    pub(crate) fn record_extensions(
        &mut self,
        complaints: Vec<Finding>,
        paths: Vec<(String, Vec<PathBuf>)>,
        generated: Vec<PathBuf>,
    ) {
        self.complaints.extend(complaints);
        self.extension_paths.extend(paths);
        self.extension_generated.extend(generated);
    }

    /// Every register this project has, the built-in ones first.
    pub(crate) fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Whether the anchor `plans` exists: the tool constructs it at [`PLANS_DIR`] in every
    /// project, unless a declaration collides with it, which is a complaint.
    pub(crate) fn plans(&self) -> bool {
        self.plans
    }

    /// Every declaration the tool refused, for `check` to report first.
    pub fn complaints(&self) -> &[Finding] {
        &self.complaints
    }

    /// The locations this project declares, by name.
    pub(crate) fn locations(&self) -> &BTreeMap<String, LocationDecl> {
        &self.declared.locations
    }

    /// Every component, the one at the root first.
    pub(crate) fn components(&self) -> Components {
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
    Ok(())
}

/// Normalise every path of one list, dropping the rows that are refused.
pub fn normalise_list(list: &str, paths: &mut Vec<PathBuf>, complaints: &mut Vec<Finding>) {
    paths.retain_mut(|path| normalise_one(list, path, complaints));
}

/// Normalise one path in place, or record why it is refused and say so with `false`.
pub fn normalise_one(list: &str, path: &mut PathBuf, complaints: &mut Vec<Finding>) -> bool {
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
/// grammar is a register nothing can point at, and `path` and `planned` are names the resolver
/// answers before it ever reaches the register list. Its home is `<home base>/<dir>`, so a `dir` that
/// is not one plain segment puts the home somewhere the anchor does not reach, and a `dir`
/// spelling a compiled document makes one file both the document and the home. Each is a
/// complaint, and the register is not declared. The built-in ones are named and placed by
/// this tool and are never refused here.
fn resolve_registers(registers: &mut Registers, complaints: &mut Vec<Finding>) {
    registers.0.retain(|register| {
        if register.built_in {
            return true;
        }
        let refusal = if crate::entity::HARNESS_KINDS.contains(&register.name.as_str()) {
            Some((
                format!(
                    "`{}` is a kind of the agent harness, a skill, an agent or a section of the \
                     primer or of the root CLAUDE.md",
                    register.name
                ),
                "rename the register; a reference whose kind segment is this word names the \
                 harness's entity and never reaches the register",
            ))
        } else if register.name == crate::entity::PATH_KIND
            || register.name == crate::entity::PLANNED_KIND
        {
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
            && Path::new("docs").join(&register.dir) == Path::new(PLANS_DIR)
        {
            // Every component carries the register, the root included, whose home would then be
            // the plans directory the tool places the anchor `plans` at.
            Some((
                format!(
                    "the {} register's directory `{}` makes its home at the root {PLANS_DIR}/",
                    register.name, register.dir
                ),
                "give the register another directory; the plans directory is the anchor `plans`, \
                 which the tool constructs, and one directory answers for one anchor",
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
    // with nothing said. The later-declared one is refused; the built-in ones come first.
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
    /// Whether this is the anchor `plans`, which the tool constructs rather than a row declares.
    tool_built: bool,
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
/// every document under its path, per `design@core@every-path-names-its-anchor`, so an
/// anchor inside another's directory is the ordinary case — thaum's two locations
/// both sit inside the root component. What that rule cannot absorb is refused here, per
/// `design@core@anchors-are-components-and-locations`, and **the refused anchor is not an
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
///   has**: every pointer at it would miss or read as the other. `plans` is reserved for the
///   anchor the tool constructs.
/// - **the root's path, spelled by a component or a location**: the root is a component and
///   carries every register already.
/// - **a component inside a location**: it nests a full register set inside a partial one.
/// - **the same path as an accepted anchor**: nothing decides which owns the documents under it.
/// - **inside, or at, an accepted anchor's register directory**: every file under it would
///   be read as an entry or a subdocument of that register.
/// - **holding an accepted anchor's register directory**: being deeper, it would own every
///   document in that home, and each slug defined there would be misplaced.
///
/// **The anchor `plans` is judged like a location**, after the declared ones of its depth, so
/// a declared anchor at [`PLANS_DIR`] or inside one of its homes is the one refused. A declared
/// location naming a plan register is refused that register: `plans` alone carries them.
/// Returns whether `plans` was accepted.
fn resolve_anchors(
    declared: &mut Declared,
    registers: &Registers,
    complaints: &mut Vec<Finding>,
) -> bool {
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
        tool_built: false,
    }];
    // Pushed right after the root, so the stable sort below puts it first among the anchors
    // of its depth: a declared anchor at the plans directory is then the one refused, and the
    // repair a complaint names is the declaration a project can change.
    let plans_dir = PathBuf::from(PLANS_DIR);
    candidates.push(Candidate {
        name: crate::entity::PLANS_ANCHOR.to_string(),
        homes: [SPEC_REGISTER, MILESTONE_REGISTER]
            .iter()
            .filter_map(|r| registers.by_name(r))
            .map(|r| (r.name.clone(), plans_dir.join(&r.dir)))
            .collect(),
        path: plans_dir,
        is_component: false,
        declared_at: format!("the anchor `plans` the tool constructs at {PLANS_DIR}/"),
        tool_built: true,
    });
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
            tool_built: false,
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
            if Registers::is_plan_register(register) {
                complaints.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!("[locations.{name}] carries `{register}`, a plan register"),
                    format!(
                        "remove it from the row; the plan anchors, which the tool constructs under \
                         {PLANS_DIR}/, carry it alone"
                    ),
                ));
                return false;
            }
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
            tool_built: false,
        });
    }
    // Stable, so among equals the order the candidates were pushed in decides: `plans` first,
    // then components in declaration order, then locations in name order.
    candidates.sort_by_key(|c| (c.depth(), !(c.is_component || c.tool_built)));

    let mut accepted: Vec<Candidate> = Vec::new();
    let mut plans = false;
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
        // The reserved words are compiled in, so an anchor wearing one could never be the
        // target of a reference: every pointer at it would read as the reserved meaning. The
        // one candidate that wears `plans` is the anchor the tool constructs.
        if !candidate.tool_built && crate::entity::is_reserved_anchor(&candidate.name) {
            refuse(
                format!(
                    "{} gives the anchor the reserved name `{}`",
                    candidate.declared_at, candidate.name
                ),
                "rename it; this word is reserved by the tool",
            );
            continue;
        }
        // A reference's head is read as a kind before an anchor, so an anchor named like a kind
        // would be shadowed in every reference's head, per `crate::entity::is_kind_name`.
        if !candidate.tool_built && crate::entity::is_kind_name(&candidate.name, registers) {
            refuse(
                format!(
                    "{} gives the anchor the name `{}`, which is the name of a kind",
                    candidate.declared_at, candidate.name
                ),
                "rename it; a reference's first segment is read as a kind before an anchor, so an \
                 anchor of this name could not be told apart from the kind",
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
        plans |= candidate.tool_built;
        accepted.push(candidate);
    }

    declared
        .project
        .components
        .retain(|path| accepted.iter().any(|a| a.is_component && a.path == *path));
    declared.locations.retain(|name, _| {
        accepted
            .iter()
            .any(|a| !a.is_component && !a.tool_built && a.name == *name)
    });
    plans
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
        // The plan registers' storage is the plans layout, which the tool fixes, so a table
        // for one changes nothing whatever it sets, and is refused as a whole.
        if Registers::is_plan_register(name) {
            complaints.0.push(Finding::in_file(
                MANIFEST_NAME,
                format!("[registers.{name}] declares a plan register"),
                format!(
                    "delete the table; `{name}` is carried by the plan anchors alone, which the \
                     tool constructs under {PLANS_DIR}/ with a fixed layout"
                ),
            ));
            continue;
        }
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
            if decl.level.is_some() {
                wrong.push("level");
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
        let level = match (shape, decl.level) {
            // Level one is the document's title, so no register's entries sit there.
            (Shape::Heading, Some(level @ 2..=6)) => Some(level as u8),
            (Shape::Heading, Some(level)) => {
                complaints.push(format!(
                    "[registers.{name}] declares level {level}; a heading register's entries \
                     sit at a heading level from 2 to 6"
                ));
                None
            }
            (Shape::Heading, None) => {
                complaints.push(format!(
                    "[registers.{name}] declares no level; a heading register declares the \
                     heading level its entries sit at, from 2 to 6"
                ));
                None
            }
            // A declared shape is `heading` or `file`; the others are the plan registers'.
            (Shape::File | Shape::Directory | Shape::Section, Some(_)) => {
                complaints.push(format!(
                    "[registers.{name}] declares a level on a file register, whose entries \
                     are files"
                ));
                None
            }
            (Shape::File | Shape::Directory | Shape::Section, None) => None,
        };
        out.push(Register {
            name: name.clone(),
            scope,
            shape,
            dir: decl.dir.clone().unwrap_or_else(|| name.clone()),
            level,
            sections: decl.sections.clone(),
            metadata: decl
                .metadata
                .iter()
                .map(|(k, v)| (k.clone(), v.values.clone()))
                .collect(),
            kinds: Vec::new(),
            section: None,
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

/// The installer's namespace under the `claude` harness: .claude/knowledge-architect/ and
/// everything under it, a skill directory `.claude/skills/<OWNED_PREFIX>…/` and everything under
/// it, and an agent file `.claude/agents/<OWNED_PREFIX>…`.
pub(crate) fn owned_path(rel: &Path) -> bool {
    // A path is bytes: a component that is not UTF-8 belongs to no name this tool writes, so the
    // whole path is outside the namespace rather than read with that component dropped.
    let Some(parts) = rel
        .iter()
        .map(|c| c.to_str())
        .collect::<Option<Vec<&str>>>()
    else {
        return false;
    };
    match parts.as_slice() {
        [".claude", "knowledge-architect", _, ..] => true,
        [".claude", "skills", dir, _, ..] => dir.starts_with(OWNED_PREFIX),
        [".claude", "agents", file] => file.starts_with(OWNED_PREFIX),
        _ => false,
    }
}

/// Refuse a command that cannot be printed on one line inside a code span, and keep the default.
/// It is printed in every repair line and generated header, so a line break would split a finding
/// and a backtick would end its span early.
fn resolve_command(declared: &mut Declared, complaints: &mut Vec<Finding>) {
    let Some(command) = declared.project.command.as_deref() else {
        return;
    };
    if command.trim().is_empty() || command.contains(['\n', '\r', '`']) {
        complaints.push(Finding::in_file(
            MANIFEST_NAME,
            format!("[project] command {command:?} cannot be printed as one command"),
            "declare a non-empty command on one line, with no backtick",
        ));
        declared.project.command = None;
    }
}

/// Read `[project] checker-version` into a [`Pin`], or refuse it as a complaint.
///
/// A complaint rather than a parse error, as [`resolve_command`] does, so the finding carries a
/// repair and a historical tree that predates the key stops in phase 1 with a line naming it.
/// The text opens with `[project] checker-version`, which `cli::refuse_another_version` repeats
/// when it refuses a run over such a manifest.
fn resolve_checker(declared: &Declared, complaints: &mut Vec<Finding>) -> Option<Pin> {
    let action =
        "write the version of the checker the project runs, \"fixture\" in a mock project \
                  inside a library's directory, or \"self\" where the checker is built from \
                  this tree";
    let Some(value) = declared.project.checker_version.as_deref() else {
        complaints.push(Finding::in_file(
            MANIFEST_NAME,
            "[project] checker-version is absent".to_string(),
            action,
        ));
        return None;
    };
    match value {
        "fixture" => Some(Pin::Fixture),
        "self" => Some(Pin::OwnBuild),
        v if is_plain_version(v) => Some(Pin::Version(v.to_string())),
        v => {
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "[project] checker-version {v:?} is not a version, \"fixture\" or \"self\""
                ),
                action,
            ));
            None
        }
    }
}

/// Refuse a harness this tool does not know, and keep the rest, per `design@core@agents-table`.
/// A refused value leaves the list, so nothing acts on it.
fn resolve_agents(declared: &mut Declared, complaints: &mut Vec<Finding>) {
    let Some(agents) = declared.agents.as_mut() else {
        return;
    };
    agents.harness.retain(|h| {
        let known = HARNESSES.contains(&h.as_str());
        if !known {
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                format!("[agents] harness `{h}` is no harness this tool knows"),
                format!(
                    "declare one of {}, or an empty list for no agent configuration",
                    HARNESSES.join(", ")
                ),
            ));
        }
        known
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// The manifest of the checkout these tests were compiled from.
    ///
    /// `CARGO_MANIFEST_DIR` is legitimate here and nowhere else: it is baked in at compile
    /// time, which is wrong for a binary and exactly right for a test that only ever runs
    /// against the tree it was built from.
    pub(crate) fn this_project() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crates/core sits two levels below the project root")
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
        ext_files: &str,
    ) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = [{components}]\n\n\
             {extra}\
             [walk]\nskip-dirs = {skip_dirs}\nskip-files = {skip_files}\n\
             exclude = {exclude}\n\n\
             [ext]\nfiles = {ext_files}\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// A declaration with one table added after the project and before the walk.
    fn with_agents(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The claim: `commits` refuses branch SHAs only where `[commits] refuse-branch-shas` is
    /// true, and an unknown key of the table is refused with the manifest. Mutation: defaulting
    /// the flag to true fails the absent case.
    #[test]
    fn branch_shas_are_refused_only_where_the_manifest_turns_it_on() {
        assert!(!with_agents("").refuses_branch_shas());
        assert!(!with_agents("[commits]\n").refuses_branch_shas());
        assert!(!with_agents("[commits]\nrefuse-branch-shas = false\n").refuses_branch_shas());
        assert!(with_agents("[commits]\nrefuse-branch-shas = true\n").refuses_branch_shas());
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n\n[commits]\nno-such-key = true\n\n\
                    [walk]\nskip-dirs = []\nskip-files = []\n";
        assert!(Manifest::parse(Path::new("/nowhere"), text).is_err());
    }

    /// The claim: without an `[agents]` table the project serves `claude` and every component
    /// owes a CLAUDE.md; `harness = []` drops that document and nothing else. Mutation: ignoring
    /// the harness in `required_documents` fails the empty case.
    #[test]
    fn the_agent_document_is_required_only_under_the_claude_harness() {
        let default = with_agents("");
        assert!(default.serves_claude());
        assert!(default.required_documents().contains(&AGENT_DOCUMENT));
        let none = with_agents("[agents]\nharness = []\n");
        assert!(!none.serves_claude());
        assert_eq!(
            none.required_documents(),
            vec!["README.md", "docs/rejected-alternatives.md"]
        );
        assert!(!none.owned(Path::new(".claude/agents/knowledge-architect-a.md")));
    }

    /// The claim: a harness the tool does not know is a phase-1 complaint naming it, and leaves
    /// the list. Mutation: keeping an unknown value loses the complaint.
    #[test]
    fn an_unknown_harness_is_refused_and_named() {
        let m = with_agents("[agents]\nharness = [\"claude\", \"gemini\"]\n");
        let complaints: Vec<&str> = m.complaints().iter().map(|f| f.what.as_str()).collect();
        assert_eq!(
            complaints,
            ["[agents] harness `gemini` is no harness this tool knows"]
        );
        assert!(m.serves_claude());
    }

    /// The claim: the namespace is the prefixed skill directories and agent files, and the
    /// installer's own directory; a project's own skill and anything else are outside it.
    #[test]
    fn the_owned_namespace_is_named_by_its_prefix() {
        for owned in [
            ".claude/skills/knowledge-architect-planning/SKILL.md",
            ".claude/skills/knowledge-architect-planning/notes/a.md",
            ".claude/agents/knowledge-architect-routing-reviewer.md",
            ".claude/knowledge-architect/PRIMER.md",
        ] {
            assert!(owned_path(Path::new(owned)), "{owned}");
        }
        for other in [
            ".claude/skills/thaum-developing/SKILL.md",
            ".claude/agents/thaum-rules-reviewer.md",
            ".claude/knowledge-architect",
            ".claude/skills/knowledge-architect-planning",
            ".claude/skills/project-knowledge-architect-x/SKILL.md",
            ".claude/agents/project-knowledge-architect-x.md",
            ".claude/agents/knowledge-architect-dir/nested.md",
            "docs/knowledge-architect-x.md",
            "CLAUDE.md",
        ] {
            assert!(!owned_path(Path::new(other)), "{other}");
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let bytes = b"x\xff/.claude/agents/knowledge-architect-x.md";
            let odd = Path::new(std::ffi::OsStr::from_bytes(bytes));
            assert!(
                !owned_path(odd),
                "a component that is not UTF-8 owns nothing"
            );
        }
    }

    /// The claim: a command that cannot be printed on one line in a code span is refused in
    /// phase 1, and the default takes its place.
    #[test]
    fn an_unprintable_command_is_refused() {
        for bad in ["\"\"", "\"a\\nb\"", "\"a`b\""] {
            let text = format!(
                "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\ncommand = {bad}\n\n\
                 [walk]\nskip-dirs = []\nskip-files = []\n"
            );
            let m = Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration");
            assert_eq!(m.complaints().len(), 1, "{bad}");
            assert_eq!(m.command(), "klarch", "{bad}");
        }
    }

    /// The claim: the command is the declared one, and `klarch` when none is declared.
    #[test]
    fn the_command_is_declared_or_the_binary_name() {
        assert_eq!(with_agents("").command(), "klarch");
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\ncommand = \"cargo klarch\"\n\n\
                    [walk]\nskip-dirs = []\nskip-files = []\n";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        assert_eq!(m.command(), "cargo klarch");
    }

    /// A manifest whose `[project]` holds the given `checker-version` line, or none.
    fn with_pin(line: &str) -> Result<Manifest, String> {
        let text = format!(
            "[project]\n{line}\nname = \"p\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text)
    }

    /// The claim: `[project] checker-version` reads a version and the two sentinels, and refuses
    /// an absent key and a string of none of the three forms as a complaint whose text opens with
    /// the key, per `design@core@installed-binary-version-check`.
    #[test]
    fn the_checker_version_reads_three_forms_and_refuses_the_rest() {
        let read = |line: &str| with_pin(line).expect("a declaration");
        assert_eq!(
            read("checker-version = \"0.2.0\"").pin(),
            Some(&Pin::Version("0.2.0".to_string()))
        );
        assert_eq!(
            read("checker-version = \"10.0.1\"").pin(),
            Some(&Pin::Version("10.0.1".to_string()))
        );
        assert_eq!(
            read("checker-version = \"fixture\"").pin(),
            Some(&Pin::Fixture)
        );
        assert_eq!(
            read("checker-version = \"self\"").pin(),
            Some(&Pin::OwnBuild)
        );
        for good in ["\"fixture\"", "\"self\"", "\"0.0.0\""] {
            let m = read(&format!("checker-version = {good}"));
            assert!(m.complaints().is_empty(), "{good}: {:?}", m.complaints());
            assert!(m.pin_complaint().is_none(), "{good}");
        }
        let absent = read("");
        assert_eq!(absent.pin(), None);
        let complaint = absent.pin_complaint().expect("the key's complaint");
        assert_eq!(complaint.what, "[project] checker-version is absent");
        for bad in [
            "01.2.3",
            "0.2",
            "0.2.0-rc.1",
            "0.2.0+b",
            "v0.2.0",
            "latest",
            "",
            "1.2.3.4",
            "99999999999999999999.0.0",
        ] {
            let m = read(&format!("checker-version = \"{bad}\""));
            assert_eq!(m.pin(), None, "{bad}");
            let complaint = m.pin_complaint().expect("the key's complaint");
            assert!(
                complaint.what.starts_with("[project] checker-version")
                    && complaint
                        .what
                        .contains("is not a version, \"fixture\" or \"self\""),
                "{bad}: {complaint:?}"
            );
        }
    }

    /// The claim: a value of the key that is not a string fails the parse, as any mistyped key
    /// does.
    #[test]
    fn a_checker_version_that_is_not_a_string_fails_the_parse() {
        for bad in [
            "checker-version = 1",
            "checker-version = true",
            "checker-version = [\"0.2.0\"]",
        ] {
            assert!(with_pin(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn this_repository_declares_a_readable_manifest() {
        let m = Manifest::load(&this_project()).expect("the manifest");
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
            "[locations.papers]\npath = \"./docs/papers\"\nregisters = [\"issue\"]\n\n\
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
        assert_eq!(m.locations()["papers"].path, PathBuf::from("docs/papers"));
        assert!(!m.locations().contains_key("here"));
        assert_eq!(m.walk().skip_dirs, vec![PathBuf::from("build")]);
        assert_eq!(m.walk().skip_files, vec![PathBuf::from("docs/index.md")]);
        assert_eq!(m.walk().exclude, vec![PathBuf::from("vendor")]);
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

    /// The claim: every kind name is refused as the name of a Component, a location or the
    /// project, and a harness kind's name as a register's, per
    /// `design@core@anchors-are-components-and-locations`. Mutation checked: deleting the anchor
    /// refusal accepts the component `parts/design`.
    #[test]
    fn a_kind_name_is_refused_for_every_anchor_and_a_harness_kind_for_a_register() {
        let m = declaring_full(
            "\"parts/design\", \"parts/skill\"",
            "[registers.agent]\nscope = \"opt-in\"\nshape = \"file\"\n\n\
             [registers.note]\nscope = \"opt-in\"\nshape = \"file\"\n\n\
             [locations.primer]\npath = \"papers\"\nregisters = [\"issue\"]\n\n\
             [locations.note]\npath = \"notes\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let whats = whats(&m);
        for name in ["design", "skill", "primer", "note"] {
            assert!(
                whats.iter().any(|w| w.contains(&format!("the name `{name}`, which is the name of a kind"))),
                "{name}: {whats:#?}"
            );
        }
        assert!(
            whats
                .iter()
                .any(|w| w.contains("`agent` is a kind of the agent harness")),
            "{whats:#?}"
        );
        assert!(m.registers().by_name("agent").is_none());
        assert!(m.registers().by_name("note").is_some());
    }

    /// The same refusals under `harness = []`: the kind names are the tool's words, whatever the
    /// harness. Mutation checked: refusing the harness kinds only under `claude` accepts them.
    #[test]
    fn a_kind_name_is_refused_under_no_harness_too() {
        let m = declaring_full(
            "\"parts/skill\"",
            "[agents]\nharness = []\n\n[registers.agent]\nscope = \"opt-in\"\nshape = \"file\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let whats = whats(&m);
        assert!(
            whats
                .iter()
                .any(|w| w.contains("the name `skill`, which is the name of a kind")),
            "{whats:#?}"
        );
        assert!(
            whats
                .iter()
                .any(|w| w.contains("`agent` is a kind of the agent harness")),
            "{whats:#?}"
        );
    }

    #[test]
    fn a_register_named_planned_is_refused_as_a_reserved_kind() {
        // The claim, per `design@core@planned-path-form`: `planned` is a kind of the tree, so a
        // register of that name would never be reached by a reference.
        let m = declaring_full(
            "",
            "[registers.planned]\nscope = \"opt-in\"\nshape = \"file\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let whats = whats(&m);
        assert!(
            whats
                .iter()
                .any(|w| w.contains("`planned` is the reserved kind")),
            "{whats:?}"
        );
        assert!(m.registers().by_name("planned").is_none());
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
        // The `[ext] files` row belongs to an extension's table, which is the extension's to
        // normalise; the manifest keeps the table as written.
        assert_eq!(complaints.len(), 5, "{complaints:#?}");
        for label in ["[walk] skip-files", "[walk] exclude"] {
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
    fn the_built_in_registers_exist_before_anything_is_declared() {
        let m = declaring("");
        let names: Vec<&str> = m
            .registers()
            .all()
            .iter()
            .map(|r| r.name.as_str())
            .collect();
        assert_eq!(
            names,
            vec![
                "design",
                "goal",
                "tripwire",
                "issue",
                "spec",
                "milestone",
                "thread",
                "argument",
                "criterion",
                "acceptance"
            ]
        );
        // The item registers read the level-three headings of one section each.
        let thread = m
            .registers()
            .by_name("thread")
            .expect("the thread register");
        assert_eq!(
            (thread.shape, thread.scope, thread.level),
            (Shape::Section, Scope::OptIn, Some(3))
        );
        assert_eq!(thread.section.as_deref(), Some("Threads"));
        // The plan registers are carried by the plan anchors alone, so no component owes them.
        let spec = m.registers().by_name("spec").expect("the spec register");
        assert_eq!((spec.shape, spec.scope), (Shape::File, Scope::OptIn));
        assert_eq!(spec.dir, "specs");
        let milestone = m
            .registers()
            .by_name("milestone")
            .expect("the milestone register");
        assert_eq!(
            (milestone.shape, milestone.scope),
            (Shape::Directory, Scope::OptIn)
        );
        assert_eq!(milestone.dir, "milestones");
        assert!(m.plans());
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

    /// A register declaration, folded into the built-in registers.
    fn with_registers(body: &str) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             {body}\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             "
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
    fn the_built_in_heading_registers_carry_their_entry_level_and_issue_none() {
        let m = declaring("");
        let level = |name: &str| m.registers().by_name(name).expect("a register").level;
        assert_eq!(level("design"), Some(3));
        assert_eq!(level("goal"), Some(2));
        assert_eq!(level("tripwire"), Some(2));
        assert_eq!(level("issue"), None);
    }

    #[test]
    fn a_heading_register_declares_its_level_and_a_file_register_does_not() {
        let heading = "[registers.note]\nscope = \"opt-in\"\nshape = \"heading\"\n";
        let m = with_registers(&format!("{heading}level = 4\n"));
        assert!(whats(&m).is_empty(), "{:?}", whats(&m));
        assert_eq!(m.registers().by_name("note").expect("note").level, Some(4));
        // Missing: the check could not tell an entry from section text.
        let m = with_registers(heading);
        assert_eq!(whats(&m).len(), 1, "{:?}", whats(&m));
        assert!(
            whats(&m)[0].contains("declares no level"),
            "{:?}",
            whats(&m)
        );
        // Out of range. Level one is the document's title, and markdown has six levels.
        for bad in [0, 1, 7] {
            let m = with_registers(&format!("{heading}level = {bad}\n"));
            assert_eq!(whats(&m).len(), 1, "{bad}: {:?}", whats(&m));
            assert!(
                whats(&m)[0].contains(&format!("declares level {bad}")),
                "{bad}: {:?}",
                whats(&m)
            );
        }
        for good in [2, 6] {
            let m = with_registers(&format!("{heading}level = {good}\n"));
            assert!(whats(&m).is_empty(), "{good}: {:?}", whats(&m));
        }
        // Not an integer: refused when the manifest is read, naming the key.
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             {heading}level = \"3\"\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             "
        );
        let e = Manifest::parse(Path::new("/nowhere"), &text).expect_err("a string level");
        assert!(e.contains("level"), "{e}");
        // On a file register: its entries are files, so a level would be read by nothing.
        let m = with_registers(
            "[registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\nlevel = 2\n",
        );
        assert_eq!(whats(&m).len(), 1, "{:?}", whats(&m));
        assert!(
            whats(&m)[0].contains("on a file register"),
            "{:?}",
            whats(&m)
        );
        assert_eq!(
            m.registers().by_name("reading").expect("reading").level,
            None
        );
        // On a built-in register, whose level is compiled in.
        let m = with_registers("[registers.design]\nlevel = 2\n");
        assert_eq!(whats(&m).len(), 1, "{:?}", whats(&m));
        assert!(whats(&m)[0].contains("sets level"), "{:?}", whats(&m));
        assert_eq!(
            m.registers().by_name("design").expect("design").level,
            Some(3)
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
    fn the_walk_up_finds_the_root_from_below_it() {
        let root = this_project();
        let deep = root.join("crates/core/src");
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
