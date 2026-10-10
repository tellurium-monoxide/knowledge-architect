---
kind: todo
---
# The checker does not read the structure of a project's own skills

## Summary

A project's agent skills are prose files in its agent configuration. The checker reads each as a
skill entity, per `design@core@harness-kinds`: its name against the id grammar, per
`design@core@unusable-harness-name-is-a-finding`, and against its frontmatter `name`, per
`design@core@harness-entity-names`, and its level-two headings against the section rule. It
reads nothing else of their structure: the project's prefix and their description fields.

## Details

### What

Checks over the rest of the structure of a project's own skills and agents: the prefix and the
description field. Long term; the shape is open. Skills and agents are entities, so the checks read the
entity table rather than guess from paths.

The owner's direction for the prefix, when the bare-name lint's reliance on it was ruled: "adding
a check that asserts a project's skill names follow the `<project-name>` prefix rule. Maybe with the
prefix being defined separately from project name in the manifest, to allow for what shortcuts
(useful if the project has a long name, and needed here)." Needed here because this repository's
own skills take the prefix `klarch-`, not its name, per `design@knowledge-architect@klarch-prefix`.
A prefix declared in the manifest would also reach `design@agent-skills@skill-name-prefix`, which
names the prefix after the project. Open for that check's design session:

- the field's name, its table, and its default when a manifest leaves it out, which decides
  whether every existing manifest must gain the field;
- the changelog entry it owes: a `manifest` entry for the field, and a `checks` entry for the
  check;
- whether a project's own agents follow the same rule: the owner's direction names skill names, and
  `design@agent-skills@skill-name-prefix` names skills and agents.

### Why it matters

The installed workflow asks a project to name its skills with its own prefix, per
`design@agent-skills@skill-name-prefix`, and to begin each skill's description with MUST and its
symptom, per `design@agent-skills@no-routing-table`, since that description is how the skill reaches
a session. A skill that breaks either is found by review or not at all. The bare-name lint, `design@core@bare-skill-name-reported`, rests on the
prefix too: a project skill named by an ordinary word, such as `check`, makes every backticked use
of that word a finding.

### What would close it

Checks shipped in the checker over the rest of a project's own skills' and agents' structure: the
project's prefix and the description field.
