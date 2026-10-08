---
kind: defect
---
# A backticked span with an empty head and a slash is reported as malformed, which the candidate rule says is silent

## Summary

The head `design@core@candidate-rule-and-retired-forms` says a backticked `@` span is a reference
candidate when its head before the first `@` is a kind or an anchor, and that "every other `@` span
is silent". The code reports a span whose head is empty and whose rest holds a `/` as a malformed
reference. The head and the code diverge; which one is the defect is not established. Below, such a span is
written without its backticks, since written with them it is the finding this entry records.

## Details

### What

In `path@core@src/entity.rs`, the function `candidate` returns
`Candidate::Malformed { why: "the kind segment is empty" }` when `head.is_empty() &&
rest.contains('/')`. Its comment gives the reason: "An empty head in front of a path shape is the
retired `@` escape, or a kind that was never typed; either way a pointer that would otherwise leave
every check. A slashless `@word` is an annotation or a handle and stays silent."

The head's title and body name two retired forms, `<anchor>@<path>` with no kind and a backticked
`<word>#<id>` or `#<id>`, and no case of an empty head. The primer's import line,
@.claude/knowledge-architect/PRIMER.md, written in backticks, is therefore silent by the head and
malformed by the code.

Reproduction: a Markdown file in the walk holding the import line above, in backticks, makes
`cargo klarch check` report "is malformed: the
kind segment is empty". The setup skill showed exactly that span until its text was taken into the
walk; it now writes the placeholder `{{primer-import}}`, which the agent-skills build fills, so the
shipped text no longer meets the case.

Not established: whether the empty-head case was a decision taken with the retired `@` escape and
never written into the head, or code that outlived the escape. `git log -S'the kind segment is
empty' -- crates/core/src/entity.rs` names only the change that restructured the imported checker
into this workspace, so the case came from the project the checker was imported from.

### Why it matters

`design@agent-skills@design-home-is-built-intent` makes the head authority over the code, and the
root CLAUDE.md restates the rule as "Any other span holding an `@` … is silent". A writer who follows
the head meets a finding the head says cannot occur, and repairs it by moving the pointer out of the
checker's reach. The agent-skills build carries a delivery substitution for the import line only
because of this behaviour.

### What would close it

The head names the empty-head case and its reason, if the behaviour is intended; or the code makes
the span silent, if it is not, with a test over a document holding the import line in backticks.
Either way, the substitution `{{primer-import}}` in `path@agent-skills@build.rs` is judged again:
it stays if the span stays a finding.
