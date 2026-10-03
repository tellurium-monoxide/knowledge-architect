---
kind: question
---
# The bump table classes a stricter check that requires a content change both minor and major

## Summary

The table of `design@knowledge-architect@versioning-policy` puts "a check becomes stricter, or a
check is added" under minor, and "a change that may require a layout or content change in a
project" under major. A stricter check that reports something in a conformant project requires a
content change, so it fits both rows. The 0.2.0 section of CHANGELOG.md applies both readings.

## Details

### What

Two rows of the bump table overlap:

| bump | the row's text |
| --- | --- |
| minor | a check becomes stricter, or a check is added |
| major | a change that may require a layout or content change in a project |

The 0.2.0 section of CHANGELOG.md, under Migration, holds entries of the same kind with both
classes, as the changelog review of the 0.2.0 release found:

- the entry for `path@*@<path>`: a reference that only a declared location carries is now
  reported, and the project must anchor it at the location. It is classed minor.
- the entries for a `path` citation of a plan document, for the sections a plan document owes, and
  for an item of a plan cited from outside it: each makes a check stricter or adds one, and each
  requires a content change. Each is classed major.

The version is unaffected: under 0.x, minor and major both bump 0.MINOR, per the decision's first
point.

### Why it matters

A writer of a changelog entry cannot tell from `design@knowledge-architect@versioning-policy`
which class a stricter check takes, and the reviewer of a release cannot judge the class either.
A consumer reads the class to know whether moving to the version may require work, which is what
`goal@knowledge-architect@any-project-can-adopt-it` relies on. The difference shows once the
project leaves 0.x, where minor and major are different numbers.

### What would close it

The owner's ruling on one reading, written into the table of the decision. Two readings stand:

- **major wins**: any change that may require a content change is major, and minor keeps a
  stricter check only where no conformant project can meet it, such as a check that reports what
  an earlier check already refused.
- **minor wins**: a stricter check is minor even when it requires a content change, and the major
  row covers changes to the required layout and to what the manifest accepts.

A released section's content never changes, per `design@knowledge-architect@changelog-entries`, so
the ruling applies from the next release.
