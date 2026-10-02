---
name: knowledge-architect-planning
description: MUST use when a design discussion has converged and its work needs a spec or a milestone, in the same session as the convergence; whenever a step of a milestone is about to be implemented or lands; and when the work of a spec lands. Covers the plan document vocabulary (spec, milestone, plans directory), choosing between a spec and a milestone, the file layout and the fixed sections, item identifiers, cutting the steps, acceptance criteria, the per-step design audit, the reviews of a plan document, the harvest, and the deletion that ends a plan document's life.
---

# Planning

Scope: turning the converged design of a discussion into a plan document that a later session
implements, then carrying that document through each landing until it leaves the repository. One
activity. It begins in the session where the discussion converged, and it re-enters at each step's
audit and at each landing.

**Not covered here**, each named where it lives:

- **The design discussion** that produces the converged design:
  `knowledge-architect-design`. It ends at convergence, the premortem and the
  owner's rulings on tripwires, and hands off to this skill.
- **Writing the code** of a step, its claims and its tests: the project's own development
  procedure. This workflow installs none.
- **Dispatching the reviewers**: `knowledge-architect-review`.
- **Recording** what a landing establishes. Recording is not done once at the end of the
  discussion: it is done at each landing, as the harvest of §7. The procedure is
  `knowledge-architect-decision-recording` for the decisions and the losing alternatives, and
  `knowledge-architect-issue-tracking` for the tripwires and the issues.

## Terms

| word | meaning |
| --- | --- |
| **plan document** | any document in the plans directory except its `README.md`. Temporary: it leaves when its work lands |
| **plans directory** | the one directory where a project keeps its plan documents. Its path is the project's choice, stated in its root `CLAUDE.md` |
| **spec** | the plan document of work done in one branch and one PR: the converged design and a concise implementation sequence |
| **milestone** | work across several PRs, with design sessions between them. Its plan documents are one directory: the **milestone document**, its `README.md`, and one spec per step |
| **converged** | every proposal of the discussion is closed, and no binding criterion is unmet |
| **thread** | one proposal of the discussion, carrying a state; an approved thread is a decision |
| **criterion** | what proposals were judged against: **binding** rules a proposal out, **weighed** makes failing it a cost the owner rules on |
| **acceptance criterion** | a check on a recorded decision that only the work's built code can apply |
| **material** | a finding made after a thread closed that defeats a reason the closure gave, a premise it rested on, or a criterion it claimed to meet. A finding that defeats none of these is not material. A material finding is presented once, with a default named, and the thread stays closed until the owner's word. The full protocol is `knowledge-architect-design`, under Decision authority |
| **audit** | the reading of a step's entry against the tree and the design homes before the step is implemented |
| **harvest** | the recording of what a landing established into the project's durable homes |

**The owner** is the person whose word closes a thread. Every ruling in this activity is theirs.
The word "plan" alone never names a document: say spec, milestone document, or plan document.

## 1. When this skill starts

**In the session where the discussion converged, before that session ends.** The discussion's
ledger, every thread with its state and its argument, lives only in that conversation. A plan
document written from memory in a later session is written from a summary, and a summary loses
exactly the losing arguments and the conditions of each closure.

A problem that arrived bounded, with a clear requirement and no open design question, has no
converged discussion to plan from. This skill does not apply to it.

## 2. A spec or a milestone

| the work | its plan document |
| --- | --- |
| one branch, one PR | a spec |
| several PRs, with design sessions between them | a milestone |

Propose which one, with the reason, and let the owner rule. A spec whose work turns out to need
several PRs becomes a milestone: the spec becomes the milestone document, and the step specs are
cut from its implementation sequence.

**Work that is known but not designed is not a plan document.** It is a `todo` issue, or a
`deferred` one if an event gates it (the default kinds; a project that declares its own kinds uses
the nearest), in the owning anchor's issue register, with its leads in the
entry. The plan document that schedules it closes that issue in the commit that adds the plan
document, so the work is listed in one place at a time. There is no roadmap file and no record of
landed work: `{{command}} issues --kind todo` lists what is planned and undesigned, and history
lists what landed.

## 3. Layout

```
<plans directory>/
├─ <subject>.md            a spec
└─ <subject>/              a milestone
   ├─ README.md            the milestone document
   └─ <step>.md            the spec of one step
```

- The milestone document links each step's spec as a navigation row, `[<step title>](<step>.md)`.
  The checker resolves a relative link only in a `README.md` or an `index.md`, which is why the head
  is a README.
- **The plans directory holds plan documents and nothing else**, except one `README.md` that says
  what the directory holds. It keeps the directory, and the path the root `CLAUDE.md` names for it,
  in the tree while no plan is open. A document with another lifetime,
  such as a record of how far a subject is implemented or a survey that outlives its work, has its
  own home. If none fits, ask the owner before writing it anywhere.
- A plan document carries **no slug anchor**: a slug is a definition other documents may cite, and
  a plan document's items must not be cited from outside the plans directory (§4).

## 4. What a spec holds

**Written for a session that did not witness the discussion.** That is the standard every section
is held to, and §8's reviews check it.

**The sections are fixed, with these titles, in this order.** The titles are what a later structure
for plan documents reads. A section with nothing to hold says
so in one line rather than being omitted, so a reader can tell an empty section from a missing one.

| section | holds |
| --- | --- |
| status and audience | what the document is for; that it leaves when its work lands; that where it and a design home disagree, the design home wins; that every name it uses is defined in it or exists in the code; that where the owner's word is needed and the owner is absent, the work does not proceed on that point |
| how a step is worked | in a milestone document: §7 of this skill, restated, with a pointer to this skill as its home. In a spec: one line naming this skill |
| names | every project shorthand the document uses, expanded to the file, function or command it names |
| what the work is | what exists today at each site the work touches; what is outside the work and why, each exclusion naming the work or the decision that owns it |
| what is already decided | the recorded decisions the design rests on and does not argue again, as references; and each recorded decision the work reverses or rewrites, with every tripwire and issue that `{{command}} show` lists as referencing it, and the harvest that judges each |
| criteria | criterion, kind, source, satisfaction |
| threads | every thread with its identifier, its final state and its resolution, a column naming the section that carries its shape, and a column naming the durable home that will harvest it |
| new names, in one place | every new name the design uses (a type, a function, a field, an event, a bound, a counter) in one fenced block with the file it goes in; a name that exists in the code is listed as existing |
| decided design | one subsection per approved thread: the shape, the argument, the nearest rival and the fact that defeated it |
| mapping tables | one table per total function the code will need, over its whole domain: which existing thing becomes which new thing. Empty when the work needs none |
| losing alternatives | every ruled-out thread, every thread withdrawn with its defeating reason, and every superseded thread under the thread that absorbed it, each with the thread it lost to and the fact that decided it |
| readings | where the work reads an external specification the project implements: each reading it makes, and where it is recorded. Empty for work that reads none |
| premortem | each cause, the thread it stresses, and its verdict: survives into a named claim, criterion or guard; converted into a named clause of the design; becomes a tripwire, on the owner's word; or fired and the thread reopened |
| acceptance criteria | §6 |
| implementation sequence | the steps, §5. Concise: what each step builds and what it fails alone on |
| order rationale | one sentence per pair of adjacent steps |
| defaults awaiting the owner | each default a reviewer's finding or the author's judgement produced, with the thread it bears on, until the owner rules |
| harvest | what lands where and when: one row per step, and one for the document itself |
| later consequences | what each later piece of work adds or replaces, so a later reader knows what was deliberately left |

**The milestone document holds the same sections** for the whole milestone, and its implementation
sequence lists the steps, each linked to its spec. **A step's spec holds** the step's entry (§5),
and, when the step had a design session of its own, that session's design in the sections above.

**Content rules.**

- **No untested code snippet is presented as authority.** An implementer forces an authoritative
  snippet into the code at any cost and copies its comments verbatim. A snippet in a plan document
  is labelled as an illustration of a shape. A detailed step-by-step implementation plan, if one is
  ever written for a less capable implementer, covers a bounded amount of work and opens by saying
  that its content rests on assumptions and may be wrong.
- **A statement about existing code is a claim, not a hypothesis.** It is checked against the tree
  before it is written, exactly as in a design head. Only the unbuilt half of a plan document is a
  bet.
- **A correction is applied in place**, with no correction log and no issue: the document leaves
  when its work lands, so a log would be history in a file that keeps none. The commit says what
  was corrected.

**Identifiers.** Every thread, step and acceptance criterion carries an identifier in the grammar
`[a-z0-9]+(-[a-z0-9]+)*`. A thread keeps the slug the discussion minted. An identifier is written in
plain text with a hash sign before it, never in backticks: the checker reports a backticked hash
and identifier as a retired reference form. Nothing outside the plans directory cites an item of a plan document: a
design head that cited one would dangle when the document leaves. A `path` reference to a whole
plan document is allowed, and its dangling at deletion lists the texts that depended on it.

The fixed layout, the fixed titles and the identifiers are the shape a later structure for plan
documents can read without rewriting them.

## 5. Cutting the steps

**Each step must be able to fail without another step's new machinery entangled in the
failure.** Every other test here serves that one.

- **The first step lands the mechanism empty and measures it.** No content that uses it, no
  observable change against the instruments the Component already has, and its cost measured against a recorded baseline where the Component has one. Every later failure is
  then a failure of the content and not of the mechanism.
- **Order by dependency.** A step that needs a mechanism comes after the step that proves it.
- **A step touches at most one subsystem it did not build.** Two subsystems in one step are two
  failure surfaces with one name.
- **The last step consumes**: the measurements, the report, the harvest.
- **One branch and one merge per step**, each merged before the next step begins, and reviewed
  before the merge.
- **A milestone document whose first step changes what the project's gates check lands in a merge
  of its own, before that step.** On the step's branch, the gates as that step changes them would
  judge the commit that added the document, whose tree predates the change.

A step's entry carries, in this order:

- **Builds**: what the step lands, by name.
- **Claims**: each with the test that could refute it, and how the test is shown to fail against a
  wrong implementation. A claim whose instrument does not exist names the instrument the step
  lands first.
- **Fixtures**, where the Component drives its tests with authored content: each written in the
  vocabulary that exists, or naming the addition it makes. A fixture that needs something the
  vocabulary cannot express is a finding against the design, not a note for the implementer.
- **Audit subjects**: the files, decisions and specification passages the step's audit is already
  known to have to read.
- **Fails alone on**: the one or two observations that tell this step's failure from every other
  step's.
- **Premises that expire**: where the step relies on something a later step or change ends, what
  ends it, and the criterion or guard that watches it.

## 6. Acceptance criteria

An acceptance criterion is a bet about a decision the work is built on, where a claim is a bet about
the work itself: firing a criterion reopens the decision, not the step. It lives in the plan
document of the work that can judge it, in its acceptance criteria section, and nowhere else. Each
names:

- the decision it guards, as a reference;
- the step that judges it;
- the observable that fires it;
- the response.

**The result a scheduled review is expected to give is not a criterion.** Passing the reviews that
every plan document and every step owes is the baseline. A criterion names an observable specific to
the decision it guards; listing "the reviews pass" in every plan document would be noise, and would
stand in for the specific criterion that is harder to find.

A number in a criterion is a threshold the owner sets. Until the owner has, it is written as a
default marked as the owner's to reset.

- **At each landing**, the landing commit reports on every criterion judged there, one line each:
  the decision guarded, fired or not, the evidence, the response taken.
- **A criterion that fires** leaves the document at once, as an issue entry or a reopened decision,
  under `knowledge-architect-issue-tracking`.
- **When the document leaves**, its last landing commit reports on every criterion once more. One
  that did not fire and recurs at later work is proposed to the owner as a tripwire, and written on
  the owner's word in the tripwires home of the Component that owns the guarded decision, naming
  the harvested head, in the shape `knowledge-architect-issue-tracking` gives. One that is
  spent, or that the owner declines, is deleted with the document.

## 7. Working a step

The milestone document restates this procedure with a pointer to this skill, so a cold session
finds it there.

1. **Ground**: the Component's `CLAUDE.md`, its design home, its rejected alternatives, its open
   issues, its tripwires; then the milestone document entire, then the step's spec.
2. **The design audit.** Read the step's entry and every decided shape it depends on against the
   code as it stands and against the design homes. List every gap: a shape the code refutes, a
   passage of a specification the entry did not read, a name the entry uses that the code does not
   have, a consequence the entry did not see. Sort each gap:
   - **Applied in place.** The gap has one answer the document's decisions already imply, or is a
     choice among shapes the document rules out all but one of. Write the answer into the step's
     spec under a heading saying the audit's findings are applied as the step's binding shape, each
     finding stating the gap, the answer and the decision it follows from. Commit the amended spec
     alone, with a subject of the shape `The <step> design audit, applied in place: <n> gaps, none
     reopening a discussion`. Earlier audits are found with `git log --grep='design audit'`.
   - **Load-bearing.** The gap is material, or is a choice between two shapes neither of which the
     document rules out, or needs a ruling the document marks as the owner's. Record it in the
     step's spec as open at the audit, with the discriminating fact, stop the step, and open a
     design session with the owner under `knowledge-architect-design`. Its
     converged design goes into the step's spec, in the sections of §4, and owes §8's reviews. The
     step resumes from it.
3. **Claims, tests, implementation, gates, commit**, per the project's development procedure. The
   commit names how each claim's test was shown to fail against a wrong implementation, and says
   of any claim whose test cannot yet do so why not.
4. **Review before the merge**, per `knowledge-architect-review`. A repair is a
   further commit. A finding not repaired becomes an issue entry.
5. **The report**: the landing commit reports on each acceptance criterion judged at this step (§6).
6. **The harvest**, per the step's rows in the harvest section: the decisions and the losing
   alternatives under `knowledge-architect-decision-recording`, then the tripwires and the issues
   under `knowledge-architect-issue-tracking`. A tripwire names the head that harvested its
   decision, so the head is written first. Where a design home is a directory, a new subdocument is
   linked from its README.
7. **The step's spec leaves** in the commit that completes its harvest, as in §9. What crosses steps stays in the
   milestone document, amended in place where the landing changed it.

## 8. Reviews of a plan document

A plan document is committed on its branch first, and that commit is what the reviewers read; each
repair is a further commit. It is read again after a revision that changes a decided shape (an
audit applied in place is not one). Its reviewers are fresh, and did not witness the
discussion. **Fresh, never a fork**: a fork inherits the discussion and reads the document as its
author. Dispatch them through `knowledge-architect-review`, with the invariants that
skill lists, the blind brief included:

- `knowledge-architect-cold-implementer-reviewer` reads the document as the implementer of its first
  step and reports every place where it cannot act: undefined names, shapes without enough detail
  to write, procedure gaps, ambiguities, and what it would have to reconstruct from a conversation
  it did not see. It also applies the readiness checks below.
- `knowledge-architect-code-claims-reviewer` verifies every statement the document makes about the
  code as it stands, and reports each as confirmed, wrong or imprecise, with the evidence.
- `knowledge-architect-transcript-conformity-reviewer` reads the discussion's transcript and checks
  that the document records what was decided, no wider and no narrower. Dispatch it whenever the transcript is
  available. When it is not, say so, and why, in the commit that adds the document.

**What their findings become.** Check each finding against the tree, or against the transcript,
before acting on it. A material finding is answered with a default, written into the sections it
touches, and listed under the defaults awaiting the owner, with the thread it bears on. The owner
rules on each at the first audit, or at once if present, and a ruled default leaves the list. A finding
that is a gap with one answer is applied in place. A finding that is wrong is dropped, with the
reproduction that showed it wrong kept in the commit message. A scope change, a clause that widens or narrows
a ruling or adds an obligation to it, is put to the owner, listed under the defaults awaiting the
owner. An agent's addition, detail the author added inside the scope of an owner's ruling, is kept
and listed in the commit message as the agent's, so the owner can contest it; it is removed only on
the owner's word.

**The readiness checks**, applied by the author before dispatch and by the cold implementer after.
This list is their one home; the reviewer reads it here.

- every name the document uses is defined in the names sections or exists in the code;
- every total function the code will need is a mapping table;
- every fixture is expressible in the vocabulary that exists, or names its addition;
- a milestone document carries the procedure of §7;
- every number is a measurement with its instrument, or a default marked as the owner's;
- every premise a later step or change ends is stated with its guard;
- every statement about the code names the file and the function, and its truth is the code-claims
  reviewer's to establish;
- every acceptance criterion names the decision it guards, the step judging it, the observable that
  fires it and the response;
- every thread maps to a section and to a harvest home;
- every tripwire and issue referencing a decision the work reverses or rewrites is named, with the
  harvest that judges it;
- every section of §4 is present, and an empty one says so.

## 9. When a plan document leaves

**A spec is deleted in the commit that completes its last harvest, and that commit's message names
its path** as a `path` reference, which resolves against the commit's parent. A milestone's step
spec leaves when its step lands; the milestone document leaves with the last step. Before deleting:

- every row of the harvest section is done;
- every acceptance criterion has been reported on, and has become a tripwire or left (§6);
- what the document established is in the design homes and the registers; what stayed a guess
  leaves with it.

A reader who needs the deliberation later finds it in history:

```sh
git log --diff-filter=D --name-only -- <plans directory>   # every deleted plan document, with its commit
git show <commit>^:<path of the document>                   # the document as it stood before deletion
```
