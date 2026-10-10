---
name: klarch-release
description: MUST use when the owner asks to publish a version of knowledge-architect to crates.io. Covers choosing the version, the changelog section, the checks of the shipped text before publishing, the package lists, the dry run, the review and merge that come before publishing, the owner's word at the moment of publishing, the tag and the publish, and what to do when a publish fails part-way.
---

# Release

Scope: publishing one version of the three published crates, `knowledge-architect-agent-skills`,
`knowledge-architect-gates` and `knowledge-architect`, from main. xtask is never published. The
versioning policy is `design@knowledge-architect@versioning-policy`, and the three crates share one
version, per `design@knowledge-architect@version-lockstep`.

**Everything that can be undone comes before the publish, which cannot.** A merge to main is
repaired by a later commit; a version on crates.io can be yanked and never deleted. So the release
is reviewed and merged first, and published from main last, on the owner's word, per
`design@knowledge-architect@publish-after-merge`.

Not covered here: **a repair to a crate's code** found on the release branch,
`skill@klarch-development`; **the changelog entries of each change**, which each branch writes, per
root `CLAUDE.md`, section Git, point 1; **the review itself**, `skill@knowledge-architect-review`,
which step 9 dispatches.

## The release branch `##release-branch`

On a branch of its own, `release-<version>`, under the Git rules of the root `CLAUDE.md`:

1. **The version.** Choose it under the versioning policy, and set it at its four sites:
   - `[workspace.package] version` in the root Cargo.toml;
   - the exact pin `knowledge-architect-agent-skills = { …, version = "=<version>" }` in
     crates/core/Cargo.toml;
   - Cargo.lock, which any cargo command rewrites for the four workspace packages;
   - the status line: the opening sentence of `## Status` in the root README.md. Rewrite its
     wording, not only the number.

   `git grep -n '<previous version>'` finds every site that still names the previous one. An
   example inside a skill, such as an xtask manifest's own version, is not a site.
2. **The changelog.** Rename CHANGELOG.md's `## Next release` to `## <version>`, then run
   `cargo x changelog` to write the crates' copies, per
   `design@knowledge-architect@the-changelog-ships-in-every-crate`. Each branch wrote its own
   entries there, per `design@knowledge-architect@the-branch-writes-its-changelog-entries`, and step 9's review checks
   them, per `design@knowledge-architect@changelog-reviewed-at-the-release`. The version chosen at step 1 follows the highest bump class among its entries, a patch at
   least.
3. **Commit**, with `cargo klarch check --staged` before and
   `cargo klarch commits origin/main..HEAD` after: the whole branch, per root `CLAUDE.md`, section
   Verify mechanically, `design@agent-skills@staged-check-before-each-commit`, and
   `issue@core@branch-sha-citations-are-judged-within-the-range-only`.
   Cargo refuses to package an uncommitted tree, so the commit comes before the next steps. Then
   push the branch and open a draft pull request, per root `CLAUDE.md`, section Git, point 1.
4. **The shipped text cites no entry.** List every backticked span with an `@` in the installed
   copies, which hold the text as it ships, its `%%` comments removed and its placeholders filled,
   and read each one:

   ```sh
   grep -rnoE '`[^`]*@[^`]*`' .claude/knowledge-architect .claude/skills/knowledge-architect-* .claude/agents/knowledge-architect-*
   ```

   A span is a reference candidate when its head before the first `@` is a kind or an anchor, per
   `design@core@candidate-rule-and-retired-forms`. Every candidate either carries a placeholder in
   angle brackets, as in `goal@<anchor>@<id>`, is a path every conforming project holds,
   `path@*@<path>` or `path@plans@<path>`, or cites the shipped set's own skills, agents, primer
   and sections, as `skill@<name>@<slug>` or `primer@<slug>`, which a test of the core resolves
   within the set. No candidate names an entry. This is done by hand until
   `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` closes, per
   `design@agent-skills@shipped-text-cites-no-entry`.
5. **Every name the shipped text uses is shipped**, per `design@agent-skills@no-external-handoff`:

   ```sh
   grep -rhoE 'knowledge-architect-[a-z-]+' .claude/knowledge-architect .claude/skills/knowledge-architect-* .claude/agents/knowledge-architect-* | sort -u
   ```

   Each name must be a shipped skill (`content/skills/<name>/`) or a shipped agent
   (`content/agents/<name>.md`), both installed as `knowledge-architect-<name>`, or one of the
   three crates. Every document the shipped text relies on in a project, such as the knowledge
   table, is written by an installed skill.
6. **crates.io.** For each crate, the API answers 404 for a first release, or shows the crate owned
   by the owner's account; in both cases the version itself must be absent:

   ```sh
   curl -s -A 'knowledge-architect release check' https://crates.io/api/v1/crates/<crate>/<version>
   ```
7. **The package lists.** `cargo package --list -p <crate>` for each crate. Each lists its sources,
   its Cargo.toml, its README, its CRATES-IO.md, its licence files and its CHANGELOG.md, and agent-skills also its
   build script, content/ and snippets/, per `design@knowledge-architect@package-include-whitelist`. The owner reads the lists.
8. **The dry run.** `cargo publish --workspace --dry-run` passes, skips xtask, and orders
   agent-skills before the core, which depends on it. Its warnings "ignoring test `binary`" and
   "ignoring test `extension_api`" are expected: the whitelist keeps the tests out.
9. **The gates and the review.** `cargo x gates --require-rebased`, then the push, and the review
   under `skill@knowledge-architect-review`. Its axes include
   `agent@klarch-changelog-reviewer`, dispatched on `v<previous>..HEAD`: every change of the range
   has the entries the policy owes, each entry's class is right, and the version is their highest
   class. A repair is a new commit, or is folded per root `CLAUDE.md`, section Git, point 2, and
   steps 4 to 8 run again on the head if it touched a crate.

## Merge, then publish `##merge-then-publish`

10. **Merge** under the root `CLAUDE.md`'s merge predicate. From this moment until step 12, main's
    README status line names a version that is not yet on crates.io.
11. **The owner's word, given at that moment.** Without it, nothing is published. The owner is
    logged in to crates.io in the shell the agent runs in, with `cargo login`, and the token never
    passes through the agent. The token needs the scopes publish-new and publish-update, and the
    account a verified email.
12. **Tag and publish**, with a clean tree on main:

    ```sh
    git checkout main && git pull --ff-only
    test "$(git rev-parse HEAD^{tree})" = "$(git rev-parse <the reviewed branch head>^{tree})"
    git tag -a v<version> -m "Release <version>"
    cargo publish --workspace
    git push origin v<version>      # only once all three crates are published
    ```

## When the publish fails `##failed-publish`

`cargo publish --workspace` uploads the crates one by one: agent-skills, gates, then the core. A
crate it uploaded stays published.

- **Nothing was uploaded**: delete the local tag with `git tag -d v<version>`, repair on a branch,
  run steps 4 to 8 again on its head if the repair touched a crate, and release the same version
  again from step 9. The repair's changelog entries go in the `## <version>` section, which no
  crate has shipped yet.
- **Some crates were uploaded**: run `cargo publish -p <crate>` for each crate still missing, from
  the same tagged commit. If that cannot succeed without a change to the code, the uploaded crates
  keep the version, and the next release is the next patch version for all three, per
  `design@knowledge-architect@version-lockstep`; the uploaded version is yanked only if it is
  broken. The tag `v<version>` is pushed all the same, since crates.io carries that version, and
  the next release's `v<previous>` is it.
- **In either case**, main's README status line is wrong until a repair lands, through a branch and a
  pull request like any other change.
