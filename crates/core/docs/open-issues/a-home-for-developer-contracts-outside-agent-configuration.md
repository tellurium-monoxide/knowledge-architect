---
kind: deferred
---
# Developer contracts have no home when a project declares no agent harness

## Summary

A component's contracts and traps that only a developer needs are routed to its CLAUDE.md. A
project that declares no agent harness owes no CLAUDE.md, so that content has no required home.

## Details

### What

A home for that content that does not depend on any agent harness, with CLAUDE.md dedicated to
agent instructions. The owner's long-term answer; the shape is open.

### Why it matters

It strains `design@core@agents-table`, which drops CLAUDE.md from the required documents under
`harness = []`, and with it the one place a developer's contracts are required to sit. The checker
is built for agentic work, so a project without agents is a minor case today. But the content is
not agent-specific: a developer needs it whether an agent reads it or not.

### Trigger

The owner schedules it, or the configuration for several agent providers is designed: that work
already decides which file holds what, per `issue@core@configuration-for-several-agent-providers`.
