---
kind: todo
---
# A change to a generated file's contract fails every earlier commit with no word on the repair

## Summary

When a branch changes the bytes a generated index must hold, `commits` reports every earlier commit
of the branch as holding an out-of-date generated file, one finding each, and says nothing of the
repair the per-commit rule asks for.

## Details

### What

On the branch that built the plans layout, a review repair added a column to the milestones index.
`cargo klarch commits origin/main..HEAD` then reported, for each of the three earlier commits, that
docs/plans/milestones/index.md, at line 5, "the generated file is out of date", and nothing else.
Every commit of the range failing on the same generated path, at the same line, is the sign that the
branch changed the generator rather than that three commits each left an index stale. The repair
the finding names, `index`, cannot repair a commit already made.

### Why it matters

`design@core@a-commit-message-is-a-document` judges every commit's tree with the tip's checker,
so a change to a generated file's contract is a change that fails earlier trees, and
`design@knowledge-architect@git-flow` folds such a repair into the earliest commit by a history
edit. A
reader of the finding as it stands repairs nothing, or runs `index` on the tip, which does not
change the earlier commits; the repair that works is found only by reasoning from the pattern.

### What would close it

When two or more commits of a range fail on the same generated path, `commits` prints one summary
line naming the path and the repair: fold the regenerated file into the earliest failing commit.
On the owner's ruling, the line names no cause: the repair is the same whatever left the file out
of date. A test plants a range of three commits whose generator changes in the last, and asserts
the summary line.
