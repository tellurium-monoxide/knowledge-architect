# The design-record axis runs each pass as one run of a saved workflow of its own

## Status and audience

This spec is the plan of the work that gives the design-record axis of the project audit a saved
workflow, as the agentic-workflow axis has one, and brings the axis's section of the audit skill
to the generic shape of `design@agent-skills@audit-axis-runs-a-workflow`. The work does five
things:

- a saved workflow, `path@agent-skills@content/workflows/design-record-audit.js`, runs one
  pass of the axis from its preflight to a report the session verifies: the entries enumerated and
  cut into groups by the script, one auditor per group, the calibration sample compared inside the
  run, one verifier per group, a check of every head a proposed split, merge, rename or move
  creates, a cross-check and a report;
- `agent@knowledge-architect-design-record-auditor` gains a verify brief, a check brief and a
  calibration brief, judges each head's Component, takes a brief that lists its entries, and loses
  the dispatch paragraph of its description;
- the axis's section of `skill@knowledge-architect-project-audit` keeps what the session needs to
  verify a report, and the method loses the points that only an axis with no saved workflow ran;
- small Components share a group;
- one real pass of the axis runs through the workflow over this repository.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree on a point that "What is already decided" does not
  list as rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits: a default listed under "Defaults awaiting the owner" is built as written until the owner
  rules.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/1dfb2744-3cfc-49df-89a8-3aeda2565a48.jsonl`,
  found by the owner's message that opens it, "The last branch created the agentic workflow audit
  axis, and a lot of work was put into defining the audit methodology." A second file of the same
  directory starts with the same message and holds nothing after it but an interrupt; it holds no
  part of the discussion. The discussion holds three owner messages, called rounds 1 to 3 below,
  each followed by the agent's reply: the reply to round 1 opened the threads, the reply to round 2
  showed the checkpoint and the premortem, and round 3 closed both. A fourth owner message, after
  the spec's first commit, ruled D1 to D3: "D1, D2 and D3 are accepted." The plan reviews that
  followed raised D4, awaiting the owner.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **the axis**: the design-record axis, `skill@knowledge-architect-project-audit@design-record-axis`.
- **the other axis**: the agentic-workflow axis,
  `skill@knowledge-architect-project-audit@agentic-workflow-axis`.
- **the auditor**: `agent@knowledge-architect-design-record-auditor`, source
  `path@agent-skills@content/agents/design-record-auditor.md`.
- **the existing script**: the other axis's saved workflow,
  `path@agent-skills@content/workflows/agentic-workflow-audit.js`, installed as
  `path@agent-config@workflows/knowledge-architect-agentic-workflow-audit.js`.
- **the audit skill**: `skill@knowledge-architect-project-audit`, source
  `path@agent-skills@content/skills/project-audit/SKILL.md`.
- **the method**: `skill@knowledge-architect-project-audit@audit-method`, its nine numbered points
  and the three paragraphs above them: "Each point leaves judgement …", "**An axis with a saved
  workflow** …" and "**An axis with no saved workflow** …".
- **an entry**: a design head, a level-three heading of a design home that ends with its slug, or
  a rejected alternative, each a paragraph that opens with its name in bold or a heading, as the
  auditor's section `agent@knowledge-architect-design-record-auditor@take-your-group` states the
  order.
- **the sample**: the shared calibration sample of the axis, about five entries every group judges
  beyond its own.
- **a created head**: a head that a proposed split, merge, rename or move would write.
- **the first audit**: the first audit of the axis on this repository, three passes, whose commits
  are "[audit] The first run of the design-record axis …", "[audit] The second pass of the
  design-record axis …", "[audit] The owner's rulings on the second pass …" and "[audit] The third
  pass of the design-record axis …", with "[audit] The owner's argument for delivery substitutions
  …" between the first two. `git log --grep='design-record axis'` finds them among other commits.
- **the reading worktree**: a worktree of the run detached at the commit audited, which the
  preflight creates first, as the existing script's preflight creates its `reading` worktree, and
  in which the preflight, the calibration and the cross-check read.

## What the work is

**What exists today.**

- The axis has no saved workflow. Its section of the audit skill gives its dispatch in prose: the
  groups, cut by ceil(count / 60) per Component; the sample, compared by the session before any
  draft is sorted; one auditor per group; the session's sort of every draft that proposes a change
  and of at least one conforming draft per group; the application rules; the re-check over the same
  groups. `issue@agent-skills@the-design-record-axis-has-no-saved-workflow` records the gap and
  lists what the design must decide.
- The order of a Component's entries is stated in the auditor's description, which the harness
  lists, and in its section `agent@knowledge-architect-design-record-auditor@take-your-group`; the
  cut of the groups in the description and in the axis's **Groups** bullet. Each auditor works out
  its own group from the first and last entry its brief names.
- The method states two shapes: "An axis with a saved workflow" runs one run per pass; "An axis
  with no saved workflow" runs points 1 to 9, of which points 3, 6, 7, 8 and 9 hold for both. Only
  the axis runs points 1, 2, 4 and 5.
- The axis's outcome table puts "a head in the wrong Component" on the owner list, and no rule the
  auditor reads lets it produce that finding:
  `issue@agent-skills@the-design-record-axis-never-judges-a-head-s-component`.
- The existing script is the model: a preflight agent sets up worktrees and listings, the auditors'
  drafts pipe to verifiers, general-purpose agents dedup, cluster, merge and propose repairs, the
  workflow auditor checks each proposal under a check brief, a cross-check proposes outcomes, and a
  report agent writes the report.
- Scale at the commit this spec was first written on, the heads by
  ``grep -cE '^### .*`##[a-z0-9-]+`\s*$' <design home>`` and the rejected alternatives by
  `grep -cE '^\*\*' <file>` plus `grep -cE '^#{2,4} ' <file>` over each rejected-alternatives
  file, the second count approximate since a bold line need not open an entry: knowledge-architect
  25 heads and 5 alternatives, core 104 and 70, agent-skills 121 and 42, gates 12 and 4 (all
  headings), xtask 4 and 0; 387 entries, 9 groups under today's rule, 7 with
  #small-components-share-a-group (knowledge-architect, gates and xtask packed into one group of
  50, in the manifest's order).
- The existing script already has a second setup agent, created once its jobs are known, for the
  worktrees of its check step.

**Outside the work.**

- Design self-consistency, contradictions between heads across the record: an axis of its own,
  named in `issue@agent-skills@audit-axes-beyond-the-design-record`.
- Whether a saved workflow run needs a permission rule:
  `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule`, unchanged; a second
  workflow meets the same question.
- Whether a subagent may write the file its brief names:
  `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names`. The installed agent types
  wrote their files in the observation that issue records, and every agent of the new run that
  judges against the axis's standard is the installed auditor.
- A checker command that lists rejected alternatives: lost to #groups-cut-by-the-script, see
  Losing alternatives.
- The other axis's script: unchanged.

**Standing entries.** Two standing-entry searchers covered the 64 issues and 46 tripwires at the
grounding. Those the work bears on are named in "What is already decided", in the steps or in the
harvest. The others they returned, each read whole and judged:

| entry | judgement |
| --- | --- |
| `issue@core@a-project-workflow-can-declare-an-installed-workflow-s-name` | a second installed name meets the same open question; no new decision |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | c8 holds it; the checker does not read the script |
| `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` | the report file's home and life are unchanged |
| `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` | where the closing audit of a move is delivered is unchanged |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | T1 to T5 re-enter at sessions of any project's audit; the harvest reads this issue against them |
| `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` | the auditor runs `show` per head, as today |
| `tripwire@agent-skills@auditor-skips-part-of-the-corpus` | guards the other axis's corpus, read whole by one agent; this axis cuts groups |
| `tripwire@agent-skills@record-check-misses-a-head` | the record check is for repairs of agent-facing text; this axis proposes none |
| `tripwire@agent-skills@pointer-not-followed` | step 2 replaces prose by a pointer to the script; the review of the work reads it, as the tripwire's re-entry says |
| `tripwire@agent-skills@audit-passes-do-not-converge` | step 3's pass gives the count on old text its commit message owes, from its second pass on |
| `tripwire@agent-skills@owner-intent-stripped`, `tripwire@agent-skills@rule-title-wider-than-its-argument`, `tripwire@agent-skills@split-rule-from-exception` | they watch the kinds of edit the auditor proposes; the check step reads the same failure modes; their re-entry points are unchanged |
| the searchers' doubtful returns: `tripwire@agent-skills@duplicate-auditors-add-nothing`, `tripwire@agent-skills@audit-drafts-mostly-noise`, `tripwire@agent-skills@a-head-verdict-is-overruled`, `tripwire@agent-skills@head-created-without-deliberation`, `tripwire@agent-skills@member-beyond-the-argument`, `tripwire@agent-skills@plain-text-pointer-found`, `tripwire@core@candidate-rule-silence`, `tripwire@agent-skills@structure-rejection-overruled`, `tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change` | each watches another axis, a review, a discussion or the checker, and no step of this work meets its firing condition |

## What is already decided

The design rests on these decisions, which it does not argue again:

- `design@agent-skills@apply-stays-with-the-session`: the run ends at its report.
- `design@agent-skills@audit-agent-per-axis`: the axis's standard reaches every agent that judges
  against it through the auditor's definition.
- `design@agent-skills@convergence-count-scope`: the count on old text decides convergence.
- `design@agent-skills@audit-outcomes`: every finding takes one outcome of its axis's table.
- `design@agent-skills@shipped-text-cites-no-entry`: the script cites no entry.
- `design@agent-skills@root-cause-repair`: the repair by cause, scoped to agent-facing text.
- Two rejected alternatives the design keeps rejected: "Three runs of the audit's saved workflow per
  pass, the session reading the drafts, the groupings and the proposals between them" and "The
  session reading every draft again after the run"; the session reads samples only.

The work rewrites six heads. Each text `cargo klarch show` lists as referencing them, and the step
that judges or updates it:

| head rewritten | what changes | referencing text | judged or updated by |
| --- | --- | --- | --- |
| `design@agent-skills@design-record-axis` | the dispatch is the saved workflow's; the session confirms a reading the calibration proposes, or reruns from verify; small Components share a group; the auditor judges each head's Component; a verifier, a check of created heads and a cross-check run inside the run | design.md, in `design@agent-skills@audit-method` | the harvest: the sentence "The groups and the sample are the design-record axis's dispatch" still holds |
| | | `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it` | the harvest: its sentence on what the axis reads still holds |
| | | `issue@agent-skills@audit-axes-beyond-the-design-record` | the harvest: "it stays open for every axis that has not adopted it" is updated, since the axis adopts the session's sample |
| | | `issue@agent-skills@the-design-record-axis-never-judges-a-head-s-component` | closed at the harvest, which does its "What would close it": the head's change, then the three edits step 1 makes |
| | | rejected alternative "An audit method with no calibration and no re-check" | the harvest: still holds |
| | | `tripwire@agent-skills@audit-edit-leaves-an-entry-stale` | the harvest: still holds; the cross-check is the step that watches it |
| `design@agent-skills@audit-method` | an axis with no saved workflow and its manual points leave | design.md, in `design@agent-skills@audit-is-an-activity` ("each axis gives only its corpus, its dispatch and its outcomes") | the harvest: reworded if the dispatch now sits in the script |
| | | design.md, in `design@agent-skills@convergence-count-scope` and `design@agent-skills@audit-repairs-the-whole-record` | the harvest: unaffected |
| | | `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names` | the harvest: its sentence read again |
| | | `issue@agent-skills@an-upstream-finding-stays-in-every-later-old-text-count`: "point 8" at three sites | step 2: renumbered to point 4, per the mapping table |
| | | `issue@agent-skills@audit-axes-beyond-the-design-record` | the harvest, with the row above |
| | | rejected alternative "An audit method with no calibration and no re-check" | the harvest |
| | | `tripwire@agent-skills@audit-defaults-overruled`: its premise names "the sort of an axis with no saved workflow" | the harvest: the premise names the verification of a run's report alone |
| `design@agent-skills@committed-audit-report` | "for an axis with none, the drafts it read at the sort" leaves | design.md, in `design@agent-skills@ruled-items-labelled` and `design@agent-skills@audit-method` | the harvest: unaffected |
| | | `issue@agent-skills@audit-axes-beyond-the-design-record` | the harvest |
| | | rejected alternative "The commit messages alone as the record of an audit's passes" | the harvest: unaffected |
| `design@agent-skills@audit-axis-runs-a-workflow` | "The design-record axis has none yet" leaves; both axes have one | design.md, in `design@agent-skills@one-run-pass` and `design@agent-skills@design-record-axis` | the harvest |
| | | `issue@agent-skills@the-design-record-axis-has-no-saved-workflow` | closed by step 3's pass |
| | | `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` | the harvest: a second workflow is named |
| | | rejected alternatives "One markdown file per audit axis …" and "The audit's workflow script kept in a fenced block …" | the harvest: unaffected |
| `design@agent-skills@one-run-pass` | its account of the steps names the other axis's alone; the worktrees the preflight sets up become, for this axis, those of a setup agent after the cut; what the session's verification reads becomes what each axis's section names | design.md, in `design@agent-skills@audit-method`, `design@agent-skills@verifier-in-the-run` and `design@agent-skills@repair-proposals-audited` | the harvest: each sentence read again |
| | | rejected alternatives "Three runs of the audit's saved workflow per pass …" and "The session reading every draft again after the run" | the harvest: unaffected, still rejected |
| `design@agent-skills@verifier-in-the-run` | the design-record auditor is a verifier too, with a sample of conforming drafts; this axis's verifiers start after the calibration's barrier, not as each auditor returns | design.md, in `design@agent-skills@one-run-pass` and `design@agent-skills@workflow-auditor-agent` | the harvest |
| | | `tripwire@agent-skills@verifier-refutes-a-true-finding`, `tripwire@agent-skills@verifiers-wave-findings-through` | the harvest: each still holds, now over both axes' verifiers |

Texts that state the axis's dispatch or the method's points and cite none of these heads, so that
`show` does not list them, judged in steps 1 and 2:

- `skill@knowledge-architect-setup`, its sentence running the axis at the end of a move of
  documents: unaffected.
- The workflow auditor's L8 pair "the decision-record reviewer and the design-record auditor",
  whose standards the owning-Component rule touches: the routing reviewer, not the decision-record
  reviewer, judges a head's Component in a review; step 1 reads the pair again.
- `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, its fourth case: "the
  design-record axis cuts its groups by an order that sits in its agent's description", made false
  by step 2 and reworded there.
- Citations of the method's points by number, which no check reaches, renumbered in step 2 per the
  mapping table: the audit skill itself, five sites; `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it`,
  "points 7 and 8"; `issue@agent-skills@an-upstream-finding-stays-in-every-later-old-text-count`,
  "point 8" at three sites; `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`,
  "point 9" at two sites. Its other numbered citations name the planning skill's points.
- `issue@core@references-are-read-in-markdown-and-rust-only`: the new script's prompts name
  sections of the auditor and of the audit skill in words, which no check reaches. Step 2 checks
  each section name the script uses against the installed texts.

## Criteria

### One run per pass, the session reading nothing between steps, and a report ready to act on `##c1`

c1. Binding, as a presumption, from `design@agent-skills@one-run-pass`. Met by the whole run and by
#design-record-cross-check.

### Nothing an agent proposes is applied before a reading confirms it `##c2`

c2. Binding, as a presumption, from `design@agent-skills@audit-method`. Met by
#design-record-verifier and the session's verification of the report.

### Each auditor's group is bounded, so the agent reads all of it `##c3`

c3. Binding, as a presumption, from `design@agent-skills@design-record-axis`. Met by
#groups-cut-by-the-script and #small-components-share-a-group, whose packed groups keep the bound.

### The sample's readings are compared before any draft is sorted, and one the rule's text admits both ways goes to the owner `##c4`

c4. Binding, as a presumption, from `design@agent-skills@design-record-axis`. Met by
#calibration-in-the-run.

### No agent of the run decides an outcome; the session decides and applies `##c5`

c5. Binding, as a presumption, from `design@agent-skills@one-run-pass` and
`design@agent-skills@apply-stays-with-the-session`. Met by #design-record-cross-check, which
proposes, and #calibration-in-the-run, whose reading the session confirms or reruns.

### The steps stay runnable by hand from the script, for another harness `##c6`

c6. Binding, as a presumption, from `design@agent-skills@audit-axis-runs-a-workflow`, and the
owner's framing in round 1: "let harnesses other than claude code rely on manual execution of the
workflow, if ever needed." Met by #dispatch-moves-to-the-script.

### Each created head is judged alone against every rule before the commit `##c7`

c7. Binding, as a presumption, from `design@agent-skills@design-record-axis`. Met by
#created-heads-checked-in-the-run.

### The script cites no entry `##c8`

c8. Binding, as a presumption, from `design@agent-skills@shipped-text-cites-no-entry`. Met by the
script built as the existing one is; checked at the review of the work.

### The session's context spent per pass `##c9`

c9. Weighed. Source: the measurement in `design@agent-skills@one-run-pass`. Met by
#design-record-verifier and #no-cause-clustering.

### The agents dispatched per pass `##c10`

c10. Weighed. Source: none recorded; a cost. Unmet: from 9 auditors to 19 to 26 agents per pass
with #small-components-share-a-group, 23 to 32 without, over this repository's 7 or 9 groups: 1
preflight, 1 setup, one auditor and one verifier per group, 1 calibration, 0 to one checker per
group, 1 cross-check and 1 report. The reply to round 2 and D1 stated 20 to 27 and 22 to 31, a
miscount the code-claims review found. Unmet-and-accepted, on the owner's
ruling of D1: "D1, D2 and D3 are accepted."

### The audit skill gets shorter `##c11`

c11. Weighed. Source: the owner's words in `design@agent-skills@audit-axis-runs-a-workflow`:
"That would also make the project audit skill much more concise." Met by
#dispatch-moves-to-the-script and #manual-points-retire.

## Threads

Every thread was proposed by the agent in the reply to round 1, and marked `presumed-settled` in
the reply to round 2 on the owner's word "I agree with your overall position here, I have nothing
to add over that." Each was approved in round 3, when the owner answered the checkpoint table with
"Keep all proposed tripwires and acceptance criterion. Proceed".

The reply to round 1 names a slug #design-record-report once, in its table of standing entries,
beside the lesson that the session reads a sample of the verdicts. No thread of that name was
opened, and no delta carries it: the report and the session's samples are part of
#design-record-cross-check, whose decided design holds them.

### The preflight enumerates the entries, the script cuts the groups, and each brief lists its entries `##groups-cut-by-the-script`

Approved, round 3. Arguments: `argument@design-record-workflow@a2`,
`argument@design-record-workflow@a3`, `argument@design-record-workflow@a15`. Shape: Decided design, "The entries and the groups". Harvest:
`design@agent-skills@design-record-axis`.

### The sample is compared inside the run, after a barrier, an agent proposing each reading and the verifiers applying it `##calibration-in-the-run`

Approved, round 3. The owner, round 2, on a reading the session rejects: "2. With a rerun, as you
proposed." Arguments: `argument@design-record-workflow@a4`, `argument@design-record-workflow@a5`,
`argument@design-record-workflow@a6`. Shape: Decided design, "The calibration". Harvest:
`design@agent-skills@design-record-axis`. D2 bears on it.

### One verifier per group refutes the drafts that propose a change, samples the conforming ones and classifies old and new text `##design-record-verifier`

Approved, round 3. Arguments: `argument@design-record-workflow@a1`,
`argument@design-record-workflow@a7`. Shape: Decided design, "The verify step". Harvest:
`design@agent-skills@verifier-in-the-run` or `design@agent-skills@design-record-axis`, by the entry
tests.

### Each created head is judged alone inside the run, by one checker per group with such a proposal `##created-heads-checked-in-the-run`

Approved, round 3. Arguments: `argument@design-record-workflow@a8`. Shape: Decided design, "The
check step". Harvest: `design@agent-skills@design-record-axis`.

### A cross-check maps overlaps, finds stale entries and proposes each finding's outcome `##design-record-cross-check`

Approved, round 3. Arguments: `argument@design-record-workflow@a9`. Shape: Decided design, "The
cross-check and the report". Harvest: `design@agent-skills@design-record-axis`.

### The axis's run has no cluster, merge or repair step `##no-cause-clustering`

Approved, round 3. Arguments: `argument@design-record-workflow@a10`,
`argument@design-record-workflow@a17`. Shape: Decided design, "What
the run does not do". Harvest: `design@agent-skills@design-record-axis`, by the entry tests (a
decision about an absence).

### Components smaller than the group size are packed whole into shared groups `##small-components-share-a-group`

Approved, round 3. Arguments: `argument@design-record-workflow@a11`,
`argument@design-record-workflow@a16`. Shape: Decided design, "The
entries and the groups". Harvest: `design@agent-skills@design-record-axis`.

### The dispatch moves into the script, and the axis's section keeps what the session needs to verify a report `##dispatch-moves-to-the-script`

Approved, round 3. The owner, round 2, on the sample: "1. Agreed with your default.", the default
being the preflight agent's choice, overridden by `args.sample`. Arguments:
`argument@design-record-workflow@a12`, `argument@design-record-workflow@a18`. Shape: Decided design, "The audit skill and the auditor",
and the mapping table. Harvest: `design@agent-skills@audit-axis-runs-a-workflow`.

### The method loses the points only an axis with no saved workflow ran `##manual-points-retire`

Approved, round 3. Arguments: `argument@design-record-workflow@a13`. Shape: Decided design, "The
audit skill and the auditor", and the mapping table. Harvest: `design@agent-skills@audit-method`
and `design@agent-skills@committed-audit-report`, rewritten.

### The auditor judges each head's Component `##owning-component-judged`

Approved, round 3. Arguments: `argument@design-record-workflow@a14`. Shape: Decided design, "The
audit skill and the auditor". Harvest: `design@agent-skills@design-record-axis`; closes
`issue@agent-skills@the-design-record-axis-never-judges-a-head-s-component`.

## Arguments

### The manual sort of the first audit cost the session most of a pass, and its third pass fell back to report lines `##a1`

Reply to round 1, agent; bears on #design-record-verifier, #no-cause-clustering. "At pass 1, the
session read every draft that proposed a change, 118 entries." "At pass 3, the message says: 'The
session sorted from the auditors' report lines'. The manual sort was already too expensive to do in
full."

### A script cannot read a file, so it needs the counts to dispatch `##a2`

Reply to round 1, agent; bears on #groups-cut-by-the-script. "the script cannot read a file, so it
needs the counts to know how many agents to dispatch."

### An order rule in prose lets two agents leave an entry unjudged or judged twice `##a3`

Reply to round 1, agent; bears on #groups-cut-by-the-script. "Today every auditor works out its own
group from an order rule in prose. Two agents that apply the rule differently leave an entry
unjudged or judged twice, and nothing would notice. An explicit list makes coverage checkable by the
script."

### Without a barrier, a settled reading reaches drafts in every group that verifiers already passed `##a4`

Reply to round 1, agent; bears on #calibration-in-the-run. "a settled reading would then reach
drafts in all 9 groups that verifiers already passed. One agent would re-read them across every
group, which is the load the bound of 60 exists to avoid. The barrier costs wall-clock time only,
the time of the slowest auditor."

### A verifier sees only its own group, so it cannot see a difference between groups `##a5`

Reply to round 1, agent; bears on #calibration-in-the-run. "at pass 1, two of the five sample
entries were read two ways. A verifier sees only its own group, so it cannot see a difference
*between* groups."

### A rejected reading costs one more verify step, which keeps the session's reading small `##a6`

Reply to round 1, agent, and round 2, owner; bears on #calibration-in-the-run. The agent: "The
rerun keeps the session's reading small. The session's own reading is faster when one or two drafts
are affected." The owner chose the rerun.

### The verifier makes the refutations the session's sort would have made `##a7`

Reply to round 1, agent; bears on #design-record-verifier. "118 drafts at pass 1 is the reading
`one-run-pass` measured as costing the session most of its context. The verifier made the
refutations the session's sort would have made, as the other axis showed."

### A check inside the run moves the judgement of created heads before the owner's ruling `##a8`

Reply to round 1, agent; bears on #created-heads-checked-in-the-run. "Today that check happens
after the owner's ruling, during the application. Checking in the run moves it before the ruling,
so you rule on heads that were already judged alone." On the count: "Pass 1 had about 25 splits, so
that would be 25 agents, each reading the rules from the start. One checker per group reads the
rules once."

### The report agent should quote, not judge `##a9`

Reply to round 1, agent; bears on #design-record-cross-check. The owner's ruling on the other
axis, "The cross check and the report should be one agent each (two total).", carries over: "the
report agent should quote, not judge."

### A head's edit adds no instruction, so edits interact only on one paragraph or one slug `##a10`

Reply to round 1, agent; bears on #no-cause-clustering. "the repair by cause (`root-cause-repair`)
answers instructions that interact. … A head is a record judged against fixed rules. Its edit adds
no instruction, and two edits interact only when they touch one paragraph or one slug, which the
cross-check's overlap map finds."

### Packing whole Components keeps every alternative with its heads, as the recorded reason asks `##a11`

Reply to round 1, agent; bears on #small-components-share-a-group. "The recorded reason for 'one
Component per group': 'a rejected alternative is judged against the head it lost to.' Packing whole
Components keeps every alternative with its heads. Today's cut already separates core's
alternatives (its last group) from most of core's heads."

### The agent's description and the skill keep only what the session needs to verify `##a12`

Reply to round 1, agent; bears on #dispatch-moves-to-the-script. The split of the reply's table;
the description "says what the workflow auditor's says: the saved workflow dispatches it, and a
session without the workflow dispatches it as the workflow's steps do (c6)."

### Once both axes have a workflow, the manual points serve no axis `##a13`

Reply to round 1, agent; bears on #manual-points-retire. "a future axis is designed with its
workflow, so the points would be text that no session follows. The manual fallback for another
harness is the script's steps (c6), not these points."

### The outcome table already names the finding, and the texts edited are the same `##a14`

Reply to round 1, agent; bears on #owning-component-judged. "the outcome table already names 'a
head in the wrong Component'. The texts this work edits are the same ones the issue's repair edits.
So the cost is one read and one judgement per head, inside agents already dispatched." The rival,
deleting that row, "would leave misfiled heads read only by the routing reviewer, and only when a
diff touches them."

### The preflight is a cost the run pays anyway, and the cut adds one setup agent `##a15`

Reply to round 1, agent; bears on #groups-cut-by-the-script. "**Cost:** one preflight agent, which
the run needs anyway for the worktrees." The decided design adds a setup agent after the cut, since
the worktrees per group are known only then, as the existing script's check worktrees are; the plan
reviews found that the argument's premise no longer covered it. The cost is one agent more than the
argument stated.

### Mixing Components in a group costs nothing any record shows `##a16`

Reply to round 1, agent; bears on #small-components-share-a-group. "It wins only if a group that
mixes Components is harder to judge, and nothing recorded says so. This is a small gain, so ruling
it out costs little."

### A cause step would cost about four agents to group what the owner list already groups by kind `##a17`

Reply to round 1, agent; bears on #no-cause-clustering. "That would cost about 4 agents, and would
group findings such as 'nine titles moved to their rule' that are already grouped by kind in the
owner list."

### A re-check may keep the sample entries whose readings differed `##a18`

Reply to round 1, agent, and round 2, owner; bears on #dispatch-moves-to-the-script. The agent:
"on a re-check, the session may want to keep the entries whose readings differed last pass." The
owner: "1. Agreed with your default."

## New names, in one place

```text
crates/agent-skills/content/workflows/design-record-audit.js     the saved workflow (new)
  meta.name   knowledge-architect-design-record-audit
  args        commit, scratch, worktrees, history, previous, kept, sample, counts, from
  sample      an array of entries, each { ref } for a head or { name, file } for an alternative
  counts      groupSize (default 60), checkersPerGroup (default 1)
  from        'calibration' | 'verify'
  files       <scratch>/preflight/entries.json (written by the preflight), issues.txt,
                tripwires.txt, history.txt, reports.txt
              <scratch>/setup/groups.json (written by the setup agent: each group, its label and
                its entries, and the sample)
              <scratch>/<group label>/  one draft per entry, <nn>-<slug>.md, nn its position in
                the group, slug the head's slug or the alternative's name in kebab case;
                return.md; kept.md on a re-check
              <scratch>/calibration/calibration.md
              <scratch>/verify/<group label>/verdicts.md
              <scratch>/check/<group label>/check-<k>.md, k from 1 to checkersPerGroup
              <scratch>/cross-check/cross-check.md
              <scratch>/report/report.md
  worktrees   <worktrees>/reading, <worktrees>/<group label>, <worktrees>/check-<group label>-<k>
  group labels  <component>-<n>, or packed-<n> for a group of whole small Components
  schemas     ENTRIES (preflight): { components: [{ name, entries: [entry] }], sample: [entry] }
              JUDGED (auditor, drafts): { judged: [{ entry, verdict: 'conforms' | 'fails' }] }
              CALIBRATED: { path, proposed, bothReadings }
              VERDICTS (verifier): the existing script's fields, plus createdHeads, the count of
                verified findings proposing a split, a merge, a rename or a move
              PATH (check, cross-check, report), GROUPS (restart): { groups: [...], sample: [...] }
.claude/workflows/knowledge-architect-design-record-audit.js      its installed copy (new)
crates/agent-skills/content/agents/design-record-auditor.md       existing; gains the sections
  verify-drafts, check-created-heads, calibrate-the-sample (new section slugs)
```

## Decided design

Every step that judges against the axis's standard runs the auditor, `agentType` the auditor's
installed name, under the brief its step names: the drafts, the calibration, the verify and the
check steps. The preflight, the setup, the restart reader, the cross-check and the report run
general-purpose agents, whose prompts are in the script, as `design@agent-skills@workflow-auditor-agent`
states for the other axis. Each auditor brief carries the existing script's line telling the agent
to read its definition, the root CLAUDE.md and the primer from its worktree where its copy differs.

### The entries and the groups

From #groups-cut-by-the-script and #small-components-share-a-group.

A preflight agent creates the reading worktree, then enumerates the entries of every Component in
the manifest's order, each Component's entries in the order the auditor's section
`agent@knowledge-architect-design-record-auditor@take-your-group` states, which the prompt names
and the agent reads in the reading worktree: that section is the order's one home. It runs the
checker's `model` for the heads and reads each rejected-alternatives file. It writes the list to
`entries.json` and returns it through the ENTRIES schema, an illustration of its shape:

```js
{ components: [ { name: 'core', entries: [ { ref: 'design@core@plans-dir-fixed', file, line },
                                           { name: 'A fixture marker', file, line } ] } ],
  sample: [ … ] }
```

It also writes the issues and tripwires listings, the history and the earlier report files, as the
existing script's preflight does, and, when `args.sample` is absent, chooses the sample: about five
entries, from different Components where there are several. `args.sample`, in the entries' shape,
overrides it.

The script cuts the groups. A Component of more than `groupSize` entries is cut alone into
ceil(count / groupSize) groups of consecutive entries, the sizes of any two differing by at most
one. The Components of at most `groupSize` entries are packed whole, in the manifest's order, each
into the first packed group whose size stays within `groupSize`. A setup agent then writes
`groups.json`, creates one worktree per group and `checkersPerGroup` check worktrees per group,
each detached at `args.commit`, and copies the kept list into each group's directory. It cannot run
before the cut; it creates the check worktrees before it is known which groups need them, and the
unused ones are removed with the rest.

Each auditor's brief lists its entries one by one, and the sample's. The auditor writes one draft
per entry, named as New names says, its usual return to `return.md`, and returns through the JUDGED
schema every entry it judged with its verdict. The script logs each listed entry missing from a
return; the report names it.

A sample entry is drafted by every group. The draft of record, which the verify step, the
cross-check and the report treat as the finding, is that of the group whose listed entries hold it;
the other groups' drafts of it serve the calibration alone.

Nearest rival: the checker lists the entries, rejected alternatives included. It is a core change
for one consumer, since rejected alternatives are not entities of the checker.

### The calibration

From #calibration-in-the-run.

The drafts step ends at a barrier: every auditor has returned. Then one auditor under its
calibration brief, in the reading worktree, reads every group's drafts on the sample entries and
writes `calibration.md`. For each sample entry: the verdict of each group and one of two outcomes,
a proposed reading where one verdict follows the rule's text, with the sentence of the rule it rests
on, or "the text admits both readings", which the report puts on the owner-list draft. It returns
the CALIBRATED counts. Each verifier receives the file and re-judges its group's drafts that a
proposed reading bears on.

The session confirms each reading at its verification. A reading it rejects is corrected in
`calibration.md`, and the workflow runs again with `from: 'verify'`. The owner chose the rerun in
round 2.

Nearest rival: the verifiers start as each auditor returns and the calibration runs after them,
re-reading the affected drafts itself; it lost to `argument@design-record-workflow@a4`. Second
rival: no sample, the verifiers alone; it lost to `argument@design-record-workflow@a5`. The
calibration agent is the auditor under a third brief, by D2, ruled.

### A restart

`from: 'calibration'` skips the preflight, the setup and the drafts; `from: 'verify'` skips the
calibration too. Either starts with a general-purpose agent that reads `groups.json` back and
returns it through the GROUPS schema, as the existing script reads its causes file back on a
restart. Every step after the starting one runs again. A step that failed or returned nothing
earlier is rerun from the first step whose files are complete.

### The verify step

From #design-record-verifier.

One auditor under its verify brief per group, in that group's worktree, given the calibration file,
the kept list, the issues and tripwires listings, and the history of the audit's earlier passes, as
the existing script's verify brief gives. It:

- tries to refute every draft of record that proposes a change: each quotation against the tree,
  each quotation of the owner against the question it answered, in its source;
- marks a draft `known` when an issue records its finding, and `kept` when it is on the kept list;
- marks `bears-on-ruling` and `bears-on-tripwire` as
  `agent@knowledge-architect-workflow-auditor@verify-drafts` does for the other axis;
- re-judges the drafts a proposed reading of the calibration bears on, and records each it
  re-judged;
- reads a sample of the drafts that find their entry conforming, at least one, of its choosing;
  where one is wrong, it re-reads the auditor's other drafts that judge the same rule;
- on a re-check, classifies each verified finding as on old or new text: an entry is on new text
  when `git diff <previous> <commit>` changes its words;
- writes `verdicts.md`: per draft, verified, refuted or plausible, with its reason, the marks, the
  drafts re-judged and the conforming drafts sampled.

It returns the VERDICTS schema, whose `createdHeads` tells the script which groups the check step
takes.

Nearest rival: no verifier, the session sorting after the run; it fails c9 by
`argument@design-record-workflow@a1`.

### The check step

From #created-heads-checked-in-the-run.

For each group whose verifier returned `createdHeads` above 0, `checkersPerGroup` auditors, one by
default, under the check brief, each in its check worktree, judge each created head of that group's
verified findings alone against every rule of `primer@design-heads`, and against the auditor's list
of the failure modes of an edit, in its section
`agent@knowledge-architect-design-record-auditor@read-the-rules`. Several checkers of one group
each judge every created head, their checks alternatives for the session, as `repairsPerCause`
gives alternatives in the existing script. Checker k writes `check-<k>.md`.

Nearest rival: one checker per proposal; it lost to `argument@design-record-workflow@a8`.

### The cross-check and the report

From #design-record-cross-check.

One general-purpose agent, in the reading worktree, reads every verified finding, every check and
the calibration file, and writes `cross-check.md`:

1. the overlap map: two findings that edit one paragraph; a rename whose citing sites another
   group's finding edits; a split in one group whose rejected alternative sits in another;
2. the stale entries: each tripwire and issue that cites a head some finding edits, with whether
   the edit leaves it false;
3. for each finding, the outcome it proposes under the axis's outcome table, with its reason, for
   the session to decide.

One general-purpose agent writes the report, quoting rather than summarising, in these parts:

1. the header: the commit audited; the agents of each step; each group with its first and last
   entry; the counts of entries, of drafts conforming and failing, verified, refuted, plausible,
   bearing on a ruling, bearing on a tripwire, known and kept; for a re-check, the counts
   `skill@knowledge-architect-project-audit@record-of-an-audit` asks of a pass; each entry missing
   from a return; each refuted draft with its reason;
2. the calibration: per sample entry, each group's verdict, the proposed reading and the sentence
   of the rule it rests on, and the drafts the verifiers re-judged;
3. the findings, grouped by proposed outcome, those applied in the branch by Component and file:
   per finding, the entry, the rule failed, the quoted evidence, the edit before and after, the
   verifier's verdict and note, old or new text, and the checks beside each created head;
4. the overlap map and the stale entries;
5. the draft of the owner list, grouped by kind, its labels left for the session to number;
6. what the run did not decide: the plausible verdicts, the "Outside the axis" and "Met outside
   the task" notes of the returns, every step that failed or returned nothing.

**What the session's verification reads**, stated in the axis's section: the report whole; each
proposed reading of the calibration against the sample drafts; every item of the owner-list draft,
and every finding marked plausible, bearing on a ruling or on a tripwire, against its draft; every
check; a sample of the refuted drafts and a sample of the conforming drafts, of its choosing. The
last sample is the lesson `issue@agent-skills@audit-axes-beyond-the-design-record` keeps open for
an axis that has not adopted it. The method's paragraph on an axis with a saved workflow then says
that the session verifies the report as its axis's section says, and the other axis's section
takes the list that paragraph holds today: the report whole, every repair proposal in full, every
finding marked bearing on a ruling, on a tripwire or plausible against its draft, and a sample of
the refuted drafts.

Nearest rival: the report agent doing the cross-check; it lost to
`argument@design-record-workflow@a9`.

### What the run does not do

From #no-cause-clustering. The run has no dedup, cluster, merge or repair step. Each entry is
listed in one group, and a sample entry's draft of record is its own group's. Each draft carries
its edit. Nearest rival: the existing script's cluster and repair steps; it lost to
`argument@design-record-workflow@a10` and `argument@design-record-workflow@a17`.

### The audit skill and the auditor

From #dispatch-moves-to-the-script, #manual-points-retire and #owning-component-judged.

- **The axis's section** keeps the corpus, the rules, what is not in the axis, the agent, the
  saved workflow's name and its `args` table, one paragraph listing the steps, what the session's
  verification reads, the outcome table, the rule for old and new text, and the application rule
  that the session judges alone each head it creates beyond what the run checked. Its dispatch
  bullets leave, per the mapping table.
- **The method** loses "An axis with no saved workflow runs the points below" and points 1, 2, 4
  and 5; points 3, 6, 7, 8 and 9 become points 1 to 5, stated for every axis. The sentence "A
  session whose harness does not list the workflow dispatches its steps by hand, in the script's
  order, with the script's prompts, read from the script" stays.
- **The Terms**, **the frontmatter description** and **the record of an audit** lose what they say
  of an axis with none, per the mapping table.
- **The auditor's description** loses its dispatch paragraph and says that the axis's saved
  workflow dispatches it, for its drafts, its calibration, its verify and its check steps, and that
  a session without the workflow dispatches it as the workflow's steps do.
- **The auditor's body**: its opening paragraph names the verifier and the report rather than the
  session's sort; "Your brief" names the listed entries; "Take your group" judges the listed
  entries and keeps the order as what the preflight reads; "Read the rules first" adds
  `skill@knowledge-architect-decision-recording@owning-component`; "Judge each entry" judges each
  head's Component by that section's questions; three sections are added, for the calibration,
  verify and check briefs, each with its return; "Return" keeps its text shape, written to
  `return.md`, and gains the JUDGED schema.

## Mapping tables

Each sentence or bullet of today's texts that this work moves, removes or makes false, and its new
home:

| today's text | new home |
| --- | --- |
| the auditor's description: the order of a Component's entries | removed; its section `take-your-group` keeps the order, which the preflight prompt names |
| the auditor's description: ceil(count / 60) groups, sizes differing by at most one, the 125-entry example | the script's group cut, with `groupSize` |
| the auditor's description: "give each the commit audited, its group's first and last entry, the shared calibration sample, a scratch directory and a worktree of its own" | the script's draft brief, listing the entries |
| the auditor's description: "Then read its drafts, never its summary alone" | removed: the verifier reads the drafts, the session reads the report and samples |
| the auditor's opening paragraph: "The session that sent you reads your drafts, sorts each finding …" | the verifier and the report, then the session's verification |
| the axis's section: "The order of a Component's entries, and the inputs read beside each head, are in … description, which the harness lists: cut the groups by that order" | removed; the section names the workflow; the inputs beside each head stay in the auditor's body |
| the axis's section, the dispatch, **Groups** | the script, with #small-components-share-a-group |
| the axis's section, the dispatch, **Calibration** | the script's calibration step; the section's step paragraph names it |
| the axis's section, the dispatch, **The brief** | the script's draft brief |
| the axis's section, the dispatch, **The sort** | the verify brief (the conforming sample, a wrong draft's siblings) and the section's paragraph on what the session's verification reads |
| the axis's section, the dispatch, **Application** | stays in the section: a split drafted first is the auditor's drafts; each created head judged alone is the check step, and the session for anything it adds |
| the axis's section, the dispatch, **The re-check** | the script's `previous` and the verifier's classification; the section keeps the old-and-new rule |
| the method, "An axis with a saved workflow …": "it reads it whole, every repair proposal in full, every finding the report marks … and a sample of the refuted drafts it chooses" | the paragraph says the session verifies the report as its axis's section says; the list moves to the other axis's section |
| the method, "An axis with no saved workflow runs the points below. Points 3, 6, 7, 8 and 9 hold for both." | removed |
| the method, points 1, 2, 4, 5 | removed; each axis's script holds its corpus prompt, its dispatch, its drafts and its verify |
| the method, points 3, 6, 7, 8, 9 | points 1, 2, 3, 4, 5, in that order |
| the audit skill's citations by number: "points 6 and 7 below … point 8; point 9" (the saved-workflow paragraph), "which point 8 reads" (point 3), "as point 6 says", "the issue entry point 8 gives" and "as point 8 defines them" (the record of an audit) | renumbered: points 2 and 3, point 4, point 5 |
| `issue@agent-skills@an-upstream-finding-stays-in-every-later-old-text-count`, "point 8", three sites | point 4 |
| `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it`, "points 7 and 8" | points 3 and 4 |
| `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`, the audit method's "point 9", two sites | point 5 |
| the audit skill's frontmatter description: "or, for an axis with none, the corpus, a dispatch each axis gives, drafts in files and the sort" | removed |
| Terms, "dispatch": "Each axis's section gives its own" | each axis's saved workflow holds it |
| Terms, "the re-check": "dispatched as its first dispatch was" | stays: a re-check is the next run of the same workflow |
| Terms, "pass", "For an axis with none: the drafts, the sort, the owner list and the application." | removed |
| the record of an audit, "for an axis with none, the drafts read at the sort, one section per finding with its evidence and its proposed edit" | removed |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, its fourth case: "the design-record axis cuts its groups by an order that sits in its agent's description" | reworded: the order sits in the auditor's section, which the saved workflow's preflight reads |

## Losing alternatives

- **The checker listing the entries, rejected alternatives included**: lost to
  #groups-cut-by-the-script. Rejected alternatives are not entities of the checker, so it is a core
  change for one consumer.
- **The verifiers starting as each auditor returns, the calibration running after them**: lost to
  #calibration-in-the-run, by `argument@design-record-workflow@a4`.
- **No sample, the verifiers alone**: lost to #calibration-in-the-run, by
  `argument@design-record-workflow@a5`.
- **The session re-reading the drafts a rejected reading bears on**: lost to #calibration-in-the-run
  on the owner's choice in round 2, "With a rerun, as you proposed."
- **No verifier, the session sorting after the run**: lost to #design-record-verifier, by
  `argument@design-record-workflow@a1`.
- **One checker per proposal**: lost to #created-heads-checked-in-the-run, by
  `argument@design-record-workflow@a8`; `counts.checkersPerGroup` keeps the count adjustable.
- **The report agent doing the cross-check**: lost to #design-record-cross-check, by
  `argument@design-record-workflow@a9`.
- **The existing script's cluster, merge and repair steps reused**: lost to #no-cause-clustering, by
  `argument@design-record-workflow@a10`.
- **One Component per group, as today**: lost to #small-components-share-a-group, by
  `argument@design-record-workflow@a11`.
- **The method's manual points kept for a future axis**: lost to #manual-points-retire, by
  `argument@design-record-workflow@a13`.
- **The outcome table's row "a head in the wrong Component" deleted**: lost to
  #owning-component-judged, by `argument@design-record-workflow@a14`.

## Readings

The work reads Claude Code's Workflow tool as the existing script does, with no new reading: a
saved workflow runs by its `meta` name from a file directly under the workflows directory, takes
its inputs in `args` and reads no file, recorded in `design@agent-skills@audit-axis-runs-a-workflow`
and the message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
installer's namespace"; `agent()` takes `agentType` and `schema`, which the existing script's verify
and check steps use.

## Premortem

Each cause, the thread it stresses, and its verdict. The owner, round 3: "Keep all proposed
tripwires and acceptance criterion."

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| 1 | a wrong calibration reading spreads to every group, the session's confirmation its one guard | #calibration-in-the-run | tripwire T1 |
| 2 | the preflight misses entries of an unusual shape, which the coverage check cannot see, since it compares returns with the preflight's list; gates writes its alternatives as headings and the others as bold paragraphs | #groups-cut-by-the-script | acceptance criterion AC1 |
| 3 | the verifier's sample of conforming drafts is too light, so a missed violation surfaces a pass later | #design-record-verifier | tripwire T2 |
| 4 | the overlap map misses a collision, such as a rename whose citing site another group's edit rewrites | #design-record-cross-check | tripwire T3 |
| 5 | one rule misread across many heads, outside the sample | #no-cause-clustering | tripwire T4 |
| 6 | the owning-Component judgement is noisy and grows the owner list | #owning-component-judged | tripwire T5 |
| 7 | the run is too large to finish in one run, by a concurrency cap or a failed step | #small-components-share-a-group, #calibration-in-the-run | acceptance criterion AC2 |

The tripwires, written at the harvest as `skill@knowledge-architect-decision-recording@ruled-tripwires`
says, each guarding the head its thread lands in:

- **T1.** Fires when: in one audit, the session's verification rejects a reading the calibration
  agent proposed, in two passes. Response: reopen #calibration-in-the-run, with the session
  settling the reading among the candidates. Re-entry: the session's verification of each report.
- **T2.** Fires when: in any pass, the session's sample of conforming drafts holds one that a
  reading shows failing a rule. Response: reopen #design-record-verifier, on how much of the
  conforming drafts it reads. Re-entry: the session's verification of each report.
- **T3.** Fires when: during an application, the session meets two findings that edit one
  paragraph or one slug, and the overlap map does not list them. Response: reopen
  #design-record-cross-check. Re-entry: the application of each pass.
- **T4.** Fires when: in one pass, the session traces findings in at least three groups to one
  misreading of one rule, and the calibration did not catch it. Response: reopen
  #no-cause-clustering, and how the sample is chosen. Re-entry: the session's verification of each
  report.
- **T5.** Fires when: in one pass, the session or the owner rejects more than half of the
  owning-Component findings, on at least 4 such findings; the bounds are the owner's to reset.
  Response: reopen #owning-component-judged. Re-entry: the session's verification of each report,
  and the owner's ruling on the owner list.

## Acceptance criteria

Both read the closing pass, step 3, an audit of this repository's record whose repairs land on the
branch; D4 puts to the owner whether they stand against
`goal@knowledge-architect@the-workflow-improves-through-real-use`.

### On the closing pass, the preflight's entry count per Component equals a count the session takes independently `##entry-count-matches-the-tree`

AC1. Guards #groups-cut-by-the-script. Judged at step 3. Observable: at the commit audited, the
session counts each Component's heads in the output of the checker's `model`, and its rejected
alternatives by reading each rejected-alternatives file whole, a bold line or a heading counted only
where it opens an alternative; a Component whose count differs from the preflight's fires it.
Response: reopen #groups-cut-by-the-script.

### The closing pass runs from preflight to report as one run, every listed entry in its own group's return `##one-run-covers-every-entry`

AC2. Guards #small-components-share-a-group and #calibration-in-the-run. Judged at step 3.
Observable: the run needs a restart through `from` after a step failed or returned nothing, or the
report names an entry missing from the return of the group that lists it. A sample entry is in
every return by design, and a rerun from verify after the session rejects a calibration reading is
the design's own path: neither fires it. Response: reopen the thread the failure points to: the
group size for a concurrency cap, the barrier for a failed calibration.

## Implementation sequence

1. **The auditor's body.** As "The audit skill and the auditor" says, its description aside: the
   opening paragraph, the listed entries, the owning-Component rule, the calibration, verify and
   check briefs, the return schema. The workflow auditor's L8 pair read again against the owning-
   Component rule. Installed in the same commit. Fails alone on: `cargo klarch check`, whose second
   phase compares the installed copies with the shipped text.
2. **The saved workflow, the description and the skill.** The script, as Decided design says; the
   auditor's description; the axis's section, the method, the frontmatter description, the Terms,
   the record of an audit and the other axis's section, per the mapping table; the renumbered
   citations in the three issues the mapping table names, and the fourth case of
   `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`; the CHANGELOG entries under
   `Next release`, and `cargo x changelog`. Installed in the same commit. The description moves
   here, with the skill's section, so that no commit holds a section pointing to a description that
   no longer holds the order. Checks:
   - the script parses, as `path@agent-skills@CLAUDE.md` says;
   - each section the script names in words exists in the installed texts, by a grep of each name;
   - a stub run in node: a harness the session writes in its scratch directory, not committed,
     stubbing `agent`, `parallel`, `pipeline`, `phase`, `log` and `args` as the existing script
     uses them, as the commit "The agentic-workflow axis runs a pass as one run of its saved
     workflow, with a lens comparing review and audit standards" reports doing. Over this
     repository's counts, with the stub verifiers returning `createdHeads` 0, a default run
     dispatches 19 agents (1 preflight, 1 setup, 7 auditors, 1 calibration, 7 verifiers, 1
     cross-check, 1 report) and 7 groups, one packed; with `createdHeads` 1 for every group, 26; a
     return missing one listed entry is logged; `from: 'verify'` dispatches no preflight, setup,
     auditor or calibration.

   Fails alone on: the stub run's counts.
3. **The closing pass.** In a session whose harness lists the new workflow and the new agent
   definition, which may need a new session after the install: one pass of the axis over this
   repository through the workflow, its report file, its owner list, the owner's rulings and the
   application, as the audit skill says; further passes on the owner's word, each giving the counts
   on old text that `tripwire@agent-skills@audit-passes-do-not-converge` reads. The session reads
   `issue@agent-skills@primer-content-bundles-each-directive-s-decision` and
   `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, whose re-entry point is this
   pass, against the pass's findings on the heads they name, and gives each an outcome under
   `skill@knowledge-architect-issue-tracking`. Judges AC1 and AC2. Fails alone on: the run's
   report.
4. **The harvest**, per the harvest row, and the deletion of this spec.

## Order rationale

- Step 1 before step 2: the script's briefs name the auditor's new sections, which must exist.
- Step 2 before step 3: the pass runs the script, and its session needs the installed copy listed.
- Step 3 before step 4: the harvest records the decisions as the pass showed them built, and the
  pass closes the issue that names it.

## Defaults awaiting the owner

- **D1**, c10, ruled accepted: the rise in agents per pass is accepted as a cost. The figure the
  owner ruled on was 20 to 27; the corrected figure is 19 to 26, per c10.
- **D2**, #calibration-in-the-run, ruled accepted: the calibration agent is the auditor under a
  calibration brief, not a general-purpose agent as the reply to round 1 implied. It judges which
  verdict follows a rule's text, which is judging against the axis's standard, and
  `design@agent-skills@audit-agent-per-axis` puts that standard in the auditor's definition.
- **D3**, ruled accepted: the work is a spec, one branch and one pull request, the spec committed
  on the work's branch.
- **D4**, AC1 and AC2, from the design-conformance review: both read how agents followed a step on
  the closing pass, and `goal@knowledge-architect@the-workflow-improves-through-real-use` says "No
  session is built to observe how agents follow its instructions: not to originate an edit, to
  choose between shapes, or to accept a piece of work." Default: both stand. The closing pass is not
  built to observe: it is an audit of this repository's record, whose findings are applied, and the
  first real use of the workflow; AC1 and AC2 read what that real use shows, which
  `design@agent-skills@synthetic-evidence-not-built` admits as "a misbehaviour observed in real
  use". The rival: drop both, and leave the pass's report to the session's verification alone.

The owner ruled D1 to D3 in one message after the spec was first committed: "D1, D2 and D3 are
accepted."

## Harvest

At step 4, under `skill@knowledge-architect-decision-recording`, then
`skill@knowledge-architect-issue-tracking`. The entry tests decide which decision earns a head and
which alternative an entry.

- `design@agent-skills@design-record-axis` rewritten: the dispatch through the saved workflow, the
  calibration inside the run, the verifier, the check of created heads, the cross-check, the
  packed groups, the owning-Component rule, the absence of a cause step. A decision that answers
  another question than the head's takes a head of its own, by `primer@design-heads`.
- `design@agent-skills@verifier-in-the-run` and `design@agent-skills@one-run-pass` rewritten, per
  "What is already decided".
- `design@agent-skills@audit-axis-runs-a-workflow`, `design@agent-skills@audit-method` and
  `design@agent-skills@committed-audit-report` rewritten, per "What is already decided".
- Each text of the table of "What is already decided" judged or updated.
- `issue@agent-skills@the-design-record-axis-never-judges-a-head-s-component` closed.
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` read
  against T1 to T5.
- The losing alternatives into `path@agent-skills@docs/rejected-alternatives.md` where a recording
  test admits them.
- T1 to T5 into `path@agent-skills@docs/tripwires.md`, each keeping its label, guarding the head
  its thread lands in.
- AC1 and AC2 reported on; each that recurs is proposed to the owner as a tripwire.
- `issue@agent-skills@the-design-record-axis-has-no-saved-workflow` closed by step 3's pass.

## Later consequences

- A new audit axis is designed with its saved workflow; the method holds no points for an axis
  with none.
- The design self-consistency axis of `issue@agent-skills@audit-axes-beyond-the-design-record`
  may reuse this script's preflight enumeration of entries.
