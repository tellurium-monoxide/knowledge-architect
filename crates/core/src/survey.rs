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
//! `design@core@ignored-targets-are-not-asserted` rests on.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::git::Entry;
use crate::manifest::Manifest;
use crate::model::Model;

/// Every project-relative path git reports with its kind, and the files the walk does not
/// cover.
pub struct Survey {
    /// Every project-relative path that exists, files and directories together.
    pub present: HashSet<PathBuf>,
    /// The subset of `present` that is directories.
    ///
    /// Derived from the listing's own ancestors: git names files, and a directory exists
    /// exactly where a listed file sits under it. A directory holding no file at all cannot be
    /// committed, so nothing that git can report is missed.
    pub directories: HashSet<PathBuf>,
    /// The files the walk does not cover, each with what reading it gave.
    ///
    /// Every such file is listed, so the assertion that an unwalked file names nothing has no
    /// file it silently skips; the states are [`Outside`]'s, and the argument is
    /// `design@core@a-failed-parse-is-loud`.
    pub outside: Vec<(PathBuf, Outside)>,
    /// The files the walk refuses by name, per `walk::refused`. Read by nothing, in `outside`
    /// no more than in the model, and reported once each by the caller's checks.
    pub refused: Vec<PathBuf>,
    /// The listing's symlink and gitlink entries a manifest row does not keep, each read by
    /// nothing and reported once.
    pub links: Vec<Entry>,
}

/// What reading a file outside the walk gave.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outside {
    /// Its text, decoded lossily where its bytes are not UTF-8: the question asked of it is
    /// whether an ASCII token appears, and a lossy decoding keeps every ASCII byte.
    Text(String),
    /// A NUL byte in its first [`BINARY_PROBE`] bytes, git's own test for a binary file. It is
    /// handed to no check: its bytes are no prose anybody wrote a claim in, and a number in it,
    /// such as a PDF's page size, is data.
    Binary,
    /// Git lists it and the working tree does not hold it: a tracked file deleted and not
    /// staged.
    Missing,
    /// A directory where git lists one entry: a repository nested in this one, listed untracked,
    /// or a tracked file the working tree replaced by a directory.
    Directory,
    /// Its bytes could not be had, with the reason.
    Unreadable(String),
}

/// How many leading bytes the binary test reads, as git's own does.
pub const BINARY_PROBE: usize = 8000;

impl Outside {
    /// A file's state from its bytes: [`Outside::Binary`] or [`Outside::Text`].
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes[..bytes.len().min(BINARY_PROBE)].contains(&0) {
            Outside::Binary
        } else {
            Outside::Text(String::from_utf8_lossy(bytes).into_owned())
        }
    }

    /// The text an extension judges, where there is one.
    pub fn text(&self) -> Option<&str> {
        match self {
            Outside::Text(t) => Some(t),
            _ => None,
        }
    }
}

pub fn survey(manifest: &Manifest, model: &Model) -> std::io::Result<Survey> {
    let root = manifest.root();
    Ok(from_listing(
        manifest,
        model,
        model.listing(),
        model.links(),
        |rel| {
            let path = root.join(rel);
            match std::fs::metadata(&path) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Outside::Missing,
                Ok(m) if m.is_dir() => Outside::Directory,
                _ => match std::fs::read(&path) {
                    Ok(bytes) => Outside::from_bytes(&bytes),
                    Err(e) => Outside::Unreadable(e.to_string()),
                },
            }
        },
    ))
}

/// The same, over a stated listing and a stated way of reading a file.
///
/// **A commit's tree is surveyed the same way a checkout is.** `commits` builds a model per
/// commit out of git objects, per `design@core@a-commit-message-is-a-document`, so what exists
/// and what sits outside the walk are answered from that tree's listing and its blobs rather
/// than from the filesystem. `survey` is the case where the listing is the model's own and the
/// bytes are on disk.
pub fn from_listing(
    manifest: &Manifest,
    model: &Model,
    listing: &[PathBuf],
    links: &[Entry],
    read: impl Fn(&std::path::Path) -> Outside,
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
        // A symlink or a gitlink is no file to read: each is reported, or kept by a row, as a
        // link below, and reading or refusing one would report it a second time.
        if links.iter().any(|e| e.rel == *rel) {
            continue;
        }
        // A name the walk refuses is a file no check reads, and one finding names it. It
        // stays in `present`: the refusal is about the file's contents, not its existence.
        // After the skips, so that a `skip-files` row is the declared way to keep one.
        if crate::walk::refused(rel) {
            refused.push(rel.clone());
            continue;
        }
        outside.push((rel.clone(), read(rel)));
    }
    outside.sort_by(|a, b| a.0.cmp(&b.0));
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
    fn a_nul_byte_in_the_probed_prefix_makes_a_file_binary_and_one_past_it_does_not() {
        let mut bytes = vec![b'a'; BINARY_PROBE + 10];
        bytes[BINARY_PROBE - 1] = 0;
        assert_eq!(Outside::from_bytes(&bytes), Outside::Binary);
        bytes[BINARY_PROBE - 1] = b'a';
        bytes[BINARY_PROBE] = 0;
        assert!(matches!(Outside::from_bytes(&bytes), Outside::Text(_)));
    }

    #[test]
    fn a_refused_name_is_present_and_neither_outside_nor_silent() {
        // Outside the walk it would be read by `check::uncovered`, whose finding would print
        // the name raw; in the model it would be a document. It is a third thing, listed for
        // one finding, and it still exists for every question about what is there.
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = []\n\
             skip-files = []\nexclude = []\n",
        )
        .expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let listing: Vec<PathBuf> = ["notes/z\rq.md", "notes/a\nb.md", "notes/c.yml"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let survey = from_listing(&manifest, &model, &listing, &[], |_| {
            Outside::Text("text".to_string())
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
             skip-files = [\"notes/a\\nb.md\"]\nexclude = []\n",
        )
        .expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let listing: Vec<PathBuf> = ["notes/a\nb.md", "old/c\nd.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let survey = from_listing(&manifest, &model, &listing, &[], |_| {
            Outside::Text("text".to_string())
        });
        assert!(survey.refused.is_empty(), "{:?}", survey.refused);
        assert!(survey.outside.is_empty());
        assert!(survey.present.contains(&PathBuf::from("notes/a\nb.md")));
    }
}
