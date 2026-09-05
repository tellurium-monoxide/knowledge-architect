//! Markdown: the sections a document divides into, and the spans that are code rather than
//! prose.
//!
//! Run over a markdown file, and over the content of every Rust comment, so a heading or a
//! fence written in a doc comment is the same thing it is in a document. The implementation
//! this replaces recognised a fence by three backticks at the start of a **raw** line, which
//! a doc-comment fence never is, so every fenced example inside Rust prose was live text.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use super::{Parsed, Prose, Scope, ScopeKind};

/// A markdown file: one prose region covering all of it, and a scope per heading.
pub fn parse(text: &str) -> Parsed {
    let mut prose = Prose {
        text: text.to_string(),
        lines: (1..=text.lines().count().max(1) as u32).collect(),
        structural: true,
        code: Vec::new(),
        line_starts: Default::default(),
    };
    let a = analyse(text, &prose);
    let (scopes, fenced, inert) = (a.scopes, a.fenced, a.inert);
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
        trouble,
    }
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
    fn a_document_opening_on_a_heading_has_no_preamble() {
        let p = parse("# T\n\nbody\n");
        assert!(!p.scopes.iter().any(|s| s.kind == ScopeKind::Preamble));
    }
}
