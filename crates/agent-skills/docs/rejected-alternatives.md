# Rejected alternatives — agent skills

The alternatives that lost to a decision of `path@agent-skills@docs/design.md`, or to a goal where
the owner ruled that the winning position needs no entry, each with what it lost to and why.

**Recording every losing thread of a design discussion** — lost to
`design@agent-skills@losing-alternatives-filter`. `live`. The rejected alternatives would grow by
every thread of every discussion, including proposals nobody would raise again, and a reader would
search through them to find the few whose refutation cannot be derived again in one round. It is
kept here because a doubt remains: the owner judges the filter "still a little imperfect", and
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks for the alternatives that lost
and why, which the filter records only in part.

**A spec plus a separate detailed implementation plan, each step written out with its code** — lost
to `design@agent-skills@spec-and-milestone` and to `design@agent-skills@no-untested-snippet-is-authority`.
`live`. Both documents carry the same decisions and the second drifts from the first. Its
refutation also rests on the owner's account of real sessions, quoted
in `design@agent-skills@no-untested-snippet-is-authority`: implementers force such a plan's untested
snippets into the code at any cost and copy their comments verbatim. That observation cannot be
derived again in one discussion round, and workflows that write such plans are in common use, so the
alternative will be proposed again.

**Reporting a finding met outside the task without recording it** ("finish the task, then say what
you found") — lost to `design@agent-skills@primer-content`. `live`. The owner observed, over real
sessions, that an agent left a small defect neither fixed nor recorded, as a one-line mention inside
a long report, where it is easy to miss. That observation cannot be derived again in one discussion
round, and the rule it defeats is the common one.

**Writing the design skill so that weaker models can follow it** — lost to
`design@agent-skills@frontier-tier-only`. `live`. Rejected as a design goal: the workflow uses
weaker models as implementers conducted by a frontier model, not as the owner's counterpart in a
discussion. The scripted comparison that entry cites, which cannot be derived again in one
discussion round, showed a smaller model reproducing the format without the discipline.
Simplifying for a cheaper model is a change that will be proposed again.

**A delegated state, under which the owner hands a decision to the agent within a stated boundary,
or a note recording that judgement was handed over** — lost to
`goal@knowledge-architect@the-owner-decides`. `live`. A word that hands judgement over is an
approval like any other, as is approving every default on a quick read. The state was decided and
reversed in one revision of designing-together, after five of that revision's eight blocking review
findings fell on it: inside a grant, a correction whose own action could not be undone ran with no
word from the owner.

**Writing the discussion's ledger to a file during the discussion, so it survives compaction or a
new session** — lost to `design@agent-skills@ledger-from-transcript`. `live`. An artifact an agent
must update every round is one it forgets to update, and a stale ledger stated with confidence is
worse than none. Written with the least effort, as a draft, it answers the first reason only. A survey of agent systems that keep state files found two
properties a thread ledger lacks: the state is corroborated against something outside the model,
such as git, and every system that lets a model overwrite its state caps it hard. A thread ledger's
only corroborant is the conversation, which is what compaction removes. In the runs of the
designing-together skill, thread states were correct without a file.

**No roadmap, with known undesigned work listed only as issues** — lost to
`design@agent-skills@roadmap-orders-issues`. `live`. It held that a roadmap file would be a second
schedule beside the issue register, and two schedules drift; it accepted as its cost that the order
of future work had no home. Two facts defeated it. thaum, the first project to use the workflow,
kept a file of its next milestones as an exception to the installed skill: a need for order
observed in real use, which cannot be derived again in one discussion round. And a roadmap whose
rows are only checked references is no second schedule: the work stays in the issue register, and
a row dangles, reported by the check, when its work closes.

**A separate installed file holding the expectation sets, read only by those who judge findings** —
lost to `design@agent-skills@expectation-set-bounds-scope`. `live`. It would keep the sets out of
every skill's text and out of the retrospective, at the cost of a change to the install layout for
a file one skill reads. It wins if a second installed activity ever needs the sets, which
`issue@agent-skills@expectation-sets-for-the-installed-skills` may bring about.

**A transcript reviewer that reports every clause added inside a ruling, as the agent's addition
for the owner to contest** — lost to `design@agent-skills@transcript-reviewer-agent`. `live`. It
protected the owner's rulings down to their wording, at the cost of a list of additions in every
review. The owner, who had meant the reviewer to check durability from the start, judged from its
reports in real sessions that it was "focusing more on the wrong side, the small deviations from
what I approved", and ruled that a detail or a better wording inside a ruling needs no report.

**A milestone document holding the design of every slice, with slice specs holding only their entry**
— lost to `design@agent-skills@milestone-split-by-lifetime`. `live`. It kept the whole design in one
place to read. It lost to a measurement: the first milestone written that way, structured-plans,
had a README of 1,624 lines when it left, read whole at the grounding of every slice. That cannot be
derived again in one discussion round.

**Every decision that earns durable text goes through the design skill** — lost to
`design@agent-skills@new-or-reshaped-head-needs-design`. `live`. It sends a member a head's argument
covers, and a relocation that adds or removes no decision, to a full discussion whose outcome the
head already records; the owner ruled such a change does not need one, and every new member through
the design skill cost a reversal for an ordinary extension, per
`design@agent-skills@title-states-the-rule`. It is kept here because a doubt remains:
`tripwire@agent-skills@head-created-without-deliberation` watches whether a session adds to a head a
member its argument does not cover, with no deliberation, and this alternative is a candidate if it
does.

**One agent reading every issue and tripwire of the project for a piece of work** — lost to
`design@agent-skills@standing-entry-search-groups`. `live`. One agent's load grows linearly with the
project: the standing entries of thaum at its commit ef21314 hold 318,947 bytes. One agent reads
every entry whole only while its context holds them, and cannot judge so many entries reliably.
It is kept here because a doubt remains: no measurement shows that agents given groups of 60 entries
judge better, and one agent sees every entry beside every other, where a group sees only its own.

**One search agent per anchor** — lost to `design@agent-skills@standing-entry-search-groups`.
`live`. An anchor bounds nothing: in thaum at its commit ef21314, one Component holds 51.9% of the
bytes of the standing entries, measured with `git ls-files` over the issue entry files and the
tripwires homes and `wc -c`, summed by the anchor that holds each file.

**A separate installed skill for bounded problems** — lost to
`design@agent-skills@bounded-path-in-design`. `live`. Whether work is bounded is known only after
the design skill's grounding, so a separate skill would repeat that grounding to classify at all.
The owner had planned one, then proposed the path inside the design skill instead, on the argument
above: "IMO, there is no way to determine whether a task is "bounded work that does not change the
project's design" without going through the grounding steps of the design skill." It is kept here
because a doubt remains: `tripwire@agent-skills@a-shortcut-decision-earns-a-head` watches whether
the path lets a decision that earns a head skip its discussion, and this alternative is a candidate
if it fires.

**A second entry test that admits a decision because it constrains work not yet built** — lost to
`design@agent-skills@a-head-is-owed-by-an-entry-test`. `live`. Read against the 178 heads the design
homes held at commit 3a10fc9, which replaced it, by four subagents, it admitted nearly every head under a wide
reading, "governs future work", and almost none under a narrow one, two of them reporting on their
own that it does not discriminate. Designed work that is not built has its home in a plan document,
per `design@agent-skills@design-home-is-built-intent`, so the test's narrow reading had nothing left
to admit but a policy, which the test that replaced it admits as a decision that no code and no
text of the project states.

**A manifest declaration in the published checker that reads only the `%%` lines of a path** — lost
to `design@agent-skills@content-is-in-the-walk`. `live`. It would have checked the comments
while content/ stayed out of the walk, at the cost of a key in the manifest format that every
consumer's checker reads, built for this repository's use of the workflow for its own text. The
owner said of the installed files: "I don't want to cater too much to this use case in the
installed files"; applying it to the checker is the session's argument. Taking content/
into the walk checks the comments with no new format. The reason is about a declaration serving this
repository alone: a declaration of paths that must cite no entry, which
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` considers, would serve any project
that ships text, and is not covered by it.

**A checked form for a commit, resolved by its subject** — lost to
`design@agent-skills@plain-text-is-no-repair`. `live`. A commit of the
branch is named by its subject, and nothing checks that a commit with that subject exists. A
checked form would read the history on every run, a cost that grows with the age of the project,
to check names that git itself does not make permanent. A check bounded to the range `commits`
judges fails too: a later branch's documents and messages cite, by subject, commits that are no
longer in its range, so the check must either load every commit or issue no finding on them, and it
then only confirms citations that are already correct. Its value is low, and a subject written in
prose escapes no check: it names history, not the tree.

**The rules on design heads in one section of the decision-recording skill** — lost to
`design@agent-skills@one-home-for-head-rules`. `live`. A session reads a head at every grounding
and every plan, and nothing loads the skill at that moment, while every subagent holds the primer
in its initial context, which a probe of two subagent types measured in one harness. It is kept
here because the probe covers one harness, and another agent provider may not load the primer.

**A directive restated wherever it has to be delivered** — lost to
`design@agent-skills@restatement-size-test`. `live`. An inventory of the installed text at commit
c282b00 found about ten texts restating parts of the decision-recording skill, with nine
inconsistencies between them and it; it is re-taken by a fresh read-only subagent listing each
statement of a rule on heads outside its home at that commit. It is kept here
because a doubt remains: `tripwire@agent-skills@pointer-not-followed` watches whether a session
misses a directive whose restatement became a pointer, and this rule is the candidate if it does.

**Splitting a head where reversing one part leaves the other standing** — lost to
`design@agent-skills@one-decision-per-head`. `live`. Four audits of the design heads, run apart, found
that the test splits a rule from its own exception, which always passes it, and that parts losing
to different nearest rivals discriminate better.

**A head citing every ruling of the owner as its ground** — lost to
`design@agent-skills@head-ground-is-the-argument`. `live`. An audit of the heads added or rewritten
in the week entry test 4 was added, up to commit c282b00, found about two in five of their citations of the owner to be an approval of the
agent's position, often a batch word, which made an approved default read as the owner's intent. It
is kept here because a doubt remains: `tripwire@agent-skills@owner-intent-stripped` watches whether
the correction removes the owner's own words, and this alternative is the candidate if it does.

**A new member routed by the scope of the owner's approval, found in the history of each head** —
lost to `design@agent-skills@extension-follows-the-ground`. `live`. It needs the word that approved
a head found over the commits and plan documents that wrote it, where the head's ground is read from
the head itself. It is kept here because a doubt remains:
`tripwire@agent-skills@member-beyond-the-argument` watches whether a session records a member its
head's argument does not cover, and this alternative is the candidate if it does.

**A head's title stating the instance built, the members or the mechanism that exist** — lost to
`design@agent-skills@title-states-the-rule`. `live`. An audit of this repository's design heads at
commit c282b00 found the instance stated as the rule in 27 of its 189 heads, and its history holds
rewrites of such heads, with their slugs and tripwires renamed, that a member the argument already
admitted forced; the count is re-taken by a fresh read-only audit of the heads at that commit.

**An audit method with no calibration and no re-check** — lost to `design@agent-skills@audit-method`
for the re-check, and to `design@agent-skills@design-record-axis` for the calibration. `live`. On
the first audit of the design-record axis on this repository, the re-check of each pass found
violations the pass had missed or made, and two agents read one shared sample entry two ways in two
of the samples; a method that trusted one reading per entry would have left both. The audit's
commits, found with `git log --grep='design-record axis'`, carry the counts.

**The repair of a head only when a change touches it** — lost to
`design@agent-skills@audit-repairs-the-whole-record`. `live`. A repair on touch never reaches a
head no change touches, so a project's record stays under the old rules after its migration. It
costs the owner one proposal per head, at the moment a session already edits it, where a project
audit puts every item of its owner list to the owner at once: the first audit on this repository put
72 items to the owner over three passes. It is kept because a doubt remains on that cost to the
owner's reading time, which `goal@knowledge-architect@the-owner-decides` spends, and it is the
candidate if a project's owner finds an audit's lists too long to read.

**One generic auditor agent, briefed to read the axis's section of the skill** — lost to
`design@agent-skills@audit-agent-per-axis`. `live`. It puts the axis's standard behind a read the
brief must ask for. The head-rules sweep that preceded the audit skill ran that way, and the
discussion that chose the agent recorded its result, in its argument a16 of the deleted spec
design-record-audit: "Today's sweep drafts came out uneven where the brief was the only standard."

**A routing table in the project's root `CLAUDE.md`, one row per installed skill or agent a project
skill or agent adds to** — lost to `design@agent-skills@no-routing-table`. `live`. Every project
skill is already listed with its description each session, and a review's added axes are owed
through the dispatching activity's skill, so no row delivered anything those two did not. Its
"adds to" had to be widened for each new kind of link, and the first agentic-workflow audit of this
repository confirmed four findings that turned only on whether a row was owed.

**The agentic-workflow axis's corpus shared out among agents of one task, as the design-record
axis shares out its entries** — lost to `design@agent-skills@workflow-axis-lenses`. `live`. The
workflow functions as a whole, and the axis exists mainly to find two instructions in different
files that no single move satisfies; a contradiction across two files sent to different agents is
found by neither. It is kept here because a doubt remains:
`tripwire@agent-skills@auditor-skips-part-of-the-corpus` watches whether the corpus outgrows one
agent's reading, and this alternative is a candidate if it does.

**One workflow auditor agent per lens** — lost to `design@agent-skills@workflow-auditor-agent`.
`live`. Each agent would repeat the axis's shared standard, the corpus, the finding standard, the
tags and the draft's shape, which is about half of any one of them, and the owner did not want
"publishing too many agents and polluting the directory". The concern it answers, one agent holding
every lens drafting mostly under lenses other than its brief's, was counted over the drafts of the
first agentic-workflow audit of this repository: more than a third of one agent's drafts named
another lens at passes 1, 2 and 6. After the first, the owner kept one agent: "multiple agents
reporting the same findings is a stronger proof of a problem." That count measures what the
workflow auditor is told to do, since a finding of another lens met on the way is drafted under
that lens, so no tripwire watches it.

**One repair per finding, at its site** — lost to `design@agent-skills@root-cause-repair`. `live`.
It lost to a measurement on the first agentic-workflow audit of this repository: passes 1 and 2
repaired at their sites, 63 findings applied and 57 ruled by the owner, and the auditors' returns
of pass 3 put about 25 of its findings in the interactions of the rules those repairs added, by the
message of the pass-3 commit, "Pass 3 of the agentic-workflow axis stops the run: the count did not
fall, the repairs move to a method by cause". `design@agent-skills@capabilities-not-structure`
records the same effect from an earlier run.

**No bound on how much a pass's repairs add** — lost to `design@agent-skills@audit-net-growth`.
`live`. Passes 1 and 2 of the first agentic-workflow audit of this repository turned most of their
findings into added rules, which the auditors of pass 3 then found interacting, about 25 findings
by their returns, per the pass-3 commit named in the entry on one repair per finding.

**Three runs of the audit's saved workflow per pass, the session reading the drafts, the groupings
and the proposals between them** — lost to `design@agent-skills@one-run-pass`. `live`. It lost to a
measurement over the files of pass 4 of the first agentic-workflow audit of this repository, `wc -l`
over each set: the sort read 63 drafts, 3,738 lines, confirmed 36 findings and refuted 3, each
refutation one a verifier given the audit's commit messages can make; the merge read two
groupings, 1,074 lines, and made two choices of boundary; the confirmation read 23 proposals, 4,030
lines, and sent none back, by the session's own account, which no file records; the diff read before the commit, 1,290 lines, changed three wrapped
lines and one claim. The readings changed little and took the bulk of the session's context.

**The session reading every draft again after the run** — lost to
`design@agent-skills@one-run-pass`. `live`. In the sort of pass 4 of the first
agentic-workflow audit of this repository, the session refuted 3 of 63 drafts: 2 mechanically, a
kept item and a pair of texts one move satisfies, and 1 on the history of the audit, a pointer an
earlier ruling had judged. An adversarial verifier given the kept list and the audit's commit
messages makes all three. It is kept here because a doubt remains:
`tripwire@agent-skills@verifier-refutes-a-true-finding` watches whether a verifier refutes a true
finding the session's sample of refuted drafts does not meet, and this alternative is the candidate
if it does.

**One markdown file per audit axis holding the steps of the manual fallback** — lost to
`design@agent-skills@audit-axis-runs-a-workflow`. `live`. It is a second home for the steps the saved
workflow already holds, and pass 4 of the first agentic-workflow audit of this repository confirmed
such a drift between the skill's prose and the script: the script's repair prompt had narrowed "the
homes merged into one" to "the section of the skill that owns the moment", which F81 of the pass-4
commit, "Pass 4 of the agentic-workflow axis, run through the saved workflow: 36 findings
confirmed, 23 causes repaired", repaired.

**The audit's workflow script kept in a fenced block of the skill, passed to the Workflow tool
inline** — lost to `design@agent-skills@audit-axis-runs-a-workflow`. `live`. Probes of Claude Code's Workflow
tool, three files under the harness's workflows directory after a reload, recorded in the message
of the commit "The agentic-workflow axis's script ships as a saved workflow in the installer's
namespace", showed a saved workflow run by the name its `meta` declares, its `args` passed through,
and a file one directory deeper not found. A saved workflow is run by name with nothing to copy,
where an inline block lives in the skill as text no session needs to read.

**The plain count of confirmed findings, its fall judged pass by pass over its context** — lost to
`design@agent-skills@convergence-count-scope`. `live`. It lost to a measurement: each of pass 5's 38 findings
of the first agentic-workflow audit of this repository was classified by the commit that last wrote
the words carrying its defect, with `git blame -w -C` and a check that the words are absent at the
earlier commit. 8 sat on text the main branch held before the audit's branch, 16 on text the
repairs of passes 1 to 4 and the commits of the method by cause wrote, and 14 on text the method's
one-run rewrite wrote. The counts of two passes are taken over two different corpora, and each
firing of the plain count, at passes 1 to 3 and at 5, was explained by the same churn, argued by
hand.

**Fewer auditors on the two-readings lens, or a narrower scope for it** — lost to
`design@agent-skills@workflow-axis-lenses`. `live`. At pass 5 of the first agentic-workflow audit of
this repository, the verifiers and the session confirmed 37 of 38 findings, each lens with more
than one agent produced findings no other agent of that lens reported, and 14 of the 15 findings of
the two-readings class were confirmed, the session rejecting one, while 9 of the 15 sat on text the
audit's own branch had just written.

**Defects in the text a repair writes, left for the next pass to find** — lost to
`design@agent-skills@repair-proposals-audited`. `live`. By the classification of pass 5 of the
first agentic-workflow audit of this repository, given in the entry on the plain count of
confirmed findings, 16 of its 38 findings sat on text the repairs of the earlier passes and the
method's commits had written, and each such finding cost a whole pass to surface.

**A confirmed finding of two readings left to judgement where no text rules either act out** —
lost to `design@agent-skills@workflow-axis-finding-standard`. `live`. It lost to
`goal@knowledge-architect@agents-get-a-complete-workflow`: "Each instruction says what it asks
plainly enough to follow without guessing what it means". An instruction a reading confirms is read
two ways is covered by an instruction, so the goal leaves it to no judgement. Its own rivals, such
a finding dropped at admission and the session leaving it alone, fall with it. It is kept here
because a doubt remains on the cost: every repair is new text, and new text is where the next pass
finds defects, as `design@agent-skills@capabilities-not-structure` records. Applied to pass 5 of
the first agentic-workflow audit of this repository, the test it rested on, whether another
instruction, a design head or a goal rules one of the two acts out, would have left 6 of the 14
two-readings findings the session kept to judgement. It is the candidate if the owner restates the
goal, under `skill@knowledge-architect-goal-setting`, so that an instruction whose readings all
lead to acceptable acts meets it.

**The check of a repair against the design heads, the goals and the rejected alternatives left to
the session at the application** — lost to `design@agent-skills@repairs-read-against-the-record`.
`live`. The application comes after the owner's ruling on the owner list, so the owner would rule
on a repair without knowing it reverses a head. It is kept here because a doubt remains:
`tripwire@agent-skills@record-check-misses-a-head` watches whether the check, whose search for the
heads that describe a behaviour is a judgement, misses one, and where the check runs is then
reopened.

**The commit messages alone as the record of an audit's passes** — lost to
`design@agent-skills@committed-audit-report`. `live`. A message holds the outcomes, not the
evidence: the classification of pass 5's findings in the first agentic-workflow audit of this
repository needed that run's confirmed-findings file and drafts, which only a scratch directory
held. And a record carried by a message alone is lost when its commit changes no file, which the
rebase merge drops, as `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`
records from a probe repository.

**Divergent verdicts between a review and an audit as a finding class with no lens of its own** —
lost to `design@agent-skills@workflow-axis-lenses`. `live`. Over the drafts of passes 1 to 4 of
the first agentic-workflow audit of this repository, the eight whose titles pair a review with an
audit are about which axes review an audit's branch and when the transcript review runs; none
compares what the two count as a finding. A deliberate comparison, made on the owner's request
before that audit, had found the design-record auditor and the decision-record reviewer judging
rejected alternatives by two restatements of one rule, which the commit "The decision-record
reviewer and the design-record auditor judge the record by the same rules" aligned.
