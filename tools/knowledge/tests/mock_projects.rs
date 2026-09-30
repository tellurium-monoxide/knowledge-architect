//! The core run against whole projects, not against strings.
//!
//! A mock project under `path@knowledge@tests/projects/` is a complete project: it carries its
//! own `knowledge.toml` and its own documents, and it declares no table the core does not own.
//! That is what makes these tests worth more than the in-memory ones — a fixture written as a
//! string cannot exercise the walk, the exclusions, or a layout different from this repository's.
//!
//! The directory stays excluded by this repository's own manifest: each mock project is a
//! complete foreign project, and its planted defects must be reported by the test that runs the
//! tool over it, never as this repository's own.

use std::path::PathBuf;

use documentation::{Manifest, Model, Observation, RetiredForm};

fn mock(name: &str) -> Manifest {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name);
    Manifest::load(&root).expect("the mock project's manifest")
}

fn model(name: &str) -> Model {
    Model::build(&mock(name), &[]).expect("a model of the mock project")
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
fn the_survey_records_which_paths_are_directories() {
    // The kind is a fact of the listing rather than an inference from entries beneath a
    // path, which could not tell an empty directory from a file.
    let manifest = mock("minimal");
    let model = Model::build(&manifest, &[]).expect("a model");
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
        _ => None,
    });
    assert_eq!(retired, Vec::<String>::new());
}

/// A project that carries what it declares reports nothing, over every core check.
#[test]
fn the_conformant_mocks_report_nothing_over_every_core_check() {
    use documentation::check::{foundation, run, Inputs};
    use std::collections::HashMap;
    for name in ["minimal", "dirhome", "core"] {
        let manifest = mock(name);
        let model = model(name);
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        // The committed generated files, read from the tree rather than regenerated, so a
        // committed index that has gone stale fails here.
        let mut committed = HashMap::new();
        for rel in documentation::index::generated_paths(&manifest) {
            if let Ok(text) = std::fs::read_to_string(manifest.root().join(&rel)) {
                committed.insert(rel, text);
            }
        }
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            committed: &committed,
            configs: &configs(&manifest),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
        };
        assert!(
            foundation(&model, &manifest, &inputs).is_ok(),
            "{name}: phases 1 to 3"
        );
        let found: Vec<String> = run(&model, &manifest, &inputs, &[])
            .findings
            .iter()
            .map(|f| f.to_string())
            .collect();
        assert!(found.is_empty(), "{name}: {found:#?}");
    }
}

/// Every committed file-register index is what the generator writes, in every core mock.
#[test]
fn every_committed_index_is_what_the_generator_writes() {
    for name in ["minimal", "dirhome", "core"] {
        let manifest = mock(name);
        let model = model(name);
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        for (rel, want) in
            documentation::index::file_register_indexes(&model, &manifest, &survey.directories)
        {
            let have = std::fs::read_to_string(manifest.root().join(&rel)).expect("committed");
            assert_eq!(have, want, "{name}: {}", rel.display());
        }
    }
}

/// The core's rows carry the line of the file they sit on.
///
/// The whole value is asserted, and the trailing newline is load-bearing: a `contains` over a
/// field that is not terminated asserts a prefix, and passes against a value with more after it.
#[test]
fn observations_come_out_of_a_real_walk_with_real_line_numbers() {
    let dump = model("minimal").canonical();
    assert!(
        dump.contains("docs/design.md\t5\tslug-def\tmock-anchor heading-3\n"),
        "{dump}"
    );
    assert!(
        dump.contains("docs/design.md\t7\tspan\tpath@notes@b.md\n"),
        "{dump}"
    );
    assert!(
        dump.contains("notes/a.md\t5\tspan\tgoal@minimal@mock-goal\n"),
        "{dump}"
    );
    assert!(
        dump.contains("code/lib.rs\t1\tspan\tdesign@minimal@mock-anchor\n"),
        "{dump}"
    );
}

mod planted {
    use super::*;
    use documentation::check::{run, Inputs, CHECKS};
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// The file-register indexes this project would have if they were current.
    fn current_indexes(manifest: &Manifest, model: &Model) -> HashMap<PathBuf, String> {
        let survey = documentation::survey::survey(manifest, model).expect("a survey of the mock");
        documentation::index::file_register_indexes(model, manifest, &survey.directories)
            .into_iter()
            .collect()
    }

    /// The findings of the core's last phase, with the committed files as stated.
    fn findings_with(
        committed: impl Fn(&Manifest, &Model) -> HashMap<PathBuf, String>,
    ) -> Vec<String> {
        let manifest = mock("planted");
        let model = Model::build(&manifest, &[]).expect("a model");
        let committed = committed(&manifest, &model);
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            committed: &committed,
            configs: &configs(&manifest),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
        };
        run(&model, &manifest, &inputs, &[])
            .findings
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    fn findings() -> Vec<String> {
        findings_with(current_indexes)
    }

    /// Every planted defect this project carries, by the core check that reports it: the check,
    /// how many findings it produces, and a fragment of one. `every_check_has_a_row` reads
    /// `CHECKS` and demands a row for each, so a check with no planted defect fails there.
    ///
    /// The definition-site findings are not here: whether a home EXISTS is phase 2, and a slug
    /// or an entry id where none may sit is phase 3, and both are `unsound`'s.
    const PLANTED: [(&str, usize, &str); 3] = [
        // One reference of each shape the resolver tells apart — dangling, unknown anchor,
        // two and four segments, an anchor and a reserved anchor in kind position, the two
        // retired slug shapes — and the path shapes: a dangling one, the unanchored bare form,
        // a wrong kind claim, an escape that resolves here, a root pointer reaching inside the
        // component, a generic pointer nothing carries, a dangling tripwire reference, a link
        // outside a navigation home, and the retired `@` escape with its empty head.
        ("references", 17, "is referenced"),
        // One defect per shape assertion over what is there: a missing index, an undeclared
        // kind, a missing owed subsection, an undeclared group, a file of another suffix, and
        // frontmatter that does not parse.
        ("registers", 6, "issue register"),
        // Three file-register indexes, and the run is handed an empty committed set, so each
        // is reported missing.
        ("generated", 3, "the generated file is missing"),
    ];

    #[test]
    fn every_check_has_a_row_and_the_run_is_their_sum() {
        // Read off `CHECKS` rather than off `PLANTED`, so a check added to the library with no
        // planted defect fails here instead of being absent from both. And nothing is invented
        // by running them together: the whole run is the sum of the rows.
        for name in CHECKS {
            assert!(
                PLANTED.iter().any(|(n, _, _)| *n == name),
                "{name} has no row: every check owes a planted defect"
            );
        }
        let whole = findings_with(|_, _| HashMap::new());
        let planted: usize = PLANTED.iter().map(|(_, n, _)| n).sum();
        assert_eq!(whole.len(), planted, "{whole:#?}");
        for (name, n, fragment) in PLANTED {
            let hits = whole.iter().filter(|f| f.contains(fragment)).count();
            assert!(
                (1..=n).contains(&hits),
                "{name}: {hits} of {fragment:?}, expected 1 to {n}, in {whole:#?}"
            );
        }
    }

    #[test]
    fn a_generated_file_that_is_absent_is_reported_as_missing() {
        let gone = findings_with(|_, _| HashMap::new());
        assert_eq!(
            gone.iter().filter(|f| f.contains("is missing")).count(),
            3,
            "the three file-register indexes: {gone:#?}"
        );
    }

    fn one(needle: &str) -> String {
        let all = findings();
        let hits: Vec<&String> = all.iter().filter(|f| f.contains(needle)).collect();
        assert_eq!(hits.len(), 1, "expected exactly one {needle:?} in {all:#?}");
        hits[0].clone()
    }

    /// A file-register index is judged the same way, and by its own bytes.
    ///
    /// The hand edit is in the rows rather than in the banner, because the banner is the one
    /// line a generator that produced nothing at all would still get right.
    #[test]
    fn a_hand_edited_file_register_index_is_reported_at_the_line_that_was_edited() {
        let path = PathBuf::from("parts/widget/docs/open-issues/index.md");
        let edited = findings_with(|m, model| {
            let mut c = current_indexes(m, model);
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
    fn the_retired_slug_form_is_reported_as_what_it_was_and_a_bare_entry_number_is_not() {
        let qualified = one("`planted#twice-defined` is the retired slug reference form");
        assert!(
            qualified.starts_with("notes/structure.md:17"),
            "{qualified}"
        );
        let bare = one("`#unqualified-anchor` is the retired slug reference form");
        assert!(bare.starts_with("notes/structure.md:19"), "{bare}");
        let all = findings();
        assert!(
            !all.iter().any(|f| f.starts_with("notes/structure.md:21")),
            "{all:#?}"
        );
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
    fn every_shape_a_file_register_asserts_has_a_planted_defect() {
        // One per shape assertion, over a real walk: the missing index, a kind the closed
        // list does not hold, a missing owed subsection, a subdirectory that is no declared
        // group, and a file of another suffix inside the instance. The retired file shape
        // and an id no reference can spell are `unsound`'s.
        assert!(one("has no index.md").starts_with("docs/open-issues/index.md"));
        assert!(one("no accepted value").starts_with("docs/open-issues/a-wrong-kind.md"));
        assert!(one("no level-three subsection `What would close it`")
            .starts_with("docs/open-issues/a-missing-subsection.md"));
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
}

mod unsound {
    use super::*;
    use documentation::check::{foundation, tree, Inputs, Phase};
    use documentation::entity::{Anchors, Entities};
    use std::collections::HashMap;

    /// The findings of `tree::check`, phase 2, over the project.
    fn phase_two() -> Vec<String> {
        let manifest = mock("unsound");
        let model = Model::build(&manifest, &[]).expect("a model");
        let survey =
            documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            committed: &HashMap::new(),
            configs: &configs(&manifest),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
        };
        assert!(
            manifest.complaints().is_empty(),
            "{:#?}",
            manifest.complaints()
        );
        let stopped = foundation(&model, &manifest, &inputs).expect_err("the run stops");
        assert_eq!(stopped.phase, Phase::Tree);
        let found: Vec<String> = tree::check(&model, &manifest, &inputs)
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect();
        let printed: Vec<String> = stopped
            .findings
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect();
        assert_eq!(
            printed, found,
            "the stop carries exactly phase 2's findings"
        );
        found
    }

    /// The findings of the entity table, phase 3, over the project.
    fn phase_three() -> Vec<String> {
        let manifest = mock("unsound");
        let model = Model::build(&manifest, &[]).expect("a model");
        let anchors = Anchors::of(&manifest);
        Entities::build(&model, &anchors)
            .definition_findings()
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    fn one_of(all: &[String], needle: &str) -> String {
        let hits: Vec<&String> = all.iter().filter(|f| f.contains(needle)).collect();
        assert_eq!(hits.len(), 1, "expected exactly one {needle:?} in {all:#?}");
        hits[0].clone()
    }

    #[test]
    fn every_phase_two_assertion_has_a_planted_defect_and_nothing_else_is_reported() {
        let found = phase_two();
        assert!(one_of(&found, "could not be read as text").starts_with("notes/latin1.md:1"));
        assert!(one_of(
            &found,
            "is declared in [walk] skip-files and does not exist"
        )
        .contains("notes/gone.md"));
        assert!(one_of(&found, "retired file shape").starts_with("docs/open-issues.md"));
        let home = one_of(&found, "carries no tripwire home");
        assert!(home.starts_with("parts/widget/docs/tripwires.md"), "{home}");
        assert!(home.contains("`widget`"), "{home}");
        // A location carries the registers it declares and nothing else, so a missing home
        // is a finding against the location.
        assert!(one_of(
            &found,
            "the anchor `agent-config` carries no issue directory"
        )
        .starts_with("agent-config/open-issues"));
        assert_eq!(found.len(), 5, "{found:#?}");
    }

    #[test]
    fn a_slug_defined_twice_in_one_instance_is_reported_at_both_sites() {
        // A rename that left one behind. The reader who opens either copy is told about the
        // other.
        let all = phase_three();
        let dup: Vec<&String> = all
            .iter()
            .filter(|f| f.contains("`design@unsound@twice-defined` is also defined at"))
            .collect();
        assert_eq!(dup.len(), 2, "{all:#?}");
        assert!(
            dup.iter().any(|f| f.starts_with("docs/design/planted.md:3")
                && f.contains("docs/design/planted.md:7")),
            "{dup:#?}"
        );
        assert!(
            dup.iter().any(|f| f.starts_with("docs/design/planted.md:7")
                && f.contains("docs/design/planted.md:3")),
            "{dup:#?}"
        );
    }

    #[test]
    fn a_misplaced_definition_is_reported_where_it_stands_and_defines_nothing() {
        // Four shapes over a real walk: a level-four heading, a line head and a table cell
        // inside the design home, and a level-three heading in a file that is no register
        // home.
        let all = phase_three();
        let deep = one_of(&all, "`##too-deep` is written at a level-4 heading");
        assert!(deep.starts_with("docs/design/planted.md:9"), "{deep}");
        let head = one_of(&all, "`##line-head` is written at the head of a plain line");
        assert!(head.starts_with("docs/design/planted.md:11"), "{head}");
        let cell = one_of(&all, "`##in-a-cell` is written in a table cell");
        assert!(cell.starts_with("docs/design/planted.md:17"), "{cell}");
        let stray = one_of(&all, "`##stray-anchor` is written in `notes/structure.md`");
        assert!(stray.starts_with("notes/structure.md:3"), "{stray}");
        assert!(
            stray.contains("no heading register home of `unsound`"),
            "{stray}"
        );
        let inline = one_of(&all, "`##twice-defined` is written in the middle of a line");
        assert!(inline.starts_with("notes/structure.md:5"), "{inline}");
        assert!(one_of(&all, "cannot be an entry id").starts_with("docs/open-issues/Not_An_Id.md"));
        assert_eq!(all.len(), 9, "{all:#?}");
    }

    #[test]
    fn a_heading_at_the_register_level_with_no_slug_is_reported_and_defines_nothing() {
        // A tripwire or a decision written without its slug reads as an entry, and nothing
        // lists it. The design register's level is three; the level-one title and the
        // level-four heading beside it owe nothing.
        let all = phase_three();
        let lost = one_of(&all, "carries no slug");
        assert!(lost.starts_with("docs/design/planted.md:13"), "{lost}");
        assert!(lost.contains("level-3 heading"), "{lost}");
        assert!(lost.contains("design home"), "{lost}");
    }

    /// The negative half of the gate: the projects that reach phase 4 have nothing in the
    /// first three, so a stop is never mistaken for a pass and a pass never hides a stop.
    #[test]
    fn planted_and_dirhome_are_complete_through_phase_three() {
        for name in ["planted", "dirhome"] {
            let manifest = mock(name);
            let model = Model::build(&manifest, &[]).expect("a model");
            let survey =
                documentation::survey::survey(&manifest, &model).expect("a survey of the mock");
            let git = git_answers(&manifest, &model);
            let inputs = Inputs {
                committed: &HashMap::new(),
                configs: &configs(&manifest),
                present: &survey.present,
                directories: &survey.directories,
                outside: &survey.outside,
                ignored: &git.0,
                tracked_and_ignored: &git.1,
                refused: &survey.refused,
                links: &survey.links,
            };
            let outcome = foundation(&model, &manifest, &inputs);
            assert!(outcome.is_ok(), "{name}: {:#?}", outcome.err());
        }
    }
}

mod checker {
    use super::*;

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
        let model = Model::build(&manifest, &[real.join("code").as_path()]).expect("a model");
        let _ = std::fs::remove_file(&link);
        assert!(model.checker_files() >= 1, "{:?}", model.checker_sources());
    }
}
