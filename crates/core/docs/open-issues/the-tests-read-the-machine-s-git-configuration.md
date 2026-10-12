---
kind: defect
---
# The core's tests read the machine's git configuration, so a machine setting can fail a correct tree

## Summary

Several of the core's test helpers run git with the machine's own configuration. A global
`commit.gpgsign` that cannot sign, or a global `core.hooksPath` whose `pre-commit` hook fails,
fails dozens of tests on a correct tree, and a per-user ignore file decides which fixture files two
helpers stage. Only `Sandbox::git_command` in `path@core@tests/binary.rs` is isolated, and only for
the identity.

## Details

### What

Measured by the adversarial reviewer of the review before the merge of the branch that gave
`Sandbox::git_command` the sandbox's identity, at that branch's tip, with the machine's real
configuration untouched: each probe set `GIT_CONFIG_GLOBAL` to a file of its own and
`GIT_CONFIG_NOSYSTEM=1`, then ran `cargo test -p knowledge-architect --no-fail-fast`.

- A global configuration holding `commit.gpgsign = true` and `gpg.program = false`: 9 failures in
  the library's unit tests, 55 in `path@core@tests/binary.rs`, 1 in `path@core@tests/extension_api.rs`; the test
  `a_sandbox_commit_carries_the_sandbox_identity` fails with "gpg failed to sign the data".
- A global `core.hooksPath` pointing at a `pre-commit` hook that exits 1: the same counts, 9, 55
  and 1.
- Adding `-c commit.gpgsign=false -c core.hooksPath=/dev/null` to `Sandbox::git` alone
  brought `path@core@tests/binary.rs` down to 43 failures under either probe; the other 43 run git through
  `History::git` and other direct `Command::new("git")` calls.
- The per-user ignore file: `Sandbox::stage` pins `core.excludesFile=/dev/null`, and so do
  `History::commit` and the helper of `path@core@tests/extension_api.rs`. Two `git add -A` calls of
  `path@core@tests/binary.rs` do not: one through the free function `git`, one through `History::git`. Found
  by reading by the self-consistency reviewer of the same review; no test was shown failing.
- Not established: which other settings a test reads, and how the library's unit-test repositories
  in `path@core@src/git.rs` and `path@core@src/cli/history.rs` are configured. The failure counts
  above were not re-run by the session that opened this entry.

### Why it matters

A contributor or a CI runner with commit signing or a hooks directory set globally sees the core's
suite fail on a correct tree, and a per-user ignore rule can change what a fixture stages, so a test
can pass or fail by the machine it runs on. The gate `cargo x gates` then reports a failure that is
no defect of the change under test: on such a machine, its exit code cannot be trusted. It strains
no recorded decision.

### What would close it

Every helper of the core's tests that runs git goes through one function that isolates the
configuration a test does not mean to read: the identity as `Sandbox::git_command` does, plus at
least `commit.gpgsign=false`, `core.hooksPath=/dev/null` and `core.excludesFile=/dev/null`, or a
`GIT_CONFIG_GLOBAL` pointing at an empty file with `GIT_CONFIG_NOSYSTEM=1`. The two probes above
then pass, run as a test or by hand, and the core's `path@core@CLAUDE.md` states the contract for
every helper.
