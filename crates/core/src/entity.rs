//! The entity table: everything a document can cite, and the one resolver every check reads.
//!
//! An entity has a **kind**, an **anchor**, an **id** and a **definition site**. The table is
//! built once from the model, and a reference `` `<kind>@<anchor>@<id>` `` is resolved against
//! it. Before this table five checks held five notions of a name; the argument is
//! `design@core@one-entity-table`.
//!
//! **A kind is a register's name, or `path`, or `planned`.** Ten registers are compiled in and a
//! project declares the rest, so the kind set is data rather than an enum —
//! `path@core@src/manifest.rs` owns what a register is, and this module owns what naming one
//! means.
//!
//! **An anchor is a named directory, or a spec file, that carries registers.** A component carries every
//! component-scoped register with its homes under `docs/`; a location carries the subset it
//! declares, with its homes directly under its own path. Both are the same shape, which is why
//! `Anchor` holds its register list and its home base as data. Three kinds of location are
//! constructed by the tool rather than declared: `plans`, at the plans directory, one anchor per
//! milestone directory and one per spec file, both read off the tree.
//!
//! **The `path` kind is resolved against the tree, not the table.** Its ids are paths, its
//! anchors are the same anchors but the plan anchors, plus two reserved words, and the check that
//! resolves it needs the survey. What this module gives it is the candidate rule, the
//! segmentation and the anchor lookup, so one grammar has one reader.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use regex::Regex;

use crate::finding::Finding;
use crate::manifest::{
    Manifest, Register, Registers, Shape, COMPONENT_DOCUMENTS, ITEM_REGISTERS, MILESTONES_HOME,
    MILESTONE_REGISTER, PLANS_DIR, SPECS_HOME, SPEC_REGISTER, STEP_SECTIONS,
};
use crate::model::Model;
use crate::scan::{Observation, SlugSite};

/// A kind that is not a register: a file or directory, defined by the tree itself.
pub(crate) const PATH_KIND: &str = "path";

/// The kind of a path a plan's work will create: anchored like `path`, and asserted absent, per
/// `design@core@planned-path-form`.
pub(crate) const PLANNED_KIND: &str = "planned";

/// What a reference names, in its first segment: a register's name, `path`, or `planned`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Kind(Arc<str>);

impl Kind {
    pub(crate) fn new(name: &str) -> Kind {
        Kind(Arc::from(name))
    }

    /// The kind of a file or directory.
    pub(crate) fn path() -> Kind {
        Kind::new(PATH_KIND)
    }

    pub(crate) fn is_path(&self) -> bool {
        &*self.0 == PATH_KIND
    }

    /// The kind of a path a plan's work will create.
    pub(crate) fn planned() -> Kind {
        Kind::new(PLANNED_KIND)
    }

    pub(crate) fn is_planned(&self) -> bool {
        &*self.0 == PLANNED_KIND
    }

    /// Whether the id is a path under the anchor, rather than an entity's id: `path` and
    /// `planned`, the two kinds the tree defines rather than a register.
    pub(crate) fn takes_a_path(&self) -> bool {
        self.is_path() || self.is_planned()
    }

    /// The word a reference spells.
    pub(crate) fn name(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The reserved anchor word for a path deliberately not resolvable in this tree.
///
/// A declared anchor may not take this name, which `manifest::resolve_anchors` asserts.
pub(crate) const ESCAPE_ANCHOR: &str = "elsewhere";

/// The reserved anchor for every component's own copy of a path.
pub(crate) const EVERY_ANCHOR: &str = "*";

/// The reserved name of the anchor the tool constructs at the plans directory.
pub(crate) const PLANS_ANCHOR: &str = "plans";

/// Whether a word is reserved by the tool, so no declared anchor and no plan may wear it: the
/// two words of the `path` kind, and the plans anchor's name.
pub(crate) fn is_reserved_anchor(word: &str) -> bool {
    word == ESCAPE_ANCHOR || word == EVERY_ANCHOR || word == PLANS_ANCHOR
}

/// The shape an anchor name must have for a reference to be able to name it.
///
/// A name a reference cannot spell is one every pointer at it misses silently, so the check
/// over the declaration asks this rather than assuming it.
static ANCHOR_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\.?[A-Za-z0-9][A-Za-z0-9._-]*$").unwrap());

/// The shape an entity's id must have, for every kind but `path` and `planned`.
static ENTITY_ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

/// Can a reference name an anchor called this.
pub(crate) fn is_anchor_name(name: &str) -> bool {
    ANCHOR_NAME.is_match(name)
}

/// Can an entity be given this id.
pub(crate) fn is_entity_id(id: &str) -> bool {
    ENTITY_ID.is_match(id)
}

/// Where one register's entries live, for one anchor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Home {
    /// The single-file shape of a heading register. Not a shape a file register has.
    pub file: PathBuf,
    /// The directory shape: a heading register's subdocuments, or a file register's entries.
    pub dir: PathBuf,
    /// The head of the directory shape. It defines nothing.
    pub readme: PathBuf,
    /// A file register's generated listing.
    pub index: PathBuf,
    /// A file register's per-instance options.
    pub config: PathBuf,
    pub shape: Shape,
}

/// A named directory carrying registers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Anchor {
    /// The one word a reference names it by.
    pub name: String,
    /// Project-relative, and empty for the component at the root.
    pub path: PathBuf,
    /// Where its register homes sit: `<path>/docs` for a component, `<path>` for a location.
    pub home_base: PathBuf,
    /// The registers it carries, by name. `path` is carried by every anchor and is not listed.
    pub registers: Vec<String>,
    /// Whether it is a component, which owes the compiled-in documents beside its registers.
    pub is_component: bool,
    /// Which location the tool constructs this is, or `None` for a declared anchor.
    pub constructed: Option<Constructed>,
}

/// The locations the tool constructs rather than a manifest row declares.
///
/// Each is a location in every respect but those this names, each a decision of the plans
/// layout: what `plans` owes, where a plan keeps its registers, and which kinds it carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Constructed {
    /// The anchor `plans` at the plans directory. It owes a `README.md` beside its two homes.
    Plans,
    /// One milestone directory. Its `spec` register has its home at the anchor's own path, it
    /// carries the four item registers, and it carries no `path` kind: a plan document is cited
    /// by its kind.
    Milestone,
    /// One spec file of specs/. It owns its own file, which stays an entry of the `spec` register
    /// of `plans` (`Anchors::owns_entry`), and carries the four item registers and no `path` kind.
    Spec,
}

/// The names of the four item registers, as an anchor lists them.
fn item_registers() -> impl Iterator<Item = String> {
    ITEM_REGISTERS.iter().map(|(name, _, _)| name.to_string())
}

impl Anchor {
    /// A component: every component-scoped register, homes under `docs/`.
    pub(crate) fn component(name: &str, path: &Path, registers: &Registers) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            home_base: path.join("docs"),
            registers: registers
                .component_scoped()
                .map(|r| r.name.clone())
                .collect(),
            is_component: true,
            constructed: None,
        }
    }

    /// A location: the registers it declares, homes directly under its own path.
    pub(crate) fn location(name: &str, path: &Path, registers: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            home_base: path.to_path_buf(),
            registers,
            is_component: false,
            constructed: None,
        }
    }

    /// The anchor `plans`, carrying the two plan registers.
    pub(crate) fn plans() -> Self {
        Self {
            constructed: Some(Constructed::Plans),
            ..Self::location(
                PLANS_ANCHOR,
                Path::new(PLANS_DIR),
                vec![SPEC_REGISTER.to_string(), MILESTONE_REGISTER.to_string()],
            )
        }
    }

    /// The anchor of one milestone directory, carrying `spec` at its own path and the items.
    pub(crate) fn milestone(name: &str, path: &Path) -> Self {
        let registers = std::iter::once(SPEC_REGISTER.to_string())
            .chain(item_registers())
            .collect();
        Self {
            constructed: Some(Constructed::Milestone),
            ..Self::location(name, path, registers)
        }
    }

    /// The anchor of one spec file, carrying the items.
    pub(crate) fn spec(name: &str, path: &Path) -> Self {
        Self {
            constructed: Some(Constructed::Spec),
            ..Self::location(name, path, item_registers().collect())
        }
    }

    /// Whether this is a milestone anchor.
    pub(crate) fn is_milestone(&self) -> bool {
        self.constructed == Some(Constructed::Milestone)
    }

    /// Whether this is a plan anchor, a milestone or a spec: it carries the item registers and
    /// no `path` kind, and its name is no anchor word.
    pub(crate) fn is_plan(&self) -> bool {
        matches!(
            self.constructed,
            Some(Constructed::Milestone | Constructed::Spec)
        )
    }

    /// The level-two sections an entry of `register` owes at this anchor: a step spec, a `spec`
    /// entry of a milestone, owes the step sections, and every other entry its register's own.
    pub(crate) fn sections_of(&self, register: &Register) -> Vec<String> {
        if self.is_milestone() && register.name == SPEC_REGISTER {
            return STEP_SECTIONS.iter().map(|s| s.to_string()).collect();
        }
        register.sections.clone()
    }

    /// Whether this is the component at the project root.
    pub(crate) fn is_root(&self) -> bool {
        self.path.as_os_str().is_empty()
    }

    /// Whether a reference of this kind may anchor here. Every anchor carries `path` but a
    /// plan anchor, whose documents are cited by their kind.
    pub(crate) fn carries(&self, kind: &Kind) -> bool {
        if kind.takes_a_path() {
            return !self.is_plan();
        }
        self.registers.iter().any(|r| r == kind.name())
    }

    /// The home of one register here, given that register's declaration.
    ///
    /// A milestone keeps its `spec` register at its own path, so the File shape's retired
    /// single file is the `<id>.md` beside the milestone directory. An item register's home is
    /// the plan's own documents, which no check asserts: a spec anchor's is its file, and a
    /// milestone's its directory.
    pub(crate) fn home_of(&self, register: &Register) -> Home {
        let (dir, file) = if self.constructed == Some(Constructed::Spec) {
            (self.path.clone(), self.path.clone())
        } else if self.is_milestone() {
            (self.path.clone(), self.path.with_extension("md"))
        } else {
            (
                self.home_base.join(&register.dir),
                self.home_base.join(format!("{}.md", register.dir)),
            )
        };
        Home {
            file,
            readme: dir.join("README.md"),
            index: dir.join("index.md"),
            config: dir.join("register.toml"),
            dir,
            shape: register.shape,
        }
    }
}

/// Every anchor of a project, and the registers they carry.
#[derive(Clone, Debug)]
pub(crate) struct Anchors {
    list: Vec<Anchor>,
    registers: Registers,
}

impl Anchors {
    /// The anchors of one tree: the components and locations its manifest declares, then the
    /// anchor `plans` and one anchor per milestone directory the tree holds.
    ///
    /// **`paths` is the tree's listing**, files and directories or files alone: the milestone
    /// anchors are read off it, so every caller that builds the anchors of a tree hands in that
    /// tree's own paths, a commit's included. A milestone directory whose name
    /// [`milestone_refusal`] refuses is no anchor, and `check::tree` reports it.
    pub(crate) fn of<'a>(
        manifest: &Manifest,
        paths: impl IntoIterator<Item = &'a PathBuf>,
    ) -> Self {
        let registers = manifest.registers().clone();
        let mut list: Vec<Anchor> = manifest
            .components()
            .all()
            .iter()
            .map(|c| Anchor::component(&c.name, &c.path, &registers))
            .collect();
        for (name, decl) in manifest.locations() {
            list.push(Anchor::location(name, &decl.path, decl.registers.clone()));
        }
        if manifest.plans() {
            list.push(Anchor::plans());
            let paths: Vec<&PathBuf> = paths.into_iter().collect();
            let milestones = milestone_dirs(paths.iter().copied());
            for (name, path) in &milestones {
                if milestone_refusal(name, manifest).is_none() {
                    list.push(Anchor::milestone(name, path));
                }
            }
            // A spec whose name a milestone refusal would refuse, or that a milestone also
            // holds, is no anchor: one name gives one anchor, and the milestone keeps it. The
            // collision is clause P2's finding, in phase 2.
            for (name, path) in spec_files(paths.iter().copied()) {
                if milestone_refusal(&name, manifest).is_none() && !milestones.contains_key(&name) {
                    list.push(Anchor::spec(&name, &path));
                }
            }
        }
        Self { list, registers }
    }

    /// The anchors of a manifest over a tree that places no milestone, for a test about the
    /// declared anchors alone.
    #[cfg(test)]
    pub(crate) fn declared(manifest: &Manifest) -> Self {
        Self::of(manifest, &[] as &[PathBuf])
    }

    /// Anchors stated directly, for a test that needs a register list no component has.
    #[cfg(test)]
    pub(crate) fn from_list(list: Vec<Anchor>, registers: Registers) -> Self {
        Self { list, registers }
    }

    pub(crate) fn all(&self) -> &[Anchor] {
        &self.list
    }

    pub(crate) fn registers(&self) -> &Registers {
        &self.registers
    }

    /// The kind a word names: a declared register, `path`, or `planned`.
    pub(crate) fn kind(&self, word: &str) -> Option<Kind> {
        if word == PATH_KIND {
            return Some(Kind::path());
        }
        if word == PLANNED_KIND {
            return Some(Kind::planned());
        }
        self.registers.by_name(word).map(|r| Kind::new(&r.name))
    }

    /// Every kind name, comma-separated, as a finding lists them.
    pub(crate) fn kinds_listed(&self) -> String {
        let mut names: Vec<&str> = self
            .registers
            .all()
            .iter()
            .map(|r| r.name.as_str())
            .collect();
        names.push(PATH_KIND);
        names.push(PLANNED_KIND);
        names.join(", ")
    }

    /// The anchor a document belongs to: the deepest whose path holds it, and the root where
    /// none does. The root's path is empty and is a prefix of every path, which is what makes
    /// it the fallback rather than a case.
    ///
    /// **Locations count here, and that is the point.** A location sits inside a component —
    /// as both of thaum's do — and its register homes are its own, so a document under
    /// it belongs to it and not to the component above. A component inside a location would
    /// make the two ambiguous, and `manifest::collides` refuses one.
    pub(crate) fn owning(&self, rel: &Path) -> &Anchor {
        // Depth first, and a component on a tie: two anchors at one path is a declaration
        // `manifest::collides` refuses, and until it is repaired the component keeps its own
        // documents rather than every slug in them being reported as misplaced.
        self.list
            .iter()
            .filter(|a| rel.starts_with(&a.path))
            .max_by_key(|a| (a.path.components().count(), a.is_component))
            .expect("the anchor at the root is a prefix of every path")
    }

    /// The anchor a reference names, or `None` where nothing declares that name.
    pub(crate) fn by_name(&self, name: &str) -> Option<&Anchor> {
        self.list.iter().find(|a| a.name == name)
    }

    /// Every anchor name, comma-separated, as a finding lists them.
    pub(crate) fn listed(&self) -> String {
        self.list
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Whether `word` could stand in the anchor position of some reference: a declared
    /// anchor, or a word the tool reserves.
    ///
    /// A milestone's name is not one: the retired form this answers for is a path written with
    /// no kind, and a milestone carries no `path` kind, so no such span ever named one. Its name
    /// comes from the tree, so counting it would turn an email address or a git remote in an
    /// unrelated document into a finding the moment a milestone of that name is created.
    pub(crate) fn is_anchor_word(&self, word: &str) -> bool {
        is_reserved_anchor(word) || self.by_name(word).is_some_and(|a| !a.is_plan())
    }

    /// Whether `rel` is an entry of `anchor`'s file register: a file the anchor owns, or a spec
    /// file, which a spec anchor at exactly its own path owns and which stays an entry of the
    /// `spec` register that holds it.
    ///
    /// The one place that case is written: every filter that decides whether a file is an
    /// entry of a File register asks this, so no later filter forgets a spec.
    pub(crate) fn owns_entry(&self, anchor: &Anchor, rel: &Path) -> bool {
        let owner = self.owning(rel);
        owner.path == anchor.path
            || (owner.constructed == Some(Constructed::Spec) && owner.path == rel)
    }

    /// The register home an anchor keeps one register in, or `None` where it carries none.
    pub(crate) fn home(&self, anchor: &Anchor, kind: &Kind) -> Option<Home> {
        if !anchor.carries(kind) {
            return None;
        }
        self.registers
            .by_name(kind.name())
            .map(|r| anchor.home_of(r))
    }

    /// Every register instance of the project: an anchor, the register, and its home.
    pub(crate) fn instances(&self) -> Vec<(&Anchor, &Register, Home)> {
        let mut out = Vec::new();
        for anchor in &self.list {
            for name in &anchor.registers {
                if let Some(register) = self.registers.by_name(name) {
                    out.push((anchor, register, anchor.home_of(register)));
                }
            }
        }
        out
    }

    /// Whether a component-relative path is a required document name in one of its shapes,
    /// and whether that shape is a directory: what the generic anchor `path@*@<path>` accepts
    /// whether or not any component carries it yet.
    ///
    /// The compiled-in document set, and every component register's homes: a heading
    /// register's file, directory and README, and a file register's directory, README and
    /// index. Naming a shape no component uses yet is legitimate.
    pub(crate) fn required_kind(&self, path: &str) -> Option<bool> {
        if COMPONENT_DOCUMENTS.contains(&path) {
            return Some(false);
        }
        for register in self.registers.component_scoped() {
            let dir = &register.dir;
            if path == format!("docs/{dir}") {
                return Some(true);
            }
            if path == format!("docs/{dir}/README.md") {
                return Some(false);
            }
            match register.shape {
                Shape::Heading if path == format!("docs/{dir}.md") => return Some(false),
                Shape::File if path == format!("docs/{dir}/index.md") => return Some(false),
                _ => {}
            }
        }
        None
    }
}

/// Each directory directly under the milestones home that holds a `README.md`, by basename:
/// the milestone anchors a tree places, before [`milestone_refusal`] judges their names.
///
/// A directory with no `README.md` is no milestone, and `check::tree` reports it. So is one
/// whose `README.md` is a directory: a listing of files and directories holds that path too,
/// and a listing of files alone does not, so it is read off what sits under it. Every caller
/// then builds the same anchors, whichever listing it holds.
pub(crate) fn milestone_dirs<'a>(
    paths: impl IntoIterator<Item = &'a PathBuf>,
) -> BTreeMap<String, PathBuf> {
    let home = Path::new(PLANS_DIR).join(MILESTONES_HOME);
    let mut out = BTreeMap::new();
    let mut not_files = Vec::new();
    for path in paths {
        let Ok(inside) = path.strip_prefix(&home) else {
            continue;
        };
        let parts: Vec<_> = inside.components().collect();
        match parts.as_slice() {
            [dir, readme] if readme.as_os_str() == "README.md" => {
                let name = dir.as_os_str().to_string_lossy().into_owned();
                out.insert(name, home.join(dir));
            }
            [dir, readme, _, ..] if readme.as_os_str() == "README.md" => {
                not_files.push(dir.as_os_str().to_string_lossy().into_owned());
            }
            _ => {}
        }
    }
    for name in not_files {
        out.remove(&name);
    }
    out
}

/// Whether a path holds a plan document by its position: an entry of specs/, or a file inside
/// a directory of milestones/.
fn is_plan_position(rel: &Path) -> bool {
    let plans = Path::new(PLANS_DIR);
    if entry_id(rel, &plans.join(SPECS_HOME)).is_some() {
        return true;
    }
    rel.extension().is_some_and(|e| e == "md")
        && rel
            .strip_prefix(plans.join(MILESTONES_HOME))
            .is_ok_and(|inside| inside.components().count() >= 2)
}

/// Each spec file of specs/, grouped or not, by id: the spec anchors a tree places, before
/// their names are judged. A second file of one id is the duplicate entry finding, and the
/// first in path order, which compares components, is the anchor.
pub(crate) fn spec_files<'a>(
    paths: impl IntoIterator<Item = &'a PathBuf>,
) -> BTreeMap<String, PathBuf> {
    let home = Path::new(PLANS_DIR).join(SPECS_HOME);
    // A listing of files and directories holds a group directory named like an entry, and a
    // listing of files alone does not, so a path under which another sits is no spec: every
    // caller then builds the same anchors.
    let all: Vec<&PathBuf> = paths.into_iter().collect();
    let mut found: Vec<&PathBuf> = all
        .iter()
        .copied()
        .filter(|p| entry_id(p, &home).is_some())
        .filter(|p| !all.iter().any(|q| q != p && q.starts_with(p)))
        .collect();
    found.sort();
    let mut out = BTreeMap::new();
    for path in found {
        if let Some(id) = entry_id(path, &home) {
            out.entry(id).or_insert_with(|| path.clone());
        }
    }
    out
}

/// Why a milestone directory's name makes it no anchor, or `None` when it is one.
///
/// A milestone is cited `spec@<id>@<step>` and `milestone@plans@<id>`, so its name is an entity
/// id, and it is no other anchor's name and no reserved word, or a reference would read as the
/// other anchor, per `design@core@a-plan-name-reads-as-nothing-else`. Judged against the anchors the manifest
/// accepted.
pub(crate) fn milestone_refusal(name: &str, manifest: &Manifest) -> Option<String> {
    if !is_entity_id(name) {
        return Some("is not lower-case words joined by hyphens".to_string());
    }
    if is_reserved_anchor(name) {
        return Some("is a word the tool reserves".to_string());
    }
    // The `<id>.md` beside a milestone is its home's retired single file, and beside `index`
    // that is the milestones home's own generated listing.
    if name == "index" {
        return Some("names the listing of milestones/".to_string());
    }
    if manifest.components().all().iter().any(|c| c.name == name) {
        return Some("is the name of a component".to_string());
    }
    if manifest.locations().contains_key(name) {
        return Some("is the name of a location".to_string());
    }
    None
}

/// Where something was written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Site {
    pub file: PathBuf,
    pub line: u32,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file.display(), self.line)
    }
}

/// What a backticked `@` span is, under the candidate rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Candidate<'a> {
    /// A well-formed reference: kind, anchor word as written, id as written.
    Reference {
        kind: Kind,
        anchor: &'a str,
        id: &'a str,
    },
    /// A known kind followed by the wrong number of segments, or an empty one.
    Malformed { why: &'static str },
    /// The head is an anchor, or a reserved anchor, where a kind goes: the old form.
    AnchorInKindPosition { head: &'a str },
    /// The head is neither a kind nor an anchor. Not a reference; silent.
    NotOne,
}

/// The candidate rule and the segmentation, over one backticked span holding an `@`.
///
/// **Which spans are candidates.** The head — the text before the first `@` — is a known kind,
/// a declared anchor or a reserved anchor; anything else is not a reference and is silent, so
/// an email address or a git remote in backticks reports nothing. A typo inside the kind is
/// silent for the same reason, which `path@core@docs/tripwires.md` guards.
///
/// **Segmentation.** Every kind but `path` and `planned` takes exactly three segments; those two
/// take an anchor and then everything after the second `@` as its id, so a path may hold an `@`.
/// An empty segment is malformed in either shape.
pub(crate) fn candidate<'a>(span: &'a str, anchors: &Anchors) -> Candidate<'a> {
    let Some((head, rest)) = span.split_once('@') else {
        return Candidate::NotOne;
    };
    // An empty head in front of a path shape is the retired `@` escape, or a kind that
    // was never typed; either way a pointer that would otherwise leave every check. A
    // slashless `@word` is an annotation or a handle and stays silent.
    if head.is_empty() && rest.contains('/') {
        return Candidate::Malformed {
            why: "the kind segment is empty",
        };
    }
    let Some(kind) = anchors.kind(head) else {
        if anchors.is_anchor_word(head) {
            return Candidate::AnchorInKindPosition { head };
        }
        return Candidate::NotOne;
    };
    let Some((anchor, id)) = rest.split_once('@') else {
        return Candidate::Malformed {
            why: "two segments; a reference has three",
        };
    };
    if anchor.is_empty() {
        return Candidate::Malformed {
            why: "the anchor segment is empty",
        };
    }
    if id.is_empty() {
        return Candidate::Malformed {
            why: "the id segment is empty",
        };
    }
    if !kind.takes_a_path() && id.contains('@') {
        return Candidate::Malformed {
            why: "four or more segments; a reference has three",
        };
    }
    Candidate::Reference { kind, anchor, id }
}

/// Why a reference to a table kind resolves to nothing, or that it resolves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Resolution {
    Resolved,
    /// Nothing declares that anchor name.
    UnknownAnchor,
    /// The anchor exists and carries no register of that kind; the anchors that do.
    AnchorLacksRegister {
        carriers: Vec<String>,
    },
    /// The anchor carries the register and defines no such id.
    Undefined,
    /// An item of a plan cited from a file outside that plan; the whole-document form that
    /// may be cited instead.
    OutsidePlan {
        form: String,
    },
}

/// The table: every entity defined in the walk, keyed by kind, anchor and id.
#[derive(Debug, Default)]
pub(crate) struct Entities {
    defined: BTreeMap<(Kind, String, String), Vec<Site>>,
    /// Misplaced, malformed and duplicate definitions, found while building.
    findings: Vec<Finding>,
}

impl Entities {
    /// Read every definition out of the model and file it under its anchor and register.
    ///
    /// A heading register's entity is a slug ending a heading at the register's declared
    /// level, in one of that register's home shapes, owned by the deepest anchor whose path
    /// holds the document. A file register's entity is a file under the instance directory,
    /// its id the basename. A slug outside every heading home, at a heading of another level,
    /// in a table cell, at the head of a plain line or mid-line defines nothing and is
    /// reported as misplaced; a heading at the declared level that carries no slug is
    /// reported too, because it reads as an entry and nothing lists it.
    pub(crate) fn build(model: &Model, anchors: &Anchors) -> Self {
        let mut out = Self::default();
        out.heading_definitions(model, anchors);
        out.file_definitions(model, anchors);
        out.report_duplicates();
        out
    }

    fn heading_definitions(&mut self, model: &Model, anchors: &Anchors) {
        for doc in model.documents() {
            // Two passes produce this document's findings, and a reader walks it top to
            // bottom, so they are put in line order once both are done.
            let first = self.findings.len();
            let owner = anchors.owning(&doc.rel);
            if owner.constructed == Some(Constructed::Plans) && is_plan_position(&doc.rel) {
                self.unanchored_plan(doc, owner);
                continue;
            }
            if owner.is_plan() && doc.is_markdown() {
                self.item_definitions(doc, owner, anchors);
                self.findings[first..].sort_by_key(|f| f.line);
                continue;
            }
            let home = register_of(anchors, owner, &doc.rel);
            if let Some(Ok(register)) = home {
                self.unslugged_headings(doc, register);
            }
            for l in &doc.observations {
                let Observation::SlugDef { id, site } = &l.what else {
                    continue;
                };
                let misplaced = |why: String| {
                    Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`##{id}` is written {why} and defines nothing"),
                        format!(
                            "a slug is defined at the end of a heading at its register's \
                             level, inside that register's home; this anchor's heading \
                             registers are {}; move it there, or delete it",
                            heading_homes(anchors, owner)
                        ),
                    )
                };
                let level = match site {
                    SlugSite::Heading(level) => *level,
                    SlugSite::Cell => {
                        self.findings.push(misplaced("in a table cell".to_string()));
                        continue;
                    }
                    SlugSite::LineHead => {
                        self.findings
                            .push(misplaced("at the head of a plain line".to_string()));
                        continue;
                    }
                    SlugSite::Inline => {
                        self.findings
                            .push(misplaced("in the middle of a line".to_string()));
                        continue;
                    }
                };
                match &home {
                    Some(Ok(register)) if register.level == Some(level) => self
                        .defined
                        .entry((Kind::new(&register.name), owner.name.clone(), id.clone()))
                        .or_default()
                        .push(Site {
                            file: doc.rel.clone(),
                            line: l.line,
                        }),
                    Some(Ok(_)) => self
                        .findings
                        .push(misplaced(format!("at a level-{level} heading"))),
                    Some(Err(dir)) => self.findings.push(misplaced(format!(
                        "in the README of the `{}` directory home",
                        dir.display()
                    ))),
                    None => self.findings.push(misplaced(format!(
                        "in `{}`, which is no heading register home of `{}`",
                        doc.rel.display(),
                        owner.name
                    ))),
                }
            }
            self.findings[first..].sort_by_key(|f| f.line);
        }
    }

    /// The items of one plan document: a slug ending a level-three heading under one of the
    /// four item sections defines an entry of that section's register, in the plan anchor
    /// that owns the document.
    ///
    /// The section is the level-two heading in force above the slug's line, read from the
    /// heading observations the scanner records, so a fenced heading neither opens a section
    /// nor defines anything. Only the item sections owe slugs: a level-three heading anywhere
    /// else in a plan document is section text, and a slug anywhere else defines nothing.
    fn item_definitions(
        &mut self,
        doc: &crate::model::Document,
        owner: &Anchor,
        anchors: &Anchors,
    ) {
        let items: Vec<&Register> = owner
            .registers
            .iter()
            .filter_map(|n| anchors.registers().by_name(n))
            .filter(|r| r.shape == Shape::Section)
            .collect();
        let headings: Vec<(u32, u8, &str)> = doc
            .observations
            .iter()
            .filter_map(|l| match &l.what {
                Observation::Heading { level, text } => Some((l.line, *level, text.as_str())),
                _ => None,
            })
            .collect();
        // The register whose section is in force at a line: the last heading of level one or
        // two above it, when that heading is an item section.
        let section_at = |line: u32| -> Option<&Register> {
            let (_, level, text) = headings
                .iter()
                .rev()
                .find(|(l, level, _)| *l < line && *level <= 2)?;
            if *level != 2 {
                return None;
            }
            items
                .iter()
                .copied()
                .find(|r| r.section.as_deref() == Some(*text))
        };
        let slugged: std::collections::HashSet<u32> = doc
            .observations
            .iter()
            .filter(|l| matches!(l.what, Observation::SlugDef { .. }))
            .map(|l| l.line)
            .collect();
        for (line, level, text) in &headings {
            if *level != 3 || slugged.contains(line) {
                continue;
            }
            if let Some(register) = section_at(*line) {
                self.findings.push(Finding::at(
                    &doc.rel,
                    *line,
                    format!(
                        "the level-3 heading \"{text}\" under {} carries no slug",
                        register.section.as_deref().unwrap_or_default()
                    ),
                    format!(
                        "every level-3 heading under an item section is a {} entry: end it with \
                         its slug, written as two hashes and the id in backticks, or move it out \
                         of the section if it is no item",
                        register.name
                    ),
                ));
            }
        }
        let sections: Vec<&str> = items.iter().filter_map(|r| r.section.as_deref()).collect();
        for l in &doc.observations {
            let Observation::SlugDef { id, site } = &l.what else {
                continue;
            };
            let register = match site {
                SlugSite::Heading(3) => section_at(l.line),
                _ => None,
            };
            // A slug at another level inside an item section is misplaced for its level, not
            // for its section.
            if let (None, SlugSite::Heading(level), Some(section)) =
                (register, site, section_at(l.line))
            {
                self.findings.push(Finding::at(
                    &doc.rel,
                    l.line,
                    format!(
                        "`##{id}` is written at a level-{level} heading of the {} section, and \
                         defines nothing",
                        section.section.as_deref().unwrap_or_default()
                    ),
                    "an item is defined at the end of a level-3 heading under its section; move \
                     it to one, or delete it",
                ));
                continue;
            }
            match register {
                Some(register) => self
                    .defined
                    .entry((Kind::new(&register.name), owner.name.clone(), id.clone()))
                    .or_default()
                    .push(Site {
                        file: doc.rel.clone(),
                        line: l.line,
                    }),
                None => self.findings.push(Finding::at(
                    &doc.rel,
                    l.line,
                    format!(
                        "`##{id}` is written outside the item sections of the plan `{}` and \
                         defines nothing",
                        owner.name
                    ),
                    format!(
                        "an item of a plan is defined at the end of a level-3 heading under one \
                         of its sections {}; move it there, or delete it",
                        sections.join(", ")
                    ),
                )),
            }
        }
    }

    /// The slugs of a plan document whose plan is no anchor: a spec or a milestone whose name
    /// the plans layout refuses, or that another plan holds. One finding per slug, naming the
    /// cause, which phase 2 or phase 3 reports on the name itself, rather than a repair that
    /// sends the reader to move each slug.
    fn unanchored_plan(&mut self, doc: &crate::model::Document, owner: &Anchor) {
        for l in &doc.observations {
            let Observation::SlugDef { id, .. } = &l.what else {
                continue;
            };
            self.findings.push(Finding::at(
                &doc.rel,
                l.line,
                format!(
                    "`##{id}` is written in a plan that is no anchor of its own, and defines \
                     nothing"
                ),
                format!(
                    "repair the plan's name, which a finding of its own names; its items are \
                     read once it is an anchor under `{}`",
                    owner.name
                ),
            ));
        }
    }

    /// Report every heading at the register's level that carries no slug.
    ///
    /// Every heading at that level in the register's home is an entry, so one without a slug
    /// is an entry nothing lists and no reference can name. A heading at another level is
    /// section text and owes nothing.
    fn unslugged_headings(&mut self, doc: &crate::model::Document, register: &Register) {
        let Some(level) = register.level else {
            return;
        };
        // The scanner records the first slug of a heading line at that heading's own level,
        // so any slug on the line is the heading's.
        let slugged: std::collections::HashSet<u32> = doc
            .observations
            .iter()
            .filter(|l| matches!(l.what, Observation::SlugDef { .. }))
            .map(|l| l.line)
            .collect();
        for l in &doc.observations {
            match &l.what {
                Observation::Heading { level: n, text }
                    if *n == level && !slugged.contains(&l.line) =>
                {
                    self.findings.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!(
                            "the level-{level} heading \"{text}\" in a {} home carries no slug",
                            register.name
                        ),
                        format!(
                            "every level-{level} heading in a {} home is an entry: end it with \
                             its slug, written as two hashes and the id in backticks, or move \
                             it to another level if it is no entry",
                            register.name
                        ),
                    ));
                }
                _ => {}
            }
        }
    }

    /// One entity per file under a file register's instance directory.
    ///
    /// The two navigation files at the instance's top level are not entries. One inside a
    /// group is: a group holds entries and nothing else, so a `README.md` there is an entry
    /// whose id no reference can spell, which is reported rather than passed over.
    fn file_definitions(&mut self, model: &Model, anchors: &Anchors) {
        for (anchor, register, home) in anchors.instances() {
            if register.shape == Shape::Directory {
                self.directory_definitions(model, anchors, anchor, register, &home);
                continue;
            }
            if register.shape != Shape::File {
                continue;
            }
            for doc in model.documents() {
                let Some(id) = entry_id(&doc.rel, &home.dir) else {
                    continue;
                };
                // A document an anchor nested inside this home owns is that anchor's, not
                // an entry here: the nesting is `manifest::collides`' finding, and reading the
                // nested anchor's files as entries would report it against the wrong register.
                // The one exception is a spec file, per `Anchors::owns_entry`.
                if !anchors.owns_entry(anchor, &doc.rel) {
                    continue;
                }
                let at = Site {
                    file: doc.rel.clone(),
                    line: 1,
                };
                // The two navigation names are not entry ids at any depth. At the instance's
                // top level they are the README and the index; inside a group they are an
                // entry wearing a name that means something else, and `index` passes the id
                // grammar, so refusing it by name is what makes the pair symmetric.
                let navigation = matches!(
                    doc.rel.file_name().and_then(|n| n.to_str()),
                    Some("README.md" | "index.md")
                );
                if navigation || !is_entity_id(&id) {
                    self.findings.push(Finding::in_file(
                        &doc.rel,
                        format!(
                            "`{id}` cannot be an entry id of the {} register",
                            register.name
                        ),
                        "name the file in lower-case words joined by hyphens, and not \
                         `README` or `index`, which name a directory's head and its listing; \
                         an id no reference can spell is an entry nothing points at",
                    ));
                    continue;
                }
                self.defined
                    .entry((Kind::new(&register.name), anchor.name.clone(), id))
                    .or_default()
                    .push(at);
            }
        }
    }

    /// One entity per milestone anchor directly under a Directory home, defined at its README.
    ///
    /// The entries are anchors, so they are read off the anchor list rather than off the
    /// documents: a directory the tree holds that is no anchor is `check::tree`'s finding. A
    /// milestone whose README the walk does not read defines nothing, as a File entry the walk
    /// leaves out defines nothing, so every reference to it dangles rather than passing over a
    /// document no check judged.
    fn directory_definitions(
        &mut self,
        model: &Model,
        anchors: &Anchors,
        anchor: &Anchor,
        register: &Register,
        home: &Home,
    ) {
        for entry in anchors.all() {
            if !entry.is_milestone() || entry.path.parent() != Some(home.dir.as_path()) {
                continue;
            }
            let readme = entry.path.join("README.md");
            if !model.documents().iter().any(|d| d.rel == readme) {
                continue;
            }
            self.defined
                .entry((
                    Kind::new(&register.name),
                    anchor.name.clone(),
                    entry.name.clone(),
                ))
                .or_default()
                .push(Site {
                    file: readme,
                    line: 1,
                });
        }
    }

    fn report_duplicates(&mut self) {
        let mut found = Vec::new();
        for ((kind, anchor, id), sites) in &self.defined {
            if sites.len() < 2 {
                continue;
            }
            // One finding at each site, each naming the others, so the reader who opens
            // either copy is told the other exists.
            for site in sites {
                let others: Vec<String> = sites
                    .iter()
                    .filter(|s| *s != site)
                    .map(Site::to_string)
                    .collect();
                found.push(Finding::at(
                    &site.file,
                    site.line,
                    format!(
                        "`{kind}@{anchor}@{id}` is also defined at {}",
                        others.join(", ")
                    ),
                    "keep one definition; a reference resolves to exactly one entity",
                ));
            }
        }
        self.findings.extend(found);
    }

    /// Resolve a reference to a table kind, with no citing file. `path` is not this table's to
    /// resolve.
    #[cfg(test)]
    pub(crate) fn resolve(
        &self,
        anchors: &Anchors,
        kind: &Kind,
        anchor: &str,
        id: &str,
    ) -> Resolution {
        self.resolve_from(anchors, kind, anchor, id, None)
    }

    /// The same, from a citing file: an item of a plan resolves only from inside that plan,
    /// and that is judged before the id, so an item cited from outside gets the repair that
    /// applies whether or not it exists. A citing file of `None` is inside every plan.
    pub(crate) fn resolve_from(
        &self,
        anchors: &Anchors,
        kind: &Kind,
        anchor: &str,
        id: &str,
        citing: Option<&Path>,
    ) -> Resolution {
        let Some(a) = anchors.by_name(anchor) else {
            return Resolution::UnknownAnchor;
        };
        if !a.carries(kind) {
            return Resolution::AnchorLacksRegister {
                carriers: anchors
                    .all()
                    .iter()
                    .filter(|a| a.carries(kind))
                    .map(|a| a.name.clone())
                    .collect(),
            };
        }
        let item = anchors
            .registers()
            .by_name(kind.name())
            .is_some_and(|r| r.shape == Shape::Section);
        if let (true, true, Some(citing)) = (item, a.is_plan(), citing) {
            if anchors.owning(citing).path != a.path {
                // The whole document that defines the item: a step spec, when one does, and the
                // milestone otherwise; an undefined item names the milestone.
                let step = self
                    .defined
                    .get(&(kind.clone(), anchor.to_string(), id.to_string()))
                    .and_then(|sites| sites.first())
                    .filter(|site| site.file.file_name().is_some_and(|n| n != "README.md"))
                    .and_then(|site| site.file.file_stem())
                    .map(|stem| stem.to_string_lossy().into_owned());
                let form = if let (true, Some(step)) = (a.is_milestone(), step) {
                    format!("{SPEC_REGISTER}@{}@{step}", a.name)
                } else if a.is_milestone() {
                    format!("{MILESTONE_REGISTER}@{PLANS_ANCHOR}@{}", a.name)
                } else {
                    format!("{SPEC_REGISTER}@{PLANS_ANCHOR}@{}", a.name)
                };
                return Resolution::OutsidePlan { form };
            }
        }
        if self
            .defined
            .contains_key(&(kind.clone(), anchor.to_string(), id.to_string()))
        {
            Resolution::Resolved
        } else {
            Resolution::Undefined
        }
    }

    /// Every entity of one kind, as anchor, id and the sites that define it.
    ///
    /// In `(anchor, id)` order, which is the table's own, so a listing that sorts by something
    /// else still starts from one order rather than from a hash.
    pub(crate) fn of_kind(&self, kind: &Kind) -> Vec<(&str, &str, &[Site])> {
        self.defined
            .iter()
            .filter(|((k, _, _), _)| k == kind)
            .map(|((_, anchor, id), sites)| (anchor.as_str(), id.as_str(), sites.as_slice()))
            .collect()
    }

    /// Whether any register of any anchor defines an entry with this id.
    pub(crate) fn defines_id(&self, id: &str) -> bool {
        self.defined.keys().any(|(_, _, defined)| defined == id)
    }

    /// How many distinct entities the table holds.
    pub(crate) fn len(&self) -> usize {
        self.defined.len()
    }

    /// The misplaced, malformed and duplicate definitions found while building the table.
    pub(crate) fn definition_findings(&self) -> &[Finding] {
        &self.findings
    }
}

/// The id of a file-register entry at `rel`, for an instance rooted at `dir`, or `None`.
///
/// `None` for anything that is not an entry: a file outside the instance, a file of another
/// suffix, and the two navigation files at the instance's own top level.
pub(crate) fn entry_id(rel: &Path, dir: &Path) -> Option<String> {
    let inside = rel.strip_prefix(dir).ok()?;
    if rel.extension().is_none_or(|e| e != "md") {
        return None;
    }
    let depth = inside.components().count();
    let name = inside.file_name()?.to_string_lossy();
    if depth == 1 && (name == "README.md" || name == "index.md") {
        return None;
    }
    Some(rel.file_stem()?.to_string_lossy().into_owned())
}

/// Which heading register home of `owner` holds `rel`: `Ok(register)` for the file home or a
/// subdocument of the directory home, `Err(dir)` for the directory home's README, `None`
/// for a file that is no heading home.
///
/// Markdown only: a home holds documents, so a source file under a directory home is not one,
/// and its comments' headings owe no slug and define nothing.
fn register_of<'a>(
    anchors: &'a Anchors,
    owner: &Anchor,
    rel: &Path,
) -> Option<Result<&'a Register, PathBuf>> {
    if rel.extension().is_none_or(|e| e != "md") {
        return None;
    }
    for name in &owner.registers {
        let register = anchors.registers().by_name(name)?;
        if register.shape != Shape::Heading {
            continue;
        }
        let home = owner.home_of(register);
        if rel == home.file {
            return Some(Ok(register));
        }
        if rel == home.readme {
            return Some(Err(home.dir));
        }
        if rel.starts_with(&home.dir) {
            return Some(Ok(register));
        }
    }
    None
}

/// The heading register homes one anchor carries, each with its entry level, as a finding
/// names them.
fn heading_homes(anchors: &Anchors, owner: &Anchor) -> String {
    let dirs: Vec<String> = owner
        .registers
        .iter()
        .filter_map(|n| anchors.registers().by_name(n))
        .filter(|r| r.shape == Shape::Heading)
        .map(|r| match r.level {
            Some(level) => format!("{} at level {level}, in `{}`", r.name, r.dir),
            None => format!("{}, in `{}`", r.name, r.dir),
        })
        .collect();
    if dirs.is_empty() {
        return "none".to_string();
    }
    dirs.join("; ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `design@core@checker-source-literals-are-data`.

    /// A project whose root component is `a-project` with one component under `parts/`.
    fn anchors() -> Anchors {
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        Anchors::declared(&Manifest::parse(Path::new("/nowhere"), text).expect("a declaration"))
    }

    fn table(docs: Vec<(&str, &str)>) -> Entities {
        table_under(docs, &anchors())
    }

    fn table_under(docs: Vec<(&str, &str)>, anchors: &Anchors) -> Entities {
        let model = Model::from_documents(
            docs.into_iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        Entities::build(&model, anchors)
    }

    /// The table of a tree of plan documents, its anchors read off the documents' own paths, as
    /// every caller reads the plan anchors off its tree.
    fn plan_table(docs: Vec<(&str, &str)>) -> (Entities, Vec<String>) {
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let paths: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let anchors = Anchors::of(&manifest, &paths);
        let e = table_under(docs, &anchors);
        let found = e
            .definition_findings()
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect();
        (e, found)
    }

    /// The claim: an item is defined by the level-two section in force above it, in the plan
    /// anchor that owns the document, and a milestone's README and its step specs share one
    /// namespace. Mutation checked: the kind read from the first level-two section of the
    /// document rather than the one in force.
    #[test]
    fn an_item_is_defined_by_the_section_in_force_in_its_plan() {
        let spec = "# A spec\n\n## Threads\n\n### A thread `##one`\n\n\
                    ## Arguments\n\n### An argument `##a1`\n";
        let readme = "# A milestone\n\n## Criteria\n\n### A criterion `##crit`\n";
        let step = "# A step\n\n## Acceptance criteria\n\n### It holds `##holds`\n";
        let (e, found) = plan_table(vec![
            ("docs/plans/specs/s.md", spec),
            ("docs/plans/milestones/m/README.md", readme),
            ("docs/plans/milestones/m/a-step.md", step),
        ]);
        assert!(found.is_empty(), "{found:#?}");
        let a = |kind: &str, anchor: &str, id: &str| {
            e.defined
                .contains_key(&(Kind::new(kind), anchor.to_string(), id.to_string()))
        };
        assert!(a("thread", "s", "one"));
        assert!(a("argument", "s", "a1"));
        assert!(!a("thread", "s", "a1"));
        assert!(a("criterion", "m", "crit"));
        assert!(a("acceptance", "m", "holds"));
        // The spec file is still an entry of the `spec` register of `plans`.
        assert!(a("spec", "plans", "s"));
    }

    /// The claims at the edges of an item section: a level-one heading closes it, a heading
    /// deeper than three owes nothing there, and a source file inside a plan is not read for
    /// items. Mutations checked: the section closed by a level-two heading alone; every level
    /// from three down owing a slug; every document of a plan read for items.
    #[test]
    fn an_item_section_ends_at_a_shallower_heading_and_owes_level_three_alone() {
        let spec = "# A spec\n\n## Threads\n\n### A thread `##one`\n\n#### A detail, no slug\n\n\
                    # Appendix\n\n### After the appendix `##stray`\n";
        let (e, found) = plan_table(vec![
            ("docs/plans/specs/s.md", spec),
            ("docs/plans/milestones/m/README.md", "# A milestone\n"),
            (
                "docs/plans/milestones/m/tool.rs",
                "//! ## Threads\n//!\n//! ### A comment heading `##in-source`\n",
            ),
        ]);
        assert!(e
            .defined
            .contains_key(&(Kind::new("thread"), "s".to_string(), "one".to_string())));
        assert!(!e.defined.contains_key(&(
            Kind::new("thread"),
            "s".to_string(),
            "stray".to_string()
        )));
        assert!(!e.defined.contains_key(&(
            Kind::new("thread"),
            "m".to_string(),
            "in-source".to_string()
        )));
        assert!(
            found
                .iter()
                .any(|f| f.contains("`##stray` is written outside")),
            "{found:#?}"
        );
        assert!(!found.iter().any(|f| f.contains("A detail")), "{found:#?}");
    }

    /// The claims found at the review: a group directory named like a spec is no spec, so a
    /// listing of files and directories gives the anchors a listing of files gives; the slugs of
    /// a plan that is no anchor name that cause; and a slug at another level inside an item
    /// section is reported for its level. Mutations checked: the prefix test dropped from
    /// `spec_files`; the cause-naming branch dropped; the level branch dropped.
    #[test]
    fn a_spec_anchor_is_a_file_and_an_unanchored_plan_says_so() {
        let files: Vec<PathBuf> = ["docs/plans/specs/x.md/z.md", "docs/plans/specs/y/x.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let mut with_dirs = files.clone();
        for d in ["docs/plans/specs/x.md", "docs/plans/specs/y"] {
            with_dirs.push(PathBuf::from(d));
        }
        assert_eq!(spec_files(&files), spec_files(&with_dirs));
        assert_eq!(
            spec_files(&with_dirs).get("x"),
            Some(&PathBuf::from("docs/plans/specs/y/x.md"))
        );
        let (_, found) = plan_table(vec![
            (
                "docs/plans/specs/Bad_Name.md",
                "# A spec\n\n## Threads\n\n### A thread `##one`\n",
            ),
            (
                "docs/plans/specs/s.md",
                "# A spec\n\n## Threads\n\n#### Too deep `##deep`\n",
            ),
        ]);
        assert!(
            found
                .iter()
                .any(|f| f.contains("`##one` is written in a plan that is no anchor")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f
                    .contains("`##deep` is written at a level-4 heading of the Threads section")),
            "{found:#?}"
        );
    }

    /// The claims on which spec files are anchors: a spec named like a milestone leaves the
    /// name to the milestone, and one named like a component is no anchor; a plan's name is no
    /// anchor word. Mutations checked: each name test dropped from `Anchors::of`; plan names
    /// counted as anchor words.
    #[test]
    fn a_spec_anchor_takes_no_name_another_anchor_holds() {
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = [\"parts/widget\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let tree: Vec<PathBuf> = [
            "docs/plans/specs/m.md",
            "docs/plans/specs/widget.md",
            "docs/plans/specs/s.md",
            "docs/plans/milestones/m/README.md",
        ]
        .iter()
        .map(PathBuf::from)
        .collect();
        let a = Anchors::of(&manifest, &tree);
        assert!(a.by_name("m").is_some_and(|m| m.is_milestone()));
        assert_eq!(a.all().iter().filter(|x| x.name == "m").count(), 1);
        assert!(a.by_name("widget").is_some_and(|w| w.is_component));
        assert_eq!(a.all().iter().filter(|x| x.name == "widget").count(), 1);
        assert!(a.by_name("s").is_some_and(|s| s.is_plan()));
        assert!(!a.is_anchor_word("s"));
        assert!(!a.is_anchor_word("m"));
    }

    /// The claim: only the four item sections owe a slug, and a slug anywhere else in a plan
    /// document defines nothing and is reported with a reason naming those sections. Mutations
    /// checked: a slug required at every level-three heading of a plan document; the generic
    /// misplaced reason for a slug of a plan document.
    #[test]
    fn a_plan_document_owes_slugs_in_its_item_sections_alone() {
        let spec = "# A spec\n\n## Decided design\n\n### A subsection, no slug\n\n\
                    ### A subsection with one `##stray`\n\n\
                    ## Threads\n\n### A thread with none\n";
        let (_, found) = plan_table(vec![("docs/plans/specs/s.md", spec)]);
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(found
            .iter()
            .any(|f| f.contains("`##stray` is written outside the item sections")));
        assert!(found
            .iter()
            .any(|f| f.contains("the level-3 heading \"A thread with none\" under Threads")));
    }

    fn findings(docs: Vec<(&str, &str)>) -> Vec<String> {
        table(docs)
            .definition_findings()
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    #[test]
    fn a_slug_in_each_register_home_defines_an_entity_of_that_kind() {
        // The three heading registers, file shape, each defining the same word: three
        // entities, because the register is part of the key.
        let e = table(vec![
            ("docs/design.md", "### A decision `##same`\n"),
            ("docs/goals.md", "## A goal `##same`\n"),
            ("docs/tripwires.md", "## Guarding it `##same`\n"),
        ]);
        assert_eq!(e.len(), 3);
        assert!(e.definition_findings().is_empty(), "{:#?}", e.findings);
        let a = anchors();
        for kind in ["design", "goal", "tripwire"] {
            assert_eq!(
                e.resolve(&a, &Kind::new(kind), "a-project", "same"),
                Resolution::Resolved,
                "{kind}"
            );
        }
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "a-project", "other"),
            Resolution::Undefined
        );
    }

    #[test]
    fn a_file_register_defines_one_entity_per_entry_file_named_by_its_basename() {
        // The two navigation files at the top level are not entries; a file in a group is.
        let e = table(vec![
            ("docs/open-issues/README.md", "# Open issues\n"),
            ("docs/open-issues/index.md", "# Index\n"),
            ("docs/open-issues/a-defect.md", "# A defect\n"),
            ("docs/open-issues/a-group/another-one.md", "# Another\n"),
        ]);
        assert_eq!(e.len(), 2, "{:#?}", e.defined);
        let a = anchors();
        for id in ["a-defect", "another-one"] {
            assert_eq!(
                e.resolve(&a, &Kind::new("issue"), "a-project", id),
                Resolution::Resolved,
                "{id}"
            );
        }
        assert!(e.definition_findings().is_empty(), "{:#?}", e.findings);
    }

    #[test]
    fn an_entry_whose_basename_no_reference_can_spell_defines_nothing_and_is_reported() {
        // Both navigation names inside a group, and a stem outside the id grammar. `index`
        // passes that grammar, so it is the one the pattern alone would let through.
        for at in [
            "docs/open-issues/a-group/README.md",
            "docs/open-issues/a-group/index.md",
            "docs/open-issues/Not_An_Id.md",
        ] {
            let found = findings(vec![(at, "# Not an entry name\n")]);
            assert_eq!(found.len(), 1, "{at}: {found:#?}");
            assert!(
                found[0].contains("cannot be an entry id"),
                "{at}: {found:#?}"
            );
            assert_eq!(
                table(vec![(at, "# Not an entry name\n")]).len(),
                0,
                "{at} must define nothing"
            );
        }
    }

    #[test]
    fn a_directory_home_defines_in_its_subdocuments_and_not_in_its_readme() {
        // Both shapes, for a register other than design, so the two-shape rule is shown to
        // generalise. The README is the head and defines nothing.
        let e = table(vec![
            ("docs/goals/one.md", "## A goal `##in-a-subdocument`\n"),
            ("docs/goals/README.md", "## A goal `##in-the-readme`\n"),
        ]);
        assert_eq!(e.len(), 1);
        let found = findings(vec![(
            "docs/goals/README.md",
            "## A goal `##in-the-readme`\n",
        )]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("the README of the `docs/goals` directory home"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_slug_outside_every_home_is_misplaced_and_defines_nothing() {
        // Mutation checked: with the `None` arm pushing no finding, the count is zero.
        let e = table(vec![("notes/a.md", "### A decision `##stray`\n")]);
        assert_eq!(e.len(), 0, "a misplaced slug defines nothing");
        let found = findings(vec![("notes/a.md", "### A decision `##stray`\n")]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("notes/a.md:1") && found[0].contains("no heading register home"),
            "{found:#?}"
        );
        // A file merely NAMED like a home, in a directory that is no anchor's docs/.
        let found = findings(vec![("notes/docs/design.md", "### A decision `##stray`\n")]);
        assert_eq!(found.len(), 1, "{found:#?}");
        // A slug inside a FILE register's instance is misplaced too: its entries are files.
        let found = findings(vec![(
            "docs/open-issues/an-entry.md",
            "### A decision `##stray`\n",
        )]);
        assert_eq!(found.len(), 1, "{found:#?}");
    }

    #[test]
    fn a_slug_at_the_wrong_heading_level_or_at_a_line_head_is_misplaced_even_in_a_home() {
        for (line, why) in [
            ("# A title `##deep`\n", "level-1"),
            // One level off the design register's level three, above and below.
            ("## A grouping `##deep`\n", "level-2"),
            ("#### A deep heading `##deep`\n", "level-4"),
            ("##### A deeper heading `##deep`\n", "level-5"),
            ("`##deep` — **The statement.**\n", "head of a plain line"),
            ("as `##deep` records\n", "middle of a line"),
            // A table cell defines nothing, in any column.
            ("| A decision in a row | `##deep` |\n", "table cell"),
            ("| `##deep` | A decision in a row |\n", "table cell"),
        ] {
            let e = table(vec![("docs/design.md", line)]);
            assert_eq!(e.len(), 0, "{line:?} must define nothing");
            let found = findings(vec![("docs/design.md", line)]);
            assert_eq!(found.len(), 1, "{line:?}: {found:#?}");
            assert!(found[0].contains(why), "{line:?}: {found:#?}");
        }
    }

    #[test]
    fn a_slug_is_judged_against_the_level_of_the_register_whose_home_holds_it() {
        // The same level-three heading defines in the design home and is misplaced in the
        // goals home, whose level is two; the reverse holds for a level-two heading.
        let e = table(vec![
            ("docs/design.md", "### A decision `##at-three`\n"),
            ("docs/goals.md", "## A goal `##at-two`\n"),
        ]);
        assert_eq!(e.len(), 2);
        assert!(e.definition_findings().is_empty(), "{:#?}", e.findings);
        let e = table(vec![("docs/goals.md", "### A goal `##at-three`\n")]);
        assert_eq!(e.len(), 0);
        let found = e.definition_findings();
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].what.contains("level-3"), "{found:#?}");
        // The action names each home with the level its entries sit at.
        assert!(found[0].action.contains("goal at level 2"), "{found:#?}");
    }

    #[test]
    fn a_heading_at_the_register_level_with_no_slug_is_a_finding_and_no_other_is() {
        // The reproduction of the issue this closes: a tripwire written without a slug.
        let found = findings(vec![("docs/tripwires.md", "## Guarding it\n")]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("docs/tripwires.md:1")
                && found[0].contains("\"Guarding it\"")
                && found[0].contains("level-2")
                && found[0].contains("carries no slug"),
            "{found:#?}"
        );
        assert_eq!(
            table(vec![("docs/tripwires.md", "## Guarding it\n")]).len(),
            0
        );
        // In the design home, level three owes a slug, and the title, a level-two grouping
        // and a level-four heading owe none.
        let found = findings(vec![(
            "docs/design.md",
            "# Decisions\n\n## A grouping\n\n### Lost its slug\n\n#### Detail\n",
        )]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].starts_with("docs/design.md:5"), "{found:#?}");
    }

    #[test]
    fn an_indented_heading_is_judged_like_any_other() {
        // Markdown reads a heading indented by up to three spaces as a heading, so the
        // invariant holds of it: unslugged, it is reported.
        let found = findings(vec![("docs/design.md", "   ### Indented and unslugged\n")]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("carries no slug"), "{found:#?}");
    }

    #[test]
    fn a_reference_on_an_unslugged_heading_is_not_its_slug() {
        // The shape a tripwire takes: its heading names what it guards. A reference span on
        // the line is no slug, so the heading is still unslugged.
        let found = findings(vec![(
            "docs/tripwires.md",
            "## Guarding `design@a-project@a-decision`\n",
        )]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("carries no slug"), "{found:#?}");
    }

    #[test]
    fn a_rust_file_under_a_directory_home_is_no_register_document() {
        // A home holds markdown. A doc comment's heading owes no slug, and a slug in one is
        // misplaced, whatever directory the file sits in.
        let e = table(vec![(
            "docs/design/example.rs",
            "//! ### A heading in a doc comment\n//! ### Another `##from-rust`\nfn f() {}\n",
        )]);
        assert_eq!(e.len(), 0, "{:#?}", e.defined);
        let found: Vec<String> = e
            .definition_findings()
            .iter()
            .map(|f| f.what.clone())
            .collect();
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no heading register home"), "{found:#?}");
    }

    #[test]
    fn the_findings_of_one_document_come_in_line_order() {
        // The unslugged-heading findings and the misplaced ones are produced by two passes,
        // and a reader walks the file top to bottom.
        let found = findings(vec![(
            "docs/design.md",
            "#### Deep `##a-deep-one`\n\n### No slug\n\n#### Deep `##another`\n",
        )]);
        let lines: Vec<&str> = found
            .iter()
            .map(|f| f.split("  ").next().unwrap())
            .collect();
        assert_eq!(
            lines,
            vec!["docs/design.md:1", "docs/design.md:3", "docs/design.md:5"],
            "{found:#?}"
        );
    }

    #[test]
    fn the_readme_of_a_directory_home_owes_no_slug() {
        // It is the head of the home: an introduction, not an entry.
        let found = findings(vec![
            ("docs/design/README.md", "# Design\n\n### How to read it\n"),
            (
                "docs/tripwires/README.md",
                "# Tripwires\n\n## How to read them\n",
            ),
        ]);
        assert!(found.is_empty(), "{found:#?}");
        // A file that is no home owes nothing either.
        assert!(findings(vec![("notes/a.md", "### A heading\n")]).is_empty());
    }

    #[test]
    fn a_declared_register_is_judged_at_the_level_it_declares() {
        // Level five, which no built-in register uses, so the level is read from the
        // declaration and not compiled in, and a heading that deep is seen at all.
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [registers.note]\nscope = \"component\"\nshape = \"heading\"\ndir = \"notes\"\n\
             level = 5\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        assert!(m.complaints().is_empty(), "{:#?}", m.complaints());
        let a = Anchors::declared(&m);
        let e = table_under(
            vec![(
                "docs/notes.md",
                "# Notes\n\n##### A note `##a-note`\n\n### Too shallow `##shallow`\n\n\
                 ##### No slug\n",
            )],
            &a,
        );
        assert_eq!(
            e.resolve(&a, &Kind::new("note"), "a-project", "a-note"),
            Resolution::Resolved
        );
        let found: Vec<String> = e
            .definition_findings()
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect();
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found
                .iter()
                .any(|f| f.starts_with("docs/notes.md:5") && f.contains("level-3")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.starts_with("docs/notes.md:7") && f.contains("carries no slug")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_register_declared_at_level_six_sees_its_headings() {
        // The deepest level markdown has: a heading there is observed, so an unslugged one
        // is reported and a slugged one defines.
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [registers.note]\nscope = \"component\"\nshape = \"heading\"\ndir = \"notes\"\n\
             level = 6\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::declared(&m);
        let e = table_under(
            vec![(
                "docs/notes.md",
                "###### A note `##a-note`\n\n###### No slug\n",
            )],
            &a,
        );
        assert_eq!(e.len(), 1);
        assert_eq!(e.definition_findings().len(), 1, "{:#?}", e.findings);
        assert!(e.definition_findings()[0].what.contains("carries no slug"));
    }

    #[test]
    fn a_definition_belongs_to_the_deepest_anchor_holding_its_document() {
        let e = table(vec![
            ("docs/design.md", "### A decision `##word`\n"),
            ("parts/a-part/docs/design.md", "### A decision `##word`\n"),
        ]);
        assert_eq!(e.len(), 2, "two anchors, two entities");
        assert!(e.definition_findings().is_empty());
        let a = anchors();
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "a-part", "word"),
            Resolution::Resolved
        );
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "a-project", "word"),
            Resolution::Resolved
        );
    }

    #[test]
    fn two_definitions_in_one_instance_are_a_finding_at_each_site() {
        // Mutation checked: iterating `sites.iter().take(1)` leaves one finding, and the
        // assertion on two fails.
        let found = findings(vec![
            ("docs/design.md", "### One `##twice`\n\n### Two `##twice`\n"),
            ("docs/goals.md", "## A goal `##twice`\n"),
        ]);
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found[0].starts_with("docs/design.md:1") && found[0].contains("docs/design.md:3"),
            "{found:#?}"
        );
        assert!(
            found[1].starts_with("docs/design.md:3") && found[1].contains("docs/design.md:1"),
            "{found:#?}"
        );
        assert!(
            found.iter().all(|f| f.contains("`design@a-project@twice`")),
            "the finding names the entity in the grammar: {found:#?}"
        );
    }

    #[test]
    fn a_misplaced_definition_does_not_count_toward_a_duplicate() {
        // It defines nothing, so it cannot collide with the one that does.
        let found = findings(vec![
            ("docs/design.md", "### One `##once`\n"),
            ("notes/a.md", "### One `##once`\n"),
        ]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("defines nothing"), "{found:#?}");
    }

    #[test]
    fn the_candidate_rule_admits_a_kind_or_an_anchor_at_the_head_and_nothing_else() {
        let a = anchors();
        assert_eq!(
            candidate("design@a-project@x-y", &a),
            Candidate::Reference {
                kind: Kind::new("design"),
                anchor: "a-project",
                id: "x-y"
            }
        );
        // The old forms: a component, the generic and the escape anchor in kind position.
        for span in ["a-project@docs/x.md", "*@docs/x.md", "elsewhere@x"] {
            assert!(
                matches!(candidate(span, &a), Candidate::AnchorInKindPosition { .. }),
                "{span}"
            );
        }
        // An email, a remote, a placeholder-free typo in the kind, an annotation: silent.
        for span in [
            "user@example.test",
            "git@host:x/y.git",
            "desing@a-project@x",
            "@Test",
        ] {
            assert_eq!(candidate(span, &a), Candidate::NotOne, "{span}");
        }
        // An empty head in front of a path shape is a pointer that would otherwise leave
        // every check: the retired `@` escape, or a kind never typed.
        assert!(
            matches!(candidate("@docs/x.md", &a), Candidate::Malformed { .. }),
            "{:?}",
            candidate("@docs/x.md", &a)
        );
    }

    #[test]
    fn a_declared_register_becomes_a_kind_the_candidate_rule_admits() {
        // Nothing about a kind is compiled in beyond `path`, `planned` and the built-in registers:
        // a project that declares a register makes its name spellable in kind position.
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"reading\", \"tripwire\"]\n\n\
             [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::declared(&m);
        assert!(a.kinds_listed().contains("reading"));
        assert_eq!(
            candidate("reading@notes@a-reading", &a),
            Candidate::Reference {
                kind: Kind::new("reading"),
                anchor: "notes",
                id: "a-reading"
            }
        );
        // Its home is under the location's own path, not under a `docs/` it does not have.
        let e = table_under(vec![("notes/readings/a-reading.md", "# A reading\n")], &a);
        assert_eq!(
            e.resolve(&a, &Kind::new("reading"), "notes", "a-reading"),
            Resolution::Resolved
        );
        // The location carries those registers and no others, so a built-in one it does not
        // carry is refused there with the carriers named.
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "notes", "anything"),
            Resolution::AnchorLacksRegister {
                carriers: vec!["a-project".to_string()]
            }
        );
    }

    #[test]
    fn a_location_inside_a_component_owns_the_documents_under_it() {
        // The location sits under the root component, so the deepest anchor is the location
        // and its heading home is its own. Read as the component's, the tripwire document
        // under the location is no register home of the root, and every entry in it is
        // reported as misplaced.
        // Mutation checked: filtering `owning` to components alone leaves this file with no
        // home and the entity count at zero.
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"tripwire\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::declared(&m);
        let e = table_under(
            vec![("notes/tripwires.md", "## Guarding it `##a-tripwire`\n")],
            &a,
        );
        assert!(
            e.definition_findings().is_empty(),
            "{:#?}",
            e.definition_findings()
        );
        assert_eq!(
            e.resolve(&a, &Kind::new("tripwire"), "notes", "a-tripwire"),
            Resolution::Resolved
        );
        // And not the root's: the two anchors are separate instances of one register.
        assert_eq!(
            e.resolve(&a, &Kind::new("tripwire"), "a-project", "a-tripwire"),
            Resolution::Undefined
        );
    }

    #[test]
    fn segmentation_takes_three_segments_except_that_a_path_may_hold_an_at_sign() {
        // Mutation checked: dropping the `!kind.is_path()` guard makes the path case
        // malformed and the first assertion fails.
        let a = anchors();
        assert_eq!(
            candidate("path@a-project@notes/a@b.md", &a),
            Candidate::Reference {
                kind: Kind::path(),
                anchor: "a-project",
                id: "notes/a@b.md"
            }
        );
        for span in [
            "design@a-project",
            "design@a-project@x@y",
            "design@@x",
            "design@a-project@",
            "path@a-project",
            "path@@x",
            "path@a-project@",
        ] {
            assert!(
                matches!(candidate(span, &a), Candidate::Malformed { .. }),
                "{span}"
            );
        }
    }

    #[test]
    fn each_way_a_reference_resolves_to_nothing_is_told_apart() {
        // The fourth arm needs an anchor that exists and lacks the register, which no
        // component has: the anchor is stated directly, the shape a location takes.
        let base = anchors();
        let root = base.by_name("a-project").expect("the root").clone();
        let bare = Anchor::location("bare", Path::new("bare"), vec!["tripwire".to_string()]);
        let a = Anchors::from_list(vec![root, bare], base.registers().clone());
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "### A decision `##word`\n".to_string(),
        )]);
        let e = Entities::build(&model, &a);
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "nowhere", "word"),
            Resolution::UnknownAnchor
        );
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "bare", "word"),
            Resolution::AnchorLacksRegister {
                carriers: vec!["a-project".to_string()]
            }
        );
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "a-project", "other"),
            Resolution::Undefined
        );
        assert_eq!(
            e.resolve(&a, &Kind::new("design"), "a-project", "word"),
            Resolution::Resolved
        );
        // Every anchor but a milestone carries `path`, whatever its register list.
        assert!(a.by_name("bare").unwrap().carries(&Kind::path()));
    }

    #[test]
    fn the_required_document_set_carries_each_shape_with_its_kind() {
        // The compiled-in files, each heading register's file, directory and README, and the
        // issue register's directory, README and index.
        let a = anchors();
        assert_eq!(a.required_kind("README.md"), Some(false));
        assert_eq!(a.required_kind("docs/tripwires.md"), Some(false));
        assert_eq!(a.required_kind("docs/goals.md"), Some(false));
        assert_eq!(a.required_kind("docs/goals"), Some(true));
        assert_eq!(a.required_kind("docs/goals/README.md"), Some(false));
        assert_eq!(a.required_kind("docs/design"), Some(true));
        assert_eq!(a.required_kind("docs/open-issues"), Some(true));
        assert_eq!(a.required_kind("docs/open-issues/README.md"), Some(false));
        assert_eq!(a.required_kind("docs/open-issues/index.md"), Some(false));
        // The README of a heading register's directory home, which is the shape a component
        // that has not split its home yet does not carry.
        assert_eq!(a.required_kind("docs/tripwires/README.md"), Some(false));
        assert_eq!(a.required_kind("docs/design/README.md"), Some(false));
        // A file register has no single-file shape, so that name is not a required document.
        assert_eq!(a.required_kind("docs/open-issues.md"), None);
        assert_eq!(a.required_kind("docs/design/one.md"), None);
        assert_eq!(a.required_kind("src/lib.rs"), None);
    }

    #[test]
    fn the_deepest_anchor_wins_and_a_component_wins_a_tie() {
        // Deepest, not shallowest: the nested component owns its own documents. The tie is
        // a declaration `manifest::collides` refuses, and until it is repaired the component
        // keeps its documents rather than every slug in them reading as misplaced.
        let a = anchors();
        assert_eq!(
            a.owning(Path::new("parts/a-part/docs/design.md")).name,
            "a-part"
        );
        assert_eq!(a.owning(Path::new("docs/design.md")).name, "a-project");
        let base = anchors();
        let root = base.by_name("a-project").expect("the root").clone();
        let shadow = Anchor::location("shadow", Path::new(""), vec!["issue".to_string()]);
        let tied = Anchors::from_list(vec![root, shadow], base.registers().clone());
        assert_eq!(tied.owning(Path::new("docs/design.md")).name, "a-project");
    }

    #[test]
    fn a_register_whose_name_merely_ends_in_the_path_word_is_not_the_path_kind() {
        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [registers.subpath]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"subpaths\"\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::declared(&m);
        assert_eq!(a.kind("subpath"), Some(Kind::new("subpath")));
        assert!(!a.kind("subpath").expect("the kind").is_path());
        assert!(a.kind("path").expect("the path kind").is_path());
    }

    #[test]
    fn an_anchor_name_is_spellable_exactly_when_the_pattern_accepts_it() {
        for (name, ok) in [
            ("a-tool", true),
            ("an.engine", true),
            (".claude", true),
            ("an_engine", true),
            ("9lives", true),
            ("-leading-dash", false),
            ("has/slash", false),
            ("has space", false),
            ("", false),
        ] {
            assert_eq!(is_anchor_name(name), ok, "{name:?}");
        }
    }
}
