# Workflow text: the entry tests and their head, local intent, the bounded path

The spec of the first slice of `milestone@plans@load-bearing-records`. It holds what only this
slice builds; what crosses slices is in the milestone document. Every item it cites is defined
there.

## Builds

- **§2 of the decision-recording skill**, `path@agent-skills@content/skills/decision-recording/SKILL.md`:
  the entry tests reworded per #t1-consumed-interface, #entry-test-locality and
  #t3-external-behaviour.
- **The primer's "Intent and claims"**, `path@agent-skills@content/PRIMER.md`: a third bullet for
  intent recorded at the code, per #local-intent-binds; the restatement of the two classes under
  "Verify before relying on anything" in `path@knowledge-architect@CLAUDE.md` follows it.
- **The decision-record reviewer**, `path@agent-skills@content/agents/decision-record-reviewer.md`:
  one predicate in its §2, per #local-intent-binds.
- **The design skill**, `path@agent-skills@content/skills/design/SKILL.md`: the section "When NOT
  to use" replaced by the bounded path, and its description reaching a session that starts a
  requested change, per #bounded-path-in-design.
- **`path@agent-skills@README.md`**: its two passages on a bounded problem rewritten to the bounded
  path.
- **The installed copies** under .claude, by `cargo klarch install-agent-skills`, in the same commit
  as each change to content/.
- **The harvest** below.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| The rewritten §2 admits a policy and refuses a local mechanism and a repeated path | the acceptance criterion `acceptance@load-bearing-records@a-cold-session-applies-the-tests` | run it once against the current §2 first: T2old admits none of the three decisions on a narrow reading, or all of them on a wide one, as the audit agents reported, so the run tells the two texts apart |
| Every reference the slice writes resolves, and no reference to a closed or renamed entry remains | `cargo klarch check` | the slug of `design@agent-skills@bounded-problem-branch` is renamed in the slice: a reference left to the old slug fails the check |
| The installed text equals the shipped text | `cargo klarch check`, which compares each installed file byte for byte | an edit to content/ committed without `cargo klarch install-agent-skills` fails it |

## Audit subjects

- The milestone document entire, and this spec.
- §1, §2 and §6 of `path@agent-skills@content/skills/decision-recording/SKILL.md`.
- `path@agent-skills@content/PRIMER.md`, and the heads `design@agent-skills@primer-content` and
  `design@agent-skills@primer-limit`, which bound what the primer may hold.
- `path@agent-skills@content/skills/design/SKILL.md`: its description, "When NOT to use", the
  in-change path and keep-or-change.
- §5 of `path@agent-skills@content/skills/retrospective/SKILL.md`, the design skill's expectation
  set: whether the bounded path adds an assumption about the owner, per
  `design@agent-skills@expectation-set-bounds-scope`.
- `path@agent-skills@content/agents/decision-record-reviewer.md` §2, and
  `path@agent-skills@content/agents/transcript-reviewer.md` §3, which stays unchanged.
- The heads `design@agent-skills@standing-entries-searched-before-the-work`,
  `design@agent-skills@conformance-before-every-merge`, `design@agent-skills@bounded-problem-branch`,
  `design@agent-skills@harvest-after-implementation` and `design@agent-skills@losing-alternatives-filter`.
- The tripwires `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` and
  `tripwire@agent-skills@search-missed-before-the-work`.
- "Verify before relying on anything" in `path@knowledge-architect@CLAUDE.md`.

## Fails alone on

- A fresh session given the rewritten §2 classifies one of the three decisions of
  `acceptance@load-bearing-records@a-cold-session-applies-the-tests` against the ruling.
- `cargo klarch check` reports a reference to `design@agent-skills@bounded-problem-branch`, or to
  the heads this slice rewrites.

## Premises that expire

- **The snippet branch is not merged.** This slice does not depend on it. Its head
  `design@agent-skills@setup-rust-section` is among the heads of the cleanup issue, so a merge of
  the snippet branch before this slice's harvest changes nothing here.

## Decided design

### #entry-test-locality, #t1-consumed-interface, #t3-external-behaviour: the three entry tests

§2 of the decision-recording skill keeps its frame: "A decision earns an entry in a design home only
if at least one of these holds", followed by the sentence that sends the rest to an inline comment
and the commit message. The three tests become, as an illustration of the shape and not as
authority over the wording:

```text
1. reversing it would change an interface others consume: a type or a signature that crosses the
   boundary of a separately built unit, a file format, a document grammar, a command line;
2. the same reason must be respected at more than one site, or at none. A reason is an argument
   against a rival someone could plausibly propose; a name, a path or a value repeated for
   consistency is not one. At one site, the reason goes in a comment there. At none: a decision
   about an absence, or a policy with no code of its own;
3. its argument turns on the behaviour of something outside the project: an external
   specification the project implements, or an external tool's behaviour, documented or measured.
```

- **The argument for T1** (a39, a40): a head about a grammar, a format or a command binds every
  adopting project; under a strict T1 such a head enforced in one module would pass no test.
  Nearest rival: T1 kept strict, the decided T2 admitting these heads; it fails on the one-module
  case.
- **The argument for T2** (a10, a28, a29, a50): a reason needed at several sites needs a home that
  each site can cite, and the checker cannot cite a comment; a policy has no site, and the old T2
  admitted it. Nearest rivals: T2 deleted (a10: a policy loses its admission test), and "the same
  statement at more than one site" (a50: a path repeated for consistency passes it).
- **The argument for T3** (a41, a42): a measured behaviour costs as much to re-take as a documented
  one to re-read. Nearest rival: documented behaviour only; it fails where the documentation is
  silent.
- **TM stays** the test of agent-skills, in `design@agent-skills@instruction-record-is-minimal`, and
  is not part of the installed skill (a30).

### #record-volume: one head records the tests and the principle

A new head of agent-skills states that a decision earns a design head only when one of the entry
tests passes, and why. Its standing argument: every grounding reads the design homes whole, and a
human overseer reviews them (a6, a23, a33); the session that changes local code reads the comment
at that code, not the design home (a23); the tests are where the checker's reach ends, since a
comment cannot be cited (a29). Nearest rival: a head for every decision that was discussed with the
owner. It is what this session did on the snippet branch, and the owner judged the result
unneeded (a1, a2). The head names `goal@knowledge-architect@design-is-recorded-with-its-arguments`.
It is reopened by the audit's outcome only on the owner's word; the audit left it standing (a36,
a65).

### #local-intent-binds: the order, and what an agent does

The primer's "Intent and claims" gains a bullet, between the design home and the claim about the
code. An illustration of its content:

```text
- A reason recorded at the code — an inline comment that says why the code is shaped so, or the
  message of the commit that argued it — is intent at the scale of that code. It binds as a
  presumption: a change that defeats it argues against it in its own commit message. Where it
  conflicts with a design home, the design home prevails, as in any divergence. Between a comment
  and a commit message, the comment is the current statement. Before removing or reshaping code as
  unneeded, read its comment and the message of the commit that introduced it (git log -L, git
  blame): absence from the design home is not evidence that code is superfluous.
```

The decision-record reviewer gains a predicate in its §2: a diff that removes or reshapes code
against the reason its comment states, or the message of the commit that introduced it, without
arguing against that reason in its own message, is a finding. The transcript reviewer is unchanged:
its §3 already counts a comment at the code and a commit message as durable outcomes (a18).

The argument: a comment is part of the code, which the design home governs (a31); the primer gives
an inline comment a home but no class, which is the gap behind the owner's worry (a5, a11). The
weakness, named and accepted: in a project whose comments the checker does not read, a reference in
a comment is not checked (a19). Nearest rival: no rule, relying on keep-or-change, which applies
only when an incumbent design is evaluated.

A new head of agent-skills records the order and its argument. The primer's restatement in
`path@knowledge-architect@CLAUDE.md` follows, with its pointer.

### #bounded-path-in-design: the shortcut after grounding

The design skill's section "When NOT to use" is replaced by a section on bounded work. An
illustration of its content:

```text
After the grounding of loop step 1, the work is bounded when all three hold:
  - it reverses no recorded decision (decision-recording §1);
  - every decision it makes fails the entry tests (decision-recording §2);
  - no second defensible shape survives the nearest-rival test; the strongest open reading is
    stated beside the bounded one.
Then: one message holding what the grounding found, the proposal, its nearest rival and why it
loses, the consequences, and the default. Wait for the owner's word. An argument in the reply
returns the work to the loop. The commit that implements the work carries the proposal and the
owner's words verbatim.
Otherwise: the loop, on the in-change path or the full path.
```

The skill's description gains the symptom that reaches a session before it starts a requested change
whose design is not settled, since a session doing an issue fix does not load the skill today
(a21).

The argument: whether work is bounded is known only after grounding (a7); the grounding runs the
standing-entry search, which no step ran for undesigned work (a12); the entry tests applied at
classification stop a head from being written for a decision that earns none (a20). Nearest rival:
a separate installed skill for bounded problems, as the issue closed by the commit that adds the
milestone document planned; it would have to repeat the grounding to classify at all (a7).

`design@agent-skills@bounded-problem-branch` is rewritten in place under the slug
`bounded-path-in-design`. `design@agent-skills@standing-entries-searched-before-the-work` states
that the search runs at the grounding of the bounded path too, and that work which does not go
through the design skill sends none. The paragraph of `design@agent-skills@conformance-before-every-merge`
that parks a search before undesigned work states that the design skill hosts it for bounded
work. `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` is restated for work that
bypasses the design skill, since its re-entry, the design of a skill for bounded problems, was this
discussion; its new re-entry is the standing-state review, which reads every deferred trigger.

### #record-audit: the cleanup issue

The harvest opens a `todo` issue, `heads-no-entry-test-admits`, in agent-skills. It lists:

- the 16 heads the audit found admitted by no test, by slug: `toolchain-is-pinned` (root);
  `content-mirrors-the-install-layout`, `in-change-waits-for-premortem`,
  `plan-landing-is-not-tied-to-its-work`, `argument-ids`, `retiring-plan-opens-issue`,
  `setup-rust-section`, `setup-default-crates-io-page`, `component-goal-refines-root`,
  `retro-content`, `premortem-as-watch-points`, `retro-two-files`, `finding-ids`,
  `version-in-report`, `retro-file-location` (agent-skills); `spec-file-owns-its-items` (core);
- `design@agent-skills@roadmap-home`, which the owner judged unneeded (a43);
- the split of `design@knowledge-architect@repo-layout`, keeping only its two absences (a44);
- the split of `design@core@spec-file-owns-its-items`: its citation form passes T1, its ownership
  rule in `Anchors::owns_entry` belongs in a comment there (a51);
- the move of `design@knowledge-architect@a-reference-claims-a-revisit` and
  `design@knowledge-architect@a-past-sentence-is-rewritten` to agent-skills (a45, a52).

It states that the agent-skills verdicts were the audit agents', and that the owner read them by
title, not as a ruling (a58): the session that does the cleanup judges each head in full and
presents its verdicts to the owner before deleting anything. Its agent-skills part waits for slice
2, since `%%` lines are where content worth keeping goes.

## Mapping tables

None: the slice rewrites text and records, and the code needs no function.

## Losing alternatives

- **#record-test-at-classification**, superseded by #bounded-path-in-design: its position, no new
  wording in the design and planning skills, is the bounded path's classification itself (a8,
  a20).
- **A separate installed skill for bounded problems**, lost to #bounded-path-in-design: the
  classification needs the design skill's grounding (a7). Recorded in the rejected alternatives at
  the harvest under test 4 of §6, a doubt remaining, which P1 watches.

## Acceptance criteria

### A cold session applies the rewritten entry tests as the discussion ruled `##a-cold-session-applies-the-tests`

- **Guards:** #entry-test-locality, with #t1-consumed-interface and #t3-external-behaviour.
- **Judged by:** this slice, before its merge. A fresh subagent, with no access to this milestone,
  is given the rewritten §2 of the installed decision-recording skill and three decisions, each
  described in two sentences: the main of the setup skill compiled by an example of xtask (the head
  the snippet branch wrote and removed); `design@knowledge-architect@stays-at-zero-x`; and
  `design@agent-skills@roadmap-home`.
- **Fires when:** the subagent does not answer "no head", "a head, by T2: a policy", and "no head"
  respectively.
- **Response:** the wording of §2 is repaired in this slice and the run repeated; a second failure
  reopens #entry-test-locality with the owner.

## Harvest

At this slice's landing, under `knowledge-architect-decision-recording` and
`knowledge-architect-issue-tracking`:

| what | home |
| --- | --- |
| #record-volume, with #entry-test-locality, #t1-consumed-interface and #t3-external-behaviour as its content: #record-volume → `a-head-is-owed-by-an-entry-test`, since the thread's slug names a quantity rather than the decision | a new head in `path@agent-skills@docs/design.md` |
| #local-intent-binds | a new head in `path@agent-skills@docs/design.md`, slug `local-intent-binds` |
| #bounded-path-in-design | `design@agent-skills@bounded-problem-branch` rewritten in place, slug `bounded-path-in-design` |
| the occasions of the search | `design@agent-skills@standing-entries-searched-before-the-work` and `design@agent-skills@conformance-before-every-merge`, rewritten in place |
| a separate installed skill for bounded problems | `path@agent-skills@docs/rejected-alternatives.md`, lost to the head of #bounded-path-in-design, `live` |
| P1, P2, P3 | `path@agent-skills@docs/tripwires.md`: `a-shortcut-decision-earns-a-head` guarding the head of #bounded-path-in-design, `a-head-verdict-is-overruled` guarding `a-head-is-owed-by-an-entry-test`, `a-comment-escalation-is-overruled` guarding `local-intent-binds` |
| `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | restated in place |
| #record-audit | `planned@agent-skills@docs/open-issues/heads-no-entry-test-admits.md` |
| the lead of the closed issue, if the owner keeps the default | `planned@agent-skills@docs/open-issues/the-open-space-test-may-admit-every-problem.md`, a `question` |
| #record-test-at-classification | none: superseded, named in the harvest's commit |

The slice's spec leaves in the commit that completes this harvest.
