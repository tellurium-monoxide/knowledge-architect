---
kind: deferred
---
# The required documents assume one agent provider's file name

## Summary

Every component must carry a CLAUDE.md, the file name one agent harness reads. Projects often keep
their agent instructions in a generic AGENTS.md, with CLAUDE.md, GEMINI.md and others as symlinks
to it or as files importing it. The checker has no notion of either.

## Details

### What

A way for a project to declare which agent providers it serves, each requiring its own file, with
AGENTS.md as the generic one. The shape is open.

### Why it matters

It strains `design@core@components-carry-the-same-documents`, which compiles CLAUDE.md in as a
required document. A project that works with several providers keeps copies in sync by hand, or
cannot adopt the checker's required set, which threatens
`goal@knowledge-architect@any-project-can-adopt-it`.

### Trigger

The owner schedules it, or a project using the checker needs a second provider: that project's
setup is already deciding which file holds its agent instructions.
