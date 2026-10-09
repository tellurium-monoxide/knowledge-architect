# xtask — design

Workflow automation for this repository. One subcommand per workflow: `changelog`, which writes the
copies of the root changelog the published crates ship, per
`design@knowledge-architect@the-changelog-ships-in-every-crate`, and `gates`, which runs this
repository's gate list through the published gates library, `path@gates@docs/design.md`. A workflow
belongs here when it is a repository operation a session runs often and gets wrong by hand.

How its arguments parse is not a decision of this tool's: it parses through clap and tests its
declaration, as `design@core@arguments-parse-through-clap` argues for the checker's binaries.

### One module per subcommand, sharing only what every subcommand needs `##one-module-per-subcommand`

A subcommand is one module owning everything it is, plus one variant in `path@xtask@src/main.rs`'s
command enum. Code is shared only when every subcommand needs it. What that admits today: the gates
library's output helpers, `say` and `complain`, which `design@gates@one-spawn-helper` keeps in its
process module; its `project_root`; and the core's `MANIFEST_NAME`, per
`design@xtask@root-finder-uses-the-core-constant`. Adding a workflow adds its module and its
variant and edits nothing else. This is what keeps later workflows independent of each other and
of `gates`.

### The gate list lives here, and CI runs it `##gates-list-primary-home`

The list in `path@xtask@src/gates.rs` is the one home of what the project's gates are: the
recommended list of the gates library, `design@gates@a-project-holds-its-gate-list`, with this
repository's own package as the checker. Root
`CLAUDE.md`'s "Verify mechanically" section points at `cargo x gates`, and
`path@knowledge-architect@.github/workflows/ci.yml` runs the tool in one step, on every push to a ready pull
request: `cargo --locked x gates --locked --fail-fast --require-rebased --full`. A list restated as
workflow steps drifts from this one, and a gate added to one list and not the other makes a
local run pass what CI fails, or the reverse. The job UI names the failing gate through
`design@gates@annotations-under-actions`, and the logs reach the reader as the workflow's
`gate-logs` artifact, uploaded when the job fails.

**Where CI and a local run differ, the difference is a flag.** `--locked`: locally a
legitimately updated `Cargo.lock` must not fail a gate, while in CI lock drift is exactly what
must fail. The tool's flag reaches its children alone: the `x` alias resolves the workspace
before the tool starts and rewrites a drifted lock, so CI also passes cargo's own `--locked`
ahead of the alias. Re-taken by deleting one package block from `Cargo.lock`: `cargo x --help`
exits 0 and restores the block, and `cargo --locked x --help` exits 101. `--fail-fast`: CI's deliverable is a verdict, a local run's is the complete work list,
per `design@gates@gates-run-all`. `--require-rebased`, per `design@gates@rebased-gate-behind-a-flag`: the merge predicate in root
`CLAUDE.md` asks the same ancestry question with git before the merge, and CI's full-depth
checkout has fetched origin/main before the gate reads it.
`--full`: under Actions nothing else streams, since the announce line is written to a terminal
alone, and a hung gate must show which one it is.

### CI builds and tests the branch's tip alone `##ci-builds-the-tip-alone`

`commits` judges every commit's message and tree, and nothing compiles or tests a commit before
the tip: the suite takes minutes per run on the runner, and a range holds several commits. The
job's step times re-take it, `gh run view <run id> --json jobs --jq '.jobs[].steps[] | [.name, .startedAt, .completedAt]'`,
where the `gates` step holds every gate. A
commit before the tip that does not compile passes CI, and is found only by whoever builds it
later.

### The root finder spells the manifest's name with the core's constant `##root-finder-uses-the-core-constant`

A subcommand runs from the project root, found by walking up to the first directory holding the
checker's manifest. xtask hands the gates library's `project_root` that name, and depends on the
core for it, `MANIFEST_NAME`, so the file name is spelled in one place and a rename of the
manifest cannot leave xtask looking for the old one. The subcommands that find the root today are
gates and changelog. The cost is that building xtask builds the core, which the check gate builds
anyway.
