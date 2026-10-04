//! The `changelog` subcommand: write the root CHANGELOG.md into every published crate.
//!
//! A crate's `include` cannot reach outside its directory, so each published crate carries a copy
//! of the root changelog, byte for byte, per
//! `design@knowledge-architect@the-changelog-ships-in-every-crate`. This writes the copies, and the test
//! below fails while any copy differs.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::process::{complain, say};
use knowledge_architect_gates::project_root;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// The changelog's file name, at the root and in each crate.
const CHANGELOG: &str = "CHANGELOG.md";

/// Where each copy goes: one per directory under `crates/` holding a `Cargo.toml`, since every
/// crate there is published and nothing else is, per
/// `design@knowledge-architect@package-include-whitelist`.
fn copies(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(root.join("crates"))? {
        let dir = entry?.path();
        if dir.join("Cargo.toml").is_file() {
            out.push(dir.join(CHANGELOG));
        }
    }
    out.sort();
    Ok(out)
}

/// Write the root changelog into each copy whose bytes differ, and say which were written.
///
/// It writes only where the bytes differ, so running it to look costs nothing and moves no
/// file's modification time.
fn sync(root: &Path) -> std::io::Result<Vec<(PathBuf, bool)>> {
    let text = std::fs::read(root.join(CHANGELOG))?;
    let mut out = Vec::new();
    for copy in copies(root)? {
        let current = std::fs::read(&copy).ok().as_deref() == Some(text.as_slice());
        if !current {
            std::fs::write(&copy, &text).map_err(|error| {
                std::io::Error::new(error.kind(), format!("{}: {error}", copy.display()))
            })?;
        }
        out.push((copy, !current));
    }
    Ok(out)
}

pub fn run() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(error) => return abort(&format!("no working directory: {error}")),
    };
    let Some(root) = project_root(&cwd, MANIFEST_NAME) else {
        return abort(&format!(
            "not inside the project: no ancestor holds {MANIFEST_NAME}"
        ));
    };
    match sync(&root) {
        Ok(written) => {
            for (copy, rewritten) in written {
                let rel = copy.strip_prefix(&root).unwrap_or(&copy);
                let what = if rewritten {
                    "rewritten"
                } else {
                    "already current"
                };
                say(&format!("{:<40} {what}\n", rel.display()));
            }
            ExitCode::SUCCESS
        }
        Err(error) => abort(&format!(
            "the changelog copies could not be written: {error}"
        )),
    }
}

fn abort(message: &str) -> ExitCode {
    complain(&format!("xtask: {message}"));
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This checkout's root: this crate sits at tools/xtask.
    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("the workspace root exists")
    }

    // The claim: every published crate lists the changelog in its `include` and carries the root
    // changelog byte for byte. This is the check that fails a branch whose changelog entry was not
    // copied, or whose crate stopped shipping the file.
    #[test]
    fn every_published_crate_carries_the_root_changelog() {
        let root = workspace_root();
        let text = std::fs::read(root.join(CHANGELOG)).expect("the root changelog");
        let copies = copies(&root).expect("crates/ is readable");
        assert_eq!(copies.len(), 3, "{copies:?}");
        for copy in copies {
            let manifest = copy.with_file_name("Cargo.toml");
            let manifest: toml::Table = std::fs::read_to_string(&manifest)
                .expect("the crate's Cargo.toml")
                .parse()
                .expect("the crate's Cargo.toml is TOML");
            let include = manifest["package"]["include"]
                .as_array()
                .expect("the crate declares `include`");
            assert!(
                include.iter().any(|i| i.as_str() == Some("/CHANGELOG.md")),
                "{} does not list /CHANGELOG.md in `include`",
                copy.with_file_name("Cargo.toml").display()
            );
            assert!(
                std::fs::read(&copy).ok().as_deref() == Some(text.as_slice()),
                "{} differs from the root CHANGELOG.md; run `cargo x changelog`",
                copy.display()
            );
        }
    }

    // The claim: a missing or differing copy is written, a current one is left alone, and a
    // directory under crates/ with no Cargo.toml gets none. The current copy is read-only, so a
    // write to it fails the run.
    #[test]
    fn sync_writes_what_differs_and_only_that() {
        let dir = std::env::temp_dir().join(format!("xtask-changelog-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for krate in ["a", "b", "c"] {
            std::fs::create_dir_all(dir.join("crates").join(krate)).expect("a crate directory");
        }
        std::fs::create_dir_all(dir.join("crates/not-a-crate")).expect("a directory");
        for krate in ["a", "b", "c"] {
            std::fs::write(dir.join("crates").join(krate).join("Cargo.toml"), "")
                .expect("a manifest");
        }
        std::fs::write(dir.join(CHANGELOG), "new\n").expect("the root changelog");
        std::fs::write(dir.join("crates/b").join(CHANGELOG), "old\n").expect("a stale copy");
        let current = dir.join("crates/c").join(CHANGELOG);
        std::fs::write(&current, "new\n").expect("a current copy");
        let mut permissions = std::fs::metadata(&current).expect("the copy").permissions();
        permissions.set_readonly(true);
        std::fs::set_permissions(&current, permissions).expect("a read-only copy");

        let written = sync(&dir).expect("the copies are written");
        let flags: Vec<(String, bool)> = written
            .iter()
            .map(|(p, w)| {
                let rel = p.strip_prefix(&dir).expect("under the root");
                (rel.display().to_string(), *w)
            })
            .collect();
        assert_eq!(
            flags,
            vec![
                ("crates/a/CHANGELOG.md".to_string(), true),
                ("crates/b/CHANGELOG.md".to_string(), true),
                ("crates/c/CHANGELOG.md".to_string(), false),
            ]
        );
        for krate in ["a", "b", "c"] {
            let copy = std::fs::read_to_string(dir.join("crates").join(krate).join(CHANGELOG))
                .expect("a copy");
            assert_eq!(copy, "new\n");
        }
        assert!(!dir.join("crates/not-a-crate").join(CHANGELOG).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
