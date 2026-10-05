---
kind: defect
---
# Design heads were rewritten from a retrospective's proposals without the design skill

## Summary

On the branch `retrospective-2026-10-05-repairs`, two design heads were rewritten from a
retrospective's proposals and the owner's rulings on them. The design skill was not loaded, and the
session's transcript holds no thread for either decision before the commit that wrote it. This is
the firing of the tripwire that guarded `design@agent-skills@every-decision-through-design`, which
asks that a decision that earns a record goes through the design skill.

## Details

### What

The heads, in `path@agent-skills@docs/design.md`:

- `design@agent-skills@premortem-tripwires-on-the-owners-word`, rewritten for finding W1 of the
  retrospective of 2026-10-05, then again in that branch's review repairs, on a reviewer's finding;
- `design@agent-skills@in-change-path`, amended for finding W3 of the same retrospective.

How it went:

1. The session wrote the retrospective's two files and showed them to the owner. Each finding
   carried a proposal; W1's proposal was that a later change to a tripwire's firing evidence is a
   material finding.
2. The owner ruled on each finding in one message. For W1 the ruling replaced the proposal with
   another shape; for W3 it chose one of the two shapes the proposal offered.
3. The session wrote the heads and the skill text, and minted the thread slugs
   #tripwire-wording-is-the-agents and #in-change-waits-for-premortem inside the commit message.
   No turn had shown them to the owner. The session did not load the design skill.
4. The branch's transcript reviewer reported the tripwire as not judged. The session judged that it
   fired.

Reproduce: in that session's transcript, search for the two slugs. Their first occurrence is the
commit's tool call. Search for a load of the design skill after the owner's message that starts
"Let's repair or record those while they are fresh in memory."; there is none.

What the session did have: a written proposal with its rival for each finding, and the owner's
ruling in the owner's words, both quoted in the commit message. Whether that counts as the argued
discussion `design@agent-skills@every-decision-through-design` asks for is part of what reopening
it decides.

### Why it matters

`design@agent-skills@every-decision-through-design` delivers its rule through the design skill's
description and a backstop in the decision-recording skill. Neither stopped a session that had
just written that rule from writing two heads without the design skill. Its premise, that the
trigger reaches a session before it writes the decision, failed in its first test. That is against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`, unless a ruled retrospective
proposal is enough argument.

### What would close it

`design@agent-skills@every-decision-through-design` reopened on where its trigger is delivered,
with the rejected primer line among the candidates, and decided. The decision also says whether a
retrospective's proposal with the owner's ruling counts as the argued discussion. Either the
trigger is moved and built, or the head is rewritten to admit that case.
