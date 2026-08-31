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

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

fn project(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

/// Stdout, stderr and the exit code of one run in a directory.
fn run_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_knowledge"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// Stdout, stderr and the exit code of one run in one mock project.
fn run(name: &str, args: &[&str]) -> (String, String, i32) {
    run_in(&project(name), args)
}

/// A throwaway copy of a mock project, for the tests whose command writes.
///
/// Every other test here runs against `tests/projects/` in place, which works only while a run
/// leaves the tree alone. `index` writes, so it gets a copy: writing into the fixture would
/// leave the repository dirty, and the next run would then be comparing against the previous
/// run's output rather than against the fixture.
struct Sandbox {
    dir: PathBuf,
}

impl Sandbox {
    fn new(tag: &str, from: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        copy_dir(&project(from), &dir);
        Sandbox { dir }
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.dir.join(rel)
    }

    fn write(&self, rel: &str, text: &str) {
        let path = self.path(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
        std::fs::write(&path, text).expect("a written fixture file");
    }

    fn run(&self, args: &[&str]) -> (String, String, i32) {
        run_in(&self.dir, args)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the sandbox directory");
    for entry in std::fs::read_dir(from).expect("a readable fixture") {
        let entry = entry.expect("a directory entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a file type").is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copied file");
        }
    }
}

fn modified(path: &Path) -> SystemTime {
    std::fs::metadata(path)
        .expect("a generated file")
        .modified()
        .expect("an mtime")
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

/// `rules show` prints a rule in the shape a citation is written in.
///
/// The migration this exists for pastes the printed line into a document as a blockquote, so
/// what is asserted is the whole line: the marker, the number as printed, and the body entire.
/// A run that dropped the number, wrapped the body or elided any of it would produce a quote
/// the checker rejects, and the whole point is that it does not.
#[test]
fn rules_show_prints_a_rule_as_a_pasteable_blockquote() {
    let (stdout, _, code) = run("planted", &["rules", "show", "100.3"]);
    assert!(
        stdout.lines().any(|l| l
            == "> 100.3 A rule whose body is long enough to be cut at either end without the \
                cut showing, so that a quote of its middle is evidence of nothing unless the \
                omission is marked."),
        "{stdout}"
    );
    assert_eq!(code, 0, "{stdout}");
}

/// A rule with subrules says which, and a rule without says nothing about any.
///
/// A whole-body quote of a parent does not stand for a claim its subrule carries, so the one
/// thing a reader must not do with this output is quote a parent for a subrule's claim
/// without being told the subrules exist.
#[test]
fn rules_show_names_the_subrules_a_parent_has() {
    let (parent, _, _) = run("planted", &["rules", "show", "100.1"]);
    assert!(parent.contains("100.1a"), "{parent}");
    let leaf = run("planted", &["rules", "show", "100.2"]).0;
    assert!(!leaf.contains("subrule"), "{leaf}");
}

/// A number the release has no rule for is named, and the run fails.
///
/// Silence here is the failure this tool exists to prevent: a session that asked for a rule
/// and got nothing back would write the citation from recollection.
#[test]
fn rules_show_fails_on_a_number_the_release_does_not_hold() {
    let (stdout, _, code) = run("planted", &["rules", "show", "100.9"]);
    assert!(stdout.contains("100.9"), "{stdout}");
    assert_eq!(code, 1, "{stdout}");
}

/// Several numbers in one run, because an enumeration cites several and each owes its quote.
#[test]
fn rules_show_takes_several_numbers_at_once() {
    let (stdout, _, code) = run("planted", &["rules", "show", "100.1", "100.2", "100.4"]);
    for number in ["100.1", "100.2", "100.4"] {
        assert!(
            stdout
                .lines()
                .any(|l| l.starts_with(&format!("> {number} "))),
            "{number} is missing: {stdout}"
        );
    }
    assert_eq!(code, 0, "{stdout}");
}

/// What `rules show` prints carries the release's own typography.
///
/// The corpus is normalised for comparison — `norm` folds a curly apostrophe to a straight one
/// so that a quote written either way verifies — and a caller PASTES this output. Printing the
/// folded form writes a quote that differs from the pinned text at every apostrophe, which
/// verifies and is not what the rule says.
#[test]
fn rules_show_prints_the_release_typography_and_not_the_folded_form() {
    let (stdout, _, code) = run("typography", &["rules", "show", "100.1"]);
    assert!(
        stdout.contains('\u{2019}'),
        "the curly apostrophe must survive: {stdout:?}"
    );
    assert!(
        !stdout.contains('\''),
        "and no straight one appears: {stdout:?}"
    );
    assert_eq!(code, 0, "{stdout}");
}

/// The claim: an invocation the hand-rolled parser accepted now exits 2 and runs nothing.
///
/// The parse-level half of this lives beside the declaration in `knowledge@src/main.rs`. This
/// is the same claim at the process boundary, which is what a gate and a session actually
/// read: a `try_parse_from` returning `Err` says nothing about the code the process leaves
/// with, and 2 is `thaum#exit-code-ladder`'s could-not-run.
///
/// Every row exited **0** against the implementation this replaces, each having done something
/// other than what was asked.
#[test]
fn an_invalid_invocation_exits_two_and_runs_nothing() {
    // Bound, so the dates are fixture data rather than prose.
    const OLD: &str = "20200101";
    const NEW: &str = "20200102";
    let invalid: [&[&str]; 6] = [
        &["outstanding", "--issues", "--tripwires"],
        &["check", "--bogus"],
        &["check", "stray"],
        &["model", "zzz"],
        &["index", "--write"],
        &["rules", "diff", OLD, NEW],
    ];
    for args in invalid {
        let (stdout, stderr, code) = run("planted", args);
        assert_eq!(code, 2, "{args:?} must be refused; stderr: {stderr}");
        assert!(stdout.is_empty(), "{args:?} ran something: {stdout}");
    }
}

/// The claim: help answers from outside a project.
///
/// Arguments parse before the project is located, which is the whole of it. Before the
/// migration `Manifest::find` ran first, so every invocation from outside a checkout — help
/// included — failed with `error:` and exit 2 and never reached its subcommand.
#[test]
fn help_answers_from_outside_a_project() {
    let outside = std::env::temp_dir();
    let (stdout, stderr, code) = run_in(&outside, &["--help"]);
    assert_eq!(code, 0, "{stderr}");
    for verb in ["check", "outstanding", "index", "model", "rules"] {
        assert!(
            stdout.contains(verb),
            "{verb} is missing from the help: {stdout}"
        );
    }
}

/// The concern file the interpretation index needs a row for. No rule number in it: this file
/// is inside the walk, and a number in an unbound literal is prose the scanner reads.
const CONCERN: &str = "\
# One concern

## R1 — A reading, recorded so that the generated index has a row to carry

The body of the reading.
";

/// The claim: `index` regenerates both files from one invocation, writes only where the bytes
/// differ, and says which of the two it moved.
///
/// **Two assertions, because they fall to different mutations.** The report is satisfied by an
/// implementation that compares and then writes anyway; the mtime is what catches that one.
/// Recorded mutation, run through `cargo mutate run`: making the write unconditional — the
/// `continue` on equality deleted — leaves the report right and moves both mtimes.
#[test]
fn index_rewrites_what_moved_and_leaves_what_is_current_alone() {
    let sandbox = Sandbox::new("index", "minimal");
    // `minimal` declares this concern directory and does not carry it, no other test needing
    // one. The interpretation index is written into it, so it has to exist; creating it is the
    // manifest's claim to make good, not something `index` should do on the manifest's behalf.
    sandbox.write("notes/readings/one-concern.md", CONCERN);
    let generated = ["corpus/index.md", "notes/readings/index.md"];

    let (first, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stderr}");
    for rel in generated {
        assert!(
            first.contains(rel),
            "{rel} is missing from the report: {first}"
        );
    }
    assert_eq!(
        first.matches("rewritten").count(),
        2,
        "neither file existed, so both were written: {first}"
    );

    let before: Vec<SystemTime> = generated
        .iter()
        .map(|r| modified(&sandbox.path(r)))
        .collect();

    let (second, _, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0);
    assert_eq!(
        second.matches("already current").count(),
        2,
        "nothing moved, so nothing was written: {second}"
    );
    for (rel, was) in generated.iter().zip(&before) {
        assert_eq!(
            modified(&sandbox.path(rel)),
            *was,
            "{rel} was rewritten with the bytes it already held"
        );
    }

    // The other half: the skip is a comparison, not a refusal to write a file twice.
    std::fs::write(sandbox.path(generated[0]), "stale\n").expect("a stale index");
    let (third, _, _) = sandbox.run(&["index"]);
    assert_eq!(third.matches("rewritten").count(), 1, "{third}");
    assert_eq!(third.matches("already current").count(), 1, "{third}");
    assert_ne!(
        std::fs::read_to_string(sandbox.path(generated[0])).expect("the index"),
        "stale\n",
        "the file that moved was regenerated"
    );
}

/// Two issues and one tripwire, so the counts differ and a selection returning the wrong kind
/// cannot pass by symmetry. An issue is recognised by the kind tag on its title, a tripwire by
/// stating when it fires; the file it sits in is what decides which it is.
const TWO_ISSUES: &str = "\
# Open issues

## The first thing outstanding `defect`

**What.** A body, so the entry is an entry.

## The second thing outstanding `todo`

**What.** Another body.
";

const ONE_TRIPWIRE: &str = "\
# Tripwires

## Guarding something the fixture decided

**Fires when:** a condition the fixture names is met.
**Response:** reopen it.
";

/// The claim: each flag selects its own kind, neither selects the other's, and a search honours
/// the flag beside it.
///
/// This is the command root `CLAUDE.md` sends every session to before diagnosing anything, and
/// nothing drove it before. Recorded mutation, `cargo mutate run` over
/// `knowledge@src/main.rs`: turning the both-flags-absent case from `(true, true)` into
/// `(false, true)` makes a bare run report zero open issues over a tree that holds two — the
/// exact symptom the closed tracker entry recorded — and it is caught here.
#[test]
fn outstanding_selects_the_kind_its_flags_name() {
    let sandbox = Sandbox::new("outstanding", "minimal");
    sandbox.write("docs/open-issues.md", TWO_ISSUES);
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);
    // The fixture carries an entry of its own in its third registered tracker. Emptied, so the
    // totals below are this test's and not the fixture's, and stay so if the fixture changes.
    sandbox.write(
        "notes/open-issues.md",
        "# Open issues\n\nNothing outstanding here.\n",
    );

    let (all, stderr, code) = sandbox.run(&["outstanding"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(
        all.contains("2 open issue(s), 1 tripwire(s)"),
        "neither flag is every kind: {all}"
    );

    let (issues, _, _) = sandbox.run(&["outstanding", "--issues"]);
    assert!(
        issues.contains("2 open issue(s), 0 tripwire(s)"),
        "--issues selects issues and not tripwires: {issues}"
    );
    assert!(!issues.contains("Guarding something"), "{issues}");

    let (tripwires, _, _) = sandbox.run(&["outstanding", "--tripwires"]);
    assert!(
        tripwires.contains("0 open issue(s), 1 tripwire(s)"),
        "--tripwires selects tripwires and not issues: {tripwires}"
    );
    assert!(
        !tripwires.contains("The first thing outstanding"),
        "{tripwires}"
    );

    // The flags bind the search too. Computed and then not read, `--issues` printed a tripwire
    // in full at exit 0.
    let (crossed, _, code) = sandbox.run(&["outstanding", "--issues", "Guarding something"]);
    assert_eq!(
        code, 1,
        "a search restricted to the other kind matches nothing: {crossed}"
    );
    let (found, _, code) = sandbox.run(&["outstanding", "--tripwires", "Guarding something"]);
    assert_eq!(code, 0, "{found}");
    assert!(found.contains("Fires when"), "the entry in full: {found}");
}
