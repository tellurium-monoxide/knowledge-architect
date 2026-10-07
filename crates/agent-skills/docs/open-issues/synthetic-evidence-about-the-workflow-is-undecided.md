---
kind: question
---
# Whether synthetic evidence about the workflow is built at all, and under which conditions, is undecided

## Summary

A design discussion produced a plan document whose acceptance criterion was a synthetic test of an
installed agent: a replay of past work in another project, whose result could change a bound or
reopen a decision. The owner ruled it out before it ran. Nothing in the record or in the installed
skills stopped it earlier: the goal and the design head on real use are worded for what originates
an edit, the design skill suggests synthetic artifacts as evidence, and no review of a plan
document reads it against the goals and the design heads. A design discussion is needed on whether
synthetic evidence about the workflow is built at all, and if so under which conditions.

## Details

### What

The instance. In the discussion that produced the spec of the search for standing entries, the agent
proposed a replay of the new search agent over thaum's history, first as evidence between two
shapes of a decision, then, at the premortem, as an acceptance criterion: the agent had to return
every entry thaum's reviews had found after the work. The criterion was written into the spec and
passed the spec's three reviews. At the step that ran it, the owner stopped the dispatch and ruled
it out: "I believe there is a design head that argues against using this kind of testing methods
for the workflow. From experience, they are not very reliable, and drive the workflow toward wrong
directions more often than good ones. This experience comes from the time when the design skill
was a standalone claude plugin."

The owner's reading of the cause: "I don't think your reading of the goal and design head was
wrong. They are simply not wide enough to cover your use case, that added to the design skill
suggesting synthetic evidence, thus you reaching this conclusion." And on the evidence itself:
"synthetic evidence in this case is not fully useless. But I'd rather test it in real conditions."

Four parts of the configuration let it through:

- **The goal's wording.** `goal@knowledge-architect@the-workflow-improves-through-real-use` says
  "Real sessions are the test of the workflow, not synthetic scenarios." It does not say whether a
  synthetic scenario may serve as an acceptance check, as evidence between two shapes, or as a
  check that a text's mechanics run.
- **The head's wording.** `design@agent-skills@additions-need-real-use` says "Published literature
  and synthetic scenarios originate no edit." The replay was proposed to measure a bound and to
  judge a decision, not to originate an instruction, so the sentence did not read as covering it.
- **The design skill's loop step 5**, "Build discriminating evidence when stalled", names "a
  failing test, a throwaway prototype, a benchmark, a mockup". The installed skill works in any
  project, and for a project whose subject is agent behaviour these are synthetic scenarios. It
  says nothing of a project whose goals rule them out.
- **The reviews of a plan document.** §8 of the installed planning skill sends the cold
  implementer, the code-claims reviewer and the transcript reviewer. None reads the document's
  decided shapes or its acceptance criteria against the goals and the design heads. The
  decision-record reviewer does, at the harvest, after the work. An acceptance criterion also
  needs no word of the owner, unlike a tripwire, so one the agent proposes at the premortem reaches
  a committed plan document without a ruling.

A second instance, in this repository. The milestone load-bearing-records, which rewrites the entry
tests of the decision-recording skill, gave its first slice an acceptance criterion: a fresh
subagent, given the rewritten tests and three decisions written for the check, had to classify them
as the owner had ruled, and a second failure would reopen the decision on the tests. The agent
proposed it while writing the slice's spec, after the premortem; no word of the owner admitted it.
It passed the milestone document's three reviews, whose cold implementer asked for its inputs to be
written out, and the branch's standing-state and last transcript reviews. The slice's design audit
found it, because the standing-entry search returned this entry. The owner dropped it before it
ran: "Agreed, drop the criterion based on synthetic evidence. And record this happening in the
existing issue too." The parts that let the first instance through let this one through: an
acceptance criterion needs no word of the owner, and no review of a plan document reads it against
the goals and this entry. The difference is where it stopped: at the audit, before the work,
rather than at the step that ran it.

### Why it matters

A synthetic result that changes a bound or reopens a decision drives the workflow by evidence the
owner judges unreliable, against `goal@knowledge-architect@the-workflow-improves-through-real-use`.
A plan document that contradicts a goal and is caught only when its work runs threatens
`goal@knowledge-architect@agents-work-without-drift`. The head
`design@agent-skills@additions-need-real-use` is strained: its boundary covers the origin of an
edit, and the instance shows evidence used for other purposes.

### What would close it

A design discussion under `knowledge-architect-design` that settles:

- whether synthetic evidence about the workflow is built at all, and if so under which conditions:
  as evidence between shapes, as an acceptance check, as a check that a text runs as written;
- the wording of the goal, under `knowledge-architect-goal-setting`, and of the head, so that both
  cover what the discussion decides;
- what the design skill's loop step 5 says of evidence in a project whose goals restrict it;
- which review reads a plan document against the goals and the design heads, and whether an
  acceptance criterion needs the owner's word.

The converged design is recorded and built.
