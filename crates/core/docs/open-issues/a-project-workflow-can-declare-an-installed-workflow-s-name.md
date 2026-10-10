---
kind: question
---
# A project's own saved workflow can declare the name of an installed one, and nothing reports it

## Summary

The installer owns a saved workflow by its file name, `.claude/workflows/knowledge-architect-<name>.js`.
Claude Code calls a saved workflow by the name its `meta` declares, not by its file name. So a
project's own file, such as `path@elsewhere@.claude/workflows/mine.js`, that declares
`name: 'knowledge-architect-agentic-workflow-audit'` takes the installed workflow's name, and
`check` reports nothing: the checker reads no JavaScript. What the harness does with two saved
workflows of one name is not established.

## Details

### What

Reproduced by the adversarial review of the change that ships the audit's script as a saved
workflow: in a worktree of this repository, a tracked `path@elsewhere@.claude/workflows/mine.js` declaring that
name, then `cargo klarch check`, which exited 0 with `PASSED: no findings`. For an agent, the
counterpart is held: a frontmatter `name` equals the file's name, per
`design@core@harness-entity-names`. The build already holds an installed workflow's declared name
equal to its file's stem, `check_workflow_name` in `path@agent-skills@src/render.rs`; nothing holds
a project's own workflow away from the installer's names.

Not established: whether the harness then runs the project's file, the installed one, either by
the order it lists them, or refuses both.

### Why it matters

`design@core@owned-namespace-check` decides ownership by a name, so that a project's own file and
an installed one never collide. For a saved workflow the name the harness uses is the declared
one, so a collision the file names avoid can still happen.

### What would close it

A probe: two files under `path@agent-config@workflows/` declaring one name, each returning its file name,
run by that name. If the harness runs the project's file, or either at random, the check reads the
declared name of each project workflow and reports one that begins with the installer's prefix,
as it reports a skill or an agent whose name does. If it refuses both, or always runs the
installed one, the entry closes with the measurement.
