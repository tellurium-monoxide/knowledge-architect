---
kind: design
---
# A plan item named as a plain `#<id>`, inside its plan or outside it, is read by no check

## Summary

The planning skill prescribes a plain `#<id>` to name a plan item from outside its plan, and the
design skill's discussion uses the same form. The retired-form check reads backticked spans only.
So a plain `#<id>` is never checked: inside a plan document, where the checked item form exists, it
leaves the item "referenced by nothing"; outside, a wrong slug passes every gate, and a name that
should follow a thread to the head its harvest writes goes stale silently.

## Details

### What

Instances seen, in three retrospectives of two projects:

- This repository, the agent-configuration-entities milestone: a wrong slug written as an
  acceptance criterion's name in a harvest message passed every gate, and a reviewer caught it.
  Inside the milestone's documents, about 35 plain `#<id>` citations of a thread let
  `cargo klarch show` report the thread as referenced by nothing.
- This repository, the session that wrote `spec@plans@path-quickfixes`: the assembly carried the
  discussion's plain form into the spec, 68 plain pointers to the spec's own items. The routing
  reviewer found them unchecked. They have since been rewritten as checked item references; two
  plain names of the spec's own items remain inside quotations, and `cargo klarch check` passes
  with them.
- The thaum project, on knowledge-architect 0.5.0: a doc comment in source and a sibling plan
  document cited a milestone's thread as a plain `#<thread>`. When the harvest gave the decision its
  design head, nothing reported that those citations should now name the head. The routing reviewer
  found three such sites by reading.

The forms and checks as they stand:

- `skill@knowledge-architect-planning@plans-layout`: "From anywhere else, a commit message and a
  design head included, the plan is cited whole, and an item is named in plain text with a hash
  sign". `design@agent-skills@plain-text-is-no-repair` admits it.
- Inside its plan, an item is cited by its checked form, `<kind>@<plan>@<id>`, per
  `design@core@plan-item-scope`, which refuses that form from outside the plan.
- A backticked `#<id>` naming a defined entry, plan items included, is reported as the retired form,
  per `design@core@candidate-rule-and-retired-forms`. A plain one is not read.
- The design skill now says the plan document cites its own items by their checked form. One
  plain `#<thread>` inside a plan document is prescribed: `primer@design-heads` states the pair
  `#<thread> → <entry slug>` in the plan document's harvest row, so a check of the inside case
  admits that pair.

The suspected mechanism, in one sentence: the one form the workflow prescribes for an item outside
its plan is the one form the checker never reads, so its correctness rests on reviewers reading.

Two rivals already lost, and a design must answer both:

- "Citing an item of another plan", lost to `design@core@plan-item-scope`: item citations across
  plans "would force a design revisit of every citing plan, item by item, at the moment a plan is
  retired".
- "The retired slug reference reported whatever it names", lost to
  `design@core@candidate-rule-and-retired-forms`: a backticked `#<id>` is often another project's
  notation, such as an issue number. A check limited to a plan document's own item ids, inside that
  document, does not meet that argument: its set of ids is the plan's own.

Re-entry point: the next change to `design@core@candidate-rule-and-retired-forms` or to
`design@core@plan-item-scope`, or earlier on the owner's word.

### Why it matters

A wrong or stale item name in a commit message or a design head is a pointer nobody revisits,
against `goal@knowledge-architect@design-is-recorded-with-its-arguments`, and
`tripwire@core@candidate-rule-silence` watches the same family of silence for backticked spans. A
new finding on plain text changes `design@core@candidate-rule-and-retired-forms`.
`issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` covers the moment after the item is
gone; this entry covers the time it exists.

### What would close it

A design discussion under `skill@knowledge-architect-design` that decides at least the inside case:
whether a plain `#<id>` in a plan document that names one of that plan's own items is reported, with
its checked reference as the repair. And that decides, or records as left, the outside case: a plain
`#<id>` matching a live plan item in a file outside the plan. A test plants each decided case.
