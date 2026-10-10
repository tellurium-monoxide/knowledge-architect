---
name: knowledge-architect-project-audit
description: MUST use when the owner asks to audit one aspect of the whole project rather than the work of one branch; when a pin move crosses a version whose changelog has a Migration entry citing an axis of this skill; and when an adopting project's existing documents have been moved into the workflow's homes. Covers what a project audit is, its method (the corpus, a dispatch each axis gives, drafts in files, the sort, one list for the owner, the application, a fresh re-check, the review), the outcome of each kind of finding, where the record of a run goes, and one section per axis: the design-record axis, which re-applies the rules on design heads and rejected alternatives to every entry, and the agentic-workflow axis, which reads every instruction the harness delivers as one whole for instructions a session cannot follow.
---

# Auditing a project

Scope: reading one aspect of a whole project, every entry or text of a corpus, against the rules
that apply to it, and bringing what fails into line. A project audit is the reading of the whole
record on one axis; a review reads a diff. One activity: its artifact is a repaired record across the
project, not a verdict on one branch.

**Not covered here**, each named where it lives:

- **A review of a diff**, the work of one branch or one plan document:
  `skill@knowledge-architect-review`. A project audit's own branch is reviewed under it, as any
  change is.
- **The design audit** of a slice or a spec before its work is built:
  `skill@knowledge-architect-planning`.
- **A retrospective's findings**: `skill@knowledge-architect-retrospective`.
- **Recording** what an audit changes: `skill@knowledge-architect-decision-recording` for a design
  head and a rejected alternative, `skill@knowledge-architect-issue-tracking` for an issue and a
  tripwire, and `skill@knowledge-architect-goal-setting` for a conflict with a goal. Read the
  first before any write into a design home.

## Terms {{slug:audit-terms}}

| word | meaning |
| --- | --- |
| **axis** | one aspect an audit reads, with its corpus, its rules, its agent and the outcome of each kind of finding. Each axis has a section below |
| **corpus** | the entries or the texts an axis reads, all of them |
| **dispatch** | how an axis shares its corpus out among agents, how many, what each brief names, and how they are calibrated, if they are. Each axis's section gives its own |
| **draft** | one agent's file for one entry or for one finding, as the axis says: the verdict or the finding, the evidence, a proposed edit and a proposed outcome |
| **the owner list** | one message to the owner holding every finding that needs the owner's word |
| **the re-check** | fresh agents of the axis, dispatched as its first dispatch was, after the edits |
| **pass** | one round of the method: the drafts, the sort, the owner list, the application. The re-check's drafts start the next pass |

## When an audit runs {{slug:when-an-audit-runs}}

- **The owner asks for one**, naming the axis. This is the main occasion.
- **A pin move crosses a version whose changelog has a Migration entry citing an axis's section of
  this skill.** The axis runs after the pin's commit, in a branch of its own, as
  `skill@knowledge-architect-setup@moving-the-pin` says. A version that changes the rules an axis
  applies names the axis in its Migration entry, so a project brings its existing record to the
  new rules.
- **An adopting project's existing documents have moved** into the workflow's homes, as
  `skill@knowledge-architect-setup@existing-documentation` says: the milestone of the move ends by
  running the design-record axis over its whole corpus.

An audit runs in a branch of its own, unless the owner gives it to another work's branch.

## The method {{slug:audit-method}}

Each point leaves judgement to the session, and an axis's section adjusts it where its corpus
needs. The points hold for every axis; the dispatch is the axis's own, in its section.

1. **The corpus.** The axis's section names its corpus and what is read beside it.
2. **The dispatch.** Dispatch the axis's agents as its section says, all in parallel, each with a
   scratch directory of its own.
3. **Pre-commitments.** An axis that measures, one whose run yields a figure a conclusion rests on,
   states before the run which figure would show which conclusion, and the owner sees them before
   the run. An axis that only applies rules owes none, and the count of violations left per pass,
   which point 8 reads, makes no axis one that measures.
4. **Drafts.** Each agent writes its drafts in its scratch directory and edits nothing else.
   Results stay in the files: the session reads the drafts, not a summary of them.
5. **The sort.** Read every draft whose proposed outcome is not "nothing" against what it cites and
   the history it cites. Sort each finding into its outcome, under
   `skill@knowledge-architect-project-audit@outcomes`. A draft found wrong is corrected, and the same
   agent's other drafts that the axis's section names are read again. Nothing an agent proposes is
   applied before a reading confirms it. A quotation of the owner is confirmed against the question
   it answered, in its source, as well as against its words: a verbatim answer to another question
   is not the owner's ground for this one.
%% The `F` label: `design@agent-skills@ruled-items-labelled`.
6. **The owner list.** One message per pass: the items grouped by kind, each under a label, `F<n>`, with a
   default and the words of the entry or the text the item turns on. Apply nothing on the list
   before the owner answers. An item the owner defers becomes an issue entry.
7. **Application.** Apply the edits, yourself or through agents dispatched on disjoint files. Read
   the whole diff before each commit.
8. **The re-check.** Dispatch fresh agents of the axis, as its first dispatch was, after the edits,
   each also given the entries, the texts and the rules the owner ruled to keep, as a file in its
   scratch directory. A finding of the re-check that a reading confirms, that is not on the kept
   list, and that no issue entry of the run records, is a violation left, whether a draft missed it
   or an edit made it: repair it or put it to the owner. The re-check's drafts start the next pass,
   from point 5. The run stops when the owner judges what is left small enough. The violations left
   then take their outcomes, as in points 5 to 7, and no re-check follows; one the owner does not
   rule on becomes an issue entry, as a deferred item does. The count of violations left is expected
   to fall from one pass to the next; a pass where it does not goes to the owner as a question,
   with the counts, and starts no new pass on its own.
9. **Review.** The audit's branch is reviewed under `skill@knowledge-architect-review`, on every
   axis of its table whose condition holds, which include self-consistency, fidelity of
   relocation, routing of knowledge, decision recording, standing state, and the transcript review
   last.

## Outcomes {{slug:outcomes}}

Every finding takes exactly one outcome. A finding that fits no named case goes on the owner list.

| outcome | the findings |
| --- | --- |
| applied in the branch | the edits the axis's section lists as applied |
| put to the owner, on the owner list | the edits the axis's section lists as the owner's; a verdict where a rule reads two ways; a conflict with a goal, which then follows `skill@knowledge-architect-goal-setting`; any finding no row names |
| sent upstream | a finding on the installed text, which the project never edits: the axis's section says how |
| an issue entry | a finding outside the axis that the primer's table routes to an issue, such as a statement false of the code when the axis does not read the code; work the owner approved that is too large for the branch; a finding the owner defers. The kind follows `skill@knowledge-architect-issue-tracking` |
| nothing | the entry or the text conforms |

A finding outside the axis takes the first case of `primer@met-outside-the-task` that applies.

## The record of a run {{slug:record-of-a-run}}

%% The record is the commit messages, as a measurement is routed to the commit that took it; the
%% rival, a report document, is the open question of
%% `issue@agent-skills@audit-axes-beyond-the-design-record` for an axis whose fixes are code.
**The commit messages are the record.** Each commit of a run says what was audited, at which
commit, named by its subject since a rebase merge gives the audit branch's commits new SHAs, how it
was dispatched, the calibration reading settled where the axis has one, and the outcome of every
finding: applied, ruled by the owner under its label, opened as an issue with its id, or found to
need nothing. A figure the run measured goes there too. A finding whose only record is a scratch
file, or a plan document that will leave, is lost when that file goes.

%% The case of a run that changes no file follows
%% `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`.
A run that changes no file leaves no commit that changes a file, and a merge that drops such a
commit, as a rebase merge does, would drop its record. Its record is then the report to the
owner.

## The design-record axis {{slug:design-record-axis}}

**The corpus**: every design head of every Component, and every entry of its rejected
alternatives.

**The order of a Component's entries, and the inputs read beside each head**, are in
`agent@knowledge-architect-design-record-auditor`'s description, which the harness lists: cut the
groups by that order.

**The rules**: every rule of `primer@design-heads` for a head, and every rule of
`skill@knowledge-architect-decision-recording@losing-alternatives` for a rejected alternative.

**Not in this axis**: whether a head is true of the code, and the restatements of a head in other
texts. A head found false of the code gets an issue entry, kind `defect`.

**The agent**: `agent@knowledge-architect-design-record-auditor`.

**The dispatch:**

- **Groups.** A group holds the entries of one Component only. The entries of a Component are taken
  in the order the agent gives, and cut into ceil(count / 60) runs of consecutive entries, the sizes
  of any two runs differing by at most one: 125 entries make three groups of 42, 42 and 41. Count
  the entries, and name each group by its first and last entry.
- **Calibration.** Pick a shared sample of about five entries from different Components, and add it
  to every group, beyond its bound. Agents dispatched together do not read a rule alike. When the
  drafts are in, compare the verdicts on the sample. Where they differ and one follows the rule's
  text, settle the reading, state it in the run's commit message, and read again yourself, in every
  group, the drafts the difference bears on, before any draft is sorted. Where the rule's text admits
  both readings, the difference goes on the owner list.
- **The brief.** One agent per group writes one draft per entry. Its brief names the commit
  audited, its group's first and last entry, the sample's entries and its scratch directory.
- **The sort** also reads a sample of the drafts that find their entry conforming, at least one per
  group, and a draft found wrong sends the same agent's other drafts that judge the same rule back
  to a reading.
- **Application.** A split, a merge or a move of a head is drafted first, like any other finding: a
  fold judged alone is the edit most often undone. Each head a split, a rename or a merge creates is
  then judged against every rule as an entry of its own, before the commit: moving text unchanged
  does not make the new head conform.
- **The re-check** runs over the same groups.

**The outcomes of this axis:**

| outcome | the findings |
| --- | --- |
| applied in the branch | a rewording; the tense; a missing or wrong reference; an approval cited as a head's ground, removed when the head stands on its argument; a split that touches no word of the owner; a rejected alternative repaired in place, its reason made checkable or its marker set; a tripwire or an issue made stale by an edit of the run, repaired in place |
| put to the owner, on the owner list | a widening of what the owner's words approved, such as a title moved from the members the owner named to the rule; the removal of a head that fails every entry test; the removal of a rejected alternative that fails every recording test; a ground that is the owner's words; a head in the wrong Component |
| an issue entry | a head false of the code, kind `defect`; a split or a move the owner approved that is too large for the branch, kind `design` |

A tripwire or an issue is edited only where an edit of the run makes it stale. An issue whose "What
would close it" an edit of the run does is closed in that commit.

## The agentic-workflow axis {{slug:agentic-workflow-axis}}

**The corpus**: every text the harness delivers to a session as an instruction: the project's root
`CLAUDE.md` and the primer it imports, the installed skills and agents, and the project's own
skills and agents. The scoped `CLAUDE.md` files next to the code are not in it, nor are the
registers. The workflow functions as a whole, and its main defect is two texts a session cannot both
obey, so every agent reads the whole corpus.

**The rules**: a session can follow every instruction. A finding is admitted when a reading confirms
it, in the classes `agent@knowledge-architect-workflow-auditor` defines: a contradiction, a broken
trigger, a factual error, two readings, a provable gap. A gap that only predicts what an agent would
do is a predicted gap: it becomes an issue, never an edit, since an instruction about how an agent
behaves is added on evidence from real use. A case no instruction covers and judgement can decide
is no finding, per `primer@room-to-judge`.

**Not in this axis**: how the owner works, which the retrospective judges against the expectation
sets; the checker's own behaviour, whose code is not in the corpus; the scoped `CLAUDE.md` files.

**The agent**: `agent@knowledge-architect-workflow-auditor`.

**The dispatch**: each agent takes one lens, and a lens's row gives its default count. Each lens is
dispatched at least once, and L2 keeps one agent per group, since a group no agent walks is not
read. The count is the session's, within the budget the owner names, and the table's total is the
default; a smaller budget drops the duplicates of the other lenses first. A budget too small for
one agent per lens and per L2 group is said to the owner, who names more agents or the lenses to
drop. Agents with the same lens find different things, so a
duplicate adds coverage. There is no calibration sample: the sort reads every finding, and the
re-check reads again. An activity walk's brief names one group of activities; a project's own skill
is walked in the group of the activity it serves.

| lens | looks for | default agents |
| --- | --- | --- |
| L1 contradiction | two instructions that no single move satisfies; a term defined two ways | 3 |
| L2 activity walk | one activity followed across files, the instructions that apply at each moment | 4, one per group: design, planning, decision recording and issue tracking; development, review and merge; audit and retrospective; setup, pin move, goal setting and agent configuration |
| L3 two readings | an instruction whose words allow two readings that lead to different acts | 2 |
| L4 provable gap | a requirement the corpus states, and an input no text supplies | 2 |
| L5 restatement | a restatement that drifted from its home, or that lost its pointer | 1 |
| L6 local against installed | the project's own text against the installed text | 2 |
| L7 checker rules | each rule the checker enforces on what an agent writes, against the installed text alone | 2 |

**The sort** merges a finding two agents report, and notes it. It splits a finding tagged `I` into
its installed side and its project side, each a finding with one outcome. A draft found wrong
sends the same agent's other drafts of the same class back to a reading. A violation left, at the
re-check, is a finding a reading confirms, that is not on the kept list, and that no issue entry of
the run records. Each pass's commit message gives the count of confirmed findings and of those that
went on the owner list, so that the passes show whether the run converges.

**The outcomes of this axis**, for a finding on the project's own text:

| outcome | the findings |
| --- | --- |
| applied in the branch | a rewording that changes no instruction, a two-reading instruction among them where a design head or another instruction states the reading meant; a broken trigger whose target exists under another name; a pointer added to a restatement without one, or to a project text that repeats an installed one; a factual error; a drifted restatement brought back to its home, which converts it to a pointer when it is longer than one |
| put to the owner, on the owner list | which side of a contradiction wins, a loop with no exit among them; a broken trigger or a two-reading instruction whose repair changes what agents are told, which a two-reading instruction is when nothing recorded states the reading meant; a provable gap, since filling it adds an instruction |
| an issue entry | a predicted gap; a finding the owner defers |

**Sent upstream**: every finding on the installed text, over every pass of the run, goes into one
file per run, written when the run stops, in the shape of the retrospective's workflow file, each
finding numbered `W<n>`: the version of the checker used, what was audited at which commit, and the
findings, with no standing questions. It is named
`<YYYY-MM-DD>-<project>-workflow-audit-klarch-workflow.md`, and handled as
`skill@knowledge-architect-retrospective@two-files` and
`skill@knowledge-architect-retrospective@what-becomes-of-files` say for that file: it is written
outside the project, and nothing of it leaves the machine before the owner has read it.
