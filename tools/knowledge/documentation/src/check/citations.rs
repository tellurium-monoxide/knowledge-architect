//! Every rule quote in every live document, verified against the release it applies to.
//!
//! Two jobs of different strength, kept apart on purpose.
//!
//! **Verification** — every quoted fragment matches the applicable release, exactly. No false
//! positives; this is the guarantee.
//!
//! **The lint** — a rule-number-shaped token carrying no marker probably wants a quote.
//! Detecting a missing citation is undecidable in general, so this is a heuristic and it is
//! reported separately. Conflating the two would let the lint's false positives erode trust
//! in the guarantee.

use rules::{norm, Corpus, RuleNumber};

use crate::finding::Finding;
use crate::model::Document;
use crate::quote::{Quote, QuoteKind};
use crate::scan::{MarkerForm, Observation};

/// Below this a fragment matches too easily to be evidence of the rule CITED.
///
/// It is not a licence to skip one. The floor this replaces COUNTED a short fragment and never
/// checked it — a silent false negative inside the guarantee, and the summary said only how
/// many. A short fragment is now checked like any other; what the floor changes is that it can
/// only come back verified or unverified, never misattributed, because a common short string
/// matches somewhere in a 500 kB corpus by accident and that verdict would be noise.
///
/// `regime::MIN_FRAGMENT` is the same number, and reports the fragment as too short to be
/// evidence at all. This one governs the verdict; that one governs whether it may be written.
pub const MIN_FRAGMENT: usize = 30;

/// How far above an identifier marker its prose marker may sit, in FILE lines.
const ORPHAN_LOOKBACK: usize = 12;

/// What one release looks like to the checker: the whole text, and one body per rule.
///
/// Both halves are needed. The per-rule bodies are what a quote is checked against; the whole
/// text is what tells a misattributed quote from an invented one.
///
/// **Clonable, because parsing one is the single largest cost in a per-commit run.** `commits`
/// builds a model per commit and holds a release per commit's pin; copying an already parsed
/// release out of a cache is a memcpy where re-parsing is a walk over two megabytes.
#[derive(Clone)]
pub struct Release {
    pub whole: String,
    pub rules: Corpus,
}

impl Release {
    pub fn new(text: &str, body_starts_at: usize) -> Self {
        Self {
            whole: norm(text),
            rules: Corpus::parse(text, body_starts_at),
        }
    }
}

/// How a fragment fared against the release.
#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    /// It matches the rule cited.
    Verified,
    /// It matches the release somewhere else — a renumbering, or the wrong number. This is
    /// the case the whole check exists for: the text still exists, under a number that now
    /// means something else, so the citation still looks right.
    Misattributed,
    /// It matches nothing in the release.
    Unverified,
}

fn verdict(fragment: &str, rule: &RuleNumber, release: &Release) -> Verdict {
    let n = norm(fragment);
    if release
        .rules
        .get(rule)
        .is_some_and(|body| body.contains(&n))
    {
        Verdict::Verified
    } else if n.chars().count() >= MIN_FRAGMENT && release.whole.contains(&n) {
        Verdict::Misattributed
    } else {
        Verdict::Unverified
    }
}

/// What a run of the citation check counted, alongside what it found.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub fragments: usize,
    pub verified: usize,
    pub misattributed: usize,
    pub unverified: usize,
    /// Fragments below the floor. They ARE checked; the count says how many are weak evidence.
    pub short: usize,
    pub commentary: usize,
    pub unmarked: usize,
    pub orphans: usize,
}

/// A blockquote may quote several consecutive rules; each is judged against its own number.
///
/// **A new rule begins at a LINE that opens with a rule number, never mid-line.** The corpus
/// prints each rule on its own line and prints cross-references inside a body, so line
/// position is what tells the two apart — and the implementation this replaces joined a
/// block's lines before splitting, which threw that evidence away and bound everything after
/// a parenthetical cross-reference to the cross-referenced rule.
///
/// One guard remains, because a document wraps its blockquotes: a line that opens with a
/// number does **not** start a rule when the line above it ended mid-sentence. A wrapped
/// cross-reference reads `… (See rules` / `603.10. …`, and a rule is a complete statement,
/// nothing legitimately begins after a dangling lowercase word.
pub fn split_rules(lines: &[String]) -> Vec<(RuleNumber, String)> {
    let mut parts: Vec<(RuleNumber, String)> = Vec::new();
    let mut current: Option<RuleNumber> = None;
    let mut buf: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let opens = leading_number(line.trim());
        let after_a_statement = i == 0 || ends_a_statement(&lines[i - 1]);
        match opens {
            Some((number, rest)) if after_a_statement => {
                if let Some(cur) = current.take() {
                    parts.push((cur, buf.join(" ")));
                }
                current = Some(number);
                buf = vec![rest.to_string()];
            }
            _ => {
                if current.is_some() {
                    buf.push(line.trim().to_string());
                }
            }
        }
    }
    if let Some(cur) = current {
        parts.push((cur, buf.join(" ")));
    }
    parts
}

/// Whether a line ends something a new rule could follow.
///
/// True when it ends with a terminator and any closing brackets or quotes, and true when its
/// last word is capitalised — a heading-shaped rule such as `701.2 Activate` ends without
/// punctuation and is still complete. False only for a dangling lowercase word, which is a
/// wrap in the middle of a sentence.
fn ends_a_statement(line: &str) -> bool {
    let line = line.trim_end();
    let terminated = line
        .rfind(|c: char| !matches!(c, ')' | ']' | '"' | '\'' | '\u{201d}' | '\u{2019}'))
        .is_some_and(|i| matches!(line[i..].chars().next(), Some('.' | '!' | '?' | ':')));
    if terminated {
        return true;
    }
    let last_word: String = line
        .chars()
        .rev()
        .skip_while(|c| !c.is_alphabetic())
        .take_while(|c| c.is_alphabetic() || *c == '\'' || *c == '\u{2019}' || *c == '-')
        .collect();
    !last_word
        .chars()
        .all(|c| c.is_lowercase() || !c.is_alphabetic())
        || last_word.is_empty()
}

/// A rule number opening a line, with an optional full stop and one space after it.
///
/// **A section head needs its printed dot.** The corpus prints a section as `NNN. Title`, and
/// the blockquote convention is the number as printed — while three bare digits opening a
/// commentary blockquote are a count far more often than a citation, and reading them as one
/// would turn the commentary finding into a false verification failure.
fn leading_number(text: &str) -> Option<(RuleNumber, &str)> {
    let (index, space) = text.char_indices().find(|(_, c)| c.is_whitespace())?;
    let token = &text[..index];
    let number = match token.strip_suffix('.') {
        Some(t) => RuleNumber::parse_any(t),
        None => RuleNumber::parse(token),
    }?;
    Some((number, &text[index + space.len_utf8()..]))
}

/// A section quote's text with its own printed number stripped, where it carries one.
///
/// The blockquote form carries the heading as printed — number, dot, title — while the corpus
/// holds the title alone, so verification strips the number the binding already carries. The
/// prescribed inline form is the title entire; one written with the heading's number verifies
/// the same way rather than being reported as text of another rule.
pub fn section_body(rule: &RuleNumber, text: &str) -> String {
    if rule.is_section() {
        if let Some(rest) = text
            .trim_start()
            .strip_prefix(rule.as_str())
            .and_then(|r| r.strip_prefix('.'))
        {
            return rest.trim_start().to_string();
        }
    }
    text.to_string()
}

/// The pieces of a quote, split at its elision marks.
///
/// **Every non-empty piece is returned and every one is checked.** The version this replaces
/// held back anything under a floor and reported only a count of them, so a fabricated
/// seven-character fragment was invisible — which is the failure the floor was meant to prevent,
/// reached from the other side.
pub fn fragments(body: &str) -> (Vec<String>, usize) {
    let stripped: String = body.chars().filter(|&c| c != '*').collect();
    let mut pieces = Vec::new();
    let mut short = 0;
    for piece in stripped.split('…') {
        let n = norm(piece);
        if n.is_empty() {
            continue;
        }
        if n.chars().count() < MIN_FRAGMENT {
            short += 1;
        }
        pieces.push(piece.to_string());
    }
    (pieces, short)
}

/// Every quote in a document, inline and blockquoted, each bound to the rule it is offered as.
///
/// Shared with `regime`, which asks whether a claim has one rather than whether it verifies.
pub fn quotes(doc: &Document) -> Vec<Quote> {
    let mut out = doc.inline_quotes();
    for block in doc.blocks() {
        let lines: Vec<String> = block
            .lines
            .iter()
            .map(|l| l.chars().filter(|&c| c != '*').collect::<String>())
            .collect();
        for (rule, part) in split_rules(&lines) {
            out.push(Quote {
                rule,
                // A block binds by the number printed at its head, never by proximity, so
                // no other marker could have owned it.
                alternatives: Vec::new(),
                body: part,
                line: block.line,
                last: block.line + block.lines.len().saturating_sub(1) as u32,
                kind: QuoteKind::Block,
            });
        }
    }
    out
}

/// Verify every quote in one document, and lint its unmarked rule numbers.
pub fn check(doc: &Document, release: &Release, lint_exempt: bool) -> (Vec<Finding>, Counts) {
    let mut findings = Vec::new();
    let mut counts = Counts::default();

    // A parse that could not be trusted is `check::tree`'s finding, and the run stops there
    // before this check reads the document.

    let mut quotes = doc.inline_quotes();
    for block in doc.blocks() {
        let lines: Vec<String> = block
            .lines
            .iter()
            .map(|l| l.chars().filter(|&c| c != '*').collect::<String>())
            .collect();
        let Some(_) = leading_number(lines.first().map_or("", |l| l.trim())) else {
            // A blockquote is RESERVED for verbatim rule text. Anything else in one is
            // opinion wearing the costume of authority, whatever its length.
            counts.commentary += 1;
            findings.push(Finding::at(
                &doc.rel,
                block.line,
                format!(
                    "a blockquote holds commentary, not rule text: {}",
                    clip(lines.join(" ").trim(), 80)
                ),
                "a blockquote is reserved for verbatim rule text; write commentary as prose",
            ));
            continue;
        };
        for (rule, part) in split_rules(&lines) {
            quotes.push(Quote {
                rule,
                // A block binds by the number printed at its head, never by proximity, so
                // no other marker could have owned it.
                alternatives: Vec::new(),
                body: part,
                line: block.line,
                last: block.line + block.lines.len().saturating_sub(1) as u32,
                kind: QuoteKind::Block,
            });
        }
    }

    for q in &quotes {
        let body = section_body(&q.rule, &q.body);
        let (long, short) = fragments(&body);
        // A whole-body quote is not weak evidence, whatever its length: there is nothing more
        // of the rule to keep, and the completeness rules already exempt it from the floor.
        // Without this every section title under thirty characters — most of them — would
        // swell a count whose label says "checked, but weak".
        let whole = release
            .rules
            .get(&q.rule)
            .is_some_and(|rule| rule == norm(&body.replace(['*', '…'], "")));
        counts.short += if whole { 0 } else { short };
        for fragment in long {
            counts.fragments += 1;
            match verdict(&fragment, &q.rule, release) {
                Verdict::Verified => counts.verified += 1,
                Verdict::Misattributed => {
                    counts.misattributed += 1;
                    // The hint is the only thing a writer meets after already being wrong,
                    // and the verdict cannot tell the two causes apart — so where a second
                    // marker could have owned the quote, the hint names the binding repair
                    // FIRST. Following a retarget hint there moves a correct citation onto
                    // the wrong rule, and the result verifies, so nothing downstream
                    // reports it.
                    let hint = if q.alternatives.is_empty() {
                        "a renumbering, or the wrong number — retarget the citation at the \
                         rule that now holds this text"
                    } else {
                        "a quote binds to the NEAREST marker before it, so if the quote \
                         belongs to an earlier marker on this line, move the marker in \
                         between out of the way; only if the number itself is wrong — a \
                         renumbering — retarget the citation"
                    };
                    findings.push(Finding::at(
                        &doc.rel,
                        q.line,
                        format!(
                            "the text verifies, but not as {}: {}",
                            q.rule,
                            clip(&norm(&fragment), 80)
                        ),
                        hint,
                    ));
                }
                Verdict::Unverified => {
                    counts.unverified += 1;
                    findings.push(Finding::at(
                        &doc.rel,
                        q.line,
                        format!(
                            "quoted as {} but no rule says this: {}",
                            q.rule,
                            clip(&norm(&fragment), 100)
                        ),
                        "fix the citation and never the quote; a release that moved under it \
                         is a bump",
                    ));
                }
            }
        }
    }

    if !lint_exempt {
        let (lint, lint_counts) = lint(doc);
        findings.extend(lint);
        counts.unmarked = lint_counts.0;
        counts.orphans = lint_counts.1;
    }
    (findings, counts)
}

/// Rule-number-shaped tokens carrying no marker, and identifier markers with no prose marker
/// above them.
fn lint(doc: &Document) -> (Vec<Finding>, (usize, usize)) {
    let mut findings = Vec::new();
    let (mut unmarked, mut orphans) = (0, 0);
    let numbered: Vec<(u32, &str)> = doc
        .prose_lines()
        .into_iter()
        .filter_map(|n| doc.prose_line(n).map(|l| (n, l)))
        .collect();

    // Rule text, whichever form carries it. The exemption this replaces was LINE-shaped —
    // it skipped a line opening with `>` — while the inline form is SPAN-shaped, so a rule
    // named inside an inline quotation was reported as an unmarked reference. Measured over
    // the pinned release: 569 of 3 162 rules name another rule inside their own body, so
    // roughly one rule in six could not be quoted through its cross-reference.
    let quoted: Vec<(u32, u32, String)> = quotes(doc)
        .into_iter()
        .map(|q| (q.line, q.last, norm(&q.body)))
        .collect();

    // Lines the parser actually extracted as a blockquote. A `>` that opens a line elsewhere
    // — inside a fence, or in an indented code block — yields NO blockquote, so it is never
    // verified; and skipping it here as though it were rule text meant nothing looked at it
    // at all. The canonical citation example in this project's own root instructions is that
    // shape, and an edit reversing the rule it quotes was reported by nothing.
    let quoted_lines: std::collections::HashSet<u32> = doc
        .blocks()
        .iter()
        .flat_map(|b| b.line..b.line + b.lines.len() as u32)
        .collect();

    for (n, line) in &numbered {
        let (n, line) = (*n, *line);
        if line.trim_start().starts_with('>') {
            if quoted_lines.contains(&n) {
                continue; // verbatim rule text, and verified as such
            }
            if leading_number(line.trim_start().trim_start_matches('>').trim()).is_some() {
                unmarked += 1;
                findings.push(Finding::at(
                    &doc.rel,
                    n,
                    format!(
                        "this reads as a rule quote and sits where nothing verifies it: {}",
                        clip(line.trim(), 76)
                    ),
                    "a blockquote inside a fenced or indented block is not a blockquote; move \
                     it into prose, or drop the rule number if it is an illustration",
                ));
            }
            continue;
        }
        // The quote's whole RANGE, not the line it opens on. A quote is one span over
        // several lines, so testing the opening line exempted the first line and left every
        // continuation unprotected — and a rule long enough to carry a cross-reference is
        // long enough to wrap. The body test still binds the token to THIS quote, so a bare
        // number in ordinary prose on a continuation line is reported as it should be.
        //
        // **The body match takes the number with its own boundaries.** A plain substring
        // test let a quote's `104.1` shelter a claiming `section 104` written beside the
        // quote on its line — the section token is a prefix of the dotted number, and the
        // exemption is line-ranged, so the substring was the only thing binding token to
        // quote.
        let inside_a_quote = |token: &RuleNumber| {
            let t = token.to_string();
            quoted.iter().any(|(first, last, body)| {
                (*first..=*last).contains(&n) && contains_number(body, &t)
            })
        };
        let marked: Vec<&RuleNumber> = doc
            .observations
            .iter()
            .filter(|l| l.line == n)
            .filter_map(|l| match &l.what {
                Observation::RuleMarker { number, form } if *form != MarkerForm::Identifier => {
                    Some(number)
                }
                _ => None,
            })
            .collect();
        for (_, token) in doc
            .observations_of(|o| match o {
                Observation::RuleToken(t) => Some(t),
                _ => None,
            })
            .filter(|(at, _)| *at == n)
        {
            if !marked.contains(&token) && !inside_a_quote(token) {
                unmarked += 1;
                findings.push(Finding::at(
                    &doc.rel,
                    n,
                    format!(
                        "rule {token} is named with no marker: {}",
                        clip(line.trim(), 88)
                    ),
                    "mark it CR: with its quote; a number that is data goes in a code span \
                     or a name-bound string literal",
                ));
            }
        }
    }

    // The identifier form sits in a NAME, which is code, so it has no prose line of its own to
    // iterate. It is judged from its file line instead, against the prose above it.
    for (n, ident) in doc.observations_of(|o| match o {
        Observation::RuleMarker { number, form } if *form == MarkerForm::Identifier => Some(number),
        _ => None,
    }) {
        let above: String = numbered
            .iter()
            .filter(|(at, _)| *at < n && n - *at <= ORPHAN_LOOKBACK as u32)
            .map(|(_, l)| *l)
            .collect::<Vec<_>>()
            .join("\n");
        if !above.contains(&format!("CR:{ident}")) {
            orphans += 1;
            findings.push(Finding::at(
                &doc.rel,
                n,
                format!("the identifier marker for {ident} has no CR:{ident} above it"),
                "a name cannot carry a quote, so put a prose marker with the quote in the \
                 comment immediately above",
            ));
        }
    }
    (findings, (unmarked, orphans))
}

pub fn clip(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// Whether `body` holds `t` as a whole number rather than as a piece of a longer one.
///
/// A digit, a letter or a dot-and-digit continuing the match means the body's number is a
/// different one: `104` is not held by `104.1`, and `104.1` is not held by `104.1a`. A dot
/// followed by anything else is sentence punctuation and does not disqualify.
fn contains_number(body: &str, t: &str) -> bool {
    let mut from = 0;
    while let Some(i) = body[from..].find(t) {
        let at = from + i;
        let before_ok = body[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '.'));
        let mut after = body[at + t.len()..].chars();
        let after_ok = match after.next() {
            Some('.') => !after.next().is_some_and(|c| c.is_ascii_digit()),
            Some(c) => !c.is_ascii_alphanumeric(),
            None => true,
        };
        if before_ok && after_ok {
            return true;
        }
        from = at + t.len();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@knowledge@checker-source-literals-are-data`.

    fn n(s: &str) -> RuleNumber {
        RuleNumber::parse(s).expect("a rule number")
    }

    /// A one-document model over markdown, for a check that reads observations.
    fn doc_with(text: &str) -> crate::model::Document {
        let model = crate::model::Model::from_documents(vec![(
            std::path::PathBuf::from("notes/a.md"),
            text.to_string(),
        )]);
        model.documents()[0].clone()
    }

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(str::to_string).collect()
    }

    // The parts keep the whitespace the lines carried, as the implementation this replaces
    // did. It never shows: `norm` removes it where a fragment is compared. Read off that
    // implementation rather than tidied, because a port that trims "harmlessly" has started
    // guessing.

    #[test]
    fn a_block_quoting_one_rule_yields_one_part() {
        let parts = split_rules(&lines("100.1 The whole of it."));
        assert_eq!(parts, vec![(n("100.1"), "The whole of it.".to_string())]);
    }

    #[test]
    fn a_rule_wrapped_over_several_lines_is_still_one_part() {
        let parts = split_rules(&lines("100.1 The first half of it\nand the second half."));
        assert_eq!(
            parts,
            vec![(
                n("100.1"),
                "The first half of it and the second half.".to_string()
            )]
        );
    }

    #[test]
    fn consecutive_rules_split_at_each_line_that_opens_with_a_number() {
        let parts = split_rules(&lines("100.1 First rule text.\n100.2 Second rule text."));
        assert_eq!(
            parts,
            vec![
                (n("100.1"), "First rule text.".to_string()),
                (n("100.2"), "Second rule text.".to_string()),
            ]
        );
    }

    #[test]
    fn a_cross_reference_inside_a_body_no_longer_splits_the_block() {
        // The recorded defect. Everything after the parenthetical used to be bound to the
        // cross-referenced rule and reported against it.
        let parts = split_rules(&lines(
            "100.1 A rule that says see rule 100.2 and then continues.",
        ));
        assert_eq!(parts.len(), 1, "a mid-line number is a cross-reference");
        assert_eq!(parts[0].0, n("100.1"));
    }

    #[test]
    fn a_cross_reference_wrapped_to_the_head_of_a_line_does_not_split_either() {
        // The residual case line position alone does not answer: a document wraps its
        // blockquotes, and the wrap can land the number at the start of a line. A rule is a
        // complete statement, so nothing begins after a dangling lowercase word.
        let parts = split_rules(&lines(
            "100.1 A rule whose sentence runs on. See rules\n100.2 and the one after it.",
        ));
        assert_eq!(parts.len(), 1, "the line above ended mid-sentence");
    }

    #[test]
    fn a_heading_shaped_rule_does_not_suppress_the_rule_after_it() {
        // `701.2 Activate` ends with no punctuation and is still complete. Treating an
        // unterminated line as mid-sentence would swallow every rule that follows a heading.
        let parts = split_rules(&lines("100.1 Card Types\n100.2 The rule after it."));
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].0, n("100.2"));
    }

    /// A section, its title as the corpus prints it, and one rule under it.
    fn section_release() -> Release {
        Release::new(
            "104. A Section Title Long Enough\n\
             104.1 A rule under the section, long enough to be evidence.\n",
            0,
        )
    }

    #[test]
    fn a_section_heading_blockquote_verifies_against_the_pinned_title() {
        // The section citation form owes the heading line, so the blockquote `NNN. Title`
        // must bind by its printed head and verify against the title the corpus holds —
        // and it must not be read as commentary, which is what the dotted-only head test
        // made of it. Mutation checked: reverting `leading_number` to dotted-only turns the
        // first assertion into a commentary finding.
        let doc = doc_with("per CR:104:\n\n> 104. A Section Title Long Enough\n");
        let (findings, counts) = check(&doc, &section_release(), false);
        assert_eq!(findings, Vec::new(), "{findings:#?}");
        assert_eq!(counts.commentary, 0);
        assert_eq!((counts.fragments, counts.verified), (1, 1));

        // A title the release does not print is the failure the quote exists to catch.
        let wrong = doc_with("per CR:104:\n\n> 104. A Wrong Title Entirely Presented\n");
        let (findings, counts) = check(&wrong, &section_release(), false);
        assert_eq!(counts.unverified, 1, "{findings:#?}");
    }

    #[test]
    fn a_whole_title_quote_is_not_counted_as_weak_evidence() {
        // Most section titles are under the thirty-character floor, and a whole-body quote
        // has nothing more of the rule to keep — counting it "checked, but weak" would let
        // the migration swell that count into noise. An elided fragment stays counted.
        let doc = doc_with("per CR:104:\n\n> 104. A Section Title Long Enough\n");
        let (_, counts) = check(&doc, &section_release(), false);
        assert_eq!(counts.short, 0);
        let elided = doc_with("per CR:104.1, *\"A rule under the section…\"*\n");
        let (_, counts) = check(&elided, &section_release(), false);
        assert_eq!(counts.short, 1, "an elided short fragment is still counted");
    }

    #[test]
    fn a_keyword_section_reference_is_linted_and_a_marked_or_quoted_one_is_not() {
        // The widened lint gate: a keyword-form section reference outside the carve-outs is
        // named with no marker, exactly as a bare dotted number is. Inside a verified
        // quote's range it is the corpus's own cross-reference and owes nothing.
        let bare = doc_with("named by rule 104 in prose\n");
        let (findings, counts) = lint(&bare);
        assert_eq!(counts.0, 1, "{findings:#?}");
        let marked = doc_with("in the order CR:104 states them\n");
        assert_eq!(lint(&marked).1 .0, 0, "the marker carries the claim");
        let quoted =
            doc_with("per CR:104.1, *\"a fragment long enough to be evidence. See rule 104.\"*\n");
        assert_eq!(lint(&quoted).1 .0, 0, "a cross-reference inside a quote");
    }

    #[test]
    fn the_mismatch_hint_names_the_binding_repair_when_a_second_marker_could_own_the_quote() {
        // A quote bound to an intervening marker is a CORRECT citation wrongly bound, and
        // the retarget-only hint sent the writer at the wrong repair: retargeting produces a
        // citation that verifies, so nothing downstream reports it. Mutation checked:
        // dropping the `alternatives` branch fails the first assertion with the
        // retarget-only hint.
        let release = Release::new(
            "100.1 a first body long enough to be checked as evidence\n\
             100.2 a second body long enough to be checked as evidence\n",
            0,
        );
        // The writer means the first rule; the sentence names the second in between; the
        // quote binds to the second and misattributes.
        let doc = doc_with(
            "per CR:100.1, which CR:100.2 restates, \
             *\"a first body long enough to be checked as evidence\"*\n",
        );
        let (findings, counts) = check(&doc, &release, true);
        assert_eq!(counts.misattributed, 1, "{findings:#?}");
        let hint = &findings
            .iter()
            .find(|f| f.what.contains("verifies, but not as"))
            .expect("the mismatch finding")
            .action;
        assert!(hint.contains("NEAREST marker"), "{hint}");
        // The control: with one marker on the line, the hint stays the retarget.
        let alone =
            doc_with("per CR:100.2, *\"a first body long enough to be checked as evidence\"*\n");
        let (findings, _) = check(&alone, &release, true);
        let hint = &findings
            .iter()
            .find(|f| f.what.contains("verifies, but not as"))
            .expect("the mismatch finding")
            .action;
        assert!(hint.contains("retarget the citation at the rule"), "{hint}");
    }

    #[test]
    fn a_formatted_signature_line_opening_with_an_angle_bracket_is_not_a_blockquote() {
        // The recorded defect's exact shape: a long generic return type broken by the
        // formatter so a line reads `> {`. The grammar parser never lets a code line into
        // the prose stream, so the commentary check cannot reach it — while a doc-comment
        // quote in the same file is still extracted and verified.
        let src =
            "/// Per CR:100.1:\n///\n/// > 100.1 a fragment long enough to be evidence here.\n\
                   fn f() -> Result<\n    (usize, usize),\n    String,\n> {\n    todo!()\n}\n";
        let model = crate::model::Model::from_documents(vec![(
            std::path::PathBuf::from("code/a.rs"),
            src.to_string(),
        )]);
        let doc = model.documents()[0].clone();
        let release = Release::new("100.1 a fragment long enough to be evidence here.\n", 0);
        let (findings, counts) = check(&doc, &release, false);
        assert_eq!(findings, Vec::new(), "{findings:#?}");
        assert_eq!(counts.commentary, 0, "the code line is not a blockquote");
        assert_eq!(counts.verified, 1, "the doc-comment quote still verifies");
    }

    #[test]
    fn a_quotes_dotted_number_does_not_shelter_a_section_token_beside_it() {
        // The carve-out is line-ranged, so the body match is the only thing binding token to
        // quote — and a substring test let `104.1` inside the quote exempt a claiming
        // `section 104` written in prose on the quote's line. Mutation checked: reverting
        // `contains_number` to `contains` fails the first assertion with zero findings.
        let doc = doc_with(
            "per CR:104.1, *\"a fragment long enough to be evidence, see rule 104.1 here.\"* \
             and section 104 places no other bound\n",
        );
        let (findings, counts) = lint(&doc);
        assert_eq!(counts.0, 1, "{findings:#?}");
        // The control: the section's own cross-reference inside the quote stays sheltered.
        let genuine =
            doc_with("per CR:104.1, *\"a fragment long enough, see section 104 there.\"*\n");
        assert_eq!(lint(&genuine).1 .0, 0);
    }

    #[test]
    fn a_section_quote_carrying_its_own_heading_number_verifies_as_the_title() {
        // The blockquote form prints the number; an inline quote written the same way is the
        // same text, and reporting it as belonging to another rule sent the writer at a
        // retarget that does not exist. Mutation checked: dropping `section_body` from the
        // verdict path fails this with a misattribution finding.
        let doc = doc_with("per CR:104, *\"104. A Section Title Long Enough\"*\n");
        let (findings, counts) = check(&doc, &section_release(), false);
        assert_eq!(findings, Vec::new(), "{findings:#?}");
        assert_eq!((counts.verified, counts.misattributed), (1, 0));
    }

    #[test]
    fn a_rule_number_inside_a_quote_is_rule_text_and_not_an_unmarked_reference() {
        // 569 of 3 162 rules name another rule inside their own body, so roughly one rule in
        // six could not be quoted through its cross-reference. The exemption this pins is
        // SPAN-shaped; the line-shaped one it replaces covered only the block form.
        let doc = doc_with(
            "per CR:100.1, *\"a fragment long enough to be evidence. See rule 100.2.\"*\n",
        );
        let (found, counts) = lint(&doc);
        assert_eq!(counts.0, 0, "{found:#?}");
        // The control: the same number in ordinary prose on the same line IS reported.
        let bare = doc_with("per CR:100.1, and also 100.2 in prose\n");
        assert_eq!(lint(&bare).1 .0, 1);
    }

    #[test]
    fn a_quote_reports_the_line_range_it_actually_occupies() {
        // `Quote::last` is the field the exemption's range test reads. The lint reaches a
        // BLOCK through a different path, so a block whose range is wrong is invisible there
        // — and the field is public and its contract is stated, so it is asserted directly
        // rather than through a caller that happens not to consult it.
        let doc = doc_with(
            "per CR:100.1:\n\n> 100.1 a fragment long enough to be evidence,\n> and it continues \
             on a second line.\n",
        );
        let block = quotes(&doc)
            .into_iter()
            .find(|q| q.kind == QuoteKind::Block)
            .expect("the block is found");
        assert_eq!(
            (block.line, block.last),
            (3, 4),
            "a two-line block spans two lines"
        );

        let inline = quotes(&doc_with(
            "per CR:100.1, *\"a fragment long enough to be evidence,\nand it continues here.\"*\n",
        ))
        .into_iter()
        .find(|q| q.kind == QuoteKind::Inline)
        .expect("the inline quote is found");
        assert_eq!((inline.line, inline.last), (1, 2));
    }

    #[test]
    fn the_quote_exemption_covers_every_line_a_wrapped_quote_occupies() {
        // A quote long enough to carry a cross-reference is usually long enough to wrap, and
        // the regime asks for long quotes — so the exemption holding only on the line the
        // quote OPENS on is the common case, not the corner. A doc comment held to 100
        // columns wraps almost every whole-body quote.
        let doc = doc_with(
            "per CR:100.1, *\"a fragment long enough to be evidence,\nand it continues here. \
             See rule 100.2.\"*\n",
        );
        let (found, counts) = lint(&doc);
        assert_eq!(counts.0, 0, "{found:#?}");
        // The control, and the reason the fix is a RANGE rather than a per-line exemption: a
        // bare number in ordinary prose on a continuation line is still reported.
        let bare = doc_with(
            "per CR:100.1, *\"a fragment long enough to be evidence,\nand it continues here.\"* \
             and then 100.2 in prose\n",
        );
        assert_eq!(lint(&bare).1 .0, 1);
    }

    #[test]
    fn every_fragment_is_returned_and_the_short_ones_are_counted() {
        // The split this replaces HELD BACK anything under the floor and reported only how
        // many, so a fabricated seven-character fragment was never checked against anything.
        let (pieces, short) = fragments("a fragment comfortably longer than the floor…tiny");
        assert_eq!(pieces.len(), 2, "both pieces are returned: {pieces:?}");
        assert_eq!(short, 1, "and the short one is counted");
    }

    #[test]
    fn emphasis_inside_a_quote_is_not_part_of_the_text() {
        let (long, _) = fragments("the **emphasised** words are still the rule");
        assert_eq!(long[0], "the emphasised words are still the rule");
    }

    /// Every rule in the pinned release, blockquoted whole at several wrap widths, must yield
    /// exactly one part.
    ///
    /// This is the closing condition the recorded defect names. The implementation this
    /// replaces produces a false split for **283 of 3 162** rules quoted on one line; splitting
    /// by line position alone leaves five at this project's own width; with the mid-sentence
    /// guard it is **zero at every width from 72 to 120**.
    mod every_rule_in_the_release {
        use super::*;

        const DIGEST_RELEASE: &str = "20260807";

        fn wrap(text: &str, width: usize) -> Vec<String> {
            let mut out: Vec<String> = Vec::new();
            let mut line = String::new();
            for word in text.split_whitespace() {
                if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
                    out.push(std::mem::take(&mut line));
                }
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word);
            }
            if !line.is_empty() {
                out.push(line);
            }
            out
        }

        #[test]
        fn produces_exactly_one_part_at_every_realistic_wrap_width() {
            let root = crate::manifest::tests::this_project();
            let manifest = crate::Manifest::load(&root).expect("this project's manifest");
            let tree = manifest.rules_tree();
            let version = std::fs::read_to_string(tree.version()).expect("VERSION");
            let pinned = rules::release::read_version(&version)["date"].clone();
            if pinned != DIGEST_RELEASE {
                eprintln!("SKIPPED: measured against {DIGEST_RELEASE}, tree is at {pinned}");
                return;
            }
            let text = std::fs::read_to_string(tree.text()).expect("the rules text");
            let corpus = rules::Corpus::parse(&text, manifest.rules().body_starts_at);
            for width in [72, 80, 98, 110, 120] {
                let mut bad = Vec::new();
                for (number, body) in corpus.iter() {
                    let quoted = wrap(&format!("{number} {body}"), width);
                    if split_rules(&quoted).len() != 1 {
                        bad.push(number.to_string());
                    }
                }
                assert!(
                    bad.is_empty(),
                    "width {width}: {} of {} rules split falsely, e.g. {:?}",
                    bad.len(),
                    corpus.len(),
                    &bad[..bad.len().min(4)]
                );
            }
        }
    }
}
