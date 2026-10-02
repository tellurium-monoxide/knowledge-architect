# Changelog

One section per released version, and one working section above them, `Next release`, which the
release renames to its version. Which change gets an entry, under which subsection, and with which
class is `design@knowledge-architect@changelog-entries`. Inside a subsection, entries are sorted by
surface, in the order `checks`, `cli`, `manifest`, `library`, `agent-skills`, `gates`; an empty
subsection is omitted.

## Next release

### Migration

- `checks`, major: every project carries docs/plans/ at its root, holding a `README.md`, specs/ and
  milestones/, and each of the two holds a `README.md` and a generated `index.md`. Move each spec
  into specs/ as one file, and each milestone into milestones/ as a directory holding its
  `README.md`; then run `index`. Any other file or directory directly under docs/plans/ is a
  finding.
- `checks`, major: a `path` citation of a plan document is refused. Cite a spec as
  `spec@plans@<id>`, a milestone as `milestone@plans@<id>` and a step of a milestone as
  `spec@<milestone>@<step>`. A citation from outside the plans directory of a file inside it is
  refused as reaching inside the anchor `plans`.
- `manifest`, major: a declared anchor named `plans` and a `[registers.spec]` or
  `[registers.milestone]` table are refused, and so is `spec` or `milestone` in a location's
  `registers` list.

### New features

- `checks`, minor: the reference kinds `spec` and `milestone`, carried by the anchor `plans`, and
  one anchor per milestone directory, carrying `spec` for its step specs. `show` prints a spec
  or a milestone document, and every reference to it.
- `checks`, minor: `commits` refuses a citation of a commit of its range by SHA, in a message or in
  a document of a commit's tree, where the manifest turns it on.
- `manifest`, minor: `[commits] refuse-branch-shas`, off when absent.

### Workflow

- `agent-skills`, patch: a plan document names every text that references a decision its work
  reverses or rewrites, restatements in a `CLAUDE.md` or a skill included, with the step or the
  harvest that judges or updates each.
- `agent-skills`, patch: a plan document is committed before its reviews, and each repair lands
  after them.
- `agent-skills`, patch: a milestone document whose first step changes what the gates check lands in
  a merge of its own, before that step.
- `agent-skills`, patch: a design discussion's first round states which grounding commands ran, and
  a measured fact carries the command that re-takes it.
- `agent-skills`, patch: a retrospective names each finding by a letter and a number (W, C, P), and
  states the version of knowledge-architect the session used.
- `agent-skills`, patch: a design audit lists an answer that widens or narrows a ruling of the owner
  as a default awaiting the owner, ruled at the audit before its point is implemented.
- `agent-skills`, patch: a step's harvest is reviewed before the merge, on the decision-record,
  routing and standing-state axes, and on transcript conformity where the transcript is available.
- `agent-skills`, patch: a review repair that would leave an earlier commit of the branch failing
  the project's checks is folded into that commit by a history edit, and the review's record says
  so.

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
