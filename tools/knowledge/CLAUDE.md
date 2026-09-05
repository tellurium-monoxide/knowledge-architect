# Knowledge checker

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says so
there, in one place, with a reason beside it. The one thing compiled in is the tool's own
directory, taken from `CARGO_MANIFEST_DIR` of the binary crate, so that the string literals of its
own source are read as data rather than as citations, per
`knowledge#checker-source-literals-are-data`. That is a fact about the tool, not about any tree.

**`git` 2.36 or newer is a hard dependency, and every invocation lives in
`knowledge@documentation/src/git.rs`.** The floor is `cat-file --batch -z`, which `commits`
reads a commit's tree with. The walk is `git ls-files` from the project root, the
ignore question a path reference asks is one `git check-ignore` batch per run, and a listing's
last-change column is one `git log`. A check may spawn nothing, so each of those is taken by the
caller and handed in. A tree with no `git`, or a project outside a worktree, is exit 2 with the
reason; an empty walk is never an answer. The decision is `knowledge#git-supplies-the-walk`.

**A test that copies a mock project runs `git init` and `git add -A` in the copy**, or the walk
is empty and the test proves nothing; `Sandbox` in `knowledge@tests/binary.rs` does it. Two
consequences a test about ignore behaviour has to choose between, because the ignore rules act on
untracked files alone:

- for a file that must be **ignored**, write the `.gitignore` before the first `git add -A`, so
  the add never tracks it. `Sandbox::seeded` is that order.
- for a file that must be **tracked and ignored**, stage it first and add the `.gitignore`
  after, so the add finds the file already in the index.

A test that deletes or rewrites a fixture file after the first add stages again, or git's listing
still names what the working tree no longer holds. The in-place tests under
`knowledge@tests/projects/` need none of this: they build under this repository's own worktree,
so a mock file is walked because this repository tracks it — which also means a new fixture file
is walked only once it is at least staged.

**A test about commit messages builds its own project.** `commits` judges a message only where
that commit's tree carries no finding, and no mock project under `knowledge@tests/projects/`
has such a tree: `planted` plants one per family on purpose and the others are migrated only as
far as an earlier piece needed. `History` in `knowledge@tests/binary.rs` writes a project out,
configures `user.name` and `user.email` in the copy's own configuration, and commits. Two
orders it has to keep, both learned by getting them wrong:

- **stage, regenerate the indexes, stage again.** The walk is git's listing, so a deleted entry
  the index still holds is still counted and `cargo knowledge index` writes the listing the tree
  no longer has.
- **`--allow-empty`** on a commit whose subject is the message rather than a tree edit.

Read `knowledge@docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
