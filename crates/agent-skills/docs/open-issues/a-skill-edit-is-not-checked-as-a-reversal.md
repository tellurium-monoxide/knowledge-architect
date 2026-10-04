---
kind: todo
---
# An edit of instruction text is not framed as a decision that may reverse a recorded one

## Summary

The decision-recording skill's first question, whether a decision reverses something already
recorded, is asked when a session knows it is making a decision. An edit of instruction text, a
skill or a CLAUDE.md, that changes what an agent is told to do is a decision too, and no installed
skill frames it as one. Finding W2 of the retrospective of 2026-10-04.

## Details

### What

The decision-recording skill, §1: "Ask this first ... Does it reverse something already
recorded?" On the branch whose commits include "A stricter check is minor even when a project must
change its content to pass it" and "Answer the five places where a fresh adoption of 0.2.0 was
left guessing", the session ran that search for the first, a change to a design head, and not for
the second, an edit of the setup skill that narrowed two heads of this Component. The session had
loaded the decision-recording skill for the first change; nothing in it told the session that the
second was a decision.

The instance is an edit of this repository's shipped text, which crates/agent-skills/CLAUDE.md
governs; the project's side of the gap is
`issue@agent-config@editing-an-installed-skill-has-no-built-intent-test`. The workflow's side is
general: any project's session that edits its own skills or CLAUDE.md meets the same framing gap.
That general case has no observed instance of its own yet.

### Why it matters

`design@agent-skills@design-home-is-built-intent` holds only while an edit of the text the heads
describe is checked against them. Without that, a project's heads drift from its instructions with
no finding, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A line in §1 of `knowledge-architect-decision-recording`, which governs any decision: an edit
of instruction text that changes what an agent is told to do is a decision, and is checked
against the design home and the goals first. Or the owner's ruling that the reviews are the
intended check.
