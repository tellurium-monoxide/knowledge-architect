# Changelog

One section per released version, and one working section above them, `Next release`, which the
release renames to its version. Which change gets an entry, under which subsection, and with which
class is `design@knowledge-architect@changelog-entries`. Inside a subsection, entries are sorted by
surface, in the order `checks`, `cli`, `manifest`, `library`, `agent-skills`, `gates`; an empty
subsection is omitted.

## Next release

### Migration

- `checks`, minor: under the `claude` harness, every level-two heading of the root CLAUDE.md, of
  each project skill, `.claude/skills/<name>/SKILL.md`, and of each project agent,
  `.claude/agents/<file>.md`, ends with a slug, two hashes and the id in backticks. A project adds
  one to each such heading; this holds for mock projects serving `claude` too.
- `checks`, minor: a project skill's directory and a project agent's file are named in lower-case
  words joined by hyphens, and a frontmatter `name`, where one is set, equals that name. A
  skill's directory is not `synced`, nor begins `anthropic-skills`, which the harness does not
  load.
- `checks`, minor: under the `claude` harness, a backticked span that is exactly the name of a
  skill or an agent, installed or the project's own, is reported. A project writes it as the
  reference, `skill@<name>` or `agent@<name>`; this holds for mock projects serving `claude` too.
- `cli`, major: `check --fix` refuses, exit 2, or 1 where it already installed agent files, when
  the index differs from HEAD and a generated file it would write differs from the one the staged
  tree needs, or when the staged tree's generated files cannot be computed, and it refuses
  `--staged`. A script or a CI step that runs `check --fix` over a partly
  staged index handles exit 2, or runs `index --staged` first.
- `manifest`, major: a Component, a location or a plan named after a kind, such as `design`,
  `issue`, `path`, a declared register or one of `skill`, `agent`, `primer` and `instructions`, is
  refused, and so is a register named after one of those four. A project renames it.
- `library`, major: `document::Observation` has a variant `BareName`, a backticked span that is
  one word in the id grammar. Code that matches the enum exhaustively adds an arm.
- `library`, major: `extension::Tree::Commit(&CommitTree)` is `Tree::Snapshot(&Snapshot)`, and
  `Snapshot::revision` says whether the snapshot is a commit's tree or the one git's index would
  commit. An extension renames the variant and the type; under `check --staged` it is prepared
  over a snapshot with `Purpose::Check`, and under `index --staged` and the comparison of
  `check --fix` with `Purpose::Index`.
- `library`, major: `cli::Command::Index` carries `IndexArgs`, and `cli::CheckArgs` has the field
  `staged`. Code that builds either by name adds the field.
- `agent-skills`, patch: the installed skills and agents no longer number their sections, and cite
  each other's sections by reference, as `skill@<name>@<slug>`. A project's own text that cites a
  section of an installed skill or agent by its number cites it by its reference instead.
- `agent-skills`, patch: the decision-recording skill's section entry-tests leaves it; its content
  is in the primer's new section on design heads. A project's text that cites that section of the
  skill cites `primer@design-heads` instead.
- `agent-skills`, patch: the design homes and the rejected alternatives a project holds are brought
  to the rules of `primer@design-heads` and to its new entry test 2, by running the design-record
  axis, `skill@knowledge-architect-project-audit@design-record-axis`, after the pin moves.

### New features

- `checks`, minor: under the `claude` harness, a skill, an agent and their sections, and a section
  of the primer or of the root CLAUDE.md, are cited with no anchor: `skill@<name>`,
  `skill@<name>@<slug>`, `agent@<name>`, `agent@<name>@<slug>`, `primer@<slug>` and
  `instructions@<slug>`. Each resolves against the installed copies and the project's own files,
  and `show` prints each.
- `checks`, minor: a slug at a level-two heading of a project skill or agent defines a section, where
  it was reported as a misplaced definition.
- `cli`, minor: `check --staged` judges the tree git's index would commit, HEAD's tree with the
  staged changes, by `check`'s rules, and prints a `tree: staged` line; it says when nothing is
  staged.
- `cli`, minor: `index --staged` writes the generated files the staged tree needs into git's index,
  and touches no working-tree file, for a commit of part of the working tree.
- `library`, minor: `extension::Tree` is `Clone` and `Copy`.
- `agent-skills`, minor: `skill@knowledge-architect-project-audit` audits a whole project on one
  axis, as opposed to a review of a diff, and its first axis, the design-record axis, re-applies
  the rules on design heads and on rejected alternatives to every entry; the agent
  `agent@knowledge-architect-design-record-auditor` drafts its verdicts, one per entry.

### Workflow

- `agent-skills`, patch: every level-two heading of the installed skills, agents and primer carries
  a slug, and the setup skill and the agent-configuration skill state that the root CLAUDE.md and a
  project's own skills and agents carry one too, and that a subagent's file name equals its
  frontmatter `name`.
- `agent-skills`, patch: the setup skill recommends `check --staged` after staging and before each
  commit, and the issue-tracking and planning skills name `index --staged` and `check --staged` for
  a commit of part of the working tree.
- `agent-skills`, patch: the rules on what a design head records and how move into one section of
  the primer, `primer@design-heads`, which every session holds; the decision-recording skill keeps
  the procedures of recording and opens by asking for that section to be read again, and the other
  installed skills and agents point to it.
- `agent-skills`, patch: a head's title states the rule that decided, and its body names the members
  built; a head holds one decision, split where two parts lose to different nearest rivals; a head
  stands on its argument and cites the owner only for what came from the owner; a member the
  argument covers is recorded directly, and one the owner's words name goes to the owner as one
  proposal; a head a change touches is brought to these rules.
- `agent-skills`, patch: before a head is written from an approval with no argument behind it, the
  decision-recording skill searches for its arguments, costs and rivals, and an even result goes to
  the owner as a tie.
- `agent-skills`, patch: a directive is restated only where the restatement is no longer than a
  pointer to it, one sentence; a longer one is a pointer to its home, read whole. The routing
  reviewer reports a longer restatement, and the design skill's in-change path reads the section on
  design heads again before its grounding.
- `agent-skills`, patch: entry test 2 counts each text that states an instruction as one site of a
  decision about agent-facing text, so a decision one text states has one site, and its reason
  lives beside the instruction rather than in a head.
- `agent-skills`, patch: moving the pin runs the audit axis a Migration entry of a version crossed
  cites, after the pin's commit, in a branch of its own; the milestone that moves an adopting
  project's existing documents ends by running the design-record axis.
- `agent-skills`, patch: an entry of the rejected alternatives leaves its file when it is reopened
  and chosen, or when the owner rules that it fails every recording test.

## 0.5.0

### Migration

- `checks`, minor: the second section every spec of specs/ and every milestone document owes is
  titled `How the work is done`, where it was `How a step is worked`. A project renames that
  heading in each of its plan documents; this holds for mock projects too.

### Workflow

- `agent-skills`, patch: a milestone's part, one branch and one PR with its own spec, is called a
  slice, and "step" names any item of an implementation sequence. The planning skill says how a
  spec's work takes the procedure once, and how many commits a step takes is the session's to judge.
- `agent-skills`, patch: a spec's design audit runs only when the work does not start in the session
  where the discussion converged, or when commits other than the spec's own have landed on the main
  branch since the spec was written.
- `agent-skills`, patch: a plan document lands before its work only when the work changes what the
  per-commit gate checks; a change only the working-tree check sees, such as installed text, does
  not count.
- `agent-skills`, patch: the primer says the installed text leaves room to judge where it is
  silent, and the retrospective counts an instruction as missing only where the workflow needed it.
- `agent-skills`, patch: the rule on writing a pointer without backticks covers only a reference or
  a path in the checker's syntax; the retrospective's standing question asks about those alone.
- `agent-skills`, patch: the design skill grounds a requested change whose design is not settled,
  and for bounded work, which reverses nothing, earns no design head and has no second defensible
  shape, sends one proposal with its default and waits for the owner's word, where it left the next
  step to the owner.
- `agent-skills`, patch: a decision earns a design head only when one of the rewritten entry tests
  passes: an interface others consume, a reason several sites or no site must respect, the
  behaviour of something outside the project, or the owner's own intent, which the agent asks the
  owner to confirm in a numbered question rather than judging it; the decision-record reviewer
  checks that such a head carries the owner's answer. The primer says a reason recorded at the code binds
  as intent below the design home and is read before code is removed, and the decision-record
  reviewer reports a diff that defeats one.
- `agent-skills`, patch: the transcript reviewer rates a misstated ruling critical only when it is
  reversed or changes what is built or a load-bearing decision, and the retrospective counts a minor one caught before the
  merge as no finding.
- `agent-skills`, patch: the retrospective names its files
  `<date>-<project>-<subject>.md` and `<date>-<project>-<subject>-klarch-workflow.md`, never
  overwrites one, and cites a finding by its id and its file's stem, as "W3 of
  <date>-<project>-<subject>-klarch-workflow".
- `agent-skills`, patch: a review item whose outcome turns on a ruling the owner has not given is
  put to the owner before its outcome is recorded; a defect a reviewer finds predating the change
  is routed by the primer's table of what is met outside the task, a fix checkable from the diff
  landing in a commit of its own that the review's record names; and the last transcript review
  waits until the owner has answered every ruling the repairs asked for.
- `agent-skills`, patch: the review of a slice or of a spec's work gives each finding one of the
  review skill's three outcomes, where the planning skill made every unrepaired finding an issue
  entry.
- `agent-skills`, patch: a dispatcher names a scratch directory of its own to each subagent sent
  together with others, and the standing-entry searcher writes its working files there.
- `agent-skills`, patch: moving the pin runs the project's gates command, or the check and the
  tests, before committing; a project writing an extension's tests reads the checker crate's
  section on testing an extension first.
- `agent-skills`, patch: on the design skill's in-change path, where a decision lands is proposed
  only after decision-recording decides its Component, for a reversal, and whether it earns a head.
- `agent-skills`, patch: the retrospective gives the owner the full path of each file it writes,
  and no longer prints either file into the conversation, nor the workflow's file when `gh` fails.
- `agent-skills`, patch: a session loads the decision-recording skill before any write into a design
  home, with no condition to judge first, as the skill's description and the primer say; a session
  editing a skill, an agent or a `CLAUDE.md` also loads it before editing a text whose behaviour a
  head describes, as the agent-configuration skill says.
- `agent-skills`, patch: a review repair that adds, removes or reverses a design head is reviewed
  by the decision-record axis whichever axis asked for it, and no other repair is, where any
  transcript-review repair that made a decision was; decision-recording's finishing step applies
  the entry tests again once the change is written.
- `agent-skills`, patch: a new reviewer, `agent@knowledge-architect-design-conformance-reviewer`,
  reads a plan document against the goals, the design heads and the rejected alternatives of every
  Component it touches, as the fourth reviewer the planning skill sends when a plan document is
  written or a decided shape in one is revised; the review skill's axis row of the standing-state
  reviewer is named "standing state".
- `agent-skills`, patch: the owner rules on each acceptance criterion, as on each tripwire, at the
  premortem; a criterion first proposed later waits as a default awaiting the owner and is not
  judged before the ruling.
- `agent-skills`, patch: the items the owner rules on one by one carry labels: `T<n>` for
  tripwires, `AC<n>` for acceptance criteria and `D<n>` for a plan document's defaults, and `Q<n>`
  where the label stays in the conversation, as for review items, the setup's proposed
  destinations, and an audit's gaps put to the owner in one question.
- `agent-skills`, patch: the design skill builds no discriminating evidence that the project's
  goals or design heads rule out, and sends the fork to the owner as a tie.
- `agent-skills`, patch: the primer says a word of the owner holds only as far as its premise; when
  the premise proves false, the session puts the corrected premise to the owner with a default, and
  proceeds on the default unless the owner answers otherwise, holding any part that cannot be undone.
- `agent-skills`, patch: the retrospective's standing question on the primer asks whether the
  session or one of its subagents acted as if a rule of the primer were absent, where it asked
  whether the primer reached the subagents.

## 0.4.0

### Migration

- `checks`, minor: a heading that markdown reads and the checker does not is reported in phase 2:
  a setext heading, a line underlined with `=` or `-`; a heading after a list or block-quote marker
  on its line; and a heading with no text. A slug on a line indented four spaces or more, or by a
  tab, or after `#` marks followed by a no-break space, defines nothing and is reported as written
  in the middle of a line. A project rewrites each such heading as a line that opens with `#`
  marks, a space and text.
- `cli`, patch: the summary of `commits` reads `<n> commits, <p> passed, <f> failed`, where it read
  `<j> judged, <f> failed`, and each commit's line reads `passed` where it read `judged`. A commit
  whose message alone carries findings now counts as failed, and its line names each source of its
  findings with its count. The verdict line and the exit code are unchanged. A script that reads
  the summary changes the words it matches.
- `manifest`, major: a register named `planned` is refused, as one named `path` is, since
  `planned` is now a kind of the reference grammar. A project that declares `[registers.planned]`
  renames it.

### New features

- `checks`, minor: a plan document cites a path its work will create as
  `planned@<anchor>@<path>`. The anchor and path follow the rules of a `path` reference, and the
  target must not exist; once it does, the finding asks for the `path` form. The form is legal in
  the plans directory only, and the repair of an unanchored path in a plan document names it.
- `cli`, minor: `show` resolves a `planned@<anchor>@<path>` reference, says whether its target now
  exists, and lists the plans that cite it. `show` on a path also lists its planned citations, and
  its citations written with or without a trailing slash.
- `library`, minor: `Finding` implements `Hash`, so findings can be collected in a set.
- `library`, minor: `CHECKER_VERSION` is the version of the core library, the value a project's
  `[project] checker-version` must hold. A test that copies a mock project out of its library's
  directory writes it into the copy's manifest.

### Workflow

- `agent-skills`, patch: the setup skill tells a test that copies a mock project out of its
  library's directory to write the library's `CHECKER_VERSION` into the copy's manifest.
- `agent-skills`, patch: the agent-configuration skill treats an edit that changes what an agent
  is told to do as a decision. Before writing it, the session searches the design homes and the
  goals for the behaviour the edited text describes. An edit that narrows or contradicts a head
  follows the decision-recording skill, and one that strains a goal goes to the owner.
- `agent-skills`, patch: the transcript reviewer's repairs land as additional commits, as many as
  their kinds need, and the review skill sends the decision-record axis at each of them that makes
  or reverses a decision.
- `agent-skills`, patch: a session that dispatches a transcript reviewer finds each transcript
  file by the message where the work begins, never by a session identifier, and the reviewer
  reports at once a named file that does not hold that message.
- `agent-skills`, patch: a finding is never repaired by writing its pointer in plain text. A pointer
  that no checked form expresses is written beside a reference to an issue entry of the project that
  records the missing form, and a gap of the checker gets such an entry in the project's own
  register.
- `agent-skills`, patch: the retrospective asks one more standing question: whether the session
  needed a pointer that no checked form expresses, and how it wrote it.
- `agent-skills`, patch: the planning skill cites a path a plan's work will create in the planned
  form, and the commit that creates the file converts it.
- `agent-skills`, patch: the review skill gives every item of a reviewer's report an outcome, a
  note outside its axis or an observation included, and the commit that records the review lists
  each one.
- `agent-skills`, patch: the decision-recording skill reads a head against the decision the owner
  approved before writing it, and puts to the owner a title or a body that widens, narrows or drops
  part of it.
- `agent-skills`, patch: the review skill gives each claim of a reviewer's finding its own outcome:
  repaired, opened as an issue, or judged to need nothing, with the reason.
- `agent-skills`, patch: a decision met during another task, such as an issue fix, goes through
  the design skill when it creates a design head, contradicts a statement of one, or takes one
  beyond what its title states. Its description names that symptom, and the decision-recording
  skill sends such a decision back to it before its text is written. An addition within what a
  head's title states, and a relocation or rewording that adds or removes no decision, are recorded
  directly.
- `agent-skills`, patch: the decision-recording skill keeps a head's title, as well as its slug,
  aligned with the full scope of its decision, and a title states a decision only while it is
  false of the nearest rival it beat.
- `agent-skills`, patch: the design skill's one-round path for a cheap decision becomes the
  in-change path. It is open when the decision's work, or its record, lands in the change under
  way, whatever the cost of reversal, and it sets no bound on threads or rounds. It writes no plan
  document: the commit message carries every thread with its final state, the owner's words
  verbatim and the rivals that lost. A cheap decision whose work comes later takes the full path,
  with a plan document.
- `agent-skills`, patch: a design thread is named by the decision it would record, not by the change
  it proposes. When a thread's slug misdescribes the approved decision, its design entry takes a
  slug that names the decision, and the plan document's harvest row, or the commit message on the
  in-change path, states the pair.
- `agent-skills`, patch: the design skill rewords a tripwire the owner ruled to record, when it is
  written or on a review finding, on the agent's judgement, and lists each change to the owner at
  the end of the turn.
- `agent-skills`, patch: on the design skill's in-change path, where the premortem runs, nothing of
  a decision is implemented or committed before it has run and the owner has ruled on its
  tripwires.
- `agent-skills`, patch: a new installed agent, `agent@knowledge-architect-standing-entry-searcher`,
  searches the issues and tripwires a piece of work bears on. The design skill dispatches it at the
  grounding of a discussion, and the planning skill at the design audit of a milestone step, one
  agent per group of at most 60 entries, in parallel. The session reads whole each entry it
  returns, where it read every issue and tripwire itself.
- `agent-skills`, patch: the design audit of a milestone step reads issues of every kind the
  step's code bears on, not only the deferred ones, and a step's grounding no longer reads the
  Component's issues and tripwires.
- `agent-skills`, patch: the standing-state reviewer reads every deferred issue's trigger against
  the change before every merge, as it reads every tripwire, and reports a trigger the change
  meets as a finding.
- `agent-skills`, patch: the review skill repairs a commit message carrying a mistake by amending it,
  or by a history edit of the branch, never by a correction written into a later commit's message.
- `agent-skills`, patch: the issue-tracking and decision-recording skills rewrite a sentence about
  the past whose reference dangles to state the present, or remove it, and never retarget it to the
  new name.

## 0.3.0

### Migration

- `checks`, minor: a generated index with one row says `1 entry`, not `1 entries`. A project with
  such an index runs `index`, or `check --fix`, once after moving to this version.
- `manifest`, major: every manifest carries `[project] checker-version`: the exact version of the
  checker the project runs, `"fixture"` in a mock project inside the directory of a library the
  binary links, and in a manifest a test only parses, or `"self"` where the checker is built from
  the project's own tree; it holds for mock projects too, an extension's included. A manifest
  without it, or a binary of another version, refuses every command with exit 2. A binary
  released before this one refuses a manifest carrying the key, as an unknown field.
- `library`, minor: `cli::refuse_another_version` refuses a run over a project whose
  `[project] checker-version` the binary does not satisfy. A project's maintenance crate or
  extension binary calls it in its `main`, after `cli::refuse_a_foreign_build` and before its
  match on the command, as the crate's template shows; without the call, that binary enforces no
  pin.

### New features

- `checks`, minor: a backticked path-shaped span is reported only when its first segment names a
  file or a directory the tree's listing holds, read from the root, from every anchor that is a
  directory and from the document's own directory. A media type, a unit, a git ref or another
  machine's path, such as `application/json` or `origin/main`, can be written in backticks.
- `checks`, minor: the retired slug reference is reported only when its id is an entry of the
  project, or its word an anchor or a kind. An issue number `#123` or a directive `#include` can be
  written in backticks.
- `cli`, minor: `check` names, above its verdict, each file holding a finding that git does not
  track, and says to commit it, or to move it out or ignore it.

### Workflow

- `agent-skills`, patch: the setup skill writes `[project] checker-version` into the smallest
  manifest, moves it with the pin, and its maintenance crate's `main` calls the version refusal.
- `agent-skills`, patch: a plan document may be merged to the main branch in a pull request of its
  own, whatever the time of its work; one whose work changes what the gates check is merged before
  that work, a spec as well as a milestone document.
- `agent-skills`, patch: when a project moves its pin, the setup skill fetches the new version and
  reads the changelog of each version crossed in the crate source cargo downloaded.
- `agent-skills`, patch: the setup skill's section for a Rust project ties every build to its
  checkout: a cargo `[env]` variable valued at the checkout's root, read by every library root and
  every target of a package with no library, and named by every build script. A target directory
  shared by two checkouts then rebuilds instead of running the other checkout's build.
- `agent-skills`, patch: the review skill has the dispatcher name each reviewer's worktree path in
  its brief, distinct for each reviewer.
- `agent-skills`, patch: the primer says that a design home prevails over diverging code until the
  issue closes, by the code meeting the head or by the head's reversal, never by following the code.
- `agent-skills`, patch: the planning skill keeps a check proposed before a plan document exists as
  a lead in its issue entry, which the design session judges.
- `agent-skills`, patch: after installing a new version, each open milestone document's procedure
  for working a step is restated from the new planning skill.
- `agent-skills`, patch: the record of a review names each reviewed commit by its subject, not by
  its SHA.
- `agent-skills`, patch: after editing a register, the issue-tracking, planning and setup skills
  run `check --fix`, which writes the stale generated files and checks, in place of `index` then
  `check`.
- `agent-skills`, patch: the setup skill asks the owner whether a Rust project with no workspace
  gains one, which it recommends, or runs a local install, and how to treat each finding the check
  reports in a file the project already has.

## 0.2.0

### Migration

- `checks`, major: every project carries docs/plans/ at its root, holding a `README.md`, specs/ and
  milestones/, and each of the two holds a `README.md` and a generated `index.md`. Move each spec
  into specs/ as one file, and each milestone into milestones/ as a directory holding its
  `README.md`; then run `index`. Any other file or directory directly under docs/plans/ is a
  finding.
- `checks`, major: a `path` citation of a plan document is refused. Cite a spec as
  `spec@plans@<id>`, a milestone as `milestone@plans@<id>` and a step of a milestone as
  `spec@<milestone>@<step>`. A citation from outside the plans directory of a file inside it is
  refused as reaching inside the anchor `plans`.
- `checks`, major: a plan document owes its sections: a spec of docs/plans/specs/ and a milestone's
  README the plan sections, in order, and a step spec the step sections. Add the missing ones.
- `checks`, minor: a `path@*@<path>` reference that only a declared location carries is reported,
  as the design of the generic form always said; anchor it at the location instead.
- `checks`, major: an item of a plan cited from outside it is refused; cite the plan whole. A
  backticked span whose head, before its first at sign, is `spec`, `milestone`, `thread`,
  `argument`, `criterion`, `acceptance`, `plans`, or the id of a spec or a milestone, is now a
  reference candidate.
- `manifest`, major: a declared anchor named `plans` and a `[registers.spec]` or
  `[registers.milestone]` table are refused, and so is `spec` or `milestone` in a location's
  `registers` list.
- `manifest`, major: a `[registers.thread]`, `[registers.argument]`, `[registers.criterion]` or
  `[registers.acceptance]` table is refused, and so is any of those names in a location's
  `registers` list. Rename a register of the project that carries one of them.
- `manifest`, major: a location declared at docs/plans/, and a component register whose `dir` is
  `plans`, are refused, because the tool constructs the anchor `plans` there. Move the location
  elsewhere, or give the register another `dir`.
- `library`, major: `cli::Command::Check` takes its arguments, `Command::Check(CheckArgs)`. Code
  that builds or matches the variant by name writes `Command::Check(CheckArgs { fix: false })`, or
  matches `Command::Check(_)`.
- `agent-skills`, major: the primer names the plans directory, docs/plans/, and the roadmap. Remove
  the row naming the plans directory from the project's own rows of the knowledge table, in its root
  `CLAUDE.md`.
- `agent-skills`, major: the agent `knowledge-architect-transcript-conformity-reviewer` is renamed
  `agent@knowledge-architect-transcript-reviewer`. Rename it wherever the project's own skills,
  agents or `CLAUDE.md` files name it.

### New features

- `checks`, minor: the plan items, `thread`, `argument`, `criterion` and `acceptance`: a
  level-three heading with its slug under the section of its kind, cited
  `<kind>@<plan>@<id>` from inside its plan, where each spec file and each milestone is an anchor.
- `checks`, minor: the reference kinds `spec` and `milestone`, carried by the anchor `plans`, and
  one anchor per milestone directory, carrying `spec` for its step specs. `show` prints a spec
  or a milestone document, and every reference to it.
- `checks`, minor: `commits` refuses a citation of a commit of its range by SHA, in a message or in
  a document of a commit's tree, where the manifest turns it on.
- `cli`, minor: `check --fix` applies every safe fix before checking: it installs the agent files
  when one is missing, differs or is no longer shipped, writes every stale or missing generated
  file, lists each, then runs the check. An upgrade that removes a shipped file takes a
  `git add` and a second run.
- `cli`, minor: `--version` prints the version of the checker that runs, from any directory.
- `manifest`, minor: `[commits] refuse-branch-shas`, off when absent.

### Workflow

- `agent-skills`, patch: a plan document names every text that references a decision its work
  reverses or rewrites, restatements in a `CLAUDE.md` or a skill included, with the step or the
  harvest that judges or updates each.
- `agent-skills`, patch: a plan document is committed before its reviews, and each repair lands
  after them.
- `agent-skills`, patch: a milestone document whose first step changes what the gates check lands in
  a merge of its own, before that step.
- `agent-skills`, patch: a design discussion's first round states which grounding commands ran, and
  a measured fact carries the command that re-takes it.
- `agent-skills`, patch: a retrospective names each finding by a letter and a number (W, C, P), and
  states the version of knowledge-architect the session used.
- `agent-skills`, patch: a design audit lists an answer that widens or narrows a ruling of the owner
  as a default awaiting the owner, ruled at the audit before its point is implemented.
- `agent-skills`, patch: a step's harvest is reviewed before the merge, on the decision-record,
  routing and standing-state axes, and by the transcript reviewer where the transcript is
  available.
- `agent-skills`, patch: a review repair that would leave an earlier commit of the branch failing
  the project's checks is folded into that commit by a history edit, and the review's record says
  so.
- `agent-skills`, patch: the planning skill writes a plan document's threads, arguments, criteria
  and acceptance criteria as items, ``### <statement> `##<id>` ``, under the section titles the
  checker matches, with Arguments right after Threads. It assembles the document from the discussion's
  transcript through a subagent, and the transcript reviewer reads every assembled document.
- `agent-skills`, patch: an optional roadmap, docs/roadmap.md, orders known work on the owner's
  word. The commit that adds a plan document rewrites the row of the issue it closes, and the commit
  that deletes a plan document removes its row.
- `agent-skills`, patch: a milestone's design is written into one spec per step from the start,
  the milestone document keeping what crosses steps. A step's design audit edits those documents in
  place, listing its findings in the commit message, and a new step found at an audit gets a spec
  of its own, listed as a scope change the owner rules on before it is implemented.
- `agent-skills`, patch: the transcript reviewer checks that everything a work's sessions
  established has a durable outcome, a reviewer's finding acted on included, and that no ruling of
  the owner is misstated. It no longer reports detail added inside a ruling. It runs once more,
  alone and last, before every merge to the main branch.
- `agent-skills`, patch: a step's design audit lists the tripwires and the `deferred` issues the
  step's planned code would fire, with `tripwires` and `issues --kind deferred`, so the owner rules
  on them before the code is written.
- `agent-skills`, patch: a load-bearing gap at an audit that is a choice among shapes stated in
  full, where the design skill's conditions for its one-round path hold, is ruled in one round with
  a default, several gaps in one question, and written into the step's documents; a gap that defeats
  what an approved thread rests on still opens a full design session. A harvest names in its commit
  each item of its row that the recording tests exclude.
- `agent-skills`, patch: the setup skill creates docs/plans/, with its `README.md`, specs/ and
  milestones/, when a project adopts the workflow, and no longer asks the owner for the plans
  directory's path.
- `agent-skills`, patch: the setup skill proposes, for a crate the project publishes, a short
  `CRATES-IO.md` as its crates.io page, named by `readme`, while its `README.md` stays
  repository-facing.
- `agent-skills`, patch: the commit that deletes a plan document cites it by its kind. A citation of
  it left in another plan is removed, and a `question` issue is opened on the citing plan.

## 0.1.0

- `manifest`: the manifest file is `knowledge-architect.toml`.
- `cli`: the binary is `klarch`.
- `library`: the library and the binary are one crate, `knowledge-architect`, imported as
  `knowledge_architect`.
- `manifest`: `[project] command` declares the command a project runs the checker by; `klarch`
  when absent. It is printed in messages and in generated index headers.
- `manifest`: `[agents] harness` declares the agent harnesses a project serves; `["claude"]` when
  absent. An empty list drops the CLAUDE.md requirement.
- `cli`: `install-agent-skills` writes the shipped agent files and removes unshipped ones from its
  namespace.
- `checks`: phase 2 reports an installed agent file missing, differing or unshipped, its deletion
  not staged, and a root CLAUDE.md that does not import a shipped primer or that the walk keeps
  out. `commits` does not compare installed files.
- `manifest`: a `command` that is empty or holds a line break or a backtick is refused.
- `cli`: `install-agent-skills` refuses a symbolic link on an owned path, and a manifest holding a
  refused declaration.
- `library`: the public API is the crate root and four modules: `cli` for a binary's `main`,
  `extension` for writing an extension, `document` for reading a document's parse, `testing`
  for running the core over a mock project. Every other module is private.
- `library`: `cli::Gathered` gathers what a run reads before any check; `complete_working_tree`
  returns it.
- `library`: `cli::Command`, `extension::Inputs`, `extension::ExtensionReport` and
  `extension::Resolution` are non-exhaustive.
- `library`: the `testing` feature is removed.
- `agent-skills`: the first installed skills, decision-recording and issue-tracking. The
  package ships its build script and its content/ directory, from which the list of shipped files
  is generated.
- `agent-skills`: the planning skill, and the transcript-conformity reviewer agent.
- `agent-skills`: review, and the reviewer agents standing-state, decision-record,
  routing, code-claims and cold-implementer.
- `agent-skills`: setup, agent-configuration, and the primer, which the project's root
  CLAUDE.md imports.
- `agent-skills`: goal-setting and the retrospective.
- `agent-skills`: goal-setting places a goal in the Component responsible for it; setup
  proposes two goals for a project's maintenance tool.
- `gates`: the package knowledge-architect-gates, the library that runs a project's merge gates,
  with the recommended list of a Rust project that uses the checker.
- `agent-skills`: setup gains a section for a Rust project: one maintenance crate pins the
  checker and runs the gates library, with its aliases and a continuous integration workflow.
- `agent-skills`: the design skill, forked from designing-together 0.6.0; the retrospective carries
  its expectation set.
- `agent-skills`: every installed skill is named by its activity as a noun: design,
  decision-recording, issue-tracking, review, setup, goal-setting, agent-configuration, planning,
  retrospective.
- `agent-skills`: the transcript-conformity reviewer tells a scope change from an agent's addition,
  and the planning and review skills keep an addition until the owner rules.
