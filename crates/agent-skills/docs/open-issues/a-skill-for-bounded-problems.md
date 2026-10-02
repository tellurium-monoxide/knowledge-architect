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

- "The bounded problem skill will be installed as part of this project."
- "The bounded problem still needs some sort of investigation, testing, and approval from the
  owner", which is why it does not go to the planning skill, per
  `design@agent-skills@bounded-problem-branch`.
- "Ultimately, I'd like to make my own skill for this use case."

A lead for the skill: a review of designing-together predicted that "a second defensible shape is
nameable for nearly any request", which would make the design-discussion skill's open-space test
classify almost every problem as open, and leave the bounded case nearly unreachable. No session
has shown it. The classification the new skill receives is where it would show.

### Why it matters

The owner has decided that the skill is installed. Until it is, a session that meets a bounded
problem has no instruction past the classification, and the next step depends on the owner's word
each time.

### What would close it

An installed skill for bounded problems, written with the owner, and the design-discussion skill's
bounded branch naming it, with `design@agent-skills@bounded-problem-branch` rewritten in place.
