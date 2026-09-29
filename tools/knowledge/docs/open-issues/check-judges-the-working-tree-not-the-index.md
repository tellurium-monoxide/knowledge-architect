---
kind: question
---
# `check` judges the working tree, and a commit records the index

## Summary

`cargo knowledge check` reads the working tree: every tracked file's bytes on disk, plus every
untracked file the ignore rules leave. `git commit` records the index. So a `check` that passes
right before a commit does not show that the committed tree passes, when files are unstaged or
untracked. Should `check` offer to judge the staged tree instead?

## Details

### What

The walk is `git ls-files --cached --others --exclude-standard`, and each file's content is
read from disk, per `design@knowledge@git-supplies-the-walk`. Two shapes where the tree `check`
judged and the tree the commit records differ:

- an edit left unstaged: `check` reads the edited bytes, and the commit holds the staged ones;
- a new file left untracked: `check` walks it, and the commit does not hold it. A reference to
  that file resolves under `check`, and dangles in the commit.

`commits` already assembles a commit's tree from git objects in memory, so a model built from
the index's tree (the object `git write-tree` names) is not new machinery. Not established:
what judging the index would cost in time over this repository.

### Why it matters

It costs efficiency, not correctness at the merge. `commits origin/main..HEAD` judges each
commit's own tree in the gates and in CI before a merge, per
`design@knowledge@a-commit-message-is-a-document`, so a commit over a failing tree cannot reach
`main`. What it costs is the repair: an amend when the commit is the newest, a history edit
after. `cargo knowledge commits HEAD~1..HEAD`, run after each commit, reports it at once, as
exit 2 on the newest commit's failing tree.

### What would close it

A decision, one of two:

- `check` gains a way to judge the index's tree, with a test that stages one edit, leaves a
  second unstaged, and shows the finding follows the staged one;
- or the question is closed as not worth a second mode, since the post-commit `commits` run
  already reports the same tree one step later. The argument goes in the closing commit.
