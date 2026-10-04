//! Workflow automation for this repository. One subcommand per workflow.
//!
//! What each subcommand does and how to invoke it is `path@xtask@README.md`; why the tool is
//! shaped this way is `path@xtask@docs/design.md`. Adding a subcommand is one module plus one
//! variant below, per `design@xtask@one-module-per-subcommand`.

mod gates;

use clap::{Parser, Subcommand};
use knowledge_architect_gates::GatesArgs;
use std::process::ExitCode;

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration sees it unset, and still compiles.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

#[derive(Parser)]
#[command(name = "xtask", about = "Workflow automation for this repository.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the CI gates, keep full logs, print what needs acting.
    Gates(GatesArgs),
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Gates(args) => gates::run(&args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // clap's own consistency check over the whole derive: conflicting flags, broken
    // defaults. It is what catches a malformed argument declaration at test time
    // rather than at first invocation.
    #[test]
    fn cli_declaration_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    // The claim: the two flags parse, both default to off, and an unknown flag is a
    // refusal rather than a silent acceptance — the failure the hand-rolled parsers had.
    #[test]
    fn gates_flags_parse_and_default_off() {
        let Command::Gates(defaults) = Cli::parse_from(["xtask", "gates"]).command;
        assert!(!defaults.fail_fast);
        assert!(!defaults.full);

        let Command::Gates(both) =
            Cli::parse_from(["xtask", "gates", "--fail-fast", "--full"]).command;
        assert!(both.fail_fast);
        assert!(both.full);

        assert!(Cli::try_parse_from(["xtask", "gates", "--no-such-flag"]).is_err());
    }
}
