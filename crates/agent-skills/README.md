# knowledge-architect-agent-skills

The text that `knowledge-architect` installs into a project's agent configuration: skills,
subagent definitions, and a primer the project's root CLAUDE.md imports. This crate only carries
the text. The checker writes it into a project with its `install-agent-skills` command, and checks
that the installed files match it.

A project depends on `knowledge-architect`, never on this crate directly: cargo fetches it.

No installed skill covers a bounded problem yet: a clear requirement whose main risk is
over-building.
