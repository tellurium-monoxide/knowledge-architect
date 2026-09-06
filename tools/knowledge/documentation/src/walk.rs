//! Which files are read.
//!
//! **The listing is git's**, and this module applies the manifest's exclusions to it. What a
//! line MEANS is `source`'s: the grammar for the file's kind says which byte ranges are prose,
//! so nothing here strips a prefix off anything.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::manifest::Walk;

/// Every file in the repository whose contents are checked.
///
/// A hand-maintained list cannot survive a source tree, so the set is a listing with an
/// exclusion set rather than an inclusion one: a new document is covered the moment it
/// exists, and a file that should be exempt has to say so here, in one place, with a reason.
///
/// `listing` is git's live listing, project-relative: every tracked file, plus every untracked
/// file the ignore rules do not cover. A tracked file is in it whatever the ignore rules say,
/// per `design@knowledge@git-supplies-the-walk`, so no ignore line can remove a live document from the
/// walk. What this function removes on top of that is the manifest's `skip-dirs`, `skip-files`
/// and `exclude`, the suffixes the tool cannot parse, and the generated indexes.
///
/// `generated` is every path the tool writes a generated index at, taken from the register
/// instances. Those are **outside the walk by construction** rather than by a declared row: a
/// generated file is not a source of citations, and deriving the set from the instances is what
/// keeps a new instance from arriving with its index inside the walk.
pub fn live_files(
    root: &Path,
    walk: &Walk,
    listing: &[PathBuf],
    generated: &HashSet<PathBuf>,
) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = listing
        .iter()
        .filter(|rel| {
            !skipped(rel, walk) && is_live(rel, walk) && !generated.contains(*rel) && !refused(rel)
        })
        .map(|rel| root.join(rel))
        .collect();
    // Sorted by path COMPONENT, not by the path as one string, which `Path`'s own ordering is.
    // The two disagree whenever one directory name is a prefix of another — a/b against
    // a-c/d, where `-` sorts before `/` — and component order is what the tree walk this
    // replaced produced and what the model dump is read in.
    out.sort();
    out
}

/// Whether the walk refuses this path for its name alone: a line break in it.
///
/// A finding is one line opening with its path, an index row is one line, and a reference is
/// one backticked span, so a name holding a newline or a carriage return can be printed by
/// nothing here and pointed at by nothing. Windows refuses to create such a file at all, so a
/// tree holding one cannot be checked out there. The file is not read, and the caller reports
/// it once by name; `skip-files` or an ignore rule is how a project keeps one deliberately.
pub fn refused(rel: &Path) -> bool {
    rel.as_os_str()
        .as_encoded_bytes()
        .iter()
        .any(|b| *b == b'\n' || *b == b'\r')
}

/// Whether a skipped directory or an excluded location holds this path.
///
/// A directory row covers everything beneath it, which is what pruning at the directory used to
/// do; git lists files and not directories, so the containment is asked here instead.
pub fn skipped(rel: &Path, walk: &Walk) -> bool {
    walk.skip_dirs.iter().any(|d| rel.starts_with(d))
        || walk.exclude.iter().any(|e| rel.starts_with(e))
}

/// The file kinds this tool can parse, and therefore the whole of the walk.
///
/// **Compiled in rather than declared.** Which suffixes a project holds is a fact about that
/// project; which of them this tool knows how to parse is a fact about this tool, and only
/// the second decides what may be walked. A project free to declare its own set would be
/// conformant with whatever it declared — `suffixes = []` passes every citation check — which
/// is the same argument `design@knowledge@components-carry-the-same-documents` makes about the document set.
///
/// Nothing is left unchecked by narrowing it. `check::uncovered` asserts the inverse, that a
/// file outside the walk may not name a rule, so a citation written in a manifest or a script
/// is a finding rather than a silence.
pub const LIVE_SUFFIXES: [&str; 2] = ["md", "rs"];

fn is_live(rel: &Path, walk: &Walk) -> bool {
    !walk.skip_files.iter().any(|f| f == rel)
        && rel
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
        assert!(!is_live(&declared, &walk));
        let elsewhere = PathBuf::from("docs/plans/index.md");
        assert!(is_live(&elsewhere, &walk), "same name, other path");
    }

    #[test]
    fn a_name_holding_a_line_break_is_refused_and_leaves_the_walk() {
        // Both bytes that end a line for some reader of the output; a tab or a space is
        // awkward and is not refused, because it breaks no line.
        assert!(refused(Path::new("docs/a\nb.md")));
        assert!(refused(Path::new("docs/a\rb.md")));
        assert!(refused(Path::new("do\ncs/b.md")), "in a directory name too");
        assert!(!refused(Path::new("docs/a b.md")));
        assert!(!refused(Path::new("docs/a\tb.md")));
        let listing: Vec<PathBuf> = ["docs/a\nb.md", "docs/b.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let walked = live_files(
            Path::new("/root"),
            &Walk::sample(),
            &listing,
            &HashSet::new(),
        );
        assert_eq!(walked, vec![PathBuf::from("/root/docs/b.md")]);
    }

    #[test]
    fn the_walk_covers_markdown_and_rust_only() {
        assert_eq!(LIVE_SUFFIXES, ["md", "rs"]);
        let walk = Walk::sample();
        assert!(is_live(Path::new("a/b.md"), &walk));
        assert!(is_live(Path::new("a/b.rs"), &walk));
        for suffix in ["py", "sh", "toml", "yml", "json", "txt"] {
            let p = PathBuf::from(format!("a/b.{suffix}"));
            assert!(!is_live(&p, &walk), "{suffix} must not be walked");
        }
    }

    #[test]
    fn the_order_is_by_path_component_and_not_by_the_path_as_one_string() {
        // The two disagree whenever one directory name is a prefix of another. By component
        // the third segment decides — `a` before `a-c` before `a.md`; as one string the
        // separators decide instead — a-c/d.md before a.md before a/b.md. Every caller
        // that reads the walk in order, the model dump above all, depends on which. `Path`
        // orders by component, so this guards the property rather than one comparator.
        let listing: Vec<PathBuf> = ["a-c/d.md", "a/b.md", "a.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let walked = live_files(
            Path::new("/root"),
            &Walk::sample(),
            &listing,
            &HashSet::new(),
        );
        assert_eq!(
            walked,
            vec![
                PathBuf::from("/root/a/b.md"),
                PathBuf::from("/root/a-c/d.md"),
                PathBuf::from("/root/a.md"),
            ]
        );
    }

    #[test]
    fn the_listing_is_filtered_by_the_manifest_and_by_the_generated_set() {
        // Git lists files and never directories, so a `skip-dirs` or an `exclude` row has to
        // be read as containment here. A row applied as an equality would leave every file
        // under a skipped directory inside the walk.
        let walk = Walk {
            skip_dirs: vec![PathBuf::from("docs/past")],
            skip_files: vec![PathBuf::from("docs/rules/index.md")],
            exclude: vec![PathBuf::from("tests/projects")],
        };
        let listing: Vec<PathBuf> = [
            "README.md",
            "docs/past/old.md",
            "docs/rules/index.md",
            "docs/open-issues/index.md",
            "tests/projects/minimal/README.md",
            "src/lib.rs",
            "Cargo.toml",
        ]
        .iter()
        .map(PathBuf::from)
        .collect();
        let generated: HashSet<PathBuf> = [PathBuf::from("docs/open-issues/index.md")]
            .into_iter()
            .collect();
        let walked = live_files(Path::new("/root"), &walk, &listing, &generated);
        assert_eq!(
            walked,
            vec![
                PathBuf::from("/root/README.md"),
                PathBuf::from("/root/src/lib.rs"),
            ],
            "the listing minus every exclusion, as absolute paths"
        );
    }
}
