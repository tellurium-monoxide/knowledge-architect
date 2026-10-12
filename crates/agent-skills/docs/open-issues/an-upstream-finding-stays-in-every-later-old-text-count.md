---
kind: defect
---
# In an adopting project, a finding sent upstream is counted again as a violation left on old text at every later pass

## Summary

In a project that installs the workflow, a finding of the agentic-workflow axis on the installed
text is sent upstream and is not edited there. The installed text stays the same until a new
version ships, so every later pass of the same audit can find it again, on text that is still old.
Point 4 of `skill@knowledge-architect-project-audit@audit-method` counts it as a violation left on
old text, since it is on no kept list and no issue entry records it. The count on old text then
cannot fall below the number of such findings, however well the project's own repairs converge.

## Details

### What

Established by reading the installed project-audit skill, not observed in a run: this repository
edits the installed text at its source, so no pass here sends a finding upstream.

The chain, in the installed `skill@knowledge-architect-project-audit`:

- `skill@knowledge-architect-project-audit@outcomes`: the row "sent upstream" takes "a finding on
  the installed text that the row "nothing" does not name; the project never edits the installed
  text". The row "nothing" names "a finding an issue entry already records; a finding on the kept
  list".
- `skill@knowledge-architect-project-audit@agentic-workflow-axis`, its paragraph "Sent upstream":
  every such finding goes into one file per audit, "written when the audit stops". Nothing reaches
  the upstream project, and nothing changes in the installed text, while the audit runs.
- `skill@knowledge-architect-project-audit@audit-method`, point 4: "A finding of the re-check that
  a reading confirms, that is not on the kept list, and that no issue entry records, is a violation
  left". The kept list holds "the entries, the texts and the rules the owner ruled to keep"; a
  finding sent upstream is not ruled kept, and its outcome is not an issue entry. Old text is "text
  the previous pass also read, whose words no commit changed after the commit that pass audited".
  The installed text's words do not change during the audit, so a finding on it is on old text.
- The same point: "The count on old text is expected to fall from one pass to the next; a pass
  where it does not goes to the owner as a question".

So with k findings sent upstream at pass n that the re-check of pass n + 1 confirms again, the
count on old text at pass n + 1 is at least k. A pass whose own repairs left nothing on the
project's text can still fail to fall, and goes to the owner as a question that the project could
not have answered with a repair. Whether a re-check re-finds every upstream finding is not
established: its agents are fresh, so a finding is re-found with some probability, not always.

The reproduction, for an adopting project that installs this workflow: run the agentic-workflow
axis on its corpus until one pass sends a finding upstream; the next pass's re-check, given the
kept list point 4 names, confirms the same finding on the same installed words, and the pass's
commit message counts it on old text.

### Why it matters

`design@agent-skills@audit-passes-repeat-while-the-count-falls` makes the fall of the count on old
text the convergence signal that decides whether a pass follows, and
`design@agent-skills@audit-outcomes` routes a finding on the installed text upstream, never to an
edit in the project. Together they let a pass fail to converge on findings the project cannot
repair, so the owner is asked about a non-convergence that the method itself produced, and may stop
an audit that was converging on the project's own text.

### What would close it

A decided way to count a finding sent upstream at later passes, recorded in the design homes and
in the installed skill: either a finding sent upstream joins the kept list given to every later
re-check, so it takes the outcome "nothing", or it is counted apart from the violations left on
old text, as the findings on new text are. Its test: a reading of point 4 and of the axis's section
under which a finding sent upstream at one pass cannot enter the old-text count of the next.
