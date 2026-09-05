---
kind: observation
---
# A path with a newline in it breaks the one-finding-per-line output

## Summary

A finding is printed as one line opening with its path, and a filename holding a newline is
printed raw. The finding then spans two lines and the first is a truncated path.

## Details

### What

A finding is printed as one line opening with its path. A filename holding a newline is
printed raw, so the finding spans two lines and the first is a truncated path.

### Why it matters

Nothing parses this output today, so it costs a reader one confusing line.
It stops being cosmetic the moment anything reads the output by line, which a hook or a CI
annotation would.

### What would close it

Escaping a path in `Finding`'s display, or refusing such a name in the
walk with a finding of its own.

### Reproduce

Commit a `.md` file whose name holds a newline and a rule number with no quote,
then read `cargo knowledge check`.
