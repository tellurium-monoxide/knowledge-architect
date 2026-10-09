---
kind: todo
---
# The workflow offers no way to audit a project as a whole

## Summary

Every review the workflow offers reads a diff: the work of one branch, or one plan document. Nothing
reads a project's record as a whole: its design homes against each other, against the code and the
goals, or the project's own agent configuration against the installed workflow. An audit of every
design head of this repository, run during the discussion that rewrote the entry tests, was useful
and was built by hand. The owner: "The audit we did is probably something that is useful to run once
in a while in any large project." The owner proposes a name that keeps such audits apart from
reviews, and a later session under `skill@knowledge-architect-design` to design them.
`spec@plans@design-record-audit` schedules the method and the first axis, the conformance of the
decision record; this entry keeps the other axes, the question of an audit report, and the lessons
on method.

## Details

### What

The owner's statement of the gap: "the project does not offer a way to review a consumer project
globally". And: "Reviews are only offered on "diffs", the work of one branch or one document." The
owner
proposes the name **project audits** rather than reviews, to avoid confusion with the installed
review skill, `skill@knowledge-architect-review`, and its reviewer agents; the name is open to
discussion.

**The audit that was run, as a worked instance.** In the discussion that produced the milestone
load-bearing-records, the owner proposed "an audit of all decisions recorded in the project against
the three existing tests of the decision record skill, and also against the locality test you
proposed", carried "by one or several subagents (maybe one per component)". As run:

- Four read-only subagents, one per group of Components (agent-skills; core; the root; gates and
  xtask, the smallest two together), each read every head of its design homes in full and the code
  it governs, and gave each head two verdicts: under the entry tests then installed and under the
  tests the discussion proposed, naming the test that passed or `none`, with a one-line reason. Each
  wrote a table to a scratch file, not to the conversation: 178 heads in all.
- Before it ran, the session and the owner fixed pre-commitments: which outcome would show
  over-recording, which would show it rare, and which would show a test wrong. The threshold, one
  head in five, was the session's proposal, presumed approved.
- The owner read a sample: every `none` verdict, five random `passes` per group, drawn with a stated
  seed, and passes the owner picked. The owner's disagreements were the valuable output: a head the
  tests admitted and the owner judged unneeded showed a test too permissive, and the test was
  refined.
- The agents' own reports surfaced what the tests left ambiguous: two readings of a test no one had
  ruled on, a test that "passes nearly everything", a test that does not discriminate.
- The follow-up, a cleanup, judged each flagged head in full before deleting anything, and showed
  the owner the verdicts first. Its review found that the session had judged one criterion, a
  ruling of the owner, from the heads' text alone, when the rulings are recorded in history: commit
  messages and deleted plan documents. A second subagent then read the history behind each head.
  This led the owner to a new entry test, recorded in
  `design@agent-skills@a-head-is-owed-by-an-entry-test`.

**Axes the owner names**, a list to be completed and refined:

- **decision-record volume**: every head against the entry tests, as run above; scheduled by
  `spec@plans@design-record-audit` as part of its design-record axis;
- **design self-consistency**, of the design homes as a whole rather than of one branch;
- **alignment of implementation with design and goals**: the code against the design homes, and
  both against the goals;
- **consistency of the project's own agent workflows**, its CLAUDE.md files and skills, combined
  with the workflow the project installs.

Additions of the session, for the discussion to judge:

- **standing state as a whole**: every issue and tripwire read against the tree as it stands, for
  an issue whose diagnosis has gone stale, a tripwire whose decision has changed under it, or a
  deferred trigger met without anyone noticing; the standing-state reviewer does this only for the
  entries a diff bears on;
- **goal coverage**: each goal against the design heads that serve it, for a goal no head serves and
  a head that serves no goal. The installed design-conformance reviewer already reads one plan
  document against the goals and the heads, per `design@agent-skills@plan-read-against-the-record`;
  its reading is a lead for this audit at the scale of the whole project;
- **restatements**: every restatement of a directive against its home, since a restatement is the
  defect where the two disagree and the checker cannot compare prose.

**A second worked instance.** The design discussion that produced
`design@agent-skills@title-states-the-rule` audited every head of this repository's five design
homes, 189, and 219 heads of thaum, read-only, one subagent per group of design homes, on three
axes: a head stating the instance built as its rule, a head bundling several decisions, and a head
citing an approval of the owner as its ground. Its record is the plan document head-rules, under
"What the audits measured", deleted at the landing of its work; `git log --diff-filter=D` on the
plans directory finds it. Two lessons on method:

- **A test given to the auditors can be wrong, and the auditors find it.** The split test briefed
  to the agents, "would reversing one part leave the other standing?", over-split a rule from its
  exception; four audits independently reported a better signal, parts that lose to different
  nearest rivals, which the discussion adopted.
- **Agents dispatched together are not calibrated alike.** One audit reported that its own
  subagents classed a ground taken from a goal differently and used different thresholds for a
  rewritten head, so its weekly counts carry that uncertainty. An audit that compares figures across
  agents owes a calibration: a shared sample classed by each.

**Scale.** Such audits must work on large projects, so they rest on subagents: groups of entries
of bounded size, as `design@agent-skills@standing-entry-search-groups` sizes its groups, each read
whole, with results written to files rather than returned into the dispatching conversation.
A harness's dynamic workflows could orchestrate them, but that is specific to one harness, against
`goal@agent-skills@installed-text-works-anywhere`. What the instance above suggests an audit owes:
pre-commitments stated before it runs; the owner's reading of a sample, since agents applying a
test measure how agents read it, not whether it is right; history read as well as text, wherever a
verdict rests on what was ruled; and a follow-up that changes nothing before the owner has seen
the verdicts.

### Why it matters

`goal@knowledge-architect@documentation-stays-consistent` is met while the reviews keep the record
consistent, and every review reads only what a diff touches: a drift between two heads that no
branch touches, or between the code and a head no branch cites, is read by no step of the workflow.
`goal@knowledge-architect@agents-work-without-drift` depends on the record being right as a whole,
since agents ground on all of it. The audit above found over-recording, a test that admitted
nearly everything, and two heads in the wrong Component, none of which a diff review had reported.

### What would close it

For each axis above other than decision-record volume, a design discussion under
`skill@knowledge-architect-design` that decides whether it is offered, its corpus and its outcomes,
and, if it is, its agent and its section of the audit skill that `spec@plans@design-record-audit`
builds; and for an axis whose fixes are code, whether it leaves an audit report. The owner left the
report "a question for later".
