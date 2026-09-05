//! Every git invocation the documentation half makes, in one place.
//!
//! **Git is the walk.** What this tool reads is what `git ls-files` reports from the manifest's
//! directory, so a nested `.gitignore` is honoured, every pattern git honours is honoured, and a
//! file git tracks can no longer leave the walk. The hand-rolled matcher this replaced could do
//! none of the three, per `knowledge#git-supplies-the-walk`.
//!
//! **One module owns the process boundary.** A check is a pure function over the model and may
//! spawn nothing, so every call here is made by the caller that builds the model or prints a
//! listing. Keeping them together is what makes the set of things this tool asks git auditable
//! in one read.
//!
//! **A git failure is exit 2 with the reason, never an empty walk.** No `git` on the path and a
//! directory that is not inside a worktree both produce an error naming what was run and where.
//! Silence would report a project with no documents as a clean one, which is the failure class
//! this tool exists to prevent.
//!
//! Every invocation runs with the project root as its working directory and asks for `-z`
//! output, so the paths that come back are project-relative and hold no escaping.

use std::collections::{BTreeMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// One git invocation, built up and run.
///
/// A builder rather than a function per call so that a new question — a revision list, a tree
/// listing, a commit message — is one more method chain here rather than one more place that
/// knows how to spawn a process and how to read its failure.
pub struct Invocation {
    root: PathBuf,
    args: Vec<OsString>,
    stdin: Option<Vec<u8>>,
    /// Exit codes that are answers rather than failures. `git check-ignore` exits 1 to say
    /// "none of these", which is a result and not an error.
    accept: Vec<i32>,
}

/// A git invocation rooted at `root`, with nothing asked yet.
pub fn git(root: &Path) -> Invocation {
    Invocation {
        root: root.to_path_buf(),
        args: Vec::new(),
        stdin: None,
        accept: vec![0],
    }
}

impl Invocation {
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|a| a.as_ref().to_os_string()));
        self
    }

    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    pub fn stdin(mut self, bytes: Vec<u8>) -> Self {
        self.stdin = Some(bytes);
        self
    }

    /// Also treat this exit code as an answer.
    pub fn accept(mut self, code: i32) -> Self {
        self.accept.push(code);
        self
    }

    /// Run it, and give back stdout.
    ///
    /// The two failures a caller must be able to tell apart from an empty answer are named in
    /// the error: git is not there, or git refused. Both reach the binary as exit 2.
    pub fn output(self) -> io::Result<Vec<u8>> {
        let shown = self.shown();
        let mut command = Command::new("git");
        command
            .current_dir(&self.root)
            // **The per-user ignore file is pinned away.** `core.excludesFile` lives in the
            // developer's home and is no part of the project, so honouring it would make the
            // walked set a property of the machine rather than of the commit: a line there
            // takes an untracked live document out of every check on one clone and not on
            // another. What still decides the walk is the tree's own ignore files and git's
            // per-clone exclude file, which git offers no way to pin; the tripwire in
            // `knowledge@docs/tripwires.md` guards what remains.
            .args(["-c", "core.excludesFile=/dev/null"])
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(if self.stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            });
        let mut child = command.spawn().map_err(|e| {
            // A missing working directory reports NotFound as readily as a missing binary
            // does, so the two are told apart before the message is written: "git is not on
            // the PATH" sent after a mistyped root is a message that costs a session.
            if e.kind() == io::ErrorKind::NotFound && self.root.is_dir() {
                io::Error::other(format!(
                    "git is not on the PATH, and this tool reads the tree through git: \
                     `{shown}` in {} could not start. Install git, or run where it is reachable.",
                    self.root.display()
                ))
            } else {
                io::Error::other(format!(
                    "`{shown}` could not start in {}: {e}",
                    self.root.display()
                ))
            }
        })?;
        // **The input is written by a thread, and the parent goes straight to reading.** Both
        // pipes are 64KB deep, and `check-ignore` answers while it reads: over a batch whose
        // answers exceed that, git blocks writing its output while a parent still writing its
        // input blocks too, and neither ever moves. Measured at 50 000 queries, which is one
        // reference per line of a large document set. The pipe is dropped when the thread ends,
        // which is what lets git finish reading and exit.
        let writer = self.stdin.map(|bytes| {
            let mut pipe = child.stdin.take().expect("a piped stdin");
            std::thread::spawn(move || {
                use std::io::Write;
                match pipe.write_all(&bytes) {
                    // git stopped reading and is on its way out; its exit code is the answer.
                    Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
                    other => other,
                }
            })
        });
        let out = child.wait_with_output()?;
        if let Some(writer) = writer {
            writer
                .join()
                .map_err(|_| io::Error::other(format!("`{shown}`: the input writer panicked")))??;
        }
        let code = out.status.code().unwrap_or(-1);
        if !self.accept.contains(&code) {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            let reason = if stderr.is_empty() {
                format!("exit status {code}")
            } else {
                stderr
            };
            return Err(io::Error::other(format!(
                "`{shown}` failed in {}: {reason}",
                self.root.display()
            )));
        }
        Ok(out.stdout)
    }

    /// The same, with the output read as a NUL-separated list of project-relative paths.
    pub fn paths(self) -> io::Result<Vec<PathBuf>> {
        Ok(nul_paths(&self.output()?))
    }

    /// The command as an error message names it.
    ///
    /// The pinned configuration is left out: it is the same on every invocation and naming it
    /// in every message would bury the subcommand that failed.
    fn shown(&self) -> String {
        let mut out = String::from("git");
        for arg in &self.args {
            out.push(' ');
            out.push_str(&arg.to_string_lossy());
        }
        out
    }
}

/// Split `-z` output on NUL, dropping the empty tail the last separator leaves.
fn nul_separated(bytes: &[u8]) -> Vec<&[u8]> {
    bytes.split(|b| *b == 0).filter(|s| !s.is_empty()).collect()
}

/// The same, read as project-relative paths.
///
/// **A path is bytes, not text.** A filename holding a byte sequence that is not UTF-8 is
/// legal on this platform and git reports it as it is under `-z`; decoding it lossily would
/// substitute a replacement character and produce a path nothing on disk answers to. The file
/// would then read as unreadable — or, where nothing reports that, leave every check while the
/// run stayed green, which is the failure `knowledge#git-supplies-the-walk` exists against.
fn nul_paths(bytes: &[u8]) -> Vec<PathBuf> {
    nul_separated(bytes).into_iter().map(as_path).collect()
}

#[cfg(unix)]
fn as_path(bytes: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    PathBuf::from(OsStr::from_bytes(bytes))
}

/// Everywhere else the platform has no bytes-to-path conversion, and a name it cannot spell is
/// a name git could not have produced there.
#[cfg(not(unix))]
fn as_path(bytes: &[u8]) -> PathBuf {
    PathBuf::from(String::from_utf8_lossy(bytes).into_owned())
}

/// The same, read as text. Used only for `check-ignore`'s echo of spellings this tool wrote,
/// which came from documents and are therefore UTF-8 by construction.
fn nul_strings(bytes: &[u8]) -> Vec<String> {
    nul_separated(bytes)
        .into_iter()
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .collect()
}

/// Every live file of the project: what git tracks, plus what it does not track and does not
/// ignore.
///
/// This is the walk's source. `--cached` is unaffected by the ignore rules, which is why a
/// tracked file cannot leave the walk however the ignore rules are written; `--exclude-standard`
/// applies them to `--others` alone, which is where build output and on-demand directories are
/// dropped. Every remaining exclusion is the manifest's, and `walk::live_files` applies it.
pub fn live_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    git(root)
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ])
        .paths()
}

/// The files git both tracks and ignores.
///
/// Each is a finding, because the two states contradict each other and the contradiction is
/// silent. The file is read by the walk, since `--cached` ignores the ignore rules; and it is
/// asserted like any other target, since `git check-ignore` skips paths the index holds. So a
/// reader of the ignore rules concludes the file is out of the project while every check reads
/// it, and deleting the file from the index flips both answers at once. The repair is to untrack
/// the file, or to narrow the rule that covers it.
pub fn tracked_and_ignored(root: &Path) -> io::Result<Vec<PathBuf>> {
    git(root)
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--ignored",
            "--exclude-standard",
        ])
        .paths()
}

/// How one path target is spelled when git is asked whether the ignore rules cover it.
///
/// The trailing slash is the reference's own kind claim, and git needs it: a `dir/` pattern
/// matches a path git can tell is a directory, and a target that does not exist yet — which is
/// the whole point of exempting generated paths — is a directory only if the spelling says so.
/// Asking by the claim rather than by what is on disk is what makes the verdict identical on a
/// fresh clone and a built tree, per `knowledge#ignored-targets-are-not-asserted`.
pub fn ignore_query(target: &Path, claims_dir: bool) -> String {
    let mut out = target.to_string_lossy().into_owned();
    if claims_dir && !out.ends_with('/') {
        out.push('/');
    }
    out
}

/// Which of these spellings the ignore rules cover, in one batch.
///
/// One process per run rather than one per reference: a project with a thousand path references
/// would otherwise spawn a thousand. `check-ignore` exits 1 when nothing matched, which is an
/// answer, and the empty input is answered without spawning anything at all.
pub fn ignored(root: &Path, queries: &[String]) -> io::Result<HashSet<String>> {
    let wanted: Vec<&String> = queries.iter().filter(|q| !q.is_empty()).collect();
    if wanted.is_empty() {
        return Ok(HashSet::new());
    }
    let mut stdin = Vec::new();
    for query in &wanted {
        stdin.extend_from_slice(query.as_bytes());
        stdin.push(0);
    }
    let out = git(root)
        .args(["check-ignore", "-z", "--stdin"])
        // Nothing matched. A run whose every reference points at a tracked path takes this
        // arm, so reading it as a failure would fail the commonest tree there is.
        .accept(1)
        .stdin(stdin)
        .output()?;
    Ok(nul_strings(&out).into_iter().collect())
}

/// When each file under `dirs` last changed, as `git` reports it, keyed by project-relative
/// path.
///
/// **One process for the whole listing**: a `git log` per entry costs a process per row. An
/// empty map is what a tree with no history produces, and the caller prints a placeholder
/// rather than failing — this column is convenience, and the hard git dependency belongs to the
/// walk rather than to a listing.
pub fn last_changed(root: &Path, dirs: &[PathBuf]) -> BTreeMap<PathBuf, String> {
    if dirs.is_empty() {
        return BTreeMap::new();
    }
    let invocation = git(root)
        // `--relative` because the names are matched against project-relative paths, and a
        // project that is a subdirectory of its repository would otherwise get
        // repository-relative ones back and match nothing at all.
        .args(["log", "--format=%cs", "--name-only", "--relative", "--"])
        .args(dirs);
    let Ok(out) = invocation.output() else {
        return BTreeMap::new();
    };
    parse_log(&String::from_utf8_lossy(&out))
}

/// The log as a map from path to the date of its newest commit.
///
/// The two line kinds are told apart by shape: `%cs` is a bare `YYYY-MM-DD`, which no path can
/// be, so the log needs no second format field and no separator to parse.
fn parse_log(text: &str) -> BTreeMap<PathBuf, String> {
    let mut out = BTreeMap::new();
    let mut date: Option<&str> = None;
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        if is_date(line) {
            date = Some(line);
            continue;
        }
        let Some(date) = date else { continue };
        // The log is newest first, so the first mention of a path is its last change.
        out.entry(PathBuf::from(line))
            .or_insert_with(|| date.to_string());
    }
    out
}

fn is_date(line: &str) -> bool {
    let bytes = line.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_nul_list_drops_the_trailing_separator_and_keeps_every_name() {
        // `-z` terminates each entry, so the last byte is a separator and a naive split leaves
        // an empty name that resolves to the project root itself.
        assert_eq!(
            nul_paths(b"a.md\0b/c.rs\0"),
            vec![PathBuf::from("a.md"), PathBuf::from("b/c.rs")]
        );
        assert_eq!(nul_paths(b""), Vec::<PathBuf>::new());
        assert_eq!(nul_paths(b"\0"), Vec::<PathBuf>::new());
    }

    #[test]
    fn a_name_that_is_not_utf_eight_is_carried_as_bytes() {
        // A filename holding a byte no UTF-8 decoding accepts is legal here and git reports it
        // verbatim under `-z`. Decoded lossily it becomes a path nothing on disk answers to,
        // and the file leaves every check while the run stays green.
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let found = nul_paths(b"notes/re\xffadme.txt\0plain.md\0");
            assert_eq!(found.len(), 2);
            assert_eq!(
                found[0].as_os_str().as_bytes(),
                b"notes/re\xffadme.txt",
                "the byte survives the round trip"
            );
            assert_eq!(found[1], PathBuf::from("plain.md"));
        }
    }

    #[test]
    fn a_directory_claim_is_asked_with_the_trailing_slash_and_a_file_claim_without() {
        // git decides a `dir/` pattern on the spelling when the path is not on disk, and a
        // generated path never is on a fresh clone.
        assert_eq!(ignore_query(Path::new("target"), true), "target/");
        assert_eq!(ignore_query(Path::new("target"), false), "target");
        assert_eq!(ignore_query(Path::new("a/b.txt"), false), "a/b.txt");
        // Idempotent: a target that already carries the slash gains no second one.
        assert_eq!(ignore_query(Path::new("a/b/"), true), "a/b/");
    }

    #[test]
    fn a_directory_outside_a_worktree_is_an_error_carrying_gits_own_reason() {
        let dir = std::env::temp_dir().join(format!(
            "knowledge-git-nonrepo-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a temporary directory");
        // A temporary directory is not inside any worktree, so git refuses. The message is
        // git's own, because a reworded one goes stale against the version installed.
        let e = live_files(&dir).expect_err("not a worktree").to_string();
        assert!(e.contains("ls-files"), "{e}");
        assert!(e.to_lowercase().contains("not a git repository"), "{e}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_empty_batch_asks_git_nothing_and_answers_nothing() {
        // No path reference in the tree means no process, and it must not mean a failure
        // either: this runs in a directory git may refuse.
        let dir = std::env::temp_dir().join(format!("knowledge-git-empty-{}", std::process::id()));
        assert_eq!(
            ignored(&dir, &[]).expect("no process, no failure"),
            HashSet::new()
        );
        assert_eq!(
            ignored(&dir, &[String::new()]).expect("an empty spelling asks nothing"),
            HashSet::new()
        );
    }

    #[test]
    fn a_batch_larger_than_a_pipe_buffer_does_not_deadlock() {
        // Both pipes hold 64KB and `check-ignore` answers while it reads, so a parent that
        // writes the whole input before reading a byte of output stops moving as soon as the
        // answers pass that. Before the input was written by its own thread, this hung
        // forever; the recv timeout is what turns a hang into a failing test.
        let dir = std::env::temp_dir().join(format!("knowledge-git-batch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a temporary directory");
        assert!(git(&dir).args(["init", "-q"]).output().is_ok_and(|_| true));
        std::fs::write(dir.join(".gitignore"), "*.gen\n").expect("an ignore rule");
        let count = 50_000;
        let queries: Vec<String> = (0..count).map(|i| format!("f{i}.gen")).collect();
        let (tx, rx) = std::sync::mpsc::channel();
        let probe = dir.clone();
        std::thread::spawn(move || {
            let _ = tx.send(ignored(&probe, &queries).map(|s| s.len()));
        });
        match rx.recv_timeout(std::time::Duration::from_secs(60)) {
            Ok(Ok(found)) => assert_eq!(found, count),
            Ok(Err(e)) => panic!("{e}"),
            Err(_) => panic!("the batch never returned"),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_log_names_paths_relative_to_the_project_and_not_to_the_repository() {
        // A project vendored as a subdirectory of its repository is the case: without
        // `--relative` git answers with repository-relative names, which match no
        // project-relative key and leave every row's last-change column empty.
        let repo = std::env::temp_dir().join(format!("knowledge-git-log-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        let project = repo.join("nested");
        std::fs::create_dir_all(project.join("notes")).expect("a nested project");
        std::fs::write(project.join("notes/a.md"), "# A\n").expect("a document");
        git(&repo)
            .args(["init", "-q"])
            .output()
            .expect("a repository");
        git(&repo)
            .args(["add", "-A"])
            .output()
            .expect("the document staged");
        git(&repo)
            .args([
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "one",
            ])
            .output()
            .expect("a commit, which a log needs");

        let found = last_changed(&project, &[PathBuf::from("notes")]);
        assert_eq!(
            found.keys().collect::<Vec<_>>(),
            vec![&PathBuf::from("notes/a.md")],
            "project-relative, not `nested/notes/a.md`"
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    #[test]
    fn the_log_maps_each_path_to_its_newest_commit_and_ignores_what_precedes_a_date() {
        // Newest first, so the first mention of a path wins; a name before any date belongs to
        // no commit and is dropped rather than attributed to the next one.
        let log = "docs/orphan.md\n2026-01-02\n\ndocs/a.md\ndocs/b.md\n\n\
                   2026-01-01\n\ndocs/a.md\ndocs/c.md\n";
        let found = parse_log(log);
        assert_eq!(
            found.get(Path::new("docs/a.md")).map(String::as_str),
            Some("2026-01-02")
        );
        assert_eq!(
            found.get(Path::new("docs/b.md")).map(String::as_str),
            Some("2026-01-02")
        );
        assert_eq!(
            found.get(Path::new("docs/c.md")).map(String::as_str),
            Some("2026-01-01")
        );
        assert_eq!(found.get(Path::new("docs/orphan.md")), None);
        assert!(parse_log("").is_empty());
    }

    #[test]
    fn a_date_is_told_from_a_path_by_its_shape_alone() {
        assert!(is_date("2026-01-02"));
        assert!(!is_date("2026-1-2"));
        assert!(!is_date("docs/a.md"));
        assert!(!is_date("2026-01-022"));
        assert!(!is_date("abcd-ef-gh"));
    }
}
