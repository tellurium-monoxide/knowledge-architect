# Agent skills

This crate carries text, not behaviour; the core reads it to install and to check. `path@agent-skills@src/lib.rs` exposes the files the checker installs, each
as its install path and its text. The set is empty until the skills are written.

The text this crate ships is read in other projects, so it names no path and no convention of this
repository, and holds no live reference: an illustration is a placeholder in angle brackets. The
decisions about the skills are `path@agent-skills@docs/design.md`.
