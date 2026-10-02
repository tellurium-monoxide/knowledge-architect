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
The one name it holds on purpose is the workflow's own upstream repository, where a retrospective's
findings on the workflow go, per `design@agent-skills@retrospective-destination`. It holds no live
reference: a reference resolves only against the tree it stands in, and no tree but this one holds
this repository's entries. An illustration writes a placeholder in angle brackets. The command a
project runs is written as the placeholder that the install fills with the project's declared
command, per `design@core@declared-command`.

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

### The design-discussion skill is named discussing-design-decisions `##designing-skill-name`

The skill is installed as `knowledge-architect-discussing-design-decisions`. The word "design"
alone is often read as visual design. The full name is not ambiguous, and its length is the cost
accepted for that. It names no domain: the owner uses the skill for programming and for game design, so
a name such as designing-software would exclude a use it serves.

### Until a skill for bounded problems is installed, a bounded problem goes back to the owner `##bounded-problem-branch`

A problem that arrives bounded, a clear requirement whose main risk is over-building, is not a
design discussion. The design-discussion skill says so, states the strongest open reading of the
problem beside the bounded one, and leaves the next step to the owner. It names no skill for the
bounded case, because no installed skill covers it, per `design@agent-skills@no-external-handoff`.
It does not send the problem to the planning skill either: the owner holds that a bounded problem
still needs investigation and testing, and a spec records a design and its implementation sequence
without running either. The skill that will cover it is
`issue@agent-skills@a-skill-for-bounded-problems`; when it lands, this branch names it.

### The design-discussion skill is written for frontier-tier models only `##frontier-tier-only`

The design-discussion skill is not simplified for smaller models. A smaller model takes part in the
workflow as an implementer of what the owner and a frontier model decided, not as the owner's
counterpart in the discussion. So "a smaller model would not follow this" is not an argument for or
against any wording of the skill. Its description states "Requires a frontier-tier model
(Opus-class or stronger)": a model below that bar produces the format without the discipline, and
the description is the only text an installer reads before the first run. The evidence is a
scripted four-turn discussion run on three models, described in `path@agent-skills@README.md`: the
smaller one reproduced the ledger's format, and it endorsed a weak proposal, invented states outside
the closed set, and dropped open threads between rounds.

### The design-discussion skill structures how a discussion is conducted, never what is proposed `##structure-the-flow`

The skill fixes the flow: proposals are argued, threads carry states, and closure waits for the
owner's word. It never fixes what may be proposed. The failure it was written against is the
solution quota: a mode that asks for two or three options gets two or three, and where one shape
applies, the others are fabricated to fill the count. A fabricated alternative makes a real
proposal appear to have won a contest that never happened, and spends the round that could have
tested it. The instructions that implement this are "never pad with alternatives to reach a count"
and the nearest-rival test, under which "no rival worth naming" is a claim to test. An edit that
specifies what must be proposed, rather than how a proposal is argued and tracked, works against
this entry.

### The design-discussion skill guards the outcome of a discussion, and its tables are a display `##outcome-over-display`

The threads, states and tables exist so the owner can keep track when an agent produces much
content at once. A discussion that reaches a well-argued outcome with an imperfect table has
worked. A finding about the skill is worth an instruction only if it corrupts the outcome: what
gets recorded, what gets built, or what the owner believes was decided. A finding that makes only an
intermediate display imperfect is not, since agent behaviour is not deterministic and an
instruction bought for display fidelity costs more rules than it saves. Together with
`design@agent-skills@expectation-set-bounds-scope`, this entry bounds which findings become text.

## How the installed text changes

### The record of the installed text keeps intent and coherence, not wording `##instruction-record-is-minimal`

A decision about the installed text is recorded only where it keeps the owner's intent on record, or
keeps two parts of the workflow consistent. A rewording, and the rationale for how one instruction
is phrased, is not recorded. Such proposals can be made without bound, unlike a technical decision,
which costs implementation work. The commit message carries a rewording's argument.

### An instruction is added to an installed skill only on evidence from real use `##additions-need-real-use`

Real use originates an addition: a behaviour seen in a real session, produced unprompted or asked
for by the owner mid-session, with the owner naming what the session would have lacked without it,
and a one-sentence statement of the mechanism that produced the need. An unprompted behaviour shows
the gap, not the wording that fills it. A review finding originates an edit only where its defect
is provable by reading: a contradiction, a broken trigger, a factual error. A finding that predicts
a behaviour is parked as an issue, which states what a real session would have to show. Published
literature and synthetic scenarios originate no edit. The skill text carries no size budget: an
edit is judged on whether it changes behaviour, and two real sessions held the design-discussion
skill's full ledger discipline at several hundred lines without drift. The rule derives from
`goal@knowledge-architect@the-workflow-improves-through-real-use`, which is stated for the whole
workflow, so it covers every installed skill. A rule written in the designing-together repository
for an abandoned discussion was argued, approved, written and reverted in one session, because its
case was already covered; no observation would have admitted it.

### A gap in an installed skill is worth text when it is a missing capability `##capability-over-conformance`

No wording makes a methodology self-enforcing: a rule set edited toward leaving no gap grows one
rule per observed interaction, and each new rule creates surface against the rules already there.
In one revision of designing-together, 34 edits produced 10 interaction defects, nearly all between
rules that scripted one exchange. So a gap is weighed by its kind. A capability gap is a move the
agent does not have, such as a way to investigate, to discriminate between positions, or to record
what the next session needs; filling it is worth text. A conformance gap is a move the agent has and
might not make; filling it is worth text only where the default behaviour is systematically wrong,
not occasionally absent, as the design-discussion skill's rules against agreeing without testing
are. When both readings fit, the capability form is written: a tool and the judgement to use it,
not a script for one interaction.

### An installed skill's scope is bounded by a stated expectation set, which the retrospective carries `##expectation-set-bounds-scope`

An installed skill may state what it expects of the owner: the assumptions about the owner's
behaviour and the project's shape that the skill is built on, each naming what degrades when it
does not hold. The set is the scope test for every proposed edit of the skill: a finding that
describes an owner's behaviour outside the set is not a gap, and no instruction is written for it.
The set scopes the owner's behaviour only. A finding that two installed instructions leave no move
satisfying both is always in scope.

The sets live in the retrospective skill, the one installed activity that judges whether something
is a defect of an installed skill. The retrospective runs in the installing project, which holds the
installed files and not this component's README, so a set the retrospective applies has to ship. A
set inside the skill it bounds would be read on every use of that skill, and an agent reading it
would check the owner against a list. The README restates each set for the owner, who is the one it
describes. It works with `design@agent-skills@additions-need-real-use`: that entry says what evidence
admits an addition, and this one says which gaps are worth admitting. In the designing-together
repository, a rule for an abandoned discussion was argued, approved, written and reverted in one
session, and four further review findings about the user's behaviour were queued behind it on the
same reasoning. Only the design-discussion skill states a set so far:
`issue@agent-skills@expectation-sets-for-the-installed-skills`.

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
milestone in the same session. The design-discussion skill ends at convergence, the premortem and
the owner's rulings on tripwires, and writes no plan document of its own. Its decisions are
recorded when their work lands, per `design@agent-skills@harvest-after-implementation`. One skill owns the document's shape: two skills describing the sections of one document
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

### A transcript reviewer checks a record of a discussion against the owner's words `##transcript-reviewer-agent`

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

## Reviews

### The standing-state review runs before every merge `##conformance-before-every-merge`

The installed dispatching-a-review sends the standing-state reviewer before every merge to the
main branch, whatever the change. That reviewer is the standing re-entry point of every tripwire
home, as the installed tracking-open-issues states, and a tripwire is read again only when some
review reads it. A re-entry point that depends on whether a change looked related to a tripwire is
one that a change touching the guarded decision indirectly skips: the reviewer reads every entry of
every home, not the subset the diff seems to concern. This serves
`goal@knowledge-architect@documentation-stays-consistent`.

## The configuration a project holds

### The workflow's rows of the knowledge table ship in the primer; the project's own rows sit in its root CLAUDE.md `##knowledge-table-home`

The knowledge table maps each kind of statement to its one home. Its rows for the workflow's own
records (a goal, a decision, a losing alternative, an issue, a tripwire, a contract, a plan
document) ship in the installed primer, and move with the version. A project's own rows, such as
its plans directory or a register it declares, sit in its root `CLAUDE.md`, which the project
owns. A table written whole into each project at setup would keep the old routing after an upgrade
that changes it, against `goal@knowledge-architect@agents-get-a-complete-workflow`. A table shipped
whole would leave a project no place for its own rows, against
`design@agent-skills@overlay-by-separate-skills`. A project row may refine a primer row with what is
the project's own, such as the path of its plans directory; it never contradicts one, and a row
that only repeats one is removed.

### The primer holds only what every session needs and no installed skill delivers `##primer-limit`

The primer reaches every session of every installing project, so its content test is the bound,
not a count of lines: it holds what every session needs and no installed skill delivers at the
moment it is needed. A convention of one project does not go in it, and neither does a procedure a
skill delivers when it loads. The workflow targets frontier-tier models, which the
design-discussion work requires, so a size limit would protect a reader the workflow does not
serve.

### The primer carries the goals rule, the intent-and-claims rule, the check before diagnosing, and the rule for what is met outside the task `##primer-content`

Besides the knowledge table and the list of installed skills, the primer carries the rule on when
to write a reference, and four directives the skills rely on and no skill delivers at the moment
they apply. **A reference is written wherever the text would have to be revisited if the entry it
names changed.** That rule is the premise the workflow relies on to be useful: an entry of a
register is referenced wherever it is load-bearing, so the checker can list what a reversal, a
closure or a firing touches, and the scope of reversing a decision can be assessed from that list,
per `goal@knowledge-architect@structure-and-workflow-work-together`.

- **The goals are the only statements assumed to come from the owner.** A recorded decision was
  reviewed, but its review can miss a detail or an implication, more often as the volume of agentic
  work grows. When a decision conflicts with a goal, the likely cause is that the owner missed the
  conflict: the goal prevails, and the conflict goes to the owner, per
  `goal@knowledge-architect@the-owner-decides`.
- **A design home is intent, and a claim about the code goes stale**: the code is checked against
  the first, and the second is verified before it is relied on.
- **Before diagnosing anything as a problem, a session checks whether it is already recorded**,
  with the listing commands of the checker.
- **Something met outside the task takes the first of four outcomes that applies.** If it bears on
  the current work, stop and present it to the owner at the top of the turn, with a default. If its
  fix is checkable from the diff alone, because it changes no behaviour, no decision and no test
  outcome, fix it in a commit of its own. If its `Why it matters` and its `What would close it` can
  be written, open an issue. Otherwise name it, with why it is dropped. A turn that met anything
  ends with a section listing every item and its outcome. A one-line mention inside a long report
  is easy to miss, as the owner observed in real sessions, so every outcome leaves a record or a
  listed line. The owner rules that a fix under the second outcome needs no word of theirs.

### A project states its plans directory in its own rows of the knowledge table `##plans-directory-declared`

The setting-up skill proposes docs/plans/ and writes the path the owner accepts among the
project's rows. A manifest key would be a checker change that nothing reads yet; it belongs to a
structure for plan documents, `issue@core@structured-plan-documents`. Nothing checks that the row
exists, so a project whose root `CLAUDE.md` lacks it leaves the planning skill without a plans
directory: a known limit until that structure exists.

### The routing table lists only what a project adds to an installed skill `##routing-table-shape`

The project's root `CLAUDE.md` carries one row per installed skill or agent that a project skill or
agent adds to, naming the additions. The setting-up skill writes the table, and the
maintaining-agent-config skill keeps it. It carries no "read it when" column: the harness already
lists every skill with its description, and a copy of it would drift. A project skill that adds to
no installed one needs no row.

### A project adds to the workflow through skills of its own, never by editing an installed one `##overlay-by-separate-skills`

An installed skill is complete on its own. A project adds its conventions through its own skills,
agents and root `CLAUDE.md`, under its own names. An installed file is compared byte for byte with
the pinned version and overwritten by the install, per `design@core@owned-namespace-check`, so an
edit to it fails the check and is lost at the next install; a project skill with the same base
name as an installed one would load beside it, not replace it. Where a project skill would have to
contradict an installed one, the installed text is wrong for that project, and the owner reports
it. A project that needs to change one instruction has no means yet:
`issue@agent-skills@patching-an-installed-skill`.

### A project's own skills and agents carry its name as a prefix `##skill-name-prefix`

A project skill or agent is named `<project>-<activity>`, the directory or file name equal to the
frontmatter `name`, so it is told apart from an installed one and from another project's. A colon,
as in `<project>:<activity>`, is not used: a subagent's name cannot hold one, and a skill so named
cannot be told apart from a plugin's.

### Setting up stops at a conformant structure, and the move of existing documents is planned work `##adopting-existing-docs`

For a project that already has documentation, the setting-up skill takes an inventory of it,
proposes a destination for each document, takes the owner's rulings, and opens one `todo` issue for
the move. The move itself is a milestone of its own, written under the planning skill. A migration
is a decision with arguments of its own, such as which recorded decisions still hold, and it fits
in one session only for a small project. The issue keeps the old documents beside the new homes
listed as outstanding work, per `design@agent-skills@planned-work-is-an-issue`.

### A project pins the checker exactly, and runs its pinned binary from inside the project `##exact-pin`

A project depends on one exact version of the checker, so moving it is an explicit edit, and the
installed skills move with it. A dependency builds no executable for the project, so a Rust project
runs the pinned version through its maintenance crate, per
`design@agent-skills@xtask-pins-checker`, and any other project installs it into a directory of its
own with `cargo install --locked --root`. A machine-wide install would give two projects on one machine one
version, which is what bundling the workflow into the checker avoids, per
`design@knowledge-architect@binary-bundles-workflow`. A project with an extension runs its own
binary under a name of its own, per `design@core@declared-command`.

### The setting-up skill recommends one gates command, run by the published gates library `##gates-convention`

One command runs every check a project owes before a merge, runs them all when one fails, and exits
non-zero when any fails, so a verdict is one exit code and nothing is read through a pipe. In a Rust
project it is a command of the maintenance crate, per `design@agent-skills@xtask-pins-checker`, so
every adopting project runs the gates refined in this repository and in thaum. The gates a project
owes are its own list. The setting-up skill proposes two goals for such a tool, for the owner's
ruling: one command runs every check owed before a merge, and a task performed repeatedly becomes a
command of the tool.

### In a Rust project, one maintenance crate pins the checker and runs the gates `##xtask-pins-checker`

The maintenance crate of a Rust project depends on the checker and on the gates library, both
pinned exactly, and serves two cargo aliases: `cargo x` for its own commands, the gates among them,
and `cargo klarch` for the checker's commands, which it carries under a command of its own through
the core's library, per `design@core@the-core-cli-is-a-library-module`. The gates run through the
published library, per `design@gates@gates-crate`, and the checker they run is the one this crate
pins. One crate does what a crate for the checker and a crate for the gates would otherwise do,
and gives the project's other repeated tasks a place, per
`goal@knowledge-architect@setup-brings-quality-tools`. The cost accepted: building the gates builds
the checker, which the check gate needs anyway.

### setting-up shows a Rust project its maintenance crate `##setup-rust-section`

The setting-up skill ends with a section for a Rust project: the maintenance crate's manifest, its
aliases, its main, the recommended gates and a continuous integration workflow that runs them on
every ready pull request, each labelled as an illustration to adapt. It serves
`goal@knowledge-architect@setup-brings-quality-tools`. The section is for Rust because the project
is focused on Rust, the language whose comments the checker reads; another language gets a section
of its own when a project needs one. No check compiles its snippet:
`issue@agent-skills@the-setting-up-snippet-is-unchecked`.

### Every Component states at least one goal `##goals-required`

The setting-up skill does not finish a Component without at least one goal, stated with the owner.
A Component with no goal gives its design nothing of its own to be judged against, and a goal that
is a Component's responsibility left unstated is one nobody is responsible for, against
`goal@knowledge-architect@the-owner-decides`. What those goals are is
`design@agent-skills@goal-placement`. Nothing checks it mechanically yet:
`issue@core@a-component-states-at-least-one-goal`.

### A decision that relies on the checker says so, and references none of its decisions `##relying-on-the-checker`

A project cannot reference an entry of another project, so a decision that relies on the checker
states that it relies on the checker working as intended. This holds until a form for such
references exists, `issue@core@cross-project-references`.

## Goals

### A goal stays while it is met, and leaves only on the owner's word `##goal-lifecycle`

A goal is met or unmet, and stays in its goals home while it is met. A goal removed when it is
achieved stops being checked, and can stop being met without anyone noticing. A goal leaves only
when the owner abandons it, per `goal@knowledge-architect@the-owner-decides`.

### A goal entry states an outcome, and what would show it met `##goal-entry-shape`

A goal is a heading stating an outcome as a sentence, with its slug, at the level the core gives
the goal register, per `design@core@an-entry-is-a-heading-at-the-register-level`, and one short
paragraph saying what it means and what would show it is met. It never states a mechanism: the test
is whether a change would make the Component for something else, which is a goal, or reach the same
end another way, which is a decision. Goals stay short, because a goals home is read whenever a
decision is argued from one.

### The owner states intent, the agent drafts, and the owner rules on every goal by its slug `##eliciting-goals`

The agent asks the owner to state their intent and any goals they have, refines the wording, and
proposes further goals from the documentation, or from the code and content when the documentation
does not say enough. It writes a short draft that goes into the goals home verbatim if approved,
each goal marked with its source, the owner's statement or what it was proposed from, and asks the
owner to read it in full. The owner rules on each goal by its slug: approved or dropped. A goal
with no ruling is asked about again; it is neither written nor dropped silently, because an unnamed
goal may not be the owner's intent either. The agent still helps a person write the document, and
the rulings by slug keep it the owner's, per `goal@knowledge-architect@the-owner-decides`. Drafting
nothing and only asking lost: it gave up the help with the wording. Drafting and letting the owner
correct lost too: a goal accepted by not objecting is not the owner's word.

### A goal sits in the Component responsible for fulfilling it `##goal-placement`

A goal is written in the goals home of the Component whose responsibility it is to fulfil it, even
when decisions of other Components serve it too: a goal of any Component can be referenced from
anywhere, per `design@core@a-slug-belongs-to-a-component`, so serving it does not require moving
it. The root's goals state what the project provides to its consumers. A published Component
serves those consumers, so its goals are encouraged to be sub-goals that refine a root goal, more
specific than the root states, and each such goal references the root goal it refines. A Component that
serves only the project, such as a maintenance tool, serves all of the root's goals at once, and
its goals need not refine one. Placing a goal at the root because several Components serve it lost:
it would lead to an excessive promotion of goals into the root, and leave a goal with no Component
responsible for it, against `goal@knowledge-architect@the-owner-decides`.

### A Component's goal names the project goal it refines `##component-goal-refines-root`

A Component's goal that refines a goal of the root carries a reference to it, so rewording or
abandoning the root goal lists every Component goal derived from it, per
`design@knowledge-architect@a-reference-claims-a-revisit`. Which goals refine a root goal is
`design@agent-skills@goal-placement`.

### A goal need not be met yet `##an-unmet-goal-is-intent`

A goal is the owner's intent about where the project should get to, and it constrains future work
and design from the moment it is written, met or not. When nothing fulfils it yet and no plan
document schedules the work, a `todo` issue holds that work and references the goal, so the gap
between the goal and the tree is listed as outstanding work.

### Goals change only through the setting-goals skill `##when-setting-goals-runs`

The setting-goals skill runs at setup for every Component, when the owner states, rewords or
abandons a purpose, and when a decision conflicts with a goal and the primer's rule sends the
conflict to the owner. An agent never edits a goal outside it, since a goal binds outright where a
decision binds as a presumption, per `goal@knowledge-architect@the-owner-decides`.

## The retrospective

### A retrospective is offered once per session, at a defined moment `##retro-trigger`

The retrospective is offered once per session, at the first of three moments: a pull request the
session worked on has merged, a plan document has left, or the owner says the session is ending.
It runs only if the owner accepts. A moment named by an event can be followed by any agent where
"a moment it judges right" could not, and offering it once keeps it from interrupting work.

### A retrospective examines four subjects `##retro-content`

It opens with what the session did, at the level of the workflow, and examines the installed skills
and agents, the project's own instructions, how the two interact, and the checker: its defects, its
blind spots, its false findings, and what would make it easier to use. For each it lists what was
unclear, missing or wrong, quoting the instruction. Its scope is wider than the installed text so
that it is useful to a project adopting the workflow, whose problems may come from its own
instructions and from their interaction with the installed ones as well, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`.

### A retrospective asks three standing questions, on the decisions they watch `##premortem-as-watch-points`

Every retrospective asks whether the session needed to change an installed skill, whether the
primer reached the session and its subagents, and whether a project skill's addition was missed.
Each watches a decision whose failure would be seen in real sessions before any check could see it:
`design@agent-skills@overlay-by-separate-skills`, the primer's delivery by an import line in
`design@core@owned-namespace-check`, and `design@agent-skills@routing-table-shape`.

### A retrospective writes one file per project whose text must change `##retro-two-files`

Each finding goes to the file of the project whose text or code must change: one file for the
project, one for knowledge-architect. A finding on an interaction whose fix may fall on either side
goes in both. The project's findings never leave the project, so only the workflow's file is held
to the rule that it carries nothing of the project beyond what a finding needs.

### The retrospective's files live outside the project, where the owner chose `##retro-file-location`

The files go to a directory outside the project, which the owner chooses the first time. Inside
the project they would enter its history if committed, and be lost to the next clean if ignored.

### Nothing of a retrospective leaves the machine without the owner's reading and word `##retrospective-destination`

The owner reads both files verbatim and may edit them. On the owner's word, and where the owner
directs, the project's findings become issue entries in its own register, and the workflow's file
becomes an issue on knowledge-architect's repository, through `gh` or by the text and the address.
The repository is the workflow's own upstream, the one name of a project the shipped text holds.
