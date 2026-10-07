# Analysis of 2026-10-06-thaum-workflow

- **Received file:** 2026-10-06-thaum-workflow.md, the workflow's side of a retrospective of a
  project that uses the workflow. The project's own file was read for grounding only.
- **Version it used:** 0.4.0.
- **Analysed against:** main at v0.4.0-41-gf5d403a, plus the commit "A received retrospective file is
  analysed under klarch-retrospective-intake, the retrospective names its files by subject, and the
  searcher's groups are balanced".
- **Standing entries that bear on it:** `tripwire@agent-skills@plain-text-pointer-found` (C1),
  `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check` and
  `issue@knowledge-architect@command-output-is-not-declared-a-contract` (W4, W5),
  `tripwire@agent-skills@expectation-set-closes-a-contradiction` (W1, W2: neither is closed by an
  expectation set here).

## W1, with W1 and W2 of 2026-10-07-thaum-mock-reduction-workflow: the review outcomes are stated in three places, and two of them disagree

- **Says.**
  - W1 here: planning §7 point 4, "A finding not repaired becomes an issue entry", contradicts
    review §3, whose third outcome is "nothing, where the finding is judged to need nothing, with
    the reason". A milestone that restated planning gave the opposite answer to the root
    instructions.
  - W1 of the mock-reduction file: review §3 offers no outcome for an item whose outcome turns on a
    ruling the owner has not given, so a session invented one ("That is the ruling").
  - W2 of the mock-reduction file: review §3 asks an outcome for "a remark that a defect predates
    the change", and the primer's table says such a fix takes "a commit of its own". Two sessions
    did it two ways.
- **Still applies.** All three.
  - `path@agent-skills@content/skills/planning/SKILL.md` lines 384-385: "A repair is a further
    commit, or folded where that skill says. A finding not repaired becomes an issue entry." The
    sentence has not changed since the commit that installed the skill.
  - `path@agent-skills@content/skills/review/SKILL.md` lines 109-115 list the three outcomes;
    lines 121-124 ask an outcome for every item, a predating defect included; only the transcript
    paragraph, lines 131-132, names the owner: "A ruling the reviewer finds misstated is the
    owner's, and is put to the owner." The primer is not named in review §3.
  - This repository's root `CLAUDE.md`, `## Git` point 4, also says "Any finding not repaired
    becomes an issue entry": the same two-outcome wording, on the project side.
- **Recorded.** No issue, tripwire or rejected alternative.
- **Kind.** Defects of the text: two installed instructions contradict (W1), and one instruction is
  missing (the two mock-reduction findings). A contradiction between installed instructions is
  always in scope.
- **Scope.** `goal@agent-skills@one-skill-per-activity` ("no two installed instructions a session
  cannot both obey") and `goal@knowledge-architect@the-owner-decides`. Owner: agent-skills, and
  this repository's root `CLAUDE.md` for its restatement. No recorded decision is reversed.
- **Better fix.**
  - For W1, the report proposes that planning restate review §3's outcomes. Rival: planning keeps
    its pointer, "per `knowledge-architect-review`", and drops the sentence. The pointer already
    stands in the same line, and a third copy of a list is a third place to drift. Proposed: drop
    the sentence.
  - For mock-reduction W1, the report proposes a fourth outcome, "put to the owner". Rival: an item
    whose outcome turns on a ruling the owner has not given is put to the owner before the record
    is written, and its outcome is then one of the three. The three outcomes are final states; "put
    to the owner" is a state that waits, and recording it as an outcome lets a record land with the
    question still open. Proposed: the rival.
  - For mock-reduction W2: one sentence in review §3, that a predating defect is routed by the
    primer's table, so it is repaired in a commit of its own, and the record names that commit by
    its subject. No rival found that is shorter.
  - The root `CLAUDE.md` restatement gets the same correction as planning.
- **Route.** A text edit whose design is settled, under the agent-skills `CLAUDE.md` for the
  installed text, and `knowledge-architect-agent-configuration` for the root `CLAUDE.md`.
- **Proposal.** Handle now: planning §7 point 4 drops its last sentence; review §3 gains the
  owner's case and the predating-defect sentence; root `CLAUDE.md` `## Git` point 4 says "becomes
  one of the outcomes of `knowledge-architect-review`". CHANGELOG `Workflow` entry, patch.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W2: planning §7 still gates a one-message ruling on the four things of reversal

- **Says.** Planning §7, a load-bearing gap at the audit, is "put to the owner in one message, with
  a default, where reversing it touches none of the four things the design skill names for the
  cost of reversal". The design skill says "The four decide whether the premortem runs, never which
  path is taken." Proposed fix: planning states its own condition and leaves the four to the
  premortem.
- **Still applies.** `path@agent-skills@content/skills/planning/SKILL.md` lines 368-370 hold the
  clause. The design skill's sentence is line 153. The clause was written beside a condition the
  design skill then had; `7c614bc`, in v0.4.0, removed that condition from the design skill on the
  owner's word and did not edit planning. `git log -G "touches none of the four"` lists only the
  three commits that wrote and moved it.
- **Recorded.** None.
- **Kind.** A defect: wording that is false since a reversed condition, a leftover of
  `design@agent-skills@in-change-path`. The two texts do not contradict word for word, since
  "path" in the design skill means the in-change or the full path, but planning gates on a test the
  design skill uses for nothing else.
- **Scope.** Agent-skills. It reverses nothing: it applies a reversal already made.
- **Better fix.** The report's: planning's condition is already in the same sentence, "a choice
  among shapes that can be stated in full, each with its consequence". Dropping the clause about
  the four is enough. No rival found.
- **Route.** A text edit, installed text.
- **Proposal.** Handle now: delete "where reversing it touches none of the four things the design
  skill names for the cost of reversal". CHANGELOG `Workflow` entry, patch, or none if judged a
  rewording.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W3: which anchor holds the entry that records a missing checked form

- **Says.** The primer says a gap of the checker "gets that entry in this project's own register";
  issue-tracking says "in the project that meets it". Neither says which anchor. The session
  invented a rule and credited it to these texts.
- **Still applies.** `path@agent-skills@content/PRIMER.md` lines 107-110 and
  `path@agent-skills@content/skills/issue-tracking/SKILL.md` lines 285-290 name no anchor; neither
  does `design@agent-skills@checker-syntax-without-backticks-names-its-gap`.
- **Recorded.** None.
- **Kind.** Unclear, and its cost was a false attribution caught by a reviewer.
- **Scope.** Agent-skills. Bears on `goal@agent-skills@installed-text-leaves-room-to-judge`: the
  question is whether a rule is needed at all.
- **Better fix.** The report offers "the anchor of the first pointer that needs it" or "any anchor".
  The entry's every site is listed by `show`, so which anchor holds it changes nothing a session
  does with it. Proposed: "any anchor of the project", stated, so the next session does not invent a
  rule. Rival: the anchor of the first pointer, which imposes a mapping with no consumer.
- **Route.** A text edit, installed text: one clause in issue-tracking.
- **Proposal.** Handle now.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W4: a check made looser was filed under New features, and broke a consumer's tests

- **Says.** A 0.3.0 entry that narrowed a check sat under New features. A project whose extension
  tests counted the core's findings broke on it, and its procedure read only Migration entries.
  Proposed fix: the changelog files every change to what a check reports, in either direction, or
  to a command's output, under Migration.
- **Still applies, in part.**
  - The changelog rule is `design@knowledge-architect@changelog-entries`, the root's, not installed
    text. Under it, the 0.3.0 narrowing asked nothing of a consumer's files, so it was no Migration
    entry.
  - `path@agent-skills@content/skills/setup/SKILL.md` line 238 says "Any version but a patch may
    make a check stricter". A looser check is not named, and that part applies.
- **Recorded.** The class half is `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`.
  The output half is `issue@knowledge-architect@command-output-is-not-declared-a-contract`, whose
  design the owner wants in a discussion.
- **Kind.** The cause was a test that counted the core's findings. Since 0.4.0 the library
  documentation advises against it: "A test pins the set of findings it expects, not their count"
  (`path@core@src/lib.rs` line 183). A consumer following that advice is not broken by a narrowing.
- **Scope.** The root (changelog) and agent-skills (setup).
- **Better fix.** The report's would reclassify every narrowing as Migration, which tells a consumer
  to change files it need not change. Rival: setup's sentence says a check may become stricter or
  looser, and the instance is added to the bump-table question. The advice on counts already
  addresses the cause; its delivery is the next finding.
- **Route.** A text edit for setup; an issue update for the instance.
- **Proposal.** Handle now: "stricter" becomes "stricter or looser" in setup's "Moving the pin"; the
  instance is added to `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W5: the pin move runs the check and not the project's tests

- **Says.** Setup, Moving the pin, step 4: "`check`, and commit the pin, the installed files and the
  repairs together." 0.4.0 changed the wording of the `commits` summary; a test of the extension
  asserted the old word; only the test suite failed.
- **Still applies.** `path@agent-skills@content/skills/setup/SKILL.md` line 245, unchanged since
  0.4.0. §6 of the same skill, lines 168-176, recommends one command that runs every gate.
- **Recorded.** The trigger is an instance in
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`. The step is not recorded.
- **Kind.** Missing.
- **Scope.** Agent-skills; `goal@knowledge-architect@setup-brings-quality-tools`.
- **Better fix.** The report's, with a fallback, since §6 recommends the gates command and a project
  may not have one: "the project's gates command of §6, or the check and the tests where it has
  none".
- **Route.** A text edit, installed text.
- **Proposal.** Handle now.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W6, with W4 of 2026-10-07-thaum-mock-reduction-workflow: the advice on testing an extension reaches no installed skill

- **Says.** The library documentation says "A mock plants only the defects whose findings the
  extension emits" and "A test pins the set of findings it expects, not their count". No installed
  skill delivers it. Proposed fix: the setup skill's section on an extension carries both.
- **Still applies.** `path@core@src/lib.rs` lines 149-187 hold the section; the setup skill's
  extension text, `path@agent-skills@content/skills/setup/SKILL.md` lines 68-70 and 330-332, says
  nothing on tests. `grep -rn "plants only\|not their count"` over the installed text returns
  nothing.
- **Recorded.** None.
- **Kind.** Missing, at the point of delivery. It recurs: the advice was itself the fix of an
  earlier finding of the same project, landed in `ab62f99`, and two later retrospectives report
  that it did not reach the session that needed it. The 0.4.0 changelog has no entry for it either.
- **Scope.** Agent-skills and core. `ab62f99` placed the advice in the library documentation on the
  owner's word.
- **Better fix.** The report's copies the advice into setup, a second home that will drift. Rival:
  setup's extension section names the section "Testing an extension" of the core's crate
  documentation, in one sentence, as the reading owed before writing an extension's tests. It keeps
  one home and fixes the delivery.
- **Route.** A text edit, installed text.
- **Proposal.** Handle now: the pointer sentence in setup.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## W7: a reviewer called a history edit of the branch the owner's call

- **Says.** The decision-record reviewer wrote "editing its message again is the owner's call",
  against review §3's rule that a message is repaired by amend or by a history edit of the branch.
  Proposed fix: the reviewer repeats review §3's rule.
- **Still applies.** The blamed text does not exist: `grep -rn "owner's call"` over the installed
  text returns nothing. The reviewer improvised it. The review skill gives every outcome to the
  dispatcher, lines 117-125.
- **Recorded.** None.
- **Kind.** A lapse of one reviewer, once. What the transcript reviewer then flagged, an item with no
  outcome, is an instruction that exists.
- **Scope.** Agent-skills.
- **Better fix.** No change. Copying one rule into one agent would leave the six others without it,
  and one occurrence does not show a systematic default.
- **Route.** Nothing.
- **Proposal.** No change: a lapse, once.
- **Default:** no change.
- **Outcome:** approved.

## C1, with C2 of 2026-10-07-thaum-mock-reduction-workflow: a pointer into another package

- **Says.** A pointer at a file of a published dependency was written with the escape anchor and the
  path src/lib.rs, refused because a file of that path exists in the tree, and then written as prose. Proposed fix: a
  form for a file of another package, or a narrower refusal of the escape anchor. The
  mock-reduction file reports the same need for a section of a dependency's documentation.
- **Still applies, in part.**
  - A checked form existed: the escape anchor accepts a path the tree does not hold, so
    `path@elsewhere@knowledge-architect/src/lib.rs` passes. The test
    `the_escape_anchor_is_exempt_unless_its_target_resolves_here`, in
    `path@core@src/check/references.rs` at line 1766, pins that shape, and both were in 0.4.0. So no
    form was missing.
  - The narrower refusal would reverse `design@core@reserved-anchors`, whose argument is that the
    escape must not silence a real path.
  - What applies: the refusal's repair says "anchor the reference at the anchor that holds it". It
    assumes the writer meant the local file, and does not say that another project's file takes a
    distinguishing first segment.
  - For the mock-reduction case: since `9b0e47d`, after 0.4.0, a description in words is outside
    the plain-text rule, per `design@agent-skills@checker-syntax-without-backticks-names-its-gap`.
- **Recorded.** `issue@core@cross-project-references` records references to another project's
  entries, not paths. No rejected alternative.
- **Tripwire.** `tripwire@agent-skills@plain-text-pointer-found` fires on "a pointer written as bare
  plain text to clear a finding". Here a refused reference was replaced by a description in words.
  Under `design@agent-skills@plain-text-is-no-repair`, "a rewrite of the sentence" is a repair, and
  since `9b0e47d` a description in words is outside the escape rule. Judged: it does not fire. The
  cost was real all the same: a checked form existed and the repair text did not lead to it.
- **Kind.** The checker's repair text, not a missing form.
- **Scope.** Core: `design@core@finding-names-the-repair`.
- **Better fix.** The refusal's repair names both moves: anchor it where the tree holds it, or, for a
  file of another project, prefix the path with that project's name. No reversal; the second line
  of the report's fix is rejected as one.
- **Route.** Rust, `klarch-development`, core. CHANGELOG: no entry (a finding's text, no check
  changed).
- **Proposal.** Handle now: the repair text of the escape-anchor refusal.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## C2: the commit-message checks did their job

- **Says.** An observation: `commits` caught two unanchored paths and a malformed reference in
  messages, with no false positive.
- **Kind.** An observation.
- **Proposal.** No change.
- **Default:** no change.
- **Outcome:** approved.

## The standing answer on the primer in subagents, with the same answer in every other received file

- **Says.** "Whether it was in their context from their first token cannot be observed from the
  transcript." The other three workflow files answer the same question with "not verified",
  "assumed" or "partly shown".
- **Still applies.** The question is `path@agent-skills@content/skills/retrospective/SKILL.md` line
  75, unchanged.
- **Recorded.** None. The question watches the import-line delivery of the primer, per
  `design@agent-skills@premortem-as-watch-points`.
- **Kind.** A defect of the question: four of four sessions could not answer it from what they can
  see, so it watches nothing.
- **Scope.** Agent-skills; `goal@knowledge-architect@the-workflow-improves-through-real-use`.
- **Better fix.** Ask what a session can observe: whether any subagent's report or behaviour showed it
  lacked a rule of the primer. Rival: the review skill's brief names the primer's path, so delivery
  no longer depends on the harness. Which of the two is a design question, since the second changes
  every brief.
- **Route.** A design discussion, small.
- **Proposal.** Open an issue: agent-skills, `question`.
- **Default:** open an issue.
- **Outcome:** approved.

## Standing entry: whether `tripwire@agent-skills@plain-text-pointer-found` fired on C1

- **Evidence.** C1's pointer was refused with the escape anchor, then written as a description in
  words, with no reference to an issue entry beside it. The tripwire fires on "a pointer written as
  bare plain text to clear a finding".
- **Judgement proposed.** It does not fire. `design@agent-skills@plain-text-is-no-repair` lists "a
  rewrite of the sentence" among the repairs, and since `9b0e47d` a description in words is outside
  `design@agent-skills@checker-syntax-without-backticks-names-its-gap`. The cost was real all the
  same, and C1's repair text addresses it: a checked form existed and the refusal did not lead to it.
- **Default:** not fired; no response.
- **Outcome:** pending.
