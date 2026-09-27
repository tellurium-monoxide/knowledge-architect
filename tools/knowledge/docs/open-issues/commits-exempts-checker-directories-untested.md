---
kind: todo
---
# No test shows that `commits` reads the checker's own directories as data in each commit's tree

## Summary

`commits` hands the checker's source directories, made project-relative, to every per-commit
model, so the tool's own fixtures are read as data there as they are under `check`. No test
fails when that list is emptied or cut to one directory: an adversarial review of the core and
extension split ran both mutations and both survived the whole suite.

## Details

### What

In `commits`, in `path@knowledge@documentation/src/cli/history.rs`, `checker_rel` is built by a
`filter_map` over the directories the binary passes, and handed to `commit_tree`, which hands it
to `Model::from_documents_under`. Replacing the `filter_map` with one that yields no directory, or
only the first, compiles and passes every test of `documentation`, `knowledge`, `citations` and
`rules-corpus`. With the first directory only, `cargo knowledge commits origin/main..HEAD` over
this repository, at the commit that recorded this entry, stops with a phase-3 tree of 140
findings per commit, every one in a rule-shaped fixture under tools/rules-corpus/. The same
mutation of the single directory survived before the split.

### Why it matters

`design@knowledge@checker-source-literals-are-data` rests on every model the binary builds being
told the directories. A regression here breaks the `commits` gate over this repository, where the
gate would catch it, but reaches nothing in a project whose history holds no checker source,
and no test names the property.

### What would close it

A test that builds a history inside a checkout holding checker source with a rule-shaped fixture
under a stated directory, runs `commits` over it with the directory stated, and asserts the
fixture is read as data; and a second directory stated beside it, so cutting the list to one is
caught. Each mutation above is then run and recorded as caught.
