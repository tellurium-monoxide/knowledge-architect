---
kind: defect
---
# A worktree build sharing the target directory leaves the live checker exempting no fixture

## Summary

`cargo knowledge` compiles the checker's own source path into the binary. A run of it from a
second checkout, a git worktree, that builds into the first checkout's target directory leaves
the binary naming the worktree's path. The first checkout's next `cargo knowledge check` does not
rebuild, exempts none of the tool's own fixtures, and fails on them. It stays so after the
worktree is removed, until the two packages are cleaned.

## Details

### What

`checker_source` in `path@knowledge@src/main.rs` returns `env!("CARGO_MANIFEST_DIR")`, and its doc
states the premise: *"the alias in `path@thaum@.cargo/config.toml` builds the binary from the
checkout on every run, so the compiled path names the tree being checked"*. The premise fails when
two checkouts share one target directory.

Reproduction, from the project root, with a clean tree that passes the check:

```sh
git worktree add --detach "$HOME/.claude/worktrees/stale-repro" HEAD
(cd "$HOME/.claude/worktrees/stale-repro" && CARGO_TARGET_DIR=<root>/target cargo knowledge check)
cargo knowledge check          # in the root again
```

where `<root>` is the first checkout's absolute path, so that the worktree's build writes the
first checkout's `target/`. Observed on the branch that landed step 4c of slice 4:

- in the worktree: `checker source: tools/knowledge, 38 file(s) with string literals read as
  data`, and `PASSED: no findings`;
- in the root afterwards: `checker source: <the worktree's absolute path>/tools/knowledge, 0
  file(s) with string literals read as data`, and `phase 3: 57 finding(s)`, every one a slug or a
  reference planted in a fixture under `path@knowledge@documentation/src/` or
  `path@rules-corpus@tests/`;
- the same after `git worktree remove`;
- `cargo clean --release -p knowledge -p documentation`, then the check in the root: 38 files
  exempted, `PASSED: no findings`.

A second occurrence, on a branch editing only the agent configuration: a `routing-reviewer`
subagent ran `cargo knowledge check` on the tip of origin/main in a temporary checkout under the session's
scratchpad directory, outside `$HOME/.claude/worktrees`, and removed it. The root then printed
`checker source: <that checkout's absolute path>/tools/knowledge, 0 file(s) with string literals
read as data` and `phase 3: 57 finding(s)`, all under `path@knowledge@documentation/src/` and
`path@rules-corpus@tests/`. Whether that reviewer set `CARGO_TARGET_DIR` is `not established`: its
report does not say, and `path@thaum@.cargo/config.toml` sets no `target-dir`.

A cheaper recovery than the clean, observed on that occurrence:

```sh
touch tools/knowledge/src/main.rs tools/knowledge/documentation/src/lib.rs
cargo knowledge check          # rebuilds; 38 files exempted, PASSED: no findings
```

Why cargo reuses the binary is `not established`. The assumption is that the two checkouts are two
package ids, each with a fingerprint of its own that stays fresh, and one uplifted binary at
the `knowledge` file under the release profile of the shared target directory that the last build wrote.

`cargo knowledge index` refuses to write while phase 3 holds findings, so it fails the same way.
Whether `cargo x gates` runs the same binary, and fails the `knowledge` gate for the same reason,
is `not established`.

### Why it matters

The failure reads as 57 defects of the tool's source, in a tree the change under way never
touched, and costs a session a bisection to find it is the build. It breaks the premise
`design@knowledge@checker-source-literals-are-data` rests on, that the compiled path names the tree
being checked. The procedure most likely to trigger it is the review one: `dispatching-a-review`
sends every reviewer that runs a binary to a detached worktree, and a reviewer that points
`CARGO_TARGET_DIR` at the live tree to save a build is what left it here.

### What would close it

Either of these, with a test that builds the binary from two checkouts into one target directory
and checks the second's exemption count:

- the binary finds its source directory at run time instead of at compile time, for example from
  the project the walk resolves and the `knowledge` Component's directory in
  `path@thaum@knowledge.toml`, so no build can name another tree;
- or the check refuses to run when the compiled path is not inside the project it walks, and names
  the clean command, so the failure states its cause instead of reporting fixtures.
