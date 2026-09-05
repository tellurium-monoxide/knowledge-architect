---
kind: observation
---
# Two anchors' register homes may overlap, and the entry count then double-counts

## Summary

Two registers carried by ONE anchor may not share a home: `check::registers` reports it. Two
ANCHORS whose homes overlap are not refused — a location inside a component's register home is
the reachable case, recorded above — and a document under the overlap is then judged once per
instance and counted once per instance.

## Details

### What

Two registers carried by ONE anchor may not share a home: `check::registers` reports it.
Two ANCHORS whose homes overlap are not refused — a location inside a component's register home is
the reachable case, recorded above — and a document under the overlap is then judged once per
instance and counted once per instance. The `registers:` summary line's entry count is the only
visible symptom.

### Why it matters

The count is what tells a run that judged the entries from a run that judged
none, which is the whole reason it is printed. A count nobody can trust is worse than no count.

### What would close it

The refusal the entry above asks for closes the reachable case. If a
second one is found, count distinct documents rather than instance-and-document pairs.
