//! The checks. Each is a pure function from the model to findings.
//!
//! No check reads a file, spawns a process, or knows how the walk works. That is what makes
//! the single walk a property of the design rather than of anyone's care, and it is what lets
//! a check be run against a model assembled in memory.

pub mod changes;
pub mod citations;
pub mod components;
pub mod generated;
pub mod interpretations;
pub mod references;
pub mod regime;
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
    pub additional_trackers: usize,
    /// Distinct entities the table holds, reference occurrences judged against it, and the
    /// relative markdown links resolved.
    pub entities: usize,
    pub references: usize,
    pub links: usize,
    pub uncovered_files: usize,
    pub concerns: usize,
    pub entries: usize,
    pub top_entry: u16,
    /// The checker's own directory as the model was told it, and how many walked Rust files
    /// sit under it with their string literals read as data. Not a family's count: it describes
    /// the walk every family read, so it is set whatever was asked for.
    pub checker_source: Option<std::path::PathBuf>,
    pub checker_files: usize,
}

/// What a whole run found and counted.
pub struct Report {
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
    /// Every path that exists in the project, files and directories, project-relative. One
    /// listing by the caller answers every question a check has about what is there.
    pub present: &'a HashSet<PathBuf>,
    /// The subset of `present` that is directories, so a check can assert a path's kind —
    /// a required document is a file, and a reference's trailing slash claims a directory.
    pub directories: &'a HashSet<PathBuf>,
    /// Files the walk does not cover, with their text — excluding the paths the manifest
    /// excludes, which are other projects rather than unchecked files of this one.
    pub outside: &'a [(PathBuf, String)],
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
    pub const COMPONENTS: Self = Self(1 << 2);
    pub const REFERENCES: Self = Self(1 << 3);
    pub const INTERPRETATIONS: Self = Self(1 << 4);
    pub const UNCOVERED: Self = Self(1 << 5);
    pub const CHANGES: Self = Self(1 << 6);
    pub const CORPUS: Self = Self(1 << 7);
    pub const REGIME: Self = Self(1 << 8);

    /// The empty set. What a selection naming no family would be.
    pub const NOTHING: Self = Self(0);

    /// Every check. What a run with no `--only` performs.
    pub const EVERYTHING: Self = Self(0b1_1111_1111);
    /// Every check that is not the citation walk.
    pub const STRUCTURE: Self = Self(Self::EVERYTHING.0 & !Self::CITATIONS.0);

    /// Each family with the name that selects it, in the order the help prints them.
    ///
    /// One table, so the parser, the error message and the help cannot disagree about what
    /// exists. A check added without a row here is selectable by no name.
    pub const NAMED: [(&'static str, Self); 9] = [
        ("citations", Self::CITATIONS),
        ("generated", Self::GENERATED),
        ("components", Self::COMPONENTS),
        ("references", Self::REFERENCES),
        ("interpretations", Self::INTERPRETATIONS),
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

/// Run the checks.
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
    if only.has(Only::COMPONENTS) {
        let (found, counts) = components::check(model, manifest, inputs);
        findings.extend(found);
        structure.components = counts.components;
        structure.additional_trackers = counts.additional_trackers;
    }
    if only.has(Only::REFERENCES) {
        let (found, c) = references::check(model, manifest, inputs);
        findings.extend(found);
        structure.entities = c.entities;
        structure.references = c.references;
        structure.links = c.links;
    }
    if only.has(Only::INTERPRETATIONS) {
        let (found, (concerns, entries, top)) = interpretations::check(model, manifest);
        findings.extend(found);
        (structure.concerns, structure.entries, structure.top_entry) = (concerns, entries, top);
    }
    if only.has(Only::UNCOVERED) {
        let (found, scanned) = uncovered::check(inputs);
        findings.extend(found);
        structure.uncovered_files = scanned;
    }
    structure.checker_source = model.checker_source().map(std::path::Path::to_path_buf);
    structure.checker_files = model.checker_files();

    Report {
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
