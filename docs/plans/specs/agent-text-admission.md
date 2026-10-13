# One standard decides what enters agent-facing text, applied by every actor that writes or judges it

## Status and audience

This spec is the plan of the work that gives the workflow one standard for what enters
agent-facing text (the primer, a `CLAUDE.md`, a skill, a subagent definition) and how a change to
it is shaped, and makes every actor that writes or judges such text apply that standard. The work
does five things:

- it writes one section of `skill@knowledge-architect-agent-configuration`, `admission-standard`,
  holding the need test, the evidence an addition needs, its kind, its content, the shape of a
  repair and the record check; the audit skill's section `repair-standard`
  moves into it, and every other text points to it;
- it adds a review axis for a diff that adds or changes agent-facing text, carried by a new
  installed agent;
- it adds a lens to the agentic-workflow axis of the project audit, which reports statements that
  fail the need test;
- it widens the real-use rule of `design@agent-skills@behaviour-additions-need-real-use` to a
  project's own agent text;
- it applies the need test once to this repository's agent-facing text.

It is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest. Where it and a design home
disagree on a point that "What is already decided" does not list as reversed or rewritten, the
design home wins. Every name it uses is defined here or exists in the tree. Where the owner's word
is awaited, `skill@knowledge-architect-planning@working-a-slice` says what waits.

The discussion's transcript is one file, the harness's log of the session that converged, found by
its opening message, "agent instruction files should ask the follwoing question before getting
anything added". The assembly read it whole. Its rounds are named below as the assistant titled
them: Round 1 to Round 6, the Round 3 addendum (the message headed "The re-census result
invalidates part of round 3") and the convergence message. The owner's messages are O1 to O7, in
order.

## How the work is done

The work of a spec, as `skill@knowledge-architect-planning@working-a-slice` says.

## Names

| name | what it is |
| --- | --- |
| the agent-configuration skill | `skill@knowledge-architect-agent-configuration`, source `path@agent-skills@content/skills/agent-configuration/SKILL.md` |
| the standard | the section `admission-standard` of the agent-configuration skill, which this work writes |
| the need test | the test of `thread@agent-text-admission@need-test`, as the standard states it |
| agent-facing text | every text the harness delivers to an agent as an instruction or as knowledge: the primer, a root or scoped `CLAUDE.md`, a skill, a subagent definition, a saved workflow's prompts |
| installed text | the files `cargo klarch install-agent-skills` writes; in this repository, their source under `path@agent-skills@content/` |
| a maintainer-only statement | a statement whose only reader is an agent that maintains the text it sits in |
| the census | the read-only reading of this repository's agent-facing text under the need test, by independent subagents, that the discussion ran three times and that step 4 runs once more |
| a part of the census | one of five groups of the agent-facing text, each read by its own agents: the primer and the `CLAUDE.md` files; the design and planning skills; the setup, issue-tracking, project-audit and decision-recording skills; the review, retrospective, agent-configuration and goal-setting skills with this repository's own skills and agent; the installed agents |
| a hard case | a statement a census agent hesitated on, returned with both readings it saw |
| R1 to R5 | labels the discussion gave the need test's readings in Round 3 and its addendum, kept here only because the owner's words quoted below use them: R1, whose act counts; R2, a reason; R3, a duplicate inside one text; R4, a definition and an example; R5, a remark about the text itself. Outside this spec a reading is named by its content |
| Q1, Q2 | questions put to the owner in the discussion: Q1, which file held the thaum line and whether that session had loaded the agent-configuration skill; Q2, what "it" meant in the root `CLAUDE.md`'s release status |
| the new lens | L9, the need-test lens of `agent@knowledge-architect-workflow-auditor` |
| the new agent | the installed agent of the new review axis, `agent-text-reviewer`, installed as `agent@knowledge-architect-agent-text-reviewer` |
| the intake | `skill@klarch-retrospective-intake` |

## What the work is

**What exists today, at each site the work touches:**

- **The agent-configuration skill**, section `content-and-style`:
  four bullets, "Current reality only", "No cautionary narratives", "References are checked like
  anywhere else", and "An edit that changes what an agent is told to do is a decision", whose
  search names "the design homes and the goals". Its section
  `skill@knowledge-architect-agent-configuration@reviewing-a-change` says "This skill adds no axis
  of its own: a change to the configuration is a change to prose, which that skill's axes cover."
  No text of it states a test of whether a statement is needed; its section
  `skill@knowledge-architect-agent-configuration@shaping-a-skill` holds a test of whether a pointer
  may stand in place of the text it points to. Its scope sentence covers "the project's own agent
  configuration", and names the installed files as not covered.
- **The audit's section** `repair-standard`: the repair by
  cause and its four kinds, the instruction count and the net growth put to the owner, and the
  record check, which also reads the rejected alternatives. It is pointed to by the audit's own
  axis section (twice), by `agent@knowledge-architect-workflow-auditor@check-a-proposal`, by the
  intake's item f, by a JavaScript string of
  `path@agent-skills@content/workflows/agentic-workflow-audit.js` (the constant `KINDS`), and by the
  heads and issue listed under "What is already decided".
- **This repository's four tests**, in `path@agent-skills@CLAUDE.md`, "Editing an installed skill
  or agent": scope, necessity, kind, built intent; an edit of an installed agent passes the last
  three, since an agent has no expectation set. They bind only an edit of the installed text in
  this repository. The intake's item f restates them by name, and the root `CLAUDE.md`'s
  `instructions@repository-skills` points to them.
- **The retrospective**, `skill@knowledge-architect-retrospective@what-it-examines`, ends "End with
  **proposals**: for each finding that has one, what to change, and where", with no standard for
  the proposal's shape.
- **The review**, `skill@knowledge-architect-review@review-axes`: no axis judges whether an added
  statement is needed, and six of the seven installed reviewer agents, the transcript reviewer
  aside, are told "Do not report style preferences".
- **The agentic-workflow axis**: eight lenses, L1 to L8, in
  `agent@knowledge-architect-workflow-auditor@the-lenses` and in the `LENSES` table of the saved
  workflow; its finding classes in `agent@knowledge-architect-workflow-auditor@what-a-finding-is`;
  its outcome table in `skill@knowledge-architect-project-audit@agentic-workflow-axis`. An
  unneeded statement can be followed, so no class reports it. Hand-written ranges "L1 to L8" sit in
  the auditor's brief section and in the axis's `counts` row, and hand-written lists of the classes
  sit in the axis's "The rules" paragraph and in `design@agent-skills@behaviour-additions-need-real-use`.
- **Moving a live instruction out of a closing issue**: `skill@knowledge-architect-issue-tracking@movement-instruction`
  and the standing-state reviewer send it "to the owning document head … or the scoped `CLAUDE.md`";
  the primer's knowledge table sends a procedure to "the owning skill".

**Outside the work:**

- The scoped `CLAUDE.md` files stay outside the agentic-workflow axis's corpus, per
  `design@agent-skills@workflow-axis-corpus`: the new lens does not read them. The writer and the
  new review axis do apply the standard to them.
- Whether an instruction that several agent texts carry earns a design head, W5 of
  `issue@knowledge-architect@three-findings-of-a-received-retrospective-await-an-intake`: that
  intake owns it.
- Whether the intake becomes installed: `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`.
  Its closing discussion now reads the standard instead of `@repair-standard`.
- How a project supersedes installed text: `issue@agent-skills@patching-an-installed-skill`.

## What is already decided

**The decisions the design rests on, not argued again:**

- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@expectation-set-bounds-scope`,
  `design@agent-skills@instruction-record-is-minimal`, `design@agent-skills@restatement-size-test`,
  `design@agent-skills@primer-limit`, `design@agent-skills@workflow-auditor-agent`;
- `design@agent-skills@synthetic-evidence-not-built`: no run is built to observe how an agent
  follows the workflow's instructions, to choose between shapes among other purposes. Whether the
  census was such a run is open: the design-conformance review read it so, and the owner reads it as
  real use, a run over a real project with an instruction the project plans to add, while holding
  that the census's way of counting agreement shows why such evidence misleads. No shape of this
  spec rests on the census's counts either way, and the need test closed on its arguments and the
  owner's word;
- `design@agent-skills@shipped-text-line-comments`, the home of a `%%` line;
- `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@installed-text-leaves-room-to-judge`,
  `goal@agent-skills@installed-text-works-anywhere`, `goal@knowledge-architect@the-workflow-improves-through-real-use`.

**The decisions the work reverses or rewrites, with every text that references each, and the step
that updates it.** The lists are the "referenced at" output of `cargo klarch show`, taken when this
spec was written; step 5 takes them again.

- `design@knowledge-architect@intake-shares-the-repair-standard`: its home clause, "The standard
  has one home, the audit's section", and its losing rival, the agent-configuration skill, reverse;
  and its "The intake keeps its own evidence, … its kind for a lapse of the session" changes, since
  the lapse rule moves into the standard and the intake points to it.
  Referenced at: `skill@klarch-retrospective-intake` (item f). Step 1 re-points the
  intake; step 5 rewrites the head.
- `design@agent-skills@root-cause-repair`: its sentence "`…@repair-standard`
  is the one home of the standard" is rewritten. Referenced at: `register@agent-skills@design`
  (two heads), `path@agent-skills@docs/rejected-alternatives.md` (two entries),
  `register@agent-skills@tripwire` (four sites), `register@knowledge-architect@design`
  (`intake-shares-the-repair-standard`). Step 1 re-points its reference; step 5 rewrites it and
  re-reads each site.
- `design@agent-skills@repairs-read-against-the-record`: its sentence naming
  `@repair-standard` as the statement of the record check is rewritten. Referenced at:
  `register@agent-skills@design` (one head), the rejected alternatives (one entry), the tripwires
  (two sites). Step 1 re-points; step 5 rewrites.
- `design@agent-skills@behaviour-additions-need-real-use`: its scope widens from "every installed
  skill and every installed agent" to a project's own agent text, per
  `thread@agent-text-admission@real-use-scope`. Referenced at: `path@agent-skills@CLAUDE.md`, the design
  skill, seven heads of `register@agent-skills@design`, five issues of `register@agent-skills@issue`,
  `register@knowledge-architect@design`. Step 5 rewrites the head and re-reads each site.
  Its list of the finding classes that originate an edit gains an unneeded statement, per
  `thread@agent-text-admission@audit-reports-unneeded-text`.
- `design@agent-skills@workflow-axis-lenses`: a ninth lens. Referenced at:
  `register@agent-skills@design` (two heads), the rejected alternatives (three entries), the
  tripwires (five sites). Step 3 builds it; step 5 rewrites the head and re-reads each site.
- `design@agent-skills@workflow-axis-finding-standard`: a new class, an unneeded statement.
  Referenced at: the rejected alternatives (one entry), the tripwires (three sites). Step 3; step 5.
- `design@agent-skills@audit-outcomes`: the outcomes of `thread@agent-text-admission@need-lens-outcomes`.
  Referenced at: `register@agent-skills@design` (four sites), two issues of
  `register@agent-skills@issue`, `register@knowledge-architect@design`. Step 3; step 5.
- `design@agent-skills@audit-net-growth`: its sentence "as
  `…@repair-standard` says" names the section that leaves.
  Referenced at: the rejected alternatives (one entry). Step 1 re-points; step 5 rewrites.
- `design@agent-skills@audit-is-an-activity`: it says the audit skill "holds the method, the
  outcomes, the standard a repair of agent-facing text is judged by, and one section per axis"; the
  standard leaves that skill. Referenced at: `issue@agent-skills@audit-axes-beyond-the-design-record`.
  Step 5 rewrites it.
- `design@agent-skills@design-home-write-loads-recording`: it names
  `content-and-style` and "test 4" of
  `path@agent-skills@CLAUDE.md` as the texts that state its rule; both leave. Referenced at:
  `register@agent-skills@design` (two sites), the tripwires (one site). Step 1 re-points; step 5
  rewrites.
- `design@agent-skills@workflow-axis-corpus`: its cost sentence says the four edit tests of
  `path@agent-skills@CLAUDE.md` are "read against its homes by no audit"; after step 1 they are a
  pointer. Referenced at: the tripwires (one site),
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`. Step 5 rewrites the
  sentence and the issue.
- The installed sentence "This skill adds no axis of its own", in
  `skill@knowledge-architect-agent-configuration@reviewing-a-change`: no head states it. Step 2.

**Texts that cite `@repair-standard` or `@content-and-style`, which step 1 re-points:**
`skill@klarch-retrospective-intake`, `agent@knowledge-architect-workflow-auditor@check-a-proposal`,
`skill@knowledge-architect-project-audit@agentic-workflow-axis` (two sites), the `KINDS` constant
of the saved workflow, four heads of `register@agent-skills@design`, one head of
`register@knowledge-architect@design`, `path@agent-skills@CLAUDE.md`, and
`issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`.

**The standing entries the discussion's search returned**, 128 positions over three searchers, and
their outcome:

| entry | how it bears on the work |
| --- | --- |
| `issue@knowledge-architect@three-findings-of-a-received-retrospective-await-an-intake` | W5 is outside the work (above); its intake reads the standard |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | carries the owner's lead that "an audit and a review that share a subject are given compatible instructions"; the new lens and the new agent share the standard, and L8 gains the pair (step 3) |
| `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` | its closing discussion names `@repair-standard`; step 1 re-points it |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | names the four tests as a restatement no audit reads; step 1 turns them into a pointer, and step 5 updates the entry's text |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | the scope test stays in the retrospective; the standard points to it. Bears on nothing else |
| `issue@agent-skills@patching-an-installed-skill` | outside the work (above) |
| `issue@agent-skills@a-reviewer-s-running-time-is-unbounded` | an addition withdrawn under the real-use rule; bears on nothing in the work |
| `issue@core@references-are-read-in-markdown-and-rust-only` | the `KINDS` string is read by no check; `acceptance@agent-text-admission@no-stale-repair-standard-pointer` guards it |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | a citation of a point inside the standard stays in words |
| `tripwire@agent-skills@pointer-not-followed` | guards `design@agent-skills@restatement-size-test`; the step-1 pointers are its subject; re-read at the review before the merge |
| `tripwire@agent-skills@structure-rejection-overruled` | a need-test refusal may cite `design@agent-skills@capabilities-not-structure`; re-read at the review |
| `tripwire@agent-skills@expectation-set-closes-a-contradiction` | unaffected: the standard keeps "always in scope" for a contradiction |
| `tripwire@agent-skills@record-check-misses-a-head` | the record check moves home, unchanged in content; re-read at the review |
| `tripwire@agent-skills@audit-drafts-mostly-noise` | counts every lens's drafts, the new lens's included: it watches premortem cause 4 |
| `tripwire@agent-skills@auditor-skips-part-of-the-corpus`, `tripwire@agent-skills@audit-passes-do-not-converge`, `tripwire@agent-skills@audit-defaults-overruled` | the new lens adds a task, not corpus; each stays as it is and is re-read at the review |
| `tripwire@agent-skills@duplicate-auditors-add-nothing` | guards `design@agent-skills@workflow-axis-lenses`, on whose premise, a second agent on one lens finds what the others missed, L9's two auditors rest; it watches L9 like every duplicated lens, and is re-read at the review |
| `issue@agent-skills@a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise` | its closing rewrite must pass the tests of `path@agent-skills@CLAUDE.md`, which step 1 turns into a pointer to the standard; step 5 re-reads the entry and updates that sentence |
| `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search`, `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` | each set a repair aside as an instruction added on a prediction; the standard's evidence part keeps that rule, so each stays as it is |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | its closing adds steps to `skill@knowledge-architect-agent-configuration@after-installing`, a section the work does not touch; bears on nothing |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision` | `design@agent-skills@primer-limit` is the primer's own admission rule, which the standard points to; the entry's discussion reads the standard. Listed under "Later consequences" |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` | the owner's rulings it records, that a rarer checker rule is delivered by its finding message, agree with the need test: no act needs text the message carries. Bears on nothing |
| `issue@core@cross-project-references` | the owner's example points at another project's issue; the need test refuses such a statement for its content, whatever form a pointer could take. Bears on nothing |
| `issue@core@tooling-for-project-skills` | checks of a project skill's structure, not of its content; bears on nothing |
| `tripwire@agent-skills@argued-closure-reopened`, `tripwire@agent-skills@a-head-verdict-is-overruled` | the discussion reopened one thread, closed `approved`, and overruled no head verdict; neither fired. Re-read at the review |

The other entries of the three groups were read and judged to bear on nothing in the work.

## Criteria

### No two producers give an addition different verdicts `##no-two-producers-diverge`

Binding, from `goal@agent-skills@one-skill-per-activity` ("no two installed instructions a session
cannot both obey"), including the verdicts of two actors on one addition. Met by
`thread@agent-text-admission@admission-rules-home`, `thread@agent-text-admission@need-lens-outcomes` and
`thread@agent-text-admission@closing-moves-to-a-skill`.

### No structure or rule the work does not need `##no-unneeded-rule`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met by
`thread@agent-text-admission@need-test`, `thread@agent-text-admission@real-use-scope` and
`thread@agent-text-admission@census-cleanup`.

### Every producer applies the same standard `##every-producer-applies-it`

Binding, the owner's request in O1: "so that the workflow audit, all the reviewer subagents, the
retrospective intake and the installed skills cannot lead to additions that would be refused or
reshaped by one of the others". Met by `thread@agent-text-admission@admission-rules-home`,
`thread@agent-text-admission@review-judges-additions`, `thread@agent-text-admission@audit-reports-unneeded-text`
and `thread@agent-text-admission@closing-moves-to-a-skill`.

### Every other text points to the standard `##others-point`

Binding, from `design@agent-skills@restatement-size-test`: at most one sentence restated. Met by
`thread@agent-text-admission@admission-rules-home` and `thread@agent-text-admission@additions-review-agent`.

### The standard is installed text `##installed-home`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met: the agent-configuration
skill is installed.

### Each actor reads the standard when it judges `##read-when-judging`

Binding, from `design@agent-skills@repairs-read-against-the-record`: before the owner rules. Met by
the pointers of step 1, each "read whole", at the intake's item f, at the audit's repair step, in
the new agent and in the new lens.

## Threads

### Every statement in agent-facing text names the act it changes, or is not written `##need-test`

Proposed by the owner in O1: "agent instruction files should ask the follwoing question before
getting anything added: does an agent following the workflow need this information to carry its
work properly ?". The checkable form, "name the act of an agent that would differ without the
statement", is the assistant's, Round 1. Final state: approved. The arguments that moved it are
`argument@agent-text-admission@a1`, `argument@agent-text-admission@a3`, `argument@agent-text-admission@a4`, `argument@agent-text-admission@a5`, `argument@agent-text-admission@a18`, `argument@agent-text-admission@a20`, `argument@agent-text-admission@a21`, `argument@agent-text-admission@a23` and `argument@agent-text-admission@a24`: three census runs refined its readings, and the owner
reopened it on `argument@agent-text-admission@a20`. The shape is in
"Decided design", "The need test". Harvest: a head of `register@agent-skills@design`.

The owner's words: O2, "need-test: I agree with your shape and wording, I think it would clear the
problems I observed."; O3, "About R1: the statement intended for the agent text maintainer should
be kept concise and not take too much space in the text. Also, for the published workflow, it
should be written as %% comment lines in the agent-skills crate content directory."; O4, "one
sentence is fine for concise. Agreed on R5, R4, R3 and R1, though you can still refine them based
on census results."; O5, "To be honest, I'm worried we've narrowed the need test a bit too much. To
be honest, this is not really about agreement between two runs, but about stripping things that
are not fully necessary. I'd accept a bit of variation, since it would only mean that text not
passing the need test would just get flagged in a later pass. Can you show me the full standard
you've narrowed it down to, before we proceed ?"; O6, "The standard looks good now, let's go with
that."

The reading on a reason, R2, was not named by the owner; it closed argued in Round 5, on
`goal@agent-skills@installed-text-leaves-room-to-judge`, and was tightened in Round 6 within the
standard O6 approved.

### One section of the agent-configuration skill holds the standard, and every other text points to it `##admission-rules-home`

Proposed by the owner in O1: "I think the correct centralized location should be the agent config
skill." Final state: argued, Round 2. It reverses the home clause of
`design@knowledge-architect@intake-shares-the-repair-standard`, whose recorded reason the
discussion defeated (`argument@agent-text-admission@a6`). Rivals B, C and D ruled out (`argument@agent-text-admission@a7`, `argument@agent-text-admission@a8`, `argument@agent-text-admission@a10`, `argument@agent-text-admission@a11`). Shape: "Decided design",
"The standard's home". Harvest: a head of `register@agent-skills@design`, and the rewrites of the
three heads named in "What is already decided".

The owner's position, not a closing word: O2, "Overall, I think our tests for accepting content
into agent texts is well designed, so I think having it apply in full to consumer projects would be
positive. This is also part of the reason why I wanted to move everything to the skill." and "One
idea this opens: An installed agent might gain weight if it becomes the holder of the standard.
Though I still lean for the skill."

### The real-use rule covers a project's own agent text `##real-use-scope`

Proposed by the assistant, Round 1 (`argument@agent-text-admission@a9`). Final state: approved. Shape: "Decided design", "The
real-use rule's scope". Harvest: the rewrite of `design@agent-skills@behaviour-additions-need-real-use`.

The owner's words: O2, "real-use-scope: I agree with that. Overall, I think our tests for
accepting content into agent texts is well designed, so I think having it apply in full to consumer
projects would be positive. This is also part of the reason why I wanted to move everything to the
skill."

### A diff that adds or changes agent-facing text is reviewed against the standard `##review-judges-additions`

Proposed by the assistant, Round 1 (`argument@agent-text-admission@a13`). Final state: approved. The thaum evidence of Round 2 (`argument@agent-text-admission@a16`)
backs it. Shape: "Decided design", "The review axis". Harvest: a head of
`register@agent-skills@design`, or the head of `thread@agent-text-admission@additions-review-agent` if one head
holds both, as the harvest judges.

The owner's words: O2, "review-judges-additions: good idea. Judging diffs on branches is quite
important, it will avoid the large cost of audits."

### The review axis is carried by a thin installed agent, and the standard stays in the skill `##additions-review-agent`

Opened by the assistant in Round 2 from the owner's idea in O2 ("An installed agent might gain
weight if it becomes the holder of the standard"). Final state: approved. Shape: "Decided design",
"The review axis". Harvest: a head of `register@agent-skills@design`.

The owner's words: O4, "Agreed on census-cleanup and additions-review-agent."

### The agentic-workflow axis reports statements that fail the need test, through a lens of its own `##audit-reports-unneeded-text`

Proposed by the assistant, Round 1 (`argument@agent-text-admission@a15`), who leaned to widening L5; the owner chose a lens of its
own. Final state: approved. Shape: "Decided design", "The new lens". Harvest: the rewrites of
`design@agent-skills@workflow-axis-lenses` and `design@agent-skills@workflow-axis-finding-standard`.

The owner's words: O2, "audit-reports-unneeded-text: agreed, otherwise things are no longer
coherent together. All work that judges or edits agent text must follow the same standard. This is
critical. It would be done through another "lens"."

### A live instruction leaving a closing issue moves where the primer's knowledge table routes it `##closing-moves-to-a-skill`

Opened by the assistant in Round 1 from the inventory of rules. Final state: approved. The primer's
row "how to perform an activity → the owning skill" settles the contradiction. Shape: "Decided
design", "Closing moves". Harvest: none; it is a repair of two installed texts to the primer's
table.

The owner's words: O2, "closing-moves-to-a-skill: agreed."

### This repository's agent-facing text is brought to the need test in this work `##census-cleanup`

Proposed by the assistant, Round 3 (`argument@agent-text-admission@a19`). Final state: approved. Its scope grew with the standard in
Round 6 (`argument@agent-text-admission@a21`). Shape: "Decided design", "The cleanup". Harvest: none.

The owner's words: O4, "Agreed on census-cleanup and additions-review-agent."; O5, "Keep all
tripwires and AC2, not AC1."

### A confirmed need-test failure takes one outcome of one table, for the review and the audit alike `##need-lens-outcomes`

Proposed by the assistant, Round 5 (`argument@agent-text-admission@a22`). Final state: approved. Shape: "Decided design", "The
outcomes". Harvest: the rewrite of `design@agent-skills@audit-outcomes`.

The owner's words: O6, "need-lens-outcomes approved."

## Arguments

### The owner's example is useless to an agent, and goes stale with no work in the project `##a1`

O1, owner. Bears on `thread@agent-text-admission@need-test`. "useless information that won't affect how an
agent work and what it needs to do. It will also become stale even if no work is done in the
project, which is a smell in itself."

### Today the verdict on an addition depends on which actor met it first `##a2`

Round 1, assistant, from the inventory of 59 statements. Bears on
`thread@agent-text-admission@admission-rules-home` and `criterion@agent-text-admission@every-producer-applies-it`. A predicted
rule added to a root `CLAUDE.md` passes the agent-configuration skill, while the audit, meeting the
same gap first, makes it an issue: "So the outcome depends on which actor met the gap first." The
owner's example is caught "only by the writer's own reading of 'Current reality only' … No reviewer
and no audit lens judges that rule".

### The checkable form of the test is the act that would differ `##a3`

Round 1, assistant. Bears on `thread@agent-text-admission@need-test`. "**name the act of an agent that
would differ without the statement.**"

### Staleness is a tell, not a refusal `##a4`

Round 1, assistant. Bears on `thread@agent-text-admission@need-test`. The harness line of the root
`CLAUDE.md`, "which the harness refuses", is needed and can go stale, so "'can go stale with no work
in the project' is a tell, not a refusal". A content blacklist "grows one clause per observed case,
the mechanism `design@agent-skills@capabilities-not-structure` records, and it would refuse the
harness line."

### The need test and the real-use rule compose `##a5`

Round 1, assistant. Bears on `thread@agent-text-admission@need-test` and `thread@agent-text-admission@real-use-scope`. "The need
test asks of each statement whether an act depends on it. Real use is the evidence that a
behaviour-instruction's act would differ in practice."

### The recorded reason against the skill confuses a home with the moment of reading `##a6`

Round 1, assistant. Bears on `thread@agent-text-admission@admission-rules-home`. The head
`design@knowledge-architect@intake-shares-the-repair-standard` rejected the skill because it "is
read when an edit is written, after that judgement is made". "That reason confuses the home with
the moment of reading. The intake reads the standard at its step f because a pointer there says
'read whole', and that holds wherever the section lives." And "the most frequent producer, a
session writing configuration, loads the agent-configuration skill (its description is MUST) and
never reads `@repair-standard`."

### Rival B, the audit's section, loses on the knowledge table `##a7`

Round 1, assistant. Bears on `thread@agent-text-admission@admission-rules-home`. "It loses on the knowledge
table: 'how to perform an activity → the owning skill' … The audit, the intake and the reviews
judge that text; they do not own its standard."

### Rival C, the primer, loses on the primer's limit `##a8`

Round 1, assistant. Bears on `thread@agent-text-admission@admission-rules-home`. "It loses on
`design@agent-skills@primer-limit`. Writing agent text is not every session's work", with the caveat
of a session writing as a side effect without loading the skill, which `argument@agent-text-admission@a10` answers.

### The real-use rule's argument does not depend on who ships the text `##a9`

Round 1, assistant. Bears on `thread@agent-text-admission@real-use-scope`. The head's two parts, a predicted
behaviour is unreliable evidence and rules grow one per interaction: "Neither part depends on who
ships the text." The exemption of a project's own text "loses on the divergence it keeps: the
writer admits what the audit refuses". The owner's request mid-session stays admissible.

### The thaum session had the skill loaded, so the failure was the missing test `##a10`

Round 2, assistant, on the owner's answer to Q1 and thaum's history: commit 7b3de8ab added the line,
commit 8f7667fd removed it, both labelled `[agent-config]`. Bears on
`thread@agent-text-admission@admission-rules-home`. "the failure was that no need test existed, not that the skill
was missed. That defeats rival C." And "The reason you gave, 'useless', is the need test's verdict.
The test reproduces your ruling."

### Rival D, an installed agent holding the standard, loses on dispatch rule 4 `##a11`

Round 2, assistant. Bears on `thread@agent-text-admission@admission-rules-home`. Rule 4 of
`skill@knowledge-architect-agent-configuration@where-text-goes`: "Is it read only by a fresh
subagent? Then it is a subagent definition." "Two actors read this standard in the main session: the
writer of configuration and the intake. A main session reaches an agent definition only by opening
its file, and nothing makes it do so. A skill is loaded by its MUST trigger."

### Judging diffs avoids the cost of audits `##a12`

O2, owner. Bears on `thread@agent-text-admission@review-judges-additions`. "Judging diffs on branches is
quite important, it will avoid the large cost of audits."

### No axis judges the content rules today `##a13`

Round 1, assistant. Bears on `thread@agent-text-admission@review-judges-additions`. The skill's "adds no axis of
its own" is "false for the content rules: no axis judges them, and every reviewer agent is told 'Do
not report style preferences'." Six of the seven installed reviewer agents carry that sentence, the
transcript reviewer aside; the argument stands on the six.

### All work that judges or edits agent text must follow one standard `##a14`

O2, owner. Bears on `thread@agent-text-admission@audit-reports-unneeded-text` and
`thread@agent-text-admission@admission-rules-home`. "otherwise things are no longer coherent together. All work
that judges or edits agent text must follow the same standard. This is critical."

### An unneeded statement is never reported, so it never leaves `##a15`

Round 1, assistant. Bears on `thread@agent-text-admission@audit-reports-unneeded-text`. "An unneeded statement is
followable, so it is never reported, and it never leaves." Diff review alone "loses on text no
branch touches again, which is the case the audit exists for."

### Five review axes passed over the thaum line `##a16`

Round 2, assistant. Bears on `thread@agent-text-admission@review-judges-additions`. The message of thaum's 7b3de8ab
records "Five fresh reviewers … self-consistency, routing, decision record, standing state, and
[thaum's rules reviewer]", none of which reported the line; the transcript reviewer caught it
on fidelity, "'is reported upstream' firmer than what happened".

### A procedure rewritten into each brief drifts `##a17`

Round 2, assistant. Bears on `thread@agent-text-admission@additions-review-agent`. An axis with no agent is
briefed "with the axis's question … and the files that carry the standard", so "each dispatching
session would write the procedure into its brief anew, and a procedure rewritten per brief drifts".
"Every other review axis with a fixed standard has an agent." Cost: "about 60 to 100 lines".

### A reason passes even when its instruction has no exception, on a goal `##a18`

Round 3, assistant. Bears on `thread@agent-text-admission@need-test`. `goal@agent-skills@installed-text-leaves-room-to-judge`
says "The installed skills and agents give agents capabilities and the reasons for each
instruction, and impose no structure the work does not need". "So a reason passes even when its instruction has no exception."

### The cleanup measures the test before the first audit runs it `##a19`

Round 3, assistant. Bears on `thread@agent-text-admission@census-cleanup`. Leaving the fails to the first audit
with the new lens loses: "the lens would then run first on text no branch measured".

### The test strips what is not fully necessary, and a variation is caught by a later pass `##a20`

O5, owner. Bears on `thread@agent-text-admission@need-test`. "this is not really about agreement between two runs,
but about stripping things that are not fully necessary. I'd accept a bit of variation, since it
would only mean that text not passing the need test would just get flagged in a later pass."

### The narrowing optimised agreement, the wrong quantity `##a21`

Round 6, assistant. Bears on `thread@agent-text-admission@need-test` and `thread@agent-text-admission@census-cleanup`. "I narrowed
the test to win agreement between two agents. That optimised the wrong quantity. Under the third
run's readings, 2 statements fail in about 2,200 lines, and whole classes of unneeded text pass."
A second reason: "One reason meets it; a repeat adds no act." A duplicate: "a loaded text is read
whole, so the second copy delivers nothing the first does not". The surviving copy: "If two passes
each delete the copy they flagged, both copies go and the instruction is lost." Cost: "grows from
about a dozen deletions to several dozen".

### One outcome table, or the review and the audit diverge `##a22`

Round 5, assistant. Bears on `thread@agent-text-admission@need-lens-outcomes`. "The review agent and the audit
lens need one outcome table, or they diverge as the audit and the writer do today." Every failure
to the owner list "loses on the audit's own table … which already applies 'a rewording that
changes no instruction' in the branch. Deleting narration changes no instruction." Each lens draft
passes an adversarial verifier and each review finding is checked by the dispatching session.

### A maintainer statement is written as a `%%` line in installed text `##a23`

O3, owner, and Round 4, assistant. Bears on `thread@agent-text-admission@need-test`. "This matches
`design@agent-skills@shipped-text-line-comments`, which already gives `%%` lines to 'comments for
this repository'." A statement read by an agent of an installing project when it writes a brief "is
not maintainer-only, and it ships." "Concise" is "at most one sentence per point it serves"; a word
bound "loses on the argument recorded in `design@agent-skills@audit-net-growth`".

### The census's disagreement counted silence and hesitation as disagreement `##a24`

Convergence message, assistant, answering O6. Bears on `thread@agent-text-admission@need-test`. Of the first
run's 26 disputed items, 15 were a hard case of the other agent, 1 an explicit pass, and 10 not
mentioned: "only 1 of the 26 was a true pass-against-fail disagreement." "The standard you chose
stands, since it was not decided on agreement."

## New names, in one place

```text
crates/agent-skills/content/skills/agent-configuration/SKILL.md
  ## <title> {{slug:admission-standard}}     the standard; absorbs content-and-style and the audit's repair-standard
crates/agent-skills/content/agents/agent-text-reviewer.md
  name: knowledge-architect-agent-text-reviewer     the new agent, installed at .claude/agents/
crates/agent-skills/content/agents/workflow-auditor.md
  L9 need test                                      the new lens, in the-lenses
  an unneeded statement                             the new class, in what-a-finding-is
crates/agent-skills/content/workflows/agentic-workflow-audit.js
  { id: 'L9', name: 'need test', count: 2 }         a row of LENSES; counts accepts L9
existing: skill@knowledge-architect-project-audit@repair-standard, which leaves (its content moves)
existing: skill@knowledge-architect-agent-configuration@content-and-style, which leaves (its content moves)
```

The new files are `path@agent-skills@content/agents/agent-text-reviewer.md` and its installed copy,
`agent@knowledge-architect-agent-text-reviewer`.

## Decided design

### The need test

The text of the standard's need test, as O6 approved it (Round 6):

> **The need test.** Every statement in agent-facing text passes one test: **name the act of an
> agent following the workflow that would differ without it**: what the agent does, when, where,
> or a fact it must use. A statement for which no such act can be named is not written, and an
> existing one is deleted. The unit is a sentence, or a clause when only part of a sentence fails.
>
> - **Whose act.** Any agent that reads the text in its work counts, an agent that maintains the
>   text or writes a brief from it included. A statement whose only reader is the text's maintainer
>   is at most one sentence; in installed text it is a `%%` line of the source, since no installing
>   project maintains the installed text.
> - **A reason** passes when it states the mechanism an instruction relies on or the failure it
>   prevents, even when the instruction has no exception. A second reason for the same instruction
>   passes only when it names another mechanism or failure. A sentence that narrates where an
>   instruction came from or how it turned out fails: "learned by getting them wrong", "and it
>   worked".
> - **A duplicate inside one text fails while its twin stays.** A text is read whole once it is
>   loaded, so a second statement of the same act adds none. The finding names the copy that stays:
>   the one in the section whose subject it is. Across texts, `primer@where-knowledge-goes` decides.
> - **A lead, a transition or an emphasis** fails when the statements it introduces or stresses
>   already carry every act. A heading is structure, not a statement.
> - **A remark about the text itself** fails unless it tells a reader where to look or what to
>   edit: what the text is classified as, or that something is not restated here.
> - **A definition** passes when an instruction anywhere in the workflow uses the term. **An
>   example** passes when it shows how a definition or an instruction applies.
> - **A pointer** passes when its target is needed to act, or when it names the home of a
>   restatement.
> - **A statement that can turn false with no change to the project** (the state of another
>   project, a tool's version, a person's plan) is suspect: it is written only when an act needs
>   it. "This issue was reported upstream" fails, since no act differs.
>
> Wording, style and placement across files are judged by other rules.

In installed text, the example quotations of the reason bullet are the census's own; step 1 may
keep them, since they are examples a reader applies. The text may be reworded in step 1 where the
installed text's grammar asks it (a reference form, the `{{command}}` placeholder), and where it
would read two ways beside the skill's own text: `skill@knowledge-architect-agent-configuration@shaping-a-skill`
already says "A pointer passes when its target is not needed to act", of a pointer standing in for
inline text, so the need test's pointer bullet says that a pointer "is needed" rather than that it
"passes". A change of what it admits or refuses is a change of the decision, which goes to the
owner.

**Nearest rival**: the narrower standard of the third census run, under which a reason passes even
when the same text gives another, and a duplicate, a lead or an emphasis is out of scope. It lost on
O5 and `argument@agent-text-admission@a21`: it optimised the agreement between two readers, and left whole classes of unneeded text
passing.

### The standard's home

One level-two section of the agent-configuration skill, slug `admission-standard`, holds every rule
on what enters agent-facing text and how a change to it is shaped. Its parts, in this order:

1. **Who applies it**: every writer of agent-facing text, the review axis, the audit, the
   retrospective's proposals, a received-findings analysis where a project has one, and any skill
   that moves text into agent-facing text.
2. **The need test**, above.
3. **Evidence**: a defect provable by reading is repaired on reading; an instruction about how an
   agent behaves is added only on evidence from real use, in installed text and in a project's own
   text alike (`thread@agent-text-admission@real-use-scope`); a lapse, an instruction that exists and was not
   followed, is not repaired by adding text, except by moving where it is delivered when the lapse
   recurs at one point of delivery (moved from the intake's item d).
4. **Kind**: a capability is worth text; a conformance rule only where the default is
   systematically wrong; a structure only where the work needs it. For an installed skill, the scope
   test of its expectation set, which stays in `skill@knowledge-architect-retrospective@expectation-sets`.
5. **Content**: current reality only (no history, no dates, no commit hashes, no session records),
   which the need test's reason and remark bullets now carry for narratives.
6. **The shape of a repair**: the repair by cause, its four kinds, one repair per cause, the
   instruction count and the net growth put to the owner first. Moved whole from
   the audit skill's section `repair-standard`.
7. **The record check**: every goals home, the design heads that `{{command}} show` and a search of
   the design homes return, the rejected alternatives of each Component that owns an edited text,
   and the scoped `CLAUDE.md` files and code comments the reversal check greps. It replaces both the
   audit's record check and the narrower search of `@content-and-style`.
8. **The outcomes of a need-test failure**, the table below.
9. **What a change is**: "References are checked like anywhere else", and "An edit that changes
   what an agent is told to do is a decision", with its load of
   `skill@knowledge-architect-decision-recording` and its goal strain sent to the owner, both
   moved from `@content-and-style` unchanged except for the wider search of part 7.

Part 4's scope test of an expectation set applies to an installed skill only: an installed agent
has none, as `path@agent-skills@CLAUDE.md` says today.

The section `@content-and-style` is absorbed and leaves. The skill's scope sentence gains that the
standard also governs installed text where a project judges or writes it: the audit's findings on
installed text, the retrospective's proposals for it, and its source in the project that publishes
the workflow; the installed files are still never edited by a project. The skill's description names
the standard in place of "the content instructions". The section `@repair-standard` of the audit skill leaves; every text that cited either
section cites the standard, each with "read whole" where a judgement rests on it.

**Nearest rival**: rival B, the standard kept in the audit's section with the agent-configuration
skill pointing to it. It lost on the knowledge table's row "how to perform an activity → the owning
skill" (`argument@agent-text-admission@a7`), since maintaining the configuration is the activity that owns what enters it, per
`goal@agent-skills@one-skill-per-activity`. Rival C, the primer, lost on `argument@agent-text-admission@a8` and `argument@agent-text-admission@a10`; rival D, an
installed agent, on `argument@agent-text-admission@a11`.

### The real-use rule's scope

`design@agent-skills@behaviour-additions-need-real-use` covers every agent-facing text, a project's
own included. The owner's request mid-session still originates an addition, as the head already
admits. **Nearest rival**: a project's own text exempt; it loses on `argument@agent-text-admission@a9`.

### The review axis

A row of the table of `skill@knowledge-architect-review@review-axes`: the axis "agent-facing text",
the new agent, applicable when the diff adds or changes agent-facing text, installed or the
project's own. `skill@knowledge-architect-agent-configuration@reviewing-a-change` points to it in
place of "adds no axis of its own". This places the row in the review table rather than in the
agent-configuration skill's own list of added axes, as the owner ruled on D1.

The new agent is thin. It holds the axis's procedure and its report: for each statement the diff
adds or changes, name the act, then judge evidence, kind and content; for each failure, the outcome
the standard's table gives, and for a duplicate, the copy that stays. For the standard itself it
reads the standard whole from the installed skill's file: a subagent does not load a skill, and
the file on disk is the copy the work under review is judged against. Its report form follows the other reviewer agents': findings with their
reproduction, or a plain statement that the axis is clean. It does not carry "Do not report style
preferences" for the need test's classes, since those are not style; style stays out.

**Nearest rival**: a general-purpose subagent briefed with the axis's question. It loses on `argument@agent-text-admission@a17`.

### The new lens

L9, "need test", in `agent@knowledge-architect-workflow-auditor@the-lenses`: read the corpus for
statements that fail the need test, as the standard says, read whole. A new class in
`agent@knowledge-architect-workflow-auditor@what-a-finding-is`: **an unneeded statement**, the
statement quoted and the act the auditor tried, or, for a duplicate, both copies and the one that
stays. A row in the saved workflow's `LENSES` table, `{ id: 'L9', name: 'need test', count: 2 }`,
and `counts` accepts the key `L9`. Two auditors by default, on the premise
`design@agent-skills@workflow-axis-lenses` records from real passes, that a second agent on one
lens finds what the others of that lens missed, which `tripwire@agent-skills@duplicate-auditors-add-nothing`
watches. The census's counts are not its ground. L8's list of pairs gains the new agent and L9, which judge one subject. The lens list
of `skill@knowledge-architect-project-audit@agentic-workflow-axis` names L9, and the hand-written
lists the mapping table names gain it: the ranges "L1 to L8", the classes of the axis's "The
rules", whose opening "a session can follow every instruction" gains "and needs every statement",
and the auditor's description, "for instructions a session cannot follow", which gains "and for
statements no act needs".

**An unneeded statement in the run.** A confirmed finding of the class goes through the run's
cause step like any other: the cluster agents group such findings, and a group is a cause of its
own, with no kind, as the standard's repair part allows. Its repair is the outcome the table below
gives each finding, and it counts the instructions it removes, none added.

**Nearest rival**: widening L5, the assistant's first lean (Round 1). The owner chose a lens of its
own in O2.

### The outcomes

One table, in the standard, which the new agent and the audit's outcome table both apply:

| the failing statement | outcome |
| --- | --- |
| it carries no instruction: narration, a remark about the text, outside state no act uses, a duplicate whose twin stays, a lead or an emphasis whose statements carry every act | deleted in the branch: by the session, which applies an audit's outcomes, or as a review's repair |
| it is maintainer-only, in installed text | moved to a `%%` line in the installed source |
| it is an instruction whose act no agent performs | put to the owner, since deleting it changes what agents are told: on an audit's owner list, as a default awaiting the owner in a plan document, or as a question in the turn elsewhere |

In a project that installs the workflow, a finding on installed text, of any row, is sent upstream,
as every finding on installed text is, per `design@agent-skills@audit-outcomes`.
| a suspect pass | no finding; a suspect statement found false is a factual error |

The audit's outcome table in `skill@knowledge-architect-project-audit@agentic-workflow-axis` gains
the rows it needs by pointing to this table. **Nearest rival**: every failure to the owner list; it
loses on `argument@agent-text-admission@a22`.

### Closing moves

`skill@knowledge-architect-issue-tracking@movement-instruction` and the standing-state reviewer's
sentence on a closed entry's live content send it where the primer's knowledge table routes it,
each by a pointer to `primer@where-knowledge-goes`, instead of listing "the owning document head …
or the scoped `CLAUDE.md`". The standing-state reviewer's omission of "a goal" leaves with its list.

### The cleanup

Step 4 runs the census once under the standard over this repository's agent-facing text: the
installed source, the root `CLAUDE.md`, the scoped `CLAUDE.md` files, and the project's own skills
and agent, cut into the five parts the names table lists. Two general-purpose subagents per part,
each in a worktree and a scratch directory of its own, as
`skill@knowledge-architect-review@review-invariants` says. Each is briefed with the standard,
read whole from the installed agent-configuration skill, and its part, and returns as its final
answer every statement it flags, each with its verdict, the act it tried and the outcome row, and
every hard case, with the two readings. The session verifies each flagged statement and hard case
by reading, and each verified failure takes its outcome from the table: the first two rows are
applied in step 4; a statement of the third row is listed under "Defaults awaiting the owner" as a
`D<n>` and stays in place until the owner rules. No agreement threshold gates it, on O5. A
statement one pass misses is left to the next audit, as `argument@agent-text-admission@a20` accepts. The fixes
already made on this branch, "both learned by getting them wrong" in crates/core/CLAUDE.md among
them, count as done.

The step's commit message records its measurements: the statements each outcome row took, the
instructions added (none) and removed, and the word count of the corpus before and after,
`wc -w` over the files of the five parts.

## Mapping tables

The pointers step 1 re-points, each old site to its new form:

| site | today | after step 1 |
| --- | --- | --- |
| the intake, item f | "as `…@repair-standard` says, read whole" | the standard, read whole |
| the intake, item d, "a lapse of the session" | states the lapse rule | points to the standard's evidence part |
| `agent@knowledge-architect-workflow-auditor@check-a-proposal` | "against the record, as … `@repair-standard` says" | as the standard says |
| `skill@knowledge-architect-project-audit@agentic-workflow-axis`, "The repair by cause" | `@repair-standard` | the standard |
| the same section, the owner list's net growth | `@repair-standard` | the standard |
| the saved workflow's `KINDS` constant | "the section repair-standard of the skill knowledge-architect-project-audit" | the section admission-standard of the skill knowledge-architect-agent-configuration |
| `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent" | four tests restated | one sentence and a pointer to the standard, read whole; the two notes after the tests stay |
| the retrospective, "End with proposals" | no standard named | each proposal shaped as the standard says |
| `design@agent-skills@root-cause-repair`, `design@agent-skills@repairs-read-against-the-record`, `design@agent-skills@audit-net-growth` | cite `@repair-standard` | cite the standard (step 1 re-points; step 5 rewrites the claims) |
| `design@knowledge-architect@intake-shares-the-repair-standard` | cites `@repair-standard` and `@content-and-style` | re-pointed in step 1; rewritten in step 5 |
| two heads of `register@agent-skills@design` citing `@content-and-style` | `@content-and-style` | the standard |
| `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` | names `@repair-standard` | names the standard |
| the intake, item f, its sentence on an installed skill | "passes the four tests of the agent-skills `CLAUDE.md` … scope, necessity, kind, built intent" | passes the standard, as the agent-skills `CLAUDE.md`'s section says, read whole |
| the root `CLAUDE.md`, `instructions@repository-skills` | "passes the tests of `path@agent-skills@CLAUDE.md`" | passes the standard, as that section says |
| the axis's section, the check of a proposal | "makes the record check of that section" | makes the standard's record check |
| the axis's section, its outcome table | "a drifted restatement brought back to its home, which converts it to a pointer when it is longer than one" | a restatement whose content drifted from its home, brought back to it; since the repair edits what it says, it becomes a pointer when it is longer than one sentence, per `primer@where-knowledge-goes` |
| the saved workflow, the comment above `KINDS` and the repair prompt | "the skill's repair section", "the repair section" | the standard of the agent-configuration skill |

The lists step 3 extends, each with L9 or the new class:

| site | today | after step 3 |
| --- | --- | --- |
| `agent@knowledge-architect-workflow-auditor@your-brief` | "one of L1 to L8 below" | one of L1 to L9 |
| the auditor's frontmatter description | "for instructions a session cannot follow" | and for statements no act needs |
| the axis's section, the `args` table, `counts` | "keyed L1 to L8" | keyed L1 to L9 |
| the axis's section, "The rules" | "a session can follow every instruction", and the list of classes | gains "and needs every statement", and the class |
| the axis's section, the lens list | L1 to L8 | adds L9 need test |
| the saved workflow's header comment | "{ L1 … L8, clusters, repairsPerCause }" | L1 … L9 |
| `agent@knowledge-architect-workflow-auditor@the-lenses`, L8 | its list of pairs | adds the new agent and L9 |

## Losing alternatives

| alternative | lost to | the fact that decided |
| --- | --- | --- |
| a content blacklist, "no state of another project" added to "Current reality only" | `thread@agent-text-admission@need-test` | `argument@agent-text-admission@a4`: it grows one clause per case and refuses the needed harness line |
| the third census run's narrower readings | `thread@agent-text-admission@need-test` | `argument@agent-text-admission@a21` and O5 |
| a duplicate inside one text failing, with no rule on which copy stays | `thread@agent-text-admission@need-test` | `argument@agent-text-admission@a21`: two passes failing different copies delete both |
| rival B, the standard in the audit's section | `thread@agent-text-admission@admission-rules-home` | `argument@agent-text-admission@a7` |
| rival C, the standard in the primer | `thread@agent-text-admission@admission-rules-home` | `argument@agent-text-admission@a8`, `argument@agent-text-admission@a10` |
| rival D, an installed agent holding the standard | `thread@agent-text-admission@admission-rules-home` | `argument@agent-text-admission@a11` |
| a project's own text exempt from the real-use rule | `thread@agent-text-admission@real-use-scope` | `argument@agent-text-admission@a9` |
| a general-purpose subagent for the review axis | `thread@agent-text-admission@additions-review-agent` | `argument@agent-text-admission@a17` |
| widening L5 instead of a lens of its own | `thread@agent-text-admission@audit-reports-unneeded-text` | the owner's choice, O2 |
| detection left to diff review alone | `thread@agent-text-admission@audit-reports-unneeded-text` | `argument@agent-text-admission@a15` |
| the cleanup left to the first audit | `thread@agent-text-admission@census-cleanup` | `argument@agent-text-admission@a19` |
| every need-test failure to the owner list | `thread@agent-text-admission@need-lens-outcomes` | `argument@agent-text-admission@a22` |
| an agreement threshold between two census readers (AC1) | `thread@agent-text-admission@census-cleanup` | the owner's ruling, O5, and `argument@agent-text-admission@a24` |
| duplicates inside one text left to the restatement rule | `thread@agent-text-admission@need-test` | Round 6: that rule governs a restatement delivered in another text, per `design@agent-skills@restatement-size-test`, and nothing decides a duplicate inside one text |

A row whose thread earns no head, `thread@agent-text-admission@census-cleanup`'s two, stays in this spec and leaves
with it, unless a recording test of `skill@knowledge-architect-decision-recording@losing-alternatives`
admits it against the head of `thread@agent-text-admission@need-test`.

## Readings

The work reads no external specification.

## Premortem

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| 1 | a session edits a `CLAUDE.md` inside another task without loading the agent-configuration skill | `thread@agent-text-admission@admission-rules-home` | tripwire T1, on the owner's word (O5) |
| 2 | the readings were measured on two of the five parts of the census | `thread@agent-text-admission@need-test` | AC1, declined by the owner (O5); the cleanup reads all five parts without a threshold |
| 3 | the review agent reports hard cases as findings, and sessions learn to ignore the axis | `thread@agent-text-admission@additions-review-agent` | tripwire T2, on the owner's word (O5) |
| 4 | the lens's drafts are mostly refuted | `thread@agent-text-admission@audit-reports-unneeded-text` | survives: `tripwire@agent-skills@audit-drafts-mostly-noise` counts every lens's drafts |
| 5 | moving `@repair-standard` leaves a stale pointer where no check reads | `thread@agent-text-admission@admission-rules-home` | AC2, on the owner's word (O5): `acceptance@agent-text-admission@no-stale-repair-standard-pointer` |
| 6 | an owner overrules a refusal of the real-use rule on a project's own text | `thread@agent-text-admission@real-use-scope` | tripwire T3, on the owner's word (O5) |

The tripwires, as put to the owner in Round 5, written at the harvest in `register@agent-skills@tripwire`:

- **T1**, guarding the head of `thread@agent-text-admission@admission-rules-home`: fires when a review or a
  retrospective finds an addition to agent-facing text that fails the need test or the real-use
  rule, made in a session that did not load the agent-configuration skill. Response: reopen the
  home on rival C, a one-sentence pointer in the primer. Re-entry: the retrospective of that
  session, and the outcome of the review finding.
- **T2**, guarding the head of `thread@agent-text-admission@additions-review-agent`: fires when, in one review
  where the axis reports at least four findings, the session or the owner judges more than half of
  them to need nothing. Response: reopen the need test's readings in the standard, which the
  agent and the new lens both apply, since narrowing them for the agent alone would let the two
  judge one statement differently. Re-entry: the outcome step of each review,
  `skill@knowledge-architect-review@what-review-leaves`.
- **T3**, guarding `design@agent-skills@behaviour-additions-need-real-use`: fires when the owner
  overrules a refusal, made under the real-use rule, of an addition to a project's own agent text.
  Response: reopen `thread@agent-text-admission@real-use-scope`'s decision. Re-entry: the retrospective of that
  session.

## Acceptance criteria

### No pointer to a leaving section survives where no check reads it `##no-stale-repair-standard-pointer`

AC2, on the owner's word (O5). Guards `thread@agent-text-admission@admission-rules-home`. Judged at step 1, and
again at step 5. Observable: `grep -rn -e '@repair-standard' -e '@content-and-style' -e 'section repair-standard' -e 'repair section'`
over `path@agent-skills@content/`, the installed copies under .claude (skills, agents and
workflows) and the `klarch-` skills returns a line; or a word-form site of the mapping table, such
as the record check's "that section" in the axis's section, still names the leaving section. The
owner ruled on applying AC2 with the observable `repair-standard` alone; the session widened it to
the second leaving section and to the word forms the code-claims review found, and narrowed the
pattern so that the slug of `design@knowledge-architect@intake-shares-the-repair-standard` does not
match. Response: re-point the line in the step that judges it; if the line is in a
text the step did not list, add it to the mapping table and report it in the landing commit.

## Implementation sequence

1. **The standard.** Write the section `admission-standard` of the agent-configuration skill, as
   "The standard's home" and "The need test" say; remove `@content-and-style` and the audit's
   `@repair-standard`; apply the mapping table; repair `@movement-instruction` and the standing-state
   reviewer as "Closing moves" says; install. Fails alone on: `cargo klarch check` (a dangling
   section reference) and the observable of `acceptance@agent-text-admission@no-stale-repair-standard-pointer`.
   Its CHANGELOG.md entries land with it.
2. **The review axis.** Write the new agent, the review table's row, and the pointer of
   `@reviewing-a-change`; install. Fails alone on: the build of `path@agent-skills@build.rs` (the file
   maps to the install layout) and `cargo klarch check` (the installed copy and the agent's name).
   Its CHANGELOG.md entry lands with it.
3. **The new lens.** Write L9 and the class in the workflow auditor, the row of `LENSES` and the
   `counts` comment in the saved workflow, the lens list and the outcome rows of the axis's
   section, the lists of the step-3 mapping table, and L8's new pair; install. Fails alone on: the
   parse check of the saved workflow, wrapped as `path@agent-skills@CLAUDE.md` says, and
   `cargo klarch check`. Its CHANGELOG.md entry lands with it.
4. **The cleanup.** Run the census under the standard as "The cleanup" says, verify each flagged
   statement, and apply each verified failure's outcome; install. Fails alone on: a diff that
   removes an instruction, which the outcome table puts to the owner rather than to the branch.
5. **The harvest**, per the harvest table below, and the deletion of this spec.

## Order rationale

- Step 1 before step 2: the new agent reads the standard, which must exist.
- Step 2 before step 3: L8's new pair names the new agent.
- Step 3 before step 4: the cleanup's deletions are then reviewed with every judge of the standard
  in place, the new agent and the lens included.
- Step 4 before step 5: the harvest records the decisions as built, the census of step 4 included.

## Defaults awaiting the owner

- **D1**, on `thread@agent-text-admission@review-judges-additions`: the axis's row sits in
  `skill@knowledge-architect-review@review-axes`, applicable to every review whose diff adds or
  changes agent-facing text, rather than an axis that the agent-configuration skill adds to the
  reviews of its own activity, as Round 1 worded it. The reason: agent-facing text is also edited
  inside other activities, a closing issue's moved instruction or a plan's harvest, whose reviews an
  axis added by the agent-configuration skill would not reach. It widens where the axis applies.
  **Ruled**: the owner approved it, "D1: yes, it should apply to any diff with agent text changes,
  from the review skill."

## Harvest

| thread | home | when |
| --- | --- | --- |
| `thread@agent-text-admission@need-test` | a head of `register@agent-skills@design`, by the entry tests; the owner's ruling on whose act counts, that a maintainer-only statement is one sentence and, in installed text, a `%%` line, attributed to the owner | step 5 |
| `thread@agent-text-admission@admission-rules-home` | a head of `register@agent-skills@design`; rewrite `design@knowledge-architect@intake-shares-the-repair-standard`, `design@agent-skills@root-cause-repair`, `design@agent-skills@repairs-read-against-the-record`, `design@agent-skills@audit-net-growth`, `design@agent-skills@audit-is-an-activity` and `design@agent-skills@design-home-write-loads-recording`, and the cost sentence of `design@agent-skills@workflow-axis-corpus`; rivals B, C and D to `path@agent-skills@docs/rejected-alternatives.md` by the recording tests | step 5 |
| `thread@agent-text-admission@real-use-scope` | rewrite `design@agent-skills@behaviour-additions-need-real-use`: its scope, and its list of the classes that originate an edit | step 5 |
| `thread@agent-text-admission@review-judges-additions`, `thread@agent-text-admission@additions-review-agent` | one head or two of `register@agent-skills@design`, by `primer@design-heads`' test of one decision per head | step 5 |
| `thread@agent-text-admission@audit-reports-unneeded-text` | rewrite `design@agent-skills@workflow-axis-lenses` and `design@agent-skills@workflow-axis-finding-standard` | step 5 |
| `thread@agent-text-admission@need-lens-outcomes` | rewrite `design@agent-skills@audit-outcomes` | step 5 |
| `thread@agent-text-admission@closing-moves-to-a-skill`, `thread@agent-text-admission@census-cleanup` | no head | step 5 names them in its commit |
| T1, T2, T3 | `register@agent-skills@tripwire`, as `skill@knowledge-architect-decision-recording@ruled-tripwires` says | step 5 |
| the losing alternatives | `path@agent-skills@docs/rejected-alternatives.md`, each by the recording tests | step 5 |
| the CHANGELOG.md entries | `Next release`, per `design@knowledge-architect@changelog-entries`: Workflow entries for the review axis, the lens, the need test and the real-use rule's scope; a Migration entry for the need test, a rule a project's own agent text must meet, citing `skill@knowledge-architect-project-audit@agentic-workflow-axis` | steps 1 to 4, each with its change |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration`, `issue@agent-skills@a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise` | each sentence on the four tests updated to the pointer | step 5 |

## Later consequences

- The intake's installed shape, `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`,
  reads the standard as the shared standard of every received-findings analysis.
- W5 of `issue@knowledge-architect@three-findings-of-a-received-retrospective-await-an-intake` is
  judged against the standard's need test when its intake runs.
- A project that supersedes installed text, `issue@agent-skills@patching-an-installed-skill`,
  writes its superseding instruction under the standard.
- The discussion of `issue@agent-skills@primer-content-bundles-each-directive-s-decision` reads the
  primer's own admission rule, `design@agent-skills@primer-limit`, beside the standard's need test.
