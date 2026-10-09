---
kind: question
---
# Entry test 2 of the primer's section on design heads does not say what a site is when the decision is about installed text

## Summary

Test 2 admits a decision whose reason "must be respected at more than one site, or at none", "none"
including "a policy with no code of its own". A decision about an installed text is always such a
policy, so the clause can be read as admitting every one of them, and a session counted two skills
stating one decision as one site. Finding W1 of 2026-10-07-knowledge-architect-workflow.

## Details

### What

The test is test 2 of `primer@design-heads`. In the
session that rewrote the entry tests, an audit of every design head judged the heads about installed
text by its own reading: for an instruction, the site is the text that states it. It counted the
sites of one decision as one where two skills state it. The owner, given the verdicts: "The problem
here is that I do not understand why they do not pass the tests. Can you make a breakdown ?" Only a
breakdown written test by test showed the miscount.

The retrospective proposed: for an instruction, the site of a reason is the text that states it,
and "a policy with no code of its own" is a policy no text states either. That narrowing contradicts
recorded intent:

- `skill@knowledge-architect-decision-recording@when-recording-happens`: a policy is admitted by test 2 "as a decision with no site of its own";
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: a policy or an absence "has no site at
  all";
- the rejected alternative on the earlier test 2, in `path@agent-skills@docs/rejected-alternatives.md`,
  says the same.

For installed text, `design@agent-skills@instruction-record-is-minimal` already narrows what is
recorded: "A rewording, and the rationale for how one instruction is phrased, is not recorded."

### Why it matters

An entry test read two ways admits or refuses heads by the reading, which is what
`design@agent-skills@a-head-is-owed-by-an-entry-test` exists to prevent, and a miscount went
unseen until the owner asked for a breakdown, against `goal@knowledge-architect@the-owner-decides`.

### What would close it

A ruling between two shapes, at least, under `skill@knowledge-architect-design`, since either
touches a head's argument:

- define a site for installed text: each text that states the reason is one site, and "no site" is
  kept for a policy no text states;
- or keep test 2, and let test 2 point at `design@agent-skills@instruction-record-is-minimal` for
  installed text.

`issue@agent-skills@test-3-admits-a-practice-its-tool-documents` is a question on the same tests,
and a discussion of one reads the other.
