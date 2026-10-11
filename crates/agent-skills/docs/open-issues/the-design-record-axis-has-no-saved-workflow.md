---
kind: todo
---
# The design-record axis has no saved workflow, and its passes still read every draft by hand

## Summary

`skill@knowledge-architect-project-audit` gives every audit axis a saved workflow whose one run
per pass delivers a report the session verifies, per its method. The agentic-workflow axis has
one, `knowledge-architect-agentic-workflow-audit`. The design-record axis has none: it runs the
manual points of the method, its session sorting every draft between the agents' work and the
owner list. Building its workflow is design work of its own, which the owner set apart when the
generic shape was decided.

## Details

### What

The design-record axis, `skill@knowledge-architect-project-audit@design-record-axis`, dispatches
`agent@knowledge-architect-design-record-auditor` over groups of at most 60 entries of one
Component, adds a shared calibration sample to every group, and has the session settle the sample's
readings, sort every draft that proposes a change, and read a sample of the conforming ones. Its
workflow's design must decide, at least:

- how the script orders a Component's entries and cuts its groups, which the agent's description
  and the section state today;
- where the calibration sample is compared and its readings settled, when no session reads between
  the steps;
- what a draft per entry becomes in a report: one section per Component, per kind of failure, or
  per entry;
- the verify step: whether the design-record auditor verifies another auditor's verdicts, as the
  workflow auditor does for its axis, and what it checks against the history of the owner's
  rulings each head cites;
- the report's parts, and what the session reads after the run;
- what stays in the axis's section of the skill, and what moves into the script, per the generic
  shape.

### Why it matters

The audit skill's method gives every axis the same shape, the decision
`design@agent-skills@audit-axis-runs-a-workflow`, and the design-record
axis is the one left on the manual points. Its sort is the session reading of every draft that the
agentic-workflow axis's first run measured as costing a session most of its context.

### What would close it

A saved workflow for the design-record axis, its section of the skill brought to the generic shape,
and one real pass of the axis through it.
