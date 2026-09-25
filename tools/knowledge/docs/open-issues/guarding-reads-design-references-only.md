---
kind: todo
---
# The tripwire listing's `guarding` column reads references of the `design` kind only

## Summary

`cargo knowledge tripwires` prints, per entry, every `design@<anchor>@<id>` reference the entry
carries, and `--guarding <ref>` filters on the same set. A tripwire guarding a reading of the
rules names an `interpretation@rules@<id>` entry instead, so every entry of the `rules` location
prints `-` in that column and none of them is reachable through `--guarding`.

## Details

### What

`guards` in `path@knowledge@documentation/src/records.rs` keeps a span when its kind is
`design` and drops every other kind. Observed on the current tree: `cargo knowledge tripwires
rules` prints `-` under `guarding` for every row, and
`cargo knowledge tripwires --guarding interpretation@rules@visibility-rules-only-grant` prints
`(no entry)` and exits 1, while `path@rules@tripwires.md` holds a heading opening with
`` Guarding `interpretation@rules@visibility-rules-only-grant` ``. The tool matches its own
README, which documents the column as the `design` references, so this is a gap rather than a
divergence: the rules location's tripwires were written to the same shape as every other
anchor's and the listing does not read them.

### Why it matters

The listing is the standing-state reviewer's instrument for which decision each tripwire guards,
and `tracking-open-issues` sends a session to `--guarding` to find the tripwires a reversed
decision dangles. For a reading of the rules both say nothing, and the only route is
`cargo knowledge show interpretation@rules@<id>`, which prints the inbound references of one
entry rather than a listing.

### What would close it

`guards` keeping every reference whose kind is a heading or file register's name rather than
`design` alone, or `design` and `interpretation`, with the column's name and the README's
sentence following; a test in `path@knowledge@documentation/src/records.rs` over a tripwire
naming an interpretation entry, asserting the row and the `--guarding` match; and the two
sentences in `tracking-open-issues` and the standing-state reviewer that describe the column
as `design` only rewritten.
