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
//! of `knowledge#design-home-two-shapes`: the single file, or the directory whose `README.md`
//! links every subdocument. A file register has one home, a directory holding one file per
//! entry beside a hand-written `README.md`, a generated `index.md` and an optional
//! `register.toml`; the entries' frontmatter, title, sections and owed subsections are asserted
//! here, per `knowledge#a-file-register-is-a-directory-of-entries`.
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
use crate::manifest::{Manifest, Register, Shape, COMPONENT_DOCUMENTS, MANIFEST_NAME};
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
        for name in &anchor.registers {
            let Some(register) = anchors.registers().by_name(name) else {
                continue;
            };
            counts.instances += 1;
            let home = anchor.home_of(register);
            match register.shape {
                Shape::Heading => {
                    heading_home(&mut out, anchor, register, &home, model, anchors, inputs)
                }
                Shape::File => {
                    counts.entries += file_home(&mut out, anchor, register, &home, model, inputs);
                }
            }
        }
    }

    (out, counts)
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

    for (name, decl) in manifest.locations() {
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
        // A component inside a location would keep its homes under its own `docs/` while the
        // location keeps a second set beside them, so a document could sit in two homes at
        // once and a decision would have two candidate places to land.
        for component in manifest.components().all() {
            if !component.is_root() && component.path.starts_with(&decl.path) {
                out.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!(
                        "the component `{}` sits inside the location `{name}`",
                        component.name
                    ),
                    "move one out of the other; a location carries a subset of the registers \
                     and a component carries them all, so nesting gives one document two homes",
                ));
            }
        }
    }

    // **Every path the manifest declares is checked to exist.** A row naming a deleted file is
    // silent in both directions: nobody is told it is dead, and a file later created at that
    // path inherits what the row grants. That matters most for `exempt-files`, which
    // `check::regime::run` reads as well as the lint, so a stale row there can exempt a
    // document from the whole citation regime without anyone deciding to.
    //
    // Existence, never file-ness: `skip-dirs` names directories, `exclude` names either, and
    // an archive directory is a declared skip that is legitimately empty in a fresh checkout.
    let walk = manifest.walk();
    let locations: Vec<PathBuf> = manifest
        .locations()
        .values()
        .map(|l| l.path.clone())
        .collect();
    let declared: [(&str, &Vec<PathBuf>); 5] = [
        ("[walk] skip-dirs", &walk.skip_dirs),
        ("[walk] skip-files", &walk.skip_files),
        ("[walk] exclude", &walk.exclude),
        ("[lint] exempt-files", &manifest.lint().exempt_files),
        ("[locations] path", &locations),
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
        // The index's CONTENT is generated and the `generated` family owns it. That it is
        // there at all is a fact about the register's shape, and it is mandatory: an index
        // nobody generates is a listing that silently stops listing.
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
    directory_contents(out, register, home, &groups, inputs);

    let mut judged = 0;
    for doc in model.documents() {
        if !is_entry(&doc.rel, home) {
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
fn directory_contents(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    groups: &BTreeSet<String>,
    inputs: &Inputs,
) {
    let mut present: BTreeSet<String> = BTreeSet::new();
    let mut paths: Vec<&PathBuf> = inputs
        .present
        .iter()
        .filter(|p| p.starts_with(&home.dir) && **p != home.dir)
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
        in_order(
            out,
            doc,
            register,
            3,
            owed,
            &headings,
            "level-three subsection",
        );
    }
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
    // its own source, per `knowledge#checker-source-literals-are-data`.

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
        // Its path is a declared path and is checked to exist.
        let found = findings(&manifest, &all_of(""));
        assert!(
            found.iter().any(|f| f.contains("[locations] path")),
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
}
