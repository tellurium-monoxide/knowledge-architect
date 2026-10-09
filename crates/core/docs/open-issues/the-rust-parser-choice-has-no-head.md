---
kind: design
---
# The choice of tree-sitter-rust as the Rust parser has no head, and two rejected alternatives lost to it

## Summary

The core parses Rust source with `tree-sitter-rust`. No head of `path@core@docs/design.md` records
that choice. Two rejected alternatives that lost to it, "`syn` as the Rust parser" and
"Rust-analyzer's syntax crate", name `design@core@grammars-not-prefixes` as the decision they lost
to, and that head names no parser.

## Details

### What

**Where the choice is recorded today**, outside any design home:

- `path@core@Cargo.toml`, the dependencies `tree-sitter = "0.25"` and `tree-sitter-rust = "0.24"`.
- `tripwire@core@grammars-not-prefixes`, which fires when "a `tree-sitter-rust` or `pulldown-cmark`
  upgrade changes a node name the extractor asks for".
- The message of the commit 94f6d67, "rework how the knowledge tool parses documents and enforces
  rule citations", in its paragraph "THE PARSERS": "`tree-sitter-rust` says which byte ranges of a
  Rust file are prose; `pulldown-cmark` reads those".

**The instances**, two entries of `path@core@docs/rejected-alternatives.md`, each naming
`design@core@grammars-not-prefixes`:

- "`syn` as the Rust parser" lost because `syn` discards ordinary comments during lexing, so a
  citation in a `//` comment needs a second raw-text pass.
- "Rust-analyzer's syntax crate" keeps comments, so it meets `design@core@grammars-not-prefixes` in
  full. It lost on release churn: it is published per nightly. That head says nothing about
  release churn.

`design@core@grammars-not-prefixes` decides grammar against line prefixes: "Nothing is decided by
a line's prefix." It names no parser.

**The suspected mechanism**: 94f6d67 wrote the head `design@core@grammars-not-prefixes` and argued
the parser choice in its message only, so the alternatives that lost to the parser, recorded later,
were pointed at the nearest head. This is an assumption: no record states why no parser head was
written.

### Why it matters

- A rejected alternative must name the decision it lost to, per
  `skill@knowledge-architect-decision-recording@losing-alternatives`. These two name a head that
  does not decide what they lost on, so a session that reopens the parser question finds no head
  stating the choice, its standing argument or its nearest rival.
- The parser choice turns on external tools' behaviour (which comments a parser keeps, how a crate
  is released), which is test 3 of `primer@design-heads`. Its argument cannot be derived again for
  free, and today it is kept only in a commit message.

### What would close it

1. A head that records the choice of `tree-sitter-rust` as the Rust parser, with its standing
   argument and its nearest rival, written through `skill@knowledge-architect-design` on its
   in-change path. Its deliberation is the message of 94f6d67.
2. Then the two rejected alternatives, "`syn` as the Rust parser" and "Rust-analyzer's syntax
   crate", re-pointed to that head.

**Re-entry point**: the next change that touches the Rust parser or its dependencies, or a design
discussion of the core's reading of Rust source.
