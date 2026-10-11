---
kind: defect
---
# The finding for a required document missing at the root Component names a repair that does not apply to the root

## Summary

When a Component carries no required document, the checker's finding offers two repairs: create
it, or stop listing the Component in the manifest's `[project]` table. The root Component is named
by `[project] name` and listed in no `components` array, so the second repair does not exist for
it, and a reader who tries it finds nothing to remove.

## Details

### What

The finding is built in `check` in `path@core@src/check/tree.rs`, in its loop over
`manifest.required_documents()` for each anchor whose `is_component` holds. Its text is the same
for every Component, the root included:

```text
<path>  the component `<name>` carries no <document>
    → create it, or stop listing the component in knowledge-architect.toml [project]; every component carries the same documents
```

The root anchor passes that loop: `anchor.is_root()` exempts it from the two `continue` branches
above the loop, and nothing tests `is_root()` in the message.

Reproduced on a scratch project, with the release binary built from this checkout at version
0.5.0. The project held, staged with `git add -A` after `git init`:

```toml
[project]
name = "demo"
checker-version = "0.5.0"
components = []

[walk]
skip-dirs = []
skip-files = []
exclude = []
```

with a root CLAUDE.md importing the primer, the installed set from `klarch install-agent-skills`,
a one-line heading file at each of `path@elsewhere@demo/docs/design.md`,
`path@elsewhere@demo/docs/goals.md`, `path@elsewhere@demo/docs/tripwires.md`,
`path@elsewhere@demo/docs/rejected-alternatives.md`, `path@elsewhere@demo/docs/open-issues/README.md`
and `path@elsewhere@demo/docs/plans/README.md`, the prefix `demo/` naming the scratch project's
root, and no root README.md. `klarch check` printed, among its 3 findings:

```text
README.md  the component `demo` carries no README.md
    → create it, or stop listing the component in knowledge-architect.toml [project]; every component carries the same documents
```

The other two findings were the missing specs and milestones directories of its plans directory,
unrelated. The same text is built for every required document, so a root missing its CLAUDE.md or
its rejected alternatives gets it too: read from the code, not run.

### Why it matters

`design@core@finding-names-the-repair` makes a finding's action name the repair, and
`design@core@components-carry-the-same-documents` makes the root a Component that owes the same
documents. For the root, half of the repair this finding names is impossible: an agent
that follows it looks for a list entry that does not exist, or edits `components`, which never
held the root.

### What would close it

The finding for the root Component names only the repair that applies, creating the document, with
a test over a project whose root lacks a required document asserting the repair line.
