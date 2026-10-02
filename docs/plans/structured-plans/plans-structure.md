# Step 1, #plans-structure: the plans anchor, its two registers, and the milestone anchors

This is the spec of step 1 of the milestone in `README.md` beside it. It holds the step's entry. The
design it implements, the threads it rests on and the procedure for working it are the milestone
document's; read that document entire first, as its section "How a step is worked" says.

**Defaults this step depends on**: D1, D4, D6, D8, D9, D10, D11, D13, D14,
D15, D17 and D18, all ruled by the owner.

**History.** This step's branch is cut from main after the milestone document merged (D18). Its
checker refuses every tree without the plans homes, so its commits up to the one that adds them,
the audit's included, are squashed into one before review, and the squashed message keeps the
audit's subject line.

## Builds

Everything below is in crates/core, except items 8 and 9. No plan document gains items in this step:
identifiers stay plain `#id` text until step 2.

1. **The anchor `plans`**, at docs/plans/ of the root Component, constructed by the tool as a
   location (D6) and never declared in the manifest (#plans-location, #plans-at-root,
   #plans-dir-fixed). Its name is reserved: a declared Component or location named `plans` is
   refused in phase 1, as `elsewhere` is today in `resolve_anchors`
   (`path@core@src/manifest.rs`). It owes docs/plans/README.md (D1) and carries the `path` kind for
   the README and index files of the plans directory and of its two homes (D8).
2. **The built-in register `spec`, carried by `plans` alone** (D1): the File shape, home
   docs/plans/specs/, a hand-written `README.md` and a generated `index.md`. An entry is
   `<id>.md`, cited `spec@plans@<id>` (#plan-document-kinds, #plans-split-dirs).
3. **The built-in register `milestone`, carried by `plans` alone** (D1): home
   docs/plans/milestones/, a hand-written `README.md` and a generated `index.md`. An entry is a
   directory `<id>/` holding a `README.md`, cited `milestone@plans@<id>`, under the rules of D10:
   a third `Shape` variant, the only one this work adds.
4. **One anchor per milestone directory**, named by the directory's basename and built from the
   tree in phase 2 (D11, #plan-anchor). It carries a `spec` register whose home is the milestone
   directory itself (D9): its `README.md` is the milestone document, its `index.md` is generated
   and lists the step specs, and every other `.md` file is a step spec, cited
   `spec@<milestone>@<step>`.
5. **Clause P1**, read as D8: a plan anchor carries no `path` kind; a `path` citation of a plan
   document is refused, and the repair names the `spec` or `milestone` form.
6. **Clause P2**: a plan name that is also the name of a Component, of a location (D15) or of a
   reserved
   anchor is a finding, and so is one name used both under `specs/` and under `milestones/`. It
   applies to milestone names and spec basenames (D13) and is a phase-2 finding (D11).
7. **A project's declaration of `spec` or `milestone`** is refused (D1): the whole
   `[registers.spec]` or `[registers.milestone]` table is a phase-1 complaint, and so is a declared
   location that names either register.

8. **The unit-test fixtures that use docs/plans are reworked**, since the name `plans` becomes
   reserved and the directory becomes the plans anchor:
   - `[locations.papers] path = "./docs/plans"` in
     `a_declared_path_the_manifest_refused_is_reported_here_and_acted_on_by_nothing`, in
     `path@core@src/check/registers.rs`, which asserts the number of findings;
   - `[locations.plans]` and `[locations.inner]` at docs/plans/inner, in the same file;
   - `[locations.papers] path = "./docs/plans"`, in the tests of `path@core@src/manifest.rs`;
   - the path string docs/plans/index.md in
     `a_skipped_file_is_named_by_its_path_and_not_by_its_basename`, in
     `path@core@src/walk.rs`, checked at the audit, since it may be unaffected.

   Each moves off docs/plans or is renamed.
9. **This repository's tree**: docs/plans/ gains `specs/` and `milestones/` with their READMEs and
   generated indexes, and this milestone moves from
   `path@knowledge-architect@docs/plans/structured-plans/` to
   docs/plans/milestones/structured-plans/. Its README keeps its navigation rows to the step
   specs, which still resolve after the move.
10. **Nothing else directly under docs/plans/** (D17): a file or directory there other than
    `README.md`, `specs/` and `milestones/` is a phase-2 finding.

### Readings of the ruled defaults

The author's readings where the cold-implementer review found a ruled default open to two readings.
The owner may contest each; the audit applies them in place unless the code refutes one.

- **Scope** (D1, D6): `spec` and `milestone` take `Scope::OptIn`, and the tool-built location
  `plans`
  names them; no `Scope` variant is added. `Registers::component_scoped` therefore leaves them out.
- **The `plans` location and declared anchors** (D6, D11): `plans` passes through
  `resolve_anchors` and `collides` like any declared location, so a declared anchor inside one of
  its
  homes, or at docs/plans, is refused. Only the milestone anchors are exempt from `collides`.
- **A subdirectory of milestones/ without `README.md`** (D10) is not an anchor and not an entry; it
  is one phase-2 finding.
- **Phases** (D10, D11, D17): every finding about the shape of the plans directory, its homes and
  its
  milestone directories is phase 2, planted in `unsound`; a `path` citation refused by P1 is phase
  4,
  planted in `planted`.
- **`register.toml`** (D10): one under milestones/ and one inside a milestone directory are each a
  phase-2 finding.
- **Spec basenames** (D13): the files directly under specs/. A step spec's basename is scoped to its
  milestone and is not a plan name.
- **How one register has two homes** (D9), and **which listing feeds the generated paths of
  milestone indexes** (D11): implementation choices, settled at the audit and written into this
  spec as applied in place.

### Every `path` target under docs/plans/, after this step

| target | a `path` citation |
| --- | --- |
| docs/plans/README.md, specs/README.md, specs/index.md, milestones/README.md, milestones/index.md | resolves, as `path@plans@<file>` (D8) |
| the directories docs/plans/specs/ and docs/plans/milestones/ | resolves, as `path@plans@<home>/` |
| a spec file under specs/ | refused; the repair names `spec@plans@<id>` (P1) |
| a milestone directory, from any anchor | refused; the repair names `milestone@plans@<id>` (P1). This overrides today's rule that lets an ancestor name a nested anchor's own directory |
| a file inside a milestone directory | refused; the repair names `spec@<milestone>@<step>` or `milestone@plans@<id>` for its README (P1) |
| any other file or directory directly under docs/plans/ | its existence is the D17 finding |
| any target under docs/plans/ cited from the root anchor | refused, as reaching inside the anchor `plans`, by the deepest-anchor rule that exists |

## Claims

Each claim names the test that could refute it and the mutation that shows the test discriminates;
the commit reports the mutation, per `klarch-development`. A test named "new" is written by this
step.

- **A project whose plans homes hold no plan document reports nothing.** The conformant mocks
  `core`, `dirhome` and `minimal` gain empty plans homes, and
  `the_conformant_mocks_report_nothing_over_every_core_check` (`path@core@src/mock_projects.rs`)
  stays green. Mutation: drop the requirement of one home's `index.md`, and the test of the next
  claim fails.
- **Each missing plans home or README is a phase-2 finding** (D1): new tests in the `unsound` module
  of `path@core@src/mock_projects.rs`, one per missing file. Mutation: skip the requirement.
- **`spec@plans@<id>`, `milestone@plans@<id>` and `spec@<milestone>@<step>` resolve**, each to its
  file or directory, and each reports the existing "defines no {kind} `{id}`" finding for an absent
  id: new tests in `path@core@src/check/references.rs`, in the style of its `checked*` tests.
  Mutation: define a milestone's steps under the `plans` anchor instead of the milestone's.
- **A declared anchor named `plans` is a phase-1 complaint**, and so is a declaration of `spec` or
  `milestone`: new tests beside
  `an_anchor_name_that_is_ambiguous_or_reserved_is_reported_at_the_manifest`, in
  `path@core@src/check/registers.rs`. Mutation: remove `plans` from the reserved words.
- **P1**: every row of the table "Every `path` target under docs/plans/" holds, planted in the
  `planted` mock and asserted by its `findings` tests. Mutation: let a plan anchor carry `path`.
- **P2, D15 and D17**: a milestone named like a Component, a spec basename named like a location,
  one
  name under both homes, and a stray file under docs/plans/ are phase-2 findings, planted in
  `unsound` and asserted by its `phase_two` tests. Mutation: skip the name comparison.
- **D10**: a milestone directory without `README.md`, a stray `.md` under milestones/, and a
  `register.toml` under milestones/ are each a phase-2 finding, planted in `unsound`. Mutation:
  accept a directory without a README as an entry.
- **A milestone's `index.md` and each home's `index.md` are generated and checked** like every
  File-shape index, by `file_register_index` (`path@core@src/index.rs`) and
  `path@core@src/check/generated.rs`, and the milestones/ index lists no file of a nested
  milestone anchor: a new test in `path@core@src/index.rs`. Mutation: drop the owning-anchor filter.
- **The anchors of a commit's tree include its milestone anchors**: a new `History` test in
  `path@core@tests/binary.rs` whose message cites `spec@<milestone>@<step>` added by its commit.
  Mutation: build the milestone anchors from the live tree instead of the commit's.
- **A commit message that cites a plan document in the commit deleting it passes** `commits`,
  resolving against the parent, for `spec@plans@<id>` and for `spec@<milestone>@<step>`: new tests
  beside `a_message_naming_the_entry_its_commit_deletes_resolves_against_the_parent`
  (`path@core@tests/binary.rs`), which covers an issue entry. This step's own spec leaves by such a
  commit, which cites it as `spec@structured-plans@plans-structure`, since a `path` citation of it
  is refused from this step on. Mutation: judge the message against the commit's tree alone.

## Fixtures

Every scaffold that builds a conformant tree gains the plans homes as soon as they are required:

- the mocks under `path@core@tests/projects/`; `planted` also gains a `path` citation of a spec,
  and `unsound` the colliding plan names, per the phase rule of `path@core@CLAUDE.md`;
- `tiny_project` in `path@core@tests/binary.rs`, used by every `History` test;
- `complete_tree` in `path@core@src/check/mod.rs`;
- `all_of` in `path@core@src/check/registers.rs`.

Every fixture is expressible with what exists: a file, a directory, a reference span.

## Audit subjects

- the core decisions this step rewrites at its harvest:
  `design@core@anchors-are-components-and-locations`,
  `design@core@reserved-anchors`, `design@core@every-path-names-its-anchor`,
  `design@core@components-carry-the-same-documents`, `design@core@registers-are-declared`,
  `design@core@a-file-register-is-a-directory-of-entries`,
  `design@core@a-file-register-index-is-rows`;
- in `path@core@src/manifest.rs`: `Shape`, `Scope`, `Register`, `Registers::built_in`,
  `Registers::by_name`, `build_registers`, `resolve_anchors`, `collides`; the comments that say four
  registers are built in, and the test
  `the_four_built_in_registers_exist_before_anything_is_declared`;
- in `path@core@src/entity.rs`: `Anchor`, `Anchor::location`, `Anchor::home_of`, `Anchors::of` and
  each of its callers (none receives the tree today, D11), `Anchors::owning`,
  `Anchors::is_anchor_word`, `Anchors::required_kind`, `entry_id`, `Entities::file_definitions`,
  the constants `ESCAPE_ANCHOR` and `EVERY_ANCHOR`;
- in `path@core@src/check/references.rs`: `path()`, `ignore_queries` and `table()`, whose arms
  match the reserved anchors by hand;
- in `path@core@src/index.rs`: `file_register_index`, which has no owning-anchor filter today, and
  `generated_paths`, which reads no tree today;
- `path@core@src/check/tree.rs` `file_home`; `path@core@src/check/registers.rs` `file_home`,
  `directory_contents`, `is_entry`, `entry`;
- the tripwires guarding the heads this step rewrites: `cargo klarch tripwires` lists them, among
  them `tripwire@core@reserved-anchors-generic-rule` and `tripwire@core@issue-kind-list-grows`;
- acceptance criterion #milestone-fits-file-register of the milestone document.

## Fails alone on

- the check of a tree with empty plans homes reports a finding: the mechanism is wrong, not a
  plan document;
- a step spec under a milestone directory is not defined as `spec@<milestone>@<step>`.

## Premises that expire

- **Plain `#id` identifiers**: plan documents keep them through this step. Step 2 ends this; its
  own claims are the guard.
- **The installed planning skill still describes the old layout** until step 3. The installed copy
  is out of the walk, so the check does not see the conflict. A session that writes a plan
  document between steps 1 and 3 follows this milestone's layout; the guard is the check itself,
  which refuses a plan document outside the two homes from this step on.
- **Spans this step makes live.** Once `plans` is an anchor and `spec` and `milestone` are kinds,
  these spans of the milestone document become findings, and this step rewrites them in the
  commit that moves the document:
  - every `path` citation of a file under docs/plans/ from the root anchor, which reaches inside the
    anchor
    `plans`; `grep -rn 'path@knowledge-architect@docs/plans' docs crates` lists them, including
    the four citations repointed in the commit that added the milestone document;
  - the owner's verbatim words in argument a42, which hold `milestone@knowledge-architect@…` and
    `spec@...@...` in backticks: the backticks are removed, the words kept;
  - every other backticked span that opens with `milestone@` or `spec@` and is not a whole
    reference, such as the bare `milestone@` of argument a81 and of the losing alternatives;
    `grep -n '`milestone@\|`spec@' docs/plans -r` lists them.

  The guard is `cargo klarch check` on the moved tree.
