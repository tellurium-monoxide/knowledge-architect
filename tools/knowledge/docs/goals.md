# Goals — knowledge

What this tool is for. A goal is met or unmet, where a decision about how the tool is built is
won or lost and lives in `path@knowledge@docs/design.md`.

## The project's documents stay tidy and do not grow stale `##documents-stay-current`

## The rules of the project, the citation regime first among them, are followed mechanically `##the-regime-is-mechanical`

## Relocating or refactoring what exists costs one manifest edit and no document `##relocation-is-one-manifest-edit`

## A recorded decision or an outstanding issue is delivered to whoever needs it, through checked references `##records-reach-their-reader`

## The documentation half is published on its own and used by other projects `##documentation-half-publishes-alone`

The checks over documents, references, registers and trackers become a crate other projects
depend on, and the rules half is rebuilt as an extension on top of it. Nothing about the
Comprehensive Rules may therefore be compiled into the documentation half.

Why it is worth publishing: agents instructed to keep documentation current and free of stale
statements fail without mechanical verification, or need several rounds of expensive self-review.
The one-home instruction plus checked references gives a self-review a work list, every site that
must be looked at when a decision is reversed, and in-tree issue registers are easier for an agent
to use than an external tracker. A uniform, readable grammar is what makes the tool reusable.

## The documents render as one linked site `##documents-render-as-a-linked-site`

A preprocessing pass rewrites every reference into a link and an mdbook build renders the result,
hosted with the project once it is released. Dense cross-referencing is what makes the rendered
site worth reading, so every citeable thing having one reference form serves this goal directly.
