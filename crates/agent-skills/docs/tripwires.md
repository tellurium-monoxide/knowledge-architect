# Tripwires — agent skills

Evidence that would reopen a decision of `path@agent-skills@docs/design.md`. A tripwire leaves this
file when it fires. What is outstanding is `path@agent-skills@docs/open-issues/`.

## Guarding `design@agent-skills@ledger-from-transcript`: a ruling lost or misstated at assembly `##ruling-lost-in-assembly`

The decision rests on the premise that assembly from the transcript, checked by the transcript
reviewer, records the owner's rulings as the owner made them.

**Fires when:** a ruling of the owner is found missing from, or misstated in, a committed plan
document, after the plan document's own reviews, where the misstatement is critical under
`design@agent-skills@misstated-ruling-rated-by-what-it-changes`: reversed, or changing what is
built or a load-bearing decision.
**Response:** open a `defect` naming the ruling and the document, and reopen
`design@agent-skills@ledger-from-transcript`, with the rejected alternative "Writing the
discussion's ledger to a file during the discussion" among the candidates.
**Re-entry:** the retrospective of the session that assembled the plan document: it reads that
session's own record against the committed document.
**Evidence when written:** in the commit that added the structured-plans milestone document, its
transcript reviewers found three rulings recorded wider than the owner made them. The author wrote
each of the three, and the extraction none, and each was repaired before that commit. That is
before the commit, so it did not fire; it is why the guard exists. Two later instances, in the
discussion that produced the milestone load-bearing-records and in an earlier spec, recorded a
hedged word of the owner firmer than given, on details that were not load-bearing; the owner ruled
that such a slip is "not a critical defect", still repaired, and under the firing condition above
neither fires.

## Guarding `design@agent-skills@expectation-set-bounds-scope` `##expectation-set-closes-a-contradiction`

**Fires when:** a finding that two installed instructions leave no move satisfying both is closed
by citing an expectation set.
**Response:** open a `defect` naming the finding and the set cited, and reopen
`design@agent-skills@expectation-set-bounds-scope` if its boundary sentence admits the reading.
**Re-entry:** the standing-state review of the change that closed the finding.

## Guarding `design@agent-skills@new-or-reshaped-head-needs-design`: a head is created, contradicted or given a member its argument does not cover with no deliberation recorded `##head-created-without-deliberation`

The decision rests on the premise that the design skill's description and the backstop of the
primer's section on design heads, read again at the head of the decision-recording skill loaded
before any write into a design home per `design@agent-skills@design-home-write-loads-recording`,
reach a session before it writes a decision that creates a head, contradicts one, or adds a member
its argument does not cover.

**Fires when:** a commit creates a design head, contradicts a statement of one, or adds to it a
member its argument does not cover, and neither its message nor a plan document carries the
deliberation: no thread, no rival, and none of the owner's words where the owner ruled. A relocation
or rewording that adds or removes no decision does not count.
**Response:** open a `defect` naming the head and the commit, and reopen
`design@agent-skills@new-or-reshaped-head-needs-design` on where its trigger is delivered; for a
member beyond the argument, with the rival that sends every decision earning durable text through
the design skill among the candidates.
**Re-entry:** the standing-state review before every merge: the commits and their messages are in
the branch's history.

## Guarding `design@agent-skills@in-change-path`: a ruling lost or misstated in an in-change commit message `##ruling-lost-in-change`

The decision rests on the premise that a commit message records the owner's rulings as the owner
made them, as a plan document does, with no review of a plan document before the work.

**Fires when:** a ruling of the owner is found missing from, or misstated in, the commit message
that carries the deliberation of a discussion on the in-change path, after the branch's transcript
review.
**Response:** open a `defect` naming the ruling and the commit, and reopen
`design@agent-skills@in-change-path` on its entry condition: whether a discussion with more than
one approved thread takes the full path.
**Re-entry:** the retrospective of each session that wrote such a commit message: it reads the
session's own record against the message.

## Guarding `design@agent-skills@standing-entries-searched-before-the-work`: an entry the work bears on that no search before the work returned `##search-missed-before-the-work`

The decision rests on the premise that a search at the grounding of a design discussion and at the
design audit of a milestone slice, or of a spec where its audit runs, meets the standing entries a
piece of work bears on before the work. A spec implemented by the session that wrote it, on an
unmoved main branch, has no audit, and an entry can be added after the grounding's search.

**Fires when:** across sessions, 2 standing-state reviews report a standing entry the work bears
on, which no search before the work returned.
**Response:** open a `design` issue naming the entries and the works, and reopen
`design@agent-skills@standing-entries-searched-before-the-work` on its occasions, with a search
when convergence is proposed, before the premortem, among the candidates.
**Re-entry:** the design discussion of
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, on the
owner's word: the evidence arises in the sessions of the projects that use the workflow.

## Guarding `design@agent-skills@standing-entries-searched-before-the-work`: a deferred trigger met by work that bypasses the design skill and found only at the review `##deferred-trigger-met-by-undesigned-work`

The decision sends no search before work that does not go through the design skill; bounded work
that does is searched at the grounding, per `design@agent-skills@bounded-path-in-design`. It rests
on the premise that a deferred trigger met by work that bypasses the skill costs little when the
review is the first to read it.

**Fires when:** a second instance is recorded of a deferred trigger met by work that did not go
through the design skill, and found only at the standing-state review. The first was in thaum: a move
of the pinned checker met the trigger of a deferred issue that named that move.
**Response:** open a `design` issue naming both instances, and propose a search before such work,
with its host.
**Re-entry:** the standing-state review before every merge, which reads every deferred trigger
against the change.

## Guarding `skill@knowledge-architect-planning@working-a-slice`: a critical gap an audit would have found, in a spec's work that skipped it as the rule allows `##skipped-spec-audit-would-have-found-a-gap`

The instruction that runs a spec's design audit only when a fresh session starts its work or the
main branch has moved, argued in the `%%` line beside it, rests on the premise that a spec
implemented by the session that wrote it, on a main branch with no commit since the spec other
than its own, has nothing load-bearing for an audit to find. The tripwire guards against a critical failure of the workflow, not a small mistake that is
cheap to repair.

**Fires when:** a review finds, in the work of a spec that skipped its design audit as the rule
allows, a critical gap an audit would have listed: a shape the tree refutes, a name the code does
not have, or a standing entry the work bears on that the grounding did not meet. A gap is critical
when the work's result, or a decision it rests on, is wrong without its repair.
**Response:** open a `defect` naming the spec and the change, and reopen the condition for the
audit in `skill@knowledge-architect-planning@working-a-slice`, with a design audit for the work of
every spec, as for every slice, the first candidate.
**Re-entry:** the standing-state review before every merge: the spec, its commit and the main
branch's history since are in the branch's range and its base.

## Guarding `design@agent-skills@capabilities-not-structure`: a rejection that cites it is overruled `##structure-rejection-overruled`

The decision rests on the premise that the principle against imposed structure is cited to remove
structure the work does not need, and not to refuse structure it does.

**Fires when:** a review or a design discussion rejects an instruction by citing
`design@agent-skills@capabilities-not-structure` or
`goal@agent-skills@installed-text-leaves-room-to-judge`, and the owner overrules that rejection.
**Response:** open a `design` issue naming the instruction and the owner's reason, and reopen
`design@agent-skills@capabilities-not-structure` on where structure is needed.
**Re-entry:** the retrospective of the session where the owner overruled it, which reads that
session's record.

## Guarding `design@agent-skills@bounded-path-in-design`: a decision taken on the bounded path earns a head `##a-shortcut-decision-earns-a-head`

The decision rests on the premise that a session classifies work as bounded only when every
decision it makes fails the entry tests, and does not use the path to skip a discussion.

**Fires when:** a decision-record review finds, in a commit made on the bounded path, a decision
that passes an entry test of `design@agent-skills@a-head-is-owed-by-an-entry-test`, at the second
instance.
**Response:** reopen `design@agent-skills@bounded-path-in-design` on its three conditions.
**Re-entry:** the decision-record review before every merge: the commit carries the proposal and
the owner's words.

## Guarding `design@agent-skills@a-head-is-owed-by-an-entry-test`: the owner overrules a review's verdict on a head `##a-head-verdict-is-overruled`

The decision rests on the premise that the entry tests decide whether a head is owed in a way the
owner agrees with, and that whether a rival is plausible is not left to case-by-case taste.

**Fires when:** the owner overrules a review's verdict on whether a decision earns a head, the
second time.
**Response:** reopen `design@agent-skills@a-head-is-owed-by-an-entry-test` on the wording of the
tests.
**Re-entry:** the decision-record review, whose verdicts the owner rules on.

## Guarding `design@agent-skills@local-intent-binds`: an escalation over a comment is ruled unneeded `##a-comment-escalation-is-overruled`

The decision rests on the premise that a reason recorded at the code is weighed as a presumption,
and does not make every change to commented code a question for the owner.

**Fires when:** the owner rules, the second time, that a session's escalation of a change because a
comment states a reason was unneeded, the comment holding no decision worth the question.
**Response:** reopen `design@agent-skills@local-intent-binds` on the rung of comments.
**Re-entry:** the retrospective of the session where the owner ruled it, which reads that session's
record; this is behaviour in sessions, with the limit
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` names.

## Guarding `design@agent-skills@plain-text-is-no-repair`: a checked pointer turned into plain text `##plain-text-pointer-found`

**Fires when:** a review finds a diff that turns a checked reference or a backticked path into
plain text naming the same target, with no reference beside it to an issue entry that records a
missing form, other than a deleted skill's or agent's name in a released changelog section, which
the decision admits; or a retrospective's answer to its question on references written without backticks reports a pointer
written as bare plain text to clear a finding.
**Response:** open a `defect` for the instance. At the second instance, reopen
`design@agent-skills@plain-text-is-no-repair`, with a mechanical check for plain-text
pointers among the candidates.
**Re-entry:** the standing-state review, which reads every tripwire before a merge, and the
retrospective's question on references written without backticks, which reads the session.

## Guarding `design@agent-skills@plan-read-against-the-record`: the design-conformance reviewer's findings are mostly needless `##design-conformance-findings-needless`

The decision rests on the premise that the reviewer reports departures from the record, and not the
widenings a plan document makes on purpose, so that its findings are worth the owner's reading. The
owner ruled on it as T1 at the premortem of its discussion.

**Fires when:** at two moments in a row at which the planning skill sends a plan document's
reviewers, a re-review after a revision included, more than half of the design-conformance
reviewer's findings end judged to need nothing, by the dispatching session or by the owner. The
count, two moments, and the proportion, half, are defaults the owner may reset.
**Response:** open a `design` issue naming the two reviews and their findings, and reopen
`design@agent-skills@plan-read-against-the-record` on the reviewer's scope.
**Re-entry:** the standing-state review before every merge, which reads the outcome each review
commit records for each finding; and the retrospective of the session where the second of those
reviews ran, which reads that session's record.

## Guarding `design@agent-skills@extension-follows-the-ground`: a member recorded directly that the argument does not cover `##member-beyond-the-argument`

T1 of the premortem of the discussion that made the decision. The decision rests on the premise
that a session judges whether a head's argument covers a new member as the owner would.

**Fires when:** the owner, or a review, names one member recorded directly, as covered by its head's
argument, that the argument does not cover.
**Response:** reopen `design@agent-skills@extension-follows-the-ground`, with the rival that routes
by the scope of the owner's approval among the candidates.
**Re-entry:** the standing-state review before every merge, and the decision-record and
design-conformance reviews, which read the heads a change writes.

## Guarding `design@agent-skills@head-ground-is-the-argument`: a quotation that was the owner's own intent is removed `##owner-intent-stripped`

T2 of the premortem of the discussion that made the decision. The decision rests on the premise
that the repair of an approval cited as a ground does not remove the owner's own words with it.

**Fires when:** a commit removes or rewords as an argument a quotation of the owner in a design head,
and the transcript or the plan document shows that the decision, or that part, came from the owner.
**Response:** restore the quotation as the ground, and reopen
`design@agent-skills@head-ground-is-the-argument` on how a ground is told apart, with the rival
that cites every ruling of the owner among the candidates.
**Re-entry:** the standing-state review before every merge, and the transcript review of the change.

## Guarding `design@agent-skills@title-states-the-rule`: a rule title wider than its argument `##rule-title-wider-than-its-argument`

T3 of the premortem of the discussion that made the decision. The decision rests on the premise
that a title moved to the rule stays within what its argument argues.

**Fires when:** a review finds a title wider than its argument in a head written or rewritten after
the decision.
**Response:** repair the title, and reopen `design@agent-skills@title-states-the-rule`.
**Re-entry:** the decision-record review of every change that writes a head.

## Guarding `design@agent-skills@one-decision-per-head`: a split that separates a rule from its own exception `##split-rule-from-exception`

T4 of the premortem of the discussion that made the decision. The decision rests on the premise
that the nearest-rival test keeps a rule's exception with the rule.

**Fires when:** a commit merges back two heads split under the decision, or a review finds an
exception split from its rule.
**Response:** reopen `design@agent-skills@one-decision-per-head`, with the reversal test among the
candidates.
**Re-entry:** the decision-record review of every change that writes a head.

## Guarding `design@agent-skills@restatement-size-test`: a directive missed because its restatement became a pointer `##pointer-not-followed`

T5 of the premortem of the discussion that made the decision. The decision rests on the premise
that a session follows a pointer to a directive's home at the moment it needs the directive.

**Fires when:** a review or a retrospective finds a directive broken in a session where its
restatement had been replaced by a pointer under the decision.
**Response:** reopen `design@agent-skills@restatement-size-test`, with the rule it replaced, a
directive restated wherever it is delivered, among the candidates.
**Re-entry:** the retrospective of each session, and the review of every change.

## Guarding `design@agent-skills@audit-method`: the owner overrules the defaults of an audit's owner list `##audit-defaults-overruled`

T6 of the premortem of the discussion that made the decision. The decision rests on the premise
that the session's reading that produces the owner list's defaults, the sort of an axis with no
saved workflow or the verification of a run's report, with the calibration where an axis has one,
gives the owner defaults the owner mostly accepts, so the owner list costs a ruling per kind rather
than a judgement per item.

**Fires when:** in one audit, the owner rules against the default on more than a quarter of the
items of the owner list; the bound is the owner's to reset.
**Response:** reopen `design@agent-skills@audit-method`.
**Re-entry:** the session that receives the owner's answers to an owner list counts them; the
standing-state review of the audit's branch reads it again.

## Guarding `design@agent-skills@audit-triggers`: a pin moves across a version whose changelog cites an axis, and the axis is not run `##axis-not-run-at-a-pin-move`

T7 of the premortem of the discussion that made the decision. The decision rests on the premise
that the move of a pin reads the Migration entries of the versions crossed and runs the axis they
cite. It watches agent behaviour in a consumer project, which this project cannot see, per
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`.

**Fires when:** a retrospective finds a pin moved across such a version with the axis not run.
**Response:** reopen `design@agent-skills@audit-triggers`.
**Re-entry:** the retrospective of a session that moved a pin.

## Guarding `design@agent-skills@design-record-axis`: an audit's edit leaves a tripwire or an issue on the edited head false `##audit-edit-leaves-an-entry-stale`

T8 of the premortem of the discussion that made the decision. The decision rests on the premise
that reading each head's tripwires and issues before judging it keeps them true of the head the
audit edits.

**Fires when:** a review of an audit's branch finds a tripwire guarding, or an issue naming, a head
the branch changed, left false.
**Response:** reopen `design@agent-skills@design-record-axis`.
**Re-entry:** the standing-state review of every audit's branch.

## Guarding `design@agent-skills@workflow-axis-lenses`: a duplicated lens adds nothing its first agent did not find `##duplicate-auditors-add-nothing`

T1 of the premortem of the discussion that made the decision. The decision rests on the premise
that a second agent on one lens finds what the others of that lens missed, so its cost buys
coverage.

**Fires when:** in one pass, for every lens dispatched more than once, one of its agents alone
reported every confirmed finding of that lens. Two L2 agents count as one lens dispatched twice
only when they share a group of activities.
**Response:** reopen `design@agent-skills@workflow-axis-lenses` on the duplicate dispatch.
**Re-entry:** the session's verification of each pass's report counts it.

## Guarding `design@agent-skills@workflow-axis-finding-standard`: most drafts of a pass fail the reading `##audit-drafts-mostly-noise`

T2 of the premortem of the discussion that made the decision. The decision rests on the premise
that the finding standard keeps auditors from reporting predicted gaps as provable ones, and L8's
auditors from reporting two standards that differ in wording and give one verdict, so the reading
of a pass is not spent rejecting drafts.

**Fires when:** in one pass, more than half of the drafts fail the reading. A draft the verifier
refutes or the session rejects fails it; a draft marked kept or known is not counted.
**Response:** reopen `design@agent-skills@workflow-axis-finding-standard` and the standard of
`agent@knowledge-architect-workflow-auditor`.
**Re-entry:** the session's verification of each pass's report.

## Guarding `design@agent-skills@workflow-axis-corpus` and `design@agent-skills@workflow-axis-lenses`: an auditor does not read the whole corpus `##auditor-skips-part-of-the-corpus`

T3 of the premortem of the discussion that made the decision. The decisions rest on the premise
that one agent reads the whole corpus, every file whole, so that a contradiction between two files
is in one agent's reading. The corpus grows with every skill and every repair.

**Fires when:** an auditor's return says it did not read every file of the corpus whole.
**Response:** reopen how the corpus is shared out among agents, in
`design@agent-skills@workflow-axis-lenses`, with the corpus shared out among agents of one task
among the candidates.
**Re-entry:** the session's verification of each pass's report, which reads each auditor's saved
return.

## Guarding `design@agent-skills@verifier-in-the-run`: a verifier refutes a true finding `##verifier-refutes-a-true-finding`

T5 of the premortem of the discussion that made the decision. The decision rests on the premise
that the adversarial verifier refutes only drafts a reading refutes, since the session reads only a
sample of the refuted drafts, chosen by its judgement.

**Fires when:** in any pass, the session's sample of refuted drafts holds one that a reading
confirms.
**Response:** reopen `design@agent-skills@verifier-in-the-run`.
**Re-entry:** the session's verification of every report.

## Guarding `design@agent-skills@verifier-in-the-run`: the verifiers wave findings through `##verifiers-wave-findings-through`

T6 of the premortem of the discussion that made the decision. The decision rests on the premise
that the verifiers refute what a reading refutes, so that the session's verification after the run
is not a second sort.

**Fires when:** in one pass, the session rejects more than a quarter of a report's confirmed
findings; the bound is the owner's to reset.
**Response:** reopen `design@agent-skills@verifier-in-the-run`.
**Re-entry:** the session's verification of every report.

## Guarding `design@agent-skills@repairs-read-against-the-record`: a repair reverses a head or brings back a rejected alternative its check did not name `##record-check-misses-a-head`

T8 of the premortem of the discussion that made the decision. The decision rests on the premise
that the check of a repair proposal finds the heads its edits would reverse, although its search
for the heads that describe a behaviour is a judgement.

**Fires when:** the review of an audit's branch finds a repair that contradicts a design head, or
brings back a rejected alternative, which the check of its proposal did not name.
**Response:** reopen `design@agent-skills@repairs-read-against-the-record` on where the record
check runs.
**Re-entry:** the review of each audit's branch.

## Guarding `design@agent-skills@root-cause-repair`: an audit's passes do not converge `##audit-passes-do-not-converge`

AC4 of the discussion that made the decision, proposed by the owner as an acceptance criterion and
kept as a tripwire, on the owner's word, to watch future audits. The decision rests on the premise
that repairing each cause rather than each finding makes an audit push the record toward
improvement, so that what a pass leaves falls from one pass to the next.

**Fires when:** from one pass to the next of an audit, the violations left on old text do not
fall, or the severe ones among them, those whose outcome is the owner list, do not fall while they
are above zero. Old text is text the previous pass also read, whose words are unchanged since the
commit it audited; a first pass is not counted.
**Response:** reopen `design@agent-skills@root-cause-repair`, with `design@agent-skills@workflow-axis-lenses`
and `design@agent-skills@workflow-axis-finding-standard` among what is read again.
**Re-entry:** the session that writes each pass's commit message, which gives the counts on old
text; the standing-state review of the audit's branch.
**Evidence when written:** in the first agentic-workflow audit of this repository, as an
acceptance criterion, it fired at passes 1 to 3 and 5 on the plain counts, and at pass 6 on the
old-text counts: 21 violations left against pass 5's 16, the severe ones falling from 8 to 7. The
owner stopped the audit after pass 6's application.
