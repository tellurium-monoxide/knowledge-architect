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
- **The spec and its work land together.** The work adds options to `check` and
  `index`, changes `cli::Command::Index` from a unit variant to one carrying `IndexArgs`, and
  renames an extension type. It does not change what `cargo klarch commits` judges in a
  commit, so `design@agent-skills@plan-lands-before-gate-change` does not apply. The spec and its
  work land on one branch, `staged-tree-check`, in one pull request.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/2d705814-4154-4871-9a54-a3712e881823.jsonl`.
  The discussion begins at the owner's message that opens "I would like for the checker to have
  the ability to run the checks on a tree restricted to either of", and ends at the owner's message
  "Table approved, keep all tripwires and acceptance criteria.proceed to the spec". It holds 3
  owner messages, called rounds 1 to 3 below, and 2 agent replies, one after each of rounds 1 and
  2. After the spec's reviews, the owner ruled on its six defaults in one message, called round 4
  below: "All defaults approved".

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **the working tree**: what `check` judges today. Git's listing, which the head
  `design@core@git-supplies-the-walk` spells `git ls-files --cached --others --exclude-standard` and
  `git::entries` of `path@core@src/git.rs` runs as two invocations, `ls-files -z -s --cached` and
  `ls-files -z --others --exclude-standard`; the bytes are read from disk. It holds untracked files
  that no ignore rule covers.
- **the staged tree**: the tree `git commit` would record, absent a pathspec or `-a`: HEAD's tree
  with the index's changes against it, intent-to-add entries excluded, as "`check --staged`" under
  Decided design lists it, with each blob's bytes read from git's object store.
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
  is `path@elsewhere@thaum/tools/rules-corpus/citations/src/rules_extension.rs`, read at its commit
  `6a2a1d79`. Its checkout sits beside this repository's, at `../thaum` from this repository's
  root, on the owner's machine.
- **F1, F2, F3, F4**: the four facts the agent's reply to round 1 stated before its proposals, each
  an argument below: F1 is `argument@staged-tree-check@a4`, F2 `argument@staged-tree-check@a5`, F3
  `argument@staged-tree-check@a6`, F4 `argument@staged-tree-check@a12`.
- **the checkpoint table**: the full table of threads and criteria the agent displayed at the end of
  its reply to round 2, which the owner approved in round 3. Every row of it is a thread or a
  criterion of this spec.
- **T1, T2, AC1, AC2, AC3**: the labels the premortem put to the owner, kept in the items below.

## What the work is

What exists today at each site the work touches:

| site | today |
| --- | --- |
| `check`, `check` in `path@core@src/cli/mod.rs` | builds the model from disk with `Model::build`, gathers `Inputs` with `Gathered::over` of `path@core@src/cli/gathered.rs`, runs `check::foundation`, prepares each extension with `Tree::Checkout(manifest.root())` and `Purpose::Check`, then `check::run_with` |
| `CheckArgs`, same file | one flag, `--fix` |
| `index`, `index` in the same file | builds the model from disk, gates with `complete_working_tree`, writes each generated file whose bytes differ into the working tree, and takes no flags, per `design@core@generated-files-are-pure` |
| `check --fix`, `fix_then_check` in the same file | repairs the installed files, gates the model on phases 1 to 3, writes the generated files into the working tree, then runs `check`, per `design@core@fix-before-the-checks`. It reads the index's listing through the walk, and never a staged blob |
| the summary block, `counts` in the same file | prints `walk: <n> file(s)` on a stop and on a full run; nothing says which tree was walked |
| a commit's snapshot, `read_tree` and `commit_tree` in `path@core@src/cli/history.rs` | lists a commit with `git::tree_entries` (`ls-tree -r -z`); reads blobs with `git::blob_bytes` and `git::blobs` (`cat-file --batch -z`, each request `tree_object(sha, "")` followed by the path's bytes); reads every listed file the walk does not skip, and adds, in the installer's namespace, the `.md` files the walk would skip (history.rs:267-285), so a skipped non-markdown file there is answered `Unreadable` by the survey (history.rs:459-461); and assembles a model and an `Assembly` whose `inputs()` sets `installed` and `shipped` empty (history.rs:81-82) and whose `tracked_and_ignored` is empty (history.rs:483). `Assembly`, `Depth`, `read_tree` and `commit_tree` are private to the module; `commits` passes them project-relative checker directories (history.rs:590-594). `CommitTree::read` and `CommitTree::object_id` name the commit by its sha (extension.rs:94 and 103), and the unit test `a_tree_object_is_named_relative_to_the_working_directory` of `path@core@src/git.rs` pins `tree_object` |
| `path@core@tests/extension_api.rs` | matches `Tree::Commit(_)` at line 49 |
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
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`. The new `tree:` line,
  printed under `--staged` alone, belongs to the `check --staged` entry of the changelog, and the
  new refusal is classed under the policy as it stands, per D1.
- **thaum's migration.** thaum moves to the renamed type when it moves its pin of the checker, in
  its own repository. This work edits no file of thaum.

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
| `issue@core@a-message-lint-is-deduplicated-in-one-tree-only` | needs nothing: it is about `judge_message`, which a staged run does not call; step 1 reshapes `read_tree` and `commit_tree` only |
| `issue@core@branch-sha-citations-are-judged-within-the-range-only` | read at step 5: the CLAUDE.md sentence step 5 edits is the one this issue's closing condition rewrites. Step 5 changes its first half, `check` to `check --staged`, and leaves the `commits origin/main..HEAD` half and its pointer to this issue as they are |
| `issue@core@a-submodule-is-a-project-of-its-own` | its trigger is not met; the staged listing reads a gitlink and a symlink by mode, as a commit's does, and a claim of step 1 tests it |
| `issue@core@finding-texts-are-not-audited-for-a-needless-cause` | needs nothing: the work adds refusals, which are errors with exit 2 and no finding, and each names its repair only |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | needs nothing new: step 5's shipped edits cite no entry, per `design@agent-skills@shipped-text-cites-no-entry`, and the review of step 5 reads them |
| `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check` | needs nothing: the new `tree:` line is printed under the new `--staged` alone, so it is part of a new feature and no output line is added to an existing command |
| `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` | needs nothing: the searcher tied it to judging a given commit's tree, which #commit-tree-in-check rules out |
| `tripwire@core@checker-source-literals-are-data-self-location` | does not fire, and a claim of step 2 watches it: the staged model matches the checker's directories project-relative, as `commit_tree` does, and the `checker source:` line under `--staged` prints the directories `check` prints, a directory outside the project included, since `commits` drops such a directory (history.rs:590-593) and `check` keeps it |
| `tripwire@core@grammars-not-prefixes` | does not fire: a blob the batch returns no text for is kept as an unreadable document and reported, as `commit_tree` does today; no blob read yields nothing silently |
| `tripwire@core@private-item-needed` | does not fire: `Snapshot` and `Revision` are public in the `extension` role module from step 2 |
| `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to` | read again at step 2: it asks which tree `Inputs.present` describes under `commits`; under `--staged` it describes the staged tree, and the doc comment written at step 2 says so |

## What is already decided

The design rests on these decisions, and does not argue them again:

- `design@core@git-supplies-the-walk`: the walk is git's listing, every git invocation is in
  `path@core@src/git.rs`, and git 2.36 is the floor.
- `design@core@a-commit-message-is-a-document`: a snapshot is assembled from git objects in
  memory, and the ignore rules are read from disk.
- `design@core@phases-gate-the-report`: no writer writes over an incomplete model.
- `design@core@safe-fix-definition` and `design@core@fix-scope`: what `--fix` may write. The work
  does not widen them. Their test is the one "every fix must pass before `--fix` applies it";
  `index --staged` is never applied by `--fix`, per #fix-with-staged, and the head
  `index-staged-write` written at the harvest carries why it may write into git's index,
  `argument@staged-tree-check@a20`.
- `design@core@arguments-parse-through-clap`: refusal 1 is a clap declaration, `conflicts_with`.
- `design@core@ne-minimal`: "A third `extension::Tree` must make every extension say how it reads
  it. A wildcard would read the working tree while the run judges another tree". The rename keeps
  the second property: a snapshot is read from git objects only, never from the working tree, and
  the rename is a compile error for every extension. It does not keep the first for the index,
  which reaches an extension as a revision of a snapshot rather than as a third `Tree`; the owner
  ruled that knowingly, D5, and the harvest rewrites the head, below.
- `design@core@exit-code-ladder`: a refusal before anything runs is exit 2.

The work rewrites four decisions. Every text that `cargo klarch show` lists as referencing each,
on the main branch before this spec:

| decision | texts referencing it | judged or updated at |
| --- | --- | --- |
| `design@core@generated-files-are-pure`: "`cargo klarch index` takes no flags" | `path@core@README.md` lines 172 and 218; `path@core@docs/design.md` lines 866 and 941; `path@core@docs/rejected-alternatives.md` lines 205, 215 and 292; `path@core@src/cli/mod.rs` lines 603, 653 and 719 | step 3 for the README and the source; the harvest for the design home and the rejected alternatives |
| `design@core@check-fix-flag`: gains the two refusals | `path@core@README.md` line 118; `path@core@src/cli/mod.rs` line 429 | step 4 for the README and the source; the harvest for the head |
| `design@core@fix-before-the-checks`: refusal 1 at parse time, and refusal 2 between the gate of phases 1 to 3 and the generated writes, per D4 | `path@core@README.md` line 127; `path@core@docs/design.md` lines 71 and 114; `issue@core@installed-file-findings-belong-in-phase-four`; `path@core@src/cli/mod.rs` line 430 | step 4 for the README and the source; the harvest for the design home and the issue |
| `design@core@git-supplies-the-walk`: the walk has a second source | `path@core@CLAUDE.md` line 23; `path@core@README.md` line 31; `path@core@docs/design.md` line 1515; `path@core@docs/rejected-alternatives.md` line 11; `tripwire@core@walked-count-differs-between-machines`; `path@core@src/check/mod.rs` line 39; `path@core@src/check/tree.rs` line 547; `path@core@src/cli/mod.rs` line 1127; `path@core@src/git.rs` lines 6 and 184; `path@core@src/manifest.rs` line 593; `path@core@src/walk.rs` lines 20 and 61 | step 1 for the source; step 2 for the README and CLAUDE.md; the harvest for the design home, the rejected alternatives and the tripwire |

Six more heads have a sentence the work makes false, or a rule the owner ruled an exception to,
and the harvest rewrites that sentence or records the exception. The texts that reference each, by
`cargo klarch show`, are judged at the harvest: each stays true where only the named sentence
changes.

- `design@core@ne-minimal`, per D5: the index reaches an extension as a revision of
  `Tree::Snapshot`, not as a third `Tree`. Referenced by `path@core@docs/design.md` lines 246
  and 324, `path@core@docs/rejected-alternatives.md` line 432, `path@core@src/check/mod.rs` line
  189, `path@core@src/cli/mod.rs` line 41, `path@core@src/extension.rs` lines 119 and 135,
  `path@core@src/lib.rs` line 209, and `path@knowledge-architect@docs/design.md` lines 143 and
  217.
- `design@core@trait-defaults`, per D5: its argument, that a new mode an extension does not know
  must not run its checks silently as nothing, is met for an extension that lists its not-run
  checks and not for one that lists none; the head records the owner's ruling. Referenced by
  `path@core@docs/rejected-alternatives.md` line 451 and `path@core@src/lib.rs` line 210.
- `design@core@a-commit-message-is-a-document`: "**`check` reads no history.**" `check --staged`
  reads HEAD's tree, one commit and no range, and its verdict is reproducible from the index; the
  sentence is reworded to say `check` reads no range of history. Its 27 referencing sites outside
  this spec, by `cargo klarch show`, cite the head for its other clauses.
- `design@agent-skills@additions-need-real-use`, per D6: the setup skill's recommendation is added
  on the owner's word with no named lack, an exception the owner ruled; the harvest's commit quotes
  the ruling, and the decision-record review judges whether the head records the exception.

- `design@core@the-core-cli-is-a-library-module`: "`check`, the commands that write files (through
  `complete_working_tree` …) … all build on it", of `Gathered`. After the work, `check --staged`
  and `index --staged` build on the staged assembly instead.
- `design@core@an-extension-plugs-in-through-phased-hooks`: "Over a commit it reads git objects
  only". After the work, over a snapshot, a commit or the index.

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

### `check --staged` judges the tree `git commit` would record, read through the snapshot reader `commits` uses `##staged-tree-source`

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
and this might not be compatible." How "not definitive" is carried into the head is D2.

### `check --fix` refuses when the index differs from HEAD and a generated file it would write differs from the staged tree's `##fix-refusal-mixed-state`

Proposed by the owner, round 1, with a wider trigger; the narrow condition by the agent, round 1.
Approved, narrow condition. Arguments: `argument@staged-tree-check@a3`,
`argument@staged-tree-check@a21`, `argument@staged-tree-check@a22`,
`argument@staged-tree-check@a23`, `argument@staged-tree-check@a24`. Shape: Decided design,
"`check --fix` refusals", refusal 2. Harvest: the new head `fix-refusal-mixed-state`. The owner's
words, round 2: "fix-refusal-mixed-state: agreed on your narrower condition". The adversarial review
of step 4 found that this condition refuses where `--fix` would write nothing, which `index`, the
repair it names, cannot clear: a material finding against the premise the closure argued from,
put to the owner as D7. The owner's words, after the work: "Narrower condition approved, go
ahead".

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
`path@elsewhere@thaum/tools/rules-corpus/citations/src/rules_extension.rs`" … "The mechanical migration is two sites:
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
The gap named, unverified at the time: "the root commit has no `<rev>~1`." Measured at assembly,
with this repository's root commit: `cargo klarch commits <root-sha>` judges that commit alone, and
`<root-sha>~1..<root-sha>` exits 2 as unresolvable. So `commits <rev>` covers a root commit, and the
gap is closed.

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
The location quoted was wrong: the sentence is in the section `instructions@verify-mechanically`
of the root CLAUDE.md, and is restated in two project skills, per the delivery table.
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
  struct SnapshotEntry { rel, kind, mode, oid }     new, pub(crate): one listing row; `Entry`,
                                                    which is public, is unchanged
  fn snapshot_entries(root, Source)                 new, -> Vec<SnapshotEntry>:
                                                    `ls-tree -r -z <sha>` for a commit; for the
                                                    index, HEAD's `ls-tree` overlaid with
                                                    `diff --cached --raw -z --no-renames
                                                    --no-abbrev --relative --ita-invisible-in-index`
  fn object_name(Source, rel) -> String             replaces tree_object: "<sha>:./<path>" or ":./<path>"
  fn blob_bytes(root, Source, paths)                existing, takes a Source instead of a sha
  fn blobs(root, Source, paths)                     existing, takes a Source instead of a sha
  fn index_differs_from_head(root) -> bool          new: `git diff --cached --quiet --relative`,
                                                    exit 1 = the project's part differs
  fn unmerged(root) -> Vec<PathBuf>                 new: `ls-files -u -z`, the paths at stage 1 to
                                                    3, each once
  fn stage_generated(root, &[(PathBuf, String)])    new: `hash-object -w --stdin` per file, then
                                                    ONE `update-index -z --index-info` call
                                                    carrying every entry, each path prefixed with
                                                    `git rev-parse --show-prefix`, since
                                                    `--index-info` reads paths from the
                                                    repository's root

crates/core/src/extension.rs
  enum Tree { Checkout(&Path), Snapshot(&Snapshot) } Commit renamed Snapshot
  struct Snapshot                                   CommitTree renamed; holds, read, object_id kept
  fn Snapshot::revision(&self) -> Revision<'_>      new
  enum Revision<'a> { Commit(&'a str), Index }      new, public in the `extension` role module

crates/core/src/cli/mod.rs
  CheckArgs::staged: bool                           new: `--staged`
  IndexArgs { staged: bool }                        new: `Command::Index` takes `IndexArgs`
  Command::Index(IndexArgs)                         changed: was a unit variant
  generated_list(manifest, model, inputs, ...)      existing, takes `Inputs` and a `Tree` instead of
                                                    `&Gathered` and a hard-coded `Tree::Checkout`

crates/core/src/cli/history.rs
  read_tree, commit_tree                            existing, take a Source and the Purpose
                                                    the extensions are prepared with; private
                                                    items become pub(super) for cli/mod.rs
  Depth::Gated                                      new: phases 1 to 3 only, for `index --staged`
                                                    and the `--fix` comparison
  Assembly::installed, Assembly::shipped            new fields: empty under `commits`, filled
                                                    under `--staged`
  Assembly::report: Option<Report>                  new: the whole `Report` of a judged tree, or of
                                                    its stop; `commits` reads its findings as today
  Assembly::snapshot: Option<Snapshot>              new: the snapshot the extensions were prepared
                                                    over, kept for `generated_list`

crates/core/src/cli/mod.rs (summary)
  print_report(report, model, tree: Option<&str>)   existing, gains the tree label; `counts`
                                                    prints `tree: <label>` after `walk:` when given
```

Each name is an illustration of a shape, not authority. The implementing session chooses the
final spelling. The owner was shown `--staged`, `Tree::Snapshot(&Snapshot)` and
`Snapshot::revision()` returning `Commit(sha)` or `Index`; `Revision`, `IndexArgs`, `Depth::Gated`,
the `Assembly` fields and the `git` function names are the author's.

## Decided design

### `check --staged`

```text
cargo klarch check              the working tree, as today
cargo klarch check --staged     the staged tree
```

- **The listing** is the tree `git commit` would record, per D3: the entries of HEAD's tree,
  `ls-tree -r -z HEAD`, or none where HEAD does not exist yet, overlaid with the index's changes
  against it, `git diff --cached --raw -z --no-renames --no-abbrev --relative --ita-invisible-in-index`.
  Each row acts on the listing as the table of status letters below says. Run from the project
  root, `ls-tree` names its entries project-relative and keeps to the project; the diff does both
  only under `--relative`, without which it names paths from the repository's root and lists
  changes outside the project. `--no-abbrev` gives the full blob ids, which `--full-index` does not
  under `--raw`. `--ita-invisible-in-index` is git 2.43.0's default for `diff --cached`, and is
  passed so that the listing does not depend on it. Mode `120000` and `160000` are read as a
  symlink and a gitlink, as in a commit's tree. `git ls-files -s` is not the listing: it lists an
  intent-to-add entry, `git add -N`, at stage 0 with the empty blob, the same line as a staged
  empty file, and `git commit` records no intent-to-add entry. Measured on git 2.43.0 at the
  review of this spec: `ls-files -s` listed both, `diff --cached` and `write-tree` only the empty
  file; with no HEAD, `diff --cached` compared against the empty tree. Measured at the re-review,
  from a project in a subdirectory of its repository: without `--relative` the diff named
  repository-relative paths and a sibling directory's change; with it, project-relative paths
  alone.
- **The bytes** are read with the existing `cat-file --batch -z`, each request `:./<path>`, which
  names the stage-0 blob relative to the working directory. Measured on git 2.43.0 in a scratch
  repository in round 1: after staging `a` and writing `b` to the file, the request returned `a`.
- **An index holding an unmerged entry**, stage 1 to 3, is refused with exit 2:
  `the index holds unmerged paths: <paths>`, each path named once, though `ls-files -u` prints a
  line per stage. A conflicted merge or rebase has no tree to commit.
- `git write-tree` is not used: it writes objects into the repository and fails on unmerged
  entries.
- **The rules are `check`'s, as on a clean checkout of the staged tree**: the installed files are
  compared against the running binary's shipped set; the tracked-and-ignored files are reported,
  asked of git as today, since the question is about the index; the untracked-files note is not
  printed, since a staged tree holds no untracked file. The ignore rules a path reference asks are
  read from disk, as `commits` reads them. The checker version pin is the working tree's, compared
  before any command per `design@core@installed-binary-version-check`, which compares no other
  tree's value; a staged change to the pin is not compared.
- **The report.** `commit_tree` at `Depth::Judged` keeps the whole `Report` that `check::run_with`
  or the stop returns, in `Assembly::report`, and `check --staged` prints it with `print_report`,
  so the summary block, the findings and the verdict line are `check`'s. `commits` reads the
  report's findings where it read `run_with(...).findings` before.
- **The summary block gains a line of its own under `--staged`**, `tree: staged`, directly after
  the `walk:` line, on a stop and on a full run. When the index equals HEAD the line reads
  `tree: staged (nothing staged: the tree of HEAD)`, per AC2. Plain `check` prints no `tree:` line,
  so its output is unchanged; a `walk:` count with no `tree:` line is of the working tree.
- **A staged tree that cannot be assembled**, `Unloadable` in `cli::history`, such as a staged
  deletion of the manifest or a failed blob read, is a could-not-run: exit 2 with the reason, for
  `check --staged` and `index --staged` alike. The unmerged refusal holds for both commands.
- **The listing's rows**, from `git diff --cached --raw` with `--no-renames`:

  | status | listing action |
  | --- | --- |
  | `A` | add the path with the row's mode and blob |
  | `M` | set the path's blob, and its mode |
  | `T` | set the path's mode and blob; the kind follows the new mode |
  | `D` | remove the path |
  | `U` | not reached: `git::unmerged` runs first and refuses |
  | any other letter | exit 2 naming the letter: `--no-renames` rules out `R` and `C` |
- **The default is unchanged.** Plain `check` judges the working tree, untracked files included.

The argument: the staged tree is the only tree a session cannot judge before the commit exists,
and `commits` judges it only after. The nearest rival, the rules of `commits` over the staged tree,
lost: CI runs `check` on a checkout, so a staged run that skips the installed files passes a tree
CI fails.

### Extensions over a snapshot

`Tree::Commit(&CommitTree)` is renamed `Tree::Snapshot(&Snapshot)`, per D5. Its doc comment
states that the index is one of a snapshot's revisions, and that `Purpose::Check` over a snapshot
asks for every check the snapshot can answer. `Snapshot::revision()`
answers `Revision::Commit(sha)` or `Revision::Index`. `object_id` answers the blob id of the
listing. Under `check --staged`, each extension is prepared with `Tree::Snapshot` and
`Purpose::Check`; under `index --staged`, with `Tree::Snapshot` and `Purpose::Index`; under
`commits`, with `Tree::Snapshot` and `Purpose::Commit` as today.

The cost, measured by reading thaum at `6a2a1d79`: two edit sites, lines 205 and 313 of its
extension. Its `commit` function sets `Purpose::Commit` itself, so its `changes` and `corpus`
checks are listed as not run under `--staged`. The gap is printed, not silent. The nearest rival, a
third variant `Tree::Index`, lost: it makes every extension write a third arm that reads the same
git objects as the second.

### `index --staged`

`cargo klarch index --staged` assembles the staged tree at `Depth::Gated`, which runs phases 1 to
3, and stops there with exit 2 and nothing written when they find anything, as `index` does over
the working tree. It then computes the generated files with `generated_list`, given the staged
assembly's `Inputs` and `Tree::Snapshot`. For each one whose bytes differ from the staged blob, or
that the staged tree does not hold, it writes the bytes into git's object store and sets the staged
entry to them. The working-tree file is not touched.

```text
                          working-tree index.md     staged index.md
index --staged            unchanged                 = the staged tree's rows
```

- After it, `check` and `check --staged` both pass, where each passed but for the generated files.
- A missing staged entry is added with mode `100644`; an existing one keeps its mode.
- **A destination is refused**, before any write, when its staged entry is a symlink or a gitlink,
  or when its directory is not in the staged listing: the two refusals of `check_destinations`,
  asked of the staged tree instead of the disk.
- `--index-info` names a path from the repository's root, so each project-relative path is
  prefixed with `git rev-parse --show-prefix`. Measured at the re-review, from a project in a
  subdirectory of its repository: a project-relative path was staged at the repository's root, and
  the prefixed path where intended.
- Nothing is written when the gate finds anything, when a destination is refused, or when git
  refuses the index lock: exit 2, per AC3. "Written" means an index entry or a working-tree file:
  `hash-object -w` runs before the lock is taken, so a refused run can leave unreachable objects in
  git's object store, which `git gc` removes.
- It prints each file it staged, `<path>  staged`, and `<path>  already current` for the others,
  as `index` prints today.

The argument: when the unstaged changes touch a register, the `index.md` the staged commit needs
differs from what `index` writes, and the only repairs today are a hand edit of a generated file or
moving the unstaged file away. The nearest rival, writing the staged rows into the working-tree
file, lost: after it, plain `check` fails, since the working-tree file no longer matches the working
tree, and `check --staged` fails until the file is staged.

### `check --fix` refusals

Two refusals:

1. **`--fix` with `--staged`** is refused at parse time: `--staged` declares
   `conflicts_with = "fix"`, as `design@core@arguments-parse-through-clap` requires of an invalid
   combination, and clap exits 2 with its own wording, before anything is read. The repair the
   owner was shown in round 2, `index --staged` then `check --staged`, is stated in the help text of
   `--staged` and in the README's `--fix` paragraph, since clap's message cannot carry it. The
   review of the spec first replaced the declaration with a hand-written test to print the repair;
   the re-review found that this contradicts the head, and the owner's shape is restored.

2. **A partial commit's mismatch.** When `git diff --cached --quiet` says the index differs from
   HEAD, `--fix` computes the generated files from the working tree and from the staged tree. A
   path that one list holds and the other does not differs, as do two lists' bytes for one path.
   Per D7, only a differing file `--fix` would write counts: one already current on disk is not
   written. When any counts, it refuses, naming each:

   ```text
   error: these generated files would differ between the working tree and the staged tree: <paths>
     → to commit the staged changes: cargo klarch index --staged, then cargo klarch check --staged
     → to fix the working tree as a whole: cargo klarch index
   ```

   When the index equals HEAD, or no differing file is one `--fix` would write, `--fix` runs as
   today. Per D8, a failure to compute the staged side other than the two below refuses, naming it.

**Where refusal 2 sits in the order of `design@core@fix-before-the-checks`**, per D4: after the
manifest's early return, which writes nothing; after the installed files are repaired, since their
bytes do not depend on the tree and are correct for both trees; after the working tree's gate of
phases 1 to 3, whose model the comparison reads; and before any generated file is written. So it
refuses with exit 2 when nothing was installed, and with exit 1 when installed files were written,
as the rule "2 promises an untouched tree" of `fix_then_check` already says. The staged side is
assembled at `Depth::Gated`. Where it stops in phases 1 to 3, or the index holds an unmerged entry,
no staged generated file can be computed, and `--fix` runs as today.

The argument: `--fix` cannot lose unstaged content, since it writes only generated and installed
files, but the `index.md` it writes reflects the working tree, and staging it with a partial commit
makes a commit whose tree fails. The owner's wider trigger, any staged change with any unstaged
change, lost: it also fires when the unstaged changes touch no input of a generated file, where the
fix is correct for both trees.

### Delivery in the shipped text and in this repository

| file | edit |
| --- | --- |
| `path@agent-skills@content/skills/issue-tracking/SKILL.md`, the paragraph on the index | after `{{command}} check --fix`: when committing part of the working tree, `{{command}} index --staged`, then `{{command}} check --staged`; and that `check --fix` refuses when the two trees need different generated files |
| `path@agent-skills@content/skills/planning/SKILL.md`, the layout bullet on `index.md` | one sentence naming `{{command}} index --staged` for a partial commit |
| `path@agent-skills@content/skills/setup/SKILL.md`, section `skill@knowledge-architect-setup@setup-gates` | recommend `{{command}} check --staged` after staging and before each commit, beside the gates command, which judges the branch before a merge |
| `path@knowledge-architect@CLAUDE.md`, section `instructions@verify-mechanically` | "Run `cargo klarch check` before each commit" becomes "Run `cargo klarch check --staged` after staging and before each commit" |
| `path@agent-config@skills/klarch-development/SKILL.md`, the paragraph "Every commit of a branch must pass `commits`" | the same restatement, the same edit |
| `path@agent-config@skills/klarch-release/SKILL.md`, step 3 "Commit" | "with `cargo klarch check` before" becomes "with `cargo klarch check --staged` before" |
| `path@core@README.md` | the command table, lines 35 and 41, gains `check --staged` and `index --staged`; the exit-code table, lines 54 to 56, gains the unmerged refusal and the two `--fix` refusals; the writer-gate paragraph, lines 112 to 115, names `index --staged`; the `--fix` paragraph, from line 117, gains the two refusals; the `## index` section's "It takes no flags", line 178, names `--staged` |

The two rows of the agent-config location's project skills restate the directive of the root CLAUDE.md row and change with
it. The shipped edits cite no entry of this repository and write the command as `{{command}}`, per
`design@agent-skills@shipped-text-cites-no-entry`. Every edit of this table is made under
`skill@knowledge-architect-agent-configuration`, and an edit of the shipped text also follows the
section "Editing an installed skill or agent" of `path@agent-skills@CLAUDE.md`.

## Mapping tables

| `check` today | `check --staged` |
| --- | --- |
| `Model::build` from disk | `commit_tree` over `Source::Index`, at `Depth::Judged` |
| `Gathered::over` | the snapshot's `Assembly::inputs()`, with `installed` from the survey, `shipped` from `agents::shipped`, and `tracked_and_ignored` from `git::tracked_and_ignored` |
| `Tree::Checkout(root)`, `Purpose::Check` | `Tree::Snapshot(&snapshot)`, `Purpose::Check` |
| `check::run_with(...)` printed by `print_report` | `Assembly::report` printed by `print_report` |
| the untracked-files note | not printed |
| `walk:` | `walk:`, then `tree: staged` |

| `index` today | `index --staged` |
| --- | --- |
| `Model::build` from disk | `commit_tree` over `Source::Index`, at `Depth::Gated` |
| `complete_working_tree`, which calls `Gathered::over` | the stop of `Depth::Gated`, then the staged assembly's `Inputs` |
| `generated_list` with `Tree::Checkout(root)`, `Purpose::Index` | `generated_list` with `Tree::Snapshot(&snapshot)`, `Purpose::Index` |
| `check_destinations`: a symlink on disk, a directory absent on disk | a symlink or gitlink entry in the staged listing, a directory absent from it |
| `fs::write` where the bytes differ from the file | `git::stage_generated` where the bytes differ from the staged blob, or no blob is staged |
| `<path>  rewritten` | `<path>  staged` |

| `check --fix` today | after |
| --- | --- |
| clap parses the arguments | `--fix` with `--staged`: exit 2, refusal 1, before anything else |
| manifest complaints: run `check` | unchanged |
| installed repairs | unchanged |
| the gate of phases 1 to 3 | unchanged |
| none | the index differs from HEAD and the staged side gates: compare the two generated lists, and refuse 2 on a difference |
| generated files written, then `check` | unchanged |

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

None. The work reads no external specification. It relies on git's documented behaviour of
`ls-tree`, `diff --cached --raw` with `--ita-invisible-in-index`, `cat-file --batch -z` with
`:./<path>`, `ls-files -u`, `diff --cached --quiet`, `hash-object -w` and
`update-index --index-info`. AC1 tests the listing and the reads, AC3 the staging, and step 4's
claims `diff --cached --quiet`, each against the git the tests run. Measured on git 2.43.0 at the
re-review of this spec: one `update-index -z --index-info` call staged two entries and left the
working files unchanged, and with `.git/index.lock` present it exited 128 with the index
unchanged.

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
`index --staged`, the lines of `git ls-files -s` that differ from its output before the run name
exactly the generated files whose staged bytes differed, a missing one included; every working-tree file is
byte-identical to before; it exits 2 with the index unchanged when phases 1 to 3 of the staged tree
find anything, and when `.git/index.lock` exists. Fires when any assertion fails. Response: reopen
#index-staged-write.

## Implementation sequence

Steps 1 to 4 follow the project's development procedure, `skill@klarch-development`, and pass the
gates they owe. Step 5 edits a CLAUDE.md, project skills and the shipped text, which that skill
excludes: it follows `skill@knowledge-architect-agent-configuration`, and for the shipped text the
section "Editing an installed skill or agent" of `path@agent-skills@CLAUDE.md`. Its tests are the
ones step 5 names. Step 1 has no command to run, so its tests are unit tests in
`path@core@src/git.rs` and `path@core@src/cli/history.rs` that run `git init` in a temporary
directory, as the unit tests of `path@core@src/git.rs` already do. The tests of steps 2 to 4 run
the binary over projects built with `Sandbox` and `History` of `path@core@tests/binary.rs`, per
`path@core@CLAUDE.md`. Each default under "Defaults awaiting the
owner" gates the point it names: that point is built on the default only once the owner has ruled,
and the rest of the step proceeds. Each commit that changes `path@agent-skills@content/` runs
`cargo klarch install-agent-skills` and holds the installed copies.

1. **The staged snapshot, empty of any command.** `git::Source`, the index listing, the
   `:./<path>` requests, the unmerged refusal, and `read_tree`/`commit_tree` taking a `Source`.
   `commits` passes `Source::Commit` and is otherwise unchanged. `CommitTree::read` and
   `object_id` name their tree through the `Source`, the unit test calling `tree_entries`
   (git.rs:899) follows its replacement, and the unit test of `tree_object` follows
   its replacement. Claims: AC1; every existing test of
   `commits` passes unchanged; a staged symlink and a staged gitlink are read as links, as in a
   commit's tree; a conflicted index makes `git::unmerged` list the path, and the staged
   assembly refuses with exit 2. Fails alone on: AC1's set or bytes.
2. **`check --staged` and the extension snapshot.** `CheckArgs::staged`, the staged run with
   `check`'s inputs, the `tree:` line, `Tree::Snapshot`, `Snapshot::revision`. The doc comments of
   `Tree`, `Snapshot`, `Purpose::Check` and `Inputs.present`, and the crate documentation of
   `path@core@src/lib.rs` at line 122, say which tree each describes; the
   match on `Tree::Commit(_)` of `path@core@tests/extension_api.rs` is renamed. The staged run
   passes the checker directories project-relative, as `commits` does, since
   `Model::from_documents_under` matches them against project-relative paths; and under `--staged`
   `read_tree` reads every file of the installer's namespace, skipped by the walk or not, so the
   installed comparison reads the bytes it reads on disk. `path@core@README.md` and
   `path@core@CLAUDE.md` updated for the second source. Claims: AC2; a test in which the working
   tree passes and the staged tree fails, and the reverse, each giving the staged verdict under
   `--staged`; a stale installed file in the staged tree is reported; the `checker source:` line under
   `--staged` prints what `check` prints; `check --staged` over a conflicted
   index exits 2 naming the unmerged paths; plain `check` prints no `tree:` line. Fails alone on: a verdict
   that follows the working tree under `--staged`.
3. **`index --staged`.** `IndexArgs`, `git::stage_generated`, the gate over the staged model.
   README and source references to `design@core@generated-files-are-pure` updated. Claims: AC3.
   Fails alone on: a working-tree file changed, or a staged entry other than a generated file's.
4. **The `--fix` refusals.** The clap declaration `conflicts_with = "fix"` on `--staged`, with the
   test of the declaration that `design@core@arguments-parse-through-clap` asks for, `git::index_differs_from_head`, the comparison of
   the two generated sets before any write. README and source references to
   `design@core@check-fix-flag` and `design@core@fix-before-the-checks` updated. Claims: a test
   where the two trees need the same `index.md` and `--fix` runs; one where they differ and it
   exits 2 with nothing written; one where they differ after an install repair and it exits 1
   with no generated file written; one where the index equals HEAD and it runs; `--fix --staged`
   exits 2 before any read. Fails alone on: a write before the refusal.
5. **The shipped text, this repository's CLAUDE.md, and the changelog.** The edits of "Delivery in
   the shipped text and in this repository". Entries in the `Next release` section of CHANGELOG.md,
   then `cargo x changelog`:
   - New features: `cli`, minor: `check --staged` judges the tree in git's index, and prints a
     `tree: staged` line; `cli`, minor:
     `index --staged` writes the generated files the staged tree needs into git's index.
   - Migration: `cli`, major, per D1: `check --fix` refuses when the staged tree and the working tree need
     different generated files, and with `--staged`; `library`, major: `extension::Tree::Commit`
     is `Tree::Snapshot`, and `CommitTree` is `Snapshot`; an extension renames both;
     `library`, major: `cli::Command::Index` takes `IndexArgs`, and `cli::CheckArgs` gains the
     field `staged`, so a struct literal of it adds the field.
   - Workflow: `agent-skills`, patch: the setup skill recommends `check --staged` before each
     commit, and the issue-tracking and planning skills name `index --staged` for a partial commit.
   Claims: `cargo klarch install-agent-skills` then `cargo klarch check` pass with the installed
   copies committed; `cargo test -p knowledge-architect-agent-skills` passes; the shipped edits
   cite no entry and write `{{command}}`, by a reading; no text of the delivery table still tells
   a session to run plain `check` before each commit, by
   `grep -rnE "klarch check. before" CLAUDE.md .claude/skills`, the dot matching the closing
   backtick. Fails alone on: a
   changelog copy that differs. The review of
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

One, D8. D7, ruled after the work, is under the subsection below.

- **D8**, on #fix-refusal-mixed-state, from the self-consistency and spec-conformity reviews of
  the work: the decided design names two states where the staged side cannot be computed and
  `--fix` runs as before, an unmerged index and a staged tree stopped in phases 1 to 3, and says
  nothing of any other failure, such as an extension that cannot prepare over a snapshot. The code
  first skipped the comparison on every failure. The default, built: any other failure refuses,
  exit 2 or 1, naming it, since a comparison skipped in silence lets the partial commit through,
  per `design@core@a-failed-parse-is-loud`. The alternative: skip it on any failure.

### The defaults the owner ruled on

Six defaults stood after the reviews of this spec. The owner ruled on all six in round 4: "All
defaults approved". Each is applied in the sections it names, and is kept here with its reason. D7
came from the adversarial review of step 4, and the owner ruled it: "Narrower condition approved,
go ahead".

- **D7**, on #fix-refusal-mixed-state: the refusal names only a differing generated file `--fix`
  would write, so a run that writes nothing refuses nothing, and `index` clears it.

- **D1**, on #fix-refusal-mixed-state: the refusal is classed `cli`, major, in the changelog,
  since a `check --fix` that succeeded before now exits 2. Round 1 named it "`checks`, minor or
  major" and left the class to
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`. The default is the
  author's judgement: the refusal changes no check's verdict, it changes what a command does. A
  major change owes an entry, and `design@knowledge-architect@changelog-entries` gives Migration
  to what a consumer changes in its own files, so the entry names one: "a script or a CI step that
  runs `check --fix` over a partly staged index handles exit 2, or runs `index --staged`".
- **D2**, on #index-staged-write: the owner ruled in round 2 "B looks fine to me. This is not
  definitive", and approved the table showing B as approved in round 3. The default: the head
  written at the harvest carries B as approved, and a later change is an ordinary reversal. The
  alternative is a head that states B as provisional.
- **D3**, on #staged-tree-source, from the cold-implementer review: the owner was shown the listing
  as `git ls-files -s`. That listing lists an intent-to-add entry, which `git commit` does not
  record, so the shape as shown fails AC1. The default: the listing is HEAD's tree overlaid with
  `git diff --cached`, which hides intent-to-add entries, as "`check --staged`" states. The
  decision, the staged tree judged with `check`'s rules, is unchanged; the mechanism shown is.
- **D4**, on #fix-refusal-mixed-state, from the cold-implementer review: the owner approved a
  refusal that writes nothing. The comparison needs the working tree's model past phases 1 to 3,
  which a stale install stops in phase 2, the case `--fix` exists to repair. The default: the
  refusal comes after the installed repairs, so it writes no generated file and may have written
  installed files, whose bytes are the same for both trees; it then exits 1 where it wrote, and 2
  where it did not. The alternative: refuse first, before any write, and skip the comparison when
  the working tree stops in phases 1 to 3.
- **D5**, on #extension-tree-for-index, from the design-conformance review: the thread closed on
  `argument@staged-tree-check@a14`, which counts "a third case to handle" as a cost of the rival
  `Tree::Index`. `design@core@ne-minimal` and `design@core@trait-defaults` count the compile error
  that makes every extension say how it reads a new tree as the effect wanted. The rename also
  breaks every extension's match, but the repair it asks is a rename, which carries an extension's
  commit-mode handling into `check --staged` without the extension deciding anything: thaum's
  `changes` and `corpus` checks are then not run under `--staged`, which thaum lists, and an
  extension that lists nothing would skip them silently. The default: the decided shape, the
  rename, with the doc comment of `Tree::Snapshot` stating that the index is one of its
  revisions and that `Purpose::Check` over a snapshot asks for every check the snapshot can
  answer. The alternative: `Tree::Index(&Snapshot)` as a third variant, which makes each extension
  write the arm for the index itself.
- **D6**, on #workflow-delivery-staged, from the design-conformance review:
  `design@agent-skills@additions-need-real-use` admits an addition to an installed skill on a
  behaviour seen in a real session or asked for by the owner mid-session, "with the owner naming
  what the session would have lacked without it". The owner asked for the delivery in round 1 and
  approved the table in round 2. The lack the owner named in round 1 is the partial commit: "the
  user intent in this case is probably to commit partially, and then --fix cannot apply its changes
  safely". It covers the issue-tracking and planning edits, which name the partial-commit forms. It
  does not cover the setup skill's new recommendation of `check --staged` before each commit, for
  which no lack was named. The default: build the whole table, and record the owner's round-1
  words as the named lack in the harvest's commit. The alternative: drop the setup row until a real
  session shows the lack.

## Harvest

At step 6, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's
commit, with the test it fails.

| item | home |
| --- | --- |
| #staged-tree-source, with #staged-check-semantics and #default-source-worktree | a new head in `path@core@docs/design.md`, slug `staged-tree-source`, under "1. The shape of a run", after `design@core@git-supplies-the-walk`; that head rewritten in place to name the second source |
| #staged-tree-source, #index-staged-write | `design@core@the-core-cli-is-a-library-module` rewritten: the staged commands build on the staged assembly, not on `Gathered` |
| #extension-tree-for-index | `design@core@an-extension-plugs-in-through-phased-hooks` rewritten: over a snapshot, git objects only; and a new head in `path@core@docs/design.md`, slug `an-extension-reads-a-snapshot`, since the thread's slug names the question rather than the decision; `design@core@api-facade` edited for the rename |
| #index-staged-write | per D2, the head states B as approved; a new head in `path@core@docs/design.md`, slug `index-staged-write`, after `design@core@generated-files-are-pure`, which is rewritten: `index` takes one flag |
| #fix-refusal-mixed-state, with #fix-with-staged | a new head in `path@core@docs/design.md`, slug `fix-refusal-mixed-state`, after `design@core@check-fix-flag`, which is rewritten to name the two refusals; `design@core@fix-before-the-checks` rewritten: refusal 1 at parse time, refusal 2 after the gate and before the generated writes, per D4; `issue@core@installed-file-findings-belong-in-phase-four` read again for its sentence on that order |
| #extension-tree-for-index, D5 | `design@core@ne-minimal` and `design@core@trait-defaults` rewritten as "What is already decided" says |
| #staged-tree-source | `design@core@a-commit-message-is-a-document`: "`check` reads no history" reworded; `tripwire@core@walked-count-differs-between-machines`: its firing clause compares counts of plain `check`, which prints no `tree:` line, and is read again |
| the existing entries of `path@core@docs/rejected-alternatives.md` that cite a rewritten head, lines 11, 205, 215 and 292 | each read again; each still lost to its head as rewritten, or is edited to say so |
| #workflow-delivery-staged | per D6, the harvest's commit quotes the owner's round-1 words as the lack named under `design@agent-skills@additions-need-real-use`; judged by the recording tests for a head in `path@agent-skills@docs/design.md` on the recommendation of `check --staged` before each commit |
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
