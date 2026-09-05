# Rejected Alternatives

**A hand-rolled gitignore matcher reading the root file alone**, with the walk pruning what it
covered — lost to `knowledge#git-supplies-the-walk`. `live`. It needed no `git` on the path and no
process per run, and its supported subset was chosen against this repository's own file. It loses
on three counts, each measured against the tree rather than argued: a tracked live document left
the walk AND the inverse assertion when one root ignore line matched its bare name at any depth,
reproduced with a one-line `.gitignore` over a copy of `knowledge@tests/projects/minimal/` and
recorded as a defect before the reversal; a nested `.gitignore` was not honoured at all, so a
project needing one had to declare the path in the manifest; and every pattern the matcher could
not honour — a negation, `**`, `?`, a character class — had to be refused by name, because
dropping an ignore rule makes the walk read more than it should and dropping a negation makes it
read less, and both are silent. Git answers all three by construction, and the walked-file count
over this repository was identical under the two.

**Families named for the subjects that read them** — lost to `knowledge#families-are-the-checks`. `live`. A
family per review axis would let a reader ask for its own subject in one word instead of listing the
checks that serve it. It loses on `thaum@tools/README.md`: nothing about this repository is compiled into the
tool, and every list a check reads comes from `thaum@knowledge.toml`. A subject's name inside
`Only::parse` is exactly that repository knowledge, compiled in. Which families serve a subject
belongs to whoever reads them.

**One family per invocation, instead of a set** — lost to `knowledge#families-are-the-checks`. `live`. It
needs no set type and no comma parsing, and each invocation stays one word. It loses to
`knowledge#model-then-checks`, which records what a walk costs: the walk happens once per invocation, so a
caller wanting five families reads every live document five times, which is the shape the single
walk was built to remove.

**A slug unique across the whole project, with the component named for the reader only** — lost to
`knowledge#a-slug-belongs-to-a-component`. `live`. It keeps one meaning per word everywhere and needs
no lookup to resolve a reference. It loses because it makes every component's vocabulary global: two
components cannot each decide something they call the same word, and the second one to want the word
has to take a worse one.

**A reference with no component read as one inside its own component** — lost to
`knowledge#a-slug-belongs-to-a-component`. `live`. It would leave a pointer inside a component as
short as it was before components existed, and qualify only the crossings. It loses on what a
reference has to carry by itself: the same text would name different decisions depending on which
file it sits in, so moving a document between components would silently retarget every unqualified
reference in it.

**A bare root-relative path form, with a targeted check on the collision cases** — lost to
`knowledge#every-path-names-its-anchor`. `live`. It keeps prose short and costs no migration.
It loses on what was counted: the required document set gives every component the same names,
so a bare `docs/`-headed reference from inside a component silently names the project's file —
twelve candidate sites in one component alone, nine of them wrong, counted by hand when the
defect was recorded. A targeted check patrols the collision; retiring the form makes it
unrepresentable, and an anchored reference is what a fixed-string grep can find.

**A fenced reference read as an illustration**, the stance slug references took before the
`@` grammar — lost to `knowledge#candidate-rule-and-retired-forms`. `live`. It let a document
explaining the convention hold an example without inventing a slug. It loses on consistency:
path references were already live in a fence, so one kind had two stances, and a sketch in a
design document names its decisions as deliberately as it names its paths. The illustration is a
placeholder in angle brackets, which the tokenizer does not record.

**`#` as the reference separator**, the incumbent of `` `<component>#<slug>` `` — lost to
`knowledge#a-slug-belongs-to-a-component`, which fixes `@`. `live`. It costs no migration of
the slug references. It loses on collision: `#` is a Rust attribute opener and a markdown
heading marker, both of which sit inside code spans in this tree, and `:` — the other
candidate — is a Rust path separator; `@` meets almost nothing in Rust and only email
addresses in prose, which never sit in backticks here. It entered the tree with the first
migration to qualified references, carrying no argument of its own.

**A configurable reference separator** — lost to `knowledge#a-slug-belongs-to-a-component`,
which fixes `@`. `live`. A project could pick the character its prose collides with least. It
loses because every instruction, every skill and the future link preprocessor would be
parameterised on it, and `@` already meets almost nothing in Rust and only email addresses in
prose, which never sit in backticks here.

**Paths without the kind prefix**, keeping `` `<anchor>@<path>` `` beside
`` `<kind>@<anchor>@<id>` `` — lost to `knowledge#every-path-names-its-anchor`. `live`. It costs
no migration of the path references. It loses on what it leaves in the scanner: two grammars, told apart by segment count, and an unsupported-shape lint
that keeps its heuristic instead of becoming "unknown kind". Five characters at every path
reference bought one tokenizer and one candidate rule.

**A register reference with no anchor**, `<kind>@<id>` for a register a project has only one
instance of — lost to `knowledge#a-slug-belongs-to-a-component`. `live`. It is shorter for the
common case. It loses because the extracted tool cannot know which register is single-instance
in a given project, and one three-part grammar serves every kind without a special case in the
resolver or in the instructions.

**Keeping the `@` prefix as the escape** — lost to `knowledge#reserved-anchors`. `live`. It
costs no migration. It loses on the census taken at the design session: the tree held 75
`@`-prefixed spans carrying three meanings — about 65 meant every component's own copy, a
handful meant a path outside the tree, three were Java annotations that were never path
syntax — and the scanner registered none of them, so a typo'd escape was invisible by
construction. One mute marker for three meanings is the confusion the anchor words dissolve.

**The `@` prefix reused as the root-relative spelling** — lost to
`knowledge#every-path-names-its-anchor`. `live`. It is one character where the project name is
five. It loses on shape: the root is a component and already has a spelling under the one
grammar, so a second one puts two shapes on one meaning — and it spends the prefix the census
above shows is needed for the generic and escape meanings.

**Widening the retired form's suffix set by one entry** — superseded by
`knowledge#every-path-names-its-anchor`, which retires suffix sets with the form that carried
them. `live`. The measured cost of the set: relocating the nine game-driving suites moved
every backticked Rust path across five knowledge documents and the checker reported none of
them, because no suffix in the set covered them; the sweep had to be grep-driven.

**A third-party mirror as the source** — lost to `knowledge#watch-reads-the-page`. `live`. Mirrors keep
stable index pages and would be less brittle than Wizards' HTML. It loses on what the corpus is: the
Comprehensive Rules are this project's only authority, and putting a third party between the project
and its authority as the _change signal_ is a dependency nothing else here has.

**A per-rule deferral in the manifest, retiring itself at a backlog of zero** — lost to
`knowledge#the-regime-has-no-opt-out`. `live`. It was built to carry one migration and it did, but
what it left standing is a way for any tree to be conformant with less than the regime. A deferral
names a rule, so it applies to every file and expires with none; the file-level exemption it is
replaced by expires with its subject. The self-retiring report made the list hard to forget and did
nothing about that.

**Banning elision outright**, so every quote is a full body checked by equality — lost to the
completeness rules `knowledge#scope-and-distance` sits beside. `live`. It is the strongest check
available and the cost was measured: about 90 000 characters of rule text across the tree, roughly
1 200 lines, with the operative clause buried inside each one. Marking every omission delivers the
content of the incident this would have prevented.

**A blessed structural form for section references owing nothing**, the space form for location
and the marker only for claims — lost to `knowledge#sections-cite-the-heading`. `live`. It costs
nearly no migration and reads naturally. It loses on what it leaves open: a claiming reference
written in the unmarked form passes every check forever, which is the exact gap the census was
taken to close — and the census showed the two uses are not mechanically separable, so no lint
could ever tell an honest structural use from a dodge.

**A lint that detects claiming references**, reporting a section number only where the sentence
claims content — lost to `knowledge#sections-cite-the-heading`. `live`. Classified by hand over
the whole tree: claiming and structural references differ only in what the surrounding sentence
does, with no separating pattern, so the lint either drowns in the structural majority or misses
the claims it exists for. The shape test is decidable; the claim test is the reviewer's.

**A checker for verbatim rule quotes that nothing marks** — lost to the rules-reviewer owning the
class, per its definition's sweep instruction. `live`. Measured over `interpretations/`: checking
every plain quoted span against the corpus produces four false positives per true finding — the
project's own coinages, card text and quoted external prose all sit in plain quotes — and a gate
at that ratio gets muted. The buildable version reads only spans already written as quotes; the
dangerous case is the unquoted verbatim, which no span scanner sees.

**A third marker for the quote-free class**, beside the content marker and the identifier form —
lost to reading a number as data by its surface. `live`. Defeated by enumeration: every genuine
quote-free use is a number being displayed, and the code span and the name-bound string
literal already say so structurally. A new marker would have been a place to hide.

**A per-usage declaration of data spans**, each carrying the whole source line — lost to the same
structural reading. `live`. Half the sites sit in doc comments that `cargo fmt` reflows, so the
declaration would break on commits that changed nothing but whitespace, and a gate that cries wolf
earns exemptions of its own.

**A permissive discharge by slug pointer**, a marker present with its quote at the far end of the
pointer — lost to the pointer REPLACING the marker. `live`. A marker present means the file is
listed in the generated rule index as citing that rule, so a file that merely points at a decision
joins the bump work list for a rule it does not depend on.

**`syn` as the Rust parser** — lost to `knowledge#grammars-not-prefixes`. `live`. Prototyped: it
parses all 68 files without error, but doc comments survive only as attributes and ordinary comments
are discarded during lexing, so a rule cited in a `//` comment inside a function body needs a second
raw-text pass. One parser that answers every question beats two that each answer half.

**Rust-analyzer's syntax crate** — lost to `knowledge#grammars-not-prefixes`. `live`. Pure Rust,
lossless and it keeps comments, but it is published per nightly, so adopting it means pinning a
crate of compiler internals that churns weekly.

**A hand-rolled indentation scanner**, leaning on the formatter gate to normalise indentation — lost
to `knowledge#grammars-not-prefixes`. `live`. Zero dependencies, and rejected because a mis-scope
would be silent, which is the failure class this tool exists to prevent. The line-prefix scanner it
would have resembled was the single cause of four recorded defects.

**A citation locator, given a rule number and a file** — lost to `knowledge#model-then-checks`.
`live`. Proposed because a grep is genuinely unreliable for the job: the scanner distinguishes four
citation shapes a text search does not — the prose marker, the identifier marker, a blockquote bound
by the number printed at its head, and an inline quote bound to the nearest preceding marker — and it
excludes rule numbers inside code spans and name-bound string literals, which a grep reports as hits.
It loses because `cargo knowledge model` already emits `file`, `line`, `kind` and `value` with
markers and rule tokens as separate kinds, so filtering that dump on a rule number **is** the
locator, from the scanner's own notion of a citation rather than from a pattern. Established by
running it: the filter lists a rule token on a line carrying no marker, which is the shape a
marker-grep misses and a number-grep cannot classify. What was missing was not a command but a line
in `knowledge@README.md` saying so. The tripwire on the completeness of that dump is in
`knowledge@docs/tripwires.md`.

**`rules diff --local`, the form that compared the vendored text against one named release** —
lost to `thaum#named-values-where-order-decides`, which settled `rules diff` as two named
releases. `live`. It loses to a mechanism rather than to an argument, and the mechanism is why
dropping it costs nothing: `resolve` in `knowledge@rules/src/release.rs` answers `local` **first**,
so a diff against the pinned date resolves the same bytes the flag would have. The two forms
differ only where the working-tree text disagrees with the version file, and `vendor` writes both
in one call, so no supported flow separates them. It was documented in no README and used by no
skill.

**`--lines` on the interpretation index**, a temporary copy carrying each citation's line numbers
— lost to `knowledge#generated-files-are-pure`. `live`. Refuted by a census over the file's whole
history rather than by argument: `git log -p -- docs/rules/interpretations/index.md` matches no
line-numbered citing row, so in every commit the index has ever had, the flag's output was never
one of them. Every generated index gates on data that does not move when unrelated prose shifts a
line — the interpretation index carries one row per entry, with no line and no citing list at all,
and the rule index names its citing files at file level — so per-line data stood against the
argument for the file it would have sat in. What it would have bought is answered instead by
filtering `cargo knowledge model` on the rule number.

**`cargo knowledge index` prints the diff it would apply, and `--write` applies it** — lost to
`knowledge#generated-files-are-pure`. `live`. This is the `cargo fmt` / `cargo fmt --check` shape,
and it lost to a prior-art survey rather than to reasoning. `fmt`, `gofmt` and `prettier` each
build the check into the generator, and the discriminator is that **none of them had a separate
verifier to build it into anything else**. Here `cargo knowledge check --only generated` is a gate
that already names the first differing line, so the split those tools could not make is already
made, and a second command answering the same question in its own format would leave nothing to
say which of the two was right.

**Every string literal outside the checker read as prose, bound to a name or not** — lost to
`knowledge#grammars-not-prefixes`' binding rule. `live`. Proposed to remove the three special
cases the binding rule costs the extractor, on a measurement that found no bound literal outside
the checker carrying rule-shaped content. The measurement was wrong: built, the checker reported
six in three crates, an address, formatted figures a display test expects and a line of tool
output a parser is fed, and the class recurs wherever a test displays a three-digit float.

**A fixture marker, a macro whose token tree the checker reads as data** — lost to
`knowledge#checker-source-literals-are-data`. `live`. It frees every fixture and needs a
convention at each one, which the location does not. It is the recorded fallback should a literal
outside the tool ever need rule-shaped bytes as data where no binding can hold it.

**Moving the tool's unit tests under `knowledge@tests/` and excluding the directory** — lost to
`knowledge#checker-source-literals-are-data`. `live`. Integration tests reach only public items.
Measured: 31 files carry a test module, 349 tests, most over private functions, so the
move makes those public or drops the tests.

**Literals as data in every `cfg(test)` module of every crate** — lost to
`knowledge#checker-source-literals-are-data`. `live`. Zero members outside the tool today, and the
owner's ruling is that a rule cited in another crate's unit test stays checked.

**Reading no string literal anywhere** — lost to `knowledge#checker-source-literals-are-data`.
`live`. Measured: 17 assertion messages in `thaum-testing`'s tests cite a rule behind a marker and
are checked today. Each would become text nothing reads.

**A manifest row naming the exempt directory** — lost to `knowledge#the-regime-has-no-opt-out`.
`live`. A row can be pointed at any directory, and the tree declaring it decides what conformance
means. The compiled path can name only the checker's own source.

**`additional-trackers` kept as a file list** — lost to
`knowledge#anchors-are-components-and-locations`. `live`. It named each tracker outside every
component directly, needed no second kind of anchor, and every check that read it worked. It
loses because a file in that list had no anchor: nothing in the project could cite what was open
there, and the register the rules directory holds could not be named at all. The whole of this
tool is about citing, so a recorded thing with no name is the one shape it may not have.

**A `theme` metadata key in place of group subdirectories** — lost to
`knowledge#a-file-register-is-a-directory-of-entries`. `live`. It keeps every entry in one flat
directory and regroups by editing one line. It loses on two counts: a subdirectory is visible to
`ls` and to a listing without parsing any file, and a group that is not part of an entry's id
makes regrouping a `git mv` that breaks no reference — which a metadata key would also have to
promise, and could not, without a second rule saying the key is not part of the id.

**The index introduction as a string in the manifest** — lost to
`knowledge#a-file-register-is-a-directory-of-entries`. `live`. It would make the whole of a
generated index generated, with nothing hand-written beside it. It loses because the introduction
is prose about the project and belongs in markdown, and because a hand-written README is also what
keeps the instance directory in git: an empty directory is not a thing git tracks, so a register
with no entries would have no home at all.

**An optional index, generated only where a project asks for one** — lost to
`knowledge#a-file-register-is-a-directory-of-entries`. `live`. It was argued on churn: a
generated file in every instance is a file that goes stale and fails the gate. It loses once the
README holds the directory open, because the rows change only on create, delete, retitle, regroup
and a metadata change — not on a wording edit — so the churn the argument feared does not happen.
An optional index is also an opt-out, which `knowledge#the-regime-has-no-opt-out` refuses.

**A summary column, or a last-change date, in a file register's index** — lost to
`knowledge#a-file-register-index-is-rows`. `live`. Either would let a reader take in what an
instance holds without opening a file, which is the whole point of a listing. The summary loses on
churn: it changes on every wording edit of every entry, so an index that today is regenerated on
create, delete, retitle, regroup and a metadata change would go stale on each one and fail the gate.
The date loses outright to `knowledge#generated-files-are-pure`: the last change is git's, not the
walked tree's, so a generator reading it would rewrite the file on a rebase and report a file stale
that nobody had touched. Both are answered by a command instead, `issues` for the dates and
`show <ref>` for the body.

**Per-instance register options declared at the root, in the project's own instance** — lost to
`knowledge#registers-are-declared`. `live`. It puts every option in one place beside the
manifest, with no second configuration file to find. It loses on where the file would sit: the
root component's own issue directory would carry the groups of every other component's issue
instance, which is the one-home failure written into the layout.

**Refusing to load a manifest whose register declaration is wrong** — lost to
`knowledge#registers-are-declared`. `live`. A declaration the tool cannot act on is a could-not-run
by `thaum#exit-code-ladder`, and exiting 2 is what the retired keys do. It loses for the case where
the tool CAN act: a bad `dir` on a built-in has a compiled default to fall back on, so refusing the
whole run would report nothing at all about the tree — and nothing at all is what a session reads
as conformance. The two retired keys keep the refusal because there is no default to fall back on.

**An open set of issue kinds, the label read off each entry as written** — lost to
`knowledge#a-file-register-is-a-directory-of-entries`. `live`. It was the incumbent: the listing
derived an entry's label from the tag its own title carried rather than matching a list,
because an entry written with a kind nobody anticipated is intended, and matching against a list
made such an entry fall through to a label that is also a real kind. It loses once the kind
decides which subsections an entry owes: an unanticipated kind then owes the wrong three, silently,
and no reader can tell. A closed list makes an unknown kind a finding naming the list and an
addition a reviewed manifest diff, which is what the concern list already did; the failure case the
open set was written against — a wrong label — cannot occur once an unknown kind is refused. No
fallback label exists now: `knowledge@documentation/src/records.rs` reads each entry's kind out of
its own frontmatter, and an entry that declares none has no kind rather than a wrong one.


**`check` reading HEAD's message when the tree is clean** — lost to
`knowledge#a-commit-message-is-a-document`. `live`. It needs no second command and no range: the
one check the gates already run would judge the message of the commit it stands on. It loses
because the verdict then depends on the working tree rather than on the checkout: `git stash`
turns a run that judged a message into a run that judged none, and back, with no finding either
way. It also judges each message exactly once, at the moment its own commit is HEAD, so a
message written before a decision was deleted is never re-judged and the closure work list of
`knowledge#one-entity-table` never reaches it.

**References and present quotes only, in commit messages** — lost to
`knowledge#a-commit-message-is-a-document`. `live`. It keeps the cheap half of the regime — a
name that resolves, a quote that verifies — and drops the obligation to quote at all, so a
message may name a rule and say what it says without carrying its text. It loses on what a
message is: history, read years later against a corpus that has moved. Rules are edited and
renumbered at comparable rates between releases, measured across two of them, so a number
carrying no text is a claim its reader cannot check even where the tree still holds the right
rule — and the tree the message described is not the tree that reader has.

**`check` reading the hook's status** — lost to `knowledge#a-commit-message-is-a-document`.
`live`. It makes the one gate everybody runs report a clone that will commit unjudged messages,
which is the moment the information is worth most. It loses for the reason
`knowledge#ignored-targets-are-not-asserted` refuses build state: `core.hooksPath` is per-clone
configuration, so a check reading it answers differently on two clones of one commit, and a
verdict nobody can reproduce is a verdict nobody can act on. `cargo x gates` prints the line
beside its verdicts instead, where it informs and gates nothing.
