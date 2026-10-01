# Tripwires — xtask

Evidence that would flip a decision recorded in `path@xtask@docs/design.md`. An entry leaves when
it fires or when the decision it guards is gone; what to do when one fires is
`tracking-open-issues`.

---

## Guarding `design@xtask@ci-builds-the-tip-alone` `##tip-alone-is-built`

**Fires when:** a `git bisect` over `main` stops on a commit that does not compile, among the
commits from the restructure commit 34b757a onwards. The commits before it are the checker's
history imported from thaum, which build in no workspace here, so a bisect reaching them proves
nothing about this decision.
**Response:** reopen `design@xtask@ci-builds-the-tip-alone`, with a
build-only pass over each commit of the range origin/main..HEAD as the candidate, without the
test suite.
**Re-entry:** the bisect itself, and the standing-state review on every dispatched review.
