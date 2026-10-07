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
reviews, and a later session under `knowledge-architect-design` to design them.

## Details

### What

The owner's statement of the gap: "the project does not offer a way to review a consumer project
globally". And: "Reviews are only offered on "diffs", the work of one branch or one document." The
owner
proposes the name **project audits** rather than reviews, to avoid confusion with the installed
review skill, `knowledge-architect-review`, and its reviewer agents; the name is open to
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

- **decision-record volume**: every head against the entry tests, as run above;
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
  a head that serves no goal;
- **restatements**: every restatement of a directive against its home, since a restatement is the
  defect where the two disagree and the checker cannot compare prose.

**Scale.** Such audits must work on large projects, so they rest on subagents: groups of entries
of bounded size, as `design@agent-skills@standing-entry-search-agent` sizes its groups, each read
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

A design discussion under `knowledge-architect-design` that decides whether project audits are
offered, under which name, which axes, how they are dispatched at scale, and what they leave behind;
and, if they are, the installed skill or skills and agents built from it.
