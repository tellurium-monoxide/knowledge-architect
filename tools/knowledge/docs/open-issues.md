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

## A markdown formatter's emphasis normalisation makes an inline rule quote invisible `defect`

**What.** The inline-quote convention is the asterisk form, and `quote.rs` finds one with
`italic_spans`, which requires the two characters `*"` to open the span. A markdown formatter that
normalises emphasis to underscores rewrites the span so it opens with `_"` instead. That form is
found by neither scanner: `italic_spans` requires the asterisk, and `plain_spans` rejects a double
quote whose neighbour is a `*`, an alphanumeric or a `_`. The quote is then not wrong — it is
**absent**, and nothing verifies it.

**Reproduced against this tree, on the branch that reworks the citation checks.** A formatter ran
over `../../crates/thaum-engine/docs/design.md` and converted 87 asterisk-delimited spans to
underscore-delimited ones. Counted with `grep -o` before and after. The signature of the same run is
visible in two other places in that file: markdown tables padded to aligned column widths, and five
lines where a literal asterisk was escaped as a backslash pair.

Measured with `cargo knowledge check --only citations`, which prints the fragment count on its first
line, over a worktree at the merge base and over the branch tip:

| tree | fragments verified |
| --- | --- |
| merge base | 661 |
| branch, after the formatter ran | 572 |
| branch, after restoring the asterisk form | 666 |

So **89 rule quotes stopped being checked** and every run in between reported `PASSED: no findings`.
The count rose past the merge base on repair because three spans were already in the underscore form
at the merge base.

**Why it matters.** The guarantee this tool exists to make is that every rule quote in a live
document verifies against the pinned release. A routine editor action removes quotes from that
guarantee with no diagnostic, and the diff that does it looks like whitespace and emphasis. It is
the exact failure the tool is built to prevent, arriving through the one path nobody inspects.

`assumption`: the formatter is Prettier, which normalises emphasis to underscores, pads tables and
escapes stray asterisks. Not established — no formatter is configured in this repository, and no
configuration file for one exists in it, so the run came from an editor rather than from the tree.

**What would close it.** Either the scanner accepts both emphasis delimiters, or the repository
declares the files a formatter must not rewrite. The first is the direction already taken: the
markdown parser represents both delimiters as one emphasis node, so the distinction disappears in
`quote::italic_spans`. Closing it means the underscore form verifying, asserted by a test, and the
fragment count not moving when a formatter runs over the tree.

## The citation report is a ratio, so a quote that stops being found reads as success `defect`

**What.** `check::citations` counts what it finds and reports `verified / fragments`. A quote the
scanner cannot see contributes to neither, so the ratio stays at 100% and the run passes. There is
no expected count and nothing compares one run against another, so **losing a quote is
indistinguishable from never having written one**.

**Observed** as the reason the entry above went unnoticed. Three separate full runs of
`cargo knowledge check` reported `572/572 rule-quote fragments verified` and `PASSED: no findings`
while 89 quotes were absent from the walk. The loss was found by comparing against a worktree at the
merge base, by hand, for an unrelated reason.

**Why it matters.** Every other family in this tool reports an absolute a reader can judge — the
number of components, of slugs, of path references, of register entries. The citation family reports
only a proportion of itself, which is the one family where absence is the dangerous direction. A
reviewer reading the summary cannot tell a tree with no quotes from a tree whose quotes all verify.

**What is ruled out.** Storing an expected count in the repository. It would go stale on every commit
that adds or removes a citation, which is most of them, and a threshold nobody can maintain is one
that gets raised until it means nothing.

**What would close it.** Two candidates, neither tried. Report the absolute alongside the ratio, so a
fall is visible in a diff of CI output — cheap, and it only helps a reader who compares runs. Or
report quoted spans the scanner found but could not bind to a rule, which turns the mangled form
above into a finding rather than a silence. `assumption`: the second is the one that would have
caught this, since a `_"…"_` span is still a quoted span; nobody has measured its false-positive rate
against ordinary quoted prose.

## `cargo fmt` can produce a line the blockquote instruction reads as a quote `defect`

**What.** A blockquote is recognised by `>` at the start of a line, which is Markdown's rule applied
to source files as well, because a rule quote inside a code comment is a citation. In Rust, `>`
closes a generic — and when a signature is long enough, the formatter breaks the return type so that
a line begins with `> {`. The check then reports a blockquote holding commentary rather than rule
text.

Reproduced against this tree: a `fn` returning a two-element tuple inside a `Result` formatted to a
line reading exactly `> {`, and both the current checker and the implementation it replaced reported
the same finding on the same line. It is not a defect introduced by the port.

**Why it matters.** The formatter and the checker disagree about one character, and the formatter
wins — `cargo fmt --all --check` is a gate, so the shape cannot simply be avoided by hand. The
workaround is a type alias, which is usually better code, but that is a coincidence rather than a
reason. It is a false positive inside the guarantee the citation check makes, and every long
signature is a new chance to hit it.

**What would close it.** A blockquote test that cannot fire on code. The suffix is already known
where the leaders are stripped, so the check has the information: in a file with comment syntax, a
`>` that survives leader-stripping is code rather than a quote, because a real doc-comment quote
arrives as `/// >` and loses only the leader. Closing this means the reproduction above producing no
finding while a quote written in a doc comment still verifies — the case
`documentation/src/walk.rs` already tests.

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

## The mismatch finding's hint sends a writer at the wrong repair `defect`

**What.** An inline quote binds to the nearest marker before it. Where a sentence names a second
rule between the introducing marker and the quote, the quote is bound to the rule the writer did
not mean, and `check::citations` reports *"the text verifies, but not as X"* with the hint *"a
renumbering, or the wrong number — retarget the citation at the rule that now holds this text"*.
That advice is wrong here: the citation is correct and the repair is to move the other marker. A
writer who follows the hint retargets a correct citation onto the wrong rule.

**What has been closed, and is no longer part of this entry.** The silent half — a quote bound to
the wrong rule whose text that rule also holds, verifying with nothing reported — is now
`regime::Rule::QuoteBindingIsAmbiguous`, and root `CLAUDE.md` states the proximity rule where a
writer reads it. Two real instances were found in the tree when the rule landed:
`crates/thaum-engine/src/runtime/instance.rs` in the doc comment for `empty_draw_attempts`, and
`crates/thaum-engine/tests/driver.rs` above the two-player departure test. Both are sentences that
deliberately name two rules sharing a sentence, so the citation is right and only the binding is
unfalsifiable.

**Why it matters.** The hint is the only thing a writer meets after already being wrong, and it is
confident and specific in the wrong direction. Retargeting produces a citation that verifies, so
nothing downstream reports it, and the generated rule index then carries a rule the document does
not depend on.

**What would close it.** The hint gains the second case: on a line carrying two or more prose
markers, suggest moving the intervening marker before suggesting a retarget. `assumption`: the
mismatch verdict cannot distinguish the two causes, so the hint has to name both rather than
choose. Nothing has tested whether a writer reads past the first clause of a hint.

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
| *`Status::BudgetExhausted` conflates two different bounds*, the engine's `open-issues.md` | the file it is written in | yes |
| the new pre-game departure refusal, `runtime/state.rs` | the engine's | fixed on sight |
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

## A line naming two concern files cannot cite an interpretation from either `defect`

**What.** The reference check in `check/interpretations.rs` compares **every** `R` number on a line
against **every** concern file the same line names, so a line that names two concern files and cites
one entry from each reports two findings and cannot be written at all. Both are false: each number
does name the file it belongs to.

**Reproduce.** In any markdown file the register check reads, put both of the following on **one**
line, in either order: a reference to `docs/rules/interpretations/game-loop.md` R29, and
a reference to `docs/rules/interpretations/object-identity.md` R30.
`cargo knowledge check` then reports *R29 is in game-loop.md, not the object-identity.md this line
names* and the mirror of it.

**They are on separate lines here deliberately**, because writing the reproduction as one line makes
this file fail the gate — which is the defect demonstrating itself and is why the entry cannot show
it directly. Met while writing the 2c-ii row of
`docs/plans/progress.md`, which records both readings that step landed; the row now names one file
and describes the other in prose, which is a worse reference than the one the check refused.

**Why it matters.** A table row is one line, so any row recording work that touched two concerns is
affected — `docs/plans/progress.md` is written entirely in such rows and is where a step's readings
are listed. The workaround costs exactly what the check exists to buy: the comment beside it says a
re-filing must rewrite every file that names the old concern, and a reference reduced to prose is one
a re-filing cannot find.

**Why the check is shaped that way.** Binding a number to a file needs a rule for which of several
named files is the one, and comparing against all of them is the approximation that needs no rule. It
is right whenever a line names one concern, which is every line in the tree until this one.

**What would close it.** Bind each `R` to the nearest concern file named before it on the line, and
compare only against that; a number with no file before it is unqualified and checked against none,
which is the shape a bare `R29` already has. `assumption`, not measured: the nearest-preceding rule
matches how every existing line reads, since each names its file immediately before its numbers.
