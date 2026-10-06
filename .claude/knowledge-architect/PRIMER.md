# The knowledge-architect workflow

This project keeps its design record with knowledge-architect. Its documents are checked by
`cargo klarch check`: every reference between records resolves, every record has the shape its
register declares, and every generated listing is current. This primer is installed, and the
project's root `CLAUDE.md` imports it. It holds what every session needs and no installed skill
delivers. The project's own rules are in its root `CLAUDE.md`, beside this primer, and add to it.

## Goals bind; decisions bind as a presumption

**The goals are the only statements assumed to come from the owner.** Each Component states them
in its goals home, `docs/goals.md` or `docs/goals/`, one heading per goal. A recorded decision was reviewed, but its review can
miss a detail or an implication, more often as the volume of agentic work grows. A decision binds
as a presumption, which a better argument may rebut, and reversing one is an ordinary move with a
procedure (`knowledge-architect-decision-recording`). **When a decision conflicts with a goal,
the likely cause is that the owner missed the conflict: the goal prevails, and the conflict goes to
the owner.** It is not resolved by following the decision.

## Intent and claims

- **A design home is built intent**: the design as built and its reasons, and the decisions that
  no work implements, recorded when made. Design that is decided and not built is in a plan
  document until it lands. Check the code against a design home, never the other way. A divergence
  is a defect in one of them: say which, open an issue, and stop. A design home can be wrong, and
  it still prevails over the code until the issue closes. It closes when the code changes to meet
  the head, or when the head is reversed under `knowledge-architect-decision-recording`; it never
  closes by following the code. Work that goes on meanwhile, on the owner's word, builds on the
  head.
- **A claim about the code as it stands** (a scoped `CLAUDE.md` invariant, a doc comment, a name, an
  issue's diagnosis) goes stale. Verify it against the code, or against a run you observe, before
  relying on it.
- **Before diagnosing anything as a problem, check whether it is already recorded:**
  `cargo klarch issues`, `cargo klarch tripwires`, and `cargo klarch show <kind>@<anchor>@<id>` for one
  entry and every reference to it.

## Something met outside the task

Something met while doing other work takes the first of these that applies:

| # | test | outcome |
| --- | --- | --- |
| 1 | it bears on the current work: the work's result, or a decision it rests on, is wrong or incomplete without it | stop and present it to the owner at the top of the turn, with a default |
| 2 | its fix is checkable from the diff alone: it changes no behaviour, no decision and no test outcome (a typo, a stale pointer, wording that is now false, a broken link) | fix it, in a commit of its own |
| 3 | its `Why it matters` and its `What would close it` can be written | open an issue entry (`knowledge-architect-issue-tracking`) |
| 4 | none of the above | name it, with why it is dropped |

**A turn that met anything outside its task ends with a section titled "Met outside the task"**,
listing every item with its outcome: fixed (with the commit), issue opened (with its id), waiting
for the owner's ruling, or dropped (with the reason). A mention inside other prose does not count.

## Where knowledge goes

**Every durable decision has exactly one home.** A second mention of a decision is a reference to
it, never a copy, because a copy starts drifting the moment it is written. **A directive is
different**: it is restated wherever it has to be delivered, with a reference to its home beside
it, and where the two disagree the restatement is the defect. A restatement is never replaced by a
reference on one-home grounds: whether a directive is needed where it is restated is the owner's
decision.

| the statement is about | home | it leaves when |
| --- | --- | --- |
| what the project, or one Component, is for, and what would show it achieved | that Component's goals home, `docs/goals.md` or `docs/goals/` (the project's root is a Component) | the owner abandons the goal |
| how the project or a Component is built, and why | that Component's design home, `docs/design.md` or `docs/design/` | the design changes: the entry is rewritten in place |
| the engineering alternative that lost, and why | that Component's `docs/rejected-alternatives.md` | never; a reversal moves the old winner into it if it meets a recording test of `knowledge-architect-decision-recording` |
| what is outstanding: a defect, an unexplained observation, an open question, missing work | one file in the owning anchor's issue directory, `docs/open-issues/` in a Component | the issue closes |
| evidence that would flip a recorded decision about code that exists | the owning Component's tripwires home, `docs/tripwires.md` or `docs/tripwires/` | it fires, or its decision is gone |
| a contract or a trap that only a developer needs, true of the code as it stands | the scoped `CLAUDE.md` nearest the code | the contract changes or the trap is removed |
| how a user can use a Component, and what to respect | its `README.md` | the contract changes |
| directions about what to find where in a directory | a `README.md` in that directory | the directory's content changes |
| what a caller must respect to use a type or a function | that item's doc comment | its contract changes |
| why a piece of code is shaped the way it is, and where that holds | an inline comment at that code | that code changes |
| work that is designed and not built: a spec or a milestone | the plans directory, docs/plans/ at the project's root: a spec in docs/plans/specs/, a milestone in docs/plans/milestones/ | the work lands |
| the order in which the owner wants known work done | docs/roadmap.md at the project's root, optional: each row cites an issue entry or a plan document | a row leaves when its plan document leaves, or when its issue closes with no plan document scheduling the work |
| how to perform an activity | the owning skill | the procedure changes |
| **none of these, nor a row of the project's own** | **ask the owner before writing it anywhere** | the table gains the row |

The project's root `CLAUDE.md` adds its own rows. **A measurement is routed by what it serves**:
the head of the decision it supports, the entry of the defect it characterises, the tripwire whose
threshold it is, or else the commit message that took it.

**A reference is written wherever the text would have to be revisited if the entry it names
changed**: a design head names the goal it derives from and a decision of another Component it
depends on; an issue names the decision it strains and the goal it threatens; a guard, a
workaround, a stub or a test that exists because of an issue names it in the comment at the site; a
tripwire names its decision; a rejected alternative names the decision it lost to; a commit message
names every entry it opens, closes, reverses or argues from. A reference whose entry's change would
leave the text unaffected is not written. A reference is one backticked span,
`<kind>@<anchor>@<id>`; an illustration that must not resolve writes a placeholder in angle
brackets. The checker reads Markdown and Rust source; a reference anywhere else is found by grep.

**A finding is repaired in a form the checker judges, never by moving the pointer into plain
text**: the right anchor, `path@elsewhere@<path>` for a path the tree does not hold, a placeholder,
or a rewritten sentence. A pointer that no checked form expresses is written in plain text only
beside a reference to an issue entry of this project that records the missing form. A gap of the
checker itself gets that entry in this project's own register, since a reference cannot reach
another project.

## The installed skills

- `knowledge-architect-decision-recording`: a design decision has been made or reversed.
- `knowledge-architect-issue-tracking`: before diagnosing a problem; parking anything; a
  tripwire fires; work closes an entry.
- `knowledge-architect-design`: a design question has an open solution space;
  keep-or-change about an existing design; a bug trend suggests the design is the problem.
- `knowledge-architect-planning`: a design discussion converged on its full path; a step of a
  milestone starts or lands.
- `knowledge-architect-review`: before merging to the main branch, or when an
  activity's skill says its work is ready.
- `knowledge-architect-agent-configuration`: before editing a `CLAUDE.md`, a skill or an agent.
- `knowledge-architect-setup`: the project adopts the workflow, or a version upgrade is
  installed.
- `knowledge-architect-goal-setting`: before writing or editing any goals home; a Component has no
  goal; the owner states or abandons a purpose; a decision conflicts with a goal.
- `knowledge-architect-retrospective`: once per session, offered when a branch the session worked on
  merges, a plan document leaves, or the session ends.

A project's own skills add to these, and never replace them. Which project skill adds to which
installed one is the routing table of the project's root `CLAUDE.md`. Read the installed skill and
every skill the table lists beside it.
