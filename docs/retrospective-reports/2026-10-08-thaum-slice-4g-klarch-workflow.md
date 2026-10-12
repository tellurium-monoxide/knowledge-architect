# Analysis of 2026-10-08-thaum-slice-4g-klarch-workflow

- **Received file:** 2026-10-08-thaum-slice-4g-klarch-workflow.md, the workflow's side of the
  thaum project's retrospective of the slice that landed its milestone. The project's own file was
  read for grounding only.
- **Version it used:** knowledge-architect 0.5.0, pinned in the project's manifest.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1.
- **Standing entries that bear on it:**
  - `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`: W4
    is near its subject. See its section.
  - `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`
    (deferred), whose trigger names an intake that reads a real case. See its section.
  - `tripwire@core@staged-pass-fails-the-checkout` and `issue@core@index-staged-clears-an-entrys-flags`
    guard and observe the staged commands that answer C1; this file, on 0.5.0, has no staged
    command and meets neither.
  - `tripwire@agent-skills@expectation-set-closes-a-contradiction`: no finding here is closed by
    citing an expectation set.

## W1: a returned standing entry judged unrelated has no home for its judgement

Cluster K1, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence-klarch-workflow.md`.
This file's instance is the largest of the three: 41 entries returned, 22 judged, 19 written later
into the audit's message by a history edit.

- **Default:** handle now, with K1.
- **Outcome:** approved.

## W2: a plan document's "Later consequences" has no destination when the document leaves

- **Says.** The milestone document left with the slice. Its "Later consequences" section held work
  handed to later milestones. The session moved each consequence as a lead into the `todo` issue of
  the milestone it names, put that to the owner as a default, and the owner kept it. No instruction
  covers the move.
- **Still applies.** Yes. `path@agent-skills@content/skills/planning/SKILL.md`:181 defines the
  section: "what each later piece of work adds or replaces, so a later reader knows what was
  deliberately left". At the leaving, :542-543: "what the document established is in the design
  homes and the registers; what stayed a guess leaves with it". A later consequence is neither a
  decision built nor a guess. The section is named nowhere else in the planning or issue-tracking
  skills.
- **Recorded.** No. `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`
  concerns the acts that need a deleted spec, not where this section's content goes.
- **Kind.** A defect of the text, missing. One real case, resolved by the owner's word on the
  session's default.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`: what was
  deliberately left is part of the argument, and a deleted plan document takes it out of every
  tree. Owned by agent-skills, the planning skill's leaving checklist.
- **Better fix.** The rival is no change: the session found the move, and the owner kept it. It
  cost a question to the owner that the text can answer once. The fix is the move the owner kept,
  stated in the leaving checklist: each later consequence is in the issue entry or the plan
  document of the later work it names, or, where no such entry exists, in a `todo` issue opened for
  it.
- **Route.** A text edit whose design is settled, at the planning skill's source.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## W3: converting an acceptance criterion into an existing tripwire owes nothing stated

- **Says.** Two criteria were converted "into existing entries" on a premise the session did not
  check: one existing entry watched the opposite direction. Three harvest reviewers found it, and
  the entry was widened. The report proposes that the planning skill state that a criterion
  converted into an existing entry folds its firing condition and its response into it, as an
  absorbed tripwire does.
- **Still applies.** Reachable, through two pointers. `path@agent-skills@content/skills/planning/SKILL.md`:346-349:
  "One that did not fire and recurs at later work is proposed to the owner as a tripwire, and
  written on the owner's word as `skill@knowledge-architect-decision-recording@ruled-tripwires`
  says" (pointer added by 6ee997d, "[audit] Pass 4 of the agentic-workflow axis…"). That section
  ends at the tripwire's shape in the issue-tracking skill, whose movement instruction says, at
  `path@agent-skills@content/skills/issue-tracking/SKILL.md`:273-275: "A tripwire is **absorbed**
  when another entry already guards the same decision: fold its firing condition into that entry".
  No text says it applies to a criterion's conversion.
- **Recorded.** No.
- **Kind.** A defect of the text, unclear, with a lapse beside it: the premise the session put to
  the owner was its own to check, per `primer@owner-word-premise`. The text part is one clause.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`. Owned by agent-skills, the
  planning skill. It reverses nothing.
- **Better fix.** The rival is no change, since the chain of pointers reaches the absorption rule.
  The session did not follow the chain, and the owner ruled on a false premise; one clause at the
  point of delivery costs less than that ruling. The fix: the conversion sentence gains "or, where
  an entry already guards the same decision, folded into it as an absorbed tripwire is".
- **Route.** A text edit whose design is settled, at the planning skill's source.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## W4: a message check right after a commit can fail while the amend is forbidden

- **Says.** The project runs the message check right after each commit. Three times it failed on a
  commit made while unrelated work was uncommitted, so the amend the review skill names was
  forbidden. The session rebuilt the branch with `git commit-tree` and `git update-ref`, which touch
  no working-tree file. The report calls the instructions satisfiable (commit the other work first)
  and proposes saying so.
- **Still applies.** The text is unchanged since 0.5.0:
  `path@agent-skills@content/skills/review/SKILL.md`:179-181, "**A commit message carrying a
  mistake is repaired by amending** while it is the newest commit, and by a history edit of the
  branch after that. Either only with no uncommitted work in the tree". No installed text names the
  plumbing route.
- **Recorded.** No. `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`
  is about the installed text not recommending the message check at all.
- **Kind.** No defect. The rule has a move that satisfies it: finish or commit the other work, then
  repair the message by a history edit with a clean tree, which the rule names. The plumbing route
  is a history edit made with uncommitted work in the tree, which the rule forbids by its terms,
  whatever files it touches: the rule's hazard is content loss, and a route that cannot lose
  content today is still the operation the rule bounds.
- **Scope.** Owned by agent-skills, the review skill. Adding the plumbing route would widen
  `design@knowledge-architect@no-operation-loses-content`'s installed counterpart without an
  argument.
- **Better fix.** The report's sentence, "commit the other work first", is a consequence the rule
  already implies. No change.
- **Route.** Nothing.
- **Proposal.** No change.

- **Default:** no change; the rule has a move, and the route taken is the operation it forbids.
- **Outcome:** approved.

## C1, with C2 of 2026-10-08-thaum-roadmap-revision-klarch-workflow: no command judges the staged tree

- **Says.** `check --fix` regenerated an index from the working tree, which held uncommitted
  sources; the staged index then failed `commits` with "the generated file is out of date". The
  report proposes a `--staged` mode for `check` and `check --fix`, or a `commits` finding that
  names the uncommitted files.
- **Still applies.** Repaired, unreleased. The series 4bc5797, "[plan] The spec of check --staged,
  index --staged and the --fix refusals", to dfe33ba, "[core] Step 4 of check --staged: check --fix
  refuses a partial commit's mismatch", then eaf0cef for the installed text. `cargo klarch check
  --help` prints "--staged  Judge the tree git's index would commit … To repair its generated
  files, run `index --staged`, then `check --staged` again; `--fix` repairs the working tree
  only". The setup skill recommends `check --staged` before each commit
  (`path@agent-skills@content/skills/setup/SKILL.md`:192-195), and the issue-tracking and planning
  skills name `index --staged`. The entries sit under `Next release` in CHANGELOG.md, so thaum at
  0.5.0 has none of it.
- **Recorded.** The decision is `design@core@staged-check-semantics`. The second proposal, a
  finding that names the cause, runs against `design@core@finding-names-the-repair`.
- **Kind.** A defect of the checker, missing; repaired.
- **Proposal.** No change. The thaum project receives it at its next pin move.

- **Default:** no change, already repaired.
- **Outcome:** approved.

## Standing answers

- **The primer rules on the owner's word and on something met outside the task.** The three
  sentences stand in `path@agent-skills@content/PRIMER.md` (:23, :26, :73). The report describes
  the session not applying them; no defect of the text is claimed.
- **A project skill's addition missed.** The project's own text; nothing here.
- **Six paths first written without an anchor in two messages.** The checker reported each, and
  each was rewritten in the checked form: the checker worked.

- **Default:** no change, lapses.
- **Outcome:** approved.

## Standing entry: the trigger of `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`

- **Judgement.** Not met. The trigger asks for a real case of one of four acts that need a deleted
  spec: the transcript reviewer's search, the standing-state reviewer's reading of criteria, two
  review axes, a thread-to-slug pair. W2 is about where a section's content goes before the
  deletion, and no received file reports an act misled by a deleted spec.

- **Default:** trigger not met.
- **Outcome:** approved.

## Standing entry: the closing condition of `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`

- **Judgement.** Not met. It closes on "A retrospective that reports a message finding first met at
  the gates, in a project that follows the installed text". This project runs the message check
  right after each commit, by its own rule, so its findings were met at once, not at the gates.

- **Default:** condition not met.
- **Outcome:** approved.
