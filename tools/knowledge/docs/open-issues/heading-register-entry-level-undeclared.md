---
kind: todo
---
# A heading register declares no entry level, so a heading that lost its slug is silently no entry

## Summary

An entry of a heading register is a heading carrying a slug, at level two or three, in any
heading register's home, or a slug in a table cell. So the tool cannot tell a heading that is
no entry from an entry whose slug is missing: a tripwire written without a slug defines
nothing, is listed by nothing and is reported by nothing. The owner decided the repair: each
heading register declares the one level its entries sit at, a heading at that level with no
slug is a finding, a slug at any other level is a finding, and table cells stop defining
entries. The work goes on a branch of its own.

## Details

### What

`heading_definitions` in `path@knowledge@documentation/src/entity.rs` builds the entity table
from headings at levels two and three that carry a slug, and from table cells carrying one,
and reports a slug that sits where none may, per `design@knowledge@a-slug-is-a-heading`. A
heading with no slug is neither, so it produces no entity and no finding. Reproduced over a copy
of `path@knowledge@tests/projects/dirhome/`, after `git init` and `git add -A` in the copy, by
appending this to the copy's tripwires home and staging it:

```markdown
## Guarding nothing in particular, a heading with no slug

**Fires when:** never.
**Response:** none.
**Re-entry:** none.
```

`cargo knowledge check` run from the copy prints `PASSED: no findings`, and `cargo knowledge
tripwires` prints `(no entry)` and exits 1.

The decided shape:

- **Each heading register declares its entry level.** A declared register takes a key in its
  `[registers.<name>]` table, for example `level = 3`. The three built-in heading registers
  carry theirs compiled in: `design` at level three, `goal` and `tripwire` at level two.
- **In a register's home, every heading at the declared level carries a slug, and no heading
  at another level carries one.** Each violation is a finding naming the heading. The
  `README.md` of a directory-shaped home is the head and stays out of scope, as it is today;
  a slug on a level-one heading stays a finding.
- **A table cell defines nothing.** The one home holding cell definitions converts each row
  into a heading at its register's level.

Measured on this tree when the design was decided, with the mock projects excluded, by listing
every `##`-to-`###` heading of every heading register's home outside fences:

| home | at the declared level | off-level slugs |
| --- | --- | --- |
| tripwires, level two | 108 slugged, 0 without | 0 |
| goals, level two | 14 slugged, 0 without | 0 |
| design, level three | 217 slugged, 2 without | 0 |

The two design headings without a slug are `### The tree` and `### Why each part is the way it
is`, under `## 2. The components` in `path@thaum@docs/design.md`; both are section text, and the
repair is to raise them to level two. The 21 level-two headings without a slug in design homes
are subject groupings and stay legal. Nine decisions are defined in table cells, all in
`path@thaum-engine@docs/design/core-shape-decisions.md`, from `design@thaum-engine@side-is-the-unit`
onward; each
becomes a level-three heading, which rewrites that document.

### Why it matters

The standing-state reviewer reads `cargo knowledge tripwires` as the list of what to re-read,
and `tracking-open-issues` sends a session to `--guarding` for the tripwires a reversed
decision dangles. An entry outside the listing is re-read by nobody and dangles nothing; the
reviewer's definition carries a work-around sentence, to read the homes as well as the listing.
`design@knowledge@a-slug-is-a-heading` reports a slug in the wrong place so that a migration off
the old form is finishable; a heading in the right place with no slug is the same failure class
with no report.

### What would close it

A branch that:

- reverses `design@knowledge@a-slug-is-a-heading` into the declared-level rule, with table cells
  dropped, and records the manifest key in the core README;
- makes the check stricter in its first commit, together with the two headings of
  `path@thaum@docs/design.md` and the nine rows of
  `path@thaum-engine@docs/design/core-shape-decisions.md`, since every earlier commit of the
  branch would otherwise fail the `commits` gate;
- plants a missing slug at the declared level and a slug off it in
  `path@knowledge@tests/projects/planted/`, declares a level on the `note` register of both
  `minimal` mocks, and tests both findings;
- rewrites the sentences stating "level two or level three" and table cells in root
  `path@thaum@CLAUDE.md`, `recording-a-decision`, the decision-record reviewer's definition and
  the core README's description of `show`,
  and drops the standing-state reviewer's sentence telling it to read the homes as well as the
  listing;
- decides whether the old rule, a slug at level two or three or in a table cell, earns an entry
  in `path@knowledge@docs/rejected-alternatives.md` under `recording-a-decision` section 6.
