# Agent skills

This crate carries text, not behaviour; the core reads it to install and to check.
`path@agent-skills@content/` holds the shipped files in the install layout, and
`path@agent-skills@build.rs` generates the list that `path@agent-skills@src/lib.rs` exposes, each
entry the install path and the text, per `design@agent-skills@content-mirrors-the-install-layout`.
A file under content/ that the layout does not map fails the build.

**A change to content/ or snippets/ is installed in the same commit.** This repository installs its
own skills, and its check compares each installed file with the shipped text byte for byte. After
editing content/ or snippets/, run `cargo klarch install-agent-skills` and commit the installed copies under .claude
with the change, or the check fails.

**content/ is in the walk, and every reference in it is checked against this repository.** So a
reference to an entry of this repository passes here and would dangle in every installing project:
the text cites no entry, per `design@agent-skills@shipped-text-cites-no-entry`, and nothing checks
that yet, per `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. It names no
Component and no convention of this repository; it writes a path every conforming project holds as
`path@*@<path>` or `path@plans@<path>`, a path that varies by project and an illustration as a
placeholder in angle brackets, and the project's command as the install placeholder. It may cite the shipped set's own skills, agents, primer and their sections,
`skill@<name>@<slug>`, which every project serving `claude` holds. A level-two heading under
content/ ends with its section slug as a placeholder, `{{slug:<id>}}`, which the build renders. A skill names
another installed skill by its installed name, which carries the installer's prefix.

**A line whose first two characters are `%%` is a comment for this repository**, per
`design@agent-skills@shipped-text-line-comments`: it may cite design heads and issues, and the build
removes it. Indented, or inside a fenced block, it fails the build. **A literal the checker would
misread** goes in `SUBSTITUTIONS` of `path@agent-skills@build.rs`, with the reason in its comment,
and the text writes its placeholder; a row no text uses fails the build.

**The maintenance crate's main that the setup skill shows is a file under
`path@agent-skills@snippets/`**, which the skill names by a placeholder line,
`{{snippet:<file>}}`, and the build script inlines. Edit the file, not the skill. The example
`path@xtask@examples/setup_snippet.rs` compiles it. rustfmt does not reach an included file, so
`rustfmt --edition 2021 --check` it by hand after an edit. The checker reads it as Rust source:
its comments are prose, a reference in them that resolves here passes the check and ships, and the
text it ships cites no entry, as content/ does, per
`design@agent-skills@shipped-text-cites-no-entry`. The release's hand check reads the installed copies, which hold the snippet inlined.

**The primer reaches every session of every installing project**, so it holds only what every
session needs and no installed skill delivers, per `design@agent-skills@primer-limit`. A line that
restates a skill, or a convention of one project, does not go in it.

## Editing an installed skill or agent

An edit of an installed skill under `path@agent-skills@content/` passes the four tests below, in
order; an edit of an installed agent passes the last three, since an agent never works with the
owner and has no expectation set. This is a restatement; its homes are the entries and the skill named.

1. **Scope.** A finding about the owner's behaviour outside the skill's expectation set is not a
   gap, and no instruction is written for it. The sets are in §5 of
   `path@agent-skills@content/skills/retrospective/SKILL.md`; a skill not listed there has none yet.
   A finding that two installed instructions leave no move satisfying both is always in scope
   (`design@agent-skills@expectation-set-bounds-scope`).
2. **Necessity.** A contradiction, a broken trigger or a factual error is repaired on reading. An
   addition needs an observation from a real session, the owner's named lack, and a one-sentence
   mechanism; a predicted behaviour is parked as an issue (`design@agent-skills@additions-need-real-use`).
3. **Kind.** A missing capability is worth text; a conformance rule only where the default is
   systematically wrong; a structure imposed on the work, such as a fixed mapping between a unit of
   the work and a unit of its record, only where the work needs it
   (`design@agent-skills@capabilities-not-structure`).
4. **Built intent.** Grep the design homes and the goals for the behaviour the edited passage
   describes. An edit that writes into a design home, or edits a text whose behaviour a head
   describes, loads `knowledge-architect-decision-recording` before it is written; that skill judges
   whether it contradicts a head, outgrows its title, or earns text at all. That trigger's home is
   `design@agent-skills@design-home-write-loads-recording`. One that strains a goal goes to the
   owner, under `knowledge-architect-goal-setting`. The rest of this test restates the installed
   `knowledge-architect-agent-configuration`, §1, and it holds because
   `design@agent-skills@design-home-is-built-intent` makes the design home authority over the
   shipped text.

For the design skill, a finding about an intermediate table rather than the outcome is
not worth text (`design@agent-skills@outcome-over-display`). A rewording needs no design entry
(`design@agent-skills@instruction-record-is-minimal`).

The decisions about the skills are `path@agent-skills@docs/design.md`.
