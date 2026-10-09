//! Every git invocation the core makes, in one place.
//!
//! **Git is the walk.** What this tool reads is what `git ls-files` reports from the manifest's
//! directory, so a nested `.gitignore` is honoured, every pattern git honours is honoured, and no
//! ignore rule can take a file git tracks out of the walk. The hand-rolled matcher this replaced could do
//! none of the three, per `design@core@git-supplies-the-walk`.
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
pub(crate) struct Invocation {
    root: PathBuf,
    args: Vec<OsString>,
    stdin: Option<Vec<u8>>,
    /// Exit codes that are answers rather than failures. `git check-ignore` exits 1 to say
    /// "none of these", which is a result and not an error.
    accept: Vec<i32>,
}

/// A git invocation rooted at `root`, with nothing asked yet.
pub(crate) fn git(root: &Path) -> Invocation {
    Invocation {
        root: root.to_path_buf(),
        args: Vec::new(),
        stdin: None,
        accept: vec![0],
    }
}

impl Invocation {
    pub(crate) fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|a| a.as_ref().to_os_string()));
        self
    }

    pub(crate) fn stdin(mut self, bytes: Vec<u8>) -> Self {
        self.stdin = Some(bytes);
        self
    }

    /// Also treat this exit code as an answer.
    pub(crate) fn accept(mut self, code: i32) -> Self {
        self.accept.push(code);
        self
    }

    /// Run it, and give back stdout.
    ///
    /// The two failures a caller must be able to tell apart from an empty answer are named in
    /// the error: git is not there, or git refused. Both reach the binary as exit 2.
    pub(crate) fn output(self) -> io::Result<Vec<u8>> {
        self.run().map(|(_, stdout)| stdout)
    }

    /// Run it, and give back the exit code, which must be one [`Invocation::accept`] names.
    pub(crate) fn code(self) -> io::Result<i32> {
        self.run().map(|(code, _)| code)
    }

    fn run(self) -> io::Result<(i32, Vec<u8>)> {
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
            // `path@core@docs/tripwires.md` guards what remains.
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
        Ok((code, out.stdout))
    }

    /// The same, with the output read as a NUL-separated list of project-relative paths.
    pub(crate) fn paths(self) -> io::Result<Vec<PathBuf>> {
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
/// run stayed green, which is the failure `design@core@git-supplies-the-walk` exists against.
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

/// What an entry of a listing is, by the mode git records for it.
///
/// A symlink is mode `120000` and a submodule's gitlink `160000`, in the index and in every
/// commit's tree alike, so the two readers of a tree classify an entry the same way before
/// either reads a byte. Neither is a document: a symlink read through the filesystem yields
/// its target's bytes and read from a tree yields the target's path as text, and a checkout
/// without symlinks holds that text as a plain file; a gitlink names a commit of another
/// repository, under which the listing never descends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Symlink,
    Gitlink,
}

/// One entry of a listing: its project-relative path and what it is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub rel: PathBuf,
    pub kind: EntryKind,
}

/// Every entry of the live listing with its kind: what git tracks, by the mode the index
/// records, plus what it neither tracks nor ignores, by what the working tree holds.
///
/// An untracked entry has no index mode, so its kind is read off the filesystem without
/// following it; a tracked one is classified by the index, which is what a checkout that
/// could not create the symlink still records.
///
/// This is the walk's source. `--cached` is unaffected by the ignore rules, which is why a
/// tracked file cannot leave the walk however the ignore rules are written; `--exclude-standard`
/// applies them to `--others` alone, which is where build output and on-demand directories are
/// dropped. Every remaining exclusion is the manifest's, and `walk::live_files` applies it.
///
/// The second list is the untracked part, `--others`, so a report can say which of its files
/// git does not track. A file staged and never committed is tracked, and is not in it.
pub(crate) fn entries(root: &Path) -> io::Result<(Vec<Entry>, Vec<PathBuf>)> {
    let staged = git(root)
        .args(["ls-files", "-z", "-s", "--cached"])
        .output()?;
    let mut out: Vec<Entry> = nul_separated(&staged)
        .into_iter()
        .filter_map(mode_and_path)
        .collect();
    let others = git(root)
        .args(["ls-files", "-z", "--others", "--exclude-standard"])
        .paths()?;
    for rel in &others {
        let kind = match std::fs::symlink_metadata(root.join(rel)) {
            Ok(meta) if meta.file_type().is_symlink() => EntryKind::Symlink,
            _ => EntryKind::File,
        };
        out.push(Entry {
            rel: rel.clone(),
            kind,
        });
    }
    out.sort_by(|a, b| a.rel.cmp(&b.rel));
    Ok((out, others))
}

/// One line of `ls-files -s` or `ls-tree -r`: the mode is the first field, the path follows
/// the first tab, and the fields between differ between the two and are read by nothing. The
/// first tab, because the fields hold none and the path may.
fn mode_and_path(line: &[u8]) -> Option<Entry> {
    let space = line.iter().position(|b| *b == b' ')?;
    let tab = line.iter().position(|b| *b == b'\t')?;
    let kind = match &line[..space] {
        b"120000" => EntryKind::Symlink,
        b"160000" => EntryKind::Gitlink,
        _ => EntryKind::File,
    };
    Some(Entry {
        rel: as_path(&line[tab + 1..]),
        kind,
    })
}

/// The files git both tracks and ignores.
///
/// Each is a finding, because the two states contradict each other and the contradiction is
/// silent. The file is read by the walk, since `--cached` ignores the ignore rules; and it is
/// asserted like any other target, since `git check-ignore` skips paths the index holds. So a
/// reader of the ignore rules concludes the file is out of the project while every check reads
/// it, and deleting the file from the index flips both answers at once. The repair is to untrack
/// the file, or to narrow the rule that covers it.
pub(crate) fn tracked_and_ignored(root: &Path) -> io::Result<Vec<PathBuf>> {
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
/// fresh clone and a built tree, per `design@core@ignored-targets-are-not-asserted`.
pub(crate) fn ignore_query(target: &Path, claims_dir: bool) -> String {
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
pub(crate) fn ignored(root: &Path, queries: &[String]) -> io::Result<HashSet<String>> {
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
pub(crate) fn last_changed(root: &Path, dirs: &[PathBuf]) -> BTreeMap<PathBuf, String> {
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

/// The commits a range names, oldest first.
///
/// **Topological order, not date order.** `--reverse` alone reverses git's default
/// reverse-chronological listing, and a commit whose recorded date precedes its parent's — a
/// rebase across a clock skew produces one — then lands before the commit it descends from.
/// The per-commit check reuses the previous model as the next commit's parent tree, so an
/// order that is not the parent chain would resolve a message against the wrong tree.
pub(crate) fn rev_list(root: &Path, range: &str) -> io::Result<Vec<String>> {
    let out = git(root)
        .args(["rev-list", "--reverse", "--topo-order", range])
        .output()?;
    Ok(String::from_utf8_lossy(&out)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect())
}

/// What one revision expression resolves to, or `None` when it names nothing.
///
/// A missing revision is an answer rather than a failure here: the caller asks whether HEAD is
/// in a range and whether a commit has a first parent, and a root commit having none is
/// ordinary. Every other git failure still reaches the caller as an error.
pub(crate) fn rev_parse(root: &Path, expression: &str) -> Option<String> {
    let out = git(root)
        .args(["rev-parse", "--verify", "--quiet", expression])
        .accept(1)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out).trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// Which snapshot a read names: one commit's tree, or the tree git's index would commit.
///
/// A snapshot is read from git objects alone, never from the working tree, so two runs over one
/// snapshot read the same bytes whatever the working tree holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Source<'a> {
    /// The tree `git commit` would record now, absent a pathspec or `-a`.
    Index,
    /// The tree of the commit this names.
    Commit(&'a str),
}

/// One entry of a snapshot's listing: the path and kind a walk reads, and the mode and blob id
/// the snapshot records for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SnapshotEntry {
    pub(crate) rel: PathBuf,
    pub(crate) kind: EntryKind,
    pub(crate) mode: String,
    pub(crate) oid: String,
}

impl SnapshotEntry {
    pub(crate) fn entry(&self) -> Entry {
        Entry {
            rel: self.rel.clone(),
            kind: self.kind,
        }
    }
}

/// Every entry of a snapshot, project-relative, sorted by path, with its kind read off the mode
/// the snapshot records.
///
/// **Run from the project root, `ls-tree` is scoped to it and names its entries relative to
/// it.** That is the default and `--full-name` is what turns it off, so a project vendored as
/// a subdirectory of its repository gets exactly the paths its manifest declares, and nothing
/// from outside it. The same holds of the blob requests below, which name a path relative to
/// the working directory.
///
/// **The index's snapshot is HEAD's tree with the index's changes against it**, the tree a
/// commit would record, per `design@core@staged-tree-source`. It is not `ls-files -s`, which
/// lists an intent-to-add entry, `git add -N`, at stage 0 with the empty blob, the same line as
/// a staged empty file, while `git commit` records no such entry. `diff --cached` leaves an
/// intent-to-add entry out, and compares against the empty tree where HEAD does not exist yet.
/// The caller asks [`unmerged`] first: an unmerged path has no blob to commit, and a `U` row
/// here is an error.
pub(crate) fn snapshot_entries(root: &Path, source: Source) -> io::Result<Vec<SnapshotEntry>> {
    let base = match source {
        Source::Commit(sha) => Some(sha.to_string()),
        Source::Index => rev_parse(root, "HEAD"),
    };
    let mut listing: BTreeMap<PathBuf, SnapshotEntry> = BTreeMap::new();
    if let Some(base) = base {
        let listed = git(root).args(["ls-tree", "-r", "-z", &base]).output()?;
        for line in nul_separated(&listed) {
            if let Some(entry) = tree_line(line) {
                listing.insert(entry.rel.clone(), entry);
            }
        }
    }
    if source == Source::Index {
        // `--relative` keeps the diff to the project and names its paths from it, as `ls-tree`
        // run from the root already does; without it the paths are the repository's and a
        // sibling directory's changes come in. `--no-abbrev` gives whole blob ids, which
        // `--full-index` does not under `--raw`. `--ita-invisible-in-index` is git's default
        // for `diff --cached` and is passed so the listing does not rest on that default.
        // `--ignore-submodules=none` because `diff` otherwise applies `diff.ignoreSubmodules` and
        // a `.gitmodules` entry's `ignore` key, and drops a staged gitlink a commit records.
        let changed = git(root)
            .args([
                "diff",
                "--cached",
                "--raw",
                "-z",
                "--no-renames",
                "--no-abbrev",
                "--no-color",
                "--relative",
                "--ita-invisible-in-index",
                "--ignore-submodules=none",
            ])
            .output()?;
        overlay(&mut listing, &changed)?;
    }
    Ok(listing.into_values().collect())
}

/// One line of `ls-tree -r -z`: `<mode> <type> <oid>` then a tab and the path. The first tab,
/// because the fields hold none and the path may.
fn tree_line(line: &[u8]) -> Option<SnapshotEntry> {
    let tab = line.iter().position(|b| *b == b'\t')?;
    let fields: Vec<&[u8]> = line[..tab].split(|b| *b == b' ').collect();
    let [mode, _, oid] = fields[..] else {
        return None;
    };
    Some(SnapshotEntry {
        rel: as_path(&line[tab + 1..]),
        kind: kind_of(mode),
        mode: String::from_utf8_lossy(mode).into_owned(),
        oid: String::from_utf8_lossy(oid).into_owned(),
    })
}

/// Apply `diff --cached --raw -z` output to a listing: each record is a header,
/// `:<src mode> <dst mode> <src oid> <dst oid> <status>`, then the path as the next field.
///
/// The status letters `--no-renames` leaves are `A`, `M`, `T`, `D` and `U`. A, M and T give the
/// path the index's mode and blob; D removes it; U is an unmerged path, which [`unmerged`]
/// refuses before this runs, and any letter this does not know is refused rather than skipped,
/// since a skipped row is a path the snapshot holds wrongly with nothing saying so.
fn overlay(listing: &mut BTreeMap<PathBuf, SnapshotEntry>, raw: &[u8]) -> io::Result<()> {
    let mut fields = nul_separated(raw).into_iter();
    while let Some(header) = fields.next() {
        let Some(path) = fields.next() else {
            return Err(io::Error::other(
                "`git diff --cached --raw` printed a record with no path",
            ));
        };
        let header = String::from_utf8_lossy(header);
        let parts: Vec<&str> = header.trim_start_matches(':').split(' ').collect();
        let [_, mode, _, oid, status] = parts[..] else {
            return Err(io::Error::other(format!(
                "`git diff --cached --raw` printed a record this tool cannot read: {header}"
            )));
        };
        let rel = as_path(path);
        match status {
            "A" | "M" | "T" => {
                listing.insert(
                    rel.clone(),
                    SnapshotEntry {
                        rel,
                        kind: kind_of(mode.as_bytes()),
                        mode: mode.to_string(),
                        oid: oid.to_string(),
                    },
                );
            }
            "D" => {
                listing.remove(&rel);
            }
            "U" => {
                return Err(io::Error::other(format!(
                    "the index holds an unmerged path: {}",
                    rel.display()
                )))
            }
            other => {
                return Err(io::Error::other(format!(
                    "`git diff --cached --raw` reported a change of kind `{other}` at {}, \
                     which this tool does not read",
                    rel.display()
                )))
            }
        }
    }
    Ok(())
}

fn kind_of(mode: &[u8]) -> EntryKind {
    match mode {
        b"120000" => EntryKind::Symlink,
        b"160000" => EntryKind::Gitlink,
        _ => EntryKind::File,
    }
}

/// Whether the project's part of the index differs from HEAD: whether anything of the project
/// is staged. `--relative` keeps the question to the project, since without it a staged change
/// anywhere in the repository answers yes; `--ignore-submodules=none`, since without it the
/// configuration can hide a staged gitlink. Before the first commit, the index is compared with
/// the empty tree.
pub(crate) fn index_differs_from_head(root: &Path) -> io::Result<bool> {
    let code = git(root)
        .args([
            "diff",
            "--cached",
            "--quiet",
            "--relative",
            "--ita-invisible-in-index",
            "--ignore-submodules=none",
        ])
        .accept(1)
        .code()?;
    Ok(code == 1)
}

/// The project's paths the index holds unmerged, at stage 1 to 3, each named once.
///
/// A conflicted merge or rebase leaves them, and the index then has no tree to commit: git
/// refuses `commit` and `write-tree` alike. `ls-files -u` prints one line per stage.
pub(crate) fn unmerged(root: &Path) -> io::Result<Vec<PathBuf>> {
    let listed = git(root).args(["ls-files", "-u", "-z"]).output()?;
    let mut out: Vec<PathBuf> = nul_separated(&listed)
        .into_iter()
        .filter_map(mode_and_path)
        .map(|e| e.rel)
        .collect();
    out.dedup();
    Ok(out)
}

/// Stage each generated file's bytes as the index's entry at its path, in one locked write, and
/// touch no working-tree file.
///
/// Each file's bytes become a blob first, with `hash-object -w`; then ONE `update-index -z
/// --index-info` call sets every entry, so the index changes for all of them or for none: git
/// takes its lock once, and a held lock fails the call with the index as it was. A refused run can
/// leave the blobs it wrote in the object store, unreachable, which `git gc` removes.
///
/// An entry the index holds keeps its mode; a new one is a regular file. `--index-info` reads a
/// path from the repository's root, not from the working directory, so each project-relative
/// path is prefixed with the project's place in the repository.
///
/// **It refuses, staging nothing, a path the index holds as anything but a regular file**: a
/// symlink or a gitlink, whose mode the text would be put under, and a directory, whose entries
/// `--index-info` would replace. It asks the index itself rather than a listing a walk row can
/// filter.
pub(crate) fn stage_generated(root: &Path, files: &[(PathBuf, String)]) -> io::Result<()> {
    if files.is_empty() {
        return Ok(());
    }
    let prefix = git(root).args(["rev-parse", "--show-prefix"]).output()?;
    let prefix = prefix.strip_suffix(b"\n").unwrap_or(&prefix).to_vec();
    let mut asked: Vec<OsString> = vec!["--literal-pathspecs".into(), "ls-files".into()];
    asked.extend(["-s", "-z", "--"].map(OsString::from));
    asked.extend(files.iter().map(|(rel, _)| rel.as_os_str().to_os_string()));
    let staged = git(root).args(asked).output()?;
    let modes: BTreeMap<PathBuf, String> = nul_separated(&staged)
        .into_iter()
        .filter_map(|line| {
            let space = line.iter().position(|b| *b == b' ')?;
            let tab = line.iter().position(|b| *b == b'\t')?;
            Some((
                as_path(&line[tab + 1..]),
                String::from_utf8_lossy(&line[..space]).into_owned(),
            ))
        })
        .collect();
    for (rel, _) in files {
        // A pathspec naming a directory matches the entries under it.
        if modes
            .keys()
            .any(|staged| staged != rel && staged.starts_with(rel))
        {
            return Err(io::Error::other(format!(
                "{} is a directory in the index, and staging a file there would replace its entries",
                rel.display()
            )));
        }
        if let Some(mode) = modes
            .get(rel)
            .filter(|m| !matches!(m.as_str(), "100644" | "100755"))
        {
            return Err(io::Error::other(format!(
                "{} is staged as a symlink or a gitlink, mode {mode}, and a generated file's text \
                 would be put under it",
                rel.display()
            )));
        }
    }
    let mut info = Vec::new();
    for (rel, text) in files {
        let oid = git(root)
            .args(["hash-object", "-w", "--stdin"])
            .stdin(text.as_bytes().to_vec())
            .output()?;
        let oid = String::from_utf8_lossy(&oid).trim().to_string();
        let mode = modes.get(rel).map(String::as_str).unwrap_or("100644");
        info.extend_from_slice(format!("{mode} {oid}\t").as_bytes());
        info.extend_from_slice(&prefix);
        info.extend_from_slice(path_bytes(rel).as_ref());
        info.push(0);
    }
    git(root)
        .args(["update-index", "-z", "--index-info"])
        .stdin(info)
        .output()?;
    Ok(())
}

/// How one path of a snapshot is named to `cat-file` and `rev-parse`.
///
/// `<rev>:./<path>` resolves relative to the working directory, where `<rev>:<path>` resolves
/// from the repository root. The first is what a project-relative path needs. `:./<path>` names
/// the index's stage-0 blob the same way.
pub(crate) fn object_name(source: Source, rel: &Path) -> String {
    match source {
        Source::Commit(sha) => format!("{sha}:./{}", rel.display()),
        Source::Index => format!(":./{}", rel.display()),
    }
}

#[cfg(test)]
mod object_naming {
    use super::*;

    /// The claim: a path is named to git relative to the working directory.
    ///
    /// `<rev>:<path>` resolves from the repository root, so a project vendored under its
    /// repository would ask for every one of its documents at a path the root does not hold
    /// and git would answer `missing` to all of them.
    #[test]
    fn a_snapshot_object_is_named_relative_to_the_working_directory() {
        assert_eq!(
            object_name(Source::Commit("abc"), Path::new("docs/a.md")),
            "abc:./docs/a.md"
        );
        assert_eq!(object_name(Source::Commit("abc"), Path::new("")), "abc:./");
        assert_eq!(
            object_name(Source::Index, Path::new("docs/a.md")),
            ":./docs/a.md"
        );
        assert_eq!(object_name(Source::Index, Path::new("")), ":./");
    }
}

/// The contents of many blobs of one snapshot, in one process.
///
/// **One `cat-file --batch`, not one process per file.** A tree of a thousand documents would
/// otherwise spawn a thousand processes at every commit in the range. A path the tree does not
/// hold contributes no entry rather than failing: the caller derives its request list from a
/// listing, and a race between the two is not worth an exit 2.
///
/// A blob whose bytes are not UTF-8 contributes no entry either, for the reason
/// `Model::build` keeps an unreadable file as an empty document: a lossy decoding produces
/// text nobody wrote. The caller sees the absence and reports it. [`blob_bytes`] answers the
/// bytes whatever they are.
pub(crate) fn blobs(
    root: &Path,
    source: Source,
    paths: &[PathBuf],
) -> io::Result<BTreeMap<PathBuf, String>> {
    Ok(blob_bytes(root, source, paths)?
        .into_iter()
        .filter_map(|(rel, bytes)| Some((rel, String::from_utf8(bytes).ok()?)))
        .collect())
}

/// The bytes of many blobs of one snapshot, in one process, as [`blobs`] reads them and
/// with no decoding. A path the snapshot does not hold contributes no entry.
pub(crate) fn blob_bytes(
    root: &Path,
    source: Source,
    paths: &[PathBuf],
) -> io::Result<BTreeMap<PathBuf, Vec<u8>>> {
    if paths.is_empty() {
        return Ok(BTreeMap::new());
    }
    let mut stdin = Vec::new();
    for rel in paths {
        // The one spelling, shared with the object lookup, so a path is named to git in one
        // way and a change to it cannot reach one caller and miss the other.
        stdin.extend_from_slice(object_name(source, Path::new("")).as_bytes());
        stdin.extend_from_slice(path_bytes(rel));
        stdin.push(0);
    }
    // **`-z`, so a request is NUL-terminated.** A tracked filename may hold a newline; under
    // the newline-terminated input that name becomes two requests, git answers both, and every
    // later answer is paired with the wrong path — silently, because the map still comes back
    // full. `-z` on `cat-file --batch` needs git 2.36 or newer, and an older git refuses the
    // flag loudly, which is the failure this module already turns into exit 2.
    let raw = git(root)
        .args(["cat-file", "--batch", "-z"])
        .stdin(stdin)
        .output()?;
    Ok(read_batch(&raw, paths))
}

/// `cat-file --batch`'s answers, paired with the requests they answer in order.
///
/// Split out because it is the one part of a per-commit read that can be stated in memory:
/// git's framing is a header line and then exactly the bytes the header counted, and reading
/// it wrong shifts every later answer onto the wrong path — which is a silent shape, since
/// the map still comes back full.
///
/// `--batch` answers each request as `<oid> <type> <size>` for a hit and `<request> missing`
/// for a miss, and a hit's body is followed by one newline. The size is read rather than a
/// separator scanned for, because a blob may hold any byte, newlines included.
fn read_batch(raw: &[u8], paths: &[PathBuf]) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut at = 0usize;
    let mut wanted = paths.iter();
    while at < raw.len() {
        let end = match raw[at..].iter().position(|b| *b == b'\n') {
            Some(i) => at + i,
            None => break,
        };
        let header = String::from_utf8_lossy(&raw[at..end]).into_owned();
        at = end + 1;
        let Some(rel) = wanted.next() else { break };
        let Some(size) = header
            .rsplit(' ')
            .next()
            .and_then(|n| n.parse::<usize>().ok())
            .filter(|_| !header.ends_with(" missing"))
        else {
            // A miss carries no body, so nothing is consumed past its header.
            continue;
        };
        let body_end = (at + size).min(raw.len());
        out.insert(rel.clone(), raw[at..body_end].to_vec());
        // The trailing newline `--batch` writes after every body.
        at = (body_end + 1).min(raw.len());
    }
    out
}

/// One commit's message, subject line and body, exactly as it was written.
pub(crate) fn commit_message(root: &Path, sha: &str) -> io::Result<String> {
    let out = git(root).args(["log", "-1", "--format=%B", sha]).output()?;
    Ok(String::from_utf8_lossy(&out).into_owned())
}

/// A path as the bytes git wrote, so a name no UTF-8 decoding accepts survives the round trip.
#[cfg(unix)]
fn path_bytes(path: &Path) -> &[u8] {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes()
}

#[cfg(not(unix))]
fn path_bytes(path: &Path) -> Vec<u8> {
    path.to_string_lossy().into_owned().into_bytes()
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
        let e = entries(&dir).expect_err("not a worktree").to_string();
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

    /// The claim: each answer is paired with the request it answers, and a body is consumed
    /// by the byte count its header states.
    ///
    /// A body holding a newline is the case that separates a size-driven read from a
    /// separator-driven one, and every document in a tree holds newlines. Reading it wrong
    /// shifts every later answer onto the wrong path and the map still comes back full, so
    /// nothing downstream would report it.
    #[test]
    fn a_batch_answer_is_read_by_the_byte_count_its_header_states() {
        let paths = vec![
            PathBuf::from("a.md"),
            PathBuf::from("b.md"),
            PathBuf::from("c.md"),
        ];
        let first = "# A\n\nTwo lines.\n";
        let third = "# C\n";
        let raw = format!(
            "aaaa blob {}\n{first}\nbbbb missing\ncccc blob {}\n{third}\n",
            first.len(),
            third.len()
        );
        let found = read_batch(raw.as_bytes(), &paths);
        assert_eq!(
            found.get(Path::new("a.md")).map(Vec::as_slice),
            Some(first.as_bytes())
        );
        assert_eq!(found.get(Path::new("b.md")), None, "a miss carries no body");
        assert_eq!(
            found.get(Path::new("c.md")).map(Vec::as_slice),
            Some(third.as_bytes())
        );
    }

    #[test]
    fn a_batch_with_nothing_in_it_answers_nothing() {
        assert!(read_batch(b"", &[PathBuf::from("a.md")]).is_empty());
        assert!(read_batch(b"aaaa blob 2\nhi\n", &[]).is_empty());
    }

    /// The claim: a tracked filename holding a newline does not shift every later answer.
    ///
    /// Under a newline-terminated batch input such a name is two requests, git answers both,
    /// and each answer after it is paired with the wrong path. The map still comes back full,
    /// so nothing downstream reports it: the model is simply built out of the wrong bytes.
    #[test]
    fn a_listing_line_is_classified_by_its_mode_in_both_formats() {
        // `ls-files -s`: mode, object, stage, tab, path. `ls-tree -r`: mode, type, object,
        // tab, path. The mode is the first field of both, and the path follows the last tab.
        let staged =
            mode_and_path(b"120000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0\tdocs/link.md")
                .expect("an entry");
        assert_eq!(staged.kind, EntryKind::Symlink);
        assert_eq!(staged.rel, PathBuf::from("docs/link.md"));
        let tree =
            mode_and_path(b"160000 commit e69de29bb2d1d6434b8b29ae775ad8c2e48c5391\tvendor/sub")
                .expect("an entry");
        assert_eq!(tree.kind, EntryKind::Gitlink);
        assert_eq!(tree.rel, PathBuf::from("vendor/sub"));
        let file = mode_and_path(b"100644 blob e69de29bb2d1d6434b8b29ae775ad8c2e48c5391\ta\tb.md")
            .expect("an entry");
        assert_eq!(file.kind, EntryKind::File);
        assert_eq!(
            file.rel,
            PathBuf::from("a\tb.md"),
            "the path may hold a tab"
        );
        assert!(mode_and_path(b"no tab here").is_none());
    }

    #[test]
    fn a_newline_in_a_tracked_name_does_not_shift_the_answers_after_it() {
        let repo =
            std::env::temp_dir().join(format!("knowledge-git-newline-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).expect("a repository directory");
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "fixture"],
            vec!["config", "user.email", "fixture@example.invalid"],
        ] {
            git(&repo).args(args).output().expect("a repository");
        }
        let odd = PathBuf::from("aa\nbb.md");
        let plain = PathBuf::from("zz.md");
        std::fs::write(repo.join(&odd), "# Odd\n").expect("a document with an odd name");
        std::fs::write(repo.join(&plain), "# Plain\n").expect("a plain document");
        git(&repo).args(["add", "-A"]).output().expect("staged");
        git(&repo)
            .args(["commit", "-q", "-m", "two documents"])
            .output()
            .expect("a commit");
        let sha = rev_parse(&repo, "HEAD").expect("the head");

        let read =
            blobs(&repo, Source::Commit(&sha), &[odd.clone(), plain.clone()]).expect("both blobs");
        assert_eq!(read.get(&odd).map(String::as_str), Some("# Odd\n"));
        assert_eq!(
            read.get(&plain).map(String::as_str),
            Some("# Plain\n"),
            "the answer after the odd name is still its own"
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// The claim: a commit's message comes back as it was written, and the walk over a range
    /// follows the parent chain.
    ///
    /// The order matters because the previous commit's model is reused as the next one's
    /// parent tree: an order that is not the chain resolves a message against the wrong tree.
    #[test]
    fn a_range_walks_the_parent_chain_and_each_message_comes_back_whole() {
        let repo = std::env::temp_dir().join(format!("knowledge-git-range-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).expect("a repository directory");
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "fixture"],
            vec!["config", "user.email", "fixture@example.invalid"],
        ] {
            git(&repo).args(args).output().expect("a repository");
        }
        let mut shas = Vec::new();
        for (n, subject) in ["the first", "the second", "the third"].iter().enumerate() {
            std::fs::write(repo.join(format!("{n}.md")), "# A\n").expect("a document");
            git(&repo).args(["add", "-A"]).output().expect("staged");
            git(&repo)
                .args(["commit", "-q", "-m", &format!("{subject}\n\nA body.")])
                .output()
                .expect("a commit");
            shas.push(rev_parse(&repo, "HEAD").expect("the new head"));
        }
        let walked = rev_list(&repo, &format!("{}..HEAD", shas[0])).expect("a range");
        assert_eq!(walked, shas[1..], "oldest first, along the parent chain");
        let message = commit_message(&repo, &shas[1]).expect("a message");
        assert!(message.starts_with("the second\n\nA body."), "{message:?}");
        assert_eq!(rev_parse(&repo, "nowhere-at-all"), None);
        assert_eq!(
            rev_parse(&repo, &format!("{}^", shas[0])),
            None,
            "a root commit has no first parent"
        );
        let listed: Vec<PathBuf> = snapshot_entries(&repo, Source::Commit(&shas[2]))
            .expect("a tree listing")
            .into_iter()
            .map(|e| e.rel)
            .collect();
        assert_eq!(
            listed,
            vec![
                PathBuf::from("0.md"),
                PathBuf::from("1.md"),
                PathBuf::from("2.md")
            ]
        );
        let read = blobs(&repo, Source::Commit(&shas[2]), &listed).expect("its blobs");
        assert_eq!(read.len(), 3);
        assert_eq!(
            read.get(Path::new("1.md")).map(String::as_str),
            Some("# A\n")
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// A repository under the temporary directory, configured to commit, with nothing in it.
    fn scratch_repository(tag: &str) -> PathBuf {
        let repo = std::env::temp_dir().join(format!(
            "knowledge-git-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).expect("a repository directory");
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "fixture"],
            vec!["config", "user.email", "fixture@example.invalid"],
        ] {
            git(&repo).args(args).output().expect("a repository");
        }
        repo
    }

    fn write(dir: &Path, rel: &str, text: &str) {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("its directory");
        std::fs::write(path, text).expect("a file");
    }

    /// The claim of `design@core@staged-tree-source`: the index's snapshot is the tree `git write-tree`
    /// records, and its bytes are the staged ones, not the working tree's.
    ///
    /// The project sits in a subdirectory of its repository beside a sibling whose change is
    /// staged too, because that is where a listing that is not scoped to the project, or that
    /// names paths from the repository's root, goes wrong. The index holds every entry kind a
    /// commit records differently from a plain listing: an intent-to-add entry, which
    /// `ls-files -s` lists and a commit does not; a staged deletion; a staged empty file, which
    /// `ls-files -s` cannot tell from the first; a file whose working bytes differ from its
    /// staged ones; a symlink; and a gitlink.
    #[test]
    fn the_index_snapshot_is_the_tree_a_commit_would_record() {
        let repo = scratch_repository("staged");
        let project = repo.join("proj");
        write(&project, "keep.md", "# Keep\n");
        write(&project, "gone.md", "# Gone\n");
        write(&project, "changed.md", "# Committed\n");
        write(&repo, "other/o.md", "# Other\n");
        git(&repo).args(["add", "-A"]).output().expect("staged");
        git(&repo)
            .args(["commit", "-qm", "the base"])
            .output()
            .expect("a commit");
        let head = rev_parse(&repo, "HEAD").expect("the head");

        write(&project, "changed.md", "# Staged\n");
        write(&project, "empty.md", "");
        write(&project, "ita.md", "# Intent to add\n");
        write(&repo, "other/o.md", "# Other, staged\n");
        git(&project)
            .args(["add", "changed.md", "empty.md", "../other/o.md"])
            .output()
            .expect("staged");
        git(&project)
            .args(["add", "-N", "ita.md"])
            .output()
            .expect("intent to add");
        git(&project)
            .args(["rm", "-q", "gone.md"])
            .output()
            .expect("a staged deletion");
        let target = String::from_utf8(
            git(&project)
                .args(["hash-object", "-w", "--stdin"])
                .stdin(b"keep.md".to_vec())
                .output()
                .expect("a symlink target"),
        )
        .expect("an id");
        // `--cacheinfo` names a path from the repository's root, not from the working directory.
        for info in [
            format!("120000,{},proj/link.md", target.trim()),
            format!("160000,{head},proj/sub"),
        ] {
            git(&project)
                .args(["update-index", "--add", "--cacheinfo", &info])
                .output()
                .expect("an entry the working tree does not hold");
        }
        write(&project, "changed.md", "# Working\n");

        let listed = snapshot_entries(&project, Source::Index).expect("the index's snapshot");
        let names: Vec<PathBuf> = listed.iter().map(|e| e.rel.clone()).collect();
        let tree = String::from_utf8(
            git(&project)
                .args(["write-tree"])
                .output()
                .expect("the tree a commit would record"),
        )
        .expect("an id");
        let recorded = git(&project)
            .args(["ls-tree", "-r", "-z", "--name-only", tree.trim()])
            .paths()
            .expect("its listing");
        assert_eq!(names, recorded, "the listing is the tree a commit records");
        assert_eq!(
            names,
            ["changed.md", "empty.md", "keep.md", "link.md", "sub"]
                .map(PathBuf::from)
                .to_vec(),
            "scoped to the project, named from it, with no intent-to-add entry"
        );
        let kind = |rel: &str| {
            listed
                .iter()
                .find(|e| e.rel == Path::new(rel))
                .map(|e| e.kind)
        };
        assert_eq!(kind("link.md"), Some(EntryKind::Symlink));
        assert_eq!(kind("sub"), Some(EntryKind::Gitlink));
        assert_eq!(kind("changed.md"), Some(EntryKind::File));
        let staged_id = rev_parse(&project, ":./changed.md").expect("the staged blob");
        assert_eq!(
            listed
                .iter()
                .find(|e| e.rel == Path::new("changed.md"))
                .map(|e| e.oid.clone()),
            Some(staged_id),
            "the whole id of the staged blob"
        );
        let read = blobs(
            &project,
            Source::Index,
            &[PathBuf::from("changed.md"), PathBuf::from("keep.md")],
        )
        .expect("the staged blobs");
        assert_eq!(
            read.get(Path::new("changed.md")).map(String::as_str),
            Some("# Staged\n"),
            "the staged bytes, not the working tree's"
        );
        assert_eq!(
            read.get(Path::new("keep.md")).map(String::as_str),
            Some("# Keep\n")
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// The claim: staging generated files from a project in a subdirectory of its repository
    /// sets the entries at the project's paths, keeps an existing entry's mode, adds a missing
    /// one as a regular file, and leaves the working files as they were.
    #[test]
    fn generated_files_are_staged_at_the_projects_paths_with_their_modes() {
        let repo = scratch_repository("stage-generated");
        let project = repo.join("proj");
        write(&project, "docs/g.md", "# Old\n");
        git(&repo).args(["add", "-A"]).output().expect("staged");
        git(&repo)
            .args(["update-index", "--chmod=+x", "proj/docs/g.md"])
            .output()
            .expect("an executable entry");
        stage_generated(
            &project,
            &[
                (PathBuf::from("docs/g.md"), "# New\n".to_string()),
                (PathBuf::from("docs/added.md"), "# Added\n".to_string()),
            ],
        )
        .expect("staged");
        let listed = String::from_utf8(
            git(&repo)
                .args(["ls-files", "-s"])
                .output()
                .expect("a listing"),
        )
        .expect("text");
        let entry = |path: &str| {
            listed
                .lines()
                .find(|l| l.ends_with(&format!("\t{path}")))
                .map(|l| l.split(' ').next().unwrap_or_default().to_string())
        };
        assert_eq!(
            entry("proj/docs/g.md").as_deref(),
            Some("100755"),
            "{listed}"
        );
        assert_eq!(
            entry("proj/docs/added.md").as_deref(),
            Some("100644"),
            "{listed}"
        );
        assert_eq!(
            entry("docs/g.md"),
            None,
            "nothing at the repository's root: {listed}"
        );
        assert_eq!(
            blobs(&project, Source::Index, &[PathBuf::from("docs/g.md")])
                .expect("the staged blob")
                .get(Path::new("docs/g.md"))
                .map(String::as_str),
            Some("# New\n")
        );
        assert_eq!(
            std::fs::read_to_string(project.join("docs/g.md")).expect("the working file"),
            "# Old\n",
            "the working file is untouched"
        );
        assert!(!project.join("docs/added.md").exists());
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// The claim: a staged submodule change is in the index's snapshot whatever the
    /// configuration says to ignore, and the project's staged part is told from a sibling's.
    ///
    /// `git diff` applies `diff.ignoreSubmodules` and a `.gitmodules` entry's `ignore` key, so a
    /// staged gitlink can leave a diff that does not ask for every submodule change. And a staged
    /// change outside the project is no change of the project.
    #[test]
    fn a_staged_submodule_change_is_listed_whatever_the_configuration_ignores() {
        let repo = scratch_repository("submodule-ignored");
        let project = repo.join("proj");
        write(&project, "keep.md", "# Keep\n");
        write(&repo, "other/o.md", "# Other\n");
        write(
            &project,
            ".gitmodules",
            "[submodule \"s\"]\n\tpath = sub\n\turl = ./x\n\tignore = all\n",
        );
        git(&repo).args(["add", "-A"]).output().expect("staged");
        git(&repo)
            .args(["commit", "-qm", "base"])
            .output()
            .expect("a commit");
        let head = rev_parse(&repo, "HEAD").expect("the head");
        git(&repo)
            .args(["config", "diff.ignoreSubmodules", "all"])
            .output()
            .expect("configured");

        // A sibling's change alone is staged: the project's part equals HEAD.
        write(&repo, "other/o.md", "# Other, staged\n");
        git(&repo)
            .args(["add", "other/o.md"])
            .output()
            .expect("staged");
        assert!(!index_differs_from_head(&project).expect("an answer"));

        git(&repo)
            .args([
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("160000,{head},proj/sub"),
            ])
            .output()
            .expect("a staged gitlink");
        assert!(index_differs_from_head(&project).expect("an answer"));
        let listed = snapshot_entries(&project, Source::Index).expect("the snapshot");
        assert_eq!(
            listed
                .iter()
                .find(|e| e.rel == Path::new("sub"))
                .map(|e| e.kind),
            Some(EntryKind::Gitlink),
            "{listed:?}"
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// Before the first commit there is no HEAD to overlay, and the snapshot is the index's
    /// additions alone, an intent-to-add entry still left out.
    #[test]
    fn the_index_snapshot_holds_the_additions_alone_before_any_commit() {
        let repo = scratch_repository("unborn");
        write(&repo, "a.md", "# A\n");
        write(&repo, "n.md", "# N\n");
        git(&repo).args(["add", "a.md"]).output().expect("staged");
        git(&repo)
            .args(["add", "-N", "n.md"])
            .output()
            .expect("intent to add");
        let names: Vec<PathBuf> = snapshot_entries(&repo, Source::Index)
            .expect("the index's snapshot")
            .into_iter()
            .map(|e| e.rel)
            .collect();
        assert_eq!(names, vec![PathBuf::from("a.md")]);
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// A conflicted path is named once, though the index holds it at three stages.
    #[test]
    fn an_unmerged_path_is_named_once() {
        let repo = scratch_repository("unmerged");
        write(&repo, "c.md", "base\n");
        git(&repo).args(["add", "-A"]).output().expect("staged");
        git(&repo)
            .args(["commit", "-qm", "base"])
            .output()
            .expect("a commit");
        git(&repo)
            .args(["checkout", "-qb", "side"])
            .output()
            .expect("a branch");
        write(&repo, "c.md", "side\n");
        git(&repo)
            .args(["commit", "-qam", "side"])
            .output()
            .expect("a commit");
        git(&repo)
            .args(["checkout", "-q", "-"])
            .output()
            .expect("back");
        write(&repo, "c.md", "main\n");
        git(&repo)
            .args(["commit", "-qam", "main"])
            .output()
            .expect("a commit");
        // The merge conflicts and exits 1; the index it leaves is the subject.
        let _ = git(&repo).args(["merge", "-q", "side"]).accept(1).output();
        assert_eq!(
            unmerged(&repo).expect("a listing"),
            vec![PathBuf::from("c.md")]
        );
        assert!(
            snapshot_entries(&repo, Source::Index).is_err(),
            "a `U` row is refused"
        );
        let _ = std::fs::remove_dir_all(&repo);
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
