//! `cargo knowledge rules …` — the corpus and its releases.
//!
//! These are the commands that manage the pinned specification rather than checking anything:
//! see what a release changed, notice that a new one exists, fetch one, and move the project
//! to it. They are the reason the tool is named for its subject and not for checking.

use std::collections::HashMap;
use std::path::Path;
use std::process::ExitCode;

use clap::Subcommand;

use documentation::Manifest;
use rules::release::{self, Tree};
use rules::{Corpus, RuleNumber};

/// The corpus verbs. Dates are named rather than positional wherever two of them meet, per
/// `design@thaum@named-values-where-order-decides`; a lone date has no order to get wrong.
#[derive(Subcommand)]
pub enum RulesCommand {
    /// The pinned text of one or more rules, shaped to be quoted.
    Show {
        #[arg(required = true, num_args = 1.., value_name = "NUMBER")]
        numbers: Vec<String>,
    },
    /// Is a release newer than the pinned one published?
    Latest,
    /// What moved between two releases, filtered to the rules this project cites.
    Diff {
        /// The earlier release.
        #[arg(long, value_name = "DATE", value_parser = release_date)]
        old: String,
        /// The later release.
        #[arg(long, value_name = "DATE", value_parser = release_date)]
        new: String,
    },
    /// Fetch a release into the vendored text and repin to it.
    Fetch {
        /// Without one, the release the version file already names.
        #[arg(value_name = "DATE", value_parser = release_date)]
        date: Option<String>,
    },
    /// Move the project to a release: archive, fetch, reindex, draft the changelog.
    Bump {
        #[arg(value_name = "DATE", value_parser = release_date)]
        date: String,
    },
}

/// A release date as the download URLs spell it: eight digits, `YYYYMMDD`.
///
/// Validated here rather than where it is used: `release::url_for` takes the year with
/// `&date[..4]`, which panics on anything shorter and on a multi-byte boundary. A bad argument
/// is exit 2 per `design@thaum@exit-code-ladder`, and a clap `value_parser` is what holds that line —
/// a panic is 101 and says nothing.
fn release_date(s: &str) -> Result<String, String> {
    if s.len() == 8 && s.bytes().all(|b| b.is_ascii_digit()) {
        return Ok(s.to_string());
    }
    Err(format!(
        "{s:?} is not a release date: expected eight digits, as in 20260807"
    ))
}

pub fn run(manifest: &Manifest, command: &RulesCommand) -> Result<ExitCode, String> {
    let tree = manifest.rules_tree();
    let code = match command {
        RulesCommand::Show { numbers } => show(manifest, &tree, numbers)?,
        RulesCommand::Latest => latest(&tree)?,
        RulesCommand::Diff { old, new } => diff(manifest, &tree, old, new)?,
        RulesCommand::Fetch { date } => fetch(&tree, date.clone())?,
        RulesCommand::Bump { date } => bump(manifest, &tree, date)?,
    };
    Ok(ExitCode::from(code as u8))
}

/// One rule as a citation is written: the marker, the number as printed, then the body entire.
///
/// The number leads the line because that is what binds the body to a rule.
/// `documentation::check::citations::split_rules` reads it off the front of the line, so what
/// this returns is a citation the checker resolves rather than a rendering of one — which is
/// the whole property, since the caller pastes it into a document.
fn quoted(number: &RuleNumber, body: &str) -> String {
    // A section head prints its dot, and the blockquote form needs it: a caller pastes what
    // this prints, and three bare digits opening a blockquote read as commentary.
    let dot = if number.is_section() { "." } else { "" };
    format!("> {number}{dot} {body}")
}

/// The pinned text of one or more rules, shaped to be quoted.
///
/// **The vendored release and no other.** A citation being written lands against what the
/// project is pinned at; text taken from an archived release would produce a quote that fails
/// the moment it lands, and the failure would name the quote rather than the release it came
/// from.
///
/// **A number that resolves to nothing fails the run.** A session that asked for a rule and
/// got silence writes the citation from recollection, which is the one thing root `CLAUDE.md`
/// forbids outright.
fn show(manifest: &Manifest, tree: &Tree, numbers: &[String]) -> Result<i32, String> {
    let text = std::fs::read_to_string(tree.text())
        .map_err(|e| format!("{}: {e}", tree.text().display()))?;
    let corpus = Corpus::parse(&text, manifest.rules().body_starts_at);

    let mut unresolved = 0;
    for arg in numbers {
        let parsed = RuleNumber::parse_any(arg.trim_end_matches('.'));
        // `raw`, never `get`. `get` is normalised for COMPARING — `norm` folds a curly
        // apostrophe to a straight one so a quote written either way verifies — and 1 933 of
        // 3 162 rules carry one. A caller pastes what this prints, so printing the folded form
        // makes every one of those quotes differ from the pinned text in the exact way root
        // `CLAUDE.md` calls the smallest form of paraphrase.
        let Some(body) = parsed.as_ref().and_then(|n| corpus.raw(n)) else {
            println!("\n!! {arg} — the pinned release holds no such rule");
            unresolved += 1;
            continue;
        };
        let number = parsed.expect("a body was found under a parsed number");
        println!("\n{}", quoted(&number, body));

        // What the reader has to know before quoting this, and nothing else. Both notes name
        // a rule of the regime that the text in front of them can fail.
        let subrules = corpus.subrules(&number);
        if !subrules.is_empty() {
            let names: Vec<String> = subrules.iter().map(|n| n.to_string()).collect();
            println!(
                "   subrules, which this body does NOT say: {}",
                names.join(" ")
            );
        }
        if body.chars().count() < documentation::check::regime::MIN_FRAGMENT {
            println!("   under the fragment floor: quote this body whole, which always passes");
        }
    }
    Ok(i32::from(unresolved > 0))
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
         cargo knowledge rules diff --old {pinned} --new {published}\n        \
         cargo knowledge rules bump {published}\n      \
         Nothing here opens a tracker entry: outstanding state goes in a component's\n      \
         issue register by hand, where `cargo knowledge issues` can see it."
    );
    Ok(1)
}

/// Which rules the project cites, from the document model.
fn cited(manifest: &Manifest) -> Result<Vec<RuleNumber>, String> {
    let model = documentation::Model::build(manifest, Some(crate::checker_source()))
        .map_err(|e| e.to_string())?;
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
fn diff(manifest: &Manifest, tree: &Tree, old: &str, new: &str) -> Result<i32, String> {
    let (old, new) = (
        corpus_at(tree, manifest, old)?,
        corpus_at(tree, manifest, new)?,
    );
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
    let (digest, lines) = vendor(tree, &date, &url, &curl(&url)?)?;
    println!("{lines} lines\ndate:   {date}\nsource: {url}\nsha256: {digest}");
    Ok(0)
}

/// Write a downloaded release into the tree: fold it, vendor the text, record its digest.
///
/// The digest recorded is of the bytes as written, never of the download, so the version
/// file can be re-checked against the committed file without refetching anything.
fn vendor(tree: &Tree, date: &str, url: &str, published: &[u8]) -> Result<(String, usize), String> {
    let bytes = release::normalize(published);
    std::fs::write(tree.text(), &bytes).map_err(|e| e.to_string())?;
    let digest = release::sha256(&bytes);
    std::fs::write(
        tree.version(),
        format!("date:   {date}\nsource: {url}\nsha256: {digest}\n"),
    )
    .map_err(|e| e.to_string())?;
    Ok((digest, bytes.iter().filter(|&&b| b == b'\n').count()))
}

/// What the two releases' effective-as-of lines say about a bump.
///
/// A release is dated by its URL, and Wizards has re-published one release's rules under a
/// new URL date with nothing changed but typography. The URL date is all the release watch
/// can see, so a bump names the re-export case outright rather than leaving it to a diff.
fn effective_report(old_text: &str, new_text: &str, old: &str, new: &str) -> String {
    match (
        release::effective_as_of(old_text),
        release::effective_as_of(new_text),
    ) {
        (Some(a), Some(b)) if a == b => format!(
            "Both releases say \"{a}\" — {new} re-publishes the same rules under a new\n\
             URL date, and any file difference is typography."
        ),
        (a, b) => format!(
            "{old}: {}\n{new}: {}",
            a.unwrap_or("(no effective-as-of line)"),
            b.unwrap_or("(no effective-as-of line)")
        ),
    }
}

/// Move the project to a new release.
///
/// The checkers ARE the change detector: every citation is a verbatim quote, so a quote that
/// stops verifying is a rule that moved under a decision we made. What this prints is a work
/// list, not a build break to silence.
fn bump(manifest: &Manifest, tree: &Tree, new: &str) -> Result<i32, String> {
    let old = pinned_date(tree)?;
    if old == new {
        println!("already pinned at {new}");
        return Ok(0);
    }
    println!("== {old} -> {new} ==");

    let old_text = std::fs::read_to_string(tree.text()).map_err(|e| e.to_string())?;
    let old_corpus = Corpus::parse(&old_text, manifest.rules().body_starts_at);
    let new_path = release::resolve(tree, new)?;
    let new_text =
        std::fs::read_to_string(&new_path).map_err(|e| format!("{}: {e}", new_path.display()))?;
    let new_corpus = Corpus::parse(&new_text, manifest.rules().body_starts_at);
    let cited = cited(manifest)?;
    let changes = rules::diff::diff(&old_corpus, &new_corpus, &cited);
    for change in &changes {
        println!("\n{change}");
    }

    println!("\n{}", effective_report(&old_text, &new_text, &old, new));

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

    // Vendor the SAME bytes the diff and the report were computed from. Fetching again here
    // opened a window in which the work list described one download and the pin held another,
    // with nothing tying the two together.
    let url = release::url_for(new);
    let (digest, lines) = vendor(tree, new, &url, new_text.as_bytes())?;
    println!("{lines} lines\ndate:   {new}\nsource: {url}\nsha256: {digest}");
    let model = documentation::Model::build(manifest, Some(crate::checker_source()))
        .map_err(|e| e.to_string())?;
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
              3. cargo knowledge check\n  \
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

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `design@knowledge@checker-source-literals-are-data`.

    #[test]
    fn the_effective_report_names_a_re_export_only_when_the_lines_agree() {
        // The mutation this guards against: inverting the agreement test, which would call a
        // genuine rules update typography — the exact misleading verdict the report exists
        // to prevent.
        let same = "title\nThese rules are effective as of January 1, 1998.\n";
        let other = "title\nThese rules are effective as of February 2, 1998.\n";
        let report = effective_report(same, same, "19980101", "19980102");
        assert!(report.contains("re-publishes the same rules"), "{report}");
        let report = effective_report(same, other, "19980101", "19980202");
        assert!(!report.contains("re-publishes"), "{report}");
        assert!(
            report.contains("January 1") && report.contains("February 2"),
            "{report}"
        );
        let report = effective_report("no dated line\n", same, "19980101", "19980202");
        assert!(report.contains("(no effective-as-of line)"), "{report}");
        assert!(!report.contains("re-publishes"), "{report}");
    }

    #[test]
    fn vendoring_folds_the_published_bytes_and_digests_what_it_wrote() {
        // The mutation this guards against: writing or digesting the raw download, either of
        // which leaves the version file's sha256 not matching the committed text.
        let dir = std::env::temp_dir().join("knowledge-vendor-test");
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let tree = Tree::new(
            &dir,
            dir.join("MagicCompRules.txt"),
            dir.join("VERSION"),
            dir.join("past"),
            dir.join("MANIFEST.tsv"),
        );
        let (digest, lines) = vendor(
            &tree,
            "19980101",
            "https://e.test/r.txt",
            b"\xef\xbb\xbfthe rules text\r\n",
        )
        .expect("a vendored release");
        let written = std::fs::read(tree.text()).expect("the vendored text");
        assert_eq!(written, b"the rules text\n");
        assert_eq!(lines, 1);
        assert_eq!(digest, release::sha256(&written));
        let version = std::fs::read_to_string(tree.version()).expect("the version file");
        assert!(
            version.contains(&digest),
            "the version file records {digest}"
        );
    }

    #[test]
    fn a_citing_list_is_read_out_of_the_pre_bump_index() {
        // An index line and a lookup key, not claims about what either rule says.
        let index = "| `104.4b` | 2 | one.md · A<br>two.md |\n";
        let n = RuleNumber::parse("104.4b").expect("a rule number");
        let cited_by = |rule: &RuleNumber| -> String {
            index
                .lines()
                .find(|l| l.starts_with(&format!("| `{rule}` |")))
                .and_then(|l| l.splitn(4, '|').nth(3))
                .map(|w| w.trim().trim_end_matches('|').trim().replace("<br>", ", "))
                .unwrap_or_else(|| "(not cited)".to_string())
        };
        assert_eq!(cited_by(&n), "one.md · A, two.md");
        let absent = RuleNumber::parse("900.1").expect("a rule number");
        assert_eq!(cited_by(&absent), "(not cited)");
    }

    /// What `show` prints parses back, through the checker's own splitter, to the rule it
    /// names and to that rule's body entire.
    ///
    /// This is the property the command exists for: the caller pastes the line into a
    /// document and the citation check accepts it. Asserting the format by eye would pass
    /// while `split_rules` read the line differently, which is the one way this can be wrong
    /// and still look right. The cross-reference case is the shape that has broken a splitter
    /// before — a rule number inside a body, which must not start a second rule.
    #[test]
    fn what_show_prints_parses_back_as_the_rule_it_names() {
        const TEXT: &str = concat!(
            "100.1 A body long enough to be worth quoting, and complete.\n",
            "100.1a A subrule, which says its own thing and not its parent's.\n",
            "100.2 A body that says see rule 100.1 for the general case.\n",
        );
        let corpus = Corpus::parse(TEXT, 0);
        assert_eq!(corpus.len(), 3, "the fixture parses");
        for (number, body) in corpus.iter() {
            let line = quoted(number, body);
            let stripped = line
                .strip_prefix("> ")
                .expect("the line opens as a blockquote")
                .to_string();
            let parts = documentation::check::citations::split_rules(&[stripped]);
            assert_eq!(
                parts,
                vec![(number.clone(), body.to_string())],
                "{line} must read back as one rule with its whole body"
            );
        }
    }

    /// `subrules` lists what `has_subrules` only counts, and neither calls a sibling a child.
    #[test]
    fn subrules_are_lettered_and_a_numbered_sibling_is_not_one() {
        const TEXT: &str = concat!(
            "612.1 A parent rule with a lettered child and a numbered sibling.\n",
            "612.1a The lettered child, which is a subrule of the parent.\n",
            "612.1aa The twenty-seventh sibling, which follows 612.1z and is not a child.\n",
            "612.10 The numbered sibling, which is not under the parent at all.\n",
        );
        let corpus = Corpus::parse(TEXT, 0);
        const PARENT: &str = "612.1";
        const CHILD: &str = "612.1a";
        const SIBLING: &str = "612.1aa";
        let parent = RuleNumber::parse(PARENT).expect("a rule number");
        let names: Vec<String> = corpus
            .subrules(&parent)
            .iter()
            .map(|n| n.to_string())
            .collect();
        assert_eq!(
            names,
            vec![CHILD.to_string(), SIBLING.to_string()],
            "both letterings are children of the numeric parent"
        );
        assert!(corpus.has_subrules(&parent));

        // The lettered child has none of its own: `612.1aa` follows `612.1z` at the SAME
        // level. This is the `704.5a`/`704.5aa` shape in the real corpus, the only two-letter
        // rule in it, and calling it a parent left its citation with no legal repair.
        let child = RuleNumber::parse(CHILD).expect("a rule number");
        assert!(
            !corpus.has_subrules(&child),
            "a rule already lettered has no subrules"
        );
    }
}
