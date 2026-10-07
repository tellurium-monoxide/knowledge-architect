---
kind: question
---
# The bump table has no row for a looser check

## Summary

The table of `design@knowledge-architect@versioning-policy` classes a stricter or added check as
minor, and leaves a patch to "code changes that change no check and no command". A check made
looser fits no row. The changelog of 0.3.0 classes it by the nearest reading. A change to a
command's printed output, met in the same release, is the question of
`issue@knowledge-architect@command-output-is-not-declared-a-contract`.

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
  differ for changes of one kind. This case is carried by
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`, whose design discussion
  decides which part of a command's output is a contract, and so which row such a change takes.

Neither changes the version of 0.3.0, whose highest class is a major entry.

The looser check was then met from a consumer's side, W4 of 2026-10-06-thaum-workflow: the entry
sat under New features, a project whose extension tests counted the core's findings broke on it,
and that project's pin-move procedure read only the Migration entries. The setup skill now says
that a version may make a check stricter or looser. Counting the core's findings is what the
checker crate's section "Testing an extension" advises against, so a consumer following it is not
broken by a looser check; which row such a change takes is still open.

### Why it matters

A writer of an entry cannot tell from the table which class these changes take, and the reviewer
of a release cannot judge it, so the same kind of change is classed two ways within one release.
Under 0.x a minor and a patch are different numbers, so the class decides whether a consumer's
plain `cargo update` reaches the change, which `goal@knowledge-architect@any-project-can-adopt-it`
relies on.

### What would close it

The owner's ruling on a row for a looser check, as a patch or a minor, written into the table of
the decision.
