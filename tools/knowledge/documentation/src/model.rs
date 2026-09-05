//! The model: every live document, read once, scanned once.

use std::path::{Path, PathBuf};

use crate::manifest::Manifest;
use crate::scan::{self, Located, Observation};
use crate::source::{self, Literals, Parsed};
use crate::walk;

/// One live document and everything that was observed in it.
#[derive(Clone, Debug)]
pub struct Document {
    /// Repository-relative, which is how every finding and every generated file names it.
    pub rel: PathBuf,
    /// The file as it is on disk.
    pub text: String,
    /// Its prose and its scopes, as the parser for its suffix found them.
    pub parsed: Parsed,
    /// The release this file's quotes verify against, where it opts out of the vendored one.
    pub pin: Option<String>,
    pub observations: Vec<Located>,
    /// How its string literals were read: `Data` only under the checker's own source.
    pub literals: Literals,
}

impl Document {
    pub fn is_markdown(&self) -> bool {
        self.rel.extension().is_some_and(|e| e == "md")
    }

    /// Every inline quote in the document, at the FILE line it sits on.
    ///
    /// Extraction runs per prose region rather than over the file, which is what makes a
    /// paragraph lookback stop at the end of a comment: a marker in one doc comment cannot own
    /// a quote in the next function's.
    pub fn inline_quotes(&self) -> Vec<crate::quote::Quote> {
        self.parsed
            .prose
            .iter()
            .flat_map(|region| {
                crate::quote::inline(&region.text)
                    .into_iter()
                    .map(|mut q| {
                        // Both endpoints, or a wrapped quote reports a range that starts in
                        // the file and ends in the region.
                        q.line = region.file_line(q.line as usize - 1);
                        q.last = region.file_line(q.last as usize - 1);
                        q
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Every `>` block in the document, at the FILE line it starts on.
    pub fn blocks(&self) -> Vec<crate::quote::Block> {
        self.parsed
            .prose
            .iter()
            .flat_map(|region| {
                crate::quote::blocks(&region.text)
                    .into_iter()
                    .map(|mut b| {
                        b.line = region.file_line(b.line as usize - 1);
                        b
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Every emphasised quotation in the document that no marker claims.
    pub fn unclaimed_quotes(&self) -> Vec<(u32, String)> {
        self.parsed
            .prose
            .iter()
            .flat_map(|region| {
                crate::quote::unclaimed(&region.text)
                    .into_iter()
                    .map(|(l, body)| (region.file_line(l as usize - 1), body))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// The prose on a file line, where that line carries any.
    ///
    /// A line of code has none, which is the point: a check asking what a line says is asking
    /// about prose, and the answer for code is that there is nothing to read.
    pub fn prose_line(&self, line: u32) -> Option<&str> {
        self.parsed.prose.iter().find_map(|region| {
            region
                .lines
                .iter()
                .position(|&n| n == line)
                .and_then(|i| region.text.split('\n').nth(i))
        })
    }

    /// Every file line that carries prose, in order.
    pub fn prose_lines(&self) -> Vec<u32> {
        let mut out: Vec<u32> = self
            .parsed
            .prose
            .iter()
            .flat_map(|r| r.lines.iter().copied())
            .collect();
        out.sort_unstable();
        out.dedup();
        out
    }

    pub fn observations_of<'a, T: 'a>(
        &'a self,
        f: impl Fn(&'a Observation) -> Option<T> + 'a,
    ) -> impl Iterator<Item = (u32, T)> + 'a {
        self.observations
            .iter()
            .filter_map(move |l| f(&l.what).map(|t| (l.line, t)))
    }
}

/// Every live document in one checkout.
///
/// Built once per run and handed to every check. A check receives this and returns findings:
/// it does not read files, so the tree is walked once and a check can be tested against a
/// model assembled in memory.
#[derive(Clone, Debug)]
pub struct Model {
    root: PathBuf,
    docs: Vec<Document>,
    /// The checker's own directory as the summary names it: relative to the root when it sits
    /// under it, absolute otherwise, `None` when the caller passed none.
    checker_source: Option<PathBuf>,
}

impl Model {
    /// Read and scan a project, as its own manifest declares it.
    ///
    /// `checker_source` is the directory of the checker's own source. A file under it is
    /// parsed with its string literals as data, per
    /// `knowledge#checker-source-literals-are-data`; every other file reads them as prose.
    /// The binary passes its compile-time location, and a library caller checking a tree the
    /// checker is no part of passes `None`. Both sides of the prefix test are canonicalised,
    /// so a symlinked checkout does not defeat it, and a compiled path that resolves to
    /// nothing exempts nothing. **The directory exempts files only when it sits inside the
    /// tree being checked.** A tree that sits inside it instead, such as a mock project under
    /// the checker's own tests, is a foreign project whose every literal is prose.
    pub fn build(manifest: &Manifest, checker_source: Option<&Path>) -> std::io::Result<Self> {
        let root = manifest.root();
        let canonical_root = root.canonicalize()?;
        let checker = checker_source.and_then(|p| p.canonicalize().ok());
        let inside = checker
            .as_ref()
            .filter(|c| c.starts_with(&canonical_root))
            .cloned();
        let walk_config = manifest.walk();
        let mut docs = Vec::new();
        for path in walk::live_files(root, walk_config, manifest.ignore())? {
            let rel_for_error = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let text = match std::fs::read_to_string(&path) {
                Ok(t) => t,
                // **Not silence.** A file the walk cannot read leaves the model AND the
                // inverse assertion, so a citation in it is checked by nothing and the run
                // still passes. One byte of Windows-1252 — a pasted em dash — does it. The
                // document is kept, empty, carrying the reason, so a check reports it.
                Err(e) => {
                    docs.push(Document {
                        rel: rel_for_error,
                        text: String::new(),
                        parsed: Parsed {
                            trouble: Some(format!("this file could not be read as text: {e}")),
                            ..Parsed::default()
                        },
                        pin: None,
                        observations: Vec::new(),
                        literals: Literals::Prose,
                    });
                    continue;
                }
            };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let literals = match &inside {
                Some(c) if canonical_root.join(&rel).starts_with(c) => Literals::Data,
                _ => Literals::Prose,
            };
            let parsed = source::parse(&rel, &text, literals);
            let observations: Vec<Located> = scan::scan(&parsed);
            docs.push(Document {
                rel,
                pin: scan::pin(&parsed, &text),
                text,
                parsed,
                observations,
                literals,
            });
        }
        let checker_source = checker.map(|c| {
            c.strip_prefix(&canonical_root)
                .map(Path::to_path_buf)
                .unwrap_or(c)
        });
        Ok(Self {
            root: root.to_path_buf(),
            docs,
            checker_source,
        })
    }

    /// The checker's own directory as the summary names it, if the caller passed one.
    pub fn checker_source(&self) -> Option<&Path> {
        self.checker_source.as_deref()
    }

    /// How many walked files sit under the checker's own directory.
    ///
    /// Zero in a checkout that holds the tool is the loud failure of
    /// `knowledge#checker-source-literals-are-data`: the compiled path and the walked tree
    /// disagree, and the tool's fixtures are being read as citations.
    pub fn checker_files(&self) -> usize {
        self.docs
            .iter()
            .filter(|d| d.literals == Literals::Data)
            .count()
    }

    /// A model assembled from text rather than from a checkout, for a test.
    ///
    /// It takes no walk configuration: what the walk skips decides which files exist, and a
    /// caller handing the text in has already decided that.
    pub fn from_documents(docs: Vec<(PathBuf, String)>) -> Self {
        let docs = docs
            .into_iter()
            .map(|(rel, text)| {
                let parsed = source::parse(&rel, &text, Literals::Prose);
                let observations = scan::scan(&parsed);
                Document {
                    rel,
                    pin: scan::pin(&parsed, &text),
                    text,
                    parsed,
                    observations,
                    literals: Literals::Prose,
                }
            })
            .collect();
        Self {
            root: PathBuf::new(),
            docs,
            checker_source: None,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn documents(&self) -> &[Document] {
        &self.docs
    }

    /// Every observation as one sorted line: `path<TAB>line<TAB>kind<TAB>value`.
    ///
    /// This exists to be compared against another implementation of the same walk and scan,
    /// over the whole tree at once rather than over a sample.
    pub fn canonical(&self) -> String {
        let mut out = String::new();
        for doc in &self.docs {
            let rel = doc.rel.display();
            for Located { line, what } in &doc.observations {
                let (kind, value) = describe(what);
                if kind.is_empty() {
                    continue;
                }
                out.push_str(&format!("{rel}\t{line}\t{kind}\t{value}\n"));
            }
        }
        out
    }
}

/// `(kind, value)` for the canonical dump. An empty kind is left out of it.
fn describe(what: &Observation) -> (&'static str, String) {
    use crate::scan::MarkerForm::*;
    match what {
        Observation::RuleMarker { number, form } => (
            match form {
                Prose => "marker-prose",
                Identifier => "marker-ident",
                IdentifierInProse => "marker-ident-prose",
            },
            number.to_string(),
        ),
        Observation::RuleToken(n) => ("rule-token", n.to_string()),
        Observation::Heading { level, text } => ("heading", format!("{level} {text}")),
        Observation::SlugDef(s) => ("slug-def", s.clone()),
        // Rendered as it is written, so the dump says which component a pointer names and a
        // reference naming none is visibly different from one that does.
        Observation::SlugRef { component, slug } => (
            "slug-ref",
            match component {
                Some(c) => format!("{c}#{slug}"),
                None => format!("#{slug}"),
            },
        ),
        // Rendered in the syntax it is written in, so a dumped row can be grepped for in the
        // tree it came from. The unsupported span is a different kind: the value is the raw
        // span, and the kind column is what says it parses as no reference.
        Observation::PathRef { anchor, path } => (
            "path-ref",
            format!(
                "{}@{path}",
                match anchor {
                    crate::scan::PathAnchor::Component(c) => c.as_str(),
                    crate::scan::PathAnchor::Elsewhere => crate::scan::ESCAPE_ANCHOR,
                    crate::scan::PathAnchor::Every => "*",
                }
            ),
        ),
        Observation::UnsupportedPath(span) => ("unsupported-path", span.clone()),
        Observation::InterpRef(n) => ("interp-ref", n.to_string()),
        Observation::Link(target) => ("link", target.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Interpolated, never spelled out: this tool's checks walk their own source, and a slug
    // or a path written literally here becomes a real anchor or a real dangling reference.
    const SLUG: &str = "a-slug";
    const COMPONENT: &str = "a-component";
    const DOC: &str = "docs/design/a.md";
    const SRC: &str = "src/b.rs";

    #[test]
    fn a_model_can_be_assembled_without_a_checkout() {
        let model = Model::from_documents(vec![
            (
                PathBuf::from(DOC),
                format!("### `##{SLUG}` **The statement.**\n"),
            ),
            (
                PathBuf::from(SRC),
                format!("/// see `{COMPONENT}#{SLUG}`\nfn f() {{}}\n"),
            ),
        ]);
        assert_eq!(model.documents().len(), 2);
        let dump = model.canonical();
        assert!(dump.contains(&format!("{DOC}\t1\tslug-def\t{SLUG}")));
        assert!(dump.contains(&format!("{SRC}\t1\tslug-ref\t{COMPONENT}#{SLUG}")));
    }

    /// A dumped path reference reads back as the text it was written as.
    ///
    /// The dump is what a person greps the tree with, so a row whose value cannot be found in
    /// the file the row names is worse than no row: it reports a reference that appears
    /// nowhere. Concatenating the two halves in the wrong order produced exactly that, on
    /// every qualified reference in the repository at once, and nothing here read the value.
    #[test]
    fn a_qualified_path_reference_dumps_as_it_is_written() {
        let written = format!("`{COMPONENT}@{DOC}`");
        let model = Model::from_documents(vec![(
            PathBuf::from(SRC),
            format!("/// see {written}\nfn f() {{}}\n"),
        )]);
        let dump = model.canonical();
        let value = format!("{COMPONENT}@{DOC}");
        assert!(
            dump.contains(&format!("{SRC}\t1\tpath-ref\t{value}")),
            "dumped as written: {dump}"
        );
        // The property the row exists for, asserted rather than assumed: the value is a
        // substring of the line it was read from.
        assert!(written.contains(&value), "the value greps in its source");
    }

    /// A span parsing as no reference keeps its raw text and is a different kind.
    #[test]
    fn an_unsupported_path_shape_dumps_as_the_span_it_was() {
        let model = Model::from_documents(vec![(
            PathBuf::from(SRC),
            format!("/// see `{DOC}`\nfn f() {{}}\n"),
        )]);
        let dump = model.canonical();
        assert!(
            dump.contains(&format!("{SRC}\t1\tunsupported-path\t{DOC}")),
            "{dump}"
        );
        assert!(!dump.contains("@"), "no separator was invented: {dump}");
    }

    #[test]
    fn a_heading_is_read_from_prose_and_never_from_code() {
        let model = Model::from_documents(vec![
            (PathBuf::from("a.md"), "# A heading\n".to_string()),
            (
                PathBuf::from("a.rs"),
                "/// # A heading\nfn f() {}\n".to_string(),
            ),
            (
                PathBuf::from("b.rs"),
                "fn f() {\n    let s = \"# A heading\";\n}\n".to_string(),
            ),
        ]);
        // Markdown carries it directly, a doc comment's content is markdown and carries it
        // too, and a string literal bound to a name is data whatever it spells.
        let headings: Vec<usize> = model
            .documents()
            .iter()
            .map(|d| {
                d.observations
                    .iter()
                    .filter(|l| matches!(l.what, Observation::Heading { .. }))
                    .count()
            })
            .collect();
        assert_eq!(headings, vec![1, 1, 0]);
    }

    #[test]
    fn a_prose_line_reports_the_file_line_it_came_from() {
        // There are no longer two views of a file to keep in step. There is the file, and
        // the prose pulled out of it, and every line of that prose says where it came from —
        // which is what a finding's line number is built out of.
        let model = Model::from_documents(vec![(
            PathBuf::from("a.rs"),
            "/// one\n// two\nfn f() {}\n".to_string(),
        )]);
        let doc = &model.documents()[0];
        assert_eq!(doc.prose_line(1), Some("one"));
        assert_eq!(doc.prose_line(2), Some("two"));
        assert_eq!(doc.prose_line(3), None, "a line of code carries no prose");
        assert_eq!(doc.prose_lines(), vec![1, 2]);
    }
}
