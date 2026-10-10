---
kind: deferred
---
# The transcript reviewer cannot see a plan document that the branch it reviews deleted

## Summary

At the review before the merge of a spec's branch, the spec has been deleted at its harvest. The
transcript reviewer searches the tree at the range's last commit and the range's messages, so an
argument kept only in the deleted spec looks to it like one with no durable outcome. It reports it,
and the dispatching session has to judge each such report to need nothing.

## Details

### What

- `agent@knowledge-architect-transcript-reviewer`, its section on finding each outcome: "Search the
  tree at the last commit of the range, and the messages of the range with `git log <range>`." The
  same section lists a plan document among the homes of an outcome.
- `skill@knowledge-architect-planning`: the work of a spec deletes the spec in the commit that
  completes its harvest, before the review before the merge.
- `skill@knowledge-architect-decision-recording@three-homes`: the deliberation lives in the plan
  document, then in history, and is not copied into the harvest's message. That section gives the
  command that finds a deleted plan document.

The same blind spot touches the standing-state reviewer's reading of "the criteria that the plan
document says that landing judges" when the spec is deleted in the range.

Reported by the L2 walk of design, planning, decision recording and issue tracking, in the fifth
pass of the agentic-workflow audit. Its verifier judged it no strict contradiction: the session can
judge each report to need nothing, so the cost is false reports, not a blocked move.

### Why it matters

False major findings at the last review of every spec's branch cost the session a reading each,
and teach it to discount the transcript reviewer's reports. That reviewer guards
`goal@knowledge-architect@agents-work-without-drift`.

### Trigger

`issue@agent-skills@an-audit-s-provable-gap-on-installed-text-against-real-use` closes in favour of
editing on a provable gap, or a review before a merge shows the transcript reviewer reporting an
argument kept only in the deleted spec. The repair proposed: the search also reads each plan
document the range deletes, as it stood before its deletion, found as
`skill@knowledge-architect-decision-recording@three-homes` says.
