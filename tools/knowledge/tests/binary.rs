//! The binary itself, run as a caller runs it.
//!
//! Everything else here tests the library, so the argument handling, the default selection
//! and the gates in `check()` were reachable by no test at all. What that cost is the shape
//! this file exists to prevent: the default selection could be changed to any subset and the
//! whole suite still passed, so `check` could stop verifying every rule quote and exit 0.
//!
//! Each test runs against a mock project under `knowledge@tests/projects/`, which the manifest excludes
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

/// The same, with `PATH` replaced, which is how the missing-`git` case is stated.
fn run_with_path(dir: &Path, args: &[&str], path: &Path) -> (String, String, i32) {
    run_with_env(dir, args, &[("PATH", path)])
}

/// The same, with environment variables replaced.
fn run_with_env(dir: &Path, args: &[&str], env: &[(&str, &Path)]) -> (String, String, i32) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_knowledge"));
    command.args(args).current_dir(dir);
    for (name, value) in env {
        command.env(name, value);
    }
    let out = command.output().expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// A throwaway copy of a mock project, for the tests whose command writes.
///
/// Every other test here runs against `knowledge@tests/projects/` in place, which works only while a run
/// leaves the tree alone. `index` writes, so it gets a copy: writing into the fixture would
/// leave the repository dirty, and the next run would then be comparing against the previous
/// run's output rather than against the fixture.
///
/// **The copy is a git repository of its own.** The walk is `git ls-files` from the project
/// root, so a copy outside any worktree walks nothing and every test over it passes for the
/// wrong reason. `git init` and `git add -A` are what make the copied files live, and anything
/// a test wants staged — a `.gitignore` above all — is written by `seeded` BEFORE the add.
struct Sandbox {
    dir: PathBuf,
}

impl Sandbox {
    fn new(tag: &str, from: &str) -> Self {
        Self::seeded(tag, from, &[])
    }

    /// The same, with extra files written into the copy before it is staged.
    fn seeded(tag: &str, from: &str, files: &[(&str, &str)]) -> Self {
        let dir = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        copy_dir(&project(from), &dir);
        let sandbox = Sandbox { dir };
        for (rel, text) in files {
            sandbox.write(rel, text);
        }
        sandbox.git(&["init", "-q"]);
        // `add` needs no identity, so none is configured: a fixture that wrote one would be
        // recording a name in a temporary repository nobody reads.
        sandbox.stage();
        sandbox
    }

    /// A copy that is NOT a git repository, for the tests about a project outside a worktree.
    fn without_git(tag: &str, from: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        copy_dir(&project(from), &dir);
        Sandbox { dir }
    }

    /// Stage whatever the copy holds now.
    ///
    /// A test that deletes a fixture file has to call this: the walk reads git's listing, and
    /// a deleted file the index still holds is listed, read, and reported as unreadable.
    ///
    /// **The per-user ignore file is pinned away for this invocation and for no other.** `add`
    /// honours it, so a developer's global rule would otherwise decide which fixture files are
    /// tracked. Nothing is written to the copy's configuration: the tool's own pin is what a
    /// test of that pin has to be able to see.
    fn stage(&self) {
        self.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
    }

    /// One git command in the copy, which must succeed.
    fn git(&self, args: &[&str]) {
        let out = Command::new("git")
            .args(args)
            .current_dir(&self.dir)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?} in the sandbox: {}",
            String::from_utf8_lossy(&out.stderr)
        );
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
    let (stdout, _, code) = run("planted", &["check", "--only", "references"]);
    assert!(stdout.contains("checked: references"), "{stdout}");
    assert!(stdout.contains("references:"), "{stdout}");
    assert!(!stdout.contains("components:"), "{stdout}");
    assert!(!stdout.contains("no rule says this"), "{stdout}");
    assert_eq!(code, 1, "the project plants reference defects: {stdout}");
}

#[test]
fn a_family_that_does_not_read_rule_text_resolves_no_release() {
    // The `pinned` project pins a release that is neither vendored nor archived, so
    // resolving it is a network fetch that fails. Nothing about references needs a release, and
    // before resolution was scoped to the families that read rule text, every run resolved
    // every pin — so this run failed, and reached the network, for another family's reason.
    //
    // Only this direction is asserted. The opposite one is a real fetch, and a test suite
    // that reaches the network is a test suite that fails when the network does.
    let (stdout, stderr, code) = run("pinned", &["check", "--only", "references"]);
    assert_eq!(code, 0, "stdout {stdout} stderr {stderr}");
    assert!(stdout.contains("checked: references"), "{stdout}");
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
    let (stdout, _, code) = run("pinned", &["check", "--only", "references"]);
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
    const OLD: &str = "20200101";
    const NEW: &str = "20200102";
    let invalid: [&[&str]; 6] = [
        &["issues", "--issues"],
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
    for verb in [
        "check",
        "show",
        "issues",
        "tripwires",
        "index",
        "model",
        "rules",
    ] {
        assert!(
            stdout.contains(verb),
            "{verb} is missing from the help: {stdout}"
        );
    }
}

/// The claim: `index` regenerates every generated file from one invocation, writes only where
/// the bytes differ, and says which of them it moved.
///
/// **Two assertions, because they fall to different mutations.** The report is satisfied by an
/// implementation that compares and then writes anyway; the mtime is what catches that one.
/// Recorded mutation, run through `cargo mutate run`: making the write unconditional — the
/// `continue` on equality deleted — leaves the report right and moves both mtimes.
#[test]
fn index_rewrites_what_moved_and_leaves_what_is_current_alone() {
    let sandbox = Sandbox::new("index", "minimal");
    // The rule index, which the mock does not carry, and one per file-register instance,
    // which it does. One invocation writes every one of them.
    let generated = [
        "corpus/index.md",
        "docs/open-issues/index.md",
        "notes/open-issues/index.md",
        "notes/readings/index.md",
    ];

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
        1,
        "only the rule index did not exist, so only it was written: {first}"
    );
    assert_eq!(
        first.matches("already current").count(),
        generated.len() - 1,
        "the committed file-register indexes are what the generator produces: {first}"
    );

    let before: Vec<SystemTime> = generated
        .iter()
        .map(|r| modified(&sandbox.path(r)))
        .collect();

    let (second, _, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0);
    assert_eq!(
        second.matches("already current").count(),
        generated.len(),
        "nothing moved, so nothing was written: {second}"
    );
    for (rel, was) in generated.iter().zip(&before) {
        assert_eq!(
            modified(&sandbox.path(rel)),
            *was,
            "{rel} was rewritten with the bytes it already held"
        );
    }

    // The other half: the skip is a comparison, not a refusal to write a file twice. Both
    // kinds of index are staled, so neither is held current by the other.
    for rel in [generated[0], generated[1]] {
        std::fs::write(sandbox.path(rel), "stale\n").expect("a stale index");
    }
    let (third, _, _) = sandbox.run(&["index"]);
    assert_eq!(third.matches("rewritten").count(), 2, "{third}");
    assert_eq!(
        third.matches("already current").count(),
        generated.len() - 2,
        "{third}"
    );
    for rel in [generated[0], generated[1]] {
        assert_ne!(
            std::fs::read_to_string(sandbox.path(rel)).expect("the index"),
            "stale\n",
            "{rel} moved and was regenerated"
        );
    }
}

/// The claim: both refusals run over EVERY destination before any is written, so a run either
/// refuses having written nothing or writes them all.
///
/// On `main` there was one destination and the ordering was vacuous. With one index per
/// file-register instance it is not: reordering the two loops in `knowledge@src/main.rs` would
/// leave a run that wrote three indexes and then exited 2 — could not run — over a tree it had
/// already changed.
#[test]
fn a_refused_destination_stops_the_run_before_anything_is_written() {
    let sandbox = Sandbox::new("index-refusal", "minimal");
    // The three that would be written: two staled file-register indexes, and the rule index,
    // which the mock does not carry at all.
    let staled = ["docs/open-issues/index.md", "notes/open-issues/index.md"];
    for rel in staled {
        sandbox.write(rel, "stale\n");
    }
    // The fourth is a symlink, which `fs::write` would follow, replacing content this command
    // never produced. It sorts last of the four, so a run refusing only at its own turn would
    // already have written the other three.
    let target = sandbox.path("notes/readings/elsewhere.md");
    std::fs::write(&target, "not generated by anything\n").expect("the symlink's target");
    let link = sandbox.path("notes/readings/index.md");
    std::fs::remove_file(&link).expect("the committed index");
    std::os::unix::fs::symlink("elsewhere.md", &link).expect("a symlinked index");

    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 2, "could not run: {out}{err}");
    assert!(err.contains("Nothing was written"), "{err}");
    for rel in staled {
        assert_eq!(
            std::fs::read_to_string(sandbox.path(rel)).expect("the staled index"),
            "stale\n",
            "{rel} was written before the refusal"
        );
    }
    assert!(
        !sandbox.path("corpus/index.md").exists(),
        "the rule index was created before the refusal"
    );
    assert_eq!(
        std::fs::read_to_string(&target).expect("the symlink's target"),
        "not generated by anything\n",
        "the write followed the symlink"
    );

    // The other refusal, over a destination whose directory is gone rather than a symlink. It
    // cannot be a file-register instance, which contributes no destination when its directory is
    // absent, so it is the rule index's declared directory.
    std::fs::remove_file(&link).expect("the symlink");
    std::fs::remove_dir_all(sandbox.path("corpus")).expect("the rules directory");
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 2, "{out}{err}");
    for rel in staled {
        assert_eq!(
            std::fs::read_to_string(sandbox.path(rel)).expect("the staled index"),
            "stale\n",
            "{rel} was written before the refusal"
        );
    }
}

/// Two issues and one tripwire, so the two listings differ and a command returning the wrong
/// register cannot pass by symmetry. An issue is one file of the issue register, labelled by its
/// own frontmatter; a tripwire is a slugged heading in the tripwire home.
const FIRST_ISSUE: &str = "\
---
kind: defect
---
# The first thing outstanding

## Summary

A body, so the entry is an entry.
";

const SECOND_ISSUE: &str = "\
---
kind: todo
---
# The second thing outstanding

## Summary

Another body, and a reference nothing defines: `design@minimal@no-such-decision`.
";

/// PLANTED: one entry declaring `kind` twice, so the block is refused and the row has no kind.
const TWO_KINDS: &str = "\
---
kind: defect
kind: todo
---
# The entry that declares two kinds

## Summary

Two values for one thing, and whichever reader looks first decides.
";

const ONE_TRIPWIRE: &str = "\
# Tripwires

## Guarding the fixture's own decision `##a-fixture-tripwire`

**Fires when:** a condition the fixture names is met, guarding `design@minimal@mock-anchor`.
**Response:** reopen it.
";

/// The claim: `issues` prints one row per issue entry with the five columns in order, and each
/// filter keeps only what it names.
///
/// Recorded mutation, `cargo mutate run` over `knowledge@src/main.rs`: turning the `--kind`
/// filter's `is_none_or` into `is_some_and` empties every unfiltered listing, and turning the
/// row's `metadata` cell into a constant makes every kind read alike. Both are caught here.
#[test]
fn issues_lists_one_row_per_entry_and_each_filter_keeps_what_it_names() {
    let sandbox = Sandbox::new("issues", "minimal");
    // The fixture's own entries are replaced, so the rows below are this test's.
    std::fs::remove_file(sandbox.path("docs/open-issues/the-mock-has-one-issue.md"))
        .expect("the fixture's own entry");
    std::fs::remove_file(sandbox.path("notes/open-issues/the-notes-are-not-a-component.md"))
        .expect("the fixture's own entry");
    sandbox.write("docs/open-issues/the-first-thing.md", FIRST_ISSUE);
    sandbox.write("docs/open-issues/the-second-thing.md", SECOND_ISSUE);
    sandbox.write("notes/open-issues/two-kinds.md", TWO_KINDS);
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);
    sandbox.stage();

    let (all, stderr, code) = sandbox.run(&["issues"]);
    assert_eq!(code, 0, "{stderr}");
    let rows: Vec<&str> = all.lines().collect();
    assert_eq!(
        rows[0].split_whitespace().collect::<Vec<_>>(),
        vec!["kind", "anchor", "id", "title", "last", "change"],
        "{all}"
    );
    // Sorted by kind then id, so `-` (the refused block) comes before `defect` before `todo`.
    assert_eq!(rows.len(), 4, "{all}");
    assert!(rows[1].starts_with("-  "), "{all}");
    assert!(rows[1].contains("two-kinds"), "{all}");
    assert!(rows[2].starts_with("defect"), "{all}");
    assert!(rows[3].starts_with("todo"), "{all}");
    // The anchor column tells the two instances apart.
    assert!(rows[1].contains("notes"), "{all}");
    assert!(rows[2].contains("minimal"), "{all}");
    // No tripwire is an issue.
    assert!(!all.contains("Guarding the fixture"), "{all}");

    let (one_kind, _, code) = sandbox.run(&["issues", "--kind", "todo"]);
    assert_eq!(code, 0, "{one_kind}");
    assert_eq!(one_kind.lines().count(), 2, "{one_kind}");
    assert!(one_kind.contains("the-second-thing"), "{one_kind}");

    let (one_anchor, _, code) = sandbox.run(&["issues", "notes"]);
    assert_eq!(code, 0, "{one_anchor}");
    assert_eq!(one_anchor.lines().count(), 2, "{one_anchor}");
    assert!(one_anchor.contains("two-kinds"), "{one_anchor}");

    let (searched, _, code) = sandbox.run(&["issues", "second"]);
    assert_eq!(code, 0, "{searched}");
    assert_eq!(searched.lines().count(), 2, "{searched}");
    assert!(searched.contains("the-second-thing"), "{searched}");

    // No row is a negative answer, and the header still says what was looked for.
    let (none, _, code) = sandbox.run(&["issues", "nothing-matches-this"]);
    assert_eq!(code, 1, "{none}");
    assert!(
        none.contains("kind") && none.contains("(no entry)"),
        "{none}"
    );
}

/// The claim: `tripwires` prints the references each entry carries, `--guarding` keeps the rows
/// carrying one, and no issue appears among them.
#[test]
fn tripwires_lists_the_decisions_each_entry_guards() {
    let sandbox = Sandbox::new("tripwires", "minimal");
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);

    let (all, stderr, code) = sandbox.run(&["tripwires"]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(
        all.lines()
            .next()
            .map(|l| l.split_whitespace().collect::<Vec<_>>()),
        Some(vec!["anchor", "id", "title", "guarding"]),
        "{all}"
    );
    let row = all.lines().nth(1).expect("one tripwire");
    // The title loses the slug that defines the entry, and the guarded decision is a column.
    assert!(row.contains("a-fixture-tripwire"), "{row}");
    assert!(row.contains("Guarding the fixture's own decision"), "{row}");
    assert!(!row.contains("##a-fixture-tripwire"), "{row}");
    assert!(
        row.trim_end().ends_with("design@minimal@mock-anchor"),
        "{row}"
    );
    assert!(!all.contains("The mock has one issue"), "{all}");

    let (kept, _, code) = sandbox.run(&["tripwires", "--guarding", "design@minimal@mock-anchor"]);
    assert_eq!(code, 0, "{kept}");
    assert_eq!(kept.lines().count(), 2, "{kept}");

    let (dropped, _, code) =
        sandbox.run(&["tripwires", "--guarding", "design@minimal@mock-anchor-2"]);
    assert_eq!(code, 1, "{dropped}");
    assert!(dropped.contains("(no entry)"), "{dropped}");
}

/// The claim: `show` prints a file entry whole and a heading entry's section, lists every inbound
/// reference as `file:line`, exits 1 on a reference that resolves to nothing and 2 on an argument
/// that is not reference-shaped.
#[test]
fn show_prints_the_entry_and_what_points_at_it() {
    let sandbox = Sandbox::new("show", "minimal");
    sandbox.write("docs/open-issues/the-second-thing.md", SECOND_ISSUE);
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);

    // A file entry: the file whole, frontmatter included.
    let (entry, stderr, code) = sandbox.run(&["show", "issue@minimal@the-second-thing"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(entry.contains("kind: todo"), "{entry}");
    assert!(entry.contains("## Summary"), "{entry}");
    assert!(
        entry.contains("docs/open-issues/the-second-thing.md:1"),
        "{entry}"
    );

    // A heading entry: that heading's section and not the whole file.
    let (heading, _, code) = sandbox.run(&["show", "tripwire@minimal@a-fixture-tripwire"]);
    assert_eq!(code, 0, "{heading}");
    assert!(heading.contains("**Fires when:**"), "{heading}");
    assert!(
        !heading.contains("# Tripwires\n"),
        "the level-one head is another section: {heading}"
    );

    // The inbound half: the entry above names a decision, and that decision's `show` finds it.
    let (inbound, _, code) = sandbox.run(&["show", "design@minimal@mock-anchor"]);
    assert_eq!(code, 0, "{inbound}");
    assert!(inbound.contains("referenced at:"), "{inbound}");
    assert!(inbound.contains("docs/tripwires.md:"), "{inbound}");

    // An entry nothing points at says so, rather than printing an empty list.
    let (alone, _, code) = sandbox.run(&["show", "issue@minimal@the-second-thing"]);
    assert_eq!(code, 0, "{alone}");
    assert!(alone.contains("referenced by nothing"), "{alone}");

    // The two failures are different questions and different codes.
    let (gone, _, code) = sandbox.run(&["show", "issue@minimal@no-such-entry"]);
    assert_eq!(code, 1, "{gone}");
    assert!(gone.contains("resolves to nothing"), "{gone}");
    for shape in ["not-a-reference", "design@minimal", "design@minimal@a@b"] {
        let (_, err, code) = sandbox.run(&["show", shape]);
        assert_eq!(code, 2, "{shape}: {err}");
    }
}

/// The summary block names the checker's own directory and counts the files under it, so a
/// binary whose compiled path misses the tree is visible in every run.
#[test]
fn the_summary_names_the_checker_source_and_counts_the_files_under_it() {
    // A mock project holds no file under the checker's source: the line prints, at zero.
    let (stdout, _, _) = run("minimal", &["check", "--only", "references"]);
    assert!(
        stdout.contains("\nchecker source: ")
            && stdout.contains(", 0 file(s) with string literals read as data"),
        "{stdout}"
    );
    // This checkout does, and the binary under test was built from it.
    let checkout = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("tools/knowledge sits two levels below the root");
    let (stdout, _, _) = run_in(checkout, &["check", "--only", "references"]);
    let line = stdout
        .lines()
        .find(|l| l.starts_with("checker source: "))
        .unwrap_or_else(|| panic!("no checker-source line in {stdout}"));
    assert!(
        line.starts_with("checker source: tools/knowledge, "),
        "{line}"
    );
    let count: usize = line
        .trim_start_matches("checker source: tools/knowledge, ")
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("no count in {line}"));
    assert!(count > 0, "{line}");
}

// --- git as the walk ---------------------------------------------------------------------

/// The `walk: n file(s)` count out of a summary block.
fn walked_count(stdout: &str) -> usize {
    let line = stdout
        .lines()
        .find(|l| l.starts_with("walk: "))
        .unwrap_or_else(|| panic!("the summary block names the walked-file count: {stdout}"));
    line.trim_start_matches("walk: ")
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("a number: {line}"))
}

/// A reference to a decision no home defines, for a document to carry.
const DANGLING: &str = "# Another mock document\n\nIt points at `design@minimal@no-such-thing`.\n";

/// The claim: a tracked document a `.gitignore` line covers is still walked, and the pair is a
/// finding naming the file.
///
/// The recipe is the one the tracker entry this closed carried, and the entry is gone with the
/// defect. Under the hand-rolled matcher the second run printed nothing:
/// the ignore line pruned the document from the walk AND from the inverse assertion, so the
/// dangling reference in it was read by no check and the run exited 0. Git's tracked listing is
/// unaffected by the ignore rules, so the reference is still found, and the contradiction
/// between the index and the ignore rules is reported instead of being silent.
#[test]
fn a_tracked_document_an_ignore_line_covers_is_walked_and_reported() {
    let sandbox = Sandbox::new("tracked-ignored", "minimal");
    sandbox.write("notes/b.md", DANGLING);
    sandbox.stage();

    let (before, stderr, code) = sandbox.run(&["check", "--only", "references"]);
    assert_eq!(
        code, 1,
        "the dangling reference is a finding: {before}{stderr}"
    );
    assert!(before.contains("no-such-thing"), "{before}");

    // The recipe: one root gitignore line holding the document's bare filename. The document
    // is already tracked, so `git add -A` leaves it tracked.
    sandbox.write(".gitignore", "b.md\n");
    sandbox.stage();
    let (after, stderr, code) = sandbox.run(&["check", "--only", "references,registers"]);
    assert_eq!(code, 1, "{after}{stderr}");
    assert!(
        after.contains("no-such-thing"),
        "the document must still be walked: {after}"
    );
    assert!(
        after.contains("notes/b.md") && after.contains("git tracks this file"),
        "the tracked-and-ignored pair must be named: {after}"
    );
    // The summary says how much was walked, on a failing run as readily as a passing one.
    assert!(walked_count(&after) > 0, "{after}");
}

/// The claim: a `.gitignore` below the root decides the walk too.
///
/// The matcher this replaced read the root file alone, so a document under a nested ignore was
/// walked and its planted defect reported. Both halves are asserted, because a test that only
/// showed the silence would pass over a walk that had stopped reading anything at all.
#[test]
fn a_nested_gitignore_is_honoured() {
    let ignored = Sandbox::seeded(
        "nested-ignored",
        "minimal",
        &[
            ("notes/.gitignore", "scratch.md\n"),
            ("notes/scratch.md", DANGLING),
        ],
    );
    let (out, stderr, code) = ignored.run(&["check", "--only", "references"]);
    assert!(
        !out.contains("no-such-thing"),
        "the nested ignore rule covers it: {out}{stderr}"
    );
    assert_eq!(code, 0, "{out}{stderr}");

    let walked = Sandbox::seeded(
        "nested-walked",
        "minimal",
        &[("notes/scratch.md", DANGLING)],
    );
    let (walked_out, stderr, code) = walked.run(&["check", "--only", "references"]);
    assert_eq!(
        code, 1,
        "without the rule the file is live: {walked_out}{stderr}"
    );
    assert!(walked_out.contains("no-such-thing"), "{walked_out}");
    // Exactly one file separates the two copies, and the count says so. A count that did not
    // follow the walk would read alike here.
    assert_eq!(walked_count(&walked_out), walked_count(&out) + 1);
}

/// The claim: no `git` on the path is exit 2 naming git, never an empty walk.
///
/// An empty walk is the dangerous answer: a project with no document is reported as one with
/// nothing wrong, which is the failure class this tool exists to prevent.
#[test]
fn a_run_with_no_git_on_the_path_is_exit_two_naming_git() {
    let empty = std::env::temp_dir().join(format!("knowledge-no-git-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&empty);
    std::fs::create_dir_all(&empty).expect("a directory holding no git");
    let (stdout, stderr, code) = run_with_path(&project("minimal"), &["check"], &empty);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("git is not on the PATH"), "{stderr}");
    assert!(!stdout.contains("PASSED"), "{stdout}");
    let _ = std::fs::remove_dir_all(&empty);
}

/// The claim: a project directory outside any git worktree is exit 2 carrying git's own reason.
#[test]
fn a_project_outside_a_worktree_is_exit_two_carrying_gits_reason() {
    let sandbox = Sandbox::without_git("no-worktree", "minimal");
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(
        stderr.to_lowercase().contains("not a git repository"),
        "{stderr}"
    );
    assert!(!stdout.contains("PASSED"), "{stdout}");
}

/// The claim: a path reference whose target the ignore rules cover is exempt from the existence
/// assertion, and the exemption is decided by the rules rather than by what is on disk.
///
/// The target below is on no disk at all: it is a generated path, which is the case the
/// exemption exists for. A run that asked the filesystem, or that asked git nothing, reports it
/// as dangling — so the second half of the test is the same tree without the ignore line.
#[test]
fn a_reference_to_an_ignored_target_is_exempt_and_the_rules_decide_it() {
    let pointer = "# A mock document\n\nIt points at `path@minimal@build-output/out.txt` and at \
                   `path@minimal@build-output/`.\n";
    let exempt = Sandbox::seeded(
        "ignored-target",
        "minimal",
        &[(".gitignore", "build-output/\n"), ("notes/a.md", pointer)],
    );
    let (out, stderr, code) = exempt.run(&["check", "--only", "references"]);
    assert_eq!(code, 0, "the ignore rules cover the target: {out}{stderr}");
    assert!(!out.contains("build-output"), "{out}");

    let asserted = Sandbox::seeded("unignored-target", "minimal", &[("notes/a.md", pointer)]);
    let (out, stderr, code) = asserted.run(&["check", "--only", "references"]);
    assert_eq!(
        code, 1,
        "without the rule the target is asserted: {out}{stderr}"
    );
    assert_eq!(
        out.lines().filter(|l| l.contains("does not exist")).count(),
        2,
        "both the file claim and the directory claim: {out}"
    );
}

/// The claim: a tracked file the working tree does not hold is reported, not dropped.
///
/// An unstaged deletion leaves a path in git's listing with no bytes behind it. Dropping it
/// would take a live document out of every check on the strength of a working-tree state, which
/// is the shape `knowledge#a-failed-parse-is-loud` refuses. The finding names the deletion
/// rather than reporting an encoding failure, because the two need different repairs.
#[test]
fn a_tracked_file_the_working_tree_does_not_hold_is_reported() {
    let sandbox = Sandbox::new("deleted-tracked", "minimal");
    let (before, stderr, code) = sandbox.run(&["check", "--only", "citations"]);
    assert_eq!(
        code, 0,
        "the fixture starts clean under this family: {before}{stderr}"
    );

    std::fs::remove_file(sandbox.path("notes/b.md")).expect("a staged fixture file");
    let (out, stderr, code) = sandbox.run(&["check", "--only", "citations"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.contains("notes/b.md") && out.contains("the working tree does not hold it"),
        "{out}"
    );
    assert_eq!(
        walked_count(&out),
        walked_count(&before),
        "the path stays in the walk while the index holds it"
    );

    // Staging the deletion is the repair, and it takes the path out of the listing.
    sandbox.stage();
    let (after, stderr, code) = sandbox.run(&["check", "--only", "citations"]);
    assert_eq!(code, 0, "{after}{stderr}");
    assert!(!after.contains("notes/b.md"), "{after}");
    assert_eq!(walked_count(&after), walked_count(&before) - 1);
}

/// The claim: the developer's own global ignore file does not decide what is walked.
///
/// `core.excludesFile` lives in the home directory and is no part of any project, so a line in
/// it would take an untracked live document out of every check on one clone and not on another.
/// The run is pinned against it, and this states the pin by giving the run a home directory
/// whose git configuration ignores one of the fixture's files.
#[test]
fn a_per_user_ignore_file_does_not_decide_the_walk() {
    // The document is left UNTRACKED on purpose: the ignore rules act on untracked files
    // alone, so a staged one would be listed whatever any ignore file said and the test would
    // pass over the pin without touching it.
    let sandbox = Sandbox::new("global-ignore", "minimal");
    sandbox.write("notes/scratch.md", DANGLING);
    let home = std::env::temp_dir().join(format!("knowledge-home-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(home.join(".config/git")).expect("a fake home");
    std::fs::write(home.join(".config/git/ignore"), "scratch.md\n").expect("a global rule");
    std::fs::write(
        home.join(".gitconfig"),
        "[core]\n\texcludesFile = ~/.config/git/ignore\n",
    )
    .expect("a global configuration");

    let config = home.join(".config");
    let env: Vec<(&str, &Path)> = vec![("HOME", home.as_path()), ("XDG_CONFIG_HOME", &config)];
    let (out, stderr, code) = run_with_env(&sandbox.dir, &["check", "--only", "references"], &env);
    assert_eq!(code, 1, "the document stays live: {out}{stderr}");
    assert!(out.contains("no-such-thing"), "{out}");

    // The same run without the fake home, so the count is the same either way.
    let (plain, _, _) = sandbox.run(&["check", "--only", "references"]);
    assert_eq!(walked_count(&out), walked_count(&plain));
    let _ = std::fs::remove_dir_all(&home);
}
