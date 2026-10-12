---
kind: design
---
# Whether a design head quotes the owner's words, or only attributes the decision to the owner

## Summary

`primer@design-heads` says "The owner's words are a ground, quoted, only where the decision came
from the owner". When ruling on pass 1 of the design-record audit of this repository, the owner
chose attribution over quotation for two items. The owner then questioned the rule itself: "I'm
tempted to remove this rule too. It's not particularly relevant IMO. It is weird to keep those
verbatim quotes in design heads anyway." That is a hedged position, so it is argued in a design
discussion and not applied.

## Details

### What

The owner's ruling on that pass: "Agreed on all defaults except F2 and F4: no need to quote my
words, as long as attribution is correct." F2 asked to quote the owner's scope ruling in
`design@knowledge-architect@versioning-policy`. F4 asked to quote the owner's request in
`design@core@milestone-entries-are-directories`. Each now attributes the decision to the owner
without quoting.

The rule appears in more places than the primer's sentence:

- test 4 of the entry tests, "The head quotes the owner's words and the owner's answer";
- the design-record auditor's failure mode about an owner quotation answering another question;
- the verifiers' check of a quotation of the owner against the question it answered;
- the many heads of this repository that quote the owner.

The pass's report file holds the item and its ruling, and
`git log --diff-filter=D --name-only -- docs/audit-reports/` finds it once the file leaves.

### Why it matters

The rule shapes every head whose ground is the owner's, and every audit pass reads heads against
it. `goal@knowledge-architect@the-owner-decides` is what the quotation serves: a later session can
tell what the owner said from what an agent inferred.

### What would close it

A design discussion under `skill@knowledge-architect-design` that keeps the rule, narrows it to
attribution, or removes it. If the rule changes, the audit standards that check quotations change
with it, and a Migration entry names the design-record axis.
