---
kind: todo
---
# After an upgrade, a stale index's finding names a hand edit as the cause

## Summary

When a release changes the bytes the generator writes, every consuming project's committed index
is stale after the upgrade, and the finding says the cause is a hand edit. Finding C1 of the
retrospective of 2026-10-04.

## Details

### What

The finding's text is in `path@core@src/check/generated.rs`, for the index of a file register:
"the generated file is out of date → run `klarch index`; the listing is a function of the entries
beside it, and a hand edit is what this reports". An extension's generated file carries its own
text. Reproduced with the checker built from main after the change of a one-row index's count line
to `1 entry`: a copy of `path@core@tests/projects/minimal/`, git-initialised, its pin set to the
binary's version, and its docs/open-issues/index.md set back to `1 entries`. `klarch check` exits
1 with that finding, and `check --fix` repairs it, as the adversarial reviewer of that branch
reported.

After an upgrade nobody edited the file. The repair named is right; the cause is wrong. The same
finding text misleads on a branch that changes the generator, where `commits` reports every earlier
commit: `issue@core@a-contract-change-fails-every-earlier-commit-unexplained`. The two close by
different work at the same site, so closing either revisits the other.

### Why it matters

A consumer reading the cause looks for an edit that does not exist, against
`goal@knowledge-architect@adoption-is-easy`. The changelog's Migration entry gives the cause, but
the finding is what the consumer reads first.

### What would close it

The finding names a change of the generator as the likely cause when the entries behind the file
are unchanged since the file's last commit, or names both causes; with a test for each wording.
