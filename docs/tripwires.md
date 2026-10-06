# Tripwires — knowledge-architect

Evidence that would reopen a decision of `path@knowledge-architect@docs/design.md`. A tripwire is
not something to do: it is a hypothesis about a future failure plus the response, and it leaves
this file when it fires. What is outstanding about the project is
`path@knowledge-architect@docs/open-issues/`.

## Guarding `design@knowledge-architect@plain-text-is-no-repair`: pointers written as plain text where a checked form exists `##plain-text-pointer-found`

**Fires when:** a review, or a retrospective's answer to its question on unchecked pointers, finds
in a committed document a path of the project or an entry id written outside backticks where a
checked form exists, or written with no reference beside it to an issue entry that records the
missing form.
**Response:** open a `defect` for the instance. At the second instance, reopen
`design@knowledge-architect@plain-text-is-no-repair`, with a mechanical check for plain-text
pointers among the candidates.
**Re-entry:** the routing review, which reads every durable statement a diff adds, and the
retrospective's question on unchecked pointers, which reads the session.
