# Changelog

One section per released version, and one working section above them, `Next release`, which the
release renames to its version. Which change gets an entry, under which subsection, and with which
class is `design@knowledge-architect@changelog-entries`. Inside a subsection, entries are sorted by
surface, in the order `checks`, `cli`, `manifest`, `library`, `agent-skills`, `gates`; an empty
subsection is omitted.

## Next release

### Workflow

- `agent-skills`, patch: the setup skill's section for a Rust project ties every build to its
  checkout: a cargo `[env]` variable valued at the checkout's root, read by every library root and
  every target of a package with no library, and named by every build script. A target directory
  shared by two checkouts then rebuilds instead of running the other checkout's build.

## 0.2.0

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
- `checks`, major: a plan document owes its sections: a spec of docs/plans/specs/ and a milestone's
  README the plan sections, in order, and a step spec the step sections. Add the missing ones.
- `checks`, minor: a `path@*@<path>` reference that only a declared location carries is reported,
  as the design of the generic form always said; anchor it at the location instead.
- `checks`, major: an item of a plan cited from outside it is refused; cite the plan whole. A
  backticked span whose head, before its first at sign, is `spec`, `milestone`, `thread`,
  `argument`, `criterion`, `acceptance`, `plans`, or the id of a spec or a milestone, is now a
  reference candidate.
- `manifest`, major: a declared anchor named `plans` and a `[registers.spec]` or
  `[registers.milestone]` table are refused, and so is `spec` or `milestone` in a location's
  `registers` list.
- `manifest`, major: a `[registers.thread]`, `[registers.argument]`, `[registers.criterion]` or
  `[registers.acceptance]` table is refused, and so is any of those names in a location's
  `registers` list. Rename a register of the project that carries one of them.
- `manifest`, major: a location declared at docs/plans/, and a component register whose `dir` is
  `plans`, are refused, because the tool constructs the anchor `plans` there. Move the location
  elsewhere, or give the register another `dir`.
- `library`, major: `cli::Command::Check` takes its arguments, `Command::Check(CheckArgs)`. Code
  that builds or matches the variant by name writes `Command::Check(CheckArgs { fix: false })`, or
  matches `Command::Check(_)`.
- `agent-skills`, major: the primer names the plans directory, docs/plans/, and the roadmap. Remove
  the row naming the plans directory from the project's own rows of the knowledge table, in its root
  `CLAUDE.md`.
- `agent-skills`, major: the agent `knowledge-architect-transcript-conformity-reviewer` is renamed
  `knowledge-architect-transcript-reviewer`. Rename it wherever the project's own skills, agents
  or `CLAUDE.md` files name it.

### New features

- `checks`, minor: the plan items, `thread`, `argument`, `criterion` and `acceptance`: a
  level-three heading with its slug under the section of its kind, cited
  `<kind>@<plan>@<id>` from inside its plan, where each spec file and each milestone is an anchor.
- `checks`, minor: the reference kinds `spec` and `milestone`, carried by the anchor `plans`, and
  one anchor per milestone directory, carrying `spec` for its step specs. `show` prints a spec
  or a milestone document, and every reference to it.
- `checks`, minor: `commits` refuses a citation of a commit of its range by SHA, in a message or in
  a document of a commit's tree, where the manifest turns it on.
- `cli`, minor: `check --fix` applies every safe fix before checking: it installs the agent files
  when one is missing, differs or is no longer shipped, writes every stale or missing generated
  file, lists each, then runs the check. An upgrade that removes a shipped file takes a
  `git add` and a second run.
- `cli`, minor: `--version` prints the version of the checker that runs, from any directory.
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
  routing and standing-state axes, and by the transcript reviewer where the transcript is
  available.
- `agent-skills`, patch: a review repair that would leave an earlier commit of the branch failing
  the project's checks is folded into that commit by a history edit, and the review's record says
  so.
- `agent-skills`, patch: the planning skill writes a plan document's threads, arguments, criteria
  and acceptance criteria as items, ``### <statement> `##<id>` ``, under the section titles the
  checker matches, with Arguments right after Threads. It assembles the document from the discussion's
  transcript through a subagent, and the transcript reviewer reads every assembled document.
- `agent-skills`, patch: an optional roadmap, docs/roadmap.md, orders known work on the owner's
  word. The commit that adds a plan document rewrites the row of the issue it closes, and the commit
  that deletes a plan document removes its row.
- `agent-skills`, patch: a milestone's design is written into one spec per step from the start,
  the milestone document keeping what crosses steps. A step's design audit edits those documents in
  place, listing its findings in the commit message, and a new step found at an audit gets a spec
  of its own, listed as a scope change the owner rules on before it is implemented.
- `agent-skills`, patch: the transcript reviewer checks that everything a work's sessions
  established has a durable outcome, a reviewer's finding acted on included, and that no ruling of
  the owner is misstated. It no longer reports detail added inside a ruling. It runs once more,
  alone and last, before every merge to the main branch.
- `agent-skills`, patch: a step's design audit lists the tripwires and the `deferred` issues the
  step's planned code would fire, with `tripwires` and `issues --kind deferred`, so the owner rules
  on them before the code is written.
- `agent-skills`, patch: a load-bearing gap at an audit that is a choice among shapes stated in
  full, where the design skill's conditions for its one-round path hold, is ruled in one round with
  a default, several gaps in one question, and written into the step's documents; a gap that defeats
  what an approved thread rests on still opens a full design session. A harvest names in its commit
  each item of its row that the recording tests exclude.
- `agent-skills`, patch: the setup skill creates docs/plans/, with its `README.md`, specs/ and
  milestones/, when a project adopts the workflow, and no longer asks the owner for the plans
  directory's path.
- `agent-skills`, patch: the setup skill proposes, for a crate the project publishes, a short
  `CRATES-IO.md` as its crates.io page, named by `readme`, while its `README.md` stays
  repository-facing.
- `agent-skills`, patch: the commit that deletes a plan document cites it by its kind. A citation of
  it left in another plan is removed, and a `question` issue is opened on the citing plan.

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
