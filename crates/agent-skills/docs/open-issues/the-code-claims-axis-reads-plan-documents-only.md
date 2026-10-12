---
kind: todo
---
# The code-claims review axis reads plan documents only, and statements about code elsewhere in a change are checked by no axis

## Summary

The code-claims reviewer is defined for "a plan document, a spec or a milestone document". A change
that writes many statements about existing code outside a plan document, in issue entries, records
or design heads, gets no axis that reads those statements against the tree. In one real case, an
improvised reviewer on that question produced the review's most consequential findings.

## Details

### What

- `agent@knowledge-architect-code-claims-reviewer`, its description: "The code-claims axis of a
  dispatched review of a plan document, a spec or a milestone document", and its scope: "every
  statement in a plan document about the code as it stands".
- `skill@knowledge-architect-review@review-axes` sends the code-claims axis at the same moment as
  the other plan-document axes: when a spec or a milestone document is written. No axis of the
  review before a merge reads a change's statements about existing code against the tree. The
  self-consistency axis judges the result against itself.
- The case, reported by the thaum project's retrospective of a roadmap revision, on
  knowledge-architect 0.5.0: the change wrote no plan document, but about 40 issue entries and a
  record of about 1000 rows, each naming types, functions and tests and saying what they do. The
  session dispatched a general-purpose subagent on that question. It found 10 wrong statements and
  about 50 imprecise ones. The report calls them the review's most consequential findings, and
  without that improvised axis none would have been read before the merge.

### Why it matters

An issue entry whose diagnosis is false of the code misleads every session that reads it before
acting, against `goal@knowledge-architect@design-is-recorded-with-its-arguments`. The review's
coverage then depends on a session thinking to improvise the axis. The reviewer's placement among
the plan-document reviewers is `design@agent-skills@plan-read-against-the-record`.

### What would close it

A change to the review skill's axis table and, if needed, to the reviewer's scope, argued under
`skill@knowledge-architect-design`: a condition on the code-claims row such as "or a change that
writes issue entries, records or heads stating what existing code does", or a wider scope of the
agent. Or a recorded decision that the axis stays on plan documents, with the reason.
