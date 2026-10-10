---
kind: deferred
---
# An audit's upstream file is written when the audit stops, and nothing keeps an earlier pass's findings on the installed text until then

## Summary

In an adopting project, every finding of the agentic-workflow axis on the installed text goes into
one upstream file, written when the audit stops. A pass's findings and their evidence live in its
scratch files and its report. The record of an audit is the commit messages, and nothing tells the
session to carry those findings into a message or a file that lasts until the stop. A later
session that stops the audit can lose them.

## Details

### What

- `skill@knowledge-architect-project-audit@agentic-workflow-axis`, "Sent upstream": "every finding
  on the installed text, over every pass of the audit, a predicted gap included, goes into one file
  per audit, written when the audit stops."
- `skill@knowledge-architect-project-audit@record-of-an-audit`: the commit messages are the record;
  "A finding whose only record is a scratch file, a run's report, or a plan document that will
  leave, is lost when that file goes."
- A pass whose findings are all on the installed text changes no file in an adopting project, so it
  leaves no commit.

Reported by the L4 lens of the fifth pass of the agentic-workflow audit, as a provable gap. The
repair proposed then had each pass add its findings to the upstream file. This repository writes
no upstream file, per `instructions@repository-skills`, so it never meets the case.

### Why it matters

The upstream file is how `goal@knowledge-architect@the-workflow-improves-through-real-use` reaches
an adopting project's audit findings. A finding lost before the stop never reaches the workflow.

### Trigger

`issue@agent-skills@an-audit-s-provable-gap-on-installed-text-against-real-use` closes in favour of
editing on a provable gap, or a retrospective of an adopting project's audit shows a pass's
installed-text findings lost before the stop. Either occasion decides how the gap is filled.
