---
kind: defect
---
# The hook reads the commit being replaced as the parent under `git commit --amend`

## Summary

`commit_message` in `path@knowledge@src/history_cmd.rs` resolves what the index refuses against
HEAD, as the parent of the commit being written. Under `git commit --amend` HEAD is the commit
being replaced, not the parent of the new one. A message naming an entry the amended commit
deletes is refused by the hook, and `commits` accepts the same message once the commit exists.

## Details

### What

Reproduction, in a clone whose hook is installed, over a tree holding a design heading with the
slug `probe-two` in some Component `<c>`:

```sh
# delete the heading `##probe-two`, stage it, then:
git commit -m 'Removes `design@<c>@probe-two`'            # passes: HEAD still defines it
git commit --amend -m 'Removes `design@<c>@probe-two`'    # refused: "defines no design probe-two"
cargo knowledge commits HEAD~1..HEAD                       # PASSED: the first parent defines it
```

Observed by the adversarial review of the branch that moved the hook onto the index. The same
fallback existed before that branch, so the defect is not introduced by it. What git tells a
`commit-msg` hook about an amend is `not established`: the hook receives only the message file.

### Why it matters

The hook refuses a message that `design@knowledge@a-commit-message-is-a-document` says is
valid, and the only way through is `--no-verify`, which disables the message check whole. A
session amending the commit that closes an issue meets it.

### What would close it

The hook resolving against the parent of the commit being written under an amend, with a test
committing and amending such a message. Or a recorded reason why it cannot, with the refusal
naming `--no-verify` as the way through for this case.
