# Tripwires — the knowledge tool

Evidence that would reopen a decision about `knowledge` itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file when it
fires. What is outstanding about the tool is `path@knowledge@docs/open-issues/` beside it.

Entry shape and the movement instruction between the two files are `tracking-open-issues`. The
standing re-entry point is `standing-state-reviewer`, which re-reads every tracker file.

## Guarding `design@knowledge@git-supplies-the-walk`'s portability `##walked-count-differs-between-machines`

The walk is a `git` invocation, so what is checked depends on the git installed and on the
per-clone configuration it reads. The design answers the two loud cases — no binary and no
worktree are exit 2 — and this guards the quiet one: two machines running the same commit and
reading a different set of files, with both runs green.

**Fires when:** the walked-file count CI prints for a commit differs from the count a local run
prints for the same commit. Both are the `walk: n file(s)` line of the summary block, which every
`cargo knowledge check` prints on a stop as on a pass.

**Response:** open a `defect` naming both counts and the two git versions.

**Re-entry:** each time CI's output is read against a local run, and at any change to the walk.

## Guarding `design@knowledge@grammars-not-prefixes` `##grammars-not-prefixes`

**Fires when:** a `.rs` file in the tree yields no prose region, or no item scope, while its text
holds a top-level `fn` — or a `tree-sitter-rust` or `pulldown-cmark` upgrade changes a node name the
extractor asks for. Read it off `cargo knowledge model`: a source file contributing zero observations
that a grep shows to carry a doc comment is the symptom.
**Response:** open a `defect` and stop trusting the run until it is closed. A grammar that yields
nothing removes every citation in the file from the walk and the run still reports success, which is
indistinguishable from a clean tree. `Parsed::trouble` catches the cases the grammar reports; this
guards the case where it reports success and returns nothing.
**Re-entry:** standing, and each time either parser's version changes.

## Guarding `design@knowledge@sections-cite-the-heading`'s keyword boundary `##sections-cite-the-heading-keyword-boundary`

**Fires when:** a review reports a section reference that claims content in a shape the lint
cannot see — no keyword before the number, or a keyword shape outside the patterns in
`path@knowledge@documentation/src/scan.rs`. The census behind the decision listed every shape the tree
held; this fires on the first shape it did not.
**Response:** reopen `design@knowledge@sections-cite-the-heading`'s boundary: widen the pattern to the
new shape, or re-accept the keyword boundary knowingly and record the shape as the reviewer's.
Either way the review finding is the evidence; open a `defect` naming the shape if the decision
is not reopened in the same change.
**Re-entry:** standing.

## Guarding `design@knowledge@sections-cite-the-heading`'s reviewer delegation `##sections-cite-the-heading-reviewer-delegation`

**Fires when:** a section citation standing where one subrule carries the claim is found in work
already merged to `main` — the delegation to the rules-reviewer was the only enforcement, and it
missed.
**Response:** reopen the delegation half of `design@knowledge@sections-cite-the-heading`: the semantic
check may need mechanical support after all, or the reviewer definition's instruction needs
sharpening. Record the missed instance as a `defect` in the component that carries it.
**Re-entry:** standing.

## Guarding `design@knowledge@sections-cite-the-heading`'s quote obligation `##sections-cite-the-heading-quote-obligation`

**Fires when:** a diff moves a section reference into a code span, or deletes one, where the
surrounding sentence still claims the section's content — the visible effect being that the
heading-quote obligation is avoided rather than met.
**Response:** the readability the heading quote bought is inverting into avoidance. Reopen the
once-per-claim weight of the quote obligation, and record the instance as an `observation`; the
rules-reviewer's genuinely-data check is where the instance surfaces.
**Re-entry:** standing.

## Guarding `design@knowledge@candidate-rule-and-retired-forms`' silence on an unknown head `##candidate-rule-silence`

**Fires when:** a review, or a session reading a document, finds a backticked span that was
meant as a reference and for which `cargo knowledge check` reported nothing — a typo inside the
kind, an unmigrated span whose head is neither a kind nor an anchor, or a shape the tokenizer
does not record. `cargo knowledge model` filtered on the file shows whether the span was
recorded as a `span` observation at all.
**Response:** widen the candidate rule to the shape found, at
`design@knowledge@candidate-rule-and-retired-forms`, with a false-positive census over the tree for
the widened class; or re-accept the silence knowingly and record the shape as the reviewer's.
Open a `defect` naming the span if the decision is not reopened in the same change.
**Re-entry:** standing, and the review of the migration that follows the grammar change.

## Guarding `design@knowledge@reserved-anchors`' generic rule `##reserved-anchors-generic-rule`

**Fires when:** a session or a review reports a finding on a `path@*@<path>` reference whose
repair was unclear to whoever hit it — the finding named neither a path to fix nor a component
to anchor at that the repairer could act on, or the reference passed while meaning something the
writer did not intend, such as a path every crate happens to carry without being generic.
**Response:** reopen the generic anchor's checking rule at `design@knowledge@reserved-anchors`: the
accepted set may need to be the required documents alone, or the at-least-one-component test
may need to become an every-component test. Record the confusing instance as an `observation`
with the finding text it produced.
**Re-entry:** standing.

## Guarding the citation index as a bump work list `##citation-index-bump-work-list`

**Fires when:** `path@rules@index.md`'s cited-rule count falls between two commits that add engine
code, **and a rule that left was cited by anything other than a `path@thaum@docs/plans/` document deleted at
its landing**. The count is in the file's own header, and the old index names each lost rule's
citers. A plan file leaves the tree with its slice by design and takes its citations with it, so a
fall it fully accounts for guards nothing; without that clause this fires at every landing that
deletes a plan file.
**Response:** the regime is being satisfied by dropping rule numbers rather than by quoting them.
That leaves the engine's dependence on the corpus invisible, and a release bump then misses what
depends on it — which is the one thing the index exists to prevent.
**Re-entry:** standing.

## Guarding the rejected citation locator `##rejected-citation-locator`

**Fires when:** a rules bump lands a citation repair at a file and line that a
`cargo knowledge model` filter for that rule number did not list. Take the filter output before the
repairs and compare it against the diff the bump produced.
**Response:** reopen the rejected locator in `path@knowledge@docs/rejected-alternatives.md`. The reason
it lost is that the model dump is complete for this purpose, and a repair the filter missed is that
reason failing — the answer is then either widening the scanner or a filter that states what it
covers, and either way the missed shape is recorded as a `defect`.
**Re-entry:** the next rules bump, and standing.

## Guarding `design@knowledge@checker-source-literals-are-data`' self-location `##checker-source-literals-are-data-self-location`

**Fires when:** a `cargo knowledge check` run reports a finding on a string literal in a Rust
file under the tool's own directory, or the summary block's checker-source line is absent or
names a directory other than the tool's in a checkout that holds the tool. The cheap half is the
binary test `the_summary_names_the_checker_source_and_counts_the_files_under_it` in
`path@knowledge@tests/binary.rs`, which asserts the line over this checkout; what it cannot reach is
a binary built from another checkout and run here.
**Response:** the compiled path and the walked tree disagree: a binary built from another
checkout, a canonicalisation gap, a symlink inside the tree the prefix test does not follow.
Open a `defect` carrying the path the line names and the checkout's. Reopen the decision only if
the mismatch is one the cargo alias cannot prevent, since the alias is what the decision rests on.
**Re-entry:** standing.

## Guarding `design@knowledge@checker-source-literals-are-data`' whole-source scope `##checker-source-literals-are-data-whole-source-scope`

**Fires when:** a review finds a string literal in the tool's non-test source that cites a rule:
a `CR:` marker followed by a rule number written out, with a sentence around it that says
something about the rule. A message that interpolates the number it names, and the marker
pattern itself in the scanner, do not fire it. The rules-reviewer's data check is where it
surfaces: nothing mechanical reads that literal any more.
**Response:** the whole-source scope gave that citation up on a count of zero. Either move the
citation into the comment above the literal, or reopen the scope half of the decision and narrow
it to test modules, which costs an attribute lookup in the extractor.
**Re-entry:** standing.

## Guarding `design@knowledge@checker-source-literals-are-data`' purpose `##checker-source-literals-are-data-purpose`

**Fires when:** a test module of the tool gains a named constant whose only use is to be
interpolated into a fixture so that its rule-shaped, slug-shaped or path-shaped bytes are not
spelled at the site. A constant of the module under test, and a constant the tests also read as
a value or reuse across several sites, are not this.
**Response:** the interpolation habit has outlived its reason, or a literal is still read where
the decision says it is not. Establish which: if the checker reported the plain literal, that is
the self-location tripwire above; otherwise open an `observation` and rewrite the fixture inline.
**Re-entry:** standing.

## Guarding `design@knowledge@a-file-register-is-a-directory-of-entries`' closed kind list `##issue-kind-list-grows`

The kind list was closed so that an unknown kind is a finding naming the list, and so that the
kind can decide which subsections an entry owes. The failure that would make the closing wrong is
the list growing instead of being chosen from: each new situation gets a kind of its own, the list
stops partitioning anything, and the finding stops carrying information.

**Fires when:** the declared kind list of any issue register exceeds ten kinds, or a kind added
after the list was closed — one outside the six compiled defaults — ends up with a single member
across the whole project.

**Response:** reopen the open-set argument this decision reversed, in
`path@knowledge@docs/rejected-alternatives.md`.

**Re-entry:** every manifest diff that touches `[registers.issue] kinds`, and
`standing-state-reviewer` on every dispatched review.

## Guarding `design@knowledge@a-commit-message-is-a-document`'s skip clause `##a-commit-is-skipped-for-a-new-reason`

`commits` skips a commit whose own tree fails, and the branch that introduced the command needs
that clause: its pre-migration commits carry a manifest this tool refuses to load, and every
commit before the tree went green carries findings. The clause was written for that transition
and for nothing else. Left standing past it, it is the mechanism by which the range check passes
while judging less and less: each commit that skips takes its message out of the run, and the
summary is the only place that says so.

**Fires when:** a branch merged to `main` had its `commits` run skip a commit for any reason
other than that transition — read the `n judged, m skipped` line and the per-commit lines of the
run over the branch.

**Response:** the skip clause has outlived its reason. Delete it: a commit whose tree fails
becomes a finding of its own rather than a silence, and `design@knowledge@a-commit-message-is-a-document`
is rewritten to say so.

**Re-entry:** every merge to `main`, read off the gate's own output; and
`standing-state-reviewer` on every dispatched review.
