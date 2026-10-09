---
kind: observation
---
# `index --staged` clears the assume-unchanged bit of the entry it stages, and maybe its skip-worktree bit

## Summary

`index --staged` replaces a generated file's index entry with `update-index --index-info`, which
writes the entry afresh, so a flag the entry carried is lost.

## Details

### What

Reproduced on git 2.43.0 by the adversarial review of `index --staged`, in a scratch copy of the
`minimal` mock project: with `git update-index --assume-unchanged docs/open-issues/index.md` set,
`git ls-files -v` tags the entry `h`; after `klarch index --staged` staged a new index there, it
tags it `H`, the bit cleared.

Not reproduced: the skip-worktree bit, which a sparse checkout sets on entries outside its cone.
`--index-info` writes the entry with no flags, so it is expected to clear that bit too. Under a
sparse checkout that would make git report the generated file deleted from the working tree. What
would establish it: the same run over an entry with `git update-index --skip-worktree` set,
reading `git ls-files -v` before and after.

### Why it matters

`design@core@index-staged-write` says the command changes the staged entry of a generated file and
nothing else. A lost skip-worktree bit changes what `git status` reports outside the generated
file's bytes, in the sparse checkouts the adversarial review found `check --staged` otherwise
correct in. A lost assume-unchanged bit only makes git compare the file again, which is no defect.

### What would close it

The skip-worktree case reproduced or ruled out. If it reproduces, `git::stage_generated` keeps an
entry's skip-worktree bit, by setting it again after the write or by refusing such an entry, with a
test in `path@core@src/git.rs` reading `git ls-files -v` before and after.
