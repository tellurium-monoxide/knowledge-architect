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

Two sessions reported their review cost without the counts, as impressions the counts would test:

- the owner-authority milestone, one design discussion and five pull requests: 37 review agents;
  13 of the 30 commit subjects from the milestone document's commit, "The owner-authority
  milestone: a question goes to the authority that answers it, and the owner rules at a fork", to
  the last commit of its last slice, "[review] Repairs from the transcript review of slice 4: the
  review skill's label sentence, and the owner's report kept with its hedges", open with
  `[review]`. Three extra rounds were each forced by a repair that touched a head before the
  transcript review could run. Its retrospective proposes measuring before letting the
  decision-record review of a head repair run in the transcript review's batch.
- a slice in thaum, on knowledge-architect 0.6.0: one finding of the decision-record axis cost
  three reviews of that axis, through `design@agent-skills@existing-heads-on-touch`, a split
  touching a second head that was then found bundled.

### Why it matters

Without the counts, `goal@knowledge-architect@agents-get-a-complete-workflow` cannot be shown met
or unmet, and a proposal to add or cut workflow text, which
`design@agent-skills@capabilities-not-structure` weighs, is argued from intuition. The counts are
also the evidence `goal@knowledge-architect@the-workflow-improves-through-real-use` names: real
sessions as the test of the workflow. The count of drift corrections would also measure the
"rather than correcting drift" of `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

The retrospective skill asks for the three numbers of each branch merged in its session, and says
where they are recorded so that they can be compared across sessions. The home of the series is a
choice for that change: the retrospective skill writes its files outside the project.
