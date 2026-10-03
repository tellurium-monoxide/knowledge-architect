# Pre-release fixes: `check --fix`, `--version`, a crates.io page per crate, and the generic anchor

## Status and audience

**This is a spec**, under the installed `knowledge-architect-planning`. It carries the design
converged in one discussion between the owner and an agent, under `knowledge-architect-design`,
and the work of one branch and one pull request that builds it. It is written for a session that
did not witness that discussion.

- **It leaves when its work lands**: it is deleted in the commit that completes its harvest, and
  that commit cites it as `spec@plans@pre-release-fixes`.
- **Where this spec and a design home disagree, the design home wins**, and this spec has the
  defect.
- **Every name it uses is defined in it, or exists in the code.** The Names section expands the
  shorthands. The New names section lists every name the work adds.
- **Where the owner's word is needed and the owner is absent, the work does not proceed on that
  point.** The points are listed under "Defaults awaiting the owner".
- **The record was assembled from the transcript**, by a subagent, as planning §4 says. The file
  read is the session log
  ~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/7fa17ca7-61f7-41c1-a0f4-7241640fbfbe.jsonl.
  The discussion begins at record 5379, counted from 0, the owner's message opening "Before the
  release, are there any open issue you think should be examined further", and runs to the owner's
  round-8 message, record 5737. The log's two compaction records, 3247 and 3251, precede the
  discussion.
- **Extraction.** Records were selected by their fields, never by text. A record of `type`
  `user` is the owner's when `isMeta` is not true and `origin.kind` is `human`. Two `user`
  records with no origin hold only the harness's text "[Request interrupted by user]", 5519 and
  5558; they are interruptions, not owner messages, and open no round. Three `isMeta` records,
  5494, 5495 and 5555, inject a note that the design skill was re-invoked and the text of the
  design and planning skills; they carry no ruling.
- **Rounds.** A round is one owner message and the reply to it, numbered from the discussion's
  first owner message. There are eight, R1 to R8, opening at records 5379, 5443, 5520, 5543, 5549,
  5565, 5583 and 5737. R5 has no reply text: the owner interrupted after the planning skill loaded.
  R8 rules the defaults D1 to D7, which the reviews of this spec and its audit produced.
- **Spans normalised.** In verbatim quotations, a backticked path of two or more segments is
  written in plain text, and a reference to one of the three issues that close with this spec is
  written without backticks, so that the checker does not read a pointer this spec cannot keep.
  In `argument@pre-release-fixes@a17`, the incomplete generic form path@*@ is written without
  backticks for the same reason. The words are unchanged.

## How a step is worked

This spec is one pull request; the procedure is `knowledge-architect-planning`.

- **The audit.** This spec's design audit is the review of the spec and the rulings of R8, applied
  in place. Its commit lists the findings.
- **The code steps**, 1, 2 and 3, follow `klarch-development`.
- **Step 4's edit of row 202** follows §4 of `knowledge-architect-agent-configuration`.
- **Step 5's edit of the setup skill** follows `path@agent-skills@CLAUDE.md` and
  `knowledge-architect-agent-configuration`.
- **Each step's commit** reports the acceptance criterion that step judges, by its identifier in
  plain text.
- **The review before the merge**, and then the review of the harvest, each run on the
  decision-record, routing and standing-state axes, with the transcript reviewer last and alone.

## Names

| name | what it names |
| --- | --- |
| klarch | the core's binary, built from `path@core@src/main.rs`; `cargo klarch` runs it from this checkout, per the alias in `path@knowledge-architect@.cargo/config.toml` |
| `check`, `index`, `install-agent-skills` | the commands of `Command` in `path@core@src/cli/mod.rs`, run by the functions `check`, `index` and `install_agent_skills` of that file |
| phase 1 to phase 4 | the four phases of a run, per `design@core@phases-gate-the-report`: the manifest, the tree, the entity table, every check |
| the gate of a writer | `complete_working_tree` in `path@core@src/cli/mod.rs`: it runs phases 1 to 3 and returns an error, exit 2, when one holds a finding |
| the generated list | the pairs of destination and bytes that `index` assembles in `path@core@src/cli/mod.rs`: each extension's `generated` after `prepare` with `Purpose::Index`, then `crate::index::file_register_indexes`. The `generated` check, in `path@core@src/check/generated.rs`, compares each file of the same list with the tree: `crate::check::run_with` hands it the extensions' files, and it computes the file-register indexes itself |
| the install | `crate::agents::install` in `path@core@src/agents.rs`, over `crate::agents::shipped`: it writes each shipped file whose bytes differ, and deletes each file of the installer's namespace that the version does not ship |
| an installed-file finding | a finding of `check` in `path@core@src/check/agents.rs`: a shipped file missing or differing, an unshipped file in the namespace, an unstaged deletion of one, a primer not imported |
| the gates | the gate list of `rust_project` in `path@gates@src/gates.rs`, run by `cargo x gates`; its `check` gate runs `check` with no other argument |
| row 202 | the row of the root `CLAUDE.md`'s own knowledge table on line 202, "how a user can use a published crate, beyond the primer's README row" |
| a crates.io page | the file Cargo packages as a crate's README, which crates.io renders as the crate's page. Today it is `README.md` by Cargo's default: no Cargo.toml under crates/ sets `readme` |
| CRATES-IO.md | the new file per crate that holds its crates.io page, under shape B |
| shape A, shape B | the two shapes for the crates.io page put to the owner in R6: A rewrites `README.md` for crates.io readers, B adds a separate file |
| the three todo issues | the core's issues the-binary-reports-no-version, the-readme-is-the-crates-io-page and check-cannot-regenerate-what-it-reports. They close in the commit that adds this spec, per planning §2, so they are named here in plain text |
| R1 to R8 | the rounds of the discussion |
| P1 to P5 | the causes of the premortem |
| T1 | the tripwire the premortem proposed, recorded on the owner's word in R5 |
| D1 to D7 | the defaults the owner ruled in R8, listed under "Defaults awaiting the owner" |

## What the work is

Four parts, in one pull request, on the owner's word in R2: "We'll do, in one PR, 1 and 2, as well
as take a look at the-generic-anchor-accepts-a-location-s-copy (explain it to me in greater
details so I rule on whether it is worth fixing). I'd like to discuss
check-cannot-regenerate-what-it-reports too."

### Part 1: `check --fix`

**Today.** `Command::Check` in `path@core@src/cli/mod.rs` is a unit variant: `check` takes no
argument, and `cargo klarch check --fix` exits 2 with clap's "unexpected argument '--fix' found",
observed at assembly. The function `check` builds the model, runs `crate::check::foundation`
(phases 1 to 3), prepares each extension with `Purpose::Check`, and runs every check. It writes
nothing. `index` builds the model, passes the gate of a writer, assembles the generated list,
refuses every destination whose directory is missing or which is a symlink before writing any, and
writes only where the bytes differ. A write failure exits 2 when nothing was written yet and 1
after a write. `install_agent_skills` refuses a manifest with complaints (phase 1), does nothing
when the manifest serves no agent harness, and otherwise runs the install. It does not run the
phase gate.

The issue check-cannot-regenerate-what-it-reports records the friction. Its owner's words, kept
here because the issue closes with this spec: "updating index is *always needed*, and needing two
commands for this kinda inefficient. check having an optional flag to also do the updates would be
convenient." It also records that a session in thaum on v0.1.0 met the same friction with a
citation index its extension generates.

**The work.** `check` takes `--fix`. A `--fix` run applies every safe fix, then runs the full
check. Two fixes exist: the install, and the generated list. The decided design is below.

### Part 2: `--version`

**Today.** The `Cli` struct of `path@core@src/main.rs` declares `name`, `about` and
`arg_required_else_help`, and no version. `cargo klarch --version` exits 2 with "unexpected
argument '--version' found", observed at assembly. A session that needs to know which checker ran
reads the project's pin instead, per `design@agent-skills@version-in-report`.

**The work.** `klarch --version` prints the version of the package `knowledge-architect`, with a
test of its output, and a CHANGELOG.md entry under New features, `cli`, minor. This is the issue's
own closing condition.

### Part 3: a crates.io page per published crate

**Today.** No Cargo.toml under crates/ sets `readme`, so each crate's `README.md` is its crates.io
page. Each crate's `include` lists /README.md. The backticked references each README holds,
counted at the root with ``grep -o '`[a-z*-]*@[^`]*`' <file> | wc -l``: crates/core/README.md 33,
crates/agent-skills/README.md 5, crates/gates/README.md 0. A looser pattern, any backticked span
holding an at sign, counts 37 for the core README, because it also matches placeholders. Each
reference renders on crates.io as a code span that leads nowhere. Row 202
says a published crate's `README.md` "is also its crates.io page, and points to docs.rs for the
library API".

**The work, under shape B.** `README.md` stays as it is, for this repository's reader. Each of the
three published crates, core, agent-skills and gates, gains a CRATES-IO.md beside its `README.md`,
named by `readme = "CRATES-IO.md"` in its Cargo.toml; the gates crate is included on the owner's
ruling of D4 in R8. Row 202 changes to the text the owner approved in R7. The setup skill,
`path@agent-skills@content/skills/setup/SKILL.md`, gains the owner's default for published Rust
crates.

### Part 4: the generic anchor

**Today.** In `path@core@src/check/references.rs`, function `path`, the arm for the generic anchor
`*` first refuses a refused path shape, through the function `refused`, then accepts a required
document. Otherwise it collects the anchors whose own copy of the path
is present and not ignored, over `anchors.all()` filtered by `a.constructed.is_none()`. That
filter keeps the Components and the declared locations. So `path@*@<path>` passes when only a
location holds the path. `design@core@reserved-anchors` accepts the generic form "when at least one
component carries the path with the claimed kind". The comment above the filter cites
the issue the-generic-anchor-accepts-a-location-s-copy. `Anchor` in `path@core@src/entity.rs`
carries `is_component`.

**The work, option (a).** The filter keeps Components only, with a unit test of the issue's
reproduction in `path@core@src/check/references.rs`: a manifest declaring a location `notes` that
holds held.md and a Component that does not. There, a generic reference to held.md is reported as
resolving in no component, and it passes once the Component holds it.

### Outside the work

| what | why, and who owns it |
| --- | --- |
| the installed skills that name `index`: the setup skill's §7, the issue-tracking skill and the planning skill. The primer does not name it | #index-stays: both commands stay, and the installed text that names them is unchanged. A later change may name `check --fix` there; see Later consequences |
| the repair texts of the findings, "run `<command> index`" in `path@core@src/check/generated.rs` and "run `<command> install-agent-skills`" in `path@core@src/check/agents.rs` | #index-stays: both commands stay, so each repair still names a command that exists |
| `commits` and the gates take no `--fix` | #fix-reports-what-it-wrote: they judge, and a stale index must fail CI |
| `issue@core@installed-file-findings-belong-in-phase-four` | the owner judged the move correct and not worth doing now, as the issue records. This work changes what it costs; see "What is already decided" |
| `issue@core@headings-are-read-unlike-markdown` | named in R1 as a checker defect that does not block the release; the owner did not take it in R2 |
| every other open issue | R1 classed each as fine to leave for later, or as a defect that does not block the release; the owner took none of them in R2 |
| a premortem of parts 2, 3 and 4 | the discussion ran the premortem over `check --fix` only. No cause was named against the other three parts |

## What is already decided

The design rests on these and does not argue them again:

- `design@core@generated-files-are-pure`: one function gives the writer and the gate the same list
  of destinations and bytes. `check --fix` becomes a second writer over that list.
- `design@core@owned-namespace-check`: the install, its namespace, and its byte comparison.
  `check --fix` runs the same install. The harvest judges whether the head names `check --fix`.
- `design@core@exit-code-ladder` and `design@core@arguments-parse-through-clap`: the exit codes,
  and the test of each binary's declaration in `path@core@src/main.rs`.
- `design@core@the-core-cli-is-a-library-module` and `design@core@ne-minimal`: `Command` is public
  and non-exhaustive. Changing the shape of its variant `Check` is D3.
- `design@core@reserved-anchors`: option (a) makes the code match this head, which is unchanged.
  In the root `CLAUDE.md`, line 141 is its restatement, and stays true. Line 127 only uses the
  reference as an example of the reference form.
- `design@agent-skills@version-in-report` and `design@agent-skills@exact-pin`: what `--version`
  serves.
- `design@agent-skills@shipped-text-is-reference-free` and
  `design@agent-skills@additions-need-real-use`: what the setup skill's addition owes.
- `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`:
  the entries the branch writes.

These are rewritten by this work, at the harvest unless the item names a step:

1. **`design@core@model-then-checks`**, its paragraph "**Nothing writes to the tree while
   checking.**", becomes the approved wording of #fix-before-the-checks: "No check writes to the
   tree; `check --fix` writes before the model its checks read is built". The reason the clause
   exists stays in the paragraph.
2. **`design@core@phases-gate-the-report`**, its sentence "**A writer refuses over an incomplete
   model**", names `check --fix` among the writers, for its generated files.
3. **`design@knowledge-architect@package-include-whitelist`** says each crate's `include` lists
   "its sources, its Cargo.toml, its README and its license files, and nothing else". Each of the
   three crates also ships its CRATES-IO.md, which Cargo packages as the `readme` file without an
   `include` entry (see Readings).
4. **Row 202 of the root `CLAUDE.md`**, the owner's configuration, becomes the text the owner
   approved in R7 (#row-202-rewrite). It is edited in step 4, on the owner's word given in R7.
   D4, ruled in R8, gives the gates crate a CRATES-IO.md too, so the approved text holds for all
   three crates.

What else references a rewritten decision, as `cargo klarch show` lists it, and the harvest that
judges each:

| entry rewritten | text referencing it | judged |
| --- | --- | --- |
| model-then-checks | `path@core@docs/design.md`, two sites in the extension heads, lines 157 and 179: the filesystem exception, and one parse per document | unaffected: neither cites the rewritten clause; read at the harvest |
| model-then-checks | `path@core@docs/rejected-alternatives.md` line 84, "One family per invocation": the single walk | unaffected; read at the harvest |
| model-then-checks | the doc comment of `pub struct Gathered`, `path@core@src/cli/gathered.rs` line 14: a check is pure over fetched inputs | unaffected; read at the harvest |
| model-then-checks | the doc comment of `Tree::Checkout`, `path@core@src/extension.rs` line 44: the filesystem exception | unaffected; read at the harvest |
| model-then-checks | the issue check-cannot-regenerate-what-it-reports, two sites | leaves with the issue, in the commit that adds this spec |
| phases-gate-the-report | `path@core@docs/design.md` line 1545, in `design@core@owned-namespace-check`: "A command that writes a generated file refuses while one stands" | the harvest judges it: `check --fix` installs before that refusal |
| phases-gate-the-report | `path@core@README.md`, its `check` section: the reference at line 87, and the paragraph after it, "`index`, and a writing command of an extension, run the first three phases too" | updated in step 3, which names `check --fix` there |
| phases-gate-the-report | `tripwire@core@phases-gate-the-report-two` | judged at this audit: it does not fire (D7), below |
| phases-gate-the-report | `issue@core@installed-file-findings-belong-in-phase-four` | judged at the harvest, below |
| phases-gate-the-report | the issue check-cannot-regenerate-what-it-reports | leaves with the issue |
| phases-gate-the-report | the root `CLAUDE.md` line 422; `path@core@CLAUDE.md` line 86; `path@core@docs/design.md` lines 56, 139, 454 and 767; `path@core@docs/rejected-alternatives.md` lines 66, 76, 82 and 88 | unaffected: each cites the four phases, the stop, or the absence of a selection, which this work does not change |
| package-include-whitelist | `path@agent-config@skills/klarch-release/SKILL.md` line 69: each package list holds "its sources, its Cargo.toml, its README and its licence files" | updated at the harvest, under `knowledge-architect-agent-configuration` |
| row 202 | the issue the-readme-is-the-crates-io-page | leaves with the issue |

**The tripwire `tripwire@core@phases-gate-the-report-two` is met by its re-entry event**, "any
change to the arguments of `check`". It is judged at this audit, before the code, and does not
fire (D7, ruled in R8). Its firing condition is a way to "let a writer write over an incomplete
model". The install's bytes come from the pinned version and the installer's file list, not from
the model, and `install_agent_skills` in `path@core@src/cli/mod.rs` already writes after checking
only the manifest's complaints. The generated files, whose bytes depend on the model, stay behind
the full gate of phases 1 to 3. The criterion #no-write-over-incomplete-model rests on that
reading.

**The issues.**

| issue | what this work does to it |
| --- | --- |
| the-binary-reports-no-version | scheduled here: closes in the commit that adds this spec. Part 2 is its closing condition |
| the-readme-is-the-crates-io-page | scheduled here: closes in the commit that adds this spec. Its closing condition asks for "A README whose every pointer resolves for a reader on crates.io, and the repository-facing text in a home the knowledge table names". Under shape B the crates.io page is CRATES-IO.md, and the repository-facing text stays in `README.md`, the home the knowledge table names |
| check-cannot-regenerate-what-it-reports | scheduled here: closes in the commit that adds this spec. Part 1 meets each point of its closing condition: a flag of `check` (named `--fix`), its three tests, the README's CLI section, and the rewrite of the clause with the argument about `--write`. One point differs, on the owner's word in R4: the issue says "A run with earlier findings writes nothing"; #fix-before-the-checks runs the install whenever phase 1 is clean |
| the issue the-generic-anchor-accepts-a-location-s-copy | a defect, not scheduled work: closes at the landing, in the commit that lands part 4. The same commit removes the citation of it from the comment in `path@core@src/check/references.rs` |
| `issue@core@installed-file-findings-belong-in-phase-four` | stays open. Its "Why it matters" says "after a version renames a skill, `index` refuses until the install runs". `check --fix` installs before the gate, so that refusal no longer costs an extra command, except for the unstaged deletion of D1. Its closing condition, the move of the check to phase 4, is not this work. The harvest rewrites its "Why it matters" to say what remains |

## Criteria

Six criteria, named in R3 for `check --fix`. Parts 2, 3 and 4 were not judged against a criteria
table in the discussion. Every satisfaction line is the one the agent showed in R4, after the
owner's approval.

### Never write over an incomplete model `##no-write-over-incomplete-model`

- **Kind:** binding, from `tripwire@core@phases-gate-the-report-two` and `complete_working_tree`
- **Source:** R3, agent
- **Satisfaction:** met: `thread@pre-release-fixes@fix-before-the-checks`

### The writer and the gate read one list of generated files `##one-list-of-generated-files`

- **Kind:** binding as a presumption, from `design@core@generated-files-are-pure`
- **Source:** R3, agent
- **Satisfaction:** met: `thread@pre-release-fixes@fix-before-the-checks`

### No check writes to the tree it judges `##no-check-writes`

- **Kind:** binding as a presumption, from the purpose of `design@core@model-then-checks`
- **Source:** R3, agent
- **Satisfaction:** met: `thread@pre-release-fixes@fix-before-the-checks`

### A fix never changes hand-written content and never makes a choice `##fix-makes-no-choice`

- **Kind:** binding, from the owner's word "safely" in R3
- **Source:** R3, agent, deriving it from the owner's argument `argument@pre-release-fixes@a5`
- **Satisfaction:** met: `thread@pre-release-fixes@safe-fix-definition`

### One command per edit cycle `##one-command-per-edit-cycle`

- **Kind:** binding, the owner's request, recorded in the issue check-cannot-regenerate-what-it-reports
- **Source:** R3, agent
- **Satisfaction:** met: `thread@pre-release-fixes@check-fix-flag`, except for an upgrade where
  the install removes an unshipped file, which takes two runs; the owner accepted that case in R8
  (D1)

### A later safe fix needs no new option `##later-fix-needs-no-option`

- **Kind:** weighed, the owner's argument `argument@pre-release-fixes@a5`
- **Source:** R3
- **Satisfaction:** met: `thread@pre-release-fixes@check-fix-flag`

## Threads

Eighteen threads, one item each. Six slugs were minted by the discussion, in R3:
check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks, fix-reports-what-it-wrote
and index-stays. Twelve threads were proposals the discussion made without a slug, and their slugs
were minted at assembly: fix-command, index-only-flag, generated-files-only, version-flag,
readme-rewritten-for-crates-io, crates-io-page-file, crates-io-file-name,
crates-io-file-per-referencing-crate, row-202-rewrite, setup-default-crates-io-page,
generic-anchor-components-only and generic-anchor-any-anchor. Whether an approved thread earns a
design entry is decided at the harvest, under `knowledge-architect-decision-recording`; one that
does takes the thread's slug.

Rulings on items that are not threads:

| item | round | the owner's words (verbatim) |
| --- | --- | --- |
| the scope: one PR for items 1 and 2, the generic anchor examined, the check flag discussed | R2 | "We'll do, in one PR, 1 and 2, as well as take a look at the-generic-anchor-accepts-a-location-s-copy (explain it to me in greater details so I rule on whether it is worth fixing). I'd like to discuss check-cannot-regenerate-what-it-reports too." |
| tripwire T1 | R5 | "Record T1, and the default for the README is fine" |
| permission to write this spec | R7 | "go ahead with the spec" |
| the defaults D1 to D7, D1 with its option (i) | R8 | "D1: go with (i), skill deletion/rename is a rare thing anyway. Not much of a problem if it takes multiple invocations to handle. All other defaults approved. proceed." |

### `check --fix` applies every safe fix, then runs the full check `##check-fix-flag`

- **Proposed by:** the owner, R3 (an optional flag on `check`, with a generic name); the agent
  named the flag `--fix` in R3, citing the owner as its proposer
- **Final state:** approved
- **Arguments:** `argument@pre-release-fixes@a3`, `argument@pre-release-fixes@a5`,
  `argument@pre-release-fixes@a8`, `argument@pre-release-fixes@a9`,
  `argument@pre-release-fixes@a19`
- **Shape in:** Decided design, "The flag"
- **Harvest home:** the core's design home
- **Closed by:** R4: "check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks,
  fix-reports-what-it-wrote, index-stays approved."

### A fix is safe when its bytes are determined by the tree and the pinned version, and it touches only files the tool generates or installs `##safe-fix-definition`

- **Proposed by:** the agent, R3
- **Final state:** approved
- **Arguments:** `argument@pre-release-fixes@a6`, `argument@pre-release-fixes@a11`
- **Shape in:** Decided design, "What a safe fix is"
- **Harvest home:** the core's design home; T1 in the core's tripwires home names its head
- **Closed by:** R4: "check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks,
  fix-reports-what-it-wrote, index-stays approved."

### The fixes are the install and the generated files `##fix-scope`

- **Proposed by:** the agent, R3
- **Final state:** approved. D1, ruled in R8, corrects one of its arguments,
  `argument@pre-release-fixes@a7`, in place
- **Arguments:** `argument@pre-release-fixes@a6`, `argument@pre-release-fixes@a7`,
  `argument@pre-release-fixes@a10`
- **Shape in:** Decided design, "What a safe fix is", and the mapping table
- **Harvest home:** the core's design home
- **Closed by:** R4: "check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks,
  fix-reports-what-it-wrote, index-stays approved."

### Install, then the gate of a writer, then the generated files, then the full check `##fix-before-the-checks`

- **Proposed by:** the agent, R3
- **Final state:** approved, with the rewritten clause of `design@core@model-then-checks`, and the
  rejected `--write` alternative not reopened
- **Arguments:** `argument@pre-release-fixes@a12`, `argument@pre-release-fixes@a13`
- **Shape in:** Decided design, "The order of a run"
- **Harvest home:** the core's design home: `design@core@model-then-checks` rewritten in place,
  and `design@core@phases-gate-the-report`
- **Closed by:** R4: "check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks,
  fix-reports-what-it-wrote, index-stays approved."

### A `--fix` run lists what it wrote or removed, and its exit code is the final check's `##fix-reports-what-it-wrote`

- **Proposed by:** the agent, R3
- **Final state:** approved. D2, ruled in R8, sets the exit code of a write that fails
- **Arguments:** `argument@pre-release-fixes@a14`
- **Shape in:** Decided design, "What a run prints"
- **Harvest home:** `path@core@README.md`, its `check` section and its exit-code table, in step 3.
  Whether the core's design home also takes an entry is decided by the recording tests at the
  harvest
- **Closed by:** R4: "check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks,
  fix-reports-what-it-wrote, index-stays approved."

### `index` and `install-agent-skills` stay as commands `##index-stays`

- **Proposed by:** the agent, R3
- **Final state:** approved
- **Arguments:** `argument@pre-release-fixes@a15`
- **Shape in:** Decided design, "The flag"
- **Harvest home:** none expected, since it keeps what exists. Whether the core's design home
  takes an entry is decided by the recording tests at the harvest
- **Closed by:** R4: "check-fix-flag, safe-fix-definition, fix-scope, fix-before-the-checks,
  fix-reports-what-it-wrote, index-stays approved."

### A separate command, `cargo klarch fix`, writes and then runs the check `##fix-command`

- **Proposed by:** the agent, R3, as the nearest rival of #check-fix-flag
- **Final state:** ruled-out
- **Arguments:** `argument@pre-release-fixes@a8`
- **Shape in:** Losing alternatives
- **Harvest home:** the core's rejected alternatives, if the recording tests say so
- **Closed by:** R4, the owner's approval of #check-fix-flag, after the agent asked for "the flag
  or a separate `fix` command": "check-fix-flag, safe-fix-definition, fix-scope,
  fix-before-the-checks, fix-reports-what-it-wrote, index-stays approved."

### A flag named for one fix, such as `--index` `##index-only-flag`

- **Proposed by:** the agent, R3, as the shape the owner's argument rules out
- **Final state:** ruled-out
- **Arguments:** `argument@pre-release-fixes@a5`, `argument@pre-release-fixes@a9`
- **Shape in:** Losing alternatives
- **Harvest home:** the core's rejected alternatives, if the recording tests say so
- **Closed by:** the owner's argument in R3, "it leans toward using a generic name for the cli
  option", and the approval of #check-fix-flag in R4, whose delta row read "ruled out: `--index`"

### Fix the generated files only `##generated-files-only`

- **Proposed by:** the issue check-cannot-regenerate-what-it-reports, before the discussion; named
  by the agent in R3 as the rival of #fix-scope
- **Final state:** ruled-out
- **Arguments:** `argument@pre-release-fixes@a7`, `argument@pre-release-fixes@a10`
- **Shape in:** Losing alternatives
- **Harvest home:** the core's rejected alternatives, if the recording tests say so
- **Closed by:** R4, the approval of #fix-scope: "check-fix-flag, safe-fix-definition, fix-scope,
  fix-before-the-checks, fix-reports-what-it-wrote, index-stays approved."

### `klarch --version` prints the package's version `##version-flag`

- **Proposed by:** the agent, R1, as item 2
- **Final state:** approved
- **Arguments:** `argument@pre-release-fixes@a2`
- **Shape in:** Decided design, "The version flag"
- **Harvest home:** `path@core@README.md`; no design entry expected, the recording tests decide
- **Closed by:** R2: "We'll do, in one PR, 1 and 2"

### `README.md` is rewritten for crates.io readers, and the repository-facing text moves to a new document `##readme-rewritten-for-crates-io`

- **Proposed by:** the agent: the question in R1, its default in R4, a new crates/core/docs/README.md
- **Final state:** ruled-out. It was approved in R5, then the owner reopened it in R6 by proposing
  the reverse, and chose the reverse in R7
- **Arguments:** `argument@pre-release-fixes@a1`, `argument@pre-release-fixes@a20`,
  `argument@pre-release-fixes@a21`, `argument@pre-release-fixes@a24`
- **Shape in:** Losing alternatives
- **Harvest home:** the root's rejected alternatives, if the recording tests say so
- **Closed by:** R5: "the default for the README is fine"; then R6: "Wait. Can't we do the reverse
  for the README ?"; then R7: "B with the three defaults."

### `README.md` stays repository-facing, and a separate file is the crates.io page `##crates-io-page-file`

- **Proposed by:** the owner, R6
- **Final state:** approved
- **Arguments:** `argument@pre-release-fixes@a1`, `argument@pre-release-fixes@a21`,
  `argument@pre-release-fixes@a22`,
  `argument@pre-release-fixes@a23`, `argument@pre-release-fixes@a24`,
  `argument@pre-release-fixes@a25`
- **Shape in:** Decided design, "The crates.io page"
- **Harvest home:** the root's design home, beside `design@knowledge-architect@package-include-whitelist`
- **Closed by:** R7: "B with the three defaults."

### The file is named CRATES-IO.md, beside each crate's `README.md` `##crates-io-file-name`

- **Proposed by:** the agent, R6, default 1 of shape B
- **Final state:** approved
- **Arguments:** none beyond the default
- **Shape in:** Decided design, "The crates.io page"
- **Harvest home:** with #crates-io-page-file
- **Closed by:** R7: "B with the three defaults."

### Each published crate gets the file, the gates crate included `##crates-io-file-per-referencing-crate`

- **Proposed by:** the agent, R6, default 2 of shape B, which gave the file to the core and the
  agent-skills crates only, and kept the gates crate's `README.md` as its page
- **Final state:** approved in R7 in that shape, then widened in R8 by D4: the gates crate gets a
  CRATES-IO.md too, so that row 202 and the setup default hold for all three crates. The slug
  keeps the R6 name
- **Arguments:** `argument@pre-release-fixes@a26`
- **Shape in:** Decided design, "The crates.io page"
- **Harvest home:** with #crates-io-page-file
- **Closed by:** R7: "B with the three defaults."; then R8, on D4: "All other defaults approved."

### Row 202 names `README.md` and the crates.io file `##row-202-rewrite`

- **Proposed by:** the agent, R6, default 3 of shape B
- **Final state:** approved, with the text shown in R6. D4, ruled in R8, makes it true for the
  gates crate too
- **Arguments:** `argument@pre-release-fixes@a23`
- **Shape in:** Decided design, "The crates.io page"
- **Harvest home:** the root `CLAUDE.md`, row 202, on the owner's word given in R7
- **Closed by:** R7: "B with the three defaults."

### The setup skill proposes, for a published Rust crate, a repository-facing `README.md` and a separate CRATES-IO.md `##setup-default-crates-io-page`

- **Proposed by:** the owner, R7
- **Final state:** approved. The owner's R7 words open with "Maybe", and the agent recorded the
  default as a decided thread in its R7 reply: "I'm adding your setup-skill default as a decided
  thread". The owner confirmed it in R8, as D5
- **Arguments:** `argument@pre-release-fixes@a27`
- **Shape in:** Decided design, "The setup skill's default"
- **Harvest home:** the setup skill's text, in step 5; the agent-skills design home if the recording
  tests say so
- **Closed by:** R8, on D5: "All other defaults approved." The proposal was R7's: "B with the
  three defaults. Maybe I'd also make it the default in the set-up skill: for published Rust
  crates, keep the README.md as a repo facing document, and use a separate CRATES-IO.md for what
  crates.io readers see."

### Option (a): the generic anchor counts Components only, as the design says `##generic-anchor-components-only`

- **Proposed by:** the agent, R3
- **Final state:** approved
- **Arguments:** `argument@pre-release-fixes@a4`, `argument@pre-release-fixes@a16`,
  `argument@pre-release-fixes@a17`
- **Shape in:** Decided design, "The generic anchor"
- **Harvest home:** none in a design home: `design@core@reserved-anchors` already says it. The issue
  closes
- **Closed by:** R4: "Fix the-generic-anchor-accepts-a-location-s-copy with (a)."

### Option (b): rewrite the design so that `*` means a copy in any declared anchor `##generic-anchor-any-anchor`

- **Proposed by:** the agent, R3, as the other way to close the issue
- **Final state:** ruled-out
- **Arguments:** `argument@pre-release-fixes@a4`, `argument@pre-release-fixes@a18`
- **Shape in:** Losing alternatives
- **Harvest home:** none expected. Whether `path@core@docs/rejected-alternatives.md` takes it is
  decided by the recording tests at the harvest
- **Closed by:** R4: "Fix the-generic-anchor-accepts-a-location-s-copy with (a)."

## Arguments

Every argument of the discussion, numbered in order of appearance. The boundaries were decided at
assembly. Instruments of the figures: the 33, 5 and 0 references of the core, agent-skills and
gates READMEs were counted at the root with ``grep -o '`[a-z*-]*@[^`]*`' <file> | wc -l``, re-taken
at the audit with the same result; "all 28 open issues" was
`cargo klarch issues` in R1; "all 409 tests pass" was `cargo test --workspace` in a scratch
worktree in R2, not re-taken at assembly.

### The core README holds 33 references that render on crates.io as dead code spans, the agent-skills README 5 more, and this fails the goal of easy adoption at the moment the release ships `##a1`

- **Round:** R1
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@readme-rewritten-for-crates-io`,
  `thread@pre-release-fixes@crates-io-page-file`
- **Key words:** "It now holds **33** references like `design@core@exit-code-ladder`, which render
  there as dead code spans." "crates/agent-skills/README.md, also published, holds 5 more, and the
  issue doesn't cover it." "This is the issue that most directly fails the new goal
  `adoption-is-easy`, at the exact moment the release ships it."

### A version flag is small, and lets a consumer or a retrospective ask the binary which version ran `##a2`

- **Round:** R1
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@version-flag`
- **Key words:** "The fix is small: a clap version flag, one test, a CHANGELOG `cli` entry." "A
  consumer running 0.2.0 next to 0.1.0, or a retrospective recording which version ran, can then
  ask the binary rather than infer it from the pin."

### The check flag in this release would spare every adopter the double run, and needs a design pass `##a3`

- **Round:** R1
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@check-fix-flag`
- **Key words:** "Shipping it in v0.2.0 would spare every adopter the `index`-then-`check` double
  run." "It needs a short design pass first: the flag's name, rewriting the \"Nothing writes to the
  tree while checking\" clause of `design@core@model-then-checks`, and an answer to the live
  rejected `--write` alternative."

### The generic anchor's code and design disagree, so a ruling on which is wrong comes before any fix `##a4`

- **Round:** R1
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@generic-anchor-components-only`,
  `thread@pre-release-fixes@generic-anchor-any-anchor`
- **Key words:** "the code and the design disagree, so it needs your ruling on which one is wrong
  before any fix."

### Tools of this kind offer a quick fix for what they can fix safely; more fixes may come, so the option takes a generic name `##a5`

- **Round:** R3
- **Who:** owner
- **Threads:** `thread@pre-release-fixes@check-fix-flag`, `thread@pre-release-fixes@index-only-flag`
- **Key words:** "IMO, it's quite common for tools similar as this one to provide a \"quick fix\"
  option, that performs the fixes it can safely do automattically. I think this is something we
  might want to propose in the future. Other than regenerating the indexes, I'm not certain whether
  any existing finding is elligible for a safe quick fix. This is an argument in favor of
  implementing the feature, and it leans toward using a generic name for the cli option."

### Two findings have a fully determined repair: a stale generated file and an installed file; every other repair touches git, hand-written text, or intent `##a6`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@safe-fix-definition`, `thread@pre-release-fixes@fix-scope`
- **Key words:** "The question is whether a finding's repair is fully determined, leaving nothing to
  choose." "**yes**: its bytes are a function of the tree" "**yes**: its bytes are a function of the
  pinned version, and a project never edits the installer's files" "no: it touches git, not files"
  "no: that file is yours, hand-written" "no: each repair is a choice"

### The install fix matters at upgrade: a fix flag covers the upgrade in one command, except when the install removes an unshipped file `##a7`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@fix-scope`, `thread@pre-release-fixes@generated-files-only`
- **Key words:** "Moving the pin to v0.2.0 makes every installed file differ. That is a phase-2
  finding, and `index` refuses until the install runs. A fix flag would cover the whole upgrade in
  one command."
- **Note:** the last sentence is true except for one case, corrected in place on the owner's
  ruling of D1 in R8. When the install removes a file the version no longer ships, the deletion is
  unstaged, and `check` in `path@core@src/check/agents.rs` reports it as a phase-2 finding that
  stops the run before the generated files. That upgrade takes `check --fix`, then `git add -A`,
  then `check --fix` again.

### The flag keeps the check one command with an option, and matches the precedent of clippy, eslint and ruff `##a8`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@check-fix-flag`, `thread@pre-release-fixes@fix-command`
- **Key words:** "Precedent: `cargo clippy --fix`, `eslint --fix`, `ruff check --fix`." "What
  separates them is that the check stays one command with an option, rather than two commands that
  both end in the same report. I recommend the flag: it is what you asked for, and it matches the
  precedent."

### A flag named for one fix needs another flag for the next fix `##a9`

- **Round:** R3
- **Who:** agent, from the owner's argument `argument@pre-release-fixes@a5`
- **Threads:** `thread@pre-release-fixes@index-only-flag`, `thread@pre-release-fixes@check-fix-flag`
- **Key words:** "**Ruled out by your argument:** a flag named for one fix (`--index`), since the
  next fix would need another flag."

### The install fix costs little, since `install-agent-skills` already exists `##a10`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@fix-scope`, `thread@pre-release-fixes@generated-files-only`
- **Key words:** "The installed-file fix costs little, since `install-agent-skills` already exists,
  and it is the one that makes an upgrade a single command."

### The definition of a safe fix is the test every future fix must pass, so it belongs in the design head `##a11`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@safe-fix-definition`
- **Key words:** "\"A fix is safe when its bytes are fully determined by the tree and the pinned
  version, and it writes or removes only files the tool generates or installs.\" This is the test
  any future fix must pass, so it goes in the design head."

### No check writes: every write happens between runs, before the model the checks read is built `##a12`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@fix-before-the-checks`
- **Key words:** "No check writes. Every write happens between runs, before the model the checks
  read is built." "That clause exists because the old implementation wrote and restored files
  inside a check, and this design still rules that out."

### The rejected `--write` alternative is not reopened: the check stays the only verifier `##a13`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@fix-before-the-checks`
- **Key words:** "It lost because a second command would answer \"is it current\" in its own
  format, beside the gate. Here the check stays the only verifier: `--fix` only writes before it
  verifies."

### A session sees what to commit; the exit code stays the check's; CI and `commits` take no `--fix` `##a14`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@fix-reports-what-it-wrote`
- **Key words:** "before the report, a `--fix` run lists each path it wrote or removed, so a session
  sees what to commit" "The exit code is the final check's. With nothing to fix, it is a plain
  `check`." "`commits` takes no `--fix`: it judges history. CI keeps running `check` without it."

### `index` and the install are named throughout the installed skills, and remain useful alone `##a15`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@index-stays`
- **Key words:** "They are named throughout the installed skills." "They remain useful when only one
  fix is wanted, for instance a reinstall with no check."

### The generic anchor accepts a location's copy: a false negative, on which no current text relies `##a16`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@generic-anchor-components-only`
- **Key words:** "The writer claims \"every Component has this file\", and the checker confirms a
  claim that is false for all five Components. It is a false negative of the checker. No current
  text relies on it: I restricted the rule to Components in a scratch worktree, and this
  repository's check still passes and all 409 tests pass."

### Option (a) is about ten lines and a test; a project that relied on it gets a new finding, a minor-version change `##a17`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@generic-anchor-components-only`
- **Key words:** "add `&& a.is_component` to one filter, plus a test planting the reproduction the
  issue gives. This is about 10 lines and a test. Risk for consumers: a project that wrote
  path@*@ for a location's file gets a new finding. That is a stricter check, so it is a
  minor-version change, and it fits v0.2.0."

### Option (b) weakens what the generic form claims `##a18`

- **Round:** R3
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@generic-anchor-any-anchor`
- **Key words:** "`*` would mean \"a copy in any declared anchor\". That weakens what the form
  claims, so I see no argument for it."

### Reversing the flag would touch the CLI, an interface consumers use, so the design takes the full path and a spec `##a19`

- **Round:** R4
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@check-fix-flag`
- **Key words:** "Reversing this decision would touch an interface consumers use, the CLI. So the
  design skill's full path applies, and the planning skill now writes a spec"

### The default home of the repository-facing text is a new document under the core's docs `##a20`

- **Round:** R4
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@readme-rewritten-for-crates-io`
- **Key words:** "For item 1 the spec still needs your ruling on one point: where the
  repository-facing text of the core README goes once the README is written for crates.io readers.
  My default is a new crates/core/docs/README.md."

### Keep the Component's README for this project's workflow, and use a separate document for crates.io readers `##a21`

- **Round:** R6
- **Who:** owner
- **Threads:** `thread@pre-release-fixes@crates-io-page-file`,
  `thread@pre-release-fixes@readme-rewritten-for-crates-io`
- **Key words:** "Wait. Can't we do the reverse for the README ? keep the component level README
  matching this project's workflow, and use a separate README document for crates.io readers ? Is
  that possible ?"

### No crate sets `readme`, so the crates.io page is `README.md` only by Cargo's default `##a22`

- **Round:** R6
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@crates-io-page-file`
- **Key words:** "Neither crate sets `readme` in its Cargo.toml. So the crates.io page is
  `README.md` only because that is Cargo's default."

### No design head ties the README to crates.io; only row 202 does `##a23`

- **Round:** R6
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@crates-io-page-file`, `thread@pre-release-fixes@row-202-rewrite`
- **Key words:** "No design head ties the README to crates.io. The one place that does is your root
  CLAUDE.md, row 202 of the knowledge table"

### Under A the move costs the most and changes what `README.md` means; under B nothing moves `##a24`

- **Round:** R6
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@crates-io-page-file`,
  `thread@pre-release-fixes@readme-rewritten-for-crates-io`
- **Key words:** "Under A, the move costs the most and changes what `README.md` means as a required
  document of every Component. Under B, nothing moves, and the new file is still checked: it sits
  in the walked tree, so a backticked reference or path in it is reported. The checker itself
  keeps the page free of dead references."
- **Note:** the second half of the key words is false, as found at the audit (D6, ruled in R8).
  A backticked reference that resolves passes `cargo klarch check`. The findings of
  `path@core@src/check/references.rs` are an unresolved reference, the two retired forms, and a
  backticked path span of two or more segments with no at sign. So the checker does not keep the
  page free of references that render as dead code spans on crates.io.

### B's one risk is two usage texts drifting apart, bounded by a short crates.io file that restates no contract `##a25`

- **Round:** R6
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@crates-io-page-file`
- **Key words:** "**The one risk of B is two texts describing usage that drift apart.** To bound
  it, I'd keep the crates.io file short" "what the crate is;" "how to install and run it;" "links,
  as absolute URLs, to the repository's README on GitHub and to docs.rs." "It would not restate the
  contract that `README.md` holds, so there is little to drift."

### The core and agent-skills READMEs hold references, the gates README none `##a26`

- **Round:** R6
- **Who:** agent
- **Threads:** `thread@pre-release-fixes@crates-io-file-per-referencing-crate`
- **Key words:** "The core README holds 33 references and the agent-skills README holds 5, so both
  get a crates.io file. The gates README holds none, so it stays its own crates.io page."

### Make the separate crates.io file the setup skill's default for published Rust crates `##a27`

- **Round:** R7
- **Who:** owner
- **Threads:** `thread@pre-release-fixes@setup-default-crates-io-page`
- **Key words:** "Maybe I'd also make it the default in the set-up skill: for published Rust
  crates, keep the README.md as a repo facing document, and use a separate CRATES-IO.md for what
  crates.io readers see."

## New names, in one place

An illustration of the shapes. Each name marked new is the author's choice, to change at
implementation if the code argues for another.

```text
path@core@src/cli/mod.rs
  Command::Check(CheckArgs)        changed: the unit variant Check gains its arguments (D3)
  CheckArgs { fix: bool }          new: `--fix`, a clap `Args` struct beside ShowArgs
  check(...)                       existing: gains the --fix run before its current body
  index(...)                       existing: takes the generated list from generated_list, keeps
                                   its own write loop and its own printed lines
  complete_working_tree(...)       existing: the gate of a writer, used by `index` only; --fix
                                   calls crate::check::foundation itself (see "The order of a run")
  install_agent_skills(...)        existing
  check_destinations(...)          new: the refusals of a missing directory and of a symlink,
                                   moved out of `index`, over every destination before any write
  generated_list(...)              new: the generated list, destination and bytes: each prepared
                                   extension's `generated`, then file_register_indexes. Used by
                                   `index` and by --fix
path@core@src/index.rs
  file_register_indexes(...)       existing: also read by the `generated` check
path@core@src/check/mod.rs
  foundation(...), run_with(...)   existing
path@core@src/agents.rs
  install(...), shipped(...)       existing
path@core@src/extension.rs
  Purpose::Index, Purpose::Check   existing
path@core@src/main.rs
  #[command(version)] on Cli       new: clap's version flag, from the package version
crates/core/Cargo.toml, crates/agent-skills/Cargo.toml, crates/gates/Cargo.toml
  readme = "CRATES-IO.md"          new key; `include` is unchanged (see Readings)
crates/core/CRATES-IO.md           new file
crates/agent-skills/CRATES-IO.md   new file
crates/gates/CRATES-IO.md          new file (D4)
path@core@tests/
  an extension whose `generated`   new test fixture, for P2: called through the library's
  returns a file                   `cli::run`; FirstHeading in tests/extension_api.rs returns
                                   nothing from `generated`
  a copy of a mock project with a  new test fixture, for P1
  stale index and a missing
  required document
path@core@docs/tripwires.md
  safe-fix-definition              new tripwire slug, T1, written at the harvest
```

## Decided design

### The flag

`cargo klarch check --fix` applies every safe fix, then runs the full check. Without `--fix`,
`check` is unchanged. `index` and `install-agent-skills` stay as commands (#index-stays). The flag
takes a generic name because a later safe fix needs no new option (#later-fix-needs-no-option).

- **Argument:** `argument@pre-release-fixes@a5`, `argument@pre-release-fixes@a8`.
- **Nearest rival:** a separate command, `cargo klarch fix` (#fix-command). It meets every
  criterion too. **The fact that defeated it:** with the flag, the check stays one command with an
  option, rather than two commands that end in the same report; and the owner asked for an option
  on `check`.

### What a safe fix is

"A fix is safe when its bytes are fully determined by the tree and the pinned version, and it
writes or removes only files the tool generates or installs." This is the test every later fix
must pass, and T1 guards it. Two fixes pass it today: the install, and the generated list
(#fix-scope). The mapping table gives every finding's fix.

- **Argument:** `argument@pre-release-fixes@a6`, `argument@pre-release-fixes@a11`.
- **Nearest rival of the scope:** the generated files only (#generated-files-only), as the issue
  proposed. **The fact that defeated it:** the install fix costs little, since the install exists,
  and it removes the install-then-index sequence from an upgrade (`argument@pre-release-fixes@a10`).
  One upgrade still takes two runs, on the owner's ruling of D1 in R8: see "The order of a run".

### The order of a run

An illustration of the order the owner approved, as shown in R3:

```text
phase 1 (manifest) has findings  → write nothing, report them, as today
install-agent-skills             → when phase 1 is clean
rebuild the model; phases 1–3    → if any finding remains, write nothing more, report
write the stale generated files  → the same list `index` and the gate read
rebuild the model; full check    → the report and the exit code, as without --fix
```

- **The install runs only when it has something to fix**: the manifest serves an agent harness,
  as `install_agent_skills` decides today, and the installed-file check of
  `path@core@src/check/agents.rs` reports a shipped file missing or differing, or an unshipped
  file in the namespace. The reason is line endings. That check compares after normalising them,
  through `crate::agents::lf`, while `crate::agents::install` in `path@core@src/agents.rs`
  compares raw bytes. An install run on every `--fix` would rewrite every installed file of a
  checkout that converts line endings, on every run.
- **An ignored shipped file stays reported.** When an ignore rule covers a shipped file, the
  install writes it, git still lists no copy, and the final check reports it, as it does without
  `--fix`.
- **The gate after the install is run by `--fix` itself.** It calls `crate::check::foundation`
  over the rebuilt model. On a stop it prints what it wrote, then the stopped report, and exits 1.
  It does not call `complete_working_tree`, whose error says "Nothing was written" and exits 2, and
  it never passes that error through.
- **The generated files** are written only once that gate passes. `index` and `--fix` take the
  list from one function, `generated_list` in New names. The `generated` check, through
  `crate::check::run_with`, assembles its list from the same two sources,
  `file_register_indexes` and each prepared extension's `generated`, which is what
  `design@core@generated-files-are-pure` asks and what keeps #one-list-of-generated-files met.
  Moving the gate onto `generated_list` would change the check's inputs and no behaviour. The refusals of `index`, a missing directory and a symlink,
  are checked over every destination before any is written, through the same function
  (`check_destinations`). Each caller keeps its own write loop and prints its own lines: `index`
  prints each path from inside its loop, as today.
- **An upgrade that removes a shipped file takes two runs**, on the owner's ruling of D1 in R8.
  The install deletes the file, the deletion is unstaged, and the installed-file check reports it
  as a phase-2 finding, "this file of the installer's namespace is deleted, and the deletion is not
  staged". The gate stops the run before the generated files. The sequence is `check --fix`, then
  `git add -A`, then `check --fix` again. The README's `check` section says so. `--fix` does not
  stage the deletion: that touches git, which fails #safe-fix-definition
  (`argument@pre-release-fixes@a6`).
- An extension is prepared with `Purpose::Index` for the generated list and again with
  `Purpose::Check` for the final check. P2 tests that the two agree.
- **The rewritten clause** of `design@core@model-then-checks`: "No check writes to the tree;
  `check --fix` writes before the model its checks read is built".
- **The rejected `--write` alternative is not reopened.** It lost because a second command would
  answer "is it current" in its own format beside the gate. Here the check stays the only
  verifier.
- **Argument:** `argument@pre-release-fixes@a12`, `argument@pre-release-fixes@a13`.
- **Nearest rival:** none was argued in the discussion. The issue's own shape wrote nothing when
  phases 1 to 3 held a finding; the approved order differs in running the install whenever phase 1
  is clean, since the install's bytes do not depend on the model.

### What a run prints

Before the report, a `--fix` run lists each path it wrote or removed, so a session sees what to
commit. An illustration, as shown in R3:

```text
fixed: wrote crates/core/docs/open-issues/index.md (regenerated)
fixed: wrote .claude/agents/knowledge-architect-transcript-reviewer.md (installed)
fixed: removed .claude/agents/knowledge-architect-transcript-conformity-reviewer.md (installed)

checked: generated, registers, references
...
PASSED: no findings
```

- `fixed: wrote <path> (regenerated)` is printed only for a generated file the run rewrote. An
  installed file the install wrote prints `fixed: wrote <path> (installed)`, and one it deleted
  prints `fixed: removed <path> (installed)`.
- The exit code is the final check's. With nothing to fix, the run is a plain `check`.
- A run stopped at phase 1, or at the gate after the install, prints what it wrote, then the
  stopped report, and exits 1, as `check` does over the same phases.
- **A write that fails** follows the rule of `index`, on the owner's ruling of D2 in R8: exit 2
  when nothing was written yet, exit 1 after any write, the install's writes counting as writes.
  The error names the path. `agents::install` in `path@core@src/agents.rs` returns no list of
  what it wrote when it fails part-way, so it gains one: on an error it returns the paths it
  already wrote or removed. `--fix` prints them as for a run that succeeded, then the error, and
  the rule above decides the exit code. Without that list, neither this rule nor the listing of
  #fix-reports-what-it-wrote could be met on that path.
- **The repair texts of the findings stay as they are**: "run `<command> index`" in
  `path@core@src/check/generated.rs`, and "run `<command> install-agent-skills`" in
  `path@core@src/check/agents.rs`. Both commands stay, per #index-stays, so each repair still
  names a command that exists.
- `commits` takes no `--fix`. The gates, and the CI workflow the setup skill gives, run `check`
  without it (P4).
- **Argument:** `argument@pre-release-fixes@a14`.

### The version flag

`klarch --version` prints the package's version, through clap's `version` attribute on `Cli` in
`path@core@src/main.rs`, which reads the package version Cargo sets at build time. A test asserts
the output names the version of the package `knowledge-architect`. The `Cli` is parsed before the
project is located, so `--version` answers from anywhere, as `--help` does. An extension's binary
declares its own `Cli`, so it gains a version flag only by its own declaration.

- **Argument:** `argument@pre-release-fixes@a2`.
- **Nearest rival:** none was argued.

### The crates.io page

- `README.md` of every crate stays as it is, for this repository's reader, references included
  (#crates-io-page-file).
- Each of the three crates, core, agent-skills and gates, gains CRATES-IO.md beside `README.md`,
  named by `readme = "CRATES-IO.md"` in its Cargo.toml (#crates-io-file-name,
  #crates-io-file-per-referencing-crate). The gates crate is included on the owner's ruling of D4
  in R8.
- Each CRATES-IO.md stays short and restates no contract that `README.md` holds, to bound drift
  (`argument@pre-release-fixes@a25`). It holds:
  - what the crate is;
  - how to install and run it. For the agent-skills crate, which has no command of its own, this
    is the `install-agent-skills` command of the checker that embeds it;
  - links, as absolute URLs that name the main branch, to the crate's `README.md` on GitHub, of
    the shape https://github.com/tellurium-monoxide/knowledge-architect/blob/main/crates/core/README.md,
    and to docs.rs.
- **The pages hold no backticked reference, by convention, and review checks it** (D6, ruled in
  R8). The file sits in the walked tree, but the checker does not keep it free of references. A
  backticked reference that resolves passes `cargo klarch check`. The findings of
  `path@core@src/check/references.rs` are an unresolved reference, the two retired forms, and a
  backticked path span of two or more segments with no at sign. A relative markdown link in the
  file is a finding, since only a `README.md` or an `index.md` may hold one, per
  `design@core@links-are-navigation-rows`. At the harvest,
  `issue@agent-skills@shipped-text-is-reference-free-mechanically` is extended to name these
  pages as needing the same mechanism.
- Each CRATES-IO.md ships in its package with no change to `include`: Cargo packages the file
  that `readme` names even when `include` does not list it (see Readings).
  `cargo package --list -p <crate>` shows it.
- Row 202 becomes the text the owner approved in R7, as proposed in R6: "| how a user can use a
  published crate, beyond the primer's README row | its `path@*@README.md`; its crates.io page is a
  short `CRATES-IO.md` in the crate, named by `readme` in its Cargo.toml, which points to the README
  and to docs.rs | the contract changes |" (#row-202-rewrite). With D4, ruled in R8, it holds for
  all three crates. The edit is made in step 4.
- **Argument:** `argument@pre-release-fixes@a21` to `argument@pre-release-fixes@a26`.
- **Nearest rival:** shape A (#readme-rewritten-for-crates-io). **The fact that defeated it:** A
  moves the repository-facing text and changes what `README.md` means as a required document of
  every Component, while B moves nothing; the owner chose B in R7.

### The setup skill's default

`path@agent-skills@content/skills/setup/SKILL.md`, in its section "In a Rust project", says: for a
published Rust crate, keep `README.md` as the repository-facing document, and give crates.io a
separate CRATES-IO.md, named by `readme` in the crate's Cargo.toml (#setup-default-crates-io-page).
The owner confirmed this default in R8, as D5.

- The text is installed text, under `path@agent-skills@CLAUDE.md`: it names no path or convention
  of this repository, holds no live reference, and writes an illustration as a placeholder in angle
  brackets. The edit is installed in the same commit, with `cargo klarch install-agent-skills`.
- **The three tests of that file**, applied at assembly. Scope: the setup skill has no expectation
  set in §5 of the retrospective skill. Necessity: the observation is this session's count of 33
  dead references on the core's crates.io page; the owner named the lack in R7; the mechanism is
  that Cargo's default `readme` makes the repository-facing `README.md` the crates.io page. Kind:
  a missing capability.
- A project that adopts the default keeps a crates.io page somewhere the primer's table does not
  name, and §5 of the same skill already asks for the project's own row for such a statement.
- **Argument:** `argument@pre-release-fixes@a27`.

### The generic anchor

The filter of the `*` arm in the function `path` of `path@core@src/check/references.rs` keeps the
anchors that are Components, `a.is_component`, in place of `a.constructed.is_none()`. A
constructed anchor is not a Component, so the plans anchors stay excluded. The comment above the
filter loses its citation of the issue and its sentence "A declared location still counts".

- A test over the issue's reproduction asserts that a generic reference to a.md, in a copy of the
  minimal mock where only the location `notes` holds a.md, is reported as resolving in no component, and that
  the required-document case still passes.
- **Argument:** `argument@pre-release-fixes@a16`, `argument@pre-release-fixes@a17`.
- **Nearest rival:** option (b) (#generic-anchor-any-anchor). **The fact that defeated it:** it
  weakens what the form claims, and no argument for it was found.

## Mapping tables

**Every finding of `check` to its fix under `--fix`.** The domain is every finding the checker
reports; the codomain is a fix or none. From the table the agent showed in R3, checked against
`path@core@src/check/agents.rs`.

| finding | phase | fix |
| --- | --- | --- |
| a manifest declaration the tool refused | 1 | none: the run writes nothing |
| a shipped installed file missing | 2 | the install |
| an installed file whose bytes differ from what the version ships | 2 | the install |
| a file of the installer's namespace that the version does not ship | 2 | the install, which deletes it |
| the deletion of an installed file not staged | 2 | none: it touches git, not files (D1) |
| the root `CLAUDE.md` not importing the primer, or outside the walk | 2 | none: the file is the project's |
| every other phase-2 finding: an unreadable file, a missing home, a refused name, a tracked-and-ignored file | 2 | none |
| a slug or an entry id where none may sit, or defined twice | 3 | none |
| a generated file stale or missing | 4 | regenerate it, from the generated list |
| every finding of `registers` and `references`, and every extension check's | 4 | none: each repair is a choice |

## Losing alternatives

| alternative | lost to | the fact that decided it |
| --- | --- | --- |
| #fix-command, a separate `cargo klarch fix` | #check-fix-flag | the check stays one command with an option; the owner asked for an option on `check` (`argument@pre-release-fixes@a8`) |
| #index-only-flag, a flag named for one fix | #check-fix-flag | the next fix would need another flag (`argument@pre-release-fixes@a5`, `argument@pre-release-fixes@a9`) |
| #generated-files-only | #fix-scope | the install fix costs little and serves an upgrade (`argument@pre-release-fixes@a10`) |
| #readme-rewritten-for-crates-io, shape A | #crates-io-page-file | A moves the repository-facing text and changes what `README.md` means; B moves nothing (`argument@pre-release-fixes@a24`) |
| #generic-anchor-any-anchor, option (b) | #generic-anchor-components-only | it weakens what the generic form claims (`argument@pre-release-fixes@a18`) |

The rejected alternative "`cargo klarch index` prints the diff it would apply, and `--write`
applies it", in `path@core@docs/rejected-alternatives.md`, is not reopened, per
`argument@pre-release-fixes@a13`.

## Readings

Two external specifications, each read at implementation and confirmed by a run:

- **Cargo's manifest key `readme`**: a path relative to Cargo.toml, packaged as the crate's README,
  which crates.io renders. **Cargo packages that file even when `include` does not list it.** A
  probe at the audit settled it: in a scratch copy of the gates crate, with
  `readme = "CRATES-IO.md"` and the crate's `include` list unchanged, `cargo package --list`
  listed CRATES-IO.md. So `include` needs no change. Confirmed again by
  `cargo package --list -p <crate>` for each crate in step 4, and by `cargo publish --dry-run` at
  the release.
- **clap's `version` attribute** of the derive: it adds `--version` and `-V`, printing the name and
  the package version. Confirmed by the test of step 2.

Neither reading is recorded elsewhere; the commit that lands each step names the run.

## Premortem

Run in R4, over `check --fix` only. "Suppose `check --fix` shipped and failed."

| # | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| P1 | a fix writes generated files over a model a later phase finds incomplete | #fix-before-the-checks | survives as a claim: a test plants a phase-2 finding and asserts no generated file is written. Its fixture is an addition: a copy of a mock project under `path@core@tests/projects/` with a stale index and a missing required document |
| P2 | an extension's generated file is written, but the extension prepares twice, once to generate and once to check, and the second run sees a different tree | #fix-before-the-checks | survives as a claim: a test with an extension's generated file asserts the `--fix` run passes. Its fixture is an addition: a test extension whose `generated` returns a file, called through the library's `cli::run` in a test under `path@core@tests/`. `FirstHeading` in `path@core@tests/extension_api.rs` returns nothing from `generated` |
| P3 | a later fix is added that makes a choice, such as adding a trailing slash, because the option invites growth, and it rewrites what a writer meant | #safe-fix-definition | becomes tripwire T1, on the owner's word |
| P4 | a session runs `check --fix` in CI or in the gates, so a stale index passes CI without being committed | #check-fix-flag | survives as a claim: the gate list of `rust_project` in `path@gates@src/gates.rs` runs `check` without `--fix`, and its test lives there. The setup skill's CI workflow runs `cargo --locked x gates`, so it inherits the gates' `check` |
| P5 | the install fix overwrites an installed file a project had edited | #fix-scope | survives: this is what `install-agent-skills` already does, and the check already reports such an edit as a finding |

**T1**, as proposed in R4: guarding #safe-fix-definition, "it fires when a fix is proposed or added
whose bytes are not fully determined by the tree and the pinned version, or that writes a file the
tool neither generates nor installs. Its response: reopen the decision before adding the fix." The
owner ruled in R5: "Record T1". It is written at the harvest, in `path@core@docs/tripwires.md`,
naming the head that harvests #safe-fix-definition, under `knowledge-architect-issue-tracking`.

## Acceptance criteria

### `check --fix` makes an edit cycle one command `##edit-cycle-is-one-command`

- **Guards:** `thread@pre-release-fixes@fix-before-the-checks`, `thread@pre-release-fixes@check-fix-flag`
- **Judged at:** step 3, in a scratch worktree of the branch, never the live tree: an issue entry
  added by hand, then one `cargo klarch check --fix`
- **Fires when:** that run does not exit 0, or a following `cargo klarch check` does not exit 0
- **Response:** reopen #fix-before-the-checks with the run's output

### A crates.io page stays short and restates no contract `##crates-io-page-stays-short`

- **Guards:** `thread@pre-release-fixes@crates-io-page-file`
- **Judged at:** this spec's harvest, over the pages step 4 writes. No later judgement is
  scheduled
- **Fires when:** a CRATES-IO.md exceeds 40 lines, a default that is the owner's to reset, or
  restates a command's flags or exit codes that its `README.md` holds
- **Response:** reopen #crates-io-page-file with the drift `argument@pre-release-fixes@a25` named

### No text of this repository relies on a location's copy under the generic anchor `##no-text-relies-on-location-copy`

- **Guards:** `thread@pre-release-fixes@generic-anchor-components-only`
- **Judged at:** step 1, by `cargo klarch check` over this repository with the fix built
- **Fires when:** the check reports a reference that "resolves in no component"
- **Response:** re-anchor each at the location that holds it, and tell the owner before the merge,
  since it is evidence the R3 measurement missed

## Implementation sequence

Five steps inside this one pull request, then the harvest. Each step is one commit or more, and
each commit passes the gates. Each step's commit reports the acceptance criterion that step judges,
by its identifier in plain text.

1. **The generic anchor.** The filter, the test over the reproduction, the comment, a CHANGELOG
   entry under Migration: `checks`, minor: a `path@*@<path>` reference that only a location
   carries is reported; anchor it at the location. The issue
   the issue the-generic-anchor-accepts-a-location-s-copy closes in this commit. It judges
   no-text-relies-on-location-copy. Fails alone on: the new test, or a new finding in this
   repository's check. **The per-commit rule holds.** This step makes a check stricter, so it lands
   as a commit whose tree passes under its own checker. No later commit's change fails it: the
   later steps write no generic reference that only a location carries, and the `commits` gate
   judges each commit's tree with the tip's checker, which holds the same rule.
2. **`--version`.** The attribute on `Cli`, a test of the output, the exit-code table and the
   command list of `path@core@README.md`, a CHANGELOG entry under New features: `cli`, minor. Fails
   alone on: the test, or `Cli::command().debug_assert()` in `path@core@src/main.rs`.
3. **`check --fix`.** `CheckArgs`, `generated_list` and `check_destinations` shared with `index`,
   the order of a run, the printed list, the tests below, the `check` section and the exit-code
   table of `path@core@README.md` (the two-run upgrade of D1 included), a CHANGELOG entry under New
   features, `cli`, minor, and the entry D3 rules, under Migration, `library`, major. It judges
   edit-cycle-is-one-command. Claims and their tests:
   - a stale index is written and the run passes;
   - a planted phase-2 finding writes no generated file (P1), over the added fixture named there;
   - an extension's generated file is written and the run passes (P2), over the added fixture
     named there;
   - a differing installed file is installed;
   - an installed file that differs from the shipped text only by line endings is not rewritten;
   - with nothing to fix, the output is a plain `check`'s;
   - the gate list of `rust_project` in `path@gates@src/gates.rs` runs `check` without `--fix`
     (P4);
   - `--fix` is refused by `commits`;
   - the setup skill's Rust snippet, in its section "In a Rust project", still compiles against
     the workspace after `Command::Check` changes: copied into a scratch directory and built with
     `cargo check`, exit 0.

   Fails alone on: those tests.
4. **The crates.io pages.** The three CRATES-IO.md files, core, agent-skills and gates, the
   `readme` keys, `cargo package --list -p` for each crate showing the file, and row 202, under
   `knowledge-architect-agent-configuration` §4, on the owner's word given in R7. `include` is
   unchanged. Fails alone on: the package list, or a finding of `cargo klarch check` in a new
   file.
5. **The setup skill's default.** The edit under content/, the install, the installed copy
   committed with it, a CHANGELOG entry under Workflow: `agent-skills`, patch. Fails alone on: the
   check comparing the installed copy with the shipped text.

Then the review under `knowledge-architect-review`, and the harvest.

## Order rationale

- 1 before 2: the generic anchor is the smallest change and the only stricter check; landing it
  first separates a new finding in this repository from any later change.
- 2 before 3: `--version` changes only the binary's declaration in `path@core@src/main.rs`, and 3
  changes the `Command` enum that `Cli` holds as its subcommand field (`#[command(subcommand)]`);
  2 first keeps each failure to one declaration.
- 3 before 4: 3 changes the core's code and 4 changes no code, so a failure in 4's package lists
  cannot come from the flag.
- 4 before 5: the setup skill's default describes the shape 4 builds in this repository first.

## Defaults awaiting the owner

No default awaits the owner. The seven defaults the reviews of this spec and its audit produced
were ruled in R8: "D1: go with (i), skill deletion/rename is a rare thing anyway. Not much of a
problem if it takes multiple invocations to handle. All other defaults approved. proceed." Each is
applied in the sections it touches.

**Ruled:**

- **D1 (i), bears on #fix-scope and #one-command-per-edit-cycle.** The decided shape is kept. An
  upgrade where the install removes an unshipped file takes `check --fix`, then `git add -A`, then
  `check --fix` again. The unstaged deletion is a phase-2 finding of
  `path@core@src/check/agents.rs`, which stops the run before the generated files. The README's
  `check` section says so. `argument@pre-release-fixes@a7`, "A fix flag would cover the whole
  upgrade in one command", is corrected in place as true except for that case. The rival, `--fix`
  staging the deletion, fails #safe-fix-definition, since it touches git
  (`argument@pre-release-fixes@a6`).
- **D2, bears on #fix-reports-what-it-wrote.** The exit code of a `--fix` run whose write fails
  follows the rule of `index`: 2 when nothing was written yet, 1 after any write, the install's
  writes counting as writes.
- **D3, bears on #check-fix-flag.** `Command::Check` becomes `Command::Check(CheckArgs)`. Code that
  names the variant, such as an extension's binary that constructs or matches it, fails to
  compile; no code in this repository does besides `run` in `path@core@src/cli/mod.rs`. A
  CHANGELOG entry under Migration, `library`, major, says what such code writes instead. Under
  0.x a major takes the same version number as the minor entries of this release, per
  `design@knowledge-architect@versioning-policy`.
- **D4, bears on #crates-io-file-per-referencing-crate and #row-202-rewrite.** The gates crate gets
  a CRATES-IO.md too, and its `README.md` stays repository-facing. Row 202 of the root `CLAUDE.md`
  is edited, in step 4, to the text the owner approved in R7, which then holds for all three
  crates.
- **D5, bears on #setup-default-crates-io-page.** The setup-skill default is decided: for a
  published Rust crate, the setup skill proposes a repository-facing `README.md` and a separate
  CRATES-IO.md named by `readme`. The thread is approved and closed by R8; the R7 "Maybe" stays in
  its record.
- **D6, bears on #crates-io-page-file.** `argument@pre-release-fixes@a24`'s claim that the new
  file "is still checked" is false: a reference that resolves passes `cargo klarch check`. Shape B
  is kept. The pages hold no backticked reference by convention, checked by review.
  `issue@agent-skills@shipped-text-is-reference-free-mechanically` is extended at the harvest to
  name these pages as needing the same mechanism.
- **D7, bears on #fix-before-the-checks.** `tripwire@core@phases-gate-the-report-two` is judged at
  this audit, before the code, and does not fire. The install's bytes come from the pinned version
  and the installer's file list, not from the model, and `install_agent_skills` in
  `path@core@src/cli/mod.rs` already writes after checking only the manifest's complaints. The
  generated files, whose bytes depend on the model, stay behind the full gate of phases 1 to 3.

## Harvest

At the end of the branch, before the merge, under `knowledge-architect-decision-recording` and then
`knowledge-architect-issue-tracking`. The recording tests decide which approved thread earns an
entry, and which loser earns a rejected alternative; an item they exclude is named in the harvest's
commit with the test it fails.

| what | where | when |
| --- | --- | --- |
| #check-fix-flag, #safe-fix-definition, #fix-scope, #fix-before-the-checks | `path@core@docs/design.md`; a head takes its thread's slug | the harvest |
| #fix-reports-what-it-wrote | `path@core@README.md`, its `check` section and its exit-code table; `path@core@docs/design.md` if the recording tests say so | step 3 and the harvest |
| #index-stays | none expected; `path@core@docs/design.md` if the recording tests say so | the harvest |
| the clause "Nothing writes to the tree while checking" | rewritten in place in `design@core@model-then-checks` | the harvest |
| `check --fix` among the writers | `design@core@phases-gate-the-report`; `design@core@owned-namespace-check` judged | the harvest |
| #fix-command, #index-only-flag, #generated-files-only | `path@core@docs/rejected-alternatives.md` if the recording tests say so, each naming the head it lost to | the harvest |
| T1, guarding the head of #safe-fix-definition | `path@core@docs/tripwires.md`, after the head is written | the harvest |
| `tripwire@core@phases-gate-the-report-two` | judged at this audit, before the code: it does not fire (D7); the audit's commit says so | the audit |
| #version-flag | `path@core@README.md`; a design entry only if the tests say so | step 2 and the harvest |
| #crates-io-page-file, #crates-io-file-name, #crates-io-file-per-referencing-crate | `path@knowledge-architect@docs/design.md`; `design@knowledge-architect@package-include-whitelist` rewritten | the harvest |
| the convention that a CRATES-IO.md holds no backticked reference (D6) | `issue@agent-skills@shipped-text-is-reference-free-mechanically` extended to name these pages as needing the same mechanism | the harvest |
| #readme-rewritten-for-crates-io | `path@knowledge-architect@docs/rejected-alternatives.md`, if the tests say so | the harvest |
| #row-202-rewrite | the root `CLAUDE.md`, row 202 | step 4 |
| the restatement of the package lists | `path@agent-config@skills/klarch-release/SKILL.md` line 69, under `knowledge-architect-agent-configuration` | the harvest |
| #setup-default-crates-io-page | the setup skill's text; `path@agent-skills@docs/design.md` if the tests say so | step 5 and the harvest |
| the measurement taken before the work: the setup skill's Rust snippet compiled against the workspace with `cargo check`, exit 0 | `issue@agent-skills@the-setup-snippet-is-unchecked` | the harvest |
| #generic-anchor-components-only | no design entry: `design@core@reserved-anchors` already states the winner | step 1 |
| #generic-anchor-any-anchor | none expected; `path@core@docs/rejected-alternatives.md` if the recording tests say so | the harvest |
| the issue the-generic-anchor-accepts-a-location-s-copy | closes | step 1 |
| `issue@core@installed-file-findings-belong-in-phase-four` | its "Why it matters" rewritten to what remains | the harvest |
| the acceptance criteria | each reported on in the landing commit by its identifier in plain text; each one that recurs is proposed to the owner as a tripwire | the harvest |
| this spec | deleted in the commit that completes the harvest, cited as `spec@plans@pre-release-fixes` | the harvest |

The three todo issues close in the commit that adds this spec, per planning §2.

## Later consequences

- **A later safe fix** joins `check --fix` with no new option, and passes #safe-fix-definition
  first; T1 watches it.
- **The installed skills that name `index`**, the setup skill's §7, the issue-tracking skill and
  the planning skill, keep naming it, per #index-stays. The primer does not name it. A later change may name `check --fix` there, and would be a Workflow entry.
- **`issue@core@installed-file-findings-belong-in-phase-four`** stays open. Its move to phase 4
  would let `index` run beside a stale install, and would let `check --fix` regenerate before the
  unstaged deletion of D1 is staged.
- **An extension's binary** that holds `Command` as a subcommand field, as `Cli` in
  `path@core@src/main.rs` does, gains `--fix` with no change of its own, and pays D3's migration
  if it names `Command::Check`. It gains `--version`
  only by its own `Cli` declaration.
- **A consumer project** that publishes Rust crates meets the setup default the next time it runs
  the setup skill; it is not asked to change anything at upgrade.
