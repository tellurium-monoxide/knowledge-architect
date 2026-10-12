# Analysis of 2026-10-10-knowledge-architect-design-record-audit-klarch-workflow

- **Received file:** 2026-10-10-knowledge-architect-design-record-audit-klarch-workflow.md, the
  workflow's side of this repository's own retrospective of the first design-record audit. The
  project's side is analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-10-knowledge-architect-design-record-audit.md`.
- **Version it used:** built from this repository's source, main at v0.5.0-80-gf422e5b.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1. The project-audit skill has been reworked
  since: both axes run each pass as one run of a saved workflow, and a second design-record pass ran
  on 2026-10-12.
- **Standing entries that bear on it:**
  - `tripwire@agent-skills@audit-defaults-overruled` reads the count W1 asks to record.
  - `tripwire@agent-skills@a-head-verdict-is-overruled`: W3's head was moved to a `%%` line by the
    audit's finding F35 and restored by the owner's Q1, which approved the review's default, per
    942532f, "[review] The owner's rulings on the review…". The owner overruled no review's
    verdict, so it does not fire.
  - `tripwire@agent-skills@plain-text-pointer-found`: P2 of the project's file, and C3 of the
    path-quickfixes file. See its section.
  - `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` is adjacent to P3 of
    the project's file: it concerns permission prompts, not the opt-in.
  - The other audit tripwires the search returned (`tripwire@agent-skills@audit-drafts-mostly-noise`,
    `tripwire@agent-skills@conforming-draft-wrong`, `tripwire@agent-skills@audit-passes-do-not-converge`,
    `tripwire@agent-skills@record-check-misses-a-head` among them) were written at this session's
    harvest or later, and guard the saved-workflow method; this file reports on the method before
    them, so it is no firing evidence for any.

## W1: the record of a pass does not count the owner's rulings against the default

- **Says.** The owner asked how many violations went through them; the shares, about 42%, 25% and
  39% per pass, were recorded in the harvest message only. The report says this figure is what a
  convergence judgement and the tripwire on overruled defaults read, and proposes that the record of
  a pass counts the violations, how many went to the owner, how many were applied, how many went to
  an issue, and how many rulings went against the default.
- **Still applies.** Partly repaired by 55a0e15, "The project audit counts convergence on old text,
  checks each repair against the record, and commits each pass's report file".
  `path@agent-skills@content/skills/project-audit/SKILL.md`:137-142, in
  `skill@knowledge-architect-project-audit@record-of-an-audit`: "the outcome of each finding in a
  line … the count of violations left on old text and on new text, as point 4 defines them, and of
  those on old text that went on the owner list, so that the passes show whether the audit
  converges." The owner-list share is recorded. The count of rulings against the default is not,
  and `tripwire@agent-skills@audit-defaults-overruled` fires on "more than a quarter" of them; its
  re-entry is the session receiving the answers, which counts and writes the count nowhere.
- **Recorded.** No issue.
- **Kind.** A defect of the text, missing: a tripwire reads a count that no record keeps, so a later
  pass cannot show it fired.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`. Owned by agent-skills, the
  project-audit skill's record. It reverses nothing.
- **Better fix.** The per-finding outcome lines let a reader derive the count; deriving it from a
  pass's report file each time is the rival. The tripwire's threshold is a share, read at each
  pass, so the count belongs beside the other counts of the pass. The applied and issue counts are
  derivable and serve no tripwire.
- **Route.** A text edit whose design is settled, at the project-audit skill's source.
- **Proposal.** Handle now: the record's counts gain "and how many of the owner's rulings went
  against the default".

- **Default:** handle now.
- **Outcome:** approved.

## W2: the project-audit skill's description said "one list for the owner"

- **Says.** An owner ruling made the method run in passes; the section and its head were updated,
  the description was not, and no reviewer reported it. The report proposes a rewording, and a
  general rule that a change to a skill's section is checked against its description.
- **Still applies.** Repaired in substance by 80bab85, "The design-record axis runs each pass as one
  run of its saved workflow, and the audit method states one shape for every axis".
  `path@agent-skills@content/skills/project-audit/SKILL.md`:3: "its method (an axis's saved workflow,
  whose one run per pass delivers a report the session verifies; then one list for the owner, the
  application, the re-check, the review)". The owner list is now read per pass. The general rule
  is absent.
- **Recorded.** No.
- **Kind.** A defect of the text, repaired. The general rule has one instance.
- **Proposal.** No change: the description is true now, and one instance does not argue a new rule
  under `design@agent-skills@behaviour-additions-need-real-use`.

- **Default:** no change, repaired by 80bab85.
- **Outcome:** approved.

## W3: whether a design head that restates an instruction is a site under entry test 2

- **Says.** During the audit, a head was moved to a `%%` comment on the premise that only one text,
  the review skill, stated its rule. Another head stated it too; the review found it, and the owner
  restored the head, Q1. The primer's test 2 names "a skill, an agent or a `CLAUDE.md`"; the
  decision-recording skill counts "a restatement of the project's own". The report proposes that
  test 2 names every text that states the instruction, a design head included.
- **Still applies.** Yes. `path@agent-skills@content/PRIMER.md`:197-200, test 2: "For a decision
  about agent-facing text, a skill, an agent or a `CLAUDE.md`, each text that states the instruction
  is one site". Its last change is 0e57dc2, the reported session's own. The other reading,
  `path@agent-skills@content/skills/decision-recording/SKILL.md`:265-267: "a site the change itself
  wrote counts, a skill or a restatement of the project's own included" (f2b1090).
- **Recorded.** No issue. The owner's Q1, which restored the head because a second head stated the
  rule, is the evidence of how the owner reads it.
- **Kind.** A defect of the text, unclear: two installed texts define a site differently, and an
  audit acted on the narrower one against the owner's reading.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Owned by
  agent-skills. Test 2 is the delivery of `design@agent-skills@a-head-is-owed-by-an-entry-test`, so a
  change of its wording changes a statement of that head, under `skill@knowledge-architect-design`,
  on its in-change path.
- **Better fix.** Rivals: name every text that states the instruction, a design head included, in
  test 2; or point test 2 to the decision-recording wording. The primer is what every session reads,
  and the auditor judges against it, so the definition belongs there; the decision-recording
  wording then becomes a restatement of it. No change leaves an auditor free to repeat the removal
  the owner reversed.
- **Route.** A decision's wording, under `skill@knowledge-architect-design`, in-change path, with
  Q1 quoted as the owner's ground.
- **Proposal.** Handle now: test 2 names "each text that states the instruction, a design head of
  the project included".

- **Default:** handle now, under the design skill's in-change path.
- **Outcome:** approved.

## W4: the session read the auditor's summaries instead of its drafts

- **Still applies.** Repaired by 80bab85 and the saved workflow. The sentence "Then read its drafts,
  never its summary alone" is gone; a verifier per group reads the drafts and a report agent writes
  the report from them (`path@agent-skills@content/workflows/design-record-audit.js`:353-385), and
  the skill bounds what the session reads,
  `path@agent-skills@content/skills/project-audit/SKILL.md`:228-233: "every item of the owner-list
  draft, and every finding the report marks as plausible or as bearing on a ruling of the owner or
  on a tripwire, against its draft; every check; and a sample of the refuted drafts and a sample of
  the conforming drafts". The proposed index is superseded.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## W5: review items without an outcome

Cluster K6, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-09-thaum-slice-cards-klarch-workflow.md`.

- **Default:** no change, with K6.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## W6: two defaults of one batch contradicted each other

- **Still applies.** The rule stands, `path@agent-skills@content/PRIMER.md`:26, "A premise the
  session supplied is the session's to check before the owner rules on it." The report proposes no
  change. A lapse, caught by the decision-record re-review as designed.

- **Default:** no change, a lapse.
- **Outcome:** approved.

## W7: the auditor and the reviewer restated one rule differently

- **Still applies.** Repaired by c9e0d4d, which added the verdict-alignment lens of the
  agentic-workflow audit, `path@agent-skills@content/agents/workflow-auditor.md`:114-119, naming "the
  decision-record reviewer and the design-record auditor, on design heads and rejected
  alternatives". The owner's lead is built.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## C1: the checker refused a branch SHA and an out-of-plan item reference

- **Kind.** An observation: two checks caught real failures of a long session.

- **Default:** no change.
- **Outcome:** approved.

## Standing answers

- **A head crediting the owner with a limit the agent proposed, and a head describing a defeated
  reading as an earlier state.** Both rules stand in `primer@design-heads`; both were caught by the
  re-review. Lapses.
- **A subagent wrote the owner's quotation where the ruling said "without my words".** A breach of
  a ruling, repaired by the session.
- **A closed issue named in plain text in a head.** P2 of the project's file.

- **Default:** no change.
- **Outcome:** approved.

## Standing entry: `tripwire@agent-skills@plain-text-pointer-found`

- **Judgement.** Not fired by either candidate. Its second clause fires on "a retrospective's answer
  … reports a pointer written as bare plain text to clear a finding", and its second instance would
  reopen `design@agent-skills@plain-text-is-no-repair`; the first is recorded.
  - P2 of the project's file: the head names a deleted entry in words, beside a command that finds
    it. It was written in a review repair as an argument's evidence, not to clear a finding, and no
    checked form names a deleted entry. It is a non-conformance with the issue-tracking skill's rule
    on past sentences, repaired by P2.
  - C3 of the path-quickfixes file: two quotations of the owner's words and of raw spans, written
    bare because in backticks each would be a finding. That is closer to the clause, but the text is
    a quotation, not a pointer: it names no target. It is K5's missing form.

- **Default:** not fired.
- **Outcome:** approved.
