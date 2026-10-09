---
kind: defect
---
# The title of the head on directories named after the project is false of the tree

## Summary

`design@knowledge-architect@no-directory-named-after-the-project` is titled "No directory is named
after the project". The tree holds a tracked directory of that name that is no anchor. The head's
argument covers a Component's directory only, so the title states more than the argument argues.

## Details

### What

The title reads "No directory is named after the project, and a published crate's directory may
differ from its package's name". Its argument: "A Component is named by the basename of its
directory, and the project root by the project's name, so a directory named `knowledge-architect`
would collide with the root". The owner's word it quotes keeps the head to "not naming a component
`knowledge-architect`".

The tree holds the directory knowledge-architect under the agent-config location, holding the
installed primer: `git ls-files` on that directory lists its PRIMER.md. It is no anchor, so it
collides with nothing, and the checker passes. The defect is in the head's title, not in the tree.

Found by a read-only audit of the root's design heads during the design discussion of
`issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle`, and reproduced by
reading the title against the listing.

### Why it matters

A session that reads the title as written would refuse, or rename, a directory the decision does
not forbid, and the title misstates the decision every reader of the outline sees, against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`.

### What would close it

The title reworded to state what the argument argues, a Component's directory, under
`skill@knowledge-architect-decision-recording`: a rewording that narrows no decision, since the
owner's word already kept the head to Components.
