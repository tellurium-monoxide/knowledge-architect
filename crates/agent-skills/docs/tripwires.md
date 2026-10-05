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
**Re-entry:** the standing-state review before every merge, and the retrospective of the session
that finds it.
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

## Guarding `design@agent-skills@every-decision-through-design`: the trigger sends the owner decisions that earn no record `##decision-trigger-fires-too-often`

The decision rests on the premise that a session judges correctly which decisions earn a record,
so that only those reach the owner as a design discussion.

**Fires when:** in two sessions, found by a retrospective or a transcript review, the owner says a
design discussion was not needed for a decision that earned no durable text.
**Response:** open a `defect` naming both sessions and the decisions, and reopen
`design@agent-skills@every-decision-through-design` on the wording of its scope and of the design
skill's description.
**Re-entry:** the retrospective of each session, and the transcript review before every merge.

## Guarding `design@agent-skills@every-decision-through-design`: a decision reaches a design head with no discussion `##decision-trigger-does-not-fire`

The decision rests on the premise that the design skill's description and the decision-recording
skill's backstop reach a session before it writes a decision met during another task.

**Fires when:** a transcript review finds a design head written or rewritten on a branch whose
transcripts hold no thread for that decision.
**Response:** open a `defect` naming the head and the branch, and reopen
`design@agent-skills@every-decision-through-design` on where the trigger is delivered, with the
primer line it rejected among the candidates.
**Re-entry:** the transcript review before every merge.

## Guarding `design@agent-skills@in-change-path`: a ruling lost or misstated in an in-change commit message `##ruling-lost-in-change`

The decision rests on the premise that a commit message records the owner's rulings as the owner
made them, as a plan document does, with no review of a plan document before the work.

**Fires when:** a ruling of the owner is found missing from, or misstated in, the commit message
that carries the deliberation of a discussion on the in-change path, after the branch's transcript
review.
**Response:** open a `defect` naming the ruling and the commit, and reopen
`design@agent-skills@in-change-path` on its entry condition: whether a discussion with more than
one approved thread takes the full path.
**Re-entry:** the standing-state review before every merge, and the retrospective of the session
that finds it.
