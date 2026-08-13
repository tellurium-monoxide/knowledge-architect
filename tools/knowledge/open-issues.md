# Open issues — the knowledge tool

What is outstanding about `tools/knowledge/` itself: the checks, the release machinery, the archive.
What is outstanding about the *rules and their readings* is `../../docs/rules/`.

**Read this file before concluding that a checker behaviour is a new problem.** Every entry states
its **kind** as a tag on its title and carries **What**, **Why it matters**, and **What would close
it** (or, for `deferred`, the **trigger**). **An entry leaves this file when it closes.**

Read `tracking-open-issues` before adding.

---

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

The tree has two uses of the form and both are in prose, so nothing depends on it yet. The first
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
entry, so a document that merely *names* one — to point at a diff, to say where a reading is filed —
is listed in `../../docs/rules/interpretations/index.md` beside the documents whose argument depends
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
item is neither, so a decision written as `- \`#slug\` — **Statement.**` is read as a *reference*
and fails the dangling check.

**Observed.** Writing `#bench-is-a-tool` into `../../docs/design/architecture.md`, in the
*Why each part is the way it is* subsection of *Repository layout*. That subsection is nothing but
consecutive bulleted arguments, which is its whole idiom, and the slug had to be broken out into a
standalone paragraph after the list. The subsection's other arguments carry no slugs, and the one it
cites  is defined elsewhere. **Whether that is cause or coincidence is not
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
`../../.claude/skills/recording-a-decision/SKILL.md` beside the head instruction rather than being a
property only a pattern states. `assumption`: the second is what was intended, since the table-cell
form was added deliberately and the list form was not. Nothing records either way.

## The unmarked-reference lint exempts a blockquote and not an inline quote `defect`

**What.** `check/citations.rs`'s `lint` skips a line whose first non-space character is `>`, with the
comment *"verbatim rule text, not a reference"*. That exemption is **line-shaped**. The inline quote
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

**What.** `quote.rs` attributes a quote to *"The NEAREST marker before the quote"*, and its comment
argues the case it exists for: *"A sentence often cites two rules in sequence, each with its own
marker and its own quote; taking the first or the last marker on the line attributes both quotes to
one of them."* Root `CLAUDE.md` states the convention differently: *"The marker goes in the clause
that introduces the quote."*

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
requires it to carry *"a verbatim quote and the exact rule number as printed"*. A `CR:` marker with
**no quote anywhere near it** satisfies neither check that exists: the quote check verifies quotes
against the rule cited and has nothing to verify, and the missing-marker lint looks for the opposite
failure — a reference with no marker.

Nothing resolves the number either. Planted in `crates/thaum-engine/src/runtime/observe.rs` and run:

```rust
/// CR:999.9 says the chooser is determined by a coin flip.
```

`cargo knowledge check` reported nothing, with `477/477 rule-quote fragments verified` and
`0 unmarked rule reference(s)`. CR~999.9 is not in the corpus.

**Why it matters.** It is the failure mode root `CLAUDE.md` names as the dangerous one, reached from
the other side: *"a bare rule number with no marker at all is the one thing that discharges nothing,
because the checker cannot see it."* A marked number with no quote discharges nothing either, and it
looks conformant to a reader **and** to the checker, which the bare number does not.

**Observed twice in one session**, writing `crates/thaum-engine/src/runtime/observe.rs`. One was
`CR:120.3c's attempt on an empty library`, which is planeswalker damage — the rule wanted was
CR:704.5b. Both were caught by re-reading the citations by hand, not by the gate.

**Why it is not the entry above it.** *A verbatim rule quote can sit unchecked if nothing marks it*
is the inverse case, a quote with no marker, and its argument turns on the false-positive ratio of
scanning plain quoted spans. This one needs no scanning: the marker is already found, and what is
missing is a check that something follows it.

**What would close it.** Two halves, separable. Resolving every cited number against the corpus is
mechanical and has no false-positive class. Requiring a quote after a prose marker needs a decision
about the discharge rule — root `CLAUDE.md` says a pointer to a slug that already carries the quote
discharges the obligation, so a marker whose quote lives elsewhere is conformant and would have to be
distinguished. `assumption`: the number-resolution half can land alone. Nobody has checked whether
any conformant citation names a rule the corpus does not hold.
