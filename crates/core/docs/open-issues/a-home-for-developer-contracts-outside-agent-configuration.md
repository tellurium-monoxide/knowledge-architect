---
kind: deferred
---
# Developer contracts sit in scoped CLAUDE.md files, which reach agents unreliably and are not required without a harness

## Summary

A component's contracts and traps that only a developer needs are routed to its CLAUDE.md. A
project that declares no agent harness owes no CLAUDE.md, so that content has no required home. The
owner also found that scoped CLAUDE.md and AGENTS.md files are delivered to agents unreliably, and
intends a home for component contracts unrelated to the agent configuration.

## Details

### What

A home for that content that does not depend on any agent harness, with CLAUDE.md dedicated to
agent instructions. The owner's long-term answer; the shape is open.

The owner then widened it, in the design discussion on modelling skills and agents as entities of
the checker: "after investigation, I found that delivery of scoped CLAUDE.md and AGENTS.md files is
very unreliable. In the future, I'm going to remove any recommendation for them, and change the
contract home. This would be its own session. I'd give it the report produced from the
investigation carried, and we would define a new home for component contracts, which would be
unrelated with agentic configuration. This is out of scope today, and its own design session. It'd
also require editing the agent-configuration sshipped skill."

So the work is no longer only for a project without agents. It concerns every project:

- the knowledge table's row for "a contract or a trap that only a developer needs" moves to a home
  unrelated to the agent configuration.
- the workflow stops recommending scoped CLAUDE.md and AGENTS.md files.
- the installed `knowledge-architect-agent-configuration` skill is edited to match. Its section `skill@knowledge-architect-agent-configuration@where-text-goes` chooses
  between the root CLAUDE.md, a skill, a scoped file and an agent.

The investigation's report is the owner's. It is not in this repository. The root CLAUDE.md stays.

### Why it matters

It strains `design@core@agents-table`, which drops CLAUDE.md from the required documents under
`harness = []`, and with it the one place a developer's contracts are required to sit. The checker
is built for agentic work, so a project without agents is a minor case today. But the content is
not agent-specific: a developer needs it whether an agent reads it or not. Where an agent does read
it, the owner's investigation found the scoped file's delivery unreliable, so a contract stated
there may not reach the agent working on the code, against
`goal@knowledge-architect@agents-work-without-drift`.

### Trigger

The owner opens the design session they announced, with the investigation's report, or the configuration for several agent providers is designed: that work
already decides which file holds what, per `issue@core@configuration-for-several-agent-providers`.
