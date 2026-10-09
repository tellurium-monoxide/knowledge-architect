---
kind: defect
---
# The gate-list head says every difference between CI and a local run is a flag, and the library also adapts its output to Actions

## Summary

`design@xtask@gates-list-primary-home` states, in bold, "Where CI and a local run differ, the
difference is a flag." Two differences are not flags: the gates library detects GitHub Actions
from the environment, and then accepts a pipe on stdout and wraps each gate in workflow commands.
The sentence holds of which gates run and how a run stops, and not of what the run prints.

## Details

### What

- The head, in `path@xtask@docs/design.md`: "**Where CI and a local run differ, the difference is
  a flag.**" It then lists `--locked`, `--fail-fast`, `--require-rebased` and `--full`.
- The code, `path@gates@src/gates.rs`, function `under_actions`: it returns true when the
  environment variable `GITHUB_ACTIONS` is `true`. Its doc comment says two things follow: "a pipe
  on stdout is accepted, and each gate is wrapped in workflow commands", per
  `design@gates@gates-refuse-a-pipe` and `design@gates@annotations-under-actions`.
- So a CI run differs from a local run in two ways that no flag of xtask selects.

Found by the first run of the design-record axis on this repository, whose auditor noted it
outside the axis, since that axis does not judge whether a head is true of the code.

### Why it matters

The head is built intent, and a reader takes its bold sentence as a rule: a session that adds a
CI-only behaviour reads it as owing a flag, while the library already adapts to Actions without
one. A head false of the code misleads every reader until it is repaired,
`goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

The sentence restated to what holds, for example "Where CI and a local run differ in what they run,
the difference is a flag", with the output the library adapts to Actions pointed to
`design@gates@annotations-under-actions`; or the library's detection of Actions turned into a flag, if that is the
better design. Either goes through the decision-recording skill.
