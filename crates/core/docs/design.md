# The core checker: design

Recorded intent for this tool: how it is built internally, and why. Present tense, each decision
carrying a slug anchor, cited from elsewhere with `core` as its component. What lost to a decision
here is `path@core@docs/rejected-alternatives.md`, and `primer@design-heads` owns the shape.

**What belongs here:** a decision that does **not** survive deleting this tool. How it works and
how the pieces inside divide the work belongs here.

What otherwise shapes it is the module documentation at the top of each file under
`path@core@src/`.

The heads are grouped by subject. A group heading carries no decision; each decision is one
level-three head below it.

## 1. The shape of a run

### Nothing about a project is compiled into the tool, and the manifest declares every list a check reads `##nothing-of-a-project-is-compiled-in`

Every list a check reads comes from the project's `knowledge-architect.toml`: the components, the
locations, the registers, the walk's exclusions. The manifest is also the marker that makes a
directory a project root. So one binary checks any project, and every mock project under
`path@core@tests/projects/`, with no special case in the code. A path that must not be
checked is declared there, in one place, with its reason beside it.

The constraint derives from `goal@knowledge-architect@any-project-can-adopt-it`. A list
compiled in for one project is a list every other project is checked against. It also keeps the
tests honest: a mock runs the same code path as a real tree, so a test over a mock tests the
code that checks the real tree.

Five things are compiled in, and none of them describes a project:

- **What the word _component_ means**: the documents a component carries and the four built-in
  component registers, per `design@core@components-carry-the-same-documents`.
- **Where plan documents live**: the anchor `plans` at docs/plans/ of the root and its two
  registers, per `design@core@plans-dir-fixed`. It is the tool's convention for a kind of
  document every project has, not a fact about one project.
- **The directory of each Component of the tool's own source**, so that its string literals are
  read as data, per `design@core@checker-source-literals-are-data`.
- **Each crate's own directory and package name**, so that a binary built from another checkout
  is refused, per `design@core@a-foreign-build-is-refused`.
- **The agent harnesses it knows, and the installer's namespace in a project's agent
  configuration**, per `design@core@agents-table` and `design@core@owned-namespace-check`: what
  this tool writes, not what a project holds.

### The model is built once, and every check is a pure function over it `##model-then-checks`

A check never reads a file, spawns a process, or knows how the walk works. Anything a check cannot
fetch for itself is resolved by the caller and handed in: the generated files as committed, the
per-instance `register.toml` files, one listing of what exists and which of those paths are
directories, the files the walk does not cover, and git's answer for every path spelling a
reference in the run could ask about. An extension's checks read, besides these, what the extension's
own preparation read for the tree, per `design@core@an-extension-plugs-in-through-phased-hooks`.
The phases that precede the checks are pure over the same inputs, per
`design@core@phases-gate-the-report`, and an extension prepares only once the last phase is
reached.

Four things follow, and the rival in which each check reads the tree for itself pays for each of them.

**The single walk is a property of the design rather than of anyone's care.** One run reads each
live document once and parses it once. The core scans each line once; an extension scans the same
parse again, as often as its checks need, per `design@core@an-extension-builds-its-own-model`. That rival, as built in thaum, performed four tree
walks and 311 file reads over thaum's 104 documents per run, then spawned its citation checker twice more to
do it again.

**A check is tested against a model assembled in memory**, so most tests need no fixture on disk at
all, which keeps a planted-defect corpus cheap to build.

**No check writes to the tree; `check --fix` writes before the model its checks read is built**,
per `design@core@fix-before-the-checks`. Every generated index is rendered into a `String` and
compared. A check that regenerates by writing the file, comparing,
and writing the old bytes back mutates the tree it is checking, and a history replay over it
needs a discarding checkout between commits.

**A check whose subject is filesystem state is the one exception, and it says so where it
lives.** Whether a vendored corpus matches its pin, whether an archive has provenance, whether a
needed release is present: there is no model to hand such a check, so an extension reads that
state while it prepares a checkout, per `design@core@an-extension-plugs-in-through-phased-hooks`.
That is a different thing from a check over documents, which stays pure. A check over another
document than the model's, such as a changelog, is pure too: the extension hands it the text and
the releases it names.

### A finding is classified by the phase that produces it, the report stops at the first non-empty phase, and every check runs at the last `##phases-gate-the-report`

Findings are not independent. Some say what the model could not read or resolve, and every
finding computed from that model afterwards is then unreliable in both directions, missing and
false. A list that mixes the two advertises a completeness it does not have. So a run is four
phases, each of the first three building one input of the next:

| phase | produced by | what it reports |
| ----- | ----------- | --------------- |
| 1 | `Manifest::parse`, and each extension's resolution of its tables | a declaration the tool refused, which is then absent from the configuration |
| 2 | `check::tree`, over the core's declared paths, the paths each extension declares and every document's parse, and `check::agents`, over the installed agent configuration | a file the walk could not read or refused, a tracked-and-ignored file, an anchor or a register home that is not there, a declared path that does not exist, a home a walk row keeps out, an installed agent file missing, differing or unshipped, a root CLAUDE.md that does not import a shipped primer, a heading markdown reads and the checker does not, per `design@core@headings-open-with-hash-marks` |
| 3 | `Entities::build` | a slug or an entry id where none may sit, or defined twice; a level-two heading of a section home with no slug; a skill's or an agent's name a reference cannot spell, or that its frontmatter contradicts |
| 4 | every check, the core's and each extension's | everything computed over a complete model |

A finding is classified by **the place it is produced**, never by a label at its site, so a
finding added to a building step is gated because of where it is raised, and nothing in the
last phase can undermine anything, because no check reads another check's findings. The run
stops at the first phase that produced anything, prints those findings, and opens its summary
with the phase, the count and the phases not judged, so a stop is never mistaken for a pass;
a clean run names every check it performed. The whole report stops: the per-document form,
hiding only what an unread file could have defined, is three times the machinery for facts
that mostly have no per-file scope, and `path@core@docs/tripwires.md` guards that choice.

**An extension prepares its tree when the last phase is reached**, so a run that stops earlier
resolves no release and fetches nothing, whatever it would have judged. **A writer refuses over an incomplete model**: `index`
and a writing command of an extension, such as thaum's `rules bump`, run the first three phases
before touching anything and exit 2 naming the
phase, since an index generated over such a model lists rows nobody asked for. `check --fix` writes
no generated file over such a model either; it prints the stopped report and exits 1, having
repaired before that gate only the installed files, whose bytes do not depend on the model, per
`design@core@fix-before-the-checks`. `commits` judges
no message's references against a commit whose tree stops early, and says so, since a message
judged against an incomplete entity table is judged against nothing. The scan for citations of the
range by SHA needs no entity table, so it still reads that message, per
`design@core@branch-shas-are-refused`.

**Nothing selects a subset of the checks.** The checks cross the phases — what `registers`
asserts sits in phases 2 and 4, what `references` asserts in 3 and 4 — so a selection could be
honoured only with a second bookkeeping saying which part of a check a stop withheld. The
selection, and what it bought, is `path@core@docs/rejected-alternatives.md`.

**One producer chain, and a rule for a second.** The manifest feeds the walk, the walk the
model, the model the table, and every check consumes the table. A future check whose findings
another check reads would be a second chain, and nothing prevents one: such a check is a phase,
not a check of the last one, and it goes before its consumers. A tripwire names the event.

### A finding names a cause only where the repair depends on which cause holds `##finding-names-the-repair`

A finding's action names the repair. It names a cause only when several causes can produce the
finding and each needs a different repair; the finding then tells them apart by naming each case
with its repair, as a dangling reference does for an unknown kind, an unknown anchor and an
undefined id. Where one repair is correct for every cause, the finding names that repair and no
cause. A cause adds nothing the reader needs to act, and it can be wrong: a stale generated index
is left by a hand edit, by an upgrade of the checker that changes the generator's bytes, and by a
branch that changes the generator, and a finding that names one of them misleads the reader in the
other two cases. The test is checkable per finding: list the causes that can produce it, and ask
whether one repair is correct for all of them. The rival condition, that a finding names no cause
where its repair is a safe fix under `design@core@safe-fix-definition`, lost: the repair of a
generated file that fails in every earlier commit of a branch is a history edit, which is no safe
fix, and it is the same repair whatever left the file out of date. How a repair is
carried out safely is the project's rule for that operation, not the finding's text. This serves
`goal@knowledge-architect@adoption-is-easy`: the finding is what a consumer reads first.

### A domain's checks plug in as an extension compiled into the binary, and the core binary registers none `##an-extension-plugs-in-through-phased-hooks`

The core checks what every project carries: the walk, the parse, the entity table, references,
registers, generated indexes and commit messages. A subject that belongs to one project, such as
thaum's verification of rule quotes against a pinned corpus, is an **extension**: a set of checks, the
manifest tables they read, and the files they generate. Each of its checks is listed and counted
like a check of the core. This is what `goal@knowledge-architect@any-project-can-adopt-it`
requires: the core is published for other projects, and nothing of thaum's subject is compiled
into it. A binary registers its extensions at
compile time. The core's own binary registers none, and a project that needs an extension builds a
binary that registers it. There is no loading at run time, because Rust has no stable ABI a
plugin could be built against.

The core calls an extension at fixed points of the run, and each call sits in the phase its output
belongs to, per `design@core@phases-gate-the-report`:

| call | when | what the extension does |
| ---- | ---- | -------------------- |
| resolve | phase 1, before the walk | reads the tables it claims, and returns its complaints, the paths it declares and the files it generates; the core reports the complaints in phase 1, asserts in phase 2 that each declared path exists, and leaves the generated files out of the walk and reads them as committed |
| prepare | once phases 1 to 3 passed | reads what its checks need for one tree, and may fail the run as could-not-run |
| check | phase 4 | its checks, with the count lines and the not-run names its summary prints |
| judge a message | `commits` | the rules a commit message is judged by, against that commit's prepared tree |
| generate | `index`, and phase 4 | the contents of the files it generates |
| dump | `model` | its observation rows |

**An extension is configured once per manifest and prepared once per tree.** `commits` judges several
trees in one run, and a message is judged against what its own commit's tree holds, so the
prepared state belongs to the tree and not to the extension. A parent tree, assembled for its entity
table alone, prepares no extension.

**The core's summary prints its own count lines, then each extension's**, and the list of checks
performed names the core's checks, then each extension's.

### An extension is handed a snapshot, `Tree::Snapshot`, for a commit's tree and for the tree git's index would commit alike `##an-extension-reads-a-snapshot`

`extension::Tree` has two variants: `Checkout`, the working tree, which an extension may read on
disk; and `Snapshot(&Snapshot)`, a tree read from git objects only, which is one commit's under
`commits` and the one git's index would commit under `check --staged`. `Snapshot::revision()` says
which. Under `check --staged` an extension is prepared with `Purpose::Check`, under `commits` with
`Purpose::Commit`, and under `index --staged` with `Purpose::Index`. `Snapshot::object_id` answers
only for a path the snapshot holds.

The index is not a third kind of tree for an extension: it is read through the same git objects as
a commit, so a third variant would make every extension write an arm reading the same objects as
the second. That departs from what `design@core@ne-minimal` asks of a new tree, that every
extension say how it reads it, and `design@core@trait-defaults` records the cost: an extension
that maps a snapshot to its commit-mode handling runs, under `--staged`, only what it runs for a
commit. thaum's rules extension lists its `changes` and `corpus` checks as not run there, and an
extension that lists nothing skips them silently. An extension that wants more under `--staged`
reads `Snapshot::revision()`.

**An extension reads the tree through the core.** Over the checkout it may read the filesystem under
the root, because a subject such as a vendored corpus is filesystem state, per
`design@core@model-then-checks`. Over a snapshot it reads git objects only, through the same
batch reader the core assembles that tree with, which also names each blob so that an extension can
cache what it parsed from one blob across commits.

### An extension scans the core's parse on its own, and the core's model carries nothing for it `##an-extension-builds-its-own-model`

The core's model holds what the walk read and the parse produced, and the observations the core's
checks read. It has no field that exists for an extension. An extension reads the parse through the core's
public API: each document's text, its prose regions and scopes, which spans are code, the Rust
names, the inert lines and the fences. It scans a second time and keys what it finds by document,
in the model it was given.

This is what keeps the core free of any one project's subject, which
`goal@knowledge-architect@any-project-can-adopt-it` requires. A model generic over an extension's
observation type would carry a type parameter into every check signature for the same data, and
a type-erased slot per document would hold it inside the core with no type checking; both are in
`path@core@docs/rejected-alternatives.md`.

**The cost is the extension's own scans.** Each document is still read once and parsed once, per
`design@core@model-then-checks`, and an extension scans the parse as often as its checks
need: thaum's rules extension scans each document in each check that reads it. Over thaum's
tree the extension's scans cost 0.07 s per `check`: the wall time of thaum's declared command's
`check`, run three times with the scan in the core and three times with it in the extension, on
one machine, compared. A measurement that grows to a share
of the run comparable with the walk and the parse reopens caching the scan inside the extension.

**An extension cannot add a kind to the entity table.** A reference kind is the core's, so an extension's
subject cannot be cited as `<kind>@<anchor>@<id>`. Nothing needs that today, and
`path@core@docs/tripwires.md` names the event that would.

### An extension claims the manifest tables it reads, and the core refuses a table nobody claims `##an-extension-claims-its-manifest-tables`

The core parses the tables it owns. Every other top-level table goes to the registered extension
that claims it. A table that no registered extension claims is a phase-1 finding, and so is a table a
registered extension claims that the manifest does not hold.

**Both refusals serve `design@core@the-regime-has-no-opt-out`.** An unclaimed table is a
declaration nothing reads, which a session would take for a regime in force. A missing claimed
table would let a manifest switch an extension off by leaving its table out. So the binary decides
which extensions run over a tree, and the manifest cannot remove one: the core binary run over a
manifest that declares thaum's `[rules]` table reports the table as unclaimed, rather than
skipping the regime in silence.

**A table keeps the extension's own name at the top level.** A namespace for extension tables is in
`path@core@docs/rejected-alternatives.md`.

### The core's commands are a library module, so a binary that registers extensions offers them unchanged `##the-core-cli-is-a-library-module`

The commands of the core, `check`, `show`, `issues`, `tripwires`, `index`, `model`, `commits` and
`install-agent-skills`, and the code that runs them, are a module of the core library rather than
of its binary. A binary that registers extensions flattens the core's command enum into its own and
adds the commands of its extensions beside them, so the core's commands keep one spelling, one set
of arguments and one exit-code contract in every binary. The core's binary is that module with no extension registered.

A binary crate cannot be depended on, so a command set held in the core's binary would be copied
into every binary that registers an extension, and the copies would drift. The helpers an
extension's own commands need from a run, such as assembling a complete working tree, are public
in the same module for the same reason.

### What a run fetches before any check is gathered once, by `Gathered`, and no caller outside the crate assembles `Inputs` `##inputs-are-gathered`

`Gathered`, in the core's command module, reads the
committed generated files, every `register.toml`, one survey, the two git batches and the shipped
agent files, and lends them as the `Inputs` a check of the working tree reads. `check`, the
commands that write files into the working tree (through `complete_working_tree`, which refuses
over an incomplete model), and an extension's tests all build on it. `check --staged` and
`index --staged` read a snapshot instead, and build their `Inputs` from it, per
`design@core@staged-tree-source`. So no caller outside the crate assembles
`Inputs`, which `design@core@ne-minimal` makes impossible for a literal. A new input is then a
change inside the core alone. The core's own mock-project tests still assemble `Inputs` by hand:
some of them hand a check an input that differs from the tree's, such as no committed file.

### The public API is a facade of role modules, one per kind of use, and every other module is private `##api-facade`

The library has three kinds of consumer: a binary that registers an extension, the extension, and
the extension's tests. **Testing an extension over a mock project is part of what the library
publishes**, because a project that adopts the tool with a subject of its own, per
`goal@knowledge-architect@any-project-can-adopt-it`, has to test that subject as the core tests
its own. `design@core@a-foreign-build-is-refused` already requires every extension binary to test
the refusal in its own suite.

Every module except the role modules is private. The crate root re-exports the nouns every
consumer meets, and the role modules hold the rest, one per kind of use, as the table below lists
them. `cli` and
`extension` are both a role module and the code of that role; `document` and `testing` only
re-export:

| module | what it serves |
| --- | --- |
| `cli` | a binary's `main`: the commands, running one, finding the project, refusing a foreign build, gathering a run's inputs |
| `extension` | writing an extension: its two traits, what the core hands them, what they return |
| `document` | reading a document's parse, per `design@core@an-extension-builds-its-own-model` |
| `testing` | running the core over a mock project as the binary does |

- **The public surface is read in one file**, lib.rs, and the role modules beside it. It is
  not computed from the implementation's module tree, so moving an item between files changes no
  public path, and is no library break under `design@knowledge-architect@versioning-policy`.
- **A public signature keeps its types public.** `Inputs` is in `Prepared::check`'s signature and
  its fields expose `Entry` and `Outside`, so those are public although no consumer names them.
- **A `pub` item no consumer can reach is refused.** lib.rs carries `#![warn(unreachable_pub)]`,
  and the clippy gate runs with `-D warnings`, so an item is public only through the facade.
- **A value only the core produces is read, not built.** `Report`, `Snapshot` and `Outside`
  have public read-only methods and no public constructor. A value a consumer starts from, such
  as a `Manifest`, a `Model` or a `Finding`, keeps its public constructors.
- **Nothing becomes public for the core's own tests.** A test that needs a private item is a
  unit-test module inside the crate, as `path@core@src/mock_projects.rs` is.
- `Manifest::command` is public although thaum's extension does not call it: an extension's
  finding that names a repair command owes the project's declared command, per
  `design@core@declared-command`.

### A public type is non-exhaustive only where no consumer can usefully match or build it `##ne-minimal`

`#[non_exhaustive]` sits on `cli::Command`, `extension::Inputs`, `extension::ExtensionReport`
and `extension::Resolution`. Every other public enum and struct is exhaustive on purpose.

| | consumer matches exhaustively | consumer uses `if let`, `==`, `matches!` or `_` |
| --- | --- | --- |
| enum without the attribute, variant added | compile error at the match | compiles, nothing changes |
| enum with the attribute, variant added | impossible: the attribute forced `_` | compiles, nothing changes |

- **Without the attribute, a new variant breaks only a consumer who chose an exhaustive match**,
  which is a consumer asking to be told. The attribute takes that choice from every consumer. The
  lint that would warn on a wildcard hiding a new variant, `non_exhaustive_omitted_patterns`, is
  unstable on the pinned toolchain.
- **For an enum the core hands an extension, the compile error is wanted.** A third
  `extension::Tree` must make every extension say how it reads it. A wildcard would read the
  working tree while the run judges another tree: a wrong verdict, with exit 0. The tree git's
  index would commit is no third kind, since it is read from the same git objects as a commit, per
  `design@core@an-extension-reads-a-snapshot`, which records the cost.
- **The attribute's gain is the library's:** a variant added to an enum without it is a breaking
  change under Rust's semver rules. Under 0.x that gain is nil, because major and minor both
  bump 0.MINOR, per `design@knowledge-architect@versioning-policy`. Every variant now foreseen
  comes with a change that already bumps 0.MINOR: a check change, a new command, a new run
  mode.
- So the attribute goes only where no consumer can usefully match or build a literal: a binary
  hands a `Command` to `cli::run` unmatched, `Inputs` comes from `Gathered`, and an extension
  fills the two reports from `Default`, which the attribute allows.
- `Generated`, `DumpRow` and `Library` stay constructible by literal. A new field there is a new
  obligation on the extension or the binary that builds the value, and the compile error says
  so.

After 1.0 the attribute decides minor against major for every added variant, so this is
re-examined before the project leaves 0.x, per `design@knowledge-architect@stays-at-zero-x`.

### A hook added to `Extension` or `Prepared` has a default body only when doing nothing is a correct answer `##trait-defaults`

The two traits have no default method. A hook added later gets a default body only when an
extension that does not know the hook is still right to do nothing, such as a new dump. A hook
whose absence would make a verdict wrong is added without a default, and adding it is a breaking
change. The argument is the one of `design@core@ne-minimal`: suppose a pre-commit mode adds
`Prepared::check_staged` with a default that returns no findings. Every existing extension
compiles unchanged, its checks silently do not run in the new mode, and the run reports success.
`check --staged` adds no hook: it hands an extension a snapshot, as `commits` does, and what an
extension runs over one is its own, per `design@core@an-extension-reads-a-snapshot`, which records
where that falls short of this argument.

### The library and the binary are one crate, `knowledge-architect` `##single-crate`

The library, imported as `knowledge_architect`, and the binary are two targets of one package. The
library already depends on clap, because its commands are a library module per
`design@core@the-core-cli-is-a-library-module`, so a separate binary crate would save its users no
dependency and would add a second package name to publish and to pin. The foreign-build refusal of
`design@core@a-foreign-build-is-refused` lists this one crate for both, beside the crate of
installed text it links.

### The binary is named `klarch` `##binary-name`

The crate is `knowledge-architect`; the binary it installs is `klarch`, as the crate ripgrep
installs `rg`. The binary's name is typed at every run, so it is short; the crate's name is typed
once, in a dependency or an install, so it is descriptive.

### Every command's exit code says whether it ran before it says what it found `##exit-code-ladder`

| code | meaning |
| ---- | ---------------------------------------------------------------------- |
| 0    | the command ran, and its subject is in order                           |
| 1    | the command ran, and reports a negative answer about its subject       |
| 2    | the command could not run: bad arguments, no project, unreadable input |

This binds the core's commands in every binary that offers them, per
`design@core@the-core-cli-is-a-library-module`. `path@core@README.md` lists which
outcome of each command maps to which code.

**The third code is what makes the other two mean anything.** A project runs `check` and
`commits` as gates, in CI and in a session, and reads the exit code without reading the output.
Without code 2, a mistyped invocation or an unreadable input is indistinguishable from a tree
that fails its check, and a gate that treats both as a failure cannot say which one to repair.

clap supplies 2 for a parse failure, per `design@core@arguments-parse-through-clap`, so
the boundary costs nothing at the parse. What the ladder costs is that a command routes its own
unreadable-input errors to 2 rather than to the code it uses for a negative answer.

### Every command's arguments are declared through clap, and every binary tests its declaration `##arguments-parse-through-clap`

The core's commands are a clap `Subcommand` enum, `knowledge_architect::cli::Command`, and a binary
that registers extensions flattens it into its own derive, per
`design@core@the-core-cli-is-a-library-module`. So clap is part of the core's public
interface, and a binary built on the core parses through clap too.

These binaries run blind, as gates, so an unknown flag or an invalid combination has to be a
refusal rather than a no-op. A hand-written parser refuses only the combinations its author
thought of. With clap, an unknown argument is refused by default, and exclusivity is a
declaration (`conflicts_with`, `requires`, `ArgGroup`) that a test checks as a whole.

**Each binary tests its own declaration**, with clap's `Command::debug_assert`, and asserts that
an unknown argument is refused. A declaration is checked only by such a test: clap finds a
malformed one at the first invocation that reaches it, not at build time.

### A parse that cannot be trusted is reported, never silent `##a-failed-parse-is-loud`

A file the walk cannot read, a source the grammar cannot parse, and a source nested deeper than
the walk goes all produce a finding naming the file. Silence would remove every citation in it
from the walk while the run reported success, which is indistinguishable from a clean file and
is the failure this tool exists to prevent.

**A file outside the walk is held to the same, for the question an extension asks of it.** An
extension may assert that no unwalked file says something, as thaum's rules extension does of a rule
number. Its bytes are handed over decoded lossily when they are not UTF-8: the question is
whether an ASCII token appears, and a lossy decoding keeps every ASCII byte, where a walked
document read lossily would be text nobody wrote. **A binary file is handed to no check**, by
git's own test, a NUL byte in its first 8000 bytes, in `check` and in `commits` alike: its bytes
are no prose anybody wrote a claim in, and a number in them, such as a PDF's page size, is data.
Every other state is a phase-2 finding naming the file, each with its own repair: a path git
lists and the working tree does not hold, a directory where git lists one entry, and a file whose
bytes cannot be had at all. A symlink and a gitlink are no file to read, and are reported as
links.

One byte of Windows-1252, such as a pasted em dash, puts a document read strictly outside the walk
and outside the inverse assertion, so a fabricated quote in it is read by nothing. Unbounded
recursion aborts the whole run with no file named and no finding printed.

### Git supplies the walk, and the manifest declares only what git tracks `##git-supplies-the-walk`

The walked set is git's listing from the manifest's directory, `git ls-files --cached --others
--exclude-standard`, with `[walk] skip-dirs`, `skip-files` and `exclude` applied after it, and the
generated indexes and the installed agent files removed by construction. Nothing generated is declared: `target` and
`path@agent-config@worktrees/` are covered by the ignore rules, so no `[walk]` row names them. A path
reference whose target the ignore rules cover is exempt from assertion the same way, per
`design@core@ignored-targets-are-not-asserted`.

**A generated index is outside the walk by construction, so its rows are read by nothing.** The
tool derives the set from the register instances rather than from a declared row, which is why no
`[walk] skip-files` row names a file-register index and a new instance cannot arrive with its
index inside the walk; an extension's generated files, such as thaum's rule index, are in the same
set, so no row names them either. Outside the walk also means outside the files the survey hands
over as unwalked: a listing is a function of the tree rather than a claim anybody wrote, so no
check reads its rows as one.

**Every remaining declaration is checkable.** A generated path cannot be asserted to exist — a
fresh clone has none of them. A manifest naming one holds a row no check can ask about: nobody
is told the row is dead, and a file later created at that path inherits what the row granted. No
generated path is declared, so every path in the manifest is one git tracks and
`check::tree` asserts each one exists. **A declared path is spelled the way git lists
it**, with no `.` segment, no `..` segment and no leading `/`: a `.` segment is dropped when
the manifest loads, so two spellings of one directory are one declaration, and a row holding
`..` or an absolute path is refused as a finding and acted on by nothing, because what it
names depends on where the manifest sits rather than on the tree.

**No ignore rule can take a tracked file out of the walk.** `--cached` is unaffected by the ignore
rules, so no ignore line, however written, can take a live document out of every check. What
leaves the walk does so by the manifest: a declared row, or an exclusion derived from a
declaration, as the generated indexes and the installed agent files are, per
`design@core@owned-namespace-check`. That is what a
hand-rolled matcher cannot promise, and it is the property the whole walk is
chosen for; the matcher and the three things it could not do are
`path@core@docs/rejected-alternatives.md`.

A tracked path the working tree does not hold — a deletion nobody has staged — stays
in the walk and is reported, because dropping it would take a live document out of every check on
the strength of a working-tree state.

**The summary block prints the walked-file count**, so two machines disagreeing about the walk is
visible in the output rather than inferred from a finding list. The tripwire is in
`path@core@docs/tripwires.md`.

**This is the working tree's walk, the one plain `check` reads.** `check --staged` and `commits`
read a snapshot instead, from git objects alone, per `design@core@staged-tree-source` and
`design@core@a-commit-message-is-a-document`.

### Every git invocation runs with the per-user ignore file pointed at the null device `##per-user-ignore-file-pinned`

**The per-user ignore file is pinned away.** `core.excludesFile` lives in a developer's home and
is no part of any project, so every invocation runs with it pointed at the null device: a line
there would otherwise take an untracked live document out of every check on one clone and not on
another. What still decides the walk beside the tree's own ignore files is git's per-clone exclude
file, which git offers no way to pin, and that is what
`tripwire@core@walked-count-differs-between-machines` is left guarding.

### A listed path is read as git's bytes, never decoded `##a-path-is-bytes`

**A path is bytes, not text.** The `-z` output is split on NUL and turned into paths byte for
byte. A name holding a byte no UTF-8 decoding accepts is legal here, and decoding it lossily
produces a path nothing on disk answers to — so the file leaves every check, and where nothing
reports the failed read the run stays green. That is the failure
`design@core@git-supplies-the-walk` exists against.

### Reading a commit's tree needs git 2.36 `##git-floor-for-commit-trees`

**Reading a commit's tree needs git 2.36.** `cat-file --batch -z` takes its requests
NUL-terminated, and a tracked filename may hold a newline: under the newline-terminated input
such a name is two requests, git answers both, and every later answer is paired with the wrong
path while the map still comes back full. An older git refuses the flag loudly, which is exit 2
with git's own reason.

### A git invocation a verdict depends on that fails is exit 2 with git's reason, never an empty walk `##failed-git-is-exit-two`

**No `git`, or no worktree, is exit 2 with the reason.** Never an empty walk: a project reported as
holding no document is a run that checked nothing and said so as a clean verdict, which is the
failure this tool exists to prevent. **Every invocation a verdict depends on fails that way** —
the listing, the tracked-and-ignored listing and the `check-ignore` batch — carrying git's own
stderr, whatever the reason. The one invocation that degrades instead is `last_changed`, which
fills a listing's convenience column and moves no verdict: where git answers nothing the column
shows `-`.

### A symlink and a gitlink are read as no document, and each is a phase-2 finding naming it `##links-are-no-documents`

Git records a symlink as mode `120000` and a submodule's gitlink as `160000`, in the index and in
every commit's tree alike, so the listing classifies an entry by mode before either reader touches
a byte and `check` and `commits` agree by construction. Read through the filesystem a symlink
yields its target's bytes, read from a tree it yields the target's path as text, and a checkout on
a platform without symlinks holds that text as a plain file, so two readers of one tree would
judge two documents; a gitlink names a commit of another repository the listing never descends
into, so everything under it would be conformant by vacuum. A `skip-files` row keeps a symlink
and an `exclude` row a submodule, as declared silences. What a submodule is to the project that
holds it is an open question, `issue@core@a-submodule-is-a-project-of-its-own`.

### A name holding a line break, or one Windows cannot create, is refused with a finding naming the file and the reason `##unportable-names-refused`

Two grounds, and the second covers more names than the first:

- A finding is one line opening with its path, an index row is one line, and a reference is one
  backticked span, so a file whose name holds a newline or a carriage return can be printed by
  nothing here and pointed at by nothing.
- Windows refuses to create a file whose path has a component holding one of `<>:"|?*\` or a
  control character below the space, ending in a space or a period, or naming a device: `CON`,
  `PRN`, `AUX`, `NUL`, `COM0` to `COM9` and `LPT0` to `LPT9`, with the superscript digits `¹²³`,
  in any case and whatever follows the first period. A tree holding one cannot be checked out
  there, and a project learns it from this gate rather than from a failed checkout.

The walk drops the file, no check reads it, and `check::tree` reports it
once with any line break escaped — `Finding` escapes a line break in the path it is located at
and in the statement it makes, so a reader taking the output by line meets one finding per line
whatever route a name took to reach it. A project that means to keep such a file names it in `skip-files` or in an ignore rule.

### A tracked file the ignore rules also cover is a phase-2 finding naming the file `##tracked-and-ignored-is-a-finding`

The two states contradict each other and the contradiction is otherwise silent: the walk reads the
file, and `git check-ignore` skips what the index holds so a reference to it is asserted too — the
reverse of what the ignore rule says. It is reported in phase 2, by `check::tree`, with the other findings
that judge what the tree holds against what the project declares. The run stops there, as for
every phase-2 finding, per `design@core@phases-gate-the-report`: the model holds a file the
project says is out of it, so a finding computed from that model is unreliable.

### Everything git answers goes through one module `##git-is-asked-through-one-module`

`path@core@src/git.rs` owns the listing, the tracked-and-ignored listing, the one `check-ignore` batch, the last-change dates, and
every read of history — `rev-list`, `rev-parse`, a commit's file list, its blobs and its message —
so the set of things this tool asks git is auditable in one read and a check keeps spawning
nothing.

### `check --staged` judges the tree git's index would commit, and plain `check` the working tree `##staged-tree-source`

`cargo klarch check` judges the working tree as git lists it, untracked files included, and prints
no `tree:` line. `cargo klarch check --staged` judges the tree
`git commit` would record now: HEAD's tree, or none before the first commit, overlaid with the
index's changes as `git diff --cached --raw` lists them, each blob read with the `cat-file` batch
`commits` reads a commit with. It is the only tree a session cannot
judge before the commit exists, and `commits` judges it only after; the motive is a partial commit,
where the working tree passes and what is committed does not.

- **The listing is not `ls-files -s`.** That lists an entry staged as intent-to-add, `git add -N`,
  at stage 0 with the empty blob, the same line as a staged empty file, and `git commit` records no
  such entry; `diff --cached` leaves it out. Measured on git 2.43.0. A path the listing does not
  hold is no path of the snapshot, a manifest staged as intent-to-add included.
- **The diff is git's, and three of its behaviours are pinned against**, each measured on git
  2.43.0 and each with a test: without `--relative` a project in a subdirectory of its repository
  meets repository-relative paths and its siblings' changes; under `--raw` only `--no-abbrev`
  gives whole blob ids; and `diff.ignoreSubmodules` or a `.gitmodules` entry's `ignore` key drops a
  staged gitlink unless `--ignore-submodules=none` overrides it. The full invocation is at
  `snapshot_entries` in `path@core@src/git.rs`.
- **An unmerged index is refused**, exit 2 naming each path once: a conflicted merge or rebase has
  no tree to commit. `git write-tree` is not used, since it writes objects into the repository and
  fails there too.
- **The summary block says which tree was walked.** `--staged` prints `tree: staged` after the
  `walk:` line, or `tree: staged (nothing staged: the tree of HEAD)` where the project's part of
  the index equals HEAD, so a run made before staging is not read as a verdict on the edits.
  Plain `check` prints no `tree:` line, so a `walk:` count CI compares with a local one is of the
  working tree on both sides.

### `check --staged` applies `check`'s rules, as on a clean checkout of the staged tree `##staged-check-semantics`

The staged installed files are compared with the running binary's shipped set, per
`design@core@owned-namespace-check`, and the files git both tracks and ignores are reported. The rival, `commits`'s rules over the
staged tree, lost because CI runs `check` on a checkout: a staged run that skips the installed
files passes a tree CI fails. Two reads stay the working tree's: the ignore rules a path reference
asks, as for `commits`, and the version pin, compared before any command per
`design@core@installed-binary-version-check`. The tripwire on a staged pass that CI then fails is
in `path@core@docs/tripwires.md`.

### `check` judges no given commit's tree; `commits` does `##check-judges-no-given-commit`

`commits <rev>~1..<rev>`, or `commits <rev>` for a root commit, judges a given commit's tree with
the tip checker, and a second judge with `check`'s rules would give one tree two verdicts, per
`design@core@one-question-one-command`.

### Every rule over a document is enforced, and no declaration exempts a document from one `##the-regime-has-no-opt-out`

The rules over documents are compiled in and the manifest declares nothing about them, so
conformance of a document means the same thing in every tree this tool checks. There is no flag, no
list and no severity: a rule either holds over a document or the run fails.

**A declared register adds obligations and removes none**, which is what keeps
`design@core@registers-are-declared` inside this head rather than an exception to it. Declaring one
gives a project a kind, a shape and a home to be checked against; it cannot loosen the ten built
in, and it cannot exempt a document from anything. A manifest that declares no register is checked
exactly as it was.

**One declaration removes obligations, and they are the agent configuration's alone.** A project
that declares no agent harness, per `design@core@agents-table`, owes no CLAUDE.md and no installed
agent file: these are documents a component carries, not rules over the documents it has. Every
document the project carries is checked exactly as under the default.

**An exemption, where an extension offers one, names FILES rather than rules.** The core offers
none. A file named is one that is leaving the tree, so the exemption expires with its subject; a
rule held back would have applied to every file and expired with nothing. That asymmetry is the
whole of it — exempting a document that is about to be deleted costs the guarantee nothing, and
exempting a rule costs it everywhere at once.

## 2. What a tree carries: anchors and registers

### Every component carries the same documents and the home of every component register `##components-carry-the-same-documents`

The components are the manifest's `[project]` list, per
`design@core@nothing-of-a-project-is-compiled-in`: the project root, named by `project.name`, and
every other project-relative directory, named by the basename of its path. Nothing is discovered by filename, so a document in a
directory that is not a component is not that kind of document: nothing counts it and nothing
reports it. A component carries `README.md`, `path@*@docs/rejected-alternatives.md`, `CLAUDE.md` while the
project serves the `claude` agent harness (per `design@core@agents-table`), and the home of every
`component`-scoped register — `design`, `goal`, `tripwire` and `issue`, and any the manifest
declares at that scope. The root also holds the plans directory, which is not a component's
document but the anchor `plans`, per `design@core@plan-register`. A component that does not carry one it owes is a finding, and
`path@core@src/check/registers.rs` is where both directions are asserted.

**Which components exist is declared, and what a component carries is compiled in.** That is
not an exception to `design@core@nothing-of-a-project-is-compiled-in`: the document list and
the four built-in component registers in `manifest.rs` belong to no project, they are what the word
_component_ means here. A project free to declare its own set would be conformant with whatever
it declared, which is the same as being checked against nothing. The one declaration that changes
the set is the agent harness, and it removes only the agent document. The flaw this leaves, that a
developer's contracts lose their required home in a project without agents, is
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`.

**A directory that carries outstanding state without being a component is a location**, per
`design@core@anchors-are-components-and-locations`.

### An anchor is a component or a location, and a location carries the registers it declares or the tool gives it `##anchors-are-components-and-locations`

Both carry registers. Every anchor is a named directory and a path anchor under
`design@core@every-path-names-its-anchor`, except the plan anchors the tool constructs, below: a
spec anchor is a file, and no plan anchor carries the `path` kind. What separates them is what they owe. A **component**
owes the compiled documents and every `component`-scoped register, with its homes under
`<component>/docs/`. A **location** owes the homes of the registers its `[locations.<name>]`
row declares and nothing else, with its homes directly under its own path — a location's
directory is already documentation, where a component's is source code.

**A location exists so that a directory outside every component can be cited.** Naming the
directory gives every entry in it a reference and makes relocating it one manifest edit, which is
what `design@core@every-path-names-its-anchor` buys everywhere else. What a location is instead of
is `path@core@docs/rejected-alternatives.md`.

**A location's own registers are the only ones it carries**, and a skill or an agent of the
project's own under one, such as this repository's under `agent-config`, is a section home of a
harness kind, which no anchor carries, per `design@core@harness-kinds`.

**Three kinds of location are constructed by the tool rather than declared**, and each is a location
in every respect but the one its layout fixes:

- **`plans`**, at docs/plans/ of the root, carrying `spec` and `milestone`, per
  `design@core@plan-register`. It is judged against the declared anchors like a location, first
  among the anchors of its depth, so a declaration that collides with it is the one refused and
  its complaint names a declaration the project can change. It owes a `README.md` beside its two
  homes.
- **One anchor per milestone**: each directory of `path@plans@milestones/` holding a file
  `README.md`, named by its basename, per `design@core@plan-document-kinds`. Its `spec` register
  has its home at the anchor's own path, so its slice specs sit beside the milestone
  document and are cited `spec@<milestone>@<slice>`, unique only inside the milestone. It carries
  no `path` kind. A directory whose name `design@core@a-plan-name-reads-as-nothing-else` refuses
  is no anchor. It carries the four item registers, per `design@core@plan-items-by-section`.
- **One anchor per spec**: each spec file of specs/, grouped or not, named by its id, per
  `design@core@spec-file-is-an-anchor`. It owns its own file, carries the four item registers,
  and carries no `path` kind. A spec whose name is refused, or that a milestone holds, is no
  anchor, and a directory is never a spec.

**The plan anchors are read off the tree, so every builder of the anchors is handed the tree's
paths**: a check the survey's listing, a command git's listing, `commits` each commit's own
listing. A milestone is a directory a session creates, not a declaration, so asking the manifest
alone would give a commit the working tree's milestones and judge its message against anchors it
never held. `manifest::collides` does not judge them, because the layout places them; the name
check is their collision check, and it is a phase-2 finding.

### No two anchors give one path two meanings: an anchor sits inside another's directory, never at its path or comparable with one of its register homes `##anchor-collisions-refused`

A location inside a component is the ordinary case, and the deepest anchor wins, so a document
under a location belongs to it and not to the component holding it. Four declarations take that rule away. Each is a complaint of the manifest's
resolution, and **the refused anchor is no anchor**: it owns nothing, carries nothing and is
asserted against nothing, per `design@core@a-wrong-declaration-is-a-finding`. Candidates are taken
shallowest first, a component before a location at equal depth, then components in declaration
order and locations in name order, so of two that collide the deeper or the later one in that
order is the one refused:

- **a component inside a location.** It nests one full register set inside a partial one and
  gives a document two candidate homes.
- **an anchor inside, or at, another anchor's register home.** Every file under it would be read
  as an entry or a subdocument of the outer register, and reported against that register and its
  anchor rather than against the declaration that caused it.
- **an anchor whose directory holds another anchor's register home** — a location at a
  component's `docs/`. Being deeper, it would own every document in that home, and each slug
  defined there would be misplaced against an anchor carrying no such register.
- **two anchors at one path.** Neither is deeper, so nothing decides which owns the documents
  under it.

The principle behind the four is that no two declarations give one path two meanings. The same
principle refuses a component register whose directory spells a compiled document's name, which
would make `path@*@docs/rejected-alternatives.md` both the document and a register's home. Outside
those shapes two anchors' homes cannot overlap: a home sits under its anchor's directory, so two
overlapping homes put one anchor's directory on the path to the other's home, which is one of the
shapes above.

### No anchor wears a kind's name `##kind-names-refused-as-anchors`

A reference's head is read as a kind before it is read as an anchor, so an anchor named `design`, `path` or `skill` would be shadowed there, and the old form
written with it would be misread. Every kind's name, the harness kinds' whatever the harness and
every declared register's included, is refused for a Component, a location, the project and a
plan, per `design@core@a-plan-name-reads-as-nothing-else` for the last. The owner, on the question:
"My ruling on this is that all kind names should be refused for anything that can be an anchor
name (components, custom locations...). I'd rather make this decision early to avoid painful
migrations."

### A component cannot opt in to an `opt-in` register `##no-component-opt-in`

No syntax exists for it, and a component that wants one declares a location under its own directory. One mechanism, and the register list
of a component stays a fact about the word _component_ rather than a per-component declaration.

### Plan documents are a structure the checker reads: the anchor `plans` carries a `spec` and a `milestone` register `##plan-register`

The plans directory holds the decided design of work that is not built yet, which no other home
holds, per `goal@knowledge-architect@design-is-recorded-with-its-arguments`. The checker reads it
as anchors and registers, so a citation of a plan is checked like any other, per
`goal@knowledge-architect@documentation-stays-consistent`: the tool constructs the anchor `plans`,
carrying two built-in registers, `spec` and `milestone`, that no project declares. `spec` is
also carried by each milestone anchor, for its slice specs, per
`design@core@anchors-are-components-and-locations`; no other anchor carries either. Inside each
plan, its threads, arguments, criteria and acceptance criteria are items, cited and checked like
any entry, per `design@core@plan-items-by-section`, so the record of a discussion is checked
before it is harvested. Plan documents read as free prose was the alternative, and it leaves
every citation of a plan unchecked and every plan document unlisted.

### The plans directory is docs/plans/ of the root, fixed by the tool `##plans-dir-fixed`

A component gets its register homes at paths the tool fixes, and a built-in register refuses a
`dir` key, so a declared plans directory would be the only declared path in that family. The tool
fixing it costs a project whose plans sit elsewhere one move, and a skill written against the fixed
path works in every project, per `goal@agent-skills@installed-text-works-anywhere`. A fixed path is the tool's convention, not a project compiled in, per
`design@core@nothing-of-a-project-is-compiled-in`.

### A project has one plans directory, in its root component `##plans-at-root`

Most work spans components, so a plan sits at the root whatever its subject: the milestone that
built the plans layout touched three, and the milestone that released version 0.1 touched every
component. Two premises are the owner's weighing: "there should never be that many planned tasks
open at once that splitting the directory would make sense", and "plans have a tendency to break
each other, when they are designed before the other is implemented", which one directory keeps in
one view. The reference grammar names the anchor, so a later split adds
anchors and rewrites no citation. `tripwire@core@plans-directory-split-asked`
watches the premise.

### The plans directory holds its README and two homes, specs/ and milestones/, and nothing else `##plans-split-dirs`

A spec is one file under specs/, and a milestone one directory under milestones/, each in a
register of its own: one register with two kinds taken from the entry's shape would be a new kind
of register, where two registers reuse the File shape and add one entry shape. Anything else
directly under docs/plans/ is a phase-2 finding, so a plan document always sits in a home and is
always listed.

### A plan document is cited by its kind, `spec` or `milestone`, and a slice spec in its milestone's anchor `##plan-document-kinds`

A spec is `spec@plans@<id>` and a milestone `milestone@plans@<id>`. Each milestone directory is an
anchor carrying `spec` for its slice specs, `spec@<milestone>@<slice>`, so a slice's name need only
be unique inside its milestone, and two milestones may each have a slice called `harvest`. The
`milestone` register's entries are directories because each is an anchor, per
`design@core@milestone-entries-are-directories`. A `path` citation of a plan document is
refused, per `design@core@every-path-names-its-anchor`: with two names for one document,
`show` would miss the citations written the other way. A whole plan document may be cited from
anywhere; when it leaves, each citation dangles, and that is the revisit it asks for.

### A plan's name reads as no kind's name, no other anchor's and no other plan's `##a-plan-name-reads-as-nothing-else`

A plan's name sits in the anchor position of a reference: a milestone's, and a spec's id, since
each spec file is an anchor. So a milestone name or a spec id that is a kind's name, a component's
name, a location's name, a reserved word, or the other home's name for another plan, is a phase-2 finding,
and a plan so named is no anchor; of a spec and a milestone of one name, the milestone keeps it. Otherwise a reference reads as the other anchor, and which one
wins depends on the order the anchors were built in. A milestone's name is also an entity id,
since it is the id of a `milestone` entry, and is not `index`: the File shape's retired single
file beside a milestone is `<id>.md`, which for `index` is the milestones home's own listing.

### A plan's items are the level-three headings of its item sections, and the section gives the kind `##plan-items-by-section`

A plan document's threads, arguments, criteria and acceptance criteria are items: a level-three
heading ending with its slug, `### <statement> ##<id>`, under the level-two section Threads,
Arguments, Criteria or Acceptance criteria, defines an item of the kind `thread`, `argument`,
`criterion` or `acceptance`, in the plan anchor that owns the document. A milestone's README and
its slice specs share one namespace, so an id defined in two of them is the duplicate finding.
Every level-three heading of those four sections owes a slug; any other level-three heading of a
plan document is section text, and a slug anywhere else in it defines nothing. Items need a model
of their own because a heading register has one home file per anchor and one level, where a plan
has one home and four kinds; an extension runs after the entity table is built, so items are the
core's, per `design@core@an-extension-plugs-in-through-phased-hooks`.

### An item is cited from inside its own plan only `##plan-item-scope`

An item citation, `<kind>@<plan>@<id>`, resolves only from a file inside the plan: the spec file, or a file of the milestone's directory. From anywhere
else, a commit message included, it is refused before its id is looked up, and the repair names
the document to cite whole: the spec, the milestone, or the slice spec that defines the item. A
whole plan may be cited from anywhere, so a dependency between plans is carried by the document,
and retiring a plan asks no other plan to be redesigned item by item. The premise is that a
whole-document citation carries every dependency between plans that matters while few plans are
open at once. `tripwire@core@item-of-another-plan-named` watches it.

### Item registers are the plan's own register shape, `Section`, whose home is the plan's documents `##items-as-section-registers`

The four item registers have the shape `Section`: each names its level-two section, its entries
sit at level three, and its home is the plan anchor's own documents, which the File and Directory
homes that hold them already judge, so a `Section` register owes no home of its own in phase 2,
no index and no walk check. The kind is read from the level-two heading in force above an item,
out of the heading records the scanner makes. Four ordinary heading registers at one home would
each claim the same file, and only the first would define anything.

### The item registers are built in, and carried by the plan anchors alone `##item-registers-built-in`

Like `spec` and `milestone`, the item registers are the tool's, because their storage is the plan
documents, which the plans layout fixes. A declaration of one is refused, per
`design@core@registers-are-declared`.

### A spec file is an anchor of its own, so a spec's items are cited by the spec's name `##spec-file-is-an-anchor`

Each spec file of specs/ is an anchor, so its items are cited `<kind>@<spec>@<id>`, as a slice's
are cited by its milestone's name, in every project that writes specs. The plan anchor stands in
the anchor position, so an item citation keeps the three-part grammar, shorter than the root
anchor and a compound id by the root anchor's name and one separator. The anchor holds its own
file, which stays an entry of the `spec` register that holds it. Making every spec a directory was
the rival; it reopens the decision that a spec is one file.

### A plan document owes the sections of its kind, and a slice spec owes its own `##slice-spec-sections`

A spec of specs/ and a milestone's README owe the plan sections, in order, with Arguments right
after Threads, matched as the documents write them: Status and audience, How the work is done,
Names, What the work is, What is already decided, Criteria, Threads, Arguments, "New names, in one
place", Decided design, Mapping tables, Losing alternatives, Readings, Premortem, Acceptance
criteria, Implementation sequence, Order rationale, Defaults awaiting the owner, Harvest, Later
consequences. A slice spec owes Builds, Claims, Audit subjects, Fails alone on and Premises that
expire, in order; the plan sections a slice spec holds, for the design only its slice builds, are
not ordered against them. Fixtures is owed only where a Component drives its tests with authored
content, a condition no check can read, so it is not checked. One `spec` register owes two lists,
chosen by the anchor that holds the spec.
The two lists are the planning skill's, which owns the shape of a plan document, per
`design@agent-skills@design-hands-off-to-planning`; a change to them changes this head.

### Which registers exist beyond the built-in ones is the manifest's `##registers-are-declared`

`[registers.<name>]` declares a register's `scope`, `shape`, `dir`, `level`, `sections` and
`metadata.<key>.values`; `[locations.<name>]` declares a directory and the registers it carries.
Ten registers are compiled in — `design`, `goal`, `tripwire`, `issue`, carried by every
component; `spec` and `milestone`, carried by the anchor `plans`, `spec` also by each milestone
anchor, per `design@core@plan-register`; and the four item registers, carried by the plan
anchors, per `design@core@item-registers-built-in`. A declaration for one of the first four accepts `kinds` on `issue`
and nothing else, because their storage is what the word component means. A table for a plan
register or an item register is refused whole, and so is any of their names in a location's
`registers` list, and a
component register whose home at the root would be the plans directory: the plans layout is the
tool's, and a declaration that changes it changes nothing a project can rely on. A declared
register named `path`, `planned` or a harness kind is refused too: a reference whose kind segment
is that word never reaches the register. Setting `scope`, `shape`, `dir`, `level`, `sections` or `metadata` on a
built-in is a finding, and the compiled value stands.

**Declarations go in the manifest because its header promises that what is checked is said in one
place.** Its bulk is comments, which is what a declaration owes a reader.

### A register instance's own options sit beside it, in `register.toml` `##instance-options-beside-the-instance`

Per-instance options go in `docs/<dir>/register.toml`. They cannot sit in
the manifest without a table keyed by anchor, and the shape that avoids the table — a single list
at the root — would put one component's groups in another's directory. Today the only such option
is `groups`. A key other than `groups` is a finding, a file with no `groups` is a finding, and a
`register.toml` beside a heading register is one too.

### A declaration that is wrong is a phase-1 finding, and absent from the configuration `##a-wrong-declaration-is-a-finding`

A declaration that is wrong is a finding rather than a load failure, and it is absent from the
configuration. A manifest that will not load reports nothing at all, and nothing at all is what
a session reads as conformance. So `Manifest::parse` resolves the declaration into a
configuration and a list of complaints, each in the shape a finding takes, and every complaint
is the first phase of a run, per `design@core@phases-gate-the-report`. What a complaint is
about is not in the configuration: a refused register is no register, a refused anchor is no
anchor, a refused row is not in its list, so nothing acts on it and its consequences are never
reported as defects of the tree. An extension's tables follow the same rule, per
`design@core@an-extension-claims-its-manifest-tables`: its complaints are phase-1 findings, and a
run that holds one stops before the extension reads anything.

### A heading register's home is `<dir>.md` or a `<dir>/` directory, never both `##heading-register-two-shapes`

The single file serves an anchor whose register fits one document. The directory serves one whose
register has outgrown it. In the directory shape, `README.md` is the head: an introduction, and a
bullet list of markdown links, one per subdocument, each target written relative to the README.
The entries and their slug anchors live in the subdocuments.

**Both shapes serve every heading register**, so `design`, `goal` and `tripwire` accept the same
two, and so does any heading register a project declares.

Five assertions carry the shape, split over three places. `check::tree` asserts exactly one
home per anchor and register and that a directory home carries its `README.md`;
`check::registers` asserts that the README links every markdown subdocument the walk covers — a gitignored scratch file owes nothing. That
each relative link resolves is the references family's assertion, made for every navigation file
alike per `design@core@links-are-navigation-rows`. The entity table of `design@core@one-entity-table`
accepts a slug definition only in the file home or in a subdocument, matched against the owning
anchor's own paths rather than against a filename suffix — a suffix match would accept a slug
in any file whose name ends in `design.md`, a plan document included.

**Exactly one home, because two give a decision two candidate places to land.** That is the
one-home failure, and the reader who finds one half acts on half the design.

**The README must link every subdocument, because a subdocument nobody links is a home nobody
finds.** A markdown link is the index's row because a reader can follow it where a renderer
shows the page, and a bullet list is its conventional shape — a table grows painful to edit as
soon as a row outgrows a short sentence. Both directions are asserted over the same links: an
existing subdocument no link names is reported, and a link that resolves to nothing is reported.
A fenced link is an illustration and discharges nothing, the stance a fenced definition takes
per `design@core@a-slug-elsewhere-defines-nothing`, and a link inside a code span is typography showing the
shape. **A row is any link shape `design@core@links-are-navigation-rows` reads**, a link
definition included, so a README written with reference-style links indexes its subdocuments
like one written with inline links.
A backticked path in the README stays what it is everywhere: a checked pointer in prose, not a
row of the index.

**The README defines no slugs.** It is the head and the index. A decision recorded there competes
with the subdocuments as a home, which is what the split exists to end.

### A file register is a directory of one file per entry, with a hand-written README and a generated index `##a-file-register-is-a-directory-of-entries`

Its home is `<dir>/`. Every `.md` file under it is an entry named by its basename, except
`README.md` and `index.md` at the instance's own top level; a subdirectory is a declared group
and nothing else, groups nest one level, and a file of another suffix is a finding. An entry
carries frontmatter with each declared metadata key, a level-one title, the declared level-two
sections in order, and — for the issue register — the level-three subsections its kind owes.

**One file per entry, because a tool that edits a tracker edits the whole file.** Sessions editing
a shared tracker with `sed` and a string replacement have replaced every occurrence in the file
more than once. One file per entry makes that failure unreachable across entries, gives each entry
its own history, and turns closing an entry into a dangling-reference work list: the file is
deleted, and every reference to it becomes a finding.

**The README is hand-written and the index is generated, and neither stands in for the other.**
The introduction is prose about the project and belongs in markdown, and the README is also what
keeps the directory in git, which an empty directory cannot be. The index is mandatory: with the
README already holding the directory open, the churn argument against generating one has no weight
left, and a listing nobody generates is one that silently stops listing. This head owns that the
index exists; what it holds is `design@core@a-file-register-index-is-rows`, and that it is generated
rather than written by hand is `design@core@generated-files-are-pure`.

### A file register's groups are subdirectories, declared beside the instance `##groups-are-subdirectories`

A subdirectory is visible to `ls` and to a listing without parsing anything, and the group is not
part of an entry's id, so regrouping is a `git mv` that breaks no reference. The declared list is what keeps the naming from
drifting, and it is checked in both directions. An entry may sit ungrouped at the top level, so a
two-entry instance owes no directory.

### An entry's frontmatter is a small subset, refused loudly outside it `##frontmatter-subset`

**The frontmatter subset is small and refused loudly outside itself**, per
`design@core@a-failed-parse-is-loud`: a block opened and closed by a line holding only `---`, at the
very top of the file, holding `key: value` lines with scalar values, no key written twice. A key
the register does not declare is a finding, so a typo in a key name cannot pass as an absent
optional. Frontmatter is required exactly when the register declares metadata, and refused when it
declares none — an undeclared block is metadata nothing checks.

**A blank line between the delimiters means there is no block**, which is what tells a block from
a document opening on a thematic break: a paragraph between two thematic breaks has blank lines
around it, and a block has none.

### An entry's frontmatter block is read as prose, outside the document's structure `##frontmatter-is-prose`

**The block is prose, entire, and its lines are marked as a fence.** That marking keeps a value
out of the document's structure — a heading, a slug definition, a navigation link — and buys
nothing else: a fence has never made its contents data here, per
`design@core@grammars-not-prefixes`. So a reference in a value is a reference, and an
extension that reads its own markers in prose reads them in a value too.

**The block is blanked byte for byte before the structure is read.** The closing `---` is a setext
underline for the `key: value` lines above it, so left in place a block would open the document
with a level-two heading nobody wrote. Each byte becomes a space rather than being removed, so
every offset the analysis returns is an offset into the file as written.

### The issue register's kind list is closed `##issue-kind-list-closed`

An unknown kind is a finding naming the list, and adding one is a reviewed manifest diff. The kind
decides which subsections the entry owes, so an unknown one would owe the wrong three in silence, which is what makes refusing it load-bearing
rather than tidy. The open set is in `path@core@docs/rejected-alternatives.md`, and
`path@core@docs/tripwires.md` guards the closing.

### A milestone entry is a directory, the register shape `Directory` `##milestone-entries-are-directories`

One register has entries that are directories: `milestone`, the shape `Directory`, per
`design@core@plan-document-kinds`. Its home holds a hand-written `README.md`, a generated
`index.md`, and one directory per entry, each holding a file `README.md`, which is the entry's
document and owes a level-one title. Nothing else sits in the home, and no entry holds a
`register.toml` or a subdirectory; each of these is a phase-2 finding, because the entries are
anchors read off the tree. An entry is defined at its `README.md`, and only while the walk reads
that file, as a File entry the walk leaves out defines nothing. The shape exists because the owner
asked to cite a milestone as `milestone@plans@<id>`, and a milestone is a directory: a File entry
is one file and could hold no slice specs. Citing the milestone by a path to its README needs no
new shape, and loses to that request.

### A file register's index is a banner, a count and one row per entry, and its bytes are the contract `##a-file-register-index-is-rows`

The generated `index.md` of a file-register instance, in order:

| line | content |
| --- | --- |
| 1 | `**Generated — do not edit.** \`<command> index\``, with the project's declared command, per `design@core@declared-command` |
| 2 | blank |
| 3 | `<n> entries`, over every entry of the instance, grouped or not, and `1 entry` where the instance holds one |
| then | the ungrouped entries' table, under no heading |
| then | per group, a blank line, `## <group>`, and that group's table |

A table's columns are the register's declared metadata keys, **ordered by name**, then `title`
holding the entry's level-one heading as a link to the file, relative to the index's own
directory. Rows sort by the first metadata key and then by id, or by id alone where the register
declares none. Groups sort by name, and the ungrouped entries come first.

**A `|` inside a cell is escaped as `\|`.** It is the one character of a title or of a metadata
value that would otherwise end the cell and shift every column right of it, turning a listing into
a table that says something else. Nothing else in a cell is rewritten.

**The bytes are a contract because `generated` compares bytes.** Any change here fails the gate on
every committed index at once, which is the cost of the comparison being exact; the gain is that a
listing cannot drift by a character without being named.

**Rows only: no summary and no date.** A row changes on create, delete, retitle, regroup and a
metadata change, and on nothing else, so an index is regenerated rarely. A summary column would
restale it on every wording edit, and a date would break
`design@core@generated-files-are-pure` outright. Both are in `path@core@docs/rejected-alternatives.md`.

**A Directory register's index has the same bytes with one column before the title, `id`**: one
row per entry, its id, then its README's level-one title linking `<id>/README.md`, sorted by id. Its rows come from the
entries, which are anchors, so no file inside an entry is a row; the entry's own index lists those.

**The rows come from the walked entries, not from the instance's declarations.** A group is the
subdirectory an entry sits in, so an undeclared group still lists its entries and a declared group
holding none prints no heading; an entry whose id, title or metadata the shape check refuses is
still listed, under its basename and an empty cell. A listing that hid what the directory holds
because a declaration disagreed would be a listing that stops listing, and the declaration is
`check::registers`' to report.

### A generated file's content is a function of the walked tree, so `index` writes it with no dry run `##generated-files-are-pure`

No generated file holds hand-written bytes: not a file-register `index.md`, and not a file an
extension generates. Each is what its generator returns over the model, which is what lets
`path@core@src/check/generated.rs` verify one by regenerating into a `String` and
comparing rather than by writing the file and reading it back.

**Writing one destroys nothing**, so `cargo klarch index` takes no dry-run flag. Its one flag,
`--staged`, chooses the tree and the destination, per `design@core@index-staged-write`. The dry run a write
command normally owes exists because a write can lose something, and here it cannot: the worst a
run can do is replace a file with what the tree says that file is. It writes only where the bytes
differ, so running it to look moves not even an mtime, and it names each file it rewrote.

**That last claim is held by two refusals, not by the paths being generated.** `fs::write` follows
a symlink and writes through it, so a generated path that is one would replace content this
command never produced; and a destination directory the manifest declares but the tree does not
have is a manifest defect rather than something to create. Both are checked over **every**
destination before any is written, so a run either refuses having written nothing or writes them
all. Remove either check and the paragraph above stops being true.

**The purity is a property to preserve, not one to observe.** A generator that embedded a
timestamp, a hostname, or anything the walk does not see would break both consequences at once: the
check would report a file stale that nobody had changed, and `index` would rewrite on every run.

### The writer and the `generated` check take the generated files from one list `##one-list-of-generated-files`

**One list names the file-register indexes, and the writer and the gate both read it.**
`cargo klarch index` and the `generated` check take the same pairs of destination and expected
bytes from one function, so neither can generate one the other does not know about, nor disagree
about what is in it. Each is derived from the file-register instances, one per instance whose
directory is there. **An extension's generated files are read the same way**: the extension
names their paths before the walk and renders their bytes when prepared, and the writer and the
gate both take them from those two calls, per
`design@core@an-extension-plugs-in-through-phased-hooks`.

**An instance with no directory contributes no index**: generating into it would create a register home as a side effect of a listing, and the
missing home is what `check::tree` reports.

### One question has one command that answers it `##one-question-one-command`

Verifying is not `index`'s question. The `generated` check of `cargo klarch check` is the gate,
and it names the first line at which the committed file and the regenerated one disagree. A second
command answering the same question in its own format is what drifts, and there would be nothing to
say which of the two was right.

### `index --staged` writes the staged tree's generated files into git's index, and no working-tree file `##index-staged-write`

`cargo klarch index --staged` generates each file from the tree git's index would commit, per
`design@core@staged-tree-source`, and sets the staged entry of each whose staged bytes differ, or
that the index lacks, leaving the working-tree file as it is. After it, `check --staged` passes on
what will be committed, and plain `check` judges the working tree as before. It serves a commit
of part of the working tree when an unstaged change touches the same register: the `index.md` that
commit needs then differs from what `index` writes, and without this command the repairs are a
hand edit of a generated file or moving the unstaged file away.

- It gates the staged tree on phases 1 to 3, per `design@core@phases-gate-the-report`, and refuses,
  having staged nothing, a destination the index holds as a symlink, a gitlink or a directory, asked
  of the index itself so that no walk row hides it, or whose directory the staged tree does not
  hold. A symlink's or a gitlink's mode would carry the generated text, and `--index-info` would
  replace the entries under a directory.
- It stages every entry in one locked write, so the index changes for all of them or for none: a
  held lock is exit 2 with the index as it was. How, and what a refused run can leave in the object
  store, is the doc comment of `stage_generated` in `path@core@src/git.rs`.

It writes into git, where `design@core@safe-fix-definition` refuses a fix that touches git. That
test is the one `--fix` applies before a fix, and `--fix` never applies this one, per
`design@core@fix-refusal-mixed-state`. What it writes is the staged entry of a generated file,
whose bytes the staged tree determines, so no bytes a writer meant are lost. The rival, writing
the staged rows into the working-tree file, lost: after it, plain `check` fails, since that file no
longer matches the working tree, and `check --staged` fails until the file is staged.

### `check --fix` applies the safe fixes `design@core@fix-scope` admits, then runs the full check `##check-fix-flag`

`cargo klarch check --fix` applies the safe fixes `design@core@fix-scope` admits, lists each file it
wrote or removed, then runs the check, whose report and exit code are the run's. It makes an edit cycle
one command: updating the generated files is needed after nearly every edit of a register, and
without the option it takes `index` then `check`. The option came from the owner: "it's quite
common for tools similar as this one to provide a "quick fix" option, that performs the fixes it
can safely do automattically", as `cargo clippy --fix` and `eslint --fix` do, and the same words
lean "toward using a generic name for the cli option". Without `--fix`, `check`
is unchanged, and `index` and `install-agent-skills` stay as commands, each for one fix alone. The
option takes a generic name because a later safe fix needs no new option. The rival, a separate
`fix` command that writes and then runs the check, lost: with the option, the check stays one
command rather than two that end in one report. `commits` judges history and takes no `--fix`,
and the gates run `check` without it, so continuous integration judges the tree as committed. It
refuses a partial commit's mismatch, per `design@core@fix-refusal-mixed-state`, and `--staged`,
per `design@core@fix-with-staged`.

### `check --fix` refuses where a generated file it would write differs from the staged tree's, not wherever the tree holds staged and unstaged changes `##fix-refusal-mixed-state`

**Where the project's part of the index differs from HEAD, and a generated file `--fix` would
write differs from the one the tree git's index would commit needs, `--fix` writes no generated
file.** It names each such path, a path the staged tree does not need included, with the two
repairs: `index --staged` then `check --staged`, to commit the staged changes, and `index`,
to fix the working tree as a whole. `--fix` cannot lose unstaged content, since it writes only
generated and installed files; the hazard is a wrong commit, since the `index.md` it writes
reflects the working tree, and staged with a partial commit it makes a commit whose tree fails. A
file already current on disk is not written, so no refusal names it: the hazard is in what `--fix`
writes, and refusing a run that writes nothing would refuse one that `index`, the repair it names,
cannot clear. Where no staged tree is there to commit, an unmerged index or a staged tree stopped
in phases 1 to 3, nothing is compared and the fix runs. Any other failure to compute the staged
side refuses, naming it, such as an extension that cannot prepare over a snapshot: a comparison
skipped in silence lets the partial commit through, per `design@core@a-failed-parse-is-loud`. The
refusal is exit 2, or 1 where agent files were installed before it, since 2 promises an untouched
tree.

A refusal wherever the tree holds both staged and unstaged changes loses: it also fires where
the unstaged changes touch no input of a generated file and the fix is correct for both trees,
which blocks the edit-then-`--fix` loop for nothing. A printed note in place of the refusal loses: a
line in a long report is missed where an exit code is not.

### `--fix` with `--staged` is refused while parsing `##fix-with-staged`

The refusal is clap's `conflicts_with`, per `design@core@arguments-parse-through-clap`. `--fix`
repairs the working tree. Combining them would tie every fix to a write into git's index. The
owner's reason: "I'd like to provide more types of quick fixes later, and this might not be
compatible."

### A fix is safe when its bytes are determined by the tree and the pinned version, and it writes or removes only files of the installer's namespace or of the generated list `##safe-fix-definition`

This is the test every fix must pass before `--fix` applies it. The installer's namespace counts
whole, per `design@core@owned-namespace-check`: an unshipped file the check reports there is
removed, as the install removes it, whoever put it there. The rival, removing only the files the
install itself wrote, has nothing to tell them by: ownership is decided by a name, with no record
and no history, per that head. A fix that makes a choice, or touches
git or a hand-written file, would rewrite what a writer meant, so its repair stays the reader's.
The test is what keeps `--fix` safe to run after every edit, which serves
`goal@knowledge-architect@agents-work-without-drift`. `tripwire@core@fix-makes-a-choice` watches
it.

### `--fix` admits a repair only when it passes the safe-fix test `##fix-scope`

`--fix` applies a repair only when it passes `design@core@safe-fix-definition`. Passing the test
makes a repair admissible, not owed: each kind of repair `--fix` applies is a member added on
purpose. The direction of the rule is the owner's, given against a title stating that `--fix`
repairs every finding whose repair passes the test: "I'd word it the other way: '--fix only admit
repairs when they pass the safe-fix test'. Otherwise, it becomes a source of defects when we might
have never intended to cover some possible repairs."

`--fix` applies two kinds of repair today: an installed file missing, differing or no longer
shipped, and a stale or missing generated file. Every other repair touches git, such as staging a
deletion, or a hand-written file, such as the root CLAUDE.md's primer import, or is a choice. The
installed set is repaired exactly where the installed-file check reports it, judged from
git's listing after normalising line endings: a file git does not list, such as an ignored file in
the namespace, is never touched. The generated files alone, the rival, lost: repairing the installed set costs little, and it removes the
install-then-index sequence from an upgrade.

### A fix run repairs the installed files, gates the model, refuses a partial commit's mismatch, writes the generated files, then checks `##fix-before-the-checks`

The order of a `--fix` run: `--fix` with `--staged` is refused while parsing; a manifest holding a
refused declaration writes nothing; the installed files are repaired; phases 1 to 3 run over the
tree as the repairs left it, and a finding there stops the run, printed as `check` prints a stop,
with nothing more written; a partial commit's mismatch is refused, per
`design@core@fix-refusal-mixed-state`, after the installed repairs, whose bytes are the same for
both trees, and after the gate, whose model the comparison reads; the generated files are
written, from the list `index` reads, after every destination is checked; the full check runs. The
installed files' bytes do not depend on the model, so they are repaired before the gate, while a
generated file is never written over an incomplete model, per `design@core@phases-gate-the-report`.
An upgrade that removes a shipped file takes two runs: the deletion is unstaged, phase 2 reports
it, and staging it touches git. The rival, `--fix` treating the deletions its own install just made
as done, adds a special case to the phase gate. The cost of two runs is small by the owner's
weighing: "skill deletion/rename is a rare thing anyway. Not much of a problem if it takes multiple
invocations to handle." A failed
write exits 2 with nothing written, and 1 after any write, since 2 promises an untouched tree. The check stays the only verifier: `--fix` only writes
before it verifies, so the rejected alternative of an `index` that prints a diff and a `--write`
that applies it, a second command answering whether a file is current in its own format, is not
reopened.

## 3. Names and references

### Every citeable thing is an entity with a kind, an id, a definition site and, where its kind takes one, an anchor, held in one table built from the model, and every check that resolves a name reads that table `##one-entity-table`

**A kind is a register's name, or `path`, or `planned`, or, under the `claude` harness, a harness
kind**: `skill`, `agent`, `primer` or `instructions`, which is no register and takes no anchor, per
`design@core@harness-kinds`. The ten built-in registers give the kinds `design`,
`goal`, `tripwire`, `issue`, `spec`, `milestone`, `thread`, `argument`, `criterion` and
`acceptance`, and a project's own declarations give the rest, so the kind set is
data rather than a compiled enumeration. A heading register's entities are slugs defined in the
register's home under an anchor, per `design@core@an-entry-is-a-heading-at-the-register-level`; a file register's are the
files under its instance directory, one per entry, per
`design@core@a-file-register-is-a-directory-of-entries`. `path` is defined by the tree itself and is
resolved against the survey, under the same anchors and with the candidate rule and segmentation
the table's resolver applies to every kind, and `planned` with it, per
`design@core@planned-path-form`. The table lives in
`path@core@src/entity.rs`, is built once per run from the model, and is what
`check::references` resolves against.

**An anchor is a named directory carrying registers**, per
`design@core@anchors-are-components-and-locations`. It carries its register list and its home base
as data, so a component and a location are two constructors and not two resolvers. The deepest
anchor owns a document, as `design@core@an-entity-belongs-to-its-anchor` states.

**One table, because a notion of a name held per check drifts.** With one table there is one
resolver, one dangling check over every kind, one command that can print any entity given its
reference, and a one-pass rewrite of every reference into a link when the documents are
published. A check that resolves a name of its own is the shape this refuses: two resolvers
disagree the first time one of them is edited, and only the entities one of them knows can be
printed or rewritten.

### Each heading register declares the one heading level its entries sit at, and every heading at that level in its home carries a slug `##an-entry-is-a-heading-at-the-register-level`

**The level is part of the register.** `design` sits at level three, `goal` and `tripwire` at
level two, compiled in; a declared heading register states its level with the `level` key of its
`[registers.<name>]` table, from 2 to 6, and a file register takes none. Level one is the
document's title. With a declared level, "every heading at level N is an entry, and only those"
is a checked invariant, so the tool can tell a heading that is no entry from an entry whose slug
is missing. Without it, a tripwire written without a slug is defined nowhere, listed by nothing
and reported by nothing.

A heading at another level is section text and owes nothing: a design home groups its
level-three decisions under level-two subjects.

**The homes are the design, goals and tripwires homes of the owning anchor**, and the home of
any heading register the project declares, each in either shape of
`design@core@heading-register-two-shapes`: the single file, or a subdocument of the directory.
The directory's `README.md` is the head: it defines nothing and owes no slug, for every register
and not only design. A plan's items are the one register family read otherwise: their home is the
plan's own documents, a milestone's README included, and the section a heading sits under gives
its kind, per `design@core@items-as-section-registers`. Which anchor owns a definition is where its document sits, per
`design@core@an-entity-belongs-to-its-anchor`.

**The section homes of the harness keep the same rule at level two**, a skill's, an agent's, the
primer and the root CLAUDE.md, per `design@core@section-homes-carry-slugs`: their entries are
the harness kinds', not a register's, and a slug at a level-two heading there defines a section.

A heading at the register's level with no slug in its home is reported, naming the heading. An id
is `[a-z0-9]+(-[a-z0-9]+)*`.

Two definitions of one id in one register instance are a finding at each site, each naming the
other.

**Which homes an anchor must carry is `check::tree`'s question, not the table's.** The
table defines from every home shape of every heading register the anchor carries, and reports a
definition that sits where none may; whether the home is there at all, and in which shape, is
`check::tree`'s.

### An entry's heading puts its statement before its slug, and the slug is read wherever it sits `##statement-precedes-the-slug`

**The statement precedes the slug in the heading**, so a document outline lists the entries
rather than a set of identifiers, and an editor's outline view is the index.

**The slug may sit anywhere in the heading.** A pattern that requires text after the slug matches
no heading carrying nothing but the slug, and every reference to such an anchor is then reported
as dangling while the definition sits in the file.

### A slug anywhere but at a heading of its register's level in the register's home defines nothing, and is reported as a misplaced definition `##a-slug-elsewhere-defines-nothing`

A slug anywhere but at a heading of its register's level in the register's home defines nothing
and is reported as a misplaced definition: at a heading of another level, level one included, in a
table cell, at the head of a plain line, in the middle of a line, as a second slug on a definition
line, in a file that is no register home — a Rust comment included — or in a directory home's
README.

- **A table cell defines nothing.** A decision written as a row has no heading, so the outline does
  not list every decision, and one register would have two definition sites.
- **A slug at the head of a plain line is reported, not accepted.** It is neither a definition
  nor a reference, so every pointer at it dangles and the site itself is named. Accepting it
  beside the heading form would leave the two indistinguishable, and nothing would say which
  anchors use which.
- **A mid-line slug is a pointer written in the definition form**, and recording it is what makes
  such a pointer visible: a census found such pointers in thaum's design homes, checked by
  nothing, in the message of 115f8ae.

**A fenced heading is an illustration, so a fenced slug neither defines nor is misplaced.** A
definition site is a heading, and the scanner already reads no heading inside a fence; the one
stance covers both. This is the one place a fence still suppresses anything in the grammar —
references are live in a fence for every kind, per `design@core@candidate-rule-and-retired-forms`.

### A heading is a line that opens with `#` marks, and a heading markdown reads in any other shape is reported `##headings-open-with-hash-marks`

The checker reads a heading where markdown reads an ATX heading at the start of a line: one to six
`#` marks, at most three spaces in, then a space or a tab, then text. A line indented four spaces
or more, or by a tab, opens no heading, so a slug on it is a mention and defines nothing; so does
a line whose marks are followed by any other character, a no-break space included. Every other
heading markdown reads is reported in phase 2, at its first line, in every markdown document of
the walk: a setext heading, a line underlined with `=` or `-`; a heading that follows a list
marker or a block-quote marker on its line; and a heading with no text. The repair is to write it
as a line that opens with `#` marks and holds text.

The opposite direction does not hold yet: a heading-shaped line inside an HTML block or a metadata
block that markdown does not render as a heading is still read as one, per
`issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`.

- **It serves `design@core@an-entry-is-a-heading-at-the-register-level`**, which rests on the
  checker reading headings as markdown does: an entry is a heading. A heading markdown shows and
  the checker does not read is a section no check sees. A slug on it defines nothing, and in a plan
  document the items under it are read under the section before it, which changes their kind.
- **Phase 2, because the model is incomplete**, per `design@core@phases-gate-the-report`. A
  finding of the entity table computed while such a heading stands is unreliable in both
  directions.
- **Reported, not read.** The scanner reads one line at a time. A setext heading's level is known
  only at its underline, which follows its last text line, and a heading inside a container starts
  after the container's markers. One shape per heading keeps the scanner line by line, and keeps
  every heading findable by a search for lines opening with `#`.
- **A Rust comment holds no section**, so a comment line underlined with dashes is not reported.

### An entity of a register belongs to its anchor and is unique inside one register instance, and a reference to it names all three: `` `<kind>@<anchor>@<id>` `` `##an-entity-belongs-to-its-anchor`

A definition is owned by where its document sits — the deepest anchor whose path holds it, and
the register whose home holds it — rather than by anything the line says, so moving a document
moves the entities in it. Two anchors may therefore each record an entity they call the same
word, and two registers of one anchor may too, which is what naming the kind and the anchor in a
reference buys. The id of a heading-register entity is called a slug.

**A harness kind names no anchor**, per `design@core@harness-kinds-cited-without-anchor`: its entities live
where the harness puts them, one namespace per kind, so an anchor would carry no information. The
refusal of a reference that names no anchor holds for every other kind.

### A reference that resolves to nothing is reported, with the repair its failing segment needs `##an-unresolved-reference-is-reported`

**A reference resolves to nothing in five ways, and each is reported as the repair it needs.**
The kind position holds an anchor, the old form, and the repair names the kinds; the anchor is
not declared, and the repair lists the anchors; the anchor is declared and carries no register of
that kind, and the repair lists the anchors that do; an item of a plan is cited from outside that
plan, and the repair names the whole document, per `design@core@plan-item-scope`; the anchor
carries the register and does not define that id, and the reference is dangling. One finding for
all five would send a reader to check the wrong segment of the pointer most of the time. Every
anchor but a plan anchor carries `path`, and a component carries every component register, so the
third way is reachable only by a location, declared or constructed by the tool.

**A reference that resolves to nothing is recorded and reported, never dropped.** The scanner
could require a resolvable shape and see nothing without one, which needs no finding and no
migration. It would also mean every pointer written in an older form stops being checked with
nothing saying so, and a silent false negative is the failure this tool exists to prevent. The
candidate rule that bounds this is `design@core@candidate-rule-and-retired-forms`.

### A backticked `@` span is a reference candidate when its head is a kind or an anchor, the retired slug reference stays a finding where it names something of this project, and every other `@` span is silent `##candidate-rule-and-retired-forms`

The scanner records every backticked span that holds an `@` and no whitespace, backtick or
angle bracket, as written; it has no manifest, so it cannot tell a kind from an email address.
The resolver reads the head — the text before the first `@` — and decides: a known kind is
segmented and resolved; a declared anchor or a reserved anchor in that position is the old form
and is reported with the repair "prefix the kind"; anything else is not a reference and reports
nothing, so an email address or a git remote in backticks is silent unless the project declares
an anchor by that word. Every kind but `path`, `planned` and the harness kinds takes exactly three
non-empty segments; `path` and `planned` take an anchor and then everything after the second `@`;
a harness kind takes the arities of `design@core@harness-kinds-cited-without-anchor`.

**The silence is bounded and named.** A typo inside the kind, `desing@<anchor>@<id>`, is silent under this
rule, because widening it to "any span with two `@`" would report every email address with a
plus tag. `path@core@docs/tripwires.md` guards the gap: a review finding a reference the scanner
reported nothing for widens the rule to the shape found.

**A span is one line.** A backticked span that opens on one line and closes on the next is a
finding when its two halves, joined, would be a reference candidate or a path, and the half
before the break holds an `@` or a `/`: read line by line, neither half is a closed span, so
the pointer would be resolved by nothing, and a renderer shows the line break as a space inside
it. A wrapped span that is no pointer, such as a command or a clause in backticks, is prose and
reports nothing. The backtick runs are read as CommonMark reads them: a span closes on a run of
the length that opened it, a backslash escapes a backtick outside a span, and a blank line ends
an open span. **This reading only adds findings.** Every other pattern reads each line as
written, so a line whose backticks are misread can gain a wrong finding and cannot lose a
reference or a path it held.

**A fullwidth at sign is not an `@`.** A span written with `＠` is no candidate, and nothing
reports it. Reading lookalike characters as the grammar's would put every script's
punctuation inside the tokenizer, and a census of walked markdown finds no such span outside this
paragraph; re-take it with `git grep -n --untracked '＠' -- '*.md'`, and a span meant as a
reference reopens the silence.

**The retired slug reference is a finding wherever it names something of this project.** A
backticked `<word>#<id>` or a bare `#<id>` names the form it was when its id is an entry some
register defines, a harness kind's names and slugs excluded, since that form never cited one and
a link to a heading in another tool's notation spells its slug so, and a qualified one also when its word is an anchor or a kind. The id is asked
whatever the word, because a form copied out of the history may carry the name an anchor had
before a rename, such as the core's former name `knowledge`. It has no `@` and no two path
segments, so without this clause a slug reference the migration missed would be silent, which is
the founding failure class. The clause does not expire with the migration: the commit history is
read by every session that runs `git log`, it holds the form, and a reference copied out of it
would be checked by nothing.

**A span with no `@` is read by three lints, and by nothing else**: a path-shaped span, per
`design@core@every-path-names-its-anchor`, the retired slug reference above, and the exact name of
a skill or an agent, per `design@core@bare-skill-name-reported`.

**A shape that names nothing here is another tool's notation, and is silent**: an issue number
`#123`, a preprocessor directive `#include`, a crate's item `serde#derive`. None was ever this
grammar's, so none is a reference a migration missed, and a project that never used the form would
otherwise have to rewrite its own notation to pass. What the silence costs is a copied form whose
entry has since left the tree and whose word is no anchor: it names nothing here, so it is not
reported. Re-take it as the path lint's census is re-taken, counting this lint's findings; a
census in which the silence hides a pointer the migration missed reopens it. Reporting every such
shape is in `path@core@docs/rejected-alternatives.md`. This serves `goal@knowledge-architect@any-project-can-adopt-it`.

**The interpretation register's old entry numbers are not read.** They were thaum's own, and a
lint naming them would belong to thaum's extension rather than to the core. Keeping that lint permanently is in `path@core@docs/rejected-alternatives.md`.

**A reference is live wherever it is prose, fenced blocks included, for every kind.** A sketch
names what it names on purpose, and an illustration writes a placeholder in angle brackets,
which the tokenizer does not record. Reading a fenced slug reference as an illustration is in
`path@core@docs/rejected-alternatives.md`. A string literal bound to a name in Rust yields no
reference, per `design@core@grammars-not-prefixes`.

### A harness kind is cited with no anchor `##harness-kinds-cited-without-anchor`

A harness kind takes no anchor: today a skill, an agent, the primer and the root CLAUDE.md. A skill or an agent is cited by its name, two segments, and one of
its sections by its name and the section's slug, three: `skill@<name>`, `skill@<name>@<slug>`,
`agent@<name>`, `agent@<name>@<slug>`. The primer and the root CLAUDE.md are one document each, so
a section of either is cited by its slug alone: `primer@<slug>`, `instructions@<slug>`. Any other
arity, and an empty segment, is malformed, with the kind's forms as its repair. The third segment
narrows within the kind, as a path's last segment does, so the kind keeps the information that a
section belongs to a skill or to an agent.

- **No anchor, because the harness fixes one namespace per kind**: a skill or an agent is a
  directory or a file of the harness's layout, unique by name, and a project has one primer and
  one root CLAUDE.md. An anchor would carry no information, the argument
  `design@core@plans-is-a-reserved-anchor` makes for `plans`. A reference to a harness kind still serves
  `goal@core@relocation-is-one-manifest-edit`, which binds a reference to what the manifest
  places: nothing a harness kind names moves with a manifest edit.
- **The rejected alternative "A register reference with no anchor"**, in
  `path@core@docs/rejected-alternatives.md`, lost on two reasons. Its first, that the tool cannot
  know which register is single-instance in a project, does not hold here: the tool fixes the four
  kinds and their layout, and no project declares one. Its second, one grammar with no special case
  in the resolver, is the cost of this form.
- **The kind is read first**, so a word that is both a kind and an anchor would read as the kind;
  no anchor may wear a kind's name, per `design@core@kind-names-refused-as-anchors`.
- **`instructions` names the document's role**, not one harness's file name, so a move of the root
  instructions to another file name changes no reference, per
  `issue@core@configuration-for-several-agent-providers`.

The nearest rival, an anchor the tool constructs for the agent configuration with three segments
everywhere, lost: every name for it stutters or names a provider, and it carries nothing. It is in
`path@core@docs/rejected-alternatives.md`. Serves `goal@knowledge-architect@any-project-can-adopt-it`:
one grammar, the same in every project, with a global namespace naming no anchor.

### Every level-two heading of a section home of the harness is a section, and owes a slug `##section-homes-carry-slugs`

A section home is a skill's `SKILL.md`, an agent's file, the primer and the root CLAUDE.md, under
the `claude` harness. The rule of `design@core@an-entry-is-a-heading-at-the-register-level` holds
there at level two: every level-two heading outside a fence owes a slug, which defines a section
of that home's skill, agent or document, and a slug anywhere else in it defines nothing, per
`design@core@a-slug-elsewhere-defines-nothing`. An id is
unique within its document. A heading-shaped line inside the frontmatter block, as the harness
reads that block, is no heading.

- **A section slug names a section apart from its position**, where a paragraph number goes stale
  at every insertion. The rule, for a skill's sections, is the owner's proposal: "Better than
  paragraph numbers, which might get stale at any time. Use this in the published skills to refer
  between each other." A slug of the installed text is an interface: a rename dangles a consuming project's
  citations of it, which `design@knowledge-architect@changelog-entries` owes a Migration entry for.
  The owner, on that consequence: "The consequence you stated is accepted and positive in my view.
  This is the fragility I mentionned at the beginning with using paragrap numbers."
- **An installed copy defines its sections and raises no finding.** It is never the project's to
  repair, and under `commits` a commit holding an earlier version's set is judged by a later
  checker. The shipped set is held to the rule where it is written, by a test of the core over the
  shipped text, which also requires every reference of a harness kind in it to resolve within the
  set.
- **A project skill's or agent's slug at a level-two heading defines a section** of that skill or
  agent, whatever anchor holds its file.

The finding on an unslugged heading names the repair, and the setup skill states the requirement,
so an adopting project meets it in the documentation. Serves `goal@core@records-reach-their-reader`:
`show` on a section lists every text that cites it.

### Every path reference names its kind and its anchor, `` `path@<anchor>@<path>` ``, and the deepest anchor wins `##every-path-names-its-anchor`

A checked path reference is one backticked span in the one grammar of
`design@core@an-entity-belongs-to-its-anchor`, with `path` as its kind and the path under the named
anchor's directory as its id. The root is a component like any other, named by `project.name`.
The kind prefix costs five characters at every path reference and buys one grammar: the scanner
has one tokenizer, the old two-segment form `<anchor>@<path>` is reported as an anchor in kind
position by `design@core@candidate-rule-and-retired-forms`, and the unanchored lint is left with
one job. There is no unanchored form for a path of this tree: a backticked span of path characters
with two or more segments and no `@` is a finding naming the grammar when its first segment names a
file or a directory this tree's listing holds. One segment is a name rather
than a path, though the exact name of a skill or an agent is reported, per
`design@core@bare-skill-name-reported`, and a span holding a space, an angle bracket, or a colon anywhere but in a line
suffix is not path-shaped, which is what lets documentation of the syntax show a placeholder with no
carve-out.

**A span whose first segment names nothing here is another tool's notation, and is silent**: a
media type `application/json`, a unit `km/h`, a git ref `origin/main`, the fixture path of a test,
a path on another machine. The first segment is read from the root, from every anchor that is a
directory, and from the document's own directory, a `..` climbing from each; a leading slash is
read from the root alone. A spec is an anchor and a file, so it is no place a path is read from.
Only the first segment is asked, so a pointer whose first segment is here and whose rest is not is
still a finding, and the anchored form it is rewritten in reports it dangling. A commit message is
judged against two trees, per `design@core@a-commit-message-is-a-document`.

**The listing alone answers; the ignore rules are not asked.** A pointer into an ignored directory,
such as a build output, is silent. Asking git costs one spelling per span and per place it is read
from, makes every root name a pointer under a whitelist `.gitignore`, and fails the run outright
for a spelling through a symlink, which git refuses.

**What it costs**: a pointer written after its first segment left the tree is silent, and so is
one whose first segment carries a typo or names an ignored directory; one written while its first
segment stood was reported then. **What it buys** is the silence of another tool's notation, which
is nearly every path-shaped span of a project new to the tool. Re-take it with `cargo klarch check`
over a foreign project carrying the required structure, counting the findings under each rule; a
census in which the silence hides a real pointer of this tree reopens it. The census that decided
it is in the commit that rewrote this head, found with
`git log --grep='only where it names something of this project'`. Reporting every path-shaped span
is in `path@core@docs/rejected-alternatives.md`. This serves `goal@knowledge-architect@any-project-can-adopt-it`
and `goal@knowledge-architect@adoption-is-easy`.

**A line suffix or a fragment keeps a span path-shaped, after a file with an extension.** A
path followed by `:12`, the location an editor prints, and a path followed by a `#` and a
heading fragment opening with a letter each point into a file, so each is a finding. Without an
extension, or with a number after the `#`, the suffix is an image tag or an issue number, and
the span is not path-shaped. Its repair names the file and drops the suffix: a reference is to a file,
and a line number goes stale at the next edit.

**Four shapes are outside the lint on purpose**, each silent:

- a path in plain prose, with no backticks. The backticks are what mark a pointer, and plain
  text stays free prose.
- a span holding a space, such as a path with a space in a name. It is prose as much as a
  path.
- a span opening with `~/` or `$VAR/`. It names a path outside every tree this tool checks.
- a span written with a backslash separator. No project this tool checks writes one, and the
  census finds none.

Each widening would be measured the way the two-segment clause was: a census of what it
reports in a tree that is conformant. The census that kept these four outside is in the commit
that recorded them, found with `git log -G'outside the lint on purpose'`.

**The path is plain**: `..`, a `.` segment and a leading `/` are refused. An upward path is
anchored at the wrong place by definition, and it is the shape that breaks when the referencing
file moves. The id is everything after the second `@`, so a path may itself hold an `@`.

**The deepest anchor wins, and inside means a proper descendant.** A reference whose target
sits inside another anchor is refused with the right anchor named. What this buys is the
same property `design@core@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
anchor edits its one line of the project's `knowledge-architect.toml` and no document, and one fixed-string
grep per anchor is an anchor's complete inbound-reference list. An anchor's own directory
is the one target with no spelling under its own name, so it is named from an ancestor — a
reference that names a location, which a move is expected to break.

**A plan document is cited by its kind, never by its path**, per
`design@core@plan-document-kinds`. A
`path` citation of a spec, of a milestone directory or of a file inside one is refused from every
anchor, and the finding names the form that resolves: `spec@plans@<id>`, `milestone@plans@<id>`
or `spec@<milestone>@<slice>`. It is judged by where the target sits, before the deepest-anchor
rule and whether or not it exists, so a citation of a deleted plan gets the same repair. This
overrides the ancestor's spelling of an anchor's own directory for a milestone. A plan anchor, a
milestone or a spec, carries no `path` kind. One name per document is what keeps `show` complete: a citation written as a path
is one `show spec@…` would not list. The README of the plans directory, and the README and index
of each of its two homes, are not plan documents, and are cited `path@plans@<file>`.

**A fenced path reference is live**, as every reference is, per
`design@core@candidate-rule-and-retired-forms`: a sketch names its paths on purpose, and an
illustration that needs a fake path writes the escape anchor or an angle-bracket placeholder.

### A backticked span that is exactly the name of a skill or an agent the entity table defines is reported, with its reference as the repair, and a project skill named by an ordinary word is renamed with the project's prefix `##bare-skill-name-reported`

The scanner records every backticked span that is one word in the id grammar, a span wrapped
across a line break at one of its hyphens joined. A lint of the last phase reports such a span when
the entity table defines a skill or an agent of that name, installed or the project's own, and its
repair is `skill@<name>` or `agent@<name>`. Under
`harness = []` the table defines neither kind, per `design@core@harness-kinds`, so nothing is
reported. A commit message is judged as every lint is, per
`design@core@a-commit-message-is-a-document`.

**A bare name is a pointer written with no kind.** Silent, it dangles unseen when the skill or the
agent is renamed or deleted, the class `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`
records for a register's slugs. The reference costs the kind and one `@`, and `show` then lists the
text among what cites the skill, per `goal@core@records-reach-their-reader`.

**The match is exact against the table.** A crate's name, a name that no longer exists, and a
section's slug are silent, so the lint reports no span that is not a skill's or an agent's name.
Whether such a name meets an ordinary word rests on the names: the installed ones carry the
installer's prefix, and a project's own carry its prefix, per
`design@agent-skills@skill-name-prefix`, which nothing checks yet, per
`issue@core@tooling-for-project-skills`. A project skill named by an ordinary word makes every
backticked use of that word a finding, and the repair is to rename the skill with the project's
prefix. Re-take the false positives with `cargo klarch check` over
a tree that serves the harness and has not rewritten its bare names, such as another project's at
its upgrade, counting the findings of this lint: one on a span that is not meant as a pointer
reopens the exact match.

**In a released section of CHANGELOG.md the name is written as its reference too**, and as the
bare name again once the skill or the agent is deleted, per
`design@knowledge-architect@changelog-entries`. That bare name names nothing in the table, so the
lint leaves it silent.

The nearest rival is no lint, which keeps the silence this lint exists to end. A narrower lint,
matching only names that carry a known prefix, would leave an unprefixed project skill's bare
names silent, the same failure for those names, so the skill is renamed instead.

### Under the `path` kind, the escape for a path this tree does not hold and `*` for every component's own copy are reserved anchor words `##reserved-anchors`

**The escape anchor** — the word elsewhere in anchor position, `path@elsewhere@<path>` — marks a
path deliberately not resolvable here: a surveyed engine's layout, a deleted file a tracker
entry discusses, a hypothetical location. It is scanned and counted like any reference, so a
typo'd escape is still a registered pointer; it is exempt from the existence, kind and shape
assertions, because a foreign layout may spell anything. One assertion runs against it: a
target that DOES resolve here, beside any anchor, is a finding — without it, the escape is the
cheap way to silence the unanchored finding on a real path.

**A file of another project is written with that project's name as its first segment**, as
`path@elsewhere@<project>/<path>`, and the refusal's repair names that form. A foreign file often
shares its relative path with a file here, such as a dependency's src/lib.rs, and without the
prefix the assertion above refuses it. A repair that names only the local anchor leaves the writer
of a foreign pointer no checked form, and prose follows. The prefix is no cheap silence: it is false of
every file in this tree, so writing it before a local path is a deliberate misstatement, which costs
what removing the backticks costs, and that move is refused as a repair by
`design@agent-skills@plain-text-is-no-repair`.

**The generic anchor `*`**, `path@*@<path>`, marks each component's own copy of a path, as in
`path@*@docs/tripwires.md` written with the kind in front. It is accepted when the path is one of the
required document names in any of its shapes — the compiled documents, every heading register's
file, directory and README, and every file register's directory, README and index, so naming a
shape no component uses yet is legitimate — and otherwise when at least one component carries the
path with the claimed kind. The required set is derived from the component-scoped registers
rather than written out, so a project that declares one gets its homes in the set. A generic
reference nothing resolves rots exactly like a dangling one. The anchors the tool constructs hold
no copy the generic form names: what they hold is cited by its kind.

**Under any other kind the two words of `path` name nothing.** A `design@*@<id>` is reported as an
unknown anchor, and the finding says the words serve `path` alone. The `planned` kind refuses them
with a finding of its own, per `design@core@planned-path-form`. The manifest's resolution
refuses a declared anchor wearing either word, and it is no anchor: every pointer at it
would read as the reserved meaning.

### The word `plans` names the anchor the tool constructs at the plans directory, and no project may declare it `##plans-is-a-reserved-anchor`

The anchor is constructed per `design@core@plan-register`. The word is reserved like the two words
of `path`, so a citation of a plan reads `spec@plans@<id>` in every project, with no project name
in it: there is one plans directory, so the root anchor in every such citation would carry no
information. The manifest's resolution refuses a declared anchor named `plans`, and it is no
anchor: every pointer at it would read as the reserved meaning.

### A path a plan's work will create is cited `planned@<anchor>@<path>`, from the plans directory only, and reported once it exists `##planned-path-form`

A plan document names files and directories its work will create. The kind `planned` names one:
its anchor and its path follow every rule of a `path` reference, the named anchor, the deepest
anchor, the refused shapes and the trailing-slash claim, and its one assertion is the reverse of
`path`'s: the target does not exist. Once it exists, the finding names the `path` form and
nothing else, so the commit that creates the file converts the reference by its kind word, and
whether the plan still holds is that session's report to make. A target the ignore rules cover is
not asserted, per `design@core@ignored-targets-are-not-asserted`. `*` and the escape anchor are
refused: a planned path is in this tree, at one anchor.

**It is legal in the plans directory alone**, since a plan document is the one place that
describes unbuilt work, per `design@core@plan-register`. Elsewhere it is a finding that asks for
the plan that creates the file. An issue entry states what is missing rather than the shape of its
fix, and a hypothetical file there is `path@elsewhere@<path>`.

It is the checked form for a pointer that `design@agent-skills@plain-text-is-no-repair`
would otherwise send to plain text: the escape anchor is refused once the work creates the file.
The escape anchor as the planned form lost on the anchor: it names none, so the deepest-anchor rule goes unchecked until the file
exists and the conversion cannot be done by the kind word alone. `planned` is a reserved kind, and
the manifest refuses a register of that name, as it refuses `path`.

### A trailing slash claims a directory, and its absence claims a file `##trailing-slash-claims-directory`

The kind claim sits in the span itself — greppable, visible to the reader, checkable — rather
than inferred from what happens to exist. The claim is asserted as a fact in both directions, every
required document is asserted to be a file rather than a directory wearing its name, and a
heading register's two shapes are told apart by recorded kind.

### A target the ignore rules cover is not asserted `##ignored-targets-are-not-asserted`

Resolution asks `git check-ignore` whether the ignore rules cover the target before asking
whether it is present. Covered means accepted with no existence or kind assertion: a generated
path cannot be asserted to exist, per `design@core@git-supplies-the-walk`, and deciding by the
RULES rather than by presence makes the verdict identical on a fresh clone and a built tree. A
verdict that depends on the checking machine's build state is a check nobody can trust twice.
The exemption is exactly as wide as git's own answer, deliberately, nested ignore files included.

**The question is asked with the reference's own kind claim.** A directory claim is spelled with
its trailing slash and a file claim without, because a `dir/` pattern matches a path git can tell
is a directory, and a target that does not exist yet is a directory only if the spelling says so.

### A relative markdown link is a navigation row, and README.md and index.md files are the navigation homes `##links-are-navigation-rows`

A README is directions about what a directory holds and an index is a listing, so a relative
link — the row a reader follows where a renderer shows the page — is legal there and reported
everywhere else; in prose, a pointer is a backticked anchored path. A navigation link resolves
against the linking file's own directory, under the same kind claim and the same refusals as
an anchored path, in the references family; `check::registers` keeps the inverse assertion,
that a directory home's README links every subdocument, for every heading register.

**Every markdown link shape a row is written in is read, and resolved like the plain one.**
The inline form `[text](target)`, with an angle-bracketed target that may hold a space, and
with a title in double quotes, single quotes or parentheses after the target. And the definition `[label]: target`
at the head of a line, which is the target a reference-style link `[text][label]` resolves
through; a label opening with `^` is a footnote, whose definition holds text. An image,
`![alt](target)`, is a link to its target.

**An HTML link, `<a href="…">`, is not read.** Markdown is where a row is written, and the
census of walked markdown finds no HTML link outside this paragraph:
`git grep -n -i --untracked '<a href' -- '*.md'`. One written as a row reopens this clause.

**Markdown documents only.** In Rust prose a markdown link is rustdoc's mechanism, resolved by
rustdoc against the crate namespace, and reading those as index rows would report every
intra-doc link.

**The generated indexes emit links**, relative to their own directory, upward segments
included. Generated text cannot go stale, which is what the upward ban on hand-written links
exists against.

## 4. What is prose

### The grammar for a file's kind decides which of its bytes are prose `##grammars-not-prefixes`

A markdown document is prose entire. A Rust file is prose only where its grammar says so — its
comments, and the string literals that are not bound to a name. Nothing is decided by a line's
prefix. The one file set whose literals are never prose is the checker's own source, per
`design@core@checker-source-literals-are-data`.

**A doc comment's content is markdown**, so the markdown analysis runs over a document and over
every Rust comment alike, and a heading, a fence or a blockquote means the same in both.

**Prefixes were the single cause of four defects in thaum's tree**, each found by a checker or a
review rather than by reasoning: a formatted return type opening a line with an angle bracket read as a
blockquote; a float literal read as a rule number; a test fixture's string literal read as live
content; and a rule quoted inside a block comment lost because the `*` continuation was left in
front of it.

**Three readings of "data" were too broad, and the measurement over thaum's tree, in the message
of 94f6d67, caught each.** A fenced block is not data — a sketch in a design document comments
its rules on purpose, and reading fences as data lost 34 citations. A string literal is data only when BOUND to a name — an assertion's
message cites rules for a human to read on failure, and reading every literal as data lost 32
of them. A macro body is not an argument list, so a binding still applies inside one. **Reading
every literal as prose is a recorded losing alternative**, in
`path@core@docs/rejected-alternatives.md`, with the bound literals outside the checker that carry
a rule-shaped number by accident. The binding rule does not serve the checker's own
fixtures, which `design@core@checker-source-literals-are-data` answers.

### A Rust name is collected where it is declared, never where it is used `##names-collected-where-declared`

**A name is the one thing prose cannot reach**, and the identifier form of a rule marker exists
for names. Names are collected where they are DECLARED. Collecting every occurrence would make a call
site and a `use` import citations owing the rule's whole body in the caller's scope, where there
is nowhere to put it.

### The checker's own source reads its string literals as data `##checker-source-literals-are-data`

Under the tool's own directories a string literal is never prose. Its comments are prose like
every other file's, and everywhere else in a tree a string literal is prose unless it is bound
to a name, per `design@core@grammars-not-prefixes`.

**The tool's unit tests are the one place in a tree whose fixtures are rule-shaped on purpose.**
A number, a marker, a slug, a path or a version pin is written so that a test can watch a check
react to it. Read as prose each one is a live claim, and hiding each behind a named constant to
be interpolated makes the tests harder to read than the code they test. Comments stay prose
because the test modules point at decisions and paths for real, and those pointers stay checked.

**The directories are compiled in, never declared.** A binary's source spans several
Components: the core's, and the Component of each extension the binary registers. Each library exports
the directory of the Component it belongs to, evaluated at build time, and the binary hands the
list of its libraries' directories to the walk. The core is one crate whose directory is its
Component, so the core library exports its own `CARGO_MANIFEST_DIR`, and the core's binary and
tests sit inside it. An extension whose library crate sits below its Component's directory exports
that Component's directory instead, or the binary and tests beside the library would sit outside
it: a binary sees only the libraries it depends on. A binary run
through a cargo alias over `cargo run` is built from the checkout on every invocation, so each
compiled path is the tree being checked, unless another checkout shares the target directory and
the build is not tied to its checkout, which `design@core@a-foreign-build-is-refused` refuses. Once the core is consumed as a published crate, its
directory is outside the tree and exempts nothing, which is correct: its source is then not part
of the tree. The root and the compiled path are canonicalised before
the prefix test, so a symlinked checkout does not defeat it; a symlink inside the tree is not
followed. A manifest row is not an option, because a row can be pointed at any directory, which
is the shape `design@core@the-regime-has-no-opt-out` exists to refuse: only the checker's own source
can ever be exempt, and it is exempt by construction. The row's entry is in
`path@core@docs/rejected-alternatives.md`.

**The failure that remains is loud.** The summary block names each of the checker's directories, relative to the root when it sits under it and
absolute otherwise, even when the compiled directory no longer exists, and prints the count of
Rust files it covered, so the state is visible in every run. A directory exempts files only when
it sits inside the tree being checked: a tree that sits inside it instead, such as a mock project under
`path@core@tests/projects/`, is a foreign project and every literal in it is prose.

**Whole source rather than test modules only.** The non-test source holds no literal that cites
a rule, a finding message interpolates the number it names, and a narrower rule would cost an
attribute lookup to protect a class with zero members.

### A binary built from another checkout of the tool is refused before any command `##a-foreign-build-is-refused`

Two checkouts sharing one
target directory leave the last build's binary for both, and cargo does not rebuild it for the
other checkout, whose own package is still fresh; observed on thaum's repository, the other
checkout then runs that binary, its code and not only its compiled paths; the reproduction is in
the message of 33d3bd2. Before any command,
the binary compares where its own crate and each library it links were compiled with the tree it
is run over: a tree that holds the same package, by the name its `Cargo.toml` declares, at a
trailing run of the compiled directory but not at the compiled directory itself is a second
checkout, and the run exits 2 naming both directories and the `cargo clean` of every workspace
package the binary links, which rebuilds them from this one. The binary's own crate is listed
because it is rebuilt whenever anything it links is, so its compiled directory is always the last
build's; a worktree nested in the tree is a second checkout like any other. A tree holding no
copy is left alone, which covers a mock project under the tool's directory and a project
consuming a library as a published crate.

**The refusal detects, and a build tied to its checkout prevents.** A project whose every target
is tied to a variable valued at its checkout's root never runs another checkout's build, as
`design@knowledge-architect@a-build-is-tied-to-its-checkout` argues for this repository. The
refusal stays, for a binary built before the tie or outside the configuration that sets it, and
for a project that does not tie its builds. Its message names the tie beside the `cargo clean` that
recovers.

**Three limits.** Two come from reading only names and places. A second checkout that moved the
crate to another relative path is not seen, and runs, unless the build is tied to its checkout; the loud failure of
`design@core@checker-source-literals-are-data`, printed by `check`, still names it. A tree
holding an unrelated crate of the same name at the same relative path, one component long or
more, is taken for a copy, and refused; a binary run through an alias that builds it from the
checkout never meets one. The third comes from where the check lives: it is part of the binary it judges, so a binary
built from a checkout older than the refusal, or from one that changed it, runs unrefused.

**The refusal, not a correction of the paths.** Finding the directories at run time would exempt
the right files and leave the other checkout's code judging this tree. Every command is refused,
not only those that read literals, because every command runs that code. Each library a binary
links exports its crate directory and its package name, and the binary hands the list over, so an
extension's library is covered by listing it; this constrains every extension binary. The values
it reads are each crate's own directory and package name, beside the Component directories that
`design@core@checker-source-literals-are-data` compiles in.

**Every binary tests both halves over its own Components.** Over a checkout holding the tool,
the summary's checker-source line names each Component whose library the binary links and
counts the walked Rust files under them. And a tree holding one of the binary's crates at
another relative path is refused. The core's binary does this in
`path@core@tests/binary.rs`; an extension binary does it in its own suite, because only it
links its extension's library.

### The manifest pins the checker's version, every binary that dispatches a command of the checker refuses to run over a project it does not satisfy, and a binary that dispatches none makes no call `##installed-binary-version-check`

Every manifest declares `[project] checker-version`: the exact version of the checker the project
runs, which is the version of the core library whatever binary links it, or one of the two values
of `design@core@checked-sentinel-values`. `Manifest::parse` reads it; an absent key, or a string
of none of the three forms, is a complaint opening with the key's name, and a value that is not a
string fails the parse. Each binary that dispatches a command of the checker calls
`cli::refuse_another_version` in its `main`, right after the build-origin refusal of
`design@core@a-foreign-build-is-refused`, before it dispatches any command, an extension's own
included: a version is accepted when it equals the core library's, and any refusal, the key's own
complaint included, is exit 2, per `design@core@exit-code-ladder`, naming the two versions and
which side is older. `cli::run` does not refuse, so a command driven in-process needs a key but no
confirmed one. A binary that links the library and dispatches no command of the checker, such as a
maintenance crate that uses it only to find the project's root through `MANIFEST_NAME`, does not
call it: nothing it runs reads the documents by the checker's rules, and where it runs the checker,
as a gate does, it starts a binary that makes the call, since the gates library depends on nothing
of the checker, per `design@gates@a-project-holds-its-gate-list`.

**Why the manifest, and why required.** Without a pin the binary reads, a binary older than the
skills a project installed reports each installed file as differing, with the repair "run
install-agent-skills", which writes the older text over the newer; under `harness = []` nothing is
compared at all; and two versions whose skill text is identical are not told apart. A pin in the
manifest covers all three; one more place to
edit when the pin moves cannot drift silently, since a binary differing from it refuses at its
first command. Optional, the key would leave unprotected the project that never wrote it, which is
the forgotten pin it exists for. This serves `goal@knowledge-architect@any-project-can-adopt-it`:
a project pins the version it uses, and moves when it chooses.

**The working tree only.** `commits` reads a historical tree's key, so a tree without it stops in
phase 1, and compares no historical value, so moving the pin fails no earlier commit.

**A binary released before the key** refuses a manifest carrying it at parse, as an unknown field,
since `[project]` denies unknown keys: it runs no command, so it cannot suggest a downgrade, but
its message does not name the pin.

### A checker version that is no version is accepted only where the running binary's build confirms it `##checked-sentinel-values`

Two values of `[project] checker-version` are not versions, for a project's own tests and for a
tree the checker is built from:

- `"fixture"` is accepted when the project lies strictly inside the core's compiled directory or
  one of the directories the binary passes as its own source: a mock project of a library the
  binary links. Strictly, because an extension whose crate is the project's root passes the root
  among its directories, and a library is no mock of itself.
- `"self"` is accepted when the core's compiled directory lies inside, or is, the project, and the
  project's git tracks that directory's `Cargo.toml`: a tree the checker is built from. The
  tracking tells it from a project whose `CARGO_HOME` is one of its own directories, where a
  registry build's directory also lies inside the root.

Both read the paths canonicalised, as `design@core@a-foreign-build-is-refused` does, and decide
from where the running binary was built, which names no project, per
`design@core@nothing-of-a-project-is-compiled-in`. Anywhere else each is refused, so no value of
the key switches a real project's pin off, the stance `design@core@the-regime-has-no-opt-out`
takes for rules over documents. A copy of a mock project outside its library carries the version.
The same values accepted wherever they are written lost to this, in
`path@core@docs/rejected-alternatives.md`.

## 5. Commit messages

### A commit message is a document under the regime, judged against the tree its commit carries `##a-commit-message-is-a-document`

A commit message is parsed as one markdown document — its subject line, its blank line and its
body — and every rule of the regime runs over it: every reference is judged against the entity
table of `design@core@one-entity-table`, and each registered extension judges the message
against the tree it prepared for that commit.

**A message is history, and it cannot be edited.** `main`'s history is never rewritten, so a
message's claims are fixed the moment it lands: a pointer that stops resolving in a document is
repaired where it stands, and the same pointer in a message can only be read wrong. That is why
the regime matters most on messages.

**A message is judged byte for byte, as the commit holds it.** `commits` reads the message
with `%B` and cleans nothing: git applied its own cleanup before the commit existed, and a
second pass would take bytes of a commit out of the regime — under `-m` a `#` line is committed
verbatim.

**One command judges messages, after the commit exists.** `commits <range>` judges every commit of
the range against its own tree, with its first parent. It is run over `HEAD~1..HEAD` after each
commit, or over the branch where `design@core@branch-shas-are-refused` is turned on, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`, so a finding in the newest
commit's message is repaired by `git commit --amend`, and over the branch by the gates and by CI
before any merge, which is where the guarantee lies: `main` is never rewritten, so what must not
happen is a message with a finding reaching it, and the range check before the merge excludes that.
No hook judges a draft before the commit exists. A hook repeats the range check's message half, not
its tree half, at every commit; it is per-clone state a session has to install and check; and it
resolves a message against HEAD, which under an amend, a reword or a squash is the commit being
replaced, so it refuses history edits the range check accepts.

**Everything a commit is judged against is read from that commit's tree.** The manifest, the
`[walk]` rules, the documents, the generated indexes, the per-instance options and every file an
extension reads all come through git objects, and the model is assembled in memory. Reading them from the
working tree instead would judge a message written a hundred commits ago against decisions that
did not exist then, and the finding list would be a list of things nobody could have known.

**Every commit of the range is judged by the tip checker, and a tree that fails is a finding.**
The tip checker is the binary built from the working tree, and it judges every commit's tree and
every commit's message. **Every commit of the range is judged alike, the last one included, and
wherever HEAD is.** A commit whose tree does not load, or carries findings, fails the run with
exit 1: its tree's findings are printed, each named by the commit and by the file inside it, and
its message is still judged wherever the tree reached the last phase, since its entity table is
then complete. Where the tree stopped earlier, the message is judged against nothing and the run
says so. No commit is set apart as the range's tip: a range that does not end at the checkout,
such as a pre-push hook's or an audit of old history, holds no commit that `check` has judged,
and treating its last commit specially would leave that commit's failure unreported as a finding.
The summary counts the range's commits, how many passed and how many failed. A commit fails when
any finding of the run belongs to it: its tree's, its message's, or a citation of the range by
SHA. Its line names each of those sources, with its count of findings, except a tree that does not
load, which is one finding. So the header agrees with the verdict line, and passed plus failed is
the number of commits. Separate counts of failed trees and
failed messages in the header lose to this: one commit can fail on both, so either such a count
overlaps the other, or one source is left out.

**A branch that makes the checker stricter orders its commits for it.** A new check, or a change
to the manifest format, makes every earlier commit of the same branch fail under the tip checker.
Such a branch puts that change in its first commit, with every fix the tree needs to pass it, or
is squashed to one commit before its review. Checker changes are rare and land on branches of
their own, so this costs a history edit on those branches alone; the alternative that avoids it,
building each commit's own checker, costs one release build per commit on every branch.

**A message's references resolve against its commit's tree or its first parent's, and its lints
stand where either tree holds what they name.** A commit that closes an issue deletes the entry
and names it in the message, and against its own tree alone every such message would dangle. A
reference finding therefore survives where both trees refuse the reference. The unanchored path
and the retired slug reference, per `design@core@every-path-names-its-anchor` and
`design@core@candidate-rule-and-retired-forms`, fire where their span names something of the
project, so a finding of either stands where either tree holds what it names: a message naming,
by a bare path, the file its commit deletes still points at it. The parent's model is the previous commit's wherever
the walk followed the parent chain — a failed commit's tree still serves as the next commit's
parent, since its entities are read and not its verdict — so an extra model is built only at
the range's start and where the chain was not followed. **A tree whose manifest fails phase 1 is
the exception**: nothing of it is read past its manifest, it is judged no further, and it serves
as no parent, so the next commit's message resolves against its own tree alone. Such a commit
has already failed the range, and reading its whole tree for its successor's sake dominates the
time `commits` takes over a range of them. A parent tree is assembled for its
entity table and the facts a path reference and the two lints ask about, and nothing else: no extension is
prepared for it and no check runs over it, because nobody reads its verdict. The two arms are compared by the site each names — the line and the span — rather than by the words each writes,
because two trees can refuse one reference for different reasons and comparing the words whole
would let a reference that resolves in neither pass.

**A historical tree's `[project] checker-version` is read, not compared.** Its absence stops the
tree in phase 1; its value is never compared with the tip checker's version, so moving the pin
fails no earlier commit, per `design@core@installed-binary-version-check`.

**`check` reads no range of history.** A message is not a file of the tree, and a check whose
verdict moved with the branch's history would be a check nobody could reproduce from a checkout:
`git stash` alone would move it. `check --staged` reads HEAD's tree and the index, one tree and no
range, per `design@core@staged-tree-source`. The range is always explicit, and the gates pass the range from the
remote `main` to `HEAD`, so what is judged is the branch's own commits and never `main`'s.

**An extension leaves out, over a commit's tree, a check whose subject is not the tree.** A
check of filesystem state has no subject there, and one that would read every archived input at
every step of the range decides nothing about whether a message's references resolve, which is
the question the per-commit model exists to answer; `check` is what judges a checkout against
it.

**The ignore rules a per-commit run asks are the working tree's.** `git check-ignore` reads the
`.gitignore` files on disk and has no form that asks a historical tree. A commit whose ignore
rules differed from today's is therefore judged against today's, which can cost a path reference
asserted where that commit's own rules exempted it — a finding rather than a silence. Since a
failing tree fails the range, a branch that changes its ignore rules orders its commits for it
as a branch that makes the checker stricter does: the change first, or a squash.

### `commits` refuses a citation of a commit of its range by SHA, where the manifest turns it on `##branch-shas-are-refused`

Under `[commits] refuse-branch-shas = true`, `commits` reports every run of 7 to 64 lowercase hex
digits, bounded by bytes that are not ASCII letters, digits or `_`, that prefixes the SHA of a
commit of the range it judges: in each commit's message, and in the whole text of each document of
that commit's tree, a Rust file's code and literals included. A commit whose tree stops at phase 2
or 3 is scanned too, since the scan needs no entity table and no verdict. One whose tree stops at
phase 1 is not: its manifest did not resolve, so the option is not read from it, and no document of
it was read. The range a branch is judged over is
its own commits, so these are exactly the commits a merge that rebases gives new SHAs; a citation of
one would name nothing once the branch merges. A SHA on the main branch, or of another project, is
not in the range and passes, unless its first 7 digits collide with a SHA of the range. **A citation
is seen only when the cited commit is in the range judged**, so where the option is on, the check
after a commit is run over the branch, `<main>..HEAD`: over `HEAD~1..HEAD`, a citation of an earlier
commit of the branch passes, and fails first at the merge gate, after the push, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`. Only lowercase counts, since git
prints a SHA in lowercase, and `_` bounds a word as a letter or a digit does, since it joins the
parts of an identifier. A project whose branch commits keep their SHAs until they reach the main
branch has nothing to refuse; a rebase onto main before a fast-forward rewrites them as surely as a
rebase merge does, so the check is off unless the manifest turns it on. It serves
`goal@core@declared-instructions-are-checked`: a rule against such citations is kept by running
`commits`, not by a reviewer remembering it.

The check lives in `commits` because it already holds every input: the range's commits, each
message byte for byte, and each commit's tree assembled into a model. A gate of its own would
list the range and read the messages again. A SHA in a file the walk does not read is not seen,
which is the checker's scope everywhere else. Sixty-four digits is the length of a SHA in a
repository under SHA-256. A citation of a commit the branch rewrote before `commits` ran is not in
the range, and is not seen: no clone can tell reliably which commits a branch once held, since a
fresh clone, such as CI's, never fetched them.

## 6. The agent configuration

### The `[agents]` table declares which agent harnesses a project serves `##agents-table`

`[agents] harness` lists the agent harnesses the project serves. The tool knows one, `claude`, and
it is the default when the table is absent, because the tool is built for projects developed
mostly by agents. Under `claude`, every component carries a CLAUDE.md, and the agent files this
version ships must be installed, per `design@core@owned-namespace-check`. An empty list declares
no agent configuration: no CLAUDE.md is required, and nothing is installed or checked. A value the
tool does not know is refused in phase 1, and leaves the list.

Under `claude`, the four harness kinds exist, per `design@core@harness-kinds`; under an empty
list, they do not.

Under an empty list, the files of the installer's namespace are walked as the project's own
documents, like any others. A harness for another provider is a value added to the list, requiring
its own file and its own layout, which `issue@core@configuration-for-several-agent-providers`
records. This is what serves `goal@knowledge-architect@any-project-can-adopt-it` for a project
without agents.

### A project declares the command it runs the checker by `##declared-command`

`[project] command` names the command a project runs the checker by; when it is absent the command
is `klarch`, the binary's name. The checker prints it in its messages, in the header of every
generated index, and in the agent files it installs, which are templates holding the placeholder
`{{command}}`. No fixed name serves every project: a project with an extension runs its own
binary, as thaum runs its rules extension through a cargo alias, and the core's binary refuses a
manifest holding a table only an extension claims. An extension project therefore declares a
command that runs its own binary, preferably a cargo alias such as `cargo klarch`, and does not
install its binary under the plain name. A command that cannot be printed on one line in a code
span, empty or holding a line break or a backtick, is refused in phase 1, and the default stands.
This serves `goal@knowledge-architect@any-project-can-adopt-it`: every project, extended or not,
reads instructions that name the command it actually runs.

### The installed agent files are committed, and checked byte for byte in a namespace the installer owns `##owned-namespace-check`

`install-agent-skills` writes each file this version ships at its install path, rendered with the
project's command, and deletes every file of the installer's namespace that the version does not
ship. The namespace is the prefix `knowledge-architect-` among the skill directories and the agent
files of `.claude/`, and the directory .claude/knowledge-architect/. A project's own skill takes
a name of its own, so ownership is decided by a name, with no record and no history: a renamed
skill shows as the old file to remove and the new one missing; one install repairs both, and the
removal awaits staging like any deletion, which the check names until it is staged.

The installed files are committed, so a session started from a fresh clone, a web session or a
review in a new worktree has them without building anything, which serves
`goal@knowledge-architect@agents-get-a-complete-workflow`. A symbolic link on an owned path is
refused by the install before it touches anything, because following one would write or delete in
another directory. Under `claude`, phase 2 reports each
shipped file missing or whose bytes differ from its rendered template, each file of the namespace
the version does not ship, and a root CLAUDE.md that does not import the shipped primer with a line
holding exactly an at sign followed by .claude/knowledge-architect/PRIMER.md. These are facts about what the tree
holds, like a missing required document, so a run stops before references while one stands, per
`design@core@phases-gate-the-report`. A command that writes a generated file refuses while one
stands, as over any phase-2 finding;
`issue@core@installed-file-findings-belong-in-phase-four` records why the last phase would fit
them better. Only `check` compares: `commits` does not, because the running binary ships its own
version's text, and an older commit's installed set would fail against it with no repair a commit
already in history can take. The comparison is of bytes: the binary holds the text, so a
digest would add a dependency and nothing else. The installed files are outside the walk by
construction, as the generated indexes are: their prose is the shipped text's, judged where that
text is written, and its illustration paths would otherwise be reported in every installing
project.

**The installed copies are also read, for definitions only**, per
`design@core@installed-entities-from-the-tree`: the model holds them parsed, no check walks them
for references, and they raise no finding of the section rule.

The install edits nothing outside the namespace. In particular it does not edit the root
CLAUDE.md, which belongs to the project: it says the import line is missing, and the check reports
it with the line as its repair. A change a project needs to an installed skill belongs in a skill
of its own, routed from the project's CLAUDE.md.

### Under the `claude` harness, skills, agents, the primer and the root CLAUDE.md are entities of kinds that are no register `##harness-kinds`

The four harness kinds, `skill`, `agent`, `primer` and `instructions`, exist under the `claude`
harness alone, per `design@core@agents-table`. They are kinds, not registers: a register is what
an anchor carries, and no anchor carries them and no project declares them, since the harness fixes
where each lives. So they sit beside `path` and `planned`, the two kinds already no register. The
rival, built-in registers of a new shape carried by no anchor, would make every reader of a
register handle a register with no carrier: the manifest's refusals, the index, the extension view.

| kind | an entity | named by |
| --- | --- | --- |
| `skill` | a directory directly under `.claude/skills/<name>/` holding `SKILL.md` | the directory, per `design@core@harness-entity-names` |
| `agent` | a Markdown file at any depth under the agents directory whose frontmatter sets `name` | the file, its `.md` removed once, per `design@core@harness-entity-names` |
| `primer` | the installed primer; its sections alone are entities | none |
| `instructions` | the root CLAUDE.md; its sections alone are entities | none |

The layout is the harness's, read in its documentation of skills,
https://code.claude.com/docs/en/skills.md, and of subagents,
https://code.claude.com/docs/en/sub-agents.md: a skill in a `<subdirectory>/.claude/skills/`
directory is loaded only when a session reads a file there, and is no entity here; the agents
directory is scanned "recursively"; a file there with no `name` is treated "as documentation kept
beside your agents", so it is no agent, and no README or index is owed there.

Under `harness = []` none of the four kinds exists, a span headed by one is silent, and installed
copies a project kept are its own documents, whose slugs are misplaced definitions. Serves
`goal@core@records-reach-their-reader`: a citation of a skill, an agent or a section is checked,
and `show` lists every one.

### Under the `claude` harness, the root CLAUDE.md is modelled and no scoped CLAUDE.md is `##root-instructions-alone-modelled`

The root CLAUDE.md alone is modelled, as the owner chose: "I'd model only root CLAUDE.md
sections." A scoped CLAUDE.md is not modelled: what such a file holds is the subject of
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`.

### Under the `claude` harness, a skill is named by its directory and an agent by its file, and a frontmatter `name`, where one is set, equals that name `##harness-entity-names`

"The directory name also invokes the skill", in the harness's documentation of skills, even where a
frontmatter `name` sets a second command. An agent's "identity comes only from the `name`
frontmatter field", in its documentation of subagents. A frontmatter `name` that differs from the
directory's or the file's name is a finding, so the name a reference cites and the name the harness
uses are one. The match is the owner's ruling, against the frontmatter `name` alone as an agent's
id: "I'm not sure our installed workflow would be able to work properly if not, and I'd rather make
the checks more strict to keep the workflow simpler than the other way around." Its cost: the
harness documents a skill whose `name` differs from its directory as a second command for it,
which is a finding here, so a project that wants a short alias renames the directory. The `name:`
line is read alone, as YAML reads it, so frontmatter the harness reads and the checker's own
frontmatter subset refuses, a list or a multi-line description, causes no finding.

### Under the `claude` harness, a skill or an agent whose name a reference cannot spell, or that the harness does not load, is a finding `##unusable-harness-name-is-a-finding`

A skill or an agent named outside `[a-z0-9]+(-[a-z0-9]+)*` could not be cited. The harness's
documentation of skills says a skill folder is not named "`synced`, in any capitalization", and that
one named `anthropic-skills` or beginning `anthropic-skills:` does not load. So a skill whose
directory is `synced`, or begins `anthropic-skills`, is a finding. The rival, recording the
documentation with no check, would pass a skill no session can use. An agent's `name` is, in the
documentation of subagents, "at most 256 characters", holds no `:` and does not start with `-`; the
id grammar is stricter, and stands as its own requirement.

### The installed skills, agents and primer are defined from the installed copies of the tree being judged `##installed-entities-from-the-tree`

The model holds the installed copies, parsed like a walked document and walked by nothing:
`Model::build` reads them off the disk, and `commits` hands in each commit's own blobs. Both read
them whatever the walk rows say, so a skip over the installer's namespace leaves the two verdicts
alike. A commit is therefore judged against the installed set it holds, so a branch that renames a
section in an upgrade and repairs its citations passes `commits` on every commit.

- **Not the binary's shipped set.** `commits` judges every commit of a range with the tip's binary,
  per `design@core@installed-binary-version-check`, so definitions from the shipped set would fail
  every earlier commit citing a section the tip renamed. That is why `design@core@owned-namespace-check`
  already compares no installed copy under `commits`.
- **An installed copy raises no finding**, per `design@core@section-homes-carry-slugs`.
- **This repository's shipped text defines nothing in its walk**: content/ writes each section slug
  as a build placeholder, per `design@agent-skills@content-mirrors-the-install-layout`, so the
  definitions are the installed copies', which this repository installs with each change to
  content/.

