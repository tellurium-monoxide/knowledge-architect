---
kind: todo
---
# An entry that sits inside a declared group

## Summary

The group is a subdirectory and is no part of the entry's id, so regrouping breaks no reference.

## Details

### What

This file sits in `housekeeping/`, which the `register.toml` beside the README declares.

### Why it matters

A group nothing declares is a finding, and a declared group with no subdirectory is a finding.
Only an instance that has both a declaration and a subdirectory shows that the two agree.

### What would close it

Nothing. It exists so that a test has a grouped entry to find.
