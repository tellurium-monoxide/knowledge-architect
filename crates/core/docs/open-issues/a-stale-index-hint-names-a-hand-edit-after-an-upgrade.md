---
kind: todo
---
# After an upgrade, a stale generated file's finding names a hand edit as the cause

## Summary

When a release changes the bytes the generator writes, every consuming project's committed index
is stale after the upgrade, and the finding says the cause is a hand edit. Finding C1 of the
retrospective of 2026-10-04.

## Details

### What

The finding's text is in `path@core@src/check/generated.rs`: "the generated file is out of date →
run `klarch index`; the listing is a function of the entries beside it, and a hand edit is what
this reports". The adversarial reviewer of the branch that changed a one-row index's count line to
`1 entry` reproduced it, as reported: a copy of the `minimal` mock, git-initialised, the pin set to
the binary's version, and its issue index set back to `1 entries`; `klarch check` exits 1 with that
finding, and `check --fix` repairs it. Not reproduced by the session that writes this entry.

After an upgrade nobody edited the file. The repair named is right; the cause is wrong.

### Why it matters

A consumer reading the cause looks for an edit that does not exist, against
`goal@knowledge-architect@adoption-is-easy`. The changelog's Migration entry gives the cause, but
the finding is what the consumer reads first.

### What would close it

The finding names a change of the generator as the likely cause when the entries behind the file
are unchanged since the file's last commit, or names both causes; with a test for each wording.
