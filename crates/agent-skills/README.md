# knowledge-architect-agent-skills

The text that `knowledge-architect` installs into a project's agent configuration: skills,
subagent definitions, and a primer the project's root CLAUDE.md imports. This crate only carries
the text. The checker writes it into a project with its `install-agent-skills` command, and checks
that the installed files match it.

A project depends on `knowledge-architect`, never on this crate directly: cargo fetches it.

A bounded problem, a clear requirement whose main risk is over-building, has no skill of its own:
the design skill grounds it, and then sends one proposal for the owner's word.

The sections below describe one skill of the set, the design skill,
`knowledge-architect-design`: it runs design discussions as open, argued
exchanges instead of narrowing questionnaires.

## The design skill: what it changes

When a design question has an open solution space — requirements still
being discovered, several defensible shapes, a keep-or-change question
about an existing architecture — the assistant:

- opens with argued proposals (its own ideas included), not a menu of
  options or a battery of clarifying questions;
- tracks the discussion as named threads with explicit states, and the
  criteria they are judged against in a table of their own; your
  explicit word closes threads (an interpreted word only marks
  a presumption, except against a checkpoint table, where a positive
  word closes what the table holds), with two declared exceptions the
  assistant announces — withdrawing its own defeated proposals, and
  parking either its own overruled dissent or an observation a bug
  hunt raised for a later discussion; a summary table makes the
  standing state contestable at a glance;
- pushes back and expects push-back — the exchange is symmetric until
  a decision: your proposal is a claim to test, held to the
  same slots (why it works, what it costs, what it rules out) as the
  assistant's own — and reverses when a better argument or evidence
  lands;
- yields immediately to your explicit decision at any point,
  converged or not — then still verifies what the choice entails, and
  brings back a material finding exactly once, with the decided shape
  as the standing default — except that a finding that the decided
  shape cannot be built at all stops the work rather than proceeding,
  and work that cannot be undone (a migration run, a release shipped)
  is held for your word while the reversible part goes ahead;
- keeps the deliberation where the work lands: a decision whose work, or whose record, lands in the
  change under way takes the in-change path, with no plan document, and its argument goes in that
  change's commit message; other work goes to a plan document. Neither path bounds how many threads or
  rounds the discussion takes;
- builds discriminating evidence (a failing test, a prototype, a
  benchmark) when the discussion stalls on intuitions — with both
  parties stating beforehand which outcome would flip them;
- proposes convergence (every proposal thread closed, every criterion
  carrying a satisfaction line with no binding one unmet, the round
  quiescent) by
  showing the full thread table for one batch confirmation, instead
  of asking approval questions — and runs a premortem before
  handing off;
- hands the converged design to `knowledge-architect-planning`, which
  writes the spec or the milestone in the same session; each decision
  is recorded in its design home when the work implementing it lands,
  or when it is made if no work implements it, and a losing alternative
  where the recording tests keep it;
- writes in plain technical English: explicit quantities and invariants
  over qualitative wording, no idioms, no aphorisms.

For a bounded problem (clear requirement, main risk is over-building)
it grounds the request first, since only the grounding shows whether the
work is bounded. If it reverses no recorded decision, earns no design
head, and has no second defensible shape, it sends you one message: the
proposal, its nearest rival and why it loses, the strongest open reading
it set aside, the consequences, and its default. It waits for your word;
an argument in your reply turns it into a discussion.

It also carries a keep-or-change discipline for questioning incumbent
designs: fresh-discussion trend framing (a new session when the
incident still dominates the context), direction argued before
migration cost inside one decision, falsifiable verdicts with
tripwires, losing arguments absorbed rather than deleted.

## What it expects of you

This section restates, for you, the expectation set that §5 of the
retrospective skill carries, `path@agent-skills@content/skills/retrospective/SKILL.md`,
which is where it is applied, per `design@agent-skills@expectation-set-bounds-scope`.

The skill is scoped on purpose. It does not try to behave well under
every possible way of working, and it is not grown to absorb each new
one. The assumptions below are what it is built on. They are not rules
you are asked to follow — each names what degrades if it does not
hold, so you can decide whether the trade is worth it.

One boundary applies to this whole list: it scopes **your behavior,
never the skill's internal consistency**. If two of the skill's own
rules leave no move that satisfies both, that is a defect in the
skill, however the discussion was being conducted.

**You bring a design question, not a task order.** The mode assumes
the answer is not yet known by either party. Give it a decision you
have already made and it will argue against it, because testing a
proposal is what it is for. Give it bounded work and it sends one
proposal; you rule on it, or answer with an argument, which turns the
work into a discussion.

**You give the word that closes a thread.**
The assistant can move threads between open states but cannot close
one for you; that asymmetry is the design. Doing neither leaves
threads open, and the discussion cannot converge. There is no way to
hand a decision to the assistant: saying "your judgement" is an
approval like any other, and every finding that follows it still
returns to you.

**A closed thread is not frozen.** Your word reopens one at any time,
and you need no new argument to use it — the assistant does. If a
decision starts to look wrong later in the discussion, say so. Nothing
about having been recorded makes it final.

**You say why when you reject something.** Ruled-out threads carry
their reasons into the record, and the reopening rule reads those
reasons to decide whether a later proposal is genuinely new. A
rejection with no recorded reason cannot do that work, so the same
argument comes back and has to be defeated again.

**You read the summary tables and contest what is wrong in them.** The
batch confirmation at a checkpoint closes everything in the table at
once, by design — it exists so you are not asked to ratify twenty
items one at a time. Confirming without reading records decisions you
did not make.

**You correct a stated misreading.** When your message could close a
thread but does not clearly, the assistant marks it `presumed-settled`
and states its reading rather than closing. That mechanism only works
if wrong readings get corrected; uncorrected, they harden into the
record as though you had ruled.

**You end a discussion rather than dropping it.** "Stop, build X" and
"park this, I want to think" are both handled and both end the
argument immediately. Abandoning silently leaves the criteria and the
losing arguments unrecorded, which is the state a later session is
least able to recover from.

**You are the person who can decide.** Every decision routes to you.
If the real authority is someone not in the conversation, the
discussion converges on a verdict nobody present can act on.

**A discussion runs in one session, and memory does not outlive it.**
The thread ledger lives in the conversation and in your harness's
transcript of it, not in a file the assistant keeps, so what survives is
what the planning skill assembled from that transcript into the plan
document, and then the records harvested from it. Picking up a
previous design discussion in a *new* session is not supported: a new
session starts a new discussion, grounded on the record like any
other. Resuming a *session* is a different thing and works — if your
harness restores the full transcript and the project is in the same
state, the assistant never knows it happened, and bounded changes made
to the project meanwhile read to it as changes landing mid-discussion.

**You are trying to converge.** The skill assumes you engage toward
agreement on a set of proposals — arguing, ruling, or
saying stop. It does not defend against a participant who simply never
answers: several of its rules can only be released by your word, and
withholding one indefinitely leaves the discussion parked rather than
producing a wrong result.

**You accept a methodology, not a script for every exchange.** The
skill structures how a discussion is conducted: proposals argued,
threads tracked, closure held to your word. It does not try to specify
every way a conversation can go, and it is not extended each time one
goes a new way. Conduct outside what is described here is the
assistant's judgement, not a gap to be filled.

One assumption is about the project rather than about you:

**Design intent is discoverable, or the project is new.** The skill
grounds itself before proposing, by reading the goals, the design
homes, the rejected alternatives, the README files, the open issues
and the tripwires, or failing those the code and its history. A
greenfield project is handled explicitly. Intent that exists only in
someone's memory is not reachable, and proposals will contradict
decisions already made without either party noticing.

## What the setup skill expects of you

This section restates the setup skill's expectation set, which §5 of
the retrospective skill carries, as the section above does for the
design skill.

**You are present to rule.** The setup proposes and you rule: the
Components, the goals, the place of each existing document, and each
choice that changes your project's build or a file it already has.
A run without an owner, such as a trial, makes those choices itself,
and a gap it reports in a choice it had to make alone is not a defect
of the skill.

## Model requirements

The decision this section is the evidence for is
`design@agent-skills@frontier-tier-only`.

The skill is written for frontier-tier models, and the requirement is
capability, not compliance: it asks the assistant to maintain a
stateful thread ledger across rounds, to refuse closure on friendly
ambiguity, and to find the expensive-to-reverse fork in a design. A
scripted four-turn discussion (open ask → weak proposal under time
pressure → a "sounds mostly right" partial approval → a decision
override) was run with the skill active on three models, in August
2026: **Claude Opus 5**, **Claude Sonnet 5**, and **Claude Haiku
4.5**; **Claude Fable 5** ran every single-turn probe used during the
skill's development. Model capability moves fast between versions, so
read these results as pinned to those exact models, not to tier names:
a later Sonnet may well handle the skill fully, and an older Opus may
not (Opus 4.8 is adequate in the author's own use, untested here).

- **Opus 5**: full machinery and full depth. The strongest conducted
  run reframed the problem before proposing, pre-committed evidence
  thresholds that bound both parties, withdrew its own criterion when
  its own arithmetic killed it, identified the one irreversible fork
  the other tiers missed, refused to close that fork on "sounds mostly
  right" ("I need one word"), and shipped a decision record with every
  losing argument and tripwire in place.
- **Fable 5**: full machinery on every single-turn probe it ran. It
  was not put through the four-turn conducted script, so it has no
  evidence on the properties that only a multi-turn run exercises —
  carrying the thread table across rounds, and holding closure under
  repeated friendly ambiguity.
- **Sonnet 5**: the machinery holds — legal states, consistent
  tables, a usable decision record with losing arguments. In the
  original test its closure discipline bent under friendly ambiguity
  (threads closed on an *interpretation* of vague approval); the
  interpretation-is-presumption rule added in response corrected that
  failure in a follow-up probe on the same tier. The remaining known
  weakness is depth: its technical stress-testing runs shallower than
  Opus 5, so expect fewer found forks, not broken bookkeeping.
- **Haiku 4.5**: reproduces the format, not the discipline. In the
  test run it endorsed the weak proposal instead of testing it,
  invented states outside the enum, misapplied `presumed-settled` to
  its own guesses, silently dropped open threads between rounds, and
  produced a record with no arguments in it. Not recommended.

## Grounding in the literature

The skill was designed from practice and mapped against published work
afterwards; the correspondence below is claimed at the level of
mechanisms, not as experimental validation of the skill itself. The
mapping relates the skill to existing work and does not govern it:
where published work contradicts a mechanism, that is recorded here,
and real use decides whether it becomes a reason to change anything.

- **Convergence proposed, never requested.** [RFC 7282, "On Consensus
  and Humming in the IETF"](https://www.rfc-editor.org/rfc/rfc7282.html)
  describes the same test: consensus is looked for throughout the
  discussion rather than asked for at the end, and the absence of
  standing objections matters more than expressed agreement. The
  skill's checkpoint table — every issue enumerated, closure
  confirmed by one batch word over the whole ledger — is that test
  made mechanical.
- **Argued push-back over agreement-seeking.** Structured conflict
  (devil's advocacy, dialectical inquiry) produced better decisions
  than consensus-seeking in
  [Schweiger, Sandberg & Ragan 1986](https://journals.aom.org/doi/10.5465/255859);
  [Schwenk's 1990 meta-analysis](https://www.sciencedirect.com/science/article/abs/pii/074959789090051A)
  found devil's advocacy also outperformed a no-conflict expert
  baseline. The documented price is lower participant satisfaction — a
  trade this skill knowingly makes.
- **Dissent must be real, strawmen are worse than nothing.**
  [Nemeth, Brown & Rogers 2001](https://onlinelibrary.wiley.com/doi/abs/10.1002/ejsp.58)
  found authentic dissent stimulated divergent thinking while
  role-played devil's advocacy tended to bolster the original
  position. Hence the ban on alternatives built to lose, and the
  requirement that push-back carry actual arguments.
- **Sycophancy is the medium's documented failure mode.**
  [Sharma et al. 2023](https://arxiv.org/abs/2310.13548) showed both
  humans and preference models prefer convincingly-written sycophantic
  responses over correct ones a non-negligible fraction of the time,
  and that sycophancy in deployed assistants is likely driven in part
  by that preference signal. The push-back mandate and the ban on
  approval-seeking closers are countermeasures aimed at exactly this.
- **Genuine alternatives beat a single track.** Decisions pursued as a
  single imposed alternative failed far more often in
  [Nutt's decision-failure research](https://journals.aom.org/doi/10.5465/AME.1999.2570556);
  [Dow et al. 2010](https://dl.acm.org/doi/10.1145/1879831.1879836)
  found parallel alternatives reduce fixation and improve outcomes.
  The skill asks for genuinely distinct shapes as separate threads —
  and refuses a fixed count, because forced multiplicity produces the
  strawman pattern above.
- **Why an interactive exchange at all.** The classic finding that
  interacting groups underperform nominal ones in brainstorming
  (Diehl & Stroebe 1987) traced the loss chiefly to production
  blocking — waiting for the floor. A turn-based human–AI pair does
  not have that bottleneck, which is why this skill can keep ideation
  inside the argued exchange rather than separating it.
- **Recording rationale pays, but capture overhead kills it.**
  Designers assessing a past design had 41% of their design-rationale
  questions answered by recorded rationale in
  [Karsenty 1996](https://dl.acm.org/doi/fullHtml/10.1145/238386.238462);
  the same literature documents why rationale systems fail in
  practice — capture disrupts the work and records go unretrieved
  ([Horner & Atwood 2006](https://dl.acm.org/doi/10.1145/1182475.1182511)).
  The skill's answer: the assistant carries the bookkeeping, and the
  thread table becomes the plan document in the session that converged,
  and the record is harvested from it as the work lands, instead of
  being maintained as a live artifact. The lineage runs
  back to Rittel's wicked-problems argument (Rittel & Webber 1973)
  that for open-ended design, the argument itself is the record.
- **Premortem.** Assuming an outcome has already happened improves
  reason generation
  ([Mitchell, Russo & Pennington 1989](https://onlinelibrary.wiley.com/doi/abs/10.1002/bdm.3960020103);
  popularized as the premortem by Gary Klein). The skill runs one at
  convergence, and its outputs become tripwires on the record.
- **Tripwires.** Predetermined signals that force reconsideration are
  a named prescription in Heath & Heath, *Decisive* (2013), countering
  escalation of commitment. Every keep-or-change verdict here ships
  with them.
- **Status-quo bias.** Incumbents win by default beyond their merits
  (Samuelson & Zeckhauser 1988); the skill therefore argues direction
  on merits before migration cost enters the exchange — while still
  requiring cost to arrive before the verdict is recorded.
- **Vague words diverge across readers.** Verbal likelihood terms are
  interpreted with large between-person variance (Wallsten, Budescu et
  al., 1980s work on the vague meanings of probability terms); the
  skill's Language rule extends the same caution to qualitative
  quality wording, preferring explicit quantities, bounds, and
  invariants.
- **An LLM as arguing counterpart is starting to be tested.** An
  LLM-powered devil's advocate improved appropriate reliance in group
  decision-making in
  [Chiang et al. 2024](https://dl.acm.org/doi/10.1145/3640543.3645199)
  (the effect held for the interactive advocate challenging the AI's
  recommendation; a variant challenging the majority opinion showed no
  reliance effect);
  LLM assistance has also been observed to homogenize ideas
  ([Anderson et al. 2024](https://dl.acm.org/doi/10.1145/3635636.3656204)),
  which is part of why the skill requires surfacing genuinely distinct
  shapes and building on your framings rather than converging on
  the model's first default.

### Watched, not acted on

Because the mapping does not govern the skill, a mechanism the
literature contradicts is recorded here rather than changed by that
contradiction. These came from an August 2026 mapping. Citations are
given as the mapping returned them, without links, because they were
verified as citations but their findings were read through the
mapping rather than from the sources directly.

- **"Giving arguments state produces an unreadable graph" is
  unsupported.** No verified source claims argument graphs become hard
  for humans to read at scale. What does hold is relation-type
  instability: Shipman & Marshall 1999 report typed links "rarely
  used, and when they were, seldom used consistently", and gIBIS's own
  field data shows roughly 1.06 links per node with nine relation
  types available and free. Every canonical scheme in fact does the
  opposite of this skill — gIBIS ships nine typed link types, DRL
  around sixteen, and Dung 1995 defines its semantics purely over the
  attack relation — so the counter-position is the mainstream. The
  practice stands, on the structural reason the skill states.
- **"Consolidating costs latency and nothing else" is contradicted.**
  Fitz et al. 2019 (n=237) found batching notifications three times a
  day improved mood and productivity, hourly was null, and
  never-reporting was negative — so granularity is the active
  ingredient rather than batching. Iqbal & Bailey 2008 found highly
  relevant content warrants an *earlier* breakpoint, which is the
  opposite of waiting for the slowest parallel arm.
- **The prior-art gate selects for the corpus's most harmful
  property.** Chan et al. 2011 found far-field examples beat near-field
  on novelty (d=0.56) but neither beat a no-example control; the
  damage came from commonness, where common examples searched less of
  the design space (d=−1.03) and produced fewer concepts (d=−0.76).
  The gate returns shipped, well-known mechanisms by construction. Nor
  does reading critically protect: Smith 1993 found instructions to
  diverge did not reduce conformity, and Linsey 2010 found design
  faculty fixate as much as novices. Against all of that, the gate's
  best-supported element is that it runs only on your word —
  Siangliulue et al. 2015 (N=97) found on-demand examples produced the
  most novel ideas (p=.0034), while fixed-interval delivery produced
  fewer ideas than no examples at all.

Known gaps, stated honestly: this mapping covers the mechanisms above
plus those in the list before it, and a majority of the skill's
mechanisms have never been mapped at all — the loop's middle steps,
most of Decision authority, the Language rules and the state machine
among them. The one-sentence mechanism compression in keep-or-change
has no direct literature support (it is congruent with root-cause
practice, but unvalidated). Linked citations come from a research pass
followed by an independent per-claim fact-check; characterizations
compress the sources, and the fact-check's corrections are folded into
the wording above.

## Origin

Extracted from a project-specific skill developed on a game project,
where the argued-exchange mode repeatedly out-performed narrowing
elicitation on open design questions (one structural pivot found, one
harmful implicit behavior removed). Project-specific machinery
(document conventions, engine invariants) was stripped; the loop, the
thread-and-state machinery, the decision-authority rules, the stop
signals, the language rules, and the keep-or-change discipline are the
parts that generalize.

It was forked into this crate from designing-together 0.6.0, and
unified with the rest of the installed workflow.
