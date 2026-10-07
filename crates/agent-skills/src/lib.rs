//! The files `knowledge-architect` installs into a project's agent configuration: skills,
//! subagent definitions and the primer.
//!
//! Each entry is the path the file is installed at, relative to the project root, and its text.
//! The list is generated from this crate's content/ directory by its build script, each file
//! rendered with the snippets of snippets/ that its placeholder lines name.

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// Every file the installer writes, as its install path and its text, in path order.
pub static FILES: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/files.rs"));

/// This crate's directory, compiled in, so a binary that links it can refuse a build made from
/// another checkout, per `design@core@a-foreign-build-is-refused`.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, for the same refusal.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod render;

#[cfg(test)]
mod tests {
    use super::FILES;

    /// The setup skill ships the maintenance crate's main that xtask's example compiles.
    #[test]
    fn the_setup_skill_ships_the_compiled_snippet() {
        let (_, setup) = FILES
            .iter()
            .find(|(path, _)| *path == ".claude/skills/knowledge-architect-setup/SKILL.md")
            .expect("the setup skill is shipped");
        // The build drops the snippet's own final line break and keeps the placeholder line's.
        let snippet = include_str!("../snippets/xtask-main.rs").trim_end_matches('\n');
        assert!(
            setup.contains(&format!("```rust\n{snippet}\n```\n")),
            "the setup skill does not hold snippets/xtask-main.rs as a fenced block"
        );
    }

    /// No shipped file holds a `%%` line: the build removes each comment line, and refuses one it
    /// would ship, per `design@agent-skills@shipped-text-line-comments`. The rules of the pass
    /// itself are held by the tests of `render`.
    #[test]
    fn no_shipped_file_holds_a_comment_line() {
        for (path, text) in FILES {
            assert!(
                !text.lines().any(|line| line.trim_start().starts_with("%%")),
                "{path} ships a `%%` line"
            );
        }
    }

    /// No shipped file holds an unfilled substitution placeholder. The text holds `{{` only as the
    /// install's `{{command}}` and as the `${{ ` that opens a GitHub Actions expression, which the
    /// setup skill writes with a space.
    #[test]
    fn no_shipped_file_holds_an_unfilled_placeholder() {
        for (path, text) in FILES {
            let rest = text.replace("{{command}}", "").replace("${{ ", "");
            assert!(!rest.contains("{{"), "{path} ships an unfilled placeholder");
        }
    }

    /// No shipped file holds a snippet placeholder: a line that is not exactly one, such as an
    /// indented one, is left as it is by the build.
    #[test]
    fn no_shipped_file_holds_a_snippet_placeholder() {
        for (path, text) in FILES {
            assert!(
                !text.contains("{{snippet:"),
                "{path} ships a snippet placeholder"
            );
        }
    }
}
