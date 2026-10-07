# knowledge-architect

knowledge-architect keeps the design record of a project's documentation consistent with its code.
It is built for projects developed mostly by AI agents, and has two parts that work together:

- **A checker**, `klarch`. A project declares in a manifest what it records and where: its
  components, their design decisions, their goals, their open issues and tripwires. The checker
  verifies that every reference between those records resolves, that every record has the shape
  its register declares, and that the generated listings are current. Its commands list and
  display the records a session must read before it changes something.
- **An agent workflow**, a set of skills and subagent definitions the checker installs into a
  project: setting up, writing goals, designing, planning, recording decisions, tracking open
  issues, reviewing, and a retrospective.

Why it exists, and what it aims for, is `path@knowledge-architect@docs/goals.md`.

## Status

The version is 0.4.0, published on crates.io. The project stays at
0.x, with breaking changes allowed, until the owner's word, given only once its first design
discussion's open issues are settled, per `design@knowledge-architect@stays-at-zero-x`.

## Using the checker

The checker's commands, its manifest and its exit codes are described in
`path@core@README.md`. In short, from inside a project whose root holds a
`knowledge-architect.toml`:

```sh
klarch check                     # every check over the tree; the last line is the verdict
klarch commits <range>           # every commit message of a range, judged as a document
klarch issues                    # every open issue, one row each
klarch show <ref>                # one entry in full, and every reference to it
klarch index                     # regenerate the generated listings
```

In this repository, the checker built from the checkout runs as `cargo klarch <subcommand>`, and
`cargo x gates` runs every gate: formatting, the check, the commit messages, clippy and the tests.

## Repository layout

| path | what it holds |
| --- | --- |
| crates/core | the checker: package `knowledge-architect`, binary `klarch` |
| crates/agent-skills | the text the checker installs into a project |
| crates/gates | the library that runs a project's merge gates, `cargo x gates` here |
| tools/xtask | this repository's gates, `cargo x gates` |
| docs/ | the project's goals, design decisions, rejected alternatives, tripwires and open issues, and the retrospective analyses whose outcomes are not all carried out |

## License

Licensed under either of the MIT license (LICENSE-MIT) or the Apache License 2.0
(LICENSE-APACHE), at your option.
