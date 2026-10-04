# xtask

Workflow automation for this repository. Each subcommand is one workflow: `changelog` writes the
copies of the root changelog that the published crates ship, and `gates` runs the project's gates
(what CI runs) through the published gates library, `path@gates@README.md`, keeps every byte they
emit, and prints only what needs acting on. The gate list in `path@xtask@src/gates.rs` is the
primary home of what the gates are, per `design@xtask@gates-list-primary-home`.

## Usage

```sh
cargo x gates                     # run the five gates, distilled output; refuses a pipe on stdout
cargo x gates --fail-fast         # stop at the first failing gate
cargo x gates --full              # additionally stream raw output live
cargo x gates --locked            # pass --locked to every gate that resolves dependencies
cargo x gates --require-rebased   # add the rebased gate: HEAD must contain origin/main
cargo x changelog                 # write the root CHANGELOG.md into every published crate
```

The gates, in cost order:

| gate | command |
| --- | --- |
| `rebased` | `git merge-base --is-ancestor origin/main HEAD`, under `--require-rebased` only |
| `fmt` | `cargo fmt --all --check` |
| `check` | `cargo run -q --release -p knowledge-architect -- check` |
| `commits` | `cargo run -q --release -p knowledge-architect -- commits origin/main..HEAD` |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `test` | `cargo test --workspace` |

## What a run produces

**Stdout must be a terminal or a file.** `gates` refuses to start when stdout is a pipe, per
`design@gates@verdict-from-exit-codes`: a pipe's reader replaces the exit code and a filter drops
lines. When the text is wanted, redirect: `cargo x gates > target/gates/run.txt 2>&1`. Under
GitHub Actions (`GITHUB_ACTIONS=true`) a pipe is accepted, since the runner reads the exit code
itself, and each gate is wrapped in `::group::` and, when it fails, one `::error` line, per
`design@gates@annotations-under-actions`.

- **stdout**: one verdict line per gate. Each failed gate adds a distilled extract, headed by
  the path of its log.
- **`target/gates/<gate>.log`**: the gate's complete output, both streams, always written.
  When the distilled view is not enough, read the log; nothing is ever only on stdout.
- **A slow-tests line** whenever libtest warned that a test ran over 60 seconds, naming the
  tests.
- **Exit code**: 0 exactly when every gate ran and passed. Output handling never changes it,
  per `design@gates@verdict-from-exit-codes` — so scripts and sessions can trust the code without
  reading anything.

## What to respect

- All gates run by default, even after a failure; the report is a complete work list. On a
  broken build the test gate repeats clippy's compile errors — pass `--fail-fast` when that
  wait is not worth it.
- The tool must be run from inside the checkout: it locates the project root by walking up to
  `knowledge-architect.toml`, the name the core's `MANIFEST_NAME` constant spells, and runs every
  gate from there.
- A closed stdout is safe: when the reader closes it, what is printed truncates and nothing
  changes about the run, the logs or the exit code.
