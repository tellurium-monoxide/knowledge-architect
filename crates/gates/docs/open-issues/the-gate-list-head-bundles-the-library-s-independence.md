---
kind: design
---
# The gate-list head bundles a second decision, the library's independence from the checker, whose reason no record holds

## Summary

`design@gates@a-project-holds-its-gate-list` holds two decisions: the library decides no gate and
offers a recommended list, and the library depends on nothing of the checker. The second loses to
a different nearest rival, and no head, plan document or commit message records why it was taken.
Under `design@agent-skills@one-decision-per-head` it belongs in a head of its own, and that head
cannot be written without its reason.

## Details

### What

**The instance.** The body of `design@gates@a-project-holds-its-gate-list`, in
`path@gates@docs/design.md`, holds two statements that answer different questions:

1. "The library runs the gates it is given and decides none of them", with `rust_project` as the
   recommended list. Its nearest rival is a library with a fixed list. Its argument is
   `goal@gates@gates-from-a-list`.
2. "The library depends on nothing of the checker: the root finder takes the marker file's name
   from the caller." Its nearest rival is a gates crate that depends on the package
   knowledge-architect, for example to take its `MANIFEST_NAME` constant. The head states no
   argument for it.

Statement 2 is true of the code: `path@gates@Cargo.toml` declares clap as its only dependency, and
`project_root` in `path@gates@src/process.rs` takes the marker's name as a parameter. Another
Component relies on it: `design@core@installed-binary-version-check`, in `path@core@docs/design.md`,
says a gate "starts a binary that makes the call, since the gates library depends on nothing of
the checker", citing the gate-list head.

**What was searched.** The commit that created the head, "The gates runner is a published
library, and xtask keeps only this repository's list", states "The library depends on nothing of
the checker." in its message, with no reason. The deleted plan document of the extraction,
`path@elsewhere@docs/plans/v0-1-extraction.md`, section 3.10, records the thread gates-crate and
the facts it rested on. Among them, "the root finder's use of the manifest's name" is listed as
one of four points that were this repository's own. That explains why the root finder takes the
marker's name from the caller. It states no reason why the crate as a whole depends on nothing of
the checker (`assumption`: the reason is that a published library should not carry one project's
assumptions, but no record says so). The owner was asked for the reason during the first run of
the design-record audit, and gave none.

**Suspected mechanism**: the sentence was written as a fact about the extracted crate, beside the
decision it was extracted with, and was never argued on its own.

**The re-entry point**: the owner's statement of the reason, or the next change that edits
`design@gates@a-project-holds-its-gate-list`, which brings the head to the rules on heads.

### Why it matters

A session that wants the gates crate to depend on the checker, for its manifest name or its
types, cannot find the argument it would have to defeat: the head names two decisions and argues
one, against `design@agent-skills@one-decision-per-head`. A reference to the independence, as the
core's head writes, cannot name the decision it depends on.

### What would close it

The owner states the reason for the independence. Then the head is split: the independence takes a
head of its own with that reason, the gate-list head keeps the list, and the reference in
`design@core@installed-binary-version-check` is re-pointed to the new head. Or the owner rules that
the independence is no decision worth a head, and the sentence leaves the gate-list head for a
comment at `project_root`.
