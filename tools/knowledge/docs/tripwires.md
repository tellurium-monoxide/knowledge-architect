
## Guarding `knowledge#families-are-the-checks`

**Fires when:** a check family exists that no subagent definition under `../../.claude/agents/`
names, or a definition names a family the tool does not accept. Read the family list out of the
tool's help, which prints every accepted name, and grep each across that directory.
**Response:** open a `defect`. A family nobody reads is a check whose output reaches no reviewer,
and a definition naming a family that does not exist sends a reviewer to a command that fails. The
mapping from a subject to its families lives in the definitions and nowhere else, which is what
keeps the tool free of this repository's review vocabulary, so nothing links the two sides.
**Re-entry:** standing, and each time a check module is added.

