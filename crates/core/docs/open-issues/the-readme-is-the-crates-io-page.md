---
kind: todo
---
# The core's README is also the crates.io page, and it is written for a reader of this repository

## Summary

Cargo picks crates/core/README.md as the package's README, so crates.io renders it as the page of
the crate `knowledge-architect`. The file is written for a reader of this repository: it holds 23
references such as `design@core@exit-code-ladder`, and on crates.io each renders as literal code
that leads nowhere. The count is re-taken with:

```sh
grep -o '`[a-z*-]*@[^`]*`' crates/core/README.md | wc -l
```

## Details

### What

Decide what the crates.io page says to a user who installs the checker, and where the
repository-facing text of the README goes. One shape: the README opens with what the checker is,
how to install and run it, and links to docs.rs and to the repository, and its references move
to the repository-facing text or become links.

### Why it matters

`goal@knowledge-architect@any-project-can-adopt-it` makes the crates.io page the first text a
new project reads. The knowledge table routes "how a user can use a Component" to the README,
and for a published crate that is this page.

### What would close it

A README whose every pointer resolves for a reader on crates.io, and the repository-facing text
in a home the knowledge table names.
