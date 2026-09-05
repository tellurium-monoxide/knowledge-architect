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
`cargo knowledge commit-message <file>` over a generated file of the wanted size.

### Why it matters

The hook sits in front of every commit, and `git commit -F` accepts a
generated file. A message of a few hundred lines — which is what this project writes — costs
nothing measurable, so this is a hazard rather than present pain.

### What would close it

Locating the quadratic term and removing it, or a stated cap on the
message size the command will read. The scanner's own line-offset lookup was already made
logarithmic for documents, so the term is likely in the same shape somewhere the message path
reaches differently.
