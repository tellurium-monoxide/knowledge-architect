---
kind: design
---
# The decision that every gate child goes through one spawn helper has no recorded argument

## Summary

`design@gates@one-spawn-helper` states that the gates start every child through the library's one
spawn helper, which a project's commands may use. No head, plan document or commit message records
why one helper is preferred over its nearest rival, so neither the decision's ground nor whether it
earns a head can be judged.

## Details

### What

The head's body describes `path@gates@src/process.rs`: it spawns a child, captures its two streams
into one buffer, optionally mirrors them live, and holds the best-effort output helpers every print
goes through. It argues nothing for having one helper. The nearest rival is each gate, or each
command of a project, spawning its child by its own code.

What was searched:

- The commit that created the head, "The gates runner is a published library, and xtask keeps
  only this repository's list", says only that the head "takes the shared-code half of
  `design@xtask@one-module-per-subcommand`".
- Before that commit, the xtask head "One module per subcommand, one shared spawn helper" gave a
  reason, that shared code earns its place "by being needed by every subcommand", and that this
  "keeps later workflows independent of each other and of `gates`". That reason is about the
  modules of the maintenance tool, not about a published library's children.
- The deleted plan document of the extraction, `path@elsewhere@docs/plans/v0-1-extraction.md`,
  lists the slug and gives no reason.

Instances seen: one, pass 1 of the design-record axis of a project audit. It split the former head
"One spawn helper captures a child's two streams through two pipes" into this one and
`design@gates@streams-through-two-pipes`, which keeps the measured argument for two pipes. The
check of the split found the parent with no standing argument and its entry test not shown: the
module's signatures are an interface, since `path@xtask@src/changelog.rs` imports `complain` and
`say` from it, but a second helper beside the first would change no existing signature. The owner
ruled the split with this entry as the default.

Suspected mechanism, an assumption: the sentence moved from xtask with the code at the extraction,
and its xtask reason was not carried over because it did not apply to the library.

### Why it matters

A session that proposes a second spawn path, for a gate with needs the helper does not meet, cannot
find the argument it would have to defeat. `primer@design-heads` asks a head for its standing
argument and for an entry test that holds; `design@gates@one-spawn-helper` shows neither. The
contracts that hang on the helper, such as removing `RUSTC_BOOTSTRAP` from every child per
`design@gates@gates-scrub-rustc-bootstrap` and the best-effort prints per
`design@gates@verdict-from-exit-codes`, hold for a child only if it is started through the helper,
which may be the reason, but no record says so (`not established`).

### What would close it

The owner states the reason for one helper, or a search finds one, and the head gains it as its
standing argument with the entry test it meets. Or the owner rules that it earns no head, and the
statement leaves the design home for a comment at `path@gates@src/process.rs`, with
`design@gates@streams-through-two-pipes` and `design@xtask@one-module-per-subcommand` re-pointed or
reworded. Re-entry point: the next change that edits `design@gates@one-spawn-helper` or adds a
second way to start a child in the gates library, or earlier on the owner's word.
