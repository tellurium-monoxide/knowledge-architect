# Shipped text comments: `%%` lines stripped at build, content/ in the walk, delivery substitutions

The spec of the second slice of `milestone@plans@load-bearing-records`. It holds what only this
slice builds; what crosses slices is in the milestone document. Every item it cites is defined
there, except its own acceptance criterion. It starts after slice 1 and the snippet branch have
merged.

**The repairs of the first three classes in the mapping table, and the narrowed head's use of
`path@*@<path>` for the directory shape, rest on the first default awaiting the owner in the
milestone document.** Until the owner rules on it, they are that default, not a decided shape.

## Builds

- **`%%` comments** in `path@agent-skills@build.rs`: a line of a file under content/ whose first
  two characters are `%%`, outside a fenced block, is removed whole before the snippets are
  inlined; a `%%` line inside a fenced block, or one with leading spaces, fails the build.
- **Delivery substitutions** in the same build script: a table of rows, each a placeholder, the
  literal it expands to, and a comment giving the reason the checker must not read that literal.
  The first row is `{{primer-import}}`, for the primer's import line. A substitution replaces its
  placeholder wherever it stands in a line, unlike a snippet, which replaces a whole line: the
  import line sits inside a sentence. It matches only the placeholders of its own table, so the
  GitHub Actions expressions of the setup skill, written `${{ … }}`, and the install's `{{command}}`
  are left as they are.
- **content/ back in the walk**: the line `"crates/agent-skills/content",` and its comment leave
  the `exclude` list of `path@knowledge-architect@knowledge-architect.toml`.
- **The 36 repairs** the walk probe reported, per the mapping table below.
- **The narrowed head of the shipped text** and the rewritten issue, per the harvest.
- **The installed copies** under .claude, by `cargo klarch install-agent-skills`, in the same commit
  as each change to content/.

## Claims

| claim | test that could refute it | how the test is shown to discriminate |
| --- | --- | --- |
| No `%%` line reaches an installed file | `acceptance@load-bearing-records@no-comment-line-ships` | a mutation of the build script that skips the strip ships the line, and the test fails |
| A `%%` line inside a fence, or indented, fails the build | the build itself, over a scratch copy of content/ holding each case | each case planted in a scratch worktree makes the build panic with a message naming the file and line; without the refusal the build passes |
| Stripping leaves the surrounding text unchanged | the installed copies: `cargo klarch install-agent-skills` reports the installed set already the shipped one after the first `%%` line is added to a file whose installed copy is committed | adding the `%%` line changes no byte of the installed file |
| A substitution row that no text uses fails the build | the build | a row whose placeholder is deleted from content/ makes the build panic |
| The import line the setup skill ships is the one the check looks for | a test in the core over `knowledge_architect_agent_skills::FILES`: the installed setup skill holds `IMPORT_LINE` of `path@core@src/agents.rs`, the only other copy of that literal, which the agent-skills build script cannot read since the core depends on agent-skills | a row whose literal differs by one character fails it |
| content/ in the walk passes the check | `cargo klarch check` | the probe's 36 findings are the baseline: each repair removes one |

## Audit subjects

- The milestone document entire, and this spec.
- `path@agent-skills@build.rs` as the snippet branch left it, and `path@agent-skills@src/lib.rs`.
- `path@knowledge-architect@knowledge-architect.toml`, its `exclude` list.
- `design@agent-skills@shipped-text-is-reference-free`, `design@agent-skills@content-mirrors-the-install-layout`
  and `design@core@reserved-anchors`.
- `issue@agent-skills@shipped-text-is-reference-free-mechanically`.
- `path@agent-skills@CLAUDE.md`, its paragraph "No check reads content/".
- `path@agent-config@skills/klarch-release/SKILL.md`, its step that checks the shipped text by hand
  and says the primer's import line "is no candidate", which the walk probe contradicts: the
  checker reported that line as malformed.
- The setup skill's section "In a Rust project", `path@agent-skills@content/skills/setup/SKILL.md`.

## Fails alone on

- `cargo klarch check` reports a finding in content/ after the repairs.
- An installed file differs from the one committed before the first `%%` line was written.

## Premises that expire

- **The snippet branch has merged.** This slice adds the comment strip and the substitutions
  beside its placeholder mechanism in `path@agent-skills@build.rs`, and orders the three. If the snippet branch has not merged when this slice starts, the
  slice stops and the owner rules, since the owner fixed the order in R7.
- **The checker accepts `path@*@<path>` for every shape of a required document**, per
  `design@core@reserved-anchors`. The repairs of the first two classes rely on it. A change to that
  head is reported by `cargo klarch check` on the repaired sites.

## Decided design

### #shipped-text-line-comments: `%%` lines, and content/ in the walk

A `%%` line is a comment for this repository's maintainers. It sits beside the instruction it
explains, and it may cite design heads and issues. The checker reads it like any other prose of
content/, so its references are checked. The build removes it whole, so no installed file holds it.
An illustration:

```text
content/skills/design/SKILL.md                  installed SKILL.md
%% design@agent-skills@structure-the-flow
**never pad with alternatives**           →     **never pad with alternatives**
to reach a count.                               to reach a count.
```

- **The marker** (a54): `%%` has no Markdown meaning, reads as a comment in Mermaid and Obsidian, and
  cannot collide with a Rust snippet, whose comments are `//`. In content/, no line began with
  `%%` when the marker was chosen, R4; `grep -rn '%%' crates/agent-skills/content` re-takes it.
- **Line comments, not HTML comments** (a49, a53): the checker treats an HTML comment as parked
  text, and reads no reference in it, so citations would go unchecked.
- **Order in the build**: comments are stripped first, then delivery substitutions, then snippets,
  so a snippet's own text is never stripped or substituted.
- **content/ back in the walk** (a59, a60, a64), rather than unchecked comments (a55, a63) or a
  test in xtask (a64), and rather than a declaration in the published checker, which would cater
  to this repository's use (a47, a56). The cost is the 36 repairs.
- **Placeholders in the Rust section** (a68): `<xtask-dir>/` for the maintenance crate's
  directory, which varies per project, and `<workspace-root>/.cargo/config.toml` for cargo's
  configuration.

### #shipped-text-entry-references-only: the shipped text cites no entry

`design@agent-skills@shipped-text-is-reference-free` is narrowed: the shipped text holds no
reference to an entry, since no tree but this one holds this repository's entries; a path every
conforming project holds may be written as a reference (a61, a62). `path@*@docs/goals.md` names
each Component's own copy, required by `design@core@components-carry-the-same-documents`, so it is
true in every project that installs the text, and it shows the syntax the checker enforces there
(a67). The checker accepts the generic anchor for every shape of a required document, so the
directory shape is written as a reference too: `path@*@docs/goals/`. The head is rewritten in place
under a slug that names the narrowed rule. This rests on the first default awaiting the owner.

The plain-text paths of the shipped text are no longer hidden from the walk, so the walk also
checks that each generic path names a document the workflow requires, or a path that at least one
Component of this repository carries, which is what `design@core@reserved-anchors` accepts.

### #delivery-substitutions: a table in the build script

A row exists only for text that must ship verbatim and that the checker would misread (a72). A
sentence that can be rewritten without harm is rewritten instead. Each row carries the reason in a
comment, so a reviewer sees the judgement (a66). The table is filled at build time, since its
literals do not vary by project (a71); the install keeps filling only the project's command. A row
no text uses fails the build, as an unused snippet does. Nearest rival: a plain rewrite, which for
the import line describes the line instead of showing the exact text an agent must copy (a73).

No design head records it: its reason stands at one site, the table, which the decided T2 sends to
a comment there.

## Mapping tables

The 36 findings of the walk probe, each class and the form that repairs it. The sites are
re-listed by running the probe again at the slice's start: `cargo klarch check` with the line
`"crates/agent-skills/content",` deleted from the `exclude` list.

| class | count | example as written today | repair |
| --- | --- | --- | --- |
| a required document of every Component, file shape, or the issue directory | 19 | the goals, design, tripwires and rejected-alternatives homes (4 each), and the issue directory (3), as plain backticked paths under docs/ | `path@*@docs/goals.md`, `path@*@docs/open-issues/`, and so on |
| a required document of every Component, directory shape | 11 | the goals (4), design (4) and tripwires (3) homes' directory shape, as plain backticked paths | `path@*@docs/goals/`, and so on |
| a path of the plans directory | 3 | the plans directory's README, specs/ and milestones/, as plain backticked paths | `path@plans@README.md`, `path@plans@specs/`, `path@plans@milestones/` |
| a path of the setup skill's Rust section | 2 | the maintenance crate's directory and cargo's configuration file, as plain backticked paths | `<xtask-dir>/`, `<workspace-root>/.cargo/config.toml` |
| the primer's import line | 1 | the backticked import line at the setup skill's line 152 | `{{primer-import}}`, a delivery substitution |

## Losing alternatives

None ruled out as a thread. The rivals each decided shape beat are under Decided design.

## Acceptance criteria

### No `%%` line reaches an installed file `##no-comment-line-ships`

- **Guards:** #shipped-text-line-comments, from P4 of the premortem.
- **Judged by:** this slice, with a test in `path@agent-skills@src/lib.rs` over `FILES`.
- **Fires when:** a line of an installed text, its leading spaces trimmed, starts with `%%`.
- **Response:** the build script's strip is repaired; if the case is one the decided rule does not
  cover, #shipped-text-line-comments reopens with the owner.

## Harvest

At this slice's landing, under `knowledge-architect-decision-recording` and
`knowledge-architect-issue-tracking`:

| what | home |
| --- | --- |
| #shipped-text-line-comments | a new head in `path@agent-skills@docs/design.md`, slug `shipped-text-line-comments`: its reason, the marker and its checking, binds every file of content/ |
| #shipped-text-entry-references-only | `design@agent-skills@shipped-text-is-reference-free` rewritten in place, slug `shipped-text-cites-no-entry`; its references in `path@agent-config@skills/klarch-release/SKILL.md`, `path@agent-skills@CLAUDE.md` and the issue below follow |
| #delivery-substitutions | none: the comment of the table in `path@agent-skills@build.rs`, and the commit |
| the walk | `issue@agent-skills@shipped-text-is-reference-free-mechanically` rewritten: the exclusion is gone; what remains is a check that the shipped text holds no reference to an entry, applied to the installed copies under .claude once a path can be declared reference-clean, per the owner in R5 |
| the developer's contract | `path@agent-skills@CLAUDE.md`: "No check reads content/" becomes the `%%` convention and the delivery substitutions; the paragraph the snippet branch adds, which says the snippets stay "reference-free as content/ does", follows the narrowed rule |
| the release procedure | `path@agent-config@skills/klarch-release/SKILL.md`, its step on the shipped text |

This slice's spec and the milestone document leave in the commit that completes this harvest.
