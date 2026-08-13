//! Every directory the project registers as carrying tracker files actually carries them.
//!
//! The registration is a two-way claim and this is one direction of it: a registered file that
//! is missing means the report silently under-counts what is outstanding, and
//! `cargo knowledge outstanding` is what root `CLAUDE.md` sends a session to before it diagnoses
//! anything.

use crate::finding::Finding;
use crate::manifest::Manifest;

use super::Inputs;

pub fn check(manifest: &Manifest, inputs: &Inputs) -> Vec<Finding> {
    let mut out = Vec::new();
    for (dir, names) in manifest.trackers().iter() {
        for name in names {
            let path = dir.join(name);
            if !inputs.present.contains(&path) {
                out.push(Finding::in_file(
                    &path,
                    format!(
                        "`{}` is registered as carrying {name} and does not",
                        dir.display()
                    ),
                    "create it, or stop registering the file in knowledge.toml [trackers]",
                ));
            }
        }
    }
    out
}
