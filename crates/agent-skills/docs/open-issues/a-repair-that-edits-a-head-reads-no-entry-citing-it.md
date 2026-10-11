---
kind: question
---
# A repair of agent-facing text that edits a design head is not asked to keep true the issues and tripwires that cite the head

## Summary

The record check of `skill@knowledge-architect-project-audit@repair-standard` reads the goals, the
design heads and the rejected alternatives a repair touches. When the repair edits a head, nothing
asks it to read the issue entries and the tripwires that cite that head, so an entry that described
the head before the edit can be left false. On the branch of the first agentic-workflow audit of
this repository, two issue entries were left false that way. Should the record check, or the
application of a repair, read the entries that cite each head it edits?

## Details

### What

The record check, as the installed skill states it: "Each repair is read against the record of
what it edits before the owner rules on it: every goals home, the design heads that
`cargo klarch show` and a search of the design homes return for the behaviour the edited text describes, and the
rejected alternatives of each Component that owns an edited text." It names no issue directory and no
tripwires home. The design-record axis reads "the tripwires guarding it, the issues naming it" for
each head, per `design@agent-skills@design-record-axis`; the agentic-workflow axis has no such
reading, and its corpus excludes the registers, per `design@agent-skills@repairs-read-against-the-record`.

The two instances, both found by the review of that branch:

- The commit "[audit] Passes 2 and 3 of the agentic-workflow axis repaired by cause: 14 causes, 16
  instructions added, 58 removed" added to `design@agent-skills@primer-content` the bullet on the
  numbering of an item put to the owner under a label and the bullet on every commit of a branch
  judged by the checker at its tip. `issue@agent-skills@primer-content-bundles-each-directive-s-decision`
  still listed six directives and said "the head names six".
- The commit "[audit] Pass 4 of the agentic-workflow axis, run through the saved workflow: 36
  findings confirmed, 23 causes repaired" widened `design@agent-skills@project-name-prefix` from a
  project's own skills and agents to its skills, agents and saved workflows.
  `issue@core@tooling-for-project-skills` still said that head "names skills and agents".

Both were found at the review, not at the pass. Not established: whether a tripwire was left false
the same way on that branch; the tripwires citing the heads its passes edited were not all read for
this entry.

The question: should a repair that edits a design head read, before the owner rules on it or at
its application, every issue entry and tripwire that `cargo klarch show` lists for the head, and
keep each true? The reading that answers it: the audit method's points 7 and 8, the repair standard
and `skill@knowledge-architect-review@review-axes`, to find whether a step already owes this
reading and was skipped, or whether no step owes it.

### Why it matters

`design@agent-skills@repairs-read-against-the-record` reads a repair against the record so that the
owner rules knowing what it changes; an issue or a tripwire left false of the head it cites
misinforms the next session that reads it, and the review of the branch is then the only step that
catches it. `tripwire@agent-skills@audit-edit-leaves-an-entry-stale` guards the premise that an
audit keeps true the tripwires and issues on the heads it edits; it is written for the design-record
axis, and these two instances are its firing evidence on an audit of the other axis.

### What would close it

The answer recorded: either a step of the repair standard or of the method's application that
reads the issues and tripwires citing each edited head, stated in the installed project-audit skill
and in its design home, or a reading showing that an existing step owes it, with that step's text
made to say so; or the owner's ruling that the review of the audit's branch is enough, recorded in
the closing commit.
