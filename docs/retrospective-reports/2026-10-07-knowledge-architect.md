# Analysis of 2026-10-07-knowledge-architect

- **Received file:** 2026-10-07-knowledge-architect.md, the project's side of this repository's own
  retrospective: findings on this repository's own instructions.
- **Version it used:** built from this repository's source, main at v0.4.0-41-gf5d403a.
- **Analysed against:** the same main, plus the commit "A received retrospective file is analysed
  under klarch-retrospective-intake, the retrospective names its files by subject, and the searcher's
  groups are balanced".
- **Standing entries that bear on it:** `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`
  (P5), `issue@core@branch-sha-citations-are-judged-within-the-range-only` (cited in the section P1
  and P2 concern).

## P1: the reword procedure checks that the tree is unchanged, not that the message changed

- **Says.** The procedure checks `git diff <old-head> HEAD` is empty after a `filter-branch` reword.
  That shows no content was lost, and not that the reword applied. Three scripted rewords failed
  silently, on a wrapped line twice and on an apostrophe once, and one pushed the old message.
  Proposed fix: after a reword, check that the new message holds the reworded text before deleting
  the backup refs and before any push.
- **Still applies.** The root `CLAUDE.md`, `## Git` point 2, the paragraph "To reword the message of a
  commit that is not the newest", unchanged since `007eda1`. The three failures are in that session's
  transcript, not in the tree.
- **Recorded.** None.
- **Kind.** Missing: the procedure's only check cannot fail on the failure it exists for.
- **Scope.** The root; `design@knowledge-architect@git-flow`, whose history-edit rule this
  restatement serves. The head states the verifications for content loss only, and this adds a check
  on the edit itself, within its title.
- **Better fix.** The report's greps for a new phrase, which needs the phrase typed twice and
  fails on a wrapped phrase the same way the script did. Rival: compare the old and new messages of
  every rewritten commit, `git range-diff <old-base>..<old-head> origin/main..HEAD`, which prints each
  commit's message diff; a reword that applied shows a diff on each commit it meant to change, and one
  that did not shows none. The command is already the one point 2 names for a rebase.
- **Route.** A text edit, the root `CLAUDE.md`.
- **Proposal.** Handle now: the paragraph gains the `range-diff` check, before the backup refs are
  deleted and before any push.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## P2: the push rule was broken once

- **Says.** A push chained with `;` after `cargo klarch commits` pushed a failing message. The
  instruction is clear; the session wrote it from habit.
- **Still applies.** The rule stands as quoted, root `CLAUDE.md` `## Git` point 1.
- **Kind.** A lapse, once. The report proposes no change beyond P1.
- **Proposal.** No change.
- **Default:** no change.
- **Outcome:** approved.

## P3: where a decision about the published workflow lives

- **Still applies.** No: repaired by `644889b`, which gave the root `CLAUDE.md` its knowledge-table row
  "a decision about the published workflow".
- **Proposal.** No change: already repaired.
- **Default:** no change.
- **Outcome:** approved.

## P4: no development procedure named for the shipped text

- **Says.** `klarch-development` excludes the installed text, and a plan named no procedure for it;
  the report says the skill now points to the agent-skills `CLAUDE.md`.
- **Still applies.** No, and not for the reason given: the skill's pointer to "Editing an installed
  skill or agent" dates from `405107f`, before that session. The session's repair was in the plan
  document, which named the procedure, in `e58516c`. The pointer existed; the plan's author did not
  use it.
- **Kind.** A lapse of the plan's author, repaired in the plan.
- **Proposal.** No change.
- **Default:** no change.
- **Outcome:** approved.

## P5: the release step said the primer's import line is no candidate

- **Still applies.** No: repaired by `249f145`, which also moved the hand check onto the installed
  copies. The underlying defect of the checker is
  `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`, the same as C2 of the workflow
  file.
- **Proposal.** No change: already repaired, and recorded.
- **Default:** no change.
- **Outcome:** approved.
