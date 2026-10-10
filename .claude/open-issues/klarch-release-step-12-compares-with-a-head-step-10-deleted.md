---
kind: question
---
# klarch-release step 12 compares with the reviewed branch head, and step 10 deletes every ref that names it

## Summary

Step 12 of `skill@klarch-release` tests that main's tree equals the tree of "the reviewed branch
head". Step 10 merges under the root CLAUDE.md's merge predicate, whose last lines delete the
remote branch and the local one, and no step records the head first. The head is recoverable from
the reflog or from the merged pull request, so a session can derive it; whether one does, or runs
the comparison against the wrong commit, is not established.

## Details

### What

- `skill@klarch-release`, step 10: merge under the root CLAUDE.md's merge predicate.
- `instructions@git-workflow`, point 5: `gh pr merge <branch> --rebase --delete-branch`, then
  `git branch -D <branch>`.
- `skill@klarch-release`, step 12: `test "$(git rev-parse HEAD^{tree})" = "$(git rev-parse <the
  reviewed branch head>^{tree})"`.

Reported by the L4 lens of the second pass of the first agentic-workflow audit, as a predicted gap.

### Why it matters

`design@knowledge-architect@publish-after-merge` publishes from main the tree that was reviewed;
the comparison is what shows it. Run against the wrong commit, such as main before the pull, it
passes vacuously.

### What would close it

The next release's session, read by its retrospective: it found the head and the comparison held,
and the entry closes; or it did not, and step 9 or 10 notes `git rev-parse HEAD` before the merge.
