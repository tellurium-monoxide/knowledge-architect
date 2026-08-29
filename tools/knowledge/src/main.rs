//! The one command. Everything this tool does is a subcommand of this binary.
//!
//! The binary owns two things the libraries deliberately do not: finding the project, and
//! deciding what is printed. A library that found the project itself would have to do it from
//! its own location, and then a check could not be run against a model built in memory —
//! which is the property the whole shape rests on.

use std::collections::HashMap;
use std::process::ExitCode;

use documentation::check::citations::Release;
use documentation::check::{Inputs, Only, Report};
use documentation::Manifest;

mod corpus_cmd;

fn main() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: cannot read the working directory: {e}");
            return ExitCode::from(2);
        }
    };
    // The project is whatever declares itself one at or above here. Nothing about any
    // particular repository is compiled in, so pointing the tool at a mock project under
    // `tools/knowledge/tests/projects/` needs no flag and no special case.
    let manifest = match Manifest::find(&cwd) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    if std::env::args().nth(1).as_deref() == Some("check") {
        return match check(&manifest) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::from(2)
            }
        };
    }

    if std::env::args().nth(1).as_deref() == Some("rules") {
        let args: Vec<String> = std::env::args().skip(2).collect();
        return match corpus_cmd::run(&manifest, &args) {
            Ok(code) => ExitCode::from(code as u8),
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::from(2)
            }
        };
    }

    if std::env::args().nth(1).as_deref() == Some("outstanding") {
        return match outstanding(&manifest) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::from(2)
            }
        };
    }

    if std::env::args().nth(1).as_deref() == Some("index") {
        return match index(&manifest) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::from(2)
            }
        };
    }

    if std::env::args().nth(1).as_deref() == Some("model") {
        // Every observation the walk and the scanner produced, one per line. This is what the
        // model was compared against the implementation it replaces with, and it is the way
        // to see what a check is actually being handed.
        return match documentation::Model::build(&manifest) {
            Ok(model) => {
                let dump = model.canonical();
                // The count goes to stderr so that redirecting stdout gives a file that is
                // only observations, while a reader still learns how much was walked. A
                // document with nothing in it produces no line, so the two numbers together
                // are what say whether the walk agrees with another implementation of it.
                eprintln!(
                    "{} documents, {} observations",
                    model.documents().len(),
                    dump.lines().count()
                );
                print!("{dump}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: cannot read the project: {e}");
                ExitCode::from(2)
            }
        };
    }

    // No subcommand is not success. A tool whose default is to exit 0 having done nothing is
    // the silent false negative everything here exists to prevent.
    eprintln!(
        "knowledge — what this project knows, and whether it still holds.\n\
         project root {}\n\
         \n\
         \x20 check [--only a,b,c]     every check, over one walk; or only these families\n\
         \x20                          {}\n\
         \x20 outstanding [text]       every tracker entry, by directory; or one in full\n\
         \x20 index [--interpretations] [--lines] [--write]\n\
         \x20                          print a generated index, or write it\n\
         \x20 model                    every observation the walk produced\n\
         \x20 rules show <number>…     the pinned text of a rule, shaped to be quoted\n\
         \x20 rules latest             is a newer rules release published?\n\
         \x20 rules diff <old> <new>   what moved, filtered to what this project cites\n\
         \x20 rules fetch [date]       fetch a release and repin to it\n\
         \x20 rules bump <date>        move to a release: archive, fetch, reindex, draft\n",
        manifest.root().display(),
        Only::NAMED
            .iter()
            .map(|(n, _)| *n)
            .collect::<Vec<_>>()
            .join(", ")
    );
    ExitCode::from(2)
}

/// Every check, over one walk.
///
/// Releases are resolved here rather than inside a check: resolving one may read the archive
/// or reach the network, and a check may do neither. What a check receives is a release
/// already parsed.
fn check(manifest: &Manifest) -> Result<ExitCode, String> {
    let args: Vec<String> = std::env::args().skip(2).collect();
    let only = match args.iter().position(|a| a == "--only") {
        Some(i) => Only::parse(args.get(i + 1).ok_or("--only needs a family")?.as_str())?,
        None => Only::EVERYTHING,
    };
    let model = documentation::Model::build(manifest).map_err(|e| e.to_string())?;
    let tree = manifest.rules_tree();
    let body_starts_at = manifest.rules().body_starts_at;

    // Only the families that read rule text get their releases resolved, and resolving a pin
    // may fetch over the network. A run asking for slugs has no business reaching for a
    // release, and before this was scoped it failed on a pin it had no reason to read.
    // `generated` renders the rule index, so it needs the vendored release and no other.
    let mut releases: HashMap<Option<String>, Release> = HashMap::new();
    if only.has(Only::CITATIONS) || only.has(Only::GENERATED) || only.has(Only::REGIME) {
        let vendored = std::fs::read_to_string(tree.text())
            .map_err(|e| format!("{}: {e}", tree.text().display()))?;
        releases.insert(None, Release::new(&vendored, body_starts_at));
    }
    if only.has(Only::CITATIONS) || only.has(Only::REGIME) {
        for doc in model.documents() {
            let Some(date) = &doc.pin else { continue };
            if releases.contains_key(&doc.pin) {
                continue;
            }
            let path = rules::release::resolve(&tree, date)?;
            let text =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            releases.insert(doc.pin.clone(), Release::new(&text, body_starts_at));
        }
    }

    // The generated files are outside the walk — it excludes them by name, because a
    // generated file is not a source of citations. They are read here so a check does not.
    let mut committed = HashMap::new();
    for rel in [
        manifest.rules().dir.join("index.md"),
        manifest.interpretations().dir.join("index.md"),
    ] {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&rel)) {
            committed.insert(rel, text);
        }
    }
    let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
    let pinned = rules::release::read_version(&version)
        .get("date")
        .cloned()
        .ok_or("VERSION names no date")?;
    // One listing answers every question a check has about what is there, and it is the
    // caller's job because a check may not touch the filesystem.
    let survey = documentation::survey::survey(manifest, &model).map_err(|e| e.to_string())?;
    let inputs = Inputs {
        releases: &releases,
        pinned: &pinned,
        committed: &committed,
        present: &survey.present,
        directories: &survey.directories,
        outside: &survey.outside,
    };
    let mut report = documentation::check::run(&model, manifest, &inputs, only);

    // The changelog is outside the walk: each section pins its own release, so it is checked
    // against that release rather than the vendored one. The caller resolves them, as with
    // every other release.
    let changes_path = manifest.rules().dir.join("CHANGES.md");
    let wants_changes = only.has(Only::CHANGES);
    let wants_corpus = only.has(Only::CORPUS);
    if wants_changes || wants_corpus {
        // Both families are gated on the changelog being readable, so when it is not there
        // neither ran. Saying otherwise would print a zero for a check nobody performed,
        // which for `corpus` means reporting provenance clean without having read a byte
        // of it.
        if std::fs::read_to_string(manifest.root().join(&changes_path)).is_err() {
            report.ran = report.ran.without(Only::CHANGES.union(Only::CORPUS));
        }
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&changes_path)) {
            let (sections, _) = documentation::check::changes::parse(&text).unwrap_or_default();
            let mut corpora = HashMap::new();
            let mut digests = HashMap::new();
            for section in &sections {
                let path = rules::release::resolve(&tree, &section.version)?;
                let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
                digests.insert(section.version.clone(), rules::release::sha256(&bytes));
                let text = String::from_utf8_lossy(&bytes).to_string();
                corpora.insert(
                    section.version.clone(),
                    rules::Corpus::parse(&text, body_starts_at),
                );
            }
            if wants_changes {
                let (found, changes) =
                    documentation::check::changes::check(&changes_path, &text, &corpora, &digests);
                report.findings.extend(found);
                report.changelog_changes = changes;
            }
            if wants_corpus {
                // Every release the changelog names must already be in the tree, alongside
                // the pin. The changelog is parsed above whichever family asked for it,
                // because this check reads it for the release list rather than to check it.
                let mut needed = vec![(pinned.clone(), "the pin".to_string())];
                needed.extend(sections.iter().map(|s| {
                    (
                        s.version.clone(),
                        format!("the changelog section {}", s.version),
                    )
                }));
                let (problems, corpus_counts) = rules::integrity::check(&tree, &needed);
                for p in problems {
                    report
                        .findings
                        .push(documentation::Finding::in_file(&p.about, p.what, p.action));
                }
                report.corpus = corpus_counts;
            }
        }
    }
    // The summary first, the findings under it, the verdict on the last line. The order is
    // the whole point: a caller reading the tail of the output has to reach the answer, and
    // when the findings came first every `| tail` and every `| grep` for a count printed a
    // success-shaped report over a failing run.
    print!("{}", counts(&report));
    if !report.findings.is_empty() {
        println!();
        for finding in &report.findings {
            println!("{finding}");
        }
    }
    println!("{}", verdict(&report));
    Ok(if report.failed() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Print a generated index, or write it.
///
/// Printing is the default because the file is read as a diff: `--write` is the deliberate
/// act, and everything else leaves the tree alone.
fn index(manifest: &Manifest) -> Result<ExitCode, String> {
    let args: Vec<String> = std::env::args().collect();
    let interpretations = args.iter().any(|a| a == "--interpretations");
    let with_lines = args.iter().any(|a| a == "--lines");
    if with_lines && !interpretations {
        return Err("--lines applies to --interpretations only".into());
    }
    let model = documentation::Model::build(manifest).map_err(|e| e.to_string())?;
    let (rel, text) = if interpretations {
        (
            manifest.interpretations().dir.join("index.md"),
            documentation::index::interpretation_index(&model, manifest, with_lines),
        )
    } else {
        let tree = manifest.rules_tree();
        let corpus_text = std::fs::read_to_string(tree.text()).map_err(|e| e.to_string())?;
        let corpus = rules::Corpus::parse(&corpus_text, manifest.rules().body_starts_at);
        let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
        let pinned = rules::release::read_version(&version)
            .get("date")
            .cloned()
            .ok_or("VERSION names no date")?;
        (
            manifest.rules().dir.join("index.md"),
            documentation::index::rule_index(&model, manifest, &corpus, &pinned),
        )
    };
    if args.iter().any(|a| a == "--write") {
        std::fs::write(manifest.root().join(&rel), &text).map_err(|e| e.to_string())?;
        eprintln!("wrote {}", rel.display());
    } else {
        print!("{text}");
    }
    Ok(ExitCode::SUCCESS)
}

/// Every tracker entry in the project, or the one entry a search names.
fn outstanding(manifest: &Manifest) -> Result<ExitCode, String> {
    let args: Vec<String> = std::env::args().skip(2).collect();
    let want_issues = !args.iter().any(|a| a == "--tripwires");
    let want_tripwires = !args.iter().any(|a| a == "--issues");
    let needle: Vec<&str> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .map(String::as_str)
        .collect();

    let model = documentation::Model::build(manifest).map_err(|e| e.to_string())?;
    let entries = documentation::outstanding::entries(&model, manifest);

    if !needle.is_empty() {
        // Print one entry in full. A recorded entry usually says more than a fresh diagnosis
        // will — the measurement already taken, what was ruled out, and often why the work was
        // deliberately left undone.
        let needle = needle.join(" ").to_lowercase();
        let hits: Vec<_> = entries
            .iter()
            .filter(|e| e.title.to_lowercase().contains(&needle))
            .collect();
        if hits.is_empty() {
            println!("no entry matching {needle:?}");
            return Ok(ExitCode::FAILURE);
        }
        for e in hits {
            println!(
                "=== {} — {} ===\n{}\n",
                e.file.display(),
                e.title,
                e.body.trim()
            );
        }
        return Ok(ExitCode::SUCCESS);
    }

    let mut files: Vec<&std::path::Path> = entries.iter().map(|e| e.file.as_path()).collect();
    files.sort();
    files.dedup();
    let (mut issues, mut tripwires) = (0, 0);
    for file in &files {
        let rows: Vec<_> = entries.iter().filter(|e| e.file == *file).collect();
        let is_issue = rows[0].is_issue;
        if (is_issue && !want_issues) || (!is_issue && !want_tripwires) {
            continue;
        }
        println!("\n{}", file.display());
        for e in &rows {
            println!("  {:<12} {}", e.kind, e.title);
        }
        if is_issue {
            issues += rows.len();
        } else {
            tripwires += rows.len();
        }
    }
    println!(
        "\n{issues} open issue(s), {tripwires} tripwire(s) across {} tracker file(s)",
        documentation::outstanding::tracker_files(&model, manifest).len()
    );
    println!("An issue needs doing. A tripwire does not — it is a hypothesis about a future");
    println!("failure, and it leaves its file when it fires. See the tracking-open-issues skill.");
    Ok(ExitCode::SUCCESS)
}

/// The last line, and the only one that states the outcome.
///
/// **It is derived from the finding list rather than tracked beside it**, so it cannot
/// disagree with the exit code: `Report::failed` is the same predicate over the same vector.
/// A caller scripting against this reads the exit code; a person reads this line.
fn verdict(report: &Report) -> String {
    match report.findings.len() {
        0 => "PASSED: no findings".to_string(),
        1 => "FAILED: 1 finding above".to_string(),
        n => format!("FAILED: {n} findings above"),
    }
}

/// What a run looked at, as the block printed above the findings.
///
/// **A family that did not run contributes nothing.** Its counts are not zero, they are
/// unasked, and a zero would read as "nothing found" for a check that never ran. The
/// `checked:` line is what makes that legible for the two families that carry no count of
/// their own: without it, a run of `generated` alone and a run that performed nothing look
/// the same.
fn counts(report: &Report) -> String {
    use std::fmt::Write;

    let mut out = String::new();
    let ran = report.ran;
    let _ = write!(out, "\nchecked: {}", ran.names().join(", "));
    let skipped = report.asked.without(ran);
    if skipped != Only::NOTHING {
        // Asked for and not performed. Without this line the two failures are the same
        // output: a family that ran and found nothing, and one that could not run at all.
        let _ = write!(
            out,
            "\nNOT RUN: {} — asked for, and the input it reads is not there",
            skipped.names().join(", ")
        );
    }

    if ran.has(Only::CITATIONS) {
        let c = &report.counts;
        let _ = write!(
            out,
            "\n{}/{} rule-quote fragments verified against the rule cited",
            c.verified, c.fragments
        );
        if c.misattributed > 0 {
            let _ = write!(
                out,
                "\n{} fragment(s) verify against a DIFFERENT rule",
                c.misattributed
            );
        }
        if c.commentary > 0 {
            let _ = write!(
                out,
                "\n{} blockquote(s) hold commentary, not rule text",
                c.commentary
            );
        }
        if c.short > 0 {
            let _ = write!(
                out,
                "\n{} elided fragment(s) under {} chars: checked, but weak evidence",
                c.short,
                documentation::check::citations::MIN_FRAGMENT
            );
        }
        if report.pinned.is_empty() {
            let _ = write!(
                out,
                "\n\nno pinned containers: every quote tracks the vendored release"
            );
        } else {
            let _ = write!(
                out,
                "\n\npinned containers (their quotes do not track the vendored release):"
            );
            for (file, date, quotes) in &report.pinned {
                let _ = write!(out, "\n  {file}: cr-version {date}, {quotes} quote(s)");
            }
        }
        let _ = write!(
            out,
            "\n\nlint: {} unmarked rule reference(s), {} orphan identifier marker(s)",
            c.unmarked, c.orphans
        );
    }

    let s = &report.structure;
    let mut structural = String::new();
    if ran.has(Only::COMPONENTS) {
        let _ = write!(
            structural,
            "\ncomponents: {} declared, {} document(s) each plus a design home, {} additional tracker(s)",
            s.components,
            documentation::manifest::COMPONENT_DOCUMENTS.len(),
            s.additional_trackers
        );
    }
    if ran.has(Only::SLUGS) {
        let _ = write!(
            structural,
            "\nslugs: {} defined, {} referenced",
            s.slugs_defined, s.slugs_referenced
        );
    }
    if ran.has(Only::PATHS) {
        let _ = write!(
            structural,
            "\npaths: {} reference(s), {} link(s)",
            s.path_references, s.links
        );
    }
    if ran.has(Only::INTERPRETATIONS) {
        let _ = write!(
            structural,
            "\ninterpretations: {} concerns, {} entries R1-R{}",
            s.concerns, s.entries, s.top_entry
        );
    }
    if ran.has(Only::UNCOVERED) {
        let _ = write!(
            structural,
            "\nuncovered files: {} scanned",
            s.uncovered_files
        );
    }
    if ran.has(Only::CORPUS) {
        let _ = write!(
            structural,
            "\ncorpus: {} archived release(s), {} manifest row(s), {} of {} release(s) resolved locally",
            report.corpus.archived,
            report.corpus.manifest_rows,
            report.corpus.releases_local,
            report.corpus.releases_needed
        );
    }
    if ran.has(Only::CHANGES) {
        let _ = write!(
            structural,
            "\nchangelog: {} rule change(s)",
            report.changelog_changes
        );
    }
    if ran.has(Only::REGIME) {
        let _ = write!(
            structural,
            "\nregime: {} claim(s) judged against their scope",
            report.regime.claims
        );
    }
    if !structural.is_empty() {
        out.push('\n');
        out.push_str(&structural);
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::{counts, Only, Report};

    fn report(ran: Only) -> Report {
        Report {
            regime: Default::default(),
            findings: Vec::new(),
            counts: Default::default(),
            structure: Default::default(),
            corpus: Default::default(),
            changelog_changes: 0,
            pinned: Vec::new(),
            ran,
            asked: ran,
        }
    }

    /// A report whose every number is different from every other.
    ///
    /// Distinct values on purpose: with zeros everywhere, two counts printed in the wrong
    /// order render identically, so the headline could read `328/328 verified` while meaning
    /// the reverse and no assertion would move.
    fn numbered(ran: Only) -> Report {
        let mut r = report(ran);
        r.counts.verified = 11;
        r.counts.fragments = 22;
        r.counts.unmarked = 33;
        r.counts.orphans = 44;
        r.structure.components = 188;
        r.structure.additional_trackers = 199;
        r.structure.slugs_defined = 55;
        r.structure.slugs_referenced = 66;
        r.structure.path_references = 77;
        r.structure.links = 78;
        r.structure.concerns = 88;
        r.structure.entries = 99;
        r.structure.top_entry = 111;
        r.structure.uncovered_files = 122;
        r.corpus.archived = 133;
        r.corpus.manifest_rows = 144;
        r.corpus.releases_local = 155;
        r.corpus.releases_needed = 166;
        r.changelog_changes = 177;
        r
    }

    #[test]
    fn every_count_is_rendered_in_the_position_its_label_promises() {
        let out = counts(&numbered(Only::EVERYTHING));
        for expected in [
            "11/22 rule-quote fragments verified against the rule cited",
            "components: 188 declared, 5 document(s) each plus a design home, 199 additional tracker(s)",
            "lint: 33 unmarked rule reference(s), 44 orphan identifier marker(s)",
            "slugs: 55 defined, 66 referenced",
            "paths: 77 reference(s), 78 link(s)",
            "interpretations: 88 concerns, 99 entries R1-R111",
            "uncovered files: 122 scanned",
            "corpus: 133 archived release(s), 144 manifest row(s),",
            "155 of 166 release(s) resolved locally",
            "changelog: 177 rule change(s)",
        ] {
            assert!(out.contains(expected), "missing {expected:?} in {out}");
        }
    }

    #[test]
    fn a_family_asked_for_that_could_not_run_is_named_as_not_run() {
        // The failure this guards is a family gated on an input that is not there: without
        // the line, it prints an empty `checked:` and reads as a run that did nothing.
        let mut r = report(Only::CORPUS.union(Only::SLUGS));
        r.ran = Only::SLUGS;
        let out = counts(&r);
        assert!(out.contains("checked: slugs"), "{out}");
        assert!(out.contains("NOT RUN: corpus"), "{out}");
        assert!(
            !out.contains("corpus:"),
            "no count for a family that did not run: {out}"
        );
        // Nothing is withheld when everything asked for ran.
        assert!(!counts(&report(Only::SLUGS)).contains("NOT RUN"));
    }

    #[test]
    fn a_family_that_did_not_run_contributes_no_count() {
        let out = counts(&report(Only::SLUGS));
        assert!(out.contains("slugs:"), "{out}");
        // Every other family's label is absent rather than present with a zero. A zero here
        // reads as "nothing found" for a check that never ran.
        for label in [
            "components:",
            "paths:",
            "interpretations:",
            "uncovered files:",
            "corpus:",
            "changelog:",
            "lint:",
            "fragments verified",
        ] {
            assert!(!out.contains(label), "{label} should be absent from {out}");
        }
    }

    #[test]
    fn the_families_that_ran_are_named_even_when_they_carry_no_count() {
        // `generated` reports findings and counts nothing, so this line is the only thing
        // separating "ran and found nothing" from "did not run".
        let out = counts(&report(Only::GENERATED));
        assert!(out.contains("checked: generated"), "{out}");
        assert!(!out.contains("NOT RUN"), "{out}");
    }

    #[test]
    fn a_full_run_names_every_family_and_prints_both_blocks() {
        let out = counts(&report(Only::EVERYTHING));
        for (name, _) in Only::NAMED {
            assert!(out.contains(name), "{name} should be named in {out}");
        }
        assert!(out.contains("fragments verified"), "{out}");
        assert!(out.contains("changelog:"), "{out}");
    }
}
