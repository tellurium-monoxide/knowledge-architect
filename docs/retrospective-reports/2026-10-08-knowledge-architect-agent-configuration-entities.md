# Analysis of 2026-10-08-knowledge-architect-agent-configuration-entities

- **Received file:** 2026-10-08-knowledge-architect-agent-configuration-entities.md, the project's
  side of this repository's own retrospective of the agent-configuration-entities milestone.
- **Version it used:** built from this repository's source, main at v0.5.0-35-g8c0bf84.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1. This file holds the cluster on the mutation
  recipe.
- **Standing entries that bear on it:** those of the workflow file's analysis,
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`,
  and `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`, which records P7.

## P1, with P3 of 2026-10-09-knowledge-architect-staged-check: the mutation recipe gives a verdict for a run that tested nothing

- **Says.** Two failures of one recipe, in two sessions.
  - Here: a run passed `--test <test name>`, which cargo reads as a target name; cargo exited 101
    because no such target exists, and the run read as caught. Proposal: the verdict names the
    failing test, read from the run's output.
  - In the staged-check session: twice, a `cargo fmt` reformatting made a script's target string
    absent, the mutation was never applied, the diff was empty, and the test run printed a passing
    verdict that read like a survivor. Proposal: after applying the mutation,
    `git diff --quiet && echo "NOT APPLIED" && exit`.
- **Still applies.** Yes, both. `path@agent-config@skills/klarch-development/SKILL.md`:100-107, in
  `skill@klarch-development@discriminating-tests`:

  ```sh
  git diff                               # shows the mutation alone: the record quotes its - and + lines
  cargo test -p <package> --no-run       # it compiles, or it proves nothing
  cargo test -p <package>                # nonzero: caught; zero: SURVIVED
  ```

  And :125-127: "**A recorded check that does not show the mutation applied establishes
  nothing.** It quotes the `-` and `+` lines `git diff` printed, the file and the function or type
  that holds them, and the verdict." The record catches an unapplied mutation after the fact; no
  step stops the run. The verdict is read from the exit code alone, and a nonzero exit for another
  cause (a wrong target, a wrong filter) reads as caught.
- **Recorded.** No issue, no rejected alternative of the root or of core.
- **Kind.** A defect of the text, wrong: the recipe's verdict line equates a nonzero exit with a
  caught mutant, which is false whenever cargo fails before the test runs. Two causes, three
  instances, each caught by the session reading its own output.
- **Scope.** It serves `goal@knowledge-architect@the-workflow-improves-through-real-use` only
  indirectly; the recipe is this repository's own, `skill@klarch-development`. It reverses nothing.
- **Better fix.** One revision of the recipe covers both: after applying the mutation,
  `git diff --quiet && { echo "NOT APPLIED"; exit 1; }` stops the run; and the verdict line reads
  "nonzero, with the named test failing in the output: caught; zero: SURVIVED; nonzero for another
  cause: no verdict". The rival is no change, since the record's quotation of the diff already
  exposes an unapplied mutation: it does so after a verdict is printed, which is when the false
  reading is made.
- **Route.** A text edit of a project skill, under `skill@knowledge-architect-agent-configuration`.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## P2: commit-message claims written from memory

Cluster K7, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence.md`.
This session's at least nine message corrections are its largest instance.

- **Default:** handle now, with K7.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## P3: a commit chained after the gates with `;`, while the gates failed

- **Says.** The commit ran after `cargo x gates > file; echo gates $?;`, and the gates had failed.
  The report proposes naming `;` beside the pipe in the rule.
- **Still applies.** The rule already covers it. `path@knowledge-architect@CLAUDE.md`:421-425:
  "**Never chain a command on a verdict that went through a pipe.** … To bound a long report,
  redirect it to a file, read its last line, and chain nothing on the redirection's command." A
  commit after `;` behind a redirected gate is a command chained on the redirection's command.
- **Kind.** A lapse: the instruction exists and is clear.
- **Proposal.** No change.

- **Default:** no change, a lapse.
- **Outcome:** approved.

## P4: edits claimed and never written

- **Still applies.** The instruction stands, `path@knowledge-architect@CLAUDE.md`:552, "**Confirm
  that what you committed is what you wrote** before moving on." The report proposes no change.
- **Kind.** A lapse.

- **Default:** no change, a lapse.
- **Outcome:** approved.

## P5: the step order against the per-commit gate

The project's side of cluster K2, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`.
The root's sentence it names has since been replaced by a pointer to `primer@branch-commits`.

- **Default:** handle now, with K2.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## P6: the scope of the owner's word to merge

- **Says.** The owner said "merge, then proceed to slice N" several times; the session asked again
  at each later merge. The report proposes a line in the Git section: the owner's word to merge
  covers the pull request it names.
- **Still applies.** The report's premise is false: no instruction asks for the owner's word to
  merge. `instructions@git-workflow`, point 5: "**When CI passed on the current head, merge the
  pull request through GitHub.**" The owner's word is required for a release, per
  `design@knowledge-architect@publish-after-merge`, not for a merge.
- **Kind.** No defect. The session chose to ask; the text did not require it.
- **Proposal.** No change. If the owner wants a merge to wait for their word, that is a new rule,
  for the owner to state.

- **Default:** no change; no instruction requires the owner's word to merge.
- **Outcome:** approved.

## P7: a looser check has no class in the versioning policy

- **Recorded** as `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`.

- **Default:** no change, recorded.
- **Outcome:** approved.
