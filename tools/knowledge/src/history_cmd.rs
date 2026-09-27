//! Commit messages under the citation regime, and the hook that judges one before it exists.
//!
//! **A commit message is a document.** It is parsed as one markdown document — the subject
//! line, the blank line and the body — and every rule of the regime runs over it: a `CR:`
//! marker owes its quote inside the message within the distance rule, every reference resolves
//! through the entity table, and the missing-marker lint reads it as it reads any other prose.
//! The decision, and why a message is judged against a tree read from git objects rather than
//! against the working tree, is `design@knowledge@a-commit-message-is-a-document`.
//!
//! Three commands live here. `commit-message <file>` judges one message against the tree the
//! index holds and is what the `commit-msg` hook calls. `commits <range>` walks the range and judges
//! each message against its own commit's tree. `hook install` and `hook status` are per-clone
//! configuration, read by no check.

use crate::output::outln;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Subcommand;

use documentation::check::citations::Release;
use documentation::check::{self, Inputs};
use documentation::entity::{Anchors, Entities};
use documentation::manifest::MANIFEST_NAME;
use documentation::survey::Survey;
use documentation::{Finding, Manifest, Model};

/// Where the hook script sits, and the value `core.hooksPath` takes.
///
/// A tracked directory rather than the repository's own hooks directory, which git does not
/// track: a hook nobody can commit is a hook every clone installs by hand and no review reads.
pub const HOOKS_PATH: &str = ".githooks";

/// The script, byte for byte.
///
/// Two lines and no logic. Everything the hook decides is decided by the command it calls, so
/// a change to the rules is a change to the tool rather than to a file every clone already
/// pointed at. The project commits it at `path@thaum@.githooks/commit-msg`; `hook install`
/// writes it where a tree does not carry one, which is what makes the command work in a
/// repository that is not this one.
pub const COMMIT_MSG_HOOK: &str = "#!/bin/sh\nexec cargo knowledge commit-message \"$1\"\n";

#[derive(Subcommand)]
pub enum HookCommand {
    /// Point this clone's `core.hooksPath` at the committed hooks.
    Install {
        /// Overwrite a `core.hooksPath` that names something else.
        #[arg(long)]
        force: bool,
    },
    /// Whether this clone runs the committed hooks.
    Status,
}

// ---------------------------------------------------------------------------------------
// The message as a document
// ---------------------------------------------------------------------------------------

/// The rel path a message is parsed under.
///
/// The suffix is what selects the markdown grammar, per `design@knowledge@grammars-not-prefixes`, so
/// a message needs one; the findings are relabelled with the commit or the file before they
/// are printed, and no reader ever sees this name.
fn message_rel() -> PathBuf {
    PathBuf::from("commit-message.md")
}

/// The draft a `commit-msg` hook is handed, with git's own comment block blanked out.
///
/// **This is for the draft alone, never for a message a commit already holds.** The hook runs
/// before git's cleanup, so the file still carries the `# Please enter the commit message…`
/// block and, under `--verbose`, the diff below the scissors line. Judging those would report
/// on text that never reaches the commit, and each `#` line would open a markdown heading that
/// splits the draft into scopes nobody wrote.
///
/// **The line is blanked, not removed**, so every finding's line number is the line of the
/// file the hook was handed. Removing them renumbers everything below the block.
///
/// A message given with `-m` keeps its `#` lines, git's default cleanup for that form being
/// whitespace-only, and nothing in the file tells the two cases apart. The hook therefore says
/// nothing about such a line and `commits` judges it: `commits` reads the message the commit
/// holds and cleans nothing, so no byte of a commit leaves the regime.
pub fn cleaned(raw: &str) -> String {
    const SCISSORS: &str = "------------------------ >8 ------------------------";
    let mut out = String::new();
    let mut past_scissors = false;
    for line in raw.lines() {
        if !past_scissors && line.starts_with('#') {
            past_scissors = line.contains(SCISSORS);
        } else if !past_scissors {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

/// Everything one tree contributes to judging a message written against it.
///
/// Held together because `Inputs` borrows every field of it: a check is a pure function over
/// the model, so whatever it needs about the tree is gathered once and handed in.
struct Assembly {
    manifest: Manifest,
    model: Model,
    releases: HashMap<Option<String>, Release>,
    pinned: String,
    committed: HashMap<PathBuf, String>,
    configs: HashMap<PathBuf, String>,
    survey: Survey,
    ignored: HashSet<String>,
    tracked_and_ignored: Vec<PathBuf>,
    /// What is wrong with the tree itself: every check but `corpus` and `changes` at
    /// `Depth::Judged`, the first three phases alone at `Depth::Foundation`, nothing at
    /// `Depth::References`.
    trouble: Vec<Finding>,
    /// The phase the tree's run stopped in, where it stopped before the last: its entity
    /// table is then incomplete, and a message judged against it is judged against nothing.
    stopped: Option<check::Phase>,
}

impl Assembly {
    fn anchors(&self) -> Anchors {
        Anchors::of(&self.manifest)
    }

    fn inputs(&self) -> Inputs<'_> {
        Inputs {
            releases: &self.releases,
            pinned: &self.pinned,
            committed: &self.committed,
            configs: &self.configs,
            present: &self.survey.present,
            directories: &self.survey.directories,
            outside: &self.survey.outside,
            ignored: &self.ignored,
            tracked_and_ignored: &self.tracked_and_ignored,
            refused: &self.survey.refused,
            links: &self.survey.links,
        }
    }

    /// The release a message's quotes verify against: the one the tree pins.
    fn release(&self) -> Option<&Release> {
        self.releases.get(&None)
    }
}

/// Judge one message against a tree, and optionally against a second one for its references.
///
/// **References resolve against either tree, every other rule against the first.** A commit
/// that closes an issue deletes the entry file and names it in the message; against its own
/// tree alone every such message would dangle. Quote verification and the distance rule have
/// no such asymmetry: what a message says about a rule is judged against the release the
/// commit pins, and nothing about the parent bears on it.
fn judge_message(
    text: &str,
    primary: &Assembly,
    primary_entities: &Entities,
    parent: Option<(&Assembly, &Entities)>,
) -> Vec<Finding> {
    let model = Model::from_documents(vec![(message_rel(), text.to_string())]);
    let doc = &model.documents()[0];
    let mut out = Vec::new();

    if let Some(release) = primary.release() {
        // The message is never lint-exempt: the exemption list names files that are leaving
        // the tree, and a message is written now.
        let (found, _) = check::citations::check(doc, release, false);
        out.extend(found);
        let (judged, _) = check::regime::check(doc, release);
        out.extend(judged.into_iter().map(|j| j.finding));
    }

    let anchors = primary.anchors();
    let (mine, _) = check::references::judge(
        model.documents(),
        primary_entities,
        &anchors,
        &primary.inputs(),
    );
    let references = match parent {
        None => mine,
        Some((other, other_entities)) => {
            let other_anchors = other.anchors();
            let (theirs, _) = check::references::judge(
                model.documents(),
                other_entities,
                &other_anchors,
                &other.inputs(),
            );
            // **A finding survives where both trees refuse the same span on the same line,
            // whatever each says about it.** Two trees can refuse one reference for different
            // reasons — one because the anchor is unknown, the other because the id is not
            // defined there — and comparing the findings whole would then intersect to
            // nothing and let a reference that resolves in NEITHER tree pass. The site is the
            // line and the span the finding opens with, which is what both arms name.
            let refused: HashSet<(Option<u32>, String)> = theirs.iter().map(site).collect();
            mine.into_iter()
                .filter(|f| refused.contains(&site(f)))
                .collect()
        }
    };
    out.extend(references);
    out.sort_by_key(|f| f.line);
    out
}

/// What a reference finding is about: the line, and the span it opens with.
///
/// Every finding the reference family writes begins with the backticked span it judged, and
/// that span plus the line is what identifies the site across two trees that phrase their
/// refusals differently. A finding opening with no span is its own site, spelled whole.
fn site(finding: &Finding) -> (Option<u32>, String) {
    let what = &finding.what;
    let span = what
        .strip_prefix('`')
        .and_then(|rest| rest.split_once('`'))
        .map(|(span, _)| span.to_string())
        .unwrap_or_else(|| what.clone());
    (finding.line, span)
}

/// The same findings, named by the commit or the file the message came from.
fn relabelled(findings: Vec<Finding>, label: &str) -> Vec<Finding> {
    findings
        .into_iter()
        .map(|mut f| {
            f.file = PathBuf::from(label);
            f
        })
        .collect()
}

// ---------------------------------------------------------------------------------------
// One commit's tree
// ---------------------------------------------------------------------------------------

/// Why a commit's tree could not be assembled at all.
///
/// Distinct from a tree that assembled and failed its checks: the first is reported as
/// `its tree does not load`, which is what a commit whose manifest the tip checker refuses
/// produces, and the second names its finding count.
struct Unloadable(String);

/// A parsed release kept across commits, keyed by the blob the commit's tree holds.
///
/// **Keyed by the object name, not by the pinned date.** Parsing the corpus is the single
/// largest cost in a per-commit run, and two commits pinning one date may still carry
/// different bytes — a fold, a re-fetch. The object name answers both questions at once.
#[derive(Default)]
struct Corpora {
    parsed: HashMap<String, Release>,
}

/// Everything read from one commit's tree, before the checks run over it.
struct FromTree {
    manifest: Manifest,
    /// Every path of the tree, the symlink and gitlink entries included.
    listing: Vec<PathBuf>,
    /// The symlink and gitlink entries, which are read as no document.
    links: Vec<documentation::git::Entry>,
    blobs: std::collections::BTreeMap<PathBuf, String>,
}

/// Read one commit's tree: its manifest, its listing, and the text of everything the walk or
/// the inverse assertion reads.
///
/// Deliberately not the whole tree. A path under a skipped directory, an excluded path and a
/// skipped file are read by no family, and the corpus alone is two megabytes at every commit
/// in the range.
fn read_tree(root: &Path, sha: &str, generated_extra: &[PathBuf]) -> Result<FromTree, Unloadable> {
    let entries = documentation::git::tree_entries(root, sha)
        .map_err(|e| Unloadable(format!("its tree could not be listed: {e}")))?;
    let listing: Vec<PathBuf> = entries.iter().map(|e| e.rel.clone()).collect();
    let links: Vec<documentation::git::Entry> = entries
        .iter()
        .filter(|e| e.kind != documentation::git::EntryKind::File)
        .cloned()
        .collect();
    let files: Vec<PathBuf> = entries
        .into_iter()
        .filter(|e| e.kind == documentation::git::EntryKind::File)
        .map(|e| e.rel)
        .collect();
    let manifest_rel = PathBuf::from(MANIFEST_NAME);
    let declaration = documentation::git::blobs(root, sha, std::slice::from_ref(&manifest_rel))
        .map_err(|e| Unloadable(format!("its {MANIFEST_NAME} could not be read: {e}")))?;
    let Some(text) = declaration.get(&manifest_rel) else {
        return Err(Unloadable(format!("its tree holds no {MANIFEST_NAME}")));
    };
    let manifest = Manifest::parse(root, text).map_err(Unloadable)?;
    let walk = manifest.walk();
    let mut wanted: Vec<PathBuf> = files
        .iter()
        .filter(|rel| !documentation::walk::skipped(rel, walk) && !walk.skip_files.contains(rel))
        .cloned()
        .collect();
    for rel in generated_extra {
        if listing.contains(rel) {
            wanted.push(rel.clone());
        }
    }
    // The corpus and the version file are skipped by the walk and are still needed: every
    // quote in the tree and in the message verifies against them.
    for rel in [
        manifest.rules().dir.join(&manifest.rules().text),
        manifest.rules().dir.join(&manifest.rules().version),
    ] {
        if listing.contains(&rel) {
            wanted.push(rel);
        }
    }
    wanted.sort();
    wanted.dedup();
    let blobs = documentation::git::blobs(root, sha, &wanted)
        .map_err(|e| Unloadable(format!("its blobs could not be read: {e}")))?;
    Ok(FromTree {
        manifest,
        listing,
        links,
        blobs,
    })
}

/// How much of a tree the caller needs.
///
/// A tree used only to resolve a message's references needs its entity table and the facts a
/// path reference asks about, and nothing else: no release parsed, no family run. That is
/// every parent tree, and parsing the corpus is the single largest cost in the run.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Depth {
    /// The entity table and the path facts. Nothing is judged.
    References,
    /// The releases the tree pins, and the first three phases: what a message's own checks
    /// need, and whether the entity table is complete. What the `commit-msg` hook builds for
    /// the index, since judging the tree's content is `commits`' work and it sits in front of
    /// every commit.
    Foundation,
    /// Everything: the releases the tree pins, and every family but `corpus` and `changes`.
    Judged,
}

/// Assemble one tree into a model, and judge it as far as `depth` asks.
///
/// `id` is a commit or a tree object: every read goes through `git ls-tree` and
/// `<id>:<path>`, which accept both.
///
/// **`corpus` and `changes` are the two families whose subject is not the model.** `corpus`
/// reads the filesystem, which a commit's tree is not; `changes` resolves every release its
/// changelog names, which at a commit means reading the whole archive out of git objects at
/// every step of the range. Neither can decide whether a message's references resolve, which
/// is the question this assembly exists to answer.
fn commit_tree(
    root: &Path,
    sha: &str,
    checker: Option<&Path>,
    corpora: &mut Corpora,
    depth: Depth,
) -> Result<Assembly, Unloadable> {
    // The rule index is generated and named by the manifest rather than derived, so it is
    // asked for by hand; every other generated index comes from the register instances.
    let read = read_tree(root, sha, &[])?;
    let manifest = read.manifest;
    let anchors = Anchors::of(&manifest);
    let generated = documentation::index::generated_paths(&manifest);
    // The walk reads through no symlink and no gitlink, so the files alone are walked.
    let files: Vec<PathBuf> = read
        .listing
        .iter()
        .filter(|rel| !read.links.iter().any(|e| e.rel == **rel))
        .cloned()
        .collect();
    let walked =
        documentation::walk::live_files(Path::new(""), manifest.walk(), &files, &generated);
    let mut blobs = read.blobs;
    // The generated indexes and the per-instance options sit outside the walk and are read by
    // the caller, exactly as `check` reads them off the filesystem.
    let mut extra: Vec<PathBuf> = vec![manifest.rules().dir.join("index.md")];
    extra.extend(generated.iter().cloned());
    for (_, _, home) in anchors.instances() {
        extra.push(home.config.clone());
    }
    extra.retain(|rel| read.listing.contains(rel) && !blobs.contains_key(rel));
    if !extra.is_empty() {
        let more = documentation::git::blobs(root, sha, &extra)
            .map_err(|e| Unloadable(format!("its generated files could not be read: {e}")))?;
        blobs.extend(more);
    }

    let mut docs = Vec::new();
    let mut missing = Vec::new();
    for rel in &walked {
        match blobs.get(rel) {
            Some(text) => docs.push((rel.clone(), text.clone())),
            // A tree entry the batch answered nothing for is a blob whose bytes are not text.
            // It is kept as an empty document carrying the reason, the way `Model::build`
            // keeps an unreadable file, so it is reported rather than silently absent.
            None => missing.push(rel.clone()),
        }
    }
    let mut model = Model::from_documents_under(docs, checker);
    for rel in missing {
        model.push_unreadable(
            rel,
            "this tree entry holds no text the tool can read; the walk reads it as a \
             document, so a binary blob at this path leaves a live document out of every \
             check"
                .to_string(),
        );
    }

    let mut releases = HashMap::new();
    let judging = depth != Depth::References;
    // Every release this tree cannot supply. A quote checked against no release is checked by
    // nothing at all, so each one is a finding of the tree rather than a family that quietly
    // did less: the commit then fails the range, which is the honest answer.
    let mut unresolved: Vec<Finding> = Vec::new();
    let rules_dir = manifest.rules().dir.clone();
    let corpus_rel = rules_dir.join(&manifest.rules().text);
    let body_starts_at = manifest.rules().body_starts_at;
    if judging && !blobs.contains_key(&corpus_rel) {
        unresolved.push(Finding::in_file(
            &corpus_rel,
            "this commit's tree holds no vendored release at the path its manifest names",
            "every quote in the tree and in the message verifies against nothing without it",
        ));
    }
    if let Some(text) = blobs.get(&corpus_rel).filter(|_| judging) {
        let key =
            documentation::git::rev_parse(root, &documentation::git::tree_object(sha, &corpus_rel))
                .unwrap_or_else(|| format!("{sha}:corpus"));
        let release = corpora
            .parsed
            .entry(key)
            .or_insert_with(|| Release::new(text, body_starts_at));
        releases.insert(None, release.clone());
    }
    let pinned = blobs
        .get(&rules_dir.join(&manifest.rules().version))
        .and_then(|t| rules::release::read_version(t).get("date").cloned())
        .unwrap_or_default();
    // A document that opts out of the vendored release verifies against the archive the
    // COMMIT holds, never against the working tree's.
    let pins: Vec<String> = {
        let mut seen: Vec<String> = model
            .documents()
            .iter()
            .filter_map(|d| d.pin.clone())
            .collect();
        seen.sort();
        seen.dedup();
        seen
    };
    if judging && !pins.is_empty() {
        let past = rules_dir.join(&manifest.rules().past);
        let wanted: Vec<PathBuf> = pins.iter().map(|d| past.join(format!("{d}.txt"))).collect();
        let archived = documentation::git::blobs(root, sha, &wanted)
            .map_err(|e| Unloadable(format!("its archive could not be read: {e}")))?;
        for (date, rel) in pins.iter().zip(&wanted) {
            match archived.get(rel) {
                Some(text) => {
                    releases.insert(Some(date.clone()), Release::new(text, body_starts_at));
                }
                None => unresolved.push(Finding::in_file(
                    rel,
                    format!("a document of this commit pins the release {date}, and its tree holds no archive of it"),
                    "the quotes of every file pinned there verify against nothing",
                )),
            }
        }
    }

    let mut committed = HashMap::new();
    for rel in extra_generated(&manifest, &generated) {
        if let Some(text) = blobs.get(&rel) {
            committed.insert(rel, text.clone());
        }
    }
    let mut configs = HashMap::new();
    for (_, _, home) in anchors.instances() {
        if let Some(text) = blobs.get(&home.config) {
            configs.insert(home.config.clone(), text.clone());
        }
    }
    let survey =
        documentation::survey::from_listing(&manifest, &model, &read.listing, &read.links, |rel| {
            blobs.get(rel).cloned()
        });
    // **The ignore rules are the working tree's.** `git check-ignore` reads the `.gitignore`
    // files that are on disk and has no form that asks a historical tree, so a commit whose
    // ignore rules differ from today's is judged against today's. What that can cost is a
    // path reference asserted where the commit's own rules would have exempted it — a finding
    // rather than a silence, and the range check's subject is the message rather than the
    // tree it names.
    let queries = check::references::ignore_queries(&model, &anchors);
    let ignored = documentation::git::ignored(root, &queries)
        .map_err(|e| Unloadable(format!("its ignore rules could not be asked: {e}")))?;

    let mut assembly = Assembly {
        manifest,
        model,
        releases,
        pinned,
        committed,
        configs,
        survey,
        ignored,
        // A commit holds no index, so nothing is both tracked and ignored in a tree.
        tracked_and_ignored: Vec::new(),
        trouble: Vec::new(),
        stopped: None,
    };
    if !judging {
        return Ok(assembly);
    }
    // The phases, as `check` runs them: a tree that stops in one of the first three carries
    // that phase's findings as its trouble and is judged no further.
    assembly.trouble =
        match check::foundation(&assembly.model, &assembly.manifest, &assembly.inputs()) {
            Err(stop) => {
                assembly.stopped = Some(stop.phase);
                stop.findings
            }
            Ok(()) if depth == Depth::Foundation => Vec::new(),
            Ok(()) => {
                let report = check::run(&assembly.model, &assembly.manifest, &assembly.inputs());
                let mut trouble = report.findings;
                trouble.extend(unresolved);
                trouble
            }
        };
    Ok(assembly)
}

/// Every generated file whose committed bytes a check compares against: the rule index, and
/// one index per file-register instance.
fn extra_generated(manifest: &Manifest, generated: &HashSet<PathBuf>) -> Vec<PathBuf> {
    let mut out = vec![manifest.rules().dir.join("index.md")];
    out.extend(generated.iter().cloned());
    out.sort();
    out.dedup();
    out
}

// ---------------------------------------------------------------------------------------
// The commands
// ---------------------------------------------------------------------------------------

/// The tree the index holds, as a tree object: what the commit a `commit-msg` hook runs for
/// will hold.
///
/// **The index is the one `GIT_INDEX_FILE` names, where git names one.** `git commit <path>`
/// and `git commit -a` hand the hook a temporary index through it, holding HEAD plus what the
/// commit takes; the default index would be a tree the commit does not have. A plain commit
/// hands the default index as a path relative to the hook's working directory, which is the
/// repository's top level; it is made absolute here, since every git this tool runs starts in
/// the project root, and a project vendored under its repository has a different one.
///
/// `git write-tree` writes the tree object into the repository and refuses an index holding
/// unmerged entries. Nothing references the object, and `git gc` removes it.
fn index_tree(root: &Path) -> Result<String, String> {
    let mut invocation = documentation::git::git(root).args(["write-tree"]);
    if let Some(named) = std::env::var_os("GIT_INDEX_FILE") {
        let named = PathBuf::from(named);
        if named.is_relative() {
            let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
            invocation = invocation.env("GIT_INDEX_FILE", cwd.join(named));
        }
    }
    let out = invocation.output().map_err(|e| {
        format!("the index could not be written as a tree, so the message has no tree to be judged against: {e}")
    })?;
    Ok(String::from_utf8_lossy(&out).trim().to_string())
}

/// Judge the message in one file against the tree the index holds.
///
/// What the `commit-msg` hook calls, so it must be fast enough to sit in front of every
/// commit. It assembles the index's tree at `Depth::Foundation`: the release its quotes verify
/// against and the first three phases, without the content families `commits` runs later.
pub fn commit_message(
    manifest: &Manifest,
    file: &Path,
    checker: Option<&Path>,
) -> Result<ExitCode, String> {
    let raw = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let text = cleaned(&raw);
    let root = manifest.root();
    let checker_rel = checker.and_then(|c| c.strip_prefix(root).ok().map(Path::to_path_buf));
    let mut corpora = Corpora::default();
    let index = index_tree(root)?;
    let tree = commit_tree(
        root,
        &index,
        checker_rel.as_deref(),
        &mut corpora,
        Depth::Foundation,
    )
    .map_err(|Unloadable(why)| format!("the index's tree could not be read: {why}"))?;
    // The message is judged against the index's entity table, and a table built over an
    // incomplete model refuses a reference into what was not read. So the hook does not judge
    // at all then: it names the phase and the commit is not made, which is what the range form
    // does at a failing tip. `git commit --no-verify` is the escape for a fix in progress.
    if let Some(phase) = tree.stopped {
        outln!("{}", phase.stop_line(tree.trouble.len()));
        outln!();
        for finding in &tree.trouble {
            outln!("{finding}");
        }
        outln!(
            "COULD NOT JUDGE: the index leaves the model incomplete, so the message was judged \
             against nothing; fix the staged tree, or commit with --no-verify"
        );
        return Ok(ExitCode::from(2));
    }
    let anchors = tree.anchors();
    let entities = Entities::build(&tree.model, &anchors);
    // **HEAD stands in for the parent tree**, as `commits` uses a commit's first parent. The
    // message being judged belongs to a commit that does not exist yet, whose parent is HEAD,
    // so a message naming the entry this commit deletes resolves exactly as it will once the
    // commit is walked. Without it the hook refuses the shape the gate accepts, and the
    // commit that closes an issue cannot be written at all.
    //
    // **It is read only where the index refused something.** Reading HEAD's tree costs as much
    // as reading the index's, and the hook sits in front of every commit; a message the index
    // already resolves cannot be turned into a finding by a second table, so the common case
    // pays nothing.
    let refused = {
        let probe = Model::from_documents(vec![(message_rel(), text.clone())]);
        let (found, _) =
            check::references::judge(probe.documents(), &entities, &anchors, &tree.inputs());
        !found.is_empty()
    };
    let parent_owned = refused
        .then(|| documentation::git::rev_parse(root, "HEAD"))
        .flatten()
        .and_then(|head| {
            commit_tree(
                root,
                &head,
                checker_rel.as_deref(),
                &mut corpora,
                Depth::References,
            )
            .ok()
            .map(|a| {
                let e = Entities::build(&a.model, &a.anchors());
                (a, e)
            })
        });
    let parent = parent_owned.as_ref().map(|(a, e)| (a, e));
    let findings = relabelled(
        judge_message(&text, &tree, &entities, parent),
        &file.display().to_string(),
    );
    outln!(
        "\nmessage: {} line(s) judged against the index{}",
        text.lines().count(),
        if parent.is_some() {
            " and, for what it refused, HEAD"
        } else {
            ""
        }
    );
    if !findings.is_empty() {
        outln!();
        for finding in &findings {
            outln!("{finding}");
        }
    }
    outln!("{}", verdict(findings.len()));
    Ok(if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// One commit's place in the run, as the summary block names it.
enum Outcome {
    Judged {
        trouble: usize,
    },
    /// A commit before the tip whose tree does not load, or carries findings under the tip
    /// checker. Each is a finding of the run. Its message is still judged wherever the tree
    /// reached the last phase, and `why` says when it was not.
    Failed {
        why: String,
    },
    /// The range's tip, whose tree could not be assembled at all.
    /// The run ends here, and everything walked before it is still printed.
    Unassembled {
        why: String,
    },
    /// The range's tip, whose tree stopped before the last phase: its message was judged
    /// against nothing, and the run ends here.
    Unjudged {
        phase: u8,
        trouble: usize,
    },
}

/// Judge every message in a range against the tree its commit carries.
pub fn commits(
    manifest: &Manifest,
    range: &str,
    checker: Option<&Path>,
) -> Result<ExitCode, String> {
    let root = manifest.root();
    let shas = documentation::git::rev_list(root, range).map_err(|e| {
        format!("{range} does not resolve to a range of commits: {e}\n       a shallow clone resolves no range until it fetches full depth")
    })?;
    if shas.is_empty() {
        outln!("\ncommits: no commit is in range {range}");
        outln!("{}", verdict(0));
        return Ok(ExitCode::SUCCESS);
    }
    // The checker's own directory, project-relative, so a per-commit model reads the tool's
    // own fixtures as data the way `check` does.
    let checker_rel = checker.and_then(|c| c.strip_prefix(root).ok().map(Path::to_path_buf));
    let head = documentation::git::rev_parse(root, "HEAD");

    let mut corpora = Corpora::default();
    let mut summary: Vec<(String, Outcome)> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut previous: Option<(String, Assembly, Entities)> = None;
    // How many findings the tip's own tree carries, `None` where it carries none. The tip's
    // tree failing is the run's own could-not-run rather than a finding.
    let mut tip_trouble: Option<usize> = None;

    let last = shas.last().cloned().unwrap_or_default();
    for sha in &shas {
        let short = &sha[..7.min(sha.len())];
        // **The tip is judged apart.** Its tree failing is the run's own could-not-run rather
        // than a finding, since `check` over the checkout is what reports that tree.
        let is_tip = *sha == last || head.as_deref() == Some(sha.as_str());
        let assembled = commit_tree(
            root,
            sha,
            checker_rel.as_deref(),
            &mut corpora,
            Depth::Judged,
        );
        let tree = match assembled {
            Ok(tree) => tree,
            Err(Unloadable(why)) => {
                if is_tip {
                    // Everything walked before the tip is still reported. A run that printed
                    // an error and nothing else would say nothing about the branch it gates.
                    summary.push((short.to_string(), Outcome::Unassembled { why }));
                    break;
                }
                // **Every commit of the range must load under the tip checker.** A branch that
                // changes the manifest format puts that change in its first commit, or is
                // squashed, per `design@knowledge@a-commit-message-is-a-document`.
                findings.push(Finding::in_file(
                    format!("commit {short}"),
                    format!("this commit's tree does not load under the tip checker: {why}"),
                    "every commit of the range must load; put the change that needs it in the \
                     branch's first commit, or squash the branch",
                ));
                summary.push((
                    short.to_string(),
                    Outcome::Failed {
                        why: format!("its tree does not load: {why}"),
                    },
                ));
                previous = None;
                continue;
            }
        };
        let trouble = tree.trouble.len();
        if trouble > 0 && !is_tip {
            // **Every commit of the range must pass under the tip checker**, so the tree's own
            // findings are the run's, each named by the commit and by the file inside it.
            findings.extend(tree.trouble.iter().cloned().map(|mut f| {
                f.file = PathBuf::from(format!("commit {short}: {}", f.file.display()));
                f
            }));
            let why = match tree.stopped {
                Some(phase) => format!(
                    "its tree stops at phase {} with {trouble} finding(s); its message was \
                     judged against nothing",
                    phase.number()
                ),
                None => format!("its tree fails {trouble} finding(s)"),
            };
            summary.push((short.to_string(), Outcome::Failed { why }));
            if tree.stopped.is_some() {
                // An incomplete table would refuse a reference into what the walk could not
                // read, so the message is judged by nothing. The tree still serves as the next
                // commit's parent: its entities are read, not its verdict.
                let entities = Entities::build(&tree.model, &tree.anchors());
                previous = Some((sha.clone(), tree, entities));
                continue;
            }
        }
        if trouble > 0 && is_tip {
            tip_trouble = Some(trouble);
        }
        // A tip whose tree stopped before the last phase has an incomplete entity table, and
        // a message judged against it would be judged against nothing: the run ends here,
        // naming the phase, as the hook does over the working tree.
        if let (true, Some(phase)) = (is_tip, tree.stopped) {
            summary.push((
                short.to_string(),
                Outcome::Unjudged {
                    phase: phase.number(),
                    trouble,
                },
            ));
            break;
        }

        // The parent model is the previous commit's where the walk followed the parent chain,
        // a failed commit's included, and is built once otherwise — at the range's first
        // commit, and wherever the previous commit is not this one's first parent.
        let first_parent = documentation::git::rev_parse(root, &format!("{sha}^"));
        let parent_owned = match (&previous, &first_parent) {
            (Some((seen, _, _)), Some(parent)) if seen == parent => None,
            (_, Some(parent)) => commit_tree(
                root,
                parent,
                checker_rel.as_deref(),
                &mut corpora,
                Depth::References,
            )
            .ok()
            .map(|a| {
                let e = Entities::build(&a.model, &a.anchors());
                (a, e)
            }),
            (_, None) => None,
        };
        let parent = match (&previous, &first_parent, &parent_owned) {
            (Some((seen, a, e)), Some(p), _) if seen == p => Some((a, e)),
            (_, _, Some((a, e))) => Some((a, e)),
            _ => None,
        };

        // **The message as the commit holds it, cleaned of nothing.** Git applied its own
        // cleanup before the commit existed, so a `#` line here is a line the author wrote and
        // a second pass over it would take bytes of a commit out of the regime.
        let message = documentation::git::commit_message(root, sha).map_err(|e| e.to_string())?;
        let entities = Entities::build(&tree.model, &tree.anchors());
        let found = judge_message(&message, &tree, &entities, parent);
        findings.extend(relabelled(found, &format!("commit {short}")));
        if is_tip || trouble == 0 {
            summary.push((short.to_string(), Outcome::Judged { trouble }));
        }
        previous = Some((sha.clone(), tree, entities));
    }

    let judged = summary
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Judged { .. }))
        .count();
    let unassembled = summary.iter().find_map(|(short, o)| match o {
        Outcome::Unassembled { why } => Some((short.clone(), why.clone())),
        _ => None,
    });
    let failed = summary
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Failed { .. }))
        .count();
    outln!("\ncommits in {range}: {judged} judged, {failed} failed");
    for (short, outcome) in &summary {
        match outcome {
            Outcome::Judged { trouble: 0 } => outln!("  {short} judged"),
            Outcome::Judged { trouble } => outln!(
                "  {short} judged; its own tree fails {trouble} finding(s), which \
                 `cargo knowledge check` reports"
            ),
            Outcome::Failed { why, .. } => outln!("  {short} failed: {why}"),
            Outcome::Unassembled { why } => {
                outln!("  {short} is the range's tip and its tree could not be read: {why}")
            }
            Outcome::Unjudged { phase, trouble } => outln!(
                "  {short} is the range's tip and its tree stops at phase {phase} with {trouble} \
                 finding(s), which `cargo knowledge check` reports; its message was judged \
                 against nothing"
            ),
        }
    }
    if !findings.is_empty() {
        outln!();
        for finding in &findings {
            outln!("{finding}");
        }
    }
    // **The last line never says PASSED over a run that could not conclude.** A reader takes
    // the verdict off the last line, per `path@thaum@CLAUDE.md`, and a tip whose own tree
    // fails leaves the run saying nothing about the branch it gates.
    outln!(
        "{}",
        match (&unassembled, tip_trouble) {
            (Some((short, _)), _) => format!(
                "COULD NOT RUN: the range's tip {short} carries a tree this tool cannot read; \
                 {} finding(s) against the messages before it",
                findings.len()
            ),
            (None, Some(trouble)) => format!(
                "COULD NOT RUN: the range's tip carries a tree with {trouble} finding(s), which \
                 `cargo knowledge check` reports; {} finding(s) against the messages judged",
                findings.len()
            ),
            (None, None) => verdict(findings.len()),
        }
    );
    Ok(if unassembled.is_some() || tip_trouble.is_some() {
        // The range's tip must be judgeable, or the run says nothing about the branch it
        // gates. Its own tree is `check`'s subject, and this is the code that says so.
        ExitCode::from(2)
    } else if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn verdict(n: usize) -> String {
    match n {
        0 => "PASSED: no findings".to_string(),
        1 => "FAILED: 1 finding above".to_string(),
        n => format!("FAILED: {n} findings above"),
    }
}

// ---------------------------------------------------------------------------------------
// The hook
// ---------------------------------------------------------------------------------------

/// Read `core.hooksPath` from this clone's configuration.
///
/// `Ok(None)` is the key being unset, which is an answer; a git that could not run at all is
/// an error, because reporting it as unset would answer a question nobody asked and exit 1
/// where `design@thaum@exit-code-ladder` asks for 2.
fn hooks_path(root: &Path) -> Result<Option<String>, String> {
    let out = documentation::git::git(root)
        .args(["config", "--get", "core.hooksPath"])
        // git exits 1 for a key that is not set, which is an answer.
        .accept(1)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&out).trim().to_string();
    Ok((!text.is_empty()).then_some(text))
}

/// What sits at the script's path: a file, and whether the file system will run it.
///
/// A directory there is neither, and is told apart from an absence because the two need
/// different repairs and `install` must not try to write through one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Script {
    Absent,
    NotAFile,
    Present { executable: bool },
}

fn script_state(root: &Path) -> Script {
    let path = root.join(HOOKS_PATH).join("commit-msg");
    let Ok(meta) = std::fs::symlink_metadata(&path) else {
        return Script::Absent;
    };
    // A symlink is followed, because git follows it too; only what it points at decides.
    let Ok(meta) = (if meta.file_type().is_symlink() {
        std::fs::metadata(&path)
    } else {
        Ok(meta)
    }) else {
        return Script::Absent;
    };
    if !meta.is_file() {
        return Script::NotAFile;
    }
    #[cfg(unix)]
    let executable = {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    };
    #[cfg(not(unix))]
    let executable = true;
    Script::Present { executable }
}

/// Give a file the executable bit, on a platform that has one.
fn make_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .permissions();
        permissions.set_mode(permissions.mode() | 0o755);
        std::fs::set_permissions(path, permissions).map_err(|e| format!("{}: {e}", path.display()))
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(())
    }
}

pub fn hook(manifest: &Manifest, command: &HookCommand) -> Result<ExitCode, String> {
    let root = manifest.root();
    match command {
        HookCommand::Install { force } => {
            // **Every refusal comes before anything is written.** Exit 2 promises the caller
            // that the clone is as they left it, so a configuration written and then a
            // failure on the script is the one shape this ordering exists against — the same
            // reason `index` checks every destination before writing the first.
            let script = root.join(HOOKS_PATH).join("commit-msg");
            let state = script_state(root);
            if let Some(current) = hooks_path(root)? {
                if current != HOOKS_PATH && !force {
                    return Err(format!(
                        "core.hooksPath already names {current}, not {HOOKS_PATH}. \
                         Nothing was written. Pass --force to replace it, or point that \
                         directory at the committed hooks yourself."
                    ));
                }
            }
            if state == Script::NotAFile {
                return Err(format!(
                    "{} is not a file, so the hook cannot be written there. Nothing was \
                     written. Remove it, or put the script there yourself.",
                    script.display()
                ));
            }
            if state == Script::Absent {
                // A tree that does not carry the script gets one. Configuring a hooks path
                // that holds nothing is a hook silently doing nothing, which is the shape
                // every check here exists against.
                std::fs::create_dir_all(root.join(HOOKS_PATH))
                    .map_err(|e| format!("{}: {e}", root.join(HOOKS_PATH).display()))?;
                std::fs::write(&script, COMMIT_MSG_HOOK)
                    .map_err(|e| format!("{}: {e}", script.display()))?;
                make_executable(&script)?;
            }
            documentation::git::git(root)
                .args(["config", "core.hooksPath", HOOKS_PATH])
                .output()
                .map_err(|e| e.to_string())?;
            outln!("core.hooksPath = {HOOKS_PATH}");
            match state {
                Script::Absent => outln!("{HOOKS_PATH}/commit-msg written"),
                Script::NotAFile => unreachable!("refused above"),
                Script::Present { executable } => {
                    if !executable {
                        make_executable(&script)?;
                        outln!("{HOOKS_PATH}/commit-msg made executable");
                    }
                    if std::fs::read_to_string(&script).ok().as_deref() != Some(COMMIT_MSG_HOOK) {
                        outln!(
                            "{HOOKS_PATH}/commit-msg is not the script this tool writes; \
                             it is left as it is"
                        );
                    }
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        HookCommand::Status => {
            let configured = hooks_path(root)?;
            let mut missing = Vec::new();
            match &configured {
                Some(p) if p == HOOKS_PATH => {}
                Some(p) => missing.push(format!("core.hooksPath names {p}, not {HOOKS_PATH}")),
                None => missing.push("core.hooksPath is not set".to_string()),
            }
            match script_state(root) {
                Script::Present { executable: true } => {}
                Script::Present { executable: false } => {
                    missing.push(format!("{HOOKS_PATH}/commit-msg is not executable"))
                }
                Script::NotAFile => missing.push(format!("{HOOKS_PATH}/commit-msg is not a file")),
                Script::Absent => missing.push(format!("{HOOKS_PATH}/commit-msg is not there")),
            }
            if missing.is_empty() {
                outln!("hook: installed — every commit message is judged before it is written");
                return Ok(ExitCode::SUCCESS);
            }
            outln!("hook: not installed — {}", missing.join("; "));
            outln!("      `cargo knowledge hook install` sets it up");
            Ok(ExitCode::FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@knowledge@checker-source-literals-are-data`.

    /// The claim: git's comment block leaves the draft, and every line below it keeps the
    /// number it had in the file the hook was handed.
    ///
    /// The hook is handed the file BEFORE git strips it, so without this every commit would
    /// be judged on `# On branch …` — and each `#` line would open a markdown heading,
    /// splitting the draft into scopes nobody wrote. Removing the lines rather than blanking
    /// them renumbers everything below, and the hook's finding then points at the wrong line
    /// of the file the author is about to re-edit.
    #[test]
    fn a_comment_block_leaves_the_draft_and_the_lines_below_it_do_not_move() {
        let raw = "The subject line\n\
                   \n\
                   # Please enter the commit message for your changes.\n\
                   #\n\
                   # On branch main\n\
                   A body naming `design@a@b`.\n";
        let kept = cleaned(raw);
        assert!(!kept.contains("On branch"), "{kept:?}");
        assert_eq!(
            kept.lines().nth(5),
            Some("A body naming `design@a@b`."),
            "line six is still line six: {kept:?}"
        );
        assert_eq!(kept.lines().count(), 6);
    }

    #[test]
    fn everything_below_the_scissors_line_leaves_the_draft() {
        let raw = "Subject\n\
                   \n\
                   Body.\n\
                   # ------------------------ >8 ------------------------\n\
                   # Do not modify or remove the line above.\n\
                   diff --git a/x b/x\n\
                   +a line the diff holds\n";
        let kept = cleaned(raw);
        assert!(!kept.contains("diff --git"), "{kept:?}");
        assert!(!kept.contains("a line the diff holds"), "{kept:?}");
        assert_eq!(
            kept.lines().take(3).collect::<Vec<_>>(),
            ["Subject", "", "Body."]
        );
    }

    /// The claim: a draft that holds no comment at all keeps every line it has.
    #[test]
    fn a_plain_draft_keeps_every_line_it_has() {
        assert_eq!(cleaned("One\n\nTwo\nThree\n"), "One\n\nTwo\nThree\n");
        assert_eq!(cleaned(""), "");
    }

    /// The claim: the line a finding names is the line of the message, counted from one.
    ///
    /// The message is parsed as a markdown document under a name no reader sees, and the
    /// relabelling replaces the file and nothing else.
    #[test]
    fn relabelling_names_the_commit_and_keeps_the_line() {
        let found = vec![Finding::at("commit-message.md", 4, "what", "action")];
        let out = relabelled(found, "commit abc1234");
        assert_eq!(out[0].location(), "commit abc1234:4");
        assert_eq!(out[0].what, "what");
    }

    #[test]
    fn the_verdict_line_counts_what_was_found() {
        assert_eq!(verdict(0), "PASSED: no findings");
        assert_eq!(verdict(1), "FAILED: 1 finding above");
        assert_eq!(verdict(3), "FAILED: 3 findings above");
    }

    /// The claim: the committed script is the two lines the design states, and the value
    /// `hook install` writes is the directory it sits in.
    #[test]
    fn the_hook_script_is_two_lines_and_names_the_command() {
        let lines: Vec<&str> = COMMIT_MSG_HOOK.lines().collect();
        assert_eq!(lines.len(), 2, "{COMMIT_MSG_HOOK:?}");
        assert_eq!(lines[0], "#!/bin/sh");
        assert!(lines[1].contains("commit-message"), "{}", lines[1]);
        assert!(lines[1].contains("\"$1\""), "{}", lines[1]);
        assert_eq!(HOOKS_PATH, ".githooks");
    }
}
