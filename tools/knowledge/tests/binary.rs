//! The binary itself, run as a caller runs it.
//!
//! Everything else here tests the library, so the argument handling, the default selection
//! and the gates in `check()` were reachable by no test at all. What that cost is the shape
//! this file exists to prevent: the default selection could be changed to any subset and the
//! whole suite still passed, so `check` could stop verifying every rule quote and exit 0.
//!
//! Each test runs against a mock project under `tests/projects/`, which the manifest excludes
//! from this repository's own walk, so the binary finds that project by walking up from the
//! working directory exactly as it would find any other.

use std::path::PathBuf;
use std::process::Command;

fn project(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

/// Stdout, stderr and the exit code of one run in one mock project.
fn run(name: &str, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_knowledge"))
        .args(args)
        .current_dir(project(name))
        .output()
        .expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

#[test]
fn a_run_with_no_only_performs_every_family() {
    let (stdout, _, code) = run("planted", &["check"]);
    for (name, _) in documentation::check::Only::NAMED {
        assert!(
            stdout.contains(name),
            "the default must name {name} as checked: {stdout}"
        );
    }
    // The citation walk is the one whose absence is silent: the project plants five citation
    // defects, so a default that skipped it would exit 0 here.
    assert!(stdout.contains("no rule says this"), "{stdout}");
    assert_eq!(
        code, 1,
        "a project with planted defects must fail: {stdout}"
    );
}

#[test]
fn a_family_gated_on_a_file_that_is_absent_is_reported_as_not_run() {
    // `minimal` carries no changelog, so `corpus` cannot run there. Before this was pinned
    // the run printed `checked: corpus` and a row of zeros, which reads as provenance
    // verified against a corpus nothing had read.
    let (stdout, _, code) = run("minimal", &["check", "--only", "corpus"]);
    assert!(stdout.contains("NOT RUN: corpus"), "{stdout}");
    assert!(
        !stdout.contains("archived release(s)"),
        "no count for a family that did not run: {stdout}"
    );
    assert_eq!(code, 0, "{stdout}");
}

#[test]
fn asking_for_one_family_does_not_read_another() {
    let (stdout, _, code) = run("planted", &["check", "--only", "slugs"]);
    assert!(stdout.contains("checked: slugs"), "{stdout}");
    assert!(stdout.contains("slugs:"), "{stdout}");
    assert!(!stdout.contains("paths:"), "{stdout}");
    assert!(!stdout.contains("no rule says this"), "{stdout}");
    assert_eq!(code, 1, "the project plants four slug defects: {stdout}");
}

#[test]
fn a_family_that_does_not_read_rule_text_resolves_no_release() {
    // The `pinned` project pins a release that is neither vendored nor archived, so
    // resolving it is a network fetch that fails. Nothing about slugs needs a release, and
    // before resolution was scoped to the families that read rule text, every run resolved
    // every pin — so this run failed, and reached the network, for another family's reason.
    //
    // Only this direction is asserted. The opposite one is a real fetch, and a test suite
    // that reaches the network is a test suite that fails when the network does.
    let (stdout, stderr, code) = run("pinned", &["check", "--only", "slugs"]);
    assert_eq!(code, 0, "stdout {stdout} stderr {stderr}");
    assert!(stdout.contains("checked: slugs"), "{stdout}");
    assert!(
        !stderr.contains("19990101") && !stderr.contains("curl"),
        "no pin was resolved: {stderr}"
    );
}

#[test]
fn an_unknown_family_names_what_is_accepted_and_checks_nothing() {
    let (stdout, stderr, code) = run("planted", &["check", "--only", "nosuch"]);
    assert!(stderr.contains("nosuch"), "{stderr}");
    for (name, _) in documentation::check::Only::NAMED {
        assert!(stderr.contains(name), "{stderr}");
    }
    assert!(stdout.is_empty(), "nothing was checked: {stdout}");
    assert_eq!(code, 2, "{stderr}");
}

#[test]
fn no_subcommand_is_not_success() {
    // A tool whose default is to exit 0 having done nothing is the silent false negative
    // every check here exists to prevent.
    let (_, stderr, code) = run("planted", &[]);
    assert_eq!(code, 2, "{stderr}");
}

/// The summary comes first, the findings under it, and the verdict is the last line.
///
/// The order is the contract, not a preference. A caller reading the tail of a run — `| tail`,
/// `| grep` for a count, a CI log truncated to its end — must reach the answer, and while the
/// findings came first the summary block printed on a failing run and read as success. That is
/// the failure this asserts against, so it asserts the positions rather than the presence.
#[test]
fn a_failing_run_ends_with_its_verdict_and_the_summary_precedes_the_findings() {
    let (stdout, _, code) = run("planted", &["check"]);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();

    let summary = lines
        .iter()
        .position(|l| l.starts_with("checked:"))
        .expect("the summary block names what ran");
    let first_finding = lines
        .iter()
        .position(|l| l.contains("no rule says this"))
        .expect("the planted project reports citation findings");
    assert!(
        summary < first_finding,
        "the summary precedes the findings: {stdout}"
    );

    let last = lines.last().expect("output is not empty");
    assert!(
        last.starts_with("FAILED: ") && last.ends_with(" findings above"),
        "the last line is the verdict: {last:?}"
    );
    assert_eq!(code, 1, "{stdout}");
}

/// The verdict counts what was printed, and the count is the exit code's own predicate.
///
/// A count tracked beside the findings could disagree with the list and with the exit code.
/// This reads both off the same run and compares them, so a second source of truth for
/// "did it fail" cannot be introduced without failing here.
#[test]
fn the_verdict_counts_the_findings_it_printed() {
    let (stdout, _, code) = run("planted", &["check"]);
    let last = stdout.lines().rfind(|l| !l.is_empty()).unwrap();
    let claimed: usize = last
        .trim_start_matches("FAILED: ")
        .trim_end_matches(" findings above")
        .trim_end_matches(" finding above")
        .parse()
        .unwrap_or_else(|_| panic!("the verdict names a count: {last:?}"));

    // A finding is two lines: the finding, then its indented `→` action. Counting the actions
    // counts the findings without parsing the finding lines themselves.
    let printed = stdout
        .lines()
        .filter(|l| l.trim_start().starts_with('→'))
        .count();
    assert_eq!(
        claimed, printed,
        "the verdict counts what was printed: {stdout}"
    );
    assert!(
        claimed > 0 && code == 1,
        "a nonzero count means failure: {stdout}"
    );
}

/// A clean run says so on its last line and exits zero.
#[test]
fn a_passing_run_ends_with_a_passed_verdict() {
    let (stdout, _, code) = run("pinned", &["check", "--only", "slugs"]);
    let last = stdout.lines().rfind(|l| !l.is_empty()).unwrap();
    assert_eq!(last, "PASSED: no findings", "{stdout}");
    assert_eq!(code, 0, "{stdout}");
}
