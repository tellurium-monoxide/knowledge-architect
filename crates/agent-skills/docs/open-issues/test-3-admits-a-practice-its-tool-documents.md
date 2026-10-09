---
kind: question
---
# Does test 3 admit a decision that only follows a practice its tool's documentation explains?

## Summary

Test 3 of `primer@design-heads` admits a decision whose argument turns on the
behaviour of something outside the project, documented or measured. It admits
`design@knowledge-architect@toolchain-is-pinned`, which pins the Rust toolchain, a practice that
Rust's own documentation explains and that nobody has a reason to reverse. The owner doubts such a
decision needs a head, and doubts the test can be narrowed without risk.

## Details

### What

The owner, on that head: "toolchain-is-pinned is a very standard practice that is explained in
several Rust documentation places AFAIK. I don't think it ever needed a design head, on this ground.
There is little reason for someone to go out of his way to reverse it, too, IMO. But you are right
that test 3 admits it. Maybe keep it to avoid confusion. Not sure if test 3 can be narrowed further
without risk." The head was kept.

A narrower test 3 might admit only a reading the project makes of a tool's behaviour that the
tool's documentation does not state, or contradicts. Not established: which heads such a test would
then refuse, and whether one of them guards against a reversal that would cost the project. A read
of every head that test 3 alone admits, against that narrower wording, answers it.

### Why it matters

`design@agent-skills@a-head-is-owed-by-an-entry-test` keeps the design homes to what an entry test
admits, since every grounding reads them and the owner reviews them. A test that admits a standard
practice adds heads that carry nothing a reader could not find in the tool's documentation. A test
narrowed too far drops a head whose reading of a tool cost work to establish, the case test 3
exists for.

### What would close it

The read above, and the owner's ruling on whether test 3 is narrowed; if it is, the head records the
narrower wording and the installed skill states it.
