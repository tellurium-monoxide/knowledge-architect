# Slice 3: design skill

## Builds

In `path@agent-skills@content/skills/design/SKILL.md`, per "The fork and the state `argued`" and
"Bounded work and a derived thread proceed on their default" in the milestone document:

- `skill@knowledge-architect-design@decision-authority`: "The owner owns every decision" scoped to
  what the authority section reserves; "The weighing is theirs" scoped to a fork.
- `skill@knowledge-architect-design@threads-and-states`: the state `argued` in the paragraph that
  defines the closed set; closure on the argument where one shape stands; `ruled-out` set by the
  session when it names the defeating fact; the checkpoint shows `argued` closures for contest;
  "exactly two declared cases" of closure without the owner's word updated.
- `skill@knowledge-architect-design@discussion-loop`: step 3's factual questions narrowed to facts
  only the owner holds.
- `skill@knowledge-architect-design@bounded-work`, and its two other sites: the skill's
  description, "bounded work that needs only a proposal and the owner's word", and the overview,
  "Bounded work ends sooner, at the owner's word on its one proposal". The one message proceeds on
  its default; its commit already attributes the owner's answer, since slice 2's review repair.
- The design skill's expectation set in `skill@knowledge-architect-retrospective@expectation-sets`,
  and its restatements in `path@agent-skills@README.md`: "two declared exceptions", "Every decision
  routes to you" and the other lines that hold closure to the owner's word.
- The texts that count the design skill's cases of closure without the owner, or that say only the
  owner decides: in `agent@knowledge-architect-transcript-reviewer`, "Only the owner's word
  decides." and its pointer to "Who moves what"; the retrospective's "the two cases the skill
  declares".
- CHANGELOG.md `Next release`: a Workflow entry for the closures that need no word and the bounded
  path's default.

## Claims

- **This discussion's threads sort as the reply to R3 sorted them.** Test: a fresh reader applies
  the new closure rule to the table under a60 in the milestone document and gets the same split; a
  rule that sends a cost to the owner sends all nine.
- **Convergence is still computable from the states.** Test: the paragraph that defines the closed
  set holds `argued`, and step 6's "every PROPOSAL thread is closed" then counts it.
- **No text counts two exceptions to closure by the owner.** Test:
  `grep -rniE "two declared|two cases|exactly two" crates/agent-skills`, each hit read.

## Audit subjects

`path@agent-skills@content/skills/design/SKILL.md` whole;
`path@agent-skills@content/skills/retrospective/SKILL.md`, its expectation sets;
`path@agent-skills@README.md`; `path@agent-skills@content/agents/transcript-reviewer.md`;
`design@agent-skills@structure-the-flow`, `design@agent-skills@bounded-path-in-design`,
`design@agent-skills@thread-slug-is-entry-id`, `design@agent-skills@frontier-tier-only`,
`design@agent-skills@expectation-set-bounds-scope`;
`issue@agent-skills@the-material-finding-duty-has-no-head`,
`issue@agent-skills@a-proposal-is-read-against-the-heads-only-after-the-plan-is-written`,
`issue@agent-skills@expectation-sets-for-the-installed-skills`.

## Fails alone on

- A thread whose rival lost to a fact still needs the owner's word under the new text.
- A thread that is a fork closes `argued` under the new text.

## Premises that expire

None beyond the milestone's.

## Implementation sequence

1. The states and the closure rule; decision authority; step 3.
2. The bounded path at its three sites; the expectation set and the README; the counting texts;
   `cargo klarch install-agent-skills`; the CHANGELOG.md entry.
3. Review, then the harvest.

## Harvest

| what | where | when |
| --- | --- | --- |
| #owner-rules-at-a-fork | a head in `register@agent-skills@design`, slug its thread's, recording the owner's reason for the discussion form attributed, after `design@agent-skills@structure-the-flow` is read for the related ground it holds; `design@agent-skills@structure-the-flow` rewritten | this slice |
| #derived-decision-proceeds-on-default, for bounded work | `design@agent-skills@bounded-path-in-design` rewritten | this slice |
| `design@agent-skills@thread-slug-is-entry-id`, `design@agent-skills@frontier-tier-only`, `design@agent-skills@expectation-set-bounds-scope` | rewritten in place where their text names closure by the owner or the closed set | this slice |
| #thread-closes-on-the-argument | `path@agent-skills@docs/rejected-alternatives.md` | this slice |
| T1 | `register@agent-skills@tripwire` | this slice |
| this spec | deleted | the commit that completes this harvest |
