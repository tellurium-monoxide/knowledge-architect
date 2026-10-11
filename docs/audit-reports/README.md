# Audit reports

This directory holds the report files of this repository's project audits: one file per pass of an
audit, named `<YYYY-MM-DD>-<axis>-pass-<n>.md`, the axis named by its section's slug. Each file
holds the pass's evidence and its owner list, and the owner's ruling on each item in its `Outcome`
line. The procedure is `skill@knowledge-architect-project-audit@record-of-an-audit`.

A file is committed once the owner has ruled on every item, before the pass's edits are applied.
The commit that carries out its last outcome, or a later commit of the same branch, deletes it. So
on the main branch this directory holds only the files whose outcomes are not all carried out, and
`git log -- docs/audit-reports/` lists every file there has been.
