# Rejected alternatives — knowledge-architect

The alternatives that lost to a decision of `path@knowledge-architect@docs/design.md`, each with
what it lost to and why. An entry stays while the argument that defeated it holds.

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
