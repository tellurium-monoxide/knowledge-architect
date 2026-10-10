---
kind: todo
---
# The project audit has one axis, and the other axes the owner named have none

## Summary

`skill@knowledge-architect-project-audit` audits a whole project on one axis, the design record,
per `design@agent-skills@audit-is-an-activity`. The other axes the owner named have no section, no
agent and no designed outcome: design self-consistency, the alignment of the code with the design
and the goals, and the consistency of the project's own agent workflows; and the session's
additions, standing state as a whole, goal coverage and the restatements. An axis whose fixes are
code may also need an output other than commit messages, an audit report, which the owner left "a
question for later". This entry keeps those axes, the report question, and the lessons of the
audits run by hand before the skill existed.

## Details

### What

The owner's statement of the gap, before the audit skill existed: "the project does not offer a way
to review a consumer project globally". The owner named such audits **project audits**, apart from
reviews.

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

**Axes the owner names**, a list to be completed and refined, the design-record axis aside:

- **design self-consistency**, of the design homes as a whole rather than of one branch;
- **alignment of implementation with design and goals**: the code against the design homes, and
  both against the goals;
- **consistency of the project's own agent workflows**, its CLAUDE.md files and skills, combined
  with the workflow the project installs. A lead from the owner for what it reads: an audit and a review that
  share a subject are given compatible instructions for the verdicts they give. A comparison of the two
  agents, made on that request after the first run, found
  `agent@knowledge-architect-design-record-auditor` and
  `agent@knowledge-architect-decision-record-reviewer` judging rejected alternatives by two
  different restatements of one rule, one of which reported as a defect what the other must not;
  the commit "The decision-record reviewer and the design-record auditor judge the record by the
  same rules" aligned them. The owner placed the restatements of directives in this axis, each read
  against its home, since the checker cannot compare prose: "The restatement item, I find it weird
  to add it in this axis. It does not read the same thing at all. It would go in the "agentic
  workflow" axis mentionned in the issue, instead, IMO."

Additions of the session, for the discussion to judge:

- **standing state as a whole**: every issue and tripwire read against the tree as it stands, for
  an issue whose diagnosis has gone stale, a tripwire whose decision has changed under it, or a
  deferred trigger met without anyone noticing; the standing-state reviewer does this only for the
  entries a diff bears on;
- **goal coverage**: each goal against the design heads that serve it, for a goal no head serves and
  a head that serves no goal. The installed design-conformance reviewer already reads one plan
  document against the goals and the heads, per `design@agent-skills@plan-read-against-the-record`;
  its reading is a lead for this audit at the scale of the whole project.

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
  rewritten head, so its weekly counts carry that uncertainty.

**Method.** The method these lessons led to is `design@agent-skills@audit-method`, approved for
the design-record axis and as the generic shape to adjust for the others: "it might differ per
axis. For now, we are only providing one axis, so we can adjust this generic shape later for the
others. But I think your general shape should hold its ground for the other axis too." It holds
bounded groups, a calibration sample, history read behind each
ruling, and nothing applied before a reading confirms it. One lesson it does not adopt stays open
here, for an axis to weigh: the owner's reading of a sample of the verdicts, since agents applying a
test measure how agents read it, not whether it is right.

### Why it matters

`goal@knowledge-architect@documentation-stays-consistent` is met while the reviews keep the record
consistent, and every review reads only what a diff touches. The design-record axis reads the
decision record as a whole against the rules on its entries; a drift between two heads that no
branch touches, between the code and a head no branch cites, between two skills, or between an
issue and the tree as it stands, is still read by no step of the workflow.
`goal@knowledge-architect@agents-work-without-drift` depends on the whole record, since agents
ground on all of it.

### What would close it

For each axis above, a design discussion under `skill@knowledge-architect-design` that decides
whether it is offered, its corpus and its outcomes, and, if it is, its agent and its section of
the audit skill, per `design@agent-skills@audit-agent-per-axis`; and, for an axis whose fixes are
code, whether it leaves an audit report.
