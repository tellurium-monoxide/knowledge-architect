---
kind: todo
---
# A plan document has no way to name a file its own work will create

## Summary

A plan document names files that do not exist yet: the homes, documents and modules its work will
create. Today it can write such a path only as plain text, which nothing checks, or as
`path@elsewhere@<path>`, which the checker refuses once the target exists, so the citation breaks at
the step that builds it. The owner wants a checked form for a planned path, usable only from the
plans directory.

## Details

### What

In the milestone document of the structured-plans milestone, about 30
backticked paths that did not exist yet were each reported as a path to anchor. The repair the finding suggests, `path@elsewhere@<path>`, names
"a deleted or hypothetical file" and is refused when its target resolves, which the plan's own work
makes true. The author wrote the paths as plain text. A form is missing that says "this path is
planned": accepted while the target is absent, and reported once the target exists, so that the plan
document is revisited when its work creates the file. It would be legal inside the plans directory
only, since a plan document is the one place that describes unbuilt work.

### Why it matters

Plain text is unchecked, and a habit of writing paths as plain text to avoid a finding is a habit
of avoiding the checker, against `goal@knowledge-architect@documentation-stays-consistent`. The gap
sits beside `design@core@reserved-anchors`, whose `elsewhere` anchor covers a path that will never
resolve here, and `design@core@every-path-names-its-anchor`, which every planned path still has to
satisfy once it exists.

### What would close it

A design for the planned-path form, under `knowledge-architect-design`: its spelling, the anchor it
names, the finding when the target appears, and how it relates to the structure of plan
documents, `design@core@plan-register`. Then its implementation, with a test that a planned path outside
the plans directory is refused and one that a planned path whose target exists is reported. Every
site that cites this entry beside a plain-text path, per
`design@knowledge-architect@a-needed-unchecked-pointer-names-its-gap`, is converted to the new form
in the change that closes it.
