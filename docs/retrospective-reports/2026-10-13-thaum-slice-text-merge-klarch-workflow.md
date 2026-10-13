# 2026-10-13-thaum-slice-text-merge-klarch-workflow

- **Received file**: 2026-10-13-thaum-slice-text-merge-klarch-workflow.md, the workflow file of the
  retrospective of a session in thaum that ran one slice of a milestone through its merge, then a
  second branch that repaired a defect of the project's checker extension.
- **Version used**: the published checker, pinned at 0.6.0.
- **Analysed against**: main at `v0.6.0-75-g5fca230`.
- **Standing entries that bear on it**: from the one search over the 83 issues and 58 tripwires:
  - `a-design-issue-s-re-entry-point-is-read-by-no-review`, the gap W6 widens;
  - `issue@agent-skills@the-retrospective-counts-no-review-cost`, which W4's cost bears on;
  - `issue@core@finding-texts-are-not-audited-for-a-needless-cause`, near C1;
  - the tripwires judged together in the last section.

## W1. An answer of the owner that names a label never put, or that two items could match

- **a. What it says.** The report to the owner stated D7 with its content and its default, and
  listed D6 as still open, with no content and no default. The owner answered "Agreed on D7 and D8,
  proceed". No D8 had been put. The session read D8 as D6, marked D6 ruled in the milestone
  document, kept the doubt to itself, and committed. The transcript reviewer reported it as critical
  before the merge; asked, the owner confirmed D6. On the second branch, the session met an answer
  that could cover two proposals and asked back. The fix proposed: in `primer@labelled-items`, an
  answer that names a label not put, or that two items could match, rules on nothing and is asked
  back; a report that asks for a ruling states the item's content and its default, not its label
  alone.
- **b. Whether it still applies.** The labelled-items section is unchanged on this point:
  `path@agent-skills@content/PRIMER.md`:64-75 numbers labels and says a label "never names what
  decided"; nothing covers an answer naming no live label. Since 0.6.0, `primer@question-authority`
  sends to the owner "a fact only the owner holds, such as an intent", at lines 30-31, and which
  item an answer meant is such a fact. Row 1 of `primer@met-outside-the-task` puts an item to the
  owner "with a default", so an item listed with no content and no default was not put.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** A lapse, at 0.6.0: the session held its doubt and did not ask, and asked back in
  the same kind of case on its second branch, under the same text. The text on main routes the
  question by its authority.
- **e. Its scope.** None beyond the session, on main.
- **f. A better fix.** The rival is the sentence proposed, a rule for one interaction, which the
  repair by cause of `skill@knowledge-architect-agent-configuration@admission-standard` deletes where
  judgement already decides. Here the primer's routing decides: the meaning of the owner's answer is
  the owner's.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change: on main, the primer sends what the owner meant to the owner, and the
  session asked back under the same text on its second branch.
- **Outcome**: approved.

## W2. A record states a question as put to the owner before it is put

- **a. What it says.** Two reviewers flagged the D6 reading. Two review records gave it the outcome
  "put to the owner in the report of this turn". The turn ran on through the harvest and the review
  before the merge, and neither promise was kept until the transcript reviewer found it. The fix
  proposed: an item awaiting the owner stops the turn at the next commit that would record its
  outcome, and a record never states a question as asked before it is asked.
- **b. Whether it still applies.** The rule the session broke stands, reworded since 0.6.0 on
  another point: `path@agent-skills@content/skills/review/SKILL.md`:153-154, "**An item whose outcome
  `primer@question-authority` gives the owner is put to the owner** before its outcome is recorded".
  It was "put to the owner before its outcome is recorded" at 0.6.0 too.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** A lapse: the records recorded an outcome before the act the rule orders first.
  The file's second proposal restates that rule. Its first adds a stop at a commit, a structure on
  the turn.
- **e. Its scope.** None beyond the session.
- **f. A better fix.** No change: the rule is clear on the order, and the transcript reviewer found
  the breach.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of a clear rule.
- **Outcome**: approved.

## W3. Whether a commit message that a later commit of the same branch reverses carries a mistake

- **a. What it says.** The audit's commit message stated a reading's planned register status. The
  harvest, three commits later, filed it under another status, for a reason found while writing the
  entry. The rules reviewer reported the audit's message as wrong, and the session reworded it by a
  rebuild of the branch. The fix proposed: such a message is no mistake when the reversing commit's
  message names what it reverses.
- **b. Whether it still applies.** Yes, unchanged since 0.6.0.
  `path@agent-skills@content/skills/review/SKILL.md`:181-185: "**A commit message carrying a mistake
  is repaired by amending** while it is the newest commit, and by a history edit of the branch after
  that. … A correction written into a later commit's message is not a repair: the mistaken message
  still reads as it did." It does not say whether a plan true when written is a mistake.
- **c. Whether it is already recorded.** No. No rejected alternative matches.
- **d. Its kind.** Unclear: two readings, with a cost that differs by a history edit of the branch.
- **e. Its scope.** The agent-skills Component, `skill@knowledge-architect-review@what-review-leaves`.
  `design@core@a-commit-message-is-a-document` judges a message against its own commit's tree, which
  is the reading under which a plan true of its commit is no mistake. Whether a head on review
  repairs states the point is checked at the edit, under the record check.
- **f. A better fix.** Rivals: the reading proposed; the other reading, under which every plan a
  later commit changes is reworded, which makes history describe a plan the branch never had at that
  commit; or no change. The first: a message is a statement about its commit, and a later message
  that names the change keeps both readable. Count: 0 added; the rewording narrows the word
  "mistake".
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent", after the record check.
- **h. The proposal.** In `skill@knowledge-architect-review@what-review-leaves`: a message is judged
  against its own commit; a plan it states that a later commit of the branch changes is no mistake
  when the later commit's message names the change.
- **Default**: handle now.
- **Outcome**: approved.

## W4. The rule that a touched head is brought to the rules widens each repair

Analysed with W7 of 2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow, the
cost of the last reviews. This file claims no defect. Its optional fix, exempting a head touched
only by a split's re-pointing, already stands in `primer@design-heads`: "Re-pointing a reference in
a head does not touch it."

- **Default**: as that section's outcome.
- **Outcome**: as the section it points to rules. The owner: "For the pending ones, I gave my word in the section holding the detailed analysis, you can point to them."

## W5. The standing-entry searcher's order can be read two ways

- **a. What it says.** Four of the five searchers sorted by the anchor field, then the id field. One
  sorted the whole reference string with `LC_ALL=C sort`. The orders differ where one anchor is a
  prefix of another, as `a` and `a-b`, since `-` sorts before `@`. Six rows fell between two groups
  and were read by no searcher; the session found them by taking the union of the groups' files. The
  fixes proposed: the definition gives the exact command and says the sort is by field; the dispatch
  checks that the union of the groups covers the listing.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/agents/standing-entry-searcher.md`:48-52:
  "the issue rows sorted by anchor, then by id; then the tripwire rows sorted by anchor, then by id,
  each sort in byte order (`LC_ALL=C sort`)". It gives no keys, and the command it names sorts whole
  lines. Measured on this repository's listings today, 83 issue rows and 58 tripwire rows as
  `<kind>@<anchor>@<id>`: `LC_ALL=C sort -t@ -k2,2 -k3,3` and `LC_ALL=C sort` give the same order,
  since no anchor here is another anchor followed by a hyphen. In this intake, the three searchers
  all sorted by field.
- **c. Whether it is already recorded.** No.
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` concerns the seeds, not
  the order.
- **d. Its kind.** Unclear: the text names a command that does not produce the order it states.
- **e. Its scope.** The agent-skills Component, the searcher's definition. It reverses no head:
  `design@agent-skills@standing-entry-search-groups` fixes the order to be one, not its command.
- **f. A better fix.** Rivals: the exact command, which removes the second reading at its cause; the
  coverage check, which catches a divergence after the search and adds a step to every dispatch; or
  the listing printed by the checker in the fixed order, with positions, a larger change. The exact
  command: with the cause removed, the coverage step guards a case that no longer arises. Count: 0
  added, the instruction made exact.
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent".
- **h. The proposal.** The definition names the sort: `LC_ALL=C sort -t@ -k2,2 -k3,3` over the
  references, by the anchor field, then the id field, not by the whole line.
- **Default**: handle now.
- **Outcome**: approved.

## W6. A re-entry clause on an issue of a kind other than `deferred` or `design` is read by no step

- **a. What it says.** An issue of kind `question` named a re-entry, "the next change to" two files.
  The branch changed both. A searcher returned the entry with a doubt, the session did not judge it,
  and the standing-state reviewer read only `deferred` triggers. The transcript reviewer found it,
  and the owner re-parked the entry. The fixes proposed: the issue-tracking skill says a re-entry
  belongs only on `deferred`, as its trigger, and on `design`; or the standing-state reviewer reads
  every re-entry clause of every entry.
- **b. Whether it still applies.** Yes.
  `path@agent-skills@content/agents/standing-state-reviewer.md`:83-85 reads the triggers of
  `deferred` issues, listed with `{{command}} issues --kind deferred`. The kinds table of
  `skill@knowledge-architect-issue-tracking@issue-entry` gives a re-entry point to `design` only.
- **c. Whether it is already recorded.** In part:
  `a-design-issue-s-re-entry-point-is-read-by-no-review` asks whether the reviewer
  reads the re-entry point of every `design` issue. W6 is the same gap for any kind that carries a
  re-entry clause, and the first real instance: the entry's own case was a trial of the search
  agent.
- **d. Its kind.** Missing, already an open question, now with a real instance and a wider member
  set.
- **e. Its scope.** The agent-skills Component.
- **f. A better fix.** The open issue already weighs the two shapes for `design`; widening it keeps
  one question rather than two.
- **g. Its route.** Nothing now; the issue's own route.
- **h. The proposal.** The open issue gains this instance, and its question widens to every entry
  that carries a re-entry clause, whatever its kind.
- **Default**: no new issue; the open issue named in c gains the instance and the wider question.
- **Outcome**: approved.

## C1, C2. Two finding texts of the project's checker extension

- **a. What it says.** C1: the stale-index finding's hint, "regenerate it and read the diff: it is
  the work list a release bump reads", fired 8 times after ordinary edits; the fix proposed names the
  command. C2: the hint "put the rule's text behind the marker, or point at the decision that
  carries it and drop the number" was met four times by a third repair, describing the rule in words;
  the fix proposed closes that path in the hint.
- **b. Whether it still applies.** Neither text is this checker's. `git grep` for "release bump
  reads" and for "behind the marker" over this repository finds nothing at 0.6.0 or on main; both
  left this repository when the rules half became the project's own extension. The core passes an
  extension's action through, `path@core@src/check/generated.rs`:35, and its own action for a file
  register's index is "run `{} index`", line 47.
- **c. Whether it is already recorded.** `issue@core@finding-texts-are-not-audited-for-a-needless-cause`
  covers the core's own findings, not an extension's.
- **d. Its kind.** Out of scope: both are the project's extension's text, fixed in the project.
- **e.–g.** None here.
- **h. The proposal.** No change.
- **Default**: no change, the texts are the adopting project's extension's.
- **Outcome**: approved.

## The standing answers, and the tripwires the file does not meet

- The standing answer on `primer@met-outside-the-task` row 1 is W1 and W2.
- The standing answer on "never by moving the pointer into plain text" is C2's repairs. Those
  described a rule's content in words in place of the extension's rule marker, which is not this
  checker's reference syntax; `design@agent-skills@plain-text-is-no-repair` scopes its rule to the
  checker's syntax and admits a description in words. So
  `tripwire@agent-skills@plain-text-pointer-found` does not fire, and the file's own answer on
  references written without backticks is "No".
- The standing answer on the project's development skill concerns the project's own text.
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: W6's entry is a `question`, the
  second branch went through the design skill, and the transcript reviewer found it, not the
  standing-state review.
- `tripwire@agent-skills@search-missed-before-the-work`: the searcher returned W6's entry, and the
  six rows of W5 were read before the work.
- `tripwire@agent-skills@ruling-lost-in-assembly`: D6 was marked ruled in a committed plan document,
  and the owner then confirmed it, so nothing built or load-bearing changed; the clause asks for a
  critical misstatement, rated per `design@agent-skills@misstated-ruling-rated-by-what-it-changes`.
- `tripwire@agent-skills@ruling-lost-in-change`: the second branch asked back on its unclear answer,
  and the file reports no misstated message.
- `tripwire@agent-skills@record-answered-questions-reach-the-owner`: the session ran 0.6.0, before
  the decision's landing, and names no question the record answered.
- `tripwire@agent-skills@split-rule-from-exception`: W4 reports splits and a bundle, no merge back
  and no exception separated from its rule.
- `tripwire@core@checker-source-literals-are-data-whole-source-scope`: C2's repairs were in the
  project's documents, not in this tool's source.

- **Default**: no change for the standing answers; none of the tripwires fired.
- **Outcome**: approved.
