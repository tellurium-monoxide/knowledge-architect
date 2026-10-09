---
name: knowledge-architect-project-audit
description: MUST use when the owner asks to audit one aspect of the whole project rather than the work of one branch; when a pin move crosses a version whose changelog has a Migration entry citing an axis of this skill; and when an adopting project's existing documents have been moved into the workflow's homes. Covers what a project audit is, its method (the corpus, bounded groups, calibration, one draft per entry, the sort, one list for the owner, the application, a fresh re-check, the review), the outcome of each kind of finding, where the record of a run goes, and one section per axis, the first the design-record axis, which re-applies the rules on design heads and rejected alternatives to every entry.
---

# Auditing a project

Scope: reading one aspect of a whole project, every entry of a corpus, against the rules that
apply to it, and bringing what fails into line. A project audit is the reading of the whole record
on one axis; a review reads a diff. One activity: its artifact is a repaired record across the
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
| **corpus** | the entries an axis reads, all of them |
| **group** | a run of consecutive entries of the corpus that one agent reads whole |
| **draft** | one agent's file for one entry: its verdict, the evidence, a proposed edit and a proposed outcome |
| **the owner list** | one message to the owner holding every finding that needs the owner's word |
| **the re-check** | fresh agents over the same groups, after the edits |

## When an audit runs {{slug:when-an-audit-runs}}

- **The owner asks for one**, naming the axis. This is the main occasion.
- **A pin move crosses a version whose changelog has a Migration entry citing an axis's section of
  this skill.** The axis runs after the pin's commit, in a branch of its own, as
  `skill@knowledge-architect-setup@moving-the-pin` says. A version that changes the rules an axis
  applies names the axis in its Migration entry, so a project brings its existing record to the
  new rules.
- **An adopting project's existing documents have moved** into the workflow's homes, as
  `skill@knowledge-architect-setup@existing-documentation` says: the milestone of the move ends by
  running the design-record axis over what it wrote.

An audit runs in a branch of its own, unless the owner gives it to another work's branch.

## The method {{slug:audit-method}}

Each point leaves judgement to the session, and an axis's section adjusts it where its corpus
needs.

1. **The corpus.** The axis's section names its corpus and the inputs read beside each entry.
2. **Groups.** A group holds the entries of one Component only, unless the axis says otherwise.
   The entries of a Component are taken in a fixed order, which the axis's section gives, and cut
   into ceil(count / 60) runs of consecutive entries, the sizes of any two runs differing by at
   most one: 125 entries make three groups of 42, 42 and 41. Count the entries, and name each group
   by its first and last entry.
3. **Pre-commitments.** An axis that measures, one whose run yields a figure a conclusion rests on,
   states before the run which figure would show which conclusion, and the owner sees them before
   the run. An axis that only applies rules owes none.
4. **Calibration.** Pick a shared sample of about five entries from different Components, and add
   it to every group, beyond its bound. Agents dispatched together do not read a rule alike. When
   the drafts are in, compare the verdicts on the sample. Where they differ and one follows the
   rule's text, settle the reading, state it in the run's commit message, and read again yourself,
   in every group, the drafts the difference bears on, before any draft is sorted. Where the rule's
   text admits both readings, the difference goes on the owner list.
5. **Drafts.** Dispatch the axis's agent once per group, all in parallel, each with a scratch
   directory of its own. Each writes one draft per entry there and edits nothing else. Its brief
   names the commit audited, its group's first and last entry, the sample's entries and its scratch
   directory. Results stay in the files: the session reads the drafts, not a summary of them.
6. **The sort.** Read every draft whose proposed outcome is not "nothing" against its entry and the
   history it cites, and a sample of the drafts that find their entry conforming, at least one per
   group. Sort each finding into its outcome, under
   `skill@knowledge-architect-project-audit@outcomes`. A draft found wrong is corrected, and the
   group's other drafts of the same kind are read again. Nothing an agent proposes is applied
   before a reading confirms it.
%% The `F` label: `design@agent-skills@ruled-items-labelled`.
7. **The owner list.** One message: the items grouped by kind, each under a label, `F<n>`, with a
   default and the entry's words the item turns on. Apply nothing on the list before the owner
   answers. An item the owner defers becomes an issue entry.
8. **Application.** Apply the edits, yourself or through agents dispatched on disjoint files. Read
   the whole diff before each commit. A split, a merge or a move of a head is drafted first, like
   any other finding: a fold judged alone is the edit most often undone.
9. **The re-check.** Dispatch fresh agents of the axis over the same groups, after the edits, each
   briefed as in point 5 and also given the entries and rules the owner ruled to keep, as a file in
   its scratch directory. A draft that finds a rule failed, and is not on the kept list, is a
   violation left, whether a draft missed it or an edit made it. Read each one against its entry,
   then repair it or put it to the owner.
10. **Review.** The audit's branch is reviewed under `skill@knowledge-architect-review`, on these
    axes: self-consistency, fidelity of relocation, routing of knowledge, decision recording,
    standing state, and the transcript review last.

## Outcomes {{slug:outcomes}}

Every finding takes exactly one outcome. A finding that fits no named case goes on the owner list.

| outcome | the findings |
| --- | --- |
| applied in the branch | the edits the axis's section lists as applied |
| put to the owner, on the owner list | the edits the axis's section lists as the owner's; a verdict where a rule reads two ways; a conflict with a goal, which then follows `skill@knowledge-architect-goal-setting`; any finding no row names |
| an issue entry | a finding outside the axis, such as a statement false of the code when the axis does not read the code; work the owner approved that is too large for the branch; a finding the owner defers. The kind follows `skill@knowledge-architect-issue-tracking` |
| nothing | the entry conforms |

## The record of a run {{slug:record-of-a-run}}

**The commit messages are the record.** Each commit of a run says what was audited, at which
commit, how many entries and groups, the calibration reading settled, and the outcome of every
finding: applied, ruled by the owner under its label, opened as an issue with its id, or found to
need nothing. A figure the run measured goes there too. A finding whose only record is a scratch
file, or a plan document that will leave, is lost when that file goes.

A run that changes no file leaves no commit that changes a file, and a merge that drops such a
commit, as a rebase merge does, would drop its record. Its record is then the report to the
owner.

## The design-record axis {{slug:design-record-axis}}

**The corpus**: every design head of every Component, and every entry of its rejected
alternatives.

**The order of a Component's entries**: its design home first, a directory-shaped home's
subdocuments in the order its README links them, each head in file order; then its rejected
alternatives in file order, each entry as the file shapes it, a heading or a paragraph that opens
with the alternative's name in bold. `{{command}} model` prints every heading and slug definition
with its file and line, which lists the heads.

**The rules**: every rule of `primer@design-heads` for a head, and every rule of
`skill@knowledge-architect-decision-recording@losing-alternatives` for a rejected alternative.

**The inputs**, for each head: every tripwire guarding it and every issue naming it, read with
`{{command}} show <ref>`; and the history behind each citation of the owner, the commit messages
that touch its slug and the plan documents that carried it, found as
`skill@knowledge-architect-decision-recording@three-homes` shows. A verdict that a ground is or is
not the owner's rests on that history, never on the head's text alone.

**Not in this axis**: whether a head is true of the code, and the restatements of a head in other
texts. A head found false of the code gets an issue entry, kind `defect`.

**The agent**: `agent@knowledge-architect-design-record-auditor`, dispatched once per group, as its
description says.

**The outcomes of this axis:**

| outcome | the findings |
| --- | --- |
| applied in the branch | a rewording; the tense; a missing or wrong reference; an approval cited as a head's ground, removed when the head stands on its argument; a split that touches no word of the owner; a rejected alternative repaired in place, its reason made checkable or its marker set; a tripwire or an issue made stale by an edit of the run, repaired in place |
| put to the owner, on the owner list | a widening of what the owner's words approved, such as a title moved from the members the owner named to the rule; the removal of a head that fails every entry test; the removal of a rejected alternative that fails every recording test; a ground that is the owner's words; a head in the wrong Component |
| an issue entry | a head false of the code, kind `defect`; a split or a move the owner approved that is too large for the branch, kind `design` |

A tripwire or an issue is edited only where an edit of the run makes it stale. An issue whose "What
would close it" an edit of the run does is closed in that commit.
