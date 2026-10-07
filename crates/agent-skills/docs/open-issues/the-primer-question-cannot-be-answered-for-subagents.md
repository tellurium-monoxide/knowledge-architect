---
kind: question
---
# The retrospective asks whether the primer reached the subagents, and no session can see it

## Summary

The retrospective's standing question "Was the primer present in this session, and in its
subagents?" watches how the primer is delivered. Four received retrospectives answered its second
half with "cannot be observed", "not verified", "assumed" or "partly shown". A question no session
can answer watches nothing. What should it ask instead?

## Details

### What

The question is in `path@agent-skills@content/skills/retrospective/SKILL.md`, among the standing
questions. It watches the primer's delivery by the import line of a project's root `CLAUDE.md`,
per `design@agent-skills@premortem-as-watch-points`.

The answers to its second half, in the four workflow files analysed under
`klarch-retrospective-intake` (the standing answer of each):

- 2026-10-06-thaum-workflow: "Whether it was in their context from their first token cannot be
  observed from the transcript."
- 2026-10-07-thaum-workflow: "only partly shown ... No subagent was asked whether the primer was in
  its context."
- 2026-10-07-thaum-mock-reduction-workflow: "In the subagents: not verified."
- 2026-10-07-knowledge-architect-workflow: "assumed, not verified per subagent."

A session sees a subagent's brief and its report, not its context. One observation that a session
can make: a general-purpose subagent dispatched during the intake of these files reported that it had
received the project's root `CLAUDE.md` with the primer imported. Whether an installed agent type
receives it as well is not established.

### Why it matters

`design@agent-skills@premortem-as-watch-points` exists so that a failure of a decision is seen in
real sessions before any check could see it, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`. An unanswerable question gives
that decision no watch, while every retrospective spends a line on it.

### What would close it

A ruling between at least two shapes, and the edit of the question:

- ask what a session can observe: whether any subagent's report or behaviour showed that it lacked a
  rule of the primer;
- or make the delivery observable: the review skill's brief names the primer's path, so that a
  reviewer reads it whatever the harness does. That changes every brief.
