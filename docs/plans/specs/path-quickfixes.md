# `check --fix` respells a reference whose target its span already names, a path citation of a skill or an agent is refused, and an unanchored path's repair asks which candidate was meant

## Status and audience

This spec is the plan of the work that lets `cargo klarch check --fix` edit hand-written documents,
within one limit: a fix rewrites the spelling of a reference whose target its span already names,
and never chooses a target the span does not name. The work does four things:

- `check --fix` gains respellings: six kinds of reference finding whose repair is determined by
  the span and the tree are rewritten in place, and each rewrite is printed;
- a `path` citation of a skill's directory, of its `SKILL.md`, or of an agent's file is a new
  finding, and `--fix` respells it as `skill@<name>` or `agent@<name>`;
- the finding on a backticked path that names no anchor asks which candidate of the tree was
  meant, listing every candidate, and writes nothing;
- the design heads on what `--fix` may do are rewritten to state the principle, "a fix never
  rewrites what a writer meant", with the respelling as its test.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec lands before its work, in a pull request of its own.** The owner, round 3: "we won't
  be implementing it right now, we'll just merge the branch with the standing spec after the
  reviews." Step 3 changes what `cargo klarch commits` judges in a commit's tree, so the spec must
  land first in any case, per `skill@knowledge-architect-planning@cutting-steps-and-slices`.
- **The work starts with the design audit** of `skill@knowledge-architect-planning@working-a-slice`
  point 2, since it does not start in the session where the discussion converged. The owner, round
  3, asks that the parked thread `thread@path-quickfixes@same-anchor-inference` be discussed again
  at that audit.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/e2ab040a-95d3-4ad6-bcf7-65e563887a82.jsonl`.
  The discussion begins at the owner's message that opens "I would like to provide quickfix (under
  --fix) for paths", and ends at the owner's message "Keep all tripwires and AC, proceed." It holds
  4 owner messages, called rounds 1 to 4 below, and 3 agent replies, one after each of rounds 1 to
  3. The agent's replies carry the headings "Round 1", "Round 2" and "Round 3" in the transcript;
  each is called here "the reply to round 1", "the reply to round 2" and "the reply to round 3".

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **a span**: a backticked span of one line, as the scanner of `path@core@src/scan.rs` records it.
- **a respelling**: a rewrite of one span into another span that names the same entity: the
  target the original span already resolves to under the checker's own rules, or the entity a
  recorded head declares the span to mean. The bytes outside the span do not change. Defined by
  `thread@path-quickfixes@fix-admission`.
- **an inference**: a rewrite that chooses an entity the span does not name, from what the tree
  holds. `--fix` never applies one.
- **a raw path**: a span with two or more path segments and no `@`, whose first segment names a
  file or a directory of the tree. The check reports it with the statement "is shaped like a path
  and names no anchor", raised at `Observation::UnanchoredPath` in `path@core@src/check/references.rs`,
  line 157.
- **a candidate**: for a raw path, a target that exists, computed as the owner proposed in round 1:
  the span, with any line suffix or fragment dropped, joined to each anchor's directory, the root
  included, and to the directory of the file that holds it, with `..` read lexically; distinct
  targets only, each spelled with its deepest anchor and with a trailing slash when it is a
  directory.
- **the census**: the measurement the agent's subagent took during the discussion, over the
  transcripts of this repository (14 sessions) and of thaum (25 sessions), of every reference
  finding and of how each session repaired it. Its tables, its scripts and their approximations
  were in the session's scratch directory, which does not outlive the session; its figures are the
  ones quoted under Arguments.
- **thaum**: the one known consumer of the checker, a separate repository whose checkout sits at
  `path@elsewhere@thaum/` beside this one on the owner's machine.
- **a misbinding**: an inference that rewrites a span into a reference that resolves, while the
  writer meant something else. The check passes on it, and nothing reports it later.
- **the generated list**: the files `generated_list` in `path@core@src/cli/mod.rs`, line 750,
  returns.
- **T1, T2, AC1 to AC4**: the labels the premortem put to the owner, kept in the items below.
- **D1, D2**: the defaults awaiting the owner, under that section.

## What the work is

What exists today at each site the work touches:

- **`check --fix`** is `fix_then_check` in `path@core@src/cli/mod.rs`, line 466. Its order, per
  `design@core@fix-before-the-checks`: a manifest holding a refused declaration writes nothing;
  the installed files are repaired and listed, `fixed: wrote <path> (installed)` or
  `fixed: removed <path> (installed)`; the gate of phases 1 to 3 runs over a model rebuilt from
  disk; the generated list is computed; the refusal of a partial commit's mismatch compares it with
  the staged tree's, per `design@core@fix-refusal-mixed-state`; the generated files are written,
  `fixed: wrote <path> (regenerated)`; the full check runs. No hand-written file is written: the
  head `design@core@safe-fix-definition` admits only "files of the installer's namespace or of the
  generated list".
- **The resolution of a `path` reference** is `path` and `anchored_target` in
  `path@core@src/check/references.rs`, lines 477 and 610. In order, `anchored_target` refuses: an
  unknown anchor; a milestone or a spec anchor; a refused shape, by `refused` at line 462 (a
  leading `/`, a `..` segment, a `.` segment); a plan document cited by its path, by
  `plan_document` at line 846, whose finding names the kind form; a target inside a deeper
  anchor, whose finding names no anchor ("anchor at the deepest anchor holding the target"). Then
  `path` asserts the trailing slash against the target's kind. Each refusal is a finding whose
  repair is a sentence; none carries the rewritten span.
- **A `path` citation of a skill or an agent** is resolved as any path and accepted. Three exist in
  the tree: `path@knowledge-architect@docs/design.md` line 239, and one each in two issue entries,
  `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` line 22 and
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` line 49. The entity table
  records each skill and each agent with one site, its file and line 1, in `section_home` of
  `path@core@src/entity.rs`, reached from `harness_definitions` for the walked documents and the
  installed ones.
- **A raw path's finding** is raised at line 157 of `path@core@src/check/references.rs`. Its
  repair is fixed text: "write `path@<anchor>@<path>`, `path@elsewhere@<path>` for a path outside
  this tree, or `path@*@<path>` for every component's own copy", plus the `planned` form in a plan
  document where nothing exists, plus a sentence on dropping a line suffix or a fragment. The test
  `an_unanchored_path_shape_s_repair_offers_no_unchecked_form`, line 2068, pins the whole repair
  line.
- **A bare skill or agent name** is reported at `Observation::BareName`, line 199, with the repair
  `write `skill@<name>`` or `write `agent@<name>``, per `design@core@bare-skill-name-reported`.
- **The retired form `<anchor>@<path>`** is reported at `Candidate::AnchorInKindPosition`, line 146,
  with the repair "prefix the kind".
- **The documentation of `--fix`**: `path@core@README.md` lines 124 to 145; the shipped setup skill,
  `path@agent-skills@content/skills/setup/SKILL.md` line 213, "it writes the installed files and
  the generated `index.md` files the check would report, then checks"; the shipped issue-tracking
  skill, `path@agent-skills@content/skills/issue-tracking/SKILL.md` lines 153 and 167; the shipped
  planning skill, `path@agent-skills@content/skills/planning/SKILL.md` line 129.

**Outside the work:**

- **Applying an inference** in any form, the owner's proposal (a) of round 1 included. Ruled out by
  `thread@path-quickfixes@fix-admission`; the narrower inference is
  `thread@path-quickfixes@same-anchor-inference`, parked.
- **Commit messages.** `commits` judges history and takes no `--fix`, per
  `design@core@check-fix-flag`. The census found 70 of the 179 raw-path findings there.
- **`--fix` with `--staged`.** It stays refused while parsing, per
  `design@core@fix-refusal-mixed-state`.
- **A `planned` reference whose target now exists**, **the retired `#<id>` form**, **a span with a
  line suffix or a fragment**, **a leading `/`**, and **a path that climbs above the root**: no
  respelling, per `thread@path-quickfixes@respelling-fixes`.
- **The retrospective finding on heads that state the built instance** is an issue entry, opened
  with this spec, `issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle`,
  per `thread@path-quickfixes@head-states-the-principle`.

## What is already decided

The design rests on these decisions, and does not argue them again:

- `design@core@check-fix-flag`: `--fix` applies every safe fix, then runs the full check; the
  option's generic name was chosen because "a later safe fix needs no new option".
- `design@core@phases-gate-the-report`: no writer writes over an incomplete model. A respelling
  reads the entity table, so it runs after the gate.
- `design@core@every-path-names-its-anchor` and `design@core@plan-document-kinds`: the deepest
  anchor wins, a plan document is cited by its kind, and the path is plain. The respellings write
  the form these heads require.
- `design@core@harness-kinds-cited-without-anchor`: a skill or an agent is cited `skill@<name>` or
  `agent@<name>`.
- `design@core@bare-skill-name-reported`: "A bare name is a pointer written with no kind." This is
  the recorded meaning the bare-name respelling relies on.
- `design@agent-skills@plain-text-is-no-repair`: a repair names checked forms only.
- `design@core@finding-names-the-repair`: the new repair text of a raw path follows it.

The work reverses or rewrites these decisions. Every text that `cargo klarch show` lists as
referencing each, on the main branch before this spec:

| decision | texts referencing it | judged or updated at |
| --- | --- | --- |
| `design@core@safe-fix-definition`: reversed in its second clause, "it writes or removes only files of the installer's namespace or of the generated list"; restated as the principle with the respelling as its test | `path@core@README.md` line 124; `path@core@docs/design.md` lines 142, 1088 and 1150; `path@core@docs/tripwires.md` lines 213 and 218; `path@core@src/cli/mod.rs` line 445 | step 1 for the README and the source; the harvest for the design home and the tripwires |
| `design@core@fix-scope`: gains the respellings | `path@core@src/agents.rs` line 181 | the harvest; the comment stays true where it names the installed files only |
| `design@core@fix-refusal-mixed-state`: its sentence "`--fix` cannot lose unstaged content, since it writes only generated and installed files" is rewritten | `path@core@README.md` line 139; `path@core@docs/design.md` lines 1090, 1108 and 1164; `path@core@docs/tripwires.md` lines 235 and 245 | step 1 for the README; the harvest for the design home and the tripwires |
| `design@core@fix-before-the-checks`: the respellings join the order, per D1 | `path@core@README.md` line 133; `path@core@docs/design.md` lines 71 and 114; `issue@core@installed-file-findings-belong-in-phase-four`, line 27; `path@core@src/cli/mod.rs` lines 445 and 542; `path@core@tests/binary.rs` line 3746 | step 1 for the README, the source and the test; the harvest for the design home and the issue |
| `tripwire@core@fix-makes-a-choice`: it fires on this work, and is rewritten to watch an inference | `path@core@docs/design.md` line 1145 | the harvest |
| `design@core@every-path-names-its-anchor`: gains the refusal of a `path` citation of a skill or an agent, and the question form of a raw path's repair | `path@knowledge-architect@CLAUDE.md` lines 142 and 180; `path@core@README.md` line 108; `path@core@docs/design.md` lines 635, 644, 765, 1369 and 1941; `path@core@docs/rejected-alternatives.md` lines 117, 146, 180, 186, 362 and 371; `path@core@src/check/references.rs` line 14; `path@core@src/manifest.rs` line 971 | steps 3 and 5 for the source; step 6 for the README and the root CLAUDE.md; the harvest for the design home and the rejected alternatives |
| `design@core@harness-kinds-cited-without-anchor`: a skill or an agent has no `path` spelling | `path@knowledge-architect@CLAUDE.md` line 135; `path@core@docs/design.md` lines 1305 and 1335; `path@core@docs/rejected-alternatives.md` lines 156, 161 and 167 | step 6 for the root CLAUDE.md; the harvest for the design home and the rejected alternatives |
| `design@core@bare-skill-name-reported`: its repair is applied by `--fix` | `path@knowledge-architect@CLAUDE.md` line 189; `path@agent-skills@docs/design.md` line 730; `path@core@README.md` line 105; `path@core@docs/design.md` lines 1370 and 1461; `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`, line 18; `issue@core@tooling-for-project-skills`, line 42; `path@knowledge-architect@docs/design.md` line 201 | step 4 for the source; step 6 for the README and the root CLAUDE.md; the harvest for the design home; the two issues read again at the harvest |

Two open entries bear on the work and are read at the audit:

- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: a fix that wrote a
  project-specific anchor into shipped text would make a reference that dangles in every
  installing project. No respelling writes a new anchor, per
  `criterion@path-quickfixes@shipped-text-stays-portable`.
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: the new repair text of a
  raw path and the new `fixed:` lines change a command's printed output.

## Criteria

### A fix never writes a reference that passes the check while naming something the writer did not mean `##no-silent-misbinding`

Binding. Derived by the agent in the reply to round 1 from
`goal@knowledge-architect@agents-work-without-drift` and
`goal@knowledge-architect@documentation-stays-consistent`. Met by
`thread@path-quickfixes@fix-admission`, `thread@path-quickfixes@respelling-fixes` and
`thread@path-quickfixes@raw-path-inference`.

### A fix's bytes are determined by the tree and the pinned version `##determined-bytes`

Binding as a presumption. Derived from the first clause of `design@core@safe-fix-definition`,
which the work keeps. Met by every thread.

### No unstaged content is lost `##no-content-loss`

Binding as a presumption. Derived from `design@core@fix-refusal-mixed-state` and from the root
CLAUDE.md, `instructions@git-workflow`, point 2. Met by `thread@path-quickfixes@fix-run-order`,
given `acceptance@path-quickfixes@a-changed-file-is-not-written`.

### No fix writes an unchecked form `##no-unchecked-form`

Binding as a presumption. Derived from `design@agent-skills@plain-text-is-no-repair`. Met by every
thread; the question form offers checked forms only.

### The share of raw-path findings repaired with no hand edit `##covers-the-common-finding`

Weighed. Stated by the owner in round 1: "It is by far the most common finding, because agents
always perfer the raw backticked form by habit." Unmet, and accepted by the owner in round 3: "I'm
not fully satisfied, because this design is quite a lot of work (especially the rewriting part),
and yet it only addresses a small portion of the findings that happen." The reply to round 3
stated this reading of the words, and the owner's round 4 did not contest it.

### No fix writes a project-specific anchor into shipped text `##shipped-text-stays-portable`

Weighed. Derived by the agent in the reply to round 1 from
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. Met: a respelling names the
target its span already names, so it writes no anchor the span did not already depend on, and the
question form writes nothing.

### The checker is not designed from this repository's own needs where a consumer's differ `##no-self-catering`

Binding, as the owner stated it in round 2: "I want to avoid catering to this project's own needs
when designing the checker." Listed as binding by the agent in the reply to round 2, until the
owner says otherwise; not contested. Met: the reply to round 2 re-argued
`thread@path-quickfixes@fix-admission` without this repository's cases,
`argument@path-quickfixes@a19` and `argument@path-quickfixes@a20`.

## Threads

### A fix may edit a hand-written document only as a respelling, and the head states the principle `##fix-admission`

Proposed by the agent in the reply to round 1, against the owner's claim of round 1 that the three
methods are "safe under the recorded condition for admitting quickfixes". Approved. Arguments:
`argument@path-quickfixes@a2`, `argument@path-quickfixes@a3`, `argument@path-quickfixes@a5`,
`argument@path-quickfixes@a17`, `argument@path-quickfixes@a18`, `argument@path-quickfixes@a19`,
`argument@path-quickfixes@a20`, `argument@path-quickfixes@a21`. Shape: Decided design, "What a fix
may write". Harvest: `design@core@safe-fix-definition` rewritten in place, and
`tripwire@core@fix-makes-a-choice` rewritten. The owner's words, round 2: "fix-admission: agreed."
After the agent's correction of the census, reply to round 2, the owner's words, round 3: "We'll go
with the current design."

### Six respellings, and five shapes left to the writer `##respelling-fixes`

Proposed by the agent in the reply to round 1. It absorbs the owner's proposal (b) of round 1, the
normalization of `path@<component>@../<path>`. Approved. Arguments: `argument@path-quickfixes@a4`,
`argument@path-quickfixes@a7`, `argument@path-quickfixes@a8`, `argument@path-quickfixes@a9`,
`argument@path-quickfixes@a10`, `argument@path-quickfixes@a22`, `argument@path-quickfixes@a23`.
Shape: Decided design, "The respellings". Harvest: `design@core@fix-scope` rewritten. The owner's
words, round 2: "respelling-fixes: this is less than I was hoping for. But I have to agree that my
bare backticked path fix proposal probably cannot be made safely..."; round 3: "We'll go with the
current design."; round 4, on the checkpoint table: "Keep all tripwires and AC, proceed."

### An unanchored path's repair asks which candidate was meant, and `--fix` writes nothing for it `##raw-path-inference`

Proposed by the owner in round 1 as proposal (a), "look at the path, under each component
(including root), and at paths relative to the containing file", applied when a single candidate
exists. The agent's reply to round 1 put three options: A, the owner's; B, the agent's default,
naming the one candidate in the repair; C, a separate flag. The owner proposed the question form
in round 2, which supersedes B. Approved. Arguments: `argument@path-quickfixes@a1`,
`argument@path-quickfixes@a6`, `argument@path-quickfixes@a11`, `argument@path-quickfixes@a12`,
`argument@path-quickfixes@a13`, `argument@path-quickfixes@a14`, `argument@path-quickfixes@a24`,
`argument@path-quickfixes@a25`. Shape: Decided design, "The question form". Harvest:
`design@core@every-path-names-its-anchor` gains it. The owner's words, round 2: "maybe the
canditates could be shown in the proposed repair: "Did you mean to write one of <path1>, <path2>...
?" (not as an affirmation "write <path1>", because as you said it could mean something else."; round
3: "We'll go with the current design."

### A `path` citation of a skill or an agent is a finding, and `--fix` respells it `##skill-cited-by-path`

Proposed by the owner in round 1 as proposal (c), "for paths to a subagent or skill: with the
skill@name or agent@name form" (its backticks dropped here, since in backticks each would cite a
skill or an agent named `name`); shaped by the agent in the reply to round 1 as a new check,
since such a citation is not a finding today. Approved. Arguments: `argument@path-quickfixes@a15`.
Shape: Decided design, "The skill and agent citation". Harvest:
`design@core@harness-kinds-cited-without-anchor` and `design@core@every-path-names-its-anchor`. The
owner's words, round 2: "skill-cited-by-path: approved."

### The respellings run after the gate, are printed one per line, and touch no commit message `##fix-run-order`

Proposed by the agent in the reply to round 1. Approved, with the order amended by D1, which
awaits the owner. Arguments: `argument@path-quickfixes@a16`. Shape: Decided design, "The order of a
run". Harvest: `design@core@fix-before-the-checks` rewritten, and the sentence of
`design@core@fix-refusal-mixed-state` named in "What is already decided". The owner's words, round
3: "We'll go with the current design."; round 4, on the checkpoint table: "Keep all tripwires and
AC, proceed."

### A head that states the instance built rather than the principle is recorded as an observation `##head-states-the-principle`

Proposed by the owner in round 2 as a remark; the agent proposed the issue entry in the reply to
round 2. Approved. Arguments: `argument@path-quickfixes@a17`, `argument@path-quickfixes@a21`.
Harvest: `issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle`, opened in
the commit that adds this spec. The owner's words, round 3: "head-states-the-principle: agreed on
the issue."

### An inference applied when its single candidate lies under the file's own anchor `##same-anchor-inference`

Measured by the agent in the reply to round 2, which did not propose it. Parked on the owner's word,
round 3: "The narrower variant you identified could be interesting to rediscuss, when I restart
working on this and the spec goes through a design audit." Arguments: `argument@path-quickfixes@a22`,
`argument@path-quickfixes@a26`.

- **Tripwire**: a census over a consumer project, other than thaum, in which the variant keeps at
  least half of the correct inferences in files (it kept 23 of 53 in the census) and makes no
  misbinding. The threshold is a default, the owner's to reset.
- **Re-entry point**: the design audit of this spec, when the work restarts.

Whether this tripwire is recorded in a tripwires home if the thread is still parked at the harvest
is D2.

## Arguments

### The request, its frequency and its safety claim `##a1`

Round 1, the owner. Bears on #raw-path-inference, #respelling-fixes, #skill-cited-by-path,
#fix-admission. "It is by far the most common finding, because agents always perfer the raw
backticked form by habit." / "the idea is to search for candidates, and apply only if a single one
exists." / "I think those methods are safe under the recorded condition for admitting quickfixes."

### The recorded condition excludes every method proposed, and its tripwire fires `##a2`

The reply to round 1. Bears on #fix-admission. `design@core@safe-fix-definition` admits only
"files of the installer's namespace or of the generated list", on the argument "A fix that makes a
choice, or touches git or a hand-written file, would rewrite what a writer meant."
`tripwire@core@fix-makes-a-choice` fires on "a fix … that writes or removes a file outside the
installer's namespace and the generated list", with the response "reopen
`design@core@safe-fix-definition` before the fix is added". "All three of your methods edit
hand-written documents, so the second clause excludes every one of them."

### A respelling keeps the writer's meaning; an inference chooses one `##a3`

The reply to round 1. Bears on #fix-admission. "the writer's meaning is already in the span, so the
fix changes only its spelling. #no-silent-misbinding holds by construction." The test, three
conditions: the span already names exactly one entity, under the checker's own resolution or under
a recorded head that declares what the span means; the rewrite names that same entity in the form
the check requires; no byte outside the span changes.

### A reference fix needs the entity table, and shipped text must stay portable `##a4`

The reply to round 1. Bears on #fix-run-order, #respelling-fixes. "a reference fix needs the entity
table, so it must run after the phase gate, never over an incomplete model", from
`tripwire@core@phases-gate-the-report-two`; and "a fix that writes a project-specific anchor into
shipped text produces a reference that passes here and dangles in every installing project", from
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`.

### The census of the unique-candidate rule: 7 apparent misbindings in 104 resolved cases `##a5`

The reply to round 1. Bears on #fix-admission, #raw-path-inference. Of 179 raw-path findings, "54
have 0 candidates, 110 have exactly 1, and 15 have 2 or more". Of the 110: 81 repaired exactly as
the rule would rewrite them, 6 with no repair found, 23 repaired otherwise, of which "Seven … are
the case that matters. The writer meant a path in the layout of every project, and the rule's
single candidate is this tree's copy … Nothing reports it afterwards." The figure was corrected in
the reply to round 2, `argument@path-quickfixes@a18`.

### Two or more candidates support no precedence rule `##a6`

The reply to round 1. Bears on #raw-path-inference. Of the 15 findings with two or more
candidates, the session chose the candidate of the file's own anchor 2 times and another
candidate 2 times, wrote no path reference 7 times, and chose the root in 4 commit messages. "This
does not support a precedence rule."

### The finding kinds a respelling would serve are rare `##a7`

The reply to round 1. Bears on #respelling-fixes. In the census: 9 findings of a target inside a
deeper anchor, 12 bare skill or agent names, 1 plan document cited by path, 2 `planned` targets
that now exist, 16 retired `#<id>` forms, and 0 of the upward, explicit-current, leading-slash,
anchor-in-kind-position and trailing-slash kinds. The owner's (b) "is correct: phase 2 refuses
symlinks, so lexical normalization equals the filesystem's. But it has never occurred in real
work." It is kept "only because it costs one normalization function".

### Five shapes are not respelled `##a8`

The reply to round 1. Bears on #respelling-fixes. A leading `/`: "its intended base is ambiguous".
A retired `<anchor>@<path>` whose target does not exist: "the span may have been a slug reference".
A `planned` reference whose target exists: the comment of `planned` in
`path@core@src/check/references.rs` keeps the conversion with the session that created the file,
since "whether the plan still holds is that session's report to make", and "An automatic rewrite
removes that prompt." A retired `#<id>`: "the census shows those spans were not meant as
references". A line suffix or a fragment: "dropping `:12` loses bytes the writer wrote."

### The bare-name respelling inherits the open prefix question `##a9`

The reply to round 1. Bears on #respelling-fixes. "It inherits the risk of
`issue@core@tooling-for-project-skills`: a skill named by an ordinary word would turn every
backticked use of that word into a rewrite." This became T1.

### Nine bare-skill findings of the census came from the lint under construction `##a10`

The reply to round 2. Bears on #respelling-fixes. The reply to round 1 had held the bare-name
respelling until 9 unexplained findings were explained. They came from the session that built the
lint, on its branch, on spans that are design slugs. "The shipped lint matches the skill kind
exactly (`entities.defines(&Kind::new(k), "", name)` …), so it cannot report a design slug." The
agent's inference, not traced to a commit: the lint matched slugs while it was being built. "The
real data on the shipped lint is 3 bare agent names, all 3 repaired exactly as the fix would
rewrite them."

### Naming the one candidate keeps the judgement with the agent `##a11`

The reply to round 1. Bears on #raw-path-inference, option B. "The agent then makes a one-token
edit with the judgement still in front of it. Most of the 81 agreements become copy-paste edits."

### A separate flag teaches agents to pass it `##a12`

The reply to round 1. Bears on #raw-path-inference, option C. "It removes the misbinding risk only
for sessions that do not pass the flag, and agents would learn to pass it."

### The repair text changes a command's output `##a13`

The reply to round 1 and the reply to round 2. Bears on #raw-path-inference. "the finding text
changes, which touches `issue@core@finding-texts-are-not-audited-for-a-needless-cause` and
`issue@knowledge-architect@command-output-is-not-declared-a-contract`." / "I would class it as a
patch with a changelog entry, as the `commits` summary change was classed."

### A question asserts no meaning `##a14`

Round 2, the owner, and the reply to round 2. Bears on #raw-path-inference. The owner: "maybe the
canditates could be shown in the proposed repair: "Did you mean to write one of <path1>, <path2>...
?" (not as an affirmation "write <path1>", because as you said it could mean something else." The
agent: "A question does not assert a meaning, and listing every candidate also serves the cases
with 2 or more candidates" / the line after it keeps the two non-tree forms, "so a writer who meant
neither candidate is not pushed toward one".

### A skill cited by path escapes `show` `##a15`

The reply to round 1. Bears on #skill-cited-by-path. "Today `path@agent-config@skills/klarch-release/SKILL.md`
is **not a finding**." The argument of `design@core@every-path-names-its-anchor` for a plan
document holds for a skill: "one name per document is what keeps `show` complete". "`show
skill@klarch-release` does not list those 3 lines today." A path to another file inside a skill's
directory, and the skills directory itself, have no harness-kind form and stay as they are.

### Where the respellings run, and what they print `##a16`

The reply to round 1. Bears on #fix-run-order. After the gate, since they read the entity table;
before the generated files, since "a respelling inside an issue title changes `index.md`"; "a
respelling rewrites the working-tree copy in place and loses nothing"; "The staged copy keeps the
old spelling, so `check --staged` fails until the file is staged again"; "each respelling is
printed as `file:line  old → new`"; commit messages are never touched; "rustfmt does not reflow
comments by default, so a longer comment line fails no gate."

### The current heads state the built instance as the rule `##a17`

Round 2, the owner, and the reply to round 2. Bears on #fix-admission, #head-states-the-principle.
The owner: "I think the current design heads are overstated on the admissible shape, I suspect they
were written as a description of what was done and not as a generic principle. I think this happens
quite often in how design heads are written. It's not too bad of an issue because reversing design
heads is not forbidden by the workflow, but it's often bothering me." The agent: "**its principle**
is in its argument: a fix must not "rewrite what a writer meant"" / "The clause states the shape
that was built as if it were the rule."

### Were the different repairs truly wrong, and only here? `##a18`

Round 2, the owner; the reply to round 2. Bears on #fix-admission, #raw-path-inference. The owner:
"would the inference have been truly wrong ? Or were the "different repairs" simply a consciously
chosen rewording, rather than just a repair ? There might have been checker avoidance too." The
agent's corrected figure: of the 7, "2 … were in commit messages", "1 was wrong as a misbinding",
and "**4 remain where the inference would have been truly wrong, all in this repository, and all
in text that describes another project's layout**": the skills and agents directories of the agent
configuration named in `path@core@src/entity.rs` and in the core design home, where the session
wrote the placeholder `.claude/skills/<dir>/SKILL.md`, "a checked form for a pattern … not checker
avoidance", and the plans directory in the agent-skills design home, "a path a project chooses".
"thaum: 0 silent misbindings in its 42 one-candidate findings in files." Its backtick removals
"look like checker avoidance, but they predate `design@agent-skills@plain-text-is-no-repair`".

### This repository's own needs weigh less `##a19`

Round 2, the owner; the reply to round 2. Bears on #fix-admission, #no-self-catering. The owner:
"Also, if they happen only here, maybe the argument is less strong. This project imposes more
constraints on the checker it publishes and uses itself than external consumers, because of the
need to describe the internal checker logic and the published workflow content in design docs. I
want to avoid catering to this project's own needs when designing the checker." The agent: "So
your suspicion holds: the observed failures come from this repository's need to describe other
projects' trees."

### The tree cannot tell two meanings of one span apart `##a20`

The reply to round 2. Bears on #fix-admission, #raw-path-inference. The identical raw span naming
the skills directory of the agent configuration, with the identical single candidate,
`path@agent-config@skills/`, meant this tree's directory in 3 plan documents, and every project's
layout in `entity.rs` and in the core design home. "The tree is the same in both cases, so no rule
that reads only the tree can tell those two meanings apart." In a consumer, thaum's CLAUDE.md held
a raw span naming the library root of a crate, meaning knowledge-architect's file; "It was refused
only because it had 2 candidates." The raw spans are described here in words, since written in
backticks each would be the finding it illustrates.

### The default kept the ruling on a narrower premise `##a21`

The reply to round 2. Bears on #fix-admission. "a wrong inference is silent for good, since nothing
reports a reference that resolves; the consumer sample is one project; your question-form repair
… recovers most of the gain with no silent case." The alternative offered: "apply the unique
candidate, and print each rewrite marked `inferred` so the session can revert it." The owner, round
3: "We'll go with the current design."

### The same-anchor variant would exclude the misbindings and most correct inferences `##a22`

The reply to round 2. Bears on #same-anchor-inference, #raw-path-inference. "It would have
excluded all 4 misbindings. It would also have dropped 30 of the 53 correct inferences in files, so
I do not propose it."

### The respellings are cheap and serve few findings `##a23`

The reply to round 2. Bears on #respelling-fixes, #covers-the-common-finding. "Measured value of the
respelling fixes in the census: about 13 findings, against 179 raw paths. They are cheap to build
because the code already computes each target form. The value of the work is mainly in the
question-form repair and in #skill-cited-by-path."

### Most one-candidate repairs agree with the rule `##a24`

The reply to round 1. Bears on #raw-path-inference, #covers-the-common-finding. 81 of the 110
one-candidate findings were repaired exactly as the rule would rewrite them; the question form
puts that rewrite in front of the session.

### Commit messages hold a large share of raw paths `##a25`

The reply to round 1. Bears on #raw-path-inference, #fix-run-order. "**179** (70 of them in commit
messages)". No `--fix` reaches a commit message, whatever the rule.

### The work is much for its share `##a26`

Round 3, the owner. Bears on #covers-the-common-finding, #same-anchor-inference. "I'm not fully
satisfied, because this design is quite a lot of work (especially the rewriting part), and yet it
only addresses a small portion of the findings that happen. The narrower variant you identified
could be interesting to rediscuss, when I restart working on this and the spec goes through a
design audit."

## New names, in one place

An illustration of the shapes, not authority; the audit and the implementing session decide the
final names.

```text
crates/core/src/fix.rs                                  new module, pub(crate)
  struct Respelling { file, line, old, new }            one rewrite: the span's text without its
                                                        backticks, before and after
  fn respellings(model, manifest, entities, inputs)     -> Vec<Respelling>, computed after the gate;
                                                        no check calls it
  fn apply(root, &[Respelling]) -> Result<Vec<..>>      reads each file once, rewrites exactly the
                                                        spans, and writes a file only if its bytes
                                                        on disk still equal the bytes read

crates/core/src/check/references.rs
  fn candidates(span, rel, anchors, inputs)             new: the candidates of a raw path, as Names
                                                        defines them, spelled as references
  fn harness_citation(target, entities)                 new: the skill or agent whose file or
                                                        directory a path target is, if any
  fn path, fn anchored_target, fn refused,              existing; each refusal that has a
  fn plan_document                                      respelling exposes the respelled span

output of check --fix
  fixed: respelled <file>:<line> `<old>` → `<new>`      new line, one per respelling, before the
                                                        generated files' lines
```

The new module is `planned@core@src/fix.rs`.

## Decided design

### What a fix may write

**A fix never rewrites what a writer meant.** That is the principle, and the head
`design@core@safe-fix-definition` states it at the harvest. Its tests:

- **the bytes are determined by the tree and the pinned version**, the clause kept;
- **a hand-written document is edited only by a respelling**: the span already names exactly one
  entity, under the checker's own resolution or under a recorded head that declares what the span
  means; the rewrite names the same entity in the form the check requires; no byte outside the
  span changes;
- **an inference is never applied**: a rewrite that chooses an entity the span does not name stays
  the writer's, however the tree constrains it;
- **git is never touched**: staging and the index stay outside `--fix`, as today.

The installed files and the generated files still pass, as their bytes are the installer's and the
generator's, not a writer's. The scope list of `design@core@fix-scope` becomes a consequence of the
tests, not the test.

Nearest rival: the owner's unique-candidate rule as the admission test. The fact that defeated it:
the census found 4 misbindings among the one-candidate findings in files, and the tree cannot tell
apart two meanings of one span, `argument@path-quickfixes@a20`. The owner weighed the corrected
premise, `argument@path-quickfixes@a18`, and kept the ruling.

### The respellings

| finding | its respelling | condition |
| --- | --- | --- |
| a target inside a deeper anchor | the deepest anchor's spelling, `path@<deepest>@<rest>` | the reference resolves otherwise |
| a `..` or a `.` segment | the path normalized lexically, then the deepest anchor's spelling | the normalized target stays inside the root and exists |
| a trailing slash that disagrees with the target's kind | the slash added for a directory, dropped for a file | the target exists |
| a plan document cited by its path | the form `plan_document` already names: `spec@plans@<id>`, `milestone@plans@<id>` or `spec@<milestone>@<slice>` | the finding is raised |
| the retired `<anchor>@<path>` | `path@<anchor>@<path>` | the target exists under that anchor |
| a bare skill or agent name | `skill@<name>` or `agent@<name>`, the skill where both exist, as the lint's repair already says | the lint reports it |
| a `path` citation of a skill or an agent | `skill@<name>` or `agent@<name>` | the new finding of "The skill and agent citation" is raised |

The table has seven rows for six kinds of finding today plus the new one. A respelling whose result
would itself be a finding is not applied; `acceptance@path-quickfixes@respellings-leave-no-finding`
judges it.

**Not respelled**, each a finding as today: a leading `/`; a path that climbs above the root; a
`planned` reference whose target exists; a retired `#<id>`; a span with a line suffix or a
fragment; a span wrapped onto two lines; a span inside a file of the generated list, which the
generator rewrites. The reasons are `argument@path-quickfixes@a8`.

Nearest rival: no respelling, the head as it stands. It lost on `argument@path-quickfixes@a3`: each
row's output is already computed or named by the check, and applying it changes no meaning.

### The question form

The finding on a raw path keeps its statement. Its repair lists the candidates as a question, and
keeps the forms for a path the tree does not hold. An illustration, two candidates:

```text
docs/x.md:12  `<span>` is shaped like a path and names no anchor
  → did you mean `path@core@<path>` or `path@gates@<path>`? Otherwise write
    `path@elsewhere@<project>/<path>` for another project's file, or a placeholder for a pattern
```

- With no candidate, the repair is today's.
- With a line suffix or a fragment, the candidates are of the file, and today's sentence on
  dropping the suffix stays.
- In a plan document, where nothing exists, today's `planned` form stays.
- `--fix` writes nothing for this finding.

The order of the candidates and a bound on how many are listed are the audit's to settle, under T2.

Nearest rival: B, a repair that names the one candidate as an instruction, "write
`path@core@<path>`". It lost to the owner's question in round 2, `argument@path-quickfixes@a14`.

### The skill and agent citation

A `path` reference whose target is a skill's directory, a skill's `SKILL.md`, or an agent's file is
a finding, judged against the entity table: the target is the site the table records for a skill
or an agent, or the directory holding a skill's site. The finding names `skill@<name>` or
`agent@<name>`, and `--fix` respells it. It is judged by where the target sits, as a plan document
citation is.

- A path to any other file inside a skill's directory stays a `path` reference.
- The skills directory and the agents directory themselves stay `path` references.
- Under `harness = []`, the table defines no skill and no agent, so nothing is reported.
- The three citations of the tree are rewritten in the commit that adds the check.

Nearest rival: no check, the citation accepted as a path. It lost on `argument@path-quickfixes@a15`.

### The order of a run

Per D1, which awaits the owner:

1. a manifest holding a refused declaration writes nothing;
2. the installed files are repaired;
3. the gate of phases 1 to 3;
4. the respellings, each printed;
5. the model rebuilt from disk, and the gate run again; a stop is printed as `check` prints it;
6. the refusal of a partial commit's mismatch, over the rebuilt model;
7. the generated files;
8. the full check.

`--fix --staged` stays refused while parsing. A respelling rewrites the working-tree file only; a
file whose staged copy differs keeps the old span in git's index, and `check --staged` reports it
until the file is staged again. A failed write exits 2 when nothing was written and 1 after any
write, as today.

Nearest rival: the order the reply to round 1 showed, the refusal before the respellings. It is
replaced by D1, below.

## Mapping tables

Every finding of `check::references` that concerns a reference's spelling, and what `--fix` does
with it after the work:

| finding | `--fix` |
| --- | --- |
| unknown kind, unknown anchor, the anchor carries no register of that kind, an undefined id | nothing: a choice |
| a malformed span | nothing |
| the retired `<anchor>@<path>` | respelled, when the target exists |
| the retired `#<id>` | nothing |
| a raw path | nothing; the repair asks |
| a bare skill or agent name | respelled |
| a wrapped span | nothing |
| a refused shape: a leading `/` | nothing |
| a refused shape: `..` or `.` | respelled, when the normalized target exists inside the root |
| a plan document cited by its path | respelled |
| a target inside a deeper anchor | respelled |
| a trailing slash against the target's kind | respelled |
| a path that does not resolve | nothing |
| a `path@elsewhere@<path>` that resolves | nothing |
| a `planned` reference outside the plans directory, or whose target exists | nothing |
| a `path` citation of a skill or an agent (new) | respelled |

The implementing session completes the table against the findings `check::references` raises at
step 1, and a finding the table misses is not respelled.

## Losing alternatives

- **The unique candidate applied by `--fix`**, the owner's proposal (a) of round 1 and option A of
  the reply to round 1. Lost to #fix-admission on `argument@path-quickfixes@a5`,
  `argument@path-quickfixes@a18` and `argument@path-quickfixes@a20`.
- **The unique candidate applied and printed as `inferred`**, the alternative of the reply to round
  2. Lost to #fix-admission on `argument@path-quickfixes@a21`.
- **A separate opt-in flag that applies inferences**, option C. Lost to #raw-path-inference on
  `argument@path-quickfixes@a12`.
- **A repair that names the one candidate as an instruction**, option B, the agent's default of
  round 1. Superseded by the owner's question form, #raw-path-inference, on
  `argument@path-quickfixes@a14`.
- **A precedence rule among two or more candidates**, the file's own anchor first. Lost to
  #raw-path-inference on `argument@path-quickfixes@a6`.
- **A respelling of a `planned` reference whose target exists, of a retired `#<id>`, of a span with
  a line suffix or a fragment, and of a leading `/`.** Lost to #respelling-fixes on
  `argument@path-quickfixes@a8`.
- **The same-anchor inference** is not lost: it is parked, #same-anchor-inference.

## Readings

None. The work reads no external specification. The layout of a skill and an agent is the one
`design@core@harness-kinds` already records.

## Premortem

Assume the work shipped and failed. The causes, as presented in the reply to round 3:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. A project names a skill by an ordinary word, such as `check`, and `--fix` turns every backticked `check` into a reference to a skill named `check` | #respelling-fixes, the bare-name row | the tripwire **T1**, on the owner's word, round 4 |
| 2. Agents take the first listed candidate without reading the sentence, and the misbinding comes back through the agent | #raw-path-inference | the tripwire **T2**, on the owner's word, round 4 |
| 3. A file edited between the moment `--fix` reads it and the moment it writes it loses that edit | #fix-run-order, #no-content-loss | the acceptance criterion **AC1**, on the owner's word, round 4 |
| 4. A respelling produces a form the check refuses, such as the deepest anchor applied before the plan-document rule | #respelling-fixes | the acceptance criterion **AC2**, on the owner's word, round 4 |
| 5. A rewrite touches bytes outside the span: line endings, a wrapped span, a multibyte neighbour | #fix-admission | the acceptance criterion **AC3**, on the owner's word, round 4 |
| 6. The new check reports a path into a skill's other files, which has no harness-kind form | #skill-cited-by-path | the acceptance criterion **AC4**, on the owner's word, round 4 |

The owner's ruling, round 4: "Keep all tripwires and AC, proceed."

T1, as the harvest writes it: guards `design@core@fix-scope`. Fires when a review or a
retrospective reports a `--fix` rewrite of a bare name the writer did not mean as a skill or an
agent. Response: reopen the bare-name respelling; the candidate fix is to respell only names that
carry a prefix, which `issue@core@tooling-for-project-skills` would check. Re-entry: the
standing-state review of every dispatched review, and each retrospective intake.

T2, as the harvest writes it: guards `design@core@every-path-names-its-anchor`, where the question
form is recorded. Fires when a review finds a raw-path repair that took a listed candidate where
the text meant another project's file or every project's layout. Response: reopen the question
form; one candidate fix is to list the candidates in a fixed order with no first-choice position.
Re-entry: the standing-state review of every dispatched review. It fires only in sessions the owner
sees, as `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`
records of every tripwire on agent behaviour.

## Acceptance criteria

### AC1: `--fix` writes a document only if its bytes are still the bytes it read `##a-changed-file-is-not-written`

Guards `thread@path-quickfixes@fix-run-order`, and `criterion@path-quickfixes@no-content-loss`.
Judged at step 1. A test changes a document between the computation of its respellings and their
application, through a hook the module exposes to its unit tests, and asserts that the file keeps
the changed bytes and that the run names the file and writes nothing to it. Fires when the file
loses the change. Response: reopen #fix-run-order on how a document is written.

### AC2: every respelling leaves no finding on its span, and a second run writes nothing `##respellings-leave-no-finding`

Guards `thread@path-quickfixes@respelling-fixes`. Judged at steps 2 to 4. A test over a project
holding every respellable shape of the table under "The respellings" runs `check --fix`, then asserts
that no finding is reported on any rewritten span, and that a second `check --fix` prints no
`fixed:` line. Fires on a finding on a rewritten span, or on a second write. Response: reopen
#respelling-fixes on the row that produced it.

### AC3: a respelling changes exactly the bytes of its span `##respelling-is-byte-exact`

Guards `thread@path-quickfixes@fix-admission`. Judged at step 1. A unit test over a file with CRLF
line endings, multibyte characters beside the span, the same span twice on one line, and a span
wrapped onto two lines asserts that the byte difference after the rewrite is exactly each recorded
span, and that the wrapped span is untouched. Fires when any other byte changes. Response: reopen
#fix-admission on what a respelling may touch.

### AC4: a path into a skill's other files, and the skills directory itself, stay silent `##other-skill-files-stay-silent`

Guards `thread@path-quickfixes@skill-cited-by-path`. Judged at step 3. A test asserts that a `path`
reference to a file under a skill's directory other than its `SKILL.md`, and one to the skills
directory itself, raise no finding of the new check. Fires when either is reported. Response:
reopen #skill-cited-by-path on what the check judges.

## Implementation sequence

Steps 1 to 5 follow the project's development procedure, `skill@klarch-development`, and pass the
gates they owe. Step 6 edits the root CLAUDE.md and the shipped text, which that skill excludes:
it follows `skill@knowledge-architect-agent-configuration`, and for the shipped text the section on
editing an installed skill or agent of `path@agent-skills@CLAUDE.md`. Each default under "Defaults
awaiting the owner" gates the point it names. A planted defect goes in the mock project `planted`,
per `path@core@CLAUDE.md`; `check --fix` is never run inside it.

1. **The respelling machinery, empty.** `planned@core@src/fix.rs` with `Respelling`, `respellings`
   returning none, and `apply`; the order of "The order of a run" in `fix_then_check`; the
   `fixed: respelled` line. The README and the source comments that cite
   `design@core@safe-fix-definition`, `design@core@fix-before-the-checks` and
   `design@core@fix-refusal-mixed-state` updated. Claims: AC1; AC3; `check --fix` over `dirhome`
   writes no document and prints what it prints today; every existing test of `--fix` passes.
   Fails alone on: a byte written outside a span, or a write over a changed file.
2. **The path respellings**: the deeper anchor, `..` and `.`, the trailing slash, the plan document
   by its path, the retired `<anchor>@<path>`. Claims: AC2 for these rows; a leading `/`, a path
   above the root, a suffix and a wrapped span are not respelled. Fails alone on: a respelled span
   still reported.
3. **The skill and agent citation.** The finding in `anchored_target` or `path`, the respelling,
   the three citations of the tree rewritten in the same commit, a planted defect in `planted`.
   Claims: AC4; AC2 for the row; nothing is reported under `harness = []`. Fails alone on: a
   citation of a skill's `SKILL.md` accepted, or another file of its directory refused.
4. **The bare-name respelling.** Claims: AC2 for the row; a name both a skill and an agent hold is
   respelled as the skill. Fails alone on: a span the lint does not report respelled.
5. **The question form.** `candidates`, the new repair text, the pinned test
   `an_unanchored_path_shape_s_repair_offers_no_unchecked_form` updated to the new line. Claims:
   zero, one and two candidates each give the repair of "The question form"; a line suffix and a
   plan document keep their sentences; `--fix` writes nothing for the finding. Fails alone on: a
   candidate that does not exist listed, or an existing one missed.
6. **The documentation and the changelog.** `path@core@README.md` on `--fix`, on the new finding,
   and on the question form; the root CLAUDE.md where it restates
   `design@core@every-path-names-its-anchor`, `design@core@harness-kinds-cited-without-anchor` and
   `design@core@bare-skill-name-reported`; the shipped setup, issue-tracking and planning skills
   where they say what `--fix` writes, then `cargo klarch install-agent-skills`. Entries in the
   `Next release` section of CHANGELOG.md, then `cargo x changelog`:
   - New features: `cli`, minor: `check --fix` respells a reference whose target its span names,
     and prints each rewrite.
   - Migration: `checks`, minor: a `path` citation of a skill or an agent is a finding; rewrite it
     as `skill@<name>` or `agent@<name>`, or run `check --fix`.
   - New features: `checks`, patch: the finding on a backticked path that names no anchor lists
     the candidates the tree holds, per `argument@path-quickfixes@a13`.
   Claims: `cargo klarch check` passes with the installed copies committed;
   `cargo test -p knowledge-architect-agent-skills` passes; the shipped edits cite no entry and
   write `{{command}}`, by a reading. Fails alone on: a changelog copy that differs.
7. **The harvest**, below, reviewed before the merge. The spec is deleted in the commit that
   completes it.

## Order rationale

Step 1 before step 2: the respellings need the machinery that step 1 proves empty. Step 2 before
step 3: the citation's respelling goes through the path resolution that step 2 changes. Step 3
before step 4: step 3 is the change to what the check judges, and lands while the respelling
machinery is fresh; step 4 changes no check. Step 4 before step 5: the question form is
independent of `--fix`, and comes last among the code so the `--fix` steps share one review of
the order. Step 5 before step 6: the documentation describes what was built. Step 6 before step 7:
the harvest records the decisions the built work implements.

## Defaults awaiting the owner

- **D1**, on #fix-run-order, from the author at assembly. The order shown in the reply to round 1
  put the refusal of a partial commit's mismatch before the respellings. The refusal compares the
  generated files of the working tree with the staged tree's; a respelling inside an issue title
  changes the working tree's `index.md` after that comparison, so the comparison would be made on
  bytes the run then changes. The default: the respellings run after the first gate; the model is
  rebuilt and the gate run again; then the refusal; then the generated files, as "The order of a
  run" lists. The alternative: the order as shown, with the refusal made before the respellings.
- **D2**, on #same-anchor-inference, from the transcript's coverage at assembly. The owner's word of
  round 4, "Keep all tripwires and AC", stands after a premortem that labelled T1 and T2, while the
  parked thread's tripwire stood in the checkpoint table with no label. The default: if the thread
  is still parked at the harvest, its tripwire is recorded in `path@core@docs/tripwires.md`,
  guarding `design@core@safe-fix-definition`. The alternative: it leaves with this spec.

## Harvest

At step 7, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's
commit, with the test it fails.

| item | home |
| --- | --- |
| #fix-admission | `design@core@safe-fix-definition` rewritten in place: the principle, its tests, the respelling and the inference; the census figure and the meaning argument, `argument@path-quickfixes@a20`, as its argument |
| #respelling-fixes | `design@core@fix-scope` rewritten: the three kinds of fix, the respellings listed as the table of "The respellings", the shapes not respelled |
| #fix-run-order | `design@core@fix-before-the-checks` rewritten with the order of "The order of a run"; the sentence of `design@core@fix-refusal-mixed-state` named in "What is already decided" rewritten; `issue@core@installed-file-findings-belong-in-phase-four` read again for its sentence on the order |
| #skill-cited-by-path | `design@core@harness-kinds-cited-without-anchor` gains that a skill and an agent have no `path` spelling, with `argument@path-quickfixes@a15`; `design@core@every-path-names-its-anchor` names the refusal beside the plan-document one |
| #raw-path-inference | `design@core@every-path-names-its-anchor` gains the question form |
| #head-states-the-principle | done in the commit that adds this spec: `issue@agent-skills@a-head-states-the-instance-built-rather-than-the-principle` |
| #same-anchor-inference | per D2 |
| `tripwire@core@fix-makes-a-choice` | rewritten: fires on a fix that applies an inference, or that changes a byte outside a span; its response unchanged |
| every item of "Losing alternatives" | `path@core@docs/rejected-alternatives.md`, each as the recording tests admit |
| the existing entries of `path@core@docs/rejected-alternatives.md` that cite a rewritten head | each read again; each still lost to its head as rewritten, or is edited to say so |
| T1 | `path@core@docs/tripwires.md`, guarding `design@core@fix-scope`, as the Premortem section words it |
| T2 | `path@core@docs/tripwires.md`, guarding `design@core@every-path-names-its-anchor`, as the Premortem section words it |
| AC1 to AC4 | reported on at the landing; each spent, or proposed to the owner as a tripwire if it recurs |
| this spec | deleted in the harvest's commit, cited as `spec@plans@path-quickfixes` |

## Later consequences

- A later fix of `--fix` is admitted by the tests of "What a fix may write", not by a list. A fix
  that would need an inference reopens #fix-admission.
- The raw-path finding stays the most frequent one this work does not repair. The owner accepted
  that in round 3; #same-anchor-inference is where it is argued again.
- `issue@core@tooling-for-project-skills`, once closed with a prefix check, is the narrowing T1's
  response names for the bare-name respelling.
