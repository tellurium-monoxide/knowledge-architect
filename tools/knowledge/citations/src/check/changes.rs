//! The changelog: every section verified against the release it names.
//!
//! `CHANGES.md` is append-only below its marker. A bump writes the sections, a human writes
//! the interpretation paragraph under each one, and nothing ever edits a past section — if an
//! old judgement turns out wrong, that is a new entry referencing it, the same discipline as
//! a losing-arguments ledger.
//!
//! **An empty interpretation paragraph is a rule change nobody has triaged, and that is a
//! failure rather than a warning.** The file is a work list with a completion check.

use rules::{norm, Corpus, RuleNumber};

use documentation::finding::Finding;

/// The vocabulary a section entry's `action:` may use.
///
/// Each says what the change cost us, and the set is closed so that a triage cannot invent a
/// verdict nobody has to act on.
pub const ACTIONS: [&str; 6] = [
    "none",
    "quote-updated",
    "citations-updated",
    "interpretation-revised",
    "decision-reopened",
    "code-change-needed",
];

const MARKER: &str = "<!-- APPEND ONLY BELOW";

/// One release's section.
#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub version: String,
    pub meta: Vec<(String, String)>,
    pub entries: Vec<Entry>,
}

/// One rule that changed, with the triage written under it.
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub title: String,
    pub action: Option<String>,
    pub prose: String,
    /// The blockquoted rule text, joined, with the leading rule number removed.
    pub quote: String,
    pub line: u32,
}

impl Section {
    fn meta_value(&self, key: &str) -> Option<&str> {
        self.meta
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Parse the changelog into its sections.
pub fn parse(text: &str) -> Result<(Vec<Section>, String), String> {
    let Some(at) = text.find(MARKER) else {
        return Err(format!("the changelog has no `{MARKER}` marker"));
    };
    let head = text[..at].to_string();
    let body = &text[at..];
    let body = body.split_once("-->").map_or(body, |(_, rest)| rest);
    let offset = text[..at].lines().count() + text[at..text.len() - body.len()].lines().count();

    let mut sections = Vec::new();
    for (start, chunk) in split_headings(body, "## ") {
        let version = chunk.lines().next().unwrap_or("").trim().to_string();
        let meta = meta_block(chunk);
        let mut entries = Vec::new();
        for (entry_start, sub) in split_headings(chunk, "### ") {
            let title = sub.lines().next().unwrap_or("").trim().to_string();
            let block = meta_block(sub);
            let action = block
                .iter()
                .find(|(k, _)| k == "action")
                .map(|(_, v)| v.clone());
            let prose = after_last_fence(sub);
            let quote = blockquote(sub);
            entries.push(Entry {
                title,
                action,
                prose,
                quote,
                line: (offset + start + entry_start) as u32,
            });
        }
        sections.push(Section {
            version,
            meta,
            entries,
        });
    }
    Ok((sections, head))
}

/// `(line offset, text)` for each chunk beginning at a heading of the given depth.
fn split_headings<'a>(text: &'a str, prefix: &str) -> Vec<(usize, &'a str)> {
    let mut out = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    let mut byte = 0;
    for (i, line) in text.lines().enumerate() {
        if let Some(rest) = line.strip_prefix(prefix) {
            if let Some((from, at_line)) = start {
                out.push((at_line, &text[from..byte]));
            }
            start = Some((byte + prefix.len(), i));
            let _ = rest;
        }
        byte += line.len() + 1;
    }
    if let Some((from, at_line)) = start {
        out.push((at_line, &text[from.min(text.len())..]));
    }
    out
}

/// The `key: value` pairs of the first fenced `meta` block, splitting on the first colon.
fn meta_block(chunk: &str) -> Vec<(String, String)> {
    let Some(open) = chunk.find("```meta\n") else {
        return Vec::new();
    };
    let rest = &chunk[open + "```meta\n".len()..];
    let end = rest.find("```").unwrap_or(rest.len());
    rest[..end]
        .lines()
        .filter_map(|l| l.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

/// Everything after the last fence: the triage paragraph.
fn after_last_fence(sub: &str) -> String {
    match sub.rfind("```") {
        Some(at) => sub[at + 3..].trim().to_string(),
        None => sub.trim().to_string(),
    }
}

/// The blockquoted rule text, joined into one line.
fn blockquote(sub: &str) -> String {
    let lines: Vec<&str> = sub
        .lines()
        .filter(|l| l.starts_with('>'))
        .map(|l| l.trim_start_matches('>').trim())
        .collect();
    lines.join(" ")
}

/// Verify one changelog against the releases its sections name.
///
/// `releases` supplies each named release already parsed, because resolving one may read the
/// archive and a check may not.
pub fn check(
    path: &std::path::Path,
    text: &str,
    releases: &std::collections::HashMap<String, Corpus>,
    digests: &std::collections::HashMap<String, String>,
) -> (Vec<Finding>, usize) {
    let mut out = Vec::new();
    let (sections, head) = match parse(text) {
        Ok(parsed) => parsed,
        Err(e) => {
            out.push(Finding::in_file(
                path,
                e,
                "the marker is what makes the file append-only; restore it",
            ));
            return (out, 0);
        }
    };
    let rows: Vec<String> = head
        .lines()
        .filter_map(|l| l.strip_prefix('|'))
        .map(|l| l.trim().trim_matches('`').to_string())
        .filter(|s| s.len() >= 8 && s[..8].bytes().all(|b| b.is_ascii_digit()))
        .map(|s| s[..8].to_string())
        .collect();

    let mut changes = 0;
    for section in &sections {
        let v = &section.version;
        if !rows.contains(v) {
            out.push(Finding::in_file(
                path,
                format!("the section for {v} is missing from the summary table"),
                "add the row: the table is what a reader sees before the sections",
            ));
        }
        for key in ["previous", "date", "source", "sha256"] {
            if section.meta_value(key).is_none() {
                out.push(Finding::in_file(
                    path,
                    format!("the section for {v} records no `{key}`"),
                    "a section pins its own release, and every field is part of that pin",
                ));
            }
        }
        if let Some(source) = section.meta_value("source") {
            if source != rules::release::url_for(v) {
                out.push(Finding::in_file(
                    path,
                    format!("the section for {v} names a source url that is not that release's"),
                    "correct it: the url is how the recorded digest can be re-checked",
                ));
            }
        }
        if let (Some(want), Some(got)) = (section.meta_value("sha256"), digests.get(v)) {
            if want != got {
                out.push(Finding::in_file(
                    path,
                    format!("the section for {v} records a sha256 the release does not have"),
                    "the recorded digest and the bytes disagree — establish which is wrong \
                     before trusting either",
                ));
            }
        }
        for entry in &section.entries {
            changes += 1;
            if entry.prose.is_empty() {
                out.push(Finding::at(
                    path,
                    entry.line,
                    format!("{} is untriaged: no interpretation is written", entry.title),
                    "write what the change costs us; an empty paragraph is a rule change \
                     nobody has read",
                ));
            }
            match &entry.action {
                Some(a) if ACTIONS.contains(&a.as_str()) => {}
                other => out.push(Finding::at(
                    path,
                    entry.line,
                    format!(
                        "{} has action {:?}, which is not one of {}",
                        entry.title,
                        other.as_deref().unwrap_or("(absent)"),
                        ACTIONS.join(", ")
                    ),
                    "use the recorded vocabulary, or argue for a new verdict before adding one",
                )),
            }
            if entry.quote.is_empty() {
                continue;
            }
            let Some(number) = entry
                .title
                .split_whitespace()
                .last()
                .and_then(RuleNumber::parse)
            else {
                continue;
            };
            let quoted = strip_leading_number(&entry.quote);
            if let Some(corpus) = releases.get(v) {
                if corpus.get(&number).map(norm) != Some(norm(&quoted)) {
                    out.push(Finding::at(
                        path,
                        entry.line,
                        format!("the quoted text is not what {number} says in {v}"),
                        "quote the release the section pins, verbatim",
                    ));
                }
            }
        }
    }
    (out, changes)
}

fn strip_leading_number(quote: &str) -> String {
    let trimmed = quote.trim_start();
    match trimmed.split_once(char::is_whitespace) {
        Some((token, rest)) if RuleNumber::parse(token.trim_end_matches('.')).is_some() => {
            rest.to_string()
        }
        _ => trimmed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_without_the_append_only_marker_is_rejected() {
        assert!(parse("# Changes\n\n## 20260807\n").is_err());
    }

    #[test]
    fn a_section_and_its_entries_are_read_back() {
        // The shape a bump writes: the blockquote comes BEFORE the meta block, so the triage
        // paragraph is everything after it — and a section nobody has triaged has nothing
        // there, which is what makes the emptiness check work at all.
        let text = format!(
            "| `20260807` |\n\n{MARKER} -->\n\n\
             ## 20260807\n\n```meta\nprevious: 20260101\ndate: 20260807\n```\n\n\
             ### edited 104.4b\n\n> 104.4b The new text.\n\n\
             ```meta\ncited-by: one file\naction: quote-updated\n```\n\n\
             What it cost us.\n"
        );
        let (sections, head) = parse(&text).expect("parses");
        assert!(head.contains("20260807"));
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].version, "20260807");
        assert_eq!(sections[0].meta_value("previous"), Some("20260101"));
        let entry = &sections[0].entries[0];
        assert_eq!(entry.action.as_deref(), Some("quote-updated"));
        assert_eq!(entry.prose, "What it cost us.");
        assert_eq!(strip_leading_number(&entry.quote), "The new text.");
    }

    #[test]
    fn an_entry_nobody_has_triaged_has_no_prose() {
        // Exactly what a bump appends, with the action left blank for a human to fill in.
        let text = format!(
            "| `20260807` |\n\n{MARKER} -->\n\n## 20260807\n\n\
             ### gone 104.4b\n\n```meta\ncited-by: one file\naction:   \n```\n\n"
        );
        let (sections, _) = parse(&text).expect("parses");
        let entry = &sections[0].entries[0];
        assert!(entry.prose.is_empty(), "{:?}", entry.prose);
        assert_eq!(entry.action.as_deref(), Some(""));
    }

    #[test]
    fn a_quote_keeps_its_text_when_it_opens_with_no_number() {
        assert_eq!(strip_leading_number("Just the text."), "Just the text.");
    }
}
