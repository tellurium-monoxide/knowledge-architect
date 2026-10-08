//! Every reference resolves against the entity table, and every relative markdown link is a
//! navigation row that resolves beside its file.
//!
//! One grammar, `` `<kind>@<anchor>@<id>` ``, and one resolver. A reference to a table kind —
//! `design`, `goal`, `tripwire` — is looked up in the table `entity::Entities` builds from the
//! walk; a `path` reference is resolved against the survey under the same anchors. A reference
//! that resolves to nothing is reported as the repair it needs, five ways: the kind position
//! holds an anchor, the anchor is unknown, the anchor does not carry that register, an item of a
//! plan is cited from outside it, or the id is not defined there. The argument is
//! `design@core@a-slug-belongs-to-a-component`.
//!
//! **Nothing that points into this project passes unregistered.** A span with no `@` that is
//! shaped like a path, and whose first segment names a file or a directory of this tree, is
//! reported as unanchored, per `design@core@every-path-names-its-anchor`. The slug reference
//! the grammar retired, `` `<word>#<word>` ``, is reported as what it was when its id is an entry
//! of this project or its word is an anchor or a kind, so a pointer the migration missed or
//! copied out of the commit history is a finding rather than silence.
//! The candidate rule and the retired-form lint are
//! `design@core@candidate-rule-and-retired-forms`. A span that names nothing here is another
//! tool's notation, and is silent.
//!
//! **The definition-site findings are not this family's.** Misplaced, malformed and duplicate
//! definitions are found while the table is built, and `check::foundation` reports them: where a
//! definition may sit is a question about a register's shape.

use crate::manifest::MANIFEST_NAME;
use std::path::{Path, PathBuf};

use crate::entity::{
    self, Anchors, Candidate, Constructed, Entities, Kind, Resolution, ESCAPE_ANCHOR, PLANS_ANCHOR,
};
use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::scan::{Observation, RetiredForm};

use super::Inputs;

/// What the check looked at: the entities the table holds, the reference occurrences it
/// judged (every kind, the escape and generic forms included), and the relative markdown links
/// it resolved.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Counts {
    pub entities: usize,
    pub references: usize,
    pub links: usize,
}

pub(crate) fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
    check_under(model, inputs, &Anchors::of(manifest, inputs.present))
}

/// The same, over a stated anchor list.
///
/// What `check` derives from the manifest, a test states directly: every component carries
/// every register, so the anchor-lacks-register arm is reachable only through an anchor with a
/// declared register subset, the shape a location takes.
pub(crate) fn check_under(
    model: &Model,
    inputs: &Inputs,
    anchors: &Anchors,
) -> (Vec<Finding>, Counts) {
    let entities = Entities::build(model, anchors);
    judge(model.documents(), &entities, anchors, inputs)
}

/// The same, over a stated document list and a stated entity table.
///
/// **The two are separated because a commit message is judged against a table it is no part
/// of.** A message is a document under the regime, per
/// `design@core@a-commit-message-is-a-document`, and the entities it names are defined by the
/// tree it commits — so the table is built from that tree's model and the documents judged
/// against it are these. `check_under` is the case where the two coincide.
pub(crate) fn judge(
    docs: &[crate::model::Document],
    entities: &Entities,
    anchors: &Anchors,
    inputs: &Inputs,
) -> (Vec<Finding>, Counts) {
    judge_part(docs, entities, anchors, inputs, Part::All)
}

/// Which findings a judgement writes.
///
/// A commit message is judged against two trees, and the two families combine differently
/// there, per `design@core@a-commit-message-is-a-document`: a reference finding survives where
/// both trees refuse it, a lint finding where either tree holds what its span names.
/// `cli::history` asks for each family apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Part {
    All,
    /// The references, the links and the wrapped spans.
    References,
    /// The unanchored path, the retired slug reference and the bare name of a skill or an
    /// agent.
    Lints,
}

/// The same, writing only the findings of one part.
pub(crate) fn judge_part(
    docs: &[crate::model::Document],
    entities: &Entities,
    anchors: &Anchors,
    inputs: &Inputs,
    part: Part,
) -> (Vec<Finding>, Counts) {
    let references = part != Part::Lints;
    let lints = part != Part::References;
    let mut out: Vec<Finding> = Vec::new();
    let mut counts = Counts {
        entities: entities.len(),
        ..Counts::default()
    };
    for doc in docs {
        let nav = is_navigation(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::Span(span) if references => match entity::candidate(span, anchors) {
                    Candidate::Reference { kind, anchor, id } => {
                        counts.references += 1;
                        if kind.is_path() {
                            path(
                                &mut out, &doc.rel, l.line, span, anchor, id, anchors, inputs,
                            );
                        } else if kind.is_planned() {
                            planned(
                                &mut out, &doc.rel, l.line, span, anchor, id, anchors, inputs,
                            );
                        } else {
                            table(
                                &mut out, &doc.rel, l.line, span, &kind, anchor, id, anchors,
                                entities,
                            );
                        }
                    }
                    Candidate::Harness { kind, owner, id } => {
                        counts.references += 1;
                        harness(&mut out, &doc.rel, l.line, span, &kind, owner, id, entities);
                    }
                    Candidate::Malformed { why, repair } => out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{span}` is malformed: {why}"),
                        repair,
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
                Observation::UnanchoredPath(span) if lints => {
                    if !names_this_tree(span, &doc.rel, anchors, inputs) {
                        continue;
                    }
                    // A line suffix or a fragment is part of the span but no part of a path
                    // reference, whose id is the file: the repair says to drop it.
                    let located = span.contains([':', '#']);
                    // In a plan document a path that does not exist yet is usually one the
                    // plan's work will create, so the repair names the form for it there.
                    // A path that exists is cited as one, so the planned form is offered only
                    // when nothing at the span's file exists, read as `names_this_tree` reads it.
                    let file = span
                        .split_once([':', '#'])
                        .map_or(span.as_str(), |(f, _)| f);
                    let exists = anchors
                        .all()
                        .iter()
                        .map(|a| a.path.join(file))
                        .chain(doc.rel.parent().map(|d| d.join(file)))
                        .any(|t| inputs.present.contains(&t));
                    let planned = if !exists && in_plans_directory(&doc.rel, anchors) {
                        ", or `planned@<anchor>@<path>` for a path the plan's work will create"
                    } else {
                        ""
                    };
                    out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{span}` is shaped like a path and names no anchor"),
                        format!(
                            "write `path@<anchor>@<path>`, `path@elsewhere@<path>` for a path \
                             outside this tree, or `path@*@<path>` for every component's own \
                             copy{planned}{}",
                            if located {
                                ". A reference names a file: drop the line number or the \
                                 fragment, and name the function or the heading in prose"
                            } else {
                                ""
                            }
                        ),
                    ))
                }
                Observation::BareName(name) if lints => {
                    // Exact against the table, so a crate's name or a name that no longer exists
                    // is silent: the lint reports a pointer written with no kind, and a word that
                    // names no skill and no agent here is no pointer. Under `harness = []` the
                    // table defines neither kind, so nothing is reported. A name both kinds hold
                    // is repaired as the skill: the order below is the only reason.
                    let Some(kind) = [entity::SKILL_KIND, entity::AGENT_KIND]
                        .into_iter()
                        .find(|k| entities.defines(&Kind::new(k), "", name))
                    else {
                        continue;
                    };
                    out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{name}` is a bare {kind} name"),
                        format!("write `{kind}@{name}`"),
                    ))
                }
                Observation::WrappedSpan(span) if references => out.push(Finding::at(
                    &doc.rel,
                    l.line,
                    format!(
                        "`{span}` crosses a line break, so no check reads it as the pointer it is"
                    ),
                    "keep the span on one line: a line break inside backticks is read as two \
                     halves, and a renderer shows it as a space",
                )),
                Observation::Retired(RetiredForm::SlugRef(span))
                    if lints && names_this_project(span, anchors, entities) =>
                {
                    out.push(Finding::at(
                        &doc.rel,
                        l.line,
                        format!("`{span}` is the retired slug reference form"),
                        "write `design@<component>@<slug>`; the form with a `#` is no longer read \
                     as a reference",
                    ))
                }
                // Markdown documents only: in Rust prose a markdown link is rustdoc's
                // mechanism, resolved by rustdoc against the crate namespace, and this
                // check reading those as index rows would report every intra-doc link.
                Observation::Link(target) if references && doc.is_markdown() => {
                    link(&mut out, &mut counts, doc, l.line, target, nav, inputs);
                }
                _ => {}
            }
        }
    }
    (out, counts)
}

/// Whether an unanchored path-shaped span names a file or a directory of this tree.
///
/// Only its first segment is asked: a pointer whose first segment is here and whose rest is
/// not is a dangling pointer, which the anchored form would report, so it is a finding.
///
/// **The places it is read from are the root, every anchor that is a directory, and the
/// document's own directory**: a path written in a component's README is often relative to that
/// component, and one written beside a file relative to it. A spec is an anchor and a file, so
/// it is no place. A `.` segment is passed over and a `..` climbs from the place; one that climbs
/// past the root names nothing from there. A leading slash is read from the root alone. The
/// suffix, a line number or a fragment, is no part of the path.
///
/// **The listing alone answers, and the ignore rules are not asked.** A pointer into an ignored
/// directory is silent: asking git would cost one spelling per span and place, would make every
/// root name a pointer under a whitelist `.gitignore`, and fails outright on a spelling through
/// a symlink.
fn names_this_tree(span: &str, rel: &Path, anchors: &Anchors, inputs: &Inputs) -> bool {
    let path = span.split([':', '#']).next().unwrap_or(span);
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut bases: Vec<&Path> = vec![Path::new("")];
    if !path.starts_with('/') {
        bases.extend(
            anchors
                .all()
                .iter()
                .map(|a| a.path.as_path())
                .filter(|p| inputs.directories.contains(*p)),
        );
        bases.push(rel.parent().unwrap_or(Path::new("")));
    }
    bases.into_iter().any(|base| {
        let mut at = base.to_path_buf();
        for segment in &segments {
            match *segment {
                "." => {}
                ".." => {
                    if !at.pop() {
                        return false;
                    }
                }
                name => {
                    at.push(name);
                    return inputs.present.contains(&at);
                }
            }
        }
        false
    })
}

/// Whether a retired slug reference names something of this project: its id an entry some
/// register defines, or its word an anchor or a kind.
///
/// The id is asked whatever the word, because a form copied out of the history may carry the
/// name an anchor had before it was renamed. The form was this grammar's alone, so a span that
/// names nothing here is another tool's notation: an issue number, a preprocessor directive, a
/// crate's item.
fn names_this_project(span: &str, anchors: &Anchors, entities: &Entities) -> bool {
    match span.split_once('#') {
        Some((word, id)) => {
            entities.defines_id(id)
                || (!word.is_empty()
                    && (anchors.kind(word).is_some() || anchors.is_anchor_word(word)))
        }
        None => false,
    }
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
    match entities.resolve_from(anchors, kind, anchor, id, Some(rel)) {
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
        Resolution::OutsidePlan { form } => out.push(Finding::at(
            rel,
            line,
            format!("`{span}` cites an item of the plan `{anchor}` from outside it"),
            format!(
                "cite the plan whole, as `{form}`; an item is cited only from inside its own plan, \
                 so retiring the plan asks no other document to be redesigned"
            ),
        )),
        Resolution::Undefined => out.push(Finding::at(
            rel,
            line,
            format!("`{span}` is referenced and `{anchor}` defines no {kind} `{id}`"),
            match anchors
                .registers()
                .by_name(kind.name())
                .and_then(|r| r.section.as_deref())
            {
                // An item is defined under its section of the plan, which has no home of its own.
                Some(section) => format!(
                    "define it under the {section} section of the plan `{anchor}`, or repair the \
                     reference"
                ),
                None => format!(
                    "define it in the {} home of `{anchor}`, or repair the reference",
                    anchors
                        .registers()
                        .by_name(kind.name())
                        .map(|r| r.dir.as_str())
                        .unwrap_or("register")
                ),
            },
        )),
    }
}

/// A reference of a harness kind, against the entity table: the skill or the agent it names, and
/// the section, or the section of the primer or of the root instructions.
///
/// A section of a skill or an agent that does not exist is reported as the missing skill or
/// agent, not as the missing section: its repair is the name, whatever the slug says.
#[allow(clippy::too_many_arguments)]
fn harness(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    span: &str,
    kind: &Kind,
    owner: &str,
    id: &str,
    entities: &Entities,
) {
    if entities.defines(kind, owner, id) {
        return;
    }
    let (what, action) = if !owner.is_empty() && !entities.defines(kind, "", owner) {
        (
            format!("`{span}` is referenced and no {kind} is named `{owner}`"),
            format!("name an existing {kind}, or repair the reference"),
        )
    } else if !owner.is_empty() {
        (
            format!("`{span}` is referenced and the {kind} `{owner}` defines no section `{id}`"),
            format!(
                "end a level-2 heading of the {kind} `{owner}` with that slug, or repair the \
                 reference"
            ),
        )
    } else if kind.is_named() {
        (
            format!("`{span}` is referenced and no {kind} is named `{id}`"),
            format!("name an existing {kind}, or repair the reference"),
        )
    } else {
        let document = if kind.name() == entity::PRIMER_KIND {
            "the primer"
        } else {
            "the root CLAUDE.md"
        };
        (
            format!("`{span}` is referenced and {document} defines no section `{id}`"),
            format!("end a level-2 heading of {document} with that slug, or repair the reference"),
        )
    };
    out.push(Finding::at(rel, line, what, action));
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
                    "the escape anchor is for a path this tree does not hold: anchor the \
                     reference at the anchor that holds it, or, for a file of another project, \
                     begin the path with that project's name, as \
                     `path@elsewhere@<project>/<path>`",
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
            // Components only, per `design@core@reserved-anchors`: the generic form claims
            // every component's own copy, so a copy a declared location alone holds makes the
            // claim false for every component. The anchors the tool constructs are no
            // components either: they hold plan documents, which are cited by their kind, per
            // `design@core@plan-document-kinds`.
            let carried: Vec<PathBuf> = anchors
                .all()
                .iter()
                .filter(|a| a.is_component)
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
            if let Some(target) = anchored_target(out, rel, line, span, name, path, anchors, false)
            {
                assert_target(out, rel, line, span, &target, claims_dir, inputs);
            }
        }
    }
}

/// The target of a `path` or `planned` reference at a named anchor, or `None` with the finding
/// that refuses it: an unknown anchor, a plan anchor, a refused shape, a plan document cited by
/// its path, or a target inside a deeper anchor. Both kinds share these rules, so a planned
/// path converts to a `path` reference by its kind word alone.
#[allow(clippy::too_many_arguments)]
fn anchored_target(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    span: &str,
    name: &str,
    path: &str,
    anchors: &Anchors,
    planned: bool,
) -> Option<PathBuf> {
    let trimmed = path.trim_end_matches('/');
    let Some(a) = anchors.by_name(name) else {
        // The two reserved words serve `path` alone, so a planned path's repair names the
        // declared anchors only.
        // A plan anchor carries no path kind, so it is no candidate for either kind.
        let names = anchors
            .all()
            .iter()
            .filter(|a| !a.is_plan())
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let action = if planned {
            format!("anchor at the one of {names} that will hold the target")
        } else {
            format!(
                "anchor at one of {names}, at `{ESCAPE_ANCHOR}` for a path outside this \
                 tree, or at `*` for every component's own copy"
            )
        };
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` names `{name}`, which is no anchor of this project"),
            action,
        ));
        return None;
    };
    if a.is_milestone() {
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` names the milestone `{name}`, which carries no path kind"),
            format!(
                "cite a slice spec as `spec@{name}@<slice>`, the milestone document as \
                     `milestone@{PLANS_ANCHOR}@{name}`, and from inside it its items as \
                     `<kind>@{name}@<id>`; a plan document has one name, so `show` finds \
                     every citation of it"
            ),
        ));
        return None;
    }
    if a.is_plan() {
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` names the spec `{name}`, which carries no path kind"),
            format!(
                "cite the spec as `spec@{PLANS_ANCHOR}@{name}`, and from inside it its \
                     items as `<kind>@{name}@<id>`; a plan document has one name, so `show` \
                     finds every citation of it"
            ),
        ));
        return None;
    }
    // Judged on the path as written: a lone `/` trims to nothing and would resolve
    // to the anchor's own directory, which has no spelling under its own name.
    if let Some(why) = refused(path) {
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` is refused: {why}"),
            "anchor at the anchor that holds the target, with a plain relative path",
        ));
        return None;
    }
    let target = a.path.join(trimmed);
    // Per `design@core@plan-document-kinds`, before the deepest-anchor rule: a plan document cited by its path is
    // refused from every anchor, and the repair names the one form that resolves.
    if let Some(form) = plan_document(anchors, &target) {
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` cites a plan document by its path"),
            format!(
                "cite it as `{form}`; a plan document has one name, so `show` finds \
                     every citation of it"
            ),
        ));
        return None;
    }
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
            format!(
                "anchor at the deepest anchor holding the target, so a move edits \
                     {MANIFEST_NAME} and no document"
            ),
        ));
        return None;
    }
    Some(target)
}

/// A `planned` reference: a path a plan's work will create, per `design@core@planned-path-form`.
///
/// It is legal in the plans directory alone, takes a named anchor under the rules of a `path`
/// reference, and asserts that its target does not exist. Once the target exists the finding
/// names the `path` form and nothing else: the session that created the file converts the
/// reference, and whether the plan still holds is that session's report to make, not the
/// check's.
#[allow(clippy::too_many_arguments)]
fn planned(
    out: &mut Vec<Finding>,
    rel: &Path,
    line: u32,
    span: &str,
    anchor: &str,
    path: &str,
    anchors: &Anchors,
    inputs: &Inputs,
) {
    if !in_plans_directory(rel, anchors) {
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` is a planned path outside the plans directory"),
            "a planned path is cited from a plan document only; name the plan that creates it",
        ));
        return;
    }
    let claims_dir = path.ends_with('/');
    let target = match planned_target(rel, line, span, anchor, path, anchors) {
        Ok(target) => target,
        Err(refusal) => {
            out.push(refusal);
            return;
        }
    };
    // A line or a fragment names no file: `x.rs:12` of an existing `x.rs` is a pointer into it,
    // which the planned form cannot be.
    if let Some((file, _)) = path.split_once([':', '#']) {
        if let Some(a) = anchors.by_name(anchor) {
            if inputs.present.contains(&a.path.join(file)) {
                out.push(Finding::at(
                    rel,
                    line,
                    format!("`{span}` points into `{file}`, which exists"),
                    format!(
                        "write `path@{anchor}@{file}`, and name the function or the heading in \
                         prose"
                    ),
                ));
                return;
            }
        }
    }
    if inputs
        .ignored
        .contains(&crate::git::ignore_query(&target, claims_dir))
    {
        return;
    }
    if inputs.present.contains(&target) {
        // The repair carries the kind the target has, so applying it raises no slash finding.
        let slash = if inputs.directories.contains(&target) {
            "/"
        } else {
            ""
        };
        let trimmed = path.trim_end_matches('/');
        out.push(Finding::at(
            rel,
            line,
            format!("`{span}` now exists, at `{}`", target.display()),
            format!(
                "write `path@{anchor}@{trimmed}{slash}`; the planned form names a path that does \
                 not exist yet"
            ),
        ));
    }
}

/// The target of a planned reference, or the finding that refuses it: the two reserved words,
/// and every rule of a named anchor that `anchored_target` applies. `show` asks the same, so it
/// resolves exactly the planned references the check accepts.
pub(crate) fn planned_target(
    rel: &Path,
    line: u32,
    span: &str,
    anchor: &str,
    path: &str,
    anchors: &Anchors,
) -> Result<PathBuf, Finding> {
    if anchor == ESCAPE_ANCHOR || anchor == entity::EVERY_ANCHOR {
        return Err(Finding::at(
            rel,
            line,
            format!("`{span}` gives a planned path the anchor `{anchor}`"),
            format!(
                "anchor at the anchor that will hold the target; `*` and `{ESCAPE_ANCHOR}` serve \
                 the path kind alone"
            ),
        ));
    }
    let mut out = Vec::new();
    match anchored_target(&mut out, rel, line, span, anchor, path, anchors, true) {
        Some(target) => Ok(target),
        None => Err(out.remove(0)),
    }
}

/// Whether a document lies in the plans directory, where a planned path is legal.
fn in_plans_directory(rel: &Path, anchors: &Anchors) -> bool {
    anchors
        .all()
        .iter()
        .any(|a| a.constructed == Some(Constructed::Plans) && rel.starts_with(&a.path))
}

/// The kind form of a plan document at `target`, or `None` when no plan document is there.
///
/// A plan document is a spec file of `specs/`, a milestone directory, or a file inside a
/// milestone directory, per `design@core@plan-document-kinds`. The README and the index of the plans
/// directory and of its two homes are not plan documents, and are cited by path. Judged by
/// where the target sits, not by whether it exists: a path to a deleted slice spec gets the same
/// repair as one to a present one.
fn plan_document(anchors: &Anchors, target: &Path) -> Option<String> {
    use crate::manifest::{MILESTONES_HOME, SPECS_HOME};
    let plans = anchors
        .all()
        .iter()
        .find(|a| a.constructed == Some(Constructed::Plans))?;
    if let Ok(inside) = target.strip_prefix(plans.path.join(MILESTONES_HOME)) {
        let parts: Vec<String> = inside
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();
        // The first segment names a milestone only in the id grammar: a directory of another
        // name is no milestone, and its finding is phase 2's.
        let m = parts.first().filter(|m| entity::is_entity_id(m))?;
        let slice = match parts.as_slice() {
            [_, file] => file
                .strip_suffix(".md")
                .filter(|s| *s != "README" && *s != "index"),
            _ => None,
        };
        return Some(match slice {
            Some(slice) => format!("spec@{m}@{slice}"),
            None => format!("milestone@{PLANS_ANCHOR}@{m}"),
        });
    }
    entity::entry_id(target, &plans.path.join(SPECS_HOME))
        .map(|id| format!("spec@{PLANS_ANCHOR}@{id}"))
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
pub(crate) fn ignore_queries(model: &Model, anchors: &Anchors) -> Vec<String> {
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
                    if !kind.takes_a_path() {
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
    // own source, per `design@core@checker-source-literals-are-data`.

    /// A manifest declaring one component beside the root, against a root nothing reads.
    fn manifest() -> Manifest {
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        Manifest::parse(std::path::Path::new("/nowhere"), text).expect("a declaration")
    }

    /// The findings and counts over documents, against a listing.
    fn checked_docs(
        manifest: &Manifest,
        docs: Vec<(&str, &str)>,
        present: &[String],
    ) -> (Vec<String>, Counts) {
        checked_under(docs, present, &Anchors::declared(manifest))
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
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let directories = crate::check::testing::implied_directories(&present);
        let outside = Vec::new();
        let inputs = Inputs {
            committed: &committed,
            configs: &HashMap::new(),
            present: &present,
            directories: &directories,
            outside: &outside,
            ignored: &ignored.iter().map(|s| (*s).to_string()).collect(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
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
            &Anchors::declared(manifest),
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

    /// The claim: a spec of `specs/`, a milestone and a slice spec each resolve by their kind,
    /// each in its own anchor, and an absent id in each is the ordinary dangling finding.
    /// Mutation checked: `file_definitions` filing a milestone's slice specs under `plans`.
    #[test]
    fn the_plan_kinds_resolve_each_in_its_own_anchor() {
        let m = manifest();
        let docs = vec![
            ("docs/plans/specs/a-spec.md", "# A spec\n"),
            ("docs/plans/milestones/m/README.md", "# A milestone\n"),
            ("docs/plans/milestones/m/a-slice.md", "# A slice\n"),
            (
                "notes/prose.md",
                "`spec@plans@a-spec`, `milestone@plans@m` and `spec@m@a-slice` resolve;\n\
                 `spec@plans@none`, `milestone@plans@none` and `spec@m@none` do not.\n",
            ),
        ];
        let present: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let anchors = Anchors::of(&m, &present);
        let present: Vec<String> = docs.iter().map(|(p, _)| p.to_string()).collect();
        let (found, counts) = checked_under(docs, &present, &anchors);
        assert_eq!(counts.references, 6, "{found:#?}");
        assert_eq!(found.len(), 3, "{found:#?}");
        assert!(found
            .iter()
            .any(|f| f.contains("`plans` defines no spec `none`")));
        assert!(found
            .iter()
            .any(|f| f.contains("`plans` defines no milestone `none`")));
        assert!(found
            .iter()
            .any(|f| f.contains("`m` defines no spec `none`")));
    }

    /// The claim: the repair a refused plan citation names is the form that resolves for each position: a slice for
    /// a slice spec, the milestone for its README, its index or anything deeper, a spec for a
    /// spec, and nothing for a navigation file. Mutations checked: the README and index filter,
    /// and the one-segment test, each removed from `plan_document`.
    #[test]
    fn the_p1_repair_names_the_form_for_each_position() {
        let m = manifest();
        let anchors = Anchors::of(&m, &[] as &[PathBuf]);
        for (target, form) in [
            ("docs/plans/milestones/m/a-slice.md", Some("spec@m@a-slice")),
            (
                "docs/plans/milestones/m/README.md",
                Some("milestone@plans@m"),
            ),
            (
                "docs/plans/milestones/m/index.md",
                Some("milestone@plans@m"),
            ),
            (
                "docs/plans/milestones/m/sub/x.md",
                Some("milestone@plans@m"),
            ),
            (
                "docs/plans/milestones/m/a.md/x.md",
                Some("milestone@plans@m"),
            ),
            ("docs/plans/milestones/m", Some("milestone@plans@m")),
            ("docs/plans/specs/a-spec.md", Some("spec@plans@a-spec")),
            ("docs/plans/specs/README.md", None),
            ("docs/plans/milestones/README.md", None),
            ("docs/plans/README.md", None),
        ] {
            assert_eq!(
                plan_document(&anchors, Path::new(target)).as_deref(),
                form,
                "{target}"
            );
        }
    }

    /// The claim: a milestone's name in the kind position is no retired form, so an email
    /// address or a git remote stays silent when a milestone of that name exists. Mutation
    /// checked: `Anchors::is_anchor_word` counting milestone anchors.
    #[test]
    fn a_milestone_name_in_the_kind_position_is_silent() {
        let m = manifest();
        let tree: Vec<PathBuf> = ["docs/plans/milestones/git/README.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let anchors = Anchors::of(&m, &tree);
        assert!(anchors.by_name("git").is_some());
        let text = "The remote is `git@github.com:org/repo.git`.\n";
        let (found, counts) = checked_under(vec![("notes/prose.md", text)], &[], &anchors);
        assert!(found.is_empty(), "{found:#?}");
        assert_eq!(counts.references, 0);
    }

    /// The claims of the reference arms that reach a plan document another way: the generic
    /// form names no copy the plans anchors hold, so a spec is no copy of any component; and a
    /// milestone whose README the walk does not read defines nothing, so a reference to it
    /// dangles. Mutations checked: the filter on the tool's anchors removed from the `*` arm of
    /// `path`; the walked-README test removed from `Entities::directory_definitions`.
    #[test]
    fn a_plan_document_reached_through_the_generic_form_or_out_of_the_walk_resolves_nothing() {
        let m = manifest();
        let tree: Vec<PathBuf> = [
            "docs/plans/specs/a-spec.md",
            "docs/plans/milestones/m/README.md",
            "docs/plans/milestones/m/a-slice.md",
        ]
        .iter()
        .map(PathBuf::from)
        .collect();
        let anchors = Anchors::of(&m, &tree);
        let present: Vec<String> = tree.iter().map(|p| p.display().to_string()).collect();
        // The milestone's README is in the tree and not in the walk.
        let docs = vec![
            ("docs/plans/specs/a-spec.md", "# A spec\n"),
            ("docs/plans/milestones/m/a-slice.md", "# A slice\n"),
            (
                "notes/prose.md",
                "`path@*@specs/a-spec.md`, `path@*@a-slice.md` and `milestone@plans@m`.\n",
            ),
        ];
        let (found, _) = checked_under(docs, &present, &anchors);
        assert_eq!(found.len(), 3, "{found:#?}");
        assert_eq!(
            found
                .iter()
                .filter(|f| f.contains("resolves in no component"))
                .count(),
            2,
            "{found:#?}"
        );
        assert!(found
            .iter()
            .any(|f| f.contains("`plans` defines no milestone `m`")));
    }

    /// The claim: an item of a plan resolves from inside its plan and is refused from outside
    /// it, with the whole-document repair, and the scope is judged before the id, so an
    /// undefined item cited from outside gets the same repair; and a spec anchor carries no
    /// `path` kind. Mutations checked: the scope judged after the id lookup; a spec anchor
    /// carrying `path`.
    #[test]
    fn an_item_resolves_from_inside_its_plan_and_is_refused_from_outside() {
        let m = manifest();
        let docs = vec![
            (
                "docs/plans/specs/s.md",
                "# A spec\n\n## Threads\n\n### A thread `##one`\n\nIt cites `thread@s@one` and \
                 `thread@s@nope`.\n",
            ),
            (
                "notes/prose.md",
                "`thread@s@one`, `thread@s@none`, `spec@plans@s` and `path@s@x.md`.\n",
            ),
        ];
        let tree: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let anchors = Anchors::of(&m, &tree);
        let present: Vec<String> = docs.iter().map(|(p, _)| p.to_string()).collect();
        let (found, _) = checked_under(docs, &present, &anchors);
        let outside: Vec<&String> = found
            .iter()
            .filter(|f| f.contains("cites an item of the plan `s` from outside it"))
            .collect();
        assert_eq!(outside.len(), 2, "{found:#?}");
        assert!(found
            .iter()
            .any(|f| f.contains("`path@s@x.md` names the spec `s`, which carries no path kind")));
        // An undefined item cited from inside its plan is the ordinary dangling finding.
        assert!(found
            .iter()
            .any(|f| f.contains("`s` defines no thread `nope`")));
        assert_eq!(found.len(), 4, "{found:#?}");
    }

    /// The claim: an item a slice spec defines, cited from outside its milestone, gets the slice
    /// spec as the whole-document form; one the README defines gets the milestone. Mutation
    /// checked: the milestone form for every item of a milestone.
    #[test]
    fn an_item_cited_from_outside_names_the_document_that_defines_it() {
        let m = manifest();
        let docs = [
            (
                "docs/plans/milestones/m/README.md",
                "# A milestone\n\n## Threads\n\n### In the README `##in-readme`\n",
            ),
            (
                "docs/plans/milestones/m/a-slice.md",
                "# A slice\n\n## Threads\n\n### In the slice `##in-slice`\n",
            ),
            (
                "notes/prose.md",
                "`thread@m@in-readme` and `thread@m@in-slice`.\n",
            ),
        ];
        let tree: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let anchors = Anchors::of(&m, &tree);
        let model = Model::from_documents(
            docs.iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        let entities = Entities::build(&model, &anchors);
        let notes = Path::new("notes/prose.md");
        let form = |id: &str| match entities.resolve_from(
            &anchors,
            &Kind::new("thread"),
            "m",
            id,
            Some(notes),
        ) {
            Resolution::OutsidePlan { form } => form,
            other => panic!("{other:?}"),
        };
        assert_eq!(form("in-readme"), "milestone@plans@m");
        assert_eq!(form("in-slice"), "spec@m@a-slice");
        // And the Undefined repair of an item names its section, not a home.
        let (found, _) = checked_under(
            vec![(
                "docs/plans/milestones/m/README.md",
                "# A milestone\n\nIt cites `thread@m@nope`.\n",
            )],
            &[],
            &anchors,
        );
        assert!(
            found.iter().any(|f| f.contains("defines no thread `nope`")),
            "{found:#?}"
        );
    }

    /// The claim: a plan document is judged by where it sits, not by whether it exists, so a
    /// citation of a deleted milestone or of a deleted slice gets the kind form too. Mutation
    /// checked: `plan_document` asking the owning anchor, which a deleted milestone no longer is.
    #[test]
    fn a_deleted_plan_document_cited_by_its_path_gets_the_kind_form() {
        let m = manifest();
        let present: Vec<PathBuf> = ["docs/plans", "docs/plans/milestones"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let anchors = Anchors::of(&m, &present);
        let text = "`path@plans@milestones/gone/` and `path@plans@milestones/gone/a-slice.md` and \
                    `path@plans@specs/gone.md` are gone.\n";
        let present: Vec<String> = present.iter().map(|p| p.display().to_string()).collect();
        let (found, _) = checked_under(vec![("notes/prose.md", text)], &present, &anchors);
        assert_eq!(found.len(), 3, "{found:#?}");
        assert!(found
            .iter()
            .all(|f| f.contains("cites a plan document by its path")));
    }

    #[test]
    fn an_anchor_carrying_no_such_register_is_reported_with_the_anchors_that_do() {
        // The third way of the four, over an anchor list no manifest produces today: an
        // anchor carrying tripwires alone. Mutation checked: with the arm's push replaced
        // by a drop, the assertion on one finding fails.
        let m = manifest();
        let base = Anchors::declared(&m);
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
                f.contains(&Anchors::declared(&manifest()).kinds_listed()),
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
    fn the_retired_slug_form_is_a_finding_naming_what_it_was() {
        // Each shape names something of this project: an anchor, a kind, or an entry the
        // table defines.
        let (found, counts) = checked_docs(
            &manifest(),
            vec![
                ("docs/design.md", head()),
                (
                    "notes/prose.md",
                    "`a-project#a-decision`, `design#a-decision`, `#a-decision` and R15\n",
                ),
            ],
            &[],
        );
        assert_eq!(found.len(), 3, "a bare R and digits is not one: {found:#?}");
        assert_eq!(
            found
                .iter()
                .filter(|f| f.contains("retired slug reference form"))
                .count(),
            3,
            "{found:#?}"
        );
        assert_eq!(counts.references, 0, "a retired form is not a reference");
    }

    #[test]
    fn a_retired_form_whose_id_is_an_entry_is_a_finding_whatever_its_word() {
        // A form copied out of the history may carry an anchor's former name; its entry is
        // still here, so the copy is still a pointer the migration missed.
        let (found, _) = checked_docs(
            &manifest(),
            vec![
                ("docs/design.md", head()),
                ("notes/prose.md", "`knowledge#a-decision`\n"),
            ],
            &[],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("retired slug reference form"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_retired_shape_that_names_nothing_of_this_project_is_silent() {
        // A head that is no anchor and no kind, and a bare id no register defines: an issue
        // number, a preprocessor directive, another tool's notation. None was ever this
        // grammar's, so none is a reference a migration missed.
        let (found, _) = checked_docs(
            &manifest(),
            vec![
                ("docs/design.md", head()),
                (
                    "notes/prose.md",
                    "`serde#derive`, `rust#123`, `#include`, `#123` and `#a-decisions`\n",
                ),
            ],
            &[],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
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
    fn the_definition_findings_belong_to_the_entity_table_and_not_to_this_check() {
        // Where a definition may sit is the entity table's finding, phase 3, which stops the
        // run before this check runs. This check still resolves against the table the
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
        // The writer may have meant another project's file of the same spelling; the repair
        // names the checked form for it.
        assert!(
            found[0].contains("as `path@elsewhere@<project>/<path>`"),
            "{found:#?}"
        );
    }

    #[test]
    fn an_empty_id_names_the_placeholder_and_the_ancestor_spelling() {
        // Two needs end in an empty id under a path kind: naming a form in prose, and naming
        // an anchor's own directory. Each has a checked form, and the repair spells both.
        let repair = |text: &str| {
            let (found, _) = checked(&manifest(), text, &[]);
            assert_eq!(found.len(), 1, "{text}: {found:#?}");
            found[0].clone()
        };
        let path = repair("the form `path@a-project@` alone\n");
        assert!(path.contains("the id segment is empty"), "{path}");
        assert!(path.contains("as `path@*@<path>`"), "{path}");
        assert!(path.contains("as `path@<parent-anchor>@<dir>/`"), "{path}");
        // A kind that takes no path has no directory to name: its repair names the entry form.
        let entry = repair("`design@a-part@`\n");
        assert!(entry.contains("as `<kind>@<anchor>@<id>`"), "{entry}");
        assert!(!entry.contains("parent-anchor"), "{entry}");
        // Every other malformed reason keeps the generic repair, and none takes the empty id's.
        for span in [
            "`path@a-project`",
            "`@notes/p.md`",
            "`design@@x`",
            "`design@a-project@x@y`",
        ] {
            let other = repair(&format!("{span}\n"));
            assert!(other.contains("and for a path"), "{other}");
            assert!(!other.contains("placeholder"), "{other}");
        }
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
    fn a_path_only_a_location_carries_resolves_in_no_component() {
        // The generic form claims every component's own copy, per
        // `design@core@reserved-anchors`, so a copy a declared location alone holds does not
        // satisfy it. Mutation checked: with the filter back to `constructed.is_none()`, the
        // location's copy satisfies the reference and nothing is reported.
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = [\"parts/a-part\"]\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"issue\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n";
        let m = Manifest::parse(std::path::Path::new("/nowhere"), text).expect("a declaration");
        let mut present = tree();
        present.extend(["notes/held.md".to_string(), "notes".to_string()]);
        let (found, _) = checked(&m, "See `path@*@held.md`.\n", &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("resolves in no component"), "{found:#?}");
        // The same path in a component passes, so the refusal is about the location alone.
        present.push("parts/a-part/held.md".to_string());
        let (found, _) = checked(&m, "See `path@*@held.md`.\n", &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
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
        let anchors = Anchors::declared(&m);
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

    #[test]
    fn an_unanchored_path_shape_s_repair_offers_no_unchecked_form() {
        // The claim, per `design@agent-skills@plain-text-is-no-repair`: the repair names
        // checked forms only, so no reading of it clears the finding by unbackticking the path.
        // The whole repair line is pinned, so an unchecked form offered in any words fails,
        // on the plain span and on both located ones.
        let forms = "write `path@<anchor>@<path>`, `path@elsewhere@<path>` for a path outside \
                     this tree, or `path@*@<path>` for every component's own copy";
        let located = ". A reference names a file: drop the line number or the fragment, and \
                       name the function or the heading in prose";
        let m = manifest();
        for (span, suffix) in [
            ("notes/real/a.md", ""),
            ("notes/real/a.md:12", located),
            ("notes/real/a.md#a-head", located),
        ] {
            let (found, _) = checked(&m, &format!("See `{span}`.\n"), &tree());
            assert_eq!(found.len(), 1, "{span}: {found:#?}");
            let action = found[0].split_once("\n    → ").map(|(_, a)| a);
            assert_eq!(action, Some(format!("{forms}{suffix}").as_str()), "{span}");
        }
    }

    // --- the planned kind, per `design@core@planned-path-form` ----------------------------

    /// A spec file of the plans directory, where a planned path is legal.
    const IN_PLANS: &str = "docs/plans/specs/a-spec.md";

    #[test]
    fn a_planned_path_whose_target_is_absent_is_silent() {
        let m = manifest();
        let (found, counts) = checked_in(
            &m,
            IN_PLANS,
            "It writes `planned@a-part@notes/new.md` and `planned@a-part@notes/fresh/`.\n",
            &tree(),
        );
        assert_eq!(counts.references, 2);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn a_planned_path_whose_target_exists_names_the_path_form_and_nothing_else() {
        // The premortem's cause: a file created by unrelated work. The repair asks for the
        // conversion alone; whether the plan still holds is the creating session's to say.
        let m = manifest();
        let (found, _) = checked_in(
            &m,
            IN_PLANS,
            "It writes `planned@a-part@notes/real/a.md`.\n",
            &tree(),
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("now exists"), "{found:#?}");
        let action = found[0].split_once("\n    → ").map(|(_, a)| a);
        assert_eq!(
            action,
            Some(
                "write `path@a-part@notes/real/a.md`; the planned form names a path that does \
                 not exist yet"
            )
        );
    }

    #[test]
    fn a_planned_path_outside_the_plans_directory_is_refused() {
        let m = manifest();
        let (found, _) = checked(&m, "It writes `planned@a-part@notes/new.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        let action = found[0].split_once("\n    → ").map(|(_, a)| a);
        assert_eq!(
            action,
            Some(
                "a planned path is cited from a plan document only; name the plan that creates it"
            )
        );
    }

    #[test]
    fn a_planned_path_takes_a_named_anchor_under_the_rules_of_a_path() {
        let m = manifest();
        let text = "Escape `planned@elsewhere@x.md`, generic `planned@*@x.md`, unknown \
                    `planned@nowhere@x.md`, upward `planned@a-part@../x.md`, and reaching inside \
                    `planned@a-project@parts/a-part/x.md`.\n";
        let (found, _) = checked_in(&m, IN_PLANS, text, &tree());
        assert_eq!(found.len(), 5, "{found:#?}");
        for (i, expected) in [
            "the anchor `elsewhere`",
            "the anchor `*`",
            "which is no anchor of this project",
            "is refused: an upward segment",
            "reaches inside the anchor `a-part`",
        ]
        .iter()
        .enumerate()
        {
            assert!(found[i].contains(expected), "{expected}: {found:#?}");
        }
        // The repairs of the three anchor findings name only what a planned path accepts.
        let action = |f: &String| f.split_once("\n    → ").map(|(_, a)| a.to_string());
        let reserved = "anchor at the anchor that will hold the target; `*` and `elsewhere` serve \
                        the path kind alone";
        assert_eq!(action(&found[0]).as_deref(), Some(reserved));
        assert_eq!(action(&found[1]).as_deref(), Some(reserved));
        assert_eq!(
            action(&found[2]).as_deref(),
            Some("anchor at the one of a-project, a-part, plans that will hold the target")
        );
    }

    #[test]
    fn an_existing_planned_target_s_repair_carries_the_target_s_kind() {
        // Applying the repair must raise no slash finding: a directory gets the slash whatever
        // the planned reference claimed, and a file loses it.
        let m = manifest();
        let text =
            "Directory `planned@a-part@notes/real`, file `planned@a-part@notes/real/a.md/`.\n";
        let (found, _) = checked_in(&m, IN_PLANS, text, &tree());
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found[0].contains("write `path@a-part@notes/real/`;"),
            "{found:#?}"
        );
        assert!(
            found[1].contains("write `path@a-part@notes/real/a.md`;"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_planned_id_may_hold_an_at_sign_and_the_kind_is_listed() {
        let m = manifest();
        let (found, counts) = checked_in(
            &m,
            IN_PLANS,
            "It writes `planned@a-part@notes/a@b.md`.\n",
            &tree(),
        );
        assert_eq!(counts.references, 1);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        // Under the default harness the four harness kinds follow `planned`.
        assert!(Anchors::declared(&m)
            .kinds_listed()
            .contains("path, planned"));
    }

    #[test]
    fn a_planned_path_s_unknown_anchor_repair_lists_no_plan_anchor() {
        let m = manifest();
        let spec = PathBuf::from(IN_PLANS);
        let anchors = Anchors::of(&m, [&spec]);
        assert!(anchors.by_name("a-spec").is_some_and(|a| a.is_plan()));
        let (found, _) = checked_ignoring_under(
            vec![(IN_PLANS, "It writes `planned@nowhere@x.md`.\n")],
            &tree(),
            &[],
            &anchors,
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        let action = found[0].split_once("\n    → ").map(|(_, a)| a);
        assert_eq!(
            action,
            Some("anchor at the one of a-project, a-part, plans that will hold the target")
        );
    }

    #[test]
    fn a_path_s_unknown_anchor_repair_lists_no_plan_anchor() {
        let m = manifest();
        let spec = PathBuf::from(IN_PLANS);
        let anchors = Anchors::of(&m, [&spec]);
        let (found, _) = checked_ignoring_under(
            vec![("notes/prose.md", "See `path@nowhere@x.md`.\n")],
            &tree(),
            &[],
            &anchors,
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        let action = found[0].split_once("\n    → ").map(|(_, a)| a);
        assert_eq!(
            action,
            Some(
                "anchor at one of a-project, a-part, plans, at `elsewhere` for a path outside \
                 this tree, or at `*` for every component's own copy"
            )
        );
    }

    #[test]
    fn a_planned_path_into_an_existing_file_is_refused() {
        let m = manifest();
        let text = "Line `planned@a-part@notes/real/a.md:12`, fragment \
                    `planned@a-part@notes/real/a.md#a-head`.\n";
        let (found, _) = checked_in(&m, IN_PLANS, text, &tree());
        assert_eq!(found.len(), 2, "{found:#?}");
        for f in &found {
            assert!(
                f.contains("points into `notes/real/a.md`, which exists"),
                "{f}"
            );
        }
    }

    #[test]
    fn an_existing_path_in_a_plan_document_is_not_offered_the_planned_form() {
        let m = manifest();
        let (found, _) = checked_in(&m, IN_PLANS, "It edits `notes/real/a.md:3`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(!found[0].contains("planned@"), "{found:#?}");
    }

    #[test]
    fn an_unanchored_path_in_a_plan_document_is_offered_the_planned_form() {
        let m = manifest();
        let (found, _) = checked_in(&m, IN_PLANS, "It writes `notes/real/b.md`.\n", &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        let action = found[0].split_once("\n    → ").map(|(_, a)| a);
        assert_eq!(
            action,
            Some(
                "write `path@<anchor>@<path>`, `path@elsewhere@<path>` for a path outside this \
                 tree, or `path@*@<path>` for every component's own copy, or \
                 `planned@<anchor>@<path>` for a path the plan's work will create"
            )
        );
    }

    #[test]
    fn an_ignored_planned_target_is_exempt_through_the_collector() {
        // The batch is exactly what the collector gathered: drop the planned kind from it and
        // the ignored target that exists reads as "now exists".
        let m = manifest();
        let text = "It writes `planned@a-part@notes/real/a.md`.\n";
        let anchors = Anchors::declared(&m);
        let model = Model::from_documents(vec![(PathBuf::from(IN_PLANS), text.to_string())]);
        let queries = super::ignore_queries(&model, &anchors);
        let borrowed: Vec<&str> = queries.iter().map(String::as_str).collect();
        let (found, _) =
            checked_ignoring_under(vec![(IN_PLANS, text)], &tree(), &borrowed, &anchors);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn a_path_shape_whose_first_segment_names_nothing_here_is_silent() {
        // A media type, a unit, a git ref, a notation: none names a file or a directory of
        // this tree from any place it could be read from, so none is a pointer here.
        let m = manifest();
        let text = "See `application/json`, `km/h`, `origin/main`, `a/b`, `**/*.rs` and \
                    `/usr/bin/env`.\n";
        let (found, _) = checked(&m, text, &tree());
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn a_path_shape_is_read_from_the_root_from_every_anchor_and_beside_its_file() {
        // One span per place a first segment may sit: the root, a component's directory, the
        // document's own directory, and above it through an upward segment. A suffix is no
        // part of the path.
        let m = manifest();
        let mut present = tree();
        for p in ["src", "src/main.rs", "notes", "notes/sub", "notes/sub/x.md"] {
            present.push(p.to_string());
        }
        for span in [
            "src/main.rs",
            "notes/real/a.md",
            "sub/x.md",
            "../parts/a-part",
            "./sub/x.md",
            "src/main.rs:12",
            "/src/main.rs",
        ] {
            let (found, _) = checked(&m, &format!("See `{span}`.\n"), &present);
            assert_eq!(found.len(), 1, "{span}: {found:#?}");
            assert!(found[0].contains("names no anchor"), "{span}: {found:#?}");
        }
        // An upward segment past the root names nothing, from the deepest place either, and
        // an absolute path is read from the root alone.
        for span in ["../../../src/main.rs", "/sub/x.md"] {
            let (found, _) = checked(&m, &format!("See `{span}`.\n"), &present);
            assert_eq!(found, Vec::<String>::new(), "{span}: {found:#?}");
        }
    }

    #[test]
    fn an_ignored_first_segment_is_silent_and_asks_git_nothing() {
        // An ignored directory is in no listing. Asking the ignore rules would cost a spelling
        // per span and place, and fails outright on a spelling through a symlink, so the lint
        // answers from the listing alone.
        let m = manifest();
        let anchors = Anchors::declared(&m);
        let text = "It writes `build/out.bin`.\n";
        let model = Model::from_documents(vec![(PathBuf::from("notes/prose.md"), text.into())]);
        assert_eq!(
            super::ignore_queries(&model, &anchors),
            Vec::<String>::new()
        );
        let (found, _) = checked_ignoring_under(
            vec![("notes/prose.md", text)],
            &tree(),
            &["build/", "build"],
            &anchors,
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn a_spec_anchor_is_no_place_a_path_is_read_from() {
        // A spec is an anchor and a file. Read as a directory, `..` from it would land in the
        // specs directory, and a span would be a finding only while some spec is open.
        let m = manifest();
        let mut present = tree();
        for p in [
            "docs",
            "docs/plans",
            "docs/plans/specs",
            "docs/plans/specs/s.md",
            "docs/plans/specs/README.md",
        ] {
            present.push(p.to_string());
        }
        let set: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let anchors = Anchors::of(&m, &set);
        assert!(
            anchors
                .all()
                .iter()
                .any(|a| a.path == Path::new("docs/plans/specs/s.md")),
            "the spec is an anchor: {}",
            anchors.listed()
        );
        let (found, _) = checked_under(
            vec![("README.md", "The outer repository is in `../README.md`.\n")],
            &present,
            &anchors,
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    #[test]
    fn a_located_path_is_told_to_drop_its_suffix() {
        let m = manifest();
        for span in ["notes/real/a.md:12", "notes/real/a.md#a-heading"] {
            let (found, _) = checked(&m, &format!("See `{span}`.\n"), &tree());
            assert_eq!(found.len(), 1, "{found:#?}");
            assert!(
                found[0].contains("names no anchor") && found[0].contains("drop the"),
                "{found:#?}"
            );
        }
    }

    #[test]
    fn a_pointer_span_across_a_line_break_is_a_finding_naming_it_joined() {
        // Whether it would resolve is not asked: the span is reported for its shape, and
        // the repair puts it on one line, where this check reads it.
        let m = manifest();
        let text = "The walk in `path@a-part@notes/\nreal/a.md` reads it.\n";
        let (found, counts) = checked(&m, text, &tree());
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("`path@a-part@notes/real/a.md`")
                && found[0].contains("crosses a line break")
                && found[0].contains("one line"),
            "{found:#?}"
        );
        assert_eq!(
            counts.references, 0,
            "a wrapped span is no reference until it is fixed"
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
    fn an_angle_bracketed_a_titled_and_a_defined_target_resolve_like_a_plain_one() {
        // An angle-bracketed target was read with its brackets and reported as dangling
        // when the file existed; a titled one and a definition were read by nothing.
        let m = manifest();
        let present = vec![
            "parts/a-part/notes/real/a.md".to_string(),
            "parts/a-part/notes/real".to_string(),
            "parts/a-part/notes".to_string(),
            "parts/a-part".to_string(),
        ];
        let at = "parts/a-part/notes/README.md";
        let ok = "[a](<real/a.md>) [b](real/a.md \"T\") [c][d]\n\n[d]: real/a.md\n";
        let (found, counts) = checked_in(&m, at, ok, &present);
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
        assert_eq!(counts.links, 3);
        let gone = "[a](<real/gone.md>) [b](real/gone.md \"T\") [c][d]\n\n[d]: real/gone.md\n";
        let (found, _) = checked_in(&m, at, gone, &present);
        assert_eq!(found.len(), 3, "{found:#?}");
        assert!(
            found.iter().all(|f| f.contains("does not exist")),
            "{found:#?}"
        );
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

    /// The harness kinds' references, against walked documents and installed copies.
    fn harness_checked(
        walked: Vec<(&str, &str)>,
        installed: Vec<(&str, &str)>,
    ) -> (Vec<String>, Counts) {
        let mut model = Model::from_documents(
            walked
                .into_iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        model.set_installed(
            installed
                .into_iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        let committed = HashMap::new();
        let present = HashSet::new();
        let directories = HashSet::new();
        let inputs = Inputs {
            committed: &committed,
            configs: &HashMap::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
        };
        let (found, counts) = check_under(&model, &inputs, &Anchors::declared(&manifest()));
        (found.iter().map(|f| f.to_string()).collect(), counts)
    }

    /// The claim: a harness kind's reference resolves to a skill, an agent or a section that
    /// exists, and to nothing with the finding its repair needs: the missing skill named as the
    /// skill, a section only under the skill that defines it. Mutation checked: resolving a
    /// section by its slug across every skill passes a section of one skill cited through another.
    #[test]
    fn a_harness_reference_resolves_to_what_its_owner_defines() {
        let a = "# A\n\n## A part `##a-part`\n";
        let b = "# B\n\n## B part `##b-part`\n";
        let citing = "See `skill@a`, `skill@a@a-part`, `skill@b@b-part`, `primer@goals-bind`.\n\
                      Not `skill@b@a-part`, `skill@c@a-part`, `skill@c`, `primer@nothing`.\n";
        let (found, counts) = harness_checked(
            vec![
                (".claude/skills/a/SKILL.md", a),
                (".claude/skills/b/SKILL.md", b),
                ("notes/prose.md", citing),
            ],
            vec![(
                ".claude/knowledge-architect/PRIMER.md",
                "# P\n\n## Goals bind `##goals-bind`\n",
            )],
        );
        assert_eq!(counts.references, 8);
        assert_eq!(found.len(), 4, "{found:#?}");
        assert!(
            found[0].contains("the skill `b` defines no section `a-part`"),
            "{found:#?}"
        );
        assert!(found[1].contains("no skill is named `c`"), "{found:#?}");
        assert!(found[2].contains("no skill is named `c`"), "{found:#?}");
        assert!(
            found[3].contains("the primer defines no section `nothing`"),
            "{found:#?}"
        );
    }

    /// The claim: an installed copy defines entities and is walked for no reference, so a
    /// dangling reference in one is reported by nothing. Mutation checked: judging the installed
    /// copies with the walked documents reports the planted reference.
    #[test]
    fn an_installed_copy_is_read_for_definitions_and_not_for_references() {
        let copy = "# R\n\n## The axes `##review-axes`\n\nSee `design@a-project@nothing`.\n";
        let (found, _) = harness_checked(
            vec![(
                "notes/prose.md",
                "See `skill@knowledge-architect-review@review-axes`.\n",
            )],
            vec![(".claude/skills/knowledge-architect-review/SKILL.md", copy)],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }

    /// The claim: a backticked span that is exactly the name of a skill or an agent the table
    /// defines, installed or the project's own, is reported with its reference as the repair, in
    /// a fenced block and in a Rust comment too; a span naming no skill and no agent is silent: a
    /// crate's name carrying the installer's prefix, a removed agent's name, a skill's section
    /// slug. Where a skill and an agent share a name, the repair names the skill. Mutations
    /// checked: matching any span with the installer's prefix reports the crate's name and the
    /// removed agent's; trying the agent kind first names the agent for `twin`.
    #[test]
    fn a_bare_name_of_a_skill_or_an_agent_is_reported() {
        let (found, _) = harness_checked(
            vec![
                (
                    ".claude/skills/a-skill/SKILL.md",
                    "# A\n\n## A part `##a-part`\n",
                ),
                (
                    ".claude/agents/an-agent.md",
                    "---\nname: an-agent\n---\n# An agent\n",
                ),
                (".claude/skills/twin/SKILL.md", "# Twin\n"),
                (".claude/agents/twin.md", "---\nname: twin\n---\n# Twin\n"),
                (
                    "notes/prose.md",
                    "Run `knowledge-architect-review`, then `a-skill` and `an-agent`.\n\
                     Not `knowledge-architect-gates`, `knowledge-architect-gone-reviewer`, `a-part`.\n\
                     ```\n`twin`\n```\n",
                ),
                ("src/lib.rs", "//! Run `a-skill`.\n"),
            ],
            vec![(".claude/skills/knowledge-architect-review/SKILL.md", "# R\n")],
        );
        let mut found = found;
        found.sort();
        assert_eq!(
            found,
            vec![
                "notes/prose.md:1  `a-skill` is a bare skill name\n    → write `skill@a-skill`",
                "notes/prose.md:1  `an-agent` is a bare agent name\n    → write `agent@an-agent`",
                "notes/prose.md:1  `knowledge-architect-review` is a bare skill name\n    \
                 → write `skill@knowledge-architect-review`",
                "notes/prose.md:4  `twin` is a bare skill name\n    → write `skill@twin`",
                "src/lib.rs:1  `a-skill` is a bare skill name\n    → write `skill@a-skill`",
            ],
            "{found:#?}"
        );
    }

    /// The claim: a `#<id>` whose id is only a harness entity's, a section slug or a skill's
    /// name, is another tool's notation, not the retired slug reference. Mutation checked:
    /// counting harness kinds in `Entities::defines_id` reports both spans.
    #[test]
    fn a_hash_naming_a_section_slug_is_no_retired_reference() {
        let (found, _) = harness_checked(
            vec![
                (
                    ".claude/skills/a/SKILL.md",
                    "# A\n\n## How to report `##how-to-report`\n",
                ),
                (
                    "notes/prose.md",
                    "The anchor `#how-to-report` and `notes.md#a`.\n",
                ),
            ],
            vec![],
        );
        assert_eq!(found, Vec::<String>::new(), "{found:#?}");
    }
}
