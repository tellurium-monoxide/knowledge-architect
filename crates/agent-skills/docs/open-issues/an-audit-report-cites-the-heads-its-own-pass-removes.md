---
kind: defect
---
# An audit's report file cites the heads its own pass removes, and the session turned those references into plain text

## Summary

On the first pass of the design-record axis through its saved workflow on this repository, the
pass's report file was committed with its rulings, then the pass removed and renamed heads that the
file cites. The check then reported the file's references as resolving to nothing. The session
repaired them by deleting their backticks, so each still named its target in the checker's syntax,
as plain text. That is the instance `tripwire@agent-skills@plain-text-pointer-found` names. The
audit skill asks that a span the checker would refuse be "quoted in words", and says nothing of a
span the pass's own edits make refusable.

## Details

### What

The commit "[audit] Pass 1 of the design-record axis applied: its edits in the branch, F1 to F18
and F21 carried out, two issues opened" changed about twenty references of the report file, for
example the owner-list item on the removed head klarch-prefix, from a backticked span to the same
text without backticks. Its message calls the result "quoted in words". The standing-state review
before the merge found it. The file left the tree at the harvest, so no plain-text pointer stays at
the tip; history holds the instance.

`skill@knowledge-architect-project-audit@record-of-an-audit`: "A span the checker would refuse is
quoted in words, since the file is checked like any other." A report file stays in the tree from
the commit of its rulings until the commit that carries out its last outcome, and every edit the
pass applies in between can remove or rename a head the file cites.

### Why it matters

`design@agent-skills@plain-text-is-no-repair` refuses a pointer moved into plain text, since no
check reaches it. The audit's own report file is the one text that is sure to cite every head the
pass removes, so the instance recurs at every pass that removes or renames a head, unless the
installed text says what the file does with those spans.

### What would close it

The audit skill states how a report file treats a reference that its own pass's edits make
dangle: a description in words of the head, as the skill's sentence means, or another form the
checker judges. Or the owner rules that a file the branch deletes needs no repair of that kind.
