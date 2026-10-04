---
kind: question
---
# The bump table has no row for a looser check, nor for a line a command adds to its output

## Summary

The table of `design@knowledge-architect@versioning-policy` classes a stricter or added check as
minor, and leaves a patch to "code changes that change no check and no command". A check made
looser, and a command that prints one more line, fit no row. The changelog of 0.3.0 classes both
kinds by the nearest reading.

## Details

### What

Two cases met in the review of the 0.3.0 release:

- **A looser check.** The commit "Report an unanchored path or a retired slug form only where it
  names something of this project" makes two checks report less. Patch excludes a change to a
  check, and the minor row names only a stricter or added check. The changelog reviewer classed
  both entries minor, as the nearest class.
- **A line added to a command's output.** `check` gained a note naming each file that holds a
  finding and that git does not track; it is classed `cli`, minor, as an addition that changes
  nothing existing. The refusal of a foreign build, in `path@core@src/build_origin.rs`, gained a
  line naming how to prevent it, and was left without an entry, as a patch. The two readings
  differ for changes of one kind.

Neither changes the version of 0.3.0, whose highest class is a major entry.

### Why it matters

A writer of an entry cannot tell from the table which class these changes take, and the reviewer
of a release cannot judge it, so the same kind of change is classed two ways within one release.
Under 0.x a minor and a patch are different numbers, so the class decides whether a consumer's
plain `cargo update` reaches the change, which `goal@knowledge-architect@any-project-can-adopt-it`
relies on.

### What would close it

The owner's ruling on a row for each case, written into the table of the decision: a looser check
as a patch or a minor, and a line a command adds to its output as a patch or a minor.
