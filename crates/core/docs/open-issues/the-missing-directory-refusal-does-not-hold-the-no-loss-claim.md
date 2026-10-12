---
kind: defect
---
# The head on `index` credits the missing-directory refusal with a claim that the refusal does not hold

## Summary

`design@core@index-takes-no-dry-run` says that its claim "the worst a run can do is replace a file
with what the tree says that file is" is "held by two refusals": a generated path that is a symlink,
and a destination directory the tree does not have. Then it says "Remove either check and the
paragraph above stops being true." Without the second refusal, a write into a missing directory
fails, and that failure destroys nothing. So the claim may not depend on that refusal. The refusal
still holds the next sentence of the head: a run either refuses having written nothing, or writes
every file.

## Details

### What

The text, at crates/core/docs/design.md, in the head created by the split of
`design@core@generated-files-are-pure` in pass 1 of the design-record audit on this repository:
"**That last claim is held by two refusals, not by the paths being generated.** `fs::write` follows
a symlink and writes through it, so a generated path that is one would replace content this
command never produced; and a destination directory the manifest declares but the tree does not
have is a manifest defect rather than something to create. Both are checked over **every**
destination before any is written, so a run either refuses having written nothing or writes them
all. Remove either check and the paragraph above stops being true."

The check agent of that pass raised it: "Without that check, `fs::write` into a missing directory
fails partway through, and nothing is destroyed." The code confirms it. `check_destinations` in
`path@core@src/cli/mod.rs` refuses a missing directory before any write. Its doc comment gives the
reason: "A run that wrote one index and then failed on the next exited 2 — could not run — having
already changed the tree". Nothing creates the directory, so without the refusal `fs::write` fails,
and nothing is destroyed. The refusal holds the all-or-nothing write, not the no-loss claim.

### Why it matters

`design@core@index-takes-no-dry-run` credits a claim to code that does not hold it, which misleads
the session that edits that code. A session could remove the symlink refusal believing that the
other refusal guards the same loss.

### What would close it

Read the writer in `path@core@src/cli/mod.rs`. Then either reword the head so that each refusal is
credited with the claim it holds, the missing-directory refusal holding the all-or-nothing write,
or record that the claim holds as written, with the reason.
