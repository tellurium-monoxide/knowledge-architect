---
kind: deferred
---
# The citation regime could be offered by the core over any pinned text

## Summary

thaum verifies every quote of a Comprehensive Rule against a pinned copy of the rules, and that
machinery is placed in thaum's own extension, not in the core, by
`design@knowledge@an-extension-plugs-in-through-phased-hooks`. Its mechanism is not specific to the rules:
a marker, an identifier form, a number grammar, a pinned text parsed into numbered entries, and
the scope and distance a quote must sit within. A project that cites a specification, a standard
or a law verbatim could use it. No project using the core needs it yet.

## Details

### What

The mechanism has two parts that separate cleanly:

- **Generic:** the marker that introduces a claim about an entry of a text (`CR:` in thaum), the
  identifier form a name carries (`cr_` in thaum), the grammar of an entry number, verification of
  a quote against the pinned text with elision marked, the whole-body rule for a name, and the
  scope and distance rules.
- **Specific to thaum:** the parser of `MagicCompRules.txt` and its table-of-contents offset, the
  release watch reading Wizards' page, the archive of past releases and its manifest, and the
  changelog check.

Offering the generic part from the core would mean a corpus trait the extension implements, with
the marker, the identifier prefix and the number grammar declared rather than compiled in.

### Why it matters

`goal@knowledge@documentation-half-publishes-alone` publishes the core for other projects. A
project that needs a verbatim-citation check and finds one only inside thaum's extension has to copy
it, and two copies drift.

It strains no recorded decision. `design@knowledge@an-extension-plugs-in-through-phased-hooks`
already lets an extension carry the whole regime, which is where thaum's design places it.

### Trigger

A project using the core needs to verify verbatim quotes of a pinned external text. The session
that designs that project's checks is already deciding how such quotes are verified, so the
question of taking the mechanism from the core rather than writing it again is part of its own
work.

When the core leaves this repository, this entry leaves with it, in the core's own issue register.
