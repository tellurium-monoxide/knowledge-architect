//! The core's own binary: the core's commands, with no extension registered, per
//! `design@knowledge@the-core-cli-is-a-library-module`.
//!
//! A project whose manifest declares only what the core reads runs this. A project with a
//! subject of its own builds a binary that registers the extension for it, as
//! tools/rules-corpus/ does for this repository.

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
    let outcome = documentation::cli::locate()
        .and_then(|manifest| documentation::cli::run(cli.command, &manifest, &dirs, &mut []));
    match outcome {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
