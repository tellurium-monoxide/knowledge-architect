# Tripwires — agent skills

Evidence that would reopen a decision of `path@agent-skills@docs/design.md`. A tripwire leaves this
file when it fires. What is outstanding is `path@agent-skills@docs/open-issues/`.

## Guarding `design@agent-skills@designing-hands-off-to-planning` `##ledger-lost-before-hand-off`

**Fires when:** a design discussion's ledger is lost to compaction before the planning skill wrote
the plan document, and the agent cannot reconstruct the threads, their states and the conditions
of each closure from what remains.
**Response:** reopen the decision, with the rejected alternative "Writing the discussion's ledger to
a file during the discussion" as the candidate.
**Re-entry:** the retrospective of the session it happens in.

## Guarding `design@agent-skills@expectation-set-bounds-scope` `##expectation-set-closes-a-contradiction`

**Fires when:** a finding that two installed instructions leave no move satisfying both is closed
by citing an expectation set.
**Response:** reopen the finding, and reread the entry's boundary sentence for the wording that
allowed it.
**Re-entry:** the standing-state review of the change that closed the finding.
