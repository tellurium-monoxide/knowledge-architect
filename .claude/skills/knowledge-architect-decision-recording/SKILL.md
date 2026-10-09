---
name: knowledge-architect-decision-recording
description: MUST use before writing into a design home, whatever the activity, and when a design decision has been made or reversed — a choice about how the project or one of its Components is built, including anything a consumer of it may depend on — in order to decide whether it earns durable text at all (most implementation choices do not) and where that text lands. Covers when recording happens, the reversal check that comes before everything else, which Component owns it, the split between the design home, the rejected alternatives and history, where a head is written and how a slug is renamed, and the tripwires a premortem produces; what earns a head and how a head is shaped are the primer's section on design heads.
---

# Recording a decision

**Read `primer@design-heads` again, whole, before writing or judging a head.** The primer was
loaded when the session started, and a recording comes late in it.

Scope: turning a decision about **what we build** into durable text. Whether it earns durable text
at all, when it is written, which Component owns it, what goes in the design home, what stays in
history, and what the premortem leaves behind.

Not covered here: **having** the discussion (`skill@knowledge-architect-design`);
**writing the spec or the milestone document** that carries a decision until its work lands
(`skill@knowledge-architect-planning`); **parking something undecided**, and writing a tripwire's
entry (`skill@knowledge-architect-issue-tracking`). A decision about the agent configuration itself
is both this and `skill@knowledge-architect-agent-configuration`: that skill owns how to write the
configuration, this one owns where the argument lands.

## When recording happens `##when-recording-happens`

**A decision is recorded when the work that implements it lands, not when it is decided.** A
design home holds built intent: the design as built and its reasons, against which the code is
checked. A head written before the code exists would report a defect in code nobody has written.
While the work is open, the decision is unbuilt intent, and the spec or the milestone document is
the only place it exists, on the main branch or on the work's. Writing it into the design homes from there is
the **harvest**, and it happens in the change that lands the work.

A decision that no work implements, such as a policy, and that is not part of any spec, is
recorded when it is made: there is no implementing change to wait for. The second entry test of
`primer@design-heads` admits it, as a decision with no site of its own.

## Does it reverse something already recorded? `##reversal-check`

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

**If nothing holds it, carry on to the entry tests of `primer@design-heads`.** If something does, do all of the following, in the same
change:

- **The Component is the incumbent's.** The test of `skill@knowledge-architect-decision-recording@owning-component` is not asked again. A reversal does not move a
  decision between Components. If the incumbent looks misfiled, that is a separate change with its
  own argument.
- **It earns durable text whatever the entry tests of `primer@design-heads` say.** The
  incumbent's head now states something false, so the minimum is rewriting it. The entry tests
  decide whether a _new_ decision earns an entry, not whether a false one may stand.
- **Rewrite the head in place** (`skill@knowledge-architect-decision-recording@current-design`), and give the new statement its own slug when it is a new
  statement. Keep the old slug only where it still names the same decision. The slug and the
  title stay aligned with the full scope of the decision, per `primer@design-heads`, so rename it
  even when that means rewriting every reference in the project. A rename is cheap, and the checker lists every reference it leaves dangling in the files it reads:
  Markdown and Rust source. A reference in a comment of another language is not read, so grep for
  the slug as well.
- **Move the incumbent into the Component's rejected alternatives** (`skill@knowledge-architect-decision-recording@losing-alternatives`) with its reason and a
  validity marker, stated as strongly as it was originally made, if it meets one of the tests of `skill@knowledge-architect-decision-recording@losing-alternatives`.
- **Argue it under `skill@knowledge-architect-design` first**, if it was not: a reversal contradicts
  a statement of the incumbent, the second case of the backstop of `primer@design-heads`.
- **Delete the tripwires guarding the reversed decision.** A tripwire whose decision is reversed
  goes outright. `skill@knowledge-architect-issue-tracking` owns that movement.
- **Repair what pointed at the old behaviour**: skills, subagent definitions, scoped `CLAUDE.md`
  files, generated headers. Grep the behaviour's wording as well as the slug, since a pointer that
  describes the behaviour without naming it is the one a slug grep misses. A sentence about the
  past that named the old slug is rewritten to state the present, or removed, never retargeted and
  never turned into plain text, as `skill@knowledge-architect-issue-tracking` says of a deleted
  entry; a verbatim quotation of the owner is left as it is, with a reference to the current entry
  beside it.
- **Close the issue entry that asked the question**, in this commit rather than a later one, and
  rewrite whatever half of it survives rather than deleting the whole.

## Which Component owns it `##owning-component`

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

## A decision approved with no argument `##unargued-approval`

**A decision approved with no argument is argued before it is recorded.** Before a head is written
from an approval that no argument for the decision, no rival and no cost stand behind, such as a
default approved in a batch, search for the arguments for and against it, its costs and its rivals,
and record what the search finds. When the search leaves the rivals equal, put the fork to the
owner as a tie. The owner's choice is then the head's ground, as a decision that came from the
owner, per `primer@design-heads`.

## Three homes, split by function `##three-homes`

| what | where | why there |
| --- | --- | --- |
| the current design, and its **standing argument** | the Component's design home | it is rewritten in place, so it cannot go stale |
| that an alternative lost, and why | the Component's rejected alternatives | a reader about to propose it must meet it without knowing to look |
| the **deliberation**: what was weighed, in which order, what evidence was built, who ruled what | the spec or milestone document while it exists, then history; the commit message when there was no spec | failing to find it costs a re-derivation, not a wrong decision |

**The standing argument**, and what stays out of the head, are `primer@design-heads`.

**The deliberation is not copied into the head**, and, where a plan document carries it, not into
the harvest's commit message either.
A plan document is deleted when its work lands, per `skill@knowledge-architect-planning`, and the
commit that deletes it cites it by its kind. A reader who needs the deliberation finds it there:

```sh
git log --diff-filter=D --name-only -- docs/plans/         # every deleted plan document, with its commit
git log -G'<slug>'                                         # every commit whose diff touches the slug
git show <commit>^:<path of the document>                  # the document as it stood before deletion
```

**`-G` matches any commit whose diff touches the slug. `-S` matches only a change in how many times
it occurs, so it misses every revision.**

When a decision was taken with no spec, on the in-change path of `skill@knowledge-architect-design`,
its commit message carries the deliberation.

## The current design `##current-design`

In the Component's design home. That is its `path@*@docs/design.md`, or, for a Component whose design has
outgrown one file, its `path@*@docs/design/` directory. In the directory shape the decision goes in the
subdocument owning its subject, never in the directory's `README.md`, which is the head and the
index. A new subdocument is linked from that index, `[title](file.md)`, the target relative to the
README, conventionally one bullet per subdocument. `cargo klarch check` refuses one that is not.

**What a head holds and how it is shaped is `primer@design-heads`**: read it whole before writing
a head. The template:

```markdown
### <The decision, stated as a sentence> `##<slug>`

<The shape, present tense. Then the standing argument, per the primer's section on design heads.>
```

The slug is an identifier. Code comments, tripwires, other documents and `git log -G` all cite it,
so **renaming one means rewriting every reference in the same change.** Grep before you rename.

## Losing alternatives `##losing-alternatives`

In the Component's `path@*@docs/rejected-alternatives.md`.

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
| a shape that lost, when the question produced no decision | the spec only. An entry names the decision it lost to, and there is none. If the question stays open, it is an issue under `skill@knowledge-architect-issue-tracking`. |

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

## Tripwires from a premortem `##premortem-tripwires`

A design discussion that runs a premortem ends with it, and the owner rules on which of its surviving causes
become tripwires, each by the label, `T<n>`, it was put to the owner under. **A tripwire is written at harvest, with the decision it guards, and only on the
owner's word.** It goes in the tripwires home of the Component that owns the guarded decision, so
the decision's head exists before the tripwire that names it. Its shape and its lifecycle are
`skill@knowledge-architect-issue-tracking`.

## Before you finish `##before-you-finish`

```sh
cargo klarch check
```

Every reference you wrote must resolve.

Then, in the commit message, say what you searched for the incumbent and what it returned (`skill@knowledge-architect-decision-recording@reversal-check`),
and re-read each head you wrote: present tense, as if the design had always been so (`primer@design-heads`).

Then apply the entry tests of `primer@design-heads` again to each decision of the change, now that every text of it is
written: a site the change itself wrote counts, a skill or a restatement of the project's own
included. A decision judged while its texts were still being written misses the second site the
same change adds.

Then re-read what you wrote against the head you replaced: **rewriting argued text is where
fidelity gets lost.** If you cannot restate a losing alternative as strongly as it was written, you
have not understood it well enough to move it.
