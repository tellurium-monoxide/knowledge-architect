# Retrospective reports

This directory holds the analyses of the retrospective files this repository receives: one file
per received file, named by its stem. Each analysis checks every finding of that file
against the tree, the registers, the goals and the design, proposes an action, and records the
owner's ruling on it. The decision is `design@knowledge-architect@committed-findings-analysis`,
and the procedure is the `klarch-retrospective-intake` skill.

An analysis lands once the owner has ruled on every finding. The issues it rules are opened next,
then the findings ruled to be handled now are handled. The commit that carries out its last outcome
deletes it. So this directory holds only the analyses whose outcomes are not all carried out, and
`git log -- docs/retrospective-reports/` lists every analysis there has been.
