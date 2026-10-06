---
kind: deferred
---
# A project cannot reference an entry of another project

## Summary

A reference names an entry of the project that holds it, and nothing else. A project that uses
this checker, or builds an extension on it, cannot point at one of the checker's own decisions:
the reference grammar has no form for another project's entry. Such a pointer is written as prose,
beside a reference to an entry of the writing project that records this gap, per
`design@knowledge-architect@a-needed-unchecked-pointer-names-its-gap`, and nothing checks the
pointer itself.

## Details

### What

A form for referencing an entry of another project, resolved against something the checker can
read: for instance the documents a published crate ships, or a pinned copy of the other project's
design homes. Nothing about the shape is decided.

### Why it matters

It would extend `design@core@a-slug-belongs-to-a-component`, under which every reference names an
anchor of its own project. The owner doubts there is a real need: a design decision of a project
that relies on the checker states that it relies on the checker working as intended, and needs no
pointer into the checker's design, per `design@agent-skills@relying-on-the-checker`. thaum meets the question first: when it moves onto the
published checker, its documents hold over a hundred references into the checker's decisions
(113 lines in 35 files outside the checker, counted on thaum's main at e98e296), and its
migration rewrites them as prose or drops them.

### Trigger

The owner takes it up, or a migration of a project onto a new version of the checker meets a
reference it cannot express in prose without losing what the pointer carried.
