# 2026-10-13-knowledge-architect-agent-text-admission-klarch-workflow

- **Received file**: 2026-10-13-knowledge-architect-agent-text-admission-klarch-workflow.md, the
  workflow file of this repository's retrospective of the session that set what enters
  agent-facing text.
- **Version used**: the checker built from this repository, main at `v0.6.0-46-ge619a1e`, branch
  `agent-text-admission`, with uncommitted changes at most moments between commits.
- **Analysed against**: main at `v0.6.0-75-g5fca230`, which holds the session's merged branch.
- **Standing entries that bear on it**: from the one search over the 83 issues and 58 tripwires:
  - `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`, W2;
  - `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`, W3;
  - `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, C1;
  - `tripwire@agent-skills@attribution-loses-the-scope`, judged in
    2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow with this file's evidence;
  - `tripwire@agent-skills@owner-intent-stripped`, `tripwire@agent-skills@pointer-not-followed`,
    `tripwire@core@fix-refusal-routed-around`, `tripwire@core@staged-pass-fails-the-checkout`,
    `tripwire@agent-skills@agent-text-written-without-the-standard` and
    `tripwire@agent-skills@real-use-refusal-overruled`, judged together in the last section.

## W1. Whether the primer is one text with the root `CLAUDE.md` that imports it

- **a. What it says.** The standard first said "Across texts, `primer@where-knowledge-goes`
  decides", and two census readers asked whether a root sentence restating the primer is a
  duplicate inside one text. The owner ruled: one text, and the primer's copy stays. The session
  first wrote the rule for any imported text, wider than the ruling; the decision-record reviewer
  caught it. The file says it was fixed in the merged work, and proposes nothing.
- **b. Whether it still applies.** No. The wording entered in the commit "Step 1: one standard in
  the agent-configuration skill decides what enters agent-facing text", was widened in "The primer
  and the root CLAUDE.md are one text: the root drops what the primer states", and narrowed to the
  pair in "[review] Repairs of the review of the one-text ruling: the ruling narrowed to the primer,
  the root's last twins removed". `path@agent-skills@content/skills/agent-configuration/SKILL.md`:44-48
  now reads: "The primer and the root `CLAUDE.md` that imports it are one text, since both are
  loaded together, and the primer's copy is the one that stays. Across other texts,
  `primer@where-knowledge-goes` decides."
- **c. Whether it is already recorded.** The head `design@agent-skills@need-test` carries the
  owner's choice. No issue is needed.
- **d. Its kind.** Unclear text, repaired in the session.
- **e. Its scope.** The agent-skills Component; done.
- **f. A better fix.** None needed.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change, already repaired on main.
- **Outcome**: approved.

## W2. Whether a census of a project's real text under a planned instruction is real use

- **a. What it says.** The head that refuses synthetic evidence refuses a run built to observe how
  an agent follows the workflow's instructions. The census applied a proposed test to real text, by
  read-only agents. A review read it as a refused run; the owner read it as real use, holding the
  point open. Its comparison counted a reader's silence as disagreement, which the owner judged the
  kind of reason why such evidence misleads. The file proposes that the owner rules on the issue,
  and no text change before that.
- **b. Whether it still applies.** Yes. The head `design@agent-skills@synthetic-evidence-not-built`
  names no census.
- **c. Whether it is already recorded.** Fully:
  `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` holds
  both readings, the bias and the owner's argument, and closes as the file proposes.
- **d. Its kind.** Unclear text, already an open question.
- **e. Its scope.** The agent-skills Component and
  `goal@knowledge-architect@the-workflow-improves-through-real-use`, which is the owner's.
- **f. A better fix.** None to weigh here: the issue is where it is weighed.
- **g. Its route.** Nothing now; the issue's own route.
- **h. The proposal.** No change: already recorded.
- **Default**: no change, recorded in the issue named in c.
- **Outcome**: approved.

## W3. The primer and its head name a guard's issue under two conditions

- **a. What it says.** The primer names the issue in the comment of a guard or a test "that exists
  because of an issue"; the head names it for one "that pins behaviour an open entry describes". The
  file proposes a ruling, then both texts brought to it in one change.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/PRIMER.md`:183-184, and the table
  of `design@agent-skills@a-reference-claims-a-revisit`.
- **c. Whether it is already recorded.** Fully:
  `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`, with the same
  two quotations and the same closing condition.
- **d. Its kind.** Unclear text, already an open question.
- **e. Its scope.** The agent-skills Component.
- **f. A better fix.** None to weigh here.
- **g. Its route.** Nothing now; the issue's own route.
- **h. The proposal.** No change: already recorded.
- **Default**: no change, recorded in the issue named in c.
- **Outcome**: approved.

## C1. A staged check compares the installed files with shipped text that is not staged

- **a. What it says.** In this repository, `check --staged` judged the staged installed copies
  against the shipped text the binary was built from, the working tree's. A commit that staged one
  source and install pair failed the staged check although it was consistent. The file proposes that
  the staged check read both members of a pair from the index.
- **b. Whether it still applies.** Yes. `path@core@src/cli/history.rs`:558-559 takes the shipped set
  from `crate::agents::shipped`, the compiled-in list, which `path@agent-skills@build.rs` builds from
  the working tree.
- **c. Whether it is already recorded.** Fully:
  `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, with the same
  reproduction and the file's proposal as its closing condition. No rejected alternative of the core
  is that fix.
- **d. Its kind.** A defect of the checker, already recorded.
- **e. Its scope.** The core Component.
- **f. A better fix.** None to weigh here.
- **g. Its route.** Nothing now; the issue's own route, under `skill@klarch-development`.
- **h. The proposal.** No change: already recorded.
- **Default**: no change, recorded in the issue named in c.
- **Outcome**: approved.

## The attribution items of the standing answers

The second and third bullets of this file's standing answers, verbatim quotations of the owner
written into heads at the harvest and two attributions widened, are analysed with the cluster of
attributions in 2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow, section "The
attribution cluster", and the tripwire they come near in the section after it.

- **Default**: as that section's outcome.
- **Outcome**: approved.

## The census agents counted silence as disagreement

- **a. What it says.** The census agents, briefed by the session, compared their lists in a way
  that counted a reader's silence as disagreement. The file says no primer rule covers how evidence
  is counted, and lists it because the owner corrected it.
- **b. Whether it still applies.** No instruction is blamed.
- **c. Whether it is already recorded.** Yes, in
  `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`, which
  holds the owner's argument on that bias.
- **d. Its kind.** An observation of the session's own method, recorded.
- **e.–g.** None.
- **h. The proposal.** No change.
- **Default**: no change, recorded in the issue on the census.
- **Outcome**: approved.

## Five reviewers dispatched without the instruction to read the changed root `CLAUDE.md` from disk

- **a. What it says.** `skill@knowledge-architect-review@review-invariants` says that once a file
  changes on disk, the dispatcher tells each subagent to read it from disk. The session changed the
  root `CLAUDE.md` and dispatched five reviewers without it. Three reported that their copy matched
  no commit and read the tree on their own. No defect followed. The file calls the rule clear.
- **b. Whether it still applies.** The rule stands, `path@agent-skills@content/skills/review/SKILL.md`:89-95.
- **c. Whether it is already recorded.** The harness fact behind it is recorded in
  `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`: the root `CLAUDE.md` delivered to a
  subagent was older than the disk. The lapse is not recorded.
- **d. Its kind.** A lapse, one instance, with no consequence.
- **e.–g.** None.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of a clear rule.
- **Outcome**: approved.

## The tripwires the file comes near and does not meet

- `tripwire@agent-skills@owner-intent-stripped`: the commits that converted quotations kept the
  owner's ground, attributed; its exception covers "A quotation converted to an attribution that
  states the same ground".
- `tripwire@agent-skills@pointer-not-followed`: the directives the session broke, the pipe rule
  and the push after `;`, are stated in full in the root `CLAUDE.md`, at
  `instructions@verify-mechanically` and `instructions@git-workflow`; neither was replaced by a
  pointer.
- `tripwire@core@fix-refusal-routed-around`: this is its re-entry, an intake. The files report no
  unstaging after a refusal of `check --fix`.
- `tripwire@core@staged-pass-fails-the-checkout`: C1 is its reverse, a staged failure that a
  checkout passes, as the C1 issue says.
- `tripwire@agent-skills@agent-text-written-without-the-standard`: the files report no addition
  that fails the need test or the real-use rule.
- `tripwire@agent-skills@real-use-refusal-overruled`: W2 is no overruled refusal of an addition;
  the owner held the point open.

- **Default**: none fired.
- **Outcome**: approved.
