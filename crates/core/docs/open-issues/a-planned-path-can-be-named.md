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

The design is approved, on the owner's word, in the discussion recorded by the commit whose subject
is "A pointer is written in a form the checker judges, and the repairs from thaum's retrospective
of step 4f". The owner ruled that it gets no spec. Its shape:

- A kind `planned`, as `planned@<anchor>@<path>`. The anchor is resolved and checked as a `path`
  reference's: the deepest anchor wins, a trailing `/` claims a directory, and no `..`, `./` or
  leading `/`. A target the ignore rules cover is not asserted, as for `path`.
- The target must not exist. Once it does, the finding names the `path` form as the repair, such
  as "`<path>` now exists under `<anchor>`: write `path@<anchor>@<path>`", and says nothing about the
  plan's state. A test pins that repair: a file created by unrelated work then asks only for the
  conversion, and the session's own report judges the plan.
- It is legal in the plans directory only. Elsewhere it is a finding: "a planned path is cited
  from a plan document only; name the plan that creates it".
- Lost: documenting `path@elsewhere@<path>` as the planned form, since it drops the anchor, so the
  conversion is not mechanical and the deepest-anchor rule goes unchecked; and a scope of every
  document, since an issue states what is missing rather than the shape of its fix, and `elsewhere`
  serves a hypothetical file there.
- The owner ruled a tripwire to record with the decision's head, guarding its scope. **Fires
  when:** `check` reports a `path@elsewhere@<path>` reference outside the plans directory whose target
  now resolves, in a sentence that said the file was to be created. **Response:** reopen the scope.
  **Re-entry:** the commit that creates the file.
- It goes through the development skill of this repository: tests shown to discriminate, an
  adversarial review, and a CHANGELOG entry of surface `checks`, class minor.

What closes the entry is that implementation, against `design@core@plan-register`, with a test that a planned path outside
the plans directory is refused and one that a planned path whose target exists is reported. Every
site that cites this entry beside a plain-text path, per
`design@knowledge-architect@a-needed-unchecked-pointer-names-its-gap`, is converted to the new form
in the change that closes it.
