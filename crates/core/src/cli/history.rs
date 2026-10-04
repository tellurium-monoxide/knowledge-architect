//! Commit messages under the regime, judged by `commits` over a range.
//!
//! **A commit message is a document.** It is parsed as one markdown document — the subject
//! line, the blank line and the body — and every rule of the regime runs over it: every
//! reference resolves through the entity table, and each registered extension judges it against
//! the tree it prepared for that commit.
//! The decision, and why a message is judged against a tree read from git objects rather than
//! against the working tree, is `design@core@a-commit-message-is-a-document`.
//!
//! One command lives here: `commits <range>` walks the range and judges each commit's message
//! and tree against that commit's own tree.

use super::output::outln;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::check::{self, Inputs};
use crate::entity::{Anchors, Entities};
use crate::extension::{CommitTree, Extension, Prepared, Purpose, Tree};
use crate::manifest::MANIFEST_NAME;
use crate::survey::Survey;
use crate::{Finding, Manifest, Model};

// ---------------------------------------------------------------------------------------
// The message as a document
// ---------------------------------------------------------------------------------------

/// The rel path a message is parsed under.
///
/// The suffix is what selects the markdown grammar, per `design@core@grammars-not-prefixes`, so
/// a message needs one; the findings are relabelled with the commit or the file before they
/// are printed, and no reader ever sees this name.
fn message_rel() -> PathBuf {
    PathBuf::from("commit-message.md")
}

/// Everything one tree contributes to judging a message written against it.
///
/// Held together because `Inputs` borrows every field of it: a check is a pure function over
/// the model, so whatever it needs about the tree is gathered once and handed in.
struct Assembly {
    manifest: Manifest,
    model: Model,
    /// Each extension prepared for this tree, with its check names; empty for a tree assembled
    /// for its references alone.
    prepared: Vec<(&'static [&'static str], Box<dyn Prepared>)>,
    committed: HashMap<PathBuf, String>,
    configs: HashMap<PathBuf, String>,
    survey: Survey,
    ignored: HashSet<String>,
    tracked_and_ignored: Vec<PathBuf>,
    /// What is wrong with the tree itself: every check but `corpus` and `changes` at
    /// `Depth::Judged`, nothing at `Depth::References`.
    trouble: Vec<Finding>,
    /// The phase the tree's run stopped in, where it stopped before the last: its entity
    /// table is then incomplete, and a message judged against it is judged against nothing.
    stopped: Option<check::Phase>,
}

impl Assembly {
    fn anchors(&self) -> Anchors {
        Anchors::of(&self.manifest, &self.survey.present)
    }

    fn inputs(&self) -> Inputs<'_> {
        Inputs {
            committed: &self.committed,
            configs: &self.configs,
            present: &self.survey.present,
            directories: &self.survey.directories,
            outside: &self.survey.outside,
            ignored: &self.ignored,
            tracked_and_ignored: &self.tracked_and_ignored,
            refused: &self.survey.refused,
            links: &self.survey.links,
            // A commit's installed files are not compared: the running binary ships its own
            // version's text, and an older commit's installed set would fail against it with no
            // repair a commit in history can take. `check` on the working tree enforces them,
            // per `design@core@owned-namespace-check`.
            installed: &[],
            shipped: &[],
        }
    }
}

/// Judge one message against a tree, and optionally against a second one for its references.
///
/// **References resolve against either tree, every other rule against the first.** A commit
/// that closes an issue deletes the entry file and names it in the message; against its own
/// tree alone every such message would dangle. An extension's rules have no such asymmetry:
/// what a message says about its subject is judged against what the commit's own tree holds,
/// and nothing about the parent bears on it.
///
/// **A lint stands where either tree holds what its span names**, so its findings are the union
/// of both judgements, per `check::references::Part`. `ignored` answers the message's own
/// spellings, which neither tree's batch asked about: the ignore rules are the working tree's
/// for both, so one batch serves the two.
fn judge_message(
    text: &str,
    primary: &Assembly,
    primary_entities: &Entities,
    parent: Option<(&Assembly, &Entities)>,
    ignored: &HashSet<String>,
) -> Vec<Finding> {
    use check::references::{judge_part, Part};
    let model = Model::from_documents(vec![(message_rel(), text.to_string())]);
    let doc = &model.documents()[0];
    let mut out = Vec::new();

    for (_, prepared) in &primary.prepared {
        out.extend(prepared.check_message(doc));
    }

    let with_message = |assembly: &Assembly| -> HashSet<String> {
        assembly.ignored.union(ignored).cloned().collect()
    };
    let primary_ignored = with_message(primary);
    let primary_inputs = Inputs {
        ignored: &primary_ignored,
        ..primary.inputs()
    };
    let anchors = primary.anchors();
    let (mine, _) = judge_part(
        model.documents(),
        primary_entities,
        &anchors,
        &primary_inputs,
        Part::References,
    );
    let (mut lints, _) = judge_part(
        model.documents(),
        primary_entities,
        &anchors,
        &primary_inputs,
        Part::Lints,
    );
    let references = match parent {
        None => mine,
        Some((other, other_entities)) => {
            let other_anchors = other.anchors();
            let other_ignored = with_message(other);
            let other_inputs = Inputs {
                ignored: &other_ignored,
                ..other.inputs()
            };
            let (theirs, _) = judge_part(
                model.documents(),
                other_entities,
                &other_anchors,
                &other_inputs,
                Part::References,
            );
            let (their_lints, _) = judge_part(
                model.documents(),
                other_entities,
                &other_anchors,
                &other_inputs,
                Part::Lints,
            );
            for f in their_lints {
                if !lints.contains(&f) {
                    lints.push(f);
                }
            }
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
    out.extend(lints);
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

/// Everything read from one commit's tree, before the checks run over it.
struct FromTree {
    manifest: Manifest,
    /// Every path of the tree, the symlink and gitlink entries included.
    listing: Vec<PathBuf>,
    /// The symlink and gitlink entries, which are read as no document.
    links: Vec<crate::git::Entry>,
    blobs: std::collections::BTreeMap<PathBuf, String>,
    /// The wanted blobs whose bytes are not UTF-8, as the survey reads them: binary, or text
    /// decoded lossily, as `check` reads the same file on disk. Read by the survey alone; a
    /// walked path here is still a document with no text.
    not_text: std::collections::BTreeMap<PathBuf, crate::survey::Outside>,
}

/// Read one commit's tree: its manifest, configured against the extensions, its listing, and
/// the text of everything the walk or the inverse assertion reads.
///
/// Deliberately not the whole tree. A path under a skipped directory, an excluded path and a
/// skipped file are read by no family, and the corpus alone is two megabytes at every commit
/// in the range. **A manifest carrying a phase-1 complaint reads nothing further**: the tree
/// stops at phase 1, and nothing it holds is judged or read as a parent.
fn read_tree(
    root: &Path,
    sha: &str,
    generated_extra: &[PathBuf],
    extensions: &mut [Box<dyn Extension>],
) -> Result<FromTree, Unloadable> {
    let entries = crate::git::tree_entries(root, sha)
        .map_err(|e| Unloadable(format!("its tree could not be listed: {e}")))?;
    let listing: Vec<PathBuf> = entries.iter().map(|e| e.rel.clone()).collect();
    let links: Vec<crate::git::Entry> = entries
        .iter()
        .filter(|e| e.kind != crate::git::EntryKind::File)
        .cloned()
        .collect();
    let files: Vec<PathBuf> = entries
        .into_iter()
        .filter(|e| e.kind == crate::git::EntryKind::File)
        .map(|e| e.rel)
        .collect();
    let manifest_rel = PathBuf::from(MANIFEST_NAME);
    let declaration = crate::git::blobs(root, sha, std::slice::from_ref(&manifest_rel))
        .map_err(|e| Unloadable(format!("its {MANIFEST_NAME} could not be read: {e}")))?;
    let Some(text) = declaration.get(&manifest_rel) else {
        return Err(Unloadable(format!("its tree holds no {MANIFEST_NAME}")));
    };
    let mut manifest = Manifest::parse(root, text).map_err(Unloadable)?;
    // Configured against this commit's own manifest, so the files its extensions generate
    // are left out of its walk and read as committed, and a table it refuses is its phase 1.
    crate::extension::configure(&mut manifest, extensions);
    if !manifest.complaints().is_empty() {
        return Ok(FromTree {
            manifest,
            listing,
            links,
            blobs: Default::default(),
            not_text: Default::default(),
        });
    }
    let walk = manifest.walk();
    let mut wanted: Vec<PathBuf> = files
        .iter()
        .filter(|rel| !crate::walk::skipped(rel, walk) && !walk.skip_files.contains(rel))
        .cloned()
        .collect();
    for rel in generated_extra {
        if listing.contains(rel) {
            wanted.push(rel.clone());
        }
    }
    wanted.sort();
    wanted.dedup();
    let mut blobs = std::collections::BTreeMap::new();
    let mut not_text = std::collections::BTreeMap::new();
    for (rel, bytes) in crate::git::blob_bytes(root, sha, &wanted)
        .map_err(|e| Unloadable(format!("its blobs could not be read: {e}")))?
    {
        match String::from_utf8(bytes) {
            Ok(text) => {
                blobs.insert(rel, text);
            }
            Err(e) => {
                not_text.insert(rel, crate::survey::Outside::from_bytes(e.as_bytes()));
            }
        }
    }
    Ok(FromTree {
        manifest,
        listing,
        links,
        blobs,
        not_text,
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
    /// Everything: every check of the core, and each extension prepared for a commit.
    Judged,
}

/// Assemble one tree into a model, and judge it as far as `depth` asks.
///
/// **An extension is prepared for a commit, not for a checkout.** A check whose subject is
/// filesystem state has no subject in a commit's tree, and one that would read the whole
/// archive at every step of the range decides nothing about whether a message's references
/// resolve, which is the question this assembly exists to answer; the extension leaves both
/// out, per `design@core@a-commit-message-is-a-document`.
fn commit_tree(
    root: &Path,
    sha: &str,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
    depth: Depth,
) -> Result<Assembly, Unloadable> {
    let read = read_tree(root, sha, &[], extensions)?;
    let manifest = read.manifest;
    if !manifest.complaints().is_empty() {
        // Phase 1 failed, so no blob was read and nothing is assembled: the tree is judged no
        // further, and it serves as no parent, per
        // `design@core@a-commit-message-is-a-document`; the range walk keeps no parent for
        // it. The early return in `read_tree` changes no verdict and is what saves reading the
        // blobs.
        let model = Model::from_documents(Vec::new());
        let survey = crate::survey::from_listing(&manifest, &model, &[], &[], |_| {
            crate::survey::Outside::Missing
        });
        let trouble = if depth == Depth::Judged {
            manifest.complaints().to_vec()
        } else {
            Vec::new()
        };
        return Ok(Assembly {
            manifest,
            model,
            prepared: Vec::new(),
            committed: HashMap::new(),
            configs: HashMap::new(),
            survey,
            ignored: HashSet::new(),
            tracked_and_ignored: Vec::new(),
            trouble,
            stopped: Some(check::Phase::Resolution),
        });
    }
    // The commit's own listing places its milestone anchors, so a message is judged against
    // the anchors of the tree it commits rather than of the working tree.
    let anchors = Anchors::of(&manifest, &read.listing);
    let generated = crate::index::generated_paths(&manifest, &read.listing);
    // The walk reads through no symlink and no gitlink, so the files alone are walked.
    let files: Vec<PathBuf> = read
        .listing
        .iter()
        .filter(|rel| !read.links.iter().any(|e| e.rel == **rel))
        .cloned()
        .collect();
    // An installed file leaves the walk as a generated one does, per
    // `design@core@owned-namespace-check`; a commit's installed files are not compared, so their
    // blobs are not read.
    let installed: Vec<PathBuf> = files
        .iter()
        .filter(|f| manifest.owned(f))
        .cloned()
        .collect();
    let mut excluded = generated.clone();
    excluded.extend(installed.iter().cloned());
    let walked = crate::walk::live_files(Path::new(""), manifest.walk(), &files, &excluded);
    let mut blobs = read.blobs;
    // The generated indexes and the per-instance options sit outside the walk and are read by
    // the caller, exactly as `check` reads them off the filesystem.
    let mut extra: Vec<PathBuf> = generated.iter().cloned().collect();
    for (_, _, home) in anchors.instances() {
        extra.push(home.config.clone());
    }
    extra.retain(|rel| read.listing.contains(rel) && !blobs.contains_key(rel));
    if !extra.is_empty() {
        let more = crate::git::blobs(root, sha, &extra)
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

    let judging = depth == Depth::Judged;

    let mut committed = HashMap::new();
    for rel in extra_generated(&generated) {
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
    let survey = crate::survey::from_listing(
        &manifest,
        &model,
        &read.listing,
        &read.links,
        |rel| match blobs.get(rel) {
            // The same binary test `check` applies on disk: a NUL is valid UTF-8, so a blob
            // that decoded may still be binary.
            Some(text) => crate::survey::Outside::from_bytes(text.as_bytes()),
            None => read.not_text.get(rel).cloned().unwrap_or_else(|| {
                crate::survey::Outside::Unreadable("the tree holds no blob for it".to_string())
            }),
        },
    );
    // **The ignore rules are the working tree's.** `git check-ignore` reads the `.gitignore`
    // files that are on disk and has no form that asks a historical tree, so a commit whose
    // ignore rules differ from today's is judged against today's. What that can cost is a
    // path reference asserted where the commit's own rules would have exempted it — a finding
    // rather than a silence, which fails the range; a branch that changes its ignore rules
    // orders its commits for it, per `design@core@a-commit-message-is-a-document`.
    let queries = check::references::ignore_queries(&model, &anchors);
    let ignored = crate::git::ignored(root, &queries)
        .map_err(|e| Unloadable(format!("its ignore rules could not be asked: {e}")))?;

    let mut assembly = Assembly {
        manifest,
        model,
        prepared: Vec::new(),
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
            Ok(()) => {
                // The foundation passed, so each extension reads what it needs from this
                // commit's git objects now, and a tree that stopped earlier read nothing.
                let commit = CommitTree::new(root, sha, read.listing, blobs);
                for extension in extensions.iter_mut() {
                    let prepared = extension
                        .prepare(
                            &assembly.manifest,
                            &assembly.model,
                            Tree::Commit(&commit),
                            Purpose::Commit,
                        )
                        .map_err(Unloadable)?;
                    assembly.prepared.push((extension.checks(), prepared));
                }
                let with: Vec<(&[&'static str], &dyn Prepared)> = assembly
                    .prepared
                    .iter()
                    .map(|(c, p)| (*c, p.as_ref()))
                    .collect();
                check::run_with(
                    &assembly.model,
                    &assembly.manifest,
                    &assembly.inputs(),
                    &with,
                )
                .findings
            }
        };
    Ok(assembly)
}

/// Every generated file whose committed bytes a check compares against: each file an
/// extension generates, and one index per file-register instance.
fn extra_generated(generated: &HashSet<PathBuf>) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = generated.iter().cloned().collect();
    out.sort();
    out.dedup();
    out
}

// ---------------------------------------------------------------------------------------
// The commands
// ---------------------------------------------------------------------------------------

/// One commit's place in the run, as the summary block names it.
enum Outcome {
    /// Its tree passed, and its message was judged.
    Judged,
    /// Its tree does not load, or carries findings under the tip checker. Each is a finding of
    /// the run. Its message is still judged wherever the tree reached the last phase, and
    /// `why` says when it was not.
    Failed { why: String },
}

/// Judge every commit of a range, its tree and its message, alike.
///
/// **No commit of the range is judged apart**, the last one included: a tree that fails is a
/// finding naming the commit, whichever commit it is, per
/// `design@core@a-commit-message-is-a-document`.
pub(super) fn commits(
    manifest: &Manifest,
    range: &str,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let root = manifest.root();
    let shas = crate::git::rev_list(root, range).map_err(|e| {
        format!("{range} does not resolve to a range of commits: {e}\n       a shallow clone resolves no range until it fetches full depth")
    })?;
    if shas.is_empty() {
        outln!("\ncommits: no commit is in range {range}");
        outln!("{}", verdict(0));
        return Ok(ExitCode::SUCCESS);
    }
    // The checker's own directories, project-relative, so a per-commit model reads the tool's
    // own fixtures as data the way `check` does.
    let checker_rel: Vec<PathBuf> = checker
        .iter()
        .filter_map(|c| c.strip_prefix(root).ok().map(Path::to_path_buf))
        .collect();
    let checker_rel: Vec<&Path> = checker_rel.iter().map(PathBuf::as_path).collect();

    let mut summary: Vec<(String, Outcome)> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut previous: Option<(String, Assembly, Entities)> = None;

    for sha in &shas {
        let short = &sha[..7.min(sha.len())];
        let assembled = commit_tree(root, sha, &checker_rel, extensions, Depth::Judged);
        let tree = match assembled {
            Ok(tree) => tree,
            Err(Unloadable(why)) => {
                // **Every commit of the range must load under the tip checker.** A branch that
                // changes the manifest format puts that change in its first commit, or is
                // squashed, per `design@core@a-commit-message-is-a-document`.
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
        if trouble > 0 {
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
            if tree.stopped == Some(check::Phase::Resolution) {
                // Phase 1 failed and nothing of the tree was read, so it serves as no parent.
                // An empty tree kept as a parent would not be the same: a finding is kept only
                // when the parent refuses the reference too, and an empty tree accepts some a
                // real one refuses, such as an escape naming a path the tree holds.
                previous = None;
                continue;
            }
            if tree.stopped.is_some() {
                // An incomplete table would refuse a reference into what the walk could not
                // read, so the message is judged by nothing. The tree still serves as the next
                // commit's parent: its entities are read, not its verdict.
                let entities = Entities::build(&tree.model, &tree.anchors());
                previous = Some((sha.clone(), tree, entities));
                continue;
            }
        }

        // The parent model is the previous commit's where the walk followed the parent chain,
        // a failed commit's included, and is built once otherwise — at the range's first
        // commit, and wherever the previous commit is not this one's first parent.
        let first_parent = crate::git::rev_parse(root, &format!("{sha}^"));
        let parent_owned = match (&previous, &first_parent) {
            (Some((seen, _, _)), Some(parent)) if seen == parent => None,
            (_, Some(parent)) => {
                commit_tree(root, parent, &checker_rel, extensions, Depth::References)
                    .ok()
                    // A parent whose phase 1 fails was not read, and serves as no parent.
                    .filter(|a| a.stopped != Some(check::Phase::Resolution))
                    .map(|a| {
                        let e = Entities::build(&a.model, &a.anchors());
                        (a, e)
                    })
            }
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
        let message = crate::git::commit_message(root, sha).map_err(|e| e.to_string())?;
        let entities = Entities::build(&tree.model, &tree.anchors());
        let queries = check::references::ignore_queries(
            &Model::from_documents(vec![(message_rel(), message.clone())]),
            &tree.anchors(),
        );
        let ignored = crate::git::ignored(root, &queries)
            .map_err(|e| format!("the ignore rules could not be asked: {e}"))?;
        let found = judge_message(&message, &tree, &entities, parent, &ignored);
        findings.extend(relabelled(found, &format!("commit {short}")));
        // **Read from this commit's own manifest**, as everything a commit is judged against
        // is, per `design@core@a-commit-message-is-a-document`.
        let cited = if tree.manifest.refuses_branch_shas() {
            branch_sha_findings(short, &message, &tree, &shas, range)
        } else {
            Vec::new()
        };
        let citations = cited.len();
        findings.extend(cited);
        if trouble == 0 {
            summary.push((
                short.to_string(),
                if citations == 0 {
                    Outcome::Judged
                } else {
                    Outcome::Failed {
                        why: format!("it cites the range by SHA {citations} time(s)"),
                    }
                },
            ));
        }
        previous = Some((sha.clone(), tree, entities));
    }

    let judged = summary
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Judged))
        .count();
    let failed = summary.len() - judged;
    outln!("\ncommits in {range}: {judged} judged, {failed} failed");
    for (short, outcome) in &summary {
        match outcome {
            Outcome::Judged => outln!("  {short} judged"),
            Outcome::Failed { why } => outln!("  {short} failed: {why}"),
        }
    }
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

/// Every citation of a commit of the range in `text`: a run of 7 to 64 lowercase hex digits,
/// with no ASCII letter, digit or underscore on either side, that is a prefix of one of `shas`.
/// Each is the line, counted from one, the run as written, and the full SHA it names.
///
/// **Only the range's commits are refused**, per `design@core@branch-shas-are-refused`: they are
/// the ones a rebase merge gives a new SHA. A SHA on the main branch, or of another project, is
/// never a prefix of one of them, except by a collision of its first 7 digits, so a citation of
/// either passes.
fn branch_sha_citations<'a>(text: &str, shas: &'a [String]) -> Vec<(u32, String, &'a str)> {
    fn is_word(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
    let mut out = Vec::new();
    for (i, line) in text.split('\n').enumerate() {
        let bytes = line.as_bytes();
        let mut start = 0;
        while start < bytes.len() {
            if !is_word(bytes[start]) {
                start += 1;
                continue;
            }
            // A word ends at the first byte that is not ASCII alphanumeric or `_`, so both
            // ends of the slice sit on ASCII bytes and the slice is valid UTF-8.
            let mut end = start;
            while end < bytes.len() && is_word(bytes[end]) {
                end += 1;
            }
            let word = &line[start..end];
            let hex = word.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
            // 64 digits is a SHA under SHA-256.
            if hex && (7..=64).contains(&word.len()) {
                if let Some(sha) = shas.iter().find(|s| s.starts_with(word)) {
                    out.push((i as u32 + 1, word.to_string(), sha.as_str()));
                }
            }
            start = end;
        }
    }
    out
}

/// The findings for every citation of a commit of the range in one commit's message and in its
/// tree's documents, each named by the commit and by the site.
fn branch_sha_findings(
    short: &str,
    message: &str,
    tree: &Assembly,
    shas: &[String],
    range: &str,
) -> Vec<Finding> {
    let finding = |file: String, (line, word, sha): (u32, String, &str)| {
        Finding::at(
            file,
            line,
            format!(
                "`{word}` cites commit {}, a commit of the range {range}, by its SHA; a rebase \
                 merge gives that commit a new SHA, and the citation then names nothing",
                &sha[..7.min(sha.len())]
            ),
            "name that commit by its subject; a SHA on the main branch may be cited",
        )
    };
    let mut out: Vec<Finding> = branch_sha_citations(message, shas)
        .into_iter()
        .map(|c| finding(format!("commit {short}"), c))
        .collect();
    for doc in tree.model.documents() {
        out.extend(
            branch_sha_citations(&doc.text, shas)
                .into_iter()
                .map(|c| finding(format!("commit {short}: {}", doc.rel.display()), c)),
        );
    }
    out
}

fn verdict(n: usize) -> String {
    match n {
        0 => "PASSED: no findings".to_string(),
        1 => "FAILED: 1 finding above".to_string(),
        n => format!("FAILED: {n} findings above"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@core@checker-source-literals-are-data`.

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

    /// The claim: a run of 7 to 64 lowercase hex digits bounded by non-word bytes is a citation
    /// exactly when it prefixes a SHA of the range. Mutations: dropping the left or the right
    /// boundary fails the embedded cases, lowering the minimum to 6 fails the six-digit case,
    /// and matching any hex run fails the outside-the-range case.
    #[test]
    fn a_citation_is_a_bounded_hex_prefix_of_a_sha_of_the_range() {
        let shas = vec![
            "0123456789abcdef0123456789abcdef01234567".to_string(),
            "fedcba9876543210fedcba9876543210fedcba98".to_string(),
        ];
        let cited = |text: &str| -> Vec<(u32, String)> {
            branch_sha_citations(text, &shas)
                .into_iter()
                .map(|(l, w, _)| (l, w))
                .collect()
        };
        assert_eq!(cited("see 0123456 here"), vec![(1, "0123456".to_string())]);
        assert_eq!(
            cited("x\n(fedcba98765)."),
            vec![(2, "fedcba98765".to_string())]
        );
        assert_eq!(
            cited("0123456789abcdef0123456789abcdef01234567"),
            vec![(1, "0123456789abcdef0123456789abcdef01234567".to_string())]
        );
        assert_eq!(
            cited("`0123456`, é 0123456-x"),
            vec![(1, "0123456".to_string()), (1, "0123456".to_string())]
        );
        // Six digits, or a run inside a longer word, cite nothing.
        assert!(cited("012345 is short").is_empty());
        assert!(cited("x0123456 and 0123456y and 0123456_z").is_empty());
        // Upper case is not how git prints a SHA.
        assert!(cited("0123456ABC").is_empty());
        // A hex run that prefixes no SHA of the range: main's, or another project's.
        assert!(cited("bd93004 and e98e296").is_empty());
        // A run that shares the first 7 digits and then differs is no prefix.
        assert!(cited("0123456ff").is_empty());
        // A SHA-256 SHA, 64 digits, is a citation of itself.
        let long = vec!["ab".repeat(32)];
        assert_eq!(branch_sha_citations(&"ab".repeat(32), &long).len(), 1);
    }

    #[test]
    fn the_verdict_line_counts_what_was_found() {
        assert_eq!(verdict(0), "PASSED: no findings");
        assert_eq!(verdict(1), "FAILED: 1 finding above");
        assert_eq!(verdict(3), "FAILED: 3 findings above");
    }
}
