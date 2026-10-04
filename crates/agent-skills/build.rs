//! Generates the list of shipped files from the content/ directory, so no hand-written list can
//! drift from what the directory holds.
//!
//! content/ mirrors the install layout under the owned namespace, with the prefix added on the
//! way out. Any other file under content/ fails the build: a file the list would not ship has no
//! reason to be in the package.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The prefix every installed skill directory and agent file carries in the project.
const PREFIX: &str = "knowledge-architect-";

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("set by cargo"));
    let content = root.join("content");
    // A directory makes cargo scan everything under it for a change.
    println!("cargo:rerun-if-changed=content");
    // The list names each file by its absolute path, so a run from another checkout is stale
    // even when no file here changed: the run follows the checkout, per
    // `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
    println!("cargo:rerun-if-env-changed=KNOWLEDGE_ARCHITECT_CHECKOUT");

    let mut files = Vec::new();
    if content.is_dir() {
        walk(&content, &mut files);
    }
    let mut entries: Vec<(String, PathBuf)> = files
        .into_iter()
        .map(|file| {
            let rel = file.strip_prefix(&content).expect("under content/");
            let install = install_path(rel).unwrap_or_else(|| {
                panic!(
                    "content/{} has no install path: content/ holds skills/<skill>/…, \
                     agents/<agent>.md and PRIMER.md, and nothing else",
                    rel.display()
                )
            });
            (install, file)
        })
        .collect();
    entries.sort();

    let mut out = String::from("&[\n");
    for (install, file) in &entries {
        let file = file.to_str().expect("a UTF-8 path");
        writeln!(out, "    ({install:?}, include_str!({file:?})),").expect("a String");
    }
    out.push(']');
    let dest = PathBuf::from(std::env::var("OUT_DIR").expect("set by cargo")).join("files.rs");
    std::fs::write(dest, out).expect("OUT_DIR is writable");
}

/// Every file under `dir`, recursively.
fn walk(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("content/ is readable") {
        let path = entry.expect("content/ is readable").path();
        if path.is_dir() {
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// The install path, relative to the project root, of a file at `rel` under content/.
fn install_path(rel: &Path) -> Option<String> {
    let parts: Vec<&str> = rel.iter().map(|c| c.to_str()).collect::<Option<_>>()?;
    match parts.as_slice() {
        ["PRIMER.md"] => Some(".claude/knowledge-architect/PRIMER.md".to_owned()),
        ["skills", skill, rest @ ..] if !rest.is_empty() => {
            Some(format!(".claude/skills/{PREFIX}{skill}/{}", rest.join("/")))
        }
        ["agents", agent] if agent.ends_with(".md") => {
            Some(format!(".claude/agents/{PREFIX}{agent}"))
        }
        _ => None,
    }
}
