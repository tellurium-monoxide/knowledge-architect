# Agent skills — design

Recorded intent for the text the checker installs: which skills and agents exist, how they are
named, and how they divide the work. Present tense, each decision carrying a slug, cited as
`design@agent-skills@<slug>`. What lost to a decision here is
`path@agent-skills@docs/rejected-alternatives.md`.

**What belongs here:** a decision about the installed text that does not survive deleting this
component. How the checker installs and verifies the text, including the prefix that names the
installed files, is `design@core@owned-namespace-check`.

## The shipped set

### content/ mirrors the install layout, and the build generates the list `##content-mirrors-the-install-layout`

The directory `path@agent-skills@content/` holds what `install-agent-skills` writes, and nothing
else. A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, and the primer
is `PRIMER.md`. The build script, `path@agent-skills@build.rs`, walks the directory and generates
`FILES`, each entry the install path and the text. It adds the installer's prefix to each skill
directory and agent file on the way out, so the installed names are the namespace of
`design@core@owned-namespace-check`. A file the layout does not map fails the build.

The list is generated because a list written by hand drifts from the directory it lists, and the
drift ships: a skill missing from the list is never installed, and nothing reports it. The core's
test `every_shipped_file_is_installed_in_the_owned_namespace` holds the build script to the
namespace, since an install path outside it would write a file the project owns.

### The shipped text names no project and holds no live reference `##shipped-text-is-reference-free`

The text under content/ is read in every project that installs it, so it names no path, no
Component and no convention of this repository, per `goal@knowledge-architect@any-project-can-adopt-it`.
It holds no live reference: a reference resolves only against the tree it stands in, and no tree
but this one holds this repository's entries. An illustration writes a placeholder in angle
brackets. The command a project runs is written as the placeholder that the install fills with the
project's declared command, per `design@core@declared-command`.

This repository's manifest takes content/ out of the walk, because its illustration paths would be
reported as paths of this repository. The installed copies under .claude are out of the walk by
construction. So no check reads this text: the reviews of each change to it judge it, until a
mechanical check exists, which `issue@agent-skills@shipped-text-is-reference-free-mechanically`
tracks.

### An installed skill names only installed skills `##no-external-handoff`

An installed skill or agent hands off only to another skill or agent of the installed set. It
never names a plugin or a skill from outside the set. A skill outside the set may be absent in the project, or
present at a version that contradicts the installed one, and either breaks
`goal@knowledge-architect@agents-get-a-complete-workflow`, under which no two parts of the workflow
contradict each other. Where an installed skill needs an activity the set does not cover yet, it
says what the activity is and leaves the next step to the owner.

## The workflow the skills carry

### A decision is recorded when the work that implements it lands `##harvest-after-implementation`

A decision is written into the design homes in the change that lands the work implementing it, not
when the spec is written. A design head is a claim about the code as it stands, so a head written
before the code is a hypothesis presented as a fact. While the work is open, the spec or the
milestone document on its branch is the only place the decision exists. A decision with no
implementing work, one that constrains work nobody has started, is recorded when it is made.

### A design head carries the standing argument, and history carries the deliberation `##standing-argument-in-head`

A design head carries the decision and its standing argument: every premise whose failure would
reopen it, which is the goal or decision it derives from as a reference, the measurement it rests
on, and the fact that defeated its nearest rival. The deliberation, which is what was weighed, in
which order, what evidence was built and who ruled what, stays in the spec while the spec exists,
and in history after that. A decision taken with no spec carries its deliberation in its commit
message.

`goal@knowledge-architect@design-is-recorded-with-its-arguments` requires that a later session can
tell what a change costs without deriving the argument again, so the premises a reversal must
defeat are in the head. The deliberation is not, because every session reads the design homes
before it changes code, and a head that carried its whole deliberation would make that read cost
the length of every discussion ever held. History serves the reader who needs it: a deleted spec is
listed by `git log --diff-filter=D` on the plans directory, and its last state is one `git show`
away.

### An alternative earns an entry by the recording tests, not by having lost `##losing-alternatives-filter`

A losing alternative earns an entry in the rejected alternatives only if it passes the tests of the
installed recording-a-decision skill: it would change an interface something outside its Component
depends on, it was refuted by evidence that cost work, or a doubt remains that the winner meets
every goal. The rest stay in the spec and the commit message. Recording every losing thread of
every discussion would grow the rejected alternatives by proposals nobody would raise again, and a
reader proposing an alternative would have to search through them to find the few whose refutation
cannot be derived again in one round.

### A thread is a candidate when a shape lost to an argument, whatever its state `##losing-shape-test`

A ruled-out thread, a thread withdrawn with a defeating reason, and a superseded thread whose
distinct shape lost are each judged by `design@agent-skills@losing-alternatives-filter`. A thread
withdrawn with no defeating reason, and a superseded thread whose shape the absorbing thread
carries whole, stay in the spec only. So do the shapes of a question that produced no decision,
since an entry names the decision its alternative lost to. The state of a thread records how it
closed, not whether an argument defeated a shape, and a rule keyed on the state would drop a
superseded thread that lost on its merits.

### A design thread's slug becomes its entry's slug `##thread-slug-is-entry-id`

A thread of a design discussion is named by a slug minted in the entry-id grammar and checked for
a collision with the entries of the Component before it is used. In discussion prose it is written
plain, with a hash sign before it, never as a backticked span, which the checker would read as a
reference candidate. When
the thread is approved and its decision earns an entry, the entry's heading ends with the same
slug, so the spec, the commit messages and the design home name the decision with one identifier.
This holds until plan documents have a structure the checker reads, which
`issue@core@structured-plan-documents` tracks.
