# Gates

The library that runs a project's merge gates. What it does for a user is
`path@gates@README.md`; why it is shaped this way is `path@gates@docs/design.md`.

**This file holds what is true of the code as it stands.**

## No test here may invoke a gates run over a real project

`cargo test --workspace`, run by the test gate, compiles and runs this crate's own tests. A test
that runs the gates over this repository runs the suite that is running it. Test the pieces
(distillers, the verdict, the list, the argument insertion) through their functions, and the
spawn helper through short `sh -c` children. The whole binary is tested by this repository's xtask,
`path@xtask@tests/gates_bin.rs`, over a fake project.

## No gate child inherits `RUSTC_BOOTSTRAP`

The spawn helper in `path@gates@src/process.rs` removes `RUSTC_BOOTSTRAP` from every child it
starts. Never set it on a gate's child, and never stop removing it: `1` lets nightly-gated code
compile on stable, so a verdict taken under it passes code CI rejects, per
`design@gates@gates-scrub-rustc-bootstrap`.

## Prints go through `say` and `complain`

Every stdout and stderr write in the gates path ignores write errors, so a reader that closes
the pipe truncates output instead of panicking the run into exit 101. A bare `println!` or
`eprintln!` added here reintroduces that panic.
