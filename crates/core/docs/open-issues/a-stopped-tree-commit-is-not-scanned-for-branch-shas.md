---
kind: defect
---
# A commit whose tree stops before the last phase is not scanned for citations of the range by SHA

## Summary

Under `[commits] refuse-branch-shas = true`, `commits` reports a citation of a commit of the range
in each commit's message and tree. A commit whose tree stops at phase 1, 2 or 3 is skipped: its
citations are not reported, though the scan needs neither its entity table nor its verdict.

## Details

### What

In `commits` in `path@core@src/cli/history.rs`, a commit whose `Assembly::stopped` is set pushes its
summary line and reaches `continue` before `branch_sha_findings` is called. So neither its message
nor its tree's documents are scanned. The scan needs the commit's manifest, its message, its
documents and the range's SHAs, which a tree stopped at phase 2 or 3 has. A tree stopped at phase 1
has no manifest read, so its option is unknown.

Reproduced on a release build of this repository's checker, whose manifest turns the option on,
from a clean checkout at HEAD:

1. `c1` = a commit made with `git commit-tree` from HEAD's tree without
   `path@xtask@docs/rejected-alternatives.md`, a required document, parent HEAD, and a message
   whose body cites the first 7 digits of HEAD's SHA;
2. `c2` = a commit of HEAD's tree, parent `c1`, with the same citation in its body;
3. `klarch commits HEAD~1..<c2>`.

The output names `c1` as `failed: its tree stops at phase 2 with 1 finding(s); its message was
judged against nothing`, and `c2` as `failed: it cites the range by SHA 1 time(s)`, with one
citation finding, at `c2`'s line 3. No citation finding names `c1`: `FAILED: 2 findings above`,
the phase-2 finding and `c2`'s citation.

The skip predates the change that made the summary count every source of a commit's findings, which
kept it.

### Why it matters

`design@core@branch-shas-are-refused` states that citations are reported in each commit's message
and in each document of that commit's tree, so the code diverges from the head. The commit is
reported anyway, for its tree, so no failing range passes. But its citations are not listed: a
session repairs the tree, re-runs, and only then meets the citations, one run later than it could
have.

### What would close it

The scan run for a commit whose tree stopped at phase 2 or 3, before the `continue`, its citations
added to the commit's causes; a test in `path@core@tests/binary.rs` with a commit that stops at
phase 2 and cites the range, asserting the citation finding and both causes on the commit's line.
Whether a tree stopped at phase 1, whose manifest was not read, is scanned is decided in that work
and stated in the head.
