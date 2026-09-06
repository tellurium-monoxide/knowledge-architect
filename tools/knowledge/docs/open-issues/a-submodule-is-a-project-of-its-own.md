---
kind: design
---
# A submodule is a project of its own, or nothing, and the tool has no model for either

## Summary

The walk reads nothing under a submodule's gitlink entry and names it as a phase-2 finding, and
an `exclude` row is the declared silence. Behind the finding is a question the tool has not
decided: what a submodule is to a project this tool checks. Two answers are defensible, and this
entry parks both until a project holds one.

## Details

### What

A submodule is a checkout of another repository at a path of this one, recorded in the tree as
one gitlink entry naming a commit. `git ls-files` lists the entry and never descends, so the
walk reads nothing under it, and both readers name the entry by its mode. No project this tool
checks holds one, and this repository plans none.

Two readings of what a submodule is, either of which would give the tool a rule:

- **A submodule holding a `knowledge.toml` is a project of its own.** Its manifest names its
  components, its registers and its corpus, and its documents are conformant to it, not to the
  enclosing project's manifest. The enclosing project's walk stops at the gitlink, as it does
  today, and a finding names it as another project. What that leaves open is a reference from
  one project into the other's entities, which the reference grammar cannot spell: an anchor
  is a name declared in one manifest. A relation between the two manifests would have to say
  how the outer project names the inner one — as an anchor of a new kind, or by mounting the
  inner project's anchors under a prefix — and how the inner project's path references are
  asserted from the outer tree.
- **A submodule holding no `knowledge.toml` is not documentation.** It is vendored code, and
  the tool ignores it the way it ignores an excluded path, which is what an `exclude` row
  naming it does today.

The suspected mechanism, in one sentence: the tool models one manifest per walk, and a
submodule is a second manifest inside the first's tree.

### Why it matters

A project vendored as a submodule is conformant by vacuum until its gitlink is excluded, and the
finding is what says so. The cost of deciding early is a design discussion and a modelling
change nobody needs yet; the cost of deciding late is nil while no checked project holds a
submodule.

### What would close it

The first project this tool checks that holds a submodule, or the tool's preparation for
publication outside this repository, whichever comes first: that is the discussion at which
the two readings above are argued, and it starts from them. Until then, nothing is owed: the
gitlink is named, and the row that keeps it is a declaration a reader can find.
