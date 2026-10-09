# The knowledge-architect workflow

This project keeps its design record with knowledge-architect. Its documents are checked by
`{{command}} check`: every reference between records resolves, every record has the shape its
register declares, and every generated listing is current. This primer is installed, and the
project's root `CLAUDE.md` imports it. It holds what every session needs and no installed skill
delivers. The project's own rules are in its root `CLAUDE.md`, beside this primer, and add to it.

## Goals bind; decisions bind as a presumption {{slug:goals-bind}}

**The goals are the only statements assumed to come from the owner.** Each Component states them
in its goals home, `path@*@docs/goals.md` or `path@*@docs/goals/`, one heading per goal. A recorded decision was reviewed, but its review can
miss a detail or an implication, more often as the volume of agentic work grows. A decision binds
as a presumption, which a better argument may rebut, and reversing one is an ordinary move with a
procedure (`skill@knowledge-architect-decision-recording`). **When a decision conflicts with a goal,
the likely cause is that the owner missed the conflict: the goal prevails, and the conflict goes to
the owner.** It is not resolved by following the decision.

## The owner's word and its premise {{slug:owner-word-premise}}

**A word of the owner holds only as far as its premise.** A ruling given on a premise, stated by
the owner or supplied by the session, does not rule on the case where that premise is false. When a
premise proves false, put the corrected premise to the owner at the top of the turn, quoting the
word it defeats, with a default chosen in view of the corrected premise, and proceed on the default
unless the owner answers otherwise; a part of the work that cannot be undone waits for the answer.
A premise the session supplied is the session's to check before the owner rules on it.

## Room to judge {{slug:room-to-judge}}

**Where the installed text is silent, judge.** It leaves that room on purpose: it states the
instructions the workflow needs and the reasons for them, and leaves the rest to the session. Room
to judge is never room to act against an instruction.

## Intent and claims {{slug:intent-and-claims}}

- **A design home is built intent**: the design as built and its reasons, and the decisions that
  no work implements and that are not part of any spec, recorded when made. Design that is decided and not built is in a plan
  document until it lands. Check the code against a design home, never the other way. A divergence
  is a defect in one of them: say which, open an issue, and stop. A design home can be wrong, and
  it still prevails over the code until the issue closes. It closes when the code changes to meet
  the head, or when the head is reversed under `skill@knowledge-architect-decision-recording`; it
  never closes by following the code. Work that goes on meanwhile, on the owner's word, builds on
  the head.
- **A reason recorded at the code** (an inline comment saying why the code is shaped so, or the
  message of the commit that argued it) is intent at the scale of that code. It binds as a
  presumption, below the design home: a change that defeats it argues against that reason in its
  own commit message, and where it conflicts with a design home, the design home prevails, as in
  any divergence. Between a comment and a commit message, the comment is the current statement.
  **Before removing or reshaping code as unneeded, read its comment and the message of the commit
  that introduced it** (`git log -L`, `git blame`): absence from the design home is not evidence
  that code is superfluous.
- **A claim about the code as it stands** (a scoped `CLAUDE.md` invariant, a doc comment, a name, an
  issue's diagnosis) goes stale. Verify it against the code, or against a run you observe, before
  relying on it.
- **Before diagnosing anything as a problem, check whether it is already recorded:**
  `{{command}} issues`, `{{command}} tripwires`, and `{{command}} show <kind>@<anchor>@<id>` for one
  entry and every reference to it.

## Something met outside the task {{slug:met-outside-the-task}}

Something met while doing other work takes the first of these that applies:

| # | test | outcome |
| --- | --- | --- |
| 1 | it bears on the current work: the work's result, or a decision it rests on, is wrong or incomplete without it | stop and present it to the owner at the top of the turn, with a default |
| 2 | its fix is checkable from the diff alone: it changes no behaviour, no decision and no test outcome (a typo, a stale pointer, wording that is now false, a broken link) | fix it, in a commit of its own |
| 3 | its `Why it matters` and its `What would close it` can be written | open an issue entry (`skill@knowledge-architect-issue-tracking`) |
| 4 | none of the above | name it, with why it is dropped |

**A turn that met anything outside its task ends with a section titled "Met outside the task"**,
listing every item with its outcome: fixed (with the commit), issue opened (with its id), waiting
for the owner's ruling, or dropped (with the reason). A mention inside other prose does not count.

## Where knowledge goes {{slug:where-knowledge-goes}}

**Every durable decision has exactly one home.** A second mention of a decision is a reference to
it, never a copy, because a copy starts drifting the moment it is written. **A directive is
restated where it has to be delivered only when the restatement is no longer than a pointer to
it**: a path, a file name, a command, a value, or one sentence. A directive sentence carries its
pointer beside it, and where the two disagree the restatement is the defect. A directive longer
than one sentence is delivered by a pointer to its home, with an instruction to read the home whole
at that moment. More than one sentence of it is never restated: a partial copy drifts, and its
reader takes it for the whole. For installed text, the home a pointer names is installed text, a skill's section or the
primer's, since installed text cites no entry of the project. An existing longer restatement is
converted when a change edits what it says; re-pointing a reference in it does not count. Whether a directive is needed at a point of delivery is the
owner's decision.

| the statement is about | home | it leaves when |
| --- | --- | --- |
| what the project, or one Component, is for, and what would show it achieved | that Component's goals home, `path@*@docs/goals.md` or `path@*@docs/goals/` (the project's root is a Component) | the owner abandons the goal |
| how the project or a Component is built, and why | that Component's design home, `path@*@docs/design.md` or `path@*@docs/design/` | the design changes: the entry is rewritten in place |
| the engineering alternative that lost, and why | that Component's `path@*@docs/rejected-alternatives.md` | never; a reversal moves the old winner into it if it meets a recording test of `skill@knowledge-architect-decision-recording` |
| what is outstanding: a defect, an unexplained observation, an open question, missing work | one file in the owning anchor's issue directory, `path@*@docs/open-issues/` in a Component | the issue closes |
| evidence that would flip a recorded decision about code that exists | the owning Component's tripwires home, `path@*@docs/tripwires.md` or `path@*@docs/tripwires/` | it fires, or its decision is gone |
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
depends on; an issue names the decision it strains and the goal it threatens, when it does
directly; a guard, a
workaround, a stub or a test that exists because of an issue names it in the comment at the site; a
tripwire names its decision; a rejected alternative names the decision it lost to; a commit message
names every entry it opens, closes, reverses or argues from. A reference whose entry's change would
leave the text unaffected is not written. A reference is one backticked span,
`<kind>@<anchor>@<id>`, naming the anchor that defines the entry, and it is live wherever it is
prose, a fenced block included; an illustration that must not resolve writes a placeholder in angle
brackets. The checker reads Markdown and Rust source; a reference anywhere else is found by grep.

**A finding is repaired in a form the checker judges, never by moving the pointer into plain
text**: the right anchor, `path@elsewhere@<path>` for a path the tree does not hold, a placeholder,
or a rewritten sentence. A reference or a path that no checked form expresses is written without
backticks only beside a reference to an issue entry of this project that records the missing form.
A gap of the checker itself gets that entry in this project's own register, since a reference
cannot reach another project. Text that is not in the checker's syntax, such as a commit named by
its subject, a commit of another project or a description in words, is outside this rule.

## Design heads {{slug:design-heads}}

A design head is an entry of a design home: a level-three heading that states a decision, and its
body. This section holds what earns a head, what a head holds and how it is shaped. Read it to read
a head, to tell in advance whether a change bears on one, and before writing or judging one;
`skill@knowledge-architect-decision-recording` holds the procedures of recording, and is loaded
before any write into a design home.

### What earns a head

Most implementation choices earn no head. A unit of work produces dozens of them, and a design home that
records dozens per unit of work stops being readable and stops being ranked.

**A decision earns an entry in a design home only if at least one of these holds:**

1. reversing it would change **an interface others consume**: a type or a signature that crosses
   the boundary of a separately built unit (a crate, a package, a library, a module others import),
   a file format, a document grammar, a command line;
2. **the same reason must be respected at more than one site, or at none.** A reason is an argument
   against a rival someone could plausibly propose; a name, a path or a value repeated for
   consistency is not one, since nothing argues for changing it and a stale copy is found by a
   search. At more than one site, the reason needs a home each site can cite, and a comment cannot
   be cited. At none: a decision about an absence ("we do not do X"), or a policy that no code and
   no text of the project states. For a decision about agent-facing text, a skill, an agent or a
   `CLAUDE.md`, each text that states the instruction is one site: a decision stated by one text
   has one site, and its reason lives in that text, beside the instruction; or
%% In this repository, the place beside an instruction of the installed text is a `%%` line:
%% `design@agent-skills@shipped-text-line-comments`.
3. **its argument turns on the behaviour of something outside the project**: an external
   specification the project implements (a standard, a protocol, a rule set), or an external tool's
   behaviour, read in its documentation or measured; or
%% The `Q` label: `design@agent-skills@ruled-items-labelled`.
4. **the owner confirms that it records the owner's own intent**: a ruling the owner gave that an
   agent could judge superfluous and reverse as a small fix, or an argument the owner made and wants
   kept so as not to restate it. The agent does not judge this. When a decision would earn a head
   by this test alone, ask the owner, one numbered question per decision (Q1, Q2, …), several in
   one message, each quoting the owner's words the decision rests on: do they record the owner's
   intent, or were they an answer to the agent's proposal, or a hedged statement ("I think",
   "maybe"), which is a position to argue under `skill@knowledge-architect-design`? The head quotes
   the owner's words and the owner's answer. With no answer, the decision earns no head by this
   test.

Test 3 matters most in a project that implements a specification or leans on a tool's behaviour. A
choice that turns on what the specification means, or on how the tool behaves, is expensive to get
wrong and expensive to derive again, and it is visible: the argument quotes the specification or
the documentation, or names the measurement.

Otherwise it belongs in an **inline comment at the code it explains, plus the commit message**.
That is not a lesser home: the comment is read by every session that touches the code, which a head
is not, and the commit carries the argument. A reason that fits in one comment at one site, about
that site's own code, fails test 2.

**The backstop. A decision that creates a head, contradicts a statement of one, its argument
included, or adds a member the head's argument does not cover, goes to the design skill before its
text is written**, unless it was argued there, under `skill@knowledge-architect-design`. This is the case of
a decision met during another task and settled there, by the owner's word or by the session's own
choice. The design skill's in-change path keeps the deliberation in the commit message, so the task
needs no plan document and no new session. An addition that answers another question than the
head's takes a head of its own, by the test of one decision per head below. A member the head's
argument covers is recorded directly, routed as the next paragraph says; where the owner approved
it, the approval is quoted in the commit, not in the head. A change that relocates or rewords
recorded decisions, a split of a head included, and adds or removes none, is not a decision: it
needs no design skill, and the routing and fidelity-of-relocation review axes judge that it adds or
removes none.

**A new member of a head is routed by the head's ground.** A member is what a head's rule admits
and its body does not yet name. Where the part of the head that admits it stands on its argument,
and the argument covers the member, it is recorded directly. Where that part's ground is the
owner's words and they state the rule, the member is within the ruling and is recorded directly,
the words quoted as before. Where the owner's words name the members, it goes to the owner as one
proposal: one message stating the rule, the member and a default, and nothing of it is written
before the owner's word; it is no change of the decision, so it owes no reversal. A member the rule
does not admit, or that the argument does not cover, is a change of the decision, and goes to the
design skill, as the backstop says.

### What a head holds

**The standing argument** is every premise whose failure would reopen the decision: the goal it
derives a constraint from and a decision of another Component it depends on, as references; the
measurement it rests on, with the command that takes it again; the fact that defeated its nearest
rival. **The test: if this premise turned false, would the decision have to be argued
again? If yes, it is in the head.** If no, it is deliberation.

**A head stands on its argument, and cites the owner only for what came from the owner.** Most
decisions are proposed, argued over their costs and rivals, and approved by the owner. The head
records such a decision on that argument, with its costs and rivals in its body or in the rejected
alternatives. An approval is not a ground, whatever its words ("approved", "agreed", "all defaults
approved", "accepted the cost"). It stays in the deliberation: the plan document or the commit
message. The owner's words are a ground, quoted, only where the decision came from the owner: the
owner proposed it, the owner chose where the argued rivals did not settle it, or it rests on a
premise only the owner can state, such as an intent, a plan or a weighing the owner made. Test 4 is
such a case. In a head that holds a part of each, the part that came from the owner says so, and
the rest stands on its argument.

**What the ground means to a later session.** A head that stands on its argument is reversed by a
better argument, which the owner then approves. A head whose ground is the owner's words is
reversed by the owner, or by a defeated premise of those words. An argument against it goes to the
owner as a question about their intent.

**The deliberation is not copied into the head.** Where it is kept, and how a reader finds it,
is `skill@knowledge-architect-decision-recording@three-homes`.

A head is written **as if the design had always been so**. Present tense, no dates, no "formerly", no
account of the change. If you find yourself writing "we used to…", that sentence belongs in the
commit, and so does an opening that motivates a decision by describing the state before it.

**A head carries intent, shape and the standing argument, not implementation.** What the project is
for, and how it is arranged in order to get there, belongs here. How a particular function does its
work belongs in a comment at that function. The test: **if changing a piece of code would force a
change to the design home, it is design; if the design home would be unaffected, it is a comment.**

**Name what the argument depends on.** The goal a constraint is derived from,
`goal@<component>@<slug>`: a constraint from a goal binds outright, where one from a decision is a
presumption, and the reference is what tells a reader which. A decision of another Component the
head depends on. **A reference is a claim that this head is revisited when that entry changes**,
so a reference whose entry's change would leave the head unaffected is not written. Never list what
cites this head: `{{command}} show design@<component>@<slug>` computes it.

A decision that relies on the checker of this workflow states that it relies on the checker
working as intended. It cannot reference the checker's own decisions: a reference resolves only
against the project that holds it.

### How a head is shaped

Give the decision a **slug anchor**: a short hyphenated name in backticks, prefixed with `##`. It
goes at the **end of a level-three heading**, the level the design register declares, so that the
outline reads as decisions under level-two subjects. **Every level-three heading in a design home
is an entry** and carries a slug: one without is a finding, so a heading that is section text sits
at level two or four. Nowhere else: a slug at another heading level, in a table cell, at the head
of a plain line, in the middle of a line or in a file that is not the design home defines nothing.
`{{command}} check` reports it as a misplaced definition, and every reference to it as dangling.
The slug is an id in the grammar `[a-z0-9]+(-[a-z0-9]+)*`, unique in the design home.

**When the decision was a thread of a design discussion, its slug is the thread's name**, unless
that name misdescribes the decision as approved, as a name for the change it proposed does. The
discussion minted it in the same grammar and checked it for a collision with the Component's
entries before using it, for that reason. When the name misdescribes the decision, the entry takes
a slug that names the decision, by the alignment rule below, and the text that keeps the
deliberation states the pair, `#<thread> → <entry slug>`: the plan document's harvest row, or the
commit message on the in-change path. `git log --grep` on either name then finds a commit message
that states the pair, and `git log -G` a plan document's diff that does.

**A list item is not a definition site either**, so a decision written as one bullet among several
carries no anchor and cannot be cited or found by `git log -G`. This is a constraint on the
document rather than a gap in the checker: **a decision worth a slug is worth its own level-three
heading**. When a section of bulleted arguments produces one, break it out of the list.

The statement comes first and the slug last, with no bold and no em dash between them, so that a
document outline reads as a list of decisions rather than a list of identifiers. The body follows as
ordinary prose.

**A title states a decision only while it is false of the nearest rival it beat.** A title that the
losing alternative would make true names a subject, not a decision: "The configuration is read
once, at start-up" is false of a configuration read again on every request, and "The configuration
is read with care" is true of nearly any rival.

**A title states the rule that decided, not the list of what it admits today.** When a decision
admits members (fixes, kinds, verbs, sections, consumers), or picks a mechanism to meet a
requirement, the title states the property that admits a member, in the argument's own terms, and
the body names the members built as what the rule admits today. A count or a list in a title goes
stale at the next member, and every later member then reads as a change of the decision. A set
closed on purpose keeps its list in the title, with the sentence that argues the closure in the
body. A title states no more than its argument argues.

**A head holds one decision.** Two statements are one decision when they answer one question and
lose to the same nearest rival. A rule's exception, its parameter and its delivery belong to its
head, since none means anything without the rule. Two statements that lose to different nearest
rivals, or whose arguments share no premise, are two decisions. Each takes a head of its own when
it passes an entry test, and a comment at its code when it passes none. A title joined by "and"
over two decisions shows a bundle. Rewording the title to state both does not make them one. A
member of a set the title's rule admits is no decision, so a member missing from a title is no
finding.

**A head written from a decision the owner approved is read against that approval before it is
written.** The title and the body state the position as the owner saw it, in a plan document's
thread or in the discussion of the in-change path. A clause that widens it, narrows it, or drops
part of it can still be false of the nearest rival, and is still not what was approved: it goes to
the owner, as a change of the decision.

**The slug and the title stay aligned with the full scope of the decision**: the slug is often the
only part a reader sees, in a citing document or in code, and the title is what a document outline
shows. A slug or a title that misdescribes its decision misinforms every reader, or undermines the
decision it names, so rename it even when that means rewriting every reference in the project.

**A head that a change touches is brought to these rules in that change**: its title to the rule it
argues, each decision it bundles to a head of its own, its ground to its argument. Where bringing it
to the rule would widen what the owner's words in it approved, as a title moved from the members
the owner named to the rule, the change goes to the owner as one proposal, as a new member does.

## The installed skills {{slug:installed-skills}}

- `skill@knowledge-architect-decision-recording`: before writing into a design home; a design
  decision has been made or reversed.
- `skill@knowledge-architect-issue-tracking`: before diagnosing a problem; parking anything; a
  tripwire fires; work closes an entry.
- `skill@knowledge-architect-design`: a design question has an open solution space; a requested
  change whose design is not settled, to ground it and find out whether it is bounded work;
  keep-or-change about an existing design; a bug trend suggests the design is the problem.
- `skill@knowledge-architect-planning`: a design discussion converged on its full path; a slice of a
  milestone starts or lands; the work of a spec starts or lands.
- `skill@knowledge-architect-review`: before merging to the main branch, or when an
  activity's skill says its work is ready.
- `skill@knowledge-architect-agent-configuration`: before editing a `CLAUDE.md`, a skill or an
  agent.
- `skill@knowledge-architect-setup`: the project adopts the workflow, or a version upgrade is
  installed.
- `skill@knowledge-architect-goal-setting`: before writing or editing any goals home; a Component
  has no goal; the owner states or abandons a purpose; a decision conflicts with a goal.
- `skill@knowledge-architect-retrospective`: once per session, offered when a branch the session
  worked on merges, a plan document leaves, or the session ends.

A project's own skills add to these, and never replace them. Which project skill adds to which
installed one is the routing table of the project's root `CLAUDE.md`. Read the installed skill and
every skill the table lists beside it.
