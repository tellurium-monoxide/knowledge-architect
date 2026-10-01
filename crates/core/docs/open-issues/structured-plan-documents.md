---
kind: todo
---
# Plan documents have no structure the checker enforces

## Summary

A project's plan documents, its specs and milestones, live in its plans directory, and nothing
about them is checked beyond references. A decision taken but not yet implemented exists only in
such a document, as prose, until the work lands and the decision is recorded in a design home. The
owner intends the checker to provide a structure for plan documents, with registers and citable
items of their own. The installed planning skill already fixes a shape that such a structure can
read.

## Details

### What

A structure for plan documents and their directory, enforced by the checker. The owner's sketch, as
stated when the planning skill was designed; nothing of it is decided:

- a project declares a milestone or a spec as such;
- each carries its own items, under registers of their own: design decisions taken and not yet
  harvested, a milestone's steps, acceptance criteria;
- those items are citable, under reference rules of their own. For instance, an item of a spec
  cannot be referenced from outside the plans directory.

The shape the planning skill fixes today, so that such a structure reads existing documents without
rewriting them, is `design@agent-skills@structure-ready`:

- the layout: a spec is a file in the plans directory; a milestone is a directory holding its
  milestone document as `README.md` and one spec per step, per
  `design@agent-skills@milestone-is-a-directory`;
- the section titles and their order, the same in every plan document;
- an identifier in the entry grammar on every thread, step and acceptance criterion, written in
  plain text with a hash sign. A backticked identifier would be a misplaced definition today; a
  structure would turn the plain identifiers into definitions with a mechanical edit;
- nothing outside the plans directory cites an item of a plan document. A `path` reference to a
  whole plan document is allowed.

### Why it matters

While work is open, a plan document on its branch is the only place its decisions exist. Nothing
tells another session or another branch what is decided, nothing checks that a later step
implements against the current text, and a thread's identifier is minted with a collision check by
hand, per `design@agent-skills@thread-slug-is-entry-id`, which holds only until this structure
exists. It strains `design@core@registers-are-declared` in that no declared register can hold a
decision before it is built. The fixed shape also has a cost while no structure reads it: the
planning skill is more prescriptive about headings than a free-form spec would be.

### What would close it

A structure for plan documents that the checker enforces, with its registers declared and its
reference rules checked, reading plan documents written in the shape above. Or the owner's ruling
that plan documents stay free prose, with `design@agent-skills@structure-ready` then reopened.
