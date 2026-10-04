# knowledge-architect — design

Recorded intent for the project as a whole: how the repository is laid out, how its parts are
versioned and released, and how work reaches main. Present tense, each decision carrying a slug,
cited as `design@knowledge-architect@<slug>`. What lost to a decision here is
`path@knowledge-architect@docs/rejected-alternatives.md`. The goals these decisions serve are
`path@knowledge-architect@docs/goals.md`.

**What belongs here:** a decision that survives deleting any one component. How the checker works
is `path@core@docs/design.md`; how the gates run is `path@gates@docs/design.md`, and this repository's own gates
`path@xtask@docs/design.md`; how the agent
skills are shaped is `path@agent-skills@docs/design.md`.

## 1. Layout and packaging

### The repository is a virtual workspace of components, each crate in a directory named by its role `##repo-layout`

The root holds the workspace manifest, the checker's manifest and the project's documents, and no
crate. Each crate sits under crates/ in a directory named by what it does: crates/core is the
checker, crates/agent-skills carries the text the checker installs, crates/gates runs a project's
merge gates. The maintenance tool sits under tools/xtask. Each of the four is a component, so each carries its own design home, goals and
registers, and a decision about one of them lives beside its code.

A crate directory is never named after its package. A component is named by the basename of its
directory, and the project root by the project's name, so a directory named `knowledge-architect`
would collide with the root: a run over a scratch project laid out that way stopped in phase 1,
reporting that the name "names 2 anchors". A future split of the core follows the same rule, as
crates/core and crates/cli.

### The checker is the package knowledge-architect, in crates/core `##crate-directory`

Users install and depend on `knowledge-architect`. The package lives in crates/core and its
component is named `core`, so a reference says `core` where cargo says `-p knowledge-architect`.
That difference is the cost of `design@knowledge-architect@repo-layout`'s naming rule, and it is
the usual shape of a Rust workspace.

### The installed text lives in its own crate, which the checker depends on `##two-crates`

The skills, subagent definitions and primer that the checker installs are a separate package,
`knowledge-architect-agent-skills`, in crates/agent-skills. The checker depends on it by path and by
exact version. A package ships only the files under its own directory, so text embedded with
`include_str!` must sit inside a package; giving the text a crate of its own keeps it a component
of its own, with its own design home, rather than a part of the core.

A user names only `knowledge-architect`, and cargo fetches the other. The two are published
together, at one version.

### Each published crate ships a whitelist of files `##package-include-whitelist`

Each crate under crates/ declares `include` in its Cargo.toml, listing its sources, its Cargo.toml, its README and its
license files, and nothing else. Cargo adds the file `readme` names, its CRATES-IO.md, whatever
`include` says, per `design@knowledge-architect@crates-io-page-file`; `cargo package --list -p
<crate>` re-takes it, and the release procedure runs it. A new directory is shipped only once it is listed, so the test
fixtures and the design documents never reach a package, and their growth cannot push a package
past the crates.io size limit. The cost is accepted: `cargo test` cannot run from a downloaded
package. The license files sit at the root and as copies in each directory under crates/, because an
`include` path cannot reach outside the package directory. tools/xtask is never published, so it
carries neither. This keeps what a project receives to what it uses, per
`goal@knowledge-architect@any-project-can-adopt-it`.

### A published crate's crates.io page is a short CRATES-IO.md, and its README stays repository-facing `##crates-io-page-file`

Each crate under crates/ names a CRATES-IO.md beside its README as its crates.io page, with
`readme = "CRATES-IO.md"` in its Cargo.toml. The page says what the crate is, how to install or use
it, and links, as absolute URLs on the main branch, to the crate's README in the repository and to
docs.rs. It restates no contract, so it cannot drift from the README, and it holds no backticked
reference: a reference that resolves passes the check, and crates.io renders it as dead code. The
README stays the crate's home for how a user uses it, per the knowledge table, with the references
the checker resolves. The rival, a README rewritten for crates.io with its repository-facing text
moved elsewhere, lost: it moves the most text, and it changes what a Component's required README
holds. Every published crate gets the page, so the rule has no exception. This serves
`goal@knowledge-architect@adoption-is-easy`.

### The checker carries the agent workflow and writes it into a project `##binary-bundles-workflow`

The skills, subagent definitions and primer of the agent workflow are embedded in the checker, and
a command of the checker writes them into a project, rendered for it. There is no plugin and no
marketplace. One artifact has one version, so the installed workflow is always the one the
project's pinned checker describes, per `design@knowledge-architect@version-lockstep`, and every
project holds the version it pinned, whatever another project on the same machine uses. Whether
the installed files match is checked like any other part of the tree, per
`design@core@owned-namespace-check`, rather than warned about. A project installs one thing, with
one command, and a later provider's layout is a target the same command writes, which serves
`goal@knowledge-architect@any-project-can-adopt-it`.

## 2. Versions and releases

### Every crate carries one version, and so does the installed text `##version-lockstep`

`[workspace.package] version` is the version of every crate. The skills a checker installs are the
skills of that exact version, and the core pins the agent-skills crate at it exactly. A project
therefore never runs a checker whose installed skills describe another version's commands, which
is what lets a project pin one version and move when it chooses, per
`goal@knowledge-architect@any-project-can-adopt-it`. The gates library is in the same lockstep: the
setup skill that recommends it is the same version's, so a project pins one version for all of
it. The cost accepted: a fix to the gates alone is released as a version of every crate.

### Versions follow the owner's scheme, mapped onto Cargo's two positions under 0.x `##versioning-policy`

| bump | when |
| --- | --- |
| patch | skill changes; code changes that change no check and no command |
| minor | a check becomes stricter, or a check is added; additions that change nothing existing; command and manifest changes that only add |
| major | a command change that breaks existing usage; a change that may require a layout or content change in a project; a breaking change to the library API |

- **Under 0.x, Cargo has two positions.** A requirement "0.3" accepts 0.3.2 and refuses 0.4. So
  while at 0.x, major and minor both bump 0.MINOR, and patch bumps 0.x.PATCH.
- **The library API is a surface of its own.** A breaking change there makes an extension fail to
  compile, which is worse than a new finding, so it is major. Under 0.x a library break and a
  stricter check therefore get the same version number, 0.MINOR, by the point above. The rule
  still keeps a library break out of a patch. Which changes to the library are
  breaking is narrowed by `design@core@ne-minimal`. The gates library's API is a surface of the
  same kind: a break there makes a project's maintenance binary fail to compile.
- **The manifest test.** Under a minor release every manifest that was valid stays valid. Renaming
  a key is major, unless the old spelling stays accepted with a deprecation finding.
- **Size does not make a major.** A major that sometimes means "breaking" and sometimes means
  "large" stops being read as "breaking".
- **A patch can bring a finding.** A skill change is a patch, and after it a project's installed
  files differ from the new text until it runs the install. A project that pins the checker
  exactly meets this only when it moves the pin.
- **CHANGELOG.md** has one section per version, and one working section above them, `Next
  release`, which the release renames to its version. Which change gets an entry, and with which
  class, is `design@knowledge-architect@changelog-entries`.

A version stricter than a patch never reaches a project through a plain `cargo update` while at
0.x, and a release that changes only skill text publishes all three crates with identical code. A
project can therefore tell from the version alone whether moving to it may require work, per
`goal@knowledge-architect@any-project-can-adopt-it`.

### The changelog records what a consumer must plan for, and the branch that makes a change writes its entries `##changelog-entries`

An entry of CHANGELOG.md is written for a consumer planning a move to the version. It exists when
the change passes one of three tests, and sits under the subsection of that test:

- **Migration**: one entry per thing a consumer must change in its own files, saying what.
  Running the install of the agent skills again is never an entry; a change the consumer must make
  to its own files because of the new skills is one.
- **New features**: something a consumer can start using, in one line; the documentation carries
  the detail.
- **Workflow**: a change to the installed skills that a person watching agent sessions would
  observe: a new or removed action, file, commit, pull-request shape, or question put to the owner.
  A rewording or a clarification is not one.

Each entry carries the surface it touches, one of checks, cli, manifest, library, agent-skills,
gates, since the version number alone cannot say which, and the bump class
`design@knowledge-architect@versioning-policy` gives it. **A change with no entry is at most a
patch**: every minor or major change passes the migration or the new-feature test. So the
release's bump is the highest class among its entries, a patch at least, and a review of the
release can tell a missing entry from a change that owes none.

The branch that makes a change writes its entries, because its author knows what changed and in
which class at that moment; rebuilt at the release from commit messages, an entry is lost when
nothing asks for it, as one change to a shipped skill after 0.1.0 was. The release is reviewed
once against these tests, rather than every merge, because a release branch can repair any gap
before anything is published. The working section may be reworded, restructured or pruned at any
time, and a change reversed before the release leaves it, since the section describes the
release's net effect. A released section's content never changes; its structure may.

### The project stays at 0.x until the owner's word, given once its first design discussion's open issues are settled `##stays-at-zero-x`

Breaking changes are allowed, and expected, while the shape of the tool and of its workflow
converges. The project leaves 0.x only on the owner's word, and not before the open issues recorded
from the discussion that designed it are implemented, or at least argued thoroughly: both are
required. Whether the tool and its workflow have converged is a weighing, and the weighing is the
owner's, per `goal@knowledge-architect@the-owner-decides`. The open issues are
`issue@core@tooling-for-project-skills`, `issue@core@configuration-for-several-agent-providers`,
`issue@core@a-component-states-at-least-one-goal`, `issue@core@cross-project-references`,
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`,
`issue@agent-skills@shipped-text-is-reference-free-mechanically`,
`issue@agent-skills@patching-an-installed-skill`,
`issue@agent-skills@a-skill-for-creating-a-component` and
`issue@agent-skills@a-skill-for-bounded-problems`.

Leaving 0.x re-examines `design@core@ne-minimal`. After 1.0, a variant added to one of the
library's exhaustive enums is a major, where under 0.x it bumps 0.MINOR.

### A decision is never argued on the grounds that changing it later would be breaking `##no-future-breaking-cost-argument`

An argument that a change is expensive names the cost it has today, a consumer's migration
included. While the project stays at 0.x, breaking changes are allowed, per
`design@knowledge-architect@stays-at-zero-x`, so a cost that exists only in a future where
changes are refused argues against nothing; accepted, it would freeze the design before it has
converged. A consumer that pins a version pays a migration only when it moves its pin, and that
migration is a present cost, named in the changelog's migration entries.

### A version is published from main, after its merge, on the owner's word `##publish-after-merge`

A merge to main publishes nothing, and any number of merges land between two releases. A release
is a branch like any other: reviewed, merged once CI passes on its head, and only then tagged and
published from main's head, on the owner's word given at that moment. The order follows what can
be undone. A mistake on main is repaired by a later commit, while a version published on crates.io
can be yanked and never deleted, so the irreversible step comes last, after review and CI have
judged exactly what it publishes, and the tag names a commit on main. The cost: main states the
new version for the minutes between the merge and the publish, and a failed publish leaves that
statement false until a repair lands. The procedure is
`path@agent-config@skills/klarch-release/SKILL.md`.

## 3. How work reaches main

### All work goes through a branch, a review and a pull request merged up to date with main `##git-flow`

- All work happens on a branch. Once it holds a first commit, it is pushed and a draft pull
  request is opened. CI does not run on a draft.
- No operation that can lose content, committed or not. With a clean tree, editing the branch's
  own history is an ordinary move, bounded by verifying that nothing was lost.
- The branch is rebased on main before review and merge, and reviewed before any merge. A repair
  from a review is a new commit, so that no history is edited for it; a repair that would leave an
  earlier commit failing the per-commit rule below is folded into the earliest commit it repairs,
  and the commit recording the review says what was folded. The rule exists to avoid history
  edits, not to keep repairs apart, and the owner's reason for the exception is that this
  repository keeps rewriting its checker, so a repair that changes what it judges is no rare case.
- **Every commit of a branch passes the check under the branch tip's checker**, as
  `design@core@a-commit-message-is-a-document` decides. So work whose intermediate trees cannot
  pass lands as one commit, squashed before review.
- main accepts no direct push. A GitHub ruleset on main, with no bypass, the owner included,
  requires a pull request, the status check of CI on a branch up to date with main, and a linear
  history, and blocks force pushes and deletion. The pull request is merged with GitHub's rebase
  merge, the only method enabled. The rules are checked by GitHub, not by a session's discipline.
  A fast-forward pushed from a checkout is a direct push, which the ruleset refuses, and GitHub
  offers no fast-forward merge method. main's history is never rewritten.
- The strict check makes the tree CI judged the tree main receives, which is what matters. Without
  it, a merge button that rebases makes CI meaningless when history was not already linear, as
  the owner put it.
- GitHub's rebase merge gives the branch's commits new SHAs, even when the branch is already up
  to date with main. Its documentation says it "always updates the committer information and
  creates new commit SHAs". A probe repository, tellurium-monoxide/rebase-merge-probe, archived,
  with a ruleset identical to this repository's, measured it: an up-to-date pull request of two
  commits reached main with the same trees, the same author, a new committer and new SHAs.
  `gh api repos/tellurium-monoxide/rebase-merge-probe/pulls/1/commits` and
  `gh api repos/tellurium-monoxide/rebase-merge-probe/commits` re-read both sides. So neither a commit message nor a document cites the SHA
  of a commit of its own branch: it names that commit by its subject. `commits` refuses such a
  citation, under this repository's manifest.

### Every component builds under one pinned toolchain `##toolchain-is-pinned`

rust-toolchain.toml at the root names one Rust release, with rustfmt and clippy, and rustup
applies it to every cargo invocation in the tree, locally and in CI. fmt and clippy are gates, and
their verdict is a property of the toolchain as much as of the tree: on floating stable, a release
adding a clippy lint fails CI on a tree that passes locally, or the reverse, with no change in the
diff. An upgrade is a one-line commit of that file, and the gates judge it like any other change.

### Every build is tied to the checkout that builds it `##a-build-is-tied-to-its-checkout`

The `[env]` table of the cargo configuration sets `KNOWLEDGE_ARCHITECT_CHECKOUT` to the checkout's
root, with `relative` and `force`. Every library root reads it with `option_env!`. A target of a
package with no library reads it itself. Every build script names it in `rerun-if-env-changed`.
`path@xtask@tests/checkout.rs` asserts both halves: the compiled value is the checkout's root, and
every target `cargo metadata` lists is tied.

The standing argument, each point observed on cargo 1.98.0:

- **Cargo reuses another checkout's build.** A workspace member's artifacts are keyed by its path
  relative to the workspace root, and freshness compares the current checkout's source times with
  the build's. In two copies of a one-crate workspace sharing one `CARGO_TARGET_DIR`, the second
  compiled nothing and ran the first's binary. `CARGO_MANIFEST_DIR` does not help: cargo does not
  track a variable it sets itself. It tracks the value of one a crate reads, so with the tie each
  switch between the copies rebuilds.
- **A build script's run is a unit of its own.** A library that reads the variable recompiles over
  the output of a build script run from the other checkout. The build script of the agent-skills
  crate names each content file by its absolute path, so without its own line a tied library
  includes the other checkout's text.
- **`force` is required.** `cargo run` exports the `[env]` table to the program it starts, and
  without `force` a cargo that program starts keeps the inherited value over its own.

The nearest rival is a separate `CARGO_TARGET_DIR` per checkout. It is an instruction, and nothing
checks that a session followed it: thaum's history records two runs of a stale build from a review
checkout. The tie is the prevention, and it holds whatever the session sets.
`design@core@a-foreign-build-is-refused` stays the detector, for a binary built before the tie or
outside this configuration. The cost is a rebuild at each switch between checkouts that share a
target directory. A single checkout pays nothing.

## 4. How documents point at each other

### A reference is written where the text would have to be revisited if the entry it names changed `##a-reference-claims-a-revisit`

Every citeable thing has one reference form, `<kind>@<anchor>@<id>`, and two mechanisms of the
checker give a reference its value. `cargo klarch show <ref>` prints every reference to an entry,
so what depends on an entry is computed from the tree and never written by hand. An entry that is
deleted or renamed dangles every reference to it, and `cargo klarch check` reports each one, so the
repair list a change produces is the list of texts that depended on what changed. A reference is
therefore a claim of dependence: this text is to be revisited when that entry is reversed, closed,
fired, abandoned or renamed.

**The test for writing one is that claim.** Where a change to the entry would leave the text
unaffected, the reference is decoration and costs a repair for nothing; where it would not, the
reference is owed, or the change reaches nobody. What follows from the test, by the kind of text:

| the text | names | so that |
| --- | --- | --- |
| a design head | the goal its argument derives a constraint from | `show` on the goal lists what abandoning it reopens |
| a design head | a decision of another component it depends on | a reversal reaches it |
| an issue entry | the decision it strains, and the goal it threatens when it does directly | `show` on a decision lists what is outstanding against it before it is reopened, and on a goal what stands between the project and it |
| a guard, a workaround, a stub or a test that pins behaviour an open entry describes | the issue it exists because of, in the comment at the site | closing the entry dangles the comment, so the site is revisited and the workaround removed |
| a tripwire | the decision it guards | a reversal dangles its tripwires |
| a rejected alternative | the decision it lost to | a reversal finds what the old winner displaced |
| a commit message | every entry it opens, closes, reverses or argues from | the commits gate judges it against the tree it was written against |
| a restatement of a directive | its home | a drift between the two is found from either end |

**An entry never lists what references it.** The inbound list is `show`'s to compute, and a
hand-written one is stale at the next reference written elsewhere.

**A reference in prose is checked wherever it stands**, a Rust comment and a fenced block
included; a string literal bound to a name yields none. So a comment in code naming an issue is
as live as a sentence in a document, and closing the issue reaches the code. This serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`.

## 5. This repository's agent configuration

### This repository's own skills and agents take the prefix klarch- `##klarch-prefix`

This repository installs the workflow it ships, and a project's own skills carry the project's
name as a prefix, per `design@agent-skills@skill-name-prefix`. Here that name is
`knowledge-architect`, which is the installer's namespace: a project skill named after it would be
reported as unshipped by `design@core@owned-namespace-check`, and removed by the install. So this
repository's own skills and agents take the prefix `klarch-`, the binary's name. The installed
skills do not mention the case: outside this repository the collision is unlikely, and a rule for
it in the shipped text would be read by every installing project for a risk the owner judges very
small.

### This repository's retrospective findings go to its own registers `##retrospective-findings-stay-here`

A retrospective writes one file for the project and one for the workflow, and the workflow's file
becomes an issue on knowledge-architect's repository where the owner directs it there, per
`design@agent-skills@retrospective-destination`. Here the project is that repository, so the owner
directs both files to this repository's own issue registers: an issue on GitHub would be a second
place for what is open, beside the registers, against
`goal@knowledge-architect@structure-and-workflow-work-together`.
