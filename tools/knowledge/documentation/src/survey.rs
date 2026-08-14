//! What is on disk, gathered once by the caller so that no check has to look.
//!
//! A check is a pure function from the model to findings. Two of them still need facts about
//! the filesystem — whether a path a document names exists, and which files no checker reads
//! — and those facts are collected here and handed in.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::manifest::Manifest;
use crate::model::Model;

/// Every project-relative path that exists, and the readable files the walk does not cover.
pub type Survey = (HashSet<PathBuf>, Vec<(PathBuf, String)>);

pub fn survey(manifest: &Manifest, model: &Model) -> std::io::Result<Survey> {
    let root = manifest.root();
    let walk = manifest.walk();
    let covered: HashSet<&std::path::Path> =
        model.documents().iter().map(|d| d.rel.as_path()).collect();
    let mut present = HashSet::new();
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
            if name == ".git" {
                continue;
            }
            // `present` is a COMPLETE listing, because the check it serves asks whether a path
            // a document names exists — and a document may legitimately point into a directory
            // the walk skips, such as the archive or the frozen survey.
            present.insert(rel.clone());
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            // `outside` asks the opposite question: files of THIS project that no checker
            // reads, and which may therefore not name a rule. A skipped directory, a skipped
            // filename and an excluded path are all left out — the first two because they are
            // deliberately unchecked and covered elsewhere, the third because it is another
            // project entirely, whose fixtures are not this project's unverified claims.
            let skipped = walk.skip_dirs.iter().any(|d| rel.starts_with(d))
                || walk.skip_files.contains(&rel)
                || walk.exclude.iter().any(|e| rel.starts_with(e));
            if !skipped && !covered.contains(rel.as_path()) {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    outside.push((rel, text));
                }
            }
        }
    }
    outside.sort();
    Ok((present, outside))
}
