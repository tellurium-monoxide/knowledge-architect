---
kind: question
---
# What review a release owes, and whether the release-status axis stays

## Summary

The root CLAUDE.md carries a section "Release status", and the routing table adds a reviewer agent,
`klarch-release-status-reviewer`, to every change that records or argues a decision. The owner
doubts that either adds value here, and proposes instead a release-related review that checks the
changelog was updated and that the version bump the changes require is recorded. Which review a
release owes, and what happens to the current section and agent, is not decided.

## Details

### What

The reviewer judges two predicates: a consumer-facing decision filed as special, and a decision
argued on the grounds that changing it later would be breaking. Both are stated in the root
CLAUDE.md, which every session reads. In the session that raised this question it ran once, at
the review before a merge, and found nothing; that run took 37052 subagent tokens, as
the harness's task notification reported. No design entry argues the axis; it was written when the
configuration was installed in this repository.

Leads the agent gave the owner, not ruled on:

- the section's statements of the version, of `design@knowledge-architect@stays-at-zero-x` and of
  "do not argue a decision on the grounds that changing it later would be breaking" are short
  baseline directives an agent does need;
- the bullet that a consumer-facing decision is not special answers a split into a "contract
  family" that this repository no longer has;
- a changelog and version-bump review would check `design@knowledge-architect@versioning-policy`,
  whose bump table and changelog tags no reviewer checks today.

### Why it matters

An axis that a review dispatches before every merge costs a subagent each time. One that checks only
what the baseline already states adds cost and no finding, while the changelog and the bump, which a
release depends on and `design@knowledge-architect@versioning-policy` decides, are checked by nobody
until `klarch-release` runs.

### What would close it

A design discussion with the owner, under `knowledge-architect-design`, deciding the instructions a
release review upholds: which changes owe a changelog item, how the required bump is recorded and
when, and whether the release-status section and agent stay, shrink or go. Then the configuration
change it decides, under `knowledge-architect-agent-configuration`.
