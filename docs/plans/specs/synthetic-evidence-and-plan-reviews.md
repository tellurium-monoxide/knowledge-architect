# No synthetic evidence judges the workflow, a plan document is read against the record, and what the owner rules on by name is labelled

## Status and audience

This spec is the plan of the work that settles the agent-skills issue on whether synthetic evidence
about the workflow is built at all. The commit adding this spec deletes that issue, since this spec
schedules its work; its text is in that commit's parent. The work does four things:

- it states that no run is built to observe how an agent follows the workflow, in the root's goal
  and in a new head, and admits a run only when its verdict reproduces;
- it adds an installed reviewer that reads a plan document against the goals, the design heads and
  the rejected alternatives of the project;
- it makes every acceptance criterion stand on the owner's word, as a tripwire does;
- it labels every item the owner is asked to rule on by name and that carries no slug.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **No gate fixes when this spec lands against its work.** The work changes installed text and
  documents, and no per-commit check that the work changes. So
  `design@agent-skills@plan-lands-before-gate-change` does not apply, and the work continues on
  this spec's branch, in one pull request, after the reviews of this spec.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/252aee87-5e8a-4158-ab08-6ba759e29de9.jsonl`.
  The discussion begins at the owner's message that opens "I want to discuss and close the
  synthetic-evidence-about-the-workflow-is-undecided issue in this session", and ends at the
  owner's message "c-review-cost accepted, go ahead with the spec". It holds 5 owner messages,
  called rounds 1 to 5 below, and 4 agent replies, one after each of rounds 1 to 4.

## How the work is done

Per `knowledge-architect-planning`, §7, the work of a spec. The work starts in the session where
the discussion converged, so §7 point 2, the design audit, runs only if commits other than this
spec's own land on the main branch before the work starts.

## Names

- **a synthetic run**: an agent session, or a dispatch of an installed agent, run on a task built
  to produce evidence, rather than on the project's work. A replay of past work is one.
- **a reproducing verdict**: the verdict of a run that, repeated, gives the same verdict, at a cost
  low enough to repeat it.
- **a §8 moment**: a moment at which §8 of the installed planning skill sends its reviewers: a plan
  document is written, or a decided shape in one is revised.
- **a label**: a prefix of capital letters naming a kind of item, followed by a number, as `T1` or
  `AC2`. It is not a slug, and the checker does not read it.
- **an item ruled by name**: an item the owner is asked to rule on one by one, such as a tripwire,
  an acceptance criterion, a question of entry test 4, or a retrospective finding.
- **the installed text**: the files under `path@agent-skills@content/`, and their installed copies
  under .claude, per `path@agent-skills@CLAUDE.md`. `{{command}}` is the install placeholder for
  the project's command; in this repository it is `cargo klarch`.
- **instance 1, instance 2**: the two events the closed issue recorded. Instance 1: a spec's
  acceptance criterion, a replay of a search agent over thaum's history, written at the premortem,
  passed the spec's three reviews, and was ruled out by the owner at the step that ran it.
  Instance 2: a slice spec's acceptance criterion, a fresh subagent classifying three decisions
  written for the check, added after the premortem with no word of the owner, passed the milestone
  document's three reviews, and was dropped at the slice's design audit, because the search
  returned the issue.

## What the work is

What exists today at each site the work touches:

| site | today |
| --- | --- |
| `path@knowledge-architect@docs/goals.md`, the goal `goal@knowledge-architect@the-workflow-improves-through-real-use` | "Real sessions are the test of the workflow, not synthetic scenarios. The retrospective collects what was unclear, missing or wrong, and the owner decides what changes." |
| `design@agent-skills@additions-need-real-use` | "Published literature and synthetic scenarios originate no edit." It covers the origin of an edit, not evidence between shapes or an acceptance check |
| the installed design skill, `path@agent-skills@content/skills/design/SKILL.md`, loop step 5 | "Build the discriminating artifact the project affords — a failing test, a throwaway prototype, a benchmark, a mockup." No sentence on evidence a project's record rules out |
| the same skill, loop step 7 | "what survives is proposed as tripwires, except a cause that only the built code can check: that is an acceptance criterion, which `knowledge-architect-planning` writes into the plan document". "Ask the owner, for each tripwire, whether it is recorded." No ruling is asked on an acceptance criterion, and no label is given to a tripwire |
| the installed planning skill, `path@agent-skills@content/skills/planning/SKILL.md`, §6 | an acceptance criterion names the decision, the step, the observable and the response; "A number in a criterion is a threshold the owner sets." No word of the owner admits the criterion itself |
| the same skill, §8 | three reviewers: `knowledge-architect-cold-implementer-reviewer`, `knowledge-architect-code-claims-reviewer`, `knowledge-architect-transcript-reviewer`. None reads the document against the goals, the design heads or the rejected alternatives |
| the same skill, §7 point 2 | the design audit reads decided shapes "against the code as it stands and against the design homes", by the implementing session, and is skipped for a spec worked in the session that converged |
| the installed review skill, `path@agent-skills@content/skills/review/SKILL.md`, §1 | its axis table names the standing-state reviewer's row "conformance", and holds a row "spec conformity" |
| the installed agents, `path@agent-skills@content/agents/` | seven agents; none reads a plan document against the project's record |
| the installed decision-recording skill, entry test 4 | "one numbered question per decision (Q1, Q2, …)" |
| the installed retrospective skill | "**W** for the installed skills and agents, **C** for the checker, **P** for the project's own instructions", numbered within one file |

Outside the work:

- **An audit of a project as a whole**, every goal against every head: owned by
  `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`. The new reviewer reads one plan
  document, not the project.
- **Whether the standing-state reviewer reads a design issue's re-entry point**: owned by
  `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review`. Its evidence is a
  trial run of the search agent, which the new rule would not build; the entry stands on its
  reading of the reviewer's text, which a reading confirms.
- **The slug `conformance-before-every-merge`** keeps its old word. Its title already reads "The
  standing-state review runs before every merge", and renaming it repairs three citations for no
  reader's gain. Only the axis row is renamed.

## What is already decided

The design rests on these decisions, and does not argue them again:

- `design@agent-skills@capabilities-not-structure`: each addition is judged against it in its
  thread.
- `design@agent-skills@premortem-tripwires-on-the-owners-word` and
  `design@agent-skills@tripwire-wording-is-the-agents`: #acceptance-criteria-on-the-owners-word
  takes their shape for acceptance criteria.
- `design@agent-skills@acceptance-criteria-in-the-document`: criteria stay in the plan document
  of the work that judges them.
- `design@agent-skills@shipped-text-cites-no-entry`: the new agent text and every edit of the
  installed text cite no entry, and write the command as `{{command}}`.
- `design@agent-skills@content-mirrors-the-install-layout`: the build maps the new agent file.

The work rewrites one goal and one decision. The goal is reworded on the owner's word in round 3,
under `knowledge-architect-goal-setting`. The decision gains a pointer to the new head. Every text
that `cargo klarch show` lists as referencing each, on the main branch before this spec, other than
the closed issue:

| text | references | judged or updated at |
| --- | --- | --- |
| `path@agent-skills@content/skills/retrospective/SKILL.md`, a `%%` comment | the goal, for the retrospective's scope | step 1: it stays true |
| `path@agent-skills@docs/design.md`, `design@agent-skills@additions-need-real-use` | the goal | the harvest, step 5: rewritten with a pointer to the new head |
| `path@agent-skills@docs/design.md`, `design@agent-skills@standing-entry-search-agent` | the goal, "real sessions measure it" | step 1: it stays true |
| `path@knowledge-architect@docs/design.md`, `design@knowledge-architect@committed-findings-analysis` | the goal | step 1: it stays true |
| `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`, `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, `issue@agent-skills@the-primer-question-cannot-be-answered-for-subagents`, `issue@agent-skills@the-retrospective-counts-no-review-cost` | the goal | step 1: each stays true, since the goal is widened, not narrowed |
| `path@agent-skills@CLAUDE.md`, test 2 | the head | the harvest, step 5: it stays true |
| `path@agent-skills@docs/design.md`, `design@agent-skills@expectation-set-bounds-scope` and `design@agent-skills@standing-entries-searched-before-the-work` | the head | the harvest, step 5: each stays true |
| `path@agent-config@skills/klarch-retrospective-intake/SKILL.md` | the head, for an addition's evidence | the harvest, step 5: it stays true |
| `issue@agent-skills@a-reviewer-s-running-time-is-unbounded`, `issue@agent-skills@expectation-sets-for-the-installed-skills` | the head | the harvest, step 5: each stays true |
| `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`, §4 | cited the closed issue for "No subagent replays the reported session or builds a scenario of agent behaviour"; the commit adding this spec points it to this spec | the harvest, step 5: pointed to the head `synthetic-evidence-not-built`, since this spec leaves |

## Criteria

### The workflow is judged by real sessions `##c-real-use`

Binding. Derived from `goal@knowledge-architect@the-workflow-improves-through-real-use`. Met by
`thread@synthetic-evidence-and-plan-reviews@synthetic-evidence-not-built` and
`thread@synthetic-evidence-and-plan-reviews@goal-wording-real-use`.

### A departure from the recorded design is caught by a check or a review before it merges `##c-caught-before-merge`

Binding. Derived from `goal@knowledge-architect@agents-work-without-drift`, which states it
verbatim. Met by `thread@synthetic-evidence-and-plan-reviews@plan-read-against-the-record`.

### The owner rules on weighings `##c-owner-weighs`

Binding. Derived from `goal@knowledge-architect@the-owner-decides`. Met by
`thread@synthetic-evidence-and-plan-reviews@acceptance-criteria-on-the-owners-word` and
`thread@synthetic-evidence-and-plan-reviews@ruled-items-labelled`.

### Installed text holds in any project `##c-works-anywhere`

Binding. Derived from `goal@agent-skills@installed-text-works-anywhere`. Met by
`thread@synthetic-evidence-and-plan-reviews@step-5-evidence-the-record-admits`, whose sentence
names the project's goals and heads rather than this project's rule, and by
`thread@synthetic-evidence-and-plan-reviews@plan-read-against-the-record`, whose agent reads the
homes every conforming project carries.

### No structure the work does not need `##c-no-imposed-structure`

Binding. Derived from `goal@agent-skills@installed-text-leaves-room-to-judge`, through
`design@agent-skills@capabilities-not-structure`. Met: one agent, one sentence, and one labelling
rule whose scope follows the record that carries the label, per
`thread@synthetic-evidence-and-plan-reviews@ruled-items-labelled`.

### Each installed addition rests on a real session, the owner's named lack, and a mechanism `##c-addition-on-real-use`

Binding as a presumption. Derived from `design@agent-skills@additions-need-real-use`. Met: the two
instances the closed issue recorded, and the owner's requests in rounds 2 and 3 of this
discussion.

### The rule does not reach tests of the checker's code, mock projects included `##c-checker-tests-untouched`

Binding. Stated by the agent in round 1. Met by
`thread@synthetic-evidence-and-plan-reviews@synthetic-evidence-boundary`, which admits such tests
as a case of a reproducing verdict.

### Each new axis adds one dispatch at every §8 moment `##c-review-cost`

Weighed. Stated by the agent in round 1. Unmet and accepted: the new reviewer is one more
dispatch at every §8 moment, where three are sent today. The owner's words, round 5:
"c-review-cost accepted, go ahead with the spec".

## Threads

### No run is built to observe how an agent follows the workflow's instructions, for any purpose `##synthetic-evidence-not-built`

Proposed by the owner, round 1. Approved. Arguments: `argument@synthetic-evidence-and-plan-reviews@a1`,
`argument@synthetic-evidence-and-plan-reviews@a2`, `argument@synthetic-evidence-and-plan-reviews@a5`,
`argument@synthetic-evidence-and-plan-reviews@a6`, `argument@synthetic-evidence-and-plan-reviews@a17`,
`argument@synthetic-evidence-and-plan-reviews@a18`. Shape: Decided design, "The rule on synthetic
evidence". Harvest: the new head `synthetic-evidence-not-built`. The owner's words, round 2:
"synthetic-evidence-not-built: approved. If the loss you mentionned is the only one you could
think of, indeed it is small. Most things we do here are already well proven to work."

### A run is admitted only when its verdict reproduces at low cost; an agent's decision is presumed not to `##synthetic-evidence-boundary`

Proposed by the agent, round 1, as a definition by subject. Approved in round 2 with the owner's
addition, reopened by the owner in round 3, and approved in round 4 as a property. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a7`, `argument@synthetic-evidence-and-plan-reviews@a17`,
`argument@synthetic-evidence-and-plan-reviews@a18`, `argument@synthetic-evidence-and-plan-reviews@a26`,
`argument@synthetic-evidence-and-plan-reviews@a28`, `argument@synthetic-evidence-and-plan-reviews@a29`.
Shape: Decided design, "The rule on synthetic evidence". Harvest: the new head
`synthetic-evidence-not-built`. The owner's words: round 2, "synthetic-evidence-boundary: agreed on
this, plus my addition above. Findings from retrospective, or observation of a misbehavior during
real use, can still be acted upon. The point is more to not waste time and resources with building
mock testing for synthetic evidence, which I consider unreliable."; round 3, reopening, "I'm not
sure what I flagged as not refused (harness mechanics) is fully exhaustive. If there is one, I'd
rather have the head name a wider category that matches my intent here."; round 4,
"synthetic-evidence-boundary wording approved".

### The goal states that no session is built to observe how agents follow the workflow's instructions `##goal-wording-real-use`

Proposed by the agent, round 1; redrafted by the agent, round 2. Approved. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a18`, `argument@synthetic-evidence-and-plan-reviews@a23`. Shape: Decided design, "The goal". Harvest:
the goal, rewritten at step 1. The owner's words: round 2, "I agree with that wording. But maybe my
above statements can lead to more rewording again (on your judgement), so it might not be closed
yet."; round 3, "goal-wording-real-use: agreed on wording".

### The decision is a new head, and the sentence of `additions-need-real-use` points to it `##synthetic-evidence-head`

Proposed by the agent, round 1. Approved. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a8`. Shape: Decided design, "The rule on synthetic
evidence". Harvest: the new head `synthetic-evidence-not-built`, and
`design@agent-skills@additions-need-real-use` rewritten in place. The owner's words, round 2:
"synthetic-evidence-head: approved."

### The design skill's step 5 builds no evidence the project's goals or heads rule out `##step-5-evidence-the-record-admits`

Proposed by the agent, round 1. Approved. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a9`. Shape: Decided design, "Step 5 of the design
skill". Harvest: none of its own; the sentence is a directive of the installed text, under the
head `synthetic-evidence-not-built`. The owner's words, round 2:
"step-5-evidence-the-record-admits: agreed."

### A fresh reviewer reads a plan document against the goals, the design heads and the rejected alternatives at every §8 moment `##plan-read-against-the-record`

Proposed by the agent, round 1. Approved, under the owner's name for the agent. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a3`, `argument@synthetic-evidence-and-plan-reviews@a10`,
`argument@synthetic-evidence-and-plan-reviews@a11`, `argument@synthetic-evidence-and-plan-reviews@a12`,
`argument@synthetic-evidence-and-plan-reviews@a13`, `argument@synthetic-evidence-and-plan-reviews@a14`,
`argument@synthetic-evidence-and-plan-reviews@a19`, `argument@synthetic-evidence-and-plan-reviews@a20`,
`argument@synthetic-evidence-and-plan-reviews@a22`, `argument@synthetic-evidence-and-plan-reviews@a25`.
Shape: Decided design, "The design-conformance reviewer". Harvest: the new head
`plan-read-against-the-record`, and the tripwire T1. The owner's words: round 2, "I would call the
agent "design-conformance-reviewer". I think it is clearer than yours, even if it does not covers
goals in the name (this is a trivial loss IMO). The shape looks fine."; round 3, on the material
finding that the name collided with the "conformance" axis, "I still think my name is clearer.
Unless you can propose a good substitute for "conformance". But even then, it fits with the
"spec-conformity" one: both check conformance of something against the first word in their name."
The owner's acknowledgement, round 2, of the limit of
`argument@synthetic-evidence-and-plan-reviews@a14`: "obviously, this is always dependent on the
project being diligent in its goals and design records. Everything in the workflow is, after all.
Project owners still have to watch agent behavior and interrupt them if they find them acting
differently than their intent. Whether this is a published workflow defect or an internal
documentation defect is left to the retrospective skill to judge."

### The review skill's "conformance" axis row is renamed "standing state" `##standing-state-axis-name`

Proposed by the agent, round 2, as one of two answers to the name collision; the owner chose it.
Approved. Arguments: `argument@synthetic-evidence-and-plan-reviews@a22`,
`argument@synthetic-evidence-and-plan-reviews@a25`. Shape: Decided design, "The design-conformance
reviewer". Harvest: none; a renamed label fails the entry tests, which the harvest's commit names.
The owner's words: round 3, "I have no idea why this row is called conformance. Rename it."; round
4, "standing-state-axis-name approved". The slug `conformance-before-every-merge` stays, as the
agent's default in round 3, which the owner's round 4 word did not contest.

### An acceptance criterion stands on the owner's word, as a tripwire does `##acceptance-criteria-on-the-owners-word`

Proposed by the owner, round 1. Approved. Arguments: `argument@synthetic-evidence-and-plan-reviews@a4`,
`argument@synthetic-evidence-and-plan-reviews@a10`, `argument@synthetic-evidence-and-plan-reviews@a15`.
Shape: Decided design, "Acceptance criteria on the owner's word". Harvest: the new head
`acceptance-criteria-on-the-owners-word`. The owner's words, round 2:
"acceptance-criteria-on-the-owners-word: approved."

### A review axis dedicated to acceptance criteria `##acceptance-criteria-axis`

Proposed by the owner, round 1. Superseded by
`thread@synthetic-evidence-and-plan-reviews@plan-read-against-the-record`. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a16`. Shape: Losing alternatives. The owner's words,
round 2: "acceptance-criteria-axis: I agree with you. I really was unsure about this proposal."

### Tripwires and acceptance criteria at the premortem carry `T<n>` and `AC<n>` labels `##premortem-items-numbered`

Proposed by the owner, round 2. Approved in round 3, then widened by the owner in the same message,
and superseded by `thread@synthetic-evidence-and-plan-reviews@ruled-items-labelled`. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a21`, `argument@synthetic-evidence-and-plan-reviews@a23`.
Shape: Decided design, "Labels". The owner's words, round 3: "premortem-items-numbered: approved.
On second thought, this should cover everything that I have to rule on by name".

### Every item the owner rules on by name and that carries no slug gets a label `##ruled-items-labelled`

Proposed by the agent, round 3, from the owner's widening. Approved, with the owner's condition on
labels that reach no committed document. Arguments:
`argument@synthetic-evidence-and-plan-reviews@a21`, `argument@synthetic-evidence-and-plan-reviews@a27`,
`argument@synthetic-evidence-and-plan-reviews@a30`, `argument@synthetic-evidence-and-plan-reviews@a31`.
Shape: Decided design, "Labels". Harvest: the new head `ruled-items-labelled`. The owner's words,
round 4: "ruled-items-labelled: approved. If too large, the list of places where I'm asked for a
ruling can be left as an open issue. But this should be small enough work to handle. Also, in the
case where the item ID does not reach any record and survive in any committed document (even
temporarily), it would be fine to reuse a common letter (such as Q, like in decision record
questions). Instead of searching for unique letters, which will necessarily end up needing more
letters than available."

## Arguments

### Synthetic evidence is too unreliable, and a mock task is designed and tested work `##a1`

Round 1, the owner. Bears on #synthetic-evidence-not-built. "the synthetic evidence is too
unreliable. It would need at least statistical analysis, which is too costly, and a proper mock
task cannot be created on a whim, it needs design and testing itself."

### Retrospectives from real work, and this project following its own workflow, cover most of the need `##a2`

Round 1, the owner. Bears on #synthetic-evidence-not-built. "the project is getting fed with
retrospective reports from real work, which is much better and covers most of the need for
synthetic evidence (though late). It is also getting hands on experience simply by the fact that
the project itself follows the workflow it publishes."

### The reviews of a plan document look at the document, not at how it fits the project `##a3`

Round 1, the owner. Bears on #plan-read-against-the-record. "nothing seems to be reviewing a plan
document against recorded goals and design. … I think they are all turned "inward", that is they
look at the plan document itself (and they look at it first), but do not check how it fits the
project."

### An acceptance criterion needs the owner's word, and maybe an axis of its own `##a4`

Round 1, the owner. Bears on #acceptance-criteria-on-the-owners-word and #acceptance-criteria-axis.
"I lean toward them needing my word indeed. And maybe those acceptance criterion would need a
specific plan review axis focused on them: are they relevant, are they covering the design
sufficiently, such questions."

### One run of an agent is a sample of one `##a5`

Round 1, the agent. Bears on #synthetic-evidence-not-built. Agent output varies between runs. "One
run is a sample of one. Instance 2's criterion reopened a decision on "a second failure", which is
a sample of two." Separating two shapes needs repeated runs per shape and a test on top. The
repository installs its own skills with `cargo klarch install-agent-skills`, so its own sessions
use each change before a release reaches a consumer.

### The one loss is a check that a text's mechanics run, and it is small `##a6`

Round 1, the agent. Bears on #synthetic-evidence-not-built. "Example: dispatching a new agent once
to see that its tools are granted and its report format parses. I argue the loss is small. The
first real dispatch is in this repository's own next review, and a failure of that kind is visible
at once and costs one re-dispatch." The cost of the rule: "a defect in a new text is found later, at
its first real use and not before the merge."

### The rule needs a boundary, or it reaches the checker's tests `##a7`

Round 1, the agent. Bears on #synthetic-evidence-boundary and #c-checker-tests-untouched. "A
synthetic session is an agent session run on a task built to observe how the agent behaves, rather
than on the project's work. A replay of past work is one. A dispatch made in real work is not, even
the first use of a new agent. A test of the checker's code is not, mock projects included."

### Widening `additions-need-real-use` would leave its title short of its scope `##a8`

Round 1, the agent. Bears on #synthetic-evidence-head. The head's title is about additions to
installed text; the decision covers evidence for any purpose. Widening it would make "its title
would stop stating its scope", which `tripwire@agent-skills@title-stops-stating-scope` watches.

### Instance 1's replay was proposed before any plan document existed `##a9`

Round 1, the agent. Bears on #step-5-evidence-the-record-admits. The rival, relying on the outward
review alone, fails: "in instance 1 the replay was first proposed as evidence between shapes during
the discussion. No plan document existed yet for a review to read." The sentence is admissible
under `design@agent-skills@additions-need-real-use`, "because instance 1 was this step's suggestion
acted on."

### What escapes is what the agent writes after the owner's last word `##a10`

Round 1, the agent. Bears on #plan-read-against-the-record and
#acceptance-criteria-on-the-owners-word. "The approved shapes were argued against the goals at the
discussion's grounding. Both instances were acceptance criteria. Instance 1 was proposed at the
premortem. Instance 2 was added while writing a slice spec. Neither had a ruling. So the outward
review and the owner's word on criteria cover one gap from two sides."

### The decision-record reviewer judges the record a diff writes, and holds plan items as not decisions yet `##a11`

Round 1, the agent. Bears on #plan-read-against-the-record. The installed decision-record reviewer
says plan documents "hold shapes for unbuilt work that are deliberately not decisions yet"; it
judges what the diff leaves in design homes. Extending it "would carry two questions with two
different yardsticks."

### The design audit is run by the implementing session, and skipped for a same-session spec `##a12`

Round 1, the agent. Bears on #plan-read-against-the-record. "It is run by the implementing session.
In instance 1 that session was the author of the criterion. It is also skipped for a same-session
spec. A fresh reader is what §8 exists for."

### A plan that contradicts a goal is still actionable `##a13`

Round 1, the agent. Bears on #plan-read-against-the-record. The cold implementer's stance is "can I
act on this", "and a plan that contradicts a goal is perfectly actionable."

### The reviewer alone would not have stopped instance 1 `##a14`

Round 1, the agent. Bears on #plan-read-against-the-record and #goal-wording-real-use. The owner
said the goal and the head were "simply not wide enough to cover your use case". So the reviewer
works only together with the rewording of the goal and the new head.

### Choosing what may reopen a decision is a weighing `##a15`

Round 1, the agent. Bears on #acceptance-criteria-on-the-owners-word. "a criterion's response
reopens a decision. Choosing the observable that may reopen your decision is a weighing. §6 already
makes its threshold yours: "A number in a criterion is a threshold the owner sets." The current
asymmetry gives the agent the power to set what reopens a decision but not the number." The
standing-state reviewer already calls a criterion "the same check for unbuilt work".

### Relevance is already handled in three places, and coverage inflates criteria `##a16`

Round 1, the agent. Bears on #acceptance-criteria-axis. Admissibility goes to the new reviewer;
being well formed is a readiness check the cold implementer applies; being worth its cost is a
weighing, the owner's. A separate agent would repeat the grounding reads of the goals and design
homes for one section. Coverage pushes toward more criteria, which
`design@agent-skills@acceptance-criteria-in-the-document` warns against: the habitual criterion "in
place of a specific one that is harder to find".

### Harness mechanics are not textual output, and their verdicts reproduce `##a17`

Round 2, the owner. Bears on #synthetic-evidence-not-built and #synthetic-evidence-boundary. "the
synthetic evidence refusal covers behavior of agents and following of instructions, but not testing
things like harness mechanics (loading skills, triggering hooks...), which is not related to the
textual output an LLM produces, and where verdicts are reproducable much more reliably."

### Real-use findings are acted on; the point is not to build mock testing `##a18`

Round 2, the owner. Bears on #synthetic-evidence-boundary and #goal-wording-real-use. "Findings
from retrospective, or observation of a misbehavior during real use, can still be acted upon. The
point is more to not waste time and resources with building mock testing for synthetic evidence,
which I consider unreliable."

### The owner's name for the agent is clearer `##a19`

Round 2, the owner. Bears on #plan-read-against-the-record. "I would call the agent
"design-conformance-reviewer". I think it is clearer than yours, even if it does not covers goals
in the name (this is a trivial loss IMO)."

### Every check depends on a diligent record, and the retrospective judges where a miss belongs `##a20`

Round 2, the owner. Bears on #plan-read-against-the-record. Quoted under its thread.

### The owner cannot tell how many tripwires a premortem message holds `##a21`

Round 2, the owner. Bears on #premortem-items-numbered and #ruled-items-labelled. "it is often quite
hard already to rule on tripwires, because I'm never certain how many there are and what they are
about. … It would be convenient to have both tripwires and acceptance criterion be referred by a
numbered id there, to rule on them by reference, and to find them in the transcript more easily.
Maybe something like T<number> for tripwires … and AC<number> for acceptance criterion (two letters
to avoid conflict with checker findings of retrospective)."

### The name collides with the "conformance" axis `##a22`

Round 2, the agent, as a material finding on a closed thread. Bears on #plan-read-against-the-record
and #standing-state-axis-name. The review skill's axis table has `| conformance |
knowledge-architect-standing-state-reviewer |` and a row "spec conformity", so the table would hold
three "conformance" names for three reviewers.

### A label is no slug, and a cause label is one more number to read `##a23`

Round 2, the agent. Bears on #premortem-items-numbered and #goal-wording-real-use. On the goal's
redraft: "The harness exception stays in the head: it is a boundary of the decision, not part of
your intent." On labels: `AC1` does not fit the id grammar, so the
checker never reads it, and the plan item keeps a content slug. The retrospective intake uses `W<n>`
and `C<n>`, as in "now: W1, W5; issue: C3". The rival, labelling the causes, fails: "you rule on T
and AC, not on causes, so a cause label would be one more number to read."

### The premortem's tripwires, kept and dropped `##a24`

Round 2, the agent; ruled by the owner in round 3. Bears on #plan-read-against-the-record,
#synthetic-evidence-boundary and #synthetic-evidence-not-built. The agent proposed T1, T2 and T3.
The owner's words, round 3: "For the premortem: keep T1 only."

### Rename the row; the new name parallels "spec conformity" `##a25`

Round 3, the owner. Bears on #standing-state-axis-name and #plan-read-against-the-record. "I have no
idea why this row is called conformance. Rename it." And: "it fits with the "spec-conformity" one:
both check conformance of something against the first word in their name."

### A narrow wording reduces the intent behind it `##a26`

Round 3, the owner. Bears on #synthetic-evidence-boundary. "I'd rather have the head name a wider
category that matches my intent here. … It is important to be careful with narrow wordings,
because they reduce the underlying intent to something lesser."

### Labelling is a recurring pattern across the workflow, and earns a head `##a27`

Round 3, the owner. Bears on #ruled-items-labelled. "things that don't get a slug and need my
ruling should be getting a similar letter+number id by default. Questions in decision record get a
Q<number> id IIRC. This is getting quite a recurring pattern, maybe it should get its own design
head."

### A harness check's verdict may read text, as long as it reproduces cheaply `##a28`

Round 3, the owner. Bears on #synthetic-evidence-boundary, and withdrew the agent's round 2 clause
that a harness check reads only whether a mechanism fired. "The verdict of a harness mechanics test
might need to look at the text output IMO. I don't think this is a wrong pratice. The point is that
the verdict must be reliably reproducible and have a relatively low cost of re deriving by the same
"mock run"."

### The category is the property, not the subject `##a29`

Round 3, the agent. Bears on #synthetic-evidence-boundary and #c-checker-tests-untouched. The
owner's own test names the category: the verdict reproduces, and re-deriving it is cheap. It
covers the installer and the checker as well as the harness, and absorbs the checker's tests as a
case. "The pair "skill invoked by name" against "skill invoked from its description" marks the
border. The first is the harness resolving a name. The second is the agent choosing."

### The workflow already labels some items, and `P` is taken `##a30`

Round 3, the agent. Bears on #ruled-items-labelled. Entry test 4 uses `Q`; the retrospective uses
`W`, `C` and `P`. The structure passes `design@agent-skills@capabilities-not-structure` because
the owner named the lack, and the label's scope follows the record that carries it.

### A label that stays in the conversation reuses `Q`, since distinct letters run out `##a31`

Round 4, the owner. Bears on #ruled-items-labelled. Quoted under its thread.

## New names, in one place

```text
knowledge-architect-design-conformance-reviewer   the installed agent; source planned below
design conformance                                its row in the review skill's axis table
standing state                                    the renamed row of the standing-state reviewer
T<n>, AC<n>                                       labels of tripwires and acceptance criteria put to the owner
Q<n>                                              existing; the common label of an item whose label reaches no committed document
W<n>, C<n>, P<n>                                  existing; the retrospective's findings
```

The agent's source file is `planned@agent-skills@content/agents/design-conformance-reviewer.md`.
The build maps it to its installed copy under .claude/agents, with the installer's prefix.

## Decided design

### The goal

`goal@knowledge-architect@the-workflow-improves-through-real-use` reads, after step 1, as the owner
approved it in round 3:

> Real sessions are the test of the workflow. No session is built to observe how agents follow its
> instructions: not to originate an edit, to choose between shapes, or to accept a piece of work.
> The retrospective, and the owner's observations of real sessions, collect what was unclear,
> missing or wrong, and the owner decides what changes.

The heading of the goal is unchanged.

### The rule on synthetic evidence

The new head `synthetic-evidence-not-built`, in the agent-skills design home, under "How the
installed text changes", next to `design@agent-skills@additions-need-real-use`. Its content, as the
owner approved it in round 4:

> A run built to produce evidence is admitted only when its verdict reproduces: the same run,
> repeated, gives the same verdict, at a cost low enough to repeat it. The verdict may read text an
> agent produced, as long as it reproduces. What an agent decides in following the workflow's
> instructions does not meet this test: its verdict varies between runs, and it would mean
> something only under a statistical design. So no run is built to observe it, and an agent's
> decision is presumed not to reproduce. Showing that one does would take the repeated runs this
> rule refuses.
>
> - **Admitted:** the harness's mechanics, such as a skill loading when invoked by name, a hook
>   firing, a tool being granted, or a hook's injected text reaching the context. Also the
>   installer's and the checker's behaviour, tested over mock projects.
> - **Refused:** whether an agent invokes a skill from its description, whether a reviewer finds a
>   given entry, whether an agent follows a step.
> - **Always admitted as evidence:** a finding from a retrospective, and a misbehaviour observed in
>   real use.

The head also states the three purposes it covers (evidence between shapes, an acceptance check, a
check that a text runs as written), argues from `argument@synthetic-evidence-and-plan-reviews@a1`,
`argument@synthetic-evidence-and-plan-reviews@a2` and `argument@synthetic-evidence-and-plan-reviews@a5`,
records the two instances as its evidence, and cites
`goal@knowledge-architect@the-workflow-improves-through-real-use`. The sentence of
`design@agent-skills@additions-need-real-use`, "Published literature and synthetic scenarios
originate no edit", stays, and points to the new head.

Nearest rival: widening `design@agent-skills@additions-need-real-use` itself. Defeated by
`argument@synthetic-evidence-and-plan-reviews@a8`.

### Step 5 of the design skill

One sentence is added to loop step 5 of the installed design skill, after the list of artifacts,
as the owner approved it in round 2:

> Evidence that the project's goals or design heads rule out is not built, whatever the stall; the
> fork goes to the owner as a tie.

It names no project's rule, so it holds in any project. A `%%` comment above it may cite the new
head. Nearest rival: no sentence, relying on the outward review. Defeated by
`argument@synthetic-evidence-and-plan-reviews@a9`.

### The design-conformance reviewer

A new installed agent, `knowledge-architect-design-conformance-reviewer`, shaped like the other
plan-document reviewers (`path@agent-skills@content/agents/code-claims-reviewer.md` is the nearest
model): a frontmatter with `name`, `description` and `tools: Read, Grep, Glob, Bash`, a scope
paragraph, the invariants that the tree is established by the reviewer, that every assertion is
reproduced, and that the reviewer does not use `Write` or `Edit`, then its sections.

- **Its question**: does the plan document fit the project's record?
- **What it reads**: every section of the plan document, against the goals homes of the project's
  root and of every Component the document touches, those Components' design homes, and their
  rejected alternatives. It finds the Components from the manifest and from the anchors the
  document's references name. It uses `{{command}} show` to read an entry and its references.
- **What it reports**, each as a finding with the plan's passage and the record's passage quoted:
  - a shape that contradicts a goal;
  - a shape that contradicts a head, or widens one, which the document's "What is already decided"
    does not list as reversed or rewritten;
  - a shape that brings back a rejected alternative without a reopening that the document records;
  - an acceptance criterion whose observable a goal or a head rules out;
  - a default, a step of the implementation sequence, or a harvest row that does any of the above.
- **What it does not judge**: whether the document is sufficient to act on (the cold implementer),
  whether its claims about the code are true (code claims), whether it records the discussion
  (transcript), whether a criterion is well formed (the readiness checks).
- **Clause from the premortem, cause 1**: a widening of a head is reported only where "What is
  already decided" does not list the head; a contradiction of a goal is reported always.
- **Where its findings go**: §8's existing route. A material finding is answered with a default,
  under the defaults awaiting the owner. A conflict with a goal goes to the owner, per the primer,
  and is not resolved by the reviewer.

The installed planning skill's §8 gains a fourth bullet sending it, with the same brief invariants
as the others. The installed review skill's §1 gains a row, `design conformance`, applicable "the
same moment" as the code-claims row, and its row `conformance` is renamed `standing state`.

Nearest rivals, each defeated: extending the decision-record reviewer
(`argument@synthetic-evidence-and-plan-reviews@a11`); extending the design audit
(`argument@synthetic-evidence-and-plan-reviews@a12`); extending the cold implementer
(`argument@synthetic-evidence-and-plan-reviews@a13`). The agent's name was proposed as
`knowledge-architect-record-fit-reviewer`; the owner named it, round 2.

### Acceptance criteria on the owner's word

The shape mirrors `design@agent-skills@premortem-tripwires-on-the-owners-word` and
`design@agent-skills@tripwire-wording-is-the-agents`:

- The owner rules on whether a criterion is applied and on what its firing reopens. The agent words
  the observable, and a rewording of it is listed to the owner at the end of the turn, as a
  tripwire's is.
- The design skill's loop step 7 asks the owner, for each acceptance criterion as for each
  tripwire, whether it is kept.
- A criterion proposed after the premortem, at assembly, by a review or at an audit, is written
  under "Defaults awaiting the owner" with its label, and is not judged before the owner's ruling.
- **Clause from the premortem, cause 4**: an unruled criterion is not judged, and the work goes on
  without it.

The installed planning skill's §6 states this; the installed design skill's step 7 asks for the
ruling. Nearest rival: no word, relying on the new reviewer. Defeated by
`argument@synthetic-evidence-and-plan-reviews@a15`: the reviewer catches a conflict with the
record, not a weighing.

### Labels

The new head `ruled-items-labelled`, as approved in round 4:

> Every item the owner is asked to rule on by name, and which carries no slug, gets a label: a
> prefix of capital letters naming its kind, and a number. Numbers run from 1 within each prefix,
> in order of appearance, and are never reused within the record that carries the ruling: a
> discussion (a resumed session included), a file, or a message. A label that reaches a committed
> document, even one that leaves later, such as a plan document, takes a prefix of its own kind,
> distinct across the installed workflow, and the head lists every such prefix. A label used only
> in the conversation reuses the common prefix `Q`.

| prefix | items | reaches a committed document |
| --- | --- | --- |
| `T` | tripwires put to the owner | yes: the plan's Premortem section |
| `AC` | acceptance criteria put to the owner | yes: the plan's Premortem and defaults |
| `W`, `C`, `P` | retrospective findings | yes: the retrospective files |
| `Q` | entry test 4 questions, and any label that stays in the conversation | no |

- **Clause from the premortem, cause 5**: the sequence continues across a resumed session.
- **Clause from the premortem, cause 6**: the head lists every distinct prefix, and an edit that
  adds one checks it against the list.
- Each installed skill that asks for such rulings restates its own prefix where it asks, with a
  `%%` comment citing the head.
- The plan document's Premortem section writes each label beside its verdict, so a reader goes from
  a ruling in the transcript to the plan.

**The sweep.** The installed files that ask for the owner's word are swept at step 3, for the sites
that ask about an item ruled by name and carry no label. `grep -rl -E "owner's word|owner rules|ask the owner"` over `path@agent-skills@content/` lists 12
files, the primer included. Each site found gets a label:
a prefix of the table above where one fits, `Q` where the label stays in the conversation, or a new
distinct prefix added to the head's table. If the sweep finds more sites than step 3 can carry, the
remainder is opened as a `todo` issue in the agent-skills register, as the owner allowed in round 4.

Nearest rival: a distinct prefix for every label. Defeated by
`argument@synthetic-evidence-and-plan-reviews@a31`.

## Mapping tables

None. The work needs no total function over existing things.

## Losing alternatives

- **A definition of a synthetic run by its subject**, harness mechanics excepted by name. Lost to
  #synthetic-evidence-boundary in its final form: the owner judged it narrow,
  `argument@synthetic-evidence-and-plan-reviews@a26`, and the property covers the installer and the
  checker as well, `argument@synthetic-evidence-and-plan-reviews@a29`.
- **A harness check reads only whether a mechanism fired, never the content of the text produced.**
  The agent's clause from round 2, withdrawn in round 3 on
  `argument@synthetic-evidence-and-plan-reviews@a28`: what matters is that the verdict reproduces.
- **Widening `design@agent-skills@additions-need-real-use`** in place of a new head. Lost to
  #synthetic-evidence-head on `argument@synthetic-evidence-and-plan-reviews@a8`.
- **No sentence in step 5.** Lost to #step-5-evidence-the-record-admits on
  `argument@synthetic-evidence-and-plan-reviews@a9`.
- **Extending the decision-record reviewer to plan documents.** Lost to
  #plan-read-against-the-record on `argument@synthetic-evidence-and-plan-reviews@a11`.
- **Extending the design audit to read the goals.** Lost to #plan-read-against-the-record on
  `argument@synthetic-evidence-and-plan-reviews@a12`.
- **Extending the cold implementer.** Lost to #plan-read-against-the-record on
  `argument@synthetic-evidence-and-plan-reviews@a13`.
- **A separate axis for acceptance criteria, judging relevance and coverage**, the thread
  #acceptance-criteria-axis. Superseded by #plan-read-against-the-record on
  `argument@synthetic-evidence-and-plan-reviews@a16`.
- **An acceptance criterion with no word of the owner, relying on the reviewer.** Lost to
  #acceptance-criteria-on-the-owners-word on `argument@synthetic-evidence-and-plan-reviews@a15`.
- **Labels on tripwires and acceptance criteria only**, the thread #premortem-items-numbered.
  Superseded by #ruled-items-labelled, on the owner's widening, `argument@synthetic-evidence-and-plan-reviews@a27`.
- **Labels on the premortem's causes.** Lost on `argument@synthetic-evidence-and-plan-reviews@a23`.
- **A distinct prefix for every label.** Lost on `argument@synthetic-evidence-and-plan-reviews@a31`.
- **Renaming the slug `conformance-before-every-merge`.** Lost to the agent's default in round 3:
  three citations repaired for no reader's gain.

## Readings

None. The work reads no external specification.

## Premortem

Assume the work shipped and failed. The causes, as presented in the agent's reply to round 2 and
revised in its reply to round 3:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. The new reviewer floods the owner: every plan widens some head by design | #plan-read-against-the-record | converted into a clause of the design ("The design-conformance reviewer"), and becomes the tripwire **T1** on the owner's word, round 3, "keep T1 only" |
| 2. "Harness mechanics" is stretched to cover an agent's decision | #synthetic-evidence-boundary | converted into a clause: an agent's decision is presumed not to reproduce. The tripwire **T2** was dropped by the owner, round 3 |
| 3. A defect ships that a synthetic run would have caught | #synthetic-evidence-not-built | the tripwire **T3** was dropped by the owner, round 3 |
| 4. Work stalls on unruled criteria | #acceptance-criteria-on-the-owners-word | converted into a clause: an unruled criterion is not judged, and the work goes on |
| 5. Labels collide when a resumed session restarts at 1 | #ruled-items-labelled | converted into a clause: the sequence continues across a resumed session |
| 6. A later skill adds a prefix that collides with one in use | #ruled-items-labelled | converted into a clause: the head lists every distinct prefix |

T1, as the harvest writes it: fires when, in two plan reviews in a row, the owner rules more than
half of the design-conformance reviewer's findings needless. The count, two reviews, and the
proportion, half, are defaults marked as the owner's to reset. Response: a `design` issue that
reopens the reviewer's scope. Re-entry: the retrospective of the session where it fires.

## Acceptance criteria

None. Every cause of the premortem is converted into a clause, kept as T1, or dropped by the owner.
A check of the new reviewer on a plan document built for the purpose would be a synthetic run, which
#synthetic-evidence-not-built refuses; its first real dispatch is at the next plan document.

## Implementation sequence

Each commit that changes `path@agent-skills@content/` runs `cargo klarch install-agent-skills` and
holds the installed copies, per `path@agent-skills@CLAUDE.md`, and passes the tests of that file's
section "Editing an installed skill or agent". Each change to the installed text is a decision
under `knowledge-architect-agent-configuration` §1: the decisions are the approved threads of this
spec, and step 5 records them. Installed text has no claims in the project's development
procedure, so each step states its own claim and check.

1. **The goal and the evidence rule.** The goal reworded, per "The goal". The sentence of step 5,
   per "Step 5 of the design skill". Claims and checks: the goal reads as the owner approved it, by
   a reading; each text of the table under "What is already decided" judged at step 1 stays true,
   by a reading of each; `cargo klarch check` passes. Fails alone on: a sentence that the widened
   goal makes false.
2. **The design-conformance reviewer.** The agent file, its installed copy, the fourth bullet of the
   planning skill's §8, and the two rows of the review skill's §1, per "The design-conformance
   reviewer". Claims and checks: `cargo build` passes, so the build maps the file; `cargo klarch
   check` passes with the installed copy committed; the agent text cites no entry and writes the
   command as `{{command}}`, by a reading; no row of the review skill's table is named
   "conformance", by `grep -n '^| conformance' crates/agent-skills/content/skills/review/SKILL.md`
   returning nothing. Fails alone on: the build, the check, or a row still named "conformance".
3. **Acceptance criteria on the owner's word, and labels.** The design skill's step 7 and the
   planning skill's §6, per "Acceptance criteria on the owner's word" and "Labels"; then the sweep
   of "Labels", each site found given its label. Claims and checks: step 7 asks for a ruling on each
   acceptance criterion and labels each tripwire and criterion it puts to the owner, by a reading;
   §6 states that an unruled criterion is a default and is not judged, by a reading; every site the
   sweep lists has a label or is named in the `todo` issue, by a reading of the sweep's list. Fails
   alone on: a site the sweep lists with neither.
4. **The changelog.** Entries in the `Next release` section of CHANGELOG.md, then `cargo x
   changelog`. Under Workflow, each `agent-skills`, patch: a new reviewer reads a plan document
   against the goals, the design heads and the rejected alternatives at every §8 moment; the owner
   rules on each acceptance criterion, and a criterion proposed after the premortem waits as a
   default; items put to the owner by name carry labels; the design skill builds no evidence the
   project's goals or heads rule out. Fails alone on: a changelog copy that differs. The review of
   §7 point 4 runs after this step.
5. **The harvest**, below, its review, and the deletion of this spec, in one commit.

## Order rationale

Step 1 before step 2: the reviewer reads the goals, and the goal it most needs is the reworded one.
Step 2 before step 3: step 3's planning skill text names the defaults that the reviewer's findings
also produce, and either order passes; the order lands the reviewer first. Step 3 before step 4:
the changelog describes the changes once all are built. Step 4 before step 5: the harvest records
the decisions the changelog's work implements.

## Defaults awaiting the owner

None at the writing of this spec.

## Harvest

At step 5, under `knowledge-architect-decision-recording` and `knowledge-architect-issue-tracking`.
The recording tests decide whether each decision and each alternative earns an entry; an item of
this row they exclude is named in the harvest's commit, with the test it fails.

| item | home |
| --- | --- |
| #synthetic-evidence-not-built, with #synthetic-evidence-boundary and #synthetic-evidence-head | a new head in `path@agent-skills@docs/design.md`, slug `synthetic-evidence-not-built`, under "How the installed text changes"; `design@agent-skills@additions-need-real-use` rewritten in place to point to it |
| #plan-read-against-the-record | a new head in `path@agent-skills@docs/design.md`, slug `plan-read-against-the-record`, under "Reviews" |
| #acceptance-criteria-on-the-owners-word | a new head in `path@agent-skills@docs/design.md`, slug `acceptance-criteria-on-the-owners-word`, under "Plan documents" |
| #ruled-items-labelled | a new head in `path@agent-skills@docs/design.md`, slug `ruled-items-labelled`, under "The workflow the skills carry" |
| #standing-state-axis-name, #step-5-evidence-the-record-admits | no head expected: a renamed label, and a restatement of `synthetic-evidence-not-built` |
| #goal-wording-real-use | done at step 1, in the goals home |
| every item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit |
| T1 | `path@agent-skills@docs/tripwires.md`, guarding the head `plan-read-against-the-record`, as the Premortem section words it |
| this spec | deleted in the harvest's commit, cited as `spec@plans@synthetic-evidence-and-plan-reviews` |

## Later consequences

- The first plan document written after this work lands is the first real dispatch of the
  design-conformance reviewer, and its review is where T1 starts counting.
- `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` may reuse the new reviewer's reading
  of goals and heads for its "goal coverage" audit at the scale of a whole project.
- A project's own skills that ask the owner for rulings by name follow the head
  `ruled-items-labelled` only once its owner restates it there; the installed text reaches only the
  installed skills.
