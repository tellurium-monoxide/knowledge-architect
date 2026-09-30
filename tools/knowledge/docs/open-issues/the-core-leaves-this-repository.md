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
repository depends on the published `documentation` crate. The core's documents point at
nothing outside tools/knowledge/: `git grep -n -e rules-corpus -e "@thaum@" -- tools/knowledge`
lists this entry alone. What the core still carries of this repository, each item to be
rewritten when it leaves:

- **Tests that read this repository's layout.** Each passes here and fails in a repository
  where the core sits at the root:
  - in `path@knowledge@documentation/src/manifest.rs`, `this_project`, which walks three
    directories up from the `documentation` crate, and the two tests over it,
    `this_repository_declares_a_readable_manifest` and
    `the_walk_up_finds_the_root_from_below_it`, the second joining
    the path tools/knowledge/documentation/src to the root;
  - in `path@knowledge@tests/binary.rs`,
    `the_core_binary_names_its_own_directory_and_counts_the_files_under_it`, which walks two
    directories up, counts files under the prefix tools/knowledge/, and expects exit 1 because
    this repository's manifest holds a `[rules]` table the core does not claim;
  - in the same file, `a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused`,
    which plants tools/knowledge/Cargo.toml and tools/knowledge/documentation/Cargo.toml in
    a mock: the relative path the refusal compares is the crate's path in this repository.
- **The mocks' exclusion.** The module comments of `path@knowledge@tests/binary.rs` and
  `path@knowledge@tests/mock_projects.rs` state that this repository's manifest excludes
  `path@knowledge@tests/projects/`. The new repository's manifest owes the same row.
- **Examples drawn from this repository.** The README's register example is this repository's
  `interpretation` register, and several design heads, rejected alternatives and comments name
  thaum's rules extension as the example of an extension. Each example is evidence for a core
  decision and stays, unless the new repository wants its own.

### Why it matters

Until the core leaves, a project other than this one can use it only by depending on this
repository by path or git URL, and the core's documents keep pointing at a layout the new
repository will not have.

### Trigger

A session that creates the core's repository, which is the session the owner scheduled for it.
Its own work is the move, so every item above is part of what it does.
