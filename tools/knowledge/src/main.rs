//! The one command. Everything this tool does is a subcommand of this binary.
//!
//! The binary owns two things the libraries deliberately do not: finding the project, and
//! deciding what is printed. A library that found the project itself would have to do it from
//! its own location, and then a check could not be run against a model built in memory —
//! which is the property the whole shape rests on.
//!
//! Arguments are declared, never parsed by hand, per `design@thaum@arguments-parse-through-clap`, and
//! the exit codes are `design@thaum@exit-code-ladder`: 0 ran-and-clean, 1 ran-and-negative, 2
//! could-not-run.

use std::collections::HashMap;
use std::path::Path;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use documentation::check::citations::Release;
use documentation::check::{Inputs, Only, Report};
use documentation::entity::{Anchors, Candidate, Entities, Kind};
use documentation::manifest::{ISSUE_REGISTER, TRIPWIRE_REGISTER};
use documentation::Manifest;

mod corpus_cmd;
mod history_cmd;

/// Every accepted `--only` value, as the help prints them.
///
/// Written out rather than built from `Only::NAMED`, because a clap help string is a literal.
/// `every_family_is_named_in_the_help` is what keeps the two in step: it fails when a family
/// is added and not listed here. It is the SHORT help, so `-h` prints it and not only `--help`;
/// the tripwire below reaches for whichever a reader typed. A tripwire in `path@knowledge@docs/tripwires.md` reads this list
/// out of the help, so a family missing from it is a check whose output reaches no reviewer.
const FAMILIES: &str = "citations, generated, registers, references, uncovered, changes, \
                        corpus, regime. `structure` names every family but citations. A \
                        comma-separated list runs their union over one walk.";

#[derive(Parser)]
#[command(
    name = "knowledge",
    about = "What this project knows, and whether it still holds.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Every check, over one walk; or only these families.
    Check(CheckArgs),
    /// One recorded entry, whole, and every reference to it.
    Show(ShowArgs),
    /// Every issue entry, one row each.
    Issues(IssuesArgs),
    /// Every tripwire entry, one row each.
    Tripwires(TripwiresArgs),
    /// Regenerate every generated index in place.
    Index,
    /// Every observation the walk produced: file, line, kind, value.
    Model,
    /// Judge one commit message, from a file, against the working tree.
    CommitMessage(CommitMessageArgs),
    /// Judge every commit message in a range, each against its own commit's tree.
    Commits(CommitsArgs),
    /// This clone's commit-message hook.
    Hook {
        #[command(subcommand)]
        command: history_cmd::HookCommand,
    },
    /// The corpus and its releases.
    Rules {
        #[command(subcommand)]
        command: corpus_cmd::RulesCommand,
    },
}

#[derive(Args)]
struct CheckArgs {
    /// Which check families to run. Without it, every one.
    #[arg(long, value_name = "FAMILIES", value_parser = Only::parse, help = FAMILIES)]
    only: Option<Only>,
}

#[derive(Args)]
struct CommitMessageArgs {
    /// The file holding the message. What a `commit-msg` hook is handed.
    #[arg(value_name = "FILE")]
    file: std::path::PathBuf,
}

#[derive(Args)]
struct CommitsArgs {
    /// The range to walk, as `git rev-list` reads one.
    #[arg(value_name = "RANGE")]
    range: String,
}

#[derive(Args)]
struct ShowArgs {
    /// The reference to print, `<kind>@<anchor>@<id>`.
    #[arg(value_name = "REF")]
    reference: String,
}

#[derive(Args)]
struct IssuesArgs {
    /// Only the entries of this kind.
    #[arg(long, value_name = "KIND")]
    kind: Option<String>,
    /// Only the entries in this group subdirectory.
    #[arg(long, value_name = "GROUP")]
    group: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    #[arg(value_name = "ANCHOR|TEXT")]
    terms: Vec<String>,
}

#[derive(Args)]
struct TripwiresArgs {
    /// Only the entries carrying this reference.
    #[arg(long, value_name = "REF")]
    guarding: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    #[arg(value_name = "ANCHOR|TEXT")]
    terms: Vec<String>,
}

fn main() -> ExitCode {
    // Parsed before the project is located, so `--help` answers from anywhere and a mistyped
    // invocation is refused without a walk. clap exits 2 on a parse failure, which is already
    // this project's could-not-run code.
    let cli = Cli::parse();

    let outcome = locate().and_then(|manifest| match cli.command {
        Command::Check(args) => check(&manifest, args.only.unwrap_or(Only::EVERYTHING)),
        Command::Show(args) => show(&manifest, &args),
        Command::Issues(args) => issues(&manifest, &args),
        Command::Tripwires(args) => tripwires(&manifest, &args),
        Command::Index => index(&manifest),
        Command::Model => model(&manifest),
        Command::CommitMessage(args) => {
            history_cmd::commit_message(&manifest, &args.file, Some(checker_source()))
        }
        Command::Commits(args) => {
            history_cmd::commits(&manifest, &args.range, Some(checker_source()))
        }
        Command::Hook { command } => history_cmd::hook(&manifest, &command),
        Command::Rules { command } => corpus_cmd::run(&manifest, &command),
    });

    match outcome {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

/// The checker's own source directory, compiled in.
///
/// The one path this binary carries about any tree is its own. `CARGO_MANIFEST_DIR` of this
/// crate is the component's directory exactly, and the alias in `path@thaum@.cargo/config.toml` builds
/// the binary from the checkout on every run, so the compiled path names the tree being
/// checked. Every model this binary builds is told it, so that the tool's own fixtures are
/// read as data rather than as citations, per `design@knowledge@checker-source-literals-are-data`.
/// A binary built elsewhere names a directory the walk never visits, exempts nothing, and the
/// summary block's `checker source` line shows the count at zero.
pub(crate) fn checker_source() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// The project is whatever declares itself one at or above the working directory.
///
/// Nothing about any particular repository is compiled in, so pointing the tool at a mock
/// project under `path@knowledge@tests/projects/` needs no flag and no special case.
fn locate() -> Result<Manifest, String> {
    let cwd =
        std::env::current_dir().map_err(|e| format!("cannot read the working directory: {e}"))?;
    Manifest::find(&cwd).map_err(|e| e.to_string())
}

/// Every observation the walk and the scanner produced, one per line.
///
/// This is what the model was compared against the implementation it replaces with, and it is
/// the way to see what a check is actually being handed. Filtering it on a rule number is also
/// how a citation is located inside the file the rule index names, because markers and rule
/// tokens are separate kinds here and a text grep tells them apart from neither each other nor
/// from a rule number that is data.
fn model(manifest: &Manifest) -> Result<ExitCode, String> {
    let model = documentation::Model::build(manifest, Some(checker_source()))
        .map_err(|e| format!("cannot read the project: {e}"))?;
    let dump = model.canonical();
    // The count goes to stderr so that redirecting stdout gives a file that is only
    // observations, while a reader still learns how much was walked. A document with nothing
    // in it produces no line, so the two numbers together are what say whether the walk agrees
    // with another implementation of it.
    eprintln!(
        "{} documents, {} observations",
        model.documents().len(),
        dump.lines().count()
    );
    print!("{dump}");
    Ok(ExitCode::SUCCESS)
}

/// Every check, over one walk.
///
/// Releases are resolved here rather than inside a check: resolving one may read the archive
/// or reach the network, and a check may do neither. What a check receives is a release
/// already parsed.
fn check(manifest: &Manifest, only: Only) -> Result<ExitCode, String> {
    let model =
        documentation::Model::build(manifest, Some(checker_source())).map_err(|e| e.to_string())?;
    let tree = manifest.rules_tree();
    let body_starts_at = manifest.rules().body_starts_at;

    // Only the families that read rule text get their releases resolved, and resolving a pin
    // may fetch over the network. A run asking for references has no business reaching for a
    // release, and before this was scoped it failed on a pin it had no reason to read.
    // `generated` renders the rule index, so it needs the vendored release and no other.
    let mut releases: HashMap<Option<String>, Release> = HashMap::new();
    if only.has(Only::CITATIONS) || only.has(Only::GENERATED) || only.has(Only::REGIME) {
        let vendored = std::fs::read_to_string(tree.text())
            .map_err(|e| format!("{}: {e}", tree.text().display()))?;
        releases.insert(None, Release::new(&vendored, body_starts_at));
    }
    if only.has(Only::CITATIONS) || only.has(Only::REGIME) {
        for doc in model.documents() {
            let Some(date) = &doc.pin else { continue };
            if releases.contains_key(&doc.pin) {
                continue;
            }
            let path = rules::release::resolve(&tree, date)?;
            let text =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            releases.insert(doc.pin.clone(), Release::new(&text, body_starts_at));
        }
    }

    // The generated files are outside the walk — the rule index by a declared row, every
    // file-register index by construction — because a generated file is not a source of
    // citations. They are read here so a check does not.
    let mut committed = HashMap::new();
    let mut generated_paths = vec![manifest.rules().dir.join("index.md")];
    generated_paths.extend(documentation::index::generated_index_paths(manifest));
    for rel in generated_paths {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&rel)) {
            committed.insert(rel, text);
        }
    }

    // A register instance's options sit beside it and are not markdown, so the walk never
    // reads them. They are read here for the same reason the generated files are: a check
    // may not touch the filesystem.
    let mut configs = HashMap::new();
    for (_, _, home) in Anchors::of(manifest).instances() {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&home.config)) {
            configs.insert(home.config.clone(), text);
        }
    }
    let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
    let pinned = rules::release::read_version(&version)
        .get("date")
        .cloned()
        .ok_or("VERSION names no date")?;
    // One listing answers every question a check has about what is there, and it is the
    // caller's job because a check may not touch the filesystem.
    let survey = documentation::survey::survey(manifest, &model).map_err(|e| e.to_string())?;
    // Whether the ignore rules cover a path target is git's answer, taken in ONE batch over
    // every spelling a reference in this run could ask about — a check spawns nothing, and a
    // process per reference would be a process per pointer in the tree.
    let queries = documentation::check::references::ignore_queries(
        &model,
        &documentation::entity::Anchors::of(manifest),
    );
    let ignored =
        documentation::git::ignored(manifest.root(), &queries).map_err(|e| e.to_string())?;
    let tracked_and_ignored =
        documentation::git::tracked_and_ignored(manifest.root()).map_err(|e| e.to_string())?;
    let inputs = Inputs {
        releases: &releases,
        pinned: &pinned,
        committed: &committed,
        configs: &configs,
        present: &survey.present,
        directories: &survey.directories,
        outside: &survey.outside,
        ignored: &ignored,
        tracked_and_ignored: &tracked_and_ignored,
    };
    let mut report = documentation::check::run(&model, manifest, &inputs, only);

    // The changelog is outside the walk: each section pins its own release, so it is checked
    // against that release rather than the vendored one. The caller resolves them, as with
    // every other release.
    let changes_path = manifest.rules().dir.join("CHANGES.md");
    let wants_changes = only.has(Only::CHANGES);
    let wants_corpus = only.has(Only::CORPUS);
    if wants_changes || wants_corpus {
        // Both families are gated on the changelog being readable, so when it is not there
        // neither ran. Saying otherwise would print a zero for a check nobody performed,
        // which for `corpus` means reporting provenance clean without having read a byte
        // of it.
        if std::fs::read_to_string(manifest.root().join(&changes_path)).is_err() {
            report.ran = report.ran.without(Only::CHANGES.union(Only::CORPUS));
        }
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&changes_path)) {
            let (sections, _) = documentation::check::changes::parse(&text).unwrap_or_default();
            let mut corpora = HashMap::new();
            let mut digests = HashMap::new();
            for section in &sections {
                let path = rules::release::resolve(&tree, &section.version)?;
                let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
                digests.insert(section.version.clone(), rules::release::sha256(&bytes));
                let text = String::from_utf8_lossy(&bytes).to_string();
                corpora.insert(
                    section.version.clone(),
                    rules::Corpus::parse(&text, body_starts_at),
                );
            }
            if wants_changes {
                let (found, changes) =
                    documentation::check::changes::check(&changes_path, &text, &corpora, &digests);
                report.findings.extend(found);
                report.changelog_changes = changes;
            }
            if wants_corpus {
                // Every release the changelog names must already be in the tree, alongside
                // the pin. The changelog is parsed above whichever family asked for it,
                // because this check reads it for the release list rather than to check it.
                let mut needed = vec![(pinned.clone(), "the pin".to_string())];
                needed.extend(sections.iter().map(|s| {
                    (
                        s.version.clone(),
                        format!("the changelog section {}", s.version),
                    )
                }));
                let (problems, corpus_counts) = rules::integrity::check(&tree, &needed);
                for p in problems {
                    report
                        .findings
                        .push(documentation::Finding::in_file(&p.about, p.what, p.action));
                }
                report.corpus = corpus_counts;
            }
        }
    }
    // The summary first, the findings under it, the verdict on the last line. The order is
    // the whole point: a caller reading the tail of the output has to reach the answer, and
    // when the findings came first every `| tail` and every `| grep` for a count printed a
    // success-shaped report over a failing run.
    print!("{}", counts(&report));
    if !report.findings.is_empty() {
        println!();
        for finding in &report.findings {
            println!("{finding}");
        }
    }
    println!("{}", verdict(&report));
    Ok(if report.failed() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Regenerate every generated index in place.
///
/// **It writes only where the bytes differ**, per `design@knowledge@generated-files-are-pure`: what a
/// generated file holds is a function of the walked tree, so rewriting an already current one
/// moves nothing but its mtime, and running this to look must cost nothing. It takes no flags
/// for the same reason — nothing here can lose content, so there is nothing for a dry run to
/// protect.
///
/// **Whether a generated file is current is not this command's question.**
/// `cargo knowledge check --only generated` is the gate, and it names the first line at which
/// the committed file and the regenerated one disagree.
fn index(manifest: &Manifest) -> Result<ExitCode, String> {
    let model =
        documentation::Model::build(manifest, Some(checker_source())).map_err(|e| e.to_string())?;
    let tree = manifest.rules_tree();
    let corpus_text = std::fs::read_to_string(tree.text()).map_err(|e| e.to_string())?;
    let corpus = rules::Corpus::parse(&corpus_text, manifest.rules().body_starts_at);
    let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
    let pinned = rules::release::read_version(&version)
        .get("date")
        .cloned()
        .ok_or("VERSION names no date")?;

    // Every generated index in one invocation, so a flag choosing between them buys nothing
    // and cannot be given an invalid combination: the rule index, then one per file-register
    // instance. The survey answers which instance directories are there, and an instance
    // without one contributes no index rather than having its home created here.
    let survey = documentation::survey::survey(manifest, &model).map_err(|e| e.to_string())?;
    let mut generated = vec![(
        manifest.rules().dir.join("index.md"),
        documentation::index::rule_index(&model, manifest, &corpus, &pinned),
    )];
    generated.extend(documentation::index::file_register_indexes(
        &model,
        manifest,
        &survey.directories,
    ));

    // Every destination is checked before any is written. A run that wrote one index and then
    // failed on the next exited 2 — could not run — having already changed the tree, which is
    // the one place `design@thaum@exit-code-ladder`'s line blurs. A missing directory here is the
    // manifest declaring one the tree does not have; creating it would paper over that, and the
    // registers check is what reports it.
    //
    // **The missing-directory arm is unreachable as the destinations stand**, and is kept for
    // the next generator rather than for this one: a file-register index is generated only for
    // an instance whose directory the survey found, and the rule index's directory holds the
    // corpus text this function already failed to read. A generator whose destination sits
    // outside both makes it reachable again, and there is nothing to construct for a test until
    // one does.
    for (rel, _) in &generated {
        let path = manifest.root().join(rel);
        let dir = path
            .parent()
            .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
        if !dir.is_dir() {
            return Err(format!(
                "{}: the directory this index is generated into is not there. Nothing was written.",
                dir.display()
            ));
        }
        // `fs::write` follows a symlink and writes through it, so a generated path that is one
        // would replace whatever sits at the far end — which is the one way this command could
        // destroy something it did not generate, and what `design@knowledge@generated-files-are-pure`
        // needs to be false for its claim to hold. The sibling tool refuses one for the same
        // reason, in `target_is_mutable`.
        if std::fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!(
                "{}: this index is a symlink, and writing would follow it. Nothing was written.",
                path.display()
            ));
        }
    }

    let mut written = 0usize;
    for (rel, text) in generated {
        let path = manifest.root().join(&rel);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(text.as_str()) {
            println!("{:<40} already current", rel.display());
            continue;
        }
        if let Err(e) = std::fs::write(&path, &text) {
            eprintln!("error: {}: {e}", path.display());
            // Once anything has been written the run is no longer a could-not-run, and 2
            // promises a caller that the tree is as they left it. The checks above catch the
            // reachable causes; a permission or device failure between them and here is what
            // this arm is for.
            return Ok(if written == 0 {
                ExitCode::from(2)
            } else {
                ExitCode::FAILURE
            });
        }
        written += 1;
        println!("{:<40} rewritten", rel.display());
    }
    Ok(ExitCode::SUCCESS)
}

/// One recorded entry, whole, and every reference to it.
///
/// **The two failure codes are different questions**, per `design@thaum@exit-code-ladder`: an argument
/// that is not reference-shaped could not be run and exits 2, and a reference the grammar accepts
/// that names nothing is a negative answer and exits 1. A reader who mistyped the grammar and a
/// reader who named a deleted entry need different things.
fn show(manifest: &Manifest, args: &ShowArgs) -> Result<ExitCode, String> {
    let model =
        documentation::Model::build(manifest, Some(checker_source())).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest);
    let reference = args.reference.trim_matches('`');
    let (kind, anchor, id) = match documentation::entity::candidate(reference, &anchors) {
        Candidate::Reference { kind, anchor, id } => (kind, anchor, id),
        Candidate::Malformed { why } => {
            return Err(format!("{reference} is malformed: {why}"));
        }
        Candidate::AnchorInKindPosition { head } => {
            return Err(format!(
                "{reference} opens with {head}, an anchor, where the kind goes: write \
                 <kind>@<anchor>@<id>, one of {}",
                anchors.kinds_listed()
            ));
        }
        Candidate::NotOne => {
            return Err(format!(
                "{reference} is not a reference: write <kind>@<anchor>@<id>, the kind one of {}",
                anchors.kinds_listed()
            ));
        }
    };

    // The body first, then what points at it. A reader asking for an entry wants the entry; the
    // inbound list is what tells them what closing it would break.
    let mut found = false;
    if kind.is_path() {
        // A path's entity is the tree's, so it is shown from the walked document where there is
        // one and from the survey where there is not: a directory and an unwalked file both
        // exist and are both worth resolving, and neither has a body to print.
        let Some(a) = anchors.by_name(anchor) else {
            println!(
                "no anchor is named {anchor}; the anchors are {}",
                anchors.listed()
            );
            return Ok(ExitCode::FAILURE);
        };
        let rel = a.path.join(id.trim_end_matches('/'));
        if let Some(doc) = model.documents().iter().find(|d| d.rel == rel) {
            println!("{reference}  {}", doc.rel.display());
            println!();
            println!("{}", doc.text.trim_end());
            found = true;
        } else {
            let survey =
                documentation::survey::survey(manifest, &model).map_err(|e| e.to_string())?;
            if survey.present.contains(&rel) {
                println!("{reference}  {}", rel.display());
                println!();
                println!("(outside the walk; nothing to print)");
                found = true;
            }
        }
    } else if let Some(record) =
        documentation::records::records(&model, &anchors, &Entities::build(&model, &anchors), &kind)
            .into_iter()
            .find(|r| r.anchor == anchor && r.id == id)
    {
        println!("{reference}  {}", record.site);
        println!();
        println!("{}", record.body.trim_end());
        found = true;
    }
    if !found {
        println!("{reference} resolves to nothing");
        return Ok(ExitCode::FAILURE);
    }

    let inbound = documentation::records::inbound(&model, &anchors, &kind, anchor, id);
    println!();
    if inbound.is_empty() {
        println!("referenced by nothing");
    } else {
        println!("referenced at:");
        for site in inbound {
            println!("  {site}");
        }
    }
    Ok(ExitCode::SUCCESS)
}

/// Every issue entry, one row each.
fn issues(manifest: &Manifest, args: &IssuesArgs) -> Result<ExitCode, String> {
    let model =
        documentation::Model::build(manifest, Some(checker_source())).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest);
    let kind = Kind::new(ISSUE_REGISTER);
    let (anchor, needle) = split_terms(&args.terms, &anchors);
    let mut rows = documentation::records::records(
        &model,
        &anchors,
        &Entities::build(&model, &anchors),
        &kind,
    );
    rows.retain(|r| {
        anchor.is_none_or(|a| r.anchor == a)
            && args
                .kind
                .as_ref()
                .is_none_or(|k| r.metadata.as_deref() == Some(k.as_str()))
            && args
                .group
                .as_ref()
                .is_none_or(|g| r.group.as_deref() == Some(g.as_str()))
            && needle.as_ref().is_none_or(|t| r.matches(t))
    });
    // Kind then id, so entries of one kind read together whatever anchor they sit under.
    rows.sort_by(|a, b| (&a.metadata, &a.id).cmp(&(&b.metadata, &b.id)));

    // One `git log` for every instance directory at once. A per-row invocation is a process per
    // entry, and this column is not worth one.
    let dirs: Vec<std::path::PathBuf> = Anchors::of(manifest)
        .instances()
        .into_iter()
        .filter(|(_, register, _)| register.name == ISSUE_REGISTER)
        .map(|(_, _, home)| home.dir)
        .collect();
    let changed = documentation::git::last_changed(manifest.root(), &dirs);

    let table: Vec<[String; 5]> = rows
        .iter()
        .map(|r| {
            [
                r.metadata.clone().unwrap_or_else(|| "-".to_string()),
                r.anchor.clone(),
                r.id.clone(),
                r.title.clone(),
                // No commit is `uncommitted`; git answering nothing at all is `-`, and the two
                // are different facts: the first is a new file, the second is no history to ask.
                match changed.get(&r.site.file) {
                    Some(date) => date.clone(),
                    None if changed.is_empty() => "-".to_string(),
                    None => "uncommitted".to_string(),
                },
            ]
        })
        .collect();
    print_rows(&["kind", "anchor", "id", "title", "last change"], &table);
    Ok(if table.is_empty() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Every tripwire entry, one row each.
fn tripwires(manifest: &Manifest, args: &TripwiresArgs) -> Result<ExitCode, String> {
    let model =
        documentation::Model::build(manifest, Some(checker_source())).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest);
    let kind = Kind::new(TRIPWIRE_REGISTER);
    let (anchor, needle) = split_terms(&args.terms, &anchors);
    let guarding = args
        .guarding
        .as_ref()
        .map(|g| g.trim_matches('`').to_string());
    let mut rows = documentation::records::records(
        &model,
        &anchors,
        &Entities::build(&model, &anchors),
        &kind,
    );
    rows.retain(|r| {
        anchor.is_none_or(|a| r.anchor == a)
            && guarding
                .as_ref()
                .is_none_or(|g| r.guards.iter().any(|c| c == g))
            && needle.as_ref().is_none_or(|t| r.matches(t))
    });
    rows.sort_by(|a, b| (&a.anchor, &a.id).cmp(&(&b.anchor, &b.id)));

    let table: Vec<[String; 4]> = rows
        .iter()
        .map(|r| {
            [
                r.anchor.clone(),
                r.id.clone(),
                r.title.clone(),
                r.guarding(),
            ]
        })
        .collect();
    print_rows(&["anchor", "id", "title", "guarding"], &table);
    Ok(if table.is_empty() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// The positional arguments of a listing: an anchor, then the text to filter on.
///
/// **The first term is the anchor only when something declares that name**, which is what lets
/// one positional list mean both. A search word that happens to be an anchor name is the one
/// ambiguity, and it is resolved towards the anchor because that is the documented first
/// position; a search for that word alone is written with the anchor before it.
fn split_terms<'a>(terms: &'a [String], anchors: &Anchors) -> (Option<&'a str>, Option<String>) {
    let mut rest = terms;
    let mut anchor = None;
    if let Some(first) = terms.first() {
        if anchors.by_name(first).is_some() {
            anchor = Some(first.as_str());
            rest = &terms[1..];
        }
    }
    let needle = (!rest.is_empty()).then(|| rest.join(" "));
    (anchor, needle)
}

/// A table with its header, each column as wide as its widest cell.
///
/// The header prints over an empty table too: a listing with no row has to say what it looked
/// for, or a filter that matched nothing reads as a project with nothing in it.
fn print_rows<const N: usize>(header: &[&str; N], rows: &[[String; N]]) {
    let mut width = [0usize; N];
    for (i, name) in header.iter().enumerate() {
        width[i] = name.chars().count();
    }
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            width[i] = width[i].max(cell.chars().count());
        }
    }
    let line = |cells: &[String; N]| {
        cells
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i + 1 == N {
                    c.clone()
                } else {
                    format!("{c:<w$}", w = width[i])
                }
            })
            .collect::<Vec<_>>()
            .join("  ")
    };
    let head: [String; N] = std::array::from_fn(|i| header[i].to_string());
    println!("{}", line(&head));
    for row in rows {
        println!("{}", line(row));
    }
    if rows.is_empty() {
        println!("(no entry)");
    }
}

/// The last line, and the only one that states the outcome.
///
/// **It is derived from the finding list rather than tracked beside it**, so it cannot
/// disagree with the exit code: `Report::failed` is the same predicate over the same vector.
/// A caller scripting against this reads the exit code; a person reads this line.
fn verdict(report: &Report) -> String {
    match report.findings.len() {
        0 => "PASSED: no findings".to_string(),
        1 => "FAILED: 1 finding above".to_string(),
        n => format!("FAILED: {n} findings above"),
    }
}

/// What a run looked at, as the block printed above the findings.
///
/// **A family that did not run contributes nothing.** Its counts are not zero, they are
/// unasked, and a zero would read as "nothing found" for a check that never ran. The
/// `checked:` line is what makes that legible for the two families that carry no count of
/// their own: without it, a run of `generated` alone and a run that performed nothing look
/// the same.
fn counts(report: &Report) -> String {
    use std::fmt::Write;

    let mut out = String::new();
    let ran = report.ran;
    let _ = write!(out, "\nchecked: {}", ran.names().join(", "));
    // Not a family: every family read this walk. Git supplies it, per
    // `design@knowledge@git-supplies-the-walk`, so the count is what a reader compares between CI and
    // a local run. It prints whatever was asked for, and on a failing run as readily as a
    // passing one.
    let _ = write!(out, "\nwalk: {} file(s)", report.structure.walked);
    let skipped = report.asked.without(ran);
    if skipped != Only::NOTHING {
        // Asked for and not performed. Without this line the two failures are the same
        // output: a family that ran and found nothing, and one that could not run at all.
        let _ = write!(
            out,
            "\nNOT RUN: {} — asked for, and the input it reads is not there",
            skipped.names().join(", ")
        );
    }

    // Not a family: it describes the walk every family read, and a count of zero in a
    // checkout that holds the tool is the loud failure the decision promises.
    if let Some(source) = &report.structure.checker_source {
        let _ = write!(
            out,
            "\nchecker source: {}, {} file(s) with string literals read as data",
            source.display(),
            report.structure.checker_files
        );
    }

    if ran.has(Only::CITATIONS) {
        let c = &report.counts;
        let _ = write!(
            out,
            "\n{}/{} rule-quote fragments verified against the rule cited",
            c.verified, c.fragments
        );
        if c.misattributed > 0 {
            let _ = write!(
                out,
                "\n{} fragment(s) verify against a DIFFERENT rule",
                c.misattributed
            );
        }
        if c.commentary > 0 {
            let _ = write!(
                out,
                "\n{} blockquote(s) hold commentary, not rule text",
                c.commentary
            );
        }
        if c.short > 0 {
            let _ = write!(
                out,
                "\n{} elided fragment(s) under {} chars: checked, but weak evidence",
                c.short,
                documentation::check::citations::MIN_FRAGMENT
            );
        }
        if report.pinned.is_empty() {
            let _ = write!(
                out,
                "\n\nno pinned containers: every quote tracks the vendored release"
            );
        } else {
            let _ = write!(
                out,
                "\n\npinned containers (their quotes do not track the vendored release):"
            );
            for (file, date, quotes) in &report.pinned {
                let _ = write!(out, "\n  {file}: cr-version {date}, {quotes} quote(s)");
            }
        }
        let _ = write!(
            out,
            "\n\nlint: {} unmarked rule reference(s), {} orphan identifier marker(s)",
            c.unmarked, c.orphans
        );
    }

    let s = &report.structure;
    let mut structural = String::new();
    if ran.has(Only::REGISTERS) {
        let _ = write!(
            structural,
            "\nregisters: {} component(s), {} location(s), {} instance(s), {} file entry(ies)",
            s.components, s.locations, s.instances, s.entries
        );
    }
    if ran.has(Only::REFERENCES) {
        let _ = write!(
            structural,
            "\nreferences: {} entities defined, {} reference(s), {} link(s)",
            s.entities, s.references, s.links
        );
    }
    if ran.has(Only::UNCOVERED) {
        let _ = write!(
            structural,
            "\nuncovered files: {} scanned",
            s.uncovered_files
        );
    }
    if ran.has(Only::CORPUS) {
        let _ = write!(
            structural,
            "\ncorpus: {} archived release(s), {} manifest row(s), {} of {} release(s) resolved locally",
            report.corpus.archived,
            report.corpus.manifest_rows,
            report.corpus.releases_local,
            report.corpus.releases_needed
        );
    }
    if ran.has(Only::CHANGES) {
        let _ = write!(
            structural,
            "\nchangelog: {} rule change(s)",
            report.changelog_changes
        );
    }
    if ran.has(Only::REGIME) {
        let _ = write!(
            structural,
            "\nregime: {} claim(s) judged against their scope",
            report.regime.claims
        );
    }
    if !structural.is_empty() {
        out.push('\n');
        out.push_str(&structural);
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::{counts, Cli, Command, Only, Report, FAMILIES};
    use crate::corpus_cmd::RulesCommand;
    use clap::Parser;

    // Arguments handed to the parser, and what it must hand back: each is named because it
    // appears on both sides of an assertion.
    const RULE: &str = "601.2";
    const ANOTHER_RULE: &str = "104.1";
    const OLD: &str = "20260807";
    const NEW: &str = "20260819";

    // clap's own consistency check over the whole derive: a flag that conflicts with an
    // argument that does not exist, a broken default, two arguments claiming one name. It is
    // what catches a malformed declaration at test time rather than at first invocation.
    #[test]
    fn cli_declaration_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    /// The claim: every family the tool accepts is named in the help.
    ///
    /// `FAMILIES` is a literal because a clap help string has to be one, so nothing but this
    /// keeps it in step with `Only::NAMED`. The tripwire guarding
    /// `design@knowledge@families-are-the-checks` reads the family list out of the help, so a family
    /// missing from it is a check whose output reaches no reviewer and nothing reports that.
    #[test]
    fn every_family_is_named_in_the_help() {
        for (name, _) in Only::NAMED {
            assert!(
                FAMILIES.contains(name),
                "{name} is accepted by --only and absent from its help"
            );
        }
        assert!(FAMILIES.contains("structure"), "the grouping name too");
    }

    /// The claim: every combination the hand-rolled parser accepted is refused, and refused
    /// while parsing rather than by a check inside a command.
    ///
    /// Each row below exited 0 before this migration, having done something other than what
    /// was asked — the first silently reported the project empty, and the rest ignored a flag
    /// or an argument. That is the discrimination: they are observed passing against the
    /// implementation this replaces.
    #[test]
    fn every_recorded_silent_acceptance_is_refused() {
        for argv in [
            // Set from each other's absence, the pair selected neither kind.
            // The command `issues` and `tripwires` replaced, and the flags it carried.
            vec!["knowledge", "outstanding"],
            vec!["knowledge", "issues", "--issues"],
            vec!["knowledge", "tripwires", "--tripwires"],
            // `show` cannot run without the reference it prints.
            vec!["knowledge", "show"],
            // Unknown flags, accepted by every subcommand.
            vec!["knowledge", "check", "--bogus"],
            vec!["knowledge", "issues", "--bogus"],
            vec![
                "knowledge",
                "rules",
                "diff",
                "--bogus",
                "--old",
                OLD,
                "--new",
                NEW,
            ],
            // Positional junk, filtered out of the arguments and never refused.
            vec!["knowledge", "check", "stray"],
            vec!["knowledge", "model", "zzz"],
            // A flag on a command that has none, silently dropped.
            vec!["knowledge", "rules", "show", "--write", RULE],
            // The flags `index` no longer has, one pair of which wrote a file the gate rejects.
            vec!["knowledge", "index", "--write"],
            vec!["knowledge", "index", "--interpretations"],
            vec!["knowledge", "index", "--lines"],
            // Dates by position, where a swap produces a reversed work list in silence.
            vec!["knowledge", "rules", "diff", OLD, NEW],
            vec!["knowledge", "rules", "diff", "--old", OLD],
            // A date `release::url_for` would slice the first four bytes of. Unvalidated,
            // each of these panicked at exit 101, which the ladder has no meaning for.
            vec!["knowledge", "rules", "diff", "--old", "202", "--new", NEW],
            vec!["knowledge", "rules", "diff", "--old", OLD, "--new", ""],
            vec!["knowledge", "rules", "fetch", "not-a-date"],
            vec!["knowledge", "rules", "bump", "2026080"],
            // Arguments a command cannot run without.
            vec!["knowledge", "rules", "show"],
            vec!["knowledge", "rules", "bump"],
            vec!["knowledge"],
        ] {
            assert!(
                Cli::try_parse_from(argv.iter().copied()).is_err(),
                "{argv:?} must be refused"
            );
        }
    }

    /// The claim: what the tool does accept parses to the command and values it names.
    ///
    /// The refusals above are satisfied by a declaration that refuses everything, so this is
    /// the half that says the interface still exists.
    #[test]
    fn the_accepted_shapes_parse_to_what_they_name() {
        let Command::Check(bare) = Cli::parse_from(["knowledge", "check"]).command else {
            panic!("check parses to the check subcommand");
        };
        assert!(bare.only.is_none(), "no --only is every family");

        let Command::Check(some) =
            Cli::parse_from(["knowledge", "check", "--only", "references,generated"]).command
        else {
            panic!("check parses to the check subcommand");
        };
        assert_eq!(some.only, Some(Only::REFERENCES.union(Only::GENERATED)));

        let Command::Show(shown) = Cli::parse_from(["knowledge", "show", "design@a@b"]).command
        else {
            panic!("show parses to the show subcommand");
        };
        assert_eq!(shown.reference, "design@a@b");

        let Command::Issues(listed) = Cli::parse_from([
            "knowledge",
            "issues",
            "--kind",
            "defect",
            "--group",
            "layers",
            "two",
            "words",
        ])
        .command
        else {
            panic!("issues parses to the issues subcommand");
        };
        assert_eq!(listed.kind.as_deref(), Some("defect"));
        assert_eq!(listed.group.as_deref(), Some("layers"));
        assert_eq!(listed.terms, ["two", "words"]);

        let Command::Tripwires(guarded) =
            Cli::parse_from(["knowledge", "tripwires", "--guarding", "design@a@b"]).command
        else {
            panic!("tripwires parses to the tripwires subcommand");
        };
        assert_eq!(guarded.guarding.as_deref(), Some("design@a@b"));
        assert!(guarded.terms.is_empty());

        let Command::Rules { command } =
            Cli::parse_from(["knowledge", "rules", "diff", "--old", OLD, "--new", NEW]).command
        else {
            panic!("rules parses to the rules subcommand");
        };
        let RulesCommand::Diff { old, new } = command else {
            panic!("diff parses to the diff verb");
        };
        // Named, so this assertion could not pass with the two the wrong way round.
        assert_eq!((old.as_str(), new.as_str()), (OLD, NEW));

        let Command::Rules { command } =
            Cli::parse_from(["knowledge", "rules", "show", RULE, ANOTHER_RULE]).command
        else {
            panic!("rules parses to the rules subcommand");
        };
        let RulesCommand::Show { numbers } = command else {
            panic!("show parses to the show verb");
        };
        assert_eq!(numbers.len(), 2, "several numbers at once, in order given");

        assert!(matches!(
            Cli::parse_from(["knowledge", "index"]).command,
            Command::Index
        ));
    }

    fn report(ran: Only) -> Report {
        Report {
            regime: Default::default(),
            findings: Vec::new(),
            counts: Default::default(),
            structure: Default::default(),
            corpus: Default::default(),
            changelog_changes: 0,
            pinned: Vec::new(),
            ran,
            asked: ran,
        }
    }

    /// A report whose every number is different from every other.
    ///
    /// Distinct values on purpose: with zeros everywhere, two counts printed in the wrong
    /// order render identically, so the headline could read `328/328 verified` while meaning
    /// the reverse and no assertion would move.
    fn numbered(ran: Only) -> Report {
        let mut r = report(ran);
        r.counts.verified = 11;
        r.counts.fragments = 22;
        r.counts.unmarked = 33;
        r.counts.orphans = 44;
        r.structure.components = 188;
        r.structure.locations = 199;
        r.structure.instances = 88;
        r.structure.entries = 99;
        r.structure.entities = 55;
        r.structure.references = 77;
        r.structure.links = 78;
        r.structure.uncovered_files = 122;
        r.structure.checker_source = Some("tools/knowledge".into());
        r.structure.checker_files = 200;
        r.corpus.archived = 133;
        r.corpus.manifest_rows = 144;
        r.corpus.releases_local = 155;
        r.corpus.releases_needed = 166;
        r.changelog_changes = 177;
        r
    }

    #[test]
    fn every_count_is_rendered_in_the_position_its_label_promises() {
        let out = counts(&numbered(Only::EVERYTHING));
        for expected in [
            "11/22 rule-quote fragments verified against the rule cited",
            "registers: 188 component(s), 199 location(s), 88 instance(s), 99 file entry(ies)",
            "lint: 33 unmarked rule reference(s), 44 orphan identifier marker(s)",
            "references: 55 entities defined, 77 reference(s), 78 link(s)",
            "uncovered files: 122 scanned",
            "checker source: tools/knowledge, 200 file(s) with string literals read as data",
            "corpus: 133 archived release(s), 144 manifest row(s),",
            "155 of 166 release(s) resolved locally",
            "changelog: 177 rule change(s)",
        ] {
            assert!(out.contains(expected), "missing {expected:?} in {out}");
        }
    }

    #[test]
    fn a_family_asked_for_that_could_not_run_is_named_as_not_run() {
        // The failure this guards is a family gated on an input that is not there: without
        // the line, it prints an empty `checked:` and reads as a run that did nothing.
        let mut r = report(Only::CORPUS.union(Only::REFERENCES));
        r.ran = Only::REFERENCES;
        let out = counts(&r);
        assert!(out.contains("checked: references"), "{out}");
        assert!(out.contains("NOT RUN: corpus"), "{out}");
        assert!(
            !out.contains("corpus:"),
            "no count for a family that did not run: {out}"
        );
        // Nothing is withheld when everything asked for ran.
        assert!(!counts(&report(Only::REFERENCES)).contains("NOT RUN"));
    }

    #[test]
    fn a_family_that_did_not_run_contributes_no_count() {
        let out = counts(&report(Only::REFERENCES));
        assert!(out.contains("references:"), "{out}");
        // Every other family's label is absent rather than present with a zero. A zero here
        // reads as "nothing found" for a check that never ran.
        for label in [
            "registers:",
            "uncovered files:",
            "corpus:",
            "changelog:",
            "lint:",
            "fragments verified",
        ] {
            assert!(!out.contains(label), "{label} should be absent from {out}");
        }
    }

    #[test]
    fn the_families_that_ran_are_named_even_when_they_carry_no_count() {
        // `generated` reports findings and counts nothing, so this line is the only thing
        // separating "ran and found nothing" from "did not run".
        let out = counts(&report(Only::GENERATED));
        assert!(out.contains("checked: generated"), "{out}");
        assert!(!out.contains("NOT RUN"), "{out}");
    }

    #[test]
    fn a_full_run_names_every_family_and_prints_both_blocks() {
        let out = counts(&report(Only::EVERYTHING));
        for (name, _) in Only::NAMED {
            assert!(out.contains(name), "{name} should be named in {out}");
        }
        assert!(out.contains("fragments verified"), "{out}");
        assert!(out.contains("changelog:"), "{out}");
    }
}
