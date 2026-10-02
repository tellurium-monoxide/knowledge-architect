---
name: klarch-release
description: MUST use when the owner asks to publish a version of knowledge-architect to crates.io. Covers choosing the version, the changelog section, the checks of the shipped text before publishing, the package lists, the dry run, the owner's word at the moment of publishing, the publish itself and the tag.
---

# Release

Scope: publishing one version of the three published crates, `knowledge-architect`,
`knowledge-architect-agent-skills` and `knowledge-architect-gates`, from a commit on a release
branch. xtask is never published. The versioning policy is
`design@knowledge-architect@versioning-policy`, and all three crates share one version, per
`design@knowledge-architect@version-lockstep`.

**A merge to main publishes nothing.** Any number of merges land between two releases.

## The procedure

On a branch of its own, like any other work:

1. **The version.** Choose it under the versioning policy. Set it in `[workspace.package]` of the
   root Cargo.toml, and in the exact pin of `knowledge-architect-agent-skills` in
   crates/core/Cargo.toml. Rename the changelog's working section to the version, and check that
   every change since the last release has an item, tagged with its surface.
2. **The status lines.** The root CLAUDE.md's release status and the root README's status state
   the version.
3. **The shipped text holds no live reference.** Every backticked span under
   crates/agent-skills/content/ that holds an `@` carries a placeholder in angle brackets, so it
   is not a reference, per `design@agent-skills@shipped-text-is-reference-free`. Until
   `issue@agent-skills@shipped-text-is-reference-free-mechanically` closes, this is checked by
   hand: list the spans, and read each one.

   ```sh
   grep -rnoE '`[^`]*@[^`]*`' crates/agent-skills/content
   ```
4. **Every name the shipped text uses is shipped.** Every `knowledge-architect-<name>` in
   crates/agent-skills/content/ is a shipped skill or agent, or one of the three published crates,
   per `design@agent-skills@no-external-handoff`. Every document the shipped text relies on in a
   project, such as the knowledge table, is written by an installed skill.
5. **The names on crates.io.** For a first release of a crate, the crates.io API answers 404 for
   its name. Otherwise it shows the crate as the owner's.
6. **The package lists.** `cargo package --list -p <crate>` for each published crate. The owner
   reads the lists.
7. **The dry run.** `cargo publish --workspace --dry-run` passes, skips xtask, and orders
   agent-skills before the core, which depends on it.
8. **The gates.** Commit the release, then run `cargo x gates --require-rebased`, and the dry run
   again on that commit.
9. **Review and merge, before publishing.** The branch is reviewed, repaired, and merged to main
   like any other, after CI passes on its head. Publishing comes after, so what is published is
   what review and CI judged, and the tag names a commit on main. Until the publish runs, main's
   status lines say a version that is not yet on crates.io; if the publish fails, a commit on main
   repairs them.
10. **The owner's word, given at that moment.** Publishing is irreversible: a version can be
    yanked, never deleted. Without the word, nothing is published.
11. **Tag and publish.** The owner logs in with `cargo login`; the token never passes through the
    agent. On main's head, with a clean tree: the tag `v<version>`, then `cargo publish
    --workspace`, then, once the publish succeeded, `git push origin v<version>`.
