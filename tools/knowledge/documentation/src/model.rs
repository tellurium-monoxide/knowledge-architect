//! The model: every live document, read once, scanned once.

use std::path::{Path, PathBuf};

use crate::manifest::{Manifest, Walk};
use crate::scan::{self, Located, Observation};
use crate::walk;

/// One live document and everything that was observed in it.
#[derive(Clone, Debug)]
pub struct Document {
    /// Repository-relative, which is how every finding and every generated file names it.
    pub rel: PathBuf,
    /// The file as it is on disk.
    pub text: String,
    /// The same file with one comment leader removed per line, and the same line count.
    pub stripped: String,
    /// The release this file's quotes verify against, where it opts out of the vendored one.
    pub pin: Option<String>,
    pub observations: Vec<Located>,
}

impl Document {
    pub fn is_markdown(&self) -> bool {
        self.rel.extension().is_some_and(|e| e == "md")
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
}

impl Model {
    /// Read and scan a project, as its own manifest declares it.
    pub fn build(manifest: &Manifest) -> std::io::Result<Self> {
        let root = manifest.root();
        let walk_config = manifest.walk();
        let mut docs = Vec::new();
        for path in walk::live_files(root, walk_config)? {
            let text = match std::fs::read_to_string(&path) {
                Ok(t) => t,
                // Not every walked suffix guarantees UTF-8. A file that cannot be read as
                // text cites nothing, which is the same answer the walk being replaced gives.
                Err(_) => continue,
            };
            let stripped = if walk::has_comments(&path, walk_config) {
                walk::strip_leaders(&text)
            } else {
                text.clone()
            };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let is_markdown = rel.extension().is_some_and(|e| e == "md");
            let observations: Vec<Located> = scan::scan(&text, &stripped, is_markdown);
            docs.push(Document {
                rel,
                pin: scan::pin(&text),
                text,
                stripped,
                observations,
            });
        }
        Ok(Self {
            root: root.to_path_buf(),
            docs,
        })
    }

    /// A model assembled from text rather than from a checkout, for a test.
    ///
    /// `walk` decides only which suffixes carry comment leaders, so a test that does not
    /// care can pass the default.
    pub fn from_documents(docs: Vec<(PathBuf, String)>, walk: &Walk) -> Self {
        let docs = docs
            .into_iter()
            .map(|(rel, text)| {
                let stripped = if walk::has_comments(&rel, walk) {
                    walk::strip_leaders(&text)
                } else {
                    text.clone()
                };
                let is_markdown = rel.extension().is_some_and(|e| e == "md");
                let observations = scan::scan(&text, &stripped, is_markdown);
                Document {
                    rel,
                    pin: scan::pin(&text),
                    text,
                    stripped,
                    observations,
                }
            })
            .collect();
        Self {
            root: PathBuf::new(),
            docs,
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
                Mention => "marker-mention",
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
        Observation::PathRef { component, path } => match component {
            None => ("old-path-ref", format!("{path}")),
            Some(cp) => ("path-ref", format!("{path}{cp}")),
        },
        Observation::InterpRef(n) => ("interp-ref", n.to_string()),
        // Fencing is a property of a line that a check reads, not something to compare.
        Observation::Fenced => ("", String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Walk;

    // Interpolated, never spelled out: this tool's checks walk their own source, and a slug
    // or a path written literally here becomes a real anchor or a real dangling reference.
    const SLUG: &str = "a-slug";
    const COMPONENT: &str = "a-component";
    const DOC: &str = "docs/design/a.md";
    const SRC: &str = "src/b.rs";

    #[test]
    fn a_model_can_be_assembled_without_a_checkout() {
        let model = Model::from_documents(
            vec![
                (
                    PathBuf::from(DOC),
                    format!("`##{SLUG}` **The statement.**\n"),
                ),
                (
                    PathBuf::from(SRC),
                    format!("/// see `{COMPONENT}#{SLUG}`\nfn f() {{}}\n"),
                ),
            ],
            &Walk::sample(),
        );
        assert_eq!(model.documents().len(), 2);
        let dump = model.canonical();
        assert!(dump.contains(&format!("{DOC}\t1\tslug-def\t{SLUG}")));
        assert!(dump.contains(&format!("{SRC}\t1\tslug-ref\t{COMPONENT}#{SLUG}")));
    }

    #[test]
    fn a_comment_leader_is_stripped_only_where_the_suffix_says_so() {
        let model = Model::from_documents(
            vec![
                (PathBuf::from("a.md"), "# A heading\n".to_string()),
                (PathBuf::from("a.py"), "# A heading\n".to_string()),
            ],
            &Walk::sample(),
        );
        // In Markdown the `#` opens a heading; in Python it opens a comment, and once the
        // leader is gone there is no heading left.
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
        assert_eq!(headings, vec![1, 0]);
    }

    #[test]
    fn the_two_views_have_the_same_number_of_lines() {
        let model = Model::from_documents(
            vec![(
                PathBuf::from("a.rs"),
                "/// one\n// two\nfn f() {}\n".to_string(),
            )],
            &Walk::sample(),
        );
        let doc = &model.documents()[0];
        assert_eq!(doc.text.lines().count(), doc.stripped.lines().count());
    }
}
