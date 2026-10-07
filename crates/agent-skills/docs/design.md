# Agent skills — design

Recorded intent for the text the checker installs: which skills and agents exist, how they are
named, and how they divide the work. Present tense, each decision carrying a slug, cited as
`design@agent-skills@<slug>`. What lost to a decision here is
`path@agent-skills@docs/rejected-alternatives.md`.

**What belongs here:** a decision about the installed text that does not survive deleting this
component, and the rules of the published workflow, which this repository follows as an adopting
project does. How the checker installs and verifies the text, including the prefix that names the
installed files, is `design@core@owned-namespace-check`.

## The shipped set

### content/ mirrors the install layout, and the build generates the list `##content-mirrors-the-install-layout`

The directory `path@agent-skills@content/` holds what `install-agent-skills` writes, and nothing
else, except that a line `{{snippet:<file>}}` stands for a file of `path@agent-skills@snippets/`,
which the build inlines there, a `%%` line is a comment the build removes, per
`design@agent-skills@shipped-text-line-comments`, and a delivery placeholder is a literal the build
fills. A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, and the primer
is `PRIMER.md`. The build script, `path@agent-skills@build.rs`, walks the directory, renders each
file, removing its comments, filling its delivery substitutions and inlining its snippets, and
generates `FILES`, each entry the install path and the text. It adds the installer's prefix to each skill
directory and agent file on the way out, so the installed names are the namespace of
`design@core@owned-namespace-check`. A file the layout does not map fails the build.

The list is generated because a list written by hand drifts from the directory it lists, and the
drift ships: a skill missing from the list is never installed, and nothing reports it. The core's
test `every_shipped_file_is_installed_in_the_owned_namespace` holds the build script to the
namespace, since an install path outside it would write a file the project owns.

### The shipped text carries comments for this repository, `%%` lines the build removes, checked by taking content/ into the walk `##shipped-text-line-comments`

A line of a file of content/ whose first two characters are `%%` is a comment for this
repository's maintainers. It sits beside the instruction it explains, and it may cite design heads
and issues, which the walk checks, since content/ is in the walk. The build removes it whole, so no
installed file holds it; a `%%` line inside a fenced block, or one with leading spaces, fails the
build rather than ship. The crate's published source holds the comment lines as written, with their
citations of this repository: no installing project reads that source, only the text the install
writes. It gives the installed text what a comment gives code, the reason at the
site it explains, per `design@agent-skills@local-intent-binds`, so a decision about one instruction
need not take a head, per `design@agent-skills@a-head-is-owed-by-an-entry-test`. The owner's
argument: it "would solve in good part the problem where the design heads of the workflow get
bloated with details that, in a code project, would be served well by a local code comment. It
would also allow to cite the design heads or other items without polluting installed skill
results."

Line comments, not HTML comments: the checker treats an HTML comment as parked text and reads no
reference in it, so a citation there would go unchecked, and the owner judged line comments "saner
for the long term". The marker `%%` has no Markdown meaning, reads as a comment in Mermaid and
Obsidian, and cannot collide with a Rust snippet, whose comments are `//`. The comments are checked
by putting content/ back in the walk, rather than leaving them unchecked or adding a test in this
repository that would repeat the checker's reference reading. A declaration in the published
checker would cater to this repository's use in what consumers receive. That extends to the checker
what the owner said of the installed files: "I don't want to cater too much to this use case in the
installed files"; the extension is the session's argument, which the owner's choice of content/ in
the walk followed.

### The shipped text cites no entry of this repository, and may write a path every conforming project holds as a reference `##shipped-text-cites-no-entry`

The text under content/, with the snippets the build inlines, is read in every project that installs
it, so it names no Component and no convention of this repository, per
`goal@agent-skills@installed-text-works-anywhere`. The one name it holds on purpose is the
workflow's own upstream repository, where a retrospective's findings on the workflow go, per
`design@agent-skills@retrospective-destination`. It cites no entry: a reference to an entry resolves
only against the tree it stands in, and no tree but this one holds this repository's entries. A path
that every conforming project holds may be written as a reference: `path@*@docs/goals.md` names
each Component's own copy, required by `design@core@components-carry-the-same-documents`, and
`path@plans@README.md` names a file of the one plans directory, whose path the checker fixes, per
`design@core@plans-dir-fixed`. Either is true in every installing project, and it shows the syntax
the checker enforces there. The owner, of the per-Component paths: "it promotes the same kind of
syntax as what the checker enforces in projects holding the installed skills"; the same holds of
the plans directory's paths, which the owner approved with the default for the repairs:
"Agreed on the three items awaiting my ruling." The checker accepts every shape of a required
document, per `design@core@reserved-anchors`, so where the text names both shapes, both are
references. A path that varies by project, and an illustration, are placeholders in
angle brackets. The command a project runs is written as the placeholder that the install fills
with the project's declared command, per `design@core@declared-command`. A literal the checker
would misread is a delivery substitution, which the build fills.

content/ and the snippets are in the walk, so every reference they hold is checked against this
repository. A reference to an entry that resolves here passes the check and would dangle in every
installing project: the reviews of each change, and the release's hand check over the installed
copies, judge that, until a mechanical check exists, which
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` tracks.

### An installed skill names only installed skills and the project's own `##no-external-handoff`

An installed skill or agent hands off only to another skill or agent of the set its version ships,
or to the project's own skills. It never names a plugin or a skill from outside the set. A skill
outside the set may be absent in the project, or present at a version that contradicts the
installed one, and either breaks `goal@knowledge-architect@agents-get-a-complete-workflow`, under
which no two parts of the workflow contradict each other.

The set is written across several changes before a version is released, so a skill may name a
member of the set by its installed name before that member lands. No release ships a name its set
does not hold.

### A skill is named by its activity, as a noun of one or two words `##naming-rule`

An installed skill is named by the activity it covers, as an activity noun in common usage, in one
or two words after the installer's prefix: planning, review, issue-tracking, decision-recording.
The unit is the activity because `goal@agent-skills@one-skill-per-activity` gives each activity of
the workflow one skill.
It is never named as an artifact the activity writes, because the text names both: a skill called
"decision-record" would share its name with the design homes and rejected alternatives the
design skill calls the decision record, and one called "open-issues" with the issue directory. One
exception is accepted: agent-configuration is named like the configuration it edits, because the
owner judged it the clearest name for that activity. A name built on a verb form, such as
discussing-design-decisions, reads as a sentence and grows long. The rule binds a project's own
skills too, through the agent-configuration skill.

### The design skill is named design `##design-skill-name`

The skill is installed as `knowledge-architect-design`, under
`design@agent-skills@naming-rule`. The installer's prefix places it inside knowledge-architect, so
the name is never read alone, and there is not much need to tell it apart from visual design. It names no domain: the owner uses the skill for programming, for game design, and
for designing names and rules, always to advance a project. Three names lost: "project-design",
because in project-management usage it often means shaping a project's plan, which the planning
skill does; "decision-design", withdrawn by the owner, since the skill does not design decisions;
and "discussing-design-decisions", which read as a sentence.
"design" also names the design register, which the design skill does not write: in prose, the noun
that follows, "the design skill" or "the design home", tells them apart.

### Bounded work takes a path inside the design skill, after its grounding `##bounded-path-in-design`

Whether a request is a design question or bounded work, a clear requirement whose main risk is
over-building, is known only after the grounding of the design skill's loop step 1, its search for
standing entries included. The work is bounded when it reverses no recorded decision, every
decision it makes fails the entry tests, per `design@agent-skills@a-head-is-owed-by-an-entry-test`,
and no second defensible shape survives the nearest-rival test. The skill then sends one message,
the proposal with its nearest rival, the strongest open reading set aside, the consequences and the
default, and waits for the owner's word; an argument in the reply returns the work to the loop. The
commit that implements it carries the proposal and the owner's words verbatim. The skill's
description reaches a session before it starts a requested change whose design is not settled.

The owner's argument: "there is no way to determine whether a task is "bounded work that does not
change the project's design" without going through the grounding steps of the design skill." The
grounding gives bounded work the search for standing entries that no step gave it, and applying the
entry tests at classification keeps a head from being written for a decision that earns none. The nearest rival, a separate installed skill for
bounded problems, would repeat the grounding to classify at all; it is in the rejected
alternatives. A bounded path names no skill outside the installed set, per
`design@agent-skills@no-external-handoff`, and serves `goal@knowledge-architect@the-owner-decides`:
the owner rules on every proposal.

### The design skill is written for frontier-tier models only `##frontier-tier-only`

The design skill is not simplified for smaller models. A smaller model takes part in the
workflow as an implementer of what the owner and a frontier model decided, not as the owner's
counterpart in the discussion. So "a smaller model would not follow this" is not an argument for or
against any wording of the skill. Its description states "Requires a frontier-tier model
(Opus-class or stronger)": a model below that bar produces the format without the discipline, and
the description is the only text an installer reads before the first run. The evidence is a
scripted four-turn discussion run on three models, described in `path@agent-skills@README.md`: the
smaller one reproduced the ledger's format, and it endorsed a weak proposal, invented states outside
the closed set, and dropped open threads between rounds.

### The design skill structures how a discussion is conducted, never what or how much is proposed `##structure-the-flow`

The skill fixes the flow: proposals are argued, threads carry states, and closure waits for the
owner's word. It never fixes what may be proposed. The failure it was written against is the
solution quota: a mode that asks for two or three options gets two or three, and where one shape
applies, the others are fabricated to fill the count. A fabricated alternative makes a real
proposal appear to have won a contest that never happened, and spends the round that could have
tested it. The instructions that implement this are "never pad with alternatives to reach a count"
and the nearest-rival test, under which "no rival worth naming" is a claim to test. An edit that
specifies what must be proposed, rather than how a proposal is argued and tracked, works against
this entry.

No bound is set on how much is proposed either: on the number of threads, of proposals, or of
rounds. On the owner's argument, what a discussion needs depends on what its grounding,
investigation and experiments bring, and nobody knows that in advance; a bound fixed before the
discussion decides its size before its content, against the open discussion the skill exists to
run. A path chosen by the cost of reversal, or by where the work lands, changes where the
deliberation is kept, never how large the discussion may grow.

### Only a decision that creates a head, contradicts one, or outgrows its title is argued under the design skill `##new-or-reshaped-head-needs-design`

A decision goes through the design skill, whatever activity met it, when it creates a design head,
contradicts a statement of one, its argument included, or extends one beyond what its title
states. In the third case the title is reworded to state both decisions, or, where no title can,
the addition gets a head of its own. A title states a decision only while it is false of the
nearest rival it beat, so a reworded title passes that test for each decision under it, and a
title made generic enough to cover both fails it. An addition within what the title states, which
contradicts nothing, is recorded directly, with the owner's words quoted where they gave a ruling.
A change that relocates or rewords recorded decisions, a split of a head included, and adds or
removes none, is not a decision and needs no design skill; the routing and fidelity-of-relocation
review axes judge that it adds or removes none. A choice that earns no durable text stays in a code
comment and a commit message.

The decision-recording skill triggers once a decision has been made, so a new decision met during
an issue fix or a review repair otherwise reaches the record with no argument, no rival weighed,
and none of the design skill's rules for reading the owner's word. This serves
`goal@knowledge-architect@the-owner-decides` and
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. An addition inside a head's stated
scope carries less risk: the decision the head records, and its argument, stand, and the title
still tells a reader of the outline what is decided. The rival, every decision that earns durable
text through the design skill, puts such an addition, and every relocation, to a full discussion
whose outcome the head already records. A test on the size of a change lost: a one-sentence
reversal is the change that most needs the discussion.

Two texts deliver it. The design skill's description names the symptom. The decision-recording
skill sends a decision in one of the three cases, not argued, back to the design skill before its
text is written. `tripwire@agent-skills@head-created-without-deliberation` watches whether they
reach a session in time. A line in the primer lost to `design@agent-skills@primer-limit`: once the design
skill's description carries the symptom, a skill delivers the rule.

### The design skill guards the outcome of a discussion, and its tables are a display `##outcome-over-display`

The threads, states and tables exist so the owner can keep track when an agent produces much
content at once. A discussion that reaches a well-argued outcome with an imperfect table has
worked. A finding about the skill is worth an instruction only if it corrupts the outcome: what
gets recorded, what gets built, or what the owner believes was decided. A finding that makes only an
intermediate display imperfect is not, since agent behaviour is not deterministic and an
instruction bought for display fidelity costs more rules than it saves. Together with
`design@agent-skills@expectation-set-bounds-scope`, this entry bounds which findings become text.

## How the installed text changes

The measurements and incidents these entries cite were taken in the designing-together repository,
whose record is docs/decisions.md on its branch next.

### The record of the installed text keeps intent and coherence, not wording `##instruction-record-is-minimal`

A decision about the installed text is recorded only where it keeps the owner's intent on record, as
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks, or keeps two parts of the
workflow consistent, as `goal@agent-skills@one-skill-per-activity` asks; or where another entry test
admits it, such as a reason several installed texts must respect. A rewording, and the rationale for how one instruction
is phrased, is not recorded. Such proposals can be made without bound, unlike a technical decision,
which costs implementation work. The commit message carries a rewording's argument.

The owner's intent is on record where §2's test 4 of the installed decision-recording skill holds, per
`design@agent-skills@a-head-is-owed-by-an-entry-test`: the owner, asked, confirms that a ruling an
agent could reverse as a small fix, or an argument the owner wants kept, records the owner's intent;
an approval of the agent's proposal does not.
Two parts of the workflow are installed texts, or an installed text and the checker it must agree
with, as the planning skill's sections agree with what the core checks; a decision of this
repository alone is not one, since the installed text names none of this repository's conventions.
Installed texts alone would leave a skill free to disagree with what the checker enforces, which an
adopting project meets as a check that fails on a document the skill told it to write. A reason that passes no test goes in a
`%%` comment at the instruction it explains, per `design@agent-skills@shipped-text-line-comments`.

### An instruction is added to an installed skill or agent only on evidence from real use `##additions-need-real-use`

Real use originates an addition: a behaviour seen in a real session, produced unprompted or asked
for by the owner mid-session, with the owner naming what the session would have lacked without it,
and a one-sentence statement of the mechanism that produced the need. An unprompted behaviour shows
the gap, not the wording that fills it. A review finding originates an edit only where its defect
is provable by reading: a contradiction, a broken trigger, a factual error. A finding that predicts
a behaviour is parked as an issue, which states what a real session would have to show. Published
literature and synthetic scenarios originate no edit, and no run is built to observe how an agent
follows the workflow's instructions, per `design@agent-skills@synthetic-evidence-not-built`. The
skill text carries no size budget: an edit is judged on whether it changes behaviour, and two real
sessions held the design skill's full ledger discipline at several hundred lines without drift. The rule derives from
`goal@knowledge-architect@the-workflow-improves-through-real-use`, which is stated for the whole
workflow, so it covers every installed skill and every installed agent: a reviewer agent grows
the same way, one predicted check at a time. The evidence behind it came from one skill, and the
skills and agents forked from thaum were never edited under it: one that needs a different
standard argues its exception.

### No run is built to observe how an agent follows the workflow's instructions, and a run is admitted only when its verdict reproduces `##synthetic-evidence-not-built`

A run built to produce evidence about the workflow is admitted only when its verdict reproduces:
the same run, repeated, gives the same verdict, at a cost low enough to repeat it. The verdict may
read text an agent produced, as long as it reproduces. What an agent decides in following the
workflow's instructions does not meet this test: its verdict varies between runs, and it would mean
something only under a statistical design. So no run is built to observe it, for any of four
purposes: to originate an edit, to choose between shapes, to accept a piece of work, or to check
that a text runs as written. An agent's decision is presumed not to reproduce, since showing that
one does would take the repeated runs this rule refuses.

- **Admitted:** the harness's mechanics, such as a skill loading when invoked by name, a hook
  firing, a tool being granted, or a hook's injected text reaching the context; one dispatch of a
  new agent to see that its tools are granted is one. Also the installer's and the checker's
  behaviour, tested over mock projects.
- **Refused:** whether an agent invokes a skill from its description, whether a reviewer finds a
  given entry, whether an agent follows a step, and the judgement of a new agent's report against
  its instructions.
- **Always admitted as evidence:** a finding from a retrospective, and a misbehaviour observed in
  real use. A finding that comes from a synthetic run anyway is used only for the part a reading
  confirms.

The rule derives from `goal@knowledge-architect@the-workflow-improves-through-real-use`, which states
it for the whole workflow. The owner's argument: "the synthetic evidence is too unreliable. It would
need at least statistical analysis, which is too costly, and a proper mock task cannot be created
on a whim, it needs design and testing itself." One run of an agent is a sample of one. The
evidence the rule gives up is supplied otherwise: retrospectives of real work reach this
repository, and this repository installs its own skills, so its own sessions use each change
before a release reaches a consumer. The cost accepted is that a defect in a new text is found at
its first real use, not before its merge. Two instances in this repository grounded the rule. A
spec's acceptance criterion, a replay of a search agent over another project's history, was ruled
out at the step that ran it: "From experience, they are not very reliable, and drive the workflow
toward wrong directions more often than good ones." A slice's criterion, a subagent classifying
decisions written for the check, was dropped at the slice's design audit. The boundary is a property
of the verdict, not a list of subjects: the owner asked for "a wider category that matches my
intent", since "narrow wordings … reduce the underlying intent to something lesser". The nearest
rival, a list of exempt subjects naming harness mechanics, leaves out the installer and the checker.

### An installed instruction adds a capability and the judgement to use it, not a rule or a structure for one interaction `##capabilities-not-structure`

No wording makes a methodology self-enforcing: a rule set edited toward leaving no gap grows one
rule per observed interaction, and each new rule creates surface against the rules already there.
In one revision of designing-together, 34 edits produced 10 interaction defects, nearly all between
rules that scripted one exchange. So a gap is weighed by its kind. A capability gap is a move the
agent does not have, such as a way to investigate, to discriminate between positions, or to record
what the next session needs; filling it is worth text. A conformance gap is a move the agent has and
might not make; filling it is worth text only where the default behaviour is systematically wrong,
not occasionally absent, as the design skill's rules against agreeing without testing
are. When both readings fit, the capability form is written: a tool and the judgement to use it,
not a script for one interaction.

A structure an instruction imposes on the work has the same cost as a rule added for one gap: a
fixed sequence, count or mapping between a unit of the work and a unit of its record meets another
instruction that needs that unit to vary, and the session is left with two instructions it cannot
both obey, and less room to work on its own. So the installed text imposes no structure the work
does not need, per `goal@agent-skills@installed-text-leaves-room-to-judge`, which gives this
argument a goal's weight, and a contradiction a structure causes is answered by removing structure
rather than by adding a rule. An instance: a plan whose steps are its commits meets the primer's
instruction that a fix met outside the task takes a commit of its own, which is why a spec's steps
are no unit of the history, per `design@agent-skills@document-vocabulary`.

### An installed skill's scope is bounded by a stated expectation set, which the retrospective carries `##expectation-set-bounds-scope`

An installed skill may state what it expects of the owner: the assumptions about the owner's
behaviour and the project's shape that the skill is built on, each naming what degrades when it
does not hold. The set is the scope test for every proposed edit of the skill: a finding that
describes an owner's behaviour outside the set is not a gap, and no instruction is written for it.
The set scopes the owner's behaviour only. A finding that two installed instructions leave no move
satisfying both is always in scope, because `goal@agent-skills@one-skill-per-activity` is met only
while no two installed instructions contradict.

The sets live in the retrospective skill, the one installed activity that judges whether something
is a defect of an installed skill. The retrospective runs in the installing project, which holds the
installed files and not this component's README, so a set the retrospective applies has to ship. A
set inside the skill it bounds would be read on every use of that skill, and an agent reading it
would check the owner against a list. The README restates each set for the owner, who is the one it
describes. It works with `design@agent-skills@additions-need-real-use`: that entry says what evidence
admits an addition, and this one says which gaps are worth admitting. In the designing-together
repository, a rule for an abandoned discussion was argued, approved, written and reverted in one
session, and four further review findings about the user's behaviour were queued behind it on the
same reasoning. Only the design skill and the setup skill state a set so far:
`issue@agent-skills@expectation-sets-for-the-installed-skills`.

## The workflow the skills carry

### A decision is recorded when the work that implements it lands `##harvest-after-implementation`

A decision is written into the design homes in the change that lands the work implementing it, not
when the spec is written, because a design home holds built intent, per
`design@agent-skills@design-home-is-built-intent`. While the work is open, the spec or the
milestone document, on the main branch or on the work's, is the only place the decision exists. A decision that
no work implements, such as a policy, is recorded when it is made.

### Every item the owner rules on by name and that carries no slug gets a label, of a prefix of its own kind where the label reaches a committed document and of the common `Q` where it does not `##ruled-items-labelled`

Every item the owner is asked to rule on by name, and which carries no slug when it is put to the
owner, gets a label: a prefix of capital letters naming its kind, and a number. Numbers run from 1
within each prefix, in order of appearance, and are never reused within the record that carries
the ruling: a discussion, a resumed session included, a file, or a message. An item that gains a
slug later keeps its label beside it. A label that reaches a committed document, even one that
leaves later, such as a plan document, takes a prefix of its own kind, distinct across the
installed workflow. A label used only in the conversation reuses the common prefix `Q`. Every
distinct prefix is listed here, and an edit that adds one checks it against the list:

| prefix | items | the committed document it reaches |
| --- | --- | --- |
| `T` | tripwires put to the owner | a plan document's Premortem section |
| `AC` | acceptance criteria put to the owner | a plan document's Premortem section and its defaults |
| `D` | a plan document's defaults awaiting the owner | its "Defaults awaiting the owner" |
| `W`, `C`, `P` | a retrospective's findings | the retrospective's files |
| `Q` | entry test 4's questions, and every label that stays in the conversation | none |

The owner asked for it, from a premortem message that mixed causes, tripwires and findings: "it is
often quite hard already to rule on tripwires, because I'm never certain how many there are and
what they are about." A ruling given in words then matches items by guess. The owner asked before
for entry test 4's questions to be numbered, per `design@agent-skills@a-head-is-owed-by-an-entry-test`,
and judged the pattern recurring: "this should cover everything that I have to rule on by name". A
label that reaches no document reuses `Q`, on the owner's argument against "searching for unique
letters, which will necessarily end up needing more letters than available". The structure passes
`design@agent-skills@capabilities-not-structure`, since the owner named the lack and the label's
scope follows the record that carries it. Each installed skill that asks for such rulings restates
its own prefix where it asks.

### A design home holds built intent, and a plan document holds unbuilt intent `##design-home-is-built-intent`

A design home holds the design as built and its reasons, and the code is checked against it. A plan
document holds decided design that is not built yet, and each decision moves into the design home at
the landing of the work that builds it. A decision that no work implements counts as built intent,
since nothing waits to be built. The rival, a design home holding intent built or not, needs a
marker on every entry to tell the two apart, and checking the code against an unbuilt entry reports
a defect in code nobody has written. Unbuilt intent has a checked home because plan documents are a
structure the checker reads, per `design@core@plan-register`. A head can be wrong, and while it
diverges from the code it still prevails: the divergence closes when the code meets the head or when
the head is reversed, never by following the code, and work that goes on meanwhile, on the owner's
word, builds on the head. Without that, a session that must keep working would build on whichever
side it met first, and a divergence would close silently in the code's favour, against
`goal@knowledge-architect@agents-work-without-drift`. This serves
`goal@knowledge-architect@documentation-stays-consistent`.

### A reason recorded at the code binds as intent at its scale, below the design home `##local-intent-binds`

The primer's intent-and-claims rule has three classes. A design home is authority. A reason recorded
at the code, an inline comment saying why the code is shaped so or the message of the commit that
argued it, is intent at the scale of that code: it binds as a presumption, a change that defeats it
argues against that reason in its own message, and where it conflicts with a design home, the design
home prevails, as in any divergence, per `design@agent-skills@design-home-is-built-intent`. Between
a comment and a commit message, the comment is the current statement. A claim about the code as it
stands goes stale and is verified. Before code is removed or reshaped as unneeded, its comment and
the message of the commit that introduced it are read: absence from the design home is not evidence
that code is superfluous. The decision-record reviewer reports a diff that defeats such a reason
without arguing against it.

The owner's argument: "Since the workflow is instructing that "code follow design", anything that is
not argumented in design records could end up judged "superfluous" and deleted autonomously." Fewer
heads, per `design@agent-skills@a-head-is-owed-by-an-entry-test`, leaves more intent in comments.
Without a class of its own, an inline comment reads as a claim about the code, to verify and
discard; and keep-or-change's rule on an implementation coincidence applies only while an incumbent
design is evaluated. A comment is part of the code, so the head prevails over it. The order serves
`goal@knowledge-architect@agents-work-without-drift`. In a project whose comments the checker does
not read, a reference in a comment is not checked, which weakens this record there:
`issue@core@references-are-read-in-markdown-and-rust-only`.

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

### A decision earns a design head only when an entry test passes, and the heads stay few `##a-head-is-owed-by-an-entry-test`

A decision earns a head in a design home only when one of the four entry tests of the installed
decision-recording skill passes: reversing it would change an interface others consume, a type or
a signature crossing a separately built unit, a file format, a document grammar or a command line;
the same reason must be respected at more than one site, or at none, a name or a path repeated for
consistency being no reason; its argument turns on the behaviour of something outside the project,
an external specification or a tool's behaviour, documented or measured; or it records the owner's
own intent, a ruling an agent could reverse as a small fix or an argument the owner wants kept,
which the owner confirms when asked, since an approval of the agent's proposal or a hedged
statement is not intent and only the owner can tell. Every other
decision lives in a comment at its code, a `%%` line for the installed text, and in the commit that
argued it, where it binds as intent at that scale, per `design@agent-skills@local-intent-binds`.
For a decision about the installed text, `design@agent-skills@instruction-record-is-minimal` admits
one case the four tests do not: one that keeps two parts of the workflow consistent.

Fewer heads is the principle, as long as no critical intent and arguments are lost, in the owner's
words. Every grounding reads the design homes whole, and a human overseer reviews them; the session
that changes local code reads the comment at that code, not a design home it would have to think to
search. Test 2 is where the checker's reach ends: a reason that several sites must respect needs a
home each site can cite, and a comment cannot be cited, while a policy or an absence has no site at
all. Test 1 covers every interface others consume, not only a type or a signature: a head about a
grammar, a file format or a command binds every adopting project, and under a strict test 1 such a
head enforced in one module would pass no test. Test 3 counts a tool's behaviour measured as well as
documented: a measurement costs as much to take again as a documentation reading to derive again,
and a documented-only test 3 fails where the documentation is silent. Test 4 is the owner's: "Me
answering "yes" or "approved" is not ground for recording a design decision, it is not "intent"
from me, it is simply me answering your question "should I do that?" The key point is that my
ruling has to carry **my** intent, not just acceptation/approval of your proposals." On a hedged
word: "Under (b), anything I says in a discussion of the design skills comes out as "my intent", as
I read it. This should not be the case." And on who judges it: "maybe the correct solution would be
to ask the owner, rather than judge yourself. It would be more reliable IMO", ruled with "Agreed on
this shape. The questions should be numbered (Q<N> or similar shape), so I can answer each one
quickly without confusion." The agent's own judgement of the owner's intent, its rival, misread a
hedged proposal as a ruling twice in the session that made the test. Its other rival, any
recorded ruling of the owner, admits nearly every head harvested from a discussion, since the
workflow asks the owner to confirm most decisions. The decision serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: a later session can tell what it
may change and what a change costs from the head where one is owed, and from the comment where none
is. An audit of the 178 heads the design homes held, against the first three tests, found
over-recording below one head in five where the owner read it. The nearest rival of the principle,
a head for every decision discussed with the owner, is what a session did for a mechanism carried
at one site by its comments, and the owner judged the head unneeded. Within test 2, "the same
statement at more than one site" admits a path repeated for consistency, which the owner judged no
reason to keep a head. The wider test 2, "constrains work that has not been built", admitted nearly
every head under a wide reading and almost none under a narrow one, and the design homes hold
built intent, per `design@agent-skills@design-home-is-built-intent`. Whether the heads cost a
session more than they save is not measured:
`issue@agent-skills@the-retrospective-counts-no-review-cost`.

### An alternative earns an entry by the recording tests, not by having lost `##losing-alternatives-filter`

A losing alternative earns an entry in the rejected alternatives only if it passes the tests of the
installed decision-recording skill: it would change a signature crossing the boundary of a
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

### A tripwire's wording is the agent's, and the owner rules only on whether its cause is watched `##tripwire-wording-is-the-agents`

The owner's word on a premortem's cause, per `design@agent-skills@premortem-tripwires-on-the-owners-word`,
records that the cause is watched. The tripwire's firing evidence, response and re-entry point are
reworded when it is written, or on a review finding, on the agent's judgement, and each change is
listed to the owner at the end of the turn. A tripwire that watches another cause is a new one, and
needs the owner's word. A ruling that bound the wording would make every repair of a tripwire a
question to the owner, against `goal@knowledge-architect@agents-work-without-drift`, which asks for
the owner to rule on decisions rather than to supervise their wording. The rival, a change to a
tripwire's firing evidence treated as a material finding, puts each such repair back to the owner.

### A design thread's slug becomes its entry's slug `##thread-slug-is-entry-id`

A thread of a design discussion is named by a slug minted in the entry-id grammar and checked for
a collision with the entries of the Component before it is used. In discussion prose it is written
plain, with a hash sign before it, never as a backticked span, which the checker reports as the
retired slug form once an entry carries that slug. When
the thread is approved and its decision earns an entry, the entry's heading ends with the same
slug, except in the case below, so the spec, the commit messages and the design home name the decision with one identifier.
In the plan document the thread is an item, a level-three heading ending with the same slug under
the Threads section, cited from inside that document only, per `design@core@plan-items-by-section`.

A thread is named by the decision it would record if approved, not by the change it proposes. A
thread's slug is minted in the round it opens, and the approved shape can drift from the proposal,
so the rule has one exception: when the thread's slug misdescribes the decision as approved, the
entry takes a slug that names the decision, and the plan document's harvest row, or the commit
message on the in-change path, states the pair. A slug that misdescribes its decision misinforms
every reader of every citation, and the decision-recording skill's alignment rule orders a rename
in that case; without the exception, the two rules would give opposite orders, against
`goal@agent-skills@one-skill-per-activity`. The rival, renaming the thread during the discussion
once it drifts, lost: every earlier delta carries the old slug, and the assembly that reads them
from the transcript, per `design@agent-skills@ledger-from-transcript`, would read one thread as
two.

### The issues and tripwires a piece of work bears on are searched by an installed agent of its own, apart from the standing-state reviewer, in groups of at most 60 entries each read whole `##standing-entry-search-agent`

The installed agent `knowledge-architect-standing-entry-searcher` searches one group of a
project's issue and tripwire entries for those a piece of work bears on. Its description carries
the dispatch rule, since a dispatcher reads an agent's description and never its body: count the
rows of the issue and tripwire listings, send the fewest agents that keep every group at 60
consecutive positions or fewer, all in parallel, with group sizes that differ by at most one, and
brief each with the work, the seeds and its group. Equal groups keep a last group of a few entries
from costing an agent of its own while another carries the full bound. A seed is a
decision or a goal the work names. Its body fixes the order of the rows, reads every entry of its
group whole, follows each seed through `show` to the entries that cite it, which it always
returns, and judges every entry of its group against the work, leaning to recall. It returns every
entry of its group in one of two lists, the entries that bear on the work, each with a reason, and
the entries judged unrelated, so that a silent report cannot be mistaken for a skipped one. The
session reads whole every entry returned, never from its reason line. It relies on the checker
working as intended: `show` prints an entry's file and definition line, and the references to it.

The search runs in a subagent so that the main session spends its context on the entries the work
bears on, per `goal@knowledge-architect@agents-get-a-complete-workflow`, and the session still
reads each of those whole, as the owner asked, since the session acts on the entry and not on a
summary of it, per `goal@knowledge-architect@the-owner-decides`. It is an agent of its own rather
than the standing-state reviewer in a second mode: that reviewer reads every entry by rule, the
search filters by relevance on purpose, and one text holding both standards lets a session apply
the wrong one. Every entry is read whole rather than by its title, since a title shows neither a tripwire's firing
clause nor a deferred trigger. It is read by several agents rather than by one: one agent's load
grows with the project, and one agent cannot judge so many entries reliably, in the owner's
judgement. The unit of a group is the entry rather than the anchor: in thaum at its commit
ef21314, one Component holds 51.9% of the bytes of the standing entries, so a group per anchor
bounds nothing. The bound of 60 rests on the mean size of an entry there, 1.6 KB, and on no
measurement of recall. Both figures are re-taken with `git ls-files` over the issue entry files
and the tripwires homes, then `wc -c`: summed by the anchor that holds each file for the first,
divided by the rows of the issue and tripwire listings for the second; real sessions measure it, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`.

### The search runs at the design skill's grounding, of a discussion or of bounded work, and at the design audit of a milestone slice or of a spec, over every issue kind and every tripwire `##standing-entries-searched-before-the-work`

The installed design skill dispatches the search at its loop step 1, with the question as the work
and the decisions and goals read so far as seeds, for a design question and for bounded work alike,
per `design@agent-skills@bounded-path-in-design`. The installed planning skill dispatches it at
the design audit of a milestone slice, §7 point 2, with the slice's spec and the milestone document
as the work and their decided entries and citations as seeds, and at the design audit of a spec
where one runs, with the spec as the work. The audit lists as a gap every
standing entry the slice's code bears on: a tripwire whose firing condition, or a deferred trigger,
the planned code meets, and an issue of any kind the slice's code touches, closes, makes worse or
depends on. The slice's grounding, §7 point 1, reads no issues and no tripwires, since the audit's
search covers every anchor. Work that does not go through the design skill sends no search: the
standing-state review reads its deferred triggers, per
`design@agent-skills@conformance-before-every-merge`, which this search adds to and never
replaces.

The occasions are those where design is discussed, so that an entry the work bears on costs a
ruling before the work rather than rework after it, per
`goal@knowledge-architect@agents-work-without-drift`. The evidence is real use, per
`design@agent-skills@additions-need-real-use`: in thaum, the standing-state reviewer found, after
the work, issues the changes bore on and tripwires that had fired, especially in the slices of large
milestones, and the repairs and decisions this forced could have been taken before the design and
the implementation, in the owner's account. The audit reads every issue kind because those late
findings included issues the changes touched, not only fired tripwires. The slices of thaum whose
reviews found such entries had their audits before the installed planning skill told an audit to
read the tripwires and the deferred issues, and the reviews of the two slices audited after it
found no entry missed. The
owner judged that the work before that change did not differ from the work just before this
search, so the evidence stands; a stream of audits that read the standing entries in the session
and miss none would weaken it. A search at the reviews of
a plan document lost to a second search when convergence is proposed, before the premortem: those
reviews run only where a discussion wrote a plan document, and a search at convergence covers a
decision taken in the change under way too. The search at convergence is parked, with a tripwire,
since grounding and the audit are where the owner wants the search.

## How documents point at each other

### A reference is written where the text would have to be revisited if the entry it names changed `##a-reference-claims-a-revisit`

Every citeable thing has one reference form, `<kind>@<anchor>@<id>`, and two mechanisms of the
checker give a reference its value. `cargo klarch show <ref>` prints every reference to an entry,
so what depends on an entry is computed from the tree and never written by hand. An entry that is
deleted or renamed dangles every reference to it, and `cargo klarch check` reports each one, so the
repair list a change produces is the list of texts that depended on what changed. A reference is
therefore a claim of dependence: this text is to be revisited when that entry is reversed, closed,
fired, abandoned or renamed.

**The test for writing one is that claim.** Where a change to the entry would leave the text
unaffected, the reference is decoration and costs a repair for nothing; where it would not, the
reference is owed, or the change reaches nobody. What follows from the test, by the kind of text:

| the text | names | so that |
| --- | --- | --- |
| a design head | the goal its argument derives a constraint from | `show` on the goal lists what abandoning it reopens |
| a design head | a decision of another component it depends on | a reversal reaches it |
| an issue entry | the decision it strains, and the goal it threatens when it does directly | `show` on a decision lists what is outstanding against it before it is reopened, and on a goal what stands between the project and it |
| a guard, a workaround, a stub or a test that pins behaviour an open entry describes | the issue it exists because of, in the comment at the site | closing the entry dangles the comment, so the site is revisited and the workaround removed |
| a tripwire | the decision it guards | a reversal dangles its tripwires |
| a rejected alternative | the decision it lost to | a reversal finds what the old winner displaced |
| a commit message | every entry it opens, closes, reverses or argues from | the commits gate judges it against the tree it was written against |
| a restatement of a directive | its home | a drift between the two is found from either end |

**An entry never lists what references it.** The inbound list is `show`'s to compute, and a
hand-written one is stale at the next reference written elsewhere.

**A reference in prose is checked wherever it stands**, a Rust comment and a fenced block
included; a string literal bound to a name yields none. So a comment in code naming an issue is
as live as a sentence in a document, and closing the issue reaches the code. This serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`.

### A sentence about the past whose reference dangles is rewritten to the present or removed, and a quotation of the owner is kept with a reference beside it `##a-past-sentence-is-rewritten`

When an entry is deleted or renamed, a sentence that recorded its past, such as what an earlier
step wrote, is rewritten to state the present, or removed, and its history stays in the commit
messages. Retargeting it to the new name would make it false, and plain text would take it out of
the check, per `design@agent-skills@plain-text-is-no-repair`. A verbatim quotation of the
owner that names a renamed entry is left as it is, with a reference to the current entry beside
it, since rewriting it would misstate the owner, against `goal@knowledge-architect@the-owner-decides`.

### Plain text is never the repair of a finding, and no instruction offers an unchecked form as the way to clear one `##plain-text-is-no-repair`

A form a writer can use is either one the checker judges, and the workflow recommends it, or one
the checker does not read, and the workflow never directs a pointer into it to clear a finding. So
no finding's repair, and no installed or project instruction, offers an unchecked form as the way
to clear a finding. A repair names a checked form: the right anchor, `path@elsewhere@<path>` for a path this
tree does not hold, `planned@<anchor>@<path>` in a plan document for a path its work will create,
per `design@core@planned-path-form`, an angle-bracket placeholder for an illustration, or a
rewrite of the sentence.
The one exception is text in the checker's syntax that no checked form expresses, written without
backticks beside a reference to the issue entry that records the missing form, per
`design@agent-skills@checker-syntax-without-backticks-names-its-gap`. Prose that mentions a
directory without asking the reader to follow it is not a repair and is outside this head.

The argument: an unchecked form that clears a finding clears it for good, so a habit of writing one
empties the check while every run still passes, against
`goal@knowledge-architect@documentation-stays-consistent`. Two narrower stances of the checker
make the same argument: `design@core@the-regime-has-no-opt-out`, where no declaration exempts a
document from a rule, and `design@core@reserved-anchors`, where the escape anchor is refused on a
path this tree holds, since it would otherwise silence the finding on a real path.

A label beside a checked pointer is not such a form. A plan item named outside its plan as #<id>
stands beside a citation of the whole plan, because an item reference is refused there, per
`design@core@plan-item-scope`.

### The plain-text escape covers only text in the checker's syntax, which is written without backticks only beside a reference to an issue entry that records the missing form `##checker-syntax-without-backticks-names-its-gap`

The checker cannot express every reference a project needs. This head covers text in the checker's
syntax, which would be read as a candidate if it were backticked: a reference,
`<kind>@<anchor>@<id>`, or a path of two or more segments. Where no checked form expresses what such
text points at, it may be written without backticks, and only beside a reference to an issue entry
of the writing project that records the missing form. Any other text that names something, in the
tree or outside it, such as another project's commit, an address on the web or a description in
words, is outside this head. Whether it needs a reference is decided by
`design@agent-skills@a-reference-claims-a-revisit`. A project that meets a gap of the checker itself opens that
entry in its own register, since a reference resolves only inside its own project, per
`issue@core@cross-project-references`. A need that a checked form already serves is not a gap.

The entry owes its `Why it matters` and its `What would close it`, so the escape is available and
never free. `cargo klarch show` on the entry lists every site, and closing it, once a checked form
ships and the sites are converted, dangles each one, so the conversion list is computed. Plain text
justified in a commit message alone lost: nothing finds the site again, and nothing revisits it
when the form ships. A generic checked opt-out marker lost too: one marker fits every finding, so
it becomes the cheap silence `design@core@reserved-anchors` refuses, while a gap concrete enough to
name is closed by shipping its own form.

The scope is the checker's syntax, so that a writer can tell from each span alone whether the head
applies, per `goal@knowledge-architect@agents-get-a-complete-workflow`: every sentence names
something, and a scope of "any pointer" would ask for an issue entry beside every mention of a
thing outside the project. The evasion the head exists to stop is a reference or a path with its
backticks removed, which this scope covers. A scope by target, any text naming something the tree
holds, lost: when such text needs a reference is already decided by the rule on references, and
whether a phrase names something cannot be decided span by span.

A commit named by its subject, as `design@knowledge-architect@git-flow` directs for a commit of the
branch, is outside this head. It names history as git names it. A reference resolves against the
entities of a tree, per `design@core@one-entity-table`, and a commit is none of them; the one
citation of a commit the checker judges is a branch commit's SHA, which
`design@core@branch-shas-are-refused` refuses.

## Plan documents

### The words: plan document, spec, milestone, slice, step, plans directory `##document-vocabulary`

A plan document is a spec, a milestone document or the spec of a slice, kept in the plans
directory, docs/plans/ at the project's root, whose path the checker fixes, per
`design@core@plans-dir-fixed`. The directory is named plans rather than planned: in common English,
planned work is intended or scheduled work, designed or not, which is the roadmap's content. A spec is the plan document of work done in one branch and one PR. A milestone is work across
several PRs with design sessions between them; its plan documents are its milestone document and
one spec per slice. A slice is a part of a milestone that is one branch and one PR, with its own
spec. A step is one item of an implementation sequence, in a spec or in a slice, and how many
commits it takes is the implementing session's to judge. The word "plan" alone never names a document: it would name the directory, a
document and a kind of document at once. The words follow common usage among developers, which the
owner made binding: a milestone groups the work toward one goal, as GitLab's milestones do, and a
spec says what will be built and how before the code exists, in the sense engineering teams give
the word. "Design doc", the closest common term, lost because "design" already names the durable
register. One word for both a milestone's part and an item of a spec's sequence lets a rule for
the one, one branch and one merge per part, contradict the other, a spec of several steps on one
branch. So the milestone's part takes its own word, slice, a part of the larger whole, and "step"
keeps the general sense it has in common usage and in the design skill's loop. Naming a spec's
items by a unit of the history instead, one step to one commit, lost: it fixes the plan onto the
history, against `goal@agent-skills@installed-text-leaves-room-to-judge`, and a commit the work
needs for another reason, such as a fix met outside the task, would then contradict the plan.

### One document per layer, and no snippet is authority `##spec-and-milestone`

The work of one PR has one plan document, its spec. A milestone has a milestone document and one
spec per slice, which hold the spec's sections between them, split as
`design@agent-skills@milestone-is-a-directory` says. A plan document is detailed about the
design and concise about the implementation sequence. No untested code snippet in it is presented
as authority: a snippet is labelled as an illustration of a shape. A spec plus a separate detailed
implementation plan lost: both carry the same decisions and the second drifts from the first,
against `goal@knowledge-architect@documentation-stays-consistent`, and the owner observed that
implementers force such plans' untested snippets into the code at any cost, copying their comments
verbatim. A detailed plan, if one is ever written for a less capable implementer, covers a bounded
amount of work and opens by saying it rests on assumptions.

### A milestone is a directory, its head a README, each slice a spec `##milestone-is-a-directory`

A milestone's plan documents are one directory under milestones/ in the plans directory, which the
checker makes an anchor named by its basename, per `design@core@plan-document-kinds`: the milestone
document is its `README.md`, a generated `index.md` lists the slices, and each slice's spec is a file
beside it, cited `spec@<milestone>@<slice>`. The head is a README because the checker
resolves a relative link only in a `README.md` or an `index.md`, per
`design@core@links-are-navigation-rows`, so the head can link each slice's spec as a navigation row.
A slice's spec is a spec, so it leaves when its slice lands, per
`design@agent-skills@spec-leaves-at-landing`: its decisions are then in the design homes. The
README leaves with the last slice.

The design is split across the documents from the start, by lifetime: a slice's spec holds what
only that slice builds, and the README holds what crosses slices or outlives one, every item cited
from more than one slice's document included. A README holding every slice's design is read whole
at every slice's grounding and grows with every slice: the README of the structured-plans milestone
held 1,624 lines when it left, counted with `wc -l` on `git show` of its deleting commit's parent.
The slice specs exist from the session that converged, so the audit of a slice applies its findings
in place, in whichever document holds the shape, and a new slice found at an audit gets a spec of its
own, as a scope change the owner rules on. The audit's commit message lists each finding with its
gap, its answer and the decision it follows from, so an edit in place loses no record of what the
audit found.


### The work of a spec takes a slice's procedure once, and its design audit only when a fresh session starts it or the main branch has moved `##spec-work-procedure`

The work of a spec, on its one branch, takes the points of the installed planning skill's procedure
for a slice once: the grounding, the claims and checks of each step, one review before the merge,
the report and the harvest in the commits that land the work, and the deletion of the spec in the
commit that completes its harvest. Its design audit runs only when the work does not start in the
session where the discussion converged, or when commits other than the spec's own have landed on
the main branch since the spec was written. An audit reads the design against the tree as it stands and recovers what a
session that did not witness the discussion lacks; a spec implemented by the session that wrote it,
on an unmoved main branch, likely gives it nothing to find: the owner doubts that an audit makes
sense there. Without a stated
procedure, a spec of several steps on one branch cannot tell which points of a slice's procedure
apply to which part of its work. The rival, an audit for every spec as for every slice, costs a
reading with nothing to find.
### A plan document leaves when its work lands `##spec-leaves-at-landing`

A plan document is deleted in the commit that completes its last harvest, and that commit's message
cites it by its kind, which resolves against the commit's parent. A plan document kept after its harvest is a second home for every decision it
carried, and it starts drifting at the first later reversal. Walked by the checker, its references
break at every reversal, and someone repairs a document about finished work; left out of the walk,
it is unchecked text that a grep finds with no marker that it is stale. Either breaks
`goal@knowledge-architect@documentation-stays-consistent`. Leaving the choice to each project lost
too: the installed planning skill would have nothing to say where a document's work ends, against
`goal@knowledge-architect@agents-get-a-complete-workflow`.

### A discussion whose work lands in the change under way keeps its deliberation in that change's commit message `##in-change-path`

The design skill has two paths, which differ only in where the deliberation is kept until the
harvest. The in-change path is open when the decision lands in the change under way: its work, or
its record, for a decision recorded when it is made, per
`design@agent-skills@harvest-after-implementation`. It writes no plan document. The message of the
commit that writes the decision's design entry, or that implements a decision that earns none,
carries what a plan document would: every thread with its final state, the owner's words verbatim
for each closure, the rivals that lost with their reasons, and the rulings on tripwires. The
premortem runs when reversal touches any of the four things the skill names: stored data, a
consumed interface, behaviour users have adapted to, and a decided thread that would reopen. The four never open the in-change path: a decision that is cheap to
reverse and whose work comes later takes the full path, per
`design@agent-skills@design-hands-off-to-planning`, as does any discussion that converges on work
that does not land in the change under way. Decided and unbuilt intent then has one home, the plan
document, per `design@agent-skills@design-home-is-built-intent`.

A plan document exists to carry a decision from convergence to a landing that comes later, and to
be reviewed before another session implements it. When the work lands in the change under way,
neither applies, and requiring a spec would make every decision met during a task cost a plan
document, which is what `design@agent-skills@new-or-reshaped-head-needs-design` must not cost to
be followed. The rival, a separate path for one decision met during a task, lost: it bounded the
discussion to one thread, against `design@agent-skills@structure-the-flow`. The cost, which the
owner accepted, is a long commit message when a discussion inside a task grows to several threads;
its length is not bounded, since a bound on it would bound the discussion.

### The planning skill writes the plan document, in the session that converged `##design-hands-off-to-planning`

The planning skill starts where a design discussion has converged on the full path, and writes the
spec or the milestone in the same session. A discussion on the in-change path has no plan document,
per `design@agent-skills@in-change-path`. The design skill ends at convergence, the premortem and
the owner's rulings on its tripwires and acceptance criteria, and writes no plan document of its
own. Its decisions are
recorded when their work lands, per `design@agent-skills@harvest-after-implementation`. One skill owns the document's shape: two skills describing the sections of one document
would drift apart, against `goal@knowledge-architect@agents-get-a-complete-workflow`. The session
matters because the discussion's records are that session's: its conversation, and the transcript
the harness keeps of it, which that session names exactly. A document written from memory in a
later session is written from a summary, and a summary loses the losing arguments and the
conditions of each closure. Where the harness keeps no transcript, the conversation is the only
record, and compaction can remove it.

### The planning skill assembles the plan document from the discussion's transcript `##ledger-from-transcript`

The planning skill assembles the document through a subagent that reads every transcript file the
discussion spans, the harness's log of the session on disk. The design skill's per-round delta is
the draft it reads, and the deltas are corroborated as written, since the owner read and answered
each. Where the harness keeps no transcript, it assembles from the conversation. The premise is
that the transcript keeps the records from before a compaction, which a summary of the
conversation loses. It holds on one observed compaction of one session log, where every user and
assistant record before the compaction boundary was still in the file;
`jq -c 'select(.type=="system" and .subtype=="compact_boundary") | input_line_number' <log>` finds
the boundaries of any log, and a count of the records before each re-takes it. The extraction can be
wrong, so the transcript reviewer checks every assembled document against the same files. The
rival, a draft ledger the agent writes to a file every round with the least effort, answers the
cost of a write per round and not the other two reasons of the rejected alternative it repeats: a
stale ledger stated with confidence is worse than none, and a file the agent writes is corroborated
only by the conversation, which compaction removes.

### Acceptance criteria live in the plan document of the work that judges them `##acceptance-criteria-in-the-document`

An acceptance criterion, a check on a recorded decision that only the work's built code can apply,
is written in its own section of the plan document of that work. Each names the decision it guards,
the step or slice that judges it, the observable that fires it and the response. Every landing reports on
the criteria it judges. When the document leaves, a criterion that recurs is proposed to the owner
as a tripwire and written on the owner's word, per
`design@agent-skills@premortem-tripwires-on-the-owners-word`; any other is deleted. A separate file
of criteria would hold statements about the same work with the same lifetime, drift from the
document, against `goal@knowledge-architect@documentation-stays-consistent`, and stay behind when
the document leaves. The plan document is written in the session that converged, so the criteria
and the document are born together.

The result a scheduled review is expected to give is not a criterion: passing the reviews every plan
document and slice owes is the baseline, and writing it as a criterion in
every plan document would be noise, and would become the habitual criterion in place of a specific
one that is harder to find.

### An acceptance criterion stands on the owner's word, as a tripwire does `##acceptance-criteria-on-the-owners-word`

The owner rules on whether an acceptance criterion is applied, and on the decision its firing
reopens. The agent words its observable, and lists a rewording to the owner at the end of the turn.
The design skill asks for the ruling at the premortem, beside the tripwires. A criterion first
proposed later, at the assembly, by a review or at an audit, is written marked as awaiting the
owner, with its label under the plan document's defaults, and is not judged before the ruling; the
work goes on without it, and one never ruled leaves with the document.

A criterion's response reopens a decision, and choosing the observable that may reopen a decision
is a weighing, which is the owner's, per `goal@knowledge-architect@the-owner-decides`. The owner
sets a criterion's threshold, and a tripwire, the same check for built work, stands on the owner's
word, per `design@agent-skills@premortem-tripwires-on-the-owners-word`, so a criterion's application
stands on the same word as its threshold and as a tripwire. The wording stays the agent's, as a
tripwire's does, per `design@agent-skills@tripwire-wording-is-the-agents`, and the label a criterion
is put to the owner under is `design@agent-skills@ruled-items-labelled`'s. Both acceptance criteria
that reached a plan document of this repository against a goal had no ruling: one was proposed at a
premortem, one while a slice's spec was written. The rival, no ruling and the design-conformance
reviewer alone, catches a conflict with the record, not a weighing.

### A plan document is committed before its reviews, and a repair lands after them `##plan-reviewed-as-a-commit`

The planning skill has a plan document's reviewers read its commit, not the working tree. The review
skill names what a reviewer reads as a commit range and gives each reviewer that runs a binary a
worktree detached at the commit under review; a document still uncommitted has neither, so the two
skills could not both be obeyed. A repair lands after the review, per
`design@agent-skills@review-repair-appended-or-folded`.

### A review repair is appended, and folded only where appending leaves an earlier commit failing `##review-repair-appended-or-folded`

The review skill has a repair land as a new commit after the work it repairs, which edits no
history. Where a project requires every commit of a branch to pass checks the repair changes, an
appended repair leaves the earlier commits failing, and the project's rule and this one leave no
legal move; the repair is then folded into the earliest commit it repairs, with a clean tree,
confirmed to have lost no content. The record of the review says what was folded, so the landing
history still says what the review found. The rule exists to avoid history edits, not to keep a
repair apart from what it repairs.

### A plan document's landing is not tied to its work's `##plan-landing-is-not-tied-to-its-work`

A plan document, a spec or a milestone document, is merged to the main branch in a pull request of
its own or with its work, whatever the time of its work. An owner
plans a feature that will not be built yet so that the work done meanwhile does not drift from it,
which needs the document where every session reads it, on the main branch; that serves
`goal@knowledge-architect@agents-work-without-drift`. What "spec" and "milestone" measure is the
work, one pull request or several, not where the document lands. The document leaves when its work
lands, per `design@agent-skills@spec-leaves-at-landing`.

The owner's intent: "It regularly happens that I will prepare a spec/design for a feature that I
won't be implementing right now, just to ensure that the intermediate tasks I will perform do not
get misaligned too much with that planned feature. Anyway, what I mean is that writing plan
document should not be constrained with when its implementation work happens." Asked whether it
records the owner's intent, the owner answered: "Q1 yes".

### A plan document lands before any work that changes what the project's per-commit gate checks `##plan-lands-before-gate-change`

The per-commit gate judges each commit of a branch, its tree and its message, with the checker at
the branch's tip. On one branch, that gate as the work's first commit changes it would judge the
commit that added the document, whose tree predates the change, and refuse it. A change that only a
check of the working tree sees, such as installed text that the checker compares on the working
tree alone, leaves the per-commit gate unchanged and is outside this rule. Keeping both on one branch would force
the fix the owner called absurd: the plan document committed after the work it plans. So that work
begins on a branch of its own, after the document is merged. This holds for a spec as for a milestone document, whose slice
that changes what the per-commit gate checks is the one this rule meets. It is the one exception to
`design@agent-skills@plan-landing-is-not-tied-to-its-work`, whose plan document may otherwise land
with its work.

### Undesigned work is an issue `##planned-work-is-an-issue`

Work that is known but not designed is a `todo` or `deferred` issue in the owning anchor, with its
leads in the entry. The plan document that schedules it closes the issue in the commit that adds
the document. `goal@knowledge-architect@structure-and-workflow-work-together` asks for one place for
what is open: the issue register holds the work, and the roadmap only orders it, per
`design@agent-skills@roadmap-orders-issues`.

### The roadmap holds only the order of known work, and every row is a checked reference `##roadmap-orders-issues`

A row of the roadmap cites an issue entry or a whole plan document, in the order the owner wants the
work done, and an unordered section may follow. The work stays in the issue register and the plans
directory, so what is open is still listed in one place, per
`goal@knowledge-architect@structure-and-workflow-work-together`; the roadmap adds only the order. A
row dangles when its issue closes or its plan document leaves, and the check reports it, so the
order cannot drift from the work unnoticed. The commit that adds a plan document rewrites the row
of the issue it closes, and the commit that deletes one removes its row. A row is added or moved
on the owner's word, since ordering work is a weighing, per
`goal@knowledge-architect@the-owner-decides`. The owner's reason for the shape: it builds the
roadmap from several register shapes using only references. The need is observed in real use: thaum kept a file
of its next milestones as an exception to the installed skill. A roadmap register of its own,
holding undesigned work, lost: two registers would each hold known, undesigned work, which needs a
routing rule and hides one of them from the issue listing.

### No record of landed work is kept `##no-progress-record`

No installed skill asks for a file recording what has landed. History lists every plan document
that left, and each landing commit says where its results live; releases are in the changelog. A
progress file would be a third document about the same work, with a lifetime of its own, against
the one place for what is open of `goal@knowledge-architect@structure-and-workflow-work-together`.

### A transcript reviewer checks that what a work's sessions established has a durable outcome, and rates a misstated ruling by what it changes `##transcript-reviewer-agent`

The installed agent `knowledge-architect-transcript-reviewer` reads the transcripts of the sessions
that produced a piece of work, named as a commit range, and checks two things. First, that each
decision of the owner, each argument that decided something, each finding of a reviewer and each
thing met outside the task has a durable outcome: recorded in its home, acted on by a repair or an
issue, or judged to need nothing with the reason written where the owner reads it. What lives only
in a conversation is lost when the session ends or is compacted. Second, that no ruling of the
owner is recorded in a state or a scope the owner did not give. Detail added inside a ruling, and a
wording better than the one the owner was shown, are not findings. The owner's words: "It bothers
me that it flags every small addition you make as a finding, even when they are clearly sane
deviations. I would like it to focus more in the other direction: whether something from the
transcript has been forgotten or not handled *durably*." The reviewer assumes no shape of
the work: a plan document is one home among the others.

It is an agent, not a line in a skill, because its standard and its extraction rule are fixed, and
the rule is learned from a failure: a filter on text substrings once dropped one of the owner's
messages. It serves `goal@knowledge-architect@the-owner-decides`. The dispatcher acts on a finding
that something has no durable outcome without waiting for the owner, and reports what it did; a
misstated ruling goes to the owner. The durability check serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. The check of rulings has shown
its worth: in the review of the change that installed the first skills, a reviewer briefed with it
was the one of six to find a decision recorded narrower than the owner's approval.

A misstated ruling is critical only when it is reversed, or when its state or scope changes what is
built or a load-bearing decision. A ruling recorded a little wider, narrower or firmer on a detail
that is not load-bearing is minor, still goes to the owner, and is no finding of the retrospective
once caught before the merge. Any other misstated ruling is major. The owner's words: "everytime, it was on details that were small and
not load bearing in my opinion", "the transcript reviewer is catching those issues reliably IMO. I
do not believe any amount of instruction anywhere would help improve more than the current
situation: LLM agents are trained to take the input they receive from humans seriously and follow
them closely", and "This kind of findings being flagged by the transcript reviewer once in a while
(for decisions that are not the most criticals, and with only a small deviation from my word, not a
complete reversal) is not a critical defect and should not be considered as such." The rival, every
misstated ruling rated critical, made a reviewer's report of a slip read as a critical defect, which
the owner ruled it is not; the slip is still repaired, and it is no defect of the workflow.

### A plan document records the whole discussion `##spec-records-the-exchange`

A plan document assembled from a discussion records every thread with its proposer and round, its
final state, the arguments on each side, the owner's rulings verbatim with their round, and its
relations. It applies to a spec, and to a milestone document with its slice specs, which share the
record by the rule of `design@agent-skills@milestone-is-a-directory`. `design@agent-skills@standing-argument-in-head` names the plan document
as the home of the deliberation while it exists, and `goal@knowledge-architect@the-owner-decides`
is served only where the rulings are recorded as the owner made them. On the in-change path, the
commit message carries the same record, for the same reason, per `design@agent-skills@in-change-path`.
The rival, a plan document
recording each thread's final state and resolution, left the rulings and the arguments to memory.
The cost, which the owner accepted by name, is a longer plan document to write and to read.

### An argument is an item, without a state `##arguments-as-items`

Each argument of the discussion is an item of the plan document, under its Arguments section, so a
ruling, a closure or a premortem cause can cite what decided it, and the standing argument a
harvest writes is a selection of named arguments rather than a rewrite. It carries no state: a
state would need a relation between many arguments and many threads, which the design skill
declines to track, and an identifier needs none.

### Where one argument ends is decided at assembly `##argument-segmentation`

The boundaries of the arguments are a judgement, made once, at assembly. The transcript reviewer
checks that no argument of the transcript was lost, and judges no boundary: a boundary drawn
differently loses nothing. The rival, arguments marked in each round, makes the extraction exact at
a cost paid every round.

### A leaving plan's citations are removed, and each citing plan gets a question `##retiring-plan-opens-issue`

A whole plan document may be cited from another plan, per `design@core@plan-item-scope`. When it
leaves, the session that meets the dangling citation, the one deleting it or the one rebasing the
citing plan onto the deletion, removes the citation and opens a `question` issue on the citing
plan: does it still hold now that the leaving plan is built, accounting for deviations or other
unplanned happenings? The issue cites
the citing plan, so it cannot outlive it, and its `Why it matters` cites what the leaving plan
harvested, since that plan no longer exists. It is a `question` rather than a `todo` because the
reading may find nothing to change. For a milestone, its next slice's audit reads it. The retiring
session revisits nothing itself: a revisit at that moment would redesign the citing plan at a time
chosen by another plan's landing, rather than at its own audit, where the owner rules on it as
`goal@knowledge-architect@the-owner-decides` asks.

The decision is recorded here because two installed skills apply it, the planning skill where a
plan document leaves and the issue-tracking skill among the issue kinds, and both act on the
dangling citation the checker reports: the same reason at two sites, and a procedure the installed
text and the checker must keep consistent, per `design@agent-skills@instruction-record-is-minimal`.
The argument is the owner's, against citing a plan's items from another plan: it "forces design
work at a moment that might not be the best".

## Reviews

### The standing-state review runs before every merge, and reads every tripwire and every deferred trigger `##conformance-before-every-merge`

The installed review skill sends the standing-state reviewer before every merge to the
main branch, whatever the change. That reviewer is the standing re-entry point of every tripwire
home and of every deferred trigger, as the installed issue-tracking skill states: it runs for every
change, where the search before the work runs only for work that goes through the design skill or a
plan, per
`design@agent-skills@standing-entries-searched-before-the-work`. A re-entry point that depends on whether a change looked
related to a tripwire is one that a change touching the guarded decision indirectly skips: the
reviewer reads every entry of every home, not the subset the diff seems to concern. This serves
`goal@knowledge-architect@documentation-stays-consistent`.

A deferred issue's trigger answers the question a tripwire's firing clause answers, what will make
someone do this, and the issue-tracking skill holds both to one test. So the reviewer reads every
deferred trigger against the change, as it reads every tripwire, and reports a trigger the change
meets as a finding, whose repair is the work the issue names or the owner's ruling. Without it, a
trigger met by work that does not go through the design skill has no reader at all. The design skill
hosts a search before bounded work, at its grounding, per `design@agent-skills@bounded-path-in-design`.
A search before every other piece of work stays parked, on the owner's word: its one host would be
the primer, which holds only what every session needs, per `design@agent-skills@primer-limit`, the
evidence for it is one instance, and a line sending every session to search before any work is a
conformance rule. In thaum, a move of the pinned checker met the trigger of a deferred issue that
named that move, and the session read it at no step.

### A fresh reviewer reads every plan document against the goals, the design heads and the rejected alternatives of the Components it touches `##plan-read-against-the-record`

At every moment the installed planning skill sends a plan document's reviewers, it sends the
design-conformance reviewer beside the cold implementer, the code-claims reviewer and the
transcript reviewer. It reads the document against the goals homes, the design homes and the
rejected alternatives of the project's root and of every Component the document touches. It
reports a contradiction of a goal always, for the owner; a contradiction or a widening of a head
that the document's "What is already decided" does not list as reversed or rewritten; a rejected
alternative brought back without a recorded reopening; and an acceptance criterion whose observable
the record rules out. The second rule keeps it from reporting each widening the document makes on
purpose. It runs only commands that read.

The decision serves `goal@knowledge-architect@agents-work-without-drift`, which is met when "a
departure from the recorded design is caught by a check or a review before it merges". The other
reviewers read the document against itself, against the code, or against the discussion. The
design audit reads design homes, but not goals, and it is run by the session that implements the
work, which may be the document's author, and skipped for a spec worked in the session that
converged. Two acceptance criteria that `goal@knowledge-architect@the-workflow-improves-through-real-use`
rules out each passed a plan document's three reviews in this repository. Its nearest rival, the decision-record reviewer
extended to plan documents, judges the record a diff writes and holds a plan's items as not
decisions yet, so it would carry two questions with two yardsticks. The cold implementer, the
other rival, asks whether the document can be acted on, and a document that contradicts a goal
can. The reviewer depends on a record worded wide enough, which the owner acknowledged: "this is
always dependent on the project being diligent in its goals and design records. Everything in the
workflow is, after all." Its cost, one more dispatch at each of those moments, the owner accepted.

### The transcript reviewer runs once more, alone and last, before every merge `##transcript-review-last-before-merge`

The installed review skill sends the transcript reviewer a last time before every merge to the main
branch, after every other axis has run and its repairs are committed, over the whole branch and the
transcripts of every session that worked on it. The findings of the other axes reach the session as
messages of its transcript, so only a reviewer reading it after their repairs can tell whether each
was acted on durably, per `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Its own
repairs land as additional commits, as many as their kinds need: a repair that the primer routes to
a commit of its own stays one, and nothing asks the dispatcher to merge them into a single commit.
A repair that would leave an earlier commit failing is folded, per
`design@agent-skills@review-repair-appended-or-folded`.
An additional commit that adds, removes or reverses a design head is reviewed by the
decision-record axis at least, and the review ends with that review's repairs; any other is reviewed
by no axis again, so the review ends, per
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.

### A review repair that adds, removes or reverses a design head is reviewed again by the decision-record axis, and no other repair is `##only-a-head-changing-repair-is-reviewed-again`

The installed review skill sends the decision-record axis at every repair commit that adds, removes
or reverses a design head, whether it answers the transcript reviewer or any other axis. A repair
made before the last transcript review is reviewed before it; a repair of the last transcript review
is reviewed after it, and the review ends with that review's repairs. Any other repair is reviewed
by no axis again: a rewording, and an argument, a mention or a reference added inside an existing
head, included. One made before the last transcript review is still read by it, as every commit of
the branch is. Without the record review, a head added, removed or reversed in answer to a review
reaches the main branch with no review of its record, against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: the last transcript review judges
fidelity to the owner's rulings and not the record.

The trigger is a whole head, which the diff shows: a heading with a slug that appears or
disappears, or a reversal under decision-recording §1. Each round of repairs then adds at most as
many re-reviews as it adds, removes or reverses heads. A trigger on any repair that "makes a
decision" lost: whether a rewording makes one is a judgement, and read wide it re-reviews repairs
that only reword a head. The rival that sends every axis again at every repair lost on the same
cost. A narrowing of an approved head that this trigger leaves out still goes to the owner, per
decision-recording §5, and the last transcript review checks it against the owner's rulings.

### Subagents dispatched together each get a scratch directory of their own `##a-scratch-directory-per-subagent`

A dispatcher that sends several subagents together names in each brief a scratch directory distinct
from the others', as it names a distinct worktree for a reviewer that builds, and an agent that
writes working files writes them there only. The installed review skill states it among the
invariants of a dispatch, and the standing-entry searcher's description, which carries its dispatch
rule, gives each searcher one. Subagents sharing one scratch directory overwrite one another's
files: parallel searchers in one session did, and one repeated its work, and reviewers in another
could not tell whether files of the same names had been overwritten. A searcher that does not notice
reads another group's rows, which `goal@knowledge-architect@agents-get-a-complete-workflow` does not
allow of a step every design discussion runs.

The rival that keeps every agent from writing files lost: the searcher's own task sorts a listing
and runs one command per entry, and searchers told to write nothing wrote files all the same, so
the instruction could not be obeyed together with the task.

## The configuration a project holds

### The workflow's rows of the knowledge table ship in the primer; the project's own rows sit in its root CLAUDE.md `##knowledge-table-home`

The knowledge table maps each kind of statement to its one home. Its rows for the workflow's own
records (a goal, a decision, a losing alternative, an issue, a tripwire, a contract, a plan
document) ship in the installed primer, and move with the version. A project's own rows, such as
its changelog or a register it declares, sit in its root `CLAUDE.md`, which the project
owns. A table written whole into each project at setup would keep the old routing after an upgrade
that changes it, against `goal@knowledge-architect@agents-get-a-complete-workflow`. A table shipped
whole would leave a project no place for its own rows, against
`design@agent-skills@overlay-by-separate-skills`. A project row may refine a primer row with what is
the project's own, such as a README that is also its package's page; it never contradicts one, and a row
that only repeats one is removed.

### The primer holds only what every session needs and no installed skill delivers `##primer-limit`

The primer reaches every session of every installing project, so its content test is the bound,
not a count of lines: it holds what every session needs and no installed skill delivers at the
moment it is needed. A convention of one project does not go in it, and neither does a procedure a
skill delivers when it loads. The workflow targets frontier-tier models, which the
design skill's work requires, per `design@agent-skills@frontier-tier-only`, so a size limit
would protect a reader the workflow does not serve.

### The primer carries the goals rule, the room to judge, the intent-and-claims rule, the check before diagnosing, and the rule for what is met outside the task `##primer-content`

Besides the knowledge table and the list of installed skills, the primer carries the rule on when
to write a reference, and five directives the skills rely on and no skill delivers at the moment
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
- **Where the installed text is silent, a session judges**, since the installed text leaves that
  room on purpose, per `goal@agent-skills@installed-text-leaves-room-to-judge`; room to judge is
  never room to act against an instruction. Every session meets a situation no instruction covers,
  and no skill reaches every session.
- **A design home is built intent, per `design@agent-skills@design-home-is-built-intent`, a reason
  recorded at the code binds below it, per `design@agent-skills@local-intent-binds`, and a claim
  about the code goes stale**: the code is checked against the first, the second is read before the
  code it explains is removed, and the third is verified before it is relied on.
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

### The primer names the plans directory the checker fixes `##plans-directory-in-primer`

The plans directory is docs/plans/ in every project, per `design@core@plans-dir-fixed`, so the
primer's row for unbuilt work names it, and the installed skills name it in plain text. A project's
own row for it would only repeat the primer's, and is removed, per
`design@agent-skills@knowledge-table-home`. A row each project declared lost: once the checker
fixes the path, a declared one could name no other.

### The routing table lists only what a project adds to an installed skill `##routing-table-shape`

The project's root `CLAUDE.md` carries one row per installed skill or agent that a project skill or
agent adds to, naming the additions. The setup skill writes the table, and the
agent-configuration skill keeps it. It carries no "read it when" column: the harness already
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

For a project that already has documentation, the setup skill takes an inventory of it,
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
own with `cargo install --locked --root`. A Rust project that is one package with no workspace
gains a workspace for that crate, which keeps the gates `goal@knowledge-architect@setup-brings-quality-tools`
asks for, or installs the binary as any other project does and builds its own gates, on its
owner's ruling, since the first changes the project's build. A machine-wide install would give two projects on one machine one
version, which is what bundling the workflow into the checker avoids, per
`design@knowledge-architect@binary-bundles-workflow`. A project with an extension runs its own
binary under a name of its own, per `design@core@declared-command`. **The manifest declares the
same pin**, in `[project] checker-version`, and every binary that dispatches a command of the
checker refuses to run over a project whose pin it does not satisfy, per `design@core@installed-binary-version-check`, so an install or a build
left behind by a move of the pin is refused at its first command rather than run. The setup skill
writes the key and moves it with the pin.

### The setup skill recommends one gates command, run by the published gates library `##gates-convention`

One command runs every check a project owes before a merge, runs them all when one fails, and exits
non-zero when any fails, so a verdict is one exit code and nothing is read through a pipe. In a Rust
project with a maintenance crate it is a command of that crate, per `design@agent-skills@xtask-pins-checker`, so
every adopting project runs the gates refined in this repository and in thaum. The gates a project
owes are its own list. The setup skill proposes two goals for such a tool, for the owner's
ruling: one command runs every check owed before a merge, and a task performed repeatedly becomes a
command of the tool.

### In a Rust project, one maintenance crate pins the checker and runs the gates `##xtask-pins-checker`

The maintenance crate of a Rust project depends on the checker and on the gates library, both
pinned exactly, at the version `[project] checker-version` declares, per
`design@core@installed-binary-version-check`, and serves two cargo aliases: `cargo x` for its own commands, the gates among them,
and `cargo klarch` for the checker's commands, which it carries under a command of its own through
the core's library, per `design@core@the-core-cli-is-a-library-module`. The gates run through the
published library, per `design@gates@gates-crate`, and the checker they run is the one this crate
pins. One crate does what a crate for the checker and a crate for the gates would otherwise do,
and gives the project's other repeated tasks a place, per
`goal@knowledge-architect@setup-brings-quality-tools`. The cost accepted: building the gates builds
the checker, which the check gate needs anyway.

### The setup skill proposes a short CRATES-IO.md for a crate the project publishes `##setup-default-crates-io-page`

For a crate the project publishes, the setup skill proposes to the owner a repository-facing
README.md and a separate, short CRATES-IO.md as its crates.io page, named by `readme`: the split
`design@knowledge-architect@crates-io-page-file` makes for this repository's own crates, for the
same reason. It serves `goal@knowledge-architect@adoption-is-easy`.

The owner's intent, on the owner's own proposal: "Maybe I'd also make it the default in the set-up
skill: for published Rust crates, keep the README.md as a repo facing document, and use a separate
CRATES-IO.md for what crates.io readers see." Asked whether it records the owner's intent, the
owner answered: "Q5: yes".

### Every Component states at least one goal `##goals-required`

The setup skill does not finish a Component without at least one goal, stated with the owner.
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

### A goal need not be met yet `##an-unmet-goal-is-intent`

A goal is the owner's intent about where the project should get to, and it constrains future work
and design from the moment it is written, met or not. When nothing fulfils it yet and no plan
document schedules the work, a `todo` issue holds that work and references the goal, so the gap
between the goal and the tree is listed as outstanding work.

### Goals change only through the goal-setting skill `##when-goal-setting-runs`

The goal-setting skill runs at setup for every Component, when the owner states, rewords or
abandons a purpose, and when a decision conflicts with a goal and the primer's rule sends the
conflict to the owner. An agent never edits a goal outside it, since a goal binds outright where a
decision binds as a presumption, per `goal@knowledge-architect@the-owner-decides`.

## The retrospective

### A retrospective is offered once per session, at a defined moment `##retro-trigger`

The retrospective is offered once per session, at the first of three moments: a pull request the
session worked on has merged, a plan document has left, or the owner says the session is ending.
It runs only if the owner accepts. A moment named by an event can be followed by any agent where
"a moment it judges right" could not, and offering it once keeps it from interrupting work.

### A retrospective asks standing questions, each on the decisions it watches `##premortem-as-watch-points`

Every retrospective asks whether the session needed to change an installed skill or agent, whether the
primer reached the session and its subagents, whether a project skill's addition was missed, and
whether the session needed to write a reference or a path without backticks because no checked form
expresses it. Each watches a decision whose
failure would be seen in real sessions before any check could see it:
`design@agent-skills@overlay-by-separate-skills`, the primer's delivery by an import line in
`design@core@owned-namespace-check`, `design@agent-skills@routing-table-shape`, and
`design@agent-skills@plain-text-is-no-repair` with
`design@agent-skills@checker-syntax-without-backticks-names-its-gap`. The last question is the
one channel by which a gap of the checker met in a consumer project reaches this repository, since
the consumer's own entry for it is not citable here. The skill never states how many questions
there are, so adding one changes no count.

### Nothing of a retrospective leaves the machine without the owner's reading and word `##retrospective-destination`

The owner reads both files verbatim and may edit them. On the owner's word, and where the owner
directs, the project's findings become issue entries in its own register, and the workflow's file
becomes an issue on knowledge-architect's repository, through `gh` or by the text and the address.
The repository is the workflow's own upstream, the one name of a project the shipped text holds.
