# 2026-10-13-thaum-slice-numbers-merge-klarch-workflow

- **Received file**: 2026-10-13-thaum-slice-numbers-merge-klarch-workflow.md, the workflow file of
  the retrospective of a session in thaum that ran one slice of a milestone through its merge.
- **Version used**: the published checker, pinned at 0.6.0.
- **Analysed against**: main at `v0.6.0-75-g5fca230`.
- **How this file reached the intake.** On the owner's direction, W1 and W2 were handled by the
  design discussion that produced the owner-authority milestone, with no analysis, and W3 to W5
  waited in `issue@knowledge-architect@three-findings-of-a-received-retrospective-await-an-intake`.
  This analysis covers the whole file.
- **Standing entries that bear on it**: from the one search over the 83 issues and 58 tripwires:
  - `issue@knowledge-architect@three-findings-of-a-received-retrospective-await-an-intake`, which
    this analysis closes, in its own section;
  - `tripwire@agent-skills@a-head-verdict-is-overruled`, whose firing clause W5 meets, judged with
    W5;
  - `issue@agent-skills@patching-an-installed-skill`, which the project's two overrides bear on, in
    its own section;
  - `design@agent-skills@standing-entries-searched-before-the-work`, a head W3's fix touches, and the
    tripwires judged together in the last section.

## W1, W2. The rank of a specification's text against the owner's answer, and the transcript reviewer's ruling check

- **a. What it says.** W1: the primer gave an external specification no rank against the owner's
  word, so a reading the specification answered was put to the owner, recorded as a ruling, and
  later handled by the premise procedure, with an option the specification forbids. W2: the
  transcript reviewer reported an answer of the owner on a point the specification governs as a
  ruling recorded wider than given. The fixes proposed: a source's text outranks the owner's answer
  on what it says; the reviewer reports such a point as a reading against the specification.
- **b. Whether it still applies.** No: the owner-authority milestone carried both out on main.
  `path@agent-skills@content/PRIMER.md`:25-28, `primer@question-authority`: "What an external
  specification the project implements says is answered by its text … the owner is told, not
  asked." Lines 47-51, `primer@owner-word-premise`: "**An answer of the owner on a reading or a fact
  gives way to its source.**" `path@agent-skills@content/agents/transcript-reviewer.md`:102-107: "An
  answer of the owner on a reading or a fact … is no ruling: check it against its source instead",
  reported "as a reading, not as a misstated ruling". The commits are "The primer routes each
  question to the authority that answers it, and a source outranks an owner's answer on a reading"
  and "Durable records attribute a decision to the owner, the transcript reviewer checks rulings on
  intent only, and an argued head's reversal needs the owner only at a fork".
- **c. Whether it is already recorded.** The heads are
  `design@agent-skills@questions-go-to-their-authority` and
  `design@agent-skills@owner-word-holds-as-far-as-its-premise`.
- **d. Its kind.** Missing text, carried out.
- **e.–g.** None now. The project's local override of W1 rests on the installed text as it stood at
  0.6.0; the file says it can be removed once upstream ranks the specification above the owner's
  answer, which the next release that carries these commits does.
- **h. The proposal.** No change.
- **Default**: no change, carried out on main by the owner-authority milestone.
- **Outcome**: approved.

## W3. The slice audit does not say whether a milestone's premortem tripwires are read against the slice

- **a. What it says.** Step 2 of the slice procedure lists as a gap "a tripwire whose firing
  condition, or a `deferred` issue whose trigger, the planned code meets", and sends the search to
  the standing-entry searcher, which reads the registers. A premortem tripwire stays in the
  milestone document until its harvest, so the search cannot return it. In the session, a premortem
  tripwire's firing clause covered every fixture of the milestone, while its re-entry named a later
  slice's audit. A fixture of this slice met the clause. The audit, the implementation and the work
  review did not report it; the standing-state reviewer did, before the merge, and the owner ruled
  to honour its response in that slice. The fixes proposed: step 2 reads every premortem tripwire of
  its milestone whose firing clause the slice's code or fixtures can meet, whatever its re-entry
  names; or the premortem writes a re-entry that covers every work its firing clause can be met by.
- **b. Whether it still applies.** Yes, unchanged since 0.6.0.
  `path@agent-skills@content/skills/planning/SKILL.md`:378-382: "a tripwire whose firing condition,
  or a `deferred` issue whose trigger, the planned code meets … Dispatch the search as the
  description of `agent@knowledge-architect-standing-entry-searcher` says, with the slice's spec and
  the milestone document as the work". The searcher reads `issues` and `tripwires` only. A premortem
  tripwire enters a register at the harvest, per
  `design@agent-skills@premortem-tripwire-written-at-harvest`.
- **c. Whether it is already recorded.** No.
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` concerns
  re-entry points that do not hold the evidence, for this Component's own tripwires, not a plan
  document's premortem. No rejected alternative concerns it.
- **d. Its kind.** Missing text: the step names tripwires as a gap and sends their search where a
  premortem tripwire is not. One instance, whose firing was met by the slice's own code.
- **e. Its scope.** The agent-skills Component, step 2 of
  `skill@knowledge-architect-planning@working-a-slice`, which
  `design@agent-skills@standing-entries-searched-before-the-work` restates in its list of gaps. It
  serves `goal@knowledge-architect@agents-work-without-drift`. The premortem tripwires of the
  milestone are a member of what the audit reads that the head's argument covers: a tripwire whose
  firing the planned code meets, ruled before the code is written.
- **f. A better fix.** Rivals: the premortem's re-entry widened, which depends on each premortem
  being written right and leaves the audit blind to a narrow one; or the audit reading the
  milestone's premortem, which holds whatever the re-entry says. The second: the audit already
  holds the milestone document as its work. Count: 0 added, a member added to an existing list.
- **g. Its route.** A text edit of the installed source, under the agent-skills `CLAUDE.md`,
  "Editing an installed skill or agent", with the head that restates the list brought to it under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** In step 2: "a tripwire whose firing condition … the planned code meets, those
  of the milestone document's premortem included".
- **Default**: handle now.
- **Outcome**: approved.

## W4. The slice audit does not route the searchers' "Met outside the task" items

- **a. What it says.** `primer@met-outside-the-task`: "the session that dispatched it routes each
  item by this table." Step 2 names "every entry the search returns", and not the searchers' other
  items. Two items a searcher met had no outcome until the transcript reviewer found them. The fix
  proposed: one sentence in step 2.
- **b. Whether it still applies.** The texts are unchanged.
  `path@agent-skills@content/PRIMER.md`:128-129 states the routing; the searcher's return,
  `path@agent-skills@content/agents/standing-entry-searcher.md`:105, holds the heading.
- **c. Whether it is already recorded.** No. The rejected alternative "Reporting a finding met
  outside the task without recording it" lost to `design@agent-skills@met-outside-the-task-outcomes`;
  the fix proposed goes the winner's way.
- **d. Its kind.** A lapse: the primer, read by every session, gives the dispatching session the
  routing of every subagent's items, and the searcher returns them under the heading the primer
  names. One instance.
- **e. Its scope.** None beyond the session.
- **f. A better fix.** The rival is the sentence proposed, a restatement of the primer's rule in a
  second home for one moment. The lapse rule keeps a clear instruction as it is on one instance.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of a clear rule of the primer.
- **Outcome**: approved.

## W5, and `tripwire@agent-skills@a-head-verdict-is-overruled`: entry test 2 counts a one-sentence restatement beside a pointer as a site

- **a. What it says.** The project stated one instruction in its root agent instructions, and
  restated one sentence of it in a skill and in a reviewer agent, which must deliver it. The
  decision-record reviewer found that test 2 then requires a design head. The owner ruled that the
  project records no design heads about the content of its agent workflow, and ruled installed test
  2 wrong for the project on this point. The fix proposed: a project may name its root agent
  instructions, or another configuration text, as the home of such a decision, and a one-sentence
  restatement beside a pointer to that home is then delivery, not a site.
- **b. Whether it still applies.** Yes, unchanged since 0.6.0. `path@agent-skills@content/PRIMER.md`:251-254,
  test 2: "For a decision about agent-facing text … each text that states the instruction is one
  site, a design head of the project that restates it included, other than the decision's own". The
  same primer allows the restatement, lines 146-151: a directive "is restated where it has to be
  delivered only when the restatement is no longer than a pointer to it … or one sentence", with
  "its pointer beside it". Nothing exempts such a restatement from the count of sites. The head is
  `design@agent-skills@a-head-is-owed-by-an-entry-test`, whose argument for test 2 is that "a reason
  that several sites must respect needs a home each site can cite, and a comment cannot be cited".
- **c. Whether it is already recorded.** No issue records the count. It meets the firing clause of
  `tripwire@agent-skills@a-head-verdict-is-overruled`: "the owner overrules a review's verdict on
  whether a decision earns a head, the second time." The first overruling is on record in the
  message of the commit "[review] Five reviewers read the cleanup: the owner's own intent becomes a
  fourth entry test, three heads that carry it come back, and the published workflow's heads all
  live in agent-skills": "this is the first overruling of a review's verdict on record, and the
  tripwire fires at the second". This is the second, in an adopting project, on the installed tests.
  Its response: "reopen `design@agent-skills@a-head-is-owed-by-an-entry-test` on the wording of the
  tests." No rejected alternative is the fix.
- **d. Its kind.** Wrong text, for a reason the head's own argument gives: the reason needs "a home
  each site can cite". A sentence restated beside a pointer cites its home already, and a section of
  a skill or of a root `CLAUDE.md` is citable, unlike a comment. Counting the restatement as a site
  then asks for a second home, the head, for a reason that already has one.
- **e. Its scope.** The agent-skills Component, test 2 in `primer@design-heads` and its head. It
  serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`, against a head written to
  hold a reason its instruction's home already holds. The fix changes a statement of the head.
- **f. A better fix.** Rivals:
  - the project's own ruling, no head ever for the content of its agent workflow, which is wider than
    the defect and a policy for one project;
  - a declared home, as proposed, which adds a declaration to the manifest or the root;
  - test 2 counting only the texts that state an instruction with no pointer to its home: a
    one-sentence restatement beside its pointer is delivery, not a site. It needs no declaration,
    and it follows from the head's own argument.

  The third. It removes a count rather than adding a mechanism, and the tripwire's response asks for
  exactly this rewording. Count: 0 added (a narrower count in an existing test).
- **g. Its route.** `skill@knowledge-architect-design`, on its in-change path, reopening the head as
  the tripwire's response says; then the primer's test 2 edited at its source.
- **h. The proposal.** Rule the tripwire fired. Reopen the head on the counting of sites: a
  restatement of one sentence beside a pointer to the instruction's home is no site.
- **Default**: the tripwire fired; handle now, under the design skill's in-change path.
- **Outcome**: no change, and the tripwire not fired, replacing the default. The owner, in the file: "I think there is nothing to do here ? Isn't this just the project applying different rules than ours ?." And, in the conversation, on reading the project's overruling as its own scope choice, with no instance in this repository: "Agreed, do nothing."

## `issue@agent-skills@patching-an-installed-skill`: two overrides in an adopting project

- **What the file shows.** The project overrode installed text twice, each on its owner's ruling,
  stated in its root agent instructions: the premise procedure of `primer@owner-word-premise` for a
  reading its specification answers (W1), and installed entry test 2 for decisions about its agent
  workflow (W5). It first wrote the first as an addition, and two reviewers showed it still
  contradicted the primer, before the owner ruled the installed text wrong for the project. The
  issue records the need from this repository's own three supersessions; these are the first in an
  adopting project.
- **The proposal.** The issue gains the two instances, and the sequence in which an addition that
  contradicts became an override.
- **Default**: the issue gains the instances.
- **Outcome**: approved.

## `issue@knowledge-architect@three-findings-of-a-received-retrospective-await-an-intake` closes

- **What it asks.** "An intake of W3, W4 and W5 of that file under
  `skill@klarch-retrospective-intake`, with its analysis file in
  `path@knowledge-architect@docs/retrospective-reports/` and the owner's ruling on each finding." Its
  "What bears on it" asks that W5's proposal be judged against `design@agent-skills@need-test` as
  well as the entry tests: the proposal above adds no instruction and removes a count, and the
  restatement it concerns passes the need test, since it is a pointer's sentence an agent acts on.
- **The proposal.** This analysis, once ruled and committed, meets the closing condition. The entry
  is deleted in the commit that opens the issues the analyses rule.
- **Default**: close it in that commit.
- **Outcome**: approved.

## The standing answers, and the tripwires the file does not meet

- The second standing answer's first bullet is W4. Its second, a harvest that added three
  decisions inside existing heads, is analysed with W2 of
  2026-10-13-knowledge-architect-owner-authority-milestone-klarch-workflow.
- The third, the project's own planning skill adding a search for a missing reading, is the
  project's text. Upstream, step 2 already lists "a passage of a specification the entry did not
  read" as a gap, and W1's repair sends a reading to the text.
- `tripwire@agent-skills@departure-overridden-as-a-reading`: the owner's answer in W1 was on a
  reading, and the owner's stated intent was to follow the specification; no deliberate departure
  was overridden.
- `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`: the work was a slice, whose
  audit ran.
- `tripwire@agent-skills@search-missed-before-the-work`: W3's tripwire was in a plan document, not
  a register the search reads, so it is no standing entry the search missed; the clause also asks for
  two standing-state reviews.

- **Default**: no change for the standing answers; none of the three tripwires fired.
- **Outcome**: approved.
