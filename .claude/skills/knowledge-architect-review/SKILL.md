---
name: knowledge-architect-review
description: MUST use before merging anything to the main branch, and whenever an activity's own skill says a unit of its work is ready for review — how to send independent subagent reviewers at it, how to write a brief that cannot mislead them, and where their findings land. The axes to send are the dispatching activity's, and its skill names them.
---

# Dispatching a review

Scope: sending independent reviewers at work you produced, whatever kind of work it is. One
sub-activity, required as a prerequisite by every activity that produces something reviewable.

**When to dispatch is the activity's**, because the moment differs and so does the vocabulary that
names it: the project's development procedure says when a piece of code is ready,
`knowledge-architect-planning` says when a plan document is, and
`knowledge-architect-agent-configuration` says when a configuration change is. Read your
activity's skill for its moment. The one moment that belongs to no activity is **before merging
anything to the main branch**.

Not covered here: **being** any of the reviewers (the record reviewers
`knowledge-architect-routing-reviewer`, `knowledge-architect-decision-record-reviewer` and
`knowledge-architect-standing-state-reviewer`; the plan-document reviewers
`knowledge-architect-cold-implementer-reviewer` and `knowledge-architect-code-claims-reviewer`;
and `knowledge-architect-transcript-reviewer`, all dispatched rather than read), and
**recording** what a review changes (`knowledge-architect-decision-recording`,
`knowledge-architect-issue-tracking`).

## 1. The axes

| axis | what it does | applicable when |
| --- | --- | --- |
| spec conformity | does the work implement what was decided, item by item. If deviations happened during implementation, are they justified? | a spec or a milestone slice's spec was written before the work |
| self-consistency | does the result contradict itself. Two instructions a reader cannot both obey, a pointer into content that is not there, a statement no longer true | nearly all the time |
| fidelity of relocation | where content moved, was anything lost? A reason dropped, a number changed, an argument compressed to an assertion | content was relocated, or forked from another source |
| routing of knowledge | `knowledge-architect-routing-reviewer` | a durable statement was added or moved |
| decision recording | `knowledge-architect-decision-record-reviewer`. If a plan document was written, hand it to this reviewer too | a decision was made, reversed or harvested |
| conformance | `knowledge-architect-standing-state-reviewer` | before every merge to the main branch, since it is the standing re-entry point of every tripwire and every deferred trigger |
| cold implementer | `knowledge-architect-cold-implementer-reviewer`: can a session that did not see the discussion act on the plan document | a spec or a milestone was written under `knowledge-architect-planning`, or a decided shape in one revised; that skill names the moment |
| code claims | `knowledge-architect-code-claims-reviewer`: is every statement the plan document makes about existing code true of the tree | the same moment |
| transcript | `knowledge-architect-transcript-reviewer`: has everything the work's sessions established that must outlive them a durable outcome, and is no ruling of the owner misstated | the transcripts of the sessions that produced the work are available; and once more before every merge to the main branch, alone and last (below) |

Each of these is conditional on the work. In other skills, more axes are added to this list, when
the work has properties these axes do not reach. **An axis named by an agent is dispatched as that
agent. An axis with no agent is dispatched as a fresh general-purpose subagent**, briefed with the
axis's question as this table states it, the commit range, and the files that carry the standard. **An axis that applies and was not run is said,
with the reason, in the commit that records the review.**

**Last, before every merge to the main branch, the transcript reviewer runs once more, alone**:
after every other axis has run and its repairs are committed. Its range is the whole branch,
`<main branch>..<head>`, and its brief names the transcripts of every session that worked on the
branch, with the message where the branch's work begins in each. It is the one axis that can see whether the findings of the others were acted on, since
their reports reach the session as messages of its transcript. Its repairs land as additional
commits, or are folded where §3 says. **Where one of them makes or reverses a decision, the decision-record axis reviews it**,
and any other axis whose condition it meets; the review ends with their repairs. An additional
commit that makes no decision is reviewed by no axis again.

**Find each transcript file by the message where the work begins, never by a session
identifier.** Search the harness's transcript directory for the opening words of that message, and
name a file only once it is seen to hold the message: a session that was cleared or compacted can
leave an earlier session's identifier in the harness's paths.

## 2. The invariants

These hold whatever the axes are. They are what makes a finding worth acting on rather than
re-checking.

- **Each axis is a separate subagent, and none sees another's findings.** Reviewers given a shared
  findings list converge on it. The transcript reviewer is the one exception: its subject is the
  session, and the session received the other axes' reports, so it reads them as items to find
  an outcome for, never as a list to agree with.
- **Each reviewer is fresh, never a fork.** A fork inherits the session's discussion and reads the
  work as its author.
- **At least one reviewer is briefed blind**: given the task and no list of what anyone else found,
  and no summary of what is believed to be true. The finding that changes a design is usually the
  one nobody was looking for.
- **Reproduce before claiming.** Every reviewer is told to reproduce anything it asserts, and to
  drop what it cannot. This is the instruction most likely to be dropped when the procedure is
  rewritten, and it is the whole difference between a finding you act on and a finding you re-check.
- **A brief must not assert what the tree contains.** Give the task and the standard, and tell the
  reviewer to establish the state itself. A brief that describes the implementation sends a reviewer
  to test something that may not be there, and it cannot tell a wrong brief from a broken artifact
  unless it was told to look.
- **Name what is reviewed as a commit range**, `<main branch>..<commit under review>` for a branch
  about to merge: the diff and every commit message in it are the subject, and a reviewer given a
  branch name alone guesses the base.
- **Name the files that carry the standard rather than restating it.** A subagent inherits the
  session's snapshot of the root `CLAUDE.md`, so a session that has just edited it is briefing from
  a copy that no longer matches disk. Point at the file; do not paraphrase what it says.
- **The dispatcher does not defend the work.** Findings arrive as claims to check, not as attacks to
  answer. Verify each against the tree before relaying or acting on it; a reviewer can be wrong, and
  saying so requires the same reproduction the reviewer owed.
- **Every reviewer that runs tests, a mutation, the checker or any binary gets its own worktree,
  detached at the commit under review, never the live tree.** A run in the live tree races the
  dispatcher's own edits, and a worktree that shares the branch ref moves under the reviewer at the
  dispatcher's next commit. The shape is `git worktree add --detach <path> <commit>`, at a path
  where it pollutes no search, and `git worktree remove <path>` once the review and the repairs are
  done. **The dispatcher names each reviewer's path in its brief, distinct for each reviewer**, so
  reviewers dispatched together never build inside one another's worktree. **The reviewer's build output
  stays inside its worktree.** Unless every target is tied to its checkout, as the setup skill
  shows, a build directory two checkouts share lets the live checkout run the reviewer's build,
  which judges the live tree with the reviewer's code. Where every target is tied, a shared
  directory costs a rebuild at each switch between the two checkouts instead. In a Rust project,
  the reviewer sets `CARGO_TARGET_DIR` to a directory inside its worktree that the project's
  ignore rules cover, such as its `target/`: a build directory git does not ignore is reported by
  the checker as untracked files.
- **No reviewer edits the tree, and none runs an operation that can lose content**: no stash, no
  reset, no checkout of a path.

## 3. What a review leaves behind

**Findings** become one of:

- repairs, done on the branch before merge, if the defect is too large to consider the task
  achieved;
- issues, one file each in the affected anchor's issue directory
  (`knowledge-architect-issue-tracking`);
- nothing, where the finding is judged to need nothing, with the reason.

**A finding that makes several claims gets an outcome for each claim**: repaired, opened as an
issue, or judged to need nothing, with the reason. A claim left without an outcome of its own is
lost unless a later review finds it.

**Every item of a report gets an outcome, not only its numbered findings**: a note outside the
reviewer's axis, an observation it declined to call a finding, a remark that a defect predates the
change. Each is a claim about the work like any finding. So is a repair the dispatcher promised the
owner while answering a review. The record of the review lists each item with its outcome.

A transcript reviewer's finding that something **has no durable outcome** is acted on by the
dispatcher without waiting for the owner: it is recorded in its home, repaired, opened as an issue,
or judged to need nothing, with the reason. The exception is a decision that creates a design head,
contradicts a statement of one, or takes one beyond what its title states, and was not argued: it goes to
`knowledge-architect-design` first, per `knowledge-architect-decision-recording`. Each outcome is reported to the owner, in the record of
the review and at the end of the turn. A ruling the reviewer finds misstated is the owner's, and is
put to the owner.

**Where the branch's commits reach the main branch as they are** (a fast-forward, or a rebase merge,
which keeps their trees and messages and may give them new SHAs):

- **A repair made on the branch is a new commit, appended**, which edits no history. Where the
  project requires every commit of a branch to pass checks the repair changes, an appended repair
  leaves the earlier commits failing; it is then folded into the earliest commit it repairs, by a
  history edit with no uncommitted work in the tree, confirmed afterwards to have lost no content.
  Either way, the paragraph recording the review says what was repaired, and what was folded.
- **A commit message carrying a mistake is repaired by amending** while it is the newest commit, and
  by a history edit of the branch after that. Either only with no uncommitted work in the tree, and
  each confirmed afterwards to have lost no content: for an amend, that it changed no file. A
  correction written into a later commit's message is not a repair: the mistaken message still
  reads as it did.
- **A paragraph in the commit message** records the review.

**Where the project squashes a branch into one commit on merge**, the record of the review goes
where the project keeps what survives the merge: the squashed commit's message, or the pull
request. Follow the project's own rules on that.

The record tells what was reviewed, on which axes, how consequential the
findings were, which were repaired, which were left and why, and which axis was not run and why.
It names each reviewed commit by its subject, not by its SHA: a SHA of the branch may name nothing
once the branch reaches the main branch, since a rebase before the merge or at it gives the
branch's commits new SHAs.
Every claim that paragraph makes about the tree is checked before it is written, like any other.
