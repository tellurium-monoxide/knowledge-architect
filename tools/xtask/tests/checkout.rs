//! Every build of this workspace is tied to its checkout, per
//! `design@knowledge-architect@a-build-is-tied-to-its-checkout`: the cargo configuration sets a
//! variable to the checkout's root, and every target reads it or links a library that does.

use std::path::{Path, PathBuf};
use std::process::Command;

// This target links no library of its package, so it reads the variable itself.
const CHECKOUT: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// What a crate root holds when it reads the variable.
const READ: &str = "option_env!(\"KNOWLEDGE_ARCHITECT_CHECKOUT\")";

/// What a build script holds when its run follows the variable.
const RERUN: &str = "cargo:rerun-if-env-changed=KNOWLEDGE_ARCHITECT_CHECKOUT";

/// The workspace root: this crate sits at tools/xtask.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root exists")
}

#[test]
fn the_compiled_value_is_this_checkouts_root() {
    let value = CHECKOUT.expect("the cargo configuration sets the variable");
    let value = Path::new(value)
        .canonicalize()
        .expect("the value names a directory");
    assert_eq!(value, workspace_root());
}

#[test]
fn every_workspace_target_reads_the_variable_or_links_a_library_that_does() {
    let root = workspace_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(root.join("Cargo.toml"))
        .output()
        .expect("cargo runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("cargo metadata prints JSON");

    let mut untied = Vec::new();
    let packages = metadata["packages"].as_array().expect("a package list");
    assert!(!packages.is_empty(), "the workspace lists no package");
    for package in packages {
        let targets = package["targets"].as_array().expect("a target list");
        let kinds = |target: &serde_json::Value| -> Vec<String> {
            target["kind"]
                .as_array()
                .expect("a kind list")
                .iter()
                .map(|k| k.as_str().expect("a kind").to_owned())
                .collect()
        };
        let is_lib = |kinds: &[String]| {
            kinds
                .iter()
                .any(|k| k == "lib" || k == "rlib" || k == "proc-macro")
        };
        // Cargo makes a package's library a dependency of each of its other targets, used or not,
        // so a rebuilt library rebuilds them.
        let has_lib = targets.iter().any(|t| is_lib(&kinds(t)));
        for target in targets {
            let kinds = kinds(target);
            let source = target["src_path"].as_str().expect("a source path");
            let text = std::fs::read_to_string(source).expect("a target's root is readable");
            let tied = if kinds.iter().any(|k| k == "custom-build") {
                text.contains(RERUN)
            } else if is_lib(&kinds) || !has_lib {
                text.contains(READ)
            } else {
                true
            };
            if !tied {
                untied.push(format!(
                    "{} {} ({source})",
                    package["name"].as_str().expect("a package name"),
                    kinds.join(",")
                ));
            }
        }
    }
    assert!(untied.is_empty(), "untied targets: {untied:#?}");
}
