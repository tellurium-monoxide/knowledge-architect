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
    pub fn location(&self) -> String {
        match self.line {
            Some(n) => format!("{}:{n}", self.file.display()),
            None => self.file.display().to_string(),
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  {}\n    → {}",
            self.location(),
            self.what,
            self.action
        )
    }
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
