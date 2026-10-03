# Step 3, #workflow-text: the installed text, this repository's configuration, and built intent

This is the spec of step 3 of the milestone in `README.md` beside it. It holds the step's entry. The
design it implements is the whole of the milestone document's "Decided design" from the installed
text's side. Read that document entire first, then `knowledge-architect-agent-configuration`,
which governs every edit below. It depends on default D3 of the milestone document.

## Builds

The installed text lives under crates/agent-skills/content/ and names no project, per
`design@agent-skills@shipped-text-is-reference-free`. Each site below was read at main's commit
90a4b56.

1. **The primer**, `path@agent-skills@content/PRIMER.md`:
   - its "Intent and claims" bullet, *"A design home is intent. Check the code against it, never
     the other way."*, reworded to built intent, covering a decision recorded when made (clause
     P9);
   - its knowledge-table row for unbuilt work, today *"the project's plans directory, named in its
     root `CLAUDE.md`"*, names docs/plans/ and its two homes, in plain text, since shipped text
     holds no live
     reference;
   - a row for the roadmap, docs/roadmap.md.
2. **The planning skill**, `path@agent-skills@content/skills/planning/SKILL.md`:
   - Terms: the plans directory is docs/plans/, in plain text; plan document, spec, milestone with
     their kinds;
   - §1: the ledger lives in the transcript as well as the conversation;
   - §2: the roadmap replaces *"There is no roadmap file"*;
   - §3: the layout with `specs/` and `milestones/`, and the generated indexes;
   - §4: the Arguments section, right after Threads, and the section titles written in sentence
     case as the documents write them, as the owner ruled in step 2's design session, so that §4
     lists the sections `design@core@step-spec-sections` checks; items as `### <statement> ##<id>`; the full record of the
     exchange (`thread@structured-plans@spec-records-the-exchange`); assembly from the transcript by a subagent, from every
     transcript the discussion spans (`thread@structured-plans@ledger-from-transcript`, clause P3), with boundaries of
     arguments decided at assembly (`thread@structured-plans@argument-segmentation`); the identifiers paragraph;
   - §8: the transcript reviewer dispatched on every assembled document;
   - §9: the deleting commit cites the plan by its kind, `spec@plans@<id>` or
     `milestone@plans@<id>`; the procedure of `thread@structured-plans@retiring-plan-opens-issue` and clause P8.
3. **The design skill**, `path@agent-skills@content/skills/design/SKILL.md`: step 8 and the hand-off
   say the ledger is
   assembled from the transcript; the per-round delta is the draft.
4. **The decision-recording skill**, `path@agent-skills@content/skills/decision-recording/SKILL.md`:
   §0 reworded to
   built intent; the `git log --diff-filter=D` line names the plans homes.
5. **The issue-tracking skill**, `path@agent-skills@content/skills/issue-tracking/SKILL.md`: a
   `todo` issue may carry
   a roadmap row; the issue `thread@structured-plans@retiring-plan-opens-issue` opens.
6. **The setup skill**, `path@agent-skills@content/skills/setup/SKILL.md`: §4's table gains the
   plans homes for the
   root; §5 no longer proposes a plans-directory row; the roadmap is mentioned as optional.
7. **The agent-configuration skill**,
   `path@agent-skills@content/skills/agent-configuration/SKILL.md`: its two
   examples naming "the path of its plans directory" as a project row.
8. **The review agents**: `path@agent-skills@content/agents/routing-reviewer.md` (*"carrying no slug
   so that nothing
   can cite it as settled. A slug on unbuilt work is a finding."*) and
   `path@agent-skills@content/agents/decision-record-reviewer.md` (*"plan documents hold shapes for
   unbuilt work that
   are deliberately not decisions and carry no slugs"*): items are slugs scoped to their plan, and
   a design-register slug on unbuilt work stays a finding.
9. **This repository's configuration**, under `knowledge-architect-agent-configuration`:
   `cargo klarch install-agent-skills`; the root `CLAUDE.md` is the owner's configuration, so its
   diff is shown to the owner before it is committed (round 3); it loses its plans-directory row and
   rewrites its "Plan documents" section and its "Intent" bullet;
   `path@plans@README.md` describes
   the two homes; CHANGELOG.md's `Next release` section gains this step's entries, under the tests of
   `design@knowledge-architect@changelog-entries`.

### The design audit's findings, applied as the step's binding shape

The audit read every site above again at main's commit 536f0b2, which holds steps 1 and 2. Eight
gaps have one answer this document's decisions imply, and are applied in place. Two answers add an
obligation to a ruling of the owner, so they are defaults awaiting the owner, listed after them.

1. **§3 of the planning skill says "A plan document carries no slug anchor".** Step 2 made a
   plan's items slugs. Answer: §3 says a plan document defines items, cited from inside the plan
   only, and defines no design entry. It follows from `design@core@plan-items-by-section` and
   `design@core@plan-item-scope`.
2. **§4's table describes threads, criteria and acceptance criteria as table rows with columns,
   and has no row for arguments.** Answer: each of the three rows describes an item,
   `### <statement> ##<id>`, with its fields, and an Arguments row follows the threads row. The
   fields are those this milestone document carries:
   - a thread: first appears, proposer, final state, arguments, shape in, harvest home, and closed
     by, with the owner's ruling verbatim and its round;
   - an argument, `##a<n>`: round, who, threads it bears on, and key words verbatim;
   - a criterion: kind, source, satisfaction;
   - an acceptance criterion: guards, judged at, fires when, response.

   It follows from `thread@structured-plans@spec-records-the-exchange`,
   `thread@structured-plans@arguments-as-items` and `thread@structured-plans@argument-ids`.
3. **§6 and §7 point 5 have a landing commit report on each acceptance criterion, and a commit
   message cannot cite an item.** Answer: the report names each criterion by its identifier in
   plain text, with a hash sign, and cites the plan whole. It follows from default D12 and
   `design@core@plan-item-scope`.
4. **§4 of `knowledge-architect-decision-recording` says the commit that deletes a spec "names its
   path".** A `path` citation of a plan document is refused since step 1. Answer: that commit cites
   the plan document by its kind, as Builds item 2 already says of the planning skill's §9. It
   follows from `design@core@every-path-names-its-anchor`.
5. **The routing reviewer says "A `path` reference to a whole plan document is allowed."** Answer
   as finding 4: a whole plan document is cited by its kind, from anywhere.
6. **§5 of the retrospective skill, and `path@agent-skills@README.md` which restates it for the
   owner, say the ledger "lives in the conversation".** Answer: the ledger lives in the
   conversation and in the harness's transcript of it, which assembly reads. It follows from
   `thread@structured-plans@ledger-from-transcript`.
7. **§2 must say how a roadmap row changes when its entry changes.** The commit that adds a plan
   document closes the issue it schedules (§2), so that issue's row dangles
   (`argument@structured-plans@a21`). Answer: that commit rewrites the row to cite the plan
   document. The commit that deletes a plan document removes its row, since the work landed. It
   follows from `thread@structured-plans@roadmap-orders-issues`.
8. **The planning skill's description lists what it covers, and lacks the roadmap and assembly
   from the transcript.** Answer: the description names both, so that a session about to edit the
   roadmap finds the skill. It follows from `thread@structured-plans@roadmap-home` and
   `thread@structured-plans@ledger-from-transcript`.

Read and found to hold, with no change:

- `path@plans@README.md` already describes the two homes, since step 1.
- The root `CLAUDE.md`'s restatement of the four cases, under "Precedent is not authority",
  restates the primer's four cases, which this step does not change.

**Default D20, awaiting the owner: the issue a leaving plan's session opens on a citing plan is a
`question`.** Clause P8 does not name its kind. The question is whether the citing plan still
holds now that the leaving plan is built. A reading of the citing plan against the leaving plan's
harvested design entries answers it. The rival is a `todo`, whose closing condition is doing work.
The discriminating fact: the reading may find nothing to change, and a `todo` would then close with
no work done.

**Default D21, awaiting the owner: the roadmap's order is the owner's.** A row is added or moved
on the owner's word. The two edits of finding 7 are mechanical and need no word: they keep a row
pointing at the same work. The rival is that any session orders the work. Ordering work is a
weighing, and the weighing is the owner's, per `goal@knowledge-architect@the-owner-decides`. This
adds an obligation to the ruling of `thread@structured-plans@roadmap-orders-issues`.

## Claims

The installed text has no unit tests; its claims are checked by the checker and by review.

- **`cargo klarch check` passes** after the reinstall: the installed copies match the shipped text.
- **No shipped file holds a live reference**: a grep of crates/agent-skills/content/ for a
  backticked
  `@` span whose head is a kind finds only placeholders in angle brackets.
- **No two installed texts contradict** on the plans directory, the roadmap, assembly or built
  intent: the self-consistency axis of `knowledge-architect-review`.
- **`acceptance@structured-plans@roadmap-needs-no-code`** of the milestone document does not fire.

## Fixtures

None: the step changes no code.

## Audit subjects

- every head named in the harvest section's step 3 row of the milestone document;
- `tripwire@agent-skills@ledger-lost-before-hand-off`, judged at this step's harvest per the
  milestone document's "What is already decided";
- `path@agent-skills@docs/rejected-alternatives.md`, the entry "Writing the discussion's ledger to a
  file during the discussion", which stays as it is;
- the root `CLAUDE.md`, its knowledge-table row for unbuilt work and its "Plan documents" section;
- `knowledge-architect-agent-configuration` on editing installed text.

## Fails alone on

- the installed copy is reported out of date, or a shipped file holds a live reference;
- the root `CLAUDE.md` restates a directive the primer now contradicts.

## Premises that expire

None beyond the milestone's: this is the last step, and this milestone document leaves in its
harvest commit.
