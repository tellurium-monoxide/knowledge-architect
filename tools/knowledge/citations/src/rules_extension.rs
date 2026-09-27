//! thaum's rules extension: every rule quote verified against a pinned corpus, per
//! `design@thaum@knowledge-is-a-generic-core`.
//!
//! It implements [`Extension`] over the checks that read the Comprehensive Rules: `citations`,
//! `regime`, `uncovered`, `changes` and `corpus`, and it generates the rule index. It is in the
//! core crate until the rules half leaves for its own Component, which is step 3 of
//! `path@thaum@docs/plans/knowledge-core-split.md`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::check::citations::{self, Release};
use crate::check::{changes, regime, uncovered};
use documentation::check::Inputs;
use documentation::extension::{
    Extension, ExtensionReport, Generated, Prepared, Purpose, Resolution, Tree,
};
use documentation::finding::Finding;
use documentation::manifest::{normalise_list, normalise_one, Manifest, MANIFEST_NAME};
use documentation::model::{Document, Model};

/// The `[rules]` table: where the corpus is, and which files are exempt from the lint.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct RulesConfig {
    /// Files exempt from the missing-marker lint and the quote regime, by their
    /// project-relative PATH.
    ///
    /// **Not by name.** A bare name matched anywhere, so a second file with the same basename
    /// inherited an exemption argued for one document. Quotes in these files are still
    /// verified.
    #[serde(default)]
    pub exempt_files: Vec<PathBuf>,
    pub dir: PathBuf,
    pub text: PathBuf,
    /// The zero-based line the rules body begins on: a property of the corpus, not of the
    /// parser.
    pub body_starts_at: usize,
    pub version: PathBuf,
    pub past: PathBuf,
    pub manifest: PathBuf,
}

impl RulesConfig {
    /// The rules corpus as `rules::Tree` needs it, with every path already resolved.
    ///
    /// The `rules` library is handed explicit paths rather than the manifest, so it keeps
    /// knowing nothing about how a project is laid out or where its declaration lives.
    pub fn tree(&self, root: &Path) -> rules::Tree {
        let dir = root.join(&self.dir);
        rules::Tree::new(
            root,
            dir.join(&self.text),
            dir.join(&self.version),
            dir.join(&self.past),
            dir.join(&self.manifest),
        )
    }

    /// Where the rule index is written: the corpus directory.
    pub fn rule_index_path(&self) -> PathBuf {
        self.dir.join("index.md")
    }
}

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
    /// The `[rules]` table as resolved against the last manifest configured, `None` where it
    /// was refused.
    config: Option<RulesConfig>,
}

impl RulesExtension {
    /// The `[rules]` table as resolved, for the `rules` commands, which read the corpus
    /// without running a check.
    pub fn config(&self) -> Option<&RulesConfig> {
        self.config.as_ref()
    }
}

impl Extension for RulesExtension {
    fn tables(&self) -> &'static [&'static str] {
        &["rules"]
    }

    /// **A refused `[rules]` path is a phase-1 finding, and the table is then absent.** The
    /// corpus has no list to drop a row from and no default to stand in, and a path kept as
    /// spelled was joined and read: `index` once wrote through a `..` corpus directory into a
    /// sibling tree.
    fn resolve(&mut self, manifest: &Manifest) -> Resolution {
        self.config = None;
        let mut resolution = Resolution::default();
        let Some(value) = manifest.table("rules") else {
            return resolution;
        };
        let mut config: RulesConfig = match value.clone().try_into() {
            Ok(config) => config,
            Err(e) => {
                resolution.complaints.push(Finding::in_file(
                    MANIFEST_NAME,
                    format!("[rules] cannot be read: {e}"),
                    "declare `dir`, `text`, `body-starts-at`, `version`, `past` and `manifest`, \
                     and optionally `exempt-files`; the citation regime runs over nothing \
                     until then",
                ));
                return resolution;
            }
        };
        normalise_list(
            "[rules] exempt-files",
            &mut config.exempt_files,
            &mut resolution.complaints,
        );
        let mut refused = false;
        for (key, path) in [
            ("dir", &mut config.dir),
            ("text", &mut config.text),
            ("version", &mut config.version),
            ("past", &mut config.past),
            ("manifest", &mut config.manifest),
        ] {
            refused |= !normalise_one(&format!("[rules] {key}"), path, &mut resolution.complaints);
        }
        if refused {
            return resolution;
        }
        // The corpus directory itself is not a row: its four files are, and a directory with
        // none of them is reported four times over. Asserting the directory would also refuse
        // a corpus at the root, whose directory is the empty path git never lists.
        let corpus: Vec<PathBuf> = [
            &config.text,
            &config.version,
            &config.past,
            &config.manifest,
        ]
        .iter()
        .map(|p| config.dir.join(p))
        .collect();
        resolution.paths.push((
            "[rules] exempt-files".to_string(),
            config.exempt_files.clone(),
        ));
        resolution.paths.push(("[rules]".to_string(), corpus));
        resolution.generated.push(config.rule_index_path());
        self.config = Some(config);
        resolution
    }

    fn checks(&self) -> &'static [&'static str] {
        &CHECKS
    }

    fn dump(&self, model: &Model) -> Vec<documentation::model::DumpRow> {
        use crate::rules_scan::{MarkerForm, RuleObservation};
        let mut rows = Vec::new();
        for (doc, document) in model.documents().iter().enumerate() {
            for l in crate::rules_scan::of(document) {
                let (kind, value) = match l.what {
                    RuleObservation::Marker { number, form } => (
                        match form {
                            MarkerForm::Prose => "marker-prose",
                            MarkerForm::Identifier => "marker-ident",
                            MarkerForm::IdentifierInProse => "marker-ident-prose",
                        },
                        number.to_string(),
                    ),
                    RuleObservation::Token(n) => ("rule-token", n.to_string()),
                };
                rows.push(documentation::model::DumpRow {
                    doc,
                    line: l.line,
                    kind,
                    value,
                });
            }
        }
        rows
    }

    fn prepare(
        &mut self,
        manifest: &Manifest,
        model: &Model,
        tree: Tree<'_>,
        purpose: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        let config = self
            .config
            .clone()
            .ok_or("the [rules] table was refused, and phase 1 reports why")?;
        match tree {
            Tree::Checkout(_) => checkout(&config, manifest, model, purpose),
            Tree::Commit(commit) => self.commit(config, model, commit),
        }
        .map(|p| Box::new(p) as Box<dyn Prepared>)
    }
}

/// The releases of the checkout: the vendored one, and for `check` every one a document pins.
///
/// Resolving a pin may read the archive or reach the network, which is why it happens here and
/// not inside a check.
fn checkout(
    config: &RulesConfig,
    manifest: &Manifest,
    model: &Model,
    purpose: Purpose,
) -> Result<RulesPrepared, String> {
    let tree = config.tree(manifest.root());
    let body_starts_at = config.body_starts_at;
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
        return Ok(RulesPrepared::new(
            config.clone(),
            releases,
            pinned,
            purpose,
        ));
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
            let pin = crate::rules_scan::pin_of(doc);
            let Some(date) = &pin else { continue };
            if releases.contains_key(&pin) {
                continue;
            }
            let path = rules::release::resolve(&tree, date)?;
            let text =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            releases.insert(pin.clone(), Release::new(&text, body_starts_at));
        }
    }
    let mut prepared = RulesPrepared::new(config.clone(), releases, pinned, purpose);
    prepared.changelog = changelog(config, manifest)?;
    Ok(prepared)
}

/// The changelog, and every release its sections name, read and parsed.
///
/// The changelog is outside the walk: each section pins its own release, so it is checked
/// against that release rather than the vendored one, and resolving one may fetch. `None` when
/// the changelog is not there, which leaves `changes` and `corpus` not run.
fn changelog(config: &RulesConfig, manifest: &Manifest) -> Result<Option<Changelog>, String> {
    let tree = config.tree(manifest.root());
    let body_starts_at = config.body_starts_at;
    let path = config.dir.join("CHANGES.md");
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
        config: RulesConfig,
        model: &Model,
        commit: &documentation::extension::CommitTree,
    ) -> Result<RulesPrepared, String> {
        let rules_dir = config.dir.clone();
        let corpus_rel = rules_dir.join(&config.text);
        let version_rel = rules_dir.join(&config.version);
        let body_starts_at = config.body_starts_at;
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
            .filter_map(crate::rules_scan::pin_of)
            .collect();
        pins.sort();
        pins.dedup();
        if !pins.is_empty() {
            let past = rules_dir.join(&config.past);
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
        let mut prepared = RulesPrepared::new(config, releases, pinned, Purpose::Commit);
        prepared.unresolved = unresolved;
        Ok(prepared)
    }
}

/// The rules extension prepared for one tree: the releases its quotes verify against.
pub struct RulesPrepared {
    config: RulesConfig,
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
        config: RulesConfig,
        releases: HashMap<Option<String>, Release>,
        pinned: String,
        purpose: Purpose,
    ) -> Self {
        RulesPrepared {
            config,
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
            .filter_map(|doc| {
                let pin = crate::rules_scan::pin_of(doc)?;
                if !self.releases.contains_key(&Some(pin.clone())) {
                    return None;
                }
                let quotes =
                    crate::quote::inline_of(doc).len() + crate::quote::blocks_of(doc).len();
                Some((doc.rel.display().to_string(), pin, quotes))
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
        let (problems, counts) =
            rules::integrity::check(&self.config.tree(manifest.root()), &needed);
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
            let Some(release) = self.releases.get(&crate::rules_scan::pin_of(doc)) else {
                continue;
            };
            let exempt = self.config.exempt_files.contains(&doc.rel);
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
        let (found, counts) = regime::run(model, &self.config.exempt_files, &self.releases);
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

    fn generated(&self, model: &Model, _: &Manifest) -> Vec<Generated> {
        let Some(vendored) = self.releases.get(&None) else {
            return Vec::new();
        };
        vec![Generated {
            rel: self.config.rule_index_path(),
            text: crate::rule_index::rule_index(
                model,
                &self.config.dir,
                &vendored.rules,
                &self.pinned,
            ),
            action: "regenerate it and read the diff: it is the work list a release bump reads",
        }]
    }
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
pub(crate) mod tests {
    use super::{render, RulesConfig, RulesExtension, Summary};
    use documentation::extension::Extension;
    use documentation::manifest::Manifest;
    use std::path::{Path, PathBuf};

    /// This repository's manifest: the nearest one above this crate.
    pub(crate) fn this_project() -> Manifest {
        Manifest::find(Path::new(env!("CARGO_MANIFEST_DIR"))).expect("this project's manifest")
    }

    /// The `[rules]` table of a manifest, as the extension resolves it.
    pub(crate) fn configured(manifest: &Manifest) -> RulesConfig {
        let mut rules = RulesExtension::default();
        let resolution = rules.resolve(manifest);
        assert!(
            resolution.complaints.is_empty(),
            "{:?}",
            resolution.complaints
        );
        rules.config().cloned().expect("a [rules] table")
    }

    fn manifest_with(rules: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = []\n\
             skip-files = []\nexclude = []\n\n[rules]\n{rules}"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    fn corpus_rows(dir: &str, text: &str, version: &str, past: &str, manifest: &str) -> String {
        format!(
            "dir = \"{dir}\"\ntext = \"{text}\"\nbody-starts-at = 0\nversion = \"{version}\"\n\
             past = \"{past}\"\nmanifest = \"{manifest}\"\n"
        )
    }

    #[test]
    fn this_repository_declares_a_corpus_that_is_there() {
        let manifest = this_project();
        assert!(configured(&manifest).tree(manifest.root()).text().is_file());
    }

    #[test]
    fn a_refused_corpus_path_is_a_complaint_and_leaves_no_table() {
        // The corpus has no default to stand in for a refused row, and a path kept as
        // spelled was joined and read: `index` once wrote through a `..` corpus directory.
        // Every one of the five rows is refused the same way.
        for (key, rows) in [
            ("dir", corpus_rows("../corpus", "t", "v", "p", "m")),
            ("text", corpus_rows("corpus", "/t", "v", "p", "m")),
            ("version", corpus_rows("corpus", "t", "../v", "p", "m")),
            ("past", corpus_rows("corpus", "t", "v", "../p", "m")),
            ("manifest", corpus_rows("corpus", "t", "v", "p", "/m")),
        ] {
            let mut rules = RulesExtension::default();
            let resolution = rules.resolve(&manifest_with(&rows));
            assert!(
                resolution
                    .complaints
                    .iter()
                    .any(|c| c.what.contains(&format!("in [rules] {key}"))),
                "{key}: {:?}",
                resolution.complaints
            );
            assert!(rules.config().is_none(), "{key}: a refused table is absent");
            assert!(resolution.generated.is_empty(), "{key}");
        }
        let config = configured(&manifest_with(&corpus_rows(
            "./corpus", "./t", "v", "p", "m",
        )));
        assert_eq!(config.dir, PathBuf::from("corpus"));
        assert_eq!(config.text, PathBuf::from("t"));
    }

    #[test]
    fn the_exemption_list_defaults_to_empty_and_a_refused_row_leaves_it() {
        let config = configured(&manifest_with(&corpus_rows("r", "t", "v", "p", "m")));
        assert!(config.exempt_files.is_empty());
        let rows = format!(
            "exempt-files = [\"./notes/x.md\", \"../y.md\"]\n{}",
            corpus_rows("r", "t", "v", "p", "m")
        );
        let mut rules = RulesExtension::default();
        let resolution = rules.resolve(&manifest_with(&rows));
        assert_eq!(
            resolution.complaints.len(),
            1,
            "{:?}",
            resolution.complaints
        );
        assert!(resolution.complaints[0]
            .what
            .contains("[rules] exempt-files"));
        let config = rules.config().expect("the table stands");
        assert_eq!(config.exempt_files, vec![PathBuf::from("notes/x.md")]);
    }

    #[test]
    fn the_corpus_paths_and_the_rule_index_are_declared_for_the_core() {
        let mut rules = RulesExtension::default();
        let resolution = rules.resolve(&manifest_with(&corpus_rows("r", "t", "v", "p", "m")));
        assert_eq!(resolution.generated, vec![PathBuf::from("r/index.md")]);
        let corpus = resolution
            .paths
            .iter()
            .find(|(label, _)| label == "[rules]")
            .expect("the corpus row");
        assert_eq!(
            corpus.1,
            ["r/t", "r/v", "r/p", "r/m"].map(PathBuf::from).to_vec()
        );
    }

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
