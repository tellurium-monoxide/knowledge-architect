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

**A sweep that brings every head stating the instance as its rule to its rule at once** — lost to
`design@agent-skills@existing-heads-on-touch`. `live`. An audit of this repository's design heads
found about nine of them whose approval named their members, so the sweep would put that many
widenings to the owner at once; the count is re-taken by a fresh read-only audit of the heads and
the words that approved each.

**A head's title stating the instance built, the members or the mechanism that exist** — lost to
`design@agent-skills@title-states-the-rule`. `live`. An audit of this repository's design heads at
commit c282b00 found the instance stated as the rule in 27 of its 189 heads, and its history holds
rewrites of such heads, with their slugs and tripwires renamed, that a member the argument already
admitted forced; the count is re-taken by a fresh read-only audit of the heads at that commit.
