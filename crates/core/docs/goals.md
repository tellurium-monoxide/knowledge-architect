# Goals — the core checker

What the core checker is for, as its share of the project's goals,
`path@knowledge-architect@docs/goals.md`. A goal is met or unmet, where a decision about how the
checker is built is won or lost and lives in `path@core@docs/design.md`. A goal stays here while
it is met, and leaves only when the owner abandons it.

## The project's documents stay tidy and do not grow stale `##documents-stay-current`

## The instructions a project declares about its documents are enforced by a check `##declared-instructions-are-checked`

A project states in its manifest what it records, where each kind of record lives, and how one
record points at another. A tree that passes the check conforms to all of it, and no project can
hold part of it back. Conformance is established by running one command, not by a reviewer
remembering the instructions.

## Relocating or refactoring what exists costs one manifest edit and no document `##relocation-is-one-manifest-edit`

## A recorded decision or an outstanding issue is delivered to whoever needs it, through checked references `##records-reach-their-reader`

## The documents render as one linked site `##documents-render-as-a-linked-site`

A preprocessing pass rewrites every reference into a link and an mdbook build renders the result,
hosted with the project once it is released. Dense cross-referencing is what makes the rendered
site worth reading, so every citeable thing having one reference form serves this goal directly.
