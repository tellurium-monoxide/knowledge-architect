//! Every snippet the setup skill ships is compiled, per the doc comment of
//! `path@agent-skills@build.rs`: the build fails on a snippet no placeholder names, and this test
//! fails on one no example of this crate includes, so the shipped text is the compiled text.

use std::path::Path;

// This target links no library of its package, so it reads the variable itself.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

#[test]
fn every_snippet_is_included_by_an_example() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let snippets = crate_dir.join("../../crates/agent-skills/snippets");
    let examples = crate_dir.join("examples");
    let mut included = String::new();
    for entry in std::fs::read_dir(&examples).expect("examples/ is readable") {
        let path = entry.expect("examples/ is readable").path();
        included.push_str(&std::fs::read_to_string(&path).expect("an example is readable"));
    }
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(&snippets).expect("snippets/ is readable") {
        let name = entry.expect("snippets/ is readable").file_name();
        let name = name.to_str().expect("a UTF-8 file name");
        let line = format!("include!(\"../../../crates/agent-skills/snippets/{name}\");");
        if !included.lines().any(|l| l.trim() == line) {
            missing.push(name.to_owned());
        }
    }
    assert!(
        missing.is_empty(),
        "snippets no example of xtask includes: {missing:?}"
    );
}
