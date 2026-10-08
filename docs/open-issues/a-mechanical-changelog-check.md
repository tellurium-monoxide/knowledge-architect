---
kind: todo
---
# CHANGELOG.md's shape is checked by a reviewer only, and no model of it exists

## Summary

The owner intends a mechanical checker for CHANGELOG.md, with a model of its sections and entries.
Today the shape that `design@knowledge-architect@changelog-entries` and the preamble of
`path@knowledge-architect@CHANGELOG.md` fix is checked only by
`agent@klarch-changelog-reviewer`, once per release, by reading.

## Details

### What

CHANGELOG.md has a structure a parser can read: one section per released version and a
`Next release` section; inside each, the subsections Migration, New features and Workflow; each
entry of the shape ``- `<surface>`, <class>: <what>``, with a surface from a closed list, a class
from patch, minor and major, and entries sorted by surface as the preamble states. Nothing parses
it. The first released section, `0.1.0`, predates this shape, and is the only one that does: it has no subsections and no classes,
and adding a class to a released entry would change its content, which the decision forbids. A mechanical check
could assert, on every commit rather than at the release:

- the section, subsection and entry shapes, the closed lists of surfaces and classes, and the order
  of entries;
- that a released section's content never changes once the version is tagged, while its structure
  may;
- that the version a release names follows the highest class among its entries, a patch at least.

What it cannot assert is whether a change owes an entry, or which class it has: those stay with
the reviewer.

An instance, from the branch that fixed four issues before the 0.3.0 release: a `checks` entry was
appended after a `manifest` entry in the Migration subsection. Every gate passed it; two reviewers
of the branch found it, before the release. Findings P2 and C2 of the retrospective of
2026-10-04.

### Why it matters

A shape defect, a misspelt surface, a class outside the list or an entry out of order, is found
today only by a subagent reading the file, at the release or when a branch's reviewers happen to, and a released section's content can
change with nothing reporting it. It strains `design@knowledge-architect@changelog-entries`, whose
rule on released sections has no mechanical guard, nor has the preamble's order of entries, against
`goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

A design discussion with the owner deciding where the check lives: a check of the core, which any
project could use with a changelog shape it declares, or a gate of this repository in its
maintenance tool, and what the check does with a section released before the shape existed. Then
the check, with a test for each assertion above that fails on a planted
defect, and the reviewer's predicates narrowed to what the check cannot assert.
