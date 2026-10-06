---
kind: todo
---
# The library does not export the checker's version, so a test reads its pin from a manifest

## Summary

A test that copies a mock project out of its library's directory must write a real version into
the copy's `[project] checker-version`, since `"fixture"` is refused there. The library exports no
constant holding its own version, so the test reads the project's pin from the root manifest. The
owner wants the version exported.

## Details

### What

`design@core@checked-sentinel-values` accepts `"fixture"` only inside a directory the running
binary was built from, and says that a copy of a mock outside its library carries the version.
The core's own binary tests write `env!("CARGO_PKG_VERSION")`, in the function that builds a
manifest's text in `path@core@tests/binary.rs`. That works because the core's package is the
checker. In an extension's crate, `CARGO_PKG_VERSION` is the extension's version, not the
checker's, so the setup skill and the crate documentation of lib.rs tell such a test to read the
pin from the root manifest instead. The need was met in thaum, whose binary tests copy each mock
into a temporary directory and run `git init` there (finding W1 of its retrospective of the move
to 0.3.0).

Reading the root manifest ties the test to the repository's layout, and to the pin rather than to
the version the binary links. The two are equal whenever the project's own check passes, since
every binary refuses a project whose pin it does not satisfy, per
`design@core@installed-binary-version-check`.

### Why it matters

An extension's test suite writes a version it cannot name from the library it tests, against
`goal@core@projects-add-their-own-checks`, whose extension API is meant to carry what an
extension needs.

### What would close it

A public constant in the facade of lib.rs holding the core library's version, per
`design@core@api-facade`, documented as the value a copied mock writes. The setup skill and the
`# Testing an extension` section of lib.rs then name it in place of the root manifest, with a
`Next release` entry under New features.
