//! Releases of the Comprehensive Rules: the pin, the archive, and resolving one to a file.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

/// Where one project keeps its rules corpus.
///
/// Every path is supplied by the caller. This library knows that a corpus has a pinned text,
/// a version, an archive and a provenance manifest; it does not know that they sit under
/// `thaum@docs/rules/`, which is a fact about a project's layout and lives in that project's
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

/// A published file, folded to the form this tree vendors: no UTF-8 BOM, LF line endings.
///
/// Wizards publishes the text as UTF-8 with a BOM and CRLF terminators. Neither carries
/// content, and vendoring them would put every line of the file in a diff on every bump — so
/// every download is folded here before anything stores or digests it. Every recorded sha256,
/// in the version file and in the archive manifest, is of the folded text, and [`resolve`]
/// folds a fetch before comparing it against one. Nothing else is touched: a CR not followed
/// by LF is content and is kept, and so is every other byte, because the vendored text is
/// what citations quote verbatim.
///
/// Folding repeats until stable, so the function is idempotent — [`resolve`] folds whatever
/// its cache holds, and a fold that is not idempotent would fail the digest of a cache it had
/// itself verified, then delete it. Stability costs a byte on the pathological run `\r\r\n`,
/// which collapses to one LF across the passes; no published release has carried one.
pub fn normalize(published: &[u8]) -> Vec<u8> {
    let mut cur = published.to_vec();
    loop {
        let body = cur.strip_prefix(b"\xef\xbb\xbf".as_slice()).unwrap_or(&cur);
        let mut out = Vec::with_capacity(body.len());
        let mut i = 0;
        while i < body.len() {
            if body[i] == b'\r' && body.get(i + 1) == Some(&b'\n') {
                out.push(b'\n');
                i += 2;
            } else {
                out.push(body[i]);
                i += 1;
            }
        }
        if out == cur {
            return out;
        }
        cur = out;
    }
}

/// The "effective as of" line near the top of a release, or `None` when the layout moved.
///
/// Two releases can differ as files while carrying the same effective date: Wizards
/// re-exports a release with new typography under a new URL date. Comparing the two lines at
/// bump time is what shows that case before anyone reads a diff.
pub fn effective_as_of(text: &str) -> Option<&str> {
    text.lines()
        .take(10)
        .map(str::trim)
        .find(|l| l.contains("effective as of"))
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

/// How `resolve` reports the bytes it is about to return.
///
/// Kept apart from the printing so the wording is checkable. Two properties it carries: the
/// digest is reported in both cases, because the manifest records only superseded releases
/// and a bump target therefore has no row against which anything could be checked; and the
/// source is named, because an existing cache is returned without any download, so a line
/// asserting a fetch would be false exactly where the bytes are least trustworthy.
fn provenance(date: &str, digest: &str, fetched: bool, known: bool) -> String {
    let source = if fetched {
        "downloaded"
    } else {
        "read from a cache an earlier run left behind"
    };
    if known {
        format!("{date}: sha256 {digest} ({source}), matching MANIFEST.tsv")
    } else {
        format!(
            "warning: {date} is not in MANIFEST.tsv, so sha256 {digest} ({source}) \
             is checked against nothing"
        )
    }
}

/// A release as a local path: the vendored text, the archive, or a fetch reported by digest.
///
/// The archive comes before the network on purpose, because Wizards rotates its download
/// URLs. A fetch is refused unless it matches the manifest's digest, where the manifest knows
/// the release. It does not know a bump target by construction, so nothing verifies those
/// bytes and the digest is printed instead: it is the only record of which text a bump was
/// built from.
///
/// The cache path is shared and predictable, so a file left there by an earlier run — or by
/// anyone else — is returned without a download. What that costs and what would close it is
/// `knowledge@docs/open-issues.md`.
pub fn resolve(tree: &Tree, date: &str) -> Result<PathBuf, String> {
    if let Some(path) = local(tree, date) {
        return Ok(path);
    }
    let cache = std::env::temp_dir().join(format!("MagicCompRules-{date}.txt"));
    let fetched = !cache.exists();
    if fetched {
        let status = Command::new("curl")
            .args(["-fsSL", "-o"])
            .arg(&cache)
            .arg(url_for(date))
            .status()
            .map_err(|e| format!("{date}: could not run curl: {e}"))?;
        if !status.success() {
            // A failed download can leave a partial file, and `cache.exists()` would trust
            // it on the next run — for a bump target no digest exists to catch that.
            let _ = std::fs::remove_file(&cache);
            return Err(format!("{date}: curl failed ({status})"));
        }
    }
    // Fold whatever the cache holds: a fresh download is the publisher's raw bytes, and a
    // cache written before folding existed is too. Idempotent, so a folded cache is untouched.
    let raw = std::fs::read(&cache).map_err(|e| e.to_string())?;
    let folded = normalize(&raw);
    if folded != raw {
        std::fs::write(&cache, &folded).map_err(|e| e.to_string())?;
    }
    let expected = std::fs::read_to_string(tree.manifest())
        .ok()
        .and_then(|t| read_manifest(&t).get(date).map(|r| r.sha256.clone()));
    // The digest is taken whether or not a row exists to compare it against, so that the run
    // always says which bytes it used.
    let got = sha256(&folded);
    match &expected {
        Some(want) if *want != got => {
            let _ = std::fs::remove_file(&cache);
            return Err(format!(
                "{date}: fetched text does not match the recorded sha256\n  \
                 expected {want}\n  got      {got}"
            ));
        }
        _ => {}
    }
    eprintln!("{}", provenance(date, &got, fetched, expected.is_some()));
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
    fn normalizing_strips_the_bom_and_folds_crlf_and_nothing_else() {
        assert_eq!(normalize(b"\xef\xbb\xbfa\r\nb\rc\n"), b"a\nb\rc\n");
        // Idempotent: text already in the vendored form passes through byte for byte.
        assert_eq!(normalize(b"a\nb\rc\n"), b"a\nb\rc\n");
    }

    #[test]
    fn normalizing_is_idempotent_even_on_a_cr_run() {
        // The adversarial shape: one pass over `\r\r\n` leaves a CRLF behind, and a fold
        // that is not idempotent fails the digest of a cache it had itself verified. The
        // fold runs to a fixpoint instead, at the recorded cost of the lone CR in the run.
        let once = normalize(b"a\r\r\nb\n");
        assert_eq!(once, b"a\nb\n");
        assert_eq!(normalize(&once), once);
    }

    #[test]
    fn a_digest_mismatch_refuses_the_bytes_and_clears_the_cache() {
        // The refusal is the property the manifest exists for; a mutant that disables it
        // must fail here, not survive the suite.
        const DATE: &str = "19980102";
        let dir = std::env::temp_dir().join("knowledge-release-mismatch-test");
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let manifest = dir.join("MANIFEST.tsv");
        std::fs::write(
            &manifest,
            format!(
                "{DATE}\thttps://e.test/r.txt\t{}\t2026-01-01\n",
                sha256(b"what the manifest recorded\n")
            ),
        )
        .expect("a manifest row");
        let tree = Tree::new(
            &dir,
            dir.join("MagicCompRules.txt"),
            dir.join("VERSION"),
            dir.join("past"),
            manifest,
        );
        let cache = std::env::temp_dir().join(format!("MagicCompRules-{DATE}.txt"));
        std::fs::write(&cache, b"something else entirely\n").expect("a seeded cache");
        let err = resolve(&tree, DATE).expect_err("the digests differ");
        assert!(err.contains("does not match"), "{err}");
        assert!(!cache.exists(), "the refused cache is cleared");
    }

    #[test]
    fn a_fetch_is_folded_before_the_digest_is_checked() {
        // A scratch corpus whose manifest records the digest of the FOLDED text, and a cache
        // seeded with the raw BOM+CRLF bytes Wizards would serve. Resolving must fold before
        // comparing, or the digest check refuses the exact bytes the publisher sends.
        const DATE: &str = "19980101";
        let folded = b"the rules text\n";
        let dir = std::env::temp_dir().join("knowledge-release-fold-test");
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let manifest = dir.join("MANIFEST.tsv");
        std::fs::write(
            &manifest,
            format!(
                "{DATE}\thttps://e.test/r.txt\t{}\t2026-01-01\n",
                sha256(folded)
            ),
        )
        .expect("a manifest row");
        let tree = Tree::new(
            &dir,
            dir.join("MagicCompRules.txt"),
            dir.join("VERSION"),
            dir.join("past"),
            manifest,
        );
        let cache = std::env::temp_dir().join(format!("MagicCompRules-{DATE}.txt"));
        std::fs::write(&cache, b"\xef\xbb\xbfthe rules text\r\n").expect("a seeded cache");
        let path = resolve(&tree, DATE).expect("the folded digest matches");
        assert_eq!(std::fs::read(&path).expect("the resolved file"), folded);
        let _ = std::fs::remove_file(&cache);
    }

    #[test]
    fn the_provenance_line_names_the_digest_and_no_fetch_that_did_not_happen() {
        // The line is the only record of which bytes a bump was built from, so it must name
        // the digest in both cases, and must not report a download when the cache answered.
        let cached = provenance("20260901", "abc123", false, false);
        assert!(cached.contains("abc123"), "{cached}");
        assert!(cached.contains("cache"), "{cached}");
        assert!(!cached.contains("downloaded"), "{cached}");
        assert!(cached.contains("checked against nothing"), "{cached}");

        let known = provenance("20260807", "def456", true, true);
        assert!(known.contains("def456"), "{known}");
        assert!(known.contains("downloaded"), "{known}");
        assert!(known.contains("matching MANIFEST.tsv"), "{known}");
    }

    #[test]
    fn a_release_the_manifest_does_not_know_resolves_from_whatever_the_cache_holds() {
        // Pins the reproduction the open issue names: with no row to compare against, the
        // seeded bytes come back. The digest is reported, not verified — so a change that
        // starts verifying them has to edit this test, which is the point of having it.
        const DATE: &str = "19980103";
        let dir = std::env::temp_dir().join("knowledge-release-unknown-test");
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let manifest = dir.join("MANIFEST.tsv");
        std::fs::write(&manifest, "date\turl\tsha256\tfetched\n").expect("an empty manifest");
        let tree = Tree::new(
            &dir,
            dir.join("MagicCompRules.txt"),
            dir.join("VERSION"),
            dir.join("past"),
            manifest,
        );
        let cache = std::env::temp_dir().join(format!("MagicCompRules-{DATE}.txt"));
        std::fs::write(&cache, b"arbitrary bytes\n").expect("a seeded cache");
        let path = resolve(&tree, DATE).expect("an unknown release resolves from the cache");
        assert_eq!(
            std::fs::read(&path).expect("the resolved file"),
            b"arbitrary bytes\n"
        );
        let _ = std::fs::remove_file(&cache);
    }

    #[test]
    fn the_effective_line_is_read_off_the_top_and_absence_is_none() {
        let text = "\u{feff}Magic Rules\n \nThese rules are effective as of August 7, 2026.\n";
        assert_eq!(
            effective_as_of(text),
            Some("These rules are effective as of August 7, 2026.")
        );
        assert_eq!(effective_as_of("no such line anywhere\n"), None);
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
