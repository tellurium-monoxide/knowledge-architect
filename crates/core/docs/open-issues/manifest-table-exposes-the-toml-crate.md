---
kind: question
---
# An extension reads its manifest table as a `toml::Value`, so the core's toml version is part of the public API

## Summary

`Manifest::table` returns `Option<&toml::Value>`, and the crate documentation tells an extension
to read its tables with it. An extension then depends on the toml crate at the major version the
core uses, and a toml major bump in the core is a library break. Nothing records whether that
coupling is intended.

## Details

### What

Decide how an extension receives its tables. Three shapes are on the table: keep `toml::Value`
and state the coupling, as `design@core@arguments-parse-through-clap` does for clap; re-export the
toml crate from the `extension` module, so an extension uses the core's copy; or hand the table
over in a form the core owns, such as a deserialized type the extension names.

### Why it matters

`design@core@api-facade` makes the public surface the facade, but this signature exposes a type
of a dependency the facade does not re-export. Under `design@knowledge-architect@versioning-policy`
a toml major bump in the core would then be a library break that no change of the core's own code
announces. thaum's rules extension calls `table("rules")` and converts the value with
`try_into`.

### What would close it

A recorded decision on the shape, in the core's design home, and the signature and the crate
documentation matching it.
