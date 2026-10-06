---
kind: question
---
# The issues and tripwires a piece of work bears on are searched in the main session, or first at the review

## Summary

Before work is designed or planned, the session itself reads every issue and tripwire against the
work. Work that is neither designed nor planned meets its tripwires first at the standing-state
review, before the merge, and a `deferred` issue whose trigger it meets at no step: that reviewer
reads every tripwire, and only the issue entries the change opens or closes. Should a subagent do the search earlier, after a plan document is written and
during the grounding of a design discussion, and report what the work bears on? Raised from finding
W3 of thaum's retrospective of its move to knowledge-architect 0.3.0.

## Details

### What

Two installed procedures read the standing entries before work starts, both in the main session:

- the grounding step of `knowledge-architect-design` runs `issues` and `tripwires`, and shows each
  entry the question bears on;
- the design audit of a milestone step, §7 of `knowledge-architect-planning`, reads every tripwire
  and every `deferred` issue against the step.

The plan document reviews of §8 of `knowledge-architect-planning` send three fresh reviewers. None
of them reads the tripwires homes. The standing-state reviewer reads them before every merge, and
its scope is written for a diff: "every issue entry the change opens or closes".

The instance: in thaum, the move of the pin to 0.3.0 was work that was neither designed nor
planned. A `deferred` issue's trigger was "The next move of the pinned knowledge-architect
version". The session did the whole move without reading it. The standing-state reviewer found it
before the merge, though its procedure does not ask it to read such an issue, and the owner ruled
on it then. The retrospective proposed a step specific to the pin move. The owner dropped that, since it would mean anticipating every trigger condition for
every kind of action.

The owner's position, in their words: the search "is supposed to be either done in grounding steps
of design work, or by the standing state reviewer", and "maybe a standing state reviewer (or
similar) should be dispatched earlier in the workflow, either after plan writing, or during
grounding steps, with the task of 'finding every issue or tripwire related to the work we are
about to plan or implement'". On design grounding: "design grounding can still benefit from the
subagent focused on searching the open issue and tripwires related to the work, because the search
itself is expensive work for the main session. This can work well if the design skill is
instructed to find and read the full reported issues and tripwires, rather than relying on the
subagent summary." The agent's argument against delegating at design grounding was that the
discussion needs the entries in its own context. The owner's answer is the clause above: the
subagent finds the entries, and the session reads each one in full.

### Why it matters

`design@agent-skills@conformance-before-every-merge` makes the standing-state review the standing
re-entry point for every tripwire, and it runs after the work is done. A standing entry found at
that point costs a round of rework, where one found before the work costs a ruling. A search done
in the main session costs that session's context. Both strain
`goal@knowledge-architect@agents-get-a-complete-workflow`, which asks for efficient work.

### What would close it

A design discussion under `knowledge-architect-design` that settles:

- the occasions of the dispatch: after a plan document is written, as a fourth reviewer of §8; at
  the grounding of a design discussion; others;
- whether it is the standing-state reviewer in a second mode, whose input is a plan document or
  the statement of a question rather than a diff, or an agent of its own;
- what it returns, so that the session reads each entry it names in full rather than its summary;
- which step reads a `deferred` issue whose trigger the work meets, for work that is neither
  designed nor planned.

The converged design is recorded and built in the installed skills and agents.
