# The rules on design heads move into one section of the primer, a head states its rule and stands on its argument, holds one decision, and a directive longer than a pointer is pointed to

## Status and audience

This spec is the plan of the work that changes how the workflow records and reads a design head.
The work does six things:

- the rules on what a design head records and how move out of the decision-recording skill into
  one new section of the primer, and every other installed text keeps a pointer to it, or a
  restatement of one sentence beside its pointer;
- a head's title states the rule that decided, and its body names the members built under it;
- a head stands on its argument, and cites the owner's words only for what came from the owner;
- a head holds one decision, by a stated test;
- a directive is restated where it is delivered only when the restatement is no longer than a
  pointer to it;
- the heads of this repository that cite an approval of the owner as their ground are repaired.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec lands before its work, in a pull request of its own**, per default D9. No gate
  requires it: `cargo klarch commits` judges each commit against that commit's own installed
  copies, per `design@core@installed-entities-from-the-tree`, "A commit is therefore judged against
  the installed set it holds", and the work changes no code of the checker. A plan document on main keeps work done meanwhile
  from drifting from it, per `skill@knowledge-architect-planning@plan-reviews`.
- **The work starts with the design audit** of `skill@knowledge-architect-planning@working-a-slice`
  point 2 when it does not start in the session where the discussion converged, or when commits
  other than this spec's own have landed on main since the spec was written.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/dbcef9fd-3bf3-45c0-9358-058b2e26aa60.jsonl`.
  The discussion begins at the owner's message that opens "I'd like to discuss the issue
  a-head-states-the-instance-built-rather-than-the-principle." and ends at the owner's message
  that opens "I agree with primer-reread-before-recording as you proposed." It holds 5 owner
  messages and 4 agent replies headed "Round 1" to "Round 4". After the reviews of this spec, the
  owner ruled on its defaults in a sixth message, "All defaults approved, proceed.", and on the
  defaults of the last review in a seventh, quoted under D16 to D18. Below,
  "round 0" is the owner's opening message, "round n" is the agent's reply headed "Round n", and
  "the reply to round n" is the owner's message after it. Find the file by that opening message,
  not by its name. The status messages the agent wrote between rounds, while audits returned, are
  called "the grounding of round n" for the round they precede.
- The grounding used read-only audits by subagents, whose tables were scratch files and are not in
  the tree. Every figure below names the audit that took it. A figure is taken again by a fresh
  read-only subagent that reads every head of the design homes named and applies the definition
  of the defect under Names; the figures are evidence for the discussion, and no step depends on
  their magnitude.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **a head**: a design entry, a level-three heading of a design home ending with its slug, and its
  body, per `design@core@an-entry-is-a-heading-at-the-register-level`.
- **the decision-recording skill**: `skill@knowledge-architect-decision-recording`, whose source is
  `path@agent-skills@content/skills/decision-recording/SKILL.md`.
- **the primer**: `path@agent-skills@content/PRIMER.md`, installed as the file the root CLAUDE.md
  imports.
- **the head section**: the new level-two section of the primer this work writes, titled "Design
  heads", with the slug design-heads. Until it exists it is named in plain text; after it, as
  its section reference.
- **the instance**: what a decision admits today: the members built (fixes, kinds, verbs,
  sections, consumers), a count of them, or the one mechanism chosen.
- **the rule**: the property that admits a member, as the head's argument states it.
- **instance as the rule**: a head whose title, or a sentence of its body presented as the
  decision, states the instance where its argument supports the rule, and argues no closure.
- **a title wider than its argument**: a head whose title states more than its argument argues.
- **a bundle**: a head that holds two or more decisions, by the test of
  `thread@head-rules@one-decision-per-head`. The audits classified bundles with an earlier test,
  "would reversing one part leave the other standing?", which `argument@head-rules@a31` shows
  over-splits; their count is an upper bound.
- **W36 to W41**: the ISO 8601 weeks 36 to 41 of 2026; W41 starts on 2026-10-05.
- **the ground of a head**: what the head stands on. **Argued**: its argument, costs and rivals.
  **The owner's words**: a quotation of the owner, where the decision came from the owner.
- **an approval-cited head**: a head that cites an approval of the owner (a single word, a batch
  word, an approved default) as its ground, for a decision that was the agent's argued position or
  default. The defect `thread@head-rules@head-ground-is-the-argument` names.
- **the deletion test**: delete the owner citation from a head; the head passes when its body still
  carries its argument, costs and rivals, or a pointer to its rejected alternatives.
- **the sweep**: the repair of the approval-cited heads, step 6, listed in "Decided design".
- **a restatement**: a directive written again at a point of delivery, away from its home.
- **the 9 inconsistencies**: the inconsistencies I1 to I9 that the inventory of the grounding of
  round 2 found between the decision-recording skill and the texts that restate it, listed under
  "What the work is". The inventory labelled them D1 to D9; this spec calls them I1 to I9.

## What the work is

### The record as it stands

**The rules on heads live in the decision-recording skill**, spread over six of its sections:

| section | rules on heads it holds |
| --- | --- |
| `skill@knowledge-architect-decision-recording@reversal-check` | "The slug and the title must stay aligned with the full scope of the decision", inside the reversal procedure |
| the skill's former entry-tests section | the four entry tests; the comment-and-commit home for a decision that passes none; the backstop sending a decision that creates, contradicts or outgrows a head to the design skill, with "rewrite the head's title to state the addition as well"; a `%%` line citing `design@agent-skills@ruled-items-labelled`, which the build removes from the installed text |
| `skill@knowledge-architect-decision-recording@owning-component` | which Component's design home holds a decision |
| `skill@knowledge-architect-decision-recording@three-homes` | the standing argument, its test, and "who ruled what" kept in the deliberation |
| `skill@knowledge-architect-decision-recording@current-design` | where a design home sits and how a subdocument is linked; present tense; intent, not implementation; slug at level three; the thread's slug as the entry's slug, and the pair when it misdescribes; a list item is no definition; statement first; "A title states a decision only while it is false of the nearest rival it beat. [...] A head that carries several decisions passes the test for each one, or is split."; fidelity to an approval; the template; the reference form; what the argument depends on; reliance on the checker; renaming a slug |
| `skill@knowledge-architect-decision-recording@before-you-finish` | re-read each head for present tense; apply the entry tests again; "rewriting argued text is where fidelity gets lost" |

The skill's description, in its frontmatter, names "the recording tests, which Component owns it,
the split between the design home, the rejected alternatives and history, slug anchors". The
primer's `primer@installed-skills` bullet names only the skill's triggers. The design skill's
description, in `path@agent-skills@content/skills/design/SKILL.md` line 3, restates the backstop's
three cases: "the decision creates a design head, contradicts a statement of one, or takes one
beyond what its title states".

Line numbers below in the design skill are those of its source,
`path@agent-skills@content/skills/design/SKILL.md`, which holds `%%` lines the installed copy does
not.

**About 10 other installed texts restate parts of them**, most with no adjacent pointer. The
inventory of the grounding of round 2 found 9 inconsistencies. Each is read against the tree at
the commit that adds this spec, with the line where it stands and the resolution step 4 applies:

| | the inconsistency | where | resolution |
| --- | --- | --- | --- |
| I1 | which Component owns a decision | the skill's three questions; `path@agent-skills@content/agents/routing-reviewer.md` line 96: "*does this decision survive deleting the Component?*" | the reviewer points to the skill's three questions, which stay in the skill, per default D8 |
| I2 | where the owner's words go for an addition within a title | the skill line 130, "quoted in the commit"; `design@agent-skills@new-or-reshaped-head-needs-design`, "quoted where they gave a ruling" | the routing of a new member, and the rule that an approval goes to the deliberation |
| I3 | "recorded when made" missing the clause "and that is not part of any spec" | the skill line 28 holds it; the primer line 37, the routing reviewer line 110, the design skill lines 135 and 649, and the root CLAUDE.md line 299 do not | each restatement points to `skill@knowledge-architect-decision-recording@when-recording-happens`, or carries the whole sentence with its pointer, under `thread@head-rules@restatement-size-test` |
| I4 | what a measurement in a head carries | the skill line 163, "the measurement it rests on"; the decision-record reviewer line 97, "with the command that takes it again" | the head section states "the measurement it rests on, with the command that takes it again"; the root CLAUDE.md's own paragraph "Never record a number that will go stale", which adds the direction that would reopen the argument, is this repository's addition and stays |
| I5 | the thread's slug as the entry's slug | the design skill line 645, "under its own slug when it earns one", without the skill's exception | the design skill points to the head section for the entry's slug |
| I6 | which losing alternatives are recorded | the design skill line 688, "Record the losing alternatives"; the skill line 269, "Most alternatives that lose earn no entry" | the design skill's keep-or-change says the plan document records them, and points to `skill@knowledge-architect-decision-recording@losing-alternatives` for the entries |
| I7 | which decisions a head references | the skill lines 162 and 163, "the goal or the decision it derives from"; the skill line 253, "A decision of another Component the head depends on" | the head section states the second, which `design@agent-skills@a-reference-claims-a-revisit` records |
| I8 | whether the owner's words go in a head | the skill lines 107 and 108, test 4, "The head quotes the owner's words"; the skill line 160, "who ruled what" in the deliberation | `thread@head-rules@head-ground-is-the-argument` |
| I9 | the decision-record reviewer says it restates nothing | `path@agent-skills@content/agents/decision-record-reviewer.md` lines 19 and 20, "This definition does not restate them", while it restates five rules and leaves out the approval-fidelity check, the extension backstop and "or is split" | the reviewer points to the head section, and keeps its own predicates on what a diff did |

**The primer's restatement rule**, in `primer@where-knowledge-goes`: "A directive is different: it
is restated wherever it has to be delivered, with a reference to its home beside it, and where the
two disagree the restatement is the defect. A restatement is never replaced by a reference on
one-home grounds: whether a directive is needed where it is restated is the owner's decision." The
root CLAUDE.md restates it in `instructions@where-knowledge-goes`. No head records the rule itself.
`design@agent-skills@a-reference-claims-a-revisit` holds one row of it, "a restatement of a
directive | its home", which the root CLAUDE.md's table of when to write a reference restates.

**Sizes**, measured with `wc -w` on the source files at the commit that adds this spec: the primer
holds 1,949 words, the decision-recording skill 4,116. The skill's entry-tests section holds 682
words, `sed -n 81,134p` of the skill; its three-homes and current-design sections together 1,265,
`sed -n 154,264p`.

### What the audits measured

Read-only subagents classified every head of this repository's five design homes, and the heads of
thaum, a project that uses the workflow. Every class is an agent's reading; the session verified a
sample of each audit against the tree and the history, and every sampled claim held.

| defect | this repository | thaum |
| --- | --- | --- |
| instance as the rule | 27 of 189 heads | 16 of 219 heads |
| a title wider than its argument | 5 of 189 | 4 of 219 |
| a bundle | 37 of 189 | 10 of the 40 most recently written |
| approval-cited, whole or in part | 19 of 189 | 1 of the 40 most recently written |

- **Instance as the rule causes rewrites.** About 15 rewrites in the two histories follow one
  shape: a head lists the members built, a member arrives, the head is rewritten, often with its
  slug and its tripwire renamed. Of 15 reversals in this repository's history that the audit of
  the root, gates and xtask homes judged, 6 come from it.
- **Approval citations rose in one week.** Counted by the audit of the root, gates and xtask
  homes, as heads added or rewritten per week across this repository's design homes: W36 to W39, 51 heads, none citing the owner; W40, 230 heads, 24
  citing the owner, 9 of them an approval; W41 to 2026-10-09, 142 heads, 75 citing the owner, 30
  of them an approval, whole or in part. Entry test 4 was added on 2026-10-07.
- **Three verified commits show the title-completeness rule pushing a list into a title**: this
  repository's 937ecee ("the title now states it"), and thaum's 2603114d and 8afc51f7.

### What is outside the work

- **thaum's heads.** The audit found the same defects there; thaum's record is thaum's to change.
- **The heads the audits found with a title wider than their argument, or bundled with a decision
  routed to another home**, `design@core@nothing-of-a-project-is-compiled-in`,
  `design@core@the-regime-has-no-opt-out`, `design@core@registers-are-declared` and the second part
  of `design@knowledge-architect@klarch-prefix`, are brought to the head section's rules when a
  change touches them, as the bundles are. Three heads the audits found false of the tree each have
  an issue entry: `issue@knowledge-architect@the-directory-head-title-is-false-of-the-tree`,
  `issue@xtask@the-subcommands-share-more-than-the-spawn-helper` and
  `issue@agent-skills@the-gates-convention-title-overstates-its-scope`.
- **The bundles and the instance-as-the-rule heads of this repository** are not swept. Each is
  brought to the head section's rules when a change touches it, per
  `thread@head-rules@existing-heads-on-touch`; the heads this work's sweep and harvest touch are
  brought to them in this work, since the rule is written at step 3, before them.
- **The existing long restatements of the root CLAUDE.md**, such as
  `instructions@mechanical-validation` and `instructions@git-workflow`, which
  `thread@head-rules@restatement-size-test` would turn into pointers, are left until a change
  touches them, per default D6. The work touches `instructions@where-knowledge-goes`, which
  restates the rule this work changes.
- **The question of `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason`**, opened on
  the branch of this spec, waits for the owner's ruling; the sweep does not touch the head it is
  about, `design@core@anchors-are-components-and-locations`, whose quotation came from the owner.
- **`design@core@safe-fix-definition`** is also rewritten by `spec@plans@path-quickfixes`, whose
  harvest restates its title. The sweep removes only its approval citation, "the owner ruled this";
  whichever of the two works lands second rebases onto the other.

## What is already decided

The design rests on these, and does not argue them again:

- `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@knowledge-architect@the-owner-decides`,
  `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@agent-skills@installed-text-leaves-room-to-judge`.
- `design@agent-skills@additions-need-real-use`: the evidence for the new rules is the audits'
  commits, cited under "What the work is"; the reread at the head of the decision-recording skill
  was asked for by the owner mid-session, naming what a long session lacks, "dilution over long
  session" (`argument@head-rules@a80`). The reread in the design skill's in-change path rests on
  the agent's prediction alone (`argument@head-rules@a83`), which that head parks as an issue:
  default D10.
- `design@agent-skills@a-reference-claims-a-revisit`: which references a head writes.

The work reverses or rewrites these decisions and texts. Every text that `cargo klarch show` lists
as referencing each, on the branch of this spec at the commit that adds it:

| decision | texts referencing it | judged or updated at |
| --- | --- | --- |
| `design@agent-skills@new-or-reshaped-head-needs-design`: its third case, "the title is reworded to state both decisions, or, where no title can, the addition gets a head of its own", rewritten by `thread@head-rules@one-decision-per-head`; an addition within the title is routed by `thread@head-rules@extension-follows-the-ground`; its paragraph "Two texts deliver it", which places the backstop in the decision-recording skill and says "A line in the primer lost to `design@agent-skills@primer-limit`", rewritten per default D7 | `path@agent-skills@docs/design.md` line 885; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, lines 20 and 43; `path@agent-skills@docs/rejected-alternatives.md` lines 80 and 87; `tripwire@agent-skills@head-created-without-deliberation`, whose heading references it at line 37, whose premise names "the decision-recording skill's backstop" and whose response, at line 49, names "the primer line it rejected" | the harvest |
| `design@agent-skills@standing-argument-in-head`: "who ruled what" stays in the deliberation, except the owner's words that are a decision's ground | `path@agent-skills@docs/design.md` line 1081; `issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle` line 40 | the harvest; the issue closes there |
| `design@agent-skills@a-head-is-owed-by-an-entry-test`: its sentence "which the owner confirms when asked", and the skill's "The head quotes the owner's words and the owner's answer", stay: a head admitted by test 4 is one whose ground is the owner's words, which `thread@head-rules@head-ground-is-the-argument` names "such a case"; its "the four entry tests of the installed decision-recording skill" names the head section; its title's "the heads stay few", per default D12; it is approval-cited in part and swept | `path@agent-skills@docs/design.md` lines 45, 139, 269, 414, 453 and 1278; the issues `issue@agent-skills@no-way-to-audit-a-project-as-a-whole`, `issue@agent-skills@test-3-admits-a-practice-its-tool-documents`, `issue@agent-skills@the-material-finding-duty-has-no-head` and `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined`; `path@agent-skills@docs/rejected-alternatives.md` line 122; `path@agent-skills@docs/tripwires.md` lines 135, 141 and 148 | step 6 for the citation; the harvest for the rest; each referencing text read again there |
| `design@agent-skills@primer-content`: its title lists the primer's directives; it gains the head section, and its title states the rule, per `thread@head-rules@title-states-the-rule` | `path@knowledge-architect@CLAUDE.md` line 410; `issue@agent-skills@the-material-finding-duty-has-no-head` lines 25, 34 and 42; `path@agent-skills@docs/rejected-alternatives.md` line 22 | the harvest |
| `design@agent-skills@primer-limit`: "neither does a procedure a skill delivers when it loads". The head section holds the rules needed to read a head and to tell in advance what a change must be checked against, which no skill delivers at that moment (`argument@head-rules@a72`); the writing procedures stay in the decision-recording skill, per the mapping table. The head is read again, and its text names the head section as what passes its test | `path@agent-skills@CLAUDE.md` line 41; `path@agent-skills@docs/design.md` lines 219, 1146 and 1276; `path@agent-skills@docs/rejected-alternatives.md` line 81 | the harvest |
| `design@agent-skills@design-home-write-loads-recording`: its argument, "that test is held by the skill the condition decides whether to load", and its rival's loss, "makes copies of a test whose home is the decision-recording skill", become false once the title test is in the primer. The load stays, for the recording procedures and the reread; its argument is rewritten | `path@agent-skills@CLAUDE.md` line 66; `path@agent-skills@docs/design.md` line 216; `path@agent-skills@docs/tripwires.md` line 41 | the harvest |
| `design@agent-skills@thread-slug-is-entry-id`: "the decision-recording skill's alignment rule orders a rename" names the head section | referenced by nothing | the harvest |
| `design@agent-skills@in-change-path`: the grounding of the in-change path reads the head section again, per default D10 | `path@agent-skills@docs/design.md` lines 895 and 1084; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` line 20; `path@agent-skills@docs/tripwires.md` lines 55 and 64 | the harvest |
| the restatement rule of `primer@where-knowledge-goes`, which has no head, per `thread@head-rules@restatement-size-test` | `instructions@where-knowledge-goes`, the restatement in the root CLAUDE.md; the row "a restatement of a directive" of `design@agent-skills@a-reference-claims-a-revisit`, which stays true, read again; no text references the primer section by its slug | step 1; the row at the harvest |
| the rejected alternative "A line in the primer that sends a decision met during another task to the design skill", lost to `design@agent-skills@new-or-reshaped-head-needs-design`: the head section carries such a line, per default D7 | `path@agent-skills@docs/rejected-alternatives.md` line 79; `tripwire@agent-skills@head-created-without-deliberation`, which names it as a candidate | the harvest |
| the approval-cited heads, listed in "Decided design", the sweep | each head's own references, read again at the sweep | step 6 |
| the sections of the decision-recording skill whose content moves, cited by section reference: the skill's former entry-tests section from the decision-record reviewer, the design skill (2), the agent-skills design home (2), the issues `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` and `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined`, the skill itself (6) and this spec; `skill@knowledge-architect-decision-recording@current-design` from the agent-skills design home, `spec@plans@path-quickfixes` (2), the skill itself (3) and this spec; `skill@knowledge-architect-decision-recording@three-homes` from this spec. `skill@knowledge-architect-decision-recording@reversal-check` stays, so its references stay, except one that cites it for the alignment clause | as listed | step 2 retargets every reference whose section moves; this spec's own references are rewritten in plain text in step 2's commit, "the skill's former entry-tests section", so that its record of the move stays readable; `cargo klarch check` lists any left dangling |
| the core design home's introduction, "`skill@knowledge-architect-decision-recording` owns the shape" | `path@core@docs/design.md` lines 5 and 6, an introduction and not a head | step 4: it names the head section |
| the skill's description, which names content that moves | the frontmatter of `path@agent-skills@content/skills/decision-recording/SKILL.md` | step 2 |
| the design skill's description, which restates the backstop's three cases | the frontmatter of `path@agent-skills@content/skills/design/SKILL.md` | step 3, reworded to the backstop's cases |
| `design@agent-skills@additions-need-real-use`: not reversed. The in-change reread is built against its rule on a predicted behaviour, per D10 | `design@agent-skills@in-change-path`, which will carry the reread | the harvest, per D16 |
| `design@agent-skills@a-reference-claims-a-revisit`, its row on a restatement of a directive, which names its home: under the approved restatement rule a bare fact, a path, a name or a command, is restated with no pointer | the root CLAUDE.md's table of when to write a reference | the harvest: the row is narrowed to a directive sentence |
| `design@agent-skills@shipped-text-cites-no-entry`: not changed. For installed text, the home a pointer names is installed text, a skill's section or the primer's; a design entry of this repository is a decision's home, which the installed text never cites. The restatement rule is read so | none to update | step 1, where the rule's text says it |
| `issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle`, which this work closes | `spec@plans@path-quickfixes` lines 182, 215, 365 and 1117; `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` line 34 | the harvest: the closing commit rewrites each to state the present |

## Criteria

### No member is admitted beyond what the owner ruled without the owner's word `##c1`

C1. Binding, derived from `goal@knowledge-architect@the-owner-decides`. Met by
`thread@head-rules@extension-follows-the-ground`.

### A head states the argued decision, so a session can tell what it may change and what a change costs `##c2`

C2. Binding, derived from `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Met by
`thread@head-rules@title-states-the-rule` and `thread@head-rules@head-ground-is-the-argument`.

### A member that the argument admits costs no reversal `##c3`

C3. Binding, derived from `goal@knowledge-architect@agents-get-a-complete-workflow`: "fewer
decisions reversed by accident". No slug rename, no tripwire rewrite, no move to the rejected
alternatives. Met by `thread@head-rules@title-states-the-rule` and
`thread@head-rules@extension-follows-the-ground`.

### No fixed structure where judgement serves `##c4`

C4. Binding, derived from `goal@agent-skills@installed-text-leaves-room-to-judge`, such as a
mandatory field in every head. Met: no field is added; the ground shows by whether the owner's
words are cited.

### A title stays false of the nearest rival it beat `##c5`

C5. Binding as a presumption, derived from
`skill@knowledge-architect-decision-recording@current-design`. Met by
`thread@head-rules@title-states-the-rule`, and `thread@head-rules@one-decision-per-head` uses the
rival as its test.

### A title states no more than its argument argues `##c6`

C6. Binding. Stated in round 1: it guards the opposite defect, which caused 2 reversals in this
repository. Met by `thread@head-rules@title-states-the-rule`, "in the argument's terms".

### A session can tell whether a proposed member is inside the approved scope without a large reading of history `##c7`

C7. Weighed. Stated in round 1. Met by `thread@head-rules@extension-follows-the-ground`: the head
itself shows its ground.

### The record tells a decision that came from the owner apart from one the owner approved `##c8`

C8. Binding, derived from `goal@knowledge-architect@the-owner-decides`: "the record shows the
owner's decisions". Met by `thread@head-rules@head-ground-is-the-argument` and
`thread@head-rules@existing-heads-on-touch`.

### Every rule on what a head records and how sits in one location, and no other text restates more than one sentence of it `##c9`

C9. Binding: the owner's direction in the reply to round 1, as `thread@head-rules@restatement-size-test`
bounds it. Met by
`thread@head-rules@one-home-for-head-rules` and `thread@head-rules@restatement-size-test`.

### The design homes stay few heads `##c10`

C10. Weighed, derived from `design@agent-skills@a-head-is-owed-by-an-entry-test`. Round 4 put it
as unmet: `thread@head-rules@one-decision-per-head` adds up to 37 heads before the entry tests
remove riders. The owner, the reply to round 4: "C10 is accepted as unmet. I would not really call
it unmet though. We are not adding content and arguments, only slugs and independent heads, as far
as I understand." The satisfaction line, per default D2: met on the owner's reading, the count of
heads accepted to rise.

## Threads

### A head's title states the rule that decided, and its body names the members built `##title-states-the-rule`

Proposed by the agent, round 1. Approved. Presumed-settled from the reply to round 1, whose word
carried a hedge, "this principle is good I believe"; re-surfaced in round 3. Arguments:
`argument@head-rules@a1`, `argument@head-rules@a3`, `argument@head-rules@a5`,
`argument@head-rules@a8`, `argument@head-rules@a10`, `argument@head-rules@a11`,
`argument@head-rules@a12`, `argument@head-rules@a13`, `argument@head-rules@a78`. Shape: "Decided
design", the same title. Harvest: a head of the agent-skills design home. The owner's words, the
reply to round 3: "I agree with title-states-the-rule."

### An approval is never a head's ground, and the owner's words are, where the decision came from the owner `##head-ground-is-the-argument`

Proposed by the owner in the reply to round 1; written as a thread by the agent in round 2.
Approved. Arguments: `argument@head-rules@a23`, `argument@head-rules@a24`,
`argument@head-rules@a26`, `argument@head-rules@a30`, `argument@head-rules@a33` to
`argument@head-rules@a39`, `argument@head-rules@a44`, `argument@head-rules@a45`. Shape: "Decided
design". Harvest: a head of the agent-skills design home, and
`design@agent-skills@standing-argument-in-head` rewritten. The owner's words, the reply to round 2:
"I agree with head-ground-is-the-argument, unargued-approval-is-argued, one-decision-per-head,
extension-follows-the-ground, existing-heads-on-touch."

### A decision approved with no argument behind it is argued before it is recorded `##unargued-approval-is-argued`

Proposed by the owner in the reply to round 1; written as a thread in round 2. Approved. Where it
runs, at recording in the decision-recording skill, is the default stated in round 2, carried as
D5. Arguments: `argument@head-rules@a25`, `argument@head-rules@a46`, `argument@head-rules@a47`.
Shape: "Decided design". Harvest: the head of `thread@head-rules@head-ground-is-the-argument`, or a
head of its own, as the entry tests and `thread@head-rules@one-decision-per-head` decide at the
harvest. The owner's words: the reply to round 2, quoted under
`thread@head-rules@head-ground-is-the-argument`.

### A head holds one decision, and two parts that lose to different nearest rivals are two heads `##one-decision-per-head`

Proposed by the owner in the reply to round 1; written as a thread in round 2. Approved. It absorbs
`thread@head-rules@title-names-decisions-not-members`. Arguments: `argument@head-rules@a22`,
`argument@head-rules@a31`, `argument@head-rules@a40` to `argument@head-rules@a42`,
`argument@head-rules@a48` to `argument@head-rules@a51`, `argument@head-rules@a85`,
`argument@head-rules@a86`. Shape: "Decided design". Harvest: a head of the agent-skills design
home, and the third case of `design@agent-skills@new-or-reshaped-head-needs-design` rewritten. The
owner's words: the reply to round 2, quoted under `thread@head-rules@head-ground-is-the-argument`.

### A member the head's argument covers is recorded directly, and a head whose ground is the owner's words goes to the owner `##extension-follows-the-ground`

Proposed by the agent in round 2, from the owner's framing in the reply to round 1. Approved. It
supersedes `thread@head-rules@extension-within-the-approval`. Arguments: `argument@head-rules@a2`,
`argument@head-rules@a3`, `argument@head-rules@a4`, `argument@head-rules@a9`,
`argument@head-rules@a54`, `argument@head-rules@a55`. Shape: "Decided design". Harvest: a head of
the agent-skills design home, or `design@agent-skills@new-or-reshaped-head-needs-design` rewritten,
as `skill@knowledge-architect-decision-recording` decides at the harvest. The owner's words: the
reply to round 2, quoted under `thread@head-rules@head-ground-is-the-argument`.

### The 19 approval-cited heads are swept, and the other defects are fixed on touch `##existing-heads-on-touch`

Proposed by the agent in round 1 as "no sweep; rewrite on touch"; widened in round 2 to a sweep of
the approval-cited heads, the list put to the owner before the commit. Approved. Arguments:
`argument@head-rules@a19`, `argument@head-rules@a20`, `argument@head-rules@a39`,
`argument@head-rules@a57`. Shape: "Decided design" and step 6. Harvest: the sweep's commit; the
rule of fixing on touch, a sentence of the head section. The owner's words: the reply to round 2,
quoted under `thread@head-rules@head-ground-is-the-argument`. Round 1's default, "I would add the
audit's list of instance-as-rule heads to the issue entry", was not restated after round 1: default
D4. Writing the rule of fixing on touch into the primer makes it a directive of every installing
project, wider than the owner was shown: default D14. The list of the sweep changes by two heads:
default D11.

### The rules on heads live in one section of the primer, and other texts point to it `##one-home-for-head-rules`

Proposed by the owner in the reply to round 1; written as a thread in round 2 with the location
(a), the decision-recording skill; the agent proposed (b), the primer, in round 3, after the owner's
arguments in the reply to round 2. Approved, shape (b). Arguments: `argument@head-rules@a27`,
`argument@head-rules@a28`, `argument@head-rules@a29`, `argument@head-rules@a32`,
`argument@head-rules@a43`, `argument@head-rules@a52`, `argument@head-rules@a53`,
`argument@head-rules@a59`, `argument@head-rules@a62` to `argument@head-rules@a65`,
`argument@head-rules@a71` to `argument@head-rules@a77`, `argument@head-rules@a79`. Shape:
"Decided design". Harvest: a head of the agent-skills design home;
`design@agent-skills@primer-content` rewritten. The owner's words, the reply to round 3:
"one-home-for-head-rules: we are aligned on (b) then. About cost: I've never seen a session that
did not load the decision record skill, anyway. I'm fairly certain these informations are needed."
The scope default of round 2, "heads only", was not ruled: default D3.

### A directive is restated only when the restatement is no longer than a pointer to it `##restatement-size-test`

Proposed by the owner in the reply to round 2; written as a thread with its wording in round 3.
Approved. Arguments: `argument@head-rules@a58` to `argument@head-rules@a61`,
`argument@head-rules@a66` to `argument@head-rules@a70`. Shape: "Decided design". Harvest: a head
of the agent-skills design home. The owner's words, the reply to round 3: "restatement-size-test: I
agree with this wording, and I believe it is a positive direction." Its scope, new and touched text
only, is default D6.

### The decision-recording skill, and the design skill's in-change path, ask for the head section to be read again `##primer-reread-before-recording`

Proposed by the owner in the reply to round 3, for the decision-recording skill; the agent added
the design skill's in-change path in round 4, "for you to rule on". Approved. Arguments:
`argument@head-rules@a80` to `argument@head-rules@a84`. Shape: "Decided design". Harvest:
`design@agent-skills@design-home-write-loads-recording` and `design@agent-skills@in-change-path`
read again, or a head of its own, as the entry tests decide. The owner's words, the reply to round
4: "I agree with primer-reread-before-recording as you proposed." The word was given against the
round 4 checkpoint table, whose row for this thread read "a reread instruction in the
decision-recording skill; mine: also in the design skill's in-change path", so it closes the
addition with the thread. A finding after the closure bears on the addition: default D10.

### A new member goes to the owner unless the owner's word was on the rule `##extension-within-the-approval`

Proposed by the agent, round 1. Superseded by `thread@head-rules@extension-follows-the-ground`,
which the owner approved by name in the reply to round 2. Arguments: `argument@head-rules@a14`,
`argument@head-rules@a15`, `argument@head-rules@a16`. The owner's reply to round 1 deferred it:
"The other threads are too closely related to my points above, so I'll give my word on them in next
round".

### A member missing from a title is no review finding when the title's rule admits it `##title-names-decisions-not-members`

Proposed by the agent, round 1. Superseded: absorbed by `thread@head-rules@one-decision-per-head`,
which the owner approved by name in the reply to round 2. Argument: `argument@head-rules@a7`,
`argument@head-rules@a17`.

### The design skill asks the owner to rule on the rule as well as the list `##rule-put-to-the-owner`

Proposed by the agent, round 1, "I hold this weakly". Withdrawn by the agent in round 2, with the
defeating reason: "Under your framing, an approval accepts the argument, and the scope of an argued
head is its argument. A separate ruling on "the rule or the list" is no longer needed." Arguments:
`argument@head-rules@a6`, `argument@head-rules@a18`, `argument@head-rules@a56`.

## Arguments

### Reopening a head that did not need it is the cost `##a1`

Round 0, the owner. Bears on `thread@head-rules@title-states-the-rule`. "I think it's quite
problematic, because it forces reopening of design heads that might not have been needed."

### A head stated as a principle may widen what the owner approved `##a2`

Round 0, the owner. Bears on `thread@head-rules@extension-follows-the-ground`. "evaluating whether
promoting records of design heads as more general principles is a good direction or not (it could
lead to a widening of owner-approved designs)."

### An addition within a title is recorded directly, so a wider title widens what an agent may add `##a3`

The grounding of round 1 and round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`,
`thread@head-rules@extension-follows-the-ground`. "An addition within what the title states "is
recorded directly". [...] So making a title wider to state the principle directly widens what an
agent may add without your word." Round 1: "taken alone, it **fails C1**."

### The approval-fidelity rule already sends a widened head to the owner `##a4`

The grounding of round 1, the agent. Bears on `thread@head-rules@extension-follows-the-ground`. "A
clause that widens it, narrows it, or drops part of it can still be false of the nearest rival, and
is still not what was approved: it goes to the owner". Round 1: "But when you approve a plan row
that lists members, it fixes the list as the decision."

### The reference case was partly a new argument, not only an instance clause `##a5`

The grounding of round 1 and round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`.
"the instance clause [...] is a test a session can check mechanically. The principle [...] needs
judgement." Round 1: "That premise is itself as narrow as the instance. A respelling touches a
hand-written file and does not change its meaning. That was a new argument, so the path-quickfixes
discussion was probably needed anyway."

### The material-finding duty has no head, and a rule sending a widening to the owner argues from it `##a6`

The grounding of round 1, the agent. Bears on `thread@head-rules@rule-put-to-the-owner`.
"`issue@agent-skills@the-material-finding-duty-has-no-head`: any rule that sends a widening back to
you would argue from this duty, and the duty has no head yet." Also: two tripwires bear,
"`tripwire@agent-skills@head-created-without-deliberation` fires when a commit adds to a head's
body "a decision its title does not state"" and `tripwire@agent-skills@ruling-lost-in-assembly`
"records three earlier cases of owner rulings "recorded wider than the owner made them"."

### The title-completeness rule pushes members into titles `##a7`

The grounding of round 1 and round 1, the agent. Bears on
`thread@head-rules@title-names-decisions-not-members`. "A review repair, commit 937ecee, moved a
body's second repair into its title: "the title now states it."" Thaum's 2603114d: "A review repair
narrowed an approval that you had given on the principle. [...] Both come from the rule [...] "A
head that carries several decisions passes the test for each one". It pushes enumerations into
titles." Round 1 adds thaum's 8afc51f7: "the title states all four decisions its head carries".

### The defect is measured, and it causes rewrites `##a8`

The grounding of round 1 and round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`,
`criterion@head-rules@c3`. "of 15 reversals the agent judged, 6 come from this defect [...] Each has
the same shape: an argued reason, plus a clause that fixed the state as built." Round 1: "In both
projects, the history holds about 15 rewrites of heads caused by the defect. A typical sequence: a
head lists the members built, a new member arrives, and the head is rewritten. The slug is often
renamed, and its tripwire with it." 43 of 408 heads, 11 %, state the instance as the rule. Silent
drift: thaum's "`two-verbs-not-one` says "`answer` gives a choice and `order` gives a sequence".
Two more verbs, `pairs` and `amounts`, have been added since". `design@agent-skills@primer-content`:
"Three separate commits rewrote it, each to add one directive that the principle in
`design@agent-skills@primer-limit` already admitted."

### Some approvals were given on the instance, and widening those heads widens the approval `##a9`

The grounding of round 1 and round 1, the agent. Bears on
`thread@head-rules@extension-follows-the-ground`, `criterion@head-rules@c1`. "you approved 4 of the
10 on the instance itself, so widening any of them to its principle would widen your approval".
Round 1: "you gave your word on the instance for 9. For 5, your word was on the principle [...]
**So widening the 9 to their principle would widen what you approved.**"

### One sentence names the mechanism `##a10`

Round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`. "A head's title is the line
a later change is tested against, and three steps of the workflow each write into it the list of
what exists rather than the rule that admits it, so every member the rule admits arrives as a
change of the decision."

### A count or a list in a title goes stale at the next member `##a11`

Round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`,
`criterion@head-rules@c2`, `criterion@head-rules@c3`, `criterion@head-rules@c6`. "A count or a list
in a title goes stale at the next member, and every later member then reads as a change of the
decision." "Serves: C2, C3 and C6, because the rule is quoted from the argument."

### A set closed on purpose keeps its list `##a12`

Round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`. "A set closed on purpose
keeps its list in the title, with the sentence that argues the closure in the body."

### Title-states-the-rule alone fails C1 `##a13`

Round 1, the agent. Bears on `thread@head-rules@title-states-the-rule`,
`criterion@head-rules@c1`. "A wider title would therefore let an agent add members you never saw.
That is why the next thread exists."

### A conservative default meets C1, and C7 only partly `##a14`

Round 1, the agent. Bears on `thread@head-rules@extension-within-the-approval`. "Serves: C1,
because the default is conservative, and C3. It meets C7 only partly: the word has to be found, in
the head or in about 2 commits."

### The status quo fails C3 `##a15`

Round 1, the agent. Bears on `thread@head-rules@extension-within-the-approval`,
`criterion@head-rules@c3`. "Nearest rival: the status quo, where every new member goes through the
design skill. It fails C3. The measured cost is about 15 rewrites with slug and tripwire renames."

### The owner's approval scope decides the route of a new member `##a16`

Round 1, the agent. Bears on `thread@head-rules@extension-within-the-approval`. "**The decision** is
the rule in the title. **What you admitted** is the rule or the members, depending on what your word
was given on."

### A member admitted by the rule is no separate decision `##a17`

Round 1, the agent. Bears on `thread@head-rules@title-names-decisions-not-members`. "A member of a
set that the title's rule admits is not a separate decision. [...] Without it, review repairs undo
#title-states-the-rule. That has happened 3 times already."

### Without a ruling on the rule, every member costs a round, and an instruction fits C4 less well `##a18`

Round 1, the agent. Bears on `thread@head-rules@rule-put-to-the-owner`, `criterion@head-rules@c4`.
"Without this, your approvals stay on lists. #extension-within-the-approval then sends every new
member back to you: correct under C1, but each one costs a round." "Nearest rival: no instruction,
leaving it to the session's judgement. That fits C4 better."

### A sweep of the instance-as-the-rule heads would put 9 widenings to the owner `##a19`

Round 1, the agent. Bears on `thread@head-rules@existing-heads-on-touch`. "There is no sweep of the
43 heads. [...] A sweep would mean 9 widenings to put to you here, and an unknown number in thaum,
which is not this repository's to change."

### Widening a head with no word of the owner is still a change `##a20`

Round 1, the agent. Bears on `thread@head-rules@existing-heads-on-touch`. "Rival: a sweep of the 18
heads here where your word was on the principle or absent. Those 18 widen no approval of yours. But
a head with no word from you is still a decision, and widening it is a change."

### The cited commits are the evidence from real use `##a21`

Round 1, the agent. Bears on every thread that changes installed text. "This is changed installed
text, so `design@agent-skills@additions-need-real-use` asks for evidence from real use. The cited
commits are that evidence."

### The skill gives no criterion for splitting a head `##a22`

The reply to round 1, the owner. Bears on `thread@head-rules@one-decision-per-head`. "I'm not
certain there is anything in the decision record skill that gives a split criteria for design
heads. For example, is a head that has two subjects accepted, shaped like this: "that thing is done
like this, and that other thing is done this other way", being recorded as a single design head
when it probably should be two."

### A head is recorded on the arguments discussed, not on the owner's approval `##a23`

The reply to round 1, the owner. Bears on `thread@head-rules@head-ground-is-the-argument`. "it is
the recorded **on the ground of the arguments that were discussed, with costs and rivals stated in
the body or as rejected alternatives**, not on the ground of **owner ruled**. In particular when the
decision was an agent's position or default, and I only approved it."

### The owner's ruling is the ground only where the rivals are stronger, the arguments even, or the owner proposed it `##a24`

The reply to round 1, the owner. Bears on `thread@head-rules@head-ground-is-the-argument`. "The
"owner ruled" recording path should happen for decisions where rivals are stronger, or when no
arguments are in favor of one way or another. Maybe, the correct framing should be that I'm the one
who proposed the design. If you proposed it, argued it as the best, and approved, it is not
"owner-ruled""

### An approval with no argument goes through a search for arguments, costs and rivals `##a25`

The reply to round 1, the owner. Bears on `thread@head-rules@unargued-approval-is-argued`. "And a
decision approved without arguments should always go through a step of searching for arguments in
favor or against, costs and rivals."

### The provenance of a head weighs on every later change `##a26`

The reply to round 1, the owner. Bears on `thread@head-rules@head-ground-is-the-argument`. "in most
cases, recorded heads are the judgement of an agent made by weighing several costs and
alternatives, that the owner then reviewed and approved to keep. The decisions recorded as owner
ruled are those that came directly from the owner, and were not decided by agreement with an
agent's own position. This provenance of design heads matters a lot for agents to judg and weigh
any design change or addition."

### The rules on heads are centralized, consistent and not restated in part `##a27`

The reply to round 1, the owner. Bears on `thread@head-rules@one-home-for-head-rules`. "I want to
make sure the rules for what is recorded in design heads and how are centralized, consistent with
each other and applicable. I think that partial restatements of them should be avoided at all
cost, instead pointing to a single location where they are all together and asking for a read of
that full location when needed."

### The restatement rule has no head, and leaves the delivery decision to the owner `##a28`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@one-home-for-head-rules`. "The rule that protects restatements has no design
head. [...] It states that whether a directive is needed at a point of delivery "is the owner's
decision". Your centralization request is therefore the kind of word it reserves to you."

### A skill load already won over a restatement once `##a29`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@one-home-for-head-rules`. "`design@agent-skills@design-home-write-loads-recording`
chose to load the decision-recording skill before any write into a design home. It rejected
restating the title test at each site, which "makes copies of a test whose home is the
decision-recording skill"."

### thaum's recent heads show no approval cited as a ground `##a30`

The grounding of round 2, the agent. Bears on `thread@head-rules@head-ground-is-the-argument`.
"Provenance: 37 stand on their argument. 2 come from your word, and 1 is mixed. None cites your word
for a decision that was the agent's position." The mixed head, `classic-threshold`, quotes "earlier
is probably better for this kind of test tooling" and "leaves out the part that shows an approval
of the agent's placement", "agreed on your placement".

### Parts that lose to different nearest rivals are separate decisions `##a31`

The grounding of round 2 and round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`.
""would reversing one part leave the other standing?" fired wrongly on parts that share one
argument. The better discriminator was "do the parts' arguments share a premise?"." Core:
"Strongest: **the parts lose to different nearest rivals.** Second: the parts came from separate
plan threads, or separate rulings of yours, merged at the harvest." Round 2: "It over-split: a rule
and its exception always pass it. Four audits independently found the same better signals".

### The restatements of the skill are partial and inconsistent `##a32`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@one-home-for-head-rules`. "There are 9 inconsistencies with that home. [...]
`agent@knowledge-architect-decision-record-reviewer` says "This definition does not restate them",
yet restates five rules. It leaves out three [...] The planning skill's harvest, the step where heads
are actually written, restates the rules partially."

### No installed text asks for an owner-ruled head, and none bounds what a head may cite `##a33`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@head-ground-is-the-argument`. "No installed text asks to record a head as
"owner-ruled". The skill already puts "who ruled what" in the deliberation, not in the head. Only
test 4 asks a head to quote your words." Round 2: "But **no text says what a head may cite as its
ground.** Test 4 asks a head to quote your words, and nothing bounds that to test 4."

### The core's citations of the owner are recent, and transcript reviews added some `##a34`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@head-ground-is-the-argument`. "13 of the 71 core heads cite your word as a
ground, and 5 of those cite an approval of the agent's position. [...] all 14 citing lines were
written between 2026-10-02 and 2026-10-09. [...] Commit 4c90345 says "Why the owner accepted the
two-run upgrade of D1 is in `design@core@fix-before-the-checks`"."

### Words are attributed to the owner that the owner did not write `##a35`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@head-ground-is-the-argument`. "`design@agent-skills@naming-rule` says "the owner
judged it the clearest name for that activity". The agent's default was that name. Your reply was
"I agree with … for the name", and the word "clearest" first appears in the agent's draft of the
head."

### Entry test 4 probably made quoting the owner the way a head shows it passes `##a36`

The grounding of round 2 and round 2, the agent. Bears on
`thread@head-rules@head-ground-is-the-argument`. "Probable cause, an inference: entry test 4 [...]
keeps a head when it records your own intent. Quoting your word then became the way a head shows it
passes." Round 2: "added on 10-07 in commits c9d6df5 and 644889b [...] The rise follows that date." Of the two commits, 644889b adds test 4; c9d6df5 does not, per the code-claims review of this
spec.

### In the most recent week the defect is the dominant pattern `##a37`

Round 2, the agent. Bears on `thread@head-rules@head-ground-is-the-argument`,
`criterion@head-rules@c8`. "Across the whole file, then, the defect is 19 heads of 189. **In the
most recent days, it is the dominant pattern.**" W36 to W39: no citation of the owner in 51 heads;
W40: 24 of 230, 9 of them an approval; W41: 75 of 142, 30 of them an approval.

### Batch approvals are cited as rulings `##a38`

Round 2, the agent. Bears on `thread@head-rules@head-ground-is-the-argument`. "Batch approvals are
cited as rulings. The common forms are "All defaults approved", "Defaults approved on all four",
"Q1: default approved", "the owner accepted the cost". Three of them are still in the tree".
The three sites round 2 named are variants of these forms, not the literal phrases: "The owner
approved this shape while calling it not definitive.", "The cost, which the owner accepted by
name", "Its cost, one more dispatch at each of those moments, the owner accepted.", per the
code-claims review.

### The citation adds a false ground and removes nothing `##a39`

Round 2, the agent. Bears on `thread@head-rules@head-ground-is-the-argument`,
`thread@head-rules@existing-heads-on-touch`. "The deletion test. Delete the citation, and does the
head still carry its argument, costs and rivals? For nearly every approval-cited head it does: the
citation adds a false ground and removes nothing."

### A fifth of the heads are bundles, and the installed text pushes toward bundling `##a40`

Round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`. "37 of the 189 heads here
bundle two or more decisions, about 20 %. [...] **Your suspicion holds, and the installed text pushes
toward bundling.**" `design@agent-skills@new-or-reshaped-head-needs-design` asks "the title is
reworded to state both decisions".

### Every split so far was triggered by a title false of its body, none by a criterion `##a41`

Round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`. "Every split in this
repository's history was triggered by a title that was false of part of its body. None was
triggered by a criterion."

### A head whose parts change one at a time holds several decisions `##a42`

Round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`.
"`design@core@a-commit-message-is-a-document`. Three separate commits changed its parts one at a
time: 9864b86 removed the hook, c3fc3d9 changed how the tip judges each commit, and 049ee30 changed
the summary counts. Each time, the rest stood."

### Four rules on heads have no head of their own `##a43`

Round 2, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "Four rules on heads have
no design head of their own: present tense, title and slug alignment, "a decision worth a slug is
worth its own level-three heading", and approval fidelity."

### The ground tells a later session how the head is reversed `##a44`

Round 2, the agent. Bears on `thread@head-rules@head-ground-is-the-argument`. "A head that stands on
its argument is reversed by a better argument, which the owner then approves. A head whose ground
is the owner's words is reversed by the owner, or by a defeated premise of those words."

### Citing every ruling in its head fails C8 `##a45`

Round 2, the agent. Bears on `thread@head-rules@head-ground-is-the-argument`,
`criterion@head-rules@c8`. "Nearest rival: cite every ruling in its head, which is the current
drift. It fails C8: in W41, 30 of 75 citations would make an approved default read as your intent."

### The search runs at recording, the last point before a head exists `##a46`

Round 2, the agent. Bears on `thread@head-rules@unargued-approval-is-argued`. "My default is at
recording, in the decision-recording skill, since that is the last point before a head exists. The
design skill and the planning skill's defaults would then point to it."

### The search costs one per unargued default `##a47`

Round 2, the agent. Bears on `thread@head-rules@unargued-approval-is-argued`. "Cost: one search per
unargued default. The batches in W41 held up to 8 defaults each."

### A rule's exception, parameter and delivery stay with it `##a48`

Round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`. "A rule's exception, its
parameter and its delivery belong to its head, since none means anything without the rule."

### A title joined by "and" over two decisions shows a bundle `##a49`

Round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`. "A title joined by "and" over
two decisions shows a bundle. Rewording the title to state both does not make them one."

### Splitting adds heads, and the entry tests may remove parts `##a50`

Round 2 and round 4, the agent. Bears on `thread@head-rules@one-decision-per-head`,
`criterion@head-rules@c10`. "Fails C10, at most +37 heads here. But the entry tests then judge each
part alone, and a part that passes none leaves for a comment. [...] The net count is therefore not
known in advance."

### A member of a set is no decision, so its absence from a title is no finding `##a51`

Round 2, the agent. Bears on `thread@head-rules@one-decision-per-head`,
`thread@head-rules@title-names-decisions-not-members`. "Absorbs #title-names-decisions-not-members:
a member of a set that the rule admits is no decision, so a missing member in a title is no
finding."

### A pointer costs each reader the whole section, and a session may never reach it `##a52`

Round 2, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "Cost: each reader reads
the section, likely 120 to 180 lines, where some now read 5 to 10. One delivery risk remains: a
session that never reaches the pointer. That is why the primer rule exists."

### The scope of the centralization: heads only, by default `##a53`

Round 2, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "Scope question: does your
direction hold for the rules on heads only, or for every directive the primer's rule covers? My
default is heads only, with the general question noted as its own thread."

### The head's ground gives a simpler test than the scope of an approval `##a54`

Round 2, the agent. Bears on `thread@head-rules@extension-follows-the-ground`. "Your framing gives a
simpler test than "what did the owner approve" [...] Serves: C1, C3 and C7, since the head itself
shows its ground."

### A wide judgement of "the argument covers it" meets a reviewer `##a55`

Round 2, the agent. Bears on `thread@head-rules@extension-follows-the-ground`. "Risk: the agent
judges "the argument covers M". The design-conformance reviewer reads every plan document against
the heads, so a wrong judgement there meets a reviewer."

### The scope of an argued head is its argument `##a56`

Round 2, the agent. Bears on `thread@head-rules@rule-put-to-the-owner`. "Under your framing, an
approval accepts the argument, and the scope of an argued head is its argument. A separate ruling
on "the rule or the list" is no longer needed."

### The sweep removes a false ground and changes no decision `##a57`

Round 2, the agent. Bears on `thread@head-rules@existing-heads-on-touch`. "Approval-cited heads: a
sweep. There are 19 heads, and the deletion test passes for nearly all. The repair removes a false
ground and changes no decision."

### The restatement rule fits short rules only `##a58`

The reply to round 2, the owner. Bears on `thread@head-rules@restatement-size-test`. "regrding the
primer's rule for restatement, I would say that it should only apply for "short" rules, whose
accuracy can be checked from the diff easily and have little chance to drift."

### A skill cannot be restated inside every other one `##a59`

The reply to round 2, the owner. Bears on `thread@head-rules@one-home-for-head-rules`. "Here, the
recording rules are nearly a whole skill. We can't restate a skill inside every other one, it would
be absurd..."

### The restatement rule exists to avoid loading unrelated text `##a60`

The reply to round 2, the owner. Bears on `thread@head-rules@restatement-size-test`. "Outside of
this case, I think it's a fine rule, which is meant to avoid loading excessive unrelated
informations."

### A pointer to a trivial fact costs as much as the fact `##a61`

The reply to round 2, the owner. Bears on `thread@head-rules@restatement-size-test`. "some skill
were starting to contain instruction of the type "where you can find this file and how it is named
is explained in that other skill" [...] Just state the path/filename, it takes as much space as the
pointer. That's probably the correct test: is the restatement significantly bigger than a pointer ?
Then use a pointer."

### Nearly every piece of work adds or rewrites recorded design `##a62`

The reply to round 2, the owner. Bears on `thread@head-rules@one-home-for-head-rules`. "Those rules
nearly baseline I think, because: - nearly every piece of work will have to add or rewrite recorded
design"

### The rules on heads are needed to interpret recorded design `##a63`

The reply to round 2, the owner. Bears on `thread@head-rules@one-home-for-head-rules`. "- they are
useful to interpret recorded design"

### The rules on heads identify in advance what a change must be checked against `##a64`

The reply to round 2, the owner. Bears on `thread@head-rules@one-home-for-head-rules`. "- it is
useful to be able to identify in advance what kind of decision may need to enter the design home
and it helps identify what kind of change might need to be checked against design."

### Every dispatched subagent holds the primer `##a65`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. Two subagents, a
general-purpose one and `agent@knowledge-architect-routing-reviewer`, with no file read and no
tool: "**Both hold the primer in their initial context.** [...] This was measured in this harness,
Claude Code, only. Another agent provider may load project instructions differently."

### The size test keeps the rule's reason and removes the measured failure `##a66`

Round 3, the agent. Bears on `thread@head-rules@restatement-size-test`. "It keeps your original
reason: a pointer to a trivial fact costs as much as the fact. It removes the failure the inventory
measured: about 10 texts each restate part of a 373-line skill, with 9 inconsistencies."

### One sentence is the checkable form of "significantly bigger than a pointer" `##a67`

Round 3, the agent. Bears on `thread@head-rules@restatement-size-test`. ""One sentence" is my
conversion of your "significantly bigger than a pointer". A pointer with "read it whole" is itself
about one sentence."

### A partial copy drifts and is read as the whole `##a68`

Round 3, the agent. Bears on `thread@head-rules@restatement-size-test`. "Part of it is never
restated: a partial copy drifts, and its reader takes it for the whole."

### The test would turn the root CLAUDE.md's long restatements into pointers `##a69`

Round 3, the agent. Bears on `thread@head-rules@restatement-size-test`. "This repository's root
CLAUDE.md holds long restatements that the test would turn into pointers. [...] Those were your
delivery choices. **Default:** the test governs new text and text a change touches. No sweep of
existing restatements."

### A bare fact needs no pointer beside it `##a70`

Round 3, the agent. Bears on `thread@head-rules@restatement-size-test`. "a bare fact (a path, a
name, a command) needs no pointer beside it. Writing the pointer would double its length."

### Reading rules in the primer and writing rules in the skill would fail C9 `##a71`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`,
`criterion@head-rules@c9`. "The third possible shape, the reading rules in the primer and the
writing rules in the skill, splits the rules over two homes and fails C9, so I do not propose it."

### Reading a head has no load trigger `##a72`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "Reading a head has no
load trigger. [...] Your arguments 2 and 3 name exactly these moments. After this discussion, a
reader must also know what the ground means [...] `design@agent-skills@primer-limit` admits what "no
installed skill delivers at the moment it is needed"."

### A pointer to the primer costs nothing `##a73`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "A pointer to the primer
costs nothing. The probe shows that every subagent already holds the primer. A reviewer then needs
no read instruction, and the planning harvest needs no load."

### The primer roughly doubles `##a74`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "The primer would roughly
double, to about 4,200 words, in every session of every installing project.
`design@agent-skills@primer-limit` states that size is not the bound for the frontier-tier models
the workflow targets. It is still the largest addition the primer has received."

### The location does not fix the failures observed `##a75`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "It does not fix what was
observed. The three measured failures [...] happened in sessions that had the skill loaded, because
they were writing heads. [...] The location serves reading and planning, whose failures nobody has
measured."

### The primer location reshapes two heads `##a76`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`,
`thread@head-rules@title-states-the-rule`. "`design@agent-skills@primer-content`: its title lists the
directives. Under #title-states-the-rule, it becomes a title stating the rule.
`design@agent-skills@primer-limit`: kept, since (b) passes its test on your arguments."

### The skill location is enough only if every reader loads it `##a77`

Round 3, the agent. Bears on `thread@head-rules@one-home-for-head-rules`. "Nearest rival, (a). It is
cheaper per session. It is enough if every session that reads a head loads the skill first, and no
trigger makes that happen. The rule on writing is covered by the load-before-write. The rule on
reading is not."

### The approved threads already rely on title-states-the-rule `##a78`

Round 3, the agent. Bears on `thread@head-rules@title-states-the-rule`. "Your arguments in round 2
already rely on it: #extension-follows-the-ground reads "the head's rule"."

### Every session loads the decision-recording skill anyway, and the rules are needed `##a79`

The reply to round 3, the owner. Bears on `thread@head-rules@one-home-for-head-rules`. "About cost:
I've never seen a session that did not load the decision record skill, anyway. I'm fairly certain
these informations are needed."

### A long session dilutes the primer, and recording comes at its end `##a80`

The reply to round 3, the owner. Bears on `thread@head-rules@primer-reread-before-recording`. "The
point that would worry me however would still be dilution over long session. Maybe a few
instructions to reread the primer should be kept at key locations, in particular the decision
recording skill whose writes happen at the end of sessions (while the design skill is called at the
beginning, with the primer fresh in memory and would need no reread)."

### One instruction in the decision-recording skill reaches every head write `##a81`

Round 4, the agent. Bears on `thread@head-rules@primer-reread-before-recording`. "It covers every
write. `design@agent-skills@design-home-write-loads-recording` loads the skill before any write into
a design home, whatever the activity [...] One instruction in that skill therefore reaches every
head write."

### A review agent starts fresh and needs no reread `##a82`

Round 4, the agent. Bears on `thread@head-rules@primer-reread-before-recording`. "A review agent
starts with a fresh context, and the probe showed it holds the primer, so it needs none either."

### The in-change path runs late in a session by definition `##a83`

Round 4, the agent. Bears on `thread@head-rules@primer-reread-before-recording`. "The design skill is
not always called at the start of a session. Its in-change path runs when a decision is met during
an issue fix or a review repair, which is late by definition. There, the primer is as far back as
at recording."

### Without the in-change reread, the grounding judges heads under a distant primer `##a84`

Round 4, the agent. Bears on `thread@head-rules@primer-reread-before-recording`. "Nearest rival:
your shape alone. It leaves the in-change grounding reading heads under a primer that may be far
back. That grounding reads heads to judge the decision, which is exactly where the ground semantics
are needed."

### Splitting adds slugs and heads, not content `##a85`

The reply to round 4, the owner. Bears on `thread@head-rules@one-decision-per-head`,
`criterion@head-rules@c10`. "I would not really call it unmet though. We are not adding content and
arguments, only slugs and independent heads, as far as I understand. Our solution has little impact
on the amount of content that gets in the design home, it is more about how it is recorded and
interpreted."

### The entry test's purpose is the volume of decisions, which a split does not raise `##a86`

The agent, in the message that closed the discussion. Bears on
`thread@head-rules@one-decision-per-head`, `criterion@head-rules@c10`.
"`design@agent-skills@a-head-is-owed-by-an-entry-test` argues from the cost of reading the design
homes, and it measured over-recording as decisions that no entry test admits. #one-decision-per-head
records no decision that was not already recorded, and adds only headings and slugs. The criterion's
purpose is therefore met."

## New names, in one place

```text
the head section     a level-two section of crates/agent-skills/content/PRIMER.md,
                     titled "Design heads", slug design-heads; installed into every project
                     with the primer
```

No type, function, file or command is added. The section's internal level-three headings, if any,
are section text: the primer's sections are level-two homes of the harness kind `primer`, per
`design@core@section-homes-carry-slugs`.

## Decided design

Each draft text below is copied into the work as approved. A rewording that changes no rule is
listed to the owner in the commit that makes it; one that changes a rule goes to the owner first.
The reviews of this spec are recorded in the messages of its commits whose subjects begin
"[review]"; "the code-claims review" and "the transcript review" below name those.

### The head section

One new level-two section of the primer holds every rule on what a head records and how, as
`thread@head-rules@one-home-for-head-rules` decided. It holds, each as relocated or as new:

| rule | from |
| --- | --- |
| the four entry tests, and the comment-and-commit home of a decision that passes none | the skill's former entry-tests section, relocated, with its `%%` line |
| the standing argument, its test, and the deliberation kept out of the head | `skill@knowledge-architect-decision-recording@three-homes`, relocated |
| present tense; intent, not implementation; the slug at level three; a list item is no definition; statement first; the thread's slug as the entry's slug; the reference form; what the argument depends on, with I7 resolved; the measurement with the command that takes it again, I4; reliance on the checker | `skill@knowledge-architect-decision-recording@current-design`, relocated |
| a title is false of its nearest rival; fidelity to an approval | `skill@knowledge-architect-decision-recording@current-design`, relocated |
| the slug and the title stay aligned with the decision's full scope | `skill@knowledge-architect-decision-recording@reversal-check`, relocated out of the reversal procedure |
| a decision that creates a head, contradicts one or adds a member its argument does not cover goes to the design skill | the backstop of the skill's former entry-tests section, rewritten in step 3 to the draft under "The backstop" |
| a title states the rule, the body names the members | new, `thread@head-rules@title-states-the-rule` |
| a head stands on its argument; the owner's words are a ground only where the decision came from the owner; what the ground means to a later session | new, `thread@head-rules@head-ground-is-the-argument` |
| a head holds one decision | new, `thread@head-rules@one-decision-per-head` |
| how a new member is routed | new, `thread@head-rules@extension-follows-the-ground` |
| a head a change touches is brought to these rules | new, `thread@head-rules@existing-heads-on-touch`, per default D14 |

**What stays in the decision-recording skill**, as the writing procedures that
`design@agent-skills@primer-limit` leaves to a skill that delivers them when it loads: when
recording happens; the reversal procedure without the alignment clause; which Component owns a
decision, per default D8; the three homes' table and the commands that find a deliberation; where
a design home sits and how a subdocument is linked; the template of a head; how to rename a slug;
losing alternatives; tripwires from a premortem; the final checks, pointing to the head section;
the search of `thread@head-rules@unargued-approval-is-argued`, per default D5; and, at its head,
the instruction of `thread@head-rules@primer-reread-before-recording`.

The argument: the rules are needed when a head is read, at every grounding and every plan, not
only when one is written (`argument@head-rules@a62` to `argument@head-rules@a64`); reading a head
loads no skill (`argument@head-rules@a72`); every subagent holds the primer, so a pointer to it
costs nothing (`argument@head-rules@a65`, `argument@head-rules@a73`). The cost: the primer roughly
doubles in every session of every installing project (`argument@head-rules@a74`), which
`design@agent-skills@primer-limit` admits for what no skill delivers at the moment of reading.
The nearest rival, one section of the decision-recording skill, lost because no trigger loads it
when a head is read (`argument@head-rules@a77`). Splitting the reading rules from the writing rules
fails C9 (`argument@head-rules@a71`); the writing procedures that stay in the skill are not rules on
what a head records, and a reader of a head does not need them.

### The title states the rule

Draft text for the head section, approved by the owner as a principle in the reply to round 3.

> **A title states the rule that decided, not the list of what it admits today.** When a decision
> admits members (fixes, kinds, verbs, sections, consumers), or picks a mechanism to meet a
> requirement, the title states the property that admits a member, in the argument's own terms,
> and the body names the members built as what the rule admits today. A count or a list in a title
> goes stale at the next member, and every later member then reads as a change of the decision. A
> set closed on purpose keeps its list in the title, with the sentence that argues the closure in
> the body.

The title stays false of the nearest rival it beat, and states no more than the argument argues.
The nearest rival, the instance in the title, lost to the measured rewrites
(`argument@head-rules@a8`). A rule title alone would widen what an agent may add without the
owner's word (`argument@head-rules@a3`, `argument@head-rules@a13`); the routing of
`thread@head-rules@extension-follows-the-ground` closes that.

### A head stands on its argument

Draft text, approved by the owner in the reply to round 2:

> **A head stands on its argument, and cites the owner only for what came from the owner.** Most
> decisions are proposed, argued over their costs and rivals, and approved by the owner. The head
> records such a decision on that argument, with its costs and rivals in its body or in the
> rejected alternatives. An approval is not a ground, whatever its words ("approved", "agreed",
> "all defaults approved", "accepted the cost"). It stays in the deliberation: the plan document or
> the commit message. The owner's words are a ground, quoted, only where the decision came from the
> owner: the owner proposed it, the owner chose where the argued rivals did not settle it, or it
> rests on a premise only the owner can state, such as an intent, a plan or a weighing the owner
> made. Test 4 is such a case. In a head that holds a part of each, the part that came from the
> owner says so, and the rest stands on its argument.
>
> **What the ground means to a later session.** A head that stands on its argument is reversed by
> a better argument, which the owner then approves. A head whose ground is the owner's words is
> reversed by the owner, or by a defeated premise of those words. An argument against it goes to
> the owner as a question about their intent.

`design@agent-skills@standing-argument-in-head` keeps "who ruled what" in the deliberation; the
owner's words that are a decision's ground are the exception this rule states. The nearest rival,
citing every ruling in its head, fails C8 (`argument@head-rules@a45`).

### A decision approved with no argument is argued before it is recorded

Draft text, approved by the owner in the reply to round 2, for the decision-recording skill, per
default D5:

> **A decision approved with no argument is argued before it is recorded.** Before a head is
> written from an approval that no argument for the decision, no rival and no cost stand behind,
> such as a default approved in a batch, search for the arguments for and against it, its costs and
> its rivals, and record what the search finds. When the search leaves the rivals equal, put the
> fork to the owner as a tie. The owner's choice is then the head's ground, as a decision that came
> from the owner.

The cost is one search per unargued default (`argument@head-rules@a47`).

### A head holds one decision

Draft text, approved by the owner in the reply to round 2:

> **A head holds one decision.** Two statements are one decision when they answer one question and
> lose to the same nearest rival. A rule's exception, its parameter and its delivery belong to its
> head, since none means anything without the rule. Two statements that lose to different nearest
> rivals, or whose arguments share no premise, are two decisions. Each takes a head of its own when
> it passes an entry test, and a comment at its code when it passes none. A title joined by "and"
> over two decisions shows a bundle. Rewording the title to state both does not make them one.

It replaces "A head that carries several decisions passes the test for each one, or is split" and
the third case of `design@agent-skills@new-or-reshaped-head-needs-design`: an addition beyond a
title takes a head of its own unless it answers the head's question. A member of a set the title's
rule admits is no decision, so a member missing from a title is no finding
(`argument@head-rules@a51`). The nearest rival, the reversal test "would reversing one part leave
the other standing?", over-splits a rule from its exception (`argument@head-rules@a31`).

### How a new member is routed

The tree as round 2 presented it, which the owner approved in the reply to round 2:

```text
a new member M, and the head's rule R admits M
├─ the head stands on its argument, and the argument covers M → recorded directly
├─ the head's ground is the owner's words, and they name members → one proposal to the owner
└─ the argument does not cover M                             → the design skill: a change of decision
```

Three cases the tree does not show, each answered by what the approved rules imply:

- **R does not admit M**: M contradicts or outgrows the head, and goes to the design skill, as the
  backstop says.
- **The ground is the owner's words, and they state the rule**: the owner ruled on the rule, so a
  member it admits is within the ruling; it is recorded directly, the words quoted as before.
- **A head with a part of each ground**: the part that admits M decides the branch.

The illustration is a shape, not text to copy. A proposal to the owner is one message stating the
rule, the member and a default, and nothing of it is written before the owner's word; it is not a
change of the decision, so it owes no reversal. The nearest rival, the scope of the owner's
approval decided by finding the word in history, met C7 only partly (`argument@head-rules@a14`,
`argument@head-rules@a54`).

### The backstop

Draft text for the head section, in place of the backstop of
the skill's former entry-tests section. It states
`thread@head-rules@one-decision-per-head` and `thread@head-rules@extension-follows-the-ground`, and
keeps the sentences of the skill's backstop that neither thread changes: the case of a decision met
during another task, the in-change path, and the review axes that judge a relocation:

> **A decision that creates a head, contradicts a statement of one, its argument included, or adds
> a member the head's argument does not cover, goes to the design skill before its text is
> written**, unless it was argued there. This is the case of a decision met during another task
> and settled there, by the owner's word or by the session's own choice. The design skill's
> in-change path keeps the deliberation in the commit message, so the task needs no plan document
> and no new session. An addition that answers another question than the head's takes a head of
> its own, by the test of one decision per head. A member the head's argument covers is recorded
> directly, routed as above; where the owner approved it, the approval is quoted in the commit, not
> in the head. A change that relocates or rewords recorded decisions, a split of a head included,
> and adds or removes none, is not a decision: it needs no design skill, and the routing and
> fidelity-of-relocation review axes judge that it adds or removes none.

### A head a change touches

Draft text for the head section, per default D14:

> **A head that a change touches is brought to these rules in that change**: its title to the rule
> it argues, each decision it bundles to a head of its own, its ground to its argument. Where
> bringing it to the rule would widen what the owner's words in it approved, as a title moved from
> the members the owner named to the rule, the change goes to the owner as one proposal, by the
> routing above.

### The sweep of the approval-cited heads

The heads, from the audits of the grounding of round 2, with the two changes of default D11. "Whole" means the head's ground is an
approval; "in part", that one part is:

| head | defect | the citation, as the audit read it |
| --- | --- | --- |
| `design@agent-skills@naming-rule` | whole | "because the owner judged it the clearest name for that activity" |
| `design@agent-skills@plain-text-is-no-repair` | whole | the owner's "this does not count as evasion", given in approving the agent's default |
| `design@agent-skills@in-change-path` | whole | "The cost, which the owner accepted" |
| `design@agent-skills@roadmap-orders-issues` | whole | "The owner's reason for the shape", for the agent's default D21 |
| `design@agent-skills@conformance-before-every-merge` | whole | "stays parked, on the owner's word" |
| `design@agent-skills@shipped-text-cites-no-entry` | in part | "which the owner approved with the default for the repairs" |
| `design@agent-skills@synthetic-evidence-not-built` | in part | "The owner accepted it" |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | in part | "Agreed on this shape", and "fewer" from a lean |
| `design@agent-skills@standing-entry-search-agent` | in part | "in the owner's judgement", for the agent's shape B |
| `design@agent-skills@transcript-reviewer-agent` | in part | the critical, major and minor scale, read as the owner's |
| `design@agent-skills@plan-read-against-the-record` | in part | "Its cost, one more dispatch at each of those moments, the owner accepted." |
| `design@agent-skills@staged-check-before-each-commit` | in part | "stands on the owner's ruling that it be built", for default D6 of its plan |
| `design@core@an-extension-reads-a-snapshot` | whole | "the owner accepted that cost, shown in that form" |
| `design@core@ne-minimal` | whole | "on the owner's ruling", for the agent's default D5 of its plan |
| `design@core@index-staged-write` | whole | "The owner approved this shape while calling it not definitive." |
| `design@core@safe-fix-definition` | whole | "the owner ruled this" |
| `design@core@harness-kinds` | in part | the owner's "agreed", on the audit's position for kinds |
| `design@knowledge-architect@the-changelog-ships-in-every-crate` | whole | "the owner chose it", for the agent's proposed shape |
| `design@agent-skills@spec-records-the-exchange` | whole | "The cost, which the owner accepted by name", for a cost approved in a batch; found by the code-claims review, added per default D11 |

The repair of each: read its provenance again in its commits and plan documents; remove or reword
the approval citation; apply the deletion test; where the head fails it, write the argument, the
cost or the rival the head lost with the citation, from the deliberation. A part that came from the
owner keeps its quotation. The list and the repair of each are put to the owner before the commit.
`design@knowledge-architect@retrospective-findings-stay-here` leaves the list, per default D11:
its "the owner directs the findings" states the direction that
`design@agent-skills@retrospective-destination` makes the owner's, not a ground taken from an
approval. Two texts understate the owner's role in the other direction and are read in the same
step: the rejected alternative at `path@agent-skills@docs/rejected-alternatives.md` line 116,
"ruled for the path inside the design skill once that was argued", where the transcript of its
discussion shows the owner proposed that path; and the head `design@agent-skills@in-change-path`,
which does not credit the owner's proposal of the widening. A third is read with them: the head
`design@agent-skills@spec-and-milestone` gives the owner's words on untested snippets only as an
observation, "the owner observed", where the rule on snippets was the owner's proposal, per the
agent-skills provenance audit; the step reads its provenance and credits it if the audit holds. The
repair of the first: its reason
says the owner proposed the path, and keeps the argument it lost to. The second is a mixed head
after the sweep: the widening of the in-change path is the owner's proposal, quoted as its ground;
the cost of a long commit message stands on its argument, its approval citation removed.

### A directive longer than a pointer is pointed to

Draft text for `primer@where-knowledge-goes`, approved by the owner in the reply to round 3, in place
of its restatement sentences:

> **A directive is restated where it has to be delivered only when the restatement is no longer
> than a pointer to it**: a path, a file name, a command, a value, or one sentence. A directive
> sentence carries its pointer beside it, and where the two disagree the restatement is the
> defect. A directive longer than one sentence is delivered by a pointer to its home, with an
> instruction to read the home whole at that moment. Part of it is never restated: a partial copy
> drifts, and its reader takes it for the whole. Whether a directive is needed at a point of
> delivery is the owner's decision.

The root CLAUDE.md's `instructions@where-knowledge-goes` restates the old rule and is brought to the
new one. The rival, the old rule, restated a 373-line skill in part in about 10 texts
(`argument@head-rules@a66`).

### The head section is read again before recording, and before an in-change grounding

Draft text, at the head of the decision-recording skill:

> **Read `primer@<head-section>` again, whole, before writing or judging a head.** The primer was
> loaded when the session started, and a recording comes late in it.

And in the design skill's in-change path, approved with the thread; default D10 bears on it:

> Read `primer@<head-section>` again, whole, before grounding.

The placeholder becomes the section's reference once the section exists. Every head write loads the
decision-recording skill first, per `design@agent-skills@design-home-write-loads-recording`, so one
instruction reaches every write (`argument@head-rules@a81`). A review agent starts fresh
(`argument@head-rules@a82`). The full path of the design skill starts a session and needs none
(`argument@head-rules@a80`).

### The other texts point to the head section

Each text below drops its restatement of a head rule and keeps one sentence naming the head section.
The work reads each against the tree; the list is the inventory's of the grounding of round 2:

- `agent@knowledge-architect-decision-record-reviewer`: its predicates on shape, standing argument
  and test 4 become a pointer; its own predicates on what a diff did stay.
- `agent@knowledge-architect-routing-reviewer`: its owning-Component question points to
  `skill@knowledge-architect-decision-recording@owning-component`, resolving I1; "recorded when
  made" points to `skill@knowledge-architect-decision-recording@when-recording-happens`, resolving
  its part of I3; present tense points to the head section.
- `agent@knowledge-architect-design-conformance-reviewer`: its test of a widened head points to the
  routing of a new member.
- `skill@knowledge-architect-design`: the naming of a thread keeps its own rule and points for the
  entry's slug, resolving I5; the keep-or-change "Record the losing alternatives" points to the
  skill's losing alternatives, resolving I6; the in-change path gains its reread.
- `skill@knowledge-architect-planning`: the harvest, point 6 of
  `skill@knowledge-architect-planning@working-a-slice`, points to the head section.
- `skill@knowledge-architect-agent-configuration`, its sentence "that skill judges whether it
  contradicts a head, outgrows its title, or earns text at all", and `skill@knowledge-architect-review@what-review-leaves`,
  its sentence on "a decision that creates a design head, contradicts a statement of one, or takes
  one beyond what its title states": each is one sentence, kept with its pointer, and reworded to
  the backstop's cases.
- The primer's own `primer@intent-and-claims` line on "recorded when made", and the design skill's
  two restatements of it, take the clause of I3 or point to its home.
- The core design home's introduction, "`skill@knowledge-architect-decision-recording` owns the
  shape", names the head section.
- This repository's own texts, touched by the change: `instructions@where-knowledge-goes` and the
  line of the root CLAUDE.md on dates in a head, `path@agent-skills@CLAUDE.md`, and
  `skill@klarch-development`.

## Mapping tables

The relocation of step 2, total over the sections of the decision-recording skill:

| section of the skill | what it becomes |
| --- | --- |
| `skill@knowledge-architect-decision-recording@when-recording-happens` | stays |
| `skill@knowledge-architect-decision-recording@reversal-check` | stays, without the alignment clause, which moves to the head section |
| the skill's former entry-tests section | moves to the head section, with its `%%` line; the section and its slug leave the skill; its backstop is rewritten in step 3 |
| `skill@knowledge-architect-decision-recording@owning-component` | stays, per default D8 |
| `skill@knowledge-architect-decision-recording@three-homes` | the standing argument, its test and the rule that the deliberation is not copied into the head move; the table and the commands stay |
| `skill@knowledge-architect-decision-recording@current-design` | its rules on a head move; where a design home sits, how a subdocument is linked, the template and how to rename a slug stay, under a section of the skill that keeps the slug `current-design` |
| `skill@knowledge-architect-decision-recording@losing-alternatives` | stays |
| `skill@knowledge-architect-decision-recording@premortem-tripwires` | stays, with its own `%%` line citing `design@agent-skills@ruled-items-labelled` |
| `skill@knowledge-architect-decision-recording@before-you-finish` | stays, pointing to the head section for the re-read of each head |

A reference to the skill's former entry-tests section is retargeted to the head
section in step 2's commit. A reference to `skill@knowledge-architect-decision-recording@current-design`
or `skill@knowledge-architect-decision-recording@reversal-check` is retargeted only where it cites a
rule that moves. A consuming project's citation of the skill's section that leaves dangles: default
D15.

## Losing alternatives

- **The rules on heads in one section of the decision-recording skill**, shape (a) of
  `thread@head-rules@one-home-for-head-rules`, lost to shape (b): no trigger loads the skill when a
  head is read (`argument@head-rules@a77`). The probe that every subagent holds the primer is a
  measurement (`argument@head-rules@a65`).
- **The reading rules in the primer and the writing rules in the skill**: lost to
  `thread@head-rules@one-home-for-head-rules`, since it splits the rules over two homes and fails C9
  (`argument@head-rules@a71`).
- **A head cites every ruling of the owner**: lost to
  `thread@head-rules@head-ground-is-the-argument`; it fails C8 (`argument@head-rules@a45`).
- **Every new member goes through the design skill**, the status quo: lost to
  `thread@head-rules@extension-follows-the-ground`; it fails C3 (`argument@head-rules@a15`).
  Already recorded as a rejected alternative of `design@agent-skills@new-or-reshaped-head-needs-design`.
- **The reversal test as the split criterion**: lost to `thread@head-rules@one-decision-per-head`;
  it over-splits a rule from its exception (`argument@head-rules@a31`).
- **A directive is restated wherever it is delivered**, the primer's rule as it stands: lost to
  `thread@head-rules@restatement-size-test` (`argument@head-rules@a66`).
- **A sweep of the instance-as-the-rule heads**, the rival of `thread@head-rules@existing-heads-on-touch`:
  lost, since it would put 9 widenings to the owner and widen heads with no word of the owner
  (`argument@head-rules@a19`, `argument@head-rules@a20`).
- `thread@head-rules@extension-within-the-approval`, superseded by
  `thread@head-rules@extension-follows-the-ground`: a distinct shape, the scope of the approval
  found in history; it lost on C7 (`argument@head-rules@a54`).
- `thread@head-rules@title-names-decisions-not-members`, absorbed whole by
  `thread@head-rules@one-decision-per-head`.
- `thread@head-rules@rule-put-to-the-owner`, withdrawn with its defeating reason
  (`argument@head-rules@a56`).

## Readings

The work reads no external specification.

## Premortem

Each cause was put to the owner in round 4 under its label; the owner, the reply to round 4: "All
tripwires and AC approved."

| label | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| T1 | An agent judges "the argument covers M" too wide, and records a member the owner would not have approved | `thread@head-rules@extension-follows-the-ground` | tripwire. Fires when the owner, or a review, names a member recorded directly that the head's argument does not cover; one instance. Reopens the decision harvested from that thread |
| T2 | The correction overshoots: a quotation that was the owner's own intent is removed or reworded as an argument, and a later session reverses that decision by argument | `thread@head-rules@head-ground-is-the-argument` | tripwire. Fires when a commit removes or reworks a quotation of the owner in a head, and the transcript or the plan document shows the decision came from the owner. Reopens the decision harvested from that thread |
| T3 | Rule titles drift wider than their arguments | `thread@head-rules@title-states-the-rule` | tripwire. Fires when a review finds a title wider than its argument in a head written after the change. Reopens the decision harvested from that thread |
| T4 | The rival test splits a rule from its own exception, or a split is merged back | `thread@head-rules@one-decision-per-head` | tripwire. Fires when a commit merges back two heads split under this rule, or a review finds an exception split from its rule. Reopens the decision harvested from that thread |
| T5 | A directive replaced by a pointer is missed, because the session never reads the home | `thread@head-rules@restatement-size-test` | tripwire. Fires when a review or a retrospective finds a directive broken in a session where its restatement had been replaced by a pointer under this test. Reopens the decision harvested from that thread |
| AC1 | The sweep misses heads, or removes an argument along with a citation | `thread@head-rules@existing-heads-on-touch` | acceptance criterion, `acceptance@head-rules@sweep-leaves-no-approval-ground` |
| AC2 | Partial restatements of the head rules remain in the installed text | `thread@head-rules@one-home-for-head-rules`, `thread@head-rules@restatement-size-test` | acceptance criterion, `acceptance@head-rules@no-partial-restatement-remains` |

## Acceptance criteria

### After the sweep, no head cites an approval as its ground, and every swept head passes the deletion test `##sweep-leaves-no-approval-ground`

AC1. Guards `thread@head-rules@existing-heads-on-touch`. Judged at step 6, after the sweep's commit.
The instrument: a fresh read-only subagent, briefed with this spec's names "an approval-cited head"
and "the deletion test", reads every head of the five design homes. For each head that quotes or
cites the owner, it reads the commits that wrote the citation and the plan document they cite, and
classifies the citation as the owner's own or an approval. It reads each swept head for the
deletion test. Fires on one approval-cited head found, or one swept head that fails the deletion
test. Response: repair the head, and reopen the thread if the miss comes from the rule rather than
the sweep.

### No installed text restates more than one sentence of the head rules `##no-partial-restatement-remains`

AC2. Guards `thread@head-rules@one-home-for-head-rules` and
`thread@head-rules@restatement-size-test`. Judged at step 4 over the installed text, and at the
harvest over the installed text and every text of this repository the work touched; text the work
leaves untouched is out of its scope, per D6. The instrument: a fresh read-only subagent reads
those texts and lists each statement of a rule the head section holds, with its file and line. It
fires on either of two observations: a statement outside the head section longer than one
sentence, or a statement that disagrees with the head section. It also reads each of the 9
inconsistencies at the lines given under "What the work is", and fires on one that remains. Response: replace it by a pointer; a rule that cannot be pointed to reopens
`thread@head-rules@one-home-for-head-rules`.

## Implementation sequence

Steps 1 to 4 change the installed text: each of their commits runs
`cargo klarch install-agent-skills` and commits the installed copies with their source. Steps 5 to
7 change no installed text. Every commit runs `cargo x gates`. A step does not start on a point a
default it names still awaits, unless the owner has ruled.

1. **The restatement rule.** Rewrite the restatement sentences of `primer@where-knowledge-goes` to
   the approved text, with one sentence more: for installed text, the home a pointer names is
   installed text, since installed text cites no entry of this repository, per
   `design@agent-skills@shipped-text-cites-no-entry`. Replace the restatement of the old rule in
   `instructions@where-knowledge-goes` by a pointer to the primer's section, since the approved
   text is longer than one sentence, under `skill@knowledge-architect-agent-configuration`. Waits on
   D6. Fails alone on: the wording of one section.
2. **The relocation.** Write the head section with the rules of the mapping table, relocated with
   no change of meaning; the decision-recording skill keeps what the table says, points to the
   section, and gains the reread instruction; its description names what moved; every reference
   whose section moves is retargeted, and this spec's own references to them are rewritten in plain
   text. The section's heading carries its slug as the build placeholder the source uses,
   `{{slug:design-heads}}`. A dangling reference of the shipped text fails the test gate too, by
   `shipped_set_violations` in `path@core@src/agents.rs`, as well as `cargo klarch check`. Waits on
   D7 and D8. Fails alone on: a rule lost, doubled or changed in the move, read as a diff of the moved
   text.
3. **The new rules.** Add to the head section the rules of
   `thread@head-rules@title-states-the-rule`, `thread@head-rules@head-ground-is-the-argument`,
   `thread@head-rules@one-decision-per-head`, `thread@head-rules@extension-follows-the-ground`
   and, per D14, `thread@head-rules@existing-heads-on-touch`, and the backstop's draft; add the
   search of `thread@head-rules@unargued-approval-is-argued` to the decision-recording skill. Waits
   on D5 and D14. Fails alone on: the new rules contradicting a relocated one.
4. **The pointers.** Replace each restatement listed under "The other texts point to the head
   section" by a pointer, apply the resolution of each of the 9 inconsistencies, add the in-change
   reread to the design skill. Judge AC2. Waits on D10. Fails alone on: a text that lost a rule it
   needed and that neither the head section nor the skill holds.
5. **This repository's own texts.** Bring the root CLAUDE.md lines it touches,
   `path@agent-skills@CLAUDE.md` and `skill@klarch-development` to the head section, under
   `skill@knowledge-architect-agent-configuration`. Fails alone on: a restatement of this
   repository left inconsistent with the head section.
6. **The sweep.** Put the list of the approval-cited heads, with the repair of each, to the owner;
   on the word, repair them and the two texts that understate the owner's role. Judge AC1. Waits on
   D11. Fails alone on: a head's argument lost with its citation.
7. **The harvest and the changelog.** The rows of "Harvest"; the CHANGELOG.md entries of the
   `Next release` section: under Workflow, surface `agent-skills`, the changes to the installed
   skills; under Migration, per D15, the section slug the decision-recording skill loses, with the
   class `design@knowledge-architect@versioning-policy` gives; then `cargo x changelog`; AC2
   judged again; the spec deleted. Waits on D2, D12 and D13.

## Order rationale

- 1 before 2: the restatement rule is what makes the pointers of 2 and 4 legal.
- 2 before 3: the relocation is checked as a move with no change of meaning, which a step that
  also adds rules could not show.
- 3 before 4: the pointers name the section's final rules.
- 4 before 5: this repository's texts follow the installed ones they restate.
- 5 before 6: the sweep applies the rules the head section states.
- 6 before 7: the harvest records the decisions after the sweep has applied them, and AC1 is
  judged before the spec leaves.

## Defaults awaiting the owner

None. D1 to D18 are under the subsection below.

### The defaults the owner ruled on

D1 left the list after the reviews: the owner's word on
`thread@head-rules@primer-reread-before-recording` was given against a checkpoint table that held
the addition, so it closed it. Fourteen defaults stood after the reviews of this spec, D2 to D15.
The owner ruled on all fourteen in one message after them: "All defaults approved, proceed." Each
is applied in the sections it names, and is kept here with its reason.

D16 to D18 came from the last transcript review of the branch; the owner ruled on them in a
seventh message, quoted under each.

- **D16**, from the last transcript review, on D10: the first wording of the ruled D10 had the head
  recording the in-change reread give the owner's ruling as its ground. The ruling was a batch
  word, "All defaults approved, proceed.", which `thread@head-rules@head-ground-is-the-argument`
  says is no ground, and D10 as the owner saw it ruled only that the reread is built. Default: the
  head records the reread on its argument, `argument@head-rules@a83` and `argument@head-rules@a84`,
  and states that it rests on a prediction; an issue entry, opened with the harvest, states what a
  real session would have to show, as `design@agent-skills@additions-need-real-use` asks of a
  predicted behaviour. The owner: "D16: it stands on an argument, but you can skip the issue.
  There's no way to show that the workflow is not functional without it once it is built,
  anyway." Ruled: the head stands on its argument and says it rests on a prediction; no issue is
  opened.
- **D17**, from the last transcript review, on `thread@head-rules@restatement-size-test`: round 3's
  sub-default, "a bare fact (a path, a name, a command) needs no pointer beside it", was not in the
  delta row the owner ruled on; the owner's word named "this wording", whose sentence "A directive
  sentence carries its pointer beside it" implies it. The spec narrows the row of
  `design@agent-skills@a-reference-claims-a-revisit` on it. Default: the sub-default holds, and the
  row is narrowed. Ruled: "Agreed on D17 and D18 defaults."
- **D18**, from the last transcript review, on `thread@head-rules@restatement-size-test`: step 1
  adds a sentence to the approved wording, "for installed text, the home a pointer names is
  installed text", answering a design-conformance finding against
  `design@agent-skills@shipped-text-cites-no-entry`. It changes which home a pointer of installed
  text names. Default: the sentence is added. Ruled: as D17.
- **D2**, on `criterion@head-rules@c10`: the satisfaction line is "met on the owner's reading, the
  count of heads accepted to rise". Ruled: as written.
- **D3**, on `thread@head-rules@one-home-for-head-rules`: its scope is the rules on heads; the
  general question became `thread@head-rules@restatement-size-test`. Default: as written.
- **D4**, on `thread@head-rules@existing-heads-on-touch`: round 1's default, adding the audit's list
  of instance-as-the-rule heads to the issue entry, was not restated. The issue closes with this
  work. Default: no list is kept; a session that touches a head applies the head section.
- **D5**, on `thread@head-rules@unargued-approval-is-argued`: the search runs at recording, in the
  decision-recording skill. Stated in round 2 and round 4 "unless you say otherwise", not
  contested. Default: as written.
- **D6**, on `thread@head-rules@restatement-size-test`: the test governs new text and text a change
  touches, with no sweep of existing restatements. Stated in round 3 and round 4, not contested.
  Default: as written.
- **D7**, on `thread@head-rules@one-home-for-head-rules`, found at assembly: the head section's
  routing of a new member carries "a line in the primer that sends a decision met during another
  task to the design skill", which `path@agent-skills@docs/rejected-alternatives.md` records as lost
  to `design@agent-skills@new-or-reshaped-head-needs-design`, on the reason "The primer holds only
  what every session needs and no installed skill delivers". The owner's arguments
  `argument@head-rules@a62` to `argument@head-rules@a64`, and the agent's `argument@head-rules@a72`,
  defeat that reason for the head rules: they are needed at every reading of a head. Default: the
  line is in the head section; the rejected alternative leaves the file at the harvest, since a
  rejected alternative that is chosen moves out of it; the paragraph "Two texts deliver it" of
  `design@agent-skills@new-or-reshaped-head-needs-design`, and the premise and the response of
  `tripwire@agent-skills@head-created-without-deliberation`, are rewritten with it. The second
  clause of the recorded reason, "the design skill's description carries the symptom", stays true:
  the description keeps the symptom, and the primer line adds a delivery at the moment a head is
  read. The owner's "All defaults approved, proceed." is the ruling that reopens and chooses the
  alternative.
- **D8**, on `thread@head-rules@one-home-for-head-rules`: the first draft of this spec moved the
  owning-Component rules into the head section. Round 3, which the owner approved as shape (b),
  said "The decision-recording skill keeps its procedures and points to that section: when to
  record, the reversal procedure, the owning Component, losing alternatives, tripwires, and the
  final checks." Default: they stay in the skill, as round 3 said; the routing reviewer points to
  them, which resolves I1.
- **D9**, on the order of landing: the first draft said the spec must land before its work because
  its commit would fail under the work's checker. That is false, per the code-claims review:
  `cargo klarch commits` judges a commit against its own installed copies. Default: the spec still
  lands on its own first, since it is already in a pull request of its own and a plan document on
  main keeps work done meanwhile from drifting from it.
- **D10**, a material finding on `thread@head-rules@primer-reread-before-recording`, after its
  closure: the reread in the design skill's in-change path rests on the agent's prediction alone
  (`argument@head-rules@a83`), and `design@agent-skills@additions-need-real-use` says "A finding
  that predicts a behaviour is parked as an issue". The reread in the decision-recording skill meets
  that head: the owner asked for it mid-session and named what a long session lacks. Default: the
  decided shape is built, the in-change reread included. Ruled: built. How its head is recorded is
  D16.
- **D11**, on `thread@head-rules@existing-heads-on-touch`, found by the reviews: the approved list
  of 19 changes by two heads. `design@knowledge-architect@retrospective-findings-stay-here` leaves
  it, since its owner citation states the direction `design@agent-skills@retrospective-destination`
  makes the owner's. `design@agent-skills@spec-records-the-exchange` joins it: "The cost, which the
  owner accepted by name" cites a batch approval, and the sweep's rule admits it. Default: the list
  as written, 19 heads.
- **D12**, on `criterion@head-rules@c10` and `design@agent-skills@a-head-is-owed-by-an-entry-test`:
  that head's title says "the heads stay few", which `thread@head-rules@one-decision-per-head`
  raises in count. The owner's reading of C10 is that the head's principle counts the decisions
  recorded, not the headings. Default: at the harvest, the head's title and its sentence "Fewer
  heads is the principle" are reworded to state the decisions recorded.
- **D13**, from the transcript review: four rules on heads have no head of their own, present
  tense, title and slug alignment, "a decision worth a slug is worth its own level-three heading",
  and approval fidelity (`argument@head-rules@a43`). Default: none gets a head of its own; the head
  harvested from `thread@head-rules@one-home-for-head-rules` records the head section as their home,
  and the entry tests judge each again at the harvest.
- **D14**, on `thread@head-rules@existing-heads-on-touch`: the owner approved fixing this
  repository's existing heads on touch. Writing the rule into the primer makes it a directive of
  every installing project. Default: the rule is written into the head section, as drafted under "A
  head a change touches". Ruled: written into the head section.
- **D15**, found by the code-claims review: the skill's section entry-tests leaves it, and a slug of
  the installed text is an interface, per `design@core@section-homes-carry-slugs`, whose change
  `design@knowledge-architect@changelog-entries` owes a Migration entry for. Default: a Migration
  entry names the section that left and the head section that holds its content.

## Harvest

| item | where it lands |
| --- | --- |
| `thread@head-rules@title-states-the-rule` | a head of `path@agent-skills@docs/design.md`; `design@agent-skills@primer-content` rewritten to state its rule |
| `thread@head-rules@head-ground-is-the-argument` | a head of `path@agent-skills@docs/design.md`; `design@agent-skills@standing-argument-in-head` rewritten; test 4 of `design@agent-skills@a-head-is-owed-by-an-entry-test` read again |
| `thread@head-rules@unargued-approval-is-argued` | the head of `thread@head-rules@head-ground-is-the-argument`, or its own, as the tests decide |
| `thread@head-rules@one-decision-per-head` | a head of `path@agent-skills@docs/design.md`; the third case of `design@agent-skills@new-or-reshaped-head-needs-design` rewritten; the title of `design@agent-skills@a-head-is-owed-by-an-entry-test`, per D12 |
| `thread@head-rules@extension-follows-the-ground` | a head, or `design@agent-skills@new-or-reshaped-head-needs-design` rewritten, as the tests decide |
| `thread@head-rules@existing-heads-on-touch` | the sweep's commit; the sentence of the head section, per D14 |
| `thread@head-rules@one-home-for-head-rules` | a head of `path@agent-skills@docs/design.md`, which records the head section as the home of the four rules of D13; `design@agent-skills@primer-content` rewritten; `design@agent-skills@primer-limit` and `design@agent-skills@design-home-write-loads-recording` read again and rewritten where "What is already decided" says; `design@agent-skills@thread-slug-is-entry-id` names the head section; the rejected alternative of D7 moved out, with the paragraph and the tripwire response D7 names |
| `thread@head-rules@restatement-size-test` | a head of `path@agent-skills@docs/design.md`; the old rule into `path@agent-skills@docs/rejected-alternatives.md` if a recording test admits it; the row of `design@agent-skills@a-reference-claims-a-revisit` read again |
| `thread@head-rules@primer-reread-before-recording` | `design@agent-skills@design-home-write-loads-recording` and `design@agent-skills@in-change-path` read again, or a head, as the tests decide |
| T1 to T5 | `path@agent-skills@docs/tripwires.md`, each naming the head its thread harvests, with its label in its text |
| AC1, AC2 | reported in the landing commit; deleted with the spec unless proposed as tripwires |
| every item of "Losing alternatives" | `path@agent-skills@docs/rejected-alternatives.md`, each as the recording tests admit; the measured probe of shape (a) by test 3 |
| the section slug the skill loses | a Migration entry of CHANGELOG.md, per D15 |
| the citations of this spec from `issue@agent-skills@no-way-to-audit-a-project-as-a-whole` and from the issue this work closes | the commit that deletes the spec rewrites the first to the present, naming the spec in words; the second closes |
| `issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle` | closed; its references in `spec@plans@path-quickfixes` and in `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` rewritten to the present |

## Later consequences

- The bundles and the instance-as-the-rule heads of this repository are fixed when a change touches
  them; no work schedules them.
- thaum receives the head section with the version that ships it, and its own record is its to
  bring to the new rules.
- The root CLAUDE.md's long restatements, such as `instructions@mechanical-validation`, become
  pointers when a change touches them, under `thread@head-rules@restatement-size-test`.
- The probe that every subagent holds the primer was taken in Claude Code only;
  `issue@core@configuration-for-several-agent-providers` holds the question for other providers.
