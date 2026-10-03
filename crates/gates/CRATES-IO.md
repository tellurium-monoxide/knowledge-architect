# knowledge-architect-gates

Runs a project's merge gates as one command: every check a branch must pass before it merges, run
in order, each one's complete output kept on disk, a distilled extract of what failed, and one
verdict taken from the exit codes alone.

A project calls it from its own maintenance binary, conventionally a crate named xtask run through
the cargo alias `cargo x`. The binary holds the project's gate list, usually the recommended list
of a Rust project that uses knowledge-architect, and hands it to the library.

## Documentation

- The library API: [docs.rs](https://docs.rs/knowledge-architect-gates).
- The [repository README of this crate](https://github.com/tellurium-monoxide/knowledge-architect/blob/main/crates/gates/README.md).
- The project: [the repository](https://github.com/tellurium-monoxide/knowledge-architect).
