---
kind: observation
---
# A gates run that could not start exits 1, as a failed gate does

## Summary

When a gates run cannot start (no working directory, no project root above it, a pipe on stdout
outside CI), it exits 1: the library's `abort` for the pipe refusal, and a project's binary for
the rest, as this repository's xtask does. A run where a gate failed also exits 1. No recorded
decision says which exit-code contract binds the gates.

## Details

### What

`abort` in `path@gates@src/gates.rs` and in `path@xtask@src/gates.rs` returns
`ExitCode::FAILURE`. In thaum, xtask's design said it followed thaum's exit-code ladder, under which
a command that could not run exits 2; that pointer did not survive the extraction, because the
ladder of this repository, `design@core@exit-code-ladder`, binds only the checker's commands.
Whether the gates should follow it is not established: CI reads only "zero or not", and no caller
here distinguishes 1 from 2 today.

It reproduces: `cargo x gates | cat` outside CI prints the pipe refusal and exits 1.

### Why it matters

A caller that wants to tell "a gate failed" from "the gates never ran" cannot. It strains
`design@gates@verdict-from-exit-codes` only if a consumer of the exit code appears.

### What would close it

A decision that the gates follow the checker's ladder (and a run that could not start exits 2),
or that they keep one failure code and say why.
