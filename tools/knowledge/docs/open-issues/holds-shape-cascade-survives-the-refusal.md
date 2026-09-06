---
kind: observation
---
# An anchor that holds another's register homes is refused, and the misplaced-definition findings still follow

## Summary

A location declared at a component's `docs/` is refused by `nesting`, and the run then also
reports every slug in that component's heading homes as misplaced against the location, and
every reference to them as dangling. The refusal is the first finding; the cascade names the
wrong anchor.

## Details

### What

With `[locations.papers] path = "docs" registers = ["issue"]` appended to a copy of
`path@knowledge@tests/projects/minimal/`, `cargo knowledge check --only registers,references`
prints the refusal, `the location `papers` at `docs` holds the design, goal, tripwire and
issue homes of `minimal``, then three findings saying that the slug `mock-anchor` is written at
`path@knowledge@tests/projects/minimal/docs/design.md`, which is no heading register home of
`papers`, and defines nothing, one per slug in the three heading homes. The count of findings was four in that run. The entity table
in `path@knowledge@documentation/src/entity.rs` attributes each definition to
`Anchors::owning`, which is the deeper location, and the location carries no heading register.

The reverse shape, an anchor INSIDE a file register's directory, no longer cascades: the
outer register's enumeration and the entity table leave to a nested anchor what it owns.
The holding shape cannot take the same repair, because there the location owns the homes
themselves and no rule says which anchor a slug in them belongs to.

### Why it matters

A reader who repairs the findings in order meets the right one first. A reader who takes the
list from its end repairs three homes before reaching the declaration that caused it. It
costs a refused manifest a wrong repair list, and a conformant manifest nothing.

### What would close it

Either of: `Anchors::of` leaving a refused location out of the anchor list, so the run judges
the tree as if the declaration were absent and the refusal is the one finding, with a test
asserting one finding over the shape above; or a decision, recorded under
`design@knowledge@anchors-are-components-and-locations`, that the cascade is accepted for a
refused declaration, with this entry's count moved into the head.
