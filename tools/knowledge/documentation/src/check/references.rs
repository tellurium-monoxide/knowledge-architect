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
//! **The definition-site findings are not this family's.** Misplaced, malformed and duplicate
//! definitions are found while the table is built, and `check::registers` reports them: where a
//! definition may sit is a question about a register's shape.

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
    check_under(model, inputs, &Anchors::of(manifest))
}

/// The same, over a stated anchor list.
///
/// What `check` derives from the manifest, a test states directly: every component carries
/// every register, so the anchor-lacks-register arm is reachable only through an anchor with a
/// declared register subset, the shape a location takes.
pub fn check_under(model: &Model, inputs: &Inputs, anchors: &Anchors) -> (Vec<Finding>, Counts) {
    let entities = Entities::build(model, anchors);
    judge(model.documents(), &entities, anchors, inputs)
}

/// The same, over a stated document list and a stated entity table.
///
/// **The two are separated because a commit message is judged against a table it is no part
/// of.** A message is a document under the regime, per
/// `knowledge#a-commit-message-is-a-document`, and the entities it names are defined by the
/// tree it commits — so the table is built from that tree's model and the documents judged
/// against it are these. `check_under` is the case where the two coincide.
pub fn judge(
    docs: &[crate::model::Document],
    entities: &Entities,
    anchors: &Anchors,
    inputs: &Inputs,
) -> (Vec<Finding>, Counts) {
    let mut out: Vec<Finding> = Vec::new();
    let mut counts = Counts {
        entities: entities.len(),
        ..Counts::default()
    };
    for doc in docs {
        let nav = is_navigation(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::Span(span) => match entity::candidate(span, anchors) {
                    Candidate::Reference { kind, anchor, id } => {
                        counts.references += 1;
                        if kind.is_path() {
                            path(
                                &mut out, &doc.rel, l.line, span, anchor, id, anchors, inputs,
                            );
                        } else {
                            table(
                                &mut out, &doc.rel, l.line, span, &kind, anchor, id, anchors,
                                entities,
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
                            anchors.kinds_listed()
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
                    "a bare `R` and digits is no longer read as a reference: a mention names \
                     the entry in the `<kind>@<anchor>@<id>` grammar, and an entry's own \
                     heading loses the number when the register becomes one file per entry",
                )),
                // Markdown documents only: in Rust prose a markdown link is rustdoc's
                // mechanism, resolved by rustdoc against the crate namespace, and this
                // check reading those as index rows would report every intra-doc link.
                Observation::Link(target) if doc.is_markdown() => {
                    link(&mut out, &mut counts, doc, l.line, target, nav, inputs);
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
    kind: &Kind,
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
                anchors
                    .registers()
                    .by_name(kind.name())
                    .map(|r| r.dir.as_str())
                    .unwrap_or("register")
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
            if let Some(why) = refused(path) {
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
            if let Some(required_dir) = anchors.required_kind(trimmed) {
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
            // state, and a verdict may not depend on the checking machine's. Presence and
            // kind are asked apart, so a path some component carries under the other kind
            // gets the kind-claim repair rather than "repair the path".
            let carried: Vec<PathBuf> = anchors
                .all()
                .iter()
                .map(|a| (a, a.path.join(trimmed)))
                .filter(|(a, t)| {
                    anchors.owning(t).path == a.path
                        && !inputs
                            .ignored
                            .contains(&crate::git::ignore_query(t, claims_dir))
                        && inputs.present.contains(t)
                })
                .map(|(_, t)| t)
                .collect();
            if carried.is_empty() {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` resolves in no component"),
                    "the generic form names a required document, or a path at least one \
                     component carries; repair the path, or anchor at one component",
                ));
            } else if !carried
                .iter()
                .any(|t| claims_dir == inputs.directories.contains(t))
            {
                let is_dir = inputs.directories.contains(&carried[0]);
                if is_dir {
                    out.push(Finding::at(
                        rel,
                        line,
                        format!("`{span}` claims a file and names a directory"),
                        "add the trailing slash, or repair the path; the slash is the kind claim",
                    ));
                } else {
                    out.push(Finding::at(
                        rel,
                        line,
                        format!("`{span}` claims a directory and names a file"),
                        "drop the trailing slash, or repair the path; the slash is the kind claim",
                    ));
                }
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
            // Judged on the path as written: a lone `/` trims to nothing and would resolve
            // to the anchor's own directory, which has no spelling under its own name.
            if let Some(why) = refused(path) {
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
            assert_target(out, rel, line, span, &target, claims_dir, inputs);
        }
    }
}

/// Assert one resolved target: it exists and has the claimed kind, unless the ignore rules
/// cover it.
///
/// The exemption runs on the ignore RULES, as `git check-ignore` states them, rather than on
/// what happens to exist, so a reference to a generated path passes on a fresh clone exactly
/// as it passes on a built tree — a verdict that depends on build state is a check nobody can
/// trust twice. The batch was taken by the caller over `ignore_queries` below, so a target
/// this function asks about that the collector did not gather reads as not ignored.
#[allow(clippy::too_many_arguments)]
fn assert_target(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    shown: &str,
    target: &PathBuf,
    claims_dir: bool,
    inputs: &Inputs,
) {
    if inputs
        .ignored
        .contains(&crate::git::ignore_query(target, claims_dir))
    {
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
    );
}

/// Every spelling a path reference in this model could ask the ignore rules about.
///
/// The caller batches these through one `git check-ignore` and hands the answers back in
/// `Inputs::ignored`, because a check may spawn nothing. **Every arm that reaches
/// `assert_target` is here**, and there are three: an anchored reference resolves under the
/// anchor it names, the generic form resolves under every anchor at once, and a relative
/// markdown link resolves beside its own file. The escape anchor resolves under none, so it
/// contributes nothing.
///
/// **A spelling this misses is a target the check reads as not ignored**, which is a finding
/// rather than a silence: the reference is asserted to exist. `an_ignored_target_is_exempt_in_
/// every_arm_the_collector_gathers` is what holds the two lists together.
pub fn ignore_queries(model: &Model, anchors: &Anchors) -> Vec<String> {
    let mut out = Vec::new();
    for doc in model.documents() {
        let nav = is_navigation(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::Span(span) => {
                    let Candidate::Reference { kind, anchor, id } =
                        entity::candidate(span, anchors)
                    else {
                        continue;
                    };
                    if !kind.is_path() {
                        continue;
                    }
                    let claims_dir = id.ends_with('/');
                    let trimmed = id.trim_end_matches('/');
                    match anchor {
                        ESCAPE_ANCHOR => {}
                        entity::EVERY_ANCHOR => {
                            for a in anchors.all() {
                                out.push(crate::git::ignore_query(
                                    &a.path.join(trimmed),
                                    claims_dir,
                                ));
                            }
                        }
                        name => {
                            if let Some(a) = anchors.by_name(name) {
                                out.push(crate::git::ignore_query(
                                    &a.path.join(trimmed),
                                    claims_dir,
                                ));
                            }
                        }
                    }
                }
                // The arms `link` returns on before resolving are the arms that ask git
                // nothing, in the order it takes them.
                Observation::Link(target) if doc.is_markdown() && nav => {
                    if target.starts_with('#') {
                        continue;
                    }
                    let file_part = target.split('#').next().unwrap_or(target);
                    if has_scheme(file_part) || refused(file_part).is_some() {
                        continue;
                    }
                    let dir = doc.rel.parent().unwrap_or(Path::new(""));
                    let resolved = dir.join(file_part.trim_end_matches('/'));
                    out.push(crate::git::ignore_query(
                        &resolved,
                        file_part.ends_with('/'),
                    ));
                }
                _ => {}
            }
        }
    }
    out.sort();
    out.dedup();
    out
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
             version = \"v\"\npast = \"p\"\nmanifest = \"m\"\n";
        Manifest::parse(std::path::Path::new("/nowhere"), text).expect("a declaration")
    }

    /// The findings and counts over documents, against a listing.
    fn checked_docs(
        manifest: &Manifest,
        docs: Vec<(&str, &str)>,
        present: &[String],
    ) -> (Vec<String>, Counts) {
        checked_under(docs, present, &Anchors::of(manifest))
    }

    /// The same, over a stated anchor list.
    fn checked_under(
        docs: Vec<(&str, &str)>,
        present: &[String],
        anchors: &Anchors,
    ) -> (Vec<String>, Counts) {
        checked_ignoring_under(docs, present, &[], anchors)
    }

    /// The same, with a stated batch of ignore answers.
    ///
    /// A spelling here is what `git::ignore_query` produces and what one `git check-ignore`
    /// batch would have returned; no test spawns git, which is the same reason a check may not.
    fn checked_ignoring_under(
        docs: Vec<(&str, &str)>,
        present: &[String],
        ignored: &[&str],
        anchors: &Anchors,
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
            configs: &HashMap::new(),
            present: &present,
            directories: &directories,
            outside: &outside,
            ignored: &ignored.iter().map(|s| (*s).to_string()).collect(),
            tracked_and_ignored: &[],
        };
        let (found, counts) = check_under(&model, &inputs, anchors);
        (found.iter().map(|f| f.to_string()).collect(), counts)
    }

    /// The findings and counts over one document, placed under `notes/`.
    fn checked(manifest: &Manifest, text: &str, present: &[String]) -> (Vec<String>, Counts) {
        checked_in(manifest, "notes/prose.md", text, present)
    }

    /// The same, with a stated batch of ignore answers.
    fn checked_ignoring(
        manifest: &Manifest,
        text: &str,
        present: &[String],
        ignored: &[&str],
    ) -> (Vec<String>, Counts) {
        checked_ignoring_under(
            vec![("notes/prose.md", text)],
            present,
            ignored,
            &Anchors::of(manifest),
        )
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
    fn an_anchor_carrying_no_such_register_is_reported_with_the_anchors_that_do() {
        // The third way of the four, over an anchor list no manifest produces today: an
        // anchor carrying tripwires alone. Mutation checked: with the arm's push replaced
        // by a drop, the assertion on one finding fails.
        let m = manifest();
        let base = Anchors::of(&m);
        let root = base.by_name("a-project").expect("the root").clone();
        let bare = crate::entity::Anchor::location(
            "bare",
            std::path::Path::new("bare"),
            vec!["tripwire".to_string()],
        );
        let anchors = Anchors::from_list(vec![root, bare], base.registers().clone());
        let (found, _) = checked_under(
            vec![
                ("docs/design.md", head()),
                ("notes/a.md", "`design@bare@a-decision`\n"),
            ],
            &[],
            &anchors,
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("`bare`, which carries no design register")
                && found[0].contains("the anchors that carry one: a-project"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_markdown_link_in_a_rust_doc_comment_is_rustdocs_and_not_a_navigation_row() {
        // Resolved by rustdoc against the crate namespace; reading it as an index row would
        // report every intra-doc link. Mutation checked: with the `is_markdown` guard made
        // always true, the dangling target below is reported.
        let (found, counts) = checked_in(
            &manifest(),
            "src/lib.rs",
            "/// see [the type](notes/gone.md)\nfn f() {}\n",
            &[],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.links, 0);
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
                f.contains(&Anchors::of(&manifest()).kinds_listed()),
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
            "`user@example.test`, `git@host:x/y.git`, `desing@a-project@x`, `@Test`\n",
            &[],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.references, 0);
        // An empty head in front of a path shape is not silent: it had a check before the
        // grammar and keeps one.
        let (found, _) = checked(&manifest(), "the retired escape `@docs/a.md`\n", &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("is malformed: the kind segment is empty"),
            "{found:#?}"
        );
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
    fn the_definition_findings_belong_to_the_register_family_and_not_to_this_one() {
        // Where a definition may sit is a question about a register's shape, so
        // `check::registers` reports it. This family still resolves against the table the
        // same build produced, so the entity count is unaffected by the move.
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
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
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
    fn a_generic_path_a_component_carries_under_the_other_kind_gets_the_kind_claim_repair() {
        // The path exists; only the slash claim is wrong. "Repair the path" would send the
        // reader to a file that is there. Mutation checked: with the kind test folded back
        // into the presence test, both report "resolves in no component".
        let m = manifest();
        let (found, _) = checked(&m, "See `path@*@notes/real/a.md/`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("claims a directory and names a file"),
            "{found:#?}"
        );
        let (found, _) = checked(&m, "See `path@*@notes/real`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("claims a file and names a directory"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_lone_slash_is_refused_rather_than_resolving_to_the_anchors_own_directory() {
        // Trimmed, `/` is nothing, and nothing joined to the anchor's path is the anchor's
        // own directory, which has no spelling under its own name.
        let m = manifest();
        for span in ["path@a-part@/", "path@*@/"] {
            let (found, _) = checked(&m, &format!("See `{span}`.\n"), &tree());
            assert_eq!(found.len(), 1, "{span}: {found:#?}");
            assert!(found[0].contains("is refused"), "{span}: {found:#?}");
        }
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
    fn an_ignored_copy_is_not_a_generic_hit() {
        // The generic form asks every anchor, so an ignored copy under one of them would
        // otherwise satisfy the reference and hide that no component really carries it.
        let m = manifest();
        let mut present = tree();
        present.push("scratch".to_string());
        present.push("scratch/x.md".to_string());
        let (found, _) = checked_ignoring(
            &m,
            "See `path@*@scratch/x.md`.\n",
            &present,
            &["scratch/x.md", "parts/a-part/scratch/x.md"],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
    }

    #[test]
    fn an_ignored_target_is_exempt_from_existence_and_kind() {
        // The directory claim is asked with its trailing slash and the file claim without,
        // because that is the spelling git decides a `dir/` pattern on.
        let m = manifest();
        let text = "See `path@a-project@generated/out.bin` and `path@a-project@generated/`.\n";
        let (found, _) = checked_ignoring(&m, text, &tree(), &["generated/out.bin", "generated/"]);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        let (found, _) = checked(&m, text, &tree());
        assert_eq!(found.len(), 2, "{found:#?}");
        // The file spelling does not answer the directory claim, and neither answers the
        // other: a batch keyed on the bare path would exempt both from one answer.
        let (found, _) = checked_ignoring(&m, text, &tree(), &["generated/out.bin"]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("generated/"), "{found:#?}");
    }

    #[test]
    fn an_ignored_target_is_exempt_in_every_arm_the_collector_gathers() {
        // The collector and the check are two lists of the same targets, and a spelling the
        // collector misses is a target the check reads as not ignored. This binds them: the
        // batch is exactly what the collector gathered, and it must silence the three
        // anchored arms while the generic arm still refuses an ignored copy as a hit.
        //
        // It discriminates in both directions. Drop the anchored or the link arm from the
        // collector and those targets become "does not exist"; drop the generic arm and its
        // finding disappears, because the ignored copy then counts as a hit.
        let m = manifest();
        let text = "Anchored `path@a-project@gone/out.bin`, its directory \
                    `path@a-project@gone/`, under a component `path@a-part@gone/out.bin`, and \
                    generically `path@*@scratch/x.md`.\n";
        // A relative markdown link is the third arm that reaches the same assertion. It is
        // legal in a navigation home alone, so the document that carries it is one.
        let link = "See [the output](gone/out.bin) and [its directory](gone/).\n";
        let mut present = tree();
        present.push("scratch".to_string());
        present.push("scratch/x.md".to_string());
        let anchors = Anchors::of(&m);
        let docs = vec![("notes/prose.md", text), ("notes/README.md", link)];
        let model = Model::from_documents(
            docs.iter()
                .map(|(p, t)| (PathBuf::from(*p), (*t).to_string()))
                .collect(),
        );
        let queries = super::ignore_queries(&model, &anchors);
        let borrowed: Vec<&str> = queries.iter().map(String::as_str).collect();
        let (found, counts) = checked_ignoring_under(docs, &present, &borrowed, &anchors);
        assert_eq!(counts.references, 4);
        assert_eq!(counts.links, 2);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
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
