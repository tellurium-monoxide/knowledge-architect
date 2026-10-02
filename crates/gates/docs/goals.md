# Goals — gates

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `path@gates@docs/design.md`. A goal stays here while it is met, and leaves only when the
owner abandons it. This component is published, so its goals are encouraged to refine a goal of
the project as a whole, `path@knowledge-architect@docs/goals.md`.

## A Rust project gets its merge gates by writing only its list of gates `##gates-from-a-list`

A Rust project that adopts knowledge-architect runs every check it owes before a merge through
this library, and writes only the list of its gates: the runner, the complete logs, the distilled
failures and the verdict from exit codes come with it. It is met while a project's maintenance
binary holds no gate logic beyond its list. It refines
`goal@knowledge-architect@setup-brings-quality-tools`.
