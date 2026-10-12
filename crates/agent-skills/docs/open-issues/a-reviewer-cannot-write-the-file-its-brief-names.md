---
kind: observation
---
# A reviewer subagent refused a write to the scratch file its brief named, and returned its findings as text

## Summary

At the review before the merge of this repository's agentic-workflow axis branch, the three
reviewers dispatched as general-purpose subagents reported that the harness refused their write of
the findings file their brief named in their scratch directory, with the message "Subagents should
return findings as text", and returned their findings in their final message instead. The review
skill tells a dispatcher to name a scratch directory for each reviewer, and the reviewers' briefs
asked for a file there.

## Details

### What

The refusals were reported by the spec-conformity, self-consistency and fidelity-of-relocation
reviewers, and by the helpers the last of them dispatched. The routing, decision-record and
standing-state reviewers, each dispatched as its installed agent, wrote their report files, and so
did the general-purpose agents that applied the audit's repairs earlier in the same session. So the
refusal is not established as a property of every subagent: what decides it, the agent type, the
wording of the brief or the harness version, is not established. Not reproduced on purpose.

The same split recurred at the review before the merge of the branch that gave the design-record
axis its saved workflow. The spec-conformity, self-consistency and fidelity-of-relocation
reviewers, dispatched as general-purpose subagents, each reported the refusal. The routing,
decision-record and standing-state reviewers, each dispatched as its installed agent, wrote their
report files. In the same session, the general-purpose agents of that workflow's run wrote their
files, and so did the general-purpose agents that applied the pass. Two observations now pair the
refusal with a general-purpose reviewer, and none with an installed agent.

### Why it matters

`skill@knowledge-architect-review@review-invariants` has the dispatcher name a scratch directory for
each subagent, and the project-audit skill's drafts and reports live in files the session reads
rather than in a summary, per `design@agent-skills@audit-method`. A reviewer whose report comes back
only as a message leaves the session no file to read again, and a long report can be cut short in
the message.

### What would close it

One dispatch of a general-purpose subagent briefed to write one file in a named scratch directory,
in the harness version then current, observing whether the write is refused. If it is, the review
skill states that a brief asks for the report as text, or names the agent types that write files;
if it is not, this entry closes as not reproduced.
