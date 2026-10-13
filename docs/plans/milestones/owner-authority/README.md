# Owner authority: a question goes to the authority that answers it, and the owner rules at a fork

## Status and audience

This is the milestone document of the work that changes how much weight the installed workflow
gives to the owner's word. After it lands, a session answers a question from the authority that
holds the answer: a specification's text, a measurement, the argument from the goals and the
record. Only what none of them settles goes to the owner. An answer keeps its source in the record,
and a durable record attributes a decision to the owner instead of quoting the owner. In a design
discussion, a fork closes on the owner's word, and a thread with one surviving shape closes on its
argument. It is written for a session that did not witness the discussion.

- It leaves the repository in the commit that completes the harvest of its last slice.
- Where it and a design home disagree on a point that "What is already decided" does not list as
  rewritten, the design home wins.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. The defaults are listed under "Defaults awaiting the owner".

The discussion ran in one session of Claude Code. Its transcript is the file of that session in the
harness's project directory,
`path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/`, found
by its opening message, "There is a retrospective at
/home/catA/tb266682/Documents/retrospectives/2026-10-13-thaum-slice-numbers-merge-klarch-workflow.md,
that I want to handle". The discussion begins at that message and ends at the owner's message that
opens "Drop T5, keep the other tripwires." This document was assembled from that transcript.
Rounds are numbered by the owner's messages, R0 the opening one to R5 the last of the discussion;
R6 is the owner's message after the reviews of this document, "D12, D17, D13, D15 approved.", and
R7 the owner's ruling on D18, during slice 1, and R8 the owner's ruling on D1 to D4, D14, D16 and
D19, at slice 1's review before its merge: "All defaults shown here are approved, proceed." The log holds R5 twice: the first copy lacks its last
paragraph, and no reply lies between the two. The complete copy, the second, is R5.

## How the work is done

Each slice follows `skill@knowledge-architect-planning@working-a-slice`: read it whole before each
slice. This milestone adds:

- **All four slices are implemented, reviewed and merged in the session that converged**, on the
  owner's word in R5: "We'll be implemeting, reviewing and merging all of it together in this
  session (over multiple PRs), though."
- **Each slice is one branch and one pull request, merged before the next begins**, as
  `skill@knowledge-architect-planning@cutting-steps-and-slices` requires of every milestone.
- **Each slice grounds two Components**: the root, `knowledge-architect`, which holds the goals,
  and `agent-skills`, which holds the installed text and its heads.
- **Every edit under `path@agent-skills@content/`** passes the tests of "Editing an installed skill
  or agent" in `path@agent-skills@CLAUDE.md`, read whole, and is followed by
  `cargo klarch install-agent-skills`, which rewrites the installed copies under .claude.
- **Every slice writes its CHANGELOG.md entries** in the `Next release` section, then runs
  `cargo x changelog`, per `design@knowledge-architect@the-branch-writes-its-changelog-entries`.
  A Workflow entry is owed for each question put to the owner that a slice adds or removes, per
  `design@knowledge-architect@changelog-entries`.
- **Between slice 1 and slice 4**, installed texts that slice 1 has made disagree stay on main
  until slices 3 and 4 align them. D13 says how a session meanwhile tells which text applies, and
  whether a version may be published in that interval.
- **A new head is written attributed from slice 2 on.** Slice 1's harvest runs under the
  quotation rule in force; slice 2 converts the heads and the tripwires slice 1 wrote, since it
  touches them.
- **The spec-conformity axis of each slice's review is briefed with criterion #net-growth**, on
  the owner's word in R5 that judging it is "basically the task of the spec conformity reviewer".

## Names

| name | what it names |
| --- | --- |
| G1 | the rewording of `goal@knowledge-architect@the-owner-decides`, text in #owner-decides-goal-wording |
| G2 | the new root goal on verifying the owner's word, text in #owner-word-is-verified |
| G3 | the rewording of the met-sentence of `goal@knowledge-architect@agents-work-without-drift`, text in #owner-decides-goal-wording |
| the authority section | the new primer section, slug question-authority, text under "Decided design" |
| a fork | as the authority section defines it |
| a ruling on intent | a word of the owner on a question the authority section reserves to the owner: a goal, a contradiction no argument resolves, an action that cannot be undone, or a fork |
| an answer on a reading or a fact | a word of the owner on a question that a specification's text, a measurement or the argument from the record answers |
| `argued` | the new closed state of a design thread: closed on its argument, one shape standing |
| the "Decided" section | the section that ends a turn that decided something, per #decisions-reported-to-the-owner |
| T1 to T4 | the premortem's tripwires the owner ruled to record, R5 |
| thaum | the project whose retrospective raised the question; it pins the checker and implements the Comprehensive Rules of a card game, its specification |
| the retrospective file | `path@elsewhere@~/Documents/retrospectives/2026-10-13-thaum-slice-numbers-merge-klarch-workflow.md`, with its project-side counterpart `path@elsewhere@~/Documents/retrospectives/2026-10-13-thaum-slice-numbers-merge.md` |
| W1 | its finding that the primer gives an external specification no rank against the owner's word: a session asked the owner how to read a clause that a definition elsewhere answered; when a reviewer found the definition, the premise procedure put the owner's answer back to the owner, and offered as an option a behaviour the specification forbids |
| W2 | its finding that the transcript reviewer reported an answer of the owner, on a choice the specification governs, as a ruling recorded wider than given; the owner answered "This is not about my ruling, but about what the CR says." |
| W3, W4, W5 | its findings on premortem tripwires read at a slice audit, on routing searchers' items met outside the task, and on entry test 2 for agent-facing text; outside this work |
| thaum's Q4, Q5, Q7, Q11 | in the project-side file: Q4 asked the owner what a rule's "any number" allowed, which a definition answers; Q5 was an answer about what the rules require, which the transcript reviewer reported as recorded wider than given; Q7 put Q4's answer back to the owner under the premise procedure; Q11 relayed the Q5 report to the owner as a question of a ruling's scope |

## What the work is

**Today**, the installed text names the owner as the only authority a session can defer to:

| site | text today |
| --- | --- |
| `skill@knowledge-architect-design@decision-authority` | "The owner owns every decision." |
| `skill@knowledge-architect-design@discussion-loop`, step 3 | "Factual questions (which database, which browsers, what scale) are grounding — ask them freely and early." |
| `skill@knowledge-architect-design@bounded-work` | "Wait for the owner's word: nothing is built before it." |
| `skill@knowledge-architect-design@threads-and-states` | closure by the owner's word, with two declared exceptions, `withdrawn` and `parked` |
| `skill@knowledge-architect-retrospective@expectation-sets`, the design skill's set | "The owner is the person who can decide. Every decision routes to the owner." |
| `agent@knowledge-architect-transcript-reviewer` | "Only the owner's word decides." |
| `goal@knowledge-architect@the-owner-decides` | "Goals and decisions belong to the project's owner. […] An agent never substitutes its own judgement for a ruling." |
| `primer@owner-word-premise` | an answer of the owner whose premise a source proves false is put back to the owner, with a default the session proceeds on |
| `primer@labelled-items` and the transcript reviewer | every item put to the owner is numbered, and its answer is checked as a ruling |
| `skill@knowledge-architect-planning@plan-reviews` | "A material finding is answered with a default […] listed under the defaults awaiting the owner" |
| `skill@knowledge-architect-review@what-review-leaves` | "An item whose outcome turns on a ruling the owner has not given is put to the owner" |
| `primer@design-heads`, test 4 and the ground | "The head quotes the owner's words and the owner's answer"; "The owner's words are a ground, quoted" |

Re-take the phrase counts over the installed content with
`grep -rhoiE "owner's word|put (it )?to the owner|ask the owner|owner rules|owner's ruling" crates/agent-skills/content | sort | uniq -c`.
Each slice's audit reads its own files whole; the sites are not listed here one by one.

**The quotes issue, closed by the commit that adds this document.** The issue
the-place-of-the-owner-s-quotes-in-the-workflow, of the agent-skills register, asked for a design
discussion that "sets where the workflow quotes the owner verbatim, where it attributes, and where
neither is owed". This discussion is that one, and #attribution-replaces-quotation its outcome, so
the work is listed here alone, as `skill@knowledge-architect-planning@spec-or-milestone` asks. What
it held that still bears on the work:

- The owner's words on pass 1 of this repository's design-record audit, F2 and F4: "Agreed on all
  defaults except F2 and F4: no need to quote my words, as long as attribution is correct." F2 asked
  to quote the owner's scope ruling in `design@knowledge-architect@versioning-policy`, and F4 the
  owner's request in `design@core@milestone-entries-are-directories`. The pass's report file held
  the item, and `git log --diff-filter=D --name-only -- docs/audit-reports/` finds it.
- On the rule itself, at the same ruling: "I'm tempted to remove this rule too. It's not
  particularly relevant IMO. It is weird to keep those verbatim quotes in design heads anyway."
- At the intake of the retrospectives of 2026-10-08 to 2026-10-10: "I don't consider these owner
  quotes very valuable most of the time, as long as attribution is correct. I do not want to
  provide this feature from the checker." The checker gives no form for quoting text that holds its
  syntax; that part was ruled, and stays so.
- Two sessions bent a verbatim quotation to get it past the checker: in the
  agent-configuration-entities milestone, spans of new reference kinds were written as
  angle-bracket placeholders; in the session that wrote `spec@plans@path-quickfixes`, the owner's
  words naming a skill's reference form were written bare, beside no issue entry, which
  `design@agent-skills@plain-text-is-no-repair` does not admit.

**After**, the work as four slices, in order: the goals and the primer; the recording side and
attribution; the design skill; the defaults and findings of planning, review, audit and setup. The
slices are listed under "Implementation sequence".

**Outside the work:**

- The intake of the retrospective file, on the owner's word in R0: "We won't be using the intake
  skill for this one, but instead the design skill, focusing on a single issue of the
  retrospective". So no analysis file under `skill@klarch-retrospective-intake` is written for it,
  though `design@knowledge-architect@committed-findings-analysis` asks one for every received file:
  W1 and W2 are carried by this milestone, and W3, W4 and W5 go where D15 says.
- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`: who rules is a separate question
  from what is the owner's to rule. The work names "the owner" as today, and blocks nothing that
  issue would change.
- thaum's local override of `primer@owner-word-premise`. It is thaum's to remove, once a pin move
  reaches the release that carries this work.
- Existing verbatim quotations of the owner in design heads that no slice touches, per D9.
- This repository's retrospective intake, `skill@klarch-retrospective-intake`, per D14.

## What is already decided

**Rested on and not argued again:** `primer@goals-bind`; `primer@intent-and-claims`, except its
hold on a divergence, which D4 states; `design@agent-skills@capabilities-not-structure`;
`design@agent-skills@head-ground-is-the-argument` for the split between an argued decision and one
that came from the owner, which slice 2 rewrites for quotation and for D19;
`design@agent-skills@unargued-approval-is-argued`, whose tie already goes to the owner as a fork;
`design@agent-skills@losing-shape-test`; `design@agent-skills@design-record-axis`, whose owner list
keeps a head in the wrong Component, since its argument is that no other step reads it;
`design@agent-skills@committed-audit-report`, which keeps the owner's words verbatim in a file that
leaves; `design@knowledge-architect@publish-after-merge` and
`design@knowledge-architect@stays-at-zero-x`, the actions this repository reserves;
`design@knowledge-architect@committed-findings-analysis` and
`design@knowledge-architect@intake-shares-the-repair-standard`, per D14.

**Rewritten**, each with the entries and files `cargo klarch show` lists as citing it, and the
slice that judges or updates each citing text:

| entry | change | slice | citing texts |
| --- | --- | --- | --- |
| `goal@knowledge-architect@the-owner-decides` | G1 | 1 | `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`, `path@agent-skills@content/skills/design/SKILL.md`, `path@agent-skills@content/skills/goal-setting/SKILL.md`, `design@agent-skills@bounded-path-in-design`, `design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@extension-follows-the-ground`, `design@agent-skills@design-home-write-loads-recording`, `design@agent-skills@head-ground-is-the-argument`, `design@agent-skills@premortem-tripwires-on-the-owners-word`, `design@agent-skills@standing-entry-search-agent`, `design@agent-skills@a-past-sentence-is-rewritten`, `design@agent-skills@acceptance-criteria-on-the-owners-word`, `design@agent-skills@roadmap-order-is-the-owners`, `design@agent-skills@transcript-reviewer-agent`, `design@agent-skills@plan-records-the-exchange`, `design@agent-skills@retiring-plan-opens-issue`, `design@agent-skills@audit-outcomes`, `design@agent-skills@repairs-read-against-the-record`, `design@agent-skills@primer-content`, `design@agent-skills@goals-required`, `design@agent-skills@goal-lifecycle`, `design@agent-skills@goals-change-through-goal-setting`, `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, `issue@agent-skills@a-proposal-is-read-against-the-heads-only-after-the-plan-is-written`, `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`, `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`, `path@agent-skills@docs/rejected-alternatives.md` (among them the delegated state, below), `design@knowledge-architect@stays-at-zero-x`, `design@knowledge-architect@publish-after-merge`, `design@knowledge-architect@committed-findings-analysis`. Each is reread at slice 1; one that argues "every decision is the owner's" is updated in the slice whose subject it is |
| `goal@knowledge-architect@agents-work-without-drift` | G3 | 1 | `design@agent-skills@head-prevails-over-diverging-code`, `design@agent-skills@local-intent-binds`, `design@agent-skills@tripwire-wording-is-the-agents` (which paraphrases the old met-sentence), `design@agent-skills@standing-entries-searched-before-the-work`, `design@agent-skills@plan-landing-is-not-tied-to-its-work`, `design@agent-skills@plan-read-against-the-record`, `goal@agent-skills@installed-text-leaves-room-to-judge`, `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review`, `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`, `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, `issue@agent-skills@audit-axes-beyond-the-design-record`, `issue@agent-skills@the-material-finding-duty-has-no-head`, `design@core@safe-fix-definition`, `goal@core@projects-add-their-own-checks`, `issue@core@a-home-for-developer-contracts-outside-agent-configuration`, the introduction of `register@knowledge-architect@goal`, `spec@plans@path-quickfixes`. Reread at slice 1 |
| `design@agent-skills@primer-content` | gains pointers to the authority section and to the "Decided" section; the premise directive scoped; case 1 of the item met outside the task follows G2, which reverses its argument "a stop beside them would give a session two acts for one item" for a contradiction; each directive slice 1 changes leaves for a head of its own, per `primer@design-heads` on a head a change touches | 1 | `issue@agent-skills@primer-content-bundles-each-directive-s-decision`, which stays open for the directives slice 1 does not touch; `issue@agent-skills@the-material-finding-duty-has-no-head`; `path@agent-skills@docs/rejected-alternatives.md` |
| `design@agent-skills@ruled-items-labelled` | the home of the label rule: a label numbers what was put to the owner, never what decided; its `D` row, at slice 4: a default the record or a source answers takes no label | 1, and 4 | the primer and 7 skills under `path@agent-skills@content/` (decision-recording, design, planning, project-audit, retrospective, review, setup); `design@agent-skills@primer-content`; `design@agent-skills@acceptance-criteria-on-the-owners-word` |
| `design@agent-skills@tripwire-wording-is-the-agents` | its end-of-turn listing joins the "Decided" section; its paraphrase of the old G3 follows G3 | 1 | `design@agent-skills@acceptance-criteria-on-the-owners-word` |
| `design@agent-skills@head-prevails-over-diverging-code`, and the hold of `primer@intent-and-claims` | per D4, with its statement in `design@agent-skills@divergence-routed-by-its-authority` | 1 | `design@agent-skills@local-intent-binds` |
| `design@agent-skills@head-ground-is-the-argument` | the ground is attributed, not quoted | 2 | `design@agent-skills@standing-argument-in-head`, `design@agent-skills@unargued-approval-is-argued`, `tripwire@agent-skills@owner-intent-stripped`, `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason`, `path@agent-skills@docs/rejected-alternatives.md` |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | test 4 attributes; as landed, no change: the head states test 4 as "which the owner confirms when asked", with no rule on quoting, and its own quotations convert when a later change touches it | 2 | the 12 heads, issues, tripwires and files `cargo klarch show` lists, read at slice 2's audit |
| `design@agent-skills@standing-argument-in-head`, `design@agent-skills@extension-follows-the-ground`, `design@agent-skills@existing-heads-on-touch`, `design@agent-skills@instruction-record-is-minimal`, `design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@a-past-sentence-is-rewritten`, `design@agent-skills@verifier-in-the-run` | quotation becomes attribution; as landed, the first four need no change, since none states a rule on quoting, and `design@agent-skills@a-past-sentence-is-rewritten` keeps its rule per D11; `design@agent-skills@new-or-reshaped-head-needs-design` and `design@agent-skills@verifier-in-the-run` are rewritten | 2 | each one's `show` list, read at slice 2's audit |
| `design@agent-skills@transcript-reviewer-agent`, `design@agent-skills@misstated-ruling-rated-by-what-it-changes` | the ruling check covers rulings on intent; an answer on a reading or a fact is checked against its source; as landed, the first is rewritten, and the second needs no change: its rating stands once `design@agent-skills@transcript-reviewer-agent` defines a ruling | 2 | `path@agent-skills@docs/rejected-alternatives.md`, `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, `tripwire@agent-skills@ruling-lost-in-assembly` |
| `design@agent-skills@plan-records-the-exchange`, `design@agent-skills@in-change-path`, `design@agent-skills@audit-method` (its quotation check) | verbatim stays in plan items and in files that leave; an answer is recorded with its source; as landed, the second and third are rewritten, and the first needs no change, since the answer rule lands in the planning skill beside it | 2 | `tripwire@agent-skills@ruling-lost-in-change`, `tripwire@agent-skills@ruling-lost-in-assembly`, `design@agent-skills@design-hands-off-to-planning`, `tripwire@agent-skills@audit-defaults-overruled` and the rest of each `show` list |
| `tripwire@agent-skills@owner-intent-stripped` | fires on a lost attribution | 2 | `path@agent-skills@docs/rejected-alternatives.md` |
| `design@agent-skills@structure-the-flow`, `design@agent-skills@bounded-path-in-design`, `design@agent-skills@thread-slug-is-entry-id`, `design@agent-skills@frontier-tier-only` (closed set of states) | a fork closes on the owner's word; `argued`; bounded work proceeds on its default | 3 | `design@agent-skills@in-change-path`, `design@agent-skills@standing-entries-searched-before-the-work`, `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review`, `issue@agent-skills@the-open-space-test-may-admit-every-problem`, `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`, `tripwire@agent-skills@a-shortcut-decision-earns-a-head`, `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`, `path@agent-skills@README.md`, `design@agent-skills@primer-limit`, `path@agent-skills@docs/rejected-alternatives.md` |
| `design@agent-skills@expectation-set-bounds-scope` | the design skill's set changes; the head stands | 3 | `path@agent-skills@CLAUDE.md`, `path@agent-skills@README.md`, `design@agent-skills@outcome-over-display`, `issue@agent-skills@expectation-sets-for-the-installed-skills`, `tripwire@agent-skills@expectation-set-closes-a-contradiction`, `path@agent-skills@docs/rejected-alternatives.md` |
| `design@agent-skills@audit-outcomes`, `design@agent-skills@adopting-existing-docs` | a finding or a choice the record or a source answers takes its outcome from the session | 4 | each one's `show` list, read at slice 4's audit; `design@knowledge-architect@workflow-findings-stay-here` |

**Kept with the owner** by the defaults below, and so not rewritten unless the owner rules
otherwise: `design@agent-skills@premortem-tripwires-on-the-owners-word`,
`design@agent-skills@premortem-tripwire-written-at-harvest`,
`design@agent-skills@acceptance-criteria-on-the-owners-word`,
`design@agent-skills@acceptance-criteria-in-the-document`,
`design@agent-skills@roadmap-order-is-the-owners`,
`design@agent-skills@audit-passes-repeat-while-the-count-falls`,
`design@agent-skills@audit-net-growth`, `design@core@plan-document-sections-by-kind`.

**A rejected alternative the work meets.** In `path@agent-skills@docs/rejected-alternatives.md`,
"A delegated state, under which the owner hands a decision to the agent within a stated boundary"
lost to `goal@knowledge-architect@the-owner-decides`: "inside a grant, a correction whose own
action could not be undone ran with no word from the owner". The state `argued` is no grant: it
closes a thread by a rule every session applies, and G1 keeps an action that cannot be undone with
the owner, so that fact still defeats a grant. The entry stays `live`; slice 1 rereads it against
G1 and rewords the sentence that argued from the goal's old wording.

**Standing entries the discussion's search returned, and those the reviews of this document
added**, with their outcome:

| entry | outcome |
| --- | --- |
| the quotes issue | closed by the commit that adds this document, as above |
| `tripwire@agent-skills@a-comment-escalation-is-overruled` | evidence for #questions-go-to-their-authority; stands |
| `tripwire@agent-skills@owner-intent-stripped` | reworded at slice 2 |
| `tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change` | reread at slice 2: "rulings" in their clauses means rulings on intent |
| `tripwire@agent-skills@member-beyond-the-argument` | reread at slice 2, with `design@agent-skills@extension-follows-the-ground` |
| `tripwire@agent-skills@head-created-without-deliberation` | reread at slice 2: "the owner's words" in its clause becomes an attribution |
| `tripwire@agent-skills@structure-rejection-overruled`, `tripwire@agent-skills@a-head-verdict-is-overruled`, `tripwire@agent-skills@audit-defaults-overruled` | reread at slices 3 and 4: the owner list changes membership |
| `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` | outside the work, as stated above |
| `issue@agent-skills@the-material-finding-duty-has-no-head` | reread at slice 3: the duty is reshaped by #owner-rules-at-a-fork; the issue stays open unless that slice's harvest writes the head it asks for |
| `issue@agent-skills@a-proposal-is-read-against-the-heads-only-after-the-plan-is-written` | reread at slice 3: the loop it asks to change is the one slice 3 rewrites, and the owner's evidence a31 is its mechanism |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision` | reread at slice 1, which splits the directives it touches |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | bears on nothing built; its "drift corrections" count is the measure of G3, left to that issue |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | reread at slice 3, where the design skill's set changes |
| `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` | reread at slice 2: a head that quotes the owner beside its own argument |
| `tripwire@agent-skills@rule-title-wider-than-its-argument`, `issue@core@citation-regime-over-any-pinned-text` | neither holds the word "owner"; the first guards a title rule slice 2 does not change, and the second's trigger, verifying quotations of a pinned text, the work does not meet |
| `tripwire@agent-skills@design-conformance-findings-needless`, `tripwire@agent-skills@conforming-draft-wrong`, `tripwire@core@fix-refusal-routed-around`, `tripwire@core@staged-pass-fails-the-checkout`, `tripwire@agent-skills@a-shortcut-decision-earns-a-head`, `tripwire@agent-skills@audit-passes-do-not-converge`, `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, `issue@agent-skills@the-open-space-test-may-admit-every-problem`, `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument`, `issue@gates@the-gate-list-head-bundles-the-library-s-independence`, `issue@gates@the-one-spawn-helper-has-no-recorded-argument`, `issue@knowledge-architect@command-output-is-not-declared-a-contract`, `issue@core@no-checked-form-cites-a-point-inside-a-section` | each bears through the wording "the owner ruled" or "the owner's word" on a premortem answer, a goal or a missing argument, which stay the owner's; each is reread at the audit of the slice whose text it names |

## Criteria

### The owner rules on what the goals and the record leave open, rather than correcting drift `##agents-work-without-drift`

Binding, from `goal@knowledge-architect@agents-work-without-drift`, worded by G3. Met by
#questions-go-to-their-authority, #derived-decision-proceeds-on-default and #owner-rules-at-a-fork.

### The record shows the owner's decisions as the owner's, and every other decision on its own ground `##the-owner-decides`

Binding, from `goal@knowledge-architect@the-owner-decides`, worded by G1. Met by
#answer-keeps-its-authority, #attribution-replaces-quotation, #decisions-reported-to-the-owner and
#owner-rules-at-a-fork.

### No work is built on a word of the owner whose premise a check would show false `##owner-word-verified`

Binding, from G2. Met by #specification-outranks-an-owner-answer and #owner-word-is-verified, with
D12 for how the two read together.

### The owner decides what the workflow changes from real use `##real-use-is-the-owners`

Binding, from `goal@knowledge-architect@the-workflow-improves-through-real-use`: "the owner decides
what changes". Met by D14, which leaves the intake of received workflow files, and an
agentic-workflow audit's findings on the installed text, with the owner.

### No project's specification, or absence of one, is assumed `##works-anywhere`

Binding, from `goal@agent-skills@installed-text-works-anywhere`, and from the owner's answer 2 in R1.
Met: the authority section names a specification as one authority among several, and leaves to the
project whether it follows one and which actions it reserves.

### No two installed instructions contradict `##no-contradiction`

Binding, from `goal@knowledge-architect@agents-get-a-complete-workflow` and
`goal@agent-skills@one-skill-per-activity`. Met at the milestone's end by design: the premise
procedure is scoped to rulings on intent, and every site the mapping table marks "session" is
rewritten. Between slices 1 and 4, met by D13. Judged on the written text by each slice's review.

### An instruction adds a capability, not structure the work does not need `##capability-not-structure`

Binding as a presumption, from `design@agent-skills@capabilities-not-structure`. Met: the authority
section adds one capability, routing a question by its authority, and the work removes waits.

### thaum's local override can be removed once this lands `##override-removable`

Weighed, from W1. Met by #specification-outranks-an-owner-answer, with D12.

### The work removes more instructions than it adds `##net-growth`

Weighed, from the discussion. Judged by the spec-conformity axis of each slice's review, on the
owner's word in R5 that AC1 restated "basically the task of the spec conformity reviewer" as "a
mechanical check that can misfire". Each slice states, in its review's record, the instructions it
added and removed.

## Threads

### A question goes to the authority that answers it, and the owner answers goals, contradictions, irreversible actions and forks `##questions-go-to-their-authority`

- **Proposed:** the agent, reply to R0; fused with the owner's direction of R1 in the reply to R3;
  the merge clause from the owner, R4.
- **States:** new; approved R1; reopened by the agent's material finding in the reply to R1 (a33);
  approved again on its round-1 wording R2; reopened by the owner R3; approved in its fused form R4.
- **Arguments:** a1, a3, a4, a5, a6, a7, a9, a10, a11, a25, a33, a34, a45, a54, a55, a57, a58, a62,
  a67, a70, a74.
- **Closed by:** R1: "questions-go-to-their-authority is a good proposal. I agree with the wording,
  I do think it covers most (if not all) cases." R3: "I said the first round draft stand, but what I
  want is a fusion of both mine and yours from the first round, if that makes sense. It should be
  inspired by all I said in the previous rounds." R4: "questions-go-to-their-authority,
  owner-decides-goal-wording, decisions-reported-to-the-owner approved". R4, the merge: "a merge to
  main is not an irreversible action (it is a normal part of working in a project), but it should
  be left to each project to decide whether thay let agents merge, or do it themselves."
- **Shape:** "Decided design", the authority section. **Harvest:** a head of its own, slug its
  thread's, slice 1; its deliveries in planning and review added to that head, slice 4.
- **Relations:** absorbs the routing part of the withdrawn #agent-decides-owner-overrules. Guarded
  by T2.

### An answer on a reading or a fact is recorded with its source, never as the owner's ruling `##answer-keeps-its-authority`

- **Proposed:** the agent, reply to R0. **States:** new; approved R1.
- **Arguments:** a2, a8, a10, a12, a26.
- **Closed by:** R1: "answer-keeps-its-authority is important too, it is one of the main source of
  the problem. When all answers (such as design heads) are recorded attributed to my word,
  questioning them is either discouraged, or gets put to me even when the answer is obvious (and I
  might not have it)."
- **Shape:** "Decided design". **Harvest:** a head of its own, slug its thread's, slice 1; the label
  rule in `design@agent-skills@ruled-items-labelled`, slice 1; the transcript reviewer's heads,
  slice 2.
- **Relations:** carries W2.

### A source that contradicts an owner's answer on a reading applies at once, and the owner is told `##specification-outranks-an-owner-answer`

- **Proposed:** the agent, reply to R0; generalised by the owner's answer 2, R1.
- **States:** new; approved R1.
- **Arguments:** a2, a13, a27, a32, a42, a75.
- **Closed by:** R1: "specification-outranks-an-owner-answer: yes, this is important. This is
  similar I think to what is required of the design skill, question the word of the owner and verify
  its premises". Answer 2, R1: "Whether a specifiation is followed, and how much closely, should be
  left to consumer projects. What we have to account for is the fact that a project can be following
  a specification, and we must avoid conflicting with that."
- **Shape:** "Decided design". **Harvest:** one head, slice 1, for the whole of
  `primer@owner-word-premise`: the premise rule, the source rule and the hold on a ruling on intent
  stay together, since the source rule is the premise rule's exception for an answer on a reading,
  per `tripwire@agent-skills@split-rule-from-exception`. The thread's name narrows the decision to a
  specification, so the head takes a slug that names it, the pair
  `#specification-outranks-an-owner-answer → owner-word-holds-as-far-as-its-premise`.
- **Relations:** carries W1; its goal is #owner-word-is-verified, read with it per D12. Guarded by
  T3.

### Bounded work and a derived thread proceed on their default `##derived-decision-proceeds-on-default`

- **Proposed:** the agent, reply to R0. **States:** new; approved R1.
- **Arguments:** a14, a28, a31, a39, a50, a51.
- **Closed by:** R1: "derived-decision-proceeds-on-default: I agree on this, as you might have seen
  from what I said above, this is in the direction I want to take."
- **Shape:** "Decided design". **Harvest:** `design@agent-skills@bounded-path-in-design` rewritten,
  slice 3. For defaults, it is a delivery of #questions-go-to-their-authority, added to that head at
  slice 4.
- **Relations:** holds the owner's evidence that defaults the heads answer are put to the owner
  (a31, a39). Its closure inside a discussion is given by #owner-rules-at-a-fork.

### The owner decides what the goals and the record leave open, is informed of every other decision, and can overrule any of them `##owner-decides-goal-wording`

- **Proposed:** the agent, reply to R0; the owner proposed a narrower goal in R1; fused in the reply
  to R3.
- **States:** new; approved R1; reopened by the material finding in the reply to R1; approved on its
  round-1 wording R2; reopened R3; approved as G1 and G3, R4.
- **Arguments:** a15, a19, a20, a21, a22, a23, a24, a29, a33, a35, a37, a45, a48, a53, a54, a63, a66.
- **Closed by:** R1: "right, I think this is a better proposal than mine above, for the goal
  wording." R2: "What I said above should still impact the wordings of G1 and G3, I think." R4:
  "goal wordings for G1 and G3 are approved."
- **Shape:** G1 and G3 verbatim, below; D17 on G1's length. **Harvest:**
  `register@knowledge-architect@goal`, slice 1.

G1, the goal entry as approved, its slug `the-owner-decides` kept:

> **The owner decides what the goals and the record leave open, is informed of every other
> decision, and can overrule any of them**
>
> The owner states the project's intent in its goals. Agents decide what the argument from the goals, the record and the sources the project follows settles, and record each such decision on that ground. What that leaves open is the owner's: a choice between shapes that all survive the argument, a contradiction no argument resolves, a goal to set or change, and an action that cannot be undone, such as a publication. A decision the owner made is recorded as the owner's. The owner is informed of every decision, and can overrule any of them, at any time and for any reason. It is met while the owner rules only on what is theirs, and learns of every other decision from the record and the reports.

G3, the met-sentence as approved:

> It is met while agents carry work from design to merge with the owner ruling only on what the goals and the record leave open, rather than correcting drift or answering what the record already answers, and a departure from the recorded design is caught by a check or a review before it merges.

### Durable records attribute a decision to the owner instead of quoting the owner `##attribution-replaces-quotation`

- **Proposed:** the agent, reply to R0. **States:** new; approved R1.
- **Arguments:** a8, a16, a30, a76.
- **Closed by:** R1: "attribution-replaces-quotation: yes, I'm for it."
- **Shape:** "Decided design". **Harvest:** a head of its own, slug its thread's, slice 2, with
  `design@agent-skills@head-ground-is-the-argument` and test 4 rewritten to point to it.
- **Relations:** closes the quotes issue. Guarded by T4.

### Agents verify the owner's word before acting on it, and the part resting on a contradiction waits `##owner-word-is-verified`

- **Proposed:** the owner, R1; the draft G2 the agent's, reply to R1.
- **States:** new; approved R2.
- **Arguments:** a27, a36, a48.
- **Closed by:** R2: "I agree with the wording for G2."
- **Shape:** G2 verbatim, below; D12 on how it reads with the source rule. **Harvest:**
  `register@knowledge-architect@goal`, slice 1, under the slug `the-owner-s-word-is-verified`, with
  the pair `#owner-word-is-verified → the-owner-s-word-is-verified`.
- **Relations:** the agent's draft narrowed the owner's "should not act while their are stil
  conflicting points" to the part resting on the conflict (a36); the approval of the wording rules
  on that narrowing.

G2, the goal entry as approved:

> **Agents verify the owner's word before acting on it, and build nothing on a contradiction**
>
> An agent following the workflow takes no word of the owner as granted. Before acting on it, the agent checks its premises against the goals, the record, the code and the sources the project follows. A contradiction it finds goes to the owner, and the part of the work that rests on it waits for the owner's answer; the rest goes on. This holds in every activity, not only in a design discussion. It is met while no work is built on a word of the owner whose premise a check would have shown false.

### The session decides everything G1 does not reserve, withdrawn `##agent-decides-owner-overrules`

- **Proposed:** the agent, reply to R1. **States:** new; withdrawn in the reply to R2.
- **Arguments:** a23, a38, a39, a45, a50.
- **Defeating reason:** its scope was "everything G1 does not reserve" against the round-2 G1, which
  the owner set aside in R2: "I think the direction I gave left too large of a scope for decisions
  that are left to agents."
- **Relations:** its remainder is #derived-decision-proceeds-on-default. **Harvest:** the
  deliberation only, as "Losing alternatives" says.

### Every thread closes on its argument, superseded by the fork rule `##thread-closes-on-the-argument`

- **Proposed:** the agent, reply to R1. **States:** new; presumed-settled as ruled out in the reply
  to R2, on the agent's reading of R2; superseded by #owner-rules-at-a-fork in R4.
- **Arguments:** a40, a46, a47, a49, a69.
- **Closed by:** R4, approving the absorbing thread by name: "owner-rules-at-a-fork: yes, this
  should be the right closure to the problem at hand."
- **Harvest:** a rejected alternative, slice 3, as "Losing alternatives" says.

### Each turn that decided something ends with a Decided section `##decisions-reported-to-the-owner`

- **Proposed:** the agent, reply to R1. **States:** new; approved R4.
- **Arguments:** a24, a41, a77.
- **Closed by:** R4: "decisions-reported-to-the-owner approved" and "I agree too with the decided
  section of decisions-reported-to-the-owner."
- **Shape:** "Decided design". **Harvest:** a head of its own, slug its thread's, slice 1, with a
  pointer from `design@agent-skills@primer-content`.
- **Relations:** serves G1's "informed of every decision". T5, which watched it, was dropped in R5.

### A fork closes on the owner's word, and a thread with one surviving shape closes argued `##owner-rules-at-a-fork`

- **Proposed:** the agent, reply to R3, answering the owner's objection of R3 (a55). The state name
  `argued` is the agent's, reply to R4, closed `argued` there.
- **States:** new; approved R4, with the condition of a new state.
- **Arguments:** a46, a47, a55, a56, a57, a58, a59, a60, a61, a64, a65, a68, a73.
- **Closed by:** R4: "owner-rules-at-a-fork: yes, this should be the right closure to the problem at
  hand. We'll go with that. A thread closed on its argument should have its own new state in the
  thread tables."
- **Shape:** "Decided design". **Harvest:** a head of its own, slug its thread's, slice 3, which
  records the owner's reason for the discussion form (a47), attributed;
  `design@agent-skills@structure-the-flow` rewritten. a52's search was of headings only:
  `design@agent-skills@structure-the-flow` already grounds the discussion's unbounded size on the
  owner's "core idea of the 'open design discussion'", so slice 3 reads that head before writing
  the reason anywhere.
- **Relations:** supersedes #thread-closes-on-the-argument. Guarded by T1.

## Arguments

Each argument's round, who gave it, the threads it bears on, and its key words verbatim. The
extraction wrote a backticked section slug as "the section <name>".

### The problem: owner words carry too much weight, reducing autonomy `##a1`

**R0, owner.** The problem: owner words carry too much weight, reducing autonomy. Threads: #questions-go-to-their-authority, #answer-keeps-its-authority. Words: "owner words are carrying too much weight in the agentic workflow, leading to much less autonomy."

### Worse with an external specification `##a2`

**R0, owner.** Worse with an external specification: heads recorded as ruled on the owner's word are argued from the spec. Threads: #answer-keeps-its-authority, #specification-outranks-an-owner-answer, #attribution-replaces-quotation. Words: "A lot of design heads and decisions are recorded as ruled on my word, when they are actually argued from the external specification."

### Questions the spec answers are still put to the owner `##a3`

**R0, owner.** Questions the spec answers are still put to the owner. Threads: #questions-go-to-their-authority. Words: "A lot of choices are also put to me, even when the external specification answers it perfectly clearly."

### The flaw works against the autonomy goal, and occurs without a specification too `##a4`

**R0, owner.** The flaw works against the autonomy goal, and occurs without a specification too. Threads: #questions-go-to-their-authority (against the spec-only rival). Words: "it directly works against the \"agent work autonomously\" goal. It has an impact even without external specification (I observed it here and an issue was opened), and it becomes even worse with one."

### Mechanism: the installed text names the owner as the only authority `##a5`

**reply to R0, agent.** Mechanism: the installed text names the owner as the only authority. Threads: #questions-go-to-their-authority. Words: "The installed text names the owner as the only authority a session can defer to. So every question the session does not settle alone goes to the owner, and every answer comes back recorded as a ruling, whatever its real source."

### Sites carrying the mechanism, verbatim `##a6`

**reply to R0, agent.** Sites carrying the mechanism, verbatim: "The owner owns every decision." (decision-authority); "Factual questions (which database, which browsers, what scale) are grounding — ask them freely and early." (loop step 3), which "invites \"what does CR:608.2d mean?\""; "Wait for the owner's word: nothing is built before it." (bounded-work); "Every decision routes to the owner." (expectation set); the-owner-decides goal "An agent never substitutes its own judgement for a ruling."; owner-word-premise makes "a correction from the spec becomes a new question (thaum's Q7)"; labelled-items turns items into "the owner's ruling Q<n>". Threads: #questions-go-to-their-authority, #specification-outranks-an-owner-answer, #answer-keeps-its-authority, #derived-decision-proceeds-on-default, #owner-decides-goal-wording.

### The workflow has no rank for authorities other than goals over decisions `##a7`

**reply to R0, agent.** The workflow has no rank for authorities other than goals over decisions; four kinds of question, only the last is the owner's. Threads: #questions-go-to-their-authority. Words: "It has no rank for the other authorities a session actually has"; "Only the last row is the owner's. Today the workflow sends all four rows to the owner."

### Measurements `##a8`

**reply to R0, agent.** Measurements. Threads: #answer-keeps-its-authority, #attribution-replaces-quotation. Words: "thaum: 15 of 63 interpretation entries mention the owner. A reading of the rules should never need to."; "of 11 questions, at least 3 were answered by the rules: Q4, Q7 and Q11."; "here: 90 of 127 agent-skills heads mention the owner." Caveat: "\"mentions the owner\" is an upper bound, not a count of heads grounded on the owner".

### No prior-art survey `##a9`

**reply to R0, agent.** No prior-art survey: no outside project ships such a mechanism. Words: "I can't name an outside project that ships a mechanism for \"which questions an agent puts to its human\"."

### Routing over recording care, from the owner's premise that instructions cannot remove the weight `##a10`

**reply to R0, agent.** Routing over recording care, from the owner's premise that instructions cannot remove the weight. Threads: #questions-go-to-their-authority over #answer-keeps-its-authority (priority). Words: "LLM agents are trained to take the input they receive from humans seriously and follow them closely", "I do not believe any amount of instruction anywhere would help"; "the fix is to send fewer questions, and to name each answer's source **in the question itself**."

### #questions-go-to-their-authority draft and deliveries `##a11`

**reply to R0, agent.** #questions-go-to-their-authority draft and deliveries; cost "one primer section of about 7 lines, and 3 rewordings". Rival thaum's W1 (spec-only sentence) loses: "It loses because the same mechanism runs without a specification", citing `tripwire@agent-skills@a-comment-escalation-is-overruled` "exists because sessions escalate what a recorded reason already settles".

### #answer-keeps-its-authority `##a12`

**reply to R0, agent.** #answer-keeps-its-authority: a label numbers a question, never names what decided. Words: "The label `Q<n>` numbers what was put to the owner. It never names what decided."; "the record says \"the owner chose X among the readings the text permits\""; transcript reviewer "reports a misread source as a reading, not as a misstated ruling". Rival (leave recording alone) loses: "Q4 still happened through a missed definition. A question that wrongly reaches the owner will recur at some rate, and its answer must not harden into a ruling."

### #specification-outranks-an-owner-answer `##a13`

**reply to R0, agent.** #specification-outranks-an-owner-answer: premise procedure only for intent; a source contradicting an answer applies at once, owner told; irreversible part waits ("Keep the wait"). Rival thaum's first draft loses: "it still asks, and two thaum reviewers found it contradicts the spec's authority." Open edge: "a departure is a scope decision, stated as such in a goal or a head, and never inferred from an answer to a reading question."

### #derived-decision-proceeds-on-default, low confidence `##a14`

**reply to R0, agent.** #derived-decision-proceeds-on-default, low confidence: proceeds on default when every premise comes from another authority; review still runs. Against: "the binding goal says \"Goals and decisions belong to the project's owner\". As worded, the goal rules this thread out." Assumed: "how often you change a bounded proposal. If you rarely do, the wait costs one round trip and buys nothing."

### #owner-decides-goal-wording `##a15`

**reply to R0, agent.** #owner-decides-goal-wording: without the goal change, "the current wording \"decisions belong to the owner\" remains a source of the weight even after the threads above land."

### #attribution-replaces-quotation `##a16`

**reply to R0, agent.** #attribution-replaces-quotation: attribute in durable records; verbatim only in plan thread items per `design@agent-skills@plan-records-the-exchange`. Consequence: owner-intent-stripped "must be reworded to fire on a lost *attribution*, or this thread trips it on the first edit." Rival (keep quotes) loses on owner's words "no need to quote my words, as long as attribution is correct", "and on the 2 cases where a verbatim quote was bent to get past the checker."

### Path: full path with a spec, this touches the primer, 3 installed skills, the transcript reviewer and a goal `##a17`

**reply to R0, agent.** Path: full path with a spec, "this touches the primer, 3 installed skills, the transcript reviewer and a goal."

### The unnamed-owner issue is outside the question `##a18`

**reply to R0, agent.** The unnamed-owner issue is outside the question: "Who rules is a separate question from what is theirs to rule. I propose to leave it out, but the shape below must not block it".

### The-owner-decides wording is part of the origin, and conflicts with autonomy `##a19`

**R1, owner.** The-owner-decides wording is part of the origin, and conflicts with autonomy. Threads: #owner-decides-goal-wording. Words: "I think the wording there is part of the origin of the problem. As stated, it even conflicts with the autonomous part of the agents-work-without-drift goal."

### History: the goal came from the design skill's early aim of informing the owner, not attributing decisions `##a20`

**R1, owner.** History: the goal came from the design skill's early aim of informing the owner, not attributing decisions. Threads: #owner-decides-goal-wording, #owner-rules-at-a-fork (later). Words: "I wanted to make sure the owner would get to see every thread clearly exposed, and requiring a ruling on each like this was a good method. It was more about information than decision attribution."

### Second original use case, now unneeded `##a21`

**R1, owner.** Second original use case, now unneeded: superseding a "better" choice as out of scope or overbuilt. Threads: #owner-decides-goal-wording, #questions-go-to-their-authority. Words: "make sure the owner would be able to supersede an arguably \"better\" design choice, if it was considered out of scope, too much work, overengineered, or other similar reasons. However, these days, I no longer find a need for this specific use case."

### Frontier models are more capable, so more design can be delegated `##a22`

**R1, owner.** Frontier models are more capable, so more design can be delegated. Words: "Agents and frontier LLM have become much more performant since then, that much more can be delegated to them in term of project design."

### Proposal to remove or narrow the goal to an overrule `##a23`

**R1, owner.** Proposal to remove or narrow the goal to an overrule; only irreversible actions need a question. Threads: #owner-decides-goal-wording, #agent-decides-owner-overrules (agent's derivation). Words: "the owner can overrule a design choice on his whim"; "Only \"irreversible\" actions truly need to be put to the owner as a question (like publising things)."

### Autonomy outranks owner making code design decisions `##a24`

**R1, owner.** Autonomy outranks owner making code design decisions; goals carry intent; informed owner can fix goals. Words: "Agents working autonomously is a more important goal than owners making code design decisions, IMO. I think the goal system is most of the time enough to carry the owner's intent. As long as owners get informed about what happens, they can see if the goals they set are not enough or not worded properly". Threads: #owner-decides-goal-wording, #decisions-reported-to-the-owner.

### #questions-go-to-their-authority approved `##a25`

**R1, owner.** #questions-go-to-their-authority approved: "questions-go-to-their-authority is a good proposal. I agree with the wording, I do think it covers most (if not all) cases."

### #answer-keeps-its-authority `##a26`

**R1, owner.** #answer-keeps-its-authority: attribution of answers to the owner discourages questioning them. Words: "When all answers (such as design heads) are recorded attributed to my word, questioning them is either discouraged, or gets put to me even when the answer is obvious (and I might not have it)."

### #specification-outranks-an-owner-answer approved, and a new goal `##a27`

**R1, owner.** #specification-outranks-an-owner-answer approved, and a new goal: agents verify the owner's word, put contradictions, do not act on conflicts; general, not design-only. Threads: #specification-outranks-an-owner-answer, #owner-word-is-verified (G2). Words: "agents following the workflow do not take the owner's word as granted, they question and verify it before acting on it, and put contradictions to the owner. They also should not act while their are stil conflicting points in what was asked of them."; "This should be a general behavior, not specific to design sessions."

### #derived-decision-proceeds-on-default approved `##a28`

**R1, owner.** #derived-decision-proceeds-on-default approved: "I agree on this, as you might have seen from what I said above, this is in the direction I want to take."

### #owner-decides-goal-wording `##a29`

**R1, owner.** #owner-decides-goal-wording: "right, I think this is a better proposal than mine above, for the goal wording."

### #attribution-replaces-quotation `##a30`

**R1, owner.** #attribution-replaces-quotation: "yes, I'm for it."

### Evidence: the quotes issue, plus defaults answerable from heads `##a31`

**R1, owner, answer 1.** Evidence: the quotes issue, plus defaults answerable from heads; the owner acts as intermediary. Threads: #derived-decision-proceeds-on-default, #questions-go-to-their-authority, #agent-decides-owner-overrules. Words: "a lot of defaults are put back to me, when I believe they should be answerable by reading the recorded design heads"; "there might be a bit too many of these defaults which appear to have been proposed without reading the related design heads"; "the workflow currently redirects the quesiton to me first, before looking at the design heads"; "I end up working as the intermediary between the agent and the design heads, having to point them to those."

### Following a spec is the consumer's choice `##a32`

**R1, owner, answer 2.** Following a spec is the consumer's choice; the workflow must not conflict with a project that follows one. Threads: #specification-outranks-an-owner-answer. Words: "Whether a specifiation is followed, and how much closely, should be left to consumer projects. What we have to account for is the fact that a project can be following a specification, and we must avoid conflicting with that."

### Material finding `##a33`

**reply to R1, agent.** Material finding: the two approved drafts reserve to the owner what the owner now wants delegated. Threads: #owner-decides-goal-wording, #questions-go-to-their-authority. Words: "Each still reserves to the owner what you now want delegated"; the clause "the weighing of costs, and every choice the argument and the sources leave open"; "By `primer@owner-word-premise`, both approvals hold only as far as their premise."

### Corrected #questions-go-to-their-authority `##a34`

**reply to R1, agent.** Corrected #questions-go-to-their-authority: owner answers only "a goal to set or change, and a contradiction between the owner's words, the goals and the record that no argument resolves. The session decides everything else ... an action that cannot be undone waits for the owner's word."

### G1 (round 2) renames the slug since the old misdescribes `##a35`

**reply to R1, agent.** G1 (round 2) renames the slug since the old misdescribes; cost "The old one is cited 35 times in 11 files". "For any reason" renders "on a whim".

### G2 narrows should not act to the part resting on the conflict, as primer@intent-and-claims does for a … `##a36`

**reply to R1, agent.** G2 narrows "should not act" to the part resting on the conflict, as `primer@intent-and-claims` does for a divergence. Threads: #owner-word-is-verified. Words: "a conflict on one point of a 5-step change would stop the other 4. That works against `goal@knowledge-architect@agents-work-without-drift`." Against: "a session may judge too narrowly what \"rests on\" the conflict." "This is your weighing. My default is the part."

### G3: the met-sentence with the owner ruling on decisions rather than correcting drift has the same flaw `##a37`

**reply to R1, agent.** G3: the met-sentence "with the owner ruling on decisions rather than correcting drift" "has the same flaw".

### #agent-decides-owner-overrules `##a38`

**reply to R1, agent.** #agent-decides-owner-overrules (absorbs #derived-decision-proceeds-on-default): a large reversal, measured: "\"owner's word\" occurs 61 times, \"owner rules\" 27, \"put to the owner\" 14, \"ask the owner\" 8"; about 12 heads reversed or narrowed (count to be re-taken). Path: milestone, "the primer, 6 skills, 3 agents and about 12 heads", slices by activity.

### Intermediary evidence located `##a39`

**reply to R1, agent.** Intermediary evidence located: planning sends every material finding of a plan review as `D<n>` and blocks harvest until "every default the document lists as awaiting the owner is ruled"; review: "An item whose outcome turns on a ruling the owner has not given is put to the owner". "Neither tells the session to answer from the heads first." Threads: #agent-decides-owner-overrules, later #derived-decision-proceeds-on-default.

### #thread-closes-on-the-argument `##a40`

**reply to R1, agent.** #thread-closes-on-the-argument: checkpoint becomes a report to contest. For: "in this round alone, you approved two drafts whose wording contradicted your intent. A confirmation step did not catch that; the agent's own check did." For the rival: "a discussion you chose to join is where you have the most context."

### #decisions-reported-to-the-owner `##a41`

**reply to R1, agent.** #decisions-reported-to-the-owner: a "Decided" section per turn. Rival (commit messages, PR) loses: "a decision made in turn 3 and reverted in turn 5 reaches no commit, and you read turns more than you read messages."

### Answer 2 folded in `##a42`

**reply to R1, agent.** Answer 2 folded in: "what the text *says* is always the text's to answer, and whether the project *follows* it is the project's own goal or decision. The workflow assumes neither." Threads: #specification-outranks-an-owner-answer.

### Merge question default no `##a43`

**reply to R1, agent.** Merge question default no: "a revert commit undoes it, and a project that consumes its main branch can reserve the merge in its own instructions. Publishing a release, and sending content off the machine, stay reserved." Threads: #questions-go-to-their-authority (G1's irreversible clause).

### Net instruction growth expected negative: most of the 61 owner's word sites shrink `##a44`

**reply to R1, agent.** Net instruction growth "expected negative: most of the 61 \"owner's word\" sites shrink".

### The owner's R1 direction left too large a scope to agents `##a45`

**R2, owner.** The owner's R1 direction left too large a scope to agents; the agent's round-1 drafts are better. Threads: #questions-go-to-their-authority, #owner-decides-goal-wording, #agent-decides-owner-overrules. Words: "I think the direction I gave left too large of a scope for decisions that are left to agents. Your's is better IMO, it keeps a bit more and gives more detailed conditions for the scopes."

### The design skill stays a discussion `##a46`

**R2, owner.** The design skill stays a discussion; owner proposes designs, agent does not make all choices. Threads: #thread-closes-on-the-argument, #owner-rules-at-a-fork (later). Words: "I want to keep the design skill shaped like a \"discussion\", where the owner proposes designs with the agent, without having the agent make all choices."

### Reason: the owner's rough designs often win `##a47`

**R2, owner.** Reason: the owner's rough designs often win; agents refine, argue smaller items and measure cost; sometimes the agent's or a fusion wins; the discussion format drives quality. Words: "most of the time, the rough designs I propose end up being better directions that what the agent proposed under the skill, and the agents mostly handles refining them and arguing the smaller items that need to be filled under the general design, as well as measuring any cost of the design. This is not always the case though, and sometimes the better design comes from the agent, or from fusing mine with the agents own. The key point is that I truly think the \"discussion\" format of this skill truly drives design improvments and quality."

### G2 approved, G1 and G3 to be affected `##a48`

**R2, owner.** G2 approved, G1 and G3 to be affected: "I agree with the wording for G2." "What I said above should still impact the wordings of G1 and G3, I think."

### Reading: first-round drafts stand `##a49`

**reply to R2, agent.** Reading: first-round drafts stand; corrected drafts dropped; #thread-closes-on-the-argument read as ruled out, `presumed-settled` "since this is my reading of your words".

### #agent-decides-owner-overrules withdrawn `##a50`

**reply to R2, agent.** #agent-decides-owner-overrules withdrawn: "Its scope was \"everything G1 does not reserve\", measured against the round-2 G1 you just set aside." The intermediary evidence moves to #derived-decision-proceeds-on-default: "A default that the heads answer is the session's decision, by the round-1 wording \"What the record decides is answered by the argument from it\"."

### Diagram: a thread on intent, scope, a cost to weigh, or an open choice closes on the owner's word `##a51`

**reply to R2, agent.** Diagram: a thread on intent, scope, "a cost to weigh", or an open choice closes on the owner's word; a derived thread is stated with its source and built on, shown in the checkpoint. Threads: #derived-decision-proceeds-on-default, #thread-closes-on-the-argument. Words: "The second branch is the \"smaller items\" you describe the agent handling."

### The owner's reason for the discussion form earns a head under entry test 4, attributed not quoted, at … `##a52`

**reply to R2, agent.** The owner's reason for the discussion form earns a head under entry test 4, attributed not quoted, at the harvest; no head records it ("I searched the design home's headings for \"discussion\", \"exchange\" and \"thread\"").

### G1 round 3: slug kept since the title still fits, its 35 citations in 11 files stay valid `##a53`

**reply to R2, agent.** G1 round 3: slug kept since the title still fits, "its 35 citations in 11 files stay valid"; "An agent never substitutes its own judgement for a ruling" removed, its meaning kept in "decision the owner made is recorded as the owner's" and the overrule.

### Wants a fusion of both first-round drafts, not the agent's alone `##a54`

**R3, owner.** Wants a fusion of both first-round drafts, not the agent's alone. Threads: #questions-go-to-their-authority, #owner-decides-goal-wording. Words: "I said the first round draft stand, but what I want is a fusion of both mine and yours from the first round, if that makes sense. It should be inspired by all I said in the previous rounds. I'm not sure what the correct synthesis should be."

### Objection: every thread rests on a cost, so the diagram sends everything to the owner `##a55`

**R3, owner.** Objection: every thread rests on a cost, so the diagram sends everything to the owner. Threads: #owner-rules-at-a-fork, #derived-decision-proceeds-on-default. Words: "nearly every thread rests on a cost to weigh, or can be artificially made so, simply by considering the work itself to be a \"cost\". For example, in the questions-go-to-their-authority thread, you considered 7 lines and 3 rewordings in the primer to be a cost."; "I think every thing will be considered closed on my word."; "This is the point that most bothers me. I do not see a reasonable way to fix that while still accounting for all other constraints..."

### Concedes: A test that every thread passes does not separate anything `##a56`

**reply to R3, agent.** Concedes: "A test that every thread passes does not separate anything."

### Fork test: a cost needs weighing only against a surviving rival `##a57`

**reply to R3, agent.** Fork test: a cost needs weighing only against a surviving rival. Threads: #owner-rules-at-a-fork, #questions-go-to-their-authority. Words: "A cost needs weighing only when there is something to weigh it against: **a second shape that also survives the argument.**"; "If the cheaper rival fails a goal, or loses to a fact, its saving buys nothing."; "after the argument, do two or more shapes still meet every binding constraint, with no fact defeating either?"; "That choice is a **fork**, and a fork is yours."

### Worked case: thread 1's rival lost to a fact, so 7 lines is weighed against nothing `##a58`

**reply to R3, agent.** Worked case: thread 1's rival lost to a fact, so "\"7 lines\" is weighed against nothing."

### The design skill already holds both halves without connecting them to closure `##a59`

**reply to R3, agent.** The design skill already holds both halves without connecting them to closure: loop step 5 "A tie is a legitimate verdict: when the options differ by less than the cost of discriminating between them, say so and let the owner pick."; threads-and-states "When no rival stands […] there is no fork to present, and the closing move is not a question at all." "Today the second case states a default and then still waits for your word. The change is that it closes."

### Sorting check over 9 threads `##a60`

**reply to R3, agent.** Sorting check over 9 threads: the test sends the owner neither everything nor nothing; "Your word changed something in the second group only"; "The test sends you the threads where you added something."


The table of the reply to R3, which sorted the nine threads not withdrawn by the fork test:

| thread | after the argument | goes to |
| --- | --- | --- |
| #answer-keeps-its-authority | rival lost to a fact: Q4 reached the owner through a missed definition, so such answers recur | argument |
| #specification-outranks-an-owner-answer | rival lost to a fact: two thaum reviewers found it contradicts the spec's authority | argument |
| #attribution-replaces-quotation | rival lost to the owner's recorded words in the quotes issue | argument |
| #decisions-reported-to-the-owner | rival lost to a fact: a decision reverted within the session reaches no commit | argument, though the owner may see a fork in the per-turn cost |
| #questions-go-to-their-authority, its reserved list | which questions are the owner's is intent | the owner |
| #owner-decides-goal-wording, G1, G3 | a goal | the owner |
| #owner-word-is-verified | a goal, plus a fork: hold the part or the whole | the owner |
| #derived-decision-proceeds-on-default | blocked by a goal as worded | the owner |
| #thread-closes-on-the-argument | intent about how the owner takes part | the owner |
### Gaming guards `##a61`

**reply to R3, agent.** Gaming guards. Inflating: "a fork names each surviving shape, the binding constraints each one meets, and the trade-off between them. \"It is more work\" alone is no fork." Deflating: "each rival is out only by a named fact or binding constraint, the existing nearest-rival test of loop step 3"; "Every closure appears in the checkpoint". Main premortem candidate: "A session judges a rival \"defeated\" on a weak argument."

### Fused primer section the section question-authority `##a62`

**reply to R3, agent.** Fused primer section the section question-authority: owner answers goals, contradictions, irreversible actions, forks; "A cost alone makes no fork". Fusion attribution: from the owner "goals carry intent; only what is irreversible or unsettled needs a question; a design choice the record settles is not yours"; from the agent "scope and cost weighing stay yours, but only at a fork, which is where your earlier use case lives: superseding a shape as out of scope or overbuilt."

### G1 fused: the goal states the outcome, the survive test is mechanism and lives in the primer `##a63`

**reply to R3, agent.** G1 fused: the goal states the outcome, the "survive" test is mechanism and lives in the primer. Words: "The test of what \"survive\" means is mechanism, so it lives in the primer section above."

### #owner-rules-at-a-fork `##a64`

**reply to R3, agent.** #owner-rules-at-a-fork: discussion stays an exchange, only closure changes; owner's proposals are threads like others. Rival #thread-closes-on-the-argument (set aside). Cost "about 2 sentences in the section threads-and-states, one changed sentence in the section decision-authority (\"The owner owns every decision\"), and the bounded path's wait narrowed to a fork."

### #owner-rules-at-a-fork approved, with a new state `##a65`

**R4, owner.** #owner-rules-at-a-fork approved, with a new state. Words: "yes, this should be the right closure to the problem at hand. We'll go with that. A thread closed on its argument should have its own new state in the thread tables."

### Approvals: questions-go-to-their-authority, owner-decides-goal-wording, decisions-reported-to-the-owner … `##a66`

**R4, owner.** Approvals: "questions-go-to-their-authority, owner-decides-goal-wording, decisions-reported-to-the-owner approved, and goal wordings for G1 and G3 are approved."; "I agree too with the decided section of decisions-reported-to-the-owner."

### Merge is not irreversible `##a67`

**R4, owner.** Merge is not irreversible; each project decides whether agents merge. Threads: #questions-go-to-their-authority. Words: "a merge to main is not an irreversible action (it is a normal part of working in a project), but it should be left to each project to decide whether thay let agents merge, or do it themselves."

### State name argued `##a68`

**reply to R4, agent.** State name `argued`; rival `settled` loses to two facts: "It sits next to the open state `presumed-settled`, so a reader confuses the two. And thaum already uses `settled` as a state of its interpretation register." Closed by the new rule itself, "the first use of the state". A losing rival keeps `ruled-out`, set by the session "when it names the fact that defeats the rival". Threads: #owner-rules-at-a-fork.

### #thread-closes-on-the-argument superseded, since the owner approved #owner-rules-at-a-fork by name `##a69`

**reply to R4, agent.** #thread-closes-on-the-argument superseded, since the owner approved #owner-rules-at-a-fork by name.

### Merge: no installed text names who merges `##a70`

**reply to R4, agent.** Merge: no installed text names who merges ("I searched the installed content for \"merge\" near \"owner\", and the only hits are the retrospective's trigger and the transcript reviewer's row"); primer gains "Which further actions a project reserves to its owner, such as a merge, is the project's to state." Threads: #questions-go-to-their-authority.

### Convergence claim `##a71`

**reply to R4, agent.** Convergence claim: "The last new argument I brought was the fork test in round 4."

### Criteria checkpoint `##a72`

**reply to R4, agent.** Criteria checkpoint: "no two instructions contradict" met "by design: the premise procedure is scoped to intent. To be checked on the written text by the review"; capabilities-not-structure met: "the primer section adds a capability, routing a question by its authority, and removes waits".

### Premortem T1 `##a73`

**reply to R4, agent.** Premortem T1 (#owner-rules-at-a-fork): "Sessions judge a rival defeated on a weak argument"; fires "the owner reopens 2 or more threads closed `argued`, saying a rival was not defeated."

### T2 (#questions-go-to-their-authority) `##a74`

**reply to R4, agent.** T2 (#questions-go-to-their-authority): "the trained weight on human input survives the new text"; fires "2 or more questions put to the owner that a goal, a head or a source text answered"; response "reopen how the routing is delivered, for example at the points where a default is written."

### T3 (#specification-outranks-an-owner-answer) `##a75`

**reply to R4, agent.** T3 (#specification-outranks-an-owner-answer): "a session overrides that departure as \"the text applies at once\""; response "reopen how a departure is recorded, so that a session can tell it from a reading."

### T4 (#attribution-replaces-quotation) `##a76`

**reply to R4, agent.** T4 (#attribution-replaces-quotation): "An attribution loses the scope that a quote would have shown."; fires on a misstated ruling "that traces to an attribution in a durable record, where the original words would have shown the scope."

### T5 (#decisions-reported-to-the-owner) `##a77`

**reply to R4, agent.** T5 (#decisions-reported-to-the-owner): "The \"Decided\" section grows long, and you stop reading it."; fires "the owner overrules, after its merge, a decision that a \"Decided\" section had listed."

### AC1 (net growth) `##a78`

**reply to R4, agent.** AC1 (net growth): fewer sites of the four phrases; "Today's count is 61 + 12 + 2 + 8 = 83", with the grep command.

### Limit of behaviour tripwires `##a79`

**reply to R4, agent.** Limit of behaviour tripwires: "T1, T2, T3 and T5 watch behaviour in sessions" and "fire mostly in other projects", per `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`. Rewording a firing clause is the agent's; "only whether a cause is watched is yours."

### Milestone path, 4 slices `##a80`

**reply to R4, agent.** Milestone path, 4 slices: goals and primer section; design skill (fork, `argued`, bounded path); planning and review (defaults the heads answer, "Decided"); recording side (attribution, test 4, transcript reviewer, owner-intent-stripped). Quotes issue closes at landing; "W1 and W2 of the retrospective are carried by this work. W3 to W5 stay outside it, as you asked."

### T5 dropped, T1 to T4 kept `##a81`

**R5, owner.** T5 dropped, T1 to T4 kept: "Drop T5, keep the other tripwires." No reason given for T5.

### AC1 rejected `##a82`

**R5, owner.** AC1 rejected: does not reliably discriminate, duplicates a reviewer's task as a mechanical check that can misfire. Words: "AC1 does feel weird as a criterion. It is not truly guaranteed to discriminate. I would not keep it. It is basically the task of the spec conformity reviewer, but restated as a mechanical check that can misfire."

### Milestone path agreed, but all of it implemented, reviewed and merged in this session over several PRs `##a83`

**R5, owner.** Milestone path agreed, but all of it implemented, reviewed and merged in this session over several PRs. Words: "Agreed on the milestone path. We'll be implemeting, reviewing and merging all of it together in this session (over multiple PRs), though."

## New names, in one place

| new name | file | slice |
| --- | --- | --- |
| primer section "A question goes to the authority that answers it", slug question-authority | `path@agent-skills@content/PRIMER.md` | 1 |
| primer section on the "Decided" section, slug decided-section | `path@agent-skills@content/PRIMER.md` | 1 |
| goal `the-owner-s-word-is-verified` (G2) | `register@knowledge-architect@goal` | 1 |
| thread state `argued` | `path@agent-skills@content/skills/design/SKILL.md` | 3 |
| heads, slugs as the harvest rows say | `register@agent-skills@design` | 1 to 4 |

Existing, kept: the goals `the-owner-decides` and `agents-work-without-drift`, the state
`ruled-out`, the plan section "Defaults awaiting the owner".

## Decided design

### The authority section

A primer section beside `primer@goals-bind`, so that every session has it, as
`design@agent-skills@primer-content` requires of a directive no skill delivers at the moment it
applies. The fused draft, reply to R3, with the merge clause of R4. Under
`path@agent-skills@content/` its heading carries its slug as the installed text's build requires:

```markdown
## A question goes to the authority that answers it {{slug:question-authority}}

What an external specification the project implements says is answered by its text. What the code or a tool does is answered by a measurement. What follows from the goals and the recorded decisions is answered by the argument from them. A question one of these answers is answered by the session, with its source cited, and the owner is told, not asked.

**The owner answers what none of them settles**: a goal to set or change; a contradiction between the owner's words, the goals and the record that no argument resolves; an action that cannot be undone; and a fork, where two or more shapes meet every binding constraint and no fact defeats either, so that choosing between them weighs a cost or a scope that no goal or decision ranks. A cost alone makes no fork: a shape that fails a goal, or loses to a fact, is out, whatever it would save. A question put to the owner names the authorities consulted and, for a fork, each surviving shape and what it trades. Which further actions a project reserves to its owner, such as a merge, is the project's to state.
```

- **Argument:** the mechanism is that the installed text names the owner as its only authority
  (a5, a7); the owner's premise that trained weight survives instructions (a10) makes routing the
  lever.
- **Nearest rival:** thaum's W1, a sentence on external specifications only. **Defeated by:** the
  same mechanism runs in this repository, which implements no specification (a4, a11, a31).
- **Deliveries**, each a site the slices rewrite: step 3's factual questions narrowed to facts only
  the owner holds; "The owner owns every decision" and "Only the owner's word decides" scoped; the
  design skill's expectation set; the planning and review defaults; the audit's common owner list.
- **Between slices 1 and 4**, per D13.

### An answer keeps its authority

- A label `Q<n>`, `D<n>` or another prefix numbers what was put to the owner, never what decided.
- An answer on a reading or a fact is recorded as such, with its source. Where the source leaves
  the point open and the owner chose, the record says "the owner chose X among the readings the
  text permits".
- The transcript reviewer checks the scope of rulings on intent only. It reports a misread source as
  a reading, not as a misstated ruling.
- **Nearest rival:** leave the recording rules alone, since fewer such answers reach the owner.
  **Defeated by:** a question that wrongly reaches the owner recurs, as thaum's Q4 did through a
  missed definition, and its answer must not harden into a ruling (a12).

### A source outranks an owner's answer on a reading

`primer@owner-word-premise` after slice 1, as a shape and not a text:

1. A word of the owner holds only as far as its premise, as today.
2. An answer on a reading or a fact that a source or a measurement contradicts gives way to the
   source at once: the code and the records follow the source, and the owner is told at the top of
   the turn. A part of the work that cannot be undone still waits.
3. A ruling on intent whose premise proves false is a contradiction, per G2 and D12: the corrected
   premise goes to the owner, and the part of the work that rests on it waits; the rest goes on.

- What a text says is the text's; whether the project follows it is the project's goal or decision.
  A departure from a specification is a scope decision the project states, never inferred from an
  answer to a reading question.
- **Nearest rival:** thaum's first draft, the premise procedure with the text as its default.
  **Defeated by:** it still puts the answer back to the owner, and two of thaum's reviewers found it
  contradicts the specification's authority (a13).

### Bounded work and a derived thread proceed on their default

- The bounded path sends its one message and proceeds on its default; the owner's answer stops or
  redirects it. The commit carries the proposal and the owner's answer, if one came, attributed.
- A default the record or a source answers is the session's decision, reported in the "Decided"
  section, never listed as awaiting the owner. A default that is a fork is listed as awaiting the
  owner and built as written until the owner rules, as today. A contradiction and an action that
  cannot be undone hold their part of the work, per G2 and `primer@owner-word-premise`.
- **Nearest rival:** the wait for the owner's word. **Defeated by:** the owner's evidence that
  defaults the heads answer reach the owner, who then points the agent to the heads (a31), and the
  sites that send them (a39).

### Attribution replaces quotation

- A design head, test 4, a commit message, a tripwire and an issue attribute a decision to the
  owner: "the owner chose X over Y", "on the owner's word". A plan document's thread items keep the
  owner's words verbatim, per `design@agent-skills@plan-records-the-exchange`, and so does a
  committed file that leaves once its outcomes are carried out, per
  `design@agent-skills@committed-audit-report`.
- `tripwire@agent-skills@owner-intent-stripped` fires on a lost attribution instead of a removed
  quotation.
- **Nearest rival:** keep quotations in heads whose ground is the owner's words. **Defeated by:** the
  two quotations bent to pass the checker, which the quotes issue recorded, and the owner's
  statement there that the quotations are of little value "as long as attribution is correct",
  which the owner turned into this decision by approving the thread in R1 (a16, a30).

### The "Decided" section

- A primer section of its own, beside the one on what is met outside the task. A turn that decided
  something ends with a section titled "Decided": one line per decision the session made in that
  turn that the owner could have made, each with its ground as a reference or a source. A thread
  closed `argued`, a default the record answered and a source applied over an owner's answer are
  such decisions. An implementation choice that a commit message records is not one.
- It sits before "Met outside the task" where both are owed. A subagent that decides something
  reports it at the end of its report under the same title, and the dispatching session carries it
  into its own section.
- A rewording the session lists to the owner today at the end of a turn, as of a tripwire's
  wording, is listed there.
- **Nearest rival:** the commit messages and the pull request. **Defeated by:** a decision reverted
  within the session reaches no commit (a41).

### The fork and the state `argued`

```
a thread, after its round of argument
  ├─ two or more shapes survive (a fork)    → closes on the owner's word, as today
  └─ one shape stands, each rival out by a   → closes `argued`; shown in the next delta and in
     named fact or binding constraint            the checkpoint, where the owner contests or
                                                 overrules it
```

- The discussion stays an exchange: the owner proposes designs, the agent fills and argues them, and
  either side's shape can win (a46, a47).
- `argued` joins the closed states, in the one paragraph that defines the closed set and from which
  convergence is computed. A losing rival keeps `ruled-out`, which the session may set when it names
  the fact that defeats it. A reopened thread closes again on its argument or, at a fork, on the
  owner's word.
- A fork names each surviving shape, the binding constraints each meets and the trade-off. "It is
  more work" alone is no fork (a61).
- The design skill already states both halves, a tie for the owner in loop step 5 and "When no rival
  stands […] there is no fork to present" in `skill@knowledge-architect-design@threads-and-states`;
  the change connects the second to closure (a59).
- **Nearest rival:** #thread-closes-on-the-argument, every thread closing on its argument.
  **Defeated by:** the owner's ruling to keep the discussion form, R2 (a46, a47). **Second rival:**
  the round-3 diagram, closure by the owner on intent, scope or a cost. **Defeated by:** every thread
  carries a cost, so it sends everything to the owner (a55, a56).
- **The name `argued`** over `settled`: `settled` sits beside the open state `presumed-settled`,
  and thaum uses `settled` as a state of its interpretation register (a68).

## Mapping tables

Each kind of question the installed text puts to the owner today, and its route after the work. A
site of a kind marked "stays" is kept; a site of a kind marked "session" is rewritten to the
session's decision, reported in the "Decided" section.

| question put to the owner today | route |
| --- | --- |
| a goal to set, reword or abandon | stays |
| a contradiction between a decision and a goal, or among the owner's words | stays, holding its part of the work |
| an action that cannot be undone: a publication, sending content off the machine, data deleted | stays, holding its part of the work |
| a choice between shapes that both survive the argument | stays, as a fork, built on its default meanwhile |
| what a specification's text says | session, from the text |
| what the code or a tool does | session, from a measurement |
| a default a recorded head or goal answers | session, from the argument |
| a member a head's rule admits and its argument covers | session, as today |
| a member the owner's words named | stays: the owner's words are its ground |
| whether a premortem cause is watched, an acceptance criterion applied | stays, per D1 |
| the order of the roadmap | stays, per D2 |
| when audit passes stop; a repair set that grows the text | stays, per D3 |
| a divergence between a design home and the code | per D4 |
| a head in the wrong Component, at a design-record audit | stays, per `design@agent-skills@design-record-axis` |
| a finding on the installed text, at an agentic-workflow audit | stays, per D14 |
| the outcome of a finding of a received workflow file, here | stays, per D14 |
| the destination of an existing document in an adoption | session where a knowledge-table row fits, per D5 |
| a merge to the main branch | the project's to state; this repository reserves it nowhere, per D10 |

## Losing alternatives

Each with what it lost to, and its route under
`skill@knowledge-architect-decision-recording@losing-alternatives`:

| alternative | lost to | route |
| --- | --- | --- |
| the owner's words as a ground, quoted, the old winner of #attribution-replaces-quotation | the bent quotations and the owner's approval, a16 | an entry, at slice 2: refuted by evidence that cost work, and T4 keeps a doubt open |
| thaum's first draft, the premise procedure with the text as its default | two of thaum's reviewers, a13 | an entry, at slice 1: refuted by evidence that cost work |
| #thread-closes-on-the-argument | the owner's ruling on the discussion form, R2 | an entry, at slice 3: some doubt remains that #owner-rules-at-a-fork reaches G3, which T1 watches |
| thaum's W1 as proposed, a sentence on specifications only | a4, a11 | the deliberation only: the argument is derived again in one round |
| #agent-decides-owner-overrules | the owner's ruling on scope, R2 | the deliberation only |
| the round-2 drafts of G1 and of the authority section | the owner's ruling on scope, R2 | the deliberation only |
| the round-3 diagram, closure by the owner on intent, scope or a cost | a55 | the deliberation only |
| the state name `settled` | a68 | the deliberation only |

## Readings

None. The work reads no external specification. The retrospective's case reads one, the
Comprehensive Rules, in thaum; that reading is thaum's and is not repeated here.

## Premortem

Assume the work shipped and failed. Each cause, the thread it stresses, and its verdict. The owner
ruled in R5: "Drop T5, keep the other tripwires." and, on AC1, "I would not keep it."

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| T1 | Sessions judge a rival defeated on a weak argument, and close threads `argued` that the owner would have decided otherwise | #owner-rules-at-a-fork | tripwire, on the owner's word: fires when, in one discussion, the owner reopens 2 or more threads closed `argued`, saying a rival was not defeated; response: reopen the decision on what defeats a rival; re-entry: the retrospective of that session. Written at slice 3's harvest |
| T2 | The trained weight on human input survives the new text, and sessions still put to the owner what the record or a source answers | #questions-go-to-their-authority | tripwire, on the owner's word: fires when a retrospective of a session after the landing finds 2 or more questions put to the owner that a goal, a head or a source text answered; response: reopen how the routing is delivered, for example at the points where a default is written; re-entry: the retrospective. Written at slice 1's harvest |
| T3 | A project departs from its specification on purpose, and a session overrides that departure as "the text applies at once" | #specification-outranks-an-owner-answer | tripwire, on the owner's word: fires when a session applies a source's text over an answer the owner gave as a deliberate departure; response: reopen how a departure is recorded, so that a session can tell it from a reading; re-entry: the retrospective, and the standing-state review. Written at slice 1's harvest |
| T4 | An attribution loses the scope a quotation would have shown | #attribution-replaces-quotation | tripwire, on the owner's word: fires when the transcript reviewer reports a misstated ruling that traces to an attribution in a durable record, where the original words would have shown the scope; response: reopen where verbatim quotations are owed; re-entry: the transcript review of each change. Written at slice 2's harvest |
| T5 | The "Decided" section grows long, and the owner stops reading it | #decisions-reported-to-the-owner | dropped on the owner's word, R5 |
| AC1 | The work adds more instructions than it removes | criterion #net-growth | not applied, on the owner's word, R5; judged by the spec-conformity axis instead |

T1, T2 and T3 watch behaviour in sessions, so they carry the limit
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` records.

## Acceptance criteria

None. The one proposed, AC1, was not applied, R5.

## Implementation sequence

1. Goals and primer, landed: G1, G2, G3; the authority section; the premise procedure scoped; the
   label rule; the "Decided" section; the hold on a divergence; the design skill's material finding
   holding a defeated premise's part.
2. Attribution, landed: the ground and test 4 attribute; the transcript reviewer's ruling check
   scoped; the recording rules of commit messages and the plan record; the reviewers and auditors
   that check quotations; the tripwires that count quotations; D19.
3. [Design skill](design-skill.md): the fork, the state `argued`, the bounded path's default, the
   decision-authority wording, step 3, the expectation set, the texts that count closures.
4. [Defaults and findings](defaults-and-findings.md): planning's and review's defaults, the
   audit's common owner list, setup's destinations.

## Order rationale

- Slice 1 before slice 2: the ruling check of slice 2 narrows to rulings on intent, which the
  authority section of slice 1 sets apart.
- Slice 2 before slice 3: the heads slice 3 writes are then written attributed, and none needs a
  second edit.
- Slice 3 before slice 4: slice 4 rewrites planning's terms and plan reviews, which cite the design
  skill's closure of a reopened thread, `skill@knowledge-architect-design@decision-authority`, that
  slice 3 rewrites.

## Defaults awaiting the owner

Each default awaiting the owner is built as written until the owner rules on it. A label is never
reused. The ones the record answers are listed after these, as decisions of this document the owner
can contest.

- **D1, ruled R8: approved.** Whether a premortem cause is watched, and whether an acceptance criterion is applied,
  stays on the owner's word. Choosing to watch or not is a fork: both shapes meet every binding
  constraint, and they trade the cost of watching against the risk. The owner changed both outcomes
  in R5. `design@agent-skills@premortem-tripwires-on-the-owners-word` and
  `design@agent-skills@acceptance-criteria-on-the-owners-word` stand.
- **D2, ruled R8: approved.** The roadmap's order stays the owner's: no goal ranks one piece of work over another.
  `design@agent-skills@roadmap-order-is-the-owners` stands.
- **D3, ruled R8: approved.** When an audit's passes stop, and a repair set that adds more instructions than it removes,
  stay with the owner, as forks of cost.
  `design@agent-skills@audit-passes-repeat-while-the-count-falls` and
  `design@agent-skills@audit-net-growth` stand.
- **D4, ruled R8: approved, as built.** A divergence between a design home and the code is routed
  by the authority section: where the head stands, the code is the defect, and the session fixes it
  or opens an issue, the work building on the head; where the session finds the head's argument
  defeated, the head's reversal goes to the design skill, and to the owner where it is a fork or the
  head's ground is the owner's, and the part of the task that rests on the point waits for that
  answer. The first draft held the part where the head stands too; the owner was shown that it holds
  only where the argument is found defeated, at the review before slice 1's merge. Slice 1 writes it into `primer@intent-and-claims`
  and `design@agent-skills@divergence-routed-by-its-authority`. It bears on G1.
- **D12, ruled R6: approved.** G2's "contradiction it finds" reads as a contradiction no source or argument resolves,
  as G1 and the authority section say. An owner's answer on a reading that a source contradicts is
  then no contradiction: the source answers it, per #specification-outranks-an-owner-answer. A
  ruling on intent whose premise proves false is one, and its part of the work waits, which replaces
  today's "proceed on the default" of `primer@owner-word-premise` for that case. Proposed
  rewording of G2's second sentence: "A contradiction it finds that no source or argument resolves
  goes to the owner, and the part of the work that rests on it waits for the owner's answer; the
  rest goes on." The rewording changes a goal, so it was not written before the owner's word, per
  `skill@knowledge-architect-goal-setting`; the owner approved it in R6, and slice 1 writes G2 with
  it. It bears on #owner-word-is-verified.
- **D13, ruled R6: approved.** The authority section's own text says that it prevails over any installed instruction
  that sends to the owner a question it gives the session, the same rank `primer@goals-bind` gives
  a goal over a decision. The sentence holds between slice 1 and slice 4, and stays after slice 4,
  since a later edit of a skill can bring the same conflict back. With it, no session meets two
  instructions it cannot both obey: one ranks the other. And no version is published between slice
  1 and slice 4, since a release would ship the texts that disagree; publishing is on the owner's
  word anyway, per `design@knowledge-architect@publish-after-merge`. It bears on criterion
  #no-contradiction.
- **D14, ruled R8: approved.** The intake of a received workflow file, here, and an agentic-workflow audit's findings on
  the installed text, stay with the owner:
  `goal@knowledge-architect@the-workflow-improves-through-real-use` says "the owner decides what
  changes". `skill@klarch-retrospective-intake`,
  `design@knowledge-architect@committed-findings-analysis` and
  `design@knowledge-architect@intake-shares-the-repair-standard` stand. This replaces D6, which
  contradicted that goal. It bears on #derived-decision-proceeds-on-default.
- **D15, ruled R6: approved.** W3, W4 and W5 get a durable outcome in slice 1: one `todo` issue in the root's register,
  which names the retrospective file and its three findings, and closes by an intake of them under
  `skill@klarch-retrospective-intake`. Without it, nothing in the tree records them once this
  document leaves. No analysis file is written now, on the owner's direction in R0 quoted under
  "Outside the work". It bears on D14.
- **D16, ruled R8: approved.** The slices run in the order of "Implementation sequence": attribution second, where the
  reply to R4 (a80) had it last, and the "Decided" section in slice 1, as a primer directive. The
  reason is in "Order rationale". The owner agreed "on the milestone path" in R5 with the earlier
  order in view.
- **D17, ruled R6: approved, as written.** G1 is written as approved, six sentences, though
  `skill@knowledge-architect-goal-setting@goal-entry-shape` asks for "two to four sentences": the
  owner approved its wording verbatim, and every sentence states a part of the goal. The rival is a
  shortened G1, put to the owner before slice 1.

- **D18, ruled R7: approved.** The authority section lists what the owner answers: a goal, a contradiction no argument
  resolves, an action that cannot be undone, and a fork. It leaves out a fact only the owner holds:
  the owner's intent, a plan, a weighing the owner made, the project's users. The primer itself
  asks the owner such questions, at entry test 4 of `primer@design-heads` and in its ground "a
  premise only the owner can state", and `primer@owner-word-premise` defines a ruling on intent by
  that list, so an answer to test 4 is no ruling on intent and no procedure covers its failed
  premise. Proposed: the list gains "a fact only the owner holds, such as an intent, a plan or a
  weighing the owner made", in the authority section, and G1's list gains the same member. The list
  is the owner's approved wording, so nothing was written before the owner's word; the owner
  approved it in R7, "D18: agreed on the proposal.", and slice 1 writes it. It bears on #questions-go-to-their-authority and #owner-decides-goal-wording.

- **D19, ruled R8: approved.** `primer@design-heads` says a head that stands on its argument "is
  reversed by a better argument, which the owner then approves", where D4 and the authority section
  send a reversal to the owner only at a fork or where the head's ground is the owner's. The
  approval clause becomes "which the owner approves where the reversal is a fork", in that section
  and in `design@agent-skills@head-ground-is-the-argument`. Slice 2 builds it, since it rewrites
  both. It bears on #owner-rules-at-a-fork.

**Answered by the record**, each a decision of this document the owner can contest:

- **D5.** In an adoption, the destination of an existing document is the session's where a row of
  the knowledge table fits it, reported; the owner is asked only where no row fits, as the
  knowledge table's last row says. `design@agent-skills@adopting-existing-docs` is rewritten at
  slice 4.
- **D6.** Withdrawn: replaced by D14.
- **D7.** The plan section "Defaults awaiting the owner" keeps its name: it still holds the
  defaults that are forks, and `design@core@plan-document-sections-by-kind` fixes the name.
- **D8.** A committed file that leaves when its outcomes are carried out, an audit report or an
  intake analysis, keeps the owner's words verbatim, per `design@agent-skills@committed-audit-report`
  and `design@knowledge-architect@committed-findings-analysis`.
- **D9.** Existing verbatim quotations in design heads are converted when a change touches the
  head, per `design@agent-skills@existing-heads-on-touch`, or at an audit, not in bulk. The
  CHANGELOG.md entry of slice 2 is a Migration entry that names the design-record axis of
  `skill@knowledge-architect-project-audit`, per `design@knowledge-architect@changelog-entries`.
- **D10.** This repository needs no new text on reserved actions: publishing a version is reserved
  by `design@knowledge-architect@publish-after-merge`, leaving 0.x by
  `design@knowledge-architect@stays-at-zero-x`, and a merge, unreserved, follows `## Git` point 5 of
  the root `CLAUDE.md`.
- **D11.** `design@agent-skills@a-past-sentence-is-rewritten`, and its twin in
  `skill@knowledge-architect-issue-tracking` on a quotation that names a renamed entry, keep their
  clause for a quotation of the owner that remains, as in a plan document or an older head.

## Harvest

This document's own row: it is deleted in the commit that completes slice 4's harvest, once every
default above that awaits the owner is ruled. Each slice's row is in its spec.

## Later consequences

- thaum removes its local override of `primer@owner-word-premise` once its pin reaches the release
  that carries slice 1.
- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` names who rules; it inherits the
  routing of the authority section as the set of questions a person rules on.
- `issue@agent-skills@the-retrospective-counts-no-review-cost` would measure G3's "rather than
  correcting drift"; it stays open.
- The issue D15 opens closes by an intake of W3 to W5.
