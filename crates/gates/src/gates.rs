//! Running the gates: each gate in order, every byte of its output in a log, a distilled extract
//! of what failed, and one verdict from the children's exit codes.
//!
//! The output and exit-code contract is `design@gates@verdict-from-exit-codes` and
//! `design@gates@distill-over-full-logs`. Which gates a project runs is its own list, per
//! `design@gates@a-project-holds-its-gate-list`.

use crate::process::{announce, complain, run_captured, say, Completed, Spec};
use clap::Args;
use std::path::Path;
use std::process::ExitCode;

/// The gates' flags. A project's binary flattens them into its own `gates` command.
#[derive(Args, Debug, Default, Clone)]
pub struct GatesArgs {
    /// Stop at the first failing gate instead of running them all.
    #[arg(long)]
    pub fail_fast: bool,

    /// Additionally stream each gate's raw output live while it runs.
    #[arg(long)]
    pub full: bool,

    /// Pass `--locked` to every gate that resolves dependencies.
    #[arg(long)]
    pub locked: bool,

    /// Run the gates marked as rebase checks: fail unless HEAD contains the main branch.
    #[arg(long)]
    pub require_rebased: bool,
}

/// What a failed gate shows on stdout, chosen from its full output. The log always holds every
/// byte, per `design@gates@distill-over-full-logs`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Distiller {
    /// The output, whole: a report already meant to be read in full, such as a formatter's or the
    /// checker's.
    Whole,
    /// The output without cargo's build-progress lines, such as a linter's.
    WithoutProgress,
    /// libtest's output reduced to each failing binary's failures block, and cargo's rerun hint.
    Tests,
    /// An ancestry check that answers by its exit code alone: a silent failure is shown as the
    /// hint that HEAD does not contain `base`.
    Ancestry {
        /// The branch HEAD must contain, such as origin/main.
        base: String,
    },
}

/// One gate: a name for the log file and the summary, the command it runs, and how its failure
/// is shown.
#[derive(Clone, Debug)]
pub struct Gate {
    /// The gate's name, which names its log, `target/gates/<name>.log`.
    pub name: String,
    /// The program the gate runs.
    pub program: String,
    /// Its arguments.
    pub args: Vec<String>,
    /// Variables added to the inherited environment, never replacing it.
    pub envs: Vec<(String, String)>,
    /// Whether the command resolves dependencies, and so takes `--locked` under that flag.
    pub resolves: bool,
    /// Whether the gate runs only under `--require-rebased`.
    pub rebased_only: bool,
    /// How a failure of this gate is shown.
    pub distiller: Distiller,
}

/// How a Rust project runs its checker through cargo: the package whose binary is the checker,
/// and the arguments before the checker's command, if the binary takes the checker's commands
/// under a command of its own.
#[derive(Clone, Copy, Debug)]
pub struct Checker<'a> {
    /// The package of the binary that runs the checker, such as `xtask`.
    pub package: &'a str,
    /// The arguments before the checker's command, such as `["klarch"]`.
    pub prefix: &'a [&'a str],
}

/// The recommended gates of a Rust project that uses the checker, in cost order: the rebase
/// check behind its flag, formatting, the document check, the commit messages of the branch,
/// clippy, and the tests. `base` is the main branch as git names it, such as origin/main. A
/// project adds, removes or reorders gates in the list it is given.
pub fn rust_project(checker: Checker, base: &str) -> Vec<Gate> {
    let owned = |args: &[&str]| args.iter().map(|a| (*a).to_owned()).collect::<Vec<_>>();
    let checker_args = |command: &[&str]| {
        let mut args = owned(&["run", "-q", "--release", "-p", checker.package, "--"]);
        args.extend(owned(checker.prefix));
        args.extend(owned(command));
        args
    };
    let gate = |name: &str, program: &str, args: Vec<String>, resolves: bool, distiller| Gate {
        name: name.to_owned(),
        program: program.to_owned(),
        args,
        envs: Vec::new(),
        resolves,
        rebased_only: false,
        distiller,
    };
    let range = format!("{base}..HEAD");
    vec![
        Gate {
            // First because it builds nothing. Behind a flag because a branch not yet rebased is
            // normal while it is worked on, per `design@gates@rebased-gate-behind-a-flag`.
            rebased_only: true,
            ..gate(
                "rebased",
                "git",
                owned(&["merge-base", "--is-ancestor", base, "HEAD"]),
                false,
                Distiller::Ancestry {
                    base: base.to_owned(),
                },
            )
        },
        gate(
            "fmt",
            "cargo",
            owned(&["fmt", "--all", "--check"]),
            false,
            Distiller::Whole,
        ),
        // The explicit package form rather than a cargo alias, so the gate does not depend on
        // the alias file being present.
        gate(
            "check",
            "cargo",
            checker_args(&["check"]),
            true,
            Distiller::Whole,
        ),
        // The branch's own commits, each message and tree judged against that commit's tree.
        // It runs after `check` because that gate builds the binary this one invokes, and
        // before clippy because it is cheaper than a workspace lint pass.
        gate(
            "commits",
            "cargo",
            checker_args(&["commits", &range]),
            true,
            Distiller::Whole,
        ),
        gate(
            "clippy",
            "cargo",
            owned(&[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ]),
            true,
            Distiller::WithoutProgress,
        ),
        // Plain on purpose: this is the verdict run, and CI runs it too. The gates only surface
        // libtest's own over-60-seconds warnings.
        gate(
            "test",
            "cargo",
            owned(&["test", "--workspace"]),
            true,
            Distiller::Tests,
        ),
    ]
}

/// The gates this run executes: every gate, minus the rebase checks unless the flag asks for them.
fn selected(gates: &[Gate], require_rebased: bool) -> Vec<&Gate> {
    gates
        .iter()
        .filter(|gate| require_rebased || !gate.rebased_only)
        .collect()
}

/// A gate's arguments, with `--locked` inserted when asked for and the gate resolves.
///
/// The flag is cargo's, so it goes before the first `--`: after it, it would be an argument
/// of the child cargo runs, which `check` and `clippy` would refuse or ignore.
fn gate_args(gate: &Gate, locked: bool) -> Vec<&str> {
    let mut args: Vec<&str> = gate.args.iter().map(String::as_str).collect();
    if locked && gate.resolves {
        let at = args.iter().position(|a| *a == "--").unwrap_or(args.len());
        args.insert(at, "--locked");
    }
    args
}

/// Whether this run is a GitHub Actions job step, which the runner says by setting
/// `GITHUB_ACTIONS` to `true`. Two things follow, per `design@gates@verdict-from-exit-codes`
/// and `design@gates@annotations-under-actions`: a pipe on stdout is accepted, and each gate
/// is wrapped in workflow commands.
fn under_actions() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok_and(|v| v == "true")
}

/// A gate that ran to completion, with everything it said.
struct Outcome<'a> {
    gate: &'a Gate,
    success: bool,
    output: Vec<u8>,
}

impl Outcome<'_> {
    fn name(&self) -> &str {
        &self.gate.name
    }
}

/// Runs `gates` from the project `root`, writes every gate's full output under
/// `<root>/target/gates/`, prints one verdict line per gate and an extract per failure, and
/// returns the run's exit code: success when every gate that ran passed and every gate could
/// run. Refuses to write its report into a pipe, outside GitHub Actions.
pub fn run(root: &Path, gates: &[Gate], args: &GatesArgs) -> ExitCode {
    // Refused before any gate runs, and for this subcommand alone: the run's product is its
    // exit code, and a pipe hands that to the reader at the far end (`| tail` exits with
    // tail's status) while a filter drops lines of a report that is already distilled. A
    // regular file passes, so `> log 2>&1` stays available, and every gate's full output is
    // under `path@knowledge-architect@target/gates/` whatever stdout was.
    //
    // Under Actions the reader is the runner, which takes the step's verdict from the exit
    // code of the shell, so the refusal's reason does not hold there and the stream stays live.
    let actions = under_actions();
    if stdout_is_a_pipe() && !actions {
        return abort(
            "gates does not write its report into a pipe: the reader would replace the exit \
             code the run exists to deliver, and a filter hides lines. Read it in the terminal \
             or redirect to a file; each gate's full output is under target/gates/",
        );
    }
    let log_dir = root.join("target/gates");
    if let Err(error) = std::fs::create_dir_all(&log_dir) {
        return abort(&format!("cannot create {}: {error}", log_dir.display()));
    }

    let mut spawn_failure = false;
    let gates = selected(gates, args.require_rebased);
    let outcomes = execute(
        &gates,
        args.fail_fast,
        |gate| {
            if actions {
                say(&format!("::group::{}\n", gate.name));
            }
            if args.full {
                // The live stream needs an attributing header; the self-erasing
                // announce would be swallowed by the first streamed line anyway.
                say(&format!(
                    "\n\u{2500}\u{2500} {} \u{2500}\u{2500}\n",
                    gate.name
                ));
            } else {
                announce(&gate.name);
            }
            let gate_args = gate_args(gate, args.locked);
            let envs: Vec<(&str, &str)> = gate
                .envs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            run_captured(
                &Spec {
                    program: &gate.program,
                    args: &gate_args,
                    envs: &envs,
                    cwd: root,
                },
                args.full,
            )
        },
        |outcome| {
            if let Err(error) = std::fs::write(log_path(&log_dir, outcome.name()), &outcome.output)
            {
                // A lost log must not lose the run: the verdict and the extract still
                // carry the result, so report the loss and carry on.
                complain(&format!(
                    "gates: could not write {}.log: {error}",
                    outcome.name()
                ));
            }
            // The group ends before the verdict line, so the line stays visible with the
            // group folded; the error annotation follows it.
            if actions {
                say("::endgroup::\n");
            }
            // Padded past the announce line's width, so overwriting it leaves no tail.
            say(&format!("{:<14}\n", verdict_line(outcome)));
            if actions && !outcome.success {
                say(&error_annotation(outcome.name()));
            }
        },
        |gate, error| {
            spawn_failure = true;
            if actions {
                say("::endgroup::\n");
            }
            say(&format!("{:<10} could not run: {error}\n", gate.name));
            if actions {
                say(&error_annotation(&gate.name));
            }
        },
    );

    // Everything from here to the exit reads the outcomes and cannot change them, per
    // `design@gates@verdict-from-exit-codes`.
    for test_outcome in outcomes
        .iter()
        .filter(|outcome| outcome.gate.distiller == Distiller::Tests)
    {
        let text = String::from_utf8_lossy(&test_outcome.output);
        let slow = slow_tests(&text);
        if !slow.is_empty() {
            say(&format!(
                "slow tests {} over 60s: {} (full log: target/gates/{}.log)\n",
                slow.len(),
                slow.join(", "),
                test_outcome.name(),
            ));
        }
    }

    for outcome in outcomes.iter().filter(|outcome| !outcome.success) {
        say(&extract(outcome));
    }

    if verdict(&outcomes) && !spawn_failure {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// The error annotation a failed gate prints under Actions, which the job summary lists by the
/// gate's name.
fn error_annotation(name: &str) -> String {
    format!(
        "::error title={name}::{name} FAILED; its full log is target/gates/{name}.log, \
         published as the gate-logs artifact\n"
    )
}

/// Runs each gate in order and reports it as it completes. A gate whose child cannot be
/// spawned at all goes to `failed_to_run` and produces no outcome; every gate that ran,
/// passing or not, produces one. With `fail_fast`, the first failed gate — or the first
/// that could not spawn — ends the run.
fn execute<'a>(
    gates: &[&'a Gate],
    fail_fast: bool,
    mut runner: impl FnMut(&Gate) -> std::io::Result<Completed>,
    mut report: impl FnMut(&Outcome),
    mut failed_to_run: impl FnMut(&Gate, &std::io::Error),
) -> Vec<Outcome<'a>> {
    let mut outcomes = Vec::new();
    for &gate in gates {
        match runner(gate) {
            Ok(done) => {
                let outcome = Outcome {
                    gate,
                    success: done.success,
                    output: done.output,
                };
                report(&outcome);
                let failed = !outcome.success;
                outcomes.push(outcome);
                if fail_fast && failed {
                    break;
                }
            }
            Err(error) => {
                failed_to_run(gate, &error);
                if fail_fast {
                    break;
                }
            }
        }
    }
    outcomes
}

/// The whole exit contract: true iff every gate that ran passed. Nothing about output
/// handling participates, per `design@gates@verdict-from-exit-codes`.
fn verdict(outcomes: &[Outcome]) -> bool {
    outcomes.iter().all(|outcome| outcome.success)
}

fn verdict_line(outcome: &Outcome) -> String {
    if outcome.success {
        format!("{:<10} ok", outcome.name())
    } else {
        format!(
            "{:<10} FAILED    full log: target/gates/{}.log",
            outcome.name(),
            outcome.name()
        )
    }
}

/// What a failed gate shows on stdout: the gate's distiller over the full text, headed by
/// the log path. A distiller chooses what to print and nothing more, per
/// `design@gates@distill-over-full-logs`; the log always holds every byte.
fn extract(outcome: &Outcome) -> String {
    let text = String::from_utf8_lossy(&outcome.output);
    let distilled = match &outcome.gate.distiller {
        Distiller::WithoutProgress => without_progress(&text),
        Distiller::Tests => distill_test(&text),
        // `merge-base --is-ancestor` answers "no" by its exit code alone and prints nothing.
        Distiller::Ancestry { base } if text.trim().is_empty() => {
            format!("HEAD does not contain {base}: fetch it and rebase the branch on it\n")
        }
        // A formatter's and the checker's reports are already meant to be read whole.
        _ => text.into_owned(),
    };
    format!(
        "\n\u{2500}\u{2500} {} \u{2500}\u{2500} full log: target/gates/{}.log\n{}",
        outcome.name(),
        outcome.name(),
        distilled,
    )
}

/// Cargo's build-progress verbs. These lines say what cargo is doing, not what is wrong,
/// and on a workspace build they are the bulk of a failing gate's output.
fn is_progress(line: &str) -> bool {
    [
        "Compiling ",
        "Checking ",
        "Building ",
        "Fresh ",
        "Finished ",
        "Downloading ",
        "Downloaded ",
        "Updating ",
        "Locking ",
        "Adding ",
        "Blocking ",
    ]
    .iter()
    .any(|verb| line.trim_start().starts_with(verb))
}

fn without_progress(text: &str) -> String {
    let mut kept = String::new();
    for line in text.lines().filter(|line| !is_progress(line)) {
        kept.push_str(line);
        kept.push('\n');
    }
    kept
}

/// The test gate's distiller. libtest prints one line per passing test; what needs acting
/// is each binary's `failures:` block, which carries the captured panics and the failing
/// names, through its closing `test result:` line — plus the `Running`/`Doc-tests` header
/// that says which binary it was, and cargo's own `error:` rerun hint.
///
/// Anything without a `failures:` block — a compile failure, a format this function does
/// not know — passes through whole, minus progress lines. Unrecognized output is shown,
/// never hidden: the distiller's safe direction is over-reporting.
fn distill_test(text: &str) -> String {
    if !text.lines().any(|line| line.trim() == "failures:") {
        return without_progress(text);
    }
    let mut kept = String::new();
    let mut target: Option<&str> = None;
    let mut in_failures = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("Running ") || trimmed.starts_with("Doc-tests ") {
            target = Some(line);
            continue;
        }
        if line.trim() == "failures:" && !in_failures {
            in_failures = true;
            if let Some(header) = target.take() {
                kept.push_str(header);
                kept.push('\n');
            }
        }
        if in_failures {
            kept.push_str(line);
            kept.push('\n');
            if line.starts_with("test result:") {
                in_failures = false;
                kept.push('\n');
            }
        } else if trimmed.starts_with("error:") {
            kept.push_str(line);
            kept.push('\n');
        }
    }
    kept
}

fn log_path(log_dir: &Path, name: &str) -> std::path::PathBuf {
    log_dir.join(format!("{name}.log"))
}

/// Stable libtest prints this for a test that crosses 60 seconds while others run
/// concurrently — a single-threaded run (`RUST_TEST_THREADS=1`) emits no such warning.
/// The gates surface those lines because a slow test is noticed when the suite drags, and
/// the warning is otherwise buried among hundreds of passing lines.
const SLOW_MARKER: &str = " has been running for over 60 seconds";

fn slow_tests(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| line.strip_suffix(SLOW_MARKER)?.strip_prefix("test "))
        .collect()
}

fn abort(message: &str) -> ExitCode {
    complain(&format!("gates: {message}"));
    ExitCode::FAILURE
}

/// Whether stdout is a FIFO: a shell pipe, or a harness capturing through one. A terminal, a
/// regular file and the null device are not. Outside unix nothing is asked and nothing is
/// refused.
#[cfg(unix)]
fn stdout_is_a_pipe() -> bool {
    use std::os::fd::AsFd;
    use std::os::unix::fs::FileTypeExt;
    std::io::stdout()
        .as_fd()
        .try_clone_to_owned()
        .map(std::fs::File::from)
        .and_then(|file| file.metadata())
        .map(|metadata| metadata.file_type().is_fifo())
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn stdout_is_a_pipe() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gate(name: &str, distiller: Distiller) -> Gate {
        Gate {
            name: name.to_owned(),
            program: String::new(),
            args: Vec::new(),
            envs: Vec::new(),
            resolves: false,
            rebased_only: false,
            distiller,
        }
    }

    fn fake<'a>(gate: &'a Gate, success: bool, output: &str) -> Outcome<'a> {
        Outcome {
            gate,
            success,
            output: output.as_bytes().to_vec(),
        }
    }

    // The claim behind `design@gates@verdict-from-exit-codes`: one failed gate fails the run,
    // whichever position it holds. Mutation check: replacing `all` with a check of the
    // last outcome alone is caught by `one_failure_anywhere_fails_the_run`'s first case.
    #[test]
    fn all_passing_gates_pass_the_run() {
        let (fmt, test) = (
            gate("fmt", Distiller::Whole),
            gate("test", Distiller::Tests),
        );
        let outcomes = [fake(&fmt, true, ""), fake(&test, true, "")];
        assert!(verdict(&outcomes));
    }

    #[test]
    fn one_failure_anywhere_fails_the_run() {
        let (fmt, test) = (
            gate("fmt", Distiller::Whole),
            gate("test", Distiller::Tests),
        );
        let first = [fake(&fmt, false, ""), fake(&test, true, "")];
        let last = [fake(&fmt, true, ""), fake(&test, false, "")];
        assert!(!verdict(&first));
        assert!(!verdict(&last));
    }

    #[test]
    fn no_gates_is_a_pass() {
        assert!(verdict(&[]));
    }

    // The claim: every gate runs whatever the earlier ones returned — the run-all default
    // of `design@gates@gates-run-all`. Mutation check: an early `break` on failure is caught by
    // the second gate's presence.
    #[test]
    fn a_failing_gate_does_not_stop_the_ones_after_it() {
        let (a, b) = (gate("a", Distiller::Whole), gate("b", Distiller::Whole));
        let gates = [&a, &b];
        let outcomes = execute(
            &gates,
            false,
            |_| {
                Ok(Completed {
                    output: Vec::new(),
                    success: false,
                })
            },
            |_| {},
            |_, _| panic!("nothing fails to spawn here"),
        );
        assert_eq!(
            outcomes.iter().map(|o| o.name()).collect::<Vec<_>>(),
            ["a", "b"]
        );
    }

    // The claim: fail-fast ends the run at the first failure — the later gate's runner is
    // never invoked. Mutation check: breaking after the loop body regardless of the flag
    // is caught by the run-all test above; ignoring the flag is caught here.
    #[test]
    fn fail_fast_stops_at_the_first_failure() {
        let (a, b) = (gate("a", Distiller::Whole), gate("b", Distiller::Whole));
        let gates = [&a, &b];
        let mut invoked = 0;
        let outcomes = execute(
            &gates,
            true,
            |_| {
                invoked += 1;
                Ok(Completed {
                    output: Vec::new(),
                    success: false,
                })
            },
            |_| {},
            |_, _| panic!("nothing fails to spawn here"),
        );
        assert_eq!(invoked, 1);
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].name(), "a");
    }

    // The claim: a gate that cannot spawn is reported through its own channel and yields
    // no outcome, so it cannot be mistaken for a pass. Mutation check: pushing a
    // success-shaped outcome on spawn failure is caught by the emptiness assertion.
    #[test]
    fn a_gate_that_cannot_spawn_yields_no_outcome() {
        let a = gate("a", Distiller::Whole);
        let gates = [&a];
        let mut reported = Vec::new();
        let outcomes = execute(
            &gates,
            false,
            |_| Err(std::io::Error::other("no such program")),
            |_| panic!("nothing completes here"),
            |gate, _| reported.push(gate.name.clone()),
        );
        assert!(outcomes.is_empty());
        assert_eq!(reported, ["a"]);
    }

    /// The claim: the recommended list is in cost order, keeps the rebase check behind its
    /// flag, holds the range check over the branch's own commits, and runs the checker through
    /// the package and prefix it is given.
    ///
    /// A project's CI runs the list it builds from this one, so a silent reorder or a dropped
    /// gate is caught here rather than by a reader.
    #[test]
    fn the_rust_list_is_in_cost_order_and_holds_the_range_check() {
        let gates = rust_project(
            Checker {
                package: "xtask",
                prefix: &["klarch"],
            },
            "origin/main",
        );
        let names: Vec<&str> = gates.iter().map(|gate| gate.name.as_str()).collect();
        assert_eq!(
            names,
            ["rebased", "fmt", "check", "commits", "clippy", "test"]
        );
        assert!(gates[0].rebased_only);
        assert!(gates[1..].iter().all(|gate| !gate.rebased_only));
        let commits = &gates[3];
        assert_eq!(
            commits.args,
            [
                "run",
                "-q",
                "--release",
                "-p",
                "xtask",
                "--",
                "klarch",
                "commits",
                "origin/main..HEAD"
            ]
        );
    }

    // The claim: `--locked` goes before the first `--`, and only into a gate that resolves.
    // Mutation check: appending at the end is caught by the check gate's position assertion.
    #[test]
    fn locked_goes_before_the_separator_of_a_resolving_gate() {
        let gates = rust_project(
            Checker {
                package: "p",
                prefix: &[],
            },
            "origin/main",
        );
        let check = gate_args(&gates[2], true);
        assert_eq!(
            check[..6],
            ["run", "-q", "--release", "-p", "p", "--locked"]
        );
        assert!(!gate_args(&gates[1], true).contains(&"--locked"));
    }

    #[test]
    fn a_failed_gate_names_its_log_in_both_lines() {
        let clippy = gate("clippy", Distiller::WithoutProgress);
        let outcome = fake(&clippy, false, "error: something\n");
        assert!(verdict_line(&outcome).contains("target/gates/clippy.log"));
        assert!(extract(&outcome).contains("target/gates/clippy.log"));
        assert!(extract(&outcome).contains("error: something"));
    }

    #[test]
    fn a_passing_gate_prints_ok_and_no_log_pointer() {
        let fmt = gate("fmt", Distiller::Whole);
        let outcome = fake(&fmt, true, "");
        let line = verdict_line(&outcome);
        assert!(line.contains("ok"));
        assert!(!line.contains(".log"));
    }

    // The claim: the clippy distiller removes what cargo is doing and keeps what is
    // wrong, context lines included. Mutation check: an identity distiller is caught by
    // the absence assertions; one keyed on `error`/`warning` prefixes alone is caught by
    // the indented context line surviving.
    #[test]
    fn clippy_extract_drops_progress_and_keeps_diagnostics() {
        let text = "   Compiling clap v4.6.6\n\
                        Checking xtask v0.0.0 (/repo/tools/xtask)\n\
                    error: unused variable: `phase`\n\
                      --> crates/thaum-engine/src/turn.rs:88:9\n\
                    warning: this could be a `let` binding\n\
                        Finished `dev` profile [unoptimized] target(s) in 1.17s\n";
        let kept = without_progress(text);
        assert!(!kept.contains("Compiling"));
        assert!(!kept.contains("Checking"));
        assert!(!kept.contains("Finished"));
        assert!(kept.contains("error: unused variable: `phase`"));
        assert!(kept.contains("--> crates/thaum-engine/src/turn.rs:88:9"));
        assert!(kept.contains("warning: this could be a `let` binding"));
    }

    const FAILING_SUITE: &str = "\
     Running unittests src/lib.rs (target/debug/deps/thaum_engine-abc123)

running 3 tests
test turn::tests::cr_500_1_phases_in_order ... ok
test turn::tests::first_strike_damage ... FAILED
test turn::tests::untap_is_simultaneous ... ok

failures:

---- turn::tests::first_strike_damage stdout ----
thread 'turn::tests::first_strike_damage' panicked at 'assertion failed'

failures:
    turn::tests::first_strike_damage

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p thaum-engine --lib`
";

    // The claim: the test distiller keeps the whole failures block — captured panic,
    // failing names, result line — the binary header attributing it, and cargo's rerun
    // hint, and drops the per-test lines of passing tests. Mutation checks: an identity
    // distiller is caught by the `... ok` absence; ending the block at the first blank
    // line instead of `test result:` is caught by the names-list assertion; dropping the
    // header scan is caught by the `Running` assertion.
    #[test]
    fn test_extract_keeps_failures_blocks_and_their_binary() {
        let kept = distill_test(FAILING_SUITE);
        assert!(kept.contains("Running unittests src/lib.rs"));
        assert!(kept.contains("panicked at 'assertion failed'"));
        assert!(kept.contains("    turn::tests::first_strike_damage"));
        assert!(kept.contains("test result: FAILED."));
        assert!(kept.contains("error: test failed, to rerun pass `-p thaum-engine --lib`"));
        assert!(!kept.contains("... ok"));
    }

    // The claim: only the binary that owns a failures block is named; a binary whose
    // tests all passed contributes nothing. Mutation check: pushing every header rather
    // than the pending one is caught by the passing binary's absence.
    #[test]
    fn test_extract_names_only_the_failing_binary() {
        let text = format!(
            "     Running unittests src/main.rs (target/debug/deps/mutate-def456)\n\n\
             running 1 test\ntest all_good ... ok\n\n\
             test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\
             {FAILING_SUITE}"
        );
        let kept = distill_test(&text);
        assert!(!kept.contains("mutate-def456"));
        assert!(kept.contains("thaum_engine-abc123"));
    }

    // The claim behind the fallback: output with no failures block — a compile failure —
    // passes through whole, minus progress, so nothing unrecognized is hidden. Mutation
    // check: returning an empty string when no block is found is caught by every
    // assertion here.
    #[test]
    fn test_extract_passes_a_compile_failure_through() {
        let text = "   Compiling thaum-engine v0.0.0\n\
                    error[E0308]: mismatched types\n\
                      --> crates/thaum-engine/src/turn.rs:12:5\n\
                    error: could not compile `thaum-engine` (lib) due to 1 previous error\n";
        let kept = distill_test(text);
        assert!(kept.contains("error[E0308]: mismatched types"));
        assert!(kept.contains("--> crates/thaum-engine/src/turn.rs:12:5"));
        assert!(kept.contains("could not compile"));
        assert!(!kept.contains("Compiling"));
    }

    #[test]
    fn fmt_and_check_extracts_pass_through_whole() {
        let check = gate("check", Distiller::Whole);
        let outcome = fake(&check, false, "some finding\nFAILED: 1 findings above\n");
        let shown = extract(&outcome);
        assert!(shown.contains("some finding"));
        assert!(shown.contains("FAILED: 1 findings above"));
    }

    // The claim: a silent ancestry failure is shown as the hint naming the base it was given.
    // Mutation check: a hint with a fixed branch name fails the base assertion.
    #[test]
    fn a_silent_ancestry_failure_names_its_base() {
        let rebased = gate(
            "rebased",
            Distiller::Ancestry {
                base: "upstream/trunk".to_owned(),
            },
        );
        let shown = extract(&fake(&rebased, false, ""));
        assert!(
            shown.contains("HEAD does not contain upstream/trunk"),
            "{shown}"
        );
    }

    // The claim: exactly the tests libtest warned about are extracted, by name, and
    // ordinary passing or failing lines contribute nothing. Mutation check: matching on
    // the substring anywhere instead of the suffix would also catch the fixture's quoted
    // mention inside another line, which the last assertion plants.
    #[test]
    fn slow_tests_are_read_off_libtest_warnings() {
        let text = "\
running 3 tests
test soak::three_sides has been running for over 60 seconds
test soak::three_sides ... ok
test view::round_trip has been running for over 60 seconds
error: a line quoting \"test x has been running for over 60 seconds\" mid-sentence
";
        assert_eq!(slow_tests(text), ["soak::three_sides", "view::round_trip"]);
    }
}
