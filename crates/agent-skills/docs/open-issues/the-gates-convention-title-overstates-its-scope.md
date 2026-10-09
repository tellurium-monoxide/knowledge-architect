---
kind: defect
---
# The title of the gates convention states every project runs the published gates library

## Summary

`design@agent-skills@gates-convention` is titled "The setup skill recommends one gates command, run
by the published gates library". Its argument covers a Rust project with a maintenance crate only,
and `design@agent-skills@exact-pin` and the setup skill say any other project builds its own gates.

## Details

### What

The head's argument: "In a Rust project with a maintenance crate it is a command of that crate, per
`design@agent-skills@xtask-pins-checker`, so every adopting project runs the gates refined in this
repository and in thaum."

`design@agent-skills@exact-pin`: a Rust project of one package "installs the binary as any other
project does and builds its own gates". The setup skill, `skill@knowledge-architect-setup`, in its
paragraph on a Rust project with no workspace: "the project builds its own gates command".

So the title, and the clause "every adopting project runs the gates refined in this repository",
state more than the argument argues, and the record contradicts itself on a project that is not
Rust or has no maintenance crate.

Found by a read-only audit of the agent-skills design heads against
`design@agent-skills@title-states-the-rule` and its neighbours, and reproduced by
reading the three texts side by side.

### Why it matters

A session setting up a project with no maintenance crate reads a title that says it runs the
published library, against `goal@knowledge-architect@setup-brings-quality-tools`, which asks that a
project reach the tools without designing them itself.

### What would close it

The title and the clause stating what the argument argues: the one gates command, and the published
library where the project has a maintenance crate, under
`skill@knowledge-architect-decision-recording`.
