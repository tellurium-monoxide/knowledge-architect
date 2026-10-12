---
kind: defect
---
# The finding on an escaped path that resolves names the first anchor that holds it, not the deepest

## Summary

A `path@elsewhere@<path>` whose target resolves in this tree is reported as resolving "beside" an
anchor. The anchor named is the first in the anchor list whose directory joined to the path is in
the listing, which is the root for any path written from the root, even when a deeper anchor holds
the target. The repair then asks the writer to anchor the reference "at the anchor that holds it"
without naming that anchor.

## Details

### What

In `fn path` of `path@core@src/check/references.rs`, the escape arm finds the anchor with
`anchors.all().iter().find(..)` over the joined paths and names it in the statement. The root's
path is empty, so a path written from the root, such as one into a component's directory, resolves
at the root first.

Reproduced by the adversarial reviewer of the register-kind work, over a scratch project with a
component `sub` at `parts/sub`: `path@elsewhere@parts/sub/docs/goals.md` was reported as resolving
"beside `proj`", the root, where the deepest anchor holding the target is `sub`. That work made a
register's home in this case name its `register` form instead; every other target keeps the
finding as described.

### Why it matters

`design@core@finding-names-the-repair` asks a finding to name the repair it needs; this one names
an anchor the deepest-anchor rule of `design@core@every-path-names-its-anchor` refuses for the
target, so a writer who follows the statement writes a reference the next run reports.

### What would close it

The statement names the deepest anchor that holds the target, as `Anchors::owning` gives it, and
the repair names the form at that anchor, with a test over a project with a nested component.
