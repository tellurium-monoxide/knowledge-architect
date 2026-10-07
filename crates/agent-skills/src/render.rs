//! The comment pass of the shipped text, shared by the build script, which includes this file with
//! `#[path]`, and by this crate's tests, which hold its rules: the build script itself has no test
//! target. A line whose first two characters are `%%` is a comment for this repository's
//! maintainers, per `design@agent-skills@shipped-text-line-comments`.

use std::path::Path;

/// What opens a comment line of the shipped text.
pub(crate) const COMMENT: &str = "%%";

/// `text` without its comment lines. A `%%` line inside a fenced block, or one with leading
/// spaces, panics with the file and the line: it would ship rather than be removed.
///
/// A fence is read as CommonMark reads one: it opens on a line indented by at most three spaces
/// with a run of at least three backticks or three tildes, and closes on a line indented by at
/// most three spaces holding a run of the same character at least as long, followed by spaces
/// alone. A fence left open runs to the end of the text.
pub(crate) fn strip_comments(text: &str, file: &Path) -> String {
    let mut stripped = String::with_capacity(text.len());
    let mut fence: Option<(char, usize)> = None;
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let bare = line.trim_end_matches(['\n', '\r']);
        let trimmed = bare.trim_start_matches(' ');
        let indent = bare.len() - trimmed.len();
        let run = |c: char| trimmed.chars().take_while(|&x| x == c).count();
        match fence {
            None if indent <= 3 => {
                for c in ['`', '~'] {
                    let len = run(c);
                    // A backtick fence's info string holds no backtick.
                    if len >= 3 && !(c == '`' && trimmed[len..].contains('`')) {
                        fence = Some((c, len));
                    }
                }
            }
            Some((c, len)) if indent <= 3 => {
                let closing = run(c);
                if closing >= len && trimmed[closing..].trim_end_matches(' ').is_empty() {
                    fence = None;
                    stripped.push_str(line);
                    continue;
                }
            }
            _ => {}
        }
        if !trimmed.starts_with(COMMENT) {
            stripped.push_str(line);
            continue;
        }
        let at = format!("{}:{}", file.display(), n + 1);
        assert!(
            fence.is_none(),
            "{at}: a `%%` line inside a fenced block would ship"
        );
        assert!(
            indent == 0,
            "{at}: a `%%` line with leading spaces would ship"
        );
    }
    stripped
}

#[cfg(test)]
mod tests {
    use super::strip_comments;
    use std::path::Path;

    fn strip(text: &str) -> String {
        strip_comments(text, Path::new("t.md"))
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
}
