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
  identifier, written plain with a hash sign, as in `thread@structured-plans@plans-location`. Step 2 turns the threads',
  arguments', criteria's and acceptance criteria's into definitions; until then nothing checks
  them.
- **The record was assembled from the transcript**, as `thread@structured-plans@ledger-from-transcript` decides: a subagent
  extracted the owner's messages, the delta tables, the rulings and the arguments from the
  session log
  `~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/5519903e-6ef5-48cc-8d25-8de28599edfd.jsonl`,
  records 8 to 224, counted from 0. A `/clear` precedes record 8 and is excluded. Round numbers
  below are the
  rounds of that discussion: one owner message and the reply to it.

The step specs, in implementation order:

- Step 1, #plans-structure: landed; its spec left in the commit that completed its harvest.
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
     An answer that widens or narrows a ruling of the owner, or adds an obligation to one, is a
     scope change even when it is the one answer this document implies: it is also listed as a
     default awaiting the owner, who rules on it at the audit, before its point is implemented.
   - **Load-bearing.** The gap is material, or is a choice between two shapes neither of which this
     document rules out, or needs a ruling marked here as the owner's. Record it in the step's
     spec as open at the audit, with the discriminating fact, stop the step, and open a design
     session with the owner under `knowledge-architect-design`. Its converged design goes into the
     step's spec, in the sections of §4 of the skill, and owes the reviews of its §8. The step
     resumes from it.
3. **Claims, tests, implementation, gates, commit**, per `klarch-development`. The commit names how
   each claim's test was shown to fail against a wrong implementation, and says of any claim whose
   test cannot yet do so why not.
4. **Review before the merge**, per `knowledge-architect-review`. A repair is a further commit,
   or folded where that skill says. A finding not repaired becomes an issue entry.
5. **The report**: the landing commit reports on each acceptance criterion judged at this step.
6. **The harvest**, per the step's rows in the harvest section: the decisions and the losing
   alternatives under `knowledge-architect-decision-recording`, then the tripwires and the issues
   under `knowledge-architect-issue-tracking`. A tripwire names the head that harvested its
   decision, so the head is written first. Where a design home is a directory, a new subdocument is
   linked from its README. The harvest is reviewed before the merge, on the decision-record,
   routing and standing-state axes, and on transcript conformity where the transcript is available.
7. **The step's spec leaves** in the commit that completes its harvest, as in §9 of the skill. What
   crosses steps stays in this document, amended in place where the landing changed it.

## Names

| name | what it names |
| --- | --- |
| the core | crates/core, package `knowledge-architect`: the checker |
| the installed text | crates/agent-skills/content/: the skills, agents and primer the checker installs |
| the plans directory | `path@knowledge-architect@docs/plans/` of the root Component |
| the plans anchor, `plans` | the anchor this work adds at the plans directory (`thread@structured-plans@plans-location`) |
| plan document | a spec or a milestone document |
| plan anchor | the anchor of one plan document: a spec file under docs/plans/specs/, or a milestone directory under docs/plans/milestones/ (`thread@structured-plans@plan-anchor`) |
| item | a thread, an argument, a criterion or an acceptance criterion defined inside a plan document (`thread@structured-plans@plan-items-by-section`) |
| the roadmap | docs/roadmap.md of the root Component (`thread@structured-plans@roadmap-home`) |
| the transcript | the harness's session log of a discussion; in Claude Code, `~/.claude/projects/<project>/<session>.jsonl` |
| assembly | the planning skill writing a plan document from the transcript (`thread@structured-plans@ledger-from-transcript`) |
| built intent | what a design home holds after this work: the design as built and its reasons (`thread@structured-plans@design-home-is-built-intent`) |
| `Shape` | the enum in `path@core@src/manifest.rs` with the variants `Heading`, `File` and, since step 1, `Directory` |
| `Anchor`, `Anchors` | the types in `path@core@src/entity.rs` that hold the Components and locations |
| `resolve_anchors`, `collides` | the functions in `path@core@src/manifest.rs` that refuse a misnamed or misplaced anchor |
| P1 to P11 | the causes of the premortem, in its section below |
| T1 to T5 | the tripwires the premortem proposed; T2, T4 and T5 are recorded, in the premortem section below |
| D1 to D18 | the defaults, in "Defaults awaiting the owner" below: all ruled, D5 decided by the owner earlier |
| the presumed rows | the six threads closed by the checkpoint batch confirmation of round 7: `thread@structured-plans@plans-dir-declared`, `thread@structured-plans@plans-per-anchor`, `thread@structured-plans@planned-name`, `thread@structured-plans@roadmap-register`, `thread@structured-plans@no-roadmap`, `thread@structured-plans@design-home-is-intent` |
| `Shape::Directory` | the third variant of `Shape` that step 1 adds for the milestone register's entries (D10) |
| shape (a) of `thread@structured-plans@plan-item-scope` | an item cited with the root anchor and a compound id, `thread@knowledge-architect@<plan>/<id>`; it lost |
| shape (b) of `thread@structured-plans@plan-item-scope` | an item cited with no anchor, `thread@<id>`, resolved against the enclosing plan; it lost |

## What the work is

**Step 1 has landed.** The checker builds the plans anchor, its `spec` and `milestone` registers
and one anchor per milestone, and `design@core@plan-register` and the heads beside it record the
design. The list below describes the tree before step 1, and holds for what steps 2 and 3 change.

**Before step 1**, measured at main's commit 90a4b56:

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
- **Several plans directories.** Ruled out by `thread@structured-plans@plans-at-root`; tripwire T5 watches it.

## What is already decided

The design rests on these and does not argue them again:

- `design@core@generated-files-are-pure`: a generated index is a function of the walked tree.
- `design@core@an-extension-plugs-in-through-phased-hooks`: an extension runs after the entity table
  is built, so items are core work (`argument@structured-plans@a83`).
- `design@agent-skills@spec-leaves-at-landing`, `design@agent-skills@standing-argument-in-head`:
  what a plan document is for and when it leaves.
- `design@agent-skills@exact-pin`: why thaum is outside the work.

These are rewritten or reversed by this work, each at the harvest named in the harvest section:

- step 1, core: `design@core@anchors-are-components-and-locations`,
  `design@core@reserved-anchors`, `design@core@every-path-names-its-anchor`,
  `design@core@components-carry-the-same-documents`, `design@core@registers-are-declared` (it
  states that four registers are compiled in, which D1 makes six),
  `design@core@a-file-register-is-a-directory-of-entries` and
  `design@core@a-file-register-index-is-rows` (the directory entry of D10, and its index, rendered
  from the milestone anchors rather than filtered from the files, per step 1's audit finding 8);
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
"Writing the discussion's ledger to a file during the discussion". `thread@structured-plans@ledger-from-transcript` changes
its premise: the ledger is assembled from the transcript on disk. It has not fired: this session's
transcript holds no compaction marker. The discussion did not see it; the review before the merge
found it. Step 3's harvest rewrites it to watch assembly from the transcript, absorbs T2 into it if
both guard the same head, or deletes it, under `knowledge-architect-issue-tracking`.

What else references a decision this work rewrites, as `cargo klarch show` lists it, and the
harvest that judges each:

| entry | references | judged at |
| --- | --- | --- |
| a tripwire on the generic rule of `design@core@reserved-anchors` | that head | step 1: fired at step 1's review, and left; `issue@core@the-generic-anchor-accepts-a-location-s-copy` carries its response |
| `tripwire@core@issue-kind-list-grows` | `design@core@a-file-register-is-a-directory-of-entries` | step 1 |
| `issue@core@a-planned-path-can-be-named` | `design@core@reserved-anchors`, `design@core@every-path-names-its-anchor` | step 1 |
| `issue@core@cross-project-references` | `design@core@a-slug-belongs-to-a-component` | step 2 |
| `tripwire@agent-skills@ledger-lost-before-hand-off` | `design@agent-skills@design-hands-off-to-planning` | step 3, as above |
| the root `CLAUDE.md`, "Mechanical validation of documents", its bullet on heading-register entries | `design@core@an-entry-is-a-heading-at-the-register-level` | step 2, which updates it |
| the core's README, its section on heading registers | `design@core@an-entry-is-a-heading-at-the-register-level` | step 2, which updates it |
| the root `CLAUDE.md`, "Mechanical validation of documents", its bullet on the reference form | `design@core@a-slug-belongs-to-a-component` | step 2, which updates it |
| the comments of the core that cite it: the module comment of `path@core@src/check/references.rs`, and comments in `path@core@src/source/mod.rs` and `path@core@src/source/rs.rs` | `design@core@a-slug-belongs-to-a-component` | step 2, which judges each |
| the root `CLAUDE.md`, "Precedent is not authority", its restatement of the four cases | `design@agent-skills@primer-content` | step 3, which updates it with the primer |
| the root `CLAUDE.md`, "Plan documents" | `design@agent-skills@document-vocabulary` | step 3, already in its spec |

The heads in the design homes that cite these decisions are judged at the harvest of the step that
rewrites the decision they cite.

## Criteria

A ninth criterion, existing-plans-readable (binding, from `design@agent-skills@structure-ready`),
left with that decision: the owner approved its reversal in round 6, "structure-ready reversal
approved."

### Known open work is listed in one place `##one-place-for-open`

- **Kind:** binding
- **Source:** `goal@knowledge-architect@structure-and-workflow-work-together`
- **Satisfaction:** met: `thread@structured-plans@roadmap-orders-issues`, `thread@structured-plans@roadmap-home`

### Every citation of a plan or of its items is checked `##citations-checked`

- **Kind:** binding
- **Source:** `goal@knowledge-architect@documentation-stays-consistent`
- **Satisfaction:** met: `thread@structured-plans@plan-register`, `thread@structured-plans@plan-items-by-section`, `thread@structured-plans@plan-item-scope`, `thread@structured-plans@plans-location`

### The installed text works in any project `##installed-text-anywhere`

- **Kind:** binding
- **Source:** `goal@agent-skills@installed-text-works-anywhere`
- **Satisfaction:** met: `thread@structured-plans@plans-dir-fixed` is the tool's convention; `thread@structured-plans@ledger-from-transcript` falls back where no transcript exists

### The deliberation behind a decision stays reachable `##deliberation-reachable`

- **Kind:** binding
- **Source:** `goal@knowledge-architect@design-is-recorded-with-its-arguments`
- **Satisfaction:** met: `thread@structured-plans@spec-records-the-exchange`, and history after the plan leaves

### The owner's rulings are recorded as the owner made them `##owner-rulings-recorded`

- **Kind:** binding
- **Source:** `goal@knowledge-architect@the-owner-decides`
- **Satisfaction:** met: `thread@structured-plans@spec-records-the-exchange`, `thread@structured-plans@ledger-from-transcript`

### A plan document leaves when its work lands `##plan-leaves-at-landing`

- **Kind:** binding, as a presumption
- **Source:** `design@agent-skills@spec-leaves-at-landing`
- **Satisfaction:** met: unchanged; `thread@structured-plans@retiring-plan-opens-issue` adds the procedure for citations of a leaving plan

### A discussion survives the compaction of its conversation `##discussion-survives-compaction`

- **Kind:** weighed, the agent's (round 1)
- **Source:** the design skill's step 8: the ledger lives in the conversation
- **Satisfaction:** met: `thread@structured-plans@ledger-from-transcript`, where a transcript exists

### A plan document stays cheap to write `##writing-cost`

- **Kind:** weighed
- **Source:** the length of a plan document
- **Satisfaction:** unmet-and-accepted: the owner named it in round 7, "The presumed rows, writing-cost and the five clauses are all approved."

## Threads

29 threads, one item each. Each gives the final state, the resolution, the arguments that moved it (each an item of the Arguments section), the section carrying its shape, and the durable home expected to
harvest it. A thread whose decision earns an entry keeps its slug, per
`design@agent-skills@thread-slug-is-entry-id`; whether it earns one is decided at its harvest, under
`knowledge-architect-decision-recording`.

Rulings on items that are not threads:

| item | round | the owner's words (verbatim) |
| --- | --- | --- |
| reversal of `design@agent-skills@structure-ready` (criterion existing-plans-readable leaves with it) | R6 | "structure-ready reversal approved." |
| thaum migration out of scope | R6 | "thaum's plan documents won't need a rewrite today. This will only happen when I make a new release of knowledge-architect, then bump the version on thaum's side. thaum migration is not in scope of this work, it no longer needs now that v0.1 is released and thaum has already migrated to it." |
| tripwires T1-T5 | R6 | "Keep tripwires T2, T4, T5. Not the others." |
| criterion writing-cost (unmet-and-accepted), the 6 presumed rows, clauses P1, P2, P3, P8, P9 | R7 | "The presumed rows, writing-cost and the five clauses are all approved." |
| the thaum evidence | R7 | "You were right to go looking for thaum, and the evidence you found there was indeed useful." |
| permission to proceed to planning | R7 | "You can proceed." |

### The plans directory is `path@knowledge-architect@docs/plans/`, fixed by the tool `##plans-dir-fixed`

- **First appears:** R1 (agent names the owner's proposal)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a1`, `argument@structured-plans@a9`, `argument@structured-plans@a11`, `argument@structured-plans@a12`
- **Shape in:** decided design
- **Harvest home:** core design home (step 1); reverses `design@agent-skills@plans-directory-declared` (step 3)
- **Closed by:** R2: "I agree with plans-dir-fixed."

### One plans directory, in the root Component `##plans-at-root`

- **First appears:** R1
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a2`, `argument@structured-plans@a13`, `argument@structured-plans@a15`, `argument@structured-plans@a39`, `argument@structured-plans@a40`
- **Shape in:** decided design
- **Harvest home:** core design home (step 1)
- **Closed by:** R2: "I agree with plans-at-root. It can always be reopened later."

### The name stays `plans` `##plans-name-kept`

- **First appears:** R1
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a16`, `argument@structured-plans@a17`, `argument@structured-plans@a18`, `argument@structured-plans@a19`
- **Shape in:** decided design
- **Harvest home:** agent-skills `design@agent-skills@document-vocabulary`, rewritten (step 3)
- **Closed by:** R2: "Agreed on plans-name-kept."

### The roadmap holds order only; every row is a checked reference `##roadmap-orders-issues`

- **First appears:** R1
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a5`, `argument@structured-plans@a20`, `argument@structured-plans@a21`, `argument@structured-plans@a22`, `argument@structured-plans@a41`, `argument@structured-plans@a98`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home, reversing `design@agent-skills@planned-work-is-an-issue` (step 3)
- **Closed by:** R2: "roadmap-orders-issues: approved. I like that shape. It brings together multiple register shapes to form the roadmap using only references, this is the right solution."

### Docs/roadmap.md at the root; fixed name, optional; no checker code `##roadmap-home`

- **First appears:** R4 (agent)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a1`, `argument@structured-plans@a90`, `argument@structured-plans@a91`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home (step 3)
- **Closed by:** R5: same sentence as row 27

### A spec is one file; a milestone is a directory of a README and one spec per step `##layout-kept`

- **First appears:** R1
- **Final state:** approved
- **Arguments:** none beyond the owner's word
- **Shape in:** decided design
- **Harvest home:** `design@agent-skills@milestone-is-a-directory`, rewritten (step 3)
- **Closed by:** R2: "layout-kept: approved."

### Plan documents are a structure the checker reads `##plan-register`

- **First appears:** R1
- **Final state:** approved (shape)
- **Arguments:** `argument@structured-plans@a25`
- **Shape in:** decided design
- **Harvest home:** core design home (steps 1 and 2)
- **Closed by:** R2: "plan-register: Agreed with the shape."

### Kinds `spec` and `milestone`; a `path` citation of a plan document is refused; a whole plan document may be cited from anywhere `##plan-document-kinds`

- **First appears:** R2 (agent names the owner's R2 side note)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a42`, `argument@structured-plans@a51`, `argument@structured-plans@a52`, `argument@structured-plans@a53`, `argument@structured-plans@a54`, `argument@structured-plans@a66`, `argument@structured-plans@a68`, `argument@structured-plans@a81`
- **Shape in:** decided design
- **Harvest home:** core design home (step 1)
- **Closed by:** R4: "plan-document-kinds: shape approved, including plans-location and plan-items-by-section." Points 2-4, R3: "Agreed on points 2,3 and 4 though."

### Docs/plans/specs/ and docs/plans/milestones/ `##plans-split-dirs`

- **First appears:** R3 (delta only; the owner's R3 proposal)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a66`, `argument@structured-plans@a78`, `argument@structured-plans@a79`
- **Shape in:** decided design
- **Harvest home:** core design home (step 1)
- **Closed by:** no explicit word naming it; the agent recorded it approved in R4 on the owner's R4 "plan-document-kinds: shape approved", which absorbs it

### The plans directory is an anchor `plans`, fixed by the tool and reserved `##plans-location`

- **First appears:** R3 (agent)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a68`, `argument@structured-plans@a80`
- **Shape in:** decided design
- **Harvest home:** core `design@core@reserved-anchors`, rewritten (step 1)
- **Closed by:** R4: "plan-document-kinds: shape approved, including plans-location and plan-items-by-section."

### Each plan document is an anchor: a spec file or a milestone directory `##plan-anchor`

- **First appears:** R2 (delta only)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a57`, `argument@structured-plans@a79`
- **Shape in:** decided design
- **Harvest home:** core `design@core@anchors-are-components-and-locations`, rewritten (steps 1 and 2)
- **Closed by:** no explicit word naming it; the agent closed it in R3 on the owner's R3 "a' looks fine to me", and listed it as absorbed into `thread@structured-plans@plan-item-scope`

### Shape (a'), `<kind>@<plan>@<id>`; an item is cited only from inside its own spec file or milestone directory `##plan-item-scope`

- **First appears:** R1
- **Final state:** approved as (a')
- **Arguments:** `argument@structured-plans@a6`, `argument@structured-plans@a55`, `argument@structured-plans@a56`, `argument@structured-plans@a57`, `argument@structured-plans@a58`
- **Shape in:** decided design
- **Harvest home:** core design home (step 2)
- **Closed by:** R2: "plan-item-scope: still unsure between options a and b." then R3: "plan-item-scope: a' looks fine to me. See my proposal above too, it aligns with that." The bound came from R3: "Mine is still the same for now: bound to the individual spec or milestone."

### An item is `### <statement> ##<id>` under a fixed level-two section, which gives its kind `##plan-items-by-section`

- **First appears:** R3 (agent)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a67`, `argument@structured-plans@a82`, `argument@structured-plans@a83`
- **Shape in:** decided design
- **Harvest home:** core design home (step 2)
- **Closed by:** R4: same sentence as row 25

### The plan document records the whole discussion: proposer, final state, arguments, rulings verbatim with their round, relations `##spec-records-the-exchange`

- **First appears:** R1 (the owner's proposal)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a6`, `argument@structured-plans@a26`, `argument@structured-plans@a27`, `argument@structured-plans@a43`, `argument@structured-plans@a50`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home (step 3)
- **Closed by:** R2: "spec-records-the-exchange: approved. I think this should also apply to milestones. Milestones are created through the design skill too after all. It contains a README, and an individual spec for each step in the milestone. I think the initial design discussion of the milestone writes its README, right ? Then the README is the \"milestone spec\"."

### Arguments are citable items without state `##arguments-as-items`

- **First appears:** R1
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a6`, `argument@structured-plans@a28`, `argument@structured-plans@a29`, `argument@structured-plans@a30`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home (step 3)
- **Closed by:** R2: "arguments-as-items: approved. Being able to cross reference arguments is certainly much better."

### `a1`, `a2`, …, never reused; one sequence per milestone `##argument-ids`

- **First appears:** R2
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a61`, `argument@structured-plans@a62`, `argument@structured-plans@a69`, `argument@structured-plans@a74`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home (step 3)
- **Closed by:** R3: "argument-ids: agreed. This avoids having to name each argument, which could be quite tedious, and not very useful since they do not get harvested in registers."

### Argument boundaries are decided at assembly, checked by the transcript reviewer `##argument-segmentation`

- **First appears:** R4 (agent)
- **Final state:** approved, shape "at assembly"
- **Arguments:** `argument@structured-plans@a92`, `argument@structured-plans@a93`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home (step 3)
- **Closed by:** R5: same sentence as row 27

### The planning skill assembles from the transcript through a subagent; without a transcript, from the conversation `##ledger-from-transcript`

- **First appears:** R3 (agent)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a71`, `argument@structured-plans@a72`, `argument@structured-plans@a73`, `argument@structured-plans@a74`, `argument@structured-plans@a75`, `argument@structured-plans@a76`, `argument@structured-plans@a77`
- **Shape in:** decided design
- **Harvest home:** agent-skills `design@agent-skills@design-hands-off-to-planning`, rewritten (step 3)
- **Closed by:** R4: "ledger-from-transcript approved, it's better than my draft ledger."

### Design homes hold built intent; plans hold unbuilt intent; harvest stays at landing `##design-home-is-built-intent`

- **First appears:** R1 (agent)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a7`, `argument@structured-plans@a8`, `argument@structured-plans@a33`, `argument@structured-plans@a34`, `argument@structured-plans@a45`
- **Shape in:** decided design
- **Harvest home:** agent-skills `design@agent-skills@harvest-after-implementation` and `design@agent-skills@primer-content`, rewritten (step 3)
- **Closed by:** R2: "design-home-is-built-intent: I agree. Once plans become structured registers, the distinction is clear, and design docs can carry the meaning of \"built intent\", while plans carry \"unbuilt intent\"." Its four consequences, R3: "Approved on the consequences of design-home-is-built-intent"

### A leaving plan's session removes whole-document citations of it from other plans and opens an issue on each citing plan `##retiring-plan-opens-issue`

- **First appears:** R4 (agent names the owner's R4 procedure)
- **Final state:** approved
- **Arguments:** `argument@structured-plans@a85`, `argument@structured-plans@a86`, `argument@structured-plans@a87`, `argument@structured-plans@a88`, `argument@structured-plans@a89`
- **Shape in:** decided design
- **Harvest home:** agent-skills design home (step 3)
- **Closed by:** R5: "retiring-plan-opens-issue and roadmap-home approved, argument-segmentation at assembly"

### The plan document is written during the discussion, round by round `##spec-written-during-discussion`

- **First appears:** R1 (agent)
- **Final state:** withdrawn (by the agent, R3)
- **Defeating reason:** no spec can be assembled while its threads are open (the owner's `argument@structured-plans@a64`); it was also the recorded lost alternative "Writing the discussion's ledger to a file during the discussion"
- **Arguments:** `argument@structured-plans@a31`, `argument@structured-plans@a32`, `argument@structured-plans@a44`, `argument@structured-plans@a46`, `argument@structured-plans@a47`, `argument@structured-plans@a48`, `argument@structured-plans@a49`, `argument@structured-plans@a64`
- **Harvest home:** judged at step 3 by `design@agent-skills@losing-shape-test`; the recorded alternative stays as it is
- **Closed by:** R3: "spec-written-during-discussion: on second thought, I'm not too keen on reopening this rejected alternative." The agent withdrew it in its R3 reply, with the owner's argument as the defeating reason.

### A draft ledger of the discussion's state is written to a file each round, with the least effort `##draft-ledger`

- **First appears:** R3 (agent names the owner's R3 idea)
- **Final state:** withdrawn (by the owner)
- **Defeating reason:** two of the recorded alternative's three reasons still apply; `thread@structured-plans@ledger-from-transcript` answers all three
- **Arguments:** `argument@structured-plans@a65`, `argument@structured-plans@a70`
- **Harvest home:** judged at step 3
- **Closed by:** R4: "ledger-from-transcript approved, it's better than my draft ledger." (closes it by implication)

### The plans directory is named "planned" `##planned-name`

- **First appears:** R1 (the owner's proposal)
- **Final state:** withdrawn (by the owner, as presumed by the agent)
- **Defeating reason:** the owner's own proposal; the agent read round 2's "Agreed on plans-name-kept." as its withdrawal, and the owner confirmed in round 7. "planned" names scheduled work, the roadmap's content (`argument@structured-plans@a16`)
- **Arguments:** `argument@structured-plans@a4`, `argument@structured-plans@a16`
- **Harvest home:** judged at step 3
- **Closed by:** no explicit word naming it; the agent read R2 "Agreed on plans-name-kept." as withdrawing it; confirmed by the checkpoint batch, R7

### A plan may cite an item of another plan `##cross-plan-references`

- **First appears:** R2
- **Final state:** withdrawn (by the agent, R3)
- **Defeating reason:** few plans are open at once and a whole-document citation carries the dependency (`argument@structured-plans@a84`); item citations would force design work at the moment of retiring a plan (`argument@structured-plans@a85`)
- **Arguments:** `argument@structured-plans@a59`, `argument@structured-plans@a60`, `argument@structured-plans@a84`, `argument@structured-plans@a85`
- **Harvest home:** judged at step 2; tripwire T4
- **Closed by:** no explicit word closing it; closed by the agent's withdrawal in R3 after the owner's R3: "cross-plan-references: what do you mean with wider bound ? I do not understand your position here. Mine is still the same for now: bound to the individual spec or milestone." The owner added a second argument in R4 (see `argument@structured-plans@a85`); the state stayed withdrawn.

### A project declares its plans directory in its own rows of the knowledge table `##plans-dir-declared`

- **First appears:** R1
- **Final state:** ruled-out
- **Reason:** the storage of a built-in register is fixed by the tool
- **Arguments:** `argument@structured-plans@a9`, `argument@structured-plans@a11`
- **Harvest home:** the reversal of `design@agent-skills@plans-directory-declared` (step 3)
- **Closed by:** no explicit word; closed by the checkpoint batch confirmation, R7: "The presumed rows, writing-cost and the five clauses are all approved."

### Each Component may have a plans directory of its own `##plans-per-anchor`

- **First appears:** R1
- **Final state:** ruled-out
- **Reason:** most work spans Components; the reference grammar makes a later split free
- **Arguments:** `argument@structured-plans@a3`, `argument@structured-plans@a13`, `argument@structured-plans@a14`, `argument@structured-plans@a15`, `argument@structured-plans@a39`
- **Harvest home:** judged at step 1; tripwire T5
- **Closed by:** no explicit word; closed by the checkpoint batch confirmation, R7 (same sentence as row 2)

### The roadmap is a register of its own, holding undesigned work `##roadmap-register`

- **First appears:** R1
- **Final state:** ruled-out
- **Reason:** two registers of undesigned work fail `criterion@structured-plans@one-place-for-open`
- **Arguments:** `argument@structured-plans@a23`
- **Harvest home:** judged at step 3
- **Closed by:** no explicit word; closed by the checkpoint batch confirmation, R7

### There is no roadmap `##no-roadmap`

- **First appears:** R1
- **Final state:** ruled-out
- **Reason:** thaum's exception showed the need
- **Arguments:** `argument@structured-plans@a20`, `argument@structured-plans@a24`
- **Harvest home:** the reversal of `design@agent-skills@planned-work-is-an-issue` (step 3)
- **Closed by:** no explicit word; closed by the checkpoint batch confirmation, R7

### A design home holds intent, built or not, and the code is checked against it `##design-home-is-intent`

- **First appears:** R1
- **Final state:** ruled-out
- **Reason:** needs a per-entry built or unbuilt marker, and reports false defects on unbuilt entries
- **Arguments:** `argument@structured-plans@a35`, `argument@structured-plans@a36`, `argument@structured-plans@a37`, `argument@structured-plans@a38`
- **Harvest home:** judged at step 3
- **Closed by:** no explicit word; closed by the checkpoint batch confirmation, R7

## Arguments

Every argument of the discussion, numbered in order of appearance, per `thread@structured-plans@argument-ids` and
`thread@structured-plans@argument-segmentation`. The boundaries were decided at assembly. Row `argument@structured-plans@a85` is the owner's second
argument on `thread@structured-plans@cross-plan-references`, given in round 4.

Notes on the table, from the review of this document:

- **`argument@structured-plans@a25` was replaced by `argument@structured-plans@a82`** in round 3: items need a model of their own, and the existing
  heading-entry code does not read them as it stands.
- **`argument@structured-plans@a72` does not reproduce as stated**, and the correction is the author's. The round-3 count
  matched the compaction markers as a substring of each record, so it also counted records whose
  text only mentioned a marker. Re-taken by record fields (`type` equal to `system` with `subtype`
  equal to `compact_boundary`, and `isCompactSummary`), over
  `~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/46dca865-fb8e-4006-be09-548d0d1ca801.jsonl`:
  7177 records, one compaction, its boundary at record 3738 counted from 0, and 512 user and 932
  assistant records before it, all present. The premise `argument@structured-plans@a72` serves, that the transcript keeps the
  records from before a compaction, holds on one observed compaction.
- **Instruments of the other figures**: the reference lengths of `argument@structured-plans@a55` are character counts of the
  three example spans; "about 727 KB" (`argument@structured-plans@a76`) was `ls -la` of this session's transcript in round 3;
  "thaum's 3 plan documents" (`argument@structured-plans@a27`) and "about 5 entries" (`argument@structured-plans@a22`) were `ls` of thaum's plans directory
  and a reading of its `next-milestones.md`.
- **Spans normalised**: where a quotation held a path in backticks, the backticks were removed or
  the path anchored so that the checker reads it, in `argument@structured-plans@a10` (backticks removed), `argument@structured-plans@a20`, `argument@structured-plans@a46` and `argument@structured-plans@a98`.

### Fixing the plans directory makes skills easier to write, and much is already fixed by the tool `##a1`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@plans-dir-fixed`, `thread@structured-plans@plans-dir-declared`
- **Key words:** "I'd recommand set in stone to facilitate writing skills and such, after all a lot is already set in stone by the project"

### In both projects that used the workflow, one plans directory was enough `##a2`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@plans-at-root`
- **Key words:** "I've only ever needed it to be unique in the projects I made with this workflow"

### A much larger project with big Components, which can have sub-Components, might want to split the plans directory `##a3`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@plans-per-anchor`
- **Key words:** "a much larger project with big components (components can have subcomponents btw) might want to split the plan directory"

### "planned" is more correct because the directory holds already designed work with decided threads `##a4`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@planned-name`
- **Key words:** "\"planned\" is actually a bit more correct, because it holds already designed work, with decided threads"

### The owner asks for a way to declare an optionally ordered list of wanted work that is not yet designed `##a5`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@roadmap-orders-issues`, `thread@structured-plans@roadmap-register`, `thread@structured-plans@no-roadmap`
- **Key words:** "a \"roadmap\" feature, which would allow declaring an (optionally ordered) list of work that we want to do, but is not necessarily already designed"

### The plan structure should integrate with the design skill's threads; the initial spec should be a precise record of the discussion, with cross references between its items bounded to the spec or milestone directory `##a6`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@spec-records-the-exchange`, `thread@structured-plans@arguments-as-items`, `thread@structured-plans@plan-item-scope`, `thread@structured-plans@cross-plan-references`
- **Key words:** "with the possibility of cross referencing all these items, bounded to the spec document or the milestone directory"

### Design registers could reflect intent, yet the standing directive keeps them current against the code; this makes the moment of harvest unclear `##a7`

- **Round:** 1
- **Who:** owner
- **Threads:** `thread@structured-plans@design-home-is-built-intent`, `thread@structured-plans@design-home-is-intent`
- **Key words:** "There is kind of a contradiction between expected usage and the workflow of design registers."

### The record contradicts itself: the primer calls a design home intent, while `harvest-after-implementation` calls a design head a claim about the code as it stands `##a8`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-built-intent`, `thread@structured-plans@design-home-is-intent`
- **Key words:** "One says a design home is intent, the other that it is a claim about the code."

### A Component already gets its storage at fixed paths from the tool, and a built-in register refuses a `dir` key; a declared plans directory would be the only declared path in that family `##a9`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-dir-fixed`, `thread@structured-plans@plans-dir-declared`
- **Key words:** "A declared directory would be the only declared path in that family."

### Cost: thaum already uses docs/plans/, so it pays nothing `##a10`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-dir-fixed`
- **Key words:** "thaum already uses docs/plans/, so it pays nothing"

### The reason recorded for the declared directory (a manifest key nothing reads) is gone once the checker reads plans `##a11`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-dir-fixed`, `thread@structured-plans@plans-dir-declared`
- **Key words:** "Once a checker reads it, that reason is gone."

### A path the tool fixes is the tool's own convention, so the goal that nothing of a project is compiled in still holds `##a12`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-dir-fixed`
- **Key words:** "A path the tool fixes is not a project compiled in."

### The reference grammar already names the anchor, so a later split adds `plan@core@<id>` and rewrites no existing citation `##a13`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-at-root`, `thread@structured-plans@plans-per-anchor`
- **Key words:** "the reference grammar already names the anchor"

### Because every Component carries every built-in home, per-anchor plans mean 4 more empty plans directories, or a new class of optional built-in register `##a14`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-per-anchor`
- **Key words:** "4 more plans directories with READMEs, holding nothing"

### Most work crosses Components (this issue touches core and agent-skills; the v0.1 milestone touched every Component), so such plans sit at the root anyway `##a15`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-at-root`, `thread@structured-plans@plans-per-anchor`
- **Key words:** "Most work crosses Components."

### In common English "planned work" means intended or scheduled work, designed or not, which is the roadmap's content `##a16`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@planned-name`, `thread@structured-plans@plans-name-kept`
- **Key words:** "If a roadmap thread is approved, \"planned\" names the wrong one of the two."

### `document-vocabulary` makes common developer usage binding, and its terms are "plan document" and "plans directory" `##a17`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-name-kept`
- **Key words:** "\"Plan document\" and \"plans directory\" are its terms."

### Every directory under `docs/` is named by a noun `##a18`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-name-kept`
- **Key words:** "Every directory under `docs/` is named by a noun"

### No common single word says "designed", so the README and skills must carry that meaning `##a19`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-name-kept`, `thread@structured-plans@planned-name`
- **Key words:** "\"plans\" does not say it either. No common single word I know does."

### thaum's `path@elsewhere@docs/next-milestones.md` exception is real use in the sense of `additions-need-real-use`, which is new evidence for reopening `planned-work-is-an-issue` `##a20`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@no-roadmap`, `thread@structured-plans@roadmap-orders-issues`, `thread@structured-plans@roadmap-register`
- **Key words:** "The new evidence is thaum's exception, which is real use"

### Drift is bounded mechanically: a closed issue's row dangles and fails the check; writing a plan closes its issue; `show` lists the roadmap as a dependent `##a21`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@roadmap-orders-issues`
- **Key words:** "Drift is bounded mechanically"

### It meets one-place-for-open, since the work stays in the issue register and the roadmap holds only order; cost for thaum about 5 entries become `todo` issues `##a22`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@roadmap-orders-issues`
- **Key words:** "the roadmap holds only the order"

### Two registers would each hold known undesigned work, which needs a routing rule, hides roadmap entries from `cargo klarch issues`, and fails one-place-for-open `##a23`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@roadmap-register`
- **Key words:** "two registers can each hold \"known, undesigned work\", which needs a routing rule"

### Keeping no roadmap leaves thaum's exception as a permanent divergence from the installed skill `##a24`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@no-roadmap`
- **Key words:** "a permanent divergence from the installed skill"

### Items defined like design entries, as headings ending with a slug, are read by the existing heading-entry code `##a25`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-register`
- **Key words:** "So the existing heading-entry code reads it."

### It fits `standing-argument-in-head`, which already names the spec as the home of the deliberation, and moves owner-rulings-recorded to met `##a26`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-records-the-exchange`
- **Key words:** "already names the spec as the home of the deliberation"

### It strains existing-plans-readable, but only thaum's 3 plan documents exist, and `structure-ready` names this as a reason to reopen it `##a27`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-records-the-exchange`
- **Key words:** "the only plan documents that exist are thaum's 3"

### The design skill refuses to track arguments because state needs a many-to-many relation; an identifier without state does not hit that reason `##a28`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@arguments-as-items`
- **Key words:** "An identifier without state does not hit that reason."

### Named arguments let rulings, premortem causes and closures cite what decided them, and harvest becomes a selection rather than a rewrite `##a29`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@arguments-as-items`
- **Key words:** "the head's standing argument becomes a selection of named arguments rather than a rewrite"

### Cost: the agent mints argument IDs every round and the spec gets longer (writing-cost) `##a30`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@arguments-as-items`
- **Key words:** "the agent mints argument IDs every round, and the spec gets longer"

### A file written from round 1 meets discussion-survives-compaction; a summary loses the losing arguments and closure conditions `##a31`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "the summary loses exactly what step 8 warns about"

### Cost: a write per round, and a change to `design-hands-off-to-planning` `##a32`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "a write per round"

### The issue itself states the cause: no declared register holds a decision before it is built; `thread@structured-plans@plan-register` closes that gap `##a33`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-built-intent`
- **Key words:** "no declared register can hold a decision before it is built."

### Unbuilt intent at the level of goals already has a home, `an-unmet-goal-is-intent` `##a34`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-built-intent`
- **Key words:** "Unbuilt intent at the level of goals already has a home"

### Without a per-entry status marker, a reader cannot tell built from unbuilt `##a35`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-intent`
- **Key words:** "a reader cannot tell built from unbuilt without a status marker"

### "Check the code against it" would report false defects on every unbuilt entry `##a36`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-intent`
- **Key words:** "reports false defects on every unbuilt entry"

### A milestone's later steps would sit in the design homes on main for several PRs before any code exists `##a37`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-intent`
- **Key words:** "for several PRs before any code exists"

### A decision reversed during implementation would be edited twice `##a38`

- **Round:** 1
- **Who:** agent
- **Threads:** `thread@structured-plans@design-home-is-intent`
- **Key words:** "A decision reversed during implementation is edited twice."

### Most work spans several Components, and there should never be so many planned tasks open at once that a split makes sense `##a39`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@plans-at-root`, `thread@structured-plans@plans-per-anchor`
- **Key words:** "there should never be that many planned tasks open at once that splitting the directory would make sense"

### Plans tend to break each other when one is designed before the other is implemented `##a40`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@plans-at-root` (later reused by the agent for `thread@structured-plans@cross-plan-references`)
- **Key words:** "plans have a tendency to break each other, when they are designed before the other is implemented"

### The shape builds the roadmap from several register shapes using only references `##a41`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@roadmap-orders-issues`
- **Key words:** "It brings together multiple register shapes to form the roadmap using only references"

### The owner wants plan documents cited by kind: milestone@knowledge-architect@structured-plans, and spec@...@... `##a42`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@plan-document-kinds`
- **Key words:** "I'd like to be able to reference path@knowledge-architect@docs/plans/structured-plans/ as milestone@knowledge-architect@structured-plans"

### Milestones are also created through the design skill, so the record applies to them; the README is the "milestone spec" `##a43`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@spec-records-the-exchange`
- **Key words:** "Milestones are created through the design skill too after all."

### The recorded decision against a written delta (agent overhead hurts the discussion) predates model progress; the owner observed Opus 5.5 carry design discussions interleaved with implementation without significant problem, and confirmed it failed in the Opus 5 time `##a44`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "I confirmed this was not working in the Opus 5 time. So the progress is real."

### Once plans are structured registers, design homes carry built intent and plans carry unbuilt intent `##a45`

- **Round:** 2
- **Who:** owner
- **Threads:** `thread@structured-plans@design-home-is-built-intent`
- **Key words:** "design docs can carry the meaning of \"built intent\", while plans carry \"unbuilt intent\""

### `path@agent-skills@docs/rejected-alternatives.md` already records this proposal as lost to `design-hands-off-to-planning`, with three reasons; reopening must answer them and needs the owner's explicit word `##a46`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "by the record test, my proposal was not new"

### The owner's capability observation answers the "forgets to update" reason, but it is an assumption about the model, so a tripwire candidate `##a47`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "It is also an assumption about the model"

### With `thread@structured-plans@plan-register` the checker corroborates the file's shape, which answers the "no corroborant" reason only partly: a wrong but well-formed state passes `##a48`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "A state that is wrong but well-formed still passes the checker"

### The file's job has changed: it holds every argument and verbatim ruling, which is what compaction removes first and a correct state ledger does not recover `##a49`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-written-during-discussion`
- **Key words:** "That is the content compaction removes first"

### Planning §4 says the milestone document holds the same sections for the whole milestone, so the README is the milestone's spec `##a50`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@spec-records-the-exchange`
- **Key words:** "The milestone document holds the same sections for the whole milestone"

### The owner's form needs one register with two kinds, taken from the entry's shape, which is new in the core; two registers sharing one home has no core rule `##a51`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-document-kinds`
- **Key words:** "One register, two kinds."

### A step spec's basename is not unique across the plans directory (two milestones can each have `harvest`) `##a52`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-document-kinds`
- **Key words:** "two milestones can each have a step called `harvest`"

### Refusing `path` citations of plan documents gives one name per document; keeping both makes `show spec@...` miss path citations `##a53`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-document-kinds`
- **Key words:** "`show spec@…` would then miss the citations written as paths"

### Citing a whole plan document stays legal everywhere; the roadmap needs it, and its dangling at departure is the wanted revisit `##a54`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-document-kinds`
- **Key words:** "When the plan leaves, the citation dangles, which is the revisit we want."

### `thread@structured-plans@arguments-as-items` multiplies internal references, so reference length matters: (a) 66 characters, (a') 45, (b) 28 `##a55`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-item-scope`
- **Key words:** "So the length of one reference matters."

### In (a) the root anchor carries no information since `thread@structured-plans@plans-at-root`; at 100 columns it fits about one reference per line `##a56`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-item-scope`
- **Key words:** "The root anchor in every reference carries no information"

### (a') keeps the three-part grammar and mirrors the core's "an anchor carries registers"; costs: a file anchor, a plan named like a Component is a finding, an undeclared anchor `##a57`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-item-scope`, `thread@structured-plans@plan-anchor`
- **Key words:** "It mirrors what the core already says: an anchor carries registers."

### (b) cannot name another plan's item, and contradicts the core rule refusing a reference with no anchor, so needs a scoped exception `##a58`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-item-scope`
- **Key words:** "A reference that names no anchor is refused, including inside the file that defines the id."

### The owner's `argument@structured-plans@a40` argues for a plans-directory bound: in the A/B scenario a single-plan bound leaves the dependency in prose, and the check never fires on rebase `##a59`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@cross-plan-references`
- **Key words:** "If the bound is the single plan, B cannot write that citation."

### Cost: one plan's landing breaks another's check, but on its branch at rebase, the session that must re-check anyway `##a60`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@cross-plan-references`
- **Key words:** "which is the session that has to do the re-checking"

### Arguments never become design entries, so they get numbers rather than content slugs, which would cost a name for dozens of statements `##a61`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@argument-ids`
- **Key words:** "they cost a name for each of what will be dozens of statements"

### One sequence across a milestone's README and step specs avoids collisions inside the milestone anchor; each argument is defined once because it bears on several threads `##a62`

- **Round:** 2
- **Who:** agent
- **Threads:** `thread@structured-plans@argument-ids`
- **Key words:** "one argument bears on several threads"

### thaum's 3 plan documents would need rewriting; a cost to rule on, or the criterion drops with `structure-ready`. Retracted in R6 (`argument@structured-plans@a96`) `##a63`

- **Round:** 2
- **Who:** agent
- **Threads:** (criterion existing-plans-readable; `structure-ready`)
- **Key words:** "3 plan documents in thaum need rewriting into the new shape"

### A proper spec cannot be written during the discussion, since threads are not yet approved and no full design can be assembled `##a64`

- **Round:** 3
- **Who:** owner
- **Threads:** `thread@structured-plans@spec-written-during-discussion`, `thread@structured-plans@draft-ledger`
- **Key words:** "It would not be possible to write it like this anyway, because threads are not yet approved, so no full design can be assembled."

### Distinguish a low-effort draft ledger, reflecting only the discussion's state, from the spec that planning assembles from it `##a65`

- **Round:** 3
- **Who:** owner
- **Threads:** `thread@structured-plans@draft-ledger`
- **Key words:** "The initial ledger would only be a draft, written with the lowest amount of effort, whose only goal is to reflect the state of the discussion."

### Avoid building a new kind or shape of register for this; split into docs/plans/milestones/ and docs/plans/specs/ `##a66`

- **Round:** 3
- **Who:** owner
- **Threads:** `thread@structured-plans@plan-document-kinds`, `thread@structured-plans@plans-split-dirs`
- **Key words:** "I'm not too keen on building a new kind/shape of registers just for that."

### Plan documents follow reference constraints that differ from other registers, so a fully different model is acceptable `##a67`

- **Round:** 3
- **Who:** owner
- **Threads:** `thread@structured-plans@plan-document-kinds`, `thread@structured-plans@plan-items-by-section`
- **Key words:** "So a fully different model is acceptable for that."

### Keep the step file's basename (unique, prefixed, or cited `spec@<milestone-name>@harvest`); carrying the project name is odd with a single source `##a68`

- **Round:** 3
- **Who:** owner
- **Threads:** `thread@structured-plans@plan-document-kinds`, `thread@structured-plans@plans-location`
- **Key words:** "It seems kinda weird to carry the project name, when there is a single source."

### Numbering avoids naming each argument, which is tedious and of little use since arguments are not harvested `##a69`

- **Round:** 3
- **Who:** owner
- **Threads:** `thread@structured-plans@argument-ids`
- **Key words:** "not very useful since they do not get harvested in registers"

### The draft ledger matches the recorded alternative; low effort answers only the first reason, while "stale stated with confidence" and "only corroborant is the conversation" still apply `##a70`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@draft-ledger`
- **Key words:** "Approving it would reopen the alternative you just declined to reopen."

### The per-round draft already exists: the design skill writes a delta table every round that changes something `##a71`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`
- **Key words:** "the per-round draft is already written"

### Measured: session `46dca865` was compacted at least 3 times; its first summary is record 1290 of 7177, and the 179 user and 353 assistant records before it are still in the file `##a72`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`
- **Key words:** "the transcript on disk keeps every delta, every argument and every ruling verbatim, after any number of compactions"

### The deltas are corroborated as written, since the owner read and answered each `##a73`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`
- **Key words:** "the deltas are what you read and answered in each round"

### Argument numbers are assigned once at assembly, never minted during the discussion `##a74`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`, `thread@structured-plans@argument-ids`
- **Key words:** "the discussion never mints argument numbers"

### Cost: the transcript path is harness-specific; the rule falls back to the conversation, as the transcript reviewer's "whose transcript is available" already does `##a75`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`
- **Key words:** "The transcript path is specific to the harness."

### Cost: this session's transcript is about 727 KB, so extraction is a subagent's job `##a76`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`
- **Key words:** "This session's transcript is about 727 KB."

### Cost: the extraction can be wrong; the transcript reviewer checks it against the same file `##a77`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@ledger-from-transcript`
- **Key words:** "The extraction can be wrong."

### The core's `Shape` has `Heading` and `File`; a file register already checks a level-one title and declared level-two sections, which is planning's fixed section list `##a78`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-split-dirs`, `thread@structured-plans@plan-document-kinds`
- **Key words:** "checked by code that exists"

### A milestone directory is a file register home (README, generated index, one file per entry), so `spec@<milestone>@harvest` is an ordinary reference, unique only inside its milestone `##a79`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-split-dirs`, `thread@structured-plans@plan-anchor`
- **Key words:** "a step's basename only needs to be unique inside its milestone"

### A reserved anchor `plans` fixed by the tool, like `*` and `elsewhere`, removes the project name from citations `##a80`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@plans-location`
- **Key words:** "so the project name goes away"

### `milestone@plans@<id>` needs a directory entry, the one new piece of register shape; the free alternative is citing the README by path, kept out because the owner asked for milestone@ `##a81`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-document-kinds`
- **Key words:** "You asked for milestone@, so I keep it, and name it as the cost."

### Today a heading register has one home and one level and nothing scopes a citation, so items need their own model: section gives kind, scope is the plan anchor; a spec file as anchor is also new `##a82`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-items-by-section`
- **Key words:** "Items need a model of their own"

### Extensions run in phase 4 after the entity table is built, so items cannot be an extension; they are core work `##a83`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@plan-items-by-section`
- **Key words:** "So the items cannot be added as an extension."

### Under the owner's bound, B cites A whole and is revisited when A leaves; the only missed event (A reverses x while open) needs two concurrent dependent plans, which the owner said is rare, so the agent withdraws `##a84`

- **Round:** 3
- **Who:** agent
- **Threads:** `thread@structured-plans@cross-plan-references`
- **Key words:** "whole-plan citations already give the dependency"

### Item citations would force an in-depth revisit of B when A is removed, at a bad moment; whole-document citations prevent deep interdependence, and the retiring session only removes the citations and opens an issue `##a85`

- **Round:** 4
- **Who:** owner
- **Threads:** `thread@structured-plans@cross-plan-references`, `thread@structured-plans@retiring-plan-opens-issue`
- **Key words:** "which forces design work at a moment that might not be the best"

### By the knowledge table, "may need revisiting" is something outstanding, so an issue is the right home `##a86`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@retiring-plan-opens-issue`
- **Key words:** "\"may need revisiting\" is something outstanding"

### The issue cites the citing plan, so if that plan leaves first, the citation dangles and the check fails: the issue cannot outlive its plan `##a87`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@retiring-plan-opens-issue`
- **Key words:** "So the issue cannot outlive the plan it is about."

### For a milestone B, the issue tells the next step's design audit (planning §7.2) what changed `##a88`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@retiring-plan-opens-issue`
- **Key words:** "The issue tells that audit what changed."

### The issue's `Why it matters` cites the leaving plan's harvested design entries, since the plan no longer exists after that commit; its path goes in the commit message `##a89`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@retiring-plan-opens-issue`
- **Key words:** "cites the leaving plan's harvested design entries, not the leaving plan"

### The owner's set-in-stone argument (`argument@structured-plans@a1`) applies: docs/roadmap.md at the root, fixed name, optional, no new checker code `##a90`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@roadmap-home`
- **Key words:** "the same argument applies here"

### The rival, `path@plans@README.md`, breaks "the plans directory holds plan documents and nothing else", because a roadmap outlives every plan it lists `##a91`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@roadmap-home`
- **Key words:** "a roadmap outlives every plan it lists"

### Where one argument ends in prose is a judgement the extracting subagent would make; default: consolidate at assembly, checked by the transcript reviewer `##a92`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@argument-segmentation`
- **Key words:** "where one argument ends and the next begins is a judgement"

### The rival, marking arguments in each round, makes extraction exact at a per-round cost, which the recorded alternative warned about `##a93`

- **Round:** 4
- **Who:** agent
- **Threads:** `thread@structured-plans@argument-segmentation`
- **Key words:** "The cost is per round"

### The agent corrects R2: existing-plans-readable is binding as a presumption and blocks convergence; since the plan shape changes, `structure-ready` is reversed at harvest and the criterion leaves with it `##a94`

- **Round:** 5
- **Who:** agent
- **Threads:** (criterion existing-plans-readable; `structure-ready`)
- **Key words:** "In round 2 I called it \"weighed\", and that was wrong."

### `thread@structured-plans@spec-records-the-exchange` and `thread@structured-plans@arguments-as-items` make a spec longer, so writing-cost is unmet unless accepted by name `##a95`

- **Round:** 5
- **Who:** agent
- **Threads:** (criterion writing-cost)
- **Key words:** "make a spec longer"

### thaum's plan documents need no rewrite until a new release and a version bump on thaum's side; thaum already migrated to v0.1 `##a96`

- **Round:** 6
- **Who:** owner
- **Threads:** (thaum migration; existing-plans-readable)
- **Key words:** "This will only happen when I make a new release of knowledge-architect, then bump the version on thaum's side."

### The agent concedes: `exact-pin` and `goal@knowledge-architect@any-project-can-adopt-it` already say a project moves when it chooses, so this costs thaum nothing until its pin moves `##a97`

- **Round:** 6
- **Who:** agent
- **Threads:** (thaum migration)
- **Key words:** "I had the fact and did not apply it."

### The thaum observation (`path@elsewhere@docs/next-milestones.md` and its roadmap exception) stays valid evidence; only the migration cost was wrong `##a98`

- **Round:** 6
- **Who:** agent
- **Threads:** `thread@structured-plans@roadmap-orders-issues`
- **Key words:** "Only the migration cost was wrong."

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

**The plans anchor** (`thread@structured-plans@plans-location`, `thread@structured-plans@plans-dir-fixed`, `thread@structured-plans@plans-at-root`). The tool constructs one
anchor, `plans`, at the plans directory of the root Component. Its name is reserved like `*` and
`elsewhere`. It is built as a location the tool constructs (default D6), carrying the two
registers below as built-in registers that no other anchor carries and no project may declare
(default D1). A Component already gets its storage at fixed paths, and a built-in register refuses
a `dir` key, so a declared plans directory would be the only declared path in that family (`argument@structured-plans@a9`).
One plans directory, because most work spans Components (`argument@structured-plans@a15`, which the owner endorsed in round 2),
because there should never be so many plans open at once that a split makes sense (`argument@structured-plans@a39`, the
owner's), and because plans designed before another is built break each other (`argument@structured-plans@a40`, the owner's).
The reference grammar names the anchor, so a later split adds citations and rewrites none (`argument@structured-plans@a13`).
Nearest rival: `thread@structured-plans@plans-per-anchor`. The agent added a cost against it, an empty home in every
Component (`argument@structured-plans@a14`); the owner did not argue that point.

**Two registers under it** (`thread@structured-plans@plans-split-dirs`, `thread@structured-plans@plan-document-kinds`). `spec`: home
docs/plans/specs/, the File shape, an entry `<id>.md`, cited `spec@plans@<id>`. `milestone`:
home docs/plans/milestones/, an entry a directory `<id>/` holding a `README.md`, cited
`milestone@plans@<id>`. The directory entry is the one new register shape, kept because the
owner asked for the `milestone` kind (`argument@structured-plans@a42`, `argument@structured-plans@a81`); its rules are default D10. The specs under
`specs/` and the step specs of a milestone are one kind, `spec`, in different anchors: the owner
asked that specs be cited spec@...@... (round 2), proposed `spec@<milestone-name>@harvest` for a
step (round 3), and approved the shape (round 4). Nearest rival: one register with two kinds taken
from the entry's shape (`argument@structured-plans@a51`), defeated by the owner's "I'm not too keen on building a new
kind/shape of registers just for that." (`argument@structured-plans@a66`).

**A milestone directory is an anchor** (`thread@structured-plans@plan-anchor`, `thread@structured-plans@layout-kept`). Named by its basename, implied
by the tree (default D11), carrying a `spec` register whose home is the directory itself (default
D9): its `README.md` is the milestone document, its `index.md` is generated and lists the step
specs, every other `.md` is a step spec, cited `spec@<milestone>@<step>`. The File shape already
has a hand-written README, a generated index and one file per entry (`argument@structured-plans@a79`). A step's basename need
only be unique in its milestone (`argument@structured-plans@a52`, `argument@structured-plans@a68`).

**A spec file is an anchor too**, for its items only (`thread@structured-plans@plan-anchor`, `thread@structured-plans@plan-items-by-section`). This is
new: today every anchor is a directory (`argument@structured-plans@a82`). It is built in step 2. Nearest rival to plan anchors:
shape (a) of `thread@structured-plans@plan-item-scope`, which keeps the root anchor in every citation (`argument@structured-plans@a55`, `argument@structured-plans@a56`).

**Clause P1.** A plan anchor carries no `path` kind. A `path` citation of a plan document is
refused, and the repair names the `spec` or `milestone` form. One name per document keeps
`cargo klarch show` complete (`argument@structured-plans@a53`). The clause as approved refuses "a `path` citation of a file
under docs/plans/specs/ or docs/plans/milestones/" and lets `plans` carry `path` "for its own
README files"; the READMEs and indexes of the two homes satisfy both halves, so the reading is
default D8: a plan document is a spec file, a milestone directory, or a file inside a milestone
directory, and the README and index files of the plans directory and of its two homes are cited
`path@plans@<file>`. The plans directory holds nothing else (default D17).

**Clause P2.** A plan name that is also the name of a Component, a location (default D15) or a
reserved anchor is a finding, and so is one name under both `specs/` and `milestones/`. It applies
in step 1 to milestone
names and spec basenames alike (default D13), and it is a phase-2 finding (default D11).

**Items** (`thread@structured-plans@plan-items-by-section`, `thread@structured-plans@arguments-as-items`). An item is a heading
`### <statement> ##<id>` under one of four level-two sections, and the section gives its kind:
Threads → `thread`, Arguments → `argument`, Criteria → `criterion`, Acceptance criteria →
`acceptance`. Items are core work in the entity table, because an extension runs after the table is
built (`argument@structured-plans@a83`). Items need a model of their own, since a heading register has one home and one level
and nothing scopes a citation (`argument@structured-plans@a82`, which replaces `argument@structured-plans@a25`). An argument has an identifier and no
state: the design skill keeps arguments stateless because state would need a many-to-many
relation, and an identifier does not (`argument@structured-plans@a28`). Named arguments let a ruling or a closure cite what
decided it, and turn the harvest of a standing argument into a selection (`argument@structured-plans@a29`).

**The structure the checker reads** (`thread@structured-plans@plan-register`). Plan documents become a structure of the
checker: the plans anchor and its registers (step 1) and the items (step 2). The owner approved the
shape in round 2; its parts are the paragraphs above and below. Its rival, the owner's ruling that
plan documents stay free prose, was the issue's other closing condition and was not argued.

**Item citations** (`thread@structured-plans@plan-item-scope`). Shape (a'): `<kind>@<plan>@<id>`, the plan anchor in the
anchor position, as in `thread@structured-plans@roadmap-orders-issues`. An item is cited only from
inside its own plan anchor: the spec file, or the milestone directory. A commit message is outside
every plan anchor, so it cites a plan document whole, never an item (default D12). (a') keeps the
three-part grammar and the id grammar (`argument@structured-plans@a57`), at about 45 characters where shape (a) needs 66 (`argument@structured-plans@a55`,
`argument@structured-plans@a56`). Nearest rival: shape (b), defeated by the core rule that refuses a reference with no anchor
(`argument@structured-plans@a58`).

**Argument identifiers** (`thread@structured-plans@argument-ids`, `thread@structured-plans@argument-segmentation`). `a1`, `a2`, …, in order of
appearance, never reused, one sequence per milestone across its README and its step specs (`argument@structured-plans@a62`).
Arguments are never harvested as entries, so a content slug would cost a name for nothing (`argument@structured-plans@a61`,
`argument@structured-plans@a69`). The boundaries are decided at assembly and checked by the transcript reviewer (`argument@structured-plans@a92`). Nearest
rival: arguments marked in each round, defeated by its per-round cost (`argument@structured-plans@a93`).

**The full record** (`thread@structured-plans@spec-records-the-exchange`). A plan document records every thread with its
proposer, final state, the arguments on each side, the owner's rulings verbatim with their round,
and its relations. It applies to a spec and to a milestone document (`argument@structured-plans@a43`, `argument@structured-plans@a50`). It applies also
to a step's spec when the step had a design session of its own (default D16). It fits
`design@agent-skills@standing-argument-in-head`, which already names the spec as the home of the
deliberation (`argument@structured-plans@a26`). Its rival is the plan document as the planning skill shapes it today, which
records each thread's final state and resolution but neither the rulings verbatim nor the arguments
as items; it leaves `criterion@structured-plans@owner-rulings-recorded` partly met.

**Assembly from the transcript** (`thread@structured-plans@ledger-from-transcript`, clause P3). The planning skill
dispatches a subagent that extracts, from every transcript the discussion spans, the delta tables,
the owner's rulings verbatim and the arguments, and assembles the plan document from them. Where the
harness keeps no transcript, it assembles from the conversation. The design skill's per-round delta
is the draft; the transcript on disk keeps the records from before a compaction (`argument@structured-plans@a71`, `argument@structured-plans@a72`, with the
correction noted under the arguments). Nearest rival: `thread@structured-plans@draft-ledger`, defeated because two of the
three reasons of the recorded lost alternative still apply to it (`argument@structured-plans@a70`).

**The roadmap** (`thread@structured-plans@roadmap-orders-issues`, `thread@structured-plans@roadmap-home`). docs/roadmap.md in the root Component,
optional. Its rows are references to issue entries and to plan documents, in order, with an
unordered section allowed. A row dangles when its issue closes or its plan leaves, and the check
forces the edit (`argument@structured-plans@a21`). The work stays in the issue register; the roadmap holds only order (`argument@structured-plans@a22`).
An illustration of the shape:

```markdown
# Roadmap

## In order
1. `milestone@plans@<id>`       designed
2. `issue@<anchor>@<id>`        not designed

## Unordered
- `issue@<anchor>@<id>`
```

Nearest rival: `thread@structured-plans@roadmap-register`, defeated because two registers would hold undesigned work (`argument@structured-plans@a23`).

**A leaving plan** (`thread@structured-plans@retiring-plan-opens-issue`, clause P8). When a plan leaves and another plan
cites it as a document, the session that meets the dangling citation, whether it retires the plan
or rebases the citing plan, removes the citation and opens an issue on the citing plan. The issue
says that the citing plan may need revisiting now that the leaving plan is built, "to account for
deviations or other unplanned happenings" (the owner, round 4). It cites the citing plan, so it
cannot outlive it (`argument@structured-plans@a87`). Its `Why it matters` cites the leaving plan's harvested design entries
(`argument@structured-plans@a89`). An item citation across plans is not available: `thread@structured-plans@cross-plan-references` was withdrawn.

**Built intent** (`thread@structured-plans@design-home-is-built-intent`, clause P9). A design home holds the design as built
and its reasons, and the code is checked against it. A plan document holds decided, unbuilt design.
Harvest stays at landing. Clause P9, as approved: the new primer wording covers decisions recorded
when made, that is, the decisions that no work implements, which
`design@agent-skills@harvest-after-implementation` already records when they are made. The
primer's sentence "A design home is intent" is reworded, and with it the root `CLAUDE.md`'s
"Intent" bullet, §0 of `knowledge-architect-decision-recording`,
`design@agent-skills@harvest-after-implementation` and `design@agent-skills@primer-content`; the
owner approved the four in round 3. Nearest rival: `thread@structured-plans@design-home-is-intent`, defeated by the false
defects it reports on unbuilt entries (`argument@structured-plans@a36`).

**The name** (`thread@structured-plans@plans-name-kept`). `plans`: "planned" means intended or scheduled in common English,
which is the roadmap's content (`argument@structured-plans@a16`).

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
| `thread@structured-plans@plans-dir-declared` | `thread@structured-plans@plans-dir-fixed` | a built-in register's storage is fixed by the tool (`argument@structured-plans@a9`) |
| `thread@structured-plans@plans-per-anchor` | `thread@structured-plans@plans-at-root` | most work spans Components (`argument@structured-plans@a15`); a later split rewrites no citation (`argument@structured-plans@a13`) |
| `thread@structured-plans@planned-name` | `thread@structured-plans@plans-name-kept` | "planned" names scheduled work, the roadmap's content (`argument@structured-plans@a16`) |
| `thread@structured-plans@roadmap-register` | `thread@structured-plans@roadmap-orders-issues` | two registers of undesigned work (`argument@structured-plans@a23`) |
| `thread@structured-plans@no-roadmap` | `thread@structured-plans@roadmap-orders-issues` | thaum's exception (`argument@structured-plans@a20`) |
| `thread@structured-plans@design-home-is-intent` | `thread@structured-plans@design-home-is-built-intent` | false defects on unbuilt entries (`argument@structured-plans@a36`) |
| `thread@structured-plans@spec-written-during-discussion` | `thread@structured-plans@ledger-from-transcript` | no spec can be assembled while threads are open (`argument@structured-plans@a64`) |
| `thread@structured-plans@draft-ledger` | `thread@structured-plans@ledger-from-transcript` | two recorded reasons still apply (`argument@structured-plans@a70`) |
| `thread@structured-plans@cross-plan-references` | `thread@structured-plans@plan-item-scope` | whole-document citations carry the dependency (`argument@structured-plans@a84`), and item citations force design work at a plan's retirement (`argument@structured-plans@a85`) |
| shape (a) of `thread@structured-plans@plan-item-scope` | shape (a') | 66 characters against 45, and a root anchor that carries no information (`argument@structured-plans@a55`, `argument@structured-plans@a56`) |
| shape (b) of `thread@structured-plans@plan-item-scope` | shape (a') | the core refuses a reference with no anchor (`argument@structured-plans@a58`) |
| one register with two kinds by entry shape | `thread@structured-plans@plans-split-dirs` | the owner's wish not to build a new register model (`argument@structured-plans@a66`) |
| arguments marked in each round | `thread@structured-plans@argument-segmentation` at assembly | the per-round cost (`argument@structured-plans@a93`) |
| content-named argument slugs | `thread@structured-plans@argument-ids` | a name for each of dozens of statements that are never harvested (`argument@structured-plans@a61`, `argument@structured-plans@a69`) |
| two registers sharing one home directory | `thread@structured-plans@plans-split-dirs` | the core has no rule for two registers at one directory (round 2); the owner's split gives each its own |
| keeping `path` citations of plan documents legal beside the kind form | `thread@structured-plans@plan-document-kinds` | `cargo klarch show spec@…` would miss the citations written as paths (`argument@structured-plans@a53`) |
| no `milestone` kind, a milestone cited by a path to its README | `thread@structured-plans@plan-document-kinds` | the owner asked for milestone@ (`argument@structured-plans@a42`); its cost, the directory entry, is named (`argument@structured-plans@a81`) |
| unique step basenames across the plans directory, or prefixed with the milestone's name | `thread@structured-plans@plan-document-kinds` | the owner's own option (`argument@structured-plans@a68`), set aside by the shape the owner approved, `spec@<milestone>@<step>`, where a basename is unique inside its milestone only (`argument@structured-plans@a79`) |
| the roadmap in the plans directory's README | `thread@structured-plans@roadmap-home` | a roadmap outlives every plan it lists, and the plans directory holds plan documents only (`argument@structured-plans@a91`) |

## Readings

Empty: the work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| P1: a `path` reference inside a milestone directory is ambiguous under deepest-anchor-wins | `thread@structured-plans@plan-anchor`, `thread@structured-plans@plan-document-kinds` | converted into clause P1 |
| P2: a spec and a milestone share a name, or a plan is named like a Component or a reserved anchor | `thread@structured-plans@plans-location`, `thread@structured-plans@plan-anchor` | converted into clause P2; claimed by a core test in step 1 |
| P3: the discussion spans several sessions and the assembly reads one transcript | `thread@structured-plans@ledger-from-transcript` | converted into clause P3 |
| P4: the harness changes its transcript format, or drops records from before a compaction | `thread@structured-plans@ledger-from-transcript` | tripwire T1, declined by the owner in round 6 |
| P5: the extracted record misses or misstates a ruling, and the error survives review | `thread@structured-plans@ledger-from-transcript`, `thread@structured-plans@argument-segmentation` | tripwire T2, recorded |
| P6: plan documents grow too long for the owner to read before commit | `thread@structured-plans@spec-records-the-exchange`, `thread@structured-plans@arguments-as-items` | tripwire T3, declined by the owner in round 6 |
| P7: a plan cannot cite another plan's item, so it describes one in words | `thread@structured-plans@plan-item-scope` | tripwire T4, recorded |
| P8: the citing plan is on another branch, so the retiring session cannot see it | `thread@structured-plans@retiring-plan-opens-issue` | converted into clause P8 |
| P9: a decision with no implementing work is excluded by the "built" wording | `thread@structured-plans@design-home-is-built-intent` | converted into clause P9 |
| P10: the roadmap's order goes stale | `thread@structured-plans@roadmap-orders-issues` | survives: a closed issue's row dangles and the check forces the edit |
| P11: a project needs several plans directories | `thread@structured-plans@plans-at-root` | tripwire T5, recorded |

The owner's ruling, round 6: "Keep tripwires T2, T4, T5. Not the others." Each is written at the
harvest of the decision it guards:

- **T2**, guarding `thread@structured-plans@ledger-from-transcript`, in the agent-skills tripwires home at step 3: a ruling
  of the owner missing from, or misstated in, a committed plan document, found after the commit.
- **T4**, guarding `thread@structured-plans@plan-item-scope`, in the core tripwires home at step 2: a plan document's prose
  names an item of another plan document.
- **T5**, guarding `thread@structured-plans@plans-at-root`: written at step 1 as
  `tripwire@core@plans-directory-split-asked`.

## Acceptance criteria

A fourth criterion, #assembly-holds-the-rulings, was judged at the commit that added this document
and fired: the transcript reviewers found rulings recorded wider than the owner made them. The
repair commit after the review before the merge reports it, and it left this table. Its evidence
goes to T2 at step 3's harvest.

Each criterion names the thread it guards rather than a reference: the decision has no design entry
until its harvest.

`acceptance@structured-plans@milestone-fits-file-register` was judged at step 1's landing and did not fire; the landing
commit reports the evidence. It is reported once more when this document leaves.

### The plans anchor and the milestone anchors fit the defaults D6, D9, D10 and D11 `##milestone-fits-file-register`

- **Guards:** `thread@structured-plans@plans-split-dirs`, `thread@structured-plans@plan-anchor`
- **Judged at:** step 1
- **Fires when:** the plans anchor and the milestone anchors need more than defaults D6, D9, D10 and D11 state: a new kind of anchor rather than a location the tool constructs, a `Shape` variant beyond the directory entry of D10, or a change to the File shape's rules for the `spec` register
- **Response:** stop step 1; put the shape back to the owner

### A spec file fits the path model as an anchor `##file-anchor-fits-path-model`

- **Guards:** `thread@structured-plans@plan-anchor`
- **Judged at:** step 2
- **Fires when:** making a spec file an anchor needs a change to `Anchors::owning` or to the deepest-anchor rule beyond excluding plan anchors from the `path` kind
- **Response:** stop step 2; design session with the owner

### The roadmap needs no checker rule `##roadmap-needs-no-code`

- **Guards:** `thread@structured-plans@roadmap-home`
- **Judged at:** step 3
- **Fires when:** writing the roadmap's instructions needs a checker rule that ordinary reference checking does not give
- **Response:** stop step 3; put it to the owner

## Implementation sequence

1. #plans-structure, landed: the plans anchor, the `spec` and `milestone` registers,
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

None. The owner ruled every default below, and D19, in the spec of step 2.

**A material finding on `thread@structured-plans@ledger-from-transcript`, presented to the owner.** Argument `argument@structured-plans@a72` stated three
compactions; the re-measurement found one (see the notes under "Arguments"). The premise the
closure rests on, that the transcript keeps the records from before a compaction, holds on that one
compaction; "after any number of compactions" is not measured. Tripwire T1, which watched this
premise, was declined in round 6. Default: `thread@structured-plans@ledger-from-transcript` stands as approved, and T1 stays
declined. The finding was presented to the owner with this default, and the owner proceeded.

**Ruled.** The owner ruled D1 to D4 and D6 to D14 ("D1 to D14 approved, proceed"), then D15 to
D18 ("Agreed on those new defaults. proceed"). D5 was decided earlier and is in "Decided design"
under "Two registers under it". D18 revises D14.

| ruled default | thread it bears on |
| --- | --- |
| D15: clause P2 also makes a plan name that is the name of a location a finding. The clause as approved names a Component and a reserved anchor; a location is an anchor whose name a reference carries in the same position, so the same ambiguity follows | `thread@structured-plans@plans-location` |
| D16: `thread@structured-plans@spec-records-the-exchange` applies also to a step's spec when the step had a design session of its own, following §4 of the planning skill. The owner ruled on specs and milestone documents | `thread@structured-plans@spec-records-the-exchange` |
| D17: the plans directory holds nothing but its README and its two homes: any other file or directory directly under docs/plans/ is a phase-2 finding. Without it, nothing enforces that plan documents sit in the homes | `thread@structured-plans@plans-split-dirs` |
| D18: D14 revised. This document lands alone, in a pull request of its own merged under the current checker, rather than with step 1: under D14, step 1's checker would refuse the trees of this branch's earlier commits, which lack the plans homes, and the root `CLAUDE.md` requires every commit of a branch to pass under its tip's checker. Step 1's own commits that its checker refuses are squashed before review | `thread@structured-plans@layout-kept` |
| D1: the plans homes are required in the root Component: the `plans` anchor owes docs/plans/README.md, and `specs/` and `milestones/` each owe a README and a generated index. `spec` and `milestone` are built-in registers carried by `plans` alone; a project's `[registers.spec]` or `[registers.milestone]`, and a location naming either, are refused as any declaration of a built-in's storage is | `thread@structured-plans@plans-dir-fixed` |
| D2: enacted in the commit that adds this document, which the owner may reverse: that commit closes the core issue structured-plan-documents, per §2 of the planning skill, and repoints its four citations to this document | `thread@structured-plans@plan-register` |
| D3: this repository writes no docs/roadmap.md in this milestone | `thread@structured-plans@roadmap-home` |
| D4: the section titles of a plan document are checked from step 2, with the items, not in step 1 | `thread@structured-plans@plan-items-by-section` |
| D6: the `plans` anchor is built as a location the tool constructs, carrying the two registers, rather than as a new kind of anchor | `thread@structured-plans@plans-location` |
| D7: this document's record of arguments uses plain `a<n>` identifiers in a table under Threads until step 2, which moves them into a level-two Arguments section as definitions | `thread@structured-plans@arguments-as-items` |
| D8: the reading of clause P1. A plan document is a spec file, a milestone directory, or a file inside a milestone directory; a `path` citation of one is refused. The README and index files of the plans directory and of its two homes are cited `path@plans@<file>` | `thread@structured-plans@plan-document-kinds` |
| D9: the `spec` register's home depends on the anchor: `specs/` under `plans`, and the anchor's own directory under a milestone anchor | `thread@structured-plans@plans-split-dirs` |
| D10: the milestone entry. A third `Shape` variant, the only one step 1 adds; step 2's #items-as-section-registers adds a fourth, `Section`, on the owner's word. Its id is the directory's basename, in the entity-id grammar. A subdirectory of milestones/ without a README.md is a finding; a `.md` file directly under milestones/ other than README.md and index.md is a finding; a `register.toml` there is a finding. The entry checks (level-one title; the sections from step 2) apply to `<id>/README.md`. Its index row is the id and the README's level-one title, linking `<id>/README.md`. The index generator skips files owned by a nested anchor, as `file_definitions` does; step 1's audit realised it with the Directory renderer, which reads only each milestone's README, and left the File renderer unfiltered | `thread@structured-plans@plan-document-kinds` |
| D11: plan anchors are built from the tree in phase 2, from the walk's present paths, by every caller that builds the anchors of a tree, the per-commit trees of `commits` included. `collides` does not apply to them, because the plans layout places them; clause P2 is their collision check, a phase-2 finding planted in `unsound`. The generated paths kept out of the walk include each milestone's index.md, computed from the tree | `thread@structured-plans@plan-anchor` |
| D12: a commit message cites a plan document whole, never an item; an item citation in a message is refused like any citation from outside its plan | `thread@structured-plans@plan-item-scope` |
| D13: clause P2 applies in step 1 to milestone names and spec basenames alike | `thread@structured-plans@plans-location` |
| D14: the branches and the landing, as "How a step is worked" states them | `thread@structured-plans@layout-kept` |

## Harvest

| when | what lands where |
| --- | --- |
| step 1 | agent-skills design home: the reason of `design@agent-skills@plans-directory-declared` rewritten (see "What is already decided"). Core design home: the step 1 heads listed under "What is already decided"; `thread@structured-plans@plan-register` (its first part), `thread@structured-plans@plans-location` (rewriting `design@core@reserved-anchors`), `thread@structured-plans@plans-dir-fixed`, `thread@structured-plans@plans-at-root`, `thread@structured-plans@plans-split-dirs`, `thread@structured-plans@plan-document-kinds`, `thread@structured-plans@plan-anchor` (rewriting `design@core@anchors-are-components-and-locations`), clauses P1 (rewriting `design@core@every-path-names-its-anchor`) and P2; `design@core@components-carry-the-same-documents` rewritten for the plans homes. Core tripwires home: T5. Core rejected alternatives: `thread@structured-plans@plans-per-anchor` if the recording tests pass it |
| step 2 | core design home: the step 2 heads listed under "What is already decided"; `thread@structured-plans@plan-register` (its second part), `thread@structured-plans@plan-item-scope`, `thread@structured-plans@plan-items-by-section`, the spec file as anchor. Core tripwires home: T4. Core rejected alternatives: shapes (a) and (b), `thread@structured-plans@cross-plan-references`, if the recording tests pass them |
| step 3 | agent-skills design home: `thread@structured-plans@design-home-is-built-intent` (rewriting `design@agent-skills@harvest-after-implementation` and `design@agent-skills@primer-content`), `thread@structured-plans@roadmap-orders-issues` and `thread@structured-plans@roadmap-home` (reversing `design@agent-skills@planned-work-is-an-issue`), `thread@structured-plans@spec-records-the-exchange`, `thread@structured-plans@arguments-as-items`, `thread@structured-plans@argument-ids`, `thread@structured-plans@argument-segmentation`, `thread@structured-plans@ledger-from-transcript` (rewriting `design@agent-skills@design-hands-off-to-planning`), `thread@structured-plans@retiring-plan-opens-issue`; `design@agent-skills@plans-directory-declared`, `design@agent-skills@structure-ready` and `design@agent-skills@thread-slug-is-entry-id` reversed or rewritten; `design@agent-skills@document-vocabulary` and `design@agent-skills@milestone-is-a-directory` rewritten. Agent-skills tripwires home: T2, carrying the evidence of #assembly-holds-the-rulings (fired at the commit that added this document: three rulings recorded wider than the owner made them, clause P9, an item citation from a commit message and clause P2, each written by the author and none by the extraction, each repaired in place before the commit); `tripwire@agent-skills@ledger-lost-before-hand-off` judged, per "What is already decided". Agent-skills rejected alternatives: `thread@structured-plans@roadmap-register`, `thread@structured-plans@design-home-is-intent`, `thread@structured-plans@draft-ledger`, `thread@structured-plans@planned-name`, `thread@structured-plans@spec-written-during-discussion`, and the incumbents of the reversals, `thread@structured-plans@plans-dir-declared`, `thread@structured-plans@no-roadmap` and `design@agent-skills@structure-ready`, if the recording tests pass them |
| this document | leaves with step 3, in the commit that completes its harvest, whose message cites it as `milestone@plans@structured-plans` |

## Later consequences

- **thaum** moves its pin when the owner chooses. The owner ruled in round 6: "thaum's plan
  documents won't need a rewrite today. This will only happen when I make a new release of
  knowledge-architect, then bump the version on thaum's side." What that rewrite holds is thaum's
  work, decided then.
- **If `tripwire@core@plans-directory-split-asked` fires**, its response applies.
- **If T4 fires**, `thread@structured-plans@cross-plan-references` is proposed to the owner again.
