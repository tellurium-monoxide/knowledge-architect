//! The checks. Each is a pure function from the model to findings.
//!
//! No check reads a file, spawns a process, or knows how the walk works. That is what makes
//! the single walk a property of the design rather than of anyone's care, and it is what lets
//! a check be run against a model assembled in memory.

pub(crate) mod agents;
pub(crate) mod generated;
pub(crate) mod references;
pub(crate) mod registers;
pub(crate) mod tree;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;

use crate::extension::{Generated, Prepared};

/// What the structural checks looked at, so that finding nothing is distinguishable from
/// looking at nothing.
#[derive(Debug, Default)]
pub struct Structure {
    pub components: usize,
    pub locations: usize,
    /// Anchor-and-register pairs whose home was asserted, and the file-register entries
    /// whose shape was.
    pub instances: usize,
    pub entries: usize,
    /// Distinct entities the table holds, reference occurrences judged against it, and the
    /// relative markdown links resolved.
    pub entities: usize,
    pub references: usize,
    pub links: usize,
    /// How many files the walk read. Not a family's count: it describes the walk every family
    /// read, so it is set whatever was asked for. It is printed because git is the walk, per
    /// `design@core@git-supplies-the-walk`, and two machines answering differently has to be
    /// visible in the output rather than inferred from a finding list.
    pub walked: usize,
    /// The checker's own directory as the model was told it, and how many walked Rust files
    /// sit under it with their string literals read as data. Not a family's count: it describes
    /// the walk every family read, so it is set whatever was asked for.
    pub checker_sources: Vec<std::path::PathBuf>,
    pub checker_files: usize,
}

/// Where in the pipeline a finding is produced, and therefore what it undermines.
///
/// Each of the first three phases builds one input of the next — the manifest resolves into
/// a configuration, the walk and the parse into a model, the model into the entity table —
/// and every check of the last phase consumes the table and produces nothing another check
/// reads. So a finding raised in one phase makes every finding of a later phase unreliable
/// in both directions, missing and false, and a finding raised in the last undermines
/// nothing. A finding is classified by the place it is produced, never by a label at its
/// site: one added to a building step is gated because of where it is raised.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    /// The manifest's declarations, resolved: every complaint `Manifest::parse` holds.
    Resolution = 1,
    /// The walk and the parse against the tree: `check::tree`.
    Tree = 2,
    /// The entity table's own findings: a definition where none may sit, an id nothing can
    /// spell, one id defined twice.
    Definitions = 3,
    /// Everything computed over a complete model.
    Content = 4,
}

impl Phase {
    pub fn number(self) -> u8 {
        self as u8
    }

    /// The summary line a stop prints: what was reported, and what was not judged and why.
    pub fn stop_line(self, findings: usize) -> String {
        let later: Vec<String> = [Phase::Tree, Phase::Definitions, Phase::Content]
            .into_iter()
            .filter(|p| *p > self)
            .map(|p| p.number().to_string())
            .collect();
        let later = match later.as_slice() {
            [one] => format!("phase {one} was"),
            [head @ .., last] => format!("phases {} and {last} were", head.join(", ")),
            [] => String::new(),
        };
        format!(
            "phase {}: {findings} finding(s); {later} not judged, because their findings would \
             be computed over a model these leave incomplete",
            self.number()
        )
    }
}

/// A run that ended before the last phase: the phase that produced something, and what.
#[derive(Debug)]
pub struct Stop {
    pub phase: Phase,
    pub findings: Vec<Finding>,
}

/// Phases 1 to 3, in order. The first that produces anything ends the run.
///
/// A check is a pure function over the model, so the releases the last phase needs are not
/// needed here and a caller resolves them only once this returns `Ok`: a run that stops
/// earlier fetches nothing.
pub fn foundation(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Result<(), Stop> {
    let complaints = manifest.complaints();
    if !complaints.is_empty() {
        return Err(Stop {
            phase: Phase::Resolution,
            findings: complaints.to_vec(),
        });
    }
    let mut findings = tree::check(model, manifest, inputs);
    findings.extend(agents::check(model, manifest, inputs));
    if !findings.is_empty() {
        return Err(Stop {
            phase: Phase::Tree,
            findings,
        });
    }
    let anchors = crate::entity::Anchors::of(manifest);
    let findings = crate::entity::Entities::build(model, &anchors)
        .definition_findings()
        .to_vec();
    if !findings.is_empty() {
        return Err(Stop {
            phase: Phase::Definitions,
            findings,
        });
    }
    Ok(())
}

/// What a whole run found and counted.
pub struct Report {
    /// The phase the findings belong to: `Content` for a run that judged everything, and
    /// the phase that stopped it otherwise, when nothing later was judged.
    pub phase: Phase,
    pub findings: Vec<Finding>,
    pub structure: Structure,
    /// Every check the last phase performed or was asked to: the core's, then each
    /// extension's, in the order the `checked:` line prints them.
    pub checks: Vec<&'static str>,
    /// The checks the tree gave no input to, by name. Each is printed as not run rather
    /// than counted, because a count nobody took would read as "nothing found" for a check
    /// that never happened.
    pub not_run: Vec<&'static str>,
    /// Each extension's block of the summary, in the order the extensions were registered.
    pub summaries: Vec<String>,
}

impl Report {
    pub fn failed(&self) -> bool {
        !self.findings.is_empty()
    }

    /// The report of a run that stopped: the stop's findings and nothing later, with what
    /// the walk read still counted, since every phase read it.
    pub(crate) fn stopped(stop: Stop, model: &Model) -> Self {
        let structure = Structure {
            walked: model.documents().len(),
            checker_sources: model.checker_sources().to_vec(),
            checker_files: model.checker_files(),
            ..Structure::default()
        };
        Report {
            phase: stop.phase,
            findings: stop.findings,
            structure,
            checks: Vec::new(),
            not_run: Vec::new(),
            summaries: Vec::new(),
        }
    }
}

/// Everything a check needs that it may not fetch for itself.
///
/// A check is a pure function from the model to findings, so anything requiring the
/// filesystem or the network is resolved by the caller and handed in. Resolving a release may
/// read the archive; reading a generated file reaches outside the walk, which excludes those
/// files by name.
///
/// Non-exhaustive: a caller outside this crate gets one from [`crate::cli::Gathered`], so a new
/// input is not a breaking change for it.
// Non-exhaustive per `design@core@ne-minimal`.
#[non_exhaustive]
pub struct Inputs<'a> {
    /// The generated files as committed, keyed by their project-relative path.
    pub committed: &'a HashMap<PathBuf, String>,
    /// Every `register.toml` beside a register instance, keyed by its project-relative path.
    ///
    /// Read by the caller like the generated files, and for the same reason: a check may not
    /// touch the filesystem, and this one is outside the walk because it is not markdown.
    pub configs: &'a HashMap<PathBuf, String>,
    /// Every path that exists in the project, files and directories, project-relative. One
    /// listing by the caller answers every question a check has about what is there.
    pub present: &'a HashSet<PathBuf>,
    /// The subset of `present` that is directories, so a check can assert a path's kind —
    /// a required document is a file, and a reference's trailing slash claims a directory.
    pub directories: &'a HashSet<PathBuf>,
    /// Files the walk does not cover, with their text — excluding the paths the manifest
    /// excludes, which are other projects rather than unchecked files of this one. A state
    /// other than text or binary is a finding.
    pub outside: &'a [(PathBuf, crate::survey::Outside)],
    /// The path spellings the ignore rules cover, out of every spelling a path reference in
    /// this run could ask about.
    ///
    /// One `git check-ignore` batch by the caller, keyed by `git::ignore_query`'s spelling, so
    /// a check answers the question with a lookup and spawns nothing. Asking the RULES rather
    /// than what is on disk is what makes the verdict identical on a fresh clone and a built
    /// tree, per `design@core@ignored-targets-are-not-asserted`.
    pub ignored: &'a HashSet<String>,
    /// The files git both tracks and ignores, each of which is a finding.
    pub tracked_and_ignored: &'a [PathBuf],
    /// The files the walk refuses by name, per `walk::refused`, each of which is a finding.
    pub refused: &'a [PathBuf],
    /// The listing's symlink and gitlink entries no walk row keeps, each of which is a finding.
    pub links: &'a [crate::git::Entry],
    /// The files of the installer's namespace, with what reading each gave, per
    /// `design@core@owned-namespace-check`.
    pub installed: &'a [(PathBuf, crate::survey::Outside)],
    /// What the installed set must be: every shipped file at its install path, rendered with
    /// the project's command. Empty when the project serves no agent harness.
    pub shipped: &'a [(PathBuf, String)],
}

/// The core's checks of the last phase, by the name each one's count line carries.
///
/// One per module under `check/` that runs in the last phase. Every one runs on every run that
/// reaches the last phase. An extension adds its own, per
/// `design@core@an-extension-plugs-in-through-phased-hooks`.
pub const CHECKS: [&str; 3] = ["generated", "registers", "references"];

/// Run the core's checks of the last phase, over a model `foundation` found complete, with
/// the files each extension generates compared by the `generated` check.
pub(crate) fn run(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    extension_files: &[Generated],
) -> Report {
    let mut findings = Vec::new();
    let mut structure = Structure::default();
    findings.extend(generated::check(model, manifest, inputs, extension_files));
    {
        let (found, counts) = registers::check(model, manifest, inputs);
        findings.extend(found);
        structure.components = counts.components;
        structure.locations = counts.locations;
        structure.instances = counts.instances;
        structure.entries = counts.entries;
    }
    {
        let (found, c) = references::check(model, manifest, inputs);
        findings.extend(found);
        structure.entities = c.entities;
        structure.references = c.references;
        structure.links = c.links;
    }
    structure.walked = model.documents().len();
    structure.checker_sources = model.checker_sources().to_vec();
    structure.checker_files = model.checker_files();

    Report {
        phase: Phase::Content,
        findings,
        structure,
        checks: CHECKS.to_vec(),
        not_run: Vec::new(),
        summaries: Vec::new(),
    }
}

/// The last phase over the core and every prepared extension: the core's checks, then each
/// extension's, merged into one report.
pub fn run_with(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    extensions: &[(&[&'static str], &dyn Prepared)],
) -> Report {
    let files: Vec<Generated> = extensions
        .iter()
        .flat_map(|(_, p)| p.generated(model, manifest))
        .collect();
    let mut report = run(model, manifest, inputs, &files);
    for (checks, prepared) in extensions {
        let theirs = prepared.check(model, manifest, inputs);
        report.checks.extend(checks.iter().copied());
        report.findings.extend(theirs.findings);
        report.not_run.extend(theirs.not_run);
        report.summaries.push(theirs.summary);
    }
    report
}

#[cfg(test)]
mod phase_tests {
    use super::*;
    use std::collections::HashSet;

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"p\"\ncomponents = []\n\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n\n\
             "
        );
        Manifest::parse(std::path::Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The tree every component owes, in the file-shaped heading homes.
    /// Directories are listed too: what exists is asked of `present`, and a checkout's
    /// listing holds a directory wherever a file sits under it.
    fn complete_tree() -> HashSet<PathBuf> {
        [
            "docs",
            "docs/open-issues",
            "README.md",
            "CLAUDE.md",
            "docs/rejected-alternatives.md",
            "docs/design.md",
            "docs/goals.md",
            "docs/tripwires.md",
            "docs/open-issues/README.md",
            "docs/open-issues/index.md",
        ]
        .iter()
        .map(PathBuf::from)
        .collect()
    }

    fn foundation_over(
        manifest: &Manifest,
        present: HashSet<PathBuf>,
        model: &Model,
    ) -> Result<(), Stop> {
        let directories = testing::implied_directories(&present);
        let inputs = Inputs {
            committed: &HashMap::new(),
            configs: &HashMap::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
        };
        foundation(model, manifest, &inputs)
    }

    #[test]
    fn a_complaint_stops_the_run_at_phase_one_before_the_tree_is_read() {
        // Both a refused row and a missing home: the complaint is what prints, alone, and
        // phase 2's missing home is not judged.
        let m = manifest("[locations.up]\npath = \"docs/../docs\"\nregisters = [\"issue\"]\n");
        let mut present = complete_tree();
        present.remove(&PathBuf::from("docs/goals.md"));
        let stop = foundation_over(&m, present, &Model::from_documents(Vec::new()))
            .expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Resolution);
        assert_eq!(stop.findings.len(), 1, "{:#?}", stop.findings);
        assert!(stop.findings[0].what.contains("spells a `..` segment"));
    }

    #[test]
    fn every_complaint_prints_at_phase_one() {
        let m = manifest(
            "[locations.up]\npath = \"docs/../docs\"\nregisters = [\"issue\"]\n\n\
             [locations.abs]\npath = \"/abs\"\nregisters = [\"issue\"]\n",
        );
        let stop = foundation_over(&m, complete_tree(), &Model::from_documents(Vec::new()))
            .expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Resolution);
        assert_eq!(stop.findings.len(), 2, "{:#?}", stop.findings);
    }

    #[test]
    fn a_location_holding_the_root_s_homes_is_the_one_finding_and_owns_no_slug() {
        // The shape the cascade entry was about: a location at the root's `docs/`, refused for
        // holding the four homes, and a slug in the design home that it would have owned. The
        // refusal is the one finding; nothing reports the slug misplaced against the location.
        let m = manifest("[locations.papers]\npath = \"docs\"\nregisters = [\"issue\"]\n");
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n### A decision `##one`\n".to_string(),
        )]);
        let stop = foundation_over(&m, complete_tree(), &model).expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Resolution, "{:#?}", stop.findings);
        assert_eq!(stop.findings.len(), 1, "{:#?}", stop.findings);
        assert!(
            stop.findings[0]
                .what
                .contains("holds the design, goal, tripwire and issue homes"),
            "{:#?}",
            stop.findings
        );
    }

    #[test]
    fn a_definition_defect_stops_the_run_at_phase_three_and_a_complete_tree_passes() {
        let m = manifest("");
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n#### Too deep `##too-deep`\n".to_string(),
        )]);
        let stop = foundation_over(&m, complete_tree(), &model).expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Definitions, "{:#?}", stop.findings);
        assert!(
            stop.findings[0].what.contains("`##too-deep`"),
            "{:#?}",
            stop.findings
        );
        let clean = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n### A decision `##one`\n".to_string(),
        )]);
        assert!(foundation_over(&m, complete_tree(), &clean).is_ok());
    }

    #[test]
    fn the_stop_line_names_the_phase_and_what_was_not_judged() {
        assert_eq!(
            Phase::Resolution.stop_line(2),
            "phase 1: 2 finding(s); phases 2, 3 and 4 were not judged, because their findings \
             would be computed over a model these leave incomplete"
        );
        assert!(Phase::Tree
            .stop_line(1)
            .starts_with("phase 2: 1 finding(s); phases 3 and 4 were"));
        assert!(Phase::Definitions
            .stop_line(1)
            .starts_with("phase 3: 1 finding(s); phase 4 was"));
    }
}

#[cfg(test)]
pub(crate) mod testing {
    use std::collections::HashSet;
    use std::path::PathBuf;

    /// The directories a flat listing implies: every path with an entry beneath it.
    ///
    /// A test states its tree as a list of paths, the way a checkout lists one, and the kinds
    /// follow from the shape. A test that needs a childless directory inserts it directly.
    pub(crate) fn implied_directories(present: &HashSet<PathBuf>) -> HashSet<PathBuf> {
        present
            .iter()
            .filter(|p| present.iter().any(|q| *q != **p && q.starts_with(p)))
            .cloned()
            .collect()
    }
}
