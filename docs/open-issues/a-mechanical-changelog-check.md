---
kind: todo
---
# CHANGELOG.md's shape is checked by a reviewer only, and no model of it exists

## Summary

The owner intends a mechanical checker for CHANGELOG.md, with a model of its sections and entries.
Today the shape that `design@knowledge-architect@changelog-entries` fixes is checked only by
`klarch-changelog-reviewer`, once per release, by reading.

## Details

### What

CHANGELOG.md has a structure a parser can read: one section per released version and a
`Next release` section; inside each, the subsections Migration, New features and Workflow; each
entry of the shape ``- `<surface>`, <class>: <what>``, with a surface from a closed list, a class
from patch, minor and major, and entries sorted by surface. Nothing parses it. A mechanical check
could assert, on every commit rather than at the release:

- the section, subsection and entry shapes, the closed lists of surfaces and classes, and the order
  of entries;
- that a released section's content never changes once the version is tagged, while its structure
  may;
- that the version a release names follows the highest class among its entries, a patch at least.

What it cannot assert is whether a change owes an entry, or which class it has: those stay with
the reviewer.

### Why it matters

A shape defect, a misspelt surface, a class outside the list or an entry out of order, is found
today only at the release, by a subagent reading the file, and a released section's content can
change with nothing reporting it. It strains `design@knowledge-architect@changelog-entries`, whose
rules on released sections and on the order of entries have no mechanical guard, against
`goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

A design discussion with the owner deciding where the check lives: a check of the core, which any
project could use with a changelog shape it declares, or a gate of this repository in its
maintenance tool. Then the check, with a test for each assertion above that fails on a planted
defect, and the reviewer's predicates narrowed to what the check cannot assert.
