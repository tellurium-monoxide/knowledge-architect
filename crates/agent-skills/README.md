# knowledge-architect-agent-skills

The text that `knowledge-architect` will install into a project's agent configuration: skills,
subagent definitions, and a primer the project's root CLAUDE.md imports. This crate only carries
the text. Neither the install command nor the check of installed files exists yet: both arrive
before the first release.

A project depends on `knowledge-architect`, never on this crate directly: cargo fetches it.

The set is empty in this version. The skills arrive in a later release.
