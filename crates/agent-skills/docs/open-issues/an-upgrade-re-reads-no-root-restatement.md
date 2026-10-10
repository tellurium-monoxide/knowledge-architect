---
kind: question
---
# After a version upgrade, nothing re-reads a project's restatements of installed directives in its root CLAUDE.md

## Summary

The agent-configuration skill's steps after an install read the primer's knowledge table against
the project's rows, the routing table, and each project skill against the installed skill it adds
to. None reads the sentences of the project's root CLAUDE.md that restate an installed directive
with a pointer beside it. After an upgrade that rewords such a directive, the restatement can
disagree with its home and no step looks. This is a prediction: no real session has shown it.

## Details

### What

Texts read, in the installed set at the commit the second pass of the first agentic-workflow audit
ran on:

- `skill@knowledge-architect-agent-configuration@after-installing`: its steps after an install.
- `skill@knowledge-architect-setup@moving-the-pin`: it hands to those steps.
- `primer@where-knowledge-goes`: "A directive sentence carries its pointer beside it, and where the
  two disagree the restatement is the defect."

`cargo klarch check` reports a pointer whose slug a new version removed, not a pointed-to sentence
whose wording changed. The input exists, the installed diff and the changelog's Workflow entries,
so judgement can find the drift; whether sessions do is not established. Reported by the L2
activity walk of setup, pin move, goal setting and agent configuration, as a predicted gap.

### Why it matters

`design@agent-skills@restatement-size-test` keeps a restatement only beside its pointer, on the
premise that a drift is found where the two disagree. A drift no step reads stays in every session's
context, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A retrospective of a session that moved a pin across a version that reworded a restated directive:
either the session found the restatement and repaired it, and the entry closes, or it did not, and
the after-installing steps gain one that reads the root CLAUDE.md's restatements against their
homes.
