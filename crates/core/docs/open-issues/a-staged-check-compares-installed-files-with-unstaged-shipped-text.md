---
kind: defect
---
# In a project that builds the checker from its own tree, `check --staged` compares the staged installed files with unstaged shipped text

## Summary

In this repository, the binary `cargo klarch` runs is built from the working tree, so the text it
ships is the working tree's `path@agent-skills@content/`, staged or not. `check --staged` compares
each staged installed file with that text. A commit that stages one edited skill and its installed
copy, while another edited skill and its copy stay unstaged, is reported: the unstaged skill's
staged installed copy "differs from what version 0.6.0 ships". A clean checkout of the same staged
tree builds a binary from the staged text and passes.

## Details

### What

Reproduction, on the branch agent-text-admission, as observed:

1. Edit three files of `path@agent-skills@content/` (two skills and one agent) and run
   `cargo klarch install-agent-skills`, which rewrites the three installed copies under .claude.
2. Stage one skill's source and its installed copy only.
3. `cargo klarch check --staged` fails in phase 2, with two findings, one per unstaged pair, each
   on the installed copy: "this installed file differs from what version 0.6.0 ships for this
   project".
4. Commit the staged pair anyway, then the other two pairs one commit each, and run
   `cargo klarch commits origin/main..HEAD`: every commit passes, the tree half included.

The comparison is the first loop of `check` in `path@core@src/check/agents.rs`, over
`inputs.shipped`. In this repository `inputs.shipped` is the list the binary was compiled with,
from the working tree's content/, since `path@agent-skills@build.rs` reads the files at build
time. The staged installed copies of the unstaged pairs hold the old text, while the binary ships
the new.

Ruled out: the commits themselves. `commits` judges each commit's tree with the tip's checker and
passed all three. Assumption, not established: a project that pins a published checker never
meets this, since its binary's shipped text does not move with its working tree.

### Why it matters

`design@core@staged-check-semantics` states that `check --staged` applies `check`'s rules "as on a
clean checkout of the staged tree". On a clean checkout of this staged tree the comparison passes,
so the staged check reports a finding that the checkout does not have: the reverse of
`tripwire@core@staged-pass-fails-the-checkout`, which watches a staged pass that a checkout fails.
The root `CLAUDE.md` asks that a failing staged check be repaired before the commit, so a session
that commits edits of several installed texts one by one meets a finding it cannot repair except
by staging everything, against `goal@knowledge-architect@documentation-stays-consistent`, whose
tools must say what a commit will record.

### What would close it

A change in which `check --staged`, in a project whose checker is built from its own tree, compares
the staged installed files with the staged shipped text, or reports that it cannot, with a
regression test that runs the reproduction above in a mock project; or a decision, recorded in
`design@core@staged-check-semantics`, that this case is accepted and how a session commits one
pair at a time.
