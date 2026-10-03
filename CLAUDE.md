# knowledge-architect

@.claude/knowledge-architect/PRIMER.md

The line above imports the installed primer of the workflow: its directives and the workflow's
rows of the knowledge table. This file adds what is this repository's own.

A documentation checker and an agent workflow, shipped as Rust crates. The checker verifies that a
project's documents stay consistent with its code and with each other, through declared registers
and checked references. The workflow is a set of agent skills that the checker embeds and writes
into a project on request.
The goals are `path@knowledge-architect@docs/goals.md`.

**The goals bind the design, and nothing else in it does.** A constraint derived from a goal binds
an argument outright. One derived from a recorded decision binds it as a stated presumption, which
a better argument can rebut. Recorded decisions were often argued before the code existed, so
implementation is expected to prove some of them wrong: reversing one is an ordinary move, and
`knowledge-architect-decision-recording` owns what it costs.

The repository is a virtual workspace with five Components, per
`design@knowledge-architect@repo-layout`:

| Component | directory | what it is |
| --- | --- | --- |
| `knowledge-architect` | the root | the project itself |
| `core` | crates/core | package `knowledge-architect`: the checker library and the binary `klarch` |
| `agent-skills` | crates/agent-skills | package `knowledge-architect-agent-skills`: the text the checker installs into a project |
| `gates` | crates/gates | package `knowledge-architect-gates`: the library that runs a project's merge gates |
| `xtask` | tools/xtask | the maintenance tool, `cargo x gates`; never published |

## Language, tone and style

### Always

- Use plain, direct technical English.
- Write **short** sentences.
- Use em dashes and semicolons sparingly in prose. They usually make a sentence longer than it
  should be.
- Use precise technical terms instead of idioms and colloquialisms.
- No metaphors, no aphorisms. The meaning is always carried explicitly.
- Prefer explicit quantities, units and invariants over qualitative wording.
- Readers are non-native speakers. Keep that in mind.

### When presenting content to the user

Answer with precise content, not vague or ambiguous descriptions.

- For prose, a verbatim is better than a reformulation.
- For code, an example or a snippet is better than a description of the effect.

Verify every claim you present to the user, per `### Verify a claim before writing it` below.

State the consequences of a request explicitly. Never assume the user has considered all of them.

### When writing into durable files in the project

- Prefer lists and tables over long prose paragraphs when listing different items.
- **Current reality only**: no dates in a head, no changelogs outside CHANGELOG.md, no "formerly
  known as".
- Avoid numbers that may go stale, except in issues that follow the cold-reader standard of the
  `knowledge-architect-issue-tracking` skill.

## Mechanical validation of documents

The project keeps a precise record of decisions, their arguments, the alternatives that lost, and
what is still open. The checker verifies that record mechanically, and this repository is checked
by its own checker. This section is a restatement. The decisions live in the core's design home,
`path@core@docs/design.md`, under the heads named below, and where the two disagree those heads
win.

- **The manifest is `path@knowledge-architect@knowledge-architect.toml`.** It declares the
  Components, the locations and the registers. Nothing about this repository is compiled into the
  checker, per `design@core@nothing-of-a-project-is-compiled-in`.

- A **register** is one kind of recorded thing together with its storage. Ten are built in.
  Every Component carries four of them: `design`, `goal` and `tripwire` are **heading
  registers**: their entries are headings carrying a slug. `issue` is a **file register**: its
  entries are one file each. Two more, `spec` and `milestone`, hold plan documents and are
  carried by the anchor `plans`, per `design@core@plan-register`. The last four, `thread`,
  `argument`, `criterion` and `acceptance`, hold a plan's items and are carried by each plan
  document, per `design@core@plan-items-by-section`. A project may declare more, per
  `design@core@registers-are-declared`. This one declares none.

- An **anchor** is a named directory, or a spec file, that carries registers. A **Component** is
  an anchor that carries the required documents and every built-in component register. Its name
  is the basename of its directory, and the project root is the Component named
  `knowledge-architect`. A **location** is an anchor that carries only the registers it declares,
  with their homes directly under its path. One location is declared: `agent-config`, at
  .claude, carrying `issue`. The tool constructs three more kinds: `plans`, at docs/plans/, one
  anchor per milestone directory, and one anchor per spec file. The decision is
  `design@core@anchors-are-components-and-locations`.

- **Each Component carries the same required documents**, per
  `design@core@components-carry-the-same-documents`: `path@*@README.md`, `path@*@CLAUDE.md`,
  `path@*@docs/rejected-alternatives.md`, and the home of each built-in component register.
  - A heading register's home is one file, `docs/<dir>.md`, or a directory, `docs/<dir>/`, once
    it outgrows one file. The directory's `README.md` is the head: an introduction, and markdown
    links naming every subdocument, each target relative to the README. The entries sit in the
    subdocuments. Exactly one of the two shapes, never both, per
    `design@core@heading-register-two-shapes`. So the design home is `path@*@docs/design.md` or
    its directory, the goals home `path@*@docs/goals.md` or its directory, and the tripwires home
    `path@*@docs/tripwires.md` or its directory. Each may hold no entry.
  - The issue register's home is the directory `path@*@docs/open-issues/`: one file per entry, a
    hand-written `README.md` and a generated `index.md`, per
    `design@core@a-file-register-is-a-directory-of-entries`.

- **An entry of a heading register is a heading at the register's level, ending with its slug**,
  per `design@core@an-entry-is-a-heading-at-the-register-level`.
  - The shape is ``### The statement `##<slug>` ``: the statement first and the slug last, with no
    bold and no em dash between them, so that a document outline reads as a list of decisions.
  - Design entries sit at level three. Goal and tripwire entries sit at level two. A plan's item
    sits at level three, and the level-two section above it gives its kind, per
    `design@core@plan-items-by-section`.
  - A slug is defined in its register's home, never in the `README.md` of a directory-shaped home.
  - Every heading at the register's level in its home carries a slug. One without is a finding. A
    heading at another level is section text.
  - A slug anywhere else defines nothing and is reported as a misplaced definition: at a heading
    of another level, in a table cell, at the head of a plain line, in the middle of a line, in a
    file that is no register home.
  - An id is defined once per register instance. Two definitions are a finding at each site.
  - An id matches `[a-z0-9]+(-[a-z0-9]+)*`. A file register's entry id is the file's basename.

- **Every reference is one backticked span, `<kind>@<anchor>@<id>`**, per
  `design@core@a-slug-belongs-to-a-component`. The kind is a register's name or `path`. The anchor
  is a Component, a location or a plan. The id is the entry's. A plan's item is cited from
  inside that plan only, per `design@core@plan-item-scope`. Examples:
  `design@core@reserved-anchors`, `design@gates@gates-run-all`, `path@core@docs/tripwires.md`. A reference that resolves to nothing
  is reported with the repair it needs: the kind is unknown, the anchor is unknown, the anchor
  carries no register of that kind, or the id is not defined there. A reference that names no
  anchor is refused, including inside the file that defines the id.

- **Under the `path` kind, the id is a path under the anchor's directory**, and its target is
  checked to exist, per `design@core@every-path-names-its-anchor`.
  - A trailing `/` claims the target is a directory. Its absence claims a file. The checker asserts
    the claim, per `design@core@trailing-slash-claims-directory`.
  - No `..`, no `./`, no leading `/`. The deepest anchor wins: a target inside a Component or a
    location is named from that anchor, never from one above it. So relocating an anchor edits the
    manifest and no document. A file under crates/core is `path@core@<file>`, and one under .claude
    is `path@agent-config@<file>`.
  - Two anchors are reserved under `path` alone, and the word `plans` names the anchor the tool
    constructs, per `design@core@reserved-anchors`. A plan document is cited by its kind,
    `spec@plans@<id>`, `milestone@plans@<id>` or `spec@<milestone>@<step>`, never by its path.
    `path@*@<path>` names every Component's own copy of a path, as in `path@*@docs/tripwires.md`.
    `path@elsewhere@<path>` names a path that is deliberately not resolvable in this tree: another
    project's layout, a deleted or hypothetical file. It is never checked for existence, and it is
    refused when its target does resolve here.
  - A target the ignore rules cover is accepted without existence or kind assertions, per
    `design@core@ignored-targets-are-not-asserted`. The ignore rules are what `git check-ignore`
    answers, nested `.gitignore` files included.
  - A relative markdown link, `[title](<path>)`, is a navigation row. It is legal in `README.md`
    and `index.md` files only, and it is resolved against the linking file's own directory, under
    the same constraints. In every other file a pointer is a reference, per
    `design@core@links-are-navigation-rows`.

- **Which backticked spans are read as references**, per
  `design@core@candidate-rule-and-retired-forms`:
  - A span with no space whose head before the first `@` is a kind or an anchor is a candidate.
    Any other span holding an `@`, such as an email address or a git remote, is silent. A typo
    inside the kind is silent too.
  - **A reference is live wherever it is prose, fenced blocks included, for every kind.** An
    illustration that must not resolve writes a placeholder in angle brackets, as in
    `<kind>@<anchor>@<id>`, which is not a candidate. A string literal bound to a name in Rust
    yields no reference. In the checker's own source every literal is data, bound or not, per
    `design@core@checker-source-literals-are-data`.
  - Two retired forms are reported, not ignored: `<anchor>@<path>` with no kind, and a backticked
    `<word>#<word>`.
  - A backticked span of path characters with two or more segments and no `@` is reported as a
    path to anchor. Write it as a `path` reference, or in plain text. A one-segment span is a name
    rather than a pointer, and is silent.

- **A commit message is a document.** It is parsed as one markdown document, and every reference
  in it must resolve, against its own commit's tree or its first parent's, per
  `design@core@a-commit-message-is-a-document`. A message is history, read years later, so a
  pointer in it that resolves to nothing tells its reader nothing.

## Where knowledge goes

**Every durable decision has exactly one home**, chosen by what it is about and when it stops
being true. What is bound is the _why_ of the decision, the losing arguments, and the evidence the
decision rests on. A second mention of a decision is a pointer, never a copy, because a restated
decision starts drifting the moment it is written.

**A directive is different, and is restated wherever it has to be delivered.** A reader who cannot
reach a statement at the moment they act is not served by a pointer to it. Three clauses bound
this, and they pay for the drift it admits:

- the decision's home is authoritative, so where a restatement and its home disagree, the
  restatement is the defect.
- a restatement carries its pointer, adjacent. The pointer marks it deliberate rather than an
  orphan copy.
- **a restatement is never replaced by a pointer on one-home grounds.** Whether a directive is
  needed at a point of delivery is a delivery decision, and those belong to the owner of the
  configuration.

The workflow's rows of the knowledge table are in the primer, per
`design@agent-skills@knowledge-table-home`. This repository's own rows, each refining or adding to
the primer's:

| the statement is about | home | it leaves when |
| --- | --- | --- |
| descriptions of unbuilt work and the plans to build it: specs and milestones. Provisional: it defines no design entry, and its items are cited from inside it only | the plans directory, `path@knowledge-architect@docs/plans/` | the planned work lands |
| what is outstanding about the agent configuration itself | the agent-config location's issue directory, `path@agent-config@open-issues/` | the issue closes |
| how a user can use a published crate, beyond the primer's README row | its `path@*@README.md` is also its crates.io page, and points to docs.rs for the library API | the contract changes |
| the description of a crate's library API | the crate-level documentation of its lib.rs, which docs.rs renders | its contract changes |
| what a consumer must change, can start using, or will see agents do differently, per release | CHANGELOG.md at the root: one section per version, and the `Next release` section | a released section's content never changes; its structure may, and the `Next release` section may change at any time, per `design@knowledge-architect@changelog-entries` |
| a finding established outside this project that a decision here leans on | `docs/grounding/<subject>.md` in the Component the decision belongs to | the source is superseded, or a better one lands |
| a Component that maintains the repository rather than shipping in it | `tools/<name>/`, one directory per tool | the tool is retired |
| **none of these, nor a row of the primer** | **ask, before writing it anywhere** | the table gains the row |

**The last row is for a statement with no home, not for a choice between two.** When two rows
could fit, pick one, say which you picked, and continue. A genuine gap means this table is
incomplete. The table is a decision about the shape of the configuration, and that decision is the
owner's. Each answer ends as a new row, so the fallback limits itself: if it fires often, the table
is wrong.

**A measurement is not a kind of statement. It is routed by what it serves.** A figure that
supports a decision goes in that decision's head. One that characterises a defect or a question
goes in that entry. A tripwire's firing threshold goes inside the tripwire. One that serves none of
them goes in the commit message that took it.

**Never record a number that will go stale, unless staleness is the point.** A figure carries the
claim it supports, the command that re-takes it, and the direction that would reopen the argument.
It does not carry the magnitude, which goes in the commit and cannot go stale there. A threshold, a
count over a frozen corpus and a zero keep their digits, because in those the value _is_ the claim.

### When to write a reference

This is a restatement. Its home is `design@knowledge-architect@a-reference-claims-a-revisit`.

**Write a reference wherever the text would have to be revisited if the entry it names changed**:
reversed, closed, fired, abandoned or renamed. `cargo klarch show <ref>` prints every reference to
an entry, so what depends on an entry is computed and never listed by hand. A deleted or renamed
entry dangles every reference to it, and `cargo klarch check` reports each one, so the repair list
is the list of texts that depended on it. A reference that the entry's change would leave
unaffected is decoration, and it costs a repair for nothing.

| the text | names |
| --- | --- |
| a design head | the goal its argument derives a constraint from, a decision of another Component it depends on |
| an issue entry, under `Why it matters` | the decision it strains, and the goal it threatens when it does directly |
| a guard, a workaround, a stub or a test that pins behaviour an open entry describes | the issue it exists because of, in the comment at the site |
| a tripwire | the decision it guards |
| a rejected alternative | the decision it lost to |
| a commit message | every entry it opens, closes, reverses or argues from |
| a restatement of a directive | its home |

A reference in prose is checked wherever it stands, a Rust comment and a fenced block included. So
a comment in code that names an issue is reached when the issue closes. Never keep a hand-written
list of what references an entry: `show` computes it.

### Plan documents

`path@knowledge-architect@docs/plans/` is this repository's plans directory. It holds a README and
two homes: specs/, one file per spec for the work of one PR, and milestones/, one directory per
milestone for work across several PRs. Plan documents are committed on the work's branch, except a
milestone document whose first step changes what the gates check: it is merged in a pull request of its own before that step, because
the gates as the step changes them would judge the commit that added the document, per the
per-commit rule of `design@knowledge-architect@git-flow`. A correction is applied in place, so a
partial reading cannot mislead. A plan document is deleted in the commit that completes its last
harvest, and that commit's message cites it by its kind. This is a restatement; its homes are
`design@agent-skills@document-vocabulary`, `design@agent-skills@spec-leaves-at-landing` and
`design@agent-skills@milestone-lands-before-gate-change`, and
the procedure is the installed `knowledge-architect-planning`.

## Verify before relying on anything

A document can describe the shape the code was committed at rather than the shape it has. Some of
the core's documents were first written in another project, thaum; a figure measured there names
thaum as its source.
Confidence is not correlated with correctness: the more obvious a behaviour looks, the less likely
anyone has checked it.

**Two classes of statement, and the instruction differs.**

- **Intent**: the root's design home, and a Component's design home. This is authority. Do not
  verify it against the code. Verify the code against it. A divergence is a defect in one of them.
  Say which, open an entry, and stop. A divergence is never licence to follow the code.
- **A claim about the code as it stands**: an invariant in a scoped `CLAUDE.md`, a doc comment, a
  name, a return value, an open issue's diagnosis. This goes stale. Check it before relying on it.

For the second class, before relying on how anything behaves (an API, a script, a file format, a
test mechanism, an open issue's diagnosis; the list is illustrative, not a boundary):

- **Establish it from the implementation, or from a run you observe.** Names, doc comments and
  prose are leads to check, never evidence.
- **Follow the whole path, not the entry point**: what calls it, what it returns on success _and_
  on failure, and what the caller does with that.
- **One confirming case does not establish a convention.** Widen the check whenever the answer
  generalises.
- **Report the evidence with the conclusion**, and label anything unverified as an assumption. A
  plausible mechanism is not a finding.
- **Re-verify a conclusion before acting on it**, and correct it in place the moment evidence
  contradicts it.
- **A transfer from another project names the property it depends on, and checks that the property
  holds here.** Naming the pattern is not enough.

**Evidence here is textual and cheap**: a verbatim reading of a file, a measurement over a tree, a
failing test. Build it rather than trading intuitions.

**Before touching the code of a Component, ground in its own `path@*@CLAUDE.md`, its design home,
`path@*@docs/open-issues/` and `path@*@docs/tripwires.md` first.** They are closer to the code than
anything in the root's documents.

### Verify a claim before writing it

The same principle, pointed the other way. **A checkable claim is checked before it is written
down.** Check each of these against the tree at the moment you write the sentence.

- **Every number comes from a measurement you can point to, with the machine named.** Re-take a
  figure whose instrument you changed in the same commit.
- **A count is re-counted**: files, lines, entries, references, occurrences.
- **"A grep returns nothing", "the only hit in the tree"**: run it, against the revision the
  sentence describes.
- **"X already says this", "the skill requires that"**: open X and read it.
- **A slug, a path, a section, a run identifier**: resolve it.
- **"This is fixed", "that entry is closed"**: diff it.

**A commit message is a document for this purpose.** It is durable, and `cargo klarch commits`
reads only its references. A measurement, a count or a claim about the tree in a message is checked
by nobody but you.

**This also applies to answers given to the owner in the conversation.**

### Check whether it is already known

Before diagnosing anything as a problem, or reporting one you met while doing something else, find
out whether it is already recorded.

```sh
cargo klarch issues               # every issue entry in the repository, one row each
cargo klarch issues <anchor>      # the entries of one Component or location
cargo klarch issues <text>        # only the rows whose id or title holds that text
cargo klarch issues --kind <kind> # only the entries of one kind
cargo klarch tripwires            # every tripwire entry, and the decision each guards
cargo klarch show <ref>           # one entry in full, and every reference to it
```

The issue and tripwire registers are spread over every Component and the agent-config location, so
grepping the one you happen to think of is not the check. The three commands read the same entity
table that `cargo klarch check` resolves against. A recorded entry usually says more than a fresh
diagnosis will: the measurement already taken, what was ruled out, and often why the work was left
undone on purpose. The `knowledge-architect-issue-tracking` skill says what to do with what
you find, either way.

### Precedent is not authority

The instructions that apply are the ones in this file, in the skills, and in the scoped `CLAUDE.md`
next to what you are editing. What already exists in the repository is evidence of what was done,
not of what should be done.

- **"The existing ones do it this way" is not an argument, and neither is how many of them do.**
  Prevalence records history, not endorsement. Where an instruction here and the surrounding
  practice disagree, the instruction wins. If you think the instruction is wrong, say so and argue
  it. Changing an instruction is a normal move. Silently following the older practice is not.
- **Match local idiom only where the instructions are silent.** Naming and the shape of the
  surrounding prose are worth matching, so a diff stays readable. An instruction stated here is not
  idiom, and no quantity of surrounding text overrides it.
- **Do not spread non-conformance while working around it.** Starting from an existing document or
  entry copies its defects with it. Read what you copied against the instructions before extending
  it.
- **Non-conformance met outside the task takes the first of the primer's four cases that
  applies.** This is a restatement; its home is `design@agent-skills@primer-content`.
  1. It bears on the current work: stop and present it to the owner at the top of the turn, with a
     default.
  2. Its fix is checkable from the diff alone, because it changes no behaviour, no decision and no
     test outcome: fix it in a commit of its own.
  3. Its `Why it matters` and its `What would close it` can be written: open an issue.
  4. Otherwise: name it, with why it is dropped.

  The turn ends with a "Met outside the task" section listing every item and its outcome.

## Verify mechanically

The gates are how a session checks that it broke nothing project-wide. **One command runs every
gate, and its exit code is trustworthy:**

```sh
cargo x gates
```

It runs five gates in cost order: `fmt`, `check` (`cargo klarch check`), `commits`
(`cargo klarch commits origin/main..HEAD`), `clippy` with `-D warnings`, and `test`. It does not
stop at a failure unless `--fail-fast` asks for it. It writes each gate's complete output under
`path@knowledge-architect@target/gates/`, and prints one verdict line per gate plus a distilled
extract per failure; when the extract is not enough, the named log holds every byte. CI runs the
same tool on every ready pull request, as
`cargo --locked x gates --locked --fail-fast --require-rebased --full`. `--locked` makes lock drift
fail, and `--require-rebased` adds a sixth gate, `rebased`, first, which fails unless HEAD contains
origin/main. Exit 0 means every gate passed. The gate list's primary home is
`design@xtask@gates-list-primary-home`, and `path@xtask@README.md` is the usage. This is a
restatement.

**Do not filter the tool's output through pipes.** A hand-built filter pipeline hides lines and
loses the exit code. `cargo x gates` refuses a pipe on stdout for that reason, outside GitHub
Actions, per `design@gates@verdict-from-exit-codes`. When the text is wanted, redirect it to a
file.

`cargo klarch` runs the checker built from this checkout, in release mode. A single gate is still
run by hand when that is cheaper:

```sh
cargo fmt --all --check
cargo klarch check
cargo klarch commits origin/main..HEAD
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

**Each fails by its exit code.** `cargo klarch check` also states its verdict on its last line:
`PASSED: no findings` or `FAILED: n findings above`. Its summary block prints whether or not it
found anything, so the middle of its output does not tell you whether it passed.

**Never chain a command on a verdict that went through a pipe.** In
`cargo klarch check | tail -n 10 && git commit`, the shell takes the exit status of `tail`, so the
commit runs whatever the check found. Run `cargo klarch check` and `cargo klarch commits` bare. To
bound a long report, redirect it to a file, read its last line, and chain nothing on the
redirection's command.

`cargo klarch check` is one walk over every live document, in four phases, per
`design@core@phases-gate-the-report`: the manifest resolved, the tree read against it, the entity
table built, then every check. It stops at the first phase that finds anything, prints that phase
alone, and says which phases were not judged, because a finding computed over an incomplete model
is unreliable in both directions. The last phase runs three checks, and nothing selects a subset
of them:

| check | judges |
| --- | --- |
| `generated` | every generated index against its regeneration |
| `references` | every reference against the entity table, the two retired forms, the path to anchor |
| `registers` | the shape of what each anchor carries: a file register's entries, groups, README and index, and a directory home's links |

What this project declares conformant, and what it exempts, is the manifest at the root.

**Which gates you owe depends on what you touched.** Anything that writes a reference or touches a
register entry owes `cargo klarch check`. Any change to the Rust source owes fmt, clippy and the
test suite. Every commit owes `commits`: its message and its tree. **Run `cargo klarch check`
before each commit, which judges the tree, and `cargo klarch commits HEAD~1..HEAD` after it, which
judges the message.** A commit that fails either is repaired by an amend while it is the newest,
and by a history edit after, both with a clean tree per `## Git` point 2. A branch about to merge
owes all of them: one `cargo x gates --require-rebased`. CI runs them on every push to a ready pull
request.

## Skills

**A skill is one activity**: the scope over which a complete set of procedures makes sense. Read
the matching skill before doing that kind of work.

**This repository installs its own skills**, with `cargo klarch install-agent-skills`, and
commits them under .claude. The harness lists each installed skill with its description, so a
session finds which applies there. An axis of a review that has an installed reviewer agent is
dispatched as that agent, and any other axis as a fresh general-purpose subagent, per
`knowledge-architect-review`.

**This repository's own skills and agents take the prefix `klarch-`**, not the project's name: a
name beginning with `knowledge-architect-` is the installer's, and the install would delete it.
This is a restatement; its home is `design@knowledge-architect@klarch-prefix`.

**A retrospective's findings stay in this repository.** Here the project is also the workflow's
upstream, so both files of `knowledge-architect-retrospective` become entries in this repository's
own issue registers, on the owner's word, rather than an issue on GitHub. This is the owner's
standing direction under that skill; its home is
`design@knowledge-architect@retrospective-findings-stay-here`.

**The routing table**: what this repository adds to an installed skill or agent. It holds no row.
**This repository's own skills, `klarch-development` and `klarch-release`, add to no installed
skill**, and its one agent, `klarch-changelog-reviewer`, is dispatched by `klarch-release`; the
harness lists each with its description.

## Git

Linear history, no merge commits, and no direct push to main. This section is a restatement. Its home is
`design@knowledge-architect@git-flow`.

1. **All work happens on a branch.**

- No exception by size or kind.
- The branch covers the full work: design, implementation, review and fixes, cleanup.
- Once it holds a first commit, the branch is pushed and a **draft** pull request is opened for
  it: `git push -u origin <branch>`, then `gh pr create --draft`. CI does not run on a draft.
- **Every commit of the branch must pass the check under the branch tip's checker.** The
  `commits` gate judges each commit's tree with the tip's binary, and refuses a tree with no
  manifest or with findings. So a step whose intermediate trees cannot pass lands as one commit,
  squashed before review. Review repairs are new commits after it, and each one passes. A repair
  that would leave an earlier commit failing, such as one that changes what the checker judges, is
  folded into the earliest commit it repairs instead, per point 2.
- **The branch writes its CHANGELOG.md entries**, in the `Next release` section, for each change
  that passes one of three tests, under the subsection of that test:
  - **Migration**: a consumer must change something in its own files; one entry per thing, saying
    what. Running the install of the agent skills again is never an entry.
  - **New features**: a consumer can start using something new.
  - **Workflow**: a change to the installed skills that a person watching agent sessions would see:
    a new or removed action, file, commit, pull-request shape or question. A rewording is not one.

  An entry reads ``- `<surface>`, <class>: <what>``. The surface is one of `checks`, `cli`,
  `manifest`, `library`, `agent-skills`, `gates`, and entries are sorted in that order. The class is
  patch for a skill change or a code change that changes no check and no command, minor for a check
  added or made stricter or an addition that changes nothing existing, major for a change that may
  require a change in a project or breaks a command or the library API. A change with no entry is
  at most a patch. This is a restatement; its homes are `design@knowledge-architect@changelog-entries`
  and `design@knowledge-architect@versioning-policy`.

2. **No operation that can lose content, committed or not.**

- The hazard is **content loss**, in two forms: an operation that removes uncommitted content from
  the working tree, and a history edit that drops a commit no other reference holds.
- **While uncommitted changes exist in the tree, run no operation that touches history or restores
  files.** Examples (non-exhaustive): `reset --hard`, `checkout -- <path>`, `restore`, `clean`,
  `stash`, `commit --amend`, `rebase`.
- **Take extra care when applying a mutation that will need reverting.** Stage the file before
  applying the mutation, for example, so the revert does not depend on a restore.
- **With a clean tree, editing the branch's own history is an ordinary move**: rewording or
  amending a commit message, squashing, rebasing. It is bounded by two verifications before the
  old head is dropped: that the tree was clean, and that no content was lost. For a rewrite that
  keeps the base, `git diff <old-head> HEAD` is empty. For a rebase onto a moved base, compare the
  branch's own delta instead: `git range-diff <old-base>..<old-head> <new-base>..HEAD` reports
  every commit carried over.
- **main's history is never rewritten.** It is the shared trunk. A pushed branch of your own may
  be rewritten and force-pushed, since origin holds the old head until then. A branch that a live
  worktree has checked out is rewritten only after that worktree is removed.
- A repair from a review is a new commit, which edits no history. The exception is a repair that
  would leave an earlier commit failing under the branch tip's checker: it is folded into the
  earliest commit it repairs, with a clean tree, and the commit that records the review says what
  was folded.
- To test a previous state of the project, create a worktree in a place where it pollutes
  nothing, such as the worktrees directory under .claude, which is ignored. Do not use
  `git stash` or another operation that can lose content. Remove the worktree and its branch once
  done.
- **Confirm that what you committed is what you wrote** before moving on. A reverted file is a
  file that once passed.

3. **The branch is rebased on main before review and merge.** GitHub's ruleset refuses to merge a
   pull request whose branch is behind main; the local check below finds it earlier.

```sh
git fetch origin main
git merge-base --is-ancestor origin/main HEAD    # if false, rebase
```

4. **Work is reviewed before any merge to main.**

- Use `knowledge-architect-review` before the merge.
- The axes come from the dispatching activity's own skill.
- Critical findings are repaired before the merge.
- The commit that lands the repairs says what was reviewed and what was decided. Where every
  repair was folded, a commit of its own after them says it.
- Any finding not repaired becomes an issue entry, per `knowledge-architect-issue-tracking`.
- Once the branch is rebased and the repairs are pushed, mark the pull request ready:
  `gh pr ready`. That starts CI, and every later push re-runs it. First check that GitHub has
  taken the push: `gh pr view <branch> --json headRefOid` must equal `git rev-parse HEAD`. A pull
  request marked ready before that runs CI on its previous head, and a push that landed while it
  was a draft is skipped, so no run covers the current head.

5. **When CI passed on the current head, merge the pull request through GitHub.** main accepts
   no direct push: a ruleset requires a pull request whose branch is up to date with main and
   whose CI check passed, and a linear history. The merge predicate:

```sh
git fetch origin main
git merge-base --is-ancestor origin/main HEAD    # still true, or go back to point 3
gh pr view <branch> --json isDraft,headRefOid    # isDraft false, headRefOid equal to `git rev-parse HEAD`
gh pr checks <branch>                            # exit 0: every check passed on that head
gh pr merge <branch> --rebase --delete-branch
git checkout main && git pull --ff-only && git branch -D <branch>
```

- The draft flag is read here because a job skipped on a draft reports `skipped`, which GitHub
  counts as passing.
- GitHub's rebase merge gives main the branch's commits with the same trees and messages, and new
  SHAs, as a probe repository measured. So **neither a commit message nor a document cites the
  SHA of a commit of its own branch**: name that commit by its subject. A SHA already on main may
  be cited. `cargo klarch commits` refuses such a citation, per
  `design@core@branch-shas-are-refused`.
- The local branch is deleted with `-D`: its commits are not ancestors of main, since their SHAs
  differ.
- A merge to main publishes nothing. A release is a separate procedure, per
  `design@knowledge-architect@publish-after-merge`.

## Release status

**The crates are published on crates.io, and at least one consumer is known.**

- **The project stays at 0.x, and breaking changes stay allowed**, until the owner's word, given
  only once the open issues of the design discussion that produced it are implemented, or at least
  argued thoroughly, per
  `design@knowledge-architect@stays-at-zero-x`. How each kind of change is versioned is
  `design@knowledge-architect@versioning-policy`.
- **Do not argue a decision on the grounds that changing it later would be breaking.** An argument
  that a change is expensive names the cost it has today, a consumer's migration included. This is
  a restatement; its home is `design@knowledge-architect@no-future-breaking-cost-argument`.
