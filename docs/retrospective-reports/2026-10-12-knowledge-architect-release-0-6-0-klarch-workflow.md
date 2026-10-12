# 2026-10-12-knowledge-architect-release-0-6-0-klarch-workflow

- **Received file**: 2026-10-12-knowledge-architect-release-0-6-0-klarch-workflow.md, the workflow
  file of the retrospective of the session that released 0.6.0.
- **Version used**: the checker built from this repository, main at `v0.5.0-155-g2ec021c`, branch
  `release-0.6.0`, no uncommitted changes.
- **Analysed against**: main at `v0.6.0`.
- **Standing entries that bear on it**: the search over the 72 issues and 51 tripwires, shared
  with the project file's analysis, returned these for this file:
  - `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it`, the gap W1 widens;
  - `tripwire@agent-skills@audit-edit-leaves-an-entry-stale`, which sends edits outside the
    design-record axis to that issue;
  - `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`, the gap in `show`
    that a step built on it meets;
  - `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow`, on W2's subject;
  - four tripwires whose firing clauses W1 or W2 come near, judged in their own sections below:
    `tripwire@agent-skills@member-beyond-the-argument`,
    `tripwire@agent-skills@head-created-without-deliberation`,
    `tripwire@agent-skills@owner-intent-stripped` and
    `tripwire@agent-skills@ruling-lost-in-change`.

## W1. Adding a member to a head asks for no reading of the texts that cite it, or of the head's own argument

- **a. What it says.** The session added a member to the bump table of
  `design@knowledge-architect@versioning-policy` on the owner's word, ran no `cargo klarch show` on
  the head, and left two texts false: the head's own closing paragraph, and a sentence of
  `design@core@ne-minimal` that relied on the old reading. Three reviewers found them before the
  merge. The fix proposed: in the closing section of the decision-recording skill, for every head a
  change edits, run `cargo klarch show` on it and read each citing text, and re-read the head's
  whole body against the change.
- **b. Whether it still applies.** Yes.
  `path@agent-skills@content/skills/decision-recording/SKILL.md`:259-277, "Before you finish", asks
  for the check, the incumbent search, a re-read of "each head you wrote: present tense", the entry
  tests again, and a re-read "against the head you replaced". It names no reading of the texts
  that cite an edited head, and no re-read of the rest of the head's body. The reversal check, lines
  40-82, has "Repair what pointed at the old behaviour", for a reversal only.
- **c. Whether it is already recorded.**
  `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it` records the same gap for
  an audit's repair: two issue entries left false of heads an audit edited. This is a third
  instance, outside any audit, and the texts left false were a design head of another Component
  and the head's own argument, not issues or tripwires.
- **d. Its kind.** Missing text. The primer's "a reference is a claim that this head is revisited
  when that entry changes" states why a citing text must be revisited, and no step delivers the
  revisit except at a reversal or a rename, where the checker reports a dangling reference. An edit
  that keeps the slug dangles nothing, so nothing reports it.
- **e. Its scope.** The agent-skills Component, `skill@knowledge-architect-decision-recording`,
  which the primer has a session load before any write into a design home. It serves
  `goal@knowledge-architect@any-project-can-adopt-it` through the record staying true. It reverses
  no head. The four tests of `path@agent-skills@CLAUDE.md`: scope, the skill has no expectation set
  and the finding is not about the owner; necessity, three instances in real use, two in an audit
  and this one; kind, a missing capability, a step with a command; built intent, it applies the
  primer's reference rule and contradicts no head.
- **f. A better fix.** Rivals:
  - a checker command listing what cites an entry, as
    `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` asks. It would ease
    the step, and would not deliver it: a session still has to know to run it.
  - the fix in the audit skill only, as the open issue proposes. It misses this instance, which no
    audit ran. Since the audit skill sends the recording of a head it edits to the decision-recording
    skill, one step there covers both.

  So the step goes in the decision-recording skill, and it answers the open issue's question: the
  step that reads the citing entries is in the skill every edit of a head loads. The default closes
  `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it` with that answer.
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent"; a Workflow entry in CHANGELOG.md's `Next release`, since a
  watcher sees a new command run before every commit that edits a head.
- **h. The proposal.** In "Before you finish": for each head the change edits, run
  `cargo klarch show` on it and read every text that cites it, and re-read the head's whole body,
  its argument included, against the change; a text left false is repaired in the change. Close
  the open issue in the same commit.
- **Default**: handle now, and close
  `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it` with it.
- **Outcome**: handle now, and close `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it` with it. The owner: "All defaults approved, proceed"

## W2. The owner's hedge was dropped from a quoted ground

- **a. What it says.** The owner's ruling began "IMO, …". The head quoted it from the second word.
  The decision-record reviewer caught it before the merge.
- **b. Whether it still applies.** No text to change: the retrospective calls the instruction clear
  and the miss the session's. The head on main quotes the ruling whole, from "IMO".
- **c. Whether it is already recorded.**
  `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` is the open design question
  on the place and weight of the owner's quotations. This instance is no evidence for it either way:
  the hedge was restored.
- **d. Its kind.** A lapse of the session, caught by the review, which exists for that rate.
- **e. Its scope.** None.
- **f. A better fix.** None: adding text does not repair a lapse, per
  `design@agent-skills@capabilities-not-structure`.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change: a lapse, repaired before the merge.
- **Outcome**: no change, a lapse repaired before the merge. The owner: "Lets act on the retrospective findings right now. We'll run the intake procedure anyway, but IMO all of your suggestions are good. For P3, owner reading can be optional."

## The standing questions

- **a. What it says.** No installed file was changed; the session acted as if three rules of the
  primer were absent, the two of W1 and the one of W2, all caught before the merge; no project
  skill's addition was missed; no reference was written without backticks.
- **b to g.** W1 and W2 carry the findings. No further action.
- **Default**: no change: noted.
- **Outcome**: no change: noted. The owner: "All defaults approved, proceed"

## `tripwire@agent-skills@member-beyond-the-argument`

- **a. What it says.** It fires when a review names a member recorded directly, as covered by its
  head's argument, that the argument does not cover.
- **b. The evidence.** The looser-check row was recorded on the owner's ruling, quoted in the head
  and in the commit "A looser check is a patch, with a Migration entry", not as covered by the
  argument. The reviewers named texts it left false, not a member taken as covered by an argument.
- **h. The proposal.** Not fired.
- **Default**: not fired.
- **Outcome**: not fired. The owner: "All defaults approved, proceed"

## `tripwire@agent-skills@head-created-without-deliberation`

- **a. What it says.** It fires when a commit creates a head, contradicts one, or adds a member its
  argument does not cover, with no deliberation in its message or a plan document.
- **b. The evidence.** The commit "A looser check is a patch, with a Migration entry" carries the
  owner's ruling, the consequence put to the owner, the default and the owner's answer, and the
  incumbent search.
- **h. The proposal.** Not fired.
- **Default**: not fired.
- **Outcome**: not fired. The owner: "All defaults approved, proceed"

## `tripwire@agent-skills@owner-intent-stripped` and `tripwire@agent-skills@ruling-lost-in-change`

- **a. What they say.** The first fires when a commit removes or rewords as an argument a quotation
  of the owner's own intent in a head; the second when a ruling is missing or misstated in an
  in-change commit message after the transcript review.
- **b. The evidence.** W2's trimmed quotation was neither removed nor reworded as an argument, and
  was repaired before the merge. The commit message quoted the ruling whole, "IMO" included.
- **h. The proposal.** Neither fired.
- **Default**: not fired.
- **Outcome**: not fired. The owner: "All defaults approved, proceed"
