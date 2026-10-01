# Changelog

One section per released version. Each item is tagged with the surface it touches: `checks`,
`cli`, `manifest`, `library`, `agent-skills`. The versioning policy is
`design@knowledge-architect@versioning-policy`.

## Unreleased

- `manifest`: the manifest file is `knowledge-architect.toml`.
- `cli`: the binary is `klarch`.
- `library`: the library and the binary are one crate, `knowledge-architect`, imported as
  `knowledge_architect`.
- `manifest`: `[project] command` declares the command a project runs the checker by; `klarch`
  when absent. It is printed in messages and in generated index headers.
- `manifest`: `[agents] harness` declares the agent harnesses a project serves; `["claude"]` when
  absent. An empty list drops the CLAUDE.md requirement.
- `cli`: `install-agent-skills` writes the shipped agent files and removes unshipped ones from its
  namespace.
- `checks`: phase 2 reports an installed agent file missing, differing or unshipped, its deletion
  not staged, and a root CLAUDE.md that does not import a shipped primer or that the walk keeps
  out. `commits` does not compare installed files.
- `manifest`: a `command` that is empty or holds a line break or a backtick is refused.
- `cli`: `install-agent-skills` refuses a symbolic link on an owned path, and a manifest holding a
  refused declaration.
- `library`: the public API is the crate root and four modules: `cli` for a binary's `main`,
  `extension` for writing an extension, `document` for reading a document's parse, `testing`
  for running the core over a mock project. Every other module is private.
- `library`: `cli::Gathered` gathers what a run reads before any check; `complete_working_tree`
  returns it.
- `library`: `cli::Command`, `extension::Inputs`, `extension::ExtensionReport` and
  `extension::Resolution` are non-exhaustive.
- `library`: the `testing` feature is removed.
- `agent-skills`: the first installed skills, recording-a-decision and tracking-open-issues. The
  package ships its build script and its content/ directory, from which the list of shipped files
  is generated.
- `agent-skills`: the planning skill, and the transcript-conformity reviewer agent.
- `agent-skills`: dispatching-a-review, and the reviewer agents standing-state, decision-record,
  routing, code-claims and cold-implementer.
- `agent-skills`: setting-up, maintaining-agent-config, and the primer, which the project's root
  CLAUDE.md imports.
