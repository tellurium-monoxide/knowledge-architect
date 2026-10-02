---
name: knowledge-architect-setting-up
description: MUST use when a project adopts the knowledge-architect workflow, after the first install has run, and when a project moves its pin of the checker to another version. Covers how the checker is pinned and run, the declared command, the manifest and its Components, the documents and register homes each Component carries, the primer's import line, the project's rows of the knowledge table and its routing table, the project's skill prefix, the gates convention, and what happens to the documentation the project already has.
---

# Setting up

Scope: making a project conformant to the workflow, from a manifest the first install accepted to
a tree over which `cargo klarch check` passes, with the owner's goals stated. And moving the pin of
the checker to another version.

Not covered here: **stating the goals**, `knowledge-architect-setting-goals`; **moving the existing
documentation into the new homes**, which is planned work of its own (§8); **writing the project's
own skills**, `knowledge-architect-maintaining-agent-config`.

**Reaching this skill.** The skill is one of the files the install writes, so a session reads it
once the project holds a manifest the install accepts and the install has run. The smallest such
manifest is the file `knowledge-architect.toml` at the project's root:

```toml
[project]
name = "<project name>"
components = []

[walk]
skip-dirs = []
skip-files = []
exclude = []
```

After the first install, `cargo klarch check` lists every document the project still owes, each with
its repair. That list is this skill's work list.

The checker needs `git` 2.36 or newer, and a project inside a git repository. Building it needs a
Rust toolchain, whatever the project's own language.

**Every step that writes a durable statement on the owner's behalf is shown to the owner first**:
the Components, the goals, the place of each existing document. The owner rules; the agent proposes.

## 1. Pin the checker, and decide how it runs

A project runs the checker at the version it chose, and moves to another version by an explicit
edit. The installed skills move with it, since one version of the checker ships one version of
them.

- **A Rust project** adds a small crate to its workspace whose `main` calls the library's command
  line, depends on `knowledge-architect = "=<version>"`, and runs it through a cargo alias, for
  instance `cargo klarch`. The `=` pins exactly; without it, `"<version>"` accepts every later
  version below the next breaking one. `Cargo.lock` records the exact version, and `--locked` turns
  any change to it into a failure. A dependency alone builds no executable for the project:
  `cargo run -p` runs only the project's own packages, which is why the small crate exists.
- **Any other project** installs the binary into a directory of its own, ignored by git:
  `cargo install --locked --root <dir> knowledge-architect --version =<version>`. The binary is
  `<dir>/bin/klarch`. A plain `cargo install` is machine-wide, and two projects on one machine
  would then share one version.
- **A project with an extension** runs its own binary, which registers the extension, through a
  cargo alias. It does not install that binary under the plain name `klarch`: with two binaries of
  that name on the path, the first one found runs.

## 2. Declare the command

The command the project runs is declared in the manifest, and the checker prints it in its
messages, in the header of every generated listing and in every installed file:

```toml
[project]
command = "<the command, for instance cargo klarch or tools/bin/klarch>"
```

When the key is absent, the command is `klarch`. After declaring it, run the install again, so the
installed files carry it.

## 3. The Components and the locations

A **Component** is a directory the manifest names under `[project] components`, by its path; its
name is the basename of that path, and the project's root is a Component named by
`[project] name`. Propose the Components to the owner: one per unit that has its own goals and its
own design, such as a published package, a service, or a maintenance tool. A directory named after
its role, not after its package, avoids a Component whose name equals the project's.

A **location** is a directory that carries only the registers it declares, for content that belongs
to no Component, such as the agent configuration:

```toml
[locations.agent-config]
path = ".claude"
registers = ["issue"]
```

What the walk must not read is declared under `[walk]`, each row with its reason in a comment
beside it: `skip-dirs` and `skip-files` for paths, `exclude` for another project kept inside this
one. `[agents] harness` is absent for the default, the `claude` harness.

## 4. The documents each Component carries

Every Component carries the same documents. `cargo klarch check` names each one missing:

| document | holds |
| --- | --- |
| `README.md` | how a user uses the Component |
| `CLAUDE.md` | the contracts and traps a developer needs, true of the code as it stands |
| `docs/goals.md`, or `docs/goals/` with a `README.md` | what the Component is for: at least one goal, under `knowledge-architect-setting-goals` |
| `docs/design.md`, or `docs/design/` with a `README.md` | how it is built and why; it may hold no entry yet |
| `docs/rejected-alternatives.md` | what lost, and why; it may hold no entry yet |
| `docs/tripwires.md`, or `docs/tripwires/` with a `README.md` | evidence that would flip a decision; it may hold no entry yet |
| `docs/open-issues/` | one file per outstanding item, a hand-written `README.md` and an `index.md` that `cargo klarch index` generates |

Each document opens with a short introduction saying what it holds and what it does not. An empty
register home says it holds no entry yet. **Every Component states at least one goal**: run
`knowledge-architect-setting-goals` with the owner for each one.

## 5. The root CLAUDE.md

The project's root `CLAUDE.md` holds, besides what the project already keeps there:

- **the primer's import line**, alone on its own line of prose, outside any code block:
  `@.claude/knowledge-architect/PRIMER.md`. The install never writes it, because the root
  `CLAUDE.md` belongs to the project; the check reports it missing until it is there;
- **the project's rows of the knowledge table**, under a heading of their own. The first is the
  plans directory: propose `docs/plans/`, and write the path the owner accepts. Then each kind of
  statement the project keeps somewhere the primer's table does not name: its changelog, a register
  it declares, a directory with a convention of its own;
- **the routing table**: one row per installed skill or agent that a project skill adds to. It is
  empty until the project writes a skill of its own;
- **the project's skill prefix**: its name and a hyphen, which names every project skill and agent.

## 6. The gates

Recommend one command that runs every check the project owes before a merge (formatting, the
document check, the commit messages, the linters, the tests), runs them all even when one fails,
and exits non-zero when any fails. A verdict is then one exit code, and nothing is read from output
filtered through a pipe. In a Rust project, the shape is a maintenance crate in the workspace, run
through a cargo alias such as `cargo x gates`, whose gates command hands the project's gate list
to the published library knowledge-architect-gates. The library runs the gates; the list is the
project's own.

A maintenance tool of that kind is a Component of its own, which serves the project rather than its
consumers. Propose these two goals for it, under `knowledge-architect-setting-goals`, for the owner's
ruling like any draft:

```markdown
## Every check the project owes before a merge runs from one command `##one-command-runs-every-gate`

One command runs every check a branch must pass before it merges: formatting, the document check,
the commit messages, the linter where the project has one, and the tests. It runs them all when one
fails, and gives one verdict as its exit code. It is met while a session needs no other command to
know whether a branch may merge, and, where the project has continuous integration, it runs the
same command.

## A task performed repeatedly in the project is a command of the tool `##repeated-tasks-are-automated`

A task that sessions perform in this repository more than once, by hand and in the same steps,
becomes a command of the tool, so it runs the same way each time. It is met while no such task is
left to be done by hand.
```

## 7. Finish

- `cargo klarch index`, then `cargo klarch check` until it passes.
- Commit the manifest, the documents, the installed files and the root `CLAUDE.md` together.

## 8. Existing documentation

A project that already has documentation keeps it until its move is planned:

1. **Inventory** every document that records decisions, goals, open work or conventions: design
   documents, decision records, notes, an issue tracker, a `CLAUDE.md` or another agent's
   configuration.
2. **Propose a destination for each**, by the primer's knowledge table: a goal, a design entry, a
   rejected alternative, an issue, a tripwire, a scoped `CLAUDE.md`, a skill, or nothing, for history
   that records no current decision. Say what is uncertain: which recorded decisions still hold, and
   which describe a design the code has left.
3. **The owner rules** on the proposal.
4. **Open one `todo` issue for the move**, in the root Component, holding the inventory and the
   rulings. The move is then planned work, under `knowledge-architect-planning`, and runs as a
   milestone when the owner schedules it. Until then, the old documents and the new homes both
   exist, and the issue is what records that.

Setting up stops at a conformant structure, the goals, and that issue.

## Moving the pin

To move to another version:

1. Edit the pin: the version in the small crate's `Cargo.toml`, or the version of the local
   install.
2. Read the changelog of every version crossed. A minor version under `0.x`, or any major version,
   may make a check stricter or ask for a change to the project's layout.
3. Run `cargo klarch install-agent-skills`, then follow `knowledge-architect-maintaining-agent-config`
   for what an upgrade owes the project's own configuration.
4. `cargo klarch check`, and commit the pin, the installed files and the repairs together.
