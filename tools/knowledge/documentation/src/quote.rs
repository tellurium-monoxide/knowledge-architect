//! Pulling rule quotes out of a document, and binding each to the rule it claims to be.
//!
//! Extraction is verification logic rather than scanning, which is why it is here and not in
//! the scanner: which rule owns a quote is decided by the nearest marker before it, and how
//! far back "before" reaches differs between the two inline forms. Getting that wrong does
//! not lose a quote, it attributes it to the wrong rule — and a quote checked against the
//! wrong rule is the failure the whole regime exists to catch.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use rules::RuleNumber;

/// A quote and the rule it is offered as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quote {
    pub rule: RuleNumber,
    /// The quoted text, still carrying its elisions.
    pub body: String,
    /// One-based line the quote starts on.
    pub line: u32,
    /// One-based line the quote ENDS on, which equals `line` unless the quote wraps.
    ///
    /// A quote is one span over several lines. Anything asking whether a position is inside a
    /// quote needs the range: asking only about `line` exempts the opening line and leaves
    /// every continuation line unprotected, and the regime asks for quotes long enough to wrap.
    pub last: u32,
    /// The other rules marked inside this quote's binding window, nearest first.
    ///
    /// `rule` is the nearest marker, which is how a quote is bound. The convention a writer
    /// reads says the marker goes in the clause that INTRODUCES the quote, and a clause is not
    /// a distance — so a sentence naming a second rule in between binds the quote to the rule
    /// the writer did not mean. These are the markers that could have owned it, kept so a
    /// check can ask whether the choice was ambiguous. Empty for a blockquote, which binds by
    /// the rule number printed at its head and never by proximity.
    pub alternatives: Vec<RuleNumber>,
    pub kind: QuoteKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuoteKind {
    /// `*"…"*` or a plain `"…"` on a line that carries a marker.
    Inline,
    /// A `>` block, which is reserved for verbatim rule text.
    Block,
}

/// A `>` block: its lines with the markers stripped, and the line it starts on.
///
/// The lines are kept SEPARATE. Joining them was what made a rule's own parenthetical
/// cross-reference indistinguishable from the start of the next rule, because the corpus
/// prints each rule on its own line and cross-references mid-line — so line position is
/// exactly the evidence that told them apart, and it was thrown away before the split saw it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub line: u32,
    pub lines: Vec<String>,
}

impl Block {
    /// The block as one line, for anything that does not care where the breaks were.
    pub fn text(&self) -> String {
        self.lines.join(" ")
    }
}

/// Every blockquote in a document, with its lines kept separate.
///
/// **Found by the markdown parser, not by a leading character.** The hand-written scan this
/// replaces asked `line.strip_prefix('>')`, and three shapes defeated it, each of them ordinary:
///
/// - a quote continued on the next line WITHOUT a marker, which CommonMark calls lazy
///   continuation and renders as one blockquote. The scan saw only the marked lines, so text
///   appended to a genuine quote was invisible — it verified, and the continuation could say
///   anything at all.
/// - one space before the marker, which any reflow introduces.
/// - a blockquote nested in a list, which MUST be indented to be one.
///
/// The last two removed the quote from verification entirely; the first left a verified quote
/// standing in front of unverified text.
pub fn blocks(text: &str) -> Vec<Block> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut range: Option<(usize, usize)> = None;
    for (event, at) in Parser::new_ext(text, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::BlockQuote(_)) => {
                if depth == 0 {
                    range = Some((at.start, at.end));
                }
                depth += 1;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    if let Some((from, to)) = range.take() {
                        out.extend(blocks_at(text, from, to));
                    }
                }
            }
            _ => {}
        }
    }
    out.retain(|b| !b.lines.is_empty());
    out
}

/// One blockquote's PARAGRAPHS, each its own block, with the markers taken off.
///
/// A blank quoted line separates two quotes, and the corpus prints one rule per line — so
/// merging the paragraphs of one blockquote runs consecutive rules together and the split by
/// leading number then binds the second rule's text to the first. Three quotes in this
/// repository fail that way if the paragraphs are merged.
///
/// A nested quote flattens: every leading marker comes off, so a stray one cannot end up inside
/// the text being matched against a rule.
fn blocks_at(text: &str, from: usize, to: usize) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut current: Option<Block> = None;
    for (i, raw) in text[from..to.min(text.len())].lines().enumerate() {
        let mut body = raw.trim_start();
        while let Some(rest) = body.strip_prefix('>') {
            body = rest.trim_start();
        }
        let body = body.trim();
        if body.is_empty() {
            out.extend(current.take());
            continue;
        }
        let at = line_of(text, from) + i as u32;
        match &mut current {
            Some(b) => b.lines.push(body.to_string()),
            None => {
                current = Some(Block {
                    line: at,
                    lines: vec![body.to_string()],
                })
            }
        }
    }
    out.extend(current);
    out
}

/// A span of quoted text found in a document, before a rule has been attached to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Span {
    /// Byte offset of the whole delimited span, including its delimiters.
    start: usize,
    end: usize,
    /// Byte offsets of the quoted text inside the delimiters.
    inner: (usize, usize),
    italic: bool,
}

/// The emphasised form, `*"…"*`, found as EMPHASIS rather than as two literal bytes.
///
/// The scan this replaces required the byte pair `*"`. A markdown formatter that normalises
/// emphasis to underscores rewrites every one of them to `_"`, which that scan could not see —
/// so the quotes did not become wrong, they became ABSENT, and the run still reported success
/// over a document whose citations had left the walk. Measured when it happened here: 89 of 661
/// verified fragments disappeared and three consecutive runs printed `PASSED: no findings`.
///
/// The parser represents both delimiters as one emphasis node, so the distinction is gone.
/// Doubled delimiters are `Strong` and are not emphasis, which is what keeps bold text out.
fn italic_spans(text: &str) -> Vec<Span> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut range: Option<(usize, usize)> = None;
    for (event, at) in Parser::new_ext(text, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::Emphasis) => {
                if depth == 0 {
                    range = Some((at.start, at.end));
                }
                depth += 1;
            }
            Event::End(TagEnd::Emphasis) => {
                depth = depth.saturating_sub(1);
                if depth > 0 {
                    continue;
                }
                let Some((start, end)) = range.take() else {
                    continue;
                };
                // The delimiters are one byte each, whichever character was used.
                let inner = (start + 1, end.saturating_sub(1));
                let body = &text[inner.0..inner.1];
                // The convention is a quotation INSIDE the emphasis. Emphasis alone is
                // ordinary prose and declares nothing.
                //
                // **Typographic delimiters count.** `norm` already folds `“”` to `"` because
                // they arrive in real input — a paste, an editor's smart quotes, a model
                // emitting them — and a scanner that tested only the straight form did not
                // leave such a quote WRONG, it left it absent, with the fragment total
                // silently one lower. That is the emphasis-normalisation incident again,
                // through the other delimiter.
                let open = body
                    .chars()
                    .next()
                    .filter(|c| matches!(c, '"' | '\u{201c}'));
                let close = body
                    .chars()
                    .next_back()
                    .filter(|c| matches!(c, '"' | '\u{201d}'));
                // The delimiters are NOT one byte each: a typographic quotation mark is
                // three, and slicing past it by one lands inside a character and panics.
                if let (Some(o), Some(c)) = (open, close) {
                    let (from, to) = (inner.0 + o.len_utf8(), inner.1 - c.len_utf8());
                    if from < to {
                        out.push(Span {
                            start,
                            end,
                            inner: (from, to),
                            italic: true,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// A plain `"…"` span of at least twelve characters, with neither delimiter touching a `*` or
/// a word character.
///
/// A plain span declares nothing on its own — a paragraph routinely holds a rule quote and
/// ordinary quoted phrases — so the caller only treats one as a citation when a marker sits
/// on its own line.
fn plain_spans(text: &str, italic: &[Span]) -> Vec<Span> {
    const MIN: usize = 12;
    let bytes = text.as_bytes();
    let is_edge = |b: u8| b == b'*' || b.is_ascii_alphanumeric() || b == b'_';
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'"' || (i > 0 && is_edge(bytes[i - 1])) {
            i += 1;
            continue;
        }
        // The content cannot hold a quote or a newline, so the closing delimiter is the next
        // one on this line and there is nothing to extend into.
        let Some(off) = bytes[i + 1..].iter().position(|&b| b == b'"' || b == b'\n') else {
            break;
        };
        let close = i + 1 + off;
        let ok = bytes[close] == b'"'
            && close - i > MIN
            && bytes.get(close + 1).is_none_or(|&b| !is_edge(b));
        if ok && !italic.iter().any(|s| s.start <= i && close < s.end) {
            out.push(Span {
                start: i,
                end: close + 1,
                inner: (i + 1, close),
                italic: false,
            });
            i = close + 1;
        } else {
            i += 1;
        }
    }
    out
}

/// How far back a marker may sit and still own a quote.
///
/// The two forms carry different claims, so they get different context. The italic form
/// declares *this is rule text*, so its marker may be anywhere in the paragraph — a sentence
/// wraps, and looking back only to the start of the quote's own line missed every marker that
/// landed on the line above. A plain span declares nothing, so its marker must be on its own
/// line, where the intent is unambiguous; giving those paragraph scope attributed ordinary
/// quotations to whatever rule the paragraph happened to cite.
const PARAGRAPH_LOOKBACK: usize = 400;

/// Every emphasised quotation that NO marker claims, with the line it sits on.
///
/// The emphasised form declares *this is rule text*. One with nothing to bind it is verified
/// against nothing, and the lint that would catch the bare number beside it is silenced when
/// that number sits in a code span — so the whole shape produced no output at all.
///
/// **Every unclaimed span is returned; the CALLER decides which are rule text.** Filtering here
/// on a rule number being on the same line missed the two shapes that matter most — a display
/// quote in its own paragraph, and a quotation of real rule text with no number near it — and
/// the caller has the release, which answers the question without a guess.
pub fn unclaimed(text: &str) -> Vec<(u32, String)> {
    let claimed: Vec<u32> = inline(text).iter().map(|q| q.line).collect();
    italic_spans(text)
        .into_iter()
        .map(|s| {
            (
                line_of(text, s.start),
                text[s.inner.0..s.inner.1].to_string(),
            )
        })
        .filter(|(line, _)| !claimed.contains(line))
        .collect()
}

/// Every inline quote in a document, each bound to the rule that owns it.
pub fn inline(text: &str) -> Vec<Quote> {
    let italic = italic_spans(text);
    let plain = plain_spans(text, &italic);
    let mut spans: Vec<Span> = italic.iter().copied().chain(plain).collect();
    spans.sort_by_key(|s| s.start);

    let marker = regex_marker();
    let mut out = Vec::new();
    for span in spans {
        let from = if span.italic {
            let para = text[..span.start].rfind("\n\n").map_or(0, |p| p + 2);
            // **A DISPLAY quote reaches back one paragraph further.** The shape the
            // convention prescribes is a marker, a colon, a blank line, then the quote alone
            // — and stopping at the paragraph break bound that quote to nothing at all. The
            // reach is granted only when the quote IS the paragraph, so an ordinary
            // quotation mid-paragraph cannot borrow a marker from the text above it.
            let alone = text[para..span.start].trim().is_empty();
            let from = if alone {
                text[..para.saturating_sub(2)]
                    .rfind("\n\n")
                    .map_or(0, |p| p + 2)
            } else {
                para
            };
            from.max(span.start.saturating_sub(PARAGRAPH_LOOKBACK))
        } else {
            text[..span.start].rfind('\n').map_or(0, |p| p + 1)
        };
        let from = floor_char_boundary(text, from);
        // The NEAREST marker before the quote owns it. A sentence often cites two rules in
        // sequence, each with its own marker and its own quote; taking the first or the last
        // marker on the line attributes both quotes to one of them. `parse_any`, because the
        // marker pattern is what establishes the token as a citation, and a section marker
        // binds a quote of its heading line exactly as a rule marker binds one of its body.
        let marked: Vec<RuleNumber> = marker
            .captures_iter(&text[from..span.start])
            .filter_map(|c| RuleNumber::parse_any(&c[1]))
            .collect();
        let rule = marked.last().cloned().or_else(|| {
            // The trailing bare number is what identified a citation before markers
            // existed, and it is still accepted so an un-retrofitted quote stays checked.
            // It is too weak a signal to promote an ordinary quotation, so only the
            // italic form may use it.
            if !span.italic {
                return None;
            }
            let to = floor_char_boundary(text, (span.end + 50).min(text.len()));
            regex_bare()
                .captures(&text[span.end..to])
                .and_then(|c| RuleNumber::parse(&c[1]))
        });
        if let Some(rule) = rule {
            let alternatives = marked
                .iter()
                .rev()
                .skip(1)
                .filter(|r| **r != rule)
                .cloned()
                .collect();
            out.push(Quote {
                rule,
                alternatives,
                body: text[span.inner.0..span.inner.1].to_string(),
                line: line_of(text, span.start),
                last: line_of(text, span.end),
                kind: QuoteKind::Inline,
            });
        }
    }
    out
}

fn line_of(text: &str, offset: usize) -> u32 {
    text[..offset].bytes().filter(|&b| b == b'\n').count() as u32 + 1
}

/// The largest index at or below `i` that starts a character.
///
/// The lookback windows are byte counts, and a document holds em dashes and ellipses, so a
/// window can land inside a character.
fn floor_char_boundary(text: &str, mut i: usize) -> usize {
    while i > 0 && !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn regex_marker() -> &'static regex::Regex {
    // The subrule part is optional so a section marker binds too. Greed keeps a dotted marker
    // whole: the alternative never wins when subrule digits follow.
    static R: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"\bCR:(\d{3}(?:\.\d+[a-z]{0,2})?)\b").unwrap()
    });
    &R
}

fn regex_bare() -> &'static regex::Regex {
    static R: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"\b(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
    &R
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture below is written inline: the checker reads no string literal of its own
    // source, per `design@knowledge@checker-source-literals-are-data`.

    fn rules_of(quotes: &[Quote]) -> Vec<String> {
        quotes.iter().map(|q| q.rule.to_string()).collect()
    }

    #[test]
    fn a_block_joins_its_lines_and_ends_at_a_blank_or_unquoted_line() {
        let b = blocks("> one\n> two\n\nprose\n> three\n");
        assert_eq!(b.len(), 2);
        assert_eq!(b[0].text(), "one two");
        assert_eq!(b[0].lines.len(), 2, "the lines stay separate");
        assert_eq!(b[0].line, 1);
        assert_eq!(b[1].text(), "three");
        assert_eq!(b[1].line, 5);
    }

    #[test]
    fn a_nested_marker_is_flattened_out_of_the_text() {
        assert_eq!(blocks(">> nested\n")[0].text(), "nested");
    }

    #[test]
    fn an_italic_quote_takes_the_nearest_marker_before_it() {
        let text = "per CR:613.8c and then CR:104.4b, *\"the quoted text\"*";
        assert_eq!(rules_of(&inline(text)), vec!["104.4b".to_string()]);
    }

    #[test]
    fn a_section_marker_binds_a_quote_the_same_way_a_rule_marker_does() {
        // A section citation owes its heading line, so the binder must let the dotless
        // marker own a quote — and nearest-wins must hold across the two forms, or a
        // sentence citing a section and then a rule binds the rule's quote to the section.
        let text = "per CR:104, *\"the quoted heading line\"*";
        assert_eq!(rules_of(&inline(text)), vec!["104".to_string()]);
        let both = "per CR:104 and then CR:104.4b, *\"the quoted text\"*";
        assert_eq!(rules_of(&inline(both)), vec!["104.4b".to_string()]);
        let alternatives: Vec<String> = inline(both)[0]
            .alternatives
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(alternatives, vec!["104".to_string()]);
    }

    #[test]
    fn an_italic_quote_reaches_a_marker_on_the_line_above() {
        let text = "per CR:104.4b,\n*\"the quoted text\"*";
        assert_eq!(rules_of(&inline(text)), vec!["104.4b".to_string()]);
    }

    #[test]
    fn an_italic_quote_reaches_across_a_paragraph_break_only_when_it_is_the_paragraph() {
        // A DISPLAY quote — marker, colon, blank line, quote alone — is the shape the
        // convention prescribes, and stopping at the break bound it to nothing.
        let display = "per CR:104.4b:\n\n*\"the quoted text\"*";
        assert_eq!(rules_of(&inline(display)), vec!["104.4b".to_string()]);
        // Ordinary prose in the paragraph, and the quote may not borrow the marker above.
        let midway = "per CR:104.4b\n\nprose first, *\"the quoted text\"*";
        assert!(inline(midway).is_empty());
    }

    #[test]
    fn a_plain_quote_needs_a_marker_on_its_own_line() {
        let above = "per CR:104.4b\n\"a plain quoted span\"";
        assert!(
            inline(above).is_empty(),
            "a marker one line up must not own it"
        );
        let same = "per CR:104.4b, \"a plain quoted span\"";
        assert_eq!(rules_of(&inline(same)), vec!["104.4b".to_string()]);
    }

    #[test]
    fn a_short_plain_span_is_not_evidence() {
        let text = "per CR:104.4b, \"too short\"";
        assert!(inline(text).is_empty());
    }

    #[test]
    fn a_doubled_delimiter_is_bold_and_not_a_quote() {
        let text = "per CR:104.4b, **\"emphasised, not quoted\"**";
        assert!(inline(text).is_empty());
    }

    #[test]
    fn a_plain_span_inside_an_italic_one_is_not_counted_twice() {
        let text = "per CR:104.4b, *\"the quoted text is long\"*";
        assert_eq!(inline(text).len(), 1);
    }

    #[test]
    fn an_italic_quote_may_be_identified_by_a_trailing_bare_number() {
        // The form that predates markers, still accepted so un-retrofitted quotes stay
        // checked. A plain span may not use it.
        let text = "*\"the quoted text\"* (104.4b)";
        assert_eq!(rules_of(&inline(text)), vec!["104.4b".to_string()]);
        let plain = "\"a plain quoted span\" (104.4b)";
        assert!(inline(plain).is_empty());
    }

    #[test]
    fn an_underscore_delimiter_is_the_same_quote_as_an_asterisk_one() {
        // A markdown formatter that normalises emphasis rewrites `*"…"*` to `_"…"_`. The
        // byte-pair scanner this replaces saw only the asterisk form, so the rewrite did not
        // make quotes wrong, it made them ABSENT: 89 of 661 verified fragments left the walk
        // and three consecutive runs printed a pass. The parser represents both delimiters as
        // one emphasis node; this pins that the distinction stays gone.
        let underscored = "per CR:104.4b, _\"the quoted text\"_";
        assert_eq!(rules_of(&inline(underscored)), vec!["104.4b".to_string()]);
        assert_eq!(inline(underscored)[0].body, "the quoted text");
    }

    #[test]
    fn a_typographic_delimiter_is_the_same_quote_as_a_straight_one() {
        // `norm` already folds these because they arrive in real input. A scanner that tested
        // only the straight form did not leave such a quote wrong, it left it ABSENT — the
        // emphasis-normalisation incident again, through the other delimiter.
        let curly = "per CR:104.4b, *\u{201c}the quoted text\u{201d}*";
        assert_eq!(rules_of(&inline(curly)), vec!["104.4b".to_string()]);
        assert_eq!(inline(curly)[0].body, "the quoted text");
    }

    #[test]
    fn a_display_quote_alone_in_its_paragraph_reaches_the_marker_above_it() {
        // The shape the convention prescribes: marker, colon, blank line, quote. Stopping at
        // the paragraph break bound it to nothing.
        let display = "the engine follows CR:104.4b:\n\n*\"the quoted text\"*\n";
        assert_eq!(rules_of(&inline(display)), vec!["104.4b".to_string()]);
        // The reach is granted only when the quote IS the paragraph, so an ordinary
        // quotation mid-paragraph cannot borrow a marker from the text above it.
        let midway = "per CR:104.4b\n\nprose first, *\"the quoted text\"*\n";
        assert!(inline(midway).is_empty());
    }

    #[test]
    fn every_unclaimed_span_is_returned_for_the_caller_to_judge() {
        let orphan = "it says *\"some quoted text here\"* and nothing marks it";
        assert_eq!(unclaimed(orphan).len(), 1);
        let claimed = "per CR:104.4b, *\"some quoted text here\"*";
        assert!(unclaimed(claimed).is_empty());
    }

    #[test]
    fn a_quote_reports_the_line_it_starts_on() {
        let text = "one\ntwo\nper CR:104.4b, *\"the quoted text\"*";
        assert_eq!(inline(text)[0].line, 3);
    }

    #[test]
    fn a_lookback_window_landing_inside_a_character_does_not_panic() {
        // The window is a byte count and documents carry em dashes; a naive slice would cut
        // one in half.
        let pad = "—".repeat(200);
        let text = format!("{pad}per CR:104.4b, *\"the quoted text\"*");
        assert_eq!(rules_of(&inline(&text)), vec!["104.4b".to_string()]);
    }
}
