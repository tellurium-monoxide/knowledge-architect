//! The core's own binary, run as a caller runs it, with no extension registered, per
//! `design@core@the-core-cli-is-a-library-module`.
//!
//! Each test runs against a mock project under `path@core@tests/projects/`, which this
//! repository's manifest excludes from its own walk, so the binary finds that project by walking
//! up from the working directory exactly as it would find any other.

use std::path::{Path, PathBuf};
use std::process::Command;

fn project(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

/// Stdout, stderr and the exit code of one run in a directory.
fn run_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
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
    let mut command = Command::new(env!("CARGO_BIN_EXE_klarch"));
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
/// Every other test here runs against `path@core@tests/projects/` in place, which works only while a run
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

    /// Write a file into the copy; a manifest is written pinned to this binary's version.
    fn write(&self, rel: &str, text: &str) {
        let path = self.path(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
        let text = if rel == "knowledge-architect.toml" {
            pinned(text)
        } else {
            text.to_string()
        };
        std::fs::write(&path, text).expect("a written fixture file");
    }

    fn run(&self, args: &[&str]) -> (String, String, i32) {
        run_in(&self.dir, args)
    }

    /// Make the copy serve the `claude` harness, as a project that adopted the workflow does: the
    /// manifest declares the harness, and the root CLAUDE.md imports the shipped primer. Then
    /// stage it. The mocks declare `harness = []`, because the shipped set changes with every
    /// version of the skills; a test about the installed set declares the harness in its own copy.
    fn serve_claude(&self) {
        let path = self.path("knowledge-architect.toml");
        let manifest = std::fs::read_to_string(&path).expect("the manifest");
        assert_eq!(manifest.matches("harness = []").count(), 1, "{manifest}");
        std::fs::write(
            &path,
            manifest.replace("harness = []", "harness = [\"claude\"]"),
        )
        .expect("the manifest is rewritten");
        let claude = std::fs::read_to_string(self.path("CLAUDE.md")).expect("the root CLAUDE.md");
        self.write(
            "CLAUDE.md",
            &format!("{claude}\n@.claude/knowledge-architect/PRIMER.md\n"),
        );
        self.stage();
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// Copy a mock project, with its manifest pinned to this binary's version.
///
/// A mock says `checker-version = "fixture"`, which the binary confirms only where the mock lies
/// inside the core's directory, per `design@core@checked-sentinel-values`; a copy elsewhere is a project
/// like any other, so it carries the version.
fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the sandbox directory");
    for entry in std::fs::read_dir(from).expect("a readable fixture") {
        let entry = entry.expect("a directory entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a file type").is_dir() {
            copy_dir(&entry.path(), &target);
        } else if entry.file_name() == "knowledge-architect.toml" {
            let text = std::fs::read_to_string(entry.path()).expect("a mock manifest");
            std::fs::write(&target, pinned(&text)).expect("a copied manifest");
        } else {
            std::fs::copy(entry.path(), &target).expect("a copied file");
        }
    }
}

/// A manifest's text with `[project] checker-version` set to this binary's version: the key's
/// line replaced where `[project]` holds one, and inserted after the `[project]` line where it
/// does not. A text with no `[project]` table is returned as it is.
fn pinned(text: &str) -> String {
    let key = format!("checker-version = \"{}\"", env!("CARGO_PKG_VERSION"));
    let mut out = Vec::new();
    let mut in_project = false;
    let mut done = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            if in_project && !done {
                out.push(key.clone());
                done = true;
            }
            in_project = trimmed == "[project]";
            out.push(line.to_string());
            if in_project && !done {
                out.push(key.clone());
                done = true;
            }
            continue;
        }
        if in_project && trimmed.starts_with("checker-version") {
            continue;
        }
        out.push(line.to_string());
    }
    let mut joined = out.join("\n");
    if text.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

/// The smallest conformant mock, which declares no table the core does not own.
fn mock() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/projects/core")
}

fn check(dir: &Path) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .arg("check")
        .current_dir(dir)
        .output()
        .expect("the knowledge binary runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
    )
}

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?}");
}

#[test]
fn the_core_binary_passes_over_a_project_that_declares_only_what_the_core_reads() {
    let (code, out) = check(&mock());
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("checked: generated, registers, references\n"),
        "{out}"
    );
    assert!(out.ends_with("PASSED: no findings\n"), "{out}");
}

/// The core binary compiles in its own Component's directory, so the core's fixtures are read as
/// data, per `design@core@checker-source-literals-are-data`. Over this checkout the summary
/// names that directory and counts the core's walked Rust files, whatever the run finds.
#[test]
fn the_core_binary_names_its_own_directory_and_counts_the_files_under_it() {
    let checkout = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/core sits two levels below the root");
    let (code, out) = check(checkout);
    assert_ne!(code, 2, "the check ran: {out}");
    let line = out
        .lines()
        .find(|l| l.starts_with("checker source: "))
        .unwrap_or_else(|| panic!("no checker-source line in {out}"));
    let listing = Command::new("git")
        .args(["ls-files", "--cached", "--others", "--exclude-standard"])
        .current_dir(checkout)
        .output()
        .expect("git lists the checkout");
    let listing = String::from_utf8_lossy(&listing.stdout).into_owned();
    let core = listing
        .lines()
        .filter(|l| {
            l.starts_with("crates/core/") && l.ends_with(".rs") && !l.contains("/tests/projects/")
        })
        .count();
    assert!(core > 0);
    assert_eq!(
        line,
        format!("checker source: crates/core, {core} file(s) with string literals read as data")
    );
}

#[test]
fn the_core_binary_refuses_a_table_no_extension_of_it_claims() {
    // A copy, so the table is added to nothing another test reads. The walk is git's listing,
    // so the copy is its own repository.
    let dir = std::env::temp_dir().join(format!("knowledge-core-claims-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    copy_dir(&mock(), &dir);
    let manifest = dir.join("knowledge-architect.toml");
    let mut text = std::fs::read_to_string(&manifest).expect("the mock manifest");
    text.push_str(
        "\n[rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\nversion = \"v\"\n\
         past = \"p\"\nmanifest = \"m\"\n",
    );
    std::fs::write(&manifest, text).expect("the manifest written");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    let (code, out) = check(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("phase 1: 1 finding(s)"), "{out}");
    assert!(
        out.contains("[rules] is a table no extension of this binary reads"),
        "{out}"
    );
}

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

// ---------------------------------------------------------------------------------------
// Commit messages: `commits`
// ---------------------------------------------------------------------------------------

/// A throwaway repository holding a project the checks find nothing wrong with.
///
/// **The mock projects under `path@core@tests/projects/` are not used here.** A commit whose
/// tree fails fails the range, and `dirhome` is the mock over which every family
/// runs and finds nothing — so a copy of it could serve as a base. The project below is written out anyway, because each test
/// below states the exact findings its commits carry, and a mock's contents are shared with
/// every other test over it: a document added here for one commit's sake would move another
/// test's counts.
struct History {
    /// The project's own directory, which is where every command is run.
    dir: PathBuf,
    /// The repository's root, which is `dir` unless the project is vendored under it.
    repo: PathBuf,
}

impl History {
    fn new(tag: &str) -> Self {
        Self::at(tag, "")
    }

    /// The same, with the project `under` a subdirectory of its repository.
    ///
    /// A project vendored that way is what the manifest's `find` already supports, and every
    /// git question the per-commit read asks has to be answered project-relative rather than
    /// repository-relative.
    fn at(tag: &str, under: &str) -> Self {
        let repo = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        let dir = if under.is_empty() {
            repo.clone()
        } else {
            repo.join(under)
        };
        std::fs::create_dir_all(&dir).expect("a temporary directory");
        let history = History { dir, repo };
        history.git(&["init", "-q"]);
        // An identity in the repository's own configuration, not the machine's: a fixture
        // that read the developer's would fail wherever none is set, which is every runner.
        history.git(&["config", "user.name", "fixture"]);
        history.git(&["config", "user.email", "fixture@example.invalid"]);
        history
    }

    /// One git command at the REPOSITORY root, which is where `init`, `add` and `commit`
    /// belong: `add -A` run inside a subdirectory stages that subdirectory alone.
    fn git(&self, args: &[&str]) {
        let out = Command::new("git")
            .args(args)
            .current_dir(&self.repo)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Write a file into the project; a manifest is written pinned to this binary's version.
    fn write(&self, rel: &str, text: &str) {
        if rel == "knowledge-architect.toml" {
            self.write_bytes(rel, pinned(text).as_bytes());
        } else {
            self.write_bytes(rel, text.as_bytes());
        }
    }

    /// The same, for bytes that are not text: a blob the walk reads as a document.
    fn write_bytes(&self, rel: &str, bytes: &[u8]) {
        let path = self.dir.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
        std::fs::write(&path, bytes).expect("a written fixture file");
    }

    fn remove(&self, rel: &str) {
        std::fs::remove_file(self.dir.join(rel)).expect("a fixture file to delete");
    }

    fn run(&self, args: &[&str]) -> (String, String, i32) {
        run_in(&self.dir, args)
    }

    /// Regenerate the indexes, stage everything, and commit with this message.
    ///
    /// The indexes are regenerated first because the `generated` family compares bytes: a
    /// commit that adds or deletes a register entry and leaves the index alone has a tree
    /// that fails, and every such commit would fail the range for its tree rather than for
    /// what the test planted.
    fn commit(&self, message: &str) -> String {
        // **Staged, then regenerated, then staged again.** The walk is git's listing, so a
        // deleted file the index still holds is walked and its register entry still counted:
        // regenerating before the deletion is staged writes the listing the tree no longer
        // has. A tree whose manifest this tool refuses to load regenerates no index, and neither
        // does a tree that stops at phase 2, such as one missing a shipped file; tests commit
        // both shapes on purpose, so the run's own code is not asserted.
        self.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
        self.run(&["index"]);
        self.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
        let file = self.repo.join("message.txt");
        std::fs::write(&file, message).expect("a message file");
        let path = file.to_string_lossy().into_owned();
        // `--allow-empty`, because a test commits a message over a tree it did not change:
        // the subject here is the message, and a tree edit per commit would be noise the
        // reader has to discount.
        self.git(&[
            "commit",
            "-q",
            "--allow-empty",
            "--cleanup=verbatim",
            "-F",
            &path,
        ]);
        std::fs::remove_file(&file).expect("the message file leaves the tree");
        let out = Command::new("git")
            .args(["rev-parse", "--short=7", "HEAD"])
            .current_dir(&self.repo)
            .output()
            .expect("git runs");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }
}

impl Drop for History {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.repo);
    }
}

/// A plan document holding every section a spec or a milestone's README owes, each empty but
/// those `sections` fills, as `(title, body)` pairs.
fn plan_document(title: &str, intro: &str, sections: &[(&str, &str)]) -> String {
    const OWED: [&str; 20] = [
        "Status and audience",
        "How the work is done",
        "Names",
        "What the work is",
        "What is already decided",
        "Criteria",
        "Threads",
        "Arguments",
        "New names, in one place",
        "Decided design",
        "Mapping tables",
        "Losing alternatives",
        "Readings",
        "Premortem",
        "Acceptance criteria",
        "Implementation sequence",
        "Order rationale",
        "Defaults awaiting the owner",
        "Harvest",
        "Later consequences",
    ];
    let mut out = format!("# {title}\n\n{intro}\n");
    for owed in OWED {
        let body = sections
            .iter()
            .find(|(t, _)| *t == owed)
            .map(|(_, b)| *b)
            .unwrap_or("None.");
        out.push_str(&format!("\n## {owed}\n\n{body}\n"));
    }
    out
}

/// A slice spec holding every section a slice owes, each empty.
fn slice_document(title: &str) -> String {
    let mut out = format!("# {title}\n");
    for owed in [
        "Builds",
        "Claims",
        "Audit subjects",
        "Fails alone on",
        "Premises that expire",
    ] {
        out.push_str(&format!("\n## {owed}\n\nNone.\n"));
    }
    out
}

/// Everything a project owes, written out, with one design slug and one issue entry.
///
/// `unloadable` writes a `[project]` key the grammar does not know, so the manifest does not
/// load and the commit fails the range.
fn tiny_project(history: &History, unloadable: bool) {
    let unknown = if unloadable { "no-such-key = []\n" } else { "" };
    history.write(
        "knowledge-architect.toml",
        &format!(
            "[project]\nname = \"tiny\"\ncomponents = []\n{unknown}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n\n\
             [agents]\nharness = []\n"
        ),
    );
    history.write(
        "README.md",
        "# tiny\n\nA project a test builds so a commit has a tree with nothing wrong in it.\n",
    );
    history.write(
        "CLAUDE.md",
        "# tiny\n\nNothing here holds of any code: this project has none.\n",
    );
    history.write(
        "docs/design.md",
        "# tiny — design\n\n### A decision this project records `##tiny-anchor`\n\n\
         It exists so that a message has something to name.\n",
    );
    history.write(
        "docs/goals.md",
        "# Goals — tiny\n\n## Be committed, so a message has a tree `##tiny-goal`\n\n\
         The one goal of this project.\n",
    );
    history.write(
        "docs/tripwires.md",
        "# Tripwires — tiny\n\n## Guarding `design@tiny@tiny-anchor` `##tiny-tripwire`\n\n\
         **Fires when:** the project stops being committed.\n**Response:** reopen it.\n",
    );
    history.write(
        "docs/rejected-alternatives.md",
        "# tiny — rejected alternatives\n\nNothing has lost yet.\n",
    );
    history.write(
        "docs/open-issues/README.md",
        "# Open issues — tiny\n\nOne file per entry; the listing beside this file is generated.\n",
    );
    history.write(
        "docs/open-issues/a-closable-issue.md",
        "---\nkind: observation\n---\n# A closable issue\n\n## Summary\n\n\
         An entry a commit can delete.\n\n## Details\n\n### What\n\n\
         This entry exists to be closed.\n\n### Why it matters\n\n\
         A message naming a deleted entry resolves against the parent tree or against nothing.\n\n\
         ### What would close it\n\nThe commit that deletes this file.\n",
    );
    // The plans layout every project carries, holding no plan document; `commit` generates
    // the two indexes.
    history.write(
        "docs/plans/README.md",
        "# Plans — tiny\n\nNo plan document is open.\n",
    );
    history.write(
        "docs/plans/specs/README.md",
        "# Specs — tiny\n\nOne file per spec; the listing beside this file is generated.\n",
    );
    history.write(
        "docs/plans/milestones/README.md",
        "# Milestones — tiny\n\nOne directory per milestone; the listing beside this file is \
         generated.\n",
    );
}

/// The claim: a definition-site defect alone stops the run at phase 3, and the references
/// that would dangle because of it are not judged.
#[test]
fn a_definition_site_defect_stops_the_run_at_phase_three() {
    let sandbox = Sandbox::seeded(
        "phase-three",
        "dirhome",
        &[(
            "docs/design/misplaced.md",
            "# Misplaced\n\n#### Too deep `##too-deep`\n\nIt points at `design@dirhome@too-deep`.\n",
        )],
    );
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("phase 3:") && stdout.contains("phase 4 was not judged"),
        "{stdout}"
    );
    assert!(
        stdout.contains("`##too-deep` is written at a level-4 heading"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("is referenced") && !stdout.contains("not linked from"),
        "no phase-4 finding: {stdout}"
    );
}

/// The claim: a symlink at a walked name is read as no document and is a phase-2 finding,
/// `check` and `commits` agree on it by the mode git records, and a `skip-files` row keeps it.
#[test]
fn a_symlink_is_a_phase_two_finding_in_both_readers_and_a_skip_row_keeps_it() {
    let sandbox = Sandbox::new("symlink", "minimal");
    std::os::unix::fs::symlink("a.md", sandbox.path("notes/link.md")).expect("a symlink");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("phase 2: 1 finding(s)") && stdout.contains("`notes/link.md` is a symlink"),
        "{stdout}"
    );
    // Untracked, the same: the kind is read off the working tree where the index has none.
    let untracked = Sandbox::new("symlink-untracked", "minimal");
    std::os::unix::fs::symlink("a.md", untracked.path("notes/link.md")).expect("a symlink");
    let (stdout, _, code) = untracked.run(&["check"]);
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.contains("`notes/link.md` is a symlink"), "{stdout}");

    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            "skip-files = [\"notes/link.md\", \"notes/generated.md\"]",
        ),
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(!stdout.contains("link.md"), "{stdout}");
}

/// The claim: a submodule's gitlink is a phase-2 finding naming it, and an `exclude` row is
/// the declared silence. The entry is written straight into the index, since a submodule
/// needs no checkout to be one.
#[test]
fn a_gitlink_is_a_phase_two_finding_and_an_exclude_row_declares_the_silence() {
    let history = History::new("gitlink");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // A gitlink names a commit, which need not be reachable; the empty directory is what an
    // uninitialised submodule looks like, and what keeps the staging step from removing it.
    std::fs::create_dir_all(history.dir.join("vendor/sub")).expect("the submodule's place");
    history.git(&[
        "update-index",
        "--add",
        "--cacheinfo",
        "160000,e69de29bb2d1d6434b8b29ae775ad8c2e48c5391,vendor/sub",
    ]);
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("phase 2: 1 finding(s)") && stdout.contains("`vendor/sub` is a submodule"),
        "{stdout}"
    );
    let sha = history.commit("A submodule is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s)"
        )),
        "the two readers agree: {stdout}"
    );

    let manifest = std::fs::read_to_string(history.dir.join("knowledge-architect.toml"))
        .expect("the manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest.replace("exclude = []", "exclude = [\"vendor/sub\"]"),
    );
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

/// The claim: a committed symlink at a walked name is one phase-2 finding in `commits` too,
/// read off the tree's mode, and is walked as no document there either.
#[test]
fn a_committed_symlink_is_one_phase_two_finding_in_commits() {
    let history = History::new("commit-symlink");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    std::os::unix::fs::symlink("design.md", history.dir.join("docs/link.md")).expect("a symlink");
    let sha = history.commit("A symlink is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s)"
        )),
        "one finding, the symlink, and not a second for a document nothing could read: {stdout}"
    );
}

#[test]
fn check_takes_no_selection() {
    // Every check runs on every run, in the phases; a selection would cut across them.
    let (stdout, stderr, code) = run("planted", &["check", "--only", "references"]);
    assert!(stderr.contains("--only"), "{stderr}");
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

/// The claim: `issues` prints one row per issue entry with the five columns in order, and each
/// filter keeps only what it names.
///
/// Recorded mutation, `cargo mutate run` over `path@core@src/cli/mod.rs`: turning the `--kind`
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

/// The claim: `--group` keeps the rows whose entry sits in the group it names, and drops the
/// ungrouped ones.
///
/// The only grouped ISSUE instance in any mock is `dirhome`'s, and it is read in place rather
/// than copied: the command writes nothing. Both directions are asserted, because a filter that
/// dropped every row would satisfy a test that only checked the grouped entry was gone.
#[test]
fn issues_group_filter_keeps_the_entries_of_the_group_it_names() {
    let (all, stderr, code) = run("dirhome", &["issues"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(all.contains("a-grouped-entry"), "{all}");
    assert!(all.contains("an-ungrouped-entry"), "{all}");

    let (grouped, _, code) = run("dirhome", &["issues", "--group", "housekeeping"]);
    assert_eq!(code, 0, "{grouped}");
    assert!(grouped.contains("a-grouped-entry"), "{grouped}");
    assert!(!grouped.contains("an-ungrouped-entry"), "{grouped}");

    // A group the instance does not declare keeps nothing, and no row is exit 1.
    let (none, _, code) = run("dirhome", &["issues", "--group", "no-such-group"]);
    assert_eq!(code, 1, "{none}");
    assert!(!none.contains("a-grouped-entry"), "{none}");
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

/// The claim, per `design@core@planned-path-form`: `show` on a planned reference resolves whether
/// or not the target exists, says which, and lists the plans that cite it, which are the sites a
/// conversion edits.
#[test]
fn show_of_a_planned_path_lists_its_citations_and_says_whether_it_exists() {
    let sandbox = Sandbox::new("show-planned", "minimal");
    sandbox.write(
        "docs/plans/README.md",
        "# Plan documents\n\nThe mock's plans directory: specs under specs/, milestones under \
         milestones/.\n\nIt creates `planned@minimal@code/new.rs` beside \
         `planned@minimal@code/lib.rs`.\n\nThe same target, slashed: `planned@minimal@code/new.rs/`.\n",
    );
    let (absent, stderr, code) = sandbox.run(&["show", "planned@minimal@code/new.rs"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(absent.contains("does not hold the target"), "{absent}");
    assert!(absent.contains("docs/plans/README.md:5"), "{absent}");
    // The slash is a kind claim, not another target: both citations are listed.
    assert!(absent.contains("docs/plans/README.md:7"), "{absent}");

    // The path the planned form converts to lists the planned citations: the sites to edit.
    let (path, _, code) = sandbox.run(&["show", "path@minimal@code/lib.rs"]);
    assert_eq!(code, 0, "{path}");
    assert!(path.contains("docs/plans/README.md:5"), "{path}");

    // A reference the check refuses resolves to nothing here too.
    let (refused, _, code) = sandbox.run(&["show", "planned@minimal@../x.rs"]);
    assert_eq!(code, 1, "{refused}");
    assert!(refused.contains("resolves to nothing"), "{refused}");

    let (present, stderr, code) = sandbox.run(&["show", "planned@minimal@code/lib.rs"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(present.contains("now exists"), "{present}");
    assert!(
        !present.contains("A doc comment pointing"),
        "no body is printed: {present}"
    );
    assert!(present.contains("docs/plans/README.md:5"), "{present}");
}

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

    let (before, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(
        code, 1,
        "the dangling reference is a finding: {before}{stderr}"
    );
    assert!(before.contains("no-such-thing"), "{before}");

    // The recipe: one root gitignore line holding the document's bare filename. The document
    // is already tracked, so `git add -A` leaves it tracked.
    sandbox.write(".gitignore", "b.md\n");
    sandbox.stage();
    let (after, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{after}{stderr}");
    // The contradiction is a phase-2 fact, so the run stops there and the dangling reference
    // is not judged; that the document is still walked is what the walk count says.
    assert_eq!(
        walked_count(&after),
        walked_count(&before),
        "the document must still be walked: {after}"
    );
    assert!(after.contains("phase 2:"), "{after}");
    assert!(
        !after.contains("no-such-thing"),
        "nothing later is judged over an incomplete model: {after}"
    );
    assert!(
        after.contains("notes/b.md") && after.contains("git tracks this file"),
        "the tracked-and-ignored pair must be named: {after}"
    );
    // The summary says how much was walked, on a failing run as readily as a passing one.
    assert!(walked_count(&after) > 0, "{after}");
}

/// The claim: a file outside the walk that cannot be read stops the run at phase 2, naming it.
///
/// The read is made to fail by a loop, not by a mode: the tracked file is replaced on disk by a
/// symlink to itself, which every user fails to open, root included, where root reads through a
/// mode of 000. The index still records a regular file, so the listing does not report a
/// symlink and the file reaches the read.
#[test]
fn an_unreadable_file_outside_the_walk_is_a_phase_two_finding_naming_it() {
    let sandbox = Sandbox::seeded("unreadable-outside", "minimal", &[("notes/old.txt", "x\n")]);
    let path = sandbox.path("notes/old.txt");
    std::fs::remove_file(&path).expect("the file");
    std::os::unix::fs::symlink("old.txt", &path).expect("a symlink to itself");
    assert!(
        std::fs::read(&path).is_err(),
        "the loop must fail every read, or this test asserts nothing"
    );
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("phase 2:"), "{out}");
    let about: Vec<&str> = out
        .lines()
        .filter(|l| l.contains("notes/old.txt"))
        .collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(about[0].contains("could not be read"), "{out}");

    // Readable again, it is an ordinary file outside the walk, and the run passes.
    std::fs::remove_file(&path).expect("the symlink");
    std::fs::write(&path, "x\n").expect("the file");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// The claim: a file outside the walk that git lists and the disk does not hold, and an untracked
/// nested repository, are each a phase-2 finding with a statement of its own, not the one for a
/// file that could not be read.
#[test]
fn a_missing_file_and_a_nested_repository_outside_the_walk_are_named_for_what_they_are() {
    let sandbox = Sandbox::seeded("missing-outside", "minimal", &[("notes/old.txt", "x\n")]);
    std::fs::remove_file(sandbox.path("notes/old.txt")).expect("the file");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out
        .lines()
        .filter(|l| l.contains("notes/old.txt"))
        .collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(
        about[0].contains("the working tree does not hold it"),
        "{out}"
    );
    assert!(out.contains("stage the deletion"), "{out}");

    let nested = Sandbox::new("nested-outside", "minimal");
    nested.write("sub/a.txt", "x\n");
    git(&nested.path("sub"), &["init", "-q"]);
    let (out, stderr, code) = nested.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out.lines().filter(|l| l.starts_with("sub")).collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(
        about[0].contains("a repository nested in this one"),
        "{out}"
    );
}

/// The claim: a symlink whose name the walk refuses is one finding, the link's, and not a second
/// for its name.
#[test]
fn a_symlink_with_a_refused_name_is_one_finding() {
    let sandbox = Sandbox::new("symlink-refused", "minimal");
    std::os::unix::fs::symlink("README.md", sandbox.path("a:b")).expect("a symlink");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out.lines().filter(|l| l.starts_with("a:b")).collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(about[0].contains("symlink"), "{out}");
}

/// The claim: a tree holding the core's own crate at another path than the binary was built
/// from is a second checkout of the tool, and no command runs over it: the binary would judge it
/// with the other checkout's code.
///
/// The copy stands for the second checkout: a crate declaring the core's package name at the
/// path relative to the tree the core has in this repository. The reverse arrangement, two
/// checkouts building into one target directory, is what produces this state in practice.
#[test]
fn a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused() {
    let sandbox = Sandbox::seeded(
        "foreign-build",
        "minimal",
        &[(
            "crates/core/Cargo.toml",
            "[package]\nname = \"knowledge-architect\"\n",
        )],
    );
    for command in [&["check"][..], &["issues"][..]] {
        let (out, stderr, code) = sandbox.run(command);
        assert_eq!(code, 2, "{command:?}: {out}{stderr}");
        assert!(stderr.contains("built from another checkout"), "{stderr}");
        assert!(
            // The profile of the clean is the binary's, which is this test's.
            stderr.contains(&format!(
                "cargo clean{} -p knowledge-architect -p knowledge-architect-agent-skills`",
                if cfg!(debug_assertions) {
                    ""
                } else {
                    " --release"
                }
            )),
            "{stderr}"
        );
        assert!(out.is_empty(), "nothing ran: {out}");
    }
}

/// The claim: a name holding a line break is one finding, on one line, and the file is read by
/// nothing; a `skip-files` row keeps it.
#[test]
fn a_name_holding_a_newline_is_one_finding_on_one_line_and_its_contents_are_read_by_nothing() {
    // The reproduction the issue named: a markdown file whose name holds a newline, holding a
    // reference that would be a finding if the file were read. The refusal is the one finding,
    // on one line, with the newline escaped; the reference inside is reported by nothing.
    let sandbox = Sandbox::seeded("newline-name", "minimal", &[("notes/a\nb.md", DANGLING)]);
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out.lines().filter(|l| l.contains("notes/a")).collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(
        about[0].starts_with("notes/a\\nb.md  ") && about[0].contains("holds a line break"),
        "{out}"
    );
    assert!(!out.contains("no-such-thing"), "read by nothing: {out}");

    // The declared way to keep such a file: a `skip-files` row, which a TOML string spells
    // with an escape. The row takes the file out of the walk like any other, so the run
    // passes and the file is still read by nothing.
    let kept = Sandbox::seeded("newline-kept", "minimal", &[("notes/a\nb.md", DANGLING)]);
    let manifest =
        std::fs::read_to_string(kept.path("knowledge-architect.toml")).expect("the manifest");
    kept.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            "skip-files = [\"notes/a\\nb.md\", \"notes/generated.md\"]",
        ),
    );
    kept.stage();
    let (out, stderr, code) = kept.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(!out.contains("notes/a"), "{out}");
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
    let (out, stderr, code) = ignored.run(&["check"]);
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
    let (walked_out, stderr, code) = walked.run(&["check"]);
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
    let (out, stderr, code) = exempt.run(&["check"]);
    assert_eq!(code, 0, "the ignore rules cover the target: {out}{stderr}");
    assert!(!out.contains("build-output"), "{out}");

    let asserted = Sandbox::seeded("unignored-target", "minimal", &[("notes/a.md", pointer)]);
    let (out, stderr, code) = asserted.run(&["check"]);
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
/// is the shape `design@core@a-failed-parse-is-loud` refuses. The finding names the deletion
/// rather than reporting an encoding failure, because the two need different repairs.
#[test]
fn a_tracked_file_the_working_tree_does_not_hold_is_reported() {
    let sandbox = Sandbox::new("deleted-tracked", "minimal");
    let (before, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(
        code, 0,
        "the fixture starts clean under this family: {before}{stderr}"
    );

    std::fs::remove_file(sandbox.path("notes/b.md")).expect("a staged fixture file");
    let (out, stderr, code) = sandbox.run(&["check"]);
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

    // Staging the deletion is the repair, and it takes the path out of the listing. What is
    // then reported is the fixture's own pointer at the file, dangling, which is the tree's
    // fact and not the walk's.
    sandbox.stage();
    let (after, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{after}{stderr}");
    assert!(
        !after.contains("the working tree does not hold it"),
        "{after}"
    );
    assert!(
        after.contains("`path@notes@b.md` does not exist"),
        "{after}"
    );
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
    let (out, stderr, code) = run_with_env(&sandbox.dir, &["check"], &env);
    assert_eq!(code, 1, "the document stays live: {out}{stderr}");
    assert!(out.contains("no-such-thing"), "{out}");

    // The same run without the fake home, so the count is the same either way.
    let (plain, _, _) = sandbox.run(&["check"]);
    assert_eq!(walked_count(&out), walked_count(&plain));
    let _ = std::fs::remove_dir_all(&home);
}

/// The claim: a project written this way has nothing wrong with it, so a commit over it is
/// judged and contributes no finding of its tree.
///
/// Every test below reads a planted finding out of a run whose other findings are none. With
/// a tree that failed, every commit would carry its tree's findings and each test's counts
/// would be counts of the fixture.
#[test]
fn the_fixture_project_is_one_the_checks_find_nothing_wrong_with() {
    let history = History::new("commit-clean");
    tiny_project(&history, false);
    history.commit("The project is created\n");
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("PASSED: no findings"), "{stdout}");
}

#[test]
fn a_message_naming_nothing_that_exists_fails_and_names_the_commit_and_the_line() {
    let history = History::new("commit-dangling");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let sha = history.commit(
        "A subject line\n\nA body naming `design@tiny@no-such-decision`, which is nowhere.\n",
    );
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{sha} failed: its message carries 1 finding(s)")),
        "{stdout}"
    );
    // The commit, then the line inside the message: the body sits on line three.
    assert!(
        stdout.contains(&format!("commit {sha}:3")),
        "the finding names the commit and the line: {stdout}"
    );
    assert!(stdout.contains("no-such-decision"), "{stdout}");
}

/// `tiny_project`, with `[commits] refuse-branch-shas = true` in its manifest.
fn tiny_project_refusing_branch_shas(history: &History) {
    tiny_project(history, false);
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        format!("{text}\n[commits]\nrefuse-branch-shas = true\n"),
    )
    .expect("the manifest is rewritten");
}

/// The claim: under `refuse-branch-shas`, a message citing a commit of the range by its SHA
/// fails, naming the commit and the line, and the citing commit is counted as failed, its line
/// naming the citations and how many. Mutation checked: the count off by one.
#[test]
fn a_message_citing_a_commit_of_the_range_by_sha_fails_under_refuse_branch_shas() {
    let history = History::new("commit-branch-sha");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    let second = history.commit(&format!(
        "A second change\n\nIt follows {first}.\nIt names {first} again.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {second}:3")), "{stdout}");
    assert!(stdout.contains(&format!("commit {second}:4")), "{stdout}");
    // The cited commit is named by its 7-digit abbreviation, followed by the rest of the sentence.
    assert!(
        stdout.contains(&format!(
            "`{first}` cites commit {first}, a commit of the range"
        )),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!(
            "{second} failed: it cites the range by SHA 2 time(s)\n"
        )),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{first} passed")), "{stdout}");
}

/// The claim: whether a commit is judged for branch SHAs is read from that commit's own
/// manifest, not from the working tree's. Mutation: reading the working tree's manifest, which
/// the last commit turns the option off in, lets the citing commit pass.
#[test]
fn a_commit_is_judged_for_branch_shas_under_its_own_manifest() {
    let history = History::new("commit-branch-sha-own-manifest");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    let second = history.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        text.replace("refuse-branch-shas = true", "refuse-branch-shas = false"),
    )
    .expect("the manifest is rewritten");
    let third = history.commit("The option is turned off\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{second} failed")), "{stdout}");
    assert!(stdout.contains(&format!("{third} passed")), "{stdout}");
}

/// The claim: a commit whose tree carries a finding of its own is still judged for branch SHAs,
/// and both are reported. Mutation: judging citations only where the tree is clean drops them.
#[test]
fn a_citation_in_a_commit_whose_tree_fails_is_reported_beside_the_tree_finding() {
    let history = History::new("commit-branch-sha-failing-tree");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write(
        "README.md",
        "# tiny\n\nA project a test builds. It names `design@tiny@no-such-decision`.\n",
    );
    let second = history.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("no-such-decision"), "{stdout}");
    assert!(stdout.contains(&format!("commit {second}:3")), "{stdout}");
}

/// The claim: a commit whose tree stops at phase 2 is still scanned for citations of the range,
/// and its line names both causes with the count, per `design@core@branch-shas-are-refused`;
/// without the option, the same commit is not scanned.
#[test]
fn a_citation_in_a_commit_whose_tree_stops_at_phase_two_is_reported() {
    let history = History::new("commit-branch-sha-stopped-tree");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write_bytes(
        "docs/latin1.md",
        b"# A note\n\nOne byte of Windows-1252: caf\xe9.\n",
    );
    let second = history.commit(&format!(
        "A second change\n\nIt follows {first}.\nIt names {first} again.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {second}:3")), "{stdout}");
    assert!(stdout.contains(&format!("commit {second}:4")), "{stdout}");
    assert!(
        stdout.contains(&format!(
            "{second} failed: its tree stops at phase 2 with 1 finding(s); its message was \
             judged against nothing; it cites the range by SHA 2 time(s)\n"
        )),
        "{stdout}"
    );

    let plain = History::new("commit-branch-sha-stopped-tree-off");
    tiny_project(&plain, false);
    let base = plain.commit("The project is created\n");
    let first = plain.commit("A first change\n");
    plain.write_bytes(
        "docs/latin1.md",
        b"# A note\n\nOne byte of Windows-1252: caf\xe9.\n",
    );
    plain.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let (stdout, stderr, code) = plain.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("its tree stops at phase 2"), "{stdout}");
    assert!(!stdout.contains("cites the range"), "{stdout}");
}

/// The claim: a commit whose tree stops at phase 1 is not scanned for citations, since its
/// manifest did not resolve, per `design@core@branch-shas-are-refused`.
#[test]
fn a_commit_whose_tree_stops_at_phase_one_is_not_scanned_for_citations() {
    let history = History::new("commit-branch-sha-phase-one");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    let manifest =
        std::fs::read_to_string(history.dir.join("knowledge-architect.toml")).expect("manifest");
    history.write("knowledge-architect.toml", &format!("{manifest}\n[lint]\n"));
    let second = history.commit(&format!(
        "A table no extension claims\n\nIt follows {first}.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{second} failed: its tree stops at phase 1")),
        "{stdout}"
    );
    assert!(!stdout.contains("cites the range"), "{stdout}");
    assert!(!stdout.contains(&format!("commit {second}:3")), "{stdout}");
}

/// The claim: a Rust document is read whole, its code included. Mutation: reading Markdown
/// documents alone misses it.
#[test]
fn a_rust_document_citing_a_commit_of_the_range_fails() {
    let history = History::new("commit-branch-sha-rust");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write(
        "src/lib.rs",
        &format!("const FOLLOWS: &str = \"{first}\";\n"),
    );
    let second = history.commit("A source file names a change\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {second}: src/lib.rs:1")),
        "{stdout}"
    );
}

/// The claim: a document of a commit's tree citing a commit of the range fails too, named by
/// the commit and the file.
#[test]
fn a_document_citing_a_commit_of_the_range_by_sha_fails_under_refuse_branch_shas() {
    let history = History::new("commit-branch-sha-doc");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write(
        "README.md",
        &format!("# tiny\n\nA project a test builds. It changed in {first}.\n"),
    );
    let second = history.commit("The readme names a change\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {second}: README.md:3")),
        "{stdout}"
    );
}

/// The claim: a SHA outside the range passes under the option, and a citation of the range
/// passes without it. Mutations: matching any hex run fails the first half, and ignoring the
/// manifest fails the second.
#[test]
fn a_sha_outside_the_range_or_a_project_without_the_option_passes() {
    let history = History::new("commit-main-sha");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    history.commit(&format!(
        "A change\n\nIt builds on {base}, already on the main branch.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");

    let plain = History::new("commit-branch-sha-off");
    tiny_project(&plain, false);
    let base = plain.commit("The project is created\n");
    let first = plain.commit("A first change\n");
    plain.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let (stdout, stderr, code) = plain.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

#[test]
fn a_message_naming_the_entry_its_commit_deletes_resolves_against_the_parent() {
    let history = History::new("commit-parent");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.remove("docs/open-issues/a-closable-issue.md");
    let sha = history
        .commit("The closable issue is closed\n\nIt closed `issue@tiny@a-closable-issue`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(
        code, 0,
        "the parent tree still defines it: {stdout}{stderr}"
    );
    assert!(stdout.contains(&format!("{sha} passed")), "{stdout}");
}

/// The claim: `--help` opens with the binary's own description, not with the doc comment the
/// library writes for a binary's author about the command enum.
#[test]
fn help_opens_with_the_binarys_description() {
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .arg("--help")
        .output()
        .expect("the binary runs");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert_eq!(
        stdout.lines().next(),
        Some("What a project's documents hold, and whether it still holds."),
        "{stdout}"
    );
    assert!(!stdout.contains("flattens this enum"), "{stdout}");
}

/// The claim: in a message, an unanchored path whose first segment only the parent tree holds
/// is still a finding, since the message points at what its commit deletes; a path-shaped span
/// that names nothing in either tree is silent.
#[test]
fn a_message_naming_the_file_its_commit_deletes_by_a_bare_path_is_a_finding() {
    let history = History::new("commit-bare-path");
    tiny_project(&history, false);
    history.write(
        "scratch/notes.md",
        "# Notes\n\nA file a later commit deletes.\n",
    );
    let base = history.commit("The project is created\n");
    let silent = history.commit("A subject line\n\nIt answers in `application/json`.\n");
    history.remove("scratch/notes.md");
    let sha = history.commit("The notes leave\n\nIt deletes `scratch/notes.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`scratch/notes.md` is shaped like a path"),
        "{stdout}"
    );
    assert!(!stdout.contains(&format!("commit {silent}:")), "{stdout}");
}

/// The claim: a commit whose message alone carries findings is counted failed, and its line
/// says so, while the commit before it passes. Mutation checked: the message's findings left
/// out of the causes.
#[test]
fn a_commit_failing_on_its_message_alone_is_counted_failed() {
    let history = History::new("commit-message-fails");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let clean = history.commit("A subject line\n\nIt argues from `design@tiny@tiny-anchor`.\n");
    let sha = history
        .commit("A subject line\n\nIt names `design@tiny@nothing` and `design@tiny@none`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{clean} passed")), "{stdout}");
    assert!(
        stdout.contains(&format!("{sha} failed: its message carries 2 finding(s)")),
        "{stdout}"
    );
    assert!(stdout.contains("2 commits, 1 passed, 1 failed"), "{stdout}");
    let (stdout, stderr, _) = history.run(&["commits", &format!("{clean}..HEAD")]);
    assert!(
        stdout.contains("1 commit, 0 passed, 1 failed"),
        "{stdout}{stderr}"
    );
}

/// The claim: a lint both trees raise on one line is reported once, and a lint on another line
/// is reported again. Mutation checked: the union of the two trees' lints made a plain extend.
#[test]
fn a_lint_both_trees_raise_is_reported_once() {
    let history = History::new("commit-lint-once");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let sha =
        history.commit("A subject line\n\nIt reads `docs/design.md`.\nAnd `docs/design.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for line in [3, 4] {
        let site = format!("commit {sha}:{line} ");
        assert_eq!(
            stdout.lines().filter(|l| l.contains(&site)).count(),
            1,
            "{stdout}"
        );
    }
}

/// The claim: in a message, a retired slug reference is a finding when its id is an entry of
/// the project, and silent when it names nothing here.
#[test]
fn a_message_holding_the_retired_slug_form_is_judged_by_what_it_names() {
    let history = History::new("commit-retired");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let silent = history.commit("A subject line\n\nIt closes `#123` and keeps `#include`.\n");
    let sha = history.commit("A subject line\n\nIt argues from `#tiny-anchor`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`#tiny-anchor` is the retired slug reference form"),
        "{stdout}"
    );
    assert!(!stdout.contains(&format!("commit {silent}:")), "{stdout}");
}

/// The claim: in a message, an unanchored path whose first segment only the commit's own tree
/// holds is a finding, as one only the parent holds is.
#[test]
fn a_message_naming_the_file_its_commit_adds_by_a_bare_path_is_a_finding() {
    let history = History::new("commit-bare-added");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write("scratch/notes.md", "# Notes\n\nA file this commit adds.\n");
    let sha = history.commit("The notes arrive\n\nIt adds `scratch/notes.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`scratch/notes.md` is shaped like a path"),
        "{stdout}"
    );
}

/// The claim: in a message, a bare path is read from the parent's anchors in the parent's tree, so
/// a path relative to a Component the commit removes is still a finding.
#[test]
fn a_message_naming_a_path_of_a_component_its_commit_removes_is_a_finding() {
    let history = History::new("commit-bare-component");
    tiny_project(&history, false);
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        text.replace("components = []", "components = [\"part\"]"),
    )
    .expect("the manifest is rewritten");
    let part = [
        (
            "part/README.md",
            "# part\n\nA Component a later commit removes.\n",
        ),
        ("part/docs/design.md", "# part — design\n\nNo entry.\n"),
        ("part/docs/goals.md", "# Goals — part\n\nNo entry.\n"),
        (
            "part/docs/tripwires.md",
            "# Tripwires — part\n\nNo entry.\n",
        ),
        (
            "part/docs/rejected-alternatives.md",
            "# part — rejected\n\nNothing.\n",
        ),
        (
            "part/docs/open-issues/README.md",
            "# Open issues — part\n\nOne file per entry.\n",
        ),
        ("part/src/lib.rs", "pub fn f() {}\n"),
    ];
    for (rel, body) in part {
        history.write(rel, body);
    }
    let base = history.commit("The project is created\n");
    std::fs::write(&manifest, text).expect("the manifest is restored");
    std::fs::remove_dir_all(history.dir.join("part")).expect("the Component is removed");
    let sha = history.commit("The part leaves\n\nIt drops `src/lib.rs`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`src/lib.rs` is shaped like a path"),
        "{stdout}"
    );
}

/// The claim: an unanchored path through a tracked symlink is judged like any other, in a
/// document and in a message, and asks git nothing that a symlink makes it refuse.
#[test]
fn a_bare_path_through_a_tracked_symlink_is_a_finding_and_not_an_error() {
    let history = History::new("bare-symlink");
    tiny_project(&history, false);
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        text.replace("skip-files = []", "skip-files = [\"lnk\"]"),
    )
    .expect("the manifest is rewritten");
    std::os::unix::fs::symlink("docs", history.dir.join("lnk")).expect("a symlink");
    let base = history.commit("The project is created\n");
    history.commit("A subject line\n\nIt reads `lnk/design.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("`lnk/design.md` is shaped like a path"),
        "{stdout}"
    );
    history.write("notes.md", "# Notes\n\nSee `lnk/design.md`.\n");
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("`lnk/design.md` is shaped like a path"),
        "{stdout}"
    );
}

/// The claim: a commit message citing a plan document in the commit that deletes it resolves
/// against the parent, for a spec of `specs/` and for a slice spec, and also when the whole
/// milestone leaves, so its anchor is gone from the commit's tree. The deletion that ends a plan
/// document's life is written this way, since a `path` citation of one is refused.
#[test]
fn a_message_naming_the_plan_document_its_commit_deletes_resolves_against_the_parent() {
    let history = History::new("commit-plan-parent");
    tiny_project(&history, false);
    history.write(
        "docs/plans/specs/a-spec.md",
        &plan_document(
            "A spec that lands",
            "It is the plan of one pull request.",
            &[],
        ),
    );
    history.write(
        "docs/plans/milestones/a-milestone/README.md",
        &plan_document(
            "A milestone that lands",
            "Its steps: [the first](first.md), [the last](last.md); [the index](index.md).",
            &[],
        ),
    );
    history.write(
        "docs/plans/milestones/a-milestone/first.md",
        &slice_document("Its first step"),
    );
    history.write(
        "docs/plans/milestones/a-milestone/last.md",
        &slice_document("Its last step"),
    );
    let base = history.commit("The plans are written\n");
    history.remove("docs/plans/specs/a-spec.md");
    history.remove("docs/plans/milestones/a-milestone/first.md");
    history.write(
        "docs/plans/milestones/a-milestone/README.md",
        &plan_document(
            "A milestone that lands",
            "Its step: [the last](last.md); [the index](index.md).",
            &[],
        ),
    );
    let spec = history.commit(
        "The spec and the first step land\n\nThey leave: `spec@plans@a-spec` and \
         `spec@a-milestone@first`.\n",
    );
    for rel in ["README.md", "last.md", "index.md"] {
        history.remove(&format!("docs/plans/milestones/a-milestone/{rel}"));
    }
    let milestone = history.commit(
        "The milestone lands\n\nIt leaves: `milestone@plans@a-milestone` and \
         `spec@a-milestone@last`.\n",
    );
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(
        code, 0,
        "the parent tree still defines each: {stdout}{stderr}"
    );
    assert!(stdout.contains(&format!("{spec} passed")), "{stdout}");
    assert!(stdout.contains(&format!("{milestone} passed")), "{stdout}");
}

/// The claim: a commit message cites a plan document whole, never an item. It is one document
/// at the root, inside no plan, so an item citation in it is the out-of-plan refusal, while the
/// whole document resolves. Mutation checked: `judge_message` resolving with no citing file,
/// which treats the message as inside every plan.
#[test]
fn a_message_cites_a_plan_whole_and_never_an_item() {
    let history = History::new("commit-plan-item");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/plans/specs/a-spec.md",
        &plan_document(
            "A spec with one thread",
            "It is the plan of one pull request.",
            &[(
                "Threads",
                "### It holds one thread `##one-thread`\n\nApproved.",
            )],
        ),
    );
    let whole = history.commit("A spec is planned\n\nIt is `spec@plans@a-spec`.\n");
    let item = history.commit("Its thread is named\n\nIt is `thread@a-spec@one-thread`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{whole} passed")), "{stdout}");
    assert!(
        stdout.contains(&format!("commit {item}:3"))
            && stdout.contains("cites an item of the plan `a-spec` from outside it"),
        "{stdout}"
    );
}

/// The claim: the anchors a message is judged against are its commit's, milestone anchors
/// included, and not the working tree's. The milestone's deletion is staged and not committed
/// before the range is judged, so git's listing of the working tree holds no anchor
/// `a-milestone`, and a run that read the anchors off it would find none in either tree.
#[test]
fn a_commit_s_milestone_anchors_are_read_off_its_own_tree() {
    let history = History::new("commit-milestone-anchor");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/plans/milestones/a-milestone/README.md",
        &plan_document(
            "A milestone",
            "Its step: [the step](a-step.md); [the index](index.md).",
            &[],
        ),
    );
    history.write(
        "docs/plans/milestones/a-milestone/a-step.md",
        &slice_document("Its step"),
    );
    let sha = history.commit("A milestone is planned\n\nIts step is `spec@a-milestone@a-step`.\n");
    for rel in ["README.md", "a-step.md", "index.md"] {
        history.remove(&format!("docs/plans/milestones/a-milestone/{rel}"));
    }
    history.git(&["add", "-A"]);
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(
        code, 0,
        "the commit's own tree defines it: {stdout}{stderr}"
    );
    assert!(stdout.contains(&format!("{sha} passed")), "{stdout}");
}

/// The claim: a reference that resolves in NEITHER tree is reported, however differently the
/// two trees phrase their refusal.
///
/// A commit that adds a component makes the two arms disagree about the words: the parent has
/// no such anchor at all, the commit has the anchor and not the id. Intersecting the findings
/// whole then produces the empty set, and a reference nothing defines anywhere passes.
#[test]
fn a_reference_no_tree_defines_is_reported_though_the_two_refuse_it_differently() {
    let history = History::new("commit-both-refuse");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");

    // A second component, which the parent tree does not know as an anchor at all.
    for (rel, text) in [
        ("other/README.md", "# other\n\nA second component.\n"),
        ("other/CLAUDE.md", "# other\n\nNothing holds of it.\n"),
        (
            "other/docs/design.md",
            "# other — design\n\n### It records one decision `##other-anchor`\n\nIt exists.\n",
        ),
        (
            "other/docs/goals.md",
            "# Goals — other\n\n## Be a second anchor `##other-goal`\n\nOne goal.\n",
        ),
        (
            "other/docs/tripwires.md",
            "# Tripwires — other\n\nNothing yet.\n",
        ),
        (
            "other/docs/rejected-alternatives.md",
            "# other — rejected alternatives\n\nNothing has lost yet.\n",
        ),
        (
            "other/docs/open-issues/README.md",
            "# Open issues — other\n\nOne file per entry.\n",
        ),
    ] {
        history.write(rel, text);
    }
    let manifest =
        std::fs::read_to_string(history.dir.join("knowledge-architect.toml")).expect("a manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest.replace("components = []", "components = [\"other\"]"),
    );
    let sha = history.commit(
        "A second component arrives\n\nIt names `design@other@no-such-slug`, which nothing defines.\n",
    );

    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "neither tree defines it: {stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {sha}:3")), "{stdout}");
    assert!(stdout.contains("no-such-slug"), "{stdout}");
}

/// The claim: a commit whose manifest does not load fails the run and is
/// named, and the commit after it is still judged.
///
/// Its message cannot be judged, since no table exists to resolve it against, so the tree's
/// failure is the finding. A run that passed over it would take the message out of the regime
/// with exit 0.
#[test]
fn a_commit_whose_manifest_does_not_load_fails_the_run_and_the_next_one_is_judged() {
    let history = History::new("commit-premigration");
    tiny_project(&history, true);
    let base = history.commit("The project is created\n");
    let old = history.commit("A commit from before the manifest was migrated\n");
    tiny_project(&history, false);
    let new = history.commit("The manifest is migrated\n\nIt records `design@tiny@tiny-anchor`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{old} failed: its tree does not load")),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{new} passed")), "{stdout}");
    assert!(stdout.contains("2 commits, 1 passed, 1 failed"), "{stdout}");
    assert!(
        stdout.contains(&format!("commit {old}  this commit's tree does not load")),
        "the finding names the commit: {stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: a commit whose tree carries a finding fails the run with that
/// finding, named by the commit and the file, and its message is still judged.
///
/// The tip checker judges every commit of the branch, so a tree that fails at any commit is a
/// finding of the branch. Its message is judged too, since the tree reached the last phase and
/// its table is complete: a dangling reference in it is reported beside the tree's own.
#[test]
fn a_failing_tree_is_a_finding_and_its_message_is_still_judged() {
    let history = History::new("commit-middle-fails");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/note.md",
        "# A note\n\nIt names `design@tiny@no-such-decision`.\n",
    );
    let mid = history.commit("A note is added\n\nIt names `design@tiny@no-such-message-ref`.\n");
    history.remove("docs/note.md");
    let fixed = history.commit("The note leaves\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{mid} failed: its tree fails 1 finding(s); its message carries 1 finding(s)"
        )),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{fixed} passed")), "{stdout}");
    assert!(stdout.contains("2 commits, 1 passed, 1 failed"), "{stdout}");
    // The tree's finding, named by the commit and by the file inside it.
    assert!(
        stdout.contains(&format!("commit {mid}: docs/note.md:3")),
        "the tree's finding names the commit and the file: {stdout}"
    );
    // The message's finding, on line three of the message.
    assert!(stdout.contains(&format!("commit {mid}:3")), "{stdout}");
    assert!(stdout.contains("no-such-message-ref"), "{stdout}");
}

/// The claim: a commit whose tree stops before the last phase fails the run,
/// and its message is judged against nothing, which the run says.
///
/// Its entity table is incomplete, so a finding against the message would be computed over
/// what the walk could not read. The tree's own findings are what is reported.
#[test]
fn a_tree_that_stops_early_fails_and_its_message_is_not_judged() {
    let history = History::new("commit-middle-stops");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write_bytes("docs/latin1.md", b"# A note\n\ncaf\xe9\n");
    let mid =
        history.commit("A note that is not text\n\nIt names `design@tiny@no-such-message-ref`.\n");
    history.remove("docs/latin1.md");
    history.commit("The note leaves\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{mid} failed: its tree stops at phase 2")),
        "{stdout}"
    );
    assert!(stdout.contains("latin1.md"), "{stdout}");
    assert!(
        !stdout.contains("no-such-message-ref"),
        "no finding computed over the incomplete table: {stdout}"
    );
}

/// The claim: a tree entry the walk reads as a document and whose blob is not text is a
/// finding of that tree, so the commit fails for it rather than being judged clean over a
/// document nothing read.
#[test]
fn a_commit_holding_a_blob_that_is_not_text_fails_its_tree() {
    let history = History::new("commit-blob");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write_bytes(
        "docs/latin1.md",
        b"# A note\n\nOne byte of Windows-1252: caf\xe9.\n",
    );
    let sha = history.commit("A note that is not text is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s)"
        )),
        "{stdout}"
    );
}

/// The claim: a last commit whose tree stops before the last phase fails the run like any other
/// commit, and has its message judged against nothing rather than against the incomplete entity
/// table.
#[test]
fn a_last_commit_whose_tree_stops_before_the_last_phase_fails_and_its_message_is_unjudged() {
    let history = History::new("commit-stopped-last");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // The new decision is defined in a file the walk cannot read, so against the table the
    // reference would dangle; the message is right, and the tree is what is wrong.
    history.write_bytes(
        "docs/new.md",
        b"# New\n\n### A new decision `##new-one`\n\ncaf\xe9\n",
    );
    let sha = history.commit("Adds `design@tiny@new-one`\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s); its message was judged \
             against nothing"
        )),
        "{stdout}"
    );
    assert!(
        !stdout.contains("is referenced and"),
        "no finding computed over the incomplete table: {stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: the range's last commit is judged like every other: its failing tree is a
/// finding naming the commit, and exit 1.
#[test]
fn a_last_commit_whose_tree_fails_is_a_finding_of_the_run() {
    let history = History::new("commit-head-fails");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // A live document naming a decision nothing defines: one `references` finding, in the
    // tree rather than in the message.
    history.write(
        "docs/note.md",
        "# A note\n\nIt names `design@tiny@no-such-decision`.\n",
    );
    let sha = history.commit("A note is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{sha} failed: its tree fails 1 finding(s)")),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {sha}: docs/note.md"))),
        "{stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: `commits` reads every checker directory it is given as data in each commit's
/// tree, per `design@core@checker-source-literals-are-data`, and not only the first.
///
/// The binary compiles its own directories in, and they are not inside a temporary history,
/// so this calls the library with directories inside it. Two tool directories each hold a
/// Rust source whose unbound string literal names a decision nothing defines, in a commit whose
/// successor removes them, so only that commit's tree can fail.
#[test]
fn commits_reads_every_checker_directory_as_data_in_each_commits_tree() {
    let history = History::new("commit-checker-dirs");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    for tool in ["tool-a", "tool-b"] {
        history.write(
            &format!("{tool}/src/lib.rs"),
            "pub fn f() {\n    println!(\"{}\", \"`design@tiny@no-such-decision`\");\n}\n",
        );
    }
    history.commit("Two tools are added\n");
    history.remove("tool-a/src/lib.rs");
    history.remove("tool-b/src/lib.rs");
    history.commit("The tools are removed\n");
    let manifest = knowledge_architect::Manifest::find(&history.dir).expect("the manifest");
    let range = format!("{base}..HEAD");
    let (a, b) = (history.dir.join("tool-a"), history.dir.join("tool-b"));
    let judge = |dirs: &[&Path]| {
        let command =
            knowledge_architect::cli::Command::Commits(knowledge_architect::cli::CommitsArgs {
                range: range.clone(),
            });
        knowledge_architect::cli::run(command, &manifest, dirs, &mut []).expect("the run completes")
    };
    // The literals are live where no directory covers them, so the fixture can fail.
    assert_eq!(judge(&[]), std::process::ExitCode::FAILURE);
    assert_eq!(judge(&[&a]), std::process::ExitCode::FAILURE);
    assert_eq!(judge(&[&a, &b]), std::process::ExitCode::SUCCESS);
}

/// The claim: a commit whose manifest fails phase 1 is read no further and serves as no
/// parent, per `design@core@a-commit-message-is-a-document`. The next commit's message then
/// resolves against its own tree alone, so naming an entry that only the failed tree held
/// dangles.
#[test]
fn a_tree_whose_phase_one_fails_serves_as_no_parent() {
    let history = History::new("commit-phase-one-parent");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let manifest =
        std::fs::read_to_string(history.dir.join("knowledge-architect.toml")).expect("manifest");
    history.write("knowledge-architect.toml", &format!("{manifest}\n[lint]\n"));
    let failed = history.commit("A table no extension claims\n");
    history.write("knowledge-architect.toml", &manifest);
    history.remove("docs/open-issues/a-closable-issue.md");
    // The escape names a path this tree holds, so a real parent refuses it as the next tree
    // does and the finding stands; an empty tree kept as a parent would accept it and drop it.
    let next =
        history.commit("Closes `issue@tiny@a-closable-issue`. See `path@elsewhere@README.md`.\n");
    history.commit("The last commit\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{failed} failed: its tree stops at phase 1")),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {next}:"))
                && l.contains("issue@tiny@a-closable-issue")),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {next}:"))
                && l.contains("path@elsewhere@README.md")),
        "{stdout}"
    );
}

/// The claim: where HEAD sits changes nothing. A HEAD checked out inside the range is judged
/// like any other commit: its failing tree is a finding naming it, and exit 1.
#[test]
fn a_head_inside_the_range_is_judged_like_any_other_commit() {
    let history = History::new("commit-head-inside");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/note.md",
        "# A note\n\nIt names `design@tiny@no-such-decision`.\n",
    );
    let planted = history.commit("A note is added\n");
    history.remove("docs/note.md");
    let last = history.commit("The note is removed\n");
    history.git(&["checkout", "-q", "--detach", &planted]);
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..{last}")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {planted}: docs/note.md"))),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{last} passed")), "{stdout}");
    let verdict = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(verdict.starts_with("FAILED"), "{verdict}");
}

/// The claim: a last commit whose tree cannot be assembled at all is a finding of the run, like
/// any other commit's, and the commits before it are still printed.
#[test]
fn a_last_commit_whose_manifest_does_not_load_is_a_finding_of_the_run() {
    let history = History::new("commit-last-unloadable");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let good = history.commit("A subject line\n\nIt records `design@tiny@tiny-anchor`.\n");
    tiny_project(&history, true);
    let bad = history.commit("The manifest goes back to a shape this tool refuses\n");
    // The working tree gets a manifest again: the binary locates its project by reading one,
    // so a tree whose manifest does not load refuses before any range is walked. What is under
    // test is the last commit's tree, which keeps it.
    tiny_project(&history, false);

    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{good} passed")), "{stdout}");
    assert!(
        stdout.contains(&format!("{bad} failed: its tree does not load")),
        "{stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: a `#` line a commit actually holds is judged like any other line.
///
/// Git's default cleanup for `-m` is whitespace-only, so such a line reaches the commit.
/// `commits` reads the message the commit holds and cleans nothing, or a `CR:` marker on such a
/// line would leave the regime in silence.
#[test]
fn a_hash_line_a_commit_holds_is_judged_and_not_cleaned_away() {
    let history = History::new("commit-hash-line");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let sha = history.commit("A subject line\n\n# A line naming `design@tiny@no-such-thing`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {sha}:3")), "{stdout}");
    assert!(stdout.contains("no-such-thing"), "{stdout}");
}

/// The claim: a project vendored under its repository is judged from its own paths.
///
/// Every git question a per-commit read asks is answered relative to the working directory —
/// `ls-tree` is scoped to it and names entries relative to it, and a blob is asked for as
/// `<sha>:./<path>`. Asked repository-relative instead, the listing carries a prefix the
/// manifest never declares, so every document falls out of the walk and every reference in a
/// message dangles while the working tree's own `check` passes.
#[test]
fn a_project_vendored_under_its_repository_is_judged_from_its_own_paths() {
    let history = History::at("commit-nested", "vendored");
    tiny_project(&history, false);
    // A file at the repository root, outside the project: nothing about it may reach the walk.
    std::fs::write(history.repo.join("outside.md"), "# Outside\n").expect("a file outside");
    let base = history.commit("The project is created\n");
    let sha = history.commit("A subject line\n\nIt records `design@tiny@tiny-anchor`.\n");

    let (clean, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 0, "the working tree is clean: {clean}{stderr}");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "and so is the commit's tree: {stdout}{stderr}");
    assert!(stdout.contains(&format!("{sha} passed")), "{stdout}");
    assert!(
        !stdout.contains("its own tree fails"),
        "the listing carried no repository prefix: {stdout}"
    );
}

#[test]
fn an_empty_range_is_a_pass_that_says_it_judged_nothing() {
    let history = History::new("commit-empty");
    tiny_project(&history, false);
    history.commit("The project is created\n");
    let (stdout, stderr, code) = history.run(&["commits", "HEAD..HEAD"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("no commit is in range"), "{stdout}");
    assert!(stdout.contains("PASSED: no findings"), "{stdout}");
}

#[test]
fn a_range_that_does_not_resolve_could_not_run() {
    let history = History::new("commit-bad-range");
    tiny_project(&history, false);
    history.commit("The project is created\n");
    let (stdout, stderr, code) = history.run(&["commits", "origin/nowhere..HEAD"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("does not resolve"), "{stderr}");
}

/// A reader that closes the pipe early ends the run quietly with exit 2, never with a panic
/// on stderr: `cargo klarch model | head` is how a citation is located, and the panic the
/// standard print macros raise on a broken pipe made that idiom print a backtrace hint.
#[test]
fn a_closed_stdout_ends_the_run_quietly_with_exit_2() {
    let (reader, writer) = std::io::pipe().expect("a pipe");
    drop(reader);
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .args(["model"])
        .current_dir(project("planted"))
        .stdout(writer)
        .output()
        .expect("the binary runs");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "stderr: {stderr}");
    assert!(
        !stderr.contains("panicked") && !stderr.contains("Broken pipe"),
        "the closed pipe must not be reported: {stderr}"
    );
}

/// The claim: a file in the installer's namespace that this version does not ship stops the check
/// in phase 2, and is not walked as a document; `install-agent-skills` removes it and leaves a
/// project's own skill, and the check then passes. Per `design@core@owned-namespace-check`.
/// Mutations: dropping the namespace from the walk's exclusions walks the stray file as a
/// document; dropping the unshipped branch of `check::agents` lets the first run pass.
#[test]
fn a_stray_file_in_the_installers_namespace_stops_the_check_and_the_install_removes_it() {
    let sandbox = Sandbox::seeded(
        "stray-installed",
        "minimal",
        &[
            (
                ".claude/agents/knowledge-architect-stray.md",
                "a reference `design@nowhere@nothing` that is read by nothing\n",
            ),
            (".claude/skills/project-own/SKILL.md", "# Project skill\n"),
        ],
    );
    sandbox.serve_claude();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    // The stray file, and each shipped file the copy does not hold yet.
    let expected = format!(
        "phase 2: {} finding(s)",
        knowledge_architect_agent_skills::FILES.len() + 1
    );
    assert!(out.contains(&expected), "{out}");
    assert!(
        out.contains(".claude/agents/knowledge-architect-stray.md  this file sits in the installer's namespace"),
        "{out}"
    );
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.contains("deleted  .claude/agents/knowledge-architect-stray.md"),
        "{out}"
    );
    assert!(!sandbox
        .path(".claude/agents/knowledge-architect-stray.md")
        .exists());
    assert!(sandbox.path(".claude/skills/project-own/SKILL.md").exists());
    // git still lists the deleted file until the deletion is staged, and the check says so.
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("the deletion is not staged"), "{out}");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// The claim: `commits` does not compare a commit's installed files, so a commit holding a file of
/// the installer's namespace that the running version does not ship is judged on its message and
/// its documents alone, while `check` on the working tree still reports it. Per
/// `design@core@owned-namespace-check`. Mutation: giving the per-commit inputs the installed set
/// fails the range.
#[test]
fn commits_does_not_compare_installed_files_and_check_does() {
    let history = History::new("commit-installed");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // The harness is declared after the base commit has generated the indexes: `index` refuses
    // while the shipped files are missing, so the indexes are not regenerated after this.
    let manifest = std::fs::read_to_string(history.dir.join("knowledge-architect.toml"))
        .expect("the manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest.replace("harness = []", "harness = [\"claude\"]"),
    );
    history.write(
        ".claude/agents/knowledge-architect-older.md",
        "an installed file of another version\n",
    );
    history.commit("An installed file of another version is committed\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let (stdout, _, code) = history.run(&["check"]);
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.contains("does not ship it"), "{stdout}");
}

/// The claim: under `harness = []` a component owes no CLAUDE.md, the namespace is walked like any
/// other path, and the install writes and deletes nothing. Mutations: requiring the compiled
/// document set regardless of the harness, or running the install without its harness guard,
/// each fail a case.
#[test]
fn no_harness_owes_no_claude_md_and_installs_nothing() {
    let sandbox = Sandbox::seeded(
        "no-harness",
        "minimal",
        &[(
            ".claude/agents/knowledge-architect-kept.md",
            "# A file the project keeps\n",
        )],
    );
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    assert!(manifest.contains("[agents]\nharness = []\n"), "{manifest}");
    std::fs::remove_file(sandbox.path("CLAUDE.md")).expect("the root CLAUDE.md is removed");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(out.contains("nothing was installed"), "{out}");
    assert!(sandbox
        .path(".claude/agents/knowledge-architect-kept.md")
        .exists());
}

/// The claim: the binary hands the check and the install the files this version ships. Before
/// the install, the check reports each one missing; the install writes each, rendered with the
/// project's command, here the default; then the check passes. Per
/// `design@core@owned-namespace-check`. Rendering with a declared command is a unit test of
/// `agents`. Mutations: handing `Gathered` an empty set instead of
/// `agents::shipped` lets the first check pass; handing the install an empty set writes nothing.
#[test]
fn the_check_and_the_install_receive_the_shipped_set() {
    let files = knowledge_architect_agent_skills::FILES;
    assert!(
        !files.is_empty(),
        "this version ships no file, so the test proves nothing"
    );
    let sandbox = Sandbox::new("shipped-set", "minimal");
    sandbox.serve_claude();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.contains(&format!("phase 2: {} finding(s)", files.len())),
        "{out}"
    );
    for (rel, _) in files {
        assert!(out.contains(rel), "{rel} is not reported missing:\n{out}");
    }
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    for (rel, template) in files {
        assert!(out.contains(&format!("wrote    {rel}")), "{out}");
        let installed = std::fs::read_to_string(sandbox.path(rel)).expect("an installed file");
        assert_eq!(
            installed,
            template
                .replace("{{command}}", "klarch")
                .replace("\r\n", "\n"),
            "{rel}"
        );
    }
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// Every file under `dir` with its bytes, `.git` left out.
fn tree_bytes(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    let mut out = std::collections::BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).expect("a readable directory") {
            let path = entry.expect("a directory entry").path();
            if path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
            } else {
                let bytes = std::fs::read(&path).expect("a readable file");
                out.insert(
                    path.strip_prefix(dir)
                        .expect("under the copy")
                        .to_path_buf(),
                    bytes,
                );
            }
        }
    }
    out
}

/// The claim: a writer refuses over an incomplete model. `index` over `unsound`, which stops at
/// phase 2, exits 2, says why, and writes nothing.
#[test]
fn index_over_an_incomplete_model_refuses_and_writes_nothing() {
    let sandbox = Sandbox::new("index-refuses", "unsound");
    let before = tree_bytes(&sandbox.dir);
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 2, "stdout:\n{out}\nstderr:\n{err}");
    assert!(err.contains("the model is incomplete"), "{err}");
    assert!(err.contains("Nothing was written."), "{err}");
    assert!(tree_bytes(&sandbox.dir) == before, "index changed the tree");
}

/// The claim: `--version` answers from any directory, project or not, with the version of the
/// package the binary was built from, so a session can ask which checker ran rather than read the
/// project's pin. Shown to fail before the flag existed: clap refused the argument and exited 2.
#[test]
fn the_binary_reports_its_version_from_anywhere() {
    let (out, err, code) = run_in(&std::env::temp_dir(), &["--version"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(
        out.trim(),
        format!("klarch {}", env!("CARGO_PKG_VERSION")),
        "{out}"
    );
}

/// The issue the `--fix` tests retitle, and the index that lists it.
const FIX_ISSUE: &str = "docs/open-issues/the-mock-has-one-issue.md";
const FIX_INDEX: &str = "docs/open-issues/index.md";

/// A copy of `minimal` whose issue is retitled, so its index is stale.
fn stale_index(tag: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "minimal");
    let issue = std::fs::read_to_string(sandbox.path(FIX_ISSUE)).expect("the mock's issue");
    sandbox.write(
        FIX_ISSUE,
        &issue.replace(
            "# The mock has one issue so the register is not empty",
            "# A retitled issue",
        ),
    );
    sandbox.stage();
    sandbox
}

/// The claim: `check --fix` rewrites a stale generated index, lists it, and the run then passes,
/// so an edit cycle is one command. Mutation checked: skipping the write of the generated list
/// leaves the index stale, and the run exits 1.
///
/// The claim: the stale index's finding names the repair and no cause, per
/// `design@core@finding-names-the-repair`. The action line is asserted whole, its line
/// break included, so a cause appended to it fails the test.
#[test]
fn check_fix_rewrites_a_stale_index_and_the_run_passes() {
    let sandbox = stale_index("fix-stale");
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("the generated file is out of date\n    → run `klarch index`\n"),
        "{out}"
    );
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: wrote {FIX_INDEX} (regenerated)")),
        "{out}"
    );
    assert!(out.trim_end().ends_with("PASSED: no findings"), "{out}");
    let index = std::fs::read_to_string(sandbox.path(FIX_INDEX)).expect("the index");
    assert!(index.contains("A retitled issue"), "{index}");
}

/// The claim: the stale index's action names the command the project declares, not the
/// default one, so a project that runs the checker through its own command is told that one.
#[test]
fn the_stale_index_action_names_the_declared_command() {
    let sandbox = stale_index("stale-command");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "name = \"minimal\"\n",
            "name = \"minimal\"\ncommand = \"probe klarch\"\n",
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains("the generated file is out of date\n    → run `probe klarch index`\n"),
        "{out}"
    );
}

/// The claim: a finding of phases 1 to 3 stops a `--fix` run before any generated file is
/// written, so no writer writes over an incomplete model. Mutation checked: removing the gate's
/// stop writes the index over the incomplete model.
#[test]
fn check_fix_writes_no_generated_file_over_an_incomplete_model() {
    let sandbox = stale_index("fix-incomplete");
    std::fs::remove_file(sandbox.path("docs/rejected-alternatives.md"))
        .expect("a required document is removed");
    sandbox.stage();
    let before = std::fs::read(sandbox.path(FIX_INDEX)).expect("the index");
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("phase 2:"), "{out}");
    assert!(!out.contains("fixed:"), "{out}");
    assert_eq!(
        std::fs::read(sandbox.path(FIX_INDEX)).expect("the index"),
        before
    );
}

/// The claim: with nothing to fix, `check --fix` prints exactly what `check` prints and exits as
/// it does. Mutation checked: an install run whenever the project serves a harness is still
/// silent here, but printing a header before the report fails the comparison.
#[test]
fn check_fix_with_nothing_to_fix_is_a_plain_check() {
    let sandbox = Sandbox::new("fix-nothing", "minimal");
    let plain = sandbox.run(&["check"]);
    let fixed = sandbox.run(&["check", "--fix"]);
    assert_eq!(plain.2, 0, "{}{}", plain.0, plain.1);
    assert_eq!(fixed, plain);
}

/// The claim: `check --fix` installs an installed file that differs from the shipped text, lists
/// it, and the run passes. Mutation checked: skipping the install when it is needed leaves the
/// phase-2 finding, and the run exits 1.
#[test]
fn check_fix_installs_a_differing_installed_file() {
    let sandbox = Sandbox::new("fix-installed", "minimal");
    sandbox.serve_claude();
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    let (rel, _) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    sandbox.write(rel, "an edit the install overwrites\n");
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: wrote {rel} (installed)")),
        "{out}"
    );
}

/// The claim: `--fix` belongs to `check` alone; `commits` judges history and refuses it, which
/// clap answers with exit 2.
#[test]
fn commits_takes_no_fix() {
    let (_, err, code) = run("minimal", &["commits", "--fix", "HEAD"]);
    assert_eq!(code, 2, "{err}");
}

/// A copy of `minimal` serving the `claude` harness, its shipped set installed and staged: a tree
/// the check passes.
fn harnessed(tag: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "minimal");
    sandbox.serve_claude();
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    sandbox.stage();
    sandbox
}

/// The claim: on a harnessed tree the check passes, `check --fix` touches nothing and prints what
/// `check` prints. Mutation checked: putting every listed shipped file in the repairs, whatever
/// its bytes, prints `fixed:` lines and fails the comparison.
#[test]
fn check_fix_on_a_clean_harnessed_tree_is_a_plain_check() {
    let sandbox = harnessed("fix-harnessed");
    let before = tree_bytes(&sandbox.path(".claude"));
    let plain = sandbox.run(&["check"]);
    assert_eq!(plain.2, 0, "{}{}", plain.0, plain.1);
    assert_eq!(sandbox.run(&["check", "--fix"]), plain);
    assert_eq!(tree_bytes(&sandbox.path(".claude")), before);
}

/// The claim: `check --fix` never deletes a file of the installer's namespace that git does not
/// list, such as an ignored editor swap file the check never reports. Mutation checked: judging
/// the namespace from the filesystem, as the install does, deletes it.
#[test]
fn check_fix_never_deletes_an_ignored_file_in_the_namespace() {
    let sandbox = harnessed("fix-ignored");
    sandbox.write(".gitignore", "*.swp\n");
    let swap = ".claude/skills/knowledge-architect-design/.SKILL.md.swp";
    sandbox.write(swap, "my unsaved edits\n");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains("fixed:"), "{out}");
    assert!(sandbox.path(swap).exists());
}

/// The claim: `check --fix` deletes an unshipped file git lists in the namespace and lists the
/// removal; the deletion is unstaged, so the run stops at phase 2, and after `git add` a second
/// run passes. This is the two-run upgrade the owner ruled. Mutation checked: dropping the
/// removal line fails it. Not counting a removal as a write survives here: it changes the exit
/// code only when a later write fails, which this case does not reach.
#[test]
fn check_fix_removes_an_unshipped_file_and_an_upgrade_takes_two_runs() {
    let sandbox = harnessed("fix-removed");
    let stray = ".claude/agents/knowledge-architect-retired.md";
    sandbox.write(stray, "a file an older version shipped\n");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: removed {stray} (installed)")),
        "{out}"
    );
    assert!(out.contains("the deletion is not staged"), "{out}");
    assert!(!sandbox.path(stray).exists());
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: when a write fails after another file was written, `check --fix` lists what it
/// wrote and exits 1, since 2 would promise an untouched tree. Mutations checked: exiting 2
/// whatever was written, or dropping the error, each fail it.
#[test]
fn check_fix_exits_one_when_a_write_fails_after_a_write() {
    let sandbox = Sandbox::new("fix-partway", "minimal");
    sandbox.serve_claude();
    let (last, _) = knowledge_architect_agent_skills::FILES
        .last()
        .expect("this version ships a file");
    std::fs::create_dir_all(sandbox.path(last)).expect("a directory where the last file goes");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("fixed: wrote"), "{out}");
    assert!(err.contains(last), "{err}");
}

/// The claim: a generated destination that is a symlink is refused before anything is written,
/// and with nothing written the run exits 2. Mutations checked: dropping the symlink refusal
/// writes through the link, outside the project; exiting 1 whatever was written fails the code.
#[test]
fn check_fix_refuses_a_symlinked_destination_and_writes_nothing() {
    let sandbox = stale_index("fix-symlink");
    let outside = std::env::temp_dir().join(format!("ka-fix-outside-{}", std::process::id()));
    std::fs::write(&outside, "outside the project\n").expect("a file outside the project");
    std::fs::remove_file(sandbox.path(FIX_INDEX)).expect("the index is removed");
    std::os::unix::fs::symlink(&outside, sandbox.path(FIX_INDEX)).expect("a symlinked index");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            &format!("skip-files = [\"notes/generated.md\", \"{FIX_INDEX}\"]"),
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(err.contains("symlink"), "{err}");
    assert_eq!(
        std::fs::read_to_string(&outside).expect("the outside file"),
        "outside the project\n"
    );
    let _ = std::fs::remove_file(&outside);
}

/// The claim: a manifest holding a refused declaration makes `check --fix` write nothing, not even
/// the agent files, since the shipped text is rendered with the declared command. Mutation
/// checked: removing the early return installs the files.
#[test]
fn check_fix_over_a_refused_manifest_writes_nothing() {
    let sandbox = Sandbox::new("fix-refused", "minimal");
    sandbox.serve_claude();
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &format!("{manifest}\n[bogus]\nkey = 1\n"),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(!out.contains("fixed:"), "{out}");
    assert!(!sandbox
        .path(".claude/knowledge-architect/PRIMER.md")
        .exists());
}

/// The claim: a finding in a file git does not track is followed by one note naming the file
/// as untracked, above the verdict; a finding in a tracked file brings no note.
#[test]
fn a_finding_in_an_untracked_file_is_followed_by_a_note_naming_it() {
    // Each finding is an unanchored path into `docs/`, a directory the mock holds.
    let sandbox = Sandbox::seeded("untracked-note", "core", &[("tracked.md", "`docs/a`\n")]);
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("tracked.md:1"), "{out}");
    assert!(!out.contains("untracked:"), "{out}");

    // Written after the copy was staged, so git does not track it.
    sandbox.write("scratch.md", "`docs/c`\n");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("scratch.md:1"), "{out}");
    let lines: Vec<&str> = out.lines().collect();
    let note = lines
        .iter()
        .position(|l| l.contains("untracked:"))
        .expect("a note naming the untracked file");
    assert_eq!(
        lines[note], "1 file above is untracked: scratch.md",
        "{out}"
    );
    assert!(
        lines[note + 1].contains("commit a file that belongs to the project"),
        "{out}"
    );
    assert!(
        lines.last().expect("a verdict").starts_with("FAILED:"),
        "{out}"
    );

    // Two findings in one file name it once, and a second file makes the count plural.
    sandbox.write("scratch.md", "`docs/c`\n`docs/e`\n");
    sandbox.write("other.md", "`docs/g`\n");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l == "2 files above are untracked: other.md, scratch.md"),
        "{out}"
    );
}

/// The claim: a finding with no line, from a run stopped at phase 2, is noted like any other,
/// and a line break in the untracked name is escaped, so the note stays one line.
#[test]
fn the_untracked_note_covers_a_phase_two_stop_and_escapes_the_name() {
    let sandbox = Sandbox::new("untracked-note-phase-two", "core");
    sandbox.write("nl\nname.md", "text\n");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("phase 2:"), "{out}");
    assert!(
        out.lines()
            .any(|l| l == "1 file above is untracked: nl\\nname.md"),
        "{out}"
    );
}

/// The claim: an untracked manifest brings no note, since it is read whatever git says of it
/// and neither moving it out nor ignoring it is a way out.
#[test]
fn an_untracked_manifest_brings_no_untracked_note() {
    let sandbox = Sandbox::new("untracked-note-manifest", "core");
    sandbox.git(&["rm", "-q", "--cached", "knowledge-architect.toml"]);
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &format!("{manifest}\n[bogus]\n"),
    );
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("knowledge-architect.toml"), "{out}");
    assert!(!out.contains("untracked:"), "{out}");
}

/// Write a copy's manifest with `[project] checker-version` set to `value` exactly, bypassing
/// the pinning every other write of a manifest goes through.
fn pin_copy_to(sandbox: &Sandbox, value: &str) {
    let path = sandbox.path("knowledge-architect.toml");
    let text = std::fs::read_to_string(&path).expect("the manifest");
    let line = format!("checker-version = \"{}\"", env!("CARGO_PKG_VERSION"));
    assert_eq!(text.matches(&line).count(), 1, "{text}");
    let replaced = if value.is_empty() {
        text.replace(&format!("{line}\n"), "")
    } else {
        text.replace(&line, &format!("checker-version = \"{value}\""))
    };
    std::fs::write(&path, replaced).expect("the manifest is rewritten");
}

/// The claim: over a copy of a mock, a pin the binary does not satisfy refuses the run with exit
/// 2 and a message opening with the case: the binary older or newer than the pin, `"fixture"`
/// outside a library, `"self"` where the checker is not built; the pin equal to the binary's
/// version passes. Per `design@core@installed-binary-version-check`.
#[test]
fn a_copy_whose_pin_this_binary_does_not_satisfy_refuses_the_run() {
    let version = env!("CARGO_PKG_VERSION");
    let sandbox = Sandbox::new("pin-refusal", "minimal");
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "the pinned copy passes: {out}{err}");
    for (value, opening) in [
        (
            "99.0.0".to_string(),
            format!("this binary runs knowledge-architect {version}, older than the 99.0.0 the manifest pins"),
        ),
        (
            "0.0.1".to_string(),
            format!("this binary runs knowledge-architect {version}, newer than the 0.0.1 the manifest pins"),
        ),
        (
            "fixture".to_string(),
            "\"fixture\" is only valid for a mock project inside a library this binary links".to_string(),
        ),
        (
            "self".to_string(),
            "\"self\" is only valid where the checker is built from this tree".to_string(),
        ),
    ] {
        let copy = Sandbox::new(&format!("pin-refusal-{value}"), "minimal");
        pin_copy_to(&copy, &value);
        let (out, err, code) = copy.run(&["check"]);
        assert_eq!(code, 2, "{value}: {out}{err}");
        assert!(err.contains(&opening), "{value}: {err}");
    }
}

/// The claim: with no key, every command refuses with the key's complaint, a command that reads
/// nothing included.
#[test]
fn a_manifest_without_the_key_refuses_every_command() {
    let sandbox = Sandbox::new("pin-absent", "minimal");
    pin_copy_to(&sandbox, "");
    for args in [
        &["check"][..],
        &["show", "design@minimal@nothing"],
        &["model"],
    ] {
        let (out, err, code) = sandbox.run(args);
        assert_eq!(code, 2, "{args:?}: {out}{err}");
        assert!(
            err.contains("[project] checker-version is absent"),
            "{args:?}: {err}"
        );
    }
}

/// The claim: a mock project in place, inside the core's directory, passes with `"fixture"`.
#[test]
fn a_mock_in_place_passes_with_fixture() {
    let (out, err, code) = run("dirhome", &["check"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: the install over a project pinning another version refuses, and writes nothing.
#[test]
fn the_install_refuses_another_version_and_writes_nothing() {
    let sandbox = Sandbox::new("pin-install", "minimal");
    sandbox.serve_claude();
    pin_copy_to(&sandbox, "0.0.1");
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        err.contains("newer than the 0.0.1 the manifest pins"),
        "{err}"
    );
    assert!(!sandbox.path(".claude").exists(), "nothing was installed");
}

/// The claim: `commits` judges a range whose earlier commit pins another version without
/// comparing that historical pin, so moving the pin fails no earlier commit.
#[test]
fn commits_compares_no_historical_pin() {
    let history = History::new("pin-history");
    tiny_project(&history, false);
    // The base is pinned to this binary, so `commit` regenerates its indexes; the next commit
    // changes the pin alone, which `commit` cannot regenerate over, and needs not.
    let base = history.commit("The project is created\n");
    let manifest = history.dir.join("knowledge-architect.toml");
    let pinned_text = std::fs::read_to_string(&manifest).expect("the manifest");
    let line = format!("checker-version = \"{}\"", env!("CARGO_PKG_VERSION"));
    std::fs::write(
        &manifest,
        pinned_text.replace(&line, "checker-version = \"0.0.1\""),
    )
    .expect("an older pin");
    history.commit("A subject line\n\nThe project pins an older checker.\n");
    std::fs::write(&manifest, &pinned_text).expect("the pin moved");
    history.commit("The pin moves\n\nThe project now pins this checker.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("2 commits, 2 passed, 0 failed"), "{stdout}");
}

/// The claim: the foreign-build refusal comes before the pin's, so over a second checkout whose
/// manifest pins another version, the message names the checkout, not the version.
#[test]
fn a_foreign_build_is_refused_before_its_pin_is_read() {
    let sandbox = Sandbox::seeded(
        "foreign-before-pin",
        "minimal",
        &[(
            "crates/core/Cargo.toml",
            "[package]\nname = \"knowledge-architect\"\n",
        )],
    );
    pin_copy_to(&sandbox, "0.0.1");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(stderr.contains("built from another checkout"), "{stderr}");
    assert!(!stderr.contains("the manifest pins"), "{stderr}");
}
