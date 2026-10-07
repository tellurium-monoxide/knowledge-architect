---
kind: todo
---
# Nothing checks mechanically that the shipped text cites no entry of this repository

## Summary

The text this crate ships, content/ and the snippets the build inlines, is installed in other
projects, so it must cite no entry of this repository. Both are in the walk, so every reference in
them is checked against this repository: a reference to an entry that resolves here passes the
check, and would dangle in every installing project. Only the reviews and the release's hand check
judge it.

## Details

### What

A mechanical check that the shipped text, as it ships, holds no reference to an entry. A path every
conforming project holds, `path@*@<path>` or `path@plans@<path>`, and a placeholder in angle
brackets stay allowed, per `design@agent-skills@shipped-text-cites-no-entry`. The text as it ships
is the installed copies under .claude: their `%%` comments are removed and their placeholders
filled, so the comments, which cite entries on purpose, are not judged by it.

The owner's direction, when content/ was taken back into the walk: open an issue "that when this
"reference clean path" feature rolls out, it gets applied to the self installed workflow (in
.claude, only on installed content, where the comments are already trimmed and thus no reference
should be left)". So one shape is a declaration, in the manifest, of paths that must hold no
reference to an entry, applied here to the installed copies. Its shape is not decided.

### Why it matters

`design@agent-skills@shipped-text-cites-no-entry` rests on the reviews and the release's hand check.
A reference to an entry that they miss ships, and in every installing project it dangles where the
project cannot repair it, against `goal@knowledge-architect@any-project-can-adopt-it`, because the
file is the installer's and the check compares its bytes.

The same mechanism is needed by the crates.io pages, each crate's CRATES-IO.md, per
`design@knowledge-architect@crates-io-page-file`: the walk reads them, and a reference that resolves
passes the check while crates.io renders it as dead code. Today a convention and review keep them
free of references.

### What would close it

The check, run by `cargo x gates`, failing on a reference to an entry planted in a scratch copy of a
file the installed copies hold, and passing on a `%%` comment that cites one in content/. The
release procedure, `path@agent-config@skills/klarch-release/SKILL.md`, checks the shipped text for
references by hand; the check replaces that step.
