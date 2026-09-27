---
kind: deferred
---
# Nothing tests that `rules diff` receives its two dates the right way round

## Summary

`design@thaum@named-values-where-order-decides` exists because a reversed `rules diff` reports
newly added rules as gone and points every renumbering backwards, and `bumping-rules` reads
those verdicts as instructions. The direction is asserted at the parse layer, and
`rules::diff::diff` has its own unit tests, but the wiring between them — the dispatch in
`path@rules-corpus@src/corpus_cmd.rs` that hands `old` and `new` to `diff` — has no test.

## Details

### What

`design@thaum@named-values-where-order-decides` exists because a reversed `rules diff` reports
newly added rules as gone and points every renumbering backwards, and `bumping-rules` reads those
verdicts as instructions. The direction is asserted at the parse layer, and `rules::diff::diff`
has its own unit tests, but the wiring between them — the dispatch in `path@rules-corpus@src/corpus_cmd.rs`
that hands `old` and `new` to `diff` — has no test. Found by an adversarial review of the clap
migration.

### Why it matters

It is the one seam the head's whole argument runs through, and a swap there is
silent by construction: the output names no release, only rule numbers and counts.

### Trigger

The test cannot be written against this tree today. It
needs two releases whose cited rules differ, and the two the tree holds — the pin and its one
archived predecessor — are a typographic re-export of each other: `rules diff` over them reports
`0 edited, 0 renumbered, 0 gone` in either direction, so no assertion over them discriminates. The
trigger is **the next rules bump**, which produces exactly such a pair and archives both. Whoever
runs that bump is already reading this diff's verdicts as their work list, so writing the test
that pins its direction is inside the work they are doing.

### Reproduce

Swap the two arguments at that dispatch site so it passes `new, old`.
`cargo test -p knowledge` stays green.
