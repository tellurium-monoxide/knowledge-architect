# knowledge-architect v0.1: extraction from thaum, the agent skills, and the first release

**This is a milestone document.** It covers work across several PRs, with two intermediate design
sessions: the public API (step 4) and the designing-together intake (step 6). It carries the design
converged in one design session held in the thaum repository on 2026-09-30 and 2026-10-01, between
the owner and an agent, under the designing-together skill.

**Its lifecycle in this repository.** It is committed on the step 2 branch, in the commit that
holds the restructure, and reaches main with that PR. Every later step corrects it in place, on
that step's branch, where the step changed anything. It is deleted in the release commit of step 7.
Nothing in it is authority over a design home once that home exists: where the two disagree, the
design home wins and this document has the defect.

**How to continue it.** A session reads, in order: this head, section 1, the step it implements in
section 9, and the sections that step names. Section 3 is the full record of the discussion, for
the case where a step meets something the design did not foresee.

**Labels.** A point marked **default** was chosen by the agent while writing this document and
has not been put to the owner. It stands unless the owner overrules it. A point marked
**approved default** was such a choice, then approved by the owner on 2026-10-01 ("I agree with
the other defaults"). A point marked **pending** waits for the owner's word, and the step that
needs it stops there.

**Words used here.**

| word | meaning |
| --- | --- |
| spec | the document of short work: its design and a concise implementation sequence |
| milestone document | the same for work across several PRs |
| plan | the docs/plans/ directory, and nothing else. A "detailed implementation plan" is the separate step-by-step document of the superpowers workflow, which this design rejects (3.2). |
| harvest | writing the decisions of a landed step into the design homes |
| material finding | information that arrived after a thread closed and defeats a named part of that closure |
| watch point | a question the retrospective skill asks at the end of a session |
| thread | one question of the discussion, named by a slug |
| arm | one of the numbered tests in recording-a-decision that decide whether a decision or an alternative earns an entry |
| history line | a line that records a past state on purpose, such as a line of a progress log, and is not rewritten by a rename |

**Repositories.** This one is https://github.com/tellurium-monoxide/knowledge-architect, public.
thaum is the source of the extraction, at /home/catA/tb266682/Documents/code/thaum on the owner's
machine; its main is at e98e296. designing-together is at
/home/catA/tb266682/Documents/code/designing-together. Its version 0.6.0 is on `next` and
on the remote's master; the local master is behind, at 0.4.0.

**The transcript.** The discussion is recorded in the owner's Claude Code session log,
~/.claude/projects/-home-catA-tb266682-Documents-code-thaum/82256f51-c589-4e66-84da-cafba28b3a26.jsonl.
A readable transcript is extracted from it by keeping the text of user and assistant messages and
dropping tool calls and subagent traffic. The filter must select by message type, never by a
substring of the text: a substring filter dropped one owner message once.

The companion document is thaum's migration spec, on thaum's branch
`knowledge-architect-migration`, file docs/plans/knowledge-architect-migration.md. It runs after
v0.1 is published. Nothing in this document touches thaum.

## 1. What the work is

The `knowledge` tool leaves thaum and becomes the project knowledge-architect. Its scope is larger
than the tool:

- **The checker.** The current core of the tool, renamed. It checks that a project's documents stay
  consistent with its code and with each other, through declared registers and checked references.
- **An agent workflow.** Skills and subagent definitions that tell an AI agent how to work in a
  project that uses the checker: setting up, writing goals, designing, planning, recording
  decisions, tracking open issues, reviewing, maintaining the agent configuration, and running a
  retrospective. The checker embeds them and writes them into a project on request.

The purpose, as the owner stated it: documentation must stay consistent with the code, because
that consistency is what makes AI agents productive, and agent-driven development is fast enough
that review alone can no longer keep it consistent.

v0.1 is all of it: the checker with the changes of section 4.3, the full skill and agent set, and
the designing-together intake. It is published on crates.io. Then thaum migrates, in a separate
session, under its own spec.

## 2. Criteria

| criterion | kind | satisfied by |
| --- | --- | --- |
| owner-intent: goals reflect the owner's intent and stay short | binding | goals-required, and the setting-goals skill once it is written (step 5) |
| ship-isolation: what a project receives is the skills, the agents and the primer, and nothing of this repository's internals | binding | two-crates (the content/ directory), package-include-whitelist |
| self-hosted: this repository passes its own check and installs its own skills | binding | repo-layout, crate-directory; the check from step 2, the installed skills from step 5 |
| thaum-keeps-working: after the split, thaum's checker runs rules-corpus on the published core, and thaum's gates pass | binding | session-sequence: thaum is untouched until v0.1 exists, then migrated under its own spec |
| nothing-compiled-in: no installed skill names a path or a convention of one project | binding | overlay-by-separate-skills, shipped-text-is-reference-free, declared-command |
| one-workflow: the installed skills never give two contradictory instructions | binding | thread-slug-is-entry-id, harvest-after-implementation, losing-alternatives-filter, spec-and-milestone. **Not yet fully met:** four points of 4.8 were never discussed, and steps 5 and 6 resolve them with the owner. |

## 3. The ledger

Every thread of the discussion, with its final state. Thread names are written plain here, because
a backticked span is read by the checker as a reference candidate. When a decision is harvested,
its thread name becomes the entry's slug unless it collides (thread-slug-is-entry-id).

### 3.1 Approved

| thread | decision | note |
| --- | --- | --- |
| repo-layout | a virtual workspace with four components: the root, crates/core, crates/agent-skills, tools/xtask | 4.1 |
| crate-directory | the checker's crate lives in crates/core, package `knowledge-architect` | absorbs crate-under-crates |
| single-crate | the core is one crate with a library target and a binary target. The current two crates (`documentation`, the library, and `knowledge`, the binary) merge. | the library already depends on clap |
| two-crates | the skill text lives in a second crate, crates/agent-skills, package `knowledge-architect-agent-skills`. The core depends on it. | the owner wished for one package bundling both; it lost on argument, 3.2 |
| binary-name | the binary is `klarch`. The crate stays `knowledge-architect`. | owner, 2026-10-01; replaces the full name for the binary, 3.2 |
| xtask-gates | tools/xtask is reproduced with the gates subcommand only | 4.6 |
| gates-convention | the setting-up skill recommends the xtask gates pattern. No code is shipped for it. | this repository follows it |
| version-lockstep | one version number for both crates, and so for the skill content | the owner called it "critical" |
| binary-bundles-workflow | the crate embeds the skills, agents and primer, and a command writes them into a project. There is no plugin marketplace. | owner's proposal. Supersedes five threads, 3.3 |
| installed-files-committed | the installed files are committed in the project, and verified by the checker | the gitignored variant is ruled out |
| primer-by-import | the primer is a file the install writes, imported by one line in the project's root CLAUDE.md | replaces the hook delivery of always-on-primer |
| install-command-name | the command is `install-agent-skills` | owner: "there is no need to save on words, this does not get typed often" |
| declared-command | a manifest key names the command a project runs. The installed skill text is a template filled with it at install, and checked as rendered. | owner, 2026-10-01; 4.3 point 2 |
| agents-table | a manifest table `[agents]` with a `harness` list, default claude. An empty list disables the install, the installed-file checks and the CLAUDE.md requirement. | reverses parts of recorded core decisions, 4.3 point 3 |
| owned-namespace-check | the prefix knowledge-architect- owns a namespace in the project's agent configuration. Each shipped file is checked one by one, and every file in the namespace that the current version does not ship is a finding. | option (b) of three, 3.2 |
| installed-prefix-length | installed skills and agents use the full prefix knowledge-architect- | approved default |
| overlay-by-separate-skills | an installed skill is complete on its own. A project adds its own conventions through skills with distinct names and through its CLAUDE.md, never by shadowing or editing an installed skill. | |
| routing-table | the setting-up skill writes, and maintaining-agent-config keeps, a table in the project's root CLAUDE.md mapping each installed skill to the project skills that add to it | owner's proposal |
| skill-name-prefix | project skills and agents carry the project name as a hyphen prefix, as thaum-developing. The directory or file name and the frontmatter `name` are equal. | a colon is invalid in an agent name |
| shipped-text-is-reference-free | installed text carries no live reference, only placeholders in angle brackets | plus an open issue for a mechanical check |
| root-goals | the root component's goals are the owner's text of 2026-10-01, written at docs/goals.md in this repository's working tree and committed in step 2 unchanged | owner: "take this draft as the definitive version" |
| core-goal-abandoned | the core's goal documentation-half-publishes-alone is abandoned and removed in step 2. It only made sense while the core lived inside thaum. | owner, 2026-10-01 |
| goal-lifecycle | a goal stays in its goals home while it is met, and leaves only when it is abandoned, on the owner's word. A goal removed when achieved stops being checked, and can stop being met without anyone noticing. | owner, 2026-10-01. It shapes the setting-goals and setting-up skills and the goals homes' introductions. thaum's goals home says the opposite today. |
| goals-required | every component states at least one goal. An instruction in the skills. The mechanical check is an open issue. | |
| thread-slug-is-entry-id | approved "for now" (owner). A thread slug is minted in the entry-id grammar and checked for collisions before use. Approved, it becomes the design entry's slug. In discussion prose a thread is written #slug. | the long-term answer is the issue on structured plan documents |
| harvest-after-implementation | decisions are recorded in the design homes when the work that implements them lands, not when the spec is written | |
| losing-alternatives-filter | recording-a-decision's rule wins: an alternative earns an entry only if it passes that skill's tests. The rest stay in the spec and the commit message. | the owner judges the rule "still a little imperfect", not improved now |
| spec-and-milestone | short work has one document, the spec. Work across several PRs with intermediate design sessions has a milestone document, which extends the spec in place. | plus the owner's content rule against untested snippets, 4.8 |
| document-vocabulary | **open until step 5** (owner, 2026-10-01): the owner rediscusses it when the planning skill is forked. The proposal on the table: "spec" for short work, "milestone document" for long work, "plan" for the docs/plans/ directory only. | this document uses the proposal meanwhile |
| designing-together-retirement | option (a): the designing-together skill is forked into this project as the designing skill, and the owner archives the old repository. Its decision record is not carried as history: each item is shown to the owner, who rules on keeping it and on its wording, because "AI agents did most of that record … Some might not fully aligned with everything today" (owner). Its README content is kept. | (b), carrying the history, lost on cost, 3.2 |
| plugin-inventory | the skills and agents listed in 4.7 | maintaining-agent-config is included |
| transcript-conformity-review | the planning skill's review of a spec or milestone document includes one reviewer that reads the verbatim transcript of the design discussion and checks the document against the decisions taken in it. Whether it is an installed agent, and whether dispatching-a-review lists it as an axis, is decided in step 5. | owner: "Probably, this mode of review should become standard in the planning skills", then "agreed, defer the decision to step 5" |
| retrospective-destination | the retrospective writes a file in a scratch location of the user's machine, then offers to open an issue on this repository, and opens it only on the owner's word | 4.7 carries its trigger and purpose |
| premortem-as-watch-points | P1, P2 and P3 of section 7 are not tripwires. They are watch points in the retrospective skill. | a skill-patching issue answers P1 |
| package-include-whitelist | each crate's Cargo.toml declares an `include` whitelist | tests and docs are not shipped |
| history-import | the history is imported with a path filter on tools/knowledge/ only | |
| import-to-main | the filtered history is pushed directly as main, with no edit. Every change after it goes through a branch and a PR. | one exception to the Git flow |
| thaum-dependency-source | the crates are published on crates.io | the git tag dependency is ruled out |
| exact-pin | the setting-up skill recommends pinning the checker exactly, as `knowledge-architect = "=0.1.0"` | finding unacknowledged by the owner, 4.5: a dependency builds no executable; its default stands |
| license | MIT OR Apache-2.0, for the whole repository | |
| versioning-policy | the owner's scheme with five refinements, 4.5. A patch can bring a finding (owner, 2026-10-01). | |
| public-api | the public Rust API is decided and described before v0.1, in a design session of its own | 4.4 |
| v01-scope | option (b): v0.1 is everything, the checker, the full skill and agent set and the designing-together intake. Option (a) was a v0.1 with a skill skeleton only. | |
| session-sequence | this project is completed and published first, over several sessions, without touching thaum. Then thaum migrates in a separate session. Two documents. | |
| spec-home | this document lives in this repository's docs/plans/ | |
| thaum-switch-today | reshaped by session-sequence into the separate migration | |

Owner principles stated in the discussion, which bound the inventory:

- carrying out the work (developing) is left to each project. No development skill is installed.
- the project stays at 0.x for a while, and allows regular breaking changes, until the open issues
  of this discussion are implemented, or at least argued thoroughly.

### 3.2 Ruled out

| alternative | lost to | why it lost |
| --- | --- | --- |
| crate-at-root | crate-directory | the owner rejected it: a crate at the root mixes the crate with the project, and a future split into several crates needs a crates/ directory |
| crate-under-crates in its first form, crates/knowledge-architect | crate-directory | refuted by a run: the directory's basename collides with the project's anchor (4.1) |
| the skills nested in the core, as a component at crates/core/workflow | two-crates | the owner: it "would bundle everything under a single component, which is bad usage of the tool itself" |
| one package bundling both components, rooted at crates/ | two-crates | the owner asked for a single published package. cargo packages only files under a package directory, so one package holding both components must be rooted at crates/. That makes crates/ a package holding two components, the mixing crate-at-root was rejected for, and a third crate could not live under it. Users still name one crate: cargo fetches the other. |
| the binary named knowledge-architect | binary-name | the declared command's default is the binary's name, and the owner wanted it short to type |
| the git tag dependency for thaum | thaum-dependency-source | the owner chose crates.io |
| a next branch for releases, as designing-together does | binary-bundles-workflow | the owner's concern was that every merge to main would become a release. Without a marketplace, a merge to main publishes nothing: crates.io receives a version only through an explicit publish. |
| gitignored installed files | installed-files-committed | every session started from a clone without a built binary (a fresh clone, a web session, a review subagent in a new worktree) would run without the workflow, and nothing would report it |
| a skill named thaum:developing | skill-name-prefix | a colon is invalid in a subagent name. Where it works for a skill, it is indistinguishable from a skill of a plugin named thaum. |
| a spec plus a separate detailed implementation plan, the superpowers shape | spec-and-milestone | both carry the same decisions, and the second drifts from the first. The owner observed that such plans hold untested code snippets in a categorical tone, which implementers force into the code at any cost, copying their guideline comments verbatim. |
| recording every ruled-out thread, designing-together's rule | losing-alternatives-filter | rejected-alternatives.md would grow by every thread of every discussion, including proposals nobody would raise again |
| owned-namespace-check option (a), every path ever shipped | owned-namespace-check | the list grows forever and misses a file nobody shipped, such as a hand-made knowledge-architect-foo |
| owned-namespace-check option (c), an install record file | owned-namespace-check | one more committed file, which can itself be edited |
| designing-together-retirement option (b), carrying the dated history | designing-together-retirement | converting 17 sections of dated entries into the heading-register shape, for history nobody would re-read |

### 3.3 Superseded

| thread | absorbed by | what it was |
| --- | --- | --- |
| crate-under-crates | crate-directory | the owner's original crates/ proposal. Its only defect was the anchor collision. |
| release-branching | binary-bundles-workflow | (b), approved before: a marketplace entry of type git-subdir pinned to a tag and its sha. The docs confirm that git-subdir takes ref and sha. |
| version-agreement-gate | binary-bundles-workflow | a gate checking that Cargo.toml, plugin.json and the marketplace ref agree |
| version-check-source | binary-bundles-workflow | the primer hook read the version from Cargo.lock, then PATH, and warned on a mismatch |
| project-scope-install | binary-bundles-workflow | the plugin installed per project, never per user. The installed check makes every install per project by construction. |
| always-on-primer | primer-by-import | a plugin SessionStart hook that found the manifest and printed the primer. It lost because, without a plugin, a hook means the installer edits the user's settings file, and because it needs a shell. |

### 3.4 Parked

| thread | re-entry | home |
| --- | --- | --- |
| cross-project-references | the owner takes it up, or finds it needed. The owner: "I doubt there is a real need to reference design decisions of another project". Until then, a design decision that relies on the tool states that it relies on the tool working as intended. | a deferred issue in the core, which owns the reference grammar (step 2); the interim rule in the agent-skills design home, step 5 |

## 4. The decided design

### 4.1 Repository layout

```
knowledge-architect/                 component "knowledge-architect" (the project root)
├─ Cargo.toml                        virtual workspace: members crates/*, tools/xtask
├─ knowledge-architect.toml          the manifest
├─ .gitignore                        target/, .claude/worktrees/
├─ .cargo/config.toml                aliases: x (xtask), klarch (the checker)
├─ rust-toolchain.toml  rustfmt.toml
├─ .github/workflows/ci.yml          cargo x gates on every ready pull request
├─ LICENSE-MIT  LICENSE-APACHE  CHANGELOG.md
├─ README.md  CLAUDE.md  docs/       the project's documents and registers
├─ .claude/                          location "agent-config": issue register, and the installed set
│  ├─ open-issues/                   README.md, index.md
│  ├─ skills/knowledge-architect-*   installed by its own binary, committed (from step 5)
│  └─ agents/knowledge-architect-*   installed, committed (from step 5)
├─ crates/core/                      component "core"; package knowledge-architect (lib + bin klarch)
│  └─ LICENSE-MIT  LICENSE-APACHE    copies, so the package ships them
├─ crates/agent-skills/              component "agent-skills"; package knowledge-architect-agent-skills (lib)
│  ├─ README.md  CLAUDE.md  docs/    the component's own documents, never installed
│  ├─ LICENSE-MIT  LICENSE-APACHE    copies
│  ├─ build.rs  src/lib.rs           the embedded files, as install path and template text
│  └─ content/                       what install-agent-skills writes, and nothing else (from step 3)
│     ├─ skills/<skill>/SKILL.md     → .claude/skills/knowledge-architect-<skill>/SKILL.md
│     ├─ agents/<agent>.md           → .claude/agents/knowledge-architect-<agent>.md
│     └─ PRIMER.md                   → .claude/knowledge-architect/PRIMER.md
└─ tools/xtask/                      component "xtask"; gates only; publish = false
```

**Approved default:** the manifest declares a location `agent-config` at .claude carrying the
issue register, as thaum's does. The core's design home already holds a reference into it.

**Default:** the licenses are copied into each crate directory. An SPDX `license` field ships no
file, and an `include` path cannot reach outside the package directory. cargo package flattens a
symlink into a file, so a symlink would also do; a copy is simpler to read.

Facts this rests on:

- **Anchor collision.** In a scratch project named knowledge-architect with a component at
  crates/knowledge-architect, the check stopped in phase 1: the name knowledge-architect "names 2
  anchors", the project root and the crate directory. With the directory named crates/core, phase
  1 passed. So a crate directory is named by its role, never by its package. A future split gives
  crates/core (package knowledge-architect-core) and crates/cli (package knowledge-architect).
- **Nested components are legal.** A component at crates/core/workflow inside crates/core passed
  phase 1 in the same scratch project. The owner rejected that placement on other grounds (3.2).
- **Packaging.** cargo package ships the files under the package directory, a Cargo.lock, a
  normalised Cargo.toml, and a readme or a license file the manifest names. An `include_str!`
  reaching outside the package fails at cargo package's own verification build. So the skill text
  must sit inside a package, which is why agent-skills is a crate.

### 4.2 Crates and publishing

- The core's package is `knowledge-architect`, with a lib target and a bin target. The binary is
  `klarch` (binary-name). Its library is imported as `knowledge_architect`.
- The agent-skills package is `knowledge-architect-agent-skills`, a library that exposes the
  embedded files. The core depends on it by path and by exact version.
- Both share `[workspace.package] version`. **Approved default:** the version stays 0.0.0 until step
  7 sets it to 0.1.0. tools/xtask declares `publish = false` permanently.
- Both declare an `include` whitelist: the core ships its src/, Cargo.toml, README.md and the
  license copies; agent-skills ships the same plus build.rs and content/. tests/ and docs/ are not
  shipped. The cost, accepted: cargo test cannot run from a downloaded package. CHANGELOG.md is not
  shipped, and a consumer reads it on GitHub.
- Both names returned 404 from the crates.io API on 2026-09-30, so both were free. The name
  `knowledge` is taken. Step 7 re-checks before publishing.
- Users name only `knowledge-architect`, and cargo fetches the other.
- The repository description, the GitHub topics and the Cargo `keywords` carry "agentic" and
  "software design". The name does not, because "agent" next to "knowledge" reads as a RAG project.
  **Default:** the Cargo keywords are `agentic` and `software-design`, the GitHub topics the same.
  The description and topics are set in step 2, on the owner's word, since they are public.
- **Approved default:** the licenses name Teo Boutin as the copyright holder.

### 4.3 The checker's new surface for v0.1

v0.1's code is thaum's current core plus exactly these changes.

**1. The manifest file is knowledge-architect.toml**, named after the tool as rustfmt.toml and
clippy.toml are. In the core, the constant `MANIFEST_NAME` (manifest.rs) is the only spelling:
the finding strings that spell the literal today (check/references.rs line 395,
check/registers.rs line 354, check/tree.rs lines 51 and 67) use the constant. Tests, fixtures and
documents spell it as data. **Default:** tools/xtask depends on the core by path and uses the
constant in its root finder, which is the fix the owner named for the hard-coded literal. So
`MANIFEST_NAME` stays public (4.4).

**2. The command a project runs is declared, not compiled in.** The core prints a command in its
messages (cli/mod.rs line 146 prints `cargo knowledge check`) and in the header of every generated
index (index.rs line 100 and check/generated.rs line 42 print `cargo knowledge index`). That is
thaum's alias. The installed skills and the primer name the command too.

- **Default:** the key is `command` in the `[project]` table. When it is absent, the command is
  `klarch`, the core's binary name, whatever binary runs.
- A fixed name cannot serve every project: a project with an extension runs its own binary, as
  thaum runs rules-corpus, and the core's binary refuses a manifest holding a table only the
  extension claims (extension.rs, the phase-1 finding "is a table no extension of this binary
  reads").
- **An extension project** declares a command that runs its own binary, preferably a cargo alias
  such as `cargo klarch`, and does not install its binary under the plain name `klarch`. If both
  were on PATH, `klarch` would run whichever comes first. (Owner, 2026-10-01; the setting-up skill
  states it.)
- **The skill text is a template.** content/ writes the command as a placeholder, and
  `install-agent-skills` fills it with the declared command. The placeholder syntax must differ
  from the reference placeholders in angle brackets; it is chosen in step 3.
- **The binary is named klarch** (owner, 2026-10-01), as the crate ripgrep ships `rg`. This
  changes the earlier decision that the binary carries the full name.

**3. The `[agents]` table.** Illustration of the decided shape:

```toml
[agents]
harness = ["claude"]   # the default when the table is absent
# harness = []         # no agent configuration at all
```

- With `["claude"]`, every component requires CLAUDE.md, as today, and the checks of point 5 run.
- With `[]`, no component requires CLAUDE.md, and there is no install and no installed-file check.
- An unknown value is a phase-1 finding. **Default.**
- A future harness value, such as one for AGENTS.md, requires its own file instead. The
  multi-provider issue extends this list and needs no new mechanism.

**This reverses parts of three recorded core decisions.** The harvest of step 3 owes, for each,
what a reversal owes under recording-a-decision: the head rewritten in place, a judgement of
whether its argument earns a rejected-alternatives entry, stated as strongly as it was made, every
pointer and restatement of the old argument repaired, and a commit message that names the
reversal and states what was searched for the incumbent and what the search returned.

- **components-carry-the-same-documents.** Its head argues: "Which components exist is declared,
  and what a component carries is compiled in. … A project free to declare its own set would be
  conformant with whatever it declared, which is the same as being checked against nothing." It
  also spells the old manifest name in its heading. The owner approved agents-table knowing that it
  changes this decision. A restatement of the same argument sits in walk.rs, lines 121 to 125.
  **Default:** the head is rewritten to make the CLAUDE.md requirement a function of the harness,
  and points at a separate entry agents-table, which owns the table and the install.
- **the-regime-has-no-opt-out.** Its head argues: "The rule set is compiled in and the
  manifest declares nothing about it, so conformance means the same thing in every tree this tool
  checks", and "A declared register adds obligations and removes none". `harness = []` is a
  manifest declaration that removes an obligation. This was not shown to the owner when
  agents-table was approved; shown afterwards, the owner ruled that agents-table stands
  (2026-10-01). This head is reversed in part: a declaration may remove the agent-configuration
  obligations, and nothing else. The owner named the flaw this leaves: with `harness = []`, the
  content routed to CLAUDE.md today (a component's contracts and traps for a developer) has no
  home. The long-term answer is an open issue (section 5).
  Two rejected alternatives lost by the old argument (the core's rejected-alternatives.md, around
  lines 233 and 258) are re-judged, and the heads that lean on it (an-extension-claims-its-manifest
  -tables, and rules-corpus's design home, line 76) are re-read.
- **git-supplies-the-walk.** Its head says "A tracked file cannot leave the walk", meaning that no
  ignore rule can take a tracked document out of every check. Point 5.4 takes the committed
  installed files out of the walk. **Default:** the exclusion is derived from the `[agents]`
  declaration, as the generated indexes are outside the walk by construction, and the head says so.
  nothing-of-a-project-is-compiled-in, which lists what is compiled in, gains the owned namespace.

**4. The command `install-agent-skills`.** It writes every embedded file to its install path,
rendered with the declared command; deletes every file in the owned namespace that the current
version does not ship; and lists what it wrote and what it deleted. **Approved default:** it does
not edit CLAUDE.md. The discussion argued against an installer editing a file the user owns. A
missing import line is a finding whose repair text is the line to add. Under `harness = []` it
writes nothing and says so.

**5. The checks, under `harness = ["claude"]`:**

1. each shipped file is present at its install path, and its bytes equal the embedded template
   rendered with the project's declared command. **Approved default:** a byte comparison, no
   digest. The binary holds the text, so a digest adds a dependency and nothing else. The install
   writes LF line endings.
2. no file in the owned namespace is unshipped. The namespace is
   .claude/skills/knowledge-architect-*/, .claude/agents/knowledge-architect-*.md and
   .claude/knowledge-architect/. The finding says the file is not shipped by this version and must
   be removed. A renamed skill shows as two findings, "remove the old" and "missing the new", and
   the install repairs both.
3. the root CLAUDE.md holds the import line: an at sign followed by the path
   .claude/knowledge-architect/PRIMER.md, alone on its line.
4. the owned paths are outside the walk, derived from the declaration (point 3). Their prose
   contains illustration paths the checker would report, and the byte comparison is their only
   judge. Under `harness = []` there is no owned namespace, and the paths are walked like any
   others.

**Approved default:** these findings belong to phase 2, as a missing required document does
today. So while they stand, the report stops before phase 4 and lists no reference finding. A
project installs first. The phase table in the head phases-gate-the-report gains the row.

**How the checks receive the shipped set. Default:** crates/agent-skills's build.rs walks content/
and generates the list of install paths and texts, so no hand list can drift from the directory.
The core's check takes the shipped set as an input: the binary passes the agent-skills list, and
the tests pass a fixture set. That is how a test fixture can plant a defect of each check while
content/ is still small.

**6. The public API decided in step 4**, 4.4.

**7. `component_dir()` follows the merge.** Today it returns the parent of the library crate's
directory, which was the component. After the merge, the crate directory is the component itself.
**Default:** step 2 makes it return the crate directory. Otherwise it returns crates/, and the
checker-source exemption covers crates/agent-skills too. The head checker-source-literals-are-data,
which says "A library's own directory would not do", is rewritten in the same step.

### 4.4 The public Rust API

Decided and described before v0.1, in a design session that is step 4. Measured on thaum's tree at
commit 5db731e; step 4 re-takes them on this repository after step 2's renames:

| fact | value | how to re-take it |
| --- | --- | --- |
| `pub mod` declared in lib.rs | 15; across all of src/ there are 23 | read lib.rs; grep |
| lines containing "pub " in the library's src/ | 408, of which 405 outside `#[cfg(test)]` items | grep, with a brace-tracked count of test items |
| `pub(crate)` items outside tests | 4 | grep |
| `#[non_exhaustive]` | 0 | grep |
| modules rules-corpus names | 13 | grep for `documentation::` over thaum's tools/rules-corpus |
| distinct paths rules-corpus names | about 53, brace imports expanded | the same grep, expanded |

Examples of what rules-corpus uses: `model::Model`, `extension::Extension`, `check::Inputs`,
`survey::survey`, `source::Literals`, `build_origin::refuse_a_foreign_build`. rules-corpus's tests
also call `check::run_with`, which an open issue of thaum (planted-row-may-name-a-missing-test)
relies on.

Inputs agreed for the session:

- the public surface is what an extension needs plus the CLI module. Every other item becomes
  `pub(crate)`.
- `#[non_exhaustive]` is decided enum by enum, weighing two things: whether the enum is expected to
  grow, and what a consumer would want to use. The owner wants a pass over each enum, not a blanket
  rule, and is not yet sure an exemption from refinement 2 is needed.
- the session re-examines refinement 2 of the versioning policy on concrete cases. Adding a variant
  to a `#[non_exhaustive]` enum is non-breaking by Rust's own definition.

Inputs found while writing:

- **Approved default:** the library's `testing` feature gates nothing today and is dropped. One
  gate existed from commit b70eaef until fcef9fd, which removed it with the rest of a migration's
  scaffolding. thaum's rules-corpus requests the feature in a dev-dependency, and the migration
  removes that request.
- `MANIFEST_NAME` stays public: tools/xtask here and thaum's bench, mutate and xtask use it.
- **Default:** the description is the crate-level documentation of lib.rs, which ships in the
  package and renders on docs.rs, with a pointer to it from the core's README.

The result is harvested as decisions of the core.

### 4.5 Versioning and release

**The policy (owner's scheme).**

| bump | when |
| --- | --- |
| patch | skill changes; code changes that change no check and no CLI |
| minor | a check becomes stricter, or a check is added; orthogonal additions; CLI and manifest changes that only add |
| major | a CLI change that breaks existing usage; a change that may require a layout or content refactor in a project; a breaking change to the library API |

**The five refinements, approved:**

1. **Under 0.x, Cargo has two positions.** A requirement "0.3" accepts 0.3.1 to 0.3.2 and refuses
   0.3 to 0.4. So while at 0.x, the scheme's major and minor both bump 0.MINOR, and its patch bumps
   0.x.PATCH. A stricter check therefore arrives only with a 0.MINOR bump.
2. **The library API is a fourth surface.** A breaking change there makes an extension fail to
   compile. So it is major. Re-examined in step 4.
3. **The manifest test.** Under a minor release, every manifest that was valid stays valid.
   Renaming a key is major, unless the old spelling stays accepted with a deprecation finding.
4. **"Very significant additions" do not make a major.** A major that sometimes means "breaking"
   and sometimes means "large" stops being read as "breaking".
5. **CHANGELOG.md** has one section per version, each item tagged with its surface: checks, cli,
   manifest, library, agent-skills. A release that changes only skill text publishes both crates
   with identical code.

**A patch can bring a finding (owner, 2026-10-01).** A skill change is a patch. After one, the
installed files differ from the embedded templates, so a project that moves to the new patch gets
findings until it runs the install. The owner accepts this. With the exact pin, such a change
arrives only when the project moves the pin. This is new with installed-files-committed: before
it, a patch changed no check, so it could bring no finding.

**The 0.x period.** The project stays at 0.x, with regular breaking changes allowed, until the open
issues of this discussion are implemented, or at least argued thoroughly (owner). Harvested into
the root design home.

**Cargo facts the setting-up skill states:**

- `knowledge-architect = "=0.1.3"` pins exactly. The default "0.1.3" means at least 0.1.3 and
  below 0.2.0.
- Cargo.lock records the exact version. A build rewrites the lock only when Cargo.toml no longer
  matches it, as when the pin moves. cargo update rewrites it to the newest version the requirement
  allows. With --locked, either becomes a failure instead.
- The exact pin makes moving the checker an explicit edit, and the installed skills move with it.

**A material finding on exact-pin (2026-10-01), unacknowledged by the owner; its default stands.** A dependency in Cargo.toml builds no
executable for the project: `cargo run -p` accepts only the packages of the project's own
workspace. thaum pins its checker only because its own crate rules-corpus links the library. A
project without an extension has two ways to run a pinned binary:

- a small crate in its workspace whose main calls the library's CLI, run through a cargo alias, as
  thaum does with its extension;
- `cargo install --locked --root <dir> knowledge-architect --version =x.y.z` into a project-local,
  gitignored directory. The binary lands in `<dir>/bin/klarch`, and the declared command names it.
  This also serves projects not written in Rust.

A plain `cargo install` is machine-wide, which is the single-version problem binary-bundles-workflow
removed for the skills. **Default:** the setting-up skill presents both, the first for Rust projects
and the second for the others, and its wording is settled in step 5. Nothing in v0.1's code depends
on it, because the command is declared.

**The release procedure** (step 7):

1. CHANGELOG.md section written, version set in `[workspace.package]`.
2. A grep of crates/agent-skills/content/ for backticked spans that hold an @ and do not begin with
   an angle bracket returns nothing (shipped-text-is-reference-free, until the mechanical check
   exists). A placeholder such as `<kind>@<anchor>@<id>` is allowed.
3. The crates.io API still answers 404 for both names, or shows them as the owner's.
4. `cargo package --list` for both crates. The owner reads the lists.
5. `cargo publish --workspace --dry-run` passes, skips xtask, and orders agent-skills first. This
   behaviour of `--workspace` with `publish = false` is not documented in the cargo 1.98 manual and
   is established by this run.
6. **The owner's word**, given at that moment. Publishing is irreversible: a version can be yanked,
   never deleted.
7. `cargo publish --workspace`, then the tag v0.x.y on that commit, pushed.

A merge to main publishes nothing. Any number of merges land between two releases.

### 4.6 This repository's own process

- **Git:** thaum's root CLAUDE.md section Git. It is restated in this repository's root CLAUDE.md,
  and its pointer names a root design entry git-flow, harvested in step 2. All work on a branch, a
  draft PR after the first commit, a rebase before review, a review before merge, a fast-forward
  merge after CI passes. The one exception is import-to-main.
- **Every commit of a branch must pass the check under the branch tip's checker.** The commits
  gate judges each commit's tree with the tip's binary, and refuses a tree with no manifest or with
  findings (the core's cli/history.rs: "every commit of the range must load under the tip checker
  … put the change that needs it in the branch's first commit, or squash the branch"). So a step
  whose intermediate trees cannot pass lands as one commit, squashed before review. Review repairs
  are new commits after it, each passing.
- **The root CLAUDE.md** carries, adapted from thaum's root CLAUDE.md at e98e296: the language and
  style instructions, the mechanical validation of documents without the rules extension, where
  knowledge goes, verify before relying on anything, verify mechanically, the skills table (until
  step 5 makes it the routing table), Git, and release status. It carries nothing about the
  Comprehensive Rules.
- **Gates:** tools/xtask with `cargo x gates`, copied from thaum at e98e296. Its subcommands today
  are gates, timings and bench; timings and bench are removed. run.rs is the shared module that
  gates.rs uses, and it stays whole. The gate list keeps rebased, fmt, the checker gate, commits,
  clippy and test, in that order. **Approved default:** the checker gate, named `knowledge` today,
  is renamed `check`; its name is asserted at gates.rs line 640 and line 764, and in
  tests/gates_bin.rs lines 210 to 212. The checker and commits gates run the package
  knowledge-architect instead of rules-corpus.
- **What leaves xtask with its subcommands:** the design entry timings-via-bootstrap-hatch, the
  tripwire of the same name, the open issue timings-fold-into-the-gates, and the cargo-nextest
  rejected alternative. The other design entries come with the port.
- **Aliases:** `cargo x` runs xtask. **Default:** `cargo klarch` runs the checker built from the
  checkout, in release mode, as thaum's alias does, and the manifest declares it as the command.
- **Dogfooding:** from step 5, this repository runs `cargo klarch install-agent-skills` and
  commits the result. A change to content/ is followed by a re-install in the same commit, or the
  check fails. **Approved default, corrected:** the `[agents]` table exists from step 3, and from
  then until step 5 this repository's manifest declares `harness = []`. Step 2 cannot declare it,
  because the step 2 checker refuses a table no extension claims.
- **content/ and this repository's own walk.** content/ holds the same prose as the installed
  copies, with the same illustration paths. **Default:** this repository's manifest excludes
  crates/agent-skills/content from the walk, with the reason written beside the row; the release
  grep and the reviews judge it until the mechanical check of shipped text exists.
- **CI:** thaum's ci.yml at e98e296, with its comments adapted.
- **Skills and reviews before step 5.** This repository has no skills of its own until step 5.
  **Default:** steps 2 to 4 follow thaum's skills, read from thaum's checkout at e98e296: developing
  for the Rust work (without `cargo mutate run`, which is not ported; a test is shown to
  discriminate by reverting the change in a scratch worktree), tracking-open-issues for section 5,
  recording-a-decision for the harvests, and dispatching-a-review for the reviews. thaum's reviewer
  agents are not loaded in this repository: each review is a general-purpose subagent briefed with
  the path of thaum's agent file, told that its thaum-specific parts (the rules, slices.md,
  `cargo knowledge`, thaum's anchors) do not apply. The axes of each step are named in section 9,
  with thaum's axis names.
- **When the owner is needed and absent.** A step that needs the owner's word (goals, the public
  API session, the intake, a public push, the publish) stops at that point, and says what it waits
  for in its PR description.

### 4.7 The installed skills and agents

Every installed skill and agent carries the prefix knowledge-architect- in its directory or file
name and in its frontmatter name. The tables give the base names.

| skill | source | what changes in the fork |
| --- | --- | --- |
| setting-up | new | the manifest and the homes, the `[agents]` table, the declared command and the extension rule of 4.3 point 2, the install, the primer import line, the routing table, the xtask gates convention, the exact pin and how a pinned binary is run (4.5), the requirement of at least one goal per component |
| setting-goals | new | eliciting goals from the owner; how a goal binds design in the long term; brevity; goals are the owner's intent and never the agent's; goal-lifecycle: a goal leaves only when the owner abandons it |
| designing | designing-together 0.6.0 | unified per 4.8 (step 6) |
| planning | thaum's planning-a-slice | the spec path and the milestone path; the vocabulary; the transcript-conformity review; the owner's content rule; what replaces slices.md as the list of milestones and the home of acceptance criteria is decided in this step |
| recording-a-decision | thaum's | thaum's examples and its rules-specific arms removed. Arms that name a crate boundary are made language-neutral. |
| tracking-open-issues | thaum's | thaum's locations, worked examples and rule quotes removed |
| dispatching-a-review | thaum's | the Rust worktree paragraph made generic, with a note for Rust projects; slice axes renamed for milestones. **Default:** the cold-implementer exclusion "never for a spec" is removed, because its premise is gone: a later session can continue a spec. |
| maintaining-agent-config | thaum's | thaum's examples removed; the routing table and the prefix rule added |
| retrospective | new | the agent offers it at a moment it judges right, at the end of a session that finished a piece of work under this workflow, and runs it only if the owner allows it. Its purpose, in the owner's words: "reflect on how the workflow was used, and give constructive remarks about what could be improved, what might be missing, what is unclear". It writes its file, asks the watch points of section 7, and offers an issue on this repository. |

| agent | source | what changes |
| --- | --- | --- |
| standing-state-reviewer | thaum's | thaum's locations and the slices.md section removed; what replaces that section follows the planning skill's decision on acceptance criteria |
| decision-record-reviewer | thaum's | the restated thaum arms replaced by a pointer to the installed recording-a-decision |
| routing-reviewer | thaum's | the section names of thaum's root CLAUDE.md replaced by what the setting-up skill writes |
| code-claims-reviewer | thaum's | "slice document" becomes "spec or milestone document" |
| cold-implementer-reviewer | thaum's | the same |

Not installed, and staying in thaum: developing, recording-an-interpretation, bumping-rules,
rules-reviewer. A future creating-a-component skill is an open issue.

**The primer.** PRIMER.md is at most 40 lines (the agent's proposal when the primer was a hook,
approved with it). It states how to read a goal against a design head, names the installed skills
and when each applies, and points at them without restating them.

**Order inside step 5.** The forked planning and dispatching skills name the designing skill,
which step 6 creates. Until step 6 they name it by its final installed name. The check does not see
a one-segment name, so nothing fails in between.

### 4.8 The unified workflow

These resolve the conflicts between designing-together and thaum's skills that the discussion
settled.

- **Thread names.** A thread slug is minted in the grammar `[a-z0-9]+(-[a-z0-9]+)*` and checked for
  a collision with the declared command's `show` subcommand before it is used. In discussion prose
  it is written #slug, never as a backticked span. An approved thread becomes a design entry whose
  heading ends with the same slug.
- **When recording happens.** At harvest, when the implementing work lands, not when the spec is
  written. A design head is a claim about the code as it stands, and a head written before the code
  is a hypothesis. While the work is open, the spec on its branch is the only place the decision
  exists.
- **Losing alternatives.** recording-a-decision's tests decide which earn an entry. The rest stay
  in the spec and the commit message.
- **Tripwires from a premortem** are written at harvest, in the tripwires home of the component
  that owns the guarded decision, and only on the owner's word.
- **Spec and milestone.** One document per layer. It is detailed about the design and concise about
  the implementation sequence. **No untested code snippet is presented as authority.** If a
  detailed implementation plan is ever written for a lower-tier implementer, it covers a bounded
  amount of work and opens with a disclaimer that its content rests on assumptions and may be
  wrong. **Default:** a snippet in a spec is labelled as an illustration of a shape.

**Not settled in the discussion, resolved with the owner:**

- in step 5: whether the spec is kept after harvest (thaum deletes it at merge; designing-together
  leaves it to the project);
- in step 5: where the argument for a decision lives (designing-together puts it in the record;
  recording-a-decision puts it in the commit message);
- in step 5: where withdrawn and superseded threads go;
- in step 6: the material-findings test. designing-together states it; thaum's planning-a-slice
  rewords it and adds "with a default named, and the thread stays closed until the owner's word".

### 4.9 The designing-together intake (step 6)

- The skill is forked from designing-together 0.6.0, on its branch `next`.
- Its decision record, docs/decisions.md on `next`, has 18 level-two sections
  (`git show next:docs/decisions.md | grep -c "^## "`). One of them, "How the decision record is
  kept", is about the file itself. The owner asked for each item to be listed. **Default:** an item
  is one dated entry or one rejected bullet, shown grouped under its section with the section's
  head. For each item the agent proposes a wording. The owner rules on keeping it and on the
  wording. A kept item becomes a slugged entry in the agent-skills design home, or a rejected
  alternative, by recording-a-decision's rules. A dropped item is listed in the commit message.
- The dated history is not carried. It stays in the archived repository.
- The README's content is kept: what the skill does, what it expects of the user, and its grounding
  in the literature. **Default:** it goes into the agent-skills component's README, adapted.
- **Default:** shown to the owner in the same way, item by item: docs/open-items.md (its live
  tripwires and parked predictions) and the operational rules of its CLAUDE.md, such as "Editing
  discipline: no behavioral change without evidence". The field reports are evidence, not
  decisions, and are not carried.
- The regression harness (tests/scenarios, tests/conductor.md, the workflow script) is not
  reproduced. Real use is the test, and the retrospective collects it.
- The owner archives the old repository.

## 5. Open issues to write in this repository

Each is one file in the owning component's open-issues directory, under thaum's tracking-open-issues
standard. Each carries only what the discussion said. **An entry is opened in the step that lands
its premise**, never before: an entry describing a mechanism that does not exist yet is a false
statement about the tree (the review of step 2 found five). An entry whose only trigger is the
owner's scheduling is a `todo`, not a `deferred`.

| issue | component | kind | opened in | what it holds |
| --- | --- | --- | --- | --- |
| cross-project-references | core | deferred | step 2 (done) | the parked thread, 3.4 |
| a-component-states-at-least-one-goal | core | deferred | step 2 (done) | the goal check. Trigger: the first release after thaum's 8 components without a goal have one. |
| structured-plan-documents | core | todo | step 2 (done) | a structure for plan documents, with registers such as planned design items. Fully open. |
| tooling-for-project-skills | core | todo | step 2 (done) | checks over the structure of a project's own skills. Long term. |
| configuration-for-several-agent-providers | core | deferred | step 2 (done) | AGENTS.md as the generic file, with other providers' files as symlinks or imports |
| xtask-abort-exits-one | xtask | observation | step 2 (done) | found by the review of step 2: no exit-code contract binds xtask |
| a home for developer contracts outside agent configuration | core | deferred | step 3 | with `harness = []`, the content routed to CLAUDE.md loses its home; the owner's long-term answer is a home independent of any harness |
| shipped text is reference-free, mechanically | agent-skills | deferred | step 5 | a check replacing the release grep and the walk exclusion of content/ |
| skill patching | agent-skills | deferred | step 5 | patches stored as diffs per project. Answers watch point P1. |
| creating a component | agent-skills | todo | step 5 | a future skill; the owner has not converged on what a new component requires |

The core's issue the-core-leaves-this-repository is closed by step 2.

## 6. Facts the arguments rested on

### 6.1 Measured on thaum's tree

| fact | how to re-take it |
| --- | --- |
| 160 of the 549 commits on thaum's main touch tools/knowledge; the newest is e98e296 | `git log --oneline -- tools/knowledge`, counted, and `git rev-list --count origin/main` |
| every file of the core was added under tools/knowledge/ and never renamed into it from elsewhere | git log with rename detection, destinations under tools/knowledge/ with a source outside: no result |
| 176 renames moved files out of tools/knowledge, all into tools/rules-corpus, so the filtered history holds the rules extension's early history and then its removal | git log with rename detection, sources under tools/knowledge/ |
| 144 tracked files, 103 of them fixtures under tests/projects; 912 558 bytes | git ls-files, and du -cb over them |
| 4 of thaum's 12 components have a goal entry | a grep for slugged level-two headings in each goals.md |
| 113 lines in 35 files outside tools/knowledge hold "@knowledge@" (115 occurrences); 297 lines inside (301 occurrences) | `git grep -c` and `git grep -o` at e98e296 |
| the root finder is hard-coded to "knowledge.toml" in three binaries: bench, mutate, xtask | `git grep -n` on the literal over tools/bench, tools/mutate, tools/xtask |
| 10 mock manifests: 5 under the core, 5 under rules-corpus | git ls-files, filtered on the manifest name |
| the core's tripwires home has 10 entries | a grep for slugged level-two headings |

### 6.2 Harness facts, from the Claude Code documentation via subagents

- A subagent `name` cannot contain a colon. Letters, digits and hyphens only.
- A project skill and a plugin skill with the same base name both load.
- A plugin's install cache is shared across projects, with one directory per version. A
  project-scope install is recorded in the user-level installed_plugins.json with its own version.
  binary-bundles-workflow made the question of two versions at once moot.
- The `@path` import in CLAUDE.md is documented, and the owner is confident it works. It was not
  exercised in the session. Step 3 verifies it.

### 6.3 The state of the skills at the start (one subagent, 2026-09-30)

- Thaum-only: developing, recording-an-interpretation, bumping-rules, rules-reviewer.
- Mostly generic, with thaum lines: tracking-open-issues, dispatching-a-review, planning-a-slice,
  recording-a-decision, maintaining-agent-config, and the five other agents.
- Seven conflicts between designing-together and thaum's skills. Numbers 1, 2, 3 and 5 are settled
  in 4.8; 4, 6 and 7 are open there:
  1. thread names against slugs;
  2. when recording happens (designing-together's step 9 records before implementation;
     planning-a-slice records at each harvest);
  3. where and when tripwires are written;
  4. whether the spec is kept;
  5. which losing alternatives are recorded;
  6. where the argument lives;
  7. withdrawn and superseded threads have no home in thaum other than the commit.
- Inside thaum: routing-reviewer calls a slug on unbuilt work a finding, while thaum's root
  CLAUDE.md records a spec's decisions before implementation. harvest-after-implementation removes
  the tension.

A session that forks a skill re-reads the source file. These facts are where to look, not a
substitute for the text.

## 7. Premortem and watch points

Assume v0.1 shipped and the project failed. The causes found:

| # | cause | thread stressed | where it goes |
| --- | --- | --- | --- |
| P1 | a project needs to change an installed skill, the check refuses, and the project disables `[agents]`, losing the overlay rule | installed-files-committed, overlay-by-separate-skills | watch point (owner); the skill-patching issue |
| P2 | the `@` import is not followed in some context, such as a subagent or another harness, and the primer is absent there | primer-by-import | watch point (owner); step 3 verifies the import first |
| P3 | an agent follows an installed skill and misses the project's additions | overlay-by-separate-skills, routing-table | watch point (owner) |
| P4 | a live reference ships in content/ | shipped-text-is-reference-free | the grep in the release procedure |
| P5 | a published package is broken, and only a yank can follow | thaum-dependency-source | the package list and dry run in the release procedure |
| P6 | thaum's renames dangle pointers into its skills | skill-name-prefix | thaum's migration spec |
| P7 | the decisions live only in this document across several sessions, and a session implements against a stale part of it | harvest-after-implementation | this document is corrected in place whenever a step changes it |

The owner ruled on P1 to P3: none is a tripwire, and each is a question in the retrospective skill:

- did this session need to edit an installed skill, and what for?
- was the primer present in this session, and in its subagents?
- did this session miss something a project skill adds to an installed skill?

## 8. Harvest map

Where each decision lands, and the step whose landing writes it. Each component's design home is
its docs/design.md.

| step | component | decisions |
| --- | --- | --- |
| 2 | knowledge-architect (root) | repo-layout, crate-directory, two-crates, version-lockstep, versioning-policy, the 0.x period, package-include-whitelist, git-flow (the restated Git section's own entry) |
| 2 | core | single-crate; binary-name; the manifest file name, written into the three heads that spell the old name (nothing-of-a-project-is-compiled-in, components-carry-the-same-documents, and the head near line 858 of the core's design home); `component_dir()` (rewrites checker-source-literals-are-data) |
| 2 | xtask | xtask-gates; the gate rename; the root finder's use of `MANIFEST_NAME` |
| 3 | core | agents-table, with the three reversals of 4.3 point 3; install-command-name; owned-namespace-check; installed-files-committed (the check side); primer-by-import (the check side, and that the install does not edit CLAUDE.md); declared-command; the phase of the new findings (phases-gate-the-report) |
| 3 | knowledge-architect (root) | binary-bundles-workflow |
| 4 | core | public-api |
| 5 | agent-skills | goal-lifecycle, plugin-inventory, installed-prefix-length, overlay-by-separate-skills, routing-table, skill-name-prefix, shipped-text-is-reference-free, goals-required, gates-convention, exact-pin, installed-files-committed (the instruction side), primer-by-import (the delivery side), declared-command (the extension rule), thread-slug-is-entry-id, harvest-after-implementation, losing-alternatives-filter, spec-and-milestone, document-vocabulary, transcript-conformity-review, retrospective-destination, premortem-as-watch-points, the interim rule of cross-project-references |
| 6 | agent-skills | designing-together-retirement, the kept items of the intake |

**Losing alternatives.** Section 3.2 and the absorbed shapes of 3.3 are judged against
recording-a-decision's tests at the harvest of the decision they lost to. The likeliest to pass:
the crate directory named after its package, which a run refuted (an argument that cost work), and
the SessionStart hook, whose loss rests on primer-by-import reaching every context, which premortem
P2 doubts.

**Commit message only.** These are decisions about this work, not about how the project is built:
history-import, import-to-main, thaum-dependency-source, v01-scope, session-sequence, spec-home,
thaum-switch-today, license (the license files and the Cargo metadata carry it).

## 9. Implementation sequence

Each step is one branch and one PR. Each harvests its decisions (section 8) in the commit that
lands it, and corrects this document where it changed anything. Every commit of a branch must pass
the check (4.6). The review axes are thaum's dispatching-a-review axes, briefed as 4.6 says, until
step 5 installs this repository's own.

**Step 1. Import.** Reads 3.1 (history-import, import-to-main) and 6.1. **Done** on 2026-10-01:
main is 12b56be, 160 commits, pushed on the owner's word.

- `git clone --no-local` of thaum's checkout into a scratch directory: git filter-repo refuses a
  clone made with hard links. Check out main, at e98e296.
- `git filter-repo --path tools/knowledge/`, and no other option. **Approved default:** no path
  rename here. The owner ruled "no edits" on the import, and a rename rewrites every commit's tree.
  Step 2 moves the files with git mv, and rename detection follows them.
- filter-repo removes the clone's remote. Add this repository's origin as a remote.
- Done when: the result holds 160 commits, and its newest has the same message and author date as
  thaum's e98e296.
- **Then stop for the owner's word**, because the push makes the history public. Push it as main.
- The imported messages refer to thaum's anchors and tree. No gate judges them, because the commits
  gate reads only the range from origin/main to HEAD.
- The local checkout fetches main and creates the step 2 branch. This document stays untracked
  until it is committed with step 2's work.

**Step 2. Restructure into a working repository.** Reads 4.1, 4.2, 4.3 points 1 and 7, 4.6, 5, 8.
**Landed on the branch restructure.** The 0.x period is the root entry stays-at-zero-x; the root
finder's use of the constant is the xtask entry root-finder-uses-the-core-constant. The port of
the gates and the gate's rename were judged to need no entry of their own: the xtask design home's
introduction and its gate list state them. The review also produced root entries
toolchain-is-pinned and a-reference-claims-a-revisit, and the xtask entry
gates-scrub-rustc-bootstrap, for decisions the extraction carried without a home. The printed
command became `cargo klarch` until step 3 makes it declared. The agent-skills component's goals
home holds no entry until the setting-goals pass of step 5.
Lands as one commit, squashed before review (4.6).

- **The crates.** git mv of tools/knowledge to crates/core. Then the merge: documentation/src/*
  moves to crates/core/src/, the former binary's src/main.rs becomes crates/core/src/main.rs, the
  package is `knowledge-architect`, the library is imported as `knowledge_architect` (every
  `documentation::` path in main.rs and the tests changes), and a `[[bin]]` declares `klarch`.
  crates/agent-skills gets its Cargo.toml and a src/lib.rs that exposes an empty list, so the
  workspace builds; its build.rs and content/ come in step 3.
- **Renames.** The manifest, its 5 mock manifests under the core, and the literal in the core's
  finding strings. `component_dir()` per 4.3 point 7.
- **References.** Every reference to the knowledge anchor becomes the core anchor, and every path
  into the former documentation crate drops that segment. The core's references outside itself:
  docs/design.md names the worktrees directory under the agent-config location (it resolves once
  that location is declared and .gitignore covers .claude/worktrees), docs/rejected-alternatives.md
  names xtask's rejected-alternatives.md (it resolves once xtask's documents are ported), and the
  issue the-core-leaves-this-repository names a thaum decision (it closes in this step). The ported
  xtask documents hold 13 references to thaum's anchors (thaum 12 with bench and knowledge, counted
  over docs/, README.md and CLAUDE.md; 18 with the sources, of which bench.rs is not ported). Each
  is retargeted to this repository where the target exists, and otherwise rewritten as prose or
  removed.
- **Tests.** The tests that read thaum's layout, listed in the issue
  the-core-leaves-this-repository, are rewritten. The mocks' exclusion row moves to the new manifest. Fixtures that only use the
  string tools/knowledge as in-memory data (cli/mod.rs, model.rs) may keep it. The issue then
  closes.
- **Documents.** The examples drawn from thaum stay where they are evidence, as that issue says.
  The root component's documents, its CLAUDE.md per 4.6. Goals: the root's goals are
  docs/goals.md, already written by the owner (root-goals), committed unchanged. The core's goal
  documentation-half-publishes-alone is removed (core-goal-abandoned). The other components' goals
  come with the setting-goals pass of step 5. The agent-skills component's documents. The agent-config location
  with its open-issues README and index. tools/xtask per 4.6.
- **Files.** .gitignore, the aliases, the toolchain and rustfmt files, CI, the licenses and their
  copies, CHANGELOG.md, the include whitelists.
- The open issues of section 5 whose premise exists at step 2.
- **On the owner's word:** the repository description and topics (4.2).
- Done when `cargo x gates` passes on the branch and CI passes on the PR.
- Review axes: self-consistency, fidelity of relocation, decision recording, routing of knowledge,
  conformance.

**Step 3. The checker's new surface.** Reads 4.3 and 6.2.

- First, the `@` import. In a scratch project, a CLAUDE.md imports a file holding one recognisable
  sentence. The agent runs `claude -p` there, asking a session to quote the sentence, then asking
  it to dispatch a subagent that quotes it. The result is written into 6.2. If either fails, the
  step stops and the owner rules, because primer-by-import rests on it.
- The issue on a home for developer contracts outside agent configuration (section 5) is opened
  with the `[agents]` table.
- Then the declared command, the `[agents]` table, the conditional CLAUDE.md requirement,
  `install-agent-skills`, the build.rs of agent-skills, the checks of 4.3 point 5, and the walk
  exclusion of content/ in this repository's manifest. This repository's manifest declares
  `harness = []` in the same commit.
- Fixtures. The existing mocks declare `harness = []`. So do the projects tests build in code (the
  sandbox helpers of tests/binary.rs). One new mock carries an installed set, with one planted
  defect of each check, against a fixture shipped set passed by the test. Each test is shown to fail
  without the code.
- Done when the gates pass.
- Review axes: spec conformity against 4.3, self-consistency, decision recording (the reversals).

**Step 4. The public API.** Reads 4.4 and 4.5.

- A design session with the owner, on the inputs of 4.4. Then the visibility changes, the
  `#[non_exhaustive]` decisions, the description, the removal of the `testing` feature.
- Done when the gates pass and the description is written.
- Review axes: spec conformity against the session's outcome, decision recording.

**Step 5. The installed skills and agents, except designing.** Reads 4.6, 4.7, 4.8, 7.

- Forked and unified per 4.7 and 4.8, with the open points of 4.8 brought to the owner. The new
  ones are written with the owner: setting-up, setting-goals, retrospective, the primer, and the
  transcript-conformity reviewer if the owner chooses an installed agent.
- The manifest switches to the default harness, and the repository installs and commits its own
  skills.
- The three agent-skills issues of section 5 are opened when the install and the skills they
  depend on exist.
- **Approved:** the setting-goals skill is then run with the owner on every component of this
  repository, as its first real use.
- Several PRs are allowed, one per group: the recording group (recording-a-decision,
  tracking-open-issues), the review group (dispatching-a-review and the agents), the planning group
  (planning, maintaining-agent-config), the new skills.
- Review axes for a skill: self-consistency, routing of knowledge, and whether it names anything of
  one project (nothing-compiled-in).

**Step 6. The designing-together intake.** Reads 4.8 and 4.9.

- The fork of the designing skill, unified per 4.8. Then the record, item by item with the owner.
- Review axes: transcript conformity against the intake session, with the transcript extracted as
  the head says, and self-consistency.

**Step 7. Release 0.1.0.** Reads 4.5.

- The release procedure of 4.5. This document is deleted in the release commit.

Then thaum's migration spec applies.
