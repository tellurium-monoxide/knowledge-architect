//! thaum's rules extension: every rule quote verified against a pinned corpus, per
//! `design@thaum@knowledge-is-a-generic-core`.
//!
//! It implements [`Extension`] over the checks that read the Comprehensive Rules: `citations`,
//! `regime`, `uncovered`, `changes` and `corpus`, and it generates the rule index. It is in the
//! core crate until the rules half leaves for its own Component, which is step 3 of
//! `path@thaum@docs/plans/knowledge-core-split.md`.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::check::citations::{self, Release};
use crate::check::{changes, regime, uncovered, Inputs};
use crate::extension::{Extension, ExtensionReport, Generated, Prepared, Purpose, Tree};
use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::{Document, Model};

/// The checks of this extension, by the name each one's count line carries.
pub const CHECKS: [&str; 5] = ["citations", "uncovered", "changes", "corpus", "regime"];

/// The rules extension, configured once per run.
#[derive(Default)]
pub struct RulesExtension {
    /// Parsed releases kept across the commits of a `commits` run, keyed by the blob that holds
    /// them.
    ///
    /// **Keyed by the object name, not by the pinned date.** Parsing the corpus is the single
    /// largest cost in a per-commit run, and two commits pinning one date may still carry
    /// different bytes — a fold, a re-fetch. The object name answers both questions at once.
    parsed: HashMap<String, Release>,
}

impl Extension for RulesExtension {
    fn checks(&self) -> &'static [&'static str] {
        &CHECKS
    }

    fn prepare(
        &mut self,
        manifest: &Manifest,
        model: &Model,
        tree: Tree<'_>,
        purpose: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        match tree {
            Tree::Checkout(_) => checkout(manifest, model, purpose),
            Tree::Commit(commit) => self.commit(manifest, model, commit),
        }
        .map(|p| Box::new(p) as Box<dyn Prepared>)
    }
}

/// The releases of the checkout: the vendored one, and for `check` every one a document pins.
///
/// Resolving a pin may read the archive or reach the network, which is why it happens here and
/// not inside a check.
fn checkout(manifest: &Manifest, model: &Model, purpose: Purpose) -> Result<RulesPrepared, String> {
    let tree = manifest.rules_tree();
    let body_starts_at = manifest.rules().body_starts_at;
    if purpose == Purpose::Index {
        // Only the vendored release: the rule index lists what the tree cites against it, and
        // regenerating it fetches nothing.
        let text = std::fs::read_to_string(tree.text())
            .map_err(|e| format!("{}: {e}", tree.text().display()))?;
        let version = std::fs::read_to_string(tree.version())
            .map_err(|e| format!("{}: {e}", tree.version().display()))?;
        let pinned = rules::release::read_version(&version)
            .get("date")
            .cloned()
            .ok_or("VERSION names no date")?;
        let releases = HashMap::from([(None, Release::new(&text, body_starts_at))]);
        return Ok(RulesPrepared::new(releases, pinned, purpose));
    }
    let pinned = std::fs::read_to_string(tree.version())
        .ok()
        .and_then(|v| rules::release::read_version(&v).get("date").cloned())
        .unwrap_or_default();
    if pinned.is_empty() {
        return Err(format!(
            "{}: names no date, and the last phase verifies every quote against it",
            tree.version().display()
        ));
    }
    let mut releases = HashMap::new();
    let vendored = std::fs::read_to_string(tree.text())
        .map_err(|e| format!("{}: {e}", tree.text().display()))?;
    releases.insert(None, Release::new(&vendored, body_starts_at));
    {
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
    let mut prepared = RulesPrepared::new(releases, pinned, purpose);
    prepared.changelog = changelog(manifest)?;
    Ok(prepared)
}

/// The changelog, and every release its sections name, read and parsed.
///
/// The changelog is outside the walk: each section pins its own release, so it is checked
/// against that release rather than the vendored one, and resolving one may fetch. `None` when
/// the changelog is not there, which leaves `changes` and `corpus` not run.
fn changelog(manifest: &Manifest) -> Result<Option<Changelog>, String> {
    let tree = manifest.rules_tree();
    let body_starts_at = manifest.rules().body_starts_at;
    let path = manifest.rules().dir.join("CHANGES.md");
    let Ok(text) = std::fs::read_to_string(manifest.root().join(&path)) else {
        return Ok(None);
    };
    let (sections, _) = changes::parse(&text).unwrap_or_default();
    let mut corpora = HashMap::new();
    let mut digests = HashMap::new();
    for section in &sections {
        let resolved = rules::release::resolve(&tree, &section.version)?;
        let bytes = std::fs::read(&resolved).map_err(|e| e.to_string())?;
        digests.insert(section.version.clone(), rules::release::sha256(&bytes));
        let text = String::from_utf8_lossy(&bytes).to_string();
        corpora.insert(
            section.version.clone(),
            rules::Corpus::parse(&text, body_starts_at),
        );
    }
    let versions = sections.iter().map(|s| s.version.clone()).collect();
    Ok(Some(Changelog {
        path,
        text,
        versions,
        corpora,
        digests,
    }))
}

/// The changelog as `prepare` read it.
struct Changelog {
    path: PathBuf,
    text: String,
    /// The release each section names, in order.
    versions: Vec<String>,
    corpora: HashMap<String, rules::Corpus>,
    digests: HashMap<String, String>,
}

impl RulesExtension {
    /// The releases one commit's tree supplies, read from its git objects.
    ///
    /// **A release the tree cannot supply is a finding of the tree**, not a could-not-run: a
    /// quote checked against no release is checked by nothing at all, so the commit then fails
    /// the range, which is the honest answer. A document that opts out of the vendored release
    /// verifies against the archive the COMMIT holds, never against the working tree's.
    fn commit(
        &mut self,
        manifest: &Manifest,
        model: &Model,
        commit: &crate::extension::CommitTree,
    ) -> Result<RulesPrepared, String> {
        let rules_dir = manifest.rules().dir.clone();
        let corpus_rel = rules_dir.join(&manifest.rules().text);
        let version_rel = rules_dir.join(&manifest.rules().version);
        let body_starts_at = manifest.rules().body_starts_at;
        let read = commit
            .read(&[corpus_rel.clone(), version_rel.clone()])
            .map_err(|e| format!("its corpus could not be read: {e}"))?;
        let mut releases = HashMap::new();
        let mut unresolved = Vec::new();
        match read.get(&corpus_rel) {
            None => unresolved.push(Finding::in_file(
                &corpus_rel,
                "this commit's tree holds no vendored release at the path its manifest names",
                "every quote in the tree and in the message verifies against nothing without it",
            )),
            Some(text) => {
                let key = commit
                    .object_id(&corpus_rel)
                    .unwrap_or_else(|| format!("{}:corpus", corpus_rel.display()));
                let release = self
                    .parsed
                    .entry(key)
                    .or_insert_with(|| Release::new(text, body_starts_at));
                releases.insert(None, release.clone());
            }
        }
        let pinned = read
            .get(&version_rel)
            .and_then(|t| rules::release::read_version(t).get("date").cloned())
            .unwrap_or_default();
        let mut pins: Vec<String> = model
            .documents()
            .iter()
            .filter_map(|d| d.pin.clone())
            .collect();
        pins.sort();
        pins.dedup();
        if !pins.is_empty() {
            let past = rules_dir.join(&manifest.rules().past);
            let wanted: Vec<PathBuf> = pins.iter().map(|d| past.join(format!("{d}.txt"))).collect();
            let archived = commit
                .read(&wanted)
                .map_err(|e| format!("its archive could not be read: {e}"))?;
            for (date, rel) in pins.iter().zip(&wanted) {
                match archived.get(rel) {
                    Some(text) => {
                        releases.insert(Some(date.clone()), Release::new(text, body_starts_at));
                    }
                    None => unresolved.push(Finding::in_file(
                        rel,
                        format!(
                            "a document of this commit pins the release {date}, and its tree \
                             holds no archive of it"
                        ),
                        "the quotes of every file pinned there verify against nothing",
                    )),
                }
            }
        }
        let mut prepared = RulesPrepared::new(releases, pinned, Purpose::Commit);
        prepared.unresolved = unresolved;
        Ok(prepared)
    }
}

/// The rules extension prepared for one tree: the releases its quotes verify against.
pub struct RulesPrepared {
    /// A pin — `None` for the vendored release — to the parsed release.
    releases: HashMap<Option<String>, Release>,
    /// The release the project is pinned at, as its own version file states it.
    pinned: String,
    purpose: Purpose,
    /// The releases the tree could not supply, each a finding of the tree.
    unresolved: Vec<Finding>,
    /// The changelog, read for `check` over the checkout.
    changelog: Option<Changelog>,
}

impl RulesPrepared {
    /// Prepared from releases already parsed, which is how a test hands in the corpus of a mock.
    pub fn new(
        releases: HashMap<Option<String>, Release>,
        pinned: String,
        purpose: Purpose,
    ) -> Self {
        RulesPrepared {
            releases,
            pinned,
            purpose,
            unresolved: Vec::new(),
            changelog: None,
        }
    }

    /// Every document that opts out of the vendored release, with the release it pins and how
    /// many quotes it carries, sorted by path.
    ///
    /// Opting out of the change detector is COUNTED, never invisible.
    pub fn pinned_containers(&self, model: &Model) -> Vec<(String, String, usize)> {
        let mut pinned: Vec<(String, String, usize)> = model
            .documents()
            .iter()
            .filter(|doc| self.releases.contains_key(&doc.pin))
            .filter_map(|doc| {
                let pin = doc.pin.as_ref()?;
                let quotes = doc.inline_quotes().len() + doc.blocks().len();
                Some((doc.rel.display().to_string(), pin.clone(), quotes))
            })
            .collect();
        pinned.sort();
        pinned
    }

    /// The changelog and the archive: their subject is filesystem state rather than the model,
    /// so they run over the checkout alone.
    ///
    /// Both are gated on the changelog being readable, so when it is not there neither ran.
    /// Saying otherwise would print a zero for a check nobody performed, which for `corpus`
    /// means reporting provenance clean without having read a byte of it.
    fn changes_and_corpus(
        &self,
        manifest: &Manifest,
        report: &mut ExtensionReport,
        summary: &mut Summary,
    ) {
        let Some(log) = &self.changelog else {
            report.not_run = vec!["changes", "corpus"];
            return;
        };
        let (found, changed) = changes::check(&log.path, &log.text, &log.corpora, &log.digests);
        report.findings.extend(found);
        summary.changelog_changes = changed;
        // Every release the changelog names must already be in the tree, alongside the pin.
        let mut needed = vec![(self.pinned.clone(), "the pin".to_string())];
        needed.extend(
            log.versions
                .iter()
                .map(|v| (v.clone(), format!("the changelog section {v}"))),
        );
        let (problems, counts) = rules::integrity::check(&manifest.rules_tree(), &needed);
        for p in problems {
            report
                .findings
                .push(Finding::in_file(&p.about, p.what, p.action));
        }
        summary.corpus = counts;
    }
}

/// The counts the extension's summary block prints.
#[derive(Default)]
struct Summary {
    citations: citations::Counts,
    pinned: Vec<(String, String, usize)>,
    uncovered_files: usize,
    corpus: rules::integrity::Counts,
    changelog_changes: usize,
    regime: regime::Counts,
}

impl Prepared for RulesPrepared {
    fn check(&self, model: &Model, manifest: &Manifest, inputs: &Inputs) -> ExtensionReport {
        let mut report = ExtensionReport::default();
        let mut summary = Summary::default();
        for doc in model.documents() {
            let Some(release) = self.releases.get(&doc.pin) else {
                continue;
            };
            let exempt = manifest.rules().exempt_files.contains(&doc.rel);
            let (found, c) = citations::check(doc, release, exempt);
            report.findings.extend(found);
            let t = &mut summary.citations;
            t.fragments += c.fragments;
            t.verified += c.verified;
            t.misattributed += c.misattributed;
            t.unverified += c.unverified;
            t.short += c.short;
            t.commentary += c.commentary;
            t.unmarked += c.unmarked;
            t.orphans += c.orphans;
        }
        summary.pinned = self.pinned_containers(model);
        let (found, counts) = regime::run(model, manifest, &self.releases);
        report.findings.extend(found);
        summary.regime = counts;
        let (found, scanned) = uncovered::check(inputs);
        report.findings.extend(found);
        summary.uncovered_files = scanned;
        if self.purpose == Purpose::Check {
            self.changes_and_corpus(manifest, &mut report, &mut summary);
        } else {
            report.not_run = vec!["changes", "corpus"];
        }
        report.findings.extend(self.unresolved.iter().cloned());
        report.summary = render(&summary, &report.not_run);
        report
    }

    fn check_message(&self, message: &Document) -> Vec<Finding> {
        // The message is never lint-exempt: the exemption list names files that are leaving
        // the tree, and a message is written now.
        let Some(release) = self.releases.get(&None) else {
            return Vec::new();
        };
        let (mut out, _) = citations::check(message, release, false);
        let (judged, _) = regime::check(message, release);
        out.extend(judged.into_iter().map(|j| j.finding));
        out
    }

    fn generated(&self, model: &Model, manifest: &Manifest) -> Vec<Generated> {
        let Some(vendored) = self.releases.get(&None) else {
            return Vec::new();
        };
        vec![Generated {
            rel: rule_index_path(manifest),
            text: crate::index::rule_index(model, manifest, &vendored.rules, &self.pinned),
            action: "regenerate it and read the diff: it is the work list a release bump reads",
        }]
    }
}

/// Where the rule index is written: the corpus directory the manifest declares.
pub fn rule_index_path(manifest: &Manifest) -> PathBuf {
    manifest.rules().dir.join("index.md")
}

/// The extension's block of the summary.
///
/// **A check that did not run contributes nothing.** Its counts are not zero, they are unasked,
/// and a zero would read as nothing found for a check that never ran.
fn render(s: &Summary, not_run: &[&str]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let c = &s.citations;
    let _ = write!(
        out,
        "\n\n{}/{} rule-quote fragments verified against the rule cited",
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
            citations::MIN_FRAGMENT
        );
    }
    if s.pinned.is_empty() {
        let _ = write!(
            out,
            "\n\nno pinned containers: every quote tracks the vendored release"
        );
    } else {
        let _ = write!(
            out,
            "\n\npinned containers (their quotes do not track the vendored release):"
        );
        for (file, date, quotes) in &s.pinned {
            let _ = write!(out, "\n  {file}: cr-version {date}, {quotes} quote(s)");
        }
    }
    let _ = write!(
        out,
        "\n\nlint: {} unmarked rule reference(s), {} orphan identifier marker(s)",
        c.unmarked, c.orphans
    );
    let _ = write!(out, "\nuncovered files: {} scanned", s.uncovered_files);
    if !not_run.contains(&"corpus") {
        let _ = write!(
            out,
            "\ncorpus: {} archived release(s), {} manifest row(s), {} of {} release(s) resolved locally",
            s.corpus.archived,
            s.corpus.manifest_rows,
            s.corpus.releases_local,
            s.corpus.releases_needed
        );
    }
    if !not_run.contains(&"changes") {
        let _ = write!(out, "\nchangelog: {} rule change(s)", s.changelog_changes);
    }
    let _ = write!(
        out,
        "\nregime: {} claim(s) judged against their scope",
        s.regime.claims
    );
    out
}

#[cfg(test)]
mod tests {
    use super::{render, Summary};

    /// A summary whose every number is different from every other, so two counts printed in
    /// the wrong order cannot render alike.
    fn numbered() -> Summary {
        let mut s = Summary::default();
        s.citations.verified = 11;
        s.citations.fragments = 22;
        s.citations.unmarked = 33;
        s.citations.orphans = 44;
        s.uncovered_files = 122;
        s.corpus.archived = 133;
        s.corpus.manifest_rows = 144;
        s.corpus.releases_local = 155;
        s.corpus.releases_needed = 166;
        s.changelog_changes = 177;
        s.regime.claims = 188;
        s
    }

    #[test]
    fn every_count_is_rendered_in_the_position_its_label_promises() {
        let out = render(&numbered(), &[]);
        for expected in [
            "11/22 rule-quote fragments verified against the rule cited",
            "lint: 33 unmarked rule reference(s), 44 orphan identifier marker(s)",
            "uncovered files: 122 scanned",
            "corpus: 133 archived release(s), 144 manifest row(s),",
            "155 of 166 release(s) resolved locally",
            "changelog: 177 rule change(s)",
            "regime: 188 claim(s) judged against their scope",
        ] {
            assert!(out.contains(expected), "missing {expected:?} in {out}");
        }
    }

    #[test]
    fn a_check_that_could_not_run_contributes_no_count() {
        // A zero for it would read as "nothing found" for a check that never ran.
        let out = render(&numbered(), &["changes", "corpus"]);
        assert!(
            !out.contains("corpus:") && !out.contains("changelog:"),
            "no count for a check that did not run: {out}"
        );
        assert!(out.contains("fragments verified"), "{out}");
    }
}
