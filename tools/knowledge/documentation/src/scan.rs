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
/// A slug is DEFINED where it opens a decision: at the head of a line or of a table cell,
/// followed by an em dash and the statement. Anywhere else it is a reference.
static SLUG_DEF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\| `##([a-z0-9][a-z0-9-]{2,})`\s|`##([a-z0-9][a-z0-9-]{2,})`\s+\S)").unwrap()
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
pub fn pin(text: &str) -> Option<String> {
    PIN.captures(text).map(|c| c[1].to_string())
}

/// Can a slug reference name a component called this.
///
/// Asked of a declared component name, so that a name no pointer can spell is reported where
/// it is declared rather than found later as a reference that resolves to nothing.
pub fn is_component_name(name: &str) -> bool {
    COMPONENT_NAME.is_match(name)
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
            // Both halves of the convention are guarded by the same flag. A slug inside a
            // fence is an ILLUSTRATION of the form, not a use of it, and there is no way to
            // write the illustration that is not a finding otherwise: unqualified it names no
            // component, and qualified it names a slug the example invented. Every document
            // that explains how to write a reference has to hold one.
            //
            // Paths and interpretation numbers are NOT guarded here, deliberately. Both need
            // backticks inside a fence to be found at all, which is rare, and neither has been
            // seen to fire on an example.
            for c in SLUG_REF.captures_iter(raw_line) {
                push(Observation::SlugRef {
                    component: c.get(1).map(|m| m.as_str().to_string()),
                    slug: c[2].to_string(),
                });
            }
        }
        for c in COMPONENT_PATH_REF.captures_iter(raw_line) {
            push(Observation::PathRef {
                component: Some(c[1].to_string()),
                path: c[2].to_string(),
            });
        }
        for c in PATH_REF.captures_iter(raw_line) {
            push(Observation::PathRef {
                component: None,
                path: c[1].to_string(),
            });
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
    fn a_slug_opens_a_decision_at_a_line_head_or_in_a_table_cell() {
        let head = format!("`##{SLUG}`  **The statement.**");
        assert!(scan_md(&head).contains(&Observation::SlugDef(SLUG.into())));
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
