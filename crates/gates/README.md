# knowledge-architect-gates

Runs a project's merge gates as one command: every check a branch must pass before it merges, run
in order, each one's complete output kept under target/gates/, a distilled extract of what
failed, and one verdict taken from the children's exit codes alone.

A project calls it from its own maintenance binary, conventionally a crate named `xtask` run
through the cargo alias `cargo x`. The binary holds the project's gate list, usually the
recommended list of a Rust project that uses knowledge-architect, and hands it to the library with
the flags it parsed. How to set that up is part of the setup skill that knowledge-architect
installs; the library's interface is at <https://docs.rs/knowledge-architect-gates>.

## What a run produces

- **stdout**: one verdict line per gate, and for each failed gate a distilled extract, headed by
  the path of its log. A run refuses to write into a pipe outside GitHub Actions: a pipe's reader
  replaces the exit code, and a filter drops lines. Redirect to a file when the text is wanted.
- **`target/gates/<gate>.log`**: the gate's complete output, both streams, always written.
- **A slow-tests line** whenever libtest warned that a test ran over 60 seconds.
- **Exit code**: 0 exactly when every gate ran and passed. Output handling never changes it.

Under GitHub Actions, each gate is wrapped in a `::group::`, and a failed gate adds one `::error`
line naming it.

## The flags

| flag | effect |
| --- | --- |
| `--fail-fast` | stop at the first failing gate; by default every gate runs |
| `--full` | stream each gate's raw output live as well |
| `--locked` | pass `--locked` to every gate that resolves dependencies |
| `--require-rebased` | run the rebase checks: HEAD must contain the main branch |

## License

Licensed under either of the MIT license (LICENSE-MIT) or the Apache License 2.0
(LICENSE-APACHE), at your option.
