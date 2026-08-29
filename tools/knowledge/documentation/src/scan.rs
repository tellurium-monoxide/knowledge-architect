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
//! parser input, a line of tool output. Slug and path references are the opposite case: the
//! conventions put them inside backticks, so a code span is where they are expected and the
//! exclusion must not reach them.

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
    /// A slug at the head of a line or a table cell, opening a decision.
    ///
    /// The name alone. Which component it belongs to is where the document sits, which is a
    /// fact about the project rather than about the line.
    SlugDef(String),
    /// A slug anywhere else: a pointer at a decision, in the component that defines it.
    SlugRef {
        /// The component named before the `#`, and `None` where the reference names none.
        ///
        /// A slug is unique inside its component and not across them, so a reference naming
        /// no component resolves to nothing. It is recorded rather than dropped: dropped, it
        /// would be a pointer that no check can see and no reader is told about.
        component: Option<String>,
        slug: String,
    },
    /// A repository-relative path named in prose.
    PathRef {
        /// The component named before the `@`
        component: Option<String>,
        path: String,
    },
    /// An `R` number naming an interpretation entry.
    InterpRef(u16),
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
static SECTION_KEYWORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:CR|[Rr]ules?|[Ss]ections?)\s+(\d{3})\b").unwrap());
static HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,4})\s+(.+?)\s*$").unwrap());
/// A reference is `` `<component>#<slug>` ``, and the component is optional only so that one
/// written without it is still seen. The component alternative cannot match a `#`, so a
/// definition — which opens with `##` — is not read as a reference to a slug of its own.
static SLUG_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`(\.?[A-Za-z0-9][A-Za-z0-9._-]*)?#([a-z0-9][a-z0-9-]{2,})`").unwrap()
});
/// The shape a component name must have for a reference to be able to name it.
///
/// The same character class the reference pattern accepts, anchored. A component whose name a
/// reference cannot spell is one every pointer at it misses silently, so the check over the
/// declaration asks this rather than assuming it.
static COMPONENT_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\.?[A-Za-z0-9][A-Za-z0-9._-]*$").unwrap());
/// A slug is DEFINED in a level-three heading, or in a table cell. Anywhere else it is a
/// reference.
///
/// **The heading form takes the slug anywhere in the heading**, so the statement may precede
/// it: `### The catalog is a question` followed by the slug reads as an outline entry, which
/// a slug alone does not. Requiring text AFTER the slug is what made a heading carrying
/// nothing else invisible, and every decision written that way was read as a reference to an
/// anchor nobody defined.
static SLUG_DEF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\|\s*`##([a-z0-9][a-z0-9-]{2,})`|###\s+.*?`##([a-z0-9][a-z0-9-]{2,})`)")
        .unwrap()
});
/// Only things shaped like a path = containing a slash. A bare filename in prose is a name,
/// not a pointer, and flagging those would bury the real dangling references under noise.
/// Temporary ignored, will come back to check that it is no longer used.
static PATH_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`((?:\.\.?/)?(?:[\w.-]+/)+[\w.-]+\.(?:md|txt|py|sh|tsv))`").unwrap()
});
/// Path relative to a component, syntax `<component>@path/to/file`
static COMPONENT_PATH_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`(\.?[A-Za-z0-9][A-Za-z0-9._-]*)@(/?(?:[\w.+-]+/)*[\w.+-]+/?)`").unwrap()
});
static INTERP_REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"R(\d{1,3})\b").unwrap());
static PIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!--\s*cr-version:\s*(\d{8})\s*-->").unwrap());

/// The release a file's quotes verify against: its own pin, or `None` for the vendored one.
///
/// **An illustration of the syntax is not a pin.** This runs over the raw file, so a document
/// EXPLAINING the mechanism — a README, a plan, a skill — used to repin itself by showing the
/// form in a fenced example, and then verified its quotes against a release it never chose. The
/// resolver fetches an absent release over the network, so the symptom was a `curl` failure in
/// a check that reads no network for anything else.
pub fn pin(parsed: &Parsed, text: &str) -> Option<String> {
    let fenced: std::collections::HashSet<u32> = parsed.fenced.iter().copied().collect();
    for (i, line) in text.lines().enumerate() {
        let n = i as u32 + 1;
        // Fenced only. The pin IS an HTML comment, so skipping the inert lines would find
        // no pin anywhere; what matters is that an ILLUSTRATION of one does not bind.
        if fenced.contains(&n) {
            continue;
        }
        if let Some(c) = PIN.captures(line) {
            return Some(c[1].to_string());
        }
    }
    None
}

/// Can a slug reference name a component called this.
///
/// Asked of a declared component name, so that a name no pointer can spell is reported where
/// it is declared rather than found later as a reference that resolves to nothing.
pub fn is_component_name(name: &str) -> bool {
    COMPONENT_NAME.is_match(name)
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
            for c in CR_SECTION.captures_iter(line) {
                let whole = c.get(0).unwrap();
                if continues_dotted(whole) {
                    continue;
                }
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
            // A FENCE suppresses the slug conventions and nothing else. Every document that
            // explains how to write a reference has to hold one, and there is no way to write
            // the illustration that is not a finding otherwise: unqualified it names no
            // component, and qualified it names a slug the example invented.
            //
            // It does NOT suppress rule numbers. A fenced sketch in a design document comments
            // its rules on purpose, and reading those as data lost 34 citations in this tree.
            let illustration = fenced.contains(&n);
            if region.structural && !illustration {
                // A slug DEFINES a decision, and a decision's home is a component's design
                // document. A source comment is not one, so a slug written there defines
                // nothing — while a REFERENCE from a source comment is ordinary and is
                // scanned below.
                if let Some(c) = SLUG_DEF.captures(line.trim_start()) {
                    let slug = c.get(1).or_else(|| c.get(2)).unwrap().as_str();
                    push(Observation::SlugDef(slug.to_string()));
                }
            }
            // Interpretation numbers are NOT guarded by the fence, deliberately: one needs no
            // delimiter to be found, none has been seen to fire on an example, and the
            // register's index is built from them.
            if region.structural {
                for c in INTERP_REF.captures_iter(line) {
                    let start = c.get(0).unwrap().start();
                    let before = line[..start].chars().next_back();
                    if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                        continue;
                    }
                    if let Ok(n) = c[1].parse() {
                        push(Observation::InterpRef(n));
                    }
                }
            }
            for c in SLUG_REF.captures_iter(line) {
                if illustration {
                    continue;
                }
                push(Observation::SlugRef {
                    component: c.get(1).map(|m| m.as_str().to_string()),
                    slug: c[2].to_string(),
                });
            }
            for c in COMPONENT_PATH_REF.captures_iter(line) {
                push(Observation::PathRef {
                    component: Some(c[1].to_string()),
                    path: c[2].to_string(),
                });
            }
            for c in PATH_REF.captures_iter(line) {
                push(Observation::PathRef {
                    component: None,
                    path: c[1].to_string(),
                });
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

    // Inputs to a scanner, bound to names.
    const RULE: &str = "104.4b";
    const OTHER: &str = "613.8c";

    fn scan_md(text: &str) -> Vec<Observation> {
        scan(&crate::source::md::parse(text))
            .into_iter()
            .map(|l| l.what)
            .collect()
    }

    /// The same, over a Rust file, so a test can say which conventions reach source comments.
    fn scan_rs(text: &str) -> Vec<Observation> {
        scan(&crate::source::rs::parse(text))
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
        // a number that is data goes in a code span, a fence or a name-bound literal.
        let text = format!("per CR:{RULE}, and {OTHER} is only a number");
        assert_eq!(markers(&text), vec![(RULE.to_string(), MarkerForm::Prose)]);
    }

    #[test]
    fn an_identifier_marker_is_found_anywhere_in_a_name() {
        let ident = RULE.replace('.', "_");
        for name in [
            format!("fn CR_{ident}_holds() {{}}"),
            // Prefixed. An underscore is a word character, so a bounded pattern never matched
            // here — and `test_…` is the commonest way to write the shape the convention
            // exists for, which left it unenforced for exactly that shape.
            format!("fn test_cr_{ident}_holds() {{}}"),
        ] {
            let seen: Vec<(String, MarkerForm)> = scan(&crate::source::rs::parse(&name))
                .into_iter()
                .filter_map(|l| match l.what {
                    Observation::RuleMarker { number, form } => Some((number.to_string(), form)),
                    _ => None,
                })
                .collect();
            assert_eq!(
                seen,
                vec![(RULE.to_string(), MarkerForm::Identifier)],
                "{name}"
            );
        }
    }

    #[test]
    fn an_identifier_marker_in_prose_is_told_apart_from_one_in_a_name() {
        // Only the second is what the convention exists for. A check that conflated them
        // would ask a sentence for the quote a name owes.
        let ident = RULE.replace('.', "_");
        let prose = format!("the test cr_{ident}_holds covers it");
        assert_eq!(
            markers(&prose),
            vec![(RULE.to_string(), MarkerForm::IdentifierInProse)]
        );
    }

    #[test]
    fn an_identifier_marker_with_too_long_a_suffix_is_not_one() {
        // Through `rs::parse`, not `scan_md`. Scanned as MARKDOWN this reached neither
        // identifier pattern, so it passed with both suffix guards removed and pinned
        // nothing at all — while reading as though it covered names.
        const TOO_LONG: &str = "fn cr_613_8cde_holds() {}";
        let seen: Vec<Observation> = scan(&crate::source::rs::parse(TOO_LONG))
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
        assert!(scan(&crate::source::rs::parse(OK))
            .into_iter()
            .any(|l| matches!(l.what, Observation::RuleMarker { .. })));
    }

    // A section number and its dotted rules, as inputs to the scanner.
    const SECTION: &str = "104";
    const IN_SECTION: &str = "104.4b";

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
        let section = format!("in the order CR:{SECTION} states them");
        assert_eq!(
            markers(&section),
            vec![(SECTION.to_string(), MarkerForm::Prose)]
        );
        assert_eq!(tokens(&section), vec![SECTION.to_string()]);
        let dotted = format!("per CR:{IN_SECTION}, the step is pinned");
        assert_eq!(
            markers(&dotted),
            vec![(IN_SECTION.to_string(), MarkerForm::Prose)]
        );
        // At a sentence end the dot belongs to the sentence, not the number.
        let sentence = format!("the order is CR:{SECTION}. The next sentence.");
        assert_eq!(
            markers(&sentence),
            vec![(SECTION.to_string(), MarkerForm::Prose)]
        );
    }

    #[test]
    fn a_keyword_section_reference_is_a_token_and_never_a_marker() {
        // The keyword form is the lint's input: a token with no marker on its line is what
        // the missing-marker lint reports. A dotted number after the keyword is a rule
        // reference the dotted patterns already carry, so the keyword match is discarded.
        for keyword in ["CR", "rule", "Rule", "rules", "section", "Sections"] {
            let text = format!("named by {keyword} {SECTION} in prose");
            assert_eq!(tokens(&text), vec![SECTION.to_string()], "{keyword}");
            assert!(markers(&text).is_empty(), "{keyword}");
        }
        let dotted = format!("named by rule {IN_SECTION} in prose");
        assert_eq!(tokens(&dotted), vec![IN_SECTION.to_string()]);
    }

    #[test]
    fn a_keyword_section_number_in_a_code_span_is_data() {
        // The data carve-outs survive the section form: inside backticks the number is being
        // displayed, and whether that shelters a claim is the reviewer's question, not a
        // pattern's.
        let sheltered = format!("named by rule `{SECTION}` in a code span");
        assert_eq!(tokens(&sheltered), Vec::<String>::new());
        // A backticked MARKER stays a marker — markers are never data — and its token keeps
        // the dotted symmetry by staying out.
        let marker = format!("named by `CR:{SECTION}` in a code span");
        assert_eq!(
            markers(&marker),
            vec![(SECTION.to_string(), MarkerForm::Prose)]
        );
        assert_eq!(tokens(&marker), Vec::<String>::new());
    }

    #[test]
    fn a_bare_three_digit_number_is_not_a_token() {
        // Three digits with no keyword is a count, a line number, a date fragment. The
        // keyword is what makes the token claimable, and everything below it stays the
        // reviewer's.
        let noise = format!("the corpus holds 3 162 rules over {SECTION} files");
        assert_eq!(tokens(&noise), Vec::<String>::new(), "{noise}");
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
        let spanned = format!("the engine follows `CR:{RULE}` here");
        assert_eq!(
            markers(&spanned),
            vec![(RULE.to_string(), MarkerForm::Prose)]
        );
        // A BARE number in a code span is still data — a sort key, a parser input.
        let bare = format!("the sort key is `{RULE}`");
        assert!(scan_md(&bare)
            .iter()
            .all(|o| !matches!(o, Observation::RuleToken(_))));
    }

    #[test]
    fn an_inline_html_tag_does_not_park_its_line() {
        // Only an HTML COMMENT parks text. Treating every inline tag as one erased every
        // citation, slug and path reference on a line carrying `Vec<Player>`.
        let tagged = format!("the engine stores a Vec<Player> and follows {RULE} here");
        assert!(
            scan_md(&tagged)
                .iter()
                .any(|o| matches!(o, Observation::RuleToken(_))),
            "{:?}",
            scan_md(&tagged)
        );
    }

    #[test]
    fn a_slug_inside_an_html_comment_defines_nothing() {
        // Parking a section by commenting it out left its anchor defined and pointing at
        // text no reader sees.
        let parked = format!("<!--\n### Parked `##{SLUG}`\n-->\n");
        assert!(
            !scan_md(&parked)
                .iter()
                .any(|o| matches!(o, Observation::SlugDef(_))),
            "{:?}",
            scan_md(&parked)
        );
    }

    #[test]
    fn every_marked_number_is_also_a_bare_token() {
        // The lint asks whether a token on this line carries a marker, so both are recorded
        // and the comparison is left to the check.
        let text = format!("per CR:{RULE}");
        let tokens: Vec<String> = scan_md(&text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::RuleToken(n) => Some(n.to_string()),
                _ => None,
            })
            .collect();
        assert_eq!(tokens, vec![RULE.to_string()]);
    }

    // Fixture slugs and paths are INTERPOLATED into their fixtures, never written out. A
    // checker that walks the whole tree cannot tell its own test data from a document, so a
    // slug spelled here would define an anchor and a path spelled here would be a reference
    // to a file that does not exist — both of which this tool's own checks then report.
    const SLUG: &str = "a-slug";
    const CELL_SLUG: &str = "in-a-cell";
    const DOC_PATH: &str = "docs/design/a.md";

    const COMPONENT: &str = "a-component";

    /// The reference a line holds, as `(component, slug)`.
    fn refs(text: &str) -> Vec<(Option<String>, String)> {
        scan_md(text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::SlugRef { component, slug } => Some((component, slug)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_slug_opens_a_decision_in_a_level_three_heading_or_in_a_table_cell() {
        // The statement precedes the slug, which is what makes the heading an outline entry.
        let stated = format!("### The statement `##{SLUG}`");
        assert!(scan_md(&stated).contains(&Observation::SlugDef(SLUG.into())));
        // A heading carrying nothing but the slug is a definition too. Requiring text after
        // it is what made every such heading invisible.
        let alone = format!("### `##{SLUG}`");
        assert!(scan_md(&alone).contains(&Observation::SlugDef(SLUG.into())));
        let cell = format!("| `##{CELL_SLUG}` | holds |");
        assert!(scan_md(&cell).contains(&Observation::SlugDef(CELL_SLUG.into())));
        // Mid-sentence it is a reference, not a definition.
        let mid = format!("as `{COMPONENT}#{SLUG}` records");
        assert_eq!(
            refs(&mid),
            vec![(Some(COMPONENT.to_string()), SLUG.to_string())]
        );
        assert!(!scan_md(&mid)
            .iter()
            .any(|o| matches!(o, Observation::SlugDef(_))));
    }

    #[test]
    fn a_slug_at_a_bare_line_head_is_no_longer_a_definition() {
        // The form that predates the heading rule. It defines nothing, so every pointer at
        // it is reported as dangling — which is how the migration is visible rather than
        // silent.
        let old = format!("`##{SLUG}` — **The statement.**");
        assert!(!scan_md(&old)
            .iter()
            .any(|o| matches!(o, Observation::SlugDef(_))));
    }

    #[test]
    fn a_slug_in_a_deeper_or_shallower_heading_is_not_a_definition() {
        for level in ["##", "####"] {
            let head = format!("{level} The statement `##{SLUG}`");
            assert!(
                !scan_md(&head)
                    .iter()
                    .any(|o| matches!(o, Observation::SlugDef(_))),
                "level {level} must not define"
            );
        }
    }

    #[test]
    fn a_slug_inside_a_fence_is_an_illustration_rather_than_a_pointer() {
        // Neither form is a reference there, and the same two forms outside the fence are.
        // Without this, a document explaining the convention cannot hold an example of it:
        // unqualified names no component, and qualified names a slug the example invented.
        let fenced = format!(
            "before\n```\n`{COMPONENT}#{SLUG}` and `#{SLUG}` and `##{SLUG}` — **A head.**\n```\nafter\n"
        );
        assert!(refs(&fenced).is_empty(), "{:#?}", refs(&fenced));
        assert!(!scan_md(&fenced)
            .iter()
            .any(|o| matches!(o, Observation::SlugDef(_))));
        // The fence is what does it, not the line: the same line outside one is both.
        let open = format!("`{COMPONENT}#{SLUG}` and `#{SLUG}`\n");
        assert_eq!(
            refs(&open),
            vec![
                (Some(COMPONENT.to_string()), SLUG.to_string()),
                (None, SLUG.to_string()),
            ]
        );
    }

    #[test]
    fn a_source_comment_carries_pointers_but_defines_no_decision() {
        // A slug's home is a component's design document, so a source comment may point at a
        // decision and may not open one. The reference still has to be found, because that is
        // how a doc comment cites the argument for the code under it.
        let src =
            format!("/// argued at `{COMPONENT}#{SLUG}`\n/// ### stated `##{SLUG}`\nfn f() {{}}\n");
        let seen = scan_rs(&src);
        assert!(seen.contains(&Observation::SlugRef {
            component: Some(COMPONENT.to_string()),
            slug: SLUG.to_string(),
        }));
        assert!(!seen.iter().any(|o| matches!(o, Observation::SlugDef(_))));
    }

    #[test]
    fn a_rule_number_in_a_string_literal_is_not_a_citation() {
        // The recorded fixture problem: this tool walks its own source, so a rule number
        // written as test data was live content and had to be interpolated to hide it.
        let src = format!("fn f() {{\n    let n = \"{RULE}\";\n}}\n");
        assert!(scan_rs(&src).is_empty(), "{:?}", scan_rs(&src));
    }

    #[test]
    fn a_rule_number_in_a_code_span_is_data_rather_than_a_citation() {
        let live = format!("The sort key is {RULE} in prose.");
        assert!(scan_md(&live)
            .iter()
            .any(|o| matches!(o, Observation::RuleToken(_))));
        let data = format!("The sort key is `{RULE}` in a code span.");
        assert!(
            !scan_md(&data)
                .iter()
                .any(|o| matches!(o, Observation::RuleToken(_))),
            "{:?}",
            scan_md(&data)
        );
    }

    #[test]
    fn a_definition_is_not_also_read_as_a_reference_to_itself() {
        // The reference pattern's component part is optional, so a definition — which opens
        // with a second `#` — could match it with no component at all. It must not: every
        // decision head in the project would then be a reference naming no component, and the
        // check that reports those would fire on all of them at once.
        for line in [
            format!("`##{SLUG}`  **The statement.**"),
            format!("| `##{CELL_SLUG}` | holds |"),
        ] {
            assert!(refs(&line).is_empty(), "{line}");
        }
    }

    #[test]
    fn a_reference_naming_no_component_is_recorded_rather_than_dropped() {
        // Dropped, it would be a pointer no check can see: nothing resolves it and nothing
        // tells the writer it resolves to nothing.
        assert_eq!(
            refs(&format!("as `#{SLUG}` records")),
            vec![(None, SLUG.to_string())]
        );
    }

    #[test]
    fn a_component_name_is_referenceable_exactly_when_a_reference_can_spell_it() {
        // Two patterns state the same character class, and a name accepted by one and not the
        // other is a component every pointer at it misses in silence.
        for name in [
            "a-tool",
            "an.engine",
            ".claude",
            "an_engine",
            "9lives",
            "-leading-dash",
            "has/slash",
            "has space",
            "",
        ] {
            let scanned = refs(&format!("see `{name}#{SLUG}`"));
            let spelled = scanned == vec![(Some(name.to_string()), SLUG.to_string())];
            assert_eq!(
                is_component_name(name),
                spelled,
                "{name:?} scanned as {scanned:?}"
            );
        }
    }

    #[test]
    fn a_path_needs_a_slash_and_a_known_suffix() {
        let text = format!("see `component@{DOC_PATH}` and `citations.py` and `a/b.json`");
        let paths: Vec<String> = scan_md(&text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::PathRef { component: _, path } => Some(path),
                _ => None,
            })
            .collect();
        assert_eq!(paths, vec![DOC_PATH.to_string()]);
    }

    #[test]
    fn an_interpretation_reference_is_not_found_inside_a_word_or_a_number() {
        let text = format!("R15 holds, but CR:{RULE} and FOR15 and 1.R3 do not");
        let refs: Vec<u16> = scan_md(&text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::InterpRef(n) => Some(n),
                _ => None,
            })
            .collect();
        assert_eq!(refs, vec![15]);
    }

    #[test]
    fn interpretation_references_are_markdown_only() {
        // In Rust an `R`-shaped token is usually a type parameter, and reading one as a
        // citation is worse than missing one.
        let refs = scan(&crate::source::rs::parse("fn f<R15>() {}"));
        assert!(!refs
            .iter()
            .any(|l| matches!(l.what, Observation::InterpRef(_))));
    }

    #[test]
    fn a_pin_is_read_from_the_whole_file() {
        // The date is interpolated rather than written next to the marker: spelled out, this
        // line would pin THIS file to a release, and the citation checker would report the
        // tool's own test fixture as an opt-out from the change detector.
        const DATE: &str = "20260807";
        let text = format!("intro\n<!-- cr-version: {DATE} -->\n");
        assert_eq!(
            pin(&crate::source::md::parse(&text), &text).as_deref(),
            Some(DATE)
        );
        const NONE: &str = "no pin here";
        assert_eq!(pin(&crate::source::md::parse(NONE), NONE), None);
    }

    #[test]
    fn a_pin_shown_inside_a_fence_is_an_illustration_and_binds_nothing() {
        // A document EXPLAINING the mechanism used to repin itself by showing the form, and
        // then verified its quotes against a release it never chose. The resolver fetches an
        // absent release, so the symptom was a network failure in a check that reads none.
        const DATE: &str = "20260807";
        let text = format!("intro\n\n```markdown\n<!-- cr-version: {DATE} -->\n```\n");
        assert_eq!(pin(&crate::source::md::parse(&text), &text), None);
    }
}
