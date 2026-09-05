//! Every reference resolves against the entity table, and every relative markdown link is a
//! navigation row that resolves beside its file.
//!
//! One grammar, `` `<kind>@<anchor>@<id>` ``, and one resolver. A reference to a table kind —
//! `design`, `goal`, `tripwire` — is looked up in the table `entity::Entities` builds from the
//! walk; a `path` reference is resolved against the survey under the same anchors. A reference
//! that resolves to nothing is reported as the repair it needs, four ways: the kind position
//! holds an anchor, the anchor is unknown, the anchor does not carry that register, or the id
//! is not defined there. The argument is `knowledge#a-slug-belongs-to-a-component`.
//!
//! **Nothing pointer-shaped passes unregistered.** A span with no `@` that is shaped like a
//! path is reported as unanchored, and the two forms the grammar retired — `` `<word>#<word>` ``
//! and a bare `R` with digits — are reported as what they were, so a pointer the migration
//! missed is a finding rather than silence. The candidate rule and the retired-form lint are
//! `knowledge#candidate-rule-and-retired-forms`.
//!
//! **The definition-site findings sit here for now.** Misplaced and duplicate definitions are
//! found while the table is built; they belong to the `registers` family, which lands with the
//! register-shape checks, and until then this family reports them.

use std::path::{Path, PathBuf};

use crate::entity::{self, Anchors, Candidate, Entities, Kind, Resolution, ESCAPE_ANCHOR};
use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::scan::{Observation, RetiredForm};

use super::Inputs;

/// What the check looked at: the entities the table holds, the reference occurrences it
/// judged (every kind, the escape and generic forms included), and the relative markdown links
/// it resolved.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub entities: usize,
    pub references: usize,
    pub links: usize,
}

pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
    let anchors = Anchors::of(manifest);
    let entities = Entities::build(model, &anchors);
    let mut out: Vec<Finding> = entities.definition_findings().to_vec();
    let mut counts = Counts {
        entities: entities.len(),
        ..Counts::default()
    };
    for doc in model.documents() {
        let nav = is_navigation(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::Span(span) => match entity::candidate(span, &anchors) {
                    Candidate::Reference { kind, anchor, id } => {
                        counts.references += 1;
                        if kind == Kind::Path {
                            path(
                                &mut out, &doc.rel, l.line, span, anchor, id, &anchors, inputs,
                                manifest,
                            );
                        } else {
                            table(
                                &mut out, &doc.rel, l.line, span, kind, anchor, id, &anchors,
                                &entities,
                            );
                        }
                    }
                    Candidate::Malformed { why } => out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{span}` is malformed: {why}"),
                        "write `<kind>@<anchor>@<id>`, and for a path `path@<anchor>@<path>`",
                    )),
                    Candidate::AnchorInKindPosition { head } => out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{span}` opens with `{head}`, an anchor, where the kind goes"),
                        format!(
                            "prefix the kind, one of {}: a path is written `path@{head}@<path>`",
                            Kind::listed()
                        ),
                    )),
                    Candidate::NotOne => {}
                },
                Observation::UnanchoredPath(span) => out.push(Finding::at(
                    &doc.rel,
                    l.line,
                    format!("`{span}` is shaped like a path and names no anchor"),
                    "write `path@<anchor>@<path>`, `path@elsewhere@<path>` for a path outside \
                     this tree, or `path@*@<path>` for every component's own copy; or rephrase \
                     so the span is not path-shaped",
                )),
                Observation::Retired(RetiredForm::SlugRef(span)) => out.push(Finding::at(
                    &doc.rel,
                    l.line,
                    format!("`{span}` is the retired slug reference form"),
                    "write `design@<component>@<slug>`; the form with a `#` is no longer read \
                     as a reference",
                )),
                Observation::Retired(RetiredForm::RegisterNumber(n)) => out.push(Finding::at(
                    &doc.rel,
                    l.line,
                    format!("`R{n}` is the retired interpretation entry number form"),
                    "name the entry in the `<kind>@<anchor>@<id>` grammar; a bare `R` and \
                     digits is no longer read as a reference",
                )),
                // Markdown documents only: in Rust prose a markdown link is rustdoc's
                // mechanism, resolved by rustdoc against the crate namespace, and this
                // check reading those as index rows would report every intra-doc link.
                Observation::Link(target) if doc.is_markdown() => {
                    link(
                        &mut out,
                        &mut counts,
                        doc,
                        l.line,
                        target,
                        nav,
                        inputs,
                        manifest,
                    );
                }
                _ => {}
            }
        }
    }
    (out, counts)
}

/// Whether relative markdown links are legal in this file.
///
/// A README is directions about what a directory holds and an index is a generated or
/// hand-written listing, so both are navigation homes; everywhere else a pointer in prose is
/// a backticked reference, and a link is reported.
pub(crate) fn is_navigation(rel: &Path) -> bool {
    rel.file_name()
        .is_some_and(|n| n == "README.md" || n == "index.md")
}

/// Whether a link target opens with a URI scheme — letters then a colon, before any slash.
///
/// A filename may hold a colon in principle; a relative path in this project never does, and
/// reading such a target as a scheme passes it over rather than reporting a false dangling.
pub(crate) fn has_scheme(target: &str) -> bool {
    target
        .split('/')
        .next()
        .is_some_and(|head| head.contains(':'))
}

/// A reference to a table kind, resolved through the entity table.
#[allow(clippy::too_many_arguments)]
fn table(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    span: &str,
    kind: Kind,
    anchor: &str,
    id: &str,
    anchors: &Anchors,
    entities: &Entities,
) {
    match entities.resolve(anchors, kind, anchor, id) {
        Resolution::Resolved => {}
        Resolution::UnknownAnchor => out.push(Finding::at(
            rel,
            line,
            format!("`{span}` names `{anchor}`, which is no anchor of this project"),
            format!(
                "anchor at one of {}; `*` and `{ESCAPE_ANCHOR}` serve the path kind alone",
                anchors.listed()
            ),
        )),
        Resolution::AnchorLacksRegister { carriers } => out.push(Finding::at(
            rel,
            line,
            format!("`{span}` names `{anchor}`, which carries no {kind} register"),
            format!("the anchors that carry one: {}", carriers.join(", ")),
        )),
        Resolution::Undefined => out.push(Finding::at(
            rel,
            line,
            format!("`{span}` is referenced and `{anchor}` defines no {kind} `{id}`"),
            format!(
                "define it in the {} home of `{anchor}`, or repair the reference",
                kind.register_dir().unwrap_or("register")
            ),
        )),
    }
}

/// Why a path's shape is refused before any resolution, or `None` for a plain relative path.
///
/// An upward or explicit-current segment breaks when the referencing file moves, which is
/// the failure the anchored grammar exists to remove, and a leading slash is outside the
/// project's namespace.
fn refused(path: &str) -> Option<&'static str> {
    if path.starts_with('/') {
        return Some("a leading slash is outside the project's namespace");
    }
    if path.split('/').any(|s| s == "..") {
        return Some("an upward segment is anchored at the wrong place by definition");
    }
    if path.split('/').any(|s| s == ".") {
        return Some("an explicit-current segment adds nothing and survives no move");
    }
    None
}

/// A `path` reference, resolved against the survey under its anchor.
#[allow(clippy::too_many_arguments)]
fn path(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    span: &str,
    anchor: &str,
    path: &str,
    anchors: &Anchors,
    inputs: &Inputs,
    manifest: &Manifest,
) {
    // The trailing slash is the writer's claim about the target's kind; the lookup drops it.
    let claims_dir = path.ends_with('/');
    let trimmed = path.trim_end_matches('/');
    match anchor {
        ESCAPE_ANCHOR => {
            // The escape is exempt from existence and kind assertions — there is nothing
            // stable to assert — and from the shape refusals, because a foreign layout may
            // spell anything. The one assertion is that it stays honest: a target that
            // resolves here is a real reference wearing the escape, which would otherwise
            // be the cheap way to silence the unanchored finding.
            let resolving = anchors
                .all()
                .iter()
                .find(|a| inputs.present.contains(&a.path.join(trimmed)));
            if let Some(a) = resolving {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` resolves in this tree, beside `{}`", a.name),
                    "the escape anchor is for a path this tree does not hold; anchor the \
                     reference at the anchor that holds it",
                ));
            }
        }
        entity::EVERY_ANCHOR => {
            if let Some(why) = refused(trimmed) {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` is refused: {why}"),
                    "the generic form takes a plain component-relative path",
                ));
                return;
            }
            // The compiled-in required set is what the generic form usually names, and it
            // is accepted whether or not any component carries the shape yet — the two
            // shapes of each heading register are the case, where naming both is legitimate
            // while only one is in use anywhere. The set carries its own kinds, so the
            // trailing-slash claim is asserted here too.
            if let Some(required_dir) = Anchors::required_kind(trimmed) {
                if claims_dir != required_dir {
                    out.push(Finding::at(
                        rel,
                        line,
                        format!("`{span}` claims the wrong kind for a required document"),
                        "a register's directory home takes the trailing slash and the \
                         document files take none; the slash is the kind claim",
                    ));
                }
                return;
            }
            // Anything else must be real somewhere, with the claimed kind. A hit is an
            // anchor's OWN copy: a path reaching a nested anchor from above is not this
            // anchor's, or the generic form would evade the deepest-anchor rule. A
            // gitignored copy is not a hit either — presence of generated content is build
            // state, and a verdict may not depend on the checking machine's.
            let hit = anchors.all().iter().any(|a| {
                let t = a.path.join(trimmed);
                anchors.owning(&t).path == a.path
                    && !manifest.ignore().covers(&t, claims_dir)
                    && inputs.present.contains(&t)
                    && claims_dir == inputs.directories.contains(&t)
            });
            if !hit {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` resolves in no component"),
                    "the generic form names a required document, or a path at least one \
                     component carries; repair the path, or anchor at one component",
                ));
            }
        }
        name => {
            let Some(a) = anchors.by_name(name) else {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` names `{name}`, which is no anchor of this project"),
                    format!(
                        "anchor at one of {}, at `{ESCAPE_ANCHOR}` for a path outside this \
                         tree, or at `*` for every component's own copy",
                        anchors.listed()
                    ),
                ));
                return;
            };
            if let Some(why) = refused(trimmed) {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` is refused: {why}"),
                    "anchor at the anchor that holds the target, with a plain relative path",
                ));
                return;
            }
            let target = a.path.join(trimmed);
            // The deepest anchor wins: a reference reaching inside another anchor breaks
            // when that anchor moves, and the anchor is what a move must not break. Inside
            // means a PROPER descendant: an anchor's own directory has no spelling under
            // its own name, so pointing at it from an ancestor is the one legal way to name
            // where it lives — and that reference names a location, which is exactly what
            // a move is expected to break.
            let owner = anchors.owning(&target);
            if owner.path != a.path && target != owner.path {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` reaches inside the anchor `{}`", owner.name),
                    "anchor at the deepest anchor holding the target, so a move edits \
                     knowledge.toml and no document",
                ));
                return;
            }
            assert_target(out, rel, line, span, &target, claims_dir, inputs, manifest);
        }
    }
}

/// Assert one resolved target: it exists and has the claimed kind, unless the root
/// gitignore covers it.
///
/// The gitignore exemption runs on the ignore RULES rather than on what happens to exist,
/// so a reference to a generated path passes on a fresh clone exactly as it passes on a
/// built tree — a verdict that depends on build state is a check nobody can trust twice.
#[allow(clippy::too_many_arguments)]
fn assert_target(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    shown: &str,
    target: &PathBuf,
    claims_dir: bool,
    inputs: &Inputs,
    manifest: &Manifest,
) {
    if manifest.ignore().covers(target, claims_dir) {
        return;
    }
    if !inputs.present.contains(target) {
        out.push(Finding::at(
            rel,
            line,
            format!("`{shown}` does not exist, at `{}`", target.display()),
            "repair the pointer, or delete it; a path that does not resolve is a guess",
        ));
        return;
    }
    let is_dir = inputs.directories.contains(target);
    if claims_dir && !is_dir {
        out.push(Finding::at(
            rel,
            line,
            format!("`{shown}` claims a directory and names a file"),
            "drop the trailing slash, or repair the path; the slash is the kind claim",
        ));
    } else if !claims_dir && is_dir {
        out.push(Finding::at(
            rel,
            line,
            format!("`{shown}` claims a file and names a directory"),
            "add the trailing slash, or repair the path; the slash is the kind claim",
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn link(
    out: &mut Vec<Finding>,
    counts: &mut Counts,
    doc: &crate::model::Document,
    line: u32,
    target: &str,
    nav: bool,
    inputs: &Inputs,
    manifest: &Manifest,
) {
    // A bare fragment stays on the page; on a file target a fragment rides along and is
    // dropped before resolution. A scheme leaves the project and is not this check's to
    // resolve; an absolute path is NOT passed over — it reaches `refused` below, under the
    // same leading-slash arm an anchored path meets.
    if target.starts_with('#') {
        return;
    }
    let file_part = target.split('#').next().unwrap_or(target);
    if has_scheme(file_part) {
        return;
    }
    counts.links += 1;
    if !nav {
        out.push(Finding::at(
            &doc.rel,
            line,
            format!("`{target}` is linked from a file that is not a navigation home"),
            "a relative link is an index row and lives in a README.md or an index.md; in \
             prose, point with a backticked `path@<anchor>@<path>` reference",
        ));
        return;
    }
    if let Some(why) = refused(file_part) {
        out.push(Finding::at(
            &doc.rel,
            line,
            format!("the link `{target}` is refused: {why}"),
            "link with a plain path relative to this file",
        ));
        return;
    }
    let dir = doc.rel.parent().unwrap_or(Path::new(""));
    let resolved = dir.join(file_part.trim_end_matches('/'));
    assert_target(
        out,
        &doc.rel,
        line,
        target,
        &resolved,
        file_part.ends_with('/'),
        inputs,
        manifest,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;

    // Every fixture spells its reference inline: the checker reads no string literal of its
    // own source, per `knowledge#checker-source-literals-are-data`.

    /// A manifest declaring one component beside the root, against a root nothing reads.
    fn manifest() -> Manifest {
        let text = "[project]\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n";
        Manifest::parse(std::path::Path::new("/nowhere"), text).expect("a declaration")
    }

    /// The same, with a `.gitignore` beside it.
    fn manifest_ignoring(lines: &str) -> Manifest {
        let mut m = manifest();
        m.set_ignore(crate::gitignore::Ignore::parse(lines).expect("a parsable ignore"));
        m
    }

    /// The findings and counts over documents, against a listing.
    fn checked_docs(
        manifest: &Manifest,
        docs: Vec<(&str, &str)>,
        present: &[String],
    ) -> (Vec<String>, Counts) {
        let model = Model::from_documents(
            docs.into_iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        let releases = HashMap::new();
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let directories = crate::check::testing::implied_directories(&present);
        let outside = Vec::new();
        let inputs = Inputs {
            releases: &releases,
            pinned: "",
            committed: &committed,
            present: &present,
            directories: &directories,
            outside: &outside,
        };
        let (found, counts) = check(&model, manifest, &inputs);
        (found.iter().map(|f| f.to_string()).collect(), counts)
    }

    /// The findings and counts over one document, placed under `notes/`.
    fn checked(manifest: &Manifest, text: &str, present: &[String]) -> (Vec<String>, Counts) {
        checked_in(manifest, "notes/prose.md", text, present)
    }

    /// The same, with the document at a chosen path — what the navigation rule reads.
    fn checked_in(
        manifest: &Manifest,
        at: &str,
        text: &str,
        present: &[String],
    ) -> (Vec<String>, Counts) {
        checked_docs(manifest, vec![(at, text)], present)
    }

    /// The mock component's tree: the document, and the directory chain above it.
    fn tree() -> Vec<String> {
        vec![
            "parts/a-part/notes/real/a.md".to_string(),
            "parts/a-part/notes/real".to_string(),
            "parts/a-part/notes".to_string(),
            "parts/a-part".to_string(),
            "parts".to_string(),
        ]
    }

    /// A design-home line defining the slug `a-decision`.
    fn head() -> &'static str {
        "### The decision `##a-decision`\n"
    }

    // --- table kinds -----------------------------------------------------------------

    #[test]
    fn a_reference_resolves_against_the_anchor_and_register_it_names() {
        // Both directions across the boundary, and a definition is owned by where its
        // document sits rather than by anything the line says.
        let (found, counts) = checked_docs(
            &manifest(),
            vec![
                (
                    "docs/design.md",
                    "### The decision `##a-decision`\n\nIt rests on `design@a-part@a-decision`.\n",
                ),
                (
                    "parts/a-part/docs/design.md",
                    "### The decision `##a-decision`\n\nIt rests on `design@a-project@a-decision`.\n",
                ),
            ],
            &[],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!((counts.entities, counts.references), (2, 2));
    }

    #[test]
    fn each_kind_resolves_in_its_own_register_and_not_in_another() {
        // The same word in the goals and tripwires homes, and a reference of each kind. A
        // design reference to the word is dangling: the register is part of the key.
        let (found, _) = checked_docs(
            &manifest(),
            vec![
                ("docs/goals.md", "## What it is for `##shared`\n"),
                ("docs/tripwires.md", "## Guarding it `##shared`\n"),
                (
                    "notes/a.md",
                    "`goal@a-project@shared` and `tripwire@a-project@shared` resolve;\n\
                     `design@a-project@shared` does not.\n",
                ),
            ],
            &[],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("notes/a.md:2") && found[0].contains("defines no design `shared`"),
            "{found:#?}"
        );
    }

    #[test]
    fn each_way_a_reference_resolves_to_nothing_is_reported_as_the_repair_it_needs() {
        // Unknown anchor, undefined id, and the two malformed counts; the anchor-lacks-
        // register arm is constructed in `entity`, no component lacking one.
        let (found, _) = checked_docs(
            &manifest(),
            vec![
                ("docs/design.md", head()),
                (
                    "docs/open-issues.md",
                    "Naming an anchor nothing declares: `design@nowhere@a-decision`.\n\
                     Naming one that does not define it: `design@a-part@a-decision`.\n\
                     Two segments: `design@a-project`.\n\
                     Four segments: `design@a-project@a@decision`.\n",
                ),
            ],
            &[],
        );
        assert_eq!(found.len(), 4, "{found:#?}");
        let at = |line: u32, needle: &str| {
            let prefix = format!("docs/open-issues.md:{line}  ");
            assert!(
                found
                    .iter()
                    .any(|f| f.starts_with(&prefix) && f.contains(needle)),
                "expected {needle:?} at line {line}: {found:#?}"
            );
        };
        at(1, "no anchor of this project");
        at(2, "defines no design `a-decision`");
        at(3, "is malformed: two segments");
        at(4, "is malformed: four or more segments");
        // The unknown-anchor repair lists the anchors, which is what makes it a repair.
        assert!(
            found.iter().any(|f| f.contains("a-project, a-part")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_reserved_anchor_serves_the_path_kind_alone() {
        // `*` and `elsewhere` are anchors for a path. Under a table kind they name nothing,
        // and the finding says which anchors do.
        let (found, _) = checked_docs(
            &manifest(),
            vec![
                ("docs/design.md", head()),
                (
                    "notes/a.md",
                    "`design@*@a-decision` and `design@elsewhere@a-decision`\n",
                ),
            ],
            &[],
        );
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found
                .iter()
                .all(|f| f.contains("no anchor of this project") && f.contains("path kind alone")),
            "{found:#?}"
        );
    }

    #[test]
    fn the_old_forms_are_reported_as_an_anchor_in_kind_position_naming_the_kinds() {
        // The migration's commonest leftover: `<anchor>@<path>`. Mutation checked: with the
        // `AnchorInKindPosition` arm pushing nothing, all three are silent.
        let (found, _) = checked(
            &manifest(),
            "`a-project@docs/design.md`, `*@docs/design.md` and `elsewhere@foreign/x`\n",
            &[],
        );
        assert_eq!(found.len(), 3, "{found:#?}");
        for f in &found {
            assert!(f.contains("where the kind goes"), "{f}");
            assert!(
                f.contains(&Kind::listed()),
                "the repair names the kinds: {f}"
            );
        }
    }

    #[test]
    fn a_span_whose_head_is_neither_kind_nor_anchor_is_silent() {
        // An email, a remote, a typo in the kind: not candidates. The typo is the accepted
        // gap, guarded by a tripwire.
        let (found, counts) = checked(
            &manifest(),
            "`user@example.test`, `git@host:x/y.git`, `desing@a-project@x`, `@docs/a.md`\n",
            &[],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.references, 0);
    }

    #[test]
    fn the_retired_forms_are_findings_naming_what_they_were() {
        let (found, counts) = checked(
            &manifest(),
            "`a-project#a-decision`, `#a-decision` and R15\n",
            &[],
        );
        assert_eq!(found.len(), 3, "{found:#?}");
        assert_eq!(
            found
                .iter()
                .filter(|f| f.contains("retired slug reference form"))
                .count(),
            2,
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.contains("`R15` is the retired interpretation entry number form")),
            "{found:#?}"
        );
        assert_eq!(counts.references, 0, "a retired form is not a reference");
    }

    #[test]
    fn a_fenced_reference_is_live_and_a_placeholder_is_silent() {
        // One stance for every kind: the sketch names what it names, and the illustration
        // writes angle brackets.
        let text = "```\n`design@a-project@nothing` and `design@<component>@<slug>`\n```\n";
        let (found, counts) = checked(&manifest(), text, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("defines no design `nothing`"),
            "{found:#?}"
        );
        assert_eq!(counts.references, 1);
    }

    #[test]
    fn the_definition_findings_are_reported_through_this_family() {
        // Misplaced and duplicate definitions come out of building the table, and this
        // family carries them until the register-shape family exists.
        let (found, counts) = checked_docs(
            &manifest(),
            vec![
                (
                    "docs/design.md",
                    "### One `##twice`\n\n### Two `##twice`\n\n#### Deep `##deep`\n",
                ),
                ("notes/a.md", "### Stray `##stray`\n"),
            ],
            &[],
        );
        assert_eq!(found.len(), 4, "{found:#?}");
        assert_eq!(
            found
                .iter()
                .filter(|f| f.contains("defines nothing"))
                .count(),
            2,
            "{found:#?}"
        );
        assert_eq!(
            found
                .iter()
                .filter(|f| f.contains("is also defined at"))
                .count(),
            2,
            "{found:#?}"
        );
        assert_eq!(counts.entities, 1, "one entity, defined twice");
    }

    #[test]
    fn the_counts_are_entities_reference_occurrences_and_links() {
        // Occurrences for references, because every one is judged; distinct entities for
        // the table, because that is what it holds.
        let (found, counts) = checked_docs(
            &manifest(),
            vec![
                ("docs/design.md", head()),
                (
                    "docs/open-issues.md",
                    "`design@a-project@a-decision` and again `design@a-project@a-decision`.\n",
                ),
                (
                    "README.md",
                    "`design@a-project@a-decision` a third time, and [a row](CLAUDE.md).\n",
                ),
            ],
            &["CLAUDE.md".to_string()],
        );
        assert!(found.is_empty(), "{found:#?}");
        assert_eq!(
            (counts.entities, counts.references, counts.links),
            (1, 3, 1)
        );
    }

    // --- the path kind ---------------------------------------------------------------

    #[test]
    fn a_path_resolves_beside_its_anchor_and_a_dangling_one_is_reported() {
        let m = manifest();
        let (found, counts) = checked(&m, "See `path@a-part@notes/real/a.md`.\n", &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.references, 1);
        let (found, _) = checked(&m, "See `path@a-part@notes/gone.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
    }

    #[test]
    fn the_root_anchors_by_the_projects_name() {
        let m = manifest();
        let present = vec!["notes/root.md".to_string(), "notes".to_string()];
        let (found, _) = checked(&m, "See `path@a-project@notes/root.md`.\n", &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn an_unknown_path_anchor_is_reported_naming_the_anchors() {
        let m = manifest();
        let (found, _) = checked(&m, "See `path@nonesuch@notes/real/a.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("no anchor") && found[0].contains("a-project, a-part"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_path_may_hold_an_at_sign() {
        // The id is everything after the second `@`, so a path with one resolves.
        let m = manifest();
        let present = vec!["notes/a@b.md".to_string(), "notes".to_string()];
        let (found, counts) = checked(&m, "See `path@a-project@notes/a@b.md`.\n", &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.references, 1);
    }

    #[test]
    fn a_trailing_slash_claims_a_directory_and_its_absence_claims_a_file() {
        // Both directions of the kind claim, plus the two matching shapes staying silent.
        let m = manifest();
        let (found, _) = checked(&m, "See `path@a-part@notes/real/`.\n", &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        let (found, _) = checked(&m, "See `path@a-part@notes/real`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("claims a file and names a directory"),
            "{found:#?}"
        );
        let (found, _) = checked(&m, "See `path@a-part@notes/real/a.md/`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("claims a directory and names a file"),
            "{found:#?}"
        );
    }

    #[test]
    fn an_upward_a_current_and_an_absolute_path_are_refused() {
        let m = manifest();
        for path in [
            "../notes/real/a.md",
            "./notes/real/a.md",
            "/notes/real/a.md",
        ] {
            let (found, _) = checked(&m, &format!("See `path@a-part@{path}`.\n"), &tree());
            assert_eq!(found.len(), 1, "{path}: {found:#?}");
            assert!(found[0].contains("is refused"), "{path}: {found:#?}");
        }
    }

    #[test]
    fn a_root_reference_reaching_inside_a_component_is_reported() {
        // What makes a component move cost one manifest line: the reference must anchor at
        // the component, so no document names the component's location.
        let m = manifest();
        let (found, _) = checked(
            &m,
            "See `path@a-project@parts/a-part/notes/real/a.md`.\n",
            &tree(),
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("reaches inside the anchor") && found[0].contains("a-part"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_components_own_directory_is_named_from_an_ancestor() {
        let m = manifest();
        let (found, _) = checked(&m, "See `path@a-project@parts/a-part/`.\n", &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn the_escape_anchor_is_exempt_unless_its_target_resolves_here() {
        let m = manifest();
        let (found, counts) = checked(
            &m,
            "See `path@elsewhere@foreign-project/src/thing.java`.\n",
            &tree(),
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(
            counts.references, 1,
            "an escape is still a counted reference"
        );
        let (found, _) = checked(&m, "See `path@elsewhere@notes/real/a.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in this tree"), "{found:#?}");
    }

    #[test]
    fn the_generic_anchor_accepts_the_required_set_and_what_a_component_carries() {
        let m = manifest();
        // A required document name passes with no component carrying it in the listing,
        // and every heading register's shapes are in the accepted set.
        let homes = "See `path@*@docs/design.md`, `path@*@docs/design/`, `path@*@docs/goals.md`, \
                     `path@*@docs/goals/`, `path@*@docs/tripwires/README.md`.\n";
        let (found, _) = checked(&m, homes, &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        // A path one component carries passes; one nobody carries is reported.
        let (found, _) = checked(&m, "See `path@*@notes/real/a.md`.\n", &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        let (found, _) = checked(&m, "See `path@*@notes/nowhere.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
    }

    #[test]
    fn the_generic_anchor_cannot_reach_inside_a_component_from_above() {
        let m = manifest();
        let (found, _) = checked(&m, "See `path@*@parts/a-part/notes/real/a.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
    }

    #[test]
    fn a_required_name_still_carries_the_kind_claim() {
        let m = manifest();
        let (found, _) = checked(&m, "See `path@*@docs/design.md/`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("wrong kind"), "{found:#?}");
        let (found, _) = checked(&m, "See `path@*@docs/goals`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("wrong kind"), "{found:#?}");
    }

    #[test]
    fn a_gitignored_copy_is_not_a_generic_hit() {
        let m = manifest_ignoring("scratch/\n");
        let mut present = tree();
        present.push("scratch".to_string());
        present.push("scratch/x.md".to_string());
        let (found, _) = checked(&m, "See `path@*@scratch/x.md`.\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
    }

    #[test]
    fn a_gitignored_target_is_exempt_from_existence_and_kind() {
        let m = manifest_ignoring("generated/\n");
        let text = "See `path@a-project@generated/out.bin` and `path@a-project@generated/`.\n";
        let (found, _) = checked(&m, text, &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        let (found, _) = checked(&manifest(), text, &tree());
        assert_eq!(found.len(), 2, "{found:#?}");
    }

    #[test]
    fn an_unanchored_path_shape_is_a_finding_naming_the_grammar() {
        let m = manifest();
        let (found, _) = checked(&m, "See `notes/real/a.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("names no anchor") && found[0].contains("path@<anchor>@<path>"),
            "{found:#?}"
        );
    }

    // --- links -----------------------------------------------------------------------

    #[test]
    fn an_absolute_link_target_is_refused_rather_than_passed_over() {
        let m = manifest();
        let present = vec![
            "parts/a-part/notes/real/a.md".to_string(),
            "parts/a-part".to_string(),
        ];
        let at = "parts/a-part/notes/README.md";
        let (found, _) = checked_in(&m, at, "[x](/no/such/place.md)\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("is refused"), "{found:#?}");
    }

    #[test]
    fn a_relative_link_is_legal_in_a_navigation_file_and_reported_elsewhere() {
        let m = manifest();
        let text = "[a](real/a.md)\n";
        let present = vec![
            "parts/a-part/notes/real/a.md".to_string(),
            "parts/a-part/notes/real".to_string(),
            "parts/a-part/notes".to_string(),
            "parts/a-part".to_string(),
        ];
        let at = "parts/a-part/notes/README.md";
        let (found, counts) = checked_in(&m, at, text, &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.links, 1);
        let index = "parts/a-part/notes/index.md";
        let (found, _) = checked_in(&m, index, text, &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        let prose = "parts/a-part/notes/prose.md";
        let (found, _) = checked_in(&m, prose, text, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("not a navigation home"), "{found:#?}");
    }

    #[test]
    fn a_navigation_link_resolves_beside_its_file_and_a_dangling_one_is_reported() {
        let m = manifest();
        let present = vec![
            "parts/a-part/notes/real/a.md".to_string(),
            "parts/a-part/notes/real".to_string(),
            "parts/a-part/notes".to_string(),
            "parts/a-part".to_string(),
        ];
        let at = "parts/a-part/notes/README.md";
        let (found, _) = checked_in(&m, at, "[gone](real/gone.md)\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
        let ok = "[a](real/a.md#top) [site](https://a.test/x) [up](#head)\n";
        let (found, counts) = checked_in(&m, at, ok, &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.links, 1, "only the relative link is this check's");
    }

    #[test]
    fn a_navigation_link_with_an_upward_segment_is_refused() {
        let m = manifest();
        let present = vec![
            "parts/a-part/notes/real/a.md".to_string(),
            "parts/a-part".to_string(),
        ];
        let at = "parts/a-part/notes/real/README.md";
        let (found, _) = checked_in(&m, at, "[a](../real/a.md)\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("is refused"), "{found:#?}");
    }
}
