# Goals — knowledge-architect

knowledge-architect is a tool that projects use to keep the quality of their documentation, with a
focus on the record of design: what was decided, and the intent and the arguments behind it. It is
built for agentic work. It gives AI agents a complete workflow, which aims to raise the quality and
the efficiency of their work, in particular in projects developed mostly by agents. Its main
purpose is to let agents work autonomously without drifting from the design intent their owners
recorded, per `goal@knowledge-architect@agents-work-without-drift`. The
documentation structure and the workflow are designed to work together.

A goal is met or unmet. A decision about how the project is built is won or lost, and lives in the
design home. **A goal stays in this document while it is met.** A goal removed when it is achieved
stops being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word, per `design@agent-skills@goal-lifecycle`. The goals are the
owner's intent: an agent may propose wording, and the owner decides it.

## Agents work productively and autonomously without drifting from the owners' design intent `##agents-work-without-drift`

The project as a whole, the workflow and the checker, lets AI agents carry more of a project's
work with less supervision, while what they build stays within the design the owners recorded. The
checker exists mostly for this: it reports where the documents and the code drift from the recorded
design, and it makes keeping that design current cheap. Nearly every other goal of the project
serves this one. It is met while agents carry work from design to merge with the owner ruling on decisions
rather than correcting drift, and a departure from the recorded design is caught by a check or a
review before it merges.

## The documentation records the design, with the intent and the arguments behind it `##design-is-recorded-with-its-arguments`

Each decision is recorded with its intent and its argument, the alternatives that lost and why they
lost, and what is still open. A later session can then tell what it may change, and what a change
costs, without re-deriving the argument or reversing a decision blind.

## AI agents get a complete workflow that raises the quality and the efficiency of their work `##agents-get-a-complete-workflow`

The workflow covers setting up a project and its goals, design discussion, planning, recording
decisions, tracking open issues, review, and a retrospective. It targets projects where agents do
most of the development. It aims for fewer review rounds, less re-derivation of past arguments, and
fewer decisions reversed by accident. Each part is an instruction an agent can follow without
guessing, and no two parts contradict each other. Writing the code itself is left to each project.

## The documentation structure and the workflow work together `##structure-and-workflow-work-together`

The workflow writes into the structure, and the structure gives the workflow what it needs: one
place to find each decision with its argument, and one place for what is open. Both live in the
repository beside the code, as files an agent reads and edits like any other, rather than in an
external tracker it would need a separate tool to reach. The tool computes the workflow's work
lists: what a reversed decision touches, and which issues and tripwires a change must re-read.
Neither is designed without the other.

## The documentation stays consistent with the code and with itself `##documentation-stays-consistent`

The reviews of the workflow keep it consistent. The tool supports them. Its check guarantees that
nothing cited is missing and that the declared structure holds. Its commands list and display the
entries a review must read. Development driven by agents changes code faster than review alone can
follow, so the reviews need these tools to keep up.

## The owner decides, and the record shows the owner's decisions `##the-owner-decides`

Goals and decisions belong to the project's owner. The workflow makes an agent argue its
proposals, state their consequences, and record the owner's rulings. An agent never substitutes
its own judgement for a ruling.

## Any project can adopt it `##any-project-can-adopt-it`

Nothing about one project is compiled in. A project declares its structure in its manifest, and the
installed skills name no project's paths. Every record is cited in one readable grammar, the same in
every project, so what an agent learns in one project applies in the next. A project pins the
version it uses, and moves to a new one when it chooses. The target is projects developed mostly by
AI agents. A project without agents can still use the checker alone.

## Adopting the project is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect finds what to do in its documentation, where they
already look for it: the README of each package, the API documentation of each library, the
installed skills, and the rendered site of the documents. The documentation says what to do in
plain words, with examples. It is met while a project adopts the checker and the workflow by
following that documentation alone, without reading the source or asking the authors.

## The workflow improves through real use `##the-workflow-improves-through-real-use`

Real sessions are the test of the workflow, not synthetic scenarios. The retrospective collects
what was unclear, missing or wrong, and the owner decides what changes.

## Adopting the workflow sets up quality tools proven to work with it `##setup-brings-quality-tools`

A project that adopts knowledge-architect is led to set up the quality-assurance tools that have
proven useful alongside it, in recommended shapes that work well with the checker: one command
running every check owed before a merge, continuous integration running that command, commit
messages checked as documents, and the checks of the project's language. It is met while a project
set up by following the workflow reaches these tools without designing them itself.
