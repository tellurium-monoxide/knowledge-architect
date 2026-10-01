---
kind: defect
kind: todo
---
# PLANTED: an entry that declares one key twice

## Summary

Two values for one thing leave whichever reader looks first to decide, so the block is refused.

## Details

### What

The frontmatter declares `kind` a second time.

### Why it matters

A listing reading the first value and a check reading the last would disagree in silence.

### What would close it

Keeping one of the two.
