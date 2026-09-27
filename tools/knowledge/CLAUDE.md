# Knowledge checker

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`path@thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`path@rules-corpus@tests/projects/` with no special case anywhere. A path that should not be checked says so
there, in one place, with a reason beside it. What is compiled in is the directory of each
Component a binary's libraries belong to — `documentation::component_dir` for the core,
`citations::component_dir` for the rules extension, each the parent of its crate's
`CARGO_MANIFEST_DIR` — so that the string literals of the tool's own source are read as data
rather than as citations, per `design@knowledge@checker-source-literals-are-data`. That is a fact
about the tool, not about any tree.

**`git` 2.36 or newer is a hard dependency, and every invocation lives in
`path@knowledge@documentation/src/git.rs`.** The floor is `cat-file --batch -z`, which `commits`
reads a commit's tree with. The walk is `git ls-files` from the project root, the
ignore question a path reference asks is one `git check-ignore` batch per run, and a listing's
last-change column is one `git log`. A check may spawn nothing, so each of those is taken by the
caller and handed in. A tree with no `git`, or a project outside a worktree, is exit 2 with the
reason; an empty walk is never an answer. The decision is `design@knowledge@git-supplies-the-walk`.

**A test that copies a mock project runs `git init` and `git add -A` in the copy**, or the walk
is empty and the test proves nothing; `Sandbox` in `path@knowledge@tests/binary.rs` does it. Two
consequences a test about ignore behaviour has to choose between, because the ignore rules act on
untracked files alone:

- for a file that must be **ignored**, write the `.gitignore` before the first `git add -A`, so
  the add never tracks it. `Sandbox::seeded` is that order.
- for a file that must be **tracked and ignored**, stage it first and add the `.gitignore`
  after, so the add finds the file already in the index.

A test that deletes or rewrites a fixture file after the first add stages again, or git's listing
still names what the working tree no longer holds. The in-place tests under
`path@knowledge@tests/projects/` need none of this: they build under this repository's own worktree,
so a mock file is walked because this repository's listing holds it. **Staging is not what puts
it there.** The listing is `--cached --others --exclude-standard`, so a new fixture file is
walked the moment it exists, tracked or not; what removes one is an ignore rule, and what a
deletion needs is staging, or the path stays in the listing with no bytes behind it.

**Five mock projects for the core binary, and each is for one thing.** They declare no table the
core does not own. The head comment of each `knowledge.toml` says what its project exercises,
and that comment is where a session decides which one to touch. In short: `planted` is detection
in the last phase, one defect per core check, and it is clean through the phases before it so a
run reaches them; `unsound` is the phase gate, one defect per assertion of phase 2 and, behind
them, the definition-site defects of phase 3, so a run over it stops at phase 2; `dirhome` is
conformance, every core check running and finding nothing; `minimal` is the walk, the
exclusions, a location and a declared register; `core` is the smallest conformant project, and
the tests of the core binary's refusal of a table it does not own. **Plant a phase-4 defect in
`planted`, a phase-2 or phase-3 one in `unsound`, and nowhere else.** The core binary run from
`dirhome`, `minimal` or `core` reports no finding; anything else it reports there is a defect in
the tool or in the fixture. The rules extension's mocks, with a corpus, are
`path@rules-corpus@tests/projects/`.

**A test about commit messages builds its own project.** `commits` fails every commit before the
tip whose tree carries a finding, and exits 2 when the tip's does, so each commit a test makes
has a tree with none, except the one it plants. `dirhome` is the one mock over which every family runs
and finds nothing, so a copy of it could serve as a base; `History` in
`path@knowledge@tests/binary.rs` writes a project out
anyway, because each test states the exact findings the commits it makes carry and a mock's
contents are shared with every other test over it. `History` writes the project out,
configures `user.name` and `user.email` in the copy's own configuration, and commits. Two
orders it has to keep, both learned by getting them wrong:

- **stage, regenerate the indexes, stage again.** The walk is git's listing, so a deleted entry
  the index still holds is still counted and `cargo knowledge index` writes the listing the tree
  no longer has.
- **`--allow-empty`** on a commit whose subject is the message rather than a tree edit.

**A check that produces something another check reads is a phase, not a check.** A run is
four phases, per `design@knowledge@phases-gate-the-report`, and every check of the last phase
consumes the entity table and produces nothing another check reads; that is what lets a
finding be classified by where it is produced. A new check whose findings another check would
read goes before its consumers, in `check::foundation`, and stops the run when it finds
anything. A new finding about what the model could not read — a home, a file, a declaration —
goes in `check::tree` or in resolution, never in a module of the last phase, or nothing gates
it.

Read `path@knowledge@docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
