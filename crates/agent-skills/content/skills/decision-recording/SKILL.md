---
name: knowledge-architect-decision-recording
description: MUST use when a design decision has been made or reversed — a choice about how the project or one of its Components is built, including anything a consumer of it may depend on — in order to decide whether it earns durable text at all (most implementation choices do not) and where that text lands. Covers when recording happens, the reversal check that comes before everything else, the recording tests, which Component owns it, the split between the design home, the rejected alternatives and history, slug anchors, and the tripwires a premortem produces.
---

# Recording a decision

Scope: turning a decision about **what we build** into durable text. Whether it earns durable text
at all, when it is written, which Component owns it, what goes in the design home, what stays in
history, and what the premortem leaves behind.

Not covered here: **having** the discussion (`knowledge-architect-design`);
**writing the spec or the milestone document** that carries a decision until its work lands
(`knowledge-architect-planning`); **parking something undecided**, and writing a tripwire's entry
(`knowledge-architect-issue-tracking`). A decision about the agent configuration itself is
both this and `knowledge-architect-agent-configuration`: that skill owns how to write the
configuration, this one owns where the argument lands.

## 0. When recording happens

**A decision is recorded when the work that implements it lands, not when it is decided.** A
design home holds built intent: the design as built and its reasons, against which the code is
checked. A head written before the code exists would report a defect in code nobody has written.
While the work is open, the decision is unbuilt intent, and the spec or the milestone document is
the only place it exists, on the main branch or on the work's. Writing it into the design homes from there is
the **harvest**, and it happens in the change that lands the work.

A decision that constrains work nobody has started, and that is not part of any spec, is recorded
when it is made: there is no implementing change to wait for. §2's second test is that case.

## 1. Does it reverse something already recorded?

**Ask this first, because the answer decides everything below it.** A decision routed by what it
_is_ lands in the Component that fits its subject. A decision that reverses a recorded one belongs
where the incumbent already lives, whatever the subject test would say. Get the order wrong and the
tree ends with two homes for one question, the old one still asserting what the new one denies.

**Search for the statement you are about to replace, not for a slug.** The slug you would look for
is the one you have not written yet, and the incumbent's own name is rarely the words you are
thinking in. Grep the subject's terms across every document that carries decisions (the design homes, the
rejected alternatives, the scoped `CLAUDE.md` files) and the comments of the code the subject
touches, then `git log -G'<term>'` for the argument behind them. Say what you searched and what it returned. A
search that returned nothing is a finding worth one line in the commit message.

**If nothing holds it, carry on to §2.** If something does, do all of the following, in the same
change:

- **The Component is the incumbent's.** §3's test is not asked again. A reversal does not move a
  decision between Components. If the incumbent looks misfiled, that is a separate change with its
  own argument.
- **It earns durable text whatever §2 says.** The incumbent's head now states something false, so
  the minimum is rewriting it. §2's tests decide whether a _new_ decision earns an entry, not
  whether a false one may stand.
- **Rewrite the head in place** (§5), and give the new statement its own slug when it is a new
  statement. Keep the old slug only where it still names the same decision. The slug and the
  title must stay aligned with the full scope of the decision: the slug is often the only part a
  reader sees, in a citing document or in code, and the title is what a document outline shows. A
  slug or a title that misdescribes its decision misinforms every reader, or undermines the
  decision it names, so rename it even when that means rewriting every reference in the project. A
  rename is cheap, and the checker lists every reference it leaves dangling in the files it reads:
  Markdown and Rust source. A reference in a comment of another language is not read, so grep for
  the slug as well.
- **Move the incumbent into the Component's rejected alternatives** (§6) with its reason and a
  validity marker, stated as strongly as it was originally made, if it meets one of §6's tests.
- **Argue it under `knowledge-architect-design` first**, if it was not: a reversal contradicts a
  statement of the incumbent, the second case of §2's backstop.
- **Delete the tripwires guarding the reversed decision.** A tripwire whose decision is reversed
  goes outright. `knowledge-architect-issue-tracking` owns that movement.
- **Repair what pointed at the old behaviour**: skills, subagent definitions, scoped `CLAUDE.md`
  files, generated headers. Grep the behaviour's wording as well as the slug, since a pointer that
  describes the behaviour without naming it is the one a slug grep misses. A sentence about the
  past that named the old slug is rewritten to state the present, or removed, never retargeted and
  never turned into plain text, as `knowledge-architect-issue-tracking` says of a deleted entry.
- **Close the issue entry that asked the question**, in this commit rather than a later one, and
  rewrite whatever half of it survives rather than deleting the whole.

## 2. Does it earn a document entry at all?

Most implementation choices do not. A unit of work produces dozens of them, and a design home that
records dozens per unit of work stops being readable and stops being ranked.

**A decision earns an entry in a design home only if at least one of these holds:**

1. reversing it would change a type or a signature that **crosses the boundary of a separately
   built unit**: a crate, a package, a library, a module others import;
2. it **constrains work that has not been built**; or
3. **its argument turns on a reading of an external specification the project implements**: a
   standard, a protocol, a rule set.

Test 3 matters most in a project that implements a specification. A choice that turns on what the
specification means is expensive to get wrong and expensive to derive again, and it is visible: the
argument quotes or cites the specification.

Otherwise it belongs in an **inline comment at the code it explains, plus the commit message**.
That is not a lesser home: the comment is read by everyone who touches the code, and the commit
carries the argument.

**A decision that creates a design head, contradicts a statement of one, its argument included,
or extends one beyond what its title states, and was not argued under `knowledge-architect-design`,
goes back to that skill before its text is written.** This is the case of a decision met during
another task and settled there, by the owner's word or by the session's own choice. The design
skill's in-change path keeps the deliberation in the commit message, so the task needs no plan
document and no new session. For the third case, rewrite the head's title to state the addition as
well, and apply §5's test to it: if no title passes, the addition gets a head of its own. An
addition within what the title states, which contradicts nothing, is recorded directly, with the
owner's words quoted in the commit where they gave a ruling. A change that relocates or rewords
recorded decisions, a split of a head included, and adds or removes none, is not a decision: it
needs no design skill, and the routing and fidelity-of-relocation review axes judge that it adds or
removes none.

## 3. Which Component owns it

The project is partitioned into Components, which its manifest declares. The project's root is a
Component too.

1. Does the decision shape the layout of the project: which Components exist, and how they depend
   on each other? Then it belongs in the root Component's design home.
2. Does it shape how every Component is implemented: a design principle every Component follows?
   Then it belongs in the root Component's design home.
3. Does it shape how a single Component is implemented, or its own interface? Then it belongs in
   that Component's design home.

If it is a statement about what the project or a Component is _for_ rather than about how it is
built, it is a goal, not a decision. Goals are a register of their own, the goals home of each
Component, each entry a level-two heading carrying a slug. A decision that leans on one names it as
`goal@<component>@<slug>`.

If none of these fits, **ask the owner before writing it anywhere.**

## 4. Three homes, split by function

| what | where | why there |
| --- | --- | --- |
| the current design, and its **standing argument** | the Component's design home | it is rewritten in place, so it cannot go stale |
| that an alternative lost, and why | the Component's rejected alternatives | a reader about to propose it must meet it without knowing to look |
| the **deliberation**: what was weighed, in which order, what evidence was built, who ruled what | the spec or milestone document while it exists, then history; the commit message when there was no spec | failing to find it costs a re-derivation, not a wrong decision |

**The standing argument** is every premise whose failure would reopen the decision: the goal or
the decision it derives from, as references; the measurement it rests on; the fact that defeated
its nearest rival. **The test: if this premise turned false, would the decision have to be argued
again? If yes, it is in the head.** If no, it is deliberation.

**The deliberation is not copied into the head**, and, where a plan document carries it, not into
the harvest's commit message either.
A plan document is deleted when its work lands, per `knowledge-architect-planning`, and the commit
that deletes it cites it by its kind. A reader who needs the deliberation finds it there:

```sh
git log --diff-filter=D --name-only -- docs/plans/         # every deleted plan document, with its commit
git log -G'<slug>'                                         # every commit whose diff touches the slug
git show <commit>^:<path of the document>                  # the document as it stood before deletion
```

**`-G` matches any commit whose diff touches the slug. `-S` matches only a change in how many times
it occurs, so it misses every revision.**

When a decision was taken with no spec, on the in-change path of `knowledge-architect-design`, its
commit message carries the deliberation.

## 5. The current design

In the Component's design home. That is its `docs/design.md`, or, for a Component whose design has
outgrown one file, its `docs/design/` directory. In the directory shape the decision goes in the
subdocument owning its subject, never in the directory's `README.md`, which is the head and the
index. A new subdocument is linked from that index, `[title](file.md)`, the target relative to the
README, conventionally one bullet per subdocument. `{{command}} check` refuses one that is not.

Rewrite it **as if the design had always been so**. Present tense, no dates, no "formerly", no
account of the change. If you find yourself writing "we used to…", that sentence belongs in the
commit.

**A head carries intent, shape and the standing argument, not implementation.** What the project is
for, and how it is arranged in order to get there, belongs here. How a particular function does its
work belongs in a comment at that function. The test: **if changing a piece of code would force a
change to this document, it is design; if the document would be unaffected, it is a comment.**

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
a slug that names the decision, by the alignment rule of §1, and the text that keeps the
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
is read with care" is true of nearly any rival. A head that carries several decisions passes the test
for each one, or is split.

```markdown
### <The decision, stated as a sentence> `##<slug>`

<The shape, present tense. Then the standing argument: what it derives from, as references, and
the fact that defeated its nearest rival.>
```

A reference is one backticked span, `design@<component>@<slug>`, naming the Component that
defines it. It is live wherever it is prose, a fenced block included. An illustration that must not
resolve writes a placeholder in angle brackets.

**Name what the argument depends on.** The goal a constraint is derived from,
`goal@<component>@<slug>`: a constraint from a goal binds outright, where one from a decision is a
presumption, and the reference is what tells a reader which. A decision of another Component the
head depends on. **A reference is a claim that this head is revisited when that entry changes**,
so a reference whose entry's change would leave the head unaffected is not written. Never list what
cites this head: `{{command}} show design@<component>@<slug>` computes it.

A decision that relies on the checker of this workflow states that it relies on the checker
working as intended. It cannot reference the checker's own decisions: a reference resolves only
against the project that holds it.

The slug is an identifier. Code comments, tripwires, other documents and `git log -G` all cite it,
so **renaming one means rewriting every reference in the same change.** Grep before you rename.

## 6. Losing alternatives

In the Component's `docs/rejected-alternatives.md`.

**Most alternatives that lose earn no entry.** An entry is read by someone reasoning from first
principles who was not there, and it is reached by grep from anywhere in the tree, so it is a
description of a rejected design sitting in the same prose as the current one.

**Which threads of a discussion are candidates.** A thread's final state does not decide it. The
question is whether **a shape lost to an argument**.

| the thread | what happens to it |
| --- | --- |
| ruled out | judged by the tests below |
| withdrawn with a defeating reason | the same as ruled out: its own proposer gave it up on an argument |
| withdrawn with no defeating reason | the spec only. It carries no argument a later reader could test. |
| superseded, when the absorbing thread carries its shape whole | the spec only |
| superseded, when a distinct shape lost | judged by the tests below, as a ruled-out thread |
| a shape that lost, when the question produced no decision | the spec only. An entry names the decision it lost to, and there is none. If the question stays open, it is an issue under `knowledge-architect-issue-tracking`. |

**An alternative earns an entry only if at least one of these holds:**

1. it would change a signature crossing the boundary of a separately built unit, or a serialized
   format;
2. its reason rests on a reading of an external specification the project implements;
3. it was refuted by **evidence that cost work to obtain**: a measurement, a prototype, a survey of
   other projects;
4. there is still some doubt that the winning alternative will achieve every goal of the project or
   of its Component.

Test 3 is the load-bearing one. Proposing something again costs one discussion round, and that is
healthy. What decides an entry is whether the refutation can be derived again inside that round.
Reasoning derives again for free, so an alternative that lost to an argument any competent reader
reconstructs needs no entry. Evidence does not, so an alternative refuted by measurement does.

Test 4 gives future design sessions the alternatives that were not chosen, so they can start from
somewhere when the chosen one proves insufficient.

An alternative meeting none of those stays in the spec and the commit message, and nowhere else.

**A losing alternative is not recorded to make sure it is never proposed again.** It is recorded
so that the next time someone brings it to the table, the arguments that justified the rejection
are known, and it is possible to examine whether they still hold at the time of reopening.

**Write the entry so that it cannot be read as the current design.** Name the alternative and the
reason it lost. Do not describe how it would work: an entry detailed enough to build from is the one
a grep turns into a specification.

One entry per alternative, keyed by **the alternative**, not by date. Flat, unordered, and **edited
in place**.

```markdown
**<The alternative, named in a few words>** — lost to `design@<component>@<slug>`. `live`.
<The reason it lost, checkable without leaving the entry.>
```

Each entry carries: what the alternative was, the slug it lost to, a **validity marker**, and the
reason.

- `live`: the reason still holds.
- `void`: the reason no longer holds. **Mark it; never delete it.** The alternative may still be
  wrong, but _this_ argument no longer supports that, and the marker prevents reopening a settled
  question on a dead argument.

**`live` records that the reason held against the arguments seen, not that the question is shut.**
A new argument or new evidence reopens any entry here at any time, and that is an ordinary move
rather than a transgression.
The record decides what counts as new: an alternative is argued-and-lost only where a recorded
reason covers the discriminating fact. A reworded proposal that defeats or evades the recorded
reason is new by definition.

**Every entry states a reason a reader can check without leaving the entry, or points at where the
reason is.** _"Refuted by measurement"_ satisfies neither: it asserts that a checkable reason exists
somewhere and does not say where.

**A reversal moves the old winner into this file** with the reason it lost, if it meets one of the
four tests.

**A rejected alternative that is reopened, chosen and implemented moves out of the file.** The file
must not describe as rejected a design the project now has.

## 7. Tripwires from a premortem

A design discussion that runs a premortem ends with it, and the owner rules on which of its surviving causes
become tripwires. **A tripwire is written at harvest, with the decision it guards, and only on the
owner's word.** It goes in the tripwires home of the Component that owns the guarded decision, so
the decision's head exists before the tripwire that names it. Its shape and its lifecycle are
`knowledge-architect-issue-tracking`.

## 8. Before you finish

```sh
{{command}} check
```

Every reference you wrote must resolve.

Then re-read what you wrote against the head you replaced: **rewriting argued text is where
fidelity gets lost.** If you cannot restate a losing alternative as strongly as it was written, you
have not understood it well enough to move it.
