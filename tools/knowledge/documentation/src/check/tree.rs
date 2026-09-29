//! Phase 2: what the tree holds against what the manifest declares, and what the walk
//! could not read.
//!
//! Every finding here says that the model is incomplete: a file the walk could not read or
//! refused, an anchor or a register home that is not there, a home a walk row keeps out,
//! a declared path nothing answers to. A finding computed from the model afterwards — a
//! reference to a slug the unread file defined, an entry the missing home would have held —
//! is then unreliable in both directions, so `check::foundation` reports this phase and
//! judges nothing later. What this phase leaves standing, `check::registers` reads for shape.

use std::path::PathBuf;

use crate::entity::{Anchor, Anchors, Home};
use crate::finding::Finding;
use crate::git::EntryKind;
use crate::manifest::{Manifest, Register, Shape, COMPONENT_DOCUMENTS, MANIFEST_NAME};
use crate::model::Model;

use super::Inputs;

pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Vec<Finding> {
    let mut out = Vec::new();

    // A parse that could not be trusted must be LOUD. Silently it removes every citation in
    // the file from the walk while the run reports success — indistinguishable from a clean
    // file, and the guard the premortem named for a grammar that changes under an upgrade.
    for doc in model.documents() {
        if let Some(trouble) = &doc.parsed.trouble {
            out.push(Finding::at(
                &doc.rel,
                1,
                trouble.clone(),
                "nothing here is verified; fix the source, or what it says is checked by nothing",
            ));
        }
    }

    declarations(&mut out, manifest, inputs);

    let anchors = Anchors::of(manifest);
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
        // A location whose path is a file is `declarations`' finding, above; asserting its
        // homes as well would ask for a directory inside a file.
        if !anchor.is_root() && !inputs.directories.contains(&anchor.path) {
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
            let home = anchor.home_of(register);
            outside_the_walk(&mut out, manifest, anchor, register, &home);
            match register.shape {
                Shape::Heading => heading_home(&mut out, anchor, register, &home, inputs),
                Shape::File => file_home(&mut out, anchor, register, &home, inputs),
            }
        }
    }
    out
}

/// A heading register has exactly one home shape, and the directory shape has a head.
fn heading_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    inputs: &Inputs,
) {
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
            }
        }
    }
}

/// A file register has its directory, and not the retired single file.
fn file_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    inputs: &Inputs,
) {
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
    }
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
fn declarations(out: &mut Vec<Finding>, manifest: &Manifest, inputs: &Inputs) {
    for (name, decl) in manifest.locations() {
        if inputs.present.contains(&decl.path) && !inputs.directories.contains(&decl.path) {
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
    }

    // **Every path the manifest declares is checked to exist.** A row naming a deleted file is
    // silent in both directions: nobody is told it is dead, and a file later created at that
    // path inherits what the row grants: an extension's exemption list, for one, exempts a
    // document from whatever that extension checks without anyone deciding to.
    //
    // Existence, never file-ness: `skip-dirs` names directories, `exclude` names either, and
    // an archive directory is a declared skip that is legitimately empty in a fresh checkout.
    //
    // A location's own path is not in this list: it is an anchor, and the anchor loop below
    // reports a directory that is not there. Both would be one fact reported twice.
    let walk = manifest.walk();
    let mut declared: Vec<(&str, &Vec<PathBuf>)> = vec![
        ("[walk] skip-dirs", &walk.skip_dirs),
        ("[walk] skip-files", &walk.skip_files),
        ("[walk] exclude", &walk.exclude),
    ];
    // Each extension's declared paths, per
    // `design@knowledge@an-extension-plugs-in-through-phased-hooks`.
    declared.extend(
        manifest
            .extension_paths()
            .iter()
            .map(|(label, paths)| (label.as_str(), paths)),
    );
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

    // **A symlink and a gitlink are findings naming the entry.** The walk reads through
    // neither: a symlink read off the filesystem yields its target's bytes and read off a
    // commit's tree yields the target's path as text, and a checkout without symlinks holds
    // that text as a plain file, so two readers of one tree would judge two documents; a
    // gitlink names a commit of another repository the listing never descends into. A walk
    // row is the declared way to keep either.
    for entry in inputs.links {
        let (what, action) = match entry.kind {
            EntryKind::Symlink => (
                format!(
                    "`{}` is a symlink, and the walk reads no document through one",
                    entry.rel.display()
                ),
                "replace it with the file it points at, or with a reference to that file; a \
                 checkout without symlinks holds the target's path as text here, so two readers \
                 of this tree would judge two documents. Name it in [walk] skip-files to keep it",
            ),
            EntryKind::Gitlink => (
                format!(
                    "`{}` is a submodule, and the walk reads nothing under it",
                    entry.rel.display()
                ),
                "name it in [walk] exclude to declare the silence; the tool models no \
                 submodule, and the knowledge tool's open issues hold the question",
            ),
            EntryKind::File => continue,
        };
        out.push(Finding::in_file(&entry.rel, what, action));
    }

    // **A file outside the walk that could not be read is a finding naming it.** An extension
    // may assert that no unwalked file says something, and a file it cannot read is one it
    // cannot judge; the run stops here rather than pass it over.
    for (path, read) in inputs.outside {
        use crate::survey::Outside;
        let (what, action) = match read {
            Outside::Text(_) | Outside::Binary => continue,
            Outside::Missing => (
                "git lists this file and the working tree does not hold it".to_string(),
                "stage the deletion, or restore the file",
            ),
            Outside::Directory => (
                "git lists this path and the working tree holds a directory there, so no check \
                 reads anything under it: a repository nested in this one, or a tracked file \
                 replaced by a directory"
                    .to_string(),
                "for a nested repository, name it in an ignore rule or in [walk] skip-dirs to \
                 declare the silence; for a replaced file, stage the change or restore the file",
            ),
            Outside::Unreadable(why) => (
                format!(
                    "this file is outside the walk and could not be read, so no check judged \
                     what it says: {why}"
                ),
                "make it readable to the user running the check, or name it in [walk] \
                 skip-files if it is deliberately unchecked",
            ),
        };
        out.push(Finding::in_file(path, what, action));
    }

    // **A name the walk refuses is a finding naming the file**, per `walk::refused`. The file
    // was read by no check, so a rule quote in it is verified by nothing, and a reference is
    // one backticked span, so nothing can point at it either.
    for path in inputs.refused {
        let why = crate::walk::refusal(path).unwrap_or_else(|| "is refused".to_string());
        out.push(Finding::in_file(
            path,
            format!("this file's name {why}, and no check read it"),
            "rename the file; a name with a line break in it fits on no output line and in no \
             reference, and Windows creates no file under a name it forbids. Name it in [walk] \
             skip-files or in an ignore rule to keep it as it is",
        ));
    }
}
