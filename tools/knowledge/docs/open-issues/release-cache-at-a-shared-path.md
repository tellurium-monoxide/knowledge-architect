---
kind: observation
---
# The release cache sits at a predictable shared path

## Summary

`resolve` in `path@knowledge@rules/src/release.rs` answers a release that is neither vendored
nor archived from `std::env::temp_dir()/MagicCompRules-<date>.txt`, downloading only when that
file is absent. When `MANIFEST.tsv` holds no row for the date — which is every bump target by
construction, since the manifest records only superseded releases — the digest has nothing to
compare against, so whatever bytes sit at that path are returned.

## Details

### What

`resolve` in `path@knowledge@rules/src/release.rs` answers a release that is neither
vendored nor archived from `std::env::temp_dir()/MagicCompRules-<date>.txt`, downloading only
when that file is absent. When `MANIFEST.tsv` holds no row for the date — which is every bump
target by construction, since the manifest records only superseded releases — the digest has
nothing to compare against, so whatever bytes sit at that path are returned. A `bump` then
builds its diff, its effective-as-of report and its `CHANGES.md` skeleton from them.

### Why it matters

The temp directory is world-writable and the path is predictable, so a stale
or foreign file there is indistinguishable from a download. The blast radius is bounded: the
pinned text a bump writes is walked by `cargo knowledge check`, so grossly wrong bytes fail
hundreds of quote verifications loudly — but a subtly wrong text that preserved every cited rule
would pin silently. First contact with a new release is inherently unverifiable against a prior
record; trusting a shared path by existence is the avoidable part.

### What would close it

Caching under a user-owned directory, or not caching across runs at all
— the file is under 1 MB — leaves only the download in the window. Either means threading the
location through `Tree`, whose constructor has six call sites, because three tests seed the
current path directly to exercise the digest refusal, the fold-before-digest order and the
reproduction above. Deciding instead that a shared cache is accepted, and saying so in `resolve`'s
doc comment, closes this as a recorded trade-off.

### Reproduce

An empty manifest and the cache pre-seeded with arbitrary bytes: `resolve`
returns those bytes as the release. First seen in the adversarial review of the first bump, and
now pinned by `a_release_the_manifest_does_not_know_resolves_from_whatever_the_cache_holds` in
`path@knowledge@rules/src/release.rs`, so a change that starts verifying them has to edit that test.

### What is already done

`resolve` prints the digest of the bytes it returns in both cases, and
names their source — downloaded, or read from a pre-existing cache — through `provenance` in the
same file. A run therefore says which text it used, and no longer asserts a fetch that did not
happen. That turns silent trust into reported trust; it verifies nothing.

### Reachability is state-dependent, not structural

`local` answers first, and `check` resolves
every release a document pins, reporting `2 of 2 release(s) resolved locally` on this tree — so
the cache branch is unreached only while every pin is vendored or archived. A `cr-version` marker
naming an unarchived release puts `check` itself on this path. `rules diff` against an unarchived
date, `rules fetch` and `rules bump` reach it by design.
