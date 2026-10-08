---
kind: design
---
# A tripwire on agent behaviour under the workflow fires in sessions this project does not see

## Summary

The tripwires that guard this Component's decisions fire on how agents behave under the installed
skills. That behaviour happens mostly in the sessions of consumer projects. Every re-entry point
these tripwires name is an occasion of this repository, so a firing elsewhere is not guaranteed to
reach it. Finding W2 of the retrospective of 2026-10-05.

## Details

### What

The instances, in `path@agent-skills@docs/tripwires.md`:

- The tripwires first recorded from the premortem of the decision now recorded as
  `design@agent-skills@new-or-reshaped-head-needs-design`, and of `design@agent-skills@in-change-path`,
  named the transcript review and the standing-state review as re-entry points. Some have since
  fired or been replaced. Neither reviewer does the check: the transcript reviewer does not read
  the tripwires homes, and the standing-state reviewer is given no transcript. The standing-state
  reviewer of that branch found it, and the re-entry points were moved to the session's
  retrospective.
- `tripwire@agent-skills@ruling-lost-in-assembly` had the same defect, repaired after that
  retrospective.
- The premortem of `design@agent-skills@checker-syntax-without-backticks-names-its-gap` named a
  cause it could not guard: a consumer's own entry for a gap of the checker stays open, with its
  plain-text sites, after a release ships the checked form, because nothing tells the consumer.
  The evidence is in the consumer's sessions, so the owner ruled that it gets no tripwire and is
  recorded here as one more instance. The retrospective's standing question on references written without backticks,
  per `design@agent-skills@premortem-as-watch-points`, is the one channel that carries such a gap
  to this repository.

After those repairs, each tripwire of this Component re-enters at a retrospective or a
standing-state review of this repository. The repairs moved the gap rather than closing it: the
installed retrospective skill opens no tripwires home, and runs only when the owner accepts it, so
the tripwires whose evidence is in a session's transcript are read by no step of any procedure. The
standing-state reviewer reads them, and has no transcript. One of them fired on the branch that
opened this entry, and was judged only because that branch's transcript reviewer raised it as an
item with no outcome. Its replacement guarding
`design@agent-skills@new-or-reshaped-head-needs-design` reads its evidence from the commit history,
which the standing-state reviewer holds.

A session of a consumer project that meets the firing evidence reaches this repository only if its
owner runs the retrospective and sends the workflow's file as an issue.

The suspected mechanism, in one sentence: a tripwire about the shipped skills watches how agents
behave in sessions, and no occasion that reads a tripwires home holds a session's record, in this
repository or in a consumer project.

The installed issue-tracking skill defines the re-entry point as "the checkpoint at which it is
read again", and names the standing-state reviewer as the standing one. It does not ask that the
occasion holds the firing evidence.

### Why it matters

`goal@knowledge-architect@the-workflow-improves-through-real-use` is met only while evidence from
real sessions reaches the decisions it bears on. `design@agent-skills@conformance-before-every-merge`
makes the standing-state review the standing re-entry point of every tripwire home, and that review
holds only the tree. A tripwire whose evidence arises where nobody reads the tripwire guards
nothing, and the decision it watches is treated as safe without having been watched.

### Re-entry

A design discussion about how the tripwires of the shipped skills are watched, raised on the owner's
word. An incoming retrospective file is analysed under this repository's
`skill@klarch-retrospective-intake`, whose search of the standing entries reads every tripwire
against the file. That search reads only what the file carries. The shape the owner is considering,
in their words: "add re-entry points 'when receiving retrospectives' to tripwires that look at agent
behaviors under the workflow, and to record the watched behaviors in the shipped retrospective skill
itself (or bundled in the agent skill crate under a command, to avoid polluting what gets committed
in other projects)." The owner calls it larger design work. In a later discussion, about where the
search for the issues and tripwires a piece of work bears on runs, the owner stated the aim of that
work: "My later goal is to automate the retrospective skill to look at the tripwires of this
project, without requiring a manual edit of the questions the retrospective skill asks." One
tripwire waits for this discussion on the owner's word,
`tripwire@agent-skills@search-missed-before-the-work`. The owner's words, about that tripwire: "This
waits for the session that discusses solving the issue
a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see, whose plan is to make use of the
retrospective skill to analyze other projects." A second tripwire of the same class,
`tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`, waits for evidence that arises
mostly in the sessions of projects that use the workflow; its re-entry is the standing-state review
of this repository, which holds only the tree.

### What would close it

A decision on how a tripwire about the shipped skills is re-entered, in this repository's own
sessions and when its evidence arises in a consumer project, recorded and built. The rule for a
re-entry point in the installed issue-tracking skill is part of it: the occasion named holds the
firing evidence.
