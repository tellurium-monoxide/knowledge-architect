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

### An installed skill names only installed skills and the project's own `##no-external-handoff`

An installed skill or agent hands off only to another skill or agent of the set its version ships,
or to the project's own skills. It never names a plugin or a skill from outside the set. A skill
outside the set may be absent in the project, or present at a version that contradicts the
installed one, and either breaks `goal@knowledge-architect@agents-get-a-complete-workflow`, under
which no two parts of the workflow contradict each other.

The set is written across several changes before a version is released, so a skill may name a
member of the set by its installed name before that member lands. No release ships a name its set
does not hold.

## The workflow the skills carry

### A decision is recorded when the work that implements it lands `##harvest-after-implementation`

A decision is written into the design homes in the change that lands the work implementing it, not
when the spec is written. A design head is a claim about the code as it stands, so a head written
before the code is a hypothesis presented as a fact, and
`goal@knowledge-architect@documentation-stays-consistent` asks that the documentation stay
consistent with the code. While the work is open, the spec or the
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
the length of every discussion ever held. History serves the reader who needs it, because a spec leaves when its work lands, per
`design@agent-skills@spec-leaves-at-landing`: a deleted spec is
listed by `git log --diff-filter=D` on the plans directory, and its last state is one `git show`
away. That agents take that route is measured, not assumed: a scan of a project's session logs for
a `git show` or `git log` command naming a deleted plan document re-takes it, and a scan that finds
none in a project whose specs are deleted reopens this decision. The cost is accepted: a reader
without a clone of the repository cannot reach a deliberation.

### An alternative earns an entry by the recording tests, not by having lost `##losing-alternatives-filter`

A losing alternative earns an entry in the rejected alternatives only if it passes the tests of the
installed recording-a-decision skill: it would change a signature crossing the boundary of a
separately built unit or a serialized format, its reason rests on a reading of an external
specification the project implements, it was refuted by evidence that cost work, or a doubt
remains that the winner meets every goal. The rest stay in the spec and the commit message.
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks that a later session need not
derive an argument again: an alternative refuted by reasoning is derived again in the one round its
proposal costs, and one refuted by evidence is not. Recording every losing thread of
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

### A tripwire from a premortem is written at harvest, on the owner's word `##premortem-tripwires-on-the-owners-word`

A premortem's surviving causes become tripwires only where the owner rules that they should, and
each is written at the harvest of the decision it guards, in the tripwires home of the Component
that owns that decision. A tripwire names its decision's head, so the head exists first. Whether a
risk is worth watching is a weighing, and the weighing is the owner's, per
`goal@knowledge-architect@the-owner-decides`.

### A design thread's slug becomes its entry's slug `##thread-slug-is-entry-id`

A thread of a design discussion is named by a slug minted in the entry-id grammar and checked for
a collision with the entries of the Component before it is used. In discussion prose it is written
plain, with a hash sign before it, never as a backticked span, which the checker would read as a
reference candidate. When
the thread is approved and its decision earns an entry, the entry's heading ends with the same
slug, so the spec, the commit messages and the design home name the decision with one identifier.
This holds until plan documents have a structure the checker reads, which
`issue@core@structured-plan-documents` tracks.

## Plan documents

### The words: plan document, spec, milestone, plans directory `##document-vocabulary`

A plan document is any document in the plans directory, the one directory where a project keeps
them. A spec is the plan document of work done in one branch and one PR. A milestone is work across
several PRs with design sessions between them; its plan documents are its milestone document and
one spec per step. The word "plan" alone never names a document: it would name the directory, a
document and a kind of document at once. The words follow common usage among developers, which the
owner made binding: a milestone groups the work toward one goal, as GitLab's milestones do, and a
spec says what will be built and how before the code exists, in the sense engineering teams give
the word. "Design doc", the closest common term, lost because "design" already names the durable
register.

### One document per layer, and no snippet is authority `##spec-and-milestone`

The work of one PR has one plan document, its spec. A milestone's document extends the spec's
sections over several steps, and each step has its own spec. A plan document is detailed about the
design and concise about the implementation sequence. No untested code snippet in it is presented
as authority: a snippet is labelled as an illustration of a shape. A spec plus a separate detailed
implementation plan lost: both carry the same decisions and the second drifts from the first,
against `goal@knowledge-architect@documentation-stays-consistent`, and the owner observed that
implementers force such plans' untested snippets into the code at any cost, copying their comments
verbatim. A detailed plan, if one is ever written for a less capable implementer, covers a bounded
amount of work and opens by saying it rests on assumptions.

### A milestone is a directory, its head a README, each step a spec `##milestone-is-a-directory`

A milestone's plan documents are one directory in the plans directory: the milestone document is
its `README.md`, and each step's spec is a file beside it. The head is a README because the checker
resolves a relative link only in a `README.md` or an `index.md`, per
`design@core@links-are-navigation-rows`, so the head can link each step's spec as a navigation row.
A step's spec is a spec, so it leaves when its step lands, per
`design@agent-skills@spec-leaves-at-landing`: its decisions are then in the design homes. The
README leaves with the last step and keeps what crosses steps.

### A plan document leaves when its work lands `##spec-leaves-at-landing`

A plan document is deleted in the commit that completes its last harvest, and that commit's message
names its path. A plan document kept after its harvest is a second home for every decision it
carried, and it starts drifting at the first later reversal. Walked by the checker, its references
break at every reversal, and someone repairs a document about finished work; left out of the walk,
it is unchecked text that a grep finds with no marker that it is stale. Either breaks
`goal@knowledge-architect@documentation-stays-consistent`. Leaving the choice to each project lost
too: the installed planning skill would have nothing to say where a document's work ends, against
`goal@knowledge-architect@agents-get-a-complete-workflow`.

### The planning skill writes the plan document, in the session that converged `##designing-hands-off-to-planning`

The planning skill starts where a design discussion has converged, and writes the spec or the
milestone in the same session. The design-discussion skill, not yet installed, is bound to end at
convergence, the premortem and the owner's rulings on tripwires, and to write no plan document of
its own. One skill owns the document's shape: two skills describing the sections of one document
would drift apart, against `goal@knowledge-architect@agents-get-a-complete-workflow`. The session
matters because the discussion's ledger lives only in the conversation, and a document written from
memory in a later session loses the losing arguments and the conditions of each closure.

### Acceptance criteria live in the plan document of the work that judges them `##acceptance-criteria-in-the-document`

An acceptance criterion, a check on a recorded decision that only the work's built code can apply,
is written in its own section of the plan document of that work. Each names the decision it guards,
the step that judges it, the observable that fires it and the response. Every landing reports on
the criteria it judges. When the document leaves, a criterion that recurs is proposed to the owner
as a tripwire and written on the owner's word, per
`design@agent-skills@premortem-tripwires-on-the-owners-word`; any other is deleted. A separate file
of criteria would hold statements about the same work with the same lifetime, drift from the
document, against `goal@knowledge-architect@documentation-stays-consistent`, and stay behind when
the document leaves. The plan document is written in the session that converged, so the criteria
and the document are born together.

### Undesigned work is an issue, and no list of milestones is kept `##planned-work-is-an-issue`

Work that is known but not designed is a `todo` or `deferred` issue in the owning anchor, with its
leads in the entry. The plan document that schedules it closes the issue in the commit that adds
the document. `goal@knowledge-architect@structure-and-workflow-work-together` asks for one place for
what is open, and a roadmap file would be a second one beside the issue register: two schedules
drift. The cost: the order of future work has no home. The owner accepted it in the checkpoint that
closed the discussion, without answering the question put to it earlier.

### No record of landed work is kept `##no-progress-record`

No installed skill asks for a file recording what has landed. History lists every plan document
that left, and each landing commit says where its results live; releases are in the changelog. A
progress file would be a third document about the same work, with a lifetime of its own, against
the one place for what is open of `goal@knowledge-architect@structure-and-workflow-work-together`.

### A transcript reviewer checks a plan document against the owner's words `##transcript-reviewer-agent`

The installed agent `knowledge-architect-transcript-conformity-reviewer` reads the transcript of a
discussion and checks that a document records the owner's decisions as made: each state, the scope
of each decision, the verbatim quotations, and nothing the owner said left out. The planning skill dispatches it on every plan document written from a
discussion whose transcript is available, and the installed dispatching-a-review lists it as the
axis for any document that records the decisions of such a discussion. It
is an agent, not a line in a skill, because its standard and its extraction rule are fixed, and the
rule is learned from a failure: a filter on text substrings once dropped one of the owner's
messages. It serves `goal@knowledge-architect@the-owner-decides`: in the review of the change that
installed the first skills, a reviewer briefed with this standard was the one of six to find a
decision recorded narrower than the owner's approval; two others found the same head contradicting
the shipped text.

### Plan documents written under the planning skill keep a shape a later structure can read `##structure-ready`

A plan document's layout and its section titles are fixed, and every thread, step and acceptance
criterion carries an identifier in the entry grammar, written plain with a hash sign and never in
backticks. Nothing outside the plans directory cites an item of such a plan document; a `path`
reference to a whole plan document is allowed, and its dangling at deletion lists the texts that
depended on it. This is the shape a structure for plan documents, with registers of their own, can
read without rewriting them: `issue@core@structured-plan-documents`. The milestone document of
this repository's v0.1 predates this decision and is the one exception: it is one file, and other
documents cite its sections and steps until it leaves.
