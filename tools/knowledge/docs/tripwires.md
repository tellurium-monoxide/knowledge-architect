# Tripwires — the knowledge tool

Evidence that would reopen a decision about `knowledge` itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file when it
fires. What is outstanding about the tool is `open-issues.md` beside it.

Entry shape and the movement instruction between the two files are `tracking-open-issues`. The
standing re-entry point is `standing-state-reviewer`, which re-reads every tracker file.

## Guarding `knowledge#families-are-the-checks`

**Fires when:** a check family exists that no subagent definition under `thaum@.claude/agents/`
names, or a definition names a family the tool does not accept. Read the family list out of the
tool's help, which prints every accepted name, and grep each across that directory.
**Response:** open a `defect`. A family nobody reads is a check whose output reaches no reviewer,
and a definition naming a family that does not exist sends a reviewer to a command that fails. The
mapping from a subject to its families lives in the definitions and nowhere else, which is what
keeps the tool free of this repository's review vocabulary, so nothing links the two sides.
**Re-entry:** standing, and each time a check module is added.

## Guarding `knowledge#grammars-not-prefixes`

**Fires when:** a `.rs` file in the tree yields no prose region, or no item scope, while its text
holds a top-level `fn` — or a `tree-sitter-rust` or `pulldown-cmark` upgrade changes a node name the
extractor asks for. Read it off `cargo knowledge model`: a source file contributing zero observations
that a grep shows to carry a doc comment is the symptom.
**Response:** open a `defect` and stop trusting the run until it is closed. A grammar that yields
nothing removes every citation in the file from the walk and the run still reports success, which is
indistinguishable from a clean tree. `Parsed::trouble` catches the cases the grammar reports; this
guards the case where it reports success and returns nothing.
**Re-entry:** standing, and each time either parser's version changes.

## Guarding `knowledge#sections-cite-the-heading`'s keyword boundary

**Fires when:** a review reports a section reference that claims content in a shape the lint
cannot see — no keyword before the number, or a keyword shape outside the patterns in
`knowledge@documentation/src/scan.rs`. The census behind the decision listed every shape the tree
held; this fires on the first shape it did not.
**Response:** reopen `knowledge#sections-cite-the-heading`'s boundary: widen the pattern to the
new shape, or re-accept the keyword boundary knowingly and record the shape as the reviewer's.
Either way the review finding is the evidence; open a `defect` naming the shape if the decision
is not reopened in the same change.
**Re-entry:** standing.

## Guarding `knowledge#sections-cite-the-heading`'s reviewer delegation

**Fires when:** a section citation standing where one subrule carries the claim is found in work
already merged to `main` — the delegation to the rules-reviewer was the only enforcement, and it
missed.
**Response:** reopen the delegation half of `knowledge#sections-cite-the-heading`: the semantic
check may need mechanical support after all, or the reviewer definition's instruction needs
sharpening. Record the missed instance as a `defect` in the component that carries it.
**Re-entry:** standing.

## Guarding `knowledge#sections-cite-the-heading`'s quote obligation

**Fires when:** a diff moves a section reference into a code span, or deletes one, where the
surrounding sentence still claims the section's content — the visible effect being that the
heading-quote obligation is avoided rather than met.
**Response:** the readability the heading quote bought is inverting into avoidance. Reopen the
once-per-claim weight of the quote obligation, and record the instance as an `observation`; the
rules-reviewer's genuinely-data check is where the instance surfaces.
**Re-entry:** standing.

## Guarding `knowledge#reserved-anchors`' generic rule

**Fires when:** a session or a review reports a `*@` finding whose repair was unclear to
whoever hit it — the finding named neither a path to fix nor a component to anchor at that the
repairer could act on, or the reference passed while meaning something the writer did not
intend, such as a path every crate happens to carry without being generic.
**Response:** reopen the generic anchor's checking rule at `knowledge#reserved-anchors`: the
accepted set may need to be the required documents alone, or the at-least-one-component test
may need to become an every-component test. Record the confusing instance as an `observation`
with the finding text it produced.
**Re-entry:** standing.

## Guarding the citation index as a bump work list

**Fires when:** `thaum@docs/rules/index.md`'s cited-rule count falls between two commits that add engine
code, **and a rule that left was cited by anything other than a `thaum@docs/plans/` document deleted at
its landing**. The count is in the file's own header, and the old index names each lost rule's
citers. A plan file leaves the tree with its slice by design and takes its citations with it, so a
fall it fully accounts for guards nothing; without that clause this fires at every landing that
deletes a plan file.
**Response:** the regime is being satisfied by dropping rule numbers rather than by quoting them.
That leaves the engine's dependence on the corpus invisible, and a release bump then misses what
depends on it — which is the one thing the index exists to prevent.
**Re-entry:** standing.

## Guarding the rejected citation locator

**Fires when:** a rules bump lands a citation repair at a file and line that a
`cargo knowledge model` filter for that rule number did not list. Take the filter output before the
repairs and compare it against the diff the bump produced.
**Response:** reopen the rejected locator in `knowledge@docs/rejected-alternatives.md`. The reason
it lost is that the model dump is complete for this purpose, and a repair the filter missed is that
reason failing — the answer is then either widening the scanner or a filter that states what it
covers, and either way the missed shape is recorded as a `defect`.
**Re-entry:** the next rules bump, and standing.
