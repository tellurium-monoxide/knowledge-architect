# Slice 4: defaults and findings

## Builds

Per "Bounded work and a derived thread proceed on their default" in the milestone document, and its
mapping table:

- `skill@knowledge-architect-planning@plan-reviews` and
  `skill@knowledge-architect-planning@working-a-slice`: a material finding the record or a source
  answers is applied and reported, never listed as awaiting the owner; the defaults awaiting the
  owner hold forks, contradictions, goals and irreversible actions; the condition before a plan
  document leaves follows. The section keeps its name, per D7.
- `skill@knowledge-architect-review@what-review-leaves`: an item whose outcome the record or a
  source decides takes it from the session; only a fork, a contradiction, a goal or an action that
  cannot be undone is put to the owner.
- `skill@knowledge-architect-project-audit`: the owner list holds what the authority section
  reserves; a verdict the record states, a provable gap whose fill a source fixes, and a head in
  the wrong Component take their outcome from the session. The stop of passes and net growth stay
  with the owner, per D3.
- `skill@knowledge-architect-setup`: the destination of an existing document, per D5.
- In this repository: `skill@klarch-retrospective-intake` and
  `design@knowledge-architect@committed-findings-analysis`, per D6;
  `path@knowledge-architect@docs/retrospective-reports/README.md` and
  `path@knowledge-architect@docs/audit-reports/README.md` follow.
- CHANGELOG.md `Next release`: a Workflow entry for each question these skills stop putting to the
  owner.

## Claims

- **A plan review's finding that a head answers does not reach the owner.** Test: the owner's case
  of a31, a default answerable from a recorded head, read against the new planning text; the old
  text lists it as `D<n>`.
- **Every question the four skills still put to the owner is of a kind the mapping table marks
  "stays".** Test: each remaining "put to the owner", "ask the owner" and "owner's word" site in the
  four skills, read and classed in the commit.

## Audit subjects

`path@agent-skills@content/skills/planning/SKILL.md`, `path@agent-skills@content/skills/review/SKILL.md`,
`path@agent-skills@content/skills/project-audit/SKILL.md`,
`path@agent-skills@content/skills/setup/SKILL.md`, each whole;
`path@agent-config@skills/klarch-retrospective-intake/SKILL.md`;
`design@agent-skills@audit-outcomes`, `design@agent-skills@audit-method`,
`design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@adopting-existing-docs`,
`design@knowledge-architect@committed-findings-analysis`;
`tripwire@agent-skills@audit-defaults-overruled`, `tripwire@agent-skills@structure-rejection-overruled`,
`tripwire@agent-skills@a-head-verdict-is-overruled`.

## Fails alone on

- A finding that is a fork no longer reaches the owner.
- The checker's plan-section check fails on a plan document written under the new planning text.

## Premises that expire

None beyond the milestone's.

## Implementation sequence

1. Planning and review.
2. The project audit and setup; `cargo klarch install-agent-skills`.
3. This repository's intake and its READMEs; the CHANGELOG.md entry.
4. Review, then the harvest, then the milestone document's deletion.

## Harvest

| what | where | when |
| --- | --- | --- |
| #derived-decision-proceeds-on-default, for defaults | a head in `register@agent-skills@design`, slug its thread's | this slice |
| the heads listed for slice 4 in the milestone document | rewritten in place | this slice |
| this spec, and the milestone document | deleted | the commit that completes this harvest |
