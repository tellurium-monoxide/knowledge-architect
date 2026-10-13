# Slice 2: attribution

## Builds

- `primer@design-heads`: the ground, "The owner's words are a ground, quoted", and test 4, "The
  head quotes the owner's words and the owner's answer", become attributions, per "Attribution
  replaces quotation" in the milestone document.
- `skill@knowledge-architect-decision-recording`: its rules on recording the owner's words in a
  head, a commit and a tripwire, rewritten to attribute; the state "ruled out" written `ruled-out`
  in its table of losing alternatives.
- `agent@knowledge-architect-transcript-reviewer`: its ruling check scoped to rulings on intent; an
  answer on a reading or a fact checked against its source and reported as a reading (W2).
- `agent@knowledge-architect-design-record-auditor` and the audit's quotation check of
  `skill@knowledge-architect-project-audit`: an attribution is confirmed against the question it
  answered.
- The in-change path's commit message, in `skill@knowledge-architect-design`, and the plan
  document's record, in `skill@knowledge-architect-planning`: rulings on intent kept as the owner
  gave them, answers recorded with their source; verbatim stays in a plan document's items (D8 for
  a file that leaves).
- `tripwire@agent-skills@owner-intent-stripped` reworded to fire on a lost attribution;
  `tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change`,
  `tripwire@agent-skills@head-created-without-deliberation` reread and reworded where "the owner's
  words" counts quotations.
- The heads slice 1 wrote, converted.
- CHANGELOG.md `Next release`: a Migration entry naming the design-record axis, per D9.

## Claims

- **No installed text requires a verbatim quotation of the owner outside a plan document or a file
  that leaves.** Test: `grep -rniE "verbatim|quot" crates/agent-skills/content`, each hit read and
  classed in the commit; a wrong implementation leaves test 4's "quotes the owner's words".
- **The transcript reviewer reports thaum's Q11 as a reading, not as a misstated ruling.** Test: a
  fresh reviewer given the new agent text and the case of the retrospective's W2 says which it
  reports; the old text gives "misstated ruling".
- **`tripwire@agent-skills@owner-intent-stripped` does not fire on a quotation replaced by a correct
  attribution.** Test: read its new firing clause against that edit, and against an edit that drops
  the attribution, which must fire it.

## Audit subjects

`primer@design-heads`; `path@agent-skills@content/skills/decision-recording/SKILL.md`,
`path@agent-skills@content/agents/transcript-reviewer.md`,
`path@agent-skills@content/agents/design-record-auditor.md`,
`path@agent-skills@content/skills/project-audit/SKILL.md`, the in-change path of
`path@agent-skills@content/skills/design/SKILL.md`, the assembly of
`path@agent-skills@content/skills/planning/SKILL.md`; the heads and tripwires slice 2 rewrites,
listed in the milestone document; `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow`.

## Fails alone on

- A head whose ground is the owner's reads, after conversion, as standing on an argument.
- An audit verifier cannot check an attribution against the transcript the way it checked a
  quotation.

## Premises that expire

None beyond the milestone's.

## Implementation sequence

1. The primer's ground and test 4; the decision-recording skill.
2. The transcript reviewer and the auditors.
3. The design and planning skills' recording rules; the tripwires; the slice-1 heads converted;
   `cargo klarch install-agent-skills`; the CHANGELOG.md entry.
4. Review, then the harvest.

## Harvest

| what | where | when |
| --- | --- | --- |
| #attribution-replaces-quotation | `design@agent-skills@head-ground-is-the-argument` and `design@agent-skills@a-head-is-owed-by-an-entry-test` rewritten | this slice |
| #answer-keeps-its-authority, for review | `design@agent-skills@transcript-reviewer-agent` and `design@agent-skills@misstated-ruling-rated-by-what-it-changes` rewritten | this slice |
| the other heads listed for slice 2 in the milestone document | rewritten in place | this slice |
| T4 | `register@agent-skills@tripwire` | this slice |
| `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` | closed | the commit of this harvest |
| this spec | deleted | the commit that completes this harvest |
