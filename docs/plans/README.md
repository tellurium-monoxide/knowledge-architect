# Plan documents

This is the plans directory: specs and milestones for work that is designed and not yet built.
The checker constructs the anchor `plans` here, and it holds this file and two homes, nothing
else:

- [specs/](specs/README.md): one file per spec, the plan of work done in one pull request,
  cited `spec@plans@<id>`;
- [milestones/](milestones/README.md): one directory per milestone, work across several pull
  requests, cited `milestone@plans@<id>`. Its `README.md` is the milestone document, and each
  other file in it is the spec of one step, cited `spec@<milestone>@<step>`.

A plan document leaves in the commit that completes its last harvest, so between pieces of planned
work the two homes hold only their README and their generated index. The procedure is the
installed `knowledge-architect-planning`.
