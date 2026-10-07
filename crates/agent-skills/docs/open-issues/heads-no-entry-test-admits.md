---
kind: todo
---
# Design heads that no entry test admits are still in the design homes

## Summary

An audit of the 178 design heads the design homes held when the tests were decided, read against the
entry tests of
`design@agent-skills@a-head-is-owed-by-an-entry-test`, found 16 heads that no test admits, and the
owner named a few more to drop or split. They are still in the design homes, which every grounding
reads whole.

## Details

### What

The audit ran during the design discussion of the milestone load-bearing-records: four read-only
subagents, one per group of Components, gave each head two verdicts, under the entry tests then
installed and under the tests that discussion decided. In the reasons below, T1, T2 and T3 are the
three entry tests in their order; T2old and T2new are the second test's wording before and after
that discussion; T1w is the first test read as any consumed interface, which the discussion adopted;
TM is the test of `design@agent-skills@instruction-record-is-minimal`.

The work:

- the 16 heads admitted by no test: `design@knowledge-architect@toolchain-is-pinned` (root);
  `design@agent-skills@content-mirrors-the-install-layout`, `design@agent-skills@in-change-waits-for-premortem`,
  `design@agent-skills@plan-landing-is-not-tied-to-its-work`, `design@agent-skills@argument-ids`, `design@agent-skills@retiring-plan-opens-issue`,
  `design@agent-skills@setup-rust-section`, `design@agent-skills@setup-default-crates-io-page`, `design@agent-skills@component-goal-refines-root`,
  `design@agent-skills@retro-content`, `design@agent-skills@premortem-as-watch-points`, `design@agent-skills@retro-two-files`, `design@agent-skills@finding-ids`,
  `design@agent-skills@version-in-report`, `design@agent-skills@retro-file-location` (agent-skills); `design@core@spec-file-owns-its-items` (core);
- `design@agent-skills@roadmap-home`, which passes, and which the owner judged unneeded: "It is
  something that has no reason to ever change, and the path being named in multiple places is just
  basic self consistency.";
- the split of `design@knowledge-architect@repo-layout`, keeping only what the owner named: "only
  the knowledge of not naming a component `knowledge-architect` and accepting directories for crates
  that are not named against published packages deserves the record.";
- the split of `design@core@spec-file-owns-its-items`: its first sentence carries the citation form
  of a spec's items, which every adopting project writes and which passes T1; its ownership rule in
  `Anchors::owns_entry` belongs in a comment at that function;
- the move of `design@knowledge-architect@a-reference-claims-a-revisit` and
  `design@knowledge-architect@a-past-sentence-is-rewritten` to agent-skills: they shape the installed
  text, which the primer and two installed skills restate, so their home is that Component.

The audit agents' reasons for the 16, as they wrote them, with the line numbers they gave dropped:

| head | the audit agent's reason |
| --- | --- |
| `design@knowledge-architect@toolchain-is-pinned` | T2new fails: one site, rust-toolchain.toml, whose comment already carries the reason. T3 fails: the argument turns on clippy adding lints across releases, not on a reading of rustup's spec (unsure). T1: none. Built, so no T2old. |
| `design@agent-skills@content-mirrors-the-install-layout` | FILES stays `&[(&str,&str)]` whether generated or hand-written, so no T1; built; one site, build.rs, carries it; about the crate's build, not the installed text's intent, so TM fails. (unsure: core's namespace test is a second site) |
| `design@agent-skills@in-change-waits-for-premortem` | One site: the design skill; TM fails: no owner ruling, rationale of one instruction. |
| `design@agent-skills@plan-landing-is-not-tied-to-its-work` | One site: the planning skill; TM fails: "an owner plans" is generic, no ruling. (unsure: may be an unmarked owner ruling) |
| `design@agent-skills@argument-ids` | Id convention `a<n>` stated in planning only; ids are core's grammar either way, so no T1w; no owner ruling or two-part consistency. |
| `design@agent-skills@retiring-plan-opens-issue` | One site: planning §9; TM fails: no owner ruling, no two-part consistency stated. (unsure: wording "unplanned happenings" may be the owner's) |
| `design@agent-skills@setup-rust-section` | One site: setup skill (and its snippet); no owner ruling; no interface. |
| `design@agent-skills@setup-default-crates-io-page` | One site: setup skill; a proposal, not a format; consistency is with this repo's own decision, not two workflow parts. |
| `design@agent-skills@component-goal-refines-root` | One site: goal-setting; TM fails: consistency is with a root decision, not two workflow parts. (unsure) |
| `design@agent-skills@retro-content` | One site: retrospective; TM fails: scope of "missing" is rationale. |
| `design@agent-skills@premortem-as-watch-points` | One site: retrospective's standing questions; TM fails: no ruling, no consistency argued. |
| `design@agent-skills@retro-two-files` | One site: retrospective; no ruling. |
| `design@agent-skills@finding-ids` | One site: retrospective; an id convention of one text. |
| `design@agent-skills@version-in-report` | One site: retrospective. |
| `design@agent-skills@retro-file-location` | One site: retrospective; the owner's choice is made at run time, not a recorded ruling. |
| `design@core@spec-file-owns-its-items` | T1/T1w fail: internal ownership mechanism, citations unchanged. T2new fails: one site, `Anchors::owns_entry` in entity.rs. T2old/T3 fail. (unsure: the rival "every spec a directory" would change the layout) |

The counts, per Component:

| Component | heads | no test passes, current tests | no test passes, decided tests |
| --- | --- | --- | --- |
| agent-skills | 80 | 57 | 14 |
| core | 62 | 4 | 1 |
| root | 22 | 4 | 1 |
| gates and xtask | 14 | 2 | 0 |

What the cleanup owes before it deletes anything:

- **The agent-skills verdicts are the agents', not the owner's.** The owner read them by title:
  "Me agreeing with those verdicts was mostly skimming through the heads and judging mostly from
  titles. My word here was not to be taken  as a ruling on what is right." The cleanup session
  judges each head in full and presents its verdicts to the owner.
- **Two readings of TM were never ruled.** The agent-skills agent required an explicit ruling, quote
  or acceptance of the owner for "the owner's intent", so an argument derived from a goal did not
  count; and it did not count consistency with core or with a root decision as "two parts of the
  workflow". The 14 agent-skills verdicts apply TM under that reading; the cleanup puts both to the
  owner first.
- **Two heads that pass stay.** The owner judged `design@agent-skills@knowledge-table-home` and
  `design@agent-skills@design-hands-off-to-planning` "not critical, but they still carry useful
  informations".
- **Content worth keeping from an agent-skills head goes into a `%%` line** beside the instruction
  it explains, per `design@agent-skills@shipped-text-line-comments`.

### Why it matters

`design@agent-skills@a-head-is-owed-by-an-entry-test` keeps the design homes to what an entry test
admits, since every grounding reads them whole and a human overseer reviews them. Heads that no
test admits cost every such read and argue nothing a comment at the code would not carry. For
agent-skills the audit's outcome is unconfirmed: if the judgement in full finds over-recording there
that the owner rules significant, `design@agent-skills@a-head-is-owed-by-an-entry-test` is reopened
with the owner, though the owner said "I do not expect the cleanup to reopen this".

### What would close it

Each listed head deleted, split or moved on the owner's word, its content worth keeping moved to a
comment, a `%%` line or another head, and `cargo klarch check` passing; or the owner's ruling that a
listed head stays.
