---
kind: question
---
# A project that uses the checker without the workflow still carries every workflow home

## Summary

The required document set is the same for every project: each Component owes its design, goals and
tripwires homes, its rejected alternatives and its issue directory, and the root owes the plans
directory. A project that adopts the checker alone, with no agent harness, carries all of them,
empty where it has nothing to put there. Whether that set should shrink for such a project is open.

## Details

### What

`goal@knowledge-architect@any-project-can-adopt-it` says "A project without agents can still use
the checker alone". Under `harness = []`, `design@core@agents-table` removes only CLAUDE.md from the
set; every register home stays required, per `design@core@components-carry-the-same-documents`,
and docs/plans/ with its README, specs/ and milestones/ stays required at the root, per
`design@core@plan-register`. An empty home passes, so the cost is files to create, not entries to
write. On rust-lang/log at commit 27e3cf7a, an adoption by a fresh agent from the published
documentation alone created 11 hand-written files and 3 generated indexes for a project of one
Component; 402 of the hand-written words were the required structure.

The owner judged the cost low: a project can leave the homes empty and ignore them. This entry
keeps the question open, not a defect.

### Why it matters

The set is what the word Component means, per `design@core@components-carry-the-same-documents`,
so a project free to declare its own set would be checked against what it declared. The other
side is `goal@knowledge-architect@adoption-is-easy`: each required file is a step between a project
and its first passing check, and the plans directory serves the workflow's planning activity, which
a project without agents never runs.

### What would close it

Either the owner rules that the set stays as it is for every project, which removes this entry,
or a design discussion under `knowledge-architect-design` settles what a project without the
workflow owes. Evidence that would move it: a project that declines adoption, or asks for an
exemption, because of the empty homes.
