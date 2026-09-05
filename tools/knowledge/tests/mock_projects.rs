//! The tool run against whole projects, not against strings.
//!
//! A mock project under `path@knowledge@tests/projects/` is a complete project: it carries
//! its own `knowledge.toml` and its own documents. That is what makes these tests worth more
//! than the in-memory ones — a fixture written as a string cannot exercise the walk, the
//! exclusions, or a layout different from this repository's.
//!
//! It is also why the directory stays excluded. The projects beside this file sit under
//! `path@knowledge@tests/projects/`, which this repository's own manifest excludes: each mock project is
//! a complete foreign project, and its planted defects — a slug, a dangling path, a rule number
//! with no quote — must be reported by the test that runs the tool over it, never as this
//! repository's own.
//!
//! Cargo compiles every `.rs` file directly under `path@knowledge@tests/`, so this file is a test target and `projects/` beside it is
//! not: a directory without a `main.rs` is data.

use std::path::PathBuf;

use documentation::{Manifest, Model, Observation, RetiredForm};

fn mock(name: &str) -> Manifest {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name);
    Manifest::load(&root).expect("the mock project's manifest")
}

fn model(name: &str) -> Model {
    Model::build(&mock(name), None).expect("a model of the mock project")
}

/// Every `register.toml` beside an instance, as the binary reads them for a check.
fn configs(manifest: &Manifest) -> std::collections::HashMap<PathBuf, String> {
    let mut out = std::collections::HashMap::new();
    for (_, _, home) in documentation::entity::Anchors::of(manifest).instances() {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&home.config)) {
            out.insert(home.config.clone(), text);
        }
    }
    out
}

/// What the binary asks git for, taken over a mock project in place.
///
/// These tests build their models under this repository's own worktree, so git answers about
/// the mock's files out of this repository's own listing. That listing is
/// `--cached --others --exclude-standard`, so a new fixture file is in it whether or not it is
/// staged; an ignore rule is what takes one out, and a deletion has to be staged or the path
/// stays in the listing with no bytes behind it.
fn git_answers(
    manifest: &documentation::Manifest,
    model: &Model,
) -> (std::collections::HashSet<String>, Vec<PathBuf>) {
    let anchors = documentation::entity::Anchors::of(manifest);
    let queries = documentation::check::references::ignore_queries(model, &anchors);
    let ignored =
        documentation::git::ignored(manifest.root(), &queries).expect("git answers the batch");
    let tracked =
        documentation::git::tracked_and_ignored(manifest.root()).expect("git answers the listing");
    (ignored, tracked)
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
            "docs/goals.md".to_string(),
            "docs/open-issues/README.md".to_string(),
            "docs/open-issues/the-mock-has-one-issue.md".to_string(),
            "docs/rejected-alternatives.md".to_string(),
            "docs/tripwires.md".to_string(),
            "notes/a.md".to_string(),
            "notes/b.md".to_string(),
            "notes/decisions.md".to_string(),
            "notes/open-issues/README.md".to_string(),
            "notes/open-issues/the-notes-are-not-a-component.md".to_string(),
            "notes/readings/README.md".to_string(),
            "notes/readings/one-concern/a-reading-the-mock-records.md".to_string(),
        ],
        "the walk should hold every markdown and Rust file, minus every exclusion and \
         every generated index"
    );
}

/// A generated index leaves the walk because the tool derives the set from the register
/// instances, and no manifest row names one.
///
/// Both halves matter. Inside the walk, the rows of a listing would be read as this project's
/// own claims; inside the inverse assertion of `uncovered`, they would be reported as a file no
/// checker reads. The mock declares one `skip-files` row and it names neither index.
#[test]
fn a_generated_index_is_outside_the_walk_and_outside_the_inverse_assertion() {
    let manifest = mock("minimal");
    let model = model("minimal");
    let generated = documentation::index::generated_index_paths(&manifest);
    assert_eq!(generated.len(), 3, "{generated:?}");
    for rel in &generated {
        assert!(
            !manifest.walk().skip_files.contains(rel),
            "{} is declared, not derived",
            rel.display()
        );
    }
    let walked = walked(&model);
    let survey = documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
    for rel in &generated {
        let name = rel.display().to_string();
        assert!(
            std::path::Path::new(&manifest.root().join(rel)).exists(),
            "{name} should be a file the mock carries"
        );
        assert!(!walked.contains(&name), "{name} should not be walked");
        assert!(
            !survey.outside.iter().any(|(p, _)| p == rel),
            "{name} should not be reported as a file no checker reads"
        );
        assert!(survey.present.contains(rel), "{name} still exists");
    }
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
    // A slug opening a decision, a path reference, a goal reference, and the two marker
    // forms — each at the line of the file it sits on.
    assert!(
        dump.contains("docs/design.md\t5\tslug-def\tmock-anchor heading-3\n"),
        "{dump}"
    );
    // The whole value, and the trailing newline is load-bearing. This assertion once named
    // the path alone, which is a prefix of the component-suffixed value the renderer wrongly
    // produced, so it passed against both the right output and the wrong one for as long as
    // the defect existed. A `contains` over a field that is not terminated asserts a prefix.
    assert!(
        dump.contains("docs/design.md\t7\tspan\tpath@notes@b.md\n"),
        "{dump}"
    );
    assert!(
        dump.contains("docs/design.md\t7\tmarker-prose\t100.1"),
        "{dump}"
    );
    assert!(
        dump.contains("notes/a.md\t5\tspan\tgoal@minimal@mock-goal\n"),
        "{dump}"
    );
    assert!(
        dump.contains("code/lib.rs\t1\tmarker-prose\t100.1"),
        "{dump}"
    );
}

#[test]
fn the_survey_records_which_paths_are_directories() {
    // The kind is a fact of the listing rather than an inference from entries beneath a
    // path, which could not tell an empty directory from a file.
    let manifest = mock("minimal");
    let model = Model::build(&manifest, None).expect("a model");
    let survey = documentation::survey::survey(&manifest, &model).expect("a survey");
    assert!(survey.directories.contains(&PathBuf::from("docs")));
    assert!(survey.present.contains(&PathBuf::from("docs")));
    assert!(!survey.directories.contains(&PathBuf::from("README.md")));
    assert!(survey.present.contains(&PathBuf::from("README.md")));
}

#[test]
fn a_location_outside_every_component_is_read_by_the_listings() {
    // What a location is for: a directory carrying a subset of the registers and nothing else
    // a component carries. Undeclared, this entry is in no listing and nobody finds it.
    use documentation::entity::{Anchors, Entities, Kind};
    use documentation::manifest::ISSUE_REGISTER;

    let manifest = mock("minimal");
    let model = model("minimal");
    let anchors = Anchors::of(&manifest);
    let entities = Entities::build(&model, &anchors);
    let rows =
        documentation::records::records(&model, &anchors, &entities, &Kind::new(ISSUE_REGISTER));
    let here: Vec<_> = rows.iter().filter(|r| r.anchor == "notes").collect();
    assert_eq!(here.len(), 1, "{rows:#?}");
    // The kind comes out of the entry's own frontmatter, which is what the register declares,
    // and the anchor is the location rather than the component above it.
    assert_eq!(here[0].metadata.as_deref(), Some("observation"));
    assert_eq!(here[0].title, "The notes are not a component");
    assert_eq!(here[0].id, "the-notes-are-not-a-component");
    assert_eq!(
        here[0].site.file,
        PathBuf::from("notes/open-issues/the-notes-are-not-a-component.md")
    );
    // The component's own instance is a second anchor, so the two are not one listing.
    assert!(rows.iter().any(|r| r.anchor == "minimal"), "{rows:#?}");
}

#[test]
fn a_fenced_illustration_is_neither_a_definition_nor_a_reference_in_a_real_file() {
    // The fenced block in that document holds a heading-shaped definition and a placeholder
    // reference, which is what a document explaining the convention holds. Neither is an
    // observation; the real definitions and pointers elsewhere in the project are, and
    // nothing in this project is written in a retired form.
    let model = model("minimal");
    let observed = |f: fn(&Observation) -> Option<String>| -> Vec<String> {
        let mut out: Vec<String> = model
            .documents()
            .iter()
            .flat_map(|d| d.observations.iter())
            .filter_map(|l| f(&l.what))
            .collect();
        out.sort();
        out
    };
    let defs = observed(|o| match o {
        Observation::SlugDef { id, .. } => Some(id.clone()),
        _ => None,
    });
    assert_eq!(
        defs,
        vec![
            "a-note-the-location-keeps".to_string(),
            "mock-anchor".to_string(),
            "mock-goal".to_string(),
            "mock-tripwire".to_string()
        ]
    );
    let spans = observed(|o| match o {
        Observation::Span(s) => Some(s.clone()),
        _ => None,
    });
    assert_eq!(
        spans,
        vec![
            "design@minimal@mock-anchor".to_string(),
            "design@minimal@mock-anchor".to_string(),
            "design@minimal@mock-anchor".to_string(),
            "goal@minimal@mock-goal".to_string(),
            "note@notes@a-note-the-location-keeps".to_string(),
            "path@notes@b.md".to_string(),
            "path@notes@decisions.md".to_string(),
            "tripwire@minimal@mock-tripwire".to_string(),
        ]
    );
    let retired = observed(|o| match o {
        Observation::Retired(RetiredForm::SlugRef(s)) => Some(s.clone()),
        Observation::Retired(RetiredForm::RegisterNumber(n)) => Some(format!("R{n}")),
        _ => None,
    });
    assert_eq!(retired, Vec::<String>::new());
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
    let model = Model::build(&manifest, None).expect("a model");
    let releases: HashMap<Option<String>, Release> = HashMap::new();
    let committed = HashMap::new();
    let survey = documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
    let git = git_answers(&manifest, &model);
    let inputs = Inputs {
        releases: &releases,
        pinned: "20200101",
        committed: &committed,
        configs: &configs(&manifest),
        present: &survey.present,
        directories: &survey.directories,
        outside: &survey.outside,
        ignored: &git.0,
        tracked_and_ignored: &git.1,
    };
    let report = run(
        &model,
        &manifest,
        &inputs,
        Only::REGISTERS.union(Only::REFERENCES),
    );
    let found: Vec<String> = report.findings.iter().map(|f| f.to_string()).collect();
    assert!(found.is_empty(), "{found:#?}");
    // The component at the root is one whether or not anything is declared beside it, and
    // every reference resolves against it by the project's own name.
    assert_eq!(report.structure.components, 1);
    assert_eq!(report.structure.locations, 1);
    // The root component's four registers, and the location's three.
    assert_eq!(report.structure.instances, 7);
    assert_eq!(report.structure.entries, 3);
    // Four heading entities — one in each built-in register and one in the declared heading
    // register the location carries — three file entities across the three file instances, and
    // eight references across the four heading kinds and two paths.
    assert_eq!(
        (report.structure.entities, report.structure.references),
        (7, 8)
    );
}

/// Every index a mock project commits is what the generator writes, byte for byte.
///
/// `planted` is excluded because a stale index and a missing one are two of its planted
/// `generated` defects. For the other four the property is the opposite one: a committed index
/// that has drifted would make every test over that project run against a listing the tree does
/// not have, and the drift is invisible until someone runs `cargo knowledge index`.
///
/// The rule index is compared only where a project carries one. `minimal` deliberately carries
/// none, which is what lets `index_rewrites_what_moved_and_leaves_what_is_current_alone` in
/// `path@knowledge@tests/binary.rs` watch a generated file be created.
#[test]
fn every_committed_index_is_what_the_generator_writes() {
    for name in ["minimal", "dirhome", "pinned", "typography"] {
        let manifest = mock(name);
        let model = model(name);
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        for (rel, want) in
            documentation::index::file_register_indexes(&model, &manifest, &survey.directories)
        {
            let got = std::fs::read_to_string(manifest.root().join(&rel))
                .unwrap_or_else(|e| panic!("{name}: {}: {e}", rel.display()));
            assert_eq!(got, want, "{name}: {} has drifted", rel.display());
        }
        let rel = manifest.rules().dir.join("index.md");
        let Ok(got) = std::fs::read_to_string(manifest.root().join(&rel)) else {
            continue;
        };
        let text = std::fs::read_to_string(manifest.rules_tree().text()).expect("the mock corpus");
        let corpus = rules::Corpus::parse(&text, manifest.rules().body_starts_at);
        let want = documentation::index::rule_index(&model, &manifest, &corpus, "20200101");
        assert_eq!(got, want, "{name}: {} has drifted", rel.display());
    }
}

/// `dirhome` is the conformant fixture, and this is the assertion that keeps it one.
///
/// It carries the other accepted heading-register shape — `path@*@docs/design/` and
/// `path@*@docs/goals/`, each headed by a README linking its subdocument, with the entries defined
/// in the subdocuments — and the only ISSUE instance in any mock that declares a group. Every
/// generated file it holds is committed and current.
///
/// **Both directions matter.** `planted` shows that the families still detect; a project that
/// is right in every shape shows that they do not report over a conformant tree, which is what
/// a false positive would look like. The counterpart at the process boundary is
/// `the_conformant_mock_passes_every_family` in `path@knowledge@tests/binary.rs`, which reaches the
/// two families `run` does not carry.
#[test]
fn the_conformant_mock_reports_nothing_over_every_family_the_model_carries() {
    use documentation::check::citations::Release;
    use documentation::check::{run, Inputs, Only};
    use std::collections::HashMap;

    let manifest = mock("dirhome");
    let model = Model::build(&manifest, None).expect("a model");
    let tree = manifest.rules_tree();
    let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
    let body = manifest.rules().body_starts_at;
    let releases = HashMap::from([(None, Release::new(&text, body))]);
    let survey = documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
    // The committed generated files, read from the tree rather than regenerated, so a
    // committed index that has gone stale fails here.
    let mut committed = HashMap::new();
    let mut paths = vec![manifest.rules().dir.join("index.md")];
    paths.extend(documentation::index::generated_index_paths(&manifest));
    for rel in paths {
        let text = std::fs::read_to_string(manifest.root().join(&rel)).expect("a committed index");
        committed.insert(rel, text);
    }
    let git = git_answers(&manifest, &model);
    let inputs = Inputs {
        releases: &releases,
        pinned: "20200101",
        committed: &committed,
        configs: &configs(&manifest),
        present: &survey.present,
        directories: &survey.directories,
        outside: &survey.outside,
        ignored: &git.0,
        tracked_and_ignored: &git.1,
    };
    let report = run(&model, &manifest, &inputs, Only::EVERYTHING);
    let found: Vec<String> = report.findings.iter().map(|f| f.to_string()).collect();
    assert!(found.is_empty(), "{found:#?}");
    // Two slugs and two issue entries, one of them grouped: a run that stopped reading the
    // group subdirectory would still report nothing, and this is what says it read it.
    assert_eq!(
        (report.structure.entities, report.structure.references),
        (4, 2)
    );
    assert_eq!(report.structure.entries, 2, "one grouped, one ungrouped");
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
        let survey = documentation::survey::survey(manifest, model).expect("a survey of the mock");
        let mut out = HashMap::from([(
            manifest.rules().dir.join("index.md"),
            index::rule_index(model, manifest, corpus, "20200101"),
        )]);
        out.extend(index::file_register_indexes(
            model,
            manifest,
            &survey.directories,
        ));
        out
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
        let model = Model::build(&manifest, None).expect("a model");
        let tree = manifest.rules_tree();
        let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
        let release = Release::new(&text, manifest.rules().body_starts_at);
        let corpus = rules::Corpus::parse(&text, manifest.rules().body_starts_at);
        let committed = committed(&manifest, &model, &corpus);
        let releases = HashMap::from([(None, release)]);
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            releases: &releases,
            pinned: "20200101",
            committed: &committed,
            configs: &configs(&manifest),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
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

    /// Where a family's planted defect is asserted.
    ///
    /// Two variants because two families are not in `run`. `changes` reads the changelog and
    /// `corpus` reads the archive, and neither subject is the model, so the binary calls them
    /// and a test over `run` can only assert that they contribute nothing to it.
    enum Planted {
        /// `run` reports it. The number is how many findings the family produces alone.
        InRun(usize),
        /// The binary reports it. The name is the test in `path@knowledge@tests/binary.rs` that
        /// asserts the finding, and `run` must produce nothing for the family.
        ByTheBinary(&'static str),
    }

    /// Every planted defect this project carries, by the family that reports it.
    ///
    /// Each row is a family, where its planted defect is asserted, and a fragment of one of
    /// them. `the_families_partition_every_finding` reads `Only::NAMED` and demands a row for
    /// each name, so a family with no planted defect fails that test rather than passing
    /// unnoticed.
    const PLANTED: [(Only, Planted, &str); 8] = [
        (Only::CITATIONS, Planted::InRun(5), "no rule says this"),
        // One reference of each shape the resolver tells apart — dangling, unknown anchor,
        // two and four segments, an anchor and a reserved anchor in kind position, the two
        // retired slug shapes, a retired entry number — and the path shapes: a dangling one,
        // the unanchored bare form, a wrong kind claim, an escape that resolves here, a root
        // pointer reaching inside the component, a generic pointer nothing carries, a
        // dangling tripwire reference, a link outside a navigation home, and the retired `@`
        // escape with its empty head. The definition-site findings are `registers`'.
        (Only::REFERENCES, Planted::InRun(18), "is referenced"),
        // One defect per assertion the register shapes make: a declared path that is not
        // there, a missing heading home, the retired file shape of a file register, a missing
        // index, an undeclared kind, a missing owed subsection, an id no reference can spell,
        // an undeclared group, a file of another suffix, frontmatter that does not parse, a
        // location whose home is absent, and the six definition-site findings the entity table
        // produces.
        (Only::REGISTERS, Planted::InRun(17), "PLANTED"),
        (Only::UNCOVERED, Planted::InRun(1), "is outside the walk"),
        // Three generated files: the rule index, and one index per file-register instance
        // whose directory is there. Every family is handed an empty committed set, so each
        // is reported missing.
        (
            Only::GENERATED,
            Planted::InRun(3),
            "the generated file is missing",
        ),
        (
            Only::REGIME,
            Planted::InRun(20),
            "with no verified quote of it in range",
        ),
        // The changelog's one section quotes a rule as something the release does not say.
        (
            Only::CHANGES,
            Planted::ByTheBinary("the_changelog_and_the_archive_each_carry_a_planted_defect"),
            "the quoted text is not what",
        ),
        // The archived release's bytes are not the ones its manifest row records.
        (
            Only::CORPUS,
            Planted::ByTheBinary("the_changelog_and_the_archive_each_carry_a_planted_defect"),
            "does not match the manifest's",
        ),
    ];

    #[test]
    fn a_set_of_families_reports_exactly_the_union_of_theirs() {
        let pair = Only::REFERENCES.union(Only::REGISTERS);
        let found = findings_of(current_indexes, pair);
        assert_eq!(
            found.len(),
            35,
            "eighteen reference defects and seventeen register defects: {found:#?}"
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

        // Read off `Only::NAMED` rather than off `PLANTED`, so a family added to the library
        // with no planted defect fails here instead of being absent from both.
        for (name, family) in Only::NAMED {
            let (_, planted, _) = PLANTED
                .iter()
                .find(|(f, _, _)| *f == family)
                .unwrap_or_else(|| panic!("{name} has no row: every family owes a planted defect"));
            let (_, found) = per_family
                .iter()
                .find(|(f, _)| *f == family)
                .expect("a declared family");
            match planted {
                Planted::InRun(n) => assert_eq!(found.len(), *n, "{name} planted {n}: {found:#?}"),
                Planted::ByTheBinary(test) => assert!(
                    found.is_empty(),
                    "{name} is asserted by {test}, so `run` must report nothing: {found:#?}"
                ),
            }
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
        let model = Model::build(&manifest, None).expect("a model");
        let tree = manifest.rules_tree();
        let text = std::fs::read_to_string(tree.text()).expect("the mock corpus");
        let body = manifest.rules().body_starts_at;
        let corpus = rules::Corpus::parse(&text, body);
        let committed = current_indexes(&manifest, &model, &corpus);
        let mut releases = HashMap::from([(None, Release::new(&text, body))]);
        if with_pin {
            releases.insert(Some("20200101".to_string()), Release::new(&text, body));
        }
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            releases: &releases,
            pinned: "20200101",
            committed: &committed,
            configs: &configs(&manifest),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
        };
        run(&model, &manifest, &inputs, only)
    }

    #[test]
    fn the_report_says_which_families_it_performed() {
        // Without this, a family carrying no count of its own — `generated`, `trackers` — is
        // indistinguishable from a run that performed nothing at all.
        let report = report_of(Only::REGISTERS);
        assert_eq!(report.ran.names(), vec!["registers"]);
        assert!(!report.ran.has(Only::REFERENCES));
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

    /// A file-register index is judged the same way, and by its own bytes.
    ///
    /// The hand edit is in the rows rather than in the banner, because the banner is the one
    /// line a generator that produced nothing at all would still get right.
    #[test]
    fn a_hand_edited_file_register_index_is_reported_at_the_line_that_was_edited() {
        let path = PathBuf::from("parts/widget/docs/open-issues/index.md");
        let edited = findings_with(|m, model, corpus| {
            let mut c = current_indexes(m, model, corpus);
            let text = c[&path].replace("| todo |", "| defect |");
            c.insert(path.clone(), text);
            c
        });
        let hits: Vec<&String> = edited
            .iter()
            .filter(|f| f.contains("out of date"))
            .collect();
        assert_eq!(hits.len(), 1, "{edited:#?}");
        // The banner, the blank line, the count, the blank line, the header, the rule, the
        // row: the edit is in the row and the finding names it.
        assert!(
            hits[0].starts_with("parts/widget/docs/open-issues/index.md:7"),
            "{}",
            hits[0]
        );
    }

    /// The index committed beside the widget's issue instance still says the instance is
    /// empty, and an entry sits beside it. This is the planted defect for a stale listing, and
    /// it is read off the tree rather than supplied, so nothing but the generator decides it.
    #[test]
    fn an_index_the_tree_holds_stale_is_reported_against_the_bytes_on_disk() {
        let manifest = mock("planted");
        let model = Model::build(&manifest, None).expect("a model");
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let generated = index::file_register_indexes(&model, &manifest, &survey.directories);
        let path = PathBuf::from("parts/widget/docs/open-issues/index.md");
        let (_, expected) = generated
            .iter()
            .find(|(rel, _)| *rel == path)
            .expect("the widget instance is generated");
        let text =
            std::fs::read_to_string(manifest.root().join(&path)).expect("the committed index");
        assert_ne!(&text, expected, "the planted index is stale");
        assert!(text.contains("0 entries"), "{text}");
        assert!(expected.contains("1 entries"), "{expected}");

        // And the family reports it, over the bytes the tree holds rather than over a map a
        // test wrote: the count line is line three, and that is what the finding names.
        let stale = findings_of(
            |m, _, _| {
                let mut out = HashMap::new();
                for rel in index::generated_index_paths(m) {
                    if let Ok(text) = std::fs::read_to_string(m.root().join(&rel)) {
                        out.insert(rel, text);
                    }
                }
                out
            },
            Only::GENERATED,
        );
        assert_eq!(
            stale,
            vec![
                "corpus/index.md  the generated file is missing".to_string(),
                "docs/open-issues/index.md  the generated file is missing".to_string(),
                "parts/widget/docs/open-issues/index.md:3  the generated file is out of date"
                    .to_string(),
            ],
            "{stale:#?}"
        );
    }

    #[test]
    fn a_generated_file_that_is_absent_is_reported_as_missing() {
        let gone = findings_with(|_, _, _| HashMap::new());
        assert_eq!(
            gone.iter().filter(|f| f.contains("is missing")).count(),
            3,
            "the rule index and the two file-register indexes: {gone:#?}"
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
    fn a_reference_whose_id_the_anchor_does_not_define_is_reported() {
        let f = one("`design@planted@dangling-anchor` is referenced");
        assert!(f.starts_with("notes/structure.md:5"), "{f}");
        assert!(f.contains("defines no design `dangling-anchor`"), "{f}");
    }

    #[test]
    fn a_reference_naming_an_anchor_that_is_not_declared_is_reported() {
        // A different repair from the one above, and the finding says which: the id exists,
        // and the anchor segment is what nothing resolves.
        let f = one("`design@nowhere@twice-defined` names `nowhere`");
        assert!(f.starts_with("notes/structure.md:7"), "{f}");
    }

    #[test]
    fn a_malformed_reference_is_reported_naming_the_segment_count() {
        let two = one("`design@planted` is malformed");
        assert!(two.starts_with("notes/structure.md:9"), "{two}");
        let four = one("`design@planted@twice@defined` is malformed");
        assert!(four.starts_with("notes/structure.md:11"), "{four}");
    }

    #[test]
    fn an_anchor_in_kind_position_is_reported_for_a_component_and_a_reserved_word() {
        let old = one("`planted@notes/p.md` opens with `planted`");
        assert!(old.starts_with("notes/structure.md:13"), "{old}");
        let reserved = one("`*@notes/p.md` opens with `*`");
        assert!(reserved.starts_with("notes/structure.md:15"), "{reserved}");
    }

    #[test]
    fn each_retired_form_is_reported_as_what_it_was() {
        let qualified = one("`planted#twice-defined` is the retired slug reference form");
        assert!(
            qualified.starts_with("notes/structure.md:17"),
            "{qualified}"
        );
        let bare = one("`#unqualified-anchor` is the retired slug reference form");
        assert!(bare.starts_with("notes/structure.md:19"), "{bare}");
        let number = one("`R99` is the retired interpretation entry number form");
        assert!(number.starts_with("notes/structure.md:21"), "{number}");
    }

    #[test]
    fn a_reference_across_a_component_boundary_resolves() {
        // Line 23 of that document points at the decision the component below the root
        // records, and the component's own design document points back. Neither is a
        // finding, and the count in `PLANTED` is what asserts that they produce none.
        let all = findings();
        assert!(
            !all.iter().any(|f| f.contains("widget-decision")),
            "a qualified reference that resolves must be silent: {all:#?}"
        );
    }

    #[test]
    fn a_slug_defined_twice_in_one_instance_is_reported_at_both_sites() {
        // A rename that left one behind. The reader who opens either copy is told about the
        // other.
        let all = findings();
        let dup: Vec<&String> = all
            .iter()
            .filter(|f| f.contains("`design@planted@twice-defined` is also defined at"))
            .collect();
        assert_eq!(dup.len(), 2, "{all:#?}");
        assert!(
            dup.iter()
                .any(|f| f.starts_with("docs/design.md:5") && f.contains("docs/design.md:9")),
            "{dup:#?}"
        );
        assert!(
            dup.iter()
                .any(|f| f.starts_with("docs/design.md:9") && f.contains("docs/design.md:5")),
            "{dup:#?}"
        );
    }

    #[test]
    fn a_misplaced_definition_is_reported_where_it_stands_and_defines_nothing() {
        // Three shapes over a real walk: a level-four heading and a line head inside the
        // design home, and a level-three heading in a file that is no register home.
        let deep = one("`##too-deep` is written at a level-4 heading");
        assert!(deep.starts_with("docs/design.md:11"), "{deep}");
        let head = one("`##line-head` is written at the head of a plain line");
        assert!(head.starts_with("docs/design.md:13"), "{head}");
        let stray = one("`##stray-anchor` is written at `notes/structure.md`");
        assert!(stray.starts_with("notes/structure.md:3"), "{stray}");
        assert!(
            stray.contains("no heading register home of `planted`"),
            "{stray}"
        );
        let inline = one("`##twice-defined` is written at the middle of a line");
        assert!(inline.starts_with("notes/structure.md:41"), "{inline}");
    }

    #[test]
    fn an_empty_head_in_front_of_a_path_shape_is_malformed_rather_than_silent() {
        let f = one("`@notes/p.md` is malformed: the kind segment is empty");
        assert!(f.starts_with("notes/structure.md:43"), "{f}");
    }

    #[test]
    fn a_path_that_does_not_resolve_is_reported() {
        // Named rather than matched on "does not exist": a declared tracker that is not there
        // says the same words, and a needle matching both would pass while checking neither.
        assert!(one("`path@planted@notes/missing.md` does not exist")
            .starts_with("notes/structure.md:25"));
    }

    #[test]
    fn each_shape_of_the_path_kind_is_enforced_over_a_real_walk() {
        // One planted defect per path check, end to end; the unit tests carry the shapes,
        // and this asserts the walk delivers each to its check.
        assert!(one("names no anchor").contains("notes/missing.md"));
        assert!(one("claims a file and names a directory").contains("path@planted@notes"));
        assert!(one("resolves in this tree").contains("path@elsewhere@notes/p.md"));
        assert!(one("reaches inside the anchor").contains("widget"));
        assert!(one("resolves in no component").contains("path@*@notes/void.md"));
        assert!(one("not a navigation home").contains("p.md"));
    }

    #[test]
    fn a_reference_to_another_register_is_resolved_in_that_register() {
        let f = one("`tripwire@planted@nothing` is referenced");
        assert!(f.starts_with("notes/structure.md:37"), "{f}");
        assert!(f.contains("defines no tripwire `nothing`"), "{f}");
    }

    #[test]
    fn a_heading_register_home_that_is_missing_is_reported_at_the_path_it_belongs_at() {
        let f = one("carries no tripwire home");
        assert!(f.starts_with("parts/widget/docs/tripwires.md"), "{f}");
        assert!(f.contains("`widget`"), "{f}");
    }

    #[test]
    fn a_location_whose_declared_register_has_no_home_is_reported() {
        // A location carries the registers it declares and nothing else, so a missing home is
        // a finding against the location. Undeclared, the directory would be read by nothing.
        let f = one("the anchor `agent-config` carries no issue directory");
        assert!(f.starts_with("agent-config/open-issues"), "{f}");
    }

    #[test]
    fn every_shape_a_file_register_asserts_has_a_planted_defect() {
        // One per assertion, over a real walk: the retired file shape beside the directory,
        // the missing index, a kind the closed list does not hold, a missing owed subsection,
        // an id no reference can spell, a subdirectory that is no declared group, and a file
        // of another suffix inside the instance.
        assert!(one("retired file shape").starts_with("docs/open-issues.md"));
        assert!(one("has no index.md").starts_with("docs/open-issues/index.md"));
        assert!(one("no accepted value").starts_with("docs/open-issues/a-wrong-kind.md"));
        assert!(one("no level-three subsection `What would close it`")
            .starts_with("docs/open-issues/a-missing-subsection.md"));
        assert!(one("cannot be an entry id").starts_with("docs/open-issues/Not_An_Id.md"));
        assert!(one("is no declared group").contains("an-undeclared-group"));
        assert!(one("is not an entry of the issue register").contains("nonsense.txt"));
        assert!(one("does not parse: line 3 declares `kind` a second time")
            .starts_with("docs/open-issues/two-kinds.md"));
    }

    #[test]
    fn a_well_formed_entry_produces_no_finding() {
        // The negative half. Without it the entry checks could fire on everything and every
        // planted assertion above would still pass.
        let all = findings();
        assert!(
            !all.iter().any(|f| f.contains("a-well-formed-one.md")),
            "{all:#?}"
        );
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
        // stated against, which is what gives `generated` its three findings.
        let planted: usize = PLANTED
            .iter()
            .map(|(_, p, _)| match p {
                Planted::InRun(n) => *n,
                Planted::ByTheBinary(_) => 0,
            })
            .sum();
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
        judged_with(None)
    }

    /// The same, with the model told where the checker's own source is.
    fn judged_with(checker_source: Option<&std::path::Path>) -> Vec<(Rule, String)> {
        let manifest = mock("planted");
        let model = Model::build(&manifest, checker_source).expect("a model");
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
            !found.iter().any(|f| f.contains("`quoted_in_scope`")),
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
                .any(|f| f.contains("A rule number that is data") || f.contains("`FIXTURE`")),
            "{found:#?}"
        );
    }

    #[test]
    fn under_the_checkers_own_source_a_literal_is_data_and_its_comments_stay_prose() {
        // The same planted file, read as the checker's own: the call message's claim is
        // gone, and the claims the comments carry are judged exactly as before.
        let manifest = mock("planted");
        let code = manifest.root().join("code");
        let model = Model::build(&manifest, Some(&code)).expect("a model");
        let doc = model
            .documents()
            .iter()
            .find(|d| d.rel == std::path::Path::new("code/regime.rs"))
            .expect("the planted source file");
        assert_eq!(doc.literals, documentation::source::Literals::Data);
        let under_code = model
            .documents()
            .iter()
            .filter(|d| d.rel.starts_with("code") && d.rel.extension().is_some_and(|e| e == "rs"))
            .count();
        assert!(under_code >= 1);
        assert_eq!(model.checker_files(), under_code);
        assert_eq!(
            model.checker_source(),
            Some(std::path::Path::new("code")),
            "named relative to the root when it sits under it"
        );
        let with = judged_with(Some(&code));
        assert!(
            !with.iter().any(|(_, w)| w.contains("`message_in_a_call`")),
            "{with:#?}"
        );
        assert!(
            with.iter().any(|(r, _)| *r == Rule::IdentifierFullQuote),
            "a comment-carried claim is still judged: {with:#?}"
        );
        // A checker directory that does not exist exempts nothing, and is still named: the
        // summary line is how a binary compiled from a directory that is gone says so.
        let elsewhere = manifest.root().join("no-such-directory");
        let model = Model::build(&manifest, Some(&elsewhere)).expect("a model");
        assert_eq!(model.checker_files(), 0);
        assert!(
            model
                .checker_source()
                .is_some_and(|p| p.ends_with("no-such-directory")),
            "{:?}",
            model.checker_source()
        );
        // A checker directory the whole tree sits INSIDE exempts nothing either: that is a
        // mock project under the checker's own tests, a foreign tree.
        let above = manifest.root().join("../../..");
        let model = Model::build(&manifest, Some(&above)).expect("a model");
        assert_eq!(model.checker_files(), 0);
        assert!(
            model.checker_source().is_some_and(|p| p.is_absolute()),
            "named absolutely when it is not under the root"
        );
    }

    /// The root is canonicalised before the prefix test, so a checkout reached through a
    /// symlink still finds the checker's directory inside it. Without the canonicalisation
    /// the joined path starts with the link and the real directory starts with its target,
    /// and nothing is exempt.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_root_still_finds_the_checker_directory_inside_it() {
        let real = mock("planted").root().to_path_buf();
        let link =
            std::env::temp_dir().join(format!("knowledge-symlinked-root-{}", std::process::id()));
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&real, &link).expect("a symlink to the mock project");
        let manifest = Manifest::load(&link).expect("the manifest through the symlink");
        let model = Model::build(&manifest, Some(&real.join("code"))).expect("a model");
        let _ = std::fs::remove_file(&link);
        assert!(model.checker_files() >= 1, "{:?}", model.checker_source());
    }

    #[test]
    fn a_message_in_a_call_is_prose_and_claims_its_rule() {
        // The planted `message_in_a_call` carries a marker in a call argument and no quote in
        // its scope. Outside the checker's own source that is a claim like any other.
        one(Rule::QuoteInScope, "claimed in `message_in_a_call`");
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
