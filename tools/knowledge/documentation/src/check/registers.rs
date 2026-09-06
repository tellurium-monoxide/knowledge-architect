//! Every anchor carries the registers it owes, in the shape that register has.
//!
//! An anchor is a component or a location. A **component** carries its README, its scoped
//! `CLAUDE.md`, its rejected-alternatives document, and every component-scoped register; a
//! **location** carries the registers it declares and nothing else. What that buys is one home
//! per anchor for each kind of statement, and one place to look for it.
//!
//! **A missing home is a finding rather than an absence.** A component with no design home has
//! its design recorded wherever the last session happened to put it, and a component with no
//! issue directory has what is open about it in no report.
//!
//! **Two register shapes, and this module asserts both.** A heading register has the two homes
//! of `design@knowledge@heading-register-two-shapes`: the single file, or the directory whose `README.md`
//! links every subdocument. A file register has one home, a directory holding one file per
//! entry beside a hand-written `README.md`, a generated `index.md` and an optional
//! `register.toml`; the entries' frontmatter, title, sections and owed subsections are asserted
//! here, per `design@knowledge@a-file-register-is-a-directory-of-entries`.
//!
//! **The definition-site findings belong here.** Which slug defines what is the entity table's
//! question; whether a definition sits somewhere a definition may sit is a question about a
//! register's shape, so the table's misplaced, malformed and duplicate findings are reported by
//! this family.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::entity::{Anchor, Anchors, Entities, Home, ESCAPE_ANCHOR, EVERY_ANCHOR};
use crate::finding::Finding;
use crate::manifest::{Manifest, Register, Scope, Shape, COMPONENT_DOCUMENTS, MANIFEST_NAME};
use crate::model::{Document, Model};
use crate::scan::Observation;

use super::Inputs;

/// What the check looked at.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub components: usize,
    pub locations: usize,
    /// Anchor-and-register pairs whose home was asserted.
    pub instances: usize,
    /// File-register entries whose shape was asserted.
    pub entries: usize,
}

/// The per-instance options, the only file a register instance carries beside its entries.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegisterConfig {
    groups: Vec<String>,
}

pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
    let anchors = Anchors::of(manifest);
    check_under(model, manifest, inputs, &anchors)
}

/// The same, over a stated anchor list, for a test that needs anchors no manifest declares.
pub fn check_under(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    anchors: &Anchors,
) -> (Vec<Finding>, Counts) {
    let mut out: Vec<Finding> = Vec::new();

    // What the manifest itself got wrong, first: a register nobody can name and an anchor
    // nothing can point at break every pointer at them, which a reader would otherwise meet
    // as a dangling reference somewhere else entirely.
    for complaint in manifest.register_complaints() {
        out.push(Finding::in_file(
            MANIFEST_NAME,
            complaint.clone(),
            "a declared register states its scope and its shape; a built-in register's \
             storage is what the word component means and is not declared",
        ));
    }
    declarations(&mut out, manifest, anchors, inputs);

    let mut counts = Counts {
        components: anchors.all().iter().filter(|a| a.is_component).count(),
        locations: manifest.locations().len(),
        ..Counts::default()
    };

    // The entity table's own findings: a definition where none may sit, an entry id nothing
    // can spell, and one id defined twice in one instance.
    out.extend(
        Entities::build(model, anchors)
            .definition_findings()
            .to_vec(),
    );

    for anchor in anchors.all() {
        // An anchor directory that is not there is ONE finding. One per document it does not
        // carry would bury the single fact that explains all of them.
        if !anchor.is_root() && !inputs.present.contains(&anchor.path) {
            out.push(Finding::in_file(
                &anchor.path,
                format!(
                    "`{}` is declared an anchor and does not exist",
                    anchor.path.display()
                ),
                "create it, or stop declaring it in knowledge.toml",
            ));
            continue;
        }
        if anchor.is_component {
            for name in COMPONENT_DOCUMENTS {
                let path = anchor.path.join(name);
                if !inputs.present.contains(&path) {
                    out.push(Finding::in_file(
                        &path,
                        format!("the component `{}` carries no {name}", anchor.name),
                        "create it, or stop listing the component in knowledge.toml [project]; \
                         every component carries the same documents",
                    ));
                } else if inputs.directories.contains(&path) {
                    // A directory wearing the document's name satisfies a presence test and is
                    // read by nothing: the walk never reads a directory as a document, so every
                    // claim that should live in it is outside every check.
                    out.push(Finding::in_file(
                        &path,
                        format!(
                            "`{}` is a directory wearing the document's name",
                            path.display()
                        ),
                        "the component documents are files; move the directory aside and \
                         create the file",
                    ));
                }
            }
        }
        let mut homes: BTreeMap<PathBuf, &str> = BTreeMap::new();
        for name in &anchor.registers {
            let Some(register) = anchors.registers().by_name(name) else {
                continue;
            };
            counts.instances += 1;
            let home = anchor.home_of(register);
            // Two registers at one home: the first one asked answers for every entry in it,
            // and every reference of the other kind dangles for ever with nothing said.
            if let Some(other) = homes.insert(home.dir.clone(), name.as_str()) {
                out.push(Finding::in_file(
                    &home.dir,
                    format!(
                        "the {} and {other} registers share the home `{}` under `{}`",
                        register.name,
                        home.dir.display(),
                        anchor.name
                    ),
                    "give each register a directory of its own; one home answers for one \
                     register, and the other's references resolve to nothing",
                ));
            }
            outside_the_walk(&mut out, manifest, anchor, register, &home);
            match register.shape {
                Shape::Heading => {
                    heading_home(&mut out, anchor, register, &home, model, anchors, inputs)
                }
                Shape::File => {
                    counts.entries +=
                        file_home(&mut out, anchor, register, &home, model, anchors, inputs);
                }
            }
        }
    }

    (out, counts)
}

/// A register home the walk does not read is a register the regime does not reach.
///
/// The home exists, so the declared-path check passes; its entries exist, so the directory
/// listing passes; but no document under it is in the model, so every entry-shape assertion
/// judges nothing and the run is green. One `[walk] skip-dirs` row would take a whole
/// register out of the regime, which `design@knowledge@the-regime-has-no-opt-out` refuses.
fn outside_the_walk(
    out: &mut Vec<Finding>,
    manifest: &Manifest,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
) {
    let walk = manifest.walk();
    let covers = |list: &[PathBuf]| {
        list.iter()
            .find(|p| home.dir.starts_with(p) || home.file.starts_with(p))
            .cloned()
    };
    let hit = covers(&walk.skip_dirs)
        .map(|p| ("[walk] skip-dirs", p))
        .or_else(|| covers(&walk.exclude).map(|p| ("[walk] exclude", p)))
        .or_else(|| covers(&walk.skip_files).map(|p| ("[walk] skip-files", p)));
    if let Some((list, path)) = hit {
        out.push(Finding::in_file(
            MANIFEST_NAME,
            format!(
                "`{}` in {list} takes the {} register of `{}` out of the walk",
                path.display(),
                register.name,
                anchor.name
            ),
            "delete the row, or move the register home out from under it; a home no \
             document of is walked is a register every assertion passes over in silence",
        ));
    }
}

/// What the manifest declares about anchors, judged before anything is looked for on disk.
fn declarations(out: &mut Vec<Finding>, manifest: &Manifest, anchors: &Anchors, inputs: &Inputs) {
    let mut by_name: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for anchor in anchors.all() {
        by_name
            .entry(anchor.name.as_str())
            .or_default()
            .push(where_it_is(anchor));
    }
    for (name, places) in &by_name {
        if places.len() > 1 {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "`{name}` names {} anchors: {}",
                    places.len(),
                    places.join(", ")
                ),
                "rename or move one; a reference names its anchor by that one word, so two \
                 of them cannot share it",
            ));
        }
        if !crate::entity::is_anchor_name(name) {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!("`{name}` cannot be spelled in a reference"),
                "name it in letters, digits, `.`, `-` and `_`; an anchor nothing can point \
                 at is one every pointer misses in silence",
            ));
        }
        // The reserved anchors are compiled in, so an anchor wearing one could never be the
        // target of a path reference: every pointer at it would read as the reserved meaning.
        if *name == ESCAPE_ANCHOR || *name == EVERY_ANCHOR {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!("`{name}` is a reserved anchor and cannot name an anchor"),
                "rename it; this word is reserved by the path kind",
            ));
        }
    }

    // A register's name is what a reference spells in kind position, so a name outside the id
    // grammar is a register nothing can point at, and `path` is a name the resolver answers
    // before it ever reaches the register list.
    for register in manifest.registers().all() {
        if register.name == crate::entity::PATH_KIND {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "`{}` is the reserved kind of a file or directory",
                    register.name
                ),
                "rename the register; a reference whose kind segment is this word resolves \
                 against the tree and never reaches the register",
            ));
        } else if !crate::entity::is_entity_id(&register.name) {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "`{}` cannot be spelled in a reference's kind segment",
                    register.name
                ),
                "name it in lower-case words joined by hyphens; a register nothing can point \
                 at is one every pointer misses in silence",
            ));
        }
    }

    // A register's home is `<home base>/<dir>`, so a `dir` that is not one plain segment
    // puts the home somewhere the anchor does not reach — and `..` puts it outside the
    // anchor entirely, where the real home then goes unchecked.
    for register in manifest.registers().all() {
        if !crate::entity::is_entity_id(&register.dir) {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "the {} register's directory `{}` is not one plain segment",
                    register.name, register.dir
                ),
                "name it in lower-case words joined by hyphens; a `/` or a `..` puts the \
                 home outside the anchor, and the real one is then read by nothing",
            ));
            continue;
        }
        // A component's homes sit under `docs/` beside its compiled documents, so a directory
        // name that spells one of those makes one file both the document and a register's
        // home. Every heading register has the file shape, so the `.md` is what collides;
        // the compiled documents are all files, so the directory shape collides with none.
        let document = format!("docs/{}.md", register.dir);
        if register.scope == Scope::Component && COMPONENT_DOCUMENTS.contains(&document.as_str()) {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "the {} register's directory `{}` makes its home the compiled document \
                     `{document}`",
                    register.name, register.dir
                ),
                "give the register another directory; a compiled document is what every \
                 component carries, and a file that is also a register home means two things",
            ));
        }
    }

    for (name, decl) in manifest.locations() {
        // A location whose path is empty is the project root, which is a component. Left
        // standing it is an anchor `is_root` skips, so nothing reports it, and it takes every
        // root document from the component under `Anchors::owning`.
        if decl.path.as_os_str().is_empty() || decl.path == Path::new(".") {
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!("[locations.{name}] names the project root"),
                "give it a directory of its own; the root is a component, and it carries \
                 every register already",
            ));
        } else if inputs.present.contains(&decl.path) && !inputs.directories.contains(&decl.path) {
            // Presence alone is satisfied by a file, and every home under it then reads as
            // absent, with a repair that says to create a directory inside a file.
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "[locations.{name}] names `{}`, which is a file",
                    decl.path.display()
                ),
                "a location is a directory carrying register homes; point it at one",
            ));
        }
        for register in &decl.registers {
            if manifest.registers().by_name(register).is_none() {
                out.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!("[locations.{name}] carries `{register}`, which is no register"),
                    format!(
                        "declare it in [registers.{register}], or name one of {}",
                        manifest.registers().listed()
                    ),
                ));
            }
        }
    }
    nesting(out, anchors);

    // **Every path the manifest declares is checked to exist.** A row naming a deleted file is
    // silent in both directions: nobody is told it is dead, and a file later created at that
    // path inherits what the row grants. That matters most for `exempt-files`, which
    // `check::regime::run` reads as well as the lint, so a stale row there can exempt a
    // document from the whole citation regime without anyone deciding to.
    //
    // Existence, never file-ness: `skip-dirs` names directories, `exclude` names either, and
    // an archive directory is a declared skip that is legitimately empty in a fresh checkout.
    //
    // A location's own path is not in this list: it is an anchor, and the anchor loop below
    // reports a directory that is not there. Both would be one fact reported twice.
    let walk = manifest.walk();
    let rules = manifest.rules();
    let corpus: Vec<PathBuf> = [&rules.text, &rules.version, &rules.past, &rules.manifest]
        .iter()
        .map(|p| rules.dir.join(p))
        .chain(std::iter::once(rules.dir.clone()))
        .collect();
    let declared: [(&str, &Vec<PathBuf>); 5] = [
        ("[walk] skip-dirs", &walk.skip_dirs),
        ("[walk] skip-files", &walk.skip_files),
        ("[walk] exclude", &walk.exclude),
        ("[lint] exempt-files", &manifest.lint().exempt_files),
        ("[rules]", &corpus),
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
                 outside the listing the walk reads and is never declared here",
            ));
        }
    }

    // **A file git both tracks and ignores is a finding naming the file.** The two states
    // contradict each other and the contradiction is silent: the walk reads the file, because
    // the tracked listing is unaffected by the ignore rules; and every path reference to it is
    // asserted, because `git check-ignore` skips what the index holds. So the ignore rule says
    // the file is out of the project and every check reads it in, and untracking it would flip
    // both answers at once. `design@knowledge@git-supplies-the-walk` is the head.
    for path in inputs.tracked_and_ignored {
        out.push(Finding::in_file(
            path,
            "git tracks this file and the ignore rules also cover it".to_string(),
            "untrack the file, or narrow the ignore rule that covers it; while both hold, the \
             walk reads the file and a path reference to it is asserted, which is the reverse \
             of what the ignore rule says",
        ));
    }

    // **A name the walk refuses is a finding naming the file**, per `walk::refused`. The file
    // was read by no check, so a rule quote in it is verified by nothing, and a reference is
    // one backticked span, so nothing can point at it either.
    for path in inputs.refused {
        out.push(Finding::in_file(
            path,
            "this file's name holds a line break, and no check read it".to_string(),
            "rename the file; a name with a newline or a carriage return in it fits on no \
             output line, in no reference and in no Windows checkout. Name it in [walk] \
             skip-files or in an ignore rule to keep it as it is",
        ));
    }
}

/// A heading register: exactly one home shape, and a directory home naming every subdocument.
#[allow(clippy::too_many_arguments)]
fn heading_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    model: &Model,
    anchors: &Anchors,
    inputs: &Inputs,
) {
    // A heading register has no per-instance option, so a file declaring one is a grouping
    // nothing reads. Left silent it looks like configuration that took effect.
    if inputs.present.contains(&home.config) {
        out.push(Finding::in_file(
            &home.config,
            format!(
                "`{}` sits beside the {} register, which is a heading register",
                home.config.display(),
                register.name
            ),
            "delete it; per-instance options belong to a file register, and a heading \
             register has none",
        ));
    }
    // The survey records each path's kind, so both impostor shapes land in the no-home arm by
    // fact rather than by inference: a directory wearing the file home's name is not the file
    // home, and a plain file named like the directory is not the directory home.
    let file_home = inputs.present.contains(&home.file) && !inputs.directories.contains(&home.file);
    let dir_home = inputs.directories.contains(&home.dir);
    match (file_home, dir_home) {
        (true, false) => {}
        (false, false) => out.push(Finding::in_file(
            &home.file,
            format!(
                "the anchor `{}` carries no {} home",
                anchor.name, register.name
            ),
            format!(
                "create `{}`, or `{}/` with a README.md; a heading register keeps its entries \
                 in exactly one of the two",
                home.file.display(),
                home.dir.display()
            ),
        )),
        (true, true) => out.push(Finding::in_file(
            &home.dir,
            format!(
                "the anchor `{}` carries both `{}` and `{}`",
                anchor.name,
                home.file.display(),
                home.dir.display()
            ),
            "keep exactly one home; with two, an entry lands in either and the reader who \
             finds the other acts on half the register",
        )),
        (false, true) => {
            // Without a head nothing can list the subdocuments, so the per-subdocument
            // findings would bury the one repair that fixes them all. A directory wearing
            // the README's name is no head either: the walk never reads a directory.
            if !inputs.present.contains(&home.readme) || inputs.directories.contains(&home.readme) {
                out.push(Finding::in_file(
                    &home.readme,
                    format!(
                        "the {} directory `{}` has no README.md",
                        register.name,
                        home.dir.display()
                    ),
                    "create it; the README is the directory home's head — an introduction, \
                     and a bullet list of markdown links naming every subdocument",
                ));
                return;
            }
            let linked = links(model, &home.readme, &home.dir);
            // Enumerated from the model rather than from the listing: a subdocument is what
            // the walk covers, so a gitignored scratch file owes no naming, a directory is
            // never one, and a document owned by an anchor nested under this directory is
            // that anchor's rather than a subdocument of this home.
            let mut subdocuments: Vec<&PathBuf> = model
                .documents()
                .iter()
                .map(|d| &d.rel)
                .filter(|p| {
                    p.starts_with(&home.dir)
                        && **p != home.readme
                        && p.extension().is_some_and(|e| e == "md")
                        && anchors.owning(p).path == anchor.path
                })
                .collect();
            subdocuments.sort();
            for subdocument in subdocuments {
                if !linked.contains(subdocument) {
                    out.push(Finding::in_file(
                        &home.readme,
                        format!(
                            "`{}` is not linked from its {} README",
                            subdocument.display(),
                            register.name
                        ),
                        "link it — [title](file.md), relative to the README — or delete the \
                         subdocument; a subdocument nobody links is a home nobody finds",
                    ));
                }
            }
        }
    }
}

/// A file register: the directory, its two navigation files, its groups, and every entry.
///
/// Returns how many entries were judged.
fn file_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    model: &Model,
    anchors: &Anchors,
    inputs: &Inputs,
) -> usize {
    // The single-file shape a heading register accepts is not a shape here, and a project
    // migrating from one is exactly where the mistake is made, so it is named.
    if inputs.present.contains(&home.file) {
        out.push(Finding::in_file(
            &home.file,
            format!(
                "`{}` is the retired file shape of the {} register",
                home.file.display(),
                register.name
            ),
            format!(
                "split it into `{}/`, one file per entry, with a README.md and an index.md",
                home.dir.display()
            ),
        ));
    }
    if !inputs.directories.contains(&home.dir) {
        out.push(Finding::in_file(
            &home.dir,
            format!(
                "the anchor `{}` carries no {} directory",
                anchor.name, register.name
            ),
            "create it with a README.md and an index.md; a file register keeps one file per \
             entry, and the README is what keeps the directory in git",
        ));
        return 0;
    }
    for (path, what) in [
        (&home.readme, "README.md"),
        // That the index is THERE is a fact about the register's shape, and it is
        // mandatory. What it holds is `check::generated`'s question, which compares the
        // committed bytes against a regeneration; this presence check reports the missing
        // file under the shape, and `generated` names the command that writes it.
        (&home.index, "index.md"),
    ] {
        if !inputs.present.contains(path) || inputs.directories.contains(path) {
            out.push(Finding::in_file(
                path,
                format!(
                    "the {} directory `{}` has no {what}",
                    register.name,
                    home.dir.display()
                ),
                "create it; the README is the hand-written head and the index is the \
                 generated listing, and neither stands in for the other",
            ));
        }
    }

    let groups = config(out, register, home, inputs);
    // What sits under an anchor nested inside this home is that anchor's, as the heading
    // shape reads it: the nesting is refused by `nesting`, and reading the nested anchor's
    // files as entries of this register would report the refusal's consequences against
    // the wrong register and the wrong anchor, nine times over.
    let owned = |rel: &Path| anchors.owning(rel).path == anchor.path;
    directory_contents(out, register, home, &groups, inputs, &owned);

    let mut judged = 0;
    for doc in model.documents() {
        if !is_entry(&doc.rel, home) || !owned(&doc.rel) {
            continue;
        }
        judged += 1;
        entry(out, register, doc);
    }
    judged
}

/// The declared groups of one instance, and the verdicts on the file that declares them.
fn config(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    inputs: &Inputs,
) -> BTreeSet<String> {
    let Some(text) = inputs.configs.get(&home.config) else {
        return BTreeSet::new();
    };
    match toml::from_str::<RegisterConfig>(text) {
        Ok(parsed) => parsed.groups.into_iter().collect(),
        Err(e) => {
            out.push(Finding::in_file(
                &home.config,
                format!("the {} register's options do not parse: {e}", register.name),
                "a register.toml declares `groups` and nothing else; a file that does not \
                 parse declares a grouping nobody checks",
            ));
            BTreeSet::new()
        }
    }
}

/// Every subdirectory is a declared group, every declared group is a subdirectory, groups
/// nest one level, and nothing but an entry and the options file sits in the instance.
///
/// `owned` says whether a path under the home belongs to this instance's anchor; what a
/// nested anchor owns is left to it.
fn directory_contents(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    groups: &BTreeSet<String>,
    inputs: &Inputs,
    owned: &dyn Fn(&Path) -> bool,
) {
    let mut present: BTreeSet<String> = BTreeSet::new();
    let mut paths: Vec<&PathBuf> = inputs
        .present
        .iter()
        .filter(|p| p.starts_with(&home.dir) && **p != home.dir && owned(p))
        .collect();
    paths.sort();
    for path in paths {
        let inside = path.strip_prefix(&home.dir).unwrap_or(path);
        let depth = inside.components().count();
        let name = inside
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if inputs.directories.contains(path) {
            if depth == 1 {
                present.insert(name.clone());
                if !groups.contains(&name) {
                    out.push(Finding::in_file(
                        path,
                        format!("`{name}` is a subdirectory of the {} register and is no declared group", register.name),
                        "declare it in the instance's register.toml, or move its entries up; \
                         a grouping nobody announced is one nobody reviewed",
                    ));
                }
            } else {
                out.push(Finding::in_file(
                    path,
                    format!("`{}` nests a group inside a group", path.display()),
                    "groups are one level deep; move the entries into a group of their own \
                     beside this one",
                ));
            }
            continue;
        }
        if *path == home.config {
            continue;
        }
        if path.extension().is_none_or(|e| e != "md") {
            out.push(Finding::in_file(
                path,
                format!(
                    "`{}` is not an entry of the {} register",
                    path.display(),
                    register.name
                ),
                "an instance holds one markdown file per entry, its README, its index and \
                 its register.toml; move anything else out",
            ));
        }
    }
    for group in groups {
        if !present.contains(group) {
            out.push(Finding::in_file(
                home.dir.join(group),
                format!(
                    "the {} register declares the group `{group}` and there is no directory",
                    register.name
                ),
                "create it, or remove the group from the instance's register.toml; the \
                 declaration is the grouping, so changing it is its own decision",
            ));
        }
    }
}

/// Whether a walked document is an entry of this instance.
fn is_entry(rel: &Path, home: &Home) -> bool {
    if !rel.starts_with(&home.dir) || rel.extension().is_none_or(|e| e != "md") {
        return false;
    }
    *rel != home.readme && *rel != home.index
}

/// One entry: its frontmatter, its title, its sections and the subsections its kind owes.
fn entry(out: &mut Vec<Finding>, register: &Register, doc: &Document) {
    let declared: Vec<&str> = register.metadata.iter().map(|(k, _)| k.as_str()).collect();
    let mut values: BTreeMap<&str, &str> = BTreeMap::new();
    match (&doc.parsed.frontmatter, declared.is_empty()) {
        (Some(Err(why)), _) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "the frontmatter of this {} entry does not parse: {why}",
                register.name
            ),
            "the accepted subset is a block opened and closed by a line holding only `---`, \
             at the very top of the file, holding `key: value` lines with scalar values",
        )),
        (Some(Ok(_)), true) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "this {} entry carries frontmatter and the register declares none",
                register.name
            ),
            "delete the block, or declare the keys in knowledge.toml under \
             [registers.<name>.metadata]",
        )),
        (None, false) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "this {} entry carries no frontmatter and the register declares {}",
                register.name,
                declared.join(", ")
            ),
            "open the file with a `---` block carrying each declared key",
        )),
        (None, true) => {}
        (Some(Ok(keys)), false) => {
            for (key, value) in keys {
                match register.metadata.iter().find(|(k, _)| k == key) {
                    None => out.push(Finding::in_file(
                        &doc.rel,
                        format!("`{key}` is no frontmatter key of the {} register", register.name),
                        format!("the declared keys are {}; a key nobody declared is a typo that would otherwise pass as an absent optional", declared.join(", ")),
                    )),
                    Some((_, accepted)) if !accepted.iter().any(|v| v == value) => {
                        out.push(Finding::in_file(
                            &doc.rel,
                            format!("`{key}: {value}` is no accepted value of the {} register", register.name),
                            format!("the accepted values are {}", accepted.join(", ")),
                        ))
                    }
                    Some(_) => {
                        values.insert(key.as_str(), value.as_str());
                    }
                }
            }
            for (key, _) in &register.metadata {
                if !keys.iter().any(|(k, _)| k == key) {
                    out.push(Finding::in_file(
                        &doc.rel,
                        format!("this {} entry declares no `{key}`", register.name),
                        "every declared metadata key is carried by every entry, so a listing \
                         can be built without reading the bodies",
                    ));
                }
            }
        }
    }

    let headings: Vec<(u8, &str)> = doc
        .observations
        .iter()
        .filter_map(|l| match &l.what {
            Observation::Heading { level, text } => Some((*level, text.as_str())),
            _ => None,
        })
        .collect();
    match headings.first() {
        Some((1, _)) => {}
        _ => out.push(Finding::in_file(
            &doc.rel,
            format!("this {} entry opens with no level-one title", register.name),
            "the first heading is the entry's title, on one line; a listing prints it and a \
             reader reads it before the body",
        )),
    }

    in_order(
        out,
        doc,
        register,
        2,
        &register
            .sections
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &headings,
        "level-two section",
    );
    // The subsections an issue owes sit under its last declared section, which is `Details`.
    // Only the issue register owes any, and the list its kind takes is the compiled one.
    let kind = register
        .metadata
        .first()
        .and_then(|(k, _)| values.get(k.as_str()).copied());
    let owed = register.owed_subsections(kind);
    if !owed.is_empty() {
        // Under that section and nowhere else. Read over the whole document, the three
        // subsections satisfy the assertion while sitting under the summary and leaving the
        // section that owes them empty, which is the shape a reader would call a defect.
        in_order(
            out,
            doc,
            register,
            3,
            owed,
            under_last_section(register, &headings),
            "level-three subsection",
        );
    }
}

/// The headings under a register's last declared section, which is where its owed subsections
/// sit.
///
/// Empty where the section is absent, so every owed subsection is reported and the absent
/// section is reported beside them: two facts, and the reader needs both.
fn under_last_section<'a>(
    register: &Register,
    headings: &'a [(u8, &'a str)],
) -> &'a [(u8, &'a str)] {
    let Some(last) = register.sections.last() else {
        return headings;
    };
    let Some(from) = headings.iter().position(|(l, t)| *l == 2 && t == last) else {
        return &[];
    };
    let rest = &headings[from + 1..];
    let to = rest.iter().position(|(l, _)| *l <= 2).unwrap_or(rest.len());
    &rest[..to]
}

/// Every owed heading of one level is present, in the declared order.
fn in_order(
    out: &mut Vec<Finding>,
    doc: &Document,
    register: &Register,
    level: u8,
    owed: &[&str],
    headings: &[(u8, &str)],
    what: &str,
) {
    let present: Vec<&str> = headings
        .iter()
        .filter(|(l, _)| *l == level)
        .map(|(_, t)| *t)
        .collect();
    let mut at = 0usize;
    for name in owed {
        match present[at..].iter().position(|t| t == name) {
            Some(i) => at += i + 1,
            None => out.push(Finding::in_file(
                &doc.rel,
                format!(
                    "this {} entry carries no {what} `{name}` after the ones before it",
                    register.name
                ),
                format!("the owed {what}s, in this order: {}", owed.join(", ")),
            )),
        }
    }
}

/// No two anchors give one path two meanings.
///
/// An anchor is a directory, and its register homes sit under it. The deepest anchor owns
/// every document under its path, per `design@knowledge@every-path-names-its-anchor`, so an
/// anchor inside another's directory is the ordinary case — this repository's two locations
/// both sit inside the root component. What that rule cannot absorb is an anchor whose
/// directory is comparable with another anchor's register home, and both directions are
/// refused here, per `design@knowledge@anchors-are-components-and-locations`:
///
/// - **the deeper anchor sits inside, or at, a home of the shallower one.** Every file under
///   it would be read as an entry or a subdocument of the outer register, and reported
///   against that register and its anchor rather than against the declaration.
/// - **the deeper anchor's directory holds a home of the shallower one.** Being deeper, it
///   would own every document in that home, and each slug defined there would be reported
///   as misplaced against an anchor that carries no such register.
///
/// Two anchors at one path are the third shape: neither is deeper, `Anchors::owning` picks
/// one on a tie, and every document under the path belongs to whichever it picked.
///
/// A component inside a location is refused on its own terms first, whether or not a home is
/// involved: it nests a full register set inside a partial one. The pair is then not judged
/// again here, so one declaration is one finding.
fn nesting(out: &mut Vec<Finding>, anchors: &Anchors) {
    let mut by_path: BTreeMap<&Path, Vec<&str>> = BTreeMap::new();
    for anchor in anchors.all() {
        by_path
            .entry(anchor.path.as_path())
            .or_default()
            .push(anchor.name.as_str());
    }
    for (path, names) in &by_path {
        // A location naming the root is reported as that by `declarations`, and the root
        // component sitting at the same path is the same fact.
        if names.len() > 1 && !path.as_os_str().is_empty() {
            let named: Vec<String> = names.iter().map(|n| format!("`{n}`")).collect();
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "two anchors sit at `{}`: {}",
                    path.display(),
                    named.join(", ")
                ),
                "move one; the deepest anchor owns every document under its path, and at one \
                 path nothing decides which of the two that is",
            ));
        }
    }

    let kind = |anchor: &Anchor| {
        if anchor.is_component {
            "component"
        } else {
            "location"
        }
    };
    for outer in anchors.all() {
        for inner in anchors.all() {
            if inner.path.components().count() <= outer.path.components().count()
                || !inner.path.starts_with(&outer.path)
            {
                continue;
            }
            if inner.is_component && !outer.is_component {
                out.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!(
                        "the component `{}` sits inside the location `{}`",
                        inner.name, outer.name
                    ),
                    "move one out of the other; a location carries a subset of the registers \
                     and a component carries them all, so nesting gives one document two homes",
                ));
                continue;
            }
            let mut held: Vec<&str> = Vec::new();
            for name in &outer.registers {
                let Some(register) = anchors.registers().by_name(name) else {
                    continue;
                };
                // The directory shape alone: a heading register's file sits beside its
                // directory, so whatever holds the one holds the other, and an anchor cannot
                // sit inside a file.
                let home = outer.home_of(register).dir;
                if inner.path.starts_with(&home) {
                    out.push(Finding::in_file(
                        MANIFEST_NAME,
                        format!(
                            "the {} `{}` sits inside the {} register's directory `{}` of `{}`",
                            kind(inner),
                            inner.name,
                            register.name,
                            home.display(),
                            outer.name
                        ),
                        "move it out; the directory is the register's home in one shape and \
                         reserved in the other, and every file under the anchor would be \
                         read as an entry or a subdocument of it",
                    ));
                } else if home.starts_with(&inner.path) {
                    held.push(&register.name);
                }
            }
            if !held.is_empty() {
                out.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!(
                        "the {} `{}` at `{}` holds the {} of `{}`",
                        kind(inner),
                        inner.name,
                        inner.path.display(),
                        list_homes(&held),
                        outer.name
                    ),
                    "move it out; the deepest anchor owns every document under its path, so \
                     each slug defined in those homes would be misplaced against an anchor \
                     that carries no such register",
                ));
            }
        }
    }
}

/// `design home`, `design and goal homes`, `design, goal and issue homes`.
fn list_homes(registers: &[&str]) -> String {
    match registers {
        [one] => format!("{one} home"),
        [head @ .., last] => format!("{} and {last} homes", head.join(", ")),
        [] => String::new(),
    }
}

/// Every path the README's markdown links resolve to.
///
/// A link's target resolves against the README's own directory, the way a renderer follows
/// it. A URL, a bare fragment and an absolute path are not index rows and are passed over; a
/// fragment on a file target is dropped before resolution. Whether each link RESOLVES is
/// `check::references`' assertion — what is asserted here is the other direction, that every
/// subdocument has a row. Backticked paths are the pointer forms of prose and do not name a
/// subdocument: the index is made of links a reader can follow.
fn links(model: &Model, readme: &PathBuf, dir: &Path) -> HashSet<PathBuf> {
    let Some(doc) = model.documents().iter().find(|d| d.rel == *readme) else {
        return HashSet::new();
    };
    let mut resolved = HashSet::new();
    for l in &doc.observations {
        let Observation::Link(target) = &l.what else {
            continue;
        };
        if target.starts_with('#') {
            continue;
        }
        let file_part = target.split('#').next().unwrap_or(target);
        if super::references::has_scheme(file_part) || file_part.starts_with('/') {
            continue;
        }
        resolved.insert(dir.join(file_part));
    }
    resolved
}

/// How a finding names an anchor's place: its path, or the root.
fn where_it_is(anchor: &Anchor) -> String {
    if anchor.is_root() {
        "the project root".to_string()
    } else {
        anchor.path.display().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::citations::Release;
    use std::collections::HashMap;
    use std::path::PathBuf;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@knowledge@checker-source-literals-are-data`.

    /// A manifest declaring `components`, against a root nothing reads.
    fn declaring(components: &str) -> Manifest {
        declaring_full(components, "", "[]", "[]", "[]", "[]")
    }

    /// The same, with every path list the manifest can declare spelled out.
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
             {extra}\n\
             [walk]\nskip-dirs = {skip_dirs}\nskip-files = {skip_files}\n\
             exclude = {exclude}\n\n\
             [lint]\nexempt-files = {exempt}\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The findings against a listing of what exists.
    fn findings<S: AsRef<str>>(manifest: &Manifest, present: &[S]) -> Vec<String> {
        findings_over(manifest, present, Model::from_documents(Vec::new()), &[])
    }

    /// The same, with documents in the model and register options beside the instances.
    ///
    /// The kinds are implied by the listing's shape: a path with an entry beneath it is a
    /// directory, which is how every committable tree looks.
    fn findings_over<S: AsRef<str>>(
        manifest: &Manifest,
        present: &[S],
        model: Model,
        configs: &[(&str, &str)],
    ) -> Vec<String> {
        let releases: HashMap<Option<String>, Release> = HashMap::new();
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = present.iter().map(|p| PathBuf::from(p.as_ref())).collect();
        let directories = crate::check::testing::implied_directories(&present);
        let outside = Vec::new();
        let configs: HashMap<PathBuf, String> = configs
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let inputs = Inputs {
            releases: &releases,
            pinned: "",
            committed: &committed,
            configs: &configs,
            present: &present,
            directories: &directories,
            outside: &outside,
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
        };
        check(&model, manifest, &inputs)
            .0
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    /// Every path a component owes, under `dir`, with the file-shaped heading homes.
    fn all_of(dir: &str) -> Vec<String> {
        let at = |name: &str| {
            if dir.is_empty() {
                name.to_string()
            } else {
                format!("{dir}/{name}")
            }
        };
        let mut out: Vec<String> = COMPONENT_DOCUMENTS.iter().map(|n| at(n)).collect();
        out.push(at("docs"));
        for name in ["design.md", "goals.md", "tripwires.md"] {
            out.push(at(&format!("docs/{name}")));
        }
        out.push(at("docs/open-issues"));
        out.push(at("docs/open-issues/README.md"));
        out.push(at("docs/open-issues/index.md"));
        // The corpus the declaration below names, which is a declared path like any other.
        for name in ["r", "r/t", "r/v", "r/p", "r/m"] {
            out.push(name.to_string());
        }
        out
    }

    #[test]
    fn a_component_carrying_every_home_reports_nothing() {
        let manifest = declaring("");
        let present = all_of("");
        let found = findings(&manifest, &present);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn each_missing_home_is_reported_at_the_path_it_belongs_at() {
        let manifest = declaring("");
        // Every home but the issue directory, so exactly one finding is owed.
        let mut present = all_of("");
        present.retain(|p| !p.starts_with("docs/open-issues"));
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("docs/open-issues") && found[0].contains("no issue directory"),
            "{found:#?}"
        );
        // The goals home is required of every component, which it was not before registers.
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no goal home"), "{found:#?}");
    }

    #[test]
    fn the_retired_file_shape_of_a_file_register_is_named_as_what_it_was() {
        // The one migration a project meets: a tracker that was a document becomes a
        // directory of entries, and the old file left beside it is read by nothing.
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("retired file shape"), "{found:#?}");
    }

    #[test]
    fn a_file_register_owes_a_readme_and_an_index_and_neither_stands_in_for_the_other() {
        let manifest = declaring("");
        for missing in ["docs/open-issues/README.md", "docs/open-issues/index.md"] {
            let mut present = all_of("");
            present.retain(|p| p != missing);
            let found = findings(&manifest, &present);
            assert_eq!(found.len(), 1, "{missing}: {found:#?}");
            assert!(found[0].starts_with(missing), "{found:#?}");
        }
    }

    #[test]
    fn a_heading_register_carries_exactly_one_home_shape() {
        let manifest = declaring("");
        // Both shapes at once: the entries have two candidate places to land.
        let mut present = all_of("");
        present.push("docs/design".to_string());
        present.push("docs/design/README.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("carries both"), "{found:#?}");
        // The directory shape alone, headed by its README: nothing is owed.
        let mut present = all_of("");
        present.retain(|p| p != "docs/tripwires.md");
        present.push("docs/tripwires".to_string());
        present.push("docs/tripwires/README.md".to_string());
        assert!(findings(&manifest, &present).is_empty());
    }

    #[test]
    fn a_register_options_file_beside_a_heading_register_is_reported() {
        // A heading register has no per-instance option, so a file declaring one took effect
        // nowhere. Silent, it reads as configuration that did.
        let manifest = declaring("");
        let mut present = all_of("");
        present.retain(|p| p != "docs/design.md");
        present.push("docs/design".to_string());
        present.push("docs/design/README.md".to_string());
        present.push("docs/design/register.toml".to_string());
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(vec![(
                PathBuf::from("docs/design/README.md"),
                "# Design\n".to_string(),
            )]),
            &[("docs/design/register.toml", "groups = []\n")],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("is a heading register"), "{found:#?}");
    }

    #[test]
    fn a_directory_home_readme_must_link_every_subdocument_of_any_heading_register() {
        // Generalised from design: the assertion is the register's, not one register's.
        let manifest = declaring("");
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/one.md".to_string());
        let model = Model::from_documents(vec![
            (
                PathBuf::from("docs/goals/README.md"),
                "# Goals\n\n- [Another](other.md)\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/one.md"),
                "## A goal `##one`\n".to_string(),
            ),
        ]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("docs/goals/one.md") && found[0].contains("goal README"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_subdocument_is_a_markdown_file_this_anchor_owns_and_nothing_else() {
        // Two things the enumeration excludes, each a false demand on the README if it did
        // not: a Rust source under the home, which the model holds like any other document,
        // and a markdown file belonging to an anchor nested under the home. The nested
        // anchor is itself refused, and that refusal is the one finding: the README is not
        // asked to link a document the refused anchor owns.
        let manifest = declaring("\"docs/goals/nested\"");
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/one.md".to_string());
        present.push("docs/goals/gen.rs".to_string());
        present.push("docs/goals/nested".to_string());
        present.extend(all_of("docs/goals/nested"));
        let model = Model::from_documents(vec![
            (
                PathBuf::from("docs/goals/README.md"),
                "# Goals\n\n- [One](one.md)\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/one.md"),
                "## A goal `##one`\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/gen.rs"),
                "// a generator that lives beside the home\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/nested/docs/goals/README.md"),
                "# Goals\n".to_string(),
            ),
        ]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("sits inside the goal register's directory"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_group_is_declared_in_the_instance_and_asserted_in_both_directions() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        present.push("docs/open-issues/a-group".to_string());
        present.push("docs/open-issues/a-group/an-entry.md".to_string());
        // Declared and present: silent.
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = [\"a-group\"]\n")],
        );
        assert!(found.is_empty(), "{found:#?}");
        // Present and undeclared.
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = []\n")],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no declared group"), "{found:#?}");
        // Declared and absent.
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[(
                "docs/open-issues/register.toml",
                "groups = [\"a-group\", \"gone\"]\n",
            )],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("`gone`"), "{found:#?}");
    }

    #[test]
    fn a_group_inside_a_group_and_a_file_of_another_suffix_are_each_reported() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        present.push("docs/open-issues/a-group".to_string());
        present.push("docs/open-issues/a-group/deeper".to_string());
        present.push("docs/open-issues/a-group/deeper/an-entry.md".to_string());
        present.push("docs/open-issues/notes.txt".to_string());
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = [\"a-group\"]\n")],
        );
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found
                .iter()
                .any(|f| f.contains("nests a group inside a group")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.contains("is not an entry of the issue register")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_register_options_file_that_does_not_parse_is_reported_rather_than_ignored() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        for text in ["theme = [\"x\"]\n", "groups = 3\n", "\n"] {
            let found = findings_over(
                &manifest,
                &present,
                Model::from_documents(Vec::new()),
                &[("docs/open-issues/register.toml", text)],
            );
            assert_eq!(found.len(), 1, "{text:?}: {found:#?}");
            assert!(found[0].contains("do not parse"), "{text:?}: {found:#?}");
        }
    }

    /// The listing and the model of one issue instance holding one entry.
    fn with_entry(body: &str) -> (Vec<String>, Model) {
        let mut present = all_of("");
        present.push("docs/open-issues/an-entry.md".to_string());
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/open-issues/an-entry.md"),
            body.to_string(),
        )]);
        (present, model)
    }

    fn entry_findings(body: &str) -> Vec<String> {
        let manifest = declaring("");
        let (present, model) = with_entry(body);
        findings_over(&manifest, &present, model, &[])
    }

    /// A well-formed issue entry, which every case below breaks in exactly one way.
    const ENTRY: &str = "---\nkind: defect\n---\n# The title\n\n## Summary\n\nWhat it is.\n\n\
                         ## Details\n\n### What\n\nOne.\n\n### Why it matters\n\nTwo.\n\n\
                         ### What would close it\n\nThree.\n";

    #[test]
    fn a_well_formed_entry_reports_nothing() {
        assert!(
            entry_findings(ENTRY).is_empty(),
            "{:#?}",
            entry_findings(ENTRY)
        );
    }

    #[test]
    fn an_entry_owes_its_declared_metadata_with_a_declared_value() {
        let no_block = entry_findings(&ENTRY.replace("---\nkind: defect\n---\n", ""));
        assert_eq!(no_block.len(), 1, "{no_block:#?}");
        assert!(
            no_block[0].contains("carries no frontmatter"),
            "{no_block:#?}"
        );
        let wrong = entry_findings(&ENTRY.replace("kind: defect", "kind: nonesuch"));
        assert_eq!(wrong.len(), 1, "{wrong:#?}");
        assert!(wrong[0].contains("no accepted value"), "{wrong:#?}");
        let extra = entry_findings(&ENTRY.replace("kind: defect", "kind: defect\ntheme: x"));
        assert_eq!(extra.len(), 1, "{extra:#?}");
        assert!(
            extra[0].contains("`theme` is no frontmatter key"),
            "{extra:#?}"
        );
        let other = entry_findings(&ENTRY.replace("kind: defect", "theme: x"));
        assert_eq!(other.len(), 2, "{other:#?}");
        assert!(
            other.iter().any(|f| f.contains("declares no `kind`")),
            "{other:#?}"
        );
    }

    #[test]
    fn an_entry_owes_a_level_one_title_its_sections_in_order_and_the_subsections_its_kind_owes() {
        let no_title = entry_findings(&ENTRY.replace("# The title", "## The title"));
        assert!(
            no_title.iter().any(|f| f.contains("no level-one title")),
            "{no_title:#?}"
        );
        let no_section = entry_findings(&ENTRY.replace("## Summary", "## Overview"));
        assert!(
            no_section
                .iter()
                .any(|f| f.contains("no level-two section `Summary`")),
            "{no_section:#?}"
        );
        // Present, and after the section that must follow it: order is asserted, not presence.
        let swapped = ENTRY.replace(
            "## Summary\n\nWhat it is.\n\n## Details",
            "## Details\n\nx\n\n## Summary",
        );
        let out_of_order = entry_findings(&swapped);
        assert!(
            out_of_order
                .iter()
                .any(|f| f.contains("no level-two section `Details`")),
            "{out_of_order:#?}"
        );
        let no_sub = entry_findings(&ENTRY.replace("### Why it matters", "### Why"));
        assert!(
            no_sub
                .iter()
                .any(|f| f.contains("no level-three subsection `Why it matters`")),
            "{no_sub:#?}"
        );
    }

    #[test]
    fn the_owed_subsections_are_asserted_under_the_last_declared_section_and_nowhere_else() {
        // All three present, all three under the summary, and the section that owes them
        // left empty. Read over the whole document the entry passes, which is the shape a
        // reader would call a defect.
        let misplaced = "---\nkind: defect\n---\n# The title\n\n## Summary\n\n\
                         ### What\n\nOne.\n\n### Why it matters\n\nTwo.\n\n\
                         ### What would close it\n\nThree.\n\n## Details\n\nNothing here.\n";
        let found = entry_findings(misplaced);
        assert_eq!(found.len(), 3, "{found:#?}");
        for name in ["What", "Why it matters", "What would close it"] {
            assert!(
                found
                    .iter()
                    .any(|f| f.contains(&format!("subsection `{name}`"))),
                "{name}: {found:#?}"
            );
        }
        // And the section itself absent: the subsections are owed and none is found, beside
        // the finding naming the missing section.
        let no_section = "---\nkind: defect\n---\n# The title\n\n## Summary\n\nWhat it is.\n";
        let found = entry_findings(no_section);
        assert_eq!(found.len(), 4, "{found:#?}");
        assert!(
            found.iter().any(|f| f.contains("section `Details`")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_deferred_entry_owes_a_trigger_where_the_others_owe_a_closure() {
        // The kind decides the last owed subsection, which is what makes the kind list worth
        // being closed: an unknown kind would silently owe the wrong three.
        let deferred = ENTRY
            .replace("kind: defect", "kind: deferred")
            .replace("### What would close it", "### Trigger");
        assert!(
            entry_findings(&deferred).is_empty(),
            "{:#?}",
            entry_findings(&deferred)
        );
        let wrong = ENTRY.replace("kind: defect", "kind: deferred");
        let found = entry_findings(&wrong);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("`Trigger`"), "{found:#?}");
    }

    #[test]
    fn frontmatter_outside_the_subset_is_reported_and_a_register_declaring_none_refuses_a_block() {
        let bad = entry_findings(&ENTRY.replace("kind: defect", "kind:\n  nested: 1"));
        assert!(bad.iter().any(|f| f.contains("does not parse")), "{bad:#?}");
        // A register with no declared metadata refuses a block: the keys would be unchecked.
        let manifest = declaring_full(
            "",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"reading\"]\n\n\
             [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("notes".to_string());
        present.push("notes/readings".to_string());
        present.push("notes/readings/README.md".to_string());
        present.push("notes/readings/index.md".to_string());
        present.push("notes/readings/a-reading.md".to_string());
        let model = Model::from_documents(vec![(
            PathBuf::from("notes/readings/a-reading.md"),
            "---\nstatus: settled\n---\n# A reading\n".to_string(),
        )]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("declares none"), "{found:#?}");
    }

    #[test]
    fn a_location_carries_the_registers_it_declares_and_no_others() {
        let manifest = declaring_full(
            "",
            "[locations.agent-config]\npath = \".claude\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push(".claude".to_string());
        present.push(".claude/open-issues".to_string());
        present.push(".claude/open-issues/README.md".to_string());
        present.push(".claude/open-issues/index.md".to_string());
        // A location owes no README of its own, no CLAUDE.md and no design home.
        assert!(
            findings(&manifest, &present).is_empty(),
            "{:#?}",
            findings(&manifest, &present)
        );
        // Its path is an anchor's directory, and a missing one is ONE finding rather than a
        // declared-path finding beside it.
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("`.claude` is declared an anchor and does not exist"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_location_naming_no_such_register_and_a_component_inside_one_are_each_reported() {
        let manifest = declaring_full(
            "\"docs/a-part\"",
            "[locations.docs]\npath = \"docs\"\nregisters = [\"nonesuch\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &["docs"]);
        assert!(
            found
                .iter()
                .any(|f| f.contains("`nonesuch`, which is no register")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.contains("sits inside the location `docs`")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_register_declaration_the_manifest_refused_is_reported_here() {
        // The manifest holds the complaint rather than refusing to load, because a manifest
        // that will not load reports nothing at all, and nothing at all reads as conformance.
        let manifest = declaring_full(
            "",
            "[registers.issue]\ndir = \"issues\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with(MANIFEST_NAME) && found[0].contains("built-in register"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_declared_path_that_does_not_exist_is_reported_wherever_it_is_declared() {
        let manifest = declaring_full("", "", "[\"gone\"]", "[]", "[]", "[]");
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("[walk] skip-dirs"), "{found:#?}");
    }

    #[test]
    fn a_component_directory_that_does_not_exist_is_one_finding() {
        let manifest = declaring("\"parts/gone\"");
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
    }

    #[test]
    fn an_anchor_name_that_is_ambiguous_or_reserved_is_reported_at_the_manifest() {
        let manifest = declaring("\"a/widget\", \"b/widget\"");
        let found = findings(&manifest, &["a/widget", "b/widget"]);
        assert!(
            found.iter().any(|f| f.contains("names 2 anchors")),
            "{found:#?}"
        );
        let manifest = declaring_full(
            "",
            "[locations.elsewhere]\npath = \"x\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &["x"]);
        assert!(
            found.iter().any(|f| f.contains("is a reserved anchor")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_register_home_the_walk_does_not_read_is_reported_at_the_row_that_takes_it() {
        // One `[walk]` row would otherwise take a whole register out of the regime: the home
        // is there, its entries are there, and no document under it is in the model, so
        // every entry-shape assertion judges nothing and the run is green.
        for (row, list) in [
            ("[walk] skip-dirs", "skip_dirs"),
            ("[walk] exclude", "exclude"),
        ] {
            let (dirs, exclude) = if list == "skip_dirs" {
                ("[\"docs/open-issues\"]", "[]")
            } else {
                ("[]", "[\"docs/open-issues\"]")
            };
            let manifest = declaring_full("", "", dirs, "[]", exclude, "[]");
            let found = findings(&manifest, &all_of(""));
            assert_eq!(found.len(), 1, "{row}: {found:#?}");
            assert!(
                found[0].contains(row) && found[0].contains("out of the walk"),
                "{row}: {found:#?}"
            );
        }
    }

    #[test]
    fn two_registers_of_one_anchor_may_not_share_a_home() {
        // The first register asked answers for every entry in the shared home, and every
        // reference of the other kind resolves to nothing for ever with nothing said.
        let manifest = declaring_full(
            "",
            "[registers.note]\nscope = \"component\"\nshape = \"heading\"\ndir = \"design\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("share the home"), "{found:#?}");
    }

    #[test]
    fn a_register_directory_that_is_not_one_plain_segment_is_reported() {
        // `..` puts the home outside the anchor and the real one is then read by nothing.
        for dir in ["", "..", "a/b", "Design"] {
            let manifest = declaring_full(
                "",
                &format!(
                    "[registers.note]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"{dir}\"\n\n"
                ),
                "[]",
                "[]",
                "[]",
                "[]",
            );
            let found = findings(&manifest, &all_of(""));
            assert_eq!(found.len(), 1, "{dir:?}: {found:#?}");
            assert!(
                found[0].contains("not one plain segment"),
                "{dir:?}: {found:#?}"
            );
        }
    }

    #[test]
    fn a_location_that_names_the_root_or_a_file_is_reported() {
        // The root is a component and carries every register already; a file satisfies a
        // presence test and every home under it then reads as absent.
        let manifest = declaring_full(
            "",
            "[locations.here]\npath = \"\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("names the project root"), "{found:#?}");
        let manifest = declaring_full(
            "",
            "[locations.here]\npath = \"README.md\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("which is a file"), "{found:#?}");
    }

    #[test]
    fn a_register_name_no_reference_can_spell_and_the_reserved_kind_are_each_reported() {
        for (body, needle) in [
            (
                "[registers.\"odd name\"]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"odd\"\n\n",
                "kind segment",
            ),
            (
                "[registers.path]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"paths\"\n\n",
                "reserved kind",
            ),
        ] {
            let manifest = declaring_full("", body, "[]", "[]", "[]", "[]");
            let found = findings(&manifest, &all_of(""));
            assert_eq!(found.len(), 1, "{body:?}: {found:#?}");
            assert!(found[0].contains(needle), "{body:?}: {found:#?}");
        }
    }

    #[test]
    fn an_anchor_name_no_reference_can_spell_is_reported_at_the_manifest() {
        // The name a reference spells comes from the path's basename, so a space in the path
        // makes an anchor nothing can point at.
        let manifest = declaring("\"parts/my thing\"");
        let found = findings(&manifest, &all_of(""));
        assert!(
            found
                .iter()
                .any(|f| f.contains("cannot be spelled in a reference")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_directory_wearing_a_component_document_s_name_is_reported() {
        // A presence test is satisfied and the walk never reads a directory as a document,
        // so every claim that should live in it is outside every check.
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("CLAUDE.md/inside.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("is a directory wearing the document's name"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_declaration_the_tool_then_discards_is_complained_about() {
        // Each of these is stored and read by nobody, so without the complaint it is
        // configuration that looks applied and is not.
        for (body, needle) in [
            (
                "[registers.issue]\n[registers.issue.metadata.theme]\nvalues = [\"a\"]\n\n",
                "sets metadata on a built-in register",
            ),
            (
                "[registers.note]\nscope = \"component\"\nshape = \"heading\"\ndir = \"notes\"\n\
                 sections = [\"One\"]\n\n",
                "on a heading register",
            ),
            (
                "[registers.note]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"notes\"\n\
                 kinds = [\"a\"]\n\n",
                "which only [registers.issue] takes",
            ),
        ] {
            let manifest = declaring_full("", body, "[]", "[]", "[]", "[]");
            let found = findings(&manifest, &all_of(""));
            assert!(
                found.iter().any(|f| f.contains(needle)),
                "{body:?}: {found:#?}"
            );
        }
    }

    #[test]
    fn a_misplaced_definition_is_this_family_s_finding() {
        // Moved from `references`, which parked it while the register shapes were unbuilt.
        let manifest = declaring("");
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# A title `##at-level-one`\n".to_string(),
        )]);
        let found = findings_over(&manifest, &all_of(""), model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("defines nothing"), "{found:#?}");
    }

    #[test]
    fn an_anchor_inside_another_anchor_s_register_home_is_reported() {
        // The reachable shape: a location declared inside a component's issue directory.
        // Every file under it would be read as an entry of the OUTER register, and the
        // findings would name that register and its anchor rather than the declaration.
        let manifest = declaring_full(
            "",
            "[locations.nested]\npath = \"docs/open-issues/nested\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/open-issues/nested".to_string());
        present.push("docs/open-issues/nested/open-issues".to_string());
        present.push("docs/open-issues/nested/open-issues/README.md".to_string());
        present.push("docs/open-issues/nested/open-issues/index.md".to_string());
        present.push("docs/open-issues/nested/scratch.txt".to_string());
        // The nested anchor's README is a walked document and a stray file sits beside its
        // home, and nothing under the nested anchor is an entry, a group or a stray file of
        // the OUTER register: the refusal is the one finding, so a reader repairs the
        // declaration and not the directory.
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/open-issues/nested/open-issues/README.md"),
            "# Nested issues\n".to_string(),
        )]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("the location `nested` sits inside the issue register's directory")
                && found[0].contains("`docs/open-issues` of `a-project`"),
            "{found:#?}"
        );
    }

    #[test]
    fn an_anchor_at_exactly_another_s_register_home_is_reported() {
        // At the home, not only inside it: the anchor's directory IS the outer register's
        // directory, and every file it holds would be read twice.
        let manifest = declaring_full(
            "",
            "[locations.at]\npath = \"docs/open-issues\"\nregisters = [\"tripwire\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/open-issues/tripwires.md".to_string());
        let found = findings(&manifest, &present);
        assert!(
            found.iter().any(|f| f.contains(
                "the location `at` sits inside the issue register's directory `docs/open-issues`"
            )),
            "{found:#?}"
        );
    }

    #[test]
    fn a_component_inside_a_location_s_home_is_one_finding_and_not_two() {
        // The pair is refused as a component inside a location, and not judged again for the
        // home it also sits in: one declaration, one finding.
        let manifest = declaring_full(
            "\"notes/open-issues/part\"",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("notes".to_string());
        present.push("notes/open-issues".to_string());
        present.push("notes/open-issues/README.md".to_string());
        present.push("notes/open-issues/index.md".to_string());
        present.push("notes/open-issues/part".to_string());
        present.extend(all_of("notes/open-issues/part"));
        let found = findings(&manifest, &present);
        let about: Vec<&String> = found.iter().filter(|f| f.contains("`part`")).collect();
        assert_eq!(about.len(), 1, "{found:#?}");
        assert!(
            about[0].contains("the component `part` sits inside the location `notes`"),
            "{found:#?}"
        );
    }

    #[test]
    fn an_anchor_whose_directory_holds_another_s_register_home_is_reported() {
        // A location at a component's `docs/` is deeper than the component, so it would own
        // every document in the component's homes, and each slug defined there would be
        // reported as misplaced against a location that carries no such register.
        let manifest = declaring_full(
            "",
            "[locations.papers]\npath = \"docs\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        let holding: Vec<&String> = found
            .iter()
            .filter(|f| f.contains("`papers`") && f.contains("holds the"))
            .collect();
        // One finding for the pair, naming every home held rather than one per home: four
        // findings from one declaration would bury the declaration.
        assert_eq!(holding.len(), 1, "{found:#?}");
        assert!(
            holding[0].contains(
                "the location `papers` at `docs` holds the design, goal, tripwire and issue \
                 homes of `a-project`"
            ),
            "{found:#?}"
        );
    }

    #[test]
    fn a_component_inside_another_component_s_register_home_is_reported() {
        // The same collision between two components: a component declared under the outer
        // goal home's directory would be both a subdirectory of that home and an anchor.
        let manifest = declaring("\"docs/goals/nested\"");
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/nested".to_string());
        present.extend(all_of("docs/goals/nested"));
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/goals/README.md"),
            "# Goals\n".to_string(),
        )]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("the component `nested` sits inside the goal register's directory"),
            "{found:#?}"
        );
    }

    #[test]
    fn the_homes_a_finding_lists_read_as_prose_at_every_count() {
        assert_eq!(list_homes(&["design"]), "design home");
        assert_eq!(list_homes(&["design", "goal"]), "design and goal homes");
        assert_eq!(
            list_homes(&["design", "goal", "issue"]),
            "design, goal and issue homes"
        );
    }

    #[test]
    fn two_anchors_at_one_path_are_reported_once() {
        // Two locations at one directory: `Anchors::owning` picks one on a tie, and every
        // document under the path then belongs to whichever it picked. A component and a
        // location at one path is the same fact, and it is reported as this rather than as
        // the component sitting inside the location.
        for (extra, needle) in [
            (
                "[locations.a]\npath = \"notes\"\nregisters = [\"issue\"]\n\n\
                 [locations.b]\npath = \"notes\"\nregisters = [\"tripwire\"]\n\n",
                "`a`, `b`",
            ),
            (
                "[locations.notes]\npath = \"parts\"\nregisters = [\"tripwire\"]\n\n",
                "`parts`, `notes`",
            ),
        ] {
            let components = if extra.contains("parts") {
                "\"parts\""
            } else {
                ""
            };
            let manifest = declaring_full(components, extra, "[]", "[]", "[]", "[]");
            let mut present = all_of("");
            present.push("notes".to_string());
            present.push("notes/open-issues".to_string());
            present.push("notes/open-issues/README.md".to_string());
            present.push("notes/open-issues/index.md".to_string());
            present.push("notes/tripwires.md".to_string());
            present.push("parts".to_string());
            present.push("parts/tripwires.md".to_string());
            present.extend(all_of("parts"));
            let found = findings(&manifest, &present);
            let same: Vec<&String> = found
                .iter()
                .filter(|f| f.contains("two anchors sit at"))
                .collect();
            assert_eq!(same.len(), 1, "{extra}: {found:#?}");
            assert!(same[0].contains(needle), "{extra}: {found:#?}");
            assert!(
                !found.iter().any(|f| f.contains("sits inside the location")),
                "{extra}: {found:#?}"
            );
        }
    }

    #[test]
    fn an_anchor_inside_another_s_directory_and_beside_its_homes_is_silent() {
        // The ordinary case, which the deepest-anchor rule exists for: a location inside a
        // component, away from every register home, carrying homes of its own.
        let manifest = declaring_full(
            "",
            "[locations.plans]\npath = \"docs/plans\"\nregisters = [\"issue\"]\n\n\
             [locations.inner]\npath = \"docs/plans/inner\"\nregisters = [\"tripwire\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/plans".to_string());
        present.push("docs/plans/open-issues".to_string());
        present.push("docs/plans/open-issues/README.md".to_string());
        present.push("docs/plans/open-issues/index.md".to_string());
        present.push("docs/plans/inner".to_string());
        present.push("docs/plans/inner/tripwires.md".to_string());
        let found = findings(&manifest, &present);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn a_refused_name_is_one_finding_naming_the_file_on_one_line() {
        let manifest = declaring("");
        let releases: HashMap<Option<String>, Release> = HashMap::new();
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = all_of("").iter().map(PathBuf::from).collect();
        let directories = crate::check::testing::implied_directories(&present);
        let refused = vec![PathBuf::from("docs/a\nb.md"), PathBuf::from("docs/c\rd.md")];
        let inputs = Inputs {
            releases: &releases,
            pinned: "",
            committed: &committed,
            configs: &HashMap::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &refused,
        };
        let found = check(&Model::from_documents(Vec::new()), &manifest, &inputs).0;
        assert_eq!(found.len(), 2, "{found:#?}");
        for (finding, head) in found.iter().zip(["docs/a\\nb.md  ", "docs/c\\rd.md  "]) {
            assert!(finding.what.contains("holds a line break"), "{found:#?}");
            assert_eq!(finding.to_string().lines().count(), 2, "{finding}");
            assert!(finding.to_string().starts_with(head), "{finding}");
        }
    }

    #[test]
    fn a_register_directory_wearing_a_compiled_document_s_name_is_reported() {
        // `path@*@docs/rejected-alternatives.md` is a compiled document of every component; a
        // heading register at that directory name would make the same file its home, so
        // one path would carry two meanings.
        let manifest = declaring_full(
            "",
            "[registers.note]\nscope = \"component\"\nshape = \"heading\"\n\
             dir = \"rejected-alternatives\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert!(
            found
                .iter()
                .any(|f| f.contains("`rejected-alternatives`") && f.contains("compiled document")),
            "{found:#?}"
        );
    }
}
