---
kind: observation
---
# A lint written twice on one line of a message is reported once or twice, by which tree holds it

## Summary

`commits` reports a message's lint once per occurrence when the commit's own tree raises it, and
once per line when only the parent tree raises it. Nothing decides which of the two is meant.

## Details

### What

`judge_message` in `path@core@src/cli/history.rs` judges a message's lints against the commit's
tree and against its first parent's, and unites the two lists. The union drops a parent's lint
equal to one already in the list, and that list grows with each parent lint pushed. So:

- two equal lints from the commit's own tree are both kept;
- two equal lints from the parent tree alone are collapsed to one.

Two findings are equal when their file, line, text and repair are equal, so two occurrences of one
span on one line are equal.

Reproduced on a release build of the checker, from a clean checkout:

1. `p1` = a commit made with `git commit-tree` from HEAD's tree plus a file `zzscratch/a.md`;
2. `c` = a commit of HEAD's tree with parent `p1` and a message of four lines: a subject, a blank
   line, a line naming `zzscratch/a.md` twice, each in its own backticks, and a line naming the
   root's goals home, `path@knowledge-architect@docs/goals.md`, twice as a bare path, each in its
   own backticks;
3. `klarch commits p1..c`.

It prints one finding on line 3, where only the parent holds the file, and two on line 4, where
both trees hold it: `FAILED: 3 findings above`. The behaviour predates the change that made the
union a set, which kept it exactly. No test pins either arm: making the union keep the parent's
own duplicates passes every test.

Whether one finding per occurrence or one per line is wanted is not established.

### Why it matters

`design@core@a-commit-message-is-a-document` says a message's lints stand where either tree holds
what they name. The count a reader sees for one defect then depends on which tree holds the target,
which no reader can predict. The consequence is cosmetic, a count off by the number of repeats,
and no verdict changes.

### What would close it

A ruling on whether a repeated span on one line is one finding or one per occurrence, the union and
the per-tree judgement made to agree with it, and a case in `a_lint_both_trees_raise_is_reported_once`
in `path@core@tests/binary.rs` pinning the chosen count for a span only the parent tree holds.
