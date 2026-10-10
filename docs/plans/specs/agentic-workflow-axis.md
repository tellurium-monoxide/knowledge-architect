# A project audit reads the agentic workflow as one whole, with auditors of varied tasks that each read all of it

## Status and audience

This spec is the plan of the work that gives the project audit its second axis, the agentic-workflow
axis, and runs it on this repository. The work does four things:

- the method of `skill@knowledge-architect-project-audit` becomes generic, and the dispatch of the
  design-record axis, its groups and its calibration sample, moves into that axis's section;
- a new installed agent, the workflow auditor, holds the standard of the new axis;
- a new section of the audit skill holds the axis: its corpus, its lenses, its finding standard and
  its outcomes;
- the axis runs on this repository, in this branch, which judges AC1 and AC4 and counts T1 to T4.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec and its work land on one branch**, `agentic-workflow-audit-axis`. No step changes what
  the per-commit gate checks: the work changes installed text and the record, not the checker.
- **The design audit** of `skill@knowledge-architect-planning@working-a-slice`, point 2, is owed when
  the work does not start in the session where the discussion converged, or when commits other than
  this spec's own land on main before the work starts.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22.jsonl`.
  The discussion begins at the owner's message that opens "The last piece of work that landed was
  related to "project audits"". Find the file by that opening message, not by its name. Below,
  "round 1" is that message and the agent's reply that opens "This is round 1"; "round 2" is the
  owner's message that opens "Agreed on your readings of additions-need-real-use" and the agent's
  reply that opens "All seven threads are now closed by your word"; "round 3" is the owner's message
  that opens "record all four, apply the three acceptance criteria". The agent's reply to round 3
  worded AC4 on screen, but the transcript keeps only a one-line summary of it, so AC4's wording
  is put to the owner as D1. "The review message" is the agent's message that opens "Two of my
  premises were wrong", which put D1 to D7 to the owner. "Round 4" is the owner's reply to it, which
  opens "All defaults approved." and proposes a lens, and the agent's reply that opens "D1 to D7 are
  now ruled"; "round 5" is the owner's message "checker-rules-lens approved, go ahead". "Round 6" is the owner's
  reply to the re-review message, which opens "All defaults approved.". An extraction agent wrote the owner messages,
  the agent messages, the tables and the arguments verbatim to scratch; the session assembled this
  spec from that file and its own context.
- **Labels.** The discussion used T1 to T4 and AC1 to AC4. No other discussion in the transcript
  used either prefix. The tripwires home already holds T labels of other discussions, each
  opening "T<n> of the premortem of the discussion that made the decision"; the new ones take the
  same form, and the decision each guards tells them apart.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **the audit skill**: `skill@knowledge-architect-project-audit`, whose source is
  `path@agent-skills@content/skills/project-audit/SKILL.md`.
- **the design-record axis**: the audit skill's first axis,
  `skill@knowledge-architect-project-audit@design-record-axis`, carried by
  `agent@knowledge-architect-design-record-auditor`.
- **the agentic-workflow axis**: the axis this work builds. The owner's name, round 1: "the axis
  related to agentic workflow". Its section of the audit skill is new, per New names.
- **the corpus of the axis**: every text the harness delivers to a session as an instruction, per
  `thread@agentic-workflow-axis@workflow-axis-corpus`.
- **the workflow auditor**: the new installed agent, per New names.
- **a lens**: one task an auditor is briefed with, L1 to L7, per
  `thread@agentic-workflow-axis@workflow-axis-lenses`. The word is the agent's, round 1.
- **a draft**: one auditor's file for one finding, per "The workflow auditor".
- **a tag**: the side of a finding, `W` for the installed text, `P` for the project's own text, `I`
  for an interaction of the two. W and P are the letters of the retrospective's findings,
  `skill@knowledge-architect-retrospective@what-it-examines`. A tag classifies a finding; it is not a
  label of an item the owner rules on, so `design@agent-skills@ruled-items-labelled` gains no prefix.
- **the owner list**: as the audit skill's terms define it,
  `skill@knowledge-architect-project-audit@audit-terms`.
- **a pass**: one round of the method: the drafts, the sort, the owner list, the application. The
  re-check's drafts start the next pass. Today the skill uses the word in point 7 and point 9 and
  does not define it; step 1 adds it to the terms.
- **the re-check**: fresh agents of the axis, dispatched as the axis's first dispatch was, after a
  pass's edits. For this axis: the same lenses with the same count per lens as the first dispatch,
  and the same groups of activities for L2.
- **confirmed**: a finding that survives the session's reading at the sort.
- **a violation left**, for this axis: a confirmed finding of a re-check that is not on the kept
  list. It is what the generic stop rule counts.
- **severe**: a confirmed finding whose outcome is the owner list, whatever its class.
- **the owner's budget**: the number of agents the owner names when asking for the run; the lens
  table's total, 16, when the owner names none.

## What the work is

### What exists today

- `design@agent-skills@audit-method` states, in its title, "An audit is read in bounded groups by
  agents calibrated on a shared sample". The audit skill's method,
  `skill@knowledge-architect-project-audit@audit-method`, holds ten points. Points 2 (groups of at
  most 60 entries of one Component, in a fixed order), 4 (the calibration sample), 5 (the brief names
  the group's first and last entry and the sample), 6 (the sort reads at least one conforming draft
  per group, and a draft found wrong sends the group's other drafts of its kind back to a reading),
  8 (a split, a merge or a move of a head is drafted first, and each head a split, a rename or a
  merge creates is judged alone) and 9 (the re-check runs over the same groups) are specific to a
  corpus of entries. The method's preamble lets an axis's section adjust each point "where its
  corpus needs", and point 2 lets an axis hold several Components in a group; the head's title, the Terms table, the
  Scope paragraph, the Outcomes row "nothing", and "The record of a run" ("how many entries and
  groups, the calibration reading settled") state entries, groups and calibration for every audit.
- The design-record auditor's description says "To dispatch it, follow
  `skill@knowledge-architect-project-audit@audit-method`", and so does the head
  `design@agent-skills@audit-method` names that section as the method's home, in its first sentence.
- The audit skill has one axis section, `skill@knowledge-architect-project-audit@design-record-axis`.
- `agent@knowledge-architect-design-record-auditor` restates the group arithmetic in its description.
- `issue@agent-skills@audit-axes-beyond-the-design-record` lists the axes the owner named, among
  them "consistency of the project's own agent workflows", and the owner's ruling that the
  restatements of directives go in that axis.
- The corpus, measured with
  `git ls-files CLAUDE.md .claude | grep -v open-issues | xargs wc -w`, is 61,065 words at the
  spec's first commit, over 25 files, counted with `… | wc -l`. Round 1 said 26, counting the
  `total` line of `wc`.
- Three scoped CLAUDE.md files mark a restatement. One restates workflow directives,
  `path@agent-skills@CLAUDE.md`, the four tests for editing an installed skill.
  `path@gates@CLAUDE.md` and `path@xtask@CLAUDE.md` restate `design@gates@verdict-from-exit-codes`,
  a decision about code. Round 1 said all three restate workflow directives.
- The agent-config issue register, `path@agent-config@open-issues/`, holds no entry; the issues
  about the workflow are in the registers of several Components, agent-skills and core among them.

### The standing entries the work bears on

The grounding search of round 1 returned these, each read whole with `cargo klarch show`:

- `issue@agent-skills@audit-axes-beyond-the-design-record`: this work takes its agentic-workflow
  axis, and the harvest rewrites it to the axes it still holds.
- `tripwire@agent-skills@audit-defaults-overruled`: guards `design@agent-skills@audit-method`. The
  first run's owner list is counted against it.
- `tripwire@agent-skills@member-beyond-the-argument`: the new axis is a member of the audit heads;
  the decisions below are argued here so that none is recorded as a member the argument does not
  cover.
- `tripwire@agent-skills@expectation-set-closes-a-contradiction`: a contradiction the run finds is
  never closed by citing an expectation set.
- `tripwire@agent-skills@pointer-not-followed`: L5 repairs may turn restatements into pointers.
- `issue@core@a-home-for-developer-contracts-outside-agent-configuration`: it supports leaving the
  scoped CLAUDE.md files out of the corpus, and gains a line naming the one scoped restatement of workflow directives.
- `issue@core@configuration-for-several-agent-providers`: the shipped text names the corpus without
  a provider's directory.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the reason for AC3.
- `tripwire@agent-skills@structure-rejection-overruled`: a provable gap the run proposes to fill
  may be rejected by citing `design@agent-skills@capabilities-not-structure`. The tripwire fires on a
  review or a design discussion, which an audit's owner list is not; a rejection the owner overrules
  on the list is still reported in that pass's commit message, for the retrospective that re-enters
  the tripwire.
- `tripwire@agent-skills@axis-not-run-at-a-pin-move`: the new axis is one a Migration entry may
  cite; see Later consequences.
- `issue@agent-skills@expectation-sets-for-the-installed-skills`: most installed skills state no
  expectation set, so a finding about the owner's behaviour has no scope test there; the auditors
  report a finding about how the owner works as outside the axis.
- `issue@agent-skills@a-reviewer-s-running-time-is-unbounded`: sixteen agents reading the whole
  corpus may run long; the session asks an agent that has not returned for what it has, rather than
  waiting without bound.
- Several issues are findings the first run will meet already recorded, and need nothing of this
  work beyond being marked as known by the auditors:
  `issue@agent-skills@the-material-finding-duty-has-no-head`,
  `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review`,
  `issue@agent-skills@a-skill-for-creating-a-component`,
  `issue@agent-skills@patching-an-installed-skill`,
  `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`,
  `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`,
  `issue@agent-skills@the-retrospective-counts-no-review-cost`,
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`,
  `issue@knowledge-architect@a-mechanical-changelog-check`,
  `issue@xtask@where-ci-and-a-local-run-differ-is-not-only-a-flag`.
- `tripwire@agent-skills@head-created-without-deliberation` and
  `tripwire@agent-skills@search-missed-before-the-work` watch any design work; this spec is the
  deliberation, and the search ran at grounding. Nothing more is owed.

### What is outside the work

- The other axes the issue names. Each takes its own discussion, per
  `issue@agent-skills@audit-axes-beyond-the-design-record`.
- The scoped CLAUDE.md files, and their three restatements, per
  `thread@agentic-workflow-axis@workflow-axis-corpus`; their home is the subject of
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`.
- A mechanical check of the properties of a project's own skills: `issue@core@tooling-for-project-skills`.

## What is already decided

The design rests on these and does not argue them again:

- `design@agent-skills@audit-triggers`, `design@agent-skills@audit-agent-per-axis`,
  `design@agent-skills@audit-repairs-the-whole-record`.
- `design@agent-skills@synthetic-evidence-not-built`, read in round 1 and agreed by the owner, round
  2: "Agreed on your readings of additions-need-real-use and synthetic-evidence-not-built.", and
  read again for the first run per D2 and D9.
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@restatement-size-test`,
  `design@agent-skills@expectation-set-bounds-scope`.

The work rewrites two heads by its threads, three more per D4 to D6, and reads a sixth per D3:

- `design@agent-skills@additions-need-real-use`, per D4 and D11: its list of findings provable by
  reading names an instruction read two ways and a provable gap, and its first sentence, "Real use
  originates an addition", is scoped to an addition about how an agent behaves. Its citing texts,
  each read at the harvest against the rewritten head: the `%%` line of
  `path@agent-skills@content/skills/design/SKILL.md`;
  `issue@agent-skills@a-reviewer-s-running-time-is-unbounded`;
  `issue@agent-skills@expectation-sets-for-the-installed-skills`; the heads
  `design@agent-skills@expectation-set-bounds-scope`,
  `design@agent-skills@standing-entries-searched-before-the-work`,
  `design@agent-skills@audit-triggers`, `design@agent-skills@staged-check-before-each-commit` and
  `design@agent-skills@staged-forms-for-a-partial-commit`;
  `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`; and the restatement of the
  Necessity test in `path@agent-skills@CLAUDE.md`, which is rewritten with the head.
- `design@agent-skills@audit-outcomes`, per D5 and D14: an installed-text finding in a project that
  does not publish the workflow is sent upstream, and a predicted gap is an issue entry.
- `design@agent-skills@audit-is-an-activity`, per D6: a corpus holds entries or texts, and the method
  is generic with a dispatch per axis.
- `design@agent-skills@restatement-size-test` is not rewritten under D3's default: L5 is narrowed to
  meet it.
- Per D12, `design@knowledge-architect@retrospective-findings-stay-here` gains the audit's findings
  on the installed text, and its restatement in `instructions@repository-skills` says so; the
  routing table, which holds only project skills and agents, stays empty. Per D15,
  `design@agent-skills@ruled-items-labelled` widens its `W` row to an audit's upstream file.

The two rewritten by the threads:

- `design@agent-skills@audit-method`: rewritten to the generic method. Its citing texts, and what
  judges each:
  - `path@agent-skills@docs/design.md`, the head `design@agent-skills@audit-repairs-the-whole-record`:
    read at the harvest; it cites the method, which stays.
  - `issue@agent-skills@audit-axes-beyond-the-design-record`: rewritten at the harvest.
  - `path@agent-skills@docs/rejected-alternatives.md`, "An audit method with no calibration and no
    re-check": rewritten at the harvest to name both heads it lost to, the generic method for the
    re-check and `design@agent-skills@design-record-axis` for the calibration, per Losing
    alternatives. Its recorded reason holds two facts, the re-check finding what a pass missed or made and
    two agents reading one sample entry two ways; both stay.
  - `tripwire@agent-skills@audit-defaults-overruled`: its premise names "the calibration and the
    session's sort"; the new axis has no calibration, so the harvest rewrites the premise to the
    sort, and the calibration where an axis has one.
  - The head's body holds the argument for the bound of 60, the rival with no calibration and no
    re-check, and the owner's stop-rule quotation: the bound's argument moves to
    `design@agent-skills@design-record-axis`; the rival and the stop rule stay.
  - The design-record auditor's description points at
    `skill@knowledge-architect-project-audit@audit-method` for its dispatch: step 1 re-points it to
    `skill@knowledge-architect-project-audit@design-record-axis`.
- `design@agent-skills@design-record-axis`: gains the groups and the calibration sample. Its citing
  text, `tripwire@agent-skills@audit-edit-leaves-an-entry-stale`, is read at the harvest.

## Criteria

### The activity stays in the audit skill, with no new skill `##c1`

Binding, from `goal@agent-skills@one-skill-per-activity`. Met by
`thread@agentic-workflow-axis@audit-method-per-axis`.

### The shipped section and agent name no project's paths `##c2`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met by
`thread@agentic-workflow-axis@workflow-axis-corpus`.

### The method fixes no count where judgement would serve `##c3`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met by
`thread@agentic-workflow-axis@workflow-axis-lenses`: each lens at least once, the count the session's
within the owner's budget.

### Only findings provable by reading lead to an edit `##c4`

Binding as a presumption, from `design@agent-skills@additions-need-real-use`. Met by
`thread@agentic-workflow-axis@workflow-axis-finding-standard`.

### The axis has an installed agent of its own `##c5`

Binding as a presumption, from `design@agent-skills@audit-agent-per-axis`. Met by
`thread@agentic-workflow-axis@workflow-auditor-agent`.

### One pass costs about the owner's budget of 15 agents, in the owner's words `##c6`

Weighed, from the owner's words, round 1: "the budget should be around 15 subagents I think." Met by
`thread@agentic-workflow-axis@workflow-axis-lenses`, whose default is 16 agents per D8.

## Threads

Every thread but #checker-rules-lens was proposed by the agent in round 1 and approved by the owner
in round 2; #checker-rules-lens was proposed by the owner in round 4 and approved in round 5.

### The audit's method is generic, and each axis gives its own dispatch `##audit-method-per-axis`

Arguments: `argument@agentic-workflow-axis@a2`, `argument@agentic-workflow-axis@a10`,
`argument@agentic-workflow-axis@a17`, `argument@agentic-workflow-axis@a18`,
`argument@agentic-workflow-axis@a19`. Shape: Decided design, "The generic method". Harvest:
`design@agent-skills@audit-method` rewritten, `design@agent-skills@design-record-axis` gains the
dispatch. Closed, round 2: "audit-method-per-axis: I agree with this shape."

### The corpus is every text the harness delivers as an instruction, and the scoped CLAUDE.md files are not in it `##workflow-axis-corpus`

Arguments: `argument@agentic-workflow-axis@a4`, `argument@agentic-workflow-axis@a6`,
`argument@agentic-workflow-axis@a13`, `argument@agentic-workflow-axis@a14`,
`argument@agentic-workflow-axis@a15`, `argument@agentic-workflow-axis@a16`,
`argument@agentic-workflow-axis@a20`, `argument@agentic-workflow-axis@a21`. Shape: Decided design,
"The corpus". Harvest: a head, by entry test 2; the line on the developer-contracts issue. Closed,
round 2: "workflow-axis-corpus: approved." The default for the scoped restatements was part of
the thread as presented, and the agent's round-2 reply stated that the approval accepted it.

### Every auditor reads the whole corpus, through one lens, each lens dispatched at least once, with no calibration sample `##workflow-axis-lenses`

Arguments: `argument@agentic-workflow-axis@a2`, `argument@agentic-workflow-axis@a3`,
`argument@agentic-workflow-axis@a7`, `argument@agentic-workflow-axis@a8`,
`argument@agentic-workflow-axis@a13`, `argument@agentic-workflow-axis@a22`,
`argument@agentic-workflow-axis@a23`, `argument@agentic-workflow-axis@a24`,
`argument@agentic-workflow-axis@a25`. Shape: Decided design, "The lenses". Harvest: a head. Closed,
round 2: "workflow-axis-lenses: approved. This covers well what I had in mind."

### A finding leads to an edit only when a reading confirms it, and a predicted gap is an issue `##workflow-axis-finding-standard`

Arguments: `argument@agentic-workflow-axis@a9`, `argument@agentic-workflow-axis@a11`,
`argument@agentic-workflow-axis@a12`, `argument@agentic-workflow-axis@a21`,
`argument@agentic-workflow-axis@a26`. Shape: Decided design, "The finding standard". Harvest: a
head, by entry test 2. Closed, round 2:
"workflow-axis-finding-standard approved."

### A finding is tagged by the side whose text changes, and an installed-text finding in a consumer goes upstream `##workflow-axis-outcomes`

Arguments: `argument@agentic-workflow-axis@a5`, `argument@agentic-workflow-axis@a27`,
`argument@agentic-workflow-axis@a28`. Shape: Decided design, "The outcomes". Harvest: a head, if it
earns one. Closed, round 2: "workflow-axis-outcomes approved."

### One installed agent carries the axis, its lens chosen by the brief `##workflow-auditor-agent`

Arguments: `argument@agentic-workflow-axis@a29`, `argument@agentic-workflow-axis@a30`,
`argument@agentic-workflow-axis@a34`. Shape: Decided design, "The workflow auditor". Harvest: a
head, if it earns one beside `design@agent-skills@audit-agent-per-axis`. Closed, round 2:
"workflow-auditor-agent: agreed. I was worried about publishing too many agents and polluting the
directory. I think this shape works."

### An auditor starts from the checker's documented rules and reports each one the installed text does not deliver `##checker-rules-lens`

Proposed by the owner in round 4, argued by the agent in its reply. Arguments:
`argument@agentic-workflow-axis@a42`, `argument@agentic-workflow-axis@a43`,
`argument@agentic-workflow-axis@a44`, `argument@agentic-workflow-axis@a45`,
`argument@agentic-workflow-axis@a46`. It widens what an agent reads beside the corpus, which
`thread@agentic-workflow-axis@workflow-axis-corpus` fixed, for L7 only. Shape: Decided design, "The
checker-rules lens". Harvest: within the head of `thread@agentic-workflow-axis@workflow-axis-lenses`.
Closed, round 5: "checker-rules-lens approved, go ahead".

### The axis's first run is in the branch that builds it `##workflow-axis-first-run`

Arguments: `argument@agentic-workflow-axis@a1`, `argument@agentic-workflow-axis@a31`,
`argument@agentic-workflow-axis@a32`, `argument@agentic-workflow-axis@a33`. Shape: Implementation
sequence, step 3. Harvest: none; a choice of this work, carried by its commits. Closed, round 2:
"workflow-axis-first-run: agreed to run in the same branch."

## Arguments

### The axis matters most to this repository, whose product is in part the workflow `##a1`

Round 1, owner. Bears on the axis and `thread@agentic-workflow-axis@workflow-axis-first-run`. "it is
specially important for ourselves, as the agentic workflow is a good part of what the project
publishes, and thus has the potential to help significantly at  improving the project."

### The workflow functions as a whole, so files shared out among agents hide contradictions across files `##a2`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@audit-method-per-axis`. "the design record axis splits the content
among multiple agents with the same tasks, but I do not think this would be very useful or even
doable for the agentic workflow, which has to function as a whole and where each file cannot be
considered independtly, and one of the main goals would be to find contradictions across different
files, instructions that cannot be both followed or might lead to confusion."

### Varied tasks, each over the full content `##a3`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "it would be more
interesting to send subagents with varied goals and tasks, but each given the full content of the
agentic workflow."

### The agentic workflow is the root CLAUDE.md and the content of the agent configuration directory `##a4`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus`. "When I say "agentic
workflow", what I mean here is root CLAUDE.md + the content of .claude/"

### Findings split between the shipped workflow and the local one with its interaction `##a5`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-outcomes`. "findings have to be
split between what is purely about the shipped workflow, and what is in our local workflow and its
interaction with the installed one."

### Scoped CLAUDE.md files are developer documentation, not instructions `##a6`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus`. "Scoped CLAUDE.md files
are out of the agentic workflow. What they contain is not instructions to follow, it is more like
developper documentation."

### Duplicated tasks can find more `##a7`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "its not necessarily a
bad thing to send several with exactly the same tasks IMO. If there is room, the duplication can
help finding more issues."

### A budget of about 15 agents `##a8`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "the budget should be
around 15 subagents I think."

### Most findings are instructions that cannot be followed, or gaps judgement cannot fill, under the principle that judgement covers the rest `##a9`

Round 1, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-finding-standard` and
`thread@agentic-workflow-axis@workflow-axis-lenses`. "most of the work should be about finding things
that cannot be followed, or gaps where judgement is not enough (those have to follow the principle
that some thing are left to agent's judgement, because we cannot cover every case that may arise,
and it is better to keep instructions restricted to what is necessary for the workflow to function
as intended globally)."

### The audit skill's method must be adapted for this axis `##a10`

Round 1, owner. Bears on `thread@agentic-workflow-axis@audit-method-per-axis`. "The project audit
skill will have to be adapted to fit this axis, in term of the general method. I'm not sure how."

### A predicted gap originates no edit `##a11`

Round 1, agent, presented as a material finding before the threads. Bears on
`thread@agentic-workflow-axis@workflow-axis-finding-standard` and
`thread@agentic-workflow-axis@workflow-axis-outcomes`. From `design@agent-skills@additions-need-real-use`:
"A finding that predicts a behaviour is parked as an issue, which states what a real session would
have to show." So "A gap that only predicts what an agent would do becomes an issue, not an edit."

### No run observes whether an agent follows a step `##a12`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-finding-standard` and
`thread@agentic-workflow-axis@workflow-axis-lenses`. "`design@agent-skills@synthetic-evidence-not-built`
refuses any run built to observe "whether an agent follows a step"."

### One agent can read the whole corpus `##a13`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus` and
`thread@agentic-workflow-axis@workflow-axis-lenses`. Measured with
`git ls-files CLAUDE.md .claude | grep -v open-issues | xargs wc -w`: "61,065 words over 26 files.
That is roughly 80–90k tokens, so one agent can read all of it."

### The developer-contracts issue points the same way as the owner's exclusion `##a14`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus`.
"`issue@core@a-home-for-developer-contracts-outside-agent-configuration` supports leaving scoped
CLAUDE.md files out".

### The shipped text names no provider's directory `##a15`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus`.
"`issue@core@configuration-for-several-agent-providers` means the shipped text must name the corpus
without hard-coding `.claude/`." "The shipped text names this without writing `.claude/`, so it holds
for a second harness."

### Registers are records, read only as findings already known `##a16`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-auditor-agent` and
`thread@agentic-workflow-axis@workflow-axis-corpus`. "About ten other issues are already-known
findings the first run will meet … The auditors must recognise them rather than report them again."
The agent-config register: "It holds records, not instructions. Auditors still read it, as a list of
findings already known."

### The method head states groups and a sample, which the new axis drops `##a17`

Round 1, agent. Bears on `thread@agentic-workflow-axis@audit-method-per-axis`.
"`design@agent-skills@audit-method` puts these in its title: "bounded groups … calibrated on a shared
sample". Your axis drops both. Changing the head's statement is a change of a decision, so it is
argued here."

### The owner expected the method to differ per axis `##a18`

Round 1, agent, quoting the owner's words recorded in
`issue@agent-skills@audit-axes-beyond-the-design-record`. Bears on
`thread@agentic-workflow-axis@audit-method-per-axis`. "it might differ per axis … we can adjust this
generic shape later for the others."

### An exception would leave the method stating a rule one of two axes breaks `##a19`

Round 1, agent. Bears on `thread@agentic-workflow-axis@audit-method-per-axis`. The rival, an
exception, loses "because the skill's method would then state one rule (groups and a sample) that one
of only two axes breaks."

### Three scoped restatements are left unread `##a20`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus`. "Three scoped files hold
restatements of workflow directives that they mark as such … With the boundary above, no step reads
them against their homes." Default: they stay out, and the developer-contracts issue gets a line
noting them.

### A false claim about the checker or the harness is a factual error, checked by a read-only command `##a21`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`. "Auditors may run read-only commands,
such as `--help` or `cargo klarch show`, to check what a text claims about the checker or the
harness. A false claim is a "factual error", one of the three classes that decision admits."

### The count is a default within the owner's budget `##a22`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "The skill would require
each lens at least once and leave the count to the session within the budget the owner sets. Your 15
would be the stated default, so the text fixes no count."

### The activity walk reads, it does not decide `##a23`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "the walker reads; it
does not make decisions as a session would. What it returns is a pair of quoted instructions and the
moment both apply. The session can check that by reading."

### Duplication replaces calibration `##a24`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "Calibration: none.
Agents with the same lens find different things, and that adds coverage. The sort removes
duplicates, and a finding reported by two agents is noted as such."

### Files shared out lose the cross-file contradiction `##a25`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. The rival, the
design-record axis's split, "loses on your argument: a contradiction across two files sent to
different agents is found by neither."

### A provable gap misses a fact fixed elsewhere, not a choice `##a26`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-finding-standard`. "the missing
piece is a fact that is fixed somewhere else, such as a format the checker enforces, a name, or a
path. It is not a choice the agent could make itself."

### An installed file is never edited in a consumer `##a27`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-outcomes`. W in a consumer:
"Never edited locally (`skill@knowledge-architect-agent-configuration@installed-files-never-edited`)."

### Filling a gap adds an instruction, which is a decision `##a28`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-outcomes`. "a provable gap.
Filling it adds an instruction, which `content-and-style` treats as a decision."

### An agent per lens would repeat the shared standard `##a29`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-auditor-agent`. The rival, one agent per lens: "Each
would repeat the shared standard, which is about half of any one of them."

### The agent-per-axis head is about the standard reaching the agent `##a30`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-auditor-agent`. "The decision that each
axis gets its own agent is about the axis's standard reaching the agent from its first token. That
holds with one agent per axis."

### The first run's lessons reach the axis before it merges `##a31`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-first-run`. "Lessons about the
method from that first run then go into the axis before it merges."

### One branch is large; two branches review smaller `##a32`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-first-run`. "Cost: a large branch.
The first run edits many skills, and every reviewer must read those edits." The rival "gives smaller
reviews, and the first run's lessons reach the axis one merge later."

### The triggers head allows the run in another work's branch `##a33`

Round 1, agent. Bears on `thread@agentic-workflow-axis@workflow-axis-first-run`.
"`design@agent-skills@audit-triggers` lets you put the run into another work's branch." The
permission is in the audit skill's `skill@knowledge-architect-project-audit@when-an-audit-runs`, not
in that head.

### The owner's concern about too many published agents `##a34`

Round 2, owner. Bears on `thread@agentic-workflow-axis@workflow-auditor-agent`. "I was worried about
publishing too many agents and polluting the directory. I think this shape works."

### Duplicate agents may add nothing `##a35`

Round 2, agent, premortem cause 1. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`. "The
second agent on a lens finds only what the first found. Each extra agent then costs about 90k tokens
and adds no coverage."

### Drafts may be mostly noise `##a36`

Round 2, agent, premortem cause 2, quoting the owner's words recorded in the deleted spec
design-record-audit, D8. Bears on `thread@agentic-workflow-axis@workflow-axis-finding-standard`. "You
said: "If you ask an LLM to find a flaw or defect in something, it will nearly always find one."
Auditors could report predicted gaps as provable ones, and the sort would then spend its time
rejecting them."

### The corpus may outgrow one agent's reading `##a37`

Round 2, agent, premortem cause 3. Bears on `thread@agentic-workflow-axis@workflow-axis-corpus` and
`thread@agentic-workflow-axis@workflow-axis-lenses`. "An agent that skims some files misses
contradictions between files, which is the axis's main purpose. Today the corpus is 61,065 words. It
will grow."

### One agent with all the lenses may flatten them `##a38`

Round 2, agent, premortem cause 4. Bears on `thread@agentic-workflow-axis@workflow-auditor-agent`. "An
agent briefed on L3 also reports L1 findings, the lenses converge on the same findings, and
duplication buys nothing."

### The relocation may lose a rule of the design-record method `##a39`

Round 2, agent, premortem cause 5. Bears on `thread@agentic-workflow-axis@audit-method-per-axis`. "A
rule of the current method, such as the 60-entry bound or the reading of a sample of conforming
drafts, is lost when the groups and the sample move."

### No check enforces that shipped text cites no entry `##a40`

Round 2, agent. Bears on AC3. "This criterion exists because
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` says no check enforces that yet."

### Passes that do not fall in count and severity show an audit that does not improve the record `##a41`

Round 3, owner. Bears on `thread@agentic-workflow-axis@workflow-axis-lenses`,
`thread@agentic-workflow-axis@workflow-axis-finding-standard` and the passes of
`thread@agentic-workflow-axis@audit-method-per-axis`. "following what was done for the design record
axis. We ran it multiple times (3) and observed that the amount of returned findings was decreasing
in both quantity and severity. I think this is important. If it does not, that means the audit is
not pushing toward improvement."

### Agents cannot work efficiently without knowing what the checker refuses and allows `##a42`

Round 4, owner. Bears on `thread@agentic-workflow-axis@checker-rules-lens`. "is the checker behavior
properly explained in the workflow ? Agents cannot work efficiently without knowing what the checker
refuses and allows to write."

### L4 starts from the corpus, and a rule no text mentions is invisible from there `##a43`

Round 4, agent. Bears on `thread@agentic-workflow-axis@checker-rules-lens`. "L4 starts from the
corpus: it finds a requirement the text states and checks whether the text gives its input. A
checker rule that no text mentions at all is invisible from there. L7 starts from the checker and
checks that each rule reaches the agent."

### A project's restatement of the checker's rules would hide a gap of the installed text `##a44`

Round 4, agent. Bears on `thread@agentic-workflow-axis@checker-rules-lens`. "our root CLAUDE.md
restates most of the checker's rules in `instructions@mechanical-validation`, so if the lens read the
whole corpus at once, every rule would look covered. Gaps in the installed text are therefore judged
on the installed text alone."

### A rule whose finding names its repair costs one round; one that shapes writing must be known first `##a45`

Round 4, agent. Bears on `thread@agentic-workflow-axis@checker-rules-lens`. "A rule whose message
names the repair costs one extra check-and-fix round when the agent breaks it. A rule that shapes how
text is written from the start, such as the reference grammar, has to be known before writing."

### The budget is kept at 15 by narrowing L5 `##a46`

Round 4, agent. Bears on `thread@agentic-workflow-axis@checker-rules-lens` and
`thread@agentic-workflow-axis@workflow-axis-lenses`. "L7 gets 2, and L5 drops from 2 to 1, since D3
narrowed L5 to drifted restatements and missing pointers." The same message said this kept the
default at 15; the counts it proposed, which the owner approved, sum to 16.

## New names, in one place

```text
skill section   skill@knowledge-architect-project-audit@agentic-workflow-axis
                  in path@agent-skills@content/skills/project-audit/SKILL.md, as {{slug:agentic-workflow-axis}}
agent           knowledge-architect-workflow-auditor
                  source  path@agent-skills@content/agents/workflow-auditor.md
                  install path@agent-config@agents/knowledge-architect-workflow-auditor.md
lenses          L1 contradiction, L2 activity walk, L3 two readings, L4 provable gap,
                L5 restatement, L6 local against installed, L7 checker rules
                                                              (in the agent and the section)
tags            W, P, I                                       (in the agent and the section)
```

The section slugs of the new agent are its own, chosen at step 2.

## Decided design

### The generic method

`thread@agentic-workflow-axis@audit-method-per-axis`. The audit skill's method states what holds for
every axis, and each axis's section gives its dispatch. The generic points:

1. **the corpus**: the axis's section names it, entries or texts, and what is read beside it;
2. **the dispatch**: the axis's section gives how the corpus is shared out among agents, how many,
   what each brief names, and how they are calibrated, if they are; all are dispatched in parallel,
   each with a scratch directory of its own;
3. **the pre-commitments**, as point 3 states them today;
4. **the drafts**: each agent writes its drafts to its scratch directory and edits nothing else;
   results stay in the files, and the session reads the drafts, not a summary of them;
5. **the sort**: every draft that proposes an outcome other than "nothing" is read against what it
   cites; a draft found wrong is corrected, and the drafts of the same kind from the same agent are
   read again; nothing an agent proposes is applied before a reading confirms it; a quotation of the
   owner is confirmed against the question it answered;
6. **the owner list**, as point 7 states it today, the deferred item becoming an issue entry;
7. **the application**, the whole diff read before each commit;
8. **the re-check**: fresh agents of the axis, dispatched as its first dispatch was, after the
   edits, each also given the kept list; a confirmed finding of the re-check that is not on the kept
   list is a violation left, read and then repaired or put to the owner; the re-check's drafts start
   the next pass; passes repeat while the violations left fall from one pass to the next, until the
   owner judges what is left small enough;
9. **the review**, on the axes point 10 lists today.

The skill's other texts that state entries, groups or calibration for every audit become generic in
the same step: the Scope paragraph ("every entry or text of a corpus"), the Terms table (a corpus of
entries or texts; a group as the design-record axis's; a draft as one file per entry or per finding,
as the axis says; the re-check as above; a pass added), the Outcomes row "nothing" ("the entry or
the text conforms"), "The record of a run" (the dispatch and the calibration reading where the axis
has one, in place of "how many entries and groups, the calibration reading settled"), and the
frontmatter description's parenthesis.

The design-record axis's section gains its dispatch, per the mapping table: the groups, the
calibration sample, the brief, the conforming drafts read at the sort, and the drafting of a split, a
merge or a move with each created head judged alone.

The nearest rival is the method kept as written, with the new axis adjusting it as the preamble
allows. It loses because the new axis replaces points 2, 4, 5, 6 and 9 whole rather than adjusting
them, and the head's title, the Terms table and "The record of a run" would still state groups and a
sample for every audit, `argument@agentic-workflow-axis@a19`.

### The corpus

`thread@agentic-workflow-axis@workflow-axis-corpus`. Every text the harness delivers to a session as
an instruction: the root CLAUDE.md and the primer it imports, the installed skills and agents, and the
project's own skills and agents. The shipped text names no provider's directory. Out of it: the
scoped CLAUDE.md files, and the registers. The auditors run `cargo klarch issues`, every anchor, and
read a listed entry that bears on a finding, so that a finding already recorded is marked with its
reference rather than reported again. An auditor may run read-only commands to check what the corpus
claims about the checker or the harness.

### The lenses

`thread@agentic-workflow-axis@workflow-axis-lenses`. Every auditor reads the whole corpus. Each takes
one lens:

| lens | looks for | default agents |
| --- | --- | --- |
| L1 contradiction | two instructions, in any files, that no single move satisfies; a term defined two ways | 3 |
| L2 activity walk | one activity followed across files, listing at each moment the instructions that apply: two that conflict, a hand-off to something no text defines, a pointer to a section that does not exist, a loop with no exit | 4, one per group of activities, per D13: design, planning, decision recording and issue tracking; review and merge; audit and retrospective; setup, pin move, goal setting and agent configuration |
| L3 two readings | an instruction whose words allow two readings that lead to different acts | 2 |
| L4 provable gap | per "The finding standard" | 2 |
| L5 restatement | each restatement against its home: one that drifted from it, or one without its pointer, per D3 | 1 |
| L6 local against installed | the project's own text against the installed text: a contradiction, a duplicate, a routing-table row that says the wrong thing | 2 |
| L7 checker rules | per "The checker-rules lens" | 2 |

Each lens is dispatched at least once. The count is the session's, within the owner's budget, and the
table's total, 16, is the stated default. An L2 agent's brief names its group of activities; two L2 agents
are duplicates only when they share a group. Each agent takes one lens; a lens takes as many agents as its row says. The re-check dispatches the
same lenses with the same count per lens as the first dispatch. There is
no calibration sample: the sort reads every finding, the duplicates of a lens add coverage, and the
re-check reads again. A finding two agents report is merged and noted as such.

Under D3's default, a restatement only longer than a pointer is no finding: per
`design@agent-skills@restatement-size-test`, it is converted when a change edits what it says, and
no sweep converts them all. A drifted restatement is repaired, which edits what it says, so its
repair converts it to a pointer when it is longer than one.

The nearest rival, the corpus shared out among agents of one task, loses on
`argument@agentic-workflow-axis@a25`. The rejected alternative "An audit method with no calibration
and no re-check" does not cover this shape. Its recorded reason holds two facts: the re-check found
what each pass missed, and two agents read one shared sample entry two ways, where a method trusts
one reading per entry. This axis keeps the re-check, gives no verdict per entry, and trusts no one
reading, since a finding is confirmed at the sort and the re-check reads again.

### The checker-rules lens

`thread@agentic-workflow-axis@checker-rules-lens`. An L7 agent reads, beside the corpus, the
checker's user documentation: its README for the version the project pins, which ships in the
checker's package, and the help of the project's declared command with the help of each command. In
this repository, per D12, the root CLAUDE.md also gives it the checker's design home.
It lists every rule the checker enforces on what an agent writes, and reports:

- a rule no installed text states: tag W, a provable gap, since it is a format the checker enforces;
- a rule only the project's own text states: tag W as well, since the installed text is all an
  adopting project receives. The installed text is judged alone for this, so that a project text
  restating the checker's rules, as this repository's root CLAUDE.md does in
  `instructions@mechanical-validation`, does not hide a gap of the installed text;
- a rule the corpus states wrongly: a factual error.

Each draft says whether the checker's finding message for the rule names its repair. A rule whose
message names the repair costs one check and one repair when it is broken; a rule that shapes how
text is written from the start, such as the reference grammar, has to be known before writing.
Which rules are explained ahead is the owner's, on the owner list, as a provable gap is per D4. Where
an explanation goes, the primer or a skill, is judged at the sort under
`design@agent-skills@primer-limit`.

The nearest rival, the README given to L4, loses on `argument@agentic-workflow-axis@a43`: L4 starts
from a requirement the corpus states, and a rule no text mentions is invisible from there.

The checker's design home and README are 26,611 and 5,173 words, `wc -w` on
`path@core@docs/design.md` and `path@core@README.md`, so an L7 agent here reads about 93,000 words;
the published crate ships the README and no docs/, per the `include` list of `path@core@Cargo.toml`.

### The finding standard

`thread@agentic-workflow-axis@workflow-axis-finding-standard`. A finding is admitted when the
session can confirm it by reading:

- **a contradiction**: both instructions quoted, and the moment at which both apply;
- **a broken trigger**: an instruction that hands off to, or points at, something no text defines,
  quoted with the search that shows the target absent;
- **a factual error**: the claim quoted, and the command that shows it false;
- **two readings**: the instruction quoted, and the two acts it allows;
- **a provable gap**: a requirement of the corpus or of the checker, and an input to it that no text
  supplies and that judgement cannot derive, since it is a fact fixed elsewhere, such as a format the
  checker enforces, a name or a path, not a choice the agent could make;
- **a predicted gap**: "an agent might misjudge X": an issue stating what a real session would have
  to show, never an edit.

A finding about how the owner works is outside the axis: the retrospective judges it against an
expectation set. The rival, every gap a finding to fill, fails `criterion@agentic-workflow-axis@c4`
and `goal@agent-skills@installed-text-leaves-room-to-judge`.

### The outcomes

`thread@agentic-workflow-axis@workflow-axis-outcomes`. Each finding carries a tag: W for the installed
text, P for the project's own text. A finding of the interaction of the two, I, is split at the sort
into its W side and its P side, each a finding with one outcome, as the retrospective writes an
interaction finding in both its files.

| tag | in the installed text |
| --- | --- |
| W | never edited in the project; sent upstream, per D5 |
| P | applied, put to the owner, or an issue |

Per D12, the installed text holds no branch for the project that publishes the workflow. This
repository's root CLAUDE.md says, in `instructions@repository-skills`, that here a W finding
is edited at its source, as `design@knowledge-architect@retrospective-findings-stay-here` already
says for a retrospective's findings, and takes the outcomes of a P finding. A finding on the
checker's own behaviour, the retrospective's `C`, is outside the axis, since the checker's code is
not in the corpus: it is an issue entry here, and in another project a finding of its next
retrospective.

Each lens target maps to a class, and each class to an outcome:

| lens target | class | outcome |
| --- | --- | --- |
| two instructions no move satisfies; a term defined two ways | contradiction | owner list |
| a hand-off or a pointer to something no text defines | broken trigger | applied when the target exists under another name; else owner list |
| a loop with no exit | contradiction, between the instructions that form it | owner list |
| a false claim about the checker, the harness or the tree | factual error | applied |
| an instruction allowing two acts | two readings | applied when the repair changes no instruction; else owner list |
| a requirement with an input no text supplies; a checker rule no installed text states | provable gap | owner list |
| a restatement that drifted from its home | contradiction, with its home | applied, brought back to its home |
| a restatement without its pointer | broken trigger | applied, the pointer added |
| a project text duplicating an installed one word for word | restatement without its pointer, when no pointer stands beside it | applied |
| a routing-table row that says the wrong thing | factual error | applied |
| an agent might misjudge X | predicted gap | issue entry |

Outcomes of a P finding, and in this repository of a W finding:

- **applied**: a rewording that changes no instruction; a broken pointer; a factual error; a drifted
  restatement brought back to its home;
- **owner list**: which side of a contradiction wins; a broken trigger whose repair changes what
  agents are told; a repair of a two-reading instruction that changes what agents are told; a
  provable gap;
- **issue**: a predicted gap; a finding the owner defers.

**Sent upstream**, per D5 and D15: the W findings of a run go into one file in the shape of the
retrospective's workflow file, each numbered `W<n>`: the version of the checker used, what was
audited at which commit, and the findings, with no standing questions, written to the directory the retrospective uses
and named `<YYYY-MM-DD>-<project>-workflow-audit-klarch-workflow.md`, under the rules of
`skill@knowledge-architect-retrospective@two-files` and
`skill@knowledge-architect-retrospective@what-becomes-of-files`: nothing leaves the machine before
the owner has read it.

### The workflow auditor

`thread@agentic-workflow-axis@workflow-auditor-agent`. One installed agent,
`agent@knowledge-architect-workflow-auditor`, tools Read, Grep, Glob, Bash, read-only in its body. Its
frontmatter description says what it drafts and how it is dispatched: each agent on one lens, as
many per lens as the lens table of the axis's new section of the audit skill says, all in parallel, each
brief naming the commit audited, the lens, for L2 the group of activities, a scratch directory of its
own and, for a re-check, the kept list; then its drafts are read, never its summary alone; "Dispatch
it; do not read it." Its body holds:

- the standard: it edits nothing; it writes only to its scratch directory; every quotation is copied
  from the file; every command listed was run;
- its brief, as above;
- the corpus, named as the axis's section names it, every file read whole;
- `cargo klarch issues` run before drafting, a finding already recorded marked with its reference;
- one section per lens, the brief naming which is its task; a finding of another lens met on the way
  is drafted too, and its draft names the finding's lens, not the brief's;
- the finding standard and the tags;
- one draft per finding: lens, tag, class, the quotations with their files and lines, the moment or
  the two acts or the command, the proposed repair, the proposed outcome;
- the return: the corpus files read whole, one line per draft, the commands run.

The nearest rival, one agent per lens, loses on `argument@agentic-workflow-axis@a29`.

## Mapping tables

### Today's method, point by point, to the generic method and the design-record axis

| today's text (`skill@knowledge-architect-project-audit@audit-method`) | after step 1 |
| --- | --- |
| preamble: each point leaves judgement; an axis adjusts it | generic preamble, unchanged |
| 1, the corpus and the inputs read beside each entry | generic 1 |
| 2, groups of one Component, the fixed order, ceil(count / 60), sizes differing by one, count and name each group | design-record dispatch |
| 3, pre-commitments | generic 3 |
| 4, the calibration sample, its comparison, settlement, re-reading in every group, owner list for two readings of a rule | design-record dispatch |
| 5, one agent per group, all in parallel, own scratch directory | generic 2 (parallel, scratch) and design-record dispatch (per group) |
| 5, one draft per entry; the brief names commit, first and last entry, sample, scratch | design-record dispatch |
| 5, results stay in files, drafts read not summarised; edits nothing else | generic 4 |
| 6, read every draft not "nothing" against its entry and history | generic 5 ("against what it cites") |
| 6, at least one conforming draft read per group | design-record dispatch |
| 6, sort into outcomes; a draft found wrong corrected, the group's other drafts of the same kind read again | generic 5 (per agent) and design-record dispatch (per group) |
| 6, nothing applied before a reading confirms it; the owner's quotation against its question | generic 5 |
| 7, the owner list, `F<n>`, deferred item to an issue | generic 6 |
| 8, disjoint application, the whole diff read | generic 7 |
| 8, a split, a merge or a move drafted first; each head a split, a rename or a merge creates judged alone | design-record dispatch |
| 9, fresh agents over the same groups, the kept list | generic 8 ("as its first dispatch was") and design-record dispatch (same groups) |
| 9, violation left, read, repaired or put to the owner; passes and the stop rule | generic 8 |
| 10, the review and its axes | generic 9 |

## Losing alternatives

- **The method kept, the new axis as its exception**: lost to
  `thread@agentic-workflow-axis@audit-method-per-axis`, on `argument@agentic-workflow-axis@a19`.
- **The corpus shared out among agents of one task**: lost to
  `thread@agentic-workflow-axis@workflow-axis-lenses`, on `argument@agentic-workflow-axis@a25`.
- **Every gap a finding to fill**: lost to
  `thread@agentic-workflow-axis@workflow-axis-finding-standard`, on `argument@agentic-workflow-axis@a11`.
- **One agent per lens**: lost to `thread@agentic-workflow-axis@workflow-auditor-agent`, on
  `argument@agentic-workflow-axis@a29` and the owner's `argument@agentic-workflow-axis@a34`.
- **The first run in a branch of its own**: lost to
  `thread@agentic-workflow-axis@workflow-axis-first-run`, on `argument@agentic-workflow-axis@a31`.

The recorded rejected alternative "An audit method with no calibration and no re-check" stays
rejected. At the harvest it names both heads it lost to: the generic method, which keeps the
re-check, and `design@agent-skills@design-record-axis`, which keeps the calibration.

## Readings

Empty: the work reads no external specification.

## Premortem

Each cause was put to the owner in round 2 under its label; the owner, round 3: "record all four,
apply the three acceptance criteria." The owner proposed AC4 in the same message; its wording is
D1. T1's firing condition is reworded here from round 2's, since "its first" agent names no agent
when agents are dispatched together; the cause it watches is unchanged. T2 and T4 count "in one
pass" where round 2 said "in one run", since a run holds several passes. The bounds of T1 to T4
are the ones round 2 put to the owner and the owner ruled to record.

`thread@agentic-workflow-axis@checker-rules-lens` adds no cause of its own: an L7 agent reads more
than the others, which T3 watches, and drafts that report rules the finding message already repairs
are weighed by the owner on the owner list and counted by T2 and AC4.

| label | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| T1 | Duplicate agents add nothing | `thread@agentic-workflow-axis@workflow-axis-lenses` | tripwire. Fires when, in one run, for every lens dispatched more than once (two L2 agents count only when they share a group of activities), one of its agents alone reported every confirmed finding of that lens. Response: reopen the duplicate dispatch. Re-entry: the sort of each pass counts it, the first in this branch |
| T2 | The drafts are mostly noise | `thread@agentic-workflow-axis@workflow-axis-finding-standard` | tripwire. Fires when, in one pass, more than half of the drafts fail the session's reading. Response: reopen the finding standard and the auditor's standard. Re-entry: the sort of each pass |
| T3 | The corpus outgrows one agent's reading | `thread@agentic-workflow-axis@workflow-axis-corpus`, `thread@agentic-workflow-axis@workflow-axis-lenses` | tripwire. Fires when an auditor's return says it did not read every corpus file whole. Response: reopen how the corpus is shared out. Re-entry: the sort of each pass. Also AC1 |
| T4 | One agent with all the lenses flattens them | `thread@agentic-workflow-axis@workflow-auditor-agent` | tripwire. Fires when, in one pass, more than a third of one agent's drafts name a lens other than its brief's. Response: reopen one agent against one agent per lens. Re-entry: the sort of each pass |
| AC1 | (cause 3) | `thread@agentic-workflow-axis@workflow-axis-corpus` | acceptance criterion, `acceptance@agentic-workflow-axis@files-read-and-drafts-tagged` |
| AC2 | The relocation loses a rule of the design-record method | `thread@agentic-workflow-axis@audit-method-per-axis` | acceptance criterion, `acceptance@agentic-workflow-axis@design-record-dispatch-kept` |
| AC3 | Shipped text cites an entry of this repository | `thread@agentic-workflow-axis@workflow-auditor-agent` | acceptance criterion, `acceptance@agentic-workflow-axis@shipped-text-cites-no-entry` |
| AC4 | The audit does not push the workflow toward improvement | `thread@agentic-workflow-axis@workflow-axis-lenses`, `thread@agentic-workflow-axis@workflow-axis-finding-standard` | acceptance criterion, `acceptance@agentic-workflow-axis@passes-fall` |

## Acceptance criteria

### Every auditor reads the whole corpus, and every draft names its lens and its tag `##files-read-and-drafts-tagged`

AC1. Withdrawn per D10, ruled in round 6: its observable judges a new agent's report against its
instructions, which `design@agent-skills@synthetic-evidence-not-built` refuses by name. T3 keeps
watching the cause, on what an auditor's return says. Step 3 does not judge it.

### The design-record axis keeps every rule its method held `##design-record-dispatch-kept`

AC2. Guards `thread@agentic-workflow-axis@audit-method-per-axis`. Judged at step 1, against the diff,
row by row of the mapping table: every row of today's text is stated after the change, at the place
the table gives. The design-record auditor's body, which restates the groups and the sample, is read
with it and stays true. Fires when one is missing. Response: restore it before the next step.

### The shipped text the branch writes cites no entry of this repository `##shipped-text-cites-no-entry`

AC3. Guards `thread@agentic-workflow-axis@workflow-auditor-agent` and
`thread@agentic-workflow-axis@audit-method-per-axis`. Judged at steps 1, 2 and 3, per D7, on every
line the branch adds under `path@agent-skills@content/` and `path@agent-skills@snippets/`. The
instrument:
``git diff origin/main -- crates/agent-skills/content crates/agent-skills/snippets | grep '^+' | grep -v '^+%%' | grep -E '`(design|issue|tripwire|goal|spec|milestone)@[a-z0-9-]+@[a-z0-9]'``
prints nothing. It matches a reference whose id starts with a letter or a digit, so a placeholder
in angle brackets and a section of the design skill, `skill@knowledge-architect-design@<slug>`, do
not match; over the whole shipped text at the spec's revision it prints nothing. Fires on a line it prints. Response: move it to a `%%` line or remove it.

### The confirmed findings and the severe ones fall from each pass to the next `##passes-fall`

AC4, proposed by the owner in round 3; its wording is D1, ruled in round 4. Guards
`thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`. Judged at step 3, read as an
observation of real use per D2. The instrument: the count of confirmed findings of each pass, and
the count of severe ones, both written in that pass's commit message. Fires when, from one pass to
the next, the confirmed findings do not fall, or the severe ones do not fall while they are above
zero. It is judged over every pass the run makes; the passes stop as the generic method says, on the
owner's judgement. Response: before the merge, reopen
`thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`.

## Implementation sequence

Every step that changes the installed text, steps 1, 2 and 3, runs
`cargo klarch install-agent-skills` and commits the installed copies with their source, passes the
edit tests of `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", read with
`design@agent-skills@additions-need-real-use` as D4 and D11 rewrite it, and adds the
CHANGELOG.md entries `design@knowledge-architect@changelog-entries` owes, then runs
`cargo x changelog`. Every commit runs `cargo klarch check --staged` before it and
`cargo klarch commits origin/main..HEAD` after it.

1. **The generic method.** The audit skill's method made generic, per "The generic method" and the
   mapping table; the skill's other texts made generic; the design-record axis's section gains its
   dispatch; the design-record auditor's description re-pointed to that section. Judges AC2 and AC3.
   Fails alone on: a row of the mapping table lost.
2. **The axis and its agent.** The workflow auditor, per "The workflow auditor"; the axis's section
   of the audit skill, per "The corpus", "The lenses", "The finding standard" and "The outcomes"; the
   audit skill's frontmatter description and scope naming the axis; the generic Outcomes table gains
   the row "sent upstream", per D5; the sentences of `instructions@repository-skills`, per D12. The commit
   converts the spec's `planned` citations, and writes the new agent's name in this spec as its
   reference. Judges AC3.
   Fails alone on: an agent whose brief cannot be filled from the section, or a section that cannot be
   dispatched from.
3. **The first run.** The axis runs on this repository under the skill, in this branch. Passes repeat
   as the generic method says. Each pass's commit message carries the counts AC4 reads and the
   observations T1 to T4 read. The owner list waits for the owner's answers. Lessons on the method
   found by the run amend the section or the agent in their own commits; a repair of a W finding is
   an edit of installed text. Judges AC3 and AC4. Fails alone on: the counts of AC4.
4. **The harvest.** The rows of "Harvest"; the spec deleted.

The branch is reviewed once, before the merge, on the axes of the generic method's review, which
include the decision-record, routing and standing-state axes the harvest owes, and on spec
conformity, which `skill@knowledge-architect-review@review-axes` owes to work a spec was written
before.

## Order rationale

- 1 before 2: the new section is written against the generic method.
- 2 before 3: the run dispatches the agent the section describes.
- 3 before 4: the harvest records the decisions after AC4 is judged and the run's lessons are in.

## Defaults awaiting the owner

None awaits the owner. D1 to D7 were ruled in round 4, on the review message: "All defaults
approved." D8 to D15 were ruled in round 6, on the re-review message, the agent's message that opens
"Process slip, already contained": "All defaults approved." Each is kept below, marked as ruled, since the sections and the harvest rows it shaped
cite it by its label.

- **D1**, ruled, on AC4, from the transcript review: the wording of AC4 shown in round 3 is not in the
  transcript, so the owner has not ruled on it. Default: the wording in "Acceptance criteria" above,
  which drops round 3's "at least three passes" and a fall at every pair, after the owner's ruling on
  the design-record axis's criterion: "I would not put such a rigid criterion. As long as we observe
  a "convergence", and the amount of leftover violations is small enough, I think that is good."
- **D2**, ruled, on `thread@agentic-workflow-axis@workflow-axis-first-run`, from the design-conformance
  review: `design@agent-skills@synthetic-evidence-not-built` refuses a run built "to accept a piece
  of work". Default: AC1 and AC4 are kept, read as observations of real use, since the first run is
  an audit of this repository's own workflow that the owner asked for, and the head admits "a
  misbehaviour observed in real use"; the same reading the owner ruled for the design-record axis's
  criterion.
- **D3**, ruled, on `thread@agentic-workflow-axis@workflow-axis-lenses`, from the design-conformance review:
  L5 as approved reports a restatement "longer than a pointer", and applying that at once is the
  sweep `design@agent-skills@restatement-size-test` rules out. Default: L5 reports a drifted
  restatement and a restatement without its pointer; a drifted one is repaired, which converts it to
  a pointer when it is longer than one; a restatement only longer than a pointer is no finding.
- **D4**, ruled, on `thread@agentic-workflow-axis@workflow-axis-finding-standard`, from the
  design-conformance review: round 1 presented "instructions that can be read two ways" as fixable
  within `design@agent-skills@additions-need-real-use`, and the owner agreed to that reading. The
  head lists three classes, "a contradiction, a broken trigger, a factual error"; two readings and a
  provable gap are not among them. Default: both stay admitted, each going to the owner list where
  its repair changes what agents are told, and the head is rewritten at the harvest to name them, as
  members its argument, a defect "provable by reading", covers. The standard gains the head's
  "broken trigger", which round 1 left out.
- **D5**, ruled, on `thread@agentic-workflow-axis@workflow-axis-outcomes`, from the design-conformance and
  cold-implementer reviews: the upstream file is an outcome `design@agent-skills@audit-outcomes`
  does not name, and an I finding with two outcomes breaks "Every finding takes exactly one outcome".
  Default: the generic Outcomes table gains "sent upstream", the head is rewritten at the harvest,
  and an I finding is split into its W and P sides at the sort.
- **D6**, ruled, on `thread@agentic-workflow-axis@audit-method-per-axis`, from the design-conformance review:
  `design@agent-skills@audit-is-an-activity` says an audit "reads every entry of a corpus" and argues
  "the method is the same for every axis". Default: rewritten at the harvest to "every entry or text
  of a corpus" and "the method is generic, with a dispatch per axis".
- **D7**, ruled, on AC3, from the transcript review: AC3 as approved covered "the new skill section and the
  new agent"; step 1 and step 3's repairs also write shipped text. Default: AC3 judges every line the
  branch adds to shipped text, at steps 1, 2 and 3, with the instrument above, and counts only
  entries, not `path@*@<path>` paths.

- **D8**, ruled, on `thread@agentic-workflow-axis@checker-rules-lens`, from the
  transcript review: round 4 proposed "keep the default at 15 agents: L7 gets 2, and L5 drops from 2
  to 1", and the counts sum to 16; the owner approved both statements. Default: 16, the counts as
  approved, which "around 15" admits.
- **D9**, ruled, on `thread@agentic-workflow-axis@workflow-axis-first-run`, from the
  design-conformance review: `goal@knowledge-architect@the-workflow-improves-through-real-use` says
  "No session is built to observe how agents follow its instructions: not to originate an edit, to
  choose between shapes, or to accept a piece of work", and AC4 and the responses of T1 to T4 judge
  the first run before the merge. D2 answered the head, not the goal. Default: the first run is real
  use, an audit of this repository's workflow the owner asked for in round 1, "then self-apply it
  for real practical testing", whose repairs land; its counts are observations of that use, and the
  goal holds. A conflict with a goal is the owner's. The owner, round 6: "About D9: what we are doing
  is real use, there is no doubt about that. Maybe there is a deeper question about what justifies
  considering real useas more valuable than synthetic use. But its not for us to answer, and not
  now."
- **D10**, ruled, on AC1, from the design-conformance review: AC1 judges a new agent's
  report against its instructions, a case `design@agent-skills@synthetic-evidence-not-built` refuses
  by name. Default: AC1 is withdrawn; T3 keeps watching its cause.
- **D11**, ruled, on `thread@agentic-workflow-axis@workflow-axis-finding-standard`, from
  the design-conformance and cold-implementer reviews: filling a provable gap adds an instruction
  from a reading, against the first sentence of `design@agent-skills@additions-need-real-use`, "Real
  use originates an addition", which D4 left as it was, and the goal it derives from. Default: the
  head's first sentence is scoped to an addition about how an agent behaves, since its argument is
  that predicted behaviour is unreliable evidence, and a provable gap predicts no behaviour: it is a
  fact an act needs and no text gives. A fill still goes on the owner list. Step 3 applies the head
  as D4 and D11 rewrite it.
- **D12**, ruled, on `thread@agentic-workflow-axis@workflow-axis-outcomes` and
  `thread@agentic-workflow-axis@checker-rules-lens`, from the design-conformance review: the shipped
  text held a branch for the project that publishes the workflow and the checker, which only this
  repository meets; the owner ruled such a branch out of the shipped text before: "I don't want to
  cater too much to this use case in the installed files". Default: the shipped text holds none;
  the root CLAUDE.md gains a routing-table row for the audit skill: here a W finding is edited at
  its source, and L7 also reads `path@core@docs/design.md`. Corrected at step 2, the substance
  unchanged: the routing table holds only a project skill or agent that adds to an installed one,
  per `skill@knowledge-architect-agent-configuration@root-claude-md-tables`, so the two sentences
  go beside the retrospective's in `instructions@repository-skills`, and the table stays empty;
  listed to the owner.
- **D13**, ruled, on `thread@agentic-workflow-axis@workflow-axis-lenses`, from the
  cold-implementer review: the four L2 groups left decision recording, issue tracking, goal setting
  and agent configuration in no group. Default: the groups are design, planning, decision recording
  and issue tracking; review and merge; audit and retrospective; setup, pin move, goal setting and
  agent configuration. A project's own skill is walked in the group of the activity it serves.
- **D14**, ruled, on `thread@agentic-workflow-axis@workflow-axis-outcomes`, from the
  transcript review: `design@agent-skills@audit-outcomes` routes a finding no case names to the
  owner, and names no predicted gap; the outcomes thread sends it to an issue. Default: the head is
  rewritten at the harvest to name a predicted gap among the issue entries, per
  `design@agent-skills@additions-need-real-use`.
- **D15**, ruled, on `thread@agentic-workflow-axis@workflow-axis-outcomes`, from the
  design-conformance review: numbering the upstream findings `W<n>` uses the retrospective's prefix
  for an audit's findings, and `design@agent-skills@ruled-items-labelled` lists `W` for "a
  retrospective's findings". Default: its row widens to the findings on the installed workflow, in a
  retrospective's file and in an audit's upstream file, since both go to the same receiver, which
  cites each by its file's stem.
- Listed, no ruling needed: a draft names a finding of another lens met on the way, with the
  finding's lens, an obligation step 2 added to the auditor so that no finding met is lost and T4
  can count drift.

## Harvest

Every head lands in `path@agent-skills@docs/design.md`, unless the row says otherwise. The threads
that take a new head pass entry test 2 of `primer@design-heads`: the axis's section and the workflow
auditor each state its reason, two texts. The harvest confirms each against the tests.

| item | where it lands |
| --- | --- |
| `thread@agentic-workflow-axis@audit-method-per-axis` | `design@agent-skills@audit-method` rewritten to the generic method; `design@agent-skills@design-record-axis` gains the dispatch and the argument for the bound of 60; `design@agent-skills@audit-is-an-activity` rewritten per D6 |
| `thread@agentic-workflow-axis@workflow-axis-corpus` | a head; a line on `issue@core@a-home-for-developer-contracts-outside-agent-configuration` naming the restatement of the four edit tests in `path@agent-skills@CLAUDE.md` |
| `thread@agentic-workflow-axis@workflow-axis-lenses` | a head |
| `thread@agentic-workflow-axis@workflow-axis-finding-standard` | a head; `design@agent-skills@additions-need-real-use` rewritten per D4 |
| `thread@agentic-workflow-axis@workflow-axis-outcomes` | `design@agent-skills@audit-outcomes` rewritten per D5 |
| `thread@agentic-workflow-axis@workflow-auditor-agent` | a head |
| `thread@agentic-workflow-axis@checker-rules-lens` | within the head of `thread@agentic-workflow-axis@workflow-axis-lenses` |
| `thread@agentic-workflow-axis@workflow-axis-first-run` | none: carried by the run's commits |
| T1, T2, T3, T4 | `path@agent-skills@docs/tripwires.md`, each naming the head of the thread it stresses, with its label |
| `tripwire@agent-skills@audit-defaults-overruled` | its premise rewritten to the sort, and the calibration where an axis has one |
| AC1 to AC4 | reported in the landing commit; one that recurs is proposed as a tripwire |
| "An audit method with no calibration and no re-check" | names both heads it lost to |
| every other item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| D12 | `design@knowledge-architect@retrospective-findings-stay-here` widened to an audit's findings on the installed text; its restatement in `instructions@repository-skills`, written at step 2 |
| D15 | `design@agent-skills@ruled-items-labelled`, its `W` row widened |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | rewritten to the axes it still holds; its open lesson, the owner's reading of a sample of the verdicts, stays, since this axis does not adopt it |

## Later consequences

- The other axes of `issue@agent-skills@audit-axes-beyond-the-design-record` each give their own
  dispatch under the generic method.
- A pin move across the version that ships the axis may cite it in a Migration entry, if the version
  changes rules the corpus must meet; `tripwire@agent-skills@axis-not-run-at-a-pin-move` then watches
  it as it watches the design-record axis.
