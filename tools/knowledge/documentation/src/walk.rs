//! Which files are read.
//!
//! What a line MEANS is `source`'s: the grammar for the file's kind says which byte ranges
//! are prose, so nothing here strips a prefix off anything.

use std::path::{Component, Path, PathBuf};

use crate::gitignore::Ignore;
use crate::manifest::Walk;

/// Every file in the repository whose contents are checked.
///
/// A hand-maintained list cannot survive a source tree, so the set is a walk with an
/// exclusion set rather than an inclusion one: a new document is covered the moment it
/// exists, and a file that should be exempt has to say so here, in one place, with a reason.
pub fn live_files(root: &Path, walk: &Walk, ignore: &Ignore) -> std::io::Result<Vec<PathBuf>> {
    let excluded: Vec<PathBuf> = walk.exclude.iter().map(|p| root.join(p)).collect();
    let mut out = Vec::new();
    collect(root, root, walk, ignore, &excluded, &mut out)?;
    // Sorted by path COMPONENT, not by the path as one string. They disagree whenever one
    // directory name is a prefix of another — a/b against a-c/d, where `-` sorts before
    // `/` — and component order is what the walk being replaced produced.
    out.sort_by(|a, b| components(a).cmp(&components(b)));
    Ok(out)
}

fn components(path: &Path) -> Vec<&std::ffi::OsStr> {
    path.components().map(Component::as_os_str).collect()
}

fn collect(
    root: &Path,
    dir: &Path,
    walk: &Walk,
    ignore: &Ignore,
    excluded: &[PathBuf],
    out: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.file_name().and_then(|n| n.to_str()).is_none() {
            continue;
        }
        if excluded.contains(&path) {
            continue;
        }
        let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
        // What git does not track, this tool does not read, and the manifest does not have to
        // say so. A generated directory cannot be declared and checked to exist: a fresh clone
        // has none of them.
        if ignore.covers(&rel, path.is_dir()) {
            continue;
        }
        if path.is_dir() {
            if !walk.skip_dirs.contains(&rel) {
                collect(root, &path, walk, ignore, excluded, out)?;
            }
        } else if is_live(&path, &rel, walk) {
            out.push(path);
        }
    }
    Ok(())
}

/// The file kinds this tool can parse, and therefore the whole of the walk.
///
/// **Compiled in rather than declared.** Which suffixes a project holds is a fact about that
/// project; which of them this tool knows how to parse is a fact about this tool, and only
/// the second decides what may be walked. A project free to declare its own set would be
/// conformant with whatever it declared — `suffixes = []` passes every citation check — which
/// is the same argument `##components-carry-the-same-documents` makes about the document set.
///
/// Nothing is left unchecked by narrowing it. `check::uncovered` asserts the inverse, that a
/// file outside the walk may not name a rule, so a citation written in a manifest or a script
/// is a finding rather than a silence.
pub const LIVE_SUFFIXES: [&str; 2] = ["md", "rs"];

fn is_live(path: &Path, rel: &Path, walk: &Walk) -> bool {
    !walk.skip_files.iter().any(|f| f == rel)
        && path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| LIVE_SUFFIXES.contains(&e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Walk;

    #[test]
    fn a_skipped_file_is_named_by_its_path_and_not_by_its_basename() {
        // A bare name matched anywhere, so exempting the generated index exempted every file
        // called `index.md` — from the walk and from the inverse assertion both.
        let walk = Walk {
            skip_files: vec![PathBuf::from("docs/rules/index.md")],
            ..Walk::sample()
        };
        let declared = PathBuf::from("docs/rules/index.md");
        assert!(!is_live(&declared, &declared, &walk));
        let elsewhere = PathBuf::from("docs/plans/index.md");
        assert!(
            is_live(&elsewhere, &elsewhere, &walk),
            "same name, other path"
        );
    }

    #[test]
    fn the_walk_covers_markdown_and_rust_only() {
        assert_eq!(LIVE_SUFFIXES, ["md", "rs"]);
        let walk = Walk::sample();
        assert!(is_live(Path::new("a/b.md"), Path::new("a/b.md"), &walk));
        assert!(is_live(Path::new("a/b.rs"), Path::new("a/b.rs"), &walk));
        for suffix in ["py", "sh", "toml", "yml", "json", "txt"] {
            let p = PathBuf::from(format!("a/b.{suffix}"));
            assert!(!is_live(&p, &p, &walk), "{suffix} must not be walked");
        }
    }
}
