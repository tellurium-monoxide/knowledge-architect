# Tripwires — the core checker

Evidence that would reopen a decision about the core checker itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file when it
fires. What is outstanding about the tool is `path@core@docs/open-issues/` beside it.

Entry shape and the movement instruction between the two files are
`knowledge-architect-issue-tracking`. The standing re-entry point is the standing-state
review, which re-reads every tracker file.

## Guarding `design@core@git-supplies-the-walk`'s portability `##walked-count-differs-between-machines`

The walk is a `git` invocation, so what is checked depends on the git installed and on the
per-clone configuration it reads. The design answers the two loud cases — no binary and no
worktree are exit 2 — and this guards the quiet one: two machines running the same commit and
reading a different set of files, with both runs green.

**Fires when:** the walked-file count CI prints for a commit differs from the count a local run
prints for the same commit. Both are the `walk: n file(s)` line of the summary block, which every
`cargo klarch check` prints on a stop as on a pass.

**Response:** open a `defect` naming both counts and the two git versions.

**Re-entry:** each time CI's output is read against a local run, and at any change to the walk.

## Guarding `design@core@grammars-not-prefixes` `##grammars-not-prefixes`

**Fires when:** a `.rs` file in the tree yields no prose region, or no item scope, while its text
holds a top-level `fn` — or a `tree-sitter-rust` or `pulldown-cmark` upgrade changes a node name the
extractor asks for. Read it off `cargo klarch model`: a source file contributing zero observations
that a grep shows to carry a doc comment is the symptom.
**Response:** open a `defect` and stop trusting the run until it is closed. A grammar that yields
nothing removes every citation in the file from the walk and the run still reports success, which is
indistinguishable from a clean tree. `Parsed::trouble` catches the cases the grammar reports; this
guards the case where it reports success and returns nothing.
**Re-entry:** standing, and each time either parser's version changes.

## Guarding `design@core@candidate-rule-and-retired-forms`' silence on an unknown head `##candidate-rule-silence`

**Fires when:** a review, or a session reading a document, finds a backticked span that was
meant as a reference and for which `cargo klarch check` reported nothing — a typo inside the
kind, an unmigrated span whose head is neither a kind nor an anchor, or a shape the tokenizer
does not record. `cargo klarch model` filtered on the file shows whether the span was
recorded as a `span` observation at all.
**Response:** widen the candidate rule to the shape found, at
`design@core@candidate-rule-and-retired-forms`, with a false-positive census over the tree for
the widened class; or re-accept the silence knowingly and record the shape as the reviewer's.
Open a `defect` naming the span if the decision is not reopened in the same change.
**Re-entry:** standing, and the review of the migration that follows the grammar change.

## Guarding `design@core@reserved-anchors`' generic rule `##reserved-anchors-generic-rule`

**Fires when:** a session or a review reports a finding on a `path@*@<path>` reference whose
repair was unclear to whoever hit it — the finding named neither a path to fix nor a component
to anchor at that the repairer could act on, or the reference passed while meaning something the
writer did not intend, such as a path every crate happens to carry without being generic.
**Response:** reopen the generic anchor's checking rule at `design@core@reserved-anchors`: the
accepted set may need to be the required documents alone, or the at-least-one-component test
may need to become an every-component test. Record the confusing instance as an `observation`
with the finding text it produced.
**Re-entry:** standing.

## Guarding `design@core@checker-source-literals-are-data`' self-location `##checker-source-literals-are-data-self-location`

**Fires when:** a `cargo klarch check` run reports a finding on a string literal in a Rust
file under one of the tool's own directories, or the summary block's checker-source line is
absent or names directories other than the Components of the tool's source in a checkout that
holds them. The cheap half is the
binary test `the_core_binary_names_its_own_directory_and_counts_the_files_under_it` in
`path@core@tests/binary.rs`, which asserts the line over this checkout. A binary built from
another checkout of the tool is refused before any command runs, per
`design@core@a-foreign-build-is-refused`, which
`a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused` in
`path@core@tests/binary.rs` asserts, so it reaches this tripwire only where the refusal
cannot see it. An extension binary's own pair is owed by
`design@core@a-foreign-build-is-refused`.
**Response:** the compiled path and the walked tree disagree in a way the refusal does not see: a
second checkout that moved the tool's crates to another relative path, a canonicalisation gap, a
symlink inside the tree the prefix test does not follow.
Open a `defect` carrying the path the line names and the checkout's. Reopen the decision only if
the mismatch is one neither an alias that builds from the checkout nor the refusal can prevent, since the two together
are what the decision rests on.
**Re-entry:** standing.

## Guarding `design@core@checker-source-literals-are-data`' whole-source scope `##checker-source-literals-are-data-whole-source-scope`

**Fires when:** a review finds a string literal in the tool's non-test source that cites a rule:
a `CR:` marker followed by a rule number written out, with a sentence around it that says
something about the rule. A message that interpolates the number it names, and the marker
pattern itself in the scanner, do not fire it. A review is where it surfaces, such as
the data check of thaum's rules-reviewer: nothing mechanical reads that literal any more.
**Response:** the whole-source scope gave that citation up on a count of zero. Either move the
citation into the comment above the literal, or reopen the scope half of the decision and narrow
it to test modules, which costs an attribute lookup in the extractor.
**Re-entry:** standing.

## Guarding `design@core@checker-source-literals-are-data`' purpose `##checker-source-literals-are-data-purpose`

**Fires when:** a test module of the tool gains a named constant whose only use is to be
interpolated into a fixture so that its rule-shaped, slug-shaped or path-shaped bytes are not
spelled at the site. A constant of the module under test, and a constant the tests also read as
a value or reuse across several sites, are not this.
**Response:** the interpolation habit has outlived its reason, or a literal is still read where
the decision says it is not. Establish which: if the checker reported the plain literal, that is
the self-location tripwire above; otherwise open an `observation` and rewrite the fixture inline.
**Re-entry:** standing.

## Guarding `design@core@a-file-register-is-a-directory-of-entries`' closed kind list `##issue-kind-list-grows`

The kind list was closed so that an unknown kind is a finding naming the list, and so that the
kind can decide which subsections an entry owes. The failure that would make the closing wrong is
the list growing instead of being chosen from: each new situation gets a kind of its own, the list
stops partitioning anything, and the finding stops carrying information.

**Fires when:** the declared kind list of any issue register exceeds ten kinds, or a kind added
after the list was closed — one outside the six compiled defaults — ends up with a single member
across the whole project.

**Response:** reopen the open-set argument this decision reversed, in
`path@core@docs/rejected-alternatives.md`.

**Re-entry:** every manifest diff that touches `[registers.issue] kinds`, and
the standing-state review on every dispatched review.

## Guarding `design@core@phases-gate-the-report` — the whole-report stop, and the one producer chain `##phases-gate-the-report-two`

Two clauses, one decision. The report stops at the first phase that finds anything, and the whole
report stops: the per-document form lost on cost, and is in
`path@core@docs/rejected-alternatives.md`. The classification of a finding by the phase that
produces it rests on one chain: the manifest feeds the walk, the walk the model, the model the
entity table, and every check consumes the table and produces nothing another check reads.

**Fires when:** a flag, an environment variable or a manifest row is proposed or added that lets
a run report a later phase while an earlier one holds findings, or lets a writer write over an
incomplete model — the gate felt as an obstacle and answered with a way around it, which is the
check-family selection under another name.

**Response:** reopen the granularity decision with the per-document form, rather than adding the
way around.

**Fires when:** a check of the core, under `path@core@src/check/`, or a check of
an extension reads another check's findings, or a check's output is stored for another check to
consume — a second producer chain,
whose findings would undermine its consumers' with nothing gating them.

**Response:** open a `design` issue proposing the new phase and its place in the order; the
producing check is a phase, not a check of the last one, and it goes before its consumers in
`check::foundation`.

**Re-entry:** the standing-state review on every dispatched review; any change to the arguments
of `check`, `index` or a writing command of an extension; any new module under the check
directory; and any new check of an extension.

## Guarding `design@core@an-extension-builds-its-own-model`' closed entity table `##extension-defines-a-kind`

An extension scans the core's parse on its own and cannot add a kind to the entity table, so its
subject cannot be cited in the `<kind>@<anchor>@<id>` grammar.

**Fires when:** a proposed feature, in a design discussion or a review, needs a reference whose
kind an extension defines rather than the core — a kind naming one rule of a corpus, for example —
and the need is stated as a requirement rather than as an option.
**Response:** reopen `design@core@an-extension-plugs-in-through-phased-hooks` to add a
call through which an extension contributes entities and their definition sites before phase 3, and
re-check `design@core@an-extension-builds-its-own-model` against it.
**Re-entry:** standing.

## Guarding `design@core@api-facade`'s membership `##private-item-needed`

**Fires when:** a consumer of the library, thaum's rules extension first, needs an item that is
private behind the facade, and no public item replaces it. Examples: the records listing, for an
extension command that lists its own entries; the entity table, to resolve references inside an
extension's subject.
**Response:** re-expose the item under the role module its use belongs to, as a 0.MINOR change,
and add it to the facade's entry if it changes the rule rather than the list.
**Re-entry:** thaum's migration onto the published crate, and each extension written after it.

## Guarding `Gathered`, per `design@core@the-core-cli-is-a-library-module` `##inputs-builder-needed`

**Fires when:** an extension's test needs an `Inputs` value that `Gathered::over` cannot produce
from a tree on disk, such as an `Outside::Unreadable` planted in memory. `Inputs` is
non-exhaustive and `Gathered`'s fields are private, so such a test cannot be written.
**Response:** add a builder on `Gathered` that sets the field the test needs, and open a `todo`
if it is not added in the same change.
**Re-entry:** each extension test that plants a defect in the inputs rather than in a mock
project.
