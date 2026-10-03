# Step 2, #plan-items: items defined by section, spec files as anchors, scoped citations

This is the spec of step 2 of the milestone in `README.md` beside it. It holds the step's entry.
It depends on defaults D1, D4, D7, D9, D10, D12 and D16 of the milestone document, and on its own
default D19, all ruled by the owner.
The design it implements is the milestone document's "Decided design", under "Items", "Item
citations" and "A spec file is an anchor too". Read that document entire first.

**The design session this step's audit opened has converged**, and its design is in the sections
after the step's entry, from "Status and audience" on. Where it and the entry below disagree, the
session's design wins, and the entry is corrected to it in place.

## Builds

Everything below is in crates/core, except items 6 and 7.

1. **Every plan anchor carries four item registers**, kinds `thread`, `argument`, `criterion` and
   `acceptance` (`thread@structured-plans@plan-items-by-section`). An item is a level-three heading ending with its slug,
   `### <statement> ##<id>`, under the level-two section whose title gives its kind: Threads,
   Arguments, Criteria, Acceptance criteria. The slug grammar is the entry-id grammar that exists.
2. **A spec file under docs/plans/specs/ is an anchor** named by its id, carrying the four
   item registers and nothing else: no `path` kind, per clause P1. It owns its file, and the file
   stays an entry of the `spec` register of `plans` (`thread@structured-plans@spec-file-owns-its-items`). A milestone directory's anchor,
   from step 1, gains the four item registers; its README and its step specs share one item
   namespace, so an id defined in both is the duplicate finding that exists
   (`Entities::report_duplicates` in `path@core@src/entity.rs`).
3. **An item citation is refused outside its plan anchor** (`thread@structured-plans@plan-item-scope`): a reference whose
   anchor is a plan anchor and whose kind is an item kind resolves only from a file inside that
   anchor, the spec file itself or the milestone directory. The repair names the whole-document
   form, `spec@plans@<id>`, `milestone@plans@<id>`, or `spec@<milestone>@<step>` for an item a step
   spec defines.
4. **A slug outside the four sections of a plan document** is the misplaced-definition finding that
   exists, with a reason naming the four sections.
5. **The fixed section titles of a plan document are checked** (default D4 of the milestone
   document): the §4 list, with Arguments right after Threads, in sentence case, owed by a spec
   of specs/ and by a milestone's README; the §5 list owed by a step spec
   (`thread@structured-plans@step-spec-sections`, `thread@structured-plans@arguments-after-threads`, `thread@structured-plans@section-titles-sentence-case`, and audit
   finding 1).
6. **This milestone document's identifiers become definitions**: each plain `#<id>` that names a
   thread, criterion or acceptance criterion, and each plain `a<n>` that names an argument, becomes
   a heading definition in its section, and each mention a citation
   `<kind>@structured-plans@<id>`, in the README and in the two step specs beside it, which share
   its anchor. The arguments move from their table under Threads into a level-two Arguments
   section (D7), and the threads, criteria and acceptance criteria from their table rows into
   headings. It changes no decision: each item gains one statement line written from its row, per
   the mapping table, and every prose block of a section that holds items is placed before the
   section's first item heading, where `records` does not read it as part of an item.
7. **This step's CHANGELOG.md entries**, under `design@knowledge-architect@changelog-entries`:
   - New features, `checks`, minor: the item kinds, cited from inside their plan;
   - Migration, `checks`, major: the section check, for a plan document a project already holds;
   - Migration, `checks`, major: an item citation from outside its plan is refused, and a span
     that opens with one of the four kind words and was silent becomes a reference candidate;
   - Migration, `manifest`, major: a declared register named like an item kind is refused
     (`thread@structured-plans@item-registers-built-in`).

### Sites the step joins, found at the plan's review

A spec anchor joins each site that is written for milestone anchors today, and `Shape::Section`
each site that compares shapes:

- `Anchor::carries` (no `path` kind), the milestone branch of `path` in
  `path@core@src/check/references.rs`, and `Anchors::is_anchor_word` (a plan's name is no anchor
  word), in `path@core@src/entity.rs`;
- `Anchor::home_of`, which joins a register's directory to the home base and assumes a directory;
- `Registers::is_plan_register` and `Registers::listed` in `path@core@src/manifest.rs`, which the
  item registers join for `thread@structured-plans@item-registers-built-in`;
- a spec whose id `milestone_refusal` would refuse, or that a milestone also holds, makes no spec
  anchor (Decided design);
- the shape comparisons that are not exhaustive matches: `generated_index_paths` in
  `path@core@src/index.rs` (`!= Shape::Heading`, which would give each item register an index),
  the `_` arm of `records` in `path@core@src/records.rs`, and the `Shape::Heading` filters of
  `register_of` and `heading_homes` in `path@core@src/entity.rs`;
- the `owned` closure of `check::registers::file_home` also filters `directory_contents`, so the
  one function of `thread@structured-plans@spec-file-owns-its-items` reaches both.

### The design audit's findings

Read against main after step 1 and the repairs of its retrospective.

**Applied in place, as the step's binding shape:**

1. **The milestone README's section check needs no path of its own.** Gap: item 5 said the README
   is excluded from the File shape's entry checks. Step 1 made it the entry of the `milestone`
   register, and `check::registers::entry` already judges a Directory entry's README. Answer:
   declaring the sections on `milestone` checks it. Follows from step 1's Directory shape (D10).
2. **The Threads section holds no grouping heading.** Gap: the milestone document groups its
   threads under level-three headings, "Approved (20)", "Withdrawn (4)", "Ruled out (5)",
   "Rulings" and "Arguments". Under item 1 every level-three heading under Threads is an item.
   Answer: item 6 writes each thread's state and its rulings into its own item's body, and the
   grouping headings leave. Follows from `thread@structured-plans@plan-items-by-section`.
3. **An item's kind is read from the level-two heading in force above it**, out of the heading
   observations the scanner already records (`Observation::Heading` carries the level and the
   text). No scanner change. Follows from `thread@structured-plans@plan-items-by-section`.

**For the owner's ruling, each with the default the step would carry; the owner approved each
default in the design session (`thread@structured-plans@item-registers-built-in`, `thread@structured-plans@arguments-after-threads`,
`thread@structured-plans@section-titles-sentence-case`, `thread@structured-plans@step-spec-sections`):**

4. **The four item registers are built in, carried by plan anchors alone, and a project's
   declaration of one is refused**, as D1 rules for `spec` and `milestone`. No ruling names the
   item registers. Default: as stated.
5. **Where the Arguments section sits.** No ruling places it. Default: right after Threads.
6. **The exact section titles.** §4 of the planning skill writes them in lower case; every plan
   document writes them in sentence case. Default: sentence case, as the documents write them;
   step 3 aligns the skill.
7. **A step spec's sections.** Item 5 says a step spec owes its entry's sections, from §5 of the
   planning skill, so the `spec` register owes one list under `plans` and another in a milestone,
   as its home already differs (D9). A step spec with a design session of its own also holds the
   sections of §4 (D16), and no ruling orders the two groups. Default: a step spec owes the §5
   list in order, and the §4 sections it holds are not ordered against it.

**Open at the audit, load-bearing; it stopped the step, and the design session resolved it as
`thread@structured-plans@spec-file-owns-its-items`, below.**

8. **A spec file cannot be an anchor and an entry of the `spec` register of `plans` under the deepest-anchor
   rule, unless one of the two changes.** A document is owned by the deepest anchor whose path holds it
   (`design@core@a-slug-belongs-to-a-component`), and a spec file that is an anchor holds itself.
   `Entities::file_definitions` and `check::registers::file_home` skip every file an anchor deeper
   than the register's own owns, so the file would leave `spec@plans@<id>` and its entry checks; the
   index renderer has no such filter and would still list it. Keeping it there needs one file owned by two anchors, or an anchor that owns
   nothing, which changes `Anchors::owning` or the deepest-anchor rule and fires acceptance
   criterion `acceptance@structured-plans@file-anchor-fits-path-model`, or a change to the File register's entry filter, which
   does not. The choice between these shapes was one the document does not rule, so the gap was
   load-bearing and the step stopped. The session chose the second shape
   (`thread@structured-plans@spec-file-owns-its-items`), so the criterion did not fire, and the landing commit reports it as
   not fired.

## Claims

Each claim names its test and the mutation that shows it discriminates.

- **The section check** (item 5): a spec, a milestone README and a step spec each missing one owed
  section, or holding two out of order, is one finding naming the list; tests beside the entry
  tests of `path@core@src/check/registers.rs`. Mutation: check a step spec against the §4 list.
- **The misplaced-slug reason names the four item sections** for a slug outside them in a plan
  document; a test in `path@core@src/entity.rs`. Mutation: the generic reason.
- **A location naming an item register is refused** at the manifest, beside the declaration
  refusal. Mutation: let the location row keep it.
- **A level-three heading with no slug inside an item section is a finding**, as in any heading
  register's home. Mutation: let an unslugged heading under Threads through.
- **A spec stays an entry of the `spec` register of `plans` while it is an anchor**: it is
  defined, listed in the specs index, and its entry checked. Mutation: drop the spec-file case from the one function the
  entry filters call.
- **A level-three heading of a plan document outside the four item sections owes no slug**: a
  subsection under "Decided design" is section text. Mutation: require a slug at every level-three
  heading of a plan document.
- **A project's declaration of an item register is refused** (`thread@structured-plans@item-registers-built-in`), as
  `spec` and `milestone` are. Mutation: let the declaration through.

- **An item is defined by its section**: a `##<id>` heading under Threads defines
  `thread@<plan>@<id>`, and the same heading under Arguments defines an `argument`. Tests in the
  style of the heading-register tests of `path@core@src/entity.rs`. Mutation: read the kind from
  the first level-two section of the document rather than the one in force.
- **A citation from inside the plan resolves; the same span outside it is refused** with the
  whole-document repair; an undefined item cited from outside is the out-of-plan refusal. Tests in
  `path@core@src/check/references.rs`. Mutation: judge the scope after resolution.
- **A spec file's items do not make it a `path` anchor**: `path@<spec>@x` is refused. Mutation:
  let a spec anchor carry `path`.
- **Every heading register of a Component is unaffected**: the conformant mocks still report
  nothing. Mutation: make `Shape::Section` registers carried by every anchor.
- **A commit message cites a plan document whole, never an item** (D12): an item citation in a
  message is refused as from outside its plan, while `spec@plans@<id>` and `milestone@plans@<id>`
  resolve there, against the parent when the commit deletes the plan. A message is parsed as one
  document at the root (`judge_message` in `path@core@src/cli/history.rs`), so it is inside no
  plan anchor. A `History` test in `path@core@tests/binary.rs`. Mutation: treat the message as
  inside every plan anchor.

## Fixtures

`planted` gains a plan with an item cited from outside it, a phase-4 `references` finding that
raises that check's count in `PLANTED` in `path@core@src/mock_projects.rs`; no row is added. `unsound` gains a slug outside the four sections and an id
defined twice in one milestone other than `twice`, both phase-3 findings, per
`path@core@CLAUDE.md`. `dirhome`, the one conformant mock holding plan documents, gains items in
its spec and its milestone, cited from inside, and every plan document of every mock gains the
sections it owes under item 5, or the section check reports them.

## Audit subjects

- `design@core@an-entry-is-a-heading-at-the-register-level` and
  `design@core@a-slug-belongs-to-a-component`, which this step extends;
- in `path@core@src/entity.rs`: `Entities::heading_definitions`, `register_of`,
  `unslugged_headings`, `Anchors::owning`;
- in `path@core@src/scan.rs`: `SlugSite` and the heading pattern;
- in `path@core@src/check/references.rs`: `judge` and `table`, where a scoped refusal would sit;
- acceptance criterion `acceptance@structured-plans@file-anchor-fits-path-model` of the milestone document.

## Fails alone on

- an item heading defines nothing, or defines the wrong kind;
- an item citation from outside its plan is accepted.

## Premises that expire

- **The installed planning skill still says identifiers are plain `#id`** until step 3. Between the
  two steps, a plan document written under the installed skill fails the section check of item 5.
  The guard is that check: such a document cannot be committed until it takes the new shape.

## Status and audience

The design of the session that this step's audit opened, written for a session that did not
witness it. The session's transcript is the log
`~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/7fa17ca7-61f7-41c1-a0f4-7241640fbfbe.jsonl`,
from the owner's message "Start step 2, #plan-items" to "Table approved, proceed". The shapes the
session decided were read against the code at the plan's review, and the step resumes at §7.3 of the
planning skill. Where the owner's word is needed and the owner is absent, the work does not proceed
on that point. This step's checker refuses every earlier tree of its branch, whose plan documents
lack the item sections, so its commits up to the one that converts them are squashed into one
before the review, keeping the audit's subject line, and a paragraph per squashed commit saying
what the plan review found and repaired. It leaves with this spec, when step 2 lands. Where it and a design home disagree, the
design home wins. Every name it uses is defined here, in the milestone document, or in the code.
The owner ruled every thread, and default D19. The acceptance criterion's name, its
wording and its response are the agent's, inside the approved clause that the spec-file case
sits in one function; the owner may contest them.

## How a step is worked

`knowledge-architect-planning`, §7, restated in the milestone document.

## Names

| name | what it names |
| --- | --- |
| spec anchor | the anchor of one spec file under docs/plans/specs/, named by the spec's id |
| item sections | the four level-two sections of a plan document whose level-three headings are items: Threads, Arguments, Criteria, Acceptance criteria |
| §4 list, §5 list | the section titles of §4 and of §5 of the installed planning skill |

## What the work is

At the audit, every anchor is a directory, and `Anchors::owning` in `path@core@src/entity.rs`
gives each document the deepest anchor whose path holds it. `Entities::file_definitions` and
`check::registers::file_home` count as entries of a File register only the files its own anchor
owns. A spec file that is an anchor owns itself, so it would leave the `spec` register of `plans`.

Outside the work: the installed planning skill's text on items, section titles and identifiers,
and the roadmap, both step 3's.

## What is already decided

- `design@core@a-slug-belongs-to-a-component`: its rule that the deepest anchor owns a document
  is unchanged; the head is rewritten at this step's harvest for the scoped refusal of an item
  citation, a fifth way a reference resolves to nothing, as the milestone document schedules.
- `design@core@an-entry-is-a-heading-at-the-register-level`, as the milestone document schedules.
- `design@core@plan-document-kinds`, `design@core@a-file-register-is-a-directory-of-entries`,
  `design@core@registers-are-declared` and `design@core@anchors-are-components-and-locations`:
  rewritten at this step's harvest for the spec anchor, the item registers and their shape.
- The milestone's `thread@structured-plans@plan-item-scope`, `thread@structured-plans@plan-items-by-section` and `thread@structured-plans@layout-kept`.

The texts `show` lists as referencing the four heads of the third point, beside the design homes'
own cross-references, which the harvest judges with the heads, and the milestone's table for the
first two:

| text | references | updated or judged at |
| --- | --- | --- |
| the root `CLAUDE.md`, "Mechanical validation of documents": the register bullet, the anchor bullet ("a named directory", false once a spec file is an anchor) and the issue-register sub-bullet; and its knowledge-table row for plans, "carries no slug", false once a plan holds items | `registers-are-declared`, `anchors-are-components-and-locations`, `a-file-register-is-a-directory-of-entries` | this step's harvest, on the owner's word, since the file is the owner's configuration |
| the core README, "Registers" and "Plan documents" | `registers-are-declared`, `a-file-register-is-a-directory-of-entries`, `plan-document-kinds` | this step's implementing commit |
| `design@agent-skills@structure-ready` | `plan-document-kinds` | this step's harvest judges it; step 3 reverses it |
| the comments of `path@core@src/check/references.rs`, `path@core@src/check/registers.rs`, `path@core@src/manifest.rs` and `path@core@src/mock_projects.rs` that cite the four heads | the four heads | this step's implementing commit |
| the core rejected alternatives that lost to `registers-are-declared`, `anchors-are-components-and-locations`, `a-file-register-is-a-directory-of-entries`, `a-slug-belongs-to-a-component` or `an-entry-is-a-heading-at-the-register-level` | those heads | this step's harvest judges each for a reason the rewrite voids |
| `tripwire@core@issue-kind-list-grows` | `a-file-register-is-a-directory-of-entries` | this step's harvest; the issue kinds are unchanged |
| `design@core@the-regime-has-no-opt-out`, `design@core@one-entity-table` and `design@core@a-plan-name-reads-as-nothing-else`, which count six built-in registers or describe the spec anchor as unbuilt | the built-in registers, the spec anchor | this step's harvest, found at the review |

## Criteria

### Every item citation is checked, and a spec stays a checked entry `##item-citations-checked`

- **Kind:** binding
- **Source:** `goal@knowledge-architect@documentation-stays-consistent`
- **Satisfaction:** met: `thread@structured-plans@spec-file-owns-its-items`, `thread@structured-plans@items-as-section-registers`

### A document has one owner, the deepest anchor holding it `##one-owner`

- **Kind:** binding, as a presumption
- **Source:** `design@core@a-slug-belongs-to-a-component`
- **Satisfaction:** met: `thread@structured-plans@spec-file-owns-its-items`

### An item is cited only from inside its plan, as ruled `##scope-as-ruled`

- **Kind:** binding, as a presumption
- **Source:** `thread@structured-plans@plan-item-scope`
- **Satisfaction:** met: unchanged

### A spec stays one file, as ruled `##layout-as-ruled`

- **Kind:** binding, as a presumption
- **Source:** `thread@structured-plans@layout-kept`
- **Satisfaction:** met: a spec stays one file

### Plan anchors need few special cases `##few-special-cases`

- **Kind:** weighed
- **Source:** the cost of keeping special cases right
- **Satisfaction:** met: one exception, in one function

## Threads

The section carrying each approved thread's shape is "Decided design"; a ruled-out thread's is "Losing alternatives".

The owner's words that closed them, verbatim, rounds counted from the round that opened the
discussion:

| round | the owner's words |
| --- | --- |
| 1 | "spec-file-owns-its-items approved, defaults approved for the rest." |
| 1 | the two rivals, `thread@structured-plans@spec-anchor-owns-nothing` and `thread@structured-plans@spec-is-a-directory`, fell with the named approval of the thread that won the fork, as the agent's reply stated |
| 2 | "Table approved, proceed" (against the checkpoint table, which closed `thread@structured-plans@items-as-section-registers` and the premortem's clauses) |

### The spec anchor owns its file; the File entry filter keeps a spec file a spec anchor at its own path owns, through one function `##spec-file-owns-its-items`

- **Proposer:** agent
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a99`, `argument@structured-plans@a100`
- **Harvest home:** core design home

### Item registers of shape `Section`: home the plan's documents, kind by the level-two section `##items-as-section-registers`

- **Proposer:** agent
- **Final state:** approved, at the checkpoint
- **Arguments:** `argument@structured-plans@a103`, `argument@structured-plans@a104`
- **Harvest home:** core design home

### Built in, plan anchors only, declarations refused `##item-registers-built-in`

- **Proposer:** agent
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a105`
- **Harvest home:** core `design@core@registers-are-declared`

### Arguments right after Threads `##arguments-after-threads`

- **Proposer:** agent
- **Final state:** approved
- **Arguments:** none beyond the default
- **Harvest home:** agent-skills, at step 3

### Titles matched as the documents write them `##section-titles-sentence-case`

- **Proposer:** agent
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a106`
- **Harvest home:** agent-skills, at step 3

### §4 list under `plans`; §5 list in a milestone; a step spec's §4 sections unordered against it `##step-spec-sections`

- **Proposer:** agent
- **Final state:** approved
- **Arguments:** none beyond the default
- **Harvest home:** core design home

### A spec anchor owns nothing, and an item's plan is found apart from ownership `##spec-anchor-owns-nothing`

- **Proposer:** agent
- **Final state:** ruled-out
- **Resolution:** lost to `thread@structured-plans@spec-file-owns-its-items`
- **Arguments:** `argument@structured-plans@a101`
- **Harvest home:** judged at this step's harvest

### Every spec is a directory holding its README `##spec-is-a-directory`

- **Proposer:** agent
- **Final state:** ruled-out
- **Resolution:** lost to `thread@structured-plans@spec-file-owns-its-items`
- **Arguments:** `argument@structured-plans@a102`
- **Harvest home:** judged at this step's harvest

## Arguments

One sequence across the milestone, continuing from `argument@structured-plans@a98`.

### Keeping the deepest-anchor rule gives the items of every plan one path `##a99`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-file-owns-its-items`
- **Argument:** The deepest-anchor rule is kept, so a spec and a milestone behave alike: the plan anchor owns the plan's documents, and items resolve through one path, owner equal to scope.

### The cost is one exception, at the two entry filters `##a100`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-file-owns-its-items`
- **Argument:** Its cost is one exception at the two sites that filter entries by owner; the index renderer has no such filter (audit finding 8 of step 1's spec, found with `git log --grep='design audit'`). Neither `Anchors::owning` nor the deepest-anchor rule changes, so `acceptance@structured-plans@file-anchor-fits-path-model` does not fire for this shape.

### Two notions of a document's anchor would put items on two code paths `##a101`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-anchor-owns-nothing`
- **Argument:** Two notions of a document's anchor, owner and scope, equal for a milestone and different for a spec, put items on two code paths, and changing `owning` fires the criterion. It bends the rule `design@core@a-slug-belongs-to-a-component` states, where `thread@structured-plans@spec-file-owns-its-items` bends only a File-shape filter.

### A spec as a directory reopens the ruled layout and moves every spec `##a102`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-is-a-directory`
- **Argument:** It reopens `thread@structured-plans@layout-kept` and moves every spec, here and later in thaum; and making `spec` a second Directory register beside `milestone` brings back the question of two kinds of one shape, which the owner ruled out in round 3 of the milestone's discussion.

### Items have one home and four kinds, decided by section `##a103`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@items-as-section-registers`
- **Argument:** A heading register has one home file per anchor and one level; items have one home and four kinds decided by section, which `argument@structured-plans@a82` said needs a model of its own. The section in force is read from the heading records the scanner already makes.

### Four heading registers at one home would each claim the same file `##a104`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@items-as-section-registers`
- **Argument:** Four ordinary heading registers at one home would each claim the same file: `register_of` in `path@core@src/entity.rs` gives a file the first heading register whose home holds it, so three of the four would define nothing. The tool refuses one home for two registers elsewhere for the same reason: `resolve_registers` for component registers, and anchor resolution for the registers of one location. (Corrected at the plan's review: the round named `resolve_registers` alone, which judges component registers only.)

### The item registers are treated as D1 treats the plan registers `##a105`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@item-registers-built-in`
- **Argument:** The item registers are treated as D1 treats `spec` and `milestone`. (Added at assembly, the author's: the item registers' storage is the plan documents, which the tool fixes, as it fixes the plan registers'.)

### Section titles are matched as the documents write them `##a106`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@section-titles-sentence-case`
- **Argument:** The titles are matched as the documents write them, as in "Status and audience"; step 3 aligns the skill's lowercase table.

## New names, in one place

An illustration of the shapes; the names are the design's, the files where each is expected to go.

```text
crates/core/src/manifest.rs
  Shape::Section                     new: the item registers' shape
  THREAD_REGISTER, ARGUMENT_REGISTER, CRITERION_REGISTER, ACCEPTANCE_REGISTER
                                     new, built in, plan anchors only
  Register::section                  new: the level-two section title an item register reads
  the item registers' fields         scope OptIn, shape Section, dir "threads", "arguments",
                                     "criteria" or "acceptance-criteria" (named in the Undefined
                                     repair), level 3, section "Threads", "Arguments", "Criteria"
                                     or "Acceptance criteria"
crates/core/src/entity.rs
  Constructed::Spec                  new: the anchor of one spec file
  Anchor::is_plan                    new: a milestone or a spec anchor
  Anchors::owns_entry                new: whether a file is an entry of an anchor's File register,
                                     the one function of `thread@structured-plans@spec-file-owns-its-items`
  Anchor::sections_of                new: a register's owed sections at this anchor
  Resolution::OutsidePlan            new: an item cited from outside its plan
```

## Decided design

**The spec anchor owns its file** (`thread@structured-plans@spec-file-owns-its-items`). A spec file under specs/ is the
anchor named by its id, and the deepest-anchor rule gives it its own file. The File register's
entry rule gains one case: a file that a spec anchor at exactly its own path owns stays an entry
of the register whose home holds it. The case is written once, in one function both entry filters
call, and names spec anchors only, so a later kind of file anchor does not inherit it (premortem
P12). Nearest rival: `thread@structured-plans@spec-anchor-owns-nothing`, defeated by its two code paths (`argument@structured-plans@a101`).

**Items are a register shape of their own** (`thread@structured-plans@items-as-section-registers`). The four item registers
have the shape `Section`. Their home is the plan anchor's own documents: a spec file, or a
milestone's README and its step specs. An entry is a level-three heading ending with its slug,
under the level-two section whose title the register carries. Only the four item sections owe a
slug at level three; every other level-three heading of a plan document is section text
(premortem P13). Nearest rival: four ordinary heading registers, defeated by the one-home rule
(`argument@structured-plans@a104`).

**The item registers are built in** (`thread@structured-plans@item-registers-built-in`), carried by plan anchors alone; a
project's declaration of one is refused, and so is a location naming one, as D1 does for `spec`
and `milestone`. The cost the owner accepted: a project that already
declares a register named `thread`, `argument`, `criterion` or `acceptance` is refused on upgrade,
and a CHANGELOG migration entry names it (premortem P14).

**The spec anchor follows the milestone anchor** (`thread@structured-plans@spec-file-owns-its-items`). It is built from the
same listing as the milestone anchors, by every caller that builds the anchors of a tree (D11):
one per entry of specs/, grouped or not, named by its id. An id that `milestone_refusal` would
refuse for a milestone, or that a milestone also holds, makes no spec anchor, and its clause P2
finding stands: one name gives one anchor, and the milestone keeps it. A plan anchor's
name is no anchor word, carries no `path` kind, and its `path` refusal names the item and the
whole-document forms. A predicate for both plan anchors replaces the milestone-only tests at each
of these sites.

**One `spec` register, two section lists** (`thread@structured-plans@step-spec-sections`). The list depends on the anchor,
as the home does (D9): the `spec` register declares the §4 list, and `Anchor::sections_of` gives a
register's owed sections at that anchor, the §5 list for `spec` at a milestone anchor and the
register's own list everywhere else. `check::registers::entry` takes the list from it rather than
from the register.

**The scoped refusal** (Builds item 3). `Entities::resolve` takes the citing file, and returns
`Resolution::OutsidePlan` when the kind is an item kind, the anchor a plan anchor, and the citing
file is not inside it. It is judged before the id is looked up, so an undefined item cited from
outside gets the repair that applies.

**The section lists** (`thread@structured-plans@step-spec-sections`, `thread@structured-plans@arguments-after-threads`,
`thread@structured-plans@section-titles-sentence-case`). A spec of specs/ and a milestone's README owe the §4 list, with
Arguments right after Threads, matched in sentence case. A step spec owes the §5 list without Fixtures,
which §5 owes only where a Component drives its tests with authored content, a condition no check
can read (D19, ruled); the §4
sections a step spec with its own design session holds are not ordered against it.

## Mapping tables

**`Shape::Section` at each site that dispatches on a register's shape:**

| site | a `Section` register |
| --- | --- |
| `check::tree::check` (phase 2, homes) | asserts nothing: its home is the plan anchor's own documents, which exist when the anchor does |
| `check::registers::check_under` (phase 4, shape) | asserts nothing beyond the definition findings of phase 3 |
| `index::file_register_indexes` and `generated_index_paths` | no index: an index lists files or directories, and an item is a heading |
| `records::records` (`show`) | the heading's section, as a heading register's entry |
| `register_of` and `heading_homes` in `path@core@src/entity.rs` | a document of a plan anchor holds its item registers; the misplaced-slug reason names the four item sections |
| `outside_the_walk` in `path@core@src/check/tree.rs` | skipped: the plan documents are judged by the File and Directory homes that hold them |
| the level match of `build_registers` in `path@core@src/manifest.rs` | not reached: only declared registers pass through it, and no declaration takes `Section` |
| `Entities::file_definitions` | skipped, as a Heading register is |
| `Anchors::required_kind` | not reached: it reads component registers only |
| `Anchor::home_of` | for a spec anchor, a `Home` whose `file` is the spec file itself; for a `Section` register at any plan anchor, a `Home` nothing asserts |

**The section list each plan document owes:**

| document | owed level-two sections, in order |
| --- | --- |
| a spec of specs/, a milestone's README | "Status and audience" / "How a step is worked" / "Names" / "What the work is" / "What is already decided" / "Criteria" / "Threads" / "Arguments" / "New names, in one place" / "Decided design" / "Mapping tables" / "Losing alternatives" / "Readings" / "Premortem" / "Acceptance criteria" / "Implementation sequence" / "Order rationale" / "Defaults awaiting the owner" / "Harvest" / "Later consequences" |
| a step spec | "Builds" / "Claims" / "Audit subjects" / "Fails alone on" / "Premises that expire" (Fixtures is not checked, per D19) |

**The milestone document's rows, as items (item 6):**

| row | the heading's statement | the item's body |
| --- | --- | --- |
| a thread | its resolution, or for a closed losing thread what it proposed, from the Decided design and the Losing alternatives sections | where it first appears, which names its proposer, final state, arguments as citations, the section carrying its shape, harvest home, and the owner's closing words from the Rulings table |
| an argument, `a<n>` | its paraphrase | round, who, threads as citations, and the key verbatim |
| a criterion | what it requires, written from its source and its satisfaction line | kind, source, satisfaction |
| an acceptance criterion | the decision it guards and what fires it | judged at, response |

Every mention of an item, `#<id>` or `a<n>`, becomes a citation `<kind>@structured-plans@<id>`.
What is no item stays plain: the step ids, the fired criterion that left the table, and the P, D
and T identifiers. The table of rulings on items that are not threads stays as section text of
Threads, under no level-three heading.

## Losing alternatives

| alternative | lost to | the deciding fact |
| --- | --- | --- |
| `thread@structured-plans@spec-anchor-owns-nothing` | `thread@structured-plans@spec-file-owns-its-items` | two notions of a document's anchor (`argument@structured-plans@a101`) |
| `thread@structured-plans@spec-is-a-directory` | `thread@structured-plans@spec-file-owns-its-items` | reopens `thread@structured-plans@layout-kept` (`argument@structured-plans@a102`) |
| four ordinary heading registers for items | `thread@structured-plans@items-as-section-registers` | one home answers for one register (`argument@structured-plans@a104`) |

## Readings

None.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| P12: the File-entry case admits a later kind of file anchor silently | `thread@structured-plans@spec-file-owns-its-items` | converted into a clause: the case names spec anchors only |
| P13: a level-three heading outside the item sections is read as an item and reported | `thread@structured-plans@items-as-section-registers` | converted into a clause: only the four sections owe slugs |
| P14: a project already declaring a register named like an item kind is refused on upgrade | `thread@structured-plans@item-registers-built-in` | survives as a cost: a CHANGELOG migration entry names it |
| P15: a third site filtering entries by owner forgets the case | `thread@structured-plans@spec-file-owns-its-items` | an acceptance criterion, `acceptance@structured-plans@exception-in-one-function` |

No tripwire was proposed.

## Acceptance criteria

### The spec-file case sits in one function `##exception-in-one-function`

- **Guards:** `thread@structured-plans@spec-file-owns-its-items`
- **Judged at:** step 2's landing
- **Fires when:** the spec-file case is spelled anywhere but `Anchors::owns_entry`, which `Entities::file_definitions` and `check::registers::file_home` (and through it `directory_contents`) call
- **Response:** put `thread@structured-plans@spec-file-owns-its-items` back to the owner

## Implementation sequence

The step's entry above, in one branch.

## Order rationale

One step; none.

## Defaults awaiting the owner

None. The owner ruled D19 at the plan's review: "Agreed on D19."

| ruled default | thread it bears on |
| --- | --- |
| D19: a step spec's checked §5 list leaves out Fixtures. §5 owes Fixtures only "where the Component drives its tests with authored content", a condition no check can read, so a fixed list would make it owed by every step spec. This narrows the ruled `thread@structured-plans@step-spec-sections`, whose list named Fixtures, so it waits for the owner; found by the code-claims review of this spec | `thread@structured-plans@step-spec-sections` |

## Harvest

| when | what lands where |
| --- | --- |
| step 2 | step 3's spec already holds `thread@structured-plans@arguments-after-threads` and `thread@structured-plans@section-titles-sentence-case`. Core design home: `thread@structured-plans@spec-file-owns-its-items`, `thread@structured-plans@items-as-section-registers`, `thread@structured-plans@item-registers-built-in`, `thread@structured-plans@step-spec-sections`, with everything the milestone's step 2 harvest row lists, T4 and its rejected alternatives included. Core rejected alternatives: `thread@structured-plans@spec-anchor-owns-nothing` and `thread@structured-plans@spec-is-a-directory`, if the recording tests pass them. The root `CLAUDE.md` edits of "What is already decided", on the owner's word |
| this spec | leaves in the commit that completes step 2's harvest |

## Later consequences

Step 3 aligns the installed planning skill with `thread@structured-plans@arguments-after-threads` and
`thread@structured-plans@section-titles-sentence-case`.

