//! The core's own binary, with no extension registered, per
//! `design@knowledge@the-core-cli-is-a-library-module`.
//!
//! The tests of every command over the mock projects run the combined binary and live in
//! `path@rules-corpus@tests/`. What is asserted here is what only the core binary shows: it runs
//! over a project that declares nothing but what the core reads, and it refuses a table no
//! extension of it claims, per `design@knowledge@an-extension-claims-its-manifest-tables`.

use std::path::{Path, PathBuf};
use std::process::Command;

fn mock() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/projects/core")
}

fn check(dir: &Path) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_knowledge"))
        .arg("check")
        .current_dir(dir)
        .output()
        .expect("the knowledge binary runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
    )
}

/// Copy a directory tree, files only.
fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a readable directory") {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a type").is_dir() {
            copy(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copy");
        }
    }
}

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?}");
}

#[test]
fn the_core_binary_passes_over_a_project_that_declares_only_what_the_core_reads() {
    let (code, out) = check(&mock());
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("checked: generated, registers, references\n"),
        "{out}"
    );
    assert!(out.ends_with("PASSED: no findings\n"), "{out}");
}

#[test]
fn the_core_binary_refuses_a_table_no_extension_of_it_claims() {
    // A copy, so the table is added to nothing another test reads. The walk is git's listing,
    // so the copy is its own repository.
    let dir = std::env::temp_dir().join(format!("knowledge-core-claims-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    copy(&mock(), &dir);
    let manifest = dir.join("knowledge.toml");
    let mut text = std::fs::read_to_string(&manifest).expect("the mock manifest");
    text.push_str(
        "\n[rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\nversion = \"v\"\n\
         past = \"p\"\nmanifest = \"m\"\n",
    );
    std::fs::write(&manifest, text).expect("the manifest written");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    let (code, out) = check(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("phase 1: 1 finding(s)"), "{out}");
    assert!(
        out.contains("[rules] is a table no extension of this binary reads"),
        "{out}"
    );
}
