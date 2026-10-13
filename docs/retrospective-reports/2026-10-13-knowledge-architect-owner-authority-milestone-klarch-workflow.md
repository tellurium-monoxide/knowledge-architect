# 2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow

- **Received file**: 2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow.md,
  the workflow file of this repository's retrospective of the session that built the
  owner-authority milestone, one design discussion and five merged pull requests.
- **Version used**: the checker built from this repository, main at `v0.6.0-46-ge619a1e`, no
  uncommitted changes at the retrospective.
- **Analysed against**: main at `v0.6.0-75-g5fca230`.
- **Standing entries that bear on it**: from the one search over the 83 issues and 58 tripwires:
  - `tripwire@agent-skills@rule-title-wider-than-its-argument`, met by its letter, judged in its
    own section;
  - `tripwire@agent-skills@attribution-loses-the-scope` and
    `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, judged in
    2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow;
  - `issue@agent-skills@the-retrospective-counts-no-review-cost`, the measurement W7 asks for;
  - `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`,
    whose structure W5 meets and whose trigger the file does not meet;
  - `three-findings-of-a-received-retrospective-await-an-intake`, the
    outcome of W8's case;
  - `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`, near C2;
  - `issue@agent-skills@the-material-finding-duty-has-no-head` and
    `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`, near
    W4;
  - the tripwires judged together in the last section.

## W1. An attribution of the owner keeps dropping hedges and scope

Analysed with the cluster of attributions in
2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow, section "The attribution
cluster", whose proposal is this finding's own: a step at the close of the decision-recording
skill that puts the owner's words beside each attribution written.

- **Default**: as that section's outcome.
- **Outcome**: as the section it points to rules. The owner: "For the pending ones, I gave my word in the section holding the detailed analysis, you can point to them."

## W2. A harvest bundles decisions into one head, or writes a title wider than its argument

This section also holds the second bullet of the second standing answer of
2026-10-13-thaum-slice-numbers-merge-klarch-workflow: a harvest added three decisions inside
existing heads, each losing to a different rival, which the decision-record reviewer found.

- **a. What it says.** Seven review rounds found a head that held two decisions, or a title or
  argument that did not match the body. Twice it came from a harvest that added a delivery or a
  member to an existing head, which then answered a second question. The closing re-read step ran
  each time and did not catch it. The fix proposed: the re-read step of
  `skill@knowledge-architect-decision-recording@before-you-finish` names the two tests: does the
  head answer one question and lose to one rival, and is its title false of that rival.
- **b. Whether it still applies.** Yes. The tests are in `path@agent-skills@content/PRIMER.md`:398
  ("A title states no more than its argument argues") and 400-401 ("A head holds one decision. Two
  statements are one decision when they answer one question and lose to the same nearest rival").
  The closing steps, `path@agent-skills@content/skills/decision-recording/SKILL.md`:258-269, ask to
  re-read "each head you wrote: present tense", to apply "the entry tests … again", and to "re-read
  the head's whole body, its argument included, against the change". None names either test.
- **c. Whether it is already recorded.** In part. `tripwire@agent-skills@rule-title-wider-than-its-argument`
  watches the title half, judged in the next section. The bundling half has no entry. No rejected
  alternative matches.
- **d. Its kind.** A lapse that recurs at one point of delivery: seven review rounds here and
  three heads in thaum's session, at the moment a harvest writes or extends a head. Point 6 of
  `skill@knowledge-architect-planning@working-a-slice` already has the session read
  `primer@design-heads` whole before writing a head, so the rule is delivered before the writing;
  the closing step after it re-applies the entry tests and not these two.
- **e. Its scope.** The agent-skills Component, the decision-recording skill's closing steps. It
  serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`. It reverses no head:
  `design@agent-skills@one-decision-per-head` and `design@agent-skills@title-states-the-rule` state
  the tests.
- **f. A better fix.** Rivals:
  - no change, the decision-record review being the catch; it caught every one, at the cost of review
    rounds;
  - the two tests named in the existing closing step that applies the entry tests again, so the
    re-check after writing covers the shape of a head as well as whether it earns one.

  The second: the failure is at the head a change has just written, which is what the closing step
  re-reads, and the step already re-applies tests from the same section. Count: 1 added (one
  clause).
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent"; a Workflow entry in the `Next release` section.
- **h. The proposal.** In `skill@knowledge-architect-decision-recording@before-you-finish`, the step
  that applies the entry tests again also applies, to each head the change writes or extends, the
  test of one decision per head and the test of its title against the nearest rival.
- **Default**: handle now.
- **Outcome**: handle now, with the closing step pointing to every test of `primer@design-heads` rather than naming two. The owner, in the file: "approved. This is the second one on this. The skill should just say to apply all of the tests from the primer instead of restating them all, shouldn't it ?" And, in the conversation, on that pointer: "agreed".

## `tripwire@agent-skills@rule-title-wider-than-its-argument`, met by its letter

- **The firing clause**: "a review finds a title wider than its argument in a head written or
  rewritten after the decision." Its response: "repair the title, and reopen
  `design@agent-skills@title-states-the-rule`."
- **The evidence.** The decision predates the milestone. The message of the commit "[review]
  Repairs from the transcript review of slice 3: the last "approved" site, the bounded head's title
  and hedge, the reviewer's rivals, and the changelog's fork" says the title of
  `design@agent-skills@derived-decision-proceeds-on-default`, written in slice 3, "still covered every
  default the record answers, where its body now states the bounded path alone; it is narrowed". W2
  counts more such rounds. No commit message of the session names the tripwire. The title was
  repaired before the merge.
- **What bears on the judgement.** As for `tripwire@agent-skills@attribution-loses-the-scope`, in
  2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow: this is one of the five
  tripwires `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` lists as firing on one
  instance with no bound, and the owner ruled out a firing of that kind on a sixth tripwire as not
  load-bearing.
- **The proposal.** Judge it not fired: the title was repaired before the merge and nothing built
  rested on its width. The severity-bound issue gains the instance.
- **Default**: not fired; the issue on the severity bound gains the instance.
- **Outcome**: approved.

## W3. A harvest wrote into a head a default the owner had not ruled on

- **a. What it says.** In three review rounds, a harvest wrote into a head something not yet ruled
  or not yet built: two unruled defaults in slice 1, a rule a later slice was to build in slice 3,
  and a default resting on a still-unruled one in slice 4. The harvest runs under
  `skill@knowledge-architect-planning@working-a-slice`, which does not repeat the condition. The fix
  proposed: the harvest step lists the defaults still awaiting the owner and holds back every head
  edit that rests on one.
- **b. Whether it still applies.** The condition stands at
  `path@agent-skills@content/skills/decision-recording/SKILL.md`:35-37: "**A default awaiting the
  owner is built as written … and enters a design head once the owner rules on it.**" Point 6 of the
  slice procedure, `path@agent-skills@content/skills/planning/SKILL.md`:434-436, sends the harvest's
  decisions to that skill.
- **c. Whether it is already recorded.** No. `tripwire@agent-skills@head-created-without-deliberation`
  does not match: the defaults were listed in the milestone document, so the deliberation existed.
- **d. Its kind.** A lapse: the condition is in the skill the harvest loads before it writes into a
  design home, per `design@agent-skills@design-home-write-loads-recording`, so it is delivered at the
  moment it applies. Three instances in one session, each caught by a review.
- **e. Its scope.** None beyond the session, unless the delivery moves.
- **f. A better fix.** The rival is the list proposed, a restatement of the condition in the
  planning skill: a second home for one moment, which the repair by cause of
  `skill@knowledge-architect-agent-configuration@admission-standard` removes rather than adds. The
  lapse rule moves a delivery only where the instruction is read at one moment and needed at
  another, and here it is read at the moment.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of a condition delivered at the moment it applies.
- **Outcome**: approved.

## W4. May a branch merge while a default it builds still waits for the owner?

- **a. What it says.** Twice, a slice's review produced a default the owner had not ruled on; the
  first came from a material finding against a decided thread. The session read the planning
  sentence "Until the owner rules on a default, it is built as written" as allowing the merge, and
  merged both slices; the owner approved each default later. A reading under
  `primer@owner-word-premise`, "the part of the work that rests on it waits for the answer", would
  have held the merge. The fix proposed: the slice procedure states whether an unruled default may
  reach the main branch, and when a material finding is a defeated premise instead.
- **b. Whether it still applies.** Yes. The sentence the file quotes stood in the plan reviews
  section and was removed as a duplicate in the commit "Step 4: this repository's agent-facing text
  is brought to the need test"; its twin stays at
  `path@agent-skills@content/skills/planning/SKILL.md`:362-363: "A default the plan documents list
  as awaiting the owner is built as they write it until the owner rules on it." No text states
  whether such a default may merge. One text implies it may, for a slice: lines 563-564 hold a plan
  document's deletion until "every default the document lists as awaiting the owner is ruled", which
  for a milestone is its last slice. The primer's sentence, `path@agent-skills@content/PRIMER.md`:56-59,
  holds the part of the work that rests on "A ruling on intent whose premise proves false".
- **c. Whether it is already recorded.** No. Two entries are near:
  `issue@agent-skills@the-material-finding-duty-has-no-head`, on the design skill's duty to bring a
  material finding back with a default, which the primer's contradiction rule now also governs; and
  `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`, the same
  wait-or-go-on question for a divergence between a head and the code.
- **d. Its kind.** Unclear: two installed texts give a default produced by a material finding two
  routes, the design skill's material finding with a default the work proceeds on, and the primer's
  contradiction that holds the work. Whether they leave a move satisfying both is the question, and
  a finding that two installed instructions leave no move satisfying both is in scope whatever an
  expectation set says.
- **e. Its scope.** The agent-skills Component: the planning skill, the design skill's material
  finding, and `design@agent-skills@owner-word-holds-as-far-as-its-premise`. It bears on
  `goal@knowledge-architect@the-owner-decides`. The answer for a material finding changes what a
  head states, so it is a decision.
- **f. A better fix.** Rivals: one sentence in the slice procedure, as proposed, which would decide
  the material-finding case in passing; or the question argued where it belongs, beside the
  material-finding duty, which has no head yet. The fact that decides is that the answer for the
  material-finding case is a decision between two installed texts, which a sentence in a third
  should not settle unargued.
- **g. Its route.** `skill@knowledge-architect-design`, with
  `issue@agent-skills@the-material-finding-duty-has-no-head`.
- **h. The proposal.** An issue, `question`, in agent-skills: whether a slice merges while a
  default it builds waits for the owner, and whether a default that answers a material finding
  against a decided thread is a contradiction that holds the work, with the two readings and the
  implication of lines 563-564; its re-entry the design discussion of the material-finding duty.
- **Default**: open an issue, `question`, in agent-skills.
- **Outcome**: handle now, in the reading the owner confirmed at the end of the design discussion, which replaces the default and the first reading: a default put to the owner is built, and its branch merges once the owner has ruled on it; a question with no sensible default holds its part, as before. The owner, in the discussion: "If the question that is being put to me has a sensible default in the agents judgement, I think that the work can proceed with that default, and my approval can be asked only at merge time, thus not interrupting the work." And, on that reading stated back: "yes, that's the reading, proceed". Earlier, in the file and the conversation, before the owner judged that those words might have been a bit too strong. The owner, in the file: "Yes, a branch with a default with no ruling can merge. This is the point of a default. It is built, shown to the owner, and merged. Something changes only if the owner says so. If the owner's word is always required, this is no longer a "default"." And, in the conversation: "This is not what the session did IIRC. I think it did ask me to rule on all defaults left, and I think the session was getting quite confused about this long before the retrospective. I'd rather have it stated clearly. I think the rules in question-authority in the PRIMER should answer for whether the material findings becomes a hold or proceed with the default." The history shows both slices merged with a default awaiting the owner, D20 and D21, each ruled at the next slice's start, in "A design thread with one shape standing closes argued, only a fork waits for the owner's word, and bounded work builds its default" and "Defaults and findings the record answers are decided by the session in planning, review, the audit and setup, and only what the owner answers waits"; the owner was told.

## W5. The outcome of each claim of a slice's spec has no reporting point

- **a. What it says.** The slice procedure asks that "The commits name how each claim's test was
  shown to fail", and not each claim's outcome. In four review rounds a reviewer found a spec claim
  whose test outcome was never reported. The spec is deleted at the harvest. The fix proposed: the
  harvest commit reports each claim of the spec with its test and its outcome, before the spec is
  deleted.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/skills/planning/SKILL.md`:427-429,
  point 3: "The commits name how each claim's test was shown to fail against a wrong
  implementation, and say of any claim whose test cannot yet do so why not."
- **c. Whether it is already recorded.** In part:
  `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`
  describes the same structure, a spec gone from the tree that the review before the merge reads.
  Its four cases do not include a claim's outcome, and its trigger is not met: no reviewer here
  judged without the deleted spec; the spec-conformity reviewers found the gaps.
- **d. Its kind.** Missing text: point 3 asks how each test was shown to fail, and nothing asks for
  its result, so a claim whose test was never run, or a claim met by reading with no test, can go
  unreported. Four instances in one session.
- **e. Its scope.** The agent-skills Component, point 3 of
  `skill@knowledge-architect-planning@working-a-slice`. It serves
  `goal@knowledge-architect@agents-get-a-complete-workflow`. It reverses no head.
- **f. A better fix.** Rivals: the report in the harvest commit, as proposed; the same clause in
  point 3, where the commits that hold the tests are already asked to name them; or no change, the
  spec-conformity axis being the catch. Point 3 is the moment the test runs, so the outcome is known
  there, and the report needs no new reporting point. Count: 1 added (one clause).
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent".
- **h. The proposal.** Point 3: the commits name each claim of the spec with its test and its
  outcome, and how the test was shown to fail.
- **Default**: handle now.
- **Outcome**: approved.

## W6. A claim of completeness over a corpus is tested by a phrase search, and misses sites

- **a. What it says.** In six review rounds a reviewer found sites that a classification or an
  inventory had missed, from a phrase split over a line break, or a site that routes to the owner in
  other words. Each time the commit message said the list was complete. The fix proposed: the
  planning skill says a claim of completeness over a corpus is tested by reading each file of the
  corpus whole, by a fresh agent if needed, never by a phrase search alone.
- **b. Whether it still applies.** Yes. A search of the installed text for a rule on testing a claim
  over a corpus finds none. The root `CLAUDE.md` has one cause of it for commit messages only:
  "A search for a phrase is run so that it matches across a line break", at
  `instructions@verify-before-relying`.
- **c. Whether it is already recorded.** No. It is near P2 of
  2026-10-13-knowledge-architect-owner-authority-milestone, whose rule is the root's.
- **d. Its kind.** Missing text: the default behaviour, a phrase search, is systematically wrong
  for this claim, since wrapped prose splits a phrase and a site may use other words. Six instances
  in one session.
- **e. Its scope.** The agent-skills Component, point 3 of
  `skill@knowledge-architect-planning@working-a-slice`, where a claim's test is chosen. It serves
  `goal@knowledge-architect@agents-get-a-complete-workflow`. It reverses no head.
- **f. A better fix.** Rivals: the sentence proposed; a sentence in the primer, read by every
  session, which `design@agent-skills@primer-limit` keeps to what no skill delivers at the moment;
  or no change. The planning skill is where a claim's test is chosen. Count: 1 added.
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent".
- **h. The proposal.** Point 3 of `skill@knowledge-architect-planning@working-a-slice`: a claim that
  holds over every site of a corpus is tested by reading the corpus, a phrase search being a lead
  that misses a phrase split over a line break and a site in other words.
- **Default**: handle now.
- **Outcome**: approved.

## W7. The order of the last reviews multiplies review rounds

This section also holds W4 of 2026-10-13-thaum-slice-text-merge-klarch-workflow, where one finding
cost three reviews of the decision-record axis through the rule that a touched head is brought to
the rules, reported as cost with no defect claimed.

- **a. What it says.** 37 review agents over 5 pull requests. Three extra rounds were each forced
  by a repair that touched a head, before the transcript review could run; a deletion that waited
  for a ruling needed a review of its own; from slice 2 on, the session ran the work review and the
  harvest review as one round, and said it was a deviation. The fix proposed: measure first, with
  the counts that `issue@agent-skills@the-retrospective-counts-no-review-cost` asks for, and if the
  rounds mostly re-confirm, let the decision-record review of a head repair run in the same batch as
  the transcript review.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/skills/review/SKILL.md`:44, the
  transcript review "alone and last", and lines 144-149, a repair that adds, removes or reverses a
  head reviewed by the decision-record axis. The thaum finding's optional fix, exempting a head
  touched only by a re-pointing, already stands: `path@agent-skills@content/PRIMER.md`:420-422,
  "Re-pointing a reference in a head does not touch it."
- **c. Whether it is already recorded.** The measurement is
  `issue@agent-skills@the-retrospective-counts-no-review-cost`. The batching change is not.
- **d. Its kind.** An observation of cost, with no defect claimed in either file.
- **e. Its scope.** The agent-skills Component, the review skill's order, if the measurement shows
  it.
- **f. A better fix.** The files propose the measurement first, which is the open issue. A change of
  the order before it would be argued from two sessions' impressions.
- **g. Its route.** Nothing now; the issue's own route.
- **h. The proposal.** No change. The review-cost issue gains this file's figures, re-taken: 37 review
  agents over 5 pull requests, as the file states; 13 of the 30 commit subjects from the milestone
  document's commit to the last commit of its last slice begin with `[review]`, by
  `git log --format=%s` over that range.
- **Default**: no change; the review-cost issue gains the figures of both files.
- **Outcome**: approved.

## W8. A received file sent in part to a design discussion leaves its other findings with no home

This section also holds P3 of 2026-10-13-knowledge-architect-owner-authority-milestone.

- **a. What it says.** The owner chose to handle two of a received file's five findings by a design
  discussion. No text says what becomes of the rest. The session opened an issue for the other three
  after the milestone's review flagged the gap. W8 proposes nothing for the installed side. P3
  proposes one sentence beside this repository's knowledge row: such a file keeps the rest in an
  issue entry until an intake takes them, and the analysis is owed only for those.
- **b. Whether it still applies.** The knowledge row in the root `CLAUDE.md`, at
  `instructions@where-knowledge-goes`, and `design@knowledge-architect@committed-findings-analysis`
  give every received file an analysis. The head covers "A finding handled now that grows into a
  design discussion", which is a finding that already has an analysis; neither covers a hand-off made
  before any intake.
- **c. Whether it is already recorded.** The case's own outcome is
  `three-findings-of-a-received-retrospective-await-an-intake`, which this
  intake closes, in 2026-10-13-thaum-slice-numbers-merge-klarch-workflow.
  `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` is the installed-side
  question, and does not cover a partial hand-off.
- **d. Its kind.** An observation: the intake already has the route the case needed. A finding the
  owner sends to a design discussion takes the outcome "handle now" under
  `skill@knowledge-architect-design`, and its outcome line is rewritten to name the plan document
  that carries it, per `skill@klarch-retrospective-intake@after-the-ruling`. Running the intake on
  the whole file first would have given the other three findings their analysis, with no issue
  between.
- **e. Its scope.** This repository's own configuration.
- **f. A better fix.** Rivals: P3's sentence, which adds a second route beside the head; or no
  change, since the existing route covers the case once the intake runs on the whole file. P3's
  sentence changes what the head asks, an analysis for every received file, so it is a change of
  the head with an existing route as its rival.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change: the intake's existing route, a finding handled now under the design skill,
  covers the case, and this intake closes the issue the case left.
- **Outcome**: approved.

## C1. A citation of a plan document that the same branch deletes is reported only after the deletion

- **a. What it says.** A commit message cited the milestone by its kind, in the commit right after
  the one that deleted it; `commits` reported it as unresolved, correctly. The file proposes nothing
  for the checker, and points to the session's side.
- **b. Whether it still applies.** The checker behaves as stated.
  `path@agent-skills@content/skills/planning/SKILL.md`:550-552 has the deleting commit cite the
  document by its kind, which resolves against the commit's parent.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** An observation: the checker judged the message against the right tree.
- **e.–g.** None.
- **h. The proposal.** No change.
- **Default**: no change, as the file proposes.
- **Outcome**: approved.

## C2. A reference that resolves, whose target no longer says what the citing text claims

- **a. What it says.** In two routing reviews, a reference still resolved but its target had been
  rewritten, so the citing sentence was false. The closing steps of the decision-recording skill
  cover it by hand. The file proposes a listing of the texts that cite an entry whose head changed
  in a range, such as a `--changed` option of `show` taking a range.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/skills/decision-recording/SKILL.md`:265-269:
  "for each head the change edits, whatever the edit, run `{{command}} show` on it and read every
  text that cites it … an edit that keeps the slug dangles no reference, so no check reports a
  citing text it made false." No command lists citing texts by a range.
- **c. Whether it is already recorded.** In part.
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` asks for a command that
  takes several references and lists the issue and tripwire entries citing them. It takes no range
  and lists no other citing text. No rejected alternative matches.
- **d. Its kind.** A missing capability of the checker, which the goal
  `goal@knowledge-architect@structure-and-workflow-work-together` names: "the tool computes the
  workflow's work lists". Two instances, caught by reviews.
- **e. Its scope.** The core Component, its command line. It serves that goal.
- **f. A better fix.** Rivals: a new issue for the range listing; the open issue widened to every
  citing text and to a range, which would make it one command for both work lists; or no change,
  the manual step being enough. The open issue's shape is undecided ("Nothing about the command's
  shape is decided"), so widening it adds a case to the same design rather than a second command.
- **g. Its route.** The open issue's own route, a design under `skill@klarch-development`.
- **h. The proposal.** `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`
  gains this case: the texts citing any entry whose head changed in a range of commits.
- **Default**: no new issue; the open issue named in c gains this case.
- **Outcome**: approved.

## The standing answers: three rules acted on as absent

- **a. What they say.** The session gave `D` labels to seven defaults the record settled; it
  reported a rewording of a tripwire's clauses under "Smaller fixes" rather than "Decided"; it cited
  approvals as grounds in three rounds of the early slices. Reviews caught each.
- **b. Whether they still apply.** The `D` labels were in the milestone document as first committed,
  and were repaired in "[review] Repairs from the review of the owner-authority milestone: the
  contradictions between G2 and the source rule put to the owner, the defaults the record answers
  decided, and the quotes issue closed", before the primer's sentence "A label numbers what was put
  to the owner; it never names what decided" existed. The other two rules stand, clear, in
  `primer@decided-section` and `primer@design-heads`.
- **c. Whether they are already recorded.** No; nothing to record.
- **d. Their kind.** The first predates its rule; the other two are lapses, each caught.
- **e.–g.** None.
- **h. The proposal.** No change.
- **Default**: no change.
- **Outcome**: approved.

## The tripwires the file comes near and does not meet

- `tripwire@agent-skills@member-beyond-the-argument`: W2's members added by a harvest are not
  reported as recorded directly as covered by the head's argument.
- `tripwire@agent-skills@head-created-without-deliberation`: W3's defaults were deliberated in the
  milestone document.
- `tripwire@agent-skills@owner-intent-stripped`: W1 reports scope widened, not the owner's ground
  removed.
- `tripwire@agent-skills@ruling-lost-in-assembly`: the file reports attributions in heads, not a
  ruling misstated in a committed plan document after its own reviews.
- `tripwire@agent-skills@ruling-lost-in-change`: the session took the full path.
- `tripwire@agent-skills@argued-closure-reopened`: the owner reopened no thread closed `argued`;
  W4's defaults were approved, not reopened.
- `tripwire@agent-skills@record-answered-questions-reach-the-owner`: the seven `D` labels predate
  the decision's landing, and the file reports no later question to the owner that the record
  answered.
- The trigger of `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`:
  no reviewer judged without a deleted spec.
- The closing condition of `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search`:
  no analysis was pending in this repository during the session.

- **Default**: none fired or met.
- **Outcome**: approved.
