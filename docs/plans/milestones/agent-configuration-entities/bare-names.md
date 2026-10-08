# Bare names: the lint on a backticked bare name of a skill or an agent, and the names rewritten as references

The spec of the third slice of `milestone@plans@agent-configuration-entities`. It holds what only
this slice builds; what crosses slices is in the milestone document. Every item it cites is defined
there. It starts after the section-citations slice has merged.

## Builds

- **The bare-name lint**, in the last phase, beside the path-to-anchor lint of
  `design@core@every-path-names-its-anchor`: a backticked span that is exactly the name of a skill
  or an agent the entity table defines, installed or the project's own, is a finding, whose repair
  is the reference `skill@<name>` or `agent@<name>` [placeholders]. A span that names no defined
  skill or agent is silent, so a crate's name and a name that no longer exists report nothing.
- **Every bare name rewritten**, in the same commit as the lint, since every commit must pass under
  the branch tip's checker. When the milestone was written, 197 spans of a skill or an agent's
  name stood in 46 walked files. The sites are re-listed by running the lint at the slice's start.
- **The released changelog sections** rewritten the same way, on the owner's ruling in R2 that it
  is a change of structure. A released section that names a skill or an agent that no longer exists
  keeps the bare name, which the lint leaves silent, as
  `knowledge-architect-transcript-conformity-reviewer` in the section that records its rename.
- **The installed copies** under .claude, by `cargo klarch install-agent-skills`, in the same commit
  as each change to content/.
- **The changelog entries** of this slice, in the `Next release` section, then `cargo x changelog`.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| A backticked span equal to a defined skill's or agent's name is reported, with the reference as its repair | a unit test of the lint over a model defining one installed and one project skill | a lint matching the installer's prefix alone reports a crate's name, and the test's crate-name case fails |
| A span naming no defined skill or agent is silent | the same test, with a crate's name and a removed agent's name | as above |
| The lint judges a commit message, against the commit's own tree | a binary test built on `History` in `path@core@tests/binary.rs` | a lint run over the walk alone passes the planted message |
| Under `harness = []` the lint reports nothing | a binary test over a copy of the `minimal` mock project | registering the lint whatever the harness reports the span |
| The tree passes with every name rewritten | `cargo klarch check`, and `cargo klarch commits origin/main..HEAD` | the run with the lint and before the rewrite reports one finding per bare name |

## Audit subjects

- The milestone document entire, and this spec.
- `path@core@src/check/references.rs`, where the path-to-anchor lint is written.
- `design@core@every-path-names-its-anchor`, `design@core@candidate-rule-and-retired-forms`,
  `design@knowledge-architect@changelog-entries`, `design@agent-skills@a-past-sentence-is-rewritten`,
  `design@agent-skills@plain-text-is-no-repair`.
- `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`, whose class this lint covers
  for the names of skills and agents.
- `tripwire@agent-skills@plain-text-pointer-found`, which a repair to a bare name after a deletion
  would read as a firing unless the changelog head states the case.
- `path@agent-config@agents/klarch-changelog-reviewer.md` and
  `path@agent-config@skills/klarch-release/SKILL.md`, which restate the changelog rules.

## Fails alone on

- The lint reports a span that names no skill or agent the entity table defines.

## Premises that expire

- **The section-citations slice has rewritten every `§N` citation.** Many of the bare names stand
  beside them, and rewriting both at once would mix the two rewrites in one review.

## Decided design

### #bare-skill-name-reported: the lint, and its repair in a released changelog section

- **The lint.** A backticked span that is exactly a defined skill's or agent's name is a pointer
  written with no kind. Silent, a rename dangles it unseen, the class of
  `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` for these names (a37). The
  match is exact against the entity table, so a crate's name is silent, and a name carries the
  project's prefix or the installer's, so it meets no ordinary word (a38). The cost is a citation
  longer by the kind and one `@` (a39).
- **A released changelog section.** Rewriting a bare name there into a reference is a change of
  structure, which a released section allows (a50). When the skill or the agent is later deleted,
  the reference dangles, and the repair is the bare name again (a53, a54). That is no evasion of
  `design@agent-skills@plain-text-is-no-repair`, "because there is nothing to cite that the checker
  would accept" (a71). `design@knowledge-architect@changelog-entries` states both moves.
- **Nearest rival:** no lint, leaving the names silent, which keeps the failure the lint exists for.

## Mapping tables

Every backticked span equal to a name, and what it becomes:

| the span names | in a released changelog section | anywhere else |
| --- | --- | --- |
| a skill the entity table defines | `skill@<name>` [placeholder] | `skill@<name>` [placeholder] |
| an agent the entity table defines | `agent@<name>` [placeholder] | `agent@<name>` [placeholder] |
| a skill or an agent that no longer exists | the bare name, silent | rewritten to the present, or removed, per `design@agent-skills@a-past-sentence-is-rewritten` |
| a crate, or anything else | unchanged | unchanged |

## Losing alternatives

In the milestone document.

## Acceptance criteria

None: no acceptance criterion of the milestone is judged by this slice.

## Harvest

At this slice's landing, under `knowledge-architect-decision-recording` and
`knowledge-architect-issue-tracking`:

| what | home |
| --- | --- |
| #bare-skill-name-reported | a new head in `path@core@docs/design.md`, §3, slug `bare-skill-name-reported`; `design@knowledge-architect@changelog-entries` rewritten in place for the released sections |
| the restatements | `path@knowledge-architect@CLAUDE.md`, its restatement of the candidate rule and of the changelog rules; `path@agent-config@agents/klarch-changelog-reviewer.md`; `path@agent-config@skills/klarch-release/SKILL.md` |
| `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` | its What narrowed: the names of skills and agents are reported; the bare mention of a slug stays open |
| the changelog | `Next release`: under Migration, every backticked bare name of a skill or an agent is written as its reference, mock projects serving `claude` included. Surface `checks`, class minor |

This slice's spec and the milestone document leave in the commit that completes this harvest.
