# Structured plan documents: the plans anchor, plan items, a roadmap, and built intent

## Status and audience

**This is a milestone document**, under the installed `knowledge-architect-planning`. It carries
the design converged in one discussion between the owner and an agent, under
`knowledge-architect-design`, and the three steps that build it. It is written for a session that
did not witness that discussion.

- **It leaves when its work lands.** Each step's spec leaves in the commit that completes that
  step's harvest; this document leaves with the last step.
- **Where this document and a design home disagree, the design home wins**, and this document has
  the defect.
- **Every name it uses is defined in it, or exists in the code.** The names section expands the
  shorthands.
- **Where the owner's word is needed and the owner is absent, the work does not proceed on that
  point.** The defaults awaiting the owner are listed in their own section.
- **Identifiers.** Every thread, argument, criterion, step and acceptance criterion carries an
  identifier, written plain with a hash sign, as in #plans-location. Step 2 turns them into
  definitions; until then nothing checks them.
- **The record was assembled from the transcript**, as #ledger-from-transcript decides: a subagent
  extracted the owner's messages, the delta tables, the rulings and the arguments from the
  session log
  `~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/5519903e-6ef5-48cc-8d25-8de28599edfd.jsonl`,
  records 8 to 224, counted from 0. A `/clear` precedes record 8 and is excluded. Round numbers
  below are the
  rounds of that discussion: one owner message and the reply to it.

The step specs, in implementation order:

- [Step 1, #plans-structure](plans-structure.md)
- [Step 2, #plan-items](plan-items.md)
- [Step 3, #workflow-text](workflow-text.md)

## How a step is worked

This restates §7 of `knowledge-architect-planning`, which is its home. Where the two disagree, the
skill wins.

**Defaults.** Each step's spec names the defaults it depends on. The audit may run while one is
unruled; the implementation of a point whose default is unruled does not start, per §8 of the skill
("The owner rules on each at the first audit, or at once if present").

**Branches and history** (D14 as ruled, revised by default D18):

- This document lands alone, on the branch `structured-plans`, in a pull request of its own merged
  under the current checker before step 1 starts.
- Each step has its own branch, cut from main after the previous step merged.
- Every commit of a branch must pass the check under the branch tip's checker, per the root
  `CLAUDE.md`. A step whose checker refuses its earlier trees (step 1 refuses every tree without the
  plans homes) squashes those commits into one before review; the squashed message keeps the
  audit's subject line, so `git log --grep='design audit'` still finds it.
- A step's harvest commits are made on its branch after the review's repairs and before the merge,
  and the merge waits for the gates on the harvested tree. So a decision this work reverses is
  contradicted by the code only on the step's branch, never on main, with one exception named under
  "What is already decided".

1. **Ground**: the Component's `CLAUDE.md`, its design home, its rejected alternatives, its open
   issues, its tripwires; then this document entire, then the step's spec.
2. **The design audit.** Read the step's entry and every decided shape it depends on against the
   code as it stands and against the design homes. List every gap: a shape the code refutes, a
   passage of a specification the entry did not read, a name the entry uses that the code does not
   have, a consequence the entry did not see. Sort each gap:
   - **Applied in place.** The gap has one answer this document's decisions already imply, or is a
     choice among shapes this document rules out all but one of. Write the answer into the step's
     spec under a heading saying the audit's findings are applied as the step's binding shape, each
     finding stating the gap, the answer and the decision it follows from. Commit the amended spec
     alone, with a subject of the shape `The <step> design audit, applied in place: <n> gaps, none
     reopening a discussion`. Earlier audits are found with `git log --grep='design audit'`.
   - **Load-bearing.** The gap is material, or is a choice between two shapes neither of which this
     document rules out, or needs a ruling marked here as the owner's. Record it in the step's
     spec as open at the audit, with the discriminating fact, stop the step, and open a design
     session with the owner under `knowledge-architect-design`. Its converged design goes into the
     step's spec, in the sections of §4 of the skill, and owes the reviews of its §8. The step
     resumes from it.
3. **Claims, tests, implementation, gates, commit**, per `klarch-development`. The commit names how
   each claim's test was shown to fail against a wrong implementation, and says of any claim whose
   test cannot yet do so why not.
4. **Review before the merge**, per `knowledge-architect-review`. A repair is a further commit. A
   finding not repaired becomes an issue entry.
5. **The report**: the landing commit reports on each acceptance criterion judged at this step.
6. **The harvest**, per the step's rows in the harvest section: the decisions and the losing
   alternatives under `knowledge-architect-decision-recording`, then the tripwires and the issues
   under `knowledge-architect-issue-tracking`. A tripwire names the head that harvested its
   decision, so the head is written first. Where a design home is a directory, a new subdocument is
   linked from its README.
7. **The step's spec leaves** in the commit that completes its harvest, as in §9 of the skill. What
   crosses steps stays in this document, amended in place where the landing changed it.

## Names

| name | what it names |
| --- | --- |
| the core | crates/core, package `knowledge-architect`: the checker |
| the installed text | crates/agent-skills/content/: the skills, agents and primer the checker installs |
| the plans directory | `path@knowledge-architect@docs/plans/` of the root Component |
| the plans anchor, `plans` | the anchor this work adds at the plans directory (#plans-location) |
| plan document | a spec or a milestone document |
| plan anchor | the anchor of one plan document: a spec file under docs/plans/specs/, or a milestone directory under docs/plans/milestones/ (#plan-anchor) |
| item | a thread, an argument, a criterion or an acceptance criterion defined inside a plan document (#plan-items-by-section) |
| the roadmap | docs/roadmap.md of the root Component (#roadmap-home) |
| the transcript | the harness's session log of a discussion; in Claude Code, `~/.claude/projects/<project>/<session>.jsonl` |
| assembly | the planning skill writing a plan document from the transcript (#ledger-from-transcript) |
| built intent | what a design home holds after this work: the design as built and its reasons (#design-home-is-built-intent) |
| `Shape` | the enum in `path@core@src/manifest.rs` with the variants `Heading` and `File` |
| `Anchor`, `Anchors` | the types in `path@core@src/entity.rs` that hold the Components and locations |
| `resolve_anchors`, `collides` | the functions in `path@core@src/manifest.rs` that refuse a misnamed or misplaced anchor |
| P1 to P11 | the causes of the premortem, in its section below |
| T1 to T5 | the tripwires the premortem proposed; T2, T4 and T5 are recorded, in the premortem section below |
| D1 to D18 | the defaults, in "Defaults awaiting the owner" below: all ruled, D5 decided by the owner earlier |
| the presumed rows | the six threads closed by the checkpoint batch confirmation of round 7: #plans-dir-declared, #plans-per-anchor, #planned-name, #roadmap-register, #no-roadmap, #design-home-is-intent |
| `Shape::Directory` | the third variant of `Shape` that step 1 adds for the milestone register's entries (D10) |
| shape (a) of #plan-item-scope | an item cited with the root anchor and a compound id, `thread@knowledge-architect@<plan>/<id>`; it lost |
| shape (b) of #plan-item-scope | an item cited with no anchor, `thread@<id>`, resolved against the enclosing plan; it lost |

## What the work is

**Today**, measured at main's commit 90a4b56:

- **The plans directory is a row of the knowledge table**, not something the checker knows. The
  root `CLAUDE.md` names it, per `design@agent-skills@plans-directory-declared`. The checker reads
  plan documents as it reads any document outside a register home: their references, their slugs
  (each one a misplaced definition), their path-shaped spans and, in a README, their links are
  checked. Nothing reads their structure.
- **No code mentions plans, specs or milestones** in crates/core/src, except fixture paths
  docs/plans in unit tests of `path@core@src/manifest.rs`, `path@core@src/walk.rs` and
  `path@core@src/check/registers.rs`.
- **An anchor is a Component or a location** (`Anchor` in `path@core@src/entity.rs`), and a register
  has one of two shapes (`Shape`). The reserved anchors under the `path` kind are `*` and
  `elsewhere`, constants in `path@core@src/entity.rs`, matched by hand in `path()`,
  `ignore_queries` and `table()` of `path@core@src/check/references.rs`, in `resolve_anchors` of
  `path@core@src/manifest.rs`, and in `Anchors::is_anchor_word`.
- **The installed planning skill fixes the layout and the section titles** of a plan document and
  writes thread identifiers plain, so that a later structure could read them, per
  `design@agent-skills@structure-ready`.
- **There is no roadmap**, per `design@agent-skills@planned-work-is-an-issue`.
- **The record contradicts itself on what a design home is.** The installed primer says *"A design
  home is intent. Check the code against it, never the other way."*
  `design@agent-skills@harvest-after-implementation` says *"A design head is a claim about the code
  as it stands"*.
- **This repository's plans directory** holds only its README.

**The work**: the checker learns plan documents as a structure of their own, with citable items;
a roadmap orders the work; the skills write plan documents as the full record of a discussion,
assembled from the transcript; and the design homes are stated to hold built intent.

**Outside the work:**

- **thaum's migration.** thaum pins the checker exactly, per `design@agent-skills@exact-pin`, so
  nothing changes for it until it moves its pin. Its plan documents and its roadmap exception are
  rewritten then, in thaum. The owner ruled this in round 6.
- **A roadmap for this repository.** The roadmap is optional; writing one here is default D3.
- **Several plans directories.** Ruled out by #plans-at-root; tripwire T5 watches it.

## What is already decided

The design rests on these and does not argue them again:

- `design@core@generated-files-are-pure`: a generated index is a function of the walked tree.
- `design@core@an-extension-plugs-in-through-phased-hooks`: an extension runs after the entity table
  is built, so items are core work (a83).
- `design@agent-skills@spec-leaves-at-landing`, `design@agent-skills@standing-argument-in-head`:
  what a plan document is for and when it leaves.
- `design@agent-skills@exact-pin`: why thaum is outside the work.

These are rewritten or reversed by this work, each at the harvest named in the harvest section:

- step 1, core: `design@core@anchors-are-components-and-locations`,
  `design@core@reserved-anchors`, `design@core@every-path-names-its-anchor`,
  `design@core@components-carry-the-same-documents`, `design@core@registers-are-declared` (it
  states that four registers are compiled in, which D1 makes six),
  `design@core@a-file-register-is-a-directory-of-entries` and
  `design@core@a-file-register-index-is-rows` (the directory entry of D10, and the owning-anchor
  filter of the index);
- step 2, core: `design@core@an-entry-is-a-heading-at-the-register-level`,
  `design@core@a-slug-belongs-to-a-component`;
- step 3, agent-skills: `design@agent-skills@plans-directory-declared`,
  `design@agent-skills@planned-work-is-an-issue`, `design@agent-skills@structure-ready`,
  `design@agent-skills@thread-slug-is-entry-id`,
  `design@agent-skills@harvest-after-implementation`, `design@agent-skills@primer-content`,
  `design@agent-skills@design-hands-off-to-planning`, `design@agent-skills@document-vocabulary`,
  `design@agent-skills@milestone-is-a-directory`.

While a step's branch is open, its code diverges from the heads it rewrites; that is the change,
not a defect, and the step's harvest on the same branch ends it. The exception is
`design@agent-skills@plans-directory-declared`: step 1 falsifies its reason ("A manifest key would
be a checker change that nothing reads yet"), while the skill text it governs changes in step 3. So
step 1's harvest rewrites that reason, and step 3's reverses the decision.

**A tripwire guards a head this work rewrites**, and its harvest judges it:
`tripwire@agent-skills@ledger-lost-before-hand-off` guards
`design@agent-skills@design-hands-off-to-planning`. It fires when a discussion's ledger is lost to
compaction before the plan document is written, and its response reopens with the alternative
"Writing the discussion's ledger to a file during the discussion". #ledger-from-transcript changes
its premise: the ledger is assembled from the transcript on disk. It has not fired: this session's
transcript holds no compaction marker. The discussion did not see it; the review before the merge
found it. Step 3's harvest rewrites it to watch assembly from the transcript, absorbs T2 into it if
both guard the same head, or deletes it, under `knowledge-architect-issue-tracking`.

What else references a decision this work rewrites, as `cargo klarch show` lists it, and the
harvest that judges each:

| entry | references | judged at |
| --- | --- | --- |
| `tripwire@core@reserved-anchors-generic-rule` | `design@core@reserved-anchors` | step 1 |
| `tripwire@core@issue-kind-list-grows` | `design@core@a-file-register-is-a-directory-of-entries` | step 1 |
| `issue@core@a-planned-path-can-be-named` | `design@core@reserved-anchors`, `design@core@every-path-names-its-anchor` | step 1 |
| `issue@core@cross-project-references` | `design@core@a-slug-belongs-to-a-component` | step 2 |
| `tripwire@agent-skills@ledger-lost-before-hand-off` | `design@agent-skills@design-hands-off-to-planning` | step 3, as above |

## Criteria

| criterion | kind | source | satisfaction |
| --- | --- | --- | --- |
| #one-place-for-open | binding | `goal@knowledge-architect@structure-and-workflow-work-together` | met: #roadmap-orders-issues, #roadmap-home |
| #citations-checked | binding | `goal@knowledge-architect@documentation-stays-consistent` | met: #plan-register, #plan-items-by-section, #plan-item-scope, #plans-location |
| #installed-text-anywhere | binding | `goal@agent-skills@installed-text-works-anywhere` | met: #plans-dir-fixed is the tool's convention; #ledger-from-transcript falls back where no transcript exists |
| #deliberation-reachable | binding | `goal@knowledge-architect@design-is-recorded-with-its-arguments` | met: #spec-records-the-exchange, and history after the plan leaves |
| #owner-rulings-recorded | binding | `goal@knowledge-architect@the-owner-decides` | met: #spec-records-the-exchange, #ledger-from-transcript |
| #plan-leaves-at-landing | binding, as a presumption | `design@agent-skills@spec-leaves-at-landing` | met: unchanged; #retiring-plan-opens-issue adds the procedure for citations of a leaving plan |
| #discussion-survives-compaction | weighed, the agent's (round 1) | the design skill's step 8: the ledger lives in the conversation | met: #ledger-from-transcript, where a transcript exists |
| #writing-cost | weighed | the length of a plan document | unmet-and-accepted: the owner named it in round 7, "The presumed rows, writing-cost and the five clauses are all approved." |

A ninth criterion, existing-plans-readable (binding, from `design@agent-skills@structure-ready`),
left with that decision: the owner approved its reversal in round 6, "structure-ready reversal
approved."

## Threads

29 threads. Each row gives the final state, the resolution, the arguments that moved it (listed in
full under "Arguments" below), the section carrying its shape, and the durable home expected to
harvest it. A thread whose decision earns an entry keeps its slug, per
`design@agent-skills@thread-slug-is-entry-id`; whether it earns one is decided at its harvest, under
`knowledge-architect-decision-recording`.

### Approved (20)

| thread | resolution | arguments | shape | harvest home |
| --- | --- | --- | --- | --- |
| #plans-dir-fixed | the plans directory is `path@knowledge-architect@docs/plans/`, fixed by the tool | a1, a9, a11, a12 | decided design | core design home (step 1); reverses `design@agent-skills@plans-directory-declared` (step 3) |
| #plans-at-root | one plans directory, in the root Component | a2, a13, a15, a39, a40 | decided design | core design home (step 1) |
| #plans-name-kept | the name stays `plans` | a16, a17, a18, a19 | decided design | agent-skills `design@agent-skills@document-vocabulary`, rewritten (step 3) |
| #roadmap-orders-issues | the roadmap holds order only; every row is a checked reference | a5, a20, a21, a22, a41, a98 | decided design | agent-skills design home, reversing `design@agent-skills@planned-work-is-an-issue` (step 3) |
| #roadmap-home | docs/roadmap.md at the root; fixed name, optional; no checker code | a1, a90, a91 | decided design | agent-skills design home (step 3) |
| #layout-kept | a spec is one file; a milestone is a directory of a README and one spec per step | none beyond the owner's word | decided design | `design@agent-skills@milestone-is-a-directory`, rewritten (step 3) |
| #plan-register | plan documents are a structure the checker reads | a25 | decided design | core design home (steps 1 and 2) |
| #plan-document-kinds | kinds `spec` and `milestone`; a `path` citation of a plan document is refused; a whole plan document may be cited from anywhere | a42, a51, a52, a53, a54, a66, a68, a81 | decided design | core design home (step 1) |
| #plans-split-dirs | docs/plans/specs/ and docs/plans/milestones/ | a66, a78, a79 | decided design | core design home (step 1) |
| #plans-location | the plans directory is an anchor `plans`, fixed by the tool and reserved | a68, a80 | decided design | core `design@core@reserved-anchors`, rewritten (step 1) |
| #plan-anchor | each plan document is an anchor: a spec file or a milestone directory | a57, a79 | decided design | core `design@core@anchors-are-components-and-locations`, rewritten (steps 1 and 2) |
| #plan-item-scope | shape (a'), `<kind>@<plan>@<id>`; an item is cited only from inside its own spec file or milestone directory | a6, a55, a56, a57, a58 | decided design | core design home (step 2) |
| #plan-items-by-section | an item is `### <statement> ##<id>` under a fixed level-two section, which gives its kind | a67, a82, a83 | decided design | core design home (step 2) |
| #spec-records-the-exchange | the plan document records the whole discussion: proposer, final state, arguments, rulings verbatim with their round, relations | a6, a26, a27, a43, a50 | decided design | agent-skills design home (step 3) |
| #arguments-as-items | arguments are citable items without state | a6, a28, a29, a30 | decided design | agent-skills design home (step 3) |
| #argument-ids | `a1`, `a2`, …, never reused; one sequence per milestone | a61, a62, a69, a74 | decided design | agent-skills design home (step 3) |
| #argument-segmentation | argument boundaries are decided at assembly, checked by the transcript reviewer | a92, a93 | decided design | agent-skills design home (step 3) |
| #ledger-from-transcript | the planning skill assembles from the transcript through a subagent; without a transcript, from the conversation | a71, a72, a73, a74, a75, a76, a77 | decided design | agent-skills `design@agent-skills@design-hands-off-to-planning`, rewritten (step 3) |
| #design-home-is-built-intent | design homes hold built intent; plans hold unbuilt intent; harvest stays at landing | a7, a8, a33, a34, a45 | decided design | agent-skills `design@agent-skills@harvest-after-implementation` and `design@agent-skills@primer-content`, rewritten (step 3) |
| #retiring-plan-opens-issue | a leaving plan's session removes whole-document citations of it from other plans and opens an issue on each citing plan | a85, a86, a87, a88, a89 | decided design | agent-skills design home (step 3) |

### Withdrawn (4)

| thread | defeating reason | arguments | harvest home |
| --- | --- | --- | --- |
| #spec-written-during-discussion | no spec can be assembled while its threads are open (the owner's a64); it was also the recorded lost alternative "Writing the discussion's ledger to a file during the discussion" | a31, a32, a44, a46, a47, a48, a49, a64 | judged at step 3 by `design@agent-skills@losing-shape-test`; the recorded alternative stays as it is |
| #draft-ledger | two of the recorded alternative's three reasons still apply; #ledger-from-transcript answers all three | a65, a70 | judged at step 3 |
| #planned-name | the owner's own proposal; the agent read round 2's "Agreed on plans-name-kept." as its withdrawal, and the owner confirmed in round 7. "planned" names scheduled work, the roadmap's content (a16) | a4, a16 | judged at step 3 |
| #cross-plan-references | few plans are open at once and a whole-document citation carries the dependency (a84); item citations would force design work at the moment of retiring a plan (a85) | a59, a60, a84, a85 | judged at step 2; tripwire T4 |

### Ruled out (5)

| thread | reason | arguments | harvest home |
| --- | --- | --- | --- |
| #plans-dir-declared | the storage of a built-in register is fixed by the tool | a9, a11 | the reversal of `design@agent-skills@plans-directory-declared` (step 3) |
| #plans-per-anchor | most work spans Components; the reference grammar makes a later split free | a3, a13, a14, a15, a39 | judged at step 1; tripwire T5 |
| #roadmap-register | two registers of undesigned work fail #one-place-for-open | a23 | judged at step 3 |
| #no-roadmap | thaum's exception showed the need | a20, a24 | the reversal of `design@agent-skills@planned-work-is-an-issue` (step 3) |
| #design-home-is-intent | needs a per-entry built or unbuilt marker, and reports false defects on unbuilt entries | a35, a36, a37, a38 | judged at step 3 |

### Rulings

The owner's words that closed each thread, verbatim, from the transcript. "R" is the round.

| # | slug | first appears | final state | the owner's closing words (verbatim), or how it closed |
| --- | --- | --- | --- | --- |
| 1 | #plans-dir-fixed | R1 (agent names the owner's proposal) | approved | R2: "I agree with plans-dir-fixed." |
| 2 | #plans-dir-declared | R1 | ruled-out | no explicit word; closed by the checkpoint batch confirmation, R7: "The presumed rows, writing-cost and the five clauses are all approved." |
| 3 | #plans-at-root | R1 | approved | R2: "I agree with plans-at-root. It can always be reopened later." |
| 4 | #plans-per-anchor | R1 | ruled-out | no explicit word; closed by the checkpoint batch confirmation, R7 (same sentence as row 2) |
| 5 | #plans-name-kept | R1 | approved | R2: "Agreed on plans-name-kept." |
| 6 | #planned-name | R1 (the owner's proposal) | withdrawn (by the owner, as presumed by the agent) | no explicit word naming it; the agent read R2 "Agreed on plans-name-kept." as withdrawing it; confirmed by the checkpoint batch, R7 |
| 7 | #roadmap-orders-issues | R1 | approved | R2: "roadmap-orders-issues: approved. I like that shape. It brings together multiple register shapes to form the roadmap using only references, this is the right solution." |
| 8 | #roadmap-register | R1 | ruled-out | no explicit word; closed by the checkpoint batch confirmation, R7 |
| 9 | #no-roadmap | R1 | ruled-out | no explicit word; closed by the checkpoint batch confirmation, R7 |
| 10 | #layout-kept | R1 | approved | R2: "layout-kept: approved." |
| 11 | #plan-register | R1 | approved (shape) | R2: "plan-register: Agreed with the shape." |
| 12 | #plan-item-scope | R1 | approved as (a') | R2: "plan-item-scope: still unsure between options a and b." then R3: "plan-item-scope: a' looks fine to me. See my proposal above too, it aligns with that." The bound came from R3: "Mine is still the same for now: bound to the individual spec or milestone." |
| 13 | #spec-records-the-exchange | R1 (the owner's proposal) | approved | R2: "spec-records-the-exchange: approved. I think this should also apply to milestones. Milestones are created through the design skill too after all. It contains a README, and an individual spec for each step in the milestone. I think the initial design discussion of the milestone writes its README, right ? Then the README is the \"milestone spec\"." |
| 14 | #arguments-as-items | R1 | approved | R2: "arguments-as-items: approved. Being able to cross reference arguments is certainly much better." |
| 15 | #spec-written-during-discussion | R1 (agent) | withdrawn (by the agent, R3) | R3: "spec-written-during-discussion: on second thought, I'm not too keen on reopening this rejected alternative." The agent withdrew it in its R3 reply, with the owner's argument as the defeating reason. |
| 16 | #design-home-is-built-intent | R1 (agent) | approved | R2: "design-home-is-built-intent: I agree. Once plans become structured registers, the distinction is clear, and design docs can carry the meaning of \"built intent\", while plans carry \"unbuilt intent\"." Its four consequences, R3: "Approved on the consequences of design-home-is-built-intent" |
| 17 | #design-home-is-intent | R1 | ruled-out | no explicit word; closed by the checkpoint batch confirmation, R7 |
| 18 | #plan-document-kinds | R2 (agent names the owner's R2 side note) | approved | R4: "plan-document-kinds: shape approved, including plans-location and plan-items-by-section." Points 2-4, R3: "Agreed on points 2,3 and 4 though." |
| 19 | #plan-anchor | R2 (delta only) | approved | no explicit word naming it; the agent closed it in R3 on the owner's R3 "a' looks fine to me", and listed it as absorbed into #plan-item-scope |
| 20 | #cross-plan-references | R2 | withdrawn (by the agent, R3) | no explicit word closing it; closed by the agent's withdrawal in R3 after the owner's R3: "cross-plan-references: what do you mean with wider bound ? I do not understand your position here. Mine is still the same for now: bound to the individual spec or milestone." The owner added a second argument in R4 (see a85); the state stayed withdrawn. |
| 21 | #argument-ids | R2 | approved | R3: "argument-ids: agreed. This avoids having to name each argument, which could be quite tedious, and not very useful since they do not get harvested in registers." |
| 22 | #draft-ledger | R3 (agent names the owner's R3 idea) | withdrawn (by the owner) | R4: "ledger-from-transcript approved, it's better than my draft ledger." (closes it by implication) |
| 23 | #ledger-from-transcript | R3 (agent) | approved | R4: "ledger-from-transcript approved, it's better than my draft ledger." |
| 24 | #plans-split-dirs | R3 (delta only; the owner's R3 proposal) | approved | no explicit word naming it; the agent recorded it approved in R4 on the owner's R4 "plan-document-kinds: shape approved", which absorbs it |
| 25 | #plans-location | R3 (agent) | approved | R4: "plan-document-kinds: shape approved, including plans-location and plan-items-by-section." |
| 26 | #plan-items-by-section | R3 (agent) | approved | R4: same sentence as row 25 |
| 27 | #retiring-plan-opens-issue | R4 (agent names the owner's R4 procedure) | approved | R5: "retiring-plan-opens-issue and roadmap-home approved, argument-segmentation at assembly" |
| 28 | #roadmap-home | R4 (agent) | approved | R5: same sentence as row 27 |
| 29 | #argument-segmentation | R4 (agent) | approved, shape "at assembly" | R5: same sentence as row 27 |

Rulings on items that are not threads:

| item | round | the owner's words (verbatim) |
| --- | --- | --- |
| reversal of `design@agent-skills@structure-ready` (criterion existing-plans-readable leaves with it) | R6 | "structure-ready reversal approved." |
| thaum migration out of scope | R6 | "thaum's plan documents won't need a rewrite today. This will only happen when I make a new release of knowledge-architect, then bump the version on thaum's side. thaum migration is not in scope of this work, it no longer needs now that v0.1 is released and thaum has already migrated to it." |
| tripwires T1-T5 | R6 | "Keep tripwires T2, T4, T5. Not the others." |
| criterion writing-cost (unmet-and-accepted), the 6 presumed rows, clauses P1, P2, P3, P8, P9 | R7 | "The presumed rows, writing-cost and the five clauses are all approved." |
| the thaum evidence | R7 | "You were right to go looking for thaum, and the evidence you found there was indeed useful." |
| permission to proceed to planning | R7 | "You can proceed." |

### Arguments

Every argument of the discussion, numbered in order of appearance, per #argument-ids and
#argument-segmentation. The boundaries were decided at assembly. Row a85 is the owner's second
argument on #cross-plan-references, given in round 4.

Notes on the table, from the review of this document:

- **a25 was replaced by a82** in round 3: items need a model of their own, and the existing
  heading-entry code does not read them as it stands.
- **a72 does not reproduce as stated**, and the correction is the author's. The round-3 count
  matched the compaction markers as a substring of each record, so it also counted records whose
  text only mentioned a marker. Re-taken by record fields (`type` equal to `system` with `subtype`
  equal to `compact_boundary`, and `isCompactSummary`), over
  `~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/46dca865-fb8e-4006-be09-548d0d1ca801.jsonl`:
  7177 records, one compaction, its boundary at record 3738 counted from 0, and 512 user and 932
  assistant records before it, all present. The premise a72 serves, that the transcript keeps the
  records from before a compaction, holds on one observed compaction.
- **Instruments of the other figures**: the reference lengths of a55 are character counts of the
  three example spans; "about 727 KB" (a76) was `ls -la` of this session's transcript in round 3;
  "thaum's 3 plan documents" (a27) and "about 5 entries" (a22) were `ls` of thaum's plans directory
  and a reading of its `next-milestones.md`.
- **Spans normalised**: where a quotation held a path in backticks, the backticks were removed or
  the path anchored so that the checker reads it, in a10 (backticks removed), a20, a46 and a98.

| id | round | who | threads | paraphrase | key verbatim |
| --- | --- | --- | --- | --- | --- |
| a1 | 1 | owner | #plans-dir-fixed, #plans-dir-declared | Fixing the plans directory makes skills easier to write, and much is already fixed by the tool. | "I'd recommand set in stone to facilitate writing skills and such, after all a lot is already set in stone by the project" |
| a2 | 1 | owner | #plans-at-root | In both projects that used the workflow, one plans directory was enough. | "I've only ever needed it to be unique in the projects I made with this workflow" |
| a3 | 1 | owner | #plans-per-anchor | A much larger project with big Components, which can have sub-Components, might want to split the plans directory. | "a much larger project with big components (components can have subcomponents btw) might want to split the plan directory" |
| a4 | 1 | owner | #planned-name | "planned" is more correct because the directory holds already designed work with decided threads. | "\"planned\" is actually a bit more correct, because it holds already designed work, with decided threads" |
| a5 | 1 | owner | #roadmap-orders-issues, #roadmap-register, #no-roadmap | The owner asks for a way to declare an optionally ordered list of wanted work that is not yet designed. | "a \"roadmap\" feature, which would allow declaring an (optionally ordered) list of work that we want to do, but is not necessarily already designed" |
| a6 | 1 | owner | #spec-records-the-exchange, #arguments-as-items, #plan-item-scope, #cross-plan-references | The plan structure should integrate with the design skill's threads; the initial spec should be a precise record of the discussion, with cross references between its items bounded to the spec or milestone directory. | "with the possibility of cross referencing all these items, bounded to the spec document or the milestone directory" |
| a7 | 1 | owner | #design-home-is-built-intent, #design-home-is-intent | Design registers could reflect intent, yet the standing directive keeps them current against the code; this makes the moment of harvest unclear. | "There is kind of a contradiction between expected usage and the workflow of design registers." |
| a8 | 1 | agent | #design-home-is-built-intent, #design-home-is-intent | The record contradicts itself: the primer calls a design home intent, while `harvest-after-implementation` calls a design head a claim about the code as it stands. | "One says a design home is intent, the other that it is a claim about the code." |
| a9 | 1 | agent | #plans-dir-fixed, #plans-dir-declared | A Component already gets its storage at fixed paths from the tool, and a built-in register refuses a `dir` key; a declared plans directory would be the only declared path in that family. | "A declared directory would be the only declared path in that family." |
| a10 | 1 | agent | #plans-dir-fixed | Cost: thaum already uses docs/plans/, so it pays nothing. | "thaum already uses docs/plans/, so it pays nothing" |
| a11 | 1 | agent | #plans-dir-fixed, #plans-dir-declared | The reason recorded for the declared directory (a manifest key nothing reads) is gone once the checker reads plans. | "Once a checker reads it, that reason is gone." |
| a12 | 1 | agent | #plans-dir-fixed | A path the tool fixes is the tool's own convention, so the goal that nothing of a project is compiled in still holds. | "A path the tool fixes is not a project compiled in." |
| a13 | 1 | agent | #plans-at-root, #plans-per-anchor | The reference grammar already names the anchor, so a later split adds `plan@core@<id>` and rewrites no existing citation. | "the reference grammar already names the anchor" |
| a14 | 1 | agent | #plans-per-anchor | Because every Component carries every built-in home, per-anchor plans mean 4 more empty plans directories, or a new class of optional built-in register. | "4 more plans directories with READMEs, holding nothing" |
| a15 | 1 | agent | #plans-at-root, #plans-per-anchor | Most work crosses Components (this issue touches core and agent-skills; the v0.1 milestone touched every Component), so such plans sit at the root anyway. | "Most work crosses Components." |
| a16 | 1 | agent | #planned-name, #plans-name-kept | In common English "planned work" means intended or scheduled work, designed or not, which is the roadmap's content. | "If a roadmap thread is approved, \"planned\" names the wrong one of the two." |
| a17 | 1 | agent | #plans-name-kept | `document-vocabulary` makes common developer usage binding, and its terms are "plan document" and "plans directory". | "\"Plan document\" and \"plans directory\" are its terms." |
| a18 | 1 | agent | #plans-name-kept | Every directory under `docs/` is named by a noun. | "Every directory under `docs/` is named by a noun" |
| a19 | 1 | agent | #plans-name-kept, #planned-name | No common single word says "designed", so the README and skills must carry that meaning. | "\"plans\" does not say it either. No common single word I know does." |
| a20 | 1 | agent | #no-roadmap, #roadmap-orders-issues, #roadmap-register | thaum's `path@elsewhere@docs/next-milestones.md` exception is real use in the sense of `additions-need-real-use`, which is new evidence for reopening `planned-work-is-an-issue`. | "The new evidence is thaum's exception, which is real use" |
| a21 | 1 | agent | #roadmap-orders-issues | Drift is bounded mechanically: a closed issue's row dangles and fails the check; writing a plan closes its issue; `show` lists the roadmap as a dependent. | "Drift is bounded mechanically" |
| a22 | 1 | agent | #roadmap-orders-issues | It meets one-place-for-open, since the work stays in the issue register and the roadmap holds only order; cost for thaum about 5 entries become `todo` issues. | "the roadmap holds only the order" |
| a23 | 1 | agent | #roadmap-register | Two registers would each hold known undesigned work, which needs a routing rule, hides roadmap entries from `cargo klarch issues`, and fails one-place-for-open. | "two registers can each hold \"known, undesigned work\", which needs a routing rule" |
| a24 | 1 | agent | #no-roadmap | Keeping no roadmap leaves thaum's exception as a permanent divergence from the installed skill. | "a permanent divergence from the installed skill" |
| a25 | 1 | agent | #plan-register | Items defined like design entries, as headings ending with a slug, are read by the existing heading-entry code. | "So the existing heading-entry code reads it." |
| a26 | 1 | agent | #spec-records-the-exchange | It fits `standing-argument-in-head`, which already names the spec as the home of the deliberation, and moves owner-rulings-recorded to met. | "already names the spec as the home of the deliberation" |
| a27 | 1 | agent | #spec-records-the-exchange | It strains existing-plans-readable, but only thaum's 3 plan documents exist, and `structure-ready` names this as a reason to reopen it. | "the only plan documents that exist are thaum's 3" |
| a28 | 1 | agent | #arguments-as-items | The design skill refuses to track arguments because state needs a many-to-many relation; an identifier without state does not hit that reason. | "An identifier without state does not hit that reason." |
| a29 | 1 | agent | #arguments-as-items | Named arguments let rulings, premortem causes and closures cite what decided them, and harvest becomes a selection rather than a rewrite. | "the head's standing argument becomes a selection of named arguments rather than a rewrite" |
| a30 | 1 | agent | #arguments-as-items | Cost: the agent mints argument IDs every round and the spec gets longer (writing-cost). | "the agent mints argument IDs every round, and the spec gets longer" |
| a31 | 1 | agent | #spec-written-during-discussion | A file written from round 1 meets discussion-survives-compaction; a summary loses the losing arguments and closure conditions. | "the summary loses exactly what step 8 warns about" |
| a32 | 1 | agent | #spec-written-during-discussion | Cost: a write per round, and a change to `design-hands-off-to-planning`. | "a write per round" |
| a33 | 1 | agent | #design-home-is-built-intent | The issue itself states the cause: no declared register holds a decision before it is built; #plan-register closes that gap. | "no declared register can hold a decision before it is built." |
| a34 | 1 | agent | #design-home-is-built-intent | Unbuilt intent at the level of goals already has a home, `an-unmet-goal-is-intent`. | "Unbuilt intent at the level of goals already has a home" |
| a35 | 1 | agent | #design-home-is-intent | Without a per-entry status marker, a reader cannot tell built from unbuilt. | "a reader cannot tell built from unbuilt without a status marker" |
| a36 | 1 | agent | #design-home-is-intent | "Check the code against it" would report false defects on every unbuilt entry. | "reports false defects on every unbuilt entry" |
| a37 | 1 | agent | #design-home-is-intent | A milestone's later steps would sit in the design homes on main for several PRs before any code exists. | "for several PRs before any code exists" |
| a38 | 1 | agent | #design-home-is-intent | A decision reversed during implementation would be edited twice. | "A decision reversed during implementation is edited twice." |
| a39 | 2 | owner | #plans-at-root, #plans-per-anchor | Most work spans several Components, and there should never be so many planned tasks open at once that a split makes sense. | "there should never be that many planned tasks open at once that splitting the directory would make sense" |
| a40 | 2 | owner | #plans-at-root (later reused by the agent for #cross-plan-references) | Plans tend to break each other when one is designed before the other is implemented. | "plans have a tendency to break each other, when they are designed before the other is implemented" |
| a41 | 2 | owner | #roadmap-orders-issues | The shape builds the roadmap from several register shapes using only references. | "It brings together multiple register shapes to form the roadmap using only references" |
| a42 | 2 | owner | #plan-document-kinds | The owner wants plan documents cited by kind: milestone@knowledge-architect@structured-plans, and spec@...@... | "I'd like to be able to reference path@knowledge-architect@docs/plans/structured-plans/ as milestone@knowledge-architect@structured-plans" |
| a43 | 2 | owner | #spec-records-the-exchange | Milestones are also created through the design skill, so the record applies to them; the README is the "milestone spec". | "Milestones are created through the design skill too after all." |
| a44 | 2 | owner | #spec-written-during-discussion | The recorded decision against a written delta (agent overhead hurts the discussion) predates model progress; the owner observed Opus 5.5 carry design discussions interleaved with implementation without significant problem, and confirmed it failed in the Opus 5 time. | "I confirmed this was not working in the Opus 5 time. So the progress is real." |
| a45 | 2 | owner | #design-home-is-built-intent | Once plans are structured registers, design homes carry built intent and plans carry unbuilt intent. | "design docs can carry the meaning of \"built intent\", while plans carry \"unbuilt intent\"" |
| a46 | 2 | agent | #spec-written-during-discussion | `path@agent-skills@docs/rejected-alternatives.md` already records this proposal as lost to `design-hands-off-to-planning`, with three reasons; reopening must answer them and needs the owner's explicit word. | "by the record test, my proposal was not new" |
| a47 | 2 | agent | #spec-written-during-discussion | The owner's capability observation answers the "forgets to update" reason, but it is an assumption about the model, so a tripwire candidate. | "It is also an assumption about the model" |
| a48 | 2 | agent | #spec-written-during-discussion | With #plan-register the checker corroborates the file's shape, which answers the "no corroborant" reason only partly: a wrong but well-formed state passes. | "A state that is wrong but well-formed still passes the checker" |
| a49 | 2 | agent | #spec-written-during-discussion | The file's job has changed: it holds every argument and verbatim ruling, which is what compaction removes first and a correct state ledger does not recover. | "That is the content compaction removes first" |
| a50 | 2 | agent | #spec-records-the-exchange | Planning §4 says the milestone document holds the same sections for the whole milestone, so the README is the milestone's spec. | "The milestone document holds the same sections for the whole milestone" |
| a51 | 2 | agent | #plan-document-kinds | The owner's form needs one register with two kinds, taken from the entry's shape, which is new in the core; two registers sharing one home has no core rule. | "One register, two kinds." |
| a52 | 2 | agent | #plan-document-kinds | A step spec's basename is not unique across the plans directory (two milestones can each have `harvest`). | "two milestones can each have a step called `harvest`" |
| a53 | 2 | agent | #plan-document-kinds | Refusing `path` citations of plan documents gives one name per document; keeping both makes `show spec@...` miss path citations. | "`show spec@…` would then miss the citations written as paths" |
| a54 | 2 | agent | #plan-document-kinds | Citing a whole plan document stays legal everywhere; the roadmap needs it, and its dangling at departure is the wanted revisit. | "When the plan leaves, the citation dangles, which is the revisit we want." |
| a55 | 2 | agent | #plan-item-scope | #arguments-as-items multiplies internal references, so reference length matters: (a) 66 characters, (a') 45, (b) 28. | "So the length of one reference matters." |
| a56 | 2 | agent | #plan-item-scope | In (a) the root anchor carries no information since #plans-at-root; at 100 columns it fits about one reference per line. | "The root anchor in every reference carries no information" |
| a57 | 2 | agent | #plan-item-scope, #plan-anchor | (a') keeps the three-part grammar and mirrors the core's "an anchor carries registers"; costs: a file anchor, a plan named like a Component is a finding, an undeclared anchor. | "It mirrors what the core already says: an anchor carries registers." |
| a58 | 2 | agent | #plan-item-scope | (b) cannot name another plan's item, and contradicts the core rule refusing a reference with no anchor, so needs a scoped exception. | "A reference that names no anchor is refused, including inside the file that defines the id." |
| a59 | 2 | agent | #cross-plan-references | The owner's a40 argues for a plans-directory bound: in the A/B scenario a single-plan bound leaves the dependency in prose, and the check never fires on rebase. | "If the bound is the single plan, B cannot write that citation." |
| a60 | 2 | agent | #cross-plan-references | Cost: one plan's landing breaks another's check, but on its branch at rebase, the session that must re-check anyway. | "which is the session that has to do the re-checking" |
| a61 | 2 | agent | #argument-ids | Arguments never become design entries, so they get numbers rather than content slugs, which would cost a name for dozens of statements. | "they cost a name for each of what will be dozens of statements" |
| a62 | 2 | agent | #argument-ids | One sequence across a milestone's README and step specs avoids collisions inside the milestone anchor; each argument is defined once because it bears on several threads. | "one argument bears on several threads" |
| a63 | 2 | agent | (criterion existing-plans-readable; `structure-ready`) | thaum's 3 plan documents would need rewriting; a cost to rule on, or the criterion drops with `structure-ready`. Retracted in R6 (a96). | "3 plan documents in thaum need rewriting into the new shape" |
| a64 | 3 | owner | #spec-written-during-discussion, #draft-ledger | A proper spec cannot be written during the discussion, since threads are not yet approved and no full design can be assembled. | "It would not be possible to write it like this anyway, because threads are not yet approved, so no full design can be assembled." |
| a65 | 3 | owner | #draft-ledger | Distinguish a low-effort draft ledger, reflecting only the discussion's state, from the spec that planning assembles from it. | "The initial ledger would only be a draft, written with the lowest amount of effort, whose only goal is to reflect the state of the discussion." |
| a66 | 3 | owner | #plan-document-kinds, #plans-split-dirs | Avoid building a new kind or shape of register for this; split into docs/plans/milestones/ and docs/plans/specs/. | "I'm not too keen on building a new kind/shape of registers just for that." |
| a67 | 3 | owner | #plan-document-kinds, #plan-items-by-section | Plan documents follow reference constraints that differ from other registers, so a fully different model is acceptable. | "So a fully different model is acceptable for that." |
| a68 | 3 | owner | #plan-document-kinds, #plans-location | Keep the step file's basename (unique, prefixed, or cited `spec@<milestone-name>@harvest`); carrying the project name is odd with a single source. | "It seems kinda weird to carry the project name, when there is a single source." |
| a69 | 3 | owner | #argument-ids | Numbering avoids naming each argument, which is tedious and of little use since arguments are not harvested. | "not very useful since they do not get harvested in registers" |
| a70 | 3 | agent | #draft-ledger | The draft ledger matches the recorded alternative; low effort answers only the first reason, while "stale stated with confidence" and "only corroborant is the conversation" still apply. | "Approving it would reopen the alternative you just declined to reopen." |
| a71 | 3 | agent | #ledger-from-transcript | The per-round draft already exists: the design skill writes a delta table every round that changes something. | "the per-round draft is already written" |
| a72 | 3 | agent | #ledger-from-transcript | Measured: session `46dca865` was compacted at least 3 times; its first summary is record 1290 of 7177, and the 179 user and 353 assistant records before it are still in the file. | "the transcript on disk keeps every delta, every argument and every ruling verbatim, after any number of compactions" |
| a73 | 3 | agent | #ledger-from-transcript | The deltas are corroborated as written, since the owner read and answered each. | "the deltas are what you read and answered in each round" |
| a74 | 3 | agent | #ledger-from-transcript, #argument-ids | Argument numbers are assigned once at assembly, never minted during the discussion. | "the discussion never mints argument numbers" |
| a75 | 3 | agent | #ledger-from-transcript | Cost: the transcript path is harness-specific; the rule falls back to the conversation, as the transcript reviewer's "whose transcript is available" already does. | "The transcript path is specific to the harness." |
| a76 | 3 | agent | #ledger-from-transcript | Cost: this session's transcript is about 727 KB, so extraction is a subagent's job. | "This session's transcript is about 727 KB." |
| a77 | 3 | agent | #ledger-from-transcript | Cost: the extraction can be wrong; the transcript reviewer checks it against the same file. | "The extraction can be wrong." |
| a78 | 3 | agent | #plans-split-dirs, #plan-document-kinds | The core's `Shape` has `Heading` and `File`; a file register already checks a level-one title and declared level-two sections, which is planning's fixed section list. | "checked by code that exists" |
| a79 | 3 | agent | #plans-split-dirs, #plan-anchor | A milestone directory is a file register home (README, generated index, one file per entry), so `spec@<milestone>@harvest` is an ordinary reference, unique only inside its milestone. | "a step's basename only needs to be unique inside its milestone" |
| a80 | 3 | agent | #plans-location | A reserved anchor `plans` fixed by the tool, like `*` and `elsewhere`, removes the project name from citations. | "so the project name goes away" |
| a81 | 3 | agent | #plan-document-kinds | `milestone@plans@<id>` needs a directory entry, the one new piece of register shape; the free alternative is citing the README by path, kept out because the owner asked for milestone@. | "You asked for milestone@, so I keep it, and name it as the cost." |
| a82 | 3 | agent | #plan-items-by-section | Today a heading register has one home and one level and nothing scopes a citation, so items need their own model: section gives kind, scope is the plan anchor; a spec file as anchor is also new. | "Items need a model of their own" |
| a83 | 3 | agent | #plan-items-by-section | Extensions run in phase 4 after the entity table is built, so items cannot be an extension; they are core work. | "So the items cannot be added as an extension." |
| a84 | 3 | agent | #cross-plan-references | Under the owner's bound, B cites A whole and is revisited when A leaves; the only missed event (A reverses x while open) needs two concurrent dependent plans, which the owner said is rare, so the agent withdraws. | "whole-plan citations already give the dependency" |
| a85 | 4 | owner | #cross-plan-references, #retiring-plan-opens-issue | Item citations would force an in-depth revisit of B when A is removed, at a bad moment; whole-document citations prevent deep interdependence, and the retiring session only removes the citations and opens an issue. | "which forces design work at a moment that might not be the best" |
| a86 | 4 | agent | #retiring-plan-opens-issue | By the knowledge table, "may need revisiting" is something outstanding, so an issue is the right home. | "\"may need revisiting\" is something outstanding" |
| a87 | 4 | agent | #retiring-plan-opens-issue | The issue cites the citing plan, so if that plan leaves first, the citation dangles and the check fails: the issue cannot outlive its plan. | "So the issue cannot outlive the plan it is about." |
| a88 | 4 | agent | #retiring-plan-opens-issue | For a milestone B, the issue tells the next step's design audit (planning §7.2) what changed. | "The issue tells that audit what changed." |
| a89 | 4 | agent | #retiring-plan-opens-issue | The issue's `Why it matters` cites the leaving plan's harvested design entries, since the plan no longer exists after that commit; its path goes in the commit message. | "cites the leaving plan's harvested design entries, not the leaving plan" |
| a90 | 4 | agent | #roadmap-home | The owner's set-in-stone argument (a1) applies: docs/roadmap.md at the root, fixed name, optional, no new checker code. | "the same argument applies here" |
| a91 | 4 | agent | #roadmap-home | The rival, `path@plans@README.md`, breaks "the plans directory holds plan documents and nothing else", because a roadmap outlives every plan it lists. | "a roadmap outlives every plan it lists" |
| a92 | 4 | agent | #argument-segmentation | Where one argument ends in prose is a judgement the extracting subagent would make; default: consolidate at assembly, checked by the transcript reviewer. | "where one argument ends and the next begins is a judgement" |
| a93 | 4 | agent | #argument-segmentation | The rival, marking arguments in each round, makes extraction exact at a per-round cost, which the recorded alternative warned about. | "The cost is per round" |
| a94 | 5 | agent | (criterion existing-plans-readable; `structure-ready`) | The agent corrects R2: existing-plans-readable is binding as a presumption and blocks convergence; since the plan shape changes, `structure-ready` is reversed at harvest and the criterion leaves with it. | "In round 2 I called it \"weighed\", and that was wrong." |
| a95 | 5 | agent | (criterion writing-cost) | #spec-records-the-exchange and #arguments-as-items make a spec longer, so writing-cost is unmet unless accepted by name. | "make a spec longer" |
| a96 | 6 | owner | (thaum migration; existing-plans-readable) | thaum's plan documents need no rewrite until a new release and a version bump on thaum's side; thaum already migrated to v0.1. | "This will only happen when I make a new release of knowledge-architect, then bump the version on thaum's side." |
| a97 | 6 | agent | (thaum migration) | The agent concedes: `exact-pin` and `goal@knowledge-architect@any-project-can-adopt-it` already say a project moves when it chooses, so this costs thaum nothing until its pin moves. | "I had the fact and did not apply it." |
| a98 | 6 | agent | #roadmap-orders-issues | The thaum observation (`path@elsewhere@docs/next-milestones.md` and its roadmap exception) stays valid evidence; only the migration cost was wrong. | "Only the migration cost was wrong." |

## New names, in one place

An illustration of the shapes; the names are the design's, the files are where each is expected to
go. A name marked existing exists in the code today.

```text
crates/core/src/entity.rs
  ESCAPE_ANCHOR, EVERY_ANCHOR        existing: the reserved anchor words
  PLANS_ANCHOR = "plans"             new: the reserved word of the plans anchor
  Anchor, Anchors                    existing; gain the plans anchor and the plan anchors
crates/core/src/manifest.rs
  Shape { Heading, File }            existing
  Shape::Directory                   new (step 1): the milestone register's entries (D10)
  Registers::built_in                existing; gains the spec and milestone registers of `plans`
kinds (reference heads)
  spec, milestone                    new (step 1)
  thread, argument, criterion, acceptance    new (step 2)
level-two section titles of a plan document that define items (step 2)
  Threads, Arguments, Criteria, Acceptance criteria
files
  docs/plans/specs/README.md, docs/plans/specs/index.md               new (step 1)
  docs/plans/milestones/README.md, docs/plans/milestones/index.md     new (step 1)
  docs/plans/milestones/<id>/index.md                                 new, generated (step 1)
  docs/roadmap.md                                                     new, optional (step 3)
```

## Decided design

Where a paragraph below says "default Dn", the shape was the author's, written to answer a review
finding inside an approved thread, and the owner has ruled it; the list is in "Defaults awaiting
the owner".

**The plans anchor** (#plans-location, #plans-dir-fixed, #plans-at-root). The tool constructs one
anchor, `plans`, at the plans directory of the root Component. Its name is reserved like `*` and
`elsewhere`. It is built as a location the tool constructs (default D6), carrying the two
registers below as built-in registers that no other anchor carries and no project may declare
(default D1). A Component already gets its storage at fixed paths, and a built-in register refuses
a `dir` key, so a declared plans directory would be the only declared path in that family (a9).
One plans directory, because most work spans Components (a15, which the owner endorsed in round 2),
because there should never be so many plans open at once that a split makes sense (a39, the
owner's), and because plans designed before another is built break each other (a40, the owner's).
The reference grammar names the anchor, so a later split adds citations and rewrites none (a13).
Nearest rival: #plans-per-anchor. The agent added a cost against it, an empty home in every
Component (a14); the owner did not argue that point.

**Two registers under it** (#plans-split-dirs, #plan-document-kinds). `spec`: home
docs/plans/specs/, the File shape, an entry `<id>.md`, cited `spec@plans@<id>`. `milestone`:
home docs/plans/milestones/, an entry a directory `<id>/` holding a `README.md`, cited
`milestone@plans@<id>`. The directory entry is the one new register shape, kept because the
owner asked for the `milestone` kind (a42, a81); its rules are default D10. The specs under
`specs/` and the step specs of a milestone are one kind, `spec`, in different anchors: the owner
asked that specs be cited spec@...@... (round 2), proposed `spec@<milestone-name>@harvest` for a
step (round 3), and approved the shape (round 4). Nearest rival: one register with two kinds taken
from the entry's shape (a51), defeated by the owner's "I'm not too keen on building a new
kind/shape of registers just for that." (a66).

**A milestone directory is an anchor** (#plan-anchor, #layout-kept). Named by its basename, implied
by the tree (default D11), carrying a `spec` register whose home is the directory itself (default
D9): its `README.md` is the milestone document, its `index.md` is generated and lists the step
specs, every other `.md` is a step spec, cited `spec@<milestone>@<step>`. The File shape already
has a hand-written README, a generated index and one file per entry (a79). A step's basename need
only be unique in its milestone (a52, a68).

**A spec file is an anchor too**, for its items only (#plan-anchor, #plan-items-by-section). This is
new: today every anchor is a directory (a82). It is built in step 2. Nearest rival to plan anchors:
shape (a) of #plan-item-scope, which keeps the root anchor in every citation (a55, a56).

**Clause P1.** A plan anchor carries no `path` kind. A `path` citation of a plan document is
refused, and the repair names the `spec` or `milestone` form. One name per document keeps
`cargo klarch show` complete (a53). The clause as approved refuses "a `path` citation of a file
under docs/plans/specs/ or docs/plans/milestones/" and lets `plans` carry `path` "for its own
README files"; the READMEs and indexes of the two homes satisfy both halves, so the reading is
default D8: a plan document is a spec file, a milestone directory, or a file inside a milestone
directory, and the README and index files of the plans directory and of its two homes are cited
`path@plans@<file>`. The plans directory holds nothing else (default D17).

**Clause P2.** A plan name that is also the name of a Component, a location (default D15) or a
reserved anchor is a finding, and so is one name under both `specs/` and `milestones/`. It applies
in step 1 to milestone
names and spec basenames alike (default D13), and it is a phase-2 finding (default D11).

**Items** (#plan-items-by-section, #arguments-as-items). An item is a heading
`### <statement> ##<id>` under one of four level-two sections, and the section gives its kind:
Threads → `thread`, Arguments → `argument`, Criteria → `criterion`, Acceptance criteria →
`acceptance`. Items are core work in the entity table, because an extension runs after the table is
built (a83). Items need a model of their own, since a heading register has one home and one level
and nothing scopes a citation (a82, which replaces a25). An argument has an identifier and no
state: the design skill keeps arguments stateless because state would need a many-to-many
relation, and an identifier does not (a28). Named arguments let a ruling or a closure cite what
decided it, and turn the harvest of a standing argument into a selection (a29).

**The structure the checker reads** (#plan-register). Plan documents become a structure of the
checker: the plans anchor and its registers (step 1) and the items (step 2). The owner approved the
shape in round 2; its parts are the paragraphs above and below. Its rival, the owner's ruling that
plan documents stay free prose, was the issue's other closing condition and was not argued.

**Item citations** (#plan-item-scope). Shape (a'): `<kind>@<plan>@<id>`, the plan anchor in the
anchor position, as in `thread@structured-plans@roadmap-orders-issues`. An item is cited only from
inside its own plan anchor: the spec file, or the milestone directory. A commit message is outside
every plan anchor, so it cites a plan document whole, never an item (default D12). (a') keeps the
three-part grammar and the id grammar (a57), at about 45 characters where shape (a) needs 66 (a55,
a56). Nearest rival: shape (b), defeated by the core rule that refuses a reference with no anchor
(a58).

**Argument identifiers** (#argument-ids, #argument-segmentation). `a1`, `a2`, …, in order of
appearance, never reused, one sequence per milestone across its README and its step specs (a62).
Arguments are never harvested as entries, so a content slug would cost a name for nothing (a61,
a69). The boundaries are decided at assembly and checked by the transcript reviewer (a92). Nearest
rival: arguments marked in each round, defeated by its per-round cost (a93).

**The full record** (#spec-records-the-exchange). A plan document records every thread with its
proposer, final state, the arguments on each side, the owner's rulings verbatim with their round,
and its relations. It applies to a spec and to a milestone document (a43, a50). It applies also
to a step's spec when the step had a design session of its own (default D16). It fits
`design@agent-skills@standing-argument-in-head`, which already names the spec as the home of the
deliberation (a26). Its rival is the plan document as the planning skill shapes it today, which
records each thread's final state and resolution but neither the rulings verbatim nor the arguments
as items; it leaves #owner-rulings-recorded partly met.

**Assembly from the transcript** (#ledger-from-transcript, clause P3). The planning skill
dispatches a subagent that extracts, from every transcript the discussion spans, the delta tables,
the owner's rulings verbatim and the arguments, and assembles the plan document from them. Where the
harness keeps no transcript, it assembles from the conversation. The design skill's per-round delta
is the draft; the transcript on disk keeps the records from before a compaction (a71, a72, with the
correction noted under the arguments). Nearest rival: #draft-ledger, defeated because two of the
three reasons of the recorded lost alternative still apply to it (a70).

**The roadmap** (#roadmap-orders-issues, #roadmap-home). docs/roadmap.md in the root Component,
optional. Its rows are references to issue entries and to plan documents, in order, with an
unordered section allowed. A row dangles when its issue closes or its plan leaves, and the check
forces the edit (a21). The work stays in the issue register; the roadmap holds only order (a22).
An illustration of the shape:

```markdown
# Roadmap

## In order
1. `milestone@plans@<id>`       designed
2. `issue@<anchor>@<id>`        not designed

## Unordered
- `issue@<anchor>@<id>`
```

Nearest rival: #roadmap-register, defeated because two registers would hold undesigned work (a23).

**A leaving plan** (#retiring-plan-opens-issue, clause P8). When a plan leaves and another plan
cites it as a document, the session that meets the dangling citation, whether it retires the plan
or rebases the citing plan, removes the citation and opens an issue on the citing plan. The issue
says that the citing plan may need revisiting now that the leaving plan is built, "to account for
deviations or other unplanned happenings" (the owner, round 4). It cites the citing plan, so it
cannot outlive it (a87). Its `Why it matters` cites the leaving plan's harvested design entries
(a89). An item citation across plans is not available: #cross-plan-references was withdrawn.

**Built intent** (#design-home-is-built-intent, clause P9). A design home holds the design as built
and its reasons, and the code is checked against it. A plan document holds decided, unbuilt design.
Harvest stays at landing. Clause P9, as approved: the new primer wording covers decisions recorded
when made, that is, the decisions that no work implements, which
`design@agent-skills@harvest-after-implementation` already records when they are made. The
primer's sentence "A design home is intent" is reworded, and with it the root `CLAUDE.md`'s
"Intent" bullet, §0 of `knowledge-architect-decision-recording`,
`design@agent-skills@harvest-after-implementation` and `design@agent-skills@primer-content`; the
owner approved the four in round 3. Nearest rival: #design-home-is-intent, defeated by the false
defects it reports on unbuilt entries (a36).

**The name** (#plans-name-kept). `plans`: "planned" means intended or scheduled in common English,
which is the roadmap's content (a16).

## Mapping tables

| today | after this work |
| --- | --- |
| `path@knowledge-architect@docs/plans/<id>.md` | `spec@plans@<id>`, file `docs/plans/specs/<id>.md` |
| `path@knowledge-architect@docs/plans/<id>/README.md` | `milestone@plans@<id>`, file `docs/plans/milestones/<id>/README.md` |
| `path@knowledge-architect@docs/plans/<id>/<step>.md` | `spec@<id>@<step>`, file `docs/plans/milestones/<id>/<step>.md` |
| plain `#<id>` under a threads table | `### <statement> ##<id>` under Threads, cited `thread@<plan>@<id>` |
| an argument in prose | `### <statement> ##a<n>` under Arguments, cited `argument@<plan>@a<n>` |
| a criterion row | `### <statement> ##<id>` under Criteria, cited `criterion@<plan>@<id>` |
| an acceptance criterion | `### <statement> ##<id>` under Acceptance criteria, cited `acceptance@<plan>@<id>` |
| the knowledge-table row naming the plans directory | removed; the primer names docs/plans/ in plain text, since shipped text holds no live reference |
| a `todo` issue, unordered | a `todo` issue, and optionally a row of docs/roadmap.md |

## Losing alternatives

Every row is judged by the recording tests of `knowledge-architect-decision-recording` at the
harvest of the step that builds the decision it lost to; the harvest rows name the thread-level
ones.

| alternative | lost to | the deciding fact |
| --- | --- | --- |
| #plans-dir-declared | #plans-dir-fixed | a built-in register's storage is fixed by the tool (a9) |
| #plans-per-anchor | #plans-at-root | most work spans Components (a15); a later split rewrites no citation (a13) |
| #planned-name | #plans-name-kept | "planned" names scheduled work, the roadmap's content (a16) |
| #roadmap-register | #roadmap-orders-issues | two registers of undesigned work (a23) |
| #no-roadmap | #roadmap-orders-issues | thaum's exception (a20) |
| #design-home-is-intent | #design-home-is-built-intent | false defects on unbuilt entries (a36) |
| #spec-written-during-discussion | #ledger-from-transcript | no spec can be assembled while threads are open (a64) |
| #draft-ledger | #ledger-from-transcript | two recorded reasons still apply (a70) |
| #cross-plan-references | #plan-item-scope | whole-document citations carry the dependency (a84), and item citations force design work at a plan's retirement (a85) |
| shape (a) of #plan-item-scope | shape (a') | 66 characters against 45, and a root anchor that carries no information (a55, a56) |
| shape (b) of #plan-item-scope | shape (a') | the core refuses a reference with no anchor (a58) |
| one register with two kinds by entry shape | #plans-split-dirs | the owner's wish not to build a new register model (a66) |
| arguments marked in each round | #argument-segmentation at assembly | the per-round cost (a93) |
| content-named argument slugs | #argument-ids | a name for each of dozens of statements that are never harvested (a61, a69) |
| two registers sharing one home directory | #plans-split-dirs | the core has no rule for two registers at one directory (round 2); the owner's split gives each its own |
| keeping `path` citations of plan documents legal beside the kind form | #plan-document-kinds | `cargo klarch show spec@…` would miss the citations written as paths (a53) |
| no `milestone` kind, a milestone cited by a path to its README | #plan-document-kinds | the owner asked for milestone@ (a42); its cost, the directory entry, is named (a81) |
| unique step basenames across the plans directory, or prefixed with the milestone's name | #plan-document-kinds | the owner's own option (a68), set aside by the shape the owner approved, `spec@<milestone>@<step>`, where a basename is unique inside its milestone only (a79) |
| the roadmap in the plans directory's README | #roadmap-home | a roadmap outlives every plan it lists, and the plans directory holds plan documents only (a91) |

## Readings

Empty: the work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| P1: a `path` reference inside a milestone directory is ambiguous under deepest-anchor-wins | #plan-anchor, #plan-document-kinds | converted into clause P1 |
| P2: a spec and a milestone share a name, or a plan is named like a Component or a reserved anchor | #plans-location, #plan-anchor | converted into clause P2; claimed by a core test in step 1 |
| P3: the discussion spans several sessions and the assembly reads one transcript | #ledger-from-transcript | converted into clause P3 |
| P4: the harness changes its transcript format, or drops records from before a compaction | #ledger-from-transcript | tripwire T1, declined by the owner in round 6 |
| P5: the extracted record misses or misstates a ruling, and the error survives review | #ledger-from-transcript, #argument-segmentation | tripwire T2, recorded |
| P6: plan documents grow too long for the owner to read before commit | #spec-records-the-exchange, #arguments-as-items | tripwire T3, declined by the owner in round 6 |
| P7: a plan cannot cite another plan's item, so it describes one in words | #plan-item-scope | tripwire T4, recorded |
| P8: the citing plan is on another branch, so the retiring session cannot see it | #retiring-plan-opens-issue | converted into clause P8 |
| P9: a decision with no implementing work is excluded by the "built" wording | #design-home-is-built-intent | converted into clause P9 |
| P10: the roadmap's order goes stale | #roadmap-orders-issues | survives: a closed issue's row dangles and the check forces the edit |
| P11: a project needs several plans directories | #plans-at-root | tripwire T5, recorded |

The owner's ruling, round 6: "Keep tripwires T2, T4, T5. Not the others." Each is written at the
harvest of the decision it guards:

- **T2**, guarding #ledger-from-transcript, in the agent-skills tripwires home at step 3: a ruling
  of the owner missing from, or misstated in, a committed plan document, found after the commit.
- **T4**, guarding #plan-item-scope, in the core tripwires home at step 2: a plan document's prose
  names an item of another plan document.
- **T5**, guarding #plans-at-root, in the core tripwires home at step 1: a project asks to split
  its plans directory.

## Acceptance criteria

A fourth criterion, #assembly-holds-the-rulings, was judged at the commit that added this document
and fired: the transcript reviewers found rulings recorded wider than the owner made them. The
repair commit after the review before the merge reports it, and it left this table. Its evidence
goes to T2 at step 3's harvest.

Each criterion names the thread it guards rather than a reference: the decision has no design entry
until its harvest.

| criterion | decision guarded | judged at | fires when | response |
| --- | --- | --- | --- | --- |
| #milestone-fits-file-register | #plans-split-dirs, #plan-anchor | step 1 | the plans anchor and the milestone anchors need more than defaults D6, D9, D10 and D11 state: a new kind of anchor rather than a location the tool constructs, a `Shape` variant beyond the directory entry of D10, or a change to the File shape's rules for the `spec` register | stop step 1; put the shape back to the owner |
| #file-anchor-fits-path-model | #plan-anchor | step 2 | making a spec file an anchor needs a change to `Anchors::owning` or to the deepest-anchor rule beyond excluding plan anchors from the `path` kind | stop step 2; design session with the owner |
| #roadmap-needs-no-code | #roadmap-home | step 3 | writing the roadmap's instructions needs a checker rule that ordinary reference checking does not give | stop step 3; put it to the owner |

## Implementation sequence

1. [#plans-structure](plans-structure.md): the plans anchor, the `spec` and `milestone` registers,
   the milestone anchors, clauses P1 and P2; this repository's plans directory takes the new
   layout. Fails alone on: a tree whose plans homes hold nothing reports a finding.
2. [#plan-items](plan-items.md): items by section, spec files as anchors, citations scoped to their
   plan anchor; this document's identifiers become definitions. Fails alone on: an item is not
   defined, or a citation from outside its plan is accepted. Expected to open a design session at
   its audit, on the spec file as an anchor.
3. [#workflow-text](workflow-text.md): the installed skills, agents and primer; this repository's
   configuration; the roadmap instructions; built intent. Fails alone on: `cargo klarch check`
   reports the installed copy out of date, or the root `CLAUDE.md` contradicts the primer.

## Order rationale

- Step 1 before step 2: items need plan anchors to belong to, and step 1 proves the anchors with no
  items in them.
- Step 2 before step 3: the installed text describes the item syntax, which must exist in the
  checker it ships with.

## Defaults awaiting the owner

None. The owner ruled every default below.

**A material finding on #ledger-from-transcript, presented to the owner.** Argument a72 stated three
compactions; the re-measurement found one (see the notes under "Arguments"). The premise the
closure rests on, that the transcript keeps the records from before a compaction, holds on that one
compaction; "after any number of compactions" is not measured. Tripwire T1, which watched this
premise, was declined in round 6. Default: #ledger-from-transcript stands as approved, and T1 stays
declined. The finding was presented to the owner with this default, and the owner proceeded.

**Ruled.** The owner ruled D1 to D4 and D6 to D14 ("D1 to D14 approved, proceed"), then D15 to
D18 ("Agreed on those new defaults. proceed"). D5 was decided earlier and is in "Decided design"
under "Two registers under it". D18 revises D14.

| ruled default | thread it bears on |
| --- | --- |
| D15: clause P2 also makes a plan name that is the name of a location a finding. The clause as approved names a Component and a reserved anchor; a location is an anchor whose name a reference carries in the same position, so the same ambiguity follows | #plans-location |
| D16: #spec-records-the-exchange applies also to a step's spec when the step had a design session of its own, following §4 of the planning skill. The owner ruled on specs and milestone documents | #spec-records-the-exchange |
| D17: the plans directory holds nothing but its README and its two homes: any other file or directory directly under docs/plans/ is a phase-2 finding. Without it, nothing enforces that plan documents sit in the homes | #plans-split-dirs |
| D18: D14 revised. This document lands alone, in a pull request of its own merged under the current checker, rather than with step 1: under D14, step 1's checker would refuse the trees of this branch's earlier commits, which lack the plans homes, and the root `CLAUDE.md` requires every commit of a branch to pass under its tip's checker. Step 1's own commits that its checker refuses are squashed before review | #layout-kept |
| D1: the plans homes are required in the root Component: the `plans` anchor owes docs/plans/README.md, and `specs/` and `milestones/` each owe a README and a generated index. `spec` and `milestone` are built-in registers carried by `plans` alone; a project's `[registers.spec]` or `[registers.milestone]`, and a location naming either, are refused as any declaration of a built-in's storage is | #plans-dir-fixed |
| D2: enacted in the commit that adds this document, which the owner may reverse: that commit closes the core issue structured-plan-documents, per §2 of the planning skill, and repoints its four citations to this document | #plan-register |
| D3: this repository writes no docs/roadmap.md in this milestone | #roadmap-home |
| D4: the section titles of a plan document are checked from step 2, with the items, not in step 1 | #plan-items-by-section |
| D6: the `plans` anchor is built as a location the tool constructs, carrying the two registers, rather than as a new kind of anchor | #plans-location |
| D7: this document's record of arguments uses plain `a<n>` identifiers in a table under Threads until step 2, which moves them into a level-two Arguments section as definitions | #arguments-as-items |
| D8: the reading of clause P1. A plan document is a spec file, a milestone directory, or a file inside a milestone directory; a `path` citation of one is refused. The README and index files of the plans directory and of its two homes are cited `path@plans@<file>` | #plan-document-kinds |
| D9: the `spec` register's home depends on the anchor: `specs/` under `plans`, and the anchor's own directory under a milestone anchor | #plans-split-dirs |
| D10: the milestone entry. A third `Shape` variant, the only one this work adds. Its id is the directory's basename, in the entity-id grammar. A subdirectory of milestones/ without a README.md is a finding; a `.md` file directly under milestones/ other than README.md and index.md is a finding; a `register.toml` there is a finding. The entry checks (level-one title; the sections from step 2) apply to `<id>/README.md`. Its index row is the id and the README's level-one title, linking `<id>/README.md`. The index generator skips files owned by a nested anchor, as `file_definitions` does | #plan-document-kinds |
| D11: plan anchors are built from the tree in phase 2, from the walk's present paths, by every caller that builds the anchors of a tree, the per-commit trees of `commits` included. `collides` does not apply to them, because the plans layout places them; clause P2 is their collision check, a phase-2 finding planted in `unsound`. The generated paths kept out of the walk include each milestone's index.md, computed from the tree | #plan-anchor |
| D12: a commit message cites a plan document whole, never an item; an item citation in a message is refused like any citation from outside its plan | #plan-item-scope |
| D13: clause P2 applies in step 1 to milestone names and spec basenames alike | #plans-location |
| D14: the branches and the landing, as "How a step is worked" states them | #layout-kept |

## Harvest

| when | what lands where |
| --- | --- |
| step 1 | agent-skills design home: the reason of `design@agent-skills@plans-directory-declared` rewritten (see "What is already decided"). Core design home: the step 1 heads listed under "What is already decided"; #plan-register (its first part), #plans-location (rewriting `design@core@reserved-anchors`), #plans-dir-fixed, #plans-at-root, #plans-split-dirs, #plan-document-kinds, #plan-anchor (rewriting `design@core@anchors-are-components-and-locations`), clauses P1 (rewriting `design@core@every-path-names-its-anchor`) and P2; `design@core@components-carry-the-same-documents` rewritten for the plans homes. Core tripwires home: T5. Core rejected alternatives: #plans-per-anchor if the recording tests pass it |
| step 2 | core design home: the step 2 heads listed under "What is already decided"; #plan-register (its second part), #plan-item-scope, #plan-items-by-section, the spec file as anchor. Core tripwires home: T4. Core rejected alternatives: shapes (a) and (b), #cross-plan-references, if the recording tests pass them |
| step 3 | agent-skills design home: #design-home-is-built-intent (rewriting `design@agent-skills@harvest-after-implementation` and `design@agent-skills@primer-content`), #roadmap-orders-issues and #roadmap-home (reversing `design@agent-skills@planned-work-is-an-issue`), #spec-records-the-exchange, #arguments-as-items, #argument-ids, #argument-segmentation, #ledger-from-transcript (rewriting `design@agent-skills@design-hands-off-to-planning`), #retiring-plan-opens-issue; `design@agent-skills@plans-directory-declared`, `design@agent-skills@structure-ready` and `design@agent-skills@thread-slug-is-entry-id` reversed or rewritten; `design@agent-skills@document-vocabulary` and `design@agent-skills@milestone-is-a-directory` rewritten. Agent-skills tripwires home: T2, carrying the evidence of #assembly-holds-the-rulings (fired at the commit that added this document: three rulings recorded wider than the owner made them, clause P9, an item citation from a commit message and clause P2, each written by the author and none by the extraction, each repaired in place before the commit); `tripwire@agent-skills@ledger-lost-before-hand-off` judged, per "What is already decided". Agent-skills rejected alternatives: #roadmap-register, #design-home-is-intent, #draft-ledger, #planned-name, #spec-written-during-discussion, and the incumbents of the reversals, #plans-dir-declared, #no-roadmap and `design@agent-skills@structure-ready`, if the recording tests pass them |
| this document | leaves with step 3, in the commit that completes its harvest, whose message cites it as `milestone@plans@structured-plans` |

## Later consequences

- **thaum** moves its pin when the owner chooses. The owner ruled in round 6: "thaum's plan
  documents won't need a rewrite today. This will only happen when I make a new release of
  knowledge-architect, then bump the version on thaum's side." What that rewrite holds is thaum's
  work, decided then.
- **If T5 fires**, a split of the plans directory is proposed to the owner again; every citation
  already names its anchor, so a split rewrites none.
- **If T4 fires**, #cross-plan-references is proposed to the owner again.
