# Analysis of 2026-10-08-knowledge-architect-synthetic-evidence

- **Received file:** 2026-10-08-knowledge-architect-synthetic-evidence.md, the project's side of
  this repository's own retrospective of the synthetic-evidence branch. It reports no numbered
  finding and two noted items.
- **Version it used:** built from this repository's source, main at v0.4.0-64-g9c1dfe5.
- **Analysed against:** main at v0.5.0-150-g0a7c4a1.
- **Standing entries that bear on it:** those of the workflow file's analysis,
  `path@knowledge-architect@docs/retrospective-reports/2026-10-08-knowledge-architect-synthetic-evidence-klarch-workflow.md`.
  None bears on K7.

## First noted item, with P2 of 2026-10-08-knowledge-architect-agent-configuration-entities, P1 of 2026-10-09-knowledge-architect-staged-check and P1 of 2026-10-10-knowledge-architect-design-record-audit: the claims of a commit message are checked after the commit, or not at all (cluster K7)

- **Says.** Four sessions in three days wrote commit messages from memory and pushed false claims:
  one claim here; at least nine message corrections in the agent-configuration-entities session; a
  wrong count three times and a mutation run that never happened in the staged-check session; five
  false claims in the design-record-audit session, a share given as "near 40%" where the passes
  measured 42%, 25% and 39% among them. Each report calls the instruction clear and its own case a
  lapse. The staged-check report proposes a fix at the point of delivery: where the commit step is
  performed, say that the claims are listed and checked before `git commit` runs. The
  design-record-audit report proposes a claim list written to a scratch file.
- **Still applies.** The instruction stands, in `instructions@verify-before-relying`, at
  `path@knowledge-architect@CLAUDE.md`:334: "**Before each commit, list every factual claim its
  message makes, and check each one.**" The text a session reads when it commits does not carry
  it:
  - `instructions@verify-mechanically`, at `path@knowledge-architect@CLAUDE.md`:444-446, gives the
    checks owed around a commit: "Run `cargo klarch check --staged` after staging and before each
    commit … and `cargo klarch commits origin/main..HEAD` after it". It names no claim check.
  - `skill@klarch-development@development-gate`, at
    `path@agent-config@skills/klarch-development/SKILL.md`:180-181, points to
    `instructions@verify-mechanically` for "the checks owed before and after each commit". The
    staged-check proposal named a line of that skill which 9180f4d, "[audit] Pass 5 of the
    agentic-workflow axis…", has since replaced by this pointer.
- **Recorded.** No issue. `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`
  is about the installed text's recommendation of `commits`, a different check.
- **Kind.** A lapse that recurs at one point of delivery: the instruction sits in a section read at
  the start of the work, and is needed at each commit, where the list of checks owed names the
  checker's commands and not this one. Four instances, four sessions, each found by a reviewer or
  by a re-read after the commit. This is the exception of
  `design@agent-skills@capabilities-not-structure` that the intake skill names: the fix moves where
  the instruction is delivered.
- **Scope.** It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`: a commit
  message is part of the record, and the root says it is checked by nobody but the session. Owned
  by the root, in this repository's own configuration. It reverses nothing.
- **Better fix.** Rivals:
  - A claim list written to a scratch file before the commit, the design-record-audit proposal. It
    adds an artifact the session must remember to write, the shape that lost in the agent-skills
    rejected alternative on "Writing the discussion's ledger to a file during the discussion", lost
    to `design@agent-skills@ledger-from-transcript`: "An artifact an agent must update every round
    is one it forgets to update".
  - No change. Four instances in three days show the current delivery does not reach the moment.
  - One sentence at the point of delivery: in `instructions@verify-mechanically`, the sentence
    that lists what each commit owes gains "and the factual claims of its message listed and
    checked, per `instructions@verify-before-relying`", before the commit runs. It is a
    restatement of one sentence with its pointer beside it, which `primer@where-knowledge-goes`
    admits. This is the fix.
- **Route.** A text edit of the root `CLAUDE.md`, under `skill@knowledge-architect-agent-configuration`.
- **Proposal.** Handle now, with the sentence above.

- **Default:** handle now.
- **Outcome:** approved.

## Second noted item: "Critical findings are repaired before the merge" defines no critical finding

- **Still applies.** No. 2bf743f, "[audit] Pass 1 of the agentic-workflow axis: 54 findings
  confirmed, 26 applied, 27 ruled by the owner, 1 issue", removed the sentence from the root
  `CLAUDE.md`; its message reads "the root's 'Critical findings are repaired' with it". A search of
  the root, the scoped `CLAUDE.md` files, the klarch skills and the installed text finds the phrase
  nowhere.

- **Default:** no change, already repaired.
- **Outcome:** approved.
