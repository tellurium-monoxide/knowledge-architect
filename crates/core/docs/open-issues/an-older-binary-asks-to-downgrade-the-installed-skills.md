---
kind: defect
---
# An older binary run over newer installed skills tells the user to downgrade them

## Summary

A binary older than the one that installed a project's agent skills reports every installed file
that changed between the two versions as differing, and its repair is `install-agent-skills`,
which writes the older text over the newer. Nothing tells the user that the binary is the stale
side. Without an agent harness nothing detects the mismatch at all.

## Details

### What

Reproduced in an adoption trial on rust-lang/log at commit 27e3cf7a: the skills were installed by
the published 0.2.0, and the published 0.1.0 was then run over the tree with `check`. It stopped at
phase 2 with 14 findings, each `<file>  this installed file differs from what version 0.1.0
ships for this project`, each with the repair `run install-agent-skills`. Following the repair
replaces the 0.2.0 text with the 0.1.0 text.

The mechanism, read in `path@core@src/check/agents.rs`: `check` compares each installed file with
the bytes the running binary's version ships, `env!("CARGO_PKG_VERSION")`, and knows no other
version. The manifest declares no version, per `design@core@nothing-of-a-project-is-compiled-in`:
the trial added `version` under `[project]` and the manifest refused the key. The pin is held by
the project's own install, a maintenance crate's `=<version>` dependency or a `cargo install
--version`, per `design@agent-skills@exact-pin`, and the binary does not read it.

Two cases follow, the second not reproduced (`assumption`):

- under `harness = []` no installed file is compared, so a binary of any version runs silently;
- two versions whose shipped text is identical cannot be told apart by the comparison, so a
  binary whose checks differ from the pinned one's passes.

A related observation, from reading `path@core@src/build_origin.rs` and not run (`assumption`): a
binary installed with `cargo install` and run inside a checkout of this repository is not refused
by `refuse_a_foreign_build`, since the compiled directory ends in
`knowledge-architect-<version>` and no trailing run of it matches the core's directory; only a developer
of this repository meets it, and the cargo alias builds from the checkout.

### Why it matters

The repair the check gives moves the project backwards, against the pin it chose, which
`goal@knowledge-architect@any-project-can-adopt-it` states: "A project pins the version it uses,
and moves to a new one when it chooses." The failure is common on a machine with several
projects, or after a pin moved and the binary was not rebuilt.

### What would close it

A run whose binary is not the version the project pins says so, names which side is older, and
never offers a repair that moves the installed text backwards; and a test that a 0.1.0-shaped
binary over a newer pin is refused with that message. Two shapes are under discussion with the
owner: the install writes its version into the installed set and the check compares it, which
needs no new place to bump but works only under an agent harness; or the manifest pins the
version and every command refuses a mismatch, which covers every project at the cost of one more
place to edit when the pin moves.
