//! Which files are read, and what a line looks like once its comment leader is gone.

use std::path::{Component, Path, PathBuf};

use crate::manifest::Walk;

/// Every file in the repository whose contents are checked.
///
/// A hand-maintained list cannot survive a source tree, so the set is a walk with an
/// exclusion set rather than an inclusion one: a new document is covered the moment it
/// exists, and a file that should be exempt has to say so here, in one place, with a reason.
pub fn live_files(root: &Path, walk: &Walk) -> std::io::Result<Vec<PathBuf>> {
    let excluded: Vec<PathBuf> = walk.exclude.iter().map(|p| root.join(p)).collect();
    let mut out = Vec::new();
    collect(root, walk, &excluded, &mut out)?;
    // Sorted by path COMPONENT, not by the path as one string. They disagree whenever one
    // directory name is a prefix of another — `a/b` against `a-c/d`, where `-` sorts before
    // `/` — and component order is what the walk being replaced produced.
    out.sort_by(|a, b| components(a).cmp(&components(b)));
    Ok(out)
}

fn components(path: &Path) -> Vec<&std::ffi::OsStr> {
    path.components().map(Component::as_os_str).collect()
}

fn collect(
    dir: &Path,
    walk: &Walk,
    excluded: &[PathBuf],
    out: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if excluded.contains(&path) {
            continue;
        }
        if path.is_dir() {
            if !walk.skip_dirs.iter().any(|d| d == name) {
                collect(&path, walk, excluded, out)?;
            }
        } else if is_live(&path, name, walk) {
            out.push(path);
        }
    }
    Ok(())
}

fn is_live(path: &Path, name: &str, walk: &Walk) -> bool {
    !walk.skip_files.iter().any(|f| f == name)
        && path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| walk.suffixes.iter().any(|s| s == e))
}

/// Whether a file's lines carry comment leaders to strip.
///
/// Scoped by suffix, not by content. `#` opens a comment in Python, shell and TOML and opens
/// a heading in Markdown; Markdown has no comment syntax at all, so there is nothing to strip
/// there and stripping anyway would corrupt the one file kind the quote conventions were
/// written for.
pub fn has_comments(path: &Path, walk: &Walk) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e != "md" && walk.suffixes.iter().any(|s| s == e))
}

/// One comment leader, removed from the front of a line.
///
/// The quote conventions are Markdown's and the walk covers source files, so a quote in a
/// comment arrives wearing a leader the conventions know nothing about: a doc-comment line
/// opening with `>` is not a blockquote, because a blockquote starts at the `>` and that one
/// starts at the `///`.
///
/// Indentation goes **with** the leader: a blockquote is recognised at the start of a line
/// and a comment inside a function is indented, so keeping the indentation would leave every
/// such blockquote invisible.
///
/// A leading `*` is a leader only when a space or the line end follows it. `*"…"*` is the
/// inline-quote convention, and stripping its opening delimiter would destroy the very quote
/// this exists to reach.
pub fn strip_leader(line: &str) -> &str {
    let body = line.trim_start_matches([' ', '\t']);
    // Ordered longest-first, matching the alternation order of the pattern this replaces:
    // `///` and `//!` before `//`, and `*/` before a continuation `*`.
    for leader in ["///", "//!", "//", "/**", "/*", "*/", "#"] {
        if let Some(rest) = body.strip_prefix(leader) {
            return rest.strip_prefix([' ', '\t']).unwrap_or(rest);
        }
    }
    if let Some(rest) = body.strip_prefix('*') {
        if rest.is_empty() || rest.starts_with([' ', '\t']) {
            return rest.strip_prefix([' ', '\t']).unwrap_or(rest);
        }
    }
    body
}

/// A file's text with one comment leader removed per line.
///
/// Line count and line order are preserved, so a finding that reports a line number still
/// reports the right one. A line carrying no leader is left exactly as it is — that line is
/// code, not prose.
pub fn strip_leaders(text: &str) -> String {
    let mut out = text
        .lines()
        .map(strip_leader)
        .collect::<Vec<_>>()
        .join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Walk;

    #[test]
    fn rust_doc_leaders_come_off_with_their_indentation() {
        assert_eq!(strip_leader("    /// > a blockquote"), "> a blockquote");
        assert_eq!(strip_leader("//! module doc"), "module doc");
        assert_eq!(strip_leader("// plain"), "plain");
    }

    #[test]
    fn a_block_comment_continuation_is_a_leader_but_an_italic_quote_is_not() {
        assert_eq!(strip_leader(" * continued"), "continued");
        assert_eq!(strip_leader(" *"), "");
        assert_eq!(strip_leader(" */"), "");
        // The inline-quote convention opens with `*"`. Stripping that `*` would destroy the
        // quote the whole walk exists to reach.
        assert_eq!(strip_leader(r#" *"quoted"*"#), r#"*"quoted"*"#);
    }

    #[test]
    fn a_hash_leader_comes_off_and_a_line_of_code_does_not() {
        assert_eq!(strip_leader("# a python comment"), "a python comment");
        assert_eq!(
            strip_leader("value = 1  # trailing"),
            "value = 1  # trailing"
        );
    }

    #[test]
    fn stripping_preserves_the_line_count() {
        let text = "// one\ncode();\n/// three\n";
        assert_eq!(strip_leaders(text), "one\ncode();\nthree\n");
        assert_eq!(strip_leaders(text).lines().count(), text.lines().count());
    }

    #[test]
    fn markdown_has_no_leaders_to_strip() {
        let walk = Walk::sample();
        assert!(!has_comments(Path::new("a/b.md"), &walk));
        for suffix in ["rs", "py", "sh", "toml"] {
            assert!(has_comments(&PathBuf::from(format!("a/b.{suffix}")), &walk));
        }
    }
}
