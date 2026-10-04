# knowledge-architect

A checker for a project's design record, and the agent workflow that writes it. It is built for
projects developed mostly by AI agents.

- **The checker, klarch.** A project declares in a manifest what it records and where: its
  components, their design decisions, goals, open issues and tripwires. The checker verifies that
  every reference between those records resolves, that every record has the shape its register
  declares, and that the generated listings are current.
- **The agent workflow.** The checker installs a set of agent skills, subagent definitions and a
  primer into the project: setting up, writing goals, designing, planning, recording decisions,
  tracking open issues, reviewing.

## Install and run

A project pins one exact version, written below as `<version>`, and declares it in its manifest as
`[project] checker-version = "<version>"`; a binary of another version refuses to run. In a Rust
project, a maintenance crate depends on this one and runs the checker; any other project installs
the binary into a directory of its own, added to the project's `.gitignore` before the first check:

```sh
cargo install --locked --root .tools --version =<version> knowledge-architect
.tools/bin/klarch install-agent-skills
.tools/bin/klarch check
```

The installed setup skill then leads the setup of the project.

## Documentation

- The commands, the manifest and the exit codes: the
  [repository README of this crate](https://github.com/tellurium-monoxide/knowledge-architect/blob/main/crates/core/README.md).
- The library API, for a binary that registers extensions:
  [docs.rs](https://docs.rs/knowledge-architect).
- The project, its goals and its design: [the repository](https://github.com/tellurium-monoxide/knowledge-architect).
