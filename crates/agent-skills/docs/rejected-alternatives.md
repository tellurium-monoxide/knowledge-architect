# Rejected alternatives — agent skills

The alternatives that lost to a decision of `path@agent-skills@docs/design.md`, or to a goal where
the owner ruled that the winning position needs no entry, each with what it lost to and why.

**Recording every losing thread of a design discussion** — lost to
`design@agent-skills@losing-alternatives-filter`. `live`. The rejected alternatives would grow by
every thread of every discussion, including proposals nobody would raise again, and a reader would
search through them to find the few whose refutation cannot be derived again in one round. It is
kept here because a doubt remains: the owner judges the filter "still a little imperfect", and
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks for the alternatives that lost
and why, which the filter records only in part.

**A spec plus a separate detailed implementation plan, each step written out with its code** — lost
to `design@agent-skills@spec-and-milestone`. `live`. Both documents carry the same decisions and the
second drifts from the first. Its refutation rests on what the owner observed over real sessions:
implementers force such a plan's untested snippets into the code at any cost and copy their
comments verbatim. That observation cannot be derived again in one discussion round, and workflows
that write such plans are in common use, so the alternative will be proposed again.

**Reporting a finding met outside the task without recording it** ("finish the task, then say what
you found") — lost to `design@agent-skills@primer-content`. `live`. The owner observed, over real
sessions, that an agent left a small defect neither fixed nor recorded, as a one-line mention inside
a long report, where it is easy to miss. That observation cannot be derived again in one discussion
round, and the rule it defeats is the common one.

**Writing the design skill so that weaker models can follow it** — lost to
`design@agent-skills@frontier-tier-only`. `live`. Rejected as a design goal: the workflow uses
weaker models as implementers conducted by a frontier model, not as the owner's counterpart in a
discussion. The scripted comparison that entry cites, which cannot be derived again in one
discussion round, showed a smaller model reproducing the format without the discipline.
Simplifying for a cheaper model is a change that will be proposed again.

**A delegated state, under which the owner hands a decision to the agent within a stated boundary,
or a note recording that judgement was handed over** — lost to
`goal@knowledge-architect@the-owner-decides`. `live`. A word that hands judgement over is an
approval like any other, as is approving every default on a quick read. The state was decided and
reversed in one revision of designing-together, after five of that revision's eight blocking review
findings fell on it: inside a grant, a correction whose own action could not be undone ran with no
word from the owner.

**Writing the discussion's ledger to a file during the discussion, so it survives compaction or a
new session** — lost to `design@agent-skills@ledger-from-transcript`. `live`. An artifact an agent
must update every round is one it forgets to update, and a stale ledger stated with confidence is
worse than none. Written with the least effort, as a draft, it answers the first reason only. A survey of agent systems that keep state files found two
properties a thread ledger lacks: the state is corroborated against something outside the model,
such as git, and every system that lets a model overwrite its state caps it hard. A thread ledger's
only corroborant is the conversation, which is what compaction removes. In the runs of the
designing-together skill, thread states were correct without a file.

**No roadmap, with known undesigned work listed only as issues** — lost to
`design@agent-skills@roadmap-orders-issues`. `live`. It held that a roadmap file would be a second
schedule beside the issue register, and two schedules drift; it accepted as its cost that the order
of future work had no home. Two facts defeated it. thaum, the first project to use the workflow,
kept a file of its next milestones as an exception to the installed skill: a need for order
observed in real use, which cannot be derived again in one discussion round. And a roadmap whose
rows are only checked references is no second schedule: the work stays in the issue register, and
a row dangles, reported by the check, when its work closes.

**A separate installed file holding the expectation sets, read only by those who judge findings** —
lost to `design@agent-skills@expectation-set-bounds-scope`. `live`. It would keep the sets out of
every skill's text and out of the retrospective, at the cost of a change to the install layout for
a file one skill reads. It wins if a second installed activity ever needs the sets, which
`issue@agent-skills@expectation-sets-for-the-installed-skills` may bring about.

**A transcript reviewer that reports every clause added inside a ruling, as the agent's addition
for the owner to contest** — lost to `design@agent-skills@transcript-reviewer-agent`. `live`. It
protected the owner's rulings down to their wording, at the cost of a list of additions in every
review. The owner observed over real sessions that the list buried what mattered, a decision or a
finding that no document or issue had kept, and ruled that a detail or a better wording inside a
ruling needs no report.

**A milestone document holding the design of every step, with step specs holding only their entry**
— lost to `design@agent-skills@milestone-is-a-directory`. `live`. It kept the whole design in one
place to read. It lost to a measurement: the first milestone written that way, structured-plans,
had a README of 1,624 lines when it left, read whole at the grounding of every step. That cannot be
derived again in one discussion round.
