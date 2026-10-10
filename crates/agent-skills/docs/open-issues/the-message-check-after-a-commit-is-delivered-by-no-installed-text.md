---
kind: question
---
# The installed text delivers the check of a commit's tree before the commit, and no check of its message after it

## Summary

The checker judges a commit message only through `commits`, which runs after the commit exists.
The setup skill recommends `check --staged` before each commit for a project's root CLAUDE.md, and
lists `commits` among the gates before a merge only. A session of an adopting project may then meet
a dangling reference in a message first at the gates, when the cheap repair is gone. This is a
prediction: no real session has shown it.

## Details

### What

- `primer@where-knowledge-goes` states that the checker reads each commit's message and tree
  through `commits`.
- `skill@knowledge-architect-setup` recommends the staged check before each commit and lists
  `commits` among the gates.
- `path@core@README.md`, on `commits`: "the range is run after each commit, and a finding in the
  newest commit is repaired by an amend."

This repository's root CLAUDE.md runs `commits` after each commit; the installed text does not
recommend it to an adopting project. Reported by the L7 checker-rules lens of the second pass of
the first agentic-workflow audit, as a predicted gap.

### Why it matters

`design@agent-skills@staged-check-before-each-commit` argued the tree half on delivery; the message
half has no recorded lack, so under `design@agent-skills@additions-need-real-use` an addition waits
for one.

### What would close it

A retrospective that reports a message finding first met at the gates, in a project that follows
the installed text: then the setup's recommendation gains "and `{command} commits <main>..HEAD`
after it", the command as the project declares it. Or retrospectives over several releases that
show no such case, and the entry closes.
