# Slice 1: goals and primer

## Builds

- G1, G2 and G3 in `register@knowledge-architect@goal`, verbatim as #owner-decides-goal-wording
  and #owner-word-is-verified of the milestone document give them, under
  `skill@knowledge-architect-goal-setting`.
- The authority section in `path@agent-skills@content/PRIMER.md`, beside `primer@goals-bind`,
  verbatim as "Decided design" gives it.
- `primer@owner-word-premise` scoped to a ruling on intent, with the source rule of "A source
  outranks an owner's answer on a reading".
- `primer@labelled-items`: a label numbers what was put to the owner, never what decided.
- `primer@met-outside-the-task`, case 1: an item that bears on the work and that a source, a
  measurement or the record answers is decided by the session and reported; a contradiction holds
  the part of the work resting on it, per G2.
- The "Decided" section, as a primer directive beside the "Met outside the task" rule.
- In this repository: the root `CLAUDE.md` states the actions it reserves (D10); its restatements
  of the primer's changed directives follow, as `primer@where-knowledge-goes` says of a
  restatement.
- CHANGELOG.md `Next release`: a Workflow entry for the questions the primer stops putting to the
  owner and for the "Decided" section.

## Claims

- **A session reading the new primer answers a question a source answers without asking.** Test:
  the authority section names the four authorities and the owner's four kinds, read against the
  "Decided design" text; a wrong implementation, one that keeps "a cost to weigh" as an owner kind,
  fails the comparison.
- **The premise procedure no longer contradicts a source's authority.** Test: a reviewer given the
  thaum case of the retrospective (an answer of the owner that a definition contradicts) finds one
  instruction that applies and no second one that asks the owner; the old text gives two.
- **No installed text still says that every decision is the owner's.** Test:
  `grep -rniE "owns every decision|every decision routes|decisions belong to" crates/agent-skills/content docs/goals.md`
  returns only lines this slice leaves to slice 3 or 4, each named in the commit.
- **Every reference to the two goals still resolves and still argues true.** Test:
  `cargo klarch check`, and a reread of each text `cargo klarch show` lists for
  `goal@knowledge-architect@the-owner-decides` and `goal@knowledge-architect@agents-work-without-drift`.

## Audit subjects

`path@agent-skills@content/PRIMER.md` whole; `register@knowledge-architect@goal`;
`design@agent-skills@primer-content`, `design@agent-skills@primer-limit`,
`design@agent-skills@ruled-items-labelled`, `design@agent-skills@tripwire-wording-is-the-agents`;
the citing texts of the two goals, listed in the milestone document; `path@knowledge-architect@CLAUDE.md`
`## Git` and `## Release status`; `design@knowledge-architect@git-flow`.

## Fails alone on

- A reviewer finds a question the authority section sends to the session that G1 reserves to the
  owner, or the reverse.
- The primer's size argument of `design@agent-skills@primer-limit` does not admit the two new
  directives.

## Premises that expire

- Until slice 3, the design skill still says "The owner owns every decision" and the bounded path
  still waits. The primer's authority section prevails where they conflict, since slice 1's commit
  says so and slice 3 removes the conflict.
- The heads this slice's harvest writes follow the quotation rule in force; slice 2 converts them.

## Implementation sequence

1. The goals: G1, G2, G3, one commit quoting the rulings, as the goal-setting skill requires.
2. The primer: the authority section, the premise scope, the label rule, case 1, the "Decided"
   section; then `cargo klarch install-agent-skills`.
3. This repository's root `CLAUDE.md` and the CHANGELOG.md entry.
4. Review, then the harvest.

## Harvest

| what | where | when |
| --- | --- | --- |
| #questions-go-to-their-authority, with the label rule of #answer-keeps-its-authority | a head in `register@agent-skills@design`, slug question-authority | this slice |
| #specification-outranks-an-owner-answer | a head in `register@agent-skills@design`, slug its thread's | this slice |
| #decisions-reported-to-the-owner | `design@agent-skills@primer-content` rewritten, and a head if an entry test passes | this slice |
| `design@agent-skills@ruled-items-labelled`, `design@agent-skills@tripwire-wording-is-the-agents` | rewritten in place | this slice |
| T2, T3 | `register@agent-skills@tripwire`, labels kept in their text | this slice |
| this spec | deleted | the commit that completes this harvest |
