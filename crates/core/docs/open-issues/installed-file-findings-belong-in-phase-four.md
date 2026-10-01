---
kind: todo
---
# The installed-file findings sit in phase 2, where the core's placement rule puts them in phase 4

## Summary

The findings of `check::agents` (an installed file missing, differing or unshipped, a primer not
imported) are raised in phase 2. The core's own placement rule puts a check before the last phase
only when another check reads its findings, or when it reports what the model could not read.
Neither holds: the installed files are outside the model, and no check reads these findings.

## Details

### What

Move the installed-file check to the last phase, as a check family of its own beside `generated`,
`registers` and `references`, and say so in `design@core@phases-gate-the-report` and
`design@core@owned-namespace-check`.

### Why it matters

In phase 2 the findings stop the run before any reference finding is listed, and every command
that writes a generated file refuses while one stands: after a version renames a skill, `index`
refuses until the install runs. The owner judged the move correct and not worth doing now.

### What would close it

The check runs in the last phase, `index` is no longer refused by a stale install, and the two
design heads say so.
