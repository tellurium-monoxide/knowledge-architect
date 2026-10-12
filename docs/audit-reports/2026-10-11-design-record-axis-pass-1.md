# Design-record axis, pass 1

The first pass of an audit of this repository's decision record on the design-record axis,
`skill@knowledge-architect-project-audit@design-record-axis`, run as one run of the saved workflow
`knowledge-architect-design-record-audit` at the commit "The design-record workflow starts each
group's checkers as soon as its verifier returns", on the branch of
`spec@plans@design-record-workflow`, whose step 3 it is. The session's verification comes first;
parts 1 to 6 are the run's report as that verification leaves it. Spans the checker would refuse,
such as the slugs of heads an edit would create, are quoted in words.

## The session's verification

**What was read.** The report whole; the calibration's three proposed readings against the sample's
drafts; every item of the owner list against the history its evidence cites, each quotation of the
owner found in the commit or the deleted plan document named: 72d9517, 8d4179f (F58 and F64), the
milestone structured-plans at f191c49^ (a42), 4b1240e, 955a610, the deleted spec design-record-audit
(added in the commit "[spec] Design-record audit: a project audit reads the whole record on one
axis, and its first axis re-applies the rules on design heads"), 43fda63 and 249f145; every check;
the seven refuted drafts with their reasons, all quoted in part 1, accepted; and a sample of
conforming drafts: packed-1 22, core-1 10, agent-skills-3 40.

**The acceptance criteria of the spec, judged on this pass.** AC1, #entry-count-matches-the-tree:
the preflight's entries per Component, knowledge-architect 25 heads and 5 rejected alternatives,
core 104 and 70, agent-skills 121 and 42, gates 12 and 4, xtask 4 and 0, equal the session's
count, the heads by a grep of each design home for level-three headings ending with a slug and the
alternatives by their opening lines in each rejected-alternatives file; not fired. AC2,
#one-run-covers-every-entry: one run, 25 agents, none failed, no restart, every listed entry in its
group's return; not fired.

**The readings settled.**

- Reading B, versioning-policy: settled as the calibration proposes. The scope paragraph came from
  the owner's ruling in 72d9517, and "the part that came from the owner says so". Its finding is F2.
- The fix-scope reading: settled as proposed. "`--fix` admits a repair only when it passes the
  safe-fix test" states a necessary condition, which the rival, a fix applying every repair that
  passes, also meets. The title is the owner's wording, so the finding is F3.
- Reading 3, a measurement and the command that takes it again: settled as proposed, with its two
  open questions answered. (a) A count is a measurement, "the one of six" included; an incident
  stated without a count is not. (b) A figure is re-taken by a command, or found by a pointer to
  the record that holds it, the form the design homes already use, "The plan document of that work
  holds the measurement, and `git log --diff-filter=D --name-only -- docs/plans/` finds it". Under
  it, agent-skills-1 21, agent-skills-2 07, 13, 28 and 30 are applied in the branch, each with the
  command or the pointer its record allows, checked at the application; the created head
  saved-workflow-name-is-its-file-stem conforms, since it names the commit whose message records
  the probe; agent-skills-1 22 stays conforming.
- Finding A of versioning-policy reads two ways: F1.

**The session's choices among the run's open points**, part 6.1:

- core-1 48, the rename to plan-document-sections-by-kind: applied in the branch, a slug renamed to
  its decision as the first audit applied its renames, the commit stating the pair the check asks.
- agent-skills-1 49, in-change-path: applied. The ruling F116 of the sixth pass of the
  agentic-workflow axis approved a default, "Go with the defaults.", which is an approval, not the
  owner's words, so neither the removed sentence nor the title is the owner's. The title takes the
  property the verifier names: "A discussion whose decision lands in the change under way keeps its
  deliberation in that change's commit message".
- core-2 02: the missing-directory clause stays in the created head index-takes-no-dry-run, read as
  the delivery of the all-or-nothing write.
- The shared phrase of core-1 18, 23 and core-2 19 cites `goal@core@declared-instructions-are-checked`
  at all three sites.
- agent-skills-2 42 and core-2 23 are applied, and the bundles their heads hold stay with
  `issue@agent-skills@primer-content-bundles-each-directive-s-decision` and
  `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, whose re-entry point this pass
  is: each issue's "What would close it" asks for its splits, which no finding of this pass
  proposes, so each stays open.
- The edits on the heads the spec's harvest rewrites, agent-skills-2 18, 25, 30, and the known
  sentence of 22: applied in the harvest commit of `spec@plans@design-record-workflow`, with
  whatever F19 and F20 rule.

**Added to the owner list by the verification.** packed-1 22, `design@knowledge-architect@klarch-prefix`,
drafted as conforming and marked plausible by its verifier, fails every entry test under the
primer's sentence "each text that states the instruction is one site: a decision stated by one text
has one site": one text states it, the root `CLAUDE.md`. It is F21.

**T2 of the spec's premortem.** Its firing evidence is "the session's sample of conforming drafts
holds one that a reading shows failing a rule". The session's sample holds packed-1 22, which fails.
The session chose it because the drafter flagged its own doubt ("Flagged for the calibration, not
drafted as a failure"), the verifier marked it plausible and the cross-check named it "the one most
likely to become a finding": the run surfaced it, and only its sort into an outcome was left to the
session. The tripwire is not yet written, since the spec's harvest writes it; the session's reading
of the firing was put to the owner beside this file, with the default that T2 is worded at the
harvest so that a draft the report itself lists among its doubts does not count, and the owner
agreed with it: "Agreed on all defaults except F2 and F4".

**The owner's remark on the rule behind F1, F2 and F4**, given with the rulings: "I'm tempted to
remove this rule too. It's not particularly relevant IMO. It is weird to keep those verbatim quotes
in design heads anyway." It is a hedged position on the primer's rule that the owner's words are a
ground, quoted, and goes to an issue entry for a design discussion.

## 1. Header

**Commit audited**: the commit "The design-record workflow starts
each group's checkers as soon as its verifier returns".

**Not a re-check.** No kept list, no `previous` commit, no old-text or new-text classification.
Every verifier states this ("Not a re-check: no old or new text column"). The counts that
`skill@knowledge-architect-project-audit@record-of-an-audit` asks of a re-check (violations left on
old text and on new text, and those on old text that went on the owner list) do not apply to this
pass. For the pass's commit message: the cross-check proposes 20 owner-list items (part 5), all on
text this pass read for the first time.

### The agents of each step

The script is .claude/workflows/knowledge-architect-design-record-audit.js; `AUDITOR` is
knowledge-architect-design-record-auditor (script line 31).

| step | agents | agent type | output |
| --- | --- | --- | --- |
| preflight | 1 | none set in the script (the harness default) | preflight/entries.json, issues.txt, tripwires.txt, history.txt, reports.txt |
| setup | 1 | none set in the script | setup/groups.json, one worktree per group and per checker |
| drafts | 7, one per group | `agent@knowledge-architect-design-record-auditor` | `<group>/NN-<slug>.md`, `<group>/return.md` |
| calibration | 1 | `agent@knowledge-architect-design-record-auditor` | calibration/calibration.md |
| verify | 7, one per group | `agent@knowledge-architect-design-record-auditor` | verify/`<group>`/verdicts.md |
| check | 6, one per group whose verifier counted a created head; one checker per group | `agent@knowledge-architect-design-record-auditor` | check/`<group>`/check-1.md, for packed-1, core-1, core-2, agent-skills-1, agent-skills-2, agent-skills-3 |
| cross-check | 1 | none set in the script | cross-check/cross-check.md |
| report | 1 | none set in the script | this file |

core-3 has no check: its verifier verified no split, merge, rename or move (its only failing draft,
58, was refuted), and the script dispatches a checker only where `createdHeads > 0` (script lines
298-302).

### The groups

| group | entries | first entry | last entry |
| --- | --- | --- | --- |
| packed-1 | 50 | `design@knowledge-architect@no-component-directory-named-after-the-project` | `design@xtask@root-finder-uses-the-core-constant` |
| core-1 | 58 | `design@core@nothing-of-a-project-is-compiled-in` | `design@core@milestone-entries-are-directories` |
| core-2 | 58 | `design@core@a-file-register-index-is-rows` | rejected alternative "A reference with no component read as one inside its own component", crates/core/docs/rejected-alternatives.md |
| core-3 | 58 | rejected alternative "A bare root-relative path form, with a targeted check on the collision cases", crates/core/docs/rejected-alternatives.md | rejected alternative "`index --staged` writing the staged rows into the working-tree file", crates/core/docs/rejected-alternatives.md |
| agent-skills-1 | 55 | `design@agent-skills@content-mirrors-the-install-layout` | `design@agent-skills@plan-reviewed-as-a-commit` |
| agent-skills-2 | 54 | `design@agent-skills@review-repair-appended-or-folded` | `design@agent-skills@staged-check-before-each-commit` |
| agent-skills-3 | 54 | `design@agent-skills@staged-forms-for-a-partial-commit` | rejected alternative "Divergent verdicts between a review and an audit as a finding class with no lens of its own", crates/agent-skills/docs/rejected-alternatives.md |

The sample, each entry with its draft of record: `design@knowledge-architect@versioning-policy`
(packed-1, draft 08, copy 51); `design@core@fix-scope` (core-2, draft 10, copy 60);
`design@agent-skills@capabilities-not-structure` (agent-skills-1, draft 21, copy 58);
`design@gates@gates-run-all` (packed-1, draft 36, copy 54); the rejected alternative "A plans
directory in every component", crates/core/docs/rejected-alternatives.md (core-3, draft 54, copy
63). Every sample entry is also an entry of its group, so the counts below count it once.

### Counts

Counted per entry. Where a verifier's rows count a sample copy as a second draft (packed-1 51 and
54, core-2 60), the copy is not counted again. core-3 and agent-skills-3 give no counts section;
their counts are taken from the rows of their tables.

| group | entries | drafts conforming | drafts failing | verified | refuted | plausible | bears on a ruling | bears on a tripwire | known | kept |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| packed-1 | 50 | 43 | 7 | 47 | 1 (08) | 2 (06, 22) | 2 (09, 25) | 0 | 1 (32) | 0 |
| core-1 | 58 | 52 | 6 | 9 | 2 (32, 51) | 47 | 0 | 0 | 0 | 0 |
| core-2 | 58 | 48 | 10 | 15 | 0 | 43 | 0 | 0 | 3 (20, 37, 41), and finding 2 of 23 | 0 |
| core-3 | 58 | 57 | 1 | 7 | 1 (58) | 50 | 0 | 0 | 0 | 0 |
| agent-skills-1 | 55 | 50 | 5 | 9 | 1 (21, with copy 58) | 45 | 0 | 0 | 0 | 0 |
| agent-skills-2 | 54 | 45 | 9 | 16 | 2 (07, 28) | 36 | 0 | 0 | 2 (22; 42 for the bundle) | 0 |
| agent-skills-3 | 54 | 51 | 3 | 15 | 0 | 39 | 0 | 0 | 0 | 0 |
| **total** | **387** | **346** | **41** | **118** | **7** | **262** | **2** | **0** | **6**, and one half of core-2 23 | **0** |

"Drafts conforming" and "drafts failing" are the drafters' verdicts as each return lists them,
before verification. Counted as the verifiers' rows count drafts, the totals are verified 120,
refuted 8, plausible 262.

Verified findings that propose a split, a merge, a rename or a move, as the verifiers count them:
packed-1 2 (09, 35); core-1 1 (48); core-2 1 (02); core-3 0; agent-skills-1 3 (01, 33, and 04's
rival edit); agent-skills-2 4 (14, 18, 25, 53); agent-skills-3 1 (10, as its check counts: "Verified
findings that propose a split, a merge, a rename or a move: one, draft 10").

### Entries missing from a return

None. Each of the 387 entries of setup/groups.json, by its reference or its name, appears in its
group's return.md (a string match of each over the return, run for this report).

### Refuted drafts, each with its reason

| group / draft | entry | drafter's verdict | verifier's reason, quoted |
| --- | --- | --- | --- |
| packed-1 08 (and copy 51) | `design@knowledge-architect@versioning-policy` | conforms | "wrong under calibration reading B. 72d9517 records that the scope paragraph came from the owner ("The owner then ruled on the policy's scope: "the versionning policy is not supposed to be about checks that are purely internal to knowledge-architect. [...]""). The head does not say so. The draft did not read 72d9517. Finding A (the owner's credits with no quotation) is a reading the rule admits both ways: the owner rules on it, not this verifier" |
| core-1 32 | `design@core@components-carry-the-same-documents` | conforms | "Conforming draft sampled and wrong under the auditor's own standard of 31: "A project free to declare its own set would be conformant with whatever it declared, which is the same as being checked against nothing" (design.md:647-648) is the constraint of goal@core@declared-instructions-are-checked, unnamed. Fails What a head holds; proposed outcome applied in the branch, a missing reference." |
| core-1 51 | `design@core@a-wrong-declaration-is-a-finding` | conforms | "Conforming draft sampled and wrong under the auditor's own standard of 18 and 23: "A manifest that will not load reports nothing at all, and nothing at all is what a session reads as conformance" (design.md:899-900) argues from goal@core@declared-instructions-are-checked without naming it. Fails What a head holds; proposed outcome applied in the branch, a missing reference." |
| core-3 58 | rejected alternative "`index --staged` writing the staged rows into the working-tree file" | fails every recording test | "The recording commit f74044d files this entry under test 4, "(test 4, the owner's "not definitive")", and the deleted spec staged-tree-check (cfc3140^) records the owner's answer to the A-or-B question of #index-staged-write: "B looks fine to me. This is not definitive"; D2 keeps the head stating B as approved, "a later change is an ordinary reversal". The owner's hedge on the winner is the doubt test 4 names, and the entry keeps the rival test 4 exists to keep. The draft read no history and judged test 4 from the entry's text and the tripwire list alone. Its quotations are exact." |
| agent-skills-1 21 (and copy 58) | `design@agent-skills@capabilities-not-structure` | conforms | "Judged again under the calibration's proposed reading 3: the head rests on "In one revision of designing-together, 34 edits produced 10 interaction defects" (line 445) with no command that takes it again; the rule asks for "the measurement it rests on, with the command that takes it again". The section opening (line 342) names the record, not a command. Under that reading the entry fails, outcome applied in the branch (a missing reference, adding the re-take command the draft itself ran, `git show next:docs/decisions.md` in designing-together). The draft flagged this strict reading itself." |
| agent-skills-2 07 | `design@agent-skills@transcript-reviewer-agent` | conforms | "conforms is wrong under the calibration's reading on the measurement rule: "a reviewer briefed with it was the one of six" names its event but no command or record that takes it again, the same defect this auditor failed in 13 and 30" |
| agent-skills-2 28 | `design@agent-skills@workflow-axis-finding-standard` | conforms | "conforms is wrong under the calibration's reading: "6 of the 14 findings of two readings ... at the fifth pass, by a reading of each" names a procedure but no record or command; this auditor failed the same defect in 13 and 30" |

Four of the seven refutations depend on a reading the calibration proposes: packed-1 08 on reading
B, agent-skills-1 21 and agent-skills-2 07 and 28 on reading 3. The session settles those readings
first (part 2, and part 6 for the two open scope questions of reading 3).

## 2. The calibration

Source: calibration/calibration.md. Its summary: "3 proposed readings (versioning-policy, finding B;
fix-scope; capabilities-not-structure). 1 entry whose rule admits both verdicts (versioning-policy,
finding A). gates-run-all and the rejected alternative: every group agrees."

### 2.1 `design@knowledge-architect@versioning-policy` (draft of record: packed-1 08)

| group | verdict | rule named |
| --- | --- | --- |
| packed-1 | conforms | |
| core-1 | fails | What a head holds: an owner ground with no quoted words (finding A) |
| core-2 | conforms | |
| core-3 | fails | What a head holds: the scope paragraph came from the owner and does not say so (finding B) |
| agent-skills-1 | conforms | |
| agent-skills-2 | fails | What a head holds: an owner ground with no quoted words, none on record (finding A) |
| agent-skills-3 | conforms | |

**Finding A: the rule's text admits both readings; the owner rules.** The head: "The table is the
owner's scheme. The bullets below refine it, and the first is the owner's argument, given in the
ruling that a stricter check is minor."

- Reading 1, fails (core-1, agent-skills-2), on the sentence "The owner's words are a ground,
  quoted, only where the decision came from the owner": "quoted" is part of the form of every
  ground that is the owner's. Words on record for the first bullet: e646ead, "The owner ruled
  "minor wins"". For the table: none ("82fc518^:docs/plans/v0-1-extraction.md, section 4.5, says
  only "**The policy (owner's scheme).**"").
- Reading 2, conforms (packed-1, core-2, agent-skills-1, agent-skills-3), on the next sentence: "In
  a head that holds a part of each, the part that came from the owner says so, and the rest stands
  on its argument." "The head says so, part by part, and history confirms both credits (82fc518^
  section 4.5; e646ead)."
- Bearing on the choice, as the calibration quotes it: 8d4179f, "F53:
  `design@agent-skills@review-repair-appended-or-folded` credits its reason to the owner, as a
  paraphrase: no verbatim is on record." The calibration adds: "These rulings concern named items.
  They do not state which reading of the rule holds in general."

**Finding B: proposed reading, fails (core-3).** The rule's sentence: "In a head that holds a part of
each, the part that came from the owner says so, and the rest stands on its argument." The sentence
before it gives, as a case of a decision that came from the owner: "the owner proposed it". History,
72d9517: "The owner raised the question: "I'm not even certain it should have a changelog entry.
[...]" The owner then ruled on the policy's scope: "the versionning policy is not supposed to be
about checks that are purely internal to knowledge-architect. It is about checks added in the
published crate, and which can affect external consumers."" The head's paragraph, "A change
internal to this repository, such as a test, a gate or a subcommand of tools/xtask, is outside the
table, and so is a file a crate ships for reading alone, such as its changelog: each is at most a
patch.", says nothing about the owner. Drafts that differ: packed-1, core-1, core-2, agent-skills-1,
agent-skills-2, agent-skills-3; "None of them read 72d9517; their conforming verdicts on this point
rest on no reading of it." Outcome under this reading: owner list, with core-3's edit as the
default; "The quotation in core-3's edit matches the message of 72d9517 word for word."

### 2.2 `design@core@fix-scope` (draft of record: core-2 10)

Verdicts: core-2 fails ("How a head is shaped: the title is true of its nearest rival"); packed-1,
core-1, core-3, agent-skills-1, agent-skills-2, agent-skills-3 conform.

**Proposed reading: fails (core-2).** The rule's sentences: "A title states a decision only while it
is false of the nearest rival it beat. A title that the losing alternative would make true names a
subject, not a decision". The title: "`--fix` admits a repair only when it passes the safe-fix
test". The nearest rival, as the head names it: "a title stating that `--fix` repairs every finding
whose repair passes the test". The calibration: "Under the rival, `--fix` applies a repair only when
it passes the test, so the title is true of it. "only when it passes" states a necessary condition,
and both shapes meet it." and "The six conforming drafts call the title false of that rival but
show no reason." Outcome under this reading: owner list: "The title is the owner's own wording
(8d4179f, F64: "I'd word it the other way: "--fix only admit repairs when they pass the safe-fix
test""), and the ground is the owner's words, so retitling touches them."

### 2.3 `design@agent-skills@capabilities-not-structure` (draft of record: agent-skills-1 21)

Verdicts: core-2, core-3, agent-skills-2 fail ("What a head holds: the measurement has no command
that takes it again"); packed-1, core-1 conform; agent-skills-1 and agent-skills-3 conform, each
noting that a strict reading would fail.

**Proposed reading ("reading 3"): fails.** The rule's sentence: "**The standing argument** is every
premise whose failure would reopen the decision: [...] the measurement it rests on, with the command
that takes it again". The head rests on "In one revision of designing-together, 34 edits produced
10 interaction defects, nearly all between rules that scripted one exchange. So a gap is weighed by
its kind." The calibration: "The conforming reading, that a figure from another project may give
its source alone (agent-skills-3: "there is no command to add"), has no sentence in the rule behind
it. A command can be written."

A correction to the failing drafts, quoted: "The section that holds the head opens, at
crates/agent-skills/docs/design.md line 342: "The measurements and incidents these entries cite
were taken in the designing-together repository, whose record is docs/decisions.md on its branch
next." The place is named, at the section's level. The command is what is missing. So the edit
should add the command over that record, not a pointer to the intake commit as core-2, core-3 and
agent-skills-2 propose." The calibration did not run the command: "That repository is not in this
tree, so this calibration did not run the command." Outcome under this reading: applied in the
branch, a missing reference added by a rewording.

### 2.4 `design@gates@gates-run-all` (draft of record: packed-1 36)

Every group: conforms. No reading proposed.

### 2.5 Rejected alternative "A plans directory in every component" (draft of record: core-3 54)

Every group: conforms. "The groups name different recording tests: test 1 (packed-1,
agent-skills-3), test 4 (core-1, agent-skills-1), or both (core-2, core-3, agent-skills-2). Every
verdict holds under either test, so no reading is proposed."

### 2.6 Drafts the verifiers judged again under a proposed reading

| verifier | drafts judged again | reading | result |
| --- | --- | --- | --- |
| packed-1 | 08, 51 | B | refuted. Then the auditor's other drafts on "an owner-sourced part said so" were read again: 02, 04, 06, 07, 12, 15, 16. "Only 06 meets the rule's question, and it is marked plausible." |
| core-1 | none | | "The calibration's three proposed readings bear on versioning-policy, fix-scope and capabilities-not-structure, whose drafts of record are packed-1's, core-2's and agent-skills-1's." |
| core-2 | 10, 60 | fix-scope, fails | "the finding holds" |
| core-3 | none | | the readings bear on other groups' drafts of record; section 5 "proposes no reading" |
| agent-skills-1 | 21, 58 | 3 | refuted. Then 22, 27, 30 were read again on the same rule: "None is refuted." 22 left plausible ("an incident, not a measurement re-taken by a command"); 27 confirmed ("lines 591-594 give the measurement with its re-take, a scan of session logs"); 30 plausible ("line 692 states that a figure "is not measured" and cites an issue") |
| agent-skills-2 | 13, 30 (failing, confirmed); 07, 28 (conforming, refuted); 34, 32 (conforming, confirmed); 20 (plausible); 18, 25 ("their figures carry git log --grep='design-record axis'") | 3 | as listed |
| agent-skills-3 | none | | "premortem-as-watch-points gives its command (git show 61a9219^:...), so draft 11 is unaffected" |

## 3. The findings

Grouped by the outcome the cross-check proposes (cross-check part 3.2), then by Component and file.
Each finding gives the entry, the rule failed, the evidence quoted from its draft, the edit before
and after, the verifier's verdict and note, and the checks of each head it creates. Not a re-check:
no finding is classified as old or new text.

The cross-check's readings that the session settles first (cross-check 3.1): **reading B** (only
packed-1 08 depends on it) and **reading 3** (agent-skills-1 21 and 01's created head,
agent-skills-2 13 and 30, and agent-skills-2 07 and 28 depend on it).

### 3.1 Applied in the branch

#### Component knowledge-architect, docs/design.md

**packed-1 21, `design@knowledge-architect@a-build-is-tied-to-its-checkout`** (docs/design.md:323)

- Rule failed: `primer@design-heads`, "How a head is shaped", "A title states no more than its
  argument argues."
- Evidence: "The title: "Every build is tied to the checkout that builds it"." The body: "**The
  value follows cargo's working directory, not the manifest it builds.** ... Started in one checkout
  with `--manifest-path` naming another, it builds the other with the first one's value; started
  outside every checkout, it sets none. So the tie holds for a cargo started inside the checkout it
  builds, which is how the aliases run." History: "the limit entered in 96776c0 ... after the title
  was written in c951527. No approval stands behind the title's wording".
- Edit, before:

      ### Every build is tied to the checkout that builds it `##a-build-is-tied-to-its-checkout`

  after:

      ### Every build that cargo starts inside a checkout is tied to that checkout `##a-build-is-tied-to-its-checkout`

- Verifier: verified. "The proposed title stays false of the rival, a separate CARGO_TARGET_DIR per
  checkout. It leaves tripwire checker-source-literals-are-data-self-location true. No word of the
  owner is touched (c951527, 96776c0)".
- Creates no head.

#### Component core, crates/core/docs/design.md

**The shared phrase.** core-1 18 and 23 and core-2 19 name a goal for the same words, "the failure
this tool exists to prevent", at crates/core/docs/design.md lines 410-413, 507-509 and 1386-1387.
core-1 names `goal@core@declared-instructions-are-checked`; core-2 names
`goal@core@documents-stay-current`. The cross-check: "The session picks one goal for the phrase and
uses it at all three sites. Default: `goal@core@declared-instructions-are-checked`, since each site
argues against a silent pass, and a pass that hides an unchecked reference is a tree that passes
without conforming."

**core-1 18, `design@core@a-failed-parse-is-loud`** (line 408)

- Rule failed: "What a head holds": the standing argument names the goal it derives a constraint
  from.
- Evidence: "Silence would remove every citation in it from the walk while the run reported
  success, which is indistinguishable from a clean file and is the failure this tool exists to
  prevent." The goal: "A tree that passes the check conforms to all of it, and no project can hold
  part of it back."
- Edit, before:
  > Silence would remove every citation in it from the walk while the run reported success, which is indistinguishable from a clean file and is the failure this tool exists to prevent.

  after:
  > Silence would remove every citation in it from the walk while the run reported success, which is indistinguishable from a clean file, against `goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of it.
- Verifier: verified. "Quotation matches design.md:410-413 ... cargo klarch show lists no tripwire or
  issue."

**core-1 23, `design@core@failed-git-is-exit-two`** (line 505)

- Rule failed: as 18.
- Evidence: "a project reported as holding no document is a run that checked nothing and said so as
  a clean verdict, which is the failure this tool exists to prevent."
- Edit, before:
  > a project reported as holding no document is a run that checked nothing and said so as a clean verdict, which is the failure this tool exists to prevent.

  after:
  > a project reported as holding no document is a run that checked nothing and said so as a clean verdict, against `goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of it.
- Verifier: verified. "Quotation matches design.md:507-509; same unnamed goal as 18; the edit adds
  the reference only. The tripwire at tripwires.md:15 guards the head but the finding carries no
  firing evidence."

**core-1 31, `design@core@the-regime-has-no-opt-out`** (line 606)

- Rule failed: as 18.
- Evidence: "The rules over documents are compiled in and the manifest declares nothing about them,
  so conformance of a document means the same thing in every tree this tool checks. There is no
  flag, no list and no severity: a rule either holds over a document or the run fails."
- Edit, before:
  > There is no flag, no list and no severity: a rule either holds over a document or the run fails.

  after:
  > There is no flag, no list and no severity: a rule either holds over a document or the run fails. The constraint derives from `goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of it, and no project can hold part of it back.
- Verifier: verified. "Quotation matches design.md:608-610 ... no goal reference in the head."

**core-1 32, `design@core@components-carry-the-same-documents`** (line 631), refuted conforming

- Rule failed: as 18.
- Evidence (verifier): "A project free to declare its own set would be conformant with whatever it
  declared, which is the same as being checked against nothing" (design.md:647-648) "is the
  constraint of goal@core@declared-instructions-are-checked, unnamed."
- Edit: no draft proposes one. Cross-check: "No draft proposes the edit; the session writes it, on
  the pattern of draft 31."
- Verifier: refuted (the draft's "conforms").

**core-1 51, `design@core@a-wrong-declaration-is-a-finding`** (line 896), refuted conforming

- Rule failed: as 18.
- Evidence (verifier): "A manifest that will not load reports nothing at all, and nothing at all is
  what a session reads as conformance" (design.md:899-900) "argues from
  goal@core@declared-instructions-are-checked without naming it."
- Edit: no draft proposes one; the session writes it, on the pattern of draft 31.
- Verifier: refuted (the draft's "conforms").

**core-1 33, `design@core@anchors-are-components-and-locations`** (line 656)

- Rule failed: as 18.
- Evidence: "**A location exists so that a directory outside every component can be cited.** Naming
  the directory gives every entry in it a reference and makes relocating it one manifest edit, which
  is what `design@core@every-path-names-its-anchor` buys everywhere else." The goal
  `goal@core@relocation-is-one-manifest-edit` "states "moving a Component or a location changes the
  manifest and no document"".
- Edit, before:
  > Naming the directory gives every entry in it a reference and makes relocating it one manifest edit, which is what `design@core@every-path-names-its-anchor` buys everywhere else.

  after:
  > Naming the directory gives every entry in it a reference and makes relocating it one manifest edit, as `goal@core@relocation-is-one-manifest-edit` asks and as `design@core@every-path-names-its-anchor` buys everywhere else.
- Verifier: verified. "Quotation matches design.md:666-668".
- Cross-check: core-1 33 and core-2 23 "both add `goal@core@relocation-is-one-manifest-edit` for the
  same property ... Apply them in one commit with one wording."

**core-1 48, `design@core@slice-spec-sections`** (line 853), a rename

- Rule failed: "How a head is shaped": "The slug and the title stay aligned with the full scope of
  the decision".
- Evidence: "The title states the rule for every plan document, and the slug names one of its
  members: "### A plan document owes the sections of its kind, and a slice spec owes its own
  (slug slice-spec-sections)". The body opens with the rule for a spec of specs/ and a milestone's
  README: "A spec of specs/ and a milestone's README owe the plan sections, in order"." "The slug was
  renamed from step-spec-sections by the commit "The core calls a milestone's part a slice spec, in
  its findings, its code, its README and its heads" (219dcb3), which followed the word and kept the
  narrowing."
- Edit, before:
  > (level-three heading) A plan document owes the sections of its kind, and a slice spec owes its own (slug slice-spec-sections)

  after:
  > (level-three heading) A plan document owes the sections of its kind, and a slice spec owes its own (slug plan-document-sections-by-kind)

  "And at crates/core/README.md:420, `design@core@slice-spec-sections` becomes
  design@core@plan-document-sections-by-kind."
- Verifier: verified. "one citing site, crates/core/README.md:420; new slug unused (git grep)."
- Check (check/core-1/check-1.md), created head `plan-document-sections-by-kind`: **conforms**, "with
  one condition on the commit that applies it": "the commit that renames it to state the pairs ...
  `#step-spec-sections → plan-document-sections-by-kind`, and the previous slug
  `slice-spec-sections`. The draft's proposed edit does not say so."
- Cross-check: "Read as a rewording of the slug: no word of the owner, and the title and body are
  unchanged. If the session reads a rename as named by no row, it goes on the owner list by the
  general rule."

**core-2 02, `design@core@generated-files-are-pure`** (line 1063), a split

- Rule failed: "How a head is shaped": one decision per head.
- Evidence: "Title: "A generated file's content is a function of the walked tree, so `index` writes
  it with no dry run"." "Statement 1, purity. Its rival is a generator that embeds bytes the walk
  does not see" and "Statement 2, the command line of `index`. Its rival is a dry-run flag: "**Writing
  one destroys nothing**, so `cargo klarch index` takes no dry-run flag."" "Statement 2 is no
  exception, parameter or delivery of statement 1: purity means the same with or without a dry-run
  flag."
- Edit, before (the title):

  ```markdown
  ### A generated file's content is a function of the walked tree, so `index` writes it with no dry run `##generated-files-are-pure`
  ```

  after, `generated-files-are-pure` keeps its first and last paragraphs:

  ```markdown
  ### A generated file's content is a function of the walked tree `##generated-files-are-pure`

  No generated file holds hand-written bytes: not a file-register `index.md`, and not a file an
  extension generates. Each is what its generator returns over the model, which is what lets
  `path@core@src/check/generated.rs` verify one by regenerating into a `String` and
  comparing rather than by writing the file and reading it back.

  **The purity is a property to preserve, not one to observe.** A generator that embedded a
  timestamp, a hostname, or anything the walk does not see would break both consequences at once: the
  check would report a file stale that nobody had changed, and `index` would rewrite on every run.
  ```

  the created head, right after it:

  ```markdown
  ### `index` takes no dry-run flag, since writing a generated file destroys nothing `##index-takes-no-dry-run`

  **Writing a generated file destroys nothing**, since its content is a function of the walked tree,
  per `design@core@generated-files-are-pure`, so `cargo klarch index` takes no dry-run flag. Its one
  flag, `--staged`, chooses the tree and the destination, per `design@core@index-staged-write`. The
  dry run a write command normally owes exists because a write can lose something, and here it
  cannot: the worst a run can do is replace a file with what the tree says that file is. It writes
  only where the bytes differ, so running it to look moves not even an mtime, and it names each file
  it rewrote.

  **That last claim is held by two refusals, not by the paths being generated.** `fs::write` follows
  a symlink and writes through it, so a generated path that is one would replace content this
  command never produced; and a destination directory the manifest declares but the tree does not
  have is a manifest defect rather than something to create. Both are checked over **every**
  destination before any is written, so a run either refuses having written nothing or writes them
  all. Remove either check and the paragraph above stops being true.
  ```

  Re-points: "crates/core/src/cli/mod.rs:691 ("It writes only where the bytes differ") and :827 (the
  symlink refusal); crates/core/README.md:239 ... moves to the created head. crates/core/README.md:193,
  design.md:967 and :1050, and rejected-alternatives.md:296 cite purity and stay."
- Verifier: verified. "The two lose to different rivals ... Quotations match design.md:1063-1085. No
  word of the owner is in either part. One correction: `git log -G'takes no dry-run flag'` returns
  f74044d, not 086c60c."
- Check (check/core-2/check-1.md): `generated-files-are-pure`, retitled: **conforms**.
  `index-takes-no-dry-run`: **fails one rule, narrowly**, one decision per head, for the
  missing-directory clause: "The missing-directory refusal loses to another rival, creating the
  directory. Its reason, "a manifest defect rather than something to create", shares no premise with
  "a write that cannot lose content needs no dry run"". Proposed repair: "keep the symlink refusal in
  the created head. Move the missing-directory clause, with its reason, to
  `design@core@one-list-of-generated-files` ... An alternative is to leave the clause in the created
  head and accept the "delivery" reading. Either way the split itself stands."
- Cross-check default: "keep the clause, read as the delivery of the all-or-nothing write, which the
  check allows. The other way, moving it into `one-list-of-generated-files`, edits a conforming head".

**core-2 19, `design@core@an-unresolved-reference-is-reported`** (line 1374)

- Rule failed: "What a head holds": the standing argument names the goal it derives a constraint
  from.
- Evidence: "It would also mean every pointer written in an older form stops being checked with
  nothing saying so, and a silent false negative is the failure this tool exists to prevent." "The
  goal it states is `goal@core@documents-stay-current`".
- Edit, before:

  ```markdown
  nothing saying so, and a silent false negative is the failure this tool exists to prevent. The
  ```

  after:

  ```markdown
  nothing saying so, and a silent false negative is the failure `goal@core@documents-stay-current`
  exists to prevent: a reference that resolves to nothing would pass unreported. The
  ```
- Verifier: verified. "The proposed wording makes a goal "exist to prevent" a failure; at
  application, a wording that serves the goal reads better." The choice of goal follows the shared
  phrase above.

**core-2 23, `design@core@every-path-names-its-anchor`** (line 1520), finding 1 (finding 2 is known,
3.3)

- Rule failed: "What a head holds": the standing argument names the goal.
- Evidence: "What this buys is the same property `design@core@an-entity-belongs-to-its-anchor` buys
  for every entity: relocating an anchor edits its one line of the project's
  `knowledge-architect.toml` and no document, and one fixed-string grep per anchor is an anchor's
  complete inbound-reference list." "Those are the words of `goal@core@relocation-is-one-manifest-edit`".
- Edit, before:

  ```markdown
  same property `design@core@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
  anchor edits its one line of the project's `knowledge-architect.toml` and no document, and one fixed-string
  ```

  after:

  ```markdown
  same property `design@core@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
  anchor edits its one line of the project's `knowledge-architect.toml` and no document, which
  `goal@core@relocation-is-one-manifest-edit` asks, and one fixed-string
  ```
- Verifier: verified, "partly known". "the head it cites, `design@core@an-entity-belongs-to-its-anchor`,
  names no goal either: a grep of its body for goal@ returns nothing."
- Cross-check, on touching a head whose bundle an issue records (with agent-skills-2 42): "Both
  issues name the next run of this axis as a re-entry point. ... The session decides: apply the edits
  and leave the bundles to the issues, as the first run did, or hold both edits until the splits.
  Default: apply, and say in the pass's commit message that the bundles stay with their issues."

#### Component agent-skills, crates/agent-skills/docs/design.md

**agent-skills-1 01, `design@agent-skills@content-mirrors-the-install-layout`** (line 15), a split

- Rule failed: "How a head is shaped", "A head holds one decision".
- Evidence: the second decision in the head:
  > The harness reads saved workflows directly under .claude/workflows/, no deeper, and calls each by
  > the name its `meta` declares, never by its file's name, as a probe of Claude Code measured, which
  > the message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
  > installer's namespace" records: so the build fails unless that name equals the installed file's
  > stem, or the prefix of the file would namespace nothing.

  "The layout and the generated list lose to a list written by hand ... The name assertion loses to
  a saved workflow whose `meta` name is free. Its argument is the probe of the harness, recorded in
  the message of e84e8ed".
- Edit, in the source head, before:
  > A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, a saved workflow
  > of Claude Code's Workflow tool is `workflows/<workflow>.js`, and the primer is `PRIMER.md`. The
  > harness reads saved workflows directly under .claude/workflows/, no deeper, and calls each by the
  > name its `meta` declares, never by its file's name, as a probe of Claude Code measured, which the
  > message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
  > installer's namespace" records: so the
  > build fails unless that name equals the installed file's stem, or the prefix of the file would
  > namespace nothing. The build script,

  after:
  > A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, a saved workflow
  > of Claude Code's Workflow tool is `workflows/<workflow>.js`, and the primer is `PRIMER.md`; the
  > build asserts a saved workflow's name, per
  > design@agent-skills@saved-workflow-name-is-its-file-stem. The build script,

  created head:

  ```markdown
  ### A saved workflow's `meta` name equals its installed file's stem, which the build asserts `##saved-workflow-name-is-its-file-stem`

  The harness reads saved workflows directly under .claude/workflows/, no deeper, and calls each by
  the name its `meta` declares, never by its file's name, as a probe of Claude Code measured, which
  the message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
  installer's namespace" records. So the build fails unless that name equals the installed file's
  stem, or the prefix the build adds to the file would namespace nothing, per
  `design@core@owned-namespace-check`.
  ```
- Verifier: verified. "different rivals, no shared premise, so two decisions. No word of the owner
  in the head (e84e8ed's owner words, on the prefix, are not in it)."
- Check (check/agent-skills-1/check-1.md): `saved-workflow-name-is-its-file-stem` **conforms,
  "except under the calibration's proposed reading 3"**: "The head rests on a probe and names only the
  commit that records it. It does not state the re-take". Amendment under reading 3: "after "as a
  probe of Claude Code measured", add "(a file whose stem differs from its declared `meta` name,
  called by each name after the harness reloads; and one a directory deeper)"."
- Cross-check: a citing site the draft missed: "crates/agent-skills/docs/design.md:1579, in
  `design@agent-skills@audit-axis-runs-a-workflow`: "the harness runs it by the name its `meta`
  declares, per `design@agent-skills@content-mirrors-the-install-layout`" ... is re-pointed to
  design@agent-skills@saved-workflow-name-is-its-file-stem. The spec's harvest rewrites this head
  too (row 183)."

**agent-skills-1 21, `design@agent-skills@capabilities-not-structure`** (line 441), sample, refuted
conforming, under reading 3

- Rule failed (under reading 3): "What a head holds": "the measurement it rests on, with the command
  that takes it again".
- Evidence: "In one revision of designing-together, 34 edits produced 10 interaction defects, nearly
  all between rules that scripted one exchange." The drafter's own note: "the rule asks for "the
  measurement it rests on, with the command that takes it again"; the head gives the source at the
  section's level and no command, which a strict reading of that sentence could count as a failure."
  The drafter read the figure with `git show next:docs/decisions.md` in designing-together: "The
  revision-interaction axis found ten interaction defects across thirty-four edits, nearly all of
  them between conformance-shaped rules", under "Decided 2026-08-03".
- Edit: the draft of record proposes none. Three other groups' sample drafts propose a pointer to the
  intake commit; the calibration rejects that form ("the edit should add the command over that
  record"). The cross-check: "Add the command once at the section opening (design.md:342-343) to
  cover the section". The verifier's form: "adding the re-take command the draft itself ran,
  `git show next:docs/decisions.md` in designing-together". The three rejected forms, for reference:
  - agent-skills-2 57, after: "In one revision of designing-together, 34 edits produced 10
    interaction defects, nearly all between rules that scripted one exchange, by that skill's own
    record, which the plan document of the workflow's extraction carried; `git log -G'34 edits'`
    finds the commit that brought it in."
  - core-2 61, after: "... nearly all between rules that scripted one exchange, as that skill's own
    decision record counts them; it is outside this repository, and its intake is the commit "The
    designing-together intake: what its record keeps, and the retrospective carries the expectation
    set". So a gap is weighed by its kind."
  - core-3 61, after: "... nearly all between rules that scripted one exchange: its record's item "A
    gap in the skill's rules is weighed by which kind it is", listed in the message of the commit
    "The designing-together intake: what its record keeps, and the retrospective carries the
    expectation set". So a gap is weighed by its kind."
- Verifier: refuted (quoted in part 1).
- Cross-check: "Under the other reading: nothing." agent-skills-1 22 (expectation-set-bounds-scope)
  rests on the same section opening; one edit there covers both if reading 3 covers incidents.

**agent-skills-1 33, `design@agent-skills@premortem-tripwires-on-the-owners-word`** (line 718), a
split

- Rule failed: "How a head is shaped", "A head holds one decision".
- Evidence, the head whole:
  > A premortem's surviving causes become tripwires only where the owner rules that they should. Each
  > is written at the harvest of the decision it guards, in the tripwires home of the Component that
  > owns that decision, since a tripwire names the decision or the instruction it guards and a head
  > exists from the landing of its work, per `design@agent-skills@design-home-is-built-intent`. Whether a
  > risk is worth watching is a weighing, and the weighing is the owner's, per
  > `goal@knowledge-architect@the-owner-decides`.

  "1. Who decides that a cause becomes a tripwire: the owner. Rival: the agent keeps the causes it
  judges worth watching." "2. When and where a ruled tripwire is written ... Rival: written when the
  owner rules, at the premortem, before the decision's head exists."
- Edit, the source head, after:
  > A premortem's surviving causes become tripwires only where the owner rules that they should.
  > Whether a risk is worth watching is a weighing, and the weighing is the owner's, per
  > `goal@knowledge-architect@the-owner-decides`. Where a ruled tripwire is written is
  > design@agent-skills@ruled-tripwire-written-with-its-decision.

  the created head as drafted:

  ```markdown
  ### A tripwire the owner ruled on is written with the decision it guards, in the tripwires home of the Component that owns it `##ruled-tripwire-written-with-its-decision`

  A tripwire the owner ruled on is written at the harvest of the decision it guards, in the tripwires
  home of the Component that owns that decision, since a tripwire names the decision or the
  instruction it guards and a head exists from the landing of its work, per
  `design@agent-skills@design-home-is-built-intent`.
  ```
- Verifier: verified. "the checker should weigh that its title says "A tripwire the owner ruled on",
  wider than the source's premortem causes, though the argument covers any tripwire."
- Check: the created head **fails** two rules of "How a head is shaped" (the title "states no more
  than its argument argues"; slug and title aligned with the full scope): "The created title and slug
  widen the subject to every ruled tripwire ... The body keeps "is written at the harvest of the
  decision it guards". A harvest is a moment of the full path only." Proposed correction, keeping the
  source's scope:

  ```markdown
  ### A premortem's cause the owner ruled on is written at the harvest of the decision it guards, in the tripwires home of the Component that owns it `##premortem-tripwire-written-at-harvest`

  A premortem's cause the owner ruled a tripwire is written at the harvest of the decision it guards,
  in the tripwires home of the Component that owns that decision, since a tripwire names the decision
  or the instruction it guards and a head exists from the landing of its work, per
  `design@agent-skills@design-home-is-built-intent`.
  ```

  "The source's last sentence then points to design@agent-skills@premortem-tripwire-written-at-harvest."
- Cross-check: "Apply the check's corrected head ... Widening it to every ruled tripwire would be a
  new member: that goes to the design skill, not to this pass. Re-point decision-recording/SKILL.md:242
  (1.3)": crates/agent-skills/content/skills/decision-recording/SKILL.md:242, which the draft does
  not list:
  `%% Written with its decision: \design@agent-skills@premortem-tripwires-on-the-owners-word\.`

**agent-skills-1 39, `design@agent-skills@a-reference-claims-a-revisit`** (line 850)

- Rule failed: "What a head holds", "Name what the argument depends on".
- Evidence: "An entry that is deleted or renamed dangles every reference to it, and
  `cargo klarch check` reports each one, so the repair list a change produces is the list of texts that depended
  on what changed." "The second premise is a decision of the core:
  `design@core@an-unresolved-reference-is-reported`, whose second paragraph reads "**A reference that
  resolves to nothing is recorded and reported, never dropped.**""
- Edit, before:
  > deleted or renamed dangles every reference to it, and `cargo klarch check` reports each one, so the
  > repair list a change produces is the list of texts that depended on what changed.

  after:
  > deleted or renamed dangles every reference to it, and `cargo klarch check` reports each one, per
  > `design@core@an-unresolved-reference-is-reported`, so the repair list a change produces is the
  > list of texts that depended on what changed.
- Verifier: verified. "The agent-skills design home already cites core decisions (36 lines), so the
  reference resolves here."

**agent-skills-1 49, `design@agent-skills@in-change-path`** (line 1060)

- Rules failed: "What a head holds", "A head stands on its argument, and cites the owner only for
  what came from the owner": an approval is cited; "How a head is shaped", "A title states the rule
  that decided, not the list of what it admits today", and slug and title aligned.
- Evidence: "the owner accepted that cost on the two readings put side by side." The rule: "An
  approval is not a ground, whatever its words ("approved", "agreed", "all defaults approved",
  "accepted the cost")." History, F116 of pass 6 of the agentic-workflow axis (5e4809a): "Outcome:
  the default, on the owner's word "Go with the defaults."". The title "A discussion whose decision
  lands in a change begun before it keeps its deliberation in that change's commit message"; the
  body admits two cases: "its work, where that change began before the discussion opened, ... or its
  record, whenever the discussion opened, for a decision recorded when it is made".
- Edit 1, before:
  > builds in one pull request, which strains the "fewer review rounds" of
  > `goal@knowledge-architect@agents-get-a-complete-workflow`; the owner accepted that cost on the
  > two readings put side by side. Widening the existing short path,

  after:
  > builds in one pull request, which strains the "fewer review rounds" of
  > `goal@knowledge-architect@agents-get-a-complete-workflow`. Widening the existing short path,

  Edit 2, before:
  > (level-three heading) A discussion whose decision lands in a change begun before it keeps its deliberation in that change's commit message (slug in-change-path)

  after:
  > (level-three heading) A discussion whose decision lands in the change under way, begun before the discussion or recording a decision no work implements, keeps its deliberation in that change's commit message (slug in-change-path)
- Verifier: verified. "Remark for the session: the proposed title lists both cases; the property the
  body states, "the decision lands in the change under way", may be the shorter title the list rule
  asks for."
- Cross-check: "If the session reads F116 as approving the title's words, it goes to the owner list,
  as a widening."

**agent-skills-2 07, `design@agent-skills@transcript-reviewer-agent`** (line 1252), refuted
conforming; conditional

- Rule failed (under reading 3, if it covers incidents): the measurement's re-take.
- Evidence: the draft: "The evidence "a reviewer briefed with it was the one of six" names the event
  it was measured in (the review of the change that installed the first skills)." The verifier: "names
  its event but no command or record that takes it again".
- Edit: none drafted. Cross-check: "applied in the branch if reading 3 covers incidents; else
  nothing ... The session writes one that names where that review's result is recorded."

**agent-skills-2 13, `design@agent-skills@plan-read-against-the-record`** (line 1360)

- Rule failed: "What a head holds": the measurement carries no way to take it again or find it.
- Evidence: "Two acceptance criteria that `goal@knowledge-architect@the-workflow-improves-through-real-use`
  rules out each passed a plan document's three reviews in this repository." "the instances are in
  the deleted spec synthetic-evidence-and-plan-reviews, at
  `git show d2c830d^:docs/plans/specs/synthetic-evidence-and-plan-reviews.md`, lines 62-66".
- Edit, before: "Two acceptance criteria that
  `goal@knowledge-architect@the-workflow-improves-through-real-use` rules out each passed a plan
  document's three reviews in this repository." after: "Two acceptance criteria that
  `goal@knowledge-architect@the-workflow-improves-through-real-use` rules out each passed a plan
  document's three reviews in this repository; the plan document of that work holds both, and
  `git log --diff-filter=D --name-only -- docs/plans/` finds it."
- Verifier: verified. "Note: the edit's "the plan document of that work" is vague; name the work or
  give git log -G'three reviews in this repository'". Cross-check: "Tighten the edit as the verifier
  asks".

**agent-skills-2 28, `design@agent-skills@workflow-axis-finding-standard`** (line 1817), refuted
conforming; conditional

- Evidence: the draft: "the figure "6 of the 14" states how it is retaken ("by a reading of each
  against the corpus, the design homes and the goals at the commit that pass audited")". The
  verifier: "names a procedure but no record or command".
- Edit: none drafted. Cross-check: "nothing, unless the session reads 3.1 (b) strictly; then applied
  in the branch ... Only the record is missing, the fifth pass's report file, found with
  `git log --name-status -- docs/audit-reports/`." "It is the weakest of the refutations."

**agent-skills-2 30, `design@agent-skills@verifier-in-the-run`** (line 1874)

- Rule failed: "What a head holds": the measurement that defeats the nearest rival carries no way to
  find it or take it again.
- Evidence: "The nearest rival, one verifier per draft, reads the whole corpus once per draft: at the
  pass that measured it, 63 drafts against 16 auditors." History:
  "`git show cbc05ae^:docs/plans/specs/agentic-workflow-axis.md`, line 1129-1130: "Measured on pass 4, the files
  in its scratch directory, `wc -l` over each set: the sort read 63 drafts"."
- Edit, before: "at the pass that measured it, 63 drafts against 16 auditors." after: "at the fourth
  pass of the agentic-workflow axis on this repository, 63 drafts against 16 auditors. The plan
  document of that work holds the measurement, and
  `git log --diff-filter=D --name-only -- docs/plans/` finds it."
- Verifier: verified. "confirmed at cbc05ae^:docs/plans/specs/agentic-workflow-axis.md lines
  1128-1129 and 1179-1180; the calibration's reading fails it".
- Cross-check: "Apply it with the spec's harvest, or after it, so that row 189 does not drop it."

**agent-skills-2 42, `design@agent-skills@primer-content`** (line 2094), the tense (the bundle is
known, 3.3)

- Rule failed: "What a head holds", present tense, in one sentence.
- Evidence: "Every session commits, and the rule was delivered only by the skills that met one case
  of it: planning for a plan document merged before its work, and review for a fold." The primer: "A
  head is written as if the design had always been so. ... and so does an opening that motivates a
  decision by describing the state before it."
- Edit, before: "Every session commits, and the rule was delivered only by the skills that met one
  case of it: planning for a plan document merged before its work, and review for a fold." after:
  "Every session commits, and a skill meets only one case of it: planning a plan document merged
  before its work, and review a fold."
- Verifier: verified, known for the bundle. "the tense finding is confirmed: ... (line 2155)
  describes the state before".

**agent-skills-3 10, `design@agent-skills@retro-trigger`** (line 2418), a split

- Rule failed: "How a head is shaped", one decision per head.
- Evidence, the head whole:
  > (level-three heading) A retrospective is offered once per session, at a defined moment (slug retro-trigger)
  >
  > The retrospective is offered once per session, at the first of three moments: a pull request the
  > session worked on has merged, a plan document has left, or the owner says the session is ending.
  > It runs only if the owner accepts. A moment named by an event can be followed by any agent where
  > "a moment it judges right" could not, and offering it once keeps it from interrupting work.

  | statement | nearest rival | its argument |
  | --- | --- | --- |
  | offered at a moment an event names | "a moment it judges right" | "can be followed by any agent where 'a moment it judges right' could not" |
  | offered once per session | offered at every such moment | "offering it once keeps it from interrupting work" |
- Edit, after (two heads in place of one):

  ```markdown
  ### A retrospective is offered at a moment an event names, never at one the agent judges `##retro-trigger`

  The retrospective is offered at the first of three moments: a pull request the session worked on
  has merged, a plan document has left, or the owner says the session is ending. It runs only if the
  owner accepts. A moment named by an event can be followed by any agent where "a moment it judges
  right" could not.

  ### A retrospective is offered at most once per session `##retro-offered-once`

  The retrospective is offered once per session, at the first of the moments
  `design@agent-skills@retro-trigger` names, and not again at a later one. Offering it once keeps it
  from interrupting work.
  ```
- Verifier: verified. "the split's commit owes the pair #retro-trigger -> retro-offered-once".
- Check (check/agent-skills-3/check-1.md): created head 1 **fails** one decision per head: ""at the
  first of" is the once-per-session statement". Repair, before: "The retrospective is offered at the
  first of three moments: a pull request the session worked on has merged, a plan document has left,
  or the owner says the session is ending." after: "The retrospective is offered when one of three
  events occurs: a pull request the session worked on has merged, a plan document has left, or the
  owner says the session is ending." Created head 2, `retro-offered-once`: **conforms**.
- Cross-check: "applied in the branch, with the check's repair ... The commit states the pair
  `#retro-trigger → retro-offered-once`."

**agent-skills-3 11, `design@agent-skills@premortem-as-watch-points`** (line 2425)

- Rule failed: "What a head holds", the standing argument: it argues from the words of
  `goal@knowledge-architect@the-workflow-improves-through-real-use` without naming it.
- Evidence: "Each watches a decision whose failure would be seen in real sessions before any check
  could see it: ..." The goal: "Real use is the test of the workflow ... The retrospective, and the
  owner's observations of real sessions, collect what was unclear, missing or wrong, and the owner
  decides what changes." The closed issue at 61a9219^: "`design@agent-skills@premortem-as-watch-points`
  exists so that a failure of a decision is seen in real sessions before any check could see it, per
  `goal@knowledge-architect@the-workflow-improves-through-real-use`."
- Edit, before:

  ```
  skill's addition was missed, and whether the session needed to write a reference or a path without
  backticks because no checked form expresses it. Each watches a decision whose
  failure would be seen in real sessions before any check could see it:
  ```

  after:

  ```
  skill's addition was missed, and whether the session needed to write a reference or a path without
  backticks because no checked form expresses it. Real sessions are the test of the workflow, per
  `goal@knowledge-architect@the-workflow-improves-through-real-use`, and each question watches a
  decision whose failure would be seen in real sessions before any check could see it:
  ```
- Verifier: verified. "the owner's words are not touched".

### 3.2 Put to the owner, on the owner list

Part 5 is the draft of the list; this section holds the evidence and the edits it cites.

#### Component knowledge-architect, docs/design.md

**packed-1 08, `design@knowledge-architect@versioning-policy`** (docs/design.md:110), sample, refuted
conforming; two findings

- Finding B (reading B): the scope paragraph came from the owner (72d9517) and does not say so.
  Evidence and history: part 2.1. Default edit, core-3 59, before:

  ```text
  alone, such as its changelog: each is at most a patch.
  ```

  after:

  ```text
  alone, such as its changelog: each is at most a patch. The scope is the owner's: "the versionning
  policy is not supposed to be about checks that are purely internal to knowledge-architect. It is
  about checks added in the published crate, and which can affect external consumers."
  ```

  core-3 59: "The clause on a file shipped for reading alone was one of the two points then put to
  the owner and agreed. That is an approval, so it needs no credit: it is a member of the owner's
  rule, since such a file cannot affect an external consumer."
- Finding A: "two owner credits with no quotation". The rule reads two ways (part 2.1). Cross-check
  default: "reading 2, conforms." The two failing drafts' edits, for the owner:
  - core-1 59, after: "The table is the owner's scheme. Its minor and major rows hold the owner's
    ruling on a stricter check that requires a content change, chosen between that reading and its
    opposite: "minor wins". The bullets below refine the scheme, and the first is the argument the
    owner gave for that ruling." core-1 59 also asks "whether the table, reworded since the v0.1
    record ("CLI" to "command", "orthogonal additions" to "additions that change nothing existing"),
    still states the scheme as the owner gave it."
  - agent-skills-2 55, after, "if the owner supplies the words": "The table is the owner's scheme:
    "<the owner's words>". The bullets below refine it, and the first is the owner's argument, given
    in the ruling "minor wins" that a stricter check is minor."; "if the owner does not": "The bullets
    below refine the table, and the first is the owner's argument, given in the ruling "minor wins"
    that a stricter check is minor."
- Verifier: refuted (part 1). Cross-check: "Put both to the owner in adjacent items, since both turn
  on how the head credits the owner, and apply whatever is ruled in one edit of the head."

**packed-1 09, `design@knowledge-architect@changelog-entries`** (docs/design.md:156), a split

- Rule failed: "How a head is shaped", "A head holds one decision".
- Evidence: the title, "The changelog records what a consumer must plan for: one entry per change
  that passes a test, under released sections whose content never changes". "1. Which change gets an
  entry ... Nearest rival: an entry per commit, or a changelog rebuilt from the commit log." "2. How
  the file's sections change over time ... Nearest rival: a released section corrected or
  reclassified after the release." "History: F58 of 8d4179f,
  "`design@knowledge-architect@changelog-entries` is retitled to state its released-section rule", on
  the owner's "Agreed on the defaults for the others"."
- Edit, before (title):

      ### The changelog records what a consumer must plan for: one entry per change that passes a test, under released sections whose content never changes `##changelog-entries`

  after, the parent:

      ### The changelog records what a consumer must plan for: one entry per change that passes a test `##changelog-entries`

  new head as drafted:

      ### A released section of the changelog keeps its content, and the working section changes until the release `##released-changelog-sections-keep-their-content`

      CHANGELOG.md has one section per version, and one working section above them, `Next release`,
      which the release renames to its version. The working section may be reworded, restructured or
      pruned at any time, and a change reversed before the release leaves it, since the section describes
      the release's net effect. A released section's content never changes; its structure may.

      **A skill's or an agent's name in a released section is structure.** Writing a bare name there as
      its reference, `skill@<name>` or `agent@<name>`, changes no statement of the section. The owner:
      "older changelog section allow structural changes, and this passes as a structural change in my
      view". So does writing it back as the bare name once the skill or the agent is deleted, since the
      reference then names nothing and the section's content cannot change. Why the bare name is no
      evasion is `design@agent-skills@plain-text-is-no-repair`, and the lint that reports a bare name
      `design@core@bare-skill-name-reported`.

  Re-points listed by the draft: CLAUDE.md:197 and :221, crates/core/docs/design.md:1638,
  crates/agent-skills/docs/design.md:945, .claude/agents/klarch-changelog-reviewer.md:43 ("whose line
  22 then names three heads where it says "two heads"").
- Verifier: verified, **bears-on-ruling**. "F58 of 8d4179f retitled the head to state both, and the
  rule says "Rewording the title to state both does not make them one". ... One citing site the draft
  does not list, CLAUDE.md:507, cites the entry tests and keeps the parent".
- Check (check/packed-1/check-1.md):
  - 09-A, the parent: **fails** the standing argument, goal not named: "The reader premise is the
    words of `goal@knowledge-architect@any-project-can-adopt-it` ... Repair: end the second paragraph
    with "per `goal@knowledge-architect@any-project-can-adopt-it`"."
  - 09-B: **fails** the standing argument and slug alignment: "No premise argues "A released section's
    content never changes"." "the rule entered with 43fda63 ... and gives no reason for it." Repair:
    "`changelog-section-fixed-at-release`, with the title "A changelog section's content is fixed at
    its release"".
  - The edit: **fails** "split a rule from its own exception": "09-A keeps the sentence "An entry
    describes the release's net effect: a change reversed before the release has none". 09-B keeps
    "The working section may be reworded, restructured or pruned at any time"."
- Cross-check: default "the split with the check's repairs ... 09-B has no recorded argument for "a
  released section's content never changes" (43fda63 gives none). The owner states it, or that part
  gets an issue entry, kind `design`. Re-point the three sites draft 09 missed (1.3, 2.1)":
  docs/plans/specs/path-quickfixes.md:220, docs/open-issues/a-mechanical-changelog-check.md:9 and :43.
- Met outside the task (packed-1): ".claude/agents/klarch-changelog-reviewer.md:22 says "The
  standard is two heads of docs/design.md"; the split drafted in 09 would make it three."

**packed-1 23, `design@knowledge-architect@workflow-findings-stay-here`** (docs/design.md:373)

- Rule failed: "A head holds one decision"; the part split off fails "What earns a head" read alone.
- Evidence, the third paragraph:
  > For the same reason, the root `CLAUDE.md` gives the auditors of the checker-rules lens, L7, the
  > checker's design home, `path@core@docs/design.md`, beside its README. The installed lens names the
  > README alone, since a project that installs the workflow holds no design home of the checker. Here
  > the checker is built in the same tree, and its design home is where its rules are decided.

  "Its nearest rival is the installed lens's reading, the README alone." Test 2: "One text states it:
  CLAUDE.md:479-481 ... (`grep -rn L7` over the Markdown, Rust and workflow files finds no other text
  stating it.)" "test 4: no record that the owner was asked."
- Edit: delete the paragraph; "the head ends with "So the root `CLAUDE.md` states this case, in
  `instructions@repository-skills`."" CLAUDE.md:479-481, before:

      The
      audit's checker-rules auditors, L7, also read the checker's design home, `path@core@docs/design.md`,
      beside its README.

  after:

      The
      audit's checker-rules auditors, L7, also read the checker's design home, `path@core@docs/design.md`,
      beside its README: the installed lens names the README alone, since an adopting project holds no
      design home of the checker, and here the checker's rules are decided in that home.
- Verifier: verified. "The edit removes no word of the owner".
- Cross-check: "The CLAUDE.md edit goes through `skill@knowledge-architect-agent-configuration`."

**packed-1 25, `design@knowledge-architect@analysis-names-nothing-of-another-project`**
(docs/design.md:420)

- Rule failed: "What earns a head": no entry test holds.
- Evidence: "it is a decision about agent-facing text, and one text states it, the project's skill
  `skill@klarch-retrospective-intake`, at two lines". "Test 4: the owner's words entered in 4b1240e
  ... No record shows the owner asked whether they record the owner's intent". "Pass 1 created the
  head by a split the owner approved by default (F22 to F29 of 3c4f97f), an approval of a split, not a
  test-4 answer."
- Edit: delete the head (docs/design.md:420-425):

      ### An analysis of a received file names nothing of another project beyond what that file holds, and no path or design head of it `##analysis-names-nothing-of-another-project`

      The repository is public and the workflow file is the one a retrospective writes to be
      publishable, so the analysis names nothing of another project beyond what its workflow file holds.
      It names no path or design head of that project either, which in the owner's words "is useless
      information here anyway": where a finding needs one, the analysis says it in words.

  In .claude/skills/klarch-retrospective-intake/SKILL.md, before:

      - **Name nothing of the other project in detail**: none of its paths, none of its design heads,
        none of its internals. Say it in words where the finding needs it, as "the project's design
        home".

  after:

      - **Name nothing of the other project in detail**: none of its paths, none of its design heads,
        none of its internals, which tell a reader of this repository nothing. Say it in words where
        the finding needs it, as "the project's design home".

  "If the owner answers that the words record their intent (test 4), the head stays and quotes that
  answer instead."
- Verifier: verified, **bears-on-ruling**. "The head exists because the owner approved the default
  of pass 1's split (F22 to F29, 3c4f97f), and its removal reverses that split. The owner's later
  ruling to keep only no-untested-snippet-is-authority as intent supports the default".

#### Component gates, crates/gates/docs/design.md

**packed-1 35, `design@gates@one-spawn-helper`** (line 47), a split

- Rules failed: "A head holds one decision"; "What a head holds", the standing argument, for the part
  the slug names.
- Evidence: "The title: "One spawn helper captures a child's two streams through two pipes"." "1.
  One helper for every child ... The head states no argument for it." "2. Two pipes read
  concurrently ... Its nearest rival: one regular file for both streams". History, 37d02d1: "the gates
  head did not carry a reason for part 1 into the library, and none is on record for it."
- Edit, the parent as drafted:

      ### Every child the gates or a project's commands start goes through the library's one spawn helper `##one-spawn-helper`

      `path@gates@src/process.rs` spawns a child, captures its two streams into one buffer, optionally
      mirrors them live, and holds the best-effort output helpers every print goes through. The gates use
      it, and a project's own commands may: a new command calls it and never edits it.

      [its argument: none on record; to be stated by the owner, or the part leaves for a comment at
      `path@gates@src/process.rs`]

  new head:

      ### A child's stdout and stderr are read concurrently through two pipes `##streams-through-two-pipes`

      The spawn helper, `design@gates@one-spawn-helper`, reads a child's stdout and stderr concurrently
      through two pipes into one buffer. Reading both pipes concurrently is what prevents the deadlock
      where the child blocks writing a pipe nobody drains. Two pipes have none of the three losses a
      regular file shared by both streams showed: it lost output written through /dev/stdout and output
      of a descendant that outlives the child, and a killed run left the file behind to fail a later run,
      as the rejected alternative on it records.

  Re-point the rejected alternative "Capturing a child's two streams through one regular file instead
  of two pipes" to `streams-through-two-pipes`; keep `design@xtask@one-module-per-subcommand` on the
  parent.
- Verifier: verified. "No argument is stated for the first, and 37d02d1^ shows its reason was about
  xtask's modules."
- Check: 35-A **fails** the standing argument, "A title states no more than its argument argues",
  and "What earns a head, not established". Title repair: "The gates start every child through the
  library's one spawn helper, which a project's commands may use". 35-B **conforms**.
- Cross-check default: "the split, with 35-A's title narrowed as the check proposes ... and the owner
  asked for its reason. With none, an issue entry, kind `design`."

#### Component xtask, tools/xtask/docs/design.md

**packed-1 50, `design@xtask@root-finder-uses-the-core-constant`** (line 55)

- Rule failed: "What earns a head": no entry test holds.
- Evidence: the rule, "A reason is an argument against a rival someone could plausibly propose; a
  name, a path or a value repeated for consistency is not one, since nothing argues for changing it
  and a stale copy is found by a search." "The head's reason against the rival (a literal file name
  in xtask) is exactly that a copy of the name could go stale." "Test 1: tools/xtask is never
  published". "Test 4: no record that the owner was asked."
- Edit: delete the head (tools/xtask/docs/design.md:55-62):

      ### The root finder spells the manifest's name with the core's constant `##root-finder-uses-the-core-constant`

      A subcommand runs from the project root, found by walking up to the first directory holding the
      checker's manifest. xtask hands the gates library's `project_root` that name, and depends on the
      core for it, `MANIFEST_NAME`, so the file name is spelled in one place and a rename of the
      manifest cannot leave xtask looking for the old one. The subcommands that find the root today are
      gates and changelog. The cost is that building xtask builds the core, which the check gate builds
      anyway.

  tools/xtask/src/gates.rs:32-33, before:

      // The root is the nearest ancestor holding the checker's manifest, its name spelled by the
      // core's constant, per `design@xtask@root-finder-uses-the-core-constant`.

  after:

      // The root is the nearest ancestor holding the checker's manifest, its name spelled by the
      // core's constant, so a rename of the manifest cannot leave xtask looking for the old one;
      // the dependency on the core costs nothing, since the check gate builds it anyway.

  "The same comment is added above tools/xtask/src/changelog.rs:56." In
  `design@xtask@one-module-per-subcommand`, before "... its `project_root`; and the core's
  `MANIFEST_NAME`, per `design@xtask@root-finder-uses-the-core-constant`. Adding a workflow ...", after
  "... its `project_root`; and the core's `MANIFEST_NAME`. Adding a workflow ...".
- Verifier: verified. "One flaw in the evidence: "a comment at each holds the reason" is false for
  tools/xtask/src/changelog.rs:56, which has no comment. The edit adds one".

#### Component core, crates/core/docs/design.md and rejected-alternatives.md

**core-1 58, `design@core@milestone-entries-are-directories`** (design.md:1009)

- Rule failed: "What a head holds": a ground that is the owner's words is quoted.
- Evidence: "The shape exists because the owner asked to cite a milestone as `milestone@plans@<id>`,
  and a milestone is a directory: a File entry is one file and could hold no slice specs. Citing the
  milestone by a path to its README needs no new shape, and loses to that request." History, the
  README of milestone structured-plans at f191c49^, a42, "Who: owner": "I'd like to be able to
  reference path@knowledge-architect@docs/plans/structured-plans/ as
  milestone@knowledge-architect@structured-plans"; a81: "You asked for milestone@, so I keep it, and
  name it as the cost."
- Edit, before:
  > The shape exists because the owner asked to cite a milestone as `milestone@plans@<id>`, and a milestone is a directory: a File entry is one file and could hold no slice specs.

  after:
  > The shape exists because the owner asked for it: "I'd like to be able to reference" a milestone's directory, cited by its path, "as" a milestone of its own kind, which the plans layout spells `milestone@plans@<id>`. A milestone is a directory: a File entry is one file and could hold no slice specs.

  "The session settles whether to keep the two spans verbatim without backticks instead, as the plan
  document wrote them."
- Verifier: verified. "a42, Who: owner, Round 2, unprompted, quoted exactly by the draft; a81
  confirms the cost was kept for that request. Outcome to the owner is right."

**core-2 10, `design@core@fix-scope`** (design.md:1194), sample

- Rule failed: the title test. Evidence and reading: part 2.2.
- Edit, before:

  ```markdown
  ### `--fix` admits a repair only when it passes the safe-fix test `##fix-scope`
  ```

  after:

  ```markdown
  ### `--fix` applies only the kinds of repair added to it on purpose, each passing the safe-fix test `##fix-scope`
  ```

  "The body is unchanged; the owner's words stay quoted as the ground."
- Verifier: verified. "The owner's F64 words match 8d4179f verbatim, and they answered the wording of
  this title, so the owner list is the right outcome."

**core-2 49, rejected alternative "Skipping a commit whose tree fails, in `commits`"**
(rejected-alternatives.md:23)

- Rule failed: `skill@knowledge-architect-decision-recording@losing-alternatives`: no recording test.
- Evidence: "It loses because each skipped commit took its message out of the regime while the run
  exited 0, and the summary line was the only trace." "(3) ... the reason is derived from the shape
  itself ... with no measurement, prototype or survey named; (4) no doubt is stated that the winner
  meets every goal".
- Edit: remove the paragraph "from "**Skipping a commit whose tree fails, in `commits`** — lost to"
  through "orders or squashes its commits instead."".
- Verifier: verified. "The first run, 3c4f97f, only repaired it in place. The owner rules on the
  removal."
- Cross-check: "If ruled, repair the four-heads issue's line 54 in the same commit (2.1)."

**core-2 54 and 55, rejected alternatives "Families named for the subjects that read them"
(rejected-alternatives.md:75) and "One family per invocation, instead of a set" (:82)**

- Rule failed: as 49.
- Evidence: 54: "a subject's name inside the parser is exactly that project's knowledge, compiled in"
  ("the reason is the head it lost to, applied"). 55: "the walk happens once per invocation, so a
  caller wanting five families would read every live document five times" ("arithmetic over the head
  it lost to").
- Edit: remove each paragraph, 54 "through "compiled in."", 55 "through "five times."".
- Verifier: verified. 54: "A reader of that head derives it again, so recording test 3 fails". 55: "It
  meets no recording test."
- Cross-check: "No other text names either (grep over the Markdown and Rust files)."

#### Component agent-skills, crates/agent-skills/docs/design.md

**agent-skills-1 04, `design@agent-skills@shipped-text-cites-no-entry`** (line 72)

- Rule failed: "A head holds one decision".
- Evidence (lines 91-95):
  > A literal the checker
  > would misread is a delivery substitution, which the build fills. The owner's argument: "to avoid
  > convoluted sentences (such as for the primer import line), the text could be applied using
  > substitution at build or install time, as documented exceptions where we are avoiding the checker
  > on purpose in order to facilitate delivery of the installed skills."

  "the substitution is its own thread ... "Proposed: the owner, R6 (a66); the shape and its limit are
  the assistant's", with the harvest "none in a design home"". 249f145: "#delivery-substitutions earns
  no head: its reason stands at one site, the comment of the table in `path@agent-skills@build.rs`."
- Edit (default), after:
  > The command a project runs is written as the placeholder that the install fills
  > with the project's declared command, per `design@core@declared-command`.

  Rival edit, as drafted:

  ```markdown
  ### A literal the checker would misread ships through a delivery substitution the build fills `##delivery-substitutions`

  A literal the checker would misread is a delivery substitution, which the build fills. The owner's
  argument: "to avoid convoluted sentences (such as for the primer import line), the text could be
  applied using substitution at build or install time, as documented exceptions where we are avoiding
  the checker on purpose in order to facilitate delivery of the installed skills."
  ```

  Precedent quoted by the draft, 3c4f97f: "For kind D, for those that recorded my words, if they also
  carry argumentation, move them to %% comments, without my words."
- Verifier: verified. "Outcome put to the owner is right, since a66 is a ground."
- Check: the rival `delivery-substitutions` **fails** ("the ground is not split by origin; the title
  is wider than the decision as built; the entry test depends on the owner"). Its corrected text:

  ```markdown
  ### A literal that must ship verbatim and that the checker would misread ships through a delivery substitution the build fills `##delivery-substitutions`

  A literal that must ship verbatim and that the checker would misread in this repository's walk is a
  delivery substitution, which the build fills; a sentence that can be rewritten is rewritten. The
  substitution is the owner's proposal: "to avoid convoluted sentences (such as for the primer import
  line), the text could be applied using substitution at build or install time, as documented
  exceptions where we are avoiding the checker on purpose in order to facilitate delivery of the
  installed skills." The build fills it, not the install, because its literals do not vary by
  project.
  ```
- Cross-check: "Default: the removal. If the owner prefers the rival split, use the check's corrected
  head text".

**agent-skills-2 14, `design@agent-skills@transcript-review-last-before-merge`** (line 1393)

- Rule failed: "A head holds one decision".
- Evidence: "Its own repairs land as additional commits, as many as their kinds need: a repair that
  the primer routes to a commit of its own stays one, and nothing asks the dispatcher to merge them
  into a single commit. The additional commits are the owner's proposal: "I'd just replace "as a last
  commit" by "as an additional commit". This is simpler, it prevents any possible conflict, without
  exceptions and no need to check a condition."" "the second loses to "a last commit", on the owner's
  premise of simplicity and no conflict."
- Edit as drafted: in this head, the landing sentences become "Its own repairs land as
  `design@agent-skills@review-repair-appended-or-folded` says."; in
  `design@agent-skills@review-repair-appended-or-folded`, appended: "The repairs of the last
  transcript review land so too, as additional commits, as many as their kinds need: a repair that
  the primer routes to a commit of its own stays one, and nothing asks the dispatcher to merge them
  into a single commit. The additional commits are the owner's proposal: "I'd just replace "as a last
  commit" by "as an additional commit". This is simpler, it prevents any possible conflict, without
  exceptions and no need to check a condition.""
- Verifier: verified. "move goes to the owner as it moves the owner's words".
- Check: the shortened head **conforms**; the receiving head **fails** ("one decision per head;
  failure mode "add a member the argument does not cover""): "That rival edits no history either, so
  the receiving head's argument does not defeat it." Proposed instead: "### The last transcript
  review's repairs land as additional commits, as many as their kinds need
  (slug transcript-review-repairs-are-additional-commits)", "with the moved text, and the owner's words
  as its ground. That head would then be checked in its turn."
- Cross-check default: "the check's version, a head of its own ... judged alone before the commit."

**agent-skills-2 18, `design@agent-skills@audit-method`** (line 1469)

- Rule failed: "A head holds one decision".
- Evidence: title "An audit's method holds for every axis, each giving its own dispatch, and nothing
  its agents propose is applied before the session's reading, fresh agents re-checking each pass
  while the violations left fall". "Decision 2, re-check passes repeated while the count falls: its
  nearest rival, "The rival, a lighter method with no re-check, lost on real audits ...", and its own
  ground, the owner's stop rule".
- Edit as drafted: the kept head retitled "### An audit's method holds for every axis, each axis
  giving its own dispatch, and nothing its agents propose is applied before the session's reading
  (slug audit-method)"; new head:

      ### Fresh agents re-check each pass of an audit, and passes repeat while the violations left fall, until the owner stops them `##audit-passes-repeat-while-the-count-falls`

      After a pass's edits, fresh agents of the axis, dispatched as its first dispatch was and given the
      kept list, re-check the result, and their findings start the next pass. Passes repeat while the
      violations left fall from one pass to the next, until the owner judges what is left small enough.
      A violation left is a finding of a re-check that a reading confirms, that is on no kept list and
      that no issue entry records. Which of them the count reads is
      `design@agent-skills@convergence-count-scope`. A pass where that count does not fall goes to the
      owner as a question, with the counts, and starts no pass on its own. The stop rule is the owner's,
      given in place of a fixed threshold: "As long as we observe a "convergence", and the amount of
      leftover violations is small enough, I think that is good. If you ask an LLM to find a flaw or
      defect in something, it will nearly always find one IMO. Especialy if that something is prose and
      not code."

      The rival, a lighter method with no re-check, lost on real audits, whose commits
      `git log --grep='design-record axis'` finds: in the first audit of the design-record axis on this
      repository, the re-check of each pass found violations the pass had missed or made.

  Re-points: convergence-count-scope's "The stop rule of `design@agent-skills@audit-method`"; issue
  an-upstream-finding-stays-in-every-later-old-text-count line 53; the rejected alternative "An audit
  method with no calibration and no re-check".
- Verifier: verified. "spec's harvest rewrites the head, so carry the split there".
- Check: kept head **fails** one decision per head ("A title joined by "and" over two decisions shows
  a bundle"); repair: "### An audit's method holds for every axis, each axis giving its own dispatch
  (slug audit-method)". New head **conforms**.
- Cross-check default: "the split, with the check's title for the kept head ... Apply it in the spec
  design-record-workflow's harvest (1.1)."

**agent-skills-2 25, `design@agent-skills@design-record-axis`** (line 1697)

- Rule failed: "A head holds one decision".
- Evidence: title "The design-record axis re-applies the rules on design heads and on rejected
  alternatives to every entry, in bounded groups calibrated on a shared sample, reading the tripwires
  and issues that bear on each head". "Decision 2 ... its ground is the owner's words, "Reading the
  tripwires and issues related to design is important IMO, so that the audit on design takes them
  into account.""
- Edit as drafted: retitle to "### The design-record axis re-applies the rules on design heads and on
  rejected alternatives to every entry, in bounded groups calibrated on a shared sample
  (slug design-record-axis)"; new head:

      ### The design-record axis reads each head with the tripwires guarding it and the issues naming it, and edits one only where its own edit leaves it stale `##design-record-axis-reads-tripwires-and-issues`

      Each head is read with the tripwires guarding it and the issues naming it. The owner narrowed the
      axis to that reading: "Reading the tripwires and issues related to design is important IMO, so
      that the audit on design takes them into account." The audit repairs a tripwire or an issue only
      where its own edit makes it stale: the conformance of a tripwire or an issue to its own register's
      rules is the axis of that register, and a judgement of it here would read entries the decision
      record does not hold, while a stale one the audit's edit leaves is a defect of the audit itself.

  `tripwire@agent-skills@audit-edit-leaves-an-entry-stale`: heading and Response re-pointed.
- Verifier: verified. "Caveat for the checker: the retained design-record-axis still holds the axis's
  scope and exclusions (the restatements on the owner's word) beside its dispatch, which share no
  premise either".
- Check: retitled head **fails** one decision per head, and "the edit splits one owner ruling across
  two heads"; new head **conforms** ("one note on tense"). The check's cut: "### The design-record axis
  re-applies the rules on design heads and on rejected alternatives to every entry, reading the
  tripwires and issues that bear on each head and judging neither them, the restatements nor the code
  (slug design-record-axis)", and "### The design-record axis is dispatched in bounded groups calibrated
  on a shared sample (slug design-record-axis-dispatch)".
- Cross-check default: "the check's cut ... Apply it in the spec's harvest. The stale entries depend
  on the cut chosen (2.1)."

**agent-skills-2 36, `design@agent-skills@intake-shares-the-repair-standard`** (line 2017), plausible

- Rule failed: `skill@knowledge-architect-decision-recording@owning-component`.
- Evidence: "The intake is `skill@klarch-retrospective-intake`, this repository's own skill". "The
  owner's direction concerns the intake: "I think it's important we ensure it analyzes findings and
  repairs on the same standard as our workflow."" "that the intake shares the standard tells no
  adopting project anything."
- Edit: the head moves to docs/design.md beside `design@knowledge-architect@committed-findings-analysis`,
  as "### The intake of received findings judges a repair of agent-facing text by the installed audit's
  repair standard (slug intake-shares-the-repair-standard)"; in `design@agent-skills@root-cause-repair`,
  "and each skill that names it, per `design@agent-skills@intake-shares-the-repair-standard`" becomes
  "and each skill that names it".
- Verifier: **plausible**. "the head also argues where the installed standard lives against the rival
  skill@knowledge-architect-agent-configuration@content-and-style, a decision about installed text that
  the move would take out of agent-skills; the retitle also narrows the title. No reading settles which
  Component the whole head belongs to".
- Cross-check: "A second default: split the installed half and the intake's half instead of moving
  the whole head."

**agent-skills-2 37, `design@agent-skills@audit-triggers`** (line 2038)

- Rule failed: "A title states the rule that decided, not the list of what it admits today."
- Evidence: title "An audit runs at the owner's request, at a pin move whose changelog cites its axis,
  and at the end of an adopting project's move of its documents". The rule in the body: "Each occasion
  is one whose own work needs the record read under the rules. The rival, a periodic audit, has no
  evidence for a period". The owner, spec design-record-audit (69aa5e4^, lines 324-331):
  "audit-triggers: agreed on the main trigger being my request. Good idea on the changelog naming the
  axis to run. And agreed for the existing docs of an adopting project."
- Edit, before: "### An audit runs at the owner's request, at a pin move whose changelog cites its
  axis, and at the end of an adopting project's move of its documents (slug audit-triggers)"; after: "###
  An audit runs on an occasion whose own work needs the record read under the rules, never on a period
  (slug audit-triggers)".
- Verifier: verified. "no closure argued; owner approved the members by name (69aa5e4^ spec line
  329), so owner list".

**agent-skills-2 53, `design@agent-skills@gates-convention`** (line 2300)

- Rule failed: "A head holds one decision".
- Evidence: "The setup skill proposes two goals for such a tool, for the owner's ruling: one command
  runs every check owed before a merge, and a task performed repeatedly becomes a command of the tool.
  The proposal is the owner's: "I would also recommend setting these two goals (with those wordings) in
  the project setting-up skill"." "The owner was not asked whether the words record the owner's intent
  (test 4)."
- Edit: common, delete that passage from the head. Form A (test-4 answer given), a new head "### The
  setup skill proposes the owner's two goals for a project's maintenance tool, in the owner's wordings
  (slug setup-proposes-the-tool-goals)" holding the passage and "Asked whether it records the owner's
  intent, the owner answered: "<the answer>"." Form B: a `%%` line before "Propose these two goals for
  it" in content/skills/setup/SKILL.md: "%% The two goals and their wordings are the owner's proposal:
  "I would also recommend setting these two goals (with those wordings) in the project setting-up
  skill"."
- Verifier: verified. "the owner proposed the two goals (955a610 table, row "maintenance-tool goals in
  setting-up")".
- Check: the shortened head **conforms**; form A **fails** "the title rule on a closed set (a count in
  the title, no closure sentence in the body)". Repair: "add to the body "The set is the owner's: a
  goal added to it or reworded goes to the owner." Then either name the two goals in the title, or
  drop the count".
- Cross-check default: "form B ..., since no test-4 answer is on record."

**agent-skills-3 05, `design@agent-skills@relying-on-the-checker`** (line 2376)

- Rule failed: "What earns a head": no entry test holds.
- Evidence: the head, "A project cannot reference an entry of another project, so a decision that
  relies on the checker states that it relies on the checker working as intended. This holds until a
  form for such references exists, `issue@core@cross-project-references`." The one stating text,
  crates/agent-skills/content/PRIMER.md:293-295: "A decision that relies on the checker of this
  workflow states that it relies on the checker working as intended. It cannot reference the
  checker's own decisions: a reference resolves only against the project that holds it."
- Edit: remove the head (lines 2376-2380); in crates/agent-skills/content/PRIMER.md, above line 293:
  "%% Until a reference can name an entry of another project, `issue@core@cross-project-references`.";
  crates/core/docs/open-issues/cross-project-references.md:28, "per
  `design@agent-skills@relying-on-the-checker`" becomes "per `primer@design-heads`".
- Verifier: verified. "the owner's words in bd93004's plan, "I doubt there is a real need to reference
  design decisions of another project", are hedged and about deferring the issue, not the interim
  rule; precedent F71 of 47765c0 verbatim".

### 3.3 Nothing: recorded by an issue entry

| group / draft | entry | finding | issue |
| --- | --- | --- | --- |
| packed-1 32 | `design@gates@a-project-holds-its-gate-list` | one decision per head: "The library depends on nothing of the checker: the root finder takes the marker file's name from the caller." | `issue@gates@the-gate-list-head-bundles-the-library-s-independence` |
| core-2 20 | `design@core@candidate-rule-and-retired-forms` | one decision per head | `issue@core@four-heads-bundle-decisions-with-many-citing-sites`; the verifier: "The draft also lists "A fullwidth at sign is not an `@`", which the issue does not name. That adds a member to a recorded bundle and does not change the outcome." |
| core-2 23, finding 2 | `design@core@every-path-names-its-anchor` | one decision per head | the same issue |
| core-2 37 | `design@core@a-commit-message-is-a-document` | one decision per head, five decisions | the same issue |
| core-2 41 | `design@core@owned-namespace-check` | one decision per head | the same issue |
| agent-skills-2 22 | `design@agent-skills@audit-axis-runs-a-workflow` | conforms; one sentence false of the installed text, "The design-record axis has none yet" | `issue@agent-skills@the-design-record-axis-has-no-saved-workflow` and the harvest row of the spec design-record-workflow |
| agent-skills-2 42, the bundle | `design@agent-skills@primer-content` | one decision per head | `issue@agent-skills@primer-content-bundles-each-directive-s-decision` |

The cross-check on the re-entry points: "Its re-entry point includes this run; the session says in
the commit that the bundles stay with it." (core-2 20, 37, 41), and for primer-content: "The bundle
issue names "the next run of the design-record axis" as its re-entry point, and this pass is that
run, so its outcome ("nothing") needs the session's reading at the re-entry point".

### 3.4 Refuted, and conforming with a doubt

The cross-check, part 3.3: "None of these is a finding at the moment. Each is listed because the
reading the session settles could turn it into one." They are listed in part 6.

## 4. The overlap map and the stale entries

Quoted from cross-check/cross-check.md, parts 1 and 2, whole. "as-1" is group agent-skills-1, and so on; "draft NN" is the draft file of that number in the group's directory; section numbers inside the tables (1.1, 2.1, part 3) are the cross-check's.

### 4.1 Two findings that edit one paragraph, one head or one slug

| findings | what they share | conflict and what the session does |
| --- | --- | --- |
| packed-1 08: finding A and finding B, on `design@knowledge-architect@versioning-policy` | one head; A turns on the sentence "The table is the owner's scheme ... the first is the owner's argument", B on the scope paragraph "A change internal to this repository ..." | Different paragraphs. Put both to the owner in adjacent items, since both turn on how the head credits the owner, and apply whatever is ruled in one edit of the head. |
| as-2 18 (split of `design@agent-skills@audit-method`) and as-2 25 (split of `design@agent-skills@design-record-axis`) | 18's kept text of audit-method ends "The groups and the sample are the design-record axis's dispatch, per `design@agent-skills@design-record-axis`" | Under 25 as drafted, the groups and the sample stay in `design-record-axis`, and the sentence stays true. Under the check's cut of 25 (a `design-record-axis` holding the corpus, and a `design-record-axis-dispatch`), the pointer in 18's text moves to `design-record-axis-dispatch`. The two items must be ruled together. |
| as-2 18, as-2 25, as-2 30 (`design@agent-skills@verifier-in-the-run`) and the known finding as-2 22 (`design@agent-skills@audit-axis-runs-a-workflow`) | the open spec `spec@plans@design-record-workflow` on this branch rewrites each of these four heads at its harvest (its table rows at lines 167-190) | Each edit of this pass on these heads lands on text the harvest rewrites. 18 and 25 already say so ("best carried with the spec's harvest"). 30's pointer can be lost when the harvest rewrites verifier-in-the-run (row 189). The spec's criteria c2, c3, c4 and c7 bind "from" `design@agent-skills@audit-method` or `design@agent-skills@design-record-axis`, so a split changes which slug each criterion means. Default: apply 18, 25 and 30 in the harvest commit of that spec, and record that ruling in the pass's report file. |
| as-1 21 (`capabilities-not-structure`) and as-1 22 (`expectation-set-bounds-scope`, plausible: conforming) | the section opening at crates/agent-skills/docs/design.md:342-343, "The measurements and incidents these entries cite were taken in the designing-together repository, whose record is docs/decisions.md on its branch next." | Under reading 3 both heads rest on that record. One edit of the section opening, adding the command that re-takes a figure from it (`git show next:docs/decisions.md` in that repository), covers every head of the section, instead of one edit per head. |
| as-1 01 (split of `content-mirrors-the-install-layout`) and the check of as-1 01 | the created head `saved-workflow-name-is-its-file-stem` | The check finds the created head conforms except under reading 3, where it owes the re-take of the probe. That is the same reading as as-1 21: settle it once. |
| core-1 18, core-1 23, core-2 19 | the same phrase, "the failure this tool exists to prevent", in three heads of crates/core/docs/design.md (lines 410-413, 507-509, 1386-1387) | Different paragraphs, but different goals for the same words: core-1 names `goal@core@declared-instructions-are-checked` ("A tree that passes the check conforms to all of it"); core-2 names `goal@core@documents-stay-current` ("Every reference resolves ..."). The core-1 verifier read 31, 32, 33 and 51 under the same rule. The session picks one goal for the phrase and uses it at all three sites. Default: `goal@core@declared-instructions-are-checked`, since each site argues against a silent pass, and a pass that hides an unchecked reference is a tree that passes without conforming. |
| core-1 33 (`anchors-are-components-and-locations`) and core-2 23 (`every-path-names-its-anchor`) | both add `goal@core@relocation-is-one-manifest-edit` for the same property; 33's sentence also cites `design@core@every-path-names-its-anchor` | Different paragraphs, no conflict. Apply them in one commit with one wording. |
| packed-1 35 (split of `design@gates@one-spawn-helper`) and packed-1 50 (removal of `design@xtask@root-finder-uses-the-core-constant`) | the paragraph of `design@xtask@one-module-per-subcommand` (tools/xtask/docs/design.md:13-19) | 50 deletes "per `design@xtask@root-finder-uses-the-core-constant`" from it. 35 keeps its reference "which `design@gates@one-spawn-helper` keeps in its process module", and stays true under the check's narrowed 35-A title. One editor for the paragraph. |
| as-2 36 (move of `intake-shares-the-repair-standard` into docs/design.md), packed-1 23 (`workflow-findings-stay-here`, docs/design.md:373-397), packed-1 25 (removal of `analysis-names-nothing-of-another-project`, docs/design.md:420-425) | 36 writes the head "beside `design@knowledge-architect@committed-findings-analysis`" (docs/design.md:399), between the two other edits | No shared paragraph. All three concern this repository's intake of received findings, so if the owner removes 25 and moves 36, the moved head sits where 25's head was. One editor for docs/design.md. |
| as-1 04 (removal of the delivery-substitution sentences from `shipped-text-cites-no-entry`) and as-1 01 | 04's default rests on the mention in `content-mirrors-the-install-layout`, "a delivery placeholder is a literal the build fills", staying | 01 keeps the layout's exceptions in the source head, so the mention stays. No conflict, as long as the edit of 01 is applied as drafted. |
| as-2 42 (tense of `primer-content`) and core-2 23 (goal reference in `every-path-names-its-anchor`) | each edits a head whose bundle an open issue records (`issue@agent-skills@primer-content-bundles-each-directive-s-decision`, `issue@core@four-heads-bundle-decisions-with-many-citing-sites`) | Not between the two, but with `design@agent-skills@existing-heads-on-touch`: "A head that a change touches is brought to these rules in that change", and "Re-pointing a reference in a head does not touch it". Adding a reference or rewording a sentence is more than re-pointing. Both issues name the next run of this axis as a re-entry point. The issue on the four core heads already records the same case after the first run ("So both heads are a change's touched heads left short of `design@agent-skills@existing-heads-on-touch`"). The session decides: apply the edits and leave the bundles to the issues, as the first run did, or hold both edits until the splits. Default: apply, and say in the pass's commit message that the bundles stay with their issues. |

### 4.2 Renames, and the citing sites another finding edits

| rename | citing sites | edited by another finding? |
| --- | --- | --- |
| core-1 48: `design@core@slice-spec-sections` → design@core@plan-document-sections-by-kind | crates/core/README.md:420 only (`cargo klarch show`) | No. core-2 02 edits crates/core/README.md:239, a different section. The check adds a condition: the commit states the pair `#step-spec-sections → plan-document-sections-by-kind` and names the old slug. |
| check of packed-1 09: the created slug `released-changelog-sections-keep-their-content` → `changelog-section-fixed-at-release` | the re-points draft 09 lists: CLAUDE.md:197 and :221, crates/core/docs/design.md:1638 (in `design@core@bare-skill-name-reported`), crates/agent-skills/docs/design.md:945 (in `design@agent-skills@plain-text-is-no-repair`), .claude/agents/klarch-changelog-reviewer.md:43 | No other finding edits these lines. packed-1 23 edits CLAUDE.md:479-481, a different paragraph. The two heads holding lines 1638 and 945 belong to core-2 and as-1, which judged them conforming. |
| check of as-1 33: the created slug `ruled-tripwire-written-with-its-decision` → `premortem-tripwire-written-at-harvest` | the sites that need the created head (see 1.3) | No. |

### 4.3 Splits, merges and moves whose rejected alternative or citing site another group drafted, or that the draft missed

| edit | site | drafted by | what the session does |
| --- | --- | --- | --- |
| as-2 18 (split of audit-method) | rejected alternative "An audit method with no calibration and no re-check", crates/agent-skills/docs/rejected-alternatives.md:182-183, "lost to `design@agent-skills@audit-method` for the re-check, and to `design@agent-skills@design-record-axis` for the calibration" | as-3 36, verified conforming | 18 re-points the re-check part to `audit-passes-repeat-while-the-count-falls`. |
| as-2 25 (split of design-record-axis) | the same rejected alternative, its calibration part | as-3 36 | It stays on `design-record-axis` under 25 as drafted. Under the check's cut it moves to `design-record-axis-dispatch`. |
| as-1 01 (split of content-mirrors-the-install-layout) | crates/agent-skills/docs/design.md:1579, in `design@agent-skills@audit-axis-runs-a-workflow`: "the harness runs it by the name its `meta` declares, per `design@agent-skills@content-mirrors-the-install-layout`" | as-2 22 (conforming, known false sentence) | The draft does not list this site. It cites exactly the part that moves, so it is re-pointed to design@agent-skills@saved-workflow-name-is-its-file-stem. The spec's harvest rewrites this head too (row 183). Draft 01 also leaves crates/agent-skills/CLAUDE.md:6 as optional, and crates/core/docs/design.md:2221 cites the slug placeholders, which stay. |
| as-1 33 (split of premortem-tripwires-on-the-owners-word) | crates/agent-skills/content/skills/decision-recording/SKILL.md:242: `%% Written with its decision: \design@agent-skills@premortem-tripwires-on-the-owners-word\.` | not judged (installed text) | The draft does not list this site. It cites the timing part that moves, so it is re-pointed to the created head. The other three sites (design.md:729, :1144, :1163) cite the owner's word and stay, as the draft says. |
| packed-1 09 (split of changelog-entries) | docs/plans/specs/path-quickfixes.md:220 lists `design@knowledge-architect@changelog-entries` among the sites `design@core@bare-skill-name-reported` touches; docs/open-issues/a-mechanical-changelog-check.md:9 and :43 | spec text; issue (part 2) | The draft does not list these three sites. Each cites the released-section rule, which moves. They are re-pointed to the created head. |
| packed-1 09 | crates/agent-skills/docs/design.md:2042 (in `design@agent-skills@audit-triggers`) | as-2 37 (retitle) | 37 edits only the title, at line 2038. Line 2042 cites the entry tests and keeps the parent's slug. No conflict. |
| core-2 02 (split of generated-files-are-pure) | rejected alternative at crates/core/docs/rejected-alternatives.md:296 (last-change date in an index); crates/core/docs/design.md:967 (in `design@core@a-file-register-is-a-directory-of-entries`) and :1050 | core-3 29 (plausible: conforming); core-1 (head at 947) and core-2 | Each cites purity and stays on `generated-files-are-pure`, as draft 02 and its check say. The check also proposes moving the missing-directory clause into `design@core@one-list-of-generated-files` (core-2 03, verified conforming). That edit would change a head another draft judged conforming. Judge it again before the commit. |
| packed-1 35 (split of one-spawn-helper) | gates rejected alternative "Capturing a child's two streams through one regular file instead of two pipes" (crates/gates/docs/rejected-alternatives.md:8); `design@xtask@one-module-per-subcommand` | packed-1 43 and 47, same group | Re-point the rejected alternative to `streams-through-two-pipes`, as drafted. The xtask head keeps the parent. |
| as-2 36 (move of intake-shares-the-repair-standard to the root) | crates/agent-skills/docs/design.md:1924, in `design@agent-skills@root-cause-repair`; destination beside `design@knowledge-architect@committed-findings-analysis` | as-2 32 (verified conforming); packed-1 24 (verified conforming) | The reference in root-cause-repair is removed, as drafted. The destination head stays as it is. |
| as-1 01 (created head) | cites `design@core@owned-namespace-check` | core-2 41, known bundle in `issue@core@four-heads-bundle-decisions-with-many-citing-sites` | One more citing site for a head the issue will split. The new site cites the namespace part (the second decision the issue lists). No edit now; the split re-points it. |
| as-2 14 (move into review-repair-appended-or-folded) | the receiving head | as-2 01, same group (conforming) | The check fails the receiving head after the move. See part 3. |
| packed-1 23 (L7 reason moved into CLAUDE.md:479-481) and packed-1 09 (re-points CLAUDE.md:197, :221) | the root CLAUDE.md | same group | Disjoint paragraphs. An edit of the root CLAUDE.md goes through `skill@knowledge-architect-agent-configuration`. |

### 4.4 The stale entries

Every tripwire and issue that `cargo klarch show` lists for a head a verified or plausible finding
edits, read whole, with whether the edit leaves it false. Heads with no tripwire and no issue:
content-mirrors-the-install-layout, premortem-tripwires-on-the-owners-word,
a-reference-claims-a-revisit, transcript-reviewer-agent, transcript-review-last-before-merge,
intake-shares-the-repair-standard, gates-convention, retro-trigger, a-failed-parse-is-loud,
the-regime-has-no-opt-out, anchors-are-components-and-locations, slice-spec-sections,
a-wrong-declaration-is-a-finding, milestone-entries-are-directories, generated-files-are-pure,
fix-scope, analysis-names-nothing-of-another-project, one-spawn-helper,
root-finder-uses-the-core-constant, one-module-per-subcommand.

#### 4.4.1 Left false by an edit: repaired in place, in the commit of that edit

| entry | head and finding | why it goes false | repair |
| --- | --- | --- | --- |
| `issue@core@cross-project-references`, line 28 | `relying-on-the-checker`, as-3 05 (removal) | it cites the removed head: "needs no pointer into the checker's design, per `design@agent-skills@relying-on-the-checker`" | as drafted: `per \primer@design-heads\`. The primer's section holds the sentence "A decision that relies on the checker of this workflow states that it relies on the checker working as intended". |
| `issue@agent-skills@an-upstream-finding-stays-in-every-later-old-text-count`, line 53 | `audit-method`, as-2 18 (split) | "`design@agent-skills@audit-method` makes the fall of the count on old text the convergence signal": that sentence moves to `audit-passes-repeat-while-the-count-falls` | re-point, as drafted. |
| `tripwire@agent-skills@audit-edit-leaves-an-entry-stale` (tripwires.md:277-286) | `design-record-axis`, as-2 25 (split as drafted) | its heading guards `design@agent-skills@design-record-axis`, its premise is "reading each head's tripwires and issues before judging it", and its Response is "reopen `design@agent-skills@design-record-axis`": that reading moves to the new head | Re-point heading and Response, as drafted. Under the check's cut it stays true and is not edited. |
| `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it`, line 24 | `design-record-axis`, as-2 25 (split as drafted) | "The design-record axis reads "the tripwires guarding it, the issues naming it" for each head, per `design@agent-skills@design-record-axis`": that sentence moves to the new head | Draft 25 does not list this site. Re-point it to `design-record-axis-reads-tripwires-and-issues`. Under the check's cut it stays true. The spec's harvest row 168 also reads this sentence again. |
| `issue@agent-skills@audit-axes-beyond-the-design-record`, line 91 | `design-record-axis`, as-2 25 (check's cut only) | "the design-record axis keeps its bounded groups and its calibration sample, per `design@agent-skills@design-record-axis`" | True under 25 as drafted. Under the check's cut, re-point it to `design-record-axis-dispatch`. |
| `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, line 54 | rejected alternative "Skipping a commit whose tree fails, in `commits`", core-2 49 (removal) | it names that rejected alternative as the rival of the third decision of `design@core@a-commit-message-is-a-document` | If the owner rules the removal, reword in the same commit, e.g. "against skipping a commit whose tree fails and against building each commit's own checker". The issue's rule ("A rejected alternative that names the head cannot be checked against the one decision it lost to") stays true. Draft 49 does not mention this site. |
| `issue@knowledge-architect@a-mechanical-changelog-check`, lines 9 and 43 | `changelog-entries`, packed-1 09 (split) | line 9, "the shape that `design@knowledge-architect@changelog-entries` and the preamble ... fix", and line 43, "It strains `design@knowledge-architect@changelog-entries`, whose rule on released sections has no mechanical guard": the section shape and the released-section rule move to the created head | Draft 09 does not list them. Line 43 is re-pointed to the created head. Line 9 names both heads, since the issue's check asserts the entry shapes, the parent's rule, and the sections, the created head's rule. |

#### 4.4.2 Read, and left true by the edit

| entry | head and finding | why it stays true |
| --- | --- | --- |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` (21, 35) | shipped-text-cites-no-entry, as-1 04 | it cites the citing rule, which stays. The delivery substitution is not in it. |
| `tripwire@agent-skills@structure-rejection-overruled`; `issue@agent-skills@a-reviewer-s-running-time-is-unbounded`; `issue@agent-skills@the-retrospective-counts-no-review-cost` | capabilities-not-structure, as-1 21 | adding a command over the record changes no claim they rest on. |
| `tripwire@agent-skills@ruling-lost-in-change`; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` (20) | in-change-path, as-1 49 | the tripwire reopens the head "on its entry condition", which neither the retitle nor the removed approval changes. |
| `tripwire@agent-skills@design-conformance-findings-needless`; `issue@agent-skills@audit-axes-beyond-the-design-record` (70) | plan-read-against-the-record, as-2 13 | a pointer is added. |
| `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` (28) | review-repair-appended-or-folded, as-2 14 | it cites the fold, which stays. The moved member, whether as drafted or as the check's own head, does not touch it. |
| `tripwire@agent-skills@audit-defaults-overruled` (252-262) | audit-method, as-2 18 | It guards the owner list's defaults, which stay with audit-method, as the check agrees. Its premise, "the sort of an axis with no saved workflow", is already false at the commit audited. The edit does not cause that, and the spec's harvest row 179 records it. |
| `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names` (30) | audit-method, as-2 18 | "drafts and reports live in files": paragraph 1 of the kept head still holds "each agent writes its drafts in a scratch directory of its own". |
| `issue@agent-skills@audit-axes-beyond-the-design-record` (89) | audit-method, as-2 18 | "generic over the axes, each axis giving its own dispatch" stays in the kept head. |
| `issue@agent-skills@the-design-record-axis-never-judges-a-head-s-component` (14, 79, 88) | design-record-axis, as-2 25 | The axis's rules sentence stays in `design-record-axis` under both cuts. Note: the installed skill at this commit already lists the owning-component questions among the axis's rules. The design head does not. That divergence is the issue's subject, and the spec's harvest closes it (row 170). |
| `tripwire@agent-skills@audit-drafts-mostly-noise`; `tripwire@agent-skills@audit-passes-do-not-converge` (373) | workflow-axis-finding-standard, as-2 28 | a pointer would be added. |
| `tripwire@agent-skills@verifier-refutes-a-true-finding`; `tripwire@agent-skills@verifiers-wave-findings-through` | verifier-in-the-run, as-2 30 | a pointer is added. |
| `tripwire@agent-skills@audit-passes-do-not-converge` (361, 371, 372) | root-cause-repair, as-2 36; convergence-count-scope, as-2 18 re-point | 36 removes a reference from the body, and the tripwire names neither the intake nor that sentence. 18 re-points "The stop rule of" in convergence-count-scope, which the tripwire does not quote. |
| `tripwire@agent-skills@axis-not-run-at-a-pin-move`; `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` | audit-triggers, as-2 37 | the body still names each occasion as a member, as the draft says. |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision`; `issue@agent-skills@the-material-finding-duty-has-no-head`; `issue@agent-skills@a-repair-that-edits-a-head-reads-no-entry-citing-it` (30) | primer-content, as-2 42 | the tense edit keeps the bullet and the count of eight bullets. The bundle issue names "the next run of the design-record axis" as its re-entry point, and this pass is that run, so its outcome ("nothing") needs the session's reading at the re-entry point (1.1, last row). |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` (34); `issue@agent-skills@patching-an-installed-skill` (48) | premortem-as-watch-points, as-3 11 | a goal reference is added. |
| `tripwire@core@walked-count-differs-between-machines` (tripwires.md:15) | failed-git-is-exit-two, core-1 23 | a goal reference is added. |
| `issue@core@a-checker-only-project-carries-the-workflow-skeleton`; `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply` | components-carry-the-same-documents, core-1 32 | a goal reference is added. |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` (69, 87); `issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` | an-unresolved-reference-is-reported, core-2 19 | a goal reference is added. The five ways stay. |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` (82); `issue@core@four-heads-bundle-decisions-with-many-citing-sites` (35) | every-path-names-its-anchor, core-2 23 | a goal reference is added. The issue's note "Its argument, that relocating an anchor edits one manifest line, is not the grammar's" stays true, and draft 23 says the reference goes with that paragraph at the split. |
| `issue@core@manifest-table-exposes-the-toml-crate`; `issue@knowledge-architect@command-output-is-not-declared-a-contract`; `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check` | versioning-policy, packed-1 08 A and B | they cite the table and its rows, which neither edit changes. |
| `tripwire@core@checker-source-literals-are-data-self-location` (core tripwires.md:80) | a-build-is-tied-to-its-checkout, packed-1 21 | "a build tied to its checkout per ..." stays true of the narrowed title, as the verifier says. |
| `issue@agent-skills@patching-an-installed-skill` (32) | workflow-findings-stay-here, packed-1 23 | it cites the retrospective's workflow file handled here, which stays. The L7 paragraph is not in it. |

## 5. The owner list

Twenty-one items, grouped by kind, each labelled `F<n>` with this file as its record, per
`primer@labelled-items`. The evidence and the full edit
texts of each item are in part 3.2. Each item ends with a `Default` line and an `Outcome` line; the
`Outcome` line is the owner's.

### A verdict where the rule reads two ways

**F1. `design@knowledge-architect@versioning-policy`, finding A: two owner credits with no quoted
words.** The head: "The table is the owner's scheme. The bullets below refine it, and the first is
the owner's argument, given in the ruling that a stricter check is minor." The rule admits both: "The
owner's words are a ground, quoted, only where the decision came from the owner" (reading 1, fails),
and "In a head that holds a part of each, the part that came from the owner says so, and the rest
stands on its argument" (reading 2, conforms). On record: the table, "**The policy (owner's
scheme).**" (82fc518^, section 4.5), no verbatim; the first bullet, "The owner ruled "minor wins""
(e646ead). The owner accepted an unquoted credit for another head where no verbatim was on record
(8d4179f, F53). Put next to the item on finding B.
Default: reading 2, the head conforms; no edit.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### A ground that is the owner's words

**F2. `design@knowledge-architect@versioning-policy`, finding B: the scope paragraph came from the
owner and does not say so.** The paragraph: "A change internal to this repository, such as a test, a
gate or a subcommand of tools/xtask, is outside the table, and so is a file a crate ships for reading
alone, such as its changelog: each is at most a patch." The owner's words, 72d9517: "the versionning
policy is not supposed to be about checks that are purely internal to knowledge-architect. It is
about checks added in the published crate, and which can affect external consumers." Depends on the
session settling the calibration's reading B.
Default: core-3's edit: append "The scope is the owner's: "the versionning policy is not supposed to
be about checks that are purely internal to knowledge-architect. It is about checks added in the
published crate, and which can affect external consumers."", applied in one edit of the head with
whatever finding A's item rules.
Outcome: attribution, not quotation: the scope paragraph says it is the owner's ruling, without the owner's words. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F3. `design@core@fix-scope`: the title is true of its nearest rival.** The title: "`--fix` admits a
repair only when it passes the safe-fix test". The rival the head names: "a title stating that
`--fix` repairs every finding whose repair passes the test". The title is the owner's wording, 8d4179f,
F64: "I'd word it the other way: "--fix only admit repairs when they pass the safe-fix test".
Otherwise, it becomes a source of defects when we might have never intended to cover some possible
repairs."
Default: retitle to "`--fix` applies only the kinds of repair added to it on purpose, each passing the
safe-fix test", the body and the owner's quoted words unchanged.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F4. `design@core@milestone-entries-are-directories`: the owner's request is paraphrased as the
ground.** The head: "The shape exists because the owner asked to cite a milestone as
`milestone@plans@<id>`". The owner's words, a42 of milestone structured-plans (f191c49^): "I'd like to
be able to reference path@knowledge-architect@docs/plans/structured-plans/ as
milestone@knowledge-architect@structured-plans".
Default: core-1 58's edit, the owner's spans given in words ("The shape exists because the owner
asked for it: "I'd like to be able to reference" a milestone's directory, cited by its path, "as" a
milestone of its own kind, which the plans layout spells `milestone@plans@<id>`."); the session
settles whether the two spans are kept verbatim without backticks instead.
Outcome: no edit: the head already attributes the request to the owner, "The shape exists because the owner asked". The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### The removal of a head, or of a decision in a head, that fails every entry test

**F5. `design@knowledge-architect@analysis-names-nothing-of-another-project`.** One text states it,
`skill@klarch-retrospective-intake`. Test 4: is the owner's "is useless information here anyway"
(4b1240e) a record of the owner's intent? The head comes from a pass-1 split the owner approved by
default (3c4f97f, F22 to F29).
Default: remove the head (docs/design.md:420-425), and give the skill's line its reason: "none of its
internals, which tell a reader of this repository nothing". If the owner answers yes to test 4, the
head stays and quotes the answer.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F6. `design@xtask@root-finder-uses-the-core-constant`.** Its reason is that a copy of a name could
go stale, which test 2 excludes: "a name, a path or a value repeated for consistency is not one".
xtask is never published; no test-4 answer is on record (bd93004).
Default: remove the head (tools/xtask/docs/design.md:55-62); the reason goes to the comments at
tools/xtask/src/gates.rs:32-33 and above tools/xtask/src/changelog.rs:56; the reference in
`design@xtask@one-module-per-subcommand` is removed.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F7. `design@agent-skills@relying-on-the-checker`.** One installed text states it,
crates/agent-skills/content/PRIMER.md:293-295, with its reason beside it. No owner's words.
Default: remove the head (crates/agent-skills/docs/design.md:2376-2380); add above PRIMER.md:293 "%%
Until a reference can name an entry of another project, `issue@core@cross-project-references`.";
re-point crates/core/docs/open-issues/cross-project-references.md:28 to `primer@design-heads`, in the
same commit.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F8. `design@knowledge-architect@workflow-findings-stay-here`, its L7 paragraph.** "For the same
reason, the root `CLAUDE.md` gives the auditors of the checker-rules lens, L7, the checker's design
home, `path@core@docs/design.md`, beside its README." A second decision, stated by one text
(CLAUDE.md:479-481).
Default: delete the paragraph from the head; carry its reason into CLAUDE.md:479-481 ("beside its
README: the installed lens names the README alone, since an adopting project holds no design home of
the checker, and here the checker's rules are decided in that home"), through
`skill@knowledge-architect-agent-configuration`.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F9. `design@agent-skills@shipped-text-cites-no-entry`, its delivery-substitution part.** "A literal
the checker would misread is a delivery substitution, which the build fills. The owner's argument:
"to avoid convoluted sentences (such as for the primer import line), the text could be applied using
substitution at build or install time, as documented exceptions where we are avoiding the checker on
purpose in order to facilitate delivery of the installed skills."" The owner proposed it (a66); its
harvest judged it earns no head (249f145).
Default: remove the two sentences; the reason stays at the table's comment in
`path@agent-skills@build.rs`. If the owner prefers a head of its own, the check's corrected text of
`delivery-substitutions` (part 3.2).
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F10. `design@agent-skills@gates-convention`, its two-goals part.** "The proposal is the owner's: "I
would also recommend setting these two goals (with those wordings) in the project setting-up
skill"." Test 4: do these words record the owner's intent?
Default: form B: the passage leaves the head, and a `%%` line before "Propose these two goals for it"
in content/skills/setup/SKILL.md carries the owner's words. If the owner confirms test 4, form A with
the check's repair (a closure sentence in the body; name the two goals in the title or drop the
count).
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### The removal of a rejected alternative that fails every recording test

**F11. "Skipping a commit whose tree fails, in `commits`"** (crates/core/docs/rejected-alternatives.md:23).
"It loses because each skipped commit took its message out of the regime while the run exited 0, and
the summary line was the only trace."
Default: remove it, and reword line 54 of `issue@core@four-heads-bundle-decisions-with-many-citing-sites`
in the same commit (part 4.4.1).
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F12. "Families named for the subjects that read them"** (rejected-alternatives.md:75). Its reason
applies `design@core@nothing-of-a-project-is-compiled-in`.
Default: remove it.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F13. "One family per invocation, instead of a set"** (rejected-alternatives.md:82). Its reason is
arithmetic over `design@core@model-then-checks`: "a caller wanting five families would read every
live document five times".
Default: remove it.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### A head in the wrong Component

**F14. `design@agent-skills@intake-shares-the-repair-standard`** (plausible). "The audit and the intake
of received findings judge a repair of agent-facing text by one installed standard". The intake is
this repository's own skill; the owner: "I think it's important we ensure it analyzes findings and
repairs on the same standard as our workflow." The verifier's doubt: the head "also argues where the
installed standard lives against the rival
skill@knowledge-architect-agent-configuration@content-and-style".
Default: move the head to docs/design.md beside `design@knowledge-architect@committed-findings-analysis`,
retitled "The intake of received findings judges a repair of agent-facing text by the installed
audit's repair standard". Second default: split the installed half from the intake's half.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### A widening of what the owner's words approved

**F15. `design@agent-skills@audit-triggers`: the title lists the members.** "An audit runs at the
owner's request, at a pin move whose changelog cites its axis, and at the end of an adopting
project's move of its documents". The owner approved the three by name: "audit-triggers: agreed on
the main trigger being my request. Good idea on the changelog naming the axis to run. And agreed for
the existing docs of an adopting project."
Default: retitle to "An audit runs on an occasion whose own work needs the record read under the
rules, never on a period", the body unchanged.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### A finding no row names: a split or a move that moves the owner's words, or creates a head with no recorded argument

**F16. `design@knowledge-architect@changelog-entries`: two decisions, the entry tests and the released
sections.** The title was set to state both by F58 (8d4179f, "Agreed on the defaults for the
others"); the split undoes that ruling. The owner's words "older changelog section allow structural
changes, and this passes as a structural change in my view" move with the released-section part.
"A released section's content never changes" has no recorded argument (43fda63).
Default: the split with the check's repairs: the parent names `goal@knowledge-architect@any-project-can-adopt-it`
and keeps "An entry describes the release's net effect: a change reversed before the release has
none"; the created head is `changelog-section-fixed-at-release`, "A changelog section's content is
fixed at its release". The owner states the reason for the released-section rule, or that part gets
an issue entry, kind `design`. Re-point the sites of part 4.2 and 4.3.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F17. `design@gates@one-spawn-helper`: one helper, and two pipes.** "One spawn helper captures a
child's two streams through two pipes". No argument is on record for the one-helper part.
Default: the split; 35-A titled "The gates start every child through the library's one spawn helper,
which a project's commands may use", the owner asked for its reason; with none, an issue entry, kind
`design`. 35-B, `streams-through-two-pipes`, as drafted; the rejected alternative re-pointed to it.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F18. `design@agent-skills@transcript-review-last-before-merge`: run last, and how its repairs land.**
The owner's words: "I'd just replace "as a last commit" by "as an additional commit". This is
simpler, it prevents any possible conflict, without exceptions and no need to check a condition."
Default: the check's version: a head of its own, `transcript-review-repairs-are-additional-commits`,
"The last transcript review's repairs land as additional commits, as many as their kinds need",
holding the moved text with the owner's words as its ground, judged alone before the commit.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F19. `design@agent-skills@audit-method`: the generic method, and the re-check with its stop rule.**
The owner's stop rule moves to a new head: "As long as we observe a "convergence", and the amount of
leftover violations is small enough, I think that is good. ..."
Default: the split; the kept head titled "An audit's method holds for every axis, each axis giving
its own dispatch"; the new head `audit-passes-repeat-while-the-count-falls` as drafted; applied in
the harvest of the spec design-record-workflow, and ruled together with the next item (part 4.1).
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

**F20. `design@agent-skills@design-record-axis`: the corpus and its reading, and the dispatch.** The
owner's words: "Reading the tripwires and issues related to design is important IMO, so that the
audit on design takes them into account."
Default: the check's cut: `design-record-axis` keeps the corpus, its reading of tripwires and issues,
its limits and the owner's words; `design-record-axis-dispatch` holds the bounded groups and the
shared sample; applied in the harvest of the spec design-record-workflow; the stale entries of part
4.4.1 follow the cut chosen.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

### Added by the session's verification: a head that fails every entry test

**F21. `design@knowledge-architect@klarch-prefix`.** One text states it: the root `CLAUDE.md`,
"This repository's own skills and agents take the prefix `klarch-`", with "This is a restatement;
its home is `design@knowledge-architect@klarch-prefix`." The primer: "For a decision about
agent-facing text, a skill, an agent or a `CLAUDE.md`, each text that states the instruction is one
site: a decision stated by one text has one site, and its reason lives in that text, beside the
instruction". Test 1 fails, since no interface others consume changes; test 3 fails, since the
installer it argues from is this project's; no test-4 answer is on record. The drafter read each
skill name as a site, a reading the primer's sentence rules out.
Default: ask test 4: do you want this head kept as the record of your intent? With no such answer,
remove the head, and carry its reason into the root `CLAUDE.md` beside the instruction, through
`skill@knowledge-architect-agent-configuration`.
Outcome: the default. The owner: "Agreed on all defaults except F2 and F4: no need to quote my words, as long as attribution is correct."

## 6. What the run did not decide

### 6.1 Readings the session settles

- **Reading B** (versioning-policy, "the part that came from the owner says so"): proposed by the
  calibration; packed-1 08 is refuted under it; the packed-1 verifier found only draft 06 meeting the
  same question, left plausible.
- **The fix-scope reading** (the title is true of the rival): proposed by the calibration; core-2's
  verifier confirmed it.
- **Reading 3** (a measurement carries the command that takes it again): proposed by the
  calibration. Two scope questions stay open, as the cross-check states them: "(a) Is an incident
  counted as a measurement? The as-1 verifier read as-1 22 ("an incident, not a measurement") as
  outside the reading. The as-2 verifier read as-2 07 ("a reviewer briefed with it was the one of
  six", design.md:1271-1272) as inside it. These two readings are inconsistent, so one of them is
  corrected. (b) Is a re-take procedure stated in words enough without its record? as-2 28's head
  names the pass and the procedure ... It is the weakest of the refutations." The form of the repair
  for capabilities-not-structure is also open: the calibration says "add the command over that
  record", and no draft writes that edit; the calibration "did not run the command".
- **Finding A** of versioning-policy reads two ways: on the owner list (part 5).
- **core-1 48** is read by the cross-check as "a rewording of the slug"; "If the session reads a
  rename as named by no row, it goes on the owner list by the general rule."
- **agent-skills-1 49**: "If the session reads F116 as approving the title's words, it goes to the
  owner list, as a widening."
- **core-2 02**: whether the missing-directory clause stays in `index-takes-no-dry-run` (cross-check
  default) or moves to `design@core@one-list-of-generated-files` (the check's first repair, which
  edits a head core-2 03 judged conforming: "Judge it again before the commit").
- **The shared phrase** of core-1 18, 23 and core-2 19: one goal for three sites (part 3.1).
- **Touched heads whose bundle an issue records** (agent-skills-2 42, core-2 23): apply the edits and
  leave the bundles to the issues, or hold the edits (part 4.1, last row).
- **The edits on heads the spec design-record-workflow's harvest rewrites** (agent-skills-2 18, 25,
  30, and 22): the cross-check's default is to apply them in that harvest commit and record the
  ruling in the pass's report file (part 4.1).

### 6.2 The plausible verdicts

262 drafts are plausible. Most are conforming drafts the verifier did not read; the verifiers'
reasons are quoted.

| group | plausible drafts | verifier's reason |
| --- | --- | --- |
| packed-1 | 06, 22 | 06: "The v0.1 plan (82fc518^, harvest row) records "owner's proposal", and the head credits nothing to the owner ... no reading confirms or refutes the draft". 22: "One text alone states the instruction (CLAUDE.md:471). The draft's own strict reading in drafts 23 and 25 would fail this head. ... no test-4 answer is on record. The session weighs this" |
| core-1 | 01-17, 19, 21, 22, 24-30, 34-38, 40-47, 49, 50, 52-56 (47 drafts) | "Conforming draft not sampled." for each but 24: "Conforming draft sampled after 32 and 51 proved wrong: "conformant by vacuum" (design.md:523) touches the same goal, but the head's argument is about what git's modes hold, so no reading settles it." |
| core-2 | 01, 04-07, 09, 11-18, 21, 24-36, 38-40, 42, 44-48, 50-53, 56, 57 (43 drafts) | "Not read in full. For 06, 11 and 44 the owner-ground line alone was checked against its head (design.md:1139-1141, :1224-1226, :2189-2191), and each matches the source the draft names." |
| core-3 | 01-07, 09-12, 14, 15, 17-22, 24-53, 55 (50 drafts) | "Conforming draft not sampled; a scan of the group's 58 paragraphs found each with a lost-to reference and a validity marker." |
| agent-skills-1 | 05-20, 22-26, 28-32, 34-38, 40-48, 50-53, 55 (45 drafts) | "Conforming; not read further." for most; 07 "draft read, head not re-read"; 20 "The "and" title read as one rule and its application is defensible ... Not refuted, not fully confirmed."; 22 "Conforming stands unless the session extends the reading to incidents."; 30 "line 692 states that a figure "is not measured" and cites an issue, so no measurement is rested on." |
| agent-skills-2 | 02-06, 08-11, 15-17, 19-21, 23, 26, 27, 29, 31, 35, 36, 38-41, 43-52 (36 drafts) | "conforms; not read by this verifier, no reading confirms or refutes it" for most; 20: "one figure ("by the session's own account, which no file records") has no command by its own statement, which the calibration's reading does not settle"; 36: a failing draft, "No reading settles which Component the whole head belongs to" (part 3.2) |
| agent-skills-3 | 04, 13-16, 18-27, 29-35, 37-53 (39 drafts) | "conforms; not read by this verifier, so no reading confirms or refutes it" for most; 04: "the core issue (written in bd93004) calls it "The owner's rule" and no history read here shows where the rule came from" |

The cross-check's list of drafts a settled reading could turn into a finding (its part 3.3), quoted:

| draft | entry | proposed outcome | doubt |
| --- | --- | --- | --- |
| core-3 58 (refuted finding) | rejected alternative "`index --staged` writing the staged rows into the working-tree file" | nothing | Test 4 holds on the owner's "This is not definitive". If the session reads that hedge as no doubt about a goal, the finding revives as a rule read two ways, and goes on the owner list. |
| as-1 20 | `synthetic-evidence-not-built` | nothing | The "and" title read as one rule with its application. |
| as-1 22 | `expectation-set-bounds-scope` | nothing | It changes if reading 3 covers incidents (3.1 a). |
| as-2 20 | `one-run-pass` | nothing | One figure, "which no file records", says that no command can exist. |
| as-3 04 | `goals-required` | nothing | Where the rule came from is unread. An issue calls it "The owner's rule". |
| packed-1 06 | `binary-bundles-workflow` | nothing | The owner proposed it, and the head credits nothing. The head stands wholly on its argument (reading B). |
| packed-1 22 | `klarch-prefix` | nothing | One text alone states it. The strict test-2 reading the same auditor applied in 23 and 25 would fail it, and no test-4 answer is on record. Of the plausible drafts, this is the one most likely to become a finding. Read it with 23 and 25 so the reading is applied the same way. |
| core-1 24 | `links-are-no-documents` | nothing | "conformant by vacuum" is near the wording of `goal@core@declared-instructions-are-checked`. |
| core-2 22 (verified conforming) | `section-homes-carry-slugs` | nothing | Under a strict reading of "accepted the cost", the owner's words would be an approval cited as a ground. |

### 6.3 "Outside the axis" notes

No draft carries a section titled "Outside the axis". The notes the drafts, checks and the cross-check
mark as outside the axis or not judged:

- packed-1 48, `design@xtask@gates-list-primary-home`: "That is a finding of the head false of the
  code, outside this axis and already recorded" (`issue@xtask@where-ci-and-a-local-run-differ-is-not-only-a-flag`).
- core-2 05, `design@core@index-staged-write`: "`issue@core@index-staged-clears-an-entrys-flags` cites
  it; it records a code defect, outside this axis."
- agent-skills-2 18, 20, 22, 25, 30, "Noted, not judged": sentences of `audit-method`, `one-run-pass`,
  `audit-axis-runs-a-workflow`, `design-record-axis` and `verifier-in-the-run` false of the installed
  text at the commit audited, each listed in the harvest table of the spec design-record-workflow.
- agent-skills-2 45, `design@agent-skills@primer-reread-before-recording`: "Noted, not judged
  (restatements and installed text are outside the axis): the design skill
  (content/skills/design/SKILL.md:147 ...) and the planning skill (content/skills/planning/SKILL.md:423
  ...) also ask the reread, which the head's "so one instruction reaches every write" does not account
  for."
- check of core-2 02, "Met outside the task": "The created head's claim "Remove either check and the
  paragraph above stops being true" may be false of the code for the missing-directory check. Without
  that check, `fs::write` into a missing directory fails partway through, and nothing is destroyed.
  This is not judged here". The cross-check: "If the session confirms it in the code, it is an issue
  entry, kind `defect`."
- check of agent-skills-3 10, "Met outside the task": "The head says "a pull request the session
  worked on has merged", where the skill and the primer say a branch has merged into the main branch.
  This is a restatement difference, not judged here." The cross-check: "dropped here; it is the
  routing of the dispatching session."
- cross-check, "Met outside the task": "The tripwire `audit-defaults-overruled` names "the sort of an
  axis with no saved workflow" in its premise, which is false at the commit audited. Already recorded
  by row 179 of the spec design-record-workflow. Outcome: nothing (known)."

### 6.4 "Met outside the task" notes of the returns

| return | item, quoted |
| --- | --- |
| packed-1 | "The owner's quotation in design@knowledge-architect@the-changelog-ships-in-every-crate, "I'd also provide a `cargo x` subcommand that updates all changelog copies at once", is in no commit message or plan document of the history; 81df1d3 records the request only as "The owner asked for the subcommand." Not a rule failure, since the credit is correct; noted in draft 04 for the verifier, to check against a transcript if one exists." |
| packed-1 | ".claude/agents/klarch-changelog-reviewer.md:22 says "The standard is two heads of docs/design.md"; the split drafted in 09 would make it three. It changes only if that split is applied." |
| core-1 | "none" |
| core-2 | "none" |
| core-3 | "design@core@owned-namespace-check bundles the primer's delivery by an import line with the committed, byte-checked installed files: its title does not state the import line, which design@agent-skills@premortem-as-watch-points and the rejected alternative "The primer delivered by a session-start hook that prints it" cite it for. This head is another group's entry: route it to that group's verifier, or else the session judges it under primer@design-heads, one decision per head." |
| core-3 | "The rejected alternative "`--lines` on thaum's interpretation index" argues from thaum's indexes ("the rule index names its citing files at file level"), which this tree does not hold. It is in my group and conforms on every rule of the axis; whether its text is still true is outside the axis. Dropped: no rule of this axis applies." |
| agent-skills-1 | "`design@agent-skills@milestone-is-a-directory` and `design@core@milestone-entries-are-directories` both state that a milestone is a directory whose `README.md` is its document, with different reasons (navigation links; citation as `milestone@plans@<id>`). Two heads on one layout is a question of `primer@where-knowledge-goes`, outside the rules this axis applies to one head; named for the session to route (an issue entry, case 3, if it judges the overlap a second home)." |
| agent-skills-1 | "The section opening of "How the installed text changes" names designing-together's record on its branch next, which moves; the figures it backs (34 edits, 10 defects) are dated 2026-08-03 there. Noted in drafts 21 and 58 for the calibration; no outcome proposed." |
| agent-skills-2 | "The design skill (content/skills/design/SKILL.md:147) and the planning skill (content/skills/planning/SKILL.md:423) also ask for `primer@design-heads` to be read again, which `design@agent-skills@primer-reread-before-recording` ("so one instruction reaches every write") does not account for: a restatement or a member outside this axis; the session routes it (likely an issue entry or a reread by the agentic-workflow axis)." |
| agent-skills-2 | "Heads false of the installed text at the commit audited: `design@agent-skills@audit-axis-runs-a-workflow` ("The design-record axis has none yet"), `design@agent-skills@design-record-axis` ("A group holds one Component's entries", "The axis has no saved workflow yet"), `design@agent-skills@audit-method` ("an axis with none runs the method's points by hand"), and the single-axis accounts of `design@agent-skills@one-run-pass`, `design@agent-skills@verifier-in-the-run` and `design@agent-skills@committed-audit-report`. Already recorded: the spec design-record-workflow on this branch lists each rewrite in its harvest table; no new issue." |
| agent-skills-3 | "none" |

Of the verifiers: core-1, core-2 and agent-skills-1 report "none"; agent-skills-2 reports "None
beyond what the drafter reported"; packed-1, core-3 and agent-skills-3 have no such section. The
calibration reports one item: "A working file was first written one level above this directory, at
drpass1/calibration-dra.md. It was moved into this directory as design-record-audit-spec.md.
Outcome: fixed, nothing remains outside this directory." The checks of packed-1, core-1 and
agent-skills-1 report "none"; the check of agent-skills-2 has no such section.

### 6.5 Steps that failed or returned nothing

- No step failed. Every one of the 7 groups returned a return.md naming each of its entries; the
  calibration, the 7 verdicts files, the 6 checks and the cross-check are present.
- core-3 has no check, by the script's rule: its verifier verified no head-creating finding.
- preflight/reports.txt is empty (0 bytes): the `history` range added no report file under
  docs/audit-reports/, so the verifiers read earlier passes from commit messages (preflight/history.txt)
  and `git show` of deleted files.
- Two verdicts files omit parts the brief's shape implies: core-3 and agent-skills-3 have no counts
  section (counted here from their rows), and packed-1, core-3 and agent-skills-3 have no "Met outside
  the task" section.
- agent-skills-1's draft for the sample entry versioning-policy (56), core-1's sample drafts 59 to 63
  and the other groups' sample copies were not verified, by design: only the draft of record is
  judged. Their verdicts enter only through the calibration.

