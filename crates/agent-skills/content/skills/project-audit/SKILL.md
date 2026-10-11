---
name: knowledge-architect-project-audit
description: MUST use when the owner asks to audit one aspect of the whole project rather than the work of one branch; when a pin move crosses a version whose changelog has a Migration entry citing an axis of this skill; and when an adopting project's existing documents have been moved into the workflow's homes. Covers what a project audit is, its method (an axis's saved workflow, whose one run per pass delivers a report the session verifies, or, for an axis with none, the corpus, a dispatch each axis gives, drafts in files and the sort; then one list for the owner, the application, the re-check, the review), the outcome of each kind of finding, where the record of an audit goes, and one section per axis: the design-record axis, which re-applies the rules on design heads and rejected alternatives to every entry, and the agentic-workflow axis, which reads every instruction the harness delivers as one whole for instructions a session cannot follow.
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
| **the owner list** | every finding of one pass that needs the owner's word, in the pass's report file, and the one message that points the owner to it |
| **the re-check** | fresh agents of the axis, dispatched as its first dispatch was, after the edits |
| **pass** | one round of the method. For an axis with a saved workflow: one run of it, the session's verification of its report, the owner list and the application. For an axis with none: the drafts, the sort, the owner list and the application. The re-check starts the next pass |
| **run** | one invocation of an axis's saved workflow, which covers one pass |
| **audit** | the passes on one axis, from the first until the owner stops them |

## When an audit runs {{slug:when-an-audit-runs}}

- **The owner asks for one**, naming the axis. This is the main occasion.
- **A pin move crosses a version whose changelog has a Migration entry citing an axis's section of
  this skill.** The axis runs after the pin's commit, in a branch of its own, as
  `skill@knowledge-architect-setup@moving-the-pin` says. A version that changes the rules an axis
  applies names the axis in its Migration entry, so a project brings its existing record to the
  new rules.
- **An adopting project's existing documents have moved** into the workflow's homes, as
  `skill@knowledge-architect-setup@existing-documentation` says: the move's work ends by
  running the design-record axis over its whole corpus.

An audit runs in a branch of its own, unless the owner gives it to another work's branch.

## The method {{slug:audit-method}}

Each point leaves judgement to the session, and an axis's section adjusts it where its corpus
needs.

**An axis with a saved workflow** runs each pass as one run of it, named in its section, with the
`args` its section lists. The run dispatches every agent of the pass, verifies their drafts and
delivers a report; the session reads nothing between its steps. The session then verifies the
report: it reads it whole, every repair proposal in full, every finding the report marks as bearing
on a ruling of the owner, on a tripwire or as plausible, against its draft, and a sample of the refuted drafts it
chooses. Then come points 6 and 7 below, and the next run is the re-check of point 8; point 9
follows once the audit stops. A session
whose harness does not list the workflow dispatches its steps by hand, in the script's order, with
the script's prompts, read from the script.

**An axis with no saved workflow** runs the points below. Points 3, 6, 7, 8 and 9 hold for both.

1. **The corpus.** The axis's section names its corpus and what is read beside it.
2. **The dispatch.** Dispatch the axis's agents as its section says, all in parallel, each with a
   scratch directory and a worktree of its own, as
   `skill@knowledge-architect-review@review-invariants` says.
3. **Pre-commitments.** An axis that measures, one whose audit yields a figure a conclusion rests
   on, states before the audit which figure would show which conclusion, and the owner sees them
   before the audit. An axis that only applies rules owes none, and the count of violations left
   per pass, which point 8 reads, makes no axis one that measures.
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
6. **The owner list.** The items of one pass, in the pass's report file of
   `skill@knowledge-architect-project-audit@record-of-an-audit`, grouped by kind, each under a
   label, `F<n>`, numbered as `primer@labelled-items` says, with a default and the words of the
   entry or the text the item turns on, and ending with a `Default` line and an `Outcome` line. One
   message gives the owner the file's full path and one line per item, and proposes that the owner
   rules by writing each `Outcome` line; a ruling given in the conversation is copied into the file
   verbatim. Apply nothing on the list before the owner answers. An item the owner defers becomes
   an issue entry.
7. **Application.** Apply the edits, yourself or through agents dispatched on disjoint files. Read
   the whole diff before each commit.
8. **The re-check.** Dispatch fresh agents of the axis, as its first dispatch was, after the edits,
   each also given the entries, the texts and the rules the owner ruled to keep, as a file in its
   scratch directory, and briefed on the edited files that carry its standard as
   `skill@knowledge-architect-review@review-invariants` says. A finding of the re-check that a
   reading confirms, that is not on the kept list, and that no issue entry records, is a violation
   left, whether a draft missed it or an edit made it: repair it or put it to the owner. The
   re-check is the next pass, and gets its findings as the axis's shape says. The audit stops when
   the owner judges what is left small enough. The violations left then take their outcomes, as
   any pass's findings do, and no re-check follows; one the owner does not rule on becomes an issue
   entry, as a deferred item does. **Each violation left is on old text or on new text**: old text
   is text the previous pass also read, whose words no commit changed after the commit that pass
   audited, a move or a reflow leaving it old; new text is text a later commit wrote, the previous
   pass's repairs among them. The axis's section says how its findings are classified. The count
   on old text is expected to fall from one pass to the next; a pass where it does not goes to the
   owner as a question, with the counts, and starts no new pass on its own. A violation left on new
   text takes its outcome like any other, and decides nothing about convergence.
9. **Review.** Once the audit stops, the audit's branch is reviewed before its merge under
   `skill@knowledge-architect-review`, on the axes `skill@knowledge-architect-review@review-axes`
   says that review owes. Its briefs name each report file the audit's branch added and deleted,
   with the commit that holds it, since a reviewer of the branch's tip does not see it.

## Outcomes {{slug:outcomes}}

Every finding takes exactly one outcome. A finding that fits no named case goes on the owner list.

| outcome | the findings |
| --- | --- |
| applied in the branch | the edits the axis's section lists as applied |
| put to the owner, on the owner list | the edits the axis's section lists as the owner's; a verdict where a rule reads two ways; a conflict with a goal, which then follows `skill@knowledge-architect-goal-setting`; any finding no row names |
| sent upstream | a finding on the installed text, which the project never edits: the axis's section says how |
| an issue entry | a finding outside the axis that the primer's table routes to an issue, such as a statement false of the code when the axis does not read the code; work the owner approved that is too large for the branch; a finding the owner defers. The kind follows `skill@knowledge-architect-issue-tracking` |
| nothing | the entry or the text conforms; a finding an issue entry already records; a finding on the kept list |

A finding outside the axis takes the first case of `primer@met-outside-the-task` that applies.

## The record of an audit {{slug:record-of-an-audit}}

%% The report file takes the shape of the analysis of a received file: committed with its rulings,
%% deleted once acted on, so history keeps both. Commit messages alone keep the outcomes and lose
%% the evidence.
**Each pass's report file holds its evidence and the owner's rulings, and the commit messages hold
its counts.** The file is `docs/audit-reports/<YYYY-MM-DD>-<axis>-pass-<n>.md` at the project's
root, the axis named by its section's slug. The session writes it after its reading of the pass:
for an axis with a saved workflow, the run's report as the session's verification left it; for an
axis with none, the drafts read at the sort, one section per finding with its evidence and its
proposed edit. The owner list closes it, as point 6 says. A span the checker would refuse is quoted
in words, since the file is checked like any other. The file is committed with its `Outcome` lines
before the pass's edits are applied; the commit that carries out its last outcome, or a later
commit of the same branch, deletes it, and its message names it by its path.

Each commit of an audit says what was audited, at which commit, named by its subject since a
rebase merge gives the audit branch's commits new SHAs, how it was dispatched, the calibration
reading settled where the axis has one, the outcome of each finding in a line, and the path of the
pass's report file. A figure the audit measured goes there too, and each pass's message gives the
count of violations left on old text and on new text, as point 8 defines them, and of those on old
text that went on the owner list, so that the passes show whether the audit converges.

## Repairs to agent-facing text {{slug:repair-standard}}

What any repair of an instruction is judged by, whether a reading or a real session found its
defect.

**The repair by cause.** A finding is repaired at the cause that produced it, not at its site: a
repair written for one finding adds a rule, and added rules interact at the next pass. A cause is
one of four kinds:

- **two homes for one moment**: instructions about one moment or one kind of act stated in more
  than one text;
- **a structure the work did not need**: a fixed sequence, count or mapping that another
  instruction needs to vary;
- **a restatement that drifted from its home**;
- **a rule written for one interaction**: a narrow rule where judgement already decides.

A finding that fits none is a cause of its own, with no kind. A repair is the homes merged into one
and the others pointing to it, the structure removed, the restatement brought back to a pointer, or
the narrow rule deleted and the case left to judgement. It adds an instruction only where none of
these closes the cause. **An instruction** is a sentence that tells an agent to do, or not to do,
an act at a moment; a rewording that makes an existing instruction's intent clearer adds none,
whatever its length. Each repair counts the instructions it adds and removes, and names the nearest
rival to itself and the fact that decides between them. **When the repairs applied together add
more instructions than they remove, the owner is told so first, with the net figure**, and rules on
that growth before any of them is applied.

**The record check.** Each repair is read against the record of what it edits before the owner
rules on it: the design heads that `{{command}} show` and a search of the design homes return for
the behaviour the edited text describes, the goals, and the rejected alternatives of the Component
that owns the text. The check names each head the repair would reverse, each rejected alternative
it brings back and each goal it strains, with its quotation. A repair that reverses a head is a
decision, recorded as `skill@knowledge-architect-decision-recording` says.

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
  in the order the agent gives, and cut into ceil(count / 60) groups of consecutive entries, the
  sizes of any two groups differing by at most one: 125 entries make three groups of 42, 42 and 41.
  Count the entries, and name each group by its first and last entry.
- **Calibration.** Pick a shared sample of about five entries from different Components, and add it
  to every group, beyond its bound. Agents dispatched together do not read a rule alike. When the
  drafts are in, compare the verdicts on the sample. Where they differ and one follows the rule's
  text, settle the reading, state it in the pass's commit message, and read again yourself, in every
  group, the drafts the difference bears on, before any draft is sorted. Where the rule's text admits
  both readings, the difference goes on the owner list.
- **The brief.** One agent per group writes one draft per entry. Its brief names the commit
  audited, its group's first and last entry, the sample's entries, its scratch directory and its
  worktree.
- **The sort** also reads a sample of the drafts that find their entry conforming, at least one per
  group, and a draft found wrong sends the same agent's other drafts that judge the same rule back
  to a reading.
- **Application.** A split, a merge or a move of a head is drafted first, like any other finding: a
  fold judged alone is the edit most often undone. Each head a split, a rename or a merge creates is
  then judged against every rule as an entry of its own, before the commit: moving text unchanged
  does not make the new head conform.
- **The re-check** runs over the same groups. An entry is on new text when a commit after the one
  the previous pass audited changed its words; the session classifies each finding at the sort.

**The outcomes of this axis:**

| outcome | the findings |
| --- | --- |
| applied in the branch | a rewording; the tense; a missing or wrong reference; an approval cited as a head's ground, removed when the head stands on its argument; a split that touches no word of the owner; a rejected alternative repaired in place, its reason made checkable or its marker set; a tripwire or an issue made stale by an edit of the audit, repaired in place |
| put to the owner, on the owner list | a widening of what the owner's words approved, such as a title moved from the members the owner named to the rule; the removal of a head that fails every entry test; the removal of a rejected alternative that fails every recording test; a ground that is the owner's words; a head in the wrong Component |
| an issue entry | a head false of the code, kind `defect`; a split or a move the owner approved that is too large for the branch, kind `design` |

A tripwire or an issue is edited only where an edit of the audit makes it stale. An issue whose
"What would close it" an edit of the audit does is closed in that commit.

## The agentic-workflow axis {{slug:agentic-workflow-axis}}

**The corpus**: every text the harness delivers to a session as an instruction, installed or the
project's own: the project's root `CLAUDE.md` and the primer it imports, and every other such text
in the project's agent configuration directory. The scoped `CLAUDE.md` files next to the code are
not in it, nor are the registers. The workflow functions as a whole, and its main defect is two
texts a session cannot both obey, so every agent reads the whole corpus.

**The rules**: a session can follow every instruction. A finding is admitted when a reading confirms
it, in the classes `agent@knowledge-architect-workflow-auditor` defines: a contradiction, a broken
trigger, a factual error, two readings, a provable gap, divergent verdicts, a restatement or a
pointer its home does not allow. A gap that only predicts
what an agent would do is a predicted gap, and is never repaired by an edit, since an instruction about how an agent
behaves is added on evidence from real use. A case no instruction covers and judgement can decide
is no finding, per `primer@room-to-judge`.

**Not in this axis**: how the owner works, which the retrospective judges against the expectation
sets; the checker's own behaviour, whose code is not in the corpus; the scoped `CLAUDE.md` files.

**The agent**: `agent@knowledge-architect-workflow-auditor`.

**The saved workflow**: `knowledge-architect-agentic-workflow-audit`. One run of it is one pass:
a preflight sets up a worktree per auditor and the listings the agents read; the auditors draft,
one per lens and count; one verifier per auditor tries to refute each draft; a dedup writes the
confirmed-findings file, each finding classified as on old or new text; cluster agents group the
findings by cause and a merge writes the causes file; repair agents propose each cause's repair, as
"The repair by cause" says; a check agent reads each proposal; a cross-check reads the proposals
and their checks together; the report gathers all of it. Its `args`:

| arg | what it is |
| --- | --- |
| `commit` | the commit audited |
| `scratch` | the run's scratch directory, under which every step and agent writes |
| `worktrees` | the directory under which the preflight creates the worktrees, outside the project or under a path its ignore rules cover |
| `history` | a git range whose commit messages, and the report files it added, record the audit's earlier passes, for the verifiers |
| `previous` | for a re-check, the commit the previous pass audited, against which each finding is classified as on old or new text |
| `kept` | for a re-check, the path of the file of entries, texts and rules the owner ruled to keep |
| `counts` | optional: the auditors per lens, keyed L1 to L8, `clusters` and `repairsPerCause`; a key left out takes its default, written in the script |
| `from` | optional: `verify`, `cluster` or `repairs`, to restart a run from the files it already wrote |

**The lenses**, each described in `agent@knowledge-architect-workflow-auditor@the-lenses`: L1
contradiction, L2 activity walk, L3 two readings, L4 provable gap, L5 restatement, L6 local against
installed, L7 checker rules, L8 verdict alignment. Each lens is dispatched at least once; L2's walkers
take its groups of activities in turn, and a group no walker takes is named in the report. Agents
with the same lens find different things, so a duplicate adds coverage. A project's own skill is
walked in the group of the activity it serves.

**The repair by cause**, as `skill@knowledge-architect-project-audit@repair-standard` says. The
repair step proposes one repair per cause, never one per finding, and a draft's proposed repair is
one of its inputs; where `counts` gives a cause several repair agents, their proposals are
alternatives for that one repair. **The check**: one workflow auditor per proposal, under its check
brief, reads the proposal's new text in the corpus through L1 and L3, makes the record check of
that section, and writes its check beside the proposal. The session's verification reads each
proposal with its check against its cause and the corpus, and corrects it or sends it back to a
repair agent of its cause, briefed with the script's repair prompt and with the check.

**Old and new text**, for a re-check: for each line that carries a confirmed finding's defect, the
dedup runs `git blame -w -C` at the commit audited. A line whose blamed commit is not an ancestor
of `previous` is on new text only where its phrase, whitespace-normalised, is absent from every
file of the corpus at `previous`; a finding on two texts takes the newer.

**The tripwires**: the preflight saves the output of `{{command}} tripwires` too, and a verifier
marks `bears-on-tripwire` a finding that carries a tripwire's firing evidence, after reading that
tripwire whole, without settling it. The session judges whether each fired, and a fired tripwire
moves as `skill@knowledge-architect-issue-tracking@movement-instruction` says.

A cause's findings on the installed text are sent upstream, as below. Its findings on the
project's own text take the outcome its repair's edits on that text take in the table below:
applied in the branch when every such edit falls in the applied row, the owner list otherwise. A
finding marked known or kept, and a predicted gap, take instead the outcome
`skill@knowledge-architect-project-audit@outcomes` and the table below give them. The
cross-check proposes each cause's outcome, and the session decides it. An item of the owner list is
one cause, with the ids of its findings, the repair, its count and a default. The owner list states
the net growth of the pass's repairs in the project first, where they add more instructions than
they remove, as `skill@knowledge-architect-project-audit@repair-standard` says. Each pass's commit
message also gives the corpus's word count before and after the pass, `wc -w` over the corpus's
files.

**The outcomes of this axis**, for a finding on the project's own text:

| outcome | the findings |
| --- | --- |
| applied in the branch | a rewording that changes no instruction, a two-reading instruction among them where a design head or another instruction states the reading meant; a broken trigger whose target exists under another name; a pointer added to a restatement without one, or to a project text that repeats an installed one; a factual error; a drifted restatement brought back to its home, which converts it to a pointer when it is longer than one |
| put to the owner, on the owner list | which side of a contradiction wins, a loop with no exit among them; which standard wins, for divergent verdicts; a broken trigger or a two-reading instruction whose repair changes what agents are told, which a two-reading instruction is when nothing recorded states the reading meant; a provable gap, since filling it adds an instruction |
| an issue entry | a predicted gap; a finding the owner defers |

**Sent upstream**: every finding on the installed text, over every pass of the audit, a predicted
gap included, goes into one file per audit, written when the audit stops from the report files
of its passes, which history keeps. That file is the
workflow's file of `skill@knowledge-architect-retrospective@two-files`, named, written and handled
as that section and `skill@knowledge-architect-retrospective@what-becomes-of-files` say, its
subject naming the audit. It differs in its content only: what was audited at which commit in
place of what the session did, each finding numbered `W<n>`, and no standing answers.

**Before a run**: the session writes the kept list, for a re-check, and names the history. A harness
may list saved workflows only when a session starts, so a session that ran the install later may
not see the workflow listed. The worktrees are removed once the pass's findings have their
outcomes, as `skill@knowledge-architect-review@review-invariants` says.
