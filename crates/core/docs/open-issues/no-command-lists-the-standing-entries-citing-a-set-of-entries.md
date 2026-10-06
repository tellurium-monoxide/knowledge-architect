---
kind: todo
---
# No command lists the issues and tripwires that cite any of a set of entries

## Summary

A search for the issues and tripwires that a piece of work bears on starts from the decisions and
goals the work names. For each one, `show` prints the file and line of every reference to it. It
does not name the entry that holds the reference, and it takes one entry per call. A command that
takes several references and prints the issue and tripwire entries citing any of them, one row
each, would make that first step mechanical.

## Details

### What

What the commands print today:

- `show <ref>` prints the entry, then `referenced at:` and one `<file>:<line>` per reference. A
  tripwires home holds many entries in one file, so a line number has to be mapped by hand to the
  heading above it. A reference from a design head, a rejected alternative or a plan document is
  listed beside those from standing entries.
- `tripwires --guarding <ref>` prints the tripwire rows guarding one entry. It takes one reference.
- `issues` filters by anchor, by kind, by group and by text in the id or the title. It has no
  filter on what an entry cites.

A design discussion of the owner wants a search agent to run `show` on every decision and goal the
work names, then keep the issue and tripwire entries among the citing sites, before reading the
other entries by meaning. The owner's words: "I think this is worth an issue, it would make the
task much easier. Left for later design." Nothing about the command's shape is decided: a new
command, a `--citing <ref>` filter repeated on `issues` and `tripwires`, or several references
accepted by `--guarding`.

### Why it matters

`goal@knowledge-architect@structure-and-workflow-work-together` states that "the tool computes the
workflow's work lists: what a reversed decision touches, and which issues and tripwires a change
must re-read". For a change that touches several decisions, the tool computes the second list one
entry at a time and by file and line, and the agent turns that into entries. That costs one call
per entry and a hand mapping, and a mapping done by hand can drop an entry.

### What would close it

A command, or a filter of an existing one, that takes several references and prints every issue
entry and every tripwire entry whose text cites any of them, one row per entry naming it as a
reference. The installed search agent, `knowledge-architect-standing-entry-searcher` once it is
built, maps citing sites to entries by hand; its body changes to use the command in the same
work. It is tested over a mock project in which two tripwires of one home and one issue cite
two different decisions. Or the owner rules that the search keeps using `show`, and this entry is
deleted with that reason in the commit.
