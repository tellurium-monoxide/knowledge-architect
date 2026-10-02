---
kind: todo
---
# No check compiles the maintenance crate the setup skill shows

## Summary

The setup skill's section for a Rust project shows a maintenance crate's main, written against
the public interfaces of the checker's library and the gates library. Nothing compiles it, so a
change to either interface can leave the shipped snippet broken while every gate passes.

## Details

### What

A test that extracts the Rust block of the section from
`path@agent-skills@content/skills/setup/SKILL.md`, compiles it against this workspace's crates,
and fails when it does not compile; or the snippet moved into a compiled example that the section
quotes from.

### Why it matters

`design@agent-skills@setup-rust-section` recommends the snippet to every adopting Rust project, and
content/ is outside the walk, so no check reads it. The interfaces it uses are
`design@core@the-core-cli-is-a-library-module` and `design@gates@the-library-owns-the-flags`, both
open to change under 0.x.

### What would close it

A gate that fails when the section's Rust block no longer compiles against the workspace, shown to
fail by renaming one item the block uses.
