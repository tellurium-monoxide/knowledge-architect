# Analysis of 2026-10-07-thaum-mock-reduction-workflow

- **Received file:** 2026-10-07-thaum-mock-reduction-workflow.md, the workflow's side of a
  retrospective of a project that uses the workflow. The project's own file was read for grounding
  only.
- **Version it used:** 0.4.0.
- **Analysed against:** main at v0.4.0-41-gf5d403a, plus the commit "A received retrospective file is
  analysed under klarch-retrospective-intake, the retrospective names its files by subject, and the
  searcher's groups are balanced".
- **Standing entries that bear on it:** `issue@knowledge-architect@command-output-is-not-declared-a-contract`
  (W5, C1), `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to` (C3),
  `tripwire@core@private-item-needed` (C1), `tripwire@agent-skills@plain-text-pointer-found` (the
  standing answer on pointers).

## W1 and W2: review §3's outcomes

Analysed with W1 of the 2026-10-06 file, as one cluster, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-06-thaum-workflow.md`.

## W3: where a policy about how a Component's tests are written lives

- **Says.** The primer offers two rows, the design home and the scoped `CLAUDE.md`. Two rulings on
  what a suite may assert were written as directives in a scoped `CLAUDE.md`; an open issue would put
  a "keep" ruling in the design home. Proposed fix: a sentence placing test-suite policy.
- **Still applies, in part.** No installed text names a test-suite policy. But the entry tests changed
  after 0.4.0: test 2 of decision-recording now admits "a policy with no code of its own"
  (`path@agent-skills@content/skills/decision-recording/SKILL.md` lines 89-92), since `3a10fc9`, and
  test 4 admits a ruling the owner confirms as intent. The primer already restates a directive where
  it is delivered, with a pointer to its home. So the rulings take a head when a test passes, and the
  scoped `CLAUDE.md` restates them with a pointer: one home, one restatement.
- **Recorded.** None. `issue@agent-skills@test-3-admits-a-practice-its-tool-documents` is adjacent: a
  policy that only follows the crate's documented advice is its case.
- **Kind.** Unclear at 0.4.0; resolved by text the session did not have.
- **Proposal.** No change: repaired since 0.4.0 by the rewritten entry tests.
- **Default:** no change.
- **Outcome:** approved.

## W4: the advice on testing an extension

Analysed with W6 of the 2026-10-06 file, as one cluster, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-06-thaum-workflow.md`.

## W5: the advice says nothing of the core's printed output

- **Says.** The library section on testing an extension is silent on the summary, the verdict line,
  refusals, help and a finding line's layout; the owner had to rule mid-review. Proposed fix: extend
  it: a test asserts no text the core prints, reads a finding's location from the documented layout,
  and reads `Report::checks`, `Report::not_run` and `Report::summaries` instead.
- **Still applies.** `path@core@src/lib.rs` lines 149-187 have no sentence on printed output. The three
  fields are public (`path@core@src/check/mod.rs` lines 138-153, re-exported in
  `path@core@src/testing.rs`). No document gives a finding line's layout; it exists only in code
  (`path@core@src/finding.rs` lines 75-80). The core README declares part of the printed output a
  contract: "**The order is a contract**" (`path@core@README.md` lines 64-66).
- **Recorded.** `issue@knowledge-architect@command-output-is-not-declared-a-contract`, a design issue
  that the owner wants settled in a discussion: which part of the output is a contract, and which data
  gets a machine-readable form or a library function.
- **Kind.** Missing, but its first sentence is a consequence of the policy that issue leaves
  undesigned: advising "assert no printed text" while the README declares an order a contract would
  write half of that policy in a doc comment.
- **Scope.** Core and the root.
- **Better fix.** Add the instance to the design issue, and write the advice once the discussion
  decides the contract. Rival: write only the part that depends on nothing undecided now, "read
  `Report::checks`, `Report::not_run` and `Report::summaries` rather than the printed lines they
  produce". It is true under any outcome of that discussion.
- **Route.** An issue update, and a small doc-comment edit.
- **Proposal.** Handle now: the instance goes into the design issue, and the one sentence on the
  `Report` fields goes into the library section.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## C1: what the binary handed the core is observable only through printed text

- **Says.** Which checker-source directories the binary passed reaches the user only through the
  `checker source:` line. With printed text out of bounds, a binary that exempts too wide a directory
  passes every test. Proposed fix: `cli::run` returns the run's `Report`, or a JSON output mode.
- **Still applies.** `cli::run` returns `Result<ExitCode, String>` (`path@core@src/cli/mod.rs` lines
  122-127). No structured output exists.
- **Recorded.** `issue@knowledge-architect@command-output-is-not-declared-a-contract` names "which
  data gets a machine-readable form or a library function", the report's second branch.
- **Tripwire.** `tripwire@core@private-item-needed` fires when a consumer needs an item private
  behind the facade and no public item replaces it. The directory list is a literal of the
  consumer's own `main` (the library documentation's example passes it inline): a binary that computes
  it in one function of its own can test that function, with no core change. That tests what the
  binary passes, not what the core did with it, and the core's handling is the core's to test.
  Judged: it does not fire.
- **Kind.** Missing, from the consumer's side; served in part by the consumer's own code.
- **Better fix.** Add the instance to the design issue; the consumer-side function is noted there as
  the workaround.
- **Route.** An issue update.
- **Proposal.** Handle now: the instance and the workaround go into
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## C2: no checked form for a section of a dependency's documentation

Analysed with C1 of the 2026-10-06 file, as one cluster, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-06-thaum-workflow.md`. In short: since
`9b0e47d`, after 0.4.0, a description in words is outside the plain-text rule, so this pointer owes
nothing.

## C3: a finding line does not name the check that emitted it

- **Says.** To sort planted findings into the core's and the extension's, the session inferred each
  finding's family from its wording. Proposed fix: print the family on each line, or expose it on
  `Finding`.
- **Still applies.** `Finding` has `file`, `line`, `what` and `action`, and no family
  (`path@core@src/finding.rs` lines 12-20).
- **Recorded.** `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to` lists "a
  `Finding` carries no field naming the check that wrote it" among its gaps.
- **Better fix.** For the split this session needed, a public route exists: `Prepared::check` returns
  an `ExtensionReport` holding the extension's findings alone (`path@core@src/extension.rs` lines
  122-123 and 228-230). It does not name which of the extension's checks emitted each one.
- **Proposal.** Handle now: the instance, and the `Prepared::check` route, go into the existing entry.
- **Default:** handle now.
- **Outcome:** approved to handle now as proposed.

## The standing answers

- **The primer in subagents:** analysed with the same answer of the other files, in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-06-thaum-workflow.md`.
- **Three pointers written as bare plain text.** Two are a description in words of a section of
  documentation, which the rule no longer covers since `9b0e47d`. The third, the core's own mock
  projects, is described in words too, as far as the file shows. The report gives no finding they
  cleared, so `tripwire@agent-skills@plain-text-pointer-found` does not fire. No change.

- **Default:** no change.
- **Outcome:** approved.

## Standing entry: whether `tripwire@core@private-item-needed` fired on C1

- **Evidence.** C1: the consumer cannot read which checker-source directories its binary passed,
  except through printed text. The tripwire fires when a consumer "needs an item that is private
  behind the facade, and no public item replaces it".
- **Judgement proposed.** It does not fire. The directory list is a literal of the consumer's own
  binary, so a function of the consumer's own can return it and be tested, with no core change.
  What the core does with the list is the core's to test.
- **Default:** not fired; no response.
- **Outcome:** approved.
