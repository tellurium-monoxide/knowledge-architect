# Slice 4: defaults and findings

## Builds

Per "Bounded work and a derived thread proceed on their default" in the milestone document, and its
mapping table:

- `path@agent-skills@content/skills/planning/SKILL.md`:
  - `skill@knowledge-architect-planning@planning-terms`, its row **material**: a reopened thread
    closes again as `skill@knowledge-architect-design@decision-authority` says after slice 3;
  - `skill@knowledge-architect-planning@plan-reviews`: a material finding the record or a source
    answers is applied and reported, never listed as awaiting the owner; the defaults awaiting the
    owner hold forks, per D7;
  - `skill@knowledge-architect-planning@working-a-slice`: its paragraph "Where the owner's word is
    awaited", which names the default of `primer@owner-word-premise`, follows slice 1's rewrite of
    that section;
  - `skill@knowledge-architect-planning@plan-document-leaves`: "every default the document lists as
    awaiting the owner is ruled" stays, since the list now holds forks only.
- `skill@knowledge-architect-review@what-review-leaves`: an item whose outcome the record or a
  source decides takes it from the session; only a fork, a contradiction, a goal or an action that
  cannot be undone is put to the owner.
- `skill@knowledge-architect-project-audit`, its common outcome table: a verdict where a rule reads
  two ways goes on the owner list only where no record states the reading. The design-record
  axis's owner list keeps a head in the wrong Component, per `design@agent-skills@design-record-axis`;
  the agentic-workflow axis's owner list is unchanged, per D14. The stop of passes and net growth
  stay with the owner, per D3.
- `skill@knowledge-architect-review@what-review-leaves`, its "Each outcome is reported to the owner,
  in the record of the review and at the end of the turn": the end of the turn is the turn's
  "Decided" section, per `primer@decided-section`.
- `skill@knowledge-architect-setup`: the destination of an existing document, per D5.
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
`design@agent-skills@audit-outcomes`, `design@agent-skills@audit-method`,
`design@agent-skills@design-record-axis`, `design@agent-skills@adopting-existing-docs`;
`tripwire@agent-skills@audit-defaults-overruled`, `tripwire@agent-skills@structure-rejection-overruled`,
`tripwire@agent-skills@a-head-verdict-is-overruled`.

## Fails alone on

- A finding that is a fork no longer reaches the owner.
- The checker's plan-section check fails on a plan document written under the new planning text.

## Premises that expire

None beyond the milestone's.

## Implementation sequence

1. Planning and review.
2. The project audit and setup; `cargo klarch install-agent-skills`; the CHANGELOG.md entry.
3. Review, then the harvest, then the milestone document's deletion.

## Harvest

| what | where | when |
| --- | --- | --- |
| #derived-decision-proceeds-on-default, for defaults | the head of #questions-go-to-their-authority gains its deliveries in planning and review | this slice |
| `design@agent-skills@ruled-items-labelled`, its `D` row | rewritten | this slice |
| `design@agent-skills@audit-outcomes`, `design@agent-skills@adopting-existing-docs` | rewritten in place | this slice |
| this spec, and the milestone document | deleted | the commit that completes this harvest |
