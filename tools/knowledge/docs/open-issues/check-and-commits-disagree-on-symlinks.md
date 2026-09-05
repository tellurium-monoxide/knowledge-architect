---
kind: observation
---
# `check` and `commits` disagree about a symlinked file, and the summary claims they agree

## Summary

`check` reads a file's bytes through the filesystem, so a symlink at a walked path yields the
bytes of what it points at. `commits` reads the tree blob, whose content for a symlink entry is
the target's path as text.

## Details

### What

`check` reads a file's bytes through the filesystem, so a symlink at a walked path
yields the bytes of what it points at. `commits` reads the tree blob, whose content for a
symlink entry is the target's path as text. The two therefore build different models of the same
commit whenever a walked `.md` or `.rs` path is a symlink. `commits`' summary line says a
skipped or failing commit's tree "fails n finding(s), which `cargo knowledge check` reports",
and for such a tree `check` reports a different number.

### Why it matters

Small today and only in one direction: `commits` reads less, so a document
reachable only through a symlink is judged by the range check and not by its own bytes. The
misleading half is the summary line, which sends a reader to a command that answers differently.
The same class as the submodule-and-symlinked-directory entry above, which is where the walk's
own answer is recorded; this entry is about the two readers disagreeing rather than about the
walk.

### What would close it

Either the summary line stops attributing its count to `check`, or the
per-commit read resolves a symlink entry the way the filesystem does. The second is what makes
the two models one, and it needs a decision about whether a symlink out of the tree is followed
at all.

### Reproduce

In a project with a clean tree, add a symlink whose name ends in `.md` beside a
document that carries a dangling reference, pointing at it; stage both and commit.
`cargo knowledge check` reports the reference twice, once per path;
`cargo knowledge commits <base>..HEAD` reports it once.
