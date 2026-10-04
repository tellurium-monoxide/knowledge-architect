---
kind: observation
---
# Reviewers dispatched in parallel pick their worktree paths independently, and one built inside another's

## Summary

In one review round of this repository, six reviewers ran in parallel, and each chose its own
worktree path under the session's scratchpad directory. One of them first created its worktrees
inside another reviewer's worktree, by its own account, and removed them. `knowledge-architect-review`
says where a worktree must not be, but not how parallel reviewers keep their paths apart.

## Details

### What

The review skill's invariant gives every reviewer that runs a binary its own worktree, "at a path
where it pollutes no search". It does not say who picks the path. In the review of the branch that
tied every build to its checkout, the dispatcher's briefs left the path to each reviewer. The
reports name the paths chosen: wt, wtA, wtB, routing-wt, adv/A, adv/B and adv/M under
the scratchpad. The adversarial reviewer reported: "The fixed scratchpad paths of the review
procedure are shared between agents. … I first created my worktrees inside `wt` by accident,
removed them at once, and confirmed `wt` was still clean."

Conditions: Claude Code, six fresh subagents dispatched in one message, all given the same
scratchpad by the harness. Whether it reproduces is `not established`: it depends on which names
each reviewer picks. What would make it reproduce: several reviewers in one round whose briefs
name no path, each told to work under the same directory.

No harm was observed. Whether a collision can lose content or mix two reviewers' builds is
`not established`.

### Why it matters

A reviewer that builds or mutates inside another reviewer's worktree changes the tree the other
reviewer is judging. The other reviewer then reports on a state it did not choose. That defeats the
independence the review skill's invariants exist to buy.

### What would close it

One more observation of a collision, or of a reviewer judging a tree another reviewer changed.
Under `design@agent-skills@additions-need-real-use`, that observation makes the addition to the
review skill: the dispatcher names a distinct worktree path in each brief. Or a reading of further
review rounds that finds no collision, which retires the entry.
