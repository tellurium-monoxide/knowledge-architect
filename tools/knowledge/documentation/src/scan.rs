//! One pass over a document's prose, producing typed observations.
//!
//! Every check reads observations rather than text, so the tree is walked once and each file
//! is read once. What a check may see is exactly what this module chose to record.
//!
//! **The scanner sees prose and nothing else.** `source` decides which byte ranges of a file
//! are prose at all — the whole of a markdown document, and a Rust file's comments — so this
//! module never asks whether a line is code. The implementation it replaces answered that
//! from a prefix, and three recorded defects came out of the guesses.
//!
//! **A rule number inside a code span or a fence is data, not a citation.** A sort key, a
//! parser input, a line of tool output. References are the opposite case: the grammar puts
//! them inside backticks, so a code span is where they are expected and the exclusion must
//! not reach them.
//!
//! **The scanner tokenizes references and does not resolve them.** A backticked span holding
//! an `@` is recorded as written; whether its head is a kind or an anchor is a fact about the
//! project, which `entity::candidate` reads with the manifest in hand. What the scanner does
//! decide on its own is shape: a slug definition and where it sits, the two retired forms,
//! and the unanchored path lint, none of which needs the manifest.

use std::sync::LazyLock;

use regex::Regex;
use rules::RuleNumber;

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

/// Something a check might care about, found at a line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Observation {
    RuleMarker {
        number: RuleNumber,
        form: MarkerForm,
    },
    /// A rule-number-shaped token, marked or not. The lint's input: one carrying no marker
    /// on its own line probably wants a quote.
    RuleToken(RuleNumber),
    Heading {
        level: u8,
        text: String,
    },
    /// A slug written as `` `##<id>` `` in a shape that could define an entity, with where it
    /// sits.
    ///
    /// The id alone. Which anchor and which register it belongs to is where the document
    /// sits, which is a fact about the project rather than about the line; the entity table
    /// decides that, and decides that a slug at the wrong level or at a line head defines
    /// nothing.
    SlugDef {
        id: String,
        site: SlugSite,
    },
    /// A backticked span holding an `@` and no whitespace, as written: the input of the
    /// reference grammar `` `<kind>@<anchor>@<id>` ``.
    ///
    /// Recorded whatever its head is, because the scanner has no manifest to ask. An email
    /// address in backticks is one of these and is silent downstream, under the candidate
    /// rule in `entity::candidate`. A span holding an angle bracket is not one: a placeholder
    /// such as `<kind>@<anchor>@<id>` is how the grammar is illustrated.
    Span(String),
    /// A backticked span shaped like a path, with no `@`: the unanchored-path lint's input.
    ///
    /// Recorded rather than dropped, so the check can name the grammar: dropped, it would be
    /// a pointer no check resolves and no reader is told about, which is how the retired bare
    /// form dangled silently through one relocation.
    UnanchoredPath(String),
    /// One of the two reference forms the `@` grammar retired.
    ///
    /// Reported, never ignored: neither has an `@`, so without this a pointer the migration
    /// missed would be silent, which is the founding failure class.
    Retired(RetiredForm),
    /// A markdown link's target, as written.
    ///
    /// Resolution is the consumer's: a design README's naming check resolves it against the
    /// linking document's own directory, the way a renderer would.
    Link(String),
}

/// Where a `` `##<id>` `` was written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlugSite {
    /// At the end of a heading of this level.
    Heading(u8),
    /// In a table cell.
    Cell,
    /// At the head of a plain line: the form that predates the heading rule.
    LineHead,
}

/// A reference form the grammar retired.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetiredForm {
    /// `` `<word>#<word>` ``, the slug reference before kinds existed. The span as written.
    SlugRef(String),
    /// A bare `R` followed by digits, the interpretation entry number.
    RegisterNumber(u16),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// One-based, as an editor counts.
    pub line: u32,
    pub what: Observation,
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
static CR_SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bCR:(\d{3})\b").unwrap());
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
    Regex::new(r"(?i)\b(?:cr:[\s*_]*|(?:cr|rules?|sections?)[\s*_]+)(\d{3})\b").unwrap()
});
static HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,4})\s+(.+?)\s*$").unwrap());
/// The retired slug reference, `` `<word>#<word>` ``: the word before the `#` is optional so
/// that the older unqualified form is seen too. The first class cannot match a `#`, so a
/// definition — which opens with `##` — is not read as a retired reference to itself.
static RETIRED_SLUG_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`(\.?[A-Za-z0-9][A-Za-z0-9._-]*)?#([a-z0-9][a-z0-9-]{2,})`").unwrap()
});
/// A slug in a heading of ANY level, in a table cell, or at the head of a plain line, each
/// alternative in that order.
///
/// The heading form takes the slug anywhere in the heading, so the statement may precede it:
/// a heading reads as an outline entry, which a slug alone does not. Requiring text AFTER the
/// slug is what made a heading carrying nothing else invisible. Every level is matched so
/// that the entity table can report a level-one or level-four slug as misplaced rather than
/// see nothing; which levels define is its decision, not this pattern's.
static SLUG_SITE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:(#{1,4})\s+.*?`##([a-z0-9][a-z0-9-]{2,})`|\|\s*`##([a-z0-9][a-z0-9-]{2,})`|`##([a-z0-9][a-z0-9-]{2,})`)",
    )
    .unwrap()
});
/// A backticked span holding an `@`, no whitespace, no backtick and no angle bracket: the
/// reference grammar's input, recorded as written.
///
/// Permissive on purpose. A `..`, a `.` segment, a leading `/` or a wrong segment count
/// still parses as a span, so the check reports the cause rather than the span falling to
/// the unanchored lint with nothing naming what is wrong. An angle bracket excludes the span
/// because that is how a placeholder illustrates the grammar.
static AT_SPAN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`([^\s`<>@]*@[^\s`<>]*)`").unwrap());
/// A backticked span of path characters holding a slash and no `@`: the unanchored-path
/// lint's input.
///
/// A span holding a space, a colon or an angle bracket is not path-shaped, which is what
/// lets meta-notation like a bracketed placeholder document the syntax without a carve-out.
/// Requiring two segments is what keeps prose livable: a single segment with a trailing slash
/// names a nearby directory, the way a bare filename names a file, and neither is a pointer.
static PATH_SHAPED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`([\w.*+-]*/[\w.*/+-]*)`").unwrap());
/// The retired interpretation entry number: a bare `R` and up to three digits.
static RETIRED_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"R(\d{1,3})\b").unwrap());
/// A markdown link: `[text](target)`. The target may not hold a space or a closing
/// parenthesis, which is the shape every link in this tree has.
static MD_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[[^\]]*\]\(([^)\s]+)\)").unwrap());
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
    // it sat in to that release, per `knowledge#checker-source-literals-are-data`.
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

/// Scan one parsed document.
pub fn scan(parsed: &Parsed) -> Vec<Located> {
    let fenced: std::collections::HashSet<u32> = parsed.fenced.iter().copied().collect();
    let inert: std::collections::HashSet<u32> = parsed.inert.iter().copied().collect();
    let mut out = Vec::new();
    for region in &parsed.prose {
        let mut line_start = 0usize;
        for (i, line) in region.text.split('\n').enumerate() {
            let at = line_start;
            line_start += line.len() + 1;
            let n = region.file_line(i);
            // Commented out. Parking a section by wrapping it in an HTML comment left its
            // slug defined and its anchor pointing at text no reader sees.
            if inert.contains(&n) {
                continue;
            }
            let mut push = |what| out.push(Located { line: n, what });
            // Data, never a citation: a BARE rule number inside a code span is a value being
            // displayed — a sort key, a parser input, a line of tool output.
            //
            // **A MARKER is never data, wherever it sits.** It is explicit intent, and
            // applying this test to one made a backticked `CR:` marker erase its own claim:
            // no quote owed, no number resolved, nothing linted, and the file absent from the
            // bump work list. Backticking is reflexive here, so that shape reads as
            // typography.
            //
            // Slugs and paths are backticked BY CONVENTION and are never tested either.
            let data = |m: regex::Match| region.is_code(at + m.start());

            for c in CR_PROSE.captures_iter(line) {
                if let Some(number) = RuleNumber::parse(&c[1]) {
                    push(Observation::RuleMarker {
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
                    push(Observation::RuleMarker {
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
                push(Observation::RuleMarker {
                    number: number.clone(),
                    form: MarkerForm::Prose,
                });
                if !data(whole) {
                    push(Observation::RuleToken(number));
                }
            }
            for c in RULE_TOKEN.captures_iter(line) {
                if data(c.get(0).unwrap()) {
                    continue;
                }
                if let Some(n) = RuleNumber::parse(&c[1]) {
                    push(Observation::RuleToken(n));
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
                push(Observation::RuleToken(RuleNumber::section(
                    c[1].parse().expect("three digits"),
                )));
            }
            // A heading-shaped line inside a fence is an ILLUSTRATION. The generated index
            // labels each citation by the last heading seen, so one in a fenced example
            // relabelled every entry after it.
            if let Some(c) = HEADING.captures(line).filter(|_| !fenced.contains(&n)) {
                push(Observation::Heading {
                    level: c[1].len() as u8,
                    text: c[2].to_string(),
                });
            }
            // A FENCE suppresses slug definitions and markdown links, and nothing else. A
            // definition site is a heading, and a fenced heading is an illustration, so a
            // fenced slug neither defines nor is misplaced. A fenced link is the same case
            // for the design README's index.
            //
            // It does NOT suppress references, in any form: a sketch names what it names on
            // purpose, and an illustration writes a placeholder in angle brackets. It does not
            // suppress rule numbers either — a fenced sketch in a design document comments
            // its rules on purpose, and reading those as data lost 34 citations in this tree.
            let illustration = fenced.contains(&n);
            if region.structural && !illustration {
                // A slug DEFINES an entity, and an entity's home is a register home under
                // an anchor. A source comment is not one, so a slug written there defines
                // nothing — while a REFERENCE from a source comment is ordinary and is
                // scanned below.
                if let Some(c) = SLUG_SITE.captures(line.trim_start()) {
                    let (id, site) = if let Some(id) = c.get(2) {
                        (id, SlugSite::Heading(c[1].len() as u8))
                    } else if let Some(id) = c.get(3) {
                        (id, SlugSite::Cell)
                    } else {
                        (c.get(4).unwrap(), SlugSite::LineHead)
                    };
                    push(Observation::SlugDef {
                        id: id.as_str().to_string(),
                        site,
                    });
                }
            }
            // The retired entry number, in every prose region. The observation it replaces
            // read markdown alone, so a number in a Rust comment was checked by nothing;
            // a comment is prose, and a type parameter is code the scanner never sees.
            for c in RETIRED_R.captures_iter(line) {
                let start = c.get(0).unwrap().start();
                let before = line[..start].chars().next_back();
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                    continue;
                }
                if let Ok(n) = c[1].parse() {
                    push(Observation::Retired(RetiredForm::RegisterNumber(n)));
                }
            }
            // A markdown link is a pointer a renderer follows, recorded as written. A fenced
            // link is an illustration, like a fenced slug, and one inside a code span is
            // typography showing the shape: the design README naming check reads links, and
            // an example must neither discharge a real obligation nor owe a real target.
            for c in MD_LINK.captures_iter(line) {
                if illustration || data(c.get(0).unwrap()) {
                    continue;
                }
                push(Observation::Link(c[1].to_string()));
            }
            for c in RETIRED_SLUG_REF.captures_iter(line) {
                let whole = c.get(0).unwrap().as_str();
                push(Observation::Retired(RetiredForm::SlugRef(
                    whole.trim_matches('`').to_string(),
                )));
            }
            for c in AT_SPAN.captures_iter(line) {
                push(Observation::Span(c[1].to_string()));
            }
            // The two classes are disjoint on `@`, so a span is one or the other and never
            // both.
            for c in PATH_SHAPED.captures_iter(line) {
                if c[1].split('/').filter(|s| !s.is_empty()).count() < 2 {
                    continue;
                }
                push(Observation::UnanchoredPath(c[1].to_string()));
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
                out.push(Located {
                    line: *line,
                    what: Observation::RuleMarker {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture below is written as the bytes it means. The checker reads no string
    // literal of its own source, per `knowledge#checker-source-literals-are-data`, so a rule
    // number, a marker, a slug or a path in one is data and not a claim.

    fn scan_md(text: &str) -> Vec<Observation> {
        scan(&crate::source::md::parse(text))
            .into_iter()
            .map(|l| l.what)
            .collect()
    }

    /// The same, over a Rust file, so a test can say which conventions reach source comments.
    fn scan_rs(text: &str) -> Vec<Observation> {
        scan(&crate::source::rs::parse(
            text,
            crate::source::Literals::Prose,
        ))
        .into_iter()
        .map(|l| l.what)
        .collect()
    }

    fn markers(text: &str) -> Vec<(String, MarkerForm)> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::RuleMarker { number, form } => Some((number.to_string(), form)),
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
                Observation::RuleMarker { number, form } => Some((number.to_string(), form)),
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
        let seen: Vec<Observation> = scan(&crate::source::rs::parse(
            TOO_LONG,
            crate::source::Literals::Prose,
        ))
        .into_iter()
        .map(|l| l.what)
        .collect();
        assert!(
            !seen
                .iter()
                .any(|o| matches!(o, Observation::RuleMarker { .. })),
            "{seen:?}"
        );
        // The control: two letters IS a rule number, and must still be found.
        const OK: &str = "fn cr_613_8c_holds() {}";
        assert!(scan(&crate::source::rs::parse(
            OK,
            crate::source::Literals::Prose
        ))
        .into_iter()
        .any(|l| matches!(l.what, Observation::RuleMarker { .. })));
    }

    /// The tokens a line yields, as printed text.
    fn tokens(text: &str) -> Vec<String> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::RuleToken(n) => Some(n.to_string()),
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
    fn a_bare_three_digit_number_is_not_a_token() {
        // Three digits with no keyword is a count, a line number, a date fragment. The
        // keyword is what makes the token claimable, and everything below it stays the
        // reviewer's.
        let noise = "the corpus holds 3 162 rules over 104 files";
        assert_eq!(tokens(noise), Vec::<String>::new(), "{noise}");
    }

    #[test]
    fn a_heading_inside_a_fence_is_an_illustration() {
        // The generated index labels each citation by the last heading seen, so a
        // heading-shaped line in a fenced example relabelled every entry after it. Two live
        // sites in this repository showed one.
        let fenced = "```sh\n# not a heading, a shell comment\n```\n";
        assert!(
            !scan_md(fenced)
                .iter()
                .any(|o| matches!(o, Observation::Heading { .. })),
            "{:?}",
            scan_md(fenced)
        );
        assert!(scan_md("# A heading\n")
            .iter()
            .any(|o| matches!(o, Observation::Heading { .. })));
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
            .all(|o| !matches!(o, Observation::RuleToken(_))));
    }

    #[test]
    fn an_inline_html_tag_does_not_park_its_line() {
        // Only an HTML COMMENT parks text. Treating every inline tag as one erased every
        // citation, slug and path reference on a line carrying `Vec<Player>`.
        let tagged = "the engine stores a Vec<Player> and follows 104.4b here";
        assert!(
            scan_md(tagged)
                .iter()
                .any(|o| matches!(o, Observation::RuleToken(_))),
            "{:?}",
            scan_md(tagged)
        );
    }

    #[test]
    fn a_slug_inside_an_html_comment_defines_nothing() {
        // Parking a section by commenting it out left its anchor defined and pointing at
        // text no reader sees.
        let parked = "<!--\n### Parked `##a-slug`\n-->\n";
        assert!(
            !scan_md(parked)
                .iter()
                .any(|o| matches!(o, Observation::SlugDef { .. })),
            "{:?}",
            scan_md(parked)
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
                Observation::RuleToken(n) => Some(n.to_string()),
                _ => None,
            })
            .collect();
        assert_eq!(tokens, vec!["104.4b".to_string()]);
    }

    /// The slug definitions a text yields, as `(id, site)`.
    fn defs(text: &str) -> Vec<(String, SlugSite)> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::SlugDef { id, site } => Some((id, site)),
                _ => None,
            })
            .collect()
    }

    /// The `@` spans a text yields, as written.
    fn spans(text: &str) -> Vec<String> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::Span(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    /// The retired forms a text yields.
    fn retired(text: &str) -> Vec<RetiredForm> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::Retired(r) => Some(r),
                _ => None,
            })
            .collect()
    }

    /// The unanchored path-shaped spans a text yields.
    fn unanchored(text: &str) -> Vec<String> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::UnanchoredPath(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_slug_is_recorded_with_its_site_at_every_heading_level_a_cell_and_a_line_head() {
        // The scanner records WHERE the slug sits and decides nothing about whether it
        // defines: that is the entity table's, which reports a level-one or level-four slug
        // as misplaced rather than seeing nothing.
        for level in 1..=4u8 {
            let head = format!("{} The statement `##a-slug`", "#".repeat(level as usize));
            assert_eq!(
                defs(&head),
                vec![("a-slug".to_string(), SlugSite::Heading(level))],
                "{head}"
            );
        }
        // A heading carrying nothing but the slug is recorded too. Requiring text after it
        // is what made every such heading invisible.
        assert_eq!(
            defs("### `##a-slug`"),
            vec![("a-slug".to_string(), SlugSite::Heading(3))]
        );
        assert_eq!(
            defs("| `##in-a-cell` | holds |"),
            vec![("in-a-cell".to_string(), SlugSite::Cell)]
        );
        // The form that predates the heading rule, recorded so the table can report it.
        assert_eq!(
            defs("`##a-slug` — **The statement.**"),
            vec![("a-slug".to_string(), SlugSite::LineHead)]
        );
        // Mid-sentence it is typography showing the shape, and nothing is recorded.
        assert_eq!(defs("write it as `##a-slug` at the end"), Vec::new());
    }

    #[test]
    fn a_fence_suppresses_a_definition_and_leaves_every_reference_live() {
        // A fenced heading is an illustration, so a fenced slug defines nothing and is not
        // misplaced either. A fenced REFERENCE is live for every kind, the retired forms
        // included: a sketch names what it names on purpose, and an illustration writes a
        // placeholder in angle brackets. Mutation checked: dropping the `illustration` test
        // from the definition arm records the fenced heading as a definition.
        let fenced = "before\n```\n### A head `##a-slug`\n`design@a-component@a-slug` and \
                      `a-component#a-slug` and R15 and `docs/a.md`\n```\nafter\n";
        assert_eq!(defs(fenced), Vec::new(), "{:#?}", scan_md(fenced));
        assert_eq!(spans(fenced), vec!["design@a-component@a-slug".to_string()]);
        let seen = retired(fenced);
        assert_eq!(seen.len(), 2, "{seen:?}");
        assert!(seen.contains(&RetiredForm::SlugRef("a-component#a-slug".to_string())));
        assert!(seen.contains(&RetiredForm::RegisterNumber(15)));
        assert_eq!(unanchored(fenced), vec!["docs/a.md".to_string()]);
        // The same heading outside the fence is a definition.
        assert_eq!(defs("### A head `##a-slug`\n").len(), 1);
    }

    #[test]
    fn a_source_comment_carries_references_and_defines_nothing() {
        // A slug's home is a register home under an anchor, so a source comment may point
        // at an entity and may not define one. The reference still has to be found, because
        // that is how a doc comment cites the argument for the code under it.
        let src =
            "/// argued at `design@a-component@a-slug`\n/// ### stated `##a-slug`\nfn f() {}\n";
        let seen = scan_rs(src);
        assert!(seen.contains(&Observation::Span("design@a-component@a-slug".to_string())));
        assert!(!seen
            .iter()
            .any(|o| matches!(o, Observation::SlugDef { .. })));
    }

    #[test]
    fn a_reference_in_a_bound_string_literal_is_not_one_and_a_message_is() {
        // Bound to a name it is a fixture; in a call it is a message a human reads.
        let bound = "fn f() {\n    let n = \"design@a-component@a-slug\";\n}\n";
        assert!(scan_rs(bound).is_empty(), "{:?}", scan_rs(bound));
        let message = "fn f() {\n    g(\"see `design@a-component@a-slug`\");\n}\n";
        assert!(
            scan_rs(message).contains(&Observation::Span("design@a-component@a-slug".to_string())),
            "{:?}",
            scan_rs(message)
        );
    }

    #[test]
    fn the_retired_slug_form_is_recorded_with_and_without_its_word_and_a_definition_is_not() {
        // The two shapes the retired pattern had, each reported so the migration is visible;
        // a definition opens with a second `#` and matches neither.
        assert_eq!(
            retired("as `a-component#a-slug` and `#a-slug` record"),
            vec![
                RetiredForm::SlugRef("a-component#a-slug".to_string()),
                RetiredForm::SlugRef("#a-slug".to_string()),
            ]
        );
        for line in [
            "### The statement `##a-slug`",
            "| `##in-a-cell` | holds |",
            "`##a-slug`  **The statement.**",
        ] {
            assert_eq!(retired(line), Vec::new(), "{line}");
        }
        // A placeholder in angle brackets is an illustration of the retired form too.
        assert_eq!(retired("the shape `<word>#<word>` is retired"), Vec::new());
    }

    #[test]
    fn the_retired_entry_number_is_a_bare_r_and_digits_outside_a_word() {
        let text = "R15 holds, but CR:104.4b and FOR15 and 1.R3 and R2D2 do not";
        assert_eq!(retired(text), vec![RetiredForm::RegisterNumber(15)]);
        // In a Rust comment as in markdown: a comment is prose, and a number there names
        // the same retired entry. Code is not prose, so a type parameter is not read.
        let comment = scan_rs("/// per R15\nfn f<R15>() {}\n");
        assert_eq!(
            comment
                .iter()
                .filter(|o| matches!(o, Observation::Retired(_)))
                .count(),
            1,
            "{comment:?}"
        );
        assert!(scan_rs("fn f<R15>() {}\n").is_empty());
    }

    #[test]
    fn a_backticked_span_holding_an_at_sign_is_recorded_as_written_whatever_its_head() {
        // The scanner has no manifest, so it cannot tell a kind from an email address; the
        // candidate rule downstream does. The trailing slash on the second is kept: it is
        // the writer's claim that the target is a directory, and the check asserts it.
        let text = "see `design@a-component@a-slug` and `path@a-component@docs/design/` and \
                    `path@*@docs/design/a.md` and `path@elsewhere@x/y` and \
                    `a-component@docs/a.md` and `user@example.test` and `@docs/a.md`";
        assert_eq!(
            spans(text),
            vec![
                "design@a-component@a-slug".to_string(),
                "path@a-component@docs/design/".to_string(),
                "path@*@docs/design/a.md".to_string(),
                "path@elsewhere@x/y".to_string(),
                "a-component@docs/a.md".to_string(),
                "user@example.test".to_string(),
                "@docs/a.md".to_string(),
            ]
        );
        assert_eq!(
            unanchored(text),
            Vec::<String>::new(),
            "a span is one or the other"
        );
    }

    #[test]
    fn a_span_with_whitespace_or_an_angle_bracket_is_not_recorded() {
        // A placeholder is how the grammar is illustrated, and a sentence in backticks is
        // not a pointer.
        let text = "write `<kind>@<anchor>@<id>` or `path@<component>@docs/a.md`, \
                    not `see the a@b form`";
        assert_eq!(spans(text), Vec::<String>::new());
        assert_eq!(unanchored(text), Vec::<String>::new());
    }

    #[test]
    fn a_bare_two_segment_span_is_an_unanchored_path_shape() {
        // The retired bare form and a dotted upward path: each is recorded so the check can
        // name the grammar, never dropped.
        for span in ["docs/design/a.md", "../../docs/design/a.md", "a/b.json"] {
            let text = format!("see `{span}`");
            assert_eq!(unanchored(&text), vec![span.to_string()], "{span}");
            assert_eq!(spans(&text), Vec::<String>::new(), "{span}");
        }
    }

    #[test]
    fn a_single_segment_span_is_a_name_rather_than_a_pointer() {
        // A directory named beside its slash and a bare filename are prose naming a thing.
        // Flagging them would bury the real dangling references under noise.
        let text = "see `past/` and `citations.py` and `a/`";
        assert_eq!(unanchored(text), Vec::<String>::new());
        assert_eq!(spans(text), Vec::<String>::new());
    }

    #[test]
    fn meta_notation_is_not_path_shaped() {
        // A space, a colon or an angle bracket puts a span outside the path class, which is
        // what lets documentation of the syntax show a placeholder without a carve-out.
        let text = "write `path@<component>@docs/design/a.md` or `see the docs/design/a.md form` \
                    or `https://a.test/docs/design/a.md`";
        assert_eq!(unanchored(text), Vec::<String>::new());
        assert_eq!(spans(text), Vec::<String>::new());
    }

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
