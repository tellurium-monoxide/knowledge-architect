# The core checker

How a project's knowledge is held: its documents, the references between them and the registers
they carry. This Component is the generic core. A subject that belongs to one project is an
extension of it, per `design@core@an-extension-plugs-in-through-phased-hooks`.

**The core's binary, and a binary per set of extensions.** The core's own binary, `klarch` in the
package `knowledge-architect`, runs every command below with no extension registered. A binary
that registers extensions runs the same commands unchanged, and its extensions' own commands beside them, per
`design@core@the-core-cli-is-a-library-module`. Over a manifest holding a table no
extension of the binary claims, the binary reports that table in phase 1, rather than skip in
silence what the table configures, per `design@core@an-extension-claims-its-manifest-tables`.
The commands below are written `cargo klarch <command>`, the command this repository declares: a
cargo alias over the binary it runs, built from the checkout. A project declares its own command
with `[project] command`, and `klarch` is the command when it declares none. The checker prints
the declared command in its messages, in the header of every generated index, and in the agent
files it installs, per `design@core@declared-command`.

**The library is for a binary that registers an extension.** Its public API, and how to write and
test an extension with it, is described at <https://docs.rs/knowledge-architect>, for each
published version. The text is the crate documentation of `path@core@src/lib.rs`.

**It needs `git` 2.36 or newer on the path, and a project inside a git worktree.** What the tool reads is what
`git ls-files` reports from the project root, so every pattern git honours decides the walk,
nested `.gitignore` files included, and a file git tracks is read whatever the ignore rules say.
A file whose name holds a line break, or a name Windows cannot create, is read by nothing and
reported once with the reason, any line break escaped: name it in `[walk] skip-files` or in an
ignore rule to keep it.
Neither failure is silent: no binary and no worktree are both exit 2 naming the reason, never an
empty walk. `commits` reads a commit's tree through `cat-file --batch -z`, which is where the
version floor comes from. The decision is `design@core@git-supplies-the-walk`.

```sh
cargo klarch check                     every check, over one walk, in four phases
cargo klarch check --fix               apply every safe fix, list each, then every check
cargo klarch show <kind>@<anchor>@<id> one recorded entry, and every reference to it
cargo klarch issues [anchor] [--kind k] [--group g] [text …]
                                       every issue entry, one row each
cargo klarch tripwires [anchor] [--guarding <ref>] [text …]
                                       every tripwire entry, and what each guards
cargo klarch index                     regenerate every generated index in place
cargo klarch model                     every observation the walk produced
cargo klarch commits <range>           judge every commit in the range, message and tree, against its own tree
cargo klarch install-agent-skills      write the agent files this version ships, remove the ones it does not
cargo klarch --version                 the version of the checker that runs
```

## Exit codes

Three, per `design@core@exit-code-ladder`, and the third is what makes the other two mean anything.

| code | meaning | where it comes from |
| ---- | ------- | ------------------- |
| `0` | the command ran and its subject is in order | `--version`, from any directory; `check` with no findings; `show` on a reference that resolves; `issues` and `tripwires` with at least one row; `index` having written every destination, or found each already current; every `model` run; `commits` with no finding, an empty range included; `install-agent-skills` having written and removed what it had to, or found nothing to do |
| `1` | the command ran and reports a negative answer | `check` with findings, `--fix` included; `check --fix` when a write failed, a destination was refused, or the final check could not run, after another file was already written; `show` on a reference that resolves to nothing; `issues` or `tripwires` with no row; `index` when a write failed after another destination was already rewritten; `commits` with a finding against a judged message or a failing tree |
| `2` | the command could not run | an unknown or invalid argument, a `show` argument that is not reference-shaped, a binary built from another checkout of the tool, which names the `cargo clean` that rebuilds it, a binary of another version than `[project] checker-version` pins, or a pin it does not confirm, or a manifest with no pin, `index` or `check --fix` refusing a destination — a symlink, or a directory that is not there — having written nothing, `commits` on a range that does not resolve, no project above the working directory, no `git` on the path or a project outside a worktree, `install-agent-skills` over a manifest holding a refused declaration, over a symbolic link on an owned path, or when a write or a removal failed, naming the path, a stdout closed before the output was written — as `\| head` does — which ends the run silently, an input that cannot be read, an input an extension prepares that it cannot resolve |

**A caller scripting against a run reads the exit code; a person reads the last line.** Arguments
are refused before the project is located, so `--help` answers from anywhere and a mistyped
invocation costs no walk.

## `check`

**A run prints the summary first, its findings under it, and its verdict on the last line** —
`PASSED: no findings`, or `FAILED: n findings above`. The verdict is derived from the finding list
rather than tracked beside it, so it cannot disagree with the exit code. **The order is a
contract**: the summary block prints on a failing run as well as a passing one, so a reader taking
the tail of the output has to reach the verdict rather than the counts. When a finding sits in a
file git does not track, one note naming each such file follows the findings, above the verdict:
the walk reads untracked files, and a scratch file left in the tree fails the run like any other.

**The `checker source:` line names the binary's own source directories**, compiled into it, and
counts the walked files under them whose string literals are read as data rather than prose, per
`design@core@checker-source-literals-are-data`. A directory inside the tree is printed relative to
the root, and one outside it as an absolute path. A binary installed with `cargo install` names its
source in cargo's registry, outside the tree, so its count is 0 and nothing of the project is read
differently. A maintenance crate that runs the checker names its own directory, as the setup
skill's `main` does, so the literals of that crate's Rust files are data, and its comments stay
prose.

**A run is four phases, and it stops at the first that finds anything.** Phase 1 resolves the
manifest: a declaration the tool refuses is reported and acted on by nothing. Phase 2 reads the
tree against what the manifest declares: a file the walk could not read, a name it refuses, an
anchor or a register home that is not there, a home a walk row keeps out, a declared path that
does not exist, a file git both tracks and ignores, a symlink or a submodule, and, under the `claude` agent
harness, an installed agent file missing, differing or unshipped, a deletion of one not staged, or
a root CLAUDE.md that does not import a shipped primer. Phase 3 builds the entity table: a slug or an entry id where none may sit, or
defined twice. Each of these says the model is incomplete, and a finding computed from the model
afterwards would be unreliable in both directions, so the run prints that phase's findings, says
which phases were not judged, and exits 1. Phase 4 is every check, over the complete model: the
core's `generated`, `registers` and `references`, each a module under
`path@core@src/check/`, then each registered extension's, per
`design@core@an-extension-plugs-in-through-phased-hooks`. `references` judges
every `` `<kind>@<anchor>@<id>` `` reference against the entity table — a register kind against the
entries its home defines, the `path` kind against the tree, the `planned` kind against its absence
from the tree — and reports the retired slug
reference form and the unanchored path shape where either names something of this project.
`registers` judges the shape of what each anchor carries: a file
register's README, index, groups and entry shapes, and a directory home's links. A check the tree
gives no input to is printed as not run rather than counted. There is no way to select a subset:
the checks cross the phases, and a run over a passing tree costs under a second. The argument is
`design@core@phases-gate-the-report`.

`index`, `check --fix` and a writing command of an extension run the first three phases too, and
write no generated file while one of them holds anything. `index` and an extension's command refuse
with exit 2; `check --fix` prints the stopped report and exits 1, having installed agent files
before that gate when they needed it, since their bytes do not depend on the model: an index generated over an incomplete model lists rows nobody asked for.

**`check --fix` applies every fix the checker can make safely, then runs the check**, per
`design@core@check-fix-flag`. A fix is safe, per `design@core@safe-fix-definition`, when its
bytes are determined by the tree and the pinned version, and it writes or removes only files of
the installer's namespace or of the generated list. Two pass: the agent files the check reports missing,
differing or no longer shipped, judged from git's listing as the check judges them, so a file git
does not list, such as an ignored swap file, is never removed; and every stale or missing generated
file. Each file written or removed is listed on a line `fixed: wrote <path> (<kind>)` or
`fixed: removed <path> (installed)`, before the report. Every other finding's repair is a choice,
or touches git or a hand-written file, and stays the reader's.

- The order, per `design@core@fix-before-the-checks`: a manifest holding a refused declaration
  writes nothing; then the agent files; then
  the first three phases, over the tree as the install left it, which stop the run and write nothing
  more; then the generated files; then the full check, whose report and exit code are the run's.
  With nothing to fix, the output is a plain `check`'s.
- **An upgrade that removes a shipped file takes two runs.** The install deletes the file, git
  still lists it, and phase 2 reports the deletion as not staged, so the generated files wait. Run
  `check --fix`, stage the deletion with `git add`, then run `check --fix` again: no fix touches
  git.
- A write that fails exits 2 when nothing was written yet, and 1 after any write, as `index` does.
- `--fix` belongs to `check` alone. `commits` judges history and takes no `--fix`, and the gates
  run `check` without it, so continuous integration judges the tree as committed.

## `show`, `issues` and `tripwires`

The three commands that print what is recorded, given a reference or an anchor. All three read the
entity table, so what they print is what `check` resolves against.

**`show <kind>@<anchor>@<id>`** prints one entry and then every reference to it, as `file:line`:

- a file register's entry is printed **whole**, frontmatter included;
- a heading register's entry is printed as **its section** — the heading through to the next
  heading at the same level or shallower;
- `path@<anchor>@<path>` prints the walked document's text, or says the path is outside the walk;
  its inbound list includes the `planned` citations of the same path.
- `planned@<anchor>@<path>` says whether the tree's listing holds the target yet, and lists the
  plans that cite it, with or without the trailing slash; one the check refuses resolves to nothing.

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
A date never enters a generated file, per `design@core@generated-files-are-pure` — it is printed
here and nowhere else.

## `index`

Regenerates every generated file in place, from one walk: each file a registered extension
generates, and one `index.md` per file-register instance whose directory is there. It takes no flags and **writes only where the
bytes differ**, naming each file it rewrote:

```
$ cargo klarch index
docs/open-issues/index.md                rewritten
crates/core/docs/open-issues/index.md    already current
```

**A file-register index is a fixed shape, and its bytes are a contract**, per
`design@core@a-file-register-index-is-rows`:

```markdown
**Generated — do not edit.** `<command> index`

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
current is not this command's question** — that is `cargo klarch check`, whose `generated` check
is a gate and names the first line at which the committed file and the regenerated one disagree.
Both halves are `design@core@generated-files-are-pure`.

## `install-agent-skills`

Under the `claude` agent harness, the default, the checker installs the agent skills, subagent
definitions and a primer, and this command writes them into the project's `.claude/` directory,
each rendered with the project's declared command. It removes every file of its own namespace the
version does not ship: a skill directory or an agent file whose name starts with
`knowledge-architect-`, and the directory .claude/knowledge-architect/. A project's own skill
takes a name of its own, and the command never touches it. It never edits the root CLAUDE.md
either: when the primer is shipped and the root CLAUDE.md does not import it, it says so, and the
line to add is one holding exactly an at sign followed by .claude/knowledge-architect/PRIMER.md.

Commit what it writes. `check` compares each installed file with what the version ships, byte for
byte, and stops in phase 2 on a missing, differing or unshipped file, so a project that moves to a
new version runs the install with the move. The decision is `design@core@owned-namespace-check`.

```toml
[agents]
harness = ["claude"]   # the default when the table is absent
# harness = []         # no agent configuration: no install, no check, no CLAUDE.md required
```

With `harness = []` no component owes a CLAUDE.md, and nothing is installed or checked, per
`design@core@agents-table`.

**Adopting the workflow starts with the install.** The skill that sets a project up,
knowledge-architect-setup, is one of the files it writes, so a project first holds the
smallest manifest the install accepts, at its root, `<version>` being the version of the checker
the project runs:

```toml
[project]
name = "<project name>"
checker-version = "<version>"
components = []

[walk]
skip-dirs = []
skip-files = []
exclude = []
```

Then `klarch install-agent-skills` writes the skills, the agents and the primer, and an agent
session follows the installed setup skill from there. `klarch check` lists every document the
project still owes, each with its repair.

## `model`

Every observation the walk and the scanner produced, one per line, as `file`, `line`, `kind`,
`value`, tab-separated on stdout; the document and observation counts go to stderr, so redirecting
stdout gives a file that is only observations.

## Registers

**Ten registers are compiled in.** `design`, `goal`, `tripwire` and `issue` are what the word
component means to this tool. `spec` and `milestone` are the plan documents, and `thread`,
`argument`, `criterion` and `acceptance` their items, described under "Plan documents" below. A project declares further ones in its
`knowledge-architect.toml`. The first two tables are from thaum, whose `rules` location carries
an `interpretation` register:

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

[registers.note]
scope = "opt-in"
shape = "heading"
dir = "decisions"
level = 2                      # a heading register only: the heading level its entries sit at
```

A **heading register** keeps its entries as slugged headings, in `<dir>.md` or in `<dir>/` behind
a `README.md` that links every subdocument. It declares `level`, from 2 to 6: every heading at that
level in its home is an entry and carries a slug, and a slug at any other level defines nothing,
per `design@core@an-entry-is-a-heading-at-the-register-level`. The built-in levels are 3 for
`design` and 2 for `goal` and `tripwire`.

A **file register** takes no `level`. It keeps one file per entry under `<dir>/`, beside a
hand-written `README.md`, a generated `index.md` and an optional `register.toml` declaring the
group subdirectories. An entry opens with frontmatter carrying each declared metadata key, then a
level-one title, then the declared sections.

`[registers.issue]` accepts `kinds` and nothing else; the design, goal and tripwire registers
accept nothing, `level` included, and a table for a plan register or an item register is refused
whole. The
arguments are `design@core@registers-are-declared` and
`design@core@a-file-register-is-a-directory-of-entries`.

## Plan documents

**Every project carries a plans directory, docs/plans/ at its root**, and the tool constructs an
anchor named `plans` there. No manifest row declares it, and a declared anchor named `plans` is
refused. It holds three things and nothing else:

| path | what it is |
| --- | --- |
| docs/plans/README.md | hand-written: what the directory holds |
| docs/plans/specs/ | the `spec` register: one file `<id>.md` per spec, beside a hand-written `README.md` and a generated `index.md`, as any file register |
| docs/plans/milestones/ | the `milestone` register: one directory `<id>/` per milestone, beside a hand-written `README.md` and a generated `index.md` |

**Each milestone directory is an anchor of its own**, named by its basename. Its `README.md` is the
milestone document, its `index.md` is generated and lists its steps, and every other `.md` file in
it is the spec of one step. A directory under milestones/ with no `README.md` is a finding. **Each
spec file is an anchor too**, named by its id, and stays an entry of the `spec` register of
`plans`.

**A plan document holds fixed sections.** A spec of specs/ and a milestone's README owe, in order:
Status and audience, How a step is worked, Names, What the work is, What is already decided,
Criteria, Threads, Arguments, "New names, in one place", Decided design, Mapping tables, Losing
alternatives, Readings, Premortem, Acceptance criteria, Implementation sequence, Order rationale,
Defaults awaiting the owner, Harvest, Later consequences. A step spec owes Builds, Claims, Audit
subjects, Fails alone on, Premises that expire.

**A plan's items are headings of four of those sections.** A level-three heading ending with its
slug, `### <statement> ##<id>`, under Threads, Arguments, Criteria or Acceptance criteria, defines
an item of the kind `thread`, `argument`, `criterion` or `acceptance`, in the plan's anchor: the
spec, or the milestone, whose README and step specs share one namespace. Every level-three heading
of those sections carries a slug; a slug anywhere else in a plan defines nothing. An item is cited
`<kind>@<plan>@<id>`, from inside its own plan only: a citation from outside it, a commit message
included, is refused, and the finding names the whole-document form.

**A plan document is cited by its kind, never by its path**:

```text
spec@plans@<id>               a spec, docs/plans/specs/<id>.md
milestone@plans@<id>          a milestone, docs/plans/milestones/<id>/
spec@<milestone>@<step>       a step spec, docs/plans/milestones/<milestone>/<step>.md
```

A `path` citation of a plan document is refused, and the finding names the form above. The
plans directory's README, and the README and index of each of its two homes, are cited by path,
as in `path@plans@README.md`.

**A path a plan's work will create is cited `planned@<anchor>@<path>`**, from a document of the
plans directory only. Its anchor and path follow the rules of a `path` reference, and the target
must not exist: once it does, the finding asks for `path@<anchor>@<path>`, which the change that
creates the file writes. `*` and `elsewhere` are refused for it, and a register named `planned` is
refused like one named `path`.

**A plan's name reads as nothing else.** A milestone's name or a spec's id that is the name of a
component, of a location or of `plans`, `elsewhere` or `*`, and one name used under both homes,
are findings. So is a milestone name outside the id grammar, `[a-z0-9]+(-[a-z0-9]+)*`.

A tree that breaks this layout stops the run at phase 2, except a home's missing `README.md` or
`index.md`, which is a finding of the last phase as in every file register. A misplaced or
duplicated item stops it at phase 3, and a missing section is a finding of the last phase. The arguments are
`design@core@plan-register`, `design@core@plans-split-dirs`, `design@core@plan-document-kinds`,
`design@core@a-plan-name-reads-as-nothing-else`, `design@core@spec-file-owns-its-items`,
`design@core@step-spec-sections`, `design@core@plan-items-by-section` and
`design@core@plan-item-scope`.

## Commit messages

**A commit message is a document under the regime.** It is parsed as one markdown document —
subject line, blank line, body — and every rule runs over it: every
`` `<kind>@<anchor>@<id>` `` reference resolves, and each registered extension judges it against
its commit's tree. The argument is
`design@core@a-commit-message-is-a-document`.

```sh
cargo klarch commits HEAD~1..HEAD        # the commit just made, where branch SHAs are not refused
cargo klarch commits origin/main..HEAD   # the branch's own commits
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

**A project whose merges rewrite SHAs refuses citations of its branch's own commits.** Under
`[commits] refuse-branch-shas = true`, `commits` reports every run of 7 to 64 lowercase hex digits
that prefixes the SHA of a commit of the range, in a message or in a document of a commit's tree: a
rebase merge gives those commits new SHAs, and the citation would then name nothing. Name such a
commit by its subject. A SHA already on the main branch may be cited. The option is off when absent,
and is read from each commit's own manifest. A citation is seen only when the cited commit is in the
range judged, so with the option on, run the check after a commit over the branch, `<base>..HEAD`:
over `HEAD~1..HEAD`, a citation of an earlier commit of the branch passes, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`. The argument is
`design@core@branch-shas-are-refused`.

`check` reads no history, and the range is always explicit. `cargo x gates` runs
`commits origin/main..HEAD` as a gate.

**The range need not end at the checkout.** Any range `git rev-list` resolves is judged, commit by
commit, and where HEAD sits changes nothing. The uses a range serves:

| use | range |
| --- | --- |
| the commit just made, before it is amended or built on | `HEAD~1..HEAD`, or `<base>..HEAD` under `refuse-branch-shas` |
| a branch's own commits, before review and merge | `<base>..HEAD` |
| CI over a pull request, which checks out the branch's head | `<base>..HEAD` |
| a pre-push hook: git gives it, per pushed ref, the local and the remote sha, and with `git push origin other-branch` the local sha is not HEAD | `<remote sha>..<local sha>` |
| a branch that is not checked out, reviewed locally | `<base>..<branch>` |
| an audit of old history, such as the commits made before a manifest migration | `<first>^..<last>` |

`<base>` is the branch the work merges into, as the remote holds it.

Two things are still read from the working tree, whatever the range: the binary that judges, and
git's ignore rules, which `git check-ignore` answers for the files on disk only.

## What to respect

**Nothing about a project is compiled into the tool.** Every list a check reads comes from the
project's `knowledge-architect.toml`, which is both the manifest and the marker that makes a
directory a project root. So one binary checks any project and every mock project under
`path@core@tests/projects/` with no special case. A path that should not be checked says so
there, in one place, with a reason beside it. What is compiled in describes the tool, not a tree:
the directory of each Component a binary's libraries belong to, so that the string literals of
the tool's own source are read as data. The decision is
`design@core@nothing-of-a-project-is-compiled-in`.

Read `path@core@docs/design.md` before changing how it works.
