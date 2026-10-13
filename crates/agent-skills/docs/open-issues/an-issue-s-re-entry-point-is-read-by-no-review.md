---
kind: question
---
# An issue's re-entry point is read by no review, for a design issue or any kind that carries one

## Summary

The standing-state reviewer reads every tripwire and every deferred issue's trigger before every
merge. An issue of kind `design` names a re-entry point, the discussion at which it is raised
again, and some re-entry points name an event a change can meet. An entry of another kind can
carry one too. No review reads them. Should the reviewer read the re-entry point of every entry
that carries one against the change, as it reads a trigger, or should a re-entry point be allowed
only where the issue-tracking skill gives it?

## Details

### What

A trial of the search agent, run during the work that made the reviewer read every deferred
trigger, returned `issue@knowledge-architect@command-output-is-not-declared-a-contract` with this
reason: "its deferral sits in a `### Re-entry point` subsection, not in a `### Trigger`, and its
kind is design, not deferred. A reviewer that reads only the triggers of deferred issues would not
read it." That entry's re-entry point is "at the latest before the project leaves 0.x", an event
a change can meet: the change that moves the version past 0.x.

The standing-state reviewer, in
`agent@knowledge-architect-standing-state-reviewer@reread-standing-entries`, runs `{{command}} issues --kind deferred` and reads
each entry's `### Trigger`. The issue-tracking skill asks a `design` entry for "the re-entry point:
the discussion at which it is raised again", with no fixed subsection name. In this repository,
`grep -rln '^### Re-entry'` over the issue directories finds two entries carrying one: the entry
above, and `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`,
whose re-entry is a discussion raised on the owner's word.

The search agent, at the grounding of a discussion and at the audit of a slice or of a spec, reads every issue whole,
so a design issue's re-entry point is read before designed or planned work, and before bounded work
the design skill grounds, per `design@agent-skills@bounded-path-in-design`. Work that does not go
through the design skill meets it at no step.

A real instance, in a slice of thaum on knowledge-architect 0.6.0: an issue of kind `question`
named a re-entry, "the next change to" two files, and the branch changed both. A searcher returned
the entry with a doubt, the session did not judge it, and the standing-state reviewer read only the
triggers of `deferred` entries. The transcript reviewer found it, and the project's owner re-parked
the entry. The kinds table of `skill@knowledge-architect-issue-tracking@issue-entry` gives a
re-entry point to `design` only. That retrospective proposed either answer: the issue-tracking skill
allows a re-entry point on `design` only, a `deferred` entry carrying its trigger instead, or the
reviewer reads every re-entry clause of every entry.

### Why it matters

`design@agent-skills@standing-state-review-before-every-merge` makes the standing-state review the reader,
for every change, of what waits for an event. A re-entry point that names such an event and that
no review reads is a parked item with no re-entry, which the issue-tracking skill says the
registers exist to avoid. It threatens `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A decision between the two answers, recorded in the head above: the standing-state reviewer reads
the re-entry point of every entry that carries one against the change, with how it finds the
re-entry point when the subsection has no fixed name; or a re-entry point is allowed only where the
issue-tracking skill gives one, a `design` entry's, and the reviewer reads those. Or the owner rules
that a re-entry is a discussion, which no change can meet, and the decision is recorded in that
head.
