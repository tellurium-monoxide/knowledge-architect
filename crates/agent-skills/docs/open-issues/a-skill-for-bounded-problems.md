---
kind: todo
---
# No installed skill covers a bounded problem

## Summary

A problem that arrives bounded, a clear requirement whose main risk is over-building, has no
installed skill. The design-discussion skill classifies it and leaves the next step to the owner,
per `design@agent-skills@bounded-problem-branch`.

## Details

### What

The design-discussion skill, `knowledge-architect-discussing-design-decisions`, applies to an open
solution space. Its "When NOT to use" section tells the agent to state the strongest open reading
of a bounded problem beside the bounded one, and to leave the next step to the owner. No installed
skill receives the problem from there. designing-together, the source of the fork, sent it to an
outside plugin skill, superpowers:brainstorming, which `design@agent-skills@no-external-handoff`
forbids.

What the owner has stated about the skill:

- it is installed as part of this project, under its own name;
- a bounded problem still needs investigation, testing, and the owner's approval, so the planning
  skill's spec alone is not enough for it;
- the owner intends to write it.

### Why it matters

`goal@agent-skills@one-skill-per-activity` is met while no activity of the workflow is left
without its skill. Handling a bounded problem is such an activity, and today a session that meets
one has no instruction past the classification.

### What would close it

An installed skill for bounded problems, written with the owner, and the design-discussion skill's
bounded branch naming it, with `design@agent-skills@bounded-problem-branch` rewritten in place.
