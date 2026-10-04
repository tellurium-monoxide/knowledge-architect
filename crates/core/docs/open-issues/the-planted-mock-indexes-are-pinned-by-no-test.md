---
kind: todo
---
# The committed indexes of the `planted` mock are compared with the generator by no test

## Summary

`every_committed_index_is_what_the_generator_writes` in `path@core@src/mock_projects.rs` covers the
`minimal`, `dirhome` and `core` mocks. No test compares the `planted` mock's committed indexes with
the generator, so they can drift from the contract with every test passing. Finding P4 of the
retrospective of 2026-10-04.

## Details

### What

The adversarial reviewer of the branch that changed a one-row index's count line reverted three
one-row indexes of `planted` (docs/plans/specs/index.md, docs/plans/milestones/index.md and
docs/plans/milestones/m-one/index.md) to `1 entries`, and the whole test suite of the core passed,
as reported. `path@core@CLAUDE.md` says the generated files of `planted` are written by hand, stale
ones among them on purpose, so they cannot all be regenerated and compared.

### Why it matters

`planted` is the mock for detection in the last phase, with one defect per core check. A generated
file of it that is meant to be current, and drifts, adds a finding nobody planted to a run over it,
and no test notices. `design@core@a-file-register-index-is-rows` makes the bytes a contract, and that contract is
not held for these files.

### What would close it

A list, beside the `planted` tests, of its generated files meant to be current, each compared with
the generator, shown to fail by reverting one of them.
