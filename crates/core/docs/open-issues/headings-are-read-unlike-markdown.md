---
kind: defect
---
# The scanner reads a slug on an indented line as a heading's, and does not see a setext heading

## Summary

Two gaps between how the scanner reads headings and how markdown does. A slug at the end of a line
indented four spaces or more defines an entry, though markdown reads the line as no heading. A
setext heading, a line underlined with `---` or `===`, is not seen at all, so the section it opens
does not exist for the checks.

## Details

### What

`HEADING` in `path@core@src/scan.rs` takes at most three spaces of indentation, as markdown does,
and `SLUG_SITE` matches the line after its leading whitespace is trimmed. Reproduced over a copy
of `path@core@tests/projects/minimal/`, whose `note` register sits at level 2 in its decisions
file, after `git init` and `git add -A`:

1. append to that file a heading `## A decision` ending with a slug `probe-entry`, a paragraph
   line, and under it the line `    ## into a continuation line` ending with a slug `lazy`, the
   second line indented four spaces;
2. cite `note@notes@lazy` from the copy's README.md and run `klarch check`.

The run passes, and `klarch show note@notes@lazy` resolves it, printing the section of
`probe-entry`. Markdown reads the indented line as a lazy continuation of the paragraph, so no
heading carries the slug.

Appending a setext heading, the line `Setext entry` with a slug `setext`, underlined with dashes,
gives the finding that the slug "is written in the middle of a line", where markdown reads a
level-2 heading. In a plan document a setext section title is not seen, so the items under it are
read under the section before it: a review of the plan-items step defined an argument as a thread
that way, in a step spec.

Ruled out: the closing `#` sequence of an ATX heading, which the scanner now strips. Not
established: whether any document of this repository holds either shape.

### Why it matters

`design@core@an-entry-is-a-heading-at-the-register-level` rests on the scanner reading headings as
markdown does: an entry is a heading. An entry markdown shows as no heading is one a reader cannot
find, and a section markdown shows that the checks do not see changes the kind of the items under
it, against `goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

`SLUG_SITE` takes the indentation `HEADING` takes, and a setext heading is either read as a heading
or reported as a shape the checker refuses, with a test for each that a slug on an indented line
defines nothing and that a setext section title opens its section, or is reported.
