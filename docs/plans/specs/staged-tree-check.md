# `check --staged` judges the tree a commit would record, `index --staged` writes it, and `check --fix` refuses a partial commit's mismatch

## Status and audience

This spec is the plan of the work that lets the checker judge the tree held in git's index, the
tree `git commit` would record now, as well as the working tree. The work does five things:

- `cargo klarch check --staged` judges the staged tree, with the rules `check` applies to a
  checkout;
- `cargo klarch index --staged` writes each generated file the staged tree needs into git's index,
  and leaves the working tree untouched;
- `cargo klarch check --fix` refuses to run when the staged tree and the working tree need
  different generated files, and refuses to be combined with `--staged`;
- the extension API hands an extension a snapshot of a commit or of the index, as
  `extension::Tree::Snapshot`;
- the installed skills and this repository's `CLAUDE.md` tell a session when to use the staged
  forms.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **No gate fixes when this spec lands against its work.** The work adds options to `check` and
  `index` and renames an extension type. It does not change what `cargo klarch commits` judges in a
  commit, so `design@agent-skills@plan-lands-before-gate-change` does not apply. The spec and its
  work land on one branch, `staged-tree-check`, in one pull request.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/2d705814-4154-4871-9a54-a3712e881823.jsonl`.
  The discussion begins at the owner's message that opens "I would like for the checker to have
  the ability to run the checks on a tree restricted to either of", and ends at the owner's message
  "Table approved, keep all tripwires and acceptance criteria.proceed to the spec". It holds 3
  owner messages, called rounds 1 to 3 below, and 2 agent replies, one after each of rounds 1 and
  2.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **the working tree**: what `check` judges today. Git's listing
  `git ls-files --cached --others --exclude-standard`, read from disk, per
  `design@core@git-supplies-the-walk`. It holds untracked files that no ignore rule covers.
- **the staged tree**: the stage-0 entries of git's index, as `git ls-files -s -z` lists them, with
  each blob's bytes read from git's object store. It is the tree `git commit` would record, absent
  a pathspec or `-a`.
- **a snapshot**: a tree read from git objects alone, a commit's tree or the staged tree. Today
  `commits` builds one per commit, in `read_tree` and `commit_tree` of
  `path@core@src/cli/history.rs`.
- **a generated file**: a file of the list `generated_list` in `path@core@src/cli/mod.rs`
  returns: one `index.md` per file-register instance, and each file an extension generates.
- **the installed files**: the agent files of the installer's namespace, per
  `design@core@owned-namespace-check`.
- **the shipped text**: the files under `path@agent-skills@content/`, installed into a project by
  `cargo klarch install-agent-skills`. `{{command}}` is its placeholder for the project's command;
  in this repository it is `cargo klarch`.
- **thaum**: the one known consumer of the extension API, a separate repository whose extension
  is `path@elsewhere@tools/rules-corpus/citations/src/rules_extension.rs`, read at its commit
  `6a2a1d79`.
- **T1, T2, AC1, AC2, AC3**: the labels the premortem put to the owner, kept in the items below.

## What the work is

What exists today at each site the work touches:

| site | today |
| --- | --- |
| `check`, `check` in `path@core@src/cli/mod.rs` | builds the model from disk with `Model::build`, gathers `Inputs` with `Gathered::over` of `path@core@src/cli/gathered.rs`, runs `check::foundation`, prepares each extension with `Tree::Checkout(manifest.root())` and `Purpose::Check`, then `check::run_with` |
| `CheckArgs`, same file | one flag, `--fix` |
| `index`, `index` in the same file | builds the model from disk, gates with `complete_working_tree`, writes each generated file whose bytes differ into the working tree, and takes no flags, per `design@core@generated-files-are-pure` |
| `check --fix`, `fix_then_check` in the same file | repairs the installed files, gates the model on phases 1 to 3, writes the generated files into the working tree, then runs `check`, per `design@core@fix-before-the-checks`. It never reads git's index |
| the summary block, `counts` in the same file | prints `walk: <n> file(s)` on a stop and on a full run; nothing says which tree was walked |
| a commit's snapshot, `read_tree` and `commit_tree` in `path@core@src/cli/history.rs` | lists a commit with `git::tree_entries` (`ls-tree -r -z`), reads blobs with `git::blob_bytes` (`cat-file --batch -z`, each request `<sha>:./<path>` from `git::tree_object`), and assembles a model and an `Assembly` whose `inputs()` sets `installed` and `shipped` empty (history.rs:77) and `tracked_and_ignored` empty |
| `Gathered::over`, `path@core@src/cli/gathered.rs` | reads the generated files and every `register.toml` from disk, surveys the disk, asks git's ignore batch and `git::tracked_and_ignored`, and reads the shipped set |
| `survey::from_listing`, `path@core@src/survey.rs` | builds a survey from a listing and a read closure, the installed files' bytes included |
| `extension::Tree` and `extension::Purpose`, `path@core@src/extension.rs` | `Tree` has two variants, `Checkout(&Path)` and `Commit(&CommitTree)`; `Purpose` has `Check`, `Commit` and `Index`; neither is non-exhaustive. `CommitTree` offers `holds`, `read` and `object_id` |
| thaum's extension | matches `Tree::Checkout` and `Tree::Commit` at lines 204-205, names `CommitTree` at line 313, and sets `Purpose::Commit` itself at line 347, so its `changes` and `corpus` checks are listed as not run over a snapshot (lines 462-465) |
| the shipped issue-tracking skill, `path@agent-skills@content/skills/issue-tracking/SKILL.md`, lines 149-156 | "regenerate it and check in one command: `{{command}} check --fix`"; "`{{command}} index` writes the generated files alone, with no check." Nothing on a partial commit |
| the shipped planning skill, `path@agent-skills@content/skills/planning/SKILL.md`, lines 128-129 | "`{{command}} index` writes every `index.md`, and `{{command}} check --fix` writes them and then checks." |
| the shipped setup skill, `path@agent-skills@content/skills/setup/SKILL.md`, section `skill@knowledge-architect-setup@setup-gates` | recommends one gates command; no shipped text tells a session to check the tree before each commit |
| this repository's `path@knowledge-architect@CLAUDE.md`, section `instructions@verify-mechanically` | "**Run `cargo klarch check` before each commit, which judges the tree, and `cargo klarch commits origin/main..HEAD` after it, which judges the message.**" |
| `path@core@README.md` | the command table, lines 35 and 41, lists `check --fix` and `index`; the `--fix` paragraph starts at line 117 |

Outside the work:

- **Judging a given commit's tree with `check`.** Ruled out, #commit-tree-in-check below:
  `cargo klarch commits <rev>~1..<rev>` judges it.
- **A `--staged` form of `commits`, `show`, `issues`, `tripwires` and `model`.** Nobody proposed one.
  Each reads the working tree as today.
- **Moving the installed-file findings to the last phase.** Owned by
  `issue@core@installed-file-findings-belong-in-phase-four`. A stale install in the staged tree
  stops `index --staged` in phase 2, as it stops `index` today.
- **Which part of a command's output is a contract.** Owned by
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`. The new `tree:` line and
  the new refusal are classed in the changelog under the policy as it stands.

**The standing entries the search returned at the discussion's grounding**, each with its outcome:

| entry | outcome |
| --- | --- |
| `tripwire@core@fix-makes-a-choice` | does not fire: #fix-with-staged keeps `--fix` off git's index, and the refusal writes nothing |
| `tripwire@core@phases-gate-the-report-two` | does not fire: `index --staged` runs phases 1 to 3 over the staged model before it writes, per AC3; its re-entry ("any change to the arguments of `check`, `index`") is met, and the standing-state review reads it |
| `tripwire@core@walked-count-differs-between-machines` | does not fire; its re-entry is met. The new `tree:` line says which tree the `walk:` count is of, so a count of the staged tree is never compared with CI's count of a checkout |
| `tripwire@core@inputs-builder-needed` | does not fire: no extension test needs an `Inputs` the snapshot assembly produces |
| `issue@core@installed-file-findings-belong-in-phase-four` | outside the work, above |
| `issue@knowledge-architect@command-output-is-not-declared-a-contract` | outside the work, above |
| `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | needs nothing: the setup skill's edit adds no TOML, alias or CI block |
| `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to` | read again at step 2: it asks which tree `Inputs.present` describes under `commits`; under `--staged` it describes the staged tree, and the doc comment written at step 2 says so |

## What is already decided

The design rests on these decisions, and does not argue them again:

- `design@core@git-supplies-the-walk`: the walk is git's listing, every git invocation is in
  `path@core@src/git.rs`, and git 2.36 is the floor.
- `design@core@a-commit-message-is-a-document`: a snapshot is assembled from git objects in
  memory, and the ignore rules are read from disk.
- `design@core@phases-gate-the-report`: no writer writes over an incomplete model.
- `design@core@safe-fix-definition` and `design@core@fix-scope`: what `--fix` may write. The work
  does not widen them.
- `design@core@ne-minimal`: "A third `extension::Tree` must make every extension say how it reads
  it. A wildcard would read the working tree while the run judges another tree". The rename keeps
  that property: a snapshot is read from git objects only, never from the working tree, and the
  rename is a compile error for every extension.
- `design@core@exit-code-ladder`: a refusal before anything runs is exit 2.

The work rewrites four decisions. Every text that `cargo klarch show` lists as referencing each,
on the main branch before this spec:

| decision | texts referencing it | judged or updated at |
| --- | --- | --- |
| `design@core@generated-files-are-pure`: "`cargo klarch index` takes no flags" | `path@core@README.md` lines 172 and 218; `path@core@docs/design.md` lines 866 and 941; `path@core@docs/rejected-alternatives.md` lines 205, 215 and 292; `path@core@src/cli/mod.rs` lines 603, 653 and 719 | step 3 for the README and the source; the harvest for the design home and the rejected alternatives |
| `design@core@check-fix-flag`: gains the two refusals | `path@core@README.md` line 118; `path@core@src/cli/mod.rs` line 429 | step 4 |
| `design@core@fix-before-the-checks`: the order gains the refusal first | `path@core@README.md` line 127; `path@core@docs/design.md` lines 71 and 114; `issue@core@installed-file-findings-belong-in-phase-four`; `path@core@src/cli/mod.rs` line 430 | step 4 for the README and the source; the harvest for the design home and the issue |
| `design@core@git-supplies-the-walk`: the walk has a second source | `path@core@CLAUDE.md` line 23; `path@core@README.md` line 31; `path@core@docs/design.md` line 1515; `path@core@docs/rejected-alternatives.md` line 11; `tripwire@core@walked-count-differs-between-machines`; `path@core@src/check/mod.rs` line 39; `path@core@src/check/tree.rs` line 547; `path@core@src/cli/mod.rs` line 1127; `path@core@src/git.rs` lines 6 and 184; `path@core@src/manifest.rs` line 593; `path@core@src/walk.rs` lines 20 and 61 | step 1 for the source; step 2 for the README and CLAUDE.md; the harvest for the design home, the rejected alternatives and the tripwire |

`design@core@api-facade` names `CommitTree` in its text, at `path@core@docs/design.md` line 278.
The rename edits that word at the harvest, and the head's decision is unchanged.

## Criteria

### Once the staged tree passes, the commit made from it passes `check` on a clean checkout and the tree half of `commits` `##c-predicts-the-commit`

Weighed. Stated by the agent in round 1 from the owner's motive in round 1. Met by
`thread@staged-tree-check@staged-tree-source` and `thread@staged-tree-check@staged-check-semantics`,
except two gaps, watched by T1: the ignore rules are read from disk, and an extension's check of
filesystem state is not run over a snapshot.

### One tree is never judged by two commands with different rules `##c-one-verdict-per-tree`

Binding as a presumption. Derived from `design@core@generated-files-are-pure`: "A second command
answering the same question in its own format is what drifts". Met: `thread@staged-tree-check@commit-tree-in-check`
is ruled out.

### `--fix` writes only bytes fixed by the tree and the pinned version, inside the installer's namespace or the generated list `##c-fix-is-safe`

Binding as a presumption. Derived from `design@core@safe-fix-definition`. Met by
`thread@staged-tree-check@fix-with-staged`, which keeps `--fix` off git's index.

### No writer writes over an incomplete model `##c-no-writer-over-incomplete-model`

Binding as a presumption. Derived from `design@core@phases-gate-the-report`. Met by
`thread@staged-tree-check@index-staged-write`, which gates on phases 1 to 3 of the staged tree, and
judged by `acceptance@staged-tree-check@index-staged-touches-only-generated-entries`.

### Plain `check`, `index` and `check --fix` keep today's behaviour `##c-default-unchanged`

Weighed. Stated by the agent in round 1. Met by `thread@staged-tree-check@default-source-worktree`,
apart from the refusal of `thread@staged-tree-check@fix-refusal-mixed-state`, which the owner
approved in round 2.

## Threads

### `check --staged` judges the stage-0 entries of git's index, read through the snapshot reader `commits` uses `##staged-tree-source`

Proposed by the owner, round 1, as item 2 of the request; shaped by the agent in its reply to round
1. Approved. Arguments: `argument@staged-tree-check@a1`, `argument@staged-tree-check@a5`,
`argument@staged-tree-check@a6`, `argument@staged-tree-check@a7`, `argument@staged-tree-check@a8`,
`argument@staged-tree-check@a9`. Shape: Decided design, "`check --staged`". Harvest: the new head
`staged-tree-source`. The owner's words, round 2: "staged-tree-source approved."

### `--staged` applies `check`'s rules, as on a clean checkout of the staged tree `##staged-check-semantics`

Proposed by the agent, round 1. Approved. Arguments: `argument@staged-tree-check@a5`,
`argument@staged-tree-check@a10`, `argument@staged-tree-check@a11`. Shape: Decided design,
"`check --staged`". Harvest: the head `staged-tree-source`. The owner's words, round 2:
"staged-check-semantics approved."

### An extension is handed a snapshot, `Tree::Snapshot`, of a commit or of the index `##extension-tree-for-index`

Proposed by the agent, round 1; refined in its reply to round 2 after reading thaum. Approved.
Arguments: `argument@staged-tree-check@a12`, `argument@staged-tree-check@a13`,
`argument@staged-tree-check@a14`, `argument@staged-tree-check@a15`,
`argument@staged-tree-check@a16`. Shape: Decided design, "Extensions over a snapshot". Harvest:
the new head `an-extension-reads-a-snapshot`. The owner's words: round 2, "extension-tree-for-index
your proposal looks sound. You can check thaum at ../thaum, to assess migration cost."; round 3,
against the checkpoint table where the thread stood presumed-settled, "Table approved, keep all
tripwires and acceptance criteria.proceed to the spec".

### `check` judges a given commit's tree `##commit-tree-in-check`

Proposed by the owner, round 1, as item 1 of the request: "the last commit (maybe even any given
one)". Argued against by the agent, round 1. Ruled out. Arguments: `argument@staged-tree-check@a1`,
`argument@staged-tree-check@a5`, `argument@staged-tree-check@a17`. Shape: none. Harvest: a
rejected alternative. The owner's words, round 2: "commit-tree-in-check agreed, drop the idea."

### The default stays the working tree as git lists it `##default-source-worktree`

Proposed by the owner, round 1: "The last one being the default (to match current behavior)."
Approved, with the agent's statement that the default includes untracked files. Arguments:
`argument@staged-tree-check@a1`, `argument@staged-tree-check@a4`. Shape: Decided design,
"`check --staged`". Harvest: the head `staged-tree-source`, and
`design@core@git-supplies-the-walk` rewritten. The owner's words, round 2:
"default-source-worktree agreed".

### `index --staged` writes the staged tree's generated files into git's index, and leaves the working tree `##index-staged-write`

Proposed by the owner, round 1, "(debatable IMO)"; shapes A and B by the agent, round 1, B the
agent's. Approved, shape B. Arguments: `argument@staged-tree-check@a2`,
`argument@staged-tree-check@a18`, `argument@staged-tree-check@a19`,
`argument@staged-tree-check@a20`, `argument@staged-tree-check@a27`. Shape: Decided design,
"`index --staged`". Harvest: the new head `index-staged-write`. The owner's words, round 2:
"index-staged-write: B looks fine to me. This is not definitive, but I would not recommend to use
this with --fix though. The reason  is that I'd like to provide more types of quick fixes later,
and this might not be compatible." The words "not definitive" are recorded with the ruling: the
head written at the harvest carries the decision as approved, and a later reversal takes the
ordinary procedure.

### `check --fix` refuses when the index differs from HEAD and a generated file differs between the two trees `##fix-refusal-mixed-state`

Proposed by the owner, round 1, with a wider trigger; the narrow condition by the agent, round 1.
Approved, narrow condition. Arguments: `argument@staged-tree-check@a3`,
`argument@staged-tree-check@a21`, `argument@staged-tree-check@a22`,
`argument@staged-tree-check@a23`, `argument@staged-tree-check@a24`. Shape: Decided design,
"`check --fix` refusals", refusal 2. Harvest: the new head `fix-refusal-mixed-state`. The owner's
words, round 2: "fix-refusal-mixed-state: agreed on your narrower condition".

### `--fix` with `--staged` is refused at parse time `##fix-with-staged`

Proposed by the agent in its reply to round 2, from the owner's remark under #index-staged-write;
it absorbs the agent's open sub-question of round 1 on `check --fix --staged`. Approved. Arguments:
`argument@staged-tree-check@a25`, `argument@staged-tree-check@a26`. Shape: Decided design,
"`check --fix` refusals", refusal 1. Harvest: the head `fix-refusal-mixed-state`. The owner's
words, round 3, against the checkpoint table where the thread stood in discussion: "Table
approved, keep all tripwires and acceptance criteria.proceed to the spec".

### The shipped skills name the staged forms, and the setup skill recommends `check --staged` before each commit `##workflow-delivery-staged`

Asked for by the owner, round 1: "identify shipped workflow changes needed to deliver this new
feature"; the table by the agent, round 1. Approved, the whole table. Arguments:
`argument@staged-tree-check@a28`, `argument@staged-tree-check@a29`. Shape: Decided design,
"Delivery in the shipped text and in this repository". Harvest: judged for a head in
`path@agent-skills@docs/design.md`. The owner's words, round 2: "workflow-delivery-staged: I agree
with the whole table".

## Arguments

### The request: three trees, the working tree by default `##a1`

Round 1, the owner. Bears on #staged-tree-source, #commit-tree-in-check, #default-source-worktree,
#workflow-delivery-staged. "run the checks on a tree restricted to either of: 1. the last commit
(maybe even any given one), 2. the current staged tree, 3. the current tree (staged changes + non
staged changes). The last one being the default (to match current behavior)." and "identify
shipped workflow changes needed to deliver this new feature".

### `index` could take the same option `##a2`

Round 1, the owner. Bears on #index-staged-write. "The index subcommand could provide the same
feature too, so that indexes can be updated from staged tree (debatable IMO)."

### A partial commit makes `--fix` unsafe `##a3`

Round 1, the owner. Bears on #fix-refusal-mixed-state. "check --fix should probably refuse to run
if the tree holds both staged and unstaged changes. Because the user intent in this case is
probably to commit partially, and then --fix cannot apply its changes safely (it could conflict
with unstaged changes). In this case, the recommended action would be to run check without --fix
and restricted to staged area, and fix findings manually (fixing indexes might need index to
accept running on staged changes)."

### Today's default is not "staged + unstaged" `##a4`

Round 1, the agent (F1). Bears on #default-source-worktree. "The walk is `git ls-files --cached
--others --exclude-standard`. So `check` also reads untracked, non-ignored files. A tracked file
deleted on disk without `git add` is a phase-2 finding". The default kept is "the working tree as
git lists it".

### `commits` already judges a commit's tree in memory, with three differences from `check` `##a5`

Round 1, the agent (F2). Bears on #staged-tree-source, #staged-check-semantics,
#commit-tree-in-check. `commit_tree` builds the full model from git objects "with three differences
from `check`: installed files are not compared (`history.rs:77`); tracked-and-ignored files are not
reported, because a commit has no index; extensions are prepared with `Purpose::Commit` over
`Tree::Commit`." So the staged run "reuses the `commits` assembly instead of writing a third
reader", and "A staged tree has no message". `Gathered::over` "builds `Inputs` from a tree on disk.
A staged tree is not one."

### Git reads the staged tree with the batch reader `commits` uses `##a6`

Round 1, the agent (F3). Bears on #staged-tree-source. Measured on git 2.43 in a scratch
repository: `ls-files -s -z` lists mode, blob and relative path, and `cat-file --batch -z` with
`:./a.md` returned the staged bytes, not the working tree's. "So the only change in `git.rs` is how
an object is named: `<sha>:./<path>` for a commit, `:./<path>` for the index."

### `git write-tree` writes objects and fails on unmerged entries `##a7`

Round 1, the agent. Bears on #staged-tree-source. "`git write-tree` would give a tree SHA usable
unchanged, but it writes objects into `.git/objects` and fails on unmerged entries." An index
holding unmerged entries is refused with exit 2: "the index holds unmerged paths".

### The summary must say which tree was walked `##a8`

Round 1, the agent. Bears on #staged-tree-source. From
`tripwire@core@walked-count-differs-between-machines`: "A staged walk prints a different `walk:`
count, so the summary must say which tree was walked." The block gains `tree: staged`.

### The lost hook's reason does not cover an explicit flag `##a9`

Round 1, the agent. Bears on #staged-tree-source. The `commit-msg` hook refusing a staged tree with
findings "lost because "every work-in-progress commit would then need a way around the hook". An
explicit flag needs no way around, so that reason does not cover this proposal."

### CI runs `check` on a checkout, so the staged run takes `check`'s rules `##a10`

Round 1, the agent. Bears on #staged-check-semantics. "Discriminating fact: CI runs `check` on a
checkout, not `commits`'s tree rules alone. Under the rival, `--staged` passes a stale installed
file that CI then fails. That defeats #c-predicts-the-commit."

### The ignore rules stay the disk's `##a11`

Round 1, the agent. Bears on #staged-check-semantics. "the ignore rules are read from disk, as
`commits` already reads them. A `.gitignore` edit that is unstaged can move the verdict. I propose
to accept that and say so in the head."

### `Tree` and `Purpose` are exhaustive, and 0.x costs a break by today's consumers `##a12`

Round 1, the agent (F4). Bears on #extension-tree-for-index. "`Tree` and `Purpose` are public and
exhaustive (`extension.rs:29`, `:42`). Adding a variant is a breaking change of the extension API.
The project is at 0.x, and per `instructions@release-status` this is costed by today's consumers,
not ruled out."

### `Tree::Checkout` would be false under `--staged` `##a13`

Round 1, the agent. Bears on #extension-tree-for-index. "`Tree::Checkout(root)` would be false,
because the files on disk are not the staged ones."

### A third variant gives every extension a third case `##a14`

Round 1, the agent. Bears on #extension-tree-for-index. Of `Tree::Index`: "That is also major, and
it gives every extension a third case to handle."

### thaum's migration is two sites `##a15`

Round 2, the agent, on the owner's request to "check thaum at ../thaum, to assess migration cost".
Bears on #extension-tree-for-index. "The only consumer of `Tree` is
`path@elsewhere@tools/rules-corpus/citations/src/rules_extension.rs`" … "The mechanical migration is two sites:
`Tree::Commit` becomes `Tree::Snapshot`, and `CommitTree` becomes `Snapshot`. The snapshot API
thaum uses is `read` and `object_id`. Both have an index answer: the blob id comes from
`ls-files -s`."

### thaum's filesystem checks are not run over a snapshot, visibly `##a16`

Round 2, the agent. Bears on #extension-tree-for-index. "`self.commit` ignores the `purpose` it is
given and sets `Purpose::Commit` at :347. Under `--staged`, thaum's `changes` and `corpus` checks
therefore do not run, and its report lists them as not run (:462-465). So for thaum, `check
--staged` judges less than `check`. The gap is visible, not silent. If thaum wants them, it changes
its own code."

### `commits <rev>~1..<rev>` already judges a commit's tree `##a17`

Round 1, the agent. Bears on #commit-tree-in-check. "`cargo klarch commits <rev>~1..<rev>` already
judges that commit's tree with the tip checker, and its message too. A `check --commit` with
`check`'s rules (installed files compared) would give a different verdict on the same tree than
`commits` gives (F2, `history.rs:77`). That fails #c-one-verdict-per-tree." With `commits`'s rules
it "is the tree half of `commits` under a second name, so it adds a command and no capability."
The gap named, unverified: "the root commit has no `<rev>~1`."

### `index --staged` earns its place when the unstaged changes touch a register `##a18`

Round 1, the agent. Bears on #index-staged-write. "Say the unstaged changes touch a register, for
example a second new issue file you want in a later commit. The `index.md` the staged commit needs
then differs from what plain `index` writes. Today the only repairs are to hand-edit a generated
file or to move the unstaged file away."

### Shape A leaves both checks failing, shape B leaves both passing `##a19`

Round 1, the agent. Bears on #index-staged-write. Shape A: "After it, `check --staged` fails until
you `git add` the file. Plain `check` also fails, because the working-tree index no longer matches
the working tree." Shape B: "After it, `check` and `check --staged` both pass. No step is left to
the user."

### Shape B writes into git, and only determined bytes `##a20`

Round 1, the agent. Bears on #index-staged-write, #fix-with-staged. "The command writes into git,
under `.git/index.lock`. That is new for this tool. It touches only the staged entry of a generated
file, whose bytes are determined, so no hand-written content can be lost." "B is a new decision for
`index`. It needs a head. If `--fix` reuses it, `tripwire@core@fix-makes-a-choice` fires and
`design@core@safe-fix-definition` reopens."

### `--fix` cannot lose unstaged content; the hazard is a wrong commit `##a21`

Round 1, the agent, correcting the premise of `argument@staged-tree-check@a3`. Bears on
#fix-refusal-mixed-state. "`--fix` writes only generated files and installed files
(`design@core@fix-scope`). Neither holds hand-written bytes, so it cannot lose unstaged content."
"The real hazard is a wrong commit. `--fix` writes `index.md` from the staged and unstaged changes
together. If you then `git add` that file and commit only the staged part, `commits` reports the
committed index as stale."

### The wide trigger fires where the fix is correct for both trees `##a22`

Round 1, the agent. Bears on #fix-refusal-mixed-state. ""Any staged change and any unstaged
change" also fires when the unstaged changes touch no input of any generated file, for example a
typo fix elsewhere. There the fix is correct for both trees, and refusing blocks the ordinary
edit-then-`check --fix` loop."

### The narrow refusal costs a second model build `##a23`

Round 1, the agent. Bears on #fix-refusal-mixed-state. "a second model build, from the staged
tree, whenever the index differs from HEAD."

### A note is missed where an exit code is not `##a24`

Round 1, the agent. Bears on #fix-refusal-mixed-state. Against a note in place of the refusal: "a
note in a long report is easy for an agent to miss, while exit 2 cannot be missed."

### Later quick fixes may not be compatible with writing into git's index `##a25`

Round 2, the owner. Bears on #fix-with-staged, #index-staged-write. "I would not recommend to use
this with --fix though. The reason  is that I'd like to provide more types of quick fixes later,
and this might not be compatible."

### A refusal, not a silent ignore, and `--fix` stays off git's index `##a26`

Round 2, the agent. Bears on #fix-with-staged. "a silent "`--fix` ignores `--staged`" would write
working-tree bytes while the user asked about the stage." "`--fix` never writes into git's index.
So `tripwire@core@fix-makes-a-choice` does not fire, and `design@core@safe-fix-definition` stays as
it is." The rival, `--fix --staged` running `index --staged` then `check --staged`, "loses on your
reason. It would also tie every future fix to a write into git's index."

### `index --staged` is gated, and leaves no stale index to commit `##a27`

Round 2, the agent, checking the closed threads against each other. Bears on #index-staged-write,
#fix-refusal-mixed-state. "`index --staged` builds the staged model and runs phases 1–3 before it
writes anything, as `index` does on the working tree. A stale install in the staged tree therefore
refuses it too". "After `index --staged`, the working-tree `index.md` keeps the rows of the working
tree. A later `git add -A` stages those rows, and they are the correct ones for the full tree. No
sequence I traced commits a stale index." "The narrow refusal's repair line names `index
--staged`, so it depends on shape B."

### The workflow ships no pre-commit instruction today `##a28`

Round 1, the agent. Bears on #workflow-delivery-staged. "The shipped skills never tell a session to
check before each commit. That instruction is this repository's (CLAUDE.md, `## Git` point 1)."
The setup line: "The workflow currently ships no pre-commit instruction, so this is new workflow (a
CHANGELOG `Workflow` entry)."

### The versioning of each change `##a29`

Round 1, the agent. Bears on every thread. "`cli`, minor for `--staged` on `check` and `index`;
`checks`, minor or major for the refusal, since it breaks an invocation that used to succeed.
`issue@knowledge-architect@command-output-is-not-declared-a-contract` leaves this unsettled;
`library`, major for the `Tree` change; `agent-skills`, patch, with a Workflow entry."

## New names, in one place

```text
crates/core/src/git.rs
  enum Source<'a> { Index, Commit(&'a str) }        new: which snapshot a read names
  fn snapshot_entries(root, Source) -> Vec<Entry>   new: `ls-tree -r -z <sha>` or `ls-files -s -z`
  fn object_name(Source, rel) -> String             replaces tree_object: "<sha>:./<path>" or ":./<path>"
  fn blob_bytes(root, Source, paths)                existing, takes a Source instead of a sha
  fn blobs(root, Source, paths)                     existing, takes a Source instead of a sha
  fn index_differs_from_head(root) -> bool          new: `git diff --cached --quiet`, exit 1 = differs
  fn unmerged(root) -> Vec<PathBuf>                 new: index entries at stage 1 to 3
  fn stage_generated(root, &[(PathBuf, String)])    new: `hash-object -w --stdin`, then
                                                    `update-index --add --cacheinfo <mode>,<oid>,<path>`

crates/core/src/extension.rs
  enum Tree { Checkout(&Path), Snapshot(&Snapshot) } Commit renamed Snapshot
  struct Snapshot                                   CommitTree renamed; holds, read, object_id kept
  fn Snapshot::revision(&self) -> Revision<'_>      new
  enum Revision<'a> { Commit(&'a str), Index }      new, public

crates/core/src/cli/mod.rs
  CheckArgs::staged: bool                           new: `--staged`, conflicts_with `fix`
  IndexArgs { staged: bool }                        new: `Command::Index` takes `IndexArgs`
  Command::Index(IndexArgs)                         changed: was a unit variant

crates/core/src/cli/history.rs
  read_tree, commit_tree                            existing, take a Source; a staged run
                                                    builds `Inputs` with installed and shipped
                                                    filled and tracked_and_ignored asked of git
```

Each name is an illustration of a shape, not authority. The implementing session chooses the
final spelling; `Snapshot`, `Revision`, `--staged` and `IndexArgs` are the names the owner was shown.

## Decided design

### `check --staged`

```text
cargo klarch check              the working tree, as today
cargo klarch check --staged     the staged tree
```

- **The listing** is `git ls-files -s -z`, run from the project root, so the paths are
  project-relative as `ls-tree` gives them. Mode `120000` and `160000` are read as a symlink and a
  gitlink, as in a commit's tree.
- **The bytes** are read with the existing `cat-file --batch -z`, each request `:./<path>`, which
  names the stage-0 blob relative to the working directory. Measured on git 2.43.0 in a scratch
  repository in round 1: after staging `a` and writing `b` to the file, the request returned `a`.
- **An index holding an unmerged entry**, stage 1 to 3, is refused with exit 2:
  `the index holds unmerged paths: <paths>`. A conflicted merge or rebase has no tree to commit.
- `git write-tree` is not used: it writes objects into the repository and fails on unmerged
  entries.
- **The rules are `check`'s, as on a clean checkout of the staged tree**: the installed files are
  compared against the running binary's shipped set; the tracked-and-ignored files are reported,
  asked of git as today, since the question is about the index; the untracked-files note is not
  printed, since a staged tree holds no untracked file. The ignore rules a path reference asks are
  read from disk, as `commits` reads them.
- **The summary block gains a line**, `tree: staged`, printed beside `walk:` on a stop and on a full
  run. When the index equals HEAD the line reads `tree: staged (nothing staged: the tree of HEAD)`,
  per AC2. Plain `check` prints `tree: working`.
- **The default is unchanged.** Plain `check` judges the working tree, untracked files included.

The argument: the staged tree is the only tree a session cannot judge before the commit exists,
and `commits` judges it only after. The nearest rival, the rules of `commits` over the staged tree,
lost: CI runs `check` on a checkout, so a staged run that skips the installed files passes a tree
CI fails.

### Extensions over a snapshot

`Tree::Commit(&CommitTree)` is renamed `Tree::Snapshot(&Snapshot)`. `Snapshot::revision()`
answers `Revision::Commit(sha)` or `Revision::Index`. `object_id` answers the blob id, from
`ls-files -s` for the index. Under `check --staged`, each extension is prepared with
`Tree::Snapshot` and `Purpose::Check`; under `commits`, with `Tree::Snapshot` and `Purpose::Commit`
as today.

The cost, measured by reading thaum at `6a2a1d79`: two edit sites, lines 205 and 313 of its
extension. Its `commit` function sets `Purpose::Commit` itself, so its `changes` and `corpus`
checks are listed as not run under `--staged`. The gap is printed, not silent. The nearest rival, a
third variant `Tree::Index`, lost: it makes every extension write a third arm that reads the same
git objects as the second.

### `index --staged`

`cargo klarch index --staged` builds the staged model, gates it on phases 1 to 3, and computes the
generated files from it. For each one whose bytes differ from the staged blob, it writes the bytes
into git's object store and sets the staged entry to them. The working-tree file is not touched.

```text
                          working-tree index.md     staged index.md
index --staged            unchanged                 = the staged tree's rows
```

- After it, `check` and `check --staged` both pass, where each passed but for the generated files.
- A missing staged entry is added with mode `100644`; an existing one keeps its mode.
- Nothing is written when the gate finds anything, when a destination is refused, or when git
  refuses the index lock: exit 2, per AC3.
- It prints each file it staged, `<path>  staged`, and `<path>  already current` for the others,
  as `index` prints today.

The argument: when the unstaged changes touch a register, the `index.md` the staged commit needs
differs from what `index` writes, and the only repairs today are a hand edit of a generated file or
moving the unstaged file away. The nearest rival, writing the staged rows into the working-tree
file, lost: after it, plain `check` fails, since the working-tree file no longer matches the working
tree, and `check --staged` fails until the file is staged.

### `check --fix` refusals

Two refusals, both exit 2 with nothing written:

1. **`--fix` with `--staged`** is refused at parse time by clap:

   ```text
   error: --fix judges and repairs the working tree; it cannot be combined with --staged
     → for the staged tree: cargo klarch index --staged, then cargo klarch check --staged
   ```

2. **A partial commit's mismatch.** Before any write, when `git diff --cached --quiet` says the
   index differs from HEAD, `--fix` computes the generated files from the working tree and from the
   staged tree. When any differ, it refuses:

   ```text
   error: <path> would differ between the working tree and the staged tree
     → to commit the staged changes: cargo klarch index --staged, then cargo klarch check --staged
     → to fix the working tree as a whole: cargo klarch index
   ```

   When the index equals HEAD, or every generated file is the same in both, `--fix` runs as today.

The refusal comes first in the order of `design@core@fix-before-the-checks`, before the installed
files are repaired. A staged tree that stops in phases 1 to 3 cannot be compared; `--fix` then
runs as today, since no staged generated file can be computed. The argument: `--fix` cannot lose
unstaged content, since it writes only generated and installed files, but the `index.md` it writes
reflects the working tree, and staging it with a partial commit makes a commit whose tree fails.
The owner's wider trigger, any staged change with any unstaged change, lost: it also fires when the
unstaged changes touch no input of a generated file, where the fix is correct for both trees.

### Delivery in the shipped text and in this repository

| file | edit |
| --- | --- |
| `path@agent-skills@content/skills/issue-tracking/SKILL.md`, the paragraph on the index | after `{{command}} check --fix`: when committing part of the working tree, `{{command}} index --staged`, then `{{command}} check --staged`; and that `check --fix` refuses when the two trees need different generated files |
| `path@agent-skills@content/skills/planning/SKILL.md`, the layout bullet on `index.md` | one sentence naming `{{command}} index --staged` for a partial commit |
| `path@agent-skills@content/skills/setup/SKILL.md`, section `skill@knowledge-architect-setup@setup-gates` | recommend `{{command}} check --staged` after staging and before each commit, beside the gates command, which judges the branch before a merge |
| `path@knowledge-architect@CLAUDE.md`, section `instructions@verify-mechanically` | "Run `cargo klarch check` before each commit" becomes "Run `cargo klarch check --staged` after staging and before each commit" |
| `path@core@README.md` | the command table gains `check --staged` and `index --staged`; the `--fix` paragraph gains the two refusals |

The shipped edits cite no entry of this repository and write the command as `{{command}}`, per
`design@agent-skills@shipped-text-cites-no-entry`.

## Mapping tables

| `check` today | `check --staged` |
| --- | --- |
| `Model::build` from disk | `commit_tree` over `Source::Index`, at `Depth::Judged` |
| `Gathered::over` | the snapshot's `Assembly::inputs()`, with `installed` from the survey, `shipped` from `agents::shipped`, and `tracked_and_ignored` from `git::tracked_and_ignored` |
| `Tree::Checkout(root)`, `Purpose::Check` | `Tree::Snapshot(&snapshot)`, `Purpose::Check` |
| the untracked-files note | not printed |
| `walk:` | `walk:` and `tree: staged` |

| `extension` today | after |
| --- | --- |
| `Tree::Checkout(&Path)` | `Tree::Checkout(&Path)` |
| `Tree::Commit(&CommitTree)` | `Tree::Snapshot(&Snapshot)` |
| `CommitTree::holds`, `read`, `object_id` | `Snapshot::holds`, `read`, `object_id` |
| none | `Snapshot::revision() -> Revision` |

## Losing alternatives

- **`check --commit <rev>`**, the thread #commit-tree-in-check, with `check`'s rules or with
  `commits`'s. Lost on `argument@staged-tree-check@a17`: `commits <rev>~1..<rev>` judges that tree,
  and a second judge gives it a second verdict or adds no capability.
- **The rules of `commits` over the staged tree.** Lost to #staged-check-semantics on
  `argument@staged-tree-check@a10`.
- **`git write-tree` to name the staged tree.** Lost to #staged-tree-source on
  `argument@staged-tree-check@a7`.
- **`Tree::Checkout` under `--staged`.** Lost to #extension-tree-for-index on
  `argument@staged-tree-check@a13`.
- **A third variant `Tree::Index`.** Lost to #extension-tree-for-index on
  `argument@staged-tree-check@a14`.
- **`index --staged` writing the working-tree file**, shape A. Lost to #index-staged-write on
  `argument@staged-tree-check@a19`.
- **Refusing `--fix` whenever the tree holds both staged and unstaged changes**, the owner's wider
  trigger. Lost to #fix-refusal-mixed-state on `argument@staged-tree-check@a21` and
  `argument@staged-tree-check@a22`.
- **A note in place of the refusal.** Lost to #fix-refusal-mixed-state on
  `argument@staged-tree-check@a24`.
- **`--fix` silently ignoring `--staged`.** Lost to #fix-with-staged on
  `argument@staged-tree-check@a26`.
- **`check --fix --staged` running `index --staged` then `check --staged`.** Lost to
  #fix-with-staged on `argument@staged-tree-check@a25` and `argument@staged-tree-check@a26`.

## Readings

None. The work reads no external specification. Git's documented behaviour of `ls-files -s`,
`cat-file --batch -z` with `:./<path>`, `diff --cached --quiet`, `hash-object -w` and
`update-index --cacheinfo` is relied on, and AC1 and AC3 test each against the git the tests run.

## Premortem

Assume the work shipped and failed. The causes, as presented in the agent's reply to round 2:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. `--staged` passes and CI fails on the same tree: a `.gitignore` edit not staged, or an extension check not run over a snapshot | #staged-check-semantics, #extension-tree-for-index | the tripwire **T1**, on the owner's word, round 3 |
| 2. The index reader lists a different file set than `git commit` records: an intent-to-add entry, a staged deletion, a sparse or skip-worktree entry | #staged-tree-source | the acceptance criterion **AC1**, on the owner's word, round 3 |
| 3. A session runs `check --staged` before staging anything, and it passes on the tree of HEAD | #workflow-delivery-staged | the acceptance criterion **AC2**, on the owner's word, round 3 |
| 4. The write into git's index damages more than one entry: a concurrent lock, a new path | #index-staged-write | the acceptance criterion **AC3**, on the owner's word, round 3 |
| 5. Sessions get past the refusal by unstaging, `git reset` or `git stash` | #fix-refusal-mixed-state | the tripwire **T2**, on the owner's word, round 3 |

T1, as the harvest writes it: fires when a commit whose tree passed `check --staged` just before
it was made then fails `check` in CI, or the tree half of `commits`, with no file edited in
between. Response: open a `defect` naming the finding and its source. Re-entry: each time a CI
failure is read against a local run.

T2, as the harvest writes it: fires when a retrospective finding, or an observation of the owner,
reports a session that unstaged or stashed changes after the refusal of
#fix-refusal-mixed-state. Response: reopen the decision. Re-entry: each retrospective intake. It
fires only in sessions the owner sees, as
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` records of
every tripwire on agent behaviour.

## Acceptance criteria

### AC1: the staged walk is the tree `git write-tree` would record `##staged-walk-equals-write-tree`

Guards `thread@staged-tree-check@staged-tree-source`. Judged at step 1. A test builds an index
holding an intent-to-add entry (`git add -N`), a staged deletion, and a file whose staged and
working bytes differ, and asserts that the set the staged snapshot lists equals
`git ls-tree -r --name-only $(git write-tree)`, and that the bytes read are the staged ones.
`write-tree` appears in the test alone. Fires when the sets or the bytes differ. Response: reopen
#staged-tree-source on how the index is read.

### AC2: `check --staged` says when nothing is staged `##staged-says-nothing-staged`

Guards `thread@staged-tree-check@workflow-delivery-staged`. Judged at step 2. A test runs
`check --staged` over a project whose index equals HEAD and asserts the summary line
`tree: staged (nothing staged: the tree of HEAD)`. Fires when the line is absent. Response: reopen
#workflow-delivery-staged on how a session is told to stage first.

### AC3: `index --staged` changes only the staged entries of the generated files that differ `##index-staged-touches-only-generated-entries`

Guards `thread@staged-tree-check@index-staged-write`. Judged at step 3. Tests assert: after
`index --staged`, `git diff --cached --name-only` against the index before it names exactly the
generated files whose staged bytes differed, a missing one included; every working-tree file is
byte-identical to before; it exits 2 with the index unchanged when phases 1 to 3 of the staged tree
find anything, and when `.git/index.lock` exists. Fires when any assertion fails. Response: reopen
#index-staged-write.

## Implementation sequence

Each step follows the project's development procedure, `skill@klarch-development`, and passes the
gates it owes. Each commit that changes `path@agent-skills@content/` runs
`cargo klarch install-agent-skills` and holds the installed copies.

1. **The staged snapshot, empty of any command.** `git::Source`, the index listing, the
   `:./<path>` requests, the unmerged refusal, and `read_tree`/`commit_tree` taking a `Source`.
   `commits` passes `Source::Commit` and is otherwise unchanged. Claims: AC1; every existing test of
   `commits` passes unchanged. Fails alone on: AC1's set or bytes.
2. **`check --staged` and the extension snapshot.** `CheckArgs::staged`, the staged run with
   `check`'s inputs, the `tree:` line, `Tree::Snapshot`, `Snapshot::revision`. The doc comments of
   `Tree`, `Snapshot` and `Inputs.present` say which tree each describes. `path@core@README.md` and
   `path@core@CLAUDE.md` updated for the second source. Claims: AC2; a test in which the working
   tree passes and the staged tree fails, and the reverse, each giving the staged verdict under
   `--staged`; a stale installed file in the staged tree is reported. Fails alone on: a verdict
   that follows the working tree under `--staged`.
3. **`index --staged`.** `IndexArgs`, `git::stage_generated`, the gate over the staged model.
   README and source references to `design@core@generated-files-are-pure` updated. Claims: AC3.
   Fails alone on: a working-tree file changed, or a staged entry other than a generated file's.
4. **The `--fix` refusals.** The clap conflict, `git::index_differs_from_head`, the comparison of
   the two generated sets before any write. README and source references to
   `design@core@check-fix-flag` and `design@core@fix-before-the-checks` updated. Claims: a test
   where the two trees need the same `index.md` and `--fix` runs; one where they differ and it
   exits 2 with nothing written; one where the index equals HEAD and it runs; `--fix --staged`
   exits 2 before any read. Fails alone on: a write before the refusal.
5. **The shipped text, this repository's CLAUDE.md, and the changelog.** The edits of "Delivery in
   the shipped text and in this repository". Entries in the `Next release` section of CHANGELOG.md,
   then `cargo x changelog`:
   - New features: `cli`, minor: `check --staged` judges the tree in git's index; `cli`, minor:
     `index --staged` writes the generated files the staged tree needs into git's index.
   - Migration: `cli`, major, per D1: `check --fix` refuses when the staged tree and the working tree need
     different generated files, and with `--staged`; `library`, major: `extension::Tree::Commit`
     is `Tree::Snapshot`, and `CommitTree` is `Snapshot`; an extension renames both;
     `library`, major: `cli::Command::Index` takes `IndexArgs`.
   - Workflow: `agent-skills`, patch: the setup skill recommends `check --staged` before each
     commit, and the issue-tracking and planning skills name `index --staged` for a partial commit.
   Fails alone on: a changelog copy that differs. The review of
   `skill@knowledge-architect-planning@working-a-slice` point 4 runs after this step.
6. **The harvest**, below, reviewed before the merge. The spec is deleted in the commit that
   completes it.

## Order rationale

Step 1 before step 2: `check --staged` reads through the snapshot step 1 proves. Step 2 before
step 3: `index --staged` builds the staged model that step 2 assembles. Step 3 before step 4: the
refusal's message names `index --staged`, and its comparison computes the staged generated files
that step 3 computes. Step 4 before step 5: the shipped text describes the commands as built. Step
5 before step 6: the harvest records the decisions the built work implements.

## Defaults awaiting the owner

- **D1**, on #fix-refusal-mixed-state: the refusal is classed `cli`, major, in the changelog,
  since a `check --fix` that succeeded before now exits 2. Round 1 named it "`checks`, minor or
  major" and left the class to
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`. The default is the
  author's judgement: the refusal changes no check's verdict, it changes what a command does.

## Harvest

At step 6, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's
commit, with the test it fails.

| item | home |
| --- | --- |
| #staged-tree-source, with #staged-check-semantics and #default-source-worktree | a new head in `path@core@docs/design.md`, slug `staged-tree-source`, under "1. The shape of a run", after `design@core@git-supplies-the-walk`; that head rewritten in place to name the second source |
| #extension-tree-for-index | a new head in `path@core@docs/design.md`, slug `an-extension-reads-a-snapshot`, since the thread's slug names the question rather than the decision; `design@core@api-facade` edited for the rename |
| #index-staged-write | a new head in `path@core@docs/design.md`, slug `index-staged-write`, after `design@core@generated-files-are-pure`, which is rewritten: `index` takes one flag |
| #fix-refusal-mixed-state, with #fix-with-staged | a new head in `path@core@docs/design.md`, slug `fix-refusal-mixed-state`, after `design@core@check-fix-flag`; `design@core@fix-before-the-checks` rewritten: the refusal comes first |
| #workflow-delivery-staged | judged by the recording tests for a head in `path@agent-skills@docs/design.md` on the recommendation of `check --staged` before each commit |
| every item of "Losing alternatives" | `path@core@docs/rejected-alternatives.md`, each as the recording tests admit |
| T1 | `path@core@docs/tripwires.md`, guarding the head `staged-tree-source`, as the Premortem section words it |
| T2 | `path@core@docs/tripwires.md`, guarding the head `fix-refusal-mixed-state`, as the Premortem section words it |
| AC1, AC2, AC3 | reported on at the landing; each spent, or proposed to the owner as a tripwire if it recurs |
| this spec | deleted in the harvest's commit, cited as `spec@plans@staged-tree-check` |

## Later consequences

- An extension that wants its filesystem checks under `--staged`, such as thaum's `changes` and
  `corpus`, reads `Snapshot::revision()` and decides in its own code.
- A later quick fix of `--fix` is not tied to git's index, since #fix-with-staged keeps the two
  apart. Combining them is reopened only by reopening that thread.
- When `issue@core@installed-file-findings-belong-in-phase-four` closes, `index --staged` is no
  longer refused by a stale install, as `index` is not.
