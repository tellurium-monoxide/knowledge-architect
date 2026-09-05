# Rejected Alternatives

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
one of them. The index gates on **file-level** citing lists deliberately — that is what keeps it
from moving when unrelated prose shifts a line — so per-line data stood against the argument for
the file it sat in. What it would have bought is answered instead by filtering
`cargo knowledge model` on the rule number.

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
Measured: 31 files carry a test module, about 300 tests, most over private functions, so the
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
