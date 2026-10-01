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

/// Every live file of the project: what git tracks, plus what it does not track and does not
/// ignore.
///
/// This is the walk's source. `--cached` is unaffected by the ignore rules, which is why a
/// tracked file cannot leave the walk however the ignore rules are written; `--exclude-standard`
/// applies them to `--others` alone, which is where build output and on-demand directories are
/// dropped. Every remaining exclusion is the manifest's, and `walk::live_files` applies it.
pub fn live_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    Ok(entries(root)?
        .into_iter()
        .filter(|e| e.kind == EntryKind::File)
        .map(|e| e.rel)
        .collect())
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
pub fn entries(root: &Path) -> io::Result<Vec<Entry>> {
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
    for rel in others {
        let kind = match std::fs::symlink_metadata(root.join(&rel)) {
            Ok(meta) if meta.file_type().is_symlink() => EntryKind::Symlink,
            _ => EntryKind::File,
        };
        out.push(Entry { rel, kind });
    }
    out.sort_by(|a, b| a.rel.cmp(&b.rel));
    Ok(out)
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
/// fresh clone and a built tree, per `design@core@ignored-targets-are-not-asserted`.
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

/// The commits a range names, oldest first.
///
/// **Topological order, not date order.** `--reverse` alone reverses git's default
/// reverse-chronological listing, and a commit whose recorded date precedes its parent's — a
/// rebase across a clock skew produces one — then lands before the commit it descends from.
/// The per-commit check reuses the previous model as the next commit's parent tree, so an
/// order that is not the parent chain would resolve a message against the wrong tree.
pub fn rev_list(root: &Path, range: &str) -> io::Result<Vec<String>> {
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
pub fn rev_parse(root: &Path, expression: &str) -> Option<String> {
    let out = git(root)
        .args(["rev-parse", "--verify", "--quiet", expression])
        .accept(1)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out).trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// Every file in one commit's tree, project-relative.
///
/// **Run from the project root, `ls-tree` is scoped to it and names its entries relative to
/// it.** That is the default and `--full-name` is what turns it off, so a project vendored as
/// a subdirectory of its repository gets exactly the paths its manifest declares, and nothing
/// from outside it. The same holds of the blob requests below, which name a path relative to
/// the working directory.
pub fn tree_files(root: &Path, sha: &str) -> io::Result<Vec<PathBuf>> {
    Ok(tree_entries(root, sha)?
        .into_iter()
        .map(|e| e.rel)
        .collect())
}

/// The same, with each entry's kind, read off the mode the tree records.
pub fn tree_entries(root: &Path, sha: &str) -> io::Result<Vec<Entry>> {
    let listed = git(root).args(["ls-tree", "-r", "-z", sha]).output()?;
    Ok(nul_separated(&listed)
        .into_iter()
        .filter_map(mode_and_path)
        .collect())
}

/// How one path in one commit's tree is named to `cat-file` and `rev-parse`.
///
/// `<rev>:./<path>` resolves relative to the working directory, where `<rev>:<path>` resolves
/// from the repository root. The first is what a project-relative path needs.
pub fn tree_object(sha: &str, rel: &Path) -> String {
    format!("{sha}:./{}", rel.display())
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
    fn a_tree_object_is_named_relative_to_the_working_directory() {
        assert_eq!(
            tree_object("abc", Path::new("docs/a.md")),
            "abc:./docs/a.md"
        );
        assert_eq!(tree_object("abc", Path::new("")), "abc:./");
    }
}

/// The contents of many blobs of one commit's tree, in one process.
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
pub fn blobs(root: &Path, sha: &str, paths: &[PathBuf]) -> io::Result<BTreeMap<PathBuf, String>> {
    Ok(blob_bytes(root, sha, paths)?
        .into_iter()
        .filter_map(|(rel, bytes)| Some((rel, String::from_utf8(bytes).ok()?)))
        .collect())
}

/// The bytes of many blobs of one commit's tree, in one process, as [`blobs`] reads them and
/// with no decoding. A path the tree does not hold contributes no entry.
pub fn blob_bytes(
    root: &Path,
    sha: &str,
    paths: &[PathBuf],
) -> io::Result<BTreeMap<PathBuf, Vec<u8>>> {
    if paths.is_empty() {
        return Ok(BTreeMap::new());
    }
    let mut stdin = Vec::new();
    for rel in paths {
        // The one spelling, shared with the object lookup, so a path is named to git in one
        // way and a change to it cannot reach one caller and miss the other.
        stdin.extend_from_slice(tree_object(sha, Path::new("")).as_bytes());
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
pub fn commit_message(root: &Path, sha: &str) -> io::Result<String> {
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

        let read = blobs(&repo, &sha, &[odd.clone(), plain.clone()]).expect("both blobs");
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
        let listed = tree_files(&repo, &shas[2]).expect("a tree listing");
        assert_eq!(
            listed,
            vec![
                PathBuf::from("0.md"),
                PathBuf::from("1.md"),
                PathBuf::from("2.md")
            ]
        );
        let read = blobs(&repo, &shas[2], &listed).expect("its blobs");
        assert_eq!(read.len(), 3);
        assert_eq!(
            read.get(Path::new("1.md")).map(String::as_str),
            Some("# A\n")
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
