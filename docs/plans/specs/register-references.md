# A register's home is cited `register@<anchor>@<register>`, and a `path` reference to a home is a finding that `--fix` respells

## Status and audience

This spec is the plan of the work that adds a reference kind naming a register's home. The work
does four things:

- a reference `register@<anchor>@<register>` resolves, through the manifest, to the home that
  anchor keeps the register in, whichever of its two shapes the home has;
- a `path` reference whose target is a register's home is a finding, whose repair is the `register`
  form;
- `cargo klarch check --fix` respells that finding;
- the primer defines the two words "register" and "home" once, beside each other, with the reason
  both are kept.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. Five defaults were put to the owner, D1 to D5, under "Defaults awaiting the owner"; the
  owner ruled on D1 to D3.
- **The spec lands before its work, in a pull request of its own.** Its work starts only once
  `spec@plans@path-quickfixes` has landed and been harvested, since this work adds rows to the
  respellings that spec builds; the order rationale gives the reason.
- **The work starts with the design audit** of
  `skill@knowledge-architect-planning@working-a-slice` point 2, since it does not start in the session
  where the discussion converged, and since `spec@plans@path-quickfixes` will have landed between.
  That landing deletes `spec@plans@path-quickfixes`, so every citation of it here dangles then:
  the session that deletes it removes each citation, as
  `skill@knowledge-architect-planning@plan-document-leaves` says, and this spec's audit reads the
  heads that spec's harvest wrote in their place. Every name this spec needs from it is defined
  here, under Names.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/c074f629-1136-4498-aff3-f53ca6bff772.jsonl`.
  Find the file by its opening message, not by its name: the discussion begins at the owner's
  message that opens "How does the checker handles segmented references with placeholders", and
  ends at the owner's message "One spec." It holds 7 owner messages, called rounds 1 to 7 below,
  each followed by one agent reply.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec. The code steps follow
`skill@klarch-development`, whose section `skill@klarch-development@ground-before-editing` comes
before any edit of the core. The edits of the installed text under `path@agent-skills@content/`
follow `skill@knowledge-architect-agent-configuration` and the section "Editing an installed skill
or agent" of `path@agent-skills@CLAUDE.md`, read whole.

## Names

- **a register**: one kind of recorded thing together with its storage, as
  `path@core@src/manifest.rs` declares it, the struct `Register`: `design`, `goal`, `tripwire`,
  `issue`, `spec`, `milestone`, the four item registers, and any a project declares. Its `shape` is
  one of the four variants of `Shape`: `Heading`, `File`, `Directory` (the `milestone` register
  alone) and `Section` (the item registers alone).
- **a home**: where an anchor keeps one register's entries, the struct `Home` of
  `path@core@src/entity.rs`, computed by `Anchor::home_of`. A heading register's home is the file
  `<home base>/<dir>.md` or the directory `<home base>/<dir>/`, never both, per
  `design@core@heading-register-two-shapes`; which of the two a tree holds is read from the tree's
  listing, as `heading_home` in `path@core@src/check/tree.rs` reads it. A file register's home, and
  the `milestone` register's, is the directory `<home base>/<dir>/`. An item register's home is a
  plan's own documents.
- **the head of a directory home**: the `README.md` of a heading register's directory home, which
  is an introduction and an index of its subdocuments, per
  `design@core@heading-register-two-shapes`. The `README.md` of a file register's directory, or of
  the `milestone` register's, is not a head in that sense: it is a hand-written guide, per
  `design@core@a-file-register-is-a-directory-of-entries`.
- **a home target**: a path that the mapping table of this spec maps to a `register` reference: a
  heading register's file, its directory or the head of its directory; a file register's directory;
  the `milestone` register's directory. Nothing else is a home target: not a subdocument, an entry,
  a file or `milestone` register's `README.md` or generated `index.md`, and no home of an item
  register.
- **the item registers**: `thread`, `argument`, `criterion` and `acceptance`, whose shape is
  `Shape::Section` and whose home is a plan's own documents, per
  `design@core@items-as-section-registers`.
- **a span**: a backticked span of one line, as the scanner of `path@core@src/scan.rs` records it.
- **a respelling**: a rewrite by `cargo klarch check --fix` of one span into another span that
  names the same entity, which changes no byte outside the span. `spec@plans@path-quickfixes`
  defines the term and the machinery; its harvest records them in the core design home.
- **a suffix**: a line number `:<n>` or a fragment `#<anchor>` at the end of a span's path.
- **a pair**: two `path` references in one sentence naming the two shapes of one heading
  register's home, as in "`path@<anchor>@docs/<dir>.md` or `path@<anchor>@docs/<dir>/`". The
  installed text and this repository's documents hold such pairs because the shape a project
  uses is not known in advance.
- **the old form**: a `path` reference whose target is a home target. **The new form**: the
  `register` reference that names the same home.

## What the work is

**What exists today**, each read in the tree at the commit this spec was written on:

- A kind is a register's name, `path`, `planned`, or, under the `claude` harness, a harness kind,
  per `design@core@one-entity-table`. `Anchors::kind` in `path@core@src/entity.rs` resolves the
  word, and `is_kind_name` in the same file lists the words no anchor may wear. No kind is named
  `register`, and no anchor of this repository is named `register`.
- A backticked span whose head names no kind and no anchor is silent, per
  `design@core@candidate-rule-and-retired-forms`. So a span such as `register@core@design` is
  silent today, and every such span this spec writes is read by no check until the work lands.
- `Anchors::home(anchor, kind)` in `path@core@src/entity.rs` returns a `Home` holding the paths of
  every shape, `file`, `dir`, `readme`, `index`, and the register's `Shape`, or `None` where the
  anchor carries no such register. It does not read the tree, so which shape of a heading home
  exists is not in it.
- `judge_part` in `path@core@src/check/references.rs` sends a `path` reference to `fn path`, a
  `planned` one to `fn planned`, and every other kind to `fn table`, whose unknown-anchor repair says
  that `*` and `elsewhere` serve the `path` kind alone.
- `fn path` resolves a `path` reference against the tree. Its generic anchor `*` accepts a required
  document name in any of its shapes, from `Anchors::required_kind` in `path@core@src/entity.rs`:
  the compiled documents, every component-scoped heading register's file, directory and README,
  and every component-scoped file register's directory, README and index. A literal anchor's
  target goes through `anchored_target`, which refuses a refused shape, a plan document cited by its
  path and a target inside a deeper anchor, then through `assert_target`, which asserts that the
  target exists with its claimed kind. A suffix on a `path` reference is not stripped: the target
  with its suffix does not exist, and `assert_target` reports it.
- `resolve_registers` in `path@core@src/manifest.rs` refuses a declared register named like a
  harness kind, `path` or `planned`, since a reference whose kind segment is that word never
  reaches the register.
- `names_this_project` in `path@core@src/check/references.rs` reports a backticked `<word>#<id>`
  as the retired form when its word is a kind.
- `cargo klarch show` (`fn show` in `path@core@src/cli/mod.rs`) resolves a reference and lists
  every site that references it, through `records::inbound` in `path@core@src/records.rs`, which
  compares the anchor as written.
- `cargo klarch check --fix` writes only the generated files and the installed agent files,
  per the doc comment of `CheckArgs::fix` in `path@core@src/cli/mod.rs`.
  `spec@plans@path-quickfixes` plans the respellings of hand-written documents, Markdown documents
  and Rust comments of the walk, and it is not built. Its respelling rule applies "never to `*` or
  `elsewhere`", and its mapping table gives nothing to "a `path` reference under the anchor `*`: any
  of its findings". It leaves a trailing slash that disagrees with the target's kind unrespelled, its
  D3.
- The installed text, under `path@agent-skills@content/`, and this repository's own documents name
  register homes with the old form, much of it as pairs. The census at the design discussion,
  over `git ls-files '*.md' '*.rs'`: 128 `path` spans with the generic anchor `*`, of which 76 are
  home targets; and about 70 `path` spans with a literal anchor that are home targets. Re-take it
  with `grep -ohE` over that listing, for backticked `path` spans whose anchor is `*`, then for
  those whose path is a heading register's file or directory, or a file register's directory,
  under docs/; and the same for every literal anchor. About half the lines are the installed
  copies under .claude of lines under `path@agent-skills@content/`.
- Tests pin today's acceptance of home targets: `the_generic_anchor_accepts_the_required_set_and_what_a_component_carries`
  and `a_required_name_still_carries_the_kind_claim` in `path@core@src/check/references.rs`, and
  `the_required_document_set_carries_each_shape_with_its_kind` in `path@core@src/entity.rs`.
- The mock project `path@core@tests/projects/dirhome/` holds the old form `path@plans@specs/` in its
  milestone document, and `the_conformant_mocks_report_nothing_over_every_core_check` in
  `path@core@src/mock_projects.rs` asserts that it reports nothing. The manifest's `exclude` keeps
  the mock projects out of this repository's walk, so `--fix` run here does not reach it.
- Text restates the kind list or the generic anchor's acceptance: the module comment and the doc
  comments of `Kind`, `is_kind_name`, `Anchors::kind` and `required_kind` in
  `path@core@src/entity.rs`; `path@core@README.md`, in its sections on `check`, on `show` and on the
  manifest's refusals; the root CLAUDE.md, `instructions@mechanical-validation`.

**What is outside the work, and why:**

- **Checking placeholder spans.** A span holding `<` or `>` stays unrecorded, as
  `design@core@candidate-rule-and-retired-forms` states. The discussion ruled it out,
  `thread@register-references@placeholder-spans-checked`. A scoped form, checking them in the
  installed text alone, is added to `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`
  as one candidate shape of that issue's check, at the harvest.
- **Replacing `*`.** The generic anchor stays, per
  `thread@register-references@component-placeholder-anchor`.
- **Rewording "home" into "register" across the documents.** Both words stay, per
  `thread@register-references@register-vocabulary`.
- **The respelling machinery itself**: the module, the run order, the printed lines and the write
  guard of `--fix` are `spec@plans@path-quickfixes`'s. This work adds rows to its respellings.
- **A `planned` reference to a future home.** `planned@<anchor>@<path>` keeps its rule,
  `design@core@planned-path-form`.
- **A milestone or a spec anchor in the new form.** A milestone's `spec` register has its home at
  the milestone's own directory, which `milestone@plans@<id>` already names, and one document with
  two names is what `design@core@plan-document-kinds` refuses. A spec anchor carries only item
  registers. Neither is an anchor of the new form, per D4. The rule the owner approved in
  `thread@register-references@home-reference-anchors` admits a milestone for its `spec` register, so
  this narrows that ruling.
- **A rendered link for the new form.** `issue@core@the-documents-do-not-render-as-a-site` owns
  the rewrite of every reference into a link; this work gives that pass one more kind to resolve.

## What is already decided

**The decisions the design rests on and does not argue again:**

- `design@core@heading-register-two-shapes`: a heading register's home has two shapes, never both.
- `design@core@a-file-register-is-a-directory-of-entries`: a file register's home is a directory.
- `design@core@kind-names-refused-as-anchors`: no anchor wears a kind's name. The owner's words in
  it state the rule, "all kind names should be refused", so the new kind word is a member within
  the ruling.
- `design@core@candidate-rule-and-retired-forms`: a span whose head is a kind is a candidate; every
  kind but `path`, `planned` and the harness kinds takes exactly three non-empty segments; a
  backticked `<word>#<id>` whose word is a kind is the retired form. The new kind is such a kind, so
  its rules apply to it unchanged, and a backticked `register#<x>` becomes a retired-form finding;
  the tree holds none (`git grep` for a backtick, the word and a hash sign). The work's `--fix` run
  re-points one span inside this head's body, its pointer to the core tripwires home. Re-pointing a
  reference in a head does not touch it, per `primer@design-heads`, so the split that
  `issue@core@four-heads-bundle-decisions-with-many-citing-sites` owes at the next edit of this head
  is not triggered. `design@agent-skills@existing-heads-on-touch` names no such exemption, which
  `issue@agent-skills@the-head-on-touching-heads-omits-the-primer-s-re-pointing-exemption` records;
  the design audit reads that entry, and the split is part of this work if it closes against the
  exemption.
- `design@core@every-path-names-its-anchor`: the `path` grammar, the deepest-anchor rule, and the
  admission of `path@plans@<file>`. The finding on a home target is a head of its own, not a member
  of this head: this head is one of the bundles that
  `issue@core@four-heads-bundle-decisions-with-many-citing-sites` records, and an addition that
  answers another question takes a head of its own, per `primer@design-heads`.
- `design@core@trailing-slash-claims-directory`: a `path` claims its target's kind by its trailing
  slash. The new form claims nothing of the kind, since the manifest and the tree give it.
- `design@core@plans-is-a-reserved-anchor` and `design@core@plan-document-kinds`: the anchor
  `plans` and the citation of a plan document by its kind.
- `design@core@items-as-section-registers` and `design@core@plan-item-scope`: an item register's
  home is a plan's documents, and an item is cited from inside its plan only.
- `design@core@finding-names-the-repair`: each new finding names its repair.
- `goal@core@declared-instructions-are-checked` and `goal@core@relocation-is-one-manifest-edit`,
  from which the two binding criteria derive.

**The decisions the work rewrites, and every text that references each**, as `cargo klarch show`
listed it at the commit this spec was written on. Re-run `show` at the design audit: the lists grow.

- `design@core@one-entity-table` is rewritten at the harvest: its kind list gains `register`,
  resolved against the anchors and the tree's listing as `path` is resolved against the survey.
  Referenced at: the agent-skills design home, line 951; the core design home, lines 931 and 1960;
  `path@core@src/entity.rs`, line 6. The restatements of the kind list in `path@core@src/entity.rs`
  are updated at step 1; the root CLAUDE.md's at step 4. The other sites cite the head for the
  table, not for the list, and the harvest's review judges each.
- `design@core@reserved-anchors` is rewritten at the harvest: `*` gains a meaning under the
  `register` kind, every Component's own home of a component-scoped register, and the generic `path`
  form no longer accepts a home target. Its illustration names the core tripwires home with the old
  form, and is rewritten to a path that is no home target. Referenced at: the root CLAUDE.md, lines
  135 and 155, updated at step 4; the agent-skills design home, lines 95, 936 and 969; the core
  rejected alternatives, lines 143 and 155; `path@core@src/check/references.rs`, lines 547 and 1917,
  updated at steps 1 and 2. The harvest's review judges each site against the rewritten head.
- `design@core@an-unresolved-reference-is-reported` is rewritten at the harvest: it counts five ways
  a reference resolves to nothing, and says the third, an anchor carrying no register of the kind,
  is reachable only by a location. Under the new kind a Component can carry no such register, as for
  `register@core@spec`, and the new kind adds refusals of its own. The head states the new kind's
  ways, or states that each kind's resolver gives its own.
- `design@core@registers-are-declared` is rewritten at the harvest: its list of the names a declared
  register may not take, "`path`, `planned` or a harness kind", gains `register`. Its argument, "a
  reference whose kind segment is that word never reaches the register", covers the member.
- `design@core@harness-kinds` is rewritten at the harvest: its sentence "So they sit beside `path`
  and `planned`, the two kinds already no register" becomes false, since the new kind is no
  register either.
- `design@agent-skills@shipped-text-cites-no-entry` is rewritten at the harvest: the forms the
  shipped text may write gain `register@*@<register>` and `register@plans@<register>`, and its
  sentence "where the text names both shapes, both are references" goes, since the text then names
  one register. The owner's quoted ground, that the shipped text "promotes the same kind of syntax as
  what the checker enforces", holds of the new form. Referenced at:
  `path@agent-config@skills/klarch-release/SKILL.md` line 66; `path@agent-skills@CLAUDE.md` lines
  24 and 46, updated at step 4; the agent-skills design home, line 2280;
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`, lines 21 and 35, edited at the
  harvest.
- The heads that `spec@plans@path-quickfixes` writes at its harvest on what `--fix` may do, among
  them `design@core@safe-fix-definition` and `design@core@fix-scope` as rewritten there. Two of its
  exclusions are changed for the finding on a home target, per D2: a respelling under `*`, and the
  dropping of a trailing slash. The design audit reads those heads as they stand then, and names
  the clause each change rewrites.
- `tripwire@core@fix-makes-a-choice` fires on this spec's proposal: its clause exempts only the
  respellings `spec@plans@path-quickfixes` defines, and this spec proposes others. Its response is
  to reopen `design@core@safe-fix-definition` before the fix is added, which
  `spec@plans@path-quickfixes` already does, and its harvest deletes the entry. Per D3, the clause is
  narrowed in this spec's branch to except this spec's respellings too, on the owner's word.

**The standing entries the discussion's search returned, and their outcome.** The search ran at the
grounding of the discussion, by three searchers over the 121 issues and tripwires of the tree: the
rows of `cargo klarch issues` and of `cargo klarch tripwires`, the header rows excluded. The last
row was added by the review of this spec.

| entry | outcome |
| --- | --- |
| `tripwire@core@candidate-rule-silence` | Not fired. A placeholder span is an illustration, not a span meant as a reference. The `register` spans this spec writes are meant as references and are silent until the work lands; that silence is accepted knowingly, the second response the tripwire allows, and ends at step 1. |
| `tripwire@agent-skills@plain-text-pointer-found` | Bears on nothing: the work adds a checked form and moves no pointer into plain text. |
| `tripwire@core@planned-path-outside-plans` | Bears on nothing: `planned` and the escape anchor are unchanged. |
| `tripwire@core@extension-defines-a-kind` | Not fired: the kind is the core's, not an extension's. |
| `tripwire@core@checker-source-literals-are-data-self-location` | Bears on nothing: the new findings are raised over documents and comments, and the checker's own string literals stay data. |
| `tripwire@core@fix-makes-a-choice` | Fired on this spec's proposal; see the list above and D3. |
| `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` | Bears on nothing: the work does not touch an empty head, and does not change the candidate rule. |
| `issue@core@cross-project-references` | Bears on nothing: the new form names an anchor of the project that holds it. |
| `issue@core@four-heads-bundle-decisions-with-many-citing-sites` | Bears on the scope: of the four heads, the work's `--fix` run re-points one span in `design@core@candidate-rule-and-retired-forms` and edits none of them, per `primer@design-heads`; the finding on a home target takes a head of its own rather than growing a bundle. At the commit this spec was written on, seven heads of the core design home held an old form, counted with a Python scan of each head's body for a backticked `path` span to a home target. |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | Bears on nothing: the harness kinds are unchanged. |
| `issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` | Bears on step 1: `Entities::defines_id` decides which backticked `#<id>` is the retired form. The new kind resolves outside the entity table's ids, so no register's name becomes such an id, and a test asserts it, `argument@register-references@a26`. |
| `issue@core@finding-texts-are-not-audited-for-a-needless-cause` | Bears on the new findings' texts, which follow `design@core@finding-names-the-repair` from the start. It audits none of the existing ones. |
| `issue@core@the-documents-do-not-render-as-a-site` | Bears on a cost of `thread@register-references@register-home-reference`: a reader of a document outside the tool does not see the file a `register` reference resolves to. The pass that issue plans resolves it to a link. No tripwire watches it, on the owner's ruling. |
| `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` | Bears on nothing: the work does not change which bare mentions are read. |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing` | Bears on nothing: its re-entry point, the next change to `design@core@candidate-rule-and-retired-forms` or to `design@core@plan-item-scope`, is not met, since the work changes neither. |
| `issue@core@a-checker-only-project-carries-the-workflow-skeleton` | Bears on nothing: the set of required homes is unchanged; only how they are cited changes. |
| `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply` | Bears on nothing: the finding it names is not touched. |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` | Bears on the new finding: a home target with a suffix gets the home-target finding, whose repair names the new form and says to drop the suffix. The suffix of any other `path` reference keeps the repair that issue records. |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | Bears on nothing: no tripwire is recorded by this work. |
| `issue@agent-skills@an-audit-report-cites-the-heads-its-own-pass-removes` | Bears on nothing: the work removes no head. |
| `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` | Bears on the ruled-out thread: checking placeholders would have removed the escape that issue records. The ruling keeps the escape. |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | Bears on the harvest: the scoped form of placeholder checking is added to it as a candidate shape, and its list of the forms the shipped text may write gains the new form. |
| `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` | Bears on a cost: the new kind word becomes a name no anchor may wear. The question that entry asks, which ground the refusal stands on, is not changed by one more member. |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | Bears on the changelog: an adopting project's root CLAUDE.md that restates a home with the old form gets a finding after the upgrade, which `--fix` respells. The Migration entry of step 4 names it. |
| `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to` | Bears on nothing now: `fn home_target` is crate-private. The read-only view that entry plans would be the place to expose it. |

## Criteria

### No pointer that fails to conform is made silent `##no-silent-pass`

Binding. Derived from `goal@core@declared-instructions-are-checked`: a tree that passes the check
conforms to all of it. Satisfaction: met. The new form is a recorded and checked kind,
`thread@register-references@register-home-reference`; the old form is reported,
`thread@register-references@path-to-a-home-is-reported`;
`thread@register-references@component-placeholder-anchor`, which failed it, is ruled out.

### Reshaping a home edits no document `##refactor-edits-no-document`

Binding. Derived from `goal@core@relocation-is-one-manifest-edit`, whose title says "relocating or
refactoring what exists costs one manifest edit and no document". Satisfaction: met by
`thread@register-references@register-home-reference` together with
`thread@register-references@path-to-a-home-is-reported`. A home that moves from one file to a
directory dangles no reference.

### One meaning has one spelling `##one-grammar`

Binding, as a presumption. Derived from `design@core@an-entity-belongs-to-its-anchor`, and from the
rejected alternative "The `@` prefix reused as the root-relative spelling", which lost on "two shapes
on one meaning". Satisfaction: met. `*` is kept, a home has one spelling since its `path` form is
refused, and a milestone's directory keeps its one name, `milestone@plans@<id>`.

### A document can still illustrate an invalid form `##illustration-escape`

Weighed. Derived from the core design home and the issue entries, which illustrate broken shapes on
purpose, and from the owner's ruling recorded in
`issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` that the checker gives
quotations no other escape. Satisfaction: met. Placeholder spans stay unrecorded,
`thread@register-references@placeholder-spans-checked`.

### The cost of migrating the spans and of splitting heads `##migration-cost`

Weighed. Measured at the discussion: 354 placeholder spans; about 70 `path` references with a
literal anchor to a register home, and 76 `*` spans naming one, in this repository; the same forms
in every adopting project's own documents. Satisfaction: met for every old form written as one span
with no suffix, by `acceptance@register-references@fix-respells-every-old-form`. Unmet for a pair,
which `--fix` turns into two identical spans that a writer collapses by hand, and for a span with a
suffix, which a writer repairs by hand: unmet-and-accepted, on the owner's ruling on D1, which names
this criterion.

## Threads

### A kind names a register's home through the manifest, either shape `##register-home-reference`

Proposed by the owner, round 1: "Or maybe a better solution would be to allow refering to a
register as a kind: `register@<anchor>@design` (with anchor being either placeholder or existing
name)." Final state: approved. Moved by `argument@register-references@a7`,
`argument@register-references@a8`, `argument@register-references@a9` and
`argument@register-references@a10`. The owner, round 2: "the only thing I found hard to write was
references to the two-shape registers (design, issues, etc) actually, but register-home-reference is
the idea that solves this." Closed by the owner's word on the checkpoint table, round 6: "Keep none
of the three tripwires, but keep the three AC. Proceed". The table said "A positive word against
this table closes #register-home-reference and confirms the rest." Shape: "The new kind" under
Decided design. Harvest: a new head in the core design home, and the rewrites of
`design@core@one-entity-table`, `design@core@an-unresolved-reference-is-reported`,
`design@core@registers-are-declared` and `design@core@harness-kinds`.

### The kind's word is `register` `##register-home-kind-word`

Proposed by the agent, round 3, with `home` as the default, `argument@register-references@a16`.
Final state: approved, as `register`. The owner, round 4: "register-home-kind-word: I'd rather use
register. home just reads weirdly." The argued rivals did not settle it, since either word costs the
same refusal of an anchor name, so the choice is the owner's. Shape: "The new kind". Harvest: in the
head of `thread@register-references@register-home-reference`, with the owner's words as the ground
for the word.

### A `path` reference to a register's home is a finding, respelled to the `register` form `##path-to-a-home-is-reported`

Proposed by the agent, round 3, `argument@register-references@a17`. Final state: approved. The
owner, round 4: "path-to-a-home-is-reported: agreed on this shape." Shape: "The finding on a home
target" and "The respelling" under Decided design. Harvest: a new head of the core design home
whose title states the finding alone; the respelling goes to the heads on what `--fix` may do,
since it loses to another rival, a migration by hand. Also the rewrite of
`design@core@reserved-anchors`.

### The anchors that admit the kind `##home-reference-anchors`

Proposed by the agent, round 3, `argument@register-references@a18`: "any anchor that carries the
named register, and whose home for it is a file or a directory", with the examples a Component, `*`
for the component registers, a declared location, and `plans` for spec and milestone; the item
registers excluded. The rule admits a milestone anchor for its `spec` register, whose home is the
milestone's directory; D4 narrows it. Final state: approved. The owner, round 4: "home-reference-anchors: agreed." Shape: "The
new kind". Harvest: in the head of `thread@register-references@register-home-reference`, as the
rule's parameter, and in the rewrite of `design@core@reserved-anchors`.

### Both words stay, defined once beside each other in the primer `##register-vocabulary`

Proposed by the owner, round 4: "I think mentions that uses home such as "design home" should all be
reworded to register (tripwires, issues, design, goals...). Home is just another vocabulary for
something that is already named register in the code, AFAIK." The agent put the corrected premise to
the owner, `argument@register-references@a20`, and argued two shapes,
`argument@register-references@a21`. Final state: approved, shape 1. The owner, round 5:
"register-vocabulary: go with shape 1, I think this is the right balance". Shape: "The vocabulary
sentence". Harvest: the primer's sentence and its reason, in the installed text, at step 4; the
harvest judges whether the decision earns a head, below.

### Placeholder spans are checked `##placeholder-spans-checked`

Proposed by the owner, round 1: "I was considering giving those shapes their own checks. They'd need
to respect the correct placeholder names depending on kind (and to use a valid kind)." Final state:
ruled out. Moved by `argument@register-references@a1` to `argument@register-references@a5`. The
owner, round 4: "placeholder-spans-checked: agreed with you position, rule this out." The position
agreed with included the scoped form going to
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. Shape: "Losing alternatives".
Harvest: the scoped form into that issue; a rejected alternative if the recording tests admit it.

### `<component>` as the generic anchor `##component-placeholder-anchor`

Proposed by the owner, round 1: "Instead of using `path@*@<path>`, using something like
`path@<component>@docs/design.md` would be nice too, I believe. It might allow relaxing the current
rules without losing conformance." The owner's message wrote the first span with three dots in place
of the path; it is written here with a placeholder, since the checker reads a quotation as it reads
any text. Reframed in round 2: "the <component> placeholder is cleared that * IMO. It could replace
the * shape." Moved by `argument@register-references@a6`, `argument@register-references@a12`,
`argument@register-references@a13` and `argument@register-references@a14`. Final state: ruled out.
The owner, round 3: "Let's keep the * shape. I still find it a bit confusing, but the change is not
worth it as you say." Shape: "Losing alternatives". Harvest: a rejected alternative if the recording
tests admit it.

## Arguments

### Placeholder spans are not recorded today `##a1`

Round 1, the agent; bears on `thread@register-references@placeholder-spans-checked` and
`thread@register-references@component-placeholder-anchor`. "The span regex excludes any span that
holds `<` or `>` (the regex `AT_SPAN` of `path@core@src/scan.rs`), so the span is never recorded."
The silence is deliberate: `design@core@candidate-rule-and-retired-forms` says "an illustration
writes a placeholder in angle brackets, which the tokenizer does not record".

### The census: 354 placeholder spans, no defect `##a2`

Round 1, the agent; bears on `thread@register-references@placeholder-spans-checked`. "There are 354
placeholder spans. I checked their literal parts against the kinds, the arity of each kind, and the
declared anchors. That found **zero real defects**." The check would flag 4: two deliberate
illustrations of invalid forms, a misspelled kind in the core design home and a rejected
alternative's `section@` form, and two Rust format strings in the checker's source. The command is a
`grep -ohE` of backticked `@` spans holding an angle bracket over `git ls-files '*.md' '*.rs'`,
piped into a Python script that splits on `@` and tests the head, the arity and a literal anchor.

### Placeholder names carry meaning `##a3`

Round 1, the agent; bears on `thread@register-references@placeholder-spans-checked`. Anchor-position
names, counted with a `grep -ohE` of a backticked lower-case kind followed by `@<…>@` over the same
listing: `<anchor>` 60, `<name>` 30, `<milestone>` 20, `<component>` 14, `<parent-anchor>` 5, `<c>`
4. "The names carry information a fixed vocabulary would erase": `spec@<milestone>@<slice>` says the
anchor is a milestone, and `path@<parent-anchor>@<dir>/` states how an anchor's own directory is
named. "Enforcing names means rewriting most of the 354 spans and buys no detected defect."

### Checking placeholders costs the escape and a head split `##a4`

Round 1, the agent; bears on `thread@register-references@placeholder-spans-checked`. "It costs the
2 illustrations a rewrite"; "It changes the candidate rule, which triggers the split of a bundled
head with 23 citing sites", the count `issue@core@four-heads-bundle-decisions-with-many-citing-sites`
states; "It narrows the escape the quotation issue records."

### A span with no literal segment stays silent `##a5`

Round 1, the agent, on the owner's question about a span whose kind is a placeholder; bears on
`thread@register-references@placeholder-spans-checked`. "With no literal segment there is nothing to
check, so it stays unrecorded. [...] it is also the escape that #illustration-escape needs."

### A placeholder anchor is silent, or a second spelling of `*` `##a6`

Round 1, the agent; bears on `thread@register-references@component-placeholder-anchor`. "Under
today's scanner this span is invisible. So adopting it alone relaxes the rule by making the pointer
silent." "To be checked, it needs #placeholder-spans-checked, and then it gets exactly the check `*`
already has. It is still "each Component's own copy", now with two spellings".

### A home's change of shape dangles every path to it `##a7`

Round 1, the agent; bears on `thread@register-references@register-home-reference`. "Under
`design@core@heading-register-two-shapes`, a design home that outgrows one file becomes a directory.
Today every literal pointer to it then dangles: core alone has 11 [pointers to its design file] and
14 [to its tripwires file]. Under this kind, none of them dangles." Counted with a `grep -ohE` of
backticked `path` spans with a literal anchor ending in `docs/(design|goals|tripwires)(\.md|/)`.

### The pair goes from the installed text `##a8`

Round 1, the agent; bears on `thread@register-references@register-home-reference`. "It removes the
file-or-directory pair, about 76 `*` uses, from the primer, the CLAUDE.md files and the installed
skills. That text then stays correct in a project that uses either shape. This serves
`goal@knowledge-architect@any-project-can-adopt-it`." The review of this spec found the 76 spread
wider: the agents under content/, two design homes, and the installed copies of the same lines.

### What the kind costs `##a9`

Round 1, the agent; bears on `thread@register-references@register-home-reference`. The kind word
becomes a name no anchor may wear, per `design@core@kind-names-refused-as-anchors`; "A reader no
longer sees the file path in the text"; the documents get migrated, "and `check --fix` could do it
mechanically"; "`*` stays, for required documents that are not register homes".

### Shape-agnostic `path` resolution breaks the kind claim `##a10`

Round 1, the agent; bears on `thread@register-references@register-home-reference`. The nearest rival:
"`path@<anchor>@docs/<dir>` would match either `<dir>.md` or `<dir>/`. The discriminating fact
against it: a `path` asserts its target's kind through the trailing slash
(`design@core@trailing-slash-claims-directory`). A path that matches either shape breaks that
assertion and adds a special case to path resolution."

### The hard case was the two-shape homes `##a11`

Round 2, the owner; bears on `thread@register-references@component-placeholder-anchor` and
`thread@register-references@register-home-reference`. "the only thing I found hard to write was
references to the two-shape registers (design, issues, etc) actually".

### Few uses of `*` remain once homes have their form `##a12`

Round 2, the agent; bears on `thread@register-references@component-placeholder-anchor`. In
Markdown, 14 spans name the rejected alternatives, 2 a README, 1 a CLAUDE.md and 1 a fixture path,
counted with a `grep -oE` of backticked `*` spans that name no home and hold no placeholder; in
Rust, 18, all test fixtures, a figure the review of this spec did not reproduce (it found 20). "So
in documents the replacement touches about 18 spans. [...] The argument is about the rule, not the
cost."

### A typo of `<component>` is silent `##a13`

Round 2, the agent; bears on `thread@register-references@component-placeholder-anchor`. "A typo of
`*` is reported, because the span is recorded and its anchor is unknown. A typo of `<component>` is
another placeholder, so the span is not recorded and nothing reports it. That fails
#no-silent-pass". And a trap for writers: "The primer's instruction, "an illustration that must not
resolve writes a placeholder in angle brackets", would then be false for one placeholder name."

### The synthesis buys only the spelling `##a14`

Round 2, the agent; bears on `thread@register-references@component-placeholder-anchor`. A `path`
span with a placeholder anchor and a literal path recorded, `<component>` resolving as `*`, any other
placeholder name a finding. Its cost: one issue entry's illustrations, two scanner tests, the
illustration of a literal file under an unnamed anchor, and the edit of
`design@core@candidate-rule-and-retired-forms` with its split. "What it buys: your spelling of the
generic anchor."

### The owner keeps `*` `##a15`

Round 3, the owner; bears on `thread@register-references@component-placeholder-anchor`. "Let's keep
the * shape. I still find it a bit confusing, but the change is not worth it as you say."

### `home` matches the words the documents use `##a16`

Round 3, the agent; bears on `thread@register-references@register-home-kind-word`. "Over tracked
`.md` files: "design home" occurs 223 times, "goals home" 50, "tripwires home" 42, and "design
register" 2." Counted by joining every file's lines with `tr '\n' ' '`, then `grep -o` of the
phrase; the review of this spec counted 227, 50 and 42 with a match across any whitespace. The owner
chose `register` over it in round 4.

### The old form must be refused, or a reshape still edits documents `##a17`

Round 3, the agent; bears on `thread@register-references@path-to-a-home-is-reported`. "If [the path
form] stays legal, writers keep writing it. A shape change of the home then dangles those
references, as it does today. #refactor-edits-no-document is met only if the path form of a home is
refused." "A path *into* a directory home stays legal [...] because it names one subdocument." The
precedent, not authority: `spec@plans@path-quickfixes` refuses a `path` citation of a skill's file
and respells it.

### Which anchors admit the kind `##a18`

Round 3, the agent; bears on `thread@register-references@home-reference-anchors`. "any anchor that
carries the named register, and whose home for it is a file or a directory". "Excluded: the
plan-item registers [...] Their home is a section of a plan document, and a plan document is already
cited as `spec@plans@<id>`." "`*` admits only the registers that every Component carries, consistent
with `design@core@reserved-anchors`."

### The owner chooses the word `register` `##a19`

Round 4, the owner; bears on `thread@register-references@register-home-kind-word`. "I'd rather use
register. home just reads weirdly."

### The two words name two things in the code `##a20`

Round 4, the agent; bears on `thread@register-references@register-vocabulary`. The owner's premise
was that "Home is just another vocabulary for something that is already named register in the
code". "The code uses both words, for two different things": `register` is a kind of recorded
thing, `home` is "**where** a register's entries are stored". Counted in `path@core@src/` with a
`grep -ohE` of identifiers containing each word: the bare identifier `register` 542 times, `home`
334 times. "A test name shows that the two must stay distinct:
`two_registers_of_one_anchor_may_not_share_a_home`." And "home" is wider than registers: the
knowledge table's "every durable decision has exactly one home" names a README, a CLAUDE.md, a
skill or a comment.

### A wholesale rewording loses sentences `##a21`

Round 4, the agent; bears on `thread@register-references@register-vocabulary`. "A slug is defined in
its register's home, never in the README of a directory-shaped home" and "Two registers of one
anchor may not share a home" cannot be written with one word. Shape 1 defines both words once; shape
2 rewords where the phrase means the record, "a judgement at each of about 330 register-related
sites", the sum of the counts of "design home(s)", "goals home(s)" and "tripwires home(s)" taken as
in `argument@register-references@a16`. "No check can verify those judgements".

### A file register's README is a guide, not a head `##a22`

Round 5, the agent, in the premortem; bears on `thread@register-references@path-to-a-home-is-reported`.
"For a heading register, the directory's README.md is the home's head, so a pointer to it means the
home. For the issue register, README.md is a hand-written guide with content of its own, per
`design@core@a-file-register-is-a-directory-of-entries`. A pointer to it is not a pointer to the
register." The tree held one pointer to a heading register's README, counted with a `grep -ohE` of
backticked `path` spans ending in a register directory's `README.md`.

### AC1 meets the cost of the migration `##a23`

Round 6, the agent; bears on the criterion `criterion@register-references@migration-cost`. "The
#migration-cost line I showed as "unmet-and-accepted?" is now **met by AC1**: `--fix` respells every
old form, so no project migrates by hand." `argument@register-references@a25` narrows it.

### `--fix` respells nothing hand-written today `##a24`

Round 6, the agent; bears on the order of the work. "AC1 needs `--fix` to respell a reference, and
today `--fix` writes only generated and installed files (the doc comment of `CheckArgs::fix` in
`path@core@src/cli/mod.rs`). `spec@plans@path-quickfixes` builds that respelling machinery, and it
rewrites the design heads on what `--fix` may do."

### Collapsing a pair is no respelling `##a25`

At the assembly, the agent; bears on `acceptance@register-references@fix-respells-every-old-form`
and `criterion@register-references@migration-cost`. AC1, as ruled, has `--fix` respell "a `*` pair
(the file form and the directory form of one register, in one sentence) as one
`register@*@<register>`". That edits bytes outside the two spans, the word "or" between them and
the second span, and a respelling changes no byte outside its span. Each span alone respells to
the same new form, so a pair becomes two identical spans in one sentence, which passes the check.
A span with a suffix is not respelled either, since dropping the suffix loses bytes the writer
wrote. D1 carries the default.

### Register instances must stay out of the retired-form lookup `##a26`

At the assembly, the agent; bears on `thread@register-references@register-home-reference`.
`issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` reads that
`Entities::defines_id` is true for an id of any register whose kind is not a harness kind. Were a
register instance an entity with its register's name as id, every register name backticked with a
hash sign before it would become a finding.

### The `*` exclusion of path-quickfixes was argued for other findings `##a27`

At the review of this spec, the agent; bears on
`thread@register-references@path-to-a-home-is-reported` and D2. `spec@plans@path-quickfixes`
applies a respelling "never to `*` or `elsewhere`", and gives nothing to "a `path` reference under
the anchor `*`: any of its findings". The findings of `*` it had are "resolves in no component" and
the kind claims, whose repair needs the writer to choose a path or a shape. The finding on a home
target under `*` names one entity, every Component's home of one register, and the new form names
the same; the trailing slash it drops chooses nothing, since the new form names the home in both
shapes. The exclusion as written still covers this case, so the change is a change of that
decision, put to the owner.

## New names, in one place

An illustration of the shapes, not authority; the design audit and the implementing session decide
the final names.

```text
crates/core/src/entity.rs
  const REGISTER_KIND: &str = "register"              new: the kind naming a register's home
  Kind::register(), Kind::is_register()               new
  Anchors::kind                                       existing: gains the word
  Anchors::kinds_listed, is_kind_name                 existing: gain the word
  Anchors::home(anchor, kind)                         existing: the paths of the home
  fn home_target(target, anchors)                     new: the (anchor, register) whose home a
                                                      target is, by the mapping table, or None
  Anchors::required_kind                              existing: stops accepting a home target

crates/core/src/manifest.rs
  resolve_registers                                   existing: refuses a declared register named
                                                      `register`, as it refuses `path`

crates/core/src/check/references.rs
  judge_part                                          existing: a branch for the new kind, before
                                                      `table`
  fn register(out, rel, line, span, anchor, id,       new: judges a `register` reference against
              anchors, inputs)                        the anchors and the tree's listing
  fn path                                             existing: the finding on a home target, for a
                                                      literal anchor and for `*`

crates/core/src/records.rs
  inbound                                             existing: lists `register` references

crates/core/src/cli/mod.rs
  fn show                                             existing: a branch for the new kind

the respelling module that spec@plans@path-quickfixes builds
  the respelling rows                                 new rows: a home target to the new form
```

## Decided design

### The new kind

`register@<anchor>@<register>` names the home that `<anchor>` keeps `<register>` in. It is resolved
against the anchors, through `Anchors::home`, and against the tree's listing for which shape of a
heading home exists, as `path` is resolved against the survey; the entity table's ids do not gain
the register's name, `argument@register-references@a26`. A home that changes shape changes no
reference.

- **The anchor** is a Component, a declared location, or the anchor `plans`, where it carries the
  register. Examples, each resolving in this repository once the work lands:
  `register@core@design`, `register@agent-config@issue`, `register@plans@spec`,
  `register@plans@milestone`.
- **`*`** names every Component's own home of a component-scoped register, as
  `register@*@design`. It replaces the pair.
- **The id** is a register's name, a built-in register's or a declared one.
- **Findings**, each naming its repair, per `design@core@finding-names-the-repair`:
  - an anchor that names nothing: the repair lists the anchors, as for every kind;
  - an anchor that does not carry the register: the repair lists the registers it carries;
  - an id that names no register: the repair lists the registers;
  - under `*`, a register that is not component-scoped: the repair names the anchors that carry it;
  - an item register, as a `thread`, an `argument`, a `criterion` or an `acceptance`, or a milestone
    or a spec anchor: the repair names the plan document's own form, `spec@plans@<id>` or
    `milestone@plans@<id>`;
  - `elsewhere` as the anchor: the escape serves `path` alone.
- **A home that exists in neither shape** is the finding `check::tree` already raises for a
  missing home; the reference itself resolves, since the manifest declares the home.
- **The word is refused for a declared register**, in `resolve_registers`, as `path` and `planned`
  are, since a reference whose kind segment is `register` never reaches that register. It is a kind
  name, so `is_kind_name` holds of it and no anchor may wear it, per
  `design@core@kind-names-refused-as-anchors`.
- **`judge_part` sends the new kind to `fn register`**, never to `fn table`, whose repair would call
  `*` a word of the `path` kind alone.
- **A backticked `register#<x>`** is a retired-form finding, by the rule of
  `design@core@candidate-rule-and-retired-forms` for a word that is a kind.
- **`cargo klarch show register@<anchor>@<register>`** prints the path of the home as the tree
  holds it, the file or the directory, with the register's shape (`Heading`, `File` or `Directory`),
  or says that the tree holds neither shape; then every site that references it. For `*`, it prints
  each Component's home. Inbound sites compare the anchor as written, as `records::inbound` does for
  every kind.

Nearest rival: shape-agnostic resolution inside the `path` kind. It lost on
`argument@register-references@a10`.

### The finding on a home target

A `path` reference whose target is a home target is a finding. Its statement says the path is a
register's home; its repair names the new form. Which paths are home targets is the mapping table.

- **For a literal anchor**, the test runs in `anchored_target` after its refusal of a refused shape
  and of a plan document cited by its path, on the target resolved to its deepest anchor. A home
  target inside a deeper anchor gets this finding alone, naming the deepest anchor:
  `register@<deepest>@<register>`. A milestone's directory stays a plan document cited by its path,
  as today.
- **For `*`**, `Anchors::required_kind` stops accepting a home target, and the test runs on the
  component-relative path. It keeps accepting the compiled documents and the paths inside a home
  that are not home targets.
- **The kind claim is not judged on a home target**: a home target written with the wrong trailing
  slash gets this finding alone, since the new form claims no kind.
- **Existence is not asserted**: the manifest decides whether a path is a home target, and a pair
  names one shape that exists and one that does not.
- **A suffix is stripped before the test.** A home target written with a suffix gets this finding;
  its repair names the new form and adds a sentence of its own: a reference names the home, so drop
  the line number or the fragment and name the point inside it in prose. This sentence is new on the
  `path` kind: today the raw-path finding alone carries such a sentence.
- A `planned` reference is not judged by it.

Nearest rival: both forms legal. It lost on `argument@register-references@a17`.

### The respelling

`--fix` respells each finding on a home target with no suffix into the new form: the anchor written,
`*` kept as `*`, or the deepest anchor where the target sat inside one; the register whose home the
target is; the trailing slash dropped. Where a `..` or a `.` segment is normalized by the
respellings of `spec@plans@path-quickfixes` and the normalized target is a home target, the result
is the new form, written in one step. Each span is respelled alone, per D1: a pair becomes two
identical spans in one sentence, which `--fix` leaves as they are. A span with a suffix is not
respelled.

These rows change two exclusions of the respelling rule that `spec@plans@path-quickfixes` defines,
per D2, `argument@register-references@a27`: its rule applies "never to `*`", and it leaves a trailing
slash that disagrees with the target's kind unrespelled.

### The vocabulary sentence

The primer, in its source `path@agent-skills@content/PRIMER.md`, gains one sentence where it first
names a home: a register is a kind of record, its home is the file or the directory that stores its
entries, and the two words are kept apart because some rules need both, as "a slug is defined in its
register's home". No other occurrence of either word changes. The exact words are the implementing
session's.

### The migration

In this repository:

- `--fix` respells every old form of the walk, in Markdown documents and Rust comments, the source
  of the installed text under `path@agent-skills@content/` included. Its output lists every
  respelled span with its file and line.
- `cargo klarch install-agent-skills` rewrites the installed copies under .claude from that source.
- Each pair is collapsed by hand, per D1. The instrument that finds them: a search for a line that
  holds one backticked `register` span twice, such as
  `git grep -nP '(\x60register@[^\x60]+\x60).*\1'`, where `\x60` is a backtick.
- Each respelled span whose sentence illustrates the `path` grammar, rather than pointing at the
  home, is rewritten by hand to a `path` reference that is no home target, such as one naming a
  Component's rejected alternatives. The respelled sites are those `--fix` listed; the root
  CLAUDE.md and `design@core@reserved-anchors` hold such illustrations.
- The mock project `path@core@tests/projects/dirhome/`, outside the walk, is edited by hand.
- The Rust test-string fixtures that hold old forms are data, per
  `design@core@checker-source-literals-are-data`, and stay as they are, except where a test asserts
  the acceptance of a home target, which the work changes.

## Mapping tables

The home targets, over every register shape, and the respelling of each. `<a>` is the anchor as the
respelling writes it; `<dir>` is the register's `dir` in the manifest; paths are relative to the
anchor's home base.

| target | register shape | new form |
| --- | --- | --- |
| `<dir>.md` | `Heading` | `register@<a>@<register>` |
| `<dir>/` | `Heading` | `register@<a>@<register>` |
| `<dir>/README.md` | `Heading` | `register@<a>@<register>` |
| `<dir>/` | `File` | `register@<a>@<register>` |
| `<dir>/` | `Directory` | `register@<a>@<register>` |
| `<dir>/README.md` or `<dir>/index.md` | `File` or `Directory` | not a home target: stays a `path` |
| a subdocument or an entry under `<dir>/` | any | not a home target: stays a `path`, or is the plan-document finding where it is a plan document |
| any path | `Section` | not a home target |

Under the anchor `plans`, whose `path` references are admitted by
`design@core@every-path-names-its-anchor`: `specs/` respells to `register@plans@spec` and
`milestones/` to `register@plans@milestone`.

## Losing alternatives

- **Checking placeholder spans**, `thread@register-references@placeholder-spans-checked`: lost to
  the status quo, `design@core@candidate-rule-and-retired-forms`. The census found 0 defects in 354
  spans, `argument@register-references@a2`; the cost is the illustration escape and the split of a
  bundled head, `argument@register-references@a4`. The scoped form, the installed text alone, stays
  a candidate of `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`.
- **`<component>` as the generic anchor**, `thread@register-references@component-placeholder-anchor`:
  lost to `design@core@reserved-anchors`. Alone it is silent, `argument@register-references@a6`; a
  typo of it is silent, `argument@register-references@a13`; the synthesis that keeps the check buys
  only the spelling, `argument@register-references@a14`.
- **`home` as the kind word**, the agent's default in
  `thread@register-references@register-home-kind-word`: lost on the owner's choice, round 4.
- **Shape-agnostic resolution inside the `path` kind**: lost to
  `thread@register-references@register-home-reference`, on `argument@register-references@a10`.
- **Both forms legal**, the incumbent of `design@core@reserved-anchors`, which accepts a home target
  under `*`: lost to `thread@register-references@path-to-a-home-is-reported`, on
  `argument@register-references@a17`.
- **Rewording "home" to "register" where it means the record**, shape 2 of
  `thread@register-references@register-vocabulary`, the owner's own proposal: lost to shape 1, on
  `argument@register-references@a21` and the owner's word of round 5.

## Readings

None: the work reads no external specification.

## Premortem

Each cause, the thread it stresses, and its verdict. The tripwires and acceptance criteria carry the
labels they were put to the owner under, in round 5.

1. **Adopting projects meet a finding at every old form after the upgrade**, stressing
   `thread@register-references@path-to-a-home-is-reported`. Becomes an acceptance criterion, AC1,
   `acceptance@register-references@fix-respells-every-old-form`, on the owner's word.
2. **A file register's README is reported as a home**, stressing
   `thread@register-references@path-to-a-home-is-reported`. Converted into the mapping table's row
   for a file register's README, and becomes AC2,
   `acceptance@register-references@readme-by-register-shape`, on the owner's word.
3. **A reader cannot follow a `register` reference**, stressing
   `thread@register-references@register-home-reference`. Becomes AC3,
   `acceptance@register-references@show-resolves-a-register`, on the owner's word. Its tripwire, T1,
   was not recorded, on the owner's word.
4. **An adopting project has an anchor named `register`**, stressing
   `thread@register-references@register-home-kind-word`. Its tripwire, T2, was not recorded, on the
   owner's word.
5. **A defect sits in a placeholder span after all**, stressing
   `thread@register-references@placeholder-spans-checked`. Its tripwire, T3, was not recorded, on
   the owner's word.

The owner's ruling, round 6: "Keep none of the three tripwires, but keep the three AC. Proceed".

## Acceptance criteria

The fixtures follow `path@core@CLAUDE.md`: a phase-4 defect is planted in the mock `planted`
alone; `dirhome` is the conformance mock and reports nothing; `check --fix` is never run inside
`planted`, so a test of `--fix` runs on a copy, through `Sandbox` in `path@core@tests/binary.rs`.

### AC1: `--fix` respells every old form written as one span with no suffix, and a second run writes nothing `##fix-respells-every-old-form`

Guards `thread@register-references@path-to-a-home-is-reported`. Judged at step 3. The observable
that fires it: a test, on a `Sandbox` copy, plants each home-target row of the mapping table under a
literal anchor, under `*`, inside a deeper anchor, with a wrong trailing slash, and as a pair; after
`check --fix`, a planted old form remains, or a span is not the new form its row gives, or a second
run respells anything. Per D1, the pair is asserted to become two identical new-form spans, not one.
Response: the respelling is repaired before the merge; if a row cannot be respelled without
choosing, the thread reopens. This wording differs from the one ruled in round 6 in two ways: D1
narrows the pair, and the second run is added as an observable of the same claim, worded by the
agent.

### AC2: a heading register's README is a home target, a file register's is not `##readme-by-register-shape`

Guards `thread@register-references@path-to-a-home-is-reported`. Judged at step 2. The observable: a
test over `planted`; a `path` reference to the `README.md` of a heading register's directory home
is not reported, or one to the `README.md` or the `index.md` of a file register's directory is
reported. Response: the definition of a home target reopens.

### AC3: `show` resolves a `register` reference to its home `##show-resolves-a-register`

Guards `thread@register-references@register-home-reference`. Judged at step 1. The observable: a
test runs `show` on a `register` reference over a mock project with a file home and one with a
directory home, `dirhome` being the latter; the output does not print the path of the home the tree
holds and the register's shape, or omits a site that references it. Response: the thread's cost, a
reader who cannot see the file, is unmet, and the thread reopens.

## Implementation sequence

Steps 1 to 3 change what `cargo klarch commits` requires of every tree of the branch, so per
`primer@branch-commits` they land in the branch's first commit with every fix the tree needs, step
3's migration included, or the branch is squashed to one commit before its review.

1. **The kind, resolved and shown, with only this spec using it.** `Anchors::kind`, `is_kind_name`,
   `kinds_listed` and `resolve_registers` gain the word; `judge_part` sends it to `fn register`,
   which judges it with each finding of "The new kind"; `records::inbound` and `show` handle it; the
   module comment and the doc comments of `path@core@src/entity.rs` that list the kinds name it. A
   test asserts that a register's name, backticked with a hash sign before it, stays silent,
   `argument@register-references@a26`. This spec's own `register` spans must resolve. Judges AC3.
   Fails alone on: the resolution of a `register` reference, a home in each shape, `*`, `plans`, a
   location, and each refusal.
2. **The finding on a home target.** `fn home_target`; the test in `anchored_target` and in the `*`
   arm of `fn path`, with the suffix stripped; `required_kind` stops accepting a home target and its
   doc comment says so; the three tests that pin today's acceptance are changed. Judges AC2. Fails
   alone on: which paths are home targets.
3. **The respelling and the migration.** The rows added to the respellings; then "The migration".
   Judges AC1. Fails alone on: a row that does not respell, or a respelled span that is still a
   finding.
4. **The text.** The primer's vocabulary sentence; the root CLAUDE.md, whose
   `instructions@mechanical-validation` restates the kind list and the generic anchor's rule;
   `path@core@README.md`, its sections on `check`, on `show` and on the manifest's refusals;
   the rules of `path@agent-skills@CLAUDE.md` on the forms the shipped text may write; the CHANGELOG
   entries in `Next release`, then `cargo x changelog`. The entries: the new kind, under the
   checker; the finding on a home target, with a Migration entry that says to run
   `cargo klarch check --fix`, to collapse each pair and repair each suffixed span by hand, and that
   a project's root CLAUDE.md restating a home is among the sites. Which subsections, which class,
   and whether the Migration entry cites an audit axis follow
   `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`,
   read whole by the implementing session.
5. **The harvest**, per the harvest section, then the deletion of this spec.

## Order rationale

- `spec@plans@path-quickfixes` lands before this work, because its respelling machinery is what
  step 3 adds rows to, and its harvest rewrites the heads on what `--fix` may do that this work's
  respellings must fit; built first here, the work would build a second machinery or contradict
  those heads before they change.
- The kind comes before the finding, because the finding's repair names the kind, and a repair that
  names a form the checker does not resolve sends the writer nowhere.
- The finding comes before the respelling, because the respelling rewrites what the finding
  reports, and AC1 asserts that no finding is left.
- The migration comes before the text, because step 4 edits documents that step 3's run has already
  respelled, so its own edits are written in the new form.
- The text comes before the harvest, because the harvest's heads state the design as built, and the
  restatements in the root CLAUDE.md, the core README and the agent-skills CLAUDE.md are then judged
  against them by the review before the merge.

## Defaults awaiting the owner

- **D1**, bears on `acceptance@register-references@fix-respells-every-old-form`,
  `thread@register-references@path-to-a-home-is-reported` and
  `criterion@register-references@migration-cost`. AC1, as ruled in round 6, asks `--fix` to respell a
  pair "as one `register@*@<register>`". That rewrite edits bytes outside the spans, which a
  respelling never does, `argument@register-references@a25`. Default: `--fix` respells each span
  alone, so a pair becomes two identical spans in one sentence, which the check passes; a span with
  a suffix is not respelled. This repository's migration collapses each pair and repairs each
  suffixed span by hand in step 3, and the Migration entry of step 4 tells adopting projects to do
  the same. So the weighed criterion `criterion@register-references@migration-cost` is unmet for a
  pair and a suffixed span: the owner's word on D1 accepts that, naming the criterion. It narrows a
  ruling of the owner. **Ruled: approved.** The owner, in a message after the review of this spec: "All three agreed, proceed".
- **D2**, bears on `thread@register-references@path-to-a-home-is-reported`. The respelling of a home
  target under `*`, and the dropping of its trailing slash, change two exclusions of the respelling
  rule `spec@plans@path-quickfixes` defines, `argument@register-references@a27`. Default: the two
  exclusions stand for every other finding, and give way for the finding on a home target alone;
  the harvest rewrites the clauses that state them, in the heads that spec's harvest writes. It
  changes a decision of another plan. **Ruled: approved.** The owner, in a message after the review of this spec: "All three agreed, proceed".
- **D3**, bears on `tripwire@core@fix-makes-a-choice`. Its firing clause exempts only the
  respellings `spec@plans@path-quickfixes` defines, and this spec proposes others, so it fired on
  this spec's proposal. Default: its clause is narrowed in this spec's branch to except the
  respellings `spec@plans@register-references` defines, citing it, as the movement instruction of
  `skill@knowledge-architect-issue-tracking` says for a reversal recorded only at a plan document's
  harvest; that spec's harvest deletes the entry as it already says. The edit of a tripwire's firing
  clause is made in this spec's branch. **Ruled: approved.** The owner, in a message after the review of this spec: "All three agreed, proceed".

- **D4**, bears on `thread@register-references@home-reference-anchors`. The rule the owner approved
  in round 4, "any anchor that carries the named register, and whose home for it is a file or a
  directory", admits a milestone anchor for its `spec` register. Its home is the milestone's own
  directory, which `milestone@plans@<id>` already names: a second name for one document is what
  `design@core@plan-document-kinds` refuses, so that `show` lists every citation. The premise the
  ruling was given on did not include this case. Default: a milestone anchor and a spec anchor are
  not anchors of the new form, and a reference with one is a finding that names the plan document's
  form. It narrows a ruling of the owner, so it waits for the owner's word, and is built as written
  until then. The review repair first presented this narrowing as a return to the approval, which it
  was not; the commit message that said so is reworded.
- **D5**, bears on `thread@register-references@register-vocabulary`. The owner approved shape 1,
  which was proposed as one sentence in the primer that "rewrites nothing else". The decision-recording
  skill keeps a losing alternative on "a line beside the instruction" when its decision has no head,
  which would add a second line to the installed primer. Default: the primer gets the one sentence
  with its reason, and the losing shape 2 is kept in the message of the commit that writes it, where
  a session that reopens the sentence finds it with `git log -S`. It departs from that skill's
  placement on the owner's scope, so it waits for the owner's word, and is built as written until
  then.

## Harvest

At the landing of step 5, under `skill@knowledge-architect-decision-recording` for the decisions and
the losing alternatives, and `skill@knowledge-architect-issue-tracking` for the issue:

| item | lands in | when |
| --- | --- | --- |
| `thread@register-references@register-home-reference`, with `thread@register-references@home-reference-anchors` as its parameter and `thread@register-references@register-home-kind-word` as its word, the owner's words its ground for the word | a new head of the core design home, slug `register-home-reference` | the harvest commit |
| `thread@register-references@path-to-a-home-is-reported`, the finding alone | a new head of the core design home, slug `path-to-a-home-is-reported`, its title stating the finding | the harvest commit |
| the respelling of a home target, with D2 as ruled | the heads `spec@plans@path-quickfixes` wrote on what `--fix` may do: the rows as members, the two exclusions rewritten | the harvest commit |
| the kind list | the rewrites of `design@core@one-entity-table` and `design@core@harness-kinds` | the harvest commit |
| the ways the new kind resolves to nothing | the rewrite of `design@core@an-unresolved-reference-is-reported` | the harvest commit |
| the register name the manifest refuses | the rewrite of `design@core@registers-are-declared` | the harvest commit |
| `*` under the new kind, and the generic `path` form's narrowed acceptance | the rewrite of `design@core@reserved-anchors` | the harvest commit |
| the forms the shipped text may write | the rewrite of `design@agent-skills@shipped-text-cites-no-entry` | the harvest commit |
| `thread@register-references@register-vocabulary` | the primer's one sentence with its reason, written at step 4; per D5, the losing shape 2 is kept in the message of the commit that writes the sentence, and no second line goes into the primer; the harvest judges by the entry tests whether the absence of a rewording earns a head | step 4, and the harvest commit |
| `thread@register-references@placeholder-spans-checked` | the scoped form, into `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`; a rejected alternative of the core if the recording tests admit it | the harvest commit |
| `thread@register-references@component-placeholder-anchor`, `home` as the kind word, and both forms legal | rejected alternatives of the core, each if the recording tests admit it | the harvest commit |
| this spec | deleted | the harvest commit |

## Later consequences

- `issue@core@the-documents-do-not-render-as-a-site`: the pass it plans resolves one more kind, a
  `register` reference, to a link to the home it names.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the check it plans admits the new
  form as one every installing project holds.
- `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`: the view it plans may
  expose `fn home_target`.
