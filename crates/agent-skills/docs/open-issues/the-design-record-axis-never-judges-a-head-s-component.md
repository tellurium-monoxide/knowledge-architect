---
kind: todo
---
# The design-record axis puts a head in the wrong Component on the owner list, and none of its rules judges a head's Component

## Summary

The project-audit skill's design-record axis sends "a head in the wrong Component" to the owner
list, but the axis's rules are only `primer@design-heads` and the rules on losing alternatives, and
its auditor neither reads nor judges which Component owns a head. So no step of the axis can
produce the finding its outcome table names. The routing reviewer judges a head's Component, by
`skill@knowledge-architect-decision-recording@owning-component`, but only for a head a diff writes.
Filling the gap adds an instruction to the axis, which is a change of
`design@agent-skills@design-record-axis` argued under `skill@knowledge-architect-design`.

## Details

### What

The texts, as installed at the commit "The project audit counts convergence on old text, checks
each repair against the record, and commits each pass's report file":

- `skill@knowledge-architect-project-audit@design-record-axis`, its outcome table, the row "put to
  the owner, on the owner list", ends: "a ground that is the owner's words; a head in the wrong
  Component".
- The same section's rules: "**The rules**: every rule of `primer@design-heads` for a head, and
  every rule of `skill@knowledge-architect-decision-recording@losing-alternatives` for a rejected
  alternative."
- `agent@knowledge-architect-design-record-auditor@read-the-rules` lists `primer@design-heads`,
  the losing-alternatives section, `skill@knowledge-architect-decision-recording@three-homes`, and
  the audit skill's outcomes and design-record-axis sections. Its list of what it judges for a
  head names every rule of `primer@design-heads` and nothing about the Component. Its draft names
  each failed rule "by the section of the rule", so a verdict citing the owning-component section
  has no rule in its read list to rest on.
- `agent@knowledge-architect-routing-reviewer`, the question "Does a decision sit in the right
  Component?": "Judge it by the three questions of
  `skill@knowledge-architect-decision-recording@owning-component`, read whole". That reviewer
  reads a diff, so a misfiled head that no branch touches is read by no step.
- `primer@design-heads` holds what earns a head, what a head holds and how it is shaped; it does
  not say which Component owns a head. That is
  `skill@knowledge-architect-decision-recording@owning-component`.

`grep -rn owning-component` over `path@agent-skills@content/` finds the section only in the
decision-recording skill, which defines it and cites it in its reversal check, and in the routing
reviewer: neither the audit skill nor the design-record auditor names it.

Reported by the L8 lens of the sixth pass of the agentic-workflow audit as a provable gap: the
draft offered no head of this project shown to sit in the wrong Component, so the two standards'
verdicts were not shown to differ on a real head; the gap is that the axis names an outcome its
rules cannot produce. The owner ruled it parked as this entry, F119 of that pass, as F96 of the
fifth pass parked a provable gap on the installed text.

**The candidate repair**, three edits of the installed text under
`path@agent-skills@content/`, adding one instruction:

1. the axis's rules in `skill@knowledge-architect-project-audit@design-record-axis` become "every
   rule of `primer@design-heads` and the questions of
   `skill@knowledge-architect-decision-recording@owning-component` for a head, and every rule of
   `skill@knowledge-architect-decision-recording@losing-alternatives` for a rejected alternative";
2. `agent@knowledge-architect-design-record-auditor@read-the-rules` gains the line
   "`skill@knowledge-architect-decision-recording@owning-component`, for the Component a head
   belongs to;" after the line for `primer@design-heads`;
3. the auditor's bullet that judges a head against every rule of `primer@design-heads` gains the
   sentence "Judge its Component by
   `skill@knowledge-architect-decision-recording@owning-component`." before "A head that argues".

The rival repair adds nothing: remove "a head in the wrong Component" from the outcome table's
owner-list row. It reverses a member of `design@agent-skills@audit-outcomes`, and leaves a
misfiled head that no branch touches read by no step, the gap
`design@agent-skills@audit-is-an-activity` argues an audit exists to close.

Checked against the record when the repair was proposed: neither repair contradicts
`skill@knowledge-architect-decision-recording@reversal-check`, whose "If the incumbent looks
misfiled, that is a separate change with its own argument" an audit's owner-list item is.

### Why it matters

`design@agent-skills@audit-outcomes` sends "a head in the wrong Component" to the owner, per
`goal@knowledge-architect@the-owner-decides`, and `design@agent-skills@design-record-axis` states
the axis's rules as `primer@design-heads` and the losing-alternatives section, which do not include
the Component. The two heads disagree on what the axis judges. Until they agree, a design-record
audit leaves every misfiled head that no diff touches unreported, while its outcome table tells
the owner such heads are reported, against
`goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

A change of `design@agent-skills@design-record-axis`, argued under `skill@knowledge-architect-design`
before its text is written: the owning-component section is not a member the head's rule admits,
since its title and body name the rules on design heads and on rejected alternatives, and
`primer@design-heads` holds no rule on the Component, so adding it is a change of the decision and
not a member recorded directly. Then either the three edits above, with the head gaining the
section among the axis's rules, or the rival, with `design@agent-skills@audit-outcomes` losing the
member. A provable gap on the installed text is filled by an edit on the owner's word, per
`design@agent-skills@behaviour-additions-need-real-use`.
