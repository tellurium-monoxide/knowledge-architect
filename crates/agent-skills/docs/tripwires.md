# Tripwires — agent skills

Evidence that would reopen a decision of `path@agent-skills@docs/design.md`. A tripwire leaves this
file when it fires. What is outstanding is `path@agent-skills@docs/open-issues/`.

## Guarding `design@agent-skills@ledger-from-transcript`: a ruling lost or misstated at assembly `##ruling-lost-in-assembly`

The decision rests on the premise that assembly from the transcript, checked by the transcript
reviewer, records the owner's rulings as the owner made them.

**Fires when:** a ruling of the owner is found missing from, or misstated in, a committed plan
document, after the plan document's own reviews.
**Response:** open a `defect` naming the ruling and the document, and reopen
`design@agent-skills@ledger-from-transcript`, with the rejected alternative "Writing the
discussion's ledger to a file during the discussion" among the candidates.
**Re-entry:** the retrospective of the session that assembled the plan document: it reads that
session's own record against the committed document.
**Evidence when written:** in the commit that added the structured-plans milestone document, its
transcript reviewers found three rulings recorded wider than the owner made them. The author wrote
each of the three, and the extraction none, and each was repaired before that commit. That is
before the commit, so it did not fire; it is why the guard exists.

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
design audit of a milestone step meets the standing entries a piece of work bears on before the
work. A spec has no audit, and an entry can be added after the grounding's search.

**Fires when:** across sessions, 2 standing-state reviews report a standing entry the work bears
on, which no search before the work returned.
**Response:** open a `design` issue naming the entries and the works, and reopen
`design@agent-skills@standing-entries-searched-before-the-work` on its occasions, with a search
when convergence is proposed, before the premortem, among the candidates.
**Re-entry:** the design discussion of
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, on the
owner's word: the evidence arises in the sessions of the projects that use the workflow.

## Guarding `design@agent-skills@standing-entries-searched-before-the-work`: a deferred trigger met by undesigned work and found only at the review `##deferred-trigger-met-by-undesigned-work`

The decision sends no search before work that is neither designed nor planned. It rests on the
premise that a deferred trigger met by such work costs little when the review is the first to read
it.

**Fires when:** a second instance is recorded of a deferred trigger met by work that is neither
designed nor planned, and found only at the standing-state review. The first was in thaum: a move
of the pinned checker met the trigger of a deferred issue that named that move.
**Response:** open a `design` issue naming both instances, and propose a search before such work,
with its host.
**Re-entry:** the design discussion of `issue@agent-skills@a-skill-for-bounded-problems`, the
skill that would host a search before undesigned work.

