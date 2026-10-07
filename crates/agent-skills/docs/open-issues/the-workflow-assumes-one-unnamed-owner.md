---
kind: design
---
# The workflow assumes one owner, unnamed, which a project with several contributors does not have

## Summary

The installed text routes every decision to "the owner", a single person it never names. A project
with several contributors has several people who rule, and a record that says "the owner ruled"
does not say who. The owner's direction: refer to contributors by name in a project's documents.
Raised from finding W4 of 2026-10-07-knowledge-architect-workflow.

## Details

### What

The instance: two reviewers referred to the owner with a gendered pronoun ("Ask him", "his words"),
and the session copied one into a commit message before it was caught. The installed text itself
writes "the owner" and the singular "they", as "Their explicit word" in
`path@agent-skills@content/skills/design/SKILL.md`, and never a gendered pronoun. No installed text
says how to refer to the owner.

The owner ruled that the pronoun is part of a larger problem, in their words: "This one is part of
a larger problem, which is that this workflow expects an owner (unnamed). This cannot work under a
multi contributor project. This should become an issue about the larger problem, and not handled
now. I have a solution in mind, but I'll detail it in the design discussion when handling this
issue. The goal would be to refer to contributors by name in project docs."

Where the assumption is stated: the expectation set of the design skill, in
`path@agent-skills@content/skills/retrospective/SKILL.md`, "The owner is the person who can
decide"; and `goal@knowledge-architect@the-owner-decides`.

The suspected mechanism, in one sentence: the workflow models authority as one anonymous role, so
nothing in its text or its record can say which person ruled.

### Why it matters

`goal@knowledge-architect@the-owner-decides` promises that the record shows the owner's decisions.
With several contributors, a ruling recorded as the owner's cannot be traced to the person who gave
it, and `goal@knowledge-architect@any-project-can-adopt-it` does not hold for such a project.

### Re-entry

A design discussion raised on the owner's word, in which the owner details the solution they have
in mind.

### What would close it

A decision on how the workflow names the people who rule, recorded and built in the installed text.
