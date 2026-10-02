# Step 2, #plan-items: items defined by section, spec files as anchors, scoped citations

This is the spec of step 2 of the milestone in `README.md` beside it. It holds the step's entry.
It depends on defaults D4, D7 and D12 of the milestone document, all ruled by the owner.
The design it implements is the milestone document's "Decided design", under "Items", "Item
citations" and "A spec file is an anchor too". Read that document entire first.

**A design session is expected at this step's audit.** A spec file as an anchor is new (every anchor
is a directory today), and #file-anchor-fits-path-model of the milestone document is judged here.
If the audit finds a load-bearing gap, the step stops and the session's design is written into this
spec before any code.

## Builds

Everything below is in crates/core, except the last item.

1. **Every plan anchor carries four item registers**, kinds `thread`, `argument`, `criterion` and
   `acceptance` (#plan-items-by-section). An item is a level-three heading ending with its slug,
   `### <statement> ##<id>`, under the level-two section whose title gives its kind: Threads,
   Arguments, Criteria, Acceptance criteria. The slug grammar is the entry-id grammar that exists.
2. **A spec file under docs/plans/specs/ is an anchor** named by its basename, carrying the four
   item registers and nothing else: no `path` kind, per clause P1. A milestone directory's anchor,
   from step 1, gains the four item registers; its README and its step specs share one item
   namespace, so an id defined in both is the duplicate finding that exists
   (`Entities::report_duplicates` in `path@core@src/entity.rs`).
3. **An item citation is refused outside its plan anchor** (#plan-item-scope): a reference whose
   anchor is a plan anchor and whose kind is an item kind resolves only from a file inside that
   anchor, the spec file itself or the milestone directory. The repair names the whole-document
   form, `spec@plans@<id>` or `milestone@plans@<id>`.
4. **A slug outside the four sections of a plan document** is the misplaced-definition finding that
   exists, with a reason naming the four sections.
5. **The fixed section titles of a plan document are checked** (default D4 of the milestone
   document): the `sections` declared on the `spec` registers, in the order of §4 of the planning
   skill as step 3 will state it, including the Arguments section. A step spec's sections are its
   entry's, from §5 of the planning skill. A milestone's README is excluded from the File shape's
   entry checks today (`is_entry` in `path@core@src/check/registers.rs`), so its section check
   needs a path of its own.
6. **This milestone document's identifiers become definitions**: each plain `#<id>` that names a
   thread, argument, criterion or acceptance criterion becomes a heading definition in its
   section, and each mention a citation `<kind>@structured-plans@<id>`. The arguments move from
   their table under Threads into a level-two Arguments section (D7), and the threads, criteria
   and acceptance criteria from their table rows into headings. It is an edit of the document's
   structure, not of its content.

## Claims

- **An item is defined by its section**: a `##<id>` heading under Threads defines
  `thread@<plan>@<id>`, and the same heading under Arguments defines an `argument`. Tests in the
  style of the heading-register tests of `path@core@src/entity.rs`.
- **A citation from inside the plan resolves; the same span outside it is refused** with the
  whole-document repair. Tests in `path@core@src/check/references.rs`.
- **A spec file's items do not make it a `path` anchor**: `path@<spec>@x` is refused.
- **Every heading register of a Component is unaffected**: the conformant mocks still report
  nothing.
- **A commit message cites a plan document whole, never an item** (D12): an item citation in a
  message is refused as from outside its plan, while `spec@plans@<id>` and `milestone@plans@<id>`
  resolve there, against the parent when the commit deletes the plan. A message is parsed as one
  document at the root (`judge_message` in `path@core@src/cli/history.rs`), so it is inside no
  plan anchor.

## Fixtures

`planted` gains a plan with an item cited from outside it, a slug outside the four sections, and an
id defined twice in one milestone. The conformant mocks gain one spec and one milestone with items,
cited from inside.

## Audit subjects

- `design@core@an-entry-is-a-heading-at-the-register-level` and
  `design@core@a-slug-belongs-to-a-component`, which this step extends;
- in `path@core@src/entity.rs`: `Entities::heading_definitions`, `register_of`,
  `unslugged_headings`, `Anchors::owning`;
- in `path@core@src/scan.rs`: `SlugSite` and the heading pattern;
- in `path@core@src/check/references.rs`: `judge` and `table`, where a scoped refusal would sit;
- acceptance criterion #file-anchor-fits-path-model of the milestone document.

## Fails alone on

- an item heading defines nothing, or defines the wrong kind;
- an item citation from outside its plan is accepted.

## Premises that expire

- **The installed planning skill still says identifiers are plain `#id`** until step 3. Between the
  two steps, a plan document written under the installed skill fails the section check of item 5.
  The guard is that check: such a document cannot be committed until it takes the new shape.
