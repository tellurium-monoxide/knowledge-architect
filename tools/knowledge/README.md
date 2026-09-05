# Knowledge checker

How this project's knowledge is held: the Comprehensive Rules corpus, and the documents that cite
it. One binary, reached through a cargo alias so nothing has to be installed.

```sh
cargo knowledge check [--only a,b,c]      every check, over one walk; or only these families
cargo knowledge show <kind>@<anchor>@<id> one recorded entry, and every reference to it
cargo knowledge issues [anchor] [--kind k] [--group g] [text …]
                                          every issue entry, one row each
cargo knowledge tripwires [anchor] [--guarding <ref>] [text …]
                                          every tripwire entry, and what each guards
cargo knowledge index                     regenerate every generated index in place
cargo knowledge model                     every observation the walk produced
cargo knowledge rules show <number> …     the pinned text of a rule, shaped to be quoted
cargo knowledge rules latest              is a newer rules release published?
cargo knowledge rules diff --old <date> --new <date>
                                          what moved, filtered to what this project cites
cargo knowledge rules fetch [<date>]      fetch a release and repin to it
cargo knowledge rules bump <date>         archive, fetch, reindex, draft the changelog
```

## Exit codes

Three, per `thaum#exit-code-ladder`, and the third is what makes the other two mean anything.

| code | meaning | where it comes from |
| ---- | ------- | ------------------- |
| `0` | the command ran and its subject is in order | `check` with no findings; `rules latest` up to date; `show` on a reference that resolves; `issues` and `tripwires` with at least one row; `index` having written every destination, or found each already current; every `model` run |
| `1` | the command ran and reports a negative answer | `check` with findings; `rules diff` with changes; `rules show` on a number the release does not hold; `show` on a reference that resolves to nothing; `issues` or `tripwires` with no row |
| `2` | the command could not run | an unknown or invalid argument, a `show` argument that is not reference-shaped, `index` refusing a destination — a symlink, or a directory that is not there — having written nothing, no project above the working directory, an input that cannot be read, `rules latest` when it cannot produce a comparison — the extractor matched nothing, or the published date is earlier than the pinned one |

**A caller scripting against a run reads the exit code; a person reads the last line.** Arguments
are refused before the project is located, so `--help` answers from anywhere and a mistyped
invocation costs no walk.

## `check`

**A run prints the summary first, its findings under it, and its verdict on the last line** —
`PASSED: no findings`, or `FAILED: n findings above`. The verdict is derived from the finding list
rather than tracked beside it, so it cannot disagree with the exit code. **The order is a
contract**: the summary block prints on a failing run as well as a passing one, so a reader taking
the tail of the output has to reach the verdict rather than the counts.

**The `--only` families are the checks themselves**, one per check:

`citations`, `generated`, `registers`, `references`, `uncovered`, `changes`, `corpus`, `regime`.

Seven are the modules under `knowledge@documentation/src/check/`. `corpus` is the integrity check
over the vendored text and its archive, which reads the filesystem rather than the model.
`references` judges every `` `<kind>@<anchor>@<id>` `` reference against the entity table — a
register kind against the entries its home defines, the `path` kind against the tree — and reports
the two retired forms and the unanchored path shape. `registers` judges what each anchor carries:
the homes, a file register's README, index, groups and entry shapes, and every definition site.
`structure` names every family but `citations`. A comma-separated list runs their union over the
one walk, so asking for several costs one run rather than one run each. A run prints which families
it performed, and a family that did not run prints no count of its own. The argument is
`knowledge#families-are-the-checks`.

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
entry with no commit shows `uncommitted`; where git cannot answer at all the column shows `-`.
A date never enters a generated file, per `knowledge#generated-files-are-pure` — it is printed
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
`knowledge#a-file-register-index-is-rows`:

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

**Every generated index is outside the walk**, the rule index by a `[walk] skip-files` row and a
file-register index by construction — the tool derives that set from the register instances, so no
manifest row names one and none can be created inside the walk.

Running it to look therefore costs nothing, not even an mtime. **Whether a generated file is
current is not this command's question** — that is `cargo knowledge check --only generated`, which
is a gate and names the first line at which the committed file and the regenerated one disagree.
Both halves are `knowledge#generated-files-are-pure`.

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
because a fenced sketch cites its rules for real; root `thaum@CLAUDE.md` owns that distinction and
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
  new rules as gone and points every renumbering backwards — `thaum#named-values-where-order-decides`.
- **`fetch`** vendors a release and rewrites the version file to match; without a date, the one
  already pinned.
- **`bump`** moves the project to a release. Read `bumping-rules` before running it.

## Registers

**Four registers are compiled in** — `design`, `goal`, `tripwire` and `issue` — because they are
what the word component means to this tool. A project declares further ones in
`thaum@knowledge.toml`:

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
nothing. The arguments are `knowledge#registers-are-declared` and
`knowledge#a-file-register-is-a-directory-of-entries`.

## What to respect

**Nothing about this repository is compiled into the tool.** Every list a check reads comes from
`thaum@knowledge.toml`, which is both the manifest and the marker that makes a directory a project
root — so the same binary checks this repository and a mock project under
`knowledge@tests/projects/` with no special case anywhere. A path that should not be checked says
so there, in one place, with a reason beside it. The one thing compiled in is the tool's own
directory, so that the string literals of its own source are read as data, per
`knowledge#checker-source-literals-are-data`; that is a fact about the tool, not about any tree.

Read `knowledge@docs/design.md` before changing how it works, and `bumping-rules` before adopting a
rules release.
