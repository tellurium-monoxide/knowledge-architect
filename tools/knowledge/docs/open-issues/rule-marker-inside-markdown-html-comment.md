---
kind: question
---
# A rule marker inside a markdown HTML comment is read by nothing, and no instruction says so

## Summary

The markdown grammar marks every line inside an HTML comment as inert, and the scanner takes no
observation from an inert line, in `path@knowledge@documentation/src/source/md.rs` and the
inert branch of `scan` in `path@knowledge@documentation/src/scan.rs`. A `CR:` marker with a
quote, a slug reference or a path reference written inside `<!-- … -->` in a markdown document
is therefore verified by nothing and reported by nothing.

## Details

### What

The markdown grammar marks every line inside an HTML comment as inert, and the scanner
takes no observation from an inert line, in `path@knowledge@documentation/src/source/md.rs` and the
inert branch of `scan` in `path@knowledge@documentation/src/scan.rs`. A `CR:` marker with a quote, a
slug reference or a path reference written inside `<!-- … -->` in a markdown document is
therefore verified by nothing and reported by nothing. Root `path@thaum@CLAUDE.md` enumerates the
places a rule number is data and says there is no third form, and names no such place.

### Why it matters

It is a place to hide a fabricated claim, of the kind the rules-reviewer is
told to read for, and the instruction that enumerates the data forms is incomplete against the
implementation. Parking a section by commenting it out is ordinary, so the inert reading itself
is wanted; what is undecided is whether a marker may sit in one at all.

### What would close it

A decision, recorded in this component's design home: either an inert
line may not name a rule, reported the way the inverse assertion reports a rule number outside
the walk, or the HTML comment is named in root `path@thaum@CLAUDE.md` as a form that carries no
claim. Re-entry: the next tool-cleanup discussion.

### Observed

By the rules axis of the review of the branch that landed
`design@knowledge@checker-source-literals-are-data`, on a scratch copy of the `minimal` mock project
under `path@rules-corpus@tests/projects/`: a line naming a mock rule behind a marker, with a claim and no
quote, appended to its `README.md` inside an HTML comment added no finding, and the same line
outside one added the expected claim-without-quote finding. Reproduces.
