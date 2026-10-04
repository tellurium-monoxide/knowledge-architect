---
kind: todo
---
# An edit of skill text is not checked as a possible reversal of a recorded decision

## Summary

The decision-recording skill's first question, whether a decision reverses something already
recorded, is asked when a session knows it is making a decision. An edit of skill text that
changes what a skill tells a project to do is a decision too, and nothing frames it as one. Finding
W2 of the retrospective of 2026-10-04.

## Details

### What

The decision-recording skill, §1: "Ask this first ... Does it reverse something already
recorded?" In the session of that retrospective, the search was run for a change to a design head
of the root Component, and not for an edit of the setup skill that narrowed two heads of this
Component. The project's side of the same gap, in the scoped CLAUDE.md of this crate, is
`issue@agent-config@editing-an-installed-skill-has-no-built-intent-test`.

### Why it matters

`design@agent-skills@design-home-is-built-intent` holds only while an edit of the text the heads
describe is checked against them. A project that edits its own skills meets the same gap, and its
heads drift from its skills without a finding, against
`goal@agent-skills@one-skill-per-activity`, which is met only while no two installed texts
contradict.

### What would close it

A line in `knowledge-architect-agent-configuration`, which owns the editing of skills: an edit
that changes what a skill tells a project to do is checked against the design home and the goals
first, under §1 of `knowledge-architect-decision-recording`. Or the owner's ruling that the
reviews are the intended check.
