# Slice 1: goals and primer

## Builds

- **The goals**, under `skill@knowledge-architect-goal-setting`, in `register@knowledge-architect@goal`:
  G1 and G3 verbatim as #owner-decides-goal-wording of the milestone document gives them, G2 as
  #owner-word-is-verified gives it with D12's rewording if the owner approves it, its slug
  `the-owner-s-word-is-verified`.
- **The authority section** in `path@agent-skills@content/PRIMER.md`, beside `primer@goals-bind`,
  as "Decided design" gives it, with the rank sentence of D13.
- **`primer@owner-word-premise`** rewritten to the three-point shape of "A source outranks an
  owner's answer on a reading".
- **`primer@labelled-items`**: a label numbers what was put to the owner, never what decided.
- **`primer@met-outside-the-task`**, case 1: an item that bears on the work points to the authority
  section for who answers it. A contradiction holds the part of the work resting on it, per G2; a
  fork is presented with a default, built on meanwhile; an action that cannot be undone waits.
- **`primer@intent-and-claims`**, its hold on a divergence, per D4.
- **The "Decided" section**, a primer section of its own, slug decided-section, as "Decided design"
  gives it.
- **The rejected alternative "A delegated state"**, in `path@agent-skills@docs/rejected-alternatives.md`,
  reread against G1 and its sentence that argued from the goal's old wording reworded, as "What is
  already decided" says.
- **The `todo` issue of D15**, in `register@knowledge-architect@issue`.
- **CHANGELOG.md `Next release`**: a Workflow entry for the questions the primer stops putting to
  the owner, the hold on a contradiction, and the "Decided" section.

## Claims

- **A session reading the new primer answers a question a source answers without asking.** Test:
  read the authority section against "Decided design": it names three authorities, a
  specification's text, a measurement and the argument from the record, and four kinds the owner
  answers; a wrong implementation, one that keeps "a cost to weigh" as an owner kind, fails the
  comparison.
- **The primer gives one procedure for an owner's answer that a source contradicts.** Test: a fresh
  reviewer given W1, as the Names table of the milestone document states it, finds one instruction
  that applies, the source rule, and no second one that asks the owner; the old text gives the
  premise procedure's question.
- **No installed text still says that every decision is the owner's, unranked.** Test:
  `grep -rniE "owns every decision|every decision routes|decisions belong to|only the owner's word decides" crates/agent-skills docs/goals.md`
  returns only lines that slice 3 or 4 rewrites, each named in the commit; the authority section's
  rank sentence covers them until then.
- **Every reference to the two goals still resolves and still argues true.** Test:
  `cargo klarch check`, and a reread of each text the milestone document lists for
  `goal@knowledge-architect@the-owner-decides` and `goal@knowledge-architect@agents-work-without-drift`.

## Audit subjects

`path@agent-skills@content/PRIMER.md` whole; `register@knowledge-architect@goal`;
`design@agent-skills@primer-content`, `design@agent-skills@primer-limit`,
`design@agent-skills@ruled-items-labelled`, `design@agent-skills@tripwire-wording-is-the-agents`,
`design@agent-skills@head-prevails-over-diverging-code`; the citing texts of the two goals, listed
in the milestone document; `issue@agent-skills@primer-content-bundles-each-directive-s-decision`;
the delegated state in `path@agent-skills@docs/rejected-alternatives.md`;
`skill@knowledge-architect-planning@working-a-slice`, its paragraph "Where the owner's word is
awaited", which names the default of `primer@owner-word-premise`.

## Fails alone on

- A reviewer finds a question the authority section sends to the session that G1 reserves to the
  owner, or the reverse.
- The content test of `design@agent-skills@primer-limit` does not admit the authority section or
  the "Decided" section as directives every session applies.

## Premises that expire

- Until slices 3 and 4 land, the design, planning and review skills still send to the owner
  questions the authority section gives the session. The guard is D13's rank sentence, in the
  authority section, which every session reads, and D13's clause that no version is published
  before slice 4.
- The heads and tripwires this slice's harvest writes follow the quotation rule in force; slice 2
  converts them.

## Implementation sequence

1. The goals: G1, G2, G3, one commit quoting the rulings, as the goal-setting skill requires.
2. The primer: the authority section, the premise procedure, the label rule, case 1, the
   divergence, the "Decided" section; then `cargo klarch install-agent-skills`.
3. The rejected alternative, the D15 issue, the CHANGELOG.md entry.
4. Review, then the harvest.

## Harvest

| what | where | when |
| --- | --- | --- |
| G1, G2, G3 | `register@knowledge-architect@goal` | step 1 of this slice |
| #questions-go-to-their-authority | a head in `register@agent-skills@design`, slug its thread's | this slice |
| #answer-keeps-its-authority | a head of its own, slug its thread's; the label rule in `design@agent-skills@ruled-items-labelled`, rewritten | this slice |
| #specification-outranks-an-owner-answer | a head, slug a-source-outranks-an-owner-answer, the pair stated in the commit | this slice |
| #decisions-reported-to-the-owner | a head, slug its thread's | this slice |
| `design@agent-skills@primer-content` | rewritten: pointers to the new heads, case 1 per G2, each directive this slice changes leaving for its own head | this slice |
| `design@agent-skills@tripwire-wording-is-the-agents`, `design@agent-skills@head-prevails-over-diverging-code` | rewritten in place | this slice |
| thaum's first draft, the premise procedure with the text as its default | `path@agent-skills@docs/rejected-alternatives.md` | this slice |
| T2, T3 | `register@agent-skills@tripwire`, labels kept in their text | this slice |
| this spec | deleted | the commit that completes this harvest |
