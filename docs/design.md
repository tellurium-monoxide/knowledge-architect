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

### No Component's directory is named after the project, and a published crate's directory may differ from its package's name `##no-component-directory-named-after-the-project`

A Component is named by the basename of its directory, and the project root by the project's name,
so a directory named `knowledge-architect` would collide with the root: a run over a scratch project
laid out that way stopped in phase 1, reporting that the name "names 2 anchors". So a published
crate may sit in a directory named by what it does rather than by its package, crates/core for the
package knowledge-architect, and a future split of the core follows the same rule, as crates/core
and crates/cli. The cost: a reference says `core` where cargo says `-p knowledge-architect`.

### The installed text lives in its own crate, which the checker depends on `##two-crates`

The skills, subagent definitions and primer that the checker installs are a separate package,
`knowledge-architect-agent-skills`, in crates/agent-skills. The checker depends on it by path and by
exact version. A package ships only the files under its own directory, so text embedded with
`include_str!` must sit inside a package; giving the text a crate of its own keeps it a component
of its own, with its own design home, rather than a part of the core.

A user names only `knowledge-architect`, and cargo fetches the other. The two are published
together, at one version.

### Each published crate ships a whitelist of files `##package-include-whitelist`

Each crate under crates/ declares `include` in its Cargo.toml, listing its sources, its Cargo.toml,
its README, its license files and its CHANGELOG.md, and nothing else. Cargo adds the file `readme`
names, its CRATES-IO.md, whatever `include` says, per
`design@knowledge-architect@crates-io-page-file`; `cargo package --list -p <crate>` re-takes it, and
the release procedure runs it. A new directory is shipped only once it is listed, so the test
fixtures and the design documents never reach a package, and their growth cannot push a package past
the crates.io size limit. The cost is accepted: `cargo test` cannot run from a downloaded package.
The license files sit at the root and as copies in each directory under crates/, because an
`include` path cannot reach outside the package directory, and the changelog ships the same way, per
`design@knowledge-architect@the-changelog-ships-in-every-crate`. tools/xtask is never published, so
it carries none of these copies. This keeps what a project receives to what it uses, per
`goal@knowledge-architect@any-project-can-adopt-it`.

### Each published crate ships the root changelog, written by `cargo x changelog` `##the-changelog-ships-in-every-crate`

A project moving its pin reads the changelog of every version crossed. Each crate under crates/
carries a copy of the root CHANGELOG.md, listed in its `include`, so the changelog of a version is
in the source cargo fetches for that version, and a project reads it there, at the version it pins,
with no address outside the crate. `cargo x changelog` writes the copies, one command for every
copy, as the owner proposed: "I'd also provide a `cargo x` subcommand that updates all changelog
copies at once". The test `every_published_crate_carries_the_root_changelog` in
`path@xtask@src/changelog.rs` fails while one differs or a crate's `include` stops listing it, so a
branch that writes an entry copies it before it can pass the gates. The walk skips the copies, by three `skip-files` rows of the manifest: the
root file is checked, and they are identical to it. One changelog for every crate, rather than one
per crate, keeps one file to write and gives each crate the whole history.

**The copies are committed, because cargo packages nothing else.** Measured on cargo 1.98.0: a copy
that `include` lists and git ignores makes `cargo package --list -p <crate>` exit 101, "files in the
working directory contain changes that were not yet committed into git"; with the copy absent, the
crate is packaged without it, and with no error. Re-taken by listing a crate after writing a
git-ignored copy into it, then after deleting the copy.

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
therefore never runs a checker whose installed skills describe another version's commands, since a
binary of another version than its manifest pins refuses to run, per
`design@core@installed-binary-version-check`, which
is what lets a project pin one version and move when it chooses, per
`goal@knowledge-architect@any-project-can-adopt-it`. The gates library is in the same lockstep: the
setup skill that recommends it is the same version's, so a project pins one version for all of
it. The cost accepted: a fix to the gates alone is released as a version of every crate.

### A version's bump is the class of what it changes in a consuming project, mapped onto Cargo's two positions under 0.x `##versioning-policy`

The table classes what the published crates do in a consuming project: the checks the checker runs
over its tree, its commands, its manifest, the library APIs and the installed skills. A check, a
command or an addition in the table is one of those. A change internal to this repository, such as
a test, a gate or a subcommand of tools/xtask, is outside the table, and so is a file a crate ships
for reading alone, such as its changelog: each is at most a patch.

| bump | when |
| --- | --- |
| patch | skill changes; code changes that change no check and no command |
| minor | a check becomes stricter, or a check is added, even when a project must change its content to pass it; additions that change nothing existing; command and manifest changes that only add |
| major | a command change that breaks existing usage; a change to the documents and homes a project must carry; a manifest that was valid and stops being accepted; a breaking change to the library API |

The table is the owner's scheme. The bullets below refine it, and the first is the owner's
argument, given in the ruling that a stricter check is minor.

- **A stricter check is minor even when it requires a content change.** Every stricter check may
  require a content change in some project, so a major row that took every content change would
  leave the minor row with no check in it, and two rows that overlap give a changelog entry no
  single class. The major row keeps the changes a project meets as a refusal of its shape: a
  required document or home, and a manifest that stops being accepted.

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
- Which change gets a CHANGELOG.md entry, and with which class, is
  `design@knowledge-architect@changelog-entries`.

A version stricter than a patch never reaches a project through a plain `cargo update` while at
0.x, and a release that changes only skill text publishes all three crates with identical code. A
project can therefore tell from the version alone whether moving to it may require work, per
`goal@knowledge-architect@any-project-can-adopt-it`.

### The changelog records what a consumer must plan for: one entry per change that passes a test, under released sections whose content never changes `##changelog-entries`

An entry of CHANGELOG.md is written for a consumer planning a move to the version. It exists when
the change passes one of three tests, and sits under the subsection of that test:

- **Migration**: one entry per thing a consumer must change in its own files, saying what.
  Running the install of the agent skills again is never an entry; a change the consumer must make
  to its own files because of the new skills is one. An entry that adds a required document or
  home says it holds for every project the consumer's tests build, mock projects included: a
  consumer whose extension tests the checker over its own fixture projects otherwise meets the
  change as failing tests.
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

CHANGELOG.md has one section per version, and one working section above them, `Next release`,
which the release renames to its version. The working section may be reworded, restructured or
pruned at any time, and a change reversed before the release leaves it, since the section describes
the release's net effect. A released section's content never changes; its structure may.

**A skill's or an agent's name in a released section is structure.** Writing a bare name there as
its reference, `skill@<name>` or `agent@<name>`, changes no statement of the section. The owner:
"older changelog section allow structural changes, and this passes as a structural change in my
view". So does writing it back as the bare name once the skill or the agent is deleted, since the
reference then names nothing and the section's content cannot change. Why the bare name is no
evasion is `design@agent-skills@plain-text-is-no-repair`, and the lint that reports a bare name
`design@core@bare-skill-name-reported`.

### The branch that makes a change writes its changelog entries `##the-branch-writes-its-changelog-entries`

The branch that makes a change writes its entries, in the `Next release` section, and creates the
section above the newest released one when it is absent, as it is after a release renames it. It
writes them because its author knows what changed and in which class at that moment; rebuilt at the
release from commit messages, an entry is lost when nothing asks for it, as one change to a shipped
skill after 0.1.0 was.

### The changelog entries are reviewed once, at the release, rather than at every merge `##changelog-reviewed-at-the-release`

The entries are reviewed once, at the release, against the tests of
`design@knowledge-architect@changelog-entries`, rather than at every merge, because a release branch
can repair any gap before anything is published.

### The project stays at 0.x until the owner's word, given once its first design discussion's open issues are settled `##stays-at-zero-x`

Breaking changes are allowed, and expected, while the shape of the tool and of its workflow
converges. The project leaves 0.x only on the owner's word, and not before the open issues recorded
from the discussion that designed it are implemented, or at least argued thoroughly: both are
required. The condition and the list are the owner's: "take (A) for 1.0", on the principle the
owner stated in the discussion that designed the project, that it stays at 0.x until the open
issues of that discussion are implemented, or at least argued thoroughly. Whether the tool and its workflow have converged is a weighing, and the weighing is the
owner's, per `goal@knowledge-architect@the-owner-decides`. The open issues are
`issue@core@tooling-for-project-skills`, `issue@core@configuration-for-several-agent-providers`,
`issue@core@a-component-states-at-least-one-goal`, `issue@core@cross-project-references`,
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`,
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`,
`issue@agent-skills@patching-an-installed-skill` and
`issue@agent-skills@a-skill-for-creating-a-component`.

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
published from main's head, on the owner's word given at that moment, per
`goal@knowledge-architect@the-owner-decides`. The order follows what can
be undone. A mistake on main is repaired by a later commit, while a version published on crates.io
can be yanked and never deleted, so the irreversible step comes last, after review and CI have
judged exactly what it publishes, and the tag names a commit on main. The cost: main states the
new version for the minutes between the merge and the publish, and a failed publish leaves that
statement false until a repair lands. The procedure is
`skill@klarch-release`.

## 3. How work reaches main

### All work goes through a branch, a review and a pull request merged up to date with main `##git-flow`

- All work happens on a branch. Once it holds a first commit, it is pushed and a draft pull
  request is opened. CI does not run on a draft.
- The branch is rebased on main before review and merge, and reviewed before any merge. A review
  repair lands as `design@agent-skills@review-repair-appended-or-folded` decides: appended, and
  folded where appending leaves an earlier commit failing the per-commit rule below. Where every
  repair was folded, no repair commit is left to carry the record of the review, and a commit of
  its own would change no file, which the rebase merge drops, per
  `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`; the record goes into
  the message of the branch's last commit, by a reword with a clean tree. The owner's reason the
  fold applies here is that this repository keeps rewriting its checker, so a repair that changes
  what it judges is no rare case.
- **Every commit of a branch passes the check under the branch tip's checker**, as
  `design@core@a-commit-message-is-a-document` decides. So work whose intermediate trees cannot
  pass lands as one commit, squashed before review.
- main accepts no direct push. A GitHub ruleset on main, with no bypass, the owner included,
  requires a pull request, the status check of CI on a branch up to date with main, and a linear
  history, and blocks force pushes and deletion. The pull request is merged with GitHub's rebase
  merge, the only method enabled. The rules are checked by GitHub, not by a session's discipline,
  with one gap: a job skipped while the pull request was a draft counts as passing, so right after
  it is marked ready the skipped run can be the only one on its head. The merge waits for a run on
  the head whose conclusion is success.
  A fast-forward pushed from a checkout is a direct push, which the ruleset refuses, and GitHub
  offers no fast-forward merge method. main's history is never rewritten.
- The strict check makes the tree CI judged the tree main receives, which is what matters. The
  owner, on the ruleset: "This is much cleaner than a "rebase and merge" button, which makes CI
  meaningless if history was not already linear."

### No operation can lose content, and the branch's own history is edited only with a clean tree `##no-operation-loses-content`

No operation can lose content, committed or not. Content is lost in two forms: an operation that
removes uncommitted content from the working tree, which no commit holds, and a history edit that
drops a commit no other reference holds. With a clean tree, editing the branch's own history is an
ordinary move, bounded by verifying that nothing was lost.

### A commit is pushed only after the commits check passed on it `##push-after-the-commits-check`

A commit is pushed only after `commits` has passed on it, since a failing message on the remote
branch is what a later fetch or review reads. The push runs in a command of its own, or behind
`&&` on the bare check: after `;`, or after a pipe, the shell runs it whatever the check found.

### No text cites the SHA of a commit of its own branch `##no-branch-sha-is-cited`

GitHub's rebase merge gives the branch's commits new SHAs, even when the branch is already up
to date with main. Its documentation says it "always updates the committer information and
creates new commit SHAs". A probe repository, tellurium-monoxide/rebase-merge-probe, archived,
with a ruleset identical to this repository's, measured it: an up-to-date pull request of two
commits reached main with the same trees, the same author, a new committer and new SHAs.
`gh api repos/tellurium-monoxide/rebase-merge-probe/pulls/1/commits` and
`gh api repos/tellurium-monoxide/rebase-merge-probe/commits` re-read both sides. So neither a commit message nor a document cites the SHA
of a commit of its own branch: it names that commit by its subject. `commits` refuses such a
citation, under this repository's manifest.

### A record carried by a commit message rides on a commit that changes a file `##a-record-rides-on-a-commit-that-changes-a-file`

**A commit that changes no file does not reach main.** GitHub's rebase merge drops it: pull request
#42 of this repository held eight commits, two of them made with `--allow-empty` to record the
owner's rulings, and main received the other six. `gh api
repos/tellurium-monoxide/knowledge-architect/pulls/42/commits --jq length` against the commits
main received re-takes it. So a record carried by a message alone rides on a commit that changes
a file.

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
`path@xtask@tests/checkout.rs` asserts that the compiled value is the checkout's root, that the
entry is relative and forced, and that every target `cargo metadata` lists is tied.

The standing argument, each point observed on cargo 1.98.0. Each is re-taken by copying a one-crate
workspace twice, building both copies into one `CARGO_TARGET_DIR`, and reading what the second
build compiles and what its binary prints:

- **Cargo reuses another checkout's build.** In the two copies, the second compiled nothing and ran
  the first's binary. `CARGO_MANIFEST_DIR` does not help: cargo does not track a variable it sets
  itself. It tracks the value of one a crate reads, so with the tie each switch between the copies
  rebuilds. The cause, from thaum's inspection of its target directory and not re-taken here: a
  workspace member's fingerprint records paths relative to its package, its path hash leaves out
  where the checkout sits, and freshness compares the current checkout's source times with the
  build's.
- **A build script's run is a unit of its own.** A library that reads the variable recompiles over
  the output of a build script run from the other checkout. The build script of the agent-skills
  crate renders each content file from the checkout it runs in, so without its own line a tied
  library includes the other checkout's text.
- **The value follows cargo's working directory, not the manifest it builds.** Cargo finds its
  configuration from where it is started, as it finds the aliases. Started in one checkout with
  `--manifest-path` naming another, it builds the other with the first one's value; started
  outside every checkout, it sets none. So the tie holds for a cargo started inside the checkout it
  builds, which is how the aliases run.
- **`force` is required.** `cargo run` exports the `[env]` table to the program it starts, and
  without `force` a cargo that program starts keeps the inherited value over its own.

The nearest rival is a separate `CARGO_TARGET_DIR` per checkout. It is an instruction, and nothing
checks that a session followed it: thaum's rejected alternatives list three runs of another
checkout's build, from two review checkouts and one probe, and thaum's own issue on the second
says whether that reviewer set the directory is not established. The tie is the prevention, and it
holds whatever target directory the session sets. The separate directory stays an instruction of the review skill,
for the rebuilds it saves and for the build lock two checkouts would otherwise wait on. `design@core@a-foreign-build-is-refused` stays the detector. The cost is
a rebuild at each switch between checkouts that share a target directory. A single checkout pays
nothing.

## 4. This repository's agent configuration

### This repository's own skills and agents take the prefix klarch- `##klarch-prefix`

This repository installs the workflow it ships, and a project's own skills carry the project's
name as a prefix, per `design@agent-skills@skill-name-prefix`. Here that name is
`knowledge-architect`, which is the installer's namespace: a project skill named after it would be
reported as unshipped by `design@core@owned-namespace-check`, and removed by the install. So this
repository's own skills and agents take the prefix `klarch-`, the binary's name.

### This repository's retrospective findings stay in this repository, never on GitHub `##retrospective-findings-stay-here`

A retrospective writes one file for the project and one for the workflow, and the workflow's file
becomes an issue on knowledge-architect's repository where the owner directs it there, per
`design@agent-skills@retrospective-destination`. Here the project is that repository, so the owner
directs the findings of both files to this repository: each is handled, opened as an entry in its
own issue registers, or closed with no change and the reason. An issue on GitHub would be a second
place for what is open, beside the registers, against
`goal@knowledge-architect@structure-and-workflow-work-together`.

### Each finding of a received retrospective file gets an outcome the owner rules, in a committed analysis that leaves when its last outcome is carried out `##committed-findings-analysis`

A retrospective's workflow file that this repository receives, from its own sessions or from a
project that uses the workflow, and the project file of this repository's own retrospective, are
each analysed finding by finding under `skill@klarch-retrospective-intake`, and the analysis is a
file of `path@knowledge-architect@docs/retrospective-reports/`, named by the received file's stem.
Each finding gets one of three outcomes, ruled by the owner: handled now, opened as an issue, or no
change with the reason, per `goal@knowledge-architect@the-owner-decides`. The analysis is committed
with the owner's ruling on each finding, the issues it rules are opened next, the findings handled
now are handled, and the commit that carries out its last outcome, or a later commit of the same
branch, deletes it. A finding handled now that grows into a design discussion has its outcome
rewritten, on the owner's word, to name the plan document or the issue that carries it, so the file
still leaves.

The file is the record of the rulings, so every finding of a received file reaches an outcome that
history keeps, per `goal@knowledge-architect@the-workflow-improves-through-real-use`. Its nearest
rival, the ledger of outcomes in the message of the commit that handles the first finding, fails
on a file whose findings all end in no change: that commit changes no file, and the rebase merge
drops a commit that changes no file, per
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`.

### An analysis of a received file names nothing of another project beyond what that file holds, and no path or design head of it `##analysis-names-nothing-of-another-project`

The repository is public and the workflow file is the one a retrospective writes to be
publishable, so the analysis names nothing of another project beyond what its workflow file holds.
It names no path or design head of that project either, which in the owner's words "is useless
information here anyway": where a finding needs one, the analysis says it in words.
