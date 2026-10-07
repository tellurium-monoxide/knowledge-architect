---
kind: todo
---
# No check reads the setup skill's Cargo.toml, alias and CI blocks against the workspace

## Summary

The setup skill's section for a Rust project shows four blocks: the maintenance crate's
Cargo.toml, its cargo aliases with the `[env]` entry, its main, and a continuous integration
workflow. The main is compiled by an example of xtask. The other three are checked by nothing, so a
change to the gates library, to the core's command line or to the tie of a build to its checkout
can leave them wrong while every gate passes.

## Details

### What

In `path@agent-skills@content/skills/setup/SKILL.md`, section "In a Rust project":

- the crate's manifest, a `toml` block: package name, `publish = false`, and the dependencies on
  clap, the checker and the gates library, pinned exactly;
- the aliases, a `toml` block for the project's cargo configuration: `x` and `klarch`, and the
  `[env]` entry that ties a build to its checkout;
- the continuous integration workflow, a `yaml` block that runs the gates on every ready pull
  request.

When the main moved into a compiled file, the session's recommendation, which the owner approved,
said: "Only the Rust block becomes checked. The three toml blocks of the section (the crate's
Cargo.toml, the aliases, the CI) stay unchecked." The issue that asked for the main to be compiled
covered the Rust block only. Each block is labelled in the skill as an illustration to adapt.

### Why it matters

The setup skill recommends these blocks to every adopting Rust project,
per `goal@knowledge-architect@setup-brings-quality-tools`. An alias that names a command the core
no longer has, a dependency key the pin no longer matches, or a CI step that no longer runs the
gates the way `design@gates@the-library-owns-the-flags` sets them, ships to every project that
follows the skill. A label "to adapt" tells a reader to change names, not that a block may be
wrong.

### What would close it

A check that fails when one of the three blocks no longer works with the workspace, shown to fail by
breaking one item each block uses: for instance the blocks moved to files of
`path@agent-skills@snippets/` that a test loads and parses or runs, as the main is. Or the owner's
ruling that the label "an illustration to adapt" is enough, recorded in the head.
