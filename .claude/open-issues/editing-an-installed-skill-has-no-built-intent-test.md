---
kind: todo
---
# Editing an installed skill has no test against the agent-skills design heads and the goals

## Summary

The section of crates/agent-skills/CLAUDE.md on editing an installed skill gives three tests:
scope, necessity and kind. None asks whether the edit narrows or contradicts a design head or
strains a goal. In the session of the retrospective of 2026-10-04 (finding P1), an edit of the
setup skill passed all three and still diverged from two heads and strained a goal.

## Details

### What

The edit offered a Rust project of one package with no workspace a local install of the checker.
It narrowed `design@agent-skills@exact-pin` and `design@agent-skills@gates-convention`, which then
stated the maintenance crate as universal, and it strained
`goal@knowledge-architect@setup-brings-quality-tools`, since such a project gets no gates command.
The session had not looked: the decision-record and self-consistency reviewers found the
divergence, and the transcript reviewer found that the goal had no outcome. The heads were repaired
on the branch, and the goal conflict went to the owner. The workflow's side of the same gap is
`issue@agent-skills@a-skill-edit-is-not-checked-as-a-reversal`.

### Why it matters

`design@agent-skills@design-home-is-built-intent` makes the design home authority over the shipped
text. An edit that contradicts a head without the session knowing ships a divergence, which a
review catches only when a reviewer reads the head, against
`goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A fourth test in that section of crates/agent-skills/CLAUDE.md, such as: grep the agent-skills
design home and the goals for the behaviour the edited passage describes; an edit that narrows or
contradicts a head is a decision, under `knowledge-architect-decision-recording`. Or the owner's
ruling that the workflow's side, once built, makes it redundant.
