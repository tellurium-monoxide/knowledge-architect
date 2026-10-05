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

- The three tripwires recorded from the premortem of `design@agent-skills@every-decision-through-design`
  and `design@agent-skills@in-change-path` first named the transcript review and the standing-state
  review as re-entry points. Neither reviewer does the check: the transcript reviewer does not read
  the tripwires homes, and the standing-state reviewer is given no transcript. The standing-state
  reviewer of that branch found it, and the re-entry points were moved to the session's
  retrospective.
- `tripwire@agent-skills@ruling-lost-in-assembly` had the same defect, repaired after that
  retrospective.

After those repairs, each of the five tripwires of this Component re-enters at a retrospective or a
standing-state review of this repository. A session of a consumer project that meets the firing
evidence reaches this repository only if its owner runs the retrospective and sends the workflow's
file as an issue.

The suspected mechanism, in one sentence: a tripwire about the shipped skills watches behaviour
that occurs in other projects' sessions, and its re-entry points are this repository's own
occasions.

The installed issue-tracking skill defines the re-entry point as "the checkpoint at which it is
read again", and names the standing-state reviewer as the standing one. It does not ask that the
occasion holds the firing evidence.

### Why it matters

`goal@knowledge-architect@the-workflow-improves-through-real-use` is met only while evidence from
real sessions reaches the decisions it bears on. A tripwire whose evidence arises where nobody reads
the tripwire guards nothing, and the decision it watches is treated as safe without having been
watched.

### Re-entry

A design discussion about how the tripwires of the shipped skills are watched, raised when the next
retrospective file from a consumer project arrives, or before then on the owner's word. The shape
the owner is considering, in their words: "add re-entry points 'when receiving retrospectives' to
tripwires that look at agent behaviors under the workflow, and to record the watched behaviors in
the shipped retrospective skill itself (or bundled in the agent skill crate under a command, to
avoid polluting what gets committed in other projects)." The owner calls it larger design work.

### What would close it

A decision on how a tripwire about the shipped skills is re-entered when its evidence arises in a
consumer project, recorded and built. The rule for a re-entry point in the installed issue-tracking
skill is part of it: the occasion named holds the firing evidence.
