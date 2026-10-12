---
kind: design
---
# A tripwire on a procedure of the workflow states no bound on how much the failure must matter

## Summary

The tripwire shape asks for firing evidence "specific enough that both parties would agree it
fired", and nothing about how much the failure must matter. A tripwire that guards a procedure of
the workflow then fires, by its letter, on a slip that is cheap to repair, and its response
reopens a recorded decision. The owner ruled one such firing out, and had that one tripwire bounded
to a critical gap; four others keep the unbounded shape.

## Details

### What

Instances seen:

- During the synthetic-evidence branch, a review found a stale restatement, one sentence listing
  the tripwire rulings but not the acceptance-criterion rulings, in the work of a spec that had
  skipped its design audit. The standing-state reviewer reported the tripwire guarding the skipped
  audit as plausibly fired, with "fired" as its default, which would reopen the rule that lets a
  spec skip its audit. The owner ruled it not fired: "The point of this kind of tripwires is to
  guard against critical failures in the workflow, not small mistakes that are not load bearing.
  We won't make significant change to the workflow over nitpicks like this." The change "The
  tripwire on a skipped spec audit fires only on a critical gap, on the owner's word" bounded
  `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`: "A gap is critical when the
  work's result, or a decision it rests on, is wrong without its repair."
- `tripwire@agent-skills@ruling-lost-in-assembly` carries a bound too, through
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes`.
- Four tripwires on a procedure still fire on one instance with no bound:
  `tripwire@agent-skills@ruling-lost-in-change`, the sibling of the bounded one on assembly, which
  watches the same failure, a misstated ruling of the owner;
  `tripwire@agent-skills@member-beyond-the-argument`;
  `tripwire@agent-skills@rule-title-wider-than-its-argument`;
  `tripwire@agent-skills@pointer-not-followed`. None of the four has been misread yet: their
  exposure is a reading of their text, not an observed firing.
- The shape is the issue-tracking skill's section on a tripwire entry, "**the firing evidence**,
  meeting the falsifiability bar: an event, or a count crossing a bound, specific enough that both
  parties would agree it fired", restated by the design skill's premortem. The standing-state
  reviewer asks of each tripwire "has it fired?", and weighs nothing.

The suspected mechanism, in one sentence: the falsifiability bar makes a firing clause precise
about the event and silent about its weight, so a procedure that fails a little fires as loudly as
one that fails badly.

Re-entry point: the next change to the tripwire shape of the issue-tracking skill, or to any of
the four tripwires above, or earlier on the owner's word. The discussion then states the bound in
the shape and puts each of the four tripwires to the owner, each having been written on the
owner's word.

### Why it matters

A firing reopens a decision, so a tripwire that fires on a nitpick costs the owner a ruling, against
`goal@knowledge-architect@the-owner-decides`. The owner's words above state the rule for "this kind
of tripwires", and every unbounded tripwire on a procedure repeats the ruling until the shape says
it. The shape is the issue-tracking skill's, and each tripwire changed is a ruled tripwire, per
`skill@knowledge-architect-decision-recording@ruled-tripwires`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that either states in the tripwire
shape that a tripwire on a procedure names the failure that matters, as the bounded tripwire above
does, and brings each of the four tripwires to the owner with a bound; or keeps the shape and
records why, with the standing-state reviewer weighing each firing instead.
