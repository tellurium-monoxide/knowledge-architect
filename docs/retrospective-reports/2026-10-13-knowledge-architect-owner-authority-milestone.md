# 2026-10-13-knowledge-architect-owner-authority-milestone

- **Received file**: 2026-10-13-knowledge-architect-owner-authority-milestone.md, the project file
  of this repository's retrospective of the session that built the owner-authority milestone.
- **Version used**: the checker built from this repository, main at `v0.6.0-46-ge619a1e`.
- **Analysed against**: main at `v0.6.0-75-g5fca230`.
- **Standing entries that bear on it**: from the one search over the 83 issues and 58 tripwires,
  `three-findings-of-a-received-retrospective-await-an-intake`, P3's own
  case, and `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`,
  whose closing condition P1 does not meet, since it asks for a project that follows the installed
  text, and this repository runs `commits` after each commit by its own root.

## P1. A push chained on something other than the bare `commits` check

Analysed with the push and commit cluster in 2026-10-13-knowledge-architect-agent-text-admission,
whose proposal weighs this finding's own, a guard delivered at the push.

- **Default**: as that section's outcome.
- **Outcome**: as the section it points to rules. The owner: "For the pending ones, I gave my word in the section holding the detailed analysis, you can point to them."

## P2. The claims of a commit message are not checked one by one before the commit

- **a. What it says.** The root `CLAUDE.md` says "**Before each commit, list every factual claim its
  message makes, and check each one.**" In six review rounds a reviewer found a false or incomplete
  claim in a commit message: a search result misreported, a classification said to be complete, an
  outcome said to be repaired that was not. The transcript shows no claim list made before any
  commit, an inference from its absence. The fixes proposed: a scratch file per commit holding the
  list, written before `git commit`, so that skipping it is visible; or the decision-record
  reviewer's brief always checks the messages' claims.
- **b. Whether it still applies.** The rule stands, at `instructions@verify-before-relying`, section
  "Verify a claim before writing it". The second fix already exists:
  `path@agent-skills@content/agents/decision-record-reviewer.md`:47-48 reads the branch's messages
  for "every factual sentence about the tree", and lines 110-114 check a sentence that states what
  was searched, counted or renamed against the tree. That reviewer runs when "a decision was made,
  reversed or harvested", per `skill@knowledge-architect-review@review-axes`.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** A lapse that recurs at one point of delivery: six instances in one session, of a
  rule read at the start of a session and needed at each commit. It is near W6 of
  2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow, whose completeness claims
  were among the false ones.
- **e. Its scope.** This repository's own configuration, the root `CLAUDE.md`.
- **f. A better fix.** Rivals:
  - the scratch file per commit, a structure on every commit for a rule that already names the act;
  - the reviewer's check, which exists and caught these, after the commit and at the cost of a
    history edit;
  - the rule delivered at the moment of the commit, by the same mechanism the push and commit
    cluster weighs for the push. The fact that decides for it is the lapse rule: the recurrence is at
    one point of delivery, and the cure it names is to move the delivery there. One design discussion
    weighs both moments.
- **g. Its route.** `skill@knowledge-architect-design`, in the same discussion as the push and
  commit cluster of 2026-10-13-knowledge-architect-agent-text-admission.
- **h. The proposal.** The design discussion of the push guard also weighs delivering the
  claim-list rule at each commit.
- **Default**: handle now, in the same design discussion as the push and commit cluster.
- **Outcome**: no change, after the short design session the owner asked for. The owner, closing it: "claim-list-no-change". Before it, in the file: This is normal behavior. The reviews are there for that. If there was a real solution to this, no review would ever be needed... Maybe the scratch file solution would help. OK for the short design session.

## P3. No route for a received file the owner sends in part to a design discussion

Analysed with W8 of 2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow, which
proposes no change: the intake's existing route covers the case.

- **Default**: as that section's outcome.
- **Outcome**: as the section it points to rules. The owner: "For the pending ones, I gave my word in the section holding the detailed analysis, you can point to them."

## A commit on main names a file outside the tree in plain text

- **a. What it says.** The commit "The owner-authority milestone: a question goes to the authority
  that answers it, and the owner rules at a fork" names the received retrospective file in plain
  text, where `path@elsewhere@<path>` expresses it. A commit on main is never reworded. The file
  reports it only.
- **b. Whether it still applies.** The message is on main, and stays.
- **c. Whether it is already recorded.** No. `tripwire@agent-skills@plain-text-pointer-found` fires
  on a pointer written as bare plain text "to clear a finding"; this one cleared no finding, and the
  checker raised none, since a plain path is not read.
- **d. Its kind.** A lapse, with no repair possible: `design@knowledge-architect@git-flow` forbids
  rewriting main's history.
- **e.–g.** None.
- **h. The proposal.** No change.
- **Default**: no change.
- **Outcome**: approved.
