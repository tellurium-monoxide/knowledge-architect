# Open issues — the knowledge tool

What is outstanding about `knowledge` itself: the checks, the release machinery, the archive.
What is outstanding about the _rules and their readings_ is `thaum@docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

## The interpretation register has no mention form, so naming an entry counts as citing it `question`

**What.** The interpretation index counts every `R`-number token in a live markdown file as a
citation of that entry, so a document that merely _names_ one — to point at a diff, to say where
a reading is filed — is listed in `thaum@docs/rules/interpretations/index.md` beside the
documents whose argument depends on the reading. The `references` family reports every such
token as a retired form since the `@` grammar landed, and `check::interpretations` no longer
resolves them; the index is the one reader left. Rule numbers have a way to say it and `R` numbers do not: a rule number that is data
goes inside a code span or a name-bound string literal, and the walk does not read those as
citations. No entry is named in this file on purpose, so that the illustration does not
create the thing it illustrates.

**Observed.** Writing the entry this one replaces, which asked whether the index should store line
numbers. Its first draft named two entries by number as examples of index churn; regenerating then
moved both from 6 citing files to 7 and from 4 to 5, the index counting that draft as citing them.
It did not — it named them to point at a diff. The draft was rewritten around the problem by naming
the concern files instead.

**Why it matters.** The index is the per-entry half of a release bump, and a citing list is read as
a work list: a file listed there with no dependency on the reading sends someone to check something
that cannot have broken. The error is one-directional and grows with the register — every document
that discusses the register rather than resting on it inflates the blast radius of every entry it
names.

**What would answer it.** Decide whether an `R` number inside a code span stops counting as a
citation, the way a rule number in one does. That is the structural answer the rule side already
uses, and it needs no new marker. The decision belongs with `recording-an-interpretation`, which
owns the entry shape.

## Whether register entries should carry text slugs in place of `R` numbers `question`

**What.** Interpretation entries are named by an `R` and a sequence number, and the owner
proposed replacing the numbers with content-named slugs, the shape design decisions already use.
The numbering's purpose is unclear, a number is inconvenient to cite from memory, and a text slug
would dissolve the mention-form question above along the way — a slug in a code span is already
the established data shape, while an entry number written anywhere counts as a citation, which
this entry's own first draft demonstrated by joining two citing lists in the generated index.
Proposed and deliberately deferred at the linting-scope discussion.

**Why it matters.** Every document and doc comment that cites a reading carries the number, so
the rename grows more expensive with each entry — and the mention-form question stays open as
long as the numbers do.

**What would answer it.** The next tool-cleanup discussion, arguing it with
`recording-an-interpretation`, which owns the entry shape and the numbering. The migration cost
is enumerable at that point from the register index's citing lists.

## The release diff calls a renumbered section a deletion `defect`

**What.** `rules diff` detects a renumbered rule by its body — `by_body` in
`knowledge@rules/src/diff.rs` builds its move maps from `Corpus::iter`, which deliberately
excludes sections — so a section that is renumbered between releases is reported `GONE` while its
own subrules are reported `MOVED` directly beneath. A retitled section is reported correctly as
changed.

**Reproduce.** Two mock corpora, the second renumbering a section and moving its rules with it —
a heading line and one dotted rule under each number. `rules diff` over them prints the section
as gone directly above the subrule move that proves the renumbering, and the summary counts one
renumbered and one gone. Found by an adversarial review of the branch that made sections citable.

**Why it matters.** Section citations owe heading quotes now, so a release that renumbers a
section breaks every citing site, and the diff is the work list a bump reads — per
`knowledge#sections-cite-the-heading`, the index's section rows say *where*, and the diff is what
says *what kind* of change moved under them. Root `CLAUDE.md` calls renumbering the dangerous
case, and `bumping-rules` reads the diff's verdicts as its instructions.

**What would close it.** Section move detection: a candidate is a vanished number whose title
appears under a new number, corroborated by its subrules' own moves — titles alone collide, the
pinned release printing the same title under two sections. Closing means the reproduction above
reporting the section as renumbered, and a genuinely deleted section still reporting `GONE`.


## A root gitignore line can unread a tracked live document `defect`

**What.** The walk prunes every path the root gitignore covers, and the survey drops it from the
files-outside listing too. Git itself does not ignore a file it already tracks, so the two
disagree exactly where it hurts: an unanchored basename pattern matches at any depth, and one
line meant for a scratch file removes a tracked live document from the walk AND from the inverse
assertion. Every citation, slug and path reference in it leaves every check, and the run reports
success.

**Reproduce.** A mock project copied from `knowledge@tests/projects/minimal/` whose `knowledge@tests/projects/minimal/notes/b.md`
carries a dangling anchored reference: without a gitignore the run fails on it; after adding a
one-line `.gitignore` holding the document's bare filename, the run passes with no finding and
nothing reports that a live file went unread. Found by an adversarial review of the
anchored-grammar branch.

**Why it matters.** This is the founding failure class: a file silently outside every check while
the run stays green, reachable by one plausible ignore line. The unsupported-pattern gate refuses
what the matcher cannot honour; it does not refuse a pattern that over-matches tracked files.

**What would close it.** The walk refusing to prune a path git tracks — which needs a source of
the tracked set the model build does not have today — or the gitignore parse refusing a bare-name
pattern that matches a tracked file, with the same loudness the unsupported patterns get. Either
way, the reproduction above must fail loudly instead of passing.

## Span and link shapes the scanner cannot see `observation`

**What.** The reference tokenizer records a backticked span holding an `@` and no whitespace,
backtick or angle bracket; the unanchored-path lint reads one character class with no `@`; and
the link scanner reads one inline pattern. Each shape below therefore produces neither a
reference nor a lint observation, and no finding:

```
`notes/a.md:12`             a colon suffix, the file-and-line idiom
a span wrapped across a line boundary
a fullwidth at sign in place of the ASCII one
[text][label]               a reference-style link, with its definition elsewhere
[text](<notes/a.md>)        an angle-bracketed target
```

Trailing punctuation inside the backticks, `` `path@<anchor>@notes/a.md,` ``, is not on the
list: the span is recorded as written and resolves to nothing, so it is reported as dangling.
The link shapes surface as a dangling target or an unlinked subdocument in
a design README, and are silent in an ordinary navigation file, per the close-enough clause of
`knowledge#links-are-navigation-rows`. Enumerated by an adversarial review of the
anchored-grammar branch; a grep at that revision found no live instance of any shape, so every
gap is latent.

**Why it matters.** The unanchored-shape lint promises that no pointer class passes
unregistered, and each shape above is a pointer a reader might write — the colon idiom most of
all. The cost of widening is false positives on prose, which the lint's two-segment clause was
tuned against; the census that tuned it did not measure these shapes.

**What would close it.** Widening the classes shape by shape with a measured false-positive
census for each, or recording beside `knowledge#every-path-names-its-anchor` that a named shape
stays outside on purpose. `assumption`: the colon idiom is the one worth widening first, being
ordinary editor output. Untested — no census taken.

## Nothing tests that `rules diff` receives its two dates the right way round `deferred`

**What.** `thaum#named-values-where-order-decides` exists because a reversed `rules diff` reports
newly added rules as gone and points every renumbering backwards, and `bumping-rules` reads those
verdicts as instructions. The direction is asserted at the parse layer, and `rules::diff::diff`
has its own unit tests, but the wiring between them — the dispatch in `knowledge@src/corpus_cmd.rs`
that hands `old` and `new` to `diff` — has no test. Found by an adversarial review of the clap
migration.

**Reproduce.** Swap the two arguments at that dispatch site so it passes `new, old`.
`cargo test -p knowledge` stays green.

**Why it matters.** It is the one seam the head's whole argument runs through, and a swap there is
silent by construction: the output names no release, only rule numbers and counts.

**Trigger, and why it is not a `todo`.** The test cannot be written against this tree today. It
needs two releases whose cited rules differ, and the two the tree holds — the pin and its one
archived predecessor — are a typographic re-export of each other: `rules diff` over them reports
`0 edited, 0 renumbered, 0 gone` in either direction, so no assertion over them discriminates. The
trigger is **the next rules bump**, which produces exactly such a pair and archives both. Whoever
runs that bump is already reading this diff's verdicts as their work list, so writing the test
that pins its direction is inside the work they are doing.

## The release cache sits at a predictable shared path `observation`

**What.** `resolve` in `knowledge@rules/src/release.rs` answers a release that is neither
vendored nor archived from `std::env::temp_dir()/MagicCompRules-<date>.txt`, downloading only
when that file is absent. When `MANIFEST.tsv` holds no row for the date — which is every bump
target by construction, since the manifest records only superseded releases — the digest has
nothing to compare against, so whatever bytes sit at that path are returned. A `bump` then
builds its diff, its effective-as-of report and its `CHANGES.md` skeleton from them.

**Reproduce.** An empty manifest and the cache pre-seeded with arbitrary bytes: `resolve`
returns those bytes as the release. First seen in the adversarial review of the first bump, and
now pinned by `a_release_the_manifest_does_not_know_resolves_from_whatever_the_cache_holds` in
`knowledge@rules/src/release.rs`, so a change that starts verifying them has to edit that test.

**What is already done.** `resolve` prints the digest of the bytes it returns in both cases, and
names their source — downloaded, or read from a pre-existing cache — through `provenance` in the
same file. A run therefore says which text it used, and no longer asserts a fetch that did not
happen. That turns silent trust into reported trust; it verifies nothing.

**Why it matters.** The temp directory is world-writable and the path is predictable, so a stale
or foreign file there is indistinguishable from a download. The blast radius is bounded: the
pinned text a bump writes is walked by `cargo knowledge check`, so grossly wrong bytes fail
hundreds of quote verifications loudly — but a subtly wrong text that preserved every cited rule
would pin silently. First contact with a new release is inherently unverifiable against a prior
record; trusting a shared path by existence is the avoidable part.

**Reachability is state-dependent, not structural.** `local` answers first, and `check` resolves
every release a document pins, reporting `2 of 2 release(s) resolved locally` on this tree — so
the cache branch is unreached only while every pin is vendored or archived. A `cr-version` marker
naming an unarchived release puts `check` itself on this path. `rules diff` against an unarchived
date, `rules fetch` and `rules bump` reach it by design.

**What would close it.** Caching under a user-owned directory, or not caching across runs at all
— the file is under 1 MB — leaves only the download in the window. Either means threading the
location through `Tree`, whose constructor has six call sites, because three tests seed the
current path directly to exercise the digest refusal, the fold-before-digest order and the
reproduction above. Deciding instead that a shared cache is accepted, and saying so in `resolve`'s
doc comment, closes this as a recorded trade-off.

## A rule marker inside a markdown HTML comment is read by nothing, and no instruction says so `question`

**What.** The markdown grammar marks every line inside an HTML comment as inert, and the scanner
takes no observation from an inert line, in `knowledge@documentation/src/source/md.rs` and the
inert branch of `scan` in `knowledge@documentation/src/scan.rs`. A `CR:` marker with a quote, a
slug reference or a path reference written inside `<!-- … -->` in a markdown document is
therefore verified by nothing and reported by nothing. Root `thaum@CLAUDE.md` enumerates the
places a rule number is data and says there is no third form, and names no such place.

**Observed.** By the rules axis of the review of the branch that landed
`knowledge#checker-source-literals-are-data`, on a scratch copy of the `minimal` mock project
under `knowledge@tests/projects/`: a line naming a mock rule behind a marker, with a claim and no
quote, appended to its `README.md` inside an HTML comment added no finding, and the same line
outside one added the expected claim-without-quote finding. Reproduces.

**Why it matters.** It is a place to hide a fabricated claim, of the kind the rules-reviewer is
told to read for, and the instruction that enumerates the data forms is incomplete against the
implementation. Parking a section by commenting it out is ordinary, so the inert reading itself
is wanted; what is undecided is whether a marker may sit in one at all.

**What would answer it.** A decision, recorded in this component's design home: either an inert
line may not name a rule, reported the way the inverse assertion reports a rule number outside
the walk, or the HTML comment is named in root `thaum@CLAUDE.md` as a form that carries no
claim. Re-entry: the next tool-cleanup discussion.

## A location nested inside another anchor's register home is not refused `defect`

**What.** `check::registers` refuses a component inside a location, because nesting one full
register set inside a partial one gives a document two candidate homes. The reverse nesting — a
location whose path sits inside another anchor's register home — is accepted. A location declared
at `<component>/docs/open-issues/nested`, carrying its own issue register, produces a dozen
findings against the OUTER component's issue register instead: the nested location's own
`README.md` and `index.md` are read as malformed entries of the outer register, and its directory
is reported as a group inside a group.

**Observed.** An adversarial review of the register-shape checks, over a copy of a mock project.
Reproduce by adding such a location to any mock project's manifest and running
`cargo knowledge check --only registers`.

**Why it matters.** The findings name the wrong anchor and the wrong register, so a reader repairs
the outer component's directory and the declaration that caused it stays. It is the same one-home
failure the component-inside-a-location refusal exists against, reached from the other side.

**What would close it.** Refuse a location whose path sits inside any anchor's register home, with
the same finding shape the component-inside-a-location case uses.

## Two anchors' register homes may overlap, and the entry count then double-counts `observation`

**What.** Two registers carried by ONE anchor may not share a home: `check::registers` reports it.
Two ANCHORS whose homes overlap are not refused — a location inside a component's register home is
the reachable case, recorded above — and a document under the overlap is then judged once per
instance and counted once per instance. The `registers:` summary line's entry count is the only
visible symptom.

**Why it matters.** The count is what tells a run that judged the entries from a run that judged
none, which is the whole reason it is printed. A count nobody can trust is worse than no count.

**What would close it.** The refusal the entry above asks for closes the reachable case. If a
second one is found, count distinct documents rather than instance-and-document pairs.

## The `design-home-two-shapes` slug names a decision wider than the slug `todo`

**What.** The head now states the two shapes of every HEADING REGISTER's home, and the slug still
says `design-home`. `recording-a-decision` asks for a new slug when a statement widens, at the cost
of renaming every reference. The head is already cited for the goals and tripwires homes inside the
same document, so a reader following the slug reads _design_ about tripwires.

**Why it matters.** A slug is the name a reader navigates by, and one that names a narrower thing
than its head sends them to the wrong place or makes them doubt they arrived.

**What would close it.** Rename it during step B of `thaum@docs/plans/knowledge-tool-overhaul.md`,
which rewrites every pointer in the tree mechanically, so the rename costs one more substitution
rather than a pass of its own. The spec's own disposition row chose "rewritten in place", which is
what this entry disagrees with.

