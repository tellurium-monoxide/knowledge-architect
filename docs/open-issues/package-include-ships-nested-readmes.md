---
kind: defect
---
# Each crate's package ships its docs/open-issues/README.md, which the include whitelist means to keep out

## Summary

The `include` list of crates/core/Cargo.toml, crates/agent-skills/Cargo.toml and
crates/gates/Cargo.toml names `"README.md"`. Cargo reads an `include` entry as a gitignore-style pattern, and a pattern with no
`/` matches at any depth. So `cargo package --list` for each of the three crates lists
docs/open-issues/README.md beside the crate's own README.md.

## Details

### What

Anchor the pattern to the package root, as `"/README.md"`, in every crate, and check that
`cargo package --list` lists one README.md per crate. The other entries are worth the same check:
`"Cargo.toml"`, `"LICENSE-MIT"` and `"LICENSE-APACHE"` would match at any depth too, though no
nested file of those names exists today.

### Why it matters

`design@knowledge-architect@package-include-whitelist` states that each crate ships its sources,
its Cargo.toml, its README and its license files, "and nothing else", so that "the design
documents never reach a package". An issue directory's README is such a document. A file
added under a crate with one of the whitelisted names would be shipped with no change to the
list.

### What would close it

Every entry of every crate's list matches only at the package root, and `cargo package --list` for each
crate shows exactly the whitelisted files.
