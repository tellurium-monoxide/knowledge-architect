---
kind: defect
---
# A reference whose id carries a line suffix or a fragment is told to define its entry or repair its pointer, not to drop the suffix

## Summary

A backticked span shaped like a path, with no anchor, and with a `:<line>` or a `#<fragment>`
suffix, gets the repair "drop the line number or the fragment". The same suffix on an anchored
reference does not: a register kind's finding says to define an entry whose id holds the suffix,
a harness kind's finding says to end a heading with that slug, and a `path` reference's finding
says the target does not resolve while the file exists.

## Details

### What

Established by reading the code at the commit "The project audit counts convergence on old text,
checks each repair against the record, and commits each pass's report file", and by a run of the
binary built from that commit, made by the L7 draft of the sixth pass of the agentic-workflow
audit. All four sites are in `path@core@src/check/references.rs`:

- the `Observation::UnanchoredPath` arm appends ". A reference names a file: drop the line number
  or the fragment, and name the function or the heading in prose" when the span holds a `:` or a
  `#`. This is the one form whose message names the repair.
- `Resolution::Undefined` in `table` gives "define it in the <register> home of `<anchor>`, or
  repair the reference", whatever the id holds.
- `harness` gives one of three repairs, by the shape of the reference:
  - for a section of an existing skill or agent: "end a level-2 heading of the <kind> `<owner>`
    with that slug, or repair the reference";
  - for a section of the primer or the root CLAUDE.md: "end a level-2 heading of <document> with
    that slug, or repair the reference", <document> being "the primer" or "the root CLAUDE.md";
  - for a skill or an agent that does not exist: "name an existing <kind>, or repair the
    reference".
- `assert_target` gives "repair the pointer, or delete it; a path that does not resolve is a
  guess".

The reproduction, in any project whose manifest declares a Component `<c>` holding a goal
`<goal-id>` and a README.md, and which installs a skill `<skill>` with a section `<slug>`: one
document holding six spans, each in backticks, `path@<c>@README.md:3`, `path@<c>@README.md#top`,
`goal@<c>@<goal-id>#top`, `goal@<c>@<goal-id>:12`, `skill@<skill>@<slug>:3` and
`primer@<slug-of-a-primer-section>#x`, each placeholder replaced by a real name. The draft's run of
`cargo klarch check` over such a project gave six findings, and none names the suffix: two `path` findings with the `assert_target`
repair, two goal findings with the `table` repair, one with the skill-section repair and one with
the primer repair. The goal, the section and the file exist; only the suffix fails. The id grammar
`[a-z0-9]+(-[a-z0-9]+)*` refuses `:` and `#`, so the first repair the register and harness
findings name cannot be carried out. The draft's run gave, for the skill section reference, "end a
level-2 heading of the skill `demo-x` with that slug, or repair the reference".

Ruled out:

- that the installed text supplies the rule instead: a search of the files under
  `path@agent-config@skills/`, `path@agent-config@agents/`, the primer, the saved workflows and the
  root CLAUDE.md for `fragment`, `:line`, `line number` and `suffix` finds the cold-reader standard's
  "Never line numbers", about issue entries, the intake's `file:line` evidence lines, and the
  root's list of claims a commit message checks; none states the rule for a reference. The
  primer's reference paragraph says "a plain path under it", which says nothing for a register
  kind or a harness kind.

Reported by the L7 lens of the sixth pass of the agentic-workflow audit. The owner ruled in its
first pass, F23, that the rarer checker rules are delivered by their finding messages rather than
by the installed text, and narrowed its lens L7 in its second pass, F69: "an unstated checker rule
is a finding only where its finding message does not name its repair". This defect is what makes
that delivery fail for the anchored forms; the owner kept the ruling, with this entry, in the
sixth pass, F121.

### Why it matters

`design@core@an-unresolved-reference-is-reported` gives each way a reference resolves to nothing
the repair it needs, and `design@core@finding-names-the-repair` names each cause with its repair
where the repair depends on the cause: an undefined id and an id carrying a suffix need different
repairs. A session that writes evidence with a location, as an analysis under
`skill@klarch-retrospective-intake` asks, follows the message to the wrong segment, and no installed
text supplies the rule instead, by the owner's ruling. Against `goal@knowledge-architect@adoption-is-easy`:
the finding is what a consumer reads first.

### What would close it

A finding on an anchored reference whose id, or whose path after the anchor, holds a `:` followed
by digits or a `#`, names the repair the unanchored form names: drop the suffix, and name the
function or the heading in prose. The exact suffix shapes follow the boundary that
`design@core@every-path-names-its-anchor` sets for the unanchored form, settled under
`skill@klarch-development`. A test per resolution path (a register kind, a harness kind with each
of its three repairs, `path`) over a mock project where the entry or the file exists and only the
suffix fails.

The fix adds a case beside the five ways `design@core@an-unresolved-reference-is-reported` counts,
a member its argument covers, so the same change records it in that head's body.
`spec@plans@path-quickfixes` changes the repairs of the same findings in the same file and decides
that a span with a line suffix or a fragment is not respelled; whichever lands second reads the
other, and its tests that pin repair lines are checked against the new message.
