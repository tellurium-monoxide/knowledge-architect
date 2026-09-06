---
kind: defect
---
# A submodule's and a symlinked directory's contents are read by nothing

## Summary

The walk is git's listing, and `git ls-files` reports a submodule as one gitlink entry and a
symlinked directory as one symlink entry, descending into neither. Both have no suffix the walk
covers, so both are dropped and named by no finding.

## Details

### What

The walk is git's listing, and `git ls-files` reports a submodule as one gitlink entry
and a symlinked directory as one symlink entry, descending into neither. Both have no suffix the
walk covers, so both are dropped and named by no finding. Every document inside is outside the
walk AND outside the inverse assertion of `uncovered`.

### Why it matters

A component vendored as a submodule would be conformant by vacuum, which breaks the property
`design@knowledge@git-supplies-the-walk` exists for: nothing leaves every check while the run stays
green. It is bounded today because this
repository holds no submodule and no symlinked directory, so nothing is currently unread.

### What would close it

A finding naming every gitlink and every symlink entry in the listing,
so a project that grows one is told rather than silently narrowed. Deciding, in
`design@knowledge@git-supplies-the-walk`, whether a submodule's own listing should be walked as a
project of its own instead.

### Reproduce

Add a submodule holding a `.md` file that cites a rule the release does not hold,
and a symlink to a directory holding another. `cargo knowledge check` reports neither. The tree
walk this replaced reported both.
