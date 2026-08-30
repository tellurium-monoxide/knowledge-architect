# Open issues — the knowledge tool

What is outstanding about `knowledge` itself: the checks, the release machinery, the archive.
What is outstanding about the _rules and their readings_ is `thaum@docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

## The tool's own fixtures are indistinguishable from real content `observation`

**What.** The checks walk the whole project, including their own source. A rule number, a slug, a
path, a `cr-version` marker or an identifier marker written as a literal in a test is therefore real
content: it is linted, it defines an anchor, it is a reference that must resolve, and it can pin the
file it sits in to a release.

**Observed** seven times while the tool was written, and an eighth at the first release bump —
rule-shaped bytes in the fold tests' fixtures, repaired to neutral text in the change "Test
fixtures stop looking like rule citations" — each caught by a checker rather than by review:
bare rule numbers in fixtures; a `cr-version` fixture that pinned its whole file to a release; an
identifier marker written out in a doc comment; fixture slugs that defined a real anchor and three
dangling references; a concern filename; two relative paths; and a slug in a title fixture.

**Why it matters.** The structural answer exists — `knowledge@tests/projects/` is excluded by the manifest, so
anything there is invisible — but it only covers integration tests. A unit test still lives in a
walked file, and the standing workaround is to interpolate every literal, which makes the tests
measurably harder to read: `format!("`#{SLUG}` — …")` in place of the thing it means. The cost is
paid by every future check module.

**What would answer it.** Three candidates, and the choice is real rather than a bug to fix.
Either unit tests that carry such literals move to `knowledge@tests/projects/`, which costs a file per case
and buys readable fixtures; or a way to mark a span as fixture text, which is a new convention and
a new thing to get wrong; or a per-family exemption in the manifest, exempting only the citation
families over a named path. The third was argued and parked at the linting-scope discussion, and
it carries a conflict to argue before it can win: `knowledge#the-regime-has-no-opt-out` accepts
only files that are leaving the tree, and the walk asserts that an unwalked file may not name a
rule, so the shape has to say what happens to that assertion. `assumption`: the first, because the
exclusion already exists and needs no new concept. Untested — nobody has tried moving one.
Re-entry: the next tool-cleanup discussion.

## The interpretation register has no mention form, so naming an entry counts as citing it `question`

**What.** The interpretation walk counts every `R`-number token in a live file as a citation of that
entry, so a document that merely _names_ one — to point at a diff, to say where a reading is filed —
is listed in `thaum@docs/rules/interpretations/index.md` beside the documents whose argument depends
on the reading. Rule numbers have a way to say it and `R` numbers do not: a rule number that is data
goes inside a code span, a fenced block or a name-bound string literal, and the walk does not read
those as citations. No entry is named in this file on purpose, so that the illustration does not
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

## A slug definition is not recognised in a list item `observation`

**What.** A slug is recognised as a definition at the start of a line or in a table cell. A list
item is neither, so a decision written as `- \`#slug\` — **Statement.**` is read as a _reference_
and fails the dangling check.

**Observed.** Writing `thaum#bench-is-a-tool` into `thaum@docs/design.md`, in the
_Why each part is the way it is_ subsection of _Repository layout_. That subsection is nothing but
consecutive bulleted arguments, which is its whole idiom, and the slug had to be broken out into a
standalone paragraph after the list. The subsection's other arguments carry no slugs, and the one it
cites is defined elsewhere. **Whether that is cause or coincidence is not
established**: nobody has checked whether those arguments lack slugs because the grammar cannot hold
one there, or because none of them was ever cited from anywhere.

**Why it matters.** The checker's grammar is deciding document shape. That is legitimate where the
shape is the point and a problem where it is not, and the two are told apart by whether a reader is
better off — a standalone paragraph after the list reads as an afterthought rather than as its last
argument. The cost is small and one-directional: it pushes decisions out of list-shaped sections, or
leaves them without anchors, and an unanchored decision cannot be cited or found by `git log -G`.

**What would answer it.** Two readings, and the choice is a real one rather than a bug to fix.
Either extend the definition grammar to accept a leading list marker, which costs one alternation
and makes the grammar match the documents; or record that a decision worth a slug is worth its own
paragraph, in which case the current behaviour is a deliberate constraint and belongs in
`thaum@.claude/skills/recording-a-decision/SKILL.md` beside the head instruction rather than being a
property only a pattern states. `assumption`: the second is what was intended, since the table-cell
form was added deliberately and the list form was not. Nothing records either way.

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

## The hand-rolled CLI has accepted invalid argument combinations silently `observation`

**What.** The owner reports that this tool's command-line parsing, hand-rolled in
`knowledge@src/main.rs`, has accepted invalid argument combinations and then done nothing,
with no error and exit 0. Reported from past sessions; whether any such combination still
reproduces on the current parser is `not established` — no failing invocation is on record.

**Why it matters.** A tool that silently does nothing on a malformed invocation reads as a
passing check. Every gate and review in this project trusts this tool's exit code.

**What would close it.** Enumerating the subcommands' argument handling and either reproducing
a silent acceptance (promoting this to `defect` with the invocation) or ruling the class out.
The clap migration in the entry below closes it structurally: clap refuses unknown flags and
invalid combinations by construction, which is the argument recorded at
`xtask#clap-for-parsing`.

## The CLI-taking tools still parse arguments by hand `todo`

**What.** The owner intends every CLI-taking tool in this repository to parse through clap (or
an equivalent crate), as `xtask` now does per `xtask#clap-for-parsing`. `knowledge` and
`mutate` still parse by hand; clients may follow later.

**Why it matters.** Hand-rolled parsing is where the silent-acceptance class above lives, and
each hand-rolled parser re-solves flag handling that a shared crate already solves refused-by-
default.

**What would close it.** Migrating `knowledge` and `mutate` argument handling to clap, with
the refusal of unknown flags and invalid combinations asserted by a test in each, as
`xtask@src/main.rs` does.

## A release the manifest does not know resolves from a cache nothing verifies `observation`

**What.** `resolve` in `knowledge@rules/src/release.rs` answers a release that is neither
vendored nor archived from `std::env::temp_dir()/MagicCompRules-<date>.txt`, downloading only
when that file is absent. When `MANIFEST.tsv` holds no row for the date — which is every bump
target by construction, since the manifest records only superseded releases — the digest check
has nothing to compare against, so whatever bytes sit at that path are returned with only a
stderr warning. Reproduced during the adversarial review of the first bump: with an empty
manifest and the cache pre-seeded with arbitrary bytes, `resolve` returned those bytes as the
release. A `bump` then builds its diff, its effective-as-of report and its `CHANGES.md`
skeleton from them.

**Why it matters.** The temp path is predictable and the directory is world-writable, and a
stale or foreign file there is indistinguishable from a download. The blast radius is bounded:
the pinned text a bump writes is walked by `cargo knowledge check`, so grossly wrong bytes fail
hundreds of quote verifications loudly — but a subtly wrong text that preserves the cited rules
would pin silently. First contact with a new release is inherently unverifiable against a prior
record; trusting a shared cache by existence is the avoidable part.

**What would close it.** Caching under a user-owned directory (or not caching across runs at
all: the file is under 1 MB), plus printing the digest of what was actually used, would reduce
the window to the download itself. Deciding that the residual first-contact trust is accepted,
and saying so in `resolve`'s doc comment, would also close this as a recorded trade-off.
