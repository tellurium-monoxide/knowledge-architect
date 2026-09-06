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
//! `design@knowledge@ignored-targets-are-not-asserted` rests on.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::git::Entry;
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
    /// The files the walk refuses by name, per `walk::refused`. Read by nothing, in `outside`
    /// no more than in the model, and reported once each by the caller's checks.
    pub refused: Vec<PathBuf>,
    /// The listing's symlink and gitlink entries a manifest row does not keep, each read by
    /// nothing and reported once.
    pub links: Vec<Entry>,
}

pub fn survey(manifest: &Manifest, model: &Model) -> std::io::Result<Survey> {
    let root = manifest.root();
    Ok(from_listing(
        manifest,
        model,
        model.listing(),
        model.links(),
        |rel| std::fs::read_to_string(root.join(rel)).ok(),
    ))
}

/// The same, over a stated listing and a stated way of reading a file.
///
/// **A commit's tree is surveyed the same way a checkout is.** `commits` builds a model per
/// commit out of git objects, per `design@knowledge@a-commit-message-is-a-document`, so what exists
/// and what sits outside the walk are answered from that tree's listing and its blobs rather
/// than from the filesystem. `survey` is the case where the listing is the model's own and the
/// bytes are on disk.
pub fn from_listing(
    manifest: &Manifest,
    model: &Model,
    listing: &[PathBuf],
    links: &[Entry],
    read: impl Fn(&std::path::Path) -> Option<String>,
) -> Survey {
    let walk = manifest.walk();
    let covered: HashSet<&std::path::Path> =
        model.documents().iter().map(|d| d.rel.as_path()).collect();
    // A generated index is outside the walk by construction, and outside the inverse
    // assertion with it: its rows are a function of the tree rather than a claim anybody wrote.
    let generated = crate::index::generated_paths(manifest);
    let mut present = HashSet::new();
    let mut directories = HashSet::new();
    let mut outside = Vec::new();
    let mut refused = Vec::new();
    for rel in listing {
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
        // A name the walk refuses is a file no check reads, and one finding names it. It
        // stays in `present`: the refusal is about the file's contents, not its existence.
        // After the skips, so that a `skip-files` row is the declared way to keep one.
        if crate::walk::refused(rel) {
            refused.push(rel.clone());
            continue;
        }
        if let Some(text) = read(rel) {
            outside.push((rel.clone(), text));
        }
    }
    outside.sort();
    refused.sort();
    // A symlink or a gitlink a walk row covers is kept as declared, like any other file the
    // rows keep out; the rest are reported.
    let links: Vec<Entry> = links
        .iter()
        .filter(|e| !crate::walk::skipped(&e.rel, walk) && !walk.skip_files.contains(&e.rel))
        .cloned()
        .collect();
    Survey {
        present,
        directories,
        outside,
        refused,
        links,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Manifest;
    use std::path::Path;

    #[test]
    fn a_refused_name_is_present_and_neither_outside_nor_silent() {
        // Outside the walk it would be read by `check::uncovered`, whose finding would print
        // the name raw; in the model it would be a document. It is a third thing, listed for
        // one finding, and it still exists for every question about what is there.
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = []\n\
             skip-files = []\nexclude = []\n\n[lint]\nexempt-files = []\n\n[rules]\n\
             dir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\nversion = \"v\"\n\
             past = \"p\"\nmanifest = \"m\"\n",
        )
        .expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let listing: Vec<PathBuf> = ["notes/z\rq.md", "notes/a\nb.md", "notes/c.yml"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let survey = from_listing(&manifest, &model, &listing, &[], |_| {
            Some("text".to_string())
        });
        // In path order, whatever order git listed them in, as every listing here is.
        assert_eq!(
            survey.refused,
            vec![
                PathBuf::from("notes/a\nb.md"),
                PathBuf::from("notes/z\rq.md")
            ]
        );
        assert_eq!(
            survey
                .outside
                .iter()
                .map(|(p, _)| p.clone())
                .collect::<Vec<_>>(),
            vec![PathBuf::from("notes/c.yml")]
        );
        assert!(survey.present.contains(&PathBuf::from("notes/a\nb.md")));
        assert!(survey.directories.contains(&PathBuf::from("notes")));
    }

    #[test]
    fn a_refused_name_a_skip_files_row_declares_is_kept_and_reported_by_nothing() {
        // The declared way to keep such a file: the row takes it out of the walk like any
        // other, so it is neither refused nor outside. An ignore rule does the same by
        // keeping it out of the listing altogether.
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = [\"old\"]\n\
             skip-files = [\"notes/a\\nb.md\"]\nexclude = []\n\n[lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\nversion = \"v\"\n\
             past = \"p\"\nmanifest = \"m\"\n",
        )
        .expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let listing: Vec<PathBuf> = ["notes/a\nb.md", "old/c\nd.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let survey = from_listing(&manifest, &model, &listing, &[], |_| {
            Some("text".to_string())
        });
        assert!(survey.refused.is_empty(), "{:?}", survey.refused);
        assert!(survey.outside.is_empty());
        assert!(survey.present.contains(&PathBuf::from("notes/a\nb.md")));
    }
}
