---
kind: todo
---
# The checker does not read the structure of a project's own skills

## Summary

A project's agent skills are prose files in its agent configuration. The checker reads each as a
skill entity, per `design@core@harness-kinds`: its name against the id grammar and against its
frontmatter `name`, and its level-two headings against the section rule. It reads nothing else of
their structure: the project's prefix, their description fields, which installed skill one adds
to.

## Details

### What

Checks over the rest of the structure of a project's own skills and agents: the prefix, the
routing table, the description field and which installed skill one adds to. Long term; the shape is open. Skills and agents are entities, so the checks read the
entity table rather than guess from paths.

The owner's direction for the prefix, when the bare-name lint's reliance on it was ruled: "adding
a check that asserts a project's skill names follow the `<project-name>` prefix rule. Maybe with the
prefix being defined separately from project name in the manifest, to allow for what shortcuts
(useful if the project has a long name, and needed here)." Needed here because this repository's
own skills take the prefix `klarch-`, not its name, per `design@knowledge-architect@klarch-prefix`.
A prefix declared in the manifest would also reach `design@agent-skills@skill-name-prefix`, which
names the prefix after the project.

### Why it matters

The installed workflow asks a project to name its skills with its own prefix, per
`design@agent-skills@skill-name-prefix`, and to route each installed skill to the project skills
that add to it, per `design@agent-skills@routing-table-shape`. A skill that breaks either is found
by review or not at all. The bare-name lint, `design@core@bare-skill-name-reported`, rests on the
prefix too: a project skill named by an ordinary word, such as `check`, makes every backticked use
of that word a finding.

### What would close it

Checks shipped in the checker over the rest of a project's own skills' and agents' structure: the
project's prefix, the routing table, the description field, and which installed skill a project
skill adds to.
