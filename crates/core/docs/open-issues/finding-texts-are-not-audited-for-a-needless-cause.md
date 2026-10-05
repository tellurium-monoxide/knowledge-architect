---
kind: todo
---
# The findings' texts are not audited against the rule that a cause is named only where the repair depends on it

## Summary

`design@core@finding-names-the-repair` says a finding names a cause only where the repair depends
on which cause holds. Two findings were changed to meet it: the stale generated index, and the
summary line that `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` asks for.
No other finding's text has been read against it.

## Details

### What

The core constructs its findings in `path@core@src/check/`, `path@core@src/manifest.rs`,
`path@core@src/entity.rs`, `path@core@src/cli/mod.rs` and `path@core@src/cli/history.rs`, which
judges commits. Each finding's statement and action are string literals at the site that raises
it. Nobody has listed, for each finding, the
causes that can produce it and whether one repair is correct for all of them.

### Why it matters

A finding that names a cause its reader does not have sends that reader to look for an edit, a
file or a decision that does not exist, as the stale index's finding did after an upgrade, against
`goal@knowledge-architect@adoption-is-easy`. The rule holds only where someone read the finding
against it.

### What would close it

Every finding the core raises read against `design@core@finding-names-the-repair`: each that names
a cause while one repair fits every cause is reworded to name the repair alone, and each that
names a cause the repair depends on is left as it is. The audit's commit lists the findings read
and the outcome for each.
