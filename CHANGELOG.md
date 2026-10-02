# Changelog

One section per released version, and one working section, `Unreleased`, for the changes since the
last release, which the release renames to its version. Each item is tagged with the surface it touches: `checks`,
`cli`, `manifest`, `library`, `agent-skills`, `gates`. The versioning policy is
`design@knowledge-architect@versioning-policy`.

## Unreleased

- `checks`: `commits` refuses a citation of a commit of its range by SHA, in a message or in a
  document of a commit's tree, where the manifest turns it on.
- `manifest`: `[commits] refuse-branch-shas`, off when absent.
- `agent-skills`: a plan document lists the tripwires and issues that reference each decision its
  work reverses or rewrites, and the readiness checks ask for them.
- `agent-skills`: a plan document is committed before its reviews, and each repair is a further
  commit.
- `agent-skills`: a plan document whose work changes what the gates check lands in a merge of its
  own, before the first step.
- `agent-skills`: the result a scheduled review is expected to give is not an acceptance criterion.
- `agent-skills`: a design discussion's first round states which grounding commands ran, and a
  measured fact carries the command that re-takes it.

## 0.1.0

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
- `agent-skills`: the first installed skills, decision-recording and issue-tracking. The
  package ships its build script and its content/ directory, from which the list of shipped files
  is generated.
- `agent-skills`: the planning skill, and the transcript-conformity reviewer agent.
- `agent-skills`: review, and the reviewer agents standing-state, decision-record,
  routing, code-claims and cold-implementer.
- `agent-skills`: setup, agent-configuration, and the primer, which the project's root
  CLAUDE.md imports.
- `agent-skills`: goal-setting and the retrospective.
- `agent-skills`: goal-setting places a goal in the Component responsible for it; setup
  proposes two goals for a project's maintenance tool.
- `gates`: the package knowledge-architect-gates, the library that runs a project's merge gates,
  with the recommended list of a Rust project that uses the checker.
- `agent-skills`: setup gains a section for a Rust project: one maintenance crate pins the
  checker and runs the gates library, with its aliases and a continuous integration workflow.
- `agent-skills`: the design skill, forked from designing-together 0.6.0; the retrospective carries
  its expectation set.
- `agent-skills`: every installed skill is named by its activity as a noun: design,
  decision-recording, issue-tracking, review, setup, goal-setting, agent-configuration, planning,
  retrospective.
- `agent-skills`: the transcript-conformity reviewer tells a scope change from an agent's addition,
  and the planning and review skills keep an addition until the owner rules.
