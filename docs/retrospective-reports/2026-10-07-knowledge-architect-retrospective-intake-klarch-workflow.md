# Analysis of 2026-10-07-knowledge-architect-retrospective-intake-klarch-workflow

- **Received file:** 2026-10-07-knowledge-architect-retrospective-intake-klarch-workflow.md, the
  workflow's side of this repository's own retrospective.
- **Version it used:** built from this repository's source; the session ended on main at
  v0.4.0-59-gf983d4f.
- **Analysed against:** main at v0.4.0-59-gf983d4f, the same.
- **Standing entries that bear on it:**
  `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` (W1),
  `issue@agent-skills@a-word-given-on-a-false-premise-outside-a-discussion` (W1),
  `issue@agent-skills@the-primer-question-cannot-be-answered-for-subagents` (the standing answer on
  the primer), `issue@agent-skills@patching-an-installed-skill` (the standing answer on changed
  skills), `tripwire@agent-skills@head-created-without-deliberation` (W2).

## W1: test 2 was applied before the change had written its second site, twice

- **Says.** Twice, the session judged whether a decision earned a head while the change was still
  writing its texts, and missed a second site the same change wrote: the retrospective's file names,
  restated in a project skill, and the scratch-directory rule, written into two installed texts.
  Proposed fix: decision-recording §8, "Before you finish", applies the entry tests once every text
  of the change is written, counting the sites the change writes.
- **Still applies.** `path@agent-skills@content/skills/decision-recording/SKILL.md`, §8, asks for
  the check, the search line, the present tense and a re-read against the replaced head; nothing in
  it applies the entry tests to the change as written. Test 2 itself, §2, is unchanged.
- **Recorded.** Not as such. `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined`
  asks what a site is for installed text; W1 is about when sites are counted, and its sites were
  plain ones, a skill restating a reason. The first case is also an instance of
  `issue@agent-skills@a-word-given-on-a-false-premise-outside-a-discussion`: the owner answered the
  test-4 question on the premise that tests 1 to 3 failed, which was false.
- **Kind.** A lapse that recurs at one point of delivery: the tests are read when a decision is
  made, at §2, and the sites exist only once the change is written. Two instances in one session.
- **Scope.** Agent-skills; `goal@knowledge-architect@the-owner-decides`, since the false premise
  reached the owner.
- **Better fix.** The report's, at §8, the skill's finish-time point, which the previous branch used
  for the same kind of lapse. Rival: a sentence in §2, read when the decision is made, which is the
  moment the lapse happens at; it would not have helped, since the second site did not exist yet.
- **Route.** A text edit of installed text, and an instance added to the false-premise issue.
- **Proposal.** Handle now: §8 gains "apply the entry tests of §2 to each decision of the change,
  now that every text of it is written: the sites the change writes count, a project's own skills
  included"; and the first case is added as an instance to
  `issue@agent-skills@a-word-given-on-a-false-premise-outside-a-discussion`. CHANGELOG `Workflow`
  entry, patch.
- **Default:** handle now.
- **Outcome:** approved.

## W2: a review repair that makes a decision is reviewed by no decision-record review unless it answers the transcript reviewer

- **Says.** The review skill sends the decision-record axis at a transcript reviewer's repair that
  makes a decision, and says nothing of a repair from another axis. A head written in a review's
  repair commit reached the main branch with no decision-record review. Proposed fix: review §3
  says that any repair commit that makes or reverses a decision is reviewed by the decision-record
  axis before the last transcript review.
- **Still applies.** `path@agent-skills@content/skills/review/SKILL.md`, §1: "Where one of them makes
  or reverses a decision, the decision-record axis reviews it", inside the paragraph on the
  transcript reviewer's repairs. `design@agent-skills@transcript-review-last-before-merge` argues it
  for that case only: "otherwise a decision taken in answer to the transcript review reaches the
  main branch with no review of its record". `design@agent-skills@review-repair-appended-or-folded`
  says nothing of a re-review. The head that reached main this way is
  `design@agent-skills@a-scratch-directory-per-subagent`, written in the commit "[review] Five
  reviewers read the retrospective-outcomes branch".
- **Recorded.** None. No rejected alternative on re-review.
- **Kind.** Missing. The argument the head gives for the transcript case holds for every axis.
- **Scope.** Agent-skills; `goal@knowledge-architect@design-is-recorded-with-its-arguments`. The fix
  widens the transcript head's rule to every repair: a decision that changes a head's statement, so
  it goes through decision-recording, recorded on the in-change path.
- **Better fix.** The report's. Rival: every axis re-runs on every repair commit. It costs a round
  per repair for repairs that only correct, which the head already argues against.
- **Route.** A decision on the in-change path, and a text edit of the review skill. Then the
  decision-record review the scratch-directory head did not get, over that commit.
- **Proposal.** Handle now: review §3 states it for every repair; the head
  `design@agent-skills@transcript-review-last-before-merge` is widened, or a head of its own carries
  it; and a decision-record reviewer reads the commit that wrote the scratch-directory head.
  CHANGELOG `Workflow` entry, patch.
- **Default:** handle now.
- **Outcome:** approved.

## Standing entry: whether `tripwire@agent-skills@head-created-without-deliberation` fired on W2

- **Evidence.** It fires when a commit creates a head and neither its message nor a plan document
  carries the deliberation. The commit that wrote the scratch-directory head carries the reason, the
  evidence of two sessions, and the rival that lost, "write nothing", with why.
- **Judgement proposed.** Not fired: the deliberation is in the message. What W2 reports is a
  missing review, not a missing deliberation.
- **Default:** not fired.
- **Outcome:** approved.

## The standing answers

- **Changed installed skills:** "Yes, as the subject of the work, not to work around an
  instruction." The trigger of `issue@agent-skills@patching-an-installed-skill` asks for a session
  that needed to change or remove an instruction for its own work: not met.
- **The primer in subagents:** "For the installed agent types, not verified." One more instance of
  `issue@agent-skills@the-primer-question-cannot-be-answered-for-subagents`, the fifth session that
  could not answer it; proposed: add it to that entry.
- **A missed project skill, and paths without backticks:** no gap.
- **Default:** add the primer instance to its issue; otherwise no change.
- **Outcome:** no need, there is enough evidence already.
