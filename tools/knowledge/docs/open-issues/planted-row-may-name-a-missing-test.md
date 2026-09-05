---
kind: observation
---
# A `PLANTED` row may name a test that does not exist

## Summary

Two rows of `PLANTED` in `path@knowledge@tests/mock_projects.rs` carry
`Planted::ByTheBinary(<test name>)`, because `changes` and `corpus` are not in `check::run`.

## Details

### What

Two rows of `PLANTED` in `path@knowledge@tests/mock_projects.rs` carry
`Planted::ByTheBinary(<test name>)`, because `changes` and `corpus` are not in `check::run`. The
name is a string used inside an assertion message and nothing resolves it, so deleting or
renaming the binary test it points at leaves both test files green.

### Why it matters

It is the same shape as the entry this piece closed, moved one step: the
partition test now refuses a family with no row, and a row can still name an assertion nobody
makes. The two families would go back to being unchecked against a real project with no test
saying so.

### What would close it

Reaching the two families from `path@knowledge@tests/mock_projects.rs`, which
means a callable that runs them over a stated tree the way `check::run` runs the other six; or a
test that resolves the named test, which Rust offers no direct way to do.

### Reproduce

Rename `the_changelog_and_the_archive_each_carry_a_planted_defect` in
`path@knowledge@tests/binary.rs` and empty its body. `cargo test -p knowledge` stays green. Found by
the spec-conformity review of the mock-project piece.
