# Knowledge checker

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says so
there, in one place, with a reason beside it. The one thing compiled in is the tool's own
directory, taken from `CARGO_MANIFEST_DIR` of the binary crate, so that the string literals of its
own source are read as data rather than as citations, per
`knowledge#checker-source-literals-are-data`. That is a fact about the tool, not about any tree.

Read `knowledge@docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
