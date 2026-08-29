//! What git does not track, and therefore what this tool does not read.
//!
//! **A project declares what is checked, not what is generated.** Build output and on-demand
//! directories are already named in `.gitignore`, and naming them a second time in
//! `knowledge.toml` puts two lists in the tree that drift apart — and the manifest's copy
//! cannot be checked to exist, because the whole point of those paths is that a fresh clone
//! has none of them. So the walk reads `.gitignore` and every remaining manifest declaration
//! is a path git tracks, which CAN be checked to exist.
//!
//! **Only the root `.gitignore` is read.** A nested one is not honoured, so a project that
//! needs one has to declare the path in `knowledge.toml` instead. This is a subset of git's
//! behaviour by choice: the alternative is reimplementing git's precedence rules, and getting
//! that subtly wrong would silently remove files from the walk, which is the failure class
//! this tool exists to prevent.
//!
//! **An unsupported pattern is an error, never a guess.** `##a-failed-parse-is-loud`: a
//! pattern this matcher cannot honour would otherwise be dropped, and dropping an ignore rule
//! makes the walk read more than it should while dropping a negation makes it read less. Both
//! are silent. The supported subset is what this repository's own file uses.

use std::path::Path;

/// One `.gitignore` line, reduced to what it matches.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Pattern {
    /// The pattern with its anchors stripped, split on `/`.
    segments: Vec<String>,
    /// It came with a leading `/`, or held a `/` of its own, so it matches from the root only.
    anchored: bool,
    /// It came with a trailing `/`, so it matches a directory and never a file.
    directory_only: bool,
}

/// The root `.gitignore`, parsed.
#[derive(Clone, Debug, Default)]
pub struct Ignore {
    patterns: Vec<Pattern>,
}

impl Ignore {
    /// Parse a `.gitignore`, or say which line cannot be honoured.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut patterns = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // Each of these changes which files are read, and a matcher that dropped one
            // would be wrong in a direction nothing reports.
            if let Some(what) = unsupported(line) {
                return Err(format!(
                    ".gitignore line {}: {what} is not supported by this matcher: {line}\n  \
                     declare the path in knowledge.toml [walk] instead, or simplify the pattern",
                    i + 1
                ));
            }
            let directory_only = line.ends_with('/');
            let trimmed = line.trim_end_matches('/');
            let anchored =
                trimmed.starts_with('/') || trimmed.trim_start_matches('/').contains('/');
            let body = trimmed.trim_start_matches('/');
            if body.is_empty() {
                continue;
            }
            patterns.push(Pattern {
                segments: body.split('/').map(str::to_string).collect(),
                anchored,
                directory_only,
            });
        }
        Ok(Self { patterns })
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Whether git ignores this project-relative path.
    ///
    /// `is_dir` decides whether a `dir/` pattern applies. A path UNDER an ignored directory is
    /// ignored too, which is what makes pruning the walk at the directory correct.
    pub fn covers(&self, rel: &Path, is_dir: bool) -> bool {
        let parts: Vec<String> = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();
        if parts.is_empty() {
            return false;
        }
        self.patterns.iter().any(|p| matches(p, &parts, is_dir))
    }
}

/// Whether one pattern matches one path.
fn matches(pattern: &Pattern, parts: &[String], is_dir: bool) -> bool {
    let n = pattern.segments.len();
    // An anchored pattern starts at the root; an unanchored one matches at any depth. Either
    // way it may match a PREFIX of the path, because everything under an ignored directory is
    // ignored — so the trailing-slash test applies to the matched prefix and not to the path.
    let starts: Vec<usize> = if pattern.anchored {
        vec![0]
    } else {
        (0..parts.len()).collect()
    };
    for start in starts {
        if start + n > parts.len() {
            continue;
        }
        if !pattern
            .segments
            .iter()
            .zip(&parts[start..start + n])
            .all(|(pat, part)| segment_matches(pat, part))
        {
            continue;
        }
        // The prefix ends the path only when it consumed all of it. Short of that, what
        // matched is a directory, so a `dir/` pattern is satisfied.
        let consumed_whole_path = start + n == parts.len();
        if pattern.directory_only && consumed_whole_path && !is_dir {
            continue;
        }
        return true;
    }
    false
}

/// One path segment against one pattern segment, where `*` matches within the segment.
fn segment_matches(pattern: &str, part: &str) -> bool {
    let Some((head, rest)) = pattern.split_once('*') else {
        return pattern == part;
    };
    if !part.starts_with(head) {
        return false;
    }
    let mut at = head.len();
    let mut pieces = rest.split('*').peekable();
    while let Some(piece) = pieces.next() {
        if piece.is_empty() {
            continue;
        }
        let last = pieces.peek().is_none();
        if last && rest.ends_with(piece) {
            // The final literal must land at the end, or `*.pyc` would accept `a.pyc.bak`.
            return part.len() >= at + piece.len() && part[at..].ends_with(piece);
        }
        match part[at..].find(piece) {
            Some(i) => at += i + piece.len(),
            None => return false,
        }
    }
    true
}

/// What in this line the matcher cannot honour, if anything.
fn unsupported(line: &str) -> Option<&'static str> {
    if line.starts_with('!') {
        return Some("a negation");
    }
    if line.contains("**") {
        return Some("a `**` wildcard");
    }
    if line.contains('?') {
        return Some("a `?` wildcard");
    }
    if line.contains('[') {
        return Some("a character class");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This repository's own file, which is what the supported subset was chosen against.
    const REAL: &str = concat!(
        "# Build trees\n",
        "/build/\n",
        "\n",
        "# Python bytecode\n",
        "__pycache__/\n",
        "*.pyc\n",
        "target/\n",
        "\n",
        "\n",
        ".claude/worktrees\n",
    );

    fn ignore() -> Ignore {
        Ignore::parse(REAL).expect("the real file parses")
    }

    #[test]
    fn the_two_paths_the_manifest_used_to_declare_are_covered() {
        // The whole point: `target` and `thaum@.claude/worktrees/` come out of `knowledge.toml`
        // because this file already names them, and neither can be checked to exist.
        let i = ignore();
        assert!(i.covers(Path::new("target"), true));
        assert!(i.covers(Path::new("target/debug/knowledge"), false));
        assert!(i.covers(Path::new(".claude/worktrees"), true));
        assert!(i.covers(Path::new(".claude/worktrees/mig-scan/CLAUDE.md"), false));
    }

    #[test]
    fn a_leading_slash_anchors_and_a_bare_name_does_not() {
        let i = ignore();
        // `/build/` is rooted, so a nested one is still walked.
        assert!(i.covers(Path::new("build"), true));
        assert!(!i.covers(Path::new("docs/build"), true));
        // `__pycache__/` is not rooted, so it matches at any depth.
        assert!(i.covers(Path::new("__pycache__"), true));
        assert!(i.covers(Path::new("a/b/__pycache__"), true));
    }

    #[test]
    fn a_trailing_slash_matches_a_directory_and_not_a_file_of_that_name() {
        let i = ignore();
        assert!(i.covers(Path::new("target"), true));
        assert!(
            !i.covers(Path::new("target"), false),
            "`target/` names a directory; a FILE called target is tracked"
        );
        // `thaum@.claude/worktrees/` carries no trailing slash, so it matches either.
        assert!(i.covers(Path::new(".claude/worktrees"), false));
    }

    #[test]
    fn a_star_matches_inside_one_segment_and_the_tail_must_land_at_the_end() {
        let i = ignore();
        assert!(i.covers(Path::new("a/b.pyc"), false));
        assert!(!i.covers(Path::new("a/b.pyc.bak"), false));
        assert!(!i.covers(Path::new("a/pyc"), false));
    }

    #[test]
    fn nothing_outside_the_file_is_covered() {
        let i = ignore();
        for path in [
            "docs/design.md",
            "crates/thaum-engine/src/lib.rs",
            "knowledge.toml",
            ".claude/skills/developing/SKILL.md",
        ] {
            assert!(!i.covers(Path::new(path), false), "{path}");
        }
    }

    #[test]
    fn a_pattern_the_matcher_cannot_honour_is_an_error_and_names_the_line() {
        // Dropping an ignore rule makes the walk read more than it should; dropping a negation
        // makes it read less. Both are silent, so neither may be dropped.
        for (text, what) in [
            ("build/\n!build/keep.md\n", "a negation"),
            ("a/**/b\n", "a `**` wildcard"),
            ("a?.md\n", "a `?` wildcard"),
            ("a[0-9].md\n", "a character class"),
        ] {
            let err = Ignore::parse(text).expect_err(what);
            assert!(err.contains(what), "{err}");
        }
    }

    #[test]
    fn comments_and_blank_lines_carry_nothing() {
        let i = Ignore::parse("# a comment\n\n   \n").expect("parses");
        assert!(i.is_empty());
        assert!(!i.covers(Path::new("a.md"), false));
    }
}
