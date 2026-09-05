//! The interpretation register: one file per declared concern, and every `R` number defined
//! exactly once.
//!
//! A gap in the numbering is a failure and not a deletion. A superseded entry is kept and
//! marked, so nothing legitimately removes a number, and a hole is an entry that was lost.
//!
//! **No reference is resolved here any more.** A bare `R` and digits is a retired form under
//! the reference grammar, reported by the `references` family wherever it stands, so there
//! is no token left for this check to bind to a concern file. The register's move to a
//! declared file register replaces the rest of this module.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;

pub fn check(model: &Model, manifest: &Manifest) -> (Vec<Finding>, (usize, usize, u16)) {
    let dir = &manifest.interpretations().dir;
    let mut out = Vec::new();

    // --- one file per declared concern, in both directions --------------------------
    let declared: BTreeSet<&str> = manifest
        .interpretations()
        .concerns
        .iter()
        .map(String::as_str)
        .collect();
    let present: BTreeSet<String> = model
        .documents()
        .iter()
        .filter(|d| d.rel.parent() == Some(dir.as_path()))
        .map(|d| {
            d.rel
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into()
        })
        .filter(|s: &String| s != "README" && s != "index")
        .collect();
    for concern in &declared {
        if !present.contains(*concern) {
            out.push(Finding::in_file(
                dir.join(format!("{concern}.md")),
                format!("the manifest declares the concern `{concern}` and there is no file"),
                "add the file, or remove the concern from knowledge.toml — the declaration is \
                 the taxonomy, so changing it is its own decision",
            ));
        }
    }
    for concern in &present {
        if !declared.contains(concern.as_str()) {
            out.push(Finding::in_file(
                dir.join(format!("{concern}.md")),
                format!("`{concern}` is a concern file that the manifest does not declare"),
                "declare it in knowledge.toml, or move the file: a partition nobody announced \
                 is one nobody reviewed",
            ));
        }
    }

    // --- every entry defined exactly once, and the sequence unbroken ----------------
    let mut seen: BTreeMap<u16, (String, u32)> = BTreeMap::new();
    for doc in model.documents() {
        if doc.rel.parent() != Some(dir.as_path()) {
            continue;
        }
        let stem = doc.rel.file_stem().unwrap_or_default().to_string_lossy();
        if stem == "README" || stem == "index" {
            continue;
        }
        let name = doc.rel.file_name().unwrap_or_default().to_string_lossy();
        for (i, line) in doc.text.lines().enumerate() {
            let Some((number, _)) = entry_heading(line) else {
                continue;
            };
            let at = (name.to_string(), i as u32 + 1);
            if let Some((other, other_line)) = seen.get(&number) {
                out.push(Finding::at(
                    &doc.rel,
                    at.1,
                    format!("R{number} is defined here and in {other}:{other_line}"),
                    "an R number is cited from other documents and from code, so it has to \
                     resolve to exactly one entry",
                ));
            } else {
                seen.insert(number, at);
            }
        }
    }
    if let Some(top) = seen.keys().max().copied() {
        let missing: Vec<u16> = (1..=top).filter(|n| !seen.contains_key(n)).collect();
        if !missing.is_empty() {
            let names: Vec<String> = missing.iter().map(|n| format!("R{n}")).collect();
            out.push(Finding::in_file(
                dir,
                format!("the sequence has holes: {}", names.join(", ")),
                "a superseded entry is kept and marked, so nothing legitimately removes a \
                 number — a hole is an entry that was lost",
            ));
        }
    }

    let top = seen.keys().max().copied().unwrap_or(0);
    (out, (declared.len(), seen.len(), top))
}

/// `## R<n> — <title>` as an entry's own heading.
fn entry_heading(line: &str) -> Option<(u16, String)> {
    let rest = line.strip_prefix("## R")?;
    let (digits, tail) = rest.split_at(rest.find(|c: char| !c.is_ascii_digit())?);
    let title = tail.strip_prefix(" — ")?;
    Some((digits.parse().ok()?, title.trim_end().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_entry_heading_is_only_the_second_level_form() {
        assert_eq!(entry_heading("## R2 — a reading").map(|(n, _)| n), Some(2));
        assert!(entry_heading("### R2 — deeper").is_none());
        assert!(entry_heading("## Rules").is_none());
    }
}
