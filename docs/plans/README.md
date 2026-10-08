# Plan documents

This is the plans directory: specs and milestones for work that is designed and not yet built.
The checker constructs the anchor `plans` here, and it holds this file and two homes, nothing
else:

- [specs/](specs/README.md): one file per spec, the plan of work done in one pull request,
  cited `spec@plans@<id>`;
- [milestones/](milestones/README.md): one directory per milestone, work across several pull
  requests, cited `milestone@plans@<id>`. Its `README.md` is the milestone document, its
  `index.md` is the generated listing of its slices, and each other file in it is the spec of one
  slice, cited `spec@<milestone>@<slice>`.

A plan document may land before its work starts, and leaves in the commit that completes its last
harvest. The procedure is the
installed `skill@knowledge-architect-planning`.
