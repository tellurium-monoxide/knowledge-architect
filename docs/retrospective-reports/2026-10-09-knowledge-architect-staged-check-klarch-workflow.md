# Analysis of 2026-10-09-knowledge-architect-staged-check-klarch-workflow

- **Received file:** 2026-10-09-knowledge-architect-staged-check-klarch-workflow.md, the workflow's
  side of this repository's own retrospective of the `check --staged` branch. The project's side is
  analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-09-knowledge-architect-staged-check.md`.
- **Version it used:** built from this repository's source, main at v0.5.0-35-g8c0bf84.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1.
- **Standing entries that bear on it:**
  - `issue@core@check-under-a-sparse-checkout-reports-every-file-outside-the-cone` records C1.
  - `tripwire@core@walked-count-differs-between-machines`: a sparse checkout changes the files the
    walk reads, but C1 reports a refusal on one machine, not a differing count; not fired.
  - `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names` is the opposite case of
    W1: a write refused, where W1 is a write made where it should not be.
  - `tripwire@agent-skills@ruling-lost-in-assembly`: W5's narrowed ruling was caught by the plan's
    re-review, before the point its firing clause names; not fired.

## W1: a reviewer wrote into the main checkout, and no step of the dispatcher sees it

- **Says.** An adversarial reviewer's script, with an unquoted path holding a space, wrote a copy of
  a mock project into the main checkout. The dispatcher learned of it only because its next `check`
  reported untracked files, and could not tell whose they were until the reviewer answered. A
  commit with `git add -A` at that moment would have recorded them. The report proposes a step for
  the dispatcher, `git status --short` of the main checkout compared with its own changes after each
  reviewer returns, and a brief clause confining every write of the reviewer.
- **Still applies.** Yes. `path@agent-skills@content/skills/review/SKILL.md`:124-125: "**No
  reviewer edits the live tree, and none runs there an operation that can lose content**: no stash,
  no reset, no checkout of a path." And :114-116: "a subagent that writes working files writes them
  there only". No installed text has the dispatcher look at the live tree after a reviewer returns;
  a search of the installed text for "git status" finds nothing.
- **Recorded.** No issue, no rejected alternative.
- **Kind.** A defect of the text, missing: the invariant binds the reviewer, and the reviewer that
  broke it did so by accident, so only a check by the dispatcher sees the breach. One real case.
- **Scope.** It serves the content-loss guard of `design@knowledge-architect@no-operation-loses-content`
  in an adopting project's terms: files the dispatcher did not write enter a commit. Owned by
  agent-skills, the review skill's invariants.
- **Better fix.** The brief clause alone is the rival: the reviewer here had the invariant and broke
  it unknowingly, so a stronger instruction to the same reader does not reach the cause. The
  dispatcher's check is cheap, mechanical, and sees every cause. The fix: "After the reviewers
  return, the dispatcher compares `git status --short` of the live tree with its own changes, and
  puts anything else to the reviewer before touching it."
- **Route.** A text edit whose design is settled, at the review skill's source.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## W2: the routing reviewer read the premortem labels as plain-text pointers

- **Says.** The design skill's step 7 keeps the `T<n>` label in a tripwire's entry; the routing
  reviewer reported the labels as pointers that dangle once the plan document leaves. The report
  reads the two as leaving no move.
- **Still applies.** Partly repaired. The design skill is unchanged in substance,
  `path@agent-skills@content/skills/design/SKILL.md`:617-620. The primer, which every reviewer
  holds, now says it outright, since 9180f4d, "[audit] Pass 5 of the agentic-workflow axis…", at
  `path@agent-skills@content/PRIMER.md`:34-35: "A label carried later with its item into another
  record, as a tripwire's into a tripwires home, keeps its number, and that record numbers
  nothing." It already said, before the session, that text not in the checker's syntax is outside
  the plain-text rule (:158-159). The routing reviewer's own definition does not mention labels.
- **Recorded.** No.
- **Kind.** Not a contradiction: the primer's rule on plain text never covered a label, and now
  states that the label is carried. The reviewer misread; that is a lapse of a subagent, against a
  text it holds.
- **Proposal.** No change: the routing reviewer reads the primer, which now settles the case.

- **Default:** no change, repaired by 9180f4d.
- **Outcome:** approved.

## W3: a criterion that fires after its report was written has no reporting instruction

- **Says.** AC1 was judged at step 1 and reported "did not fire". A review of steps 1 and 2 then
  found a state in which its observable fired, and the response was carried out at once. The
  harvest wrote "did not fire" again, with a caveat, and the standing-state reviewer caught it. The
  report proposes one sentence: a criterion whose observable a later review or test shows firing is
  reported as fired in the next landing commit, which names the earlier report as superseded.
- **Still applies.** Yes. `path@agent-skills@content/skills/planning/SKILL.md`:341-350, in
  `skill@knowledge-architect-planning@acceptance-criteria`: "**At each landing**, the messages of the
  commits that land it report on every criterion judged there"; "**A criterion that fires** leaves
  the document at once"; "**When the document leaves**, the landing that deletes it reports on every
  criterion once more." Nothing covers a firing observed after the step that judged it, or an
  earlier report it supersedes.
- **Recorded.** No.
- **Kind.** A defect of the text, missing. One real case.
- **Scope.** Owned by agent-skills, the planning skill. It reverses nothing.
- **Better fix.** The report's sentence states the rule that decides ("a criterion is judged by its
  observable, wherever the observation was made") and its consequence. No change is the rival; the
  standing-state reviewer caught it, at the cost of a repair of a landing message.
- **Route.** A text edit whose design is settled, at the planning skill's source.
- **Proposal.** Handle now, the report's sentence.

- **Default:** handle now.
- **Outcome:** approved.

## W4: whether an external tool's measured behaviour goes in a head or a comment

- **Says.** A harvest put into a head the git invocation whose flags each pin a measured behaviour.
  The decision-record reviewer judged the full invocation implementation. Both readings follow the
  text. The report proposes a sentence: a measured behaviour goes in the head as a fact about the
  tool, and the invocation stays at the code.
- **Still applies.** The texts moved to the primer with d95e227, unchanged in substance: entry test
  3, `path@agent-skills@content/PRIMER.md`:203-205, "an external tool's behaviour, read in its
  documentation or measured"; and :281-284, "**A head carries intent, shape and the standing
  argument, not implementation.**" Since 546faad, the standing argument holds "the measurement it
  rests on, with the command that takes it again" (:254).
- **Recorded.** No.
- **Kind.** Not a defect now. The primer's two sentences already separate the two: the measurement
  is a premise of the standing argument, with its re-take command; the invocation is code, which a
  head does not carry. The session reached that split, and the reviewer's verdict agreed with it.
- **Proposal.** No change.

- **Default:** no change; the primer separates the measurement from the invocation.
- **Outcome:** approved.

## W5: a one-answer gap applied in place narrowed a ruling

Cluster K3, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence-klarch-workflow.md`.

- **Default:** handle now, with K3.
- **Outcome:** approved.

## C1: plain `check` cannot pass under a sparse checkout

- **Recorded** as `issue@core@check-under-a-sparse-checkout-reports-every-file-outside-the-cone`,
  whose closing condition is the ruling C1 proposes, stated in `design@core@git-supplies-the-walk`.

- **Default:** no change, recorded.
- **Outcome:** approved.

## Standing answers

- **Shipped skills edited.** True: eaf0cef touches exactly the issue-tracking, planning and setup
  skills.
- **The sparse-checkout remark without an outcome.** It has one now, the issue above.
- **The adversarial axis.** `skill@klarch-development@code-review-axes` holds it.

- **Default:** no change.
- **Outcome:** approved.
