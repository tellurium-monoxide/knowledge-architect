# The manifest pins the checker's version, and a binary of another version refuses to run

## Status and audience

This spec is the plan of the work that makes the manifest declare the version of the checker a
project pins: the key is required in every manifest, holds a version, or one of two values a
fixture or the checker's own repository uses, each accepted only where the running binary's build
confirms it; every binary refuses to run a command over the working tree when what the key says
does not hold. It is written for a session that did not witness the design discussion that
produced it. It leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the code.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec and its work are one branch and one pull request**, per
  `design@agent-skills@milestone-lands-before-gate-change`, which gives a spec no split. The work
  changes the manifest format: under the tip checker, a tree whose manifest lacks the key is
  refused. So step 1's commit carries the format change with every manifest it needs, and once it
  is built and committed it is moved to the front of the branch by a history edit with a clean
  tree, as `design@core@a-commit-message-is-a-document` and `klarch-development` §4 ask of a change
  to the manifest format. It is not pushed before the move, since `cargo klarch commits` fails on
  every earlier commit until then. Every later commit of the branch, this spec's included, then
  holds a manifest with the key. The moved commit's tree does not hold this spec, so its message
  names the spec in plain words, not by a reference.
- **The audits and reviews.** One design audit runs before step 1 and reads the entries of all
  three steps; its commit is named "The checker-version-pin design audit, applied in place: …",
  and a load-bearing gap it finds is recorded in this spec's Threads and stops the work. The code
  review of `klarch-development` §3 runs after step 1; the harvest review of
  `knowledge-architect-planning` §7, point 6, runs after step 3; the transcript reviewer runs
  last, alone.
- It is a spec, so the places `knowledge-architect-planning` §7 gives a milestone document, such as
  the list of defaults an audit adds to, are this spec's own sections.
- A step "lands" in its commit: the commit that completes a step reports on each acceptance
  criterion judged at it, and the pull request is marked ready only after step 3.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/19fd993c-9ae8-43cf-8609-41d92e79b77a.jsonl`.
  The discussion spans its lines 171 to 1923; the owner's messages that bear on it are at lines
  171, 220, 988, 1270, 1366, 1378, 1712, 1738 and 1923. Lines 1270, 1366, 1712 and 1923 are queued
  messages,
  recorded with `attachment.type: queued_command` and `origin.kind: human`.

## How a step is worked

Per `knowledge-architect-planning`, §7.

## Names

- **the pin**: the version of the checker a project chooses to run, per
  `design@agent-skills@exact-pin`. Today it lives only in the project's install: the maintenance
  crate's `knowledge-architect = "=<version>"` dependency, or the `--version` of a local
  `cargo install`.
- **the maintenance crate**: the setup skill's term for the crate through which a Rust project
  pins and runs the checker, per `design@agent-skills@xtask-pins-checker`.
- **the pin key**: the new `[project]` key that declares the pin in the manifest,
  `checker-version`, the name the owner chose. In this spec "the key" always means it.
- **a sentinel**: one of the two values of the key that are not versions, `"fixture"` and
  `"self"`.
- **the binary's version**: the version of the core library the running binary links,
  `env!("CARGO_PKG_VERSION")` read inside the `knowledge-architect` crate. For the core's own
  binary, `klarch`, it is also the binary's package version; for an extension's binary it is not.
- **the core directory**: the directory the core library was compiled from,
  `knowledge_architect::component_dir()` in `path@core@src/lib.rs`.
- **the checker directories**: the directories of the running binary's own source, which a
  binary's `main` already passes to `cli::run` as its `checker` argument; `klarch` passes the core
  directory, and an extension binary passes its own crate's directory. The new functions call this
  argument `checker_dirs`, so that it is not confused with the key.
- **the working tree**: the tree `cli::locate` finds, the one every command but `commits` judges.
- **a historical tree**: a commit's tree, as `commits` assembles it: `commit_tree` in
  `path@core@src/cli/history.rs` calls `read_tree`, which parses the commit's manifest.
- **a mock project**: a project under a library's tests directory, such as those under
  `path@core@tests/projects/`.
- **a complaint**: a finding `Manifest::parse` records about a declaration it refuses, read by
  `Manifest::complaints`; `check` reports complaints in phase 1, and the commands that write refuse
  while one stands.

## What the work is

**The defect.** An issue entry of the core recorded it until this spec, which schedules its fix,
closed it in the commit that added the spec; its reproduction moved here. In an adoption trial on
rust-lang/log at commit 27e3cf7a, the skills were installed by the published 0.2.0, and the
published 0.1.0 was then run with `check`: it stopped at phase 2 with 14 findings, each
`<file>  this installed file differs from what version 0.1.0 ships for this project`, each with
the repair `run install-agent-skills`, which writes the 0.1.0 text over the 0.2.0 text.

Two further cases, established by reading the code and not run:

- under `harness = []`, `check` in `path@core@src/check/agents.rs` returns before comparing any
  installed file, so a binary of any version runs silently;
- a binary installed with `cargo install` from the registry and run inside a checkout of this
  repository is not refused by `refuse_a_foreign_build` in `path@core@src/build_origin.rs`: the
  compiled directory ends in `knowledge-architect-<version>`, and no trailing run of it matches a
  directory of the checkout. Under this spec the root manifest says `"self"`, which such a binary
  does not confirm, so it refuses; #registry-core-confirms-no-sentinel guards it.

**What exists today at each site:**

- `Project` in `path@core@src/manifest.rs` deserialises `[project]` with `deny_unknown_fields`
  and holds `name`, `components` and `command`. A `version` key was refused in the trial, so no
  key exists to hold the pin. `resolve_command` in the same file validates `command` after the
  deserialisation, records a complaint for a value it refuses, and keeps the default.
- `path@core@src/main.rs`, the `main` of `klarch`, calls `cli::locate`, which parses the
  manifest, then `refuse_a_foreign_build`, then `cli::run` in `path@core@src/cli/mod.rs`. The
  template in the crate-level documentation of `path@core@src/lib.rs`, a compiled `no_run`
  doctest, matches the parsed command first: an extension's own commands are arms of that match,
  dispatched before `cli::locate` and `refuse_a_foreign_build`, through the public
  `cli::complete_working_tree` and `cli::Gathered`. An `Err(String)` from any of them is printed
  and exits 2, per `design@core@exit-code-ladder`. The setup skill's maintenance crate `main` runs
  the same calls in its `Klarch` arm; it is compiled by nothing, per
  `issue@agent-skills@the-setup-snippet-is-unchecked`, and its `Gates` arm runs `cargo klarch`,
  which refuses on its own.
- `Manifest::parse` returns `Err` for a manifest that does not deserialise, an unknown key of
  `[project]` included, since `Project` denies unknown fields: every released binary, 0.1.0 and
  0.2.0, stops in `cli::locate` with exit 2 over a manifest that carries the key.
- `install_agent_skills` in `path@core@src/cli/mod.rs` writes the shipped set of the running
  binary's version, and refuses while the manifest holds a complaint. `show`, `issues`,
  `tripwires` and `model` run with a complaint standing; `check --fix` runs `check` when one
  stands.
- `history::commits` reads each commit's manifest through `commit_tree` and `read_tree`; a tree
  that stops in phase 1 fails the range and serves as no parent, per
  `design@core@a-commit-message-is-a-document`.
- `refuse_a_foreign_build` compares the compiled directories of the libraries a binary links with
  the tree it is run over, per `design@core@a-foreign-build-is-refused`.
- The manifests the workspace holds, each of which the work gives the key:
  - 6 committed manifests: the root's, and those of the mock projects `core`, `dirhome`,
    `minimal`, `planted` and `unsound` under `path@core@tests/projects/`. Re-take with
    `git ls-files | grep -c 'knowledge-architect.toml$'`.
  - 33 manifest texts written inline in the core's library source, each passed to
    `Manifest::parse`, in `path@core@src/manifest.rs`, `path@core@src/entity.rs`,
    `path@core@src/index.rs`, `path@core@src/survey.rs`, `path@core@src/records.rs`,
    `path@core@src/extension.rs`, `path@core@src/agents.rs` and the modules `agents`, `generated`,
    `mod`, `references` and `registers` under `path@core@src/check/`. Re-take with
    `grep -rn '\[project\]' --include=*.rs crates/core/src`, which prints 40 lines, of which these
    7 are not manifests: `path@core@src/manifest.rs` at lines 33, 727, 952, 980, 1394 and 1764,
    and `path@core@src/check/tree.rs` at line 86.
  - In `path@core@tests/binary.rs`: `Sandbox`, whose `seeded` and `without_git` copy a mock
    project to the system's temporary directory; the free function `tiny_project`, which writes a
    manifest for a `History`; the test
    `the_core_binary_refuses_a_table_no_extension_of_it_claims`, which copies a mock with
    `copy_dir`; the test `the_core_binary_names_its_own_directory_and_counts_the_files_under_it`,
    which runs `klarch check` over this checkout's root; and the other tests that write a manifest
    of their own, found with `grep -n 'knowledge-architect.toml' crates/core/tests/binary.rs`.
  - In `path@core@tests/extension_api.rs`: three tests copy `dirhome` to the temporary directory
    with `copy_tree` and call `cli::run` with no checker directories.
  - `path@xtask@tests/gates_bin.rs` writes an empty manifest as a root marker for a stubbed
    `cargo`, and runs no check over it: it is unaffected.
- The texts that show a manifest, the pin or the exit-2 causes: the setup skill,
  `path@agent-skills@content/skills/setup/SKILL.md`, whose opening shows "the smallest such
  manifest", whose section 1 says how a project pins the checker, whose section "Moving the pin"
  says how it moves the pin, and whose section "In a Rust project" shows the maintenance crate and
  its `main`; the core README, `path@core@README.md`, whose table of exit codes lists the exit-2
  causes and which shows "the smallest manifest the install accepts"; and
  `path@core@CRATES-IO.md`, which holds no manifest but says "A project pins one exact version"
  and shows the install and the first `install-agent-skills` and `check`.

**Outside the work:**

- The build-origin refusal is unchanged. It reads the same directories the sentinels read, the
  core directory and the binary's own crate directory, through its `Library` list rather than the
  `checker` argument; nothing about what it refuses changes.
- An extension's access to the entity table,
  `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`, is separate work.
- thaum's own manifests: thaum adds the key when it next moves its pin, as the changelog's
  Migration entry tells every consumer. Where thaum's mock projects sit was not examined: those
  inside its extension crate's directory can say `"fixture"`, and any others carry the version.

## What is already decided

The design rests on these, and does not argue them again:

- `design@core@nothing-of-a-project-is-compiled-in`: no value names a project; a sentinel is
  confirmed by facts about the running binary's build.
- `design@core@a-foreign-build-is-refused` and `design@core@checker-source-literals-are-data`: the
  precedent for deciding from the binary's compiled directories, and for a refusal every binary's
  `main` calls before any command.
- `design@core@the-regime-has-no-opt-out`: no declaration exempts a document from a rule. The pin
  is a refusal of the run rather than a rule over a document, so this spec extends the head's
  stance to it: no value of the key lets a project switch the refusal off.
- `design@core@exit-code-ladder`: a refusal before a command runs is exit 2.
- `design@core@a-commit-message-is-a-document`: `commits` judges every commit with the tip
  checker, which is why a historical tree's value is never compared, and why the change of the
  manifest format comes first in its branch.
- `design@agent-skills@milestone-lands-before-gate-change`: a spec and its work are one branch.
- `goal@knowledge-architect@any-project-can-adopt-it`: "A project pins the version it uses, and
  moves to a new one when it chooses", and an existing project's adoption cost is never an
  argument against a check the workflow needs.

The work rewrites two recorded decisions, and adds one sentence to a third:

| decision | change | the texts that reference or restate it, and who updates each |
| --- | --- | --- |
| `design@agent-skills@exact-pin` | rewritten: the manifest declares the pin, and the binary refuses another version | `design@agent-skills@version-in-report`, which cites it: re-read at the harvest. The setup skill and `path@core@CRATES-IO.md` restate it without citing it: rewritten in step 2 |
| `design@agent-skills@xtask-pins-checker` | rewritten: the maintenance crate's pin and the key move together | `design@agent-skills@gates-convention`, which cites it: re-read at the harvest. The setup skill's section "In a Rust project" restates it: rewritten in step 2 |
| `design@core@a-commit-message-is-a-document` | one sentence: a historical tree's value is not compared | its referrers are unaffected, since no existing sentence changes |

Three more texts state what the pin guarantees, and are re-read at the harvest:
`design@knowledge-architect@version-lockstep` ("A project therefore never runs a checker whose
installed skills describe another version's commands"), which the pin makes true where the trial
showed it false; `design@knowledge-architect@binary-bundles-workflow`; and the retrospective
skill, `path@agent-skills@content/skills/retrospective/SKILL.md`, which records "the exact version
of the pin".

## Criteria

### A project pins the version it runs, and moves only when it chooses `##pin-is-the-projects-choice`

Binding, derived from `goal@knowledge-architect@any-project-can-adopt-it`. Met by
#installed-binary-version-check: a binary of another version refuses rather than moving the
installed text.

### No rule names one project `##no-project-compiled-in`

Binding as a presumption, from `design@core@nothing-of-a-project-is-compiled-in`. Met by
#checked-sentinel-values, whose sentinels are confirmed by the running binary's build.

### No value lets a project switch the refusal off `##no-opt-out`

Binding as a presumption, from `design@core@the-regime-has-no-opt-out`, extended to the run-level
refusal as "What is already decided" says. Met by #checked-sentinel-values: a sentinel where the
build does not confirm it is refused.

### Fixture manifests need an escape from the required pin `##fixtures-need-an-escape`

Weighed. The need is the owner's, at line 1366: "we need an escape for the fixture manifest on the
required pin". The cost it removes, that every fixture would change at every release, is the
session's measurement, #a11. Met by #checked-sentinel-values for a fixture inside a library
directory; a copy elsewhere carries the version, which the owner accepted with the synthesis.

### A project's history stays judgeable after the pin moves `##history-stays-judgeable`

Binding as a presumption, from `design@core@a-commit-message-is-a-document`. Met by
#history-compares-no-pin.

## Threads

### The manifest pins the checker's version exactly, the key is required, and every command over the working tree and the install refuse a mismatch `##installed-binary-version-check`

- **Proposed** by the session at transcript line 217, after the owner's question at line 171: "I'm
  not certain those things work as intended when it is an installed binary and not an extension,
  though." Its first default, to observe it in the adoption trial, was approved at line 220 by
  "go ahead with the branch and the trials".
- **Moved** by the trial (#a3, #a4), then by the session's own proposal at line 741, an installed
  version marker; then by the owner's alternative at line 988 (#a6); the session withdrew its
  proposal at line 1125 (#a8).
- **Closed, approved**, by the owner at line 1270: "installed-binary-version-check: go with
  required manifest pin." The owner gave no reason for "required"; the fork shown at line 1125
  argued it from #a9, and the reworded goal binds it (#pin-is-the-projects-choice). The owner's
  proposal at line 1712 restated it: "keep the key required in all situations".
- **Shape**: Decided design, "The key and the refusal".
- **Harvest**: a new head in `path@core@docs/design.md`, and rewrites of
  `design@agent-skills@exact-pin` and `design@agent-skills@xtask-pins-checker`.

### No pin is checked when the core was compiled from inside the tree, or the tree lies inside a library's compiled directory `##pin-fixture-escape`

- **Raised** by the owner at line 1366: "I suppose we need an escape for the fixture manifest on
  the required pin. Something that would only be available when working in this project. Maybe we
  would need to make it available to projects writing extensions too, however..."
- **Shaped** by the session at line 1370 (#a11, #a12, #a13): an exemption by build origin, with no
  key in an exempt tree.
- **Approved** by the owner at line 1378: "pin-fixture-escape approved".
- **Superseded** by #checked-sentinel-values, approved by the owner's word at line 1738. The
  absorbing thread keeps its two build-origin cases, as the confirmation of a value the manifest
  states rather than as an exemption the manifest does not show.
- **Harvest**: none of its own; its cases are harvested with #checked-sentinel-values.

### The key's presence and its value are checked in the command path, on the working tree only `##pin-checked-on-the-working-tree`

- **Proposed** by the session at line 1370 (#a14), inside the #pin-fixture-escape section of the
  message the owner approved at line 1378.
- **Superseded** by #checked-sentinel-values at line 1738: the key is required in every manifest,
  so its presence and its form are checked by `Manifest::parse`; its value is still compared on
  the working tree only, which #history-compares-no-pin keeps.
- **Harvest**: none of its own; judged with the losing alternatives.

### `commits` never compares a historical tree's value `##history-compares-no-pin`

- **Proposed** by the session at line 1370 (#a15).
- **Closed, approved**, with #pin-fixture-escape at line 1378, and kept by the synthesis at line
  1731, which says: "**Unchanged in every option:** `commits` never compares the pin of an old
  commit."
- **Shape**: Decided design, "Where it is checked".
- **Harvest**: inside the head of #installed-binary-version-check, and a sentence in
  `design@core@a-commit-message-is-a-document`.

### An exempt tree needs no key at all `##exempt-tree-needs-no-pin`

- **Proposed** at assembly by the author (#a16), as a default awaiting the owner.
- **Superseded** by #checked-sentinel-values at line 1738: every manifest carries the key, and a
  fixture or this repository says so with a sentinel.
- **Harvest**: none of its own; judged with the losing alternatives.

### The key is required everywhere, and takes a version or one of two sentinels, `"fixture"` and `"self"` `##pin-sentinel-values`

- **Proposed** by the owner at line 1712: "keep the key required in all situations, use a special
  value "fixture" for fixtures which triggers the behavior we want, and maybe allow another special
  value that this project would use for itself."
- **Superseded** by #checked-sentinel-values at line 1738, which takes its values and its required
  key whole, and adds the confirmation by the build.
- **Harvest**: none of its own; judged with the losing alternatives.

### Each sentinel is accepted only where the running binary's build confirms it `##checked-sentinel-values`

- **Proposed** by the session at line 1731, as a synthesis of #pin-sentinel-values and
  #pin-fixture-escape (#a17, #a18, #a19, #a20).
- **Closed, approved**, by the owner at line 1738: "go with the checked synthesis, revise the
  spec".
- **Shape**: Decided design, "The values and their confirmation".
- **Harvest**: a new head in `path@core@docs/design.md`.

## Arguments

### The build-origin refusal and the source line work for an installed binary over an unrelated project `##a1`

Line 217, the session, bears on #installed-binary-version-check: "`foreign_copy` refuses a run only
when the tree holds a second copy of the compiled package. A consumer's tree holds none, so the
check passes."

### No code compares the binary's version with a project's pin `##a2`

Line 217, the session, bears on #installed-binary-version-check: "`design@agent-skills@exact-pin`
enforces the pin through the setup procedure, not at run time."

### An older binary over newer installed skills asks for a downgrade `##a3`

Line 421, the trial agent, relayed at line 741, bears on #installed-binary-version-check: "The 0.1.0
binary over the 0.2.0-installed tree stops in phase 2 with 14 findings, each "differs from what
version 0.1.0 ships". Each tells you to run `install-agent-skills`, which would downgrade the files."

### No manifest key holds a version today `##a4`

Line 421, the trial agent, bears on #installed-binary-version-check: "I added `version = "0.2.0"`
under `[project]` as a test; the manifest refused it, which shows no such key exists."

### Without an agent harness, nothing detects a mismatch `##a5`

Line 421, the trial agent, bears on #installed-binary-version-check: "with `harness = []`, no version
mismatch would be detected at all." Relayed at line 741 by the session as "With `harness = []`,
nothing would detect the mismatch. That part is an assumption". The code review of this spec
confirmed it by reading `path@core@src/check/agents.rs`.

### The owner's alternative is a pin in the manifest, at the cost of one more place to edit `##a6`

Line 988, the owner, bears on #installed-binary-version-check: "how about pinning the version in the
manifest ? I think this is quite common too. But it would require one more place to bump when
bumping versions."

### The extra place cannot drift silently `##a7`

Line 1125, the session, bears on #installed-binary-version-check: "A binary that differs from the
pin, in either direction, refuses to run and says which side is older. So a forgotten edit fails
at the first command. That moment is also when the setup skill asks the user to read the changelog
of each version crossed."

### The manifest pin covers what the installed marker cannot `##a8`

Line 1125, the session, bears on #installed-binary-version-check: "The two discriminating facts are
that the manifest pin covers projects without agents, and versions whose skill text is identical."

### A required key protects the case it exists for `##a9`

Line 1125, the session, bears on #installed-binary-version-check: "With an optional key, a project
that never wrote it gets no protection, and the forgotten-pin case is exactly the one it guards."
Against it, the same line: "**Optional:** no migration, and the setup skill writes it for new
projects."

### Adoption cost does not argue against a needed check `##a10`

Line 1270, the owner, given as the reason for rewording
`goal@knowledge-architect@any-project-can-adopt-it`, not as a reason on this thread: "a check
judged as important, in particular for the agent workflow to function better, should not be
argued against because of adoption cost for existing projects." It bears on
#installed-binary-version-check through the reworded goal, which answers the cost #a9 names
against a required key.

### A required exact pin would change every fixture at every release `##a11`

Lines 1352 and 1370, the session, bears on #pin-fixture-escape and #checked-sentinel-values: "there
are 6 committed manifests (the root and 5 mock projects), and 40 inline `[project]` manifests in
the core's test source." "With an exact pin, each of these would change at every release." The
code review of this spec counted 33 of the 40 lines as manifests; the argument does not change.

### An escape for one project conflicts with a recorded decision, and the build's own facts do not `##a12`

Line 1370, the session, bears on #pin-fixture-escape and #checked-sentinel-values:
"`design@core@nothing-of-a-project-is-compiled-in` refuses any rule that names one project." "The
checker already has a way around this, used twice: it decides from facts about **its own build**,
not about any project."

### A value that disables the pin, open to every project, is an opt-out `##a13`

Line 1370, the session, bears on #pin-fixture-escape, against the rival `"*"`: "That is new manifest
syntax, open to every project." "A consumer can switch off its own protection with it, which goes
against `design@core@the-regime-has-no-opt-out`." Line 1731 applies the same argument to an
unchecked sentinel (#a18).

### A key required at parse would change every inline test manifest, and fail every earlier commit `##a14`

Line 1370, the session, bears on #pin-checked-on-the-working-tree: "If `Manifest::parse` refused a
manifest without it, all 40 inline test manifests would change." At assembly, a second fact: every
commit before the key existed parses its manifest under the tip checker, and its tree then stops
in phase 1. Line 1731 answered both halves: the inline manifests change once, to `"fixture"`, and
the change of format comes first in its branch.

### Comparing a historical value fails every earlier commit when the pin moves `##a15`

Line 1370, the session, bears on #history-compares-no-pin: "Otherwise every move of the pin would
fail every earlier commit of the branch."

### A key in an exempt tree is never compared, so its value would only go stale `##a16`

At assembly, the author, bears on #exempt-tree-needs-no-pin. A required key in an exempt tree
would hold a value nothing reads, edited at every release for nothing. A sentinel answers it: the
value says what the tree is, and never needs editing.

### The owner's sentinels make the key required everywhere, and the session reads three more advantages in them `##a17`

Line 1712, the owner's proposal: "keep the key required in all situations". Line 1731, the
session's reading of it, bears on #pin-sentinel-values and #checked-sentinel-values: "**The intent
is visible in the file.**" "**It does not depend on layout.**" "**The parse can require the key.**
The 33 inline test manifests get `checker = "fixture"` once, not once per release."

### An unchecked sentinel is an opt-out `##a18`

Line 1731, the session, bears on #pin-sentinel-values and #checked-sentinel-values: "A consumer, or
an agent stuck on the refusal, can write `checker = "fixture"` in its real manifest, and its pin is
silently off. That is the opt-out that `design@core@the-regime-has-no-opt-out` refuses, and the same
argument (#a13) that defeated the `"*"` requirement."

### Checking the sentinels against the build keeps the values and removes the opt-out `##a19`

Line 1731, the session, bears on #checked-sentinel-values: "It also keeps the property that no real
project can switch its pin off, because the values only claim what the build can confirm. The
build-origin cases stop deciding on their own; they only verify what the manifest claims." And:
"The defect this whole thread fixes is a project running the wrong version without noticing. An
unchecked `"fixture"` brings that back through a value that the setup skill itself would have to
document."

### Checking costs a version in test copies and in mocks outside a library directory `##a20`

Line 1731, the session, bears on #checked-sentinel-values: "A mock copied to the temporary
directory is no longer inside a library directory, so its `"fixture"` is refused. The test helpers
must still rewrite it to the binary's version." "If thaum keeps them somewhere else, they cannot
use `"fixture"` and must carry the version."

## New names, in one place

```text
path@core@src/manifest.rs
  Project::checker_version: Option<String>  the pin key as written, kebab-case "checker-version"
  enum Pin { Version(String), Fixture, OwnBuild }
                                            pub(crate); "<MAJOR.MINOR.PATCH>", "fixture", "self"
  fn resolve_checker(declared: &Declared, complaints: &mut Vec<Finding>) -> Option<Pin>
                                            private, beside resolve_command; an absent key or a
                                            value of none of the three forms is a complaint
  Manifest::pin: Option<Pin>                the field resolve_checker fills; None only when the
                                            key is absent or of none of the three forms
  Manifest::pin(&self) -> Option<&Pin>      pub(crate)

path@core@src/cli/mod.rs, defined there with the two predicates and their unit tests
  pub fn refuse_another_version(manifest: &Manifest, checker_dirs: &[&Path]) -> Result<(), String>
                                            called by each binary's main right after
                                            refuse_a_foreign_build; passes
                                            knowledge_architect::component_dir() as the core
                                            directory to the two predicates below
  fn is_a_library_fixture(root: &Path, core: &Path, checker_dirs: &[&Path]) -> bool
                                            private; the root lies inside, or is, the core
                                            directory or one of the checker directories
  fn is_its_own_build(root: &Path, core: &Path) -> bool
                                            private; the core directory lies inside, or is, the
                                            root
                                            Both canonicalise each path and keep the path as
                                            written when canonicalisation fails, as foreign_copy
                                            in build_origin.rs does; the core directory is a
                                            parameter so that a unit test can place it.

path@core@tests/binary.rs
  fn pinned(manifest_text: &str) -> String  test helper: sets the key to env!("CARGO_PKG_VERSION"),
                                            replacing a checker line under [project], or inserting
                                            one after the [project] line when there is none
```

`OwnBuild` is the variant's name in the code; the value in the manifest is `"self"`, since `Self`
is a Rust keyword. `path@core@tests/extension_api.rs` needs no helper: no test there runs a binary. Existing names the work uses: `cli::run`, `cli::locate`,
`cli::complete_working_tree`, `install_agent_skills`, `history::commits`, `commit_tree`,
`read_tree`, `Manifest::parse`, `Manifest::complaints`, `resolve_command`, `Declared`,
`knowledge_architect::component_dir`, `refuse_a_foreign_build`, `foreign_copy`, `Sandbox`,
`Sandbox::seeded`, `Sandbox::without_git`, `Sandbox::serve_claude`, `History`, `tiny_project`,
`copy_dir`, `copy_tree`.

## Decided design

### The key and the refusal

`[project] checker-version` is required in every manifest. It takes `"<MAJOR.MINOR.PATCH>"`, the version
of the checker the project runs, which is the version of the core library whatever binary links
it, exact as `design@agent-skills@exact-pin` makes the install's pin; or one of the two sentinels
of the next section. A version is three non-negative integers in decimal with no leading zero
except a lone `0`, separated by dots, with no pre-release and no build metadata.

`Manifest::parse` validates the key in `resolve_checker`, as `resolve_command` validates `command`:
an absent key, or a string of none of the three forms, is a complaint, whose text opens with
`[project] checker-version`, which the tests match: `[project] checker-version is absent` or
`[project] checker-version "<value>" is not a version, "fixture" or "self"`, with the repair "write the version of the
checker the project runs". A value that is not a string fails the deserialisation, as any
mistyped key does: `Manifest::parse` returns `Err`. In a historical tree the complaint stops the
tree in phase 1, so `commits` fails a commit that lacks the key.

Each binary's `main` calls `cli::locate`, then `refuse_a_foreign_build`, then
`cli::refuse_another_version`, before it dispatches any command: `klarch`'s `main`, the crate-level
template, which is restructured to make the three calls before its match on the command, and the
setup skill's maintenance crate `main`. So every command refuses, the core's and an extension's
own, the install included. Where the key's complaint stands, `refuse_another_version` returns
`Err` with that complaint's text, so no command runs without a key, `show` and `model` included;
other complaints are left to the command. Otherwise a refusal is `Err`, exit 2, and each message
opens with the fixed text below, which the tests match, and goes on to the repair:

| case | the message opens with | repair it names |
| --- | --- | --- |
| a version, the binary older | `this binary runs knowledge-architect <binary's version>, older than the <pin> the manifest pins` | run or build the pinned version |
| a version, the binary newer | `this binary runs knowledge-architect <binary's version>, newer than the <pin> the manifest pins` | run the pinned version, or read the changelog of each version crossed and set the key |
| `"fixture"`, not confirmed | `"fixture" is only valid for a mock project inside a library this binary links` | set the key to the version the project runs |
| `"self"`, not confirmed | `"self" is only valid where the checker is built from this tree` | set the key to the version the project runs |

A version is accepted when it equals the binary's version as text. Where it differs, the two are
compared as three integers to choose between the older and the newer message; a binary's version
that is not three integers, which no release has, gets the newer message's wording with "another
version than" in place of "newer than". A build from another checkout is refused by
`refuse_a_foreign_build` before this runs, so its message comes first even when the key is absent.

A binary released before this work refuses a manifest carrying the key at parse, with an
unknown-field error, since `Project` denies unknown fields: it runs no command, so it cannot
suggest a downgrade, but its message does not name the pin. The messages above are given from the
first release that carries this work.

**Argument**: #a3, #a7, #a8, #a9, with #a10 through the goal. **Nearest rival**: the installed set
records its version and the check compares it; defeated by #a8, since it covers neither
`harness = []` nor two versions whose skill text is identical.

### The values and their confirmation

- `"fixture"` is accepted when `is_a_library_fixture` holds: the root lies inside, or is, the core
  directory or one of the checker directories. That is a mock project of a library the running
  binary links, run by that library's binary. The 5 mock projects under
  `path@core@tests/projects/` and the 33 inline manifests say `"fixture"`. An inline manifest is
  only parsed, never confirmed, so the value needs no confirming there.
- `"self"` is accepted when `is_its_own_build` holds: the core directory lies inside, or is, the
  root. That is this repository, or a project that vendors the core. The root manifest of this
  repository says `"self"`.
- A version is accepted when it equals the binary's version.

A copy of a mock project in the system's temporary directory is not inside a library directory, so
every test that makes one and runs a binary over it rewrites the key to the binary's version,
through `pinned`. A mock project an extension keeps outside its crate's directory carries
the version. An extension's test that calls `cli::run` in-process over one of its own mocks
confirms nothing, since `cli::run` does not refuse; the refusal is the binary's `main`'s.

**Argument**: #a12, #a17, #a19, #a20. **Nearest rival**: the same values, accepted wherever they
are written (#pin-sentinel-values); defeated by #a18. **Second rival**: no key in a tree the build
exempts (#pin-fixture-escape); superseded by #checked-sentinel-values, approved at line 1738;
argument #a17.

### Where it is checked

The key's presence and its form are checked by `Manifest::parse`, so every tree, historical ones
included, needs it: a working tree without it refuses every command, and a historical tree without
it stops in phase 1. Its value is confirmed by
`refuse_another_version`, over the working tree's manifest only. `commits` confirms no historical
tree's value, so moving the pin fails no earlier commit (#a15); the working tree's manifest is
still confirmed when `commits` itself runs, like any command.

Since a tree without the key stops in phase 1 under the tip checker, step 1's commit, which adds
the key to the workspace's manifests, is moved to the front of its branch (Status and audience).
Its first parent is main's tip, which stops in phase 1 under it and so serves as no parent: the
first commit's message is judged against its own tree alone, so it names no entry that only
main's tip defines.

## Mapping tables

What the parse and `refuse_another_version` return, over the key's whole domain. "Fixture holds"
is `is_a_library_fixture`; "own build holds" is `is_its_own_build`.

| key | condition | result |
| --- | --- | --- |
| absent | n/a | the key's complaint; every command exits 2 with its text; a historical tree stops in phase 1 |
| a string of none of the three forms | n/a | the same |
| a value that is not a string | n/a | `Manifest::parse` returns `Err`; exit 2; `commits` reports the tree as not loading |
| `"<MAJOR.MINOR.PATCH>"` | equal to the binary's version | Ok |
| `"<MAJOR.MINOR.PATCH>"` | the binary is older | Err, exit 2: older than the pin |
| `"<MAJOR.MINOR.PATCH>"` | the binary is newer | Err, exit 2: newer than the pin |
| `"fixture"` | fixture holds | Ok |
| `"fixture"` | fixture does not hold | Err, exit 2: `"fixture"` not confirmed |
| `"self"` | own build holds | Ok |
| `"self"` | own build does not hold | Err, exit 2: `"self"` not confirmed |

Which manifest says what, and whether the build confirms it:

| tree | the core compiled from | value | confirmed |
| --- | --- | --- | --- |
| this repository, run with its cargo alias | inside it | `"self"` | yes |
| this repository, run with a binary installed from the registry | the registry | `"self"` | no: refused |
| this repository, run with a binary installed from this checkout | inside it | `"self"` | yes: it is the tree's own build |
| a mock project under `path@core@tests/projects/`, binary built here | the core directory, which holds the mock | `"fixture"` | yes |
| a copy of a mock project in the temporary directory | elsewhere | rewritten to the version | yes, when it equals |
| a consumer's root, the core from crates.io | the registry | the version | yes, when it equals |
| a consumer's root that writes `"fixture"` | the registry | `"fixture"` | no: refused |
| a mock project under an extension's crate, run by the extension's binary | the registry; the extension's crate directory holds the mock | `"fixture"` | yes |
| any project carrying the key, run with a binary released before this work | n/a | any | refused at parse: unknown field |
| a mock project an extension keeps outside its crate | the registry | the version | yes, when it equals |
| a consumer vendoring the core inside its tree | inside it | `"self"` | yes |

## Losing alternatives

- **Observe in the trial only, change no code**: superseded by #installed-binary-version-check
  once the trial reproduced the downgrade (#a3).
- **The install writes its version into the installed set, and the check compares it**: withdrawn
  by its proposer, the session, at line 1125, lost to #installed-binary-version-check; defeating
  reason #a8.
- **An optional key**: ruled out by the owner's word at line 1270, lost to
  #installed-binary-version-check; the session's argument #a9, and the reworded goal.
- **An escape available only in this project**, the owner's first wording at line 1366: reshaped
  into #pin-fixture-escape; defeating reason #a12.
- **A version requirement in the key, fixtures writing `"*"`**: lost to #pin-fixture-escape; #a13.
- **No key in a tree the build exempts**, #pin-fixture-escape with #exempt-tree-needs-no-pin:
  superseded by #checked-sentinel-values, approved at line 1738; argument #a17.
- **The sentinels accepted wherever they are written**, #pin-sentinel-values as proposed: lost to
  #checked-sentinel-values; #a18.
- **The key optional at parse, checked in the command path only**, #pin-checked-on-the-working-tree:
  superseded by #checked-sentinel-values; the owner's "required in all situations" and #a17.
- **`commits` compares each historical tree's value**: lost to #history-compares-no-pin; #a15.

## Readings

None: the work reads no external specification. The form `MAJOR.MINOR.PATCH` is the one
`cargo help install` gives for an exact `--version`.

## Premortem

Assuming the design shipped and failed. Causes 1 to 4 were given by the session at line 1370 for the
earlier shape and are re-judged here against the approved one; causes 5 and 6 come from the
synthesis at line 1731 and from this revision.

1. **A consumer copies a project into a temporary directory and runs the checker there.** Stresses
   #checked-sentinel-values. Verdict: survives; a copy carrying the version is compared, which is
   correct, and one carrying `"fixture"` is refused. Becomes the step 1 test of a copied mock that
   keeps `"fixture"`.
2. **A release forgets to bump the root manifest.** Stresses #checked-sentinel-values. Verdict:
   converted; the root says `"self"`, which no release edits. Becomes #root-says-self.
3. **An extension links the core through a path dependency inside its own tree.** Stresses
   #checked-sentinel-values. Verdict: survives; it writes `"self"`, which its build confirms.
4. **A binary installed with `cargo install` has its compiled directories in the registry.**
   Stresses #checked-sentinel-values. Verdict: survives; a tree never lies inside the registry, so
   the binary confirms no sentinel. Becomes #registry-core-confirms-no-sentinel.
5. **An agent stuck on a refusal writes `"fixture"` in a real project's manifest.** Stresses
   #checked-sentinel-values. Verdict: survives; the value is refused outside a library directory,
   with the repair "set the key to the version the project runs".
6. **The branch that adds the key fails its own `commits` gate.** Stresses
   #installed-binary-version-check. Verdict: converted into the order of work; step 1's commit is
   moved to the front of the branch.

The session judged that none of causes 1 to 4 needs a tripwire, and the owner recorded none.
Causes 5 and 6 are handled by the design and the order; no tripwire is proposed.

## Acceptance criteria

### A core compiled outside the tree confirms no sentinel `##registry-core-confirms-no-sentinel`

- **Guards**: #checked-sentinel-values.
- **Judged at**: step 1, two ways. By unit tests of `is_a_library_fixture` and `is_its_own_build`
  over scratch directories: a root, and a core directory outside it, with no checker directories.
  And end to end: `cargo package -p knowledge-architect --no-verify --allow-dirty`, the packaged
  crate unpacked into a scratch directory outside the tree, `cargo install --locked --path` from
  there into another scratch directory, and that binary's `check` in this checkout. Its compiled
  directory lies outside the tree and ends in `knowledge-architect-<version>`, as a registry
  build's does. It is a manual run, which needs the network for the packaged dependencies, with
  `--locked` where the package holds a `Cargo.lock` and without it otherwise; its result goes in
  step 1's landing message.
- **Fires when**: either predicate returns true for that root, or the installed binary does not
  exit 2 with `"self" is only valid where the checker is built from this tree`.
- **Response**: reopen #checked-sentinel-values.

### This repository passes with `"self"` in its root manifest `##root-says-self`

- **Guards**: #checked-sentinel-values.
- **Judged at**: step 1.
- **Fires when**: `cargo klarch check` at step 1's commit, with `checker-version = "self"` in the root
  manifest, exits anything but 0, or
  `the_core_binary_names_its_own_directory_and_counts_the_files_under_it` fails.
- **Response**: reopen #checked-sentinel-values.

## Implementation sequence

1. **The key, the values and the refusal, with every manifest of the workspace**, in one commit,
   moved to the front of the branch once built. `Pin`, `Project::checker_version`, `resolve_checker`,
   `Manifest::pin`, `cli::refuse_another_version`, `is_a_library_fixture` and `is_its_own_build`;
   the call in `path@core@src/main.rs` after `refuse_a_foreign_build`, and the crate-level template
   of `path@core@src/lib.rs` restructured to call `cli::locate`, `refuse_a_foreign_build` and
   `cli::refuse_another_version` before its match on the command. Installed text is step 2's. The
   root
   manifest says `"self"`; the 5 committed mock manifests and the 33 inline manifests say
   `"fixture"`. Tests:
   - the parse records the complaint for an absent key and for a string of none of the three forms,
     each matching its opening text, and returns `Err` for a value that is not a string;
   - by the binary over a copy whose key is absent, `show` exits 2 with the complaint's text;
   - by the binary over copies of a mock, one per row of the first mapping table that ends in
     `Err`, each matching the message's opening text: a version older, a version newer, a copy that
     keeps `"fixture"`, a copy that says `"self"`; and one with the version equal, which passes;
   - an in-place mock run passes with `"fixture"`; `cargo klarch check` passes on this repository
     with `"self"`, and so does
     `the_core_binary_names_its_own_directory_and_counts_the_files_under_it`;
   - unit tests of both predicates over scratch directories, true and false, the equal-path case
     included (#registry-core-confirms-no-sentinel);
   - `install-agent-skills` over a copy of `unsound`, or a copy given the default harness with
     `Sandbox::serve_claude`, with another version: it exits 2 with the older or newer message, and
     the installed set is unchanged;
   - `commits` over a range whose earlier commit pins another version passes.

   Every test that copies a mock out of the core directory and runs a binary over it rewrites the
   key to the binary's version: `Sandbox::seeded` and `Sandbox::without_git`, `tiny_project`, and
   `the_core_binary_refuses_a_table_no_extension_of_it_claims` through `pinned`, and every other
   test of `path@core@tests/binary.rs` that writes a manifest of its own. The three tests of
   `path@core@tests/extension_api.rs` that copy `dirhome` call `cli::run` in-process, which does
   not refuse; their copies keep `"fixture"`, which the parse accepts. Fails alone on: a test of the refusal; or a test unrelated to the pin
   that now exits 2 or stops in phase 1, which names a manifest the list above missed.
2. **The documentation**, under the procedure for installed text: the section "Editing an
   installed skill or agent" of `path@agent-skills@CLAUDE.md` and
   `knowledge-architect-agent-configuration`. The setup skill's smallest manifest carries the key;
   its section 1 says that the manifest declares the pin, that the binary refuses another version,
   and that a mock project inside the project's own library may say `"fixture"`; the first item of
   its section "Moving the pin" edits the key with the other pins; its section "In a Rust project"
   says the maintenance crate's pin and the key move together, and its `main` calls
   `cli::refuse_another_version` after `refuse_a_foreign_build`, before its match, as the crate
   template does.
   The installed copy is re-installed with `cargo klarch install-agent-skills`. The core README's
   smallest manifest carries the key, and its exit-code table lists the refusal among the exit-2
   causes. `path@core@CRATES-IO.md` says that the manifest's key names the pinned version. The
   changelog carries a Migration entry, in a Migration subsection the `Next release` section does
   not hold yet, `manifest`, major: every manifest carries the key, the
   version a project runs, `"fixture"` for a mock project inside a library's directory and for a
   manifest a test only parses, an extension's included; and a Workflow entry, `agent-skills`,
   patch: the setup skill writes the key and moves it with the pin. `cargo x changelog` writes the
   copies. Fails alone on: `cargo klarch check`, or a changelog copy that differs.
3. **The harvest**, below, and the deletion of this spec.

## Order rationale

Step 1 before step 2: the documentation describes a key the code reads, and a skill naming a key
the binary refuses would mislead every session between the two. Step 2 before step 3: the harvest
rewrites `design@agent-skills@exact-pin` and `design@agent-skills@xtask-pins-checker`, which the
setup skill restates, so both change against the same built text. Step 1's commit is then the first
of the branch, by the history edit Status and audience describes, so that no commit of the branch
holds a manifest without the key.

## Defaults awaiting the owner

None. The key's name was the one default: the session used `checker` as an example at lines 1125
and 1731, and the owner then named it at line 1923, a queued message: "maybe checker-version for
the key ? It is a bit more explicit IMO. Or just version."; the session took `checker-version`,
since `version` under `[project]` reads as the project's own version, and the owner may still
correct it.

## Harvest

- `path@core@docs/design.md`: a head under the slug `installed-binary-version-check`, holding the
  key, its forms, the complaint, the refusal in each binary's `main`, where it is checked and
  #history-compares-no-pin; a head under the slug `checked-sentinel-values`, holding the two
  sentinels and their confirmation by the build; a sentence in
  `design@core@a-commit-message-is-a-document` that a historical tree's value is not compared.
- `path@agent-skills@docs/design.md`: `design@agent-skills@exact-pin` and
  `design@agent-skills@xtask-pins-checker` rewritten in place;
  `design@agent-skills@version-in-report` and `design@agent-skills@gates-convention` re-read.
- `path@knowledge-architect@docs/design.md`: `design@knowledge-architect@version-lockstep` and
  `design@knowledge-architect@binary-bundles-workflow` re-read against the pin; the retrospective
  skill's line on the pin re-read.
- `path@core@docs/rejected-alternatives.md` and `path@agent-skills@docs/rejected-alternatives.md`:
  each losing alternative above judged by the recording tests of
  `knowledge-architect-decision-recording`; the installed version marker, the unchecked sentinels
  and the build exemption with no key are the candidates under its test 4.
- No tripwire: the owner recorded none.
- Nothing to close at the harvest: the issue this spec schedules was closed when it was added. The
  build-origin observation it carried, an installed binary inside a checkout of this repository,
  needs no record after the harvest: under the pin such a binary refuses `"self"`, as
  #registry-core-confirms-no-sentinel judges end to end.

## Later consequences

- Every consumer adds the key once, at its next move of the pin, as the Migration entry says; thaum
  is the one known.
- A release of this repository edits no manifest for the pin, since its root says `"self"` and its
  fixtures say `"fixture"`.
- A mock project an extension keeps outside its crate's directory carries the version, and moves
  with the pin.
- An extension that does not call `cli::refuse_another_version` in its `main` is not refused, as
  one that does not call `refuse_a_foreign_build` is not; the setup skill's template is what makes
  both calls the default.
