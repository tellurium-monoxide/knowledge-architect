//! Pulling rule quotes out of a document, and binding each to the rule it claims to be.
//!
//! Extraction is verification logic rather than scanning, which is why it is here and not in
//! the scanner: which rule owns a quote is decided by the nearest marker before it, and how
//! far back "before" reaches differs between the two inline forms. Getting that wrong does
//! not lose a quote, it attributes it to the wrong rule — and a quote checked against the
//! wrong rule is the failure the whole regime exists to catch.

use rules::RuleNumber;

/// A quote and the rule it is offered as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quote {
    pub rule: RuleNumber,
    /// The quoted text, still carrying its elisions.
    pub body: String,
    /// One-based line the quote starts on.
    pub line: u32,
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

/// Every `>` block in a document, each joined into one line.
///
/// A block ends at a blank quoted line or at any unquoted line. `>` characters are stripped
/// from the front — all of them, so a nested quote flattens rather than keeping a stray
/// marker inside the text being matched.
pub fn blocks(text: &str) -> Vec<Block> {
    let mut out = Vec::new();
    let mut buf: Vec<String> = Vec::new();
    let mut start = 0u32;
    for (i, line) in text.lines().enumerate() {
        let n = i as u32 + 1;
        if let Some(rest) = line.strip_prefix('>') {
            let body = rest.trim_start_matches('>').trim();
            if !body.is_empty() {
                if buf.is_empty() {
                    start = n;
                }
                buf.push(body.to_string());
            } else if !buf.is_empty() {
                out.push(Block {
                    line: start,
                    lines: std::mem::take(&mut buf),
                });
            }
        } else if !buf.is_empty() {
            out.push(Block {
                line: start,
                lines: std::mem::take(&mut buf),
            });
        }
    }
    if !buf.is_empty() {
        out.push(Block {
            line: start,
            lines: buf,
        });
    }
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

/// The italic form, `*"…"*`, with neither delimiter doubled.
///
/// Hand-scanned rather than matched with a pattern: the original uses lookarounds Rust's
/// engine has none of, and the faithful replacement is not "match then filter" — a rejected
/// match must not consume its span, because the closing delimiter of a rejected attempt can
/// open the next one. Scanning explicitly is what keeps that true.
fn italic_spans(text: &str) -> Vec<Span> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] != b'*' || bytes[i + 1] != b'"' || (i > 0 && bytes[i - 1] == b'*') {
            i += 1;
            continue;
        }
        // Non-greedy: the nearest `"*` that is not followed by another `*`.
        let mut j = i + 2;
        let close = loop {
            match bytes[j..].windows(2).position(|w| w == b"\"*") {
                None => break None,
                Some(off) => {
                    let at = j + off;
                    if bytes.get(at + 2) != Some(&b'*') {
                        break Some(at);
                    }
                    j = at + 1;
                }
            }
        };
        match close {
            Some(at) if at > i + 2 => {
                out.push(Span {
                    start: i,
                    end: at + 2,
                    inner: (i + 2, at),
                    italic: true,
                });
                i = at + 2;
            }
            _ => i += 1,
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
            para.max(span.start.saturating_sub(PARAGRAPH_LOOKBACK))
        } else {
            text[..span.start].rfind('\n').map_or(0, |p| p + 1)
        };
        let from = floor_char_boundary(text, from);
        // The NEAREST marker before the quote owns it. A sentence often cites two rules in
        // sequence, each with its own marker and its own quote; taking the first or the last
        // marker on the line attributes both quotes to one of them.
        let rule = marker
            .captures_iter(&text[from..span.start])
            .last()
            .and_then(|c| RuleNumber::parse(&c[1]))
            .or_else(|| {
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
            out.push(Quote {
                rule,
                body: text[span.inner.0..span.inner.1].to_string(),
                line: line_of(text, span.start),
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
    static R: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"\bCR:(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
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

    const RULE: &str = "104.4b"; // CR~104.4b
    const OTHER: &str = "613.8c"; // CR~613.8c

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
        let text = format!("per CR:{OTHER} and then CR:{RULE}, *\"the quoted text\"*");
        assert_eq!(rules_of(&inline(&text)), vec![RULE.to_string()]);
    }

    #[test]
    fn an_italic_quote_reaches_a_marker_on_the_line_above() {
        let text = format!("per CR:{RULE},\n*\"the quoted text\"*");
        assert_eq!(rules_of(&inline(&text)), vec![RULE.to_string()]);
    }

    #[test]
    fn an_italic_quote_does_not_reach_across_a_paragraph_break() {
        let text = format!("per CR:{RULE}\n\n*\"the quoted text\"*");
        assert!(inline(&text).is_empty());
    }

    #[test]
    fn a_plain_quote_needs_a_marker_on_its_own_line() {
        let above = format!("per CR:{RULE}\n\"a plain quoted span\"");
        assert!(
            inline(&above).is_empty(),
            "a marker one line up must not own it"
        );
        let same = format!("per CR:{RULE}, \"a plain quoted span\"");
        assert_eq!(rules_of(&inline(&same)), vec![RULE.to_string()]);
    }

    #[test]
    fn a_short_plain_span_is_not_evidence() {
        let text = format!("per CR:{RULE}, \"too short\"");
        assert!(inline(&text).is_empty());
    }

    #[test]
    fn a_doubled_delimiter_is_bold_and_not_a_quote() {
        let text = format!("per CR:{RULE}, **\"emphasised, not quoted\"**");
        assert!(inline(&text).is_empty());
    }

    #[test]
    fn a_plain_span_inside_an_italic_one_is_not_counted_twice() {
        let text = format!("per CR:{RULE}, *\"the quoted text is long\"*");
        assert_eq!(inline(&text).len(), 1);
    }

    #[test]
    fn an_italic_quote_may_be_identified_by_a_trailing_bare_number() {
        // The form that predates markers, still accepted so un-retrofitted quotes stay
        // checked. A plain span may not use it.
        let text = format!("*\"the quoted text\"* ({RULE})");
        assert_eq!(rules_of(&inline(&text)), vec![RULE.to_string()]);
        let plain = format!("\"a plain quoted span\" ({RULE})");
        assert!(inline(&plain).is_empty());
    }

    #[test]
    fn a_quote_reports_the_line_it_starts_on() {
        let text = format!("one\ntwo\nper CR:{RULE}, *\"the quoted text\"*");
        assert_eq!(inline(&text)[0].line, 3);
    }

    #[test]
    fn a_lookback_window_landing_inside_a_character_does_not_panic() {
        // The window is a byte count and documents carry em dashes; a naive slice would cut
        // one in half.
        let pad = "—".repeat(200);
        let text = format!("{pad}per CR:{RULE}, *\"the quoted text\"*");
        assert_eq!(rules_of(&inline(&text)), vec![RULE.to_string()]);
    }
}
