---
name: knowledge-architect-agent-configuration
description: MUST use before adding or editing the project's root CLAUDE.md, a scoped CLAUDE.md next to code, a project skill, or a project subagent definition, and after installing a new version of the workflow. Covers choosing between root, a skill, a scoped file and an agent; how a skill's scope and description are shaped; the content instructions; the project's rows of the knowledge table; the routing table that maps each installed skill to the project skills that add to it; and why an installed file is never edited.
---

# Maintaining the agent configuration

Scope: the project's own agent configuration, the text an agent reads to work in this project.
Its root `CLAUDE.md`, its scoped `CLAUDE.md` files, its own skills and its own subagent definitions.

Not covered here: **the installed files**, the skills, agents and primer that
`cargo klarch install-agent-skills` writes. They are never edited by hand (`skill@knowledge-architect-agent-configuration@installed-files-never-edited`). **Where the argument
for a decision about the configuration lands**: `skill@knowledge-architect-decision-recording`; this
skill owns how to write the configuration, that one owns where the argument goes. **Setting the
configuration up the first time**: `skill@knowledge-architect-setup`.

## Content and style of agent-facing files `##content-and-style`

Everything written in a `SKILL.md`, a subagent definition or a `CLAUDE.md` follows these:

- **Current reality only**: no history, no dates, no commit hashes, no session records. Examples
  kept minimal. What "once was" never goes into an agent-facing file.
- **No cautionary narratives.** They cost context, divert the attention, and buy less instruction
  strength than they cost.
- **References are checked like anywhere else.** A project's skills and agents are walked by
  `cargo klarch check`, and a live reference in them must resolve. Run it after editing one.
- **An edit that changes what an agent is told to do is a decision.** Before writing it, search
  the design homes and the goals for the behaviour the edited text describes, as
  `skill@knowledge-architect-decision-recording@reversal-check` searches for a recorded statement a decision would
  reverse. An edit that writes into a design home, or edits a text whose behaviour a head
  describes, loads `skill@knowledge-architect-decision-recording` before it is written; that skill
  judges whether it contradicts a head, outgrows its title, or earns text at all. One that strains a
  goal goes to the owner, as a decision that conflicts with a goal does, under
  `skill@knowledge-architect-goal-setting`. A rewording that changes no instruction is not a
  decision.

## Where a piece of agent-facing text goes `##where-text-goes`

An agent needs two kinds of text: **knowledge** about the project (what exists, how it works) and
**instructions** about the tasks it performs. They are split first by kind, then by **when the place
reaches a session**.

### Knowledge

| place | when it reaches a session | what it holds |
| --- | --- | --- |
| root `CLAUDE.md` | every session, unconditionally | the knowledge every task needs |
| a scoped `CLAUDE.md` | when a session reads, writes or edits a file in that directory, or opens it | the contracts and traps of one Component |
| an issue directory | when a session searches for open issues: before recording a new one, or when asked to fix one | what is outstanding in that Component or location |

### Instructions

| place | when it reaches a session | what it holds |
| --- | --- | --- |
| root `CLAUDE.md` | every session, unconditionally | instructions that span all activities, the project's rows of the knowledge table, the routing table |
| a skill | its description is listed every session; its body loads only when invoked | a procedure, and the knowledge shaped by a task. Length is nearly free, so a skill can be thorough |
| a subagent definition | only when a subagent of that type is spawned, and then from its first token | a task only ever done by a subagent, such as a reviewer following a fixed standard |

#### Dispatching between them

**The unit is an activity**: the scope over which a complete set of procedures makes sense.
Decided in order; the first match wins.

1. **Is it needed by every activity?** Then it is **baseline**: root `CLAUDE.md`. **Reading is
   always baseline**, because a session cannot know what it will read before reading it, and so
   cannot know which skill it would have needed. **Producing is an activity, unless the producing
   is itself universal**, as committing is: every session commits.
2. **Otherwise, does it produce a nameable artifact?** Then it is an **activity**, and it owns a
   skill. The discriminator is artifact versus property, not locality: work that produces one work
   list across every file it touches is still an activity, while a property that must hold in
   every file produces no artifact and is baseline.
3. **Is it shared by two or more activities but not all?** Then it is a **sub-activity**: its own
   skill, named as a prerequisite by each activity that needs it. The installed decision-recording,
   issue-tracking and review skills are this.
4. **Is it read only by a fresh subagent?** Then it is a subagent definition. A definition sets a
   **standard**, not a one-off task, because it must be reusable. A review axis whose content
   depends too much on the task to be standardized stays as a line in the activity's skill.

## Shaping a skill `##shaping-a-skill`

- **Name it by its activity, as a noun of one or two words in common usage** (planning, review,
  issue-tracking), never as an artifact the activity writes: the text names both, and a skill named
  like its artifact cannot be told from it. Give it the project's prefix: the project's name and a
  hyphen, as in `<project>-development`. The directory name and the frontmatter `name` are equal.
  The same prefix names the project's subagent definitions.
- **End every level-two heading of a skill, a subagent definition and the root `CLAUDE.md` with a
  slug**: two hashes and the id in backticks, the id naming the section's subject. It is what a
  reference such as `skill@<name>@<slug>` cites, and the check reports a heading without one. A
  subagent definition's file name and its frontmatter `name` are equal, as a skill's directory and
  its `name` are where a skill sets one.
- **Begin the description with MUST**, and make the trigger **symptom-shaped, not request-shaped**.
  A session rarely asks to "track open issues"; it does meet a behaviour that looks wrong. Write the
  symptom.
- **State what the skill does not cover**, naming the skill that does. A seam left unnamed becomes a
  dead zone where neither skill loads.
- When two skills legitimately both apply to one change, say so, and in what order.

**One skill covers one activity.** A description naming several unrelated scopes is matched by no
task, so it loads for none. The cause looks like the wording of the description, but the content
was scoped wrong before the description was written: fix the scope, not the wording. Enumerating the facets of one activity is the opposite move and is
correct. The test is whether a single task can want all of them at once.

**A skill is self-sufficient.** It holds every piece of knowledge its activity needs, such as a
build command, a dispatch table or a common trap, even where it is stated elsewhere: restated where
the restatement is no longer than a pointer, and otherwise by a pointer to a complete home read
whole, per `primer@where-knowledge-goes`. It points only to **task material**, data that varies per
instance, to **a named prerequisite skill** or a section of the primer, one complete instruction
rather than a fragment to reassemble. The test, per pointer: could a session complete this activity correctly
without opening it? If not, the content belongs in the skill. What this guards against is
dilution rather than length: each pointer is an extra read a session must remember, and the more
there are, the less likely all are followed.

## The two tables of the root CLAUDE.md `##root-claude-md-tables`

**The project's rows of the knowledge table.** The installed primer carries the workflow's own rows:
where a goal, a decision, a losing alternative, an issue, a tripwire, a contract or a plan document
goes. The project's root `CLAUDE.md` carries the rows that are the project's alone, under a heading
of its own: its changelog, a register it declares, a directory with a convention of its own. The
last row of the combined table is **ask the owner before writing it anywhere**. It is for a
statement with no home, not for a choice between two: when two rows could fit, pick one, say which,
and carry on. A genuine gap means the table is incomplete, and what the table holds is a decision
about the shape of the configuration, which is the owner's. **Each answer ends as a new project
row**, so the fallback limits itself: if it fires often, the table is what needs changing, rather
than the entry. A project row may refine a row of the primer with what is the project's own, such as
a README that is also its package's page on a registry; it never contradicts one, and a row that
only repeats one is removed.

**The routing table.** One row per installed skill or agent that a project skill or agent adds to:

| installed | project additions |
| --- | --- |
| `knowledge-architect-<installed skill>` | `<project>-<activity>` |

A project skill that adds to no installed one needs no row: its own description triggers it. When
a project skill is added, renamed or removed, its row changes in the same commit.

## Installed files are never edited `##installed-files-never-edited`

An installed skill, agent or primer is compared byte for byte with the version the project pins, and
the install overwrites it. **A change the project needs is a project skill or agent of its own**,
with its own name and its row in the routing table. It adds to the installed text: an extra step,
an extra review axis, a convention of the project. It never contradicts it. Where it would have to
contradict it, the installed text is wrong for this project: say so to the owner, who may report it
to the workflow's maintainers.

## After installing a new version `##after-installing`

`cargo klarch install-agent-skills` writes the files the new version ships and removes the ones it no
longer ships. In the same commit:

- read the new primer's table against the project's rows, and raise any project row that now
  restates or contradicts a primer row;
- update the routing table: a row whose installed skill was renamed or removed changes or goes;
- read each project skill against the installed skill it adds to, for an instruction that now
  contradicts it;
- write into each open milestone document, afresh from the new `skill@knowledge-architect-planning`,
  its restatement of the procedure for working a slice, whether it held one before or not;
- run `cargo klarch check`.

## Reviewing a configuration change `##reviewing-a-change`

Dispatch a review when an instruction is written and a mechanism is in place to deliver it, per
`skill@knowledge-architect-review`. This skill adds no axis of its own: a change to the
configuration is a change to prose, which that skill's axes cover.
