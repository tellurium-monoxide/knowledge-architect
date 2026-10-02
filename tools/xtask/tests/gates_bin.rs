//! End-to-end runs of the built `xtask` binary against a fake project whose `cargo` is a
//! shell stub. The stub is what keeps these tests off the real gates: no test here may
//! invoke the real `cargo x gates`, per `path@xtask@CLAUDE.md`, or the suite would run itself.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A directory holding the checker's manifest (so the tool accepts it as a project root) and,
/// in its `bin`, a `cargo` stub that prints one line per invocation — with the value of
/// `RUSTC_BOOTSTRAP` it received — sleeps briefly so the caller can act while gates are
/// still running, and fails when `FAIL_ON` names a substring of its arguments.
fn fake_project(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("xtask-gates-bin-{name}-{}", std::process::id()));
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).expect("the fake project can be created");
    std::fs::write(dir.join("knowledge-architect.toml"), "")
        .expect("the root marker can be written");
    let stub = r#"#!/bin/sh
sleep 0.2
echo "fake cargo: $* bootstrap=[$RUSTC_BOOTSTRAP]"
case "$*" in
  *"$FAIL_ON"*) [ -n "$FAIL_ON" ] && { echo "induced failure"; exit 1; };;
esac
exit 0
"#;
    let cargo = bin.join("cargo");
    std::fs::write(&cargo, stub).expect("the stub can be written");
    let mut permissions = std::fs::metadata(&cargo)
        .expect("the stub exists")
        .permissions();
    use std::os::unix::fs::PermissionsExt;
    permissions.set_mode(0o755);
    std::fs::set_permissions(&cargo, permissions).expect("the stub can be made executable");
    dir
}

fn xtask_in(project: &Path, fail_on: &str) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_xtask"));
    let path = format!(
        "{}:{}",
        project.join("bin").display(),
        std::env::var("PATH").expect("PATH is set")
    );
    command
        .arg("gates")
        .current_dir(project)
        .env("PATH", path)
        .env("FAIL_ON", fail_on)
        // Hostile on purpose: every test asserts the tool scrubs it from its children.
        .env("RUSTC_BOOTSTRAP", "1")
        // CI runs this suite under Actions, and the variable turns the pipe refusal off and
        // the annotations on; a test that wants either sets it back explicitly.
        .env_remove("GITHUB_ACTIONS")
        .stdin(Stdio::null());
    command
}

/// Make the fake project a git repository with a base commit and one branch commit on it at
/// HEAD, and a remote-tracking origin/main at `origin_at`: `"base"` for the base, which HEAD
/// contains and which does not contain HEAD, anything else for a commit on a side line from the
/// base, which HEAD does not contain.
fn with_history(project: &Path, origin_at: &str) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(project)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };
    git(&["init", "-q"]);
    git(&["config", "user.name", "fixture"]);
    git(&["config", "user.email", "fixture@example.invalid"]);
    git(&["commit", "-q", "--allow-empty", "-m", "base"]);
    let base = git(&["rev-parse", "HEAD"]);
    git(&["commit", "-q", "--allow-empty", "-m", "the branch"]);
    let target = if origin_at == "base" {
        base
    } else {
        // A commit on a side line from the base: main moved, and the branch was not rebased.
        let tree = git(&["rev-parse", "HEAD^{tree}"]);
        git(&["commit-tree", &tree, "-p", &base, "-m", "main moved"])
    };
    git(&["update-ref", "refs/remotes/origin/main", &target]);
}

// The claim behind `design@gates@verdict-from-exit-codes`'s pipe refusal: a pipe on stdout is
// refused before any gate runs, because the pipe's reader would replace the exit code the run
// exists to deliver. A harness capturing through a pipe is exactly that shape, so this is the
// one test that captures through one. Mutation check: dropping the refusal makes every fake
// gate run and pass, and both assertions name it.
#[test]
fn a_piped_stdout_is_refused_before_any_gate_runs() {
    let project = fake_project("pipe");
    let output = xtask_in(&project, "")
        .stdout(Stdio::piped())
        .output()
        .expect("the built xtask binary runs");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success() && stderr.contains("does not write its report into a pipe"),
        "the pipe must be refused, with the reason: {status:?} {stderr}",
        status = output.status
    );
    assert!(
        !project.join("target/gates/fmt.log").is_file(),
        "no gate may run before the refusal"
    );
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: a failing gate fails the run while the gates after it still run and log —
// the run-all default, end to end. Mutation check: inverting the verdict or skipping
// later gates is caught by the exit assertion and the test.log presence.
#[test]
fn a_failing_gate_fails_the_run_and_the_rest_still_log() {
    let project = fake_project("redgate");
    let (status, stdout, _) = gates_output(&project, "clippy");
    assert!(!status.success());
    assert!(stdout.contains("clippy     FAILED"));
    assert!(stdout.contains("induced failure"), "the extract is shown");
    assert!(project.join("target/gates/test.log").is_file());
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: a gate's child never inherits a caller-exported RUSTC_BOOTSTRAP either — a
// local `cargo x gates` under that export would otherwise pass nightly-gated code that CI
// rejects, the false-green class the verdict run exists to exclude. Mutation check:
// dropping the `env_remove` in the gates library's spawn helper is caught by the bootstrap=[] assertion, since
// `xtask_in` exports the hostile value.
#[test]
fn gate_children_never_inherit_a_caller_exported_bootstrap() {
    let project = fake_project("scrubbed");
    let (status, _, _) = gates_output(&project, "");
    assert!(status.success());
    let log = std::fs::read_to_string(project.join("target/gates/test.log"))
        .expect("the test gate log was written");
    assert!(
        log.contains("bootstrap=[]"),
        "the export was scrubbed: {log}"
    );
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: `--locked` reaches every gate that resolves dependencies, before any `--`, and
// no other. The fake cargo prints its arguments into each gate's log. Mutation check:
// appending the flag after `--` puts it in the child's own arguments and fails the
// check and clippy assertions; dropping it from one gate fails that gate's assertion.
#[test]
fn locked_reaches_every_gate_that_resolves_dependencies() {
    let project = fake_project("locked");
    let (status, stdout, _) = gates_output_with(&project, "", &["--locked"], &[]);
    assert!(status.success(), "{stdout}");
    let log = |gate: &str| {
        std::fs::read_to_string(project.join(format!("target/gates/{gate}.log")))
            .unwrap_or_else(|_| panic!("the {gate} log was written"))
    };
    assert!(!log("fmt").contains("--locked"), "{}", log("fmt"));
    assert!(
        log("check").contains("--locked -- check"),
        "{}",
        log("check")
    );
    assert!(
        log("commits").contains("--locked -- commits"),
        "{}",
        log("commits")
    );
    assert!(
        log("clippy").contains("--locked -- -D warnings"),
        "{}",
        log("clippy")
    );
    assert!(
        log("test").contains("test --workspace --locked"),
        "{}",
        log("test")
    );
    // Without the flag, no gate receives it.
    let (_, _, _) = gates_output_with(&project, "", &[], &[]);
    assert!(!log("test").contains("--locked"), "{}", log("test"));
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: under `--require-rebased`, a HEAD that does not contain origin/main fails the
// `rebased` gate, and one that does passes it; without the flag no such gate runs. Mutation
// check: inverting the ancestry arguments passes the unrebased case and fails the rebased one.
#[test]
fn require_rebased_fails_a_branch_main_has_moved_past() {
    let project = fake_project("unrebased");
    with_history(&project, "side");
    let (status, stdout, _) = gates_output_with(&project, "", &["--require-rebased"], &[]);
    assert!(!status.success(), "{stdout}");
    assert!(stdout.contains("rebased    FAILED"), "{stdout}");
    assert!(
        stdout.contains("does not contain origin/main"),
        "the extract says why: {stdout}"
    );
    let _ = std::fs::remove_dir_all(&project);

    let project = fake_project("rebased");
    with_history(&project, "base");
    let (status, stdout, _) = gates_output_with(&project, "", &["--require-rebased"], &[]);
    assert!(status.success(), "{stdout}");
    assert!(stdout.contains("rebased    ok"), "{stdout}");
    let _ = std::fs::remove_dir_all(&project);
}

#[test]
fn without_require_rebased_no_rebased_gate_runs() {
    let project = fake_project("norebase");
    with_history(&project, "side");
    let (status, stdout, _) = gates_output_with(&project, "", &[], &[]);
    assert!(status.success(), "{stdout}");
    assert!(!stdout.contains("rebased"), "{stdout}");
    assert!(!project.join("target/gates/rebased.log").exists());
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: the variable must say `true`; any other value leaves the refusal in place.
// Mutation check: reading the variable's presence alone is caught here.
#[test]
fn a_piped_stdout_is_refused_when_the_actions_variable_is_not_true() {
    let project = fake_project("actions-false");
    let output = xtask_in(&project, "")
        .env("GITHUB_ACTIONS", "false")
        .stdout(Stdio::piped())
        .output()
        .expect("the built xtask binary runs");
    assert!(!output.status.success());
    assert!(!project.join("target/gates/fmt.log").is_file());
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: under Actions, a pipe on stdout is accepted, because the reader there is the
// runner, which takes the verdict from the exit code. Mutation check: dropping the
// `under_actions` condition from the refusal is caught by the success assertion.
#[test]
fn a_piped_stdout_is_accepted_under_actions() {
    let project = fake_project("actions-pipe");
    let output = xtask_in(&project, "")
        .env("GITHUB_ACTIONS", "true")
        .stdout(Stdio::piped())
        .output()
        .expect("the built xtask binary runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(project.join("target/gates/test.log").is_file());
    let _ = std::fs::remove_dir_all(&project);
}

// The claim: under Actions each gate is wrapped in a group and a failed gate prints one error
// annotation naming it; outside Actions neither appears. Mutation check: printing the
// annotations unconditionally is caught by the second half.
#[test]
fn annotations_appear_under_actions_alone() {
    let project = fake_project("annotations");
    let (status, stdout, _) =
        gates_output_with(&project, "clippy", &[], &[("GITHUB_ACTIONS", "true")]);
    assert!(!status.success());
    assert!(stdout.contains("::group::fmt\n"), "{stdout}");
    // The verdict line follows the group's end, so it stays visible with the group folded.
    assert!(
        stdout.contains("::endgroup::\nclippy     FAILED"),
        "{stdout}"
    );
    assert_eq!(stdout.matches("::error ").count(), 1, "{stdout}");
    assert!(stdout.contains("::error title=clippy::"), "{stdout}");

    let (_, stdout, _) = gates_output_with(&project, "clippy", &[], &[]);
    assert!(!stdout.contains("::group::"), "{stdout}");
    assert!(!stdout.contains("::error"), "{stdout}");
    let _ = std::fs::remove_dir_all(&project);
}

/// One `gates` run with stdout redirected to a file, as a caller who wants the text does it:
/// the subcommand refuses a pipe on stdout, so a harness reads it back from a regular file.
/// Returns the exit status, stdout's text and stderr's text.
fn gates_output(project: &Path, fail_on: &str) -> (std::process::ExitStatus, String, String) {
    gates_output_with(project, fail_on, &[], &[])
}

/// The same, with extra `gates` flags and extra environment variables.
fn gates_output_with(
    project: &Path,
    fail_on: &str,
    flags: &[&str],
    envs: &[(&str, &str)],
) -> (std::process::ExitStatus, String, String) {
    let stdout_path = project.join("stdout.txt");
    let file = std::fs::File::create(&stdout_path).expect("the stdout file can be created");
    let output = xtask_in(project, fail_on)
        .args(flags)
        .envs(envs.iter().copied())
        .stdout(file)
        .stderr(Stdio::piped())
        .output()
        .expect("the built xtask binary runs");
    let stdout = std::fs::read_to_string(&stdout_path).expect("stdout was written");
    (
        output.status,
        stdout,
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}
