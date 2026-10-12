# A register's home is cited `register@<anchor>@<register>`, and a `path` reference to a home is a finding that `--fix` respells

## Status and audience

This spec is the plan of the work that adds a reference kind naming a register's home. The work
does four things:

- a reference `register@<anchor>@<register>` resolves, through the manifest, to the home that
  anchor keeps the register in, whichever of its two shapes the home has;
- a `path` reference whose target is exactly a register's home is a finding, whose repair is the
  `register` form;
- `cargo klarch check --fix` respells that finding;
- the primer defines the two words "register" and "home" once, beside each other.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. One default awaits it, D1, under "Defaults awaiting the owner".
- **The spec lands before its work, in a pull request of its own.** Its work starts only once
  `spec@plans@path-quickfixes` has landed, since it reuses that spec's respelling machinery; see
  "What the work is" and the order rationale.
- **The work starts with the design audit** of
  `skill@knowledge-architect-planning@working-a-slice` point 2, since it does not start in the session
  where the discussion converged, and since `spec@plans@path-quickfixes` will have landed between.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/c074f629-1136-4498-aff3-f53ca6bff772.jsonl`.
  Find the file by its opening message, not by its name: the discussion begins at the owner's
  message that opens "How does the checker handles segmented references with placeholders", and
  ends at the owner's message "One spec." It holds 7 owner messages, called rounds 1 to 7 below,
  each followed by one agent reply.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **a register**: one kind of recorded thing together with its storage, as
  `path@core@src/manifest.rs` declares it, the struct `Register`: `design`, `goal`, `tripwire`,
  `issue`, `spec`, `milestone`, the four item registers, and any a project declares.
- **a home**: where an anchor keeps one register's entries, the struct `Home` of
  `path@core@src/entity.rs`, computed by `Anchor::home_of`. A heading register's home is the file
  `<home base>/<dir>.md` or the directory `<home base>/<dir>/`, never both, per
  `design@core@heading-register-two-shapes`; a file register's home is the directory
  `<home base>/<dir>/`; a milestone's `spec` register and the plans anchor's two registers have the
  homes `home_of` gives them.
- **the head of a directory home**: the `README.md` of a heading register's directory home, which
  is an introduction and an index of its subdocuments, per
  `design@core@heading-register-two-shapes`. The `README.md` of a file register's directory is not
  a head in that sense: it is a hand-written guide, per
  `design@core@a-file-register-is-a-directory-of-entries`.
- **a home target**: a path that is exactly a home, or the head of a directory home. A path into a
  directory home that is not its head, such as one subdocument, an entry file or a generated
  `index.md`, is not a home target.
- **the item registers**: `thread`, `argument`, `criterion` and `acceptance`, whose shape is
  `Shape::Section` and whose home is a plan's own documents, per
  `design@core@items-as-section-registers`.
- **a span**, **a respelling**, **`--fix`**: as `spec@plans@path-quickfixes` defines them in its
  Names, once it has landed. A respelling rewrites one span into another span naming the same
  entity, and no byte outside the span.
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
- `Anchors::home(anchor, kind)` in `path@core@src/entity.rs` already returns the home an anchor
  keeps one register in, or `None` where the anchor carries none.
- A `path` reference resolves against the tree, in `fn path` of
  `path@core@src/check/references.rs`. Its generic anchor `*` accepts a required document name in
  any of its shapes, from `Anchors::required_kind` in `path@core@src/entity.rs`: the compiled
  documents, every component-scoped heading register's file, directory and README, and every
  component-scoped file register's directory, README and index. A literal anchor's target is
  asserted to exist with its claimed kind, by `fn assert_target`.
- `resolve_registers` in `path@core@src/manifest.rs` refuses a declared register named like a
  harness kind, `path` or `planned`, since a reference whose kind segment is that word never
  reaches the register.
- `cargo klarch show` (`fn show` in `path@core@src/cli/mod.rs`) resolves a reference and lists
  every site that references it, through `records::inbound` in `path@core@src/records.rs`, which
  compares the anchor as written.
- `cargo klarch check --fix` writes only the generated files and the installed agent files,
  per the doc comment of `CheckArgs::fix` in `path@core@src/cli/mod.rs`.
  `spec@plans@path-quickfixes` plans the respellings of hand-written documents, and it is not
  built. Its respelling rule applies "never to `*` or `elsewhere`".
- The installed text, under `path@agent-skills@content/`, and this repository's own documents name
  register homes with the old form, much of it as pairs. The census at the design discussion,
  over `git ls-files '*.md' '*.rs'`: 128 `path` spans with the generic anchor `*`, of which about
  76 name a register home. Re-take it with `grep -ohE` over that listing, for backticked `path`
  spans whose anchor is `*`, then for those whose path is a heading register's file or directory or
  the issue directory under docs/.

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
  `design@core@planned-path-form`. A home is a home of a declared anchor, which exists.
- **A rendered link for the new form.** `issue@core@the-documents-do-not-render-as-a-site` owns
  the rewrite of every reference into a link; this work gives that pass one more kind to resolve.

## What is already decided

**The decisions the design rests on and does not argue again:**

- `design@core@one-entity-table`: one table, one resolver, every kind through it.
- `design@core@heading-register-two-shapes`: a heading register's home has two shapes, never both.
- `design@core@a-file-register-is-a-directory-of-entries`: a file register's home is a directory.
- `design@core@kind-names-refused-as-anchors`: no anchor wears a kind's name. It admits the new
  kind word as a member without change.
- `design@core@candidate-rule-and-retired-forms`: a span whose head is a kind is a candidate; every
  kind but `path`, `planned` and the harness kinds takes exactly three non-empty segments. The new
  kind is such a kind, so this head is **not** edited, and the split that
  `issue@core@four-heads-bundle-decisions-with-many-citing-sites` owes at its next edit is not
  triggered by this work.
- `design@core@trailing-slash-claims-directory`: a `path` claims its target's kind by its trailing
  slash. The new form claims nothing of the kind, since the manifest knows it.
- `design@core@plans-is-a-reserved-anchor` and `design@core@plan-document-kinds`: the anchor
  `plans` and the citation of a plan document by its kind.
- `design@core@items-as-section-registers` and `design@core@plan-item-scope`: an item register's
  home is a plan's documents, and an item is cited from inside its plan only.
- `goal@core@declared-instructions-are-checked` and `goal@core@relocation-is-one-manifest-edit`,
  from which the two binding criteria derive.

**The decisions the work rewrites, and every text that references each**, as `cargo klarch show`
listed it at the commit this spec was written on. Re-run `show` at the design audit: the lists grow.

- `design@core@one-entity-table` is rewritten at the harvest: its kind list gains `register`.
  Referenced at: the agent-skills design home, line 951; the core design home, lines 931 and 1960;
  `path@core@src/entity.rs`, line 6. The module comment at `path@core@src/entity.rs` line 8
  restates the kind list, and step 1 updates it; the root CLAUDE.md, `instructions@mechanical-validation`,
  restates it, and step 4 updates it. The other sites cite the head for the table, not for the
  list, and the harvest's review judges each.
- `design@core@reserved-anchors` is rewritten at the harvest: `*` gains a meaning under the
  `register` kind, every Component's own register, and the generic `path` form no longer accepts a
  home target. Referenced at: the root CLAUDE.md, lines 135 and 155, which step 4 updates; the
  agent-skills design home, lines 95, 936 and 969; the core rejected alternatives, lines 143 and
  155; `path@core@src/check/references.rs`, lines 547 and 1917, which steps 1 and 2 update. The
  harvest's review judges each site against the rewritten head.
- `design@agent-skills@shipped-text-cites-no-entry` is rewritten at the harvest: the forms the
  shipped text may write gain `register@*@<register>` and `register@plans@<register>`, and its
  sentence "where the text names both shapes, both are references" goes, since the text then names
  one register. The owner's quoted ground, that the shipped text "promotes the same kind of syntax as
  what the checker enforces", holds of the new form. Referenced at:
  `path@agent-config@skills/klarch-release/SKILL.md` line 66; `path@agent-skills@CLAUDE.md` lines
  24 and 46, which step 4 updates; the agent-skills design home, line 2280;
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`, lines 21 and 35, which the
  harvest edits.
- The heads that `spec@plans@path-quickfixes` writes at its harvest on what `--fix` may do, among
  them `design@core@safe-fix-definition` and `design@core@fix-scope` as rewritten there, gain the
  respelling rows of this work as members of the respelling rule. The design audit reads those heads
  as they stand then.

**The standing entries the discussion's search returned, and their outcome.** The search ran at the
grounding of the discussion, by three searchers over the 121 issues and tripwires of the tree.

| entry | outcome |
| --- | --- |
| `tripwire@core@candidate-rule-silence` | Not fired: a placeholder span is an illustration, not a span meant as a reference. The new kind is a kind, so the candidate rule admits it unchanged. |
| `tripwire@agent-skills@plain-text-pointer-found` | Bears on nothing: the work adds a checked form and moves no pointer into plain text. |
| `tripwire@core@planned-path-outside-plans` | Bears on nothing: `planned` and the escape anchor are unchanged. |
| `tripwire@core@extension-defines-a-kind` | Not fired: the kind is the core's, not an extension's. |
| `tripwire@core@checker-source-literals-are-data-self-location` | Bears on nothing: the new findings are raised over documents, and the checker's own string literals stay data. |
| `tripwire@core@fix-makes-a-choice` | Bears on the order: its firing clause exempts only the respellings `spec@plans@path-quickfixes` defines, and that spec's harvest deletes the entry. This work starts after that harvest. If the entry still stands at the design audit, the work's respelling fires it. |
| `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` | Bears on nothing: the work does not touch an empty head, and does not edit the candidate rule. |
| `issue@core@cross-project-references` | Bears on nothing: the new form names an anchor of the project that holds it. |
| `issue@core@four-heads-bundle-decisions-with-many-citing-sites` | Bears on the scope: the work edits neither `design@core@candidate-rule-and-retired-forms` nor `design@core@every-path-names-its-anchor`. The finding on a home target is a head of its own, written at the harvest. If the design audit finds that one of the two must be edited, the split that entry owes is part of the work. |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | Bears on nothing: the harness kinds are unchanged. |
| `issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` | Bears on step 1: `Entities::defines_id` decides which backticked `#<id>` is the retired form. If register instances enter the entity table with their register's name as id, every register name backticked with a hash sign before it becomes a retired-form finding. Step 1 keeps the register kind out of `defines_id`, and a test asserts it. |
| `issue@core@finding-texts-are-not-audited-for-a-needless-cause` | Bears on the new findings' texts, which follow `design@core@finding-names-the-repair` from the start. It audits none of the existing ones. |
| `issue@core@the-documents-do-not-render-as-a-site` | Bears on a cost of `thread@register-references@register-home-reference`: a reader of a document outside the tool does not see the file a `register` reference resolves to. The pass that issue plans resolves it to a link. No tripwire watches it, on the owner's ruling. |
| `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported` | Bears on nothing: the work does not change which bare mentions are read. |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing` | Bears on nothing: its re-entry point, the next change to `design@core@candidate-rule-and-retired-forms`, is not met, since the work does not edit that head. |
| `issue@core@a-checker-only-project-carries-the-workflow-skeleton` | Bears on nothing: the set of required homes is unchanged; only how they are cited changes. |
| `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply` | Bears on nothing: the finding it names is not touched. |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` | Bears on the new finding's repair: a home target written with a line suffix or a fragment is not respelled, as `spec@plans@path-quickfixes` excludes such spans, and the repair text of this work's finding names the new form only. |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | Bears on nothing: no tripwire is recorded by this work. |
| `issue@agent-skills@an-audit-report-cites-the-heads-its-own-pass-removes` | Bears on nothing: the work removes no head. |
| `issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` | Bears on the ruled-out thread: checking placeholders would have removed the escape that issue records. The ruling keeps the escape. |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | Bears on the harvest: the scoped form of placeholder checking is added to it as a candidate shape, and its list of the forms the shipped text may write gains the new form. |
| `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` | Bears on a cost: the new kind word becomes a name no anchor may wear. The question that entry asks, which ground the refusal stands on, is not changed by one more member. |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | Bears on the changelog: an adopting project's root CLAUDE.md that restates a home with the old form gets a finding after the upgrade, which `--fix` respells. The Migration entry of step 4 names it. |

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
on one meaning". Satisfaction: met. `*` is kept, and a home has one spelling since its `path` form
is refused.

### A document can still illustrate an invalid form `##illustration-escape`

Weighed. Derived from the core design home and the issue entries, which illustrate broken shapes on
purpose, and from the owner's ruling recorded in
`issue@agent-skills@the-place-of-the-owner-s-quotes-in-the-workflow` that the checker gives
quotations no other escape. Satisfaction: met. Placeholder spans stay unrecorded,
`thread@register-references@placeholder-spans-checked`.

### The cost of migrating the spans and of splitting heads `##migration-cost`

Weighed. Measured at the discussion: 354 placeholder spans; about 70 `path` references with a
literal anchor to a register home, and about 76 `*` spans naming one, in this repository; the same
forms in every adopting project's own documents. Satisfaction: met by
`acceptance@register-references@fix-respells-every-old-form`. `--fix` respells every old form, so
no project migrates a span by hand. D1 narrows what it does with a pair.

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
Decided design. Harvest: a new head in the core design home, and the rewrite of
`design@core@one-entity-table`.

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
target" and "The respelling" under Decided design. Harvest: a new head in the core design home, and
the rewrite of `design@core@reserved-anchors`.

### The anchors that admit the kind `##home-reference-anchors`

Proposed by the agent, round 3, `argument@register-references@a18`. Final state: approved. The
owner, round 4: "home-reference-anchors: agreed." Shape: "The new kind". Harvest: in the head of
`thread@register-references@register-home-reference`, as the rule's parameter, and in the rewrite of
`design@core@reserved-anchors`.

### Both words stay, defined once beside each other in the primer `##register-vocabulary`

Proposed by the owner, round 4: "I think mentions that uses home such as "design home" should all be
reworded to register (tripwires, issues, design, goals...). Home is just another vocabulary for
something that is already named register in the code, AFAIK." The agent put the corrected premise to
the owner, `argument@register-references@a20`, and argued two shapes,
`argument@register-references@a21`. Final state: approved, shape 1. The owner, round 5:
"register-vocabulary: go with shape 1, I think this is the right balance". Shape: "The vocabulary
sentence". Harvest: the primer's sentence, in the installed text; no design head, since one text
states the instruction and its reason lives beside it.

### Placeholder spans are checked `##placeholder-spans-checked`

Proposed by the owner, round 1: "I was considering giving those shapes their own checks. They'd need
to respect the correct placeholder names depending on kind (and to use a valid kind)." Final state:
ruled out. Moved by `argument@register-references@a1` to `argument@register-references@a5`. The
owner, round 4: "placeholder-spans-checked: agreed with you position, rule this out." The position
agreed with included the scoped form going to
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. Shape: "Losing alternatives".
Harvest: the scoped form into that issue; a rejected alternative if the recording tests admit it.

### `<component>` as the generic anchor `##component-placeholder-anchor`

Proposed by the owner, round 1: a placeholder anchor, as `path@<component>@docs/<dir>.md`, in
place of the generic anchor `*`, in the owner's words "would be nice too, I believe. It might allow
relaxing the current rules without losing conformance." The owner's message wrote the two spans with
a literal path; they are rewritten here as placeholders, since the checker reads a quotation as it
reads any text. Reframed in round 2: "the <component> placeholder is cleared
that * IMO. It could replace the * shape." Moved by `argument@register-references@a6`,
`argument@register-references@a12`, `argument@register-references@a13` and
`argument@register-references@a14`. Final state: ruled out. The owner, round 3: "Let's keep the *
shape. I still find it a bit confusing, but the change is not worth it as you say." Shape: "Losing
alternatives". Harvest: a rejected alternative if the recording tests admit it.

## Arguments

### Placeholder spans are not recorded today `##a1`

Round 1, the agent; bears on `thread@register-references@placeholder-spans-checked` and
`thread@register-references@component-placeholder-anchor`. "The span regex excludes any span that
holds `<` or `>` (the regex `AT_SPAN` of `path@core@src/scan.rs`), so the span is never recorded." The silence is
deliberate: `design@core@candidate-rule-and-retired-forms` says "an illustration writes a
placeholder in angle brackets, which the tokenizer does not record".

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
names counted: `<anchor>` 60, `<name>` 30, `<milestone>` 20, `<component>` 14, `<parent-anchor>`
5, `<c>` 4. "The names carry information a fixed vocabulary would erase": `spec@<milestone>@<slice>`
says the anchor is a milestone, and `path@<parent-anchor>@<dir>/` states how an anchor's own
directory is named. "Enforcing names means rewriting most of the 354 spans and buys no detected
defect."

### Checking placeholders costs the escape and a head split `##a4`

Round 1, the agent; bears on `thread@register-references@placeholder-spans-checked`. "It costs the
2 illustrations a rewrite"; "It changes the candidate rule, which triggers the split of a bundled
head with 23 citing sites"; "It narrows the escape the quotation issue records."

### A span with no literal segment stays silent `##a5`

Round 1, the agent, on the owner's question about `<kind>@...@...`; bears on
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
`goal@knowledge-architect@any-project-can-adopt-it`."

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
Markdown, 14 spans name the rejected alternatives, 2 a README, 1 a CLAUDE.md and 1 a fixture path;
in Rust, 18, all test fixtures. "So in documents the replacement touches about 18 spans. [...] The
argument is about the rule, not the cost."

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
register" 2." The owner chose `register` over it in round 4.

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
thing, `home` is "**where** a register's entries are stored". Counted in `path@core@src/`: the
identifier `register` 542 times, `home` 334 times. "A test name shows that the two must stay
distinct: `two_registers_of_one_anchor_may_not_share_a_home`." And "home" is wider than registers:
the knowledge table's "every durable decision has exactly one home" names a README, a CLAUDE.md, a
skill or a comment.

### A wholesale rewording loses sentences `##a21`

Round 4, the agent; bears on `thread@register-references@register-vocabulary`. "A slug is defined in
its register's home, never in the README of a directory-shaped home" and "Two registers of one
anchor may not share a home" cannot be written with one word. Shape 1 defines both words once; shape
2 rewords where the phrase means the record, "a judgement at each of about 330 register-related
sites. No check can verify those judgements".

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
old form, so no project migrates by hand."

### `--fix` respells nothing hand-written today `##a24`

Round 6, the agent; bears on the order of the work. "AC1 needs `--fix` to respell a reference, and
today `--fix` writes only generated and installed files (the doc comment of `CheckArgs::fix` in `path@core@src/cli/mod.rs`).
`spec@plans@path-quickfixes` builds that respelling machinery, and it rewrites the design heads on
what `--fix` may do."

### Collapsing a pair is no respelling `##a25`

At the assembly, the agent; bears on `acceptance@register-references@fix-respells-every-old-form`.
AC1, as ruled, has `--fix` respell "a `*` pair (the file form and the directory form of one register,
in one sentence) as one `register@*@<register>`". That edits bytes outside the two spans, the word
"or" between them and the second span, and `spec@plans@path-quickfixes` defines a respelling as
changing no byte outside its span. Each span alone respells to the same new form, so a pair becomes
two identical spans in one sentence, which passes the check. D1 carries the default.

### Register instances must stay out of the retired-form lookup `##a26`

At the assembly, the agent; bears on `thread@register-references@register-home-reference`.
`issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` reads that
`Entities::defines_id` is true for an id of any register whose kind is not a harness kind. Were a
register instance an entity with its register's name as id, every register name backticked with a hash sign
before it would become a finding.

## New names, in one place

An illustration of the shapes, not authority; the design audit and the implementing session decide
the final names.

```text
crates/core/src/entity.rs
  const REGISTER_KIND: &str = "register"              new: the kind naming a register's home
  Kind::register(), Kind::is_register()               new
  Anchors::kind                                       existing: gains the word
  Anchors::kinds_listed, is_kind_name                 existing: gain the word
  Anchors::home(anchor, kind)                         existing: the resolution of the new form
  fn home_target(path, anchors)                       new: the (anchor, register) whose home a
                                                      path is, or None; reads Home and its head

crates/core/src/manifest.rs
  resolve_registers                                   existing: refuses a declared register named
                                                      `register`, as it refuses `path`

crates/core/src/check/references.rs
  fn register(out, rel, line, span, anchor, id,       new: judges a `register` reference
              anchors)
  fn path                                             existing: raises the finding on a home target,
                                                      for a literal anchor and for `*`

crates/core/src/records.rs
  inbound                                             existing: lists `register` references

crates/core/src/cli/mod.rs
  fn show                                             existing: a branch for the `register` kind

crates/core/src/fix.rs, once spec@plans@path-quickfixes has landed it
  the respelling rows                                 new rows: a home target to the new form
```

## Decided design

### The new kind

`register@<anchor>@<register>` names the home that `<anchor>` keeps `<register>` in. It resolves
through `Anchors::home`, so it names the home in whichever shape it has, and a home that changes
shape changes no reference.

- **The anchor** is any anchor that carries the register, where the register's home is a file or a
  directory: a Component, a declared location, the anchor `plans`, a milestone for its `spec`
  register. Examples, each resolving in this repository once the work lands:
  `register@core@design`, `register@agent-config@issue`, `register@plans@spec`,
  `register@plans@milestone`.
- **`*`** names every Component's own home of a component-scoped register, as
  `register@*@design`. It replaces the pair. Under `*`, a register that is not component-scoped is
  a finding.
- **The id** is a register's name, a built-in register's or a declared one.
- **Findings**, each naming its repair, per `design@core@finding-names-the-repair`:
  - an anchor that names nothing: the repair lists the anchors, as for every kind;
  - an anchor that does not carry the register: the repair lists the registers it carries;
  - an id that names no register: the repair lists the registers;
  - an item register, as a `thread`, an `argument`, a `criterion` or an `acceptance`: the repair
    names the plan document's own form, `spec@plans@<id>` or `milestone@plans@<id>`, since the
    item register's home is that document;
  - `elsewhere` as the anchor: the escape serves `path` alone.
- **The word is refused for a declared register**, in `resolve_registers`, as `path` and `planned`
  are, since a reference whose kind segment is `register` never reaches that register. It is a kind
  name, so `is_kind_name` holds of it and no anchor may wear it, per
  `design@core@kind-names-refused-as-anchors`.
- **The entity table** resolves the new form; it does not add the register's name to the ids that
  `Entities::defines_id` reports, `argument@register-references@a26`.
- **`cargo klarch show register@<anchor>@<register>`** prints the home's path, its shape, and every
  site that references it; for `*`, every Component's home. Inbound sites compare the anchor as
  written, as `records::inbound` does for every kind.

Nearest rival: shape-agnostic resolution inside the `path` kind. It lost on
`argument@register-references@a10`.

### The finding on a home target

A `path` reference whose target is a home target is a finding. Its statement says the path is a
register's home; its repair names the new form for that anchor and register.

- It is raised for a literal anchor and for `*`. For `*`, `Anchors::required_kind` stops accepting
  a home target, and keeps accepting the compiled documents and the paths inside a home that are not
  home targets: a file register's `README.md` and `index.md`.
- **A home target** is: the file of a heading register's single-file home; the directory of any
  directory home; the head of a heading register's directory home. Not a home target: a
  subdocument, an entry, a file register's `README.md` or generated `index.md`.
- It is raised whether or not the path exists, since the manifest decides whether it is a home: a
  pair names one shape that exists and one that does not.
- A home target written with a line suffix or a fragment is a finding of this kind too, and its
  repair adds the existing sentence on dropping the suffix.
- A `planned` reference is not judged by it.

Nearest rival: both forms legal. It lost on `argument@register-references@a17`.

### The respelling

`--fix` respells each finding on a home target into the new form: `<anchor>` kept as written,
`*` included, and `<register>` the register whose home the target is. The trailing slash is dropped,
since the new form claims no kind. Each span is respelled alone, per D1: a pair becomes two
identical spans in one sentence, which `--fix` leaves as they are. A span with a line suffix or a
fragment is not respelled, as `spec@plans@path-quickfixes` excludes such spans.

These are rows added to the respellings of `spec@plans@path-quickfixes`. That spec applies a
respelling "never to `*` or `elsewhere`": the `*` row here is a member its rule admits, since the
span names one entity, every Component's home of one register, and the rewrite names the same.

### The vocabulary sentence

The primer, in its source `path@agent-skills@content/PRIMER.md`, gains one sentence where it first
names a home: a register is a kind of record, and its home is the file or the directory that stores
its entries. No other occurrence of either word changes. The exact words are the implementing
session's.

### The migration

In this repository, the work's own commits respell every old form, by running `--fix`, and collapse
each pair to one span by hand, per D1. That covers the installed text's source under
`path@agent-skills@content/`, and the installed copies under .claude are rewritten by the install.

## Mapping tables

The respelling, over every home target. `<a>` is the anchor as written, `*` included; `<dir>` is the
register's `dir` in the manifest.

| target, relative to the anchor's home base | register shape | new form |
| --- | --- | --- |
| `<dir>.md` | heading | `register@<a>@<register>` |
| `<dir>/` | heading | `register@<a>@<register>` |
| `<dir>/README.md` | heading | `register@<a>@<register>` |
| `<dir>/` | file | `register@<a>@<register>` |
| `<dir>/README.md` | file | not a home target: stays a `path` |
| `<dir>/index.md` | file | not a home target: stays a `path` |
| a subdocument or an entry under `<dir>/` | either | not a home target: stays a `path` |

For the anchor `plans`, whose `path` references are legal under `design@core@plans-is-a-reserved-anchor`:
`specs/` respells to `register@plans@spec` and `milestones/` to `register@plans@milestone`.

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
- **Both forms legal**: lost to `thread@register-references@path-to-a-home-is-reported`, on
  `argument@register-references@a17`.
- **Rewording "home" to "register" where it means the record**, shape 2 of
  `thread@register-references@register-vocabulary`: lost to shape 1, on
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
   `thread@register-references@path-to-a-home-is-reported`. Converted into the clause "Not a home
   target: a file register's README.md" of "The finding on a home target", and becomes AC2,
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

### AC1: `--fix` respells every old form, and a second run writes nothing `##fix-respells-every-old-form`

Guards `thread@register-references@path-to-a-home-is-reported`. Judged at step 3. The observable that
fires it: a test plants each row of the mapping table under a literal anchor and under `*`, and a
pair; after `check --fix`, a planted old form remains, or a span is not the new form its row gives,
or a second run respells anything. Per D1, the pair is asserted to become two identical new-form
spans, not one. Response: the respelling is repaired before the merge; if a row cannot be respelled
without choosing, the thread reopens. The wording of this criterion differs from the one ruled in
round 6 by D1.

### AC2: a heading register's README is a home target, a file register's is not `##readme-by-register-shape`

Guards `thread@register-references@path-to-a-home-is-reported`. Judged at step 2. The observable: a
test over a mock project; a `path` reference to the `README.md` of a heading register's directory
home is not reported, or one to the `README.md` or the `index.md` of a file register's directory is
reported. Response: the definition of a home target reopens.

### AC3: `show` resolves a `register` reference to its home `##show-resolves-a-register`

Guards `thread@register-references@register-home-reference`. Judged at step 1. The observable: a
test runs `show` on a `register` reference over a mock project with a file home and one with a
directory home; the output does not print the home's path and shape, or omits a site that
references it. Response: the thread's cost, a reader who cannot see the file, is unmet, and the
thread reopens.

## Implementation sequence

Steps 1 and 2 change what `cargo klarch commits` requires of every tree of the branch, so per
`primer@branch-commits` they land in the branch's first commit with every fix the tree needs, step
3's migration included, or the branch is squashed to one commit before its review.

1. **The kind, resolved and shown, with no document using it.** `Anchors::kind`, `is_kind_name`,
   `kinds_listed` and `resolve_registers` gain the word; `fn register` judges the new form with
   each finding of "The new kind"; `records::inbound` and `show` handle it; the module comment of
   `path@core@src/entity.rs` names the kind. A test asserts that the register name `design`, backticked with a hash
   sign before it, stays silent, `argument@register-references@a26`. Judges AC3. Fails alone on: the resolution of a
   `register` reference, a home in each shape, `*`, `plans`, a location, and each refusal.
2. **The finding on a home target.** `fn home_target`; `fn path` raises the finding for a literal
   anchor and for `*`; `required_kind` stops accepting a home target. Judges AC2. Fails alone on:
   which paths are home targets.
3. **The respelling and the migration.** The rows added to the respellings of
   `spec@plans@path-quickfixes`; `--fix` run over this repository; each pair collapsed by hand, per
   D1; the install run so the installed copies match. Judges AC1. Fails alone on: a row that does
   not respell, or a respelled span that is still a finding.
4. **The text.** The primer's vocabulary sentence; the root CLAUDE.md, whose
   `instructions@mechanical-validation` restates the kind list and the generic anchor's rule; the
   rules of `path@agent-skills@CLAUDE.md` on the forms the shipped text may write; the CHANGELOG
   entries in `Next release`, then `cargo x changelog`. The entries: the new kind, under the
   checker; the finding on a home target, with a Migration entry that says to run
   `cargo klarch check --fix` and to collapse each pair by hand, and that a project's root CLAUDE.md
   restating a home is among the sites. Which subsections and which class follow
   `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`,
   read whole by the implementing session.
5. **The harvest**, per the harvest section, then the deletion of this spec.

## Order rationale

- The kind comes before the finding, because the finding's repair names the kind, and a repair that
  names a form the checker does not resolve sends the writer nowhere.
- The finding comes before the respelling, because the respelling rewrites what the finding
  reports, and AC1 asserts that no finding is left.
- The migration comes before the text, because step 4 edits documents that step 3's run has already
  respelled, so its own edits are written in the new form.
- The text comes before the harvest, because the harvest's heads state the design as built, and the
  restatements in the root CLAUDE.md and the agent-skills CLAUDE.md are then judged against them by
  the review before the merge.

## Defaults awaiting the owner

- **D1**, bears on `acceptance@register-references@fix-respells-every-old-form` and
  `thread@register-references@path-to-a-home-is-reported`. AC1, as ruled in round 6, asks `--fix` to
  respell a pair "as one `register@*@<register>`". That rewrite edits bytes outside the spans, which
  `spec@plans@path-quickfixes` defines a respelling never to do, `argument@register-references@a25`.
  Default: `--fix` respells each span alone, so a pair becomes two identical spans in one sentence,
  which the check passes. This repository's migration collapses each pair by hand in step 3, and
  the Migration entry of step 4 tells adopting projects to do the same. It narrows a ruling of the
  owner, so it waits for the owner's word, and is built as written until then.

## Harvest

At the landing of step 5, under `skill@knowledge-architect-decision-recording` for the decisions and
the losing alternatives, and `skill@knowledge-architect-issue-tracking` for the issue:

| item | lands in | when |
| --- | --- | --- |
| `thread@register-references@register-home-reference`, with `thread@register-references@home-reference-anchors` as its parameter and `thread@register-references@register-home-kind-word` as its word, the owner's words its ground for the word | a new head of the core design home, slug `register-home-reference` | the harvest commit |
| `thread@register-references@path-to-a-home-is-reported` | a new head of the core design home, slug `path-to-a-home-is-reported` | the harvest commit |
| the kind list | the rewrite of `design@core@one-entity-table` | the harvest commit |
| `*` under the new kind, and the generic `path` form's narrowed acceptance | the rewrite of `design@core@reserved-anchors` | the harvest commit |
| the respelling rows | the heads `spec@plans@path-quickfixes` wrote on what `--fix` may do, as members of its respelling rule | the harvest commit |
| the forms the shipped text may write | the rewrite of `design@agent-skills@shipped-text-cites-no-entry` | the harvest commit |
| `thread@register-references@register-vocabulary` | the primer's sentence, written at step 4; no head | step 4 |
| `thread@register-references@placeholder-spans-checked` | the scoped form, into `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`; a rejected alternative of the core if the recording tests admit it | the harvest commit |
| `thread@register-references@component-placeholder-anchor`, and `home` as the kind word | rejected alternatives of the core, each if the recording tests admit it | the harvest commit |
| this spec | deleted | the harvest commit |

## Later consequences

- `issue@core@the-documents-do-not-render-as-a-site`: the pass it plans resolves one more kind, a
  `register` reference, to a link to the home it names.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the check it plans admits the new
  form as one every installing project holds.
