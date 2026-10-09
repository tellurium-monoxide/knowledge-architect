# A project audit reads the whole record on one axis, and its first axis re-applies the rules on design heads and rejected alternatives

## Status and audience

This spec is the plan of the work that gives the workflow a way to audit one aspect of a whole
project, as opposed to reviewing a diff, and its first axis: the conformance of the decision record
to the rules on design heads and rejected alternatives. The work does five things:

- entry test 2 says what a site is for agent-facing text;
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
- **The spec and its work land on one branch**, on the owner's word in the reply to round 4: "I
  think we can land the spec and the work on the same branch though." No gate requires a separate
  merge: `cargo klarch commits` judges each commit against the installed set it holds, per
  `design@core@installed-entities-from-the-tree`.
- **The design audit** of `skill@knowledge-architect-planning@working-a-slice`, point 2, is owed
  when the work does not start in the session where the discussion converged, or when commits other
  than this spec's own land on main before the work starts. The work starts in the session where the
  discussion converged; a session that picks it up later takes the audit.
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
- After the four plan reviews, two more rulings came in the same transcript. "The `%%` message" is
  the agent's message that opens "Code claims has reported", and the owner's reply to it is "default
  approved". "The review message" is the agent's message that opens "The design-conformance reviewer
  has reported, and all four reviews are in", which put D1, D2, D4, D5 and D6 under the labels D1, D2, R1, R2 and R3, and the
  owner's reply to it is "Agreed on these defaults".
- **Labels.** The head-rules discussion, earlier in the same transcript, used T1 to T5 and AC1 to
  AC2. Round 2 of this discussion put its acceptance criterion as AC1 again. This spec labels it AC3,
  so that no label names two items in this file; "AC1" and "AC2" in a quotation of round 1 name the
  head-rules work's criteria.

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
- **the agentic-workflow axis**: the axis the issue names "consistency of the project's own agent
  workflows, its CLAUDE.md files and skills, combined with the workflow the project installs". The
  owner calls it the "agentic workflow" axis in the reply to round 1. It is not built by this work.
- **the head-rules work**: the work of the deleted plan document head-rules, which wrote
  `primer@design-heads` and its heads in `path@agent-skills@docs/design.md`.
  `git log --diff-filter=D --name-only -- docs/plans/` finds it, deleted by the commit "The harvest
  of the head-rules work: the decisions are recorded, the tripwires written, the issue closed, and
  the spec leaves". Its CHANGELOG.md entries, written by that commit in the `Next release` section,
  are four Workflow entries on the rules of design heads, and one Migration entry that moves a
  citation from the decision-recording skill's section entry-tests to `primer@design-heads`.
- **the audit skill**: the new installed skill, named per D1.
- **the design-record auditor**: the new installed agent that drafts the axis's verdicts for one
  group of entries.
- **a group**: a run of consecutive entries of one Component's design home and rejected
  alternatives, of at most 60 entries besides the calibration sample, per "The audit's method".
- **a draft**: one auditor's file per entry, in the shape of "The design-record auditor".
- **the owner list**: one message to the owner holding every finding that needs the owner's word,
  grouped by kind, each item under a label `F<n>` with a default.
- **the re-check**: fresh auditors dispatched over the same groups after the edits, whose clean
  reports are the acceptance of a run.
- **a site, for agent-facing text**: a text that states the instruction, per
  `thread@design-record-audit@test-2-site-for-installed-text`.

## What the work is

### The record as it stands

- **The workflow has no whole-project reading.** Every review of `skill@knowledge-architect-review`
  reads a diff. `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` records the gap and two
  worked instances; this repository's history holds a third, the sweep of the commit "The heads that
  cited an approval as their ground stand on their argument, and six bundles are split", with its
  drafts, owner list, applying agents and re-check (the head-rules work's acceptance criterion on the sweep).
- **Entry test 2** in `primer@design-heads` reads: "**the same reason must be respected at more than
  one site, or at none.** [...] At none: a decision about an absence ("we do not do X"), or a policy
  with no code of its own". `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined`
  records that a decision about installed text can be read as such a policy, and that an audit
  counted two skills stating one decision as one site. The clause "no site" is also stated by
  `skill@knowledge-architect-decision-recording@when-recording-happens` ("as a decision with no site
  of its own"), by `design@agent-skills@a-head-is-owed-by-an-entry-test` ("a policy or an absence has
  no site at all"), and by the rejected alternative "A second entry test that admits a decision
  because it constrains work not yet built" of `path@agent-skills@docs/rejected-alternatives.md`
  ("admits as a decision with no site").
- **Entry test 3** admits a decision whose argument turns on an outside tool's behaviour, read in its
  documentation or measured. `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` asks
  whether it admits too much.
- **Moving the pin**, `skill@knowledge-architect-setup@moving-the-pin`, has four steps: edit the pin,
  read the changelog of every version crossed, install, then run the gates and commit. It names no
  audit.
- **Adopting existing documentation**, `skill@knowledge-architect-setup@existing-documentation`,
  ends with a `todo` issue for the move, which "runs as a milestone when the owner schedules it".
- **The changelog's Migration entries**, per `design@knowledge-architect@changelog-entries`, say
  what a consumer must change in its own files. The head-rules work's entries in the `Next release`
  section are four Workflow entries and one Migration entry that moves a citation. No Migration entry asks a consumer to bring its design homes to the
  new rules, and the design-record axis's Migration entry is a new one.
- **The installed agents** are seven reviewers and the standing-entry searcher, under
  `path@agent-skills@content/agents/`. Each declares `tools: Read, Grep, Glob, Bash`, so read-only is
  enforced by the agent's text, not by its tools: the searcher's body forbids `Write` and `Edit` on
  the project and allows writes to its scratch directory only. The searcher's description carries its
  dispatch rule, groups of at most 60 entries, per `design@agent-skills@standing-entry-search-groups`.
  The review skill's "Not covered here" paragraph lists the seven reviewers, under "**being** any of
  the reviewers"; the searcher is named by the design and planning skills, which dispatch it.
- **No command is dedicated to listing design heads or rejected alternatives.** `cargo klarch issues`
  and `cargo klarch tripwires` list their registers. `cargo klarch model` prints every observation of
  the walk with its file and line, headings and slug definitions included, so the heads can be
  filtered from it. A rejected alternative carries no slug, and its shape varies: a paragraph that
  opens with its name in bold in four of the five files, a level-two heading in
  `path@gates@docs/rejected-alternatives.md`.
- **`design@agent-skills@ruled-items-labelled`** lists every label prefix; a label that reaches a
  committed document takes a prefix of its own kind. An audit's owner list reaches its commit
  messages.
- **Tripwires on the rules of design heads** fire on the kind of edit this axis makes:
  `tripwire@agent-skills@owner-intent-stripped`, `tripwire@agent-skills@split-rule-from-exception`,
  `tripwire@agent-skills@rule-title-wider-than-its-argument`,
  `tripwire@agent-skills@member-beyond-the-argument`, and
  `tripwire@agent-skills@head-created-without-deliberation`. The auditor reads them as the failure
  modes of its edits, stated in words, since shipped text cites no entry of this repository; the
  run's commit messages carry the deliberation of each split.
  `tripwire@agent-skills@a-head-verdict-is-overruled` fires on a review's verdict, not an audit's.

### The standing entries the work bears on

The two standing-entry searches of round 1 returned these, each with its outcome here:

- **Five open entries were this axis's output as issues.** The first run read each as an input to
  its head, and closed the four whose "What would close it" its edits did: the title of the head on
  directories named after the project, the title of the gates convention, the xtask head on shared
  code, and the klarch-prefix head's decision of the published workflow.
  `issue@agent-skills@primer-content-bundles-each-directive-s-decision` stays open.
- `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` asks the same
  question for a batch of received findings: handle now, open an issue, or close with a reason. The
  audit skill does not cover a received retrospective; the question stays open.
- `issue@agent-skills@expectation-sets-for-the-installed-skills`: the audit skill is one more skill
  with no expectation set. Needs nothing here: the issue covers every skill not listed.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: steps 2 and 3 write shipped text,
  which cites no entry of this repository, per the rule of `path@agent-skills@CLAUDE.md`. The issue
  stays open.
- `issue@knowledge-architect@a-mechanical-changelog-check`: the new convention is one more rule of
  `design@knowledge-architect@changelog-entries`, which a model of the changelog would derive from
  that head. Needs nothing here.
- `tripwire@core@issue-kind-list-grows`: the audit's issue outcomes use the existing kinds, `defect`
  for a head false of the code and `design` for a split or a move too large for the branch. Needs
  nothing.

### What is outside the work

- **The other axes the issue names**: design self-consistency, alignment of the code with the design
  and goals, the agentic-workflow axis (which takes the restatements, per the reply to round 1),
  standing state as a whole, goal coverage. They stay in
  `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`, which the commit that repaired this spec
  after its first plan reviews rewrote to them. The planning skill says the plan document that
  schedules a `todo` closes it in the commit that adds the plan document, so that the work is listed
  in one place at a time. This issue also holds work no plan schedules, so it is rewritten rather
  than closed, which keeps that purpose.
- **An audit report document**, for axes whose fixes are code: the owner left it "a question for
  later" in round 0; it stays in that issue.
- **Whether a head is true of the code**: left out of this axis, per the reply to round 1; a head the
  axis finds false of the tree gets an issue entry.
- **Test 3**: unchanged, per `thread@design-record-audit@test-3-reading-not-practice`.
- **The installed agents' tools lines**: unchanged. The auditor follows the searcher's precedent.

## What is already decided

The design rests on these, and does not argue them again:

- `goal@knowledge-architect@documentation-stays-consistent`, `goal@knowledge-architect@agents-work-without-drift`,
  `goal@knowledge-architect@the-owner-decides`, `goal@agent-skills@one-skill-per-activity`,
  `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@installed-text-leaves-room-to-judge`.
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@synthetic-evidence-not-built`
  (an applied verdict is confirmed by reading), `design@agent-skills@standing-entry-search-groups`
  (bounded groups), `design@agent-skills@a-scratch-directory-per-subagent`,
  `design@agent-skills@head-ground-is-the-argument`, `design@agent-skills@shipped-text-line-comments`
  (the `%%` line, this repository's place for a reason beside an instruction of the installed text),
  `design@agent-skills@shipped-text-cites-no-entry`.
- `design@agent-skills@primer-content` and `design@agent-skills@goals-change-through-goal-setting`: a conflict
  with a goal goes to the owner, under `skill@knowledge-architect-goal-setting`.
- `design@agent-skills@additions-need-real-use`: the evidence is the three worked instances and the
  owner's named lack in round 0, a migration of every adopting project's design homes at the next pin.

The work rewrites these decisions and texts. Each row names the texts that `cargo klarch show` lists
as referencing it, and where each is judged:

| decision or text | what changes | judged or updated at |
| --- | --- | --- |
| entry test 2 of `primer@design-heads` | a site for agent-facing text is a text that states the instruction; "no site" is an absence or a policy that no code and no text states | step 1 |
| `skill@knowledge-architect-decision-recording@when-recording-happens`, "as a decision with no site of its own" | reworded to the new test 2 | step 1 |
| the rejected alternative "A second entry test that admits a decision because it constrains work not yet built", "admits as a decision with no site" | reworded to the new test 2 | step 1 |
| the released 0.5.0 section of CHANGELOG.md, "a reason several sites or no site must respect" | unchanged: a released section's content never changes, per `design@knowledge-architect@changelog-entries` | none |
| `design@agent-skills@a-head-is-owed-by-an-entry-test`, "a policy or an absence has no site at all" | reworded to the new test 2. Its other referencing texts are read again: the lines of `path@agent-skills@docs/design.md` that cite it, `tripwire@agent-skills@a-head-verdict-is-overruled`, `tripwire@agent-skills@a-shortcut-decision-earns-a-head`, `issue@agent-skills@the-material-finding-duty-has-no-head`, `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`, and the rejected alternative above. `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` and `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` close | the harvest |
| `design@agent-skills@instruction-record-is-minimal` | read again: its exception for a decision that keeps two parts consistent stays | the harvest |
| `design@agent-skills@existing-heads-on-touch` | rewritten, per D4: a head is repaired when a change touches it, and the whole record is repaired by a project audit, at a migration or at the owner's request. Its sentence that a sweep repairs an approval cited as a ground is the narrower case the audit now covers. Its referencing text is read again: `issue@agent-skills@primer-content-bundles-each-directive-s-decision` | the harvest |
| the rejected alternative "A sweep that brings every head stating the instance as its rule to its rule at once" | leaves `path@agent-skills@docs/rejected-alternatives.md`: it is chosen now, per D4 | the harvest |
| `design@agent-skills@ruled-items-labelled` | its prefix table gains `F`, a project audit's findings put to the owner, which reach the audit's commit messages | step 2 writes the prefix where the skill asks; the head's table at the harvest. Its nine referencing texts, the primer and six skills that restate their own prefix and one line of `path@agent-skills@docs/design.md`, are unchanged: none restates the table |
| `design@knowledge-architect@changelog-entries` | gains the convention: a Migration entry that changes the rules on recorded content cites the axis to run | the harvest. Its referencing texts are unchanged, since the convention adds to the Migration test and makes no clause of any of them false: the root `CLAUDE.md` in `instructions@mechanical-validation`, `instructions@where-knowledge-goes` and the restatement of the three tests in `instructions@git-workflow`; the preamble of CHANGELOG.md and its crate copies; one line each of `path@knowledge-architect@docs/design.md` and `path@agent-skills@docs/design.md` and two of `path@core@docs/design.md`; `issue@knowledge-architect@a-mechanical-changelog-check`; `agent@klarch-changelog-reviewer`, which reads the head in full; `skill@klarch-release` |
| `skill@knowledge-architect-setup@moving-the-pin` and `skill@knowledge-architect-setup@existing-documentation` | run the axis a Migration entry cites; the move of existing documents runs the axis | step 4 |
| the primer's `primer@installed-skills`, the review skill's scope, the planning skill's scope | name the audit skill; a review stays a review of a diff, and a design audit stays the planning skill's | step 2 |
| the root `path@knowledge-architect@README.md`, which lists the workflow's activities | gains auditing | step 2 |
| `goal@agent-skills@one-skill-per-activity` and `goal@knowledge-architect@agents-get-a-complete-workflow`, which list the workflow's activities | auditing is an activity they do not list, and the first names "the review agents its skills dispatch", which the auditor is not, per D2 | the harvest, on the owner's word |
| `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` | rewritten to the other axes, the report question and the lessons on method | the commit that repaired this spec after its first plan reviews |
| `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` | closed | the harvest |
| `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` | closed on the owner's ruling | the harvest |

## Criteria

### The audit reaches what no diff review reads `##c1`

C1. Binding, derived from `goal@knowledge-architect@documentation-stays-consistent`. Met by
`thread@design-record-audit@audit-is-an-activity` and `thread@design-record-audit@design-record-axis`.

### Nothing in it is specific to one harness `##c2`

C2. Binding, derived from `goal@agent-skills@installed-text-works-anywhere`: subagents only, no
workflow tool, no convention of this repository. Met by `thread@design-record-audit@audit-method`
and, for test 2, by D3.

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

C8. Weighed. Unmet and accepted. Round 1 said "today's sweep took about 10 subagents for 189
heads", and "A project the size of thaum, about 220 heads, would cost about the same per run."
The owner, the reply to round 2: "Agreed on all, tripwires and AC approved." 189 was right at
commit c282b00, where the sweep counted it. The record has grown since, and the axis reads more than
the heads, so the cost of a run differs; D7 carried the corrected premise, and the owner accepted it. Re-taken at the commit
that added this spec, by
counting the level-three headings that end with a slug in the five design homes: 206 heads. The
rejected alternatives add about 100 entries, counted as the paragraphs that open with bold text,
and the level-two headings of `path@gates@docs/rejected-alternatives.md`. Groups of at most 60 entries per Component give 9 groups in this repository, so a run costs
about 9 auditors and 9 re-check auditors.

## Threads

### A project audit is an activity with an installed skill of its own, and a review stays a review of a diff `##audit-is-an-activity`

Proposed by the agent, round 1. Approved. The skill holds the method and one section per axis.
Arguments: `argument@design-record-audit@a5`, `argument@design-record-audit@a6`. Shape: "Decided
design". Harvest: a head of `path@agent-skills@docs/design.md`. The owner's words, the reply to
round 1: "audit-is-an-activity : agreed on that shape."

### The audit's method: scope, groups, calibration, drafts, a verified sort, one owner list, disjoint application, a fresh re-check, a reviewed branch `##audit-method`

Proposed by the agent, round 1, from the three worked instances. Approved, generic for now and
adjusted when another axis comes. Arguments: `argument@design-record-audit@a3`,
`argument@design-record-audit@a4`, `argument@design-record-audit@a7`, `argument@design-record-audit@a8`,
`argument@design-record-audit@a24`, `argument@design-record-audit@a25`,
`argument@design-record-audit@a26`. Shape: "Decided design". Harvest: a head of
`path@agent-skills@docs/design.md`. The owner's words, the reply to round 1: "audit-method: that
part is quite hard to do properly IMO, because it might differ per axis. For now, we are only
providing one axis, so we can adjust this generic shape later for the others. But I think your
general shape should hold its ground for the other axis too. We can go with that for now."

### The design-record axis audits the design heads and the rejected alternatives, reading the tripwires and issues that bear on each head `##design-record-axis`

Proposed by the agent, round 1, with three more members; narrowed by the owner in the reply to round
1; the agent's reading of that word stated in round 2, presumed-settled, and approved in the reply to
round 2. The tripwires and issues are inputs: the auditor reads every tripwire guarding a head and
every issue naming it before judging it, and the audit repairs one of them only where an edit makes
it stale. Restatements of heads go to the agentic-workflow axis; whether a head is true of the code
is left out. Arguments: `argument@design-record-audit@a9`, `argument@design-record-audit@a10`,
`argument@design-record-audit@a11`. Shape: "Decided design". Harvest: a head of
`path@agent-skills@docs/design.md`. The owner's words, the reply to round 1: "design-record-axis:
agreed on the primer and the rejected alternatives. About the others: they are not really "design".
Reading the tripwires and issues related to design is important IMO, so that the audit on design
takes them into account. The restatement item, I find it weird to add it in this axis. It does not
read the same thing at all. It would go in the "agentic workflow" axis mentionned in the issue,
instead, IMO. Agreed to leave the "true of code" part out of this audit axis."; the reply to round 2:
"Agreed on all".

### Each finding is applied in the audit's branch, put to the owner, or opened as an issue `##audit-outcomes`

Proposed by the agent, round 1. Approved, then corrected by D5: a conflict with a goal goes to the
owner, not to an issue entry. The commit messages are the record; no report document for this axis.
Arguments: `argument@design-record-audit@a1`, `argument@design-record-audit@a12`. Shape: "Decided
design". Harvest: a head of `path@agent-skills@docs/design.md`. The owner's words, the reply to round
1: "audit-outcomes: agreed on those conditions."; the reply to the review message: "Agreed on these
defaults".

### The axis runs at the owner's request, at a pin move whose Migration entry cites it, and at the move of an adopting project's existing documents `##audit-triggers`

Proposed by the agent, round 1. Approved. Arguments: `argument@design-record-audit@a13`,
`argument@design-record-audit@a14`. Shape: "Decided design". Harvest: a head of
`path@agent-skills@docs/design.md`, and `design@knowledge-architect@changelog-entries` gains the
convention. The owner's words, the reply to round 1: "audit-triggers: agreed on the main trigger
being my request. Good idea on the changelog naming the axis to run. And agreed for the existing docs
of an adopting project."

### Each axis has an installed agent of its own, holding the axis's standard `##audit-agent-per-axis`

Proposed by the agent, round 2. Approved. Arguments: `argument@design-record-audit@a15`,
`argument@design-record-audit@a16`. Shape: "Decided design". Harvest: a head of
`path@agent-skills@docs/design.md`. The owner's words, the reply to round 2: "Agreed on all".

### The open questions on the entry tests are settled before the audit ships `##entry-tests-settled-first`

Proposed by the owner, the reply to round 2. Approved. Arguments: `argument@design-record-audit@a17`,
`argument@design-record-audit@a18`. Shape: the implementation sequence, step 1 first. Harvest: none
of its own; the threads below carry the decisions. The owner's words, the reply to round 2: "I don't
think we can ship this audit workflow without fixing them."; the reply to round 3:
"entry-tests-settled-first: agreed."

### For agent-facing text, each text that states an instruction is one site, and "no site" is an absence or a policy no text states `##test-2-site-for-installed-text`

Proposed by the agent, round 3, as the first of the two shapes the issue lists. Approved; its wording of where the reason
lives corrected by D3. Arguments: `argument@design-record-audit@a19`,
`argument@design-record-audit@a20`. Shape: "Decided design". Harvest:
`design@agent-skills@a-head-is-owed-by-an-entry-test` rewritten, since it holds the argument of
every entry test; the issue closes. The owner's words, the reply to round 3: "test-2-site-for-installed-text:
we can go with your solution, it looks better than the current ambiguity."; the reply to the `%%`
message: "default approved".

### Test 3 admits only a reading of an outside thing that cost the project work `##test-3-reading-not-practice`

Proposed by the agent, round 3. Ruled out by the owner in the reply to round 3, on the owner's
argument. Arguments: `argument@design-record-audit@a21`, `argument@design-record-audit@a22`. The
owner's words: "Unless you think there is a real inconsistency in the entry tests, I think we can keep
as is. I'd rather keep the tests simpler than make them harder to judge, which is more likely to
reduce quality of the design homes."

### A project audit repairs the whole record at once, beside the repair of a head a change touches `##whole-record-repair-at-a-migration`

Raised by the design-conformance review of this spec; proposed by the agent in the review message as
R1, recorded here as D4. Approved. The design-record axis applies every rule to every head at once,
which is the alternative "A sweep that brings every head stating the instance as its rule to its rule
at once", recorded as lost to `design@agent-skills@existing-heads-on-touch`. This thread reopens it.
Arguments: `argument@design-record-audit@a3`, `argument@design-record-audit@a23`. Shape: "Decided
design". Harvest: `design@agent-skills@existing-heads-on-touch` rewritten; the rejected alternative
leaves. The owner's words, the reply to the review message: "Agreed on these defaults".

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
`thread@design-record-audit@audit-triggers`, `thread@design-record-audit@whole-record-repair-at-a-migration`.
"With the changes we just did to decision record conditions and shapes, at the next version pin,
projects migrating will need to migrate their design home to match them. We need to provide a way
for the workflow to efficiently cover the recorded design and reapply the record tests to them."

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
depending on the brief. A second axis adds an agent, not a branch inside one text." The first
sentence is false of the installed agents: each declares Bash, which can write, and read-only is
enforced by the agent's text. The other two sentences carry the argument.

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
site. The installed text already has that local home, a `%%` line beside the instruction". The last
sentence holds of this repository only, since the build removes every `%%` line; D3 restates it for
every project.

### The narrower test 2 moves some heads to the text beside their instruction `##a20`

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

### A migration is the occasion a repair on touch never reaches, and its widenings still go to the owner `##a23`

The review message, the agent. Bears on `thread@design-record-audit@whole-record-repair-at-a-migration`.
"Your argument in round 0 defeats that reason: at a pin move, every adopting project must bring its
design homes to the new rules, and an audit is the way to do it. The widenings still go to you,
through the owner list." The recorded reason was "the sweep would put that many widenings to the
owner at once"; the owner list groups them by kind with a default each, the shape the head-rules
sweep used.

### A fold judged alone, without the drafts, had to be undone `##a24`

Round 1, the agent, under what failed in the head-rules sweep. Bears on
`thread@design-record-audit@audit-method`, its drafts. "I folded a head, the in-change reread, into
another head and had to undo it, because I judged it alone, without the drafts' discipline."

### A finding lost its only record when the plan document that held it left `##a25`

Round 1, the agent, under what failed in the head-rules sweep. Bears on
`thread@design-record-audit@audit-method` and `thread@design-record-audit@audit-outcomes`. "A finding
lost its only record when the spec that held it was deleted." The instance was repaired by the commit
"Six reviewers read the head-rules work: a lost reference rule restored, the restatement rule's scope
recorded, the reread head split, and every head in the present tense". The method keeps each
finding's outcome in a commit message or an entry, never in a scratch file or a plan document alone.

### An audit that measures states its pre-commitments before it runs `##a26`

Round 1, the agent, from the issue. Bears on `thread@design-record-audit@audit-method`. "What I take
from the issue: 'pre-commitments stated before it runs' (point 3, for an audit that measures)." The
issue's lesson: "pre-commitments stated before it runs; the owner's reading of a sample, since agents
applying a test measure how agents read it, not whether it is right".

## New names, in one place

- the audit skill: `path@agent-skills@content/skills/project-audit/SKILL.md`, per D1.
- the design-record auditor: `path@agent-skills@content/agents/design-record-auditor.md`.

```text
the audit skill, installed as  knowledge-architect-project-audit
the auditor, installed as      knowledge-architect-design-record-auditor
the axis's section             the level-two section of the audit skill whose slug is design-record-axis
the label prefix F             a project audit's findings put to the owner
```

The installed names are written without backticks, here and everywhere in this spec: the checker
reports a backticked one-word span that is exactly the name of an installed skill or agent, fenced
or not, per `design@core@bare-skill-name-reported`.

## Decided design

### Entry test 2 for agent-facing text

Shape. Test 2's last sentence becomes, as a target wording that step 1 may adjust without changing
its meaning: "At none: a decision about an absence ("we do not do X"), or a policy that no code and
no text of the project states. For a decision about agent-facing text, a skill, an agent or a
`CLAUDE.md`, each text that states the instruction is one site: a decision stated by one text has one
site, and its reason lives in that text, beside the instruction." A `%%` line beside it in the
primer's source says that in this repository the place beside an instruction of the installed text
is a `%%` line, citing `design@agent-skills@shipped-text-line-comments`. The three other texts that
state "no site" follow the same wording.

Argument: `argument@design-record-audit@a19`. The nearest rival, test 2 as it reads, pointing to
`design@agent-skills@instruction-record-is-minimal`, keeps the clause that admits every decision
about installed text as a policy, and the miscount the issue records. The cost is heads about
agent-facing text stated by one text alone, which the first run finds and puts to the owner
(`argument@design-record-audit@a20`). D3: the shipped text names no convention of this repository,
per `goal@agent-skills@installed-text-works-anywhere`; the rival wording, "whose `%%` line is its
reason's home", names one.

### The audit skill is an activity of its own

Shape. One installed skill, named per D1. Its description begins with MUST and names its symptoms:
the owner asks for an audit of one aspect of the whole project; a Migration entry of a version
crossed at a pin move cites an axis; an adopting project's existing documents have moved. It states
what it does not cover: a review of a diff (`skill@knowledge-architect-review`), the design audit of
a slice or a spec (`skill@knowledge-architect-planning`), and a received retrospective. It holds the
method, the outcomes, and one section per axis, the first the design-record axis. The primer's
`primer@installed-skills` names it; the review skill's and the planning skill's scopes name it as not
theirs.

Argument: `argument@design-record-audit@a5`. The nearest rival, a whole-project mode of the review
skill, mixes what a diff owes with what the whole record owes. The other rival, a skill per axis,
restates the method in each (`argument@design-record-audit@a6`).

### The audit's method

Shape. The skill states each point with the judgement it needs, and an axis's section adjusts it:

1. **Corpus.** The axis names its corpus and its inputs.
2. **Groups.** A group holds the entries of one Component only, since a rejected alternative is
   judged against the head it lost to. A Component's entries are ordered: its design home first,
   a directory-shaped home's subdocuments in the order its README links them, then its rejected
   alternatives in file order, each entry as the file shapes it, a heading or a paragraph that opens
   with its name in bold. They are cut into ceil(count / 60) runs of consecutive entries, the sizes
   of any two runs differing by at most one. The bound of 60 is the one
   `design@agent-skills@standing-entry-search-groups` argues for a standing entry; an auditor also
   reads, per head, its tripwires, its issues and its history, and no measurement shows the bound
   right for that load, which T6 and AC3 watch. The session lists the heads from
   `cargo klarch model` or by reading, counts the entries, and names each group by its first and
   last entry.
3. **Pre-commitments and calibration.** An axis that measures, one whose run yields a figure a
   conclusion rests on, states before the run which figure would show which conclusion; an axis
   that only applies rules, as the design-record axis does, owes none. The session picks a shared
   sample of about five entries from different Components, a size no measurement fixes, and adds it
   to every group beyond its bound. When the drafts are in, it compares the verdicts on the sample.
   Where they differ and one follows the rule's text, the session settles the reading, states it in
   the run's commit message, and itself re-reads, in every group, the drafts that the difference
   bears on, before any draft is sorted. Where the rule's text admits both, the difference goes on
   the owner list, as a verdict where a test reads two ways.
4. **Drafts.** One auditor per group, in a scratch directory of its own, writes one draft per entry
   there and edits nothing else. Its brief names the commit audited, its group's first and last
   entry, the sample's entries, and its scratch directory.
5. **The sort.** The session reads every draft whose outcome is not "nothing", against the entry and
   its history, and a sample of the drafts that find the entry conforming, at least one per group. It sorts each
   finding into its outcome. A draft it finds wrong is corrected, and the group's other drafts of the
   same kind are read again.
6. **The owner list.** One message, the items grouped by kind, each under a label `F<n>` with a
   default. The session applies nothing on the list before the owner answers.
7. **Application.** The session, or agents it dispatches on disjoint files, applies the edits. The
   session reads the whole diff before each commit. Each commit message records what it applied:
   what was audited, at which commit, the counts, and each finding's outcome. A run that changes no
   file leaves no commit, since a commit that changes no file does not reach main, per
   `design@knowledge-architect@no-branch-sha-is-cited`; its record is then the report to the owner.
8. **The re-check.** Fresh auditors over the same groups, after the edits, each briefed as in point
   4 and also given the entries and rules the owner ruled to keep, as a file in its scratch
   directory. Each writes its drafts as in point 4. A draft that finds a rule failed, and is not on
   the kept list, is a violation left, whether a draft missed it or an edit made it.
9. **Review.** The audit's commits are reviewed on these axes of `skill@knowledge-architect-review`:
   self-consistency, fidelity of relocation, routing of knowledge, decision recording, standing
   state, and the transcript review last. An audit runs in a branch of its
   own, unless the owner gives it to another work's branch, as the first run is.

Arguments: `argument@design-record-audit@a3`, `argument@design-record-audit@a4`,
`argument@design-record-audit@a7`, `argument@design-record-audit@a8`,
`argument@design-record-audit@a24`, `argument@design-record-audit@a25`,
`argument@design-record-audit@a26`. The nearest rival, a lighter method with no calibration and no
re-check, lost because the re-check found real defects each time it ran.

### The design-record axis

Shape. The corpus is every design head of every Component and every entry of its rejected
alternatives. The inputs, for each head: every tripwire guarding it and every issue naming it, read
with `cargo klarch show`, and the history behind each citation of the owner: the commit messages
that touch its slug, `git log -G'<slug>'`, and the deleted plan documents,
`git log --diff-filter=D --name-only -- docs/plans/`, read as the decision-recording skill's
section `skill@knowledge-architect-decision-recording@three-homes` shows. The rules: every rule of
`primer@design-heads` for a head, and every rule of
`skill@knowledge-architect-decision-recording@losing-alternatives` for a rejected alternative. A
tripwire or an issue is repaired only where an edit of the run makes it stale. Restatements of heads
and the truth of a head about the code are not in this axis.

Arguments: `argument@design-record-audit@a9`, `argument@design-record-audit@a10`,
`argument@design-record-audit@a11`. The nearest rival, the axis with restatements and the truth of
the code in it, reads another thing than the record.

### The outcomes

Shape. Every finding takes exactly one outcome. The list is total: a finding that fits none of the
named cases goes to the owner list.

| outcome | the findings |
| --- | --- |
| applied in the branch | a rewording; the tense; a missing or wrong reference; an approval cited as a head's ground, removed when the head stands on its argument; a split that touches no word of the owner; a rejected alternative repaired in place: its reason made checkable, its marker set; a tripwire or an issue made stale by an edit, repaired in place |
| put to the owner, on the owner list | a widening of what the owner's words approved; the removal of a head that fails every entry test; the removal of a rejected alternative that fails every recording test; a ground that is the owner's words; a verdict where a test reads two ways; a head in the wrong Component; a conflict with a goal, handled under `skill@knowledge-architect-goal-setting`; any finding the other rows do not name |
| an issue entry | a head false of the code, which this axis does not judge, kind `defect`; a split or a move the owner approved that is too large for the branch, kind `design`; a finding the owner defers, kind `todo`, or `deferred` where the owner names the event that should make someone do it |
| nothing | the entry conforms |

Arguments: `argument@design-record-audit@a1`, `argument@design-record-audit@a12`. The nearest rival,
a report document, is left to an axis whose fixes are code (`argument@design-record-audit@a2`). D5
moved a conflict with a goal from the issue row to the owner.

### The design-record auditor

Shape. One installed agent, the design-record auditor of "New names, in one place", dispatched once
per group. Its description carries the dispatch rule of the method's point 2 with a worked example,
and its body a section on its brief, as the standing-entry searcher's does: the brief of point 4,
and, for a re-check, the kept list of point 8. Its tools line is the searcher's, and its body forbids `Write`
and `Edit` on the project and allows writes to its scratch directory only. Its body is the axis's
standard:

- read `primer@design-heads` and `skill@knowledge-architect-decision-recording@losing-alternatives`
  whole first; and keep in view the failure modes of an edit, stated in words since the agent cites
  no entry of this repository: removing the owner's own words with an approval; splitting a rule
  from its own exception; a title wider than its argument; a member the argument does not cover; a
  head created with no deliberation recorded;
- for each head of its group, read every tripwire guarding it and every issue naming it, with
  `cargo klarch show`, and the history behind every citation of the owner;
- judge each head against every rule of `primer@design-heads`, and each rejected alternative against
  its rules;
- write one draft per entry, and return the scratch directory and one row per entry: the entry,
  the verdict and the proposed outcome.

A draft holds: the entry, by its reference or, for a rejected alternative, its name and file; the
verdict, `conforms` or each rule it fails; the evidence, quoted, with the commits read where the
verdict rests on a ruling; the proposed edit as the text before and after; and the proposed outcome,
one of the four of "The outcomes", with its reason.

Arguments: `argument@design-record-audit@a15`, `argument@design-record-audit@a16`. The nearest rival,
one generic auditor briefed to read the axis's section, puts the standard behind a read the brief
must ask for.

### The triggers

Shape.

- **The owner's request**, the main one.
- **A pin move.** `skill@knowledge-architect-setup@moving-the-pin` gains a fifth step, after the
  commit of step 4: where a Migration entry of a version crossed cites an axis's section of the audit
  skill, run that axis under the audit skill, in a branch of its own. Step 2, which reads the
  changelogs, lists those entries for it.
- **The convention.** A Migration entry that changes the rules on recorded content cites the axis
  to run, as a reference to the axis's section of the audit skill. The `Next release` section of
  CHANGELOG.md gains one, surface `agent-skills`, class patch, which asks a consumer to bring its
  design homes and rejected alternatives to the rules of the head-rules work and of the new test 2,
  and cites the design-record axis's section.
- **The move of an adopting project's existing documents.** Point 4 of
  `skill@knowledge-architect-setup@existing-documentation` says that the milestone of the move ends
  by running the design-record axis over the design homes and rejected alternatives the move wrote.

Arguments: `argument@design-record-audit@a13`, `argument@design-record-audit@a14`. The nearest
rival, a periodic run, has no evidence for a period.

### A whole-record repair beside the repair on touch

Shape. `design@agent-skills@existing-heads-on-touch` states both: a head a change touches is brought
to the rules in that change, and a project audit brings the whole record to them, at a migration or
at the owner's request, its widenings going to the owner on the owner list. The head's sentence that a sweep
repairs an approval cited as a ground, since that repair widens nothing, becomes a case of the
audit. The rejected alternative
"A sweep that brings every head stating the instance as its rule to its rule at once" leaves the
rejected alternatives, since it is now chosen.

Arguments: `argument@design-record-audit@a3`, `argument@design-record-audit@a23`. The rival is the
repair on touch alone, which never reaches a head no change touches, and leaves an adopting project's
record under the old rules after its migration.

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
- **Test 2 naming the `%%` line as a reason's home**, lost to
  `thread@design-record-audit@test-2-site-for-installed-text` by D3: it ships a convention of this
  repository.
- **The repair on touch alone**, lost to `thread@design-record-audit@whole-record-repair-at-a-migration`
  (`argument@design-record-audit@a23`).
- `thread@design-record-audit@test-3-reading-not-practice`, ruled out on the owner's argument
  (`argument@design-record-audit@a22`).

## Readings

The work reads no external specification.

## Premortem

Each cause was put to the owner in round 2 under its label; the owner, the reply to round 2: "Agreed
on all, tripwires and AC approved." Round 2 labelled them T6 to T8 and AC1; this spec labels the
acceptance criterion AC3, per "Status and audience".

- Round 2 also put a cause with no label: an audit commit that removes the owner's own words. It
  proposed no new tripwire, since `tripwire@agent-skills@owner-intent-stripped` already watches it.
  Approved with the rest.
- Round 4 put the cause of the new test 2 and proposed no new tripwire. T6 watches it in an audit.
  `tripwire@agent-skills@a-head-verdict-is-overruled` watches it in a review only, since it fires on a
  review's verdict.

| label | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| T6 | The calibration fails silently, and the owner list fills with defaults the owner reverses | `thread@design-record-audit@audit-method` | tripwire. Fires when, in one run, the owner rules against the default on more than a quarter of the listed items; the bound is the owner's to reset. Response: reopen the decision harvested from that thread. Re-entry: the session that receives the owner's answers to an owner list counts them, and the standing-state review of the audit's branch reads it again |
| T7 | A Migration entry cites the axis, and the pin moves without the axis run | `thread@design-record-audit@audit-triggers` | tripwire. Fires when a retrospective finds a pin moved across such a version with the axis not run. Response: reopen the decision harvested from that thread. Re-entry: the retrospective of a session that moved a pin. It watches agent behaviour in a consumer project, which `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` says this project cannot see; the harvested tripwire cites that issue |
| T8 | An edit makes a tripwire or an issue it never read stale | `thread@design-record-audit@design-record-axis` | tripwire. Fires when a review of an audit branch finds a tripwire guarding, or an issue naming, a head the branch changed, left false. Response: reopen the decision harvested from that thread. Re-entry: the standing-state review of every audit branch |
| AC3 | The method does not work at its first real run | `thread@design-record-audit@audit-method`, `thread@design-record-audit@design-record-axis` | acceptance criterion, `acceptance@design-record-audit@passes-converge` |

## Acceptance criteria

### Repeated passes of the axis on this repository converge, and what is left is small enough in the owner's judgement `##passes-converge`

AC3, put to the owner as AC1 in round 2. Guards `thread@design-record-audit@audit-method` and `thread@design-record-audit@design-record-axis`.
Judged at step 5. The instrument: each pass's re-check, fresh design-record auditors over the same
groups after the pass's edits, against the rules shipped by steps 1 to 4. Each violation it reports,
whether the drafts missed it or an edit made it, counts once the session's reading confirms it. A
pass's re-check is the next pass's drafts. The observable is the count of violations left, pass
after pass: it fires when a pass leaves as many as the pass before, or when the owner judges what is
left not small enough. There is no fixed bound, per D8. Response: reopen the thread the misses come
from.
Per D6, the run is this repository's own migration, an observation of real use under
`design@agent-skills@synthetic-evidence-not-built`; no finding of the re-check is acted on before a
reading confirms it.

## Implementation sequence

Steps 1 to 4 change the installed text: each commit runs `cargo klarch install-agent-skills` and
commits the installed copies with their source. Each passes the edit tests of
`path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", and cites no entry of this
repository outside a `%%` line. Every commit runs `cargo klarch check --staged` before it and
`cargo klarch commits origin/main..HEAD` after it.

1. **Test 2.** The primer's test 2, its `%%` line, the decision-recording skill's sentence, and the
   rejected alternative's clause, per "Entry test 2 for agent-facing text". Fails alone on: the
   wording of one test.
2. **The audit skill.** The skill of "The audit skill is an activity of its own", "The audit's
   method", "The design-record axis" and "The outcomes", named per D1, with its `%%` line for the
   `F` label; the primer's `primer@installed-skills`; the review skill's and the planning skill's
   scopes; the root README's list of activities. The commit converts the spec's `planned` citation
   of the skill to a `path` citation.
   Fails alone on: a skill that reads as a review.
3. **The design-record auditor.** The agent of "The design-record auditor"; the audit skill names
   it where it dispatches it. The commit converts the spec's `planned` citation of the agent. Fails alone on: an agent whose description cannot be dispatched from, or whose body edits.
4. **The triggers.** The setup skill's two sections, per "The triggers"; the CHANGELOG.md Migration
   entry citing the axis; `cargo x changelog`. Fails alone on: a pin move that reads no axis.
5. **The first run.** The axis runs on this repository under the skill, on this branch. Its owner
   list waits for the owner's answers, and nothing on it is applied before them; the branch does not
   merge before they come. Its re-check judges AC3. Fails alone on: a draft or an edit the method
   did not prevent.
6. **The harvest and the changelog.** The rows of "Harvest"; the CHANGELOG.md entries the three
   tests of `design@knowledge-architect@changelog-entries` owe for steps 1 to 4; the spec deleted.

## Order rationale

- 1 before 2: the skill's axis applies the test as step 1 leaves it.
- 2 before 3: the agent's description names the skill that dispatches it.
- 3 before 4: the Migration entry cites a section of the skill, and the run it asks for dispatches the
  agent.
- 4 before 5: the run is the migration the entry asks for, so it runs on the text that asks.
- 5 before 6: the harvest records the decisions after AC3 is judged.

## Defaults awaiting the owner

- **D1**, on `thread@design-record-audit@audit-is-an-activity`: the skill is named `project-audit`,
  since the planning skill already has a "design audit" of a slice or a spec. Ruled, the reply to the
  review message: "Agreed on these defaults".
- **D2**, on `thread@design-record-audit@audit-is-an-activity`:
  `goal@agent-skills@one-skill-per-activity` and `goal@knowledge-architect@agents-get-a-complete-workflow`
  list the workflow's activities, and auditing is not among them; the first also names "the review
  agents its skills dispatch", which the auditor is not. At the harvest, the session drafts
  the two goals with auditing added and puts the drafts to the owner, under
  `skill@knowledge-architect-goal-setting`. The default ruled, the reply to the review message; the
  drafts await the owner at the harvest.
- **D3**, on `thread@design-record-audit@test-2-site-for-installed-text`, from the code-claims
  review: the shipped test 2 says the reason of a decision stated by one text lives in that text,
  beside the instruction; in this repository that place is the `%%` line. Ruled, the reply to the
  `%%` message: "default approved".
- **D4** (R1 in the review message), on `thread@design-record-audit@whole-record-repair-at-a-migration`,
  from the design-conformance review: the reopening of the rejected sweep is recorded as a thread,
  the alternative leaves the rejected alternatives at the harvest, and
  `design@agent-skills@existing-heads-on-touch` states both repairs. Ruled, the reply to the review
  message.
- **D5** (R2), on `thread@design-record-audit@audit-outcomes`, from the design-conformance review: a
  conflict with a goal goes on the owner list and is handled under the goal-setting skill. Ruled, the
  reply to the review message.
- **D6** (R3), on `acceptance@design-record-audit@passes-converge`, from the
  design-conformance review: AC3 is kept, read as an observation of real use, since step 5 is this
  repository's own migration; the re-check's findings are confirmed by reading before anything is
  done with them. Ruled, the reply to the review message.
- **D7**, on `criterion@design-record-audit@c8`, from the code-claims and cold-implementer reviews: the
  owner accepted C8 on a cost of "about ten" subagents for 189 heads. The corpus holds 206 heads and
  about 100 rejected alternatives, and a re-check bounded by C7 takes one auditor per group, so a run
  costs about 18 subagents. Default: keep the per-group re-check, since one auditor over the whole
  corpus breaks C7 on a large project. D7 corrected the premise of a ruling, so
  `primer@owner-word-premise` applied rather than the status rule on the owner's absence. Ruled,
  the owner's message after step 1: "D7 default approved".
- **D8**, on `acceptance@design-record-audit@passes-converge`, from the first run's re-check, which
  found violations left in 8 of its 9 groups: AC3 as first written fired on one violation left. The
  owner, before the ruling: "IMO, it is not too bad that a run of the audit and repair leaves (or
  recreate) some violations. We'll try to improve by iterating, but I believe the important point
  is that it should converge and leave no more findings after a few passes." The session's default
  was a bound of 3 passes. Ruled against the default: "About AC3: I would not put such a rigid
  criterion. As long as we observe a "convergence", and the amount of leftover violations is small
  enough, I think that is good. If you ask an LLM to find a flaw or defect in something, it will
  nearly always find one IMO. Especialy if that something is prose and not code."
- **D9**, on `thread@design-record-audit@audit-method`, from the first run's re-check: a head that a
  split, a rename or a merge creates is drafted as an entry of its own and judged against every rule
  before its commit, since about a dozen of the violations left were in heads the run created; and
  a quotation of the owner is checked against the question it answered, not only against its words,
  since the run quoted a verbatim answer to another question. Ruled: "D9: agreed on these
  refinements."

## Harvest

Every head lands in `path@agent-skills@docs/design.md`, unless the row says otherwise.

| item | where it lands |
| --- | --- |
| `thread@design-record-audit@audit-is-an-activity` | a head |
| `thread@design-record-audit@audit-method` | a head |
| `thread@design-record-audit@design-record-axis` | a head |
| `thread@design-record-audit@audit-outcomes` | a head |
| `thread@design-record-audit@audit-triggers` | a head; `design@knowledge-architect@changelog-entries`, in `path@knowledge-architect@docs/design.md`, gains the convention |
| `thread@design-record-audit@audit-agent-per-axis` | a head |
| `thread@design-record-audit@test-2-site-for-installed-text` | `design@agent-skills@a-head-is-owed-by-an-entry-test` rewritten |
| `thread@design-record-audit@whole-record-repair-at-a-migration` | `design@agent-skills@existing-heads-on-touch` rewritten; the rejected sweep leaves `path@agent-skills@docs/rejected-alternatives.md` |
| `thread@design-record-audit@test-3-reading-not-practice` | `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` closed on the owner's ruling |
| the `F` prefix | `design@agent-skills@ruled-items-labelled`, its table |
| T6, T7, T8 | `path@agent-skills@docs/tripwires.md`, each naming its head, with its label; T7 cites `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` |
| AC3 | reported in the landing commit |
| every item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined` | closed |
| `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` | retitled and rewritten to the axes it still holds, since the workflow then offers a project audit |
| D2 | the two goals, on the owner's word |

## Later consequences

- The other axes, the agentic-workflow axis first among them, each add an agent and a section of the
  audit skill, and adjust the method where they need to.
- An axis whose fixes are code may need a report document, which the issue keeps.
- Adopting projects meet the axis at their next pin move, through the Migration entry.
