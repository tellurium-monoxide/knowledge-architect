---
kind: observation
---
# The mock has one issue so the register is not empty

## Summary

An empty file register is a legitimate shape, and this project exercises the other one.

## Details

### What

The issue register of this component holds exactly one entry, this one.

### Why it matters

A register with no entry never exercises the entry-shape assertions.

### What would close it

Nothing. It exists so that a test has an entry to find.
