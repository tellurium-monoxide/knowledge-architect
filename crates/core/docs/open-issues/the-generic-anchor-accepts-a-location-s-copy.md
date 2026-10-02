---
kind: defect
---
# The generic anchor accepts a path that only a location carries

## Summary

`path@*@<path>` passes when a declared location holds the path and no component does. The design
says the generic form is accepted when at least one component carries it.

## Details

### What

The `*` arm of `path` in `path@core@src/check/references.rs` tries every anchor that is not one
the tool constructs, so a declared location counts as a carrier. Reproduction, over a copy of
`path@core@tests/projects/minimal/`, whose location `notes` holds notes/a.md and whose root
component holds no a.md:

1. append to the copy's README.md a line holding two references, each in single backticks:
   path@*@a.md and path@*@zz.md;
2. run `git init` and `git add -A` in the copy, then `klarch check` from inside it.

The run reports one finding, that path@*@zz.md resolves in no component. path@*@a.md passes,
though no component carries a.md.

Ruled out: the plans anchor and the milestone anchors, which the same arm skips since the change
that built the plans layout; this entry is about declared locations alone. The behaviour predates
that change. Not established: whether a project relies on it.

### Why it matters

`design@core@reserved-anchors` states the generic form is accepted "when at least one component
carries the path with the claimed kind". A pointer that names "every component's own copy" and
resolves only in a location passes while meaning something its writer did not intend. That was the
firing condition of a tripwire on the generic rule, which this case fired and which left the
tripwires home when it did: this entry is its consequence, and its response was to reopen the
generic anchor's checking rule. The code and the design disagree, and one of them is wrong.

### What would close it

A decision on which one: restrict the arm to components, with a test over the reproduction above
that reports path@*@a.md; or rewrite `design@core@reserved-anchors` to say any anchor the
project declares, with the reason.
