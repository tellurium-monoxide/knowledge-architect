---
kind: observation
---
# An entry that sits at the instance's top level

## Summary

An entry may stay ungrouped even where the instance declares groups.

## Details

### What

This file sits beside the README rather than inside `housekeeping/`, and the generated index
lists it above every group heading.

### Why it matters

Ungrouped entries come first in the index, and an instance whose every entry is grouped never
exercises that half of the ordering.

### What would close it

Nothing. It exists so that a test has an ungrouped entry to find.
