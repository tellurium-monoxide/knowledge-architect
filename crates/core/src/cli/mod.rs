//! The core's commands, as a library module, per `design@core@the-core-cli-is-a-library-module`.
//!
//! A binary parses its arguments, flattening [`Command`] into its own command enum, and hands
//! the parsed command to [`run`] with the directories of its own source. This module owns two
//! things the checks deliberately do not: finding the project, and deciding what is printed. A
//! check that found the project itself would have to do it from its own location, and then it
//! could not be run against a model built in memory — which is the property the whole shape
//! rests on.
//!
//! Arguments are declared, never parsed by hand, per `design@core@arguments-parse-through-clap`, and
//! the exit codes are `design@core@exit-code-ladder`: 0 ran-and-clean, 1 ran-and-negative, 2
//! could-not-run.

use std::path::Path;
use std::process::ExitCode;

pub mod output;
use output::{out, outln};

use clap::{Args, Subcommand};

use crate::check::{Phase, Report};
use crate::entity::{Anchors, Candidate, Entities, Kind};
use crate::extension::{Extension, Prepared, Purpose, Tree};
use crate::manifest::{ISSUE_REGISTER, TRIPWIRE_REGISTER};
use crate::Manifest;

mod history;

mod gathered;
pub use gathered::Gathered;

// A binary's `main` refuses a build made from another checkout before any command, per
// `design@core@a-foreign-build-is-refused`.
pub use crate::build_origin::{refuse_a_foreign_build, this_library, Library};

/// The core's commands. A binary flattens this enum into its own.
///
/// Non-exhaustive: a binary hands a parsed command to [`run`] without matching it, so a new
/// command is not a breaking change for any binary.
// Non-exhaustive per `design@core@ne-minimal`.
#[derive(Subcommand)]
#[non_exhaustive]
pub enum Command {
    /// Every check, over one walk, in four phases.
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
    /// Judge every commit message in a range, each against its own commit's tree.
    Commits(CommitsArgs),
    /// Write the agent files this version ships into the project, and remove the ones it no
    /// longer ships.
    InstallAgentSkills,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Apply every fix the checker can make safely before checking: write the generated files and
    /// the installed agent files whose bytes the tree and this version determine, and list each.
    #[arg(long)]
    pub fix: bool,
}

#[derive(Args)]
pub struct CommitsArgs {
    /// The range to walk, as `git rev-list` reads one.
    #[arg(value_name = "RANGE")]
    pub range: String,
}

#[derive(Args)]
pub struct ShowArgs {
    /// The reference to print, `<kind>@<anchor>@<id>`.
    #[arg(value_name = "REF")]
    pub reference: String,
}

#[derive(Args)]
pub struct IssuesArgs {
    /// Only the entries of this kind.
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,
    /// Only the entries in this group subdirectory.
    #[arg(long, value_name = "GROUP")]
    pub group: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    #[arg(value_name = "ANCHOR|TEXT")]
    pub terms: Vec<String>,
}

#[derive(Args)]
pub struct TripwiresArgs {
    /// Only the entries guarding this entry: a decision, a goal or a declared register's
    /// entry, as the `guarding` column lists them.
    #[arg(long, value_name = "REF")]
    pub guarding: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    #[arg(value_name = "ANCHOR|TEXT")]
    pub terms: Vec<String>,
}

/// Run one of the core's commands over `manifest`.
///
/// `checker` is every directory of the running binary's own source, compiled into it, per
/// `design@core@checker-source-literals-are-data`. The exit code is
/// `design@core@exit-code-ladder`: `Err` is could-not-run, and the caller prints it and exits 2.
/// `extensions` is every extension the binary registers, per
/// `design@core@an-extension-plugs-in-through-phased-hooks`; the core's own binary passes
/// none.
pub fn run(
    command: Command,
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    // Every command reads the manifest as its extensions resolved it: the files they generate
    // are outside every walk, and a table they refuse is phase 1 of `check`.
    let mut configured = manifest.clone();
    crate::extension::configure(&mut configured, extensions);
    let manifest = &configured;
    match command {
        Command::Check(args) if args.fix => fix_then_check(manifest, checker, extensions),
        Command::Check(_) => check(manifest, checker, extensions),
        Command::Show(args) => show(manifest, &args, checker),
        Command::Issues(args) => issues(manifest, &args, checker),
        Command::Tripwires(args) => tripwires(manifest, &args, checker),
        Command::Index => index(manifest, checker, extensions),
        Command::Model => model(manifest, checker, extensions),
        Command::Commits(args) => history::commits(manifest, &args.range, checker, extensions),
        Command::InstallAgentSkills => install_agent_skills(manifest),
    }
}

/// Write the shipped agent files into the project, per `design@core@owned-namespace-check`.
///
/// Exit 0 when the installed set is the shipped one afterwards, 2 when the filesystem refused a
/// write. It edits nothing outside the installer's namespace: a missing primer import line is
/// said, not written, because the root CLAUDE.md belongs to the project.
fn install_agent_skills(manifest: &Manifest) -> Result<ExitCode, String> {
    if !manifest.complaints().is_empty() {
        return Err(format!(
            "the manifest holds declarations this tool refused: `{} check` reports them. \
             Nothing was installed.",
            manifest.command()
        ));
    }
    if !manifest.serves_claude() {
        outln!("the project declares no agent harness: nothing was installed");
        return Ok(ExitCode::SUCCESS);
    }
    let shipped = crate::agents::shipped(manifest);
    let done = crate::agents::install(manifest.root(), &shipped)?;
    for rel in &done.written {
        outln!("wrote    {}", rel.display());
    }
    for rel in &done.deleted {
        outln!("deleted  {}", rel.display());
    }
    if done.written.is_empty() && done.deleted.is_empty() {
        outln!("the installed set is already the shipped one");
    }
    let primer = std::path::Path::new(crate::agents::PRIMER);
    if shipped.iter().any(|(p, _)| p == primer) {
        let claude = std::fs::read_to_string(manifest.root().join(crate::manifest::AGENT_DOCUMENT))
            .unwrap_or_default();
        if !crate::agents::imports_primer(&claude) {
            outln!(
                "the root CLAUDE.md does not import the primer: add a line holding exactly `{}`",
                crate::agents::IMPORT_LINE
            );
        }
    }
    Ok(ExitCode::SUCCESS)
}

/// What a run gathers from the working tree, once the first three phases have found the model
/// complete.
///
/// **A writer refuses over an incomplete model.** An index generated over a model missing a
/// home, an unreadable file or a refused declaration lists rows nobody asked for, so a command
/// that writes a generated file asks this first and touches nothing when it refuses: the
/// answer is exit 2 naming the phase, and `check` is what reports the findings. Releases are
/// not needed by the phases this runs, so nothing is fetched to refuse.
pub fn complete_working_tree(
    manifest: &Manifest,
    model: &crate::Model,
) -> Result<Gathered, String> {
    let gathered = Gathered::over(manifest, model)?;
    let inputs = gathered.inputs();
    match crate::check::foundation(model, manifest, &inputs) {
        Ok(()) => Ok(gathered),
        Err(stop) => Err(format!(
            "the model is incomplete: {}. `{} check` reports them. Nothing was \
             written.",
            stop.phase.stop_line(stop.findings.len()),
            manifest.command()
        )),
    }
}

/// The project is whatever declares itself one at or above the working directory.
///
/// Nothing about any particular repository is compiled in, so pointing the tool at a mock
/// project under `path@core@tests/projects/` needs no flag and no special case.
pub fn locate() -> Result<Manifest, String> {
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
fn model(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker)
        .map_err(|e| format!("cannot read the project: {e}"))?;
    let extra: Vec<crate::model::DumpRow> =
        extensions.iter().flat_map(|e| e.dump(&model)).collect();
    let dump = model.canonical_with(&extra);
    // The count goes to stderr so that redirecting stdout gives a file that is only
    // observations, while a reader still learns how much was walked. A document with nothing
    // in it produces no line, so the two numbers together are what say whether the walk agrees
    // with another implementation of it.
    eprintln!(
        "{} documents, {} observations",
        model.documents().len(),
        dump.lines().count()
    );
    out!("{dump}");
    Ok(ExitCode::SUCCESS)
}

/// Every check, over one walk: the core's, then each extension's.
///
/// An extension prepares its tree here rather than inside a check: preparing may read the
/// archive or reach the network, and a check may do neither.
fn check(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;

    // What a check may not fetch for itself is fetched here, once.
    let gathered = Gathered::over(manifest, &model)?;
    let inputs = gathered.inputs();
    if let Err(stop) = crate::check::foundation(&model, manifest, &inputs) {
        let report = Report::stopped(stop, &model);
        print_report(&report);
        return Ok(ExitCode::FAILURE);
    }

    // The foundation has passed, so what each extension reads is needed now and by nothing
    // before: a run that stopped earlier resolved and fetched nothing.
    let mut prepared: Vec<(&'static [&'static str], Box<dyn Prepared>)> = Vec::new();
    for extension in extensions.iter_mut() {
        let p = extension.prepare(
            manifest,
            &model,
            Tree::Checkout(manifest.root()),
            Purpose::Check,
        )?;
        prepared.push((extension.checks(), p));
    }
    let with: Vec<(&[&'static str], &dyn Prepared)> =
        prepared.iter().map(|(c, p)| (*c, p.as_ref())).collect();
    let report = crate::check::run_with(&model, manifest, &inputs, &with);
    print_report(&report);
    Ok(if report.failed() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Apply every safe fix, then run the full check: `check --fix`.
///
/// **A fix is safe when its bytes are determined by the tree and the pinned version, and it
/// writes or removes only files the tool generates or installs.** Two pass that test: the install
/// of the agent files, and the generated files. Every other finding's repair is a choice, or
/// touches git or a hand-written file, and stays the reader's.
///
/// **No check writes; every write happens before the model the checks read is built.** The order:
/// a manifest that holds refused declarations writes nothing, and the check reports them; the
/// repairs of the installed set, exactly what the installed-file check reports, judged from git's
/// listing; the writer gate of phases 1 to 3 over a model rebuilt
/// from the tree as the install left it, which stops the run and writes nothing more; the
/// generated files; then the full check, whose report and exit code are the run's. The installed
/// files' bytes do not depend on the model, which is why they are repaired before that gate: a writer whose output
/// is read off the model never writes over an incomplete one.
///
/// Each file written or removed is listed before the report, so a session sees what to commit.
/// A failed write exits 2 when nothing was written yet and 1 after any write, as `index` does:
/// 2 promises a caller that the tree is as they left it.
fn fix_then_check(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    if !manifest.complaints().is_empty() {
        return check(manifest, checker, extensions);
    }
    let mut written = 0usize;
    let failed = |written: usize, e: String| {
        eprintln!("error: {e}");
        Ok(if written == 0 {
            ExitCode::from(2)
        } else {
            ExitCode::FAILURE
        })
    };

    // The installed set is repaired from git's listing of the namespace, judged as the check
    // judges it, so `--fix` writes or deletes only what the check reports: never an ignored file
    // in the namespace, which the install's filesystem walk would delete, and never a copy that
    // differs only by its line endings.
    if manifest.serves_claude() {
        let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
        let gathered = Gathered::over(manifest, &model)?;
        let inputs = gathered.inputs();
        let repairs = crate::agents::repairs(manifest.root(), inputs.shipped, inputs.installed);
        if !repairs.is_empty() {
            let mut done = crate::agents::Installed::default();
            let outcome =
                crate::agents::apply(manifest.root(), inputs.shipped, &repairs, &mut done);
            for rel in &done.written {
                outln!("fixed: wrote {} (installed)", rel.display());
            }
            for rel in &done.deleted {
                outln!("fixed: removed {} (installed)", rel.display());
            }
            written += done.written.len() + done.deleted.len();
            if let Err(e) = outcome {
                return failed(written, e);
            }
        }
    }

    // The writer gate, over the tree as the install left it. A stop is reported as `check`
    // reports it; what was installed is already listed above. A deletion the install made is
    // unstaged, and the installed-file check reports it here: staging touches git, which no fix
    // does, so an upgrade that removes a shipped file takes `git add` and a second run.
    let model = match crate::Model::build(manifest, checker) {
        Ok(model) => model,
        Err(e) => return failed(written, e.to_string()),
    };
    let gathered = match Gathered::over(manifest, &model) {
        Ok(gathered) => gathered,
        Err(e) => return failed(written, e),
    };
    if let Err(stop) = crate::check::foundation(&model, manifest, &gathered.inputs()) {
        let report = Report::stopped(stop, &model);
        if written > 0 {
            outln!();
        }
        print_report(&report);
        return Ok(ExitCode::FAILURE);
    }

    let generated = match generated_list(manifest, &model, &gathered, extensions) {
        Ok(generated) => generated,
        Err(e) => return failed(written, e),
    };
    if let Err(e) = check_destinations(manifest, &generated) {
        return failed(written, e);
    }
    for (rel, text) in generated {
        let path = manifest.root().join(&rel);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(text.as_str()) {
            continue;
        }
        if let Err(e) = std::fs::write(&path, &text) {
            return failed(written, format!("{}: {e}", path.display()));
        }
        written += 1;
        outln!("fixed: wrote {} (regenerated)", rel.display());
    }
    if written > 0 {
        outln!();
    }
    // The final check can still fail to run: a file it reads, or an extension's preparation. With
    // files already written, that is no could-not-run, which promises an untouched tree.
    match check(manifest, checker, extensions) {
        Err(e) => failed(written, e),
        outcome => outcome,
    }
}

/// The summary first, the findings under it, the verdict on the last line.
///
/// The order is the whole point: a caller reading the tail of the output has to reach the
/// answer, and when the findings came first every `| tail` and every `| grep` for a count
/// printed a success-shaped report over a failing run.
fn print_report(report: &Report) {
    out!("{}", counts(report));
    if !report.findings.is_empty() {
        outln!();
        for finding in &report.findings {
            outln!("{finding}");
        }
    }
    outln!("{}", verdict(report));
}

/// Regenerate every generated index in place.
///
/// **It writes only where the bytes differ**, per `design@core@generated-files-are-pure`: what a
/// generated file holds is a function of the walked tree, so rewriting an already current one
/// moves nothing but its mtime, and running this to look must cost nothing. It takes no flags
/// for the same reason — nothing here can lose content, so there is nothing for a dry run to
/// protect.
///
/// **Whether a generated file is current is not this command's question.**
/// The `generated` check of `cargo klarch check` is the gate, and it names the first line at which
/// the committed file and the regenerated one disagree.
fn index(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    // The gate first: a file an extension reads that is not there is phase 2's finding, named
    // by path, and reading it before the gate would turn that into an error naming nothing.
    let gathered = complete_working_tree(manifest, &model)?;
    let generated = generated_list(manifest, &model, &gathered, extensions)?;
    check_destinations(manifest, &generated).map_err(|e| format!("{e} Nothing was written."))?;

    let mut written = 0usize;
    for (rel, text) in generated {
        let path = manifest.root().join(&rel);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(text.as_str()) {
            outln!("{:<40} already current", rel.display());
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
        outln!("{:<40} rewritten", rel.display());
    }
    Ok(ExitCode::SUCCESS)
}

/// Every generated file, destination and bytes, over a model the writer gate has passed: each
/// extension's, then one index per file-register instance.
///
/// One function for `index` and `check --fix`, so the two write the same files, per
/// `design@core@generated-files-are-pure`. Every generated file comes in one call, so a flag
/// choosing between them buys nothing and cannot be given an invalid combination. The survey
/// answers which instance directories are there, and an instance without one contributes no index
/// rather than having its home created here.
fn generated_list(
    manifest: &Manifest,
    model: &crate::Model,
    gathered: &Gathered,
    extensions: &mut [Box<dyn Extension>],
) -> Result<Vec<(std::path::PathBuf, String)>, String> {
    let mut generated = Vec::new();
    for extension in extensions.iter_mut() {
        let prepared = extension.prepare(
            manifest,
            model,
            Tree::Checkout(manifest.root()),
            Purpose::Index,
        )?;
        generated.extend(
            prepared
                .generated(model, manifest)
                .into_iter()
                .map(|g| (g.rel, g.text)),
        );
    }
    generated.extend(crate::index::file_register_indexes(
        model,
        manifest,
        gathered.inputs().present,
        gathered.inputs().directories,
    ));
    Ok(generated)
}

/// Refuse every destination that cannot be written safely, before any is written.
///
/// The error names the path and says nothing of what was written: the caller knows that, and
/// `check --fix` may already have installed files when it calls this.
///
/// Every destination is checked before any is written. A run that wrote one index and then
/// failed on the next exited 2 — could not run — having already changed the tree, which is
/// the one place `design@core@exit-code-ladder`'s line blurs. A missing directory here is the
/// manifest declaring one the tree does not have; creating it would paper over that, and the
/// registers check is what reports it.
///
/// **The missing-directory arm is reached by an extension's generated file** whose directory
/// the tree does not hold: the extension names the path before the walk, and nothing creates its
/// directory. A file-register index cannot reach it, since one is generated only for an instance
/// whose directory the survey found.
fn check_destinations(
    manifest: &Manifest,
    generated: &[(std::path::PathBuf, String)],
) -> Result<(), String> {
    for (rel, _) in generated {
        let path = manifest.root().join(rel);
        let dir = path
            .parent()
            .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
        if !dir.is_dir() {
            return Err(format!(
                "{}: the directory this index is generated into is not there.",
                dir.display()
            ));
        }
        // `fs::write` follows a symlink and writes through it, so a generated path that is one
        // would replace whatever sits at the far end — which is the one way this command could
        // destroy something it did not generate, and what `design@core@generated-files-are-pure`
        // needs to be false for its claim to hold. The sibling tool refuses one for the same
        // reason, in `target_is_mutable`.
        if std::fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!(
                "{}: this index is a symlink, and writing would follow it.",
                path.display()
            ));
        }
    }
    Ok(())
}

/// One recorded entry, whole, and every reference to it.
///
/// **The two failure codes are different questions**, per `design@core@exit-code-ladder`: an argument
/// that is not reference-shaped could not be run and exits 2, and a reference the grammar accepts
/// that names nothing is a negative answer and exits 1. A reader who mistyped the grammar and a
/// reader who named a deleted entry need different things.
fn show(manifest: &Manifest, args: &ShowArgs, checker: &[&Path]) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    let reference = args.reference.trim_matches('`');
    let (kind, anchor, id) = match crate::entity::candidate(reference, &anchors) {
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
            outln!(
                "no anchor is named {anchor}; the anchors are {}",
                anchors.listed()
            );
            return Ok(ExitCode::FAILURE);
        };
        let rel = a.path.join(id.trim_end_matches('/'));
        if let Some(doc) = model.documents().iter().find(|d| d.rel == rel) {
            outln!("{reference}  {}", doc.rel.display());
            outln!();
            outln!("{}", doc.text.trim_end());
            found = true;
        } else {
            let survey = crate::survey::survey(manifest, &model).map_err(|e| e.to_string())?;
            if survey.present.contains(&rel) {
                outln!("{reference}  {}", rel.display());
                outln!();
                outln!("(outside the walk; nothing to print)");
                found = true;
            }
        }
    } else if let Some(record) =
        crate::records::records(&model, &anchors, &Entities::build(&model, &anchors), &kind)
            .into_iter()
            .find(|r| r.anchor == anchor && r.id == id)
    {
        outln!("{reference}  {}", record.site);
        outln!();
        outln!("{}", record.body.trim_end());
        found = true;
    }
    if !found {
        outln!("{reference} resolves to nothing");
        return Ok(ExitCode::FAILURE);
    }

    let inbound = crate::records::inbound(&model, &anchors, &kind, anchor, id);
    outln!();
    if inbound.is_empty() {
        outln!("referenced by nothing");
    } else {
        outln!("referenced at:");
        for site in inbound {
            outln!("  {site}");
        }
    }
    Ok(ExitCode::SUCCESS)
}

/// Every issue entry, one row each.
fn issues(manifest: &Manifest, args: &IssuesArgs, checker: &[&Path]) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    let kind = Kind::new(ISSUE_REGISTER);
    let (anchor, needle) = split_terms(&args.terms, &anchors);
    let mut rows =
        crate::records::records(&model, &anchors, &Entities::build(&model, &anchors), &kind);
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
    let dirs: Vec<std::path::PathBuf> = anchors
        .instances()
        .into_iter()
        .filter(|(_, register, _)| register.name == ISSUE_REGISTER)
        .map(|(_, _, home)| home.dir)
        .collect();
    let changed = crate::git::last_changed(manifest.root(), &dirs);

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
fn tripwires(
    manifest: &Manifest,
    args: &TripwiresArgs,
    checker: &[&Path],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    let kind = Kind::new(TRIPWIRE_REGISTER);
    let (anchor, needle) = split_terms(&args.terms, &anchors);
    let guarding = args
        .guarding
        .as_ref()
        .map(|g| g.trim_matches('`').to_string());
    let mut rows =
        crate::records::records(&model, &anchors, &Entities::build(&model, &anchors), &kind);
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
    outln!("{}", line(&head));
    for row in rows {
        outln!("{}", line(row));
    }
    if rows.is_empty() {
        outln!("(no entry)");
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
/// **A check that did not run contributes nothing.** Its counts are not zero, they are
/// unasked, and a zero would read as "nothing found" for a check that never ran. The
/// `checked:` line is what makes that legible for a check that carries no count of its own,
/// such as `generated`: without it, a run of `generated` alone and a run that performed
/// nothing look the same.
/// The checker's directories as the summary line names them, comma-separated.
fn sources(dirs: &[std::path::PathBuf]) -> String {
    dirs.iter()
        .map(|d| d.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn counts(report: &Report) -> String {
    use std::fmt::Write;

    let mut out = String::new();
    if report.phase != Phase::Content {
        // A stop names itself, and prints what every phase read: the walk. No family count
        // follows, because no family ran, and a zero would read as nothing found.
        let _ = write!(out, "\n{}", report.phase.stop_line(report.findings.len()));
        let _ = write!(out, "\nwalk: {} file(s)", report.structure.walked);
        if !report.structure.checker_sources.is_empty() {
            let _ = write!(
                out,
                "\nchecker source: {}, {} file(s) with string literals read as data",
                sources(&report.structure.checker_sources),
                report.structure.checker_files
            );
        }
        return out;
    }
    // Every check the last phase performed, by name. The line is what makes a check that
    // carries no count of its own — `generated` — legible as having run.
    let checked: Vec<&str> = report
        .checks
        .iter()
        .copied()
        .filter(|name| !report.not_run.contains(name))
        .collect();
    let _ = write!(out, "\nchecked: {}", checked.join(", "));
    // Not a check: every check read this walk. Git supplies it, per
    // `design@core@git-supplies-the-walk`, so the count is what a reader compares between CI and
    // a local run. It prints on a failing run as readily as a passing one.
    let _ = write!(out, "\nwalk: {} file(s)", report.structure.walked);
    if !report.not_run.is_empty() {
        // Not performed. Without this line the two failures are the same output: a check
        // that ran and found nothing, and one that could not run at all.
        let _ = write!(
            out,
            "\nNOT RUN: {} — the input it reads is not there",
            report.not_run.join(", ")
        );
    }

    // Not a check: it describes the walk every check read, and a count of zero in a
    // checkout that holds the tool is the loud failure the decision promises.
    if !report.structure.checker_sources.is_empty() {
        let _ = write!(
            out,
            "\nchecker source: {}, {} file(s) with string literals read as data",
            sources(&report.structure.checker_sources),
            report.structure.checker_files
        );
    }

    let s = &report.structure;
    let _ = write!(
        out,
        "\n\nregisters: {} component(s), {} location(s), {} instance(s), {} file entry(ies)",
        s.components, s.locations, s.instances, s.entries
    );
    let _ = write!(
        out,
        "\nreferences: {} entities defined, {} reference(s), {} link(s)",
        s.entities, s.references, s.links
    );
    // Each extension's block, after the core's lines, per
    // `design@core@an-extension-plugs-in-through-phased-hooks`.
    for summary in &report.summaries {
        out.push_str(summary);
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::{counts, Report};
    use crate::check::{Phase, CHECKS};

    fn report() -> Report {
        Report {
            phase: Phase::Content,
            findings: Vec::new(),
            structure: Default::default(),
            checks: CHECKS.to_vec(),
            not_run: Vec::new(),
            summaries: Vec::new(),
        }
    }

    /// A report whose every number is different from every other.
    ///
    /// Distinct values on purpose: with zeros everywhere, two counts printed in the wrong
    /// order render identically, so a line could read the reverse of what it means and no
    /// assertion would move.
    fn numbered() -> Report {
        let mut r = report();
        r.structure.components = 188;
        r.structure.locations = 199;
        r.structure.instances = 88;
        r.structure.entries = 99;
        r.structure.entities = 55;
        r.structure.references = 77;
        r.structure.links = 78;
        r.structure.checker_sources = vec!["tools/knowledge".into()];
        r.structure.checker_files = 200;
        r
    }

    #[test]
    fn every_count_is_rendered_in_the_position_its_label_promises() {
        let out = counts(&numbered());
        for expected in [
            "registers: 188 component(s), 199 location(s), 88 instance(s), 99 file entry(ies)",
            "references: 55 entities defined, 77 reference(s), 78 link(s)",
            "checker source: tools/knowledge, 200 file(s) with string literals read as data",
        ] {
            assert!(out.contains(expected), "missing {expected:?} in {out}");
        }
    }

    #[test]
    fn each_extension_block_follows_the_core_lines_in_order() {
        let mut r = numbered();
        r.summaries = vec!["\nfirst: 1".to_string(), "\nsecond: 2".to_string()];
        let out = counts(&r);
        let references = out.find("references:").expect("the core line");
        let first = out.find("first: 1").expect("the first block");
        let second = out.find("second: 2").expect("the second block");
        assert!(references < first && first < second, "{out}");
    }

    #[test]
    fn a_check_that_could_not_run_is_named_as_not_run_and_left_out_of_checked() {
        // The failure this guards is a check gated on an input that is not there: its name in
        // `checked:` would say it ran.
        let mut r = report();
        r.checks.extend(["history", "sources"]);
        r.not_run = vec!["history", "sources"];
        let out = counts(&r);
        assert!(out.contains("NOT RUN: history, sources"), "{out}");
        let checked = out
            .lines()
            .find(|l| l.starts_with("checked:"))
            .expect("the line");
        assert!(
            !checked.contains("sources") && !checked.contains("history"),
            "{checked}"
        );
        // Nothing is withheld when everything ran.
        assert!(!counts(&report()).contains("NOT RUN"));
    }

    #[test]
    fn a_full_run_names_every_check_it_performed() {
        // `generated` reports findings and counts nothing, so the `checked:` line is the
        // only thing separating "ran and found nothing" from "did not run".
        let mut r = report();
        r.checks.push("an-extension-check");
        let out = counts(&r);
        let checked = out
            .lines()
            .find(|l| l.starts_with("checked:"))
            .expect("the line");
        for name in CHECKS.iter().chain(&["an-extension-check"]) {
            assert!(
                checked.contains(name),
                "{name} should be named in {checked}"
            );
        }
    }

    #[test]
    fn a_stop_prints_its_line_and_no_check_count() {
        let mut r = report();
        r.phase = Phase::Tree;
        r.findings
            .push(crate::Finding::in_file("a.md", "what", "action"));
        r.structure.checker_sources = vec!["crates/core".into()];
        r.structure.checker_files = 27;
        let out = counts(&r);
        assert!(
            out.contains("phase 2: 1 finding(s); phases 3 and 4 were not judged"),
            "{out}"
        );
        // A stop still names the checker's directories, so the exemption's state is visible
        // in every run, per `design@core@checker-source-literals-are-data`.
        assert!(
            out.contains(
                "\nchecker source: crates/core, 27 file(s) with string literals read as data"
            ),
            "{out}"
        );
        assert!(out.contains("walk: "), "{out}");
        assert!(
            !out.contains("checked:") && !out.contains("registers:"),
            "{out}"
        );
    }
}
