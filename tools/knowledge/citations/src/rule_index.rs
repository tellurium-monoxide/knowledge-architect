//! The rule index, `path@rules@index.md`: every rule the live files cite, and where.
//!
//! **Rendering returns a `String`; nothing here writes**, per
//! `design@knowledge@generated-files-are-pure`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use documentation::index::{label, rel_from};
use documentation::model::Model;
use documentation::scan::Observation;
use rules::{Corpus, RuleNumber};

/// Every rule the live files cite, and where, as `document · heading`.
///
/// The heading is tracked down the file, and **a heading line is applied before the rule
/// numbers on that same line**: a rule named in a heading belongs under that heading, not
/// under the one above it. The scanner emits the observations in the other order, so this
/// reorders them per line rather than replaying them as they arrive.
fn citations(model: &Model, index_dir: &std::path::Path) -> BTreeMap<Sort, Vec<String>> {
    let mut out: BTreeMap<Sort, BTreeSet<String>> = BTreeMap::new();
    for doc in model.documents() {
        // A markdown link, so a reader follows the row where a renderer shows the page.
        let name = format!(
            "[{}]({})",
            label(doc),
            rel_from(index_dir, &doc.rel).display()
        );
        let mut heading = String::new();
        // The headings are the core's observations and the tokens the rules scan's, so the
        // two are gathered per line rather than read as one stream.
        let mut by_line: BTreeMap<u32, (Option<&str>, Vec<RuleNumber>)> = BTreeMap::new();
        for l in &doc.observations {
            if let Observation::Heading { text, .. } = &l.what {
                by_line
                    .entry(l.line)
                    .or_default()
                    .0
                    .get_or_insert(text.as_str());
            }
        }
        for l in crate::rules_scan::of(doc) {
            if let crate::rules_scan::RuleObservation::Token(rule) = l.what {
                by_line.entry(l.line).or_default().1.push(rule);
            }
        }
        for (_, (line_heading, tokens)) in by_line {
            if let Some(text) = line_heading {
                // Everything from an em dash onwards is the statement, not the name.
                heading = text.split('—').next().unwrap_or(text).trim().to_string();
            }
            for rule in tokens {
                let where_ = if heading.is_empty() {
                    name.clone()
                } else {
                    format!("{name} · {heading}")
                };
                out.entry(Sort(rule)).or_default().insert(where_);
            }
        }
    }
    out.into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect()
}

/// A rule number in the order the rule index is generated in: section, then the printed text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sort(pub RuleNumber);

impl Ord for Sort {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp_index(&other.0)
    }
}

impl PartialOrd for Sort {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// `path@rules@index.md`: every rule cited, and what depends on it.
///
/// This is what makes a bump actionable — when a rule changes or is renumbered, it says
/// exactly what has to be re-read.
pub fn rule_index(model: &Model, dir: &Path, corpus: &Corpus, pinned: &str) -> String {
    let cites = citations(model, dir);
    let (known, unknown): (Vec<_>, Vec<_>) = cites
        .iter()
        .partition(|(Sort(rule), _)| corpus.contains(rule));

    let mut out = String::new();
    out.push_str("# Rule citation index\n\n");
    // The provenance line names the command that regenerates the file, because a generated
    // file a reader cannot regenerate is a file they will edit by hand.
    out.push_str("**Generated — do not edit.** `cargo knowledge index`\n\n");
    out.push_str(
        "Every Comprehensive Rule the live files cite, and where. This is what makes a\n\
         rules bump actionable: when a rule changes or is renumbered, this says exactly what\n\
         depends on it — documents today, code and tests once they exist. The survey is\n\
         excluded, being frozen against its own release.\n\n",
    );
    out.push_str(&format!(
        "Pinned rules release: `{pinned}` · {} rules cited\n\n",
        known.len()
    ));
    out.push_str("| Rule | Cites | Where |\n|---|---|---|\n");
    for (Sort(rule), where_) in &known {
        out.push_str(&format!(
            "| `{rule}` | {} | {} |\n",
            where_.len(),
            where_.join("<br>")
        ));
    }
    if !unknown.is_empty() {
        let names: Vec<String> = unknown
            .iter()
            .map(|(Sort(r), _)| format!("`{r}`"))
            .collect();
        out.push_str(&format!(
            "\n**Not rules in this release** (citation error, or prose that parses as one): {}\n",
            names.join(", ")
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn a_rule_index_row_links_relative_to_the_index_directory() {
        // A markdown link a renderer follows, with the upward segments the index's own
        // directory requires — legal here because the generated files sit outside the walk
        // and regenerate, so the link cannot go stale.
        let corpus = Corpus::parse("100.1 A mock rule body.\n", 0);
        let doc = "parts/x/doc.md";
        let model = Model::from_documents(vec![(PathBuf::from(doc), "per CR:100.1\n".to_string())]);
        let index = rule_index(&model, Path::new("r"), &corpus, "20200101");
        let row = format!("[{doc}](../{doc})");
        assert!(index.contains(&row), "{index}");
    }
}
