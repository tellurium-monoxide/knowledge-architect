---
kind: defect
---
# The subcommands share the manifest's name, which the head on shared code does not state

## Summary

`design@xtask@one-module-per-subcommand` says "The shared code is the gates library's spawn
helper". Both subcommands also use the checker library's manifest name and the gates library's
`project_root`. The head's own rule admits
it, since every subcommand needs it, so the head states the instance built rather than its rule.

## Details

### What

The head: "The shared code is the gates library's spawn helper, `design@gates@one-spawn-helper`.
[...] a second piece of shared code has to earn its place by being needed by every subcommand."

`path@xtask@src/gates.rs` and `path@xtask@src/changelog.rs` each import
`knowledge_architect::MANIFEST_NAME` and pass it to the gates library's `project_root` to find the
project's root. The changelog subcommand also uses the gates library's `complain` and `say`, and
spawns no child, so it uses no spawn helper at all.

Found by a read-only audit of the xtask design head against
`design@agent-skills@title-states-the-rule` and its neighbours, and reproduced by
grep for `MANIFEST_NAME` under the xtask sources.

### Why it matters

The head's statement of what is shared is false of the code, so a session that reads it as the
rule would treat the manifest name's import as a departure, or refuse a third shared item its rule
admits. It is the defect `design@agent-skills@title-states-the-rule` names: a head that states the
instance as the rule.

### What would close it

The head stating its rule, shared code is what every subcommand needs, with the members built named
as such, under `skill@knowledge-architect-decision-recording`.
