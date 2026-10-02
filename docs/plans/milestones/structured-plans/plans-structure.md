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
   generated indexes, and this milestone moves from docs/plans/structured-plans/ to
   docs/plans/milestones/structured-plans/. Its README keeps its navigation rows to the step
   specs, which still resolve after the move.
10. **Nothing else directly under docs/plans/** (D17): a file or directory there other than
    `README.md`, `specs/` and `milestones/` is a phase-2 finding.
11. **This step's CHANGELOG.md entries**, under `design@knowledge-architect@changelog-entries`:
    the plans layout is a migration entry (a project moves its plan documents into `specs/` and
    `milestones/`), and the `spec` and `milestone` kinds a new-feature entry.

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
  its milestone directories is phase 2, planted in `unsound`, except a home's missing `README.md`
  or `index.md`, which is phase 4 as in every File home (audit finding 1); a `path` citation
  refused by P1 is phase 4, planted in `planted`.
- **`register.toml`** (D10): one under milestones/ and one inside a milestone directory are each a
  phase-2 finding.
- **Spec basenames** (D13): the files directly under specs/. A step spec's basename is scoped to its
  milestone and is not a plan name.
- **How one register has two homes** (D9), and **which listing feeds the generated paths of
  milestone indexes** (D11): implementation choices, settled at the audit and written into this
  spec as applied in place.

### The design audit's findings, applied as the step's binding shape

Read against the core at the branch point, main's commit 6c20f2b. Each finding states the gap, the
answer, and the decision it follows from. Where a finding corrects a claim or a reading above, the
claim or reading is corrected in place too.

1. **A plans home's README and index are phase 4, as every File home's are.** Gap: the reading
   "Phases" puts every finding about the plans homes in phase 2, and claim 2 says each missing
   README is one. In the code a File register's missing `README.md` or `index.md` is reported by
   `file_home` of `path@core@src/check/registers.rs`, phase 4; `check::tree` asserts only the home
   directory. Moving the two for `spec` alone would change the File shape's rules for the `spec`
   register, which fires #milestone-fits-file-register. Answer: the plans directory, its
   `README.md` and each home directory are phase 2; a home's `README.md` and `index.md`, for
   `specs/`, `milestones/` and every milestone directory, are phase 4. Follows from D10 ("The File
   shape already has a hand-written README, a generated index") and the criterion.
2. **Milestone anchors are built from the tree's present paths, passed to every builder of the
   anchors.** Gap: `Anchors::of` in `path@core@src/entity.rs` reads the manifest alone, and 20
   call sites use it; `index::generated_paths` too, and `Model::build` calls it before the walk.
   Answer: `Anchors::of(manifest, paths)` and `generated_paths(manifest, paths)` take the tree's
   paths: `Inputs::present` in a check, `Model::listing` (git's listing, before any exclusion) in
   `Model::build` and in a command, the commit's own listing in `commits`. A milestone anchor is
   each directory `docs/plans/milestones/<id>/` that holds a `README.md`, whose `<id>` is in the
   entity-id grammar and is the name of no Component, location or reserved anchor. One that fails
   either test is no anchor, and its phase-2 finding stops the run. Follows from D11 ("from the
   walk's present paths, by every caller that builds the anchors of a tree").
3. **One `spec` register, and the anchor decides where its home is.** Gap: `Anchor::home_of` joins
   the register's `dir` to the anchor's home base, so a `spec` home at a milestone's own
   directory has no spelling. Answer: `Anchor` gains a marker, `constructed`, naming the two
   anchors the tool builds, `Plans` and `Milestone`; `home_of` puts every home of a `Milestone`
   anchor at the anchor's own path. The `spec` register keeps `dir = "specs"`. The marker changes
   nothing a location owes or carries beyond D1 (the README of `plans`), D9 (the home) and P1 (no
   `path` kind). Follows from D6, D9 and D11.
4. **The `plans` anchor is held beside the declared locations, not among them.** Gap: putting it
   in `Manifest::locations` makes `check::tree` name it as `[locations.plans]` and tell a reader to
   stop declaring it. Answer: `resolve_anchors` judges a candidate for it after the declared rows,
   and the manifest records whether it was accepted; `Anchors::of` constructs it. Its missing
   directory and its missing `README.md` are phase-2 findings whose repair names the plans layout.
   Follows from D6 ("never declared in the manifest") and D1.
5. **A declared `[registers.spec]` or `[registers.milestone]` is one complaint, whatever it sets.**
   Gap: `build_registers` accepts a table for a built-in and complains per key, accepting `kinds`
   on `issue`. Answer: for the two plan registers the table is refused whole, before any key is
   read. Follows from D1 ("the whole table is a phase-1 complaint").
6. **Clause P2 covers every entry of `specs/`, grouped or not.** Gap: `specs/` has the File shape,
   which accepts declared groups, and the reading "Spec basenames" says "the files directly under
   specs/". Answer: every entry id of the `specs/` instance; each is cited `spec@plans@<id>`, and
   the reading's purpose was to keep step specs out. Follows from D13.
7. **`milestones/` holds its README, its index and milestone directories, and nothing else.** Gap:
   D10 names a stray `.md` and a `register.toml`, and is silent on another file. Answer: every file
   directly under `milestones/` other than `README.md` and `index.md` is a phase-2 finding.
   Follows from D10 and D17.
8. **The milestones index is rendered from the milestone anchors, and the File renderer is
   unchanged.** Gap: claim 8 names an owning-anchor filter in `file_register_index` of
   `path@core@src/index.rs`. No nested anchor can sit in a File home in this step:
   `manifest::collides` refuses a declared one, and the milestone anchors sit in the Directory
   home. Step 2 makes each spec file an anchor, and such a filter would then drop every spec from
   the index of specs/. Answer: the Directory home's index has one row per milestone anchor, its
   README's level-one title linking `<id>/README.md`, in the File index's byte format with no
   metadata column; the File renderer gains no filter. Claim 8's mutation becomes: render the
   milestones home with the File renderer. Follows from D10's row and its skip.
9. **P1 is judged before the deepest-anchor rule.** Gap: a plan document cited from the root both
   reaches inside `plans` and is a plan document. Answer: the P1 finding, whose repair is the form
   that resolves; the table's last row covers targets that are not plan documents. Follows from P1.
10. **The generic anchor `*` skips an anchor that carries no `path` kind.** Gap: the `*` arm of
    `path()` in `path@core@src/check/references.rs` tries every anchor. Answer: a milestone anchor
    is skipped. Follows from P1.
11. **A `<dir>.md` sibling is reported once.** Gap: `check::tree`'s `file_home` reports
    `<home base>/<dir>.md` as the File shape's retired single file; for `spec` under `plans` that
    is docs/plans/specs.md, which D17 also refuses, and for a milestone anchor it is
    `<id>.md` beside the milestone directory, which D10 also refuses. Answer: the File rule
    reports it, and the D17 and D10 checks pass over that one path. Follows from the File rule
    being unchanged (finding 1).
12. **`show milestone@plans@<id>` prints the milestone document.** Gap: `records` in
    `path@core@src/records.rs` dispatches on File and Heading. Answer: a Directory entry's record
    is its README whole, with no group. Follows from D10 (the entry is the directory holding a
    README).
13. **The walk fixture is unaffected.** Gap: item 8 lists `a_skipped_file_is_named_by_its_path_and_not_by_its_basename`
    in `path@core@src/walk.rs` for checking. It hands `walk::live_files` a path string and reads no
    anchor. Answer: unchanged.

**Acceptance criterion #milestone-fits-file-register, read at the audit.** The shape above adds
the one `Shape` variant D10 allows, keeps the File shape's rules for `spec` (finding 1), and builds
both new anchors as locations the tool constructs, marked by finding 3. It is judged again on the
built code, and reported in the landing commit.

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
- **A missing plans directory, plans `README.md` or plans home is a phase-2 finding** (D1): new
  tests in the `unsound` module of `path@core@src/mock_projects.rs`, one per missing path. A
  home's missing `README.md` or `index.md` is phase 4, as in every File home (audit finding 1),
  and a new test asserts it for the milestones home. Mutation: skip the requirement.
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
  `path@core@src/check/generated.rs`, and the milestones/ index lists each milestone once and no
  file inside one: a new test in `path@core@src/index.rs`. Mutation: render the milestones home
  with the File renderer (audit finding 8).
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
  - the owner's verbatim words in argument a42, which hold a milestone citation anchored at the
    root and the spec form with three dots, each in backticks: the backticks are removed, the
    words kept;
  - every other backticked span that opens with the word milestone or spec followed by an at
    sign and is not a whole reference, such as the bare milestone@ of argument a81 and of the
    losing alternatives; a grep under docs/plans for a backtick followed by either word and an
    at sign lists them.

  The guard is `cargo klarch check` on the moved tree.
