# Knowledge checker

How this project's knowledge is held: the Comprehensive Rules corpus, and the documents that cite
it. One binary, reached through a cargo alias so nothing has to be installed.

**It needs `git` 2.36 or newer on the path, and a project inside a git worktree.** What the tool reads is what
`git ls-files` reports from the project root, so every pattern git honours decides the walk,
nested `.gitignore` files included, and a file git tracks is read whatever the ignore rules say.
A file whose name holds a line break is read by nothing and reported once, with the break
escaped: name it in `[walk] skip-files` or in an ignore rule to keep it.
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
cargo knowledge commit-message <file>     judge one message against the index's tree
cargo knowledge commits <range>           judge every commit in the range, message and tree, against its own tree
cargo knowledge hook install [--force]    point this clone at the committed hooks
cargo knowledge hook status               whether this clone judges a message before it is written
cargo knowledge rules show <number> …     the pinned text of a rule, shaped to be quoted
cargo knowledge rules latest              is a newer rules release published?
cargo knowledge rules diff --old <date> --new <date>
                                          what moved, filtered to what this project cites
cargo knowledge rules fetch [<date>]      fetch a release and repin to it
cargo knowledge rules bump <date>         archive, fetch, reindex, draft the changelog
```

## Exit codes

Three, per `design@thaum@exit-code-ladder`, and the third is what makes the other two mean anything.

| code | meaning | where it comes from |
| ---- | ------- | ------------------- |
| `0` | the command ran and its subject is in order | `check` with no findings; `rules latest` up to date; `rules diff` with no change; `rules show` on numbers the release holds; `rules fetch` and `rules bump` having done what they name; `show` on a reference that resolves; `issues` and `tripwires` with at least one row; `index` having written every destination, or found each already current; every `model` run; `commit-message` and `commits` with no finding, an empty range included; `hook install` having set `core.hooksPath`; `hook status` on a clone that runs the committed hook |
| `1` | the command ran and reports a negative answer | `check` with findings; `rules diff` with changes; `rules show` on a number the release does not hold; `show` on a reference that resolves to nothing; `issues` or `tripwires` with no row; `index` when a write failed after another destination was already rewritten; `commit-message` or `commits` with a finding against a judged message; `hook status` on a clone that does not run it |
| `2` | the command could not run | an unknown or invalid argument, a `show` argument that is not reference-shaped, `index` refusing a destination — a symlink, or a directory that is not there — having written nothing, `hook install` refusing a `core.hooksPath` that names something else or a script path that is not a file, having written nothing, `hook status` where git cannot run, `commits` on a range that does not resolve or whose tip carries a failing tree, no project above the working directory, no `git` on the path or a project outside a worktree, a stdout closed before the output was written — as `\| head` does — which ends the run silently, an input that cannot be read, a release that cannot be resolved — a pin neither vendored nor archived that the network does not answer — `rules latest` when it cannot produce a comparison — the extractor matched nothing, or the published date is earlier than the pinned one |

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
which phases were not judged, and exits 1. Phase 4 is every check, over the complete model:

`citations`, `generated`, `registers`, `references`, `uncovered`, `changes`, `corpus`, `regime`.

Seven are modules under `path@knowledge@documentation/src/check/`. `corpus` is the integrity check
over the vendored text and its archive, which reads the filesystem rather than the model, and
`changes` reads the changelog; both are run by the binary beside `check::run`. `references` judges
every `` `<kind>@<anchor>@<id>` `` reference against the entity table — a register kind against the
entries its home defines, the `path` kind against the tree — and reports the two retired forms and
the unanchored path shape. `registers` judges the shape of what each anchor carries: a file
register's README, index, groups and entry shapes, and a directory home's links. A check the tree
gives no input to is printed as not run rather than counted. There is no way to select a subset:
the checks cross the phases, and a run over a passing tree costs under a second. The argument is
`design@knowledge@phases-gate-the-report`.

`index`, `rules bump` and the `commit-msg` hook run the first three phases too, and refuse with
exit 2 while one of them holds anything: an index generated over an incomplete model lists rows
nobody asked for, and a message judged against an incomplete entity table is judged against
nothing.

## `show`, `issues` and `tripwires`

The three commands that print what is recorded, given a reference or an anchor. All three read the
entity table, so what they print is what `check` resolves against.

**`show <kind>@<anchor>@<id>`** prints one entry and then every reference to it, as `file:line`:

- a file register's entry is printed **whole**, frontmatter included;
- a heading register's entry is printed as **its section** — the heading through to the next
  heading at the same level or shallower. A slug in a table cell is shown with the section holding
  the table, because a cell means nothing without it;
- `path@<anchor>@<path>` prints the walked document's text, or says the path is outside the walk.

**The two failures are different questions**: an argument that is not reference-shaped is exit 2
with the grammar named, and a reference the grammar accepts that names nothing is exit 1.

**`issues [anchor] [--kind k] [--group g] [text …]`** prints one row per issue entry — kind,
anchor, id, title, last change — sorted by kind then id. **`tripwires [anchor] [--guarding <ref>]
[text …]`** prints one row per tripwire entry — anchor, id, title, and every
`` `design@<anchor>@<id>` `` reference the entry carries — sorted by anchor then id. For both, the
text keeps the rows whose id or title contains it, case-insensitively, and **no row is exit 1**.

**The first positional argument is an anchor when something declares that name**, and text
otherwise, which is what lets one positional list mean both. A search for a word that is also an
anchor name is written with the anchor before it.

**The last-change column is git's**, taken in one `git log` for every issue instance at once. An
entry with no commit shows `uncommitted`; where git cannot answer the column shows `-`.
A date never enters a generated file, per `design@knowledge@generated-files-are-pure` — it is printed
here and nowhere else.

## `index`

Regenerates every generated index in place, from one walk: the rule index, and one `index.md` per
file-register instance whose directory is there. It takes no flags and **writes only where the
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

**Every generated index is outside the walk by construction** — the tool derives the set, the rule
index and one per file-register instance, from the manifest, so no `[walk] skip-files` row names one
and none can be created inside the walk.

Running it to look therefore costs nothing, not even an mtime. **Whether a generated file is
current is not this command's question** — that is `cargo knowledge check`, whose `generated` check
is a gate and names the first line at which the committed file and the regenerated one disagree.
Both halves are `design@knowledge@generated-files-are-pure`.

## `model`

Every observation the walk and the scanner produced, one per line, as `file`, `line`, `kind`,
`value`, tab-separated on stdout; the document and observation counts go to stderr, so redirecting
stdout gives a file that is only observations.

**This is also how a citation is located inside the file that holds it.** The generated rule index
says which files cite a rule, deliberately at file level; filtering this dump on the rule number
says where in each. It answers from the scanner's own notion of a citation rather than from a
pattern, which matters because markers and rule tokens are separate kinds here. A rule number that
is **data** — inside an inline code span, inside a string literal bound to a name in Rust, or
inside any string literal of the checker's own source — is not a citation and does not appear in
the dump. A number inside a **fenced block does** appear,
because a fenced sketch cites its rules for real; root `path@thaum@CLAUDE.md` owns that distinction and
this is a restatement of it.

```sh
cargo knowledge model | awk -F'\t' '$4=="<number>" && ($3=="rule-token" || $3 ~ /^marker-/)'
```

## `rules`

- **`show`** prints a rule as a citation is written: `> <number> <body>`, the body entire, on one
  line, from the vendored release and no other. Paste the line into a document as a blockquote, or
  take the body alone for the inline form. It names the rule's subrules where it has any, because a
  whole-body quote of a parent does not stand for a claim its subrule carries, and it fails the run
  on a number the release does not hold rather than printing nothing.
- **`latest`** compares the pinned release against what is published, and treats *no match* as the
  extractor failing rather than as an answer.
- **`diff`** names its two releases, `--old` and `--new`, both required. They are flags rather than
  positions because the output names no direction, so the pair given the wrong way round reports
  new rules as gone and points every renumbering backwards — `design@thaum@named-values-where-order-decides`.
- **`fetch`** vendors a release and rewrites the version file to match; without a date, the one
  already pinned.
- **`bump`** moves the project to a release. Read `bumping-rules` before running it.

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
sections = ["Rules", "Reading", "Consequences"]

[registers.interpretation.metadata.status]
values = ["settled", "interpretation", "ambiguous", "cr-gap"]
```

A **heading register** keeps its entries as slugged headings, in `<dir>.md` or in `<dir>/` behind
a `README.md` that links every subdocument. A **file register** keeps one file per entry under
`<dir>/`, beside a hand-written `README.md`, a generated `index.md` and an optional
`register.toml` declaring the group subdirectories. An entry opens with frontmatter carrying each
declared metadata key, then a level-one title, then the declared sections.

`[registers.issue]` accepts `kinds` and nothing else; the other three built-in registers accept
nothing. The arguments are `design@knowledge@registers-are-declared` and
`design@knowledge@a-file-register-is-a-directory-of-entries`.

## Commit messages, and the hook

**A commit message is a document under the citation regime.** It is parsed as one markdown
document — subject line, blank line, body — and every rule runs over it: a `CR:` marker owes its
verbatim quote inside the message, every `` `<kind>@<anchor>@<id>` `` reference resolves, and the
missing-marker lint reads it as it reads any other prose. The argument is
`design@knowledge@a-commit-message-is-a-document`.

```sh
cargo knowledge commit-message .git/COMMIT_EDITMSG   # one message, against the index's tree
cargo knowledge commits origin/main..HEAD            # the branch's own commits
```

`commits` reads everything from each commit's own tree through git objects — the manifest, the
documents, the generated indexes, the pinned corpus — so a message is judged against the tree it
was written against, byte for byte and cleaned of nothing. **A commit before the tip whose tree does not
load or carries findings fails the run**, with its tree's findings named by the commit and the
file, and its message is still judged where its tree reached the last phase; the summary block
counts judged and failed commits, and a tip whose tree fails is exit 2 with a last line that says
so. The checker that judges every commit is the one built from the working tree, so a branch that
makes it stricter puts that change in its first commit or is squashed before review. A message's references resolve against its
commit's tree **or its first parent's**, which is what lets a commit that closes an issue name it.

`commit-message` is handed a draft rather than a commit, so it blanks out git's `#` comment block
and the `--verbose` diff. It judges the draft against the tree the index holds, the one git names
in `GIT_INDEX_FILE` under `git commit <path>` and `git commit -a`, and it runs the first three
phases over that tree and no content family. It takes HEAD as the parent tree — HEAD being the
parent of the commit the draft is for — reading it only where the index refused something, so a
clean message costs one model. A `#` line a `-m` message keeps is therefore silent to the hook and
reported by `commits`.

`check` reads no history, and the range is always explicit. `cargo x gates` runs
`commits origin/main..HEAD` as a gate.

**The hook judges a message before the commit exists.**

```sh
cargo knowledge hook install     # core.hooksPath = .githooks
cargo knowledge hook status      # 0 installed, 1 not
```

`path@thaum@.githooks/commit-msg` is committed, so a review can read it; `install` writes it where a tree
carries none, and refuses to replace a `core.hooksPath` that names something else without
`--force`. The script runs `cargo knowledge`, which is the alias in `path@thaum@.cargo/config.toml`: a
tree without that alias needs the script rewritten to whatever reaches the binary there. **No check reads the hook's status**: per-clone configuration must not move a verdict.
`cargo x gates` prints the status line beside its verdicts and gates on nothing about it.

## What to respect

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`path@thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`path@knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says
so there, in one place, with a reason beside it. The one thing compiled in is the tool's own
directory, so that the string literals of its own source are read as data, per
`design@knowledge@checker-source-literals-are-data`; that is a fact about the tool, not about any tree.

Read `path@knowledge@docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
