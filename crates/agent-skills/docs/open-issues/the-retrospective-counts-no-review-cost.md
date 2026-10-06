---
kind: todo
---
# The retrospective records no count of what a merged branch cost in review

## Summary

The owner approved that the retrospective records three numbers per merged branch: the review
rounds, the repair commits, and the owner's corrections of drift. Nothing records them yet, so
whether the workflow lowers the cost of a change has no measurement.

## Details

### What

`goal@knowledge-architect@agents-get-a-complete-workflow` promises "fewer review rounds, less
re-derivation of past arguments, and fewer decisions reversed by accident". No instrument counts
any of them. The retrospective, `path@agent-skills@content/skills/retrospective/SKILL.md`, asks
standing questions and writes two files, and none of its sections asks for a count.

The three numbers, as the owner approved them in a keep-or-change evaluation of the project's
direction:

- **review rounds**: the review dispatches the branch went through before its merge;
- **repair commits**: the commits that repair a review's findings, `[review]` commits in this
  repository included;
- **drift corrections**: the times the owner corrected work that departed from a recorded
  decision, as distinct from ruling on a decision.

The evaluation that raised it measured, over this repository's history since the checker was
split from thaum, that a third of the commits were `[review]` commits and that the project's
record about itself took more churn than its Rust. Those figures are in no file, and the window
was four days, so they separate no bootstrap from a steady state; that is what the counts are
for. Re-take them with `git log --format=%s` over the range, counting subjects that open with
`[review]`.

### Why it matters

Without the counts, `goal@knowledge-architect@agents-get-a-complete-workflow` cannot be shown met
or unmet, and a proposal to add or cut workflow text, which
`design@agent-skills@capability-over-conformance` weighs, is argued from intuition. The counts are
also the evidence `goal@knowledge-architect@the-workflow-improves-through-real-use` names: real
sessions as the test of the workflow.

### What would close it

The retrospective skill asks for the three numbers of each branch merged in its session, and says
where they are recorded so that they can be compared across sessions. The home of the series is a
choice for that change: the retrospective's files live outside the project, per
`design@agent-skills@retro-file-location`.
