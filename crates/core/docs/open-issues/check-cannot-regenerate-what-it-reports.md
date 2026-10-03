---
kind: todo
---
# The check reports a stale generated file, and a second command has to rewrite it

## Summary

Nearly every edit that adds, removes or retitles an entry makes a generated index stale, so
`check` fails until `index` runs, and then `check` runs again. The owner wants an optional flag
on `check` that also writes the generated files.

## Details

### What

`check` verifies every generated file against its regeneration, and on a mismatch names the repair,
"run `<command> index`"; `index` writes the files. Both take their destinations and bytes from one
function, per `design@core@generated-files-are-pure`. Updating the indexes is needed after nearly
every such edit, in the owner's words "always needed", so a session runs `index` and then `check`
again each time: two commands for one step. A session in thaum on v0.1.0 met the same friction
with a citation index its extension generates.

The owner's words: "updating index is *always needed*, and needing two commands for this kinda
inefficient. check having an optional flag to also do the updates would be convenient."

### Why it matters

Each edit to a register costs a second command and a second run of the check, and a session that
forgets `index` reads a failed check before it reads its own work. The flag must not let a writer
write over an incomplete model: `index` refuses when an earlier phase holds findings, and a flag
that wrote anyway would fire `tripwire@core@phases-gate-the-report-two` against
`design@core@phases-gate-the-report`. The flag also reverses a clause of
`design@core@model-then-checks`: "Nothing writes to the tree while checking". And it meets the
live rejected alternative "`cargo klarch index` prints the diff it would apply, and `--write`
applies it", which lost to `design@core@generated-files-are-pure` on a survey of fmt, gofmt and
prettier.

### What would close it

An optional flag of `check`, whose name is a decision of its own, that writes every generated file
when phases 1 to 3 hold no finding, as `index` does, and then judges the last phase against the
written tree. A run with earlier findings writes nothing, as `index` does today. Tests that the
flag writes a stale index and passes, that it writes nothing over a phase-2 finding, and that an
extension's generated file is written too. The CLI section of `path@core@README.md` names it. The
closing change rewrites in place the "Nothing writes to the tree while checking" paragraph of
`design@core@model-then-checks`, and its argument says whether the reason the `--write`
alternative lost covers a writer folded into the check, or only a check folded into the writer.
