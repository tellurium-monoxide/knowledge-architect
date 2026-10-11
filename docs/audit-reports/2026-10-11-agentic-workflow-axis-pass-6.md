# Pass 6 of the agentic-workflow axis: report and owner list

The report of the sixth pass of the agentic-workflow axis of
`skill@knowledge-architect-project-audit`, run on this repository in the branch of
`spec@plans@agentic-workflow-axis`, as the session's verification left it, with its owner list in
section 5. Its record is `skill@knowledge-architect-project-audit@record-of-an-audit`. The run's
working files are outside the tree, in the session's scratch directory, whose paths the sections
below keep.

**Commits of this branch are named by a tag**, since a rebase merge gives them new SHAs; a commit
already on the main branch keeps its SHA. Each tag is the commit whose subject follows it:

| tag | commit |
| --- | --- |
| [S9] | "The project audit counts convergence on old text, checks each repair against the record, and commits each pass's report file" |
| [D32-35] | "[plan] The owner rules D32 to D35: the goal prevails over leaving a finding to judgement" |
| [P5] | "[audit] Pass 5 of the agentic-workflow axis: one run of the saved workflow, verified, ruled and applied by cause" |
| [S7] | "The agentic-workflow axis runs a pass as one run of its saved workflow, with a lens comparing review and audit standards" |
| [P4] | "[audit] Pass 4 of the agentic-workflow axis, run through the saved workflow: 36 findings confirmed, 23 causes repaired" |
| [P2-3R] | "[audit] Passes 2 and 3 of the agentic-workflow axis repaired by cause: 14 causes, 16 instructions added, 58 removed" |
| [P3] | "[audit] Pass 3 of the agentic-workflow axis stops the run: the count did not fall, the repairs move to a method by cause" |
| [P2] | "[audit] Pass 2 of the agentic-workflow axis: 70 findings confirmed, 37 applied, 30 ruled by the owner, 3 issues" |
| [P1] | "[audit] Pass 1 of the agentic-workflow axis: 54 findings confirmed, 26 applied, 27 ruled by the owner, 1 issue" |
| [AX] | "The project audit gains the agentic-workflow axis, carried by the workflow auditor" |
| [PR1] | "[review] The four plan reviews of the agentic-workflow axis spec: gaps applied in place, seven defaults put to the owner" |

The session's verification read the report whole, every proposal through its summary and check,
every finding marked bearing on a ruling or plausible against its verifier's line, and the five
refuted drafts' reasons. It rejected no finding. It re-classified the findings on new text by the
words that carry each defect, as F108 gives. Line numbers are those of the worktrees at [S9].

---

## 1. Header

### Commit audited

`[S9]` "The project audit counts convergence on old text, checks each repair against the
record, and commits each pass's report file". The previous pass audited `[S7]` "The
agentic-workflow axis runs a pass as one run of its saved workflow, with a lens comparing review
and audit standards". The worktrees are under `path@agent-config@worktrees/pass6/`: one per auditor label and
`reading`.

### The agents of each step

| step | agents | agent type (script line) | files |
| --- | --- | --- | --- |
| preflight | 1 | general-purpose (96) | the 18 worktrees; `preflight/history.txt` (1209 lines), `preflight/issues.txt`, `preflight/tripwires.txt`, `preflight/reports.txt` (it holds `path@knowledge-architect@docs/audit-reports/README.md` only); the session's kept list `kept.md` copied into each auditor's directory |
| drafts | 17 auditors: L1-a, L1-b, L1-c; L2-a, L2-b, L2-c, L2-d; L3-a, L3-b; L4-a, L4-b; L5-a; L6-a, L6-b; L7-a, L7-b; L8-a | `agent@knowledge-architect-workflow-auditor` (142, 149) | `<label>/return.md` and the drafts |
| verify | 17 verifiers, one per auditor, in its auditor's worktree | `agent@knowledge-architect-workflow-auditor` (142, 150) | `verify/<label>/verdicts.md` |
| dedup | 1 | general-purpose (163) | `dedup/confirmed.md`, with its tool `dedup/tools/classify.sh` |
| cluster | 2 | general-purpose (182) | `cluster-1/grouping.md` (G1 to G19), `cluster-2/grouping.md` (G1 to G16) |
| merge | 1 | general-purpose (195) | `merge/causes.md` (K1 to K19) |
| repairs | 19, one per cause (`repairsPerCause` at its default, 1) | general-purpose (245) | `repairs/K1/proposal.md` to `repairs/K19/proposal.md` |
| check | 19, one per proposal | `agent@knowledge-architect-workflow-auditor` (247) | `repairs/K<n>/check.md` |
| cross-check | 1 | general-purpose (254) | `cross-check/cross-check.md` |
| report | 1 (this file) | general-purpose (270) | `report/report.md` |

L2's four walkers took the four groups of the script (lines 24-27): L2-a "design, planning,
decision recording and issue tracking", L2-b "development, review and merge", L2-c "audit and
retrospective", L2-d "setup, pin move, goal setting and agent configuration". No group was left
unwalked.

### Counts

| count | value | source |
| --- | --- | --- |
| drafts | 61 | the 17 returns: L1-a 2, L1-b 4, L1-c 3, L2-a 6, L2-b 3, L2-c 4, L2-d 3, L3-a 7, L3-b 4, L4-a 2, L4-b 3, L5-a 9, L6-a 1, L6-b 2, L7-a 2, L7-b 3, L8-a 3 |
| verdict lines | 64 | three drafts tagged I were split into a W line and a P line: L5-a/d6, L6-b/draft-02, L7-a/D1 |
| verified | 54 lines (53 drafts; L7-a/D1 verified on both sides) | the 17 verdicts files |
| plausible | 5 lines, 5 drafts: L2-a/draft-05, L2-a/draft-06, L2-b/d2, L2-d/draft-1, L4-b/d3 | the verdicts files |
| refuted | 5 lines: 3 drafts whole (L1-b/d3, L2-a/draft-02, L3-a/06) and 2 sides (the W side of L5-a/d6, the P side of L6-b/draft-02) | the verdicts files |
| bearing on a ruling | 7: L1-b/d2 (F97), L2-a/draft-03 (F58), L2-b/d1 (F44), L2-c/d4 (D35), L2-d/draft-1 (F94), L7-b/draft-01 (F23, F69), L8-a/draft-01 (F99, D3) | the verdicts files |
| bearing on a tripwire | 0 | the verdicts files; the cross-check, section 4, states the same |
| known | 2: L5-a/d8 (known, `issue@core@references-are-read-in-markdown-and-rust-only`); L2-d/draft-2 (known in part, `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`) | the verdicts files |
| kept | 1: L7-b/draft-01 (kept list item 5) | the verdicts files |
| after dedup | 38 findings, C1 to C38, from 59 unrefuted drafts and draft sides | `dedup/confirmed.md`: "Counts: 38 findings from 59 unrefuted drafts and draft sides (58 drafts, L7-a/D1 counted once per side); 18 on new text, 20 on old text." |
| tags of the 38 | W 31; P 7 (C10, C12, C14, C26, C27, C28, C32); none left as I, each I draft split into its sides | the "tag" column of `dedup/confirmed.md` |

Ten verdicts files end with a count line; seven (L1-b, L1-c, L2-d, L4-a, L4-b, L6-a, L6-b) have
none, so every figure above was counted from the verdict lines themselves.

### For this re-check: the confirmed findings and the "severe" ones, on old text and on new text

No text of the corpus defines "severe" (that is finding C2). The figures below use the
definition of the spec, line 142, "**severe**: a confirmed finding whose outcome is the owner list,
whatever its class", with the outcomes the cross-check proposes, which the session has not yet
decided. They also give the counts the section "record-of-an-audit" asks for, "the count of violations left
on old text and on new text, as point 8 defines them, and of those on old text that went on the
owner list". Computed by this report from `dedup/confirmed.md` and `cross-check/cross-check.md`
section 2; the session re-takes them once it decides the outcomes.

| count | old text | new text | total |
| --- | --- | --- | --- |
| confirmed findings | 20: C6, C7, C8, C10, C11, C12, C13, C15, C22, C23, C24, C27, C28, C29, C31, C32, C33, C34, C37, C38 | 18: C1, C2, C3, C4, C5, C9, C14, C16, C17, C18, C19, C20, C21, C25, C26, C30, C35, C36 | 38 |
| violations left, by point 8 ("not on the kept list, and that no issue entry records"): C34 (kept) and C28 (known) out | 18 | 18, or 17 if C16, "known in part", is taken as recorded | 36 or 35 |
| violations left by the spec's own term, line 140, "a confirmed finding of a re-check that is not on the kept list" | 19 | 18 | 37 |
| severe, as the cross-check proposes the outcomes | 4: C6 (K13), C7 (K11), C37 (K19, default an issue entry), C38 (K15) | 12: C1, C3, C4, C14, C16, C17, C18, C19, C21, C26, C30, C36 | 16 |

**What these figures mean for convergence, for the session to judge.** The spec's AC4 (lines
2228-2232) reads, from pass 6 on, "the violations left on old text and the severe ones among
them". Its baseline for pass 6 (spec, lines 2300-2306): "Pass 6's old-text counts are compared
with 16 and 8." On this run's classification, the old-text count is 18 (19 by the spec's term),
up from 16, and the severe ones among them are 4, down from 8. Point 8 of the section "audit-method" (audit
SKILL 110-113): "The count on old text is expected to fall from one pass to the next; a pass where
it does not goes to the owner as a question, with the counts, and starts no new pass on its own."
Three facts weigh on the comparison:

- The baseline was classified "by the newest commit that changed each defect-carrying sentence's
  words" (spec 2300-2302); this run's dedup classified by the whole blamed line, and states that
  "no finding was re-tested under" the reading by the defect's words (`dedup/confirmed.md`). K5's
  check notes that "the comparison with 16 and 8 is not taken by one method".
- If the session adopts K5's repair (old and new decided by the words carrying the defect), the 18
  findings on new text are re-classified first; that can only move findings from new to old.
- Three of the 20 old-text findings are plausible predicted gaps (C8, C11, C15), which point 8
  counts as confirmed.

### The refuted drafts, each with its reason

From `verify/L1-b/verdicts.md`, line 9:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L1-b/d3-old-and-new-text-defined-two-ways.md | refuted | none | Quotations match (SKILL.md 107-110, 294-297, 148-150), but no single move fails: the method says "an axis's section adjusts it where its corpus needs" (SKILL.md:56-57) and point 8 itself says "The axis's section says how its findings are classified" (SKILL.md:110), so for this axis the classification of SKILL.md:294-297 governs, and the count "as point 8 defines them" follows that deferral. The copy case is a consequence of the test the owner let stand ("The rest of 'The AC4 response' stands: the convergence count on old text", commit [D32-35]), not a second definition in force. |

From `verify/L2-a/verdicts.md`, line 8:

> | draft-02-a-bug-hunt-observation-owes-a-tripwire-its-entry-has-no-place-for.md | refuted | none | Already ruled. F41 in [P2] ("a parked thread's tripwire is a tripwire or a deferred trigger", approved "All defaults approved for F28 to F57.") was written into the same `parked` bullet that names the bug-hunt observation (`git show [P2]:crates/agent-skills/content/skills/design/SKILL.md`, lines 351-361). F91 in [P4] moved it to ruled-tripwires with "a parked thread's included". Act 1 also goes against the plain words "Every parked thread names its tripwire ... missing either one is open", so the two readings fail. Whether the `design` entry also carries the trigger is left to judgement. |

From `verify/L3-a/verdicts.md`, line 12:

> | 06-sent-upstream-finding-as-violation-left.md | refuted | none | The class fails. Point 8's words allow one reading only: a finding that a reading confirms, that is not on the kept list and that no issue entry records "is a violation left". Reading 2 adds an exception for findings already sent upstream that no word of the text admits. The consequence under the only reading (an old-text count that no repair lowers, and the pass then going to the owner as a question) can be followed as written. It is a proposal for a new rule, at most a design question or a predicted gap, not two readings. |

From `verify/L5-a/verdicts.md`, line 16:

> | d6-intake-kinds-restate-the-retrospective-s-without-room-to-judge.md (W side) | refuted | none | the installed home, retrospective SKILL.md:41-45, states the exclusion and points to `primer@room-to-judge`; no installed text needs a change, as the draft itself says. |

From `verify/L6-b/verdicts.md`, line 7:

> | draft-02-audit-report-file-carries-branch-shas.md | P | refuted | none | The P side holds no finding: the root CLAUDE.md lines 608-611 state the reading meant ("neither a commit message nor a document cites the SHA of a commit of its own branch"), and `refuse-branch-shas = true` makes `cargo klarch commits` refuse the other reading. Outcome nothing, as the draft itself proposes. |

### Returns that say a corpus file was not read whole

None. Each of the 17 returns lists the same 27 corpus files under "Corpus files read whole" (the
root CLAUDE.md, the primer, 13 skills, 11 agents, the script), compared by a `diff` of the lists.
What the returns add beside the corpus:

- `L2-b/return.md` annotates its root CLAUDE.md and primer: "(read in the worktree; the session's
  copy predated [S9] and lacked `ls docs/audit-reports/`)" and "(read in the worktree; the
  session's copy lacked the docs/audit-reports/ row)".
- `L3-b/return.md`: "(.claude/open-issues/ is a register, outside the corpus, and was not read.)"
- `L7-b/return.md`: "crates/core/docs/design.md read by head (cargo klarch show), not whole." The
  root the section "repository-skills" says the L7 auditors "also read the checker's design home,
  `path@core@docs/design.md`, beside its README". `L7-a/return.md` read "the parts of [...]
  crates/core/docs/design.md and crates/core/src the drafts cite".
- A verifier, not an auditor, `verify/L2-a/verdicts.md`: "The project-audit skill was read in
  part (its outcomes and record-of-an-audit sections). No other file of the corpus was read whole."

---

## 2. The figures

- **Findings**: 38 (C1 to C38), 18 on new text and 20 on old text. **Causes**: 19 (K1 to K19),
  each finding in exactly one (`merge/causes.md`, "Result").
- **Causes by kind**: two homes for one moment 7 (K3, K4, K6, K9, K10, K13, K19); a structure the
  work did not need 2 (K1, K12); a restatement that drifted from its home 3 (K2, K16, K17); none 7
  (K5, K7, K8, K11, K14, K15, K18); a rule written for one interaction 0.
- **Instructions added and removed, per proposal**, as each proposal counts them, with the
  stricter or alternative count the proposal or its check states:

| cause | added | removed | other count stated |
| --- | --- | --- | --- |
| K1 | 3 | 0 | none; all three are the prompt of a new setup agent |
| K2 | 0 | 0 | none |
| K3 | 0 | 1 | none |
| K4 | 2 | 2 | none |
| K5 | 0 | 0 | none |
| K6 | 1 | 2 | "0 added and 1 removed" if Edit 3 is read as a rewording; net -1 either way |
| K7 | 0 | 0 | none |
| K8 | 0 | 0 | none |
| K9 | 0 | 0 | none |
| K10 | 0 | 0 | none |
| K11 | 0 | 0 | none |
| K12 | 0 | 0 | none (an issue entry only) |
| K13 | 0 | 0 | none |
| K14 | 0 | 0 | none |
| K15 | 0 | 0 | none |
| K16 | 0 | 0 | "A stricter count reads [E5] as 1 removed and 1 added, net 0 either way" |
| K17 | 0 | 0 | none |
| K18 | 0 | 0 | none (an issue entry only) |
| K19 | 1 | 0 | 2 added if E6 to E8 (C37's alternative) are applied |
| **sum** | **7** | **5** | **net +2; net +3 with K19's E6 to E8** |

The cross-check, section 2: "Per the section "repair-standard", "When the repairs applied together add more
instructions than they remove, the owner is told so first, with the net figure, and rules on that
growth before any of them is applied". So the owner list opens with the net figure, and **no edit
of the pass, the applied row included, is applied before that ruling**."

The corpus's word count before and after the pass, which the axis asks of the pass's commit
message (audit SKILL 312-314), is in no proposal. K14 gives its bullet only: "58 before, 69 after
(`wc -w`)". The session takes `wc -w` after it applies the edits.

---

## 3. The causes

Each cause: its kind and mechanism, quoted from `merge/causes.md`; each finding with the dedup's
row, its verifier's line and the opening of its draft's evidence; the repair as the sentences each
edit changes, before and after; the instructions it adds and removes; the nearest rival; its
check; the outcome the cross-check proposes, with the default; and, where a finding is marked
bears-on-ruling, the cross-check's quotation of the ruling and the premise that may be false. No
finding is marked bears-on-tripwire.

Each finding below gives the dedup's row, its verifier's line verbatim, and the opening of its
draft's evidence verbatim, capped at a fixed number of lines with the rest named by line. Each
repair gives the sentences each edit changes, quoted before and after; the full texts are in the
proposal at the lines named. In this repository every edit to installed text is made at its source
under `path@agent-skills@content/` and installed again (`instructions@repository-skills`, kept
list item 1); line numbers are the installed copies' unless "source" is said.

### K1. One worktree for every check agent

From `merge/causes.md`, K1:

> - **Kind**: a structure the work did not need.
> - **Mechanism**: the preflight creates its worktrees from a list fixed before the run (one per
>   auditor label, plus `reading`), so the check step, whose agents are one per cause and exist only
>   after the merge, is given the shared `reading` worktree, while each check agent runs
>   `cargo klarch show` there concurrently and review-invariants requires a distinct worktree for
>   each subagent.

**Findings**

#### C1 (W, new text)

- **Draft kept**: `L1-a/d1-check-agents-share-one-worktree.md`. **Merged into it**: L1-b/d1-check-agents-share-one-worktree.md, L1-c/D1-check-agents-share-one-worktree.md, L2-c/d1-check-agents-share-the-reading-worktree.md, L4-a/draft-02-check-agents-share-the-reading-worktree.md, L4-b/d2-check-agents-share-the-reading-worktree.md, L7-b/draft-02-check-agents-share-the-reading-worktree.md.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the check agents run concurrently, each runs `cargo klarch show`, all in the one `reading` worktree, against review-invariants' distinct worktree per subagent".
- **Class decided by** (dedup): workflow .js 234 249: lines 237 and 243-248 blamed [S9], phrase absent at [S7].

**The verifier**, its line verbatim:

> | d1-check-agents-share-one-worktree.md | verified | none | Every quotation matches the worktree (review SKILL.md 105-116, auditor 3, 24-25, 196-198, script 77, 99-100, 234-249, audit SKILL.md 65-67, 288-290); the check stage of `pipeline` runs one auditor per proposal concurrently, each told to run `cargo klarch show` in the one `reading` worktree, against "distinct for each subagent"; a hand dispatch "with the script's prompts" cannot also give each "a worktree of its own". No issue entry, no kept item, no tripwire; the spec's "reading worktree, for the steps that only read" (spec lines 1752-1754, 1824-1825) predates the check step and rules nothing on its worktree. One inaccuracy that changes nothing: the draft quotes the setup skill's alias for an adopting project; this repository's alias, .cargo/config.toml, is `run -q --release -p knowledge-architect --`, which also builds in the worktree. |

**The evidence**, verbatim from `L1-a/d1-check-agents-share-one-worktree.md`, from its first section on:

> **Evidence**
>
> The instruction that each such subagent gets a worktree of its own,
> .claude/skills/knowledge-architect-review/SKILL.md, lines 105-116:
>
> > - **Every subagent that judges a commit, and runs tests, a mutation, or any command of the checker
> >   or of a program the project builds, gets its own worktree, detached at that commit, never the
> >   live tree**: a reviewer at the commit under review, an auditor at the commit audited.
> > ...
> > **The
> >   dispatcher names each subagent's path in its brief, distinct for each subagent**, so subagents
> >   dispatched together never build inside one another's worktree.
>
> The check agent runs a command of the checker,
> .claude/agents/knowledge-architect-workflow-auditor.md, lines 196-198:
>
> [... the draft continues to line 67, before its proposed repair; quoted here: lines 9-24]

**The repair** (`repairs/K1/proposal.md`, lines 36-173)

- **Edit 1, script, after line 219**: inserts `checkTree = (j) => at(args.worktrees, 'check-' +
  j.dir.split('/').pop())` and a setup agent, run when `jobs.length`, whose prompt reads "You set
  up the worktrees of the check step of one pass of the agentic-workflow axis of a project audit.
  From the project root, for each of these paths that is not already a worktree, run git worktree
  add --detach <path> " + commit + ", and report what failed: <paths>." and "Edit nothing else.
  Return done true only if every command succeeded."; the run throws on a failed setup. Closes C1.
- **Edit 2, script `checkBrief`, lines 237 and 240-241**: "The worktree to read, detached at that
  commit: ' + reading" becomes "Your worktree, detached at that commit: ' + checkTree(j)"; "read
  both in the worktree" becomes "read both in your worktree". Closes C1.
- **Edit 3, script line 13 and audit SKILL line 271 (source 275)**: "the directory under which the
  preflight creates the worktrees" becomes "the directory under which the run creates the
  worktrees". A factual correction after Edit 1.

**Instructions**: added 3, removed 0. Quoted by the proposal: "From the project root, for each of
these paths that is not already a worktree, run git worktree add --detach <path> <commit>, and
report what failed: <paths>."; "Edit nothing else."; "Return done true only if every command
succeeded." Its argument: "the worktrees need one agent whose prompt tells it to create them. The
three sentences are that prompt, in the preflight's own shape."

**The nearest rival**: "each check agent reuses an auditor's worktree". The deciding fact: "the
number of check jobs is not bounded by the number of auditors. [...] This pass's causes file holds
19 causes [...] Under the rival, two auditor worktrees would each serve two concurrent check
agents." Cost of the proposal: "one build per check job (19 at this pass). The build time was not
measured."

**The check** (`repairs/K1/check.md`): "Defects the new text creates (L1 contradiction, L3 two
readings): 0"; heads reversed 0; rejected alternatives 0; goals strained 0. Notes, not defects:
note 1, "If the clause is kept, the cheapest form is: "for each of these paths that is not already
a worktree detached at <commit>""; note 2, the spec's criterion c6 records the agent budget the
owner approved and "The repair adds one more agent per pass"; note 3, the spec's check-step
paragraph (spec 1944-1945) "would gain the setup agent and its worktrees in the same change"; note
5, "This check ran `cargo klarch show` in the shared `reading` worktree, as this pass's brief
directs (script 237). That is the K1 defect itself, observed in use."

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 104:

> | K1 | C1 (W, new) | **owner list**: which side of a contradiction wins | A contradiction, repaired by removing a structure, adding 3 instructions (a setup agent's prompt). Default: the invariant wins, Edits 1 to 3, merged with K2 at `checkBrief` (1.1). Check: 0 defects. State beside the count: one more agent per pass, against the owner's budget recorded in the spec's criterion c6 (check note 2); spec "The proposal check" corrected in place (1.2). Optional, from check note 1: "not already a worktree detached at <commit>". |

Default: the invariant wins; Edits 1 to 3, merged with K2's Edit 2d in `checkBrief` (section 4).

### K2. The axis's run restated across the skill, the auditor and the script, with no pointer, and drifted

From `merge/causes.md`, K2:

> - **Kind**: a restatement that drifted from its home.
> - **Mechanism**: the script's prompts, the auditor's definition and the axis's section each restate
>   a rule whose home is another of them, or review-invariants, with no pointer beside the copy, and
>   three copies drifted: the convergence count became "the confirmed findings and the severe ones"
>   (C2), the files read from disk shrank to two of the invariant's three (C13), and the script's
>   corpus lost its exclusions (C24); the axis section's copies of the check brief and the
>   bears-on-tripwire mark agree today but carry no pointer (C25).

**Findings**

#### C2 (W, new text)

- **Draft kept**: `L1-b/d4-report-counts-severe-findings-no-text-defines.md`. **Merged into it**: L1-a/d2-severe-is-undefined.md, L1-c/D3-severe-is-undefined-in-the-corpus.md, L2-c/d2-report-counts-severe-undefined.md, L3-a/01-severe-undefined-in-report-header.md, L3-b/d1-report-header-severe-and-confirmed.md, L4-a/draft-01-severe-undefined-in-report-header.md, L4-b/d1-severe-is-defined-in-no-corpus-text.md, L5-a/d7-report-prompt-counts-severe-findings-no-text-defines.md.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the report prompt counts "the confirmed findings and the severe ones"; no corpus text defines "severe", and the skill's count is of violations left and of those on old text that went on the owner list".
- **Class decided by** (dedup): workflow .js 277 281: lines 278-280 blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L1-b/d4-report-counts-severe-findings-no-text-defines.md | verified | none | Quotations match (script 277-281; SKILL.md 101-103, 148-150); grep -rn -i severe over .claude (worktrees excluded) and CLAUDE.md returns script line 279 alone, the term being defined only in the spec (agentic-workflow-axis.md:142), which no run delivers; "severe" entered the script at [S9] (git log -S), so no earlier ruling covers it; "confirmed findings" counts every unrefuted draft (script 166), known and kept included, where the skill's count is of violations left. Class two readings, lens L3, as drafted. |

**The evidence**, verbatim from `L1-b/d4-report-counts-severe-findings-no-text-defines.md`, from its first section on:

> **Evidence**
>
> The report prompt of the saved workflow,
> `path@agent-config@workflows/knowledge-architect-agentic-workflow-audit.js` lines 277-281:
>
> > '1. the header: the commit audited, ' + args.commit + '; the agents of each step; the counts of ' +
> >   'drafts, verified, plausible, bearing on a ruling, bearing on a tripwire, known, kept, after ' +
> >   'dedup and refuted; for a re-check, the confirmed findings and the severe ones on old text and ' +
> >   'on new text; each ' +
>
> The count the pass owes, `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md` lines 148-150:
>
> > each pass's message gives the
> > count of violations left on old text and on new text, as point 8 defines them, and of those on old
> > text that went on the owner list, so that the passes show whether the audit converges.
>
> [... the draft continues to line 51, before its proposed repair; quoted here: lines 7-22]

#### C13 (W, old text)

- **Draft kept**: `L2-c/d3-briefs-omit-the-primer-from-the-disk-copies.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the script's briefs name the definition and the root CLAUDE.md as possibly stale copies, not the primer, which review-invariants names".
- **Class decided by** (dedup): workflow .js 124 125 and 139 140: blamed [S7], ancestor; 240 (check brief, same words): blamed [S7] by -C, ancestor.

**The verifier**, its line verbatim:

> | d3-briefs-omit-the-primer-from-the-disk-copies.md | verified | none | Script lines 124-125, 139-140, 240-241 name the definition and the root CLAUDE.md only; review skill lines 95-101 name the primer too; [S9] changed PRIMER.md (new row at worktree line 115), and the delivered primer in this verifier's context lacks that row, as the delivered CLAUDE.md says "adds one" where worktree line 352 says "adds two". Not recorded by `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, which is about a pin move's own session. No ruling on the brief's file list in the history. The class might read as a restatement drifted from review-invariants rather than a contradiction; the finding holds under either. |

**The evidence**, verbatim from `L2-c/d3-briefs-omit-the-primer-from-the-disk-copies.md`, from its first section on:

> **Evidence**
>
> The moment: any pass that runs after a commit changed the primer on disk, as this pass 6 does:
> [S9] adds a row to the primer (`git show --stat [S9]` lists `.claude/knowledge-architect/PRIMER.md | 1 +`).
>
> 1. The installed invariant, .claude/skills/knowledge-architect-review/SKILL.md lines 95-101:
>    "A subagent inherits the session's snapshot of the root `CLAUDE.md` and of the primer it imports,
>    and runs on its agent definition as the harness registered it when the session started, so once
>    any of them changes on disk, by an edit or by an install, the session is briefing from a copy
>    that no longer matches disk. ... Where the file changed on disk after the session started, tell
>    the subagent to read it from disk and to follow the disk where its copy differs."
>    The audit skill sends the re-check through it, .claude/skills/knowledge-architect-project-audit/SKILL.md
>    lines 100-101: "briefed on the edited files that carry its standard as
>    `skill@knowledge-architect-review@review-invariants` says."
>
> 2. The script's three briefs, .claude/workflows/knowledge-architect-agentic-workflow-audit.js
> [... the draft continues to line 43, before its proposed repair; quoted here: lines 12-27]

#### C24 (W, old text)

- **Draft kept**: `L5-a/d3-axis-corpus-stated-three-times-without-pointer.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the corpus definition is restated in the workflow auditor and the script with no pointer, the script dropping the exclusions".
- **Class decided by** (dedup): workflow-auditor.md 39 43: [P4], [AX], ancestors; workflow .js 81 83: [P4], [S7], ancestors.

**The verifier**, its line verbatim:

> | d3-axis-corpus-stated-three-times-without-pointer.md | verified | none | workflow-auditor.md:39-43 restates project-audit SKILL.md:240-243 in two sentences with no pointer; the script's CORPUS (lines 81-83) has no pointer and drops the exclusion of scoped CLAUDE.md files and registers. A restatement without its pointer is an L5 finding per D3 of the spec. F76 (pass 4) reworded both texts; it did not rule on the pointer. |

**The evidence**, verbatim from `L5-a/d3-axis-corpus-stated-three-times-without-pointer.md`, from its first section on:

> **Evidence**
>
> The home: the audit skill's own rule that an axis's section names its corpus,
> .claude/skills/knowledge-architect-project-audit/SKILL.md:71:
>
> > 1. **The corpus.** The axis's section names its corpus and what is read beside it.
>
> and that section, `skill@knowledge-architect-project-audit@agentic-workflow-axis`, lines 240-244:
>
> > **The corpus**: every text the harness delivers to a session as an instruction, installed or the
> > project's own: the project's root `CLAUDE.md` and the primer it imports, and every other such text
> > in the project's agent configuration directory. The scoped `CLAUDE.md` files next to the code are
> > not in it, nor are the registers.
>
> Restatement 1, `agent@knowledge-architect-workflow-auditor@the-corpus`,
> .claude/agents/knowledge-architect-workflow-auditor.md:39-43, word for word but the second sentence:
> [... the draft continues to line 52, before its proposed repair; quoted here: lines 13-28]

#### C25 (W, new text)

- **Draft kept**: `L5-a/d4-audit-axis-restates-auditor-check-and-tripwire-mark-without-pointer.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the axis section restates the auditor's check brief and its bears-on-tripwire mark with no pointer".
- **Class decided by** (dedup): project-audit SKILL.md 288 290 and 299 301: blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | d4-audit-axis-restates-auditor-check-and-tripwire-mark-without-pointer.md | verified | none | project-audit SKILL.md:288-290 and 299-301 restate the check brief (workflow-auditor.md:191-202) and the bears-on-tripwire mark (176-177) with no pointer beside either sentence; the only agent reference is line 257. New text ([S9]). |

**The evidence**, verbatim from `L5-a/d4-audit-axis-restates-auditor-check-and-tripwire-mark-without-pointer.md`, from its first section on:

> **Evidence**
>
> Home 1, `agent@knowledge-architect-workflow-auditor@check-a-proposal`,
> .claude/agents/knowledge-architect-workflow-auditor.md:191-202, the check's procedure:
>
> > A check brief names one proposal of the repair step, the cause it repairs, your directory, and the
> > corpus. Read the corpus as for drafting, then read the proposal's text after its edits:
> >
> > - through L1 and L3, ...
> > - against the record, as `skill@knowledge-architect-project-audit@repair-standard` says: ...
> >
> > Write `check.md` in your directory: ...
>
> Restatement 1, `skill@knowledge-architect-project-audit@agentic-workflow-axis`,
> .claude/skills/knowledge-architect-project-audit/SKILL.md:288-290:
>
> [... the draft continues to line 51, before its proposed repair; quoted here: lines 9-24]

**The repair** (`repairs/K2/proposal.md`, lines 20-242). Homes chosen: the record section for the
count (C2); the section "review-invariants" for the files read from disk (C13); **the auditor's
the section "the-corpus"**, not the skill's section, for the corpus (C24), on
`design@agent-skills@audit-agent-per-axis`; the auditor's the section "check-a-proposal" and
the section "verify-drafts" for the check and the tripwire mark (C25).

- **Edit 1, script 279-280** (C2): "for a re-check, the confirmed findings and the severe ones on
  old text and on new text" becomes "for a re-check, the counts the section record-of-an-audit of
  the skill knowledge-architect-project-audit asks of a pass, the owner-list one as the cross-check
  proposes it".
- **Edit 2, script** (C13): a constant `FROM_DISK` after line 91, "Your copies of your definition,
  of the root CLAUDE.md and of the primer it imports may predate this session: read each in the
  worktree this brief names, and follow the worktree where your copy differs.", replaces the
  two-file sentence in `draftBrief` (124-125), `verifyBrief` (139-140) and `checkBrief` (240-241).
- **Edit 3, script `CORPUS` 81-85** (C24): the restated definition becomes "The corpus is the one
  the section the-corpus of the agent definition knowledge-architect-workflow-auditor defines: read
  that section whole first.", the rest of the constant kept.
- **Edit 4, audit SKILL 240-244** (C24): "**The corpus**: every text the harness delivers [...] nor
  are the registers." becomes "**The corpus**: the one
  `agent@knowledge-architect-workflow-auditor@the-corpus` defines, read whole.", its reason
  sentence kept.
- **Edit 5, audit SKILL 288-290** (C25): "under its check brief," gains
  "`agent@knowledge-architect-workflow-auditor@check-a-proposal`,".
- **Edit 6, audit SKILL 299-301** (C25): "after reading that tripwire whole," becomes "as
  `agent@knowledge-architect-workflow-auditor@verify-drafts` says,".

**Instructions**: added 0, removed 0. Edit 2 "is the same one, brought back to its home's scope;
it is counted neither as added nor as removed."

**The nearest rival**: "keep the axis section of the skill as the home of the corpus, as the
causes file names it, and make the auditor's the section "the-corpus" point to it". The deciding fact:
`design@agent-skills@audit-agent-per-axis` "rejects "one generic auditor briefed to read the axis's
section of the skill", because it "puts the standard behind a read the brief must ask for"".

**The check** (`repairs/K2/check.md`): "defects the new text creates: 1 (D1)"; heads 0; rejected
alternatives 0; goals 0; S1 outside the record; N1 not created by the repair.

- **D1** (L3; under one reading L1): "Edit 3 delivers the auditor's checker commands to agents the
  same prompt forbids any command of the checker". The section it sends the four general-purpose
  agents to read whole holds, at auditor 48-54, "run `cargo klarch issues` and keep its listing"
  and "You may run read-only commands [...] such as `cargo klarch --help`, the help of a command, or
  `cargo klarch show`", while the same prompt says "run no command of the checker". Repair: "in
  Edit 3's "After", after "read that section whole first", add "; the commands that section gives
  an auditor are not yours"".
- **S1**: "Edits 3 and 4 make two sentences of the branch's spec false": spec 11 and 1625 ("the
  corpus, named as the axis's section names it"). "For the session: amend them in place in the
  commit that applies the repair [...] or put the relocation to the owner if it reads the approved
  thread as fixing the home."
- **N1**: Edit 5 keeps a two-sentence restatement of the section "check-a-proposal"; "It predates the
  repair."

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 105:

> | K2 | C2, C13 (W, old), C24 (W, old), C25 | **applied in the branch**: drifted restatements brought back to a home, pointers added | Every finding is in the applied row. Check D1 must be repaired first: the new `CORPUS` sends four general-purpose agents to read the auditor's the section "the-corpus" whole, whose lines 48-54 tell an auditor to run `cargo klarch issues` and `show`, while the same prompt says "run no command of the checker". Add the check's clause "; the commands that section gives an auditor are not yours". Check S1: Edits 3 and 4 make the auditor the corpus's home, which makes spec lines 11 and 1625 false; correct them in place. If the session reads #workflow-auditor-agent as fixing the skill as the home, Edit 4 alone goes to the owner list. Check N1 (Edit 5 keeps a two-sentence restatement) predates the repair: the session's judgement. |

Default: applied in the branch, Edits 1 to 6 with check D1's clause, spec 11 and 1625 corrected in
place.

### K3. What an earlier disposition does to a finding found again is stated in several texts

From `merge/causes.md`, K3:

> - **Kind**: two homes for one moment.
> - **Mechanism**: whether a later pass's finding counts, and which outcome it takes, when an issue
>   entry, the kept list or a ruling of the owner already covers it, is stated separately by point 8,
>   the outcomes table, the verifier's marks and refute bullet, the axis's "instead" sentence and
>   "Sent upstream", each with its own list of dispositions: a ruling to repair that did not close
>   the defect falls between "already ruled on" (refuted) and "violation left" (C3), and a known or
>   kept finding on the installed text takes both "sent upstream" and "nothing" (C4).

**Findings**

#### C3 (W, new text)

- **Draft kept**: `L1-b/d2-verifier-refutes-a-violation-left-on-ruled-text.md`. **Merged into it**: L3-a/05-verify-refute-already-ruled.md (verified; none).
- **Verdict and marks** (dedup): verified; bears-on-ruling (F97, commit [P5]).
- **Shared defect** (dedup): "a re-found finding on text whose approved repair did not close the defect is refuted as "already ruled on" by the auditor and is a violation left by point 8".
- **Class decided by** (dedup): workflow-auditor.md 172 179: lines 176-179 blamed [S9], phrase absent; project-audit SKILL.md 101 104: 101-103 blamed [P2-3R], ancestor (old); takes the newer.

**The verifier**, its line verbatim:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L1-b/d2-verifier-refutes-a-violation-left-on-ruled-text.md | verified | bears-on-ruling (F97, commit [P5]) | Quotations match (auditor 164-179; SKILL.md 61-64, 101-104, 148-150; script 137-138, 166). Where an approved repair failed to close a defect, the re-found finding is one the owner ruled on (refute, auditor 178) and a confirmed finding not kept nor known (a violation left, SKILL.md 101-103): no verdict satisfies both. The proposed repair deletes the bullet that F97 (pass 5, K2 E1) kept and reordered, on K2's premise that after E1 "only a ruled finding of neither case is refuted" and loses nothing; the draft shows that premise false for re-check findings on repaired text. Not settled here: the session does. Under the narrower reading (the ruled-on finding is the earlier pass's, not the re-found one) it is two readings rather than a contradiction. |

**The evidence**, verbatim from `L1-b/d2-verifier-refutes-a-violation-left-on-ruled-text.md`, from its first section on:

> **Evidence**
>
> The moment: the verify step of a re-check run (pass n+1), on a draft whose finding sits on text the
> owner ruled on in pass n with an outcome that changes the text (a repair the owner approved on the
> owner list), where the approved repair did not close the defect, or its new text carries one.
>
> Instruction A, the verify brief,
> `path@agent-config@agents/knowledge-architect-workflow-auditor.md` lines 172-179:
>
> > - mark known a draft whose finding an issue entry records, naming the entry, and kept a draft on
> >   the kept list;
> > - mark `bears-on-ruling` a draft whose finding is that a ruling's premise has proved false, and
> >   do not settle it: the session does;
> > [...]
> > - refute any other draft whose finding the owner already ruled on, naming the commit or the report
> >   file of the history that holds the ruling;
> [... the draft continues to line 63, before its proposed repair; quoted here: lines 7-22]

#### C4 (W, new text)

- **Draft kept**: `L1-c/D2-known-installed-finding-takes-two-outcomes.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "a known or kept finding on the installed text takes both "sent upstream" and the generic outcome, against "exactly one outcome"".
- **Class decided by** (dedup): project-audit SKILL.md 304 309: 306-308 blamed [P5], phrase absent; 121 121: db44ebb, old; 324 325: 324 old, 325 blamed [S9], phrase absent; takes the newer.

**The verifier**, its line verbatim:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L1-c/D2-known-installed-finding-takes-two-outcomes.md | verified | none | The quotations match audit SKILL.md 121, 127, 129, 304-308 and 324-326. For a known or kept finding on the installed text, line 307 sends it "instead" to the generic table. Two rows of that table name it, "sent upstream" and "nothing", and line 324 puts "every finding on the installed text" into the upstream file, so no single outcome satisfies line 121's "exactly one". The session meets this when it decides outcomes (line 309). The history's ruling "a draft an issue records is marked known, not refuted" ([P5] message) does not rule on the upstream overlap. Kept item 1 covers only this repository's departure. No issue entry. Its class could also be read as two readings, but the "every" and "exactly one" sentences make it a contradiction. |

**The evidence**, verbatim from `L1-c/D2-known-installed-finding-takes-two-outcomes.md`, from its first section on:

> **Evidence**
>
> `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md`:
>
> - line 121: "Every finding takes exactly one outcome. A finding that fits no named case goes on the owner list."
> - line 127: "| sent upstream | a finding on the installed text, which the project never edits: the axis's section says how |"
> - line 129: "| nothing | the entry or the text conforms; a finding an issue entry already records; a finding on the kept list |"
> - line 304-308: "A cause's findings on the installed text are sent upstream, as below. Its findings on the project's own text take the outcome its repair's edits on that text take in the table below: applied in the branch when every such edit falls in the applied row, the owner list otherwise. A finding marked known or kept, and a predicted gap, take instead the outcome `skill@knowledge-architect-project-audit@outcomes` and the table below give them."
> - line 324-326: "**Sent upstream**: every finding on the installed text, over every pass of the audit, a predicted gap included, goes into one file per audit, written when the audit stops from the report files of its passes, which history keeps."
>
> **The moment**
>
> An adopting project's pass of the agentic-workflow axis, at the session's decision of the outcomes (line 309: "The cross-check proposes each cause's outcome, and the session decides it."), for a confirmed finding on the installed text that a verifier marked known (an issue entry of the project records it, as `skill@knowledge-architect-issue-tracking@issue-entry` has a project do for a gap of the checker or the workflow) or kept (on the kept list of a re-check).
>
> - Line 324: "every finding on the installed text, over every pass" goes into the upstream file: outcome "sent upstream".
> - Line 307-308 sends a known or kept finding "instead" to the outcomes table, whose row "nothing" (line 129) names "a finding an issue entry already records; a finding on the kept list", while its row "sent upstream" (line 127) names "a finding on the installed text". Line 121 allows "exactly one outcome".
> [... the draft continues to line 29, before its proposed repair; quoted here: lines 9-24]

**The repair** (`repairs/K3/proposal.md`, lines 17-150). Home: point 8 with the row "nothing".

- **Edit 1, auditor the section "verify-drafts" 178-179** (C3): the bullet "refute any other draft whose
  finding the owner already ruled on, naming the commit or the report file of the history that
  holds the ruling;" is deleted.
- **Edit 2, audit SKILL the section "outcomes" 127** (C4): "| sent upstream | a finding on the installed
  text, which the project never edits: the axis's section says how |" becomes "| sent upstream | a
  finding on the installed text, which the project never edits, unless the row "nothing" names it:
  the axis's section says how |".
- **Edit 3, audit SKILL 324-325** (C4): "**Sent upstream**: every finding on the installed text,
  over every pass of the audit" becomes "**Sent upstream**: every finding whose outcome is sent
  upstream, over every pass of the audit".

**Instructions**: added 0, removed 1: "refute any other draft whose finding the owner already
ruled on, naming the commit or the report file of the history that holds the ruling". Net -1.

**The nearest rival**, to Edit 1: "narrow the bullet instead of deleting it" (L3-a/05's repair).
The deciding fact: "the bullet's narrowed members, a ruling that left the text as it stands, are
what the kept list holds by its definition [...] The narrowed bullet is therefore a second home for
the kept list's moment with another verdict, refuted, whose drafts the session sees only by
sample". To Edits 2 and 3, "the reverse direction": known and kept findings kept upstream; it
"makes the axis's paragraph a second home that overrides them for one side".

**The check** (`repairs/K3/check.md`): "defects the new text creates 1; heads reversed 0; rejected
alternatives brought back 0; goals strained 2. Notes for the session, outside those counts: 3."

- **D1** (L3): "Edit 2's "unless" clause reads two ways": "it" can be the finding, or the installed
  text, under which "the clause is empty" and the overlap stays. Repair: "| sent upstream | a
  finding on the installed text that the row "nothing" does not name; the project never edits the
  installed text: the axis's section says how |".
- **G1**: "A known finding on the installed text no longer reaches the workflow's upstream (Edit
  2)", against `goal@agent-skills@one-skill-per-activity` and
  `goal@knowledge-architect@agents-get-a-complete-workflow`: "The installed text keeps the defect
  these goals exclude."
- Heads: none reversed in a design home; the spec's decided design diverges at 325-326, 1570, 1605
  and 1764-1766, all to correct in the same commit.
- Note 1: "The cause keeps a second overlap of the same kind": point 8's "one the owner does not
  rule on becomes an issue entry" against "sent upstream". Note 2: argument a77 leans on the
  removed bullet; its thread was voided by D34. Note 3: the proposal's met-outside item is
  "confirmed by reading".

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 106:

> | K3 | C3 (bears on F97), C4 (W, new) | **owner list**: which side of a contradiction wins, with F97's corrected premise quoted first (section 3) | Default: Edits 1 and 3, and Edit 2 reworded as check D1 says, since "unless the row "nothing" names it" binds the finding or the installed text: "a finding on the installed text that the row "nothing" does not name; the project never edits the installed text: the axis's section says how". Name to the owner check G1: in an adopting project, a known finding on the installed text no longer reaches the upstream file. Spec 325-326, 1570, 1605 and 1764-1766 corrected in place. Check note 1, the same overlap at point 8's "one the owner does not rule on", is not repaired by K3: route it as met outside the task. Net -1. |

**Bears on a ruling** (C3; cross-check, section 3):

From `cross-check/cross-check.md`, lines 132-143:

> **C3 (cause K3): F97 of pass 5**
>
> - Source, message of [P5]: "- F97 K2 (C3, C8, C9, C22, C24): E1 to E5." The owner's word, same
>   message: "Agreed on all the defaults, proceed".
> - The premise, pass5/repairs/K2/proposal.md line 83: "only a ruled finding of neither case is
>   refuted", where the two cases are a kept draft and a `bears-on-ruling` draft.
> - Why it may be false: a ruling to apply a repair that did not close the defect is a ruling of
>   neither case. The verifier refutes the re-found finding as "already ruled on" (auditor 178-179),
>   while point 8 counts it as a violation left, "whether a draft missed it or an edit made it" (audit
>   SKILL 101-103). A refuted draft never reaches the confirmed-findings file.
> - Default: K3 Edit 1, the refute bullet deleted.
>

Default: Edits 1 and 3, and Edit 2 in check D1's wording; net -1; G1 named to the owner.

### K4. The record of a pass is split between its report file and its commit messages

From `merge/causes.md`, K4:

> - **Kind**: two homes for one moment.
> - **Mechanism**: the section "record-of-an-audit" makes the report file a second record of each pass beside
>   the commit messages, and keeps rules written when the messages were the only record: "each commit
>   of an audit" and "each pass's message" leave open which of a pass's two or more commits carries
>   the counts (C18), the file "committed ... before the pass's edits" admits a commit before or
>   after the rulings (C19), the `F<n>` label reaches both records (C21), and naming a commit by its
>   subject is given for messages only while the file's header carries the branch's own SHA (C30).

**Findings**

#### C18 (W, new text)

- **Draft kept**: `L3-b/d2-each-pass-s-message-with-two-commits.md`. **Merged into it**: L3-a/03-each-commit-of-an-audit.md (verified; none).
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): ""each commit of an audit" and "each pass's message" leave open which of a pass's two or more commits carries the counts".
- **Class decided by** (dedup): project-audit SKILL.md 141 150: 141-143 and 147-150 blamed [S9], phrase absent; 145-146 blamed [S9], phrase present (old); takes the newer.

**The verifier**, its line verbatim:

> | d2-each-pass-s-message-with-two-commits.md | verified | none | The quotations match SKILL.md 141-150 and 312-314. Lines 141-142 make a pass hold at least two commits: the report file before the edits, then the edits. Lines 145 ("Each commit of an audit") and 148 and 312 ("each pass's message", "Each pass's commit message") leave open which commit carries the counts. The report-file commit cannot know the word count "after the pass". No recorded ruling in the history fixes the commit; the earlier rulings (history 719, 773: "each pass's commit gives the corpus's word count") predate the second commit. No issue entry or kept item records it. |

**The evidence**, verbatim from `L3-b/d2-each-pass-s-message-with-two-commits.md`, from its first section on:

> **Evidence**
>
> `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md`:
>
> - lines 141-143: "The file is committed with its `Outcome` lines before the pass's edits are
>   applied; the commit that carries out its last outcome, or a later commit of the same branch,
>   deletes it, and its message names it by its path."
> - lines 145-150: "Each commit of an audit says what was audited, at which commit, named by its
>   subject since a rebase merge gives the audit branch's commits new SHAs, how it was dispatched, the
>   calibration reading settled where the axis has one, the outcome of each finding in a line, and the
>   path of the pass's report file. A figure the audit measured goes there too, and each pass's
>   message gives the count of violations left on old text and on new text, as point 8 defines them,
>   and of those on old text that went on the owner list, so that the passes show whether the audit
>   converges."
> - lines 312-314: "Each pass's commit message also gives the corpus's word count before and after
>   the pass, `wc -w` over the corpus's files."
> [... the draft continues to line 47, before its proposed repair; quoted here: lines 9-24]

#### C19 (W, new text)

- **Draft kept**: `L3-b/d3-report-file-committed-before-the-edits.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the report file "committed with its Outcome lines before the pass's edits" admits a commit before or after the owner's rulings".
- **Class decided by** (dedup): project-audit SKILL.md 141 142: blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | d3-report-file-committed-before-the-edits.md | verified | none | The quotations match SKILL.md 34, 88-95 and 141-142, and design.md 1480-1493. The delivered words allow both acts, since point 6 writes a pending `Outcome` line into every item. The finding stands, but the reading meant is Act A, not the Act B the repair adopts. docs/audit-reports/README.md:8 says "A file is committed once the owner has ruled on every item, before the pass's edits are applied." The source's `%%` comment, crates/agent-skills/content/skills/project-audit/SKILL.md:136, says "committed with its rulings". The spec thread committed-audit-report, line 1972, says the same. Neither text is a head or an instruction. The head audit-outcomes speaks of the design-record axis, and its "at once" means applied in the branch, not before the owner rules. So the repair should state Act A, and the outcome is the owner list unless the session takes these texts as stating the reading meant. Not refuted: the two readings are real. |

**The evidence**, verbatim from `L3-b/d3-report-file-committed-before-the-edits.md`, from its first section on:

> **Evidence**
>
> `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md`:
>
> - line 34, the term: "**the owner list** | every finding of one pass that needs the owner's word,
>   in the pass's report file, and the one message that points the owner to it"
> - lines 88-95, point 6: "The items of one pass, in the pass's report file ..., each under a label,
>   `F<n>`, ... and ending with a `Default` line and an `Outcome` line. One message gives the owner
>   the file's full path and one line per item, and proposes that the owner rules by writing each
>   `Outcome` line; a ruling given in the conversation is copied into the file verbatim. Apply
>   nothing on the list before the owner answers."
> - lines 141-142: "The file is committed with its `Outcome` lines before the pass's edits are
>   applied;"
> - line 125 and 320: the "applied in the branch" rows, findings that need no owner's word.
>
> The record: `path@agent-skills@docs/design.md`, head `design@agent-skills@audit-outcomes`, lines
> [... the draft continues to line 45, before its proposed repair; quoted here: lines 9-24]

#### C21 (W, new text)

- **Draft kept**: `L3-a/07-owner-list-label-record.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the F label reaches both the report file and the commit messages, so its numbering record has two readings".
- **Class decided by** (dedup): project-audit SKILL.md 88 91: 88, 89, 91 blamed [S9], phrase absent; 90 phrase present (old); takes the newer.

**The verifier**, its line verbatim:

> | 07-owner-list-label-record.md | verified | none | SKILL.md:88-91, PRIMER.md:30-36, the `F` row of design@agent-skills@ruled-items-labelled ("the audit's commit messages", crates/agent-skills/docs/design.md:495) and the pass-5 line "The owner list was F95 to F106" (history.txt:164) match. The F label reaches both the report file and the commit messages. The primer's carried-label sentence leans to reading 1 (the file is written first), while the design head and five passes of practice give reading 2. |

**The evidence**, verbatim from `L3-a/07-owner-list-label-record.md`, from its first section on:

> **Evidence**
>
> Point 6 of the method, `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md` lines 88-91:
>
> > 6. **The owner list.** The items of one pass, in the pass's report file of
> >    `skill@knowledge-architect-project-audit@record-of-an-audit`, grouped by kind, each under a
> >    label, `F<n>`, numbered as `primer@labelled-items` says, with a default and the words of the
> >    entry or the text the item turns on, and ending with a `Default` line and an `Outcome` line.
>
> The record section, same file, lines 145-148: "Each commit of an audit says ... the outcome of each
> finding in a line, and the path of the pass's report file."
>
> `path@agent-config@knowledge-architect/PRIMER.md` lines 30-36:
>
> > **An item put to the owner under a label**, `Q<n>` or the prefix a skill gives it, **takes the
> > next number of its prefix in the record the label reaches, from 1, and that number is never reused
> [... the draft continues to line 53, before its proposed repair; quoted here: lines 7-22]

#### C30 (W, new text)

- **Draft kept**: `L6-b/draft-02-audit-report-file-carries-branch-shas.md (W side)`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the committed report file carries the branch's own SHA in its header, and the name-by-subject reason is given for commit messages only".
- **Class decided by** (dedup): project-audit SKILL.md 137 142: blamed [S9], phrase absent; workflow .js 170 172: [S9], absent; 277 277: [S7], old; takes the newer.

**The verifier**, its line verbatim:

> | draft-02-audit-report-file-carries-branch-shas.md | W | verified | none | Quotations match (audit skill lines 137-142, not 137-143, and line 145-146; workflow lines 170-172 and 277; CLAUDE.md 608-611; toml line 63). The report header the workflow writes carries args.commit, here [S9], which `git merge-base --is-ancestor [S9] origin/main` shows is a branch commit (exit 1). The installed text says to commit "the run's report as the session's verification left it" and gives the name-by-subject reason for commit messages only, so it allows committing the SHA or renaming it. No ruling in the history (no SHA item), no issue entry, not kept. Outcome, edited at source here: applied in the branch, the reading meant being stated by `design@knowledge-architect@no-branch-sha-is-cited` and the skill's own reason at line 145-146; the session judges whether the moved clause changes what agents are told. |
> | draft-02-audit-report-file-carries-branch-shas.md | P | refuted | none | The P side holds no finding: the root CLAUDE.md lines 608-611 state the reading meant ("neither a commit message nor a document cites the SHA of a commit of its own branch"), and `refuse-branch-shas = true` makes `cargo klarch commits` refuse the other reading. Outcome nothing, as the draft itself proposes. |

**The evidence**, verbatim from `L6-b/draft-02-audit-report-file-carries-branch-shas.md`, from its first section on:

> **Evidence**
>
> The installed procedure, `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md`, section
> the section "record-of-an-audit", lines 137-143:
>
> > The session writes it after its reading of the pass:
> > for an axis with a saved workflow, the run's report as the session's verification left it; for an
> > axis with none, the drafts read at the sort, one section per finding with its evidence and its
> > proposed edit. The owner list closes it, as point 6 says. A span the checker would refuse is quoted
> > in words, since the file is checked like any other. The file is committed with its `Outcome` lines
> > before the pass's edits are applied;
>
> and lines 145-146, which name the audited commit by its subject in a commit message only:
>
> > Each commit of an audit says what was audited, at which commit, named by its subject since a
> > rebase merge gives the audit branch's commits new SHAs,
> [... the draft continues to line 77, before its proposed repair; quoted here: lines 10-25]

**The repair** (`repairs/K4/proposal.md`, lines 29-161; source lines given there)

- **Edit 1, point 6, audit SKILL 90** (C21): "numbered as `primer@labelled-items` says" becomes
  "numbered as `primer@labelled-items` says with that file as its record". Consequence, stated by
  the proposal: "each pass's list starts at F1, so pass 6 starts at F1, not at F107".
- **Edit 2, the section "record-of-an-audit", audit SKILL 135-150** (C18, C19, C30):
  - "and the commit messages hold its counts" becomes "and the pass's commit message holds its
    counts";
  - added: "The file and every commit message of the audit name a commit of the audit's branch by
    its subject, never by its SHA, since a rebase merge gives the branch's commits new SHAs.";
  - "The file is committed with its `Outcome` lines before the pass's edits are applied" becomes
    "The file is committed once every `Outcome` line holds the owner's ruling, before any edit of
    the pass is applied";
  - "Each commit of an audit says what was audited, at which commit, named by its subject since a
    rebase merge gives the audit branch's commits new SHAs, how it was dispatched, [...]" becomes
    "The pass's commit is the one that applies its last edit, or the one that adds its report file
    where the pass applies none. Its message says what was audited, at which commit, how it was
    dispatched, [...]";
  - "and each pass's message gives the count of violations left" becomes "and the count of
    violations left".

**Instructions**: added 2, removed 2. Removed: "Each commit of an audit says what was audited, at
which commit, [...] and the path of the pass's report file." and the clause "named by its subject
since a rebase merge gives the audit branch's commits new SHAs". Added: "The file and every commit
message of the audit name a commit of the audit's branch by its subject, never by its SHA, since a
rebase merge gives the branch's commits new SHAs." and "The pass's commit is the one that applies
its last edit, or the one that adds its report file where the pass applies none. Its message says
[...]".

**The nearest rival**: "merge the record into the report file alone". The deciding fact: "the
corpus's word count "before and after the pass", which the axis's section asks of the pass's
commit message [...], is known only once the edits are applied, after the file is committed under
C19's reading; and the spec as built keeps the counts in the message".

**The check** (`repairs/K4/check.md`): "defects the new text creates: 2. Heads the repair would
reverse: 0 by itself; 2 heads it contradicts or strains".

- **D1** (L1): "the file waits for a ruling that point 8 says may never come": at the audit's stop,
  "one the owner does not rule on becomes an issue entry", while the new text commits the file only
  "once every `Outcome` line holds the owner's ruling". Repair: "The file is committed once every
  `Outcome` line holds the owner's ruling, or, at the audit's stop, the issue entry point 8 gives
  an item the owner does not rule on, before any edit of the pass is applied".
- **D2** (L1, on this branch only): "`F` labels reused in a record that numbered them": "Passes 1
  to 5 numbered `F` in the branch's commit messages [...] holds labels up to F106"; under Edit 1,
  "F1 to F106 come back in a record where the primer says a number "is never reused there"".
  Options: "number pass 6's file from F107, as a one-time transition, or accept the reuse and cite
  each label beside its file's path".
- Heads: `design@agent-skills@ruled-items-labelled`, its `F` row, "contradicted until the harvest",
  which the spec schedules; `design@agent-skills@audit-outcomes`, "strained": "Whether "at once"
  meant "in the branch rather than deferred" or "before the list's ruling" is the owner's to say."

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 107:

> | K4 | C18, C19, C21, C30 (W, new) | **owner list** | Edit 2 changes which commit carries the record, and Edit 1 contradicts the `F` row of `design@agent-skills@ruled-items-labelled` until the spec's harvest. Default: Edits 1 and 2, with check D1's rewording: the file is committed once every `Outcome` line holds the owner's ruling "or, at the audit's stop, the issue entry point 8 gives an item the owner does not rule on". Check D2, a sub-question: this branch's messages already numbered F1 to F106, and this pass's own findings cite F23, F44, F58, F94, F97 and F99 by label, which the next pass's verifiers read through `history`. Proposed default for this branch: pass 6's file continues at F107; from the next audit on, each file numbers from F1. Name the strain on `design@agent-skills@audit-outcomes` ("applied at once"), whose ground is the owner's words. Net 0. |

Default: Edits 1 and 2 with check D1's rewording; pass 6's file continues at F107, and later
audits number from F1.

### K5. "its phrase" in the old-and-new-text test names the blamed line or the defect's words

From `merge/causes.md`, K5:

> - **Kind**: none.
> - **Mechanism**: the axis's classification test says a line is on new text "only where its phrase
>   ... is absent" from the corpus at `previous`, where "its" grammatically names the line, while
>   point 8 defines old text by "words no commit changed"; a line edited elsewhere, the defect's
>   words unchanged, is new by one reading and old by the other, and the class decides the
>   convergence count.

**Findings**

#### C17 (W, new text)

- **Draft kept**: `L3-b/d4-old-and-new-text-its-phrase.md`. **Merged into it**: L3-a/02-old-new-text-its-phrase.md (verified; none).
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): ""its phrase" in "Old and new text" can mean the blamed line's whole text or the words carrying the defect".
- **Class decided by** (dedup): project-audit SKILL.md 294 297: blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | d4-old-and-new-text-its-phrase.md | verified | none | The quotations match SKILL.md 294-297 and 107-110, and the script's lines 170-172. Grammatically "its" refers to "A line", the whole line (Act A). Under Act A, a line edited elsewhere after `previous` is new text, which contradicts point 8's definition by words (Act B). The spec intends Act B: line 1227 says "the words carrying its defect", and AC7 at line 2255 says "the words carrying its defect are absent from every file of it". The spec is no instruction, but point 8's definition at lines 107-110 is one, so "applied in the branch" toward Act B is supportable. No issue entry, kept item or ruling covers it. |

**The evidence**, verbatim from `L3-b/d4-old-and-new-text-its-phrase.md`, from its first section on:

> **Evidence**
>
> `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md`:
>
> - lines 294-297: "**Old and new text**, for a re-check: for each line that carries a confirmed
>   finding's defect, the dedup runs `git blame -w -C` at the commit audited. A line whose blamed
>   commit is not an ancestor of `previous` is on new text only where its phrase,
>   whitespace-normalised, is absent from every file of the corpus at `previous`; a finding on two
>   texts takes the newer."
> - lines 107-110, the definition it applies: "old text is text the previous pass also read, whose
>   words no commit changed after the commit that pass audited, a move or a reflow leaving it old;
>   new text is text a later commit wrote, the previous pass's repairs among them."
>
> The dedup prompt of the script, lines 170-172, sends the dedup to that paragraph: "Classify each
> finding as on old or new text, as ' + SECTION + ' says under "Old and new text"".
>
> [... the draft continues to line 39, before its proposed repair; quoted here: lines 9-24]

**The repair** (`repairs/K5/proposal.md`, lines 24-86)

- **Edit 1, audit SKILL 294-297** (source 298-301; C17): "is on new text only where its phrase,
  whitespace-normalised, is absent from every file of the corpus at `previous`;" becomes "is on new
  text only where the words on it that carry the defect, whitespace-normalised, are absent from
  every file of the corpus at `previous`, so an edit elsewhere in the line leaves the finding on old
  text;". Why reading 2: point 8 "defines the class by words, not by lines", and AC7 (spec 2255)
  says "the words carrying its defect".

**Instructions**: added 0, removed 0, "a rewording that makes an existing instruction's intent
clearer".

**The nearest rival**: "**Reading 1, made explicit**: "only where the whole line,
whitespace-normalised, is absent"". The deciding fact: "AC7, which the owner applied, judges pass
6's classification by "the words carrying its defect", and point 8 defines the class by "whose
words no commit changed". Under the rival, AC7 fires on a correct application of the axis's own
sentence".

**Consequences the proposal names**: "a finding marked new whose defect lines were edited after
[S7] elsewhere in the line may be old. The counts of this pass's commit message, and AC7's
reading of at least five findings classified new, should use reading 2".

**The check** (`repairs/K5/check.md`): "defects the new text creates: 1. Heads reversed: 0.
Rejected alternatives brought back: 0. Goals strained: 0."

- **D1** (L3): "the "so" clause reads as a consequence or as a rule of its own": for a line a
  commit A wrote after `previous` and a commit B edited elsewhere, reading 2 classifies it old "from
  B's diff alone, with no search", which contradicts point 8. Repair: "so an edit elsewhere in a
  line the previous pass read leaves the finding on old text", or drop the clause.
- Note on AC4: "the comparison with 16 and 8 is not taken by one method"; the baseline was taken
  by sentence.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 108:

> | K5 | C17 (W, new) | **owner list** | The reading meant (the words carrying the defect) is stated outright only by AC7, a plan document's criterion, which is neither a design head nor an instruction; point 8 supports it without excluding the other reading. So neither the applied row nor the owner-list row fits by its words, and the section "outcomes" sends a finding that fits no named case to the owner list. Default: Edit 1 with the "so" clause scoped as check D1 says, "so an edit elsewhere in a line the previous pass read leaves the finding on old text"; spec 1909-1912 corrected in place. Consequence before the pass's counts: confirmed.md classified by the whole line, and "no finding was re-tested under that reading"; the session re-classifies the 18 findings on new text by the defect's words before it writes the counts AC4 reads. Check's note on the pass-5 baseline, taken by sentence, goes with AC4's reading. |

Default: Edit 1 with the clause scoped as check D1 says; spec 1909-1912 corrected in place; this
pass's findings re-classified by that reading before its counts are written.

### K6. The intake states the shaping and the record check of a fix at items c, e and f as well as at their home

From `merge/causes.md`, K6:

> - **Kind**: two homes for one moment.
> - **Mechanism**: the intake points to the section "repair-standard" from item c (the record check of "the fix
>   item f settles on"), item e (the heads it would reverse) and item f (the shape), and glosses
>   part of the section at item f; so the one act of shaping a fix and reading it against the record
>   is split across items run by different agents at different moments (c by subagents before f
>   exists, C14), and is partly restated, dropping the growth condition and the owner's ruling on it
>   and the instruction to read the home whole (C26).

**Findings**

#### C14 (P, new text)

- **Draft kept**: `L2-c/d4-intake-item-c-needs-item-f-fix.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; bears-on-ruling (D35, commit [D32-35]).
- **Shared defect** (dedup): "intake item c reads "the fix it settles on" against the record, a fix item f settles later and c may run in subagents".
- **Class decided by** (dedup): klarch-retrospective-intake SKILL.md 91 96: 94-96 blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | d4-intake-item-c-needs-item-f-fix.md | verified | bears-on-ruling | Intake lines 91-96, 113-117, 118-129, 146-151 and 80 quoted correctly: item c reads "the fix item f settles on" against the record, while c may run in subagents told not to search the registers again, and f is the session's. The placement in c is D35 ([D32-35]: "the intake points to it from its items c, e and f", approved as "D34 and D35 defaults too"); its premise in the spec (lines 1934-1936), that items c and e "already read ... the decisions a fix would reverse", is false for c run by a subagent without f's fix. The session settles the ruling; the draft's owner-list outcome is consistent with that. |

**The evidence**, verbatim from `L2-c/d4-intake-item-c-needs-item-f-fix.md`, from its first section on:

> **Evidence**
>
> The moment: a session under `skill@klarch-retrospective-intake` that dispatches subagents for the
> checks a to c of a received file's findings.
>
> .claude/skills/klarch-retrospective-intake/SKILL.md:
> - lines 91-96, item c: "**c. Whether it is already recorded.** An issue or a tripwire, from the
>   search ... A rejected alternative ... The fix item f settles on is read against the record as the
>   record check of `skill@knowledge-architect-project-audit@repair-standard` says."
> - lines 113-117, item e: "Whether the fix reverses a recorded decision, per
>   `skill@knowledge-architect-decision-recording@reversal-check`: ... The record check of item c
>   names the heads it would reverse."
> - lines 118-129, item f: "**f. A better fix.** Name the nearest rival to the fix the report
>   proposes ... The fix is shaped as `skill@knowledge-architect-project-audit@repair-standard` says".
> - lines 146-151, "Subagents": "The checks a to c read text, and fresh subagents may run them, one
>   per file or per cluster. Brief each with the findings, the files to read, the entries the search
> [... the draft continues to line 38, before its proposed repair; quoted here: lines 8-23]

#### C26 (P, new text)

- **Draft kept**: `L6-b/draft-01-intake-restates-repair-standard.md`. **Merged into it**: L5-a/d5-intake-restates-the-repair-standard-in-part.md (verified; none), L6-a/d01-intake-restates-repair-standard.md (verified; none).
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the intake glosses several directives of repair-standard in one sentence, without "read whole", dropping the growth condition and ruling".
- **Class decided by** (dedup): klarch-retrospective-intake SKILL.md 125 127: blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | draft-01-intake-restates-repair-standard.md | P | verified | none | Quotations match the worktree (intake lines 125-127, audit skill lines 168-176, primer lines 93-97). The intake's sentence glosses five directives of the section "repair-standard" and drops its condition ("When the repairs applied together add more ...") and the owner's ruling on growth: more than one sentence restated, which `primer@where-knowledge-goes` never allows. D35 ([D32-35]: "the intake points to it from its items c, e and f") ruled a pointer, not the gloss, so no ruling refutes it. No issue entry, not kept. Read tripwire pointer-not-followed whole: the draft carries no firing evidence. Outcome: applied in the branch, drifted restatement brought back to a pointer. |

**The evidence**, verbatim from `L6-b/draft-01-intake-restates-repair-standard.md`, from its first section on:

> **Evidence**
>
> The project's text, `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`, lines 125-127, item f:
>
> > The fix is shaped as `skill@knowledge-architect-project-audit@repair-standard` says, the audit's
> > standard for the same text: at the cause, removing before adding, its instructions counted, its
> > nearest rival named, and the net growth of the fixes stated to the owner first.
>
> The home, installed, `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md`, section
> the section "repair-standard", lines 157-176, is several directive sentences. Among them, lines 173-176:
>
> > Each repair counts the instructions it adds and removes, and names the nearest
> > rival to itself and the fact that decides between them. **When the repairs applied together add
> > more instructions than they remove, the owner is told so first, with the net figure**, and rules on
> > that growth before any of them is applied.
>
> [... the draft continues to line 61, before its proposed repair; quoted here: lines 10-25]

**The repair** (`repairs/K6/proposal.md`, lines 39-134), in `path@agent-config@skills/klarch-retrospective-intake/SKILL.md`:

- **Edit 1, item c, 94-96** (C14): the sentence "The fix item f settles on is read against the
  record as the record check of `skill@knowledge-architect-project-audit@repair-standard` says." is
  removed.
- **Edit 2, item e, 117** (C14): "The record check of item c names the heads it would reverse."
  becomes "The record check of item f names the heads it would reverse."
- **Edit 3, item f, 125-127** (C26, C14): "The fix is shaped as
  `skill@knowledge-architect-project-audit@repair-standard` says, the audit's standard for the same
  text: at the cause, removing before adding, its instructions counted, its nearest rival named,
  and the net growth of the fixes stated to the owner first." becomes "The fix is shaped, and read
  against the record, as `skill@knowledge-architect-project-audit@repair-standard` says, read
  whole: it is the audit's standard for the same text."

**Instructions**: added 1 (Edit 3's sentence), removed 2 (item c's sentence and item f's gloss).
"You can also count edit 3 as a rewording that absorbs item c's act, so 0 added and 1 removed.
Either count gives net -1."

**The nearest rival**: "keep a sentence in item c that points to item f", "the literal wording of
D35". The deciding fact: "item c is one of the checks a to c, and the section "intake-subagents" gives those
checks to subagents that run before item f has settled a fix."

**The check** (`repairs/K6/check.md`): "Defects the new text creates (L1, L3): **0**"; heads 0;
rejected alternatives 0; goals 0. Note 1: "The owner's words for D35 are not the ones the proposal
quotes as its "literal wording"": the owner's words are "D32 and D33 approved. D34 and D35 defaults
too.", and the placement is the decided design's (spec 1933-1936) and step 9's (2396). Note 2: step
9's line 2396 "needs the same correction". Note 3: `tripwire@agent-skills@pointer-not-followed`
watches Edit 3, without firing. Note 4: pre-existing, the record test of ruled-out threads runs
before f.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 109:

> | K6 | C14 (bears on D35), C26 (P, new) | **owner list**, one item, with D35's corrected premise quoted (section 3) | Edit 3 alone is in the applied row; Edits 1 and 2 move a placement the spec's decided design and step 9 state under D35. Default: Edits 1 to 3, net -1. Check note 1: quote the owner's words "D32 and D33 approved. D34 and D35 defaults too." and the default they answered (spec 2704-2713), not the commit's summary; name spec 1933-1936 and 2396 as corrected in place. If the owner keeps the placement in item c, Edit 3 alone is applied and closes C26. |

**Bears on a ruling** (C14; cross-check, section 3):

From `cross-check/cross-check.md`, lines 170-185:

> **C14 (cause K6): D35**
>
> - Source, message of [D32-35]: "D35: the shared section "Repairs to agent-facing text" holds the
>   repair by cause and the record check; the intake points to it from its items c, e and f." The
>   owner's words, same message: "D32 and D33 approved. D34 and D35 defaults too." The default they
>   answered, spec 2704-2713: "the record check is part of judging a repair's shape, the intake's
>   items c and e already read the same record, so one home keeps one standard". The placement, spec
>   1933-1936: "its items c and e, which already read the issues, the tripwires, the rejected
>   alternatives and the decisions a fix would reverse, point to the section's record check for a
>   fix's own reading against the record".
> - The premise: item c can read the fix against the record.
> - Why it may be false: item c is one of the checks "a to c" that fresh subagents may run
>   (`skill@klarch-retrospective-intake@intake-subagents`), before item f, which the session
>   establishes, has settled the fix.
> - Default: K6, the record check stated at item f; D35's one home kept.
>

Default: Edits 1 to 3, net -1; spec 1933-1936 and 2396 corrected in place. If the owner keeps
D35's placement, Edit 3 alone, which closes C26.

### K7. The repair standard claims every repair of an instruction, and only an audit and the intake deliver it

From `merge/causes.md`, K7:

> - **Kind**: none.
> - **Mechanism**: the section "repair-standard" opens "What any repair of an instruction is judged by, whether
>   a reading or a real session found its defect", but no installed text on the path of a real
>   session's finding in an adopting project (retrospective, issue-tracking, agent-configuration)
>   names it, so the section either binds a repair the session cannot know of, or binds less than
>   its word "any".

**Findings**

#### C16 (W, new text)

- **Draft kept**: `L2-d/draft-2-repair-standard-scope-two-readings.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; known in part (issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis).
- **Shared defect** (dedup): "repair-standard's scope sentence claims any repair, while the audit skill is bounded to an audit and no real-use path names the section".
- **Class decided by** (dedup): project-audit SKILL.md 152 155: 152 and 154 blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | draft-2-repair-standard-scope-two-readings.md | verified | known in part (`issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`) | Quotations hold: project-audit SKILL.md:152-155 ("What any repair of an instruction is judged by, whether a reading or a real session found its defect.") and :174-176; retrospective SKILL.md:124-125; agent-configuration SKILL.md:27-36. The search `grep -rn 'repair-standard' CLAUDE.md .claude/` returns exactly the sites the draft names: project-audit 152, 285 and 312, the workflow auditor 196, the saved workflow 89, and klarch-retrospective-intake 96 and 125. No installed text on an adopting project's real-use path names the section. The two acts are real. The section's scope sentence claims every repair. The audit skill's own Scope (lines 8-11) and its description bound it to an audit, and the agent-configuration skill gives a narrower record check at the edit. Spec citations check out: docs/plans/specs/agentic-workflow-axis.md:1927-1930 (the decided design, "any repair of an instruction found by reading or by real use") and :2119-2121 with a80. That decided design was closed by the owner's checkpoint word in round 24, so the record does state the scope "any repair". What it does not state is how the section reaches an adopting project, and a80 places the reading at "the audit's sort and at the intake's item f". The adopting project's missing analysis step, where item f would sit, is what the known issue records ("A project that receives a retrospective's findings has no installed procedure to analyse them"). The draft's narrowing repair would narrow the spec's owner-confirmed "any repair". So it is a change to put to the owner, not a rewording, and the outcome "owner list" stands. Not on the kept list, and no ruling in the history settles it. |

**The evidence**, verbatim from `L2-d/draft-2-repair-standard-scope-two-readings.md`, from its first section on:

> **Evidence**
>
> The instruction, `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md` lines 152-155:
>
> > **Repairs to agent-facing text the section "repair-standard"**
> >
> > What any repair of an instruction is judged by, whether a reading or a real session found its
> > defect.
>
> and, in the same section, `:174-176`: "**When the repairs applied together add
> more instructions than they remove, the owner is told so first, with the net figure**, and rules on
> that growth before any of them is applied."
>
> The moment: a session in a project that adopted the workflow repairs its own root `CLAUDE.md`
> after a real session found a defect in it. The path the installed text gives that defect:
> `path@agent-config@skills/knowledge-architect-retrospective/SKILL.md` lines 124-125, "**The project's file**: each finding the owner keeps becomes an issue entry in the project's own
> [... the draft continues to line 61, before its proposed repair; quoted here: lines 7-22]

**The repair** (`repairs/K7/proposal.md`, lines 48-105)

- **Edit 1, audit SKILL 154-155** (source 158-159; C16): "What any repair of an instruction is
  judged by, whether a reading or a real session found its defect." becomes "What a repair of an
  instruction is judged by, in an audit and wherever a skill names this section, whether a reading
  or a real session found its defect."
- **Edit 2, outside the corpus**: `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`,
  "What would close it", gains "and whether an installed analysis names
  `skill@knowledge-architect-project-audit@repair-standard`, whose scope sentence names the texts
  that deliver it".

**Instructions**: added 0, removed 0.

**The nearest rival**: "Reading 1 kept and delivered: a pointer to
`skill@knowledge-architect-project-audit@repair-standard` added to agent-configuration's
the section "content-and-style" [...] Count: 1 instruction added, 0 removed." The deciding facts: "The rival
is the alternative the spec records as lost, on a80", and "The rival adds an instruction to an
installed skill on a finding of reading (two readings), not on evidence from real use".

**The check** (`repairs/K7/check.md`): "Defects the new text creates (L1, L3): 1"; heads 0;
rejected alternatives 0; goals 0.

- **Defect 1** (L3, weak): "wherever a skill names this section" reads as a step of a skill that
  names it (A) or "a repair of an instruction *in* a skill that names the section" (B), under which
  an ordinary edit of the audit skill or the intake owes the count and the net growth. Repair: "in
  an audit, and at each step of a skill that names this section, whether ...".
- Note on the record: "The edit narrows the words of an approved decided design that is not yet
  harvested", spec 1927-1929, "any repair of an instruction found by reading or by real use"; "the
  owner item should say that it narrows the approved words "any repair"", and "the same commit
  should correct the decided design's sentence to the new scope".

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 110:

> | K7 | C16 (W, new; known in part) | **owner list** | A two-reading scope sentence; nothing in force states the reading meant, and the edit narrows the spec's approved words "any repair ... by reading or by real use". Default: Edit 1 with check D1's wording, "in an audit, and at each step of a skill that names this section, whether ...", and Edit 2 (the issue entry amended). The owner item says it narrows the approved words; spec 1927-1929 corrected in place, merged with K6's correction (1.2). |

Default: Edit 1 in the check's wording, and Edit 2; spec 1927-1929 corrected in place.

### K8. The record check's "the goals" reads as every goal or as the owning Component's goals

From `merge/causes.md`, K8:

> - **Kind**: none.
> - **Mechanism**: in the record check's sentence "the goals, and the rejected alternatives of the
>   Component that owns the text", the qualifier can bind the rejected alternatives alone or the
>   goals too; under the second reading a repair to installed text is never read against a root
>   goal, while the auditor's check brief ("the goals homes") leans to the first.

**Findings**

#### C20 (W, new text)

- **Draft kept**: `L3-a/04-record-check-whose-goals.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): ""of the Component that owns the text" attaches to the rejected alternatives alone or to the goals too".
- **Class decided by** (dedup): project-audit SKILL.md 178 182: blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | 04-record-check-whose-goals.md | verified | none | SKILL.md:178-182, auditor:196-198, intake:94-96 and 114, and design skill:429-430 match. "of the Component that owns the text" attaches either to the rejected alternatives alone or to both the goals and the rejected alternatives. One error in the evidence that does not change the finding: `grep -n "^## " docs/goals.md` gives ten root goals, not nine. The "refine" sentence is goal-setting:53, inside ##goal-entry-shape. |

**The evidence**, verbatim from `L3-a/04-record-check-whose-goals.md`, from its first section on:

> **Evidence**
>
> `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md` lines 178-182:
>
> > **The record check.** Each repair is read against the record of what it edits before the owner
> > rules on it: the design heads that `cargo klarch show` and a search of the design homes return for
> > the behaviour the edited text describes, the goals, and the rejected alternatives of the Component
> > that owns the text. The check names each head the repair would reverse, each rejected alternative
> > it brings back and each goal it strains, with its quotation.
>
> The check brief that carries it out, `path@agent-config@agents/knowledge-architect-workflow-auditor.md` lines 196-198:
> "against the record, as `skill@knowledge-architect-project-audit@repair-standard` says: for this you
> read the design homes, the goals homes and the rejected alternatives beside the corpus". The intake
> also applies it: `path@agent-config@skills/klarch-retrospective-intake/SKILL.md` lines 94-96, "The fix item f settles
> on is read against the record as the record check of
> `skill@knowledge-architect-project-audit@repair-standard` says."
> [... the draft continues to line 54, before its proposed repair; quoted here: lines 7-22]

**The repair** (`repairs/K8/proposal.md`, lines 45-95)

- **Edit 1, audit SKILL 178-181** (source 182-185; C20): "the design heads that `cargo klarch
  show` and a search of the design homes return for the behaviour the edited text describes, the
  goals, and the rejected alternatives of the Component that owns the text." becomes "every goals
  home, the design heads that `cargo klarch show` and a search of the design homes return for the
  behaviour the edited text describes, and the rejected alternatives of the Component that owns
  the text." The reading is stated by auditor 196-198, "the goals homes".

**Instructions**: added 0, removed 0.

**The nearest rival**: the draft's own, "the goals of the project's root and of the Component that
owns the text, and that Component's rejected alternatives". The deciding fact: the agent that runs
the check "is told [...] to read "the goals homes", every one [...] The rival is narrower than all
three".

**The check** (`repairs/K8/check.md`): defects 0; heads 0; rejected alternatives 0; goals 0. Note
1: spec 1945-1947 states the same sentence in the old words; "Proposed addition to the edit set:
the same rewording at spec lines 1945-1947". Note 2: "A factual error in the proposal's own record
check": "The root holds ten" goals, not nine, "and the gates and xtask homes are omitted"; "five
homes, 22 goals".

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 111:

> | K8 | C20 (W, new) | **applied in the branch** | A two-reading instruction whose reading meant is stated by another instruction, the auditor's the section "check-a-proposal" ("the goals homes"). Check: 0 defects. Add the same rewording at spec 1945-1947 (check note 1), merged with K1's correction of that paragraph. The proposal's goal count is wrong (check note 2: 10 root goals, 5 homes); it is in no edit. |

Default: applied in the branch, with the same rewording at spec 1945-1947.

### K9. The places a session reads for what is already known are listed separately in several texts

From `merge/causes.md`, K9:

> - **Kind**: two homes for one moment.
> - **Mechanism**: the check before diagnosing, and the standing-entry searches at grounding and at a
>   design audit, each carry their own list of places that hold outstanding work, so the audit
>   report directory the primer's table added reaches only the root's list (C9), and the development
>   skill's copy of the root's list names two of its four places with no pointer (C12).

**Findings**

#### C12 (P, old text)

- **Draft kept**: `L2-b/d3-development-grounding-restates-the-already-known-check-without-pointer.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "klarch-development restates the already-known check with no pointer and names two of the root's four places".
- **Class decided by** (dedup): klarch-development SKILL.md 40 41: 0bb6b18, [P2-3R], ancestors.

**The verifier**, its line verbatim:

> | d3-development-grounding-restates-the-already-known-check-without-pointer.md | verified | none | klarch-development SKILL.md:40-41 restates the check of whether a defect is recorded with no pointer in its sentence, and names two of the four places the root's section (CLAUDE.md:349-357) gives this repository, against PRIMER.md:92-93 and 96-97. A weaker point the session should weigh: line 43 points to `primer@intent-and-claims`, read whole, which holds the primer's form of the directive, so the defect is the left-out root additions more than a missing home. Not on the kept list: item 7's entry concerns the design grounding and the design audit, not this moment. No ruling of an earlier pass covers the sentence; F65 of pass 2 edited its paragraph and left it. |

**The evidence**, verbatim from `L2-b/d3-development-grounding-restates-the-already-known-check-without-pointer.md`, from its first section on:

> **Evidence**
>
> .claude/skills/klarch-development/SKILL.md:40-41, in "Ground before editing":
>
> > A behaviour that looks like a new defect is often recorded: `cargo klarch issues` and
> > `cargo klarch tripwires` list every entry.
>
> Its homes. CLAUDE.md:349-357, under "Check whether it is already known":
>
> > Before diagnosing anything as a problem, or reporting one you met while doing something else, find
> > out whether it is already recorded, per `primer@intent-and-claims`. The listings, every option they
> > take, and why they and not a grep are the check, are
> > `skill@knowledge-architect-issue-tracking@outstanding-across-registers`. This repository adds two:
> >
> > ```sh
> > ls docs/retrospective-reports/    # the retrospective analyses whose outcomes are not all carried out
> > ```
> [... the draft continues to line 55, before its proposed repair; quoted here: lines 16-31]

#### C9 (W, new text)

- **Draft kept**: `L2-a/draft-06-a-pending-audit-report-is-read-by-no-installed-search.md`. **Merged into it**: L4-b/d3-pending-audit-report-reached-by-no-installed-search.md (plausible; none).
- **Verdict and marks** (dedup): plausible; none.
- **Shared defect** (dedup): "a pending audit report file is outstanding work that no installed search reads".
- **Class decided by** (dedup): PRIMER.md 115 115 and CLAUDE.md 356 356: blamed [S9], phrase absent.

**The verifier**, its line verbatim:

> | draft-06-a-pending-audit-report-is-read-by-no-installed-search.md | plausible | none | Quotations match (primer 115; design 432-436; planning 352-353; CLAUDE.md 356; standing-entry-searcher take-your-group runs only `issues` and `tripwires`). `grep -rn audit-reports .claude CLAUDE.md` finds no installed search. Not known: `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search` covers docs/retrospective-reports/ only, and its premise that installed text cannot name the directory does not hold for the installed row. D32 ([D32-35]) ruled only where the root lists it. Predicted, since report files are normally deleted on the audit's own branch. |

**The evidence**, verbatim from `L2-a/draft-06-a-pending-audit-report-is-read-by-no-installed-search.md`, from its first section on:

> **Evidence**
>
> The moments: loop step 1 of `skill@knowledge-architect-design@discussion-loop` (the grounding) and
> point 2 of `skill@knowledge-architect-planning@working-a-slice` (the design audit), in a project that
> adopted the workflow and has a report file of a project audit on its main branch.
>
> .claude/knowledge-architect/PRIMER.md:115 (the installed knowledge table):
> > | the evidence of one pass of a project audit, and the owner's ruling on each of its items | one file per pass in docs/audit-reports/ at the project's root, as `skill@knowledge-architect-project-audit@record-of-an-audit` says | the commit that carries out its last outcome, or a later commit of the same branch, deletes it |
>
> So a report file stays while one of its outcomes is not carried out.
>
> .claude/skills/knowledge-architect-design/SKILL.md:432-436:
> > The open issues and
> > the tripwires are searched by subagents: dispatch the search as the
> > description of `agent@knowledge-architect-standing-entry-searcher` says,
>
> [... the draft continues to line 45, before its proposed repair; quoted here: lines 14-29]

**The repair** (`repairs/K9/proposal.md`, lines 36-160)

- **Edit 1, `path@agent-config@skills/klarch-development/SKILL.md` 40-41** (a project skill, edited in
  place; C12): "A behaviour that looks like a new defect is often recorded: `cargo klarch issues`
  and `cargo klarch tripwires` list every entry." becomes "A behaviour that looks like a new defect
  is often recorded: check it as `instructions@verify-before-relying` says, in its subsection
  "Check whether it is already known", read whole."
- **Edit 2, a new issue entry** (C9):
  `path@elsewhere@crates/agent-skills/docs/open-issues/a-pending-audit-report-is-reached-by-no-installed-search.md`,
  kind `question`, titled "A pending audit report file is outstanding work that no installed search
  of what is already known reads"; its "Why it matters" names
  `design@agent-skills@standing-entries-searched-before-the-work`; its "What would close it": "A
  retrospective of a session, in any project that keeps a pending audit report file, that
  discussed a design or audited a slice or a spec bearing on one of the file's ruled outcomes."
  (full text at proposal lines 77-150).

**Instructions**: added 0, removed 0.

**The nearest rival**: "the cause merged now on the installed side too." "Name
docs/audit-reports/ in `skill@knowledge-architect-issue-tracking@outstanding-across-registers`
[...]". The deciding
fact: "C9 has no real-use evidence", and the axis says a predicted gap "is never repaired by an
edit". Edit 2's rival, widening the agent-config entry, loses on the fix site: "One entry would
hold two fixes in two anchors and two closing conditions."

**The check** (`repairs/K9/check.md`): defects 0; heads 0; rejected alternatives 0; goals 0; 5
notes. Note 1: "read that subsection whole" removes a doubt "at no cost". Note 2: the proposal's
ground against its second rival "is the wrong sentence of the primer". Note 3: "The premise of the
new entry has no route in the audit skill": a report file is "normally deleted before the merge".
Note 4: the entry's closing fix "is a decision, not a direct edit", to say in the entry. Note 5:
`tripwire@agent-skills@pointer-not-followed` watches Edit 1.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 112:

> | K9 | C12 (P, old), C9 (W, new, predicted gap) | **C12 applied in the branch**; **C9 an issue entry** | C12: a partial restatement converted to a pointer. Take the check's wording "read that subsection whole". C9: a predicted gap; the new agent-skills entry. Add to its "What would close it" that the fix changes `design@agent-skills@standing-entries-searched-before-the-work`, argued under the design skill (check note 4). If the commit quotes the rival's reason, use the check's corrected reason (note 2). |

Default: Edit 1 with "read that subsection whole"; the new entry, its "What would close it" naming
the change of `design@agent-skills@standing-entries-searched-before-the-work` under the design
skill.

### K10. The in-change acceptance criterion falls under planning's "Each names", whose items are a plan document's

From `merge/causes.md`, K10:

> - **Kind**: two homes for one moment.
> - **Mechanism**: the acceptance criterion of the in-change path is stated by design step 7 (applied
>   as a test, its label in the test's comment) and named again in planning's acceptance-criteria
>   section, whose next sentence, "Each names", asks for a thread reference and a step that the
>   in-change path has not; `design@agent-skills@acceptance-criteria-in-the-document` states the
>   plan-document reading.

**Findings**

#### C5 (W, new text)

- **Draft kept**: `L2-a/draft-01-in-change-criterion-owes-a-thread-reference.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): ""Each names" can take in an in-change acceptance criterion, which has no thread reference and no step".
- **Class decided by** (dedup): planning SKILL.md 292 302: 294-296 blamed [P5] (not [S9], as the draft says), phrase absent; 299-302 old; takes the newer.

**The verifier**, its line verbatim:

> | draft-01-in-change-criterion-owes-a-thread-reference.md | verified | none | Quotations match (design 586-589, 596-598, 244-246; planning 36, 225-228, 292-302, 330-339; agent-skills design.md 1111-1113). [S9] inserted "One of the in-change path ... is step 7's." right before "Each names:", so "Each" can take in the in-change criterion, whose thread reference and step have no form (`git diff [S7] [S9]`). New text. `design@agent-skills@acceptance-criteria-in-the-document` states reading 2, so "applied" fits. |

**The evidence**, verbatim from `L2-a/draft-01-in-change-criterion-owes-a-thread-reference.md`, from its first section on:

> **Evidence**
>
> The moment: a design discussion on the in-change path whose premortem keeps a cause only the built
> code can check; the owner rules it applied as AC<n>; the session writes the test that applies it.
>
> .claude/skills/knowledge-architect-design/SKILL.md:586-589:
> > criterion, which `skill@knowledge-architect-planning` writes into the plan document,
> > or which, on the in-change path, the change applies as a test.
>
> .claude/skills/knowledge-architect-design/SKILL.md:596-598:
> > tripwire's entry, and the criterion's item or, on the in-change path,
> > the comment at its test, keep that label in their
> > text, so the ruling in the transcript is found from either.
>
> .claude/skills/knowledge-architect-planning/SKILL.md:292-302:
> > An acceptance criterion is a bet about a decision the work is built on, [...] It lives in the plan
> [... the draft continues to line 63, before its proposed repair; quoted here: lines 9-24]

**The repair** (`repairs/K10/proposal.md`, lines 38-103)

- **E1, planning SKILL 295-297** (source 301-303; C5): "`skill@knowledge-architect-design@discussion-loop`
  step 7's. Each names:" becomes "`skill@knowledge-architect-design@discussion-loop` step 7's. Each
  acceptance criterion of a plan document names:". The reading is that of
  `design@agent-skills@acceptance-criteria-in-the-document`.

**Instructions**: added 0, removed 0: "It states the scope the instruction had before pass 5
inserted the pointer sentence".

**The nearest rival**: "**Move the in-change pointer sentence after the list**". The deciding
fact: "the antecedent of "Each" would still be the paragraph's subject, "An acceptance criterion",
which is the general class. [...] Both readings of C5 survive."

**The check** (`repairs/K10/check.md`): "Counts: defects the new text creates 0; heads reversed 0;
rejected alternatives brought back 0; goals strained 0." Its note: "after E1, no text states that
an in-change criterion names its response or its observable in the comment at its test [...] E1
removes no settled obligation."

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 113:

> | K10 | C5 (W, new) | **applied in the branch** | A two-reading instruction whose reading `design@agent-skills@acceptance-criteria-in-the-document` states. Check: 0 defects. The proposal's met-outside item (the in-change head lacks "acceptance criteria") is for the session to route. |

Default: applied in the branch.

### K11. A requested change that converges before its code exists meets both path conditions

From `merge/causes.md`, K11:

> - **Kind**: none.
> - **Mechanism**: the in-change path is open when the decision lands in "the change under way", and
>   the full path takes work "nobody has started"; a requested change discussed under bounded work's
>   loop step 1 and converging before any of its code exists is both, so the session may write a
>   spec with its four plan reviews or none.

**Findings**

#### C7 (W, old text)

- **Draft kept**: `L2-a/draft-04-a-requested-change-s-discussion-has-no-stated-path.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): ""the change under way" against "nobody has started it" leaves both paths open for a requested change that converges before its code exists".
- **Class decided by** (dedup): design SKILL.md 147 150: 77ee033, 7c614bc, ancestors; 170 172 and 184 185: 3a10fc9, ancestor.

**The verifier**, its line verbatim:

> | draft-04-a-requested-change-s-discussion-has-no-stated-path.md | verified | none | Quotations match (design 125-127, 147-150, 170-172, 184-185; agent-skills design.md 1059-1061). "the change under way" against "nobody has started it" allows both acts for a requested change that converges before any of its code exists. Line 184-185 allows either path. The head's argument covers decisions "met during a task". No ruling found: F90 ([P4]) covers approved bounded work, not an unbounded request, and `git log -G'nobody has started'` names only 77ee033. |

**The evidence**, verbatim from `L2-a/draft-04-a-requested-change-s-discussion-has-no-stated-path.md`, from its first section on:

> **Evidence**
>
> The moment: the owner asks for a change whose design is not settled. The session starts the design
> skill at loop step 1, as bounded work says; the grounding shows the work is not bounded; the
> discussion converges in the same session, before any code of the request is written, on work one
> branch and one PR will hold. The session must now choose a path: write a spec and dispatch the
> plan reviews, or implement in this change with the deliberation in its commit message.
>
> .claude/skills/knowledge-architect-design/SKILL.md:170-172:
> > Whether a request is a design question or bounded work, a clear requirement whose main risk is
> > over-building, is known only after grounding. So a requested change whose design is not settled
> > starts here, at loop step 1, the search for standing entries included.
>
> .claude/skills/knowledge-architect-design/SKILL.md:184-185:
> > A reply that brings an argument, a new shape or a doubt returns the work to the loop, on the
> > in-change path or the full path.
> [... the draft continues to line 61, before its proposed repair; quoted here: lines 12-27]

**The repair** (`repairs/K11/proposal.md`, lines 94-137)

- **One edit, design SKILL 125-127** (source 138-140; C7): "change under way: its work, as when
  the decision was met during an issue fix or a review repair, or its record" becomes "change under
  way: its work, in a change begun before the discussion opened, as an issue fix or a review repair
  is, or its record". Reading 2, argued from `design@agent-skills@adopting-existing-docs` ("The move
  itself is planned work of its own, written under the planning skill").

**Instructions**: added 0, removed 0; "It still changes what a session that read reading 1 does,
so its outcome is the owner list".

**The nearest rival**: "**Rival: state reading 1**, the draft's default. [...] "or nobody has
started it" would become "or no change under way will build it"". The deciding fact: "Reading 1
gives that small-project move the in-change path, against the head [adopting-existing-docs]".
On what the owner decides between them: "the weighing. Reading 2 costs a spec and four plan
reviews for every requested change that is not bounded".

**The check** (`repairs/K11/check.md`): "defects the new text creates 1; heads the repair would
reverse 1 (in part: it narrows the head's entry condition and leaves the head unedited); rejected
alternatives brought back 0; goals strained 1."

- **D1** (L3): "does "in a change begun before the discussion opened" bind the record clause
  too?" Under reading B a keep verdict discussed in a fresh session has no in-change path, against
  the keep-or-change verdict record (design 715-716). Repair: "its work, where that change began
  before the discussion opened, as an issue fix or a review repair does, or its record, whenever
  the discussion opened, for a decision that no work implements [...]".
- **H1**: `design@agent-skills@in-change-path`, "its entry condition narrowed in the skill, the head
  left as it is"; its argument, "When the work lands in the change under way, neither applies",
  argues against the narrowing, so per the primer's backstop it goes to the design skill, and "the
  item put to the owner should name that the entry condition is the subject of a standing
  tripwire", `tripwire@agent-skills@ruling-lost-in-change`.
- **G1**: `goal@knowledge-architect@agents-get-a-complete-workflow`, "fewer review rounds": "every
  requested change that is not bounded and is built in one pull request by the converging session
  owes a spec, its plan reviews, its harvest and its deletion".

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 114:

> | K11 | C7 (W, old) | **owner list** | Nothing recorded states which path a requested change takes, and either reading changes what agents do. Check H1: the default (reading 2) narrows the entry condition of `design@agent-skills@in-change-path`, against that head's argument; by the primer's backstop it is argued under `skill@knowledge-architect-design` (in-change path) and the head is edited in the same commit. Check D1: the new clause can bind the record member too, which contradicts the keep-or-change verdict record (design 715-716); use the check's wording. Name to the owner: G1 (`goal@knowledge-architect@agents-get-a-complete-workflow`, a spec and its reviews for every unbounded requested change) and `tripwire@agent-skills@ruling-lost-in-change`, which reserves a reopening of that head "on its entry condition". Default: reading 2 as reworded, with the head's edit; the alternative is reading 1 with setup step 4 brought into line. |

Default: reading 2 in check D1's wording, with the head edited in the same commit under the design
skill's in-change path; the alternative is reading 1 with setup step 4 brought into line.

### K12. A plan document leaves in the commit that completes its harvest, before acts that still need it

From `merge/causes.md`, K12:

> - **Kind**: a structure the work did not need.
> - **Mechanism**: planning fixes the spec's deletion in the commit that completes its harvest, and
>   the harvest's review is the review before the merge; so the spec is gone from the tree that
>   review's spec-conformity axis reads (C11), and a thread-to-slug pair written into the harvest row
>   in that same commit enters no diff that `git log -G` finds (C8).

**Findings**

#### C8 (W, old text)

- **Draft kept**: `L2-a/draft-05-a-pair-written-in-the-deleting-commit-never-enters-history.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): plausible; none.
- **Shared defect** (dedup): "a thread-to-slug pair written at the harvest into a plan document the same commit deletes is found by no `git log -G`".
- **Class decided by** (dedup): PRIMER.md 303 310: d95135a, 118fe7f, d95e227, ea03dda, ancestors; planning SKILL.md 409 411: 546faad, ancestor.

**The verifier**, its line verbatim:

> | draft-05-a-pair-written-in-the-deleting-commit-never-enters-history.md | plausible | none | Quotations match (primer 303-310; planning 409-411, 419, 433; decision-recording 117-118). The probe, re-run in a scratch repository, prints nothing for -G'old-thread -> new-slug', and -G'new-slug' finds only design.md's line. The texts allow the head, the pair and the deletion in one commit. Whether a session writes the pair only at harvest is a prediction no session has shown. Not known: no issue entry in the listing matches. |

**The evidence**, verbatim from `L2-a/draft-05-a-pair-written-in-the-deleting-commit-never-enters-history.md`, from its first section on:

> **Evidence**
>
> The moment: the harvest of a spec (or a slice), point 6 of
> `skill@knowledge-architect-planning@working-a-slice`. The session writes a head for an approved
> thread and judges, as the primer asks at that moment, that the thread's name misdescribes the
> decision as approved, so the entry takes another slug.
>
> .claude/knowledge-architect/PRIMER.md:303-310:
> > **When the decision was a thread of a design discussion, its slug is the thread's name**, unless
> > that name misdescribes the decision as approved, [...] When the name misdescribes the decision, the entry takes
> > a slug that names the decision, by the alignment rule below, and the text that keeps the
> > deliberation states the pair, `#<thread> → <entry slug>`: the plan document's harvest row, or the
> > commit message on the in-change path. `git log --grep` on either name then finds a commit message
> > that states the pair, and `git log -G` a plan document's diff that does.
>
> .claude/skills/knowledge-architect-planning/SKILL.md:409-411 (the harvest):
> [... the draft continues to line 55, before its proposed repair; quoted here: lines 10-25]

#### C11 (W, old text)

- **Draft kept**: `L2-b/d2-spec-conformity-at-the-merge-review-reads-a-deleted-spec.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): plausible; none.
- **Shared defect** (dedup): "the spec is deleted before the review before the merge, whose spec-conformity axis still reads it".
- **Class decided by** (dedup): planning SKILL.md 415 419 and 431 433: [P4], 65263d1, ancestors; review SKILL.md 34 34: 37dc8c6, ancestor.

**The verifier**, its line verbatim:

> | d2-spec-conformity-at-the-merge-review-reads-a-deleted-spec.md | plausible | none | The moment exists (planning SKILL.md:415-419 and 431-433: the spec is deleted before the review before the merge, whose spec-conformity condition, review SKILL.md:34, still holds), but no reading shows a reviewer misled: the range's diff carries the deleted spec as removed lines, and planning SKILL.md:543-544 gives `git show <commit>^:<path>`, so a dispatcher can name it. A predicted gap on installed text. issue@agent-skills@the-transcript-reviewer-cannot-see-a-plan-document-its-range-deletes records the same deletion for two other axes only, so it is related, not known. |

**The evidence**, verbatim from `L2-b/d2-spec-conformity-at-the-merge-review-reads-a-deleted-spec.md`, from its first section on:

> **Evidence**
>
> The review before the merge owes every axis whose condition holds,
> .claude/skills/knowledge-architect-review/SKILL.md:46-50:
>
> > the work has properties these axes do not reach. **A review owes every axis of this table whose
> > condition holds, and every axis that the skill of an activity whose work it reviews adds, where
> > that axis's condition holds.** The review before a merge is the last review of a branch, after
> > which only its own repairs land, whichever activity's skill dispatched it, such as the review of a
> > plan document on a branch of its own; the work it reviews is the whole branch.
>
> Two axes whose standard is the plan document, same file, lines 34 and 36:
>
> > | spec conformity | does the work implement what was decided, item by item. If deviations happened during implementation, are they justified? | a spec or a milestone slice's spec was written before the work |
>
> > | fidelity of relocation | where content moved, was anything lost? A reason dropped, a number changed, an argument compressed to an assertion | content was relocated, or forked from another source |
> [... the draft continues to line 68, before its proposed repair; quoted here: lines 10-25]

**The repair** (`repairs/K12/proposal.md`, lines 73-271): no edit of the corpus, "since a
predicted gap [...] is never repaired by an edit".

- **Edit 1**: `issue@agent-skills@the-transcript-reviewer-cannot-see-a-plan-document-its-range-deletes`
  renamed to `a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it` and
  rewritten, kind `deferred`, title "A plan document deleted in the commit that completes its
  harvest is unseen by the acts that still need it", with four cases: the transcript reviewer, the
  standing-state reviewer, "The spec-conformity and fidelity-of-relocation axes" (C11), and "The
  thread-to-slug pair" (C8); its repairs proposed: "**The structure removed**: the spec is deleted
  in a commit of its own after the review before the merge has read the tip that holds it" and
  "**The briefs extended**" (full text at proposal lines 134-224).
- **Edits 2 to 4**: the old id re-pointed in the provable-gap entry (line 35), in the spec (724,
  1386, 2190, 2751), and the index regenerated.

**Instructions**: added 0, removed 0.

**The nearest rival**: "**Two entries for the two findings' sites**". The deciding fact: "the four
cases share one structure, and one change of that structure [...] closes all four, while the
brief-side repair that the existing entry proposes closes cases 1 to 3 and not case 4".

**The check** (`repairs/K12/check.md`): "defects the new text creates **7**; heads reversed **0**;
rejected alternatives brought back **0**; goals strained **0**."

- **X1**: the Summary and the Trigger state case 1's parking two ways; the title states as fact
  what the Summary marks as predicted.
- **X2**: "The rewritten Trigger drops a clause the owner approved, and names a retrospective as
  deciding a repair": the old Trigger's "or a review before a merge shows the transcript reviewer
  reporting an argument kept only in the deleted spec" is gone; the old entry is F105's outcome.
  Repair: "keep the old clause beside the new one, generalised to the four cases [...] and name the
  intake, not the retrospective, as the deciding occasion; or put the narrowing to the owner."
- **X3**: "Case 4's reproduction is described and not given (cold-reader standard)".
- **X4**: "Why it matters omits the decisions case 4 strains directly":
  `design@agent-skills@thread-slug-is-entry-id` and `design@agent-skills@standing-argument-in-head`.
- **X5**: "Why it matters attributes to the head a limit its text does not state" ("on the main
  branch").
- **X6**: "Edit 2 retargets a sentence about the past", against issue-tracking 297-299.
- **X7**: "The structural route lists less of the installed text than it would change" (planning
  419, 433, 512, and the primer's pair paragraph).
- Note: "Kept list item 8 names the old id; a rename makes the next pass's kept list name the new
  one."

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 115:

> | K12 | C8, C11 (W, old, predicted gaps) | **an issue entry**: the existing entry widened and renamed | Predicted gaps, never repaired by an edit. Check: 7 defects in the entry's text, X1 to X7, each to repair before applying. X2 matters most: the rewritten trigger drops "a review before a merge shows the transcript reviewer reporting an argument kept only in the deleted spec", which F105 ([P5]) approved and which fires at each review before a merge; keep it, generalised to the four cases. X3: the reproduction inline, per the cold-reader standard. X6: the re-pointed sentence of the provable-gap entry rewritten to the present. Kept.md item 8 names the old id for the next pass. |

Default: the entry widened and renamed, X1 to X7 repaired first, X2's clause kept.

### K13. What waits when a ruling's premise proves false is stated in three texts

From `merge/causes.md`, K13:

> - **Kind**: two homes for one moment.
> - **Mechanism**: the primer proceeds on a default and holds only the part that cannot be undone,
>   planning stops the whole slice for a material load-bearing gap, and the design skill names the
>   design-audit exception for itself while the primer names none.

**Findings**

#### C6 (W, old text)

- **Draft kept**: `L2-a/draft-03-a-load-bearing-gap-stops-where-the-primer-proceeds-on-a-default.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; bears-on-ruling (F58, commit [P2-3R]).
- **Shared defect** (dedup): "a material load-bearing gap stops the slice in planning, while primer@owner-word-premise proceeds on a default".
- **Class decided by** (dedup): PRIMER.md 21 25: blamed 61a9219, 9eead6c, ancestors; planning SKILL.md 346 349: [P2-3R], [P4], ancestors; 382 385: dc108a0, 37dc8c6, d76cf14, ancestors.

**The verifier**, its line verbatim:

> | draft-03-a-load-bearing-gap-stops-where-the-primer-proceeds-on-a-default.md | verified | bears-on-ruling | Quotations match (primer 21-25; planning 47, 346-349, 382-385; design 648-651). A load-bearing gap that is material defeats a premise of an approved thread, so primer@owner-word-premise (proceed on the default) and planning (stop the slice) both apply, and no move satisfies both. The design skill states its own exception; the primer states none. Bears on F58 ([P2-3R], "one home at the opening of working-a-slice" for what waits): that one-home premise is false, since the primer's premise rule, ruled in 9eead6c ("the premise rule proceeds on its default"), predates it and also says what waits. The session settles it. |

**The evidence**, verbatim from `L2-a/draft-03-a-load-bearing-gap-stops-where-the-primer-proceeds-on-a-default.md`, from its first section on:

> **Evidence**
>
> The moment: point 2 of `skill@knowledge-architect-planning@working-a-slice`, the design audit of a
> slice or a spec. The audit finds that a premise an approved thread rested on is false: a gap that is
> material by planning's own term. Both texts below apply: the primer reaches every session, and the
> planning skill is loaded for the audit.
>
> .claude/knowledge-architect/PRIMER.md:21-25:
> > **A word of the owner holds only as far as its premise.** A ruling given on a premise, stated by
> > the owner or supplied by the session, does not rule on the case where that premise is false. When a
> > premise proves false, put the corrected premise to the owner at the top of the turn, quoting the
> > word it defeats, with a default chosen in view of the corrected premise, and proceed on the default
> > unless the owner answers otherwise; a part of the work that cannot be undone waits for the answer.
>
> .claude/skills/knowledge-architect-planning/SKILL.md:47 (term "material"):
> > a finding made after a thread closed that defeats a reason the closure gave, a premise it rested on, or a criterion it claimed to meet.
> [... the draft continues to line 52, before its proposed repair; quoted here: lines 17-32]

**The repair** (`repairs/K13/proposal.md`, lines 37-92)

- **Edit 1, planning SKILL 346-349** (source 356-359; C6): "that puts it to the owner names one: it
  stops the slice." becomes "that puts it to the owner names one: it stops the slice, and where the
  gap defeats a premise of a ruling, this stop replaces the default that `primer@owner-word-premise`
  proceeds on."
- **Edit 2, design SKILL 648-651** (source 664-667): "which stops that work under
  `skill@knowledge-architect-planning`." becomes "which stops that work as
  `skill@knowledge-architect-planning@working-a-slice` says."

**Instructions**: added 0, removed 0: Edit 1 "states which of two existing instructions governs,
which is the intent F58 ruled".

**The nearest rival**: "**the exception stated in the primer**". The deciding fact: "that clause
changes the rule that `design@agent-skills@primer-content` states for the primer [...] It also puts
into the text every session reads a case that only a session with the planning skill loaded meets,
against `design@agent-skills@primer-limit`". A second rival, "delete the stop", "reverses F58".

**The check** (`repairs/K13/check.md`): "Defects the new text creates (L1, L3): 1"; heads 0;
rejected alternatives 0; goals 0.

- **D1** (L3, minor): "this stop replaces the default" reads as replacing the proceeding (A, the
  message still names a default) or the default itself (B). Repair: "it stops the slice, and where
  the gap defeats a premise of a ruling, the slice stops instead of proceeding on the default that
  `primer@owner-word-premise` names."
- Residual, not created: design the section "decision-authority" 83-84, "Two cases defeat that default, both
  narrow", while the design-audit stop is a third.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 116:

> | K13 | C6 (W, old; bears on F58) | **owner list**: which side of a contradiction wins, with F58's corrected premise quoted (section 3) | Default: Edit 1 with check D1's wording, "the slice stops instead of proceeding on the default that `primer@owner-word-premise` names", and Edit 2. Net 0. The check's residual, design the section "decision-authority" 83-84 "Two cases defeat that default", is old text the repair leaves: met outside the task. |

**Bears on a ruling** (C6; cross-check, section 3):

From `cross-check/cross-check.md`, lines 144-157:

> **C6 (cause K13): F58 of passes 2 and 3**
>
> - Source, message of [P2-3R]: "F58, what waits for an owner who has not ruled: one home at the
>   opening of `skill@knowledge-architect-planning@working-a-slice`; a listed default is built, a
>   scope change waits, a load-bearing gap of the audit stops the slice. 4 added, 4 removed." The
>   owner's word: "I agree with the full list from F58 onward".
> - The premise: that paragraph is the one home of what waits.
> - Why it may be false: `primer@owner-word-premise` also says what waits for any false premise of
>   the owner's word, and says the opposite for the material case: "proceed on the default unless
>   the owner answers otherwise". The owner ruled that rule before F58, in 9eead6c, whose subject
>   reads "the premise rule proceeds on its default".
> - Default: K13, planning's stop wins for a load-bearing gap that defeats a premise; the primer's
>   rule governs every other case.
>

Default: Edit 1 in check D1's wording, and Edit 2; net 0.

### K14. The fold's rebase onto `origin/main` also rebases onto a moved base

From `merge/causes.md`, K14:

> - **Kind**: none.
> - **Mechanism**: the root's fold command, `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash
>   origin/main`, rebases onto `origin/main`, which any fetch moves; after a fetch the fold also moves
>   the branch's base, and the empty-diff check the bullet prescribes fails with no content lost
>   (reproduced by the draft on git 2.43.0).

**Findings**

#### C10 (P, old text)

- **Draft kept**: `L2-b/d1-fold-rebases-onto-a-moved-origin-main.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; bears-on-ruling (F44 of pass 2, commit [P2]).
- **Shared defect** (dedup): "the fold's `rebase -i --autosquash origin/main` rebases onto a moved origin/main, so the empty-diff check fails with no content lost".
- **Class decided by** (dedup): CLAUDE.md 542 545: [P2], ancestor.

**The verifier**, its line verbatim:

> | d1-fold-rebases-onto-a-moved-origin-main.md | verified | bears-on-ruling (F44 of pass 2, commit [P2], "All defaults approved for F28 to F57.") | Reproduced: after `git fetch` moves origin/main, `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash origin/main` exits 0, rebases the branch onto the new main and leaves `git diff <old-head> HEAD` non-empty (`m \| 1 +`), so the check of CLAUDE.md:544 fails with no content lost, against CLAUDE.md:527-532's own split between a kept base and a moved one; the merge-base form leaves the diff empty. F44 was approved on a probe "to fold and leave `git diff <old-head> HEAD` empty", a premise false in this case: the session settles it. |

**The evidence**, verbatim from `L2-b/d1-fold-rebases-onto-a-moved-origin-main.md`, from its first section on:

> **Evidence**
>
> CLAUDE.md:542-545 (Git, point 2):
>
> > - **To fold a repair into an earlier commit**: commit the repair as `git commit --fixup=<commit>`,
> >   then, with a clean tree, run `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash origin/main`,
> >   which needs no editor. Check that `git diff <old-head> HEAD` is empty and that the branch holds one
> >   commit fewer; `git rebase --autosquash` without `-i` folds nothing and exits 0.
>
> CLAUDE.md:527-532 (same point) distinguishes the two cases the fold bullet merges:
>
> > - **With a clean tree, editing the branch's own history is an ordinary move**: rewording or
> >   amending a commit message, squashing, rebasing. It is bounded by two verifications before the
> >   old head is dropped: that the tree was clean, and that no content was lost. For a rewrite that
> >   keeps the base, `git diff <old-head> HEAD` is empty. For a rebase onto a moved base, compare the
> >   branch's own delta instead: `git range-diff <old-base>..<old-head> <new-base>..HEAD` reports
> [... the draft continues to line 86, before its proposed repair; quoted here: lines 15-30]

**The repair** (`repairs/K14/proposal.md`, lines 23-69)

- **One edit, root CLAUDE.md the section "git-workflow" point 2, 542-545** (C10): "run
  `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash origin/main`, which needs no editor." becomes
  "run `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash --keep-base origin/main`, which needs no
  editor and keeps the branch's base wherever `origin/main` has moved since." The probe (git 2.43.0,
  proposal lines 23-39): the current form leaves "`m | 1 +`: not empty"; `--keep-base` and the
  merge-base form leave it empty.

**Instructions**: added 0, removed 0. "Word count of the bullet: 58 before, 69 after (`wc -w`)."

**The nearest rival** is the draft's repair: "the same edit with the upstream
`$(git merge-base origin/main HEAD)` in place of `--keep-base origin/main`". The deciding fact:
"`--keep-base` is git's documented option for this exact case [...] That makes the margin small.
The cost of `--keep-base` is a git version of 2.24.0 or later."

**The check** (`repairs/K14/check.md`): defects 0; heads 0; rejected alternatives 0; goals 0. A
wording observation, not counted: "wherever" can be read two ways and "since" has no antecedent;
"A plainer clause would be "and keeps the branch's base even when `origin/main` has moved"". Its
probe confirmed the case; it notes a `--keep-base` fix in git 2.36.2 that "concerns the form with
an explicit branch, which the new command does not use".

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 117:

> | K14 | C10 (P, old; bears on F44) | **applied in the branch**: a factual error in the project's own text | F44's corrected premise goes to the owner at the top of the list, with this repair as the default, per `primer@owner-word-premise`; the session proceeds on it unless the owner answers. Check: 0 defects; optional plainer clause "even when `origin/main` has moved". Name the cost: git 2.24 or later; the merge-base form is the version-free rival. |

**Bears on a ruling** (C10; cross-check, section 3):

From `cross-check/cross-check.md`, lines 158-169:

> **C10 (cause K14): F44 of pass 2**
>
> - Source, message of [P2]: "F44 the root CLAUDE.md's fold procedure,
>   `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash`, probed on git 2.43 to fold and leave
>   `git diff <old-head> HEAD` empty, where `--autosquash` without `-i` folds nothing". The owner's
>   word, same message: "All defaults approved for F28 to F57."
> - The premise: the probe's `origin/main` was at the branch's base.
> - Why it is false: any `git fetch origin main` moves `origin/main`; the fold then rebases onto it
>   and the empty-diff check fails with no content lost. Reproduced by the draft and by K14's probe
>   on git 2.43.0.
> - Default: K14, `--keep-base origin/main`.
>

Default: the edit applied; F44's corrected premise at the top of the owner list.

### K15. The audit a pin move owes is placed "after the pin's commit", with no base and no session named

From `merge/causes.md`, K15:

> - **Kind**: none.
> - **Mechanism**: step 5 of moving the pin, which the audit skill's trigger points to, places the
>   audit "after the pin's commit, in a branch of its own", which states neither whether that branch
>   starts from the pin's branch or from the main branch once the pin lands (C38), nor whether it
>   runs in the session that ran the install, whose harness has not registered the agents and the
>   saved workflow the install added (C15).

**Findings**

#### C38 (W, old text)

- **Draft kept**: `L2-d/draft-3-audit-after-the-pins-commit-two-readings.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): ""after the pin's commit, in a branch of its own" leaves open whether the audit branch starts from the pin's branch or from the main branch once the pin lands".
- **Class decided by** (dedup): setup SKILL.md 279 281: b3f8515, ancestor.

**The verifier**, its line verbatim:

> | draft-3-audit-after-the-pins-commit-two-readings.md | verified | none | Quotations hold: setup SKILL.md:275-281, project-audit SKILL.md:43-45 and :114-117. The head `design@agent-skills@audit-triggers` says only "runs it after the pin's commit" (crates/agent-skills/docs/design.md:1500). No text of the corpus fixes the base of the audit's branch. Searches: `grep -rn -i "pin's commit\|after the pin"` over the corpus and the agent-skills design home, and `grep -n -i "branch of its own"` over the spec, find no other site and no thread on it. The two acts differ: what the audit branch's review range `<main branch>..<head>` holds, and whether that branch is rebased after the pin branch lands. Neither breaks another instruction. The finding still stands as two readings under D34, approved by the owner in commit [D32-35], which rules that a confirmed two-readings finding is repaired as the axis's table says and is not left to judgement. No earlier pass or report file rules on this phrase, and it is not on the kept list. No tripwire's firing evidence: `tripwire@agent-skills@axis-not-run-at-a-pin-move` fires on an axis not run, not on the branch's base. |

**The evidence**, verbatim from `L2-d/draft-3-audit-after-the-pins-commit-two-readings.md`, from its first section on:

> **Evidence**
>
> - `path@agent-config@skills/knowledge-architect-setup/SKILL.md` lines 279-281, step 5 of moving the pin: "For each axis that step 2 listed, run it under `skill@knowledge-architect-project-audit`, after
>   the pin's commit, in a branch of its own."
> - `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md` lines 43-45: "The axis runs after the pin's commit, in a branch of its own, as
>   `skill@knowledge-architect-setup@moving-the-pin` says."
> - Step 4, `path@agent-config@skills/knowledge-architect-setup/SKILL.md` lines 275-277, commits the pin on the pin
>   move's branch, which is then reviewed and merged under the project's rules
>   (`skill@knowledge-architect-review`, "before merging to the main branch").
>
> The moment: step 5, once step 4's commit exists and before the pin move's branch has merged.
>
> The two acts the words allow:
>
> 1. "after the pin's commit": the session opens the audit's branch at once, from the pin move's
>    branch, whose commit it needs, since the audit reads the record under the new version's rules.
> [... the draft continues to line 35, before its proposed repair; quoted here: lines 7-22]

#### C15 (W, old text)

- **Draft kept**: `L2-d/draft-1-new-audit-agent-unregistered-at-pin-move.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): plausible; bears-on-ruling (F94 of pass 4, commit [P4]).
- **Shared defect** (dedup): "a pin move that installs the audit agents and runs the axis in one session dispatches an agent type the harness did not register".
- **Class decided by** (dedup): setup SKILL.md 272 281: d76cf14, 65263d1, [P2-3R], b3f8515, ancestors; project-audit SKILL.md 332 334: [S7], ancestor; 65 67: 66-67 old, 65 blamed [P5], phrase absent but carries only "follows once the audit stops. A session"; review SKILL.md 96-97, blamed [P5], are evidence of the harness fact, not defect lines.

**The verifier**, its line verbatim:

> | draft-1-new-audit-agent-unregistered-at-pin-move.md | plausible | bears-on-ruling (pass 4, commit [P4] "[audit] Pass 4 ...", F94) | Every quotation holds: setup SKILL.md:272-281, CHANGELOG.md:47-49, project-audit SKILL.md:65-67, 190-192, 200, 332-334, review SKILL.md:95-97, the saved workflow's line 142 `agentType: 'knowledge-architect-workflow-auditor'`; `git ls-tree --name-only v0.5.0 crates/agent-skills/content/agents/` lists eight agents with neither auditor, and `.../skills/` lists no project-audit. Whether the harness registers an agent type that an install adds mid-session is not established by any reading, so the gap stays predicted. Not known: `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, third case, states the opposite ("The subagents the session dispatches are covered"), and the draft's proposed outcome corrects that claim. Bears on F94 of pass 4, approved by the owner: "the sentence asking a session to reload the harness's skills is deleted; a session whose harness does not list the saved workflow dispatches the same agents by hand". The draft puts that premise in doubt for an agent the install adds, and its fallback repair, "from a session started after the install", would partly restore what F94 deleted. The session settles this. Not on the kept list. Lens and class (L2 walk, W, predicted gap) are correct; the proposed outcome, an issue entry, is the axis table's row for a predicted gap. |

**The evidence**, verbatim from `L2-d/draft-1-new-audit-agent-unregistered-at-pin-move.md`, from its first section on:

> **Evidence**
>
> The moment: a project pinned at 0.5.0 moves its pin to the version the `Next release` section
> describes, and runs `skill@knowledge-architect-setup@moving-the-pin` steps 3 to 5 in one session.
>
> - `path@agent-config@skills/knowledge-architect-setup/SKILL.md` lines 272-274, step 3: "Run `cargo klarch install-agent-skills`, then follow
>   `skill@knowledge-architect-agent-configuration` for what an upgrade owes the project's own
>   configuration."
> - same file, `:279-281`, step 5: "For each axis that step 2 listed, run it under `skill@knowledge-architect-project-audit`, after
>   the pin's commit, in a branch of its own." No step says to start a new session.
> - `CHANGELOG.md:47-49`, a Migration entry of `Next release`: "the design homes and the rejected alternatives a project holds are brought
>   to the rules of `primer@design-heads` and to its new entry test 2, by running the design-record
>   axis, `skill@knowledge-architect-project-audit@design-record-axis`, after the pin moves."
> - `path@agent-config@skills/knowledge-architect-project-audit/SKILL.md` lines 190-192: "**The order of a Component's entries, and the inputs read beside each head**, are in
>   `agent@knowledge-architect-design-record-auditor`'s description, which the harness lists: cut the
>   groups by that order." and `:200`: "**The agent**: `agent@knowledge-architect-design-record-auditor`."
> [... the draft continues to line 56, before its proposed repair; quoted here: lines 7-22]

**The repair** (`repairs/K15/proposal.md`, lines 20-177)

- **Edit 1, setup SKILL 279-281** (source 282-284; C38): "run it under
  `skill@knowledge-architect-project-audit`, after the pin's commit, in a branch of its own." becomes
  "run it under `skill@knowledge-architect-project-audit`, in a branch of its own started from the
  main branch once the pin move has merged into it, so that the audit's range holds the audit's
  work alone."
- **Edit 2, audit SKILL 43-47**: "The axis runs after the pin's commit, in a branch of its own, as
  `skill@knowledge-architect-setup@moving-the-pin` says." becomes "The axis runs when and where
  `skill@knowledge-architect-setup@moving-the-pin` says."
- **Edit 3, CHANGELOG.md `Next release`**: "after the pin's commit, in a branch of its own" becomes
  "in a branch of its own started from the main branch once the pin move has merged".
- **The issue entry for C15**: `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`
  extended: its title gains "and of the agents it adds"; the false sentence "The subagents the
  session dispatches are covered" becomes "A subagent of an agent type the session already had
  registered is covered"; a fourth case added, "an agent type the crossed version adds"; its "What
  would close it" gains "The fourth case closes on a retrospective of a session that moved a pin
  across a version adding an audit agent and ran the axis".

**Instructions**: added 0, removed 0. Edit 1 "changes what agents are told, so it goes to the
owner"; Edit 2 "replaces a restatement of edit 1's instruction by a pointer to it".

**The nearest rival**: "reading 1 stated: "in a branch of its own, started from the pin move's
branch right after the pin's commit, and rebased onto the main branch once the pin move merges"".
The deciding fact: "the pin's commit is not final until the pin move merges" (a repair is folded
into it under `primer@branch-commits`), and the audit's review range would hold the pin move's
commits. For C15, the draft's clause ", from a session started after the install" "partly restores
what F94 deleted".

**The check** (`repairs/K15/check.md`): "1 defect the new text creates; 0 heads reversed; 0
rejected alternatives brought back; 0 goals strained."

- **D1**: "Edit 2 replaces an allowed one-sentence restatement with a bare pointer that the skill's
  self-sufficiency test does not pass". Repair: "The axis runs in a branch of its own started from
  the main branch once the pin move has merged into it, as
  `skill@knowledge-architect-setup@moving-the-pin` says."
- N1: audit SKILL 52's "unless the owner gives it to another work's branch" may or may not reach a
  pin move's audit. N2: the issue's new title, "A session holds no copy of an agent the install
  adds"; wording "or the session's own copy of the new text, nor the agent types it adds". N3: Edit
  1 puts a merge between steps 4 and 5 that no step names.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 118:

> | K15 | C38 (W, old), C15 (W, old, predicted gap; bears on F94) | **C38 owner list**; **C15 an issue entry** | C38: a two-reading instruction, nothing recorded states the base. Default: Edits 1 and 3, and Edit 2 replaced by check D1's one-sentence restatement with its pointer, "The axis runs in a branch of its own started from the main branch once the pin move has merged into it, as `skill@knowledge-architect-setup@moving-the-pin` says.", since the bare pointer fails the self-sufficiency test. C15: the extension of `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, with check N2's title wording; F94's premise goes to the owner (section 3). |

**Bears on a ruling** (C15; cross-check, section 3):

From `cross-check/cross-check.md`, lines 186-199:

> **C15 (cause K15): F94 of pass 4**
>
> - Source, message of [P4]: "F94 (K23): the sentence asking a session to reload the harness's
>   skills is deleted; a session whose harness does not list the saved workflow dispatches the same
>   agents by hand. 1 removed." The owner's word, same message: "I agree with the full list from F72
>   onward, apply them."
> - The premise: the session that ran the install can dispatch the audit's agents.
> - Why it may be false: `skill@knowledge-architect-review@review-invariants` says a subagent "runs on
>   its agent definition as the harness registered it when the session started"; an agent type the
>   install adds may have no registered type. No reading establishes it, so the finding is plausible
>   and a predicted gap. The only observation, in [P4], is that "in this session the new
>   workflow was listed right after its install", which concerns the workflow, not an agent.
> - Default: K15, an issue entry; the reload sentence is not restored.
>

Default: C38 on the owner list, Edits 1 and 3, Edit 2 replaced by check D1's sentence; C15 the
issue's extension with check N2's title wording.

### K16. Installed restatements carry no pointer, and their homes changed after them

From `merge/causes.md`, K16:

> - **Kind**: a restatement that drifted from its home.
> - **Mechanism**: four texts restate a directive of another text in their own words with no pointer
>   beside it, so `cargo klarch show` on the home does not list them, and a clause the home gained
>   (the goals exception of setup, the after-install trigger of agent-configuration, the
>   room-to-judge exclusion of the retrospective's defect kinds) did not reach the copy; the
>   decision-record reviewer's copy of the tripwire directive agrees with its home but has no
>   pointer.

**Findings**

#### C22 (W, old text)

- **Draft kept**: `L5-a/d1-retrospective-setup-expectation-drops-the-goals-exception.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the retrospective's setup expectation says a run without an owner makes the choices itself, dropping the setup's "writes no goal"".
- **Class decided by** (dedup): retrospective SKILL.md 188 191: b5f5ef2, 4feb940, ancestors; setup SKILL.md 44 46: 4feb940, [P1], ancestors.

**The verifier**, its line verbatim:

> | d1-retrospective-setup-expectation-drops-the-goals-exception.md | verified | none | retrospective SKILL.md:188-191 says a run without an owner "makes those choices itself", the goals among them; setup SKILL.md:44-46 says such a run "writes no goal". Blame: setup 45-46 from [P1] (pass 1, F6), retrospective 188-191 from b5f5ef25/4feb940e. F6 ruled the setup side only; no ruling covers the copy. |

**The evidence**, verbatim from `L5-a/d1-retrospective-setup-expectation-drops-the-goals-exception.md`, from its first section on:

> **Evidence**
>
> The home, `skill@knowledge-architect-setup` (the skill that owns how setup is done),
> .claude/skills/knowledge-architect-setup/SKILL.md:41-46:
>
> > **Setting up is done with the owner.** Every step that writes a durable statement on the owner's
> > behalf is shown to the owner first: the Components, the goals, the place of each existing document,
> > and each choice below that changes the project's build or a file it already has. The owner rules;
> > the agent proposes. The skill is built on an owner present to rule: a run without one, such as a
> > trial, makes those choices itself, except the goals, which only the owner states: such a run writes
> > no goal, and leaves each goals home with no entry, awaiting the owner's.
>
> The restatement, `skill@knowledge-architect-retrospective@expectation-sets`,
> .claude/skills/knowledge-architect-retrospective/SKILL.md:188-191:
>
> > - **The owner is present to rule.** The setup proposes and the owner rules: the Components, the
> [... the draft continues to line 46, before its proposed repair; quoted here: lines 11-26]

#### C23 (W, old text)

- **Draft kept**: `L5-a/d2-primer-skill-list-drops-the-after-install-trigger.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the primer's list of installed skills drops agent-configuration's after-install trigger".
- **Class decided by** (dedup): PRIMER.md 373 374: d76cf14, ancestor.

**The verifier**, its line verbatim:

> | d2-primer-skill-list-drops-the-after-install-trigger.md | verified | none | PRIMER.md:373-374 lists only "before editing a `CLAUDE.md`, a skill or an agent"; the skill's description (agent-configuration SKILL.md:3) and its section the section "after-installing" add "after an install that changes the installed files". `git log -S` gives [P2-3R] alone; no other corpus text points to the after-install section. |

**The evidence**, verbatim from `L5-a/d2-primer-skill-list-drops-the-after-install-trigger.md`, from its first section on:

> **Evidence**
>
> The home, the skill's own trigger, .claude/skills/knowledge-architect-agent-configuration/SKILL.md:3:
>
> > description: MUST use before adding or editing the project's root CLAUDE.md, a scoped CLAUDE.md
> > next to code, a project skill, or a project subagent definition, and after an install that changes
> > the installed files. ...
>
> and its section `skill@knowledge-architect-agent-configuration@after-installing`, line 141:
> "## After an install that changes the installed files".
>
> The restatement, `primer@installed-skills`, .claude/knowledge-architect/PRIMER.md:373-374:
>
> > - `skill@knowledge-architect-agent-configuration`: before editing a `CLAUDE.md`, a skill or an
> >   agent.
>
> [... the draft continues to line 42, before its proposed repair; quoted here: lines 10-25]

#### C27 (P, old text)

- **Draft kept**: `L5-a/d6-intake-kinds-restate-the-retrospective-s-without-room-to-judge.md (P side)`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the intake restates the retrospective's defect kinds with no pointer and drops the room-to-judge exclusion".
- **Class decided by** (dedup): klarch-retrospective-intake SKILL.md 98 99: 45441b3, ancestor.

**The verifier**, its line verbatim:

> | d6-intake-kinds-restate-the-retrospective-s-without-room-to-judge.md (P side) | verified | none | intake SKILL.md:98-99 restates retrospective SKILL.md:41-45 with no pointer, and drops "A decision the session made by its own judgement where no instruction covers it is not missing". D34 dropped an item d change about the wrong-act test, not this pointer: no ruling covers it. |
> | d6-intake-kinds-restate-the-retrospective-s-without-room-to-judge.md (W side) | refuted | none | the installed home, retrospective SKILL.md:41-45, states the exclusion and points to `primer@room-to-judge`; no installed text needs a change, as the draft itself says. |

**The evidence**, verbatim from `L5-a/d6-intake-kinds-restate-the-retrospective-s-without-room-to-judge.md`, from its first section on:

> **Evidence**
>
> The home, `skill@knowledge-architect-retrospective@what-it-examines`,
> .claude/skills/knowledge-architect-retrospective/SKILL.md:40-45:
>
> > subjects below, list what was **unclear** (it could be read two ways), **missing** (the session
> > could not follow the workflow, or could not produce a document the checker accepts, without an
> > instruction that does not exist), or **wrong** (following it produced a defect or a correction). A
> > decision the session made by its own judgement where no instruction covers it is not missing: the
> > installed text leaves that room on purpose, per `primer@room-to-judge`.
>
> The restatement, `skill@klarch-retrospective-intake@what-to-establish`, item d,
> .claude/skills/klarch-retrospective-intake/SKILL.md:98-99:
>
> > - **a defect of the text**: unclear (two readings), missing (the session needed an instruction
> >   that does not exist), or wrong (following it produced a defect or a correction);
> [... the draft continues to line 39, before its proposed repair; quoted here: lines 10-25]

#### C29 (W, old text)

- **Draft kept**: `L5-a/d9-decision-record-reviewer-tripwire-predicate-narrower-than-its-home.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified (narrowed to the missing pointer; the drift claim refuted); none.
- **Shared defect** (dedup): "the decision-record reviewer restates the tripwire directive of decision-recording with no pointer".
- **Class decided by** (dedup): decision-record-reviewer.md 108 110: d3ba396, ancestor.

**The verifier**, its line verbatim:

> | d9-decision-record-reviewer-tripwire-predicate-narrower-than-its-home.md | verified | none | narrowed: decision-record-reviewer.md:108-110 restates the directive of decision-recording SKILL.md:242-243 with no pointer beside it, which D3 makes a finding. Its drift claim is refuted: line 109-110, "A tripwire the diff adds with no ruling recorded ... is a finding", reaches a parked thread's tripwire too. Only the timing clause, "with the decision it guards", is absent, and the reviewer reads the skill whole as its standard (lines 17-21). The proposed pointer still repairs it. |

**The evidence**, verbatim from `L5-a/d9-decision-record-reviewer-tripwire-predicate-narrower-than-its-home.md`, from its first section on:

> **Evidence**
>
> The home, `skill@knowledge-architect-decision-recording@ruled-tripwires`,
> .claude/skills/knowledge-architect-decision-recording/SKILL.md:242-247:
>
> > **A
> > tripwire is written only on the owner's word, and with the decision it guards**, whatever named
> > it, a parked thread included: in the change that records that decision, which on the full path of
> > `skill@knowledge-architect-design` is the harvest, or in a later change once that decision is
> > recorded. It goes in the tripwires home of the Component that owns the guarded decision, so the
> > decision's head exists before the tripwire that names it.
>
> The restatement, `agent@knowledge-architect-decision-record-reviewer@record-predicates`,
> .claude/agents/knowledge-architect-decision-record-reviewer.md:108-110:
>
> > **Does a tripwire stand on the owner's word?** A tripwire a premortem or an acceptance criterion
> [... the draft continues to line 48, before its proposed repair; quoted here: lines 11-26]

**The repair** (`repairs/K16/proposal.md`, lines 29-175)

- **E1, retrospective SKILL 188-191** (source 205-208; C22): "makes those choices itself, and a gap
  it reports" becomes "makes those choices itself, the goals excepted, as
  `skill@knowledge-architect-setup` says, and a gap it reports".
- **E2, `path@agent-skills@README.md` 189-194**, outside the corpus: the README's copy gains
  "except the goals, which it leaves for you to state".
- **E3, PRIMER 373-374** (source 377-378; C23): "before editing a `CLAUDE.md`, a skill or an agent."
  becomes "before editing a `CLAUDE.md`, a skill or an agent; after an install that changes the
  installed files."
- **E4, intake 98-99** (C27): "**a defect of the text**: unclear (two readings), missing (the
  session needed an instruction that does not exist), or wrong (following it produced a defect or a
  correction);" becomes "**a defect of the text**: unclear, missing or wrong, as
  `skill@knowledge-architect-retrospective@what-it-examines` defines the three, read whole;".
- **E5, decision-record reviewer 108-110** (source the same; C29): "A tripwire a premortem or an
  acceptance criterion produced is written only where the owner ruled that it should be." becomes
  "Judge each tripwire the diff adds by
  `skill@knowledge-architect-decision-recording@ruled-tripwires`."

**Instructions**: added 0, removed 0; E5 "counted as a rewording of one instruction [...] A stricter
count reads it as 1 removed and 1 added, net 0 either way."

**The nearest rival**: "keep the copies as they are and add one instruction where a home is
edited [...] "after editing a directive, run `cargo klarch show` on its home and bring every
restatement it lists into line"". The deciding fact: "three of the four copies, C22, C27 and C29,
carried no pointer to their home. An edit-time instruction keyed on what `show` lists could not
have reached them."

**The check** (`repairs/K16/check.md`): "defects the new text creates **1**; heads reversed **0**;
rejected alternatives brought back **0**; goals strained **0**."

- **D1** (L3, with an L1 edge): E5's "Judge each tripwire the diff adds by
  `...@ruled-tripwires`" reads as the whole section, which includes where a tripwire goes, a
  routing question the reviewer's scope excludes ("not _which_ document family owns it, which is
  the routing axis"), or as the owner's word alone. Repair: "Judge each tripwire the diff adds by
  `skill@knowledge-architect-decision-recording@ruled-tripwires`: whether the owner's word stands
  behind it, and whether it is written with the decision it guards."
- N1: "the fact the proposal gives against that rival is false for one entry": the primer's design
  entry lacks two triggers of its description. N2: `pointer-not-followed` watches E4 and E5. N3: E4
  leaves a judgement call with no kind of item d, which judgement decides. N4: a `Next release`
  entry and the install are owed.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 119:

> | K16 | C22, C23, C27, C29 (old) | **applied in the branch**: drifted restatements brought back, a pointer added | Check D1: E5 reads two ways and one reading crosses the reviewer's scope; take the check's second wording, "whether the owner's word stands behind it, and whether it is written with the decision it guards". E2, the README, outside the corpus, in the same commit. Check N1: the primer's design entry also lacks a trigger of its description ("reverting to multiple-choice menus ..."), the cause's defect in a further text no draft reports; the session may add it to E3, as K17 added its Edit 3. |

Default: applied in the branch, E5 in check D1's second wording, E2 in the same commit.

### K17. The corpus states the checker's behaviour in its own words, and the statements are false of the checker or of their named home

From `merge/causes.md`, K17:

> - **Kind**: a restatement that drifted from its home.
> - **Mechanism**: the primer, the root `CLAUDE.md` and issue-tracking state what `cargo klarch check`
>   and `cargo klarch index` do from the checker's design heads or README, without the exceptions and
>   scope those carry: a slug in a fence is said to be reported, which the checker passes (C31, C32);
>   `index` is said to run no check, while it runs phases 1 to 3 and refuses on a finding (C33); and
>   the root says a reference is checked in Markdown and Rust only, under a named home that says
>   "wherever it stands" (C28).

**Findings**

#### C31 (W, old text)

- **Draft kept**: `L7-a/D1-fenced-slug-reported.md (W side)`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the primer says a misplaced slug is reported, but the checker passes a fenced one".
- **Class decided by** (dedup): PRIMER.md 298 301: 6c660b8, [P1], ancestors.

**The verifier**, its line verbatim:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L7-a/D1-fenced-slug-reported.md | W (PRIMER.md:300, source crates/agent-skills/content/PRIMER.md:304) | verified | none | Every quotation matches the worktree. A scratch copy of the mock project dirhome, checked with target/release/klarch 0.5.0 built at [S9], reports a plain slug as misplaced (exit 1) but passes a fenced `### ... the section "fenced-one"` in a design subdocument and a fenced `## ... the section "fenced-two"` in README.md (exit 0, "PASSED: no findings"). The primer's "reports it as a misplaced definition" is false for a fenced slug. Outcome: applied, edited at its source per kept item 1. |
> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L7-a/D1-fenced-slug-reported.md | P (CLAUDE.md:121) | verified | none | Same run. The root restatement copies the title of design@core@a-slug-elsewhere-defines-nothing (crates/core/docs/design.md:1311) and omits the fence exception its body states at 1329-1330. Outcome: applied. Caveat on the draft's instance: by CLAUDE.md:121-123, a level-two slug in a skill, which is a section home of the harness, would be a definition, not misplaced. So "by both quotations ... reports each as a misplaced definition" overstates the root side. The checker still does neither, so the finding stands. |

**The evidence**, verbatim from `L7-a/D1-fenced-slug-reported.md`, from its first section on:

> **Evidence**
>
> The corpus says a slug outside a definition site is reported:
>
> - `CLAUDE.md:121-124` (project text, a restatement of `design@core@a-slug-elsewhere-defines-nothing`, per its
>   section's opening "This section is a restatement"):
>   "A slug anywhere else defines nothing and is reported as a misplaced definition: at a heading
>   of another level, in a table cell, at the head of a plain line, in the middle of a line, in a
>   file that is no register home and no section home of the harness, per
>   `design@core@a-slug-elsewhere-defines-nothing` and `design@core@section-homes-carry-slugs`."
> - `path@agent-config@knowledge-architect/PRIMER.md` lines 298-300 (installed; source `path@agent-skills@content/PRIMER.md` lines 302-304):
>   "Nowhere else: a slug at another heading level, in a table cell, at the head
>   of a plain line, in the middle of a line or in a file that is not the design home defines nothing.
>   `cargo klarch check` reports it as a misplaced definition; a reference to it resolves to nothing."
>
> The checker passes over a slug inside a fenced block, and its home says so, in the body of the head
> [... the draft continues to line 71, before its proposed repair; quoted here: lines 9-24]

#### C32 (P, old text)

- **Draft kept**: `L7-a/D1-fenced-slug-reported.md (P side)`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the root restatement omits the fence exception of design@core@a-slug-elsewhere-defines-nothing".
- **Class decided by** (dedup): CLAUDE.md 120 123: bd93004, 415df73, ancestors.

**The verifier**, its line verbatim:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L7-a/D1-fenced-slug-reported.md | W (PRIMER.md:300, source crates/agent-skills/content/PRIMER.md:304) | verified | none | Every quotation matches the worktree. A scratch copy of the mock project dirhome, checked with target/release/klarch 0.5.0 built at [S9], reports a plain slug as misplaced (exit 1) but passes a fenced `### ... the section "fenced-one"` in a design subdocument and a fenced `## ... the section "fenced-two"` in README.md (exit 0, "PASSED: no findings"). The primer's "reports it as a misplaced definition" is false for a fenced slug. Outcome: applied, edited at its source per kept item 1. |
> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L7-a/D1-fenced-slug-reported.md | P (CLAUDE.md:121) | verified | none | Same run. The root restatement copies the title of design@core@a-slug-elsewhere-defines-nothing (crates/core/docs/design.md:1311) and omits the fence exception its body states at 1329-1330. Outcome: applied. Caveat on the draft's instance: by CLAUDE.md:121-123, a level-two slug in a skill, which is a section home of the harness, would be a definition, not misplaced. So "by both quotations ... reports each as a misplaced definition" overstates the root side. The checker still does neither, so the finding stands. |

**The evidence**: the same draft as C31, quoted there.

#### C33 (W, old text)

- **Draft kept**: `L7-a/D2-index-with-no-check.md`. **Merged into it**: L7-b/draft-03-index-is-said-to-run-no-check.md (verified; none).
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "issue-tracking says `cargo klarch index` writes with no check, but it runs phases 1 to 3 and refuses on a finding".
- **Class decided by** (dedup): issue-tracking SKILL.md 157 160: [P2], a1371dd, ancestors.

**The verifier**, its line verbatim:

> | /tmp/claude-266682/-home-catA-tb266682-Documents-code-knowledge-architect/ef462278-a887-481e-aae0-8d187b6d6c22/scratchpad/pass6/L7-a/D2-index-with-no-check.md | W | verified | none | Quotations match the skill (installed line 160, source line 160), crates/core/README.md:118-121 and `klarch index --help`. In the scratch mock, an unstaged deletion of docs/open-issues/a-entry.md made `klarch index` exit 2 with "the model is incomplete: phase 2 ... Nothing was written". "with no check" is false. Outcome: applied, at the source. |

**The evidence**, verbatim from `L7-a/D2-index-with-no-check.md`, from its first section on:

> **Evidence**
>
> `path@agent-config@skills/knowledge-architect-issue-tracking/SKILL.md` lines 157-160 (source
> `path@agent-skills@content/skills/issue-tracking/SKILL.md` lines 160):
>
> "Delete an entry with `git rm`, or stage its deletion: the check reads git's listing, and a deletion
> it still lists stops the run in phase 2, before the index is written.
>
> `cargo klarch index` writes the generated files alone, with no check."
>
> The checker's README, `path@core@README.md` lines 118-121: "`index`, `index --staged` over the staged tree,
> `check --fix` and a writing command of an extension run the first three phases too, and write no
> generated file while one of them holds anything. `index` and an extension's command refuse with exit 2".
>
> Run (scratch copy of the mock project `dirhome`, pin set to 0.5.0, binary
> `path@knowledge-architect@target/release/klarch` built from the worktree at [S9]):
> [... the draft continues to line 39, before its proposed repair; quoted here: lines 9-24]

#### C28 (P, old text)

- **Draft kept**: `L5-a/d8-root-reference-scope-disagrees-with-its-named-home.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; known (issue@core@references-are-read-in-markdown-and-rust-only).
- **Shared defect** (dedup): "the root says references are checked in Markdown and Rust only, its named home says wherever they stand".
- **Class decided by** (dedup): CLAUDE.md 262 264: [P3], ancestor.

**The verifier**, its line verbatim:

> | d8-root-reference-scope-disagrees-with-its-named-home.md | verified | known: `issue@core@references-are-read-in-markdown-and-rust-only` | CLAUDE.md:243 names `design@agent-skills@a-reference-claims-a-revisit` as home; CLAUDE.md:262-264 says Markdown and Rust only; the head (agent-skills design.md:861) says "checked wherever it stands"; walk.rs:131 `LIVE_SUFFIXES = ["md", "rs"]`. The root is right and the head is false of the code: outside the axis. The issue records the limit, and its Why it matters does not name the head. The proposed outcome extends that entry. |

**The evidence**, verbatim from `L5-a/d8-root-reference-scope-disagrees-with-its-named-home.md`, from its first section on:

> **Evidence**
>
> The restatement, `instructions@where-knowledge-goes`, subsection "When to write a reference",
> CLAUDE.md:243:
>
> > This is a restatement. Its home is `design@agent-skills@a-reference-claims-a-revisit`.
>
> and CLAUDE.md:262-264:
>
> > A reference in prose is checked in Markdown and Rust source, a Rust comment and a fenced block
> > included, per `primer@where-knowledge-goes`. So a Rust comment that names an issue is reached when
> > the issue closes.
>
> The named home, crates/agent-skills/docs/design.md:861-862 (`cargo klarch show
> design@agent-skills@a-reference-claims-a-revisit`):
>
> [... the draft continues to line 49, before its proposed repair; quoted here: lines 14-29]

**The repair** (`repairs/K17/proposal.md`, lines 47-203)

- **Edit 1, PRIMER 300** (source 304; C31): "`cargo klarch check` reports it as a misplaced
  definition; a reference to it resolves to nothing." becomes "Outside a fenced block, `cargo
  klarch check` reports it as a misplaced definition; a reference to it resolves to nothing."
- **Edit 2, root CLAUDE.md 121-124** (C32): "A slug anywhere else defines nothing and is reported as
  a misplaced definition:" becomes "A slug anywhere else defines nothing, and outside a fenced block
  it is reported as a misplaced definition:".
- **Edit 3, issue-tracking 241-242** (no finding; the cause's defect in a third text): "defines
  nothing and is reported as misplaced." becomes "defines nothing, and outside a fenced block it is
  reported as misplaced."
- **Edit 4, issue-tracking 160** (C33): "`cargo klarch index` writes the generated files alone,
  with no check." becomes "`cargo klarch index` writes the generated files alone, without the
  check's last phase: like `cargo klarch check --fix`, it writes no generated file while the first
  three phases find anything."
- **Edit 5, outside the corpus** (C28): `issue@core@references-are-read-in-markdown-and-rust-only`,
  "Why it matters", gains a paragraph naming `design@agent-skills@a-reference-claims-a-revisit`, and
  "What would close it" gains ", and the head `design@agent-skills@a-reference-claims-a-revisit`
  states the scope the walk reads".

**Instructions**: added 0, removed 0: "Edits 1 to 4 each change a claim about what the checker does,
a fact".

**The nearest rival**: "**Delete the clauses that state the checker's report, instead of
qualifying them**". The deciding fact: "the installed setup skill writes fenced slugs on purpose
(the section "setup-gates", 198-212). So the fence exception is not one of item 5's rarer rules." A second
rival, "make the checker report a fenced slug", "reverses `design@core@a-slug-elsewhere-defines-nothing`".

**The check** (`repairs/K17/check.md`): defects 0; heads 0; rejected alternatives 0; goals 0; 4
notes. Its probes confirmed the fence exception and the `index` gate, and a conflicted `index.md`
regenerated by `klarch index` with exit 0. Note 1: "end the first clause with a full stop" in Edit
1. Note 2: Edit 5 is not C28's outcome ("nothing"); "The session should record it under that case
[case 2 of `primer@met-outside-the-task`], not as C28's outcome", and its sentence on the root
"can be dropped without loss". Note 3: "C28 stays in the corpus" as the known finding. Note 4:
planning 130-131 and setup 216 say `check --fix` "then checks", an imprecision of old text.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 120:

> | K17 | C31 (W), C32 (P), C33 (W), all old; C28 (P, old, known) | **C31, C32, C33 applied in the branch** (factual errors, a drifted restatement), with Edit 3; **C28 nothing** (known, `issue@core@references-are-read-in-markdown-and-rust-only`) | Check: 0 defects; its probes confirm the fence exception and the `index` gate, and found no contradiction with point 3 on a conflicted index. Edit 5 is not C28's outcome: route it as case 2 of `primer@met-outside-the-task` (the entry's "Why it matters" names no decision), without its sentence on the root CLAUDE.md (check note 2). Optional, check note 1: a full stop in Edit 1. |

Default: Edits 1 to 4 applied; C28 nothing; Edit 5 routed as case 2 of `primer@met-outside-the-task`,
without its sentence on the root CLAUDE.md.

### K18. The rule against a `:line` or `#fragment` suffix is stated nowhere, and the checker's messages name another repair

From `merge/causes.md`, K18:

> - **Kind**: none.
> - **Mechanism**: by the owner's ruling, the rule is delivered by the checker's finding message
>   rather than by an installed text, and for the anchored forms the message names "repair the
>   pointer" or "define it" instead of dropping the suffix, so the ruling's premise is false and no
>   text supplies the rule. The defect is in the checker's messages, outside the corpus.

**Findings**

#### C34 (W, old text)

- **Draft kept**: `L7-b/draft-01-anchored-reference-suffix-message-names-no-repair.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; kept (kept list item 5); bears-on-ruling (pass 1 F23, pass 2 F69).
- **Shared defect** (dedup): "the rule against a `:line` or `#fragment` suffix is in no corpus text, and the anchored forms' finding messages name no repair".
- **Class decided by** (dedup): the rule is absent; the sites checked are PRIMER.md 136 137 (the reference paragraph): [P1], ancestor, and klarch-retrospective-intake SKILL.md 141 141 (the moment, "file:line"): 45441b3, ancestor.

**The verifier**, its line verbatim:

> | draft-01-anchored-reference-suffix-message-names-no-repair.md | verified | kept (kept list item 5, "a `:line` or `#fragment` suffix"); bears-on-ruling (pass 1 F23, the rarer checker rules left to their finding messages, and pass 2 F69, an unstated rule is a finding only where its message does not name its repair: history.txt lines 666-667, 857-858, 1019-1020) | Quotations match crates/core/src/check/references.rs:161-162 and :191-192 and klarch-retrospective-intake/SKILL.md:141. Reproduced: target/release/klarch check in the auditor's scratch project p4 prints the six findings quoted, exit 1; the anchored forms get "repair the pointer, or delete it" or "define it ... or repair the reference", and only the unanchored form's message says "drop the line number or the fragment". The ruling's premise, that the message delivers the rule, is false for the anchored forms; the session settles it. No issue entry records it. |

**The evidence**, verbatim from `L7-b/draft-01-anchored-reference-suffix-message-names-no-repair.md`, from its first section on:

> **The rule**
>
> A reference names a whole file or a whole entry; a line number or a fragment is no part of it. The
> checker's own source states it, `path@core@src/check/references.rs` lines 161-162:
>
> > // A line suffix or a fragment is part of the span but no part of a path
> > // reference, whose id is the file: the repair says to drop it.
>
> No installed text states it: `grep -rn -i 'line number\|fragment\|:line\|suffix'` over the corpus
> returns only `path@agent-config@skills/knowledge-architect-issue-tracking/SKILL.md` lines 317 ("**Never line
> numbers**", about issue entries, not references) and three `file:line` mentions in the project's
> own `path@agent-config@skills/klarch-retrospective-intake/SKILL.md` (lines 85, 141, 150).
>
> **The messages, reproduced**
>
> The repair "drop the line number or the fragment" is named only for the unanchored form,
> [... the draft continues to line 66, before its proposed repair; quoted here: lines 14-29]

**The repair** (`repairs/K18/proposal.md`, lines 65-161): no edit of the corpus.

- **Edit 1, a new core issue entry**,
  `path@elsewhere@crates/core/docs/open-issues/a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair.md`,
  kind `defect`, titled "A reference whose id carries a line suffix or a fragment is told to define
  its entry or repair its pointer, not to drop the suffix"; its "What would close it": "A finding on
  an anchored reference whose id, or whose path after the anchor, holds a `:` followed by digits or
  a `#`, names the repair the unanchored form names: drop the suffix, and name the function or the
  heading in prose." (full text at proposal lines 85-150).

**Instructions**: added 0, removed 0.

**The nearest rival**: "**State the rule in the primer's reference paragraph**", "1 instruction
[...] and it reverses the owner's F23 ruling for this rule". The deciding fact: "the owner ruled
the finding message the delivery of this rule, and the message is the only site that is wrong",
and two core heads already require the anchored messages to name the repair.

**The check** (`repairs/K18/check.md`): "Defects the new text creates: 2. Both are minor and sit
in the entry's own wording"; heads 0; rejected alternatives 0; goals 0. Its trial of the entry:
`cargo klarch check` "PASSED: no findings".

- **D1**: "a factual error in "Why it matters". An issue entry does not ask for a location." Repair:
  "as an analysis under `skill@klarch-retrospective-intake` asks".
- **D2**: "the quoted harness message leaves out the variant that the reproduction's skill section
  reference gets", "end a level-2 heading of the {kind} `{owner}` with that slug, or repair the
  reference".
- N1: the fix makes the "five ways" of `design@core@an-unresolved-reference-is-reported` stale; "What
  would close it" could say the fix records the case in that head. N2: `spec@plans@path-quickfixes`
  changes the same messages. N3, N4: on the proposal's notes, harmless.

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 121:

> | K18 | C34 (W, old; kept; bears on F23 and F69) | **an issue entry** (a finding outside the axis: the checker's messages); not a violation left, since it is kept | F23's corrected premise goes to the owner (section 3), default: keep the ruling, open the core entry. Repair the entry first: check D1 (an issue entry does not ask for a location), D2 (the skill-section variant of the harness message), N1 (the fix records the case in the "five ways" of `design@core@an-unresolved-reference-is-reported`), N2 (`spec@plans@path-quickfixes` changes the same messages). |

**Bears on a ruling** (C34; cross-check, section 3):

From `cross-check/cross-check.md`, lines 200-216:

> **C34 (cause K18): F23 of pass 1, narrowed by F69**
>
> - Source: F23 as the owner ruled it, message of [P1]: "F23: the primer states the wrapped span,
>   the bare name, the path forms, the heading shapes and the commit check." What it left to the
>   finding messages, message of [P2]: "the owner's F23 ruling, that the rarer checker rules are
>   left to their finding messages"; kept.md item 5 lists "a `:line` or `#fragment` suffix". F69,
>   message of [P2-3R]: "F69, lens L7 narrowed: an unstated checker rule is a finding only where its
>   finding message does not name its repair". Owner's word on F58 onward: "I agree with the full
>   list from F58 onward".
> - The premise: the finding message delivers the rule's repair.
> - Why it is false for the anchored forms: only the unanchored form's message says "drop the line
>   number or the fragment"; a register kind, a harness kind and `path` get "define it ... or repair
>   the reference" or "repair the pointer, or delete it" (crates/core/src/check/references.rs,
>   reproduced by the verifier).
> - Default: K18, keep the ruling and open the core issue entry, so that the premise holds once the
>   entry closes.
>

Default: keep the ruling and open the core entry, repaired by D1, D2, N1 and N2 first.

### K19. A subject a review agent and an audit agent both judge is judged by rules each states for itself

From `merge/causes.md`, K19:

> - **Kind**: two homes for one moment.
> - **Mechanism**: on a restatement, on an argument in two texts and on a head's Component, the
>   routing reviewer and the audit agents each state the rule they apply instead of pointing to one
>   home, and they diverge: L5 lost the exemption of an existing longer restatement that the primer
>   and the reviewer keep (C35); the audit's classes admit no argument copied into two texts, which
>   the reviewer reports (C36); the design-record axis names a wrong-Component outcome that its rules
>   and its auditor never judge, which the reviewer does (C37).

**Findings**

#### C35 (W, new text)

- **Draft kept**: `L8-a/draft-01-l5-no-longer-exempts-an-existing-longer-restatement.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; bears-on-ruling (F99 of [P5], D3 of [PR1]).
- **Shared defect** (dedup): "L5 allows both acts on an existing longer restatement since pass 5 removed its exemption".
- **Class decided by** (dedup): workflow-auditor.md 101 104: 101-102 and 104 blamed [P5], phrase absent; PRIMER.md 96 100: [P4], old; takes the newer.

**The verifier**, its line verbatim:

> | draft-01-l5-no-longer-exempts-an-existing-longer-restatement.md | verified | bears-on-ruling (F99 of [P5], and D3 of [PR1]) | The class asks for "the sentence it fails of primer@where-knowledge-goes". An existing longer restatement fails "More than one sentence of it is never restated" (PRIMER 96) if read literally, and is left alone by "An existing longer restatement is converted when a change edits what it says" (PRIMER 99) if read as an allowance. So L5 allows both acts, and the head restatement-size-test states reading B ("no sweep converts them all"). The removal of L5's sentence "A restatement that is only longer than a pointer is no finding." was listed among the removed instructions of K10 in the pass-5 report (pass5/report/report.md, K10, "Instructions: removed 3"), and the owner approved it in F99 ("Agreed on all the defaults, proceed", [P5]). The premise of that ruling was that K10 aligned the audit with the routing reviewer. This draft says the removal reopened a misalignment for existing restatements. That claim is the session's to settle, so the draft is marked and not refuted. The sentence removed was also the text of D3 ("L5 narrowed to meet restatement-size-test", [PR1]). |

**The evidence**, verbatim from `L8-a/draft-01-l5-no-longer-exempts-an-existing-longer-restatement.md`, from its first section on:

> **The instruction, and what it dropped**
>
> .claude/agents/knowledge-architect-workflow-auditor.md:101-102:
>
> > - **L5 restatement.** Each restatement of a directive and each pointer out of an instruction,
> >   under the class of a restatement or a pointer its home does not allow.
>
> L6, line 103-104: "a duplicate is judged as L5 judges a restatement."
>
> The class, lines 73-76:
>
> > - **a restatement or a pointer its home does not allow**: a restatement of a directive, or a
> >   pointer out of an instruction, quoted, and the sentence it fails of `primer@where-knowledge-goes`
> >   or, for a pointer, of `skill@knowledge-architect-agent-configuration@shaping-a-skill`, each read
> >   whole before you judge one;
>
> [... the draft continues to line 88, before its proposed repair; quoted here: lines 13-28]

#### C36 (W, new text)

- **Draft kept**: `L8-a/draft-02-an-argument-in-two-texts-is-a-routing-finding-and-no-audit-class.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "an argument copied into two texts is a routing finding, and no audit class admits it".
- **Class decided by** (dedup): workflow-auditor.md 73 73: blamed [P5], phrase absent; routing-reviewer.md 72 78, standing-state-reviewer.md 16 17, issue-tracking SKILL.md 250 251, planning SKILL.md 58 60, design SKILL.md 628 631: all ancestors (old); takes the newer.

**The verifier**, its line verbatim:

> | draft-02-an-argument-in-two-texts-is-a-routing-finding-and-no-audit-class.md | verified | none | The routing reviewer binds an argument to one home ("Does an argument have two homes? ... The fix is a pointer", 72-74), and sets the size rule apart for a directive only ("A directive is bound by a size, not by that", 78). The audit admits only the classes of the workflow auditor, and its restatement class and L5 name "a restatement of a directive" (73, 101). Both copied reasons exist at [S9] and are reasons, not directives: standing-state-reviewer 16-17 and issue-tracking 250-251; planning 58-60 and design 628-631. The history holds no ruling on a copied argument: F99 covers directives and pointers only. Note for the session: an L5 auditor could take each reason sentence as part of a restatement of the directive beside it, which narrows this divergence but does not close it, since the class judges that unit by the size rule and not by the one-home rule. |

**The evidence**, verbatim from `L8-a/draft-02-an-argument-in-two-texts-is-a-routing-finding-and-no-audit-class.md`, from its first section on:

> **The texts of the project both standards judge**
>
> 1. The reason why the standing re-entry point re-reads every tripwire, stated in two installed
>    texts, each stating it in full (each text names the other agent or skill for its instruction, not as the home of the reason):
>    - .claude/agents/knowledge-architect-standing-state-reviewer.md:16-17: "A tripwire nobody
>      re-reads is a parked item with no re-entry point, which is the thing those files exist to
>      avoid rather than to become."
>    - .claude/skills/knowledge-architect-issue-tracking/SKILL.md:250-251: "**A tripwire nobody reads
>      again is a parked item with no re-entry point**, which is the thing this register exists to
>      avoid, not to become."
> 2. The reason why a plan document is assembled in the session that converged, stated in two
>    installed texts, each stating it in full (each text names the other agent or skill for its instruction, not as the home of the reason):
>    - .claude/skills/knowledge-architect-planning/SKILL.md:58-60: "A plan document written from
>      memory in a later session is written from a summary, and a summary loses exactly the losing
>      arguments and the conditions of each closure."
>    - .claude/skills/knowledge-architect-design/SKILL.md:628-631: "A plan document written from
> [... the draft continues to line 62, before its proposed repair; quoted here: lines 13-28]

#### C37 (W, old text)

- **Draft kept**: `L8-a/draft-03-the-design-record-axis-names-a-wrong-component-outcome-its-rules-never-judge.md`. **Merged into it**: none.
- **Verdict and marks** (dedup): verified; none.
- **Shared defect** (dedup): "the design-record axis names a wrong-Component outcome that its rules and its auditor never judge".
- **Class decided by** (dedup): project-audit SKILL.md 232 232 and 194 195: db44ebb, ancestor; design-record-auditor.md 74 77: 15977c6, ancestor.

**The verifier**, its line verbatim:

> | draft-03-the-design-record-axis-names-a-wrong-component-outcome-its-rules-never-judge.md | verified | none | Project-audit 232 puts "a head in the wrong Component" on the owner list. The axis's rules (194-195), the auditor's read list (35-42) and its list of what it judges (74-77) all leave out decision-recording@owning-component, and the verdict line asks for each rule "named by the section of the rule" (93). `grep -rn owning-component .claude/agents .claude/skills/knowledge-architect-project-audit` returns only routing-reviewer.md:91. The primer's design-heads section, PRIMER 173-359, names a Component only at lines 248, 282, 283, 286 and 305, and none of these says where a head belongs. The input exists in an installed text, but the auditor is not sent to it. So this is a gap in what the auditor is given to read, not a rule missing from the corpus, and the cluster step may class it as a structure the work did not need. Its outcome falls under issue@agent-skills@an-audit-s-provable-gap-on-installed-text-against-real-use (kept item 8): that entry parks the policy on such gaps but does not record this finding, so the draft is not known. |

**The evidence**, verbatim from `L8-a/draft-03-the-design-record-axis-names-a-wrong-component-outcome-its-rules-never-judge.md`, from its first section on:

> **Why not divergent verdicts**
>
> The two standards differ, but no head of this project has been shown to sit in the wrong
> Component, so no text is offered on which their verdicts differ; the class's "a written example is
> no evidence" rules a hypothetical head out. The finding is drafted as the gap it proves.
>
> **The requirement**
>
> The axis's outcome table, which the auditor reads to propose an outcome,
> .claude/skills/knowledge-architect-project-audit/SKILL.md:232, owner list row:
>
> > [...] a ground that is the owner's words; a head in the wrong Component |
>
> The auditor's draft, .claude/agents/knowledge-architect-design-record-auditor.md:93:
>
> > - **the verdict**: `conforms`, or each rule it fails, named by the section of the rule;
> [... the draft continues to line 62, before its proposed repair; quoted here: lines 14-29]

**The repair** (`repairs/K19/proposal.md`, lines 37-272)

- **E1, PRIMER 99-100** (source 100-101; C35): "re-pointing a reference in it does not count."
  gains "Until then, its length alone is no defect."
- **E2, auditor 73-76** (C36): "a restatement of a directive, or a pointer out of an instruction,"
  becomes "a restatement of a directive, a second statement of an argument, or a pointer out of an
  instruction,".
- **E3, auditor 101-102** (C36): "**L5 restatement.** Each restatement of a directive and each
  pointer out of an instruction, under the class of a restatement or a pointer its home does not
  allow." becomes "**L5 restatement.** Each text the class of a restatement or a pointer its home
  does not allow describes."
- **E4, standing-state reviewer 16-18** (C36): the copied reason "A tripwire nobody re-reads is a
  parked item with no re-entry point, which is the thing those files exist to avoid rather than to
  become." becomes ", for the reason `skill@knowledge-architect-issue-tracking@tripwire-entry`
  gives."
- **E5, design SKILL 625-631** (C36): "before the session ends, and owns which ones and their
  shape." gains "for the reason `skill@knowledge-architect-planning@when-planning-starts` gives,";
  the sentence "A plan document written from memory in a later session is written from a summary,
  and a summary loses the losing arguments and the conditions of each closure." is removed.
- **E6 to E8, not applied by default** (C37): the design-record axis's rules (audit SKILL 194-195)
  and the design-record auditor (37, 74-76) gain
  `skill@knowledge-architect-decision-recording@owning-component`, "Judge its Component by
  `skill@knowledge-architect-decision-recording@owning-component`."

**Instructions**: added 1, removed 0. "added by E2: the auditor drafts "a second statement of an
argument [...], quoted, and the sentence it fails of `primer@where-knowledge-goes`""; with E6 to E8,
"added by E6 to E8: "Judge its Component by
`skill@knowledge-architect-decision-recording@owning-component`."", 2 added.

**The nearest rival**: "**each audit text states its rule in full again**, the drafts' site
repairs". The deciding fact: "that is the state pass 5 had and lost. D3's sentence was in L5 as a
second statement beside the primer, and pass 5's rewrite of L5 dropped it, which is C35." For C36,
"narrow the routing reviewer", which "makes the review contradict its home"; for C37, "remove "a
head in the wrong Component" from the owner-list row", which "reverses a member of
`design@agent-skills@audit-outcomes`".

**The check** (`repairs/K19/check.md`): "defects the new text creates: 2. Heads reversed: 0.
Rejected alternatives brought back: 0. Goals strained: 0."

- **D1** (L3): E1's "existing" reads as present in the tree (A) or written before the rule governed
  it (B), for a restatement a previous pass's own repair wrote. D3 states reading A; repair: "Until
  then, its length alone is no defect, whenever it was written."
- **D2** (L3): E2's "a second statement of an argument" reads as covering a reason inside a
  one-sentence restatement (A) or not (B); "it goes with C36's owner-list item".
- **R1**: the owner-list item for E2 "should cite D30", which admits "divergent verdicts" among the
  origins of an edit at the harvest of `design@agent-skills@additions-need-real-use`.
- **R2**: C37's alternative is "a change of the section "design-record-axis" under
  `skill@knowledge-architect-design` (in-change path), not [...] a member recorded directly".

**Outcome** (cross-check, section 2):

From `cross-check/cross-check.md`, line 122:

> | K19 | C35 (W, new; bears on F99 and D3), C36 (W, new), C37 (W, old) | **C35 applied in the branch**; **C36 owner list** (which standard wins, for divergent verdicts); **C37 owner list**, default an issue entry | C35: E1 with check D1's wording, "Until then, its length alone is no defect, whenever it was written", which is D3's reading; the corrected premise of F99 goes to the owner (section 3). C36: E2 to E5, net +1; check D2 (a reason inside a one-sentence restatement) goes with this item; cite D30 for `design@agent-skills@additions-need-real-use` (check R1). C37: default a todo entry in agent-skills, per F96 and kept.md item 8; the alternative (E6 to E8) is a change of `design@agent-skills@design-record-axis` under the design skill, not a member recorded directly (check R2). |

**Bears on a ruling** (C35; cross-check, section 3):

From `cross-check/cross-check.md`, lines 217-229:

> **C35 (cause K19): F99 of pass 5 and D3 of the spec**
>
> - Source, F99, message of [P5]: "- F99 K10 (C5, C38): Edits 1 to 5; K6 E1 (C7) follows it." The
>   owner's word: "Agreed on all the defaults, proceed". D3, spec 2505-2509, ruled in round 4 ("All
>   defaults approved."): "Default: L5 reports a drifted restatement and a restatement without its
>   pointer; a drifted one is repaired, which converts it to a pointer when it is longer than one; a
>   restatement only longer than a pointer is no finding."
> - The premise of F99, per the verifier: pass 5's K10 aligned L5 with the routing reviewer.
> - Why it may be false: K10 removed L5's sentence "A restatement that is only longer than a pointer
>   is no finding.", D3's text, so L5 now admits both acts on an existing longer restatement, while
>   the routing reviewer does not report one.
> - Default: K19 E1, D3's reading stated in the primer, F99's pointer from L5 kept.
>

Default: C35 applied, E1 in check D1's wording; C36 on the owner list, E2 to E5, net +1; C37 on the
owner list, default a `todo` issue entry in agent-skills.

---

## 4. The overlap map

The cross-check's section 1, verbatim. In short: **one paragraph of the corpus is edited by two
causes**, the script's `checkBrief` (lines 234-242), by K1 (Edit 2) and K2 (Edit 2d), whose edits
of lines 240-241 conflict as written and are merged as the cross-check states. Outside the corpus,
four sites meet several causes: the spec's "The repair standard" (K6, K7), the spec's "The
proposal check" (K1, K8), the agent-skills issue index (K9, K12, K15, K19) and the changelog's
`Next release` (K15, with K10, K16 and K19 possibly owing a line).

From `cross-check/cross-check.md`, lines 25-90:

> **1. Paragraphs that the proposals of two or more causes edit**
>
> **1.1 In the corpus**
>
> **One paragraph: the script's `checkBrief`, lines 234-242. Causes K1 and K2.**
>
> | cause | its edit of the paragraph |
> | --- | --- |
> | K1, Edit 2 | line 237 `'The worktree to read, detached at that commit: ' + reading` becomes `'Your worktree, detached at that commit: ' + checkTree(j)`; lines 240-241 become "... read both in your worktree, and follow the worktree where your copy differs." |
> | K2, Edit 2d | lines 240-241 are replaced by `FROM_DISK`, a constant inserted after line 91 (Edit 2a) that names the definition, the root CLAUDE.md and the primer |
>
> The two edits of lines 240-241 conflict as written: K1's after-text keeps the two-file sentence,
> K2 replaces it. Merged form, which keeps both causes' intent: line 237 as K1 writes it, and lines
> 240-241 as K2's `FROM_DISK`. `FROM_DISK` says "read each in the worktree this brief names", which
> stays true of K1's "Your worktree" line. K2's justification sentence "the check brief names 'The
> worktree to read'" becomes false after K1; it is in the proposal only, not in the edited text.
> Apply both in one edit of the paragraph.
>
> No other paragraph of the corpus is edited by two causes. Checked file by file:
>
> | file | causes and paragraphs, all distinct |
> | --- | --- |
> | audit SKILL | K15 E2 (trigger bullet, 43-47); K4 E1 (point 6, 88-95); K4 E2 (the section "record-of-an-audit", 135-150); K3 E2 (the section "outcomes" row, 127); K7 (the section "repair-standard" scope sentence, 154-155); K8 (the section "repair-standard" record check, 178-183); K19 E6, not applied by default (the section "design-record-axis" rules, 194-195); K2 E4 (axis "The corpus", 240-244); K1 E3 (args row `worktrees`, 271); K2 E5 (axis "The repair by cause ... The check", 285-293); K5 (axis "Old and new text", 294-297); K2 E6 (axis "The tripwires", 299-302); K3 E3 (axis "Sent upstream", 324-330) |
> | script | K1 E3 (line 13); K2 E3 (`CORPUS`, 81-85); K2 E2a (insert after 91); K2 E2b (124-125); K2 E2c (139-140); K1 E1 (insert after 219); K1 E2 and K2 E2d (`checkBrief`, 1.1 above); K2 E1 (report prompt, 279-280) |
> | auditor | K19 E2 (class bullet, 73-76); K19 E3 (L5, 101-102); K3 E1 (refute bullet, 178-179) |
> | PRIMER.md | K19 E1 (99-100); K17 E1 (300); K16 E3 (373-374) |
> | CLAUDE.md | K17 E2 (121-124); K14 (542-545) |
> | klarch-retrospective-intake SKILL.md | K6 E1 (item c, 91-96); K16 E4 (item d's first sub-bullet, 98-99); K6 E2 (item e, 115-117); K6 E3 (item f, 125-129) |
> | planning SKILL.md | K10 (295-297); K13 E1 (346-349) |
> | design SKILL.md | K11 (125-127); K19 E5 (step 8, 625-631); K13 E2 (closing paragraph, 648-651) |
> | issue-tracking SKILL.md | K17 E4 (160); K17 E3 (241-242) |
> | others, one cause each | setup SKILL.md 279-281 (K15 E1); retrospective SKILL.md 188-191 (K16 E1); klarch-development SKILL.md 40-41 (K9); decision-record-reviewer 108-110 (K16 E5); standing-state-reviewer 16-18 (K19 E4); design-record-auditor 37 and 74-76 (K19 E7, E8, not applied by default) |
>
> **1.2 Outside the corpus, where application must merge**
>
> | site | causes | what meets there |
> | --- | --- | --- |
> | spec, "The repair standard", 1927-1938 | K6 (its proposal: correct in place "its items c and e ... point to the section's record check", 1933-1936); K7 (its check: correct "holds what any repair of an instruction found by reading or by real use is judged by", 1927-1929, to the new scope) | two in-place corrections of one paragraph; both follow only if the owner rules for K6's and K7's defaults |
> | spec, "The proposal check", 1938-1953 | K1 (its check, note 3: "the script's prompt names the proposal, the corpus and the scratch directory" gains the check worktrees and their setup agent); K8 (its check, note 1: the record-check sentence, 1945-1947, reworded as K8's edit) | two in-place corrections of one paragraph, both proposed by checks, not by the proposals |
> | crates/agent-skills/docs/open-issues/index.md, generated | K9 (new entry), K12 (rename and new title), K15 (new title of `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`), K19 (C37's default, a new entry) | one regeneration after all four register edits; no hand merge |
> | CHANGELOG.md, `Next release` | K15 E3 edits the existing entry on moving the pin; K10, K16 (check N4) and K19 say a line may be owed | the session writes the pass's lines once, under `design@knowledge-architect@changelog-entries` |
>
> Other spec corrections, one cause each: K3 (spec 325-326, 1764-1766, and, per its check, 1570 and
> 1605); K6 (its check: step 9, 2396); K2 (its check S1: lines 11 and 1625); K5 (its check: 1909-1912
> carries the same "its phrase" wording); K12 (724, 1386, 2190, 2751, a re-pointed reference).
>
> **1.3 Edits in different paragraphs that depend on each other**
>
> - **K6 and K8.** K8's second rival argues from "the intake's item c applies the sentence
>   directly". After K6, item c no longer holds the record check; item f does. K8's edit is
>   unaffected; its argument then reads "item f".
> - **K6 and K7.** K7's new scope "wherever a skill names this section" is met, in this repository,
>   by the intake's items c and f. After K6, item f alone names the section. K7's edit still holds.
> - **K2 and K4.** K2 Edit 1 points the report prompt to the section "record-of-an-audit", which K4 Edit 2
>   rewrites. K4 keeps the count "of those on old text that went on the owner list", so K2's pointer
>   holds. K4's new naming rule binds the report file that the script's header (line 277,
>   `args.commit`) and the dedup's class (170-172) fill with SHAs; K2 edits the report prompt beside
>   line 277 and does not touch it. The session rewrites those SHAs as subjects when it writes the
>   report file.
> - **K2 and K5.** K2 Edit 1's count is "as point 8 defines them"; K5 changes the classification test
>   point 8 delegates to. The pointer holds under either.
> - **K13 and K19.** Both edit the section "discussion-loop" of the design skill, in adjacent paragraphs (step 8
>   and the closing paragraph). Apply in one pass over the section.
> - **K3 and K2.** K3 Edit 1 deletes the refute bullet of the section "verify-drafts"; K2 Edit 6 points the
>   axis's tripwire sentence to the section "verify-drafts". The bullet K2 relies on (176-177) stays.
>

---

## 5. The owner list

The session's verification of this report left its findings, proposals and checks as section 3
gives them: it rejected no finding, and it re-classified the findings on new text by the words
that carry each defect, as F108 says. The labels continue this branch's record, from F107, as K4's
check D2 proposes for this branch. Rule on each item by writing its `Outcome` line, or in the
conversation, whose words are then copied here.

### Before the list

#### F107: the net growth of the pass

The repairs, as their defaults below give them, add 7 instructions and remove 5: net +2 (K1 +3;
K3 -1; K4 +2 -2; K6 +1 -2; K19 +1). With K19's alternative for C37 (E6 to E8), net +3.
`skill@knowledge-architect-project-audit@repair-standard`: "the owner is told so first, with the
net figure, and rules on that growth before any of them is applied". Nothing of the pass is applied
before this ruling.

- Default: the growth is accepted, net +2.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F108: the convergence question

Point 8: "The count on old text is expected to fall from one pass to the next; a pass where it
does not goes to the owner as a question, with the counts, and starts no new pass on its own."

The dedup classified by the whole blamed line; the session had the 18 findings it put on new text
re-classified by the words that carry each defect, the reading point 8 and AC7 give (file
`reclass.md` of the run's scratch directory). C3, C18 and C36 move to old text; the 20 on old text
stay old, since a line whose words are all old has old defect words. So:

| count | pass 5 (baseline) | pass 6 |
| --- | --- | --- |
| violations left on old text | 16 | 21: 23 confirmed on old text, less C34 (kept) and C28 (known) |
| severe among them, by the defaults below | 8 | 7: C3, C6, C7, C18, C36, C37, C38 |
| confirmed on new text | not compared | 15 |

**AC4 fires**: the violations left on old text do not fall, while the severe ones fall. Its
response is to reopen #workflow-axis-lenses and
#workflow-axis-finding-standard before the merge. **AC7 fires** on
the dedup's classification: three of its 18 findings on new text carry words present at the
commit pass 5 audited. Its response is to reopen the classification rule; F114's repair states the
rule as AC7 reads it, and this pass's counts are taken by it.

What the old-text findings are: 8 of the 21 sit wholly on text main held before this branch, by
the commits their defect lines blame to (C7, C8, C23, C27, C29, C32, C37, C38), among them C7 on
the design skill's two paths and C23 and C29 on installed restatements; the lenses keep finding
defects in text five passes read. Your stop rule, quoted in
`design@agent-skills@audit-method`: "As long as we observe a "convergence", and the amount of
leftover violations is small enough, I think that is good. If you ask an LLM to find a flaw or
defect in something, it will nearly always find one IMO."

- Default: AC7's response is F114; AC4's response is that the audit stops after this pass's
  application, by your stop rule, on the severe count that falls; no pass 7 runs, the findings left
  take their outcomes as this list gives them, and the branch goes on to the harvest and its
  review. The alternative: a pass 7 after this application, or the design discussion AC4 names.
- Outcome: the default, on the owner's word "Go with the defaults."

### Which side of a contradiction wins

#### F109: K1, one worktree for every check agent (C1, new text)

Turns on: review skill, "**The dispatcher names each subagent's path in its brief, distinct for
each subagent**", against the script's check brief, "The worktree to read, detached at that
commit: ' + reading", given to every check agent. Repair: a setup agent creates one worktree per
check job (Edits 1 to 3); 3 instructions added, the setup agent's prompt; one more agent per pass,
against the agent budget of the spec's criterion c6.

- Default: the invariant wins; Edits 1 to 3.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F110: K3, what an earlier ruling does to a finding found again (C3, C4, new text)

Turns on: the auditor's verify bullet, "refute any other draft whose finding the owner already
ruled on, naming the commit or the report file of the history that holds the ruling", against
point 8, "is a violation left, whether a draft missed it or an edit made it"; and "Every finding
takes exactly one outcome" against the rows "sent upstream" and "nothing" for a known or kept
finding on the installed text. **A corrected premise, F97 of pass 5**: you approved K2's Edits 1
to 5 ("Agreed on all the defaults, proceed") on the premise that after them "only a ruled finding
of neither case is refuted"; a ruling to apply a repair that did not close the defect is of
neither case, so the verifier refutes a finding point 8 counts. Repair: the refute bullet deleted,
and the upstream row and paragraph narrowed to the findings the row "nothing" does not name; 1
removed. Its check names a consequence: in an adopting project, a known finding on the installed
text no longer reaches the upstream file.

- Default: point 8 and the row "nothing" win; Edits 1 and 3, and Edit 2 in the check's wording.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F111: K13, what waits when a load-bearing gap defeats a ruling's premise (C6, old text)

Turns on: `primer@owner-word-premise`, "proceed on the default unless the owner answers otherwise",
against planning's design audit, "A load-bearing gap of point 2 is not such a default, even where
the message that puts it to the owner names one: it stops the slice." **A corrected premise, F58
of passes 2 and 3**: you approved "one home at the opening of
`skill@knowledge-architect-planning@working-a-slice`" for what waits; the primer's premise rule,
ruled earlier, also says what waits, and says the opposite for this case. Repair: planning states
that its stop replaces the primer's default for this case; 0 added, 0 removed.

- Default: planning's stop wins; Edit 1 in the check's wording, "the slice stops instead of
  proceeding on the default that `primer@owner-word-premise` names", and Edit 2.
- Outcome: the default, on the owner's word "Go with the defaults."

### Which standard wins, for divergent verdicts

#### F112: K19, an argument restated in a second text (C36, new text)

Turns on: the routing reviewer, "**Does an argument have two homes?** [...] The fix is a pointer.",
against the auditor's L5 class, "a restatement of a directive, or a pointer out of an instruction",
which admits no argument. Repair: E2 to E5; 1 added, 0 removed. Its check's D2: "a second
statement of an argument" may or may not take in a reason inside a one-sentence restatement the
primer allows. The admission of divergent verdicts as an origin of an edit is D30 of the spec.

- Default: the review's standard wins, and the audit reports a restated argument too; E2 to E5,
  a reason inside an allowed one-sentence restatement not counted as a second statement.
- Outcome: the default, on the owner's word "Go with the defaults."

### Two-reading instructions whose repair changes what agents are told

#### F113: K4, which commit carries a pass's record, and when the report file is committed (C18, C19, C21, C30, new text)

Turns on: the record section, "The file is committed with its `Outcome` lines before the pass's
edits are applied", "Each commit of an audit says", "each pass's message gives the count"; point
6's `F<n>`, "numbered as `primer@labelled-items` says", whose record is the file or the messages;
and the file carrying the branch's own SHAs. Repair: the file is committed once every `Outcome`
line holds a ruling, before any edit; the pass's commit is the one applying its last edit; the
file and every message name a branch commit by its subject; `F` numbered with the file as its
record. 2 added, 2 removed. It strains `design@agent-skills@audit-outcomes` ("applied at once"),
whose ground is your words, and contradicts the `F` row of `design@agent-skills@ruled-items-labelled`
until the spec's harvest rewrites it.

- Default: Edits 1 and 2, with the check's clause for an item you do not rule on at the audit's
  stop; this pass continues at F107, and later audits number each file from F1.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F114: K5, "its phrase" in the old-and-new-text test (C17, new text)

Turns on: "is on new text only where its phrase, whitespace-normalised, is absent from every file
of the corpus at `previous`": the whole blamed line, or the words carrying the defect. Point 8
defines the class by words, and AC7 says "the words carrying its defect". Repair: one rewording; 0
added, 0 removed.

- Default: the words carrying the defect, with the check's clause "so an edit elsewhere in a line
  the previous pass read leaves the finding on old text"; this pass's counts are already taken so,
  per F108.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F115: K7, whom the repair standard binds (C16, new text; known in part)

Turns on: "What any repair of an instruction is judged by, whether a reading or a real session
found its defect", while only an audit and the intake name the section. Repair: "in an audit, and
at each step of a skill that names this section"; the entry
`issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` gains the section in
what would close it. It narrows the words the checkpoint of round 24 approved, "any repair of an
instruction found by reading or by real use".

- Default: Edit 1 in the check's wording, and Edit 2.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F116: K11, which path a requested change takes when it converges before its code exists (C7, old text)

Turns on: the in-change path, "when the decision lands in the change under way: its work, as when
the decision was met during an issue fix or a review repair", against the full path, "work that
no longer lands in the change under way (it needs a branch of its own, several pull requests, or
nobody has started it)". Reading 1: the request is the change under way, no plan document.
Reading 2: the in-change path is for a change begun before the discussion; a requested change that
is not bounded takes a spec. Reading 2 agrees with `design@agent-skills@adopting-existing-docs` and
narrows the entry condition of `design@agent-skills@in-change-path` against its argument ("When the
work lands in the change under way, neither applies"), so its head is edited in the same commit,
and `tripwire@agent-skills@ruling-lost-in-change` watches that condition. Its cost: a spec, four
plan reviews, a harvest and a deletion for every unbounded requested change built in one pull
request. Reading 1 keeps the head, and brings setup step 4, the move of an adopting project's
documents, to the full path in its own words.

- Default: reading 2, in the check's wording, with the head's edit.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F117: K15, where the audit a pin move owes runs (C38, old text; C15, predicted gap)

Turns on: setup step 5, "after the pin's commit, in a branch of its own": from the pin's branch at
once, or from the main branch once the pin move merges. Repair: the latter, stated in setup, in
the audit skill's trigger as one sentence with its pointer, and in the CHANGELOG's entry. **A
premise that may be false, F94 of pass 4**: you approved deleting the sentence asking a session to
reload the harness's skills, on the premise that a session without the workflow listed dispatches
the agents by hand; whether the harness registers an agent type an install adds mid-session is not
established. C15 is a predicted gap: an issue entry, the reload sentence not restored.

- Default: Edits 1 and 3, and Edit 2 replaced by the check's sentence; C15 the extension of
  `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`.
- Outcome: the default, on the owner's word "Go with the defaults."

### A placement a ruling of the owner names

#### F118: K6, where the intake reads a fix against the record (C14, C26, new text)

Turns on: the intake's item c, "The fix item f settles on is read against the record [...]", while
"The checks a to c read text, and fresh subagents may run them", before item f settles a fix; and
item f's one-sentence gloss of several directives of the repair section. **A corrected premise,
D35**: your words "D32 and D33 approved. D34 and D35 defaults too." answered the default "the
record check is part of judging a repair's shape, the intake's items c and e already read the same
record, so one home keeps one standard"; item c cannot read a fix that item f has not settled.
Repair: the sentence in c removed, item e pointed at f, item f a pointer read whole; 1 added, 2
removed.

- Default: Edits 1 to 3; D35's one home kept, stated at item f. If you keep the placement in item
  c, Edit 3 alone.
- Outcome: the default, on the owner's word "Go with the defaults."

### A provable gap, since filling it adds an instruction

#### F119: K19, the design-record axis's rule for a head in the wrong Component (C37, old text)

Turns on: the design-record axis's owner-list row, "a head in the wrong Component", while its
rules name only `primer@design-heads` and the losing-alternatives rules. Filling it adds a rule,
a change of `design@agent-skills@design-record-axis` under the design skill.

- Default: a `todo` issue entry in agent-skills, as F96 of pass 5 parked a provable gap on the
  installed text.
- Outcome: the default, on the owner's word "Go with the defaults."

### Corrected premises on items the session applies or parks

Per `primer@owner-word-premise`, each goes to you with its default, and the session proceeds on it
unless you answer otherwise.

#### F120: F44 of pass 2, the fold's rebase (C10, K14, applied)

Your word approved the fold "probed on git 2.43 to fold and leave `git diff <old-head> HEAD`
empty"; the probe kept `origin/main` at the branch's base, and after a fetch that moves it the
fold rebases onto a moved base and the check fails with no content lost. Repair: `--keep-base
origin/main`, which needs git 2.24 or later.

- Default: the repair applied.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F121: F23 of pass 1, narrowed by F69, the rarer checker rules (C34, K18, an issue entry)

Your rulings: the rarer checker rules are left to their finding messages, "an unstated checker
rule is a finding only where its finding message does not name its repair". For a `:line` or
`#fragment` suffix on an anchored reference, the message names "define it ... or repair the
reference", not the suffix's repair.

- Default: the ruling kept; a core issue entry for the messages, repaired by its check first.
- Outcome: the default, on the owner's word "Go with the defaults."

#### F122: F99 of pass 5 and D3, a restatement only longer than a pointer (C35, K19, applied)

Your word on F99 approved pass 5's K10, which aligned L5 with the routing reviewer and dropped from
L5 D3's sentence, "a restatement only longer than a pointer is no finding". Repair: D3's reading
stated in the primer, "Until then, its length alone is no defect, whenever it was written".

- Default: the repair applied.
- Outcome: the default, on the owner's word "Go with the defaults."

### For your information: applied in the branch, issue entries, nothing

- Applied in the branch, after F107: K2 (C2, C13, C24, C25), with its check's clause; K8 (C20);
  K9's Edit 1 (C12); K10 (C5); K14 (C10); K16 (C22, C23, C27, C29), E5 in its check's wording;
  K17's Edits 1 to 4 (C31, C32, C33); K19's E1 (C35).
- Issue entries: K9's new entry (C9); K12's entry widened and renamed (C8, C11), its check's seven
  defects repaired first; K15's extension (C15); K18's new core entry (C34).
- Nothing: C28, known by `issue@core@references-are-read-in-markdown-and-rust-only`.
- The spec is corrected in place where a default makes it false, as each cause names.

---

## 6. What the run did not decide

### The merge choices (`merge/causes.md`, "Choices")

From `merge/causes.md`, lines 523-593:

> **Choices**
>
> Where the groupings agree on both membership and kind (K1, K3, K4, K9, K12, K13, K16, K17, K18,
> K19), the cause is taken as they give it. The differences, and the choice made for each:
>
> 1. **K2's kind: "a restatement that drifted from its home"** (cluster 1), not "two homes for one
>    moment" (cluster 2). For each of its four findings, the drafts name one text as the home and
>    the other as its copy: the record section's count for C2, review-invariants for C13, the axis
>    section's corpus for C24, the auditor's sections for C25. The repair of every draft is the copy
>    brought back to a pointer, which is the repair of a drifted restatement; nothing is merged.
>    C25's copies have not drifted yet, but its repair has the same form, so it stays in the cause.
>
> 2. **C14 and C26 form one cause, K6. C16 and C20 are causes of their own, K7 and K8** (cluster 1).
>    Cluster 2's G6 grouped all four as one act stated in four texts. They do not share a repair:
>    L5-a/d5's single edit set at items c and f closes C14 and C26, and it closes neither C16, a
>    sentence of the section "repair-standard" on its own scope, nor C20, a sentence of the section "repair-standard" on
>    whose goals. A cause whose repair is four independent edits gains nothing from being one cause.
>    Two further reasons for C16: read as "two homes" with agent-configuration's own check, its
>    repair would point agent-configuration to the section, the losing alternative C16's draft
>    cites from the spec (lost on a80); and its draft's repair is a choice of reading for the owner,
>    which is the shape of a cause with no kind.
>
> 3. **C5 and C7 are split, K10 and K11** (cluster 1). Cluster 2's G8 grouped them as the boundary
>    between the two paths. Their texts and repairs are disjoint: C5 is one sentence of planning
>    ("Each names") whose scope a design head already states, so it is repaired by a rewording
>    applied in the branch; C7 is the full-path condition in the design skill ("nobody has started
>    it") whose reading no record states, so it goes to the owner. C5 is on new text and C7 on old
>    text, and grouping them would mix their convergence counts in one owner-list item.
>
> 4. **K10 (C5) keeps the kind "two homes for one moment"**, as both groupings gave it: design step 7
>    states the in-change criterion, and planning's paragraph takes it in again under its own list;
>    the repair narrows planning's "Each" so that step 7 is the one statement.
>
> 5. **K11 (C7) has no kind** (cluster 1). Its defect is one condition, read two ways, in one
>    paragraph of the design skill; no second text states the path choice in other terms, and no
>    fixed structure or narrow rule produces it.
>
> 6. **K5 (C17) has no kind** (cluster 1), not "two homes" (cluster 2). Point 8 states that "The
>    axis's section says how its findings are classified", so the axis's test is the delegated
>    statement of the classification, not a second home of point 8's definition. The defect is the
>    pronoun "its" in that one sentence, which reads two ways; both drafts repair it by a rewording
>    of that word.
>
> 7. **K14 (C10) has no kind** (cluster 1), not "two homes" (cluster 2). The draft's repair corrects
>    the fold's command to `$(git merge-base origin/main HEAD)` and keeps its check; it merges no
>    statements. The general history-edit bullet already states both cases correctly, and the fold
>    bullet's defect is a command whose effect differs from what its own check presumes, a factual
>    error in one sentence.
>
> 8. **K15 (C15, C38) has no kind, and stays one cause** (cluster 1's kind; both groupings' membership).
>    The audit skill's trigger sentence points to setup's step 5 ("as
>    `skill@knowledge-architect-setup@moving-the-pin` says"), a one-sentence restatement with its
>    pointer, which the primer allows, so the two texts are not two homes. The two findings share one
>    clause, step 5's "after the pin's commit, in a branch of its own", which leaves the branch's base
>    and the session open; their outcomes still differ (C15 a predicted gap, an issue entry; C38 a
>    two-reading instruction, the owner list), as the axis section allows within one cause.
>
> 9. **C19 stays in K4** (both groupings). Cluster 1 notes that its moment, what waits for the
>    owner's ruling, also has a home in point 6. Its two readings come from the record section's
>    sentence that fixes when the report file is committed, which is K4's mechanism, and K4's repair
>    rewrites that section.
>
> 10. **C2 stays in K2, not K4** (both groupings). Its defect is the script's copy of the record
>     section's count; bringing the copy back to its home closes it whatever K4 and K5 decide.
>
> 11. **C12 stays in K9, not K16** (both groupings). Its copy drifted from the root's list of places
>     as that list grew, which is K9's mechanism; its repair is K9's pointer.
>
> 12. **C34 stays apart from K17** (both groupings). K17's texts state the checker's behaviour
>     wrongly; for C34 the corpus states nothing, and the defect is in the checker's finding
>     messages.

### The plausible verdicts

Five drafts, all predicted gaps; four reached the confirmed findings as the kept draft of their
finding (C8, C9, C11, C15), and L4-b/d3 was merged into C9.

From `verify/L2-a/verdicts.md`, lines 11-12:

> | draft-05-a-pair-written-in-the-deleting-commit-never-enters-history.md | plausible | none | Quotations match (primer 303-310; planning 409-411, 419, 433; decision-recording 117-118). The probe, re-run in a scratch repository, prints nothing for -G'old-thread -> new-slug', and -G'new-slug' finds only design.md's line. The texts allow the head, the pair and the deletion in one commit. Whether a session writes the pair only at harvest is a prediction no session has shown. Not known: no issue entry in the listing matches. |
> | draft-06-a-pending-audit-report-is-read-by-no-installed-search.md | plausible | none | Quotations match (primer 115; design 432-436; planning 352-353; CLAUDE.md 356; standing-entry-searcher take-your-group runs only `issues` and `tripwires`). `grep -rn audit-reports .claude CLAUDE.md` finds no installed search. Not known: `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search` covers docs/retrospective-reports/ only, and its premise that installed text cannot name the directory does not hold for the installed row. D32 ([D32-35]) ruled only where the root lists it. Predicted, since report files are normally deleted on the audit's own branch. |

From `verify/L2-b/verdicts.md`, line 10:

> | d2-spec-conformity-at-the-merge-review-reads-a-deleted-spec.md | plausible | none | The moment exists (planning SKILL.md:415-419 and 431-433: the spec is deleted before the review before the merge, whose spec-conformity condition, review SKILL.md:34, still holds), but no reading shows a reviewer misled: the range's diff carries the deleted spec as removed lines, and planning SKILL.md:543-544 gives `git show <commit>^:<path>`, so a dispatcher can name it. A predicted gap on installed text. issue@agent-skills@the-transcript-reviewer-cannot-see-a-plan-document-its-range-deletes records the same deletion for two other axes only, so it is related, not known. |

From `verify/L2-d/verdicts.md`, line 7:

> | draft-1-new-audit-agent-unregistered-at-pin-move.md | plausible | bears-on-ruling (pass 4, commit [P4] "[audit] Pass 4 ...", F94) | Every quotation holds: setup SKILL.md:272-281, CHANGELOG.md:47-49, project-audit SKILL.md:65-67, 190-192, 200, 332-334, review SKILL.md:95-97, the saved workflow's line 142 `agentType: 'knowledge-architect-workflow-auditor'`; `git ls-tree --name-only v0.5.0 crates/agent-skills/content/agents/` lists eight agents with neither auditor, and `.../skills/` lists no project-audit. Whether the harness registers an agent type that an install adds mid-session is not established by any reading, so the gap stays predicted. Not known: `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, third case, states the opposite ("The subagents the session dispatches are covered"), and the draft's proposed outcome corrects that claim. Bears on F94 of pass 4, approved by the owner: "the sentence asking a session to reload the harness's skills is deleted; a session whose harness does not list the saved workflow dispatches the same agents by hand". The draft puts that premise in doubt for an agent the install adds, and its fallback repair, "from a session started after the install", would partly restore what F94 deleted. The session settles this. Not on the kept list. Lens and class (L2 walk, W, predicted gap) are correct; the proposed outcome, an issue entry, is the axis table's row for a predicted gap. |

From `verify/L4-b/verdicts.md`, line 7:

> | d3-pending-audit-report-reached-by-no-installed-search.md | plausible | none | A predicted gap: no reading confirms or refutes it. Quotations match (PRIMER.md line 115, standing-entry-searcher lines 40-41, root CLAUDE.md line 356). Its moment, a report file left pending on the main branch, rests on no installed path: the audit outcomes are carried out in the branch or become an issue entry (a deferred item, work too large for the branch), which the register searches do reach; only the primer row's "leaves when" and this repository's README admit it. Not marked known: `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search` records the retrospective analyses on the project's side only, not docs/audit-reports/ nor the installed text. |

### "Outside the axis" notes of the returns

Every one of the 17 returns says "Outside the axis: none".

### "Met outside the task" notes of the returns

Eleven returns say "none": L1-a, L1-b, L1-c, L2-d, L3-b, L4-a, L4-b, L5-a, L6-a, L6-b, L8-a. The
other six, verbatim:

From `L2-a/return.md`, lines 59-61:

> Met outside the task:
> - `design@agent-skills@in-change-path` (crates/agent-skills/docs/design.md:1043-1051) says the in-change path's deliberation is carried by "the message of the commit that writes the decision's design entry, or that implements a decision that earns none", with "the rulings on tripwires"; the installed design skill now adds a third place, "where `skill@knowledge-architect-review@what-review-leaves` puts a record carried by a message", and the rulings on acceptance criteria (pass 5, F101, K11). The head is not brought to that text: a design-record finding, outside this axis; default an issue entry or an in-place repair of the head under `skill@knowledge-architect-decision-recording`.
> - `design@agent-skills@acceptance-criteria-in-the-document` (design.md:1109-1121) says an acceptance criterion "is written in its own section of the plan document of that work", while the in-change path now applies one as a test with its label in the test's comment; the head does not cover that member. Same route as above.

From `L2-b/return.md`, lines 58-60:

> Met outside the task:
> - `cargo klarch commits --help` reads "Judge every commit message in a range, each against its own commit's tree", while crates/core/README.md:438-442 and the primer state that a commit whose tree carries findings fails the run: the help names the message only. The checker's help, not the corpus; for the L7 lens or an issue on the core.
> - A reviewer's or a test worktree nested under .claude/worktrees picks up the main tree's .cargo/config.toml fields (probe cargoprobe: an alias defined only in the parent resolves in the nested directory), so a review of a commit that removes an alias or an [env] field builds with the main tree's value; klarch-development states this only for a mutation of the cargo configuration. No such commit is in the history of .cargo/config.toml (two commits, both adding), so dropped as a prediction with no instance.

From `L2-c/return.md`, lines 59-60:

> Met outside the task:
> - design@agent-skills@ruled-items-labelled, its F row, says an audit's F labels reach "the audit's commit messages", while the audit skill's point 6 now writes the owner list into the pass's report file, so a session numbering pass 6's items may read the head as continuing the branch's F107 or restart at F1 in the file. Not drafted: the spec's harvest row (docs/plans/specs/agentic-workflow-axis.md line 2751) already schedules the F row's rewrite to name the report file. Outcome proposed: none needed beyond that harvest; the session may state the numbering in pass 6's report file.

From `L3-a/return.md`, lines 62-63:

> Met outside the task:
> - The session's snapshot of the root CLAUDE.md and of the primer predates [S9]: it lacks the primer's docs/audit-reports/ row and the root's "ls docs/audit-reports/" line, which the worktree holds. Followed the worktree, as the brief says; this is the case issue@agent-skills@an-upgrade-re-reads-no-root-restatement records (kept list, item 7). No outcome needed.

From `L7-a/return.md`, lines 55-56:

> Met outside the task:
> - The help of the commits command says "Judge every commit message in a range, each against its own commit's tree", while crates/core/README.md:49 and the command judge each commit's tree as well as its message ("a commit fails on any finding of its tree, of its message or of a citation by SHA", README.md:441-442). A help string of the checker, outside the corpus; for the session to route by primer@met-outside-the-task (its fix looks checkable from the diff alone, case 2).

From `L7-b/return.md`, lines 52-53:

> Met outside the task:
> - The checker's message for a missing root document names a repair that does not apply to the root Component: in scratch project p1, "README.md  the component `demo` carries no README.md → create it, or stop listing the component in knowledge-architect.toml [project]"; the root is named by [project] name and listed nowhere. The checker's own behaviour, outside this axis; for the session to route.

### Notes of the later steps that the session also routes

The cross-check's own section, verbatim:

From `cross-check/cross-check.md`, lines 249-256:

> **Met outside the task**
>
> None found by this cross-check beyond the items the proposals and checks already route: K2
> (general-purpose prompts lack the from-disk line), K3 (a finding sent upstream counts at every
> later pass; point 8's unruled-item overlap), K5 (a case between the axis table's two rows; the
> design-record axis classifies by entry), K10 (the in-change head lacks "acceptance criteria"), K13
> (design "Two cases defeat that default"), K16 (the primer's design entry), K17 (planning and setup
> "then checks"; Edit 5 as case 2). The session routes each by `primer@met-outside-the-task`.

The items it names, with where each is stated:

- K2 proposal, "Met outside the task": the script's four general-purpose prompts carry no
  instruction to follow the worktree where a session copy differs; and the axis's "Not in this
  axis" line overlapping the corpus's exclusion, left as it is.
- K3 proposal, "Met outside the task": a finding sent upstream counts as a violation left on old
  text at every later pass, in an adopting project; K3 check note 1: point 8's "one the owner does
  not rule on" overlaps an upstream finding the same way.
- K5 proposal, "Met outside the task": the axis outcome table leaves a case between its two rows (a
  reading stated by a plan document's criterion); the design-record axis classifies by entry.
- K8 proposal: "the rejected alternatives of the Component that owns the text" is singular while
  a repair may edit texts of several Components; dropped by the proposal, for judgement.
- K9 proposal, "Note for the session": the intake's the section "ground-before-judging" step 3 reads only the
  issue and tripwire listings.
- K10 proposal, "Met outside the task": `design@agent-skills@in-change-path` lists "the rulings on
  tripwires" only, where the design skill says "tripwires and acceptance criteria" (the same as
  L2-a's first note above).
- K13 check, "Residual": design the section "decision-authority" 83-84, "Two cases defeat that default,
  both narrow", while the design-audit stop is a third.
- K16 check N1: the primer's design entry lacks the trigger "when a discussion is reverting to
  multiple-choice menus, minimal-solution defaults, or approval-seeking closers"; K16 proposal: the
  claim that `cargo klarch show` on a skill's section lists its restatements was not verified.
- K17 proposal and check: planning SKILL 130-131 and setup SKILL 216 say `check --fix` writes
  "and then checks", while it runs phases 1 to 3 before writing (check note 4); K17's check probed
  a conflicted `index.md` and found `klarch index` exit 0 there, so the proposal's lead on point 3
  is not borne out for markers between table rows (markers outside the table not probed).

### Snapshot observations met during the run

Four agents state that their delivered copies predated [S9], the case K2's Edit 2 (C13)
addresses, and one check observed K1's defect in use:

- `L2-b/return.md` and `L3-a/return.md` (quoted above): the session's copies of the root CLAUDE.md
  and the primer lacked the docs/audit-reports/ line and row.
- `verify/L2-c/verdicts.md`, on L2-c/d3: "the delivered primer in this verifier's context lacks
  that row, as the delivered CLAUDE.md says "adds one" where worktree line 352 says "adds two"".
- `repairs/K2/check.md`, "Supporting evidence for Edit 2": "The copy of the primer this session
  received lacked the row of the knowledge table that the worktree's primer holds at
  .claude/knowledge-architect/PRIMER.md:115".
- `repairs/K1/check.md`, note 5: "This check ran `cargo klarch show` in the shared `reading`
  worktree, as this pass's brief directs (script 237). That is the K1 defect itself, observed in
  use."

### Steps that failed or returned nothing

None found in the files. Every step wrote its file: 17 returns and their drafts, 17 verdicts
files, the confirmed-findings file, two groupings, the causes file, 19 proposals and 19 checks
(K1 to K19, each with both), and the cross-check. The run's log is not among the files this report
read, so a step that returned and wrote nothing usable would show only as a missing file, and none
is missing. Seven verdicts files have no count line (named in section 1). No L2 group was left
unwalked.
