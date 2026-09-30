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
//! **Rule numbers are not observed here.** An extension that reads them scans the same prose
//! itself, per `design@knowledge@an-extension-builds-its-own-model`. References are recorded
//! inside backticks, since the grammar puts them there, and a fence leaves them live.
//!
//! **The scanner tokenizes references and does not resolve them.** A backticked span holding
//! an `@` is recorded as written; whether its head is a kind or an anchor is a fact about the
//! project, which `entity::candidate` reads with the manifest in hand. What the scanner does
//! decide on its own is shape: a slug definition and where it sits, the retired slug form,
//! and the unanchored path lint, none of which needs the manifest.

use std::sync::LazyLock;

use regex::Regex;

use crate::source::Parsed;

/// Something a check might care about, found at a line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Observation {
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
    /// A reference form the `@` grammar retired.
    ///
    /// Reported, never ignored: it has no `@`, so without this a pointer the migration
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
    /// In a table cell, which defines nothing.
    Cell,
    /// At the head of a plain line: the form that predates the heading rule.
    LineHead,
    /// In the middle of a line, where a pointer belongs and a definition cannot be.
    Inline,
}

/// A reference form the grammar retired.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetiredForm {
    /// `` `<word>#<word>` ``, the slug reference before kinds existed. The span as written.
    SlugRef(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// One-based, as an editor counts.
    pub line: u32,
    pub what: Observation,
}

/// A heading of any level markdown has, so that a register may declare its entries at any
/// of them and a heading there is seen.
static HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+?)\s*$").unwrap());
/// The retired slug reference, `` `<word>#<word>` ``: the word before the `#` is optional so
/// that the older unqualified form is seen too. The first class cannot match a `#`, so a
/// definition — which opens with `##` — is not read as a retired reference to itself.
static RETIRED_SLUG_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`(\.?[A-Za-z0-9][A-Za-z0-9._-]*)?#([a-z0-9][a-z0-9-]{2,})`").unwrap()
});
/// The first slug in a heading of ANY level, or a slug at the head of a plain line. The id is
/// `[a-z0-9]+(-[a-z0-9]+)*`.
///
/// The heading form takes the slug anywhere in the heading, so the statement may precede it:
/// a heading reads as an outline entry, which a slug alone does not. Requiring text AFTER the
/// slug is what made a heading carrying nothing else invisible. Every level is matched so
/// that the entity table can report a level-one or level-five slug as misplaced rather than
/// see nothing; which levels define is its decision, not this pattern's.
static SLUG_SITE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:(#{1,6})\s+.*?(`##([a-z0-9]+(?:-[a-z0-9]+)*)`)|(`##([a-z0-9]+(?:-[a-z0-9]+)*)`))",
    )
    .unwrap()
});
/// Every slug-shaped span in a line. In a table row each is recorded as a cell; elsewhere,
/// one the pattern above did not take is a mention where a reference belongs. Both are
/// recorded so the table can report them rather than see nothing.
static SLUG_ANY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`##([a-z0-9]+(?:-[a-z0-9]+)*)`").unwrap());
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
/// A markdown link: `[text](target)`. The target may not hold a space or a closing
/// parenthesis, which is the shape every link in this tree has.
static MD_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[[^\]]*\]\(([^)\s]+)\)").unwrap());

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
            if !illustration {
                // A slug DEFINES an entity, and an entity's home is a register home under
                // an anchor. A source comment is not one, so a slug written there is
                // recorded with its site and the table reports it as misplaced — while a
                // REFERENCE from a source comment is ordinary and is scanned below.
                let trimmed = line.trim_start();
                // The one span the heading or line-head form takes, by its range in the
                // trimmed line, so every other slug-shaped span on the line is recorded as
                // what it is: a second slug on a definition line is a mention, not a second
                // definition. Dropped, it was neither defined nor misplaced.
                let primary = SLUG_SITE.captures(trimmed).map(|c| {
                    if let Some(span) = c.get(2) {
                        (span.range(), SlugSite::Heading(c[1].len() as u8))
                    } else {
                        (c.get(4).unwrap().range(), SlugSite::LineHead)
                    }
                });
                let in_row = trimmed.starts_with('|');
                for c in SLUG_ANY.captures_iter(trimmed) {
                    let span = c.get(0).unwrap();
                    let site = match &primary {
                        Some((range, site)) if *range == span.range() => *site,
                        // A table row: any cell, so a slug in any column is reported.
                        _ if in_row => SlugSite::Cell,
                        // Mid-line, a slug-shaped span is a pointer written in the
                        // definition form. Four such pointers in this tree were checked
                        // by nothing.
                        _ => SlugSite::Inline,
                    };
                    push(Observation::SlugDef {
                        id: c[1].to_string(),
                        site,
                    });
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
    out.sort_by_key(|l| l.line);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture below is written as the bytes it means. The checker reads no string
    // literal of its own source, per `design@knowledge@checker-source-literals-are-data`, so a rule
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
        for level in 1..=6u8 {
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
        // Any cell of a row, and every slug the row holds.
        assert_eq!(
            defs("| `##in-a-cell` | holds | `##last-cell` |"),
            vec![
                ("in-a-cell".to_string(), SlugSite::Cell),
                ("last-cell".to_string(), SlugSite::Cell),
            ]
        );
        // A second slug on a heading or line-head line is a mention, recorded as inline.
        assert_eq!(
            defs("### One `##first` and `##second`"),
            vec![
                ("first".to_string(), SlugSite::Heading(3)),
                ("second".to_string(), SlugSite::Inline),
            ]
        );
        // Indentation does not move the site.
        assert_eq!(
            defs("   ### Head `##indented-head`"),
            vec![("indented-head".to_string(), SlugSite::Heading(3))]
        );
        // The form that predates the heading rule, recorded so the table can report it.
        assert_eq!(
            defs("`##a-slug` — **The statement.**"),
            vec![("a-slug".to_string(), SlugSite::LineHead)]
        );
        // Mid-sentence it is a pointer written in the definition form, recorded so the
        // table can report it; a placeholder is not slug-shaped and is silent.
        assert_eq!(
            defs("write it as `##a-slug` at the end, or `##b` and `##c-d`"),
            vec![
                ("a-slug".to_string(), SlugSite::Inline),
                ("b".to_string(), SlugSite::Inline),
                ("c-d".to_string(), SlugSite::Inline),
            ]
        );
        assert_eq!(defs("write it as `##<slug>` at the end"), Vec::new());
    }

    #[test]
    fn an_id_is_lowercase_words_joined_by_single_hyphens() {
        // The grammar of the specification: one or more `[a-z0-9]` runs joined by single
        // hyphens, so a one-letter id is an id and a doubled or trailing hyphen is not.
        assert_eq!(
            defs("### One `##a`"),
            vec![("a".to_string(), SlugSite::Heading(3))]
        );
        for bad in [
            "### One `##a--b`",
            "### One `##a-`",
            "### One `##-a`",
            "### One `##A`",
        ] {
            assert_eq!(defs(bad), Vec::new(), "{bad}");
        }
    }

    #[test]
    fn a_fence_suppresses_a_definition_and_leaves_every_reference_live() {
        // A fenced heading is an illustration, so a fenced slug defines nothing and is not
        // misplaced either. A fenced REFERENCE is live for every kind, the retired slug form
        // included: a sketch names what it names on purpose, and an illustration writes a
        // placeholder in angle brackets. Mutation checked: dropping the `illustration` test
        // from the definition arm records the fenced heading as a definition.
        let fenced = "before\n```\n### A head `##a-slug`\n`design@a-component@a-slug` and \
                      `a-component#a-slug` and R15 and `docs/a.md`\n```\nafter\n";
        assert_eq!(defs(fenced), Vec::new(), "{:#?}", scan_md(fenced));
        assert_eq!(spans(fenced), vec!["design@a-component@a-slug".to_string()]);
        assert_eq!(
            retired(fenced),
            vec![RetiredForm::SlugRef("a-component#a-slug".to_string())]
        );
        assert_eq!(unanchored(fenced), vec!["docs/a.md".to_string()]);
        // The same heading outside the fence is a definition.
        assert_eq!(defs("### A head `##a-slug`\n").len(), 1);
    }

    #[test]
    fn a_source_comment_carries_references_and_its_slugs_are_recorded_for_the_table() {
        // A slug's home is a register home under an anchor, and a Rust file is never one,
        // so a slug in a comment is recorded with its site for the table to report as
        // misplaced. The reference still has to be found, because that is how a doc comment
        // cites the argument for the code under it.
        let src =
            "/// argued at `design@a-component@a-slug`\n/// ### stated `##a-slug`\nfn f() {}\n";
        let seen = scan_rs(src);
        assert!(seen.contains(&Observation::Span("design@a-component@a-slug".to_string())));
        assert!(seen.contains(&Observation::SlugDef {
            id: "a-slug".to_string(),
            site: SlugSite::Heading(3),
        }));
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
    fn a_bare_r_and_digits_is_not_a_retired_form_in_any_prose() {
        // The interpretation register's old entry number is thaum's own, and its migration is
        // finished; the core reads it as nothing, in markdown, in a code span and in a comment.
        assert_eq!(retired("R15 holds, and `R15` here"), Vec::new());
        let comment = scan_rs("/// per R15\nfn f() {}\n");
        assert!(
            !comment.iter().any(|o| matches!(o, Observation::Retired(_))),
            "{comment:?}"
        );
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
}
