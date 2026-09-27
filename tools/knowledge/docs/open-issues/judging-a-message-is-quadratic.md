---
kind: observation
---
# Judging a message costs time quadratic in its line count

## Summary

The message analysis is quadratic in the number of lines. Measured on a debug build of this
tool, one reference per line: 1 000 lines in 0.08 s, 5 000 in 0.94 s, 10 000 in 3.35 s, 20 000
in 13 s, and 200 000 lines (4.7 MB) not finished after 120 s.

## Details

### What

The message analysis is quadratic in the number of lines. Measured on a debug build of
this tool, one reference per line: 1 000 lines in 0.08 s, 5 000 in 0.94 s, 10 000 in 3.35 s,
20 000 in 13 s, and 200 000 lines (4.7 MB) not finished after 120 s. Re-take with
`cargo knowledge commits HEAD~1..HEAD` after committing a generated message of the wanted size
with `git commit --allow-empty -F <file>`, and subtract the same run over a one-line message,
which is the cost of assembling the tree. The alias builds in release, so a re-take is not
comparable with the debug figures above.

### Why it matters

`commits` runs after each commit and in every gate run and CI run, and `git commit -F` accepts a
generated file. A message of a few hundred lines — which is what this project writes — costs
nothing measurable, so this is a hazard rather than present pain.

### What would close it

Locating the quadratic term and removing it, or a stated cap on the
message size the command will read. The scanner's own line-offset lookup was already made
logarithmic for documents, so the term is likely in the same shape somewhere the message path
reaches differently.
