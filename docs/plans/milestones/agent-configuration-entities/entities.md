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
- **The definitions**, built in `Entities::build`, whose signature and callers are unchanged:
  - an installed skill, an installed agent and the primer, and their sections, from the installed
    copies the model holds, parsed like a walked document and walked by nothing: `Model::build`
    reads them off the disk, and `commit_tree` in `path@core@src/cli/history.rs` hands in each
    commit's own blobs, through `Model::set_installed`;
  - a project skill, a project agent and the root instructions, and their sections, from the walk.

  Found while building, applied in place: the spec first had every caller of `Entities::build`
  hand the copies in, from `Survey::installed`. The public `Inputs` holds no copy under `commits`
  on purpose, and the model carrying them reaches the same end with no change to that interface.
- **The section rule**: every level-two heading outside a fenced block of a section home owes a
  slug, and a slug at another level of a section home is a misplaced definition. A section home is
  each skill's `SKILL.md`, each agent's file, the primer and the root instructions. An id is unique
  within its one document. A slug defined at a level-two heading of a project skill or agent is no
  longer a misplaced definition.
- **A name outside the id grammar**, on the owner's ruling of D1: a skill directory or an agent file
  whose name is outside `[a-z0-9]+(-[a-z0-9]+)*` is a finding, naming the file.
- **A name that its frontmatter contradicts**, on the owner's ruling of Q1: a frontmatter `name`
  that differs from the skill's directory name or the agent's file name, or a `name` written twice,
  is a finding, naming the file and both names. Both are raised while the entity table is built,
  phase 3, where an entry id no reference can spell is already reported; the spec first placed
  them in phase 2, which reads no frontmatter.
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
- **The finding texts** of every new finding name the repair, and a cause only where the repair
  depends on it, per `design@core@finding-names-the-repair`.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| `skill@<name>` resolves to an installed skill, a project skill, or to nothing with a finding naming the skill | a unit test of `path@core@src/entity.rs` over a model holding one project skill and a list of installed copies holding one installed skill | a mutation that drops the installed list in `Entities::build` makes the installed case dangle |
| A section reference resolves only to a slug defined in that skill's own file | a unit test citing a slug of skill A through skill B | a resolver that looks the slug up across every skill passes the wrong case, and the test fails |
| Every arity the mapping table does not list is malformed, with the forms in the repair | a unit test per arity, for each kind | a segmentation that accepts any arity for a harness kind passes a four-segment span |
| Under `harness = []`, a span headed by a harness kind is silent | a unit test of `candidate` over the anchors of a manifest declaring `harness = []`, in `path@core@src/entity.rs` | registering the kinds whatever the harness reports the span |
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

**The branch's first commit holds every stricter check and the content it needs**: the section rule,
the name-grammar finding and the kind-name refusals, with the 130 slugs. `commits` judges every
earlier commit of a branch with the tip's checker, so a commit before the one that makes a check
stricter fails under the tip, per `design@core@a-commit-message-is-a-document` and §4 of
`klarch-development`. Steps 1 and 2 may be developed as separate commits, and are squashed into the
first commit before the review; step 3 and the review repairs follow it. Found at the audit, from
`issue@core@a-contract-change-fails-every-earlier-commit-unexplained`, which describes the same
failure for a generated file.

## Decided design

### #skill-register and #claude-md-sections: four kinds, under the `claude` harness

| kind | an entity | defined from | cited |
| --- | --- | --- | --- |
| `skill` | a directory directly under `path@agent-config@skills/` holding `SKILL.md`, its id the directory's name, which a frontmatter `name` must equal where one is set, per Q1 below | the installed copies for a name the installer's namespace holds, the walk otherwise | `skill@<name>` |
| `agent` | a Markdown file at any depth under `path@agent-config@agents/` with a frontmatter `name`, its id the file's basename, which the `name` must equal, per Q1 below | the same | `agent@<name>` |
| `primer` | the primer, `path@agent-config@knowledge-architect/PRIMER.md`; only its sections are entities | the installed copies | `primer@<slug>` |
| `instructions` | the root CLAUDE.md; only its sections are entities | the walk | `instructions@<slug>` |

The paths are this repository's spelling. The layout is the harness's, a directory `.claude` at the
project's root, in every project.

- **Nothing is declared** (`argument@agent-configuration-entities@a6`): the harness fixes the layout. Under `harness = []` none of the four
  kinds exists, and a span headed by one is silent, as any span whose head is no kind and no anchor.
- **Kinds, not registers**, ruled at the audit, Q2 below. A harness kind is answered as `path` and
  `planned` are, with no anchor and no declared home, so nothing a project declares can add one.
- **No README and no index** are owed under the skills and agents directories (`argument@agent-configuration-entities@a8`). Read at
  the audit, in the harness's documentation of subagents,
  https://code.claude.com/docs/en/sub-agents.md: a file there with "No `name`: Claude Code treats
  the file as documentation kept beside your agents", so a README is harmless, and owing one would
  be structure the harness does not need.
- **A skill is named by its directory.** Read at the audit, in
  https://code.claude.com/docs/en/skills.md: "The directory name, or the frontmatter `name` when
  you set one, becomes the command you type", and when `name` is set, "The directory name also
  invokes the skill". So the directory's name always invokes a skill, and the frontmatter adds a
  second command. A skill is cited by its directory's name, and no check compares the two.
- **A skill below the project's root**, in a directory `<subdirectory>/.claude/skills/`, is
  loaded by the harness when a session reads a file there, per the same page. It is no entity of
  this slice: a reference to it is reported dangling.
- **What is no entity**: a directory under the skills directory that holds no `SKILL.md`, a file of
  a skill directory other than `SKILL.md`, a file under the agents directory that is not
  Markdown, and a Markdown file there with no frontmatter `name`. None is reported. A project-owned one stays an ordinary
  walked document; an installed one stays outside the walk.
- **A scoped CLAUDE.md is not modelled** (`argument@agent-configuration-entities@a51`, `argument@agent-configuration-entities@a52`). The root CLAUDE.md stays, so its sections are.
- **Under `harness = []`, the installed copies a project kept** are walked as its own documents,
  per `design@core@agents-table`, and their rendered slugs are misplaced definitions there. Per D8
  of the milestone document, those findings stand: the repair is to remove the copies or to serve
  the harness.
- **Nearest rival:** no entities, with skills cited by path, as today. It leaves every citation of a
  skill unchecked, which is the need the owner named in R1 (`argument@agent-configuration-entities@a1`).

### Ruled at the audit

- **Q1, an agent's id: the file-system name and the frontmatter `name` must match.** The owner:
  "this finding makes me want to apply the check that the filesystem name and the frontmatter name
  do match, returning a finding if they do not. I'm not sure our installed workflow would be able to
  work properly if not, and I'd rather make the checks more strict to keep the workflow simpler
  than the other way around." So a skill's id is its directory's name and an agent's id its file's
  basename, and each equals its frontmatter `name` where one is set. How the name is read is D9 of
  the milestone document, the assistant's default:
  - only the `name:` line is read, by the frontmatter subset's line rule of
    `path@core@src/source/md.rs`, even where other lines of the block fall outside the subset, so a
    project keeps frontmatter the harness reads, such as a list or a multi-line description;
  - a `name` that differs from the directory's or the file's name is a finding, and so is a `name`
    written twice;
  - a skill with no `name` is accepted, since the harness makes the field optional and invokes the
    skill by its directory's name;
  - a Markdown file under the agents directory with no `name` is no agent, since the harness reads
    it as documentation;
  - an agent in a subdirectory of the agents directory is an agent, since the harness scans the
    directory recursively; two agents of one name are the duplicate finding.

  The discussion's rival, the frontmatter `name` alone as the id, lost to the owner's ruling for
  strictness.
- **Q2, kinds, not registers.** The owner: "agreed, they are kinds only and not "register". I don't
  think they fit the "register" model we have, and it would be weird to force them to fit IMO."

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
an interface, so this table is where its names are judged. A slug is written in the code span of a
slug, here without the two hashes.

| file | heading | slug |
| --- | --- | --- |
| `path@agent-skills@content/skills/agent-configuration/SKILL.md` | 1. Content and style of agent-facing files | `content-and-style` |
|  | 2. Where a piece of agent-facing text goes | `where-text-goes` |
|  | 3. Shaping a skill | `shaping-a-skill` |
|  | 4. The two tables of the root CLAUDE.md | `root-claude-md-tables` |
|  | 5. Installed files are never edited | `installed-files-never-edited` |
|  | 6. After installing a new version | `after-installing` |
|  | 7. Reviewing a configuration change | `reviewing-a-change` |
| `path@agent-skills@content/skills/decision-recording/SKILL.md` | 0. When recording happens | `when-recording-happens` |
|  | 1. Does it reverse something already recorded? | `reversal-check` |
|  | 2. Does it earn a document entry at all? | `entry-tests` |
|  | 3. Which Component owns it | `owning-component` |
|  | 4. Three homes, split by function | `three-homes` |
|  | 5. The current design | `current-design` |
|  | 6. Losing alternatives | `losing-alternatives` |
|  | 7. Tripwires from a premortem | `premortem-tripwires` |
|  | 8. Before you finish | `before-you-finish` |
| `path@agent-skills@content/skills/design/SKILL.md` | Overview | `discussion-overview` |
|  | Decision authority | `decision-authority` |
|  | Bounded work | `bounded-work` |
|  | Language | `discussion-language` |
|  | Threads and states | `threads-and-states` |
|  | The loop | `discussion-loop` |
|  | Keep-or-change (evaluating an incumbent design) | `keep-or-change` |
| `path@agent-skills@content/skills/goal-setting/SKILL.md` | 1. A goal, and what is not one | `goal-or-not` |
|  | 2. The shape of a goal entry | `goal-entry-shape` |
|  | 3. Drawing out the owner's intent | `drawing-out-intent` |
|  | 4. When this runs again | `when-it-runs-again` |
| `path@agent-skills@content/skills/issue-tracking/SKILL.md` | What is outstanding, across every register | `outstanding-across-registers` |
|  | Read before you diagnose | `read-before-diagnosing` |
|  | An issue entry | `issue-entry` |
|  | The trigger test | `trigger-test` |
|  | A tripwire entry | `tripwire-entry` |
|  | The movement instruction | `movement-instruction` |
|  | The cold-reader standard | `cold-reader-standard` |
|  | Reviews | `reviews-and-entries` |
| `path@agent-skills@content/skills/planning/SKILL.md` | Terms | `planning-terms` |
|  | 1. When this skill starts | `when-planning-starts` |
|  | 2. A spec or a milestone | `spec-or-milestone` |
|  | 3. Layout | `plans-layout` |
|  | 4. What a spec holds | `spec-contents` |
|  | 5. Cutting the steps and the slices | `cutting-steps-and-slices` |
|  | 6. Acceptance criteria | `acceptance-criteria` |
|  | 7. Working a slice, and the work of a spec | `working-a-slice` |
|  | 8. Reviews of a plan document | `plan-reviews` |
|  | 9. When a plan document leaves | `plan-document-leaves` |
| `path@agent-skills@content/skills/retrospective/SKILL.md` | 1. When it is offered | `when-offered` |
|  | 2. What it examines | `what-it-examines` |
|  | 3. Two files, sorted by whose text must change | `two-files` |
|  | 4. What becomes of each file | `what-becomes-of-files` |
|  | 5. What the installed skills expect of the owner | `expectation-sets` |
| `path@agent-skills@content/skills/review/SKILL.md` | 1. The axes | `review-axes` |
|  | 2. The invariants | `review-invariants` |
|  | 3. What a review leaves behind | `what-review-leaves` |
| `path@agent-skills@content/skills/setup/SKILL.md` | 1. Pin the checker, and decide how it runs | `pin-the-checker` |
|  | 2. Declare the command | `declare-the-command` |
|  | 3. The Components and the locations | `components-and-locations` |
|  | 4. The documents each Component carries | `component-documents` |
|  | 5. The root CLAUDE.md | `root-claude-md` |
|  | 6. The gates | `setup-gates` |
|  | 7. Finish | `finish-setup` |
|  | 8. Existing documentation | `existing-documentation` |
|  | Moving the pin | `moving-the-pin` |
|  | In a Rust project | `rust-project` |
| `path@agent-skills@content/agents/code-claims-reviewer.md` | 1. Collect the claims | `collect-the-claims` |
|  | 2. Verify each | `verify-each-claim` |
|  | 3. Look for what the document does not say | `what-is-unsaid` |
|  | Reporting | `how-to-report` |
| `path@agent-skills@content/agents/cold-implementer-reviewer.md` | 1. Ground as the implementer would | `ground-as-implementer` |
|  | 2. The five questions | `five-questions` |
|  | 3. The readiness checks | `readiness-checks` |
|  | Reporting | `how-to-report` |
| `path@agent-skills@content/agents/decision-record-reviewer.md` | 1. Tools to carry your task | `carrying-tools` |
|  | 2. The predicates | `record-predicates` |
|  | Reporting | `how-to-report` |
| `path@agent-skills@content/agents/design-conformance-reviewer.md` | 1. Find the record the document touches | `find-the-record` |
|  | 2. Read the document against the record | `read-against-record` |
|  | Reporting | `how-to-report` |
| `path@agent-skills@content/agents/routing-reviewer.md` | 1. Mechanical | `mechanical-checks` |
|  | 2. The predicates | `routing-predicates` |
|  | Reporting | `how-to-report` |
| `path@agent-skills@content/agents/standing-entry-searcher.md` | Your brief | `your-brief` |
|  | 1. Take your group | `take-your-group` |
|  | 2. Read every entry of your group | `read-every-entry` |
|  | 3. Follow the seeds | `follow-the-seeds` |
|  | 4. Judge every entry against the work | `judge-each-entry` |
|  | Return | `what-to-return` |
| `path@agent-skills@content/agents/standing-state-reviewer.md` | 1. Mechanical | `mechanical-checks` |
|  | 2. Re-read every tripwire and every deferred trigger | `reread-standing-entries` |
|  | 3. A landing plan document's acceptance criteria | `landing-acceptance-criteria` |
|  | 4. The predicate | `standing-predicate` |
|  | Reporting | `how-to-report` |
| `path@agent-skills@content/agents/transcript-reviewer.md` | 1. Extract the transcripts | `extract-transcripts` |
|  | 2. List what must outlive the sessions | `list-what-outlives` |
|  | 3. Find the outcome of each | `find-each-outcome` |
|  | 4. Check each recorded ruling against the owner's words | `check-recorded-rulings` |
|  | 5. Report | `how-to-report` |
| `path@agent-skills@content/PRIMER.md` | Goals bind; decisions bind as a presumption | `goals-bind` |
|  | The owner's word and its premise | `owner-word-premise` |
|  | Room to judge | `room-to-judge` |
|  | Intent and claims | `intent-and-claims` |
|  | Something met outside the task | `met-outside-the-task` |
|  | Where knowledge goes | `where-knowledge-goes` |
|  | The installed skills | `installed-skills` |
| `path@knowledge-architect@CLAUDE.md` | Language, tone and style | `language-and-style` |
|  | Mechanical validation of documents | `mechanical-validation` |
|  | Where knowledge goes | `where-knowledge-goes` |
|  | Verify before relying on anything | `verify-before-relying` |
|  | Verify mechanically | `verify-mechanically` |
|  | Skills | `repository-skills` |
|  | Git | `git-workflow` |
|  | Release status | `release-status` |
| `path@agent-config@skills/klarch-development/SKILL.md` | 0. Ground before editing | `ground-before-editing` |
|  | 1. The loop | `development-loop` |
|  | 2. Claims, and tests that discriminate | `discriminating-tests` |
|  | 3. The review axes for code | `code-review-axes` |
|  | 4. The gate | `development-gate` |
|  | 5. Incompleteness is recorded, never encoded | `incompleteness-recorded` |
|  | 6. Comments: two species, two audiences | `comment-species` |
| `path@agent-config@skills/klarch-release/SKILL.md` | 1. The release branch | `release-branch` |
|  | 2. Merge, then publish | `merge-then-publish` |
|  | 3. When the publish fails | `failed-publish` |
| `path@agent-config@skills/klarch-retrospective-intake/SKILL.md` | 1. What it takes | `what-it-takes` |
|  | 2. Ground before judging any finding | `ground-before-judging` |
|  | 3. What to establish for each finding or cluster | `what-to-establish` |
|  | 4. Subagents | `intake-subagents` |
|  | 5. The analysis file and the owner's ruling | `analysis-and-ruling` |
|  | 6. After the ruling | `after-the-ruling` |
| `path@agent-config@agents/klarch-changelog-reviewer.md` | The standard | `changelog-standard` |
|  | The predicates | `changelog-predicates` |
|  | Reporting | `how-to-report` |

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
| the changelog | `Next release`. Under Migration: every level-two heading of the root CLAUDE.md and of each project skill and agent owes a slug, mock projects serving `claude` included; a Component, location or plan named after any kind, and a register named after a harness kind, is renamed. Under New features: the four harness kinds and the section references. Surfaces and classes: `manifest`, major, for the refused names, per `design@knowledge-architect@versioning-policy`, since a manifest that was valid stops being accepted; `checks`, minor, for the slug requirement and the new kinds; `agent-skills`, patch, for the slugs in the installed text. A project skill's slug stops being a misplaced definition, a check made looser, which the table has no row for, per `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`: it is classed minor, the nearest reading, as the 0.3.0 changelog did |

This slice's spec leaves in the commit that completes this harvest.
