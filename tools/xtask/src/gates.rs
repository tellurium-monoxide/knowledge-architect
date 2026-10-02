//! The `gates` subcommand: this repository's gate list, run by the published gates library.
//!
//! The list below is the primary home of what this repository's gates are, per
//! `design@xtask@gates-list-primary-home`. How they run is the library's,
//! `path@gates@docs/design.md`.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::GatesArgs;
use knowledge_architect_gates::{process::complain, project_root, rust_project, Checker, Gate};
use std::process::ExitCode;

/// This repository's gates: the recommended list of a Rust project that uses the checker, with
/// the checker being this repository's own package, built from the checkout. CI runs it through
/// `cargo x gates --locked --fail-fast --require-rebased --full`: `--locked` is a flag because
/// locally a legitimately updated `Cargo.lock` must not fail a gate, while in CI lock drift is
/// exactly what must fail.
fn gates() -> Vec<Gate> {
    rust_project(
        Checker {
            package: "knowledge-architect",
            prefix: &[],
        },
        "origin/main",
    )
}

pub fn run(args: &GatesArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(error) => return abort(&format!("no working directory: {error}")),
    };
    // The root is the nearest ancestor holding the checker's manifest, its name spelled by the
    // core's constant, per `design@xtask@root-finder-uses-the-core-constant`.
    let Some(root) = project_root(&cwd, MANIFEST_NAME) else {
        return abort(&format!(
            "not inside the project: no ancestor holds {MANIFEST_NAME}"
        ));
    };
    knowledge_architect_gates::run(&root, &gates(), args)
}

fn abort(message: &str) -> ExitCode {
    complain(&format!("xtask: {message}"));
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim: this repository runs the recommended list unchanged, with its own package as
    /// the checker and origin/main as the base. Mutation check: another package or base fails
    /// the commits gate's assertion.
    #[test]
    fn this_repository_runs_the_recommended_list_over_its_own_checker() {
        let gates = gates();
        let names: Vec<&str> = gates.iter().map(|gate| gate.name.as_str()).collect();
        assert_eq!(
            names,
            ["rebased", "fmt", "check", "commits", "clippy", "test"]
        );
        assert_eq!(
            gates[3].args,
            [
                "run",
                "-q",
                "--release",
                "-p",
                "knowledge-architect",
                "--",
                "commits",
                "origin/main..HEAD"
            ]
        );
    }
}
