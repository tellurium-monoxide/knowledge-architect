# Analysis of 2026-10-08-thaum-roadmap-revision-klarch-workflow

- **Received file:** 2026-10-08-thaum-roadmap-revision-klarch-workflow.md, the workflow's side of
  the thaum project's retrospective of a roadmap revision run on the in-change path. The project's
  own file was read for grounding only.
- **Version it used:** knowledge-architect 0.5.0, pinned.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1.
- **Standing entries that bear on it:**
  - `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names`: W2 is an occurrence of
    it. See W2.
  - `tripwire@agent-skills@ruling-lost-in-change` watches the in-change path's message; W5 reports
    no ruling lost or misstated, so it does not fire.
  - `tripwire@agent-skills@head-created-without-deliberation`: W4 describes a decision of absence
    written into a head's body, in another project; this tripwire watches this repository's own
    history, so the case is evidence for W4 only.

## W1, with W6 of 2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow: the searcher's groups fix no sort order

- **Says.** Searchers sorted the listings differently, so groups of positions could overlap or
  leave a gap. The report proposes stating the order in the agent's description, or a command that
  prints a numbered listing.
- **Still applies.** Repaired. At 0.5.0 the agent's body already fixed the order by anchor then id;
  the report blamed the description only. The collation was open, and 2bf743f, "[audit] Pass 1 of
  the agentic-workflow axis…", fixed it:
  `path@agent-skills@content/agents/standing-entry-searcher.md`:49-53, "each sort in byte order
  (`LC_ALL=C sort`), since other collations order hyphenated ids differently. The issue listing
  prints its rows by kind first, so always sort it." The description needs no order: the
  dispatcher hands out positions, and each searcher reads the order from its body.
- **Recorded.** No entry. No rejected alternative.
- **Note.** This intake's own brief told its two searchers to take the rows "as printed",
  against the agent's body. Both found the conflict, read the union of the two slices, and
  reported it. The fault was the brief's, and the agent's text held.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## W2: a subagent's write to its scratch directory can be refused by the harness

- **Says.** A general-purpose subagent was refused a write to its own scratch directory, with
  "Subagents should return findings as text, not write report files", and returned its table as
  text. The report proposes a fallback in every skill that has a subagent write a working file.
- **Still applies.** Yes. `path@agent-skills@content/skills/review/SKILL.md`:114-116 names a
  scratch directory per subagent and says nothing of a refused write. The report's claim that "The
  review skill already says reviewers return findings as text" is false, at 0.5.0 and now.
- **Recorded.** Yes: `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names`, an
  observation with two occurrences in this repository, both general-purpose reviewers, none an
  installed agent. Its closing condition is one dispatch of a general-purpose subagent "briefed to
  write one file in a named scratch directory", observing the refusal. This occurrence is the
  third, the first in another project, and again a general-purpose subagent. In this intake's own
  session, three general-purpose checkers wrote files in their scratch directories without refusal,
  but neither was briefed to write a report file, so the closing test was not run.
- **Kind.** An observation of the harness, recorded.
- **Proposal.** The issue gains this occurrence in its "What", from another project, with the
  same pairing: a general-purpose subagent refused, no installed agent refused.

- **Default:** handle now: the issue gains the third occurrence.
- **Outcome:** approved.

## W3: no review axis checks the code claims of documents that are not plan documents

- **Says.** The change wrote no plan document, but about 40 issue entries and a record of about
  1000 rows naming types, functions and tests. An improvised general-purpose reviewer found 10
  wrong statements and about 50 imprecise ones, the most consequential findings of the review. The
  report proposes widening the code-claims reviewer's scope, or a review axis for code claims
  outside plan documents.
- **Still applies.** Yes. `path@agent-skills@content/agents/code-claims-reviewer.md`:3, "The
  code-claims axis of a dispatched review of a plan document, a spec or a milestone document", and
  its scope, :11-12, "every statement in a plan document about the code as it stands". The review
  skill's table, `path@agent-skills@content/skills/review/SKILL.md`:41, sends it "the same moment"
  as the other plan-document axes.
- **Recorded.** No issue, no rejected alternative of agent-skills on its scope.
- **Kind.** A defect of the text, missing: the review before a merge has no axis that reads a
  change's statements about existing code against the tree, when they sit outside a plan document.
  One real case, with a measured yield.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`: an issue
  entry's diagnosis that is false of the code misleads every later session. Owned by agent-skills:
  the review skill's axis table and the reviewer agent. A new row of the axis table is a decision
  about the review's shape, under `skill@knowledge-architect-design`, and it reads against
  `design@agent-skills@plan-read-against-the-record`, which places the code-claims reviewer among
  the plan-document reviewers.
- **Better fix.** Rivals: widen the agent's scope to "every document of a change that states what
  the code does"; add a condition to the review table's code-claims row ("or a change that writes
  issue entries, records or heads stating what existing code does"); no change, since a session
  may dispatch a general-purpose reviewer, as this one did. The last relies on the session thinking
  of it; this one did, and the yield shows what is lost when a session does not. Between the first
  two, the row's condition is the narrower change and keeps the agent's brief as it is. The choice
  is a design question with one real case behind it.
- **Route.** A decision, under `skill@knowledge-architect-design`; not for this intake's branch.
- **Proposal.** Open an issue, kind `todo`, in agent-skills: the code-claims axis reads plan
  documents only, with this case and its yield.

- **Default:** open an issue, `todo`, in agent-skills.
- **Outcome:** approved.

## W4: a sentence in a head's body can carry a decision its title does not state

- **Says.** The owner's ruling "a terminal interface is not built" landed as the last paragraph of
  a head titled after another decision. The session read it as an existing statement rewritten,
  the reviewer as a new decision of absence. The report proposes saying whether such a rewrite
  extends the head.
- **Still applies.** Repaired. The blamed phrase, "extends one beyond what its title states", is
  gone from the installed text: it moved to the primer with d95e227, "The rules on design heads
  move from the decision-recording skill into one section of the primer", and was replaced by
  71da373, "The primer's section on design heads gains the rules the head-rules discussion
  decided", and 546faad. `path@agent-skills@content/PRIMER.md`:227-234: "A decision that creates a
  head, contradicts a statement of one, its argument included, or adds a member the head's argument
  does not cover, goes to the design skill … An addition that answers another question than the
  head's takes a head of its own". And :341-342: "**A head holds one decision.** Two statements are
  one decision when they answer one question and lose to the same nearest rival." A decision of
  absence about another subject answers another question, so it takes a head of its own.
- **Recorded.** `tripwire@agent-skills@head-created-without-deliberation` watches such events in
  this repository.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## W5: the in-change path puts a whole discussion's deliberation into one commit message

- **Says.** A discussion of about 40 threads landed every thread in one change, so one message of
  about 150 lines carried the deliberation, later edited twice. The report proposes a size above
  which the full path is preferred, or a split of the deliberation across the change's commits.
- **Still applies.** Partly repaired. f449b17, "[audit] Pass 6 of the agentic-workflow axis
  applied by cause; the audit stops", narrowed the path. `design@agent-skills@in-change-path` now
  says, at `path@agent-skills@docs/design.md`:1085-1087, "A requested change that is not bounded,
  and whose work has not started when its discussion opens, takes the full path too, even when one
  pull request builds it." The path's other opening stands,
  `path@agent-skills@content/skills/design/SKILL.md`:141-153: "or its record, whenever the
  discussion opened, for a decision that no work implements and that is not part of any spec". A
  roadmap revision, whose decisions no work implements, still takes the in-change path. No size
  bound and no split exist.
- **Recorded.** No issue. A reason at the text argues against one split:
  `path@agent-skills@content/skills/design/SKILL.md`:128-134, a comment saying the commit that
  carries the deliberation carries the rulings, and that "the rival, the rulings landing in the
  later commit that writes the tripwires, splits the deliberation over two commits."
- **Kind.** A cost, not a defect: the message carried every thread, and the later edits were
  repairs of claims, which a plan document would have needed too. One instance.
- **Scope.** Owned by agent-skills, `design@agent-skills@in-change-path`. A size bound or a split
  changes that head, under `skill@knowledge-architect-design`.
- **Better fix.** The split meets the reason at the text above. A size bound needs a figure that
  one case cannot set. No change, with this case as the first evidence should a second come.
- **Proposal.** No change.

- **Default:** no change; one instance, and the split rival meets a recorded reason.
- **Outcome:** approved.

## C1: a skill or an agent name has no checked form

- **Says.** A head named a skill by a wrong name in a code span; the checker passed. The report
  proposes `skill@<name>` and `agent@<name>` kinds.
- **Still applies.** Repaired, unreleased. 9b401e0, "The harness kinds: skills, agents, the primer
  and the root CLAUDE.md cited by kind and section, every level-two heading slugged", and 415df73.
  The heads are `design@core@harness-kinds-cited-without-anchor`,
  `design@core@bare-skill-name-reported` and `design@core@harness-entity-names`. A misspelt name in
  a bare code span is still silent, since the bare-name check reports only an exact defined name;
  written as its reference, the misspelt name dangles and is reported.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## C2: no command checks the staged tree

Analysed with C1 of 2026-10-08-thaum-slice-4g-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-thaum-slice-4g-klarch-workflow.md`.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## Standing answers

- **The reversal check applied too narrowly, and a review record that called a repair done.** The
  quoted instructions stand: `path@agent-skills@content/skills/decision-recording/SKILL.md`:44 and
  `path@agent-skills@content/skills/review/SKILL.md`:199. Lapses, as the report says.
- **Statements about the code copied without re-reading.** `path@agent-skills@content/PRIMER.md`:65
  stands. A lapse; it is also the evidence of W3.
- **Skill names written in code spans.** Reported by the checker now, per
  `design@core@bare-skill-name-reported`.

- **Default:** no change.
- **Outcome:** approved.
