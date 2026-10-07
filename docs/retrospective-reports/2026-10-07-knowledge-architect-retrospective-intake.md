# Analysis of 2026-10-07-knowledge-architect-retrospective-intake

- **Received file:** 2026-10-07-knowledge-architect-retrospective-intake.md, the project's side of
  this repository's own retrospective.
- **Version it used:** built from this repository's source; the session ended on main at
  v0.4.0-59-gf983d4f.
- **Analysed against:** main at v0.4.0-59-gf983d4f, the same.
- **Standing entries that bear on it:** `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`
  (P1 changes a lifecycle that entry describes as this repository's own),
  `tripwire@agent-skills@title-stops-stating-scope` (P1).

## P1: the analysis's head says the deleting commit carries out the last outcome, and the skill's steps lead to a separate one

- **Says.** The session deleted the analyses in a commit after the one that carried out their last
  outcome. The head says "the commit that carries out its last outcome deletes it". Proposed fix:
  "or a later commit of the same branch", at every site that states it.
- **Still applies.** The rule stands at four sites: the head
  `design@knowledge-architect@committed-findings-analysis` in `path@knowledge-architect@docs/design.md`,
  step 4 of §6 of `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`, the README of
  `path@knowledge-architect@docs/retrospective-reports/`, and the row of the root `CLAUDE.md`
  knowledge table. Step 3 of the same §6 says the findings are handled "in as many commits or
  branches as their routes need", so a last outcome may land in a commit about one subject, and the
  deletion in another.
- **Recorded.** None.
- **Kind.** Wrong: following the steps produced the deviation a reviewer reported.
- **Scope.** The root. It changes a statement of a head within its title, which says the analysis
  "leaves when its last outcome is carried out": a later commit of the same branch still meets it, so
  `tripwire@agent-skills@title-stops-stating-scope` does not fire. The reason the head gives, every
  finding reaching an outcome that history keeps, holds either way.
- **Better fix.** The report's. Rival: keep the rule and fold the deletion into the last handling
  commit; it ties two subjects into one commit, which the primer's commit-of-its-own rule and the
  step list both work against.
- **Route.** A text edit at four sites, one a head: on the in-change path, under
  decision-recording.
- **Proposal.** Handle now.
- **Default:** handle now.
- **Outcome:** approved.

## P2: false claims in commit messages and an issue entry, against "Verify a claim before writing it"

- **Says.** Seven claims were false when first written; three caught by the session, four by
  reviewers. Proposed fix: none.
- **Still applies.** The instruction stands in the root `CLAUDE.md`, section "Verify a claim before
  writing it": "Before each commit, list every factual claim its message makes, and check each one."
- **Recorded.** None.
- **Kind.** A lapse: the instruction is clear and delivered every session. The report notes that a
  retrospective of another project counted seven in one session too.
- **Better fix.** No change. A delivery change has no better point: the instruction is already read
  at every commit, being in the root `CLAUDE.md`.
- **Proposal.** No change: a lapse.
- **Default:** no change.
- **Outcome:** approved.

## P3: restatements without their pointer, three times

- **Says.** The routing reviewer found three restatements without their pointer. Proposed fix: none.
- **Still applies.** The rule stands in the root `CLAUDE.md`: "a restatement carries its pointer,
  adjacent."
- **Kind.** A lapse, caught by the axis that exists to catch it.
- **Proposal.** No change: a lapse, caught by the routing review each time.
- **Default:** no change.
- **Outcome:** approved.
