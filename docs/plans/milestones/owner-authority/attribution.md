# Slice 2: attribution

## Builds

Per "Attribution replaces quotation" and "An answer keeps its authority" in the milestone document:

- **`primer@design-heads`**: the ground, "The owner's words are a ground, quoted", and test 4, "The
  head quotes the owner's words and the owner's answer", become attributions.
- **`skill@knowledge-architect-decision-recording`**: its rules on the owner's choice as a ground
  and on a verbatim quotation kept on a rename, read against the primer's new rule; the state
  "ruled out" written `ruled-out` in its table of losing alternatives.
- **`skill@knowledge-architect-issue-tracking`**: its twin rule on a verbatim quotation that names a
  renamed entry, per D11.
- **`agent@knowledge-architect-transcript-reviewer`**: its ruling check scoped to rulings on intent;
  an answer on a reading or a fact checked against its source and reported as a reading (W2).
- **`agent@knowledge-architect-decision-record-reviewer`**, its check of "A quotation of the owner a
  head cites as its ground"; **`agent@knowledge-architect-design-record-auditor`**, its verifier,
  and the quotation check of `skill@knowledge-architect-project-audit`; and the quotation check of
  `agent@knowledge-architect-workflow-auditor`: an attribution is confirmed against the question it
  answered.
- **The in-change path's commit message**, in `skill@knowledge-architect-design`, and **the plan
  document's record**, in `skill@knowledge-architect-planning`: rulings on intent recorded as the
  owner gave them, answers recorded with their source; verbatim stays in a plan document's items,
  and in a file that leaves, per D8.
- **The tripwires**: `tripwire@agent-skills@owner-intent-stripped` reworded to fire on a lost
  attribution, in the same commit as the first quotation this slice removes;
  `tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change`,
  `tripwire@agent-skills@head-created-without-deliberation` reread and reworded where "the owner's
  words" counts quotations.
- **D19**: the approval clause of `primer@design-heads` and of
  `design@agent-skills@head-ground-is-the-argument`, "which the owner approves where the reversal is
  a fork".
- **What slice 1 wrote**: its heads and T2 and T3, converted.
- **CHANGELOG.md `Next release`**: a Migration entry naming the design-record axis, per D9.

## Claims

- **No installed text requires a verbatim quotation of the owner outside a plan document, a file
  that leaves, or the bounded path.** Test: `grep -rniE "verbatim|quot" crates/agent-skills/content`,
  each hit read and classed in the commit; the bounded path's "the owner's words verbatim" is
  slice 3's. A wrong implementation leaves test 4's "quotes the owner's words".
- **The transcript reviewer reports thaum's Q5 case as a reading, not as a misstated ruling.** Test:
  a fresh reviewer given the new agent text and the case of W2, as the Names table of the milestone
  document states it, says which it reports; the old text gives "recorded wider than the owner gave
  it".
- **`tripwire@agent-skills@owner-intent-stripped` does not fire on a quotation replaced by a correct
  attribution.** Test: read its new firing clause against that edit, and against an edit that drops
  the attribution, which must fire it.

## Audit subjects

`primer@design-heads`; `path@agent-skills@content/skills/decision-recording/SKILL.md`,
`path@agent-skills@content/skills/issue-tracking/SKILL.md`,
`path@agent-skills@content/agents/transcript-reviewer.md`,
`path@agent-skills@content/agents/decision-record-reviewer.md`,
`path@agent-skills@content/agents/design-record-auditor.md`,
`path@agent-skills@content/agents/workflow-auditor.md`,
`path@agent-skills@content/skills/project-audit/SKILL.md`, the in-change path of
`path@agent-skills@content/skills/design/SKILL.md`, the assembly of
`path@agent-skills@content/skills/planning/SKILL.md`; the heads and tripwires slice 2 rewrites,
listed in the milestone document; `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason`.

## Fails alone on

- A head whose ground is the owner's reads, after conversion, as standing on an argument.
- An audit verifier cannot check an attribution against the transcript the way it checked a
  quotation.

## Premises that expire

None beyond the milestone's.

## Implementation sequence

1. The primer's ground and test 4; the decision-recording and issue-tracking skills.
2. The transcript reviewer, the decision-record reviewer, the auditors.
3. The design and planning skills' recording rules; the tripwires; what slice 1 wrote, converted;
   `cargo klarch install-agent-skills`; the CHANGELOG.md entry.
4. Review, then the harvest.

## Harvest

| what | where | when |
| --- | --- | --- |
| #attribution-replaces-quotation | a head in `register@agent-skills@design`, slug its thread's; `design@agent-skills@head-ground-is-the-argument` and `design@agent-skills@a-head-is-owed-by-an-entry-test` rewritten to point to it | this slice |
| #answer-keeps-its-authority, for review | `design@agent-skills@transcript-reviewer-agent` and `design@agent-skills@misstated-ruling-rated-by-what-it-changes` rewritten | this slice |
| the other heads listed for slice 2 in the milestone document, `design@agent-skills@verifier-in-the-run` among them | rewritten in place | this slice |
| the owner's words as a ground, quoted, the old winner | `path@agent-skills@docs/rejected-alternatives.md` | this slice |
| T4 | `register@agent-skills@tripwire` | this slice |
| this spec | deleted | the commit that completes this harvest |
