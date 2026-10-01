//! An extension written against the library's public paths alone, run as an extension's own
//! tests run it.
//!
//! The claim is that the public API suffices to write an extension and to test it over a mock
//! project: configure it, gather the inputs, run the foundation, then the whole run with the
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
        report.summary = format!("first-heading: {} file(s) present", inputs.present.len());
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
            .any(|s| s.starts_with("first-heading: ")),
        "{:?}",
        report.summaries
    );
}
