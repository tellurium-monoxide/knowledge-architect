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
with no address outside the crate. `cargo x changelog` writes the copies, and
`every_published_crate_carries_the_root_changelog` in `path@xtask@src/changelog.rs` fails while one
differs or a crate's `include` stops listing it, so a branch that writes an entry copies it before
it can pass the gates. The walk skips the copies, by three `skip-files` rows of the manifest: the
root file is checked, and they are identical to it. One changelog for every crate, rather than one
per crate, keeps one file to write and gives each crate the whole history; the owner chose it.

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

### Versions follow the owner's scheme, mapped onto Cargo's two positions under 0.x `##versioning-policy`

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

The branch that makes a change writes its entries, in the `Next release` section, and creates the
section above the newest released one when it is absent, as it is after a release renames it. It
writes them because its author knows what changed and in which class at that moment; rebuilt at the
release from commit messages, an entry is lost when nothing asks for it, as one change to a shipped
skill after 0.1.0 was. The release is reviewed once against these tests, rather than every merge,
because a release branch can repair any gap before anything is published. The working section may be
reworded, restructured or pruned at any time, and a change reversed before the release leaves it,
since the section describes the release's net effect. A released section's content never changes;
its structure may.

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
  request is opened. CI does not run on a draft. A commit is pushed only after `commits` has
  passed on it, since a failing message on the remote branch is what a later fetch or review reads.
  The push runs in a command of its own, or behind `&&` on the bare check: after `;`, or after a
  pipe, the shell runs it whatever the check found.
- No operation that can lose content, committed or not. With a clean tree, editing the branch's
  own history is an ordinary move, bounded by verifying that nothing was lost.
- The branch is rebased on main before review and merge, and reviewed before any merge. A repair
  from a review is a new commit, so that no history is edited for it; a repair that would leave an
  earlier commit failing the per-commit rule below is folded into the earliest commit it repairs,
  and the commit recording the review says what was folded. Where every repair was folded, no
  repair commit is left to carry that record, and a commit of its own would change no file, which
  the rebase merge drops; the record goes into the message of the branch's last commit, by a
  reword with a clean tree. The rule exists to avoid history
  edits, not to keep repairs apart, and the owner's reason for the exception is that this
  repository keeps rewriting its checker, so a repair that changes what it judges is no rare case.
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
- **A commit that changes no file does not reach main.** The rebase merge drops it: pull request
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
  crate names each content file by its absolute path, so without its own line a tied library
  includes the other checkout's text.
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

### Plain text is never the repair of a finding, and no instruction offers an unchecked form as the way to clear one `##plain-text-is-no-repair`

A form a writer can use is either one the checker judges, and the workflow recommends it, or one
the checker does not read, and the workflow never directs a pointer into it to clear a finding. So
no finding's repair, and no installed or project instruction, offers an unchecked form as the way
to clear a finding. A repair names a checked form: the right anchor, `path@elsewhere@<path>` for a path this
tree does not hold, `planned@<anchor>@<path>` in a plan document for a path its work will create,
per `design@core@planned-path-form`, an angle-bracket placeholder for an illustration, or a
rewrite of the sentence.
The one exception is text in the checker's syntax that no checked form expresses, written without
backticks beside a reference to the issue entry that records the missing form, per
`design@knowledge-architect@checker-syntax-without-backticks-names-its-gap`. Prose that mentions a
directory without asking the reader to follow it is not a repair and is outside this head.

The argument: an unchecked form that clears a finding clears it for good, so a habit of writing one
empties the check while every run still passes, against
`goal@knowledge-architect@documentation-stays-consistent`. Two narrower stances of the checker
make the same argument: `design@core@the-regime-has-no-opt-out`, where no declaration exempts a
document from a rule, and `design@core@reserved-anchors`, where the escape anchor is refused on a
path this tree holds, since it would otherwise silence the finding on a real path.

A label beside a checked pointer is not such a form. A plan item named outside its plan as #<id>
stands beside a citation of the whole plan, because an item reference is refused there, per
`design@core@plan-item-scope`.

### A sentence about the past whose reference dangles is rewritten to the present or removed, and a quotation of the owner is kept with a reference beside it `##a-past-sentence-is-rewritten`

When an entry is deleted or renamed, a sentence that recorded its past, such as what an earlier
step wrote, is rewritten to state the present, or removed, and its history stays in the commit
messages. Retargeting it to the new name would make it false, and plain text would take it out of
the check, per `design@knowledge-architect@plain-text-is-no-repair`. A verbatim quotation of the
owner that names a renamed entry is left as it is, with a reference to the current entry beside
it, since rewriting it would misstate the owner, against `goal@knowledge-architect@the-owner-decides`.

### Text in the checker's syntax that no checked form expresses is written without backticks only beside a reference to an issue entry that records the missing form `##checker-syntax-without-backticks-names-its-gap`

The checker cannot express every reference a project needs. This head covers text in the checker's
syntax, which would be read as a candidate if it were backticked: a reference,
`<kind>@<anchor>@<id>`, or a path of two or more segments. Where no checked form expresses what such
text points at, it may be written without backticks, and only beside a reference to an issue entry
of the writing project that records the missing form. Any other text that names something, in the
tree or outside it, such as another project's commit, an address on the web or a description in
words, is outside this head. Whether it needs a reference is decided by
`design@knowledge-architect@a-reference-claims-a-revisit`. A project that meets a gap of the checker itself opens that
entry in its own register, since a reference resolves only inside its own project, per
`issue@core@cross-project-references`. A need that a checked form already serves is not a gap.

The entry owes its `Why it matters` and its `What would close it`, so the escape is available and
never free. `cargo klarch show` on the entry lists every site, and closing it, once a checked form
ships and the sites are converted, dangles each one, so the conversion list is computed. Plain text
justified in a commit message alone lost: nothing finds the site again, and nothing revisits it
when the form ships. A generic checked opt-out marker lost too: one marker fits every finding, so
it becomes the cheap silence `design@core@reserved-anchors` refuses, while a gap concrete enough to
name is closed by shipping its own form.

The scope is the checker's syntax, so that a writer can tell from each span alone whether the head
applies, per `goal@knowledge-architect@agents-get-a-complete-workflow`: every sentence names
something, and a scope of "any pointer" asked for an issue entry beside every mention of a thing
outside the project. The evasion the head exists to stop is a reference or a path with its
backticks removed, which this scope covers. A scope by target, any text naming something the tree
holds, lost: when such text needs a reference is already decided by the rule on references, and
whether a phrase names something cannot be decided span by span.

A commit named by its subject, as `design@knowledge-architect@git-flow` directs for a commit of the
branch, is outside this head. It names history as git names it. A reference resolves against the
entities of a tree, per `design@core@one-entity-table`, and a commit is none of them; the one
citation of a commit the checker judges is a branch commit's SHA, which
`design@core@branch-shas-are-refused` refuses.

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
