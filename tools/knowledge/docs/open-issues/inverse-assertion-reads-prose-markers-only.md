---
kind: defect
---
# The inverse assertion reads the prose marker form alone

## Summary

`first_rule_number` in `path@knowledge@documentation/src/check/uncovered.rs` finds a dotted
rule number by splitting on characters that are neither alphanumeric nor a dot, and a section
number by a word-bounded regex over the keyword forms. The identifier form, which root
`path@thaum@CLAUDE.md` accepts wherever a name cannot hold punctuation, uses underscores, so
neither finder sees it.

## Details

### What

`first_rule_number` in `path@knowledge@documentation/src/check/uncovered.rs` finds a dotted
rule number by splitting on characters that are neither alphanumeric nor a dot, and a section
number by a word-bounded regex over the keyword forms. The identifier form, which root
`path@thaum@CLAUDE.md` accepts wherever a name cannot hold punctuation, uses underscores, so neither
finder sees it. A file outside the walk may therefore carry `cr_613_8c` in a job name, a fixture
name or a dotfile, and nothing reports it.

### Why it matters

The module's own head states the assertion without qualifying the form:
outside the walk, a rule number is forbidden. One of the two accepted forms is exempt in fact
and not in any statement, and the founding case for the family was a workflow file — exactly
where a test name is written.

### What would close it

Reading the identifier form in the same finder, with the same
prose-form message, and a planted case in `path@knowledge@tests/projects/planted/` beside the
existing one. Deciding instead that an unwalked file may carry the identifier form, and saying
so in the module head and in root `path@thaum@CLAUDE.md`, closes it as a recorded exemption.

### Reproduce

In `path@knowledge@tests/projects/dirhome/`, which is clean, write a
workflow file under `path@thaum@.github/workflows/` holding a line `run: cargo test cr_100_1` and run
`cargo knowledge check --only uncovered`: `PASSED: no findings`. Replace the token with the
prose form behind its marker and the same run reports the line. Found by an adversarial review
of the mock-project piece.
