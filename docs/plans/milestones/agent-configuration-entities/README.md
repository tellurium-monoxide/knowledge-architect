# Agent configuration entities: skills, agents, the primer and the root instructions, cited by kind and section

## Status and audience

This is the milestone document of the work that makes a project's agent configuration part of the
checker's entity table. Under the `claude` harness, every skill and every agent becomes an entity
of a built-in kind, `skill` or `agent`, cited without an anchor. Each of its level-two sections
carries a slug and is cited by a third segment. The installed primer and the project's root
CLAUDE.md get the same section rule, under the kinds `primer` and `instructions`. A backticked
bare name of a skill or an agent becomes a finding. The work also replaces the paragraph numbers,
`§N`, by which the installed skills cite each other. It is written for a session that did not
witness the discussion.

- It leaves the repository in the commit that completes the harvest of its last slice.
- Where it and a design home disagree, the design home wins.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where a point below needs the owner's word and the owner is absent, the work does not proceed on
  that point.

The discussion ran in one session of Claude Code. Its transcript is the file of that session in the
harness's project directory,
`path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/`, found
by its opening message: "I would like to model skills and agent configuration of a project as part
of the checker." A second file of that directory opens with the same message and holds nothing
else. It is an aborted session, and it was not read. This document was assembled from the first
file. The rounds are numbered R1 to R5 by the owner's messages. R1 is the opening message, and R5
is "Agreed on the milestone."

**A quotation below that holds a form of reference which lost, or a host name, writes part of it as
a placeholder in angle brackets, and says so with `[placeholder]`.** Slice 1 makes `skill`, `agent`,
`primer` and `instructions` kinds. From then on, such a span written in full would be read as a
reference to nothing. The transcript holds each one verbatim.

## How the work is done

The procedure for each slice is `skill@knowledge-architect-planning@working-a-slice`, restated below from the
installed skill. That skill is its home; where the two disagree, the skill wins.

#### Working a slice, and the work of a spec (`skill@knowledge-architect-planning@working-a-slice`)

1. **Ground**: the Component's `CLAUDE.md`, its design home, its rejected alternatives; then the
   milestone document entire, then the slice's spec. The issues and the tripwires, of every anchor,
   are read at the audit.
2. **The design audit.** Read the slice's entry and every decided shape it depends on against the
   code as it stands and against the design homes. List every gap: a shape the code refutes, a
   passage of a specification the entry did not read, a name the entry uses that the code does not
   have, a consequence the entry did not see, and **a standing entry the slice's planned code bears on**:
   a tripwire whose firing condition, or a `deferred` issue whose trigger, the planned code meets,
   and an issue of any kind the slice's code touches, closes, makes worse or depends on. Dispatch the
   search as the description of `knowledge-architect-standing-entry-searcher` says, with the slice's
   spec and the milestone document as the work and, as seeds, the decisions the milestone document
   lists under "What is already decided" and the decisions and goals the slice's spec cites. Read
   whole, with `cargo klarch show`, every entry the search returns, and judge it against the slice;
   never act on an entry from the reason the search gave. A firing found at the audit is ruled before the code is written, where one
   found only by the review of the harvest reopens the harvest. Sort each gap:
   - **Applied in place.** The gap has one answer the document's decisions already imply, or is a
     choice among shapes the document rules out all but one of. Apply the answer in place, in the
     milestone document or the slice's spec, wherever the shape it changes is written. Commit the
     amended documents alone, with a subject of the shape `The <slice> design audit, applied in
     place: <n> gaps, none reopening a discussion`, and a message listing each finding: the gap, the
     answer and the decision it follows from. Earlier audits are found with
     `git log --grep='design audit'`.
     **An answer that widens or narrows a ruling of the owner, or adds an obligation to one, is a
     scope change even when it is the one answer the document implies.** It is applied with the
     others, and also listed in the milestone document as a default awaiting the owner, who rules
     on it at the audit; the implementation of that point does not start before the ruling.
   - **A change to the slices.** An audit that finds the work needs another slice, or a slice split
     or reordered, writes each new slice's spec with its harvest row, adds the slice to the
     implementation sequence and the order rationale, and moves into its spec the design it takes
     from other slices, so that each shape keeps one home. It changes a sequence the owner ruled on, so it is a scope
     change, listed and ruled as above.
   - **Load-bearing.** The gap is material, or is a choice between two shapes neither of which the
     document rules out, or needs a ruling the document marks as the owner's. Record it in the
     slice's spec as open at the audit, with the discriminating fact, stop the slice, and open a
     design session with the owner under `knowledge-architect-design`. **The session's depth
     follows the gap.**
     - A choice among shapes that can be stated in full, each with its consequence, is put to the
       owner in one message, with a default; several such gaps go in one question, each under a
       label, `Q<n>`, since each is written in place as a thread with its slug. The slice's
       documents exist, so the ruling is written in place in the milestone's documents by the rule
       of `skill@knowledge-architect-planning@spec-contents`, as a thread with the owner's words verbatim, like the audit's
       other answers, and the audit's commit lists it among its gaps. It owes no new review of the
       plan document, since it changes no decided shape.
     - A gap that defeats a reason, a premise or a criterion an approved thread rests on needs the
       full session. Its converged design goes into the milestone's documents by the rule of
       `skill@knowledge-architect-planning@spec-contents`, and owes the reviews of
       `skill@knowledge-architect-planning@plan-reviews`.

     The slice resumes from the ruling or the converged design.
3. **Claims, tests, implementation, gates, commits**, per the project's development procedure:
   `klarch-development` for the Rust source; for a file under `path@agent-skills@content/`,
   `knowledge-architect-agent-configuration` and the section "Editing an installed skill or agent"
   of `path@agent-skills@CLAUDE.md`; in as many commits as the session judges the work needs. The commits name
   how each claim's test was shown to fail against a wrong implementation, and say of any claim
   whose test cannot yet do so why not.
4. **Review before the merge**, per `knowledge-architect-review`. A repair is a further commit,
   or folded where that skill says.
5. **The report**: the landing commit reports on each acceptance criterion judged at this slice,
   by its identifier in plain text, beside a citation of the milestone document.
6. **The harvest**, per the harvest row of the slice's spec: the decisions and the losing
   alternatives under `knowledge-architect-decision-recording`, then the tripwires and the issues
   under `knowledge-architect-issue-tracking`. The row names what is judged; the tests of
   `knowledge-architect-decision-recording` decide whether each decision and each alternative
   earns an entry, and they govern where the two disagree: an item of the row the tests exclude is
   named in the harvest's commit, with the test it fails. A decision harvested from a thread takes
   the thread's slug, unless the slug misdescribes the approved decision: the entry then takes a
   slug that names it, and the slice's harvest row states the pair. A tripwire names the head that
   harvested its decision, so the head is written first. Where a design home is a directory, a new
   subdocument is linked from its README. **The harvest is reviewed before the merge**, per
   `knowledge-architect-review`, on the decision-record, routing and standing-state axes, and by
   the transcript reviewer where the transcripts are available: it writes the record those axes
   judge, so the review of point 4 cannot see it.
7. **The slice's spec leaves** in the commit that completes its harvest. What crosses slices stays
   in the milestone document, amended in place where the landing changed it.

## Names

| name | what it names |
| --- | --- |
| the harness | the agent harness a project serves, declared by `[agents] harness` in its manifest, per `design@core@agents-table`; the checker knows one, `claude` |
| the installed copies | the files `cargo klarch install-agent-skills` writes into a project's namespace under .claude: `.claude/skills/knowledge-architect-<name>/`, `.claude/agents/knowledge-architect-<name>.md` and `path@agent-config@knowledge-architect/PRIMER.md`, per `design@core@owned-namespace-check`; `owned_path` in `path@core@src/manifest.rs` decides membership |
| the shipped set | the text the agent-skills crate ships, `FILES` of `path@agent-skills@src/lib.rs`, generated by `path@agent-skills@build.rs` from `path@agent-skills@content/` |
| a project skill, a project agent | a skill or an agent of the project's own, outside the installer's namespace, such as `klarch-development` under `path@agent-config@skills/` |
| the root instructions | the project's root CLAUDE.md, in this repository `path@knowledge-architect@CLAUDE.md`; a scoped CLAUDE.md is not one |
| a section | a level-two heading of a skill's SKILL.md, of an agent's file, of the primer or of the root instructions, outside a fenced block, and the text under it |
| a harness kind | one of the four kinds this work adds: `skill`, `agent`, `primer`, `instructions` |
| the two-segment form | a reference `<kind>@<id>` with no anchor, legal for a harness kind alone |
| a section slug placeholder | the build placeholder by which a heading under content/ carries its slug without defining it in this repository, rendered into the slug by the build, per `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` |
| the bare-name lint | the finding on a backticked span that is exactly the name of a defined skill or agent, per `thread@agent-configuration-entities@bare-skill-name-reported` |
| `Survey::installed` | the field of `Survey` in `path@core@src/survey.rs` holding each installed copy with its text; `survey` fills it from the working tree, and `from_listing` from a commit's own listing and blobs under `commits` |
| `Entities::build` | the function of `path@core@src/entity.rs` that builds the entity table from the model and the anchors; it reads nothing of `Survey::installed` today |
| `candidate` | the function of `path@core@src/entity.rs` that applies the candidate rule and the segmentation to one backticked `@` span |

## What the work is

What exists at each site the work touches:

- **The reference grammar** takes exactly three non-empty segments for every kind but `path` and
  `planned`, per `design@core@candidate-rule-and-retired-forms`. `candidate` asks
  `Anchors::kind` for the head before it asks `Anchors::is_anchor_word`, so a kind wins over an
  anchor of the same name. `design@core@a-slug-belongs-to-a-component` refuses a reference that
  names no anchor. The first head states the arity with no reason. The second gives one for the
  anchor: "Two anchors may therefore each record an entity they call the same word". The commit
  that introduced the arity, the one titled "The entity table and the `<kind>@<anchor>@<id>`
  grammar", lists "two and four segments" only among the planted defects. **The rejected
  alternatives hold the argued case**: "A register reference with no anchor", in
  `path@core@docs/rejected-alternatives.md`, lost to `design@core@a-slug-belongs-to-a-component`
  "because the extracted tool cannot know which register is single-instance in a given project,
  and one three-part grammar serves every kind without a special case in the resolver or in the
  instructions". The discussion did not read that entry; it was put to the owner after this
  document's first reviews, under `thread@agent-configuration-entities@skill-cited-without-anchor`.
- **Ten registers are compiled in**, per `design@core@registers-are-declared`, and the kind set is
  their names with `path` and `planned`, per `design@core@one-entity-table`. In this repository
  one register's home is under .claude: the `issue` register of the location `agent-config`, at
  `path@agent-config@open-issues/`. No register's home is under `path@agent-config@skills/` or
  `path@agent-config@agents/`.
- **The installed copies are outside the walk** in every project, this one included, and only
  `check::agents` judges their bytes, per `design@core@owned-namespace-check`. `commits` does not
  compare them: `Inputs.installed` is empty under `commits`. `Survey::installed` already holds
  each installed copy with what reading it gave, `Outside::Text` for a file that reads as text,
  for a checkout and, through `read_tree` and `from_listing` in `path@core@src/cli/history.rs`, for
  each commit.
- **A project skill or agent is walked** like any document. Under .claude in this repository it
  belongs to the location `agent-config`. A slug on one of its headings is a misplaced definition
  today: the file is no register home.
- **content/ is in the walk.** Its skill directories are unprefixed, as in
  `path@agent-skills@content/skills/review/SKILL.md`, and its frontmatter `name:` fields already
  carry the prefixed name, as in `name: knowledge-architect-decision-recording`. The build adds the
  prefix to each skill directory and agent file, per
  `design@agent-skills@content-mirrors-the-install-layout`.
- **Skills cite each other's sections by number.** At origin/main, `grep -rc '§'` finds 65 lines in
  13 files of `path@agent-skills@content/`, and 29 lines in 9 walked files outside it: CLAUDE.md
  files, the agent-skills README and design home, three issues and two `klarch-` skills. The walk
  for the second count is `git ls-files '*.md' '*.rs'` minus the installed copies, the core's mock
  projects and the crates' changelog copies. This branch adds one more line, in
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`, and the plan documents'
  own lines. The section-citations slice re-takes the count.
- **Skills and agents are named by backticked bare names**: at origin/main, 205 spans matching
  `` `(knowledge-architect|klarch)-[a-z-]+` `` in 46 walked files, by `grep -ohE` over the walk
  above. 8 of them are crate names, `knowledge-architect-agent-skills` and
  `knowledge-architect-gates`, so 197 spans name a skill or an agent, in 45 files. 1 is
  `knowledge-architect-transcript-conformity-reviewer`, an agent that no longer exists, named in
  the released changelog section that records its rename.
- **The sections to slug.** A count of the lines opening with `## ` outside blocks fenced by
  three backticks or tildes, `awk` over each file, gives 130,
  over the 9 skills and 8 agents of content/, content/PRIMER.md, the root CLAUDE.md, and this
  repository's three `klarch-` skills and its agent `klarch-changelog-reviewer`.

Outside this work:

- **The checks over a project skill's structure**, its prefix and the routing table, stay
  `issue@core@tooling-for-project-skills`, per `thread@agent-configuration-entities@project-skill-structure-checks`.
- **Scoped CLAUDE.md files and the home of component contracts.** The owner announced a design
  session of its own, recorded in
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`, per `thread@agent-configuration-entities@claude-md-sections`.
- **Other harnesses**, `issue@core@configuration-for-several-agent-providers`. The kind
  `instructions` is named so that the move to AGENTS.md changes no reference, per
  `thread@agent-configuration-entities@root-instructions-cited`.
- **A check that the shipped text cites no entry of this repository**,
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. Slice 1 narrows what that issue
  covers and leaves it open.

## What is already decided

The decisions this work rests on and does not argue again:

- `design@core@one-entity-table`: one table and one resolver for every kind. The harness kinds are
  more entries of it.
- `design@core@an-entry-is-a-heading-at-the-register-level`: a heading register declares one
  level, and every heading at that level in its home owes a slug. The section rule of the entities
  slice is that rule at level two, over each section home.
- `design@core@phases-gate-the-report`: the entity table is phase 3, so the definitions from the
  installed copies are built there.
- `design@core@owned-namespace-check`: the installed copies stay byte-checked by `check` and
  outside the walk for references.
- `design@agent-skills@shipped-text-line-comments`: content/ is in the walk, and its comments are
  checked here.
- `design@agent-skills@a-reference-claims-a-revisit`: why a section reference is worth writing.
- `design@knowledge-architect@changelog-entries`: a released section's content never changes, and
  its structure may.

The recorded decisions the work reverses or rewrites, each with every text that
`cargo klarch show` lists as referencing it, and the slice that updates each:

| decision | change | referencing texts | updated by |
| --- | --- | --- | --- |
| `design@core@candidate-rule-and-retired-forms` | its arity sentence names the two-segment form of a harness kind; the four new heads become candidates | `path@knowledge-architect@CLAUDE.md` (2, its restatement of the candidate rule); `path@agent-config@skills/klarch-release/SKILL.md`; `path@core@docs/design.md` (5); `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`; `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`; `path@core@docs/rejected-alternatives.md` (3); `path@core@docs/tripwires.md` (2); `path@core@README.md`; `path@core@src/check/references.rs` | slice 1, and slice 3 for the bare-name lint |
| `design@core@a-slug-belongs-to-a-component` | its refusal of a reference that names no anchor exempts a harness kind, whose namespace is the harness's | `path@knowledge-architect@CLAUDE.md`; `path@agent-skills@docs/design.md`; `path@core@docs/design.md` (4); `issue@core@cross-project-references`; `path@core@docs/rejected-alternatives.md` (5); `path@core@src/check/references.rs`; `path@core@src/source/mod.rs`; `path@core@src/source/rs.rs` | slice 1 |
| `design@core@owned-namespace-check` | the installed copies are read for definitions, still outside the walk for references | `path@agent-skills@docs/design.md` (4); `issue@agent-skills@patching-an-installed-skill`; `path@core@docs/design.md` (4); `issue@core@installed-file-findings-belong-in-phase-four`, whose premise "the installed files are outside the model" stops holding; `path@core@docs/rejected-alternatives.md` (2); `path@core@README.md`; `path@core@src/agents.rs`; `path@core@src/check/agents.rs`; `path@core@src/check/mod.rs`; `path@core@src/cli/history.rs` (2); `path@core@src/cli/mod.rs`; `path@core@src/manifest.rs` (2); `path@core@src/mock_projects.rs`; `path@core@src/survey.rs`; `path@core@src/walk.rs`; `path@core@tests/binary.rs` (3); `path@knowledge-architect@docs/design.md` (2) | slice 1 |
| `design@core@registers-are-declared` and `design@core@one-entity-table` | the kind list grows by the four harness kinds, which, like `path` and `planned`, are no register, so the count of built-in registers stays ten; a declared register under one of their names is refused, per AC1 | `path@knowledge-architect@CLAUDE.md` ("Ten are built in"); `path@core@README.md` ("Ten registers are compiled in"); `path@core@docs/design.md`; `path@core@docs/rejected-alternatives.md` (2); `path@core@src/manifest.rs`; `path@agent-skills@docs/design.md`; `path@core@src/entity.rs` | slice 1 |
| `design@core@an-entry-is-a-heading-at-the-register-level` | its rule is applied at level two to the section homes: a skill's SKILL.md, an agent's file, the primer, the root instructions | `path@knowledge-architect@CLAUDE.md`; `path@agent-skills@docs/design.md`; `path@core@docs/design.md` (3); `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`; `path@core@docs/rejected-alternatives.md` (2); `path@core@README.md` | slice 1 |
| `design@core@agents-table` | under `claude`, the harness kinds exist; under an empty list, they do not | `path@core@docs/design.md` (3); `issue@core@a-checker-only-project-carries-the-workflow-skeleton`; `issue@core@a-home-for-developer-contracts-outside-agent-configuration`; `issue@core@configuration-for-several-agent-providers`; `path@core@docs/rejected-alternatives.md`; `path@core@README.md`; `path@core@src/agents.rs`; `path@core@src/check/agents.rs`; `path@core@src/manifest.rs` (4) | slice 1 |
| `design@agent-skills@shipped-text-cites-no-entry` | the shipped text may cite the entities of the shipped set: its skills, its agents, the primer, and their sections | `path@agent-config@skills/klarch-release/SKILL.md`; `path@agent-skills@CLAUDE.md` (2); `path@agent-skills@content/skills/setup/SKILL.md`; `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` (2) | slice 1 |
| `design@agent-skills@content-mirrors-the-install-layout` | the build also renders the section slug placeholders | `path@agent-skills@CLAUDE.md` | slice 1 |
| `design@knowledge-architect@changelog-entries` | rewriting a bare name of a skill or agent into a reference in a released section, and back into a bare name after its entity is deleted, is a change of structure | `path@knowledge-architect@CHANGELOG.md`; `path@agent-config@agents/klarch-changelog-reviewer.md`; `path@knowledge-architect@CLAUDE.md` (2); `path@agent-config@skills/klarch-release/SKILL.md`; `path@knowledge-architect@docs/design.md`; `issue@knowledge-architect@a-mechanical-changelog-check` (2) | slice 3 |
| `design@core@one-entity-table`, its title | "an entity with a kind, an anchor, an id and a definition site, held in one table built from the walk" no longer holds for a harness kind, which names no anchor and is partly defined from the installed copies, outside the walk | as in its row above | slice 1 |
| `design@core@phases-gate-the-report` | its chain, "the manifest feeds the walk, the walk the model, the model the table", gains the installed copies, which the survey of phase 2 reads and the table of phase 3 takes | `path@knowledge-architect@CLAUDE.md`; `path@core@CLAUDE.md`; `path@core@docs/design.md` (7); `issue@core@installed-file-findings-belong-in-phase-four`; `path@core@docs/rejected-alternatives.md` (4); `path@core@docs/tripwires.md`; `path@core@README.md` | slice 1 |
| `design@core@anchors-are-components-and-locations` | a location "owes the homes of the registers its row declares and nothing else"; a project skill under a location is a section home of a harness kind, which no anchor carries; and every kind name is refused as a Component's or a location's name, per D7 | `path@knowledge-architect@CLAUDE.md`; `path@core@docs/design.md` (3); `path@core@docs/rejected-alternatives.md`; `path@core@src/manifest.rs` | slice 1 |
| `design@core@a-plan-name-reads-as-nothing-else` | a plan's name is not a kind name either, per D7 | `path@core@docs/design.md`; `path@core@README.md`; `path@core@src/check/tree.rs` (2); `path@core@src/entity.rs`; `path@core@src/mock_projects.rs` | slice 1 |
| `design@agent-skills@plain-text-is-no-repair` | the bare name in a released changelog section, after its skill or agent is deleted, is stated as no unchecked form clearing a finding, on the owner's ruling in R3 | `path@knowledge-architect@CLAUDE.md`; `path@agent-skills@docs/design.md` (2); `path@agent-skills@docs/tripwires.md` (2); `path@core@docs/design.md`; `path@core@src/check/references.rs` | slice 3 |
| `design@agent-skills@a-past-sentence-is-rewritten` | a released changelog section is its exception: its content never changes, so a dangling skill or agent reference there goes back to the bare name | none outside the plans directory | slice 3 |
| `goal@core@relocation-is-one-manifest-edit` | its sentence "Every reference names its anchor" is contradicted by the two-segment form; reworded by the owner on D4: "Every reference to what the manifest places names its anchor" | `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`, which the rewording leaves true | done, in the commit of this branch titled "The owner rewords the core's relocation goal: a reference to what the manifest places names its anchor" |

The parenthesised count is the number of referencing lines `cargo klarch show` printed for that
file, taken in this session. "Updated by" names the slice whose harvest reads each referencing
text, and edits each one whose statement the change makes false; the slice's harvest rows name the
texts already known to change.

## Criteria

### Every record is cited in one readable grammar, the same in every project `##one-grammar`

Binding, from `goal@knowledge-architect@any-project-can-adopt-it`. Met by
`thread@agent-configuration-entities@skill-cited-without-anchor`, read as one grammar in which a kind whose namespace is global takes no
anchor. The owner's reading in R2, which the assistant then shared (`argument@agent-configuration-entities@a44`, `argument@agent-configuration-entities@a55`, `argument@agent-configuration-entities@a56`).

### A reference in the shipped text resolves in every project that installs it `##works-anywhere`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met by
`thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`: the entities of the installed text are defined from the
installed copies, which every project serving `claude` holds, and AC3 judges the shipped set.

### A renamed or removed section dangles every citation of it, and `show` lists them `##revisit-computed`

Binding, from `goal@core@records-reach-their-reader`. Met by `thread@agent-configuration-entities@skill-sections-carry-slugs`,
`thread@agent-configuration-entities@agent-sections-carry-slugs`, `thread@agent-configuration-entities@primer-sections`, `thread@agent-configuration-entities@claude-md-sections` and `thread@agent-configuration-entities@bare-skill-name-reported`.

### A required slug is structure the work needs `##needed-structure`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met: a slug is a name, not a
mapping between a unit of the work and a unit of its record, and the 94 lines of `§` citations are
the need (`argument@agent-configuration-entities@a26`).

### Every reference names an anchor `##names-an-anchor`

Binding as a presumption, from `design@core@a-slug-belongs-to-a-component`. Rebutted for the harness
kinds by `thread@agent-configuration-entities@skill-cited-without-anchor`: an anchor there would carry no information (`argument@agent-configuration-entities@a56`). The head is
rewritten at slice 1. The same sentence stood in `goal@core@relocation-is-one-manifest-edit`, a
binding criterion the discussion did not list; the owner reworded the goal on D4, so it binds every
reference to what the manifest places, which the harness kinds are not.

### The published checker carries no declaration that serves only this repository `##no-catering`

Weighed, from the owner's words recorded in `design@agent-skills@shipped-text-line-comments`: "I
don't want to cater too much to this use case". Met by the section slug placeholder of
`thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, which needs no manifest declaration.

### A citation stays short enough to read inline `##short-pointers`

Weighed, the assistant's, R1: agents read these files at every load. Met by
`thread@agent-configuration-entities@skill-cited-without-anchor`.

## Threads

### Skills and agents are entities of the checker under the `claude` harness `##skill-register`

- **Proposed:** the owner, R1.
- **States:** R1 new; R2 approved.
- **Arguments:** `argument@agent-configuration-entities@a6`, `argument@agent-configuration-entities@a7`, `argument@agent-configuration-entities@a8`, `argument@agent-configuration-entities@a9`.
- **Closed by:** R2: "We align on skill-register."
- **D1, ruled after the document's first commit:** a skill directory or an agent file whose name is
  outside the id grammar `[a-z0-9]+(-[a-z0-9]+)*` is a phase-2 finding, naming the file, since such
  a skill could not be cited (`argument@agent-configuration-entities@a59`). The assistant had said in R2 that "the register reports it".
  The owner: "Agreed on D1". D8, a kept installed copy under `harness = []` reporting its slugs, was
  ruled after the first reviews: "Agreed on all defaults, and on the D4 goal rewording".
- **Shape:** the decided design of the `entities` slice. **Harvest:** a new head in
  `path@core@docs/design.md`, slug `harness-kinds`, slice 1: the thread's name named the shape the owner ruled against in Q2, so the head takes a slug naming the decision, `thread@agent-configuration-entities@skill-register` → harness-kinds.

### A harness kind is cited without an anchor: `skill@<name>` and `skill@<name>@<section>` `##skill-cited-without-anchor`

- **Proposed:** the owner, R1.
- **States:** R1 new; R2 presumed-settled on the owner's conditional word, once the assistant's
  check found no major syntax conflict; R3 approved.
- **Arguments:** `argument@agent-configuration-entities@a3`, `argument@agent-configuration-entities@a10`, `argument@agent-configuration-entities@a11`, `argument@agent-configuration-entities@a12`, `argument@agent-configuration-entities@a13`, `argument@agent-configuration-entities@a14`, `argument@agent-configuration-entities@a15`, `argument@agent-configuration-entities@a21`, `argument@agent-configuration-entities@a44`, `argument@agent-configuration-entities@a45`, `argument@agent-configuration-entities@a55`, `argument@agent-configuration-entities@a56`, `argument@agent-configuration-entities@a57`, `argument@agent-configuration-entities@a58`, `argument@agent-configuration-entities@a59`, `argument@agent-configuration-entities@a60`, `argument@agent-configuration-entities@a61`.
- **Closed by:** R2: "Unless it causes a major syntax conflict, I'd still argue for
  skill-cited-without-anchor. The decision head that everything must have exactly three segments is
  overstated IMO. I see no real argument in favor of that, I think it was written like this as a
  description of what was done rather than what was decided." R3: "Agreed on
  skill-cited-without-anchor".
- **Shape:** the decided design of the `entities` slice. **Harvest:** a new head in
  `path@core@docs/design.md`, slug `harness-kinds-cited-without-anchor`, since the head covers every harness kind, `thread@agent-configuration-entities@skill-cited-without-anchor` → harness-kinds-cited-without-anchor, and the rewrites of
  `design@core@candidate-rule-and-retired-forms` and `design@core@a-slug-belongs-to-a-component`,
  slice 1.
- **Relations:** absorbs the kinds of `thread@agent-configuration-entities@primer-sections` and `thread@agent-configuration-entities@root-instructions-cited`, which take the
  same form. After the first reviews, the owner ruled D3, that the shape stands against the
  rejected alternative "A register reference with no anchor", and D4, the goal's rewording: "Agreed on all defaults, and on the D4 goal rewording".
  Guarded by AC1, which the owner widened to every kind after the first reviews, as D7
  records.

### The tool constructs an anchor for the agent configuration, and every reference keeps three segments `##skill-cited-in-a-constructed-anchor`

- **Proposed:** the assistant, R1.
- **States:** R1 new; R2 withdrawn.
- **Arguments:** `argument@agent-configuration-entities@a16`, `argument@agent-configuration-entities@a17`, `argument@agent-configuration-entities@a18`, `argument@agent-configuration-entities@a19`, `argument@agent-configuration-entities@a20`, `argument@agent-configuration-entities@a21`, `argument@agent-configuration-entities@a45`, `argument@agent-configuration-entities@a46`, `argument@agent-configuration-entities@a47`, `argument@agent-configuration-entities@a56`.
- **Closed by:** withdrawn by the assistant in R2, after the owner's argument in R2: "Under
  skill-cited-in-a-constructed-anchor, the <cfg> placeholder is just added noise, and the best fit
  being "agents" creates a weird stutter to refer to subagent definitions. I see no other word that
  fits." The defeating reason: the anchor carries no information, as
  `design@core@reserved-anchors` argues for `plans` (`argument@agent-configuration-entities@a56`).
- **Shape:** Losing alternatives. **Harvest:** judged at slice 1 for an entry in
  `path@core@docs/rejected-alternatives.md`.

### A section is cited `section@<skill>@<slug>`, a kind of its own `##section-kind`

- **Proposed:** the assistant, R1.
- **States:** R1 new; R2 withdrawn.
- **Arguments:** `argument@agent-configuration-entities@a4`, `argument@agent-configuration-entities@a22`, `argument@agent-configuration-entities@a23`, `argument@agent-configuration-entities@a48`, `argument@agent-configuration-entities@a61`.
- **Closed by:** withdrawn by the assistant in R2, after the owner's argument in R2: "section-kind:
  I think your argument for it still stand, even if we end up taking the two segment syntax for
  skills and agents. However, the cost is that it loses the information that it is a section from a
  skill or agent. I thought this was quite important." The defeating reason: under the two-segment
  form, the third segment narrows within the same kind, as `path@<anchor>@<dir>/` and
  `path@<anchor>@<dir>/<file>` do, so the kind keeps one meaning (`argument@agent-configuration-entities@a61`).
- **Shape:** Losing alternatives. **Harvest:** judged at slice 1 for an entry in
  `path@core@docs/rejected-alternatives.md`.

### Every level-two heading of a skill owes a slug `##skill-sections-carry-slugs`

- **Proposed:** the owner, R1.
- **States:** R1 new; R2 approved.
- **Arguments:** `argument@agent-configuration-entities@a1`, `argument@agent-configuration-entities@a24`, `argument@agent-configuration-entities@a25`, `argument@agent-configuration-entities@a26`, `argument@agent-configuration-entities@a27`, `argument@agent-configuration-entities@a49`.
- **Closed by:** R2: "skill-sections-carry-slugs: approved. The consequence you stated is accepted
  and positive in my view. This is the fragility I mentionned at the beginning with using paragrap
  numbers."
- **Conditions:** a section slug of an installed skill is an interface. An upgrade that renames one
  dangles a consumer's citations of it, and each such rename owes a Migration entry in the
  changelog. The owner accepted this as intended.
- **Shape:** the decided design of the `entities` slice. **Harvest:** a new head in
  `path@core@docs/design.md`, slug `section-homes-carry-slugs`, since the head binds every section home, `thread@agent-configuration-entities@skill-sections-carry-slugs` → section-homes-carry-slugs, recording the section rule for
  every harness kind, and the rewrite of `design@core@an-entry-is-a-heading-at-the-register-level`,
  slice 1.

### An agent's level-two headings owe slugs too `##agent-sections-carry-slugs`

- **Proposed:** the assistant, R1, answering the owner's doubt in R1, "(not sure paragraph form is
  needed here too)".
- **States:** R1 new; R2 approved.
- **Arguments:** `argument@agent-configuration-entities@a2`, `argument@agent-configuration-entities@a28`, `argument@agent-configuration-entities@a29`.
- **Closed by:** R2: "agent-sections-carry-slugs approved."
- **Shape:** the decided design of the `entities` slice. **Harvest:** the head of
  `thread@agent-configuration-entities@skill-sections-carry-slugs`, slice 1.

### The installed text is defined from the tree's installed copies, and content/ carries its slugs as build placeholders `##installed-skills-defined-from-shipped-set`

- **Proposed:** the assistant, R1.
- **States:** R1 new, with two shapes for content/, (a) a manifest declaration and (b) build
  placeholders; R2 approved with (b), its definitions taken from the binary's shipped set; R3 a
  material finding found at the premortem, proposing the tree's installed copies as the source; R4
  approved on that source.
- **Arguments:** `argument@agent-configuration-entities@a5`, `argument@agent-configuration-entities@a30`, `argument@agent-configuration-entities@a31`, `argument@agent-configuration-entities@a32`, `argument@agent-configuration-entities@a33`, `argument@agent-configuration-entities@a34`, `argument@agent-configuration-entities@a35`, `argument@agent-configuration-entities@a70`, `argument@agent-configuration-entities@a72`, `argument@agent-configuration-entities@a73`, `argument@agent-configuration-entities@a74`, `argument@agent-configuration-entities@a75`.
- **Closed by:** R2: "installed-skills-defined-from-shipped-set: (b) looks right here IMO." R4:
  "installed-skills-defined-from-shipped-set approved. Though the problem you mention is only
  bearing on the current project, and not on external consumers, I believe."
- **The premise of the owner's remark.** In R4 the assistant said it would check the remark, and
  showed no answer in that round. The answer was put to the owner after this document's first
  reviews: `design@core@installed-binary-version-check` says of `commits` that it "compares no
  historical value, so moving the pin fails no earlier commit", so the tip's binary judges every
  earlier commit of a branch. In a consumer, a branch whose first commit cites an installed section
  on the old pin, and whose second commit moves the pin, installs and repairs the citation, would
  fail on its first commit if the definitions came from the binary. The problem therefore reaches
  consumers too. The owner ruled the default of D2, that the approval stands: "Agreed on all
  defaults, and on the D4 goal rewording".
- **The slug:** it names the R2 shape, "shipped set". The approved decision is "from the tree's
  installed copies", so the harvested head takes the slug `installed-entities-from-the-tree`, per
  point 6 of `skill@knowledge-architect-planning@working-a-slice`.
- **Shape:** the decided design of the `entities` slice. **Harvest:** a new head in
  `path@core@docs/design.md`, slug `installed-entities-from-the-tree`; the rewrites of
  `design@core@owned-namespace-check`, `design@agent-skills@shipped-text-cites-no-entry` and
  `design@agent-skills@content-mirrors-the-install-layout`, slice 1.

### The numbering of skill sections is removed `##section-numbers-dropped`

- **Proposed:** the assistant, R1.
- **States:** R1 new; R2 approved.
- **Arguments:** `argument@agent-configuration-entities@a36`.
- **Closed by:** R2: "section-numbers-dropped: agreed." After the first reviews, D5 extended it to
  the agents and this repository's `klarch-` files: "Agreed on all defaults, and on the D4 goal rewording".
- **Shape:** the `section-citations` slice. **Harvest:** none: a rewording of the installed text,
  per `design@agent-skills@instruction-record-is-minimal`; the commit records it.

### A backticked bare name of a defined skill or agent is a finding `##bare-skill-name-reported`

- **Proposed:** the assistant, R1.
- **States:** R1 new, argued to no position; R2 approved, with the owner's ruling on released
  changelog sections, and a consequence put to the owner with a default; R3 the default approved.
- **Arguments:** `argument@agent-configuration-entities@a37`, `argument@agent-configuration-entities@a38`, `argument@agent-configuration-entities@a39`, `argument@agent-configuration-entities@a40`, `argument@agent-configuration-entities@a50`, `argument@agent-configuration-entities@a53`, `argument@agent-configuration-entities@a54`, `argument@agent-configuration-entities@a71`, `argument@agent-configuration-entities@a77`.
- **Closed by:** R2: "bare-skill-name-reported: approved. For the changelog problem: older changelog
  section allow structural changes, and this passes as a structural change in my view." R3: "Agreed
  on the repair in a released changelog section citing a skill/agent to be replaced by bare text on
  deletion, this does not count as evasion, because there is nothing to cite that the checker would
  accept."
- **Shape:** the decided design of the `bare-names` slice. **Harvest:** a new head in
  `path@core@docs/design.md`, slug `bare-skill-name-reported`, and the rewrite of
  `design@knowledge-architect@changelog-entries`, slice 3.

### The root CLAUDE.md is modelled, and no scoped CLAUDE.md `##claude-md-sections`

- **Proposed:** the assistant, R1, as "not modelled now"; the owner moved it to the root CLAUDE.md,
  R2.
- **States:** R1 new; R2 approved on the owner's position.
- **Arguments:** `argument@agent-configuration-entities@a41`, `argument@agent-configuration-entities@a42`, `argument@agent-configuration-entities@a51`, `argument@agent-configuration-entities@a52`, `argument@agent-configuration-entities@a62`, `argument@agent-configuration-entities@a63`.
- **Closed by:** R2: "claude-md-sections: I'd model only root CLAUDE.md sections." and "Under this
  ruling, modeling only the root CLAUDE.md (which would stay) is acceptable, and probably a good
  thing to do."
- **Conditions:** every project serving `claude` adds a slug to every level-two heading of its root
  CLAUDE.md: one Migration entry, minor. After the first reviews, D6 extended AC4 to project skills
  and agents: "Agreed on all defaults, and on the D4 goal rewording".
- **The owner's direction on scoped files, R2**, recorded in
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration` in the commit of this
  branch titled "Record the owner's direction on scoped CLAUDE.md files in the developer-contracts
  issue": "after investigation, I found that delivery of scoped CLAUDE.md and AGENTS.md files
  is very unreliable. In the future, I'm going to remove any recommendation for them, and change the
  contract home. This would be its own session."
- **Shape:** the decided design of the `entities` slice. **Harvest:** the heads of
  `thread@agent-configuration-entities@skill-register` and `thread@agent-configuration-entities@skill-sections-carry-slugs`, slice 1.

### A section of the root instructions is cited `instructions@<slug>` `##root-instructions-cited`

- **Proposed:** the assistant, R2.
- **States:** R2 new; R3 approved.
- **Arguments:** `argument@agent-configuration-entities@a64`, `argument@agent-configuration-entities@a65`, `argument@agent-configuration-entities@a66`, `argument@agent-configuration-entities@a67`.
- **Closed by:** R3: "root-instructions-cited: agreed with instructions."
- **Shape:** the decided design of the `entities` slice. **Harvest:** the head of
  `thread@agent-configuration-entities@skill-cited-without-anchor`, slice 1.

### The primer's sections carry slugs, cited `primer@<slug>` `##primer-sections`

- **Proposed:** the assistant, R2.
- **States:** R2 new; R3 approved.
- **Arguments:** `argument@agent-configuration-entities@a68`, `argument@agent-configuration-entities@a69`, `argument@agent-configuration-entities@a70`.
- **Closed by:** R3: "primer-sections: good idea, approved."
- **Shape:** the decided design of the `entities` slice. **Harvest:** the heads of
  `thread@agent-configuration-entities@skill-cited-without-anchor` and `thread@agent-configuration-entities@skill-sections-carry-slugs`, slice 1.

### The checks over a project skill's structure are not bundled `##project-skill-structure-checks`

- **Proposed:** the assistant, R1.
- **States:** R1 new; R2 approved.
- **Arguments:** `argument@agent-configuration-entities@a9`, `argument@agent-configuration-entities@a43`.
- **Closed by:** R2: "project-skill-structure-checks: agreed."
- **Shape:** What the work is, its exclusions. **Harvest:** none; `issue@core@tooling-for-project-skills`
  stays open, and slice 1's harvest notes in it that skills and agents are entities.
## Arguments

Each argument's key words are quoted from the transcript. A quotation that a form of reference
would make live writes it with a placeholder, marked `[placeholder]`, as Status and audience says.

### Paragraph numbers go stale; slugs let the published skills cite each other `##a1`

R1, owner; bears on `thread@agent-configuration-entities@skill-sections-carry-slugs`, `thread@agent-configuration-entities@skill-cited-without-anchor`.

> Better than paragraph numbers, which might get stale at any time. Use this in the published skills to refer between each other.

### Agents get the same reference form, and their sections may not need it `##a2`

R1, owner; bears on `thread@agent-configuration-entities@agent-sections-carry-slugs`.

> same for subagent definitions: `agent@<agent-name>` (not sure paragraph form is needed here too).

### The grammar has no two-segment form, so the proposed whole-skill form is a grammar change `##a3`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor`.

> F1. The grammar has no two-segment form." ... "So `skill@<skill-name>` as proposed is a grammar change.

### The three-segment section form already parses, as a plan item does `##a4`

R1, assistant; bears on `thread@agent-configuration-entities@section-kind`, `thread@agent-configuration-entities@skill-cited-without-anchor`.

> `skill@<skill-name>@<section-slug>` already parses today: kind `skill`, anchor `<skill-name>`, id `<section-slug>`. That is the plan-item precedent

### The installed skills are outside the walk, so their entities need a source every project has `##a5`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`.

> F2. The installed skills are outside the walk in every project, this one included." ... "So the entities of an installed skill cannot be defined from the walk. They have to come from something every project has.

### The harness fixes the layout, so one table and one resolver cover skills with nothing declared `##a6`

R1, assistant; bears on `thread@agent-configuration-entities@skill-register`.

> the harness already fixes the layout, so nothing is declared. One entity table and one resolver cover them

### The registers' homes would be the harness's directories, not a Component's docs/ `##a7`

R1, assistant; bears on `thread@agent-configuration-entities@skill-register` (cost).

> Two built-in registers whose homes are the harness's directories, not a Component's `docs/`.

### The harness directories cannot owe a README and an index `##a8`

R1, assistant; bears on `thread@agent-configuration-entities@skill-register` (cost).

> They cannot owe a README and an index the way a file register does. A `README.md` in `path@agent-config@agents/` may be read by the harness as an agent definition. That is an assumption I have not checked. [the anchor added]

### Entities turn the prefix and routing checks into checks over entities `##a9`

R1, assistant; bears on `thread@agent-configuration-entities@skill-register`, `thread@agent-configuration-entities@project-skill-structure-checks`.

> the checks that `issue@core@tooling-for-project-skills` asks for (prefix, routing table) become checks over entities instead of path guesses.

### The two-segment form is the shortest `##a10`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (for).

> It is the shortest form. `` `skill@knowledge-architect-review` `` is 34 characters, against 26 for the bare name.

Correction, from the review of this document: the 34 counts the backticks and the 26 does not. It is
32 against 26 without them.

### A skill name is already unique, as a directory name `##a11`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (for).

> A skill name is already unique: the harness makes it a directory name.

### The anchor serves disambiguation and relocation, and neither applies to skills `##a12`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (for).

> The anchor position exists for disambiguation and relocation (`goal@core@relocation-is-one-manifest-edit`). The harness fixes where skills live, so neither applies.

### Under the two-segment form, the arity of a span changes what its kind means `##a13`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (against).

> It breaks F1. The arity of a span changes what the kind means: two segments name a skill, three name a section.

### The resolver, the link pass and the extension view gain a special case `##a14`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (against).

> The resolver gains a special case, and so do the future link pass and the extension view.

### The two-segment form fails the presumption that every reference names an anchor `##a15`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (against).

> It fails #names-an-anchor, a presumption. You can rebut it, but the rebuttal has to be argued.

### A constructed anchor named skills or agents stutters `##a16`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (cost).

> `skills` and `agents` stutter: `skill@skills@<name>` [placeholder].

### A constructed anchor named after a provider would cite one skill two ways `##a17`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (cost).

> The anchor must not be named after a provider. Otherwise a skill served to two harnesses would be cited two ways, against `issue@core@configuration-for-several-agent-providers`.

### A constructed anchor at .claude collides with this repository's agent-config location `##a18`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (cost).

> An anchor at `.claude/` itself collides with this repository's declared `agent-config` location: "two anchors at one path" is refused. ... There are 6 references to `agent-config` today.

Correction, from the review of this document: one of the 6 spans is the placeholder
`path@agent-config@<file>`, which the scanner does not record, so there are 5 references.

### A constructed anchor keeps the grammar and copies the plans precedent `##a19`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (for).

> it keeps F1 and #names-an-anchor whole, and copies a built precedent: a plans anchor plus one anchor per spec.

### A constructed anchor lengthens the whole-skill citation `##a20`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (against).

> the whole-skill citation is longer. `` `skill@harness@<name>` `` [placeholder] is 42 characters, against 34.

### The three-segment form needs no new grammar case `##a21`

R1, assistant; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` over `thread@agent-configuration-entities@skill-cited-without-anchor`.

> The discriminating fact is that the three-segment form needs no new grammar case, and it keeps the section form you proposed nearly as written.

### A distinct section kind keeps one meaning per kind `##a22`

R1, assistant; bears on `thread@agent-configuration-entities@section-kind` (for).

> Under your form, the kind `skill` means "a skill" at one anchor and "a section" at another. ... A distinct kind keeps one meaning per kind.

### The word section would become a candidate head `##a23`

R1, assistant; bears on `thread@agent-configuration-entities@section-kind` (cost).

> the word `section` becomes a candidate head. A span like `` `section@<word>` `` [placeholder] in prose would start being read. That is the kind of change `tripwire@core@candidate-rule-silence` watches.

### The section rule reuses the rule that every heading at the level owes a slug `##a24`

R1, assistant; bears on `thread@agent-configuration-entities@skill-sections-carry-slugs`.

> It uses the existing rule: "every heading at the register's level in its home carries a slug". ... Level-3 subsections stay section text.

### Illustrative headings sit in fences or use placeholders, so they define nothing `##a25`

R1, assistant; bears on `thread@agent-configuration-entities@skill-sections-carry-slugs`.

> Illustrative headings in skills ... sit in fences or use placeholders, so they define nothing.

### A slug is a name, not imposed structure, and the section-number citations are the need `##a26`

R1, assistant; bears on `thread@agent-configuration-entities@skill-sections-carry-slugs` (`criterion@agent-configuration-entities@needed-structure`).

> a slug is a name, not a mapping between a unit of the work and a unit of its record. The 94 lines of `§` citations are the need, so I judge that it passes.

### A section slug of an installed skill becomes an interface consumers depend on `##a27`

R1, assistant; bears on `thread@agent-configuration-entities@skill-sections-carry-slugs` (consequence).

> a section slug of an installed skill becomes an interface. A consumer's own skill that cites it dangles when an upgrade renames it. That is #revisit-computed working as intended. Each such rename also owes a Migration entry in the changelog.

### Agents already cite their own sections and are cited by section `##a28`

R1, assistant; bears on `thread@agent-configuration-entities@agent-sections-carry-slugs`.

> The transcript reviewer cites its own §2 and §4. The cold-implementer reviewer cites its own §1. `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review` cites "its section 2" of the standing-state reviewer, in words.

### The agent sections cost nothing once skills have the register `##a29`

R1, assistant; bears on `thread@agent-configuration-entities@agent-sections-carry-slugs`.

> It is the same register shape, so the added cost is close to zero once skills have it.

### Installed entities from the binary's shipped set, project entities from the walk `##a30`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`.

> Installed skills and agents are defined from the binary's shipped set, in every project. The owned-namespace check already guarantees that the installed copies match it." / "A project's own skills, such as `klarch-*`, are defined from the walk.

### A slug heading in content/ would be a misplaced definition here `##a31`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`.

> A `##<slug>` [placeholder] heading in `path@agent-skills@content/skills/review/SKILL.md` [the anchor added] sits in "a file that is no register home", so it is a misplaced-definition finding.

### A manifest declaration for content/ fails the criterion against catering `##a32`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, option (a).

> A manifest declaration that a directory is the source of a shipped skill set. It fails #no-catering, unless other projects that publish skills would use it too.

### Build placeholders let content/ define nothing, and a core test judges the shipped set `##a33`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, option (b).

> content/ writes each section slug as a build placeholder. The build renders the placeholder into the slug, so content/ defines nothing. A core test then judges the shipped set

### The placeholders make a sibling reference in the shipped text legitimate and checked `##a34`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, option (b).

> (b) also closes part of `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: inside the shipped set, a reference to a sibling becomes legitimate and checked.

### The placeholders' cost is that content/ shows a placeholder where the installed copy shows the slug `##a35`

R1, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, option (b) cost.

> My lean: (b), on #no-catering. Its cost is that content/ shows a placeholder where the installed copy shows the slug.

### A section number is a second name that goes stale on every insertion `##a36`

R1, assistant; bears on `thread@agent-configuration-entities@section-numbers-dropped`.

> Once sections carry slugs, a number is a second name for the section, and it goes stale on every insertion. Rival: keep the numbers for reading order. I see no fact that favours keeping them.

### Without the lint, a renamed skill dangles every bare name unseen `##a37`

R1, assistant; bears on `thread@agent-configuration-entities@bare-skill-name-reported` (for).

> without it, the 197 existing name spans stay silent. A rename then dangles them unseen, which is `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` for this class.

### An exact-name match reports no crate name, and prefixed names meet no ordinary word `##a38`

R1, assistant; bears on `thread@agent-configuration-entities@bare-skill-name-reported` (false positives).

> an exact match against defined names reports none of the 8 crate-name spans. Names carry a prefix, so a collision with an ordinary word cannot happen.

### Every skill citation grows by 8 to 16 characters `##a39`

R1, assistant; bears on `thread@agent-configuration-entities@bare-skill-name-reported` (against).

> Every citation of a skill grows by 8 to 16 characters.

Correction, from the review of this document: rewriting a bare name into its reference adds the
kind and one `@`, 6 characters for `skill` and `agent`.

### A released changelog section naming an existing skill would be reported `##a40`

R1, assistant; bears on `thread@agent-configuration-entities@bare-skill-name-reported` (against).

> A released changelog section that names a removed agent would be reported, and that section's content never changes. ... The risk is a released section naming a skill that still exists.

### CLAUDE.md holds restatements, and the citations into it are internal `##a41`

R1, assistant; bears on `thread@agent-configuration-entities@claude-md-sections`.

> CLAUDE.md holds restatements with their pointers, and its decisions have other homes. The citations into it I saw are internal, such as "`## Git` point 2".

### Modelling CLAUDE.md now would pre-empt the discussion of the contract home `##a42`

R1, assistant; bears on `thread@agent-configuration-entities@claude-md-sections`.

> Issue `issue@core@a-home-for-developer-contracts-outside-agent-configuration` is about what CLAUDE.md holds, so modelling it now would pre-empt that discussion.

### The project skill checks depend on the register and are not bundled `##a43`

R1, assistant; bears on `thread@agent-configuration-entities@project-skill-structure-checks`.

> They depend on #skill-register. I propose not bundling them, so this design stays one change to the grammar and the registers.

### The three-segment rule records what was done, not what was decided `##a44`

R2, owner; bears on `thread@agent-configuration-entities@skill-cited-without-anchor`.

> The decision head that everything must have exactly three segments is overstated IMO. I see no real argument in favor of that, I think it was written like this as a description of what was done rather than what was decided.

### The constructed anchor is noise, and its best name stutters `##a45`

R2, owner; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (against).

> the <cfg> placeholder is just added noise, and the best fit being "agents" creates a weird stutter to refer to subagent definitions. I see no other word that fits.

### An empty anchor segment would avoid breaking the grammar, and reads oddly `##a46`

R2, owner; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (variant).

> To avoid breaking, we could use the form `skill@@<name>` [placeholder] instead, I suppose (your form with empty <cfg>). Though for now it is rejected too I think, and reads a bit weird compared to others.

### An anchor named workflow is acceptable `##a47`

R2, owner; bears on `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor` (variant).

> `skill@workflow@<name>` [placeholder] is acceptable to me too.

### A section kind loses the information that a section belongs to a skill or an agent `##a48`

R2, owner; bears on `thread@agent-configuration-entities@section-kind` (against).

> I think your argument for it still stand, even if we end up taking the two segment syntax for skills and agents. However, the cost is that it loses the information that it is a section from a skill or agent. I thought this was quite important.

### Section renames dangling citations is the fragility the owner meant to fix `##a49`

R2, owner; bears on `thread@agent-configuration-entities@skill-sections-carry-slugs`.

> The consequence you stated is accepted and positive in my view. This is the fragility I mentionned at the beginning with using paragrap numbers.

### Rewriting a released changelog section into references is a change of structure `##a50`

R2, owner; bears on `thread@agent-configuration-entities@bare-skill-name-reported`.

> For the changelog problem: older changelog section allow structural changes, and this passes as a structural change in my view.

### Scoped CLAUDE.md and AGENTS.md files are delivered unreliably, and their recommendation goes `##a51`

R2, owner; bears on `thread@agent-configuration-entities@claude-md-sections`.

> after investigation, I found that delivery of scoped CLAUDE.md and AGENTS.md files is very unreliable. In the future, I'm going to remove any recommendation for them, and change the contract home.

### Modelling only the root CLAUDE.md is acceptable and probably good `##a52`

R2, owner; bears on `thread@agent-configuration-entities@claude-md-sections`.

> Under this ruling, modeling only the root CLAUDE.md (which would stay) is acceptable, and probably a good thing to do.

### A deleted skill dangles a released changelog section that cites it `##a53`

R2, assistant; bears on `thread@agent-configuration-entities@bare-skill-name-reported` (consequence).

> A released section that cites a skill then dangles when that skill is later removed or renamed. Two of today's rules then disagree on the repair: `design@agent-skills@a-past-sentence-is-rewritten` asks for the sentence to be rewritten. ... Turning the reference back into a bare name is what `tripwire@agent-skills@plain-text-pointer-found` watches for.

### The repair in a released section is the bare name again `##a54`

R2, assistant; bears on `thread@agent-configuration-entities@bare-skill-name-reported`.

> Default, unless you say otherwise: in a released section, the repair is to write the bare name again. Once the skill no longer exists, the bare name names nothing, so the lint is silent. I count this as structural under the same ruling.

### The arity rule is stated without a reason, in the head and in its commit `##a55`

R2, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor`.

> The head states the arity as a fact, with no reason ... The commit that introduced it ... lists "two and four segments" only as planted defects, again with no reason.

### One namespace per harness means an anchor carries no information, as for plans `##a56`

R2, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor`, `thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor`.

> a skill or agent has one namespace per harness, which the harness enforces as directory and file names. An anchor there would carry nothing. The record makes this exact argument for `plans` in `design@core@reserved-anchors` ... So your form does not defeat an argued reason. It defeats a description.

### A kind shadows a Component or location of the same name `##a57`

R2, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (conflict 1).

> A kind wins over an anchor in the head position. ... So a Component or location named `skill` or `agent` is shadowed. The same is true today for one named `design`. I found no check that refuses an anchor name equal to a kind.

### A two-segment span with a new head becomes a dangling candidate `##a58`

R2, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (conflict 2).

> A two-segment span headed `skill@<rest>` or `agent@<rest>` [placeholder], such as `` `agent@<host>` `` [placeholder], becomes a reference, and a dangling one. I expect this to be rare. Not measured.

### A skill name outside the id grammar cannot be cited `##a59`

R2, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor` (conflict 3).

> A skill directory name outside `[a-z0-9]+(-[a-z0-9]+)*` cannot be cited. The register reports it. I believe Claude Code restricts skill names to lowercase letters, digits and hyphens, but that is from memory and not verified here.

### A grammar change touches an interface others use, so the premortem runs `##a60`

R2, assistant; bears on `thread@agent-configuration-entities@skill-cited-without-anchor`.

> This changes an interface that other people already use: the reference grammar. So the premortem will run before convergence.

### Under the two-segment form, the third segment narrows within one kind, as a path does `##a61`

R2, assistant; bears on `thread@agent-configuration-entities@section-kind` (withdrawal).

> With the two-segment form, `skill@<name>@<section>` reads the way `path@<anchor>@<dir>/` and `path@<anchor>@<dir>/<file>` [placeholder] read: the third segment narrows within the same kind. My objection was that one kind would mean two things. That no longer holds, because the whole and the part are the same family.

### Every consuming project slugs its root CLAUDE.md headings `##a62`

R2, assistant; bears on `thread@agent-configuration-entities@claude-md-sections` (consequence).

> Consumers migrate. Every project serving `claude` adds a slug to every level-2 heading of its root CLAUDE.md. That is a stricter check, so a minor class, and one Migration entry.

### Two citations of a root CLAUDE.md section exist today `##a63`

R2, assistant; bears on `thread@agent-configuration-entities@claude-md-sections` (value).

> The value here is small today. I found 2 citations of a root CLAUDE.md section, both of `## Git`

### One root CLAUDE.md per project, so its sections take the two-segment form `##a64`

R2, assistant; bears on `thread@agent-configuration-entities@root-instructions-cited`.

> A project has one root CLAUDE.md, so the anchor would carry nothing here either. The form is two segments: `<kind>@<section-slug>`.

### The kind instructions names the document's role and survives the move to AGENTS.md `##a65`

R2, assistant; bears on `thread@agent-configuration-entities@root-instructions-cited` (for `instructions`).

> It names the document's role. When the contract-home session or `issue@core@configuration-for-several-agent-providers` replaces CLAUDE.md with AGENTS.md, no reference changes.

### The kind claude-md is recognisable and tied to one harness `##a66`

R2, assistant; bears on `thread@agent-configuration-entities@root-instructions-cited` (rival `claude-md`).

> It is immediately recognisable, and it is tied to one harness's file name.

### The announced AGENTS.md move decides for instructions `##a67`

R2, assistant; bears on `thread@agent-configuration-entities@root-instructions-cited`.

> My position: `instructions`. The discriminating fact is the AGENTS.md move you have already announced.

### The primer has seven sections and few citations `##a68`

R2, assistant; bears on `thread@agent-configuration-entities@primer-sections` (rival).

> Rival: leave the primer unmodelled. It has 7 level-2 sections and few citations.

### Primer sections cost nothing more and give restatements a checked home `##a69`

R2, assistant; bears on `thread@agent-configuration-entities@primer-sections` (for).

> the cost is near zero once the shipped-set mechanism exists. And the primer is what the root CLAUDE.md restates most, so a restatement could point at its home in checked form, per `design@agent-skills@a-reference-claims-a-revisit`.

### The primer is in the shipped set and is defined like the skills `##a70`

R2, assistant; bears on `thread@agent-configuration-entities@primer-sections`.

> The primer is in the shipped set, so it is defined like the installed skills, by (b).

### A bare name after deletion is no evasion: nothing the checker accepts remains to cite `##a71`

R3, owner; bears on `thread@agent-configuration-entities@bare-skill-name-reported`.

> this does not count as evasion, because there is nothing to cite that the checker would accept.

### commits judges old commits with the tip's binary, so definitions from the binary fail them `##a72`

R3, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` (material finding).

> `commits` judges every commit of a range with the tip's binary. `design@core@owned-namespace-check` already refuses to compare installed files there, for this reason: "the running binary ships its own version's text, and an older commit's installed set would fail against it". Defining entities from the binary's shipped set brings that failure back through references.

### A section rename fails earlier commits that cite it, here and in a consumer `##a73`

R3, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` (concrete case).

> a branch renames a section slug of the review skill in its third commit. Commits 1 and 2 hold texts that cite the old slug. Under the tip's binary, those citations dangle ... The same happens in a consumer project: a branch that moves its pin fails `commits` on every commit before the upgrade commit that cites an installed section.

### Definitions come from the judged tree's installed copies `##a74`

R3, assistant; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` (reopening).

> Installed skills, agents and the primer are defined from the installed copies the judged tree holds, under .claude. They are read for definitions only. They stay outside the walk for references" / "`check` already guarantees those copies equal the pinned shipped set." / "In this repository, an edit of content/ is installed in the same commit, so each commit's copies match that commit's content/.

### The owner reads the problem as bearing on this repository only `##a75`

R4, owner; bears on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`.

> Though the problem you mention is only bearing on the current project, and not on external consumers, I believe.

### The slug requirement and its content must land in one commit, and the migrations are large `##a76`

R4, assistant; bears on plan shape (milestone).

> The work cannot be one pull request without one oversized commit. The slug requirement and the content that satisfies it must land in the same commit, because every commit must pass under the tip's checker. And the two migrations, `§N` citations and bare names, touch about 46 files each.

Correction, from the review of this document: the `§N` migration touched 22 files at origin/main,
13 in content/ and 9 outside it; the bare-name migration 45.

### The later slices need the kinds, and the lint comes after the citation rewrite `##a77`

R4, assistant; bears on plan shape (order), `thread@agent-configuration-entities@bare-skill-name-reported`.

> slice 2 and slice 3 each need slice 1's kinds. Slice 3 comes last because its lint would report every name that slice 2's rewrite has not yet touched.

### The milestone document changes the per-commit gate, so it merges first `##a78`

R4, assistant; bears on plan shape (landing).

> the milestone document changes what the per-commit gate checks, so it merges in a pull request of its own before slice 1.

### One spec would mix the core change with about 290 mechanical lines `##a79`

R4, assistant; bears on plan shape (against one spec).

> A review would then read the core change mixed with roughly 290 mechanical lines, and a failure in the mechanism could not be told from a failure in the migration.
## New names, in one place

```text
crates/core/src/entity.rs                 the kinds skill, agent, primer and instructions, answered by
                                          Anchors::kind beside path and planned (slice 1)
                                          a variant of Candidate for the anchorless reference, beside
                                          Candidate::Reference, holding the kind, the name and the slug (slice 1)
crates/core/src/model.rs                  Model::set_installed and Model::installed: the parsed installed
                                          copies, walked by nothing (slice 1)
                                          milestone_refusal refuses every kind name (slice 1, AC1)
crates/core/src/manifest.rs               every kind name refused as an anchor's name, and the four harness
                                          kinds as a register's name (slice 1, AC1)
crates/core/tests/binary.rs               a helper of History that serves claude and installs the shipped set (slice 1)
crates/core/src/scan.rs                   an observation of a backticked span with no @ and no /, which the
                                          bare-name lint reads (slice 3)
crates/agent-skills/build.rs              the section slug placeholder, written {{slug:<id>}} at the end of a
                                          heading, rendered into a backticked ##<id> (slice 1)
crates/core/src/check/references.rs       the bare-name lint (slice 3)
crates/core/docs/design.md                design slugs, written at the harvests:
                                          harness-kinds                     (slice 1, from #skill-register, #claude-md-sections)
                                          harness-kinds-cited-without-anchor (slice 1, from #skill-cited-without-anchor, #root-instructions-cited, #primer-sections)
                                          section-homes-carry-slugs         (slice 1, from #skill-sections-carry-slugs, #agent-sections-carry-slugs, #primer-sections)
                                          installed-entities-from-the-tree  (slice 1, from #installed-skills-defined-from-shipped-set)
                                          bare-skill-name-reported          (slice 3)
```

The placeholder's spelling and the module of each check are defaults of this document; slice 1's
audit may change them, as an answer applied in place.

## Decided design

The entities slice has landed. The design of `thread@agent-configuration-entities@skill-register`,
`thread@agent-configuration-entities@claude-md-sections` and `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` is in
`design@core@harness-kinds` and `design@core@installed-entities-from-the-tree`; that of
`thread@agent-configuration-entities@skill-cited-without-anchor`, `thread@agent-configuration-entities@root-instructions-cited` and `thread@agent-configuration-entities@primer-sections` in
`design@core@harness-kinds-cited-without-anchor`; that of `thread@agent-configuration-entities@skill-sections-carry-slugs` and
`thread@agent-configuration-entities@agent-sections-carry-slugs` in `design@core@section-homes-carry-slugs`.
The section-citations slice has landed: `thread@agent-configuration-entities@section-numbers-dropped` earns no head, a rewording of
the installed text per `design@agent-skills@instruction-record-is-minimal`.
[bare-names](bare-names.md) holds the design of `thread@agent-configuration-entities@bare-skill-name-reported`.

## Mapping tables

In the slices' specs.

## Losing alternatives

- **`thread@agent-configuration-entities@skill-cited-in-a-constructed-anchor`**, lost to `thread@agent-configuration-entities@skill-cited-without-anchor`. The tool would
  construct one anchor for the agent configuration and keep three segments everywhere, as
  `skill@<cfg>@<name>` [placeholder]. Every candidate name for the anchor stutters or names a
  provider (`argument@agent-configuration-entities@a16`, `argument@agent-configuration-entities@a17`, `argument@agent-configuration-entities@a45`). An anchor at .claude collides with this repository's `agent-config`
  location (`argument@agent-configuration-entities@a18`). The deciding fact: a skill or an agent has one namespace per harness, so the
  anchor carries no information, the argument `design@core@reserved-anchors` makes for `plans`
  (`argument@agent-configuration-entities@a56`). The owner's variants, an empty anchor segment (`argument@agent-configuration-entities@a46`) and the anchor `workflow` (`argument@agent-configuration-entities@a47`), fall
  with it.
- **`thread@agent-configuration-entities@section-kind`**, lost to `thread@agent-configuration-entities@skill-cited-without-anchor`. A section would be `section@<skill>@<slug>`
  [placeholder]. It loses the information that the section belongs to a skill or to an agent (`argument@agent-configuration-entities@a48`).
  The deciding fact: under the two-segment form, the third segment narrows within one kind, as a
  path does, so one kind keeps one meaning (`argument@agent-configuration-entities@a61`).
- **A manifest declaration of content/ as the source of a shipped set**, shape (a) of
  `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, lost to the build placeholders, shape (b): it fails
  `criterion@agent-configuration-entities@no-catering` (`argument@agent-configuration-entities@a32`).
- **Definitions from the binary's shipped set**, the R2 shape of
  `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`, lost to definitions from the tree's installed copies:
  `commits` judges every commit of a range with the tip's binary, so an earlier commit citing a
  section the tip renamed would fail (`argument@agent-configuration-entities@a72`, `argument@agent-configuration-entities@a73`).
- **Scoped CLAUDE.md files**, rejected for `thread@agent-configuration-entities@claude-md-sections`: the owner will remove the
  recommendation for them (`argument@agent-configuration-entities@a51`).
- **The kind name `claude-md`**, lost to `instructions` under `thread@agent-configuration-entities@root-instructions-cited`: it is tied
  to one harness's file name, and the owner announced a move to AGENTS.md (`argument@agent-configuration-entities@a66`, `argument@agent-configuration-entities@a67`).

Judged at the entities slice's harvest by the tests of `knowledge-architect-decision-recording`: the
constructed anchor and the section kind each earned an entry in
`path@core@docs/rejected-alternatives.md`, by its first test, since each would change the reference
grammar, a format every project writes. The manifest declaration of content/, the binary's shipped
set as the source, scoped CLAUDE.md files and the kind name `claude-md` earned none: each lost to an
argument a reader derives again, and the first two are argued in the heads that beat them.

## Readings

The work reads the layout the `claude` harness gives an agent configuration: a skill is a directory
under `path@agent-config@skills/` holding `SKILL.md` with `name` and `description` frontmatter, and an agent
is a Markdown file under `path@agent-config@agents/`. The installer already writes that layout, per
`design@core@owned-namespace-check`. The entities slice's audit read the harness's documentation,
https://code.claude.com/docs/en/skills.md and https://code.claude.com/docs/en/sub-agents.md, and
records what it read in `design@core@harness-kinds`: a skill is invoked by its directory's name; an agent's
identity is its frontmatter `name`, the agents directory is scanned recursively, and a file there
with no `name` is documentation; an agent's `name` is at most 256 characters, holds no `:` and does
not start with `-`; a skill's name defers to the Agent Skills specification. The name grammar the
owner ruled in D1 is stricter than either, and stands as the checker's own requirement.

## Premortem

Assume the work shipped and failed. Each cause, the thread it stresses, and its verdict. The owner
ruled in R4: "Keep none of the tripwires." and "All four AC are applied."

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| T1 | The four new heads turn spans that were never pointers into dangling references, such as `agent@<host>` [placeholder] | `thread@agent-configuration-entities@skill-cited-without-anchor` | proposed as a tripwire, not recorded on the owner's word |
| T2 | The bare-name lint reports a span that is a name and not a pointer, such as a skill named in an example command line | `thread@agent-configuration-entities@bare-skill-name-reported` | proposed as a tripwire, not recorded on the owner's word |
| T3 | Section slugs of the installed text churn, so every upgrade costs consumers a round of repairs; proposed to fire when one release's changelog holds more than 3 Migration entries for renamed or removed sections | `thread@agent-configuration-entities@skill-sections-carry-slugs`, `thread@agent-configuration-entities@primer-sections` | proposed as a tripwire, not recorded on the owner's word |
| AC1 | A kind shadows an anchor of the same name in the head position | `thread@agent-configuration-entities@skill-cited-without-anchor` | an acceptance criterion of the entities slice, applied on the owner's word, and widened to every kind by the owner's ruling recorded under D7 |
| AC2 | Definitions taken from anything but the judged tree fail `commits` on an earlier commit | `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` | an acceptance criterion of the entities slice, applied on the owner's word |
| AC3 | The shipped text cites a section that does not exist | `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set` | an acceptance criterion of the entities slice, applied on the owner's word |
| AC4 | Adopting projects meet the root CLAUDE.md slug requirement as an unexplained finding | `thread@agent-configuration-entities@claude-md-sections` | an acceptance criterion of the entities slice, applied on the owner's word |

## Acceptance criteria

AC1 to AC4 were judged by the entities slice, and reported in the commit that landed its harvest: none fired. The later slices judge none.

## Implementation sequence

1. Entities, landed: the four harness kinds and the two-segment form; definitions from the
   tree's installed copies and from the walk; the section rule at level two, with a slug on
   every level-two heading it covers, the section slug placeholder in content/; AC1 to AC4.
2. Section citations, landed: every `§N` citation of a skill's or an agent's section
   rewritten as a reference; the numbering of skill and agent sections removed.
3. [Bare names](bare-names.md): the bare-name lint, and every backticked bare name of a skill or an
   agent rewritten as a reference, released changelog sections included.

## Order rationale

Section citations comes after entities because a section reference resolves only once the kinds and
the section slugs exist; bare names comes after section citations because its lint would report
every bare skill name that the citation rewrite leaves beside a `§N` it has not yet rewritten, and
the two rewrites touch many of the same sentences (`argument@agent-configuration-entities@a77`).

## Defaults awaiting the owner

None awaits the owner. D1 was ruled by the owner after this document's first commit, "Agreed on D1", and is written
into `thread@agent-configuration-entities@skill-register`. The first reviews of this document produced D2 to D8. The owner ruled D7 while
its repairs were written, and the rest after them: "Agreed on all defaults, and on the D4 goal
rewording". Each ruling is applied in the sections it touches; the list stays as their record:

- **D2, ruled as its default**, on `thread@agent-configuration-entities@installed-skills-defined-from-shipped-set`: the corrected premise of the owner's
  remark in R4, `argument@agent-configuration-entities@a75`. The remark reads the problem as bearing on this repository only. It reaches
  consumers too, as the thread's item states. Default: the approval stands, since it covers both.
- **D3, ruled as its default**, on `thread@agent-configuration-entities@skill-cited-without-anchor`: the rejected alternative "A register reference with no
  anchor", which the discussion did not read, and on whose absence the owner's R2 word rested ("I
  see no real argument in favor of that"). Its first reason does not hold for a harness kind; its
  second is the cost `argument@agent-configuration-entities@a14`, which the owner weighed. Default: the decided shape stands, and the entry
  is amended at slice 1's harvest.
- **D4, ruled as its draft**, on `thread@agent-configuration-entities@skill-cited-without-anchor` and the criterion `criterion@agent-configuration-entities@names-an-anchor`:
  `goal@core@relocation-is-one-manifest-edit` states "Every reference names its anchor", which the
  two-segment form contradicts by its letter. Its met condition, "such a move leaves no reference
  to repair", still holds: nothing a harness kind names moves with a manifest edit. A goal binds,
  and changes only on the owner's word, under `knowledge-architect-goal-setting`. Default, as a
  draft for the owner's ruling: "Every reference to what the manifest places names its anchor, and
  each anchor's directory is declared once, in the manifest, so moving a Component or a location
  changes the manifest and no document." Written into the goals home on the owner's word.
- **D5, ruled as its default**, on `thread@agent-configuration-entities@section-numbers-dropped`: the owner ruled on removing "the "1." numbering of skill
  sections". The section-citations slice also removes it from the agents, and from this
  repository's `klarch-` skills and agent. Default: both, since every one of them carries a slug.
- **D6, ruled as its default**, on `thread@agent-configuration-entities@claude-md-sections` and AC4: AC4 was applied as the setup skill stating the root
  CLAUDE.md's slug requirement. The entities slice also states it for a project skill or agent,
  which `thread@agent-configuration-entities@skill-sections-carry-slugs` and `thread@agent-configuration-entities@agent-sections-carry-slugs` bind too. Default: both.
- **D7, ruled**, on `thread@agent-configuration-entities@skill-cited-without-anchor` and AC1: AC1 was applied to the four harness kinds.
  The review found that the same shadowing reaches every existing kind, and a plan's name too. The
  owner ruled, after the first reviews: "My ruling on this is that all kind names should be refused
  for anything that can be an anchor name (components, custom locations...). I'd rather make this
  decision early to avoid painful migrations." So AC1 refuses every kind name as the name of a
  Component, a location or a plan: `path`, `planned`, the ten built-in registers, the four harness
  kinds whatever the harness, and every register the manifest declares. The two readings of the ruling were put to the owner with the defaults: a plan is "anything that can be an anchor name", and a declared register's name is a kind name. The owner's word on the list that showed them, "Agreed on all defaults, and on the D4 goal rewording", confirms both. The refusals make a manifest that was valid stop being accepted,
  a major change on the `manifest` surface.
- **D8, ruled as its default**, on `thread@agent-configuration-entities@skill-register`: under `harness = []`, a project that kept its installed copies walks
  them as its own documents, and their slugs are misplaced definitions there. Default: the findings
  stand; the repair is to remove the copies or to serve the harness.
- **D9, ruled as its default**, on `thread@agent-configuration-entities@skill-register`, from Q1 of the entities slice's audit: how a skill's or an agent's
  frontmatter `name` is read for the match the owner ruled. Default: the `name:` line alone, so a
  frontmatter the harness reads but the subset refuses, a list or a multi-line description, is no
  finding; a skill may omit `name`; an agent file with no `name` is no agent; an agent may sit in a
  subdirectory.
- **D10, ruled as its default**, on `thread@agent-configuration-entities@skill-sections-carry-slugs`, from the review of the entities slice: an installed copy
  defines its entities and raises no finding. A consuming project whose branch upgrades after other
  commits holds, under `commits`, an earlier version's set with no slugs, and every heading of it
  would be a finding inside a file the project may not edit. The shipped set is held to the section
  rule in this repository, by AC3. This narrows the ruling that every level-two heading of a skill
  owes a slug to the files a project can repair. Default: the narrowing, as built.

The owner ruled D9 and D10 after the review of the entities slice: "D9 approved. D10 approved."

## Harvest

This document's own row: it is deleted in the commit that completes the bare-names slice's harvest.
Each slice's row is in its spec.

## Later consequences

- **`issue@core@tooling-for-project-skills`** can check a project skill's prefix and the routing
  table over entities rather than paths, once slice 1 lands.
- **The contract-home session** the owner announced, recorded in
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`, decides the home of a
  component's contracts. The `instructions` kind is unaffected, since the root CLAUDE.md stays.
- **A second harness**, `issue@core@configuration-for-several-agent-providers`, would give the
  harness kinds a second layout to read. The kind names name no provider, so no reference changes.
- **A link pass**, `issue@core@the-documents-do-not-render-as-a-site`, has to give the two-segment
  form a link target: the installed copy's file, and the heading for a section.
