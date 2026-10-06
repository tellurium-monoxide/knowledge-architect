---
kind: design
---
# Nothing states which part of a command's printed output is a contract

## Summary

The versioning policy classes changes to "a command" without saying which part of a command's
behaviour is its usage. The owner's direction is that printed output is not a stable interface,
apart from documented exceptions such as the verdict line, and that data a user needs is offered in
a stable, documented machine-readable form instead. That policy is not designed or recorded yet.

## Details

### What

Instances seen:

- `commits` changed its summary from `<j> judged, <f> failed` to
  `<n> commits, <p> passed, <f> failed`, and a commit whose message alone carries findings now
  counts as failed. The verdict line and the exit code are unchanged. No row of the table of
  `design@knowledge-architect@versioning-policy` names a change to a command's printed report: the
  patch row excludes a change to a command, the minor row takes only a change that adds, and the
  major row takes "a command change that breaks existing usage" without saying whether parsing the
  report is usage. The change was classed patch, with a changelog entry, on the owner's ruling
  below.
- The two output additions that `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check`
  records, classed minor and patch within one release.

The README of the core, `path@core@README.md`, states two output contracts: for `check`, the order
of the summary, the findings and the verdict line; and for a generated file-register index, written
by `index` and `check --fix`, its bytes, per `design@core@a-file-register-index-is-rows`. Nothing
states whether the words of a printed summary are one.

Suspected mechanism: the policy names commands as a surface, and a command has several parts (its
arguments, its exit code, its verdict line, its report text) with no statement of which are
stable.

The owner's ruling, verbatim: "My take on this usually is that output of commands is not advertised
as stable public API (even the contrary, users are warned about avoiding parsing the outputs), with
the possibility of documented exceptions left open (like the verdict). Instead, the project should
be able to output data in a stable, documented machine readable format (or output of the function
in the code), when users may need to access it. Designing this policy precisely and conforming to
it would need a design discussion IMO." On the instance above: "make the change we just did a
patch, but still write an entry about it in the CHANGELOG, because the ruling I just gave was not
yet official."

### Why it matters

`design@knowledge-architect@versioning-policy` cannot class a change to a command's report, so each
such change is classed by the nearest reading, and two changes of one kind already took two classes
in one release. Under 0.x a patch and a minor are different numbers, so the class decides whether a
consumer's plain `cargo update` reaches the change, which
`goal@knowledge-architect@any-project-can-adopt-it` relies on. A consumer that parses a report has
no statement telling it not to.

### What would close it

A design discussion under `knowledge-architect-design` whose outcome is recorded: which parts of a
command's output are stable, where users are told so, which data gets a machine-readable form or a
library function, and the row of the versioning table each kind of change takes. That includes a line added to a command's
output, which `issue@knowledge-architect@the-bump-table-has-no-row-for-a-looser-check` records
and leaves to this entry.

### Re-entry point

That discussion, opened on the owner's word, at the latest before the project leaves 0.x under
`design@knowledge-architect@stays-at-zero-x`.
