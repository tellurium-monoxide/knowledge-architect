# Analysis of 2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow

- **Received file:** 2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md,
  the workflow's side of this repository's own retrospective of the agent-configuration-entities
  milestone. The project's side is analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities.md`.
- **Version it used:** built from this repository's source, main at v0.5.0-35-g8c0bf84.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1. This file holds the clusters K2, K4 and K5.
- **Standing entries that bear on it:**
  - `issue@core@tooling-for-project-skills` records C1.
  - `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check` records C6.
  - `tripwire@agent-skills@plain-text-pointer-found` is W8's subject.
  - `tripwire@core@candidate-rule-silence` watches spans meant as references that the check reports
    nothing on. K4's and K5's spans are not backticked, or are quotations, so they are outside its
    firing clause.
  - `tripwire@core@item-of-another-plan-named` guards the scope that makes C3 a plain form; see
    its section in
    `path@knowledge-architect@docs/retrospective-reports/2026-10-09-thaum-slice-cards-klarch-workflow.md`.
  - `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` covers part of K4: a plain
    name that dangles once its entry is gone.
  - `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` is adjacent to K2: it is
    the checker's side of earlier commits failing under the tip's checker.
  - `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` is adjacent to W7: it concerns
    shipped text that cites an entry, not restatements that cite none.
  - `tripwire@agent-skills@expectation-set-closes-a-contradiction`: K2 is a contradiction of two
    installed texts, and is not closed by an expectation set.

## W1 and W2, with P5 of 2026-10-08-knowledge-architect-agent-configuration-entities: the design audit's commit, and a step that cannot pass alone, against the per-commit gate (cluster K2)

- **Says.** The slice's first code commit added a check that every earlier tree fails, and the
  project judges every commit with the tip's checker. The audit commit "alone" could not pass, so
  the audit was folded into the code commit, its record dropped from the message, and
  `git log --grep='design audit'` found nothing until a reviewer had the message reworded. In two
  slices, step 1 (the mechanism) and step 2 (its content) could not be separate commits either. The
  report proposes one sentence in the planning skill: where every commit is judged by the tip's
  checker, an audit or a step whose tree fails the new check lands in the commit that makes it pass,
  with its record in that commit's message.
- **Still applies.** Yes, for the audit; partly, for the steps.
  - The audit: `path@agent-skills@content/skills/planning/SKILL.md`:383-386, "Commit the amended
    documents alone, with a subject that names the slice's design audit in those words, and a
    message listing each finding … Earlier audits are found with `git log --grep='design audit'`."
    c744b93 changed the subject's shape only. `primer@branch-commits` says that a change setting what
    the tip's checker requires "goes in the branch's first commit … or the branch is squashed to
    one commit before its review". An audit commit before that change fails the gate; squashed, it
    is no commit of its own. The planning skill's sentence on the per-commit gate, :466, covers the
    plan document only: "One whose work's first commit changes what the project's per-commit gate
    checks is merged before that work".
  - The steps: :264-280 already says, since 37dc8c6, before the report, "A step is no unit of the
    history: how many commits it takes … is the implementing session's to judge." So the report's
    premise, that the tests assume one commit per step, was false of the text. Still true: "The
    first step lands the mechanism empty", and nothing links step-cutting to
    `primer@branch-commits`.
  - P5's site: 6ee997d, "[audit] Pass 4 of the agentic-workflow axis…", removed the root's
    sentence "a step whose intermediate trees cannot pass lands as one commit" in favour of
    `primer@branch-commits`, which every session reads.
- **Recorded.** No issue, no rejected alternative.
- **Kind.** For the audit, two installed instructions leave no move that satisfies both, in a
  project whose work changes what the per-commit gate checks: "alone", and the primer's rule. This
  is in scope whatever else holds, per `tripwire@agent-skills@expectation-set-closes-a-contradiction`.
  For the steps, a defect of delivery: the rule exists in the primer, and the step-cutting text,
  read when a spec is written, does not point to it.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`: the audit's
  record is lost from the lookup the skill itself names. Owned by agent-skills, the planning skill.
  It reverses nothing: it states how the audit's "alone" yields to `primer@branch-commits`.
- **Better fix.** Rivals: a change in how such a project cuts its steps only, the report's
  project-side reading, which leaves the audit's contradiction; dropping "alone" entirely, which
  loses the separate audit commit in the common case where the gate does not change. The fix, one
  sentence at each site: the audit's commit is its own, "unless the slice's first change sets what
  the per-commit gate requires, per `primer@branch-commits`: the audit's documents then land in
  that commit, and its message carries the audit's subject and its list"; and in step-cutting, "a
  step whose trees cannot pass the tip's checker alone lands with the step that makes them pass,
  per `primer@branch-commits`".
- **Route.** A text edit whose design is settled, at the planning skill's source.
- **Proposal.** Handle now, the two sentences.

- **Default:** handle now.
- **Outcome:** approved.

## W3: a default awaiting the owner was written into a head, then removed, then restored

- **Says.** A reviewer's material finding went to the owner with a default. The session proceeded
  on the default, per `primer@owner-word-premise`, and wrote it into a design head. The
  decision-record reviewer removed the sentence, since it stood on no ruling. The owner approved the
  default, and the sentence was restored: one review round and two commits. The report reads the two
  texts as leaving no move, and proposes that a default awaiting the owner is not written into a
  head until the owner's word.
- **Still applies.** Both texts stand. `path@agent-skills@content/PRIMER.md`:350, "**A head written
  from a decision the owner approved is read against that approval before it is written.**" The
  planning skill, `path@agent-skills@content/skills/planning/SKILL.md`:357-358: "A default the plan
  documents list as awaiting the owner is built as they write it until the owner rules on it." No
  installed text says whether a default awaiting the owner reaches a head.
- **Recorded.** No.
- **Kind.** A defect of the text, unclear, rather than a contradiction: a move satisfies both,
  building the default while keeping it out of the head until the ruling. The texts do not name it.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`. Owned by agent-skills. The
  sentence belongs where a head is written: the decision-recording skill, which a session loads
  before any write into a design home, per its description.
- **Better fix.** The report's proposal, placed at the primer, competes for the primer's room
  under `design@agent-skills@primer-limit`; the decision-recording skill is read at the moment of
  writing a head, so it delivers the sentence where it is needed. The rival "no change" spends a
  review round each time.
- **Route.** A text edit whose design is settled, at the decision-recording skill's source.
- **Proposal.** Handle now: "A default awaiting the owner is built as written, and enters a design
  head once the owner rules on it."

- **Default:** handle now.
- **Outcome:** approved.

## W4: whether a title edit is a rewording, for the decision-record re-review

- **Says.** Repairs narrowed or widened head titles; each round sent the decision-record axis
  again, three rounds, each finding something in the previous round's title edits. The report
  proposes stating that a title change aligning a title with its body is a rewording.
- **Still applies.** Yes. `path@agent-skills@content/skills/review/SKILL.md`:145-150: "**A repair
  that adds, removes or reverses a design head is reviewed by the decision-record axis** … Any
  other repair, a rewording or an argument, a mention or a reference added inside an existing head
  included, is reviewed by no axis again". A title edit is not named. The primer's "A head that a
  change touches is brought to these rules in that change: its title to the rule it argues"
  (:361-362) makes such edits ordinary.
- **Recorded.** No.
- **Kind.** A defect of the text, unclear. One case, three rounds.
- **Scope.** Owned by agent-skills, the review skill. The line it draws follows the primer: a
  title brought to what its body already argues decides nothing new; a title that widens or
  narrows what the head decides changes the decision, and goes to the design skill by the primer's
  backstop anyway.
- **Better fix.** The report's wording ("only aligns a title with its body") is the case; the rule
  that admits it is the primer's distinction between a rewording and a change of decision. No
  change is the rival; it cost two extra rounds here.
- **Route.** A text edit whose design is settled, at the review skill's source.
- **Proposal.** Handle now: the sentence on other repairs gains "a title brought to what the
  head's body already argues included".

- **Default:** handle now.
- **Outcome:** approved.

## W5: the outcome of each returned standing entry has no home

Cluster K1, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence-klarch-workflow.md`.

- **Default:** handle now, with K1.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## W6: the searcher's sort order

Analysed with W1 of 2026-10-08-thaum-roadmap-revision-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-thaum-roadmap-revision-klarch-workflow.md`.

- **Default:** no change, already repaired.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## W7: the installed agents restate checker rules, and `show` on the head does not list them

- **Says.** The routing reviewer restated a rule of the checker; a new check made it false; `show`
  on the changed head did not list the agent, and a grep for the head's wording missed it. A
  reviewer found it one round later. The report proposes a commented citation in the shipped
  source that the build removes.
- **Still applies.** Partly repaired.
  - The mechanism exists, and existed before the report: `design@agent-skills@shipped-text-line-comments`,
    from 249f145. A `%%` line in the shipped source may cite a head, and the build removes it. It is
    used for a checker rule once, at `path@agent-skills@content/skills/agent-configuration/SKILL.md`:89.
    No file under `path@agent-skills@content/agents/` holds one, though several restate checker
    rules, the routing reviewer's "What the checks cannot see" among them
    (`path@agent-skills@content/agents/routing-reviewer.md`:55-62).
  - The checker-rules lens of the agentic-workflow audit, added by 29551db, reads the corpus
    against the checker's documentation (`path@agent-skills@content/agents/workflow-auditor.md`:106-108),
    and this repository adds the core's design home to it. It catches drift at an audit, not when
    the head changes.
- **Recorded.** No issue. `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` is about
  a citation that resolves nowhere in an adopting project, which a `%%` line avoids by removal.
- **Kind.** A defect of delivery: the reference that `primer@where-knowledge-goes` asks for, where
  a text must be revisited if the entry changes, is missing at these restatements.
- **Scope.** Owned by agent-skills, the installed agents' sources. It reverses nothing.
- **Better fix.** The rival is no change, since the audit lens now finds the drift. It finds it
  only when an audit runs, and a change to a core head ships the drift in between. `%%` lines make
  `cargo klarch show` on each head list the agents that restate it, at the moment of the change.
- **Route.** A text edit, at the agents' sources: one `%%` line per restatement of a core head.
- **Proposal.** Handle now: add `%%` citations at the agents' restatements of the checker's rules,
  each naming the core head it restates.

- **Default:** handle now.
- **Outcome:** approved.

## W8: the plain-text-pointer tripwire covers conversions only

- **Says.** A reviewer found new text written as plain-text paths, not converted from a reference,
  and judged that the tripwire did not fire. The report asks whether new plain text counts.
- **Still applies.** The firing clause is unchanged, `tripwire@agent-skills@plain-text-pointer-found`:
  "a diff that turns a checked reference or a backticked path into plain text naming the same
  target … or a retrospective's answer … reports a pointer written as bare plain text to clear a
  finding".
- **Recorded.** It is the tripwire itself.
- **Kind.** No defect. The tripwire guards `design@agent-skills@plain-text-is-no-repair`, a
  decision about how a finding is repaired. New plain text written where a checked form exists is
  a non-conformance the review repairs, and the path-to-anchor finding reports most of it; it is
  not the repair the decision forbids. The reviewer's reading was right.
- **Proposal.** No change.

- **Default:** no change; the tripwire guards repairs, and new plain text is not one.
- **Outcome:** approved.

## C1: a project skill named by an ordinary word

- **Recorded** as `issue@core@tooling-for-project-skills`, which quotes the owner's direction, a
  prefix declared in the manifest.

- **Default:** no change, recorded.
- **Outcome:** approved.

## C2: every new kind of observation is a major library change

- **Says.** The observation enum is public and exhaustive; the report proposes `#[non_exhaustive]`.
- **Still applies.** The enum has no such attribute, on purpose, per `design@core@ne-minimal`.
- **Recorded.** The proposal lost:
  `path@core@docs/rejected-alternatives.md`:413, "**`#[non_exhaustive]` on every public enum
  expected to grow** — lost to `design@core@ne-minimal`. … Under 0.x it buys no version number".
- **Proposal.** No change: the proposal is a recorded loser, and the report brings no new argument.

- **Default:** no change, a rejected alternative.
- **Outcome:** approved.

## C3, with W3 and C1 of 2026-10-09-knowledge-architect-path-quickfixes-spec-klarch-workflow and C2 of 2026-10-09-thaum-slice-cards-klarch-workflow: a plan item named as plain `#<id>` is checked by nothing (cluster K4)

- **Says.** Three files, two projects, four findings.
  - Outside its plan, a plan item is named as plain `#<id>`, as the planning skill prescribes. A
    wrong slug written as a criterion's name in a harvest message passed every gate here; in thaum,
    a doc comment and a sibling plan cited a thread as `#<thread>`, and at the harvest nothing
    reported that they should now name the new head.
  - Inside its plan, about 35 plain `#<id>` citations here, and 68 in the path-quickfixes spec,
    carried over from the design discussion's own `#<id>` rule, were silent although a checked form
    exists there; `show` then reported the items as referenced by nothing.
  - Proposals: a check on a plain `#<id>` whose id is a plan item; in a plan document, a report of
    a plain `#<id>` naming one of its own items, with the reference as the repair; a checked thread
    form that resolves while the plan exists; and, for the text, the design skill saying its `#<id>`
    rule holds in the discussion and the plan document cites its own items in the checked form.
- **Still applies.** Yes.
  - The prescribed form: `path@agent-skills@content/skills/planning/SKILL.md`:229-232, "From
    anywhere else, a commit message and a design head included, the plan is cited whole, and an
    item is named in plain text with a hash sign, as #<id>." Admitted by
    `design@agent-skills@plain-text-is-no-repair`.
  - The design skill, `path@agent-skills@content/skills/design/SKILL.md`:261-265: "In prose, write
    it plain with a `#` before it … In the plan document the thread becomes an item, cited from
    inside its own plan only, per `skill@knowledge-architect-planning`." It points to the planning
    rule; it does not say the `#` form stays in the discussion, nor name the checked form.
  - The checker: a backticked `#<id>` naming a defined entry is reported as the retired form, plan
    items included, per `design@core@candidate-rule-and-retired-forms`. A plain one, inside a plan
    or outside, is read by nothing. `cargo klarch check` passes on main while
    `spec@plans@path-quickfixes` holds two plain names of its own items, both inside quotations.
- **Recorded.** No issue records the check.
  - The checked citation of an item from outside its plan lost:
    `path@core@docs/rejected-alternatives.md`:462-466, "Citing an item of another plan — lost to
    `design@core@plan-item-scope`", because item citations across plans "would force a design
    revisit of every citing plan, item by item, at the moment a plan is retired". The thaum
    report's first proposal is that loser.
  - "The retired slug reference reported whatever it names" lost to
    `design@core@candidate-rule-and-retired-forms` (:360-366), because a backticked `#<id>` is often
    another project's notation, such as an issue number. A check of plain `#<id>` inside a plan
    document, limited to that plan's own item ids, does not meet that argument: the set of ids is
    the plan's own.
  - `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` covers a plain name once its
    entry is gone, not while it exists.
- **Kind.** Inside a plan: a defect of the text (the design skill's rule carried into the plan, two
  instances) and a blind spot of the checker. Outside a plan: a blind spot by decision; the plain
  form is the price of `design@core@plan-item-scope`, paid at every harvest by a reviewer reading.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`. The text
  part is owned by agent-skills, the design skill. The check is owned by core; a new finding on
  plain text is a change of `design@core@candidate-rule-and-retired-forms`, under
  `skill@knowledge-architect-design`.
- **Better fix.** For the text, the design skill's sentence states the scope: "The `#<id>` form is
  the discussion's; the plan document cites its own items by their checked form, per
  `skill@knowledge-architect-planning`." One sentence, no rival but no change, which leaves the
  carried-over form to two more reviewers. For the checker, the narrowest rule that meets a real
  case is a finding on a plain `#<id>` inside a plan document that names one of that plan's own
  items; the outside case needs a design discussion that answers both rejected alternatives above.
  One issue holds both.
- **Route.** The text: an edit whose design is settled, at the design skill's source. The checker:
  a design question, an issue.
- **Proposal.** Handle the design skill's sentence now, and open an issue, kind `design`, in core:
  a plain `#<id>` naming a plan item is read by nothing, inside its plan or outside it.

- **Default:** handle the text now; open an issue, `design`, in core, for the check.
- **Outcome:** approved.

## C4: no checked form names a level-three subsection or a list point of a skill's section

- **Says.** A citation such as "test 4 of" a section stays in words. Three such citations went stale
  or were wrong from the start, found only by a fidelity reviewer reading by hand.
- **Still applies.** Yes. Only level-two sections of a skill, an agent, the primer and the root are
  citable, per `design@core@section-homes-carry-slugs`. Citations in words remain, for example
  "test 4 included" at `path@agent-skills@content/agents/decision-record-reviewer.md`:71, "A
  load-bearing gap of point 2" at `path@agent-skills@content/skills/planning/SKILL.md`:359, and
  "loop step" numbers across the design skill.
- **Recorded.** No issue. The rejected alternative on entries at level three in core concerns
  register entries, not section citations.
- **Kind.** A missing form of the checker. Three instances in one session, each a stale pointer no
  check reaches.
- **Scope.** Owned by core, with the installed text's citation style. A new form is a change of
  `design@core@harness-kinds-cited-without-anchor` or of `design@core@section-homes-carry-slugs`,
  under `skill@knowledge-architect-design`.
- **Better fix.** Rivals: a form for a numbered point; the report's alternative, that each such point
  is a heading with a slug, which multiplies slugs in every skill; no change, the citations in
  words reviewed by hand. The choice needs a design discussion.
- **Route.** A design question, an issue.
- **Proposal.** Open an issue, kind `design`, in core.

- **Default:** open an issue, `design`, in core.
- **Outcome:** approved.

## C5, with C3 of 2026-10-09-knowledge-architect-path-quickfixes-spec-klarch-workflow and P2 of 2026-10-09-knowledge-architect-path-quickfixes-spec: no checked form quotes text that holds the checker's syntax (cluster K5)

- **Says.** Two sessions quoted the owner's words, or raw spans, holding text in the checker's
  syntax. Backticked, each became a dangling reference or a finding. One session wrote them as
  angle-bracket placeholders marked as such, so the quotation is no longer verbatim; the other wrote
  them bare, beside no issue entry. The proposals: a quotation form the checker leaves silent, or an
  issue entry in core that such a passage cites.
- **Still applies.** Yes.
  - The rule: `design@agent-skills@plain-text-is-no-repair` admits "text in the checker's syntax
    that no checked form expresses, written without backticks beside a reference to the issue entry
    that records the missing form" (`path@agent-skills@docs/design.md`:914-916). No such issue
    exists, so no passage can meet the rule.
  - `design@agent-skills@a-past-sentence-is-rewritten` leaves a verbatim quotation of the owner
    naming a renamed entry as it is, with the current reference beside it. It does not cover a
    quoted span that is itself a candidate.
  - The instance on main: `spec@plans@path-quickfixes`, at its lines 340-342, quotes the owner's
    words naming the reference form of a skill and an agent, written bare, "(its backticks dropped
    here, since in backticks each would cite a skill or an agent named `name`)", beside no issue
    entry. The other passage of that spec, at 559-560, describes the spans in words, which the
    primer puts outside the rule.
- **Recorded.** No issue of a missing form. At the analysis, the entry now
  `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` asked only where the owner's
  words go in a head, not their syntax.
- **Kind.** A missing form of the checker, and a non-conformance on main that only the missing
  issue can make conformant. Two sessions.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`: a ruling quoted inexactly is
  the failure the record guards against. Owned by core, for the form.
- **Better fix.** Rivals: the placeholder form, which the primer already names for an illustration
  and which the agent-configuration-entities session used, at the cost of verbatim; a quotation
  form the checker leaves silent, a change of `design@core@candidate-rule-and-retired-forms`; a
  rewrite in words. The issue is what the rule asks for first, whichever form is chosen.
- **Route.** An issue in core; then the spec's passage cites it, a text edit.
- **Proposal.** Open an issue, kind `todo`, in core: no checked form quotes text that holds the
  checker's syntax; then handle now the spec's passage, which gains a reference to that issue.

- **Default:** open an issue, `todo`, in core; then the spec's passage cites it.
- **Outcome:** IMO, the problem here is the other way around: I don't consider these owner quotes very valuable most of the  time, as long as attribution is correct. I do not want to provide this feature from the checker. The design question to ba recorded in the issue is instead, rethink the place and importance of owner quotes in the workflow. .
- **Consequences, put to the owner after the ruling:** no core issue is opened;
  `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow`, which asked the same question
  for design heads, is widened to the whole workflow, with K5 as its evidence; and the bare passage of
  `spec@plans@path-quickfixes`, which no issue of a missing form can now make conformant, is
  rewritten in words, the attribution kept. The owner: "Agreed on both points, proceed".

## C6: no versioning row for a check made looser

- **Recorded** as `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`.

- **Default:** no change, recorded.
- **Outcome:** approved.

## Standing answers

- **The owner's word and its premise.** Three cases of a premise the session supplied and did not
  check, or a challenge it judged false without telling the owner. `path@agent-skills@content/PRIMER.md`:23-26
  stands. Lapses.
- **An answer written before a tool call never reached the owner, and a record then quoted it.**
  A property of the harness's display, not of the workflow's text. The record's quotation is the
  lapse; the transcript reviewer reads what the owner was shown.
- **References written without backticks.** The four cases are K4, C4 and the changelog case,
  which `design@agent-skills@plain-text-is-no-repair` admits.

- **Default:** no change.
- **Outcome:** approved.
