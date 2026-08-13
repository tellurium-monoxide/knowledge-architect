//! The pinned rules text, parsed into one body per rule number.

use std::collections::HashMap;

use crate::number::RuleNumber;
use crate::text::norm;

/// Every rule in one release, keyed by the number the text prints.
///
/// The body is normalised and excludes the number itself. Excluding it is what lets move
/// detection match a rule by its text across releases: a body carrying its own number could
/// never match itself after a renumbering.
#[derive(Clone, Debug, Default)]
pub struct Corpus {
    rules: HashMap<RuleNumber, String>,
}

impl Corpus {
    /// Parse a release.
    ///
    /// A rule runs from its numbered line to the next numbered line, the next section
    /// heading, or a blank line. Continuation lines are part of the rule: 214 rules carry
    /// Example lines that clarify meaning and can change independently, so they are part of
    /// the pinned unit rather than commentary to drop.
    ///
    /// The first occurrence of a number wins, matching the Python's `setdefault`. The
    /// Glossary repeats rule numbers in its entries, and a later occurrence overwriting an
    /// earlier one would silently replace a rule's body with a glossary gloss.
    /// `body_starts_at` is the zero-based line the rules body begins on, which a project
    /// declares because it is a property of its corpus and not of this parser. This one's
    /// text opens with a table of contents that repeats every section heading, so a scan from
    /// the top matches each heading twice and binds half the file to the wrong numbers.
    pub fn parse(text: &str, body_starts_at: usize) -> Self {
        let lines: Vec<&str> = text.lines().collect();
        let mut rules: HashMap<RuleNumber, String> = HashMap::new();
        let mut i = body_starts_at;
        while i < lines.len() {
            let Some((number, rest)) = rule_head(lines[i]) else {
                i += 1;
                continue;
            };
            let mut body = vec![rest];
            let mut j = i + 1;
            while j < lines.len()
                && !lines[j].trim().is_empty()
                && rule_head(lines[j]).is_none()
                && !is_section_head(lines[j])
            {
                body.push(lines[j]);
                j += 1;
            }
            rules.entry(number).or_insert_with(|| norm(&body.join(" ")));
            i = j;
        }
        Self { rules }
    }

    pub fn get(&self, number: &RuleNumber) -> Option<&str> {
        self.rules.get(number).map(String::as_str)
    }

    pub fn contains(&self, number: &RuleNumber) -> bool {
        self.rules.contains_key(number)
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RuleNumber, &str)> {
        self.rules.iter().map(|(k, v)| (k, v.as_str()))
    }

    /// The whole corpus as one line per rule, sorted by number: `number\tbody\n`.
    ///
    /// This exists to be hashed. It is how the Rust parse was compared against the Python
    /// one it replaces, over all 3 162 rules at once rather than over a sample.
    pub fn canonical(&self) -> String {
        let mut keys: Vec<&RuleNumber> = self.rules.keys().collect();
        keys.sort_by(|a, b| a.cmp_index(b));
        keys.iter()
            .map(|k| format!("{k}\t{}\n", self.rules[*k]))
            .collect()
    }
}

/// A rule's opening line: the number, an optional full stop, one whitespace character, and
/// the rest of the line.
///
/// The single whitespace character is the Python's `\s`, which matches exactly one. A second
/// space stays in the body, where `norm` collapses it — the distinction never shows in the
/// output and is kept because guessing which of two behaviours the original had is how a
/// port drifts.
fn rule_head(line: &str) -> Option<(RuleNumber, &str)> {
    let (index, space) = line.char_indices().find(|(_, c)| c.is_whitespace())?;
    let token = &line[..index];
    let number = RuleNumber::parse(token.strip_suffix('.').unwrap_or(token))?;
    Some((number, &line[index + space.len_utf8()..]))
}

/// A section heading — three digits, a full stop, whitespace — which ends the rule above it.
fn is_section_head(line: &str) -> bool {
    let mut chars = line.chars();
    let digits: String = chars.by_ref().take(3).collect();
    digits.len() == 3
        && digits.bytes().all(|b| b.is_ascii_digit())
        && chars.next() == Some('.')
        && chars.next().is_some_and(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bound to names on lines carrying their CR~ mentions, so no other line holds a bare
    // number. These are inputs to a parser, not claims about what any rule says.
    const FIRST: &str = "100.1"; // CR~100.1
    const SECOND: &str = "100.2"; // CR~100.2
    const CONDITIONED: &str = "104.4b"; // CR~104.4b
    const LAYERED: &str = "613.8c"; // CR~613.8c

    /// This project's corpus opens with 181 lines of contents.
    const CONTENTS_LINES: usize = 181;

    fn n(s: &str) -> RuleNumber {
        RuleNumber::parse(s).expect("a rule number")
    }

    fn parse_contents(text: &str) -> Corpus {
        Corpus::parse(text, CONTENTS_LINES)
    }

    fn body_lines(head: &str) -> String {
        // 181 filler lines stand in for the table of contents the parser skips.
        let mut text = "contents\n".repeat(CONTENTS_LINES);
        text.push_str(head);
        text
    }

    #[test]
    fn a_rule_runs_to_the_next_number() {
        let c = parse_contents(&body_lines(&format!(
            "{FIRST} First rule.\n{SECOND} Second rule.\n"
        )));
        assert_eq!(c.len(), 2);
        assert_eq!(c.get(&n(FIRST)), Some("First rule."));
    }

    #[test]
    fn continuation_lines_join_the_rule() {
        let c = parse_contents(&body_lines(&format!(
            "{FIRST} Head\n  wrapped on\n  three lines\n"
        )));
        assert_eq!(c.get(&n(FIRST)), Some("Head wrapped on three lines"));
    }

    #[test]
    fn a_blank_line_and_a_section_heading_both_end_a_rule() {
        let c = parse_contents(&body_lines(&format!("{FIRST} Head\n\ntrailing prose\n")));
        assert_eq!(c.get(&n(FIRST)), Some("Head"));
        let c = parse_contents(&body_lines(&format!("{FIRST} Head\n200. Section\ntitle\n")));
        assert_eq!(c.get(&n(FIRST)), Some("Head"));
    }

    #[test]
    fn the_first_occurrence_of_a_number_wins() {
        // The Glossary repeats numbers; a later one must not overwrite the rule's body.
        let c = parse_contents(&body_lines(&format!(
            "{FIRST} The rule.\n\n{FIRST} The glossary gloss.\n"
        )));
        assert_eq!(c.get(&n(FIRST)), Some("The rule."));
    }

    #[test]
    fn the_table_of_contents_is_not_parsed() {
        // A numbered line above the body start is a contents entry, not a rule.
        let mut text = format!("{FIRST} Contents entry\n");
        text.push_str(&"filler\n".repeat(CONTENTS_LINES - 1));
        text.push_str(&format!("{SECOND} Real rule.\n"));
        let c = Corpus::parse(&text, CONTENTS_LINES);
        assert!(!c.contains(&n(FIRST)));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn a_trailing_full_stop_on_the_number_is_not_part_of_it() {
        let c = parse_contents(&body_lines(&format!("{FIRST}. With a stop.\n")));
        assert_eq!(c.get(&n(FIRST)), Some("With a stop."));
    }

    #[test]
    fn the_body_excludes_the_number_so_a_move_can_be_detected() {
        let c = parse_contents(&body_lines(&format!(
            "{CONDITIONED} Text that could move.\n"
        )));
        assert_eq!(c.get(&n(CONDITIONED)), Some("Text that could move."));
        assert!(!c.get(&n(CONDITIONED)).unwrap().contains(CONDITIONED));
    }

    /// The release the cross-implementation digest below was taken against.
    const DIGEST_RELEASE: &str = "20260807";

    /// `sha256` of [`Corpus::canonical`] over the pinned text, as produced by the Python
    /// `index_rules` this parser replaces.
    ///
    /// Re-take from the implementation being replaced, while it still exists:
    ///
    /// ```text
    /// python3 -c 'import sys,hashlib; sys.path.insert(0,"tools/rules");
    /// from _common import index_rules, TEXT; r=index_rules(TEXT)
    /// print(len(r), hashlib.sha256("".join(f"{k}\t{v}\n" for k,v in sorted(r.items())).encode()).hexdigest())'
    /// ```
    const PYTHON_PARSE_DIGEST: &str =
        "f69b0dae08d71defd7158600766ddffd5f21172d0a3278ec0afcfbd5c508b42e";

    const RULES_IN_DIGEST_RELEASE: usize = 3162;

    #[test]
    fn the_whole_parse_matches_the_implementation_it_replaces() {
        // Not a sample: every rule in the release, compared at once. It is guarded by the
        // release rather than updated on a bump, because the comparison is against an
        // implementation that will not exist to re-take it from — after a bump this stops
        // being a cross-implementation check and says so, instead of becoming a constant
        // somebody edits until it passes.
        let tree = crate::testing::this_tree();
        let version = std::fs::read_to_string(tree.version()).expect("VERSION");
        let pinned = crate::release::read_version(&version)
            .get("date")
            .expect("a pinned date")
            .clone();
        if pinned != DIGEST_RELEASE {
            eprintln!(
                "SKIPPED: the digest was taken against {DIGEST_RELEASE} and the tree is \
                 pinned at {pinned}, so it no longer compares two implementations."
            );
            return;
        }
        let text = std::fs::read_to_string(tree.text()).expect("the rules text");
        let corpus = Corpus::parse(&text, CONTENTS_LINES);
        assert_eq!(corpus.len(), RULES_IN_DIGEST_RELEASE);
        assert_eq!(
            crate::release::sha256(corpus.canonical().as_bytes()),
            PYTHON_PARSE_DIGEST
        );
    }

    #[test]
    fn typography_is_normalised_into_the_body() {
        let c = parse_contents(&body_lines(&format!(
            "{LAYERED} It\u{2019}s   \u{201c}applied\u{201d}.\n"
        )));
        assert_eq!(c.get(&n(LAYERED)), Some("It's \"applied\"."));
    }
}
