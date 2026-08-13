//! The corpus is what the project says it is.
//!
//! These are the only checks that read the filesystem themselves, and the reason is that
//! their subject IS filesystem state: whether the vendored bytes match the pin, whether every
//! archived release has provenance, whether a release the project needs is present rather
//! than a download away. There is no model of that to be handed; a check over the documents
//! is a different thing and stays pure.

use std::path::PathBuf;

use crate::release::{self, Tree};

/// Something wrong with the corpus, as a path and a sentence.
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub about: PathBuf,
    pub what: String,
    pub action: String,
}

/// What the corpus checks looked at, so that finding nothing is not the same as looking at
/// nothing.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub archived: usize,
    pub manifest_rows: usize,
    pub releases_needed: usize,
    pub releases_local: usize,
}

/// The pinned text is the bytes the version file names.
///
/// Everything else is verified against this file and nothing verified this file, so the
/// cheapest way to turn a failing citation green was to edit the specification.
fn vendored_matches_pin(tree: &Tree, out: &mut Vec<Problem>) {
    let Ok(version) = std::fs::read_to_string(tree.version()) else {
        out.push(Problem {
            about: tree.version().to_path_buf(),
            what: "the version file cannot be read".into(),
            action: "restore it: it is what the pinned text is checked against".into(),
        });
        return;
    };
    let declared = release::read_version(&version);
    let Ok(bytes) = std::fs::read(tree.text()) else {
        out.push(Problem {
            about: tree.text().to_path_buf(),
            what: "the pinned rules text cannot be read".into(),
            action: "fetch the release the version file names".into(),
        });
        return;
    };
    let got = release::sha256(&bytes);
    match declared.get("sha256") {
        Some(want) if *want == got => {}
        Some(want) => out.push(Problem {
            about: tree.text().to_path_buf(),
            what: format!(
                "sha256 {}… does not match the version file's {}…",
                &got[..16],
                &want[..16.min(want.len())]
            ),
            action: "the pinned specification has been edited — restore it, or bump \
                     deliberately and re-verify every citation"
                .into(),
        }),
        None => out.push(Problem {
            about: tree.version().to_path_buf(),
            what: "the version file records no sha256".into(),
            action: "record it: without it nothing verifies the specification itself".into(),
        }),
    }
}

/// Every archived release has a manifest row whose digest matches its bytes, and every row has
/// its file.
fn archive_is_accounted_for(tree: &Tree, counts: &mut Counts, out: &mut Vec<Problem>) {
    let manifest_text = std::fs::read_to_string(tree.manifest()).unwrap_or_default();
    let rows = release::read_manifest(&manifest_text);
    counts.manifest_rows = rows.len();

    let mut archived = Vec::new();
    if let Ok(entries) = std::fs::read_dir(tree.past()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "txt") {
                if let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().to_string()) {
                    archived.push((stem, path));
                }
            }
        }
    }
    archived.sort();
    counts.archived = archived.len();

    for (date, path) in &archived {
        match rows.get(date) {
            None => out.push(Problem {
                about: path.clone(),
                what: format!("the archived release {date} has no row in the manifest"),
                action: "record its url, digest and fetch date: a download nobody checked is \
                         not evidence"
                    .into(),
            }),
            Some(row) => {
                let got = std::fs::read(path)
                    .map(|b| release::sha256(&b))
                    .unwrap_or_default();
                if got != row.sha256 {
                    out.push(Problem {
                        about: path.clone(),
                        what: format!(
                            "sha256 {}… does not match the manifest's {}…",
                            &got[..16.min(got.len())],
                            &row.sha256[..16.min(row.sha256.len())]
                        ),
                        action: "the archived bytes are not the ones recorded — re-fetch and \
                                 verify, or correct the row"
                            .into(),
                    });
                }
            }
        }
    }
    for date in rows.keys() {
        if !archived.iter().any(|(d, _)| d == date) {
            out.push(Problem {
                about: tree.past().join(format!("{date}.txt")),
                what: format!("the manifest has a row for {date} and the file is missing"),
                action: "commit the archived release, or remove the row".into(),
            });
        }
    }
}

/// Every release the project needs is already in the tree.
///
/// The property is that no checker reaches the network to do its job. A bump archives the
/// outgoing release, so it holds by construction; what it does not survive is a bump committed
/// without the archive.
fn needed_releases_are_local(
    tree: &Tree,
    needed: &[(String, String)],
    counts: &mut Counts,
    out: &mut Vec<Problem>,
) {
    counts.releases_needed = needed.len();
    for (date, who) in needed {
        if release::local(tree, date).is_some() {
            counts.releases_local += 1;
        } else {
            out.push(Problem {
                about: tree.past().join(format!("{date}.txt")),
                what: format!("{who} needs release {date}, which is neither vendored nor archived"),
                action: "commit the archived release: resolving it would otherwise reach the \
                         network, which is the thing this asserts against"
                    .into(),
            });
        }
    }
}

/// Run every corpus check. `needed` is `(release date, who needs it)`.
pub fn check(tree: &Tree, needed: &[(String, String)]) -> (Vec<Problem>, Counts) {
    let mut out = Vec::new();
    let mut counts = Counts::default();
    vendored_matches_pin(tree, &mut out);
    archive_is_accounted_for(tree, &mut counts, &mut out);
    needed_releases_are_local(tree, needed, &mut counts, &mut out);
    (out, counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_projects_corpus_is_intact() {
        let tree = crate::testing::this_tree();
        let version = std::fs::read_to_string(tree.version()).expect("VERSION");
        let pinned = release::read_version(&version)["date"].clone();
        let (problems, counts) = check(&tree, &[(pinned, "the pin".into())]);
        assert!(problems.is_empty(), "{problems:#?}");
        assert_eq!(counts.releases_local, 1);
        assert_eq!(counts.archived, counts.manifest_rows);
    }

    #[test]
    fn a_release_that_is_neither_vendored_nor_archived_is_reported() {
        let tree = crate::testing::this_tree();
        let (problems, counts) = check(&tree, &[("19700101".into(), "a test".into())]);
        assert_eq!(counts.releases_local, 0);
        assert!(
            problems.iter().any(|p| p.what.contains("19700101")),
            "{problems:#?}"
        );
    }
}
