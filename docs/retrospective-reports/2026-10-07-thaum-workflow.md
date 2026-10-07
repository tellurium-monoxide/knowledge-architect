# Analysis of 2026-10-07-thaum-workflow

- **Received file:** 2026-10-07-thaum-workflow.md, the workflow's side of a retrospective of a
  project that uses the workflow. The project's own file was read for grounding only.
- **Version it used:** 0.4.0.
- **Analysed against:** main at v0.4.0-41-gf5d403a, plus the commit "A received retrospective file is
  analysed under klarch-retrospective-intake, the retrospective names its files by subject, and the
  searcher's groups are balanced".
- **Standing entries that bear on it:** `tripwire@agent-skills@ruling-lost-in-change` (W1, W4: it
  does not fire, since the transcript reviewer found the ruling before the merge, and the clause asks
  for a ruling found lost after it), `tripwire@agent-skills@search-missed-before-the-work` (W3: it
  does not fire; no entry was missed).

## W1, with W8 of 2026-10-07-knowledge-architect-workflow: the in-change path names decision-recording without saying what it decides

- **Says.**
  - W1 here: at convergence on the in-change path, the session proposed where the decision would
    land by the subject test, and the owner approved. Decision-recording §1 then put it in the
    incumbent's Component, since it reversed a recorded decision, and the owner had to rule twice.
    Proposed fix: the design skill's closing move applies §1 before it proposes where a decision
    lands.
  - W8 of the knowledge-architect workflow file: the same sentence, "The decision is recorded at that
    landing like any other", was read as "earns a head".
- **Still applies.** `path@agent-skills@content/skills/design/SKILL.md` lines 139-141: "The decision
  is recorded at that landing like any other, under `knowledge-architect-decision-recording`." The
  design skill names §1 only for bounded work, line 169. The W8 report says the sentence was repaired
  in its session; its history says otherwise: "under `knowledge-architect-decision-recording`" dates
  from `aa743fc`, "for a decision that earns none" from `77ee033`, both before that session.
- **Recorded.** None.
- **Kind.** One sentence read wrong twice, in two ways, by two sessions. It delegates without saying
  what it delegates: a defect of delivery, not a lapse, since the text read alone licenses both
  readings.
- **Scope.** Agent-skills; `goal@knowledge-architect@the-owner-decides`: an approval given before the
  placement rule ran had to be asked again.
- **Better fix.** The report's adds §1 to the closing move. Rival covering both findings: the one
  sentence says what decision-recording decides there, "whose §1 decides the Component when the
  decision reverses a recorded one, and whose §2 decides whether it earns a head", and the closing
  move proposes a landing only after that. One edit, at the sentence both sessions read.
- **Route.** A text edit, installed text.
- **Proposal.** Handle now. CHANGELOG `Workflow` entry, patch: a session asks where a reversal lands
  only after the reversal check.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W2: the routing reviewer and the decision-record reviewer gave opposite verdicts on a reversal's Component

- **Says.** The routing reviewer judged the root right, by "does this decision survive deleting the
  Component?"; the decision-record reviewer judged it wrong, by §1. Proposed fix: the routing
  reviewer defers to §1 when the decision reverses a recorded one.
- **Still applies.** `path@agent-skills@content/agents/routing-reviewer.md` lines 95-102 hold the
  one question, with no reversal exception. The decision-record reviewer declines the question:
  "**Not** whether the decision is right, and not _which_ document family owns it, which is the
  routing axis" (`path@agent-skills@content/agents/decision-record-reviewer.md` lines 14-15), while
  its standard is all of decision-recording, §1 included.
- **Recorded.** None.
- **Kind.** Two installed agents that leave the dispatcher no move satisfying both: always in scope.
- **Scope.** Agent-skills; `goal@agent-skills@one-skill-per-activity`.
- **Better fix.** The report's. Rival: the decision-record reviewer owns the Component of a reversal
  outright. That moves a question the routing axis owns for every other decision; the exception is
  narrower in the routing reviewer, where the question is asked.
- **Route.** A text edit, installed agent.
- **Proposal.** Handle now: the routing reviewer's Component question gains "unless it reverses a
  recorded decision, which stays in the incumbent's Component, per §1 of
  `knowledge-architect-decision-recording`".
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W3, with W5 of 2026-10-07-knowledge-architect-workflow: subagents dispatched together share one scratch directory

- **Says.**
  - W3 here: parallel standing-entry searchers wrote their files in one scratch directory and
    overwrote each other's; one repeated its work.
  - W5 of the knowledge-architect workflow file: the review skill names a distinct worktree per
    reviewer, and no path for scratch output; two reviewers reported a possible overwrite.
- **Still applies.**
  - `path@agent-skills@content/agents/standing-entry-searcher.md` lines 18-19: "**You write
    nothing.** You do not use `Write` or `Edit`, and you run only commands that read." Searchers
    write all the same: their task sorts a listing and runs one `show` per entry. In this session,
    two of the seven agents dispatched for this intake wrote files, under the directory the brief
    named.
  - `path@agent-skills@content/skills/review/SKILL.md` lines 90-96 name a distinct worktree per
    reviewer; nothing names a scratch path.
  - The transcript reviewer is the precedent: "You may write your extraction script and its output to
    a scratch directory the brief names, through the shell, and nowhere else"
    (`path@agent-skills@content/agents/transcript-reviewer.md` lines 28-29).
- **Recorded.** None.
- **Kind.** Missing, twice in two sessions on one day. The searcher's "you write nothing" is an
  instruction its own task leads it to break.
- **Scope.** Agent-skills.
- **Better fix.** One rule for both: a dispatcher that sends subagents together names a distinct
  scratch directory in each brief, and an agent that writes working files writes them there only.
  Placed in review §2 beside the worktree rule, in the searcher's description beside its dispatch
  rule, and in the searcher's body in place of "You write nothing", worded as the transcript
  reviewer's. Rival: forbid files and keep "write nothing". It fails on the searcher's own task.
- **Route.** A text edit, installed skill and agent.
- **Proposal.** Handle now.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W4: the transcript reviewer was dispatched while a ruling it would read was pending

- **Says.** Repairs were committed and two defaults put to the owner; the transcript reviewer was
  sent before the answer and reported the ruling missing. Proposed fix: send it once no ruling the
  repairs asked for is pending, or have the dispatcher reconcile its findings.
- **Still applies.** Review §1, lines 46-47: "after every other axis has run and its repairs are
  committed". Nothing names a pending ruling. The transcript reviewer handles a message that
  overtook an item inside what it read (lines 81-82), not one after.
- **Recorded.** None.
- **Kind.** Missing. The cost was one critical finding about a state that no longer held.
- **Scope.** Agent-skills; `design@agent-skills@transcript-review-last-before-merge`, whose title
  this stays within.
- **Better fix.** The first of the report's two. Reconciling afterwards is already what review §3
  asks of a misstated ruling; waiting removes the false finding instead of handling it.
- **Route.** A text edit, installed text: one clause in review §1.
- **Proposal.** Handle now: "and no ruling those repairs asked of the owner is still unanswered".
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## C1, with C1 of 2026-10-07-knowledge-architect-workflow: the empty-id finding does not name the forms that serve what the writer meant

- **Says.**
  - C1 here: a path reference to a location with an empty id was refused, "the id segment is
    empty", when a commit message needed a location's own directory; the session named three files instead. Proposed fix: accept the empty
    id or `./` as the root, or name the form in the finding.
  - C1 of the knowledge-architect workflow file: writing the generic form in prose gave the same
    finding, six times; the repair is a placeholder, which the message could name.
- **Still applies, in part.**
  - A checked form for an anchor's own directory exists: it is named from an ancestor anchor, as
    `path@knowledge-architect@.claude/` names the agent-config location's directory. The core's
    design states it under `design@core@every-path-names-its-anchor`, and the test
    `a_components_own_directory_is_named_from_an_ancestor` pins it. Accepting an empty id or `./`
    would reverse that head, which refuses a `.` segment on purpose.
  - The finding's repair, `path@core@src/check/references.rs` around line 139, reads "write
    `<kind>@<anchor>@<id>`, and for a path `path@<anchor>@<path>`". It names neither the ancestor
    spelling nor that an illustration takes a placeholder in angle brackets. No user documentation
    states the ancestor spelling.
- **Recorded.** `issue@core@finding-texts-are-not-audited-for-a-needless-cause` is the neighbour: an
  audit of findings against `design@core@finding-names-the-repair`, for a needless cause, not a
  missing repair. Not recorded.
- **Kind.** The checker's repair text, two sessions.
- **Scope.** Core.
- **Better fix.** One repair text for both: an illustration of a form writes its id as a placeholder,
  `path@*@<path>`; an anchor's own directory is named from the anchor above it. And one sentence in
  the core's README where it states the path rules. Rival: the report's new spellings, which reverse a
  head for a need a form already serves.
- **Route.** Rust, `klarch-development`, core, and the core README.
- **Proposal.** Handle now.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## The standing answers

- **The primer in subagents:** analysed with the same answer of the other files, in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-06-thaum-workflow.md`.
- **A pointer no checked form expresses:** the root of a location, which C1 above shows a checked
  form serves.
- **Default:** no change beyond C1.
- **Outcome:** pending.

## Noted, not findings

- **The setup skill's recommendation of CI on every ready pull request.** The owner of the project
  ruled that a recommendation is not to be flagged; the setup text, lines 188-189, conditions CI on a
  project having it. No change.
- **A `show` loop reading stdin.** Not reproduced. The checker spawns git only, with `Stdio::null()`
  unless input is piped (`path@core@src/git.rs` lines 93-97). No change.

- **Default:** no change for either noted item.
- **Outcome:** pending.

## Standing entries: two tripwires the file comes near

- **`tripwire@agent-skills@ruling-lost-in-change`**, on W1 and W4. It fires on a ruling "found
  missing from, or misstated in, the commit message that carries the deliberation of a discussion
  on the in-change path, after the branch's transcript review". Here the transcript reviewer found
  the ruling before the merge. Judgement proposed: not fired.
- **`tripwire@agent-skills@search-missed-before-the-work`**, on W3. It fires on an entry the work
  bears on that no search before the work returned. W3 is about the searchers' scratch files; the
  file reports no missed entry. Judgement proposed: not fired.
- **Default:** neither fired; no response.
- **Outcome:** pending.
