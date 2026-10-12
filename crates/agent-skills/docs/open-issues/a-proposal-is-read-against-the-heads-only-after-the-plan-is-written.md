---
kind: todo
---
# A design discussion's proposals are read against the heads they may contradict only after the plan document is written

## Summary

The design skill grounds the discussion in the record before proposing, and reads no shaped
proposal against the heads it may contradict before the owner rules on it. The design-conformance
reviewer does that reading after the plan document exists. In one real case, the owner approved a
proposal that contradicted a head, and the contradiction cost a material-finding round, a default
and a second review cycle.

## Details

### What

- The design skill's loop, its first step: "**Ground first**: read the project's record before
  proposing", which reads the goals, design homes and rejected alternatives of the Components the
  question touches. Its later steps hold no reading of each shaped proposal against the heads. The
  reversal check of `skill@knowledge-architect-decision-recording@reversal-check` is asked on the
  bounded path only.
- `design@agent-skills@plan-read-against-the-record` places the reading of a plan document against
  the record after the plan document is written, by the design-conformance reviewer.
- The case, this repository's discussion that produced `spec@plans@path-quickfixes`: one row of an
  approved proposal rewrote a reference's kind claim from the tree. A core design head says the
  claim sits in the span "rather than inferred from what happens to exist", and the tripwire the
  proposal fired had been recorded from a premortem cause naming that very fix. The session had
  read the general heads of the subject, not that one. The owner approved the row; the spec's
  decision-record and design-conformance reviewers found the contradiction.

### Why it matters

The owner ruled without the contradicted head in view, and ruled twice, against
`goal@knowledge-architect@the-owner-decides`. The catch exists, after the plan; the cost is a ruling
given on an incomplete premise and a review cycle.

### What would close it

A change to the design skill's loop, argued under `skill@knowledge-architect-design`, that reads the
checkpoint's proposals against the heads they may contradict before convergence: the reversal
search by wording run by the session over the checkpoint's rows, or a reviewer dispatched at the
checkpoint. Or a recorded decision that the reading stays after the plan, with the reason; a second
case of a ruling given against an unread head is the evidence that would weigh against that.
