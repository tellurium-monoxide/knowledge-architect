# Tripwires — knowledge-architect

Evidence that would reopen a decision of `path@knowledge-architect@docs/design.md`. A tripwire is
not something to do: it is a hypothesis about a future failure plus the response, and it leaves
this file when it fires. What is outstanding about the project is
`path@knowledge-architect@docs/open-issues/`.

## Guarding `design@knowledge-architect@plain-text-is-no-repair`: a checked pointer turned into plain text `##plain-text-pointer-found`

**Fires when:** a review finds a diff that turns a checked reference or a backticked path into
plain text naming the same target, with no reference beside it to an issue entry that records a
missing form; or a retrospective's answer to its question on unchecked pointers reports a pointer
written as bare plain text to clear a finding.
**Response:** open a `defect` for the instance. At the second instance, reopen
`design@knowledge-architect@plain-text-is-no-repair`, with a mechanical check for plain-text
pointers among the candidates.
**Re-entry:** the standing-state review, which reads every tripwire before a merge, and the
retrospective's question on unchecked pointers, which reads the session.
