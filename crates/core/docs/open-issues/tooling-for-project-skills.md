---
kind: todo
---
# The checker does not read the structure of a project's own skills

## Summary

A project's agent skills are prose files in its agent configuration. The checker reads their
references like any document, and nothing about their structure: their names, their description
fields, which other skill they extend.

## Details

### What

Checks over the structure of a project's own skills. Long term; the shape is open.

### Why it matters

The installed workflow asks a project to name its skills with its own prefix, per
`design@agent-skills@skill-name-prefix`, and to route each installed skill to the project skills
that add to it, per `design@agent-skills@routing-table-shape`. A skill that breaks either is found
by review or not at all.

### What would close it

Checks over the structure of a project's own skills, shipped in the checker.
