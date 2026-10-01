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
