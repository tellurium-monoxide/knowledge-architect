//! One pass over a document's lines, producing typed observations.
//!
//! Every check reads observations rather than text, so the tree is walked once, each file is
//! read once, and each line is scanned once. What a check may see is exactly what this
//! module chose to record.
//!
//! **Two views of a document, and which one a kind is read from is load-bearing.** Rule
//! markers, bare rule numbers and headings are read from the text with comment leaders
//! removed, because a citation inside a code comment is a citation. Slugs, path references
//! and interpretation references are read from the raw text, because they are prose
//! conventions and a `///` in front of one means it is not at the start of a line. Reading
//! either kind from the other view silently changes what is found.

use std::sync::LazyLock;

use regex::Regex;
use rules::RuleNumber;

/// How a rule number was written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerForm {
    /// `CR:` — a claim about what the rule says. It owes a quote.
    Prose,
    /// `CR_` with underscores — the identifier form, for a test name that cannot hold
    /// punctuation. It owes a prose marker with its quote above it.
    Identifier,
    /// `CR~` — the number used as a name rather than as a claim about content. It owes no
    /// quote, and marking it keeps the distinction visible instead of silently absent.
    Mention,
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
    SlugDef(String),
    /// A slug anywhere else: a pointer at a decision.
    SlugRef(String),
    /// A repository-relative path named in prose.
    PathRef(String),
    /// An `R` number naming an interpretation entry.
    InterpRef(u16),
    /// A line inside a fenced code block, so that a check can tell an example from a claim.
    Fenced,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// One-based, as an editor counts.
    pub line: u32,
    pub what: Observation,
}

static CR_PROSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bCR:(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
static CR_MENTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bCR~(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
static CR_IDENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bCR_(\d{3})_(\d+[a-z]{0,2})").unwrap());
static RULE_TOKEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d{3}\.\d+[a-z]{0,2})\b").unwrap());
static HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,4})\s+(.+?)\s*$").unwrap());
static SLUG_REF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`#([a-z0-9][a-z0-9-]{2,})`").unwrap());
/// A slug is DEFINED where it opens a decision: at the head of a line or of a table cell,
/// followed by an em dash and the statement. Anywhere else it is a reference.
static SLUG_DEF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\| `##([a-z0-9][a-z0-9-]{2,})`\s|`##([a-z0-9][a-z0-9-]{2,})`\s+\S)").unwrap()
});
/// Only things shaped like a path = containing a slash. A bare filename in prose is a name,
/// not a pointer, and flagging those would bury the real dangling references under noise.
static PATH_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`((?:\.\.?/)?(?:[\w.-]+/)+[\w.-]+\.(?:md|txt|py|sh|tsv))`").unwrap()
});
static INTERP_REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"R(\d{1,3})\b").unwrap());
static PIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!--\s*cr-version:\s*(\d{8})\s*-->").unwrap());

/// The release a file's quotes verify against: its own pin, or `None` for the vendored one.
pub fn pin(text: &str) -> Option<String> {
    PIN.captures(text).map(|c| c[1].to_string())
}

/// Scan one document.
///
/// `raw` and `stripped` are the same file in the two views described at the top of this
/// module. They have the same number of lines, so a line number means the same thing in both.
pub fn scan(raw: &str, stripped: &str, is_markdown: bool) -> Vec<Located> {
    let mut out = Vec::new();
    let mut fenced = false;
    for (i, (raw_line, stripped_line)) in raw.lines().zip(stripped.lines()).enumerate() {
        let line = i as u32 + 1;
        let mut push = |what| out.push(Located { line, what });

        // --- the comment-stripped view -------------------------------------------------
        for c in CR_PROSE.captures_iter(stripped_line) {
            if let Some(n) = RuleNumber::parse(&c[1]) {
                push(Observation::RuleMarker {
                    number: n,
                    form: MarkerForm::Prose,
                });
            }
        }
        for c in CR_MENTION.captures_iter(stripped_line) {
            if let Some(n) = RuleNumber::parse(&c[1]) {
                push(Observation::RuleMarker {
                    number: n,
                    form: MarkerForm::Mention,
                });
            }
        }
        for c in CR_IDENT.captures_iter(stripped_line) {
            // The pattern this replaces ends in a lookahead for `_` or a word boundary, so
            // that a marker embedded in a longer test name is still recognised while one
            // whose letter suffix runs past two is not. Rust's engine has no lookahead; the
            // character after the match answers the same question, and the greedy suffix
            // cannot be shortened into a match the lookahead would have accepted.
            let after = stripped_line[c.get(0).unwrap().end()..].chars().next();
            if after.is_some_and(|c| c.is_alphanumeric() && c != '_') {
                continue;
            }
            if let Some(n) = RuleNumber::parse(&format!("{}.{}", &c[1], &c[2])) {
                push(Observation::RuleMarker {
                    number: n,
                    form: MarkerForm::Identifier,
                });
            }
        }
        for c in RULE_TOKEN.captures_iter(stripped_line) {
            if let Some(n) = RuleNumber::parse(&c[1]) {
                push(Observation::RuleToken(n));
            }
        }
        if let Some(c) = HEADING.captures(stripped_line) {
            // A `##` in a Python or shell file becomes `# …` once one leader is stripped and
            // is then read as a heading. That is the behaviour of the implementation this
            // replaces, and it is reproduced rather than corrected: changing it would change
            // the section labels in a generated index, which is a decision to take on its
            // own.
            push(Observation::Heading {
                level: c[1].len() as u8,
                text: c[2].to_string(),
            });
        }

        // --- the raw view ---------------------------------------------------------------
        if raw_line.trim_start().starts_with("```") {
            fenced = !fenced;
            push(Observation::Fenced);
            continue;
        }
        if fenced {
            push(Observation::Fenced);
        }
        if !fenced {
            if let Some(c) = SLUG_DEF.captures(raw_line.trim_start()) {
                let slug = c.get(1).or_else(|| c.get(2)).unwrap().as_str();
                push(Observation::SlugDef(slug.to_string()));
            }
        }
        for c in SLUG_REF.captures_iter(raw_line) {
            push(Observation::SlugRef(c[1].to_string()));
        }
        for c in PATH_REF.captures_iter(raw_line) {
            push(Observation::PathRef(c[1].to_string()));
        }
        if is_markdown {
            for c in INTERP_REF.captures_iter(raw_line) {
                // The pattern this replaces opens with a lookbehind rejecting a preceding
                // word character or full stop, so the `R` inside a `CR:` marker and that of a word
                // are not entry references. The character before the match answers it.
                let start = c.get(0).unwrap().start();
                let before = raw_line[..start].chars().next_back();
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                    continue;
                }
                if let Ok(n) = c[1].parse() {
                    push(Observation::InterpRef(n));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Inputs to a scanner, bound on lines carrying their CR~ mentions.
    const RULE: &str = "104.4b"; // CR~104.4b
    const OTHER: &str = "613.8c"; // CR~613.8c

    fn scan_md(text: &str) -> Vec<Observation> {
        scan(text, text, true).into_iter().map(|l| l.what).collect()
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
    fn the_three_marker_forms_are_told_apart() {
        let text = format!("per CR:{RULE}, and CR~{OTHER} is only a name");
        assert_eq!(
            markers(&text),
            vec![
                (RULE.to_string(), MarkerForm::Prose),
                (OTHER.to_string(), MarkerForm::Mention),
            ]
        );
    }

    #[test]
    fn an_identifier_marker_must_start_the_identifier_it_sits_in() {
        let ident = RULE.replace('.', "_");
        // Opening the name: recognised, and the trailing text does not end it.
        let opens = format!("fn CR_{ident}_holds() {{}}");
        assert_eq!(
            markers(&opens),
            vec![(RULE.to_string(), MarkerForm::Identifier)]
        );
        // Prefixed by anything word-shaped: NOT recognised, because the pattern opens with a
        // word boundary. Verified against the implementation being replaced, which returns
        // no match for the same input. This is a live false negative — a test named in the
        // `test_…` style carries a marker that nothing checks — and it is reproduced rather
        // than corrected, because widening it changes the convention rather than the port.
        let prefixed = format!("fn test_CR_{ident}_holds() {{}}");
        assert!(markers(&prefixed).is_empty());
    }

    #[test]
    fn an_identifier_marker_with_too_long_a_suffix_is_not_one() {
        let text = "fn test_CR_613_8cde() {}";
        assert!(markers(text).is_empty());
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

    #[test]
    fn a_slug_opens_a_decision_at_a_line_head_or_in_a_table_cell() {
        let head = format!("`##{SLUG}`  **The statement.**");
        assert!(scan_md(&head).contains(&Observation::SlugDef(SLUG.into())));
        let cell = format!("| `##{CELL_SLUG}` | holds |");
        assert!(scan_md(&cell).contains(&Observation::SlugDef(CELL_SLUG.into())));
        // Mid-sentence it is a reference, not a definition.
        let mid = scan_md(&format!("as `#{SLUG}` records"));
        assert!(mid.contains(&Observation::SlugRef(SLUG.into())));
        assert!(!mid.iter().any(|o| matches!(o, Observation::SlugDef(_))));
    }

    #[test]
    fn a_path_needs_a_slash_and_a_known_suffix() {
        let text = format!("see `{DOC_PATH}` and `citations.py` and `a/b.json`");
        let paths: Vec<String> = scan_md(&text)
            .into_iter()
            .filter_map(|o| match o {
                Observation::PathRef(p) => Some(p),
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
        let refs = scan("fn f<R15>() {}", "fn f<R15>() {}", false);
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
        assert_eq!(pin(&text).as_deref(), Some(DATE));
        assert_eq!(pin("no pin here"), None);
    }
}
