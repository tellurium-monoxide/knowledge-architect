---
kind: question
---
# The refusal of a kind's name as an anchor quotes a reason the root forbids

## Summary

`design@core@kind-names-refused-as-anchors` refuses every kind's name for an anchor, and
quotes the owner's ruling as its ground. The ruling's reason is a future migration, the kind of
reason `design@knowledge-architect@no-future-breaking-cost-argument` says argues against nothing.
The head also carries an argument of its own, the shadowing of a reference's head, so the question
is which of the two the head stands on.

## Details

### What

The head, in `path@core@docs/design.md`, argues
first: "A reference's head is read as a kind before it is read as an anchor, so an anchor named
`design`, `path` or `skill` would be shadowed there, and the old form written with it would be
misread." It then quotes: "The owner, on the question: "My ruling on this is that all kind names
should be refused for anything that can be an anchor name (components, custom locations...). I'd
rather make this decision early to avoid painful migrations.""

`design@knowledge-architect@no-future-breaking-cost-argument` states: "An argument that a change is
expensive names the cost it has today, a consumer's migration included. [...] a cost that exists
only in a future where changes are refused argues against nothing".

The ruling came from the owner, after a review found that the shadowing reaches every kind and the
agent had proposed an issue entry for it; the commit that put the ruling into the milestone
agent-configuration-entities says "The issue opened for the shadowing of the existing kinds was
never committed: the ruling puts that work in the milestone."
So under `design@agent-skills@head-ground-is-the-argument`, which cites the owner's words only for
what came from the owner, the quotation is a legitimate ground of provenance. Its stated reason is still the forbidden one.

Not established: whether the owner meant the migration cost of today's consumers, which the root
head admits as a present cost, or a cost in a future where changes are refused.

### Why it matters

A head that quotes a reason `design@knowledge-architect@no-future-breaking-cost-argument` refuses
teaches a later session that the reason is admissible, which strains that decision. A session
weighing a change to the refusal cannot tell whether it must defeat the shadowing argument, the
owner's intent, or both.

### What would close it

The owner's ruling, recorded in this entry's closing commit, on one of:

- the quotation stays as the provenance of the refusal, and the head says the decision stands on
  the shadowing argument;
- the quotation is reworded to the present cost the owner meant, if the owner meant one;
- the reason is kept as a deliberate exception to
  `design@knowledge-architect@no-future-breaking-cost-argument`, which that head then states.
