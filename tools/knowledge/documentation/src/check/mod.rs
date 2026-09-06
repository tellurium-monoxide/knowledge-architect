//! The checks. Each is a pure function from the model to findings.
//!
//! No check reads a file, spawns a process, or knows how the walk works. That is what makes
//! the single walk a property of the design rather than of anyone's care, and it is what lets
//! a check be run against a model assembled in memory.

pub mod changes;
pub mod citations;
pub mod generated;
pub mod references;
pub mod regime;
pub mod registers;
pub mod tree;
pub mod uncovered;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;

use citations::{Counts, Release};

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
    pub uncovered_files: usize,
    /// How many files the walk read. Not a family's count: it describes the walk every family
    /// read, so it is set whatever was asked for. It is printed because git is the walk, per
    /// `design@knowledge@git-supplies-the-walk`, and two machines answering differently has to be
    /// visible in the output rather than inferred from a finding list.
    pub walked: usize,
    /// The checker's own directory as the model was told it, and how many walked Rust files
    /// sit under it with their string literals read as data. Not a family's count: it describes
    /// the walk every family read, so it is set whatever was asked for.
    pub checker_source: Option<std::path::PathBuf>,
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
    let findings = tree::check(model, manifest, inputs);
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
    pub counts: Counts,
    pub structure: Structure,
    /// What the corpus checks looked at. They are the one family that reads the filesystem
    /// itself, because their subject IS filesystem state.
    pub corpus: rules::integrity::Counts,
    pub changelog_changes: usize,
    /// What the regime found.
    pub regime: regime::Counts,
    /// Files that opted out of the vendored release, and how many quotes each carries.
    pub pinned: Vec<(String, String, usize)>,
    /// The families this run performed. A count belonging to a family that is not here was
    /// never taken, and printing it as zero would read as "nothing found" for a check that
    /// never ran.
    pub ran: Only,
    /// The families the caller selected. A family in `asked` and not in `ran` is one that
    /// could not run, and saying so is the difference between a check that found nothing and
    /// a check that never happened.
    pub asked: Only,
}

impl Report {
    pub fn failed(&self) -> bool {
        !self.findings.is_empty()
    }

    /// The report of a run that stopped: the stop's findings and nothing later, with what
    /// the walk read still counted, since every phase read it.
    pub fn stopped(stop: Stop, model: &Model) -> Self {
        let structure = Structure {
            walked: model.documents().len(),
            checker_source: model.checker_source().map(std::path::Path::to_path_buf),
            checker_files: model.checker_files(),
            ..Structure::default()
        };
        Report {
            phase: stop.phase,
            findings: stop.findings,
            counts: Counts::default(),
            structure,
            corpus: rules::integrity::Counts::default(),
            changelog_changes: 0,
            regime: regime::Counts::default(),
            pinned: Vec::new(),
            ran: Only::NOTHING,
            asked: Only::NOTHING,
        }
    }
}

/// Everything a check needs that it may not fetch for itself.
///
/// A check is a pure function from the model to findings, so anything requiring the
/// filesystem or the network is resolved by the caller and handed in. Resolving a release may
/// read the archive; reading a generated file reaches outside the walk, which excludes those
/// files by name.
pub struct Inputs<'a> {
    /// A pin — `None` for the vendored release — to the parsed release.
    pub releases: &'a HashMap<Option<String>, Release>,
    /// The release the project is pinned at, as its own version file states it.
    pub pinned: &'a str,
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
    /// excludes, which are other projects rather than unchecked files of this one.
    pub outside: &'a [(PathBuf, String)],
    /// The path spellings the ignore rules cover, out of every spelling a path reference in
    /// this run could ask about.
    ///
    /// One `git check-ignore` batch by the caller, keyed by `git::ignore_query`'s spelling, so
    /// a check answers the question with a lookup and spawns nothing. Asking the RULES rather
    /// than what is on disk is what makes the verdict identical on a fresh clone and a built
    /// tree, per `design@knowledge@ignored-targets-are-not-asserted`.
    pub ignored: &'a HashSet<String>,
    /// The files git both tracks and ignores, each of which is a finding.
    pub tracked_and_ignored: &'a [PathBuf],
    /// The files the walk refuses by name, per `walk::refused`, each of which is a finding.
    pub refused: &'a [PathBuf],
}

/// Which families of checks to run.
///
/// A set over the checks, one family per check, so a caller asks for exactly the subject
/// it is about to read. All but one are the modules in this directory. The exception is
/// `corpus`, which
/// is `rules::integrity::check` and lives outside them for the reason `Report::corpus` gives:
/// its subject is filesystem state, so there is no model to hand it and it is called by the
/// binary rather than from `run`.
///
/// The families are the checks themselves rather than any grouping of them: a grouping named
/// for its consumer would compile that consumer's vocabulary into the tool, and nothing about
/// this repository is compiled in.
///
/// `citations` and `structure` are kept as names for the two groupings that had consumers
/// before the set existed: `citations` is every quote judged against the rule it names, and
/// `structure` is every other family. They are the split a caller wanting one half asks for
/// without listing the families in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Only(u16);

impl Only {
    pub const CITATIONS: Self = Self(1 << 0);
    pub const GENERATED: Self = Self(1 << 1);
    pub const REGISTERS: Self = Self(1 << 2);
    pub const REFERENCES: Self = Self(1 << 3);
    pub const UNCOVERED: Self = Self(1 << 5);
    pub const CHANGES: Self = Self(1 << 6);
    pub const CORPUS: Self = Self(1 << 7);
    pub const REGIME: Self = Self(1 << 8);

    /// The empty set. What a selection naming no family would be.
    pub const NOTHING: Self = Self(0);

    /// Every check. What a run with no `--only` performs.
    pub const EVERYTHING: Self = Self(
        Self::CITATIONS.0
            | Self::GENERATED.0
            | Self::REGISTERS.0
            | Self::REFERENCES.0
            | Self::UNCOVERED.0
            | Self::CHANGES.0
            | Self::CORPUS.0
            | Self::REGIME.0,
    );
    /// Every check that is not the citation walk.
    pub const STRUCTURE: Self = Self(Self::EVERYTHING.0 & !Self::CITATIONS.0);

    /// Each family with the name that selects it, in the order the help prints them.
    ///
    /// One table, so the parser, the error message and the help cannot disagree about what
    /// exists. A check added without a row here is selectable by no name.
    pub const NAMED: [(&'static str, Self); 8] = [
        ("citations", Self::CITATIONS),
        ("generated", Self::GENERATED),
        ("registers", Self::REGISTERS),
        ("references", Self::REFERENCES),
        ("uncovered", Self::UNCOVERED),
        ("changes", Self::CHANGES),
        ("corpus", Self::CORPUS),
        ("regime", Self::REGIME),
    ];

    /// Parse a comma-separated list of family names into their union.
    ///
    /// A list rather than one name per invocation, because the single walk is a property of
    /// this design: a caller wanting five families would otherwise read every live document
    /// five times.
    pub fn parse(names: &str) -> Result<Self, String> {
        let mut set = Self(0);
        for name in names.split(',') {
            let name = name.trim();
            // An empty component contributes no family rather than failing as an unknown one.
            // A trailing comma is a typo with an obvious meaning, and asking for nothing at
            // all is caught below, where the message can say what was actually wrong.
            if name.is_empty() {
                continue;
            }
            if name == "structure" {
                set = set.union(Self::STRUCTURE);
                continue;
            }
            match Self::NAMED.iter().find(|(n, _)| *n == name) {
                Some((_, family)) => set = set.union(*family),
                None => {
                    let known: Vec<&str> = Self::NAMED.iter().map(|(n, _)| *n).collect();
                    return Err(format!(
                        "unknown check family {name:?}; expected one of {}, or structure for \
                         every family but citations, as a comma-separated list",
                        known.join(", ")
                    ));
                }
            }
        }
        if set == Self::NOTHING {
            return Err("--only needs at least one check family".to_string());
        }
        Ok(set)
    }

    /// Are ALL of `family`'s members in this set.
    ///
    /// Containment rather than overlap, because a caller may pass a group as readily as one
    /// family: `structure` is a name callers type, so `only.has(Only::STRUCTURE)` is a
    /// question someone will ask, and under an overlap test a set holding one structural
    /// family would answer it yes.
    pub fn has(self, family: Self) -> bool {
        self.0 & family.0 == family.0
    }

    /// This set without `other`'s members.
    ///
    /// What a caller uses to record that a family it asked for did not run after all.
    pub fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// The name of each family in the set, in the order `NAMED` declares them.
    pub fn names(self) -> Vec<&'static str> {
        Self::NAMED
            .iter()
            .filter(|(_, f)| self.has(*f))
            .map(|(n, _)| *n)
            .collect()
    }

    /// The union of two sets. What `parse` builds a comma list out of.
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Run the last phase, over a model `foundation` found complete.
///
/// Each family is gated on its own, so a run performs exactly what `only` names and the
/// report says which families that was.
pub fn run(model: &Model, manifest: &Manifest, inputs: &Inputs, only: Only) -> Report {
    let releases = inputs.releases;
    let mut findings = Vec::new();
    let mut counts = Counts::default();
    let mut pinned = Vec::new();

    if only.has(Only::CITATIONS) {
        for doc in model.documents() {
            let Some(release) = releases.get(&doc.pin) else {
                continue;
            };
            let exempt = manifest.lint().exempt_files.contains(&doc.rel);
            let (found, c) = citations::check(doc, release, exempt);
            findings.extend(found);
            counts.fragments += c.fragments;
            counts.verified += c.verified;
            counts.misattributed += c.misattributed;
            counts.unverified += c.unverified;
            counts.short += c.short;
            counts.commentary += c.commentary;
            counts.unmarked += c.unmarked;
            counts.orphans += c.orphans;

            // Opting out of the change detector is COUNTED, never invisible.
            if let Some(pin) = &doc.pin {
                let quotes = doc.inline_quotes().len() + doc.blocks().len();
                pinned.push((doc.rel.display().to_string(), pin.clone(), quotes));
            }
        }
        pinned.sort();
    }

    let mut regime = regime::Counts::default();
    if only.has(Only::REGIME) {
        let (found, counts) = regime::run(model, manifest, releases);
        findings.extend(found);
        regime = counts;
    }

    let mut structure = Structure::default();
    if only.has(Only::GENERATED) {
        findings.extend(generated::check(model, manifest, inputs));
    }
    if only.has(Only::REGISTERS) {
        let (found, counts) = registers::check(model, manifest, inputs);
        findings.extend(found);
        structure.components = counts.components;
        structure.locations = counts.locations;
        structure.instances = counts.instances;
        structure.entries = counts.entries;
    }
    if only.has(Only::REFERENCES) {
        let (found, c) = references::check(model, manifest, inputs);
        findings.extend(found);
        structure.entities = c.entities;
        structure.references = c.references;
        structure.links = c.links;
    }
    if only.has(Only::UNCOVERED) {
        let (found, scanned) = uncovered::check(inputs);
        findings.extend(found);
        structure.uncovered_files = scanned;
    }
    structure.walked = model.documents().len();
    structure.checker_source = model.checker_source().map(std::path::Path::to_path_buf);
    structure.checker_files = model.checker_files();

    Report {
        phase: Phase::Content,
        findings,
        counts,
        structure,
        corpus: rules::integrity::Counts::default(),
        changelog_changes: 0,
        regime,
        pinned,
        ran: only,
        asked: only,
    }
}

#[cfg(test)]
mod phase_tests {
    use super::*;
    use std::collections::HashSet;

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"p\"\ncomponents = []\n\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n"
        );
        Manifest::parse(std::path::Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The tree every component owes, in the file-shaped heading homes, plus the corpus.
    /// Directories are listed too: what exists is asked of `present`, and a checkout's
    /// listing holds a directory wherever a file sits under it.
    fn complete_tree() -> HashSet<PathBuf> {
        [
            "docs",
            "docs/open-issues",
            "r",
            "README.md",
            "CLAUDE.md",
            "docs/rejected-alternatives.md",
            "docs/design.md",
            "docs/goals.md",
            "docs/tripwires.md",
            "docs/open-issues/README.md",
            "docs/open-issues/index.md",
            "r/t",
            "r/v",
            "r/p",
            "r/m",
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
            releases: &HashMap::new(),
            pinned: "",
            committed: &HashMap::new(),
            configs: &HashMap::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
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
    pub fn implied_directories(present: &HashSet<PathBuf>) -> HashSet<PathBuf> {
        present
            .iter()
            .filter(|p| present.iter().any(|q| *q != **p && q.starts_with(p)))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Only;

    #[test]
    fn each_family_name_selects_that_family_and_no_other() {
        for (name, family) in Only::NAMED {
            let parsed = Only::parse(name).expect("a declared family name parses");
            assert_eq!(parsed, family, "{name}");
            assert_eq!(parsed.names(), vec![name]);
        }
    }

    #[test]
    fn a_comma_list_is_the_union_of_its_members() {
        let set = Only::parse("references,generated").expect("two declared names");
        assert!(set.has(Only::REFERENCES) && set.has(Only::GENERATED));
        assert!(!set.has(Only::CITATIONS));
        assert_eq!(set.names(), vec!["generated", "references"]);
        // Order and spacing are the caller's, not a second meaning.
        assert_eq!(
            Only::parse(" generated , references ").expect("spaced"),
            set
        );
    }

    #[test]
    fn the_two_names_that_had_consumers_keep_their_meaning() {
        // Both names had callers before the set existed, and both are still what a caller
        // asks for when they want one half of the run rather than a list of families. A
        // change to either changes what those callers get.
        assert_eq!(
            Only::parse("citations").expect("citations"),
            Only::CITATIONS
        );
        let structure = Only::parse("structure").expect("structure");
        assert!(!structure.has(Only::CITATIONS));
        for (name, family) in Only::NAMED {
            if name != "citations" {
                assert!(structure.has(family), "structure should hold {name}");
            }
        }
        assert_eq!(Only::CITATIONS.union(structure), Only::EVERYTHING);
    }

    #[test]
    fn membership_asks_whether_every_named_family_is_present() {
        // Overlap is not membership. `structure` is a name a caller types, so a set holding
        // one structural family must not answer yes to holding `structure`.
        let references = Only::parse("references").expect("a family");
        assert!(references.has(Only::REFERENCES));
        assert!(!references.has(Only::STRUCTURE));
        assert!(!references.has(Only::EVERYTHING));
        assert!(!Only::CITATIONS.has(Only::EVERYTHING));
        assert!(Only::EVERYTHING.has(Only::STRUCTURE));
        assert!(Only::parse("references,generated")
            .expect("two")
            .has(Only::REFERENCES.union(Only::GENERATED)));
    }

    #[test]
    fn a_name_repeated_or_already_covered_adds_nothing_and_removes_nothing() {
        // Union, not symmetric difference. Both inputs are ones a caller writes by hand.
        assert_eq!(
            Only::parse("references,references").expect("dup"),
            Only::REFERENCES
        );
        let mixed = Only::parse("structure,references").expect("overlapping");
        assert!(
            mixed.has(Only::REFERENCES),
            "references must survive being named twice"
        );
        assert_eq!(mixed, Only::STRUCTURE);
    }

    #[test]
    fn without_removes_only_what_it_names() {
        let pair = Only::REFERENCES.union(Only::GENERATED);
        assert_eq!(pair.without(Only::GENERATED), Only::REFERENCES);
        assert_eq!(pair.without(Only::CITATIONS), pair);
        assert_eq!(Only::EVERYTHING.without(Only::CITATIONS), Only::STRUCTURE);
    }

    #[test]
    fn an_unknown_family_is_an_error_that_names_what_is_accepted() {
        let e = Only::parse("references,nonesuch").expect_err("not a family");
        assert!(e.contains("nonesuch"), "{e}");
        for (name, _) in Only::NAMED {
            assert!(e.contains(name), "the error should name {name}: {e}");
        }
    }

    #[test]
    fn an_empty_selection_is_an_error_rather_than_a_run_that_checks_nothing() {
        // The message matters as much as the failure: reported as an unknown family named
        // "", it sends the reader looking for a name they did not type.
        for empty in ["", ",", "  ", " , "] {
            let e = Only::parse(empty).expect_err("no family named");
            assert!(
                e.contains("at least one"),
                "{empty:?} should say what was missing, got {e}"
            );
        }
        // A trailing comma is a typo with one obvious meaning, and is not an empty selection.
        assert_eq!(
            Only::parse("references,").expect("trailing comma"),
            Only::REFERENCES
        );
    }
}
