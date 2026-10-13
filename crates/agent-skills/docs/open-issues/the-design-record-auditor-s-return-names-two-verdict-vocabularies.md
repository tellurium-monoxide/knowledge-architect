---
kind: question
---
# The design-record auditor's return names two verdict vocabularies

## Summary

`agent@knowledge-architect-design-record-auditor@what-to-return` says "Write `return.md` in your
scratch directory, in exactly this shape", with a line `- <entry>: <verdict>; proposed outcome
<outcome>`, and then "Return each entry you judged with its verdict, `conforms` or `fails`, in the
shape the brief's schema asks". Its section `write-a-draft` defines a draft's verdict as
"`conforms`, or each rule it fails". Which vocabulary does `<verdict>` in `return.md` take?

## Details

### What

Three statements of one agent definition, `path@agent-skills@content/agents/design-record-auditor.md`:

- the draft's verdict: "`conforms`, or each rule it fails, named by the section of the rule";
- the line of `return.md`: `<verdict>`, with no vocabulary named;
- the structured return: "`conforms` or `fails`", which the saved workflow's schema
  `JUDGED` in `path@agent-skills@content/workflows/design-record-audit.js` enforces as an enum.

An auditor can fill `<verdict>` in `return.md` either with the draft's verdict, naming each rule,
or with the schema's two values. Two census agents of the design discussion on what enters
agent-facing text reported the pair independently. Two census agents of step 4 of the work on what enters
agent-facing text read the saved workflow: it passes the schema to the drafting agent, and its
report step is an agent that reads every `return.md`, so no step parses the verdict. Not
established: whether that report agent treats the two vocabularies alike.

### Why it matters

`goal@knowledge-architect@agents-get-a-complete-workflow` asks that each instruction say what it
asks "plainly enough to follow without guessing what it means". An auditor that guesses may write a
`return.md` whose verdicts the report step reads differently from the structured return.

### What would close it

A reading of every step of `path@agent-skills@content/workflows/design-record-audit.js` that reads
`return.md`, which settles whether the two vocabularies can diverge; then either a rewording of
`agent@knowledge-architect-design-record-auditor@what-to-return` that names the vocabulary of
`<verdict>`, or a note in this entry that the two readings lead to one result, and its closure.
