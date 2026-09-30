//! The core's own binary: the core's commands, with no extension registered, per
//! `design@knowledge@the-core-cli-is-a-library-module`.
//!
//! A project whose manifest declares only what the core reads runs this. A project with a
//! subject of its own builds a binary that registers the extension for it.

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "knowledge",
    about = "What a project's documents hold, and whether it still holds.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: documentation::cli::Command,
}

fn main() -> ExitCode {
    // Parsed before the project is located, so `--help` answers from anywhere. clap exits 2 on
    // a parse failure, which is already the could-not-run code.
    let cli = Cli::parse();
    // The one path compiled in: the core's own Component, whose fixtures are read as data,
    // per `design@knowledge@checker-source-literals-are-data`.
    let dir = documentation::component_dir();
    let dirs: Vec<&Path> = vec![dir.as_path()];
    let outcome = documentation::cli::locate().and_then(|manifest| {
        // A binary built from another checkout would judge this tree with that checkout's
        // code, so no command runs, per `design@knowledge@a-foreign-build-is-refused`.
        documentation::build_origin::refuse_a_foreign_build(
            manifest.root(),
            &[
                // The binary's own crate first: it is rebuilt whenever anything it links is.
                documentation::build_origin::Library {
                    crate_dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
                    package: env!("CARGO_PKG_NAME"),
                },
                documentation::build_origin::this_library(),
            ],
            &[env!("CARGO_PKG_NAME"), "documentation"],
        )?;
        documentation::cli::run(cli.command, &manifest, &dirs, &mut [])
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
    // `design@knowledge@arguments-parse-through-clap`.
    #[test]
    fn cli_declaration_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    /// The claim: an argument nobody declared is refused while parsing, which exits 2, rather
    /// than ignored.
    #[test]
    fn an_unknown_argument_is_refused() {
        assert!(Cli::try_parse_from(["knowledge", "check"]).is_ok());
        assert!(Cli::try_parse_from(["knowledge", "check", "--only", "references"]).is_err());
        assert!(Cli::try_parse_from(["knowledge", "no-such-command"]).is_err());
    }
}
