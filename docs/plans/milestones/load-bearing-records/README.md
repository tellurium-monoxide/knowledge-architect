# Load-bearing records: a design head only where an entry test passes, and the shipped text commented

## Status and audience

This is the milestone document of the work that makes the workflow record fewer design heads, each
admitted by a stated test, protects the intent that then stays in comments and commit messages,
gives bounded work a path inside the design skill, and lets the shipped text of agent-skills carry
comments that are stripped at build and checked by the walk. It is written for a session that did
not witness the discussion.

- It leaves the repository in the commit that completes the harvest of its last slice.
- Where it and a design home disagree, the design home wins.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where a point below needs the owner's word and the owner is absent, the work does not proceed on
  that point.

The discussion ran in one session of Claude Code. Its transcript is the file of that session in the
harness's project directory, `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/`,
found by its opening message, "I'd like to fix the the-setup-snippet-is-unchecked issue." The
discussion itself begins at the owner's message that opens "First, I think we should fix the
agentic instructions." and ends at the one that opens "Tables approved, keep all tripwires". This
document was assembled from that transcript. Rounds are numbered R1 to R7 by the owner's
messages; R8 is the owner's message after this document's first reviews, which ruled on its
defaults, and R9 the one after the last review, which ruled on #record-volume's condition: R3 holds two owner messages, because a reply was cut by a connection error and the owner
wrote "resume"; R4 holds the owner's message, an interruption, and the owner's message typed
after it.

## How the work is done

The procedure for each slice is §7 of `knowledge-architect-planning`, restated below from the
installed skill. That skill is its home; where the two disagree, the skill wins.

#### Working a slice, and the work of a spec (§7 of the installed planning skill)

The milestone document restates this procedure with a pointer to this skill, so a cold session
finds it there. The work of a spec follows it too, as its last paragraph says.

1. **Ground**: the Component's `CLAUDE.md`, its design home, its rejected alternatives; then the
   milestone document entire, then the slice's spec. The issues and the tripwires, of every anchor,
   are read at the audit.
2. **The design audit.** Read the slice's entry and every decided shape it depends on against the
   code as it stands and against the design homes. List every gap: a shape the code refutes, a
   passage of a specification the entry did not read, a name the entry uses that the code does not
   have, a consequence the entry did not see, and **a standing entry the slice's planned code bears on**:
   a tripwire whose firing condition, or a `deferred` issue whose trigger, the planned code meets,
   and an issue of any kind the slice's code touches, closes, makes worse or depends on. Dispatch the
   search as the description of `knowledge-architect-standing-entry-searcher` says, with the slice's
   spec and the milestone document as the work and, as seeds, the decisions the milestone document
   lists under "What is already decided" and the decisions and goals the slice's spec cites; for a
   spec, the spec is the work and its own "What is already decided" gives the seeds. Read
   whole, with `cargo klarch show`, every entry the search returns, and judge it against the slice;
   never act on an entry from the reason the search gave. A firing found at the audit is ruled before the code is written, where one
   found only by the review of the harvest reopens the harvest. Sort each gap:
   - **Applied in place.** The gap has one answer the document's decisions already imply, or is a
     choice among shapes the document rules out all but one of. Apply the answer in place, in the
     milestone document or the slice's spec, wherever the shape it changes is written. Commit the
     amended documents alone, with a subject of the shape `The <slice> design audit, applied in
     place: <n> gaps, none reopening a discussion`, and a message listing each finding: the gap, the
     answer and the decision it follows from. Earlier audits are found with
     `git log --grep='design audit'`.
     **An answer that widens or narrows a ruling of the owner, or adds an obligation to one, is a
     scope change even when it is the one answer the document implies.** It is applied with the
     others, and also listed in the milestone document as a default awaiting the owner, who rules
     on it at the audit; the implementation of that point does not start before the ruling.
   - **A change to the slices.** An audit that finds the work needs another slice, or a slice split
     or reordered, writes each new slice's spec with its harvest row, adds the slice to the
     implementation sequence and the order rationale, and moves into its spec the design it takes
     from other slices, so that each shape keeps one home. It changes a sequence the owner ruled on, so it is a scope
     change, listed and ruled as above.
   - **Load-bearing.** The gap is material, or is a choice between two shapes neither of which the
     document rules out, or needs a ruling the document marks as the owner's. Record it in the
     slice's spec as open at the audit, with the discriminating fact, stop the slice, and open a
     design session with the owner under `knowledge-architect-design`. **The session's depth
     follows the gap.**
     - A choice among shapes that can be stated in full, each with its consequence, is put to the
       owner in one message, with a default, where reversing it touches none of the four things the
       design skill names for the cost of reversal; several such gaps go in one question. The slice's documents exist, so the ruling
       is not left to a commit message as that path would leave it: it is written in place in the
       milestone's documents by the rule of §4, as a thread with the owner's words verbatim, like
       the audit's other answers, and the audit's commit lists it among its gaps. It owes no new
       §8 review, since it changes no decided shape.
     - A gap that defeats a reason, a premise or a criterion an approved thread rests on needs the
       full session. Its converged design goes into the milestone's documents by the rule of §4,
       and owes §8's reviews.

     The slice resumes from the ruling or the converged design.
3. **Claims, tests, implementation, gates, commits**, per the project's development procedure, in
   as many commits as the session judges the work needs. The commits name how each claim's test was
   shown to fail against a wrong implementation, and say of any claim whose test cannot yet do so
   why not.
4. **Review before the merge**, per `knowledge-architect-review`. A repair is a further commit,
   or folded where that skill says. A finding not repaired becomes an issue entry.
5. **The report**: the landing commit reports on each acceptance criterion judged at this slice,
   by its identifier in plain text, beside a citation of the milestone document (§6).
6. **The harvest**, per the harvest row of the slice's spec: the decisions and the losing
   alternatives under `knowledge-architect-decision-recording`, then the tripwires and the issues
   under `knowledge-architect-issue-tracking`. The row names what is judged; the tests of
   `knowledge-architect-decision-recording` decide whether each decision and each alternative
   earns an entry, and they govern where the two disagree: an item of the row the tests exclude is
   named in the harvest's commit, with the test it fails. A decision harvested from a thread takes
   the thread's slug, unless the slug misdescribes the approved decision: the entry then takes a
   slug that names it, and the slice's harvest row states the pair, per
   `knowledge-architect-decision-recording`. A tripwire names the head that harvested its
   decision, so the head is written first. Where a design home is a directory, a new subdocument is
   linked from its README. **The harvest is reviewed before the merge**, per
   `knowledge-architect-review`, on the decision-record, routing and standing-state axes, and by
   the transcript reviewer where the transcripts are available: it writes the record those axes
   judge, so the review of point 4 cannot see it.
7. **The slice's spec leaves** in the commit that completes its harvest, as in §9. What crosses slices stays in the
   milestone document, amended in place where the landing changed it.

**The work of a spec** takes the same points once, for the whole spec, on its one branch:

- point 1, the grounding;
- point 2, the design audit, only when the work does not start in the session where the discussion
  converged, or when commits other than the spec's own have landed on the main branch since the
  spec was written. Otherwise the session goes straight to the work, since the design was read
  against the tree as it stands;
- point 3, the claims and checks of each step, in the commits the session judges right;
- point 4, the review, once, before the merge;
- points 5 and 6, the report and the harvest, in the commits that land the work, the harvest
  reviewed before the merge;
- point 7, the deletion of the spec, in the commit that completes its harvest, as in §9.

Where a point names the milestone document or the slice's spec, the work of a spec reads the spec:
its defaults, its threads, its harvest row, and an audit's commit subject of the shape `The <spec>
design audit, applied in place: …`.

## Names

| name | what it names |
| --- | --- |
| the entry tests | the tests of §2 of the installed `knowledge-architect-decision-recording` that decide whether a decision earns a design head; the current text is in `path@agent-skills@content/skills/decision-recording/SKILL.md` |
| T1, T2, T3 | the three entry tests, in their order in that §2; T2old and T2new are the second test's wording before and after slice 1 |
| TM | the test of `design@agent-skills@instruction-record-is-minimal`, which agent-skills applies to a decision about the installed text |
| the bounded path | the shortcut inside the design skill decided by #bounded-path-in-design |
| a `%%` line | a line of a file under `path@agent-skills@content/` whose first two characters are `%%`: a comment for this repository's maintainers, stripped by the build |
| a delivery substitution | a row of the table that the agent-skills build script fills in the shipped text, decided by #delivery-substitutions |
| the walk | the set of files `cargo klarch check` reads, which the `exclude` list of `path@knowledge-architect@knowledge-architect.toml` narrows |
| the walk probe | the run of `cargo klarch check` on a scratch worktree of origin/main with the line `"crates/agent-skills/content",` deleted from that `exclude` list, taken in R5 |
| the audit | the classification of all 178 design heads against the entry tests by four read-only subagents, in R3 |
| the snippet branch | the branch `setup-snippet-compiled`, pull request #58 on GitHub, which closes the issue on the unchecked setup snippet and adds the placeholder line `{{snippet:<file>}}` to `path@agent-skills@build.rs`; it merges between slice 1 and slice 2 |
| the cleanup issue | the issue entry opened at slice 1's harvest for the heads the audit found unowed, per #record-audit |

## What the work is

What existed at each site the work touches when this document was written, and what slice 1 has
changed since:

- **The entry tests** were three numbered tests in §2 of the decision-recording skill, T2 reading
  "it **constrains work that has not been built**", and no design head recorded them. Slice 1
  rewrote them and recorded them in `design@agent-skills@a-head-is-owed-by-an-entry-test`.
- **The primer's "Intent and claims" section**, in `path@agent-skills@content/PRIMER.md`, knew two
  classes until slice 1 added a third, per `design@agent-skills@local-intent-binds`: a design home, which is authority, and "a claim about the code as it stands (a scoped
  `CLAUDE.md` invariant, a doc comment, a name, an issue's diagnosis)", which is verified. An
  inline comment is a home in the primer's knowledge table ("why a piece of code is shaped the way
  it is"), and no class said whether it was intent or claim. The root `path@knowledge-architect@CLAUDE.md`
  restates the classes under "Verify before relying on anything", three since slice 1.
- **The design skill**, `path@agent-skills@content/skills/design/SKILL.md`, had a section "When
  NOT to use" that classified a bounded problem and left the next step to the owner. Slice 1
  replaced it with the bounded path, and its head with `design@agent-skills@bounded-path-in-design`. Its keep-or-change section already says that
  behaviour argued by no document and no observed use is "an implementation coincidence" to raise
  with the owner, and that "a commit message that argues it counts".
- **The decision-record reviewer**, `path@agent-skills@content/agents/decision-record-reviewer.md`,
  judges in its §2 whether each decision earned its entry, in both directions.
- **The search before the work** ran at the grounding of a design discussion and at a design
  audit, and work neither designed nor planned sent none. Since slice 1 it runs at the grounding of
  bounded work too, per `design@agent-skills@standing-entries-searched-before-the-work`.
- **content/ is out of the walk**: `path@knowledge-architect@knowledge-architect.toml` lists it
  under `exclude`. The walk probe found 36 findings with it back in the walk: 30 generic paths of
  documents every Component carries, 3 paths of the plans directory, 2 paths of the setup skill's
  Rust section, and the primer's import line at the setup skill's line 152, read as a malformed
  reference.
- **The agent-skills build script** generates `FILES` from content/ with `include_str!`. On the
  snippet branch it also replaces a line `{{snippet:<file>}}` with a file of `snippets/`.

Outside this work:

- **The cleanup of the existing heads.** The audit found 16 heads that no test admits, plus the
  heads the owner named in R4. Removing or splitting them is the cleanup issue, opened at slice 1's
  harvest; its agent-skills part waits for slice 2, since `%%` lines are where that content can go.
- **A check that the shipped text holds no reference to an entry.** It is
  `issue@agent-skills@shipped-text-is-reference-free-mechanically`, which slice 2 rewrites.
- **Generic names for the homes whose shape may vary** ("the design home"), which the owner named
  in R7 as the better fix for a coupling raised in R6. That coupling does not exist: see the first
  thread #shipped-text-entry-references-only.
- **The owner's global CLAUDE.md**, which sends bounded problems to another plugin's skill. It is
  the owner's private configuration; the owner said in R2: "I'm gonna rewrite my global CLAUDE.md
  to fit too."

## What is already decided

The decisions this work rests on and does not argue again:

- `design@agent-skills@design-home-is-built-intent`: a design home holds built intent and prevails
  over the code. #local-intent-binds places comments below it.
- `design@agent-skills@harvest-after-implementation`: a decision is recorded when its work lands;
  one with no implementing work is recorded when made. The decided T2 keeps that case admitted.
- `design@agent-skills@instruction-record-is-minimal`: TM stays the test for decisions about the
  installed text.
- `design@agent-skills@new-or-reshaped-head-needs-design`: a decision that creates or contradicts a
  head goes through the design skill. The bounded path is inside that skill, so it changes nothing
  here.
- `design@agent-skills@capabilities-not-structure` and `design@agent-skills@additions-need-real-use`:
  the additions are capabilities, and each rests on this session's instance and the owner's named
  lack.
- `design@agent-skills@no-external-handoff`: the bounded path names no skill outside the installed
  set.
- `design@core@components-carry-the-same-documents`: every Component carries the same documents,
  which is why `path@*@docs/goals.md` resolves in every conforming project.

The recorded decisions the work reverses or rewrites, each with every text that
`cargo klarch show` lists as referencing it, and the slice that updates it:

| decision | change | referencing texts | updated by |
| --- | --- | --- | --- |
| the head now `design@agent-skills@bounded-path-in-design`, the bounded-problem branch before slice 1 | reversed by the bounded path | only the issue that asked for a skill for bounded problems, which this document's commit closes | slice 1, rewritten in place under a new slug |
| `design@agent-skills@standing-entries-searched-before-the-work` | bounded work routed through the design skill is searched at its grounding | `design@agent-skills@conformance-before-every-merge`; `tripwire@agent-skills@search-missed-before-the-work`; `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | slice 1 |
| `design@agent-skills@conformance-before-every-merge` | its paragraph that parks a search before undesigned work is answered: the design skill hosts it for bounded work | `design@agent-skills@standing-entries-searched-before-the-work`; `issue@agent-skills@a-design-issue-s-re-entry-point-is-read-by-no-review`, whose sentence "Work that is neither designed nor planned meets it at no step" stops holding for bounded work; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | slice 1 |
| `design@agent-skills@primer-content` | the intent-and-claims rule gains a class, within the head's title | `path@knowledge-architect@CLAUDE.md` (its restatement of the primer's content); `path@agent-skills@docs/rejected-alternatives.md` | slice 1 |
| `design@agent-skills@shipped-text-is-reference-free` | narrowed to "no reference to an entry", renamed | `path@agent-config@skills/klarch-release/SKILL.md` (its shipped-text step); `path@agent-skills@CLAUDE.md`; `issue@agent-skills@shipped-text-is-reference-free-mechanically` | slice 2 |
| `issue@agent-skills@shipped-text-is-reference-free-mechanically` | rewritten: the walk exclusion goes, and the check applies to the installed copies | `path@agent-config@skills/klarch-release/SKILL.md`; `path@agent-skills@CLAUDE.md`; `path@agent-skills@docs/design.md` (the head above); `path@knowledge-architect@docs/design.md` (the list of `design@knowledge-architect@stays-at-zero-x`) | slice 2 |
| `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | its re-entry was the design of a skill for bounded problems, which this discussion was | `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | slice 1, restated |

The commit that adds this document closes the issue that asked for a skill for bounded problems, which
this milestone schedules, and repairs the texts that cited it:
- the two mentions in `path@agent-skills@README.md`;
- the head of the bounded-problem branch, now `design@agent-skills@bounded-path-in-design`;
- `design@agent-skills@conformance-before-every-merge`;
- the re-entry of `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`;
- the list under `design@knowledge-architect@stays-at-zero-x`.

Each cited this milestone until slice 1 rewrote it to the head or the text that slice built; the
list under `design@knowledge-architect@stays-at-zero-x` cites it until this document leaves.

## Criteria

### A later session can tell what it may change and what a change costs, without re-deriving the argument `##tell-what-may-change`

Binding, from `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Met by
#entry-test-locality, #local-intent-binds and #record-volume.

### A departure from recorded intent is caught by a check or a review before it merges `##departure-caught`

Binding, from `goal@knowledge-architect@agents-work-without-drift`. Met by #local-intent-binds,
through the decision-record reviewer's new predicate, and by #shipped-text-line-comments, whose
comments the walk checks.

### An agent never substitutes its judgement for the owner's ruling, wherever the ruling is recorded `##owner-rules`

Binding, from `goal@knowledge-architect@the-owner-decides`. Met by #bounded-path-in-design, which
waits for the owner's word, and by #local-intent-binds, under which a head prevails over a comment.

### No structure the work does not need `##no-needless-structure`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`, and from
`design@agent-skills@capabilities-not-structure` as a presumption. Met by #bounded-path-in-design,
one message rather than a procedure, and by #delivery-substitutions, limited to literals that must
ship verbatim.

### An addition to the installed text rests on real use `##real-use`

Binding as a presumption, from `design@agent-skills@additions-need-real-use`. Met: the head this
session wrote on the snippet branch and later removed, the owner's named lack in R1, and the audit.

### A design home stays small enough to read at every grounding, for an agent and for a human overseer `##readable-design-home`

Weighed; stated by the owner in R1, from no goal. Met in part by #record-volume and
#entry-test-locality. The removal of existing heads is the cleanup issue, outside this work.

## Threads

### A head is owed when the same reason must stand at more than one site, or at none: the decided T2 `##entry-test-locality`

- **Proposed:** the idea is the owner's, R1 (a1, a2, a4); the shape and the slug are the
  assistant's, R1.
- **States:** R1 new, as "no site can carry it" (a15); R2 in-discussion, revised to "the same
  statement" after the owner's objection (a24, a28, a29); R3 approved; R4 a material finding
  (a43, a44, a50) and a refined wording, kept approved by default; R5 approved with the
  refinement; R7 the checkpoint table approved.
- **Arguments:** a1, a2, a4, a10, a15, a24, a28, a29, a30, a37, a38, a43, a44, a50, a57.
- **Closed by:** R3: "entry-test-locality and local-intent-binds look correct to me." R5:
  "entry-test-locality: agreed with the refinement. This was already how I interpreted the test
  before this round, but now I see it was a purely subjective interpretation." R7: "Tables
  approved, keep all tripwires".
- **Shape:** the decided design of slice 1. **Harvest:** the head of #record-volume, slice 1.
- **Relations:** for agent-skills, TM stays the test (a30). Guarded by P2.

### Local intent binds as a presumption, below a head: goal, head, comment, commit, unrecorded code `##local-intent-binds`

- **Proposed:** the idea is the owner's, R1 (a5); the shape is the assistant's, R1.
- **States:** R1 new; R2 in-discussion, with the order and the rule for a conflict with a head,
  answering the owner's question (a25); R3 approved; R7 the checkpoint table approved.
- **Arguments:** a5, a11, a16, a17, a18, a19, a25, a31, a32.
- **Closed by:** R3: "entry-test-locality and local-intent-binds look correct to me." R7: "Tables
  approved, keep all tripwires".
- **Shape:** the decided design of slice 1. **Harvest:** a head of its own, slice 1.
- **Relations:** the transcript reviewer is unchanged, argued in a18. Its weakness in projects
  whose comments the checker does not read is `issue@core@references-are-read-in-markdown-and-rust-only`
  (a19). Guarded by P3.

### Bounded work takes a path inside the design skill, after its grounding `##bounded-path-in-design`

- **Proposed:** the owner, R1 (a7); the shape is the assistant's, R1.
- **States:** R1 new; R2 approved; R7 the checkpoint table approved.
- **Arguments:** a7, a12, a13, a20, a21, a22.
- **Closed by:** R2: "bounded-path-in-design: I agree with that. The existing bounded skill issue
  would be closed by this. I'm gonna rewrite my global CLAUDE.md to fit too."
- **Shape:** the decided design of slice 1. **Harvest:** the head of the bounded-problem
  branch, rewritten in place as `design@agent-skills@bounded-path-in-design`, slice 1.
- **Relations:** reverses the bounded-problem branch, whose head is now `design@agent-skills@bounded-path-in-design`. Supersedes the owner's
  earlier word in the closed issue, "Ultimately, I'd like to make my own skill for this use case."
  (a13). Hosts the search that `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`
  asks for (a12). Absorbs #record-test-at-classification. Guarded by P1.
- **The lead of the closed issue:** The issue that asked for a skill for
  bounded problems, closed by the commit that adds this document, carried a lead, from a review of
  the project the design skill came from: "a second defensible shape is nameable for nearly any
  request", which would make the design skill's open-space test classify almost every problem as
  open, and leave the bounded case nearly unreachable. No session had shown it. Its last state is
  `path@elsewhere@crates/agent-skills/docs/open-issues/a-skill-for-bounded-problems.md`, read with
  `git show` at the parent of the commit that adds this document. It also said that the lead, untested by then, is carried to an issue of its own about
  the design skill. Ruled: slice 1 opens `the-open-space-test-may-admit-every-problem`, a
  `question`, with that lead. It is the opposite failure to P1, and the bounded path's third
  condition is where it would show. R8: "Agreed on the three items awaiting my ruling."

### Fewer heads: a decision earns one only when an entry test passes, and one head records the tests `##record-volume`

- **Proposed:** the owner's question, R1 (a6); the default is the assistant's, R1 (a14, a23).
- **States:** R1 new; R2 presumed-settled on the owner's lean, with the shape the owner proposed
  (a26, a33); R3 approved with a condition; R4 it stands on the audit's outcome; R7 the checkpoint
  table approved.
- **Arguments:** a6, a9, a14, a23, a26, a33.
- **Closed by:** R2: "record-volume: I lean to think that fewer is better too, here. If we want to
  adhere to this principle in the long term, I think it has to be recorded as a design head." R3:
  "record-volume: approved with that shape. But the result of the audit could still change that."
- **Shape:** the decided design of slice 1. **Harvest:** a new head of agent-skills recording the
  entry tests and the principle, under a slug that names the decision, slice 1.
- **Relations:** the audit may reopen it, as the owner's condition in R3 and the checkpoint row of
  R6 say. Where the owner read the sample, it found over-recording below the threshold (a36). For
  agent-skills its outcome is unconfirmed (a65): the cleanup issue judges each head in full, and
  its result could reopen this thread. `issue@agent-skills@the-retrospective-counts-no-review-cost`
  records the instrument that would measure what the design homes cost a session (a14).
- **The condition, discharged for the code Components and open for agent-skills,** where the
  cleanup issue's judgement of each head can reopen the thread; ruled in R9: "(b) is right, generic names overtaken." and "Here, both a and b would be fine by me. I do not expect the cleanup to reopen this, especially because the agent-skills component is kind of an exception in the way it handles design decisions." The owner's R8
  words, which open with a question and give a lean: "Anyway, the audit did not surface anything that would change
  my mind on this. I still lean to think that lower design record volume is better, as long as no
  critical intent and arguments are lost, and the audit did not suggest dropping anything I found
  critical in the code components here (and we refined the tests once more after it). For the
  agent-skills component, this was less clear, but that is to be expected given the wildly
  different scope and structure of this component."

### No new wording in the design and planning skills: the entry tests move to classification `##record-test-at-classification`

- **Proposed:** from the owner's diagnosis, R1 (a3); the position is the assistant's, R1 (a8).
- **States:** R1 new; R2 superseded, absorbed by #bounded-path-in-design.
- **Arguments:** a3, a8, a20.
- **Closed by:** R2: "record-test-at-classification: agreed."
- **Shape and harvest:** those of #bounded-path-in-design.

### The audit of every head: below the threshold where the owner read, agent-skills unconfirmed, one cleanup issue `##record-audit`

- **Proposed:** the owner, R2 (a27); the shape, the pre-commitments and the threshold are the
  assistant's, R2 (a34, a35).
- **States:** R2 new; R3 approved and run, with the threshold presumed at one head in five; R3
  results; R4 the owner's reading and two corrections; R5 the owner's reading of agent-skills
  withdrawn as a ruling (a58); R6 presumed-settled; R7 the checkpoint table approved.
- **Arguments:** a27, a34, a35, a36, a44, a45, a51, a52, a58, a65.
- **Closed by:** R3: "Let's run the audit. I think the shape you proposed is good. For the samples I
  look at, in addition to all the "none" verdicts and a sample of "passes" you provide, I'll sample
  a few myself among "passes" that I chose myself too." R7: "Tables approved, keep all tripwires".
- **Shape:** under Decided design of slice 1, the cleanup issue. **Harvest:** the cleanup issue,
  slice 1.
- **Relations:** produced #t1-consumed-interface, #t3-external-behaviour and the finding of R4
  against #entry-test-locality.

### T1 covers any interface others consume `##t1-consumed-interface`

- **Proposed:** the assistant, R3, from the audit.
- **States:** R3 new; R4 approved; R7 the checkpoint table approved.
- **Arguments:** a39, a40, a51.
- **Closed by:** R4: "t1-consumed-interface and t3-external-behaviour approved."
- **The alternatives' first test:** §6 of the decision-recording skill
  states, for a losing alternative, "a signature crossing the boundary of a separately built unit,
  or a serialized format". The ruling widened the decision's T1 only. Ruled: slice 1 leaves §6 as it
  stands. R8: "Agreed on the three items awaiting my ruling."
- **Shape:** the decided design of slice 1. **Harvest:** the head of #record-volume, slice 1.

### T3 counts an external tool's behaviour, documented or measured `##t3-external-behaviour`

- **Proposed:** the assistant, R3, from the questions of two audit agents.
- **States:** R3 new; R4 approved; R7 the checkpoint table approved.
- **Arguments:** a41, a42.
- **Closed by:** R4: "t1-consumed-interface and t3-external-behaviour approved."
- **Shape:** the decided design of slice 1. **Harvest:** the head of #record-volume, slice 1.

### The shipped text carries `%%` line comments, stripped at build, and content/ is back in the walk `##shipped-text-line-comments`

- **Proposed:** the owner, R4: HTML comments first (a46, a47, a48), then a line prefix (a49); the
  slug and the shape are the assistant's, R4 (a53, a54). Option (a), content/ back in the walk, is
  the owner's, R5 (a59).
- **States:** R4 new, with checking undecided (a55, a56); R5 in-discussion, a material finding of
  36 repairs (a60), default (a) (a63, a64); R6 approved; R6 a second finding, 11 repairs that are
  not mechanical (a69, a70); R7 the checkpoint table approved.
- **Arguments:** a46, a47, a48, a49, a53, a54, a55, a56, a59, a60, a63, a64, a68, a69.
- **Closed by:** R5: "shipped-text-line-comments: agreed with proposed shape and %% as marker." R6:
  "Go with (a), and the narrowing is approved." R6: "In the Rust section, I think placeholders
  would be better than convoluted sentences (especially tools/xtask, which might vary per
  project)". R7: "Tables approved, keep all tripwires".
- **Shape:** the decided design of slice 2. **Harvest:** a new head of agent-skills, slice 2.
- **Relations:** spawned #shipped-text-entry-references-only and #delivery-substitutions. Rewrites
  `issue@agent-skills@shipped-text-is-reference-free-mechanically`. Depends on the snippet branch.
  P4 becomes an acceptance criterion of slice 2.

### The shipped text cites no entry; a path every conforming project holds may be a reference `##shipped-text-entry-references-only`

- **Proposed:** the assistant, R5, as a consequence of option (a).
- **States:** R5 new; R6 approved; R6 the finding of a69 and a70; R7 the checkpoint table
  approved, and an issue asked for the coupling.
- **Arguments:** a61, a62, a67, a69, a70, a75.
- **Closed by:** R6: "Go with (a), and the narrowing is approved." R7: "Tables approved, keep all
  tripwires, Open an issue for the path@*@docs/design.md potential problem on migrating the shape
  of one component here."
- **Shape:** the decided design of slice 2. **Harvest:** `design@agent-skills@shipped-text-is-reference-free`
  rewritten in place under a new slug, slice 2.
- **The finding of R6 was refuted** (a69, a70), after R7: R6 stated that
  `path@*@docs/goals/` "would be a finding here", and that moving a Component to the directory
  shape would dangle every `path@*@docs/design.md` of the shipped text. Both are false.
  `design@core@reserved-anchors` accepts the generic anchor "when the path is one of the required
  document names in any of its shapes", "so naming a shape no component uses yet is legitimate". A
  run confirmed it after R7: a scratch file citing `path@*@docs/goals/`, `path@*@docs/design/`,
  `path@*@docs/tripwires/`, `path@*@docs/goals.md`, `path@plans@README.md` and `path@plans@specs/`
  gave `PASSED: no findings` under `cargo klarch check`. So 33 of the 36 repairs are mechanical: 30
  as `path@*@<path>` in both shapes, 3 as `path@plans@<path>`. Ruled: slice 2 writes both shapes as
  `path@*@<path>` references, no form "or its directory" is used, and no issue is opened for the
  coupling. The owner's generic names ("the design home", defined once in the primer) remain
  available as an improvement of wording; on the owner's word, they become an issue. R8: "Agreed on the three items awaiting my ruling."

### Literals the checker would misread are delivery substitutions, filled at build `##delivery-substitutions`

- **Proposed:** the owner, R6 (a66); the shape and its limit are the assistant's, R6 (a71, a72,
  a73, a74).
- **States:** R6 new, then presumed-settled in the checkpoint; R7 the checkpoint table approved.
- **Arguments:** a66, a71, a72, a73, a74.
- **Closed by:** R7: "Tables approved, keep all tripwires".
- **Shape:** the decided design of slice 2. **Harvest:** none in a design home: the decided T2 sends
  it to the build script's comment, with the reason stated per row (see slice 2's harvest).

## Arguments

Each argument's key words are quoted verbatim from the transcript. "O" is the owner, "A" the
assistant.

### A decision on localized code is carried by its comments, and its reversal cost is easy to evaluate `##a1`

R1, owner; bears on #entry-test-locality, #record-volume.

> the "decision" you wrote is bearing on a very "localized" part of the code, so the intent can easily be carried by local code comments. And its reversal is cheap (or the cost is easy to evaluate: the snippet would no longer be tested).

### Any design that tests the snippet solves the problem as well `##a2`

R1, owner; bears on #entry-test-locality.

> Also, any design that tests the snippet would solve the problem just as well.

### The design and planning skills may send agents to record without the decision-recording tests `##a3`

R1, owner; bears on #record-test-at-classification.

> But they might have stopped being followed, because both design and planning skill are saying to "record the decisions", maybe without having the agent go through the decision record skill and its tests.

### T2 is outdated: design homes hold built design, plan documents hold unbuilt design `##a4`

R1, owner; bears on #entry-test-locality (test 2 outdated).

> Since a recent planning skill rework, it has been decided that design records are supposed to carry "built design", while unbuilt design goes in plan documents.

### Unargued code could be judged superfluous and deleted, since code follows design `##a5`

R1, owner; bears on #local-intent-binds.

> Since the workflow is instructing that "code follow design", anything that is not argumented in design records could end up judged "superfluous" and deleted autonomously.

### More records bloat the design homes; fewer leave the rest to the agents' judgement `##a6`

R1, owner; bears on #record-volume.

> more decision records, which over time bloat the design docs and make them harder to review by human overseers of the agents, as well as harder to ground on for agents, or less decision records, only when load bearing, but anything not argued in there has to be judged by the agents.

### Whether work is bounded cannot be known without the design skill's grounding `##a7`

R1, owner; bears on #bounded-path-in-design.

> IMO, there is no way to determine whether a task is "bounded work that does not change the project's design" without going through the grounding steps of the design skill.

### The one instance held is the decision-recording skill loaded and its §2 skipped `##a8`

R1, assistant; bears on #record-test-at-classification (against the owner's diagnosis a3).

> So the one instance we hold is "§2 loaded and skipped", not "the design skill said record". I cannot claim a general mechanism from one instance.

### The entry tests have no design head `##a9`

R1, assistant; bears on #record-volume (the tests' head).

> The three tests for a decision exist only in the skill text, `decision-recording/SKILL.md`, from when the recording skills were first installed. Nothing in a design home argues them.

### Deleting T2 leaves a policy such as stays-at-zero-x with no admission test `##a10`

R1, assistant; bears on #entry-test-locality (rival "delete test 2" loses).

> That covers a policy, such as `design@knowledge-architect@stays-at-zero-x`. Delete test 2 outright and that policy case loses its admission test.

### Intent-and-claims does not say whether an inline comment is intent or claim `##a11`

R1, assistant; bears on #local-intent-binds.

> Yet intent-and-claims does not say whether such a comment is intent or claim. That gap is the real ground for your worry (3).

### Bounded work routed through the design skill gets the standing-entry search `##a12`

R1, assistant; bears on #bounded-path-in-design.

> Bounded work routed through the design skill's grounding gets the standing-entry search, and the design skill hosts it.

### The bounded path supersedes the owner's earlier wish for a skill of its own `##a13`

R1, assistant; bears on #bounded-path-in-design.

> that this reverses:** "Ultimately, I'd like to make my own skill for this use case." Your message today supersedes it.

### No instrument measures what reading the design homes costs `##a14`

R1, assistant; bears on #record-volume.

> No instrument measures what reading them costs.

### The snippet case under the first form of T2 earns no head `##a15`

R1, assistant; bears on #entry-test-locality (snippet case under the first form).

> Every site is reached, so no head is owed, which matches your ruling.

### A comment and a commit argument bind as a presumption, like a head `##a16`

R1, assistant; bears on #local-intent-binds.

> They bind as a presumption, like a head. A change that defeats them argues against the stated reason in its own commit message.

### Absence from the design home is not evidence that code is superfluous `##a17`

R1, assistant; bears on #local-intent-binds.

> Absence from the design home is not evidence that code is superfluous.

### The transcript reviewer already counts a comment and a commit message as durable outcomes `##a18`

R1, assistant; bears on #local-intent-binds (transcript reviewer unchanged).

> Its §3 already counts "a comment at the code, a commit message for what the table routes there" as a durable outcome.

### In a project whose comments the checker does not read, local intent is weaker `##a19`

R1, assistant; bears on #local-intent-binds (weakness).

> in a project not written in Rust or Markdown, a comment's references are not checked (`issue@core@references-are-read-in-markdown-and-rust-only`). So the local record is weaker there.

### Applying the entry tests at classification would have prevented the head written in this session `##a20`

R1, assistant; bears on #bounded-path-in-design, #record-test-at-classification.

> This applies the entry test at classification, before any work. That would have stopped my error: the classification would already have said "earns no entry".

### The design skill's description must reach sessions that start an issue fix `##a21`

R1, assistant; bears on #bounded-path-in-design (cost: delivery risk).

> The design skill's description must reach sessions that start an issue fix, which today never load it. That is the delivery risk.

### The owner's global configuration sends bounded problems elsewhere `##a22`

R1, assistant; bears on #bounded-path-in-design (owner's global configuration).

> Your global CLAUDE.md sends bounded problems to `superpowers:brainstorming`.

### A session changing local code reads its comment, not the design home `##a23`

R1, assistant; bears on #record-volume.

> The session that changes local code reads the comment at that code. A head for local code is read only if that session thinks to search the design home, and it adds to every grounding read.

### Copying a head into a comment at every site passes the first form of T2 `##a24`

R2, owner; bears on #entry-test-locality (objection).

> simply duplicating what would go in a decision head as a code comment everywhere would pass this test for nearly any design IMO.

### What happens when a comment conflicts with a design head `##a25`

R2, owner; bears on #local-intent-binds (question that moved the thread).

> what happens in case of conflict with a design head ? Which wins, which actions are taken...?

### The principle of fewer heads must itself be a head to last `##a26`

R2, owner; bears on #record-volume.

> If we want to adhere to this principle in the long term, I think it has to be recorded as a design head.

### An audit of every head against the tests would show a direction `##a27`

R2, owner; bears on #record-audit.

> Maybe an audit of all decisions recorded in the project against the three existing tests of the decision record skill, and also against the locality test you proposed, could be interesting and help us determine a better direction.

### The first form of T2 admits exactly the copy the one-home rule forbids `##a28`

R2, assistant; bears on #entry-test-locality.

> So the test admits exactly the copy that the one-home rule forbids.

### The checker cannot cite a comment, so a statement needed at several sites needs an entry `##a29`

R2, assistant; bears on #entry-test-locality.

> The reason is mechanical, not a matter of style: the checker cannot cite a comment. Only an entry can be the one home that every site points to with a reference.

### T2 says nothing useful for skill text, where TM stays the test `##a30`

R2, assistant; bears on #entry-test-locality (limit).

> So for agent-skills, test 2 says nothing useful. There the existing `instruction-record-is-minimal` does that job, and it should stay the test for that Component.

### A comment is part of the code, so a head prevails over it `##a31`

R2, assistant; bears on #local-intent-binds.

> A comment is part of the code, and `design-home-is-built-intent` already makes the design home authority over the code.

### Between a comment and a commit message, the comment is the current statement `##a32`

R2, assistant; bears on #local-intent-binds.

> Between a comment and a commit message, the comment wins. The comment is the current statement.

### The cost of every grounding read and every human review argues for fewer heads `##a33`

R2, assistant; bears on #record-volume.

> the argument for that principle, which is the cost of every grounding read and of every human review.

### Agents applying the tests measure how agents read the tests, not whether the tests are right `##a34`

R2, assistant; bears on #record-audit.

> An agent applying the tests measures how agents read the tests. It does not measure whether the tests are right.

### A qualitative threshold closes nothing: one head in five `##a35`

R2, assistant; bears on #record-audit (threshold).

> "Sizeable" is a qualitative word. It cannot close anything until you give it a number. I propose one in five heads.

### Under the new tests, 9% of heads are unowed, below the threshold `##a36`

R3, assistant; bears on #record-audit.

> Under the new set, 9% of heads are unowed overall, below the one-in-five threshold.

### Two audit agents found the current T2 does not discriminate `##a37`

R3, assistant; bears on #entry-test-locality (old test 2 does not discriminate).

> Two agents, the core one and the agent-skills one, reported on their own that the current test 2 does not discriminate. Read widely, "governs future work" passes every head.

### The revised T2 alone admits 25 heads `##a38`

R3, assistant; bears on #entry-test-locality (risk on the revised test).

> The revised test 2 now carries a lot of weight: 25 heads pass by it alone.

### Reversing a grammar or format head breaks every adopting project, the harm T1 exists to catch `##a39`

R3, assistant; bears on #t1-consumed-interface.

> Reversing one breaks every adopting project, which is the harm test 1 exists to catch. The alternatives' test 1 in the same skill already adds "or a serialized format".

### A grammar rule enforced in one module would fail every test under a strict T1 `##a40`

R3, assistant; bears on #t1-consumed-interface (rival loses).

> A grammar rule enforced in one module would then fail every test, even though it binds every adopting project.

### A measurement is as expensive to re-take as a documented reading `##a41`

R3, assistant; bears on #t3-external-behaviour.

> T3 exists because re-deriving such a reading is expensive, and a measurement is just as expensive to re-take.

### Cargo's documentation is silent on behaviours some heads depend on `##a42`

R3, assistant; bears on #t3-external-behaviour (rival loses).

> It loses because cargo's documentation is silent on several of the behaviours these heads depend on.

### roadmap-home has no reason to change; a path named in several places is self-consistency `##a43`

R4, owner; bears on #entry-test-locality (finding source).

> It is something that has no reason to ever change, and the path being named in multiple places is just basic self consistency.

### Only two rules of repo-layout deserve the record `##a44`

R4, owner; bears on #entry-test-locality, #record-audit.

> repo-layout's main body does not passes IMO, only the knowledge of not naming a component `knowledge-architect` and accepting directories for crates that are not named against published packages deserves the record.

### Two rules of how documents point at each other sit in the root but are about the installed workflow `##a45`

R4, owner; bears on #record-audit (misplaced root heads).

> a-past-sentence-is-rewritten and a-reference-claims-a-revisit look to be about agentic instructions, it seems weird to me that they are here.

### agent-skills heads serve as comments for the skill text, which cannot hold any `##a46`

R4, owner; bears on #shipped-text-line-comments (agent-skills is special).

> It is probably quite normal for it to hold a bit more design heads, because those are kinda serving as "comments" for what's inside the agent skills, which cannot be done otherwise.

### This repository's use of the workflow for its own text should not shape the installed files `##a47`

R4, owner; bears on #shipped-text-line-comments (do not cater; bears later on checking option 2).

> Here it is also used for the agent workflow text content, but I don't want to cater too much to this use case in the installed files.

### Comments in the shipped text would relieve the design homes and allow citing heads `##a48`

R4, owner; bears on #shipped-text-line-comments.

> This would solve in good part the problem where the design heads of the workflow get bloated with details that, in a code project, would be served well by a local code comment. It would also allow to cite the design heads or other items without polluting installed skill results.

### HTML comments carry too many drawbacks; line comments are saner `##a49`

R4, owner; bears on #shipped-text-line-comments (drawbacks of HTML comments).

> Right, I think using html comments might carry too many drawbacks.
> I think restricting this to line comments is saner for the long term too.

### The sites of roadmap-home and repo-layout repeat a name or a path, not a reason `##a50`

R4, assistant; bears on #entry-test-locality (material finding).

> What the two cases have in common: the sites repeat a **name or a path**. They do not respect a **reason** that a plausible change would defeat.

### spec-file-owns-its-items carries the citation form every adopting project writes `##a51`

R4, assistant; bears on #record-audit, #t1-consumed-interface (`spec-file-owns-its-items`).

> So the head carries the citation form, which every adopting project writes.

### The two root heads shape the installed text, so their home is agent-skills `##a52`

R4, assistant; bears on #record-audit (relocation of two root heads).

> Under §3 of decision-recording, a decision that shapes one Component belongs to that Component. These shape the installed text, so their home is agent-skills.

### The checker treats an HTML comment as parked text, so references in it are not read `##a53`

R4, assistant; bears on #shipped-text-line-comments (HTML comments are parked).

> Nothing in it is live: a slug in it defines nothing, and a reference in it is not read. So HTML comments would defeat your second purpose, citing heads from beside the instruction they explain.

### %% has no Markdown meaning, reads as a comment elsewhere, and cannot collide with Rust `##a54`

R4, assistant; bears on #shipped-text-line-comments (choice of `%%`).

> it has no Markdown meaning;
> it reads as a comment in Mermaid and Obsidian;
> it cannot collide with a Rust snippet, whose comments are `//`.

### Unchecked comments let their references rot `##a55`

R4, assistant; bears on #shipped-text-line-comments (checking option 1).

> The references rot silently, which defeats the purpose.

### A manifest feature for this would cater to this repository in the published checker `##a56`

R4, assistant; bears on #shipped-text-line-comments (checking option 2, citing a47).

> It is general, but it is a feature of the published crate built for this repository's use. You said you don't want to cater to this use case in what consumers receive.

### The owner's earlier reading of T2 was a subjective interpretation `##a57`

R5, owner; bears on #entry-test-locality.

> This was already how I interpreted the test before this round, but now I see it was a purely subjective interpretation.

### The owner's reading of the agent-skills sample was a skim, not a ruling `##a58`

R5, owner; bears on #record-audit (retraction of the agent-skills reading).

> Me agreeing with those verdicts was mostly skimming through the heads and judging mostly from titles. My word here was not to be taken  as a ruling on what is right.

### Take content/ back into the walk now, and update the issue on reference-free paths `##a59`

R5, owner; bears on #shipped-text-line-comments (option (a)).

> I think we can just take it back into the walk for now, and open an issue (or update the existing one about needing the ability to declare paths that must not carry any references)

### With content/ in the walk the check reports 36 findings `##a60`

R5, assistant; bears on #shipped-text-line-comments (finding: cost).

> Result: `FAILED: 36 findings above`, all of them in content/.

### A path every conforming project holds names no entry of this repository `##a61`

R5, assistant; bears on #shipped-text-entry-references-only.

> A `path@*@docs/goals.md` names no entry of this repository. It names a path that every conforming project holds, because the documents are required by `design@core@components-carry-the-same-documents`.

### Narrowing the rule to entries keeps the head's argument `##a62`

R5, assistant; bears on #shipped-text-entry-references-only.

> The repair narrows the rule to "no reference to an entry", and the head's argument still holds.

### Excluded and unchecked comments are allowed only beside an issue recording the missing form `##a63`

R5, assistant; bears on #shipped-text-line-comments (option (b)).

> But every citation written in a comment until then is unchecked, and the primer allows that only beside an issue entry that records the missing form.

### A test in xtask would rebuild the checker's reference collection on an unverified API `##a64`

R5, assistant; bears on #shipped-text-line-comments (default (a); rival (c) loses).

> (c) loses because it rebuilds, in one repository's test, the reference collection the checker already does, on an API whose fitness is unverified.

### The audit's conclusion is unconfirmed for agent-skills `##a65`

R5, assistant; bears on #record-audit.

> The audit's conclusion stands on the parts you did read: core and gates+xtask, including passes you picked yourself, and root. Over-recording is below the threshold there. For agent-skills, its share stays unconfirmed.

### Literals the checker would misread can be substituted at build or install, as documented exceptions `##a66`

R6, owner; bears on #delivery-substitutions.

> to avoid convoluted sentences (such as for the primer import line), the text could be applied using substitution at build or install time, as documented exceptions where we are avoiding the checker on purpose in order to facilitate delivery of the installed skills.

### The checker's syntax for generic per-Component paths is an upgrade over the current shape `##a67`

R6, owner; bears on #shipped-text-entry-references-only.

> For the generic per component paths, I think using the syntax of the checker is an upgrade over the current shape: it promotes the same kind of syntax as what the checker enforces in projects holding the installed skills.

### Placeholders are better than convoluted sentences in the Rust section `##a68`

R6, owner; bears on #shipped-text-line-comments (Rust section).

> In the Rust section, I think placeholders would be better than convoluted sentences (especially tools/xtask, which might vary per project)

### path@*@ claims every Component holds the path, so the directory shape is a finding here `##a69`

R6, assistant; bears on #shipped-text-line-comments, #shipped-text-entry-references-only (finding).

> `path@*@<path>` claims that every Component holds the path. So `path@*@docs/goals/` would be a finding here.

### A layout choice of this repository would force edits to the installed text `##a70`

R6, assistant; bears on #shipped-text-entry-references-only (coupling).

> If this repository ever moves a Component to the directory shape, every `path@*@docs/design.md` in the shipped text dangles.
> But a layout choice of this repository would then force edits to the installed text.

### Delivery substitutions do not vary by project, so they are filled at build `##a71`

R6, assistant; bears on #delivery-substitutions (build time).

> Filled at build time, not at install time,** for the same reason as the snippets: these literals do not vary by project.

### A substitution row exists only for text that must ship verbatim `##a72`

R6, assistant; bears on #delivery-substitutions (limit).

> a row exists only for text that must ship verbatim and that the checker would misread. A sentence that can be rewritten without harm is rewritten instead.

### A plain rewrite of the import line describes it instead of showing it `##a73`

R6, assistant; bears on #delivery-substitutions (rival: plain rewrite loses).

> For the import line it costs a sentence that describes the line instead of showing it, and an agent following the setup skill must copy that line exactly.

### Delivery substitutions reuse the snippet branch's mechanism `##a74`

R6, assistant; bears on #delivery-substitutions (dependency).

> It reuses the placeholder mechanism of the snippet branch, which is not merged yet.

### The better fix is generic names of homes, defined once with both shapes `##a75`

R7, owner; bears on #shipped-text-entry-references-only (preferred fix for the coupling; generic names).

> IMO, the better fix is to name those homes whose path may change into a generic sentence, like "the design home", "the goals home", "the component's design home". And potentially define those terms more clearly with both path alternatives in a single location (such as the primer).


## New names, in one place

```text
crates/agent-skills/docs/design.md        design slugs, written at the harvests:
                                          a-head-is-owed-by-an-entry-test   (slice 1, from #record-volume)
                                          local-intent-binds                (slice 1)
                                          bounded-path-in-design            (slice 1, replaces bounded-problem-branch)
                                          shipped-text-line-comments        (slice 2)
                                          shipped-text-cites-no-entry       (slice 2, replaces shipped-text-is-reference-free)
crates/agent-skills/docs/tripwires.md     tripwire slugs, slice 1:
                                          a-shortcut-decision-earns-a-head  (P1)
                                          a-head-verdict-is-overruled       (P2)
                                          a-comment-escalation-is-overruled (P3)
crates/agent-skills/docs/open-issues/     issue ids:
                                          heads-no-entry-test-admits                     (slice 1, the cleanup issue)
                                          the-open-space-test-may-admit-every-problem    (slice 1, from the closed issue's lead)
crates/agent-skills/build.rs              the comment marker "%%", and a table of delivery substitutions
                                          whose first row is {{primer-import}}           (slice 2)
crates/agent-skills/content/              placeholders in the setup skill's Rust section:
                                          <xtask-dir>, <workspace-root>                  (slice 2)
```

## Decided design

Slice 1, workflow-text, has landed: the design of #entry-test-locality, #local-intent-binds,
#bounded-path-in-design, #record-volume, #t1-consumed-interface and #t3-external-behaviour is in
`design@agent-skills@a-head-is-owed-by-an-entry-test`, `design@agent-skills@local-intent-binds` and
`design@agent-skills@bounded-path-in-design`, and #record-audit is
`issue@agent-skills@heads-no-entry-test-admits`. [shipped-text-comments](shipped-text-comments.md)
holds the design of #shipped-text-line-comments, #shipped-text-entry-references-only and
#delivery-substitutions.

## Mapping tables

In the slices' specs.

## Losing alternatives

In the slices' specs.

## Readings

None. The work reads no external specification the project implements. Slice 2 relies on
CommonMark's fenced blocks to define "outside a fence", as the checker's own Markdown reader does.

## Premortem

Assume the work shipped and failed. Each cause, the thread it stresses, and its verdict. The owner
ruled in R7: "Tables approved, keep all tripwires".

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| P1 | Agents classify open problems as bounded to skip the discussion, and a decision that passes an entry test lands in a commit only | #bounded-path-in-design | tripwire `a-shortcut-decision-earns-a-head`, on the owner's word: fires when a decision-record review finds a commit made through the bounded path whose decision passes an entry test, at 2 instances; response: reopen the bounded path's three conditions; re-entry: the decision-record review before every merge |
| P2 | Whether a rival is plausible stays subjective, and heads keep accumulating or keep being refused case by case | #entry-test-locality | tripwire `a-head-verdict-is-overruled`, on the owner's word: fires when the owner overrules a review's verdict on whether a head is owed, 2 times; response: reopen the head that records the entry tests; re-entry: the decision-record review |
| P3 | Comments become untouchable: an agent escalates every refactor that meets a "why" comment, and the owner rules each time that the comment was no decision | #local-intent-binds | tripwire `a-comment-escalation-is-overruled`, on the owner's word: fires when the owner rules twice that such an escalation was unneeded; response: reopen the rung of comments in the order; re-entry: the retrospective, since this is behaviour in sessions, with the limit `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` names |
| P4 | A `%%` line ships: it is indented, or inside a fence | #shipped-text-line-comments | survives into an acceptance criterion of slice 2, since code can check it |
| P5 | A `path@*@<path>` reference in the shipped text is wrong in a conforming project that uses the directory shape | #shipped-text-entry-references-only | does not arise: the checker accepts `path@*@<path>` for every shape of a required document, per `design@core@reserved-anchors`, verified after R7 and ruled in R8 (see #shipped-text-entry-references-only) |

## Acceptance criteria

In the slices' specs: each criterion is judged by one slice.

## Implementation sequence

1. Workflow text, landed: the entry tests and their head, the local intent, the bounded
   path, the decision-record reviewer's predicate, the cleanup issue. Text and records only.
2. [Shipped text comments](shipped-text-comments.md): `%%` comments stripped at build, content/
   back in the walk with its 36 repairs, the delivery substitutions, the narrowed head of the
   shipped text.

Between them, the snippet branch merges, on the owner's word in R7: "slice 1 first, merge it, then
merge the snippet branch, then slice 2."

## Order rationale

Slice 1 comes first because its entry tests decide which records slice 2 writes, among them that
#delivery-substitutions earns no head; and slice 2 comes after the snippet branch because it
adds its comment strip and its substitutions beside that branch's placeholder mechanism in
`path@agent-skills@build.rs`.

## Defaults awaiting the owner

None. The three defaults this section held were ruled in R8, and the state of #record-volume's
condition in R9, and each is written into the thread
it bears on: #shipped-text-entry-references-only, #bounded-path-in-design, #t1-consumed-interface,
#record-volume.

## Harvest

This document's own row: it is deleted in the commit that completes slice 2's harvest. That
commit removes its citation from the list under `design@knowledge-architect@stays-at-zero-x`,
since the work it stood for has landed, and from every other text that cites it. Each slice's row
is in its spec.

## Later consequences

- **The cleanup issue** removes or splits the heads no entry test admits. Its agent-skills part
  moves what is worth keeping into `%%` lines beside the instructions, which slice 2 makes
  possible.
- **`issue@agent-skills@shipped-text-is-reference-free-mechanically`**, as slice 2 rewrites it,
  applies a future reference-clean declaration to the installed copies under .claude.
