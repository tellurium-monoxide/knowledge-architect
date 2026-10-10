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
  that opens "record all four, apply the three acceptance criteria" and the agent's reply that words
  AC4. An extraction agent wrote the owner messages, the agent messages, the tables and the
  arguments verbatim to scratch; the session assembled this spec from that file and its own context.
- **Labels.** The discussion used T1 to T4 and AC1 to AC4. No other discussion in the transcript
  used either prefix.

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
- **a lens**: one task an auditor is briefed with, L1 to L6, per
  `thread@agentic-workflow-axis@workflow-axis-lenses`. The word is the agent's, round 1.
- **a draft**: one auditor's file for one finding, per "The workflow auditor".
- **a tag**: the side of a finding, `W` for the installed text, `P` for the project's own text, `I`
  for an interaction of the two. W and P are the letters of the retrospective's findings,
  `skill@knowledge-architect-retrospective@what-it-examines`. A tag classifies a finding; it is not a
  label of an item the owner rules on, so `design@agent-skills@ruled-items-labelled` gains no prefix.
- **the owner list**, **a pass**, **the re-check**: as the audit skill's terms define them,
  `skill@knowledge-architect-project-audit@audit-terms`.
- **confirmed**: a finding that survives the session's reading at the sort.
- **severe**: a confirmed finding whose outcome is the owner list: a contradiction, a provable gap,
  or a two-reading instruction whose repair changes what agents are told to do.

## What the work is

### What exists today

- `design@agent-skills@audit-method` states, in its title, "An audit is read in bounded groups by
  agents calibrated on a shared sample". The audit skill's method,
  `skill@knowledge-architect-project-audit@audit-method`, holds ten points. Points 2 (groups of at
  most 60 entries of one Component, in a fixed order), 4 (the calibration sample), 5 (the brief names
  the group's first and last entry and the sample), 6 (the sort reads at least one conforming draft
  per group), 8 (a split, a merge or a move of a head is drafted first, and each head it creates is
  judged alone) and 9 (the re-check runs over the same groups) are stated for every axis, and are
  specific to a corpus of entries.
- The audit skill has one axis section, `skill@knowledge-architect-project-audit@design-record-axis`.
- `agent@knowledge-architect-design-record-auditor` restates the group arithmetic in its description.
- `issue@agent-skills@audit-axes-beyond-the-design-record` lists the axes the owner named, among
  them "consistency of the project's own agent workflows", and the owner's ruling that the
  restatements of directives go in that axis.
- The corpus, measured with
  `git ls-files CLAUDE.md .claude | grep -v open-issues | xargs wc -w` at the spec's commit, is 26
  files; the figure taken in round 1 was 61,065 words.
- Three scoped CLAUDE.md files hold restatements of workflow directives that they mark as such:
  `path@agent-skills@CLAUDE.md`, `path@gates@CLAUDE.md`, `path@xtask@CLAUDE.md`.

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
  scoped CLAUDE.md files out of the corpus, and gains a line naming the three scoped restatements.
- `issue@core@configuration-for-several-agent-providers`: the shipped text names the corpus without
  a provider's directory.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the reason for AC3.
- Several issues are findings the first run will meet already recorded, such as
  `issue@agent-skills@the-material-finding-duty-has-no-head`; the auditors mark them as known.

### What is outside the work

- The other axes the issue names. Each takes its own discussion, per
  `issue@agent-skills@audit-axes-beyond-the-design-record`.
- The scoped CLAUDE.md files, and their three restatements, per
  `thread@agentic-workflow-axis@workflow-axis-corpus`; their home is the subject of
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`.
- A mechanical check of the properties of a project's own skills: `issue@core@tooling-for-project-skills`.

## What is already decided

The design rests on these and does not argue them again:

- `design@agent-skills@audit-is-an-activity`, `design@agent-skills@audit-outcomes`,
  `design@agent-skills@audit-triggers`, `design@agent-skills@audit-agent-per-axis`,
  `design@agent-skills@audit-repairs-the-whole-record`.
- `design@agent-skills@additions-need-real-use` and `design@agent-skills@synthetic-evidence-not-built`,
  read in round 1 and agreed by the owner, round 2: "Agreed on your readings of
  additions-need-real-use and synthetic-evidence-not-built."
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@restatement-size-test`,
  `design@agent-skills@expectation-set-bounds-scope`.

The work rewrites two heads:

- `design@agent-skills@audit-method`: rewritten to the generic method. Its citing texts, and what
  judges each:
  - `path@agent-skills@docs/design.md`, the head `design@agent-skills@audit-repairs-the-whole-record`:
    read at the harvest; it cites the method, which stays.
  - `issue@agent-skills@audit-axes-beyond-the-design-record`: rewritten at the harvest.
  - `path@agent-skills@docs/rejected-alternatives.md`, "An audit method with no calibration and no
    re-check": re-pointed at the harvest to the head that keeps the calibration, per Losing
    alternatives.
  - `tripwire@agent-skills@audit-defaults-overruled`: read at the harvest; its premise, the
    calibration and the sort giving defaults the owner accepts, holds for both axes.
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

### One pass costs about the owner's budget of 15 agents `##c6`

Weighed, from the owner's words, round 1: "the budget should be around 15 subagents I think." Met by
`thread@agentic-workflow-axis@workflow-axis-lenses`.

## Threads

Every thread was proposed by the agent in round 1 and approved by the owner in round 2.

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
"The corpus". Harvest: a head, if it earns one; the line on the developer-contracts issue. Closed,
round 2: "workflow-axis-corpus: approved." The default for the three scoped restatements was part of
the thread as presented, and the agent's round-2 reply stated that the approval accepted it.

### Every auditor reads the whole corpus, through one of six lenses, each lens dispatched at least once, with no calibration sample `##workflow-axis-lenses`

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
head, if it earns one; otherwise the skill's section and a `%%` line. Closed, round 2:
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

### The axis's first run is in the branch that builds it `##workflow-axis-first-run`

Arguments: `argument@agentic-workflow-axis@a1`, `argument@agentic-workflow-axis@a31`,
`argument@agentic-workflow-axis@a32`, `argument@agentic-workflow-axis@a33`. Shape: Implementation
sequence, step 4. Harvest: none; a choice of this work, carried by its commits. Closed, round 2:
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
"`design@agent-skills@audit-triggers` lets you put the run into another work's branch."

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

### One agent with six lenses may flatten them `##a38`

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

## New names, in one place

```text
skill section   skill@knowledge-architect-project-audit@agentic-workflow-axis
                  in path@agent-skills@content/skills/project-audit/SKILL.md, as {{slug:agentic-workflow-axis}}
agent           knowledge-architect-workflow-auditor
                  source  planned@agent-skills@content/agents/workflow-auditor.md
                  install planned@agent-config@agents/knowledge-architect-workflow-auditor.md
lenses          L1 contradiction, L2 activity walk, L3 two readings, L4 provable gap,
                L5 restatement, L6 local against installed   (in the agent and the section)
tags            W, P, I                                       (in the agent and the section)
```

The section slugs of the new agent are its own, chosen at step 2.

## Decided design

### The generic method

`thread@agentic-workflow-axis@audit-method-per-axis`. The audit skill's method states what holds for
every axis:

1. the corpus, which the axis's section names;
2. the dispatch, which the axis's section gives: how the corpus is shared out, how many agents, and
   how they are calibrated, if they are;
3. the pre-commitments, as point 3 states them today;
4. the drafts in files, each agent with a scratch directory of its own, editing nothing else, and
   read by the session, not summarised;
5. the sort: no finding is applied before a reading confirms it, and a quotation of the owner is
   confirmed against the question it answered;
6. one owner list per pass, items labelled `F<n>`, nothing on it applied before the owner answers;
7. the application, the whole diff read before each commit;
8. the re-check by fresh agents of the axis, dispatched as the axis gives, with the kept list;
   passes repeat while the violations left fall, until the owner judges what is left small enough;
9. the review, on the axes listed today.

The design-record axis's section gains its dispatch: groups of at most 60 entries of one Component
in the order its agent gives, the calibration sample and its settlement, at least one conforming
draft read per group at the sort, the drafting of a split, a merge or a move before it is applied
with each created head judged alone, and the re-check over the same groups. AC2 checks that no rule
is lost.

The nearest rival, the method kept and the new axis written as its exception, loses because the
skill's method would state one rule that one of its two axes breaks, `argument@agentic-workflow-axis@a19`.

### The corpus

`thread@agentic-workflow-axis@workflow-axis-corpus`. Every text the harness delivers to a session as
an instruction: the root CLAUDE.md and the primer it imports, the installed skills and agents, and the
project's own skills and agents. The shipped text names no provider's directory. Out of it: the
scoped CLAUDE.md files, and the registers; the auditors read the issue registers of the agent
configuration and of the workflow, through `cargo klarch issues`, as findings already known. An
auditor may run read-only commands to check what the corpus claims about the checker or the harness.

### The lenses

`thread@agentic-workflow-axis@workflow-axis-lenses`. Every auditor reads the whole corpus. Each takes
one lens:

| lens | looks for | default agents |
| --- | --- | --- |
| L1 contradiction | two instructions, in any files, that no single move satisfies; a term defined two ways | 3 |
| L2 activity walk | one activity followed across files, listing at each moment the instructions that apply: two that conflict, a hand-off to something no text defines, a pointer to a section that does not exist, a loop with no exit | 4, one per group of activities: design, planning and building; review and merge; audit and retrospective; setup and pin move |
| L3 two readings | an instruction whose words allow two readings that lead to different acts | 2 |
| L4 provable gap | per "The finding standard" | 2 |
| L5 restatement | each restatement against its home: drifted, longer than a pointer, or without its pointer | 2 |
| L6 local against installed | the project's own text against the installed text: a contradiction, a duplicate, a routing-table row that says the wrong thing | 2 |

Each lens is dispatched at least once; the count is the session's, within the owner's budget, and the
table's 15 is the stated default. There is no calibration sample: the sort reads every finding, the
duplicates of a lens add coverage, and the re-check reads again. A finding two agents report is
merged and noted as such.

The nearest rival, the corpus shared out among agents of one task, loses on
`argument@agentic-workflow-axis@a25`. The rejected alternative "An audit method with no calibration
and no re-check" does not cover this shape: its recorded reason is two agents reading one shared
entry two ways, where a method trusts one reading per entry; this axis gives no verdict per entry,
and trusts no one reading, since a finding is confirmed at the sort and the re-check reads again.

### The finding standard

`thread@agentic-workflow-axis@workflow-axis-finding-standard`. A finding is admitted when the
session can confirm it by reading:

- **a contradiction**: both instructions quoted, and the moment at which both apply;
- **two readings**: the instruction quoted, and the two acts it allows;
- **a factual error**: the claim quoted, and the command that shows it false;
- **a provable gap**: a requirement of the corpus or of the checker, and an input to it that no text
  supplies and that judgement cannot derive, since it is a fact fixed elsewhere, such as a format the
  checker enforces, a name or a path, not a choice the agent could make;
- **a predicted gap**: "an agent might misjudge X": an issue stating what a real session would have
  to show, never an edit.

The rival, every gap a finding to fill, fails `criterion@agentic-workflow-axis@c4` and
`goal@agent-skills@installed-text-leaves-room-to-judge`.

### The outcomes

`thread@agentic-workflow-axis@workflow-axis-outcomes`. Each finding carries a tag, W, P or I.

| | in an adopting project | in the project that publishes the workflow |
| --- | --- | --- |
| W | never edited locally; written into a file in the shape of the retrospective's workflow file, which goes upstream on the owner's word, as `skill@knowledge-architect-retrospective@what-becomes-of-files` says | edited at its source |
| P | applied, put to the owner, or an issue | the same |
| I | each side as its own tag says | the same |

The installed text says "the project that publishes the workflow" in words, naming no repository.
Outcomes:

- **applied**: a rewording that changes no instruction; a broken pointer; a factual error; a
  restatement replaced by a pointer or brought back to its home;
- **owner list**: which side of a contradiction wins; a repair of a two-reading instruction that
  changes what agents are told; a provable gap;
- **issue**: a predicted gap; a finding the owner defers.

### The workflow auditor

`thread@agentic-workflow-axis@workflow-auditor-agent`. One installed agent,
`knowledge-architect-workflow-auditor`, tools Read, Grep, Glob, Bash, read-only in its body. It holds:

- the standard: it edits nothing; it writes only to its scratch directory; every quotation is copied
  from the file; every command listed was run;
- its brief: the commit audited, its lens, and for L2 its group of activities, its scratch directory,
  and for a re-check the kept list;
- the corpus, named as the axis's section names it, every file read whole;
- `cargo klarch issues` run before drafting, a finding already recorded marked with its reference;
- one section per lens, the brief naming which applies;
- the finding standard and the tags;
- one draft per finding: lens, tag, class, the quotations with their files and lines, the moment or
  the two acts or the command, the proposed repair, the proposed outcome;
- the return: the corpus files read whole, one line per draft, the commands run.

The nearest rival, one agent per lens, loses on `argument@agentic-workflow-axis@a29`.

## Mapping tables

Empty: the work needs no total function over existing things.

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
rejected; at the harvest it is re-pointed to the head that keeps the calibration, since the generic
method no longer holds it.

## Readings

Empty: the work reads no external specification.

## Premortem

Each cause was put to the owner in round 2 under its label; the owner, round 3: "record all four,
apply the three acceptance criteria." The owner added AC4 in the same message; its observable was
worded by the agent in its round-3 reply.

| label | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| T1 | Duplicate agents add nothing | `thread@agentic-workflow-axis@workflow-axis-lenses` | tripwire. Fires when, in one run, every lens dispatched more than once gets no confirmed finding from its second or later agents that its first missed. Response: reopen the duplicate dispatch. Re-entry: the sort of each run counts it, the first in this branch |
| T2 | The drafts are mostly noise | `thread@agentic-workflow-axis@workflow-axis-finding-standard` | tripwire. Fires when, in one run, more than half of the drafts fail the session's reading; the bound is the owner's to reset. Response: reopen the finding standard and the auditor's standard. Re-entry: the sort of each run |
| T3 | The corpus outgrows one agent's reading | `thread@agentic-workflow-axis@workflow-axis-corpus`, `thread@agentic-workflow-axis@workflow-axis-lenses` | tripwire. Fires when an auditor's return says it did not read every corpus file whole. Response: reopen how the corpus is shared out. Re-entry: the sort of each run. Also AC1 |
| T4 | One agent with six lenses flattens them | `thread@agentic-workflow-axis@workflow-auditor-agent` | tripwire. Fires when, in one run, more than a third of one agent's drafts carry a lens other than its brief's; the bound is the owner's to reset. Response: reopen one agent against one agent per lens. Re-entry: the sort of each run |
| AC1 | (cause 3) | `thread@agentic-workflow-axis@workflow-axis-corpus` | acceptance criterion, `acceptance@agentic-workflow-axis@files-read-and-drafts-tagged` |
| AC2 | The relocation loses a rule of the design-record method | `thread@agentic-workflow-axis@audit-method-per-axis` | acceptance criterion, `acceptance@agentic-workflow-axis@design-record-dispatch-kept` |
| AC3 | Shipped text cites an entry of this repository | `thread@agentic-workflow-axis@workflow-auditor-agent` | acceptance criterion, `acceptance@agentic-workflow-axis@shipped-text-cites-no-entry` |
| AC4 | The audit does not push the workflow toward improvement | `thread@agentic-workflow-axis@workflow-axis-lenses`, `thread@agentic-workflow-axis@workflow-axis-finding-standard` | acceptance criterion, `acceptance@agentic-workflow-axis@passes-fall` |

## Acceptance criteria

### Every auditor reads the whole corpus, and every draft names its lens and its tag `##files-read-and-drafts-tagged`

AC1. Guards `thread@agentic-workflow-axis@workflow-axis-corpus`. Judged at step 4, on every return
and every draft of the first run. Fires when a return omits a corpus file from its list of files read
whole, or a draft has no lens or no tag. Response: reopen how the corpus is shared out, and repair
the agent's return before the next pass.

### The design-record axis keeps every rule its method held `##design-record-dispatch-kept`

AC2. Guards `thread@agentic-workflow-axis@audit-method-per-axis`. Judged at step 1, against the diff:
the bound of 60 entries and its arithmetic, the order of a Component's entries, the calibration
sample and its settlement, at least one conforming draft read per group, the drafting of a split, a
merge or a move with each created head judged alone, and the re-check over the same groups are each
stated after the change, in the generic method or the design-record axis's section. Fires when one is
missing. Response: restore it before the next step.

### The new section and the new agent cite no entry of this repository `##shipped-text-cites-no-entry`

AC3. Guards `thread@agentic-workflow-axis@workflow-auditor-agent` and
`thread@agentic-workflow-axis@audit-method-per-axis`. Judged at steps 1 and 2: outside a `%%` line,
the edited section, the new section and the new agent hold no reference of the kinds `design`,
`issue`, `tripwire`, `goal` or `path` to this repository. Fires on one. Response: move it to a `%%`
line or remove it.

### Over at least three passes, the confirmed findings and the severe ones fall from each pass to the next `##passes-fall`

AC4, proposed by the owner in round 3. Guards `thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`. Judged at step 4. The instrument: the
count of confirmed findings of each pass, and the count of severe ones, both written in that pass's
commit message. Fires when, over at least three passes, either count does not fall from a pass to the
next. Response: before the merge, reopen `thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`.

## Implementation sequence

Steps 1 and 2 change the installed text: each commit runs `cargo klarch install-agent-skills` and
commits the installed copies with their source. Each passes the edit tests of
`path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent". Every commit runs
`cargo klarch check --staged` before it and `cargo klarch commits origin/main..HEAD` after it.

1. **The generic method.** The audit skill's method made generic, per "The generic method"; the
   design-record axis's section gains its dispatch; the design-record auditor's description checked
   against it. Judges AC2 and AC3. Fails alone on: a rule of the design-record method lost.
2. **The axis and its agent.** The workflow auditor, per "The workflow auditor"; the axis's section
   of the audit skill, per "The corpus", "The lenses", "The finding standard" and "The outcomes"; the
   audit skill's frontmatter description and scope naming the axis. The commit converts the spec's
   `planned` citations. Judges AC3. Fails alone on: an agent whose brief cannot be filled from the
   section, or a section that cannot be dispatched from.
3. **The changelog.** The CHANGELOG.md entries the tests of `design@knowledge-architect@changelog-entries`
   owe for steps 1 and 2; `cargo x changelog`. Fails alone on: an entry's class.
4. **The first run.** The axis runs on this repository under the skill, in this branch, at the
   commit of step 3. Passes repeat as the generic method says. Each pass's commit message carries the
   counts AC4 reads and the observations T1 to T4 read. The owner list waits for the owner's answers.
   Lessons on the method found by the run amend the section or the agent in their own commits. Judges
   AC1 and AC4. Fails alone on: the counts of AC4.
5. **The harvest.** The rows of "Harvest"; the spec deleted.

## Order rationale

- 1 before 2: the new section is written against the generic method.
- 2 before 3: the entries describe the shipped text as it lands.
- 3 before 4: the run reads the corpus as it will ship, the changelog aside.
- 4 before 5: the harvest records the decisions after AC4 is judged and the run's lessons are in.

## Defaults awaiting the owner

None.

## Harvest

Every head lands in `path@agent-skills@docs/design.md`, unless the row says otherwise. Whether a
thread earns a head is decided by the entry tests of `primer@design-heads`.

| item | where it lands |
| --- | --- |
| `thread@agentic-workflow-axis@audit-method-per-axis` | `design@agent-skills@audit-method` rewritten to the generic method; `design@agent-skills@design-record-axis` gains the dispatch |
| `thread@agentic-workflow-axis@workflow-axis-corpus` | a head, if it earns one; a line on `issue@core@a-home-for-developer-contracts-outside-agent-configuration` naming the three scoped restatements |
| `thread@agentic-workflow-axis@workflow-axis-lenses` | a head |
| `thread@agentic-workflow-axis@workflow-axis-finding-standard` | a head, if it earns one; otherwise a `%%` line at the section |
| `thread@agentic-workflow-axis@workflow-axis-outcomes` | a head, if it earns one; otherwise a `%%` line at the section |
| `thread@agentic-workflow-axis@workflow-auditor-agent` | a head, if it earns one beside `design@agent-skills@audit-agent-per-axis` |
| `thread@agentic-workflow-axis@workflow-axis-first-run` | none: carried by the run's commits |
| T1, T2, T3, T4 | `path@agent-skills@docs/tripwires.md`, each naming its head, with its label |
| AC1 to AC4 | reported in the landing commit; one that recurs is proposed as a tripwire |
| "An audit method with no calibration and no re-check" | re-pointed to the head that keeps the calibration |
| every other item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | rewritten to the axes it still holds |

## Later consequences

- The other axes of `issue@agent-skills@audit-axes-beyond-the-design-record` each give their own
  dispatch under the generic method.
- A pin move across the version that ships the axis may cite it in a Migration entry, if the version
  changes rules the corpus must meet.
