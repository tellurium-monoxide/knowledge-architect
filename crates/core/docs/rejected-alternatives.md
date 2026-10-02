# Rejected Alternatives

**A hand-rolled gitignore matcher reading the root file alone**, with the walk pruning what it
covered — lost to `design@core@git-supplies-the-walk`. `live`. It needed no `git` on the path and no
process per run, and its supported subset was chosen against thaum's own file. It loses
on three counts, each measured against the tree rather than argued: a tracked live document left
the walk AND the inverse assertion when one root ignore line matched its bare name at any depth,
reproduced with a one-line `.gitignore` over a copy of `path@core@tests/projects/minimal/` and
recorded as a defect before the reversal; a nested `.gitignore` was not honoured at all, so a
project needing one had to declare the path in the manifest; and every pattern the matcher could
not honour — a negation, `**`, `?`, a character class — had to be refused by name, because
dropping an ignore rule makes the walk read more than it should and dropping a negation makes it
read less, and both are silent. Git answers all three by construction, and the walked-file count
over thaum's tree was identical under the two.

**Skipping a commit whose tree fails, in `commits`** — lost to
`design@core@a-commit-message-is-a-document`. `live`. A commit whose tree did
not load or carried findings was named in the summary and judged no further, so the range could
walk over commits older than a manifest migration, and over the commits of a branch before its
tree went green. It loses because each skipped commit took its message out of the regime while the
run exited 0, and the summary line was the only trace. The migration it served is done, and a
branch that tightens the checker orders or squashes its commits instead. Judging each commit with
the checker built from its own tree would also serve a migration branch; it costs one release
build per commit of every range.

**`check` and `commits` refusing a pipe on stdout** — lost to
`design@core@a-commit-message-is-a-document`. `live`. It was meant to stop
`cargo klarch check | tail -n 10 && git commit`, which commits over a failing tree because
the shell takes the exit status of `tail`. It was built and reviewed, and it loses because it
cannot stop that chain: a process inside a pipeline does not set the pipeline's status.
Reproduced with the refusal in place, `bash -c 'cargo klarch check | tail -n 3 && echo
commit'` printed the refusal, then `commit`, and exited 0. The chained command still ran, now
over a tree the checker had not judged at all. Two further costs were measured. The test was
`is_fifo` on stdout, and ksh93 builds its pipelines from sockets, so `klarch check | tail`
under ksh93 was not refused. And every caller that captures the output had to give the child a
regular file: `cargo x gates` and the test helpers of thaum's two checker binaries; the xtask
side is its own entry in `path@xtask@docs/rejected-alternatives.md`. The merge is guarded either way by the
range check this entry lost to.

**A `commit` subcommand that runs the checks and then `git commit`** — lost to
`design@core@a-commit-message-is-a-document`. `live`. It would judge the tree and the
message at the moment of committing. It loses on three counts. It has to offer `git commit`'s
interface: `-a`, `-m` and `-F`, the editor, pathspecs, `--amend`, `-p`. A session can still
call `git commit` directly, so it guards nothing a session does not opt into. And judging a
message before the commit exists repeats the reason the `commit-msg` hook below lost: under an
amend or a reword, HEAD is the commit being replaced.

**A `commit-msg` hook judging each message before the commit exists** — lost to
`design@core@a-commit-message-is-a-document`. `live`. It repaired a message by an edit of
the draft rather than by an amend, and cost 0.62 s per commit against 1.6 s for
`commits HEAD~1..HEAD` after it; the figures and the two refusals are in the message of the change
that removed the hook. It loses on three counts. The range check runs in the gates and
in CI before every merge, so the hook guarded nothing the merge does not. It judged a message
against HEAD as the parent, which under an amend or a reword is the commit being replaced: a
rebase rewording two messages that `commits` accepted was refused by it twice and needed
`--no-verify`. And it was per-clone configuration every clone had to install and every session
had to check. The variant that also refused a staged tree carrying findings is the entry below.

**The `commit-msg` hook refusing a commit whose staged tree carries content findings** — lost to
`design@core@a-commit-message-is-a-document`. `live`. It would catch a failing tree at the
moment it is written rather than at the range check. It loses because every work-in-progress
commit would then need a way around the hook, and `commits` already judges each commit's tree
before a merge.

**Selecting a subset of the checks with `--only`, one family per check** — lost to
`design@core@phases-gate-the-report`. `live`. It let a reviewer run the checks of its own axis
and let a run that read no rule text resolve no release. It loses to the phases: the checks cross
them, so a selection is honoured only with a second bookkeeping saying which part of a check a
stop withheld; every consumer was a reviewer on a tree the gate already passes, where the selected
and the full run print one verdict; and resolving releases at the last phase gives every early
stop the no-network property the selection gave one run. What it bought was measured: a full run
over thaum's tree against a references-only one, with the GNU time command, and the figures
are in the message of the change that removed the selection. A full run of `cargo klarch check`
over thaum's tree growing past a few seconds is what would reopen it.

**Families named for the subjects that read them** — lost to `design@core@phases-gate-the-report`.
`live`. No selection exists to name families for, and the reason it lost while one did holds
against any that returns: nothing about a project is compiled into the tool, per
`design@core@nothing-of-a-project-is-compiled-in`, and a subject's name inside the parser
is exactly that project's knowledge, compiled in.

**One family per invocation, instead of a set** — lost to `design@core@phases-gate-the-report`.
`live`. No selection exists, and the reason it lost while one did, from
`design@core@model-then-checks`, holds against any that returns: the walk happens once per
invocation, so a caller wanting five families would read every live document five times.

**A per-document phase gate, hiding only what an unread file could have defined** — lost to
`design@core@phases-gate-the-report`. `live`. It would let a citation finding in one file
print while another file is unreadable, at the cost of tracing which entities the unread file
could have defined and hiding only the findings that name them, roughly three times the machinery
of the whole-report stop. It lost on that cost, and because the tree-level facts of phase 2 — a
home that is not there, a refused declaration — have no per-file scope to trace. It is the form
the tripwire on the whole-report stop reopens with, which is why it is here.

**A slug unique across the whole project, with the component named for the reader only** — lost to
`design@core@a-slug-belongs-to-a-component`. `live`. It keeps one meaning per word everywhere and needs
no lookup to resolve a reference. It loses because it makes every component's vocabulary global: two
components cannot each decide something they call the same word, and the second one to want the word
has to take a worse one.

**A reference with no component read as one inside its own component** — lost to
`design@core@a-slug-belongs-to-a-component`. `live`. It would leave a pointer inside a component as
short as it was before components existed, and qualify only the crossings. It loses on what a
reference has to carry by itself: the same text would name different decisions depending on which
file it sits in, so moving a document between components would silently retarget every unqualified
reference in it.

**A bare root-relative path form, with a targeted check on the collision cases** — lost to
`design@core@every-path-names-its-anchor`. `live`. It keeps prose short and costs no migration.
It loses on what was counted: the required document set gives every component the same names,
so a bare `docs/`-headed reference from inside a component silently names the project's file —
twelve candidate sites in one component alone, nine of them wrong, counted by hand when the
defect was recorded. A targeted check patrols the collision; retiring the form makes it
unrepresentable, and an anchored reference is what a fixed-string grep can find.

**A fenced reference read as an illustration**, the stance slug references took before the
`@` grammar — lost to `design@core@candidate-rule-and-retired-forms`. `live`. It let a document
explaining the convention hold an example without inventing a slug. It loses on consistency:
path references were already live in a fence, so one kind had two stances, and a sketch in a
design document names its decisions as deliberately as it names its paths. The illustration is a
placeholder in angle brackets, which the tokenizer does not record.

**`#` as the reference separator**, the incumbent of `` `<component>#<slug>` `` — lost to
`design@core@a-slug-belongs-to-a-component`, which fixes `@`. `live`. It costs no migration of
the slug references. It loses on collision: `#` is a Rust attribute opener and a markdown
heading marker, both of which sit inside code spans in thaum's tree, and `:` — the other
candidate — is a Rust path separator; `@` meets almost nothing in Rust and only email
addresses in prose, which never sit in backticks here. It entered the tree with the first
migration to qualified references, carrying no argument of its own.

**A configurable reference separator** — lost to `design@core@a-slug-belongs-to-a-component`,
which fixes `@`. `live`. A project could pick the character its prose collides with least. It
loses because every instruction, every skill and the future link preprocessor would be
parameterised on it, and `@` already meets almost nothing in Rust and only email addresses in
prose, which never sit in backticks here.

**Paths without the kind prefix**, keeping `` `<anchor>@<path>` `` beside
`` `<kind>@<anchor>@<id>` `` — lost to `design@core@every-path-names-its-anchor`. `live`. It costs
no migration of the path references. It loses on what it leaves in the scanner: two grammars, told apart by segment count, and an unsupported-shape lint
that keeps its heuristic instead of becoming "unknown kind". Five characters at every path
reference bought one tokenizer and one candidate rule.

**A register reference with no anchor**, `<kind>@<id>` for a register a project has only one
instance of — lost to `design@core@a-slug-belongs-to-a-component`. `live`. It is shorter for the
common case. It loses because the extracted tool cannot know which register is single-instance
in a given project, and one three-part grammar serves every kind without a special case in the
resolver or in the instructions.

**Keeping the `@` prefix as the escape** — lost to `design@core@reserved-anchors`. `live`. It
costs no migration. It loses on the census taken at the design session: the tree held 75
`@`-prefixed spans carrying three meanings — about 65 meant every component's own copy, a
handful meant a path outside the tree, three were Java annotations that were never path
syntax — and the scanner registered none of them, so a typo'd escape was invisible by
construction. One mute marker for three meanings is the confusion the anchor words dissolve.

**The `@` prefix reused as the root-relative spelling** — lost to
`design@core@every-path-names-its-anchor`. `live`. It is one character where the project name is
five. It loses on shape: the root is a component and already has a spelling under the one
grammar, so a second one puts two shapes on one meaning — and it spends the prefix the census
above shows is needed for the generic and escape meanings.

**Widening the retired form's suffix set by one entry** — superseded by
`design@core@every-path-names-its-anchor`, which retires suffix sets with the form that carried
them. `live`. The measured cost of the set: relocating thaum's nine game-driving suites moved
every backticked Rust path across five of its knowledge documents and the checker reported none of
them, because no suffix in the set covered them; the sweep had to be grep-driven.

**`syn` as the Rust parser** — lost to `design@core@grammars-not-prefixes`. `live`. Prototyped: it
parses all 68 files without error, but doc comments survive only as attributes and ordinary comments
are discarded during lexing, so a rule cited in a `//` comment inside a function body needs a second
raw-text pass. One parser that answers every question beats two that each answer half.

**Rust-analyzer's syntax crate** — lost to `design@core@grammars-not-prefixes`. `live`. Pure Rust,
lossless and it keeps comments, but it is published per nightly, so adopting it means pinning a
crate of compiler internals that churns weekly.

**A hand-rolled indentation scanner**, leaning on the formatter gate to normalise indentation — lost
to `design@core@grammars-not-prefixes`. `live`. Zero dependencies, and rejected because a mis-scope
would be silent, which is the failure class this tool exists to prevent. The line-prefix scanner it
would have resembled was the single cause of four recorded defects.

**`--lines` on thaum's interpretation index**, a temporary copy carrying each citation's line
numbers — lost to `design@core@generated-files-are-pure`. `live`. Refuted by a census over the file's whole
history rather than by argument: `git log -p -- docs/rules/interpretations/index.md` matches no
line-numbered citing row, so in every commit the index has ever had, the flag's output was never
one of them. Every generated index gates on data that does not move when unrelated prose shifts a
line — the interpretation index carries one row per entry, with no line and no citing list at all,
and the rule index names its citing files at file level — so per-line data stood against the
argument for the file it would have sat in. What it would have bought is answered instead by
filtering `cargo klarch model` on the rule number.

**`cargo klarch index` prints the diff it would apply, and `--write` applies it** — lost to
`design@core@generated-files-are-pure`. `live`. This is the `cargo fmt` / `cargo fmt --check` shape,
and it lost to a prior-art survey rather than to reasoning. `fmt`, `gofmt` and `prettier` each
build the check into the generator, and the discriminator is that **none of them had a separate
verifier to build it into anything else**. Here the `generated` check of `cargo klarch check` is a gate
that already names the first differing line, so the split those tools could not make is already
made, and a second command answering the same question in its own format would leave nothing to
say which of the two was right.

**Every string literal outside the checker read as prose, bound to a name or not** — lost to
`design@core@grammars-not-prefixes`' binding rule. `live`. Proposed to remove the three special
cases the binding rule costs the extractor, on a measurement that found no bound literal outside
the checker carrying rule-shaped content. The measurement was wrong: built, the checker reported
six in three crates, an address, formatted figures a display test expects and a line of tool
output a parser is fed, and the class recurs wherever a test displays a three-digit float.

**A fixture marker, a macro whose token tree the checker reads as data** — lost to
`design@core@checker-source-literals-are-data`. `live`. It frees every fixture and needs a
convention at each one, which the location does not. It is the recorded fallback should a literal
outside the tool ever need rule-shaped bytes as data where no binding can hold it.

**Moving the tool's unit tests under `path@core@tests/` and excluding the directory** — lost to
`design@core@checker-source-literals-are-data`. `live`. Integration tests reach only public items.
Measured: 31 files carry a test module, 349 tests, most over private functions, so the
move makes those public or drops the tests.

**Literals as data in every `cfg(test)` module of every crate** — lost to
`design@core@checker-source-literals-are-data`. `live`. Zero members outside the tool today, and the
owner's ruling is that a rule cited in another crate's unit test stays checked.

**Reading no string literal anywhere** — lost to `design@core@checker-source-literals-are-data`.
`live`. Measured: 17 assertion messages in the tests of thaum's `thaum-testing` crate cite a rule
behind a marker and are checked today. Each would become text nothing reads.

**The binary finding the checker's directories at run time**, from the tree or its manifest, in
place of refusing a binary built elsewhere — lost to `design@core@a-foreign-build-is-refused`.
`live`. Reproduced on thaum's repository: a binary built from a second checkout into the first one's
target directory, run in the first, printed a verdict string changed only in the second, so the
other checkout's code runs and not only its compiled paths; correct paths would still judge the
tree with the wrong code. The exemption half of the argument is the manifest-row entry below.

**A manifest row naming the exempt directory** — lost to `design@core@the-regime-has-no-opt-out`.
`live`. A row can be pointed at any directory, and the tree declaring it decides what conformance
means. The compiled path can name only the checker's own source.

**`additional-trackers` kept as a file list** — lost to
`design@core@anchors-are-components-and-locations`. `live`. It named each tracker outside every
component directly, needed no second kind of anchor, and every check that read it worked. It
loses because a file in that list had no anchor: nothing in the project could cite what was open
there, and the register thaum's rules directory holds could not be named at all. The whole of this
tool is about citing, so a recorded thing with no name is the one shape it may not have.

**A `theme` metadata key in place of group subdirectories** — lost to
`design@core@a-file-register-is-a-directory-of-entries`. `live`. It keeps every entry in one flat
directory and regroups by editing one line. It loses on two counts: a subdirectory is visible to
`ls` and to a listing without parsing any file, and a group that is not part of an entry's id
makes regrouping a `git mv` that breaks no reference — which a metadata key would also have to
promise, and could not, without a second rule saying the key is not part of the id.

**The index introduction as a string in the manifest** — lost to
`design@core@a-file-register-is-a-directory-of-entries`. `live`. It would make the whole of a
generated index generated, with nothing hand-written beside it. It loses because the introduction
is prose about the project and belongs in markdown, and because a hand-written README is also what
keeps the instance directory in git: an empty directory is not a thing git tracks, so a register
with no entries would have no home at all.

**An optional index, generated only where a project asks for one** — lost to
`design@core@a-file-register-is-a-directory-of-entries`. `live`. It was argued on churn: a
generated file in every instance is a file that goes stale and fails the gate. It loses once the
README holds the directory open, because the rows change only on create, delete, retitle, regroup
and a metadata change — not on a wording edit — so the churn the argument feared does not happen.
An optional index is also an opt-out, which `design@core@the-regime-has-no-opt-out` refuses.

**A summary column, or a last-change date, in a file register's index** — lost to
`design@core@a-file-register-index-is-rows`. `live`. Either would let a reader take in what an
instance holds without opening a file, which is the whole point of a listing. The summary loses on
churn: it changes on every wording edit of every entry, so an index that today is regenerated on
create, delete, retitle, regroup and a metadata change would go stale on each one and fail the gate.
The date loses outright to `design@core@generated-files-are-pure`: the last change is git's, not the
walked tree's, so a generator reading it would rewrite the file on a rebase and report a file stale
that nobody had touched. Both are answered by a command instead, `issues` for the dates and
`show <ref>` for the body.

**Per-instance register options declared at the root, in the project's own instance** — lost to
`design@core@registers-are-declared`. `live`. It puts every option in one place beside the
manifest, with no second configuration file to find. It loses on where the file would sit: the
root component's own issue directory would carry the groups of every other component's issue
instance, which is the one-home failure written into the layout.

**One link shape, the plain inline form with a spaceless target**, so that the scanner stays
one pattern — lost to `design@core@links-are-navigation-rows`. `live`. A row written in
another shape was not silent: an angle-bracketed target was read with its brackets and an
existing file was reported as missing, which a test in a navigation README showed. A directory
home's README written with link definitions would have been reported as linking none of its
subdocuments. Both are wrong findings, where the head had judged the gap to be a silence.

**Refusing to load a manifest whose register declaration is wrong** — lost to
`design@core@registers-are-declared`. `live`. A declaration the tool cannot act on is a could-not-run
by `design@core@exit-code-ladder`, and exiting 2 is what a manifest the grammar cannot
parse does, a key of `[project]` it does not know included. It loses for the case where the tool
CAN act: a bad `dir` on a built-in has a compiled default to fall back on, so refusing the whole
run would report nothing at all about the tree — and nothing at all is what a session reads as
conformance. An unknown key keeps the refusal because there is no default to fall back on.

**An open set of issue kinds, the label read off each entry as written** — lost to
`design@core@a-file-register-is-a-directory-of-entries`. `live`. It was the incumbent: the listing
derived an entry's label from the tag its own title carried rather than matching a list,
because an entry written with a kind nobody anticipated is intended, and matching against a list
made such an entry fall through to a label that is also a real kind. It loses once the kind
decides which subsections an entry owes: an unanticipated kind then owes the wrong three, silently,
and no reader can tell. A closed list makes an unknown kind a finding naming the list and an
addition a reviewed manifest diff, which is what the concern list already did; the failure case the
open set was written against — a wrong label — cannot occur once an unknown kind is refused. No
fallback label exists now: `path@core@src/records.rs` reads each entry's kind out of
its own frontmatter, and an entry that declares none has no kind rather than a wrong one.


**A model generic over the extension's observation type**, `Model<E>` with the extension's
observations in each document's list — lost to `design@core@an-extension-builds-its-own-model`.
`live`. It scans each document once, and the extension's observations are typed and sit beside the
core's. It loses because the type parameter reaches every type and every check signature that
touches a document, to carry data the core never reads: at the split, 18 of the 28 Rust files of
the core's library and binary named `Model`, `Document`, `Observation` or `Located`
(`git grep -lwE`). Two extensions would also need a hand-written composition of their two types.

**A type-erased slot per document**, a map from extension name to a boxed value that each extension
downcasts — lost to `design@core@an-extension-builds-its-own-model`. `live`. It keeps one
model and needs no type parameter. It loses because the extension keeps the same data outside the
core with its type checked at compile time, so the slot adds a downcast that can fail at run time
and gains nothing.

**A namespace for an extension's tables**, `[extensions.<name>]` — lost to
`design@core@an-extension-claims-its-manifest-tables`. `live`. It shows in the manifest itself
which tables are an extension's. It loses because the refusal of an unclaimed table already reports a
table nobody owns, and because it changes the manifest format of every project and every mock
for no check that reads the difference.

**The interpretation entry number kept as a retired form permanently**, a bare `R` followed by
digits reported wherever prose holds it — lost to
`design@core@candidate-rule-and-retired-forms`. `live`. It was kept because a retired form
must stay visible or the migration is unfinishable, and it was read in every prose region, Rust
comments included. It loses because the migration is finished, the tree holds no such span, and
the number is thaum's own register's: kept, the lint would have had to move into thaum's extension
when the core was split from the rules half. The commit history holds the form in 65 of the 500
messages before that split, counted with the lint's own pattern; the slug reference, which stays
a finding, is in 64 of them, so exposure in history does not separate the two.

**A slug defined at the end of any level-two or level-three heading, or in a table cell, with no
level declared by the register** — lost to
`design@core@an-entry-is-a-heading-at-the-register-level`. `live`. It needed no manifest
key, it let a register keep entries at either level, and the table cell served decision tables
written before the heading form. It loses because it cannot tell a heading that is no entry from
an entry whose slug is missing: a tripwire written without a slug is defined nowhere, listed by
no `tripwires` run and reported by no check, and a reviewer reading the listing as the list of
what to re-read never opens it. With one declared level per register, every heading at that level
is an entry and one without a slug is a finding. A table row has no heading, so a decision defined
in a cell is missing from the document outline, which is the index the heading form exists to
give.

**Table cells kept as a second definition site, behind a per-register manifest key** — lost to
`design@core@an-entry-is-a-heading-at-the-register-level`. `live`. It would have kept decision
tables readable as tables, and left the declared level to govern headings alone. It loses because
one register would then have two definition sites, and the outline would again fail to list every
entry of a register that opts in. The one home that held cell definitions converts to headings
without losing any content.

**CLAUDE.md required in every component, whatever the project declares** — lost to
`design@core@agents-table`. `live`. Its argument was that what a component carries is compiled in,
because a project free to declare its own set is checked against nothing, and CLAUDE.md is where a
component's contracts for a developer are routed, so requiring it guarantees them a home. It loses
for the agent document alone: the declaration that removes it removes nothing else, so every other
document is still compiled in and checked. Doubt remains on the second half of the argument, and
`issue@core@a-home-for-developer-contracts-outside-agent-configuration` records it.

**The implementation's module tree as the public API**, with only its visibility narrowed — lost
to `design@core@api-facade`. `live`. A consumer would learn the internal layout: thaum's rules
extension at thaum's e98e296 named 13 of the 16 modules public at aefb45a: the distinct names
`git grep -o "documentation::[a-z_]*"` prints over its tools/rules-corpus, the function
`component_dir` aside. Moving an item between files
would be a library break, and the public surface would be computed from the whole module tree
rather than read in one file.

**`#[non_exhaustive]` on every public enum expected to grow** — lost to `design@core@ne-minimal`.
`live`. It assumed the attribute prevents breakage. It relabels a break that hits only the
consumers who opted into an exhaustive match, and removes that option from every consumer. Under
0.x it buys no version number, because major and minor both bump 0.MINOR.

**A struct literal of `Inputs` in each caller, accepting a break per new input** — lost to
`design@core@the-core-cli-is-a-library-module`, which gathers the inputs. `live`. The checker
surface of v0.1 added two inputs and broke all five literals in thaum's rules extension; each
later input would do the same.

**The items only the core's own tests use kept public under `#[doc(hidden)]`** — lost to
`design@core@api-facade`. `live`. It publishes a second contract that nothing states, beside the
one the facade states.

**The items only the core's own tests use gated behind a `testing` feature** — lost to
`design@core@api-facade`. `live`. A unit-test module inside the crate reaches them with no feature
and no second surface, and the feature gated nothing before.

**A default body that does nothing on every hook added to the extension traits** — lost to
`design@core@trait-defaults`. `live`. A judging hook defaulted to nothing turns a compile error
into a check that silently does not run, with exit 0.

**The primer delivered by a session-start hook that prints it** — lost to
`design@core@owned-namespace-check`, which delivers it by an import line in the root CLAUDE.md.
`live`. A hook needs the installer to edit the user's settings file, which belongs to the user,
and it needs a shell. It is kept because a doubt remains: the import is verified for the main
session and a subagent of the `claude` harness, and another context or harness might not follow
it.

**The binary named knowledge-architect, after its package** — lost to `design@core@binary-name`.
`live`. The binary's name is the default of the declared command, which a project types at every
run, and the owner wanted it short to type. Renaming the binary is a change to the command-line
surface every installing project scripts against, so the alternative will be raised again whenever
the name is questioned.

**An install record file listing what the installer wrote** (option (c) of the owned namespace) —
lost to `design@core@owned-namespace-check`. `live`. The checker would read a committed file of its
own format to know which installed files are its own. It is one more committed file, which can
itself be edited, while the prefix makes ownership readable from a path alone with no record and
no history.
