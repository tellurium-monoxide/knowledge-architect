---
kind: defect
---
# A heading-shaped line that markdown reads as no heading still defines an entry

## Summary

The scanner reads a line opening with `#` marks as a heading wherever markdown does not mark it
as code. Inside an HTML block, a `+++` metadata block, or a `---` block that holds a blank line,
markdown opens no section, and the scanner still reads a heading there, with any slug on it
defining an entry.

## Details

### What

`scan` in `path@core@src/scan.rs` matches `HEADING` and `SLUG_SITE` on every line that is not
fenced and not inside an HTML comment. The markdown parse in `path@core@src/source/md.rs` marks
code blocks and HTML comments, and blanks a frontmatter block it accepts, but nothing marks an
HTML block, a `+++` block, or a `---` block the frontmatter reader refuses because it holds a
blank line.

Reproduced with a unit probe over `md::parse` and `scan`, for each input listing the sections the
parser opened and the scanner's heading and slug observations:

| input | sections | scanner |
| --- | --- | --- |
| a `<div>` line, then `## Raw` with a slug `html-entry`, then `</div>` | none | a level-2 heading and a `Heading(2)` slug site at line 2 |
| a `+++` line, then `## In toml` with a slug `toml-entry`, then `+++` | none | the same at line 2 |
| `---`, `kind: todo`, a blank line, `## In yaml` with a slug `yaml-entry`, `---` | none | the same at line 4 |

The opposite direction, a heading markdown reads and the scanner does not, is reported in phase 2
per `design@core@headings-open-with-hash-marks`.

### Why it matters

`design@core@an-entry-is-a-heading-at-the-register-level` rests on the scanner reading headings as
markdown does. An entry defined on a line that a renderer shows as raw HTML or as metadata is one
a reader of the rendered document cannot find, against
`goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

Every heading the scanner reads in a markdown document opens a section of the parse, or is
reported in phase 2: for instance by reporting a `HEADING` line at which no section starts, with a
test for each of the three inputs above. `design@core@headings-open-with-hash-marks` then states
both directions.
