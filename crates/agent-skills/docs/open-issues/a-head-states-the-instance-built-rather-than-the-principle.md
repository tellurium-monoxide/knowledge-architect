---
kind: design
---
# A design head can state the instance that was built as if it were the rule

## Summary

The owner observes that design heads are often written as a description of what was built, and
not as the principle that decided it. The head then reads as a narrower rule than its argument
supports, and a later change that its principle admits must reverse it.

## Details

### What

**The instance seen.** `design@core@safe-fix-definition` admits a fix when "its bytes are
determined by the tree and the pinned version, and it writes or removes only files of the
installer's namespace or of the generated list". Its argument states the principle: "A fix that
makes a choice, or touches git or a hand-written file, would rewrite what a writer meant." The
second clause of the title is the list of the two fixes that existed when it was written. A fix that
rewrites the spelling of a reference, and not its meaning, meets the principle and fails the
clause, so the design discussion of `spec@plans@path-quickfixes` plans the reversal of the head to
admit it.

**The owner's words**, in that discussion: "I think the current design heads are overstated on the
admissible shape, I suspect they were written as a description of what was done and not as a
generic principle. I think this happens quite often in how design heads are written. It's not too
bad of an issue because reversing design heads is not forbidden by the workflow, but it's often
bothering me."

**Suspected mechanism**: a head is written at the harvest of the work that built it, from that
work's shape, and `skill@knowledge-architect-decision-recording` asks for the decision and its
standing argument but not for a separation of the principle from the instance that implements it.

Other instances are not established. A search of the design homes for a head whose title is a list
of what exists while its argument states a wider rule would establish the frequency.

### Why it matters

`design@agent-skills@standing-argument-in-head` makes a head carry "every premise whose failure
would reopen it". A head that states the instance as its rule is reopened by a change that defeats
no premise, which costs a reversal for an ordinary extension, and makes a session read a head as
forbidding what its argument allows. It strains
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: the record shows the built shape
in the place of the argued one.

### What would close it

A ruling on whether `skill@knowledge-architect-decision-recording` asks a head to state its
principle apart from its current instance, and how: for instance, a principle in the title and the
current instance as a consequence in the body. Or the owner's ruling that heads stay as they are,
recorded in this entry's closing commit.

### Re-entry point

A design discussion under `skill@knowledge-architect-design`, opened on the owner's word, whose
grounding includes the search above.

The discussion has run and converged: `spec@plans@head-rules` plans the work that answers this
entry, and the commit that completes that work's harvest closes it.
