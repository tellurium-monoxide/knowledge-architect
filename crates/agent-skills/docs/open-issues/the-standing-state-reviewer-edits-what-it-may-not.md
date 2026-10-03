---
kind: defect
---
# The standing-state reviewer is forbidden to edit, and told to delete and create files

## Summary

The installed agent `knowledge-architect-standing-state-reviewer` opens with "You do not use
`Write` or `Edit`", and later tells the reader to delete a fired tripwire, create its consequence,
create a missing document and split a file. A reader cannot obey both.

## Details

### What

In crates/agent-skills/content/agents/standing-state-reviewer.md, as installed under
.claude/agents/:

- "**You do not use `Write` or `Edit`.** A reviewer that mutates the tree corrupts what the other
  axes ...";
- "A tripwire that has fired **leaves its home**. Delete it there and create the consequence, an
  issue file or a reopened decision.";
- in the table of check failures: "a Component misses a required document. Create it." and "a
  single-file issue register where the register is a directory of entries. Split it."

The second and third read as instructions to the reviewer itself. The likely intent is a repair the
reviewer reports for the dispatcher to make; that intent is an assumption, not established.

Found by the self-consistency review of step 3 of the structured-plans milestone, which did not
edit this file.

### Why it matters

`goal@agent-skills@one-skill-per-activity` is met only while no two installed instructions
contradict. A reviewer that follows the later sentences edits the tree it reviews, which the review
skill's invariants forbid; one that follows the first silently drops the movement.

### What would close it

The sentences that name a file operation say that the reviewer reports it as the repair the
dispatcher makes, and the installed copy is reinstalled with the change.
