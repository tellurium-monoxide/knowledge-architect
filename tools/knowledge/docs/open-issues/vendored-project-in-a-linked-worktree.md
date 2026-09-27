---
kind: defect
---
# A project vendored under its repository cannot be judged by the hook in a linked worktree

## Summary

In a linked worktree git sets `GIT_DIR` for the hook. A project vendored in a subdirectory runs
the tool from that subdirectory, and every git the tool runs there then treats the subdirectory
as the top of the worktree. The hook refuses every commit.

## Details

### What

Reproduction: a repository holding the project under `sub/`, a hook of the shape
`f="$(pwd)/$1"; cd sub && exec cargo knowledge commit-message "$f"`, and a linked worktree made
with `git worktree add`. In that worktree, `git commit -a` is refused with "its tree holds no
knowledge.toml": `git write-tree` and `git ls-tree` answer paths relative to the worktree top,
while the tool looks for the manifest at the project root. Before the hook read the index, the
same setup produced 546 phase-2 findings instead: refused either way.

Not established: whether unsetting `GIT_DIR` in the tool, or reading paths through
`git rev-parse --show-prefix`, is the right repair. The main worktree is not affected, since git
sets no `GIT_DIR` there.

### Why it matters

`design@knowledge@a-commit-message-is-a-document` judges a message against the tree the commit
will hold, and the tool supports a vendored project (`History::at` in
`path@knowledge@tests/binary.rs` builds one). In a linked worktree that combination has no
working hook. This repository is not vendored, so it is not affected today.

### What would close it

A test committing through the hook in a linked worktree of a vendored project, passing.
