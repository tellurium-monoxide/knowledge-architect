//! What a run fetches before any check runs, gathered once for every caller.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::check::Inputs;
use crate::entity::Anchors;
use crate::survey::Survey;
use crate::{Manifest, Model};

/// Everything a check needs that it may not fetch for itself, read from the working tree.
///
/// A check is a pure function over the model, so the files and the git answers it reads are
/// fetched here by the caller, per `design@core@model-then-checks`. `check` builds its
/// [`Inputs`] from this, a writer asks [`complete_working_tree`](super::complete_working_tree)
/// for one, and an extension's tests build theirs the same way, so no caller assembles
/// [`Inputs`] field by field.
pub struct Gathered {
    committed: HashMap<PathBuf, String>,
    configs: HashMap<PathBuf, String>,
    survey: Survey,
    ignored: HashSet<String>,
    tracked_and_ignored: Vec<PathBuf>,
    shipped: Vec<(PathBuf, String)>,
}

impl Gathered {
    /// Read the working tree under the manifest's root: the committed generated files, every
    /// `register.toml`, one survey, the two git batches, and the agent files this version
    /// ships.
    ///
    /// `Err` is could-not-run: git or the filesystem refused a read the run cannot do without.
    /// A generated file or a `register.toml` that is absent is not an error: the checks that
    /// read them report the absence.
    pub fn over(manifest: &Manifest, model: &Model) -> Result<Gathered, String> {
        // The generated files are outside the walk — an extension's by a declared row, every
        // file-register index by construction — because a generated file is not a source of
        // citations. The set includes the extensions' files once `extension::configure` has
        // run over the manifest.
        let mut committed = HashMap::new();
        for rel in crate::index::generated_paths(manifest, model.listing()) {
            if let Ok(text) = std::fs::read_to_string(manifest.root().join(&rel)) {
                committed.insert(rel, text);
            }
        }
        // A register instance's options sit beside it and are not markdown, so the walk never
        // reads them.
        let anchors = Anchors::of(manifest, model.listing());
        let mut configs = HashMap::new();
        for (_, _, home) in anchors.instances() {
            if let Ok(text) = std::fs::read_to_string(manifest.root().join(&home.config)) {
                configs.insert(home.config.clone(), text);
            }
        }
        // One listing answers every question a check has about what is there.
        let survey = crate::survey::survey(manifest, model).map_err(|e| e.to_string())?;
        // Whether the ignore rules cover a path target is git's answer, taken in ONE batch over
        // every spelling a reference in this run could ask about: a process per reference
        // would be a process per pointer in the tree.
        let queries = crate::check::references::ignore_queries(model, &anchors);
        let ignored = crate::git::ignored(manifest.root(), &queries).map_err(|e| e.to_string())?;
        let tracked_and_ignored =
            crate::git::tracked_and_ignored(manifest.root()).map_err(|e| e.to_string())?;
        let shipped = crate::agents::shipped(manifest);
        Ok(Gathered {
            committed,
            configs,
            survey,
            ignored,
            tracked_and_ignored,
            shipped,
        })
    }

    /// The inputs of one run, borrowed from what was gathered.
    pub fn inputs(&self) -> Inputs<'_> {
        Inputs {
            committed: &self.committed,
            configs: &self.configs,
            present: &self.survey.present,
            directories: &self.survey.directories,
            outside: &self.survey.outside,
            ignored: &self.ignored,
            tracked_and_ignored: &self.tracked_and_ignored,
            refused: &self.survey.refused,
            links: &self.survey.links,
            installed: &self.survey.installed,
            shipped: &self.shipped,
        }
    }
}
