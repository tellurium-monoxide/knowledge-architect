---
kind: question
---
# After a version upgrade, nothing re-reads a project's restatements of installed directives, a project skill whose activity an installed skill now covers, or the session's own copy of the new text

## Summary

The agent-configuration skill's steps after an install read the primer's knowledge table against
the project's rows, and each project skill and agent against the installed text the install
changed. None reads the sentences of the project's root CLAUDE.md that restate an installed directive
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

A second case of the same step list, reported by the L6 lens of the third pass, also a predicted
gap: a new version whose installed skill now covers the activity a project skill covers. The steps
read each project skill and agent against the installed text that changed, for a contradiction; an
overlap that contradicts nothing, two skills for one activity, is read by no step.

A third case of the same step list, reported by the L2 walk of the fifth pass, also a predicted
gap: the session that moves the pin keeps its own copies of the primer and of
`skill@knowledge-architect-setup@moving-the-pin` from before the install. Steps 4 and 5 and the
review are then followed from the old version's text, such as the old `primer@branch-commits`.
No step of `skill@knowledge-architect-setup@moving-the-pin` or of
`skill@knowledge-architect-agent-configuration@after-installing` says to read the new text from disk after
the install. The subagents the session dispatches are covered: `skill@knowledge-architect-review@review-invariants`
tells the session to have them read a file that changed on disk after the session started.

An observation of the same harness fact, from real use and not from a pin move: in the session that
ran that fifth pass, the root CLAUDE.md delivered to the session, and the one delivered to the
saved workflow's cross-check agent, both held a "routing table" row under the Skills section that
the root on disk at the commit audited no longer held. The copy delivered was older than disk.

### Why it matters

`design@agent-skills@restatement-size-test` keeps a restatement only beside its pointer, on the
premise that a drift is found where the two disagree. A drift no step reads stays in every session's
context, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A retrospective of a session that moved a pin across a version that reworded a restated directive:
either the session found the restatement and repaired it, and the entry closes, or it did not, and
the after-installing steps gain one that reads the root CLAUDE.md's restatements against their
homes. The second case closes the same way, on a session that moved a pin across a
version whose new installed text covers a project skill's activity. The third case closes on a
retrospective of a session that moved a pin across a version that changed the primer or the
moving-the-pin steps: either it read the new text after the install, or step 3 gains a clause to
re-read the primer and the moving-the-pin section from disk before step 4.
