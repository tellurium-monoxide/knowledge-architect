---
kind: observation
---
# The notes are not a component

## Summary

This directory carries outstanding state and nothing else a component carries, so it is
declared a location rather than a component.

## Details

### What

`notes/` is declared in the mock's manifest as a location carrying the issue and reading
registers.

### Why it matters

Without the declaration nothing reads this directory, and what is open here would not appear
in the report at all.

### What would close it

Nothing. It exists so that a test has an entry to find.
