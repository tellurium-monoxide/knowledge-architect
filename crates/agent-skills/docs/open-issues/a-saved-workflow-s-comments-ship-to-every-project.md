---
kind: defect
---
# A saved workflow's `//` comments ship to every installing project, though the standard sends maintainer-only text to a comment the build removes

## Summary

`skill@knowledge-architect-agent-configuration@admission-standard` says a statement whose only
reader is the text's maintainer is, in installed text, "a comment of the workflow's source,
removed by the build". The build removes only `%%` lines. A saved workflow's JavaScript comments,
written with `//`, are not removed, so every installing project receives them.

## Details

### What

- The build's comment pass, `path@agent-skills@src/render.rs`, removes a line whose first two
  characters are `%%`, and nothing else: its constant `COMMENT` is `"%%"`.
- `path@agent-skills@content/workflows/design-record-audit.js` holds `//` comment lines written
  for its maintainers, such as the one above the judgement of an application and the `args`
  comment block at its head. The installed copy,
  `path@agent-config@workflows/knowledge-architect-design-record-audit.js`, holds them too, read on
  the branch of the intake of seven retrospective files.
- The agentic-workflow axis's saved workflow carries `//` comments as well; not counted here.

Found by the agent-facing-text axis of the review of that intake, as an item outside its axis: it
judged the comments of that diff no finding, since code comments are not prompts, and named the
disagreement between the standard and the build.

Not established: whether every `//` comment of the two workflows is maintainer-only, or whether
some serve the agent that reads a workflow to dispatch its steps by hand, which
`skill@knowledge-architect-project-audit@audit-method` asks of a session whose harness does not
list the workflow.

### Why it matters

`design@agent-skills@need-test` keeps maintainer-only text out of what an installing project
receives. A rule whose stated mechanism does not exist for one kind of installed file is a
contradiction a writer of that file cannot satisfy, against
`goal@knowledge-architect@agents-get-a-complete-workflow`.

### What would close it

Either the maintainer-only comments of the saved workflows become `%%` lines, which the build
removes even inside a template literal per `path@agent-skills@CLAUDE.md`, and a comment a session
dispatching by hand needs stays as `//`; or the standard says what a saved workflow's comments
are, and the build's behaviour is stated with it.
