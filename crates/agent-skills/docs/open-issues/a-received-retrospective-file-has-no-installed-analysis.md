---
kind: question
---
# A project that receives a retrospective's findings has no installed procedure to analyse them

## Summary

The installed retrospective sends each finding the owner keeps straight to an issue entry, with no
step that checks whether it still applies, whether it is already recorded, whether it is in scope,
or whether a better fix exists. This repository built that step as its own skill,
`klarch-retrospective-intake`, and every check of it is one any project would need. Should the
method become part of the installed workflow, and in which shape?

## Details

### What

`path@agent-skills@content/skills/retrospective/SKILL.md`, §4, "What becomes of each file": "each
finding the owner keeps becomes an issue entry in the project's own register". A finding fixed at
once needs no entry, and the owner decides with no analysis in front of them.

The local skill, at `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`, establishes
for each finding what it says, whether it still applies, whether it is recorded, its kind, its
scope, a better fix, its route and a proposal, and offers three outcomes: handle now, open an
issue, or no change with the reason. Its first use analysed five received files, 40 findings in
all. Among them, it found four already repaired, three already recorded, two whose premise was false
because a checked form already existed, two whose report claimed a repair the history places
earlier, and three whose proposed fix would have reversed a recorded head.

What is this repository's own in the skill: the files it takes in scope, the home of the analysis
under docs/retrospective-reports/, and the commit lifecycle of that file. The checks are not.

### Why it matters

`goal@knowledge-architect@the-workflow-improves-through-real-use` is met only while findings are
judged on evidence before they change the workflow, and `goal@knowledge-architect@the-owner-decides`
asks that the owner rule with the facts in front of them. A consuming project receives its own
project file's findings with neither.

### What would close it

A design discussion under `knowledge-architect-design` on the installed shape: a section of the
retrospective skill, or an installed skill of its own for the session that receives the files; where
a consuming project keeps the analysis, given that the primer's knowledge table has no row for it;
and whether the analysis home and its lifecycle ship or stay each project's own. Then the change, or
the owner's ruling that the method stays local.
