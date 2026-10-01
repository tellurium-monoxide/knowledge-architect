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

The decisions about the skills are `path@agent-skills@docs/design.md`.
