---
kind: question
---
# The quote scope is the innermost heading section at any level, where the design says the level-three subsection

## Summary

`design@knowledge@scope-and-distance` says a claim's scope in a document is the level-three
subsection, and `design@knowledge@a-slug-is-a-heading` restates it as the innermost level-three
subsection where one exists. The code takes the innermost heading section at any level. Which
of the two is right is undecided.

## Details

### What

Established from the source, not from a run. `analyse` in
`path@knowledge@documentation/src/source/md.rs` collects a heading of every level, `ScopeKind::Section`
in `path@knowledge@documentation/src/source/mod.rs` carries the level as data, and `scope_at` picks
the deepest section holding a line. So a claim under a level-four heading is judged inside that
level-four section, and a quote in the level-three body above it does not discharge it; a
document with level-two headings only judges each claim against its level-two section. Root
`path@thaum@CLAUDE.md` restates the design's wording, the level-three subsection.

### Why it matters

Intent and code disagree: the entry strains `design@knowledge@scope-and-distance`, whose
statement the code does not implement. A reader placing a quote by either rule can be reported
by the other: a quote put in a level-three body above a level-four claim, legal under the design, is a
finding under the code. `assumption`: no live document in this tree carries a claim under a
level-four heading whose quote sits in the level-three body above, since the check passes.

### What would close it

A decision. Either the two heads are rewritten to say the innermost heading section at any
level, with root `path@thaum@CLAUDE.md` following, or `scope_at` folds level four and deeper into
the enclosing level-three section. Either way, a test in
`path@knowledge@documentation/src/check/regime.rs` naming the level a claim under a level-four
heading is judged against.
