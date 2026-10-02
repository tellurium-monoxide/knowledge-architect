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
   - §4: the Arguments section; items as `### <statement> ##<id>`; the full record of the
     exchange (#spec-records-the-exchange); assembly from the transcript by a subagent, from every
     transcript the discussion spans (#ledger-from-transcript, clause P3), with boundaries of
     arguments decided at assembly (#argument-segmentation); the identifiers paragraph;
   - §8: the transcript reviewer dispatched on every assembled document;
   - §9: the deleting commit cites the plan by its kind, `spec@plans@<id>` or
     `milestone@plans@<id>`; the procedure of #retiring-plan-opens-issue and clause P8.
3. **The design skill**, `path@agent-skills@content/skills/design/SKILL.md`: step 8 and the hand-off
   say the ledger is
   assembled from the transcript; the per-round delta is the draft.
4. **The decision-recording skill**, `path@agent-skills@content/skills/decision-recording/SKILL.md`:
   §0 reworded to
   built intent; the `git log --diff-filter=D` line names the plans homes.
5. **The issue-tracking skill**, `path@agent-skills@content/skills/issue-tracking/SKILL.md`: a
   `todo` issue may carry
   a roadmap row; the issue #retiring-plan-opens-issue opens.
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
   `path@knowledge-architect@docs/plans/README.md` describes
   the two homes; the CHANGELOG gains the section for the next version.

## Claims

The installed text has no unit tests; its claims are checked by the checker and by review.

- **`cargo klarch check` passes** after the reinstall: the installed copies match the shipped text.
- **No shipped file holds a live reference**: a grep of crates/agent-skills/content/ for a
  backticked
  `@` span whose head is a kind finds only placeholders in angle brackets.
- **No two installed texts contradict** on the plans directory, the roadmap, assembly or built
  intent: the self-consistency axis of `knowledge-architect-review`, and the
  `klarch-release-status-reviewer`, whose future `issue@agent-config@what-review-a-release-owes`
  decides.
- **#roadmap-needs-no-code** of the milestone document does not fire.

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
