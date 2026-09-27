//! How a project's own subject plugs into the core, per
//! `design@knowledge@an-extension-plugs-in-through-phased-hooks`.
//!
//! An extension is a set of checks over a subject one project has, such as thaum's rule quotes
//! verified against a pinned corpus. A binary registers its extensions at compile time and
//! hands them to [`crate::cli::run`]; the core calls each one at fixed points of a run. The
//! core itself knows no extension.
//!
//! **Two levels.** An [`Extension`] is configured once per run. It prepares one [`Prepared`]
//! per tree it judges, because `commits` judges several trees in one run and a message is
//! judged against what its own commit's tree holds.

use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::check::Inputs;
use crate::finding::Finding;
use crate::manifest::{Manifest, MANIFEST_NAME};
use crate::model::{Document, Model};

/// What a tree is prepared for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    /// `check` over the checkout: every check of the extension.
    Check,
    /// One commit's tree under `commits`. A check whose subject is filesystem state has no
    /// subject here, per `design@knowledge@a-commit-message-is-a-document`, and a release the
    /// tree cannot supply is a finding of the tree rather than a could-not-run.
    Commit,
    /// `index`: what the extension generates, and nothing fetched over the network.
    Index,
}

/// The tree being judged.
pub enum Tree<'a> {
    /// The working tree. An extension may read the filesystem under this root: a subject such
    /// as a vendored corpus is filesystem state, per `design@knowledge@model-then-checks`.
    Checkout(&'a Path),
    /// One commit's tree, read from git objects only.
    Commit(&'a CommitTree),
}

/// One commit's tree, as an extension may read it.
pub struct CommitTree {
    root: PathBuf,
    sha: String,
    listing: Vec<PathBuf>,
    /// Every blob already read for the walk, so an extension asking for one costs nothing.
    blobs: BTreeMap<PathBuf, String>,
}

impl CommitTree {
    pub fn new(
        root: &Path,
        sha: &str,
        listing: Vec<PathBuf>,
        blobs: BTreeMap<PathBuf, String>,
    ) -> Self {
        CommitTree {
            root: root.to_path_buf(),
            sha: sha.to_string(),
            listing,
            blobs,
        }
    }

    /// Whether the tree holds a path, as a file, a symlink or a gitlink.
    pub fn holds(&self, rel: &Path) -> bool {
        self.listing.iter().any(|p| p == rel)
    }

    /// The text of each path the tree holds as a file, read in one batch. A path the tree does
    /// not hold, or whose bytes are not text, is absent from the answer.
    pub fn read(&self, rels: &[PathBuf]) -> io::Result<BTreeMap<PathBuf, String>> {
        let mut out = BTreeMap::new();
        let mut wanted = Vec::new();
        for rel in rels {
            match self.blobs.get(rel) {
                Some(text) => {
                    out.insert(rel.clone(), text.clone());
                }
                None if self.holds(rel) => wanted.push(rel.clone()),
                None => {}
            }
        }
        if !wanted.is_empty() {
            out.extend(crate::git::blobs(&self.root, &self.sha, &wanted)?);
        }
        Ok(out)
    }

    /// The name of the blob at a path, which stays the same across commits that hold the same
    /// bytes. An extension keys what it parsed from a blob by it, so a parse is paid once per
    /// content rather than once per commit.
    pub fn object_id(&self, rel: &Path) -> Option<String> {
        crate::git::rev_parse(&self.root, &crate::git::tree_object(&self.sha, rel))
    }
}

/// A file an extension generates, and what its bytes must be now.
pub struct Generated {
    pub rel: PathBuf,
    pub text: String,
    /// What the `generated` check tells a reader to do when the committed file differs.
    pub action: &'static str,
}

/// What an extension's checks produced over one tree.
#[derive(Default)]
pub struct ExtensionReport {
    pub findings: Vec<Finding>,
    /// The names of the checks that got no input and did not run, printed as not run rather
    /// than counted, because a count nobody took reads as nothing found.
    pub not_run: Vec<&'static str>,
    /// The extension's block of the summary, printed after the core's count lines. Each line
    /// starts with a newline, as the core's lines do.
    pub summary: String,
}

/// What an extension made of its tables.
#[derive(Default)]
pub struct Resolution {
    /// Declarations it refused, each a finding of phase 1. A refused declaration is absent
    /// from its configuration, as the core's are.
    pub complaints: Vec<Finding>,
    /// The paths it declares, by the label a finding names them with. Phase 2 asserts that
    /// each exists, with the core's own declared paths.
    pub paths: Vec<(String, Vec<PathBuf>)>,
    /// The files it generates. The walk leaves them out, and the `generated` check reads them
    /// as committed.
    pub generated: Vec<PathBuf>,
}

/// Configure every extension against a manifest, and record in the manifest what each made of
/// its tables.
///
/// **A table no registered extension claims, and a claimed table the manifest does not hold,
/// are phase-1 findings**, per `design@knowledge@an-extension-claims-its-manifest-tables`. The
/// first is a declaration nothing reads, which a session would take for a regime in force; the
/// second would let a manifest switch an extension off by leaving its table out.
pub fn configure(manifest: &mut Manifest, extensions: &mut [Box<dyn Extension>]) {
    let mut complaints = Vec::new();
    let claimed: Vec<&str> = extensions
        .iter()
        .flat_map(|e| e.tables().iter().copied())
        .collect();
    for table in manifest.extension_tables() {
        if !claimed.contains(&table) {
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                format!("[{table}] is a table no extension of this binary reads"),
                "delete it, or run the binary that registers the extension it belongs to; a \
                 declaration nothing reads looks like a check in force",
            ));
        }
    }
    let present: Vec<String> = manifest.extension_tables().map(str::to_string).collect();
    for table in &claimed {
        if !present.iter().any(|p| p == table) {
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                format!("[{table}] is missing, and an extension this binary registers reads it"),
                "declare it; a manifest cannot switch an extension off by leaving its table out",
            ));
        }
    }
    let mut paths = Vec::new();
    let mut generated = Vec::new();
    for extension in extensions.iter_mut() {
        let resolution = extension.resolve(manifest);
        complaints.extend(resolution.complaints);
        paths.extend(resolution.paths);
        generated.extend(resolution.generated);
    }
    manifest.record_extensions(complaints, paths, generated);
}

/// An extension, configured once per manifest.
pub trait Extension {
    /// The top-level manifest tables it reads.
    fn tables(&self) -> &'static [&'static str];

    /// Read its tables out of the manifest. Called once per manifest, before the walk, by
    /// [`configure`]; a table it claims that the manifest does not hold is already reported.
    fn resolve(&mut self, manifest: &Manifest) -> Resolution;

    /// The names of its checks, as the `checked:` line prints them after the core's.
    fn checks(&self) -> &'static [&'static str];

    /// Its observation rows for `model`, merged into the core's. Runs without the foundation,
    /// as `model` does.
    fn dump(&self, model: &Model) -> Vec<crate::model::DumpRow>;

    /// Read what its checks need for one tree. Runs only once the core's first three phases
    /// passed over that tree, so a run that stops earlier resolves and fetches nothing. `Err`
    /// is could-not-run.
    fn prepare(
        &mut self,
        manifest: &Manifest,
        model: &Model,
        tree: Tree<'_>,
        purpose: Purpose,
    ) -> Result<Box<dyn Prepared>, String>;
}

/// An extension prepared for one tree.
pub trait Prepared {
    /// Every check of the extension over the tree, in the last phase.
    fn check(&self, model: &Model, manifest: &Manifest, inputs: &Inputs) -> ExtensionReport;

    /// The rules a commit message is judged by, against this tree.
    fn check_message(&self, message: &Document) -> Vec<Finding>;

    /// The files the extension generates, with the bytes they must hold for this tree.
    fn generated(&self, model: &Model, manifest: &Manifest) -> Vec<Generated>;
}

#[cfg(test)]
mod tests {
    use super::{configure, Extension, Prepared, Purpose, Resolution, Tree};
    use crate::manifest::Manifest;
    use crate::model::Model;
    use std::path::Path;

    /// An extension that claims one table and reads nothing out of it.
    struct Claiming;

    impl Extension for Claiming {
        fn tables(&self) -> &'static [&'static str] {
            &["claimed"]
        }
        fn resolve(&mut self, _: &Manifest) -> Resolution {
            Resolution::default()
        }
        fn checks(&self) -> &'static [&'static str] {
            &[]
        }
        fn dump(&self, _: &Model) -> Vec<crate::model::DumpRow> {
            Vec::new()
        }
        fn prepare(
            &mut self,
            _: &Manifest,
            _: &Model,
            _: Tree<'_>,
            _: Purpose,
        ) -> Result<Box<dyn Prepared>, String> {
            Err("not prepared in this test".to_string())
        }
    }

    fn manifest_with(tables: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = []\n\
             skip-files = []\n\n{tables}"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    fn complaints(tables: &str, extensions: &mut [Box<dyn Extension>]) -> Vec<String> {
        let mut manifest = manifest_with(tables);
        configure(&mut manifest, extensions);
        manifest
            .complaints()
            .iter()
            .map(|f| f.what.clone())
            .collect()
    }

    #[test]
    fn a_table_no_extension_claims_is_a_complaint() {
        // A `[lint]` table left behind, read by nothing, would look like a check in force.
        let found = complaints("[lint]\nexempt-files = []\n", &mut []);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("[lint] is a table no extension"),
            "{found:?}"
        );
        let found = complaints("[claimed]\n", &mut [Box::new(Claiming)]);
        assert!(found.is_empty(), "a claimed table is read: {found:?}");
    }

    #[test]
    fn a_claimed_table_the_manifest_leaves_out_is_a_complaint() {
        // Otherwise a manifest could switch an extension off by leaving its table out.
        let found = complaints("", &mut [Box::new(Claiming)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("[claimed] is missing"), "{found:?}");
    }
}
