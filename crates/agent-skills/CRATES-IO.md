# knowledge-architect-agent-skills

The text that the knowledge-architect checker installs into a project's agent configuration:
skills, subagent definitions, and a primer the project's root CLAUDE.md imports.

A project does not depend on this crate directly. It uses the checker's install command, which
embeds this text and writes it into the project:

```sh
klarch install-agent-skills
```

The checker then reports any installed file that differs from the text of its version.

## Documentation

- The library API, the list of files it carries:
  [docs.rs](https://docs.rs/knowledge-architect-agent-skills).
- The [repository README of this crate](https://github.com/tellurium-monoxide/knowledge-architect/blob/main/crates/agent-skills/README.md).
- The checker: [knowledge-architect on crates.io](https://crates.io/crates/knowledge-architect).
- The project: [the repository](https://github.com/tellurium-monoxide/knowledge-architect).
