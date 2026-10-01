//! Phase 2: the installed agent configuration against what this version ships.
//!
//! Under the `claude` harness, each shipped file must be present at its install path with the
//! bytes of its template rendered for the project; no file of the installer's namespace may be
//! one this version does not ship; and the root CLAUDE.md must import the primer when the primer
//! is shipped. The decisions are `design@core@owned-namespace-check` and
//! `design@core@agents-table`. The repair of a missing, differing or unshipped file is the
//! install; a deletion awaiting staging and a missing import line each name their own.
//!
//! **Phase 2, not the last phase.** A missing or stale installed file is a fact about what the
//! tree holds, like a missing required document, so a run stops before references while one
//! stands.

use std::path::{Path, PathBuf};

use crate::agents::{imports_primer, IMPORT_LINE, PRIMER};
use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::survey::Outside;

use super::Inputs;

pub(crate) fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Vec<Finding> {
    let mut out = Vec::new();
    if !manifest.serves_claude() {
        return out;
    }
    let install = format!("run `{} install-agent-skills`", manifest.command());
    let version = env!("CARGO_PKG_VERSION");
    let installed = |rel: &Path| inputs.installed.iter().find(|(p, _)| p == rel);
    for (rel, expected) in inputs.shipped {
        match installed(rel).map(|(_, state)| state) {
            None | Some(Outside::Missing) => out.push(Finding::in_file(
                rel,
                format!(
                    "version {version} ships this file, and git lists no installed copy: it is \
                     missing, or an ignore rule covers it"
                ),
                format!("{install}, and commit the file; an installed file is never ignored"),
            )),
            Some(Outside::Text(text)) if crate::agents::lf(text) == *expected => {}
            Some(_) => out.push(Finding::in_file(
                rel,
                format!(
                    "this installed file differs from what version {version} ships for this \
                     project"
                ),
                format!(
                    "{install}; a change a project needs belongs in a skill of its own, never \
                     in an installed one"
                ),
            )),
        }
    }
    for (rel, state) in inputs.installed {
        if inputs.shipped.iter().any(|(s, _)| s == rel) {
            continue;
        }
        if *state == Outside::Missing {
            // The install removed it and git still lists it: the deletion awaits staging.
            out.push(Finding::in_file(
                rel,
                "this file of the installer's namespace is deleted, and the deletion is not \
                 staged",
                "stage the deletion",
            ));
            continue;
        }
        out.push(Finding::in_file(
            rel,
            format!(
                "this file sits in the installer's namespace, and version {version} does \
                 not ship it"
            ),
            format!("{install}, which removes it; a project's own file takes a name of its own"),
        ));
    }
    if inputs.shipped.iter().any(|(s, _)| s == Path::new(PRIMER)) {
        let root = PathBuf::from(crate::manifest::AGENT_DOCUMENT);
        match model.documents().iter().find(|d| d.rel == root) {
            Some(doc) if !imports_primer(&doc.text) => out.push(Finding::in_file(
                &root,
                "the root CLAUDE.md does not import the installed primer",
                format!("add a line holding exactly `{IMPORT_LINE}`"),
            )),
            Some(_) => {}
            // A root CLAUDE.md a walk row keeps out cannot be read for the import, and no row may
            // waive the import: only declaring no harness does.
            None if inputs.present.contains(&root) => out.push(Finding::in_file(
                &root,
                "the root CLAUDE.md is outside the walk, so its import of the installed primer \
                 cannot be checked",
                "remove the row of [walk] that keeps it out",
            )),
            // A missing root CLAUDE.md is the required-documents finding of `check::tree`.
            None => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nname = \"p\"\ncomponents = []\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    fn run(
        manifest: &Manifest,
        claude_md: &str,
        installed: &[(&str, Outside)],
        shipped: &[(&str, &str)],
    ) -> Vec<Finding> {
        let model =
            Model::from_documents(vec![(PathBuf::from("CLAUDE.md"), claude_md.to_string())]);
        let installed: Vec<(PathBuf, Outside)> = installed
            .iter()
            .map(|(p, s)| (PathBuf::from(p), s.clone()))
            .collect();
        let shipped: Vec<(PathBuf, String)> = shipped
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let empty_set = HashSet::new();
        let empty_map = HashMap::new();
        let inputs = Inputs {
            committed: &empty_map,
            configs: &empty_map,
            present: &empty_set,
            directories: &empty_set,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &installed,
            shipped: &shipped,
        };
        check(&model, manifest, &inputs)
    }

    const SKILL: &str = ".claude/skills/knowledge-architect-a/SKILL.md";
    const IMPORTING: &str = "# P\n\n@.claude/knowledge-architect/PRIMER.md\n";

    /// The claim: an installed set equal to the shipped one, with the primer imported, passes.
    /// Mutation: comparing against the template instead of the rendered text, or requiring the
    /// import line when nothing is shipped, each fails one of the cases below.
    #[test]
    fn a_matching_installed_set_passes() {
        let m = manifest("");
        let shipped = [(SKILL, "run `klarch check`"), (PRIMER, "primer")];
        let installed = [
            (SKILL, Outside::Text("run `klarch check`".into())),
            (PRIMER, Outside::Text("primer".into())),
        ];
        assert!(run(&m, IMPORTING, &installed, &shipped).is_empty());
        assert!(
            run(&m, "# P\n", &[], &[]).is_empty(),
            "nothing shipped, nothing owed"
        );
    }

    /// The claim: each defect of the installed set is one finding on its own path: a missing
    /// shipped file, a differing one, an unshipped file in the namespace, and a root CLAUDE.md
    /// that does not import the shipped primer. Mutation: deleting any of the four branches
    /// loses its finding.
    #[test]
    fn each_defect_of_the_installed_set_is_one_finding() {
        let m = manifest("");
        let shipped = [(SKILL, "fresh"), (PRIMER, "primer")];
        let installed = [
            (SKILL, Outside::Text("edited".into())),
            (
                ".claude/agents/knowledge-architect-old.md",
                Outside::Text("old".into()),
            ),
        ];
        let found = run(&m, "# P\n", &installed, &shipped);
        let at: Vec<String> = found.iter().map(|f| f.file.display().to_string()).collect();
        assert_eq!(
            at,
            [
                SKILL,
                PRIMER,
                ".claude/agents/knowledge-architect-old.md",
                "CLAUDE.md"
            ],
            "{found:?}"
        );
        assert!(found[0].what.contains("differs"), "{found:?}");
        assert!(found[1].what.contains("no installed copy"), "{found:?}");
        assert!(found[2].what.contains("does not ship"), "{found:?}");
        assert!(
            found
                .iter()
                .all(|f| f.action.contains("install-agent-skills")
                    || f.file == Path::new("CLAUDE.md"))
        );
    }

    /// The claim: under `harness = []` nothing about the agent configuration is checked.
    #[test]
    fn no_harness_checks_nothing() {
        let m = manifest("\n[agents]\nharness = []\n");
        let installed = [(SKILL, Outside::Text("anything".into()))];
        assert!(run(&m, "# P\n", &installed, &[(PRIMER, "primer")]).is_empty());
    }

    /// The claim: the repair names the project's declared command.
    #[test]
    fn the_repair_names_the_declared_command() {
        let m = manifest("command = \"cargo klarch\"\n");
        let found = run(&m, "# P\n", &[], &[(SKILL, "x")]);
        assert!(
            found[0]
                .action
                .contains("`cargo klarch install-agent-skills`"),
            "{found:?}"
        );
    }

    /// The claim: a shipped file git lists and the working tree does not hold is reported as not
    /// installed, a binary or unreadable file of the namespace is reported as unshipped, and CRLF
    /// line endings in an installed copy do not make it differ. Mutations: passing a `Missing`
    /// state silently, skipping a non-text stray, or comparing without normalising line endings
    /// each fail one case.
    #[test]
    fn missing_binary_and_crlf_states_are_judged() {
        let m = manifest("");
        let found = run(&m, "# P\n", &[(SKILL, Outside::Missing)], &[(SKILL, "x\n")]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].what.contains("no installed copy"), "{found:?}");
        let stray = ".claude/agents/knowledge-architect-blob.md";
        let found = run(&m, "# P\n", &[(stray, Outside::Binary)], &[]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].what.contains("does not ship"), "{found:?}");
        let crlf = [(SKILL, Outside::Text("a\r\nb\r\n".into()))];
        assert!(run(&m, "# P\n", &crlf, &[(SKILL, "a\nb\n")]).is_empty());
    }

    /// The claim: a root CLAUDE.md the walk keeps out still owes the import, and says so; a
    /// missing one is left to the required-documents check. Mutation: skipping the case of a
    /// present but unwalked root CLAUDE.md lets a `skip-files` row waive the import.
    #[test]
    fn an_unwalked_root_claude_md_cannot_waive_the_import() {
        let m = manifest("");
        let model = Model::from_documents(Vec::new());
        let present: HashSet<PathBuf> = [PathBuf::from("CLAUDE.md")].into_iter().collect();
        let empty_map = HashMap::new();
        let shipped = vec![(PathBuf::from(PRIMER), "primer".to_string())];
        let installed = vec![(PathBuf::from(PRIMER), Outside::Text("primer".into()))];
        let inputs = Inputs {
            committed: &empty_map,
            configs: &empty_map,
            present: &present,
            directories: &HashSet::new(),
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &installed,
            shipped: &shipped,
        };
        let found = check(&model, &m, &inputs);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].what.contains("outside the walk"), "{found:?}");
        let absent = Inputs {
            present: &HashSet::new(),
            ..inputs
        };
        assert!(check(&model, &m, &absent).is_empty());
    }
}
