---
name: klarch-retrospective-intake
description: MUST use when the owner hands this session one or more retrospective files to act on — a `-klarch-workflow.md` file, or a `-workflow.md` file written before that name, from this repository or from a project that uses the workflow, or this repository's own project file — or asks what to do with the findings of a retrospective already run. Covers checking each finding against the tree as it stands, against the registers and the history, against the goals and the design, looking for a better fix than the one proposed, writing the analysis to a file, and offering the owner three outcomes per finding. Not for running a retrospective, nor for carrying out an outcome.
---

# Retrospective intake

Scope: the analysis of the findings a retrospective sends to this repository, up to the owner's
ruling on each. A retrospective is run by `knowledge-architect-retrospective`, in this repository
or in a project that uses the workflow, and its files land outside the project, in a directory the
owner names. A later session receives them, and this skill is that session's procedure.

Not covered here:
- **running a retrospective**: `knowledge-architect-retrospective`;
- **carrying out an outcome**: an issue is opened under `knowledge-architect-issue-tracking`; a
  finding handled now is handled under the skill §4 names for it.

**This skill decides nothing.** It establishes the facts about each finding, proposes an action
with its argument, and the owner rules, per `goal@knowledge-architect@the-owner-decides`.

## 1. What it takes

- **In scope: a retrospective's workflow file**, whatever project wrote it. It is named
  `<date>-<project>-<subject>-klarch-workflow.md`, or `<date>-<project>-workflow.md` before that
  name. It holds the findings on the installed skills and agents (W), on the checker (C), and the
  answers to the standing questions.
- **In scope: this repository's own project file**, `<date>-knowledge-architect-<subject>.md`.
  Its findings (P) are on this repository's own instructions, and the same checks apply to them.
- **For grounding only: another project's file** of the same retrospective, when it is present. It
  can explain a workflow finding: what the project's text said, what the session did. Its own
  findings are that project's, and are fixed in its text. **Nothing of it enters the analysis
  beyond what the workflow file already holds**: the workflow file is written to be publishable,
  the project's file is not, and the analysis is committed to a public repository.

The findings of this repository's two files are handled here, never as an issue on GitHub, per
`design@knowledge-architect@retrospective-findings-stay-here`. A consumer's workflow file
arrives the same way once the owner hands it over.

A finding is cited by its id and its file's stem, as "W3 of
2026-10-07-thaum-mock-reduction-workflow", since several retrospectives share a date.

## 2. Ground before judging any finding

1. **Read the workflow file whole**, and the project's file if present.
2. **Note the version the retrospective used**, from its Version section, and the main commit you
   analyse against: `git describe --tags origin/main`. A finding against an older version may
   already be repaired.
3. **Search the standing entries with the received file as the work.** Dispatch
   `knowledge-architect-standing-entry-searcher`:
   - count the rows of `cargo klarch issues` and of `cargo klarch tripwires`, the header rows
     excluded;
   - send ceil(count / 60) agents, all in parallel, on consecutive groups whose sizes differ by at
     most one;
   - give each the work (the file's path and its subject), the seeds (the design heads and goals
     its findings name, or none), and its group's first and last positions.

   Read every entry returned whole with `cargo klarch show <ref>`, never from its reason line. The
   search finds what a grep per finding does not: a finding already recorded, a deferred issue
   whose trigger the file meets, a tripwire whose firing evidence the file carries, a design
   issue whose re-entry it meets.
4. **Cluster the findings.** One defect is often reported in several files, or as two findings of
   one file. A cluster is analysed once, and cites every id in it. That a finding recurs across
   sessions is evidence: say how many times.

## 3. What to establish for each finding or cluster

Each item below is answered with its evidence. The order is a suggestion; the content is not.

- **a. What it says.** What happened in the session, which text it blames, quoted, and the fix it
  proposes.
- **b. Whether it still applies.** Read the blamed text on main as it stands, and quote it with
  its `file:line`. The report is a claim about the text at its version, so it is checked like any
  claim about the code. A finding may be already repaired: name the commit, found with
  `git log -S '<phrase>'` or `git log --grep`. The report says "repaired in the session" sometimes;
  verify it too.
- **c. Whether it is already recorded.** An issue or a tripwire, from the search of §2. A
  rejected alternative: read the rejected alternatives of the Component that owns the blamed text.
  A proposed fix that already lost is not proposed again, unless the finding brings a fact the
  recorded reason does not cover.
- **d. Its kind.**
  - **a defect of the text**: unclear (two readings), missing (the session needed an instruction
    that does not exist), or wrong (following it produced a defect or a correction);
  - **a lapse of the session**: the instruction exists, is clear, and was not followed. Adding
    text does not repair a lapse, per `design@agent-skills@capabilities-not-structure`. The
    exception is a lapse that recurs at one point of delivery: the instruction is read at one
    moment and needed at another. Then the fix moves where it is delivered, and the recurrence is
    its evidence;
  - **about the owner's behaviour**: judged against the expectation set of the skill concerned, in
    §5 of the installed retrospective skill. Behaviour the set says the skill assumes otherwise is
    outside that skill's scope, per `design@agent-skills@expectation-set-bounds-scope`. A skill
    with no set has no such test yet, per
    `issue@agent-skills@expectation-sets-for-the-installed-skills`. **A finding that two installed
    instructions leave no move satisfying both is always in scope**, and is never closed by citing
    an expectation set, per `tripwire@agent-skills@expectation-set-closes-a-contradiction`;
  - **an observation**: a use that went well, or a behaviour not reproduced. It needs no action.
- **e. Its scope.** The goal it serves or threatens, from the goals homes. The Component that owns
  the fix: agent-skills for the installed text, core for the checker, the root or the agent-config
  location for this repository's own configuration. Whether the fix reverses a recorded decision,
  per §1 of `knowledge-architect-decision-recording`: if it does, the fix is a reversal and goes
  through that skill.
- **f. A better fix.** Name the nearest rival to the fix the report proposes, and the fact that
  decides between them. Rivals worth looking for:
  - the same defect fixed at another site: the checker rather than the text, or the reverse;
  - one fix for a whole cluster;
  - a fix where the instruction is delivered, rather than more instruction;
  - no change.

  A fix to an installed skill passes the four tests of the agent-skills `CLAUDE.md`, "Editing an
  installed skill or agent": scope, necessity, kind, built intent. An addition needs an observation
  from a real session, which the finding is, and a one-sentence mechanism, which the analysis
  writes.
- **g. Its route.** What handling it needs, which says what "now" would mean:
  - a text edit whose design is settled: under `knowledge-architect-agent-configuration`, and for
    the installed text the agent-skills `CLAUDE.md`;
  - a decision, or one whose design is not settled: `knowledge-architect-design`;
  - a change to the Rust source: `klarch-development`;
  - an issue only, when the work is not to be done in this session;
  - nothing.
- **h. The proposal.** The action, its argument, and the default outcome (§5).

**Every verdict carries its evidence**: a quotation with its `file:line`, a command and its
output, a commit. A verdict without evidence is labelled an assumption.

## 4. Subagents

The checks a to c read text, and fresh subagents may run them, one per file or per cluster. Brief
each with the findings, the files to read, and a scratch directory of its own, never shared. Each
returns, per finding: the quotation, its `file:line`, and its verdict. The session itself
establishes d to h, and re-checks every verdict a proposal rests on. No subagent replays the
reported session or builds a scenario of agent behaviour, per
`issue@agent-skills@synthetic-evidence-about-the-workflow-is-undecided`.

## 5. The analysis file and the owner's ruling

Write the analysis in `path@knowledge-architect@docs/retrospective-reports/`, as `<stem>.md`,
where `<stem>` is the received file's stem. The directory's README says what it holds and when a file leaves it.

- A head: the received file's name, the version it used, the main commit analysed against, and the
  standing entries the search returned that bear on it.
- **One section per finding or cluster**, headed by its ids and a short title, with a bullet per
  item of §3, in prose. No table: the content of each item is too long to read in a cell.
- **Name nothing of the other project in detail**: none of its paths, none of its design heads,
  none of its internals. Say it in words where the finding needs it, as "the project's design
  home". This repository's paths and entries are written in the checked forms, since the file is
  checked like any other.

Then give the owner the file's full path, and in the conversation one line per finding or
cluster: its ids, the proposal in a few words, and the default outcome. The owner rules in words,
such as "now: W1, W5; issue: C3; no change as proposed for the rest". **Three outcomes**:

- **handle now**: the finding is handled in this session, under the skill its route names. It
  needs no issue entry, and the proposal does not suggest one;
- **open an issue**: name its anchor and its kind;
- **no change, with the reason**: a lapse, already repaired, out of scope, or an observation.

A question tool is not the way to collect the ruling: it shows a label, not the evidence and the
rival fix the ruling depends on.

## 6. After the ruling

Each section ends with an **Outcome** line: the owner's choice and the owner's words, verbatim.
The analysis is then committed on a branch, under the root `CLAUDE.md` `## Git` rules. Its life
after that:

1. a commit adds the analysis, with its Outcome lines;
2. the next commit opens the issues it ruled;
3. the findings handled now are handled, in as many commits or branches as their routes need;
4. the commit that carries out its last outcome deletes it, and its message names the analysis by
   its path. History keeps it: `git log -- docs/retrospective-reports/` lists every analysis.

A finding handled now can turn out to need a design discussion and a plan. Its Outcome line is then
rewritten, on the owner's word, to name the plan document or the issue that now carries it, so the
analysis can still leave.
