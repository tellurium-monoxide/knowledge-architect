//! The interpretation register: one file per declared concern, and every `R` number resolving
//! to exactly one entry.
//!
//! A gap in the numbering is a failure and not a deletion. A superseded entry is kept and
//! marked, so nothing legitimately removes a number, and a hole is an entry that was lost.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::scan::Observation;

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

    // --- every reference resolves, and names the file that holds it -----------------
    for doc in model.documents() {
        if !doc.is_markdown() {
            continue;
        }
        let inside = doc.rel.parent() == Some(dir.as_path());
        for (line, number) in doc.observations_of(|o| match o {
            Observation::InterpRef(n) => Some(*n),
            _ => None,
        }) {
            let text = doc.text.lines().nth(line as usize - 1).unwrap_or("");
            if inside && entry_heading(text).is_some() {
                continue; // the entry's own heading defines it
            }
            let Some((file, _)) = seen.get(&number) else {
                out.push(Finding::at(
                    &doc.rel,
                    line,
                    format!("R{number} has no entry in the register"),
                    "point at an entry that exists, or write the entry",
                ));
                continue;
            };
            // A reference naming the register but not the entry's file passes a path check,
            // because the directory always resolves. This is the half the addressing decision
            // rests on: a re-filing has to rewrite every file that names the old concern.
            for named in concern_files_named(text, dir) {
                if named != *file {
                    out.push(Finding::at(
                        &doc.rel,
                        line,
                        format!("R{number} is in {file}, not the {named} this line names"),
                        "repair the concern file it names; a re-filing rewrites every \
                         reference in the same change",
                    ));
                }
            }
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

/// Concern filenames a line names, as `<dir>/<concern>.md`.
fn concern_files_named(line: &str, dir: &std::path::Path) -> Vec<String> {
    let needle = format!("{}/", dir.file_name().unwrap_or_default().to_string_lossy());
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find(&needle) {
        let after = &rest[at + needle.len()..];
        let end = after
            .find(|c: char| !(c.is_ascii_lowercase() || c == '-'))
            .unwrap_or(after.len());
        let stem = &after[..end];
        if !stem.is_empty() && after[end..].starts_with(".md") {
            out.push(format!("{stem}.md"));
        }
        rest = &rest[at + needle.len()..];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn a_line_naming_a_concern_file_is_read_back() {
        // Interpolated, never spelled out: a path literal here is a reference this
        // project's own path check would then have to resolve.
        const DIR: &str = "interpretations";
        const A: &str = "layers";
        const B: &str = "game-loop";
        let dir = Path::new("docs/rules").join(DIR);
        let one = format!("see `{DIR}/{A}.md` for it");
        assert_eq!(concern_files_named(&one, &dir), vec![format!("{A}.md")]);
        let two = format!("`../{DIR}/{B}.md` and `{DIR}/{A}.md`");
        assert_eq!(
            concern_files_named(&two, &dir),
            vec![format!("{B}.md"), format!("{A}.md")]
        );
        // Naming the directory alone is not naming a file, and is what the check exists for.
        let bare = format!("see the {DIR}/ directory");
        assert!(concern_files_named(&bare, &dir).is_empty());
    }

    #[test]
    fn an_entry_heading_is_only_the_second_level_form() {
        assert_eq!(entry_heading("## R2 — a reading").map(|(n, _)| n), Some(2));
        assert!(entry_heading("### R2 — deeper").is_none());
        assert!(entry_heading("## Rules").is_none());
    }
}
