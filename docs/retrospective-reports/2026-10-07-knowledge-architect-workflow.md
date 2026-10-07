# Analysis of 2026-10-07-knowledge-architect-workflow

- **Received file:** 2026-10-07-knowledge-architect-workflow.md, the workflow's side of this
  repository's own retrospective.
- **Version it used:** built from this repository's source, main at v0.4.0-41-gf5d403a.
- **Analysed against:** the same main, plus the commit "A received retrospective file is analysed
  under klarch-retrospective-intake, the retrospective names its files by subject, and the searcher's
  groups are balanced".
- **Standing entries that bear on it:** `issue@agent-skills@the-retrospective-counts-no-review-cost`
  (W3), `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` (W9),
  `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` (C2),
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` (C4),
  `tripwire@core@candidate-rule-silence` (C3), `tripwire@agent-skills@a-head-verdict-is-overruled`
  (W1, W2: the owner's words in that session corrected the agent's reading of the owner's own words,
  and overruled no review verdict, so it does not fire).

## W1: test 2 does not say what a "site" is for an instruction

- **Says.** Test 2 admits a reason "respected at more than one site, or at none", none including "a
  policy with no code of its own". Every decision about installed text is such a policy, so the
  clause can be read as admitting all of them. An audit counted two skills stating one decision as one
  site. Proposed fix: for an instruction, the site is the text that states it, and "a policy with no
  code of its own" is one no text states either.
- **Still applies.** `path@agent-skills@content/skills/decision-recording/SKILL.md` lines 89-94; no
  definition of a site for installed text.
- **Recorded.** None. The proposed narrowing contradicts recorded intent: §0 of the same skill says
  test 2 admits a policy "as a decision with no site of its own"; the head
  `design@agent-skills@a-head-is-owed-by-an-entry-test` says a policy "has no site at all"; and the
  rejected alternative on the earlier test 2 says the same. For installed text,
  `design@agent-skills@instruction-record-is-minimal` already narrows: "A rewording, and the rationale
  for how one instruction is phrased, is not recorded."
- **Kind.** Unclear, and the fix as proposed is a reversal of a recorded head.
- **Scope.** Agent-skills; `design@agent-skills@a-head-is-owed-by-an-entry-test`.
  `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` is a question on the same tests.
- **Better fix.** Not settled. Two shapes: define a site for installed text (each text stating the
  reason), which keeps "no site" for policies no text states; or keep test 2 and let
  `design@agent-skills@instruction-record-is-minimal` carry the narrowing for installed text,
  with a pointer from test 2. Either touches a head's argument.
- **Route.** A design discussion.
- **Proposal.** Open an issue: agent-skills, `question`, citing the head and the adjacent question.
- **Default:** open an issue.
- **Outcome:** issue approved.

## W2: a word of the owner given on a premise that later proved false

- **Says.** The agent read three kinds of the owner's words as rulings recording intent: an approval,
  a hedge, and a word given on a premise that later proved false ("They should be kept anyway",
  given in the belief that the heads passed other tests). Test 4 now asks the owner. The third case
  is still open outside a discussion. Proposed fix: such a word is checked against its premise before
  it is acted on, and the owner is told when the premise fails.
- **Still applies, in part.** The first two cases were repaired in that session: test 4, lines 98-105,
  "The agent does not judge this", since `2ac38cb`. The third: the duty exists in the design skill
  alone, "**Never assume the decision saw all its consequences.**" and a material finding defeats "a
  premise it rested on" (`path@agent-skills@content/skills/design/SKILL.md` lines 61-71). A word met
  during a review repair is under no design skill. The primer holds no rule on the owner's words; the
  report's "the primer's owner-word rules" names a section that does not exist.
- **Recorded.** None.
- **Kind.** A defect of delivery: the instruction exists where the case did not arise.
- **Scope.** Agent-skills; `goal@knowledge-architect@the-owner-decides`.
- **Better fix.** Where it is delivered is the question: beside test 4, where this case arose, or in
  the primer, which reaches every session, under `design@agent-skills@primer-limit`. The case is not
  specific to decision recording, so beside test 4 covers one site of a general rule, and the primer
  is held to what every session needs.
- **Route.** A design discussion on the placement, small.
- **Proposal.** Open an issue: agent-skills, `question`.
- **Default:** open an issue.
- **Outcome:** issue approved.

## W3: two instructions read when a decision is made are needed when its commit is written

- **Says.** "Say what you searched and what it returned" (decision-recording §1) and "Rewrite it as if
  the design had always been so" (§5) were missed at commit time; the decision-record reviewer
  reported them five and four times. Proposed fix: a closing checklist in §1.
- **Still applies.** Lines 43-44 and 187-189 of the skill. Its §8, "Before you finish", lines 349-359,
  runs the check and re-reads against the replaced head, and reminds of neither. The counts are in that
  session's transcript, not in the tree; one instance is visible in the message of `86c6dec`.
- **Recorded.** The cost bears on `issue@agent-skills@the-retrospective-counts-no-review-cost`, which
  records that no such count is kept. The fix is not recorded.
- **Kind.** A lapse that recurs at one point of delivery: the instructions are read at §1 and §5, and
  needed when the commit is written. That is the exception where a delivery change is worth text.
- **Scope.** Agent-skills.
- **Better fix.** The report puts a checklist in §1. Rival: two lines in §8, "Before you finish",
  which is already the finish-time point of the skill. A list in §1 is read at the start.
- **Route.** A text edit, installed text.
- **Proposal.** Handle now: §8 gains the search line and the "as if always so" re-read.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W4: no installed text says how to refer to the owner

- **Says.** Two reviewers wrote "Ask him" and "his words"; a pronoun reached a commit message before
  it was caught. Proposed fix: "Refer to the owner as the owner; never infer a pronoun", in each
  reviewer or in the review skill's brief invariants.
- **Still applies.** No rule: `git grep -i pronoun` returns nothing. The report's claim that the text
  says "the owner" throughout is partly false: the installed text uses the singular "they" for the
  owner, as "Their explicit word" (`path@agent-skills@content/skills/design/SKILL.md` line 56) and
  "they are" (line 379), and never a gendered pronoun.
- **Recorded.** None.
- **Kind.** Missing; twice in one session, with the defect reaching a commit message.
- **Scope.** Agent-skills.
- **Better fix.** The report's "never a pronoun" contradicts the installed text's own "they".
  Proposed wording: "the owner, or they; never a gendered pronoun". Placement: the review skill's
  brief invariants, so every brief carries it, including a general-purpose reviewer's. Rival: a line
  in each of the seven agents, seven copies.
- **Route.** A text edit, installed text.
- **Proposal.** Handle now.
- **Default:** handle now.
- **Outcome:** This one is part of a larger problem, which is that this workflow expects an owner (unnamed). This cannot work under a multi contributor project. This should become an issue about the larger problem, and not handled now. I have a solution in mind, but I'll detail it in the design discussion when handling this issue. The goal would be to refer to contributors by name in project docs.

## W5: no scratch path per reviewer

Analysed with W3 of 2026-10-07-thaum-workflow, as one cluster, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-07-thaum-workflow.md`.

## W6: a deferred retrospective

- **Says.** The owner deferred the retrospective to a later merge; the skill defines a declined one
  and not a deferred one; the session kept the deferral across several compactions. Proposed fix: a
  deferred retrospective is offered at the moment the owner named.
- **Still applies.** `path@agent-skills@content/skills/retrospective/SKILL.md` lines 16-23 cover a
  declined retrospective only.
- **Recorded.** None.
- **Kind.** A choice the session made by judgement where no instruction covers it, and it worked. The
  skill itself says such a choice "is not missing: the installed text leaves that room on purpose"
  (lines 48-49).
- **Scope.** `goal@agent-skills@installed-text-leaves-room-to-judge`.
- **Better fix.** No change. The risk is a deferral lost to a compaction, which did not happen.
- **Proposal.** No change: room to judge, used well.
- **Default:** no change.
- **Outcome:** approved.

## W7: the transcript reviewer's severity

- **Says.** A small widening was rated critical; repaired in the session.
- **Still applies.** No: repaired by `9a994d1` and `86c6dec`. The reviewer's tiers are in
  `path@agent-skills@content/agents/transcript-reviewer.md` lines 114-118, and the retrospective
  counts such a slip as no finding.
- **Proposal.** No change: already repaired.
- **Default:** no change.
- **Outcome:** approved.

## W8: "recorded at that landing" read as "earns a head"

Analysed with W1 of 2026-10-07-thaum-workflow, as one cluster, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-07-thaum-workflow.md`. The report says it
was repaired in its session; the history places the wording it credits before that session, so the
cluster's fix covers it.

## W9: no way to audit a project as a whole

- **Still applies.** Yes. **Recorded:** `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`,
  as the report says.
- **Proposal.** No change: already recorded.
- **Default:** no change.
- **Outcome:** approved.

## C1: the empty-id finding does not name the placeholder repair

Analysed with C1 of 2026-10-07-thaum-workflow, as one cluster, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-07-thaum-workflow.md`.

## C2: an empty-head span is reported malformed

- **Still applies.** Yes, `path@core@src/entity.rs` lines 666-670. **Recorded:**
  `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`, as the report says. P5 of
  this repository's project file is the same defect.
- **Proposal.** No change: already recorded.
- **Default:** no change.
- **Outcome:** approved.

## C3: a bare slug that names a deleted entry is never reported

- **Says.** A one-segment span is silent by design, so a comment naming a deleted entry by its bare
  slug dangles unseen; reviewers found three on two branches. Proposed fix: `show`, or a new command,
  lists every bare mention of a slug, which the session deleting the entry runs.
- **Still applies.** Yes: "One segment is a name rather than a pointer"
  (`path@core@docs/design.md`, under `design@core@candidate-rule-and-retired-forms`). `show` lists
  checked references only. The installed text tells a session to grep only for a file the checker does
  not read, or for a rename.
- **Recorded.** Not as such. The rejected alternative "The retired slug reference reported whatever it
  names", in `path@core@docs/rejected-alternatives.md`, lost because in other projects every match is
  their own notation; that covers a check, not a listing asked for one slug.
- **Tripwire.** `tripwire@core@candidate-rule-silence` fires on "a backticked span that was meant as
  a reference and for which `cargo klarch check` reported nothing". If the three bare slugs were
  backticked, it fired; the file does not say, and the sites were repaired before the merge. Its
  response offers two moves: widen the candidate rule, or re-accept the silence knowingly. The
  proposed listing is a third: keep the silence, and give the session that deletes an entry a way to
  find the bare mentions.
- **Kind.** A blind spot, by design.
- **Scope.** Core.
- **Better fix.** The report's, as a listing rather than a check, recorded as the tripwire's
  re-acceptance with its compensating tool.
- **Route.** Rust work, after a small decision.
- **Proposal.** Open an issue: core, `todo`, naming the tripwire. Whether the tripwire fired is
  judged in the section "Standing entries: two tripwires the file comes near" below.
- **Default:** open an issue.
- **Outcome:** I think that the best fix would be to check and report as findings any bare mention of a declared slug. Maybe it is too costly, unsure about this. Ooen an issue.

## C4: no way to declare a path whose references must resolve in every installing project

- **Still applies.** Yes. **Recorded:** `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`,
  as the report says.
- **Proposal.** No change: already recorded.
- **Default:** no change.
- **Outcome:** approved.

## The standing answers

- **The primer in subagents:** analysed with the same answer of the other files, in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-06-thaum-workflow.md`.
- **An installed skill changed:** yes, as the subject of the work, not to work around an instruction.
  The trigger of `issue@agent-skills@patching-an-installed-skill` asks for a session that needed to
  change or remove an instruction for its own work; it is not met.
- **The other two answers** name no gap.
- **Default:** no change; the trigger of `issue@agent-skills@patching-an-installed-skill` is not met.
- **Outcome:** approved.

## Standing entries: two tripwires the file comes near

- **`tripwire@agent-skills@a-head-verdict-is-overruled`**, on W1 and W2. It fires the second time
  the owner overrules a review's verdict on a head. The owner's words in that session, "Me saying
  "They should be kept anyway" was me thinking that those heads would pass other record tests, not
  a direct ruling", are recorded in the message of the commit "A decision-record review read test
  4's new shape", which also says "So it is no overruling of a review's verdict". Judgement
  proposed: not fired.
- **`tripwire@core@candidate-rule-silence`**, on C3. It fires on "a backticked span that was meant
  as a reference and for which `cargo klarch check` reported nothing". The file does not say whether
  the three bare slugs were backticked, and the sites were repaired before the merge, so the
  evidence is not established. The C3 section above proposed to judge it fired; the issue opened
  for C3, `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`, records it as not
  established. Judgement proposed: not established, so no response now; the issue's census will
  show whether such spans are backticked.
- **Default:** the first not fired; the second not established, carried by the issue.
- **Outcome:** approved.

## Standing entries: three tripwires read at the search and not judged in this analysis

- **`tripwire@agent-skills@head-created-without-deliberation`**, on W8. It fires when a commit
  creates a design head, or contradicts one, and neither its message nor a plan document carries the
  deliberation. W8 reports a head written for an issue fix with no entry test applied, then removed
  in that session. Whether the commit that wrote it carried a deliberation is not established from
  the file, and was not checked against this repository's history. Judgement proposed: not
  established.
- **`tripwire@agent-skills@ruling-lost-in-assembly`**, on W2 and W7. It fires on a ruling found
  missing from, or misstated in, a committed plan document after its reviews, where the misstatement
  is critical. W2 is about classifying the owner's words when recording heads, not about assembling
  a plan document; W7's widening was minor and caught by the transcript reviewer. Judgement proposed:
  not fired.
- **`tripwire@agent-skills@a-shortcut-decision-earns-a-head`**, on W8. It fires on a decision taken
  on the bounded path that passes an entry test, at the second instance. W8's head was written on
  the in-change path, not the bounded one. Judgement proposed: not fired.
- **Default:** the first not established, with no response; the two others not fired.
- **Outcome:** approved, in the owner's words "All defaults approved, go ahead".
