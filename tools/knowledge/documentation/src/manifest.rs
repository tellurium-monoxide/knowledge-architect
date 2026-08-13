//! `knowledge.toml` — what a project declares must stay conformant, and what is exempt.
//!
//! Nothing about any particular repository is compiled into this tool. Every list a check
//! reads comes from here, so the same binary checks this repository and a mock project under
//! `tests/projects/` with no special case anywhere, and a path that should not be checked has
//! to say so in one file with a reason beside it.
//!
//! The file's presence is also what makes a directory a project root. That is one mechanism
//! rather than two: a directory either declares itself a project or it does not, and the tool
//! refuses to run outside one instead of guessing a root from its own location.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The file that both marks a project root and declares its conformance surface.
pub const MANIFEST_NAME: &str = "knowledge.toml";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Walk {
    pub suffixes: Vec<String>,
    pub skip_dirs: Vec<String>,
    pub skip_files: Vec<String>,
    /// Project-relative paths skipped by location rather than by name.
    #[serde(default)]
    pub exclude: Vec<PathBuf>,
}

#[cfg(test)]
impl Walk {
    /// A walk configuration for a unit test that only needs to know which suffixes carry
    /// comment leaders. It is not this project's — a test that cares about this project's
    /// declaration reads the real manifest.
    pub fn sample() -> Self {
        Self {
            suffixes: ["md", "rs", "py", "sh", "toml"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            skip_dirs: vec![".git".to_string()],
            skip_files: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Lint {
    /// Exempt from the missing-marker lint only. Quotes in these files are still verified.
    #[serde(default)]
    pub exempt_files: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Rules {
    pub dir: PathBuf,
    pub text: PathBuf,
    pub body_starts_at: usize,
    pub version: PathBuf,
    pub past: PathBuf,
    pub manifest: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Interpretations {
    pub dir: PathBuf,
    /// The concerns the register is partitioned by, one file each.
    ///
    /// Listed rather than derived from anything. A partition read off another artifact
    /// cannot diverge from it without editing that artifact, and here that artifact is the
    /// frozen survey. The check compares this list against the directory in both directions,
    /// so a file with no entry and an entry with no file are both failures.
    pub concerns: Vec<String>,
}

/// Every directory that carries tracker files, mapped to the files it carries.
///
/// Listed rather than discovered by filename. A walk that recognised a tracker by its name
/// cannot tell one from a stray file, so a tracker in an unregistered directory would join
/// `outstanding` silently and a directory that lost one would be silently absent from the
/// report. Registering both facts makes each of them a diff.
///
/// A `BTreeMap` rather than a `HashMap`: findings are reported in the order this is walked,
/// and a hash order would reshuffle them between runs.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Trackers(pub BTreeMap<PathBuf, Vec<String>>);

impl Trackers {
    /// Directory and its tracker files, in path order.
    pub fn iter(&self) -> impl Iterator<Item = (&PathBuf, &Vec<String>)> {
        self.0.iter()
    }

    /// The project-relative path of every tracker file the project declares.
    pub fn paths(&self) -> Vec<PathBuf> {
        self.0
            .iter()
            .flat_map(|(dir, names)| names.iter().map(|n| dir.join(n)))
            .collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct Declared {
    walk: Walk,
    lint: Lint,
    rules: Rules,
    interpretations: Interpretations,
    trackers: Trackers,
}

/// A project: where its root is, and what it declares.
#[derive(Debug, Clone)]
pub struct Manifest {
    root: PathBuf,
    declared: Declared,
}

impl Manifest {
    /// The nearest project root at or above `start`, and its manifest.
    ///
    /// Walking up rather than reading a compiled-in path is what lets the tool be pointed at
    /// a mock project: the mock carries its own manifest and is therefore its own root.
    pub fn find(start: &Path) -> Result<Self, String> {
        let root = start
            .ancestors()
            .find(|dir| dir.join(MANIFEST_NAME).is_file())
            .ok_or_else(|| {
                format!(
                    "no {MANIFEST_NAME} at or above {}\n       \
                     a project declares its conformance surface in that file; \
                     run this from inside one",
                    start.display()
                )
            })?;
        Self::load(root)
    }

    /// Read a manifest from a known root.
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join(MANIFEST_NAME);
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let declared: Declared =
            toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(Self {
            root: root.to_path_buf(),
            declared,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn walk(&self) -> &Walk {
        &self.declared.walk
    }

    pub fn lint(&self) -> &Lint {
        &self.declared.lint
    }

    pub fn rules(&self) -> &Rules {
        &self.declared.rules
    }

    pub fn interpretations(&self) -> &Interpretations {
        &self.declared.interpretations
    }

    pub fn trackers(&self) -> &Trackers {
        &self.declared.trackers
    }

    /// The rules corpus as `rules::Tree` needs it, with every path already resolved.
    ///
    /// The `rules` library is handed explicit paths rather than the manifest, so it keeps
    /// knowing nothing about how a project is laid out or where its declaration lives.
    pub fn rules_tree(&self) -> rules::Tree {
        let dir = self.root.join(&self.declared.rules.dir);
        rules::Tree::new(
            &self.root,
            dir.join(&self.declared.rules.text),
            dir.join(&self.declared.rules.version),
            dir.join(&self.declared.rules.past),
            dir.join(&self.declared.rules.manifest),
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// The manifest of the checkout these tests were compiled from.
    ///
    /// `CARGO_MANIFEST_DIR` is legitimate here and nowhere else: it is baked in at compile
    /// time, which is wrong for a binary and exactly right for a test that only ever runs
    /// against the tree it was built from.
    pub fn this_project() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .expect("documentation/ sits three levels below the project root")
            .to_path_buf()
    }

    #[test]
    fn this_repository_declares_a_readable_manifest() {
        let m = Manifest::load(&this_project()).expect("knowledge.toml");
        assert!(m.walk().suffixes.contains(&"md".to_string()));
        assert!(m
            .trackers()
            .paths()
            .iter()
            .any(|p| p.ends_with("open-issues.md")));
        assert!(m.rules_tree().text().is_file());
    }

    #[test]
    fn the_walk_up_finds_the_root_from_below_it() {
        let root = this_project();
        let deep = root.join("tools/knowledge/documentation/src");
        assert_eq!(Manifest::find(&deep).expect("a manifest").root(), root);
    }

    #[test]
    fn a_directory_outside_any_project_is_an_error_and_says_what_is_missing() {
        let e = Manifest::find(Path::new("/")).expect_err("no project at /");
        assert!(e.contains(MANIFEST_NAME), "{e}");
    }

    #[test]
    fn an_unknown_key_is_rejected_rather_than_ignored() {
        // A misspelled key that parsed and did nothing would be an exclusion silently not
        // applied, which is the failure mode this whole file exists to make impossible.
        let dir = std::env::temp_dir().join("knowledge-manifest-unknown-key");
        std::fs::create_dir_all(&dir).expect("a temp dir");
        std::fs::write(dir.join(MANIFEST_NAME), "[walk]\nsuffix = []\n").expect("write");
        let e = Manifest::load(&dir).expect_err("unknown key");
        assert!(e.contains("suffix"), "{e}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
