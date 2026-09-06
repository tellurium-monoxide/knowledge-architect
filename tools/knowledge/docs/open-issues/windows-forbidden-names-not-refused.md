---
kind: deferred
---
# Only a line break is refused in a file name, and Windows forbids more

## Summary

The walk refuses a path holding a newline or a carriage return, on two grounds: no output
line, reference or index row can hold one, and Windows refuses to create such a file. The
second ground covers more names than the first, and only the first is enforced.

## Details

### What

`walk::refused` in `path@knowledge@documentation/src/walk.rs` matches the two line-break
bytes and nothing else. Windows also refuses the characters `<`, `>`, `:`, `"`, `|`, `?`, `*`
and `\`, every other control character below the space, a name ending in a space or a
period, and the reserved device names such as `CON`, `NUL` and `COM1` whatever their
extension. A tree holding any of those checks clean here and cannot be checked out there.
The decision is `design@knowledge@git-supplies-the-walk`, whose refusal paragraph names the
portability ground.

### Why it matters

A project that means to be usable on Windows learns it is not from a failed checkout rather
than from this gate. That is the failure the line-break refusal exists against, met on a
name the refusal does not cover. For this repository the cost today is nil: no such name is
in the tree, and `cargo knowledge check` over it is the way to re-take that.

### Trigger

The knowledge tool being prepared for publication outside this repository, as a tool other
projects run over their own trees. Whoever packages it decides which trees it accepts, and
this check is one line of that decision: one predicate in `walk::refused`, the finding text
in `check::registers` widened from a line break to the class, and the head's refusal
paragraph restated for the class. A tree holding such a name under a `skip-files` row or an
ignore rule is kept, as a line-break name is.
