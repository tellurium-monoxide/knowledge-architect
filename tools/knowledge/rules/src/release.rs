//! Releases of the Comprehensive Rules: the pin, the archive, and resolving one to a file.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

/// Where one project keeps its rules corpus.
///
/// Every path is supplied by the caller. This library knows that a corpus has a pinned text,
/// a version, an archive and a provenance manifest; it does not know that they sit under
/// `docs/rules`, which is a fact about a project's layout and lives in that project's
/// manifest. Handing it explicit paths is what lets the same code check a mock project whose
/// layout differs.
#[derive(Clone, Debug)]
pub struct Tree {
    root: PathBuf,
    text: PathBuf,
    version: PathBuf,
    past: PathBuf,
    manifest: PathBuf,
}

impl Tree {
    pub fn new(
        root: impl AsRef<Path>,
        text: PathBuf,
        version: PathBuf,
        past: PathBuf,
        manifest: PathBuf,
    ) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            text,
            version,
            past,
            manifest,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The vendored release: the one every live quote is checked against.
    pub fn text(&self) -> &Path {
        &self.text
    }

    pub fn version(&self) -> &Path {
        &self.version
    }

    pub fn past(&self) -> &Path {
        &self.past
    }

    pub fn manifest(&self) -> &Path {
        &self.manifest
    }
}

/// A `key: value` file, one pair per line, splitting on the first colon only.
///
/// The first colon is what matters: a `source:` line holds a URL, whose scheme colon would
/// otherwise split the value in half.
pub fn read_version(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

pub fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// One archived release's provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchivedRelease {
    pub url: String,
    pub sha256: String,
    pub fetched: String,
}

/// `date -> provenance` for every superseded release held in `past/`.
///
/// This is what lets a fetched release be checked against the bytes Wizards published, which
/// the whole citation regime assumes: a download nobody checked is not evidence.
pub fn read_manifest(text: &str) -> HashMap<String, ArchivedRelease> {
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#') && !l.starts_with("date\t"))
        .filter_map(|line| {
            let mut cols = line.split('\t');
            let date = cols.next()?.trim().to_string();
            let mut col = || cols.next().unwrap_or("").trim().to_string();
            Some((
                date,
                ArchivedRelease {
                    url: col(),
                    sha256: col(),
                    fetched: col(),
                },
            ))
        })
        .collect()
}

pub fn url_for(date: &str) -> String {
    format!(
        "https://media.wizards.com/{}/downloads/MagicCompRules%20{date}.txt",
        &date[..4]
    )
}

/// A release already in the tree: the vendored text, or the archive. `None` otherwise.
///
/// Separate from [`resolve`] so the question can be asked without taking the fetching
/// branch. The structural check that every release the tooling needs is already present
/// asks exactly this, and asking it through [`resolve`] would answer it by fetching, which
/// is the thing being asserted against.
pub fn local(tree: &Tree, date: &str) -> Option<PathBuf> {
    let pinned = std::fs::read_to_string(tree.version())
        .ok()
        .and_then(|t| read_version(&t).get("date").cloned());
    if pinned.as_deref() == Some(date) {
        return Some(tree.text().to_path_buf());
    }
    let archived = tree.past().join(format!("{date}.txt"));
    archived.exists().then_some(archived)
}

/// A release as a local path: the vendored text, the archive, or a verified fetch.
///
/// The archive comes before the network on purpose, because Wizards rotates its download
/// URLs. A fetch is checked against the manifest's digest where the manifest knows the
/// release, and a release the manifest does not know is reported as unverified rather than
/// silently trusted.
pub fn resolve(tree: &Tree, date: &str) -> Result<PathBuf, String> {
    if let Some(path) = local(tree, date) {
        return Ok(path);
    }
    let cache = std::env::temp_dir().join(format!("MagicCompRules-{date}.txt"));
    if !cache.exists() {
        let status = Command::new("curl")
            .args(["-fsSL", "-o"])
            .arg(&cache)
            .arg(url_for(date))
            .status()
            .map_err(|e| format!("{date}: could not run curl: {e}"))?;
        if !status.success() {
            return Err(format!("{date}: curl failed ({status})"));
        }
    }
    let expected = std::fs::read_to_string(tree.manifest())
        .ok()
        .and_then(|t| read_manifest(&t).get(date).map(|r| r.sha256.clone()));
    match expected {
        Some(want) => {
            let got = sha256(&std::fs::read(&cache).map_err(|e| e.to_string())?);
            if got != want {
                let _ = std::fs::remove_file(&cache);
                return Err(format!(
                    "{date}: fetched text does not match the recorded sha256\n  \
                     expected {want}\n  got      {got}"
                ));
            }
        }
        None => eprintln!("warning: {date} is not in MANIFEST.tsv; fetched text is unverified"),
    }
    Ok(cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_version_value_keeps_its_own_colons() {
        let v = read_version("date:   20260807\nsource: https://example.test/a%20b.txt\n");
        assert_eq!(v["date"], "20260807");
        assert_eq!(v["source"], "https://example.test/a%20b.txt");
    }

    #[test]
    fn the_manifest_skips_its_header_blanks_and_comments() {
        let m = read_manifest(
            "date\turl\tsha256\tfetched\n\
             # a comment\n\
             \n\
             20260101\thttps://e.test/a.txt\tabc123\t2026-01-02\n",
        );
        assert_eq!(m.len(), 1);
        assert_eq!(m["20260101"].sha256, "abc123");
        assert_eq!(m["20260101"].fetched, "2026-01-02");
    }

    #[test]
    fn a_short_manifest_row_does_not_lose_the_date() {
        // A hand-edited row missing its trailing columns still names an archived release,
        // and dropping it would make the archive check report the file as unrecorded.
        let m = read_manifest("20260101\thttps://e.test/a.txt\n");
        assert_eq!(m["20260101"].url, "https://e.test/a.txt");
        assert_eq!(m["20260101"].sha256, "");
    }

    #[test]
    fn the_download_url_takes_its_year_from_the_date() {
        assert_eq!(
            url_for("20260807"),
            "https://media.wizards.com/2026/downloads/MagicCompRules%2020260807.txt"
        );
    }

    #[test]
    fn sha256_matches_the_known_digest_of_the_empty_input() {
        assert_eq!(
            sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn the_vendored_release_resolves_locally_and_an_unknown_one_does_not() {
        let tree = crate::testing::this_tree();
        let pinned = read_version(&std::fs::read_to_string(tree.version()).expect("VERSION"))
            .get("date")
            .expect("a pinned date")
            .clone();
        assert_eq!(local(&tree, &pinned), Some(tree.text().to_path_buf()));
        // Nothing may reach the network to answer this.
        assert_eq!(local(&tree, "19700101"), None);
    }
}
