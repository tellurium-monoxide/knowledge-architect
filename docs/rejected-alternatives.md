# Rejected alternatives — knowledge-architect

The alternatives that lost to a decision of `path@knowledge-architect@docs/design.md`, each with
what it lost to and why. An entry stays while the argument that defeated it holds.

**A checked form for a commit, resolved by its subject** — lost to
`design@knowledge-architect@a-needed-unchecked-pointer-names-its-gap`. `live`. A commit of the
branch is named by its subject, and nothing checks that a commit with that subject exists. A
checked form would read the history on every run, a cost that grows with the age of the project,
to check names that git itself does not make permanent. A check bounded to the range `commits`
judges fails too: a later branch's documents and messages cite, by subject, commits that are no
longer in its range, so the check must either load every commit or issue no finding on them, and it
then only confirms citations that are already correct. Its value is low, and a subject written in
prose escapes no check: it names history, not the tree.

**The checker's crate in crates/knowledge-architect** — lost to
`design@knowledge-architect@repo-layout`. `live`. Refuted by a run, not by argument: in a scratch project
named knowledge-architect with a component at that path, the check stopped in phase 1, because the
directory's basename and the project's name are the same anchor. A crate directory is named by its
role.

**One published package holding both the checker and the installed text** — lost to
`design@knowledge-architect@two-crates`. `live`. The owner asked for a single package. cargo packages only
the files under a package directory, so a single package holding both components must be rooted
at crates/. That makes crates/ a package holding two components, which mixes a crate with the
directories around it, and a third crate could not live under it. Users name one crate either way,
because cargo fetches a dependency without being asked.

**The agent workflow as a plugin published on a marketplace** — lost to
`design@knowledge-architect@binary-bundles-workflow`. `live`. A Claude Code plugin is how the
harness distributes skills, and it was designed in full: a marketplace in this repository, the
plugin pinned to each release's tag and sha, a hook printing the primer and warning on a version
mismatch. It loses on three facts. Two artifacts, the crate and the plugin, had to be released in
step, and a release had to edit the marketplace after tagging. The harness's plugin cache is per
user, so two projects on one machine needing two versions was never shown to work. And a version
mismatch could only be warned about, where an installed file can be checked. A custom marketplace
gave no discoverability in exchange.

**Tying the build to `CARGO_MANIFEST_DIR`, which every crate already reads** — lost to
`design@knowledge-architect@a-build-is-tied-to-its-checkout`. `live`. Refuted by a run on cargo
1.98.0: in two copies of a one-crate workspace sharing one `CARGO_TARGET_DIR`, a binary printing
`env!("CARGO_MANIFEST_DIR")` was not rebuilt in the second copy and printed the first copy's
directory. Cargo does not track a variable it sets itself.

**The crates' changelog copies written at packaging time and kept out of the tree** — lost to
`design@knowledge-architect@the-changelog-ships-in-every-crate`. `live`. Refuted by a run on cargo
1.98.0: a git-ignored copy that `include` lists makes `cargo package --list` exit 101 as an
uncommitted change, and an absent one is left out of the package with no error.
