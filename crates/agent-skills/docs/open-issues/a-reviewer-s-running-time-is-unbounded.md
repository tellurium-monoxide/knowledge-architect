---
kind: observation
---
# A reviewer that queues long work reports nothing until it is asked

## Summary

In a session in thaum on v0.1.0, two adversarial reviewers queued long batches of mutations, each
signalled for many minutes that it was waiting on its own background work, and each reported only
when the dispatcher asked. `knowledge-architect-review` says nothing about how long a reviewer runs
or when it reports.

## Details

### What

The installed review skill states the invariants of a dispatched review: fresh reviewers, a blind
brief, reproduction, a worktree each. None bounds a reviewer's running time, or says that a
reviewer reports what it has reproduced before it queues more work. The observation is from the
retrospective of one thaum session, written on 2026-10-02 outside this repository; it was seen
once, with two reviewers of one review round. Whether it recurs, and whether the default
behaviour is systematically wrong, is not established.

A second instance, in this repository: the adversarial reviewer of the pre-release branch, which
added `check --fix`, queued a batch of mutation runs and stopped with its background work still
running. The harness reported it as "waiting on its own background work" twice, and its findings,
one of them critical, arrived about three minutes after the last of the other three reviewers had
reported, by the run durations the harness gave: 178 to 242 seconds for those three, 408 for it.
The dispatcher had committed the other reviews' repairs in the meantime.

### Why it matters

A reviewer that holds its findings for many minutes delays every repair, and a dispatcher that does
not ask may wait on a reviewer that has stopped producing. An instruction is worth text only where
the default is systematically wrong, per `design@agent-skills@capability-over-conformance`, and one
observation does not show that.

### What would close it

A second real session where a reviewer withholds its findings while queueing work: it then becomes
an addition to the review skill, such as "report what is reproduced before queueing more", under
`design@agent-skills@additions-need-real-use`. Or a year of reviews in which it does not recur,
which closes it as noise.
