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
    /// The same bodies with their typography untouched, for anything a human will paste.
    ///
    /// `rules` is normalised: `norm` folds a curly apostrophe to a straight one so that a quote
    /// written either way verifies. That is right for COMPARING and wrong for DISPLAYING —
    /// 1 933 of 3 162 rules contain a curly quote or apostrophe, and a session that pastes the
    /// folded form writes a quote that differs from the pinned text in exactly the way root
    /// `CLAUDE.md` names as the smallest form of paraphrase.
    raw: HashMap<RuleNumber, String>,
    /// Section titles, keyed by section number: what a whole-section citation quotes.
    ///
    /// Kept apart from `rules` so that everything iterating the rules — the canonical digest,
    /// the release diff, move detection — sees exactly what it saw before sections existed.
    /// The two maps meet only in the lookups, which route on [`RuleNumber::is_section`].
    sections: HashMap<u16, String>,
    sections_raw: HashMap<u16, String>,
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
        let mut raw: HashMap<RuleNumber, String> = HashMap::new();
        let mut sections: HashMap<u16, String> = HashMap::new();
        let mut sections_raw: HashMap<u16, String> = HashMap::new();
        let mut i = body_starts_at;
        while i < lines.len() {
            let Some((number, rest)) = rule_head(lines[i]) else {
                // A section heading is one line: the number, its printed dot, the title.
                // First occurrence wins here too, so a title-shaped line deeper in the text
                // cannot overwrite the heading a citation quotes.
                if let Some((section, title)) = section_head(lines[i]) {
                    sections_raw
                        .entry(section)
                        .or_insert_with(|| title.split_whitespace().collect::<Vec<_>>().join(" "));
                    sections.entry(section).or_insert_with(|| norm(title));
                }
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
            let joined = body.join(" ");
            // Whitespace is collapsed in both, because a body spans lines and a quote is one
            // run of text. Only the CHARACTERS differ between the two maps.
            raw.entry(number.clone())
                .or_insert_with(|| joined.split_whitespace().collect::<Vec<_>>().join(" "));
            rules.entry(number).or_insert_with(|| norm(&joined));
            i = j;
        }
        Self {
            rules,
            raw,
            sections,
            sections_raw,
        }
    }

    /// The body a citation of this number is checked against.
    ///
    /// For a whole-section number that is the section's TITLE — the heading line minus the
    /// number — which is all the text a section has of its own; its rules each carry theirs.
    pub fn get(&self, number: &RuleNumber) -> Option<&str> {
        if number.is_section() {
            return self.sections.get(&number.major()).map(String::as_str);
        }
        self.rules.get(number).map(String::as_str)
    }

    /// The body as the release prints it, typography and all.
    ///
    /// **Use this for anything a person will read or paste, and `get` for anything compared.**
    /// A quote taken from `get` verifies and is not what the rule says.
    pub fn raw(&self, number: &RuleNumber) -> Option<&str> {
        if number.is_section() {
            return self.sections_raw.get(&number.major()).map(String::as_str);
        }
        self.raw.get(number).map(String::as_str)
    }

    pub fn contains(&self, number: &RuleNumber) -> bool {
        if number.is_section() {
            return self.sections.contains_key(&number.major());
        }
        self.rules.contains_key(number)
    }

    /// Whether this release holds any rule numbered below this one.
    ///
    /// A parent rule's body says what it says and its subrules say the rest, so a quote of the
    /// parent entire does not stand for a claim that belongs to a subrule. Asked by the check
    /// that guards the whole-body exception.
    pub fn has_subrules(&self, number: &RuleNumber) -> bool {
        self.rules.keys().any(|k| is_subrule_of(k, number))
    }

    /// Every rule numbered below this one, in index order.
    ///
    /// The list rather than the predicate, for a reader deciding WHICH subrule a claim rests
    /// on. `has_subrules` answers the check; this answers the person repairing what it found.
    pub fn subrules(&self, number: &RuleNumber) -> Vec<&RuleNumber> {
        let mut out: Vec<&RuleNumber> = self
            .rules
            .keys()
            .filter(|k| is_subrule_of(k, number))
            .collect();
        out.sort_by(|a, b| a.cmp_index(b));
        out
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

/// Whether `candidate` is numbered below `parent`.
///
/// A subrule appends LETTERS. A digit makes it a sibling: `612.10` is not under `612.1`, and a
/// plain prefix test called seventeen leaf rules parents — two of them cited whole in this
/// repository, where the only repair would have been to stop quoting a leaf rule in full.
fn is_subrule_of(candidate: &RuleNumber, parent: &RuleNumber) -> bool {
    let prefix = parent.to_string();
    let n = candidate.to_string();
    // **A rule whose own number ends in a letter has no subrules.** Lettering runs a, b, … z,
    // aa, so `704.5aa` is the twenty-seventh SIBLING under `704.5`, not a child of `704.5a`.
    // Measured over the pinned release: 3 162 rules, exactly one with a two-letter suffix,
    // none with three. Without this test `704.5a` is called a parent, and its body is one
    // 59-character sentence, so every honest quote of it is the whole body and fires a check
    // that has no subrule to offer — both available repairs are dressing.
    if prefix.ends_with(|c: char| c.is_ascii_lowercase()) {
        return false;
    }
    n.len() > prefix.len()
        && n.starts_with(&prefix)
        && n[prefix.len()..].chars().all(|c| c.is_ascii_lowercase())
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
    section_head(line).is_some()
}

/// The section heading's parts: the number, and the title after the printed dot.
fn section_head(line: &str) -> Option<(u16, &str)> {
    let (digits, rest) = line.split_at_checked(3)?;
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let rest = rest.strip_prefix('.')?;
    let mut chars = rest.chars();
    let space = chars.next().filter(|c| c.is_whitespace())?;
    let title = &rest[space.len_utf8()..];
    Some((digits.parse().ok()?, title))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sibling_numbered_with_a_digit_is_not_a_subrule() {
        // `612.10` is a SIBLING of `612.1`, not a subrule of it. A plain prefix test could not
        // tell them apart and called seventeen leaf rules parents — two of them quoted whole
        // in this repository, where the only repair would have been to stop quoting a leaf
        // rule in full.
        const TEXT: &str = concat!(
            "100.1 A leaf rule with a digit sibling.\n",
            "100.10 The sibling, numbered with a digit.\n",
            "100.2 A parent rule.\n",
            "100.2a Its subrule, numbered with a letter.\n",
        );
        // BOUND, so the numbers are fixture data. Written into the call they are prose, and
        // this crate is walked like any other.
        const LEAF: &str = "100.1";
        const PARENT: &str = "100.2";
        let c = Corpus::parse(TEXT, 0);
        let leaf = RuleNumber::parse(LEAF).expect("a rule number");
        let parent = RuleNumber::parse(PARENT).expect("a rule number");
        assert!(!c.has_subrules(&leaf), "a digit makes it a sibling");
        assert!(c.has_subrules(&parent), "a letter makes it a subrule");
    }

    // Bound to names, so no other line holds a bare number. These are inputs to a parser,
    // not claims about what any rule says.
    const FIRST: &str = "100.1";
    const SECOND: &str = "100.2";
    const CONDITIONED: &str = "104.4b";
    const LAYERED: &str = "613.8c";

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
    fn a_section_heading_is_held_as_a_title_and_answers_the_section_form() {
        // What a whole-section citation quotes is the heading line, so the corpus must hold
        // it. The rules maps stay exactly as they were — the digest test below is the guard —
        // and only the section-form lookups reach the titles.
        const SECTION_LINE: &str = "100. A Section Title";
        let c = parse_contents(&body_lines(&format!(
            "{SECTION_LINE}\n{FIRST} First rule.\n"
        )));
        let section = RuleNumber::section(100);
        assert_eq!(c.get(&section), Some("A Section Title"));
        assert_eq!(c.raw(&section), Some("A Section Title"));
        assert!(c.contains(&section));
        assert_eq!(c.len(), 1, "the section is not a rule");
        assert!(!c.contains(&RuleNumber::section(200)));
        // A section's rules are not its SUBRULES: a title quoted whole must not be asked to
        // disclose them, the way a parent rule's body must. Mutation checked: making
        // `is_subrule_of` accept a dotted number under a section prefix fails this.
        assert!(!c.has_subrules(&section));
    }

    #[test]
    fn a_heading_above_the_body_start_is_a_contents_entry_and_binds_no_title() {
        // The table of contents duplicates every heading, so parsing from the top would bind
        // each title twice; `body_starts_at` is the property that prevents it, for sections
        // exactly as for rules.
        const SECTION_LINE: &str = "100. A Contents Title";
        let mut text = format!("{SECTION_LINE}\n");
        text.push_str(&"filler\n".repeat(CONTENTS_LINES - 1));
        text.push_str(&format!("{FIRST} Real rule.\n"));
        let c = Corpus::parse(&text, CONTENTS_LINES);
        assert!(!c.contains(&RuleNumber::section(100)));
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
