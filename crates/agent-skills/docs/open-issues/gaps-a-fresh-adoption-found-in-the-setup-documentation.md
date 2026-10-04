---
kind: todo
---
# A fresh adoption found five places where the setup documentation left it guessing

## Summary

An agent adopted the published 0.2.0 on rust-lang/log from the published documentation and the
installed skills alone. It reached a passing check, and recorded five places where the text left
it to choose without telling it how.

## Details

### What

Each item quotes the agent's report; the choices it made are its own, with no owner available.

1. **A project that is one crate with no workspace.** The setup skill says "A Rust project runs
   the checker from its maintenance crate". For a single published crate, that means turning its
   Cargo.toml into a workspace, which the skill does not discuss; the agent installed the binary
   locally instead.
2. **The skill prefix and the routing table.** The setup skill asks for "the project's skill
   prefix" and "the routing table" and gives no shape for either. The shape is in
   `knowledge-architect-agent-configuration` §4, which the setup skill does not name at that point.
3. **Existing documentation against the check.** The setup skill says existing documentation
   "keeps it until its move is planned", and the check still demanded an edit to an existing Rust
   doc comment holding a bare path to its tests/macros.rs, a real pointer under
   `design@core@every-path-names-its-anchor`. The text gives no move for a finding in a file the
   adoption is not supposed to change yet.
4. **The local install directory.** The install line uses `--root .tools`, and neither the setup
   skill nor CRATES-IO.md says to ignore `.tools`, which the walk otherwise reads.
5. **The `checker source:` line** that every run prints is explained nowhere a consumer reads.

### Why it matters

`goal@knowledge-architect@adoption-is-easy` is met "while a project adopts the checker and the
workflow by following that documentation alone". Each item is a point where an adopter guesses,
and two of the guesses, editing upstream source and leaving `.tools` unignored, change the
adopted project.

### What would close it

The setup skill, and CRATES-IO.md or the core README where the item concerns a consumer of the
binary alone, answer each item; or the owner judges an item not worth text, under
`design@agent-skills@additions-need-real-use`, and it is struck from this list.
