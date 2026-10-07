---
kind: question
---
# A design issue's re-entry point is read by no review

## Summary

The standing-state reviewer reads every tripwire and every deferred issue's trigger before every
merge. An issue of kind `design` names a re-entry point, the discussion at which it is raised
again, and some re-entry points name an event a change can meet. No review reads them. Should the
reviewer read the re-entry point of every design issue against the change, as it reads a trigger?

## Details

### What

A trial of the search agent, run during the work that made the reviewer read every deferred
trigger, returned `issue@knowledge-architect@command-output-is-not-declared-a-contract` with this
reason: "its deferral sits in a `### Re-entry point` subsection, not in a `### Trigger`, and its
kind is design, not deferred. A reviewer that reads only the triggers of deferred issues would not
read it." That entry's re-entry point is "at the latest before the project leaves 0.x", an event
a change can meet: the change that moves the version past 0.x.

The standing-state reviewer, in its section 2, runs `{{command}} issues --kind deferred` and reads
each entry's `### Trigger`. The issue-tracking skill asks a `design` entry for "the re-entry point:
the discussion at which it is raised again", with no fixed subsection name. In this repository,
`grep -rln '^### Re-entry'` over the issue directories finds two entries carrying one: the entry
above, and `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`,
whose re-entry is a discussion raised on the owner's word.

The search agent, at the grounding of a discussion and at the audit of a slice or of a spec, reads every issue whole,
so a design issue's re-entry point is read before designed or planned work, and before bounded work
the design skill grounds, per `design@agent-skills@bounded-path-in-design`. Work that does not go
through the design skill meets it at no step.

### Why it matters

`design@agent-skills@conformance-before-every-merge` makes the standing-state review the reader,
for every change, of what waits for an event. A re-entry point that names such an event and that
no review reads is a parked item with no re-entry, which the issue-tracking skill says the
registers exist to avoid. It threatens `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A decision on whether the standing-state reviewer reads the re-entry point of every design issue
against the change, and, if it does, how it finds the re-entry point when the subsection has no
fixed name. Or the owner rules that a design issue's re-entry is a discussion, which no change can
meet, and the decision is recorded in the head above.
