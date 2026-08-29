# Open issues — the knowledge tool

What is outstanding about `tools/knowledge/` itself: the checks, the release machinery, the archive.
What is outstanding about the _rules and their readings_ is `../../docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

## The plan claimed a parser port that was not made, and three reviewers found it before anyone else did `observation`

**What.** The design that adopted a markdown parser stated, as a consequence of adopting it, that
two recorded defects were closed by it: an emphasis delimiter a
formatter rewrites, and a backticked reference a formatter wraps across a line. Neither was closed.
The parser was wired into scanning and into scoping; quote EXTRACTION was left on the hand-written
byte scanners, which is where both defects live. The plan is corrected, and the port is now made.

**Why this is recorded rather than only corrected.** A plan document is amended in place and leaves
the repository when its work lands, so correcting it erases the fact that the claim was made. What
is worth keeping is not the wrong sentence but its shape: **a consequence was written into a design
document as though it had been implemented, in the same change that implemented the thing it was a
consequence of.** Nothing distinguished it from the clauses that were true, and the gate could not:
the tool passed throughout, because a quote that stops being found is subtracted from the numerator
and the denominator together.

**Observed.** Three independent reviewers found it — a spec-conformity axis, an adversarial evasion
axis and a blind one — each reproducing it separately. No checker did, and neither did the author.

**Why it matters.** The same shape is available to every design document in this project: a head
that states what a mechanism achieves, written by the session that built the mechanism, verified by
nobody. Root `CLAUDE.md` `Verify a claim before writing it` binds it, and the failure was not that
the instruction is missing but that it was not applied to a sentence about the author's own work.

**What would close it.** Nothing here is outstanding as work — the port is made and the plan is
corrected. It stays as an `observation` because the question it raises is open: whether a design
head asserting what a change achieves should be reviewed against the change by a separate axis by
default, rather than only when someone dispatches one. `assumption`: the reviewers caught it because
the brief named the spec as the standard and told them to establish the tree themselves. Untested —
it has happened once.

## The tool's own fixtures are indistinguishable from real content `observation`

**What.** The checks walk the whole project, including their own source. A rule number, a slug, a
path, a `cr-version` marker or an identifier marker written as a literal in a test is therefore real
content: it is linted, it defines an anchor, it is a reference that must resolve, and it can pin the
file it sits in to a release.

**Observed** seven times while the tool was written, each caught by a checker rather than by review:
bare rule numbers in fixtures; a `cr-version` fixture that pinned its whole file to a release; an
identifier marker written out in a doc comment; fixture slugs that defined a real anchor and three
dangling references; a concern filename; two relative paths; and a slug in a title fixture.

**Why it matters.** The structural answer exists — `tests/projects/` is excluded by the manifest, so
anything there is invisible — but it only covers integration tests. A unit test still lives in a
walked file, and the standing workaround is to interpolate every literal, which makes the tests
measurably harder to read: `format!("`#{SLUG}` — …")` in place of the thing it means. The cost is
paid by every future check module.

**What would answer it.** Two readings, and the choice is real rather than a bug to fix. Either
unit tests that carry such literals move to `tests/projects/`, which costs a file per case and buys
readable fixtures; or a way to mark a span as fixture text, which is a new convention and a new
thing to get wrong. `assumption`: the first, because the exclusion already exists and needs no new
concept. Untested — nobody has tried moving one.

## The interpretation register has no mention form, so naming an entry counts as citing it `question`

**What.** The interpretation walk counts every `R`-number token in a live file as a citation of that
entry, so a document that merely _names_ one — to point at a diff, to say where a reading is filed —
is listed in `docs/rules/interpretations/index.md` beside the documents whose argument depends
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

## A slug definition is not recognised in a list item `observation`

**What.** A slug is recognised as a definition at the start of a line or in a table cell. A list
item is neither, so a decision written as `- \`#slug\` — **Statement.**` is read as a _reference_
and fails the dangling check.

**Observed.** Writing `thaum#bench-is-a-tool` into `docs/design.md`, in the
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
`.claude/skills/recording-a-decision/SKILL.md` beside the head instruction rather than being a
property only a pattern states. `assumption`: the second is what was intended, since the table-cell
form was added deliberately and the list form was not. Nothing records either way.

## A Component-relative document name resolves against the project root and is never reported `defect`

**What.** Three path syntaxes are declared: a full path from the project root, `<component>@path`
relative to a Component's root, and `@path` for a reference that is not checked. A reference written
as the first when the second was meant still resolves, because **every Component is required to
carry the same six document names** — `README.md`, `CLAUDE.md` and four under `docs/` — and the
project is itself a Component carrying them too. So `docs/open-issues.md` written inside
`crates/thaum-engine/` names the project's file, the file exists, and the path check passes.

**How to enumerate them**, since a hand-written list of sites went stale within one review:

```sh
grep -rn '`docs/\(open-issues\|design\|tripwires\|rejected-alternatives\)\.md`' \
  crates clients tools --include=*.rs --include=*.md | grep -v '@docs/'
```

Every hit is a candidate and **not** every hit is an instance: a bare root-relative reference from
inside a Component is legitimate where the project's file really is meant. Classifying requires
reading each one.

**In `crates/thaum-engine/` alone, twelve candidates and nine outstanding instances**, counted this
way rather than by recall:

| site | means | outstanding |
| --- | --- | --- |
| `Roster`'s doc comment, `runtime/state.rs` | the engine's | yes |
| `Status`'s doc comment, `runtime/step.rs` | the engine's | yes |
| `advance`'s doc comment, `runtime/step.rs` | the engine's `tripwires.md` | yes |
| `the_game_and_the_free_functions_play_the_same_game`, `tests/log.rs` | the engine's `tripwires.md` | yes |
| `a_copied_game_carries_its_history_and_is_independent`, `tests/log.rs` | the engine's `tripwires.md` | yes |
| `cr_402_2_a_hand_never_exceeds_the_maximum_after_a_cleanup_step`, `tests/pregame.rs` | an entry on the absent discard, which exists in no tracker in the tree | yes |
| *Whether a `State` pins a card-corpus version*, the engine's `open-issues.md` | **`thaum-corpus`'s** `open-issues.md` | gone with the entry — closed by the corpus design |
| *`Status::BudgetExhausted` conflates two different bounds*, the engine's `open-issues.md` | the file it was written in | gone with the entry — closed by the status split |
| the new pre-game departure refusal, `runtime/state/boundary.rs` | the engine's | fixed on sight |
| `Side::life`'s doc comment, `runtime/instance.rs` | the engine's | fixed |
| the two deleted acting-player helpers, `runtime/step.rs` | the engine's | gone with the functions |
| `record_mulligan_round`'s doc comment, `runtime/step/pregame.rs` | **the project's**, correctly | not an instance |

**The seventh row breaks the mechanism stated above.** It points from one Component into a
*different* Component, so the collision is not only "the project carries the same names" — a bare
path from inside any Component names the project's file whatever Component was meant. `assumption`:
other Components carry instances too; only `crates/thaum-engine/` has been counted.

**Why it matters.** The reader is sent to a document that does not carry what the sentence names,
and nothing reports it. The project's `docs/open-issues.md` holds two entries, on a
scenario-interchange format and on design-review artifacts; none of the sentences above means
either, and one of them names an entry that exists nowhere at all. This is silent in exactly the
case the two syntaxes exist to distinguish, and the collision is universal rather than accidental,
because the required document set is what creates it.

**What is ruled out.** Forbidding the shape. A bare root-relative reference from inside a Component
is legitimate and is in use: `tools/knowledge/docs/design.md` says *"What belongs here rather than in
`docs/design.md`"* meaning the project's, `tools/knowledge/docs/open-issues.md` names the same file
the same way, and the last row of the table above is a third. So the check cannot key on the shape
alone.

**What would close it.** A check that reports a root-relative reference whose tail is one of the
Component-required document names, made from a file inside a Component, **and** a way for a writer
to say they meant the project's — the `@` form already exists and is not checked, so the
disambiguation may already be spelled. Closing this means every outstanding row above being
reported and the three legitimate root references still passing. Whether it is a finding or a lint
is open.

## The path scanner's suffix set omits `.rs`, so Rust paths in documents are never checked `defect`

**What.** `PATH_REF` in `tools/knowledge/documentation/src/scan.rs` extracts a backticked
root-stem path only when its suffix is one of `md|txt|py|sh|tsv`. A backticked `.rs` path in
any document is not extracted, so the paths check in
`tools/knowledge/documentation/src/check/paths.rs` never validates it and it can dangle
silently. Root `CLAUDE.md` states that backticked paths are checked to point at existing
locations, with no suffix carve-out, so the instruction and the tool disagree. The
component-relative form (`COMPONENT_PATH_REF`) carries no suffix restriction and is unaffected.

**Reproduce.** Append a backticked reference to a nonexistent path ending in `.rs` — for
example a `bogus-probe.rs` under any real directory — to a walked document, then run
`cargo knowledge check --only paths`: the reference count does not change and the check
passes. The same reference with a `.md` suffix is reported as dangling.

**Why it matters.** The relocation of the nine game-driving suites to `tools/thaum-testing`
moved every `.rs` test path named across five knowledge documents, and the checker reported
none of them; the sweep had to be grep-driven. Any rename of a Rust file leaves silent
dangling pointers in the documents, which is the failure the paths check exists to prevent —
its own module doc records nine dangling references to a deleted file as its founding case.

**What would close it.** Adding `rs` to `PATH_REF`'s suffix set, then repairing whatever the
widened check reports over the live documents. `assumption`, not measured: the suffix set
exists to keep prose with slashes from reading as paths, and widening by one code suffix
keeps that property because the pattern still requires backticks, a slash and a filename
shape.
