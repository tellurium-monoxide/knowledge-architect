# Tripwires — the knowledge tool

Evidence that would reopen a decision about `tools/knowledge/` itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file when it
fires. What is outstanding about the tool is `open-issues.md` beside it.

Entry shape and the movement instruction between the two files are `tracking-open-issues`. The
standing re-entry point is `standing-state-reviewer`, which re-reads every tracker file.

## Guarding `knowledge#families-are-the-checks`

**Fires when:** a check family exists that no subagent definition under `../../.claude/agents/`
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

## Guarding `knowledge#scope-and-distance`

**Fires when:** the count of file-and-rule pairs carrying two or more verified quotes rises above 10.
Near zero today. Re-take by grouping the verified quotes `cargo knowledge check --only citations`
walks by their file and rule.
**Response:** the bound is forcing a copy rather than a repair. Re-argue the distance, or the scope
unit, before the duplicates spread — a rule quoted twice in one document is two things to keep in
step at the next release.
**Re-entry:** standing, and at any change to the bound.

## Guarding `knowledge#deferral-retires-itself`

**Fires when:** the deferred list in `knowledge.toml [migration]` gains an entry, or a backlog rises
between two runs. It is a migration's work list and may only shrink.
**Response:** a rising backlog means new work is being written against the old regime. A new entry
means a rule was enforced and then was not. Either is a decision, not a maintenance step, and it is
argued before it lands.
**Re-entry:** standing, and at every change to that list.

## Guarding the citation index as a bump work list

**Fires when:** `docs/rules/index.md`'s cited-rule count falls between two commits that add engine
code. It is 372 at the time of writing; the number is in the file's own header.
**Response:** the regime is being satisfied by dropping rule numbers rather than by quoting them.
That leaves the engine's dependence on the corpus invisible, and a release bump then misses what
depends on it — which is the one thing the index exists to prevent.
**Re-entry:** standing.

