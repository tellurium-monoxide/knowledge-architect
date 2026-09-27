//! The rules extension's own scan of a document: rule markers, rule tokens and the release
//! pin, per `design@knowledge@an-extension-builds-its-own-model`.
//!
//! It reads the core's parse and nothing else, so it sees exactly the prose the core scanner
//! sees. **A bare rule number inside an inline code span is data, not a citation**: a sort key,
//! a parser input, a line of tool output. **A fence is live**: a sketch cites its rules on
//! purpose, per `design@knowledge@grammars-not-prefixes`. **A marker is never data**, wherever it
//! sits, and the identifier form in a code span is the one exception, because there it names an
//! identifier.

use std::sync::LazyLock;

use regex::Regex;
use rules::RuleNumber;

use crate::model::Document;
use crate::source::Parsed;

/// How a rule number was written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerForm {
    /// `CR:` — a claim about what the rule says. It owes a quote.
    Prose,
    /// The identifier form, in a NAME. It owes the rule's whole body above it, because a name
    /// has no room to qualify what it asserts.
    Identifier,
    /// The identifier form written in PROSE, where the prose form fits. Told apart from the
    /// one above because only the second is what the convention exists for, and a check that
    /// conflated them would ask a sentence for a quote a name owes.
    IdentifierInProse,
}

/// A rule-shaped observation, found at a line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleObservation {
    Marker {
        number: RuleNumber,
        form: MarkerForm,
    },
    /// A rule-number-shaped token, marked or not. The lint's input: one carrying no marker
    /// on its own line probably wants a quote.
    Token(RuleNumber),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleLocated {
    /// One-based, as an editor counts.
    pub line: u32,
    pub what: RuleObservation,
}

static CR_PROSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bCR:(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
/// The identifier form, in ANY case.
///
/// The prescribed upper-case spelling fails `non_snake_case` under the clippy gate, so every
/// rule-named test in this tree uses the lower-case one — and a case-sensitive pattern left the
/// orphan lint structurally dead for the only shape the convention exists for.
static CR_IDENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bCR_(\d{3})_(\d+[a-z]{0,2})").unwrap());
/// The same, ANYWHERE inside a name.
///
/// A name is one token, so there is no word boundary to lean on and no prose around it to
/// mistake for one. The bounded pattern above could not see a marker inside a `test_`-prefixed
/// name — an underscore is a word character, so the boundary never matched — which left the
/// convention unenforced for the commonest way to write the shape it exists for.
static CR_IDENT_IN_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)CR_(\d{3})_(\d+[a-z]{0,2})").unwrap());
static RULE_TOKEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
/// The section-level marker: `CR:` and three digits with no subrule part.
///
/// The pattern alone also matches the front of a dotted marker — the boundary sits between
/// the third digit and the dot — so the scan discards a match that a subrule digit continues.
static CR_SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bCR:([0-9]{3})\b").unwrap());
/// A section named through a keyword: the lint's input, never a marker.
///
/// A bare three-digit number is noise at a measured ratio of hundreds of counts and line
/// numbers to a handful of section references, so the keyword is what makes the token
/// claimable at all. The dotted alternative in the tail is there to be DISCARDED: `rule
/// 601.2` names a rule, and the dotted patterns above already carry it.
///
/// Any casing, and emphasis characters may sit between the keyword and the digits: `RULE
/// 104`, `rule *104*` and a lowercase `cr:104` all claim exactly what the plain form claims,
/// and a case-varying evasion is cheaper than a code span. The `cr:` alternative also matches
/// the uppercase marker, so the scan drops a match whose digits a marker already carries.
static SECTION_KEYWORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:cr:[\s*_]*|(?:cr|rules?|sections?)[\s*_]+)([0-9]{3})\b").unwrap()
});
static PIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!--\s*cr-version:\s*(\d{8})\s*-->").unwrap());

/// The release a file's quotes verify against: its own pin, or `None` for the vendored one.
///
/// **An illustration of the syntax is not a pin.** This runs over the raw file, except under
/// the checker's own source where only the comments are read, so a document
/// EXPLAINING the mechanism — a README, a plan, a skill — used to repin itself by showing the
/// form in a fenced example, and then verified its quotes against a release it never chose. The
/// resolver fetches an absent release over the network, so the symptom was a `curl` failure in
/// a check that reads no network for anything else.
pub fn pin(parsed: &Parsed, text: &str) -> Option<String> {
    let fenced: std::collections::HashSet<u32> = parsed.fenced.iter().copied().collect();
    // Under the checker's own source a pin is read from the comments only. The raw text holds
    // the fixtures a `Data` parse dropped, and a pin spelled inside one of them pinned the file
    // it sat in to that release, per `design@knowledge@checker-source-literals-are-data`.
    let prose: Option<std::collections::HashSet<u32>> =
        (parsed.literals == crate::source::Literals::Data).then(|| {
            parsed
                .prose
                .iter()
                .flat_map(|p| p.lines.iter().copied())
                .collect()
        });
    for (i, line) in text.lines().enumerate() {
        let n = i as u32 + 1;
        // Fenced only. The pin IS an HTML comment, so skipping the inert lines would find
        // no pin anywhere; what matters is that an ILLUSTRATION of one does not bind.
        if fenced.contains(&n) {
            continue;
        }
        if prose.as_ref().is_some_and(|p| !p.contains(&n)) {
            continue;
        }
        if let Some(c) = PIN.captures(line) {
            return Some(c[1].to_string());
        }
    }
    None
}

/// The rule observations of one parsed document, in line order.
pub fn scan(parsed: &Parsed) -> Vec<RuleLocated> {
    let inert: std::collections::HashSet<u32> = parsed.inert.iter().copied().collect();
    let mut out = Vec::new();
    for region in &parsed.prose {
        let mut line_start = 0usize;
        for (i, line) in region.text.split('\n').enumerate() {
            let at = line_start;
            line_start += line.len() + 1;
            let n = region.file_line(i);
            // Commented out: an HTML comment parks its lines, as it does for the core scan.
            if inert.contains(&n) {
                continue;
            }
            let mut push = |what| out.push(RuleLocated { line: n, what });
            // Data, never a citation: a BARE rule number inside a code span is a value being
            // displayed. A marker is never data; see the module head.
            let data = |m: regex::Match| region.is_code(at + m.start());

            for c in CR_PROSE.captures_iter(line) {
                if let Some(number) = RuleNumber::parse(&c[1]) {
                    push(RuleObservation::Marker {
                        number,
                        form: MarkerForm::Prose,
                    });
                }
            }
            for c in CR_IDENT.captures_iter(line) {
                let whole = c.get(0).unwrap();
                // The ONE marker a code span exempts, and it is the one the convention
                // names: inside backticks the identifier form is naming an identifier —
                // a test, a fixture — rather than claiming what a rule says.
                if data(whole) {
                    continue;
                }
                // A marker embedded in a longer name is still a marker; one whose letter
                // suffix runs past two is not. The character after the match answers it,
                // Rust's engine having no lookahead.
                let after = line[whole.end()..].chars().next();
                if after.is_some_and(|c| c.is_alphanumeric() && c != '_') {
                    continue;
                }
                if let Some(number) = RuleNumber::parse(&format!("{}.{}", &c[1], &c[2])) {
                    push(RuleObservation::Marker {
                        number,
                        form: MarkerForm::IdentifierInProse,
                    });
                }
            }
            // A subrule digit after the match means the boundary landed inside a dotted
            // number, which the dotted patterns already carry.
            let continues_dotted = |m: regex::Match| {
                let mut rest = line[m.end()..].chars();
                rest.next() == Some('.') && rest.next().is_some_and(|c| c.is_ascii_digit())
            };
            let mut marker_digits: Vec<(usize, usize)> = Vec::new();
            for c in CR_SECTION.captures_iter(line) {
                let whole = c.get(0).unwrap();
                if continues_dotted(whole) {
                    continue;
                }
                let digits = c.get(1).unwrap();
                marker_digits.push((digits.start(), digits.end()));
                let number = RuleNumber::section(c[1].parse().expect("three digits"));
                // Marker AND token, the same pair a dotted marker line carries: the token is
                // what the lint compares against the marker, and what the index collects. The
                // token half keeps the dotted symmetry — a marker is never data, its token is.
                push(RuleObservation::Marker {
                    number: number.clone(),
                    form: MarkerForm::Prose,
                });
                if !data(whole) {
                    push(RuleObservation::Token(number));
                }
            }
            for c in RULE_TOKEN.captures_iter(line) {
                if data(c.get(0).unwrap()) {
                    continue;
                }
                if let Some(n) = RuleNumber::parse(&c[1]) {
                    push(RuleObservation::Token(n));
                }
            }
            for c in SECTION_KEYWORD.captures_iter(line) {
                let number = c.get(1).unwrap();
                if data(number) || continues_dotted(number) {
                    continue;
                }
                // The `cr:` alternative also matches the uppercase marker, whose token the
                // marker scan above already pushed once.
                if marker_digits.contains(&(number.start(), number.end())) {
                    continue;
                }
                push(RuleObservation::Token(RuleNumber::section(
                    c[1].parse().expect("three digits"),
                )));
            }
        }
    }
    // The identifier form lives in a NAME, which is code. Nothing above can see it, and it is
    // the one shape the convention was written for.
    for (line, name) in &parsed.names {
        for c in CR_IDENT_IN_NAME.captures_iter(name) {
            let whole = c.get(0).unwrap();
            let after = name[whole.end()..].chars().next();
            if after.is_some_and(|c| c.is_alphanumeric() && c != '_') {
                continue;
            }
            if let Some(number) = RuleNumber::parse(&format!("{}.{}", &c[1], &c[2])) {
                out.push(RuleLocated {
                    line: *line,
                    what: RuleObservation::Marker {
                        number,
                        form: MarkerForm::Identifier,
                    },
                });
            }
        }
    }
    out.sort_by_key(|l| l.line);
    out
}

/// The rule observations of one document.
pub fn of(doc: &Document) -> Vec<RuleLocated> {
    scan(&doc.parsed)
}

/// The release a document's quotes verify against: its own pin, or `None` for the vendored one.
pub fn pin_of(doc: &Document) -> Option<String> {
    pin(&doc.parsed, &doc.text)
}

/// Every observation `f` picks, with the line it sits on.
pub fn rules_of<'a, T: 'a>(
    observations: &'a [RuleLocated],
    f: impl Fn(&'a RuleObservation) -> Option<T> + 'a,
) -> impl Iterator<Item = (u32, T)> + 'a {
    observations
        .iter()
        .filter_map(move |l| f(&l.what).map(|t| (l.line, t)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture below is written as the bytes it means. The checker reads no string
    // literal of its own source, per `design@knowledge@checker-source-literals-are-data`, so a rule
    // number or a marker in one is data and not a claim.

    fn scan_md(text: &str) -> Vec<RuleObservation> {
        scan(&crate::source::md::parse(text))
            .into_iter()
            .map(|l| l.what)
            .collect()
    }

    fn markers(text: &str) -> Vec<(String, MarkerForm)> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                RuleObservation::Marker { number, form } => Some((number.to_string(), form)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_prose_marker_is_told_from_an_ordinary_number() {
        // A bare number beside a marked one is NOT a marker. What catches it is the
        // missing-marker lint, and that is why there is no form for a number owing no quote:
        // a number that is data goes in a code span or a name-bound literal.
        let text = "per CR:104.4b, and 613.8c is only a number";
        assert_eq!(
            markers(text),
            vec![("104.4b".to_string(), MarkerForm::Prose)]
        );
    }

    #[test]
    fn an_identifier_marker_is_found_anywhere_in_a_name() {
        for name in [
            "fn CR_104_4b_holds() {}",
            // Prefixed. An underscore is a word character, so a bounded pattern never matched
            // here — and `test_…` is the commonest way to write the shape the convention
            // exists for, which left it unenforced for exactly that shape.
            "fn test_cr_104_4b_holds() {}",
        ] {
            let seen: Vec<(String, MarkerForm)> = scan(&crate::source::rs::parse(
                name,
                crate::source::Literals::Prose,
            ))
            .into_iter()
            .filter_map(|l| match l.what {
                RuleObservation::Marker { number, form } => Some((number.to_string(), form)),
                _ => None,
            })
            .collect();
            assert_eq!(
                seen,
                vec![("104.4b".to_string(), MarkerForm::Identifier)],
                "{name}"
            );
        }
    }

    #[test]
    fn an_identifier_marker_in_prose_is_told_apart_from_one_in_a_name() {
        // Only the second is what the convention exists for. A check that conflated them
        // would ask a sentence for the quote a name owes.
        let prose = "the test cr_104_4b_holds covers it";
        assert_eq!(
            markers(prose),
            vec![("104.4b".to_string(), MarkerForm::IdentifierInProse)]
        );
    }

    #[test]
    fn an_identifier_marker_with_too_long_a_suffix_is_not_one() {
        // Through `rs::parse`, not `scan_md`. Scanned as MARKDOWN this reached neither
        // identifier pattern, so it passed with both suffix guards removed and pinned
        // nothing at all — while reading as though it covered names.
        const TOO_LONG: &str = "fn cr_613_8cde_holds() {}";
        let seen: Vec<RuleObservation> = scan(&crate::source::rs::parse(
            TOO_LONG,
            crate::source::Literals::Prose,
        ))
        .into_iter()
        .map(|l| l.what)
        .collect();
        assert!(
            !seen
                .iter()
                .any(|o| matches!(o, RuleObservation::Marker { .. })),
            "{seen:?}"
        );
        // The control: two letters IS a rule number, and must still be found.
        const OK: &str = "fn cr_613_8c_holds() {}";
        assert!(scan(&crate::source::rs::parse(
            OK,
            crate::source::Literals::Prose
        ))
        .into_iter()
        .any(|l| matches!(l.what, RuleObservation::Marker { .. })));
    }

    /// The tokens a line yields, as printed text.
    fn tokens(text: &str) -> Vec<String> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                RuleObservation::Token(n) => Some(n.to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_section_marker_is_a_marker_and_a_token_and_a_dotted_marker_is_not_one() {
        // The dotless marker is the section citation form. The boundary in its pattern also
        // matches the FRONT of a dotted marker, so the guard is what keeps `CR:` and a dotted
        // number from reporting twice. Mutation checked: removing the subrule-digit guard
        // fails the second assertion with two markers on the line.
        let section = "in the order CR:104 states them";
        assert_eq!(
            markers(section),
            vec![("104".to_string(), MarkerForm::Prose)]
        );
        assert_eq!(tokens(section), vec!["104".to_string()]);
        let dotted = "per CR:104.4b, the step is pinned";
        assert_eq!(
            markers(dotted),
            vec![("104.4b".to_string(), MarkerForm::Prose)]
        );
        // At a sentence end the dot belongs to the sentence, not the number.
        let sentence = "the order is CR:104. The next sentence.";
        assert_eq!(
            markers(sentence),
            vec![("104".to_string(), MarkerForm::Prose)]
        );
    }

    #[test]
    fn a_keyword_section_reference_is_a_token_and_never_a_marker() {
        // The keyword form is the lint's input: a token with no marker on its line is what
        // the missing-marker lint reports. A dotted number after the keyword is a rule
        // reference the dotted patterns already carry, so the keyword match is discarded.
        for keyword in ["CR", "rule", "Rule", "rules", "section", "Sections"] {
            let text = format!("named by {keyword} 104 in prose");
            assert_eq!(tokens(&text), vec!["104".to_string()], "{keyword}");
            assert!(markers(&text).is_empty(), "{keyword}");
        }
        let dotted = "named by rule 104.4b in prose";
        assert_eq!(tokens(dotted), vec!["104.4b".to_string()]);
    }

    #[test]
    fn a_case_or_emphasis_evasion_of_the_keyword_shape_is_still_a_token() {
        // A case-varying or emphasised keyword claims exactly what the plain form claims,
        // and either evasion is cheaper than a code span. The lowercase colon form is a
        // token rather than a marker: the marker convention is uppercase, and legitimising
        // the lowercase spelling would let two forms drift.
        for shape in [
            "named by RULE 104 in prose",
            "named by SECTION 104 in prose",
            "named by rule *104* in prose",
            "named by cr:104 in prose",
        ] {
            assert_eq!(tokens(shape), vec!["104".to_string()], "{shape}");
            assert!(markers(shape).is_empty(), "{shape}");
        }
        // The uppercase marker matches the widened keyword pattern too, and must still
        // yield exactly one token beside its marker, not two.
        let marker = "in the order CR:104 states them";
        assert_eq!(tokens(marker), vec!["104".to_string()]);
    }

    #[test]
    fn a_keyword_section_number_in_a_code_span_is_data() {
        // The data carve-outs survive the section form: inside backticks the number is being
        // displayed, and whether that shelters a claim is the reviewer's question, not a
        // pattern's.
        let sheltered = "named by rule `104` in a code span";
        assert_eq!(tokens(sheltered), Vec::<String>::new());
        // A backticked MARKER stays a marker — markers are never data — and its token keeps
        // the dotted symmetry by staying out.
        let marker = "named by `CR:104` in a code span";
        assert_eq!(
            markers(marker),
            vec![("104".to_string(), MarkerForm::Prose)]
        );
        assert_eq!(tokens(marker), Vec::<String>::new());
    }

    #[test]
    fn a_non_ascii_digit_is_not_a_section_number_and_does_not_abort_the_scan() {
        // `\d` is Unicode in the regex crate, so a fullwidth or Arabic-Indic digit run
        // matched the section patterns and reached a parse that expected ASCII. Neither is
        // a rule number.
        for text in [
            "per CR:１０４ fullwidth",
            "per rule ١٠٤ here",
            "CR:１０４.４b",
        ] {
            assert!(markers(text).is_empty(), "{text}");
            assert!(tokens(text).is_empty(), "{text}");
        }
    }

    #[test]
    fn a_bare_three_digit_number_is_not_a_token() {
        // Three digits with no keyword is a count, a line number, a date fragment. The
        // keyword is what makes the token claimable, and everything below it stays the
        // reviewer's.
        let noise = "the corpus holds 3 162 rules over 104 files";
        assert_eq!(tokens(noise), Vec::<String>::new(), "{noise}");
    }

    #[test]
    fn a_marker_inside_a_code_span_is_still_a_marker() {
        // Backticking is reflexive here, so `` `CR:x` `` reads as typography. Treating it as
        // data erased the claim entirely: no quote owed, no number resolved, nothing linted.
        let spanned = "the engine follows `CR:104.4b` here";
        assert_eq!(
            markers(spanned),
            vec![("104.4b".to_string(), MarkerForm::Prose)]
        );
        // A BARE number in a code span is still data — a sort key, a parser input.
        let bare = "the sort key is `104.4b`";
        assert!(scan_md(bare)
            .iter()
            .all(|o| !matches!(o, RuleObservation::Token(_))));
    }

    #[test]
    fn an_inline_html_tag_does_not_park_its_line() {
        // Only an HTML COMMENT parks text. Treating every inline tag as one erased every
        // citation, slug and path reference on a line carrying `Vec<Player>`.
        let tagged = "the engine stores a Vec<Player> and follows 104.4b here";
        assert!(
            scan_md(tagged)
                .iter()
                .any(|o| matches!(o, RuleObservation::Token(_))),
            "{:?}",
            scan_md(tagged)
        );
    }

    #[test]
    fn every_marked_number_is_also_a_bare_token() {
        // The lint asks whether a token on this line carries a marker, so both are recorded
        // and the comparison is left to the check.
        let text = "per CR:104.4b";
        let tokens: Vec<String> = scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                RuleObservation::Token(n) => Some(n.to_string()),
                _ => None,
            })
            .collect();
        assert_eq!(tokens, vec!["104.4b".to_string()]);
    }

    /// The slug definitions a text yields, as `(id, site)`.
    #[test]
    fn a_pin_is_read_from_the_whole_file() {
        let text = "intro\n<!-- cr-version: 20260807 -->\n";
        assert_eq!(
            pin(&crate::source::md::parse(text), text).as_deref(),
            Some("20260807")
        );
        const NONE: &str = "no pin here";
        assert_eq!(pin(&crate::source::md::parse(NONE), NONE), None);
    }

    #[test]
    fn a_pin_shown_inside_a_fence_is_an_illustration_and_binds_nothing() {
        // A document EXPLAINING the mechanism used to repin itself by showing the form, and
        // then verified its quotes against a release it never chose. The resolver fetches an
        // absent release, so the symptom was a network failure in a check that reads none.
        let text = "intro\n\n```markdown\n<!-- cr-version: 20260807 -->\n```\n";
        assert_eq!(pin(&crate::source::md::parse(text), text), None);
    }

    #[test]
    fn a_pin_inside_a_dropped_literal_binds_nothing_and_one_in_a_comment_does() {
        // The checker's own source: the fixture below is exactly the shape this file carries,
        // and read from the raw text it pinned this file to that release.
        use crate::source::{rs, Literals};
        let inside = "fn f() {\n    let s = \"<!-- cr-version: 20260807 -->\";\n}\n";
        assert_eq!(pin(&rs::parse(inside, Literals::Data), inside), None);
        assert_eq!(
            pin(&rs::parse(inside, Literals::Prose), inside).as_deref(),
            Some("20260807"),
            "outside the checker the raw text is read, as before"
        );
        let comment = "// <!-- cr-version: 20260807 -->\nfn f() {}\n";
        assert_eq!(
            pin(&rs::parse(comment, Literals::Data), comment).as_deref(),
            Some("20260807")
        );
    }
}
