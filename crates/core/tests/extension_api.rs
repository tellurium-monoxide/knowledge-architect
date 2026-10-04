//! An extension written against the library's public paths alone, run as an extension's own
//! tests run it.
//!
//! The claim is that the public API suffices to write an extension and to test it over a mock
//! project: configure it, gather the inputs, run phases 1 to 3, then the last phase with the
//! extension prepared. An integration test is compiled as a separate crate, so a public item
//! this file needs that the facade leaves private is a compile error here.

use std::path::PathBuf;

use knowledge_architect::document::Observation;
use knowledge_architect::extension::{
    configure, DumpRow, Extension, ExtensionReport, Inputs, Prepared, Purpose, Resolution, Tree,
};
use knowledge_architect::testing::{foundation, run_with, CHECKS};
use knowledge_architect::{cli, Document, Finding, Manifest, Model};

/// One check: the first level-1 heading of the root README is reported, at its line.
#[derive(Default)]
struct FirstHeading;

impl Extension for FirstHeading {
    fn tables(&self) -> &'static [&'static str] {
        &[]
    }

    fn resolve(&mut self, _: &Manifest) -> Resolution {
        Resolution::default()
    }

    fn checks(&self) -> &'static [&'static str] {
        &["first-heading"]
    }

    fn dump(&self, _: &Model) -> Vec<DumpRow> {
        Vec::new()
    }

    fn prepare(
        &mut self,
        _: &Manifest,
        _: &Model,
        tree: Tree<'_>,
        purpose: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        // Exhaustive on purpose: a new tree kind must make an extension say how it reads it.
        match tree {
            Tree::Checkout(_) => {}
            Tree::Commit(_) => return Err("this extension reads the checkout only".into()),
        }
        assert_eq!(purpose, Purpose::Check);
        Ok(Box::new(Prepared1))
    }
}

struct Prepared1;

impl Prepared for Prepared1 {
    fn check(&self, model: &Model, _: &Manifest, inputs: &Inputs) -> ExtensionReport {
        let mut report = ExtensionReport::default();
        let readme: &Document = model
            .documents()
            .iter()
            .find(|d| d.rel == std::path::Path::new("README.md"))
            .expect("the mock's root README is walked");
        let first = readme.observations.iter().find_map(|o| match &o.what {
            Observation::Heading { level: 1, .. } => Some(o.line),
            _ => None,
        });
        if let Some(line) = first {
            report.findings.push(Finding::at(
                &readme.rel,
                line,
                "the first level-1 heading",
                "nothing: this extension reports it to show it ran",
            ));
        }
        // Each line of a summary starts with a newline, as the core's lines do.
        report.summary = format!("\nfirst-heading: {} file(s) present", inputs.present.len());
        report
    }

    fn check_message(&self, _: &Document) -> Vec<Finding> {
        Vec::new()
    }

    fn generated(&self, _: &Model, _: &Manifest) -> Vec<knowledge_architect::extension::Generated> {
        Vec::new()
    }
}

#[test]
fn an_extension_written_against_the_public_api_runs_over_a_mock_project() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/projects/dirhome");
    let mut manifest = Manifest::load(&root).expect("the mock project's manifest");
    let mut extensions: Vec<Box<dyn Extension>> = vec![Box::new(FirstHeading)];
    configure(&mut manifest, &mut extensions);
    assert!(
        manifest.complaints().is_empty(),
        "{:?}",
        manifest.complaints()
    );

    let model = Model::build(&manifest, &[]).expect("a model of the mock");
    let gathered = cli::Gathered::over(&manifest, &model).expect("the inputs of the mock");
    let inputs = gathered.inputs();
    if let Err(stop) = foundation(&model, &manifest, &inputs) {
        panic!("dirhome stopped at {:?}: {:#?}", stop.phase, stop.findings);
    }

    let prepared = extensions[0]
        .prepare(
            &manifest,
            &model,
            Tree::Checkout(manifest.root()),
            Purpose::Check,
        )
        .expect("the extension prepares");
    let report = run_with(
        &model,
        &manifest,
        &inputs,
        &[(extensions[0].checks(), prepared.as_ref())],
    );

    // The line the heading is on, read from the file rather than from the model, so the
    // observation's line number is what is checked.
    let text = std::fs::read_to_string(root.join("README.md")).expect("the mock's README");
    let line = text
        .lines()
        .position(|l| l.starts_with("# "))
        .expect("the README opens with a level-1 heading") as u32
        + 1;
    let found: Vec<String> = report.findings.iter().map(|f| f.to_string()).collect();
    assert_eq!(
        report.findings.len(),
        1,
        "dirhome holds no core finding, so the extension's is the only one: {found:#?}"
    );
    assert_eq!(report.findings[0].file, PathBuf::from("README.md"));
    assert_eq!(report.findings[0].line, Some(line));
    assert!(report.failed(), "a finding fails the run");
    let mut checks: Vec<&str> = CHECKS.to_vec();
    checks.push("first-heading");
    assert_eq!(
        report.checks, checks,
        "the core's checks, then the extension's"
    );
    assert!(
        report
            .summaries
            .iter()
            .any(|s| s.starts_with("\nfirst-heading: ")),
        "{:?}",
        report.summaries
    );
}

/// An extension that generates one file, `listing.md`: the number of walked documents.
#[derive(Default)]
struct Listing;

impl Extension for Listing {
    fn tables(&self) -> &'static [&'static str] {
        &[]
    }

    fn resolve(&mut self, _: &Manifest) -> Resolution {
        let mut resolution = Resolution::default();
        resolution.generated.push(PathBuf::from("listing.md"));
        resolution
    }

    fn checks(&self) -> &'static [&'static str] {
        &[]
    }

    fn dump(&self, _: &Model) -> Vec<DumpRow> {
        Vec::new()
    }

    fn prepare(
        &mut self,
        _: &Manifest,
        _: &Model,
        _: Tree<'_>,
        _: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        Ok(Box::new(ListingPrepared))
    }
}

struct ListingPrepared;

impl Prepared for ListingPrepared {
    fn check(&self, _: &Model, _: &Manifest, _: &Inputs) -> ExtensionReport {
        ExtensionReport::default()
    }

    fn check_message(&self, _: &Document) -> Vec<Finding> {
        Vec::new()
    }

    fn generated(
        &self,
        model: &Model,
        _: &Manifest,
    ) -> Vec<knowledge_architect::extension::Generated> {
        vec![knowledge_architect::extension::Generated {
            rel: PathBuf::from("listing.md"),
            text: format!("{} documents\n", model.documents().len()),
            action: "regenerate it",
        }]
    }
}

/// The same generated file, from an extension that cannot prepare for the check.
#[derive(Default)]
struct ListingThenFails;

impl Extension for ListingThenFails {
    fn tables(&self) -> &'static [&'static str] {
        &[]
    }

    fn resolve(&mut self, manifest: &Manifest) -> Resolution {
        Listing.resolve(manifest)
    }

    fn checks(&self) -> &'static [&'static str] {
        &[]
    }

    fn dump(&self, _: &Model) -> Vec<DumpRow> {
        Vec::new()
    }

    fn prepare(
        &mut self,
        _: &Manifest,
        _: &Model,
        _: Tree<'_>,
        purpose: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        match purpose {
            Purpose::Check => Err("this extension cannot prepare for the check".into()),
            _ => Ok(Box::new(ListingPrepared)),
        }
    }
}

/// Copy `from` into `to`, recursively.
fn copy_tree(from: &std::path::Path, to: &std::path::Path) {
    std::fs::create_dir_all(to).expect("the destination");
    for entry in std::fs::read_dir(from).expect("a readable directory") {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copy");
        }
    }
}

/// Run git in `dir`, with the per-user ignore file pinned away, as the binary tests do.
fn git(dir: &std::path::Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(["-c", "core.excludesFile=/dev/null"])
        .args(args)
        .current_dir(dir)
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?}");
}

/// The claim: `check --fix` writes an extension's generated file,
/// prepared for the index, and the check that follows, prepared for the check, finds it current.
/// Mutation checked: leaving the extensions out of the generated list leaves the file missing,
/// and the run fails.
#[test]
fn check_fix_writes_an_extensions_generated_file_and_the_run_passes() {
    let root = std::env::temp_dir().join(format!("ka-fix-extension-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/projects/dirhome"),
        &root,
    );
    git(&root, &["init", "-q"]);
    git(&root, &["add", "-A"]);
    let manifest = Manifest::load(&root).expect("the copy's manifest");
    let mut extensions: Vec<Box<dyn Extension>> = vec![Box::new(Listing)];
    let check = |fix: bool, extensions: &mut Vec<Box<dyn Extension>>| {
        cli::run(
            cli::Command::Check(cli::CheckArgs { fix }),
            &manifest,
            &[],
            extensions,
        )
        .expect("the check runs")
    };
    assert_eq!(
        check(false, &mut extensions),
        std::process::ExitCode::FAILURE
    );
    assert_eq!(
        check(true, &mut extensions),
        std::process::ExitCode::SUCCESS
    );
    let listing = std::fs::read_to_string(root.join("listing.md")).expect("the generated file");
    assert!(listing.ends_with(" documents\n"), "{listing}");
    assert_eq!(
        check(false, &mut extensions),
        std::process::ExitCode::SUCCESS
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// The claim: once `check --fix` has written a file, a final check that cannot run exits 1, not
/// 2, since 2 promises a caller an untouched tree. Mutation checked: passing the final check's
/// error through makes `cli::run` return it, which a binary exits 2 on.
#[test]
fn check_fix_after_a_write_never_reports_could_not_run() {
    let root = std::env::temp_dir().join(format!("ka-fix-unpreparable-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/projects/dirhome"),
        &root,
    );
    git(&root, &["init", "-q"]);
    git(&root, &["add", "-A"]);
    let manifest = Manifest::load(&root).expect("the copy's manifest");
    let mut extensions: Vec<Box<dyn Extension>> = vec![Box::new(ListingThenFails)];
    let outcome = cli::run(
        cli::Command::Check(cli::CheckArgs { fix: true }),
        &manifest,
        &[],
        &mut extensions,
    );
    assert_eq!(outcome, Ok(std::process::ExitCode::FAILURE));
    assert!(
        root.join("listing.md").exists(),
        "the file was written first"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// An extension whose generated file sits in a directory the tree does not hold.
#[derive(Default)]
struct ListingNowhere;

impl Extension for ListingNowhere {
    fn tables(&self) -> &'static [&'static str] {
        &[]
    }

    fn resolve(&mut self, _: &Manifest) -> Resolution {
        let mut resolution = Resolution::default();
        resolution
            .generated
            .push(PathBuf::from("nowhere/listing.md"));
        resolution
    }

    fn checks(&self) -> &'static [&'static str] {
        &[]
    }

    fn dump(&self, _: &Model) -> Vec<DumpRow> {
        Vec::new()
    }

    fn prepare(
        &mut self,
        _: &Manifest,
        _: &Model,
        _: Tree<'_>,
        _: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        Ok(Box::new(NowherePrepared))
    }
}

struct NowherePrepared;

impl Prepared for NowherePrepared {
    fn check(&self, _: &Model, _: &Manifest, _: &Inputs) -> ExtensionReport {
        ExtensionReport::default()
    }

    fn check_message(&self, _: &Document) -> Vec<Finding> {
        Vec::new()
    }

    fn generated(&self, _: &Model, _: &Manifest) -> Vec<knowledge_architect::extension::Generated> {
        vec![knowledge_architect::extension::Generated {
            rel: PathBuf::from("nowhere/listing.md"),
            text: "listing\n".to_string(),
            action: "regenerate it",
        }]
    }
}

/// The claim: a generated destination in a directory the tree does not hold makes the run exit 2
/// with nothing written, and no directory is created for it: the registers check, not a writer,
/// reports a missing home. Mutation checked: creating the missing directory before the write
/// fails it. Dropping the refusal alone survives, since the write then fails with the same code;
/// the refusal's own contribution is a message naming the directory.
#[test]
fn check_fix_refuses_a_destination_whose_directory_is_missing() {
    let root = std::env::temp_dir().join(format!("ka-fix-nowhere-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/projects/dirhome"),
        &root,
    );
    git(&root, &["init", "-q"]);
    git(&root, &["add", "-A"]);
    let manifest = Manifest::load(&root).expect("the copy's manifest");
    let mut extensions: Vec<Box<dyn Extension>> = vec![Box::new(ListingNowhere)];
    let outcome = cli::run(
        cli::Command::Check(cli::CheckArgs { fix: true }),
        &manifest,
        &[],
        &mut extensions,
    );
    assert_eq!(outcome, Ok(std::process::ExitCode::from(2)));
    assert!(!root.join("nowhere").exists());
    let _ = std::fs::remove_dir_all(&root);
}

/// The claim: a binary that flattens the core's commands and describes nothing of itself shows no
/// description, rather than the doc comment the core writes for a binary's author.
#[test]
fn a_binary_that_describes_nothing_shows_no_description() {
    use clap::{CommandFactory, Parser};

    #[derive(Parser)]
    struct Bare {
        #[command(subcommand)]
        command: cli::Command,
    }

    let command = Bare::command();
    assert_eq!(command.get_about(), None);
    assert_eq!(command.get_long_about(), None);
}
