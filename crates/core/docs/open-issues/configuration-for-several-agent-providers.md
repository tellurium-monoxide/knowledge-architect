---
kind: deferred
---
# The agent configuration serves one provider's harness

## Summary

`[agents] harness` declares the agent harnesses a project serves, and the checker knows one,
`claude`: it requires a CLAUDE.md in every component and installs into the `.claude/` layout.
Projects often keep their agent instructions in a generic AGENTS.md, with CLAUDE.md, GEMINI.md and
others as symlinks to it or as files importing it. The checker has no harness for either.

## Details

### What

Harness values for other providers, each requiring its own file and installing into its own
layout, and AGENTS.md as the generic file. The mechanism exists; what each value requires is open.

### Why it matters

It extends `design@core@agents-table`, whose list holds one value. A project that works with
several providers keeps copies in sync by hand, or serves one provider only, which threatens
`goal@knowledge-architect@any-project-can-adopt-it`.

### Trigger

The owner schedules it, or a project using the checker needs a second provider: that project's
setup is already deciding which file holds its agent instructions.
