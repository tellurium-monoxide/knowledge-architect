# Knowledge checker

How this project's knowledge is held: the Comprehensive Rules corpus, and the
documents that cite it. One binary, reached through a cargo alias so nothing has to be installed:

```sh
cargo knowledge check [--only a,b,c]     # every check, over one walk; or only these families
cargo knowledge outstanding [text]       # every tracker entry, by directory; or one in full
cargo knowledge index [--interpretations] [--lines] [--write]
cargo knowledge model                    # every observation the walk produced
cargo knowledge rules show <number> [<number> ...]   # the pinned text of a rule, shaped to be quoted
cargo knowledge rules latest|diff|fetch|bump
```

**A run prints the summary first, its findings under it, and its verdict on the last line** —
`PASSED: no findings`, or `FAILED: n findings above`. The verdict is derived from the finding list
rather than tracked beside it, so it cannot disagree with the exit code, which is `0` when there are
none and `1` when there are any. A caller scripting against a run reads the exit code; a person
reads the last line. The order matters because the summary block prints on a failing run too, so
while it came last a `| tail` showed a success-shaped report over a red tree.

**The `--only` families are the checks themselves**, one per check: `citations`, `generated`,
`components`, `slugs`, `paths`, `interpretations`, `uncovered`, `changes`, `corpus`. Eight are the
modules under `knowledge@documentation/src/check/`, and `corpus` is the integrity check over the
vendored text and its archive, which reads the filesystem rather than the model. `structure` names
every family but `citations`. A comma-separated list runs their union over the one walk, so asking
for several costs one run rather than one run each. A run prints which families it performed, and a
family that did not run prints no count of its own. The argument is `knowledge#families-are-the-checks`.

**`rules show` prints a rule as a citation is written**: `> <number> <body>`, the body entire, on
one line, from the vendored release and no other. Paste the line into a document as a blockquote,
or take the body alone for the inline form. It names the rule's subrules where it has any, because
a whole-body quote of a parent does not stand for a claim its subrule carries, and it fails the run
on a number the release does not hold rather than printing nothing.

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says so
there, in one place, with a reason beside it.

Read `knowledge@docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
