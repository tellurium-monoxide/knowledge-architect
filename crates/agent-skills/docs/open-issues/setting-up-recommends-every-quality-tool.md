---
kind: todo
---
# The setting-up skill does not yet lead a project to every quality tool the goal names

## Summary

The project's goal is that a project adopting the workflow is led to set up the quality-assurance
tools proven useful alongside it. The setting-up skill recommends one gates command and proposes
goals for a maintenance tool, and stops there.

## Details

### What

The setting-up skill recommends one command that runs every check owed before a merge, with the
commit messages among them, and gives the shape a Rust project uses for it. It does not recommend
continuous integration running that command, and it does not say which checks of the project's
language belong in it, nor their recommended shapes.

### Why it matters

It is the work that fulfils `goal@knowledge-architect@setup-brings-quality-tools`, which a project
following the setting-up skill does not reach today: it designs its continuous integration and its
language's checks itself.

### What would close it

The setting-up skill recommending continuous integration that runs the gates command, and the
checks of at least the languages it names, each in a recommended shape, so that a project following
it reaches every tool the goal names.
