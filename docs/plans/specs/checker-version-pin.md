# The manifest pins the checker's version, and a binary of another version refuses to run

## Status and audience

This spec is the plan of the work that makes the manifest declare the version of the checker a
project pins: the key is required in every manifest, holds a version, or one of two values a
fixture or the checker's own repository uses, each accepted only where the running binary's build
confirms it; every command run over the working tree refuses when what the key says does not
hold. It is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the code.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point. **Step 1 does not start before the owner rules on the default under Defaults awaiting
  the owner**, since it governs its code.
- **This spec lands in a pull request of its own, merged before the work starts.** The work's
  first commit changes the manifest format: under the tip checker, a tree whose manifest lacks the
  key does not load. The commit that adds this spec has such a tree, so on the work's branch the
  `commits` gate would refuse it. This is the reason `knowledge-architect-planning` gives for
  landing a milestone document in a merge of its own, and it holds for this spec. The work is
  then one branch and one pull request.
- It is a spec, so the places `knowledge-architect-planning` §7 gives a milestone document, such as
  the list of defaults an audit adds to, are this spec's own sections.
- A step "lands" in its commit on the work's branch: the commit that completes a step reports on
  each acceptance criterion judged at it, and the pull request is marked ready only after step 3.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/19fd993c-9ae8-43cf-8609-41d92e79b77a.jsonl`.
  The discussion spans its lines 171 to 1738; the owner's messages that bear on it are at lines
  171, 220, 988, 1270, 1366, 1378, 1712 and 1738. Lines 1270, 1366 and 1712 are queued messages,
  recorded with `attachment.type: queued_command` and `origin.kind: human`.

## How a step is worked

Per `knowledge-architect-planning`, §7.

## Names

- **the pin**: the version of the checker a project chooses to run, per
  `design@agent-skills@exact-pin`. Today it lives only in the project's install: the maintenance
  crate's `knowledge-architect = "=<version>"` dependency, or the `--version` of a local
  `cargo install`.
- **the pin key**: the new `[project]` key that declares the pin in the manifest, named `checker`
  by default (see Defaults awaiting the owner). In this spec "the key" always means it.
- **a sentinel**: one of the two values of the key that are not versions, `"fixture"` and
  `"self"`.
- **the binary's version**: the version of the core library the running binary links,
  `env!("CARGO_PKG_VERSION")` read inside the `knowledge-architect` crate. For the core's own
  binary, `klarch`, it is also the binary's package version; for an extension's binary it is not.
- **the core directory**: the directory the core library was compiled from,
  `knowledge_architect::component_dir()` in `path@core@src/lib.rs`.
- **the checker directories**: the `checker` argument of `cli::run` in
  `path@core@src/cli/mod.rs`, every directory of the running binary's own source; `klarch` passes
  the core directory, and an extension binary passes its own crate's directory. The new functions
  call this argument `checker_dirs`, so that it is not confused with the key.
- **the working tree**: the tree `cli::locate` finds, the one every command but `commits` judges.
- **a historical tree**: a commit's tree, as `commits` assembles it: `commit_tree` in
  `path@core@src/cli/history.rs` calls `read_tree`, which parses the commit's manifest.
- **a mock project**: a project under a library's tests directory, such as those under
  `path@core@tests/projects/`.

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
- a binary installed with `cargo install` and run inside a checkout of this repository is not
  refused by `refuse_a_foreign_build` in `path@core@src/build_origin.rs`: the compiled directory
  ends in `knowledge-architect-<version>`, and no trailing run of it matches a directory of the
  checkout. Under this spec the root manifest says `"self"`, which such a binary does not confirm,
  so it refuses; #registry-core-confirms-no-sentinel guards it.

**What exists today at each site:**

- `Project` in `path@core@src/manifest.rs` deserialises `[project]` with `deny_unknown_fields`
  and holds `name`, `components` and `command`. A `version` key was refused in the trial, so no
  key exists to hold the pin.
- `cli::run` in `path@core@src/cli/mod.rs` configures the extensions and dispatches the command.
  It is the one entry every binary calls with the working tree's manifest, the core's `main` in
  `path@core@src/main.rs` and an extension's `main` alike, per
  `design@core@the-core-cli-is-a-library-module`. It returns `Err(String)` for could-not-run, which
  the caller prints and exits 2 on, per `design@core@exit-code-ladder`. Each binary's `main` calls
  `refuse_a_foreign_build` before `cli::run`, so a foreign build is refused before the pin is read.
- `install_agent_skills` in the same file writes the shipped set of the running binary's version.
- `history::commits` reads each commit's manifest through `commit_tree` and `read_tree`, and turns
  a manifest that does not parse into a finding that the commit "does not load under the tip
  checker"; it judges every commit of the range with the tip checker, per
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
    `path@core@src/extension.rs`, `path@core@src/agents.rs` and five modules under
    `path@core@src/check/`. Re-take with
    `grep -rn '\[project\]' --include=*.rs crates/core/src`, which also prints 7 lines that are
    not manifests (doc comments and messages).
  - The manifests the crate-level documentation of `path@core@src/lib.rs` shows, and the one
    `path@core@src/mock_projects.rs` names.
  - In `path@core@tests/binary.rs`: `Sandbox`, whose `seeded` and `without_git` copy a mock
    project to the system's temporary directory; the free function `tiny_project`, which writes a
    manifest for a `History`; the test
    `the_core_binary_refuses_a_table_no_extension_of_it_claims`, which copies a mock with
    `copy_dir` and runs `klarch check`; and the other tests that write a manifest of their own,
    found with `grep -n 'knowledge-architect.toml' crates/core/tests/binary.rs`.
  - In `path@core@tests/extension_api.rs`: three tests copy `dirhome` to the temporary directory
    with `copy_tree` and call `cli::run` with no checker directories.
  - `path@xtask@tests/gates_bin.rs` writes an empty manifest as a root marker for a stubbed
    `cargo`, and runs no check over it: it is unaffected.
- The texts that show a manifest or list the exit-2 causes: the setup skill,
  `path@agent-skills@content/skills/setup/SKILL.md`, whose opening shows "the smallest such
  manifest", whose section 1 says how a project pins the checker, whose section "Moving the pin"
  says how it moves the pin, and whose section "In a Rust project" shows the maintenance crate;
  the core README, `path@core@README.md`, whose table of exit codes lists the exit-2 causes and
  which shows "the smallest manifest the install accepts". `path@core@CRATES-IO.md` holds no
  manifest.

**Outside the work:**

- The build-origin refusal is unchanged; the sentinels read the same compiled directories, and
  change nothing about what `refuse_a_foreign_build` refuses.
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
  precedent for deciding from the binary's compiled directories.
- `design@core@the-regime-has-no-opt-out`: no declaration exempts a document from a rule. The pin
  is a refusal of the run rather than a rule over a document, so this spec extends the head's
  stance to it: no value of the key lets a project switch the refusal off.
- `design@core@exit-code-ladder`: a refusal before a command runs is exit 2, and a manifest that
  does not parse is exit 2.
- `design@core@a-commit-message-is-a-document`: `commits` judges every commit with the tip
  checker, which is why a historical tree's value is never compared, and why the change of the
  manifest format comes first in its branch.
- `goal@knowledge-architect@any-project-can-adopt-it`: "A project pins the version it uses, and
  moves to a new one when it chooses", and an existing project's adoption cost is never an
  argument against a check the workflow needs.

The work rewrites two recorded decisions, and adds one sentence to a third:

| decision | change | the texts that reference or restate it, and who updates each |
| --- | --- | --- |
| `design@agent-skills@exact-pin` | rewritten: the manifest declares the pin, and the binary refuses another version | `design@agent-skills@version-in-report`, which cites it: re-read at the harvest. The setup skill restates it without citing it: rewritten in step 2 |
| `design@agent-skills@xtask-pins-checker` | rewritten: the maintenance crate's pin and the key move together | the setup skill's section "In a Rust project" restates it: rewritten in step 2 |
| `design@core@a-commit-message-is-a-document` | one sentence: a historical tree's value is not compared | its referrers are unaffected, since no existing sentence changes |

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

### The key is required everywhere, and takes a version or one of two sentinels, `"fixture"` and `"self"` `##pin-sentinel-values`

- **Proposed** by the owner at line 1712: "keep the key required in all situations, use a special
  value "fixture" for fixtures which triggers the behavior we want, and maybe allow another special
  value that this project would use for itself."
- **Superseded** by #checked-sentinel-values at line 1738, which takes its values and its required
  key whole, and adds the confirmation by the build.

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
unchecked sentinel (#a19).

### A key required at parse would change every inline test manifest, and fail every earlier commit `##a14`

Line 1370, the session, bears on #pin-checked-on-the-working-tree: "If `Manifest::parse` refused a
manifest without it, all 40 inline test manifests would change." At assembly, a second fact: every
commit before the key existed parses its manifest under the tip checker, and `commits` reports a
manifest that does not parse as a commit that does not load. Line 1731 answered both halves: the
inline manifests change once, to `"fixture"`, and the change of format comes first in its branch.

### Comparing a historical value fails every earlier commit when the pin moves `##a15`

Line 1370, the session, bears on #history-compares-no-pin: "Otherwise every move of the pin would
fail every earlier commit of the branch."

### A key in an exempt tree is never compared, so its value would only go stale `##a16`

At assembly, the author, bears on #exempt-tree-needs-no-pin. A required key in an exempt tree
would hold a value nothing reads, edited at every release for nothing. A sentinel answers it: the
value says what the tree is, and never needs editing.

### The owner's sentinels make the intent visible and the key required everywhere `##a17`

Line 1712, the owner's proposal; line 1731, the session's reading of it, bears on
#pin-sentinel-values and #checked-sentinel-values: "**The intent is visible in the file.**"
"**"Required" means required everywhere.**" "**It does not depend on layout.**" "**The parse can
require the key.** The 33 inline test manifests get `checker = "fixture"` once, not once per
release."

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
  Project::checker: Pin                     the pin key, kebab-case "checker", required
  enum Pin { Version(String), Fixture, OwnBuild }
                                            pub(crate); "<MAJOR.MINOR.PATCH>", "fixture", "self";
                                            any other value, or an absent key, fails the parse
  Manifest::pin(&self) -> &Pin              pub(crate)

path@core@src/cli/mod.rs
  fn refuse_another_version(manifest: &Manifest, checker_dirs: &[&Path]) -> Result<(), String>
                                            private; called first in cli::run; passes
                                            knowledge_architect::component_dir() as the core
                                            directory to the two predicates below
  fn is_a_library_fixture(root: &Path, core: &Path, checker_dirs: &[&Path]) -> bool
                                            private; the root lies inside the core directory or
                                            one of the checker directories
  fn is_its_own_build(root: &Path, core: &Path) -> bool
                                            private; the core directory lies inside the root
                                            Both over canonical paths; the core directory is a
                                            parameter so that a unit test can place it.

path@core@tests/binary.rs
  fn pinned(manifest_text: &str) -> String  test helper: the text with the key set to
                                            env!("CARGO_PKG_VERSION")

path@core@tests/extension_api.rs
  fn pin(dir: &Path)                        test helper: rewrites the key of a copied mock
                                            project's manifest to the binary's version; a
                                            separate test crate cannot reach binary.rs
```

`OwnBuild` is the variant's name in the code; the value in the manifest is `"self"`, since `Self`
is a Rust keyword. Existing names the work uses: `cli::run`, `cli::locate`, `install_agent_skills`,
`history::commits`, `commit_tree`, `read_tree`, `Manifest::parse`,
`knowledge_architect::component_dir`, `refuse_a_foreign_build`, `Sandbox`, `Sandbox::seeded`,
`Sandbox::without_git`, `History`, `tiny_project`, `copy_dir`, `copy_tree`.

## Decided design

### The key and the refusal

`[project] checker` is required in every manifest. It takes `"<MAJOR.MINOR.PATCH>"`, the version
of the checker the project runs, which is the version of the core library whatever binary links
it, exact as `design@agent-skills@exact-pin` makes the install's pin; or one of the two sentinels
of the next section. `Manifest::parse` refuses a manifest without the key, or with any other value,
naming the three forms.

`cli::run` calls `refuse_another_version` before it configures the extensions, so every command
refuses, the install included, and so does every extension binary, since each calls `cli::run`. A
refusal is `Err`, exit 2. Each message opens with the fixed text below, which the tests match, and
goes on to the repair:

| case | the message opens with | repair it names |
| --- | --- | --- |
| a version, the binary older | `this binary runs knowledge-architect <binary's version>, older than the <pin> the manifest pins` | run or build the pinned version |
| a version, the binary newer | `this binary runs knowledge-architect <binary's version>, newer than the <pin> the manifest pins` | run the pinned version, or read the changelog of each version crossed and set the key |
| `"fixture"`, not confirmed | `"fixture" is only valid for a mock project inside a library this binary links` | set the key to the version the project runs |
| `"self"`, not confirmed | `"self" is only valid where the checker is built from this tree` | set the key to the version the project runs |

Versions are compared as three unsigned integers; the order only chooses the message, since any
difference refuses. In `klarch`, a build from another checkout is refused by
`refuse_a_foreign_build` in `main` before this runs.

**Argument**: #a3, #a7, #a8, #a9, with #a10 through the goal. **Nearest rival**: the installed set
records its version and the check compares it; defeated by #a8, since it covers neither
`harness = []` nor two versions whose skill text is identical.

### The values and their confirmation

- `"fixture"` is accepted when `is_a_library_fixture` holds: the root lies inside the core
  directory, or inside one of the checker directories. That is a mock project of a library the
  running binary links. The 5 mock projects under `path@core@tests/projects/` and the 33 inline
  manifests say `"fixture"`; an inline manifest is only parsed, never run through `cli::run`, so
  the value is never confirmed there and needs no confirming.
- `"self"` is accepted when `is_its_own_build` holds: the core directory lies inside the root. That
  is this repository, or a project that vendors the core. The root manifest of this repository says
  `"self"`.
- A version is accepted when it equals the binary's version.

A copy of a mock project in the system's temporary directory is not inside a library directory, so
every test that makes one rewrites the key to the binary's version, through `pinned` or `pin`. A
mock project an extension keeps outside its crate's directory carries the version.

**Argument**: #a12, #a17, #a19, #a20. **Nearest rival**: the same values, accepted wherever they
are written (#pin-sentinel-values); defeated by #a18. **Second rival**: no key in a tree the build
exempts (#pin-fixture-escape); superseded by the owner's word, for #a17.

### Where it is checked

The key's presence and its form are checked by `Manifest::parse`, so every tree, historical ones
included, needs it, and a tree without it does not load. Its value is confirmed in `cli::run`, over
the working tree's manifest only. `commits` confirms no historical tree's value, so moving the pin
fails no earlier commit (#a15); the working tree's manifest is still confirmed when `commits`
itself runs, like any command. Since a manifest without the key does not load under the tip
checker, the commit that adds the key to the workspace's manifests is the first of its branch, and
this spec lands before that branch (Status and audience).

## Mapping tables

What the parse and `refuse_another_version` return, over the key's whole domain. "Fixture holds"
is `is_a_library_fixture`; "own build holds" is `is_its_own_build`.

| key | condition | result |
| --- | --- | --- |
| absent | n/a | the parse fails: exit 2 |
| a value of none of the three forms | n/a | the parse fails: exit 2 |
| `"<MAJOR.MINOR.PATCH>"` | equal to the binary's version | Ok |
| `"<MAJOR.MINOR.PATCH>"` | the binary is older | Err: older than the pin |
| `"<MAJOR.MINOR.PATCH>"` | the binary is newer | Err: newer than the pin |
| `"fixture"` | fixture holds | Ok |
| `"fixture"` | fixture does not hold | Err: `"fixture"` not confirmed |
| `"self"` | own build holds | Ok |
| `"self"` | own build does not hold | Err: `"self"` not confirmed |

Which manifest says what, and whether the build confirms it:

| tree | the core compiled from | value | confirmed |
| --- | --- | --- | --- |
| this repository, run with its cargo alias | inside it | `"self"` | yes |
| this repository, run with a binary from `cargo install` | the registry | `"self"` | no: refused |
| a mock project under `path@core@tests/projects/`, binary built here | the core directory, which holds the mock | `"fixture"` | yes |
| a copy of a mock project in the temporary directory | elsewhere | rewritten to the version | yes, when it equals |
| a consumer's root, the core from crates.io | the registry | the version | yes, when it equals |
| a consumer's root that writes `"fixture"` | the registry | `"fixture"` | no: refused |
| a mock project under an extension's crate | the registry; the extension's crate directory holds the mock | `"fixture"` | yes |
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
  superseded by #checked-sentinel-values; the intent is invisible in the file and "required"
  narrowed (#a17).
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
   correct, and one carrying `"fixture"` is refused. Becomes a claim of step 1.
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
   #installed-binary-version-check. Verdict: converted into the order of work; this spec lands in
   its own pull request, and the work's first commit carries the format change and every manifest.

The session judged that none of causes 1 to 4 needs a tripwire, and the owner recorded none.
Causes 5 and 6 are handled by the design and the order; no tripwire is proposed.

## Acceptance criteria

### A core compiled outside the tree confirms no sentinel `##registry-core-confirms-no-sentinel`

- **Guards**: #checked-sentinel-values.
- **Judged at**: step 1, by unit tests of `is_a_library_fixture` and `is_its_own_build` over
  scratch directories: a root, and a core directory outside it standing for the registry, with no
  checker directories.
- **Fires when**: either predicate returns true for that root.
- **Response**: reopen #checked-sentinel-values.

No published binary carries the refusal yet, and a binary installed from this checkout is refused
first as a foreign build, so the end-to-end run waits for the first release; Later consequences
names it.

### This repository passes with `"self"` in its root manifest `##root-says-self`

- **Guards**: #checked-sentinel-values.
- **Judged at**: step 1.
- **Fires when**: `cargo klarch check` at step 1's commit, with `checker = "self"` in the root
  manifest, exits anything but 0.
- **Response**: reopen #checked-sentinel-values.

## Implementation sequence

1. **The key, the values and the refusal, with every manifest of the workspace**, in the first
   commit of the work's branch. `Pin`, `Project::checker`, the parse's refusal of an absent or
   malformed key, `refuse_another_version`, `is_a_library_fixture` and `is_its_own_build`. The root
   manifest says `"self"`; the 5 committed mock manifests and the 33 inline manifests say
   `"fixture"`; the crate-level documentation's manifests say a version placeholder. Tests:
   - the parse refuses an absent key and a value of none of the three forms;
   - by the binary over copies of a mock, one per row of the first mapping table whose result is
     an `Err` of the refusal, each matching the message's opening text, and one with the version
     equal, which passes;
   - an in-place mock run passes with `"fixture"`, and `cargo klarch check` passes on this
     repository with `"self"`;
   - unit tests of both predicates over scratch directories, true and false
     (#registry-core-confirms-no-sentinel);
   - `install-agent-skills` over a copy with another version refuses and writes nothing;
   - `commits` over a range whose earlier commit pins another version passes.

   Every test that copies a mock out of the core directory rewrites the key to the binary's
   version: `Sandbox::seeded` and `Sandbox::without_git`, `tiny_project`, and
   `the_core_binary_refuses_a_table_no_extension_of_it_claims` through `pinned`, and every other
   test of `path@core@tests/binary.rs` that writes a manifest of its own; the three tests of
   `path@core@tests/extension_api.rs` that copy `dirhome`, through `pin`. Fails alone on: a test of
   the refusal; or a test unrelated to the pin that now exits 2, which names a manifest the list
   above missed.
2. **The documentation.** The setup skill's smallest manifest carries the key; its section 1 says
   that the manifest declares the pin, that the binary refuses another version, and that a mock
   project inside the project's own library may say `"fixture"`; the first item of its section
   "Moving the pin" edits the key with the other pins; its section "In a Rust project" says the
   maintenance crate's pin and the key move together. The installed copy is re-installed with
   `cargo klarch install-agent-skills`. The core README's smallest manifest carries the key, and its
   exit-code table lists the refusal among the exit-2 causes. The changelog carries a Migration
   entry, which holds for mock projects too, and a Workflow entry; `cargo x changelog` writes the
   copies. Fails alone on: `cargo klarch check`, or a changelog copy that differs.
3. **The harvest**, below, and the deletion of this spec.

## Order rationale

This spec's pull request before the work's: the work's first commit makes a manifest without the
key unloadable, and the commit adding this spec has one. Step 1 before step 2: the documentation
describes a key the code reads, and a skill naming a key the binary refuses would mislead every
session between the two. Step 2 before step 3: the harvest rewrites `design@agent-skills@exact-pin`
and `design@agent-skills@xtask-pins-checker`, which the setup skill restates, so both change
against the same built text.

## Defaults awaiting the owner

- **The key's name, `checker`**: the session used it as an example at lines 1125 and 1731, and the
  owner approved the pin and the synthesis without naming the key. Default: `checker`. Step 1 does
  not start before the ruling.

## Harvest

- `path@core@docs/design.md`: a head under the slug `installed-binary-version-check`, holding the
  key, its forms, the refusal, where it is checked and #history-compares-no-pin; a head under the
  slug `checked-sentinel-values`, holding the two sentinels and their confirmation by the build; a
  sentence in `design@core@a-commit-message-is-a-document` that a historical tree's value is not
  compared.
- `path@agent-skills@docs/design.md`: `design@agent-skills@exact-pin` and
  `design@agent-skills@xtask-pins-checker` rewritten in place; `design@agent-skills@version-in-report`
  re-read.
- `path@core@docs/rejected-alternatives.md` and `path@agent-skills@docs/rejected-alternatives.md`:
  each losing alternative above judged by the recording tests of
  `knowledge-architect-decision-recording`; the installed version marker, the unchecked sentinels
  and the build exemption with no key are the candidates under its test 4.
- No tripwire: the owner recorded none.
- Nothing to close at the harvest: the issue this spec schedules was closed when it was added. The
  build-origin observation it carried, an installed binary inside a checkout of this repository,
  needs no record after the harvest: under the pin such a binary refuses `"self"`, as
  #registry-core-confirms-no-sentinel guards.

## Later consequences

- Every consumer adds the key once, at its next move of the pin, as the Migration entry says; thaum
  is the one known.
- At the first release that carries the refusal, the end-to-end form of
  #registry-core-confirms-no-sentinel can be run: `cargo install --locked --root <scratch>
  knowledge-architect --version =<that release>`, then that binary's `check` in a clone of this
  repository, which must refuse with `"self" is only valid where the checker is built from this
  tree`.
- A release of this repository edits no manifest for the pin, since its root says `"self"` and its
  fixtures say `"fixture"`.
- A mock project an extension keeps outside its crate's directory carries the version, and moves
  with the pin.
