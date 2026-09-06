---
kind: defect
---
# The changelog's own counts are asserted by nothing

## Summary

`check::changes` verifies a changelog section's row, keys, url, digest and quotes, and compares
no count. The `edited`, `renumbered` and `gone` values are checked against nothing, so a section
may claim a number its own entries contradict.

## Details

### What

`check::changes` verifies a section's summary-table row, its four `meta` keys, its
source url, its recorded digest and every blockquote against the release the section pins. It
compares no count: neither the `edited`, `renumbered` and `gone` values in the `meta` block nor
the same three columns in the summary table are checked against the entries the section carries.

### Why it matters

`bumping-rules` reads the changelog as its work list, and the counts are the
first thing a reader takes off it. A section claiming six changes above zero entries sends
whoever reads it looking for triage that was never owed, or hides triage that was.

### What would close it

Comparing each of the three counts against the entries the section
holds, classified by the verb its title opens with, and reporting a mismatch as a finding naming
both numbers. The table row and the `meta` block are two statements of one fact, so the check
covers both or says which one is authoritative.

### Reproduce

In `path@knowledge@tests/projects/dirhome/`, whose one section legitimately carries no
entry, edit its `CHANGES.md` so the table row reads `| 3 | 2 | 1 |` and the `meta` block says
`edited: 3`, `renumbered: 2`, `gone: 1`. `cargo knowledge check` prints
`changelog: 0 rule change(s)` and `PASSED: no findings`. Found by an adversarial review of the
mock-project piece.
