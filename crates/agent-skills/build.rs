//! Generates the list of shipped files from the content/ directory, so no hand-written list can
//! drift from what the directory holds.
//!
//! content/ mirrors the install layout under the owned namespace, with the prefix added on the
//! way out. Any other file under content/ fails the build: a file the list would not ship has no
//! reason to be in the package.
//!
//! A line of shipped text that is a snippet placeholder, `{{snippet:<file>}}`, is replaced with
//! the file of that name under snippets/, so the text shipped is the text a workspace target
//! compiles. A placeholder naming no file, and a file no placeholder names, fail the build. The
//! placeholder is filled here rather than at install, where the project's command is filled,
//! because a snippet does not vary by project.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The prefix every installed skill directory and agent file carries in the project.
const PREFIX: &str = "knowledge-architect-";

/// What opens a snippet placeholder; `}}` closes it.
const SNIPPET: &str = "{{snippet:";

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("set by cargo"));
    let content = root.join("content");
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("set by cargo"));
    // A directory makes cargo scan everything under it for a change.
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=snippets");
    // The rendered copies are read from this checkout's files, so a run from another checkout is
    // stale even when no file here changed: the run follows the checkout, per
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

    let mut snippets = Snippets::read(&root.join("snippets"));
    let mut list = String::from("&[\n");
    for (install, file) in &entries {
        let text = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("{} is readable as UTF-8: {e}", file.display()));
        // The rendered copy sits under OUT_DIR at the install path, so the list still names one
        // file per entry and `include_str!` keeps the text out of the generated source.
        let rendered = out.join("rendered").join(install);
        std::fs::create_dir_all(rendered.parent().expect("an install path has a parent"))
            .expect("OUT_DIR is writable");
        std::fs::write(&rendered, snippets.render(&text, file)).expect("OUT_DIR is writable");
        let rendered = rendered.to_str().expect("a UTF-8 path");
        writeln!(list, "    ({install:?}, include_str!({rendered:?})),").expect("a String");
    }
    list.push(']');
    snippets.all_used();
    std::fs::write(out.join("files.rs"), list).expect("OUT_DIR is writable");
}

/// The files under snippets/, by name, each with whether a placeholder has named it.
struct Snippets(Vec<(String, String, bool)>);

impl Snippets {
    /// Every file directly under `dir`; none when the directory is absent. A directory under it
    /// fails the build: a snippet is one file.
    fn read(dir: &Path) -> Self {
        let mut snippets = Vec::new();
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir).expect("snippets/ is readable") {
                let path = entry.expect("snippets/ is readable").path();
                assert!(
                    !path.is_dir(),
                    "{} is a directory: snippets/ holds files only",
                    path.display()
                );
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .expect("a UTF-8 file name")
                    .to_owned();
                let text = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("{} is readable as UTF-8: {e}", path.display()));
                snippets.push((name, text, false));
            }
        }
        Snippets(snippets)
    }

    /// `text` with each line that is a placeholder replaced by its snippet. A snippet's final line
    /// break is the placeholder line's own, so the surrounding lines are unchanged.
    fn render(&mut self, text: &str, file: &Path) -> String {
        let mut rendered = String::with_capacity(text.len());
        for line in text.split_inclusive('\n') {
            let bare = line.trim_end_matches(['\n', '\r']);
            let Some(name) = bare
                .strip_prefix(SNIPPET)
                .and_then(|rest| rest.strip_suffix("}}"))
            else {
                rendered.push_str(line);
                continue;
            };
            let Some((_, snippet, used)) = self.0.iter_mut().find(|(n, _, _)| n == name) else {
                panic!(
                    "{} names snippets/{name}, which does not exist",
                    file.display()
                )
            };
            *used = true;
            rendered.push_str(snippet.strip_suffix('\n').unwrap_or(snippet));
            rendered.push_str(&line[bare.len()..]);
        }
        rendered
    }

    /// Fails the build on a snippet no placeholder named: it would be compiled and never shipped.
    fn all_used(&self) {
        let unused: Vec<&str> = self
            .0
            .iter()
            .filter(|(_, _, used)| !used)
            .map(|(n, _, _)| n.as_str())
            .collect();
        assert!(
            unused.is_empty(),
            "snippets/ holds files no placeholder of content/ names: {unused:?}"
        );
    }
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
