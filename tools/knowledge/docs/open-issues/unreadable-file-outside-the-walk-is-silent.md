---
kind: defect
---
# A file outside the walk that cannot be read is silent to the inverse assertion

## Summary

The survey reads every file outside the walk to hand its text to the `uncovered` check, and a
file it cannot read is dropped without a finding. A workflow file holding a rule number and one
byte of Windows-1252, or one the process may not read, passes the assertion that an unwalked file
names no rule.

## Details

### What

`from_listing` in `path@knowledge@documentation/src/survey.rs` reads each file outside the walk
through the closure the caller hands it, and keeps only the files that read as text: a read that
fails is `None` and the path is not in `outside`. Observed by an adversarial review over a copy of
`path@rules-corpus@tests/projects/minimal/`, run with the rules extension: a text file under the mock's notes directory, outside
the walk by its suffix, holding a marked mock rule number and one accented byte in Windows-1252,
and the same file made unreadable with mode 000, both gave `PASSED: no findings`, while the
same content readable and in UTF-8 was the `uncovered` finding. The walk's own files
are handled: an unreadable walked file is an empty document carrying its trouble, phase 2.

### Why it matters

`design@knowledge@the-regime-has-no-opt-out` and the `uncovered` check exist so that a rule number
outside the walk is a finding rather than a silence, and one encoding byte reopens the silence.
The check predates the phases and the defect is older than them.

### What would close it

`from_listing` recording a file outside the walk that could not be read, as a finding of phase 2
beside the walked file that could not be, so the run stops there and names it. A test over a
copy of `minimal` with such a file, asserting the phase-2 stop and its path.
