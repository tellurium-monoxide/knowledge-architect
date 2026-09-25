---
kind: question
---
# Is the per-file release pin useful, and worth keeping?

## Summary

A document may carry an HTML comment, `<!-- cr-version: YYYYMMDD -->`, that makes every rule
quote in it verify against that release instead of the vendored one. No live document carries
one, no instruction says when to write one, and the convention it served, a design home stating
the release it was argued against, has left the instructions. Whether the mechanism earns its
place is undecided, and the working belief is that it does not.

## Details

### What

`pin` in `path@knowledge@documentation/src/scan.rs` matches the first such comment outside a fence
and returns its date; the `check` command in `path@knowledge@src/main.rs` resolves that release
for the file and verifies the file's quotes against it, and its summary prints one line per
pinned file, or `no pinned containers: every quote tracks the vendored release` when there is
none. On the current tree the summary prints the latter. A grep for `cr-version` outside the
tool's own directory hits nothing; inside it, the scanner, the summary line, the fixtures of
`path@knowledge@tests/mock_projects.rs` and one sentence of
`issue@knowledge@release-cache-at-a-shared-path`. No head in `path@knowledge@docs/design.md`
records the mechanism as a decision.

### Why it matters

A mechanism nothing uses is still read, tested and kept consistent at every change to the
scanner and to release resolution: a pinned file is one more branch in how a release is chosen,
and `issue@knowledge@release-cache-at-a-shared-path` already reasons about it. Its one argument,
that a document argued against an older release keeps verifying after a bump, is the argument
`bumping-rules` refuses: a quote that stops verifying is the work list, and a pin would keep it
off the list.

### What would close it

A decision. If the pin is not kept: remove `pin`, the per-file release resolution and the
summary line, delete the two fixtures that exercise it, and rewrite the sentence in
`issue@knowledge@release-cache-at-a-shared-path` that depends on it. If it is kept: name the use,
record it as a head in `path@knowledge@docs/design.md`, and say in `bumping-rules` when a pin is
written and when it is removed.
