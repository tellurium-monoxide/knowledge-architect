//! What is on disk, gathered once by the caller so that no check has to look.
//!
//! A check is a pure function from the model to findings. Two of them still need facts about
//! the filesystem — whether a path a document names exists, and which files no checker reads
//! — and those facts are collected here and handed in.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::manifest::Manifest;
use crate::model::Model;

/// Every project-relative path that exists with its kind, and the readable files the walk
/// does not cover.
pub struct Survey {
    /// Every project-relative path that exists, files and directories together.
    pub present: HashSet<PathBuf>,
    /// The subset of `present` that is directories.
    ///
    /// Recorded rather than inferred: a directory wearing a document's name has entries
    /// beneath it in every committable case, but the inference cannot tell an empty directory
    /// from a file, and a reference's trailing-slash claim needs the kind to be a fact.
    pub directories: HashSet<PathBuf>,
    /// The readable files the walk does not cover, with their text.
    pub outside: Vec<(PathBuf, String)>,
}

pub fn survey(manifest: &Manifest, model: &Model) -> std::io::Result<Survey> {
    let root = manifest.root();
    let walk = manifest.walk();
    let covered: HashSet<&std::path::Path> =
        model.documents().iter().map(|d| d.rel.as_path()).collect();
    let mut present = HashSet::new();
    let mut directories = HashSet::new();
    let mut outside = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            let Ok(rel) = path.strip_prefix(root) else {
                continue;
            };
            let rel = rel.to_path_buf();
            let name = rel
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            // `present` is a COMPLETE listing, because the checks it serves ask whether a path
            // exists — one a document names, or one the manifest declares. A document may
            // legitimately point into a directory the walk skips, such as the archive or the
            // frozen survey, and the manifest declares `.git` as a skip. So the insert comes
            // BEFORE the descent is refused: `.git` exists, and a listing that omitted it
            // reported the manifest's own declaration of it as dead.
            present.insert(rel.clone());
            if path.is_dir() {
                directories.insert(rel.clone());
            }
            if name == ".git" {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            // `outside` asks the opposite question: files of THIS project that no checker
            // reads, and which may therefore not name a rule. A skipped directory, a skipped
            // filename and an excluded path are all left out — the first two because they are
            // deliberately unchecked and covered elsewhere, the third because it is another
            // project entirely, whose fixtures are not this project's unverified claims.
            //
            // What git does not track is not this project's file at all: it is build output or
            // a directory made on demand, and a rule number inside one is not a claim anybody
            // wrote. `present` still carries it, because a document may legitimately point at
            // a generated path and `paths` has to resolve it.
            let skipped = walk.skip_dirs.iter().any(|d| rel.starts_with(d))
                || walk.skip_files.contains(&rel)
                || walk.exclude.iter().any(|e| rel.starts_with(e))
                || manifest.ignore().covers(&rel, false);
            if !skipped && !covered.contains(rel.as_path()) {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    outside.push((rel, text));
                }
            }
        }
    }
    outside.sort();
    Ok(Survey {
        present,
        directories,
        outside,
    })
}
