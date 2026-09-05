//! What the project holds, gathered once by the caller so that no check has to look.
//!
//! A check is a pure function from the model to findings. Two of them still need facts about
//! the tree — whether a path a document names exists, and which files no checker reads — and
//! those facts are derived here from git's live listing and handed in.
//!
//! **The listing is git's, not a tree walk.** What exists, for every question a check asks, is
//! what git tracks plus what it neither tracks nor ignores. Build output and on-demand
//! directories are therefore absent whether or not the checking machine has built anything, so
//! the same commit surveys the same on a fresh clone and on a working tree — which is what
//! `knowledge#ignored-targets-are-not-asserted` rests on.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::manifest::Manifest;
use crate::model::Model;

/// Every project-relative path git reports with its kind, and the readable files the walk
/// does not cover.
pub struct Survey {
    /// Every project-relative path that exists, files and directories together.
    pub present: HashSet<PathBuf>,
    /// The subset of `present` that is directories.
    ///
    /// Derived from the listing's own ancestors: git names files, and a directory exists
    /// exactly where a listed file sits under it. A directory holding no file at all cannot be
    /// committed, so nothing that git can report is missed.
    pub directories: HashSet<PathBuf>,
    /// The readable files the walk does not cover, with their text.
    pub outside: Vec<(PathBuf, String)>,
}

pub fn survey(manifest: &Manifest, model: &Model) -> std::io::Result<Survey> {
    let root = manifest.root();
    let walk = manifest.walk();
    let covered: HashSet<&std::path::Path> =
        model.documents().iter().map(|d| d.rel.as_path()).collect();
    // A generated index is outside the walk by construction, and outside the inverse
    // assertion with it: its rows are a function of the tree rather than a claim anybody wrote.
    let generated = crate::index::generated_index_paths(manifest);
    let mut present = HashSet::new();
    let mut directories = HashSet::new();
    let mut outside = Vec::new();
    for rel in model.listing() {
        present.insert(rel.clone());
        for ancestor in rel.ancestors().skip(1) {
            if ancestor.as_os_str().is_empty() {
                continue;
            }
            present.insert(ancestor.to_path_buf());
            directories.insert(ancestor.to_path_buf());
        }
        if covered.contains(rel.as_path()) {
            continue;
        }
        // `outside` asks the opposite question to `present`: files of THIS project that no
        // checker reads, and which may therefore not name a rule. A skipped directory, a
        // skipped filename and an excluded path are all left out — the first two because they
        // are deliberately unchecked and covered elsewhere, the third because it is another
        // project entirely, whose fixtures are not this project's unverified claims.
        let skipped = crate::walk::skipped(rel, walk)
            || walk.skip_files.contains(rel)
            || generated.contains(rel);
        if skipped {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(root.join(rel)) {
            outside.push((rel.clone(), text));
        }
    }
    outside.sort();
    Ok(Survey {
        present,
        directories,
        outside,
    })
}
