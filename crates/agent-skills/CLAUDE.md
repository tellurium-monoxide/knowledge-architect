# Agent skills

This crate carries text, not behaviour; the core reads it to install and to check.
`path@agent-skills@content/` holds the shipped files in the install layout, and
`path@agent-skills@build.rs` generates the list that `path@agent-skills@src/lib.rs` exposes, each
entry the install path and the text, per `design@agent-skills@content-mirrors-the-install-layout`.
A file under content/ that the layout does not map fails the build.

**A change to content/ is installed in the same commit.** This repository installs its own skills,
and its check compares each installed file with the shipped text byte for byte. After editing
content/, run `cargo klarch install-agent-skills` and commit the installed copies under .claude
with the change, or the check fails.

**No check reads content/.** The manifest takes it out of the walk, and the installed copies are
out of the walk by construction, so a live reference or a path of this repository in the shipped
text passes every gate, per `issue@agent-skills@shipped-text-is-reference-free-mechanically`. The
text names no path and no convention of this repository, holds no live reference, writes an
illustration as a placeholder in angle brackets, and writes the project's command as the install
placeholder, per `design@agent-skills@shipped-text-is-reference-free`. A skill names another
installed skill by its installed name, which carries the installer's prefix.

**The primer reaches every session of every installing project**, so it holds only what every
session needs and no installed skill delivers, per `design@agent-skills@primer-limit`. A line that
restates a skill, or a convention of one project, does not go in it.

## Editing an installed skill

An edit of a skill or an agent under `path@agent-skills@content/` passes three tests, in order. This
is a restatement; its homes are the entries named.

1. **Scope.** A finding about the owner's behaviour outside the skill's expectation set is not a
   gap, and no instruction is written for it. The sets are in §5 of
   `path@agent-skills@content/skills/retrospective/SKILL.md`; a skill not listed there has none yet.
   A finding that two installed instructions leave no move satisfying both is always in scope
   (`design@agent-skills@expectation-set-bounds-scope`).
2. **Necessity.** A contradiction, a broken trigger or a factual error is repaired on reading. An
   addition needs an observation from a real session, the owner's named lack, and a one-sentence
   mechanism; a predicted behaviour is parked as an issue (`design@agent-skills@additions-need-real-use`).
3. **Kind.** A missing capability is worth text; a conformance rule only where the default is
   systematically wrong (`design@agent-skills@capability-over-conformance`).

For the design-discussion skill, a finding about an intermediate table rather than the outcome is
not worth text (`design@agent-skills@outcome-over-display`). A rewording needs no design entry
(`design@agent-skills@instruction-record-is-minimal`).

The decisions about the skills are `path@agent-skills@docs/design.md`.
