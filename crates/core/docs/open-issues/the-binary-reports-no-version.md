---
kind: todo
---
# The binary reports no version

## Summary

`klarch --version` is refused: the command line declares no version flag. A session that needs to
know which checker ran, such as a retrospective recording the version it used, reads the project's
pin instead, which says what the project declares rather than what ran.

## Details

### What

`cargo klarch --version`, run in this repository, prints clap's error for an unknown argument and
"For more information, try '--help'." No other command prints the version either. The owner rated
the flag not critical, since consumers pin the checker exactly, and worth having.

### Why it matters

A report of which checker ran rests on the pin, per `design@agent-skills@finding-ids` and
`design@agent-skills@exact-pin`. The pin and the binary match unless a pin was edited and the binary
not rebuilt, so the gap is small; but a report from a project that builds the checker some other
way has no uniform instrument at all.

### What would close it

A `--version` flag on the binary, printing the version of the `knowledge-architect` package, with a
test of its output, and its CHANGELOG.md entry under New features (`cli`, minor).
