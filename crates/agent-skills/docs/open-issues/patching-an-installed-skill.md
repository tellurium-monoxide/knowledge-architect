---
kind: todo
---
# A project cannot change one instruction of an installed skill

## Summary

An installed skill is compared byte for byte with the version the project pins, so a project
cannot edit it. A project adds to it through a skill of its own, and the installed
agent-configuration skill says such an addition "never contradicts" the installed text. A project
whose need is to change or remove one instruction of an installed skill has no sanctioned means.
This repository already sets three installed instructions aside in its root CLAUDE.md, so the need
is met here, not only in an adopting project.

## Details

### What

The owner's leads, from the owner list of the first run of the agentic-workflow axis (F26): a
project is allowed to supersede installed text, but this is not recommended. A supersession is
documented in the root CLAUDE.md, either by holding the superseding instruction or by a table that
maps every superseding instruction, and each says what is superseded, where the superseding
instruction is, and why it is needed, concisely. For instance, a project's own `<project>-review`
skill could hold instructions that supersede some of `skill@knowledge-architect-review`. The
owner: "This practice should be allowed, but not recommended."

The three supersessions this repository holds today, each ruled by the owner:

- the project skill prefix `klarch-`, against the installed rule that the prefix is the project's
  name and a hyphen, per `design@knowledge-architect@klarch-prefix`;
- a retrospective's workflow file handled here rather than filed as an issue on GitHub, per
  `design@knowledge-architect@retrospective-findings-stay-here`;
- the agentic-workflow axis's findings on the installed text edited at their source, with no
  upstream file, in `instructions@repository-skills`.

An earlier lead, kept: a patch stored as a diff in the project, applied by the install and checked
by the check, so that an upgrade either applies it again or reports that it no longer applies.

### Why it matters

`design@agent-skills@overlay-by-separate-skills` lets a project add, never contradict. A project
whose need is a contradiction is left two moves: report it to this repository, or declare
`harness = []` and lose the installed workflow, the checks of `design@core@owned-namespace-check`
and the overlay rule with it. That threatens `goal@knowledge-architect@any-project-can-adopt-it`.
This repository's own three supersessions stand against the agent-configuration skill's "never
contradicts it" as written, which the first run of the agentic-workflow axis reported. The
retrospective asks about the need at every session, per
`design@agent-skills@premortem-as-watch-points`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that decides the shape from the
owner's leads, and its work landed: the agent-configuration skill and
`design@agent-skills@overlay-by-separate-skills` say how a project supersedes installed text and
where it documents each supersession, and this repository's root CLAUDE.md documents its three in
that shape.
