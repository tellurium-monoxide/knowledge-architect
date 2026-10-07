# xtask

Workflow automation. What it does and how to run it is `path@xtask@README.md`; why it is shaped
this way is `path@xtask@docs/design.md`. The gates run through the published library,
`path@gates@CLAUDE.md`, whose contracts hold for every gate this tool runs.

**This file holds what is true of the code as it stands.**

## Prints go through `say` and `complain`

The library's output helpers, `knowledge_architect_gates::process::{say, complain}`, ignore write
errors, so a reader that closes the pipe truncates output instead of panicking the run into exit
101. xtask prints through them too, as its `abort` does; a bare `println!` or `eprintln!` added
here reintroduces that panic. This is a restatement; its home is
`design@gates@verdict-from-exit-codes`.

## No test here may invoke the gates

`cargo test --workspace`, run by the `gates` subcommand, compiles and runs this crate's own
tests. A test that invokes `cargo x gates` (or `cargo run -p xtask -- gates`) therefore runs
the suite that is running it. `path@xtask@tests/gates_bin.rs` runs the built binary over a fake
project, whose gates are stubbed, and asserts the library's contracts through it: the pipe
refusal, the run-all default, the scrubbed `RUSTC_BOOTSTRAP`, `--locked`, the rebase check and
the annotations under Actions.

## The example is another project's main

`path@xtask@examples/setup_snippet.rs` compiles the maintenance crate's main that the setup skill
shows. That main is written for an adopting project, so the conventions of this file, `say` and
`complain` among them, do not apply to it.
