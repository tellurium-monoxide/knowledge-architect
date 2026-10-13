---
kind: question
---
# The project audit gives a rebase merge as the reason to cite a commit by its subject, in every installing project

## Summary

`skill@knowledge-architect-project-audit@record-of-an-audit` says the audit's report file and every
commit message of the audit "name a commit of the audit's branch by its subject, never by its SHA,
since a rebase merge gives the branch's commits new SHAs". The installed text is read in every
project that installs it, and nothing in the installed set establishes that a project merges by
rebase. Does the rule hold for a project that merges otherwise, and on what reason?

## Details

### What

In a project that merges with a merge commit, or fast-forwards, a branch's commits keep their SHAs
on the main branch, so the stated reason is false there. The instruction itself, naming a commit by
its subject, is followable in any project. Two census agents of the design discussion on what
enters agent-facing text reported it independently. This repository merges by rebase, per its root
`CLAUDE.md`, so the reason holds here. Not established: how the adopting projects known today
merge.

### Why it matters

`goal@agent-skills@installed-text-works-anywhere` asks that the installed text work unchanged in any
project. A reason that is false in a project leaves its agent unable to judge whether the rule
applies there, against `goal@agent-skills@installed-text-leaves-room-to-judge`, which gives each
instruction its reason so an agent can judge.

### What would close it

A decision, under `skill@knowledge-architect-decision-recording`, of whether the rule holds in every
project, with a reason true in each, or holds only where a merge rewrites SHAs, as the installed text
then says; and the edit of `skill@knowledge-architect-project-audit@record-of-an-audit` that
follows.
