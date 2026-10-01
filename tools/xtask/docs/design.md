# xtask — design

Workflow automation for this repository. One subcommand per workflow; the only one is `gates`,
which runs what CI runs and distills the output to what needs acting. A workflow belongs here
when it is a repository operation a session runs often and gets wrong by hand.

How its arguments parse is not a decision of this tool's: it parses through clap and tests its
declaration, as `design@core@arguments-parse-through-clap` argues for the checker's binaries.

### One module per subcommand, one shared spawn helper `##one-module-per-subcommand`

A subcommand is one module owning everything it is, plus one variant in `path@xtask@src/main.rs`'s
command enum. The only shared code is `path@xtask@src/run.rs`: spawn a child, capture both streams,
optionally mirror them live, and the best-effort output helpers every subcommand prints
through. Adding a workflow adds its module and its variant and edits
nothing else; a second piece of shared code has to earn its place the way `run.rs` did, by
being needed by every subcommand. This is what keeps later workflows independent of each other
and of `gates`.

### The gate list lives here, and CI runs it `##gates-list-primary-home`

The list in `path@xtask@src/gates.rs` is the one home of what the project's gates are. Root
`CLAUDE.md`'s "Verify mechanically" section points at `cargo x gates`, and
`path@knowledge-architect@.github/workflows/ci.yml` runs the tool in one step, on every push to a ready pull
request: `cargo --locked x gates --locked --fail-fast --require-rebased --full`. A list restated as
workflow steps drifts from this one, and a gate added to one list and not the other makes a
local run pass what CI fails, or the reverse. What the per-step layout gave, the failing gate's
name in the job UI, comes back through `design@xtask@annotations-under-actions`.

**Where CI and a local run differ, the difference is a flag.** `--locked`: locally a
legitimately updated `Cargo.lock` must not fail a gate, while in CI lock drift is exactly what
must fail. The tool's flag reaches its children alone: the `x` alias resolves the workspace
before the tool starts and rewrites a drifted lock, so CI also passes cargo's own `--locked`
ahead of the alias. `--fail-fast`: CI's deliverable is a verdict, a local run's is the complete work list,
per `design@xtask@gates-run-all`. `--require-rebased`, per `design@xtask@rebased-gate-behind-a-flag`.
`--full`: under Actions nothing else streams, since the announce line is written to a terminal
alone, and a hung gate must show which one it is.

### CI builds and tests the branch's tip alone `##ci-builds-the-tip-alone`

`commits` judges every commit's message and tree, and nothing compiles or tests a commit before
the tip: the suite takes minutes per run on the runner, and a range holds several commits. A
commit before the tip that does not compile passes CI, and is found only by whoever builds it
later.

### Every gate runs; nothing fails fast by default `##gates-run-all`

CI stops at the first failing step because its deliverable is a red X. This tool's deliverable
is a complete work list, and a fail-fast default costs one full re-run per fixed gate. So the
default runs all gates in cost order whatever the earlier ones returned, and `--fail-fast`
is the flag CI or an impatient session opts into. The accepted cost: on a broken build,
`cargo test` repeats the compile errors clippy already reported.

### The verdict comes from child exit codes alone `##verdict-from-exit-codes`

The run exits 0 exactly when every gate ran and passed. No distiller, no log writing, no
slow-test extraction sits between a child's exit status and the tool's own: a bug in output
handling can degrade what is printed, never flip a verdict. Printing itself is inside the
contract: every stdout and stderr write is best-effort, so a write error truncates the report
and never the run, the logs or the exit code.

**Outside GitHub Actions, `gates` refuses a pipe on stdout before running anything, and this
subcommand alone.** The
exit code is the run's product, and a pipe hands it to the reader at the far end: `| tail`
exits with tail's status, and a filter drops lines of a report that is already distilled, so
the loss is silent both ways. A terminal and a regular file pass, so `> log 2>&1` stays
available, and the complete logs under `path@knowledge-architect@target/gates/` need no `tee`.

**Under GitHub Actions the pipe is accepted.** The runner captures a step's output through a
pipe and takes the step's verdict from the shell's exit code, so no reader stands between the
run and its verdict, which is the refusal's reason. `GITHUB_ACTIONS` set to `true` is how the
tool knows; the output then stays live in the job log.

### The rebased gate runs behind a flag `##rebased-gate-behind-a-flag`

`--require-rebased` adds a gate named `rebased`, first in the list:
`git merge-base --is-ancestor origin/main HEAD`. Under a fast-forward merge, the tree CI judges
is the tree `main` receives only when the branch contains origin/main; a branch cut before
`main` moved is tested alone and lands on a different tree. It is first because it builds
nothing. It is behind a flag because a branch not yet rebased is normal while it is worked on;
CI passes the flag, and the merge predicate in root `CLAUDE.md` asks the same ancestry
question with git before the fast-forward. It reads the local origin/main, so a caller fetches
first; CI's full-depth checkout has.

### Under Actions each gate is a group, and a failed one an error `##annotations-under-actions`

CI runs every gate in one step, so the job's step list no longer names the failing gate. Under
Actions the tool prints GitHub workflow commands around each gate: `::group::<gate>` before it
and `::endgroup::` after its verdict line, so the job log folds per gate, and for a failed gate
one `::error title=<gate>::` line, which the job summary lists by the gate's name. Outside
Actions none of it is printed. The logs themselves reach the reader as the workflow's
`gate-logs` artifact, uploaded when the job fails.

### Distilled stdout over complete logs `##distill-over-full-logs`

Every gate's full output — both streams — is always written to `target/gates/<gate>.log`,
under `target/` so it is ignored and leaves with the build. stdout gets one verdict line per
gate and, for each failed gate, a distilled extract headed by its log path. A distiller is a
function beside its gate that chooses what to print from the full text; it can be aggressively
wrong at the cost of one read of the log, which is what makes conservative distillation safe
to iterate on. Distillers start conservative: fmt, check and commits pass through whole, clippy
drops cargo's progress lines, test keeps the failures and the compiler's own diagnostics.

### The root finder spells the manifest's name with the core's constant `##root-finder-uses-the-core-constant`

The gates run from the project root, found by walking up to the first directory holding the
checker's manifest. xtask depends on the core for one item, `MANIFEST_NAME`, so the file name is
spelled in one place and a rename of the manifest cannot leave xtask looking for the old one. The
cost is that building xtask builds the core, which the check gate builds anyway.

### No gate inherits `RUSTC_BOOTSTRAP` from its caller `##gates-scrub-rustc-bootstrap`

The spawn helper removes `RUSTC_BOOTSTRAP` from the environment of every child the tool starts,
gates included, and a child that needs a value sets it explicitly. `RUSTC_BOOTSTRAP=1` makes a
whole build nightly-equivalent, so nightly-gated code that CI rejects compiles and passes: thaum's
review reproduced it with a `#![feature(test)]` doc test. libtest also reads the variable's mere
presence. A caller who exported it would turn every local gate run into that false green, and the
verdict run exists to exclude it. The test `gate_children_never_inherit_a_caller_exported_bootstrap`
asserts the scrub.
