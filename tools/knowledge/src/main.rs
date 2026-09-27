//! The one command this repository runs, `cargo knowledge`: the core's commands and the rules
//! corpus's.
//!
//! The core's commands, their handlers and what they print are `documentation::cli`, flattened
//! into [`Command`] here, per `design@knowledge@the-core-cli-is-a-library-module`. This binary
//! adds the `rules` commands and the one fact only it can know: the directory of its own source.
//!
//! Arguments are declared, never parsed by hand, per `design@thaum@arguments-parse-through-clap`, and
//! the exit codes are `design@thaum@exit-code-ladder`: 0 ran-and-clean, 1 ran-and-negative, 2
//! could-not-run.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod corpus_cmd;

/// `outln!` for the commands of this binary, through the core's writer, so a reader that closes
/// the pipe early ends the run with exit 2 rather than a panic.
macro_rules! outln {
    () => { documentation::cli::output::write(format_args!("\n")) };
    ($($arg:tt)*) => {
        documentation::cli::output::write(format_args!("{}\n", format_args!($($arg)*)))
    };
}
pub(crate) use outln;

#[derive(Parser)]
#[command(
    name = "knowledge",
    about = "What this project knows, and whether it still holds.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(flatten)]
    Core(documentation::cli::Command),
    /// The corpus and its releases.
    Rules {
        #[command(subcommand)]
        command: corpus_cmd::RulesCommand,
    },
}

fn main() -> ExitCode {
    // Parsed before the project is located, so `--help` answers from anywhere and a mistyped
    // invocation is refused without a walk. clap exits 2 on a parse failure, which is already
    // this project's could-not-run code.
    let cli = Cli::parse();

    let outcome = documentation::cli::locate().and_then(|manifest| match cli.command {
        Command::Core(command) => {
            // This repository's own subject, the Comprehensive Rules, is an extension of the
            // core, per `design@thaum@knowledge-is-a-generic-core`.
            let mut extensions: Vec<Box<dyn documentation::extension::Extension>> = vec![Box::new(
                citations::rules_extension::RulesExtension::default(),
            )];
            let dirs = checker_sources();
            let dirs: Vec<&Path> = dirs.iter().map(PathBuf::as_path).collect();
            documentation::cli::run(command, &manifest, &dirs, &mut extensions)
        }
        Command::Rules { command } => corpus_cmd::run(&manifest, &command),
    });

    match outcome {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

/// The checker's own source directories, compiled in: one per Component its libraries belong
/// to, per `design@knowledge@checker-source-literals-are-data`.
///
/// Every model this binary builds is told them, so that the tool's own fixtures are read as
/// data rather than as citations. The alias in `path@thaum@.cargo/config.toml` builds the binary
/// from the checkout on every run, so each compiled path names the tree being checked. A binary
/// built elsewhere names directories the walk never visits, exempts nothing, and the summary
/// block's `checker source` line shows the count at zero.
pub(crate) fn checker_sources() -> Vec<PathBuf> {
    let mut dirs = vec![documentation::component_dir(), citations::component_dir()];
    dirs.dedup();
    dirs
}

#[cfg(test)]
mod tests {
    use super::{Cli, Command};
    use crate::corpus_cmd::RulesCommand;
    use clap::Parser;
    use documentation::cli::Command as Core;

    // Arguments handed to the parser, and what it must hand back: each is named because it
    // appears on both sides of an assertion.
    const RULE: &str = "601.2";
    const ANOTHER_RULE: &str = "104.1";
    const OLD: &str = "20260807";
    const NEW: &str = "20260819";

    // clap's own consistency check over the whole derive: a flag that conflicts with an
    // argument that does not exist, a broken default, two arguments claiming one name. It is
    // what catches a malformed declaration at test time rather than at first invocation.
    #[test]
    fn cli_declaration_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    /// The claim: every combination the hand-rolled parser accepted is refused, and refused
    /// while parsing rather than by a check inside a command.
    ///
    /// Each row below exited 0 before this migration, having done something other than what
    /// was asked — the first silently reported the project empty, and the rest ignored a flag
    /// or an argument. That is the discrimination: they are observed passing against the
    /// implementation this replaces.
    #[test]
    fn every_recorded_silent_acceptance_is_refused() {
        for argv in [
            // Set from each other's absence, the pair selected neither kind.
            // The command `issues` and `tripwires` replaced, and the flags it carried.
            vec!["knowledge", "outstanding"],
            vec!["knowledge", "issues", "--issues"],
            vec!["knowledge", "tripwires", "--tripwires"],
            // `show` cannot run without the reference it prints.
            vec!["knowledge", "show"],
            // Unknown flags, accepted by every subcommand.
            vec!["knowledge", "check", "--bogus"],
            vec!["knowledge", "issues", "--bogus"],
            vec![
                "knowledge",
                "rules",
                "diff",
                "--bogus",
                "--old",
                OLD,
                "--new",
                NEW,
            ],
            // Positional junk, filtered out of the arguments and never refused.
            vec!["knowledge", "check", "stray"],
            vec!["knowledge", "model", "zzz"],
            // A flag on a command that has none, silently dropped.
            vec!["knowledge", "rules", "show", "--write", RULE],
            // The flags `index` no longer has, one pair of which wrote a file the gate rejects.
            vec!["knowledge", "index", "--write"],
            vec!["knowledge", "index", "--interpretations"],
            vec!["knowledge", "index", "--lines"],
            // Dates by position, where a swap produces a reversed work list in silence.
            vec!["knowledge", "rules", "diff", OLD, NEW],
            vec!["knowledge", "rules", "diff", "--old", OLD],
            // A date `release::url_for` would slice the first four bytes of. Unvalidated,
            // each of these panicked at exit 101, which the ladder has no meaning for.
            vec!["knowledge", "rules", "diff", "--old", "202", "--new", NEW],
            vec!["knowledge", "rules", "diff", "--old", OLD, "--new", ""],
            vec!["knowledge", "rules", "fetch", "not-a-date"],
            vec!["knowledge", "rules", "bump", "2026080"],
            // Arguments a command cannot run without.
            vec!["knowledge", "rules", "show"],
            vec!["knowledge", "rules", "bump"],
            vec!["knowledge"],
        ] {
            assert!(
                Cli::try_parse_from(argv.iter().copied()).is_err(),
                "{argv:?} must be refused"
            );
        }
    }

    /// The claim: what the tool does accept parses to the command and values it names.
    ///
    /// The refusals above are satisfied by a declaration that refuses everything, so this is
    /// the half that says the interface still exists.
    #[test]
    fn the_accepted_shapes_parse_to_what_they_name() {
        assert!(matches!(
            Cli::parse_from(["knowledge", "check"]).command,
            Command::Core(Core::Check)
        ));
        // Every check runs on every run: a selection is refused as an unknown argument.
        assert!(Cli::try_parse_from(["knowledge", "check", "--only", "references"]).is_err());

        let Command::Core(Core::Show(shown)) =
            Cli::parse_from(["knowledge", "show", "design@a@b"]).command
        else {
            panic!("show parses to the show subcommand");
        };
        assert_eq!(shown.reference, "design@a@b");

        let Command::Core(Core::Issues(listed)) = Cli::parse_from([
            "knowledge",
            "issues",
            "--kind",
            "defect",
            "--group",
            "layers",
            "two",
            "words",
        ])
        .command
        else {
            panic!("issues parses to the issues subcommand");
        };
        assert_eq!(listed.kind.as_deref(), Some("defect"));
        assert_eq!(listed.group.as_deref(), Some("layers"));
        assert_eq!(listed.terms, ["two", "words"]);

        let Command::Core(Core::Tripwires(guarded)) =
            Cli::parse_from(["knowledge", "tripwires", "--guarding", "design@a@b"]).command
        else {
            panic!("tripwires parses to the tripwires subcommand");
        };
        assert_eq!(guarded.guarding.as_deref(), Some("design@a@b"));
        assert!(guarded.terms.is_empty());

        let Command::Rules { command } =
            Cli::parse_from(["knowledge", "rules", "diff", "--old", OLD, "--new", NEW]).command
        else {
            panic!("rules parses to the rules subcommand");
        };
        let RulesCommand::Diff { old, new } = command else {
            panic!("diff parses to the diff verb");
        };
        // Named, so this assertion could not pass with the two the wrong way round.
        assert_eq!((old.as_str(), new.as_str()), (OLD, NEW));

        let Command::Rules { command } =
            Cli::parse_from(["knowledge", "rules", "show", RULE, ANOTHER_RULE]).command
        else {
            panic!("rules parses to the rules subcommand");
        };
        let RulesCommand::Show { numbers } = command else {
            panic!("show parses to the show verb");
        };
        assert_eq!(numbers.len(), 2, "several numbers at once, in order given");

        assert!(matches!(
            Cli::parse_from(["knowledge", "index"]).command,
            Command::Core(Core::Index)
        ));
    }
}
