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
            //
            // **Each `R` binds to the NEAREST concern file named before it on its line.**
            // Comparing every number against every file on the line made a line citing one
            // entry from each of two concerns unwritable: both pairings reported, both
            // false. A number with no file before it is unqualified and checked against
            // none, which is the shape a bare reference already has.
            let named = concern_files_named(text, dir);
            let bound = r_position(text, number).and_then(|p| {
                named
                    .iter()
                    .filter(|(at, _)| *at < p)
                    .max_by_key(|(at, _)| *at)
            });
            if let Some((_, named_file)) = bound {
                if named_file != file {
                    out.push(Finding::at(
                        &doc.rel,
                        line,
                        format!("R{number} is in {file}, not the {named_file} this line names"),
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

/// Concern filenames a line names, as `(byte position, <concern>.md)`.
///
/// The position is where the name starts in the line, which is what nearest-before binding
/// compares against the `R` token's own position.
fn concern_files_named(line: &str, dir: &std::path::Path) -> Vec<(usize, String)> {
    let needle = format!("{}/", dir.file_name().unwrap_or_default().to_string_lossy());
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(i) = line[from..].find(&needle) {
        let at = from + i;
        let after = &line[at + needle.len()..];
        let end = after
            .find(|c: char| !(c.is_ascii_lowercase() || c == '-'))
            .unwrap_or(after.len());
        let stem = &after[..end];
        if !stem.is_empty() && after[end..].starts_with(".md") {
            out.push((at, format!("{stem}.md")));
        }
        from = at + needle.len();
    }
    out
}

/// Where `R<number>` sits in a line, with the boundaries the scanner's pattern uses.
///
/// The scanner records the line and not the column, so the binding re-finds the token. The
/// first occurrence wins; a line repeating the same number is not a shape the register's
/// prose has, and the nearest-file rule reads the same for every copy that shares a prefix.
fn r_position(line: &str, number: u16) -> Option<usize> {
    let token = format!("R{number}");
    let mut from = 0;
    while let Some(i) = line[from..].find(&token) {
        let at = from + i;
        let before_ok = line[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !(c.is_alphanumeric() || c == '_' || c == '.'));
        let after_ok = line[at + token.len()..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_ascii_digit());
        if before_ok && after_ok {
            return Some(at);
        }
        from = at + token.len();
    }
    None
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
        let names = |line: &str| -> Vec<String> {
            concern_files_named(line, &dir)
                .into_iter()
                .map(|(_, f)| f)
                .collect()
        };
        let one = format!("see `{DIR}/{A}.md` for it");
        assert_eq!(names(&one), vec![format!("{A}.md")]);
        let two = format!("`../{DIR}/{B}.md` and `{DIR}/{A}.md`");
        assert_eq!(names(&two), vec![format!("{B}.md"), format!("{A}.md")]);
        // Naming the directory alone is not naming a file, and is what the check exists for.
        let bare = format!("see the {DIR}/ directory");
        assert!(names(&bare).is_empty());
    }

    #[test]
    fn an_r_number_binds_to_the_nearest_concern_file_named_before_it() {
        // The recorded defect: comparing every number against every file on the line made a
        // line citing one entry from each of two concerns unwritable — two findings, both
        // false. Mutation checked: restoring the every-against-every comparison fails the
        // two-concern line below with two findings.
        const DIR: &str = "interpretations";
        const A: &str = "game-loop";
        const B: &str = "object-identity";
        let dir = Path::new("docs/rules").join(DIR);
        let root = crate::manifest::tests::this_project();
        let manifest = crate::Manifest::load(&root).expect("this project's manifest");
        let entry = |n: u16| format!("## R{n} — a reading recorded for this test\n");
        let mistargets = |line: &str| -> Vec<String> {
            let model = crate::model::Model::from_documents(vec![
                (dir.join(format!("{A}.md")), entry(1)),
                (dir.join(format!("{B}.md")), entry(2)),
                (std::path::PathBuf::from("notes/a.md"), line.to_string()),
            ]);
            check(&model, &manifest)
                .0
                .into_iter()
                .filter(|f| f.what.contains("not the"))
                .map(|f| f.what)
                .collect()
        };
        // One entry from each concern on ONE line: both bindings are right, no finding.
        let both = format!("`{DIR}/{A}.md` R1 and `{DIR}/{B}.md` R2, together\n");
        assert_eq!(mistargets(&both), Vec::<String>::new());
        // The nearest file before the number is the wrong one: exactly one finding.
        let wrong = format!("`{DIR}/{B}.md` R1 names the wrong file\n");
        assert_eq!(mistargets(&wrong).len(), 1, "{:#?}", mistargets(&wrong));
        // A number with no file before it is unqualified and checked against none, even
        // with a file named later on the line.
        let after = format!("R1, argued beside `{DIR}/{B}.md`\n");
        assert_eq!(mistargets(&after), Vec::<String>::new());
    }

    #[test]
    fn an_entry_heading_is_only_the_second_level_form() {
        assert_eq!(entry_heading("## R2 — a reading").map(|(n, _)| n), Some(2));
        assert!(entry_heading("### R2 — deeper").is_none());
        assert!(entry_heading("## Rules").is_none());
    }
}
