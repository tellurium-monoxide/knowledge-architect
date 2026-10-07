---
kind: todo
---
# A bare slug that names a deleted entry is never reported

## Summary

A one-segment span, or a word, that names an entry by its bare slug is silent by design. When the
entry is deleted, such a mention dangles and nothing reports it. Reviewers found three on two
branches. Finding C3 of 2026-10-07-knowledge-architect-workflow.

## Details

### What

`design@core@candidate-rule-and-retired-forms` makes a one-segment span a name rather than a
pointer, so `cargo klarch check` reads none. `cargo klarch show` lists checked references only. The
installed text tells a session to grep for a reference only in a file the checker does not read, or
for a rename: nothing makes the session that deletes an entry look for its bare mentions. The three
instances are in the transcript of that session, and were repaired before the merge, so they are
not reproducible from the tree.

Fixes, with what each costs:

- **The owner's preferred fix:** report as a finding any bare mention of a declared slug. The
  owner: "I think that the best fix would be to check and report as findings any bare mention of a
  declared slug. Maybe it is too costly, unsure about this." Its cost is the false positives: a slug
  is a word in the grammar `[a-z0-9]+(-[a-z0-9]+)*`, and short slugs are ordinary words. The rejected
  alternative "The retired slug reference reported whatever it names", in
  `path@core@docs/rejected-alternatives.md`, lost on that kind of evidence: in other projects every
  match was their own notation. Its argument is about a form, not about a declared slug; whether it
  covers this check is not established.
- **A listing:** `show`, or a new command, lists every bare mention of a slug, and the session that
  deletes an entry runs it. No false finding; it relies on the session running it.

`tripwire@core@candidate-rule-silence` fires on a backticked span meant as a reference that the
check left silent. Whether the three mentions were backticked is not established, so whether it
fired is not established either. Its first response, widening the candidate rule, is the owner's
preferred fix.

### Why it matters

A dangling mention misleads every reader of the text that holds it, against
`goal@knowledge-architect@documentation-stays-consistent`, and it is found only by a reviewer who
happens to read it.

### What would close it

A census of the bare mentions of every declared slug over this tree and one other project's, which
measures the false positives of the check; then the check, or the listing if the census shows the
check too noisy.
