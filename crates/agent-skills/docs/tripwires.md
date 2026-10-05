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

## Guarding `design@agent-skills@new-or-reshaped-head-needs-design`: the trigger sends the owner decisions it does not cover `##decision-trigger-fires-too-often`

The decision rests on the premise that a session judges correctly which decisions create,
contradict or reshape a head, so that only those reach the owner as a design discussion.

**Fires when:** in two sessions, found by a retrospective or a transcript review, the owner says a
design discussion was not needed for a decision that created, contradicted and reshaped no head.
**Response:** open a `defect` naming both sessions and the decisions, and reopen
`design@agent-skills@new-or-reshaped-head-needs-design` on the wording of its cases and of the
design skill's description.
**Re-entry:** the retrospective of each session, which reads back what the owner corrected.

## Guarding `design@agent-skills@new-or-reshaped-head-needs-design`: an addition recorded directly outgrows its head's title `##title-stops-stating-scope`

The decision rests on the premise that a session reads a head's title strictly, and sends an
addition outside it to the design skill rather than recording it directly.

**Fires when:** a review finds a design head whose title no longer states a decision added to its
body by the direct route.
**Response:** open a `defect` naming the head and the commit, and reopen
`design@agent-skills@new-or-reshaped-head-needs-design` on its third case.
**Re-entry:** the standing-state review before every merge: the head and its title are in the
tree.

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
