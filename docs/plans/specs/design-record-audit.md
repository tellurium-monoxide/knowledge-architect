# A project audit reads the whole record on one axis, and its first axis re-applies the rules on design heads and rejected alternatives

## Status and audience

This spec is the plan of the work that gives the workflow a way to audit one aspect of a whole
project, as opposed to reviewing a diff, and its first axis: the conformance of the decision record
to the rules on design heads and rejected alternatives. The work does five things:

- entry test 2 says what a site is for installed text;
- an installed skill holds the method of a project audit and one section per axis;
- an installed agent holds the standard of the first axis, the design-record auditor;
- a Migration entry of the changelog that changes the rules on recorded content names the axis to
  run, and the setup skill's step for moving the pin reads it;
- the axis runs once on this repository, which judges the acceptance criterion.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec and its work land on one branch**, on the owner's word, round 4 below. No gate requires
  a separate merge: `cargo klarch commits` judges each commit against the installed set it holds,
  per `design@core@installed-entities-from-the-tree`.
- **The work starts in the session where the discussion converged**, so it takes no design audit
  unless commits other than this spec's own land on main before the work starts, per
  `skill@knowledge-architect-planning@working-a-slice`.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/dbcef9fd-3bf3-45c0-9358-058b2e26aa60.jsonl`,
  which holds two discussions. This one begins at the owner's message that opens "Good. Now that
  this is done, there is a closely related subject that needs to be discussed." and ends at the
  owner's message that opens "Agreed on that table as is." It holds 5 owner messages and 4 agent
  replies. Below, "round 0" is the owner's opening message, "round n" the agent's reply that opens
  "Round n" (round 1 opens "This is round 1"), and "the reply to round n" the owner's message after
  it. Find the file by that opening message, not by its name. An extraction agent wrote the owner
  messages and the rounds verbatim to scratch and stopped before the rest; the session assembled
  the remainder from those files and its own context.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **a project audit**: the reading of one aspect of a whole project, every entry of a corpus, by an
  installed method, as opposed to a review, which reads a diff. The owner's name, from
  `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`.
- **an axis**: one aspect a project audit reads, with its corpus, its rules, and the outcome of each
  kind of finding.
- **the design-record axis**: the first axis, which re-applies `primer@design-heads` to every design
  head and the rules of `skill@knowledge-architect-decision-recording@losing-alternatives` to every
  rejected alternative, reading the tripwires and issues that bear on each head.
- **the audit skill**: the new installed skill, named per D1.
- **the design-record auditor**: the new installed agent that drafts the axis's verdicts for one
  group of entries.
- **a draft**: one auditor's file per entry: verdict, evidence, proposed edit.
- **the owner list**: one message to the owner holding every finding that needs the owner's word,
  grouped by kind, each kind under a label `Q<n>` with a default.
- **the re-check**: a fresh auditor dispatched over the result after the edits, whose clean report is
  the acceptance of a run.
- **a site, for installed text**: a text of the installed set that states the reason, per
  `thread@design-record-audit@test-2-site-for-installed-text`.

## What the work is

### The record as it stands

- **The workflow has no whole-project reading.** Every review of `skill@knowledge-architect-review`
  reads a diff. `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` records the gap and two
  worked instances; this repository's history holds a third, the sweep of the commit "The heads that
  cited an approval as their ground stand on their argument, and six bundles are split", with its
  drafts, owner list, applying agents and re-check (AC1 of the plan head-rules, deleted at its
  landing).
- **Entry test 2** in `primer@design-heads` reads: "**the same reason must be respected at more than
  one site, or at none.** [...] At none: a decision about an absence ("we do not do X"), or a policy
  with no code of its own". `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined`
  records that a decision about installed text can be read as such a policy, and that an audit
  counted two skills stating one decision as one site.
- **Entry test 3** admits a decision whose argument turns on an outside tool's behaviour, read in its
  documentation or measured. `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` asks
  whether it admits too much.
- **Moving the pin**, `skill@knowledge-architect-setup@moving-the-pin`, reads the changelog of every
  version crossed, installs, and runs the gates. It names no audit.
- **Adopting existing documentation**, `skill@knowledge-architect-setup@existing-documentation`,
  ends with a `todo` issue for the move, planned as a milestone.
- **The changelog's Migration entries**, per `design@knowledge-architect@changelog-entries`, say
  what a consumer must change in its own files. The `Next release` section of CHANGELOG.md holds the
  entries of the head-rules work, whose rules every adopting project's design homes must now meet.
- **The installed agents** are the reviewers and the standing-entry searcher, under
  `path@agent-skills@content/agents/`. The searcher's description carries its dispatch rule, groups
  of at most 60 entries, per `design@agent-skills@standing-entry-search-groups`.

### What is outside the work

- **The other axes the issue names**: design self-consistency, alignment of the code with the design
  and goals, the project's own agent workflow (which takes the restatements, per the reply to round
  1), standing state as a whole, goal coverage. They stay in
  `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`, rewritten at the harvest.
- **An audit report document**, for axes whose fixes are code: the owner left it "a question for
  later" in round 0; it stays in that issue.
- **Whether a head is true of the code**: left out of this axis, per the reply to round 1; a head the
  axis finds false of the tree gets an issue entry.
- **Test 3**: unchanged, per `thread@design-record-audit@test-3-reading-not-practice`.

## What is already decided

The design rests on these, and does not argue them again:

- `goal@knowledge-architect@documentation-stays-consistent`, `goal@knowledge-architect@agents-work-without-drift`,
  `goal@knowledge-architect@the-owner-decides`, `goal@agent-skills@one-skill-per-activity`,
  `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@installed-text-leaves-room-to-judge`.
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@synthetic-evidence-not-built`
  (an applied verdict is confirmed by reading), `design@agent-skills@standing-entry-search-groups`
  (bounded groups), `design@agent-skills@a-scratch-directory-per-subagent`,
  `design@agent-skills@existing-heads-on-touch` (a gradual repair, which the audit complements),
  `design@agent-skills@head-ground-is-the-argument`, `design@agent-skills@shipped-text-line-comments`
  (the `%%` line, the local home of a reason in installed text).
- `design@agent-skills@additions-need-real-use`: the evidence is the three worked instances and the
  owner's named lack in round 0, a migration of every adopting project's design homes at the next pin.

The work rewrites these decisions and texts. Every text that `cargo klarch show` lists as referencing
each is judged at the harvest:

| decision or text | what changes | judged or updated at |
| --- | --- | --- |
| entry test 2 of `primer@design-heads` | a site for installed text is a text that states the reason; "no site" is an absence or a policy no text states | step 1 |
| `skill@knowledge-architect-decision-recording@when-recording-happens`, "as a decision with no site of its own" | reworded to the new test 2 | step 1 |
| `design@agent-skills@a-head-is-owed-by-an-entry-test`, "a policy or an absence has no site at all" | reworded to the new test 2 | the harvest |
| `design@agent-skills@instruction-record-is-minimal` | read again: its exception for a decision that keeps two parts consistent stays | the harvest |
| `design@knowledge-architect@changelog-entries` | gains the convention: a Migration entry that changes the rules on recorded content names the axis to run | the harvest |
| `skill@knowledge-architect-setup@moving-the-pin` and `skill@knowledge-architect-setup@existing-documentation` | read the axis a Migration entry names, and run it; the move of existing documents runs the axis | step 4 |
| the primer's `primer@installed-skills` and the review skill's scope | name the audit skill; a review stays a review of a diff | step 2 |
| `goal@agent-skills@one-skill-per-activity` and `goal@knowledge-architect@agents-get-a-complete-workflow`, which list the workflow's activities | auditing is an activity they do not list, per D2 | the harvest, on the owner's word |
| `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` | rewritten to the axes and the report question left open | the harvest |
| `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` | closed | the harvest |
| `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` | closed on the owner's ruling | the harvest |

## Criteria

### The audit reaches what no diff review reads `##c1`

C1. Binding, derived from `goal@knowledge-architect@documentation-stays-consistent`. Met by
`thread@design-record-audit@audit-is-an-activity` and `thread@design-record-audit@design-record-axis`.

### Nothing in it is specific to one harness `##c2`

C2. Binding, derived from `goal@agent-skills@installed-text-works-anywhere`: subagents only, no
workflow tool. Met by `thread@design-record-audit@audit-method`.

### It gives capabilities, not a script `##c3`

C3. Binding as a presumption, derived from `design@agent-skills@capabilities-not-structure`. Met:
the method is a shape with judgement at each point, adjusted per axis.

### No edit widens what the owner approved, or removes a ground that is the owner's, without the owner's word `##c4`

C4. Binding, derived from `goal@knowledge-architect@the-owner-decides`. Met by
`thread@design-record-audit@audit-outcomes`.

### One activity, one skill; a review stays a review of a diff `##c5`

C5. Binding, derived from `goal@agent-skills@one-skill-per-activity`. Met by
`thread@design-record-audit@audit-is-an-activity`.

### Every verdict that is applied is confirmed by reading `##c6`

C6. Binding, derived from `design@agent-skills@synthetic-evidence-not-built`. Met by
`thread@design-record-audit@audit-method`, its sort and its re-check.

### It scales to a large project `##c7`

C7. Binding: bounded groups, results in files. Met by `thread@design-record-audit@audit-method`.

### Its cost in subagents and in the owner's reading time stays proportionate `##c8`

C8. Weighed. Unmet and accepted: about one subagent per group plus the re-check, about ten for this
repository's 189 heads. The owner, the reply to round 2: "Agreed on all, tripwires and AC approved."

## Threads

### A project audit is an activity with an installed skill of its own, and a review stays a review of a diff `##audit-is-an-activity`

Proposed by the agent, round 1. Approved. The skill holds the method and one section per axis.
Arguments: `argument@design-record-audit@a5`, `argument@design-record-audit@a6`. Shape: "Decided
design". Harvest: a head of `path@agent-skills@docs/design.md`. The owner's words, the reply to
round 1: "audit-is-an-activity : agreed on that shape."

### The audit's method: scope, groups, calibration, drafts, a verified sort, one owner list, disjoint application, a fresh re-check, a reviewed branch `##audit-method`

Proposed by the agent, round 1, from the three worked instances. Approved, generic for now and
adjusted when another axis comes. Arguments: `argument@design-record-audit@a3`,
`argument@design-record-audit@a4`, `argument@design-record-audit@a7`, `argument@design-record-audit@a8`.
Shape: "Decided design". Harvest: a head. The owner's words, the reply to round 1: "audit-method:
that part is quite hard to do properly IMO, because it might differ per axis. For now, we are only
providing one axis, so we can adjust this generic shape later for the others. But I think your
general shape should hold its ground for the other axis too. We can go with that for now."

### The design-record axis audits the design heads and the rejected alternatives, reading the tripwires and issues that bear on each head `##design-record-axis`

Proposed by the agent, round 1, with three more members; narrowed by the owner in the reply to round
1; the agent's reading of that word stated in round 2, presumed-settled, and approved in the reply to
round 2. The tripwires and issues are inputs: the auditor reads every tripwire guarding a head and
every issue naming it before judging it, and the audit repairs one of them only where an edit makes
it stale. Restatements of heads go to the agentic-workflow axis; whether a head is true of the code
is left out. Arguments: `argument@design-record-audit@a9`, `argument@design-record-audit@a10`,
`argument@design-record-audit@a11`. Shape: "Decided design". Harvest: a head. The owner's words, the
reply to round 1: "design-record-axis: agreed on the primer and the rejected alternatives. About the
others: they are not really "design". Reading the tripwires and issues related to design is
important IMO, so that the audit on design takes them into account. The restatement item, I find it
weird to add it in this axis. It does not read the same thing at all. It would go in the "agentic
workflow" axis mentionned in the issue, instead, IMO. Agreed to leave the "true of code" part out of
this audit axis."; the reply to round 2: "Agreed on all".

### Each finding is applied in the audit's branch, put to the owner, or opened as an issue `##audit-outcomes`

Proposed by the agent, round 1. Approved. The commit messages are the record; no report document for
this axis. Arguments: `argument@design-record-audit@a1`, `argument@design-record-audit@a12`. Shape:
"Decided design". Harvest: a head. The owner's words, the reply to round 1: "audit-outcomes: agreed
on those conditions."

### The axis runs at the owner's request, at a pin move whose Migration entry names it, and at the move of an adopting project's existing documents `##audit-triggers`

Proposed by the agent, round 1. Approved. Arguments: `argument@design-record-audit@a13`,
`argument@design-record-audit@a14`. Shape: "Decided design". Harvest: a head, and
`design@knowledge-architect@changelog-entries` gains the convention. The owner's words, the reply to
round 1: "audit-triggers: agreed on the main trigger being my request. Good idea on the changelog
naming the axis to run. And agreed for the existing docs of an adopting project."

### Each axis has an installed agent of its own, holding the axis's standard `##audit-agent-per-axis`

Proposed by the agent, round 2. Approved. Arguments: `argument@design-record-audit@a15`,
`argument@design-record-audit@a16`. Shape: "Decided design". Harvest: a head. The owner's words,
the reply to round 2: "Agreed on all".

### The open questions on the entry tests are settled before the audit ships `##entry-tests-settled-first`

Proposed by the owner, the reply to round 2. Approved. Arguments: `argument@design-record-audit@a17`,
`argument@design-record-audit@a18`. Shape: the implementation sequence, step 1 first. Harvest: none
of its own; the threads below carry the decisions. The owner's words, the reply to round 2: "I don't
think we can ship this audit workflow without fixing them."; the reply to round 3:
"entry-tests-settled-first: agreed."

### For installed text, each text that states a reason is one site, and "no site" is an absence or a policy no text states `##test-2-site-for-installed-text`

Proposed by the agent, round 3, as shape A of the issue. Approved. Arguments:
`argument@design-record-audit@a19`, `argument@design-record-audit@a20`. Shape: "Decided design".
Harvest: `design@agent-skills@a-head-is-owed-by-an-entry-test` rewritten, or a head of its own, as the
tests decide; the issue closes. The owner's words, the reply to round 3: "test-2-site-for-installed-text:
we can go with your solution, it looks better than the current ambiguity."

### Test 3 admits only a reading of an outside thing that cost the project work `##test-3-reading-not-practice`

Proposed by the agent, round 3. Ruled out by the owner in the reply to round 3, on the owner's
argument. Arguments: `argument@design-record-audit@a21`, `argument@design-record-audit@a22`. The
owner's words: "Unless you think there is a real inconsistency in the entry tests, I think we can keep
as is. I'd rather keep the tests simpler than make them harder to judge, which is more likely to
reduce quality of the design homes."

## Arguments

### Most of this axis's findings are text edits, applied at once `##a1`

Round 0, the owner. Bears on `thread@design-record-audit@audit-outcomes`. "For the axis we are
discussing, I think the goal is usually immediate application. I don't think there is much reason to
delay, since those are mostly text edits, unless some design entries have major, issues that bear on
project goals for example."

### An axis whose fixes are code may need another output, a report `##a2`

Round 0, the owner. Bears on `thread@design-record-audit@audit-outcomes`. "For the other proposed
axes (which we won't introduce now), code edits might become needed, and under that case, immediate
application of a lot of findings might be too much. But opening lots of issues would also be quite
noisy for the project. Maybe a different output shape would be needed (an audit report?). This might
stay as a question for later".

### The migration at the next pin needs the record re-read under the new rules `##a3`

Round 0, the owner. Bears on `thread@design-record-audit@audit-method`,
`thread@design-record-audit@audit-triggers`. "With the changes we just did to decision record
conditions and shapes, at the next version pin, projects migrating will need to migrate their design
home to match them. We need to provide a way for the workflow to efficiently cover the recorded
design and reapply the record tests to them."

### Today's sweep was the third run of this axis by hand, and its shape worked `##a4`

Round 1, the agent. Bears on `thread@design-record-audit@audit-method`. "Grouped read-only agents
[...] One draft file per entry [...] One list to you, the edits grouped by kind [...] Applying agents
on disjoint files [...] A fresh agent judges the result against the same rules: today, AC1 and AC2. It
found what the sweep missed."

### A review stays a review of a diff, since the two standards differ `##a5`

Round 1, the agent. Bears on `thread@design-record-audit@audit-is-an-activity`. "an audit is an
activity with its own artifact, a repaired record across the whole project, not a verdict on one
branch. Adding a whole-project mode to the review skill would mix two standards: what a diff owes,
and what the whole record owes."

### A skill per axis would restate the method in each `##a6`

Round 1, the agent. Bears on `thread@design-record-audit@audit-is-an-activity`. "a skill per axis. It
would restate the method in each skill, against the restatement rule, and against
`goal@agent-skills@one-skill-per-activity` while the method is the same."

### Agents dispatched together are not calibrated alike `##a7`

Round 1, the agent, from the second worked instance. Bears on `thread@design-record-audit@audit-method`.
"The agents were not calibrated alike." The issue's lesson: "An audit that compares figures across
agents owes a calibration: a shared sample classed by each."

### The re-check found real defects each time it ran `##a8`

Round 1, the agent. Bears on `thread@design-record-audit@audit-method`. "a lighter method, with no
calibration and no re-check [...] Today, the re-check (AC1 and AC2) found real defects each time it
ran, so I propose keeping it."

### The same pass can read every entry the decision record holds `##a9`

Round 1, the agent. Bears on `thread@design-record-audit@design-record-axis`. "these all read the same
entries against rules already installed".

### Tripwires and issues are not design, but the audit must take them into account `##a10`

The reply to round 1, the owner. Bears on `thread@design-record-audit@design-record-axis`. "they are
not really "design". Reading the tripwires and issues related to design is important IMO, so that the
audit on design takes them into account."

### Restatements read another thing, and belong to the agentic-workflow axis `##a11`

The reply to round 1, the owner. Bears on `thread@design-record-audit@design-record-axis`. "The
restatement item, I find it weird to add it in this axis. It does not read the same thing at all. It
would go in the "agentic workflow" axis mentionned in the issue, instead, IMO."

### The commit messages are the record, as a measurement is routed to the commit `##a12`

Round 1, the agent. Bears on `thread@design-record-audit@audit-outcomes`. "The durable record is the
commit messages: what was audited, at which commit, the counts, and the outcome of every finding.
Measurements belong there by the primer's routing rule."

### A Migration entry that changes recorded rules names the axis to run `##a13`

Round 1, the agent; the owner, the reply to round 1: "Good idea on the changelog naming the axis to
run." Bears on `thread@design-record-audit@audit-triggers`. "a Migration entry that changes the rules
on recorded content names the axis to run."

### No evidence gives a period `##a14`

Round 1, the agent. Bears on `thread@design-record-audit@audit-triggers`. "a periodic run. The record
gives no evidence for a period, and `design@agent-skills@additions-need-real-use` asks for evidence
from real use."

### An agent per axis carries the axis's standard from its first token `##a15`

Round 2, the agent. Bears on `thread@design-record-audit@audit-agent-per-axis`. "An agent's tools
enforce read-only drafting. The axis text reaches the subagent from its first token, without
depending on the brief. A second axis adds an agent, not a branch inside one text."

### A generic agent puts the standard behind a read the brief must ask for `##a16`

Round 2, the agent. Bears on `thread@design-record-audit@audit-agent-per-axis`. "one generic auditor
agent, briefed to read the axis section of the skill. It needs one agent for every axis, but it puts
the axis standard behind a read the brief must ask for. Today's sweep drafts came out uneven where
the brief was the only standard."

### An audit cannot ship over tests read two ways `##a17`

The reply to round 2, the owner. Bears on `thread@design-record-audit@entry-tests-settled-first`.
"the existing open issues about the consistency of the entry tests. I don't think we can ship this
audit workflow without fixing them."

### The audit's design does not depend on the wording, only its verdicts do `##a18`

Round 3, the agent. Bears on `thread@design-record-audit@entry-tests-settled-first`. "An audit that
re-applies tests read two ways gives verdicts that change with the agent, and it sends each of them to
your list. [...] The audit's design does not depend on the tests' wording, since the axis applies
whatever `primer@design-heads` says. Only its verdicts do."

### A site defined for installed text removes the recorded miscount `##a19`

Round 3, the agent. Bears on `thread@design-record-audit@test-2-site-for-installed-text`. "It removes
the miscount the issue records, where two skills stating one decision were counted as one site. It
makes test 2 mean for installed text what it means for code: a reason stated at one site lives at that
site. The installed text already has that local home, a `%%` line beside the instruction".

### The narrower test 2 moves some heads to `%%` lines `##a20`

Round 3, the agent. Bears on `thread@design-record-audit@test-2-site-for-installed-text`. "a decision
about installed text stated in one text alone fails test 2. It keeps a head only by another test, such
as test 4, or the exception of `design@agent-skills@instruction-record-is-minimal` [...] Some heads of
the agent-skills design home would move to `%%` lines. How many is not known."

### A narrower test 3 keeps the case it exists for and refuses a documented practice `##a21`

Round 3, the agent. Bears on `thread@design-record-audit@test-3-reading-not-practice`. "it keeps the
case test 3 exists for: an expensive reading that is hard to derive again. It refuses
`design@knowledge-architect@toolchain-is-pinned`".

### Documentation ranks below a design head, and simpler tests serve the design homes better `##a22`

The reply to round 3, the owner. Bears on `thread@design-record-audit@test-3-reading-not-practice`.
"The project's own documentation is a lower level source than design heads, by our hierarchy of
knowledge sources (I think from the primer). This argues toward keeping it as a design head.
Otherwise, this new test would be slightly contradicting with the usual rule of design taking
precedence over implementation (and that includes its documentation in my reading). [...] I'd rather
keep the tests simpler than make them harder to judge, which is more likely to reduce quality of the
design homes."

## New names, in one place

```text
the audit skill            planned@agent-skills@content/skills/project-audit/SKILL.md, per D1;
                           installed as knowledge-architect-project-audit
the design-record auditor  planned@agent-skills@content/agents/design-record-auditor.md;
                           installed as knowledge-architect-design-record-auditor
```

## Decided design

### Entry test 2 for installed text

Test 2 of `primer@design-heads` gains, after "At none: a decision about an absence ("we do not do X"),
or a policy with no code of its own": for installed text, each text of the installed set that states
the reason is one site, and a policy stated by one text has one site, whose `%%` line is its reason's
home. The sentence of `skill@knowledge-architect-decision-recording@when-recording-happens` names a
policy that no text states. The rival, test 2 as it reads, pointing to
`design@agent-skills@instruction-record-is-minimal`, keeps the clause that admits every decision
about installed text (`argument@design-record-audit@a19`). The cost is heads about installed text
stated by one text alone, which the first run finds and puts to the owner
(`argument@design-record-audit@a20`).

### The audit skill

One installed skill, named per D1, whose description names its symptom: the owner asks for an audit
of one aspect of the whole project, a pin move whose Migration entry names an axis, or the move of an
adopting project's documents. It states what it does not cover: a review of a diff
(`skill@knowledge-architect-review`), and the design audit of a slice or a spec
(`skill@knowledge-architect-planning`). It holds:

- **The method**, generic, with the judgement each point needs:
  1. the axis names its corpus;
  2. the corpus is cut into bounded groups, as `design@agent-skills@standing-entry-search-groups`
     cuts its own;
  3. every auditor first classes one small shared sample, and the session compares the classes
     before the run counts;
  4. one auditor per group, read-only, in a scratch directory of its own, writes one draft per
     entry: verdict, evidence quoted (the history where a verdict rests on a ruling), proposed edit;
  5. the session verifies every draft that needs the owner and a sample of the rest, and sorts each
     finding into its outcome;
  6. one owner list, by kind, `Q<n>` labels, a default each;
  7. applying agents on disjoint files, the session reading the whole diff;
  8. the re-check, a fresh auditor over the result;
  9. a branch of its own, reviewed as any change, the transcript review last.
- **The outcomes**, as `thread@design-record-audit@audit-outcomes` says: applied in the branch (a
  rewording, tense, a missing reference, an approval removed from a ground, a split that touches no
  word of the owner, a rejected alternative or a tripwire repaired in place); put to the owner on the
  list (a widening of what the owner's words approved, the removal of a head that fails every entry
  test, a ground that is the owner's words, a verdict where a test reads two ways); an issue entry (a
  head false of the code, a conflict with a goal, a split too large for the branch). The commit
  messages are the record: what was audited, at which commit, the counts, every finding's outcome.
- **One section per axis**: the design-record axis, its corpus, its inputs, its outcomes, and the
  agent that drafts it.

### The design-record auditor

One installed agent, `knowledge-architect-design-record-auditor`, read-only, dispatched once per
group. Its description carries the dispatch rule, as the standing-entry searcher's does. Its body is
the axis's standard: read `primer@design-heads` and
`skill@knowledge-architect-decision-recording@losing-alternatives` whole; for each head of its group,
read every tripwire guarding it and every issue naming it, with `cargo klarch show`, and the history
behind every citation of the owner; judge the head against every rule of the section and each
rejected alternative against its rules; write one draft per entry; return the list of drafts. It
does not edit the tree.

### The triggers

- **The owner's request**, the main one.
- **A pin move.** Step 2 of `skill@knowledge-architect-setup@moving-the-pin` gains: where a Migration
  entry of a version crossed names an audit axis, run it under the audit skill, in a branch of its
  own, after the pin's commit. `design@knowledge-architect@changelog-entries` gains the convention,
  and the `Next release` section of CHANGELOG.md gains a Migration entry naming the design-record
  axis for the rules the head-rules work changed and for the new test 2.
- **The move of an adopting project's existing documents**:
  `skill@knowledge-architect-setup@existing-documentation` names the axis as the reading the move
  runs over the design records.

## Mapping tables

The work needs none.

## Losing alternatives

- **A whole-project mode of the review skill**, lost to `thread@design-record-audit@audit-is-an-activity`
  (`argument@design-record-audit@a5`).
- **A skill per axis**, lost to `thread@design-record-audit@audit-is-an-activity`
  (`argument@design-record-audit@a6`).
- **A lighter method, with no calibration and no re-check**, lost to
  `thread@design-record-audit@audit-method` (`argument@design-record-audit@a8`).
- **Restatements and the truth of the code in this axis**, lost to
  `thread@design-record-audit@design-record-axis` (`argument@design-record-audit@a11`).
- **A periodic run**, lost to `thread@design-record-audit@audit-triggers` (`argument@design-record-audit@a14`).
- **One generic auditor agent**, lost to `thread@design-record-audit@audit-agent-per-axis`
  (`argument@design-record-audit@a16`).
- **Test 2 as it reads, pointing to the minimal record of installed text**, lost to
  `thread@design-record-audit@test-2-site-for-installed-text` (`argument@design-record-audit@a19`).
- `thread@design-record-audit@test-3-reading-not-practice`, ruled out on the owner's argument
  (`argument@design-record-audit@a22`).

## Readings

The work reads no external specification.

## Premortem

Each cause was put to the owner in round 2 under its label; the owner, the reply to round 2: "Agreed
on all, tripwires and AC approved." Round 4 put the cause of the new test 2 and proposed no new
tripwire: `tripwire@agent-skills@a-head-verdict-is-overruled` and T6 watch it.

| label | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| T6 | The calibration fails silently, and the owner list fills with defaults the owner reverses | `thread@design-record-audit@audit-method` | tripwire. Fires when, in one run, the owner rules against the default on more than a quarter of the listed items; the bound is the owner's to reset. Reopens the decision harvested from that thread |
| T7 | A Migration entry names the axis, and the pin moves without the axis run | `thread@design-record-audit@audit-triggers` | tripwire. Fires when a retrospective finds a pin moved across such a version with the axis not run. Reopens the decision harvested from that thread |
| T8 | An edit makes a tripwire or an issue it never read stale | `thread@design-record-audit@design-record-axis` | tripwire. Fires when a review of an audit branch finds a tripwire guarding, or an issue naming, a head the branch changed, left false. Reopens the decision harvested from that thread |
| AC1 | The method does not work at its first real run | `thread@design-record-audit@audit-method`, `thread@design-record-audit@design-record-axis` | acceptance criterion, `acceptance@design-record-audit@first-run-leaves-nothing-missed` |

## Acceptance criteria

### The first run of the axis on this repository ends with a re-check that finds no violation the drafts missed `##first-run-leaves-nothing-missed`

AC1. Guards `thread@design-record-audit@audit-method` and `thread@design-record-audit@design-record-axis`.
Judged at step 5. The instrument: the run's own re-check, a fresh design-record auditor over the
design homes and rejected alternatives after the run's edits, against the rules shipped by steps 1 to
4. Fires on one violation of `primer@design-heads` or of the rules on rejected alternatives that the
drafts missed. Response: repair it, and reopen the thread the miss comes from.

## Implementation sequence

Steps 1 to 4 change the installed text: each commit runs `cargo klarch install-agent-skills` and
commits the installed copies with their source. Every commit runs `cargo x gates`.

1. **Test 2.** The primer's test 2 and the decision-recording skill's sentence, per "Entry test 2 for
   installed text". Fails alone on: the wording of one test.
2. **The audit skill.** The skill of "The audit skill", named per D1; the primer's
   `primer@installed-skills` names it; the review skill's and the planning skill's scope say a review
   and a design audit are not it. Fails alone on: a skill that reads as a review.
3. **The design-record auditor.** The agent of "The design-record auditor". Fails alone on: an agent
   whose description cannot be dispatched from, or whose body edits.
4. **The triggers.** The setup skill's two steps; the CHANGELOG.md Migration entry naming the axis.
   Fails alone on: a pin move that reads no axis.
5. **The first run.** The axis runs on this repository under the skill, through the owner list, and
   its re-check judges AC1. Fails alone on: a draft or an edit the method did not prevent.
6. **The harvest and the changelog.** The rows of "Harvest"; the Workflow entries; the spec deleted.

## Order rationale

- 1 before 5: the run applies the new test 2.
- 2 before 3: the agent's description names the skill that dispatches it.
- 3 before 5: the run dispatches the agent.
- 4 before 6: the harvest records the convention the step built.
- 5 before 6: the harvest records the decisions after AC1 is judged.

## Defaults awaiting the owner

- **D1**, on `thread@design-record-audit@audit-is-an-activity`, found at assembly: the planning skill
  already has a "design audit" of a slice or a spec, so a skill named "audit" can be read as it.
  Default: the skill is named `project-audit`, the owner's name in the issue, "project audits".
- **D2**, on `thread@design-record-audit@audit-is-an-activity`, found at assembly:
  `goal@agent-skills@one-skill-per-activity` lists the activities of the workflow, and auditing is not
  among them; `goal@knowledge-architect@agents-get-a-complete-workflow` lists them too. A goal changes
  only on the owner's word, under `skill@knowledge-architect-goal-setting`. Default: at the harvest,
  the agent drafts the two goals with auditing added and puts them to the owner.

## Harvest

| item | where it lands |
| --- | --- |
| `thread@design-record-audit@audit-is-an-activity` | a head of `path@agent-skills@docs/design.md` |
| `thread@design-record-audit@audit-method` | a head |
| `thread@design-record-audit@design-record-axis` | a head |
| `thread@design-record-audit@audit-outcomes` | a head |
| `thread@design-record-audit@audit-triggers` | a head; `design@knowledge-architect@changelog-entries` gains the convention |
| `thread@design-record-audit@audit-agent-per-axis` | a head |
| `thread@design-record-audit@test-2-site-for-installed-text` | `design@agent-skills@a-head-is-owed-by-an-entry-test` rewritten, or a head, as the tests decide |
| `thread@design-record-audit@test-3-reading-not-practice` | `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` closed on the owner's ruling |
| T6, T7, T8 | `path@agent-skills@docs/tripwires.md`, each naming its head, with its label |
| AC1 | reported in the landing commit |
| every item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` | rewritten to the other axes and the report question |
| `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` | closed |
| D2 | the two goals, on the owner's word |

## Later consequences

- The other axes, the agentic-workflow axis first among them, each add an agent and a section of the
  audit skill, and adjust the method where they need to.
- An axis whose fixes are code may need a report document, which the issue keeps.
- Adopting projects meet the axis at their next pin move, through the Migration entry.
