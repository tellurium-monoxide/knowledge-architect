---
kind: defect
---
# `commits` treats HEAD as the tip when a range holds HEAD before its last commit

## Summary

In `commits`, `path@knowledge@documentation/src/cli/history.rs`, a commit is the tip when it is the walk's last
commit **or** HEAD. With HEAD detached inside the range, HEAD is misreported as the tip: its tree
failing gives exit 2 with "COULD NOT RUN" rather than a finding naming the commit, and a HEAD
whose tree stops early ends the walk before the commits after it.

## Details

### What

Reproduction: commits B, then X adding a live document that names a reference nothing defines,
then Y removing it.

```sh
git checkout Y && cargo knowledge commits B..Y   # exit 1, X failed, its finding listed: correct
git checkout X && cargo knowledge commits B..Y   # exit 2, "COULD NOT RUN: the range's tip carries a tree with 1 finding(s)"
```

The second run lists no finding for X. The walk-stopping case, a mid-range HEAD whose tree stops
before phase 4, is reasoned from the `break` on `Outcome::Unjudged` and not reproduced. The exit
code is nonzero in both, so no range passes that should fail. The two mutations that drop either
half of the tip test, `*sha == last` alone and HEAD alone, both survive the suite.

### Why it matters

`design@knowledge@a-commit-message-is-a-document` names the tip as the walk's last commit, and
HEAD wherever the range holds it, which is this behaviour for a range whose last commit is HEAD
and a wrong verdict label otherwise. The gates always pass a range ending at HEAD, so they are
not affected; a session running `commits` by hand over another range is.

### What would close it

Deciding whether HEAD belongs in the tip test at all, rewriting the head's sentence to match, and
a test over a range whose last commit is not HEAD that kills both mutations.
