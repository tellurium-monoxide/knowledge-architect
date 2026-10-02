---
name: knowledge-architect-retrospective
description: MUST use once per session, at the first of these moments, to offer a retrospective to the owner - a branch the session worked on has merged into the main branch, a plan document has left, or the owner says the session is ending - and run it only if the owner accepts. Covers what the retrospective examines (the installed skills and agents, the project's own instructions, how the two interact, and the checker), the three questions it always asks, the two files it writes outside the project, and how each file reaches the project that must change.
---

# Retrospective

Scope: looking back at one session's use of the workflow, and turning what was unclear, missing or
wrong into findings the owner can act on. Real sessions are the test of the workflow, and this is
how they report.

Not covered here: **opening an issue entry in the project**, `knowledge-architect-tracking-open-issues`.

## 1. When it is offered

**Once per session, at the first of these moments**, and never during a task:

- a branch the session worked on has merged into the main branch;
- a plan document has left the repository;
- the owner says the session is ending.

Offer it in one line, and run it only if the owner accepts. A declined retrospective is not offered
again in that session.

## 2. What it examines

Open with **what the session did**: one paragraph, at the level of the workflow ("a design
discussion and two merged branches"), not of the project's subject matter.

Then read back over the session: the instructions that were followed, where they were followed with
difficulty, where they were not followed, and what the owner corrected. Then, for each of the four
subjects below, list what was **unclear** (it could be read two ways), **missing** (the session had
to decide something no instruction covered), or **wrong** (following it produced a defect or a
correction). Each item quotes the instruction, says what happened when it was followed, and what was
done instead.

1. **The installed skills and agents.**
2. **The project's own instructions**: its root `CLAUDE.md`, its scoped `CLAUDE.md` files, its own
   skills and agents.
3. **How the two interact**: an installed instruction and a project instruction that contradicted
   each other, overlapped, or each assumed the other would cover something.
4. **The checker**: a defect; a blind spot, something it should have reported and did not; a false
   finding; and what would make it easier to use, such as a missing command or option, or a message
   that did not say what to do.

**Always ask these three questions**, and answer each with what the session shows:

- Did this session need to change an installed skill, and what for?
- Was the primer present in this session, and in its subagents?
- Did this session miss something a project skill adds to an installed skill?

End with **proposals**: for each finding that has one, what to change, and where.

## 3. Two files, sorted by whose text must change

Each finding goes to the file of the project whose text or code must change:

| file | holds |
| --- | --- |
| `<YYYY-MM-DD>-<project>.md` | what the session did, the findings on the project's own instructions, and the project's side of an interaction |
| `<YYYY-MM-DD>-<project>-workflow.md` | what the session did, the findings on the installed skills and agents and on the checker, the workflow's side of an interaction, and the three questions |

An interaction finding whose fix may fall on either side goes in both files, each written from its
own side.

**The second file may become public.** It holds nothing of the project beyond what a finding needs
to be understood: no code, no names of the project's internals, no content of its documents beyond
the instruction quoted.

**Where the files go**: a directory outside the project, chosen by the owner. If the owner's
user-level agent configuration names one, use it. Otherwise ask, propose a directory under the
owner's home, and offer to record the answer in that user-level configuration, on the owner's word,
so later sessions find it. Never inside the project: committed, the files would enter its history;
ignored, they would be lost to the next clean.

## 4. What becomes of each file

Show the owner both files verbatim. The owner may edit them. Then, **on the owner's word only, and
where the owner directs**:

- **The project's file**: each finding the owner keeps becomes an issue entry in the project's own
  register, under `knowledge-architect-tracking-open-issues`.
- **The workflow's file**: it becomes an issue on the repository of knowledge-architect,
  <https://github.com/tellurium-monoxide/knowledge-architect>, opened with
  `gh issue create --repo tellurium-monoxide/knowledge-architect --title "<title>" --body-file <file>`.
  If `gh` is absent or fails, give the owner the text and the address of the repository's new-issue
  page.

Nothing leaves the machine without the owner having read it.
