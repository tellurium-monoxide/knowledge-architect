---
kind: observation
---
# `commits` assembles the whole tree of a commit whose manifest fails phase 1

## Summary

When a commit's manifest carries a phase-1 complaint, such as a table no registered extension
claims, `commits` still reads every blob of the tree, builds the model and the survey, and asks
git's ignore rules, before phase 1 stops the tree. Over 49 such commits of this repository's own
history, the run took 24.3 s, where the binary before the core and extension split took 0.30 s,
because that binary failed the manifest parse before reading anything.

## Details

### What

`commit_tree` in `path@knowledge@documentation/src/cli/history.rs` configures the extensions
against the commit's manifest, which records each extension complaint as a phase-1 complaint,
and only then assembles the model and runs `check::foundation`, which stops at phase 1. Before
the split, a table no extension claimed, or a missing `[rules]` table, failed `Manifest::parse`,
and the tree was `Unloadable` before any blob was read.

Measured by an adversarial review of the core and extension split, with the binary before it and
the one after, each run from its own worktree at the same commit, on one 22-core machine:

| range | before the split | after |
| ----- | ---------------- | ----- |
| `commits HEAD~60..HEAD~11` at that commit: 49 commits, each holding the retired `[lint]` table | 0.30 s | 24.3 s |
| `commits HEAD~11..HEAD`: 11 commits whose manifests load | 11.4 s, 11.9 s | 13.5 s, 12.7 s |

Reproduces: run `cargo knowledge commits <range>` over any range of commits made before the
`[lint]` table moved into `[rules]`.

### Why it matters

`commits` runs after each commit and in the gates over the branch's own range, where no commit
carries such a manifest, so the gates do not pay it. A session judging older history does.

The cheap repair conflicts with a recorded behaviour of
`design@knowledge@a-commit-message-is-a-document`: `commits` states that a tree stopped
before the last phase still serves as the next commit's parent, its entities read and not its
verdict. Returning early with no model would make a phase-1 tree serve as no parent, which is
what the binary before the split did for a manifest that failed to parse, and which differs from
what it did for a manifest whose phase 1 carried a core complaint.

### What would close it

Deciding whether a tree whose phase 1 fails should serve as a parent at all. If not,
`commit_tree` returns before reading the tree's blobs when the configured manifest carries a
complaint, and the 49-commit range is re-timed against the 0.30 s figure. If so, the cost stays,
and this entry closes by recording that decision at `design@knowledge@a-commit-message-is-a-document`.
