---
kind: todo
---
# An extension cannot see which register, anchor or entry a document belongs to

## Summary

The public API gives an extension the documents and their observations, and nothing of the entity
table or the anchors. An extension that checks issue entries or design homes has to recognise them
by path shape, and misses every register home the manifest places elsewhere. The owner chose a
narrow read-only view as the direction, and scheduled no work for it yet.

## Details

### What

What `Prepared::check` receives is `Model`, `Manifest` and `Inputs`, each through the facade of
`design@core@api-facade`. `Manifest` exposes `root`, `command`, `table` and `complaints`; the
anchors, the registers and `Entities` are crate-private. So an extension cannot ask which Component
or location owns a document, whether a document is an entry of a register or a design home, which
entries exist, or what a reference resolves to.

Seen in an extension built by a fresh agent from the published 0.2.0 documentation alone, with
three checks of its own. It reached a working build on the first compile, and recognised entries
with hand-written path tests:

```rust
fn is_issue_entry(doc: &Document) -> bool {
    // ... true when the path holds the two segments docs and open-issues in a row
}
fn is_design_home(doc: &Document) -> bool {
    // ... true for docs/design.md, or a file under docs/design/ other than its README
}
```

Those tests miss a location's issue directory, whose homes sit directly under the location's path,
as this repository's agent-config location shows; a register declared with another `dir`; and any
move of a Component. Its check that an issue's "Why it matters" names a decision tested for a span
opening with the design kind, with no way to know whether it resolves.

Three smaller gaps came out of the same trial:

- a `Finding` carries no field naming the check that wrote it, so the extension prefixed its
  check's name into the text. A consuming project met it again: its tests sorted planted findings
  into the core's and the extension's by their wording, C3 of
  2026-10-07-thaum-mock-reduction-workflow. For that split, `Prepared::check` already returns an
  `ExtensionReport` holding the extension's findings alone; it does not name which of the
  extension's own checks wrote each;
- the rustdoc holds no complete `impl Extension`; the only one is in
  `path@core@tests/extension_api.rs`, which a reader of the published crate does not see.

Thaum's extension, the consumer the tripwire names first, was not examined: whether it meets the
same need, and how it works around it, is `not established`.

This is the need `tripwire@core@private-item-needed` watches for. The tripwire guards the facade's
membership for every later item too, so it stays, restated to name the class, and this entry
records the instance.

### Why it matters

It strains `design@core@api-facade`, whose membership leaves an extension nothing to ask these
questions with. `goal@core@projects-add-their-own-checks` is met "while an extension needs no
change to the core and uses only its public, documented API". An extension that has to guess membership from paths
quietly breaks where the manifest says something else, and with it
`goal@core@relocation-is-one-manifest-edit` for any project that runs one.

### What would close it

The direction the owner chose: a narrow read-only view in the `extension` module, borrowing the
entity table and the anchors, with an owner of a path, the entry at a site, the entries of a
register, and the resolution of a span; rather than making `Entities` and `Anchors` public as
they are. It widens what `design@core@an-extension-builds-its-own-model` lists an extension as
reading. Its design settles how the view reaches `check` (a new parameter breaks every
extension; a field of `Inputs` meets `tripwire@core@inputs-builder-needed`), and whether a
`Finding` gains a check name. Then the implementation, a worked `impl Extension` in the crate
docs.
