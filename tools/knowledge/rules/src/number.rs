//! Comprehensive Rule numbers: parsing, and the two orders the generated indexes use.

use std::cmp::Ordering;
use std::fmt;

/// A Comprehensive Rule number as the rules text prints it.
///
/// The grammar is the one every checker already uses: three digits, a dot, one or more
/// digits, then at most two lowercase letters — CR~613.8c is one.
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

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    /// The order `../../../../docs/rules/index.md` is generated in: section number, then the printed
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

    // Every rule number below is bound to a name on a line that carries its CR~ mention.
    // They are input to a parser, not claims about what any rule says, and `selftest.py`
    // already uses this shape for the same reason: a test about the checker cannot move the
    // checker's numbers. Binding them once keeps every other line free of a bare number.
    const PLAIN: &str = "400.1"; // CR~400.1
    const SECOND: &str = "400.2"; // CR~400.2
    const TENTH: &str = "400.10"; // CR~400.10
    const SUFFIXED: &str = "613.8c"; // CR~613.8c
    const ELEVENTH: &str = "613.11"; // CR~613.11
    const FIRST_LETTERED: &str = "613.1a"; // CR~613.1a
    const LONG_TAIL: &str = "702.169b"; // CR~702.169b

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
    fn index_order_compares_the_tail_as_text() {
        // Verified against the committed `../../../../docs/rules/index.md`, which carries CR~400.10 and
        // CR~400.11 between CR~400.1 and CR~400.2, and CR~613.11 before CR~613.1a. Ordering
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
