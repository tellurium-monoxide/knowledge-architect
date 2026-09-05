---
kind: observation
---
# Span and link shapes the scanner cannot see

## Summary

The reference tokenizer, the unanchored-path lint and the link scanner each read one shape, so
several pointer-shaped spans produce neither a reference nor a lint observation and no finding.
The list of shapes is below.

## Details

### What

The reference tokenizer records a backticked span holding an `@` and no whitespace,
backtick or angle bracket; the unanchored-path lint reads one character class with no `@`; and
the link scanner reads one inline pattern. Each shape below therefore produces neither a
reference nor a lint observation, and no finding:

```
`notes/a.md:12`             a colon suffix, the file-and-line idiom
a span wrapped across a line boundary
a fullwidth at sign in place of the ASCII one
[text][label]               a reference-style link, with its definition elsewhere
[text](<notes/a.md>)        an angle-bracketed target
```

Trailing punctuation inside the backticks, `` `path@<anchor>@notes/a.md,` ``, is not on the
list: the span is recorded as written and resolves to nothing, so it is reported as dangling.
The link shapes surface as a dangling target or an unlinked subdocument in
a design README, and are silent in an ordinary navigation file, per the close-enough clause of
`design@knowledge@links-are-navigation-rows`. Enumerated by an adversarial review of the
anchored-grammar branch; a grep at that revision found no live instance of any shape, so every
gap is latent.

### Why it matters

The unanchored-shape lint promises that no pointer class passes
unregistered, and each shape above is a pointer a reader might write — the colon idiom most of
all. The cost of widening is false positives on prose, which the lint's two-segment clause was
tuned against; the census that tuned it did not measure these shapes.

### What would close it

Widening the classes shape by shape with a measured false-positive
census for each, or recording beside `design@knowledge@every-path-names-its-anchor` that a named shape
stays outside on purpose. `assumption`: the colon idiom is the one worth widening first, being
ordinary editor output. Untested — no census taken.
