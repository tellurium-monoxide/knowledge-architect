---
kind: todo
---
# The installed-file findings sit in phase 2, where the core's placement rule puts them in phase 4

## Summary

The findings of `check::agents` (an installed file missing, differing or unshipped, a primer not
imported) are raised in phase 2. The core's own placement rule puts a check before the last phase
only when another check reads its findings, or when it reports what the model could not read.
Neither holds: the installed files are outside the walk, and no check reads these findings. The
model holds them for definitions only, per `design@core@installed-entities-from-the-tree`, and
`check::agents` reads their bytes from the survey, not from the model.

## Details

### What

Move the installed-file check to the last phase, as a check family of its own beside `generated`,
`registers` and `references`, and say so in `design@core@phases-gate-the-report` and
`design@core@owned-namespace-check`.

### Why it matters

In phase 2 the findings stop the run before any reference finding is listed, and every command
that writes a generated file refuses while one stands. `check --fix` repairs the installed files
before that gate, per `design@core@fix-before-the-checks`, so an ordinary upgrade no longer needs the
install first. What remains is an upgrade that removes a shipped file: the deletion is unstaged,
which no fix stages, so phase 2 stops the run before the generated files, and it takes a second run.
The owner judged the move correct and not worth doing now.

### What would close it

The check runs in the last phase, `index` and `index --staged` are no longer refused by a stale
install, and the two
design heads say so.
