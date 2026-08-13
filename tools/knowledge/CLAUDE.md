# Knowledge checker

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says so
there, in one place, with a reason beside it.

Read `tools/knowledge/docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
