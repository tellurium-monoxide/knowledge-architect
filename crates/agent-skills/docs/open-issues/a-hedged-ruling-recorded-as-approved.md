---
kind: defect
---
# A hedged word of the owner was recorded as an approved thread in a committed plan document

## Summary

The first commit of the pre-release spec recorded the owner's "Maybe I'd also make it the default
in the set-up skill" as an approved thread. The approval rested on the agent's own reply, not on
the owner's word. The transcript reviewer of that commit found it, and the owner ruled it as D5.
`tripwire@agent-skills@ruling-lost-in-assembly` fired on it, under the wording it then had.

## Details

### What

The owner's round-7 message of the discussion that produced the pre-release spec read: "B with the
three defaults. Maybe I'd also make it the default in the set-up skill: for published Rust crates,
keep the README.md as a repo facing document, and use a separate CRATES-IO.md for what crates.io
readers see. go ahead with the spec". The agent replied "I'm adding your setup-skill default as a
decided thread", and the spec's commit recorded the thread's final state as approved, with the
hedge quoted beside it.

The assembly subagent quoted the owner verbatim. The state came from the agent's reply. The design
skill already says a word that requires interpretation closes nothing and leaves the thread
presumed-settled. The agent did not apply that rule, and the assembly carried the agent's state
through. Whether this recurs is not established: it is one instance.

The spec was repaired in place before any work rested on the thread: the owner's next word
decided it.

### Why it matters

`design@agent-skills@ledger-from-transcript` rests on assembly recording the owner's rulings as the
owner made them, and `goal@knowledge-architect@the-owner-decides` is met only while a decision
recorded as the owner's is one the owner made. A state the agent gave, recorded as the owner's,
would build work on a decision nobody made, if no review catches it.

### What would close it

A second instance closes it as a pattern to act on: the planning skill's assembly would then state
that a thread whose closing word is the agent's, or a word that requires interpretation, is
recorded as presumed-settled, never approved. The tripwire's re-entry, the standing-state review
before every merge, is where a second instance is found. A year of plan documents without one
closes it as an isolated slip.
