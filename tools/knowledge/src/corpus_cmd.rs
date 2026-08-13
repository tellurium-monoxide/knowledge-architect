//! `cargo tools rules …` — the corpus and its releases.
//!
//! These are the commands that manage the pinned specification rather than checking anything:
//! see what a release changed, notice that a new one exists, fetch one, and move the project
//! to it. They are the reason the tool is named for its subject and not for checking.

use std::collections::HashMap;
use std::path::Path;

use documentation::Manifest;
use rules::release::{self, Tree};
use rules::{Corpus, RuleNumber};

pub fn run(manifest: &Manifest, args: &[String]) -> Result<i32, String> {
    let tree = manifest.rules_tree();
    match args.first().map(String::as_str) {
        Some("latest") => latest(&tree),
        Some("diff") => diff(manifest, &tree, &args[1..]),
        Some("fetch") => fetch(&tree, args.get(1).cloned()),
        Some("bump") => bump(manifest, &tree, args.get(1)),
        other => Err(format!(
            "unknown rules command {:?}; expected latest, diff, fetch or bump",
            other.unwrap_or("(none)")
        )),
    }
}

/// Download a URL, or say why not.
fn curl(url: &str) -> Result<Vec<u8>, String> {
    let out = std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "60", url])
        .output()
        .map_err(|e| format!("could not run curl: {e}"))?;
    if !out.status.success() {
        return Err(format!("could not fetch {url} (curl {})", out.status));
    }
    Ok(out.stdout)
}

fn pinned_date(tree: &Tree) -> Result<String, String> {
    let text = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
    release::read_version(&text)
        .get("date")
        .cloned()
        .ok_or_else(|| "the version file names no date".to_string())
}

/// Is a release newer than the pinned one published?
///
/// **No match is a failure, never an answer.** The page is HTML nobody here controls, so the
/// extractor will one day match nothing; reading that as *no new release* keeps this green
/// while the watch is dead, and the first symptom would be a citation failing months later.
fn latest(tree: &Tree) -> Result<i32, String> {
    let pinned = pinned_date(tree)?;
    let page = String::from_utf8_lossy(&curl(rules::watch::PAGE)?).to_string();
    let Some(published) = rules::watch::newest(&page) else {
        println!(
            "FAIL  {} carries no rules link this can read.\n      \
             This is the EXTRACTOR failing, not an answer about the corpus:\n      \
             check the page by hand, then fix the pattern in rules/src/watch.rs.",
            rules::watch::PAGE
        );
        return Ok(2);
    };
    if published == pinned {
        println!("pinned at {pinned}; published {published} — up to date");
        return Ok(0);
    }
    if published < pinned {
        println!(
            "FAIL  published {published} is EARLIER than the pinned {pinned}.\n      \
             The page rolled back, or the version file names a release that was never\n      \
             published. Neither is a bump; establish which before doing anything."
        );
        return Ok(2);
    }
    println!(
        "FAIL  a newer release is published: {published} (pinned at {pinned}).\n      \
         Read the bumping-rules skill, then:\n        \
         cargo tools rules diff {pinned} {published}\n        \
         cargo tools rules bump {published}\n      \
         Nothing here opens a tracker entry: outstanding state goes in a family's\n      \
         open-issues.md by hand, where `cargo tools outstanding` can see it."
    );
    Ok(1)
}

/// Which rules the project cites, from the document model.
fn cited(manifest: &Manifest) -> Result<Vec<RuleNumber>, String> {
    let model = documentation::Model::build(manifest).map_err(|e| e.to_string())?;
    let mut out: Vec<RuleNumber> = model
        .documents()
        .iter()
        .flat_map(|d| d.observations.iter())
        .filter_map(|l| match &l.what {
            documentation::Observation::RuleToken(n) => Some(n.clone()),
            _ => None,
        })
        .collect();
    out.sort_by(|a, b| a.cmp_index(b));
    out.dedup();
    Ok(out)
}

fn corpus_at(tree: &Tree, manifest: &Manifest, date: &str) -> Result<Corpus, String> {
    let path = release::resolve(tree, date)?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(Corpus::parse(&text, manifest.rules().body_starts_at))
}

/// What moved between two releases, filtered to the rules this project cites.
fn diff(manifest: &Manifest, tree: &Tree, args: &[String]) -> Result<i32, String> {
    let local = args.iter().any(|a| a == "--local");
    let dates: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let (old, new) = match (local, dates.len()) {
        (true, 1) => {
            let text = std::fs::read_to_string(tree.text()).map_err(|e| e.to_string())?;
            (
                Corpus::parse(&text, manifest.rules().body_starts_at),
                corpus_at(tree, manifest, dates[0])?,
            )
        }
        (false, 2) => (
            corpus_at(tree, manifest, dates[0])?,
            corpus_at(tree, manifest, dates[1])?,
        ),
        _ => {
            return Err("usage: rules diff <old> <new>   |   rules diff --local <new>".to_string())
        }
    };
    let cited = cited(manifest)?;
    let changes = rules::diff::diff(&old, &new, &cited);
    for change in &changes {
        println!("\n{change}");
    }
    let real = cited
        .iter()
        .filter(|r| old.contains(r) || new.contains(r))
        .count();
    let (mut edited, mut moved, mut gone) = (0, 0, 0);
    for change in &changes {
        match change {
            rules::diff::Change::Changed { .. } => edited += 1,
            rules::diff::Change::Moved { .. } => moved += 1,
            rules::diff::Change::Gone { .. } => gone += 1,
        }
    }
    println!("\n{real} cited rules: {edited} edited, {moved} renumbered, {gone} gone");
    Ok(i32::from(!changes.is_empty()))
}

/// Fetch a release into the vendored text and rewrite the version file to match.
///
/// The filename carries no date on purpose: the version file holds the mutable part, so no
/// cross-reference goes stale on a bump.
fn fetch(tree: &Tree, date: Option<String>) -> Result<i32, String> {
    let date = match date {
        Some(d) => d,
        None => pinned_date(tree)?,
    };
    let url = release::url_for(&date);
    println!("fetching {url}");
    let bytes = curl(&url)?;
    std::fs::write(tree.text(), &bytes).map_err(|e| e.to_string())?;
    let digest = release::sha256(&bytes);
    std::fs::write(
        tree.version(),
        format!("date:   {date}\nsource: {url}\nsha256: {digest}\n"),
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{} lines\ndate:   {date}\nsource: {url}\nsha256: {digest}",
        bytes.iter().filter(|&&b| b == b'\n').count()
    );
    Ok(0)
}

/// Move the project to a new release.
///
/// The checkers ARE the change detector: every citation is a verbatim quote, so a quote that
/// stops verifying is a rule that moved under a decision we made. What this prints is a work
/// list, not a build break to silence.
fn bump(manifest: &Manifest, tree: &Tree, new: Option<&String>) -> Result<i32, String> {
    let Some(new) = new else {
        return Err("usage: rules bump <date>".to_string());
    };
    let old = pinned_date(tree)?;
    if old == *new {
        println!("already pinned at {new}");
        return Ok(0);
    }
    println!("== {old} -> {new} ==");

    let old_corpus = {
        let text = std::fs::read_to_string(tree.text()).map_err(|e| e.to_string())?;
        Corpus::parse(&text, manifest.rules().body_starts_at)
    };
    let new_corpus = corpus_at(tree, manifest, new)?;
    let cited = cited(manifest)?;
    let changes = rules::diff::diff(&old_corpus, &new_corpus, &cited);
    for change in &changes {
        println!("\n{change}");
    }

    // Archive the OUTGOING release before it is overwritten. A release that is no longer
    // vendored is still needed — to compare two, and to hold any container pinned to it — and
    // Wizards rotates its download URLs, so the archive is what keeps that working offline and
    // permanently.
    let archived = tree.past().join(format!("{old}.txt"));
    if archived.exists() {
        println!("past/{old}.txt already archived");
    } else {
        std::fs::create_dir_all(tree.past()).map_err(|e| e.to_string())?;
        std::fs::copy(tree.text(), &archived).map_err(|e| e.to_string())?;
        let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
        let v = release::read_version(&version);
        let row = format!(
            "{old}\t{}\t{}\t{}\n",
            v.get("source").map_or("", String::as_str),
            v.get("sha256").map_or("", String::as_str),
            today()
        );
        append(tree.manifest(), &row)?;
        println!("archived {old} -> past/{old}.txt (+ a manifest row)");
    }

    // Snapshot the PRE-bump index. The section skeleton reads it for each rule's citing list,
    // and regenerating first would drop every renumbered or deleted rule into the index's
    // "not rules in this release" footnote — so the work list would report "(not cited)" for
    // exactly the rules the diff just flagged as the dangerous case.
    let index_path = manifest.root().join(manifest.rules().dir.join("index.md"));
    let pre_index = std::fs::read_to_string(&index_path).unwrap_or_default();

    fetch(tree, Some(new.clone()))?;
    let model = documentation::Model::build(manifest).map_err(|e| e.to_string())?;
    let fresh = {
        let text = std::fs::read_to_string(tree.text()).map_err(|e| e.to_string())?;
        Corpus::parse(&text, manifest.rules().body_starts_at)
    };
    std::fs::write(
        &index_path,
        documentation::index::rule_index(&model, manifest, &fresh, new),
    )
    .map_err(|e| e.to_string())?;

    let version = std::fs::read_to_string(tree.version()).map_err(|e| e.to_string())?;
    let section = skeleton(
        &changes,
        &fresh,
        &release::read_version(&version),
        &pre_index,
        &old,
        new,
    );
    append(
        &manifest
            .root()
            .join(manifest.rules().dir.join("CHANGES.md")),
        &section,
    )?;
    println!(
        "\nCHANGES.md: appended {new}. Every subsection's interpretation paragraph is EMPTY \
         on purpose, and the changelog check fails until a human writes it."
    );
    println!("\nNext, by hand:\n  \
              1. write the interpretation under each subsection and set its action:\n  \
              2. add a row to the summary table at the top of CHANGES.md\n  \
              3. cargo tools check\n  \
              4. commit the rules text, the version file, past/, both indexes, CHANGES.md and\n     \
              every document whose quote or decision changed, TOGETHER. past/ is not\n     \
              optional: a container pinned to the outgoing release resolves from it, and\n     \
              leaving it out breaks that permanently and offline.");
    Ok(0)
}

/// The section a bump appends: one subsection per change, each with its citing list pre-filled
/// and its interpretation left empty.
fn skeleton(
    changes: &[rules::diff::Change],
    corpus: &Corpus,
    version: &HashMap<String, String>,
    pre_index: &str,
    old: &str,
    new: &str,
) -> String {
    use rules::diff::Change;
    let cited_by = |rule: &RuleNumber| -> String {
        pre_index
            .lines()
            .find(|l| l.starts_with(&format!("| `{rule}` |")))
            .and_then(|l| l.splitn(4, '|').nth(3))
            .map(|w| w.trim().trim_end_matches('|').trim().replace("<br>", ", "))
            .unwrap_or_else(|| "(not cited)".to_string())
    };
    let (edited, moved, gone) = (
        changes
            .iter()
            .filter(|c| matches!(c, Change::Changed { .. }))
            .count(),
        changes
            .iter()
            .filter(|c| matches!(c, Change::Moved { .. }))
            .count(),
        changes
            .iter()
            .filter(|c| matches!(c, Change::Gone { .. }))
            .count(),
    );
    let get = |k: &str| version.get(k).cloned().unwrap_or_default();
    let mut out = format!(
        "\n## {new}\n\n```meta\nprevious: {old}\ndate:     {}\nsource:   {}\nsha256:   {}\n\
         edited:   {edited}\nrenumbered: {moved}\ngone:     {gone}\n```\n\n",
        get("date"),
        get("source"),
        get("sha256")
    );
    for change in changes {
        match change {
            Change::Changed { rule, .. } => {
                let body = corpus.get(rule).unwrap_or("(missing)");
                out.push_str(&format!(
                    "### edited {rule}\n\n> {rule} {body}\n\n```meta\ncited-by: {}\naction:   \n```\n\n",
                    cited_by(rule)
                ));
            }
            Change::Moved { rule, to, .. } => out.push_str(&format!(
                "### renumbered {rule} -> {to}\n\n```meta\ncited-by: {}\naction:   \n```\n\n",
                cited_by(rule)
            )),
            Change::Gone { rule, .. } => out.push_str(&format!(
                "### gone {rule}\n\n```meta\ncited-by: {}\naction:   \n```\n\n",
                cited_by(rule)
            )),
        }
    }
    out
}

fn append(path: &Path, text: &str) -> Result<(), String> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(text.as_bytes()).map_err(|e| e.to_string())
}

/// Today, as the archive manifest records it.
fn today() -> String {
    let out = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output();
    out.ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_citing_list_is_read_out_of_the_pre_bump_index() {
        const RULE: &str = "104.4b"; // CR~104.4b
        let index = format!("| `{RULE}` | 2 | one.md · A<br>two.md |\n");
        let n = RuleNumber::parse(RULE).expect("a rule number");
        let cited_by = |rule: &RuleNumber| -> String {
            index
                .lines()
                .find(|l| l.starts_with(&format!("| `{rule}` |")))
                .and_then(|l| l.splitn(4, '|').nth(3))
                .map(|w| w.trim().trim_end_matches('|').trim().replace("<br>", ", "))
                .unwrap_or_else(|| "(not cited)".to_string())
        };
        assert_eq!(cited_by(&n), "one.md · A, two.md");
        let absent = RuleNumber::parse("900.1").expect("a rule number"); // CR~900.1
        assert_eq!(cited_by(&absent), "(not cited)");
    }
}
