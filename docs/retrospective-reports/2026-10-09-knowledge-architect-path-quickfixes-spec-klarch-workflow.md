# Analysis of 2026-10-09-knowledge-architect-path-quickfixes-spec-klarch-workflow

- **Received file:** 2026-10-09-knowledge-architect-path-quickfixes-spec-klarch-workflow.md, the
  workflow's side of this repository's own retrospective of the session that wrote
  `spec@plans@path-quickfixes`. The project's side is analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-09-knowledge-architect-path-quickfixes-spec.md`.
- **Version it used:** built from this repository's source, main at v0.5.0-51-g66168e4.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1. The spec is still on main and its work has
  not started: no source file of the core holds its respelling, and its status section says the
  work "starts with the design audit". Several findings stand on it.
- **Standing entries that bear on it:**
  - `tripwire@agent-skills@design-conformance-findings-needless`: W1 is evidence against its
    premise, since that reviewer found a real contradiction; not fired.
  - `tripwire@core@fix-makes-a-choice`: W4's instance, narrowed to except the spec's work.
  - `issue@knowledge-architect@command-output-is-not-declared-a-contract` records C2.
  - `tripwire@agent-skills@plain-text-pointer-found`: C3's passages are judged in the section on
    that tripwire in
    `path@knowledge-architect@docs/retrospective-reports/2026-10-10-knowledge-architect-design-record-audit-klarch-workflow.md`.

## W1: no step reads an approved shape against the heads it may contradict before the owner rules

- **Says.** One row of an approved proposal contradicted a design head and fired a tripwire whose
  premortem cause named that very fix. The session had read the general heads of the subject, not
  that one. The owner approved the row; the decision-record and design-conformance reviewers of the
  spec found the contradiction, at the cost of a material-finding round, a default and a second
  review cycle. The report proposes that, before convergence on the full path, the design skill
  reads the checkpoint's proposals against the record: the design-conformance reviewer, or the
  reversal search by wording of the decision-recording skill.
- **Still applies.** Yes. `path@agent-skills@content/skills/design/SKILL.md`:446-449, "1. **Ground
  first**: read the project's record before proposing." The later loop steps hold no reading of each
  shaped proposal against the heads. The reversal check is asked on the bounded path only (:192).
  The design-conformance reviewer runs after the plan document is written, per
  `design@agent-skills@plan-read-against-the-record`.
- **Recorded.** No issue, no rejected alternative.
- **Kind.** A defect of the text, missing, with one real case. The design is caught later by design;
  the cost is the owner ruling twice.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`: the owner ruled without the
  contradicted head in view. Owned by agent-skills, the design skill's loop. A new step of the loop
  is a decision about the discussion's shape, under `skill@knowledge-architect-design`, and it reads
  against `design@agent-skills@plan-read-against-the-record`, which places the reading after the
  plan.
- **Better fix.** Rivals: a reviewer dispatched at the checkpoint, which costs a subagent per
  discussion; the reversal search of `skill@knowledge-architect-decision-recording@reversal-check` run
  by the session over the checkpoint's rows, which costs a grep per row; no change, the later reader
  catching it at one review cycle's cost. One case does not settle the choice.
- **Route.** A decision, under `skill@knowledge-architect-design`; not for this intake's branch.
- **Proposal.** Open an issue, kind `todo`, in agent-skills: a proposal is read against the heads
  it may contradict only after the plan document is written.

- **Default:** open an issue, `todo`, in agent-skills.
- **Outcome:** approved.

## W2: a parked thread's tripwire has no label, so a ruling on "all tripwires" does not clearly reach it

- **Says.** The owner answered "Keep all tripwires and AC, proceed." after a premortem labelled T1
  and T2, while a parked thread's tripwire stood unlabelled in the checkpoint table. Whether the word
  covered it became a default put to the owner. The report proposes labelling every tripwire put to
  the owner, a parked thread's included, in the one `T<n>` sequence.
- **Still applies.** Partly repaired. 2bf743f and 6ee997d, passes 1 and 4 of the agentic-workflow
  audit, give a parked thread's tripwire the owner's word: the design skill, :361-363, "A parked
  thread's tripwire is named on the ledger, and written as
  `skill@knowledge-architect-decision-recording@ruled-tripwires` says"; that section,
  `path@agent-skills@content/skills/decision-recording/SKILL.md`:244-247, "each by the label,
  `T<n>`, it was put to the owner under. **A tripwire is written only on the owner's word, and with
  the decision it guards**, whatever named it, a parked thread included". The head
  `design@agent-skills@ruled-items-labelled` labels every item put to the owner by name. The design
  skill's step 7, :613-616, still gives `T<n>` to the premortem's survivors only, and no sentence
  says that a parked thread's tripwire takes a number in the same sequence.
- **Recorded.** No.
- **Kind.** A defect of the text, unclear; implied by the head and the decision-recording section,
  not stated where the label is given.
- **Scope.** It serves `goal@knowledge-architect@the-owner-decides`. Owned by agent-skills, the
  design skill. It applies `design@agent-skills@ruled-items-labelled`, a member its argument covers.
- **Better fix.** The report's proposal is the fix: at the parked thread's tripwire, "put to the
  owner under a label, `T<n>`, in the premortem's sequence". No change is the rival; it left one
  default to the owner here.
- **Route.** A text edit whose design is settled, at the design skill's source.
- **Proposal.** Handle now.

- **Default:** handle now.
- **Outcome:** approved.

## W3: the discussion's `#<id>` rule is carried into the plan document

Cluster K4, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`.
The spec's 68 plain pointers to its own items have since been rewritten; 121 checked item
references stand, and two plain names remain inside quotations.

- **Default:** handle the design skill's sentence now, with K4.
- **Outcome:** approved.

## W4: a tripwire that fires on a proposal whose reversal waits on a plan document

- **Says.** A tripwire fired on a proposal; the discussion reopened its decision and ruled, but the
  reversal is recorded at the harvest, after work scheduled for later. Three reviewers reported the
  entry as fired and still listed. The owner approved a default: the firing clause excepts the
  planned work, citing the spec, so the citation dangles when the spec leaves and forces the harvest
  to delete the entry. The report proposes that the issue-tracking skill names this case and shape.
- **Still applies.** Yes. `path@agent-skills@content/skills/issue-tracking/SKILL.md`:262-266: "A
  tripwire that **fires** has its heading and body **deleted** … It never becomes a
  fired-but-still-listed entry." The case is named nowhere. The instance is live:
  `tripwire@core@fix-makes-a-choice` reads "other than by the respellings `spec@plans@path-quickfixes`
  defines, which reverse this head at that spec's harvest. That harvest deletes this entry."
- **Recorded.** No.
- **Kind.** A defect of the text, missing: the movement instruction assumes the decision is
  reopened at once, and a full-path reversal is recorded at a harvest. One case, whose shape the
  owner approved.
- **Scope.** Owned by agent-skills, the issue-tracking skill's movement instruction. It reverses
  nothing: it names the shape the movement instruction admits for a reversal that waits.
- **Better fix.** Rivals: delete the tripwire at once and let the spec carry its condition, which
  leaves the head unguarded against every other cause until the harvest; no change, which costs
  three reviewer reports per occurrence. The owner-approved shape keeps the guard and ends it
  mechanically, since the citation dangles at the spec's deletion.
- **Route.** A text edit whose design is settled, at the issue-tracking skill's source.
- **Proposal.** Handle now: one sentence naming the shape.

- **Default:** handle now.
- **Outcome:** approved.

## W5, with P1 of 2026-10-09-knowledge-architect-path-quickfixes-spec: no home for the instrument that re-takes a figure

- **Says.** A census over session logs was taken by scripts in the session's scratch directory. No
  row of the knowledge table covers a measuring instrument; the scripts were lost; a parked thread's
  tripwire asks for the census again over another project. The reports propose a row, or a sentence
  that such scripts are not kept and the method is described in prose.
- **Still applies.** The workflow file attributes the sentence "A figure carries the claim it
  supports, the command that re-takes it…" to the primer; it is the root's,
  `path@knowledge-architect@CLAUDE.md`:232-233. The primer has "**A measurement is routed by what it
  serves**" (`path@agent-skills@content/PRIMER.md`:122-124) and "the measurement it rests on, with
  the command that takes it again" (:254). No row names an instrument. The instance was ruled by the
  owner, D6 of the spec: "the scripts are not kept; the census method is described under Names".
- **Recorded.** No issue.
- **Kind.** The knowledge table worked as designed: its last row, "ask the owner before writing it
  anywhere", applied, and the owner ruled. One case.
- **Scope.** A row would be a decision of the primer, under `design@agent-skills@primer-limit`, or of
  the root's table.
- **Better fix.** No change, until a second instrument meets the same question. The owner's D6
  settles this one.
- **Proposal.** No change.

- **Default:** no change; the table's last row applied, and the owner ruled the instance.
- **Outcome:** approved.

## C1: a plain `#<id>` naming one of a plan's own items, inside that plan, is not reported

Cluster K4, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`.
Confirmed on main: `cargo klarch check` passes while the spec holds two plain names of its own
items.

- **Default:** open an issue, `design`, in core, with K4.
- **Outcome:** approved, as ruled in the section that holds the analysis. The owner: "For those that
  were pointing to another place for the analyis, I only ruled in the place that shows the
  analysis."

## C2: no machine-readable form of the findings

- **Recorded** as `issue@knowledge-architect@command-output-is-not-declared-a-contract`, which
  quotes the owner: "the project should be able to output data in a stable, documented machine
  readable format". The census is one more use for it.

- **Default:** no change, recorded.
- **Outcome:** approved.

## C3: no checked way to quote verbatim text that holds the checker's syntax

Cluster K5, analysed in
`path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`.
The spec's passage written bare beside no issue entry is the instance on main.

- **Default:** open an issue, `todo`, in core, with K5; then the spec's passage cites it.
- **Outcome:** as ruled in C5 of the agent-configuration-entities analysis: no checked form is
  provided; `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` is widened to the place and
  importance of the owner's quotes in the workflow; the spec's bare passage is rewritten in words.
  The owner: "Agreed on both points, proceed".

## Standing answers

- **A figure relayed without reading the disagreements in context.** The census's 7 became 4 on
  reading; the spec now states both. A lapse against `primer@intent-and-claims`.
- **A "breach" written unchecked, and a default contradicting a head.** Lapses; the second was
  caught by the re-review, as cc6ac92 records.
- **References written without backticks.** The two quotations of C3.

- **Default:** no change.
- **Outcome:** approved.
