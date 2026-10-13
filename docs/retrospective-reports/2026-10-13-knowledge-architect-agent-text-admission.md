# 2026-10-13-knowledge-architect-agent-text-admission

- **Received file**: 2026-10-13-knowledge-architect-agent-text-admission.md, the project file of
  this repository's retrospective of the session that set what enters agent-facing text.
- **Version used**: the checker built from this repository, main at `v0.6.0-46-ge619a1e`, branch
  `agent-text-admission`.
- **Analysed against**: main at `v0.6.0-75-g5fca230`.
- **Standing entries that bear on it**: the one search over the 83 issues and 58 tripwires returned
  none for this file beyond those its workflow file's analysis names.

## P1. The root `CLAUDE.md` restated much of the primer it imports

- **a. What it says.** The root restated many primer sentences, each with its pointer. Both files
  are loaded together, so each was a second copy of one act in one loaded text. The owner ruled
  that the primer's copy stays. The file says it was fixed in the merged work, and proposes nothing.
- **b. Whether it still applies.** No. The commits "The primer and the root CLAUDE.md are one text:
  the root drops what the primer states" and "[review] Repairs of the review of the one-text ruling:
  the ruling narrowed to the primer, the root's last twins removed" removed them. A search of the
  root `CLAUDE.md` on main for "Goals bind", "presumption", "Met outside the task", "branch's first
  commit" and "A reference is written wherever" finds none. The whole root was not read again for
  twins in this analysis.
- **c. Whether it is already recorded.** No entry is needed.
  `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` concerns restatements of a skill's
  directive after an upgrade, and was reworded to say a restatement of the primer is a duplicate
  inside one text.
- **d. Its kind.** Wrong text, repaired in the session.
- **e.–g.** None.
- **h. The proposal.** No change.
- **Default**: no change, already repaired on main.
- **Outcome**: approved.

## The push and commit cluster: a commit or a push chained on a check it did not wait for

This cluster holds three items: the two "Instructions not followed" of this file, and P1 of
2026-10-13-knowledge-architect-owner-authority-milestone.

- **a. What it says.** Here: the session ran `cargo klarch check --staged` piped into another
  command, chained to `git commit`; two staged checks failed and the commits ran anyway. And it
  pushed once after `;`. In the owner-authority milestone: the session ran `commits` with its
  output redirected, then pushed after `;`, 10 times, each check passing; an eleventh time, the
  push followed `&&` on `tail` of the redirected output, the check had failed, and the failing
  commit was pushed. That file proposes a mechanical guard, a `pre-push` hook installed by the
  maintenance tool that runs `commits` on the pushed range, with the rule then stating the hook.
- **b. Whether it still applies.** The rules stand, unchanged in substance.
  `instructions@verify-mechanically`: "**Never chain a command on a verdict that went through a
  pipe.**" `instructions@git-workflow`, point 1: "A push after `;`, or after a check whose output
  went through a pipe, runs whatever the check found", whose home is
  `design@knowledge-architect@push-after-the-commits-check`.
- **c. Whether it is already recorded.** No issue. Two rivals already lost, in
  `path@core@docs/rejected-alternatives.md`: "`check` and `commits` refusing a pipe on stdout", since
  a process inside a pipeline does not set the pipeline's status; and "A `commit-msg` hook judging
  each message before the commit exists", on three counts: it guarded nothing the merge does not,
  it judged an amend against the commit being replaced, and "it was per-clone configuration every
  clone had to install and every session had to check". A `pre-push` hook runs after the commit
  exists, so the second count does not apply to it; the third applies in full; the first is weaker,
  since `design@knowledge-architect@push-after-the-commits-check` argues that a failing message on
  the remote branch is what a later fetch or review reads.
- **d. Its kind.** A lapse that recurs at one point of delivery: at least 14 commands across two
  sessions chained a commit or a push on a check whose verdict they did not take (at least two
  commits and one push here, eleven pushes in the other session), three of them over a failure. The rules are in the root `CLAUDE.md`, read at the start of a session, and needed at each
  commit and each push. The lapse rule of
  `skill@knowledge-architect-agent-configuration@admission-standard` repairs that by moving where the
  instruction is delivered.
- **e. Its scope.** This repository's own configuration: the root `CLAUDE.md`, and whatever
  mechanism delivers the rule at the moment of the push. It serves
  `goal@knowledge-architect@agents-work-without-drift`. A guard that runs the check is a new
  mechanism, a decision, and it must answer the lost `commit-msg` hook's third count.
- **f. A better fix.** Rivals:
  - a `pre-push` git hook, as proposed: per-clone configuration, the count that defeated the
    `commit-msg` hook;
  - a hook of the agent harness, committed in this repository's agent configuration, that runs
    before a `git push` command and refuses it unless `cargo klarch commits origin/main..HEAD` passes:
    it reaches every session started in this repository with no per-clone step, and it is delivered
    at the moment of the push. Whether the harness's committed settings carry such a hook for every
    session is a fact to measure, not established here; this repository commits no harness settings
    file today;
  - no change: the merge gate catches a failing message before main, as the lost hook's first count
    says, at the cost of a force-push.

  The fact that decides between the first two is the per-clone count, already argued in the record.
  The commit side, a staged check chained to `git commit`, is the same question at the commit; the
  discussion weighs whether one guard covers both. P2 of
  2026-10-13-knowledge-architect-owner-authority-milestone, a commit-time rule of the root also not
  followed, is weighed in the same discussion, per its own section.
- **g. Its route.** `skill@knowledge-architect-design`, on its in-change path: a new mechanism, with
  a rival that already lost on a count it must answer.
- **h. The proposal.** A design discussion of a guard delivered at the push, with the harness hook
  as the default shape, and the root's rule rewritten to state the guard.
- **Default**: handle now, under the design skill's in-change path.
- **Outcome**: no change, on the owner's word. The owner, in the file: "Skip or open an issue. I do NOT want to ship a prehook. Committing or even pushing content that failed the check is a minor issue anyway. It never prevents any repair, which are not that hard to do.  This is not even worth the prehook. This barely deserves to be a retrospective finding IMO." And, in the conversation, after being told the proposed hook was this repository's own and not shipped: "agreed, skip."
