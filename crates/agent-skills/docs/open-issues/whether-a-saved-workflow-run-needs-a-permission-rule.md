---
kind: question
---
# Whether a session in Claude Code's default permission mode is asked for permission, or needs an allow rule, to run an installed saved workflow

## Summary

The agentic-workflow axis runs each pass as one run of the saved workflow the installer writes to
`path@agent-config@workflows/`. Every run of this repository's first agentic-workflow audit ran under a
permission mode that asked nothing. Whether a session in the default mode is prompted for the run,
or needs an allow rule naming the workflow before it can run it, is not established, and no
installed text says either.

## Details

### What

When the saved workflow was proposed as the axis's delivery, the session stated to the owner, among
its consequences: "Permission prompt (not established). Here the probes ran with no prompt, under
auto mode. Whether a default-mode session gets a prompt or needs a `Workflow(<name>)` allow rule is
not established." The owner approved the delivery on that statement. No later run tested it: every
pass of the audit ran in the same mode, and so did the first pass of the design-record axis through
its own saved workflow, `knowledge-architect-design-record-audit`, which meets the same question.
The installed audit skill and the setup skill name no permission for the workflow.

### Why it matters

`design@agent-skills@audit-axis-runs-a-workflow` puts every step of an axis in its saved workflow.
In a project whose sessions run in the default mode, a run that waits on a prompt nobody answers, or
that is refused for want of a rule, stops the pass at its first step; the setup skill would then owe
the rule to a project that adopts the workflow.

### What would close it

One run of the installed saved workflow, by name, from a session in Claude Code's default permission
mode with no allow rule naming it, observing whether the harness prompts, refuses or runs it. This
is a run of the harness's mechanics, which `design@agent-skills@synthetic-evidence-not-built`
admits. If it prompts or refuses, the setup skill states the rule and this entry closes with it.
