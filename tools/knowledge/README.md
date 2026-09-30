# Knowledge checker

How a project's knowledge is held: its documents, the references between them and the registers
they carry. This Component is the generic core; this repository's Comprehensive Rules half is an
extension of it, `path@rules-corpus@README.md`, per `design@thaum@knowledge-is-a-generic-core`.

**Two binaries.** `cargo knowledge` runs the binary of rules-corpus: every command below, with
the rules extension registered, reached through a cargo alias so nothing has to be installed. The
core's own binary, the package `knowledge`, runs the same commands but `rules` with no extension.
Over a manifest holding a table no extension of it claims, such as `[rules]`, it reports that
table in phase 1, rather than skip in silence what the table configures, per
`design@knowledge@an-extension-claims-its-manifest-tables`.

**It needs `git` 2.36 or newer on the path, and a project inside a git worktree.** What the tool reads is what
`git ls-files` reports from the project root, so every pattern git honours decides the walk,
nested `.gitignore` files included, and a file git tracks is read whatever the ignore rules say.
A file whose name holds a line break, or a name Windows cannot create, is read by nothing and
reported once with the reason, any line break escaped: name it in `[walk] skip-files` or in an
ignore rule to keep it.
Neither failure is silent: no binary and no worktree are both exit 2 naming the reason, never an
empty walk. `commits` reads a commit's tree through `cat-file --batch -z`, which is where the
version floor comes from. The decision is `design@knowledge@git-supplies-the-walk`.

```sh
cargo knowledge check                     every check, over one walk, in four phases
cargo knowledge show <kind>@<anchor>@<id> one recorded entry, and every reference to it
cargo knowledge issues [anchor] [--kind k] [--group g] [text …]
                                          every issue entry, one row each
cargo knowledge tripwires [anchor] [--guarding <ref>] [text …]
                                          every tripwire entry, and what each guards
cargo knowledge index                     regenerate every generated index in place
cargo knowledge model                     every observation the walk produced
cargo knowledge commits <range>           judge every commit in the range, message and tree, against its own tree
```

The `rules` commands are the rules extension's, in `path@rules-corpus@README.md`.

## Exit codes

Three, per `design@thaum@exit-code-ladder`, and the third is what makes the other two mean anything.

| code | meaning | where it comes from |
| ---- | ------- | ------------------- |
| `0` | the command ran and its subject is in order | `check` with no findings; `show` on a reference that resolves; `issues` and `tripwires` with at least one row; `index` having written every destination, or found each already current; every `model` run; `commits` with no finding, an empty range included |
| `1` | the command ran and reports a negative answer | `check` with findings; `show` on a reference that resolves to nothing; `issues` or `tripwires` with no row; `index` when a write failed after another destination was already rewritten; `commits` with a finding against a judged message or a failing tree |
| `2` | the command could not run | an unknown or invalid argument, a `show` argument that is not reference-shaped, a binary built from another checkout of the tool, which names the `cargo clean` that rebuilds it, `index` refusing a destination — a symlink, or a directory that is not there — having written nothing, `commits` on a range that does not resolve, no project above the working directory, no `git` on the path or a project outside a worktree, a stdout closed before the output was written — as `\| head` does — which ends the run silently, an input that cannot be read, an input an extension prepares that it cannot resolve |

**A caller scripting against a run reads the exit code; a person reads the last line.** Arguments
are refused before the project is located, so `--help` answers from anywhere and a mistyped
invocation costs no walk.

## `check`

**A run prints the summary first, its findings under it, and its verdict on the last line** —
`PASSED: no findings`, or `FAILED: n findings above`. The verdict is derived from the finding list
rather than tracked beside it, so it cannot disagree with the exit code. **The order is a
contract**: the summary block prints on a failing run as well as a passing one, so a reader taking
the tail of the output has to reach the verdict rather than the counts.

**A run is four phases, and it stops at the first that finds anything.** Phase 1 resolves the
manifest: a declaration the tool refuses is reported and acted on by nothing. Phase 2 reads the
tree against what the manifest declares: a file the walk could not read, a name it refuses, an
anchor or a register home that is not there, a home a walk row keeps out, a declared path that
does not exist, a file git both tracks and ignores, a symlink or a submodule. Phase 3 builds the entity table: a slug or an entry id where none may sit, or
defined twice. Each of these says the model is incomplete, and a finding computed from the model
afterwards would be unreliable in both directions, so the run prints that phase's findings, says
which phases were not judged, and exits 1. Phase 4 is every check, over the complete model: the
core's `generated`, `registers` and `references`, each a module under
`path@knowledge@documentation/src/check/`, then each registered extension's, per
`design@knowledge@an-extension-plugs-in-through-phased-hooks`. `references` judges
every `` `<kind>@<anchor>@<id>` `` reference against the entity table — a register kind against the
entries its home defines, the `path` kind against the tree — and reports the retired slug
reference form and the unanchored path shape. `registers` judges the shape of what each anchor carries: a file
register's README, index, groups and entry shapes, and a directory home's links. A check the tree
gives no input to is printed as not run rather than counted. There is no way to select a subset:
the checks cross the phases, and a run over a passing tree costs under a second. The argument is
`design@knowledge@phases-gate-the-report`.

`index`, and a writing command of an extension, run the first three phases too, and refuse with
exit 2 while one of them holds anything: an index generated over an incomplete model lists rows nobody asked for.

## `show`, `issues` and `tripwires`

The three commands that print what is recorded, given a reference or an anchor. All three read the
entity table, so what they print is what `check` resolves against.

**`show <kind>@<anchor>@<id>`** prints one entry and then every reference to it, as `file:line`:

- a file register's entry is printed **whole**, frontmatter included;
- a heading register's entry is printed as **its section** — the heading through to the next
  heading at the same level or shallower;
- `path@<anchor>@<path>` prints the walked document's text, or says the path is outside the walk.

**The two failures are different questions**: an argument that is not reference-shaped is exit 2
with the grammar named, and a reference the grammar accepts that names nothing is exit 1.

**`issues [anchor] [--kind k] [--group g] [text …]`** prints one row per issue entry — kind,
anchor, id, title, last change — sorted by kind then id. **`tripwires [anchor] [--guarding <ref>]
[text …]`** prints one row per tripwire entry — anchor, id, title, and every reference the
entry carries to what it guards — sorted by anchor then id. What a tripwire guards is an entry of
any register but `issue` and `tripwire`: a decision, a goal, or an entry of a register the
manifest declares. For both, the
text keeps the rows whose id or title contains it, case-insensitively, and **no row is exit 1**.

**The first positional argument is an anchor when something declares that name**, and text
otherwise, which is what lets one positional list mean both. A search for a word that is also an
anchor name is written with the anchor before it.

**The last-change column is git's**, taken in one `git log` for every issue instance at once. An
entry with no commit shows `uncommitted`; where git cannot answer the column shows `-`.
A date never enters a generated file, per `design@knowledge@generated-files-are-pure` — it is printed
here and nowhere else.

## `index`

Regenerates every generated file in place, from one walk: each file a registered extension
generates, and one `index.md` per file-register instance whose directory is there. It takes no flags and **writes only where the
bytes differ**, naming each file it rewrote:

```
$ cargo knowledge index
docs/rules/index.md                      rewritten
docs/rules/interpretations/index.md      already current
```

**A file-register index is a fixed shape, and its bytes are a contract**, per
`design@knowledge@a-file-register-index-is-rows`:

```markdown
**Generated — do not edit.** `cargo knowledge index`

2 entries

| kind | title |
| --- | --- |
| defect | [A thing that is broken](a-thing-that-is-broken.md) |

## a-group

| kind | title |
| --- | --- |
| todo | [Work left undone](a-group/work-left-undone.md) |
```

The columns are the register's declared metadata keys ordered by name, then the title as a link
relative to the index's own directory. Ungrouped entries come first, under no heading; rows sort by
the first metadata key and then by id. No summary and no date: a row changes on create, delete,
retitle, regroup and a metadata change, and on nothing else.

**Every generated file is outside the walk by construction** — the tool derives the set, each file
an extension generates and one index per file-register instance, from the manifest, so no `[walk] skip-files` row names one
and none can be created inside the walk.

Running it to look therefore costs nothing, not even an mtime. **Whether a generated file is
current is not this command's question** — that is `cargo knowledge check`, whose `generated` check
is a gate and names the first line at which the committed file and the regenerated one disagree.
Both halves are `design@knowledge@generated-files-are-pure`.

## `model`

Every observation the walk and the scanner produced, one per line, as `file`, `line`, `kind`,
`value`, tab-separated on stdout; the document and observation counts go to stderr, so redirecting
stdout gives a file that is only observations.

## Registers

**Four registers are compiled in** — `design`, `goal`, `tripwire` and `issue` — because they are
what the word component means to this tool. A project declares further ones in
`path@thaum@knowledge.toml`:

```toml
[locations.rules]              # a directory carrying a subset of the registers
path = "docs/rules"
registers = ["issue", "tripwire", "interpretation"]

[registers.interpretation]
scope = "opt-in"               # `component` (every component carries it) or `opt-in`
shape = "file"                 # `file` (one file per entry) or `heading` (slugs in a home)
dir = "interpretations"        # the basename of the home; defaults to the register's name
                               # `level = 3` on a heading register: the level its entries sit at
sections = ["Rules", "Reading", "Consequences"]

[registers.interpretation.metadata.status]
values = ["settled", "interpretation", "ambiguous", "cr-gap"]
```

A **heading register** keeps its entries as slugged headings, in `<dir>.md` or in `<dir>/` behind
a `README.md` that links every subdocument. It declares `level`, from 2 to 6: every heading at that
level in its home is an entry and carries a slug, and a slug at any other level defines nothing,
per `design@knowledge@an-entry-is-a-heading-at-the-register-level`. The built-in levels are 3 for
`design` and 2 for `goal` and `tripwire`. A file register takes no `level`. A **file register** keeps one file per entry under
`<dir>/`, beside a hand-written `README.md`, a generated `index.md` and an optional
`register.toml` declaring the group subdirectories. An entry opens with frontmatter carrying each
declared metadata key, then a level-one title, then the declared sections.

`[registers.issue]` accepts `kinds` and nothing else; the other three built-in registers accept
nothing, `level` included. The arguments are `design@knowledge@registers-are-declared` and
`design@knowledge@a-file-register-is-a-directory-of-entries`.

## Commit messages

**A commit message is a document under the regime.** It is parsed as one markdown document —
subject line, blank line, body — and every rule runs over it: every
`` `<kind>@<anchor>@<id>` `` reference resolves, and each registered extension judges it against
its commit's tree. The argument is
`design@knowledge@a-commit-message-is-a-document`.

```sh
cargo knowledge commits HEAD~1..HEAD        # the commit just made
cargo knowledge commits origin/main..HEAD   # the branch's own commits
```

`commits` reads everything from each commit's own tree through git objects — the manifest, the
documents, the generated indexes, every file an extension reads — so a message is judged against the tree it
was written against, byte for byte and cleaned of nothing. **A commit whose tree does not load or
carries findings fails the run**, the range's last commit included, with its tree's findings
named by the commit and the file, and its message is still judged where its tree reached the last
phase; the summary block counts judged and failed commits. The checker that judges every commit is the one built from the working tree, so a branch
that makes it stricter puts that change in its first commit or is squashed before review. A
message's references resolve against its commit's tree **or its first parent's**, which is what
lets a commit that closes an issue name it; a parent whose manifest fails phase 1 is not read, and
serves as no parent. No hook judges a message before the commit exists:
the range is run after each commit, and a finding in the newest commit is repaired by an amend.

`check` reads no history, and the range is always explicit. `cargo x gates` runs
`commits origin/main..HEAD` as a gate.

**The range need not end at the checkout.** Any range `git rev-list` resolves is judged, commit by
commit, and where HEAD sits changes nothing. The uses a range serves:

| use | range |
| --- | --- |
| the commit just made, before it is amended or built on | `HEAD~1..HEAD` |
| a branch's own commits, before review and merge | `<base>..HEAD` |
| CI over a pull request, which checks out the branch's head | `<base>..HEAD` |
| a pre-push hook: git gives it, per pushed ref, the local and the remote sha, and with `git push origin other-branch` the local sha is not HEAD | `<remote sha>..<local sha>` |
| a branch that is not checked out, reviewed locally | `<base>..<branch>` |
| an audit of old history, such as the commits made before a manifest migration | `<first>^..<last>` |

`<base>` is the branch the work merges into, as the remote holds it.

Two things are still read from the working tree, whatever the range: the binary that judges, and
git's ignore rules, which `git check-ignore` answers for the files on disk only.

## What to respect

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`path@thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so one binary checks this repository and a mock project under
`path@knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says
so there, in one place, with a reason beside it. What is compiled in is the directory of each
Component a binary's libraries belong to, so that the string literals of the tool's own source are
read as data, per
`design@knowledge@checker-source-literals-are-data`; that is a fact about the tool, not about any tree.

Read `path@knowledge@docs/design.md` before changing how it works.
