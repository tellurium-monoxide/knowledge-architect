# Tripwires — agent skills

Evidence that would reopen a decision of `path@agent-skills@docs/design.md`. A tripwire leaves this
file when it fires. What is outstanding is `path@agent-skills@docs/open-issues/`.

## Guarding `design@agent-skills@ledger-from-transcript`: a ruling lost or misstated at assembly `##ruling-lost-in-assembly`

The decision rests on the premise that assembly from the transcript, checked by the transcript
reviewer, records the owner's rulings as the owner made them.

**Fires when:** a ruling of the owner is found missing from, or misstated in, a committed plan
document, after the plan document's own reviews, where the misstatement is critical under
`design@agent-skills@transcript-reviewer-agent`: reversed, or changing what is built or a
load-bearing decision.
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

## Guarding `design@agent-skills@new-or-reshaped-head-needs-design`: a head is created or contradicted with no deliberation recorded `##head-created-without-deliberation`

The decision rests on the premise that the design skill's description and the decision-recording
skill's backstop reach a session before it writes a decision that creates a head or contradicts
one.

**Fires when:** a commit creates a design head, or contradicts a statement of one, and neither its
message nor a plan document carries the deliberation: no thread, no rival, and none of the owner's
words where the owner ruled. A relocation or rewording that adds or removes no decision does not
count.
**Response:** open a `defect` naming the head and the commit, and reopen
`design@agent-skills@new-or-reshaped-head-needs-design` on where its trigger is delivered, with the
primer line it rejected among the candidates.
**Re-entry:** the standing-state review before every merge: the commits and their messages are in
the branch's history.

## Guarding `design@agent-skills@new-or-reshaped-head-needs-design`: an addition recorded directly outgrows its head's title `##title-stops-stating-scope`

The decision rests on the premise that a session reads a head's title strictly, and sends an
addition outside it to the design skill rather than recording it directly.

**Fires when:** a review finds a design head whose title no longer states a decision added to its
body by the direct route: an addition whose commit carries no deliberation.
**Response:** open a `defect` naming the head and the commit, and reopen
`design@agent-skills@new-or-reshaped-head-needs-design` on its third case.
**Re-entry:** the standing-state review before every merge: the head, its title and the commits
that changed it are in the branch's history.

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

## Guarding `design@agent-skills@spec-work-procedure`: a gap an audit would have found, in a spec's work that skipped it as the rule allows `##skipped-spec-audit-would-have-found-a-gap`

The decision rests on the premise that a spec implemented by the session that wrote it, on a main
branch with no commit since the spec other than its own, has nothing for an audit to find.

**Fires when:** a review finds, in the work of a spec that skipped its design audit as the rule
allows, a gap an audit would have listed: a shape the tree refutes, a name the code does not have,
or a standing entry the work bears on that the grounding did not meet.
**Response:** open a `defect` naming the spec and the change, and reopen
`design@agent-skills@spec-work-procedure` on the condition for the audit.
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
missing form; or a retrospective's answer to its question on references written without backticks reports a pointer
written as bare plain text to clear a finding.
**Response:** open a `defect` for the instance. At the second instance, reopen
`design@agent-skills@plain-text-is-no-repair`, with a mechanical check for plain-text
pointers among the candidates.
**Re-entry:** the standing-state review, which reads every tripwire before a merge, and the
retrospective's question on references written without backticks, which reads the session.
