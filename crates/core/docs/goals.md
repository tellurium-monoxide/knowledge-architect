# Goals — the core checker

What the core checker is for, as its share of the project's goals,
`path@knowledge-architect@docs/goals.md`. A goal is met or unmet, where a decision about how the
checker is built is won or lost and lives in `path@core@docs/design.md`. A goal stays here while
it is met, and leaves only when the owner abandons it.

## The project's documents stay tidy and do not grow stale `##documents-stay-current`

Every reference resolves, every register entry has the shape its register declares, and every
generated listing matches what regenerating it writes, at every commit. It is met while the check
passes on the main branch and on every commit of a branch that merges. It refines
`goal@knowledge-architect@documentation-stays-consistent`.

## The instructions a project declares about its documents are enforced by a check `##declared-instructions-are-checked`

A project states in its manifest what it records, where each kind of record lives, and how one
record points at another. A tree that passes the check conforms to all of it, and no project can
hold part of it back. Conformance is established by running one command, not by a reviewer
remembering the instructions. It refines `goal@knowledge-architect@documentation-stays-consistent`.

## Relocating or refactoring what exists costs one manifest edit and no document `##relocation-is-one-manifest-edit`

Every reference to what the manifest places names its anchor, and each anchor's directory is
declared once, in the manifest, so moving a Component or a location changes the manifest and no
document. It is met while such a move
leaves no reference to repair. It refines `goal@knowledge-architect@documentation-stays-consistent`.

## A recorded decision or an outstanding issue is delivered to whoever needs it, through checked references `##records-reach-their-reader`

For any entry, the checker lists every text that references it, and it lists every open issue and
tripwire of the project, so a session reading a decision meets what stands against it, and a
reversal comes with the list of what it touches. It is met while those listings are computed from
the tree, never kept by hand. It refines
`goal@knowledge-architect@structure-and-workflow-work-together`.

## The documents render as one linked site `##documents-render-as-a-linked-site`

A preprocessing pass rewrites every reference into a link and an mdbook build renders the result,
hosted with the project once it is released. Dense cross-referencing is what makes the rendered
site worth reading, so every citeable thing having one reference form serves this goal directly.

## A project adds checks of its own through a documented extension API `##projects-add-their-own-checks`

A project whose conventions the built-in checks do not cover writes its own checks as an extension,
against a library API documented with its contract and its examples, and runs them in the same
command, with the same report, as the built-in ones. It is met while an extension needs no change to
the core and uses only its public, documented API. It refines
`goal@knowledge-architect@agents-work-without-drift`: a project's own checks guard its own design
intent.
