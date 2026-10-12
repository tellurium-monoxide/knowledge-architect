# 2026-10-12-knowledge-architect-release-0-6-0

- **Received file**: 2026-10-12-knowledge-architect-release-0-6-0.md, the project file of the
  retrospective of the session that released 0.6.0.
- **Version used**: the checker built from this repository, main at `v0.5.0-155-g2ec021c`, branch
  `release-0.6.0`, no uncommitted changes.
- **Analysed against**: main at `v0.6.0`.
- **Standing entries that bear on it**: the search over the 72 issues and 51 tripwires, shared
  with the workflow file's analysis, returned these for this file:
  - `issue@agent-config@klarch-release-step-12-compares-with-a-head-step-10-deleted`, P5's
    subject;
  - `issue@knowledge-architect@a-mechanical-changelog-check` and
    `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument`, beside P1; the
    second argues the released section, and P1 adds to the working section's rule only;
  - `issue@core@branch-sha-citations-are-judged-within-the-range-only`, beside P5: the head P5
    records is kept in the shell, and no text cites it;
  - `tripwire@agent-skills@axis-not-run-at-a-pin-move`, beside P1: P1 removes only entries for
    text no release shipped, which no pin move crosses, so it does not bear on the tripwire's
    premise.

## P1. A branch writes a new entry for a change to something the release has not yet shipped

- **a. What it says.** The changelog reviewer of the 0.6.0 release found entries written by
  branches of the range for changes to text that was itself new in the range: a Migration entry
  renaming a section of the audit skill that 0.5.0 never shipped, and Workflow entries tracing the
  audit skill pass by pass. The release removed three entries and members of five others. The fix
  proposed: one sentence in `design@knowledge-architect@changelog-entries`, saying that a change to
  something added since the last release edits that addition's entry and writes none of its own.
- **b. Whether it still applies.** Yes. `path@knowledge-architect@docs/design.md`:193-194 reads
  "An entry describes the release's net effect: a change reversed before the release has none." It
  names a reversed change only. `path@agent-config@agents/klarch-changelog-reviewer.md`:40-41
  restates it the same way: "an entry describes the release's net effect, so a change reversed
  inside the range has no entry."
- **c. Whether it is already recorded.** `issue@knowledge-architect@a-mechanical-changelog-check`
  would check part of the changelog at every commit; it does not state this rule. No rejected
  alternative of the root Component covers it.
- **d. Its kind.** Unclear text: "net effect" admits the reading, but states only the reversed case,
  and three branches wrote entries the other way.
- **e. Its scope.** The root Component, `goal@knowledge-architect@any-project-can-adopt-it`: an
  entry is for a consumer planning a move from the last release, and an entry about a thing that
  consumer never received tells it nothing. The fix is a new member of the head's "net effect"
  rule, which its argument covers, so it is recorded directly, per `primer@design-heads`. It
  reverses nothing.
- **f. A better fix.** The rival is the mechanical check of
  `issue@knowledge-architect@a-mechanical-changelog-check`; it cannot tell what a release shipped
  from a sentence's wording, so the text is the fix. The reviewer agent's restatement is edited in
  the same change, or it keeps the narrower reading.
- **g. Its route.** A text edit of a settled design: the head and the reviewer's restatement,
  under `skill@knowledge-architect-agent-configuration` for the agent.
- **h. The proposal.** Add to the head: "A change to something added since the last release, a
  skill, a section, a check or a command, edits the entry that announces it, and writes no entry
  of its own." Restate it in the changelog reviewer.
- **Default**: handle now.
- **Outcome**: handle now. The owner: "Lets act on the retrospective findings right now. We'll run the intake procedure anyway, but IMO all of your suggestions are good. For P3, owner reading can be optional."

## P2. Step 11 asks for the owner's word "at that moment", and the owner gave it before the merge

- **a. What it says.** The owner wrote, before the merge, "You can proceed. Once merged, you can
  publish on crates.io, I am logged in". The session took it as step 11's word. The fix proposed:
  step 11 says a word given in advance holds when the merge changed nothing beyond what the owner
  saw, which step 12's tree comparison shows.
- **b. Whether it still applies.** Yes. `path@agent-config@skills/klarch-release/SKILL.md`:100
  reads "**The owner's word, given at that moment.** Without it, nothing is published." The design
  head says the same: `path@knowledge-architect@docs/design.md`:255-256, "only then tagged and
  published from main's head, on the owner's word given at that moment, per
  `goal@knowledge-architect@the-owner-decides`."
- **c. Whether it is already recorded.** No issue, tripwire or rejected alternative covers it.
- **d. Its kind.** Unclear text.
- **e. Its scope.** The root Component's `design@knowledge-architect@publish-after-merge`, which
  serves `goal@knowledge-architect@the-owner-decides`. **The fix edits a statement of a design
  head, not only the skill**, so it goes to `skill@knowledge-architect-design` on its in-change
  path before its text is written, per the backstop of `primer@design-heads`. It reverses nothing
  the head argues: the head's argument is that the irreversible step comes after review and CI
  judged exactly what it publishes, and a word given in advance and conditioned on the merge keeps
  that, as long as the tree published is the tree reviewed.
- **f. A better fix.** The rival is to keep "at that moment" and have the session ask again after
  the merge. It costs the owner one more message and protects nothing the tree comparison of step
  12 does not: the owner rules on the reviewed tree either way. A word that does not name the
  publish, such as "you can proceed" alone, is still no word to publish.
- **g. Its route.** `skill@knowledge-architect-design`, in-change path, then the edit of the head
  and of step 11.
- **h. The proposal.** The head: "on the owner's word, given after the review, before or after the
  merge; a word given before the merge holds for the tree the review judged". Step 11: the word
  names the publish, and is given after the review; a word given before the merge holds only once
  step 12's comparison passes.
- **Default**: handle now.
- **Outcome**: handle now, the head and step 11 both edited. The owner: "All defaults approved, proceed"

## P3. Step 7 says "The owner reads the lists", and nothing tells the session it happened

- **a. What it says.** The session gave the paths of the three package lists and asked the owner to
  read them. The owner answered "You can proceed". The transcript reviewer reported the step as
  open. The fix proposed: either the session checks the lists against
  `design@knowledge-architect@package-include-whitelist` and the owner's reading is optional, or
  the step names the confirmation it expects.
- **b. Whether it still applies.** Yes. `path@agent-config@skills/klarch-release/SKILL.md`:85, "The
  owner reads the lists." It entered with 0.1.0's release skill, before step 7 listed what each
  package holds; the review commit of 0.1.0, "[review] Six reviewers read the release: the packages
  ship only their whitelist, and 1.0 waits for the owner's word", added that list beside it.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Unclear text.
- **e. Its scope.** The agent-config location, `skill@klarch-release`. The head
  `design@knowledge-architect@package-include-whitelist` says "the release procedure runs it", not
  who reads the result, so the fix edits no head.
- **f. A better fix.** The first option, the session's own comparison, is the better of the two: step 7 names what each list must hold, and a list compared against that by the
  session is checked; the owner's reading adds a check only for a file the whitelist admits by
  mistake.
- **g. Its route.** A text edit of `skill@klarch-release`, under
  `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** Step 7: the session compares each list with what the step names and reports
  any other file; the owner may read the lists, and the release does not wait for it.
- **Default**: handle now, with the owner's reading optional.
- **Outcome**: handle now, the owner's reading optional. The owner: "Lets act on the retrospective findings right now. We'll run the intake procedure anyway, but IMO all of your suggestions are good. For P3, owner reading can be optional."

## P4. Step 1's "Rewrite its wording, not only the number" names no target wording

- **a. What it says.** The status sentence "The version is 0.5.0, published on crates.io." was
  correct with the number changed, and the session changed the number only. The fix proposed: name
  what the wording must say, or drop the sentence.
- **b. Whether it still applies.** Yes. `path@agent-config@skills/klarch-release/SKILL.md`:32-33,
  "the status line: the opening sentence of `## Status` in the root README.md. Rewrite its wording,
  not only the number." It entered at 0.1.0, when the status lines named the first release; the
  README sentence has read "The version is <version>, published on crates.io." from 0.2.0 to 0.6.0,
  per `git show <release commit>:README.md` at each of the release commits.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Unclear text: a rule with no target, kept from a wording that no longer exists.
- **e. Its scope.** `skill@klarch-release` alone.
- **f. A better fix.** Dropping the sentence loses the case it was written for: a status sentence
  carrying something besides the number that the release makes false, as "the first release" did at
  0.2.0. Naming that case is one clause.
- **g. Its route.** A text edit of `skill@klarch-release`.
- **h. The proposal.** "Change the number, and any other part of the sentence the release makes
  false."
- **Default**: handle now.
- **Outcome**: handle now. The owner: "Lets act on the retrospective findings right now. We'll run the intake procedure anyway, but IMO all of your suggestions are good. For P3, owner reading can be optional."

## P5. `issue@agent-config@klarch-release-step-12-compares-with-a-head-step-10-deleted`: the comparison held

- **a. What it says.** The session recorded the reviewed head and its tree before the merge, and
  after the pull found main's tree equal to it. It recorded the head only because the issue entry,
  read while checking for blockers, told it to. The fix proposed: close the entry by its second
  branch, step 10 recording `git rev-parse HEAD` before the merge and step 12 naming that value.
- **b. Whether it still applies.** Yes. `path@agent-config@skills/klarch-release/SKILL.md`:98-99,
  step 10, records nothing; step 12 at line 108 compares with "<the reviewed branch head>".
- **c. Whether it is already recorded.** It is the subject of that entry, whose closing condition
  reads: "it found the head and the comparison held, and the entry closes; or it did not, and step
  9 or 10 notes `git rev-parse HEAD` before the merge."
- **d. Its kind.** The first branch of the closing condition is met, but by a session that was led
  to the head by the entry itself: the next session without the entry would not be. So the second
  branch's text is the durable close.
- **e. Its scope.** `skill@klarch-release`, serving `design@knowledge-architect@publish-after-merge`.
- **f. A better fix.** The rival is to close on the first branch with no edit. It leaves the next
  release with the gap the entry describes, since the entry leaves with the close.
- **g. Its route.** A text edit of `skill@klarch-release`, and the entry deleted in that commit.
- **h. The proposal.** Step 10 records `git rev-parse HEAD` and its tree before the merge; step 12
  compares with that recorded value; the entry closes.
- **Default**: handle now.
- **Outcome**: handle now. The owner: "Lets act on the retrospective findings right now. We'll run the intake procedure anyway, but IMO all of your suggestions are good. For P3, owner reading can be optional."
