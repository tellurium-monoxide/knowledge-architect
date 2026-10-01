# The knowledge-architect workflow

This project keeps its design record with knowledge-architect. Its documents are checked by
`{{command}} check`: every reference between records resolves, every record has the shape its
register declares, and every generated listing is current. This primer is installed, and the
project's root `CLAUDE.md` imports it. It holds what every session needs and no installed skill
delivers. The project's own rules are in its root `CLAUDE.md`, beside this primer, and add to it.

## Goals bind; decisions bind as a presumption

**The goals are the only statements assumed to come from the owner.** Each Component states them
in its `docs/goals.md`, one heading per goal. A recorded decision was reviewed, but its review can
miss a detail or an implication, more often as the volume of agentic work grows. A decision binds
as a presumption, which a better argument may rebut, and reversing one is an ordinary move with a
procedure (`knowledge-architect-recording-a-decision`). **When a decision conflicts with a goal,
the likely cause is that the owner missed the conflict: the goal prevails, and the conflict goes to
the owner.** It is not resolved by following the decision.

## Intent and claims

- **A design home is intent.** Check the code against it, never the other way. A divergence is a
  defect in one of them: say which, open an issue, and stop.
- **A claim about the code as it stands** (a scoped `CLAUDE.md` invariant, a doc comment, a name, an
  issue's diagnosis) goes stale. Verify it against the code, or against a run you observe, before
  relying on it.
- **Before diagnosing anything as a problem, check whether it is already recorded:**
  `{{command}} issues`, `{{command}} tripwires`, and `{{command}} show <kind>@<anchor>@<id>` for one
  entry and every reference to it.

## Something met outside the task

Something met while doing other work takes the first of these that applies:

| # | test | outcome |
| --- | --- | --- |
| 1 | it bears on the current work: the work's result, or a decision it rests on, is wrong or incomplete without it | stop and present it to the owner at the top of the turn, with a default |
| 2 | its fix is checkable from the diff alone: it changes no behaviour, no decision and no test outcome (a typo, a stale pointer, wording that is now false, a broken link) | fix it, in a commit of its own |
| 3 | its `Why it matters` and its `What would close it` can be written | open an issue entry (`knowledge-architect-tracking-open-issues`) |
| 4 | none of the above | name it, with why it is dropped |

**A turn that met anything outside its task ends with a section titled "Met outside the task"**,
listing every item with its outcome: fixed (with the commit), issue opened (with its id), waiting
for the owner's ruling, or dropped (with the reason). A mention inside other prose does not count.

## Where knowledge goes

**Every durable decision has exactly one home.** A second mention of a decision is a reference to
it, never a copy, because a copy starts drifting the moment it is written. **A directive is
different**: it is restated wherever it has to be delivered, with a reference to its home beside
it, and where the two disagree the restatement is the defect.

| the statement is about | home | it leaves when |
| --- | --- | --- |
| what the project, or one Component, is for | that Component's `docs/goals.md` (the project's root is a Component) | the owner abandons the goal |
| how the project or a Component is built, and why | that Component's design home, `docs/design.md` or `docs/design/` | the design changes: the entry is rewritten in place |
| an alternative that lost, and why | that Component's `docs/rejected-alternatives.md` | never; a reversal moves the old winner into it |
| what is outstanding: a defect, an unexplained observation, an open question, missing work | one file in the owning anchor's issue directory, `docs/open-issues/` in a Component | the issue closes |
| evidence that would flip a recorded decision about code that exists | the owning Component's `docs/tripwires.md` | it fires, or its decision is gone |
| a contract or a trap a developer needs, true of the code as it stands | the scoped `CLAUDE.md` nearest the code | the contract changes |
| how a user uses a Component | its `README.md` | the contract changes |
| what a caller must respect to use a type or a function | that item's doc comment | its contract changes |
| why a piece of code is shaped the way it is | an inline comment at that code | that code changes |
| work that is designed and not built: a spec or a milestone | the project's plans directory, named in its root `CLAUDE.md` | the work lands |
| how to perform an activity | the owning skill | the procedure changes |
| **none of these, nor a row of the project's own** | **ask the owner before writing it anywhere** | the table gains the row |

The project's root `CLAUDE.md` adds its own rows. **A measurement is routed by what it serves**:
the head of the decision it supports, the entry of the defect it characterises, the tripwire whose
threshold it is, or else the commit message that took it.

**A reference is written wherever the text would have to be revisited if the entry it names
changed**: a design head names the goal it derives from, an issue names the decision it strains, a
code comment names the issue it exists because of, a tripwire names its decision, a commit message
names every entry it opens, closes or reverses. A reference is one backticked span,
`<kind>@<anchor>@<id>`; an illustration that must not resolve writes a placeholder in angle
brackets. The checker reads Markdown and Rust source; a reference anywhere else is found by grep.

## The installed skills

- `knowledge-architect-recording-a-decision`: a design decision has been made or reversed.
- `knowledge-architect-tracking-open-issues`: before diagnosing a problem; parking anything; a
  tripwire fires; work closes an entry.
- `knowledge-architect-planning`: a design discussion converged; a step of a milestone starts or
  lands.
- `knowledge-architect-dispatching-a-review`: before merging to the main branch, or when an
  activity's skill says its work is ready.
- `knowledge-architect-maintaining-agent-config`: before editing a `CLAUDE.md`, a skill or an agent.
- `knowledge-architect-setting-up`: the project adopts the workflow, or a version upgrade is
  installed.

A project's own skills add to these, and never replace them. Which project skill adds to which
installed one is the routing table of the project's root `CLAUDE.md`. Read the installed skill and
every skill the table lists beside it.
