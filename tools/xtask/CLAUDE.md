# xtask

Workflow automation. What it does and how to run it is `path@xtask@README.md`; why it is shaped
this way is `path@xtask@docs/design.md`.

**This file holds what is true of the code as it stands.**

## No test here may invoke the gates

`cargo test --workspace`, run by the `gates` subcommand, compiles and runs this crate's own
tests. A test that invokes `cargo x gates` (or `cargo run -p xtask -- gates`) therefore runs
the suite that is running it. Test the pieces — distillers, parsers, the verdict — through
their functions, and the spawn helper through short `sh -c` children.

## No gate child inherits `RUSTC_BOOTSTRAP`

The spawn helper in `path@xtask@src/run.rs` removes `RUSTC_BOOTSTRAP` from every child it
starts. Never set it on a gate's child, and never stop removing it: `1` lets nightly-gated code
compile on stable, so a verdict taken under it passes code CI rejects.
`path@xtask@tests/gates_bin.rs` asserts that a gate's child never inherits it.

## Prints go through `say` and `complain`

Every stdout and stderr write in the gates path ignores write errors, so a reader that closes
the pipe truncates output instead of panicking the run into exit 101. A bare `println!` or
`eprintln!` added here reintroduces that panic; the integration test over the built binary
catches it.
