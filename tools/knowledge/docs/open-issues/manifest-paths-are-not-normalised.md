---
kind: defect
---
# A manifest path spelled with `.` or `..` is compared unnormalised, so an existing directory is reported absent

## Summary

Every path the manifest declares is compared component by component against git's listing,
and neither side is normalised. A location declared with `path = "./docs"` is reported as not existing
while `docs` exists; one with `path = "."` is reported twice, as the project root and as a directory that
does not exist.

## Details

### What

`check::registers::declarations` and the anchor loop of `check_under`, in
`path@knowledge@documentation/src/check/registers.rs`, ask `inputs.present.contains(&path)`
for a declared path, and `nesting` compares anchor paths with `Path::starts_with`. Git lists
`docs`, never the spelling with a leading dot segment, so a declaration with
`path = "./docs"` matches nothing and is reported as declared and not existing. Observed over a copy of
`path@knowledge@tests/projects/minimal/` with three declarations appended in turn:

- `[locations.rooty] path = "."` — two findings: `[locations.rooty] names the project root`,
  which is right, and `` `.` is declared an anchor and does not exist ``, which is false.
- `[locations.papers]` with `path = "./docs"` — declared an anchor and does not exist,
  false; `docs` exists.
- reported by the adversarial review of the nesting refusal, not reproduced here:
  `path = "docs/../docs"` is reported absent the same way, and
  `path = "docs/open-issues/../plans"` is reported as sitting inside the issue register's
  directory, on a component-wise prefix.

### Why it matters

A declaration that is right is reported wrong, and the repair text sends the reader to create a
directory that exists. The nesting refusal in `nesting` reads `..` as a further component, so
a spelling with `..` can be refused for a place it does not name, or pass one it does.

### What would close it

Normalise every path the manifest declares once, at load in `Manifest::parse`: drop `.`
segments, refuse `..` and a leading `/` as a finding naming the row, the way a register `dir`
is already refused when it is not one plain segment. A test per spelling above, asserting
one finding for `path = "."` and none for `path = "./docs"`.

### Reproduce

Copy `path@knowledge@tests/projects/minimal/`, `git init` and `git add -A` in the copy, append
`[locations.papers]` with `path = "./docs"` and `registers = ["issue"]` to its manifest, run
`cargo knowledge check --only registers` from the copy, and read the anchor finding.
