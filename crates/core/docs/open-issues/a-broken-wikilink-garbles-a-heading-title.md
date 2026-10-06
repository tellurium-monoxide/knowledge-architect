---
kind: observation
---
# A heading holding a wikilink the parser gives up on gets a garbled scope name

## Summary

The markdown parser runs with every extension enabled, wikilinks among them. On a heading such as
`# A [[x|]] `a` ]` it hands over text events out of order and a code span twice, so the scope the
heading opens is named `A ]] a ]  ]` instead of `A [[x|]] a ]`.

## Details

### What

`md::analyse` in `path@core@src/source/md.rs` runs `pulldown_cmark::Parser::new_ext` with
`Options::all()`, which turns on `ENABLE_WIKILINKS`. On version 0.13.4, the version in Cargo.lock,
a throwaway unit test printed, for each input, the scope names of `parse(t).scopes` or the code
spans of `parse(t).prose[0].code`:

```
"# A [[x|]] `a` ]\n"  scope names ["A ]] a ]  ]"]
"[[x|]] `a` ]\n"      code spans  [(7, 10), (7, 10)]
```
 `analyse` now sorts and
dedups the code spans, so the doubled span no longer reaches any lookup. The heading's title is
still built from the out-of-order events.

The scanner reads headings with its own pattern, `HEADING` in `path@core@src/scan.rs`, not from the
scopes, so no check of the core reads the garbled name. `document::Scope` is public, so an
extension that reads a scope's name does. Whether any project writes `[[` in a heading is not
established.

### Why it matters

An extension that matches a section by its name misses this one. No decision strains: no head
states which markdown extensions the parser enables.

### What would close it

Either the parser options name the extensions the checker reads, without `ENABLE_WIKILINKS`, and a
test shows the heading's scope named as written; or the quirk is reported upstream and the entry
waits on a parser version that fixes it.
