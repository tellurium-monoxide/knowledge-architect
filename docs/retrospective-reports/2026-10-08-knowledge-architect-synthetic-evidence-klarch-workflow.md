# Analysis of 2026-10-08-knowledge-architect-synthetic-evidence-klarch-workflow

- **Received file:** 2026-10-08-knowledge-architect-synthetic-evidence-klarch-workflow.md, the
  workflow's side of this repository's own retrospective of the synthetic-evidence branch. The
  project's side is analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence.md`.
- **Version it used:** built from this repository's source, main at v0.4.0-64-g9c1dfe5.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1. Eight workflow files were received together
  and searched once; this file holds the clusters K1 and K3, and the judgement of one standing
  entry that no received file meets.
- **Standing entries that bear on it:**
  - `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`: W1 is about this tripwire;
    the owner ruled it not fired at the time, and narrowed it. See W1.
  - `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`
    (deferred): W2 names an outcome deferred to a harvest and written into the deleting commit. It
    is none of the entry's four cases: the transcript review found that the outcome had never been
    given, not that it sat in the deleted spec.
  - `design@agent-skills@standing-entries-searched-before-the-work` decides where the search runs,
    and says nothing of where its outcomes are recorded.
  - `tripwire@agent-skills@search-missed-before-the-work` concerns an entry the search did not
    return, a different failure from W2. No received file reports such an entry.
  - `tripwire@core@fix-refusal-routed-around`: its re-entry is each retrospective intake. See its
    section.

## W1: a tripwire on a procedure states no bound on how much the failure must matter

- **Says.** The tripwire on a skipped spec audit fired, by its letter, on a stale restatement of
  one sentence. The standing-state reviewer proposed "fired" as the default, which would reopen a
  recorded decision. The owner ruled it not fired: "The point of this kind of tripwires is to guard
  against critical failures in the workflow, not small mistakes that are not load bearing." The
  report proposes that the tripwire shape asks a tripwire on a procedure to name how much the
  failure must matter, or that the standing-state reviewer weighs a firing against the cost of its
  response.
- **Still applies.** Partly repaired. The one tripwire met is bounded since 48114b1, "The tripwire
  on a skipped spec audit fires only on a critical gap, on the owner's word":
  `path@agent-skills@docs/tripwires.md`:110-112, "A gap is critical when the work's result, or a
  decision it rests on, is wrong without its repair." `tripwire@agent-skills@ruling-lost-in-assembly`
  carries a bound too, through `design@agent-skills@misstated-ruling-rated-by-what-it-changes`. The
  general shape is unchanged:
  `path@agent-skills@content/skills/issue-tracking/SKILL.md`:235-236, "**the firing evidence**,
  meeting the falsifiability bar: an event, or a count crossing a bound, specific enough that both
  parties would agree it fired." The standing-state reviewer asks "For each: has it fired?"
  (`path@agent-skills@content/agents/standing-state-reviewer.md`:75). Four tripwires on a procedure
  still fire on one instance with no bound: `tripwire@agent-skills@ruling-lost-in-change`, whose
  sibling on assembly is bounded, `tripwire@agent-skills@member-beyond-the-argument`,
  `tripwire@agent-skills@rule-title-wider-than-its-argument` and
  `tripwire@agent-skills@pointer-not-followed`.
- **Recorded.** No issue records the general shape. No rejected alternative of agent-skills names
  a severity bound on a tripwire.
- **Kind.** A defect of the text, missing: the shape admits a firing clause whose response, a
  reopened decision, is out of proportion with the evidence. One real case, ruled by the owner.
  The other four tripwires have not been misread yet; their exposure is a reading of their text.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`: a decision reopened on a
  nitpick costs the owner a ruling. Owned by agent-skills: the tripwire shape in the issue-tracking
  skill, its restatement in the design skill (`path@agent-skills@content/skills/design/SKILL.md`:626-627),
  and the entries of `path@agent-skills@docs/tripwires.md`. Writing a bound into each of the four
  tripwires changes what each guards; each was written on the owner's word, so each change goes to
  the owner, per `skill@knowledge-architect-decision-recording@ruled-tripwires`.
- **Better fix.** The nearest rival is the report's second proposal: the standing-state reviewer
  weighs a firing against its cost. It moves the judgement to the reader, case by case, which is
  the reading the owner had to correct; a bound in the firing clause is read the same way by every
  reader. A second rival is no change, since the owner can rule each case: one ruling was already
  spent this way, and every ruling on a nitpick repeats it. The bound in the shape, stated once
  ("a tripwire on a procedure names the failure that matters: one without whose repair the work's
  result, or a decision it rests on, is wrong"), then a proposal to the owner for each of the four
  tripwires, is the fix.
- **Route.** A decision about the tripwire shape, under `skill@knowledge-architect-design`, and
  four tripwire changes on the owner's word. Not small enough for this intake's branch.
- **Proposal.** Open an issue, kind `design`, in agent-skills: the tripwire shape states no bound
  on how much a procedure's failure must matter, with the four unbounded tripwires as instances and
  the owner's words as its ground.

- **Default:** open an issue, `design`, in agent-skills.
- **Outcome:** approved.

## W2, with W1 of 2026-10-08-thaum-slice-4g-klarch-workflow and W5 of 2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow: no text says where the outcome of each returned standing entry is written (cluster K1)

- **Says.** Three sessions, two projects. Each ran the standing-entry search, judged the returned
  entries, wrote outcomes only for those that bore on the work, and the transcript reviewer then
  reported the rest as items with no durable outcome: nine entries in this file's spec, 19 of 41 in
  the thaum slice audit, 3 of 18 in the agent-configuration-entities audit. Each was repaired by
  writing the outcomes into a commit message, twice by a history edit. The proposals: the spec's
  "What the work is" holds the returned entries with their outcomes; the audit's commit lists every
  returned entry with its outcome.
- **Still applies.** Yes, at every site.
  - The design discussion,
    `path@agent-skills@content/skills/design/SKILL.md`:457-459: "The first round states which of
    these were read and run, which entries the search returned, and which of them the question
    bears on." This is a statement in the conversation, and "bears on" filters.
  - The spec's sections, `path@agent-skills@content/skills/planning/SKILL.md`:160-182: no row
    holds the returned entries.
  - The design audit, same file, :376-377: "Read whole, with `{{command}} show`, every entry the
    search returns, and judge it against the slice; never act on an entry from the reason the
    search gave." Its commit carries "a message listing each finding: the gap, the answer and the
    decision it follows from" (:384). An entry judged not to bear on the slice is no gap, so its
    judgement has no home.
  - The transcript reviewer counts "a finding of a reviewer or of another agent" among the items
    owed an outcome (`path@agent-skills@content/agents/transcript-reviewer.md`:76), so it reports
    them. The two texts disagree in practice.
- **Recorded.** No issue, no rejected alternative. The nearest heads,
  `design@agent-skills@standing-entries-searched-before-the-work` and
  `design@agent-skills@standing-entry-search-agent`, say nothing on the record of the outcomes.
- **Kind.** A defect of the text, unclear: the search's instruction asks for a judgement of every
  returned entry and gives a home only to those that are gaps. Three instances in three sessions,
  two projects, each repaired at a cost the reviewer imposed after the fact. It meets
  `design@agent-skills@behaviour-additions-need-real-use`.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Owned by
  agent-skills: the planning skill, at the assembly and at the design audit. It reverses no head.
- **Better fix.** The nearest rival is to narrow the transcript reviewer: a returned entry judged
  not to bear needs no outcome. It loses because the judgement "does not bear" is the very claim a
  later reader cannot check without it, and an entry that did bear and was dismissed is the failure
  the search exists to prevent. A second rival is a section of its own in the spec; one line per
  entry in the existing places costs less. The fix: in the design audit, the audit's commit message
  lists every returned entry with its outcome, one line each, an entry judged not to bear included;
  in the assembly, the plan document's "What is already decided" carries, beside the decisions, the
  standing entries the grounding search returned and the outcome of each. The second follows the
  design skill's first-round statement into the record, where the first-round statement stays.
- **Route.** A text edit whose design is settled, at the planning skill's source, under the
  agent-skills `CLAUDE.md`, "Editing an installed skill or agent".
- **Proposal.** Handle now, with the two sentences above.

- **Default:** handle now.
- **Outcome:** approved.

## Read-back, second item, with W5 of 2026-10-09-knowledge-architect-staged-check-klarch-workflow: the plan reviews apply a one-answer gap in place even where it changes a ruling (cluster K3)

- **Says.** Twice, a finding of a plan review had one answer, was applied in place as a gap, and
  changed what the owner had ruled: here a label kept beside a slug that is gained later, which
  widened a ruling; in the staged-check spec, a mechanism the owner approved, a clap declaration,
  replaced by a hand-written test, which narrowed one. Both were caught by a later reviewer and went
  back to the owner. The staged-check report proposes a test: an answer that changes a mechanism, a
  name or an interface the owner was shown is a scope change, however few answers it has.
- **Still applies.** Yes. `path@agent-skills@content/skills/planning/SKILL.md`:507-512, in
  `skill@knowledge-architect-planning@plan-reviews`: "A finding that is a gap with one answer is
  applied in place. … A scope change, a clause that widens or narrows a ruling or adds an
  obligation to it, is put to the owner". Nothing says which wins when both describe one finding.
  The design audit of the same skill already settles it, :386-387: "**An answer that widens or
  narrows a ruling of the owner, or adds an obligation to one, is a scope change even when it is
  the one answer the document implies.**"
- **Recorded.** No issue, no rejected alternative.
  `tripwire@agent-skills@ruling-lost-in-assembly` fires only after the plan's own reviews; here the
  re-review caught it, so it did not fire.
- **Kind.** A defect of the text, unclear: two rules of one paragraph overlap, and the precedence
  is stated in another section only. Two instances.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`. Owned by agent-skills, the
  planning skill. It reverses nothing: it carries the design audit's rule to the plan reviews.
- **Better fix.** The report's test (a mechanism, a name, an interface the owner was shown) is a
  list of members; the design audit's sentence states the rule that admits them, and is already
  the skill's own wording. Reusing it is the fix. No change is the rival; it leaves the same
  question to be decided by a reviewer each time.
- **Route.** A text edit whose design is settled, at the planning skill's source.
- **Proposal.** Handle now: the plan-reviews paragraph gains the design audit's sentence, so a
  one-answer gap that widens or narrows a ruling is put to the owner.

- **Default:** handle now.
- **Outcome:** approved.

## Read-back, first and third items: the assembly written from memory, a commit message's claim

- **Says.** The spec was written before the assembly subagent returned, and one approved default
  was dropped; a harvest message carried a false claim.
- **Still applies.** The instructions stand: `path@agent-skills@content/skills/planning/SKILL.md`:238-239,
  "It is assembled from the transcript, not from memory", and the root
  `path@knowledge-architect@CLAUDE.md`:334, "**Before each commit, list every factual claim its
  message makes, and check each one.**"
- **Kind.** Lapses: each instruction exists, is clear and was not followed. The commit-message
  claim recurs across four files; it is analysed as cluster K7 in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence.md`.
- **Proposal.** No change here.

- **Default:** no change, lapses; the claim check is carried by K7.
- **Outcome:** approved.

## Standing answers

- **The primer in subagents.** The question was then open as an issue; 61a9219, "The primer bounds
  a word of the owner by its premise, and the retrospective asks what the primer's absence looked
  like", closed it, and the retrospective now asks "Did this session, or one of its subagents, act
  as if a rule of the primer were absent?"
- **The routing table.** It is gone, per `design@agent-skills@no-routing-table`.
- **Plain `#<id>` for plan items.** True of the text; the form is cluster K4, analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`.

- **Default:** no change.
- **Outcome:** approved.

## Standing entry: `tripwire@core@fix-refusal-routed-around`, whose re-entry is each intake

- **Judgement.** Not fired. It fires on a session that unstaged changes after `check --fix`
  refused a partial commit's mismatch. That refusal landed in dfe33ba, "[core] Step 4 of check
  --staged: check --fix refuses a partial commit's mismatch", on 2026-10-09, and is unreleased. The
  thaum files used 0.5.0, which has no refusal. Of this repository's files, none reports a refusal
  met: the staged-check session built it, and the path-quickfixes and design-record-audit files
  report no `--fix` refusal.

- **Default:** not fired.
- **Outcome:** approved.
