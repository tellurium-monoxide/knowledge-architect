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
fn leading_number(text: &str) -> Option<(RuleNumber, &str)> {
    let (index, space) = text.char_indices().find(|(_, c)| c.is_whitespace())?;
    let token = &text[..index];
    let number = RuleNumber::parse(token.strip_suffix('.').unwrap_or(token))?;
    Some((number, &text[index + space.len_utf8()..]))
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

    // A parse that could not be trusted must be LOUD. Silently it removes every citation in
    // the file from the walk while the run reports success — indistinguishable from a clean
    // file, and the guard the premortem named for a grammar that changes under an upgrade.
    if let Some(trouble) = &doc.parsed.trouble {
        findings.push(Finding::at(
            &doc.rel,
            1,
            trouble.clone(),
            "nothing here is verified; fix the source, or the citations in it are unchecked",
        ));
    }

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
        let (long, short) = fragments(&q.body);
        counts.short += short;
        for fragment in long {
            counts.fragments += 1;
            match verdict(&fragment, &q.rule, release) {
                Verdict::Verified => counts.verified += 1,
                Verdict::Misattributed => {
                    counts.misattributed += 1;
                    findings.push(Finding::at(
                        &doc.rel,
                        q.line,
                        format!(
                            "the text verifies, but not as {}: {}",
                            q.rule,
                            clip(&norm(&fragment), 80)
                        ),
                        "a renumbering, or the wrong number — retarget the citation at the \
                         rule that now holds this text",
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
        let inside_a_quote = |token: &RuleNumber| {
            let t = token.to_string();
            quoted
                .iter()
                .any(|(first, last, body)| (*first..=*last).contains(&n) && body.contains(&t))
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
                    "mark it CR: with its quote; a number that is data goes in a code span, \
                     a fenced block or a name-bound string literal",
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

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = "100.1";
    const B: &str = "100.2";

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
        let parts = split_rules(&lines(&format!("{A} The whole of it.")));
        assert_eq!(parts, vec![(n(A), "The whole of it.".to_string())]);
    }

    #[test]
    fn a_rule_wrapped_over_several_lines_is_still_one_part() {
        let parts = split_rules(&lines(&format!(
            "{A} The first half of it\nand the second half."
        )));
        assert_eq!(
            parts,
            vec![(
                n(A),
                "The first half of it and the second half.".to_string()
            )]
        );
    }

    #[test]
    fn consecutive_rules_split_at_each_line_that_opens_with_a_number() {
        let parts = split_rules(&lines(&format!(
            "{A} First rule text.\n{B} Second rule text."
        )));
        assert_eq!(
            parts,
            vec![
                (n(A), "First rule text.".to_string()),
                (n(B), "Second rule text.".to_string()),
            ]
        );
    }

    #[test]
    fn a_cross_reference_inside_a_body_no_longer_splits_the_block() {
        // The recorded defect. Everything after the parenthetical used to be bound to the
        // cross-referenced rule and reported against it.
        let parts = split_rules(&lines(&format!(
            "{A} A rule that says see rule {B} and then continues."
        )));
        assert_eq!(parts.len(), 1, "a mid-line number is a cross-reference");
        assert_eq!(parts[0].0, n(A));
    }

    #[test]
    fn a_cross_reference_wrapped_to_the_head_of_a_line_does_not_split_either() {
        // The residual case line position alone does not answer: a document wraps its
        // blockquotes, and the wrap can land the number at the start of a line. A rule is a
        // complete statement, so nothing begins after a dangling lowercase word.
        let parts = split_rules(&lines(&format!(
            "{A} A rule whose sentence runs on. See rules\n{B} and the one after it."
        )));
        assert_eq!(parts.len(), 1, "the line above ended mid-sentence");
    }

    #[test]
    fn a_heading_shaped_rule_does_not_suppress_the_rule_after_it() {
        // `701.2 Activate` ends with no punctuation and is still complete. Treating an
        // unterminated line as mid-sentence would swallow every rule that follows a heading.
        let parts = split_rules(&lines(&format!("{A} Card Types\n{B} The rule after it.")));
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].0, n(B));
    }

    #[test]
    fn a_rule_number_inside_a_quote_is_rule_text_and_not_an_unmarked_reference() {
        // 569 of 3 162 rules name another rule inside their own body, so roughly one rule in
        // six could not be quoted through its cross-reference. The exemption this pins is
        // SPAN-shaped; the line-shaped one it replaces covered only the block form.
        let doc = doc_with(&format!(
            "per CR:{A}, *\"a fragment long enough to be evidence. See rule {B}.\"*\n"
        ));
        let (found, counts) = lint(&doc);
        assert_eq!(counts.0, 0, "{found:#?}");
        // The control: the same number in ordinary prose on the same line IS reported.
        let bare = doc_with(&format!("per CR:{A}, and also {B} in prose\n"));
        assert_eq!(lint(&bare).1 .0, 1);
    }

    #[test]
    fn a_quote_reports_the_line_range_it_actually_occupies() {
        // `Quote::last` is the field the exemption's range test reads. The lint reaches a
        // BLOCK through a different path, so a block whose range is wrong is invisible there
        // — and the field is public and its contract is stated, so it is asserted directly
        // rather than through a caller that happens not to consult it.
        let doc = doc_with(&format!(
            "per CR:{A}:\n\n> {A} a fragment long enough to be evidence,\n> and it continues \
             on a second line.\n"
        ));
        let block = quotes(&doc)
            .into_iter()
            .find(|q| q.kind == QuoteKind::Block)
            .expect("the block is found");
        assert_eq!(
            (block.line, block.last),
            (3, 4),
            "a two-line block spans two lines"
        );

        let inline = quotes(&doc_with(&format!(
            "per CR:{A}, *\"a fragment long enough to be evidence,\nand it continues here.\"*\n"
        )))
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
        let doc = doc_with(&format!(
            "per CR:{A}, *\"a fragment long enough to be evidence,\nand it continues here. \
             See rule {B}.\"*\n"
        ));
        let (found, counts) = lint(&doc);
        assert_eq!(counts.0, 0, "{found:#?}");
        // The control, and the reason the fix is a RANGE rather than a per-line exemption: a
        // bare number in ordinary prose on a continuation line is still reported.
        let bare = doc_with(&format!(
            "per CR:{A}, *\"a fragment long enough to be evidence,\nand it continues here.\"* \
             and then {B} in prose\n"
        ));
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
