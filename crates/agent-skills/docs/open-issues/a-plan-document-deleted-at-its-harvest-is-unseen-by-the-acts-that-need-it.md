---
kind: deferred
---
# A plan document deleted in the commit that completes its harvest is absent from the tree of the acts that still need it

## Summary

`skill@knowledge-architect-planning` deletes a spec in the commit that completes its harvest, and
that harvest's review is the review before the merge. So the spec is absent from the tree that
review reads, and a text written into the spec in the deleting commit enters no tree at all. Four
acts still need the spec at that point: the transcript reviewer's search, the standing-state
reviewer's reading of the criteria, the spec-conformity and fidelity-of-relocation axes, and a later
`git log -G` for a thread-to-slug pair written into the harvest row. Cases 2 to 4 are predicted
gaps, and case 1 a provable gap, which `design@agent-skills@behaviour-additions-need-real-use` admits for the
owner list: no real session has shown any of them. On the owner's ruling, case 1 stays parked here
with the other three until their shared structural repair is designed.

## Details

### What

The structure:

- `skill@knowledge-architect-planning@working-a-slice`, point 6: "**The harvest's review is the
  review before the merge**"; point 7: "**The slice's spec leaves** in the commit that completes
  its harvest"; and the work of a spec takes both points once.
- `skill@knowledge-architect-planning@plan-document-leaves`: "A spec is deleted in the commit that
  completes its last harvest".
- `skill@knowledge-architect-review@review-invariants`: a subagent that runs commands has a
  worktree detached at the commit it judges, the branch's tip for the review before the merge.

The acts that need the spec after that point:

1. **The transcript reviewer.** `agent@knowledge-architect-transcript-reviewer`, its section on
   finding each outcome: "Search the tree at the last commit of the range, and the messages of the
   range with `git log <range>`." The same section lists a plan document among the homes of an
   outcome. An argument kept only in the deleted spec looks to it like one with no durable outcome,
   and it reports it. Reported by the L2 walk of design, planning, decision recording and issue
   tracking, in the fifth pass of the agentic-workflow audit; its verifier judged it no strict
   contradiction, since the session can judge each report to need nothing, so the cost is false
   reports, not a blocked move.
2. **The standing-state reviewer**, reading "the criteria that the plan document says that landing
   judges" when the spec is deleted in the range.
3. **The spec-conformity and fidelity-of-relocation axes** of the review before the merge.
   `skill@knowledge-architect-review@review-axes` briefs an axis with no agent with "the commit
   range, and the files that carry the standard", the spec among them, and that file is absent at
   the tip. No text says to name it with the commit that holds it, as
   `skill@knowledge-architect-project-audit@audit-method` point 5 does for an audit's report files.
   Reported by the L2 lens in the sixth pass of the agentic-workflow audit, as a predicted gap.
   Ruled out as a certain failure, by its verifier: the range's diff carries the deleted spec as
   removed lines, and `skill@knowledge-architect-planning@plan-document-leaves` gives
   `git show <commit>^:<path>`, so a dispatcher can name it; no reading shows a reviewer misled.
4. **The thread-to-slug pair.** `primer@design-heads`, "How a head is shaped", states the pair
   `#<thread> → <entry slug>` in "the plan document's harvest row", found by "`git log -G` a plan
   document's diff that does"; `skill@knowledge-architect-decision-recording@three-homes` keeps the
   deliberation out of the harvest's commit message. A pair written into the row in the deleting
   commit is in no tree, and `git log -G` finds it nowhere. Reported by the L2 walk of design,
   planning, decision recording and issue tracking in the sixth pass, as a predicted gap: the texts
   also allow the pair to be written at the assembly or in an earlier harvest commit, and which a
   session does is not established.

The reproduction of case 4, on git 2.43.0, in an empty directory:

```sh
git init -q . && mkdir -p docs/plans/specs
printf '| thread | home |\n| --- | --- |\n| #old-thread | a head |\n' > docs/plans/specs/s.md
echo '# Design' > design.md
git add -A && git commit -qm spec
# the harvest: the pair written into the row, the head added, the spec deleted, in one commit
sed -i 's/#old-thread | a head/#old-thread -> new-slug | a head/' docs/plans/specs/s.md
echo '### A decision `##new-slug`' >> design.md
git rm -qf docs/plans/specs/s.md && git add -A && git commit -qm harvest
git log --format=%s -G'old-thread -> new-slug'   # prints nothing
git log --format=%s -G'new-slug'                 # prints "harvest", from design.md's line only
```

The deleting commit's diff removes the row as its parent held it, without the pair, so no diff
ever holds the pair.

A file an audit's branch adds and deletes is the same structure for a report file; the audit
skill's point 5 names each such file with its commit in the review's briefs.

### Why it matters

The structure strains `design@agent-skills@plan-document-leaves-at-landing`, "A plan document is
deleted in the commit that completes its last harvest". Its argument is against a plan document
kept after its harvest, a second home that starts drifting at the first later reversal; whether the
deletion may fall in a later commit of the same branch is the question the structural repair below
puts to it.

- Cases 1 to 3: false or blind findings at the last review of every spec's branch, which cost the
  session a reading each and teach it to discount the reviewers that guard
  `goal@knowledge-architect@agents-work-without-drift`.
- Case 4: a reader of a renamed entry slug cannot reach the thread that argued it, against
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`. It defeats
  `design@agent-skills@thread-slug-is-entry-id`, whose pair in the harvest row is what lets the
  spec, the commit messages and the design home name the decision with one identifier, and strains
  `design@agent-skills@standing-argument-in-head`, whose reader finds the deliberation in a deleted
  spec's last state, which a pair written in the deleting commit is not part of.

### Trigger

Any of these:

- a review before a merge shows a reviewer reporting, or judging without, a spec the range
  deletes: the transcript reviewer reporting an argument kept only in the deleted spec, the
  standing-state reviewer or the spec-conformity or fidelity-of-relocation axis judging with the
  spec absent; the standing-state reviewer reads this trigger at that review;
- an intake of a retrospective's file, under `skill@klarch-retrospective-intake`, reads a real
  session in which one of the four cases occurred, a pair written at a harvest found by no
  `git log -G` included; deciding a repair of the installed text from real use is that occasion's
  work, per `design@agent-skills@behaviour-additions-need-real-use`.

The repairs proposed, either one decided under `skill@knowledge-architect-design`:

- **The structure removed**: the spec is deleted in a commit of its own after the review before the
  merge has read the tip that holds it. It closes all four cases. It changes the statement of
  `design@agent-skills@plan-document-leaves-at-landing`, the sentence of
  `skill@knowledge-architect-review@review-axes` that only the review's own repairs land after it,
  and every installed text that states the deletion moment:
  `skill@knowledge-architect-planning@working-a-slice` point 7 and its paragraph on the work of a
  spec, `skill@knowledge-architect-planning@plan-document-leaves`, and the paragraph of
  `primer@design-heads` that places the pair in the harvest row.
- **The briefs extended**: each reviewer of the review before the merge also reads each plan
  document the range deletes, as it stood before its deletion, found as
  `skill@knowledge-architect-decision-recording@three-homes` says. It closes cases 1 to 3, and not
  case 4, since no commit holds the pair.
