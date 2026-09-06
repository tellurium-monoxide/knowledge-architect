//! The one shape every check reports.

use std::fmt;
use std::path::{Path, PathBuf};

/// One thing a check found wrong, in the one shape every check reports.
///
/// `what` states the defect and `action` states what to do about it. Both are required: a
/// report that says only what is wrong makes every reader derive the same fix, and deriving
/// it is where they disagree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Repository-relative, so a finding reads the same from any working directory.
    pub file: PathBuf,
    /// One-based, as an editor counts. `None` where the defect is the file itself — a
    /// missing tracker, a stale generated file — and there is no line to point at.
    pub line: Option<u32>,
    pub what: String,
    pub action: String,
}

impl Finding {
    pub fn at(
        file: impl AsRef<Path>,
        line: u32,
        what: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            file: file.as_ref().to_path_buf(),
            line: Some(line),
            what: what.into(),
            action: action.into(),
        }
    }

    /// A finding about a file as a whole, with no line to point at.
    pub fn in_file(
        file: impl AsRef<Path>,
        what: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            file: file.as_ref().to_path_buf(),
            line: None,
            what: what.into(),
            action: action.into(),
        }
    }

    /// `file:line`, or just the file where there is no line.
    ///
    /// This is the half a terminal turns into a link, so it is built once here rather than
    /// formatted at each call site where one check would eventually differ from the rest.
    ///
    /// **A finding's first line is one line, whatever the path holds.** A line break in the
    /// file's name is written as `\n` or `\r`, so a reader taking the output by line meets a
    /// finding per line rather than a truncated path and a stray tail. The walk refuses such
    /// a name, so the one finding that reports the refusal is the ordinary way a path
    /// reaches here; `Display` escapes the statement the same way.
    pub fn location(&self) -> String {
        let file = escaped(&self.file.display().to_string());
        match self.line {
            Some(n) => format!("{file}:{n}"),
            None => file,
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The statement is escaped like the location: a path a check formatted into it — an
        // anchor's path out of the manifest, a declared row — may hold a line break the walk
        // never saw, and the first line is the one a reader takes by line.
        write!(
            f,
            "{}  {}\n    → {}",
            self.location(),
            escaped(&self.what),
            self.action
        )
    }
}

/// The text with each line break written as `\n` or `\r`.
fn escaped(text: &str) -> String {
    text.replace('\n', "\\n").replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_located_finding_renders_file_line_then_the_action() {
        let f = Finding::at(
            "crates/thaum-engine/docs/design.md",
            604,
            "`thaum-engine#staging-not-mutation` is defined twice",
            "delete one definition; a reference resolves to exactly one anchor",
        );
        assert_eq!(f.location(), "crates/thaum-engine/docs/design.md:604");
        assert_eq!(
            f.to_string(),
            "crates/thaum-engine/docs/design.md:604  `thaum-engine#staging-not-mutation` is defined twice\n    \
             → delete one definition; a reference resolves to exactly one anchor"
        );
    }

    #[test]
    fn a_line_break_in_the_path_is_escaped_so_the_finding_stays_one_line() {
        let f = Finding::in_file("docs/a\nb\rc.md", "two anchors sit at `pa\npers`", "action");
        assert_eq!(f.location(), "docs/a\\nb\\rc.md");
        assert!(
            f.to_string()
                .starts_with("docs/a\\nb\\rc.md  two anchors sit at `pa\\npers`\n"),
            "{f}"
        );
        let shown = f.to_string();
        assert_eq!(
            shown.lines().count(),
            2,
            "the location and the action, and nothing the path adds: {shown:?}"
        );
    }

    #[test]
    fn a_whole_file_finding_omits_the_colon_rather_than_inventing_a_line() {
        let f = Finding::in_file(
            "crates/thaum-corpus/docs/open-issues.md",
            "the component carries no tripwires.md",
            "create it, or stop declaring the directory a component",
        );
        assert_eq!(f.location(), "crates/thaum-corpus/docs/open-issues.md");
        assert!(!f.to_string().contains(":0"));
    }
}
