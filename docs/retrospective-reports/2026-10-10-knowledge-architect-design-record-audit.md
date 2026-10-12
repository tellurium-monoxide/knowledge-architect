# Analysis of 2026-10-10-knowledge-architect-design-record-audit

- **Received file:** 2026-10-10-knowledge-architect-design-record-audit.md, the project's side of
  this repository's own retrospective of the first design-record audit.
- **Version it used:** built from this repository's source, main at v0.5.0-80-gf422e5b.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1.
- **Standing entries that bear on it:** those of the workflow file's analysis,
  `path@knowledge-architect@docs/retrospective-reports/2026-10-10-knowledge-architect-design-record-audit-klarch-workflow.md`,
  and `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`, which records that a plain
  name of a deleted entry is reported by nothing, as P2's is.

## P1: commit-message claims not listed and checked before the commit

Cluster K7, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence.md`.
This file's proposal, a claim list in a scratch file, is weighed there and loses to a sentence at
the point of delivery.

- **Default:** handle now, with K7.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## P2: a closed issue named in plain text in a design head

- **Says.** The head `design@agent-skills@a-head-is-owed-by-an-entry-test` names a closed issue in
  plain text, with a `git log --diff-filter=D` command that finds it. The issue-tracking skill says a
  sentence about the past whose reference dangles "is rewritten to state the present, or removed …
  never turned into plain text". The defeating fact it carries is already stated in the head. The
  report proposes removing the clause and its command.
- **Still applies.** Yes. `path@agent-skills@docs/design.md`:665-669: "an audit counted two skills
  stating one decision as one site, and the clause could be read to admit every decision about
  installed text, as the closed issue what-a-site-is-for-installed-text-is-undefined records, found
  with `git log --diff-filter=D -- '*/what-a-site-is-for-installed-text-is-undefined.md'`." It
  entered in 8a09a07, "[review] Six reviewers read the branch…". The design-record pass of
  2026-10-12 did not touch it. The rule is at
  `path@agent-skills@content/skills/issue-tracking/SKILL.md`:299-301.
- **Recorded.** No issue. The tripwire it was weighed against is judged not fired, in the workflow
  file's analysis.
- **Kind.** A non-conformance on main, with the fix the rule names.
- **Scope.** Owned by agent-skills, the design home. Removing a clause that names where a fact is
  recorded, while the fact stays in the head, changes no statement of the decision.
- **Better fix.** The report's: remove the clause from "as the closed issue" to the end of the
  command. The deliberation stays reachable through the commit that closed the issue, whose message
  cites it against its parent tree.
- **Route.** A text edit of a design home, under `skill@knowledge-architect-decision-recording`,
  which is loaded before any write into a design home.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## P3: the Workflow tool started without the owner's opt-in

- **Still applies.** No longer arises for an audit. `design@agent-skills@audit-axis-runs-a-workflow`
  records "The tool runs only on an explicit opt-in, such as the owner invoking a skill whose
  instructions tell the session to call it", and the project-audit skill now tells the session to run
  each pass as the saved workflow, since 80bab85 and c9e0d4d. The report states the harness's rule
  and proposes nothing.

- **Default:** no change, repaired by design.
- **Outcome:** approved.

## Met outside the task: fourteen local branches from earlier work

- **Says.** Fourteen local branches, such as gates-crate and release-0-1-0, wait for the owner's
  word, since "Deleting one is a content-loss risk".
- **Still applies.** The premise is false. The same fourteen stand today, and for each,
  `git merge-base --is-ancestor <branch> main` is true and an origin ref exists: deleting the local
  branch loses no commit.
- **Kind.** Not a finding on any instruction; local housekeeping.
- **Proposal.** No change to any text. Deleting the fourteen local branches is the owner's to
  decide; it loses nothing.

- **Default:** no change to the text; the branches are left for the owner.
- **Outcome:** approved.
