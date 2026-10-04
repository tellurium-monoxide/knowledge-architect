---
name: klarch-development
description: MUST use before starting or iterating on any change to this repository's Rust source — the crates under crates/ and the maintenance tool under tools/ — covering grounding in the Component's own documents, specifying a unit of work as claims with tests that discriminate, showing a test discriminates, the gate, when a piece of code is ready for review and which axes to send, and which comment species a sentence belongs in. Not for the installed text under crates/agent-skills/content/, the CI workflow, or the agent configuration.
---

# Development

Scope: writing this repository's Rust source, from grounding to the commit: the crates under
`path@knowledge-architect@crates/`, and the maintenance tool under `path@knowledge-architect@tools/`.
One activity, and the facets below are its parts rather than separate scopes. Where a section reads
_the crate_, read _the crate or the tool_: all of them are workspace members, so §4's gate sees all
of them.

**Not the installed text, the CI workflow, or the agent configuration.** §4's gate compiles and
tests Rust. It passes over a workflow file without seeing it, so a session that follows this skill
for one of those gets a green gate on an unverified change.

- The installed skills, agents and primer under `path@agent-skills@content/` follow
  `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", and
  `knowledge-architect-agent-configuration`.
- This repository's own agent configuration is `knowledge-architect-agent-configuration`.
- A release is `klarch-release`.

Not covered here either: **reviewing** (`knowledge-architect-review`), **recording a decision**
(`knowledge-architect-decision-recording`), and **parking anything**
(`knowledge-architect-issue-tracking`).

## 0. Ground before editing

Read the Component's own files first:
- its `CLAUDE.md`, for the invariants and traps that hold of the code as it stands;
- its design home, for the recorded intent;
- its `path@*@docs/rejected-alternatives.md`, for what has already lost;
- its `path@*@docs/open-issues/` and `path@*@docs/tripwires.md`, for what is outstanding and what
  would reopen a decision.

A behaviour that looks like a new defect is often recorded: `cargo klarch issues` and
`cargo klarch tripwires` list every entry. `path@knowledge-architect@docs/design.md` at the root
holds only what binds every Component, so read it when the change crosses one.

The primer's intent-and-claims rule governs how to read all of them. **Intent is authority, and the
code is checked against it. A claim about the code as it stands goes stale, and is checked.** A
divergence between a design document and the code is a defect in one of them. Say which, and open
an entry in that Component's `path@*@docs/open-issues/`. It is not licence to follow the code.

## 1. The loop

1. **Claims.** Take the unit of work's claims, each with the test that could refute it.
2. **Write the tests, and show that they discriminate** (§2).
3. **Implement.**
4. **Gate** (§4).
5. **Commit.** The commit contract is root `CLAUDE.md`, section Git. A decision that clears the
   recording threshold takes `knowledge-architect-decision-recording` as well.
6. **Review** (§3), at any checkpoint where a coherent piece works, not only at the end. It follows
   the commit because a reviewer working on its own copy of the tree sees committed content only,
   so uncommitted work is reviewed by nobody. A repair the review asks for is a further commit,
   unless it would leave an earlier commit failing under the branch tip's checker: it is then
   folded into the earliest commit it repairs, per root `CLAUDE.md`, section Git, point 2.

## 2. Claims, and tests that discriminate

**A unit of work is a list of claims, each with the test that could refute it.** A part with no
claim is cost with no information. Where the work has a spec or a milestone, its steps present the
claims, under `knowledge-architect-planning`; this procedure does not depend on one existing.

**Every claim's test must be shown to fail against a plausible wrong implementation.** A test
written first can still be written to the implementation its author already has in mind. It then
passes on its first run, and nothing was ever at risk. Two ways to show it, and the second is not a
lesser one:

- **Observed failing before its code exists.** Available when the answer is not yet known.
- **A recorded mutation check.** When the trap is known before either the test or the code is
  written, both come out right on the first run. Introducing the wrong implementation on purpose to
  watch a red bar is theatre. Changing the code afterwards, running the test, and recording whether
  it caught the change is the same evidence, obtained honestly. **Name the mutation, not just the
  outcome.**

**A mutation runs in a scratch worktree by default.** Reverting a hand-made mutation in the live
tree with a restore has lost uncommitted work in thaum several times. Where the live tree is used
anyway, the file is staged before the mutation, per root `CLAUDE.md`, section Git, so the revert
does not depend on a restore. The worktree starts at HEAD, and the session's uncommitted work is
brought into it, because the test under check is usually not committed yet. From the main tree's
root, with `<name>` unique to the session so that no two sessions share a worktree:

```sh
git add --intent-to-add <each new file>              # new files then appear in the diff; nothing is lost
git worktree add --detach .claude/worktrees/<name> HEAD
git diff HEAD --binary | git -C .claude/worktrees/<name> apply
git -C .claude/worktrees/<name> commit -qam "scratch: the work under check"
```

Then, inside the worktree, run the commands below with `export CARGO_TARGET_DIR="$PWD/target"`.
Every build here is tied to its checkout
(`design@knowledge-architect@a-build-is-tied-to-its-checkout`), so a build directory shared with
the main tree would not run the other's build, but each checkout would rebuild at every switch.
`<package>` is the package name, such as
`knowledge-architect-gates`, not the Component's:

```sh
cargo test -p <package>                # the baseline: it passes over the unmutated work
# apply the mutation, then
git diff                               # shows the mutation alone: this is what the record quotes
cargo test -p <package> --no-run       # it compiles, or it proves nothing
cargo test -p <package>                # nonzero: caught; zero: SURVIVED
```

**A mutation of the cargo configuration runs in a worktree outside the main tree**, such as one
under the session's scratchpad directory. Cargo merges the configuration of every parent directory
field by field, so in a nested worktree the main tree's `path@knowledge-architect@.cargo/config.toml` restores a field the
mutation removed, and the mutant survives a test that would catch it.

Finally, from the main tree's root, `git worktree remove --force .claude/worktrees/<name>`. The
force is needed because the worktree holds the mutation, and it removes that worktree alone.

Each of the three test runs carries a claim, so leaving one out weakens the check rather than
shortening it.
- **The baseline:** over a red base, every mutation reads as caught. It must run with the work
  under check, or it passes over a tree that lacks the new test.
- **The compile:** a mutant that does not compile also reads as caught, and the sites this check
  most often sends you to are the ones that do not compile.
- **The test run:** its verdict is the result.

**A recorded check that does not show the mutation applied establishes nothing.** It quotes the
diff `git diff` printed, the file and line it changed, and the verdict.

The property being bought is that **the test discriminates**. Watching it fail first is one way of
learning that. A claim already settled by a probe beforehand enters as a regression guard, and is
recorded as such.

### Unreachable states, and which of them earn a test

A guard, a refusal or an `expect` often stands against a state nothing can currently produce.
**Three kinds, and only the third owes a test.**

- **The language or a format makes it unreachable**, and stays so for the life of the project: a
  type that cannot hold the value, or a format the parser guarantees. No test: the state would have
  to be constructed, and the result asserts something about the construction rather than about
  the code. **Name the guarantee.** A justification phrased as a property of the current code,
  rather than of the language or the format, makes it the second or third kind.
- **A validated boundary makes it unreachable.** Whatever checks untrusted input refuses every state
  that would reach the code, such as the manifest's validation of a declaration. There is no state
  to construct and run, so what is tested is **the refusal**, and the mutation is deleting it.
- **Only the current implementation makes it unreachable.** It stops being unreachable at whichever
  unit of work populates that region, and anything shipped under it has no coverage at the moment it
  starts mattering. **Test it**, constructing the state where the boundary accepts one.

**The operational test is to ask the boundary.** If it refuses every state that reaches the code,
the guard is the second kind, and the refusal is what carries the claim. If it accepts such a
state, the guard is the third kind, whatever an argument about normal use says.

**A mutation set chosen by the author of the tests is not enough.** The author mutates _the thing
the claim is about_, and the survivors are adjacent to it: in thaum, two units of work passed their
own mutation checks and an independent review found survivors in both within one pass. So the
adversarial review of §3 chooses its own mutations.

## 3. The review axes for code

**Dispatch when a coherent piece of code compiles, passes its tests, and does what its claims say**,
and a defect found after the next piece is built on it would mean unbuilding both. That is the
moment in this activity's terms; the list of such moments is open. Merging to main
is not on this list, because it belongs to every activity, and root `CLAUDE.md`, section Git,
carries it.

**How to dispatch is `knowledge-architect-review`**: the invariants that make a finding worth acting
on, how to write a brief, and where findings land. Read it before sending anything.

The axis this activity adds:

| axis | what it does | when |
| --- | --- | --- |
| adversarial | chooses its own mutations, in its own worktree, and tries to reach a panic, a wrong verdict of the check, or a false finding from an accepted input | for any change to the code |

Several adversarial reviewers may be dispatched, with different angles of attack.

## 4. The gate

**`cargo x gates` runs every gate in one command**: fmt, `cargo klarch check`,
`cargo klarch commits origin/main..HEAD`, clippy with `-D warnings`, and the tests. It gives one
trustworthy exit code, a distilled report, and full logs under `path@knowledge-architect@target/gates/`.
Its usage is `path@xtask@README.md`, and this is a restatement of root `CLAUDE.md`, section Verify
mechanically. Do not filter its output through pipes. Before a merge, run
`cargo x gates --require-rebased`.

**Every commit of a branch must pass `commits` under the checker built from the working tree**, not
only the branch's tip: its message and its tree. Run `cargo klarch check` before each commit, and
`cargo klarch commits origin/main..HEAD` after it: the whole branch, since a citation of an earlier
commit of the branch by SHA is refused only when that commit is in the range judged. Amend the
commit if either fails, with a clean tree; once later commits sit on top, the repair is a history
edit. **A change to the core that makes a check stricter, or changes the manifest format, makes
every earlier commit of its branch fail.** Put that change in the branch's first commit, with every
fix the tree needs to pass it, or squash the branch to one commit before its review. The decision is
`design@core@a-commit-message-is-a-document`. Run both checks bare, and never chain a command on a
verdict that went through a pipe.

**No test may run the gates over this repository**, since the test gate would run the suite that is
running it: `path@gates@CLAUDE.md` and `path@xtask@CLAUDE.md`.

## 5. Incompleteness is recorded, never encoded

Every unit of work is wrong about everything later work builds. A guard, a refusal or a special case
added to mark work as unbuilt singles out one incompleteness among many and gives it a mechanism.
Within a single session that mechanism is indistinguishable from a decision, and it is _harder_ to
remove than a note: deleting a refusal looks like relaxing a guarantee, so the session that
finishes the work needs an argument it does not have.

Unbuilt work belongs in a plan document or in the issue registers, whose entries leave. Code says
what is true. A statement in code that is false, such as a doc comment claiming a capability the
crate lacks, is corrected, which is a different thing from enforcing the lack.

## 6. Comments: two species, two audiences

Neither carries the history of a change: that lives in the commit message.

- **A doc comment addresses the caller.** What someone using the type or function must respect: the
  invariants it maintains, what it expects of its arguments and of the object's state, what it does
  not do, and the pitfalls of using it. It does not explain the implementation: a reader who never
  opens the body must get everything they need from it. For a published crate's library API, the
  crate-level documentation of its lib.rs is the description docs.rs renders.
- **An inline comment addresses the next developer of this code.** What the code is supposed to do
  and why it is shaped that way, so the original intent can be recovered when it later turns out to
  be wrong. **Give the domain of validity explicitly**: the cases the code is expected to handle,
  and why it holds for them. Write the current intent, not the episode. A past-tense reference
  earns its place only where a reader would otherwise re-introduce the defect, and then it names
  the mechanism, not the fix. **A guard, a workaround, a
  stub or a test that exists because of an open issue names it**, `issue@<anchor>@<id>`, so that
  closing the entry dangles the comment, and `cargo klarch check` sends the closing session here to
  remove what the entry justified, per `design@knowledge-architect@a-reference-claims-a-revisit`. A
  comment is prose, so the reference is checked. A string literal bound to a name is not; in the
  checker's own source, every string literal is data, bound or not, per
  `design@core@checker-source-literals-are-data`.

**Where a comment ends and a document begins** is `knowledge-architect-decision-recording`: if
changing a piece of code would force a change to a document, it is design; if the document would be
unaffected, it is a comment.
