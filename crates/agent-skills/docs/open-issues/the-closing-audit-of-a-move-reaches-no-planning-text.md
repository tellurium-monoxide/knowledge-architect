---
kind: question
---
# The audit that ends the move of an adopting project's documents is stated where the move's sessions do not read

## Summary

The setup skill says the milestone that moves an adopting project's existing documents ends by
running the design-record axis of the project audit. That obligation is written in the setup skill
and in the audit skill's description. The `todo` issue the setup opens for the move holds the
inventory and the rulings, not the audit, and the planning skill names no project audit. A session
that lands the move's last slice under the planning skill may delete the milestone with no audit
run. This is a prediction about agent behaviour: no real session has shown it.

## Details

### What

Texts read, in the installed set at the commit the first agentic-workflow audit ran on:

- `skill@knowledge-architect-setup@existing-documentation`: "Open one `todo` issue for the move, in
  the root Component, holding the inventory and the rulings" and "The milestone ends by running
  the design-record axis".
- `skill@knowledge-architect-planning`: its sections, the procedure of a slice and the deletion of
  a plan document name no project audit.
- `skill@knowledge-architect-project-audit`: its description and
  `skill@knowledge-architect-project-audit@when-an-audit-runs` name the occasion.

Reported by the L2 activity walk of setup, pin move, goal setting and agent configuration, in the
first run of the agentic-workflow axis, as a predicted gap.

### Why it matters

`design@agent-skills@audit-triggers` names the end of an adopting project's move as an occasion of
the design-record axis. A move milestone that leaves without that run leaves the project's record
under rules it was never brought to, against `goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

A real session that lands a move milestone, read by its retrospective: either it ran the audit,
and the entry closes, or it did not, and the setup's `todo` issue then carries the closing audit as
the milestone's last step, so that the planning of the milestone writes it into the milestone
document.
