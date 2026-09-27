---
kind: defect
---
# A level-two heading with no slug in a tripwires home is silently not an entry

## Summary

An entry of a heading register is a heading carrying a slug. A level-two heading in a tripwires
home that carries none defines nothing, is listed by nothing, can be referenced by nothing, and
no family reports it. A tripwire written that way is invisible to the standing-state reviewer's
listing and to `tripwires --guarding`, which is the silence the register exists to prevent.

## Details

### What

`heading_definitions` in `path@knowledge@documentation/src/entity.rs` builds the entity table
from headings at levels two and three that carry a slug, and reports a slug that sits where none
may. A heading with no slug is neither, so it produces no entity and no finding. Reproduced over
a copy of `path@rules-corpus@tests/projects/dirhome/`, after `git init` and `git add -A` in the copy,
by appending this to the copy's tripwires home and staging it:

```markdown
## Guarding nothing in particular, a heading with no slug

**Fires when:** never.
**Response:** none.
**Re-entry:** none.
```

`cargo knowledge check` run from the copy prints `PASSED: no findings`, and `cargo knowledge
tripwires` prints `(no entry)` and exits 1. The same holds for every heading register: a
level-three heading with no slug in a design home is a decision nothing can cite, and a
level-two heading with no slug in a goals home a goal nothing can name. Whether those two
should be findings as well is undecided, and this entry is about the tripwires home, where the
silence costs the most: a tripwire nobody lists is a parked item with no re-entry point.

### Why it matters

The standing-state reviewer reads `cargo knowledge tripwires` as the list of what to re-read,
and `tracking-open-issues` sends a session to `--guarding` for the tripwires a reversed
decision dangles. An entry outside the listing is re-read by nobody and dangles nothing.
`design@knowledge@a-slug-is-a-heading` reports a slug in the wrong place so that a migration
off the old form is finishable; a heading in the right place with no slug is the same failure
class with no report.

### What would close it

The `registers` family reporting a level-two heading with no slug in a tripwires home as a
finding naming the heading, with a fixture in `path@rules-corpus@tests/projects/planted/` and a
test asserting it; the reviewer sentence that says to read the homes as well as the listing then
leaves. Whether the same finding is owed in a design home and a goals home is decided at the same
time, and the answer is recorded here or in `path@knowledge@docs/design.md`.
