---
kind: todo
---
# Plan documents have no structure the checker enforces

## Summary

A project's plan documents live under its docs/plans/ directory, and nothing about them is checked
beyond references. A decision taken but not yet implemented exists only in such a document, as
prose, until the work lands and the decision is recorded in a design home.

## Details

### What

A structure for plan documents and their directory, enforced by the checker, with registers of
their own: for instance planned design items, decided and not implemented. The shape is fully
open.

### Why it matters

While work is open, a plan on its branch is the only place its decisions exist. Nothing tells
another session or another branch what is decided, and nothing checks that a later step
implements against the current text. It strains `design@core@registers-are-declared` only in that
no declared register can hold a decision before it is built.

### What would close it

A structure for plan documents that the checker enforces, with its registers declared.
