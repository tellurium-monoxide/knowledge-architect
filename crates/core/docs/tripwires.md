# Tripwires — the core checker

Evidence that would reopen a decision about the core checker itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file when it
fires. What is outstanding about the tool is `path@core@docs/open-issues/` beside it.

Entry shape and the movement instruction between the two files are
`skill@knowledge-architect-issue-tracking`. The standing re-entry point is the standing-state
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

## Guarding `design@core@planned-path-form`'s scope: a planned file named outside the plans directory `##planned-path-outside-plans`

**Fires when:** `check` reports a `path@elsewhere@<path>` reference outside the plans directory
whose target now resolves, in a sentence that said the file was to be created.
**Response:** reopen the scope of `design@core@planned-path-form`, with every document as the
candidate.
**Re-entry:** the standing-state review of every dispatched review, and the commit that creates
the file, which fails `check` on that reference.

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
the mismatch is one that neither an alias that builds from the checkout, nor the refusal, nor a
build tied to its checkout per `design@knowledge-architect@a-build-is-tied-to-its-checkout` can
prevent, since the three together are what the decision rests on.
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
private behind the facade, and no public item replaces it. An example still unmet: the records
listing, for an extension command that lists its own entries. The entity table and the anchors
fired it once, recorded as `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`.
**Response:** open an issue for the instance. Its fix gives the consumer what it needs under the
role module its use belongs to, as a 0.MINOR change: the item itself where its shape is fit to
publish, or a narrow read-only view of it where publishing it would expose the core's internals,
the direction the owner chose for the entity table. Add it to the facade's entry if it changes the
rule rather than the list.
**Re-entry:** thaum's migration onto the published crate, and each extension written after it.

## Guarding `Gathered`, per `design@core@the-core-cli-is-a-library-module` `##inputs-builder-needed`

**Fires when:** an extension's test needs an `Inputs` value that `Gathered::over` cannot produce
from a tree on disk, such as an `Outside::Unreadable` planted in memory. `Inputs` is
non-exhaustive and `Gathered`'s fields are private, so such a test cannot be written.
**Response:** add a builder on `Gathered` that sets the field the test needs, and open a `todo`
if it is not added in the same change.
**Re-entry:** each extension test that plants a defect in the inputs rather than in a mock
project.

## Guarding `design@core@plans-at-root`: a project asks for more than one plans directory `##plans-directory-split-asked`

The decision rests on the premises its head states. A project that outgrows them asks for the
split before it builds one.

**Fires when:** a project's owner, in a design discussion, an issue or a review, asks for a plans
directory per component, or for more than one plans directory in a project.
**Response:** reopen `design@core@plans-at-root` with the alternative it lost, a plans directory
in every component, in `path@core@docs/rejected-alternatives.md`. Every citation already names its
anchor, so a split adds anchors and rewrites none.
**Re-entry:** the standing-state review on every dispatched review, and each design discussion
about plan documents.

## Guarding `design@core@plan-item-scope`: a plan's prose names another plan's item `##item-of-another-plan-named`

An item is cited from inside its own plan only. The decision rests on the premise its head
states: a whole-document citation carries every dependency between plans that matters.

**Fires when:** a plan document's prose names an item of another plan document, by its id or by
its statement, because no citation of it can be written.
**Response:** reopen `design@core@plan-item-scope` with the alternative it set aside, citations of
items across plans, in `path@core@docs/rejected-alternatives.md`.
**Re-entry:** the standing-state review on every dispatched review, and each design discussion
about plan documents.

## Guarding `design@core@safe-fix-definition`: a fix that makes a choice `##fix-makes-a-choice`

**Fires when:** a fix is proposed for, or added to, `check --fix` whose bytes are not fully
determined by the tree and the pinned version, or that writes or removes a file outside the
installer's namespace and the generated list.
**Response:** reopen `design@core@safe-fix-definition` before the fix is added, rather than widen
what `--fix` does under its present argument.
**Re-entry:** the standing-state review before every merge, and any change that adds a fix.


## Guarding `design@core@staged-tree-source`: a staged pass that a clean checkout of the same tree fails `##staged-pass-fails-the-checkout`

T1 of the premortem of the work that built `check --staged`. Two reads stay outside the snapshot:
the ignore rules a path reference asks, read from disk, and an extension's checks of filesystem
state, not run over a snapshot.

**Fires when:** a commit whose tree passed `check --staged` just before it was made then fails
`check` in CI, or the tree half of `commits`, with no file edited in between.
**Response:** open a `defect` naming the finding and which read it came from: an unstaged ignore
rule, an extension check not run under `--staged`, or another.
**Re-entry:** each time a CI failure is read against a local run.

## Guarding `design@core@fix-refusal-mixed-state`: sessions unstage to get past the refusal `##fix-refusal-routed-around`

T2 of the premortem of the work that built `check --staged`. It fires only in sessions the owner
sees, as `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`
records of every tripwire on agent behaviour.

**Fires when:** a retrospective finding, or an observation of the owner, reports a session that
ran `git reset`, `git restore --staged` or `git stash`, or otherwise unstaged changes, after
`check --fix` refused a partial commit's mismatch, instead of taking one of the two repairs it
named.
**Response:** reopen `design@core@fix-refusal-mixed-state`.
**Re-entry:** each retrospective intake.
