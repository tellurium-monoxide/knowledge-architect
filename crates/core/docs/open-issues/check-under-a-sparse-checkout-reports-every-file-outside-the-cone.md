---
kind: observation
---
# Plain `check` under a sparse checkout reports every file outside the cone as an unstaged deletion

## Summary

In a cone-mode sparse checkout, plain `check` stops in phase 2 with one finding per tracked file
outside the cone, each "git lists this file and the working tree does not hold it". `check --staged`
over the same tree passes.

## Details

### What

Reproduced on git 2.43.0 with the release binary of the branch that built `check --staged`: a copy
of the `dirhome` mock project, its pin set to the binary's version, committed, then
`git sparse-checkout init --cone`, `git sparse-checkout set docs/design docs/goals` and
`git config index.sparse true`, then `git sparse-checkout reapply`. `klarch check` exits 1 with
`phase 2: 10 finding(s)`, among them `docs/open-issues/README.md:1  git lists this file and the
working tree does not hold it`. `klarch check --staged` exits 0. The adversarial review of
`check --staged` saw it first and recorded it as outside its change.

The cause is read from the code and not traced further: the walk lists the index with
`ls-files --cached`, which holds the files outside the cone with the skip-worktree bit, and reads
each from disk, where a sparse checkout leaves it absent. Whether `index.sparse` changes the count
is not established.

### Why it matters

`design@core@git-supplies-the-walk` keeps a tracked path the working tree does not hold in the walk
and reports it, so that a working-tree state takes no live document out of every check. A sparse
checkout is a working-tree state a user chose, and under it plain `check` cannot pass at all,
which `goal@knowledge-architect@any-project-can-adopt-it` strains for a project worked on that
way. Whether the head meant to cover a skip-worktree entry is not established.

### What would close it

A ruling on what plain `check` judges under a sparse checkout: the files on disk, the index's
blobs for the skip-worktree entries, or a refusal naming the sparse checkout. Then the head
`design@core@git-supplies-the-walk` states it, and a test in `path@core@tests/binary.rs` builds a
sparse checkout and asserts the chosen verdict.
