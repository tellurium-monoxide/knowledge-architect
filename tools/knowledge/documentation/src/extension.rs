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
use crate::manifest::Manifest;
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

/// An extension, configured once per run.
pub trait Extension {
    /// The names of its checks, as the `checked:` line prints them after the core's.
    fn checks(&self) -> &'static [&'static str];

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
