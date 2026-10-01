---
kind: todo
---
# References are read in Markdown and Rust files only

## Summary

The walk parses two file kinds, Markdown and Rust source. A reference written in a comment of any
other language is never read: it is not checked, `show` does not list it, and a deleted or renamed
entry leaves it dangling with no finding. A project whose code is in another language gets the
document checks and none of the code-comment checks.

## Details

### What

`LIVE_SUFFIXES` in `path@core@src/walk.rs` lists the suffixes the walk reads, `md` and `rs`. The
Rust grammar is what tells the reader which byte ranges of a source file are prose. A project in
Python, TypeScript, Go or any other language would need, for each language, a way to find its
comments, and a way for the manifest to say which source files are read. Nothing about the shape is
decided.

### Why it matters

`goal@knowledge-architect@any-project-can-adopt-it` targets projects whatever their language.
The installed skills tell an agent to name an issue in the comment at the code that exists because
of it, so that closing the issue dangles the comment. In a project that is not written in Rust that
instruction produces references nothing reads, and the skills have to tell the agent to grep
instead.

### What would close it

The walk reads the comments of at least one further language, chosen by the manifest, with a test
over a mock project in that language in which a dangling reference in a comment is reported. Or
the owner rules that the checker stays limited to Rust source, and the limit is recorded in the
core's design home.
