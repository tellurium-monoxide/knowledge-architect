//! Is there a release newer than the one we are pinned to?
//!
//! The pin exists so a rules change is detected rather than absorbed, and the checkers do
//! detect one — a quote that stops verifying is a rule that moved. But every path into a
//! bump starts from a date a human already has, so nothing here noticed that a release
//! existed. This is what notices.
//!
//! It reads the page Wizards announces on rather than probing the download URL, because the
//! URLs rotate and a rotation makes every probe miss. And **no match is a failure, never an
//! answer**: the page is HTML nobody here controls, so the extractor will one day match
//! nothing, and reading that as *no new release* keeps the watch green while it is dead.

use std::sync::LazyLock;

use regex::Regex;

pub const PAGE: &str = "https://magic.wizards.com/en/rules";

/// The page carries the same link three times — `.txt`, `.pdf`, `.docx` — and again inside a
/// JSON-escaped copy of the same markup, so the separator before the date is a literal space
/// in the HTML and the extension is not fixed. Matching all three extensions is deliberate:
/// a release published as PDF before the plain text appears is still a release, and being a
/// few hours early is the harmless direction to be wrong in.
static LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)MagicCompRules(?:\s|%20|\\u0020)+(\d{8})\.(?:txt|pdf|docx)")
        .expect("the link pattern compiles")
});

/// The latest release date the page advertises, or `None` if it advertises none.
///
/// `None` means the extractor failed, never that no release exists. A caller that reads it
/// as an answer about the corpus turns a broken watch into a quiet one.
pub fn newest(page: &str) -> Option<String> {
    LINK.captures_iter(page).map(|c| c[1].to_string()).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each case pairs a fragment with what [`newest`] must return for it.
    ///
    /// Three of the first four are forms the live page carries: it has six occurrences —
    /// `.txt`, `.pdf` and `.docx` with a literal space, and the same three again inside a
    /// JSON-escaped copy. The percent-encoded form is **not** on the page and is here
    /// because it is what the download URL is built with and what `VERSION` records, so a
    /// page that ever switches to it keeps parsing.
    ///
    /// **Four of the nine must return `None`** — an empty page, a page without the link, a
    /// link with no date, and a malformed date. Those four are the ones the design turns on.
    /// A false *nothing new* is silent; a wrong date is loud and gets corrected by whoever
    /// reads it.
    const CASES: &[(&str, Option<&str>, &str)] = &[
        (
            r#"href="https://media.wizards.com/2026/downloads/MagicCompRules 20260807.txt""#,
            Some("20260807"),
            "the plain-text link as the page writes it, with a literal space",
        ),
        (
            "MagicCompRules%2020260807.txt",
            Some("20260807"),
            "percent-encoded separator",
        ),
        (
            "MagicCompRules 20260807.pdf",
            Some("20260807"),
            "the PDF link alone",
        ),
        (
            r"MagicCompRules 20260807.docx>",
            Some("20260807"),
            "inside the JSON-escaped copy",
        ),
        (
            "MagicCompRules 20260807.txt and MagicCompRules 20261107.txt",
            Some("20261107"),
            "two releases named: the later one wins",
        ),
        ("", None, "empty page"),
        (
            r#"<a href="/en/rules">Comprehensive Rules</a>"#,
            None,
            "a page that no longer carries the link",
        ),
        ("MagicCompRules.txt", None, "a link with no date in it"),
        (
            "MagicCompRules 2026087.txt",
            None,
            "a seven-digit date is not a date",
        ),
    ];

    #[test]
    fn the_extractor_reads_every_form_the_page_uses() {
        for (page, want, why) in CASES {
            assert_eq!(
                newest(page).as_deref(),
                *want,
                "{why}: expected {want:?} for {page:?}"
            );
        }
    }

    #[test]
    fn four_of_the_nine_cases_must_return_nothing() {
        // The count is asserted so that weakening a case into a passing one is visible: a
        // false negative here is the silent failure the whole design is arranged against.
        assert_eq!(
            CASES.iter().filter(|(_, want, _)| want.is_none()).count(),
            4
        );
    }

    #[test]
    fn a_seven_digit_date_does_not_match_by_truncation() {
        // The digit count is exact rather than a minimum. Eight digits inside a longer run
        // would otherwise be read as a date.
        assert_eq!(newest("MagicCompRules 202608071.txt"), None);
    }
}
