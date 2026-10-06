# The issues and tripwires a piece of work bears on are searched by subagents before the work, and every deferred trigger is read at the review

## Status and audience

This spec is the plan of the work that changes where the workflow reads its standing entries. A new
installed agent searches every issue and every tripwire for the entries a piece of work bears on,
at the grounding of a design discussion and at the design audit of a milestone step. The session
then reads each entry it returns, whole. The standing-state reviewer also reads every `deferred`
issue's trigger before every merge, as it reads every tripwire. The spec is written for a session
that did not witness the design discussion that produced it. It leaves the repository in the
commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **No gate fixes when this spec lands against its work.** The work changes installed text, and
  only `check` on the working tree compares the installed copies with the shipped text: `commits`
  does not, per `design@core@owned-namespace-check`. So the commit that adds this spec passes under
  the work's tip checker, and `design@agent-skills@plan-lands-before-gate-change` does not apply.
  The work continues on this spec's branch, in one pull request, after the reviews of this spec,
  as the owner ruled at the audit, "Agreed on the 4 defaults, you can proceed."
- It is a spec, so the places `knowledge-architect-planning` §7 gives a milestone document are this
  spec's own sections: the defaults an audit adds go under "Defaults awaiting the owner", and a
  load-bearing gap the audit leaves open is recorded under "Threads", as a thread in state `open
  at the audit`, with the discriminating fact.
- **How §7 applies to this spec**, whose steps are commits of one branch:
  - point 1 and point 2, the grounding and the design audit, run once, before step 1, and read
    every step's entry. The owner rules on every default of "Defaults awaiting the owner" at the
    audit, and no step that depends on a default starts before its ruling;
  - point 3 applies to each step that changes files, as its entry says. Installed text has no
    claims in the project's development procedure, so each entry states its own claim and check;
  - point 4, the review before the merge, runs once, after step 5, per
    `knowledge-architect-review`, with the axes that skill gives for the work;
  - point 5, the report on the acceptance criterion, goes in the message of step 6's commit;
  - points 6 and 7, the harvest, its review and the deletion of this spec, are step 6.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/5be6d68e-c5d9-4cd9-83d4-50f27935c356.jsonl`.
  The discussion begins at the owner's message that opens "I'd like to discuss the", and ends at
  the owner's message that holds "You can proceed with the spec." It holds 4 owner messages,
  called rounds 1 to 4 below, and 3 agent replies, each after rounds 1, 2 and 3.

## How a step is worked

Per `knowledge-architect-planning`, §7.

## Names

- **a standing entry**: an issue entry or a tripwire entry, of any anchor. `{{command}}` below is
  the install placeholder for the project's command; in this repository it is `cargo klarch`.
- **a deferred trigger**: the `### Trigger` subsection of an issue entry of kind `deferred`, which
  takes the place of `### What would close it`, per the installed issue-tracking skill,
  `path@agent-skills@content/skills/issue-tracking/SKILL.md`.
- **the search**: the dispatch of one or more search agents for one piece of work, and the
  session's reading of what they return.
- **a search agent**: one dispatch of the new installed agent, named under New names.
- **the listings**: the output of `{{command}} issues` and `{{command}} tripwires`, one row per
  entry after a header row. The issue rows carry `kind`, `anchor`, `id`, `title` and `last change`;
  the tripwire rows carry `anchor`, `id`, `title` and `guarding`.
- **a group**: the slice of the listings' rows one search agent is given.
- **N**: the largest number of rows in a group.
- **a seed**: a decision or a goal the work names, as a reference. The seeds are the input of the
  first phase of a search agent's reading.
- **the dispatcher**: the session that sends the search agents and reads what they return.
- **the standing-state reviewer**: the installed agent `knowledge-architect-standing-state-reviewer`,
  `path@agent-skills@content/agents/standing-state-reviewer.md`.
- **thaum**: a project that uses knowledge-architect, on the owner's machine at
  `path@elsewhere@~/Documents/code/thaum`. The evidence for this work came from its sessions.
- **W3**: finding W3 of thaum's retrospective of its move to knowledge-architect 0.3.0, which
  raised the question this spec answers. It is the section "W3. Missing: a pin move does not list
  the deferred issues and tripwires whose occasion is the pin move" of
  `path@elsewhere@~/Documents/retrospectives/2026-10-04-thaum-workflow.md`. The move is thaum's
  commit 6e0da6e, "Move the checker's pin to 0.3.0".

## What the work is

**The question.** An issue entry of the agent-skills Component recorded it until this spec, which
schedules the answer, closed it in the commit that added the spec. Its content moved here.

In thaum, the move of the pin to knowledge-architect 0.3.0 was work that was neither designed nor
planned. A `deferred` issue's trigger was "The next move of the pinned knowledge-architect
version". The session did the whole move without reading it. The standing-state reviewer found it
before the merge, though its procedure does not ask it to read such an issue, and the owner ruled
on it then. The retrospective proposed a step specific to the pin move. The owner dropped that,
since it would mean anticipating every trigger condition for every kind of action. The owner's
position, recorded in the issue: the search "is supposed to be either done in grounding steps of
design work, or by the standing state reviewer", and "maybe a standing state reviewer (or similar)
should be dispatched earlier in the workflow, either after plan writing, or during grounding steps,
with the task of 'finding every issue or tripwire related to the work we are about to plan or
implement'". On design grounding: "design grounding can still benefit from the subagent focused on
searching the open issue and tripwires related to the work, because the search itself is expensive
work for the main session. This can work well if the design skill is instructed to find and read
the full reported issues and tripwires, rather than relying on the subagent summary."

**What exists today at each site:**

- **The grounding of a design discussion**, loop step 1 of
  `path@agent-skills@content/skills/design/SKILL.md`, reads in the main session "the open issues
  and the tripwires, with `{{command}} issues` and `{{command}} tripwires`; and, for each entry the
  question bears on, `{{command}} show <kind>@<anchor>@<id>`".
- **Working a step**, §7 of `path@agent-skills@content/skills/planning/SKILL.md`. Its point 1
  grounds in the Component's "open issues, its tripwires". Its point 2, the design audit, lists as
  a gap "a standing entry the step's planned code would fire: a tripwire whose firing condition, or
  a `deferred` issue whose trigger, the planned code meets. `{{command}} tripwires` and
  `{{command}} issues --kind deferred` list them; read each against the step." Both reads are in
  the main session. An issue of another kind that the step's code touches is outside point 2's
  read.
- **The reviews of a plan document**, §8 of the same skill, send three fresh reviewers. The cold implementer grounds
  in "the open issues and the tripwires of that Component", the Component the document is about.
  None of them reads the standing entries of every anchor against the document.
- **The standing-state reviewer** reads every tripwire of every tripwires home, in its section
  "2. Re-read every tripwire". Its description, in its frontmatter, says it re-reads "every issue
  entry the change opens or closes". Its section 1 runs `{{command}} issues` as a mechanical
  check, and its section "4. The predicate" also reads an issue left open beside a plan document
  the diff adds. No section reads the deferred triggers against the change.
- **The review skill**, `path@agent-skills@content/skills/review/SKILL.md`, §1, sends the
  conformance axis, the standing-state reviewer, "before every merge to the main branch, since it
  is the standing re-entry point of every tripwire".
- **The issue-tracking skill**, under its tripwire entry section: "One standing re-entry point:
  `knowledge-architect-standing-state-reviewer` reads every tripwire home again".
- **The installed agents** are the files under `path@agent-skills@content/agents/`. The build
  lists every file there, and the install writes each to `path@agent-config@agents/` with the prefix
  `knowledge-architect-`, per `design@agent-skills@content-mirrors-the-install-layout`. The
  opening of the review skill names the six installed agents, as reviewers.

**Measured.** The discussion measured this repository on a main branch that has moved since. The
figures below were re-taken on this spec's branch, and in thaum at its commit ef21314:

| what | figure | command |
| --- | --- | --- |
| entries in this repository | 36 issues and 22 tripwires; 7 of the issues are `deferred` | the rows of `cargo klarch issues`, `cargo klarch tripwires` and `cargo klarch issues --kind deferred`, the header row excluded |
| bytes in this repository | 75,280 in the issue entry files, 21,169 in the tripwires homes | the two commands below |
| entries in thaum | 90 issue entry files and 107 tripwire headings; the owner counted 91 and 108 in round 2 | the first command below with `wc -l` in place of `xargs cat \| wc -c`; the second with `xargs grep -hE` on the pattern of a level-two heading ending with a slug, then `wc -l` |
| bytes in thaum | 209,294 in the issue entry files, 109,653 in the tripwires homes, 318,947 in total | the two commands below, run in thaum |
| thaum, per anchor | the Component crates/thaum-engine holds 165,575 bytes in 46 files, 51.9% of the total; the location `rules`, at docs/rules, holds 50,044 bytes in 9 files; every other anchor holds less | the bytes of each file of the two commands, summed by the anchor that holds the file |

```sh
git ls-files '*open-issues/*.md' '.claude/open-issues/*.md' | grep -v -e README.md -e index.md -e tests/ | xargs cat | wc -c
git ls-files '*tripwires.md' '*docs/tripwires/*.md' | grep -v tests/ | xargs cat | wc -c
```

The token figures assume 4 bytes per token. That ratio is an assumption, not a measurement: about
24k tokens for this repository, about 80k for thaum.

**Outside the work:**

- **A search before undesigned work.** No installed skill runs at the start of every piece of
  work that is neither designed nor planned; two run at the start of some of it, the
  agent-configuration skill before an edit of the configuration and the issue-tracking skill
  before a diagnosis. The owner parked it, with its re-entry at the design of
  `issue@agent-skills@a-skill-for-bounded-problems`; see #search-before-undesigned-work.
- **A search at convergence or at the reviews of a plan document.** Parked and ruled out; see
  #search-at-convergence and #search-at-plan-review.
- **A checker command that lists the standing entries citing any of several entries.** The owner
  ruled it worth an issue and left it for later design:
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`.
- **The primer's check before diagnosing**, which runs `issues`, `tripwires` and `show` for one
  observed problem. It is a lookup by the session, not a search for a piece of work, and the
  discussion did not touch it.
- **This repository's own root CLAUDE.md**, whose section "Verify before relying on anything" asks
  to ground in a Component's open issues and tripwires before touching its code. It is the
  project's directive, not installed text, and the discussion did not touch it.

## What is already decided

The design rests on these decisions, and does not argue them again:

- `design@agent-skills@additions-need-real-use` and `design@agent-skills@capability-over-conformance`:
  each addition is judged in `thread@standing-entry-search@deferred-triggers-at-review`,
  `thread@standing-entry-search@search-at-design-grounding` and
  `thread@standing-entry-search@search-at-step-audit` against them.
- `design@agent-skills@primer-limit`: the reason #search-before-undesigned-work is parked.
- `design@agent-skills@shipped-text-is-reference-free`: the new agent text names no path of this
  repository, holds no live reference, and writes the command as `{{command}}`.
- `design@agent-skills@content-mirrors-the-install-layout`: the new agent file is listed by the
  build.
- `design@core@owned-namespace-check`: `check` on the working tree compares the installed copies
  with the shipped text, and `commits` does not. The rule that each commit changing installed text
  installs it in the same commit is `path@agent-skills@CLAUDE.md`'s.

The work rewrites one decision, `design@agent-skills@conformance-before-every-merge`, to say that
the standing-state reviewer reads every deferred trigger as well as every tripwire. On the main branch
before this spec, `{{command}} show` listed two texts referencing it; the texts that restate it
are found by reading:

| text | what it says | judged or updated at |
| --- | --- | --- |
| `path@agent-skills@docs/open-issues/a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see.md` | cites the head as making the standing-state review the re-entry point of every tripwires home | the harvest, step 6: it stays true, and is read again |
| the issue this spec closed | cited the head under `Why it matters` | deleted in the commit that adds this spec |
| the standing-state reviewer: its description, its opening scope text ("This axis is the standing re-entry point for every tripwire home."), its section 2 and its section "Reporting" | restates the head for tripwires | step 3 |
| the review skill, §1, the conformance row | "the standing re-entry point of every tripwire" | step 3 |
| the issue-tracking skill, its tripwire entry section | "reads every tripwire home again" | step 3 |

## Criteria

### Every tripwire and every deferred trigger is read against every piece of work before it merges, designed or not `##every-entry-has-a-reader`

Binding. Derived from `goal@knowledge-architect@agents-work-without-drift`: "a departure from the
recorded design is caught by a check or a review before it merges". Met by
`thread@standing-entry-search@deferred-triggers-at-review`.

### The review keeps reading every entry, and an early search adds to that read without replacing it `##full-read-stays`

Binding, as a presumption. Derived from `design@agent-skills@conformance-before-every-merge`: "the
reviewer reads every entry of every home, not the subset the diff seems to concern". An early
search filters by relevance, so it can only add to that read. Met: no thread replaces the review's
read.

### As little main-session context as possible goes to the search `##main-context-cost`

Weighed. Derived from `goal@knowledge-architect@agents-get-a-complete-workflow`, which asks for
efficient work. Met by `thread@standing-entry-search@search-at-design-grounding`,
`thread@standing-entry-search@search-at-step-audit` and
`thread@standing-entry-search@search-partition-unit`.

### Every entry the work bears on is read whole by the session that acts on it, not as a summary `##entries-read-whole`

Binding. Derived from the owner's clause in the issue, "find and read the full reported issues and
tripwires, rather than relying on the subagent summary", and from
`goal@knowledge-architect@the-owner-decides`. Met by
`thread@standing-entry-search@search-return-shape`.

### Each addition traces to evidence from real use `##real-use-origin`

Binding, as a presumption. Derived from `design@agent-skills@additions-need-real-use`. Met: W3 for
`thread@standing-entry-search@deferred-triggers-at-review`; the owner's observation of thaum's
reviews, round 2, for `thread@standing-entry-search@search-at-design-grounding` and
`thread@standing-entry-search@search-at-step-audit`.

### A tool and the judgement to use it, not a scripted step for one occasion `##capability-form`

Binding, as a presumption. Derived from `design@agent-skills@capability-over-conformance`, and
from the owner's drop of the step specific to the pin move. Met by
`thread@standing-entry-search@standing-entry-search-agent`.

### The agent text names no project's paths `##no-project-paths`

Binding. Derived from `goal@agent-skills@installed-text-works-anywhere`. Met in the design: the
agent writes the command as `{{command}}`. The review of the work judges the text.

## Threads

### The standing-state reviewer reads every deferred trigger against the change, as it reads every tripwire `##deferred-triggers-at-review`

Proposed by the agent, round 1. Approved. Arguments: `argument@standing-entry-search@a1`,
`argument@standing-entry-search@a2`, `argument@standing-entry-search@a5`,
`argument@standing-entry-search@a6`, `argument@standing-entry-search@a8`. Shape: Decided design,
"The standing-state reviewer". Harvest: rewrites `design@agent-skills@conformance-before-every-merge`.
The owner's words, round 2: "deferred-triggers-at-review: approved."

### A search before the first commit of any branch, written in the primer `##search-before-undesigned-work`

Proposed in the issue, as the question of which step reads a deferred trigger for undesigned work;
argued by the agent, round 1. Parked. Argument: `argument@standing-entry-search@a7`. Tripwire: a
second instance of a deferred trigger met by undesigned work and found only at the review.
Re-entry: the design of `issue@agent-skills@a-skill-for-bounded-problems`. Harvest: a tripwire in
`path@agent-skills@docs/tripwires.md`. The owner's words, round 2: "search-before-undesigned-work:
agreed to park."

### The search is an installed agent of its own `##standing-entry-search-agent`

Proposed by the agent, round 1, against `thread@standing-entry-search@standing-state-second-mode`.
Approved. Arguments: `argument@standing-entry-search@a9`, `argument@standing-entry-search@a10`,
`argument@standing-entry-search@a11`. Shape: Decided design, "The search agent". Harvest: a new
head, slug standing-entry-search-agent. The owner's words, round 2:
"standing-entry-search-agent: agreed, I don't think using the same agent for both task would be
good here."

### The standing-state reviewer, with a second mode for a question or a plan document `##standing-state-second-mode`

Proposed by the owner, in the issue. Withdrawn by the owner, round 2, on the defeating reason of
`argument@standing-entry-search@a10`: one agent text would hold two reading standards. The owner's
words, round 2: "I don't think using the same agent for both task would be good here." Harvest:
the rejected alternatives, if the recording tests admit it.

### The search returns references, a reason for each, the entries judged unrelated, and leans to recall `##search-return-shape`

Proposed by the agent, round 1. Approved. Argument: `argument@standing-entry-search@a12`. Shape:
Decided design, "What a search agent returns". Harvest: inside
the head standing-entry-search-agent. The owner's words, round 2:
"search-return-shape: approved."

### A search agent first runs `show` on every seed, then reads the remaining entries by meaning `##search-seeds-from-references`

Proposed by the agent, round 1. Approved. Arguments: `argument@standing-entry-search@a3`,
`argument@standing-entry-search@a13`, `argument@standing-entry-search@a14`,
`argument@standing-entry-search@a15`. The premortem converted cause 4 into a clause of it: the
brief names the seeds when they are known. Shape: Decided design, "How a search agent reads".
Harvest: inside the head standing-entry-search-agent; its sub-question is
`issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`. The owner's words,
round 2: "search-seeds-from-references: agreed. For the subquestion: I think this is worth an
issue, it would make the task much easier. Left for later design."

### The search runs at the grounding of a design discussion, and the session reads each returned entry whole `##search-at-design-grounding`

Proposed by the owner, in the issue. Approved. Arguments: `argument@standing-entry-search@a16`,
`argument@standing-entry-search@a20`, `argument@standing-entry-search@a21`,
`argument@standing-entry-search@a22`. Shape: Decided design, "Where the search runs". Harvest: a
new head, slug standing-entries-searched-before-the-work. The owner's words,
round 2: "search-at-design-grounding: approved." and "the task should at least run at design
grounding and step audits, which are the main moments when design gets discussed, and nothing
should be missed."

### The search runs a fourth time, as a reviewer of a plan document `##search-at-plan-review`

Proposed by the owner, in the issue ("either after plan writing, or during grounding steps").
Ruled out: it lost to `thread@standing-entry-search@search-at-convergence` on the in-change path.
Arguments: `argument@standing-entry-search@a17`, `argument@standing-entry-search@a18`. The owner's
words, round 2: "I agree with you that search-at-convergence looks better than
search-at-plan-review." The agent read that as a ruling out, marked the thread `presumed-settled`
in the replies to rounds 2 and 3, and showed it in the checkpoint table of the reply to round 3.
The owner's word against that table, round 4: "You can proceed with the spec." Harvest: the
rejected alternatives, if the recording tests admit it.

### The search runs again when convergence is proposed, before the premortem `##search-at-convergence`

Proposed by the agent, round 1. Parked. Arguments: `argument@standing-entry-search@a18`,
`argument@standing-entry-search@a21`, `argument@standing-entry-search@a29`,
`argument@standing-entry-search@a30`, `argument@standing-entry-search@a32`. Tripwire, as widened at
the premortem: across sessions, 2 standing-state reviews report a standing entry the work bears on,
which no search before the work returned. The count 2 was the agent's default, which it named the
owner's weighing (`argument@standing-entry-search@a29`); the owner ruled at the audit, "Agreed on the 4 defaults, you can proceed." Re-entry: the design discussion of
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`. Harvest: a
tripwire in `path@agent-skills@docs/tripwires.md`, guarding
the head standing-entries-searched-before-the-work. The owner's words: round 2,
"Running it at convergence or plan review could be parked with a tripwire if I continue to observe
the problem too often, IMO."; round 3, "you do not need to do anything to guarantee that evidence
reaches it and it gets revisited. This waits for the session that discusses solving the issue
a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see"; round 4, "I'd keep only the
widened search one which we already discussed." and, against the checkpoint table, "You can
proceed with the spec."

### The search runs at the design audit of a milestone step, in place of the main session's read `##search-at-step-audit`

Proposed by the agent, round 1. Approved. Arguments: `argument@standing-entry-search@a19`,
`argument@standing-entry-search@a21`, `argument@standing-entry-search@a22`. Shape: Decided design,
"Where the search runs". Harvest: inside
the head standing-entries-searched-before-the-work. The owner's words, round 2:
"search-at-step-audit approved."

### The search at the step audit reads every issue kind and every tripwire `##search-covers-every-kind`

Proposed by the agent, in the reply to round 2. Approved. Arguments:
`argument@standing-entry-search@a23`, `argument@standing-entry-search@a24`. Shape: Decided design,
"Where the search runs". Harvest: inside
the head standing-entries-searched-before-the-work. The owner's words, round 3:
"search-covers-every-kind: approved, this would probably be better."

### Every entry is read whole, by several search agents `##search-reading-depth`

Proposed by the agent, in the reply to round 2, as shape B of three. Approved. Arguments:
`argument@standing-entry-search@a25`, `argument@standing-entry-search@a26`,
`argument@standing-entry-search@a27`. Shape: Decided design, "How a search agent reads". Harvest:
inside the head standing-entry-search-agent; shapes A and C to the rejected
alternatives, if the recording tests admit them. The owner's words, round 3: "search-reading-depth:
I agree on shape B. You are right that even for a subagent, this might quickly become too large
for a single one to judge reliably."

### The listings' rows are cut into groups of at most N entries, ordered by anchor `##search-partition-unit`

Proposed by the agent, in the reply to round 3. Approved; N = 60, a default until the owner ruled at the audit, "Agreed on the 4 defaults, you can proceed."
Argument: `argument@standing-entry-search@a28`. Shape: Decided design, "How the dispatcher cuts
the groups". Harvest: inside the head standing-entry-search-agent. The owner's word
against the checkpoint table of the reply to round 3, which showed the thread as "groups of at
most N entries, ordered by anchor; N = 60, a default awaiting your word", round 4: "You can
proceed with the spec."

## Arguments

### The mechanism: a standing entry reaches work only through a reader who holds both, and no reader exists for a deferred trigger met by undesigned work `##a1`

Round 1, the agent. Bears on #deferred-triggers-at-review and #every-entry-has-a-reader. "A
standing entry reaches a piece of work only when one reader holds both the entry and the work."
The workflow schedules that reader "before the work, inside the design and planning skills; after
the work, at the review, and there for tripwires only; so a `deferred` trigger met by undesigned
work has no reader at all."

### The trigger test assumes the session meets the entry, and nothing delivers it `##a2`

Round 1, the agent. Bears on #deferred-triggers-at-review. "It assumes that the session doing that
work meets the entry. Nothing delivers the entry to that session. The thaum trigger, 'The next
move of the pinned knowledge-architect version', passes the test, and the session still never
read it."

### The record holds no rival, and one goal asks the tool to compute the list `##a3`

Round 1, the agent. Bears on #search-seeds-from-references. "No rejected alternative covers this
question." "No tripwire guards `design@agent-skills@conformance-before-every-merge`." The goal
`goal@knowledge-architect@structure-and-workflow-work-together` says: "The tool computes the
workflow's work lists: what a reversed decision touches, and which issues and tripwires a change
must re-read."

### No prior-art survey applies `##a4`

Round 1, the agent. Bears on no thread. "The question depends on this workflow's own registers and
skills. No outside project has shipped a mechanism for it."

### A trigger and a tripwire answer the same question, and nothing argues why the reviewer treats them apart `##a5`

Round 1, the agent. Bears on #deferred-triggers-at-review. "Issue-tracking already says that a
trigger and a tripwire's firing evidence 'answer the same question' and pass one test. No recorded
decision argues why the reviewer treats them differently." "This puts the read where you placed
it." "It satisfies #every-entry-has-a-reader for every kind of work, because the review runs
before every merge."

### The cost of the late read is small for undesigned work `##a6`

Round 1, the agent. Bears on #deferred-triggers-at-review. "7 more entries per review here, read
in a subagent, so no main-session context." "One widened design head." "A trigger met this late
costs rework instead of a ruling. For undesigned work, which is usually small, that rework is
small too."

### An early search for undesigned work has no host but the primer `##a7`

Round 1, the agent. Bears on #search-before-undesigned-work. "no installed skill runs at the start
of undesigned work. The bounded-problem skill does not exist. So the line would go in the primer."
Against it: "`design@agent-skills@primer-limit` keeps the primer for what every session needs." "The evidence is one
instance." "A line saying 'always dispatch before work' is a conformance rule, against
#capability-form." "park it. Re-enter when `a-skill-for-bounded-problems` is designed: that skill
is the natural host."

### A trigger that no diff shows already fails the trigger test `##a8`

The reply to round 3, the agent, premortem cause 6. Bears on #deferred-triggers-at-review. "A
`deferred` trigger names an event that no diff shows, for example 'a second consumer appears'."
"It survives: the trigger test already requires that a trigger names a change that includes the
work. A trigger outside a diff fails that test. The reviewer reports it under the predicate it
already applies."

### The reviewer and the search differ in input, output, standard and framing `##a9`

Round 1, the agent. Bears on #standing-entry-search-agent and #standing-state-second-mode.
"Input: it takes a diff. The search takes a question or a plan." "Output: it returns findings. The
search returns a list." "Standard: it reads every entry by rule. The search filters by relevance on
purpose." "Framing: its text says 'You are one axis of a review' and 'do not review outside this
axis'." "A dual-mode agent needs one description that serves two dispatchers. The harness picks
agents by that description."

### One agent text would hold two reading standards `##a10`

Round 1, the agent. Bears on #standing-state-second-mode. For it: "one agent text, and the
knowledge of where the homes live stays in one place." Against it: "that knowledge is two
commands, `issues` and `tripwires`. Mixing a relevance filter into the agent whose whole rule is
'do not decide in advance which entries were worth reading' puts the two standards side by side in
one text. A session could then apply the wrong one." The reviewer's own words are "What is not
available is deciding in advance which entries were worth reading."

### The owner judges one agent for both tasks a poor fit `##a11`

Round 2, the owner. Bears on #standing-entry-search-agent. "I don't think using the same agent for
both task would be good here."

### References make a whole read mechanical, and recall is cheaper than a miss `##a12`

Round 1, the agent. Bears on #search-return-shape. "Each item is a checked reference, so the
session reads it whole with `{{command}} show <ref>`." "The search is biased to recall. A false
positive costs the main session one `show`. A miss costs rework." "The 'judged unrelated' list
exists for the reason the reviewer names the tripwires that did not fire: without it, a silent
report looks the same as a skipped one."

### The reference graph has full recall for entries that cite the work's decisions `##a13`

Round 1, the agent. Bears on #search-seeds-from-references. "The agent first runs `show` on every
decision and goal that the work names. That lists every entry citing them, mechanically." "The
judgement by meaning is left for entries such as the thaum trigger, which cite no decision."

### A command for the union of citing entries is worth an issue `##a14`

Round 2, the owner. Bears on #search-seeds-from-references. "I think this is worth an issue, it
would make the task much easier. Left for later design."

### The seed is empty for a new question, and the brief names the seeds when they are known `##a15`

The reply to round 3, the agent, premortem cause 4. Bears on #search-seeds-from-references. "A new
question names no decision yet, so the seeding finds nothing, and the search rests on meaning
alone." "This is not a failure, so I convert it into a clause of the design. The brief names the
decisions the work touches when they are known: at the step audit, the spec's 'What is already
decided' section; at grounding, the entries already found."

### The discussion still holds each entry whole when the session reads what the agent returns `##a16`

Round 1, the agent. Bears on #search-at-design-grounding. "The search runs in the subagent." "The
session runs `show` on each returned entry, so the discussion still holds each one whole." The
agent's earlier objection, that the discussion needs the entries in its own context, recorded in
the issue, "is answered by #entries-read-whole."

### A plan-document reviewer adds a read after grounding, on the full path only `##a17`

Round 1, the agent. Bears on #search-at-plan-review. "What it adds: entries that bear on shapes the
discussion added after grounding." "Its gap: §8 runs only on the full path. A discussion on the
in-change path writes no plan document, so it never gets this read."

### A search at convergence covers both paths, at the step that produces tripwires `##a18`

Round 1, the agent. Bears on #search-at-convergence and #search-at-plan-review. "A second dispatch
when convergence is proposed, before the premortem, against the converged design." "the same
widening, on both paths." "at the step that produces tripwires, so the premortem sees existing
entries next to new causes." "What decides between the two: the in-change path."

### The step audit reads the same bytes in the main session at every step `##a19`

Round 1, the agent. Bears on #search-at-step-audit. "§7 does this read in the main session today,
once per milestone step, on the same 88 KB." The figure is
imprecise: point 2 reads the tripwires and the deferred issues, and point 1 one Component's issues
and tripwires, so no read of §7 covers every entry.

### Two of the occasions had no real-use origin in round 1 `##a20`

Round 1, the agent. Bears on #real-use-origin. W3 for #deferred-triggers-at-review; the owner's
request for #search-at-design-grounding; "your mid-session request, but worded with 'maybe'" for
#search-at-plan-review; "no real use yet" for #search-at-convergence and #search-at-step-audit.
"So #search-at-convergence and #search-at-step-audit need your word as the origin, or they wait."

### The owner's evidence: thaum's reviews find, after the work, entries that bear on its design `##a21`

Round 2, the owner. Bears on #real-use-origin, #search-at-design-grounding, #search-at-step-audit,
#search-at-convergence and #search-at-plan-review. "the standing state reviewer is too often
finding issues that are impacted by the changes or triggered tripwires that bear on the design
'after the work', especially on large work that is part of big milestones in thaum, which leads to
sometimes important repairs and decisions being needed, when those could have been taken into
account before the design and implementation." "I want to reverse the order a little bit." "the
task should at least run at design grounding and step audits, which are the main moments when
design gets discussed, and nothing should be missed." "Running it at convergence or plan review
could be parked with a tripwire if I continue to observe the problem too often, IMO." On the
scale: "thaum, which relies on it, is around 4 times bigger in code and 2 in markdown docs, and it
is expected to grow much more, so it will probably carry many more entries. (it has 91 issues and
108 tripwires right now)."

### The owner's evidence meets the rule for additions, and its mechanism `##a22`

The reply to round 2, the agent. Bears on #real-use-origin. "Your account meets
`design@agent-skills@additions-need-real-use` for grounding and for the step audit." The mechanism: "before the
work, the only reads of standing entries happen in the main session, and at the audit they cover
only two of the issue kinds, so the full read happens only at the review, after the code exists."
"The second half of that sentence is an inference. I have not checked it against thaum's history."
The audit's point 2 covers two kinds; its point 1 reads every issue of the step's Component, of
every kind, but of that Component only.

### The owner's late findings include issue kinds the audit does not read `##a23`

The reply to round 2, the agent. Bears on #search-covers-every-kind. "You say the late findings in
thaum include 'issues that are impacted by the changes', not only fired tripwires. §7 today reads
only tripwires and `deferred` issues." "A defect, a todo or a question that the step's code
touches is outside that read." "Moving §7's read to the agent keeps that gap." The rival, keeping
§7 at its two kinds, "leaves the late findings you describe in place." "§7 today reads only tripwires and `deferred`
issues" holds for point 2; point 1 reads the step's own Component's issues of every kind.

### The owner judges the wider read better `##a24`

Round 3, the owner. Bears on #search-covers-every-kind. "approved, this would probably be better."

### Three reading depths, and the agent reads the owner's "nothing should be missed" as ruling out reading by title `##a25`

The reply to round 2, the agent. Bears on #search-reading-depth. Shape A, one agent reading every
entry whole: "maximal, while the context holds", "about 80k tokens in thaum today, and it grows
linearly". Shape B, several agents reading every entry whole: "maximal", "bounded by the group
size", "N dispatches; the main session merges N lists of refs". Shape C, one agent reading firing
clauses and triggers whole and other entries by title: "depends on titles", "misses are possible".
"You said 'nothing should be missed', which rules out C as the default." "A cannot hold as thaum
grows, and you said it will grow much more."

### One agent cannot judge so many entries reliably `##a26`

Round 3, the owner. Bears on #search-reading-depth. "You are right that even for a subagent, this
might quickly become too large for a single one to judge reliably."

### A replay over thaum's history would discriminate the shapes and set the group size `##a27`

The reply to round 2, the agent. Bears on #search-reading-depth and #search-partition-unit. The
experiment: find the milestone steps where a standing-state review reported an entry the work bore
on, check out the commit of the step's audit, dispatch the draft agent with the step's spec, and
check whether it returns the entry. The pre-commitments: "If shape A returns every known late
entry, I withdraw B"; "If A misses entries that B finds, B stands, and the miss rate sets the group
size."; "If an entry was missed because §7 does not read its kind, that confirms
#search-covers-every-kind."; "If both shapes miss an entry, the search alone does not fix the late
findings, and the discussion has to look at the reading procedure, not only the occasions." It was
not run. The owner approved shape B in round 3, and the replay became
`acceptance@standing-entry-search@replay-returns-late-entries`.

### One anchor holds half of thaum's standing entries, so the unit of the split is the entry `##a28`

The reply to round 3, the agent. Bears on #search-partition-unit. "One anchor holds 52% of the 319
KB. An agent given that anchor whole reads about 41k tokens, and that anchor grows the most. So
the unit of the split cannot be the anchor." The unit "is the entry": the rows of the listings,
ordered by anchor, "so entries that cite each other stay together", cut "into groups of at most N
entries". "Default N = 60": the mean size of an entry in thaum is 1.6 KB, 319 KB over the owner's
199 entries, so 60 entries are about 96 KB, about 24k tokens. "N has no measurement behind it. It
is a default awaiting your word." The rival, "a cap in bytes instead of entries", lost: "The
listing gives no sizes, so the dispatcher would need one more command per entry." The transcript
says "199 files", and "51" files for the 13 smaller directories, grouped by directory rather than
by anchor. The measurement in thaum at ef21314 gives 197 entries in 102 files, and 55 files in
those 13 directories; 318,947 bytes over 197 entries is still 1.6 KB, and 4 agents at N = 60.

### "Too often" is converted to a count, and the parked thread's tripwire fires where this repository cannot see it `##a29`

The reply to round 2, the agent. Bears on #search-at-convergence. "'Too often' is not checkable,
so I convert it to a count. The count is your weighing; 2 is my default." The evidence arises in
thaum's sessions, which
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` records as a
gap: "no occasion that reads a tripwires home holds a session's record". The agent proposed a
standing question in the retrospective skill as the re-entry, at the cost of "one more standing
question in every retrospective".

### The tripwire waits for the discussion of watching agent behaviour in other projects `##a30`

Round 3, the owner. Bears on #search-at-convergence. "you do not need to do anything to guarantee
that evidence reaches it and it gets revisited. This waits for the session that discusses solving
the issue a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see, whose plan is to make
use of the retrospective skill to analyze other projects. My later goal is to automate the
retrospective skill to look at the tripwires of this project, without requiring a manual edit of
the questions the retrospective skill asks." The retrospective standing question was dropped, and
the owner's aim was added to that issue in its own commit.

### A spec has no audit, so an entry added after grounding meets no search before the work `##a31`

The reply to round 3, the agent, premortem cause 1. Bears on #search-at-convergence. "A spec has
no audit: the planning skill names no occasion at the start of a spec's work, only at its landing.
A spec written at one date and implemented weeks later gets no read between grounding and review."
"The parked tripwire, as I drafted it, does not see this case: it requires that the entry 'existed
at that search'." The widened firing clause: "a standing entry the work bears on, which no search
before the work returned". "It stays one tripwire with the same re-entry."

### The owner keeps one tripwire of the premortem `##a32`

Round 4, the owner. Bears on #search-at-convergence, #search-return-shape, #search-reading-depth
and #search-at-design-grounding. "I don't think the tripwires from the premortem are worth
watching. I'd keep only the widened search one which we already discussed."

## New names, in one place

```text
planned@agent-skills@content/agents/standing-entry-searcher.md
    the new installed agent; its frontmatter name is knowledge-architect-standing-entry-searcher
planned@agent-config@agents/knowledge-architect-standing-entry-searcher.md
    its installed copy in this repository, written by `cargo klarch install-agent-skills`
```

Two design heads of the agent-skills Component, written at the harvest: slug
standing-entry-search-agent and slug standing-entries-searched-before-the-work.

Until the harvest writes them, the two heads are named by their slugs in plain text, since a
reference to a head that does not exist does not resolve.

## Decided design

### The standing-state reviewer

Thread: #deferred-triggers-at-review.

- The reviewer's section 2 becomes "Re-read every tripwire and every deferred trigger". Beside the
  tripwires, it runs `{{command}} issues --kind deferred`, and reads the `### Trigger` of every
  entry listed, against the change, as a firing clause. A reviewer does not decide in advance which
  triggers are worth reading, for the reason the section already gives for tripwires.
- A trigger the change meets is a finding. It names the issue and what in the change meets the
  trigger. By the trigger test, the occasion the trigger names includes the work the issue names,
  so the repair is that work in this change, or the owner's ruling. The reviewer reports; the
  dispatcher acts, as for every finding.
- "Reporting" names, beside the tripwires re-read, the deferred triggers re-read, with the ones
  the change did not meet.
- The description in the frontmatter adds "every deferred issue's trigger" to what the reviewer
  re-reads.
- The review skill's conformance row says the reviewer is the standing re-entry point of every
  tripwire and every deferred trigger. The issue-tracking skill's line on the standing re-entry
  point says the same.
- Nearest rival: #search-before-undesigned-work, an early search with no host today. It lost
  because no installed skill runs at the start of undesigned work, the evidence is one instance,
  and the line would be a conformance rule in the primer
  (`argument@standing-entry-search@a7`).

### The search agent

Thread: #standing-entry-search-agent.

- A new installed agent, `knowledge-architect-standing-entry-searcher`, with the tools `Read`,
  `Grep`, `Glob` and `Bash`, and no `Write` or `Edit`.
- **Its description carries the dispatch rule**, because a dispatcher reads an agent's
  description and never its body ("Dispatch it; do not read it."). The description says what the
  agent does, who sends it, how many to send and what each brief holds:
  - it searches a group of the issues and the tripwires for the entries a piece of work bears on,
    for the grounding of a design discussion and the design audit of a milestone step, and returns
    them as references;
  - the dispatcher counts the rows of `{{command}} issues` and of `{{command}} tripwires`, the
    header row excluded, and a listing that prints only `(no entry)` counts 0. It sends one agent
    per group of at most N consecutive rows of the combined count, all in parallel. The groups are
    positions 1 to N, N + 1 to 2N, and so on; the last may be smaller;
  - each brief names the work, the seeds and the group's first and last positions;
  - it ends "Dispatch it; do not read it.", as the installed reviewers' descriptions do.

  The two skills that send it say "dispatch the search as the description of
  `knowledge-architect-standing-entry-searcher` says", and do not restate N or the cut. N then has
  one home in the shipped text, and step 4 changes it there.
- **Its body** states the order of the rows, the reading, the standard and the return, below. It
  is written in the vocabulary of the installed skills, and names no project's path.
- **It runs in the live tree, and writes nothing.** It judges the tree the dispatcher has, at a
  moment when the dispatcher edits nothing: the grounding of a discussion, or an audit before a
  step's code. The commands it runs only read. Where the project's command builds the checker,
  the agents of one search share the project's build directory, as the dispatcher's own runs do.
- Nearest rival: #standing-state-second-mode. It lost because the reviewer reads every entry by
  rule and the search filters by relevance, and one text holding both standards lets a session
  apply the wrong one (`argument@standing-entry-search@a10`).

**The brief a search agent receives**, from the dispatcher. A seed is always a decision or a goal,
never an issue or a tripwire:

| field | at design grounding | at a step's design audit |
| --- | --- | --- |
| the work | the question under discussion, stated as the session would state it to the owner | the step's spec, by its citation, and the milestone document |
| the seeds | the decisions and goals the grounding has already read and found bearing on the question; none, when it has found none yet | the decisions the milestone document lists under "What is already decided", and the decisions and goals the step's spec and its threads cite |
| the group | its first and last positions, as cut below | the same |

### How the dispatcher cuts the groups

Thread: #search-partition-unit.

- The dispatcher counts the rows and cuts the groups as the agent's description says, above,
  without reading the rows into its context.
- The order of the rows is fixed in the agent's body, so that every search agent computes the same
  slice: the issue rows sorted by anchor, then by id; then the tripwire rows sorted by anchor, then
  by id. `{{command}} issues` prints its rows by kind first, so the agent always sorts them.
- **N = 60**, as the owner ruled at the audit, "Agreed on the 4 defaults, you can proceed." It rests on the mean entry size in thaum, 1.6 KB,
  and on no measurement of recall. `acceptance@standing-entry-search@replay-returns-late-entries`
  measures it.
- Today that is 1 search agent in this repository, with 58 entries, and 4 in thaum, with 197.
- Nearest rival: one group per anchor. It lost to the measurement that one Component of thaum holds
  51.9% of its standing entries' bytes, so an anchor bounds nothing
  (`argument@standing-entry-search@a28`). A cap in bytes lost because the listings give no sizes.

### How a search agent reads

Threads: #search-seeds-from-references, #search-reading-depth.

1. It runs the two listings, sorts them as above, and takes the rows of its group.
2. **It locates each entry of its group.** It runs `{{command}} show <ref>` on each, which prints
   the reference, then the file and the line of the entry's definition, then the entry whole. The
   agent keeps the file and the line of each, and reads the entry at the same time.
3. **Seeds.** For each seed, it runs `{{command}} show <seed>`, and maps each citing site of its
   output to an entry of its group by the mapping table "From a citing site to an entry", below.
   Each entry so found is returned, with the seed it cites as its reason, whatever step 4 judges.
4. **Every entry of its group, judged.** For every entry of its group, read at step 2, it judges
   whether the work bears on it: the work would fire a tripwire, meet a deferred trigger, touch
   the subject of an issue, close it, make it worse, or depend on its answer.
5. It leans to recall. An entry it cannot rule out is returned, with the doubt as its reason.

Nearest rivals: shape A, one agent for every entry, lost because its load grows with the project
and one agent cannot judge so many entries reliably (`argument@standing-entry-search@a25`,
`argument@standing-entry-search@a26`). Shape C, reading most entries by title, lost to the agent's
argument that the owner's "nothing should be missed", said of where the search runs, rules out a
reading that can miss (`argument@standing-entry-search@a25`).

### What a search agent returns

Thread: #search-return-shape. An illustration of the shape, not authority:

```text
Group: rows 1 to 60 of the sorted listings, 60 entries read.

Bears on the work:
- tripwire@<anchor>@<id>: <one line: what in the work meets which clause of the entry>
- issue@<anchor>@<id> (<kind>): <one line>

Read and judged unrelated:
- <every other reference of the group>

Commands run: <each command, with its arguments>
```

- Each returned entry is a reference, so the dispatcher reads it whole with
  `{{command}} show <ref>`, never from the reason line alone.
- The "judged unrelated" list is complete, so a silent report cannot be mistaken for a skipped one.

### Where the search runs

Threads: #search-at-design-grounding, #search-at-step-audit, #search-covers-every-kind.

- **The design skill**, loop step 1. In place of reading "the open issues and the tripwires" in
  the session, the session dispatches the search as the agent's description says, with the brief
  above. It then runs `{{command}} show` on every entry they return, and reads each
  whole before proposing. The first round states which entries the search returned, and which the
  session judged bearing on the question after reading them. The rest of step 1 is unchanged: the
  goals homes, the design homes, the rejected alternatives and the `README.md` files are still
  read by the session, and `show` is still run on each entry the question bears on.
- **The planning skill**, §7 point 2. In place of "`{{command}} tripwires` and `{{command}} issues
  --kind deferred` list them; read each against the step", the session dispatches the search for
  the step as the agent's description says, with the brief above, and reads whole every entry
  returned. The gap the audit lists
  becomes "a standing entry the step's planned code bears on": a tripwire whose firing condition,
  or a deferred trigger, the planned code meets, and any issue of any kind the step's code touches,
  closes, makes worse or depends on. The sorting of each gap is unchanged.
- **The planning skill**, §7 point 1, drops "its open issues, its tripwires" from the grounding,
  since the audit's search covers every anchor. The approved thread moved the audit's read, and point
  1's read is a separate one, so this was a default, until the owner ruled at the audit, "Agreed on the 4 defaults, you can proceed."
- Nearest rivals: #search-at-plan-review lost to #search-at-convergence, because §8 runs on the
  full path only (`argument@standing-entry-search@a17`); #search-at-convergence is parked with its
  tripwire.

## Mapping tables

**From a citing site to an entry**, used by step 3 of "How a search agent reads". A citing site is
one line of the `referenced at:` part of `{{command}} show`, a file and a line. The locations come
from step 2.

| the citing site's file | the entry it belongs to |
| --- | --- |
| the file of an issue entry of the group | that issue entry |
| a tripwires home holding entries of the group | the entry of the group, in that file, whose definition line is the greatest one not after the site's line; none if every definition line of the group in that file comes after it |
| any other file | no entry of the group: a design head, a rejected alternative, a plan document, a skill or code |

**Each site of the installed text, and what it becomes:**

| site | today | after the work |
| --- | --- | --- |
| design skill, loop step 1 | the session reads the issues and the tripwires | the session dispatches the search as the agent's description says, and reads whole each entry returned |
| planning skill, §7 point 1 | grounds in the Component's issues and tripwires | drops them, as the owner ruled at the audit |
| planning skill, §7 point 2 | the session reads every tripwire and every deferred issue | the session dispatches the search over every issue kind and every tripwire, and reads whole each entry returned |
| standing-state reviewer, description, opening scope text and section 2 | every tripwire | every tripwire and every deferred trigger |
| standing-state reviewer, "Reporting" | names the tripwires re-read | names the tripwires and the deferred triggers re-read |
| review skill, §1, conformance row | "the standing re-entry point of every tripwire" | of every tripwire and every deferred trigger |
| issue-tracking skill, tripwire entry section | "reads every tripwire home again" | also every deferred trigger |
| `path@agent-skills@content/agents/` | six agents | seven: `standing-entry-searcher.md` added |

## Losing alternatives

- **#standing-state-second-mode**, withdrawn by the owner: one agent text would hold the
  reviewer's rule to read every entry beside the search's relevance filter
  (`argument@standing-entry-search@a10`).
- **#search-at-plan-review**, ruled out: lost to #search-at-convergence, since a discussion on the
  in-change path writes no plan document and would get no read between grounding and review
  (`argument@standing-entry-search@a17`, `argument@standing-entry-search@a18`).
- **Shape A of #search-reading-depth**, one agent reading every entry whole: lost because its load
  grows linearly with the project, about 80k tokens in thaum today, and one agent cannot judge so
  many entries reliably (`argument@standing-entry-search@a25`, `argument@standing-entry-search@a26`).
- **Shape C of #search-reading-depth**, most entries read by title: lost to the agent's argument
  that the owner's "nothing should be missed" rules out a reading that can miss
  (`argument@standing-entry-search@a25`).
- **One group per anchor**, in #search-partition-unit: lost to the measurement that one Component
  of thaum holds 51.9% of the bytes (`argument@standing-entry-search@a28`).
- **A cap in bytes per group**: lost because the listings give no sizes
  (`argument@standing-entry-search@a28`).
- **A retrospective standing question as the re-entry of the tripwire of #search-at-convergence**:
  lost to the owner's word that the tripwire waits for the discussion of the agent-behaviour issue
  (`argument@standing-entry-search@a30`).
- **The rival to #search-covers-every-kind**, the audit kept at tripwires and deferred issues:
  lost because it leaves the late findings the owner described in place
  (`argument@standing-entry-search@a23`).

## Readings

The work reads no external specification.

## Premortem

Assume the work shipped and thaum's reviews keep finding entries after the work.

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. An entry is added after the grounding search, and a spec's work meets no search before the review, since a spec has no audit | #search-at-convergence | becomes a tripwire, on the owner's word, widened: "a standing entry the work bears on, which no search before the work returned" (`argument@standing-entry-search@a31`, `argument@standing-entry-search@a32`) |
| 2. The session works from the reason lines and never runs `show` | #search-return-shape | a tripwire was proposed; the owner declined it: "I don't think the tripwires from the premortem are worth watching." |
| 3. A search agent misses an entry of its group | #search-reading-depth, #search-partition-unit | survives into `acceptance@standing-entry-search@replay-returns-late-entries`; the tripwire proposed for after the landing was declined by the owner |
| 4. The seed is empty at the grounding of a new question | #search-seeds-from-references | converted into a clause of the design: the brief names the seeds when they are known |
| 5. Grounding dispatches between 1 and 4 agents in thaum, and sessions skip it on small decisions | #search-at-design-grounding | a tripwire was proposed; the owner declined it |
| 6. A deferred trigger names an event no diff shows | #deferred-triggers-at-review | survives: such a trigger fails the trigger test, and the reviewer already reports it under its predicate (`argument@standing-entry-search@a8`) |

## Acceptance criteria

### A replay over thaum's history returns every entry its reviews found after the work `##replay-returns-late-entries`

- **Guards**: `thread@standing-entry-search@search-reading-depth` and
  `thread@standing-entry-search@search-partition-unit`.
- **Judged at**: step 4, before the harvest.
- **The replay**:
  1. In a worktree of thaum outside this repository, find the commits whose message records a
     finding of the standing-state reviewer about an issue or a tripwire that the work bore on,
     and that was found after the work: in a review commit of a milestone step, or the W3 case.
     The commit messages are the record; `git log --grep` over "standing-state" finds the
     candidates, and each is read before it is counted.
  2. For each case, create a detached worktree of thaum at the commit before the work began: the
     commit of the step's design audit, or for W3 the parent of thaum's commit 6e0da6e. The new
     agent exists only in this repository's branch, and the harness may not list an agent added
     during a session. So each search agent is dispatched as a general-purpose subagent, told to
     follow the body of `planned@agent-skills@content/agents/standing-entry-searcher.md` as it
     stands on this branch, with `{{command}}` read as thaum's command, `cargo klarch`, run in the
     worktree with the build directory inside it. The dispatcher follows the agent's description,
     with the work stated as the step's spec or, for W3, as "move the pinned version of
     knowledge-architect to 0.3.0", and N = 60. The W3 case is a test of recall only: work that is
     neither designed nor planned sends no search under this design.
  3. Record, for each case, whether the entry the reviewer found is returned under "Bears on the
     work". A case whose pinned checker lacks a command the agent runs, such as `issues` or
     `tripwires`, is not replayed, and the report names it with that reason.
- **Fires when**: an entry of step 3 is not returned.
- **Response**: replay the cases that missed with N = 30. If they are then returned, the default
  of N becomes 30, put to the owner. If an entry is still missed, the criterion fires: the reading
  of #search-reading-depth reopens, under `knowledge-architect-design`, before the harvest.
- N = 60 and N = 30 are as the owner ruled at the audit. If thaum is not on the machine, or holds
  no such case, step 6's commit says so, and the owner rules whether the work lands without the
  replay.
- **Reported** in the message of step 6's commit, which lands the work and deletes this spec: each
  case, its commit, whether the entry was returned, and the N used.

## Implementation sequence

Steps 1, 2, 3, 5 and 6 are commits; step 4 is a run, which makes a commit only when it changes N.
The commits sit on this spec's branch, after the commits of its reviews and of its audit, as the
owner ruled at the audit. Each commit that changes `path@agent-skills@content/` runs
`cargo klarch install-agent-skills` and holds the installed copies, per
`path@agent-skills@CLAUDE.md`. Each passes the tests of that file's section "Editing an installed
skill or agent". Each changes what an agent is told to do, so it is a decision under
`knowledge-architect-agent-configuration` §1: the decisions are the approved threads of this spec,
and step 6 records them.

1. **The search agent.** `planned@agent-skills@content/agents/standing-entry-searcher.md`, per
   "The search agent", "How the dispatcher cuts the groups", "How a search agent reads", the
   mapping table "From a citing site to an entry" and "What a search agent returns", and its
   installed copy, with N = 60. Claims and checks:
   - the build maps the file and the install writes its copy: `cargo build` passes, and
     `cargo klarch check` passes with the copy committed;
   - the agent returns every entry of its group, under one of its two lists. Check: a trial
     search over this repository, the work stated as "the standing-state reviewer reads every
     deferred trigger", the seed `design@agent-skills@conformance-before-every-merge`. Every
     reference of the group appears in exactly one list, and
     `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, which
     cites the seed, is under "Bears on the work" with the seed as its reason. The same trial with
     a group that excludes that issue's row must not return it, which shows that the check can
     fail. The search agent is dispatched as the replay of step 4 says, since the harness may not
     list the new agent.

   Fails alone on: the build, `cargo klarch check`, or the trial.
2. **The occasions.** The design skill's loop step 1 and the planning skill's §7 points 1 and 2, per "Where the search runs". Claim and check: no
   sentence of either skill still sends the session to read every issue and every tripwire itself;
   a reading of loop step 1 and of §7 points 1 and 2 after the edit. Fails alone on: such a
   sentence.
3. **The review side.** The standing-state reviewer, the review skill's conformance row and the
   issue-tracking skill's line, per "The standing-state reviewer". Claim and check: every
   restatement in the table of "What is already decided" names the deferred triggers beside the
   tripwires; a reading of each. Fails alone on: a text that still names the standing-state
   reviewer as the re-entry point of the tripwires alone.
4. **The replay**, per `acceptance@standing-entry-search@replay-returns-late-entries`. When it
   changes N, it changes the agent's description in a commit of its own, installed.
5. **The changelog.** Three entries in the `Next release` section of CHANGELOG.md, under Workflow,
   each `agent-skills`, patch: a new agent searches the issues and tripwires a piece of work bears
   on, dispatched at the grounding of a design discussion and at the design audit of a milestone
   step, and the session reads each entry it returns whole; the step audit reads every issue kind,
   not only the deferred ones; the standing-state reviewer reads every deferred issue's trigger
   before every merge. Then `cargo x changelog`. Fails alone on: a changelog copy that differs.
   The review of §7 point 4 runs after this step.
6. **The harvest**, below, its review, and the deletion of this spec, in one commit whose message
   reports on the acceptance criterion.

## Order rationale

The audit before step 1: the owner's rulings on the defaults set N and the scope of step 2. Step 1
before step 2: the skills send the session to an agent that must exist. Step 2 before step 3:
step 3 changes only the review side, and either order passes; the order keeps the before-the-work
changes together. Step 3 before step 4: the replay runs the installed text whole. Step 4 before
step 5: the replay can change N, which the changelog does not name, and can reopen a decision,
which would change the entries. Step 5 before step 6: the harvest writes the heads the changelog's
work implements.

## Defaults awaiting the owner

None. Four defaults stood after the reviews of this spec: N = 60 and N = 30; the count 2 in the
tripwire of #search-at-convergence; the drop of "its open issues, its tripwires" from §7 point 1;
and the work on this spec's branch. The owner ruled at the audit, "Agreed on the 4 defaults, you can proceed."

## Harvest

At step 6, under `knowledge-architect-decision-recording` and `knowledge-architect-issue-tracking`.
The recording tests decide whether each decision and each alternative earns an entry; an item of
this row they exclude is named in the harvest's commit, with the test it fails.

| item | home |
| --- | --- |
| #deferred-triggers-at-review | `design@agent-skills@conformance-before-every-merge`, rewritten in place; the slug stays, since the head already names the decision the thread widens; its title is reworded to name the deferred triggers beside the tripwires if it no longer states the widened decision |
| #standing-entry-search-agent, with #search-return-shape, #search-seeds-from-references, #search-reading-depth and #search-partition-unit | a new head in `path@agent-skills@docs/design.md`, slug `standing-entry-search-agent`, under "The workflow the skills carry" |
| #search-at-design-grounding, with #search-at-step-audit and #search-covers-every-kind | a new head in `path@agent-skills@docs/design.md`, slug `standing-entries-searched-before-the-work`, under "The workflow the skills carry"; the thread slug #search-at-design-grounding names one of its two occasions, so the head takes a slug that names both |
| every item of "Losing alternatives": #standing-state-second-mode, #search-at-plan-review, shapes A and C, one group per anchor, a cap in bytes, the retrospective standing question, the audit kept at two kinds | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| the tripwire of #search-at-convergence | `path@agent-skills@docs/tripwires.md`, guarding the head `standing-entries-searched-before-the-work`; fires when, across sessions, 2 standing-state reviews report a standing entry the work bears on which no search before the work returned, the count as the owner rules it; re-entry: the design discussion of `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` |
| the tripwire of #search-before-undesigned-work | `path@agent-skills@docs/tripwires.md`, guarding `design@agent-skills@conformance-before-every-merge`; fires when a second instance of a deferred trigger is met by undesigned work and found only at the review; re-entry: the design of `issue@agent-skills@a-skill-for-bounded-problems` |
| the acceptance criterion | reported on in the message of step 6's commit; it does not recur at later work, so it leaves with this spec |
| this spec | deleted in the harvest's commit, cited as `spec@plans@standing-entry-search` |

## Later consequences

- `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`, once built, replaces
  steps 2 and 3 of "How a search agent reads" and the mapping table "From a citing site to an
  entry": one command gives the seeded entries, already named. That issue's closing condition
  includes the change of the agent's body, since the shipped text cannot reference the issue.
- The design of `issue@agent-skills@a-skill-for-bounded-problems` is the re-entry of
  #search-before-undesigned-work, and may send the search before undesigned work.
- The design discussion of
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` reads the
  tripwire of #search-at-convergence.
- A project whose own instructions ask to ground in a Component's issues and tripwires, as this
  repository's root CLAUDE.md does, keeps that main-session read until it changes its own text.
