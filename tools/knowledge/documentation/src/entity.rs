//! The entity table: everything a document can cite, and the one resolver every check reads.
//!
//! An entity has a **kind**, an **anchor**, an **id** and a **definition site**. The table is
//! built once from the model, and a reference `` `<kind>@<anchor>@<id>` `` is resolved against
//! it. Before this table five checks held five notions of a name; the argument is
//! `design@knowledge@one-entity-table`.
//!
//! **A kind is a register's name, or `path`.** Four registers are compiled in and a project
//! declares the rest, so the kind set is data rather than an enum — `path@knowledge@documentation/src/manifest.rs`
//! owns what a register is, and this module owns what naming one means.
//!
//! **An anchor is a named directory that carries registers.** A component carries every
//! component-scoped register with its homes under `docs/`; a location carries the subset it
//! declares, with its homes directly under its own path. Both are the same shape, which is why
//! `Anchor` holds its register list and its home base as data.
//!
//! **The `path` kind is resolved against the tree, not the table.** Its ids are paths, its
//! anchors are the same anchors plus two reserved words, and the check that resolves it needs
//! the survey. What this module gives it is the candidate rule, the segmentation and the anchor
//! lookup, so one grammar has one reader.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use regex::Regex;

use crate::finding::Finding;
use crate::manifest::{Manifest, Register, Registers, Shape, COMPONENT_DOCUMENTS};
use crate::model::Model;
use crate::scan::{Observation, SlugSite};

/// The one kind that is not a register: a file or directory, defined by the tree itself.
pub const PATH_KIND: &str = "path";

/// What a reference names, in its first segment: a register's name, or `path`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kind(Arc<str>);

impl Kind {
    pub fn new(name: &str) -> Kind {
        Kind(Arc::from(name))
    }

    /// The kind of a file or directory.
    pub fn path() -> Kind {
        Kind::new(PATH_KIND)
    }

    pub fn is_path(&self) -> bool {
        &*self.0 == PATH_KIND
    }

    /// The word a reference spells.
    pub fn name(&self) -> &str {
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
/// A declared anchor may not take this name, which `check::registers` asserts.
pub const ESCAPE_ANCHOR: &str = "elsewhere";

/// The reserved anchor for every component's own copy of a path.
pub const EVERY_ANCHOR: &str = "*";

/// The shape an anchor name must have for a reference to be able to name it.
///
/// A name a reference cannot spell is one every pointer at it misses silently, so the check
/// over the declaration asks this rather than assuming it.
static ANCHOR_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\.?[A-Za-z0-9][A-Za-z0-9._-]*$").unwrap());

/// The shape an entity's id must have, for every kind but `path`.
static ENTITY_ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

/// Can a reference name an anchor called this.
pub fn is_anchor_name(name: &str) -> bool {
    ANCHOR_NAME.is_match(name)
}

/// Can an entity be given this id.
pub fn is_entity_id(id: &str) -> bool {
    ENTITY_ID.is_match(id)
}

/// Where one register's entries live, for one anchor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Home {
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
pub struct Anchor {
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
}

impl Anchor {
    /// A component: every component-scoped register, homes under `docs/`.
    pub fn component(name: &str, path: &Path, registers: &Registers) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            home_base: path.join("docs"),
            registers: registers
                .component_scoped()
                .map(|r| r.name.clone())
                .collect(),
            is_component: true,
        }
    }

    /// A location: the registers it declares, homes directly under its own path.
    pub fn location(name: &str, path: &Path, registers: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            home_base: path.to_path_buf(),
            registers,
            is_component: false,
        }
    }

    /// Whether this is the component at the project root.
    pub fn is_root(&self) -> bool {
        self.path.as_os_str().is_empty()
    }

    /// Whether a reference of this kind may anchor here.
    pub fn carries(&self, kind: &Kind) -> bool {
        kind.is_path() || self.registers.iter().any(|r| r == kind.name())
    }

    /// The home of one register here, given that register's declaration.
    pub fn home_of(&self, register: &Register) -> Home {
        let dir = self.home_base.join(&register.dir);
        Home {
            file: self.home_base.join(format!("{}.md", register.dir)),
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
pub struct Anchors {
    list: Vec<Anchor>,
    registers: Registers,
}

impl Anchors {
    /// The anchors a manifest declares: its components, then its locations.
    pub fn of(manifest: &Manifest) -> Self {
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
        Self { list, registers }
    }

    /// Anchors stated directly, for a test that needs a register list no component has.
    pub fn from_list(list: Vec<Anchor>, registers: Registers) -> Self {
        Self { list, registers }
    }

    pub fn all(&self) -> &[Anchor] {
        &self.list
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// The kind a word names: a declared register, or `path`.
    pub fn kind(&self, word: &str) -> Option<Kind> {
        if word == PATH_KIND {
            return Some(Kind::path());
        }
        self.registers.by_name(word).map(|r| Kind::new(&r.name))
    }

    /// Every kind name, comma-separated, as a finding lists them.
    pub fn kinds_listed(&self) -> String {
        let mut names: Vec<&str> = self
            .registers
            .all()
            .iter()
            .map(|r| r.name.as_str())
            .collect();
        names.push(PATH_KIND);
        names.join(", ")
    }

    /// The anchor a document belongs to: the deepest whose path holds it, and the root where
    /// none does. The root's path is empty and is a prefix of every path, which is what makes
    /// it the fallback rather than a case.
    ///
    /// **Locations count here, and that is the point.** A location sits inside a component —
    /// both of this repository's do — and its register homes are its own, so a document under
    /// it belongs to it and not to the component above. A component inside a location would
    /// make the two ambiguous, and `check::registers` refuses one.
    pub fn owning(&self, rel: &Path) -> &Anchor {
        // Depth first, and a component on a tie: two anchors at one path is a declaration
        // `check::registers` reports, and until it is repaired the component keeps its own
        // documents rather than every slug in them being reported as misplaced.
        self.list
            .iter()
            .filter(|a| rel.starts_with(&a.path))
            .max_by_key(|a| (a.path.components().count(), a.is_component))
            .expect("the anchor at the root is a prefix of every path")
    }

    /// The anchor a reference names, or `None` where nothing declares that name.
    pub fn by_name(&self, name: &str) -> Option<&Anchor> {
        self.list.iter().find(|a| a.name == name)
    }

    /// Every anchor name, comma-separated, as a finding lists them.
    pub fn listed(&self) -> String {
        self.list
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Whether `word` could stand in the anchor position of some reference: a declared
    /// anchor, or one of the two words reserved for `path`.
    pub fn is_anchor_word(&self, word: &str) -> bool {
        word == EVERY_ANCHOR || word == ESCAPE_ANCHOR || self.by_name(word).is_some()
    }

    /// The register home an anchor keeps one register in, or `None` where it carries none.
    pub fn home(&self, anchor: &Anchor, kind: &Kind) -> Option<Home> {
        if !anchor.carries(kind) {
            return None;
        }
        self.registers
            .by_name(kind.name())
            .map(|r| anchor.home_of(r))
    }

    /// Every register instance of the project: an anchor, the register, and its home.
    pub fn instances(&self) -> Vec<(&Anchor, &Register, Home)> {
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
    pub fn required_kind(&self, path: &str) -> Option<bool> {
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

/// Where something was written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
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
pub enum Candidate<'a> {
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
/// silent for the same reason, which `path@knowledge@docs/tripwires.md` guards.
///
/// **Segmentation.** Every kind but `path` takes exactly three segments; `path` takes an anchor
/// and then everything after the second `@` as its id, so a path may hold an `@`. An empty
/// segment is malformed in either shape.
pub fn candidate<'a>(span: &'a str, anchors: &Anchors) -> Candidate<'a> {
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
    if !kind.is_path() && id.contains('@') {
        return Candidate::Malformed {
            why: "four or more segments; a reference has three",
        };
    }
    Candidate::Reference { kind, anchor, id }
}

/// Why a reference to a table kind resolves to nothing, or that it resolves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    Resolved,
    /// Nothing declares that anchor name.
    UnknownAnchor,
    /// The anchor exists and carries no register of that kind; the anchors that do.
    AnchorLacksRegister {
        carriers: Vec<String>,
    },
    /// The anchor carries the register and defines no such id.
    Undefined,
}

/// The table: every entity defined in the walk, keyed by kind, anchor and id.
#[derive(Debug, Default)]
pub struct Entities {
    defined: BTreeMap<(Kind, String, String), Vec<Site>>,
    /// Misplaced, malformed and duplicate definitions, found while building.
    findings: Vec<Finding>,
}

impl Entities {
    /// Read every definition out of the model and file it under its anchor and register.
    ///
    /// A heading register's entity is a slug in one of that register's home shapes, owned by
    /// the deepest anchor whose path holds the document. A file register's entity is a file
    /// under the instance directory, its id the basename. A slug outside every heading home,
    /// or at a heading level the grammar does not accept, or at the head of a plain line,
    /// defines nothing and is reported as misplaced.
    pub fn build(model: &Model, anchors: &Anchors) -> Self {
        let mut out = Self::default();
        out.heading_definitions(model, anchors);
        out.file_definitions(model, anchors);
        out.report_duplicates();
        out
    }

    fn heading_definitions(&mut self, model: &Model, anchors: &Anchors) {
        for doc in model.documents() {
            let owner = anchors.owning(&doc.rel);
            for l in &doc.observations {
                let Observation::SlugDef { id, site } = &l.what else {
                    continue;
                };
                let at = Site {
                    file: doc.rel.clone(),
                    line: l.line,
                };
                let misplaced = |why: String| {
                    Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`##{id}` is written at {why} and defines nothing"),
                        format!(
                            "a slug is defined at the end of a level-two or level-three \
                             heading, or in a table cell, inside a heading register's home; \
                             this anchor's are {}; move it there, or delete it",
                            heading_dirs(anchors, owner)
                        ),
                    )
                };
                match site {
                    SlugSite::Heading(level) if !(2..=3).contains(level) => {
                        self.findings
                            .push(misplaced(format!("a level-{level} heading")));
                        continue;
                    }
                    SlugSite::LineHead => {
                        self.findings
                            .push(misplaced("the head of a plain line".to_string()));
                        continue;
                    }
                    SlugSite::Inline => {
                        self.findings
                            .push(misplaced("the middle of a line".to_string()));
                        continue;
                    }
                    _ => {}
                }
                match register_of(anchors, owner, &doc.rel) {
                    Some(Ok(kind)) => self
                        .defined
                        .entry((kind, owner.name.clone(), id.clone()))
                        .or_default()
                        .push(at),
                    Some(Err(dir)) => self.findings.push(misplaced(format!(
                        "the README of the `{}` directory home",
                        dir.display()
                    ))),
                    None => self.findings.push(misplaced(format!(
                        "`{}`, which is no heading register home of `{}`",
                        doc.rel.display(),
                        owner.name
                    ))),
                }
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
            if register.shape != Shape::File {
                continue;
            }
            for doc in model.documents() {
                let Some(id) = entry_id(&doc.rel, &home.dir) else {
                    continue;
                };
                // A document an anchor nested inside this home owns is that anchor's, not
                // an entry here: the nesting is `check::registers`' finding, and reading the
                // nested anchor's files as entries would report it against the wrong register.
                if anchors.owning(&doc.rel).path != anchor.path {
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

    /// Resolve a reference to a table kind. `path` is not this table's to resolve.
    pub fn resolve(&self, anchors: &Anchors, kind: &Kind, anchor: &str, id: &str) -> Resolution {
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
    pub fn of_kind(&self, kind: &Kind) -> Vec<(&str, &str, &[Site])> {
        self.defined
            .iter()
            .filter(|((k, _, _), _)| k == kind)
            .map(|((_, anchor, id), sites)| (anchor.as_str(), id.as_str(), sites.as_slice()))
            .collect()
    }

    /// Where one entity is defined, or `None` where nothing defines it.
    pub fn sites(&self, kind: &Kind, anchor: &str, id: &str) -> Option<&[Site]> {
        self.defined
            .get(&(kind.clone(), anchor.to_string(), id.to_string()))
            .map(Vec::as_slice)
    }

    /// How many distinct entities the table holds.
    pub fn len(&self) -> usize {
        self.defined.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defined.is_empty()
    }

    /// The misplaced, malformed and duplicate definitions found while building the table.
    pub fn definition_findings(&self) -> &[Finding] {
        &self.findings
    }
}

/// The id of a file-register entry at `rel`, for an instance rooted at `dir`, or `None`.
///
/// `None` for anything that is not an entry: a file outside the instance, a file of another
/// suffix, and the two navigation files at the instance's own top level.
pub fn entry_id(rel: &Path, dir: &Path) -> Option<String> {
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

/// Which heading register home of `owner` holds `rel`: `Ok(kind)` for the file home or a
/// subdocument of the directory home, `Err(dir)` for the directory home's README, `None`
/// for a file that is no heading home.
fn register_of(anchors: &Anchors, owner: &Anchor, rel: &Path) -> Option<Result<Kind, PathBuf>> {
    for name in &owner.registers {
        let register = anchors.registers().by_name(name)?;
        if register.shape != Shape::Heading {
            continue;
        }
        let home = owner.home_of(register);
        if rel == home.file {
            return Some(Ok(Kind::new(name)));
        }
        if rel == home.readme {
            return Some(Err(home.dir));
        }
        if rel.starts_with(&home.dir) {
            return Some(Ok(Kind::new(name)));
        }
    }
    None
}

/// The heading register directories one anchor carries, as a finding names them.
fn heading_dirs(anchors: &Anchors, owner: &Anchor) -> String {
    let dirs: Vec<&str> = owner
        .registers
        .iter()
        .filter_map(|n| anchors.registers().by_name(n))
        .filter(|r| r.shape == Shape::Heading)
        .map(|r| r.dir.as_str())
        .collect();
    if dirs.is_empty() {
        return "none".to_string();
    }
    dirs.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `design@knowledge@checker-source-literals-are-data`.

    /// A project whose root component is `a-project` with one component under `parts/`.
    fn anchors() -> Anchors {
        let text = "[project]\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        Anchors::of(&Manifest::parse(Path::new("/nowhere"), text).expect("a declaration"))
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
            ("#### A deep heading `##deep`\n", "level-4"),
            ("##### A deeper heading `##deep`\n", "level-5"),
            ("`##deep` — **The statement.**\n", "head of a plain line"),
            ("as `##deep` records\n", "middle of a line"),
        ] {
            let e = table(vec![("docs/design.md", line)]);
            assert_eq!(e.len(), 0, "{line:?} must define nothing");
            let found = findings(vec![("docs/design.md", line)]);
            assert_eq!(found.len(), 1, "{line:?}: {found:#?}");
            assert!(found[0].contains(why), "{line:?}: {found:#?}");
        }
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
        // Nothing about a kind is compiled in beyond `path` and the built-in four: a project
        // that declares a register makes its name spellable in kind position.
        let text = "[project]\nname = \"a-project\"\ncomponents = []\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"reading\", \"tripwire\"]\n\n\
             [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::of(&m);
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
        let text = "[project]\nname = \"a-project\"\ncomponents = []\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"tripwire\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::of(&m);
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
        // Every anchor carries `path`, whatever its register list.
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
        // a declaration `check::registers` reports, and until it is repaired the component
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
        let text = "[project]\nname = \"a-project\"\ncomponents = []\n\n\
             [registers.subpath]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"subpaths\"\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        let m = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let a = Anchors::of(&m);
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
