---
kind: design
---
# The design-record cross-check misses a tripwire that a split or a retitle leaves false while it still resolves

## Summary

The design-record axis asks its cross-check for every tripwire and issue that an edit of the pass
would leave false. In a real pass, two tripwires on heads the pass split or retitled were left
false, the cross-check did not list them, and the standing-state review of the audit's branch found
them. Both still resolved, so the checker reported neither. This fired
`tripwire@agent-skills@audit-edit-leaves-an-entry-stale`, whose response reopens
`design@agent-skills@design-record-axis`; this entry carries that reopening.

## Details

### What

Instances seen, one pass: pass 1 of the design-record axis in thaum, on knowledge-architect 0.6.0,
reported by its retrospective's workflow file as W3:

- a tripwire whose firing evidence was a member that the retitled head now admits as open, so the
  tripwire fires on growth the head expects;
- a clause guarding a decision that a split moved to a new head, left under the old head's
  tripwire, so a reversal of the old head would delete the guard of the new one.

What the axis asks today:

- `skill@knowledge-architect-project-audit@design-record-axis`: the cross-check maps "the tripwires
  and issues an edit would leave false";
- the cross-check's prompt in `path@agent-skills@content/workflows/design-record-audit.js` asks for
  each tripwire and each issue that cites a head some verified finding edits, read whole, and
  whether the edit leaves it false.

The scope is instructed. A reading clause by clause, against the head that now holds what each
clause guards, is not.

The suspected mechanism, in one sentence: the cross-check judges a tripwire against the edited head
as a whole, so a clause whose guarded decision moved to another head, or whose firing member the
retitle made expected, reads as still true of the head it names.

Re-entry point: the next pass of the design-record axis, in this repository or one whose
retrospective reaches it, or earlier on the owner's word.

### Why it matters

`design@agent-skills@design-record-axis` rests on the premise, guarded by the tripwire that fired,
that reading each head's tripwires and issues before judging it keeps them true of the head the
audit edits. A tripwire left false either fires on expected growth, which costs the owner a ruling,
or guards nothing, which leaves its decision watched by no one, against
`goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that reopens
`design@agent-skills@design-record-axis` on the cross-check's reading of tripwires, among the rivals
the intake named: the cross-check reads every tripwire of every head the pass splits or retitles
clause by clause, against the head that now holds what each clause guards; or no change, with the
standing-state review of the audit's branch as the catch, which found both here. Then the change,
or the decision to keep the axis as it is, recorded in the head.
