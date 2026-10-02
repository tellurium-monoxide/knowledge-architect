# Gates — design

The library that runs a project's merge gates: every check a branch must pass before it merges,
run in order, with complete logs, distilled failures and one verdict from exit codes. It is
published so that every project using knowledge-architect runs its gates the same way. What lost
to a decision here is `path@gates@docs/rejected-alternatives.md`.

**What belongs here:** how the gates run. Which gates a project runs is that project's own; this
repository's are `design@xtask@gates-list-primary-home`.

### The gates runner is a published library, and a project keeps a small binary of its own `##gates-crate`

The runner is the package knowledge-architect-gates. A project's maintenance binary, conventionally
a crate named xtask run through a cargo alias, hands it the project root, its gate list and the
flags it parsed. The runner was shared, almost line for line, by this repository and by thaum, as a comparison of
`git -C <thaum checkout> show e98e296:tools/xtask/src/gates.rs` with this crate shows, and it serves
`goal@knowledge-architect@setup-brings-quality-tools` and `goal@gates@gates-from-a-list`: a project
adopting the workflow gets gates refined over two projects instead of writing its own. A `gates` command of the
checker's own binary lost: it would make the document checker run cargo, the linter and the
tests, and a gate list declared in the manifest cannot carry a distiller. A published binary
configured by a file lost on the same grounds, and leaves the project no place for its other
repeated tasks.

### A project holds its gate list, and the library offers a recommended one `##a-project-holds-its-gate-list`

The library runs the gates it is given and decides none of them, so a project writes only its
list, per `goal@gates@gates-from-a-list`. `rust_project` builds the
recommended list of a Rust project that uses the checker, in cost order: the rebase check behind
its flag, formatting, the document check, the commit messages of the branch, clippy, and the
tests. It takes how the project runs its checker, a package and the arguments before the
checker's command, and the main branch as git names it. A project adds, removes or reorders gates
in the list it receives. The library depends on nothing of the checker: the root finder takes the
marker file's name from the caller.

### The library owns the gates' flags and the set of distillers `##the-library-owns-the-flags`

The flags of a gates run are the library's `GatesArgs`, which derives clap's `Args`, so a project's
binary flattens them into its own command and every project's gates take the same flags. clap's
major version is therefore part of the library's interface, as it is of the core's. The distillers
are a closed set, `Distiller`: a project chooses one for each gate and cannot add its own, so every
distiller is tested here. A project that needs another distiller is a reason to add one to the
library.

### One spawn helper captures a child's two streams through two pipes `##one-spawn-helper`

`path@gates@src/process.rs` spawns a child, reads its stdout and stderr concurrently through two
pipes into one buffer, optionally mirrors them live, and holds the best-effort output helpers
every print goes through. The gates use it, and a project's own commands may: a new command calls
it and never edits it. Reading both pipes concurrently is what prevents the deadlock where the
child blocks writing a pipe nobody drains.

### Every gate runs; nothing fails fast by default `##gates-run-all`

CI stops at the first failing step because its deliverable is a red X. The gates' deliverable
is a complete work list, and a fail-fast default costs one full re-run per fixed gate. So the
default runs all gates in cost order whatever the earlier ones returned, and `--fail-fast`
is the flag CI or an impatient session opts into. The accepted cost: on a broken build,
`cargo test` repeats the compile errors clippy already reported.

### The verdict comes from child exit codes alone `##verdict-from-exit-codes`

The run exits 0 exactly when every gate ran and passed. No distiller, no log writing, no
slow-test extraction sits between a child's exit status and the run's own: a bug in output
handling can degrade what is printed, never flip a verdict. Printing itself is inside the
contract: every stdout and stderr write is best-effort, so a write error truncates the report
and never the run, the logs or the exit code.

**Outside GitHub Actions, a gates run refuses a pipe on stdout before running anything.** The
exit code is the run's product, and a pipe hands it to the reader at the far end: `| tail`
exits with tail's status, and a filter drops lines of a report that is already distilled, so
the loss is silent both ways. A terminal and a regular file pass, so `> log 2>&1` stays
available, and the complete logs under the project's target/gates/ need no `tee`.

**Under GitHub Actions the pipe is accepted.** The runner captures a step's output through a
pipe and takes the step's verdict from the shell's exit code, so no reader stands between the
run and its verdict, which is the refusal's reason. `GITHUB_ACTIONS` set to `true` is how the
tool knows; the output then stays live in the job log.

### The rebased gate runs behind a flag `##rebased-gate-behind-a-flag`

`--require-rebased` adds the gates marked as rebase checks. The recommended list holds one,
`rebased`, first: `git merge-base --is-ancestor <base> HEAD`, where the base is the main branch
as the project names it, such as origin/main. Under a fast-forward merge, the tree CI judges
is the tree the main branch receives only when the branch contains the base; a branch cut before
`main` moved is tested alone and lands on a different tree. It is first because it builds
nothing. It is behind a flag because a branch not yet rebased is normal while it is worked on;
CI passes the flag. It reads the local copy of the base, so a caller fetches first.

### Under Actions each gate is a group, and a failed one an error `##annotations-under-actions`

CI runs every gate in one step, so the job's step list no longer names the failing gate. Under
Actions the library prints GitHub workflow commands around each gate: `::group::<gate>` before it
and `::endgroup::` after its verdict line, so the job log folds per gate, and for a failed gate
one `::error title=<gate>::` line, which the job summary lists by the gate's name. Outside
Actions none of it is printed.

### Distilled stdout over complete logs `##distill-over-full-logs`

Every gate's full output — both streams — is always written to `target/gates/<gate>.log`,
under `target/` so it is ignored and leaves with the build. stdout gets one verdict line per
gate and, for each failed gate, a distilled extract headed by its log path. A distiller is named on its gate
and chooses what to print from the full text; it can be aggressively
wrong at the cost of one read of the log, which is what makes conservative distillation safe
to iterate on. Distillers start conservative: in the recommended list, fmt, check and commits pass through
whole, clippy drops cargo's progress lines, test keeps the failures and the compiler's own
diagnostics.

### No gate inherits `RUSTC_BOOTSTRAP` from its caller `##gates-scrub-rustc-bootstrap`

The spawn helper removes `RUSTC_BOOTSTRAP` from the environment of every child the library starts,
gates included, and a child that needs a value sets it explicitly. `RUSTC_BOOTSTRAP=1` makes a
whole build nightly-equivalent, so nightly-gated code that CI rejects compiles and passes: thaum's
review reproduced it with a `#![feature(test)]` doc test. libtest also reads the variable's mere
presence. A caller who exported it would turn every local gate run into that false green, and the
verdict run exists to exclude it. The test `gate_children_never_inherit_a_caller_exported_bootstrap`
in `path@xtask@tests/gates_bin.rs` asserts the scrub.
