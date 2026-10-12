---
kind: design
---
# No checked form cites a level-three subsection or a numbered point inside a section of a skill, an agent or the primer

## Summary

A skill, an agent, the primer and the root `CLAUDE.md` are citable down to a level-two section.
A citation of a point inside one, such as an entry test, a loop step or a numbered point, stays in
words, and nothing checks it. In one session, three such citations went stale or were wrong from the
start, and only a reviewer reading by hand found them.

## Details

### What

- `design@core@section-homes-carry-slugs` gives every level-two heading of a skill, an agent, the
  primer and the root `CLAUDE.md` a slug, and `design@core@harness-kinds-cited-without-anchor`
  cites them, as `skill@<name>@<slug>`. A level-three heading carries no slug, and a numbered point
  of a list carries none.
- Citations in words in the installed text, for example: "test 4 included" in
  `agent@knowledge-architect-decision-record-reviewer`; "A load-bearing gap of point 2" in
  `skill@knowledge-architect-planning@working-a-slice`; and "loop step" numbers across
  `skill@knowledge-architect-design@discussion-loop`.
- The case, this repository's agent-configuration-entities milestone: three citations in words
  went stale or were wrong from the start; a fidelity reviewer reading by hand found them.
- The core's rejected alternative on register entries at level three concerns defining entries,
  not citing sections.

The suspected mechanism, in one sentence: the citable unit is the level-two section, and the
installed text argues and cross-refers at the level of a point inside one.

Re-entry point: the design discussion that takes up this entry.

### Why it matters

A citation of a numbered point is a pointer `cargo klarch show` does not list and `check` does not
judge, so renumbering a list or moving a test silently falsifies every citation of it, against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. A new form changes
`design@core@harness-kinds-cited-without-anchor` or `design@core@section-homes-carry-slugs`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that chooses among: a checked form for
a point inside a section; a convention that each such point is a heading with a slug, which
multiplies slugs in every skill; or keeping the citations in words, recorded with its reason.
