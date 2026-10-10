//! The passes of the shipped text that the build script and this crate's tests share: the build
//! script includes this file with `#[path]`, and it has no test target of its own.
//!
//! - **The comment pass.** A line whose first two characters are `%%` is a comment for this
//!   repository's maintainers, per `design@agent-skills@shipped-text-line-comments`.
//! - **The section slug pass.** A level-two heading ends with a placeholder `{{slug:<id>}}`, which
//!   ships as the slug the checker reads, so content/ defines no slug in this repository's walk
//!   and the installed copy does.
//! - **The workflow name check.** A saved workflow is called by the name its `meta` declares, not
//!   by its file's name, so the name must carry the installer's prefix as the file does.

use std::path::Path;

/// What opens a comment line of the shipped text.
pub(crate) const COMMENT: &str = "%%";

/// What opens a section slug placeholder; `}}` closes it.
pub(crate) const SLUG: &str = "{{slug:";

/// Whether a line is inside a fenced block, read as CommonMark reads one.
///
/// A fence opens on a line indented by at most three spaces with a run of at least three
/// backticks or three tildes, and closes on a line indented by at most three spaces holding a run
/// of the same character at least as long, followed by spaces alone. A fence left open runs to
/// the end of the text.
#[derive(Default)]
struct Fence(Option<(char, usize)>);

impl Fence {
    /// Step over one line, its line break removed: whether the line itself is part of a fenced
    /// block, its opening and closing lines included.
    fn fenced(&mut self, bare: &str) -> bool {
        let trimmed = bare.trim_start_matches(' ');
        let indent = bare.len() - trimmed.len();
        let run = |c: char| trimmed.chars().take_while(|&x| x == c).count();
        match self.0 {
            None if indent <= 3 => {
                for c in ['`', '~'] {
                    let len = run(c);
                    // A backtick fence's info string holds no backtick.
                    if len >= 3 && !(c == '`' && trimmed[len..].contains('`')) {
                        self.0 = Some((c, len));
                        return true;
                    }
                }
                false
            }
            None => false,
            Some((c, len)) => {
                let closing = run(c);
                if indent <= 3
                    && closing >= len
                    && trimmed[closing..].trim_end_matches(' ').is_empty()
                {
                    self.0 = None;
                }
                true
            }
        }
    }
}

/// `text` without its comment lines. A `%%` line inside a fenced block, or one with leading
/// spaces, panics with the file and the line: it would ship rather than be removed.
pub(crate) fn strip_comments(text: &str, file: &Path) -> String {
    let mut stripped = String::with_capacity(text.len());
    let mut fence = Fence::default();
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let bare = line.trim_end_matches(['\n', '\r']);
        let fenced = fence.fenced(bare);
        let trimmed = bare.trim_start_matches(' ');
        if !trimmed.starts_with(COMMENT) {
            stripped.push_str(line);
            continue;
        }
        let at = format!("{}:{}", file.display(), n + 1);
        assert!(
            !fenced,
            "{at}: a `%%` line inside a fenced block would ship"
        );
        assert!(
            bare.len() == trimmed.len(),
            "{at}: a `%%` line with leading spaces would ship"
        );
    }
    stripped
}

/// `text` with each section slug placeholder rendered into the slug the checker reads.
///
/// A placeholder is legal at one place only: at the end of a level-two heading outside a fenced
/// block, after a space, holding an id in the grammar `[a-z0-9]+(-[a-z0-9]+)*`. It ships as a
/// backticked `##<id>`, which defines a section in the installed copy, per the section rule of
/// the checker's entity table. Anywhere else it panics with the file and the line: a placeholder
/// that shipped unrendered would leave the section without its slug, and one in a fence would
/// render an illustration into a definition.
pub(crate) fn render_slugs(text: &str, file: &Path) -> String {
    let mut rendered = String::with_capacity(text.len());
    let mut fence = Fence::default();
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let bare = line.trim_end_matches(['\n', '\r']);
        let fenced = fence.fenced(bare);
        let Some(at) = bare.find(SLUG) else {
            rendered.push_str(line);
            continue;
        };
        let site = format!("{}:{}", file.display(), n + 1);
        assert!(
            !fenced,
            "{site}: a section slug placeholder inside a fenced block would ship"
        );
        assert!(
            bare.starts_with("## ") && !bare.starts_with("###"),
            "{site}: a section slug placeholder ends a level-two heading, and this line is none"
        );
        let id = bare[at + SLUG.len()..]
            .strip_suffix("}}")
            .filter(|id| !id.contains('}'))
            .unwrap_or_else(|| {
                panic!("{site}: a section slug placeholder ends its heading, closed by `}}}}`")
            });
        assert!(
            is_id(id),
            "{site}: `{id}` is no slug: lower-case words and digits joined by hyphens"
        );
        assert!(
            bare[..at].ends_with(' ')
                && !bare[..at].trim_end().is_empty()
                && bare[..at].trim_end() != "##",
            "{site}: a section slug placeholder follows the heading's text and a space"
        );
        rendered.push_str(&bare[..at]);
        rendered.push_str("`##");
        rendered.push_str(id);
        rendered.push('`');
        rendered.push_str(&line[bare.len()..]);
    }
    rendered
}

/// What opens a saved workflow's declaration; its `name` key follows inside the object.
const WORKFLOW_META: &str = "export const meta = {";

/// Panics unless the saved workflow `text` declares `name` as `expected`, its installed file's
/// stem. Claude Code calls a saved workflow by its declared `name` and never by its file name, so
/// a prefixed file whose declared name lacks the prefix would sit in the installer's namespace
/// while it answers to a name a project's own workflow can take. The name is read as the first
/// line of the declaration that opens with `name:`, holding one quoted string: the declaration is
/// a literal the harness parses, so it holds no computed value.
pub(crate) fn check_workflow_name(text: &str, expected: &str, file: &Path) {
    let site = file.display();
    let start = text
        .find(WORKFLOW_META)
        .unwrap_or_else(|| panic!("{site}: a saved workflow opens with `{WORKFLOW_META}`"));
    let declaration = &text[start + WORKFLOW_META.len()..];
    let declaration = &declaration[..declaration
        .find(
            "
}",
        )
        .unwrap_or(declaration.len())];
    let name = declaration
        .lines()
        .find_map(|line| line.trim_start().strip_prefix("name:"))
        .map(|rest| rest.trim().trim_end_matches(','))
        .and_then(|quoted| {
            ['\'', '"']
                .iter()
                .find_map(|q| quoted.strip_prefix(*q)?.strip_suffix(*q))
        })
        .unwrap_or_else(|| panic!("{site}: the workflow's meta declares no quoted `name`"));
    assert!(
        name == expected,
        "{site}: the workflow declares the name `{name}`, and is installed as `{expected}`: \
         the harness calls it by the declared name, so the two are equal"
    );
}

/// The id grammar of the checker's entity table, `[a-z0-9]+(-[a-z0-9]+)*`, written out: this
/// crate depends on no regex engine, and the checker's own pattern is in the core, which depends
/// on this crate.
fn is_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|w| {
            !w.is_empty()
                && w.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use super::{check_workflow_name, render_slugs, strip_comments};
    use std::path::Path;

    fn strip(text: &str) -> String {
        strip_comments(text, Path::new("t.md"))
    }

    fn slugs(text: &str) -> String {
        render_slugs(text, Path::new("t.md"))
    }

    #[test]
    fn a_placeholder_ending_a_level_two_heading_ships_as_its_slug() {
        // Each expected text is bound to a name: an unbound literal of this crate is prose to the
        // checker's walk, and the slug it holds would read as a definition.
        let rendered = "# T\n\n## The axes `##review-axes`\nbody\n";
        assert_eq!(
            slugs("# T\n\n## The axes {{slug:review-axes}}\nbody\n"),
            rendered
        );
        let crlf = "## A `##a1`\r\nx";
        assert_eq!(slugs("## A {{slug:a1}}\r\nx"), crlf);
    }

    #[test]
    fn a_text_with_no_placeholder_is_unchanged() {
        let text = "## Plain\n```\n## {{x}}\n```\n";
        assert_eq!(slugs(text), text);
    }

    #[test]
    #[should_panic(
        expected = "t.md:2: a section slug placeholder inside a fenced block would ship"
    )]
    fn a_placeholder_inside_a_fence_fails() {
        slugs("```\n## A {{slug:a}}\n```\n");
    }

    #[test]
    #[should_panic(expected = "t.md:1: a section slug placeholder ends a level-two heading")]
    fn a_placeholder_on_a_level_three_heading_fails() {
        slugs("### A {{slug:a}}\n");
    }

    #[test]
    #[should_panic(expected = "ends a level-two heading")]
    fn a_placeholder_in_prose_fails() {
        slugs("see {{slug:a}} here\n");
    }

    #[test]
    #[should_panic(expected = "closed by")]
    fn a_placeholder_before_more_text_fails() {
        slugs("## A {{slug:a}} more\n");
    }

    #[test]
    #[should_panic(expected = "is no slug")]
    fn a_placeholder_outside_the_id_grammar_fails() {
        slugs("## A {{slug:Not_An_Id}}\n");
    }

    #[test]
    #[should_panic(expected = "follows the heading's text and a space")]
    fn a_placeholder_with_no_heading_text_fails() {
        slugs("## {{slug:a}}\n");
    }

    #[test]
    fn a_comment_line_is_removed_and_nothing_else() {
        assert_eq!(strip("a\n%% why\nb\n"), "a\nb\n");
        assert_eq!(strip("a\r\n%% why\r\nb"), "a\r\nb");
        assert_eq!(strip("a\n%% last, no line break"), "a\n");
    }

    #[test]
    fn a_comment_after_a_closed_fence_is_removed() {
        assert_eq!(strip("```sh\nls\n```\n%% why\nb\n"), "```sh\nls\n```\nb\n");
        assert_eq!(strip("~~~~\nx\n~~~~~\n%% why\n"), "~~~~\nx\n~~~~~\n");
    }

    #[test]
    #[should_panic(expected = "t.md:2: a `%%` line inside a fenced block would ship")]
    fn a_comment_inside_a_fence_fails() {
        strip("```\n%% x\n```\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_shorter_run_does_not_close_a_longer_fence() {
        strip("````markdown\n```sh\n%% x\n```\n````\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_bare_shorter_run_does_not_close_a_longer_fence() {
        strip("````\n```\n%% x\n````\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_tilde_run_does_not_close_a_backtick_fence() {
        strip("```\n~~~\n%% x\n```\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_fence_left_open_runs_to_the_end() {
        strip("```\nx\n%% y\n");
    }

    #[test]
    fn a_run_with_an_info_string_does_not_close_a_fence() {
        assert_eq!(
            strip("```markdown\n```rust\n```\n\n%% outside\nb\n"),
            "```markdown\n```rust\n```\n\nb\n"
        );
    }

    #[test]
    #[should_panic(expected = "t.md:1: a `%%` line with leading spaces would ship")]
    fn an_indented_comment_fails() {
        strip("  %% x\n");
    }

    #[test]
    #[should_panic(expected = "follows the heading's text and a space")]
    fn a_placeholder_glued_to_the_heading_text_fails() {
        slugs("## Text{{slug:a}}\n");
    }

    fn named(text: &str) {
        check_workflow_name(text, "knowledge-architect-w", Path::new("w.js"));
    }

    #[test]
    fn a_workflow_declaring_its_installed_name_passes() {
        named("export const meta = {\n  name: 'knowledge-architect-w',\n  description: 'd',\n}\n");
        named("export const meta = {\n  name: \"knowledge-architect-w\"\n}\nreturn 1\n");
    }

    #[test]
    #[should_panic(expected = "w.js: the workflow declares the name `w`")]
    fn a_workflow_declaring_another_name_fails() {
        named("export const meta = {\n  name: 'w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "declares no quoted `name`")]
    fn a_name_outside_the_declaration_is_not_read() {
        named("export const meta = {\n  description: 'd',\n}\nconst x = {\n  name: 'knowledge-architect-w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "w.js: a saved workflow opens with")]
    fn a_workflow_with_no_declaration_fails() {
        named("return 1\n");
    }
}
