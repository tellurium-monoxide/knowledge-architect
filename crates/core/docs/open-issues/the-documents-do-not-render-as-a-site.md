---
kind: todo
---
# The documents do not yet render as one linked site

## Summary

The core's goal is that a project's documents render as one linked site: every reference rewritten
into a link, the result built by mdbook and hosted with the project. Nothing builds it yet.

## Details

### What

A preprocessing pass that rewrites each reference into a link to its entry, and an mdbook build of
the result, hosted with the project once it is released. Nothing about the shape is decided.

### Why it matters

It is the work that fulfils `goal@core@documents-render-as-a-linked-site`. Until it exists, a
reader follows a reference by searching for its entry, not by a link.

### What would close it

The pass and the build, run over this repository's own documents, producing a site in which every
reference is a working link.
