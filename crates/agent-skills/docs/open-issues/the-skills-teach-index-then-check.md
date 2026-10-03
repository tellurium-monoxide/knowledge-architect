---
kind: todo
---
# The installed skills teach `index` then `check` where `check --fix` is one command

## Summary

`check --fix` writes the stale generated files and then checks, in one command. The installed
skills still tell a session to run the index command, then the check, as two steps.

## Details

### What

The setup skill's list of steps says "`{{command}} index`, then `{{command}} check` until it
passes" (crates/agent-skills/content/skills/setup/SKILL.md). The setup skill's table of
documents, and the planning skill's layout section, name the index command as what writes each
`index.md`. Those lines stay true, since `index` stays as a command, per
`design@core@check-fix-flag`, but none names `--fix`.

### Why it matters

A session that follows the skills runs two commands where one serves, the friction
`design@core@check-fix-flag` removes. It does not strain a decision: both commands stay correct.

### What would close it

The installed skills name `check --fix` where a session edits a register and then checks, keeping
`index` where it is the subject. The installed copies are reinstalled, and a CHANGELOG Workflow
entry is written if a person watching sessions would see the change.
