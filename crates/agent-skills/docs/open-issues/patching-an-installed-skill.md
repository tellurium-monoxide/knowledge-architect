---
kind: deferred
---
# A project cannot change one instruction of an installed skill

## Summary

An installed skill is compared byte for byte with the version the project pins, so a project
cannot edit it. A project adds to it through a skill of its own. A project whose need is to change
or remove one instruction of an installed skill has no means, and may give up on the installed
workflow altogether.

## Details

### What

A means for a project to change an installed skill in one place and keep the rest: for instance a
patch stored as a diff in the project, applied by the install and checked by the check, so that an
upgrade either applies it again or reports that it no longer applies. Nothing about the shape is
decided.

### Why it matters

`design@agent-skills@overlay-by-separate-skills` lets a project add, never contradict. A project
whose need is a contradiction is left two moves: report it to this repository, or declare
`harness = []` and lose the installed workflow, the checks of `design@core@owned-namespace-check`
and the overlay rule with it. That threatens `goal@knowledge-architect@any-project-can-adopt-it`.
The retrospective asks about it at every session, per
`design@agent-skills@premortem-as-watch-points`.

### Trigger

A retrospective's workflow file from a project that uses the workflow, filed as an issue on
knowledge-architect's repository or received and analysed under `skill@klarch-retrospective-intake`,
per `design@agent-skills@retrospective-destination`, names a session that needed to change or remove
an instruction of an installed skill for its own project. A session of this repository does not meet
it: here the installed text is edited at its source, and the repository follows the published
version without deviation, so that its own sessions test it. The session that triages that report decides what the workflow
changes, and this means is one of the answers it weighs.
