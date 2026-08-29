//! The tool run against whole projects, not against strings.
//!
//! A mock project under `tools/knowledge/tests/projects/` is a complete project: it carries
//! its own `knowledge.toml` and its own documents. That is what makes these tests worth more
//! than the in-memory ones — a fixture written as a string cannot exercise the walk, the
//! exclusions, or a layout different from this repository's.
//!
//! It is also what stops fixtures leaking. This file and the projects beside it sit under
//! `tools/knowledge/tests`, which this repository's own manifest excludes, so a planted slug,
//! a dangling path or a rule number here is invisible to the checks that run on the
//! repository. Written as a literal into a test that IS walked, each of those would be a real
//! anchor, a real broken reference and a real citation — which is what happened before this
//! directory existed, four times, each caught by a checker rather than by review.
//!
//! Cargo compiles `tests/*.rs`, so this file is a test target and `projects/` beside it is
//! not: a directory without a `main.rs` is data.

use std::path::PathBuf;

use documentation::{Manifest, Model, Observation};

fn mock(name: &str) -> Manifest {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name);
    Manifest::load(&root).expect("the mock project's manifest")
}

fn model(name: &str) -> Model {
    Model::build(&mock(name)).expect("a model of the mock project")
}

fn walked(model: &Model) -> Vec<String> {
    let mut names: Vec<String> = model
        .documents()
        .iter()
        .map(|d| d.rel.display().to_string())
        .collect();
    names.sort();
    names
}

#[test]
fn the_walk_obeys_the_project_that_declares_it() {
    let names = walked(&model("minimal"));
    assert_eq!(
        names,
        vec![
            "CLAUDE.md".to_string(),
            "README.md".to_string(),
            "code/lib.rs".to_string(),
            "docs/design.md".to_string(),
            "docs/open-issues.md".to_string(),
            "docs/rejected-alternatives.md".to_string(),
            "docs/tripwires.md".to_string(),
            "notes/a.md".to_string(),
            "notes/b.md".to_string(),
            "notes/open-issues.md".to_string(),
        ],
        "the walk should hold every markdown and Rust file, minus every exclusion"
    );
}

#[test]
fn each_of_the_three_exclusion_kinds_removes_its_file() {
    let names = walked(&model("minimal"));
    // A directory name, a path, and a filename — one document each, and each planted with a
    // slug definition that would collide if it were walked.
    assert!(!names.iter().any(|n| n.starts_with("build/")), "skip-dirs");
    assert!(!names.iter().any(|n| n.starts_with("vendor/")), "exclude");
    assert!(
        !names.iter().any(|n| n.ends_with("generated.md")),
        "skip-files"
    );
}

#[test]
fn a_suffix_the_tool_cannot_parse_is_not_walked() {
    // The walk is markdown and Rust, compiled in rather than declared, so the mock's own TOML
    // manifest and its rules text are both outside it whatever the manifest says.
    let names = walked(&model("minimal"));
    assert!(!names.iter().any(|n| n.ends_with(".txt")));
    assert!(!names.iter().any(|n| n.ends_with(".tsv")));
    assert!(!names.iter().any(|n| n.ends_with(".toml")));
}

#[test]
fn observations_come_out_of_a_real_walk_with_real_line_numbers() {
    let model = model("minimal");
    let dump = model.canonical();
    // A slug opening a decision, a reference to it from a Rust doc comment, a path reference,
    // and the two marker forms — each at the line of the file it sits on.
    assert!(
        dump.contains("docs/design.md\t5\tslug-def\tmock-anchor"),
        "{dump}"
    );
    // The whole value, and the trailing tab is load-bearing. This assertion once named the
    // path alone, which is a prefix of the component-suffixed value the renderer wrongly
    // produced, so it passed against both the right output and the wrong one for as long as
    // the defect existed. A `contains` over a field that is not terminated asserts a prefix.
    assert!(
        dump.contains("docs/design.md\t7\tpath-ref\tminimal@notes/b.md\n"),
        "{dump}"
    );
    assert!(
        dump.contains("docs/design.md\t7\tmarker-prose\t100.1"),
        "{dump}"
    );
    assert!(dump.contains("notes/a.md\t5\tinterp-ref\t7"), "{dump}");
    assert!(
        dump.contains("code/lib.rs\t1\tmarker-prose\t100.1"),
        "{dump}"
    );
}

#[test]
fn a_tracker_outside_every_component_is_read_by_the_report() {
    // What `additional-trackers` is for: a directory carrying outstanding state and nothing
    // else a component carries. Undeclared, this entry is in no report and nobody finds it.
    let manifest = mock("minimal");
    let model = model("minimal");
    let files: Vec<String> = documentation::outstanding::tracker_files(&model, &manifest)
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    assert!(
        files.contains(&"notes/open-issues.md".to_string()),
        "{files:#?}"
    );
    let entries = documentation::outstanding::entries(&model, &manifest);
    let here: Vec<&documentation::outstanding::Entry> = entries
        .iter()
        .filter(|e| e.file.ends_with("notes/open-issues.md"))
        .collect();
    assert_eq!(here.len(), 1, "{entries:#?}");
    assert!(here[0].is_issue, "an open-issues.md holds issues");
    assert_eq!(here[0].kind.to_string(), "observation");
}

#[test]
fn a_slug_inside_a_fence_is_neither_a_definition_nor_a_reference_in_a_real_file() {
    // The fenced block in that document holds all three forms — a head, a qualified pointer
    // and an unqualified one — which is what a document explaining the convention holds. None
    // of them is an observation, and the two real pointers elsewhere in the project are.
    let model = model("minimal");
    let observed = |f: fn(&Observation) -> Option<String>| -> Vec<String> {
        model
            .documents()
            .iter()
            .flat_map(|d| d.observations.iter())
            .filter_map(|l| f(&l.what))
            .collect()
    };
    let defs = observed(|o| match o {
        Observation::SlugDef(s) => Some(s.clone()),
        _ => None,
    });
    assert_eq!(defs, vec!["mock-anchor".to_string()]);
    let refs = observed(|o| match o {
        Observation::SlugRef { component, slug } => {
            Some(format!("{}#{slug}", component.clone().unwrap_or_default()))
        }
        _ => None,
    });
    assert_eq!(
        refs,
        vec![
            "minimal#mock-anchor".to_string(),
            "minimal#mock-anchor".to_string()
        ]
    );
}

#[test]
fn the_corpus_parses_under_the_project_that_declares_where_its_body_starts() {
    // This mock's text has no table of contents, so its body starts at line zero. The
    // repository's starts at 181. Nothing in the parser knows either number.
    let manifest = mock("minimal");
    let tree = manifest.rules_tree();
    let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
    let corpus = rules::Corpus::parse(&text, manifest.rules().body_starts_at);
    assert_eq!(corpus.len(), 2);
    let second = rules::RuleNumber::parse("100.2").expect("a rule number");
    assert_eq!(
        corpus.get(&second),
        Some("A second mock rule, wrapped over two lines."),
        "a continuation line should join the rule"
    );
}

/// A project that carries what it declares reports nothing.
///
/// The planted project below cannot show this: everything there is wrong on purpose, so a check
/// that had started reporting a correct component as incomplete would look the same. `minimal`
/// declares one component — the one at the root — and carries every document it owes.
#[test]
fn a_project_carrying_every_component_document_reports_nothing() {
    use documentation::check::citations::Release;
    use documentation::check::{run, Inputs, Only};
    use std::collections::HashMap;

    let manifest = mock("minimal");
    let model = Model::build(&manifest).expect("a model");
    let releases: HashMap<Option<String>, Release> = HashMap::new();
    let committed = HashMap::new();
    let (present, outside) =
        documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
    let inputs = Inputs {
        releases: &releases,
        pinned: "20200101",
        committed: &committed,
        present: &present,
        outside: &outside,
    };
    let report = run(
        &model,
        &manifest,
        &inputs,
        Only::COMPONENTS.union(Only::SLUGS),
    );
    let found: Vec<String> = report.findings.iter().map(|f| f.to_string()).collect();
    assert!(found.is_empty(), "{found:#?}");
    // The component at the root is one whether or not anything is declared beside it, and its
    // two references resolve against it by the project's own name.
    assert_eq!(report.structure.components, 1);
    assert_eq!(report.structure.additional_trackers, 1);
    assert_eq!(
        (
            report.structure.slugs_defined,
            report.structure.slugs_referenced
        ),
        (1, 1)
    );
}

/// The other accepted design home, end to end: `docs/design/` headed by a README that links
/// the one subdocument, with the anchor defined in the subdocument and referenced from the
/// project's own README. `paths` runs too, so the fixture shows the three families passing
/// together over a real walk.
#[test]
fn a_directory_design_home_passes_end_to_end() {
    use documentation::check::citations::Release;
    use documentation::check::{run, Inputs, Only};
    use std::collections::HashMap;

    let manifest = mock("dirhome");
    let model = Model::build(&manifest).expect("a model");
    let releases: HashMap<Option<String>, Release> = HashMap::new();
    let committed = HashMap::new();
    let (present, outside) =
        documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
    let inputs = Inputs {
        releases: &releases,
        pinned: "20200101",
        committed: &committed,
        present: &present,
        outside: &outside,
    };
    let report = run(
        &model,
        &manifest,
        &inputs,
        Only::COMPONENTS.union(Only::SLUGS).union(Only::PATHS),
    );
    let found: Vec<String> = report.findings.iter().map(|f| f.to_string()).collect();
    assert!(found.is_empty(), "{found:#?}");
    assert_eq!(
        (
            report.structure.slugs_defined,
            report.structure.slugs_referenced
        ),
        (1, 1)
    );
}

/// The checks run against a project whose documents are wrong on purpose.
///
/// This is the half the repository itself cannot test. Running the checks here proves only
/// that nothing is found, because everything here is correct; a checker that had stopped
/// detecting would look exactly the same. These assertions are on the findings themselves.
///
/// There is no cross-check against the implementation being replaced for this project: it has
/// no manifest to read and could not be pointed at a mock corpus. The planted defects are
/// checked against intent instead, which is what a fixture is for.
mod planted {
    use super::*;
    use documentation::check::{citations::Release, run, Inputs, Only};
    use documentation::index;
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// The generated files this project would have if they were current.
    ///
    /// Supplied rather than written: a check receives them, so a test can hand it any state
    /// it likes without a file existing anywhere.
    fn current_indexes(
        manifest: &Manifest,
        model: &Model,
        corpus: &rules::Corpus,
    ) -> HashMap<PathBuf, String> {
        HashMap::from([
            (
                manifest.rules().dir.join("index.md"),
                index::rule_index(model, manifest, corpus, "20200101"),
            ),
            (
                manifest.interpretations().dir.join("index.md"),
                index::interpretation_index(model, manifest, false),
            ),
        ])
    }

    fn findings_with(
        committed: impl Fn(&Manifest, &Model, &rules::Corpus) -> HashMap<PathBuf, String>,
    ) -> Vec<String> {
        findings_of(committed, Only::EVERYTHING)
    }

    /// The findings of one run, over whichever families `only` names.
    fn findings_of(
        committed: impl Fn(&Manifest, &Model, &rules::Corpus) -> HashMap<PathBuf, String>,
        only: Only,
    ) -> Vec<String> {
        let manifest = mock("planted");
        let model = Model::build(&manifest).expect("a model");
        let tree = manifest.rules_tree();
        let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
        let release = Release::new(&text, manifest.rules().body_starts_at);
        let corpus = rules::Corpus::parse(&text, manifest.rules().body_starts_at);
        let committed = committed(&manifest, &model, &corpus);
        let releases = HashMap::from([(None, release)]);
        let (present, outside) =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let inputs = Inputs {
            releases: &releases,
            pinned: "20200101",
            committed: &committed,
            present: &present,
            outside: &outside,
        };
        run(&model, &manifest, &inputs, only)
            .findings
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    fn findings() -> Vec<String> {
        findings_with(current_indexes)
    }

    /// Every planted defect this project carries, by the family that reports it.
    ///
    /// Each row is a family, the number of findings it owns, and a fragment of one of them.
    /// The rows must account for every finding a whole run produces, which
    /// `the_families_partition_every_finding` asserts, so a family cannot be left out of this
    /// table without a test failing.
    const PLANTED: [(Only, usize, &str); 8] = [
        (Only::CITATIONS, 5, "no rule says this"),
        (Only::SLUGS, 6, "is referenced"),
        (Only::PATHS, 1, "does not exist"),
        // Two missing documents, plus the manifest declaring `.git`, which this project does
        // not have. One planted row across the four declared path lists rather than four
        // identical ones: what needs pinning is that a declared path is checked at all.
        (Only::COMPONENTS, 3, "carries no"),
        (Only::INTERPRETATIONS, 1, "has no entry in the register"),
        (Only::UNCOVERED, 1, "is outside the walk"),
        (Only::GENERATED, 2, "the generated file is missing"),
        (Only::REGIME, 19, "with no verified quote of it in range"),
    ];

    #[test]
    fn a_set_of_families_reports_exactly_the_union_of_theirs() {
        let pair = Only::SLUGS.union(Only::PATHS);
        let found = findings_of(current_indexes, pair);
        assert_eq!(
            found.len(),
            7,
            "two misplaced slug defs, four slug defects and one path defect: {found:#?}"
        );
        assert!(
            !found.iter().any(|f| f.contains("no rule says this")),
            "the citation family did not run: {found:#?}"
        );
    }

    /// Every family, and the findings each produces when run alone.
    ///
    /// EVERY family is fed the same empty set of committed files, so `generated` has findings
    /// of its own in every run. Unlike the other eight it reports on what the tree does NOT
    /// contain, so under current indexes it is silent — and a family that is silent cannot
    /// leak visibly, which is how a deleted gate on it survived a leak test that gave the
    /// other families a different input.
    fn findings_per_family() -> Vec<(Only, Vec<String>)> {
        Only::NAMED
            .iter()
            .map(|(_, family)| (*family, findings_of(|_, _, _| HashMap::new(), *family)))
            .collect()
    }

    #[test]
    fn no_family_reports_a_finding_that_belongs_to_another() {
        // The leak test. A gate deleted from any family makes that family's findings appear
        // in every other family's run, so the pairwise intersection stops being empty. This
        // catches a leak from a family that plants no defect of its own, which counting
        // findings per family cannot.
        let per_family = findings_per_family();
        for (a, found_a) in &per_family {
            for (b, found_b) in &per_family {
                if a == b {
                    continue;
                }
                let shared: Vec<&String> = found_a.iter().filter(|f| found_b.contains(f)).collect();
                assert!(
                    shared.is_empty(),
                    "{} and {} both report {shared:#?}",
                    a.names().join(","),
                    b.names().join(",")
                );
            }
        }
    }

    #[test]
    fn the_families_partition_every_finding() {
        // Nothing is lost between the families and nothing is invented by running them
        // together, so `PLANTED` is a complete account rather than a sample.
        let per_family = findings_per_family();
        let mut apart: Vec<String> = per_family.iter().flat_map(|(_, f)| f.clone()).collect();
        let mut whole = findings_of(|_, _, _| HashMap::new(), Only::EVERYTHING);
        apart.sort();
        whole.sort();
        assert_eq!(apart, whole, "the families must partition a whole run");

        for (family, planted, _) in PLANTED {
            let (_, found) = per_family
                .iter()
                .find(|(f, _)| *f == family)
                .expect("a declared family");
            assert_eq!(
                found.len(),
                planted,
                "{} planted {planted}: {found:#?}",
                family.names().join(",")
            );
        }
    }

    #[test]
    fn every_family_reports_exactly_what_it_was_asked_for() {
        // `ran` under-reporting is as wrong as over-reporting, and only one direction was
        // pinned before: a report that names fewer families than ran makes a check that
        // happened look like one that did not.
        for (name, family) in Only::NAMED {
            let report = report_of(family);
            assert_eq!(report.ran, family, "{name}");
            assert_eq!(report.ran.names(), vec![name], "{name}");
            assert_eq!(report.asked, family, "{name}");
        }
    }

    #[test]
    fn the_pinned_containers_come_out_in_a_stable_order() {
        // The walk orders documents by path component and this list orders them by display
        // path, so the two disagree whenever a directory name is a prefix of another. Without
        // the sort the block reshuffles between runs on an unchanged tree.
        let report = report_with_pin(Only::CITATIONS);
        let files: Vec<&str> = report.pinned.iter().map(|(f, _, _)| f.as_str()).collect();
        assert!(
            files.len() >= 2,
            "the fixture must pin at least two: {files:#?}"
        );
        let mut sorted = files.clone();
        sorted.sort();
        assert_eq!(files, sorted, "pinned containers must be sorted");
    }

    /// One run's whole report, over `only`, with the generated files current.
    fn report_of(only: Only) -> documentation::check::Report {
        report_inner(only, false)
    }

    /// The same, with the pinned release the two `cr-version` fixtures name also supplied.
    ///
    /// A pinned document is skipped unless its release is in the map, so the fixtures are
    /// invisible to every other test here and change no count.
    fn report_with_pin(only: Only) -> documentation::check::Report {
        report_inner(only, true)
    }

    fn report_inner(only: Only, with_pin: bool) -> documentation::check::Report {
        let manifest = mock("planted");
        let model = Model::build(&manifest).expect("a model");
        let tree = manifest.rules_tree();
        let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
        let body = manifest.rules().body_starts_at;
        let corpus = rules::Corpus::parse(&text, body);
        let committed = current_indexes(&manifest, &model, &corpus);
        let mut releases = HashMap::from([(None, Release::new(&text, body))]);
        if with_pin {
            releases.insert(Some("20200101".to_string()), Release::new(&text, body));
        }
        let (present, outside) =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let inputs = Inputs {
            releases: &releases,
            pinned: "20200101",
            committed: &committed,
            present: &present,
            outside: &outside,
        };
        run(&model, &manifest, &inputs, only)
    }

    #[test]
    fn the_report_says_which_families_it_performed() {
        // Without this, a family carrying no count of its own — `generated`, `trackers` — is
        // indistinguishable from a run that performed nothing at all.
        let report = report_of(Only::COMPONENTS);
        assert_eq!(report.ran.names(), vec!["components"]);
        assert!(!report.ran.has(Only::SLUGS));
    }

    #[test]
    fn a_generated_file_that_has_drifted_is_reported_at_the_line_it_drifted_on() {
        let stale = findings_with(|m, model, corpus| {
            let mut c = current_indexes(m, model, corpus);
            let path = m.rules().dir.join("index.md");
            let text = c[&path].replace("Rule citation index", "Rule citation index (edited)");
            c.insert(path, text);
            c
        });
        let hits: Vec<&String> = stale.iter().filter(|f| f.contains("out of date")).collect();
        assert_eq!(hits.len(), 1, "{stale:#?}");
        assert!(hits[0].starts_with("corpus/index.md:1"), "{}", hits[0]);
    }

    #[test]
    fn a_generated_file_that_is_absent_is_reported_as_missing() {
        let gone = findings_with(|_, _, _| HashMap::new());
        assert_eq!(
            gone.iter().filter(|f| f.contains("is missing")).count(),
            2,
            "both generated files: {gone:#?}"
        );
    }

    fn one(needle: &str) -> String {
        let all = findings();
        let hits: Vec<&String> = all.iter().filter(|f| f.contains(needle)).collect();
        assert_eq!(hits.len(), 1, "expected exactly one {needle:?} in {all:#?}");
        hits[0].clone()
    }

    #[test]
    fn a_fabricated_quote_is_reported_as_verifying_against_nothing() {
        assert!(one("no rule says this").starts_with("notes/quotes.md:5"));
    }

    #[test]
    fn a_quote_that_belongs_to_another_rule_is_reported_as_misattributed() {
        // The dangerous case: the text exists, under a number that means something else, so
        // the citation still looks right to a reader.
        let f = one("verifies, but not as");
        assert!(f.starts_with("notes/quotes.md:7"), "{f}");
    }

    #[test]
    fn commentary_inside_a_blockquote_is_reported() {
        assert!(one("commentary, not rule text").starts_with("notes/quotes.md:9"));
    }

    #[test]
    fn a_rule_number_with_no_marker_is_linted() {
        assert!(one("named with no marker").starts_with("notes/quotes.md:11"));
    }

    #[test]
    fn an_identifier_marker_with_no_prose_marker_above_it_is_reported() {
        assert!(one("has no CR:100.1 above it").starts_with("code/lib.rs:2"));
    }

    #[test]
    fn a_slug_referenced_in_a_component_that_does_not_define_it_is_reported() {
        let f = one("`planted#dangling-anchor` is referenced");
        assert!(f.starts_with("notes/structure.md:9"), "{f}");
    }

    #[test]
    fn a_reference_naming_no_component_is_reported() {
        let f = one("`#unqualified-anchor` names no component");
        assert!(f.starts_with("notes/structure.md:11"), "{f}");
    }

    #[test]
    fn a_reference_naming_a_component_that_is_not_declared_is_reported() {
        // A different repair from the two above, and the finding says which: the slug exists,
        // and the word before the `#` is what nothing resolves.
        let f = one("which is no component of this project");
        assert!(f.starts_with("notes/structure.md:13"), "{f}");
    }

    #[test]
    fn a_reference_across_a_component_boundary_resolves() {
        // Line 15 of that document points at the decision the component below the root
        // records, and line 5 of the component's own design document points back. Neither is
        // a finding, and the count in `PLANTED` is what asserts that they produce none.
        let all = findings();
        assert!(
            !all.iter().any(|f| f.contains("widget-decision")),
            "a qualified reference that resolves must be silent: {all:#?}"
        );
    }

    #[test]
    fn a_slug_defined_twice_in_one_component_is_reported_once_naming_both_places() {
        // A rename that left one behind. The reader who finds the stale one acts on it.
        let f = one("is defined 2 times");
        assert!(f.starts_with("notes/structure.md:3"), "{f}");
        assert!(f.contains("notes/structure.md:7"), "{f}");
        assert!(f.contains("`planted#twice-defined`"), "{f}");
    }

    #[test]
    fn a_path_that_does_not_resolve_is_reported() {
        // Named rather than matched on "does not exist": a declared tracker that is not there
        // says the same words, and a needle matching both would pass while checking neither.
        assert!(one("`notes/missing.md` does not exist").starts_with("notes/structure.md:17"));
    }

    #[test]
    fn a_reference_to_an_entry_that_does_not_exist_is_reported() {
        assert!(one("has no entry in the register").starts_with("notes/structure.md:19"));
    }

    #[test]
    fn a_component_document_that_is_missing_is_reported_at_the_path_it_belongs_at() {
        let f = one("carries no docs/tripwires.md");
        assert!(f.starts_with("parts/widget/docs/tripwires.md"), "{f}");
        assert!(f.contains("`widget`"), "{f}");
    }

    #[test]
    fn a_declared_tracker_outside_every_component_that_is_missing_is_reported() {
        // The report reads the declared paths, so this one would make it under-count what is
        // open rather than fail — which is what the declaration exists to prevent.
        let f = one("is declared an additional tracker");
        assert!(f.starts_with("notes/open-issues.md"), "{f}");
    }

    #[test]
    fn the_correct_quote_produces_no_finding_and_the_exempt_file_is_not_linted() {
        let all = findings();
        assert!(
            !all.iter().any(|f| f.contains("exempt.md")),
            "the manifest exempts that file from the lint: {all:#?}"
        );
        // Every planted defect found and nothing else, counted against `PLANTED` so the two
        // cannot drift apart. Taken over the same empty set of committed files that table is
        // stated against, which is what gives `generated` its two findings.
        let planted: usize = PLANTED.iter().map(|(_, n, _)| n).sum();
        let whole = findings_of(|_, _, _| HashMap::new(), Only::EVERYTHING);
        assert_eq!(whole.len(), planted, "{whole:#?}");
    }
}

/// The citation regime, against a project that plants one violation of each of its rules.
///
/// The rules are judged against SCOPES, so a string fixture cannot exercise them: what a
/// finding turns on is which section or item a claim sits in and how far its quote is, and
/// neither exists until a real document is walked.
mod regime {
    use super::*;
    use documentation::check::citations::Release;
    use documentation::check::regime::{self, Rule};

    /// Every finding the regime produces over the planted project.
    fn judged() -> Vec<(Rule, String)> {
        let manifest = mock("planted");
        let model = Model::build(&manifest).expect("a model");
        let tree = manifest.rules_tree();
        let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
        let release = Release::new(&text, manifest.rules().body_starts_at);
        let mut out = Vec::new();
        for doc in model.documents() {
            if doc.pin.is_some() {
                continue;
            }
            let (found, _) = regime::check(doc, &release);
            for j in found {
                out.push((
                    j.rule,
                    format!("{}  {}", j.finding.location(), j.finding.what),
                ));
            }
        }
        out
    }

    fn of(rule: Rule) -> Vec<String> {
        judged()
            .into_iter()
            .filter(|(r, _)| *r == rule)
            .map(|(_, w)| w)
            .collect()
    }

    fn one(rule: Rule, needle: &str) {
        let found = of(rule);
        assert!(
            found.iter().any(|f| f.contains(needle)),
            "expected `{}` to report {needle:?}, got {found:#?}",
            rule.name()
        );
    }

    #[test]
    fn every_rule_of_the_regime_has_a_planted_violation() {
        // A rule with no fixture is a rule nothing proves fires. The list is the tool's own,
        // so adding a rule without planting one fails here rather than passing silently.
        let fired: Vec<Rule> = judged().into_iter().map(|(r, _)| r).collect();
        let missing: Vec<&str> = Rule::NAMED
            .iter()
            .filter(|(_, r)| !fired.contains(r))
            .map(|(n, _)| *n)
            .collect();
        assert!(missing.is_empty(), "no planted violation for: {missing:?}");
    }

    #[test]
    fn a_quote_two_markers_could_own_is_reported_when_both_rules_hold_its_text() {
        // The fixture plants the whole shape rather than the one finding, because all three
        // are what a writer meets: the binding is ambiguous, the rule it bound to has a tail
        // the quote drops, and the rule the writer meant is left with no quote in range. The
        // last is the one whose hint sends them at the wrong repair, which is why the first
        // exists.
        one(
            Rule::QuoteBindingIsAmbiguous,
            "bound to 100.6 because that marker is nearest, and its text is also 100.5",
        );
        one(Rule::QuoteInScope, "100.5 is claimed");
    }

    #[test]
    fn a_quote_whose_characters_are_not_the_releases_is_reported() {
        // Verification folds typography on both sides so the words alone cannot catch it, and
        // 1 933 of the pinned release's 3 162 rules carry a character that folds.
        one(
            Rule::QuoteTypography,
            "does not use the release's own characters",
        );
    }

    #[test]
    fn a_claim_with_no_quote_in_its_scope_is_reported() {
        one(Rule::QuoteInScope, "no verified quote of it in range");
    }

    #[test]
    fn a_quote_in_a_neighbouring_scope_does_not_discharge_a_claim() {
        // The whole point of the innermost rule: the quote exists in the document, one
        // section away, and the reader arriving by grep never sees it.
        let found = of(Rule::QuoteInScope);
        assert!(
            found.iter().any(
                |f| f.contains("A claim whose quote is in a different scope")
                    || f.contains("100.1 is claimed")
            ),
            "{found:#?}"
        );
    }

    #[test]
    fn a_claim_quoted_beside_it_is_not_reported() {
        // The negative half. Without it the rule could fire on everything and still pass.
        let found = of(Rule::QuoteInScope);
        assert!(
            !found.iter().any(|f| f.contains("code/regime.rs")),
            "a doc comment carrying its own quote must pass: {found:#?}"
        );
    }

    #[test]
    fn a_number_the_release_does_not_hold_is_reported() {
        one(Rule::NumberResolves, "no such rule");
    }

    #[test]
    fn the_identifier_form_written_in_prose_is_reported() {
        one(Rule::IdentifierInProse, "identifier form names");
    }

    #[test]
    fn a_name_quoting_part_of_its_rule_is_reported_and_the_whole_body_is_not() {
        one(Rule::IdentifierFullQuote, "does not carry its whole body");
        let found = of(Rule::IdentifierFullQuote);
        assert!(
            !found.iter().any(|f| f.contains("100.4")),
            "a name whose rule is quoted entire must pass: {found:#?}"
        );
    }

    #[test]
    fn an_unmarked_omission_is_reported_at_the_end_it_drops() {
        let found = of(Rule::OmissionMarked);
        assert!(
            found.iter().any(|f| f.contains("drops the rule's opening")),
            "{found:#?}"
        );
        assert!(
            found.iter().any(|f| f.contains("drops the rule's tail")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_fragment_under_the_floor_is_reported() {
        one(Rule::FragmentLongEnough, "under the floor");
    }

    #[test]
    fn a_parent_rule_quoted_whole_is_reported_and_a_leaf_rule_is_not() {
        one(Rule::ParentRuleIsNotItsSubrules, "has subrules of its own");
        let found = of(Rule::ParentRuleIsNotItsSubrules);
        assert!(
            !found.iter().any(|f| f.contains("100.4")),
            "a rule with no subrules must pass: {found:#?}"
        );
    }

    #[test]
    fn a_rule_number_that_is_data_produces_no_claim() {
        // A code span, and a string bound to a name. Neither is a citation, so neither can
        // be reported for lacking a quote.
        let found = of(Rule::QuoteInScope);
        assert!(
            !found
                .iter()
                .any(|f| f.contains("A rule number that is data")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_quotation_no_marker_claims_is_reported() {
        one(Rule::QuoteHasNoMarker, "no marker claims it");
    }

    #[test]
    fn a_quote_in_the_enclosing_section_does_not_discharge_a_claim_in_a_subsection() {
        // There is no outward search, and this is the fixture that pins it: the quote is one
        // level out, which a reader arriving by grep at the subsection never sees.
        one(Rule::QuoteInScope, "A claim in the subsection under it");
    }

    #[test]
    fn a_claim_further_from_its_quote_than_the_cap_is_reported() {
        // Same section, same rule, quote present — only the distance differs. Without this
        // the cap could be removed entirely and every test would still pass.
        one(
            Rule::QuoteInScope,
            "A claim further from its quote than the cap allows",
        );
    }
}
