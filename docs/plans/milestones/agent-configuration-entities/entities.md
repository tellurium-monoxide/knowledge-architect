# Entities: the harness kinds, their definitions from the tree, and the section rule

The spec of the first slice of `milestone@plans@agent-configuration-entities`. It holds what only
this slice builds; what crosses slices is in the milestone document. Every item it cites is defined
there, except its own acceptance criteria. It starts after the milestone document has merged.

## Builds

- **The four harness kinds**, `skill`, `agent`, `primer` and `instructions`, answered by
  `Anchors::kind` in `path@core@src/entity.rs` under the `claude` harness alone, the way `path` and
  `planned` are answered there. They are kinds and no register: no anchor carries them and no home
  is declared, so `Registers` keeps its ten built-in registers. `Anchors::kinds_listed` lists them.
- **The anchorless reference** in `candidate`: a new variant beside `Candidate::Reference`, holding
  the kind, the name of the skill or the agent where the kind has one, and the section's slug where
  the span names one. The arities are the mapping table's; any other arity is malformed, with the
  forms in the repair. Every consumer of `Candidate::Reference` judges the new variant:
  `path@core@src/check/references.rs` (the resolution and the dangling finding),
  `path@core@src/records.rs`, and `show` in `path@core@src/cli/mod.rs`, which prints the entity and
  every reference to it, as for any kind.
- **The definitions**, built in `Entities::build`, which gains the installed copies as an input, as
  `Survey::installed` holds them:
  - an installed skill, an installed agent and the primer, and their sections, from each installed
    copy whose `Outside` is `Text`, parsed with `parse` of `path@core@src/source/md.rs`, so the
    fence, frontmatter and heading rules are the walk's;
  - a project skill, a project agent and the root instructions, and their sections, from the walk.
- **Every caller of `Entities::build` hands the installed copies in**: `foundation` in
  `path@core@src/check/mod.rs` and `check_under` in `path@core@src/check/references.rs`, from the
  survey built in `path@core@src/cli/gathered.rs`; the three calls in
  `path@core@src/cli/history.rs`, from `Assembly.survey.installed`, never from `Inputs.installed`,
  which is empty under `commits` on purpose; `show`, `issues` and `tripwires` in
  `path@core@src/cli/mod.rs`, which today build no survey and read the installed copies off the
  disk for this; the test callers in `path@core@src/records.rs`, `path@core@src/mock_projects.rs`,
  `path@core@src/check/registers.rs` and `path@core@src/entity.rs`, with an empty list where the
  test has no installed copy.
- **The section rule**: every level-two heading outside a fenced block of a section home owes a
  slug, and a slug at another level of a section home is a misplaced definition. A section home is
  each skill's `SKILL.md`, each agent's file, the primer and the root instructions. An id is unique
  within its one document. A slug defined at a level-two heading of a project skill or agent is no
  longer a misplaced definition.
- **A name outside the id grammar**, on the owner's ruling of D1: a skill directory or an agent file
  whose name is outside `[a-z0-9]+(-[a-z0-9]+)*` is a phase-2 finding, naming the file.
- **AC1, widened by the owner's ruling recorded under D7 of the milestone document**: every kind
  name is refused as an anchor's name. The kind names are `path`, `planned`, the ten built-in
  registers, the four harness kinds whatever the harness, and every register the manifest declares.
  A Component or a location so named is refused in the anchor refusals of
  `path@core@src/manifest.rs`, beside the reserved words; a plan so named in `milestone_refusal` of
  `path@core@src/entity.rs`. The four harness kinds are also refused as a declared register's name,
  in `resolve_registers` of `path@core@src/manifest.rs`, as `planned` is.
- **The section slug placeholder** in `path@agent-skills@build.rs`: a level-two heading of a file
  under content/ ends with `{{slug:<id>}}`, which the build renders into a backticked `##<id>`. A
  placeholder anywhere else fails the build, as an unused substitution row does.
- **The slugs**: a slug on each of the 130 level-two headings the milestone document counts. Under
  content/ and in its primer as placeholders, and directly in the root CLAUDE.md, in the three
  `klarch-` skills and in the agent `klarch-changelog-reviewer`. They are listed first, in the
  mapping table "The slugs" below. The section numbers stay until the section-citations slice.
- **The installed copies** under .claude, by `cargo klarch install-agent-skills`, in the same commit
  as each change to content/.
- **The core test over the shipped set**, AC3.
- **The setup skill's text**, AC4: `path@agent-skills@content/skills/setup/SKILL.md` states that
  every level-two heading of the root CLAUDE.md, and, per D6 of the milestone document, of a project
  skill or agent, carries a slug; and the finding's text names that repair.
- **The changelog entries** of this slice, in the `Next release` section, then `cargo x changelog`.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| `skill@<name>` resolves to an installed skill, a project skill, or to nothing with a finding naming the skill | a unit test of `path@core@src/entity.rs` over a model holding one project skill and a list of installed copies holding one installed skill | a mutation that drops the installed list in `Entities::build` makes the installed case dangle |
| A section reference resolves only to a slug defined in that skill's own file | a unit test citing a slug of skill A through skill B | a resolver that looks the slug up across every skill passes the wrong case, and the test fails |
| Every arity the mapping table does not list is malformed, with the forms in the repair | a unit test per arity, for each kind | a segmentation that accepts any arity for a harness kind passes a four-segment span |
| Under `harness = []`, a span headed by a harness kind is silent | a binary test over a copy of the `minimal` mock project, which declares `harness = []`, with such a span planted | registering the kinds whatever the harness reports the span |
| A level-two heading of a section home with no slug is a finding; a slug at another level there is misplaced | a binary test over a project serving `claude` and past phase 2, as `harnessed` builds one in `path@core@tests/binary.rs` | a rule applied at level three reports neither case |
| The installed copies define entities and stay outside the walk for references | a unit test planting a dangling reference in an installed copy | walking the installed copies reports the planted reference |
| `show` prints a harness-kind entity and every reference to it | a binary test of `show` on a whole skill and on a section | today the span is refused as malformed, exit 2 |
| AC1 to AC4 | each criterion's test, below | each criterion's own discrimination |
| The tree passes with every slug in place | `cargo klarch check` and `cargo klarch commits origin/main..HEAD` | the run of step 2 before the slugs land reports one finding per unslugged heading: 130 is the expected count |

## Fixtures

- **The `harnessed` sandbox** of `path@core@tests/binary.rs` serves `claude`, installs the shipped
  set and stages it, so a test over it reaches phase 3. The section-rule tests write a project skill
  into it.
- **A `History` project serving `claude`**, for AC2. `History` has no harness support today. The
  slice adds a helper that writes `[agents] harness = ["claude"]` into its manifest, a root
  CLAUDE.md holding the primer's import line, and the installed copies, by running
  `install-agent-skills` in the history project before its first commit, as `harnessed` does for a
  `Sandbox`. Commit 2 then edits one installed copy by hand, which `commits` does not compare.
- **A list of installed copies** for the unit tests of `path@core@src/entity.rs`: pairs of a path
  under the installer's namespace and an `Outside::Text`, built in the test.

## Audit subjects

- The milestone document entire, and this spec.
- `path@core@src/entity.rs`: `candidate`, `Candidate`, `Anchors::kind`, `Anchors::kinds_listed`,
  `Anchors::is_anchor_word`, `Entities::build`, `milestone_refusal`.
- `path@core@src/survey.rs`: `Survey::installed`, `survey`, `from_listing`; the survey's builders,
  `path@core@src/cli/gathered.rs` for `check`, and `read_tree` and the assembly in
  `path@core@src/cli/history.rs` for `commits`.
- `path@core@src/manifest.rs`: `owned_path`, `Manifest::owned`, `OWNED_PREFIX`,
  `resolve_registers`, the anchor refusals, and the module doc that lists the built-in registers.
- `path@agent-skills@build.rs` and `path@agent-skills@src/render.rs`: the comment strip and the
  substitutions, beside which the placeholder goes.
- The core's test `every_shipped_file_is_installed_in_the_owned_namespace` in
  `path@core@src/agents.rs`, beside which AC3's test goes.
- The heads the milestone document lists as rewritten by this slice, and
  `design@core@installed-binary-version-check`.
- The harness's documentation of skills and of subagents, on the provider's documentation site,
  for the two readings the milestone's Readings section leaves not established. The audit records
  the address it read. It also reads **whether the harness names a skill by its directory or by its
  frontmatter `name`**. This spec takes the directory's name as the id, and adds no check of the
  frontmatter. If the harness names a skill by its frontmatter and the two can differ, that is a
  load-bearing gap of the audit.
- The standing entries the milestone's grounding returned:
  `issue@core@installed-file-findings-belong-in-phase-four`, whose premise this slice changes;
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`;
  `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`, since a section home
  opens with a frontmatter block;
  `tripwire@core@candidate-rule-silence`; `tripwire@core@extension-defines-a-kind`;
  `tripwire@core@phases-gate-the-report-two`; `tripwire@agent-skills@structure-rejection-overruled`.

## Fails alone on

- A reference of a harness kind that names an existing section is reported dangling.
- `cargo klarch commits` fails on an earlier commit of a branch that renames a section and repairs
  its citations in a later commit.

## Premises that expire

- **The section numbers stay in the headings**, as in `## 1. Does it reverse something already
  recorded?`, until the section-citations slice removes them. The `§N` citations stay valid prose
  meanwhile. Guard: the section-citations slice's mapping table is built from the headings this
  slice leaves, numbers included.
- **No span headed by `skill`, `agent`, `primer` or `instructions` exists outside the milestone's
  documents**: a grep of the walk and of the messages on origin/main found none when this spec was
  written. Guard: the run of `cargo klarch check` at step 1 reports any that has appeared since.
- **The installed copies of this repository match content/ in every commit**, since an edit of
  content/ is installed in the same commit, per `path@agent-skills@CLAUDE.md`. Nothing checks it
  per commit: `commits` compares no installed copy. Guard: `cargo klarch commits` over the slice's
  branch, which resolves content/'s references against each commit's copies and reports one that a
  commit's copies do not define.

## Implementation sequence

1. **The kinds, empty of sections.** The harness kinds, the anchorless reference and its consumers,
   the definitions of whole skills and agents from the installed copies and the walk, every caller
   of `Entities::build`, `show`, AC1 and the name-grammar finding. No section rule yet, and no
   content uses the kinds. Fails alone on: a whole-skill reference of the milestone's documents
   reported dangling.
2. **The section rule and the slugs, in one commit.** The rule, the placeholder in the build, the
   130 slugs from the mapping table, the installed copies, AC2 and AC3. The rule and the slugs that
   satisfy it land in one commit, because every commit must pass under the branch tip's checker
   (`argument@agent-configuration-entities@a76`). Fails alone on: a heading the table lists reported unslugged.
3. **The text that states it.** AC4's setup-skill text, the changelog entries, then the harvest.

## Decided design

### #skill-register and #claude-md-sections: four kinds, under the `claude` harness

| kind | an entity | defined from | cited |
| --- | --- | --- | --- |
| `skill` | a directory directly under `path@agent-config@skills/` holding `SKILL.md`, its id the directory's name | the installed copies for a name the installer's namespace holds, the walk otherwise | `skill@<name>` |
| `agent` | a Markdown file directly under `path@agent-config@agents/`, its id the file's basename | the same | `agent@<name>` |
| `primer` | the primer, `path@agent-config@knowledge-architect/PRIMER.md`; only its sections are entities | the installed copies | `primer@<slug>` |
| `instructions` | the root CLAUDE.md; only its sections are entities | the walk | `instructions@<slug>` |

The paths are this repository's spelling. The layout is the harness's, a directory `.claude` at the
project's root, in every project.

- **Nothing is declared** (`argument@agent-configuration-entities@a6`): the harness fixes the layout. Under `harness = []` none of the four
  kinds exists, and a span headed by one is silent, as any span whose head is no kind and no anchor.
- **Kinds, not registers.** A harness kind is answered as `path` and `planned` are, with no anchor
  and no declared home, so nothing a project declares can add one. This is a choice inside the
  scope of `thread@agent-configuration-entities@skill-register`: the owner ruled that skills and agents are entities, and no ruling
  names their storage.
- **No README and no index** are owed under the skills and agents directories (`argument@agent-configuration-entities@a8`). The reason is
  an assumption, not established: the harness may read any Markdown file there as a definition.
- **What is no entity**: a directory under the skills directory that holds no `SKILL.md`, a file of
  a skill directory other than `SKILL.md`, and a file under the agents directory that is not
  Markdown or sits in a subdirectory. None is reported. A project-owned one stays an ordinary
  walked document; an installed one stays outside the walk.
- **A scoped CLAUDE.md is not modelled** (`argument@agent-configuration-entities@a51`, `argument@agent-configuration-entities@a52`). The root CLAUDE.md stays, so its sections are.
- **Under `harness = []`, the installed copies a project kept** are walked as its own documents,
  per `design@core@agents-table`, and their rendered slugs are misplaced definitions there. Per D8
  of the milestone document, those findings stand: the repair is to remove the copies or to serve
  the harness.
- **Nearest rival:** no entities, with skills cited by path, as today. It leaves every citation of a
  skill unchecked, which is the need the owner named in R1 (`argument@agent-configuration-entities@a1`).

### #skill-cited-without-anchor, #root-instructions-cited and #primer-sections: the two-segment form

```text
skill@<name>                 the whole skill
skill@<name>@<slug>          one of its sections
agent@<name>                 the whole agent
agent@<name>@<slug>          one of its sections
primer@<slug>                a section of the primer
instructions@<slug>          a section of the root instructions
```

An illustration of the shapes, not of code.

- **No anchor**, because a skill or an agent has one namespace per harness, which the harness
  enforces as directory and file names, so an anchor carries no information (`argument@agent-configuration-entities@a56`). The primer and
  the root instructions are one document each in a project (`argument@agent-configuration-entities@a64`).
- **The rejected alternative "A register reference with no anchor"**, in
  `path@core@docs/rejected-alternatives.md`, gives two reasons. Its first, that "the extracted tool
  cannot know which register is single-instance in a given project", does not hold for a harness
  kind: the tool fixes the four kinds and their layout, and no project declares one. Its second,
  "one three-part grammar serves every kind without a special case in the resolver or in the
  instructions", is the cost `argument@agent-configuration-entities@a14` names, which the owner weighed. Per D3 of the milestone document.
- **The third segment narrows within the kind** (`argument@agent-configuration-entities@a61`), so a section keeps the information that it
  belongs to a skill or to an agent (`argument@agent-configuration-entities@a48`).
- **The kind is asked first**, as `candidate` does today, so a Component, a location or a plan named
  after a harness kind would be shadowed (`argument@agent-configuration-entities@a57`). AC1 refuses such a name.
- **`instructions` names the document's role** (`argument@agent-configuration-entities@a65`), so the announced move to AGENTS.md changes no
  reference.
- **Nearest rival:** `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor`, see the milestone's Losing alternatives.

### #skill-sections-carry-slugs, #agent-sections-carry-slugs and #primer-sections: the section rule

- **The rule of `design@core@an-entry-is-a-heading-at-the-register-level`, at level two**, over the
  section homes: every level-two heading outside a fence owes a slug, and a slug elsewhere in a
  section home is misplaced (`argument@agent-configuration-entities@a24`). An id is unique within its one document.
- **Fenced headings define nothing** (`argument@agent-configuration-entities@a25`), so the illustrations of an issue entry's shape in the
  issue-tracking skill, or of a design entry in the decision-recording skill, owe no slug.
- **A slug's form**: the id grammar, two to four words naming the section's subject, not its number
  or its position.
- **A section slug of the installed text is an interface** (`argument@agent-configuration-entities@a27`, `argument@agent-configuration-entities@a49`): a rename dangles a
  consumer's citations, and owes a Migration entry in the changelog.

### #installed-skills-defined-from-shipped-set: definitions from the judged tree, slugs as placeholders in content/

- **The source.** `Entities::build` takes the installed copies of the tree it judges, as
  `Survey::installed` holds them, and defines the installed skills, agents, the primer, and their
  sections. Under `commits`, `read_tree` reads each commit's installed blobs with every other
  unskipped file, so each commit is judged against its own installed set (`argument@agent-configuration-entities@a72`, `argument@agent-configuration-entities@a73`, `argument@agent-configuration-entities@a74`). `check`
  guarantees that a working tree's copies equal the pinned shipped set, per
  `design@core@owned-namespace-check`.
- **Still outside the walk for references.** A reference inside an installed copy is not read, as
  today. The shipped text's references are judged in this repository, in content/, and by AC3.
- **content/ defines nothing** (`argument@agent-configuration-entities@a31`, `argument@agent-configuration-entities@a33`). A heading there ends with `{{slug:<id>}}`, and the build
  renders it. So the slug a reference in content/ resolves to is the one in the installed copy,
  which this repository installs in the same commit as each change to content/.
- **`design@agent-skills@shipped-text-cites-no-entry` is rewritten**: the shipped text may cite the
  shipped set's own skills, agents, primer and their sections, which every project serving `claude`
  holds, and never `instructions`, whose slugs are each project's own (`argument@agent-configuration-entities@a34`).
- **Nearest rival:** the binary's shipped set as the source, which fails `commits` on an earlier
  commit; and a manifest declaration of content/, which fails `criterion@agent-configuration-entities@no-catering`. See the milestone's
  Losing alternatives.

## Mapping tables

Every arity of a span headed by a harness kind, and what it is:

| kind | 2 segments | 3 segments | any other |
| --- | --- | --- | --- |
| `skill` | the skill | a section of the skill | malformed |
| `agent` | the agent | a section of the agent | malformed |
| `primer` | a section of the primer | malformed | malformed |
| `instructions` | a section of the root instructions | malformed | malformed |

Every source of a harness-kind entity, by project:

| document | in this repository | in a consuming project | under `commits` |
| --- | --- | --- | --- |
| an installed skill, agent, the primer | the installed copies under .claude, which an edit of content/ updates in the same commit | the installed copies | the commit's own installed copies |
| a project skill or agent | the walk, under the location `agent-config` | the walk | the commit's walk |
| the root instructions | the walk | the walk | the commit's walk |

**The slugs.** One row per level-two heading of a section home, outside a fence: the file, the
heading as it stands, and its slug. The slice writes it into this spec in its audit's commit,
before step 2, re-taking the 130 headings with the count the milestone document names; step 2
writes each slug from it, and the slice's review reads it. A section slug of the installed text is
an interface, so this table is where its names are judged.

## Acceptance criteria

### A name a kind would shadow is refused as an anchor's name `##a-kind-name-is-refused-as-a-name`

AC1, applied on the owner's word in R4, its scope widened by the owner's ruling recorded under D7 of
the milestone document.

- **Guards:** `thread@agent-configuration-entities@skill-cited-without-anchor`.
- **Judged by:** this slice, with unit tests of `path@core@src/manifest.rs` and of
  `milestone_refusal`.
- **Fires when:** a manifest declaring a Component directory or a location named after any kind
  loads with no phase-1 complaint naming it; or a plan so named is an anchor; or a declared register
  named `skill`, `agent`, `primer` or `instructions` is accepted; under either harness.
- **Response:** the refusal is repaired. If a name cannot be refused without breaking a consuming
  project, `thread@agent-configuration-entities@skill-cited-without-anchor` reopens with the owner.

### A branch that renames an installed section passes `commits` on every commit `##commits-judges-each-commit-by-its-own-copies`

AC2, applied on the owner's word in R4.

- **Guards:** `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`.
- **Judged by:** this slice, with a test in `path@core@tests/binary.rs` over the `History` project
  of the Fixtures section: commit 1 cites a section of an installed skill; commit 2 renames that
  section in the installed copies and repairs the citation.
- **Fires when:** `commits` over both reports a dangling reference on commit 1.
- **Response:** the source of the definitions is repaired. If the judged tree's copies cannot be
  read there, `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` reopens with the owner.

### The shipped set cites only sections it defines `##the-shipped-set-cites-only-its-own-sections`

AC3, applied on the owner's word in R4.

- **Guards:** `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`.
- **Judged by:** this slice, with a test of the core over `knowledge_architect_agent_skills::FILES`:
  every level-two heading outside a fence of each skill, agent and the primer carries a slug, unique
  within its file, and every reference of a harness kind in the shipped text resolves within the
  shipped set, with no `instructions` reference.
- **Fires when:** the test passes on a scratch copy of the set holding a reference to a missing
  section of a sibling skill, or fails on the real set.
- **Response:** the test is repaired; a real-set failure is repaired in content/.

### An adopting project learns the slug requirement from the setup skill and from the finding `##the-slug-requirement-reaches-an-adopter`

AC4, applied on the owner's word in R4, its scope per D6 of the milestone document.

- **Guards:** `thread@agent-configuration-entities@claude-md-sections`.
- **Judged by:** this slice, by reading the setup skill as installed, and with a binary test of the
  finding's text.
- **Fires when:** the installed setup skill does not state that every level-two heading of the root
  CLAUDE.md carries a slug, or, per D6, of a project skill or agent, or the finding on an unslugged
  heading names no repair.
- **Response:** the text is repaired.

## Harvest

At this slice's landing, under `knowledge-architect-decision-recording` and
`knowledge-architect-issue-tracking`. Each text the milestone document lists as referencing a head
this slice rewrites is read at the harvest, and edited where the rewrite makes its statement false.
The rows below name the texts known to change.

| what | home |
| --- | --- |
| `thread@agent-configuration-entities@skill-register`, `thread@agent-configuration-entities@claude-md-sections` | a new head in `path@core@docs/design.md`, §6, slug `skill-register`, with the readings of the harness's layout the audit made |
| `thread@agent-configuration-entities@skill-cited-without-anchor`, `thread@agent-configuration-entities@root-instructions-cited`, `thread@agent-configuration-entities@primer-sections` | a new head in `path@core@docs/design.md`, §3, slug `skill-cited-without-anchor`; `design@core@candidate-rule-and-retired-forms` and `design@core@a-slug-belongs-to-a-component` rewritten in place; the entry "A register reference with no anchor" of `path@core@docs/rejected-alternatives.md` gains that its first reason does not hold for a harness kind, per D3; `goal@core@relocation-is-one-manifest-edit` was reworded before the slice, on D4 |
| `thread@agent-configuration-entities@skill-sections-carry-slugs`, `thread@agent-configuration-entities@agent-sections-carry-slugs`, `thread@agent-configuration-entities@primer-sections` | a new head in `path@core@docs/design.md`, §3, slug `skill-sections-carry-slugs`; `design@core@an-entry-is-a-heading-at-the-register-level` rewritten in place |
| `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` | a new head in `path@core@docs/design.md`, §6, slug `installed-entities-from-the-tree`, the slug naming the approved source; `design@core@owned-namespace-check`, `design@core@phases-gate-the-report` and `design@core@anchors-are-components-and-locations` rewritten in place; `design@agent-skills@shipped-text-cites-no-entry` and `design@agent-skills@content-mirrors-the-install-layout` rewritten in place |
| the kinds and AC1 | `design@core@one-entity-table` rewritten, its title and its kind list; `design@core@registers-are-declared`, the four names refused as a register; `design@core@anchors-are-components-and-locations` and `design@core@a-plan-name-reads-as-nothing-else`, every kind name refused as an anchor's name, with the owner's argument |
| the restatements | `path@knowledge-architect@CLAUDE.md`: the kind list, the candidate rule and the anchor rule; `path@core@CLAUDE.md` and `path@agent-skills@CLAUDE.md` where they state the rewritten heads; the module docs of `path@core@src/manifest.rs` and `path@core@src/entity.rs` |
| `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor`, `thread@agent-configuration-entities@section-kind`, and the other losing alternatives of the milestone | judged for entries in `path@core@docs/rejected-alternatives.md` |
| `issue@core@installed-file-findings-belong-in-phase-four` | its premise "the installed files are outside the model" rewritten: they define entities, and no check reads their findings |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | its What narrowed: a reference of a harness kind in the shipped text is legitimate and judged by AC3 |
| `issue@core@tooling-for-project-skills` | narrowed in its Summary and its What: a project skill's name is checked against the id grammar, and its level-two headings against the section rule, both built by this slice; skills and agents are entities, so its remaining checks read the entity table. What stays open: the project's prefix, the routing table, the description field, and which installed skill a project skill adds to |
| the changelog | `Next release`. Under Migration: every level-two heading of the root CLAUDE.md and of each project skill and agent owes a slug, mock projects serving `claude` included; a Component, location or plan named after any kind, and a register named after a harness kind, is renamed. Under New features: the four harness kinds and the section references. Surfaces and classes: `manifest`, major, for the refused names, per `design@knowledge-architect@versioning-policy`, since a manifest that was valid stops being accepted; `checks`, minor, for the slug requirement and the new kinds; `agent-skills`, patch, for the slugs in the installed text |

This slice's spec leaves in the commit that completes this harvest.
