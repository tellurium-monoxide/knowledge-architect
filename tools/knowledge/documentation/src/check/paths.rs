//! Every path reference resolves under its anchor, and every relative markdown link is a
//! navigation row that resolves beside its file.
//!
//! Slugs were checked and paths were not, so a rename once left nine references to a deleted
//! file sitting in live documents while every checker reported green. A pointer with no
//! enforcement is a guess. The anchored grammar is the other half of the repair: a reference
//! names the component it resolves under, so moving a component edits the manifest and no
//! document, a trailing slash claims the target's kind, and a span shaped like a path that
//! follows no accepted syntax is reported rather than silently unread.

use std::path::{Path, PathBuf};

use crate::finding::Finding;
use crate::manifest::{
    Components, Manifest, COMPONENT_DOCUMENTS, DESIGN_DIR, DESIGN_FILE, DESIGN_README,
};
use crate::model::Model;
use crate::scan::{Observation, PathAnchor, ESCAPE_ANCHOR};

use super::Inputs;

/// What the check looked at: anchored references (the escape and generic forms included),
/// and the relative markdown links it resolved.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub references: usize,
    pub links: usize,
}

pub fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
    let components = manifest.components();
    let mut out = Vec::new();
    let mut counts = Counts::default();
    for doc in model.documents() {
        let nav = is_navigation(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::PathRef { anchor, path } => {
                    counts.references += 1;
                    reference(
                        &mut out,
                        &doc.rel,
                        l.line,
                        anchor,
                        path,
                        &components,
                        inputs,
                        manifest,
                    );
                }
                Observation::UnsupportedPath(span) => {
                    out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{span}` is shaped like a path and follows no accepted syntax"),
                        "anchor it — `<component>@<path>`, `elsewhere@<path>` for a path outside \
                         this tree, `*@<path>` for every component's own copy — or rephrase so \
                         the span is not path-shaped",
                    ));
                }
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
/// a backticked anchored path, and a link is reported.
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

#[allow(clippy::too_many_arguments)]
fn reference(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    anchor: &PathAnchor,
    path: &str,
    components: &Components,
    inputs: &Inputs,
    manifest: &Manifest,
) {
    // The trailing slash is the writer's claim about the target's kind; the lookup drops it.
    let claims_dir = path.ends_with('/');
    let trimmed = path.trim_end_matches('/');
    match anchor {
        PathAnchor::Component(name) => {
            let Some(cr) = components.by_name(name) else {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{name}` is no component of this project"),
                    "repair the anchor; a path anchors at a declared component, at the \
                     project by its name, at the escape anchor for a path outside this \
                     tree, or at `*` for every component's own copy",
                ));
                return;
            };
            if let Some(why) = refused(trimmed) {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{name}@{path}` is refused: {why}"),
                    "anchor at the component that holds the target, with a plain relative \
                     path",
                ));
                return;
            }
            let target = cr.path.join(trimmed);
            // The deepest anchor wins: a reference reaching inside another component breaks
            // when that component moves, and the anchor is what a move must not break.
            let owner = components.owning(&target);
            if owner.path != cr.path {
                out.push(Finding::at(
                    rel,
                    line,
                    format!(
                        "`{name}@{path}` reaches inside the component `{}`",
                        owner.name
                    ),
                    "anchor at the deepest component holding the target, so a component \
                     move edits knowledge.toml and no document",
                ));
                return;
            }
            assert_target(
                out,
                rel,
                line,
                &format!("{name}@{path}"),
                &target,
                claims_dir,
                inputs,
                manifest,
            );
        }
        PathAnchor::Elsewhere => {
            // The escape is exempt from existence and kind assertions — there is nothing
            // stable to assert — and from the shape refusals, because a foreign layout may
            // spell anything. The one assertion is that it stays honest: a target that
            // resolves here is a real reference wearing the escape, which would otherwise
            // be the cheap way to silence the unsupported-shape finding.
            let resolving = components
                .all()
                .iter()
                .find(|c| inputs.present.contains(&c.path.join(trimmed)));
            if let Some(c) = resolving {
                out.push(Finding::at(
                    rel,
                    line,
                    format!(
                        "`{ESCAPE_ANCHOR}@{path}` resolves in this tree, beside `{}`",
                        c.name
                    ),
                    "the escape anchor is for a path this tree does not hold; anchor the \
                     reference at the component that holds it",
                ));
            }
        }
        PathAnchor::Every => {
            if let Some(why) = refused(trimmed) {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`*@{path}` is refused: {why}"),
                    "the generic form takes a plain component-relative path",
                ));
                return;
            }
            // The compiled-in required set is what the generic form usually names, and it
            // is accepted whether or not any component carries the shape yet — the two
            // design homes are the case, where naming both shapes is legitimate while only
            // one is in use anywhere.
            let required = COMPONENT_DOCUMENTS.contains(&trimmed)
                || trimmed == DESIGN_FILE
                || trimmed == DESIGN_DIR
                || trimmed == DESIGN_README;
            if required {
                return;
            }
            // Anything else must be real somewhere, with the claimed kind: a generic
            // reference nothing resolves rots exactly like a dangling one.
            let hit = components.all().iter().any(|c| {
                let t = c.path.join(trimmed);
                inputs.present.contains(&t) && claims_dir == inputs.directories.contains(&t)
            });
            if !hit {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`*@{path}` resolves in no component"),
                    "the generic form names a required document, or a path at least one \
                     component carries; repair the path, or anchor at one component",
                ));
            }
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
    // dropped before resolution. A scheme and an absolute path leave the project and are
    // not this check's to resolve.
    if target.starts_with('#') {
        return;
    }
    let file_part = target.split('#').next().unwrap_or(target);
    if has_scheme(file_part) || file_part.starts_with('/') {
        return;
    }
    counts.links += 1;
    if !nav {
        out.push(Finding::at(
            &doc.rel,
            line,
            format!("`{target}` is linked from a file that is not a navigation home"),
            "a relative link is an index row and lives in a README.md or an index.md; in \
             prose, point with an anchored backticked path",
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

    // Interpolated, never spelled out: the tool's own source is walked, so a path or an
    // anchored reference written literally here would be real content.
    const PART: &str = "parts/a-part";
    const NAME: &str = "a-part";
    const PROJECT: &str = "a-project";
    const DOC: &str = "notes/real/a.md";
    const DIR: &str = "notes/real/";
    const NOWHERE: &str = "notes/nowhere.md";

    /// A manifest declaring one component beside the root, against a root nothing reads.
    fn manifest() -> Manifest {
        let text = format!(
            "[project]\nname = \"a-project\"\ncomponents = [\"{PART}\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             [lint]\nexempt-files = []\n\n\
             [rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\n\
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n\n\
             [interpretations]\ndir = \"i\"\nconcerns = []\n"
        );
        Manifest::parse(std::path::Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The same, with a `.gitignore` beside it.
    fn manifest_ignoring(lines: &str) -> Manifest {
        let mut m = manifest();
        m.set_ignore(crate::gitignore::Ignore::parse(lines).expect("a parsable ignore"));
        m
    }

    /// The findings and counts over one document, against a listing.
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
        let model = Model::from_documents(vec![(PathBuf::from(at), text.to_string())]);
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

    /// The mock component's tree: the document, and the directory chain above it.
    fn tree() -> Vec<String> {
        vec![
            format!("{PART}/{DOC}"),
            format!("{PART}/notes/real"),
            format!("{PART}/notes"),
            PART.to_string(),
            "parts".to_string(),
        ]
    }

    #[test]
    fn a_reference_resolves_beside_its_component_and_a_dangling_one_is_reported() {
        let m = manifest();
        let (found, counts) = checked(&m, &format!("See `{NAME}@{DOC}`.\n"), &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.references, 1);
        let gone = "notes/gone.md";
        let (found, _) = checked(&m, &format!("See `{NAME}@{gone}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
    }

    #[test]
    fn the_root_anchors_by_the_projects_name() {
        let m = manifest();
        let at_root = "notes/root.md";
        let present = vec![at_root.to_string(), "notes".to_string()];
        let (found, _) = checked(&m, &format!("See `{PROJECT}@{at_root}`.\n"), &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn an_unknown_anchor_is_reported_as_no_component() {
        let m = manifest();
        let (found, _) = checked(&m, &format!("See `nonesuch@{DOC}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no component"), "{found:#?}");
    }

    #[test]
    fn a_trailing_slash_claims_a_directory_and_its_absence_claims_a_file() {
        // Both directions of the kind claim, plus the two matching shapes staying silent.
        let m = manifest();
        let (found, _) = checked(&m, &format!("See `{NAME}@{DIR}`.\n"), &tree());
        assert_eq!(
            found,
            Vec::<String>::new(),
            "a directory with its slash: {found:#?}"
        );
        let trimmed = DIR.trim_end_matches('/');
        let (found, _) = checked(&m, &format!("See `{NAME}@{trimmed}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("claims a file and names a directory"),
            "{found:#?}"
        );
        let (found, _) = checked(&m, &format!("See `{NAME}@{DOC}/`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("claims a directory and names a file"),
            "{found:#?}"
        );
    }

    #[test]
    fn an_upward_a_current_and_an_absolute_path_are_refused() {
        // Mutation checked: with the refusal deleted, the upward form resolves and the
        // first assertion sees no finding.
        let m = manifest();
        for path in [format!("../{DOC}"), format!("./{DOC}"), format!("/{DOC}")] {
            let (found, _) = checked(&m, &format!("See `{NAME}@{path}`.\n"), &tree());
            assert_eq!(found.len(), 1, "{path}: {found:#?}");
            assert!(found[0].contains("is refused"), "{path}: {found:#?}");
        }
    }

    #[test]
    fn a_root_reference_reaching_inside_a_component_is_reported() {
        // What makes a component move cost one manifest line: the reference must anchor at
        // the component, so no document names the component's location.
        let m = manifest();
        let (found, _) = checked(&m, &format!("See `{PROJECT}@{PART}/{DOC}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("reaches inside the component"),
            "{found:#?}"
        );
        assert!(found[0].contains(NAME), "{found:#?}");
    }

    #[test]
    fn the_escape_anchor_is_exempt_unless_its_target_resolves_here() {
        // The one assertion the escape carries, so it cannot silence the lint on a real
        // path. Mutation checked: with the resolution test deleted, the second half sees
        // no finding.
        let m = manifest();
        let foreign = "foreign-project/src/thing.java";
        let (found, counts) = checked(&m, &format!("See `{ESCAPE_ANCHOR}@{foreign}`.\n"), &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(
            counts.references, 1,
            "an escape is still a counted reference"
        );
        let (found, _) = checked(&m, &format!("See `{ESCAPE_ANCHOR}@{DOC}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in this tree"), "{found:#?}");
    }

    #[test]
    fn the_generic_anchor_accepts_the_required_set_and_what_a_component_carries() {
        let m = manifest();
        // A required document name passes with no component carrying it in the listing,
        // and both design-home shapes are in the accepted set.
        let homes = format!("See `*@{DESIGN_FILE}` and `*@{DESIGN_DIR}/`.\n");
        let (found, _) = checked(&m, &homes, &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        // A path one component carries passes; one nobody carries is reported.
        let (found, _) = checked(&m, &format!("See `*@{DOC}`.\n"), &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        let (found, _) = checked(&m, &format!("See `*@{NOWHERE}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
    }

    #[test]
    fn an_unsupported_path_shape_is_a_finding_naming_the_accepted_forms() {
        // The enforcement half of the grammar: the retired bare form is reported, never
        // silently unresolved.
        let m = manifest();
        let (found, _) = checked(&m, &format!("See `{DOC}`.\n"), &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("follows no accepted syntax"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_gitignored_target_is_exempt_from_existence_and_kind() {
        // Decided by the ignore rules rather than by presence, so the verdict on a fresh
        // clone equals the verdict on a built tree. The target here is in neither listing.
        let m = manifest_ignoring("generated/\n");
        let out_file = "generated/out.bin";
        let out_dir = "generated/";
        let text = format!("See `{PROJECT}@{out_file}` and `{PROJECT}@{out_dir}`.\n");
        let (found, _) = checked(&m, &text, &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        // The same references with no ignore rule are two findings.
        let (found, _) = checked(&manifest(), &text, &tree());
        assert_eq!(found.len(), 2, "{found:#?}");
    }

    #[test]
    fn a_relative_link_is_legal_in_a_navigation_file_and_reported_elsewhere() {
        let m = manifest();
        let text = "[a](real/a.md)\n";
        let present = vec![
            format!("{PART}/{DOC}"),
            format!("{PART}/notes/real"),
            format!("{PART}/notes"),
            PART.to_string(),
        ];
        let at = format!("{PART}/notes/README.md");
        let (found, counts) = checked_in(&m, &at, text, &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.links, 1);
        let index = format!("{PART}/notes/index.md");
        let (found, _) = checked_in(&m, &index, text, &present);
        assert_eq!(
            found,
            Vec::<String>::new(),
            "an index.md is a navigation home: {found:#?}"
        );
        let prose = format!("{PART}/notes/prose.md");
        let (found, _) = checked_in(&m, &prose, text, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("not a navigation home"), "{found:#?}");
    }

    #[test]
    fn a_navigation_link_resolves_beside_its_file_and_a_dangling_one_is_reported() {
        let m = manifest();
        let present = vec![
            format!("{PART}/{DOC}"),
            format!("{PART}/notes/real"),
            format!("{PART}/notes"),
            PART.to_string(),
        ];
        let at = format!("{PART}/notes/README.md");
        let (found, _) = checked_in(&m, &at, "[gone](real/gone.md)\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
        // A fragment rides along and is dropped; a URL and a bare fragment are passed over.
        let ok = "[a](real/a.md#top) [site](https://a.test/x) [up](#head)\n";
        let (found, counts) = checked_in(&m, &at, ok, &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.links, 1, "only the relative link is this check's");
    }

    #[test]
    fn a_navigation_link_with_an_upward_segment_is_refused() {
        // The ban is against staleness under relocation, and it is what lets a directory
        // move wholesale with its links intact.
        let m = manifest();
        let present = vec![format!("{PART}/{DOC}"), PART.to_string()];
        let at = format!("{PART}/notes/real/README.md");
        let (found, _) = checked_in(&m, &at, "[a](../real/a.md)\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("is refused"), "{found:#?}");
    }
}
