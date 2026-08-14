# Open issues — the knowledge tool

What is outstanding about `tools/knowledge/` itself: the checks, the release machinery, the archive.
What is outstanding about the _rules and their readings_ is `../../docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

## The plan claimed a parser port that was not made, and three reviewers found it before anyone else did `observation`

**What.** `../../../../docs/plans/citation-enforcement-design.md` stated, as a consequence of
adopting a markdown parser, that two recorded defects were closed by it: an emphasis delimiter a
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
declares the files a formatter must not rewrite. The first is the direction
`../../../../docs/plans/citation-enforcement-design.md` already takes: the markdown parser it adopts
represents both delimiters as one emphasis node, so the distinction disappears. Closing it means the
underscore form verifying, asserted by a test, and the fragment count not moving when a formatter
runs over the tree.

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

## An identifier marker is invisible unless it opens its identifier `defect`

**What.** The identifier form exists because a test name cannot carry punctuation, and it owes a
prose marker with its quote above it. The pattern that finds one opens with a word boundary, so a
marker standing alone after a space or a backtick matches, and the same marker inside a
`test_`-prefixed name does not. (Neither form is written out here: the first would be a live marker
in this file, owing a quote it has no business carrying.)

Verified against the implementation this replaced, which returns no match for the same input, so the
behaviour is reproduced rather than introduced. `documentation/src/scan.rs` pins both directions.

**Why it matters.** The convention's whole purpose is test names, and `test_` is the commonest way
to write one. A test named that way carries a marker that the orphan check cannot see, so the carve
-out to the no-exceptions citation instruction — an identifier marker must have a prose marker with
its quote immediately above it — is unenforced for exactly the shape it was written for. It is a
silent false negative: nothing reports it, and the missing quote is invisible.

**That last clause is no longer true, and the convention it describes cannot be followed in Rust.**
The prescribed identifier form is upper case. A Rust function named that way fails the gate:
`rustc -D warnings` reports that the function *"should have a snake case name"* under
`non_snake_case`, which `cargo clippy --workspace --all-targets -- -D warnings` runs. (The form is
not written out here, for the reason the parenthesis above gives.) So every rule-named test in the
tree uses the lower-case form instead — 56 of them, `grep -rn 'fn cr_[0-9]' --include=*.rs .` — and
the check's pattern is case-sensitive, so the orphan lint is structurally dead for the one thing the
convention exists for. A test named for a rule its comment does not quote is reported by nothing.

Two ways out and they are not equivalent: the pattern accepts the lower-case form, which makes 56
existing names live markers and may surface a backlog; or root `CLAUDE.md` prescribes the lower-case
form, which is what the tree already does and what Rust permits. The second is a configuration
change and is `maintaining-agent-config`'s.

The tree has two uses of the upper-case form and both are in prose, so nothing depends on it yet. The first
Rust test named this way is when it starts costing something.

**What would close it.** Either the pattern widened to find a marker anywhere in an identifier, or
root `CLAUDE.md` stating that the marker must open the name. Both are changes to the convention
rather than to the checker, so this is a decision before it is a fix. Closing it means a test named
`test_CR_…` with no prose marker above it being reported.

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
on the reading. Root `CLAUDE.md` gives rule numbers the distinction: `CR~` marks a number used as a
name rather than as a claim about content, and the register has no equivalent. No entry is named in
this file on purpose, so that the illustration does not create the thing it illustrates.

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

**What would answer it.** Decide whether `R` numbers get a mention form, and if so make the
interpretation walk skip it, the way the rule walk skips `CR~`. The decision belongs with
`recording-an-interpretation`, which owns the entry shape and would have to state the marker.

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

## The unmarked-reference lint exempts a blockquote and not an inline quote `defect`

**What.** `check/citations.rs`'s `lint` skips a line whose first non-space character is `>`, with the
comment _"verbatim rule text, not a reference"_. That exemption is **line-shaped**. The inline quote
form is **span-shaped**, so a rule number inside an inline quotation is scanned as an ordinary
reference and reported as unmarked. The identical rule text quoted as a blockquote is exempt.

**Observed** writing `../../crates/thaum-engine/src/runtime/step.rs`. The rule quoted was CR:800.4n,
whose body ends with a sentence naming CR~800.4a as the rule it makes an exception to. Quoting that
rule in full, inline, put a bare subrule number inside the quotation, and the check reported it as a
rule named with no marker. The quotation was verbatim and verified.

**This entry cannot show its own example**, for the reason the fixtures entry above records: writing
the offending line out here reproduces the finding in this file. The first draft did exactly that and
the check reported it twice.

Measured over the pinned release, filtering to `NR>181`: 3 162 numbered rules in the body, of which
569 contain a token matching the lint's own number shape after their own number. So roughly one rule
in six cannot be quoted in full by the inline form without tripping this, and the count grows with
every cross-reference WotC adds. Re-take from `../../docs/rules/`:

```python
import re
tok = re.compile(r'\b\d{3}\.\d+[a-z]{0,2}\b')
num = re.compile(r'^(\d{3}\.\d+[a-z]{0,2})\.?\s')
body = open('MagicCompRules.txt', encoding='utf-8').read().split('\n')[181:7099]
rules = [(m.group(0), L.strip()[len(m.group(0)):])
         for L in body if (m := num.match(L.strip()))]
print(len(rules), sum(1 for _, rest in rules if tok.search(rest)))
```

The body bound is the corpus layout root `CLAUDE.md` states, and it moves with the release, so
re-take against the pinned text rather than against these numbers.

**Why it matters.** It is a false positive inside a gate, and its shape pushes the writer the wrong
way. Three escapes exist and each costs something: switch to a blockquote, which is not always wanted
inside a doc comment; elide the cross-reference, which is right only when that fragment is genuinely
not needed; or add a marker for a rule the sentence makes no claim about, which is the worst of the
three, because it puts a citation in the tree that no reader asked for and that a release bump will
later treat as a work item. Nothing distinguishes a real unmarked reference from this one, so every
occurrence is judged by hand.

**What would close it.** The quote scanner already computes every quoted span and which rule owns it,
so the information exists: a rule-number token falling **inside a verified quote span** is verbatim
rule text rather than a reference, whichever quote form carries it. Closing this means CR:800.4n
quoted in full by the inline form producing no finding, while a bare rule number in ordinary prose on
the same line still does.

## The nearest-marker rule and the stated convention are not the same rule `observation`

**What.** `quote.rs` attributes a quote to _"The NEAREST marker before the quote"_, and its comment
argues the case it exists for: _"A sentence often cites two rules in sequence, each with its own
marker and its own quote; taking the first or the last marker on the line attributes both quotes to
one of them."_ Root `CLAUDE.md` states the convention differently: _"The marker goes in the clause
that introduces the quote."_

A clause is not a distance. The two agree whenever the introducing clause's marker is also the
closest one, and they diverge when a sentence names a second rule between the introducing marker and
the quote. There the writer has followed the stated instruction and the check reports a mismatch.

**Observed** writing `../../crates/thaum-engine/tests/driver.rs`: one sentence introduced a quote of
CR:800.4n while naming CR~800.4a after it and before the quote, so the nearest preceding marker was
not the introducing one. **The report was useful.** The sentence was ambiguous to a human reader too,
and reordering it improved the prose. That is why this is an observation rather than a defect.

Only the prose form can own a quote, since the marker pattern matches `CR:` alone, so a mention
marker cannot steal one. That was checked against the implementation, because the opposite would have
been a defect rather than a divergence.

**Why it matters.** The failure is legible only to someone who already knows the rule is proximity.
The finding says the text verifies against a different rule, which reads as a renumbering, and its
hint says to retarget the citation. That advice is wrong for this case, where the repair is to move
the other marker, and a writer who follows it retargets a correct citation at the wrong rule.

**What would answer it.** Either root `CLAUDE.md` states the proximity rule, which makes the
convention checkable where a writer reads it and costs one sentence; or the finding's hint gains the
second case, so a mismatch on a line carrying two prose markers suggests reordering before
retargeting. `assumption`: the first, because the convention is what a writer consults and the hint is
what they meet only after already being wrong. Nothing records either way.

## A `CR:` marker that owns no quote is never reported, and its number is never resolved `defect`

**What.** The prose-form marker asserts a claim about a rule's content, and root `CLAUDE.md`
requires it to carry _"a verbatim quote and the exact rule number as printed"_. A `CR:` marker with
**no quote anywhere near it** satisfies neither check that exists: the quote check verifies quotes
against the rule cited and has nothing to verify, and the missing-marker lint looks for the opposite
failure — a reference with no marker.

Nothing resolves the number either. Planted in `crates/thaum-engine/src/runtime/observe.rs` and run:

A prose-form marker naming a three-digit rule that does not exist, with no quote after it, planted
in a doc comment. **The marker is not written out here**, for the reason the entry two above gives
about this file's own fixtures: it would be a live citation in this document and would put its own
row in the generated index. It did — `docs/rules/index.md` carried a _"Not rules in this release"_
row for the planted number until this sentence replaced the planting.

`cargo knowledge check` reported nothing: every quote fragment verified and zero unmarked rule
references, over a doc comment asserting what a rule says, behind a marker, about a number the
corpus does not hold.

**Why it matters.** It is the failure mode root `CLAUDE.md` names as the dangerous one, reached from
the other side: _"a bare rule number with no marker at all is the one thing that discharges nothing,
because the checker cannot see it."_ A marked number with no quote discharges nothing either, and it
looks conformant to a reader **and** to the checker, which the bare number does not.

**Observed twice in one session**, writing `crates/thaum-engine/src/runtime/observe.rs`. One was
`CR:120.3c's attempt on an empty library`, which is planeswalker damage — the rule wanted was
CR:704.5b. Both were caught by re-reading the citations by hand, not by the gate.

**A third occurrence, in prose rather than in a doc comment.** Writing
`docs/plans/slice-2-design.md`, 886 lines against the pinned release: an audit by hand of every
prose marker in that file found **eighteen** carrying no quote in range, each a claim about a rule's
content whose quote sat in a different section or a different document. `cargo knowledge check`
passed on the file before the audit and passed after it, so the whole repair was invisible to the
gate. That the count is high is a property of a long document written in one pass, and the figure is
over that file at that revision rather than a rate to expect elsewhere. What it adds to the two
observations above is that the failure is not confined to code comments, and that a session writing
a document of this size cannot rely on the gate to find it.

**Why it is not the entry above it.** _A verbatim rule quote can sit unchecked if nothing marks it_
is the inverse case, a quote with no marker, and its argument turns on the false-positive ratio of
scanning plain quoted spans. This one needs no scanning: the marker is already found, and what is
missing is a check that something follows it.

**What would close it.** Two halves, separable. Resolving every cited number against the corpus is
mechanical and has no false-positive class. Requiring a quote after a prose marker needs a decision
about the discharge rule — root `CLAUDE.md` says a pointer to a slug that already carries the quote
discharges the obligation, so a marker whose quote lives elsewhere is conformant and would have to be
distinguished. `assumption`: the number-resolution half can land alone. Nobody has checked whether
any conformant citation names a rule the corpus does not hold.

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
| *Whether a `State` pins a card-corpus version*, the engine's `open-issues.md` | **`thaum-corpus`'s** `open-issues.md` | yes |
| *`Status::BudgetExhausted` conflates two different bounds*, the engine's `open-issues.md` | the file it is written in | yes |
| the new pre-game departure refusal, `runtime/state.rs` | the engine's | fixed on sight |
| `Side::life`'s doc comment, `runtime/instance.rs` | the engine's | fixed |
| the two deleted acting-player helpers, `runtime/step.rs` | the engine's | gone with the functions |
| `record_mulligan_round`'s doc comment, `runtime/step.rs` | **the project's**, correctly | not an instance |

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

## A float literal shaped like a rule number is reported as an unmarked reference `defect`

**What.** The missing-marker lint matches a bare rule token as `\b(\d{3}\.\d+[a-z]{0,2})\b`, which
a numeric literal satisfies. Multiplying a ratio by one hundred, written as a float literal in
`../../crates/thaum-ai/tests/answerers.rs`, was reported as an unmarked reference to a rule with
that number.

**This entry could not be written using the literal that causes it**, which is the clearest
statement of the cost: the text above says "one hundred as a float literal" because spelling it
produced two more findings against this file.

**Why it matters.** The repair available to an author is to spell the number differently — the line
now carries a Rust numeric suffix instead — which is a source change made to satisfy a checker
rather than a reader. The cost is small per instance and paid in the wrong place: percentages,
tolerances and any three-digit constant with a decimal are ordinary in a project that records
measurements, and a lint firing on them teaches the author to route around it.

**Why it is not simply exempted.** The same pattern is what catches a genuine unmarked citation, and
that lint is load-bearing: root `CLAUDE.md` names a bare rule number as _"the one thing that
discharges nothing, because the checker cannot see it"_. An exemption keyed on "inside a `.rs` file"
would blind it exactly where rule citations sit in code comments.

**What would close it.** A decision on how to tell the two apart. The candidate that costs nothing
elsewhere is to skip a token in arithmetic or with a numeric suffix — a preceding `*`, `+` or `=`
and no `CR` marker on the line. `assumption`: no genuine citation is ever written adjacent to an
arithmetic operator. Nobody has checked that against the tree.
