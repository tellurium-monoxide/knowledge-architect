---
kind: todo
---
# `commits` sees a citation of a branch commit by SHA only when the cited commit is in the range it judges

## Summary

Under `[commits] refuse-branch-shas = true`, `commits HEAD~1..HEAD` cannot see a message that cites
an earlier commit of the branch by SHA, because that commit is outside the range. The finding
appears first at the merge gate, `commits <main>..HEAD`, after the commit was pushed, and the repair
is then a history edit of pushed commits. This repository's instructions run the check after a
commit over the whole branch. The checker itself could judge citations against the branch whatever
range it is given.

## Details

### What

`commits` in `path@core@src/cli/history.rs` judges a citation against the SHAs of the commits of the
range it was asked for. A session that follows the usual after-commit check, `HEAD~1..HEAD`, passes
a message citing the SHA of the branch's previous commit. Reported by thaum's retrospective of
2026-10-04, on knowledge-architect 0.2.0, and not re-observed here: two review-record commits copied
a reviewed range, written with SHAs, into their messages. Both passed `HEAD~1..HEAD` and were
refused by the merge gate after the push.

The work: with the option on, judge SHA citations against the commits of `<main>..HEAD`, whatever
range was asked. `commits` does not know the name of the main branch, so the work needs a way to
learn it: a manifest field, or a command-line option. That choice is the design question this
entry leaves open. A smaller alternative is a note printed when the range judged is narrower than
the branch.

### Why it matters

`design@core@branch-shas-are-refused` exists so that the rule against such citations is kept by
running `commits`, per `goal@core@declared-instructions-are-checked`. A check that lets the
violation through at the moment a session runs it, and catches it only after the push, turns a cheap
amend into a history edit of a pushed branch. Until this is done, every project with the option on
must run the after-commit check over the whole branch, as this repository's root CLAUDE.md says.
That costs judging every commit of the branch after each commit.

### What would close it

`commits` refuses a citation of any commit of `<main>..HEAD` when the option is on, whatever range
it judges, with a test in `path@core@tests/binary.rs` that commits a message citing an earlier
branch commit and runs `commits HEAD~1..HEAD`. Then the instructions return to `HEAD~1..HEAD`
after a commit. Or a design session rules that the instruction alone is enough, and this entry
closes with that ruling.
