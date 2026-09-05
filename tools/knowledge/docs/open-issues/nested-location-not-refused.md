---
kind: defect
---
# A location nested inside another anchor's register home is not refused

## Summary

`check::registers` refuses a component inside a location, because nesting one full register set
inside a partial one gives a document two candidate homes. The reverse nesting — a location
whose path sits inside another anchor's register home — is accepted.

## Details

### What

`check::registers` refuses a component inside a location, because nesting one full
register set inside a partial one gives a document two candidate homes. The reverse nesting — a
location whose path sits inside another anchor's register home — is accepted. A location declared
at `<component>/docs/open-issues/nested`, carrying its own issue register, produces a dozen
findings against the OUTER component's issue register instead: the nested location's own
`README.md` and `index.md` are read as malformed entries of the outer register, and its directory
is reported as a group inside a group.

### Why it matters

The findings name the wrong anchor and the wrong register, so a reader repairs
the outer component's directory and the declaration that caused it stays. It is the same one-home
failure the component-inside-a-location refusal exists against, reached from the other side.

### What would close it

Refuse a location whose path sits inside any anchor's register home, with
the same finding shape the component-inside-a-location case uses.

### Observed

An adversarial review of the register-shape checks, over a copy of a mock project.
Reproduce by adding such a location to any mock project's manifest and running
`cargo knowledge check --only registers`.
