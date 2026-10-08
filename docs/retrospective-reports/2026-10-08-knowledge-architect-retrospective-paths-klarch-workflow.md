# Analysis of 2026-10-08-knowledge-architect-retrospective-paths-klarch-workflow

- **Received file:** 2026-10-08-knowledge-architect-retrospective-paths-klarch-workflow.md, the
  workflow's side of this repository's own retrospective. The project's side is analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-retrospective-paths.md`.
- **Version it used:** built from this repository's source, main at v0.4.0-76-gd2c830d.
- **Analysed against:** main at v0.4.0-77-g6fe677b, the merge of the reported session's branch.
- **Standing entries that bear on it:**
  - `tripwire@agent-skills@title-stops-stating-scope`: W1 is its firing evidence. See its section.
  - `tripwire@agent-skills@head-created-without-deliberation`: W1 is evidence against its premise.
    See its section.
  - `issue@agent-skills@the-primer-question-cannot-be-answered-for-subagents`: the standing answer
    on the primer is one more instance of it.
  - `issue@agent-skills@patching-an-installed-skill`: the standing answer on changing an installed
    skill is read against its trigger.
  - Read whole and judged not to bear beyond a shared subject:
    - `issue@agent-skills@a-word-given-on-a-false-premise-outside-a-discussion`, another rule
      delivered only by a skill the session has not loaded;
    - `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, whose
      re-entry is this intake's search;
    - `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` and
      `issue@agent-skills@test-3-admits-a-practice-its-tool-documents`, on the entry tests;
    - `issue@core@a-contract-change-fails-every-earlier-commit-unexplained`, on folding;
    - `issue@agent-skills@expectation-sets-for-the-installed-skills`,
      `issue@agent-skills@the-retrospective-counts-no-review-cost`,
      `issue@agent-skills@a-reviewer-s-running-time-is-unbounded` and
      `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`;
    - `tripwire@agent-skills@structure-rejection-overruled`,
      `tripwire@agent-skills@a-head-verdict-is-overruled`,
      `tripwire@agent-skills@a-shortcut-decision-earns-a-head` and
      `tripwire@agent-skills@ruling-lost-in-change`. No rejection was overruled, no verdict on a
      head was overruled, the session took no bounded path of the design skill, and no
      discussion ran on the in-change path.

## W1, with P1 of the project's file: the condition that sends a session to decision-recording needs that skill's title test to judge

- **Says.** The session edited the body of `design@agent-skills@retrospective-destination` without
  loading `knowledge-architect-decision-recording`. The added sentence carried a decision that the
  head's title does not state. The session judged only whether the edit contradicted the head.
  The decision-record review caught it before the merge. Proposed fix: an edit that writes into a
  design head loads decision-recording before the edit, with no condition to judge first.
  Optionally, add the symptom "before writing into a design home" to the description of
  decision-recording.
- **Still applies.** The same condition stands at three sites. Each needs the title test to judge:
  - `path@agent-skills@content/skills/agent-configuration/SKILL.md`:30, §1: "An edit that
    contradicts a statement of a head, or takes one beyond what its title states, follows that
    skill."
  - `path@agent-skills@CLAUDE.md`:61, test 4, the same sentence ending "is a decision, under
    `knowledge-architect-decision-recording`".
  - the description of the installed design skill: "the decision creates a design head,
    contradicts a statement of one, or takes one beyond what its title states".

  The title test is in `path@agent-skills@content/skills/decision-recording/SKILL.md`:224, §5: "A
  title states a decision only while it is false of the nearest rival it beat." The backstop that
  applies it is at line 118 of the same file, §2. Neither reaches a session that has not loaded the
  skill. The description of decision-recording triggers on "when a design decision has been made or
  reversed". An owner's request to change one instruction did not read as that symptom.
- **Recorded.** No issue. Both tripwires guarding `design@agent-skills@new-or-reshaped-head-needs-design`
  watch this premise; see their sections. The rejected alternatives of agent-skills hold "a line in
  the primer that sends a decision met during another task to the design skill". It lost to
  `design@agent-skills@primer-limit`. The fix proposed here is not that line, since it is delivered
  by a skill's description and by the two existing sites. No recorded alternative proposes an
  unconditional load before writing into a design home.
  `git log --grep` on the two tripwire slugs finds no earlier firing.
- **Kind.** A defect of the text, of the delivery kind: the instruction is read at one moment and
  its standard is held at another. The condition to load a skill is stated in terms that only the
  loaded skill defines. The session's partial reading is a lapse too, but adding text does not
  repair that part. Moving where the standard reaches the session does. This is the first observed
  instance. The structure, a load conditioned on a test held by the skill to load, makes it
  recurrent by construction rather than by chance.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides` and
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`: a decision written into a head
  with no deliberation and a paraphrased ruling is what both guard against. The fix is owned by
  agent-skills, at two installed texts and one description. It is mirrored in the project's scoped
  `path@agent-skills@CLAUDE.md`, which is P1. It does not reverse
  `design@agent-skills@new-or-reshaped-head-needs-design`: the three cases and the title test stay
  as they are, inside decision-recording. It changes a statement of that head's delivery paragraph,
  "Two texts deliver it", at line 203 of `path@agent-skills@docs/design.md`. Under that head's own
  rule, a change to a statement of a head is argued under the design skill.
- **Better fix.** The rivals, each with the fact that decides it:
  - **The owner's direction: any write into a design home loads decision-recording first.** The
    trigger is an act a session can see without any standard: the file it is about to edit. The
    skill then judges contradiction, extension and whether the change earns text at all. Cost: a
    load on every head edit, including rewordings that the skill then sends away.
  - **Restate the title test beside the condition at each site.** This keeps the condition and
    gives the session the standard. It creates three copies of a test whose home is §5, and they
    would drift. It still reaches only sessions that read one of those sites, not an issue fix or a
    review repair that writes a head.
  - **A line in the primer.** This lost to `design@agent-skills@primer-limit`. A skill's description
    reaches every session and is the delivery that head prefers.
  - **No change, as a lapse.** Refused by the evidence. The session met all three sites, with two
    of them loaded or read before the edit, and none of them caused the load, because each needed
    the unloaded standard.

  The deciding fact is that the act, writing into a design home, is visible before any judgement.
  So the owner's direction is the one fix that does not depend on the standard it delivers. It
  passes the four tests of `path@agent-skills@CLAUDE.md`:
  - scope: it is about the installed text, not the owner's behaviour;
  - necessity: an observation from this session, the owner's named lack ("The trigger paragraph
    testing complex condition is a problem"), and the mechanism in one sentence: a load
    conditioned on a test held by the skill to load fires only when the session already applies
    that test;
  - kind: a replacement of a conditional rule by an unconditional one, not a new structure;
  - built intent: it changes the delivery paragraph of
    `design@agent-skills@new-or-reshaped-head-needs-design`, see Scope.
- **Route.** `knowledge-architect-design`, as bounded work: one proposal with its default, then the
  owner's word. Then the edit under `knowledge-architect-agent-configuration` and the four tests.
  The sites:
  - the description of decision-recording gains the symptom;
  - §1 of agent-configuration and test 4 of `path@agent-skills@CLAUDE.md` load the skill whenever
    an edit writes into a design head, or changes behaviour a head describes;
  - the delivery paragraph of `design@agent-skills@new-or-reshaped-head-needs-design` is rewritten
    in place.

  Whether the design skill's description keeps its three-case symptom is a question for that
  discussion.
- **Proposal.** Handle now, on the owner's direction, through the design skill's bounded path.
- **Default:** handle now.
- **Outcome:** approved. The owner's words: "all defaults approved. I still lean in the same direction, unconditional trigger on writing in a design home."

## W2: review repairs folded into the reviewed commit without the fold condition

- **Says.** The session squashed content repairs and a message repair into the single reviewed
  commit. The review skill folds a repair only where an appended one would leave an earlier commit
  failing. Proposed fix: none.
- **Still applies.** `path@agent-skills@content/skills/review/SKILL.md`:161 to 163: "A repair made
  on the branch is a new commit, appended, which edits no history. Where the project requires every
  commit of a branch to pass checks the repair changes, an appended repair leaves the earlier
  commits failing; it is then folded into the earliest commit it repairs". The root `CLAUDE.md`,
  `## Git` point 2, restates it.
- **Recorded.** None. `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` concerns
  the fold that is owed, not one that is not.
- **Kind.** A lapse. The instruction is clear, and the session's own file says so. The merged commit
  records the fold, so nothing was hidden. No content was lost: the session checked that the diff
  against the pre-squash head was empty.
- **Scope.** The installed review skill. No goal is threatened by one instance.
- **Better fix.** Nearest rival: an addition to the review skill on combining a message repair with
  a content repair. The text already orders both: an amend or a history edit for the message, and an
  appended commit for the content. A lapse is not repaired by adding text, per
  `design@agent-skills@capabilities-not-structure`.
- **Route.** Nothing.
- **Proposal.** No change: a lapse, recorded in the received file.
- **Default:** no change.
- **Outcome:** approved. The owner's words: "all defaults approved."

## Standing answers

- **Changing an installed skill.** The session changed one as its own work, at the owner's request,
  in the workflow's upstream. The trigger of `issue@agent-skills@patching-an-installed-skill` needs
  a session that *needed* to change or remove an instruction of an installed skill in its project
  and could not. This session could, being the upstream. Not met.
- **The primer in subagents.** The answer is "could not be established", the same answer as the
  four instances in `issue@agent-skills@the-primer-question-cannot-be-answered-for-subagents`. Rival:
  add it as a fifth instance. It adds a count to a question that already has the evidence it needs.
- **Project skill additions, references without backticks.** No and no. Nothing to act on.
- **Proposal.** No change on any of these.
- **Default:** no change.
- **Outcome:** approved. The owner's words: "all defaults approved."

## Tripwire: title-stops-stating-scope

- **Fires when:** "a review finds a design head whose title no longer states a decision added to
  its body by the direct route: an addition whose commit carries no deliberation."
- **Evidence.** The decision-record review of the reported branch found exactly this: the sentence
  added to the body of `design@agent-skills@retrospective-destination` stated a decision its title
  does not state. The commit carried a paraphrase of the owner and no rival. The branch's
  standing-state reviewer judged it not fired because the commit carried the owner's request. A
  request is not a deliberation: no rival, no thread, and no quotation. The review found it before
  the merge, and the sentence never reached main. The clause does not require that it reach main.
- **Judgement.** Fired.
- **Response, as written:** open a `defect` naming the head and the commit, and reopen
  `design@agent-skills@new-or-reshaped-head-needs-design` on its third case. Proposed instead:
  - The reopening is W1's design run. The cause was not the third case's rule, which caught the
    edit as soon as it was applied. The cause was that the rule's skill was never loaded. So the
    run is on the head's delivery, not on its third case.
  - No defect is opened. Nothing on main is defective, and the cause is handled now under W1. A
    defect entry would open and close in the same branch.

  The commit that carries W1 names this firing.
- **Default:** fired; response through W1, with no defect entry.
- **Outcome:** approved. The owner's words: "all defaults approved."

## Tripwire: head-created-without-deliberation

- **Fires when:** "a commit creates a design head, or contradicts a statement of one, and neither
  its message nor a plan document carries the deliberation".
- **Evidence.** The reported commit extended a head beyond its title. It created none and
  contradicted none, and the extension is the third case, which this tripwire does not name. Its
  premise, that "the design skill's description and the decision-recording skill's backstop reach a
  session before it writes", failed here for the third case. This is the same evidence as W1.
- **Judgement.** Not fired as written. Its response, to reopen the head on where its trigger is
  delivered, is what W1 does anyway.
- **Default:** not fired; no action beyond W1.
- **Outcome:** approved. The owner's words: "all defaults approved."
