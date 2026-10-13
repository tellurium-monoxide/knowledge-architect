---
kind: question
---
# A head in the wrong Component goes to the owner, though the record settles the Component

## Summary

A design-record audit puts on the owner list every head it finds in the wrong Component. The
Component a head belongs to is settled by a recorded rule, so the owner is asked a question the
record answers. Should the move of such a head be applied in the audit's branch instead?

## Details

### What

`design@agent-skills@audit-outcomes` sends to the owner "a head in the wrong Component", in the
same sentence as "which side of a contradiction wins" and "a provable gap". Its first rule says
"An edit that changes no ruling of the owner is applied in the audit's branch", and a move of a
misfiled head changes no ruling of the owner. The same sentence cites
`design@agent-skills@questions-go-to-their-authority`, which gives the session what the record
settles. Which Component owns a head is settled by
`skill@knowledge-architect-decision-recording@owning-component`.

The milestone that wrote the authority routing kept this outcome with the owner, citing
`design@agent-skills@design-record-axis`. That head argues why the axis judges a head's Component,
not why the outcome goes to the owner. A slice 4 review of that milestone found the gap; it was
not argued there, since the slice's spec named the outcome as staying.

A move can still bear on the owner: a head whose Component is uncertain under the owning-component
rule, or whose move changes what another Component's goals constrain. Those cases may justify the
owner list for some moves and not others.

### Why it matters

`goal@knowledge-architect@agents-work-without-drift` asks for an owner ruling only on what the
goals and the record leave open. Each misfiled head an audit finds is one more question the owner
answers that the record already answers. The head also contradicts itself: its first rule applies
the move, and its list of owner outcomes holds it.

### What would close it

A design discussion, under `skill@knowledge-architect-design`, that either keeps the outcome with
the owner, on an argument the head then states, or applies the move in the branch where the
owning-component rule settles it. The outcome table of
`skill@knowledge-architect-project-audit@outcomes` and the head are then rewritten to match.
