//! A file the walk does not cover may not name a rule.
//!
//! The walk selects by suffix, so a rule quote in a `.yml`, a `.json` or a dotfile would be
//! unverified AND invisible: a fabricated quote, a dangling slug and a nonexistent path
//! planted in a workflow file once left every checker exiting 0. Extending the walk is the
//! wrong fix, because the answer differs per file kind and the list would need extending again
//! for the next one. The assertion is the inverse and needs no list: outside the walk, a rule
//! number is forbidden. A file with something to say about the rules belongs in the walk; a
//! file with nothing to say has no reason to name one.
//!
//! An EXCLUDED path is neither. It is another project, with its own manifest and its own
//! documents, and a mock corpus naming a mock rule is a fixture rather than an unverified
//! claim — so the caller leaves those out of what it hands in.

use rules::RuleNumber;

use crate::finding::Finding;

use super::Inputs;

pub fn check(inputs: &Inputs) -> (Vec<Finding>, usize) {
    let mut out = Vec::new();
    for (path, text) in inputs.outside {
        for (i, line) in text.lines().enumerate() {
            if let Some(rule) = first_rule_number(line) {
                out.push(Finding::at(
                    path,
                    i as u32 + 1,
                    format!("rule {rule} is named here, and this file is outside the walk"),
                    "say it in a file the walk covers, or do not say it here: nothing verifies \
                     a rule number that no checker reads",
                ));
            }
        }
    }
    (out, inputs.outside.len())
}

/// The first rule-number-shaped token on a line, marked or not.
///
/// The section form counts only behind a marker or a keyword: a bare three-digit number in an
/// unwalked file is a count or a date fragment, and asserting over those would make the
/// assertion unusable. The dotted form counts bare, as it always has.
fn first_rule_number(line: &str) -> Option<RuleNumber> {
    static SECTION: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(?i)\b(?:cr:[\s*_]*|(?:cr|rules?|sections?)[\s*_]+)(\d{3})\b").unwrap()
    });
    let dotted = line
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '.'))
        .filter_map(|token| {
            let token = token.trim_start_matches(|c: char| !c.is_ascii_digit());
            RuleNumber::parse(token.trim_end_matches('.'))
        })
        .next();
    dotted.or_else(|| {
        SECTION.captures(line).and_then(|c| {
            let whole = c.get(1).unwrap();
            let mut rest = line[whole.end()..].chars();
            if rest.next() == Some('.') && rest.next().is_some_and(|c| c.is_ascii_digit()) {
                return None;
            }
            Some(RuleNumber::section(c[1].parse().ok()?))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rule_number_is_found_however_it_is_written() {
        const RULE: &str = "104.4b";
        for line in [
            format!("see {RULE} here"),
            format!("CR:{RULE}"),
            format!("(rule {RULE}.)"),
        ] {
            assert_eq!(
                first_rule_number(&line).map(|r| r.to_string()).as_deref(),
                Some(RULE),
                "{line}"
            );
        }
    }

    #[test]
    fn ordinary_text_and_versions_are_not_rule_numbers() {
        for line in [
            "a plain sentence",
            "version 1.2.3",
            "20260807",
            "10.4 and 1004.4",
        ] {
            assert!(first_rule_number(line).is_none(), "{line}");
        }
    }

    #[test]
    fn a_section_counts_behind_a_marker_or_keyword_and_a_bare_number_does_not() {
        // The branch made the section form citable, so an unwalked file naming one is the
        // exact silence this module's assertion exists to prevent — while a bare three-digit
        // number stays a count or a date fragment.
        const SECTION: &str = "104";
        for line in [
            format!("per CR:{SECTION} the game ends"),
            format!("see rule {SECTION} here"),
        ] {
            assert_eq!(
                first_rule_number(&line).map(|r| r.to_string()).as_deref(),
                Some(SECTION),
                "{line}"
            );
        }
        for line in [
            format!("{SECTION} files were scanned"),
            format!("rule {SECTION}.4b is dotted, and its own token wins"),
        ] {
            let found = first_rule_number(&line);
            assert_ne!(
                found.as_ref().map(|r| r.to_string()).as_deref(),
                Some(SECTION),
                "{line}: {found:?}"
            );
        }
    }
}
