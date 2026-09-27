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
    pub observations: Vec<Located>,
    /// How its string literals were read: `Data` only under the checker's own source.
    pub literals: Literals,
}

impl Document {
    pub fn is_markdown(&self) -> bool {
        self.rel.extension().is_some_and(|e| e == "md")
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
    /// Git's live listing of the project, project-relative and unfiltered: every tracked file
    /// plus every untracked file the ignore rules do not cover.
    ///
    /// Kept beside the documents because the survey answers what exists out of it, and asking
    /// git twice in one run would let the two answers disagree. A model assembled in memory
    /// carries none.
    listing: Vec<PathBuf>,
    /// The listing's symlink and gitlink entries, which the walk reads through neither.
    links: Vec<crate::git::Entry>,
    /// The checker's own directories as the summary names them: each relative to the root when
    /// it sits under it, absolute otherwise, empty when the caller passed none.
    checker_sources: Vec<PathBuf>,
}

impl Model {
    /// Read and scan a project, as its own manifest declares it.
    ///
    /// `checker_sources` are the directories of the checker's own source, one per Component
    /// the running binary is built from. A file under one is parsed with its string literals as
    /// data, per `design@knowledge@checker-source-literals-are-data`; every other file reads
    /// them as prose. The binary passes the compile-time locations its libraries export, and a
    /// library caller checking a tree the checker is no part of passes none. The root and the
    /// compiled paths are canonicalised before the prefix test, so a symlinked checkout does
    /// not defeat it; a symlink inside the tree is not followed. A compiled path that resolves
    /// to nothing exempts nothing and is still named, so the summary can say so. **A directory
    /// exempts files only when it sits inside the tree being checked.** A tree that sits inside it
    /// instead, such as a mock project under the checker's own tests, is a foreign project
    /// whose every literal is prose. Only a Rust file is marked `Data`: markdown has no
    /// literals, and `checker_files` counts what the mode changed.
    pub fn build(manifest: &Manifest, checker_sources: &[&Path]) -> std::io::Result<Self> {
        let root = manifest.root();
        let canonical_root = root.canonicalize()?;
        let checkers: Vec<(&Path, Option<PathBuf>)> = checker_sources
            .iter()
            .map(|p| (*p, p.canonicalize().ok()))
            .collect();
        let inside: Vec<PathBuf> = checkers
            .iter()
            .filter_map(|(_, c)| c.clone())
            .filter(|c| c.starts_with(&canonical_root))
            .collect();
        let walk_config = manifest.walk();
        let mut docs = Vec::new();
        let generated = crate::index::generated_paths(manifest);
        // Git is the walk. No `git` on the path and a directory outside a worktree are both
        // errors naming the reason, never an empty listing: a project reported as holding no
        // document is a run that checked nothing and said it was clean.
        let entries = crate::git::entries(root)?;
        let listing: Vec<PathBuf> = entries.iter().map(|e| e.rel.clone()).collect();
        // A symlink or a gitlink is not a document: the walk reads through neither, and
        // `check::tree` names each as what it is.
        let files: Vec<PathBuf> = entries
            .iter()
            .filter(|e| e.kind == crate::git::EntryKind::File)
            .map(|e| e.rel.clone())
            .collect();
        let links: Vec<crate::git::Entry> = entries
            .into_iter()
            .filter(|e| e.kind != crate::git::EntryKind::File)
            .collect();
        for path in walk::live_files(root, walk_config, &files, &generated) {
            let rel_for_error = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let text = match std::fs::read_to_string(&path) {
                Ok(t) => t,
                // **Not silence.** A file the walk cannot read leaves the model AND the
                // inverse assertion, so a citation in it is checked by nothing and the run
                // still passes. One byte of Windows-1252 — a pasted em dash — does it. The
                // document is kept, empty, carrying the reason, so a check reports it.
                Err(e) => {
                    // A path git lists and the working tree does not hold is a tracked file
                    // deleted and not yet staged. It stays in the walk and is reported, rather
                    // than being dropped: dropping it would take a live document out of every
                    // check on the strength of a working-tree state, and the whole point of
                    // reading the listing from git is that nothing does that silently.
                    let trouble = if e.kind() == std::io::ErrorKind::NotFound {
                        "git lists this file and the working tree does not hold it: stage the \
                         deletion, or restore the file"
                            .to_string()
                    } else {
                        format!("this file could not be read as text: {e}")
                    };
                    docs.push(Document {
                        rel: rel_for_error,
                        text: String::new(),
                        parsed: Parsed {
                            trouble: Some(trouble),
                            ..Parsed::default()
                        },
                        observations: Vec::new(),
                        literals: Literals::Prose,
                    });
                    continue;
                }
            };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let is_rust = rel.extension().is_some_and(|e| e == "rs");
            let under = inside
                .iter()
                .any(|c| canonical_root.join(&rel).starts_with(c));
            let literals = if is_rust && under {
                Literals::Data
            } else {
                Literals::Prose
            };
            let parsed = source::parse(&rel, &text, literals);
            let observations: Vec<Located> = scan::scan(&parsed);
            docs.push(Document {
                rel,
                text,
                parsed,
                observations,
                literals,
            });
        }
        // Named even when it does not exist: the summary line is how a binary compiled from
        // a directory that is gone says so, and a missing line is the silent shape.
        let checker_sources = checkers
            .into_iter()
            .map(|(given, canonical)| {
                let c = canonical.unwrap_or_else(|| given.to_path_buf());
                c.strip_prefix(&canonical_root)
                    .map(Path::to_path_buf)
                    .unwrap_or(c)
            })
            .collect();
        Ok(Self {
            root: root.to_path_buf(),
            docs,
            listing,
            links,
            checker_sources,
        })
    }

    /// Git's live listing of the project, project-relative, before any manifest exclusion.
    ///
    /// What the survey answers "does this path exist" out of. Empty for a model assembled in
    /// memory, which has no tree behind it.
    pub fn listing(&self) -> &[PathBuf] {
        &self.listing
    }

    /// The listing's symlink and gitlink entries, each a phase-2 finding.
    pub fn links(&self) -> &[crate::git::Entry] {
        &self.links
    }

    /// The checker's own directories as the summary names them, empty if the caller passed none.
    pub fn checker_sources(&self) -> &[PathBuf] {
        &self.checker_sources
    }

    /// How many walked Rust files sit under the checker's own directory, their literals read
    /// as data.
    ///
    /// Zero in a checkout that holds the tool is the loud failure of
    /// `design@knowledge@checker-source-literals-are-data`: the compiled path and the walked tree
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
        Self::from_documents_under(docs, &[])
    }

    /// The same, with the checker's own directory named as a PROJECT-RELATIVE path.
    ///
    /// `build` takes an absolute directory and canonicalises it against the checkout; there is
    /// no checkout here, so the caller states the directory the way every document in the list
    /// is stated. A Rust file under it reads its string literals as data, per
    /// `design@knowledge@checker-source-literals-are-data`. `commits` needs this because a per-commit
    /// model is assembled from git objects and would otherwise read the tool's own fixtures as
    /// live citations at every commit in the range.
    pub fn from_documents_under(docs: Vec<(PathBuf, String)>, checker: &[&Path]) -> Self {
        let docs = docs
            .into_iter()
            .map(|(rel, text)| {
                let is_rust = rel.extension().is_some_and(|e| e == "rs");
                let literals = if is_rust && checker.iter().any(|dir| rel.starts_with(dir)) {
                    Literals::Data
                } else {
                    Literals::Prose
                };
                let parsed = source::parse(&rel, &text, literals);
                let observations = scan::scan(&parsed);
                Document {
                    rel,
                    text,
                    parsed,
                    observations,
                    literals,
                }
            })
            .collect();
        Self {
            root: PathBuf::new(),
            docs,
            listing: Vec::new(),
            links: Vec::new(),
            checker_sources: checker.iter().map(|p| p.to_path_buf()).collect(),
        }
    }

    /// Add a document the caller could not read, empty and carrying the reason.
    ///
    /// The way `build` keeps an unreadable file: in the model, so that `check::tree` reports
    /// it and the run stops there, rather than absent, which is the silence the walk exists
    /// against. `commits` uses it for a tree entry whose blob is not text.
    pub fn push_unreadable(&mut self, rel: PathBuf, trouble: String) {
        self.docs.push(Document {
            rel,
            text: String::new(),
            parsed: Parsed {
                trouble: Some(trouble),
                ..Parsed::default()
            },
            observations: Vec::new(),
            literals: Literals::Prose,
        });
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
        self.canonical_with(&[])
    }

    /// The same, with each extension's rows merged in: per document and line, an extension's
    /// rows before the core's, as one scanner emitting both would order them.
    pub fn canonical_with(&self, extra: &[DumpRow]) -> String {
        // (document, line, source, order) sorts every row; the source puts an extension's
        // row first on a shared line, and the order keeps each source's own sequence.
        let mut rows: Vec<(usize, u32, u8, usize, String)> = Vec::new();
        for (i, doc) in self.docs.iter().enumerate() {
            let rel = doc.rel.display();
            for (k, Located { line, what }) in doc.observations.iter().enumerate() {
                let (kind, value) = describe(what);
                if kind.is_empty() {
                    continue;
                }
                rows.push((i, *line, 1, k, format!("{rel}\t{line}\t{kind}\t{value}\n")));
            }
        }
        for (k, row) in extra.iter().enumerate() {
            let rel = self.docs[row.doc].rel.display();
            rows.push((
                row.doc,
                row.line,
                0,
                k,
                format!("{rel}\t{}\t{}\t{}\n", row.line, row.kind, row.value),
            ));
        }
        rows.sort_by_key(|r| (r.0, r.1, r.2, r.3));
        rows.into_iter().map(|r| r.4).collect()
    }
}

/// One row an extension adds to the canonical dump: the document by its index in the model,
/// the line, the kind and the value.
pub struct DumpRow {
    pub doc: usize,
    pub line: u32,
    pub kind: &'static str,
    pub value: String,
}

/// `(kind, value)` for the canonical dump. An empty kind is left out of it.
fn describe(what: &Observation) -> (&'static str, String) {
    match what {
        Observation::Heading { level, text } => ("heading", format!("{level} {text}")),
        // The site rides along, so the dump says whether a slug sat where a definition can
        // be — a level-two or level-three heading, a cell — or somewhere the table reports.
        Observation::SlugDef { id, site } => (
            "slug-def",
            match site {
                crate::scan::SlugSite::Heading(level) => format!("{id} heading-{level}"),
                crate::scan::SlugSite::Cell => format!("{id} cell"),
                crate::scan::SlugSite::LineHead => format!("{id} line-head"),
                crate::scan::SlugSite::Inline => format!("{id} inline"),
            },
        ),
        // As written, so a dumped row can be grepped for in the tree it came from.
        Observation::Span(span) => ("span", span.clone()),
        Observation::UnanchoredPath(span) => ("unanchored-path", span.clone()),
        Observation::Retired(crate::scan::RetiredForm::SlugRef(span)) => {
            ("retired-slug-ref", span.clone())
        }
        Observation::Link(target) => ("link", target.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@knowledge@checker-source-literals-are-data`.

    #[test]
    fn a_model_can_be_assembled_without_a_checkout() {
        let model = Model::from_documents(vec![
            (
                PathBuf::from("docs/design/a.md"),
                "### `##a-slug` **The statement.**\n".to_string(),
            ),
            (
                PathBuf::from("src/b.rs"),
                "/// see `design@a-component@a-slug`\nfn f() {}\n".to_string(),
            ),
        ]);
        assert_eq!(model.documents().len(), 2);
        let dump = model.canonical();
        assert!(dump.contains("docs/design/a.md\t1\tslug-def\ta-slug heading-3"));
        assert!(dump.contains("src/b.rs\t1\tspan\tdesign@a-component@a-slug"));
    }

    /// A dumped reference reads back as the text it was written as.
    ///
    /// The dump is what a person greps the tree with, so a row whose value cannot be found in
    /// the file the row names is worse than no row: it reports a reference that appears
    /// nowhere. Concatenating two halves in the wrong order once produced exactly that, on
    /// every qualified reference in the repository at once, and nothing here read the value.
    #[test]
    fn a_reference_dumps_as_it_is_written() {
        let written = "`path@a-component@docs/design/a.md`";
        let model = Model::from_documents(vec![(
            PathBuf::from("src/b.rs"),
            format!("/// see {written}\nfn f() {{}}\n"),
        )]);
        let dump = model.canonical();
        let value = "path@a-component@docs/design/a.md";
        assert!(
            dump.contains(&format!("src/b.rs\t1\tspan\t{value}")),
            "dumped as written: {dump}"
        );
        // The property the row exists for, asserted rather than assumed: the value is a
        // substring of the line it was read from.
        assert!(written.contains(value), "the value greps in its source");
    }

    /// A span parsing as no reference keeps its raw text and is a different kind, and so
    /// does the retired slug form. A bare `R` and digits is no observation at all.
    #[test]
    fn an_unanchored_path_and_a_retired_form_dump_as_their_own_kinds() {
        let model = Model::from_documents(vec![(
            PathBuf::from("src/b.rs"),
            "/// see `docs/design/a.md` and `a-component#a-slug` and R15\nfn f() {}\n".to_string(),
        )]);
        let dump = model.canonical();
        assert!(
            dump.contains("src/b.rs\t1\tunanchored-path\tdocs/design/a.md\n"),
            "{dump}"
        );
        assert!(
            dump.contains("src/b.rs\t1\tretired-slug-ref\ta-component#a-slug\n"),
            "{dump}"
        );
        assert!(!dump.contains("R15"), "{dump}");
        assert!(
            !dump.contains("@a-slug"),
            "no separator was invented: {dump}"
        );
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

    /// The claim: a stated checker directory reads the Rust literals under it as data, and
    /// leaves every other file's alone.
    ///
    /// `commits` assembles a model per commit out of git objects, so it cannot canonicalise a
    /// compiled path against a checkout the way `build` does. Without the directory the tool's
    /// own fixtures are read as live citations at every commit in the range, and every commit
    /// is then reported as a tree that fails.
    #[test]
    fn a_stated_checker_directory_reads_its_rust_literals_as_data() {
        // Unbound on purpose: a literal BOUND to a name is data in either mode, per
        // `design@knowledge@grammars-not-prefixes`, so a fixture written that way would pass
        // whichever mode the model chose.
        let source = "fn f() {\n    report(\"see `design@a-component@a-slug`\");\n}\n";
        let docs = vec![
            (
                PathBuf::from("tools/knowledge/src/a.rs"),
                source.to_string(),
            ),
            (PathBuf::from("crates/engine/src/b.rs"), source.to_string()),
        ];
        let told = Model::from_documents_under(docs.clone(), &[Path::new("tools/knowledge")]);
        let dump = told.canonical();
        assert!(
            !dump.contains("tools/knowledge/src/a.rs"),
            "the tool's own literal is data: {dump}"
        );
        assert!(
            dump.contains("crates/engine/src/b.rs\t2\tspan\tdesign@a-component@a-slug"),
            "every other file's is a claim: {dump}"
        );
        assert_eq!(told.checker_files(), 1);
        assert_eq!(told.checker_sources(), [PathBuf::from("tools/knowledge")]);

        // Told nothing, every literal is prose — which is what `from_documents` gives.
        let untold = Model::from_documents(docs);
        assert!(
            untold.canonical().contains("tools/knowledge/src/a.rs"),
            "{}",
            untold.canonical()
        );
        assert_eq!(untold.checker_files(), 0);
    }

    /// The claim: every stated checker directory exempts the Rust literals under it, since the
    /// tool's source spans one Component per library a binary is built from.
    #[test]
    fn each_stated_checker_directory_reads_its_rust_literals_as_data() {
        let source = "fn f() {\n    report(\"see `design@a-component@a-slug`\");\n}\n";
        let docs = vec![
            (PathBuf::from("tools/core/src/a.rs"), source.to_string()),
            (
                PathBuf::from("tools/extension/src/b.rs"),
                source.to_string(),
            ),
            (PathBuf::from("crates/engine/src/c.rs"), source.to_string()),
        ];
        let told = Model::from_documents_under(
            docs,
            &[Path::new("tools/core"), Path::new("tools/extension")],
        );
        let dump = told.canonical();
        assert!(!dump.contains("tools/core/src/a.rs"), "{dump}");
        assert!(!dump.contains("tools/extension/src/b.rs"), "{dump}");
        assert!(dump.contains("crates/engine/src/c.rs"), "{dump}");
        assert_eq!(told.checker_files(), 2);
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
