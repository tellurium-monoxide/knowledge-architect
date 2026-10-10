# A project audit reads the agentic workflow as one whole, with auditors of varied tasks that each read all of it

## Status and audience

This spec is the plan of the work that gives the project audit its second axis, the agentic-workflow
axis, and runs it on this repository. The work does seven things:

- the method of `skill@knowledge-architect-project-audit` becomes generic, and the dispatch of the
  design-record axis, its groups and its calibration sample, moves into that axis's section;
- a new installed agent, the workflow auditor, holds the standard of the new axis;
- a new section of the audit skill holds the axis: its corpus, its lenses, its finding standard and
  its outcomes;
- the axis runs on this repository, in this branch, which judges AC4 and counts T1 to T4;
- the axis repairs by cause, with a workflow script beside its prose, per "The repair by cause" and
  "The workflow script";
- the findings of passes 2 and 3 are repaired by cause, and a fourth pass runs through the script;
- a pass runs as one run of the axis's saved workflow, which delivers a report the session verifies
  after the run, the generic method takes that shape for every axis, the axis gains a lens that
  compares the review standards with the audit standards, and a fifth pass uses it, per "The
  one-run pass" and "The verdict-alignment lens".

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec and its work land on one branch**, `agentic-workflow-audit-axis`. One step changes the
  checker: the installer's namespace gains saved workflows, which no earlier commit's tree holds,
  and `commits` compares no installed file, so the per-commit gate judges every earlier commit as
  before.
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
  reply to the re-review message, which opens "All defaults approved.". Passes 1 and 2 of the first
  run put their owner lists in messages that open "Pass 1 of the agentic-workflow axis: the sort,
  and the owner list" and "Pass 2 is sorted"; the owner's replies open "F1: agreed" and "All
  defaults approved for F28 to F57.", and their rulings are in the passes' commit messages. "Round
  7" is the owner's message that opens "Hmm, this is interesting." and the agent's reply that opens
  "A premise to correct first"; "round 8" is the owner's message that opens "audit-net-growth: I
  don't think word count is the correct metric" and the reply that opens "F37 to F57 aside"; "round
  9" is the owner's message "audit-net-growth approved with that shape, proceed as proposed."; "the
  third review message" is the agent's message that opens "All four reviews are back", which put D16
  to D21 to the owner, and "round 10" is the owner's reply, which opens "After updating the spec and
  implementing the workflow". "Round 11" is the owner's message that opens "I think it would be
best to deliver the workflow as a file". "Round 12" is the owner's message that opens "Wait before
the harvest. Honestly, I'm not fully satisfied", the owner's next message, "I think we should
carry the discussion about this under the design skill once again.", and the agent's reply that
opens "## Round 1: one run of the workflow for a whole pass"; "round 13" is the owner's message
that opens "The cross check and the report should be one agent each" and the reply that opens
"## Round 2"; "round 14" is the owner's message that opens "audit-axis-runs-a-workflow: agreed on
this shape" and the reply that opens "## Round 3"; "round 15" is the owner's message
"preflight-step approved; record T5, T6, apply AC5, AC6". "Round 16" is the owner's message that
opens "Hmm, AC6 looks a bit too vague" and the reply that opens "## Round 4"; "round 17" is the
owner's message "verdict-alignment-lens approved.", which came while the session applied the
review repairs; "round 18" is the owner's message "D23 to D27 approved, run the re-review". The
replies of rounds 12 to 14 call the
criteria of that discussion C1 to C6; they are the items `criterion@agentic-workflow-axis@c7` to
`criterion@agentic-workflow-axis@c12`, in order. An extraction agent wrote the owner messages,
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
- **a lens**: one task an auditor is briefed with, L1 to L8, per
  `thread@agentic-workflow-axis@workflow-axis-lenses`. The word is the agent's, round 1.
- **a draft**: one auditor's file for one finding, per "The workflow auditor".
- **a tag**: the side of a finding, `W` for the installed text, `P` for the project's own text, `I`
  for an interaction of the two. W and P are the letters of the retrospective's findings,
  `skill@knowledge-architect-retrospective@what-it-examines`. A tag classifies a finding; it is not a
  label of an item the owner rules on, so `design@agent-skills@ruled-items-labelled` gains no prefix.
- **the owner list**: as the audit skill's terms define it,
  `skill@knowledge-architect-project-audit@audit-terms`.
- **a pass**: one round of the method: the drafts, the sort, the owner list, the application; for
  this axis also the cluster, the repair proposals and their confirmation, between the sort and the
  owner list. The re-check's drafts start the next pass. Today the skill uses the word in point 7 and point 9 and
  does not define it; step 1 adds it to the terms.
- **the re-check**: fresh agents of the axis, dispatched as the axis's first dispatch was, after a
  pass's edits. For this axis: the same lenses with the same count per lens as the first dispatch,
  and the same groups of activities for L2.
- **confirmed**: a finding that survives the session's reading: at the sort, before the one-run
  pass; in the one-run pass, a finding of the confirmed-findings file the session's verification
  does not reject, whatever its verdict there, the session reading the findings "The one-run pass"
  names and leaving the others as they are.
- **verified**: the verdict the verify step of the one-run pass gives a draft it fails to refute.
- **an audit**: the passes of one audit of the axis, from its first dispatch until the owner stops
  it; an audit holds several passes. "A run" alone is one run of the saved workflow, one pass.
- **pipeline** and **barrier**: `pipeline()` is a function of the Workflow tool that runs each item
  through every stage on its own, so one item can be at a later stage while another is at an
  earlier one; a barrier is a pattern, an awaited `parallel()` that waits for every item before the
  next stage starts. The tool runs at most min(16, available CPUs - 2) agents at once in one
  workflow, and queues the rest. Source: the tool's description and its authoring reference, the
  skill `workflow-authoring` of Claude Code, which a session of that harness lists.
- **a violation left**, for this axis: a confirmed finding of a re-check that is not on the kept
  list. It is what the generic stop rule counts.
- **severe**: a confirmed finding whose outcome is the owner list, whatever its class.
- **a cause**: what produced one or more confirmed findings, of one of the four kinds of "The
  repair by cause".
- **an instruction**: a sentence that tells an agent to do, or not to do, an act at a moment. A
  rewording that makes an existing instruction's intent clearer adds none, whatever its length.
- **the run's scratch directory**: one directory the session chooses per run, holding each agent's
  scratch directory, the confirmed-findings file and the causes file, whose formats are in "The
  repair by cause".
- **the Workflow tool**: Claude Code's tool that runs a JavaScript script orchestrating subagents. A
  script begins with a literal `export const meta`; it reads no file and no clock; it takes its
  inputs in `args`; it calls `agent(prompt, opts)`, where `opts` may name a `schema` the agent's
  return must match and an `agentType`, `parallel()`, a barrier, and `pipeline()`; a top-level
  `return` gives its result. The tool runs only on an explicit opt-in: among its cases, the owner
  invoking a skill whose instructions tell the session to call it, and the owner asking to run a
  named saved workflow. A **saved
  workflow** is a script file directly under .claude/workflows/, run by the name its `meta`
  declares; the harness lists it with the skills. Source: the tool's description and its
  authoring reference, read in this session, and the probes of `argument@agentic-workflow-axis@a57`.
- **the run**: one invocation of the axis's saved workflow, which covers a whole pass from the
  preflight to the report, per "The one-run pass". Its scratch directory is the scratch root that
  `args` name.
- **the steps**: the steps of the run, in order: preflight, drafts, verify, dedup, cluster, merge,
  repairs, cross-check, report.
- **the report**: report.md, which the report step writes in its directory of the run's scratch
  directory, holding
  the six parts of "The one-run pass".
- **the counts**: `args.counts` of the run, the number of auditors per lens; the lens table's
  values, 17 auditors in all with L8, when the session passes none. Before the one-run pass, this was the
  owner's budget, a total the session shared out.

## What the work is

### What exists today

This section states the sites as they stood at the spec's first commit, before step 1. Steps 1 to 6
have since changed them; what step 7 rewrites, as it stands now, is the subject of the mapping
table "Step 7: the audit skill's sections to the one-run pass".

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
- Found again by the grounding search of round 12, for the one-run pass:
  `tripwire@agent-skills@audit-defaults-overruled` watches the owner list the report's draft
  feeds, every default of steps 5 and 6 having been accepted; `tripwire@agent-skills@owner-intent-stripped`
  fires if the rewrite of `design@agent-skills@audit-method` drops the owner's quoted stop rule,
  which the harvest keeps; `issue@agent-skills@audit-axes-beyond-the-design-record` keeps "an audit
  report" as "a question for later" for an axis whose fixes are code, which this axis's are not, so
  the report of the one-run pass is a working file of the session and the commit messages stay the
  record of a run; and the lesson of the owner's sample of verdicts, which
  `thread@agentic-workflow-axis@session-verifies-the-report` adopts as the session's sample of
  refuted drafts; `issue@agent-skills@a-reviewer-s-running-time-is-unbounded` bears on one long
  run, which logs its progress per step with the tool's `log()` and `phase()`;
  `issue@core@a-project-workflow-can-declare-an-installed-workflow-s-name` bears on every saved
  workflow the generic shape adds: each takes the installer's prefix in its declared name, and the
  entry's question is unchanged.
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
- The design-record axis's workflow: its own design work, per
  `thread@agentic-workflow-axis@audit-axis-runs-a-workflow`; step 7 opens the `todo` issue that
  holds it.

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
  reading names an instruction read two ways, a provable gap and, per D30, divergent verdicts over a
  text of the project, and its first sentence, "Real use
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
  on the installed text, and its restatement in `instructions@repository-skills` says so. Per D15,
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
`thread@agentic-workflow-axis@workflow-axis-lenses`: each lens at least once, the count per lens the
session's, passed to the run as the counts, per `thread@agentic-workflow-axis@lens-table-in-the-script`.

### Only findings provable by reading lead to an edit `##c4`

Binding as a presumption, from `design@agent-skills@additions-need-real-use`. Met by
`thread@agentic-workflow-axis@workflow-axis-finding-standard`.

### The axis has an installed agent of its own `##c5`

Binding as a presumption, from `design@agent-skills@audit-agent-per-axis`. Met by
`thread@agentic-workflow-axis@workflow-auditor-agent`.

### One pass costs about the owner's budget of 15 agents, in the owner's words `##c6`

Weighed, from the owner's words, round 1: "the budget should be around 15 subagents I think."
Unmet-and-accepted. Before the one-run pass, `thread@agentic-workflow-axis@workflow-axis-lenses`
met it with 16 agents per D8. The
one-run pass dispatches 17 auditors by default, with L8, and adds 22 agents to a pass: a preflight,
17 verifiers, a dedup, a merge, a cross-check and a report. Round 12 stated "About 18 more agents
per pass", without the preflight, the merge and the report, and D26 put about 21; the owner
approved `thread@agentic-workflow-axis@one-run-pass` with the first and D26 with the second, round
18: "D23 to D27 approved", which marks this criterion unmet-and-accepted.

### Nothing is applied before the session's reading, and the owner's word where the outcome needs it `##c7`

Binding, from `design@agent-skills@audit-method` and `goal@knowledge-architect@the-owner-decides`;
C1 of round 12. Met by `thread@agentic-workflow-axis@session-verifies-the-report` and
`thread@agentic-workflow-axis@apply-stays-with-the-session`.

### A finding is used only for the part a reading confirms `##c8`

Binding, from `design@agent-skills@synthetic-evidence-not-built`; C2 of round 12. Met by
`thread@agentic-workflow-axis@verifier-in-the-run` and
`thread@agentic-workflow-axis@session-verifies-the-report`.

### The method stays usable by a session without the Workflow tool `##c9`

Round 12 worded C3 "The prose method stays the method"; round 14's checkpoint reworded it to this
title, since `thread@agentic-workflow-axis@manual-fallback-home` makes the script the home of the
steps. The rewording follows from that approved thread; the owner gave no word on it.

Binding as a presumption, from `thread@agentic-workflow-axis@audit-harness`; C3 of round 12. Met by
`thread@agentic-workflow-axis@manual-fallback-home`.

### The session's burden during a pass is small `##c10`

Weighed, from the owner's words, round 12: "minimizing the burden placed on the main session"; C4
of round 12. Met by `thread@agentic-workflow-axis@one-run-pass`,
`thread@agentic-workflow-axis@lens-table-in-the-script` and
`thread@agentic-workflow-axis@preflight-step`.

### The report is enough to discuss with the owner and act on `##c11`

Weighed, from the owner's words, round 12: "a bit more thorough to be at the level where the main
session can discuss with the owner and act"; C5 of round 12. Met by
`thread@agentic-workflow-axis@report-contents`; AC6, which would have judged it, was declined by
the owner in round 16, who reads pass 5 instead.

### The counts AC4 reads stay comparable from pass to pass `##c12`

Weighed; C6 of round 12. Met by `thread@agentic-workflow-axis@session-verifies-the-report`: the
counts are taken after the session's reading, as pass 4's were after its sort.

## Threads

The first seven threads were proposed by the agent in round 1 and approved by the owner in round 2;
#checker-rules-lens was proposed by the owner in round 4 and approved in round 5. The next six were
raised in round 7 and closed in rounds 8 and 9; the last ten were raised in rounds 12 to 16 and
closed in rounds 13 to 17, each as its entry says.

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

### A finding is repaired at the cause that produced it, not at its site `##root-cause-repair`

Proposed by the owner in round 7: "search for root causes, not local fixes"; shaped by the agent in
its reply, which named the four kinds of cause. Arguments: `argument@agentic-workflow-axis@a47`, `argument@agentic-workflow-axis@a48`,
`argument@agentic-workflow-axis@a49`. Shape: Decided design, "The repair by cause". Harvest: a head.
Closed, round 8: "For the other threads, I agree with your positions."

### Every instruction about one moment or one kind of act lives in one home `##moment-homes`

Proposed by the owner in round 7: "try to centralize instructions that pertain to the same moment or
type of action (like we did for the decision record rules)". Argument:
`argument@agentic-workflow-axis@a50`. Shape: a repair kind of "The repair by cause"; the homes
themselves are what step 5 lands, from the causes it confirms, per round 10. Harvest: a head, if a
home lands. Closed, round 8, as above.

### A second phase of agents clusters the confirmed findings by cause and proposes a repair per cause `##audit-synthesis-phase`

Proposed by the owner in round 7: "the audit method for this axis should include a second pass of
dispatching subagents, this time focused on classifying the issues and proposing fixes". Arguments:
`argument@agentic-workflow-axis@a51`. Shape: Decided design, "The repair by cause"; its agents per
D16. Harvest: within the head of `thread@agentic-workflow-axis@root-cause-repair`. Closed, round 8.

### The method stays readable for subagents alone, and ships a workflow script used where the Workflow tool exists `##audit-harness`

Raised by the owner in round 7: "I'm starting to consider going back on my goal of being harness
agnostic". The agent corrected the premise: no goal names a harness; `design@agent-skills@audit-method`
reads `goal@agent-skills@installed-text-works-anywhere` more widely than its words. Arguments:
`argument@agentic-workflow-axis@a52`, `argument@agentic-workflow-axis@a53`. Shape: Decided design,
"The workflow script". Harvest: `design@agent-skills@audit-method` loses its "runs under any harness"
argument; a head. Closed, round 8. Round 11: the script ships as a saved workflow, per
`argument@agentic-workflow-axis@a57`. Rounds 12 to 15 change its shape: a pass is one run of the
script, per `thread@agentic-workflow-axis@one-run-pass`, and the script, not the prose, is the
home of the steps, per `thread@agentic-workflow-axis@manual-fallback-home`.

### A pass whose repairs add more instructions than they remove goes to the owner with its net figure `##audit-net-growth`

Proposed by the agent in round 7 as a bound on words; the owner, round 8: "I don't think word count
is the correct metric here ... It should probably apply to "more instructions" rather than more
words. Though monitoring net word count change produced by this process is still interesting, and
should at least be done and recorded in commit messages of audit repairs." Arguments:
`argument@agentic-workflow-axis@a54`, `argument@agentic-workflow-axis@a55`. Shape: Decided design,
"The repair by cause". Harvest: a head of its own, per D20.
Closed, round 9: "audit-net-growth approved with that shape, proceed as proposed."

### Pass 2's additions that only spell out a judgement agents already make are reverted `##revisit-pass-2`

Proposed by the agent in round 7 from the owner's observation after pass 2. Argument:
`argument@agentic-workflow-axis@a56`. Shape: step 5 gives the clusters stage pass 2's additions the
owner observed as judged correctly already, each as a finding of the kind "a rule written for one
interaction", per round 10. Harvest: none; step 5's commits. Closed, round 8.

### The axis's first run is in the branch that builds it `##workflow-axis-first-run`

Arguments: `argument@agentic-workflow-axis@a1`, `argument@agentic-workflow-axis@a31`,
`argument@agentic-workflow-axis@a32`, `argument@agentic-workflow-axis@a33`. Shape: Implementation
sequence, step 3. Harvest: none; a choice of this work, carried by its commits. Closed, round 2:
"workflow-axis-first-run: agreed to run in the same branch."

### A pass runs as one run of the axis's saved workflow, from its preflight to its report, with no reading by the session between steps `##one-run-pass`

Proposed by the owner in round 12: "the main power of that tool is to organize multiple complex
steps that produce a complete result (such as a report) which is as ready as possible for the main
session to dicuss it with the owner, then act on it, while minimizing the burden placed on the
main session", and "verification by the session is indeed necessary, but not in the middle of the
workflow steps". It reopens the three-stage shape of `thread@agentic-workflow-axis@audit-harness`.
Arguments: `argument@agentic-workflow-axis@a51`, `argument@agentic-workflow-axis@a58`. Shape:
Decided design, "The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 13, the owner: "Agreed on your shapes
for all 5 threads. That matches what I had in mind."

### One adversarial verifier per auditor confirms or refutes its drafts inside the run `##verifier-in-the-run`

Proposed by the agent in round 12, as what `thread@agentic-workflow-axis@one-run-pass` needs.
Arguments: `argument@agentic-workflow-axis@a58`, `argument@agentic-workflow-axis@a59`. Shape:
Decided design, "The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 13, by the same words.

### The report holds the six parts that let the session discuss and act without reopening drafts `##report-contents`

Proposed by the agent in round 12. Shape: Decided design, "The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 13, by the same words, with the
owner's detail: "The cross check and the report should be one agent each (two total)."

### The session verifies the report after the run, before the owner list `##session-verifies-the-report`

Proposed by the agent in round 12. Arguments: `argument@agentic-workflow-axis@a58`. Shape: Decided
design, "The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 13, by the
same words, with the owner's change: "Sample left to judgement", where round 12 proposed a sample
chosen by a stated rule.

### The session applies the approved repairs, and no workflow does `##apply-stays-with-the-session`

Proposed by the agent in round 12. Arguments: `argument@agentic-workflow-axis@a60`. Shape: Decided
design, "The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 13,
by the same words.

### Every audit axis runs a saved workflow of its own that delivers a report, and the generic method takes that shape `##audit-axis-runs-a-workflow`

Proposed by the owner in round 13: "ultimately, I think it should become the generic shape of every
audit axis, with every audit axis having its own dynamic workflow that produces a ready to discuss
and act report. That would also make the project audit skill much more concise." Arguments:
`argument@agentic-workflow-axis@a61`. Shape: Decided design, "The generic shape of an axis". The
design-record axis's workflow is a `todo` issue, its own design work. Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 14, the owner: "audit-axis-runs-a-workflow: agreed
on this shape, agreed on a todo issue for the design-record axis, which is its own piece of design
work."

### The saved workflow is the home of the manual fallback `##manual-fallback-home`

Raised by the owner in round 13: "The detailed descriptions needed for a "manual fallback" outside
of Claude code harness could be relocated somewhere else, one markdown file per axis." The agent
proposed the script itself as that home. Arguments: `argument@agentic-workflow-axis@a62`. Shape:
Decided design, "The generic shape of an axis". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 14, the owner: "manual-fallback-home:
agreed on shape 2."

### The lens table lives in the script, and the run takes a count per lens `##lens-table-in-the-script`

Proposed by the agent in round 13; the owner changed its input in round 14. Arguments:
`argument@agentic-workflow-axis@a63`, `argument@agentic-workflow-axis@a64`. Shape: Decided design,
"The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`.
Closed, round 14, the owner: "I think it's a good thing indeed for the lens table to move directly
into the workflow rather than passed as args. this is indeed quite error prone. Also, as argument,
it would probably be better to pass a list of count per lens, rather than a total budget (which
needs to be dispatched by the script rather than by the judgement of the session). Using our
current values as the defaults."

### A preflight agent sets up the run: the auditors' worktrees, their kept lists and the issues listing `##preflight-step`

Proposed by the agent in round 14. Arguments: `argument@agentic-workflow-axis@a65`. Shape: Decided
design, "The one-run pass". Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 15, the owner: "preflight-step
approved".

### A lens of the agentic-workflow axis reads every review standard and audit standard that judge one subject, for verdicts that diverge `##verdict-alignment-lens`

Proposed by the owner in round 16: "it is quite important that the reviews and the audit axis are
aligned together in what they consider findings. I'm thinking that maybe this point deserves its own
lens in the workflow audit." Arguments: `argument@agentic-workflow-axis@a66`,
`argument@agentic-workflow-axis@a67`. Shape: Decided design, "The verdict-alignment lens".
Harvest: per the Harvest row of the one-run pass's threads, under `design@agent-skills@one-decision-per-head`. Closed, round 17,
the owner: "verdict-alignment-lens approved."

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

### Local repairs add rules, and added rules interact `##a47`

Round 7, owner. Bears on `thread@agentic-workflow-axis@root-cause-repair`. "Indeed, as
capabilities-not-structure records, adding instructions to handle corner cases is always more likely
to give rise to incompatible interactions, simply due to the shear amount of rules. I think we might
need to define a more thorough method to repair incompatible and contradicting instructions.
Solving them locally, as the current audit method encourages, is not viable."

### The run measured it: pass 3 found about 25 interactions between the rules passes 1 and 2 added `##a48`

Round 7, agent. Bears on `thread@agentic-workflow-axis@root-cause-repair`. Passes 1 and 2 applied 63
findings and the owner ruled 57; pass 3's lenses found about 25 findings in the interactions of the
rules they added, by the auditors' returns before the sort, and the
contradiction lens "no strict contradiction ... that is not on the kept list or recorded".
`design@agent-skills@capabilities-not-structure` records the same effect: "34 edits produced 10
interaction defects".

### The principle already existed, and the audit's outcome table bypassed it `##a49`

Round 7, agent. Bears on `thread@agentic-workflow-axis@root-cause-repair`.
`design@agent-skills@capabilities-not-structure` and
`goal@agent-skills@installed-text-leaves-room-to-judge` answer a conflict a structure causes "by
removing structure rather than by adding a rule"; the axis's outcome table gave each finding its own
repair at its site.

### One home per moment, as the rules on design heads were gathered `##a50`

Round 7, owner and agent. Bears on `thread@agentic-workflow-axis@moment-homes`. The owner: "like we
did for the decision record rules"; the agent named the moments where pass 3's findings cluster:
the review before a merge, the transcript, labels, commits and history edits, what a subagent does
with what it meets outside its task.

### One session sorting 160 drafts reads each at its site, and misses the cause that crosses them `##a51`

Round 7, agent. Bears on `thread@agentic-workflow-axis@audit-synthesis-phase`. The session's sort
dropped a pass-1 finding and left the F23 ruling off pass 2's kept list; both were its own slips.
The owner: "I think we are touching on the limits of using only subagents. These kind of tasks are
much better handled by a dynamic workflow".

### No goal makes the project harness-agnostic `##a52`

Round 7, agent. Bears on `thread@agentic-workflow-axis@audit-harness`. `grep -i "harness\|provider\|claude
code"` over every goals home returns nothing; `goal@agent-skills@installed-text-works-anywhere`
says "a project of any language and layout". The one sentence that binds the audit to any harness
is `design@agent-skills@audit-method`'s "so it runs under any harness". Other records keep a second
provider open as a design target, `design@knowledge-architect@binary-bundles-workflow`,
`design@core@agents-table` and `issue@core@configuration-for-several-agent-providers`: a script
used where the tool exists, beside a prose method, keeps it open.

### A workflow fixes the order and the inputs of every agent, and the owner uses Claude Code only `##a53`

Round 7, agent and owner. Bears on `thread@agentic-workflow-axis@audit-harness`. A script runs the
dispatch, the clustering, the repair proposals and the re-check in a fixed order, and its prompts are
versioned with it. The owner: "After all, I use only Claude Code personally... I'm not really
expecting anyone to start using this, outside of myself in more projects than thaum."

### A word count penalises a clarifying rewording and is unknown before the repair `##a54`

Round 8, owner. Bears on `thread@agentic-workflow-axis@audit-net-growth`. "Repairing a sentence to
make intent clearer may need adding words locally, and this should not need my ruling. The net
growth is hard to account for before the fact, too."

### An instruction is countable before the edit `##a55`

Round 8, agent. Bears on `thread@agentic-workflow-axis@audit-net-growth`. Each repair proposal
states the instructions it adds and removes, so the net is known when the owner list is written; the
net words are measured after the edit, for monitoring.

### Agents already judged most of pass 2's items correctly `##a56`

The owner, on pass 2's owner list: "from my observation and experience working with the installed
workflow, for most of those items, agents were making the correct judgements already and going with
these defaults you propose." Bears on `thread@agentic-workflow-axis@revisit-pass-2`.

### A saved workflow is called by its declared name, from .claude/workflows/ alone `##a57`

Round 11, owner and agent. Bears on `thread@agentic-workflow-axis@audit-harness`. The owner: "I
think it would be best to deliver the workflow as a file directly in .claude/workflows/ (it would
need to use the knowledge-architect prefix too, for namespacing)." Three probe files under
.claude/workflows/, after a reload of the harness's skills: a file whose `meta` declares
`probe-saved-workflow` ran by that name, its `args` passed through; the file `probe-file-name.js`,
declaring `probe-meta-name`, ran as `probe-meta-name` and was "not found" as `probe-file-name`; a
file one directory deeper was "not found". Saved workflows appear in the skill listing with their
description. So the prefix namespaces a workflow only when its declared name carries it, and the
file sits directly under the directory. A fenced block passed inline is read by the checker but
lives in a skill as text no session needs to read; a saved workflow is run by name, with nothing to
copy. The owner, on the proposal stating these consequences: "Agreed, proceed with the delivery as
proposed".

### The readings between the stages of pass 4 changed little, and cost the session the bulk of its context `##a58`

Round 12, agent. Bears on `thread@agentic-workflow-axis@one-run-pass`,
`thread@agentic-workflow-axis@verifier-in-the-run` and
`thread@agentic-workflow-axis@session-verifies-the-report`. Measured on pass 4, the files in its
scratch directory, `wc -l` over each set: the sort read 63 drafts, 3,738 lines, and confirmed 36
findings, 60 drafts merged into them, 14 findings merging two to six drafts each, and refuted 3;
the merge read two groupings, 1,074 lines, and made two choices of boundary; the confirmation read
23 proposals, 4,030 lines, and, by the session's account, sent none back, which no file records;
the diff read before the commit was 1,290 lines, and the session rewrapped three lines and softened
one claim. Of the 3 refutations, 2 were mechanical, a kept item and a pair of texts one move
satisfies, and 1 rested on the history of the audit, a pointer the step-5 cause F66 had judged; a
verifier given the audit's commit messages can make all three. The three stages ran 574 s, 297 s
and 321 s, by the harness's notices of each run's end in the session's transcript; round 12 put a
one-run pass at about 20 minutes with the verify step, and the owner approved it with that cost.
One pass's figures: evidence, not proof. Argument a51 had already pointed here, and the script of
step 4 still left the sort to the session.

### A verifier per auditor reads the corpus once per auditor, and sees that auditor's drafts together `##a59`

Round 12, agent. Bears on `thread@agentic-workflow-axis@verifier-in-the-run`. The rival, one
verifier per draft, 63 agents in pass 4, reads the whole corpus once per draft; one per auditor
reads it 16 times, and the sort's rule, "A draft found wrong sends the same agent's other drafts
of the same class back to a reading", applies within one verifier.

### Applying repairs merges overlapping edits against a tree that may have moved `##a60`

Round 12, agent. Bears on `thread@agentic-workflow-axis@apply-stays-with-the-session`. In steps 5
and 6 the applying agents merged repairs that edited one paragraph six times, three in each, by
their reports, which are messages in the session's transcript and on no other record; round 12
counted four. The session softened one claim in the diff. Two runs of
evidence: the position is held loosely.

### Each axis's section keeps only what the session needs to verify a report and write the owner list `##a61`

Round 13, agent and owner. Bears on `thread@agentic-workflow-axis@audit-axis-runs-a-workflow`. The
session reads the axis's corpus, rules and outcome table to verify a report; the dispatch tables,
the prompts and the order of the steps live in the script. The owner: "That would also make the
project audit skill much more concise." The design-record axis's workflow is design work of its
own, its groups of 60, its calibration sample and its drafts per entry; its section stays until it
exists, since moving its prose now and again then would relocate it twice.

### The prompts and the order of the steps already have one home, the script `##a62`

Round 13, agent. Bears on `thread@agentic-workflow-axis@manual-fallback-home`. A markdown file per
axis restating the steps would be a second home for one moment, against
`thread@agentic-workflow-axis@moment-homes`; pass 4 confirmed such a drift, the script's repair
prompt narrowing "the homes merged into one" to "the section of the skill that owns the moment",
which F81 of pass 4's commit repaired. The rival's one advantage, that the checker reads
markdown and no JavaScript, bears on references, and the script's prompts cite no entry. A file per
axis stays available for reading guidance that is not a step.

### Sixteen agent entries written by hand were the largest and most error-prone input of a run `##a63`

Round 13, agent and owner. Bears on `thread@agentic-workflow-axis@lens-table-in-the-script`. In
pass 4 the session wrote the drafts stage's `args` as sixteen entries, each with a lens, a group,
a scratch directory and a kept list, the four L2 entries alone with a group; the session wrote
L2's second group as "review and merge", where the installed skill says "development, review and
merge", the kind of slip the thread removes. The owner, round 14: "this is indeed quite error prone."

### A count per lens keeps the choice with the session, where a total budget leaves it to the script `##a64`

Round 14, owner. Bears on `thread@agentic-workflow-axis@lens-table-in-the-script`: "it would
probably be better to pass a list of count per lens, rather than a total budget (which needs to be
dispatched by the script rather than by the judgement of the session)."

### Each auditor's own worktree is set up inside the run, where the script names the auditors `##a65`

Round 14, agent. Bears on `thread@agentic-workflow-axis@preflight-step`. F92 of pass 4's commit
gives every auditor a worktree of its own, detached at the commit audited, and the script step 7
builds makes the auditors' labels from the counts; a session creating the worktrees before the run would first rebuild those
labels, the hand input `thread@agentic-workflow-axis@lens-table-in-the-script` removes. The rival,
one worktree shared by every auditor, each building into its own scratch directory, contradicts
F92's "its own worktree", approved one round earlier.

### A misalignment between a review and an audit is no contradiction, since no single session obeys both standards `##a66`

Round 16, agent. Bears on `thread@agentic-workflow-axis@verdict-alignment-lens`. L1's class is "two
instructions that no single move satisfies": one session, one moment. A review passes a text at the
merge and an audit reports it later, or the reverse; each standard can be obeyed on its own, so the
misalignment fails L1's admission test, and an L1 agent that meets one has no class to draft it
under. Its cost is a cycle across time: a review approves, an audit rejects, a repair lands, and
the next review rejects the repair.

### The general lenses never compared a review's standard with an audit's, and a deliberate comparison once found a misalignment `##a67`

Round 16, agent. Bears on `thread@agentic-workflow-axis@verdict-alignment-lens`. The owner's lead,
in `issue@agent-skills@audit-axes-beyond-the-design-record`: "an audit and a review that share a
subject are given compatible instructions for the verdicts they give". A comparison made on it after
the design-record axis's first run found that axis's auditor and the decision-record reviewer
"judging rejected alternatives by two different restatements of one rule, one of which reported as a
defect what the other must not", which the commit "[review] The decision-record reviewer and
the design-record auditor judge the record by the same rules" aligned. Over the draft files of passes
1 to 4 in the session's scratch directory, the eight whose titles pair "review" with "audit" are about which axes review an audit's branch and
when the transcript review runs; none compares what the two count as a finding.

## New names, in one place

```text
skill section   skill@knowledge-architect-project-audit@agentic-workflow-axis
                  in path@agent-skills@content/skills/project-audit/SKILL.md, as {{slug:agentic-workflow-axis}}
agent           knowledge-architect-workflow-auditor
                  source  path@agent-skills@content/agents/workflow-auditor.md
                  install path@agent-config@agents/knowledge-architect-workflow-auditor.md
lenses          L1 contradiction, L2 activity walk, L3 two readings, L4 provable gap,
                L5 restatement, L6 local against installed, L7 checker rules,
                L8 verdict alignment
finding class   divergent verdicts                            (in the agent and the section)
verdicts        verified, refuted, plausible; the mark bears-on-ruling   (in the script's verify prompt and the agent's verify section)
run inputs      commit, scratch, worktrees, kept, history, counts { L1 … L8, clusters, repairsPerCause }, from
                                                              (in the script; their exact identifiers are step 7's)
step files      verdicts.md, confirmed.md, grouping.md, causes.md, proposal.md, cross-check.md, report.md,
                and each auditor's return, under the run's scratch directory
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
| L2 activity walk | one activity followed across files, listing at each moment the instructions that apply: two that conflict, a hand-off to something no text defines, a pointer to a section that does not exist, a loop with no exit | 4, one per group of activities, per D13: design, planning, decision recording and issue tracking; development, review and merge, as the installed skill words it; audit and retrospective; setup, pin move, goal setting and agent configuration |
| L3 two readings | an instruction whose words allow two readings that lead to different acts | 2 |
| L4 provable gap | per "The finding standard" | 2 |
| L5 restatement | each restatement against its home: one that drifted from it, or one without its pointer, per D3 | 1 |
| L6 local against installed | the project's own text against the installed text: a contradiction, a duplicate | 2 |
| L7 checker rules | per "The checker-rules lens", narrowed by F69 of step 5: a rule no installed text states is reported only when its finding message does not name its repair | 2 |
| L8 verdict alignment | per "The verdict-alignment lens" | 1 |

Each lens is dispatched at least once. The count per lens is the session's, passed as the counts,
and the table's values, 17 in all, are the defaults, per
`thread@agentic-workflow-axis@lens-table-in-the-script`. An L2 agent's brief names its group of activities; two L2 agents
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
own and, for a re-check, the kept list; then its drafts are read, never its summary alone. Its body holds:

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

### The repair by cause

`thread@agentic-workflow-axis@root-cause-repair`, `thread@agentic-workflow-axis@audit-synthesis-phase`,
`thread@agentic-workflow-axis@audit-net-growth`. The axis's pass, from point 5 of the generic method
on:

1. **The sort** confirms each finding by a reading, as today, and writes the confirmed-findings file:
   one line per finding, its label, its tag, and the path of its draft.
2. **The cluster.** Cluster agents, two by default, each read the file, every draft it names and the
   whole corpus, and group the findings by cause. A cause is one of these:
   - **two homes for one moment**: instructions about one moment or one kind of act stated in more
     than one text;
   - **a structure the work did not need**: a fixed sequence, count or mapping that another
     instruction needs to vary;
   - **a restatement that drifted from its home**;
   - **a rule written for one interaction**: a narrow rule where judgement already decides.

   A finding that fits none stands alone. Each group names its kind, its mechanism in one sentence,
   and the texts that state an instruction about its moment. The agents run independently; the
   session merges their groupings into the causes file, one section per cause, and where they
   differ it reads the findings concerned and chooses. The session may merge or split causes.
3. **The repair proposals.** One repair agent per cause by default reads the cause, its drafts and
   the whole corpus, and proposes one repair for the cause, never one per finding. The repair by
   kind: merge the homes into one and point to it from the others, per
   `thread@agentic-workflow-axis@moment-homes`; remove the structure; bring the restatement back to
   a pointer; delete the narrow rule and leave the case to judgement. It adds an instruction only
   where none of these closes the cause, and says why. Its proposal states the edits, the findings
   each closes, and the instructions it adds and removes, counted. In a project that is not the
   workflow's upstream, a cause whose findings are on both sides is repaired in two parts: the
   installed side is sent upstream, the project side takes its outcome here.
4. **The confirmation.** The session reads each proposal against its cause and the corpus, as point
   5 of the generic method reads a draft. A proposal found wrong is corrected, or sent back to a
   repair agent.
5. **The owner list** holds one item per cause, under its label `F<n>`, with the labels of its
   findings, the repair, its instruction count and a default. A cause whose repair the axis's
   outcome table lists as applied in the branch is applied without the owner's word; any other goes
   on the list. Each finding takes the outcome of its cause's repair. **When the repairs of a pass,
   taken together, add more instructions than they remove, the list says so first, with the net
   figure**, and the owner rules on the pass's growth before any repair of the pass is applied.
6. **Application** as point 7 of the generic method. The commit message of each pass gives the
   corpus's word count before and after, measured with `wc -w` over the corpus's files, and the
   instructions added and removed, from the proposals.

In the one-run pass, points 1, 2 and 4 are steps of the run, done by agents, and the session's
reading moves after the run, per "The one-run pass".

The finding classes and the tags stand.

### The workflow script

`thread@agentic-workflow-axis@audit-harness`. The script for the Workflow tool ships as a saved
workflow: its source is `path@agent-skills@content/workflows/agentic-workflow-audit.js`, installed
as .claude/workflows/knowledge-architect-agentic-workflow-audit.js, its `meta` declaring that stem
as its name, per `argument@agentic-workflow-axis@a57`. The installer's namespace holds it, so the
install writes and removes it and `check` compares it, as it does a skill or an agent. Where the
harness lists it, the session runs it by name, once per pass, as "The one-run pass" says.

**Its file.** The checker reads no JavaScript, so a reference in the script is checked by nobody,
and it cites none. The build fails unless the declared name equals the installed stem. Its `meta` is
a pure literal, which the tool requires, so its description is one string. It is checked with
`node --check` on its body wrapped in `async function body() {…}` with `export` removed, since a
top-level `return` fails a bare check, as `path@agent-skills@CLAUDE.md` says.

### The one-run pass

`thread@agentic-workflow-axis@one-run-pass`, `thread@agentic-workflow-axis@verifier-in-the-run`,
`thread@agentic-workflow-axis@report-contents`, `thread@agentic-workflow-axis@session-verifies-the-report`,
`thread@agentic-workflow-axis@apply-stays-with-the-session`,
`thread@agentic-workflow-axis@lens-table-in-the-script`, `thread@agentic-workflow-axis@preflight-step`.
A pass of the axis is one run of its saved workflow. The session reads nothing between its steps;
it verifies the report after the run. Every step writes its files in the run's scratch directory,
so the session can open any draft, grouping or proposal the report cites.

```
            ┌──── one run of knowledge-architect-agentic-workflow-audit ────┐
args ──►  preflight (1 agent: worktrees, kept lists, issues listing)
           ▼
          drafts (one auditor per lens and count)
           │ pipeline: each auditor's drafts go to its verifier at once
           ▼
          verify (1 adversarial verifier per auditor)
           ▼  barrier: needs every verdict
          dedup (1 agent: the confirmed-findings file)
           ▼
          cluster (2 agents) ──► merge (1 agent: the causes file, its choices stated)
           ▼
          repairs (1 agent per cause)
           ▼  barrier
          cross-check (1 agent)
           ▼
          report (1 agent: report.md)
            └──────────────────────────────────────────────────────────────┘
                    ▼
          session: verifies the report ──► owner list ──► the owner's word ──► application
```

This diagram illustrates the shape; the script is its home.

**Inputs**, in `args`: the commit audited; the scratch root, the run's scratch directory, under
which each agent gets a directory of its own; the worktree root, under which the preflight creates
the worktrees; for a re-check, the kept list's path; the history, a git range whose commit messages
record the audit's earlier passes, which the session names, per D29; `counts`, optional, an object keyed by lens id
whose defaults are the lens table's values, L1 3, L2 4, L3 2, L4 2, L5 1, L6 2, L7 2, L8 1, and
which also takes `clusters`, default 2, and `repairsPerCause`, default 1, per D25; and `from`,
optional, which restarts the chain at the verify, cluster or repairs step, per "Files, ids and
returns". A
lens missing from `counts` takes its default; a count below 1 is refused, since each lens is
dispatched at least once; L2's count is its number of groups by default, a count above it adds
walkers on the same groups in turn, and a count below it walks the first groups in the table's order
and the report names the groups left unwalked, per D27. The lens table lives in the script: each
lens's id, name, default count and, for L2, its groups of activities, copied at step 7 from the
installed skill's table as it stands, whose L2 groups read "design, planning, decision recording
and issue tracking", "development, review and merge", "audit and retrospective", "setup, pin move,
goal setting and agent configuration". What each lens looks for stays in the workflow auditor's
section `agent@knowledge-architect-workflow-auditor@the-lenses`, its one home.

**The preflight** creates each auditor's worktree, detached at the commit audited, per
`skill@knowledge-architect-review@review-invariants`, and one more worktree at that commit, the
reading worktree, for the steps that only read; writes the kept list, where `args` name one, into
each auditor's scratch directory; saves, in its own scratch directory, the issues listing at that
commit, run in the reading worktree with the project's command as the install writes it into the
script, and the messages of the commits of the history `args` name, for the auditors and the
verifiers. Each worktree is under the worktree root, and is removed by
the session once the pass's findings have their outcomes, as the invariant says.

**The verify step**: one adversarial verifier per auditor, the workflow auditor dispatched with a
verify brief, per D24, in its auditor's worktree, which the auditor has finished with, the pipeline
handing over each auditor's drafts when it returns. It tries to refute each draft. It checks every
quotation against the worktree; reads the kept list, the issues listing and the messages of the
audit's earlier passes; refutes a draft whose finding the owner already ruled on, naming the
commit; marks known a draft whose finding an issue entry records, naming the entry, and kept a draft
on the kept list, both going on with their outcome under the axis's table; checks a quotation of
the owner against the question it answered, in its source; checks that the moment the draft
describes exists; and tests the finding's class against the auditor's definitions. A draft whose finding is that a
ruling's premise has proved false is not settled by the verifier: it is marked `bears-on-ruling`,
for the session. Its verdict per draft is verified, refuted or plausible, with a reason, and it
splits a finding tagged `I` into its W side and its P side, each with one outcome. A refuted draft
is kept with its reason; verified, plausible, `bears-on-ruling`, known and kept drafts go on to the
dedup, each marked. The verifier returns in the shape the workflow auditor's verify section gives,
per D24. Each auditor's own return, with its files read, its "Outside the axis" and its "Met outside
the task", is saved in its directory, for T3 and the report.

**The dedup step**, one agent, reads every verdicts file and writes the confirmed-findings file,
merging the drafts that report one finding: one line per finding, its id, its tag, its verdict, the
path of its draft and the ids merged into it.

**Files, ids and returns.** The script reads no file, so every step writes its files under the
scratch root and returns their paths and ids, which the script hands to the next step. A draft's
id is its auditor's label and its file name, as `L1-a/d1-<slug>.md`; a finding keeps the id of the
draft the dedup kept. Each verifier writes `verdicts.md` in its directory; the dedup writes
`confirmed.md`; each cluster agent `grouping.md`; the merge `causes.md`, and returns the cause ids;
each repair agent `proposal.md`; the cross-check `cross-check.md`; the report `report.md`, each
under the step's directory of the scratch root. `from: 'verify'` restarts from the drafts in the auditors' directories; `from: 'cluster'` restarts
from `confirmed.md`;
`from: 'repairs'` from `causes.md`, through one agent that reads it and returns the cause ids. Every
agent but the auditors and the verifiers is a general-purpose agent whose prompt is in the script,
as D16 decided for the cluster and repair agents. Each step logs its counts with the tool's `log()`
and groups its agents with `phase()`.

**The cluster and merge steps**: two cluster agents group the confirmed findings by cause, as "The
repair by cause" says; one merge agent writes the causes file from both groupings, and states each
choice it made where they differ.

**The repairs step**: one repair agent per cause, as "The repair by cause" says.

**The cross-check step**, one agent, reads every proposal against the others and the corpus: the
paragraphs several repairs edit, the instruction counts summed, the outcome it proposes for each
cause under the axis's outcome table, which the session decides, per D31, and the findings that
bear on an earlier ruling of the owner, with the ruling quoted and the premise that may be false.

**The report step**, one agent, writes report.md in its directory of the run's scratch directory,
holding:

1. the header: the commit audited; the agents per step; the drafts, verified, after dedup and
   refuted, each refuted draft with its reason;
2. the figures: the run's counts, from which the session takes the counts AC4 reads after its
   verification, and the instructions added and removed, from the proposals;
3. one section per cause: its kind and its one-sentence mechanism; its findings, each with its
   quoted evidence, the verifier's verdict and note, and its tag; the repair as edits before and
   after, per file; the instructions it adds and removes, quoted; its outcome under the axis's
   table, and the default; a finding marked `bears-on-ruling`, with the ruling quoted and the
   premise that may be false;
4. the overlap map: each paragraph several repairs edit, and the causes concerned;
5. the draft of the owner list, its labels left for the session to number per
   `primer@labelled-items`;
6. what the run did not decide: the merge's choices, the plausible verdicts, and every step that
   failed or returned nothing.

**The reading steps** (dedup, cluster, merge, repairs, cross-check, report) read the corpus in the
reading worktree and run no command of the checker.

**The session's verification**, after the run: it reads the whole report; every proposal in full,
since the owner list goes out under its responsibility; every finding marked `bears-on-ruling` or
plausible, against its draft; and a sample of the refuted drafts, chosen by its judgement. Then it
writes the owner list. The counts AC4 reads are taken after this reading.

**The application** stays with the session, after the owner's word where the outcome needs it, as
steps 5 and 6 did it:
agents on disjoint files, every repair merged where two edit one paragraph, the whole diff read
before the commit.

**Without the Workflow tool**, or where the harness does not list the saved workflow, the session
dispatches the same steps by hand, in the script's order, with the script's prompts, read from the
script.

Nearest rivals, each with the fact that defeated it: the three runs of step 4 with a reading
between each, by `argument@agentic-workflow-axis@a58`; a verifier per draft, by
`argument@agentic-workflow-axis@a59`; the session reading every draft again after the run, by
`argument@agentic-workflow-axis@a58`, since the verifier with the run's history makes the
refutations pass 4's sort made; a second saved workflow that applies the approved repairs, by
`argument@agentic-workflow-axis@a60`; one worktree shared by every auditor, by
`argument@agentic-workflow-axis@a65`.

### The verdict-alignment lens

`thread@agentic-workflow-axis@verdict-alignment-lens`. A lens L8, verdict alignment, default count
1. For each subject that a review axis and an audit axis both judge, its auditor reads the two
standards side by side, and reports a text that one standard reports as a finding and the other
must not report, or treats as conforming. Its brief names these pairs as a floor, not a limit:

- the decision-record reviewer and the design-record auditor: design heads, rejected alternatives;
- the routing reviewer and the workflow auditor's L5 and L6: restatements, pointers;
- the review's self-consistency axis and L1: contradictions;
- the standing-state reviewer and the audits' reading of tripwires and issues;
- the review skill's outcomes and the audit's outcome table.

It judges every audit agent the corpus holds, the design-record auditor included, not only this
axis's. A new finding class, **divergent verdicts**: the draft names one text of the project and
quotes the sentence of each standard that judges it, the two giving different verdicts; a reading
confirms it. A written example is not admitted, since `design@agent-skills@additions-need-real-use`
lets no synthetic scenario originate an edit, per D30. Its outcome is the owner list, as a contradiction's is:
which standard wins is the owner's. The pairs are written in the workflow auditor's L8 text, the one
home of what each lens looks for. Step 7 adds the lens and the class to every list that names the
lenses or the classes: the script's table, the workflow auditor's lenses, finding classes and brief
("one of L1 to L7"), the skill's lens list and its "The rules".

Nearest rival: the class with no lens, any lens drafting under it, by
`argument@agentic-workflow-axis@a67`: the general lenses never compared two standards unprompted.

### The generic shape of an axis

`thread@agentic-workflow-axis@audit-axis-runs-a-workflow`,
`thread@agentic-workflow-axis@manual-fallback-home`. The generic method of the audit skill takes the
one-run pass as its shape for every axis: run the axis's saved workflow; verify the report; put the
owner list; apply on the owner's word; the next run is the re-check. Each axis's section keeps what
the session needs to verify a report and write the owner list, its corpus, its rules and its
outcome table, and names its saved workflow and the `args` it takes. The dispatch tables, the
prompts and the order of the steps live in the script, which is also the home of the manual
fallback: the skill keeps one sentence that a session without the tool dispatches the script's
steps by hand, in its order, with its prompts. A per-axis markdown file is written only for reading
guidance that is not a step; this axis has none.

The design-record axis has no workflow yet. Until its workflow is designed, which a `todo` issue
holds, its section stays as it is, as round 13 put it to the owner, and the generic method keeps
its points in place, under a sentence that an axis with no saved workflow runs them, per D28. The
one-run shape is stated beside them for an axis with a workflow. Step 7 judges AC2 again over the
rewrite.

Nearest rival: one markdown file per axis holding the manual fallback, the owner's first shape, by
`argument@agentic-workflow-axis@a62`.

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

### Step 7: the audit skill's sections to the one-run pass

Each text of `path@agent-skills@content/skills/project-audit/SKILL.md` and of the workflow auditor
that step 7 rewrites, as it stands at the amendment's commit, and where it goes.

| today's text | after step 7 |
| --- | --- |
| The frontmatter description's method, "(the corpus, a dispatch each axis gives, drafts in files, the sort, one list for the owner, the application, a fresh re-check, the review)" | names both shapes: the run of an axis's workflow and the session's verification of its report, or the manual points of an axis with no workflow |
| Terms, the row "pass": "the drafts, the sort, the owner list, the application" | a pass is one run of the axis's workflow, the session's verification of its report, the owner list and the application; for an axis with no workflow, the drafts, the sort, the owner list and the application |
| Terms, the rows "dispatch", "draft" and "the re-check" | kept, each read for both shapes: an axis's workflow dispatches its agents; the next run is the re-check |
| The method, points 1 to 9 | kept in place, per D28, under one sentence: an axis with no saved workflow runs them; an axis with one runs its pass as one run, verifies the report, puts the owner list, applies, and its next run is the re-check |
| The method, point 3's "which point 8 reads" | kept |
| The method, point 5's check of a quotation of the owner against the question it answered | kept for the manual points; the verify step of the one-run pass carries it |
| The method, point 8: the stop rule, the violation left, the question when the count does not fall, the issue for an item the owner does not rule on | kept, and stated for both shapes |
| The record of a run | kept: the commit messages are the record; a pass's report is a working file, and its outcomes go in the pass's commit message, which gives the count of confirmed findings and of those on the owner list, moved here from "The sort" |
| The agentic-workflow axis: the corpus, the rules, what is not in it, the agent | kept, "The rules" naming the class divergent verdicts |
| The agentic-workflow axis: the dispatch, with its lens table and the budget | the lenses listed by id and name, L1 to L8, and the counts named as the workflow's input; what each looks for stays in the workflow auditor's lenses; the default counts and L2's groups live in the script; the budget sentences gone |
| The sort | gone: the verify and dedup steps of the workflow, and the session's verification of the report; its last sentence, the counts each pass's commit message gives, moves to "The record of a run" |
| The repair by cause, points 1 to 3 | its kinds and its repair forms kept, as the rules the session verifies proposals against; the cluster, the repair proposals and the confirmation are steps of the workflow and of the session's verification |
| The cause's outcome: installed side sent upstream, project side applied when every edit falls in the applied row; an item of the owner list is one cause | kept, with the cross-check proposing and the session deciding, per D31 |
| The net-growth rule and the word count | kept |
| The outcomes of this axis | kept, with the class divergent verdicts on the owner list |
| Sent upstream: "one file per run, written when the run stops" | kept, "one file per audit, written when the audit stops" |
| The workflow script, with its stages table | the saved workflow named, its `args` listed, and one sentence for a session without the tool |
| The workflow auditor: its body's opening, "The session that sent you reads your drafts, sorts each finding" | the session or the verify step reads its drafts |
| The workflow auditor: its brief's "one of L1 to L7" and its `{{command}} issues` before drafting | "one of L1 to L8"; it reads the preflight's issues listing where its brief names one |
| The workflow auditor: its return | kept for a draft brief; a verify section gains its own return, per D24 |

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

- **One repair per finding, at its site**: lost to
  `thread@agentic-workflow-axis@root-cause-repair`, on `argument@agentic-workflow-axis@a48`.
- **Moments stated across texts, their restatements policed by L5**: lost to
  `thread@agentic-workflow-axis@moment-homes`: L5 finds a drift only after it happens; one home
  leaves nothing to drift.
- **The session synthesises the findings alone**: lost to
  `thread@agentic-workflow-axis@audit-synthesis-phase`, on `argument@agentic-workflow-axis@a51`.
- **A skill written for the Workflow tool only**: lost to
  `thread@agentic-workflow-axis@audit-harness`: it fails in every other harness, and adds to
  `issue@core@configuration-for-several-agent-providers` a dependency no text-only skill has, for a
  gain the script beside the prose already gives.
- **One workflow auditing every axis of a project at once, delivering one report**, the shape of
  the example the owner showed in round 12, `path@elsewhere@~/Documents/code/endless-engrenage/docs/reviews/2026-07-30-matrix-review-3.workflow.js`,
  from which the verifier, the refuted drafts kept with their reason and the preflight of "The
  one-run pass" are taken: lost to `thread@agentic-workflow-axis@audit-axis-runs-a-workflow`, on the
  owner's words, round 12: "it audits all axes of the project at once and produces a single report,
  which is too large of a scope".
- **A class of divergent verdicts with no lens of its own**: lost to
  `thread@agentic-workflow-axis@verdict-alignment-lens`, on `argument@agentic-workflow-axis@a67`.
- **Three runs of the workflow per pass, the session reading between each**: lost to
  `thread@agentic-workflow-axis@one-run-pass`, on `argument@agentic-workflow-axis@a58`.
- **One verifier per draft**: lost to `thread@agentic-workflow-axis@verifier-in-the-run`, on
  `argument@agentic-workflow-axis@a59`.
- **The session reading every draft again after the run**: lost to
  `thread@agentic-workflow-axis@session-verifies-the-report`, on `argument@agentic-workflow-axis@a58`.
- **A second saved workflow that applies the approved repairs**: lost to
  `thread@agentic-workflow-axis@apply-stays-with-the-session`, on `argument@agentic-workflow-axis@a60`.
- **One markdown file per axis holding the manual fallback**: lost to
  `thread@agentic-workflow-axis@manual-fallback-home`, on `argument@agentic-workflow-axis@a62`.
- **A total budget the script shares out among the lenses**: lost to
  `thread@agentic-workflow-axis@lens-table-in-the-script`, on `argument@agentic-workflow-axis@a64`.
- **One worktree shared by every auditor**: lost to `thread@agentic-workflow-axis@preflight-step`,
  on `argument@agentic-workflow-axis@a65`.
- **The script in a fenced block of the skill, passed to the tool inline**: lost to
  `thread@agentic-workflow-axis@audit-harness`, on `argument@agentic-workflow-axis@a57`.
- **A bound on words**: lost to `thread@agentic-workflow-axis@audit-net-growth`, on
  `argument@agentic-workflow-axis@a54`.
- **No bound on a pass's growth**: lost to `thread@agentic-workflow-axis@audit-net-growth`: passes 1
  and 2 turned most findings into added rules, which pass 3 then found interacting.

The recorded rejected alternative "An audit method with no calibration and no re-check" stays
rejected. At the harvest it names both heads it lost to: the generic method, which keeps the
re-check, and `design@agent-skills@design-record-axis`, which keeps the calibration.

## Readings

Empty: the work reads no external specification.

## Premortem

The premortem of rounds 12 to 15 put T5, T6, AC5 and AC6 to the owner in round 14; the owner,
round 15: "record T5, T6, apply AC5, AC6". In round 16 the owner declined AC6: "AC6 looks a bit
too vague to carry much meaning IMO. I'd rather just wait for the pass and see then what happens."
Its causes are the rows from T5 to the one after AC6, and the row of round 16.

Each cause was put to the owner in round 2 under its label; the owner, round 3: "record all four,
apply the three acceptance criteria." The owner proposed AC4 in the same message; its wording is
D1. T1's firing condition is reworded here from round 2's, since "its first" agent names no agent
when agents are dispatched together; the cause it watches is unchanged. T2 and T4 count "in one
pass" where round 2 said "in one run", since an audit holds several passes. The bounds of T1 to T4
are the ones round 2 put to the owner and the owner ruled to record.

In the one-run pass, the re-entry "the sort of each pass" of T1 to T4 is the session's verification
of each pass's report, and a "run" in T1, T2 and T4 is one pass; the harvest writes them so, a
rewording listed to the owner at that turn.

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
| T5 | The verifiers refute true findings, and the session sees only a sample of what they refute | `thread@agentic-workflow-axis@verifier-in-the-run` | tripwire. Fires when the session's sample of refuted drafts finds one that a reading confirms, in any pass. Response: reopen `thread@agentic-workflow-axis@verifier-in-the-run`. Re-entry: the session's verification of every report |
| T6 | The verifiers wave findings through, and the session sorts again after the run | `thread@agentic-workflow-axis@verifier-in-the-run` | tripwire. Fires when the session rejects more than a quarter of a report's confirmed findings in one pass; the bound is the owner's to reset. Response: reopen `thread@agentic-workflow-axis@verifier-in-the-run`. Re-entry: the session's verification of every report |
| AC5 | The run dies part-way, and the report covers half a pass as if whole | `thread@agentic-workflow-axis@one-run-pass` | acceptance criterion, `acceptance@agentic-workflow-axis@one-run-delivers` |
| (AC6, declined) | The report compresses the evidence, and the session reopens drafts for most causes | `thread@agentic-workflow-axis@report-contents` | watched by the owner's reading of pass 5, with no criterion, the owner's word, round 16 |
| (cause 5 of round 14) | The merge agent draws a cause boundary wrongly, and one proposal mixes two mechanisms | `thread@agentic-workflow-axis@one-run-pass` | survives into a named clause: the session reads every proposal in full, per `thread@agentic-workflow-axis@session-verifies-the-report`, and the report lists the merge's choices |
| (round 16) | L8 drafts pairs of standards that differ in wording and give one verdict | `thread@agentic-workflow-axis@verdict-alignment-lens` | survives into T2, which fires when more than half of a pass's drafts fail the reading |
| AC4 | The audit does not push the workflow toward improvement | `thread@agentic-workflow-axis@workflow-axis-lenses`, `thread@agentic-workflow-axis@workflow-axis-finding-standard` | acceptance criterion, `acceptance@agentic-workflow-axis@passes-fall` |

## Acceptance criteria

### Every auditor reads the whole corpus, and every draft names its lens and its tag `##files-read-and-drafts-tagged`

AC1. Withdrawn per D10, ruled in round 6: its observable judges a new agent's report against its
instructions, which `design@agent-skills@synthetic-evidence-not-built` refuses by name. T3 keeps
watching the cause, on what an auditor's return says. Step 3 does not judge it.

### The design-record axis keeps every rule its method held `##design-record-dispatch-kept`

Step 7 judges it again, over the rewrite of the generic method.


AC2. Guards `thread@agentic-workflow-axis@audit-method-per-axis`. Judged at step 1, against the diff,
row by row of the mapping table: every row of today's text is stated after the change, at the place
the table gives. The design-record auditor's body, which restates the groups and the sample, is read
with it and stays true. Fires when one is missing. Response: restore it before the next step.

### The shipped text the branch writes cites no entry of this repository `##shipped-text-cites-no-entry`

AC3. Guards `thread@agentic-workflow-axis@workflow-auditor-agent` and
`thread@agentic-workflow-axis@audit-method-per-axis`. Judged at steps 1 to 8, per D7 and step 7, on every
line the branch adds under `path@agent-skills@content/` and `path@agent-skills@snippets/`. The
instrument:
``git diff origin/main -- crates/agent-skills/content crates/agent-skills/snippets | grep '^+' | grep -v '^+%%' | grep -E '`(design|issue|tripwire|goal|spec|milestone)@[a-z0-9-]+@[a-z0-9]'``
prints nothing. It matches a reference whose id starts with a letter or a digit, so a placeholder
in angle brackets and a section of the design skill, `skill@knowledge-architect-design@<slug>`, do
not match; over the whole shipped text at the spec's revision it prints nothing. Fires on a line it prints. Response: move it to a `%%` line or remove it.

### The confirmed findings and the severe ones fall from each pass to the next `##passes-fall`

AC4, proposed by the owner in round 3; its wording is D1, ruled in round 4. It fired on passes 1 to
3, per "AC4's firing on the first run", and is judged again at step 6. Guards
`thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`. Judged at step 3, read as an
observation of real use per D2. The instrument: the count of confirmed findings of each pass, and
the count of severe ones, both written in that pass's commit message. Fires when, from one pass to
the next, the confirmed findings do not fall, or the severe ones do not fall while they are above
zero. It is judged over every pass the audit makes; the passes stop as the generic method says, on the
owner's judgement. Response: before the merge, reopen
`thread@agentic-workflow-axis@workflow-axis-lenses` and
`thread@agentic-workflow-axis@workflow-axis-finding-standard`.

### A pass runs from its preflight to its report in one run `##one-run-delivers`

AC5, applied by the owner in round 15. Guards `thread@agentic-workflow-axis@one-run-pass`. Judged
at step 8. The observable: pass 5 runs from the preflight to the report in one run of the saved
workflow, its steps completing in order. Fires when a step fails and the run is restarted with
`args.from`; a restart is reported as such. Response: reopen
`thread@agentic-workflow-axis@one-run-pass`. The observable is the run's mechanics, which
`design@agent-skills@synthetic-evidence-not-built` admits; whether the session acts between steps
is not observed, per D23. Pass 5 is real use, as D9 ruled the first run, its repairs landing in this
repository, per D23.

## AC4's firing on the first run

| pass | confirmed | severe, to the owner list | source |
| --- | --- | --- | --- |
| 1 | 54 | 27 | the pass-1 commit's message |
| 2 | 70 | 30 | the pass-2 commit's message |
| 3 | about 70 | about 35 | the pass-3 commit's message, from the auditors' returns before a full reading |
| 3, read at step 5 | 48 | not counted | the step-5 commit's message: the open drafts confirmed by quotation and claim |
| 4 | 36 | 9 | the pass-4 commit's message: the owner list showed all 23 causes, 14 of them listed for visibility with the outcome applied in the branch under the axis's table; the 9 others, F86 to F94, held 9 findings |

Neither count fell, so AC4 fired. During pass 2's sort the owner had written: "I think it would be
very hard to find a proper way to score severity for comparison. But here, I'm not sure we should
consider AC4 triggered." The response was taken before the merge: the owner read the cause in round
7, "Solving them locally, as the current audit method encourages, is not viable", and the six
threads of rounds 7 to 9 reopen the repair side of the method. The lenses and the finding standard
stand, since the lenses found what the passes left, and the repairs made it. Per round 10, AC4 is
judged again in this branch, at step 6: pass 4 against pass 3, whose drafts step 5 sorts.

At step 6 both counts fell, 48 to 36 confirmed and about 35 to 9 to the owner list, so AC4 does not
fire on pass 4. By the session's reading at the sort, about 19 of pass 4's findings are in text
this branch wrote after pass 3: the repairs of step 5, the method of step 4 and the saved
workflow's delivery.

## Implementation sequence

Every step that changes the installed text, steps 1 to 8, runs
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
4. **The repair by cause.** The axis's section rewritten per "The repair by cause" and "The
   workflow script", the script shipped as a saved workflow; the workflow auditor's description
   names its drafts stage. The script checked with `node --check`, wrapped as "The workflow script"
   says. Judges AC3. Fails alone on: a script that does not parse.
5. **The repair of passes 2 and 3**, per round 10. The session sorts pass 3's drafts, which this
   session holds in its scratch directory, and writes the confirmed-findings file: pass 3's
   confirmed findings, and pass 2's additions per `thread@agentic-workflow-axis@revisit-pass-2`. The
   clusters and repairs stages run through the script; the owner list of causes waits for the
   owner's answers; the repairs are applied. Its commit closes the issue entry that held the next
   run's inputs, whose work it does. Judges AC3.
   Fails alone on: a stage whose agents cannot do their brief from the section's prompts.
6. **The fourth pass**, per round 10: the axis runs again on this repository through the script, all
   three stages, as real use before the merge. Its commit message carries the counts AC4 reads.
   Judges AC3 and AC4. Fails alone on: the counts of AC4.
7. **The one-run pass.** The saved workflow rewritten to "The one-run pass": the lens table in the
   script, the steps from the preflight to the report, their prompts, and `args.from`. The audit
   skill's generic method rewritten to "The generic shape of an axis", and the agentic-workflow
   axis's section to the one-run pass, its manual fallback one sentence, per the mapping table
   "Step 7: the audit skill's sections to the one-run pass". The verdict-alignment lens added, per
   "The verdict-alignment lens". The workflow auditor's description and its brief: "within the
   owner's budget" gone, its dispatch named as the run's drafts and verify steps, its verify brief
   added, per D24. The script's `meta` description and phases name the run's steps; no prompt line
   opens with `%%`, which the build strips. The CHANGELOG.md `Next release` entries that name "the
   stages" or the budget rewritten. The `todo` issue for the design-record axis's workflow opened in
   the agent-skills issue register, naming what its design must decide: its groups of 60 and their
   order, its calibration sample, its drafts per entry, its verify step and its report. The script
   checked with `node --check`, and the build's name check passes. Judges AC2 and AC3. Fails alone
   on: a script that does not parse, or a section that names a step the script lacks.
8. **The fifth pass**, per round 13: the axis runs on this repository in one run of the saved
   workflow, the session verifies the report, the owner list waits for the owner's answers, and the
   repairs are applied. Before the run, the session checks that the harness lists the saved
   workflow with step 7's description, after step 7's install; where it does not, the owner reloads
   the harness's skills, in Claude Code with `/reload-skills`, or the pass starts in a new session.
   Its commit message carries the counts AC4 reads, and the observations T5, T6 and AC5 read; T1 to
   T4 are counted from the drafts, the auditors' saved returns and the verifiers' verdicts, a draft
   the verifier refutes or the session rejects counting as failing the reading for T2, and a draft
   marked kept or known not counting. The pass is a re-check: its kept list is pass 4's, the seven
   items of "Pass 5's kept list", which the session writes to a file and names in `args`, and its
   history is the branch since its merge base with the main branch. The audit stops on the owner's
   judgement, by the generic method's stop rule. Judges AC3, AC4 and AC5. Fails alone on: AC5's
   observable.
9. **The harvest.** The rows of "Harvest"; the spec deleted.

A session that starts step 7 cold owes the design audit of
`skill@knowledge-architect-planning@working-a-slice`, point 2, over steps 7 to 9 and the decided
design they build; steps 1 to 6 are done.

The branch is reviewed once, before the merge, on the axes of the generic method's review, which
include the decision-record, routing and standing-state axes the harvest owes, and on spec
conformity, which `skill@knowledge-architect-review@review-axes` owes to work a spec was written
before.

### Pass 5's kept list

The texts and readings the owner ruled to keep, and the findings the audit already gave an outcome,
as pass 4's auditors received them, with the issue the review of the saved workflow opened. A
finding on one of them is still drafted, and marked as kept or as known.

1. The root CLAUDE.md sets three installed instructions aside, each ruled by the owner, and
   `issue@agent-skills@patching-an-installed-skill` records how a project supersedes installed
   text: the project skill prefix `klarch-`; a retrospective's files, the workflow file and the
   project file, handled in this repository; the agentic-workflow axis's findings on the installed
   text edited at their source here, with no upstream file, and the L7 auditors also reading the
   checker's design home. A finding that these contradict the installed text is kept.
2. "The branch tip's checker" and "the checker built from the working tree" name one checker.
3. No text names the harness's transcript directory: agents find it unaided.
4. One workflow auditor carries every lens; a draft names the lens of its finding.
5. The owner ruled that the rarer checker rules are left to their finding messages, not stated in
   the installed text: kind names refused as anchor or plan names, reserved words, a directory
   home's README linking every subdocument, a plan document's level-one title and frontmatter, the
   `*` and `elsewhere` anchors, a `:line` or `#fragment` suffix, file names that do not port, a
   file both tracked and ignored, untracked files being walked, the `refuse-branch-shas` option, the
   harness's reserved skill names.
6. The CI workflow is named by no skill: judgement decides.
7. Issue entries that record findings of this audit, which are not violations left:
   `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text`,
   `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`,
   `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`,
   `issue@agent-config@klarch-release-step-12-compares-with-a-head-step-10-deleted`,
   `issue@agent-skills@patching-an-installed-skill`,
   `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search`,
   `issue@core@a-project-workflow-can-declare-an-installed-workflow-s-name`.

## Order rationale

- 1 before 2: the new section is written against the generic method.
- 2 before 3: the run dispatches the agent the section describes.
- 3 before 4: the repair by cause answers AC4's firing on the run.
- 4 before 5: passes 2 and 3 are repaired with the method built at step 4.
- 5 before 6: the fourth pass reads the corpus the repairs left, so its counts judge them.
- 6 before 7: the one-run pass answers the owner's reading of how pass 4 used the workflow.
- 7 before 8: the fifth pass is the one-run pass's real use.
- 8 before 9: the harvest records the decisions after the method is built and used.

## Defaults awaiting the owner

None awaits the owner. D16 to D21 were ruled in round 10, on the third review message: "I agree with
your defaults, as long as they align with that plan.", the plan being steps 4 to 6; D17 is replaced
by that plan. D1 to D7 were ruled in round 4, on the review message: "All defaults
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
  unchanged: the routing table then held only a project skill or agent that adds to an installed
  one, so the two sentences go beside the retrospective's in `instructions@repository-skills`, and
  the table stays empty; listed to the owner. Step 5 removes the table, per
  `design@agent-skills@no-routing-table`.
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
- **D16**, ruled, on `thread@agentic-workflow-axis@audit-synthesis-phase`: which agents cluster
  and propose repairs, and how many. The agent's round-8 reply left it open: "Either the workflow
  auditor gains two more briefs, or the script carries their prompts." In round 7 it had argued:
  "the prompt goes in the script, not in a definition the harness registered at session start".
  Default: the script block carries the cluster and repair prompts, and the prose path dispatches
  general-purpose agents with them; the lens agents stay the workflow auditor. Two cluster agents
  and one repair agent per cause are defaults the session may vary within the owner's budget, since
  `goal@agent-skills@installed-text-leaves-room-to-judge` refuses a fixed count another instruction
  needs to vary. Outside the upstream project, a cause whose findings are on both sides is repaired
  in two parts.
- **D17**, replaced in round 10. Its default ran the clusters and repairs stages on pass 3, applying
  nothing; the design-conformance review found it built to observe agents follow new text, which
  `goal@knowledge-architect@the-workflow-improves-through-real-use` refuses. The owner's plan: "After
  updating the spec and implementing the workflow, we'll fix the findings from run 2 and 3 by
  following its principles. Then, we'll run a fourth pass using the dynamic workflow, before
  shipping it, on the project (a real usage)." Steps 5 and 6.
- **D18**, ruled, on AC4: what becomes of it once the spec leaves. Default: AC4 is judged at step 6;
  at the harvest it is proposed to the owner as a tripwire on the head of
  `thread@agentic-workflow-axis@root-cause-repair`, firing when the confirmed findings or the severe
  ones do not fall from one pass to the next of a run. The agent's two round-7 proposals the owner
  did not rule, recording the mechanism "as a reason not to run many passes" and "rewording AC4 to
  judge severity by your reading", are dropped: the repair by cause answers the first, and the
  owner-list count stays the measure of severity.
- **D19**, ruled, on `thread@agentic-workflow-axis@root-cause-repair`: the kinds of cause the
  agent's round-7 reply listed, "A cause is one of these:", restored as a closed list; a finding
  that fits none stands alone.
- **D20**, ruled, on `thread@agentic-workflow-axis@audit-net-growth`: it answers another question
  than the repair by cause and loses to another rival, a bound on words, so it takes a head of its
  own, per `design@agent-skills@one-decision-per-head`. The synthesis phase is how the repair by
  cause is delivered, and stays in its head. Each finding takes the outcome of its cause's repair,
  so "Every finding takes exactly one outcome" holds unchanged.
- **D21**, ruled, on `thread@agentic-workflow-axis@audit-net-growth`: the definition of an
  instruction restored to the round-8 wording, under Names.
- Listed, no ruling needed: a draft names a finding of another lens met on the way, with the
  finding's lens, an obligation step 2 added to the auditor so that no finding met is lost and T4
  can count drift.
- **D22**, dropped with AC6, which the owner declined in round 16.
- **D23**, ruled, round 18, on `thread@agentic-workflow-axis@one-run-pass`, from the
  design-conformance review: AC5 observed whether the session acts between steps, an agent's
  conduct `design@agent-skills@synthetic-evidence-not-built` refuses to observe for accepting work.
  Default: AC5 observes only the run's mechanics, its steps completing in order with no restart,
  and pass 5 is real use, as D9 ruled the first run, its repairs landing in this repository.
- **D24**, ruled, round 18, on `thread@agentic-workflow-axis@verifier-in-the-run`, from the
  design-conformance review: a verifier briefed by the script's prompt would carry the axis's
  standard outside the axis's installed agent, the shape `design@agent-skills@audit-agent-per-axis`
  rejected for "one generic auditor agent". Default: the verifier is the workflow auditor itself,
  dispatched with a verify brief, its definition gaining a section on verifying another auditor's
  drafts; the rival is a verifier agent of its own.
- **D25**, ruled, round 18, on `thread@agentic-workflow-axis@lens-table-in-the-script`, from the
  design-conformance review: D16 kept the cluster count and the repair agents per cause as defaults
  the session may vary, on `goal@agent-skills@installed-text-leaves-room-to-judge`, and the one-run
  pass's inputs gave no way to vary them. Default: `counts` also takes `clusters`, default 2, and
  `repairsPerCause`, default 1; a verifier per auditor stays fixed, since it is the auditor's
  pair, not a count.
- **D26**, ruled, round 18, on criterion c6: the one-run pass adds about 21 agents, where round 12
  stated about 18 and the owner approved #one-run-pass with that figure. Default: the owner's word
  on #one-run-pass, read with its stated cost, accepts c6 as unmet-and-accepted, since c6 is
  weighed.
- **D27**, ruled, round 18, on `thread@agentic-workflow-axis@lens-table-in-the-script`, from the
  cold-implementer review: the counts' edge cases. Default: a lens missing from `counts` takes its
  default; a count below 1 is refused; an L2 count below its four groups walks the first groups in
  the table's order, and the report names the groups left unwalked.
- **D28**, awaiting the owner, on `thread@agentic-workflow-axis@audit-axis-runs-a-workflow`, from
  the transcript re-review: round 13 put to the owner that the design-record axis's section "stays
  as it is until then", and the review repairs had moved the generic method's points into it, a
  change the owner was not shown. Default: the section stays as it is, and the generic method keeps
  its points in place, under a sentence that an axis with no saved workflow runs them, the one-run
  shape stated beside them; the move is dropped.
- **D29**, awaiting the owner, on `thread@agentic-workflow-axis@preflight-step`, from the
  design-conformance and cold-implementer re-reviews: the preflight read the audit's earlier passes
  as the commits whose subject opens with `[audit]`, a convention of this repository that
  `goal@agent-skills@installed-text-works-anywhere` keeps out of shipped text. Default: the session
  names the history in `args`, a git range; for this repository's pass 5, the branch since its
  merge base with the main branch.
- **D30**, awaiting the owner, on `thread@agentic-workflow-axis@verdict-alignment-lens`, from the
  design-conformance re-review: a draft of divergent verdicts resting on "a minimal example it
  writes" is a synthetic scenario, which `design@agent-skills@additions-need-real-use` lets
  originate no edit. Default: the class needs a text of the project; with it, the class is provable
  by reading, a member the head's rewrite per D4 admits, and "What is already decided" lists it.
- **D31**, awaiting the owner, on `thread@agentic-workflow-axis@report-contents`, from the
  design-conformance re-review: the cross-check and report agents apply the axis's outcome table
  through script prompts, close to the shape `design@agent-skills@audit-agent-per-axis` rejected.
  Default: the cross-check proposes each cause's outcome and the session decides it, as the session
  decided outcomes before the one-run pass; the head's argument concerns the standard an auditor
  judges drafts by, and no agent decides an outcome. The rival: the workflow auditor carries the
  outcome table in a cross-check section.

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
| `tripwire@agent-skills@audit-defaults-overruled` | its premise rewritten to the session's reading that produces the owner list's defaults, the sort of an axis with no workflow or the verification of a report, and the calibration where an axis has one; "in one run" becomes "in one audit" |
| AC1 to AC4 | reported in the landing commit; AC4 proposed to the owner as a tripwire, per D18 |
| "An audit method with no calibration and no re-check" | names both heads it lost to |
| every other item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| D12 | `design@knowledge-architect@retrospective-findings-stay-here` widened to an audit's findings on the installed text; its restatement in `instructions@repository-skills`, written at step 2 |
| D15 | `design@agent-skills@ruled-items-labelled`, its `W` row widened |
| `thread@agentic-workflow-axis@root-cause-repair` | a head, holding `thread@agentic-workflow-axis@audit-synthesis-phase`: two texts state it, the axis's prose and its saved workflow |
| `thread@agentic-workflow-axis@audit-net-growth` | a head, per D20 |
| `thread@agentic-workflow-axis@audit-harness` | `design@agent-skills@audit-method` loses "so it runs under any harness"; a head, by entry test 3, since it turns on the Workflow tool's behaviour. The saved workflow's place in the install layout and the namespace is recorded in `design@agent-skills@content-mirrors-the-install-layout` and `design@core@owned-namespace-check` by the change that built it |
| `thread@agentic-workflow-axis@moment-homes` | a head, if step 5 lands a home |
| `thread@agentic-workflow-axis@revisit-pass-2` | none: step 5's commits |
| `thread@agentic-workflow-axis@one-run-pass`, `thread@agentic-workflow-axis@session-verifies-the-report`, `thread@agentic-workflow-axis@apply-stays-with-the-session`, `thread@agentic-workflow-axis@audit-axis-runs-a-workflow`, `thread@agentic-workflow-axis@verifier-in-the-run`, `thread@agentic-workflow-axis@report-contents`, `thread@agentic-workflow-axis@manual-fallback-home`, `thread@agentic-workflow-axis@preflight-step`, `thread@agentic-workflow-axis@lens-table-in-the-script`, `thread@agentic-workflow-axis@verdict-alignment-lens` | each judged by the entry tests and by `design@agent-skills@one-decision-per-head`: a thread that loses to a nearest rival of its own takes a head of its own when it passes an entry test, and a comment or a line beside its instruction when it passes none; the harvest's commit names the outcome of each. `design@agent-skills@audit-method` is rewritten to the one-run pass, the owner's quoted stop rule kept |
| T5, T6 | `path@agent-skills@docs/tripwires.md`, each naming the head that holds `thread@agentic-workflow-axis@verifier-in-the-run`, with its label |
| AC5 | reported in the landing commit |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | rewritten to the axes it still holds; its open lesson, the owner's reading of a sample of the verdicts, is adopted by this axis as the session's sample of refuted drafts, and stays for the axes that have not adopted it; its question of an audit report is answered for this axis, a working file whose outcomes the commit messages record |

## Later consequences

- The other axes of `issue@agent-skills@audit-axes-beyond-the-design-record` each give their own
  dispatch under the generic method, as a saved workflow of their own, per
  `thread@agentic-workflow-axis@audit-axis-runs-a-workflow`.
- The design-record axis's workflow, held by the `todo` issue step 7 opens; its section then
  shrinks to the generic shape.
- A pin move across the version that ships the axis may cite it in a Migration entry, if the version
  changes rules the corpus must meet; `tripwire@agent-skills@axis-not-run-at-a-pin-move` then watches
  it as it watches the design-record axis.
