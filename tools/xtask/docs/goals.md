# Goals — xtask

What this tool is for. A goal is met or unmet, where a decision about how the tool is built is
won or lost and lives in `path@xtask@docs/design.md`. A goal stays here while it is met, and leaves
only when the owner abandons it. This tool serves the project rather than its consumers, so its
goals refine no goal of `path@knowledge-architect@docs/goals.md`.

## Every check the project owes before a merge runs from one command `##one-command-runs-every-gate`

One command runs every check a branch must pass before it merges: formatting, the document check,
the commit messages, the linter and the tests. It runs them all when one fails, and gives one
verdict as its exit code. It is met while a session needs no other command to know whether a branch
may merge, and the continuous integration runs the same command.

## A task performed repeatedly in the project is a command of the tool `##repeated-tasks-are-automated`

A task that sessions perform in this repository more than once, by hand and in the same steps,
becomes a command of the tool, so it runs the same way each time. It is met while no such task is
left to be done by hand. None exists today.
