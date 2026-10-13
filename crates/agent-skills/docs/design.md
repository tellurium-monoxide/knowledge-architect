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
`design@agent-skills@shipped-text-line-comments`, a delivery placeholder is a literal the build
fills, and a placeholder `{{slug:<id>}}` ending a level-two heading is rendered into that heading's
section slug, so content/ defines no section in this repository's walk and the installed copy does,
per `design@core@installed-entities-from-the-tree`. A slug placeholder anywhere else fails the
build. A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, a saved
workflow of Claude Code's Workflow tool is `workflows/<workflow>.js`, and the primer is `PRIMER.md`;
the build asserts a saved workflow's name, per
`design@agent-skills@saved-workflow-name-is-its-file-stem`. The build script,
`path@agent-skills@build.rs`, walks the directory, renders each file, removing its comments, filling
its delivery substitutions and inlining its snippets, and generates `FILES`, each entry the install
path and the text. It adds the installer's prefix to each skill directory, agent file and workflow
file on the way out, so the installed names are the namespace of
`design@core@owned-namespace-check`. A file the layout does not map fails the build.

The list is generated because a list written by hand drifts from the directory it lists, and the
drift ships: a skill missing from the list is never installed, and nothing reports it. The core's
test `every_shipped_file_is_installed_in_the_owned_namespace` holds the build script to the
namespace, since an install path outside it would write a file the project owns.

### A saved workflow's `meta` name equals its installed file's stem, which the build asserts `##saved-workflow-name-is-its-file-stem`

The harness reads saved workflows directly under .claude/workflows/, no deeper, and calls each by
the name its `meta` declares, never by its file's name, as a probe of Claude Code measured, which
the message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
installer's namespace" records. So the build fails unless that name equals the installed file's
stem, or the prefix the build adds to the file would namespace nothing, per
`design@core@owned-namespace-check`.

### The shipped text carries comments for this repository, `%%` lines the build removes `##shipped-text-line-comments`

A line of a file of content/ whose first two characters are `%%` is a comment for this
repository's maintainers. It sits beside the instruction it explains, and it may cite design heads
and issues, per `design@agent-skills@content-is-in-the-walk`. The build removes it whole, so no
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
Obsidian, and cannot collide with a Rust snippet, whose comments are `//`.

### content/ is in the walk, so the references of its comments are checked against this repository `##content-is-in-the-walk`

The `%%` comments of `design@agent-skills@shipped-text-line-comments` are checked
by putting content/ back in the walk, rather than leaving them unchecked or adding a test in this
repository that would repeat the checker's reference reading. A declaration in the published
checker would cater to this repository's use in what consumers receive. That extends to the checker
what the owner said of the installed files: "I don't want to cater too much to this use case in the
installed files"; the extension is the session's argument, which the owner's choice of content/ in
the walk followed.

### The shipped text cites only what every installing project holds, and so no entry of this repository `##shipped-text-cites-no-entry`

The text under content/, with the snippets the build inlines, is read in every project that installs
it, so it names no Component and no convention of this repository, per
`goal@agent-skills@installed-text-works-anywhere`. The one name it holds on purpose is the
workflow's own upstream repository, where a retrospective's findings on the workflow go, per
`design@agent-skills@retrospective-destination`. It cites no entry: a reference to an entry resolves
only against the tree it stands in, and no tree but this one holds this repository's entries. A path
or a home that every conforming project holds may be written as a reference:
`path@*@docs/rejected-alternatives.md` names each Component's own copy of a required document, per
`design@core@components-carry-the-same-documents`; `register@*@goal` names each Component's own
goals home, in whichever shape it has, per `design@core@register-home-reference`; and
`path@plans@README.md` and `register@plans@spec` name a file and a home of the one plans directory,
whose path the checker fixes, per `design@core@plans-dir-fixed`. Each is true in every installing
project, and it shows the syntax the checker enforces there. Writing a per-Component path in that
syntax is the owner's proposal: "it promotes the same kind of syntax as what the checker enforces
in projects holding the installed skills". The same argument holds of the plans directory's paths
and homes, since the checker fixes that directory's path, and of a home named by its register,
since the checker refuses a home named by its path. A path that varies by project, and an
illustration, are placeholders in angle brackets. The command a project runs is written as the placeholder that the install fills
with the project's declared command, per `design@core@declared-command`.

**The shipped set may cite its own skills, agents and primer, and their sections**, as
`skill@<name>@<slug>` or `primer@<slug>`: every project serving `claude` holds them, installed, and
they are defined from its installed copies, per `design@core@installed-entities-from-the-tree`. A
test of the core holds every such reference to resolve within the shipped set. It never cites
`instructions@<slug>`, whose slugs are each project's own.

content/ and the snippets are in the walk, so every reference they hold is checked against this
repository. A reference to an entry that resolves here passes the check and would dangle in every
installing project: the reviews of each change, and the release's hand check over the installed
copies, judge that, until a mechanical check exists, which
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` tracks.

### An installed skill or agent names only the installed set's skills and agents, and the project's own `##no-external-handoff`

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
exception holds: agent-configuration is named like the configuration it edits. Its nearest rival,
agent-setup, reads like the setup skill, and the installer's prefix makes the full name,
knowledge-architect-agent-configuration, read as a skill; the cost is that the name is also the
artifact's. A name built on a verb form, such as discussing-design-decisions, reads as a sentence
and grows long. That argument is the owner's: "naming an "action" makes the skill names into full
sentences, which is weird and often too long". The rule binds a project's own skills too, through
the agent-configuration skill.

### Bounded work takes a path inside the design skill, after its grounding `##bounded-path-in-design`

Whether a request is a design question or bounded work, a clear requirement whose main risk is
over-building, is known only after the grounding of the design skill's loop step 1, its search for
standing entries included. The work is bounded when it reverses no recorded decision, every
decision it makes fails the entry tests, per `design@agent-skills@a-head-is-owed-by-an-entry-test`,
and no second defensible shape survives the nearest-rival test. The skill then sends one message,
the proposal with its nearest rival, the strongest open reading set aside, the consequences and the
default, which it builds, per `design@agent-skills@derived-decision-proceeds-on-default`; an
argument in the reply returns the work to the loop. The commit that implements it carries the
proposal and, if one came, the owner's answer, attributed, per
`design@agent-skills@attribution-replaces-quotation`. The skill's
description reaches a session before it starts a requested change whose design is not settled.

The owner argued that whether a task is bounded work, which does not change the project's design,
cannot be determined without going through the design skill's grounding steps. The
grounding gives bounded work the search for standing entries that no step gave it, and applying the
entry tests at classification keeps a head from being written for a decision that earns none. The nearest rival, a separate installed skill for
bounded problems, would repeat the grounding to classify at all; it is in the rejected
alternatives. A bounded path names no skill outside the installed set, per
`design@agent-skills@no-external-handoff`.

### The design skill is written for frontier-tier models only `##frontier-tier-only`

The design skill is not simplified for smaller models. A smaller model takes part in the
workflow as an implementer of what the owner and a frontier model decided, not as the owner's
counterpart in the discussion. So "a smaller model would not follow this" is not an argument for or
against any wording of the skill. Its description states "Requires a frontier-tier model
(Opus-class or stronger)": a model below that bar produces the format without the discipline, and
the description is the only text an installer reads before the first run. The evidence is a
scripted four-turn discussion run on three models, described
in `path@agent-skills@README.md`; it is not re-taken, since
`design@agent-skills@synthetic-evidence-not-built` builds no such run, and it stands for the part a
reading of its transcripts confirms: the smaller one reproduced the ledger's format, and it endorsed a weak proposal, invented states outside
the closed set, and dropped open threads between rounds.

### The design skill structures how a discussion is conducted, never what or how much is proposed `##structure-the-flow`

The skill fixes the flow: proposals are argued, threads carry states, a fork waits for the owner's
word and a thread with one shape standing closes on its argument, per
`design@agent-skills@owner-rules-at-a-fork`. It never fixes what may be proposed. The failure it was written against is the
solution quota: a mode that asks for two or three options gets two or three, and where one shape
applies, the others are fabricated to fill the count. A fabricated alternative makes a real
proposal appear to have won a contest that never happened, and spends the round that could have
tested it. The instructions that implement this are "never pad with alternatives to reach a count"
and the nearest-rival test, under which "no rival worth naming" is a claim to test. An edit that
specifies what must be proposed, rather than how a proposal is argued and tracked, works against
this entry.

No bound is set on how much is proposed either: on the number of threads, of proposals, or of
rounds. The ground is the owner's: the owner ruled that a bound on what and how much the design
skill may propose must not be added, since it would go against the core idea of the open design
discussion the skill tries to encourage. Answering a proposed path that would bound a discussion to
one decision met during a task, the owner argued, as the owner's opinion, that a discussion that
starts with one decision should be bounded in neither threads nor rounds, since its size depends
only on what grounding, investigation and experiment bring, which nobody knows in advance. A bound fixed before the discussion decides its size
before its content. A path chosen by the cost of reversal, or by where the work lands, changes where the
deliberation is kept, never how large the discussion may grow.

### A decision met in any activity goes to the design skill only when it creates a head, contradicts one, or adds a member its argument does not cover `##new-or-reshaped-head-needs-design`

A decision goes through the design skill, whatever activity met it, when it creates a design head,
contradicts a statement of one, its argument included, or adds a member the head's argument does
not cover. An addition that answers another question than the head's takes a head of its own, per
`design@agent-skills@one-decision-per-head`. A member the head's argument covers is recorded
directly, routed per `design@agent-skills@extension-follows-the-ground`; an approval the owner gave
it is recorded in the commit, attributed to the owner, not in the head. A change that relocates or rewords recorded decisions, a split of a head included, and adds or
removes none, is not a decision and needs no design skill; the routing and fidelity-of-relocation
review axes judge that it adds or removes none. A choice that earns no durable text stays in a code
comment and a commit message.

The decision-recording skill triggers once a decision has been made, or at the latest when its
text is about to be written, so a new decision met during an issue fix or a review repair otherwise
reaches the record with no argument, no rival weighed,
and none of the design skill's rules for reading the owner's word. This serves
`goal@knowledge-architect@the-owner-decides` and
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. A member inside a head's argument
carries less risk: the decision the head records, and its argument, stand, and the title still
tells a reader of the outline what is decided. The rival, every decision that earns durable
text through the design skill, puts such an addition, and every relocation, to a full discussion
whose outcome the head already records. A test on the size of a change lost: a one-sentence
reversal is the change that most needs the discussion.

Three texts deliver it. The design skill's description names the symptom: it fires while a
decision is still being settled in the conversation, before any write. The primer's section on
design heads holds the backstop, which sends a decision in one of the three cases, not argued, back
to the design skill before its text is written; it reaches every session at every reading of a
head, per `design@agent-skills@one-home-for-head-rules`. The decision-recording skill, loaded
before any write into a design home per `design@agent-skills@design-home-write-loads-recording`,
asks for that section to be read again at the moment of writing.
`tripwire@agent-skills@head-created-without-deliberation` watches whether they reach a session in
time.

### A head's title states the rule that decided, and its body names the members built `##title-states-the-rule`

When a decision admits members, such as fixes, kinds, verbs, sections or consumers, or picks a
mechanism to meet a requirement, the head's title states the property that admits a member, in its
argument's own terms, and its body names the members built as what the rule admits today. A set
closed on purpose keeps its list in the title, with the sentence that argues the closure in the
body. A title states no more than its argument argues.

A count or a list in a title goes stale at the next member, and every later member then reads as a
change of the decision, with its reversal, its slug rename and its tripwire rename, against
`goal@knowledge-architect@agents-get-a-complete-workflow`, which aims for fewer decisions reversed
by accident. The nearest rival, the instance in the title, has that cost: an audit of this
repository's design heads at commit c282b00, reading every head and its history, finds the instance
stated as the rule in 27 of its 189 heads, and its history holds rewrites of such heads that a
member the argument already admitted forced. The figure is re-taken by a fresh read-only audit of
the heads at that commit against this rule. A review repair that moves a member
into a title makes the defect: "the title now states it". The opposite defect, a title
wider than its argument, caused reversals too, which the last sentence of the rule guards. A title
stating a rule alone would let a session add members the owner never saw, so it holds only with the
routing of `design@agent-skills@extension-follows-the-ground`.

### A head holds one decision, and parts that lose to different nearest rivals are separate heads `##one-decision-per-head`

Two statements are one decision when they answer one question and lose to the same nearest rival.
A rule's exception, its parameter and its delivery belong to its head, since none means anything
without the rule. Two statements that lose to different nearest rivals, or whose arguments share no
premise, are two decisions, and each takes a head of its own when it passes an entry test, and a
comment at its code when it passes none. A title joined by "and" over two decisions shows a bundle,
and rewording the title to state both does not make them one. A member of a set the title's rule
admits is no decision, so a member missing from a title is no finding.

The owner raised the question: "I'm not certain there is anything in the decision record skill that
gives a split criteria for design heads." A rule that only asks a head carrying several decisions
to pass "the test for each one, or is split", and rewords a title to state an addition beyond it,
gives no test: an audit of the design heads at commit c282b00, re-taken by a fresh read-only audit
of the heads at that commit against this rule, finds about one head in five holding two or more
decisions, and every split in this repository's history up to that commit is one a title false of
part of its body triggered, none a criterion.
The nearest rival, the test
"would reversing one part leave the other standing?", splits a rule from its exception, which always
passes it. The cost is
more heads, not more decisions: the owner's words, "We are not adding content and arguments, only
slugs and independent heads", and the entry tests still judge each part, so the decisions recorded
stay few, per `design@agent-skills@a-head-is-owed-by-an-entry-test`.

### A member that a head's argument covers is recorded directly, and one that the owner's words name goes to the owner `##extension-follows-the-ground`

A new member of a head, which the head's rule admits and its body does not name, is routed by the
head's ground, as `primer@design-heads` states it. Where the part of the head that admits it stands
on its argument, and the argument covers the member, it is recorded directly. Where that part's
ground is the owner's words and they state the rule, the member is within the ruling and is
recorded directly. Where the owner's words name the members, it goes to the owner as one proposal,
which owes no reversal. A member the rule does not admit, or the argument does not cover, is a
change of the decision, per `design@agent-skills@new-or-reshaped-head-needs-design`.

The owner approves an argued decision on its argument, so a member the argument covers is within
what was approved, and a member the owner's own words enumerate is not, per
`goal@knowledge-architect@the-owner-decides`. The head shows its ground, so the route is read from
the head itself. The nearest rival, routing by the scope of the owner's approval, needs that scope
found in the history of each head, often over several commits. Sending every new member through
the design skill costs a reversal for an ordinary extension, which
`design@agent-skills@title-states-the-rule` measured. The risk is a session judging "the argument
covers it" too widely; the design-conformance reviewer reads every plan document against the heads.

### A head that a change touches is brought to the rules on heads in that change `##existing-heads-on-touch`

A head that a change touches is brought, in that change, to the rules of `primer@design-heads`:
its title to the rule it argues, each decision it bundles to a head of its own, its ground to its
argument. Where that would widen what the owner's words in it approved, as a title moved from the
members the owner named to the rule, the change goes to the owner as one proposal.

**Re-pointing a reference in a head does not touch it.** The edit states nothing new in the head,
and a change that re-points references across the record, as a renamed entry or a migrated
reference form does, would otherwise owe the repair of every head it passes through, which is a
project audit's scope, not that change's. Adding a reference, or rewording a sentence, is more than
re-pointing.

The rules apply at the moment a session already reads the head and changes what it says, so a head is repaired at
the cost of one read. The whole record is repaired at once only by a project audit, on the
occasions of `design@agent-skills@audit-triggers`, per
`design@agent-skills@audit-repairs-the-whole-record`.

### A write into a design home, or an edit of agent-facing text a head describes, loads the decision-recording skill first, with no condition judged before it `##design-home-write-loads-recording`

A session loads the decision-recording skill before it writes into a design home, whatever its
activity. A session editing agent-facing text, a skill, an agent or a `CLAUDE.md`, also loads it
before an edit of a text whose behaviour a head describes. The skill, with `primer@design-heads`,
then judges whether the edit contradicts a head, adds a member its argument does not cover, or
records a decision at all. The skill's description
states the first trigger, and reaches every session.
`skill@knowledge-architect-agent-configuration@admission-standard` states both, and reaches a
session editing agent-facing text, which is the one activity that asks for a search of the design homes before an edit.

A session can see both conditions without any standard: the file it is about to edit, and the
result of that search. A load conditioned on whether an edit contradicts a head or adds a member
fires only in a session that already applies those tests at the moment of writing; the tests are
in the primer, but the skill holds the recording procedures and the instruction to read the
primer's section again, which brings it back late in a session, per
`design@agent-skills@primer-reread-before-recording`. This serves `goal@knowledge-architect@the-owner-decides`
and `goal@knowledge-architect@design-is-recorded-with-its-arguments`.

The nearest rival restates the backstop's tests beside the condition at each site. It makes copies of a
test whose home is `primer@design-heads`, and it reaches only a session editing agent-facing
text, not an issue fix or a review repair that writes a head. The cost is one skill load on every
edit of a design home, rewordings included; a rewording records no decision and leaves by the last
sentence of the backstop of `primer@design-heads`.

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
whose record is docs/decisions.md on its branch next. A figure is taken again from that record with
`git show next:docs/decisions.md`, run in that repository.

### The record of the installed text keeps intent and coherence, not wording `##instruction-record-is-minimal`

A decision about the installed text is recorded only where it keeps the owner's intent on record, as
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks, or keeps two parts of the
workflow consistent, as `goal@agent-skills@one-skill-per-activity` asks; or where another entry test
admits it, such as a reason several installed texts must respect. A rewording, and the rationale for how one instruction
is phrased, is not recorded. Such proposals can be made without bound, unlike a technical decision,
which costs implementation work. The commit message carries a rewording's argument.

The owner's intent is on record where test 4 of `primer@design-heads` holds, per
`design@agent-skills@a-head-is-owed-by-an-entry-test`: the owner, asked, confirms that a ruling an
agent could reverse as a small fix, or an argument the owner wants kept, records the owner's intent;
an approval of the agent's proposal does not.
Two parts of the workflow are installed texts, or an installed text and the checker it must agree
with, as the planning skill's sections agree with what the core checks; a decision of this
repository alone is not one, since the installed text names none of this repository's conventions.
Installed texts alone would leave a skill free to disagree with what the checker enforces, which an
adopting project meets as a check that fails on a document the skill told it to write. A reason that passes no test goes in a
`%%` comment at the instruction it explains, per `design@agent-skills@shipped-text-line-comments`.

### An instruction about how an agent behaves is added to agent-facing text only on evidence from real use `##behaviour-additions-need-real-use`

A defect provable by reading is repaired on reading, and an instruction about how an agent behaves
is added only on evidence from real use. Real use originates such an addition: a behaviour seen in
a real session, produced unprompted or asked for by the owner mid-session, with the owner naming
what the session would have lacked without it, and a one-sentence statement of the mechanism that
produced the need. An unprompted behaviour shows the gap, not the wording that fills it. A finding
of a review or of an audit originates an edit only where its defect is provable by reading: a
contradiction, a broken trigger, a factual error, an instruction whose words allow two acts, a
provable gap, divergent verdicts that a review standard and an audit standard give over one text
of the project, or a statement that fails the need test of `design@agent-skills@need-test`. A
provable gap predicts no behaviour: it is an input that an act needs, that no text
supplies and that judgement cannot derive, since it is a fact fixed elsewhere, such as a format the
checker enforces, a name or a path. So a reading admits its fill, and the fill, which adds an
instruction, goes to the owner, per `design@agent-skills@audit-outcomes`. The rule is scoped to an
addition about how an agent behaves because its argument is that a predicted behaviour is
unreliable evidence, and a fact that no text gives is no prediction. A finding that predicts a
behaviour is parked as an issue, which states what a real session would have to show. Published
literature and synthetic scenarios originate no edit, a written example of divergent verdicts
among them, and no run is built to observe how an agent
follows the workflow's instructions, per `design@agent-skills@synthetic-evidence-not-built`. The rule derives from
`goal@knowledge-architect@the-workflow-improves-through-real-use`, which is stated for the whole
workflow, so it covers every installed skill and every installed agent, a reviewer agent growing
the same way, one predicted check at a time, and a project's own agent-facing text alike: neither
part of its argument, that a predicted behaviour is unreliable evidence and that rules grow one per
interaction, depends on who ships the text, and a project's own text exempt would let its writer
admit what the audit refuses. The rule is stated in
`skill@knowledge-architect-agent-configuration@admission-standard`, per
`design@agent-skills@admission-rules-home`. The evidence behind it comes from one skill, and the skills and agents forked from thaum have not
been edited under it, so a text that needs a different standard argues its exception. One exception is argued in its own head,
`design@agent-skills@staged-check-before-each-commit`, and one beside its instruction, the reread
of the primer on the in-change path of `skill@knowledge-architect-design@decision-authority`.

### An installed skill's text carries no size budget `##no-size-budget`

The skill text carries no size budget: an edit is judged on whether it changes behaviour, and two
real sessions held the design skill's full ledger discipline at several hundred lines without
drift.

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

The rule derives from `goal@knowledge-architect@the-workflow-improves-through-real-use`, which
states it for the whole workflow. The owner's argument: synthetic evidence is too unreliable, since
reading it soundly would need at least a statistical analysis, which costs too much, and a proper
mock task needs a design and a test of its own, so it cannot be created on demand. One run of an agent is a sample of one. The
evidence the rule gives up is supplied otherwise: retrospectives of real work reach this repository,
and this repository installs its own skills, so its own sessions use each change before a release
reaches a consumer. The cost accepted is that a defect in a new text is found at its first real use,
not before its merge. The cost is small: the defect a run would catch first is a text's mechanics,
such as a new agent's tools or report format, and this repository's own next review is that text's
first real dispatch, where such a failure shows at once and costs one re-dispatch. The owner judged
that loss small, if it is the only one the session could name, since most of what this repository
does is already well proven to work. Two instances in this repository grounded the rule. A spec's
acceptance criterion, a replay of a search agent over another project's history, was ruled out by
the owner at the step that ran it, on the owner's experience that such testing methods are not very reliable, and drive the
workflow toward wrong directions more often than toward good ones. A slice's criterion, a subagent
classifying decisions written for the check, was dropped at the slice's design audit. The boundary
is a property of the verdict, not a list of subjects: the owner preferred that the head name a wider
category matching the owner's intent, since a narrow wording reduces the intent behind it to
something lesser.
The nearest rival, a list of exempt subjects naming harness mechanics, leaves out the installer and
the checker.

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
are no unit of the history, per `design@agent-skills@step-is-no-commit`.

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
describes. It works with `design@agent-skills@behaviour-additions-need-real-use`: that entry says what evidence
admits an addition, and this one says which gaps are worth admitting. In the designing-together
repository, a rule for an abandoned discussion was argued, approved, written and reverted in one
session, and four further review findings about the user's behaviour were queued behind it on the
same reasoning. Only the design skill and the setup skill state a set so far:
`issue@agent-skills@expectation-sets-for-the-installed-skills`.

### Every statement of agent-facing text names an act an agent does differently because of it, or it is not written `##need-test`

Every statement of agent-facing text, installed or a project's own, passes the need test of
`skill@knowledge-architect-agent-configuration@admission-standard`: name the act of an agent
following the workflow that would differ without it. A statement that names none is not written,
and an existing one leaves by that section's outcome table: deleted where it carries no
instruction, moved to a `%%` comment of the installed source where only the source's maintainer
reads it, and put to the owner where it is an instruction no agent performs. A reason passes when
it states a mechanism or a failure prevented, as
`goal@agent-skills@installed-text-leaves-room-to-judge` asks the reasons of each instruction; a
second reason that names no other mechanism, a duplicate inside one text, a lead, an emphasis and a
remark about the text fail, since a loaded text is read whole and they add no act. A statement
about state outside the project is suspect, and is written only where an act needs it.

The question the test asks is the owner's proposal, made on a statement an agent had written into a project's root
`CLAUDE.md` about another project's issue, which the owner judged useless to an agent and liable to
go stale with no work in the project. Whose act counts is the owner's ruling: an agent that
maintains the text counts, a statement for the maintainer alone is at most one sentence, and in
the installed text it is a `%%` line, per `design@agent-skills@shipped-text-line-comments`, since
no installing project maintains the installed text. Where the standard left it open, the owner chose
to count the primer and the root `CLAUDE.md` that imports it as one text, since the two are loaded
together, and ruled that the primer's copy is the one that stays. The test's reach, stripping what
is not fully necessary at the price of some variation between its readers, is the owner's
weighing: a statement one reader passes is met by a later pass.

The nearest rival, a narrower test under which a second reason, a duplicate inside one text, a
lead and an emphasis pass, loses on that weighing: it leaves whole classes of statements no act
needs in place. A content rule naming each kind of unneeded statement, such as "no state of another
project" beside "current reality only", grows one clause per case observed, the mechanism
`design@agent-skills@capabilities-not-structure` records, and would refuse a statement an act needs,
such as a tool's behaviour a command relies on. A duplicate fails only while its twin stays, and its
finding names the copy that stays, the one in the section whose subject it is, or the primer's
across the primer and the root `CLAUDE.md`: two passes that each delete the copy they flagged would
otherwise delete both.

### Every rule on what enters agent-facing text and how a change to it is shaped lives in one section of the agent-configuration skill `##admission-rules-home`

`skill@knowledge-architect-agent-configuration@admission-standard` holds the need test of
`design@agent-skills@need-test`, the evidence an addition needs, the lapse rule, the kind of a gap,
the shape of a repair with its instruction count and net growth, the record check, and the outcomes
of a need-test failure. Every actor that writes or judges agent-facing text reads it whole at the moment it judges: a
session writing such text, a skill that moves text into it, the review axis on agent-facing text,
the agentic-workflow axis's auditors and repair steps, a retrospective's proposals, and an analysis
of received findings. A session writing the text, a skill's move included, reaches it through the
agent-configuration skill's trigger; each judging actor's text points to it. None restates more
than one sentence of it, per `design@agent-skills@restatement-size-test`.

The centralization is the owner's proposal, made so that the audit, the reviewers, an intake of
received findings and the installed skills cannot produce an addition that another of them would
refuse or reshape. The home in the agent-configuration skill is the owner's proposal too, and it
stands on the argument below. Maintaining the configuration is the activity whose skill governs what
enters it, per `goal@agent-skills@one-skill-per-activity` and the knowledge table's row "how to
perform an activity → the owning skill"; the audit, the reviews and an intake judge that text and
do not own its standard. Where the section lives does not decide when it is read: each actor's
pointer asks for it read whole at that actor's own moment.

The nearest rival, the standard kept in the audit skill with the agent-configuration skill pointing
to it, loses on that row; under it, the writer of a project's configuration, the most frequent producer,
reads another subset of the rules than the audit applies. An installed agent holding the standard loses on rule 4 of
`skill@knowledge-architect-agent-configuration@where-text-goes`: a session writing configuration
reads the standard in the main session, which loads a skill and reads an agent's definition only by
opening its file.

## The workflow the skills carry

### Every item the owner rules on by name and that carries no slug gets a label, of a prefix of its own kind where the label reaches a committed document and of the common `Q` where it does not `##ruled-items-labelled`

Every item the owner is asked to rule on by name, and which carries no slug when it is put to the
owner, gets a label: a prefix of capital letters naming its kind, and a number. Numbers run from 1
within each prefix, in order of appearance, and are never reused within the record the label
reaches: the committed document, a milestone's documents counting as one; a
file; or the commit messages of one branch, counting as one. A label that reaches none is numbered
within the message that puts it to the owner. An item that gains a
slug later keeps its label beside it. A label that reaches a committed document, even one that
leaves later, such as a plan document, takes a prefix of its own kind, distinct across the
installed workflow. A label used only in the conversation reuses the common prefix `Q`. A label
numbers what was put to the owner; it never names what decided, and an answer keeps the authority
that answered it, per `design@agent-skills@answer-keeps-its-authority`. Every distinct prefix is
listed here, and an edit that adds one checks it against the list:

| prefix | items | the committed document it reaches |
| --- | --- | --- |
| `T` | tripwires put to the owner | a plan document's Premortem section |
| `AC` | acceptance criteria put to the owner | a plan document's Premortem section and its defaults |
| `D` | a plan document's defaults awaiting the owner | its "Defaults awaiting the owner" |
| `W` | the findings on the installed workflow: a retrospective's, and a project audit's sent upstream | the workflow's file of the retrospective, whose shape an audit's upstream file takes |
| `C`, `P` | a retrospective's other findings | the retrospective's files |
| `F` | a project audit's findings put to the owner | the audit report file of the pass |
| `Q` | entry test 4's questions, and every label that stays in the conversation | none |

The owner asked for it, from a premortem message that mixed causes, tripwires and findings: "it is
often quite hard already to rule on tripwires, because I'm never certain how many there are and
what they are about." A ruling given in words then matches items by guess. The owner asked before
for entry test 4's questions to be numbered, per `design@agent-skills@a-head-is-owed-by-an-entry-test`,
and judged the pattern recurring: "this should cover everything that I have to rule on by name". A
label that reaches no document reuses `Q`, on the owner's argument against "searching for unique
letters, which will necessarily end up needing more letters than available". The structure passes
`design@agent-skills@capabilities-not-structure`, since the owner named the lack and the label's
scope follows the record that carries it. The numbering is stated once, in `primer@labelled-items`,
since labels are given in sessions of every activity and a range stated per skill drifted: three
skills gave `T`, `AC` and `F` three different ranges. Each installed skill that asks for such
rulings restates its own prefix where it asks. A project audit's findings sent upstream reuse `W`:
its upstream file takes the shape of the retrospective's workflow file and reaches the same
receiver, which cites each finding by its id and its file's stem, so one prefix names one kind of
finding in one kind of file. An `F` item is numbered within the audit report file of its pass,
which holds the item and the owner's ruling on it, per `design@agent-skills@committed-audit-report`.

### A design home holds built intent, and a plan document holds unbuilt intent `##design-home-is-built-intent`

A design home holds the design as built and its reasons, and the code is checked against it. A plan
document holds decided design that is not built yet, and each decision moves into the design home at
the landing of the work that builds it, not when the spec is written. While the work is open, the
spec or the milestone document, on the main branch or on the work's, is the only place the
decision exists. A decision that no work implements, such as a policy, and that is not part of
any spec, counts as built intent, since nothing waits to be built, and is recorded when it is made. The rival, a design home holding intent built or not, needs a
marker on every entry to tell the two apart, and checking the code against an unbuilt entry reports
a defect in code nobody has written. Unbuilt intent has a checked home because plan documents are a
structure the checker reads, per `design@core@plan-register`. This serves
`goal@knowledge-architect@documentation-stays-consistent`.

### A head prevails over diverging code until the divergence closes, never by following the code `##head-prevails-over-diverging-code`

A head can be wrong, and while it
diverges from the code it still prevails: the divergence closes when the code meets the head or when
the head is reversed, never by following the code, and work that goes on meanwhile builds on the
head. Without that, a session that must keep working would build on whichever
side it met first, and a divergence would close silently in the code's favour, against
`goal@knowledge-architect@agents-work-without-drift`.

### A divergence between a head and the code goes to the owner only where reversing the head is a fork or its ground is the owner's `##divergence-routed-by-its-authority`

`primer@intent-and-claims` routes a divergence by the authority that settles it, per
`design@agent-skills@questions-go-to-their-authority`. Where the head stands, its argument or the
owner's ground undefeated, the code is the defect: the session fixes it or opens an issue, and the
work builds on the head. Where the session finds the head's argument defeated, the head's reversal
goes to the design skill, and to the owner where it is a fork or the head's ground is the owner's;
the part of the task that rests on the point waits for that answer.

It serves `goal@knowledge-architect@the-owner-decides`, which gives the session what the argument
from the record settles. The nearest rival, every divergence holding its part of the task until the
owner's word, sends the owner a question the record answers wherever the head stands, since a design
home is built intent the code is checked against.

### A reason recorded at the code binds as intent at its scale, below the design home `##local-intent-binds`

The primer's intent-and-claims rule has three classes. A design home is authority. A reason recorded
at the code, an inline comment saying why the code is shaped so or the message of the commit that
argued it, is intent at the scale of that code: it binds as a presumption, a change that defeats it
argues against that reason in its own message, and where it conflicts with a design home, the design
home prevails, as in any divergence, per `design@agent-skills@head-prevails-over-diverging-code`. Between
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
reopen it, which is the goal it derives a constraint from and a decision of another Component it
depends on, as references, the measurement it rests on, with the command that takes it again, and
the fact that defeated its nearest rival. The deliberation, which is what was weighed, in which
order, what evidence was built and who ruled what, stays in the spec while the spec exists, and in
history after that. The owner's words that are a decision's ground are the exception, per
`design@agent-skills@head-ground-is-the-argument`. A decision taken with no spec carries its deliberation in its commit
message.

`goal@knowledge-architect@design-is-recorded-with-its-arguments` requires that a later session can
tell what a change costs without deriving the argument again, so the premises a reversal must
defeat are in the head. The deliberation is not, because every session reads the design homes
before it changes code, and a head that carried its whole deliberation would make that read cost
the length of every discussion ever held. History serves the reader who needs it, because a spec leaves when its work lands, per
`design@agent-skills@plan-document-leaves-at-landing`: a deleted spec is
listed by `git log --diff-filter=D` on the plans directory, and its last state is one `git show`
away. That agents take that route is measured, not assumed: a scan of a project's session logs for
a `git show` or `git log` command naming a deleted plan document re-takes it, and a scan that finds
none in a project whose specs are deleted reopens this decision. The cost is accepted: a reader
without a clone of the repository cannot reach a deliberation.

### A head stands on its argument, and cites the owner only for what came from the owner `##head-ground-is-the-argument`

A head records a decision that was proposed, argued over its costs and rivals and approved, on that
argument, with its costs and rivals in its body or in the rejected alternatives. An approval is no
ground, whatever its words: it stays in the deliberation. The owner's words are a ground,
attributed to the owner per `design@agent-skills@attribution-replaces-quotation`, only where the decision came from the owner: the owner proposed it, chose where the argued rivals
did not settle it, or stated a premise only the owner can state. A head holding a part of each
says which part came from the owner. A head that stands on its argument is reversed by a better
argument, which the owner approves where the reversal is a fork, per
`design@agent-skills@questions-go-to-their-authority`; one whose ground is the owner's words is reversed by the owner,
or by a defeated premise of those words.

The rule is the owner's proposal: a head is recorded on the arguments discussed, with its costs and
rivals in its body or in the rejected alternatives, not on the ground that the owner ruled, in
particular where the decision was the agent's position or default and the owner only approved it.
The owner judged this provenance to matter a lot for agents that weigh any change or addition to
the design. It serves `goal@knowledge-architect@the-owner-decides`,
which asks that the record show the owner's decisions. The nearest rival, citing every ruling in its
head, makes an approved default read as the owner's intent: an audit of the heads added or
rewritten in the week entry test 4 was added, up to commit c282b00, finds about two in five of their
citations of the owner to be an approval of the agent's position, often a batch word such as "All
defaults approved", and the citation, deleted, takes no argument with it. The figure is re-taken by
a fresh read-only audit of those heads at that commit, against the commits and plan documents that
wrote each citation.

### A durable record attributes a decision to the owner, and the owner's words verbatim stay in a plan document or a file that leaves `##attribution-replaces-quotation`

`primer@design-heads` states it: a design head, a commit message, a tripwire and an issue entry
state what the owner decided and how far the decision reaches, never the owner's words verbatim.
The verbatim words stay where the deliberation is kept: a plan document's items, per
`design@agent-skills@plan-records-the-exchange`, and a committed file that leaves once its outcomes
are carried out, per `design@agent-skills@committed-audit-report`. An attribution is checked against
the question it answered, as a quotation is: by its writer, with the owner's words beside it, at
the close of the decision-recording skill,
`skill@knowledge-architect-decision-recording@before-you-finish`, and by the transcript reviewer,
the decision-record reviewer and the auditors' verifiers, and it keeps the scope and the hedges of
the words. A
quotation already in a head is converted when a change touches the head, per
`design@agent-skills@existing-heads-on-touch`; the reference a rename sets beside a quotation does
not count, per `design@agent-skills@a-past-sentence-is-rewritten`.

It serves `goal@knowledge-architect@the-owner-decides`, under which a decision the owner made is
recorded as the owner's: the attribution carries that, and the record shows the owner's decisions.
The owner judged verbatim quotations of little value most of the time, as long as the attribution is
correct, and, as a hedged view, out of place in design heads. A verbatim quotation also costs the checker's syntax: two sessions bent one to get it past
the checker, one by writing spans of reference kinds still to come as placeholders, in the
agent-configuration-entities milestone, and one by writing the owner's words naming a skill's
reference form bare, in the session that wrote the path-quickfixes spec, which
`design@agent-skills@plain-text-is-no-repair` does not admit. The nearest rival, keeping the
quotation in a head whose ground is the owner's words, keeps that cost where the ground is most
read. The risk is that an attribution loses a scope or a hedge the words showed: in this
repository, a paraphrase of the owner in `design@agent-skills@transcript-reviewer-agent` once
misstated the owner's request, and a quotation replaced it, in the commit "[review] The last
transcript review of the small-fixes branch: no ruling misstated, three items given a durable
outcome, one paraphrase replaced by the owner's words". The transcript reviewer reads the words
against the record, and `tripwire@agent-skills@attribution-loses-the-scope` watches.

### A decision approved with no argument is argued before it is recorded `##unargued-approval-is-argued`

Before a head is written from an approval that no argument, rival or cost stands behind, such as a
default approved in a batch, the session searches for the arguments for and against it, its costs
and its rivals, and records what it finds. Where the rivals come out equal, the fork goes to the
owner as a tie, and the owner's choice is the ground, as a decision that came from the owner. It is
the decision-recording skill's, `skill@knowledge-architect-decision-recording@unargued-approval`,
since recording is the last point before a head exists.

The rule is the owner's proposal: "a decision approved without arguments should always go through a
step of searching for arguments in favor or against, costs and rivals." Without it, a batch approval
of an unargued default gives a head with neither an argument nor, under
`design@agent-skills@head-ground-is-the-argument`, a ground. The nearest rival, recording the
approval as it stands, is that head. The cost is one search per unargued default.

### A decision earns a design head only when an entry test passes, and the decisions recorded stay few `##a-head-is-owed-by-an-entry-test`

A decision earns a head in a design home only when one of the four entry tests of
`primer@design-heads` passes: reversing it would change an interface others consume, a type or
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

Fewer decisions recorded is the principle, as long as no critical intent or argument is lost. Every
grounding reads the design homes whole, and a human overseer reviews them, so each head costs every
later session and every review; the owner's statement of the trade-off: "more decision records,
which over time bloat the design docs and make them harder to review by human overseers of the
agents, as well as harder to ground on for agents, or less decision records, only when load bearing,
but anything not argued in there has to be judged by the agents." The session that changes local
code reads the comment at that code, not a design home it would have to think to search. Test 2 is
where the checker's reach ends: a reason that several sites must respect needs a home each site can
cite, and a comment cannot be cited, while a policy no code and no text states, or an absence, has
none; for agent-facing text, each text that states the instruction is one site, a design head
that restates it included, other than the decision's own, which would otherwise earn itself, so a
reason stated by one text lives beside its instruction. A head that
restates an instruction is read and acted on as the skill is, so the reason must reach it too; an
audit that counted the skill alone moved a head to a `%%` line while a second head still stated its
rule, and the head was restored. The rival reading, a policy with no code of its own as the
case of installed text, fails: an audit counted two skills stating one decision as one site, and the
clause could be read to admit every decision about installed text. Test 1 covers
every interface others consume, not only a type or a signature: a head about a grammar, a file
format or a command binds every adopting project, and under a strict test 1 such a head enforced in
one module would pass no test. Test 3 counts a tool's behaviour measured as well as documented: a
measurement costs as much to take again as a documentation reading to derive again, and a
documented-only test 3 fails where the documentation is silent. Test 4 is the owner's: "Me answering
"yes" or "approved" is not ground for recording a design decision, it is not "intent" from me, it is
simply me answering your question "should I do that?" The key point is that my ruling has to carry
**my** intent, not just acceptation/approval of your proposals." On a hedged word: "Under (b),
anything I says in a discussion of the design skills comes out as "my intent", as I read it. This
should not be the case." And on who judges it: "maybe the correct solution would be to ask the
owner, rather than judge yourself. It would be more reliable IMO". The questions are numbered at the
owner's request: "The questions should be numbered (Q<N> or similar shape), so I can answer each one
quickly without confusion." The agent's own judgement of the owner's intent, its rival, misread a
hedged proposal as a ruling twice in the session that made the test. Its other rival, any recorded
ruling of the owner, admits nearly every head harvested from a discussion, since the workflow asks
the owner to confirm most decisions. The decision serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: a later session can tell what it
may change and what a change costs from the head where one is owed, and from the comment where none
is. The nearest rival of the principle, a
head for every decision discussed with the owner, records a mechanism carried at one site by its
comments, of which the owner's words are: "the intent can easily be carried by local code
comments". Within test 2, "the same statement at more than one site" admits a path repeated for
consistency, of which the owner's words are: "It is something that has no reason to ever change, and
the path being named in multiple places is just basic self consistency." The wider test 2, "constrains work that has not been built", admitted nearly every head
under a wide reading and almost none under a narrow one, and the design homes hold built intent, per
`design@agent-skills@design-home-is-built-intent`. Whether the heads cost a session more than they
save is not measured: `issue@agent-skills@the-retrospective-counts-no-review-cost`.

### An alternative earns an entry by the recording tests, not by having lost `##losing-alternatives-filter`

A losing alternative earns an entry in the rejected alternatives only if it passes the tests of the
installed decision-recording skill: it would change an interface others consume, as entry test 1
of `primer@design-heads` defines it, its reason rests on a reading of an external
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

### A premortem's cause becomes a tripwire only on the owner's word `##premortem-tripwires-on-the-owners-word`

A premortem's surviving causes become tripwires only where the owner rules that they should.
Whether a risk is worth watching is a fork: watching it and not watching it both meet every binding
constraint, and they trade the cost of watching against the risk, which no goal or decision ranks,
so it is the owner's, per `goal@knowledge-architect@the-owner-decides`. Where a ruled tripwire is written is
`design@agent-skills@premortem-tripwire-written-at-harvest`.

### A premortem's cause the owner ruled on is written at the harvest of the decision it guards, in the tripwires home of the Component that owns it `##premortem-tripwire-written-at-harvest`

A premortem's cause the owner ruled a tripwire is written at the harvest of the decision it guards,
in the tripwires home of the Component that owns that decision, since a tripwire names the decision
or the instruction it guards and a head exists from the landing of its work, per
`design@agent-skills@design-home-is-built-intent`.

### A tripwire's wording is the agent's, and the owner rules only on whether its cause is watched `##tripwire-wording-is-the-agents`

The owner's word on a premortem's cause, per `design@agent-skills@premortem-tripwires-on-the-owners-word`,
records that the cause is watched. The tripwire's firing evidence, response and re-entry point are
reworded when it is written, or on a review finding, on the agent's judgement, and each change is
listed in the turn's "Decided" section, per `design@agent-skills@decisions-reported-to-the-owner`.
A tripwire that watches another cause is a new one, and needs the owner's word. A ruling that bound
the wording would make every repair of a tripwire a question to the owner, against
`goal@knowledge-architect@agents-work-without-drift`, under which the owner rules only on what the
goals and the record leave open, which a tripwire's wording is not. The rival, a change to a
tripwire's firing evidence treated as a material finding, puts each such repair back to the owner.

### A fork closes on the owner's word, and a thread with one shape standing closes on its argument `##owner-rules-at-a-fork`

`skill@knowledge-architect-design@threads-and-states` closes a thread in the state `argued` when
one shape stands, each rival out by a named fact or binding constraint, and the session closes the
rivals `ruled-out`, naming the fact. Both show in the next delta and in the checkpoint, where the
owner contests or overrules them, and the turn lists them under "Decided", per
`design@agent-skills@decisions-reported-to-the-owner`. A fork, two or more shapes that meet every
binding constraint with no fact defeating either, closes on the owner's word, as every question
`design@agent-skills@questions-go-to-their-authority` reserves to the owner does. A cost alone makes no fork.
The discussion stays an exchange: the owner proposes designs, the agent fills and argues them, and
either side's shape can win.

It serves `goal@knowledge-architect@agents-work-without-drift`, under which the owner does not
answer what the record already answers, and `goal@knowledge-architect@the-owner-decides`, which
keeps a fork with the owner. The discussion form is the owner's intent: the owner observed that,
most of the time, the owner's rough designs end up a better direction than the agent's proposals,
the agent mostly refining them, arguing the smaller items under them and measuring their cost,
though not always, the better design coming at times from the agent or from a fusion of both; and the owner
truly thinks that the discussion format drives the improvement and the quality of a design. The nearest
rival, every thread closing on its argument, gives up that exchange where it matters, at a fork. A
second rival, the owner closing any thread whose choice weighs a cost, closes every thread on the
owner's word, since nearly every choice has a cost or can be made to have one. The rule is the
design skill's tie for the owner at loop step 5 and its rule that no fork remains where no rival
stands, joined to closure. The
risk, a session judging a rival defeated on a weak argument, is what the checkpoint's contest and
`tripwire@agent-skills@argued-closure-reopened` watch.

### A default that holds no fork, on bounded work or settled by the record, is built without waiting for the owner's word `##derived-decision-proceeds-on-default`

The bounded path's one message names its default, which the session builds: the owner's answer
stops or redirects it, and a part that cannot be undone waits for the owner's word, per
`skill@knowledge-architect-design@bounded-work`, and the turn lists it under "Decided", per
`design@agent-skills@decisions-reported-to-the-owner`. A default whose every premise comes from
the record, a measurement or a source is the session's decision in the same way: in a plan
review's findings, written into the plan document with its ground and no label, per
`skill@knowledge-architect-planning@plan-reviews`; in a review's items, per
`skill@knowledge-architect-review@what-review-leaves`; and in an audit's rule read two ways that a
record states, per `skill@knowledge-architect-project-audit@outcomes`.

It serves `goal@knowledge-architect@the-owner-decides`, under which the record settles what such a
default decides, since it holds no fork, while the owner, told of it in the bounded path's one
message or in the turn's "Decided" section, overrules it at any time. The nearest rival, the wait
for the owner's word, sends the owner the defaults that, by the owner's report recorded in
`design@agent-skills@questions-go-to-their-authority`, the recorded design heads should answer. It
is a delivery of
`design@agent-skills@questions-go-to-their-authority` where a skill writes a default, and a head
of its own because its rival, the wait, is another than that head's.

### A default put to the owner is built, and the branch that builds it merges once the owner has ruled on it `##default-ruled-before-merge`

`primer@question-authority` states it. A question the primer gives the owner, put with a default
the session judges sensible, lets the work go on with that default, as row 1 of
`primer@met-outside-the-task` and `skill@knowledge-architect-planning@working-a-slice` build it.
The owner's ruling is needed before the branch that builds the default merges, so the session
puts every default still open to the owner by the merge at the latest. A question with no default
the work can build on, a contradiction or an action that cannot be undone, holds its part instead,
per `design@agent-skills@owner-word-holds-as-far-as-its-premise` and
`goal@knowledge-architect@the-owner-s-word-is-verified`.

The timing is the owner's: the work goes on with no interruption, and the owner's approval is asked
at the merge at the latest. Two rivals lost. The work waiting for each answer of the owner
interrupts the work, which the owner does not want, and makes a default no default. A branch
merging with a default unruled puts on the main branch a choice the owner has not made, against
`goal@knowledge-architect@the-owner-decides`; the check of a plan document's deletion in
`skill@knowledge-architect-planning@plan-document-leaves` catches it only when the document
leaves, which for a milestone is its last slice.

### A design thread's slug becomes its entry's slug `##thread-slug-is-entry-id`

A thread of a design discussion is named by a slug minted in the entry-id grammar and checked for
a collision with the entries of the Component before it is used. In discussion prose it is written
plain, with a hash sign before it, never as a backticked span, which the checker reports as the
retired slug form once an entry carries that slug, per
`design@core@candidate-rule-and-retired-forms`. When
the thread is approved, or closed `argued`, and its decision earns an entry, the entry's heading ends with the same
slug, except in the case below, so the spec, the commit messages and the design home name the decision with one identifier.
In the plan document the thread is an item, a level-three heading ending with the same slug under
the Threads section, cited from inside that document only, per `design@core@plan-items-by-section`.

A thread is named by the decision it would record if approved, not by the change it proposes. A
thread's slug is minted in the round it opens, and the approved shape can drift from the proposal,
so the rule has one exception: when the thread's slug misdescribes the decision as approved, the
entry takes a slug that names the decision, and the plan document's harvest row, or the commit
message on the in-change path, states the pair. A slug that misdescribes its decision misinforms
every reader of every citation, and the alignment rule of `primer@design-heads` orders a rename
in that case; without the exception, the two rules would give opposite orders, against
`goal@agent-skills@one-skill-per-activity`. The rival, renaming the thread during the discussion
once it drifts, lost: every earlier delta carries the old slug, and the assembly that reads them
from the transcript, per `design@agent-skills@ledger-from-transcript`, would read one thread as
two.

### The issues and tripwires a piece of work bears on are searched by an installed agent of its own, apart from the standing-state reviewer `##standing-entry-search-agent`

The installed agent `agent@knowledge-architect-standing-entry-searcher` searches one group of a
project's issue and tripwire entries for those a piece of work bears on. Its description carries
the dispatch rule, since a dispatcher reads an agent's description and never its body: count the
rows of the issue and tripwire listings, cut them into groups by the rule of
`design@agent-skills@standing-entry-search-groups`, send one agent per group, all in parallel, and
brief each with the work, the seeds and its group. A seed is a decision or a goal the work names.
Its body fixes the order of the rows, reads every entry of its group whole, follows each seed
through `show` to the entries that cite it, which it always returns, and judges every entry of its
group against the work, leaning to recall. It returns every entry of its group in one of two lists,
the entries that bear on the work, each with a reason, and the entries judged unrelated, so that a
silent report cannot be mistaken for a skipped one. The session reads whole every entry returned,
never from its reason line. It relies on the checker working as intended: `show` prints an entry's
file and definition line, and the references to it.

The search runs in a subagent so that the main session spends its context on the entries the work
bears on, per `goal@knowledge-architect@agents-get-a-complete-workflow`, and the session still
reads each of those whole, as the owner asked, to "find and read the full reported issues and
tripwires, rather than relying on the subagent summary", since the session acts on the entry and
not on a summary of it, per `goal@knowledge-architect@the-owner-decides`. It is an agent of its own rather
than the standing-state reviewer in a second mode: that reviewer reads every entry by rule, the
search filters by relevance on purpose, and one text holding both standards lets a session apply
the wrong one.

### The search reads every standing entry whole, split over parallel agents in groups of a bounded number of entries `##standing-entry-search-groups`

Every entry is read whole rather than by its title, since a title shows neither a tripwire's firing
clause nor a deferred trigger. It is read by several agents rather than by one: one agent's load
grows linearly with the project, so one agent reads every entry whole only while its context holds
them, and cannot judge so many entries reliably; a group size bounds each agent's load whatever the
project's size. The groups are consecutive positions of the combined rows of the issue and
tripwire listings, 60 or fewer each, the fewest groups that keep that bound, with sizes that differ
by at most one. Equal groups keep a last group of a few entries from costing an agent of its own
while another carries the full bound. The unit of a group is the entry rather than the anchor: in
thaum at its commit ef21314, one Component holds 51.9% of the bytes of the standing entries, so a
group per anchor bounds nothing. The bound of 60 rests on the mean size of an entry there, 1.6 KB,
and on no measurement of recall. Both figures are re-taken with `git ls-files` over the issue entry
files and the tripwires homes, then `wc -c`: summed by the anchor that holds each file for the
first, divided by the rows of the issue and tripwire listings for the second; real sessions measure
it, per `goal@knowledge-architect@the-workflow-improves-through-real-use`.

### The search runs where design is discussed, before the work, over every issue kind and every tripwire `##standing-entries-searched-before-the-work`

The installed design skill dispatches the search at its loop step 1, with the question as the work
and the decisions and goals read so far as seeds, for a design question and for bounded work alike,
per `design@agent-skills@bounded-path-in-design`. The installed planning skill dispatches it at
the design audit of a milestone slice, point 2 of `skill@knowledge-architect-planning@working-a-slice`, with the slice's spec and the milestone document
as the work and their decided entries and citations as seeds, and at the design audit of a spec
where one runs, with the spec as the work. The audit lists as a gap every
standing entry the slice's code bears on: a tripwire whose firing condition, or a deferred trigger,
the planned code meets, and an issue of any kind the slice's code touches, closes, makes worse or
depends on. A tripwire of the milestone document's premortem whose firing condition the planned
code meets is listed with them: it enters a tripwires home only at the harvest, so the search
cannot return it, and the audit reads it in the milestone document. The slice's grounding, point 1 of `skill@knowledge-architect-planning@working-a-slice`, reads no issues and no tripwires, since the audit's
search covers every anchor. Work that does not go through the design skill sends no search: the
standing-state review reads its deferred triggers, per
`design@agent-skills@standing-state-review-before-every-merge`, which this search adds to and never
replaces. The design skill
hosts a search before bounded work, at its grounding, per `design@agent-skills@bounded-path-in-design`.
A search before every other piece of work stays parked: its one host would be
the primer, which holds only what every session needs, per `design@agent-skills@primer-limit`, the
evidence for it is one instance, and a line sending every session to search before any work is a
conformance rule. In thaum, a move of the pinned checker met the trigger of a deferred issue that
named that move, and the session read it at no step.

The occasions are those where design is discussed, so that an entry the work bears on costs a
ruling before the work rather than rework after it, per
`goal@knowledge-architect@agents-work-without-drift`. The evidence is real use, per
`design@agent-skills@behaviour-additions-need-real-use`: in thaum, the standing-state reviewer found, after
the work, issues the changes bore on and tripwires that had fired, especially in the slices of large
milestones, and the repairs and decisions this forced could have been taken before the design and
the implementation. The owner's account is that the reviewer too often finds, after the work,
issues the changes bear on or tripwires that fired and bear on the design, especially on large work
within big milestones in thaum. The audit reads every issue kind because those late
findings included issues the changes touched, not only fired tripwires. The slices of thaum whose
reviews found such entries had their audits before the installed planning skill told an audit to
read the tripwires and the deferred issues, and the reviews of the two slices audited after it
found no entry missed. The
owner judged the work before 0.2.0 not that different from the work just before the change, so the
evidence stands; a stream of audits that read the standing entries in the session
and miss none would weaken it. A search at the reviews of
a plan document lost to a second search when convergence is proposed, before the premortem: those
reviews run only where a discussion wrote a plan document, and a search at convergence covers a
decision taken in the change under way too. The search at convergence is parked, with a tripwire.
The occasions are the owner's: the search runs at least at a design discussion's grounding and at
the audits of a plan's steps, the main moments design is discussed, and should miss nothing.

## How documents point at each other

### A reference is written where the text would have to be revisited if the entry it names changed `##a-reference-claims-a-revisit`

Every citeable thing has one reference form, `<kind>@<anchor>@<id>`, and two mechanisms of the
checker give a reference its value. `cargo klarch show <ref>` prints every reference to an entry,
so what depends on an entry is computed from the tree and never written by hand. An entry that is
deleted or renamed dangles every reference to it, and `cargo klarch check` reports each one, per
`design@core@an-unresolved-reference-is-reported`, so the repair list a change produces is the
list of texts that depended on what changed. A reference is
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
| a restatement of a directive sentence, per `design@agent-skills@restatement-size-test` | its home | a drift between the two is found from either end |

**An entry never lists what references it.** The inbound list is `show`'s to compute, and a
hand-written one is stale at the next reference written elsewhere.

**A reference in prose is checked wherever it stands**, a Rust comment and a fenced block
included; a string literal bound to a name yields none, per
`design@core@candidate-rule-and-retired-forms`. So a comment in code naming an issue is
as live as a sentence in a document, and closing the issue reaches the code. This serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`.

### A sentence about the past whose reference dangles is rewritten to the present or removed, except a deleted skill's or agent's reference in a released changelog section and an absent entry in a record of a deliberation, and a quotation of the owner is kept with a reference beside it `##a-past-sentence-is-rewritten`

When an entry is deleted or renamed, a sentence that recorded its past, such as what an earlier
step wrote, is rewritten to state the present, or removed, and its history stays in the commit
messages. Retargeting it to the new name would make it false, and plain text would take it out of
the check, per `design@agent-skills@plain-text-is-no-repair`. A verbatim quotation of the
owner that names a renamed entry is left as it is, with a reference to the current entry beside
it, since rewriting it at a rename, with no change to what it decides, risks misstating the owner,
the risk `design@agent-skills@attribution-replaces-quotation` names, against
`goal@knowledge-architect@the-owner-decides`.
In a released section of CHANGELOG.md, a reference to a deleted skill or agent goes back to the
bare name instead, per `design@agent-skills@plain-text-is-no-repair`. In a record of a deliberation,
an entry its tree does not hold is named by its bare slug instead, per the same head.

### Plain text is never the repair of a finding, except text in the checker's syntax beside the issue that records its missing form, a deleted skill's or agent's name in a released changelog section, and an absent entry's bare slug in a record of a deliberation, and no instruction offers an unchecked form as the way to clear one `##plain-text-is-no-repair`

A form a writer can use is either one the checker judges, and the workflow recommends it, or one
the checker does not read, and the workflow never directs a pointer into it to clear a finding. So
no finding's repair, and no installed or project instruction, offers an unchecked form as the way
to clear a finding. A repair names a checked form: the right anchor, `path@elsewhere@<path>` for a path this
tree does not hold, `planned@<anchor>@<path>` in a plan document for a path its work will create,
per `design@core@planned-path-form`, an angle-bracket placeholder for an illustration, or a
rewrite of the sentence.
One exception is text in the checker's syntax that no checked form expresses, written without
backticks beside a reference to the issue entry that records the missing form. Prose that mentions a
directory without asking the reader to follow it is not a repair and is outside this head.

The checker cannot express every reference a project needs. The first exception covers text in the checker's
syntax, which would be read as a candidate if it were backticked: a reference,
`<kind>@<anchor>@<id>`, or a path of two or more segments. Where no checked form expresses what such
text points at, it may be written without backticks, and only beside a reference to an issue entry
of the writing project that records the missing form. Any other text that names something, in the
tree or outside it, such as another project's commit, an address on the web or a description in
words, is outside the exception. Whether it needs a reference is decided by
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

The scope is the checker's syntax, the owner's proposal: the rule was meant for plain text that
copies the checker's syntax but omits the backticks to avoid the check, which the owner thought a
better scope. It lets a writer tell from each span alone whether the exception
applies, per `goal@knowledge-architect@agents-get-a-complete-workflow`: every sentence names
something, and a scope of "any pointer" would ask for an issue entry beside every mention of a
thing outside the project. The evasion the exception exists to stop is a reference or a path with its
backticks removed, which this scope covers. A scope by target, any text naming something the tree
holds, lost: when such text needs a reference is already decided by the rule on references, and
whether a phrase names something cannot be decided span by span.

A commit named by its subject, as `design@knowledge-architect@no-branch-sha-is-cited` directs for a commit of the
branch, is outside the exception. It names history as git names it. A reference resolves against the
entities of a tree, per `design@core@one-entity-table`, and a commit is none of them; the one
citation of a commit the checker judges is a branch commit's SHA, which
`design@core@branch-shas-are-refused` refuses.

The second exception is a deleted skill's or agent's reference in a released section of
CHANGELOG.md: the section's content never changes, per
`design@knowledge-architect@changelog-section-fixed-at-release`, and nothing exists that a
reference could cite.
Its repair is the bare name, which `design@core@bare-skill-name-reported` leaves silent. It evades
nothing, since no checked form remains that it could have used. The rival, rewriting the sentence
as `design@agent-skills@a-past-sentence-is-rewritten` asks elsewhere, would change a released
section's content.

The third exception is an entry that the tree of a record of a deliberation does not hold, nor, for
a commit message, its first parent's, removed or renamed by the work the record carries, or still to
be created by it, named there by its bare slug in backticks. A record of a deliberation is where the deliberation is kept: a plan document
or a commit message, per `skill@knowledge-architect-decision-recording@three-homes`, and a committed
file that leaves once its outcomes are carried out, per `primer@design-heads`. The exception
for an entry the work removes or is still to create is the owner's proposal: for such an entry the
checker has nothing left to check, and what this head refuses is avoiding the check on a live
entry, which leaves dangling references and makes grounding less efficient. A renamed entry's old
slug is in the same case, since nothing resolves it once the rename lands. It evades nothing, as the changelog exception does not: no entry remains,
or none exists yet, that a checked form could cite, and a one-segment span is silent, per
`design@core@candidate-rule-and-retired-forms`. The owner chose the form among three, the bare slug
in backticks, the bare slug in plain text and the full reference with its backticks removed, and the
scope:
records of a deliberation only, since every other document states current reality, where a
sentence naming an absent entry is rewritten, per `design@agent-skills@a-past-sentence-is-rewritten`.
The full reference with its backticks removed is the evasion this head refuses. The bare slug in
plain text, the owner's first proposal, gave way to the owner's choice of the backticked slug. A
description in words, the form the audit report's issue proposed, the owner judged confusing to
every reader. A
checked form for an entry still to be created, as `design@core@planned-path-form` is for a path,
lost too: a plan document and a committed file leave with or before the branch that creates the
entry, so such a check would fire on a file about to be deleted; a commit message is judged against
its own commit's tree, where the check would assert only that the entry is absent, which nothing
reads again; and the form would need a grammar of its own in the core.
A renamed entry still exists, under its new slug, so its old slug carries beside it the reference
to the live entry, the shape a quotation of the owner naming a renamed entry already takes, per
`design@agent-skills@a-past-sentence-is-rewritten`: the old slug alone would leave a reader no
checked way to the entry it names.

The set of exceptions is closed: each lets a finding clear without a checked form,
so another one weakens the check, and is argued as a change of this decision.

The argument: an unchecked form that clears a finding clears it for good, so a habit of writing one
empties the check while every run still passes, against
`goal@knowledge-architect@documentation-stays-consistent`. Two narrower stances of the checker
make the same argument: `design@core@the-regime-has-no-opt-out`, where no declaration exempts a
document from a rule, and `design@core@reserved-anchors`, where the escape anchor is refused on a
path this tree holds, since it would otherwise silence the finding on a real path.

A label beside a checked pointer is not such a form. A plan item named outside its plan as #<id>
stands beside a citation of the whole plan, because an item reference is refused there, per
`design@core@plan-item-scope`.

## Plan documents

### The plan documents' words follow common developer usage, one word to one meaning `##document-vocabulary`

A plan document is a spec, a milestone document or the spec of a slice, kept in the plans
directory, docs/plans/ at the project's root, whose path the checker fixes, per
`design@core@plans-dir-fixed`. The directory is named plans rather than planned: in common English,
planned work is intended or scheduled work, designed or not, which is the roadmap's content. A spec is the plan document of work done in one branch and one PR. A milestone is work across
several PRs with design sessions between them; its plan documents are its milestone document and
one spec per slice. A slice is a part of a milestone that is one branch and one PR, with its own
spec. A step is one item of an implementation sequence, in a spec or in a slice, and no unit of
the history, per `design@agent-skills@step-is-no-commit`. The word "plan" alone never names a document: it would name the directory, a
document and a kind of document at once. The words follow common usage among developers, which the
owner made binding: "the vocabulary must be conform with common usage, especially in the developing
community". A milestone groups the work toward one goal, as GitLab's milestones do, and a
spec says what will be built and how before the code exists, in the sense engineering teams give
the word. "Design doc", the closest common term, lost because "design" already names the durable
register. One word for both a milestone's part and an item of a spec's sequence lets a rule for
the one, one branch and one merge per part, contradict the other, a spec of several steps on one
branch. So the milestone's part takes its own word, slice, on the owner's proposal, a part of the
larger whole, and "step" keeps the general sense it has in common usage and in the design skill's
loop.

### A spec's step is no unit of the history: how many commits it takes is the implementing session's to judge `##step-is-no-commit`

Naming a spec's
items by a unit of the history, one step to one commit, lost: it fixes the plan onto the
history, against `goal@agent-skills@installed-text-leaves-room-to-judge`, and a commit the work
needs for another reason, such as a fix met outside the task, would then contradict the plan. The
owner ruled it out: "since it designates something 'real', it rigidifies the workflow and
'hardcodes' that one step equals one commit. This is likely to cause conflicts with other workflow
instructions, for example fixing with a commit a 'met outside this task' item".

### A layer of planned work has one plan document `##spec-and-milestone`

The work of one PR has one plan document, its spec. A milestone has a milestone document and one
spec per slice, which hold the spec's sections between them, split as
`design@agent-skills@milestone-split-by-lifetime` says. The two layers are the owner's proposal: "I
would decompose this into two layers: plans for short work, and plan for multi session work". A
spec plus a separate detailed implementation plan lost: both carry the same decisions and the
second drifts from the first, against `goal@knowledge-architect@documentation-stays-consistent`.
The one exception is a detailed plan, if one is ever written for a less capable implementer: it covers a bounded
amount of work and opens by saying it rests on assumptions, per
`design@agent-skills@no-untested-snippet-is-authority`.

### No untested code snippet in a plan document is authority `##no-untested-snippet-is-authority`

A plan document is detailed about the design and concise about the implementation sequence, and no
untested code snippet in it is presented as authority: a snippet is labelled as an illustration of
a shape. The rule is the owner's: "a detailed design/spec document, with a concise implementation
sequencing plan, is much better than a very detailed plan", from what the owner saw of detailed
plans: "an implementer that is only given the resulting plan often takes it as absolute truth, and
does anything to make the code snippets work, at all costs (sometimes breaking everything else
instead). They also often keep comments in snippets (that are meant as guidelines for the
implementer) verbatim in the real project", and "it must keep a disclaimer that content is based on
assumption and might not be perfect". The rival, a plan document whose snippets read as
instructions, is not prevented by one document per layer, since a spec with no separate plan can
still carry them. Asked, under entry test 4, whether these words record the owner's intent, the
owner answered: "this one is quite important and I want to give it a higher ranking. It could
apply elsewhere in the future too."

### A milestone is a directory, its head a README, each slice a spec `##milestone-is-a-directory`

A milestone's plan documents are one directory under milestones/ in the plans directory, which the
checker makes an anchor named by its basename, per `design@core@plan-document-kinds`: the milestone
document is its `README.md`, a generated `index.md` lists the slices, and each slice's spec is a file
beside it, cited `spec@<milestone>@<slice>`. The head is a README because the checker
resolves a relative link only in a `README.md` or an `index.md`, per
`design@core@links-are-navigation-rows`, so the head can link each slice's spec as a navigation row.
A slice's spec is a spec, so it leaves when its slice lands, per
`design@agent-skills@plan-document-leaves-at-landing`: its decisions are then in the design homes. The
README leaves with the last slice.

### A milestone's design is split across its documents by lifetime, from the session that converged `##milestone-split-by-lifetime`

The design of a milestone is split across its documents from the start, by lifetime: a slice's spec holds what
only that slice builds, and the README holds what crosses slices or outlives one, every item cited
from more than one slice's document included. A README holding every slice's design is read whole
at every slice's grounding and grows with every slice: the README of the structured-plans milestone
held 1,624 lines when it left, counted with `wc -l` on `git show` of its deleting commit's parent.
The slice specs exist from the session that converged, so the audit of a slice applies its findings
in place, in whichever document holds the shape, and a new slice found at an audit gets a spec of its
own, as a scope change the owner rules on. The audit's commit message lists each finding with its
gap, its answer and the decision it follows from, so an edit in place loses no record of what the
audit found.

### A plan document leaves when its work lands `##plan-document-leaves-at-landing`

A plan document is deleted in the commit that completes its last harvest, and that commit's message
cites it by its kind, which resolves against the commit's parent. A plan document kept after its harvest is a second home for every decision it
carried, and it starts drifting at the first later reversal. Walked by the checker, its references
break at every reversal, and someone repairs a document about finished work; left out of the walk,
it is unchecked text that a grep finds with no marker that it is stale. Either breaks
`goal@knowledge-architect@documentation-stays-consistent`. Leaving the choice to each project lost
too: the installed planning skill would have nothing to say where a document's work ends, against
`goal@knowledge-architect@agents-get-a-complete-workflow`.

### A discussion whose decision lands in the change under way keeps its deliberation in that change's commit message `##in-change-path`

The design skill has two paths, which differ only in where the deliberation is kept until the
harvest. The in-change path is open when the decision lands in the change under way: its work,
where that change began before the discussion opened, as an issue fix or a review repair does, or
its record, whenever the discussion opened, for a decision recorded when it is made, per
`design@agent-skills@design-home-is-built-intent`. It writes no plan document. The message of the
commit that writes the decision's design entry, or that implements a decision that earns none,
carries what a plan document would: every thread with its final state, the ground of each
closure, the owner's ruling attributed per `design@agent-skills@attribution-replaces-quotation` or
the argument of a thread closed `argued`, the rivals that lost with their reasons, and the rulings on tripwires. The
premortem runs when reversal touches any of the four things the skill names: stored data, a
consumed interface, behaviour users have adapted to, and a decided thread that would reopen. The four never open the in-change path: a decision that is cheap to
reverse and whose work comes later takes the full path, per
`design@agent-skills@design-hands-off-to-planning`, as does any discussion that converges on work
that does not land in the change under way. A requested change that is not bounded, and whose work
has not started when its discussion opens, takes the full path too, even when one pull request
builds it. Decided and unbuilt intent then has one home, the plan document, per
`design@agent-skills@design-home-is-built-intent`.

A plan document exists to carry a decision from convergence to a landing that comes later, and to
be reviewed before its work is built. A decision met during a change begun before the discussion
lands in work already under way, so neither applies, and requiring a spec would make every
decision met during a task cost a plan document, which is what
`design@agent-skills@new-or-reshaped-head-needs-design` must not cost to be followed. A requested
change whose work has not started has all of that work ahead of it, and its plan can be reviewed
before any of it is built. The move of an adopting project's existing documents starts as such a
change, and it is planned work under the planning skill, per
`design@agent-skills@adopting-existing-docs`. The cost is a spec, its plan reviews, its harvest and
its deletion for every requested change that is not bounded, including one the converging session
builds in one pull request, which strains the "fewer review rounds" of
`goal@knowledge-architect@agents-get-a-complete-workflow`. Widening the existing short path, rather
than adding a path for one decision met during a task, is the owner's proposal, on the owner's
argument, given as the owner's opinion, that a discussion that starts with one decision should be
bounded in neither threads nor rounds, since its size depends on what grounding, investigation and
experiment bring, which nobody knows in advance. The separate path bounded the discussion to one thread, against
`design@agent-skills@structure-the-flow`. The cost is a long commit message when a discussion
inside a task grows to several threads; its length is not bounded, since a bound on it would bound
the discussion.

### The planning skill writes the plan document, and the design skill writes none `##design-hands-off-to-planning`

The planning skill starts where a design discussion has converged on the full path, and writes the
spec or the milestone. A discussion on the in-change path has no plan document,
per `design@agent-skills@in-change-path`. The design skill ends at convergence, the premortem and
the owner's rulings on its tripwires and acceptance criteria, and writes no plan document of its
own. Its decisions are
recorded when their work lands, per `design@agent-skills@design-home-is-built-intent`. One skill owns the document's shape: two skills describing the sections of one document
would drift apart, against `goal@knowledge-architect@agents-get-a-complete-workflow`.

### A plan document is written in the session where its discussion converged `##plan-written-in-the-converging-session`

The planning skill writes the spec or the milestone in the session where the design discussion
converged. The session
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
the document leaves. The plan document is written in the session that converged, per
`design@agent-skills@plan-written-in-the-converging-session`, so the criteria and the document
are born together.

### An acceptance criterion stands on the owner's word, as a tripwire does `##acceptance-criteria-on-the-owners-word`

The owner rules on whether an acceptance criterion is applied, and on the decision its firing
reopens. The agent words its observable, and lists a rewording in the turn's "Decided" section.
The design skill asks for the ruling at the premortem, beside the tripwires. A criterion first
proposed later, at the assembly, by a review or at an audit, is written marked as awaiting the
owner, with its label under the plan document's defaults, and is not judged before the ruling; the
work goes on without it, and the branch that carries it merges once the owner has ruled on it, as
any default put to the owner, per `design@agent-skills@default-ruled-before-merge`.

A criterion's response reopens a decision, and choosing whether to watch with it is a fork, which is
the owner's, per `goal@knowledge-architect@the-owner-decides`: applying it and not applying it both
meet every binding constraint, and they trade the cost of the check against the risk. The owner
sets a criterion's threshold, and a tripwire, the same check for built work, stands on the owner's
word, per `design@agent-skills@premortem-tripwires-on-the-owners-word`, so a criterion's application
stands on the same word as its threshold and as a tripwire. The wording stays the agent's, as a
tripwire's does, per `design@agent-skills@tripwire-wording-is-the-agents`, and the label a criterion
is put to the owner under is `design@agent-skills@ruled-items-labelled`'s. Both acceptance criteria
that reached a plan document of this repository against a goal had no ruling: one was proposed at a
premortem, one while a slice's spec was written. The rival, no ruling and the design-conformance
reviewer alone, catches a conflict with the record, not a weighing.

### A plan document is committed before its reviews, which read that commit `##plan-reviewed-as-a-commit`

The planning skill has a plan document's reviewers read its commit, not the working tree. The review
skill names what a reviewer reads as a commit range and gives each reviewer that runs the tests,
the checker or a program the project builds a worktree detached at the commit under review; a document still uncommitted has neither, so the two
skills could not both be obeyed. A repair lands after the review, per
`design@agent-skills@review-repair-appended-or-folded`.

### A review repair is appended, and folded only where appending leaves an earlier commit failing `##review-repair-appended-or-folded`

The review skill has a repair land as a new commit after the work it repairs, which edits no
history. Where a project requires every commit of a branch to pass checks the repair changes, an
appended repair leaves the earlier commits failing, and the project's rule and this one leave no
legal move; the repair is then folded into the earliest commit it repairs, with a clean tree,
confirmed to have lost no content. The record of the review says what was folded, so the landing
history still says what the review found. The rule exists to avoid history edits, not to keep a
repair apart from what it repairs, which is the owner's reason for it.

### A plan document's landing is not tied to its work's, except before work that changes what the per-commit gate checks `##plan-landing-is-not-tied-to-its-work`

A plan document, a spec or a milestone document, is merged to the main branch in a pull request of
its own or with its work, whatever the time of its work. An owner
plans a feature that will not be built yet so that the work done meanwhile does not drift from it,
which needs the document where every session reads it, on the main branch; that serves
`goal@knowledge-architect@agents-work-without-drift`. What "spec" and "milestone" measure is the
work, one pull request or several, not where the document lands. The document leaves when its work
lands, per `design@agent-skills@plan-document-leaves-at-landing`.

The owner's intent: "It regularly happens that I will prepare a spec/design for a feature that I
won't be implementing right now, just to ensure that the intermediate tasks I will perform do not
get misaligned too much with that planned feature. Anyway, what I mean is that writing plan
document should not be constrained with when its implementation work happens." Asked whether it
records the owner's intent, the owner answered: "Q1 yes".

The one exception: a plan document lands before any work that changes what the project's
per-commit gate checks. The per-commit gate judges each commit of a branch, its tree and its
message, with the checker at the branch's tip, per `design@core@a-commit-message-is-a-document`.
On one branch, that gate as the work's first commit changes it would judge the
commit that added the document, whose tree predates the change, and refuse it. A change that only a
check of the working tree sees, such as installed text that the checker compares on the working
tree alone, leaves the per-commit gate unchanged and is outside this exception. Keeping both on one branch would force
the fix the owner called absurd: the plan document committed after the work it plans. So that work
begins on a branch of its own, after the document is merged. This holds for a spec as for a milestone document, whose slice
that changes what the per-commit gate checks is the one this exception meets.

### Undesigned work is an issue `##undesigned-work-is-an-issue`

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
of the issue it closes, and the commit that deletes one removes its row. The need is observed in
real use: thaum kept a file of its next milestones as an exception to the installed skill. A
roadmap register of its own, holding undesigned work, lost: two registers would each hold known,
undesigned work, which needs a routing rule and hides one of them from the issue listing.

### The roadmap's order is the owner's `##roadmap-order-is-the-owners`

A row of the roadmap is added or moved on the owner's word, since ordering known work is a fork: no
goal or decision ranks one piece of it over another, so the order is the owner's, per
`goal@knowledge-architect@the-owner-decides`. The rival, any session ordering the work, would make
the order a session's choice where no argument settles it. The two edits that keep a row pointing at the same work, rewriting
the row of an issue a new plan document closes and removing the row of a plan document that
leaves, need no word, per `design@agent-skills@roadmap-orders-issues`.

### No record of landed work is kept `##no-progress-record`

No installed skill asks for a file recording what has landed. History lists every plan document
that left, and each landing commit says where its results live; releases are in the changelog. A
progress file would be a third document about the same work, with a lifetime of its own, against
the one place for what is open of `goal@knowledge-architect@structure-and-workflow-work-together`.

### A transcript reviewer checks that what a work's sessions established has a durable outcome, and that no ruling of the owner is recorded in a state or a scope the owner did not give `##transcript-reviewer-agent`

The installed agent `agent@knowledge-architect-transcript-reviewer` reads the transcripts of the
sessions that produced a piece of work, named as a commit range, and checks two things. First, that
each decision of the owner, each argument that decided something, each finding of a reviewer and
each thing met outside the task has a durable outcome: recorded in its home, acted on by a repair or
an issue, or judged to need nothing with the reason written where the owner reads it. What lives
only in a conversation is lost when the session ends or is compacted. Second, that no ruling of the
owner is recorded in a state or a scope the owner did not give. Detail added inside a ruling, and a
wording better than the one the owner was shown, are not findings. The owner was bothered that the reviewer flagged every small addition as a finding, even a clearly
sane deviation, and asked it to focus more in the other direction: whether something from the
transcript was forgotten or not handled durably. A ruling is a word of the owner on a question
`primer@question-authority` reserves to the owner; an answer on a reading or a fact is checked
against its source, per `design@agent-skills@answer-keeps-its-authority`. The reviewer assumes no shape of
the work: a plan document is one home among the others.

It is an agent, not a line in a skill, because its standard and its extraction rule are fixed, and
the rule is learned from a failure: a filter on text substrings once dropped one of the owner's
messages. It serves `goal@knowledge-architect@the-owner-decides`. The durability check serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. The check of rulings has shown
its worth: in the review of the change that installed the first skills, a reviewer briefed with it
was the one of six to find a decision recorded narrower than the owner's approval. The milestone
document of the v0.1 extraction holds that count, in its section on session B of step 5, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it; the message of the commit "[review]
Six reviewers read PR 1: the hand-off rule, the shipped text and the record are corrected" records
the review's six axes and its finding.

### A misstated ruling is critical only when it reverses the ruling, or changes what is built or a load-bearing decision `##misstated-ruling-rated-by-what-it-changes`

A misstated ruling is critical only when it is reversed, or when its state or scope changes what is
built or a load-bearing decision. A ruling recorded a little wider, narrower or firmer on a detail
that is not load-bearing is minor, still goes to the owner, and is no finding of the retrospective
once caught before the merge. Any other misstated ruling is major. The cut is what the misstatement
would build: a reversal, or a change of what is built or of a decision others rest on, produces
wrong work if it merges, where a slip on a detail that bears on nothing built produces none, and is
repaired once the owner sees it. That such a slip is no critical defect is the owner's weighing,
from the owner's observation of real sessions: "everytime, it was on details that were small and
not load bearing in my opinion", "the transcript reviewer is catching those issues reliably IMO. I do not believe any amount of instruction anywhere would help improve
more than the current situation: LLM agents are trained to take the input they receive from humans
seriously and follow them closely", and "This kind of findings being flagged by the transcript
reviewer once in a while (for decisions that are not the most criticals, and with only a small
deviation from my word, not a complete reversal) is not a critical defect and should not be
considered as such." The rival, every misstated ruling rated critical, made a reviewer's report of a
slip read as a critical defect, which the owner ruled it is not; the slip is still repaired, and it
is no defect of the workflow.

### A plan document records the whole discussion `##plan-records-the-exchange`

A plan document assembled from a discussion records every thread with its proposer and round, its
final state, the arguments on each side, the owner's rulings verbatim with their round, and its
relations. The record is the owner's proposal: "I'd like to have the initial spec aim to be contain
a precise record of the design work and exchange that happened, with all threads written, their
final states, and the arguments, with the possibility of cross referencing all these items, bounded
to the spec document or the milestone directory." It applies to a spec, and to a milestone document
with its slice specs, which share the record by the rule of
`design@agent-skills@milestone-split-by-lifetime`; the owner extended it to milestones: "Milestones are
created through the design skill too after all." `design@agent-skills@standing-argument-in-head`
names the plan document as the home of the deliberation while it exists, and
`goal@knowledge-architect@the-owner-decides` is served only where the rulings are recorded as the
owner made them. On the in-change path, the commit message carries the same record, for the same
reason, per `design@agent-skills@in-change-path`. The rival, a plan document recording each thread's
final state and resolution, leaves the rulings and the arguments to memory. The cost is a longer plan
document to write and to read.

### An argument is an item, without a state `##arguments-as-items`

Each argument of the discussion is an item of the plan document, under its Arguments section, so a
ruling, a closure or a premortem cause can cite what decided it, and the standing argument a
harvest writes is a selection of named arguments rather than a rewrite. It carries no state: a
state would need a relation between many arguments and many threads, which the design skill
declines to track, and an identifier needs none.

### A leaving plan's citations are removed, and each citing plan gets a question `##retiring-plan-opens-issue`

A whole plan document may be cited from another plan, per `design@core@plan-item-scope`. When it
leaves, the session that meets the dangling citation, the one deleting it or the one rebasing the
citing plan onto the deletion, removes the citation and opens a `question` issue on the citing
plan: does it still hold now that the leaving plan is built, accounting for deviations or other
unplanned happenings? The issue cites
the citing plan, so it cannot outlive it, and its `Why it matters` cites what the leaving plan
harvested, since that plan no longer exists. It is a `question` rather than a `todo` because the
reading may find nothing to change. For a milestone, its next slice's audit reads it. A milestone's
own documents are one plan, per `design@core@plan-item-scope`, so its milestone document and its
sibling slice specs re-point a citation of a leaving slice with no question: that audit already
reads the milestone against what landed. The retiring
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

### The standing-state review runs before every merge, and reads every tripwire and every deferred trigger `##standing-state-review-before-every-merge`

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
trigger met by work that does not go through the design skill has no reader at all.

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
departure from the recorded design is caught by a check or a review before it merges". The owner
named the lack: "nothing seems to be reviewing a plan document against recorded goals and design."
The other reviewers read the document against itself, against the code, or against the discussion.
The design audit reads design homes, but not goals, and it is run by the session that implements the
work, which may be the document's author, and skipped for a spec worked in the session that
converged. Two acceptance criteria that
`goal@knowledge-architect@the-workflow-improves-through-real-use` rules out each passed a plan
document's three reviews in this repository. The deleted spec synthetic-evidence-and-plan-reviews
holds both, as its instances 1 and 2, and `git log --diff-filter=D --name-only -- docs/plans/`
finds it. The decision's nearest rival, the decision-record reviewer
extended to plan documents, judges the record a diff writes and holds a plan's items as not
decisions yet, so it would carry two questions with two yardsticks. The cold implementer, the other
rival, asks whether the document can be acted on, and a document that contradicts a goal can. The
reviewer depends on a record worded wide enough. The owner's words state where a miss then belongs:
"this is always dependent on the project being diligent in its goals and design records. Everything
in the workflow is, after all. Project owners still have to watch agent behavior and interrupt them
if they find them acting differently than their intent. Whether this is a published workflow defect
or an internal documentation defect is left to the retrospective skill to judge." Its cost is one
more dispatch at each of those moments; what it buys is a reader of the record that neither the
author nor the implementing session is, which the two acceptance criteria above show the other
reviews lacked.

### Before every merge, the transcript reviewer runs alone and last, and not with the other axes `##transcript-review-last-before-merge`

The installed review skill sends the transcript reviewer before every merge to the main branch
once, after every other axis has run and its repairs are committed, and not also in their batch, over the whole branch and the
transcripts of every session that worked on it. The findings of the other axes reach the session as
messages of its transcript, so only a reviewer reading it after their repairs can tell whether each
was acted on durably, per `goal@knowledge-architect@design-is-recorded-with-its-arguments`. A
transcript reviewer sent in their batch as well would read the transcript before the owner answers
the items the batch raises, and an answer of the owner given after its extraction reads to it as
missing, as `skill@knowledge-architect-review@review-axes` says: its findings would report as
missing what the last transcript review then finds answered. Its own repairs land as
`design@agent-skills@transcript-review-repairs-are-additional-commits` says.
An additional commit that adds, removes or reverses a design head is reviewed by the
decision-record axis at least, and the review ends with that review's repairs; any other is reviewed
by no axis again, so the review ends, per
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.

### The last transcript review's repairs land as additional commits, as many as their kinds need `##transcript-review-repairs-are-additional-commits`

The repairs of the last transcript review before a merge land as additional commits, as many as
their kinds need: a repair that the primer routes to a commit of its own stays one, and nothing asks
the dispatcher to merge them into a single commit. The additional commits are the owner's proposal:
"I'd just replace "as a last commit" by "as an additional commit". This is simpler, it prevents any
possible conflict, without exceptions and no need to check a condition."
A repair that would leave an earlier commit failing is folded, per
`design@agent-skills@review-repair-appended-or-folded`.

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
disappears, or a reversal under `skill@knowledge-architect-decision-recording@reversal-check`. Each round of repairs then adds at most as
many re-reviews as it adds, removes or reverses heads. A trigger on any repair that "makes a
decision" lost: whether a rewording makes one is a judgement, and read wide it re-reviews repairs
that only reword a head. The rival that sends every axis again at every repair lost on the same
cost. A narrowing of an approved head that this trigger leaves out still goes to the owner, per
`primer@design-heads`, and the last transcript review checks it against the owner's rulings.

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

### Every review whose diff adds to or changes agent-facing text owes an axis that judges each statement against the standard on that text `##review-judges-additions`

A row of `skill@knowledge-architect-review@review-axes` holds the axis on agent-facing text,
applicable to every diff that adds to or changes it, installed or a project's own, whatever
activity produced the diff. It judges each added or changed statement against
`skill@knowledge-architect-agent-configuration@admission-standard`, the standard every writer and
judge of that text applies, per `design@agent-skills@admission-rules-home`. No other axis judges
whether a statement is needed: the routing axis judges where it goes, and the reviewer agents leave
style aside. In the thaum session that wrote the statement the owner judged useless, five review axes passed
over it, by the message of thaum's commit that added it. The owner holds the review of a diff the cheaper place to catch such a statement,
against the cost of an audit.

The row sits in the review skill's table rather than among the axes the
agent-configuration skill adds to the reviews of its own activity: agent-facing text is also
edited inside other activities, such as an issue's closure or a plan's harvest, whose reviews an
axis of that skill would not reach. Text no branch touches is read by the need-test lens of the
agentic-workflow axis, per `design@agent-skills@workflow-axis-lenses`.

### The review axis on agent-facing text is carried by an installed agent, which reads the standard from the skill `##additions-review-agent`

`agent@knowledge-architect-agent-text-reviewer` holds the axis's procedure, naming the act of each
statement and then judging its evidence and its kind, and its report. For the standard itself it
reads `skill@knowledge-architect-agent-configuration@admission-standard` whole from the installed
skill's file, and holds no copy of it, per `design@agent-skills@admission-rules-home`. The nearest rival, a general-purpose subagent briefed with the axis's question, has each
dispatching session write the procedure into its brief anew, and a procedure rewritten per brief
drifts; every other review axis with a fixed standard has an agent of its own.

## Project audits

### Reading a whole project on one aspect is an activity with an installed skill of its own, and a review stays the reading of a diff `##audit-is-an-activity`

A project audit reads every entry or text of a corpus on one axis and brings what fails into line,
under `skill@knowledge-architect-project-audit`, which holds the method, the outcomes and one
section per axis; a repair of agent-facing text is judged by the standard of
`design@agent-skills@admission-rules-home`. Its artifact is a repaired
record across the project, where a review's is a verdict on one branch, so the two hold different
standards: what a diff owes, and what the whole record owes. The nearest rival, a whole-project
mode of the review skill, would hold both standards in one text. A skill per axis would restate the
method in each, against `goal@agent-skills@one-skill-per-activity`, while the method is generic
and each axis gives only its corpus, its dispatch and its outcomes, per
`design@agent-skills@audit-method`. Every review reads a diff, so without an audit a drift between
two heads no branch touches is read by no step of the workflow, against
`goal@knowledge-architect@documentation-stays-consistent`.

### An audit's method holds for every axis, each axis giving its own dispatch `##audit-method`

The method of `skill@knowledge-architect-project-audit@audit-method` holds for every axis. An axis
gives what varies: its corpus and what is read beside it, its dispatch, its drafts, its outcomes,
and how its findings are classified as on old or new text. Its dispatch, its drafts and the reading
of them are its saved workflow's, per `design@agent-skills@audit-axis-runs-a-workflow`, and each
pass is one run of it, with one more on the application where the axis judges it, per
`design@agent-skills@one-run-pass`. Each agent writes its drafts in a
scratch directory of its own and edits nothing else, and nothing an agent proposes is applied before
a reading by the session confirms it, per `design@agent-skills@synthetic-evidence-not-built`. A
quotation or an attribution of the owner is confirmed against the question it answered, as well as
against its words.
The items that need the owner's word go on one list per pass, labelled `F<n>`, in the pass's report
file, per `design@agent-skills@committed-audit-report`. The edits are applied and their diff read
whole. How passes follow one another is
`design@agent-skills@audit-passes-repeat-while-the-count-falls`.

The reading guards against a quotation of the owner verbatim but taken from the answer to another
question, which the first audit of the design-record axis on this repository showed. The rival that
keeps the design-record axis's groups and calibration sample in the method, a later axis adjusting
them as an exception, would state a rule that one of two axes breaks whole: the agentic-workflow
axis shares no corpus out, per `design@agent-skills@workflow-axis-lenses`. The groups and the
sample are the design-record axis's dispatch, per `design@agent-skills@design-record-axis-dispatch`.
No point of the method serves an axis with no saved workflow: every axis has one, and a new axis is
designed with its own. The rival that keeps such points for a future axis keeps text no session
follows, since the fallback for a harness without the tool is the script's own steps.

### Fresh agents re-check each pass of an audit, and passes repeat while the violations left fall, until the owner stops them `##audit-passes-repeat-while-the-count-falls`

After a pass's edits, the axis's saved workflow runs again, given the kept list and the commit the
previous pass audited, and its findings start the next pass. Passes repeat while the violations
left fall from one pass to the next, until the owner judges what is left small enough. A violation
left is a finding of a re-check that a reading confirms, that is on no kept list and that no issue
entry records. Which of them the count reads is `design@agent-skills@convergence-count-scope`. A
pass where that count does not fall goes to the owner as a question, with the counts, and starts no
pass on its own. The stop rule is the owner's, given in place of a fixed threshold: "As long as we
observe a "convergence", and the amount of leftover violations is small enough, I think that is
good. If you ask an LLM to find a flaw or defect in something, it will nearly always find one IMO.
Especialy if that something is prose and not code."

The rival, a lighter method with no re-check, lost on real audits, whose commits
`git log --grep='design-record axis'` finds: in the first audit of the design-record axis on this
repository, the re-check of each pass found violations the pass had missed or made.

### The convergence count of an audit reads only the violations left on text the previous pass also read `##convergence-count-scope`

A violation left is on old text when the previous pass also read its words and no commit after the
one that pass audited changed them; a move, a reflow or a rename leaves text old. Any other is on
new text, the previous pass's repairs among it. The stop rule of
`design@agent-skills@audit-passes-repeat-while-the-count-falls` reads the count on old text. A
violation left on new text takes its outcome like any other, and decides nothing about convergence.
Each axis's section says how its findings are classified.

The count reads old text because each pass's repairs write new text, and the agents find defects in
new text at least as readily as in old. On the fifth pass of the agentic-workflow axis on this
repository, 30 of its 38 findings sat on words the branch that built the axis had written, 14 of
them by the change that made a pass one run, classified by `git blame -w -C` on each defect's line
at the commit audited, with the diffs read by hand where blame and the words disagreed. The plan
document of that work holds the classification, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it. A count over the whole corpus then
compares two corpora, and measures how much text the last pass wrote more than how much defect is
left. The nearest rival, the plain count with its fall judged over its context, was argued by hand
at each firing, with the same explanation each time. Dropping the count for the owner's judgement
alone drops the convergence signal the stop rule reads.

### A pass of an axis is one run of its saved workflow, from its preflight to its report, with one more run to judge its application where the axis has one, and the session reads each report after its run, never between its steps `##one-run-pass`

One run dispatches every agent of the pass and delivers one report, and the session reads nothing
between its steps. The saved workflow is their one home, per
`design@agent-skills@audit-axis-runs-a-workflow`. For the agentic-workflow axis the steps are the
preflight, the auditors' drafts, the verify step of `design@agent-skills@verifier-in-the-run`, the
dedup into the confirmed findings, the cluster and the merge into causes, the repair proposals and
their checks, per `design@agent-skills@root-cause-repair`, the cross-check, and the report. For the
design-record axis they are the preflight, a setup agent, the auditors' drafts, the calibration of
`design@agent-skills@calibration-in-the-run`, the verify step, the checks of
`design@agent-skills@created-heads-checked-in-the-run`, the cross-check, and the report. Every step
writes its files in the run's scratch directory, so the session can open any draft, grouping or
proposal the report cites. The run sets itself up inside it: the listings the agents read, and a
worktree of its own for each agent dispatched alongside others, as
`skill@knowledge-architect-review@review-invariants` asks; an agent that runs after another may work
in its worktree, as a verifier works in its auditor's. The preflight creates them where the script
names the agents from its own table; where the agents follow from what the preflight lists, as the
design-record axis's groups do, a setup agent creates their worktrees once the script has cut them.

The cross-check and the report are one agent each, two in all, on the owner's word. The cross-check
maps the findings that edit one
paragraph or one slug, and proposes each finding's outcome for the session to decide; in the
design-record axis it also lists each tripwire and issue an edit would leave false, read whole. The
report quotes what the session needs to discuss each finding with the owner and act: the refuted
drafts with their reasons, the run's counts, each finding or cause with its quoted evidence and
verdicts, its edit before and after, its check and a proposed outcome, the overlap map, a draft of
the owner list, and what the run did not decide.

After the run the session verifies the report, as the axis's section of the audit skill says,
since the owner list goes out under its responsibility: the report whole, every finding the report
marks as plausible or as bearing on a ruling of the owner or on a tripwire, against its draft, and
a sample of the refuted drafts, with what the axis adds: every repair proposal in full for the
agentic-workflow axis; the calibration's readings, every check and a sample of the conforming drafts
for the design-record axis. The samples are the session's to choose, on the owner's word that
leaves the sample to judgement. The owner list, the owner's word and the application follow. The run ends at
its report, and the application is the session's, per
`design@agent-skills@apply-stays-with-the-session`. The design-record axis then runs its workflow
once more on the application, in the mode `application`, per
`design@agent-skills@application-judged-by-diff`: the owner's ruling and the session's
application sit between the two runs, so no single run can hold both.

The shape is the owner's: the main power of a saved workflow is to organize several complex steps
into one complete result, such as a report, as ready as possible for the main session to discuss
with the owner and then act on, with the least burden on the main session; and verification by the
session is necessary, but not in the middle of the workflow's steps.
The nearest rival, the steps run as separate runs with a reading by the session between each, lost
on the pass of the agentic-workflow axis that ran so: the session's sort refuted 3 drafts, each a
refutation a verifier given the audit's earlier commit messages can make, its reading of 23 repair
proposals sent none back, by the session's own account, which no file records, and the readings
cost the session most of its context. The rival that has the session read every draft again after
the run lost on the same measurement, since the verifier makes the refutations that sort made. The
plan document of that work holds the measurement, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it.

### The session applies the repairs the owner approves, and no saved workflow does `##apply-stays-with-the-session`

A run of an axis's saved workflow ends at its report. The session applies the edits the owner
approves, itself or through agents dispatched on disjoint files, and reads the whole diff before
each commit, as `skill@knowledge-architect-project-audit@audit-method` says.

The nearest rival, a second saved workflow that applies the approved repairs, lost on the merges
the application needs. In the two applications that agents made on the agentic-workflow axis on
this repository, the repair of passes 2 and 3 and that of pass 4, the applying agents merged edits
that several repairs made to one paragraph, six times, three in each, by their reports, which no
file records. A workflow would make those merges against a tree that may have moved since the run
read it. Two applications are the evidence, so the decision is held loosely.

### An audit axis's steps live in a saved workflow of its own, their one home for any harness, and its section of the skill keeps what the session needs to verify the run's report `##audit-axis-runs-a-workflow`

An axis's saved workflow holds its dispatch, the prompts of its steps and their order, and covers
that axis alone. It ships in the installer's namespace, and the harness runs it by the name its
`meta` declares, per `design@agent-skills@saved-workflow-name-is-its-file-stem`. The axis's section
of the audit skill names the workflow and the `args` it takes, and keeps what the session needs to
verify a report and write the owner list: the axis's corpus, its rules, what the session's
verification reads, and its outcome table. The axis's agent states the standard its steps judge by,
and no dispatch. A session whose harness does not list the workflow dispatches its steps by hand,
in the script's order, with the script's prompts, read from the script. The agentic-workflow axis
runs `knowledge-architect-agentic-workflow-audit`, and the design-record axis
`knowledge-architect-design-record-audit`.

The shape is the owner's: "ultimately, I think it should become the generic shape of every audit
axis, with every audit axis having its own dynamic workflow that produces a ready to discuss and act
report. That would also make the project audit skill much more concise." The delivery as a file of
the harness's directory is the owner's too: "I think it would be best to deliver the workflow as a
file directly in .claude/workflows/ (it would need to use the knowledge-architect prefix too, for
namespacing)."

The argument turns on the behaviour of Claude Code's Workflow tool, read in its description and its
authoring reference and measured by probes, which the message of the commit "The agentic-workflow
axis's script ships as a saved workflow in the installer's namespace" records. The tool runs only
on an explicit opt-in, such as the
owner invoking a skill whose instructions tell the session to call it, or asking for a saved
workflow by name. A script reads no file and takes its inputs in `args`. A saved workflow runs by
the name its `meta` declares, only from a file directly under the workflows directory, and the
harness lists it with the skills. So the order of the steps and every agent's inputs are fixed by
the script and versioned with it, and the session copies nothing to run it; a script in a fenced
block, passed to the tool inline, would sit in a skill as text no session needs to read. The rival
that keeps the steps in prose beside the script, or in one markdown file per axis for a session
without the tool, keeps two homes for one moment, and one drifted: a repair prompt of the script
narrowed what the skill's prose said, "the homes merged into one", to "the section of the skill
that owns the moment". That the checker reads markdown and no JavaScript bears on references, and
the script cites none. A file per axis stays available for reading guidance that is not a step.
The rival of one workflow auditing every axis at once lost on the owner's
words: "it audits all axes of the project at once and produces a single report, which is too large
of a scope".

No goal binds the audit to a harness: `grep -il 'harness\|provider\|claude code'` over every goals
home returns nothing, and `goal@agent-skills@installed-text-works-anywhere` names a project's
language and layout. A second provider stays a design target, per
`issue@core@configuration-for-several-agent-providers`, and the dispatch by hand keeps it open. The
rival of a skill written for the Workflow tool alone fails in every other harness, for a gain the
dispatch by hand already gives. The owner's weighing of the dependency: "After all, I use only
Claude Code personally... I'm not really expecting anyone to start using this, outside of myself in
more projects than thaum." The cost of shipping them: each saved workflow is listed in every
session of every project that installs the workflow, one that never audits included.

### Each audit pass commits a report file holding its evidence and the owner's rulings, on every axis, and the file leaves once its outcomes are carried out `##committed-audit-report`

The file is `docs/audit-reports/<YYYY-MM-DD>-<axis>-pass-<n>.md` at the project's root, the axis
named by its section's slug, per `skill@knowledge-architect-project-audit@record-of-an-audit`. The
session writes it after its reading of the pass: the run's report as its verification left it. The
owner list closes it, each item ending with a `Default` line
and an `Outcome` line, which the owner writes, or the session writes with the owner's words
verbatim. The file is committed with every `Outcome` line before any edit of the pass is applied.
The commit that carries out its last outcome, or a later commit of the same branch, deletes it, and
its message names it by its path. Each pass's commit message keeps the counts and one line per
outcome. The primer's knowledge table carries its row for every project, per
`design@agent-skills@knowledge-table-home`. A review of the audit's branch is told each report file
the branch added and deleted, with the commit that holds it, since a reviewer of the branch's tip
does not see a file the branch deleted.

The proposal is the owner's: "I think it might be a good thing for the audit report to be kept in
history, committed somewhere (maybe docs/audit-reports) and deleted once acted upon, just like retro
intake reports.", and so is its scope, every axis: "every axis for the report". A working file of the session is lost with its scratch directory, and the evidence of a pass, its
drafts and their quoted texts, was needed after the pass, to classify its findings again as on old
or new text. The nearest rival, the commit messages alone, keeps the outcomes and loses the
evidence, and a commit that records rulings and changes no file is dropped by GitHub's rebase merge, as
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file` measured. History grows
by the report's size at each pass, and the tree by nothing once the file leaves.

### Every finding of an audit takes exactly one outcome of its axis's table, and none is left to judgement `##audit-outcomes`

Every finding takes exactly one outcome, from the generic table of
`skill@knowledge-architect-project-audit@outcomes` as its axis's section fills it: applied in the
branch, put to the owner, sent upstream, an issue entry, or nothing. An edit that changes no
ruling of the owner is applied in the audit's branch: a rewording, a tense, a reference, a split
that moves no word of the owner, a rejected alternative repaired in place, a factual error, a
pointer whose target exists under another name, a drifted restatement brought back to its home, an
instruction read two ways reworded to the reading a design head or another instruction states. An
approval cited as a head's ground is removed in the branch where the head stands on its argument,
since its repair removes a false ground and widens nothing. A widening of what the owner's words
approved, the removal of an entry, a ground that is the owner's words, which side of a
contradiction wins where no design head, goal or source settles it, a verdict where a rule reads
two ways and no record states the reading, a
provable gap, a head in the wrong Component and a conflict with a goal go to the owner, per
`goal@knowledge-architect@the-owner-decides` and `design@agent-skills@questions-go-to-their-authority`; any finding no case names goes there too. In the
agentic-workflow axis, an instruction read two ways goes to the owner where nothing recorded states
the reading meant, and is applied where a design head or another instruction states it. A predicted gap, per `design@agent-skills@behaviour-additions-need-real-use`, becomes an issue on the project's own text and is sent upstream on the installed text. An unneeded statement, per
`design@agent-skills@need-test`, takes the outcome of the table of
`skill@knowledge-architect-agent-configuration@admission-standard`: deleted in the branch where it
carries no instruction, moved to a `%%` comment of the installed source where only its maintainer
reads it, and put to the owner where it is an instruction no agent performs; a review applies the
same table, so the two judge one statement alike. Work too large for the branch and a finding the owner
defers each become an issue. A finding outside the axis takes the first case of
`primer@met-outside-the-task` that applies. A finding that an issue entry already records, or that
is on the kept list, takes no action.

A finding on the installed text is never edited in a project that installs it, per
`skill@knowledge-architect-agent-configuration@installed-files-never-edited`. It is sent upstream,
in one file per audit, written when the audit stops from the report files of its passes, in the
shape of the file in which a retrospective sends the workflow's findings, so it reaches the project
that publishes the workflow by the same route, per `design@agent-skills@retrospective-destination`.
A finding of the interaction of the installed text and the project's own is split into its side on
each text, each side with one outcome, so a finding still takes exactly one. The installed text
holds no branch for the project that publishes the workflow, which edits the installed text at its
source by its own instructions.

No outcome leaves a confirmed finding to judgement.
`goal@knowledge-architect@agents-get-a-complete-workflow` asks that "Each instruction says what it
asks plainly enough to follow without guessing what it means", so an instruction that a reading
confirms is read two ways is repaired, even where every act it allows is acceptable. What no
instruction covers is left to judgement on purpose, and is no finding. The rival outcome, a finding
of two readings left to judgement where no text rules either act out, leaves such an instruction
standing against that goal. Its cost stands: each repair writes new text, which
`design@agent-skills@convergence-count-scope` counts apart and `design@agent-skills@repair-proposals-audited`
reads before the repair lands.

Most findings of the design-record axis are text edits, so they are applied at once: the owner
thinks the goal for that axis is usually immediate application, with little reason to delay since
those are mostly text edits, unless some design entries have major issues that bear on the
project's goals, for example.

### The design-record axis re-applies to every entry the rules on design heads, on the Component a head sits in and on rejected alternatives, reading the tripwires and issues that bear on each head and judging neither them, the restatements nor the code `##design-record-axis`

Its corpus is every design head and every rejected alternative of every Component. Its rules are
`primer@design-heads` for a head, the questions of
`skill@knowledge-architect-decision-recording@owning-component` for the Component it sits in, and
`skill@knowledge-architect-decision-recording@losing-alternatives` for a rejected alternative. Each
head is read with the tripwires guarding it, the issues naming it and the history behind each
citation of the owner. The owner narrowed the axis to that reading: "Reading the tripwires and
issues related to design is important IMO, so that the audit on design takes them into account."
The audit repairs a tripwire or an issue only where its own edit makes it stale: the conformance of
a tripwire or an issue to its own register's rules is the axis of that register, and a judgement of
it here would read entries the decision record does not hold, while a stale one the audit's edit
leaves is a defect of the audit itself. The restatements of a head in other texts left the axis on
the owner's word: "It does not read the same thing at all." The truth of a head about the code is
another axis too, since it reads the code rather than the decision record. A head the axis meets
false of the code gets an issue. The axis is carried by
`agent@knowledge-architect-design-record-auditor`.

The Component a head sits in is judged because the axis's outcomes put a head in the wrong Component
on the owner list, and no other step reads it for a head no branch touches: the routing reviewer,
`agent@knowledge-architect-routing-reviewer@routing-predicates`, judges it only for a head a diff
writes. The rival, that outcome removed, leaves a misfiled head unread until a change touches it.

### The design-record axis is dispatched in bounded groups calibrated on a shared sample `##design-record-axis-dispatch`

Its saved workflow's preflight lists every entry of every Component, in the order the auditor's
section `agent@knowledge-architect-design-record-auditor@take-your-group` gives, and the script
cuts the groups. A Component of more than the group size, 60 by default, is cut alone into groups
of consecutive entries whose sizes differ by at most one; the smaller ones are packed whole, each
into the first group it fits in. One auditor per group writes one draft per entry, its brief
listing its entries one by one, and the script logs every listed entry its return omits. A shared
sample of about five entries from different Components is added to every group, beyond its bound,
and compared as `design@agent-skills@calibration-in-the-run` says.

The listing is the script's because a script reads no file and needs the counts to dispatch, and
because auditors that each derive their group from an order stated in prose can leave an entry
unjudged or judged twice with nothing to notice it; the listed entries make coverage a check of the
script. A group never splits a small Component, since a rejected alternative is judged against the
head it lost to; packing whole Components keeps that, where one group per Component spends an agent
on a handful of entries, and a group that mixes Components is no harder to judge by any record. The
bound of 60 is the one `design@agent-skills@standing-entry-search-groups` argues for a standing
entry; an auditor reads more per head, its tripwires, its issues and its history, and no
measurement shows the bound right for that load, though the passes of the axis's audits on this
repository ran with it. Bounded groups keep each agent's reading whole on a large project. The
sample exists because agents dispatched together do not read a rule alike: in the axis's first
audit on this repository, two agents read one shared sample entry two ways in two of the samples,
and a method that trusted one reading per entry would have left both. The audit's commits, found
with `git log --grep='design-record axis'`, carry the counts.

### The design-record axis compares its calibration sample inside the run, an auditor proposing each reading and the session confirming it `##calibration-in-the-run`

Once every group has returned, one design-record auditor under its calibration brief compares the
verdicts the groups gave on the sample. Where they differ and one follows the rule's text, it
proposes that reading, with the rule's sentence; where the text admits both, the difference goes on
the owner list. Each verifier then judges again, under a proposed reading, its group's drafts the
reading bears on. The session confirms each reading at its verification; a reading it rejects is
corrected in the calibration file, and the run starts again from its verify step. The rerun is the
owner's choice: "With a rerun, as you proposed." The calibration is the auditor's, since judging
which verdict follows a rule's text is judging against the axis's standard, per
`design@agent-skills@audit-agent-per-axis`.

The nearest rival, the session settling each reading and reading again the drafts it bears on,
puts a reading across every group back into the session, against
`design@agent-skills@one-run-pass`. The rival that starts each verifier as its auditor returns and
calibrates after them needs one agent to read again, across every group, the drafts a settled
reading bears on, the load the group bound exists to avoid; the barrier costs the time of the
slowest auditor. A verifier sees one group only, so it cannot replace the sample: the sample's
readings differ between groups, not inside one.

### Each head a proposed split, merge, rename, retitle or move would create is judged alone inside the run, before the owner rules `##created-heads-checked-in-the-run`

In the design-record axis's run, for each group whose verified findings propose a split, a merge, a
rename, a retitle or a move, a design-record auditor under its check brief judges each head the edit
would create, or the head under its new title, alone against every rule of `primer@design-heads`,
and the edit against the auditor's failure modes of an edit. One checker per group is the default,
and the count is the session's, passed to the run.

A head created by a split failed a rule read alone in the axis's first audit on this repository,
and moving text unchanged does not make the new head conform. The rival, that judgement made only
at the application, comes after the owner's ruling, so the owner would rule on a head nobody had
judged alone. One checker per group reads the rules once for every created head of the group, where
one per proposal reads them once per head. A retitle is judged with them, since the owner rules on
its new title as on a created head.

### Every entry the application of a design-record pass changes is judged after the application, in a second run of the axis's saved workflow `##application-judged-by-diff`

After the owner's ruling, the session commits its application of a pass of the design-record axis
and runs the axis's saved workflow on it, in its mode `application`: one design-record auditor per
group, under an application brief, judges every head and rejected alternative whose text the
application changed, a retitle and a body edit included and a re-pointed reference alone left out,
against every rule, its change against the failure modes of an edit and the owner's ruling it
carries out, and each tripwire and issue that cites a changed head, clause by clause, against the
head that now holds what each clause guards. The session repairs what a reading confirms of the
report and folds the repairs into the pass's commit.

The application writes text the run never read: the edits of the owner's rulings, a check's repair,
and every entry a split or a retitle touches. In the first pass of the axis in thaum, on
knowledge-architect 0.6.0, the application touched about 200 heads and entries; four auditors
judging every changed entry found 19 failures the run's checks could not see, among them a retitle
that made a title true of a recorded rival and an attribution wider than the owner's argument, and
the standing-state review found two tripwires the application left false while they still resolved.
The rivals lost. The session judging alone does not read about 200 entries against every rule
within one context. The re-check of the audit method runs only when a pass follows, and not when
the owner stops the audit. The standing-state review before the merge reads the tripwires and the
issues against the change, not each changed entry against every rule. A dispatch by hand from the
skill puts a step outside the saved workflow, against
`design@agent-skills@audit-axis-runs-a-workflow`. The cross-check's reading of tripwires runs
before the application, and can only predict what the edits will do. The run applies nothing,
so `design@agent-skills@apply-stays-with-the-session` holds.

### The agentic-workflow axis reads every text the harness delivers to a session as an instruction, and neither the scoped CLAUDE.md files nor the registers are in its corpus `##workflow-axis-corpus`

The corpus is the project's root `CLAUDE.md` and the primer it imports, and every other text the
harness delivers to a session as an instruction from the project's agent configuration directory,
installed or the project's own. `agent@knowledge-architect-workflow-auditor@the-corpus` defines it,
and the axis's section and the saved workflow point there. The definition names no provider's
directory, so it holds for a second harness, per
`issue@core@configuration-for-several-agent-providers`. The registers are records, not
instructions: an auditor reads the issues listing so that a finding already recorded is marked with
its reference rather than reported again. An auditor may run read-only commands to check what the
corpus claims about the checker or the harness.

The corpus is the owner's: the root `CLAUDE.md` and the content of the agent configuration
directory. So is the exclusion of the scoped `CLAUDE.md` files next to the code, which the owner judged more like developer documentation than instructions to follow. `issue@core@a-home-for-developer-contracts-outside-agent-configuration`
points the same way. The cost is that no audit reads a scoped `CLAUDE.md`: the need test of
`design@agent-skills@need-test` reaches one through its writer and the review axis on agent-facing
text only. Every auditor
reads the whole corpus, per `design@agent-skills@workflow-axis-lenses`, which rests on one agent
being able to read it whole: `git ls-files CLAUDE.md .claude | grep -v open-issues | xargs wc -w`
measures it, and an auditor's return that says it did not read every file whole reopens how the
corpus is shared out.

### Every auditor of the agentic-workflow axis reads the whole corpus through one lens, each lens dispatched at least once, with no calibration sample `##workflow-axis-lenses`

A lens is one task: what an auditor looks for. The lenses built are L1 contradiction, L2 activity
walk, L3 two readings, L4 provable gap, L5 restatement, L6 local against installed, L7 checker rules,
L8 verdict alignment and L9 need test, and `agent@knowledge-architect-workflow-auditor@the-lenses` is the one
home of what each looks for. Each lens is dispatched at least once, and the count per lens is the
session's, passed to the run as `counts`. The saved workflow's lens table holds each lens's default
count and L2's groups of activities, so the session writes no entry per agent. An L2 walker takes
one group of activities, and two walkers are duplicates only when they share one. A finding two
agents report is merged. There is no calibration sample: the agents give no verdict per entry, the
duplicates of a lens add coverage, a verifier and the session read every finding, and the re-check
reads again.

The shape is the owner's: the owner judged that the workflow functions as a whole, that none of
its files can be judged alone, and that a main aim is finding contradictions across files, and
preferred agents with varied tasks, each given the whole corpus, duplicates included where there is
room, since a duplicate may find more. The nearest rival, the corpus shared out among
agents of one task as the design-record axis shares its own, loses on that argument: a
contradiction across two files sent to different agents is found by neither. Cutting a lens's count
or narrowing its scope lost on the fifth pass of the axis on this repository: the verifiers and the
session confirmed 37 of its 38 findings, each lens with more than one agent produced findings no
other agent of that lens reported, and 14 of the 15 findings of two readings, L3's class, were
confirmed, the session rejecting one, while 9 of the 15 sat on text written for the axis itself. The plan document of that work holds the
measurement, and `git log --diff-filter=D --name-only -- docs/plans/` finds it. The count per lens
rather than a total budget is the owner's preference, stated with a hedge, so that the script
rather than the session's judgement dispatches the agents. The table lives in the script, since the agents' entries written by
hand were a run's largest input and the source of its slips, which the owner judged error
prone. A fixed count would be a structure another instruction needs to vary, against
`goal@agent-skills@installed-text-leaves-room-to-judge`.

Two lenses read beside the corpus. L7 starts from the checker's user documentation, its README for
the version the project uses and the help of each command, and reports a rule the corpus states
wrongly, and a rule on what an agent writes that no installed text states, when its finding message
does not name its repair. It judges the installed text alone, since a project text restating the
checker's rules would hide a gap the installed text leaves every adopting project. The owner raised
it, judging that agents cannot work efficiently without knowing what the checker refuses and allows
them to write.
Its rival, the README given to L4, loses: L4 starts from a requirement the corpus states, and a rule
no text mentions is invisible from there. A rule whose message names its repair costs one check and
one repair when it is broken, while a rule that shapes writing from the start, such as the
reference grammar, must be known before writing. L8 reads each pair of a review standard and an
audit standard that judge one subject, side by side, for a text of the project that one reports as
a finding and the other must not, the class of divergent verdicts. The owner proposed it, holding it quite important that the reviews and the audit axis agree on what they count as
findings. Two standards that no single session obeys together are no contradiction, so no other
lens drafts such a pair, and none of the drafts of the axis's first four passes compared what two
standards count as a finding. The rival,
the class with no lens of its own, loses on that.

L5 reads each restatement of a directive against its home, since the checker cannot compare
prose. The restatements are in this axis on the owner's word, given when they were proposed for the
design-record axis: the owner judged that they read another thing than that axis does, and placed
them, as the owner's opinion, in the agentic-workflow axis.

L9 reads each statement of the corpus against the need test of `design@agent-skills@need-test`, so
that the audit and the review axis on agent-facing text judge one statement by one standard, per
`design@agent-skills@admission-rules-home`. A lens of its own, rather than a widened L5, is the owner's choice; L5 judges a restatement
against its home, and L9 whether a statement is needed at all.
Its two auditors by default rest on the premise above, that a second agent of one lens finds what
the others missed.

### The agentic-workflow axis admits a finding only when a reading confirms it, and a gap that only predicts what an agent would do leads to no edit `##workflow-axis-finding-standard`

A finding is admitted in a class whose evidence a reading checks: a contradiction, two instructions
quoted and the moment both apply; a broken trigger, the instruction and the search that shows its
target absent; a factual error, the claim and the command that shows it false; two readings, the
instruction and the two acts its words allow; a provable gap, a requirement and an input to it that
no text supplies and judgement cannot derive, since it is a fact fixed elsewhere, such as a format
the checker enforces, a name or a path, not a choice an agent could make; divergent verdicts, one text of the project and the sentence of each standard that judges it; a
restatement or a pointer its home does not allow; and an unneeded statement, the statement and the
act tried, per `design@agent-skills@need-test`. A predicted gap, "an agent might misjudge X", leads to no edit:
on the project's own text it is an issue that states what a real session would have to show, and
on the installed text it is sent upstream, per `design@agent-skills@audit-outcomes`. A case no instruction covers, where judgement can
decide, is no finding, per `primer@room-to-judge`. A finding about how the owner works is outside
the axis: the retrospective judges it against an expectation set.
`agent@knowledge-architect-workflow-auditor@what-a-finding-is` is the one home of the classes.

The standard derives from `design@agent-skills@behaviour-additions-need-real-use`: a defect provable by
reading is repaired on reading, and an instruction about how an agent behaves is added only on
evidence from real use. A divergent verdict needs a text of the project, since a written example is
a synthetic scenario. The owner set the axis's aim: most of its work is finding what cannot be followed, and the gaps
where judgement is not enough, those gaps following the principle that some things are left to the
agent's judgement, since no text covers every case, and that instructions are best kept to what the
workflow needs to function as intended. The class of an unneeded statement is in the axis because
the owner ruled that all work that judges or edits agent-facing text follows one standard, per
`design@agent-skills@admission-rules-home`.
The nearest rival, every gap a finding to fill, fails
`goal@agent-skills@installed-text-leaves-room-to-judge`.

A confirmed finding of two readings is repaired even where both acts it allows are acceptable, as
`design@agent-skills@audit-outcomes` says. The rival that left it to judgement where no text rules
either act out would have left 6 of the 14 findings of two readings the session kept at the fifth
pass, by a reading of each against the corpus, the design homes and the goals at the commit that
pass audited. The plan document of that work, the deleted spec agentic-workflow-axis, holds the
measurement, and `git log --diff-filter=D --name-only -- docs/plans/` finds it. The findings of a
reading are worth their repairs: of the first pass, the owner observed that several findings were likely the
source of a misbehaviour the owner had seen.

### One installed agent carries the agentic-workflow axis's standard, its brief choosing its lens, or its role as a verifier or a checker `##workflow-auditor-agent`

`agent@knowledge-architect-workflow-auditor` holds the standard every auditor of the axis follows:
it edits nothing and writes only in its scratch directory, it copies every quotation from the file
and lists only commands it ran, it reads the corpus whole, and it holds the finding classes, the
tags, one draft per finding and its return. Its definition has one section per lens, and the brief
names which is its task; a finding of another lens met on the way is drafted too, under the
finding's lens. A verify brief makes it the verifier of another auditor's drafts, per
`design@agent-skills@verifier-in-the-run`, and a check brief makes it the checker of one repair
proposal, per `design@agent-skills@repair-proposals-audited`. So the standard reaches every agent
that judges a finding of the axis from its definition's first token, per
`design@agent-skills@audit-agent-per-axis`. The steps whose agents judge no finding against the
standard, the dedup, the cluster, the merge, the repairs, the cross-check and the report, run
general-purpose agents whose prompts are in the saved workflow. No agent of the run decides an
outcome: the cross-check proposes one, and the session decides it.

The nearest rival, one agent per lens, would repeat the standard shared by every lens, which is most
of the definition, by `wc -l` over the definition and over its section on the lenses. The owner
weighed the count of published agents: "I was worried about publishing too many agents and
polluting the directory. I think this shape works." The rival of the verify brief, a verifier agent
of its own, would carry the axis's standard outside the axis's agent, the shape
`design@agent-skills@audit-agent-per-axis` rejects in one generic auditor.

### One adversarial verifier per auditor tries to refute that auditor's drafts inside the run `##verifier-in-the-run`

The verifier is the axis's own auditor under a verify brief, per
`design@agent-skills@audit-agent-per-axis`, in its auditor's worktree. It checks every quotation
against the tree, a quotation or an attribution of the owner against the question it answered, and that what a draft
describes exists. It marks known a draft whose finding an issue entry records, and kept a draft on
the kept list. It marks a finding that a ruling's premise has proved false as bearing on that
ruling, and leaves it to the session. Its verdict per draft is verified, refuted or plausible, with
a reason, and a refuted draft is kept with its reason, for the session's sample. It reads the kept
list, the issues listing and the record of the audit's earlier passes, their commit messages and
their report files, so it can make the refutations that rest on the audit's history.

In the agentic-workflow axis, each auditor's drafts go to its verifier as soon as the auditor
returns, and the verifier splits a finding of the interaction of the installed text and the
project's own into its two sides, per `design@agent-skills@workflow-auditor-agent`. In the
design-record axis, the verifiers start after the calibration, which needs every group's drafts,
and each judges again, under a reading the calibration proposes, its group's drafts that reading
bears on, per `design@agent-skills@calibration-in-the-run`. A design-record verifier judges only
the draft of record of a sample entry, its own group's; it reads a sample of the drafts that find
their entry conforming, at least one, and where one is wrong it reads again that auditor's other
drafts on the same rule; and on a re-check it classifies each finding as on old or new text.

One verifier per auditor reads the corpus once per auditor and sees that auditor's drafts together,
so the drafts that share a wrong draft's class are read by the same agent, against the same
corpus, in one verify step. The nearest rival, one verifier per draft, reads the whole corpus once
per draft: at the fourth pass of the agentic-workflow axis on this repository, 63 drafts against 16
auditors. The plan document of that work holds the measurement, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it. Nothing a verifier passes is applied
before the session's verification, per `design@agent-skills@one-run-pass`.

### A verifier of the agentic-workflow axis marks a finding that carries a tripwire's firing evidence, after reading the tripwire whole, and leaves the firing to the session `##tripwires-read-by-the-audit`

The run's preflight saves the project's tripwires listing, and each verify brief names it. A
verifier reads whole, with the checker's `show`, each tripwire whose guarded decision or subject the
draft's finding touches, and marks a finding that carries its firing evidence, naming the tripwire.
The cross-check and the report list the marked findings beside those bearing on a ruling, and the
session judges whether each tripwire fired; a fired tripwire moves as
`skill@knowledge-architect-issue-tracking@movement-instruction` says. The corpus excludes the
registers, so without the mark no step of the axis reads a tripwire, and a tripwire whose evidence
the audit holds has no reader. The nearest rival, a search of the standing entries at each pass,
looks for the entries one piece of work bears on; an audit's work is the whole workflow, so the
search would return most of the register of the installed text, while a verifier holds one finding
at a time, the narrow question a tripwire's firing needs.

### A finding of agent-facing text is repaired at the cause that produced it, not at its site `##root-cause-repair`

A cause is one of four kinds: two homes for one moment, instructions about one moment or one kind of
act stated in more than one text; a structure the work did not need, a fixed sequence, count or
mapping that another instruction needs to vary; a restatement that drifted from its home; and a rule
written for one interaction, a narrow rule where judgement already decides. A finding that fits none
is a cause of its own. Each kind has its repair: the homes merged into one and the others pointing
to it, the structure removed, the restatement brought back to a pointer, the narrow rule deleted and
the case left to judgement. A repair adds an instruction only where none of these closes the cause,
and says why. One repair is proposed per cause, never one per finding, and each finding takes the
outcome of its cause's repair. Outside the project that publishes the workflow, a cause whose
findings are on both sides, the installed text and the project's own, is repaired in two parts: its
findings on the installed text are sent upstream, per `design@agent-skills@audit-outcomes`, and
those on the project's own text take the outcome of its repair's edits on that text.
`skill@knowledge-architect-agent-configuration@admission-standard` is the one home of the standard,
per `design@agent-skills@admission-rules-home`, and the audit and each text that names it read it
there.

In the agentic-workflow axis a second phase of agents delivers it inside the run: cluster agents,
two by default, group the confirmed findings by cause independently; a merge agent writes one
causes file from their groupings and states each choice it made where they differ; and repair
agents, one per cause by default, propose its repair, which
`design@agent-skills@repair-proposals-audited` and `design@agent-skills@repairs-read-against-the-record`
check before the session reads it. The counts are the session's, passed to the run. These agents
judge no finding against the axis's standard, so they are general-purpose agents whose prompts are
in the saved workflow, not briefs of the workflow auditor, since an agent's definition is the copy
the harness registered when the session started. The rival, the workflow auditor gaining two more
briefs, one to cluster and one to repair, loses on that.

The design-record axis's run has no cause step: an edit of a head adds no instruction, so two
edits interact only where they touch one paragraph or one slug, which its cross-check maps, per
`design@agent-skills@one-run-pass`. The rival, the cluster and repair steps of the
agentic-workflow axis, would group by cause findings the owner list already groups by kind.

The direction is the owner's. The owner judged that instructions added for corner cases are always more likely to interact
incompatibly by their number alone, as `design@agent-skills@capabilities-not-structure`
records, that repairing contradictions one at a time at each site is not viable, and that a more
thorough method of repair might be needed. One home per moment is the owner's too: the owner asked to try to centralize the instructions about one moment or one kind of act, as the rules on design
heads were. So is the second phase: the owner asked that the axis dispatch a second set of agents to
classify the findings and propose their repairs. `design@agent-skills@capabilities-not-structure`
states the principle the repair applies: a conflict a structure causes is answered by removing
structure rather than by adding a rule. The nearest rival, one repair per finding at its site, lost
on the axis's first audit on this repository: the repairs of its first two passes turned most findings
into added rules, and its third pass found about 25 findings in the interactions of those rules, by
the auditors' returns, which the message of the commit "[audit] Pass 3 of the agentic-workflow axis
stops the run: the count did not fall, the repairs move to a method by cause" records. The rival of
the second phase, the session grouping the findings alone, lost on the same audit: the session's
sort dropped a finding and left a ruling of the owner off the next pass's kept list.

### A set of repairs that adds more instructions than it removes goes to the owner first, with its net count, and the word count is recorded beside it `##audit-net-growth`

An instruction is a sentence that tells an agent to do, or not to do, an act at a moment; a
rewording that makes an existing instruction's intent clearer adds none, whatever its length. Each
repair proposal counts the instructions it adds and removes. When the repairs to be applied together
add more than they remove, the owner is told so first, with the net figure, and rules on that growth
before any of them is applied, as `skill@knowledge-architect-agent-configuration@admission-standard` says.
Each pass of the agentic-workflow axis also gives the corpus's word count before and after in its
commit message, for monitoring.

The metric is the owner's: "I don't think word count is the correct metric here ... It should
probably apply to "more instructions" rather than more words. Though monitoring net word count
change produced by this process is still interesting, and should at least be done and recorded in
commit messages of audit repairs." A bound on words loses on the owner's argument: "Repairing a
sentence to make intent clearer may need adding words locally, and this should not need my ruling.
The net growth is hard to account for before the fact, too." An instruction is countable from a
proposal before the edit. No bound at all loses on the axis's first passes, whose repairs turned
most findings into added rules that the next pass found interacting, per
`design@agent-skills@root-cause-repair`.

### A step of the run reads each repair proposal's new text for contradictions and two readings before the session reads it `##repair-proposals-audited`

In the agentic-workflow axis's run a check step follows the repair proposals: one workflow auditor
per proposal, under its check brief, in a worktree of its own, each starting when its proposal is
written. It reads the proposal's text after its edits, in the corpus, through L1 and L3, for a
contradiction or two readings that the new text creates with the rest, and writes its check beside
the proposal. The cross-check and the report read the checks, and the report gives each beside its
proposal. The run does not revise a proposal: the session's verification corrects it, or sends it
back to a repair agent of its cause briefed with the script's repair prompt and with the check, so
a repair is checked before it lands.

Each pass's repairs write new text, and the next pass finds defects in it, per
`design@agent-skills@convergence-count-scope`: of the fifth pass's 38 findings, 16 sat on text that the
repairs of the earlier passes and the commits of the method by cause wrote. The nearest rival, those
defects left for the next pass to find, costs a full pass to surface each. The rival of the session
reading every proposal's new text through L1 and L3 moves those readings into the session, against
the burden `design@agent-skills@one-run-pass` keeps from it. The step adds one agent per proposal
to a pass.

### Each repair of agent-facing text is read against the goals, the design heads and the rejected alternatives it touches before the owner rules on it `##repairs-read-against-the-record`

The record check reads every goals home, the design heads that the checker's `show` and a search of
the design homes return for the behaviour the edited text describes, the rejected alternatives
of each Component that owns an edited text, and the scoped `CLAUDE.md` files and code comments that
`skill@knowledge-architect-decision-recording@reversal-check` searches. It names each head the repair would reverse, each rejected
alternative it brings back and each goal it strains, with its quotation. A repair that reverses a
head is a decision, recorded under `skill@knowledge-architect-decision-recording`.
`skill@knowledge-architect-agent-configuration@admission-standard` states it, for every actor that
writes or judges agent-facing text, per `design@agent-skills@admission-rules-home`. In the agentic-workflow axis's run the check of
`design@agent-skills@repair-proposals-audited` makes it, reading those homes beside the corpus,
which they stay outside, and every repair prompt names the nearest rival to its repair and the fact
that decides between them.

The corpus excludes the registers, so without the check no step of the run reads a design head, a
goal or a rejected alternative, and a repair can bring back an alternative that already lost. This
repository's intake of received findings read the same record for its fixes, and the owner asked
whether the audit could learn from what the intake did more thoroughly. The nearest rival, the check
left to the session at the application, when the edit is written, comes after the owner's ruling, so the owner would rule on a repair without knowing that it reverses
a head, against `goal@knowledge-architect@the-owner-decides`. The search by behaviour is a
judgement, and a head it misses is found at the review of the audit's branch, which reads each
repair against the design heads.

### An audit runs on an occasion whose own work needs the record read under the rules, never on a period `##audit-triggers`

The owner's request is the main occasion. A version that changes the rules recorded content must
meet cites the axis that brings a project's record to them in its Migration entry, per
`design@knowledge-architect@changelog-entries`, and `skill@knowledge-architect-setup@moving-the-pin`
runs it after the pin's commit; `skill@knowledge-architect-setup@existing-documentation` runs the
design-record axis at the end of the work that moves a project's documents into the homes. Each
occasion is one whose own work needs the record read under the rules. The rival, a periodic audit,
has no evidence for a period, and `design@agent-skills@behaviour-additions-need-real-use` asks for one.

### Each audit axis has an installed agent of its own, which carries the axis's standard `##audit-agent-per-axis`

The agent's definition reaches a subagent from its first token, so the axis's standard does not
depend on what a brief asks it to read, and a second axis adds an agent rather than a branch inside
one text. Its read-only rule is in its body, since its tools include a shell. The rival, one
generic auditor briefed to read the axis's section of the skill, puts the standard behind a read the
brief must ask for.

### A project audit repairs the whole record at once, on each occasion an audit runs `##audit-repairs-the-whole-record`

Beside the repair of a head a change touches, `design@agent-skills@existing-heads-on-touch`, a
project audit brings every head and every rejected alternative to the rules at once, under
`design@agent-skills@audit-method`, its widenings going to the owner on one list, on the occasions
of `design@agent-skills@audit-triggers`. Its premise is the owner's: "With the changes we just did
to decision record conditions and shapes, at the next version pin, projects migrating will need to
migrate their design home to match them." A repair on touch never reaches a head no change touches.
The rival, the repair on touch alone, leaves a project's record under the old rules after its
migration.

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
skill delivers when it loads. The rules on what a design head records and how pass that test, since
every reading of a head needs them and no skill loads at that moment, per
`design@agent-skills@one-home-for-head-rules`; the procedures of writing one stay in the
decision-recording skill. The workflow targets frontier-tier models, which the
design skill's work requires, per `design@agent-skills@frontier-tier-only`, so a size limit
would protect a reader the workflow does not serve.

### The primer carries the directives that every session applies and that no skill delivers at the moment they apply `##primer-content`

Besides the knowledge table and the list of installed skills, the primer carries the rule on when
to write a reference, the section on design heads, per
`design@agent-skills@one-home-for-head-rules`, and the directives the skills rely on and no skill
delivers at the moment they apply. The directives it carries today follow. **A reference is written wherever the text would have to be revisited if the entry it
names changed.** That rule is the premise the workflow relies on to be useful: an entry of a
register is referenced wherever it is load-bearing, so the checker can list what a reversal, a
closure or a firing touches, and the scope of reversing a decision can be assessed from that list,
per `goal@knowledge-architect@structure-and-workflow-work-together`.

- **The goals are the only statements assumed to come from the owner.** A recorded decision was
  reviewed, but its review can miss a detail or an implication, more often as the volume of agentic
  work grows. When a decision conflicts with a goal, the likely cause is that the owner missed the
  conflict: the goal prevails, and the conflict goes to the owner, per
  `goal@knowledge-architect@the-owner-decides`.
- **A question goes to the authority that answers it**, per
  `design@agent-skills@questions-go-to-their-authority`: every session meets questions, and a skill
  that routes them reaches only the sessions that load it.
- **A word of the owner holds only as far as its premise, and an answer on a reading gives way to
  its source**, per `design@agent-skills@owner-word-holds-as-far-as-its-premise`.
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
- **Something met outside the task takes the first of four outcomes that applies**, per
  `design@agent-skills@met-outside-the-task-outcomes`.
- **A turn that decided something the owner could have decided ends with a "Decided" section**, per
  `design@agent-skills@decisions-reported-to-the-owner`.
- **An item put to the owner under a label takes the next number of its prefix in the record the
  label reaches**, per `design@agent-skills@ruled-items-labelled`, and an answer keeps the
  authority that answered it, per `design@agent-skills@answer-keeps-its-authority`. Labels are given
  and answers recorded in sessions of every activity, and no one skill is loaded in all of them.
- **Every commit of a branch is judged by the checker at the branch's tip**, where the project's
  gates run the commits check over the branch, so a change that sets what that checker requires
  goes in the branch's first commit, and a repair that would leave an earlier commit failing is
  folded into it. Every session commits, and a skill meets only one case of it: planning a plan
  document merged before its work, and review a fold.

### A question is answered by its authority, and the owner answers only what no authority settles `##questions-go-to-their-authority`

`primer@question-authority` routes each question to what answers it: what a specification the
project implements says, to its text; what the code or a tool does, to a measurement; what follows
from the goals and the recorded decisions, to the argument from them. The session answers those
itself, cites the source and tells the owner. The owner answers a goal to set or change, a fact
only the owner holds, such as an intent, a plan or a weighing the owner made, a contradiction no
argument resolves, an action that cannot be undone, and a fork: two or more shapes
that meet every binding constraint with no fact defeating either. A cost alone makes no fork, since
a shape that fails a goal or loses to a fact is out whatever it would save. The section prevails
over an installed instruction that sends to the owner a question it gives the session, as a goal
prevails over a decision. Which further actions a project reserves to its owner is the project's
to state.

It serves `goal@knowledge-architect@the-owner-decides`, which gives the owner what the goals and
the record leave open, and `goal@knowledge-architect@agents-work-without-drift`, which asks that the
owner not answer what the record already answers. A workflow whose only authority is the owner
sends every question to the owner, and records every answer as a ruling. In thaum, which implements a published rules text, at least three of
the eleven questions one session put to the owner were answered by the rules, and in this
repository the owner reported many defaults put back to the owner that, the owner believes, the
recorded design heads should answer, and which appear to have been proposed without reading those
heads; the owner thinks the workflow sends such a question to the owner before it looks at the
heads. The nearest rival, a sentence that gives an external specification alone a rank above
the owner's word, loses on that second instance: the mechanism runs in a project that implements
no specification. A second rival, sending to the owner every choice that weighs a cost, sends every
choice, since nearly every choice has a cost or can be made to have one. The rank sentence holds because an edit of a skill can bring
back a routing to the owner, and with it no session meets two instructions it cannot both obey. The
owner thought that a list keeping only goals, contradictions and irreversible actions with the
owner left too large a scope to agents. The fork is the fusion of that list with one that kept scope and weighing with the
owner too, and keeps with the owner only the weighing no argument settles. A merge is no action that
cannot be undone, since a revert commit undoes it, and whether agents merge is left to each project,
as the owner directed. A fact only the owner holds is among the kinds because the primer asks the
owner such facts, as at the fourth entry test of
`design@agent-skills@a-head-is-owed-by-an-entry-test`, and a ruling on intent is defined by these
kinds.

### An answer of the owner on a reading or a fact is recorded with its source, never as the owner's ruling `##answer-keeps-its-authority`

`primer@labelled-items` states it beside the label rule of
`design@agent-skills@ruled-items-labelled`: an answer of the owner on a question that a source, a
measurement or the record answers is recorded as a reading or a fact, with its source. Where the
source leaves the point open and the owner chose, the record says the owner chose among the
readings the source permits.

It serves `goal@knowledge-architect@the-owner-decides`, under which a decision the owner made is
recorded as the owner's and every other decision on its own ground. An answer recorded as a ruling
weighs on every later session as the owner's intent: the owner observed that questioning such
answers is discouraged, or that they come back to the owner even when the answer is obvious and the
owner may not hold it. The nearest rival, leaving
the recording rules as they are since fewer such questions reach the owner under
`design@agent-skills@questions-go-to-their-authority`, loses because a question still reaches the
owner by mistake: in thaum, one went to the owner through a definition the session had not looked
up, and its answer became "the owner's ruling" in every later text.

### A word of the owner holds only as far as its premise, and an answer on a reading gives way to its source `##owner-word-holds-as-far-as-its-premise`

`primer@owner-word-premise` holds three rules. A ruling does not rule on the case where its
premise is false, and a premise the session supplied is the session's to check before the owner
rules on it. An answer of the owner on a reading or a fact that the source or a measurement
contradicts gives way to the source at once, the owner told, and a part of the work that cannot be
undone waits. A ruling on intent, a word on a question `primer@question-authority` reserves to the
owner per `design@agent-skills@questions-go-to-their-authority`, whose premise proves false is a contradiction: the corrected premise goes to the owner, and
the part of the work that rests on it waits, the rest going on. Whether a project follows a
specification is the project's goal or decision, and a departure from one is a scope decision the
project states, never inferred from an answer to a reading question.

The three rules sit in one section because the second and the third are the first's two cases,
split by the authority of the question, per `tripwire@agent-skills@split-rule-from-exception`. The
hold derives from `goal@knowledge-architect@the-owner-s-word-is-verified`. The source rule's nearest
rival, putting the owner's answer back to the owner with the source's reading as the default, is
what thaum built first: it still asked, offered an option the specification forbids, and two of
thaum's reviewers found it contradicted the specification's authority. The design skill holds the
same duty for a closed thread, its material finding, and reaches only a session that has loaded it,
where a word of the owner can arrive in any session, which is the content test of
`design@agent-skills@primer-limit`. So the directive sits in the primer: it arrived twice outside a discussion,
in a review repair and at the fourth entry test of `design@agent-skills@a-head-is-owed-by-an-entry-test`,
the second on a premise the session supplied, that the decision failed the first three tests, when
the same change had made the second pass. A word acted on past its premise records a decision the
owner did not make, against `goal@knowledge-architect@the-owner-decides`.

### Something met outside the task takes the first outcome of the primer's table that applies, routed by its authority, and the turn lists every item `##met-outside-the-task-outcomes`

`primer@met-outside-the-task` gives four outcomes, the first that applies. An item that bears on
the work is routed by `primer@question-authority`, per
`design@agent-skills@questions-go-to-their-authority`: what a source, a measurement or the record
answers, the session decides and reports; a fork, a goal or a fact only the owner holds goes to the
owner with a default the work proceeds on, a goal standing as it is meanwhile; a contradiction, or an action that cannot be undone, holds the part of the work
that rests on it, as a contradiction does wherever it is met, per
`goal@knowledge-architect@the-owner-s-word-is-verified`, and as `primer@owner-word-premise` and the
design skill's material finding say. A fix checkable from the diff alone lands in a commit of its
own; an item whose `Why it matters` and `What would close it` can be written is opened as an
issue; any other is named, with why it is dropped. A turn that met anything ends with a section
listing every item and its outcome, and a subagent that edits nothing reports each item to the
session that dispatched it, which routes it.

A one-line mention inside a long report is easy to miss: the owner saw an agent find a small issue,
judge it too small for an issue entry, not fix it, and report it in the conversation in one line,
so every outcome leaves a record or a listed line. A fix under the second outcome needs no word of
the owner's: the owner accepts such an autonomous fix as long as it passes the test the session
wrote. The directive sits
in the primer since every session meets something outside its task.

### A turn that decided what the owner could have decided ends with a Decided section `##decisions-reported-to-the-owner`

`primer@decided-section`: a turn in which the session decided something the owner could have
decided ends with a section titled "Decided", one line per decision with its ground, before "Met
outside the task" where both are owed. A design thread closed on its argument, a default the record
answered, a source applied over an answer of the owner and a rewording of a tripwire's clauses are
such decisions; an implementation choice a commit message records is not. A subagent reports its
decisions under the same title, and the dispatching session carries them into its own.

It serves `goal@knowledge-architect@the-owner-decides`, under which the owner is informed of every
decision and can overrule any of them, and
`design@agent-skills@questions-go-to-their-authority` gives the session many of those decisions. The nearest rival, the commit messages and the pull request, loses because a decision
made and reverted within one session reaches no commit, while the owner reads the turn. An
implementation choice is left out because a list of every choice would hide the few the owner could
have made. The directive sits in the primer since a session of any activity makes such decisions.

### Every rule on what a design head records and how lives in one section of the primer `##one-home-for-head-rules`

What earns a head, what a head holds and how it is shaped are one section of the primer,
`primer@design-heads`. The decision-recording skill keeps the procedures of writing one; every
other text points to the section rather than restating more than one sentence of it, per
`design@agent-skills@restatement-size-test`.

The centralization is the owner's proposal: "I want to make sure the rules for what is recorded in
design heads and how are centralized, consistent with each other and applicable. I think that
partial restatements of them should be avoided at all cost, instead pointing to a single location
where they are all together and asking for a read of that full location when needed." The rival,
restating the rules wherever they are delivered, gives partial copies that drift: an inventory of
the installed text at commit c282b00, re-taken by a fresh read-only subagent listing each statement
of a rule on heads outside its home at that commit, finds about ten texts restating parts of the
decision-recording skill, with nine inconsistencies between them and it. The section is also the home of four rules on heads that have
no head of their own: present tense, the alignment of the slug and the title, a decision worth a
slug has its own level-three heading, and fidelity to what the owner approved; this head records
them as part of the section. The location is the primer on the owner's arguments that the
rules are "nearly baseline": "nearly every piece of work will have to add or rewrite recorded
design", "they are useful to interpret recorded design", and "it is useful to be able to identify
in advance what kind of decision may need to enter the design home". Reading a head loads no skill,
and every subagent holds the primer in its initial context, so a pointer to the section costs
nothing. That is measured in one harness by a probe, re-taken by dispatching a subagent of each
type, with no file read and no tool, and asking it to quote a sentence of the primer from its
initial context; a subagent that cannot reopens this decision. The nearest rival, one section
of the decision-recording skill, is enough only where every session that reads a head loads it
first, and nothing makes it do so. The cost is the primer's size, roughly doubled, in every session
of every installing project, which `design@agent-skills@primer-limit` admits for what no skill
delivers at the moment it is needed; it is re-taken with `wc -w` on the primer.

### A directive is restated only where the restatement is no longer than a pointer to it `##restatement-size-test`

A directive is restated at its point of delivery only when the restatement is no longer than a
pointer to it: a path, a file name, a command, a value, or one sentence, a sentence carrying its
pointer beside it. A longer directive is delivered by a pointer to its home, read whole at that
moment, and more than one sentence of it is never restated. For installed text, the home a pointer names is
installed text, since installed text cites no entry of the project, per
`design@agent-skills@shipped-text-cites-no-entry`. The rule governs new text and text a change
touches: an existing longer restatement is converted when a change edits what it says, and no sweep
converts them all; re-pointing a reference in it does not count as an edit, since it changes no
directive. A sweep would convert at once every long restatement a project holds, each of them a
delivery choice of its owner, where a conversion at an edit costs one read of a text the change
already reads. The rule is the primer's `primer@where-knowledge-goes`.

The test is the owner's proposal: "is the restatement significantly bigger than a pointer ? Then use
a pointer." Its reason is the owner's too: the rule that a directive is restated wherever it is
delivered "is meant to avoid loading excessive unrelated informations", and "Just state the
path/filename, it takes as much space as the pointer", while "We can't restate a skill inside every
other one". One sentence is the checkable form of that size, since a pointer with its instruction to
read the home whole is itself about one sentence. A partial copy drifts, and its reader takes it for
the whole: the rival, a directive restated wherever it is delivered, gives the partial restatements that
`design@agent-skills@one-home-for-head-rules` measures. That rival's own reason, that a reader who cannot reach a statement at the moment
of acting is not served by a pointer, stays answered for a short directive, which is still restated.

### The decision-recording skill asks for the primer's section on design heads to be read again before a head is written or judged `##primer-reread-before-recording`

The decision-recording skill opens by asking for `primer@design-heads` to be read again, whole,
before a head is written or judged.

The rule is the owner's proposal: "The point that would worry me however would still be dilution
over long session. Maybe a few instructions to reread the primer should be kept at key locations,
in particular the decision recording skill whose writes happen at the end of sessions". Every write
into a design home loads that skill first, per
`design@agent-skills@design-home-write-loads-recording`, so one instruction reaches every write,
and that skill is also loaded before an edit of a text a head describes, where the session judges
the edit against the heads; such an edit comes at any point of a session, and where it comes early
the reread costs one read. The reread is owed by the session that writes or edits: a review agent
that judges heads starts with a fresh context that holds the primer, so it needs none.

### The primer names the plans directory the checker fixes `##plans-directory-in-primer`

The plans directory is docs/plans/ in every project, per `design@core@plans-dir-fixed`, so the
primer's row for unbuilt work names it, and the installed skills name it in plain text. A project's
own row for it would only repeat the primer's, and is removed, per
`design@agent-skills@knowledge-table-home`. A row each project declared lost: once the checker
fixes the path, a declared one could name no other.

### A project skill reaches a session through its own description and through the skill of the activity it serves, with no table mapping it to an installed skill `##no-routing-table`

A project's root `CLAUDE.md` carries no table of which project skill adds to which installed one.
Two mechanisms already deliver a project skill at its moment: the harness lists every skill with
its description every session, and the description begins with MUST and names its symptom, per
`skill@knowledge-architect-agent-configuration@shaping-a-skill`; and an axis a project adds to a
review is owed through the skill of an activity whose work the review reads, per
`skill@knowledge-architect-review@review-axes`. A table restates both, and its "adds to" is a fixed
mapping between a project skill and an installed one, which each new kind of link between them
must vary, against `design@agent-skills@capabilities-not-structure`. The fact that defeated it: in
this repository's first agentic-workflow audit, no row of the table delivered anything the
description or the dispatching skill did not, and every finding on the routing side turned on
whether a row was owed. After an install that changes the installed files, each project skill and
agent is read against the installed text that changed, not against a row.

### A project adds to the workflow through skills of its own, never by editing an installed one `##overlay-by-separate-skills`

An installed skill is complete on its own. A project adds its conventions through its own skills,
agents and root `CLAUDE.md`, under its own names. An installed file is compared byte for byte with
the pinned version and overwritten by the install, per `design@core@owned-namespace-check`, so an
edit to it fails the check and is lost at the next install; a project skill with the same base
name as an installed one would load beside it, not replace it. Where a project skill would have to
contradict an installed one, the installed text is wrong for that project, and the owner reports
it. A project that needs to change one instruction has no means yet:
`issue@agent-skills@patching-an-installed-skill`.

### A project's own files in the installer's directories carry its name as a prefix `##project-name-prefix`

A project skill, agent or saved workflow is named `<project>-<activity>`, the directory or file
name equal to the frontmatter `name`, or to the `name` of a saved workflow's `meta`, so it is told
apart from an installed one and from another project's. A colon,
as in `<project>:<activity>`, is not used: a subagent's name cannot hold one, and a skill so named
cannot be told apart from a plugin's.

### Setting up stops at a conformant structure, and the move of existing documents is planned work `##adopting-existing-docs`

For a project that already has documentation, the setup skill takes an inventory of it,
proposes a destination for each document, places a document a row of the knowledge table fits,
per `design@agent-skills@questions-go-to-their-authority`, takes the owner's rulings on the others,
and opens one `todo` issue for the move. The move itself is planned work of its own, written under
the planning skill. A migration is a decision with arguments of its own, such as which recorded decisions still hold, and it fits
in one session only for a small project. The issue keeps the old documents beside the new homes
listed as outstanding work, per `design@agent-skills@undesigned-work-is-an-issue`.

### A project pins the checker exactly, and its manifest declares the pin `##exact-pin`

A project depends on one exact version of the checker, so moving it is an explicit edit, and the
installed skills move with it. **The manifest declares the
same pin**, in `[project] checker-version`, and every binary that dispatches a command of the
checker refuses to run over a project whose pin it does not satisfy, per `design@core@installed-binary-version-check`, so an install or a build
left behind by a move of the pin is refused at its first command rather than run. The setup skill
writes the key and moves it with the pin.

### A project runs its pinned checker from inside the project, never from a machine-wide install `##pinned-binary-runs-inside-the-project`

A dependency builds no executable for the project, so a Rust project
runs the pinned version through its maintenance crate, per
`design@agent-skills@xtask-pins-checker`, and any other project installs it into a directory of its
own with `cargo install --locked --root`. A Rust project that is one package with no workspace
gains a workspace for that crate, which keeps the gates `goal@knowledge-architect@setup-brings-quality-tools`
asks for, or installs the binary as any other project does and builds its own gates, on its
owner's ruling, since the first changes the project's build. A machine-wide install would give two projects on one machine one
version, which is what bundling the workflow into the checker avoids, per
`design@knowledge-architect@binary-bundles-workflow`. A project with an extension runs its own
binary under a name of its own, per `design@core@declared-command`.

### The setup skill recommends one gates command, run by the published gates library where the project has a maintenance crate `##gates-convention`

One command runs every check a project owes before a merge, runs them all when one fails, and exits
non-zero when any fails, so a verdict is one exit code and nothing is read through a pipe. In a Rust
project with a maintenance crate it is a command of that crate, per `design@agent-skills@xtask-pins-checker`, run by
the published gates library, per `design@gates@gates-crate`, so every adopting project with a maintenance crate runs
the gates refined in this repository and in thaum. The gates a project owes are its own list, per
`design@gates@a-project-holds-its-gate-list`.

### The setup skill recommends `check --staged` after staging and before each commit `##staged-check-before-each-commit`

The setup skill recommends that a project's root `CLAUDE.md` tell a session to run
`check --staged` after staging and before each commit, beside the gates command, which
judges the branch before a merge. The command is the core's, `design@core@staged-tree-source`. The
reason: the staged tree is the one the commit records, and a file left unstaged, or a change staged
in part, passes a check of the working tree and fails the commit's own, which the gates meet only
before the merge.

`design@agent-skills@behaviour-additions-need-real-use` asks of an addition a behaviour seen in real use, or
asked for by the owner with the lack named. The setup recommendation has no named lack of its own,
so it is an exception to that rule, argued on delivery: without it, no shipped text tells a session to check
before a commit, so a command that judges the commit's tree is used only where a project's own
instructions already ask for it. Its rival, dropping the recommendation until a real session shows
the lack, keeps the rule and leaves the command undelivered. Its cost is an instruction no observed
session asked for.

### The skills name the staged forms of the checker's commands for a commit of part of the working tree `##staged-forms-for-a-partial-commit`

The issue-tracking and planning skills name `index --staged` and `check --staged` for a commit of
part of the working tree. Both commands are the core's, `design@core@staged-tree-source` and
`design@core@index-staged-write`. The staged tree is the one the commit records, and a change
staged in part passes a check of the working tree and fails the commit's own.
`design@agent-skills@behaviour-additions-need-real-use` asks of an addition a behaviour seen in real use, or
asked for by the owner with the lack named. The owner asked for the delivery, "identify shipped
workflow changes needed to deliver this new feature", and, on the refusal of `--fix` for a partial
commit, named what a session does instead: "the recommended action would be to run check without
--fix and restricted to staged area, and fix findings manually (fixing indexes might need index to
accept running on staged changes)". The rival, the skills naming only the working-tree forms, leaves
that action undelivered.

### In a Rust project with a maintenance crate, that one crate pins the checker and runs the gates `##xtask-pins-checker`

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
`goal@knowledge-architect@the-owner-decides`. Nothing checks it mechanically yet:
`issue@core@a-component-states-at-least-one-goal`.

### The installed skills do not mention a project whose name is the installer's namespace `##installed-text-omits-the-prefix-collision`

The installed skills name a project's prefix rule, per `design@agent-skills@project-name-prefix`,
and say nothing of a project whose name is the installer's namespace. Outside this repository the
collision is unlikely, and a rule for it in the shipped text would be read by every installing
project. The owner: "The collision risk outside of this exact project is ridiculously small. This
would only add noise to every project that uses this workflow".

## Goals

### A goal stays while it is met, and leaves only on the owner's word `##goal-lifecycle`

A goal is met or unmet, and stays in its goals home while it is met. The rule and its reason are the
owner's, in the root goals home as the owner adopted it: "A goal removed when it is achieved stops
being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word." A goal leaves only when the owner abandons it, per
`goal@knowledge-architect@the-owner-decides`.

### A goal entry states an outcome, and what would show it met `##goal-entry-shape`

A goal is a heading stating an outcome as a sentence, with its slug, at the level the core gives
the goal register, per `design@core@an-entry-is-a-heading-at-the-register-level`, and one short
paragraph saying what it means and what would show it is met. It never states a mechanism: the test
is whether a change would make the Component for something else, which is a goal, or reach the same
end another way, which is a decision. Goals stay short, because a goals home is read whenever a
decision is argued from one.

### Goals change only through the goal-setting skill `##goals-change-through-goal-setting`

The goal-setting skill runs at setup for every Component, when the owner states, rewords or
abandons a purpose, and when a decision conflicts with a goal and the primer's rule sends the
conflict to the owner. An agent never edits a goal outside it, since a goal binds outright where a
decision binds as a presumption, per `goal@knowledge-architect@the-owner-decides`.

## The retrospective

### A retrospective is offered at a moment an event names, never at one the agent judges `##retro-trigger`

The retrospective is offered when one of three events occurs: a branch the session worked on has
merged into the main branch, a plan document has left, or the owner says the session is ending. It
runs only if the owner accepts. A moment named by an event can be followed by any agent where "a
moment it judges right" could not.

### A retrospective is offered at most once per session `##retro-offered-once`

The retrospective is offered once per session, at the first of the moments
`design@agent-skills@retro-trigger` names, and not again at a later one. Offering it once keeps it
from interrupting work.

### A retrospective asks standing questions, each on the decisions it watches `##premortem-as-watch-points`

Every retrospective asks whether the session needed to change an installed skill or agent, whether the
session or one of its subagents acted as if a rule of the primer were absent, whether a project
skill's addition was missed, and whether the session needed to write a reference or a path without
backticks because no checked form expresses it. Real use is the test of the workflow, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`, and each question watches a
decision whose failure would be seen in real sessions before any check could see it:
`design@agent-skills@overlay-by-separate-skills`, the primer's delivery by an import line in
`design@core@owned-namespace-check`, `design@agent-skills@no-routing-table`, and
`design@agent-skills@plain-text-is-no-repair`. The last question is the
one channel by which a gap of the checker met in a consumer project reaches this repository, since
the consumer's own entry for it is not citable here. The primer question asks about behaviour, on
the owner's direction: "this one should directly ask a session about what it observed in the
behavior. No edit of the subagent briefs." A session sees a subagent's brief and its report, not
its context: asked whether the primer reached the subagents, four received retrospectives answered
that it could not be observed, or was assumed, as the issue closed in commit 61a9219 lists, read
with `git show 61a9219^:crates/agent-skills/docs/open-issues/the-primer-question-cannot-be-answered-for-subagents.md`. The skill never states how many questions there are, so adding one changes no count.

### A retrospective's files go where the owner directs, and nothing of them leaves the machine before the owner's reading and word `##retrospective-destination`

The owner reads both files verbatim and may edit them. The workflow's file can become a public
issue on another repository, and a finding written in a session can carry the project's content, so
nothing of either file leaves before the owner has read it. On the owner's word, and where the owner
directs, the project's findings become issue entries in its own register, and the workflow's file
becomes an issue on knowledge-architect's repository, through `gh` or by the file and the address.
The repository is the workflow's own upstream, the one name of a project the shipped text holds.
