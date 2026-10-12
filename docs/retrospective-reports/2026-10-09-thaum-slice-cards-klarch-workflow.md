# Analysis of 2026-10-09-thaum-slice-cards-klarch-workflow

- **Received file:** 2026-10-09-thaum-slice-cards-klarch-workflow.md, the workflow's side of the
  thaum project's retrospective of the first two slices of a milestone. The project's own file was
  read for grounding only.
- **Version it used:** knowledge-architect 0.5.0, pinned.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1. This file holds the cluster K6.
- **Standing entries that bear on it:**
  - `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` records C1 in part.
  - `tripwire@core@item-of-another-plan-named` guards the scope that C2 meets. See its section.
  - `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` covers part of C2.
  - `issue@agent-skills@the-retrospective-counts-no-review-cost`: twelve reports and about 120
    items in one branch are such a count; the entry asks for the retrospective to record it, which
    this file did unprompted. No change follows.

## W1: a later round's commit range can span a history edit

- **Says.** The session rewrote the branch between two review rounds, then briefed a reviewer with
  a range from an old commit to the head; the old commit was no longer an ancestor. The reviewer
  used the main-based range and found the amended commits by `git range-diff`. The report proposes
  that a later round's range is the main-based one, and that commits are named by subject.
- **Still applies.** The text already prescribes the main-based range:
  `path@agent-skills@content/skills/review/SKILL.md`:92-94, "**Name what is reviewed as a commit
  range**, `<main branch>..<commit under review>` for a branch about to merge, unless the axis names
  its own range". The record names commits by subject (:196). The incremental range was the
  session's own choice.
- **Recorded.** No.
- **Kind.** A lapse: the instruction is clear, and the reviewer's own reading of it repaired the
  brief.
- **Proposal.** No change.

- **Default:** no change, a lapse.
- **Outcome:** approved.

## W2, with W5 of 2026-10-10-knowledge-architect-design-record-audit-klarch-workflow: items of a review report left without an outcome (cluster K6)

- **Says.** Two sessions, two projects. Here, twelve reports in one branch, and seven items without
  an outcome, each in a part of a report the skill names: a note outside the axis, an observation
  the reviewer declined to call a finding, a remark that a defect predates the change. In the
  design-record-audit session, five findings the session told the owner were "mine to repair" got
  no outcome. Both were found by the transcript reviewer and handled before the merge. Proposals: a
  ledger file, one row per report item, from which the record is written; or every reviewer numbers
  every item it reports.
- **Still applies.** The instruction is unchanged and covers both cases by name:
  `path@agent-skills@content/skills/review/SKILL.md`:140-143, "**Every item of a report gets an
  outcome, not only its numbered findings**: a note outside the reviewer's axis, an observation it
  declined to call a finding, a remark that a defect predates the change. … So is a repair the
  dispatcher promised the owner while answering a review." The second sentence predates the
  design-record-audit session (bd8a2dd).
- **Recorded.** No issue. The ledger's nearest rival lost, in the design discussion's form:
  `path@agent-skills@docs/rejected-alternatives.md`:44-52, "**Writing the discussion's ledger to a
  file during the discussion, so it survives compaction or a new session** — lost to
  `design@agent-skills@ledger-from-transcript`. … An artifact an agent must update every round is
  one it forgets to update". Reviewer reports are already files in the scratch directories
  (:114-116), so the source a ledger would copy exists.
- **Kind.** A lapse that recurs, twice, at the moment of writing the record. Unlike K7, the
  instruction is delivered where it is needed: the review skill's section on what review leaves is
  read when the record is written. The designed catch, the transcript reviewer, caught every item
  both times.
- **Scope.** Owned by agent-skills, the review skill.
- **Better fix.** The ledger meets the rejected alternative's argument. Numbering every item makes
  each reviewer's report carry what the dispatcher must answer, with no artifact for the dispatcher
  to keep; it is an instruction to every reviewer agent, and it costs nothing at the reviewer's end.
  It is the stronger of the two, but the measured failure is the dispatcher's, and the transcript
  reviewer already closes it before the merge. No change is the default; numbering is the fix should
  a third case reach the merge.
- **Proposal.** No change.

- **Default:** no change; a lapse the transcript reviewer catches as designed.
- **Outcome:** approved.

## W3: whether the parent milestone document is "another plan" for the leaving rule

- **Says.** A leaving slice spec was cited by two sibling slice specs and by its own milestone
  document. The session opened question issues for the siblings and only re-pointed the parent's
  citations, since a milestone cites its slices by design and the next slice's audit asks whether
  the milestone holds. The report asks whether the parent is "another plan".
- **Still applies.** Yes. `path@agent-skills@content/skills/planning/SKILL.md`:546-557, in
  `skill@knowledge-architect-planning@plan-document-leaves`: "**A citation of the leaving document
  from another plan dangles.** … it removes the citation from the citing plan; it opens a `question`
  issue … For a milestone, its next slice's audit reads the issue." The same skill's terms forbid the
  wording at :52: "The word "plan" alone never names a document: say spec, milestone document, or
  plan document." A milestone's documents count as one plan for citations (:224-228).
- **Recorded.** No.
- **Kind.** A defect of the text, unclear, caused by a breach of the skill's own terms.
- **Scope.** Owned by agent-skills, the planning skill. It reverses nothing.
- **Better fix.** The session's reading is the one the skill's structure supports: a milestone's
  documents are one plan for citation, and its next slice's audit is where the question is answered.
  The fix rewrites the paragraph in the skill's terms ("from another plan document") and states that
  a milestone's own documents re-point their citations of a leaving slice with no question issue.
  No change is the rival; it leaves the term the skill forbids.
- **Route.** A text edit whose design is settled, at the planning skill's source.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## W4: a repaired message can reference only what its own commit's tree holds

- **Says.** A repair needed an earlier commit's message to name an entry a later commit creates; it
  would dangle there, so the repair stated the content in prose. The report calls this correct and
  proposes one sentence in the review skill.
- **Still applies.** The rule is enforced by `commits`, per `design@core@a-commit-message-is-a-document`,
  and stated in `path@core@README.md`:452, a message's references resolve "against its commit's tree
  or its first parent's". The session met it as a finding and repaired it.
- **Kind.** An observation: the checker delivered the rule at the moment it applied.
- **Proposal.** No change.

- **Default:** no change; the checker states the rule when it applies.
- **Outcome:** approved.

## C1: no command regenerates the generated files across a branch's commits

- **Says.** A fold added quotations to an early commit; the generated rule index went stale at that
  commit and every later one, and `commits` reported "the generated file is out of date" once per
  commit. The session regenerated per commit with a `git rebase --exec` recipe. The report proposes a
  `commits --fix` over a range, or the recipe documented beside the finding.
- **Still applies.** `cargo klarch commits --help` shows a range and no `--fix`. No installed text
  documents a per-commit regeneration.
- **Recorded.** In part: `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` records
  the same symptom, and its closing condition is "`commits` prints one summary line naming the path
  and the repair: fold the regenerated file into the earliest failing commit", the report's second
  proposal in another shape. It does not record a `--fix` over a range, nor the case where later
  commits also change the index's inputs, where one fold into the earliest commit leaves later
  commits stale and the per-commit recipe is needed.
- **Kind.** A missing form of the checker, recorded.
- **Proposal.** The issue gains this case: a fold into one early commit leaves every later commit's
  generated file stale when later commits change its inputs, and the per-commit `rebase --exec`
  recipe that repaired it.

- **Default:** handle now: the issue gains this case.
- **Outcome:** approved.

## C2: a plan thread's name cited outside its plan is unchecked, and goes stale at the harvest

Cluster K4, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`.
Of the report's two proposals, a checked form for a thread across plans is the rejected alternative
"Citing an item of another plan"; a finding on a plain `#<slug>` matching a thread is K4's issue.

- **Default:** open an issue, `design`, in core, with K4.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## Standing answers

- **A premise supplied and not checked.** `path@agent-skills@content/PRIMER.md`:26 stands; the
  session then followed the primer's procedure. A lapse.
- **A project skill named in the project's routing table was missed.** The table is the project's
  own; `design@agent-skills@no-routing-table` holds that such a table delivers nothing beyond the
  skill's description, and this case is consistent with it. The standing-state review caught the
  fired tripwire, as designed.

- **Default:** no change.
- **Outcome:** approved.

## Standing entry: `tripwire@core@item-of-another-plan-named`

- **Judgement.** Not fired. It fires on a plan document's prose naming another plan's item. Here the
  sibling citing a thread was a slice spec of the same milestone, one plan for citation, and the
  other citer was source code. Neither is a plan document naming another plan's item.

- **Default:** not fired.
- **Outcome:** approved.
