# Open issues — the knowledge tool

What is outstanding about `knowledge` itself: the checks, the release machinery, the archive.
What is outstanding about the _rules and their readings_ is `thaum@docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

## The interpretation register has no mention form, so naming an entry counts as citing it `question`

**What.** The interpretation walk counts every `R`-number token in a live file as a citation of that
entry, so a document that merely _names_ one — to point at a diff, to say where a reading is filed —
is listed in `thaum@docs/rules/interpretations/index.md` beside the documents whose argument depends
on the reading. Rule numbers have a way to say it and `R` numbers do not: a rule number that is data
goes inside a code span, and the walk does not read one as a citation. No entry is named in this file on purpose, so that the illustration does not
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

**What.** The path scanner reads one line at a time and two character classes, and the link
scanner reads one inline pattern, so these shapes produce neither a reference nor an
unsupported-shape observation, and no finding:

```
`notes/a.md:12`             a colon suffix, the file-and-line idiom
`minimal@notes/a.md,`       trailing punctuation inside the backticks
a span wrapped across a line boundary
a fullwidth at sign in place of the ASCII one
[text][label]               a reference-style link, with its definition elsewhere
[text](<notes/a.md>)        an angle-bracketed target
```

The link shapes surface as a dangling target or an unlinked subdocument in a design README, and
are silent in an ordinary navigation file, per the close-enough clause of
`knowledge#links-are-navigation-rows`. Enumerated by an adversarial review of the
anchored-grammar branch; a grep at that revision found no live instance of any shape, so every
gap is latent.

**Why it matters.** The unsupported-shape lint promises that no pointer class passes
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
