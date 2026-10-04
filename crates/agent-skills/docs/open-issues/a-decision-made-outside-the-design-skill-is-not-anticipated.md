---
kind: design
---
# The workflow does not anticipate a design decision made outside the design skill

## Summary

The design skill carries the rules that keep an owner's word from being recorded wider, narrower
or in another state than given. A design decision taken from the owner's words during another
activity, such as an issue fix, meets none of them. Finding W1 of the retrospective of 2026-10-04.

## Details

### What

The instance: while fixing an issue, the owner said a skill "is not really meant to run without a
project owner", and that an item "probably needs the project owner to rule". The session wrote the
first into the setup skill, the retrospective's expectation set and a README as a hard rule, that
the setup stops when no owner is present, and dropped the hedge from the second in a commit
message. It then showed both to the owner inside a long report, and the owner's "agreed" answered
another question. The transcript reviewer found both, and the owner then ruled on each.

The design skill's rule that would have caught it: a word that requires interpretation "closes
nothing: mark the threads your reading would close as `presumed-settled`, state the reading, and
let their next word ... promote or correct" it. No installed skill carries an equivalent outside a
design discussion.

The suspected mechanism, the owner's, given with an "I think": the workflow does not anticipate
that a design decision is made outside the use of the design skill.

On the owner's ruling, this is not counted as an instance of
`issue@agent-skills@a-hedged-ruling-recorded-as-approved`: the owner believes it is separate. The
agent's note beside it: that entry is about assembling a plan document from a discussion.

### Why it matters

`goal@knowledge-architect@the-owner-decides` is met only while a decision recorded as the owner's
is the one the owner made. Outside the design skill, only the transcript review, before a merge,
catches a ruling recorded wider than given.

### Re-entry

The next design discussion about the decision-recording skill, or a second instance found by a
transcript review. One shape to argue there: in `knowledge-architect-decision-recording`, record the
owner's words verbatim beside the reading, and put any reading that adds an obligation or drops a
hedge to the owner as its own question before writing it.

### What would close it

A decision on where the rule for a decision outside a design discussion lives, recorded and built,
or the owner's ruling that the transcript review is the intended guard.
