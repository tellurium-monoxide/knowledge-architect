# 2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow

- **Received file**: 2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow.md, the
  workflow file of the retrospective of a session in thaum that moved its pin of the checker from
  0.5.0 to 0.6.0 and then ran pass 1 of the design-record axis of a project audit.
- **Version used**: the published checker, pinned at exactly 0.6.0.
- **Analysed against**: main at `v0.6.0-75-g5fca230`.
- **Standing entries that bear on it**: one search over the 83 issues and 58 tripwires covered the
  seven files received together. For this file it returned:
  - `an-audit-report-cites-the-heads-its-own-pass-removes`, the defect W5
    reports again;
  - `audit-edit-leaves-an-entry-stale`, whose firing clause W3 meets, judged
    in its own section;
  - `tripwire@agent-skills@attribution-loses-the-scope`, judged in the attribution cluster below,
    with the evidence of two other files;
  - `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, which bears on that
    judgement;
  - `tripwire@agent-skills@audit-defaults-overruled` and
    `tripwire@agent-skills@verifiers-wave-findings-through`, whose counts the file does not carry,
    judged in their own section;
  - the tripwires of the design-record axis the file does not meet, judged together in the last
    section.

## W1, W2. Nothing names the corpus or the means of the judgement of what the application writes

- **a. What it says.** W1: the session's application of pass 1 touched about 200 heads and
  entries, far more than the run's checks read: retitles, re-points, merged heads, text a check's
  repair rewrote, heads the owner's rulings created. The audit skill says each head the application
  creates beyond what the run checked "is judged against every rule", with no step, agent or brief.
  The session dispatched four fresh auditor agents briefed to check, on every entry whose text
  differed between the audited commit and the application's commit, against its old text and the
  owner's rulings. They found 19 failures the run's checks could not have seen, among them a retitle
  that made a title true of a recorded rival, and an attribution wider than the owner's argument.
  The owner judged the step worth proposing. W2: the run's check judges "each head that a verified
  split, merge, rename or move creates"; one check judged a retitle as created, another did not,
  and seven retitles went unjudged until the step of W1. The fix proposed: a judge step after the
  application whose corpus is every entry whose text differs, judged against every rule and against
  its old text.
- **b. Whether it still applies.** Yes, both, unchanged since 0.6.0.
  `path@agent-skills@content/skills/project-audit/SKILL.md`:190-193: "Each head the session's
  application creates beyond what the run checked is judged against every rule as an entry of its
  own, before the commit: moving text unchanged does not make the new head conform." It names no
  corpus beyond "creates" and no means. Line 166 gives the run's check "each head that a verified
  split, merge, rename or move creates", and
  `path@agent-skills@content/agents/design-record-auditor.md`:176-177 the same list. None names a
  retitle or a body edit.
- **c. Whether it is already recorded.** No issue or tripwire. The head that states the duty,
  `design@agent-skills@created-heads-checked-in-the-run`, ends "The session judges alone each head
  its application creates beyond what the run checked", and names as the rival that lost "that
  judgement made at the application", on the ground that it would come after the owner's ruling.
  The fix keeps the run's check before the ruling and adds a judgement after the application, so it
  is not that rival. No rejected alternative bears on it.
- **d. Its kind.** Unclear text, and a missing corpus: "creates" is read two ways for a retitle,
  and the duty names no reader. The 19 failures are real-use evidence that the judgement matters at
  the scale of a pass.
- **e. Its scope.** The agent-skills Component: the audit skill, the auditor agent's check brief,
  and the head above, which says the session judges "alone". It serves
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Giving the judgement to agents
  and widening its corpus to every changed entry changes a statement of
  `design@agent-skills@created-heads-checked-in-the-run`, so the fix is a change of that head.
- **f. A better fix.** Rivals:
  - the re-check of point 4 of `skill@knowledge-architect-project-audit@audit-method`, which runs
    the saved workflow again after the edits and catches "a violation left, whether a draft missed
    it or an edit made it". It does not run when the owner stops the audit after a pass, as here,
    and it reads every entry, not the changed ones.
  - a new step in the saved workflow. The application is the session's, after the run, so the
    workflow cannot hold it.
  - the sentence at lines 190-193 rewritten to name the corpus, every entry whose text the
    application changed, retitles and body edits included, and the means, the design-record
    auditor under its check brief, which the session dispatched here. One sentence closes both W1
    and W2.

  The third is the fix: it rewrites an existing instruction and adds none (count: 0 added, 0
  removed). The fact that decides against the first is that the re-check does not run at the stop
  of an audit.
- **g. Its route.** A decision: the head's "judges alone" changes, so the change goes through
  `skill@knowledge-architect-design` on its in-change path, then the text edit of the installed
  source under the agent-skills `CLAUDE.md`, "Editing an installed skill or agent".
- **h. The proposal.** Rewrite the application sentence of the design-record axis and its head:
  after the application, every entry whose text differs between the audited commit and the
  application's commit is judged against every rule and against its old text, by the design-record
  auditor under its check brief, before the commit.
- **Default**: handle now, under the design skill's in-change path.
- **Outcome**: approved.

## W3, and `audit-edit-leaves-an-entry-stale`

- **a. What it says.** The cross-check maps "the tripwires and issues an edit would leave false".
  It missed two that the standing-state reviewer found on the audit's branch: a tripwire whose
  firing evidence was a member the retitled head now admits as open, so it fires on expected
  growth; and a clause guarding a decision that a split moved to a new head, left under the old
  head's tripwire. Both resolve, so the checker reports neither. The fix proposed: the cross-check
  reads every tripwire of every head the pass splits or retitles, clause by clause, against the
  head that now holds what the clause guards.
- **b. Whether it still applies.** In part. The scope the fix asks for is already instructed:
  `path@agent-skills@content/workflows/design-record-audit.js`:338-340 asks the cross-check for
  "each tripwire and each issue that cites a head some verified finding edits, read whole … and
  whether the edit leaves it false". The clause-by-clause reading against the head that now holds
  the guarded part is not instructed.
- **c. Whether it is already recorded.** It meets the firing clause of
  `audit-edit-leaves-an-entry-stale`: "a review of an audit's branch finds a
  tripwire guarding, or an issue naming, a head an edit of the design-record axis changed, left
  false." The second tripwire is plainly left false: its guard sits under a head that no longer
  holds the guarded decision. The tripwire's re-entry, "the standing-state review of every audit's
  branch", names no project, and the decision it guards is installed text that thaum ran. Its
  response: "reopen `design@agent-skills@design-record-axis`".
- **d. Its kind.** Missing text: the reading the cross-check is asked to make did not reach a
  clause whose guarded decision moved. It is also the firing of a tripwire this Component wrote for
  exactly this premise.
- **e. Its scope.** The agent-skills Component, `design@agent-skills@design-record-axis` and the
  cross-check's prompt in the saved workflow. It serves
  `goal@knowledge-architect@agents-work-without-drift`.
- **f. A better fix.** Rivals: the clause-by-clause reading in the cross-check, as proposed; a
  checker listing of tripwires by the head they guard, which already exists as
  `cargo klarch tripwires --guarding`, and does not judge a clause; or no change, with the
  standing-state review as the catch, which found both here. The tripwire's response is a
  reopening, which weighs these in a design discussion rather than in this analysis.
- **g. Its route.** `skill@knowledge-architect-design`, reopening the head, as the tripwire's
  response says.
- **h. The proposal.** Rule the tripwire fired, on the second tripwire the file reports. Its
  reopening is carried by a `design` issue in agent-skills that names this firing, W3's proposal and
  the rivals above, with the next pass of the design-record axis as its re-entry point.
- **Default**: open an issue, `design`, in agent-skills, carrying the fired tripwire's reopening.
- **Outcome**: approved.

## W4. Recording test 1 for a losing alternative says "a serialized format", where the primer's test 1 says an interface others consume

- **a. What it says.** Test 1 of the losing alternatives reads "it would change a signature
  crossing the boundary of a separately built unit, or a serialized format". Read literally, every
  field of a serialized state is a format, and nearly every internal alternative passes. The
  verifiers split on several entries on this alone. The fix proposed: say whether adding or
  removing a field counts.
- **b. Whether it still applies.** Yes.
  `path@agent-skills@content/skills/decision-recording/SKILL.md`:179-180, unchanged since 0.6.0.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Unclear text, from a restatement that drifted: the primer's entry test 1, in
  `primer@design-heads`, reads "reversing it would change **an interface others consume**: … a file
  format, a document grammar, a command line". The primer says who consumes the format; the losing
  alternatives' test does not, and that is the reading the verifiers split on.
- **e. Its scope.** The agent-skills Component, `skill@knowledge-architect-decision-recording`. It
  serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`. It reverses no head: it
  brings a restatement back to its home.
- **f. A better fix.** The fix proposed adds a rule for fields. The cause is a restatement that
  drifted, and the repair by cause is the restatement brought back to its home: test 1 states "it
  would change an interface others consume, as test 1 of `primer@design-heads` defines it". A field
  of a type nobody outside the unit reads is then not an interface others consume, which answers
  the question with no rule about fields. Count: 0 added, 0 removed.
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent".
- **h. The proposal.** Rewrite test 1 of `skill@knowledge-architect-decision-recording@losing-alternatives`
  as a pointer to the primer's test 1, as in f.
- **Default**: handle now.
- **Outcome**: approved.

## W5. The report file's references go stale when the pass applies its edits

- **a. What it says.** The audit skill says "A span the checker would refuse is quoted in words".
  Spans the checker accepts when the report is written stop resolving once the pass removes or
  renames what they name, so a commit between the application and the file's deletion fails the
  per-commit gate. The session deleted the file in the commit that applied the last edit. The fix
  proposed: the report names in words the entries the pass removes or renames, or the deletion
  lands in the application's commit.
- **b. Whether it still applies.** Yes.
  `path@agent-skills@content/skills/project-audit/SKILL.md`:126-127, unchanged since 0.6.0.
- **c. Whether it is already recorded.** Yes:
  `an-audit-report-cites-the-heads-its-own-pass-removes` is the same defect, seen
  in this repository on the first pass of the same axis, where the session repaired the dangling
  spans by removing their backticks. This is a second instance, in a second project, with the
  other workaround. Its closing condition is the first fix proposed: "The audit skill states how a
  report file treats a reference that its own pass's edits make dangle: a description in words of
  the head, as the skill's sentence means, or another form the checker judges."
- **d. Its kind.** Missing text, shown twice in real use.
- **e. Its scope.** The agent-skills Component, `skill@knowledge-architect-project-audit@record-of-an-audit`.
  It serves `goal@knowledge-architect@documentation-stays-consistent`. Whether a head states the
  file's spans is checked at the edit; `design@agent-skills@committed-audit-report` states when the
  file lands and leaves, not how it names a head.
- **f. A better fix.** Rivals: the deletion in the application's commit, which the skill already
  permits ("the commit that carries out its last outcome") and which thaum did, and which does not
  hold where the outcomes take more than one commit; or a description in words from the start,
  which holds whatever the commits are, and which `design@agent-skills@plain-text-is-no-repair`
  admits, since a description in words is outside the checker's syntax. The second is the fix.
  Count: 1 added (one clause).
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent", which closes the issue.
- **h. The proposal.** In `skill@knowledge-architect-project-audit@record-of-an-audit`: an entry
  the pass's own findings remove or rename is named in the file by a description in words, not by
  its reference. Close `an-audit-report-cites-the-heads-its-own-pass-removes`
  with it.
- **Default**: handle now, and close the issue with it.
- **Outcome**: handle now, under the design skill's in-change path, in the shape the owner gave, which replaces the default. The owner, in the file: "If the entry is removed, the reference can be kept with only the slug, in plain text. This is not checker avoidance, because the checker has nothing left to check there. A description in word would just be confusing to everyone. Maybe the text that forbids plain text checker syntax should be reworded to allow this case. What's forbidden is avoiding the checker on live entries, which would lead to dangling references and less efficient grounding." And, in the conversation, choosing the bare slug in backticks among three forms and the records of a deliberation as the scope: "agreed, with shape b. This holds only in records of deliberation, the list you proposed. Everything else is supposed to be current reality only, no stale refs."

## W6. Reading a report of 7 000 lines "whole"

- **a. What it says.** The verification begins "The session reads the report whole". The run
  produced 7 000 lines over 410 entries. The session read the header, the calibration, the
  owner-list draft, the overlap map and the verdict line of every finding, and dispatched four fresh
  verifiers on the items the verification names, re-running their load-bearing claims. They
  corrected eight of the run's defaults, two of which rested on a premise the record defeats. The
  fix proposed: say whether the verification may be delegated, and what the session must read
  itself.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/skills/project-audit/SKILL.md`:182,
  and `design@agent-skills@one-run-pass`: "the session verifies the report … since the owner list
  goes out under its responsibility: the report whole, …".
- **c. Whether it is already recorded.** No issue. The nearest rejected alternative, "The session
  reading every draft again after the run", lost to `design@agent-skills@one-run-pass`; it asks for
  more reading by the session, not for delegation.
- **d. Its kind.** Unclear at scale: an instruction the session could not follow as written at
  this size, and replaced with a delegation that worked. One instance.
- **e. Its scope.** The agent-skills Component. Delegation changes the clause of
  `design@agent-skills@one-run-pass` that the session reads the report whole, on the ground that
  the owner list goes out under its responsibility, so it is a change of that head.
- **f. A better fix.** Rivals: delegation with a stated part the session keeps, as proposed; a
  shorter report, which the saved workflow's report agent could produce, and which moves the
  problem rather than removing it; or no change until a second pass of this size shows it again.
  The fact that decides is the evidence: one pass, in which the delegation found eight defaults to
  correct. One instance does not establish what the session must keep reading itself.
- **g. Its route.** `skill@knowledge-architect-design`, since the head changes.
- **h. The proposal.** An issue that records the instance, the delegation and its result, and the
  question for the head, with the next pass of either axis whose report the session cannot read
  whole as its re-entry point.
- **Default**: open an issue, `design`, in agent-skills.
- **Outcome**: Skip this one. It should happen rarely, this was a big migration which sould not happen again, as the design record condition converge.

## W7. A fix for a commit of its own landed in the pass's commit, through the application agents

- **a. What it says.** Row 2 of `primer@met-outside-the-task` sends a fix checkable from the diff
  alone to a commit of its own. Application on disjoint paths put three such fixes in the agents'
  diffs, and they landed in the pass's commit. The fix proposed: the application step keeps row-2
  fixes out of the agents' briefs.
- **b. Whether it still applies.** The text is unchanged. `path@agent-skills@content/PRIMER.md`:120,
  row 2: "fix it, in a commit of its own".
  `path@agent-skills@content/skills/project-audit/SKILL.md`:81-82, point 3: "Apply the edits,
  yourself or through agents dispatched on disjoint files. Read the whole diff before each commit."
- **c. Whether it is already recorded.** No.
- **d. Its kind.** A lapse of the session. Row 2 binds the session, which wrote the agents' briefs
  and read the whole diff before the commit, as point 3 asks; the three fixes were in that diff.
  One instance.
- **e. Its scope.** None beyond the session.
- **f. A better fix.** The rival is the clause proposed, in point 3. The lapse rule of
  `skill@knowledge-architect-agent-configuration@admission-standard` keeps a clear instruction as it
  is unless the lapse recurs at one point of delivery, and this is one instance.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of one session against a clear instruction.
- **Outcome**: approved.

## W8. The pair of a thread's name and its slug when a split gives the thread's second half a head of its own

- **a. What it says.** A split head kept its thread's name, and the other half, from the same
  thread, took another slug. The rule does not say whether a pair is owed for the second. The
  session stated it in the commit. No fix is proposed.
- **b. Whether it still applies.** The text is unchanged. `path@agent-skills@content/PRIMER.md`:369-376:
  "its slug is the thread's name, unless that name misdescribes the decision as approved … the text
  that keeps the deliberation states the pair".
- **c. Whether it is already recorded.** No.
- **d. Its kind.** An observation. The rule covers the case by its own clause: the thread's name
  names the half that kept it, so it misdescribes the second half, which then takes a slug of its
  own and owes the pair. The session did that.
- **e. Its scope.** None.
- **f. A better fix.** No change: the text admits the case.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, the rule's own clause covers the case and the session applied it.
- **Outcome**: approved.

## W9. One auditor draft applied the code-change test backwards

- **a. What it says.** The test reads "if changing a piece of code would force a change to the
  design home, it is design". One draft read it as removing a statement the code does not force. A
  verifier caught it. The fix proposed: the auditor's definition quotes the test with an example of
  each direction.
- **b. Whether it still applies.** The text is unchanged. `path@agent-skills@content/PRIMER.md`:347-348.
  The auditor's definition sends to `primer@design-heads` whole and quotes neither the test nor an
  example.
- **c. Whether it is already recorded.** No. `tripwire@agent-skills@rule-misread-across-groups`
  fires on findings "in at least three groups" traced to one misreading; this is one draft.
- **d. Its kind.** A lapse of one draft, caught by the verify step, which exists for that.
- **e. Its scope.** None beyond the run.
- **f. A better fix.** The rival is the quotation and examples, proposed. The verify step caught
  the draft, and the tripwire above watches the case where a misreading spreads.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, one draft caught by the step built to catch it.
- **Outcome**: approved.

## W10. The session put items to the owner on premises it had not checked

- **a. What it says.** `primer@owner-word-premise`: "A premise the session supplied is the
  session's to check before the owner rules on it." Twice in 42 items, the session put an item to
  the owner on a premise it had not checked: that a plan line held the owner's verbatim words, and
  that a definition passed entry test 2. A review caught both after the owner ruled, and each went
  back to the owner. The fix proposed: the owner-list step names the premise each default rests
  on, and the verification checks each before the list is sent.
- **b. Whether it still applies.** The rule stands at `path@agent-skills@content/PRIMER.md`:43-45.
  The owner list, `path@agent-skills@content/skills/project-audit/SKILL.md`:73-80, asks each item
  for "a default and the words of the entry or the text the item turns on", not for its premise.
- **c. Whether it is already recorded.** No. `tripwire@agent-skills@audit-defaults-overruled`
  counts rulings against the default, not unchecked premises.
- **d. Its kind.** A lapse: the file calls the rule clear, and its failure one of scale.
- **e. Its scope.** The agent-skills Component, if anything changed.
- **f. A better fix.** The rival is a premise line per item of the owner list, a structure on
  every item for two failures in 42. The lapse rule keeps a clear instruction as it is, and the
  primer delivers it in every session. The review after the ruling caught both, and the primer's
  procedure brought each back to the owner.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of a clear rule, caught and routed by the procedure it names.
- **Outcome**: approved.

## C1. The crate documentation says an extension is configured once per run

- **a. What it says.** At 0.6.0, `check --fix` configures the extension over the staged manifest
  and then checks the working tree, and `commits` configures it once per commit. A project's
  extension that kept the configuration of its last resolve judged the working tree with the
  staged tree's table. The fix proposed: the documentation says an extension may be configured once
  per tree it judges, and that `prepare` receives each tree's own manifest.
- **b. Whether it still applies.** Yes, at one sentence. `path@core@src/extension.rs`:9, the module
  documentation docs.rs renders: "An [`Extension`] is configured once per run." The code configures
  it again for each snapshot tree: `path@core@src/cli/history.rs`:303, "Configured against this
  commit's own manifest". The rest of the documentation already says "once per manifest": the same
  file at lines 247 and 252, `path@core@src/lib.rs`:117, and the head
  `design@core@an-extension-plugs-in-through-phased-hooks`, whose design home states "An extension is
  configured once per manifest and prepared once per tree".
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Wrong text: one sentence of a doc comment that diverges from the design home and
  from the code, which agree.
- **e. Its scope.** The core Component, its published API documentation. It serves
  `goal@core@projects-add-their-own-checks`.
- **f. A better fix.** The proposed wording adds that `prepare` receives each tree's own manifest,
  which lines 247 and 252 already carry. The repair is the one sentence brought to "once per
  manifest". No rival: the head and the code agree.
- **g. Its route.** A change to the Rust source, a doc comment, under `skill@klarch-development`.
- **h. The proposal.** Reword `path@core@src/extension.rs`:9 to "configured once per manifest",
  and say in the same sentence that `commits` judges several manifests in one run.
- **Default**: handle now.
- **Outcome**: approved.

## C2. No checked form names a head a document proposes and that does not exist yet

- **a. What it says.** The audit's report file names heads the pass would create. No form
  expresses "a head this pass will create", and a placeholder in angle brackets is for
  illustrations. The session wrote them as bare code spans of one segment. No fix is proposed.
- **b. Whether it still applies.** Yes, no such form exists. The only planned form is for a path
  in a plan document, `design@core@planned-path-form`.
- **c. Whether it is already recorded.** No issue. The nearest record is the rejected alternative
  "A planned path legal in every document", which lost to that head; it concerns paths.
- **d. Its kind.** An observation. A slug-to-be written as a one-segment span is a name, which
  `design@core@candidate-rule-and-retired-forms` leaves silent by design, and it becomes the head's
  slug when the head is written. The primer's rule on a pointer written without backticks concerns
  a reference or a path that no checked form expresses, and a head not yet written is neither. The
  session's spans broke nothing and dangled nothing.
- **e. Its scope.** The core Component, if a form were wanted.
- **f. A better fix.** Rivals: a planned form for an entry, as for a path; or no change. Nothing in
  the file shows a cost of the bare name.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change: a proposed slug is a name, and nothing broke.
- **Outcome**: handle now, with W5 of this file, in the change the owner gave there. The owner, in the file: "See my note on W5 in this file, this is pretty much the same case, and we should do the change I mentionned so we no longer receive such findings". And, in the conversation: "agreed, with shape b. This holds only in records of deliberation, the list you proposed. Everything else is supposed to be current reality only, no stale refs."

## The attribution cluster: an attribution of the owner widened or stripped of its hedges

This cluster holds three items: the second standing answer of this file, the second and third
bullets of the standing answers of 2026-10-13-knowledge-architect-agent-text-admission-klarch-workflow,
and W1 of 2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow.

- **a. What it says.** Here, an application agent widened two of the owner's rulings, in an
  attribution and in a clause added to an attributed head; the judges and the transcript reviewer
  caught both. In the agent-text admission session, the session wrote verbatim quotations of the
  owner into heads at the harvest, and widened two attributions and dropped their hedges; reviews
  caught all of them before the merge. In the owner-authority milestone, a reviewer found an
  attribution that dropped a hedge, widened the scope or credited the owner with the agent's
  argument in 9 of 14 review rounds, 5 of them after the rule on attributions was written. The fix
  proposed there: a step in `skill@knowledge-architect-decision-recording@before-you-finish` that
  puts the owner's words beside each attribution the change writes and checks that every hedge and
  qualifier is kept.
- **b. Whether it still applies.** Yes. The rule is at `path@agent-skills@content/PRIMER.md`:328-329:
  "An attribution keeps the scope and the hedges of the owner's words, and is checked, like a
  quotation, against the question it answered." The closing steps of the decision-recording skill,
  `path@agent-skills@content/skills/decision-recording/SKILL.md`:250-273, name no attribution, no
  owner's word and no hedge. At 0.6.0, which this file's session ran, the primer held no rule on
  attributions; the rule entered after the release.
- **c. Whether it is already recorded.** In part. `tripwire@agent-skills@attribution-loses-the-scope`
  watches the class; its response reopens `design@agent-skills@attribution-replaces-quotation` on
  where verbatim words are owed, not on a check at writing time. The rejected alternative "Quoting
  the owner's words verbatim in a head whose ground is the owner's" lost to that head; the fix
  proposed keeps the words beside the attribution while it is written, not in the head, so it is
  not that rival.
- **d. Its kind.** A lapse that recurs at one point of delivery: across three sessions, in two
  projects (two widened rulings here; two widened attributions and verbatim quotations in the
  second; findings in 9 of 14 review rounds in the third), all at the moment a head or an
  attribution is written, while the rule is in the primer, read at the start of a session. The lapse rule of
  `skill@knowledge-architect-agent-configuration@admission-standard` repairs a recurring lapse by
  moving where the instruction is delivered, and names the recurrence as its evidence.
- **e. Its scope.** The agent-skills Component. The closing steps of the decision-recording skill
  are loaded before every write into a design home, per
  `design@agent-skills@design-home-write-loads-recording`, which is the moment of the failure. It
  serves `goal@knowledge-architect@the-owner-decides`. It reverses no head:
  `design@agent-skills@attribution-replaces-quotation` already says an attribution "is checked
  against the question it answered, as a quotation is".
- **f. A better fix.** Rivals:
  - no change, the reviews being the catch. They caught every instance, at the cost of a review
    round each, which `issue@agent-skills@the-retrospective-counts-no-review-cost` would measure.
  - quoting the owner in the head again: that lost.
  - the step proposed, at the moment of writing. It adds one instruction, and delivers an existing
    rule where it is needed.

  The fact that decides for the step is the recurrence after the rule existed: five review rounds
  in the session that wrote it, and two widened attributions in the next session. Count: 1 added.
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent"; a Workflow entry in the `Next release` section.
- **h. The proposal.** In `skill@knowledge-architect-decision-recording@before-you-finish`: for each
  attribution of the owner the change writes, read the owner's words from the plan document or the
  transcript beside it, and check that its scope and every hedge are kept.
- **Default**: handle now.
- **Outcome**: approved.

## `tripwire@agent-skills@attribution-loses-the-scope`, and the bound it lacks

- **The firing clause**: "the transcript reviewer reports a misstated ruling that traces to an
  attribution in a durable record, where the owner's words would have shown the scope."
- **The evidence.** This file's instance predates the guarded decision and does not count. In the
  agent-text admission session, the transcript review found two attributions wider than given, as
  the message of the commit "[review] Repairs of the transcript review: two attributions narrowed to
  the owner's words, an open question recorded" says; one traces to a quotation converted to an
  attribution at the harvest that dropped "most" and detached a clause, and the owner's words showed
  that scope. That meets the clause's letter. In the owner-authority milestone, the transcript
  review of slice 4 judged that the attribution tripwire had not fired. Every instance was repaired
  before its merge, and none changed what is built.
- **What bears on the judgement.** `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`
  lists this tripwire among five that fire on one instance with no bound, and says none had been
  met yet. On a sixth, the owner ruled a firing of that kind out: "The point of this kind of
  tripwires is to guard against critical failures in the workflow, not small mistakes that are not
  load bearing." The five were not brought to that bound; that is the issue's open question.
- **The proposal.** Judge it not fired: the slips changed nothing load-bearing and were repaired
  before the merge. Add the instance to the severity-bound issue, as the first observed case of
  its exposure, with the instance of `tripwire@agent-skills@rule-title-wider-than-its-argument` in
  2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow.
- **Default**: not fired; the issue on the severity bound gains the instance.
- **Outcome**: approved.

## `tripwire@agent-skills@audit-defaults-overruled` and `tripwire@agent-skills@verifiers-wave-findings-through`

- **The firing clauses**: "the owner rules against the default on more than a quarter of the items
  of the owner list", and "the session rejects more than a quarter of a report's confirmed
  findings".
- **The evidence.** The file gives 42 items on the owner list, and eight of the run's defaults
  corrected by the session's verification before the list went out. It gives neither the number of
  rulings against the default nor the number of confirmed findings. The audit skill puts those
  counts in the pass's commit message, which is thaum's.
- **The proposal.** Not judged fired: the file does not carry the counts. Eight corrections out of
  42 items, if they were rejections of confirmed findings, are under a quarter.
- **Default**: not fired on this evidence.
- **Outcome**: approved.

## The tripwires of the design-record axis the file does not meet

Each was read against the file:

- `tripwire@agent-skills@axis-not-run-at-a-pin-move`: this file is its re-entry, a retrospective
  of a session that moved a pin. The axis the 0.6.0 Migration entry cites, the design-record axis,
  was run, on the session's second branch.
- `tripwire@agent-skills@overlap-missed`: the file reports no two findings that edit one paragraph
  or one slug and that the overlap map did not list.
- `tripwire@agent-skills@conforming-draft-wrong`: the draft of W9 proposed a change; it was no
  conforming draft.
- `tripwire@agent-skills@rule-misread-across-groups`: one draft, not three groups.
- `tripwire@agent-skills@calibration-reading-rejected`, `tripwire@agent-skills@component-judgement-noisy`,
  `tripwire@agent-skills@verifier-refutes-a-true-finding` and
  `tripwire@agent-skills@record-check-misses-a-head`: the file reports nothing of their events.
- `tripwire@agent-skills@split-rule-from-exception`: the file reports merged heads, not a merge of
  two heads split under the decision, and no exception split from its rule.
- `tripwire@agent-skills@plain-text-pointer-found`: W5's session deleted the report file rather
  than turn its spans into plain text.

- **Default**: none fired.
- **Outcome**: approved.
