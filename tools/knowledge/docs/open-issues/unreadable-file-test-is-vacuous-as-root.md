---
kind: todo
---
# The unreadable-file test asserts nothing when it runs as root

## Summary

`an_unreadable_file_outside_the_walk_is_a_phase_two_finding_naming_it`, in
`path@knowledge@tests/binary.rs`, makes a file outside the walk unreadable with mode 000 and
returns early, passing, when the process can still read it. A process running as root reads
through the mode, so under a root CI runner the test passes and checks nothing.

## Details

### What

The test writes a text file under the notes directory of a copy of the `minimal` mock, sets its mode to 000, and
calls `std::fs::read` on it. If the read succeeds it returns before running `check`. Mode bits
do not stop root, so the early return is taken whenever the suite runs as root. Found by the
adversarial review of the core on the branch that made an unreadable file outside the walk a
phase-2 finding; `not established` whether this repository's CI runner runs as root.

The other states of `Outside` in `path@knowledge@documentation/src/survey.rs` have tests that do
not depend on the user: a missing file and a nested repository in
`a_missing_file_and_a_nested_repository_outside_the_walk_are_named_for_what_they_are`. Only
`Outside::Unreadable` depends on the mode.

### Why it matters

`design@knowledge@a-failed-parse-is-loud` states that a file outside the walk whose bytes cannot
be had is a phase-2 finding. Under root, a regression that drops that finding passes the suite.

### What would close it

A way to make the read fail that does not depend on the user, for example a `survey` unit test
that hands `from_listing` a read closure answering `Outside::Unreadable` and asserts the
phase-2 finding in `check::tree`; or a check of the CI runner's user, recorded here, that shows it
is not root, with the test failing loudly instead of passing when it cannot reproduce the state.
