# Tripwires — agent skills

Evidence that would reopen a decision of `path@agent-skills@docs/design.md`. A tripwire leaves this
file when it fires. What is outstanding is `path@agent-skills@docs/open-issues/`.

## Guarding `design@agent-skills@design-hands-off-to-planning`, where no transcript exists `##ledger-lost-before-hand-off`

**Fires when:** where the harness keeps no transcript, a design discussion's ledger is lost to
compaction before the planning skill wrote the plan document, and the agent cannot reconstruct the
threads, their states and the conditions of each closure from what remains.
**Response:** reopen the decision, with the rejected alternative "Writing the discussion's ledger to
a file during the discussion" as the candidate.
**Re-entry:** the retrospective of the session it happens in.

## Guarding `design@agent-skills@ledger-from-transcript`: a ruling lost or misstated at assembly `##ruling-lost-in-assembly`

The decision rests on the premise that assembly from the transcript, checked by the transcript
reviewer, records the owner's rulings as the owner made them.

**Fires when:** a ruling of the owner is found missing from, or misstated in, a committed plan
document, after the commit.
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
