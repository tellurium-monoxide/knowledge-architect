# Open issues — the notes

## The notes are not a component `observation`

**What.** This directory carries outstanding state and nothing else a component carries, so it
is declared one tracker at a time rather than as a component.

**Why it matters.** Without the declaration nothing reads this file, and what is open here would
not appear in the report at all.

**What would answer it.** Nothing. It exists so that a test has an entry to find.
