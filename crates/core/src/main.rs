//! The core's own binary: the core's commands, with no extension registered, per
//! `design@core@the-core-cli-is-a-library-module`.
//!
//! A project whose manifest declares only what the core reads runs this. A project with a
//! subject of its own builds a binary that registers the extension for it.

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "klarch",
    version,
    about = "What a project's documents hold, and whether it still holds.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: knowledge_architect::cli::Command,
}

fn main() -> ExitCode {
    // Parsed before the project is located, so `--help` answers from anywhere. clap exits 2 on
    // a parse failure, which is already the could-not-run code.
    let cli = Cli::parse();
    // The one path compiled in: the core's own Component, whose fixtures are read as data,
    // per `design@core@checker-source-literals-are-data`.
    let dir = knowledge_architect::component_dir();
    let dirs: Vec<&Path> = vec![dir.as_path()];
    let outcome = knowledge_architect::cli::locate().and_then(|manifest| {
        // A binary built from another checkout would judge this tree with that checkout's
        // code, so no command runs, per `design@core@a-foreign-build-is-refused`.
        knowledge_architect::cli::refuse_a_foreign_build(
            manifest.root(),
            // The library and the binary are one crate, so its one entry covers both. The crate
            // of installed text is linked too, so a stale copy of it is refused as well.
            &[
                knowledge_architect::cli::this_library(),
                knowledge_architect::cli::Library {
                    crate_dir: std::path::PathBuf::from(
                        knowledge_architect_agent_skills::CRATE_DIR,
                    ),
                    package: knowledge_architect_agent_skills::PACKAGE,
                },
            ],
            &[
                env!("CARGO_PKG_NAME"),
                knowledge_architect_agent_skills::PACKAGE,
            ],
        )?;
        // A binary of another version than the project pins would judge it by other rules, or
        // install other skills over it, per `design@core@installed-binary-version-check`.
        knowledge_architect::cli::refuse_another_version(&manifest, &dirs)?;
        knowledge_architect::cli::run(cli.command, &manifest, &dirs, &mut [])
    });
    match outcome {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // clap's own consistency check over the whole derive, the core's commands flattened in:
    // two arguments claiming one name, a conflict naming an argument that does not exist. A
    // malformed declaration is otherwise found at the first invocation that reaches it, per
    // `design@core@arguments-parse-through-clap`.
    #[test]
    fn cli_declaration_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    /// The claim: an argument nobody declared is refused while parsing, which exits 2, rather
    /// than ignored.
    #[test]
    fn an_unknown_argument_is_refused() {
        assert!(Cli::try_parse_from(["klarch", "check"]).is_ok());
        assert!(Cli::try_parse_from(["klarch", "check", "--only", "references"]).is_err());
        assert!(Cli::try_parse_from(["klarch", "no-such-command"]).is_err());
    }

    /// The claim: `--fix` and `--staged` are refused together while parsing, since `--fix`
    /// repairs the working tree and `--staged` judges the index.
    #[test]
    fn fix_and_staged_are_refused_together() {
        assert!(Cli::try_parse_from(["klarch", "check", "--staged"]).is_ok());
        assert!(Cli::try_parse_from(["klarch", "check", "--fix"]).is_ok());
        assert!(Cli::try_parse_from(["klarch", "check", "--fix", "--staged"]).is_err());
    }
}
