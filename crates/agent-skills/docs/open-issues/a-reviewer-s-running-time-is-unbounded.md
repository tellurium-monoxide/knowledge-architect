---
kind: todo
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
reviewer reports what it has reproduced before it queues more work. The first observation is
from the retrospective of one thaum session, written on 2026-10-02 outside this repository, with
two reviewers of one review round. The second is below. It recurs in two real sessions, both in
adversarial reviews; the shape of an addition is not approved, per the closing condition below.

A second instance, in this repository: the adversarial reviewer of the pre-release branch, which
added `check --fix`, queued a batch of mutation runs and stopped with its background work still
running. The harness reported it as "waiting on its own background work" twice, and its findings,
one of them critical, arrived about three minutes after the last of the other three reviewers had
reported, by the run durations the harness gave: 178 to 242 seconds for those three, 408 for it.
The dispatcher had committed the other reviews' repairs in the meantime.

### Why it matters

A reviewer that holds its findings for many minutes delays every repair, and a dispatcher that does
not ask may wait on a reviewer that has stopped producing. An instruction is worth text only where
the default is systematically wrong, per `design@agent-skills@capabilities-not-structure`. Two
sessions show it recurs; they do not show how often.

### What would close it

An addition to the review skill whose shape the owner approves, under
`design@agent-skills@additions-need-real-use`, or the owner's ruling that the problem belongs to the
projects that meet it. The owner approved an addition after the second session, "report what is
reproduced before queueing more", and then withdrew that approval as written: "I'm not fully
satisfied with the solution written in there. This problem only affects adversarial code reviews
and not the others, so it is a project specific problem, actually. I don't want to block the
release while I don't have a better solution in mind, and it has been fixed locally on thaum's
side." Both instances were adversarial reviewers that queued mutation runs. A shape that the owner
approves, or a third instance outside adversarial code review, reopens the addition.
