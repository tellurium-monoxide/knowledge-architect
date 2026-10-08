# Entities: the harness kinds, their definitions from the tree, and the section register

The spec of the first slice of `milestone@plans@agent-configuration-entities`. It holds what only
this slice builds; what crosses slices is in the milestone document. Every item it cites is defined
there, except its own acceptance criteria. It starts after the milestone document has merged.

## Builds

- **The four harness kinds** in `path@core@src/entity.rs`, `skill`, `agent`, `primer` and
  `instructions`, each a built-in register that exists under the `claude` harness alone.
- **The two-segment form** in `candidate`: a harness kind takes the arities of the mapping table
  below, and any other arity is a malformed reference whose repair names the forms.
- **The definitions**, built in `Entities::build`: an installed skill, agent or primer, and its
  sections, from the text of `Survey::installed`; a project skill or agent, and the root
  instructions, and their sections, from the walk.
- **The section register**: a heading register at level two whose homes are each skill's
  `SKILL.md`, each agent's file, the primer and the root instructions. Every level-two heading
  outside a fenced block there owes a slug. A slug defined there is no longer a misplaced
  definition.
- **AC1**: the four kind names refused as a declared register's name and as a Component's or a
  location's name, in phase 1, as `planned` is refused as a register name.
- **A name outside the id grammar**: a skill directory or an agent file whose name is outside
  `[a-z0-9]+(-[a-z0-9]+)*` is a phase-2 finding, naming the file, since such a skill could not be
  cited. This was D1, ruled by the owner after the first review dispatch: "Agreed on D1".
- **The section slug placeholder** in `path@agent-skills@build.rs`: a level-two heading of a file
  under content/ ends with `{{slug:<id>}}`, which the build renders into a backticked `##<id>`. A
  placeholder anywhere else fails the build, as an unused substitution row does.
- **The slugs**: a slug on each of the 130 level-two headings the milestone document counts. Under
  content/ and in its primer as placeholders, and directly in the root CLAUDE.md, in the three
  `klarch-` skills and in the agent `klarch-changelog-reviewer`. Each slug names its section's
  subject. The section numbers stay until the section-citations slice.
- **The installed copies** under .claude, by `cargo klarch install-agent-skills`, in the same commit
  as each change to content/.
- **The core test over the shipped set**, AC3.
- **The setup skill's text**, AC4: `path@agent-skills@content/skills/setup/SKILL.md` states that
  every level-two heading of the root CLAUDE.md and of a project skill or agent carries a slug, and
  the finding's text names that repair.
- **The changelog entries** of this slice, in the `Next release` section, then `cargo x changelog`.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| `skill@<name>` resolves to an installed skill, a project skill, or to nothing with a finding naming the skill | a unit test of `path@core@src/entity.rs` over a model holding one installed and one project skill | a mutation that drops the installed copies from `Entities::build` makes the installed case dangle |
| A section reference resolves only to a slug defined in that skill's own file | a unit test citing a slug of skill A through skill B | a resolver that looks the slug up across every skill passes the wrong case, and the test fails |
| Every arity the mapping table does not list is malformed, with the forms in the repair | a unit test per arity, for each kind | a segmentation that accepts any arity for a harness kind passes a four-segment span |
| Under `harness = []`, a span headed by a harness kind is silent | a binary test over the `minimal` mock project, which declares `harness = []`, with such a span planted in a copy | registering the kinds whatever the harness reports the span |
| A level-two heading of a section home with no slug is a finding; a slug at another level there is misplaced | a binary test over a copy of a mock project serving `claude`, as `Sandbox::serve_claude` builds one in `path@core@tests/binary.rs` | a register declared at level three reports neither case |
| The installed copies define entities and stay outside the walk for references | a unit test planting a dangling reference in an installed copy | walking the installed copies reports the planted reference |
| AC1 to AC4 | each criterion's test, below | each criterion's own discrimination |
| The tree passes with every slug in place | `cargo klarch check` and `cargo klarch commits origin/main..HEAD` | the run before the slugs land reports one finding per unslugged heading: 130 is the expected count |

## Audit subjects

- The milestone document entire, and this spec.
- `path@core@src/entity.rs`: `candidate`, `Anchors::kind`, `Anchors::is_anchor_word`,
  `Entities::build`.
- `path@core@src/survey.rs`: `Survey::installed`, `survey`, `from_listing`, and where the run
  builds the survey relative to `Entities::build` in `path@core@src/check/mod.rs`.
- `path@core@src/manifest.rs`: `owned_path`, `OWNED_PREFIX`, and the refusal of a register named
  `planned`.
- `path@agent-skills@build.rs` and `path@agent-skills@src/render.rs`: the comment strip and the
  substitutions, beside which the placeholder goes.
- The core's test `every_shipped_file_is_installed_in_the_owned_namespace` in
  `path@core@src/agents.rs`, beside which AC3's test goes.
- `design@core@candidate-rule-and-retired-forms`, `design@core@a-slug-belongs-to-a-component`,
  `design@core@owned-namespace-check`, `design@core@registers-are-declared`,
  `design@core@one-entity-table`, `design@core@an-entry-is-a-heading-at-the-register-level`,
  `design@core@agents-table`, `design@core@installed-binary-version-check`,
  `design@agent-skills@shipped-text-cites-no-entry`,
  `design@agent-skills@content-mirrors-the-install-layout`.
- The harness's documentation of skills and agents, for the reading the milestone's Readings
  section leaves not established, and for **whether the harness names a skill by its directory or
  by its frontmatter `name`**. This spec takes the directory name as the id. Where the two can
  differ, the audit decides whether a mismatch is a finding.
- The standing entries the milestone's grounding returned:
  `issue@core@installed-file-findings-belong-in-phase-four`, whose premise this slice changes;
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`;
  `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`, since a section home
  opens with a frontmatter block; `tripwire@core@candidate-rule-silence`;
  `tripwire@core@extension-defines-a-kind`; `tripwire@core@phases-gate-the-report-two`.

## Fails alone on

- A reference of a harness kind that names an existing section is reported dangling.
- `cargo klarch commits` fails on an earlier commit of a branch that renames a section and repairs
  its citations in a later commit.

## Premises that expire

- **The section numbers stay in the headings**, as in `## 1. Does it reverse something already
  recorded?`, until the section-citations slice removes them. The `§N` citations stay valid prose
  meanwhile.
- **No span headed by `skill`, `agent`, `primer` or `instructions` exists outside the milestone's
  documents**: a grep of the walk and of the messages on origin/main found none when this spec was
  written. The run of `cargo klarch check` at this slice reports any that has appeared since.

## Decided design

### #skill-register and #claude-md-sections: four kinds, under the `claude` harness

| kind | an entity | defined from | cited |
| --- | --- | --- | --- |
| `skill` | a directory under `path@agent-config@skills/` holding `SKILL.md`, its id the directory's name | the installed copies for a name under the installer's prefix, the walk otherwise | `skill@<name>` |
| `agent` | a Markdown file directly under `path@agent-config@agents/`, its id the file's basename | the same | `agent@<name>` |
| `primer` | the primer, `path@agent-config@knowledge-architect/PRIMER.md`; only its sections are entities | the installed copies | `primer@<slug>` |
| `instructions` | the root CLAUDE.md; only its sections are entities | the walk | `instructions@<slug>` |

- **Nothing is declared** (a6): the harness fixes the layout. Under `harness = []` none of the four
  kinds exists, and a span headed by one is silent, as any span whose head is no kind and no anchor.
- **No README and no index** are owed under `path@agent-config@skills/` or `path@agent-config@agents/` (a8): the harness
  reads every file there.
- **A scoped CLAUDE.md is not modelled** (a51, a52). The root CLAUDE.md stays, so its sections are.
- **Nearest rival:** no entities, with skills cited by path, as today. It leaves every citation of a
  skill unchecked, which is the need the owner named in R1 (a1).

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
  enforces as directory and file names, so an anchor carries no information (a56). The primer and
  the root instructions are one document each in a project (a64).
- **The third segment narrows within the kind** (a61), so a section keeps the information that it
  belongs to a skill or to an agent (a48).
- **The kind is asked first**, as `candidate` does today, so a Component or a location named after
  a harness kind would be shadowed (a57). AC1 refuses such a name.
- **`instructions` names the document's role** (a65), so the announced move to AGENTS.md changes no
  reference.
- **The heads it rewrites**: `design@core@candidate-rule-and-retired-forms` states the arities of a
  harness kind beside "every kind but `path` and `planned` takes exactly three non-empty segments";
  `design@core@a-slug-belongs-to-a-component` states that a harness kind names no anchor because its
  namespace is the harness's.
- **Nearest rival:** #skill-cited-in-a-constructed-anchor, see the milestone's Losing alternatives.

### #skill-sections-carry-slugs, #agent-sections-carry-slugs and #primer-sections: the section register

- **A heading register at level two.** Its homes are each skill's `SKILL.md`, each agent's file,
  the primer and the root instructions. The rule is `design@core@an-entry-is-a-heading-at-the-register-level`
  unchanged: every level-two heading of a home outside a fence owes a slug, and a slug elsewhere in
  it is misplaced (a24). An id is unique within its one document.
- **Fenced headings define nothing** (a25), so the illustrations of an issue entry's shape in the
  issue-tracking skill, or of a design entry in the decision-recording skill, owe no slug.
- **A section slug of the installed text is an interface** (a27, a49): a rename dangles a
  consumer's citations, and owes a Migration entry in the changelog.

### #installed-skills-defined-from-shipped-set: definitions from the judged tree, slugs as placeholders in content/

- **The source.** `Entities::build` takes the texts of `Survey::installed` and defines from them the
  installed skills, agents and the primer, and their sections. Under `commits`, `from_listing`
  gives each commit's own copies, so each commit is judged against its own installed set (a72, a73,
  a74). `check` already guarantees that the copies equal the pinned shipped set, per
  `design@core@owned-namespace-check`.
- **Still outside the walk for references.** A reference inside an installed copy is not read, as
  today. The shipped text's references are judged in this repository, in content/, and by AC3.
- **content/ defines nothing** (a31, a33). A heading there ends with `{{slug:<id>}}`, and the build
  renders it. So the slug a reference in content/ resolves to is the one in the installed copy,
  which this repository installs in the same commit as each change to content/.
- **`design@agent-skills@shipped-text-cites-no-entry` is rewritten**: the shipped text may cite the
  shipped set's own skills, agents, primer and their sections, which every project serving `claude`
  holds, and never `instructions`, whose slugs are each project's own (a34).
- **Nearest rival:** the binary's shipped set as the source, which fails `commits` on an earlier
  commit; and a manifest declaration of content/, which fails #no-catering. See the milestone's
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
| an installed skill, agent, the primer | the installed copies under .claude, which match content/ in every commit | the installed copies | the commit's own installed copies |
| a project skill or agent | the walk, under the location `agent-config` | the walk | the commit's walk |
| the root instructions | the walk | the walk | the commit's walk |

## Losing alternatives

In the milestone document: every losing alternative of this work crosses its threads.

## Acceptance criteria

### A declared name that a harness kind would shadow is refused `##a-kind-name-is-refused-as-a-name`

AC1, applied on the owner's word in R4.

- **Guards:** #skill-cited-without-anchor.
- **Judged by:** this slice, with unit tests of `path@core@src/manifest.rs`.
- **Fires when:** a manifest declaring a register, a Component directory or a location named
  `skill`, `agent`, `primer` or `instructions` loads with no phase-1 complaint naming it.
- **Response:** the refusal is repaired. If a name cannot be refused without breaking a consuming
  project, #skill-cited-without-anchor reopens with the owner.

### A branch that renames an installed section passes `commits` on every commit `##commits-judges-each-commit-by-its-own-copies`

AC2, applied on the owner's word in R4.

- **Guards:** #installed-skills-defined-from-shipped-set.
- **Judged by:** this slice, with a test in `path@core@tests/binary.rs` built on `History`: commit 1
  cites a section of an installed skill; commit 2 renames that section in the installed copies and
  repairs the citation.
- **Fires when:** `commits` over both reports a dangling reference on commit 1.
- **Response:** the source of the definitions is repaired. If the judged tree's copies cannot be
  read there, #installed-skills-defined-from-shipped-set reopens with the owner.

### The shipped set cites only sections it defines `##the-shipped-set-cites-only-its-own-sections`

AC3, applied on the owner's word in R4.

- **Guards:** #installed-skills-defined-from-shipped-set.
- **Judged by:** this slice, with a test of the core over `knowledge_architect_agent_skills::FILES`:
  every level-two heading outside a fence of each skill, agent and the primer carries a slug, unique
  within its file, and every reference of a harness kind in the shipped text resolves within the
  shipped set, with no `instructions` reference.
- **Fires when:** the test passes on a scratch copy of the set holding a reference to a missing
  section of a sibling skill, or fails on the real set.
- **Response:** the test is repaired; a real-set failure is repaired in content/.

### An adopting project learns the slug requirement from the setup skill and from the finding `##the-slug-requirement-reaches-an-adopter`

AC4, applied on the owner's word in R4.

- **Guards:** #claude-md-sections.
- **Judged by:** this slice, by reading the setup skill as installed, and with a binary test of the
  finding's text.
- **Fires when:** the installed setup skill does not state that every level-two heading of the root
  CLAUDE.md and of a project skill or agent carries a slug, or the finding on an unslugged heading
  names no repair.
- **Response:** the text is repaired.

## Harvest

At this slice's landing, under `knowledge-architect-decision-recording` and
`knowledge-architect-issue-tracking`:

| what | home |
| --- | --- |
| #skill-register, #claude-md-sections | a new head in `path@core@docs/design.md`, §6, slug `skill-register` |
| #skill-cited-without-anchor, #root-instructions-cited, #primer-sections | a new head in `path@core@docs/design.md`, §3, slug `skill-cited-without-anchor`; `design@core@candidate-rule-and-retired-forms` and `design@core@a-slug-belongs-to-a-component` rewritten in place |
| #skill-sections-carry-slugs, #agent-sections-carry-slugs, #primer-sections | a new head in `path@core@docs/design.md`, §3, slug `skill-sections-carry-slugs`; `design@core@an-entry-is-a-heading-at-the-register-level` rewritten in place |
| #installed-skills-defined-from-shipped-set | a new head in `path@core@docs/design.md`, §6, slug `installed-entities-from-the-tree`, the slug naming the approved source; `design@core@owned-namespace-check` rewritten in place; `design@agent-skills@shipped-text-cites-no-entry` and `design@agent-skills@content-mirrors-the-install-layout` rewritten in place |
| AC1 | `design@core@registers-are-declared` and `design@core@one-entity-table` rewritten: the count and the kind list, and the four names refused |
| the restatements | `path@knowledge-architect@CLAUDE.md`: "Ten are built in", the candidate rule and the anchor rule; `path@core@README.md`, "Ten registers are compiled in"; `path@core@CLAUDE.md` and `path@agent-skills@CLAUDE.md` where they state the rewritten heads |
| #skill-cited-in-a-constructed-anchor, #section-kind, and the other losing alternatives of the milestone | judged for entries in `path@core@docs/rejected-alternatives.md` |
| `issue@core@installed-file-findings-belong-in-phase-four` | its premise "the installed files are outside the model" rewritten: they define entities, and no check reads their findings |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | its What narrowed: a reference of a harness kind in the shipped text is legitimate and judged by AC3 |
| `issue@core@tooling-for-project-skills` | a sentence in its What: skills and agents are entities, so its checks read the entity table |
| the changelog | `Next release`: under Migration, every level-two heading of the root CLAUDE.md and of each project skill and agent owes a slug, mock projects serving `claude` included; a register, Component or location named after a harness kind is renamed. Under New features, the four harness kinds and the section references. Surfaces `checks` and `agent-skills`, class minor |

This slice's spec leaves in the commit that completes this harvest.
