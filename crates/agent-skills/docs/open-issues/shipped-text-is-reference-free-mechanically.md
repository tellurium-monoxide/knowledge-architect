---
kind: todo
---
# Nothing checks mechanically that the shipped text holds no live reference and names no project path

## Summary

The text under content/ is installed in other projects, so it must hold no live reference and name
no path of this repository. This repository's manifest takes content/ out of the walk, and the
installed copies are out of it by construction, so no check reads that text. Only the reviews of
each change to it judge it.

## Details

### What

A mechanical check over the shipped text: every backticked span that the reference reader would
take as a candidate is a placeholder, and no span names a path of this repository. The check could
run over content/ in this repository's gates, or be a mode of the checker that reads a directory as
shipped text rather than as documents. The shape is not decided.

### Why it matters

`design@agent-skills@shipped-text-is-reference-free` rests on the reviews alone. A live reference
that a review misses ships, and in every installing project it is a dangling reference the project
cannot repair, against `goal@knowledge-architect@any-project-can-adopt-it`, because the file is
the installer's and the check compares its bytes. The walk
exclusion of content/ in `path@knowledge-architect@knowledge-architect.toml` exists only because
no such check exists.

### What would close it

The check, run by `cargo x gates`, failing on a live reference or a path of this repository planted
in a scratch copy of content/, and the walk exclusion of content/ either removed or kept with the
check as its stated reason. The release procedure planned in the milestone document greps the
shipped text for references; the check replaces that grep.
