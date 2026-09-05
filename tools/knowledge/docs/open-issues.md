# Open issues — the knowledge tool

What is outstanding about `knowledge` itself: the checks, the release machinery, the archive.
What is outstanding about the _rules and their readings_ is `thaum@docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

## The inverse assertion reads the prose marker form alone `defect`

**What.** `first_rule_number` in `knowledge@documentation/src/check/uncovered.rs` finds a dotted
rule number by splitting on characters that are neither alphanumeric nor a dot, and a section
number by a word-bounded regex over the keyword forms. The identifier form, which root
`thaum@CLAUDE.md` accepts wherever a name cannot hold punctuation, uses underscores, so neither
finder sees it. A file outside the walk may therefore carry `cr_613_8c` in a job name, a fixture
name or a dotfile, and nothing reports it.

**Reproduce.** In `knowledge@tests/projects/dirhome/`, which is clean, write a
workflow file under `path@thaum@.github/workflows/` holding a line `run: cargo test cr_100_1` and run
`cargo knowledge check --only uncovered`: `PASSED: no findings`. Replace the token with the
prose form behind its marker and the same run reports the line. Found by an adversarial review
of the mock-project piece.

**Why it matters.** The module's own head states the assertion without qualifying the form:
outside the walk, a rule number is forbidden. One of the two accepted forms is exempt in fact
and not in any statement, and the founding case for the family was a workflow file — exactly
where a test name is written.

**What would close it.** Reading the identifier form in the same finder, with the same
prose-form message, and a planted case in `knowledge@tests/projects/planted/` beside the
existing one. Deciding instead that an unwalked file may carry the identifier form, and saying
so in the module head and in root `thaum@CLAUDE.md`, closes it as a recorded exemption.

## The changelog's own counts are asserted by nothing `defect`

**What.** `check::changes` verifies a section's summary-table row, its four `meta` keys, its
source url, its recorded digest and every blockquote against the release the section pins. It
compares no count: neither the `edited`, `renumbered` and `gone` values in the `meta` block nor
the same three columns in the summary table are checked against the entries the section carries.

**Reproduce.** In `knowledge@tests/projects/dirhome/`, whose one section legitimately carries no
entry, edit its `CHANGES.md` so the table row reads `| 3 | 2 | 1 |` and the `meta` block says
`edited: 3`, `renumbered: 2`, `gone: 1`. `cargo knowledge check --only changes` prints
`changelog: 0 rule change(s)` and `PASSED: no findings`. Found by an adversarial review of the
mock-project piece.

**Why it matters.** `bumping-rules` reads the changelog as its work list, and the counts are the
first thing a reader takes off it. A section claiming six changes above zero entries sends
whoever reads it looking for triage that was never owed, or hides triage that was.

**What would close it.** Comparing each of the three counts against the entries the section
holds, classified by the verb its title opens with, and reporting a mismatch as a finding naming
both numbers. The table row and the `meta` block are two statements of one fact, so the check
covers both or says which one is authoritative.

## A `PLANTED` row may name a test that does not exist `observation`

**What.** Two rows of `PLANTED` in `knowledge@tests/mock_projects.rs` carry
`Planted::ByTheBinary(<test name>)`, because `changes` and `corpus` are not in `check::run`. The
name is a string used inside an assertion message and nothing resolves it, so deleting or
renaming the binary test it points at leaves both test files green.

**Reproduce.** Rename `the_changelog_and_the_archive_each_carry_a_planted_defect` in
`knowledge@tests/binary.rs` and empty its body. `cargo test -p knowledge` stays green. Found by
the spec-conformity review of the mock-project piece.

**Why it matters.** It is the same shape as the entry this piece closed, moved one step: the
partition test now refuses a family with no row, and a row can still name an assertion nobody
makes. The two families would go back to being unchecked against a real project with no test
saying so.

**What would close it.** Reaching the two families from `knowledge@tests/mock_projects.rs`, which
means a callable that runs them over a stated tree the way `check::run` runs the other six; or a
test that resolves the named test, which Rust offers no direct way to do.

## `cargo knowledge issues` prints nothing over this repository until the migration lands `defect`

**What.** `outstanding` is gone and `issues` reads the issue register, which is a directory of one
file per entry. This repository still keeps its trackers as `*@docs/open-issues.md`, one file of
level-two headings, which the register reports as its retired file shape. So `cargo knowledge
issues` finds no instance with entries and prints an empty table at exit 1. `tripwires` prints only
the entries that have been given slugs, four in this tool's own file, and its `guarding` column
is `-` for each, because a tripwire heading names the decision it guards in the retired
`<component>#<slug>` form and the column reads `design@<anchor>@<id>` references alone.

**Why it matters.** Root `CLAUDE.md` sends every session to that command before diagnosing
anything, and an empty listing reads as a repository with nothing recorded. That is the exact
failure the command exists against. Three invocation sites now say so in place — root
`CLAUDE.md`, `tracking-open-issues` and the standing-state reviewer — and a session that reads
none of them still gets a wrong answer from the tool itself.

**What would close it.** Step B of `thaum@docs/plans/knowledge-tool-overhaul.md`, which splits every
tracker file into entries and gives every tripwire heading a slug. Nothing else is owed: the
command is right and the tree is not migrated yet.

---

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


## A file the walk cannot read is reported by the citation family alone `defect`

**What.** `Model::build` keeps a file it cannot read as an empty document carrying the reason, and
`check::citations` is the only family that prints it. So `cargo knowledge check --only structure`,
and every single-family run that is not `citations`, is silent about a document whose every
citation, reference and slug left the model.

**Reproduce.** Copy `knowledge@tests/projects/minimal/`, `git init` and `git add -A` in the copy,
delete `knowledge@tests/projects/minimal/notes/b.md` from the copy, then run
`check --only citations` and `check --only structure` in it. The first names the file; the second
prints findings and does not.

**Why it matters.** `knowledge#a-failed-parse-is-loud` promises that a file the walk cannot read
is a finding naming the file, with no family named. A reviewer running one family reads a clean
verdict over a document nothing read. The full run does report it, so the gate is not blind; a
narrower run is.

**What would close it.** The trouble report moving out of `citations` to a place every run
performs — the summary block, or a family that always runs — so that no selection can hide it. A
test that asserts the finding under a selection that excludes `citations`.

## A submodule's and a symlinked directory's contents are read by nothing `defect`

**What.** The walk is git's listing, and `git ls-files` reports a submodule as one gitlink entry
and a symlinked directory as one symlink entry, descending into neither. Both have no suffix the
walk covers, so both are dropped and named by no finding. Every document inside is outside the
walk AND outside the inverse assertion of `uncovered`.

**Reproduce.** Add a submodule holding a `.md` file that cites a rule the release does not hold,
and a symlink to a directory holding another. `cargo knowledge check` reports neither. The tree
walk this replaced reported both.

**Why it matters.** A component vendored as a submodule would be conformant by vacuum, which is
the binding `no-silent-gap` criterion of the overhaul plan. It is bounded today because this
repository holds no submodule and no symlinked directory, so nothing is currently unread.

**What would close it.** A finding naming every gitlink and every symlink entry in the listing,
so a project that grows one is told rather than silently narrowed. Deciding, in
`knowledge#git-supplies-the-walk`, whether a submodule's own listing should be walked as a
project of its own instead.

## A path with a newline in it breaks the one-finding-per-line output `observation`

**What.** A finding is printed as one line opening with its path. A filename holding a newline is
printed raw, so the finding spans two lines and the first is a truncated path.

**Reproduce.** Commit a `.md` file whose name holds a newline and a rule number with no quote,
then read `cargo knowledge check`.

**Why it matters.** Nothing parses this output today, so it costs a reader one confusing line.
It stops being cosmetic the moment anything reads the output by line, which a hook or a CI
annotation would.

**What would close it.** Escaping a path in `Finding`'s display, or refusing such a name in the
walk with a finding of its own.

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
rather than a pass of its own. The plan's record of the heads, in its section 8, lists this one
among those rewritten in place with the slug kept, which is what this entry disagrees with.


## `check` and `commits` disagree about a symlinked file, and the summary claims they agree `observation`

**What.** `check` reads a file's bytes through the filesystem, so a symlink at a walked path
yields the bytes of what it points at. `commits` reads the tree blob, whose content for a
symlink entry is the target's path as text. The two therefore build different models of the same
commit whenever a walked `.md` or `.rs` path is a symlink. `commits`' summary line says a
skipped or failing commit's tree "fails n finding(s), which `cargo knowledge check` reports",
and for such a tree `check` reports a different number.

**Reproduce.** In a project with a clean tree, add a symlink whose name ends in `.md` beside a
document that carries a dangling reference, pointing at it; stage both and commit.
`cargo knowledge check` reports the reference twice, once per path;
`cargo knowledge commits <base>..HEAD` reports it once.

**Why it matters.** Small today and only in one direction: `commits` reads less, so a document
reachable only through a symlink is judged by the range check and not by its own bytes. The
misleading half is the summary line, which sends a reader to a command that answers differently.
The same class as the submodule-and-symlinked-directory entry above, which is where the walk's
own answer is recorded; this entry is about the two readers disagreeing rather than about the
walk.

**What would close it.** Either the summary line stops attributing its count to `check`, or the
per-commit read resolves a symlink entry the way the filesystem does. The second is what makes
the two models one, and it needs a decision about whether a symlink out of the tree is followed
at all.

## Judging a message costs time quadratic in its line count `observation`

**What.** The message analysis is quadratic in the number of lines. Measured on a debug build of
this tool, one reference per line: 1 000 lines in 0.08 s, 5 000 in 0.94 s, 10 000 in 3.35 s,
20 000 in 13 s, and 200 000 lines (4.7 MB) not finished after 120 s. Re-take with
`cargo knowledge commit-message <file>` over a generated file of the wanted size.

**Why it matters.** The hook sits in front of every commit, and `git commit -F` accepts a
generated file. A message of a few hundred lines — which is what this project writes — costs
nothing measurable, so this is a hazard rather than present pain.

**What would close it.** Locating the quadratic term and removing it, or a stated cap on the
message size the command will read. The scanner's own line-offset lookup was already made
logarithmic for documents, so the term is likely in the same shape somewhere the message path
reaches differently.
