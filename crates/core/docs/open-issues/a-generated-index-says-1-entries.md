---
kind: defect
---
# A generated index with one row says "1 entries"

## Summary

The count line of every generated index is `<n> entries`, so an index with one row reads
"1 entries".

## Details

### What

`render` in `path@core@src/index.rs` writes the count with `format!("{} entries\n", rows.len())`.
Three committed fixtures show it, each a one-row index:
`path@core@tests/projects/dirhome/docs/plans/specs/index.md`,
`path@core@tests/projects/dirhome/docs/plans/milestones/index.md` and
`path@core@tests/projects/dirhome/docs/plans/milestones/a-milestone/index.md`. An adoption trial
on rust-lang/log met it in its first issue index.

### Why it matters

It is a wrong word in a file every project commits, against
`goal@knowledge-architect@adoption-is-easy`. The fix is not cosmetic for a consumer: the
`generated` check compares bytes, per `design@core@a-file-register-index-is-rows`, so every
consuming project with a one-row index fails until it runs `index` once after upgrading. That is
a Migration entry of the changelog, which is why it is an entry rather than a quiet fix.

### What would close it

`render` writes "1 entry" for one row; the one-row fixtures are regenerated; a test asserts both
forms; and the changelog carries the Migration entry.
