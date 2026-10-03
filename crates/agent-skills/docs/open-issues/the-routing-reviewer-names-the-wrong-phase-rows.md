---
kind: defect
---
# The routing reviewer says its last three table rows are found in an earlier phase, and one is not

## Summary

The installed routing reviewer tells its reader that the last three rows of its table of check
failures come from a phase before the last. The last row is a finding of the last phase, and a row
that does come from an earlier phase is not among the three.

## Details

### What

crates/agent-skills/content/agents/routing-reviewer.md says: "The last three rows of the table
below are found in an earlier phase, so when they appear, references and registers were not
judged." The last three rows are:

- `heading` … `home carries no slug`: raised in crates/core/src/entity.rs, in the definitions
  phase, phase 3;
- `is also defined at`: raised in crates/core/src/entity.rs, phase 3;
- `is linked from a file that is not a navigation home`: raised in
  crates/core/src/check/references.rs, a check of the last phase, phase 4.

The row `is written` … `and defines nothing` is raised in crates/core/src/entity.rs, phase 3, and
sits before the last three. The self-consistency review of step 3 of the structured-plans
milestone reproduced both: a run reporting a navigation-home finding printed
`checked: … references`, and one reporting "defines nothing" printed `phase 3: 1 finding(s); phase 4
was not judged`.

### Why it matters

A reviewer that sees the navigation-home row concludes that references and registers were not
judged, when they were. A reviewer that sees "defines nothing" concludes that they were judged,
when they were not, and reports a clean axis over an unjudged one. The goal is
`goal@agent-skills@one-skill-per-activity`: the instruction contradicts the checker it describes.

### What would close it

The sentence names the rows by their phase, as the module that raises each one gives it, and the
installed copy is reinstalled with the change.
