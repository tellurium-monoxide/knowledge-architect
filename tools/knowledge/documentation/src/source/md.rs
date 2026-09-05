//! Markdown: the sections a document divides into, and the spans that are code rather than
//! prose.
//!
//! Run over a markdown file, and over the content of every Rust comment, so a heading or a
//! fence written in a doc comment is the same thing it is in a document. The implementation
//! this replaces recognised a fence by three backticks at the start of a **raw** line, which
//! a doc-comment fence never is, so every fenced example inside Rust prose was live text.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use super::{Frontmatter, Parsed, Prose, Scope, ScopeKind};

/// A markdown file: one prose region covering all of it, and a scope per heading.
pub fn parse(text: &str) -> Parsed {
    let (frontmatter, block_lines) = frontmatter(text);
    // The structure is analysed over the text with the block blanked out. Left in place, the
    // closing `---` is a setext underline for the `key: value` lines above it, so a
    // frontmatter block would open the document with a level-two heading nobody wrote.
    // **The blanking replaces each byte with a space rather than removing it**, so every
    // byte offset the analysis returns is an offset into the file as written; the key lines
    // stay in the prose region itself, so a reference in a value is still scanned.
    let analysed = match &block_lines {
        Some((first, last)) => blank_lines(text, *first, *last),
        None => text.to_string(),
    };
    let mut prose = Prose {
        text: analysed.clone(),
        lines: (1..=text.lines().count().max(1) as u32).collect(),
        structural: true,
        code: Vec::new(),
        line_starts: Default::default(),
    };
    let a = analyse(&analysed, &prose);
    prose.text = text.to_string();
    let (scopes, mut fenced, inert) = (a.scopes, a.fenced, a.inert);
    // The block's lines are marked as a fence, which is what keeps a metadata value from
    // being read as document structure: a heading, a slug definition or a navigation link.
    // It does NOT make a rule number data — a fence never has, per `design@knowledge@grammars-not-prefixes`
    // — and it does not touch references, which are live inside a fence for every kind. A
    // `CR:` marker in a value therefore claims its rule with nowhere in the block to put the
    // quote, and is reported exactly as one anywhere else is.
    if let Some((first, last)) = block_lines {
        fenced.extend(first..=last);
        fenced.sort_unstable();
        fenced.dedup();
    }
    let trouble = a.unterminated.map(|line| {
        format!("an HTML comment opens at line {line} and never closes, so the rest of this file is parked")
    });
    prose.code = a.code;
    Parsed {
        literals: super::Literals::Prose,
        prose: vec![prose],
        scopes,
        fenced,
        // Markdown has no names. The identifier form belongs in code, and prose that spells
        // one is making a claim the prose form fits better.
        names: Vec::new(),
        inert,
        frontmatter,
        trouble,
    }
}

/// `text` with every byte of lines `first..=last` replaced by a space.
///
/// Byte-for-byte, so every offset into the result is the same offset into the original. A
/// line of spaces is blank to the markdown parser, which is the whole of what is wanted.
fn blank_lines(text: &str, first: u32, last: u32) -> String {
    let mut out = Vec::with_capacity(text.len());
    let mut line = 1u32;
    for byte in text.bytes() {
        if byte == b'\n' {
            out.push(byte);
            line += 1;
            continue;
        }
        out.push(if line >= first && line <= last {
            b' '
        } else {
            byte
        });
    }
    String::from_utf8(out).expect("only ASCII spaces were substituted")
}

/// The frontmatter block at the top of a markdown file, and the file lines it occupies.
///
/// **A block is opened by a first line holding only `---` and closed by another**, with no
/// blank line between them. The blank line is what tells a block from a document that opens
/// on a thematic break: a `---` followed by prose and later another `---` is two thematic
/// breaks around a paragraph, and paragraphs are separated by blank lines. Inside the block
/// every line is `key: value` with a scalar value; anything else refuses the block by name
/// rather than passing as an absent optional.
fn frontmatter(text: &str) -> (Option<Frontmatter>, Option<(u32, u32)>) {
    let mut lines = text.lines();
    if lines.next().map(str::trim_end) != Some("---") {
        return (None, None);
    }
    let mut keys: Vec<(String, String)> = Vec::new();
    let mut refusal: Option<String> = None;
    for (i, line) in lines.enumerate() {
        let n = i as u32 + 2;
        let line = line.trim_end();
        if line == "---" {
            let parsed = match refusal {
                Some(why) => Err(why),
                None => Ok(keys),
            };
            return (Some(parsed), Some((1, n)));
        }
        if line.trim().is_empty() {
            // Not a block: the opening `---` was a thematic break.
            return (None, None);
        }
        if refusal.is_some() {
            continue;
        }
        match scalar(line) {
            // A key written twice is two values for one thing, and whichever reader looks
            // first decides. Refused rather than resolved, so nobody has to know which.
            Some((key, _)) if keys.iter().any(|(k, _): &(String, String)| *k == key) => {
                refusal = Some(format!("line {n} declares `{key}` a second time"))
            }
            Some(pair) => keys.push(pair),
            None => {
                refusal = Some(format!(
                    "line {n} is not `key: value` with a scalar value, which is the whole of \
                     the frontmatter subset"
                ))
            }
        }
    }
    // Opened and never closed: a thematic break with no paragraph break after it.
    (None, None)
}

/// `key: value` with a plain scalar value, which is the whole of the accepted subset.
///
/// No nesting, no lists, no quoting rules. A key is letters, digits, `-` and `_`; the value
/// is the rest of the line, trimmed, and it may not be empty.
fn scalar(line: &str) -> Option<(String, String)> {
    if line.starts_with(char::is_whitespace) {
        return None;
    }
    let (key, value) = line.split_once(':')?;
    if key.is_empty()
        || !key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return None;
    }
    let value = value.trim();
    // A flow collection is nesting written on one line, and a leading `-` opens a block
    // sequence. The subset holds none of the three.
    if value.is_empty() || value.starts_with(['-', '{', '[']) {
        return None;
    }
    Some((key.to_string(), value.to_string()))
}

/// The headings, fences and inline code spans in one prose region.
///
/// Separated from [`parse`] because a Rust file calls it once per comment run, against a
/// region whose lines are not contiguous in the file. Scopes and fences come out in FILE
/// lines; code spans come out as byte ranges in `text`, because a scanner tests them against
/// the offset of the token it just matched.
pub fn analyse(text: &str, prose: &Prose) -> Analysis {
    let mut heads: Vec<(u8, String, u32)> = Vec::new();
    let mut fenced: Vec<u32> = Vec::new();
    let mut code: Vec<(usize, usize)> = Vec::new();
    let mut inert: Vec<u32> = Vec::new();
    let mut unterminated: Option<u32> = None;
    let mut in_comment = false;

    let mut level: Option<u8> = None;
    let mut title = String::new();
    let mut start_line = 0u32;

    for (event, range) in Parser::new_ext(text, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level: l, .. }) => {
                level = Some(l as u8);
                title.clear();
                start_line = prose.file_line_at(range.start);
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(l) = level.take() {
                    heads.push((l, title.trim().to_string(), start_line));
                }
            }
            // A heading's own text arrives as several events when it holds a code span or
            // emphasis, so it is accumulated rather than read from one of them.
            Event::Text(t) if level.is_some() => title.push_str(&t),
            Event::Code(t) => {
                code.push((range.start, range.end));
                if level.is_some() {
                    title.push_str(&t);
                }
            }
            // Everything inside a fence is an illustration. Recorded by line, because that is
            // what a scanner asks.
            // An HTML COMMENT is markdown's way of parking text, and everything in one is
            // parked. Nothing else that is HTML is: an inline tag — `Vec<Player>`, a
            // `<component>` placeholder, a `<br>` — is ordinary prose, and treating its line
            // as inert erased every citation, slug and path reference on it.
            // The parser hands a comment over as one event per line, so the opener and the
            // closer arrive separately and a per-event test sees only the first.
            Event::Html(h) if in_comment || h.trim_start().starts_with("<!--") => {
                let first = prose.file_line_at(range.start);
                let last = prose.file_line_at(range.end.saturating_sub(1));
                inert.extend(first..=last);
                if h.contains("-->") {
                    in_comment = false;
                    unterminated = None;
                } else {
                    // An unterminated comment parks the rest of the file. That is what
                    // markdown says and almost never what the author meant, so it is
                    // reported rather than obeyed in silence.
                    in_comment = true;
                    unterminated.get_or_insert(first);
                }
            }
            Event::Start(Tag::CodeBlock(_)) => {
                let first = prose.file_line_at(range.start);
                let last = prose.file_line_at(range.end.saturating_sub(1));
                // Both kinds are code: an indented block has no delimiters to exclude, and a
                // fenced one's own fence lines are inside the range and are code as much as
                // its body.
                fenced.extend(first..=last);
            }
            _ => {}
        }
    }

    let last_line = prose.lines.last().copied().unwrap_or(1);
    let mut scopes = Vec::new();
    if heads.first().map(|h| h.2) != Some(1) {
        let end = heads
            .first()
            .map(|h| h.2.saturating_sub(1))
            .unwrap_or(last_line);
        if end >= 1 {
            scopes.push(Scope {
                kind: ScopeKind::Preamble,
                name: String::new(),
                first: 1,
                last: end,
            });
        }
    }
    for (i, (level, name, line)) in heads.iter().enumerate() {
        // A section runs to the next heading at the same level or shallower. Nesting is what
        // makes the innermost scope narrower than the one holding it.
        let end = heads[i + 1..]
            .iter()
            .find(|(l, _, _)| l <= level)
            .map(|(_, _, l)| l.saturating_sub(1))
            .unwrap_or(last_line);
        scopes.push(Scope {
            kind: ScopeKind::Section(*level),
            name: name.clone(),
            first: *line,
            last: end.max(*line),
        });
    }
    fenced.sort_unstable();
    fenced.dedup();
    inert.sort_unstable();
    inert.dedup();
    Analysis {
        scopes,
        fenced,
        code,
        inert,
        unterminated,
    }
}

/// What one prose region holds, in the terms every check downstream asks in.
#[derive(Debug, Default)]
pub struct Analysis {
    /// Sections, in file lines.
    pub scopes: Vec<Scope>,
    /// File lines inside a code block.
    pub fenced: Vec<u32>,
    /// Byte ranges in the region's text that are inline code spans.
    pub code: Vec<(usize, usize)>,
    /// File lines inside an HTML comment.
    pub inert: Vec<u32>,
    /// Where an HTML comment opens and never closes, parking the rest of the file.
    pub unterminated: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scopes(text: &str) -> Vec<(u8, String, u32, u32)> {
        parse(text)
            .scopes
            .into_iter()
            .filter_map(|s| match s.kind {
                ScopeKind::Section(l) => Some((l, s.name, s.first, s.last)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_section_runs_to_the_next_heading_at_its_level_or_shallower() {
        let text = "# T\n\nintro\n\n## A\n\nbody\n\n### A1\n\ndeep\n\n## B\n\nlast\n";
        let s = scopes(text);
        assert_eq!(s[0], (1, "T".into(), 1, 15));
        assert_eq!(s[1], (2, "A".into(), 5, 12));
        assert_eq!(
            s[2],
            (3, "A1".into(), 9, 12),
            "nested inside A, ends with it"
        );
        assert_eq!(s[3], (2, "B".into(), 13, 15));
    }

    #[test]
    fn the_innermost_scope_is_the_deepest_one_holding_the_line() {
        let text = "# T\n\n## A\n\n### A1\n\ndeep\n";
        let p = parse(text);
        let inner = p.scope_at(7).expect("a scope holds line 7");
        assert_eq!(inner.kind, ScopeKind::Section(3));
        assert_eq!(inner.name, "A1");
    }

    #[test]
    fn a_heading_keeps_the_text_of_its_code_spans() {
        // A decision head names a type, and the anchor pattern reads the heading, so the
        // backticked half cannot be dropped.
        let s = scopes("### `Question` has three shapes\n\nbody\n");
        assert_eq!(s[0].1, "Question has three shapes");
    }

    #[test]
    fn a_fence_is_recorded_by_line_and_includes_its_delimiters() {
        let text = "prose\n\n```rust\nlet x = 1;\n```\n\nafter\n";
        assert_eq!(parse(text).fenced, vec![3, 4, 5]);
    }

    #[test]
    fn text_before_the_first_heading_is_a_preamble_scope() {
        let p = parse("intro\n\nmore\n\n## A\n\nbody\n");
        let pre = p.scope_at(1).expect("a scope holds line 1");
        assert_eq!(pre.kind, ScopeKind::Preamble);
        assert_eq!((pre.first, pre.last), (1, 4));
    }

    #[test]
    fn a_frontmatter_block_gives_up_its_keys_and_opens_no_heading() {
        let p = parse("---\nkind: defect\nstatus: open\n---\n# The title\n\nbody\n");
        assert_eq!(
            p.frontmatter,
            Some(Ok(vec![
                ("kind".to_string(), "defect".to_string()),
                ("status".to_string(), "open".to_string()),
            ]))
        );
        // The closing `---` is a setext underline for the two key lines above it. Left in
        // place it opens a level-two section called `kind: defect status: open`, and the
        // document's own title then nests under metadata.
        let s = scopes("---\nkind: defect\nstatus: open\n---\n# The title\n\nbody\n");
        assert_eq!(s.len(), 1, "{s:#?}");
        assert_eq!(s[0], (1, "The title".into(), 5, 7));
        // A rule number in the block is data, so the block's lines are fenced; the key lines
        // stay in the prose, so a reference in a value is still scanned.
        assert_eq!(p.fenced, vec![1, 2, 3, 4]);
        assert!(p.prose[0].text.contains("kind: defect"));
    }

    #[test]
    fn a_line_outside_the_subset_refuses_the_block_by_name() {
        // A typo in a key, a nested value, both flow collections, a key outside the charset
        // and a key written twice each land here rather than passing as an absent optional.
        for (body, why) in [
            ("---\nkind:\n  nested: 1\n---\n", "line 2"),
            ("---\nkind:\n---\n", "line 2"),
            ("---\nkind: defect\ntags:\n---\n", "line 3"),
            ("---\nnot a pair\n---\n", "line 2"),
            ("---\ntags: {a: b}\n---\n", "line 2"),
            ("---\ntags: [a, b]\n---\n", "line 2"),
            ("---\nkind.sub: x\n---\n", "line 2"),
            (
                "---\nkind: a\nkind: b\n---\n",
                "declares `kind` a second time",
            ),
        ] {
            let p = parse(body);
            assert!(
                matches!(&p.frontmatter, Some(Err(e)) if e.contains(why)),
                "{body:?}: {:?}",
                p.frontmatter
            );
        }
    }

    #[test]
    fn a_delimiter_carrying_trailing_whitespace_still_opens_and_closes_the_block() {
        // A trailing space is invisible in an editor. Read strictly, the opening one makes
        // the block vanish and the closing one leaves the key lines as a setext heading,
        // which is the shape the blanking exists to prevent.
        for body in [
            "--- \nkind: defect\n---\n# T\n",
            "---\nkind: defect\n--- \n# T\n",
        ] {
            let p = parse(body);
            assert_eq!(
                p.frontmatter,
                Some(Ok(vec![("kind".to_string(), "defect".to_string())])),
                "{body:?}"
            );
            assert_eq!(scopes(body).len(), 1, "{body:?}: {:#?}", scopes(body));
        }
    }

    #[test]
    fn every_byte_offset_after_a_block_indexes_the_file_as_written() {
        // The blanking replaces each byte with a space rather than removing it, so an offset
        // the analysis returns is an offset into the original text. Removing the bytes
        // instead leaves every code span after the block shifted by the block's length, and
        // nothing else in the suite reads an offset past one. Non-ASCII inside the block is
        // the case that makes "byte for byte" more than "character for character".
        let text = "---\nkind: défaut\n---\n\nsee `path@thaum@docs/goals.md` here\n";
        let p = parse(text);
        let (a, b) = p.prose[0].code[0];
        assert_eq!(&p.prose[0].text[a..b], "`path@thaum@docs/goals.md`");
        assert!(p.prose[0].is_code(a) && !p.prose[0].is_code(a - 1));
    }

    #[test]
    fn a_leading_thematic_break_is_not_a_frontmatter_block() {
        // The blank line is what tells them apart: a paragraph break cannot sit inside a
        // block, and a document opening on a rule has one before the next `---`.
        assert_eq!(parse("---\n\nsome prose\n\n---\n").frontmatter, None);
        // Opened and never closed is not a block either.
        assert_eq!(parse("---\nkind: defect\n").frontmatter, None);
        assert_eq!(parse("# A title\n\nbody\n").frontmatter, None);
    }

    #[test]
    fn a_document_opening_on_a_heading_has_no_preamble() {
        let p = parse("# T\n\nbody\n");
        assert!(!p.scopes.iter().any(|s| s.kind == ScopeKind::Preamble));
    }
}
