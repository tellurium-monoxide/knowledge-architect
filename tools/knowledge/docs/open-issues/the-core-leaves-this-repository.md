---
kind: deferred
---
# The core leaves this repository as a crate of its own

## Summary

`goal@knowledge@documentation-half-publishes-alone` publishes the core for other projects.
The core and the rules extension are split, per `design@thaum@knowledge-is-a-generic-core`, and
the core sits in one directory, tools/knowledge/, ready to leave. Creating its repository and
publishing its crate is deferred to a session of its own. This entry holds what that session
has to do that this repository's documents would otherwise not tell it.

## Details

### What

The move itself: tools/knowledge/ becomes a repository, its crates are published, and this
repository depends on the published `documentation` crate. What the core still carries of this
repository, each to be rewritten or dropped when it leaves:

- **Pointers into rules-corpus**, where the core's documents describe how this repository
  composes the two halves: the core's README and scoped `CLAUDE.md`, the issue register's README
  and two issue entries, the self-location tripwire, the two core mocks' head comments, the core's
  `main.rs`, and the module comments of both integration test files.
  `git grep -n rules-corpus -- tools/knowledge` lists them.
- **Pointers into the rest of this repository**: every `design@thaum@<id>`, `goal@thaum@<id>` and
  `path@thaum@<path>` reference under tools/knowledge/. `git grep -n "@thaum@" -- tools/knowledge`
  lists them.
- **Rule-shaped residue that compiles nothing about the rules but reads as thaum's**: unit-test
  manifests that write a `[rules]` table (`git grep -n "\[rules\]" --
  tools/knowledge/documentation`), the `const RULE` fixtures of the Rust grammar's tests, the check
  names `changes` and `corpus` in one rendering test of `cli`, and the retired-key message naming
  thaum's old `[interpretations]` table in `retired_keys`.
- **The one-home question for each generic head that argues from a rules example**: the example
  is evidence for a core decision and stays unless the new repository wants its own.

### Why it matters

Until the core leaves, a project other than this one can use it only by depending on this
repository by path or git URL, and the core's documents keep pointing at a layout the new
repository will not have.

### Trigger

A session that creates the core's repository, which is the session the owner scheduled for it.
Its own work is the move, so every item above is part of what it does.
