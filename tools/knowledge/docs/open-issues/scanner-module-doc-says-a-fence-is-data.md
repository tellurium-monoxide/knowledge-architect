---
kind: todo
---
# The scanner's module doc says a rule number in a fence is data, and the code reads a fence as live

## Summary

The module doc at the top of `path@knowledge@documentation/src/scan.rs` states that a rule number
inside a code span or a fence is data, not a citation. The code, `design@knowledge@grammars-not-prefixes`
and root `path@thaum@CLAUDE.md` all say a fenced block cites its rules for real, and only an
inline code span is data.

## Details

### What

The second bold paragraph of the module doc of `path@knowledge@documentation/src/scan.rs` reads
`A rule number inside a code span or a fence is data, not a citation.` The scanner's own comment
beside the fence handling in the same file says a fence suppresses slug definitions and markdown
links and not references or citations, and `is_code` in `path@knowledge@documentation/src/source/mod.rs`
covers inline code ranges alone. `cargo knowledge check` verifies the fenced quotes in root
`path@thaum@CLAUDE.md` today.

### Why it matters

A doc comment is a claim about the code as it stands, read before the code by whoever changes
the scanner, and this one states the losing alternative of
`design@knowledge@grammars-not-prefixes` as the current behaviour.

### What would close it

Rewriting the paragraph to say that an inline code span is data and a fence is live.
