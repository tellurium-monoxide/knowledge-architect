# Section citations: every `§N` citation of a skill or an agent rewritten as a reference, and the section numbers removed

The spec of the second slice of `milestone@plans@agent-configuration-entities`. It holds what only
this slice builds; what crosses slices is in the milestone document. Every item it cites is defined
there. It starts after the entities slice has merged.

## Builds

- **Every `§N` citation rewritten.** A citation of a section of a skill or an agent by its number
  becomes a reference to that section: `skill@<name>@<slug>` or `agent@<name>@<slug>`
  [placeholders], the skill's own name included when a skill cites itself. A citation of a point
  inside a section keeps the point in prose beside the reference, as "test 4 of" followed by the
  section's reference. When this spec was written, `grep -rc '§'` found 65 lines in 13 files of
  `path@agent-skills@content/` and 29 lines in 9 walked files outside it. The milestone document's
  own citations, in its restatement of the planning skill's procedure, are among them.
- **The section numbers removed** from the level-two headings of the skills and agents of content/,
  and of this repository's `klarch-` skills and agent, per #section-numbers-dropped. A heading
  `## 1. Does it reverse something already recorded? ` with its slug keeps its text and its slug,
  without `1. `.
- **The installed copies** under .claude, by `cargo klarch install-agent-skills`, in the same commit
  as each change to content/.
- **The changelog entry** of this slice, in the `Next release` section, then `cargo x changelog`.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| No line of the walk cites a skill's or an agent's section by number | `grep -rn '§'` over the walk, minus the core's mock projects and the crates' changelog copies, each remaining line judged | the grep before the rewrite returns the 94 lines this spec counts |
| Each rewritten citation names the section its number named | the mapping table below, built before any rewrite, and the review of the slice | a citation rewritten from the heading text alone, without the table, is found by the review comparing the old number with the table |
| No level-two heading of a skill or an agent opens with a number | `grep -nE '^## [0-9]+\.'` over content/, the `klarch-` skills and agent, outside fenced blocks | the grep before the removal returns every numbered heading |
| Every rewritten reference resolves | `cargo klarch check`, and `cargo klarch commits origin/main..HEAD` | a reference to a slug that does not exist is reported dangling |

## Audit subjects

- The milestone document entire, and this spec.
- `design@agent-skills@instruction-record-is-minimal`: the removal of the numbers is a rewording of
  the installed text, recorded by no head.
- The head the entities slice wrote for the section register, and its slugs in each file.
- `path@agent-skills@CLAUDE.md`, whose test 1 cites "§5 of" the retrospective skill, and
  `path@agent-skills@README.md`, which restates two expectation sets "§5 of" that skill names.
- The issues whose text cites a section by number:
  `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`,
  `issue@agent-skills@what-a-site-is-for-installed-text-is-undefined`,
  `issue@agent-skills@test-3-admits-a-practice-its-tool-documents`, and
  `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review`, which names "its
  section 2" of the standing-state reviewer in words.

## Fails alone on

- A rewritten citation resolves, and names a section other than the one its number named.

## Premises that expire

- **The numbers of the sections are still in the headings when the slice starts**, which the
  mapping table needs. The table is built first, before any heading is touched.

## Decided design

### #section-numbers-dropped: the numbers go, and each citation names the section's slug

A number is a second name for a section, and it goes stale on every insertion (a36). Once each
section carries a slug, the slug is the name a citation uses, and the number is removed. A
citation of a point inside a section, such as a numbered test or a numbered step, keeps the
point's number in prose, since those points carry no slug. Nearest rival: keeping the numbers for
reading order, which no fact in the discussion favoured.

## Mapping tables

One table per file whose sections are cited by number, built at the slice's start from the headings
the entities slice left: section number, heading text, slug. Every `§N` citation is rewritten from
its file's table. A citation whose file is ambiguous in its sentence is resolved by reading the
sentence, and named in the commit. The tables are a working artifact of the slice: the commit that
rewrites the citations carries them in its message.

## Losing alternatives

In the milestone document.

## Acceptance criteria

None: no acceptance criterion of the milestone is judged by this slice.

## Harvest

At this slice's landing:

| what | home |
| --- | --- |
| #section-numbers-dropped | none: a rewording of the installed text, per `design@agent-skills@instruction-record-is-minimal`; the commit records it |
| the changelog | `Next release`, under Migration: a project's own text that cites a section of an installed skill or agent by number now cites it by its reference, since the numbers are gone. Surface `agent-skills`, class patch |

This slice's spec leaves in the commit that completes this harvest.
