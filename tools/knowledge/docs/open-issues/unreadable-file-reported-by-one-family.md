---
kind: defect
---
# A file the walk cannot read is reported by the citation family alone

## Summary

`Model::build` keeps a file it cannot read as an empty document carrying the reason, and
`check::citations` is the only family that prints it.

## Details

### What

`Model::build` keeps a file it cannot read as an empty document carrying the reason, and
`check::citations` is the only family that prints it. So `cargo knowledge check --only structure`,
and every single-family run that is not `citations`, is silent about a document whose every
citation, reference and slug left the model.

### Why it matters

`design@knowledge@a-failed-parse-is-loud` promises that a file the walk cannot read
is a finding naming the file, with no family named. A reviewer running one family reads a clean
verdict over a document nothing read. The full run does report it, so the gate is not blind; a
narrower run is.

### What would close it

The trouble report moving out of `citations` to a place every run
performs — the summary block, or a family that always runs — so that no selection can hide it. A
test that asserts the finding under a selection that excludes `citations`.

### Reproduce

Copy `path@knowledge@tests/projects/minimal/`, `git init` and `git add -A` in the copy,
delete `path@knowledge@tests/projects/minimal/notes/b.md` from the copy, then run
`check --only citations` and `check --only structure` in it. The first names the file; the second
prints findings and does not.
