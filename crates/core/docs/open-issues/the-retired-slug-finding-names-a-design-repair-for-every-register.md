---
kind: defect
---
# The retired slug reference finding names a design reference as the repair, whatever register the id belongs to

## Summary

A backticked `#<id>` or `<word>#<id>` whose id is an entry of this project is reported as the
retired slug reference form. The finding's repair always reads "write `design@<component>@<slug>`".
When the id is an issue, a goal, a tripwire or a plan item, that repair names the wrong kind.

## Details

### What

Established by reading the code, not by a run:

- `names_this_project` in `path@core@src/check/references.rs` reports the span when
  `Entities::defines_id` holds for its id.
- `Entities::defines_id` in `path@core@src/entity.rs` is true for an id of any register whose kind
  is not a harness kind: design, goal, tripwire, issue, spec, milestone and the plan items.
- The finding for `Observation::Retired(RetiredForm::SlugRef(span))` in the same file writes the
  fixed repair "write `design@<component>@<slug>`; the form with a `#` is no longer read as a
  reference".

So a span such as a backticked `#<id>` whose id is an issue entry's gets a repair that names
a design reference. The reproduction: a mock project with one issue entry, and a document that writes its
id with a `#` in backticks; the finding's repair line names a design reference.

Reported by the L7 lens of the fifth pass of the agentic-workflow audit, as an item met outside
its axis.

### Why it matters

`design@core@an-unresolved-reference-is-reported` gives each finding the repair it needs. An agent
that follows this message writes a design reference to an issue, which the next check reports
as unresolved, so the repair costs a second round.

### What would close it

The finding names the kind of the entry the id resolves to, or every kind that defines it when
several do, with a test over a mock project whose `#<id>` names an issue entry.
