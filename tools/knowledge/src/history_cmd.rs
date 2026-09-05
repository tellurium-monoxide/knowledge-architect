//! Commit messages under the citation regime, and the hook that judges one before it exists.
//!
//! **A commit message is a document.** It is parsed as one markdown document — the subject
//! line, the blank line and the body — and every rule of the regime runs over it: a `CR:`
//! marker owes its quote inside the message within the distance rule, every reference resolves
//! through the entity table, and the missing-marker lint reads it as it reads any other prose.
//! The decision, and why a message is judged against a tree read from git objects rather than
//! against the working tree, is `knowledge#a-commit-message-is-a-document`.
//!
//! Three commands live here. `commit-message <file>` judges one message against the working
//! tree and is what the `commit-msg` hook calls. `commits <range>` walks the range and judges
//! each message against its own commit's tree. `hook install` and `hook status` are per-clone
//! configuration, read by no check.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Subcommand;

use documentation::check::citations::Release;
use documentation::check::{self, Inputs, Only};
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
/// The suffix is what selects the markdown grammar, per `knowledge#grammars-not-prefixes`, so
/// a message needs one; the findings are relabelled with the commit or the file before they
/// are printed, and no reader ever sees this name.
fn message_rel() -> PathBuf {
    PathBuf::from("commit-message.md")
}

/// A message as git will store it: every comment line dropped, trailing blanks trimmed.
///
/// **The `commit-msg` hook sees the message before git's own cleanup.** The file it is handed
/// still holds the `#` comment block git strips afterwards, and the `--verbose` diff below the
/// scissors line. Judging those would report on text that never reaches the commit, and every
/// `#` line would open a markdown heading, which would split the message into scopes the
/// author did not write. Stripping them here is what makes `commit-message <file>` and
/// `commits` judge the same bytes.
pub fn cleaned(raw: &str) -> String {
    const SCISSORS: &str = "------------------------ >8 ------------------------";
    let mut out = String::new();
    for line in raw.lines() {
        if line.starts_with('#') {
            if line.contains(SCISSORS) {
                break;
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    while out.ends_with("\n\n") {
        out.pop();
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
    /// What is wrong with the tree itself, judged by every family but `corpus` and `changes`.
    ///
    /// Empty for the working tree, which this command does not judge: `check` is what judges
    /// a checkout, and judging it twice would put the tree's findings inside a message's.
    trouble: Vec<Finding>,
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
            // A finding survives only where BOTH trees report it. Comparing the findings
            // rather than the resolutions is what keeps the two arms honest: the union rule
            // has to hold for every shape `references` reports, including a path claim whose
            // kind changed between the trees, and a rule stated over resolutions alone would
            // silently exempt the shapes it did not enumerate.
            mine.into_iter().filter(|f| theirs.contains(f)).collect()
        }
    };
    out.extend(references);
    out.sort_by_key(|f| f.line);
    out
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
// The working tree
// ---------------------------------------------------------------------------------------

/// The working tree, assembled the way `check` assembles it, minus what a message cannot need.
///
/// `changes` and `corpus` are left out because neither reads the model, and the tree's own
/// findings are left out because `check` is what reports them.
fn working_tree(manifest: &Manifest, checker: Option<&Path>) -> Result<Assembly, String> {
    let model = Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let tree = manifest.rules_tree();
    let body_starts_at = manifest.rules().body_starts_at;
    let vendored = std::fs::read_to_string(tree.text())
        .map_err(|e| format!("{}: {e}", tree.text().display()))?;
    let mut releases = HashMap::new();
    releases.insert(None, Release::new(&vendored, body_starts_at));
    let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
    let pinned = rules::release::read_version(&version)
        .get("date")
        .cloned()
        .ok_or("VERSION names no date")?;
    let anchors = Anchors::of(manifest);
    let mut committed = HashMap::new();
    let mut generated_paths = vec![manifest.rules().dir.join("index.md")];
    generated_paths.extend(documentation::index::generated_index_paths(manifest));
    for rel in generated_paths {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&rel)) {
            committed.insert(rel, text);
        }
    }
    let mut configs = HashMap::new();
    for (_, _, home) in anchors.instances() {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&home.config)) {
            configs.insert(home.config.clone(), text);
        }
    }
    let survey = documentation::survey::survey(manifest, &model).map_err(|e| e.to_string())?;
    let queries = check::references::ignore_queries(&model, &anchors);
    let ignored =
        documentation::git::ignored(manifest.root(), &queries).map_err(|e| e.to_string())?;
    let tracked_and_ignored =
        documentation::git::tracked_and_ignored(manifest.root()).map_err(|e| e.to_string())?;
    Ok(Assembly {
        manifest: manifest.clone(),
        model,
        releases,
        pinned,
        committed,
        configs,
        survey,
        ignored,
        tracked_and_ignored,
        trouble: Vec::new(),
    })
}

// ---------------------------------------------------------------------------------------
// One commit's tree
// ---------------------------------------------------------------------------------------

/// Why a commit's tree could not be assembled at all.
///
/// Distinct from a tree that assembled and failed its checks: the first is reported as
/// `manifest does not load`, which is what every pre-migration commit of the branch that
/// introduced this command produces, and the second names its finding count.
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
    listing: Vec<PathBuf>,
    blobs: std::collections::BTreeMap<PathBuf, String>,
}

/// Read one commit's tree: its manifest, its listing, and the text of everything the walk or
/// the inverse assertion reads.
///
/// Deliberately not the whole tree. A path under a skipped directory, an excluded path and a
/// skipped file are read by no family, and the corpus alone is two megabytes at every commit
/// in the range.
fn read_tree(root: &Path, sha: &str, generated_extra: &[PathBuf]) -> Result<FromTree, Unloadable> {
    let listing = documentation::git::tree_files(root, sha)
        .map_err(|e| Unloadable(format!("its tree could not be listed: {e}")))?;
    let manifest_rel = PathBuf::from(MANIFEST_NAME);
    let declaration = documentation::git::blobs(root, sha, std::slice::from_ref(&manifest_rel))
        .map_err(|e| Unloadable(format!("its {MANIFEST_NAME} could not be read: {e}")))?;
    let Some(text) = declaration.get(&manifest_rel) else {
        return Err(Unloadable(format!("its tree holds no {MANIFEST_NAME}")));
    };
    let manifest = Manifest::parse(root, text).map_err(Unloadable)?;
    let walk = manifest.walk();
    let mut wanted: Vec<PathBuf> = listing
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
        blobs,
    })
}

/// Assemble one commit's tree into a model, and judge it with every family but `corpus` and
/// `changes`.
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
) -> Result<Assembly, Unloadable> {
    // The rule index is generated and named by the manifest rather than derived, so it is
    // asked for by hand; every other generated index comes from the register instances.
    let read = read_tree(root, sha, &[])?;
    let manifest = read.manifest;
    let anchors = Anchors::of(&manifest);
    let generated = documentation::index::generated_index_paths(&manifest);
    let walked =
        documentation::walk::live_files(Path::new(""), manifest.walk(), &read.listing, &generated);
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
    let model = Model::from_documents_under(docs, checker);

    let mut releases = HashMap::new();
    let rules_dir = manifest.rules().dir.clone();
    let corpus_rel = rules_dir.join(&manifest.rules().text);
    let body_starts_at = manifest.rules().body_starts_at;
    if let Some(text) = blobs.get(&corpus_rel) {
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
    if !pins.is_empty() {
        let past = rules_dir.join(&manifest.rules().past);
        let wanted: Vec<PathBuf> = pins.iter().map(|d| past.join(format!("{d}.txt"))).collect();
        let archived = documentation::git::blobs(root, sha, &wanted)
            .map_err(|e| Unloadable(format!("its archive could not be read: {e}")))?;
        for (date, rel) in pins.iter().zip(&wanted) {
            if let Some(text) = archived.get(rel) {
                releases.insert(Some(date.clone()), Release::new(text, body_starts_at));
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
    let survey = documentation::survey::from_listing(&manifest, &model, &read.listing, |rel| {
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
    };
    let report = check::run(
        &assembly.model,
        &assembly.manifest,
        &assembly.inputs(),
        Only::EVERYTHING
            .without(Only::CORPUS)
            .without(Only::CHANGES),
    );
    assembly.trouble = report.findings;
    for rel in missing {
        assembly.trouble.push(Finding::in_file(
            &rel,
            "this tree entry holds no text the tool can read",
            "the walk reads it as a document, so a binary blob at this path leaves a live \
             document out of every check",
        ));
    }
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

/// Judge the message in one file against the working tree.
///
/// What the `commit-msg` hook calls, so it must be fast enough to sit in front of every
/// commit. What it costs is one model of the tree, which is what `check` already pays.
pub fn commit_message(
    manifest: &Manifest,
    file: &Path,
    checker: Option<&Path>,
) -> Result<ExitCode, String> {
    let raw = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let text = cleaned(&raw);
    let tree = working_tree(manifest, checker)?;
    let anchors = tree.anchors();
    let entities = Entities::build(&tree.model, &anchors);
    let findings = relabelled(
        judge_message(&text, &tree, &entities, None),
        &file.display().to_string(),
    );
    println!(
        "\nmessage: {} line(s) judged against the working tree",
        text.lines().count()
    );
    if !findings.is_empty() {
        println!();
        for finding in &findings {
            println!("{finding}");
        }
    }
    println!("{}", verdict(findings.len()));
    Ok(if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// One commit's place in the run, as the summary block names it.
enum Outcome {
    Judged { trouble: usize },
    Skipped { why: String },
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
        println!("\ncommits: no commit is in range {range}");
        println!("{}", verdict(0));
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
    let mut head_failed = false;

    let last = shas.last().cloned().unwrap_or_default();
    for sha in &shas {
        let short = &sha[..7.min(sha.len())];
        // **HEAD is never skipped.** A range whose tip could not be judged would pass with
        // every commit skipped, which is the vacuous run the summary counts exist against.
        let never_skipped = *sha == last || head.as_deref() == Some(sha.as_str());
        let assembled = commit_tree(root, sha, checker_rel.as_deref(), &mut corpora);
        let tree = match assembled {
            Ok(tree) => tree,
            Err(Unloadable(why)) => {
                if never_skipped {
                    return Err(format!(
                        "the range's tip {short} could not be assembled: {why}"
                    ));
                }
                summary.push((
                    short.to_string(),
                    Outcome::Skipped {
                        why: format!("manifest does not load: {why}"),
                    },
                ));
                previous = None;
                continue;
            }
        };
        let trouble = tree.trouble.len();
        if trouble > 0 && !never_skipped {
            summary.push((
                short.to_string(),
                Outcome::Skipped {
                    why: format!("tree fails {trouble} finding(s)"),
                },
            ));
            // The tree still becomes the next commit's parent: its entities are read, not
            // its verdict, and a message that names an entry this commit deleted needs them.
            let entities = Entities::build(&tree.model, &tree.anchors());
            previous = Some((sha.clone(), tree, entities));
            continue;
        }
        if trouble > 0 {
            head_failed = true;
        }

        // The parent model is the previous commit's where the walk followed the parent chain,
        // and is built once otherwise — at the range's first commit, and after a skip.
        let first_parent = documentation::git::rev_parse(root, &format!("{sha}^"));
        let parent_owned = match (&previous, &first_parent) {
            (Some((seen, _, _)), Some(parent)) if seen == parent => None,
            (_, Some(parent)) => commit_tree(root, parent, checker_rel.as_deref(), &mut corpora)
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

        let raw = documentation::git::commit_message(root, sha).map_err(|e| e.to_string())?;
        let entities = Entities::build(&tree.model, &tree.anchors());
        let found = judge_message(&cleaned(&raw), &tree, &entities, parent);
        findings.extend(relabelled(found, &format!("commit {short}")));
        summary.push((short.to_string(), Outcome::Judged { trouble }));
        previous = Some((sha.clone(), tree, entities));
    }

    let judged = summary
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Judged { .. }))
        .count();
    let skipped = summary.len() - judged;
    println!("\ncommits in {range}: {judged} judged, {skipped} skipped");
    for (short, outcome) in &summary {
        match outcome {
            Outcome::Judged { trouble: 0 } => println!("  {short} judged"),
            Outcome::Judged { trouble } => println!(
                "  {short} judged; its own tree fails {trouble} finding(s), which \
                 `cargo knowledge check` reports"
            ),
            Outcome::Skipped { why } => println!("  {short} skipped: {why}"),
        }
    }
    if !findings.is_empty() {
        println!();
        for finding in &findings {
            println!("{finding}");
        }
    }
    println!("{}", verdict(findings.len()));
    Ok(if head_failed {
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

/// Read `core.hooksPath` from this clone's configuration. `None` when it is unset.
fn hooks_path(root: &Path) -> Option<String> {
    let out = documentation::git::git(root)
        .args(["config", "--get", "core.hooksPath"])
        .accept(1)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out).trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// Whether the committed script is there and the file system will run it.
fn script_state(root: &Path) -> (bool, bool) {
    let path = root.join(HOOKS_PATH).join("commit-msg");
    let Ok(meta) = std::fs::metadata(&path) else {
        return (false, false);
    };
    #[cfg(unix)]
    let executable = {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    };
    #[cfg(not(unix))]
    let executable = meta.is_file();
    (meta.is_file(), executable)
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
            if let Some(current) = hooks_path(root) {
                if current != HOOKS_PATH && !force {
                    return Err(format!(
                        "core.hooksPath already names {current}, not {HOOKS_PATH}. \
                         Nothing was written. Pass --force to replace it, or point that \
                         directory at the committed hooks yourself."
                    ));
                }
            }
            documentation::git::git(root)
                .args(["config", "core.hooksPath", HOOKS_PATH])
                .output()
                .map_err(|e| e.to_string())?;
            println!("core.hooksPath = {HOOKS_PATH}");
            let script = root.join(HOOKS_PATH).join("commit-msg");
            let (present, executable) = script_state(root);
            if !present {
                // A tree that does not carry the script gets one. Configuring a hooks path
                // that holds nothing is a hook silently doing nothing, which is the shape
                // every check here exists against.
                std::fs::create_dir_all(root.join(HOOKS_PATH)).map_err(|e| e.to_string())?;
                std::fs::write(&script, COMMIT_MSG_HOOK).map_err(|e| e.to_string())?;
                make_executable(&script)?;
                println!("{HOOKS_PATH}/commit-msg written");
            } else {
                if !executable {
                    make_executable(&script)?;
                    println!("{HOOKS_PATH}/commit-msg made executable");
                }
                if std::fs::read_to_string(&script).ok().as_deref() != Some(COMMIT_MSG_HOOK) {
                    println!(
                        "{HOOKS_PATH}/commit-msg is not the script this tool writes; \
                         it is left as it is"
                    );
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        HookCommand::Status => {
            let configured = hooks_path(root);
            let (present, executable) = script_state(root);
            let mut missing = Vec::new();
            match &configured {
                Some(p) if p == HOOKS_PATH => {}
                Some(p) => missing.push(format!("core.hooksPath names {p}, not {HOOKS_PATH}")),
                None => missing.push("core.hooksPath is not set".to_string()),
            }
            if !present {
                missing.push(format!("{HOOKS_PATH}/commit-msg is not there"));
            } else if !executable {
                missing.push(format!("{HOOKS_PATH}/commit-msg is not executable"));
            }
            if missing.is_empty() {
                println!("hook: installed — every commit message is judged before it is written");
                return Ok(ExitCode::SUCCESS);
            }
            println!("hook: not installed — {}", missing.join("; "));
            println!("      `cargo knowledge hook install` sets it up");
            Ok(ExitCode::FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `knowledge#checker-source-literals-are-data`.

    /// The claim: git's comment block never reaches the document.
    ///
    /// The hook is handed the file BEFORE git strips it, so without this every commit would
    /// be judged on `# On branch …` and on the `--verbose` diff — and each `#` line would
    /// open a markdown heading, splitting the message into scopes nobody wrote.
    #[test]
    fn a_comment_block_and_a_verbose_diff_leave_the_message() {
        let raw = "The subject line\n\
                   \n\
                   A body naming `design@a@b`.\n\
                   # Please enter the commit message for your changes.\n\
                   #\n\
                   # On branch main\n";
        assert_eq!(
            cleaned(raw),
            "The subject line\n\nA body naming `design@a@b`.\n"
        );
    }

    #[test]
    fn everything_below_the_scissors_line_leaves_the_message() {
        let raw = "Subject\n\
                   \n\
                   Body.\n\
                   # ------------------------ >8 ------------------------\n\
                   # Do not modify or remove the line above.\n\
                   diff --git a/x b/x\n\
                   +a line the diff holds\n";
        let kept = cleaned(raw);
        assert_eq!(kept, "Subject\n\nBody.\n");
        assert!(!kept.contains("diff --git"), "{kept}");
    }

    /// The claim: a message that holds no comment at all is unchanged but for its trailing
    /// blank lines, which `%B` supplies and which would otherwise move every finding's line.
    #[test]
    fn a_plain_message_keeps_every_line_it_has() {
        assert_eq!(cleaned("One\n\nTwo\nThree\n\n\n"), "One\n\nTwo\nThree\n");
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
