//! Comprehensive Rule numbers: parsing, and the two orders the generated indexes use.

use std::cmp::Ordering;
use std::fmt;

/// A Comprehensive Rule number as the rules text prints it.
///
/// The grammar is the one every checker already uses: three digits, a dot, one or more
/// digits, then at most two lowercase letters — `613.8c` is one.
///
/// The printed text is stored verbatim and is what `Display` returns, so a number always
/// round-trips to the bytes it was read from. The parsed parts exist only for ordering.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuleNumber {
    text: String,
    major: u16,
    minor: u32,
    suffix: String,
}

impl RuleNumber {
    /// Parse a rule number, or `None` if the token is not one.
    ///
    /// A bare section number is NOT one: three digits with no subrule part is a count, a line
    /// number or a date fragment far more often than it is a citation, so accepting it here
    /// would make every caller that scans free text read noise as rules. A caller that has
    /// already established the token names a section — a marker behind `CR:`, a blockquote
    /// head with its printed dot — asks [`RuleNumber::parse_any`] instead.
    pub fn parse(text: &str) -> Option<Self> {
        let (major_text, rest) = text.split_once('.')?;
        if major_text.len() != 3 || !major_text.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        if digits == 0 {
            return None;
        }
        let (minor_text, suffix) = rest.split_at(digits);
        if suffix.len() > 2 || !suffix.bytes().all(|b| b.is_ascii_lowercase()) {
            return None;
        }
        Some(Self {
            text: text.to_string(),
            major: major_text.parse().ok()?,
            minor: minor_text.parse().ok()?,
            suffix: suffix.to_string(),
        })
    }

    /// A whole-section number: three digits and no subrule part.
    ///
    /// Sections cite like rules — the marker owes a verbatim quote of the section's heading
    /// line — and differ only in what the corpus holds for them: a title rather than a body.
    pub fn section(major: u16) -> Self {
        Self {
            text: format!("{major:03}"),
            major,
            minor: 0,
            suffix: String::new(),
        }
    }

    /// Parse a token already established to name a rule OR a whole section.
    ///
    /// The section alternative accepts exactly three digits. It is a separate entry point
    /// rather than a widening of [`RuleNumber::parse`] because only a caller that has seen the
    /// citation shape around the token — a `CR:` marker, a blockquote head printed with its
    /// dot — can afford to read three bare digits as a number of the rules.
    pub fn parse_any(text: &str) -> Option<Self> {
        Self::parse(text).or_else(|| {
            (text.len() == 3 && text.bytes().all(|b| b.is_ascii_digit()))
                .then(|| text.parse().ok().map(Self::section))
                .flatten()
        })
    }

    /// Whether this names a whole section rather than one rule.
    pub fn is_section(&self) -> bool {
        !self.text.contains('.')
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    /// The order `thaum@docs/rules/index.md` is generated in: section number, then the printed
    /// text compared as text.
    ///
    /// The text tiebreak is deliberate and is **not** numeric — the committed index carries
    /// a two-digit tail between the first and second entries of its section. `citations.py`
    /// sorts by `(int(major), whole string)`, and the generated file is byte-compared by its
    /// freshness check, so ordering these numerically would rewrite the index on the first
    /// run.
    pub fn cmp_index(&self, other: &Self) -> Ordering {
        (self.major, &self.text).cmp(&(other.major, &other.text))
    }

    /// The order an interpretation entry's *Turns on* line is generated in: section number,
    /// then the number after the dot compared numerically, then the printed text.
    ///
    /// It differs from [`RuleNumber::cmp_index`] in the middle term, and both are reproduced
    /// rather than reconciled: each generates a committed file that its freshness check
    /// compares byte for byte.
    pub fn cmp_turns(&self, other: &Self) -> Ordering {
        (self.major, self.minor, &self.text).cmp(&(other.major, other.minor, &other.text))
    }
}

impl fmt::Display for RuleNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every rule number below is bound to a name. They are input to a parser, not claims
    // about what any rule says, and `selftest.py` already uses this shape for the same
    // reason: a test about the checker cannot move the checker's numbers. Binding them once
    // keeps every other line free of a bare number.
    const PLAIN: &str = "400.1";
    const SECOND: &str = "400.2";
    const TENTH: &str = "400.10";
    const SUFFIXED: &str = "613.8c";
    const ELEVENTH: &str = "613.11";
    const FIRST_LETTERED: &str = "613.1a";
    const LONG_TAIL: &str = "702.169b";

    fn n(s: &str) -> RuleNumber {
        RuleNumber::parse(s).expect("should parse")
    }

    #[test]
    fn parses_the_printed_shapes() {
        assert_eq!(n(PLAIN).as_str(), PLAIN);
        assert_eq!(n(SUFFIXED).suffix(), "c");
        assert_eq!(n(LONG_TAIL).major(), 702);
        assert_eq!(n(PLAIN).suffix(), "");
    }

    #[test]
    fn round_trips_to_the_bytes_it_was_read_from() {
        for s in [PLAIN, SUFFIXED, LONG_TAIL, ELEVENTH] {
            assert_eq!(n(s).to_string(), s);
        }
    }

    #[test]
    fn rejects_tokens_that_are_not_rule_numbers() {
        // None of these is rule-number-shaped, and each is a form the tree actually holds:
        // a two- or four-digit section, a release date, a section heading with no tail, an
        // uppercase suffix, three trailing letters, and a number glued to a word.
        for s in [
            "10.4", "1004.4", "20260807", "103", "103.", "103.4B", "103.4abc", "a103.4",
        ] {
            assert!(RuleNumber::parse(s).is_none(), "{s} should not parse");
        }
    }

    #[test]
    fn a_section_parses_only_through_the_deliberate_entry_point() {
        // `parse` rejecting three bare digits is what keeps free-text scanning from reading a
        // count or a date fragment as a citation; `parse_any` exists for callers that have
        // already seen the citation shape around the token. Mutation checked: widening `parse`
        // to accept the section form fails `rejects_tokens_that_are_not_rule_numbers` above.
        const SECTION: &str = "103";
        let s = RuleNumber::parse_any(SECTION).expect("a section number");
        assert_eq!(s.to_string(), SECTION);
        assert_eq!(s.major(), 103);
        assert!(s.is_section());
        assert!(!n(PLAIN).is_section());
        assert_eq!(s, RuleNumber::section(103));
        assert_ne!(Some(&s), RuleNumber::parse_any(PLAIN).as_ref());
        // The dotted form still comes through `parse_any` unchanged.
        assert_eq!(RuleNumber::parse_any(SUFFIXED), RuleNumber::parse(SUFFIXED));
        for not_a_section in ["10", "1034", "1a3", "103."] {
            assert!(
                RuleNumber::parse_any(not_a_section).is_none(),
                "{not_a_section}"
            );
        }
    }

    #[test]
    fn a_section_sorts_ahead_of_its_own_rules_in_both_orders() {
        // Both generated files group by section, so the section row must open its group
        // rather than land mid-list by a quirk of the text comparison.
        let s = RuleNumber::section(400);
        assert_eq!(s.cmp_index(&n(PLAIN)), Ordering::Less);
        assert_eq!(s.cmp_turns(&n(PLAIN)), Ordering::Less);
    }

    #[test]
    fn index_order_compares_the_tail_as_text() {
        // Verified against the committed `thaum@docs/rules/index.md`, which carries `400.10`
        // and `400.11` between `400.1` and `400.2`, and `613.11` before `613.1a`. Ordering
        // the tail numerically would rewrite the file on the first run and fail its
        // freshness check.
        assert_eq!(n(PLAIN).cmp_index(&n(TENTH)), Ordering::Less);
        assert_eq!(n(TENTH).cmp_index(&n(SECOND)), Ordering::Less);
        assert_eq!(n(ELEVENTH).cmp_index(&n(FIRST_LETTERED)), Ordering::Less);
    }

    #[test]
    fn turns_order_compares_the_tail_numerically() {
        // The other generated file sorts the same pairs the other way.
        assert_eq!(n(TENTH).cmp_turns(&n(SECOND)), Ordering::Greater);
        assert_eq!(n(ELEVENTH).cmp_turns(&n(FIRST_LETTERED)), Ordering::Greater);
    }
}
