//! Turning a file into the two things every check reads: **prose regions** and **scopes**.
//!
//! **One markdown engine, two sources.** A Rust doc comment's content is markdown, so the
//! markdown analysis is written once and run over a markdown file and over each Rust comment
//! alike. What the Rust grammar contributes is not a second set of conventions — it is the
//! answer to *which byte ranges are prose at all*, which no line-oriented scanner can give.
//!
//! **Nothing is stripped.** The implementation this replaces removed one comment leader per
//! line and scanned the result, which meant code and prose were told apart by a prefix. Three
//! recorded defects came from that single choice: a formatted return type opening a line with
//! an angle bracket read as a blockquote, a float literal read as a rule number, and a test
//! fixture's string literal read as live content. Here the grammar says which is which, so
//! none of them can arise.
//!
//! A scope is where a citation's quote must live. It is the innermost enclosing section in a
//! document and the innermost enclosing item in Rust, and it never searches outward: a quote a
//! thousand lines above the claim is one the reader never sees.

pub mod md;
pub mod rs;

use std::path::Path;

/// A markdown file's opening frontmatter block: its keys in declaration order, or why the
/// block was refused.
///
/// The accepted subset is a block opened and closed by a line holding only `---`, at the very
/// top of the file, holding `key: value` lines with scalar values and nothing else. No
/// nesting, no lists, no quoting rules, and no key written twice.
///
/// **The block is prose, entire.** Its lines are marked as a fence so that a value cannot be
/// read as document structure — a heading, a slug definition, a navigation link — and that is
/// the whole of what the marking buys. A reference in a value is a reference, and a `CR:`
/// marker in one claims its rule with nowhere in the block to put the quote, so a marker does
/// not belong in metadata and is reported where it stands.
pub type Frontmatter = Result<Vec<(String, String)>, String>;

/// How a Rust file's string literals are read.
///
/// Decided by the caller from where the file sits. The checker's own source is the one place
/// whose every literal is a fixture, per `design@core@checker-source-literals-are-data`;
/// everywhere else the grammar decides per literal, and `rs::BINDINGS` says how.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Literals {
    /// A string literal bound to a name is data; every other one is prose.
    #[default]
    Prose,
    /// No string literal is prose. Comments are read as in every other file.
    Data,
}

/// A run of prose pulled out of a file, with the line it came from kept for every line of it.
///
/// For a markdown file there is exactly one, covering the whole text. For a Rust file there is
/// one per contiguous comment run, carrying the comment's content with its markers already
/// removed by the grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prose {
    /// Markdown. In a Rust file this is a comment's content, not its source line.
    pub text: String,
    /// For each line of `text`, the one-based file line it came from.
    ///
    /// A comment run is not contiguous in the file once a blank line or an attribute sits
    /// inside it, and a block comment's content does not start at its own line, so the map is
    /// stored rather than derived from a starting offset.
    pub lines: Vec<u32>,
    /// Whether this run may carry a decision's anchor and a document's structure.
    ///
    /// Markdown only. A slug definition inside a doc comment would move a decision's home into
    /// a source file, which `design@core@a-slug-belongs-to-a-component` places in a component's design
    /// document.
    pub structural: bool,
    /// Byte ranges in `text` that are inline code spans.
    ///
    /// A rule number, a slug or a path inside one is **data being displayed** — a sort key, a
    /// parser input, a citation being reported as wrong — rather than a claim. It is the same
    /// judgement a fenced block and a Rust string literal get, and it is what leaves the
    /// quote-free class needing no marker of its own.
    pub code: Vec<(usize, usize)>,
    /// Byte offset of each line of `text`, built once on first use.
    #[doc(hidden)]
    pub line_starts: std::sync::OnceLock<Vec<usize>>,
}

impl Prose {
    /// The file line a zero-based line of `text` came from.
    ///
    /// Out of range answers with the last known line rather than panicking: a caller derives
    /// the index from a byte offset into `text`, and an offset at the very end of the text is
    /// one line past the last.
    pub fn file_line(&self, line0: usize) -> u32 {
        match self.lines.get(line0) {
            Some(n) => *n,
            None => self.lines.last().copied().unwrap_or(1),
        }
    }

    /// Whether a byte offset in `text` falls inside an inline code span.
    pub fn is_code(&self, offset: usize) -> bool {
        self.code.iter().any(|&(a, b)| a <= offset && offset < b)
    }

    /// The file line a byte offset into `text` falls on.
    ///
    /// **Binary search over the line starts, not a count from the beginning.** Counting made
    /// the analysis quadratic in file size, because it runs once per heading and twice per
    /// fenced block: measured at 21 ms over 61 kB, 284 ms over 244 kB and 1.1 s over 488 kB,
    /// a clean fourfold per doubling. The largest live document is 100 kB today, so it was a
    /// scaling hazard rather than present pain, and one generated table would have found it.
    pub fn file_line_at(&self, offset: usize) -> u32 {
        self.file_line(self.line_index(offset))
    }

    /// The zero-based line of `text` a byte offset falls on.
    fn line_index(&self, offset: usize) -> usize {
        let offset = offset.min(self.text.len());
        let starts = self.line_starts.get_or_init(|| {
            let mut v = vec![0usize];
            v.extend(
                self.text
                    .bytes()
                    .enumerate()
                    .filter(|(_, b)| *b == b'\n')
                    .map(|(i, _)| i + 1),
            );
            v
        });
        starts.partition_point(|&s| s <= offset).saturating_sub(1)
    }
}

/// What a scope is, which decides where a discharging quote may sit inside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    /// A markdown section, at its heading level. A quote anywhere inside it discharges.
    Section(u8),
    /// A Rust item. A quote anywhere inside it discharges, doc comment or body: the point of
    /// the scope is proximity to the claim, and a claim made in a function's body is as
    /// entitled to its quote there as one made in the doc comment above it.
    Item,
    /// Everything before the first heading, or outside every item.
    Preamble,
}

/// A region of a file in which a rule quote discharges the claims made in it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub kind: ScopeKind,
    /// The heading text, or the item's name. Empty for a preamble.
    pub name: String,
    /// One-based file lines, both ends inclusive.
    pub first: u32,
    pub last: u32,
}

impl Scope {
    pub fn holds(&self, line: u32) -> bool {
        self.first <= line && line <= self.last
    }

    /// How deeply nested this scope is, so the innermost one containing a line can be picked.
    ///
    /// A deeper heading level is a narrower scope; an item is narrower than any preamble it
    /// sits in. Two scopes at the same depth never overlap.
    pub fn depth(&self) -> u8 {
        match self.kind {
            ScopeKind::Preamble => 0,
            ScopeKind::Section(level) => level,
            ScopeKind::Item => u8::MAX,
        }
    }
}

/// A parsed file: its prose, and the scopes a citation in it can be judged against.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parsed {
    pub prose: Vec<Prose>,
    pub scopes: Vec<Scope>,
    /// File lines that are code rather than prose in a markdown sense: a fenced block, and in
    /// Rust everything the grammar did not hand over as a comment.
    ///
    /// A rule number here is **data being displayed** rather than a citation, which is the
    /// same judgement a string literal and an inline code span get.
    pub fenced: Vec<u32>,
    /// Identifiers written in code, with the line each sits on.
    ///
    /// **The one thing prose cannot reach.** The identifier form of a rule marker exists
    /// because a name cannot hold punctuation, so it lives in a name and nowhere else — and a
    /// scanner that reads only prose is blind to exactly the shape the convention was written
    /// for. Empty for markdown, which has no names.
    pub names: Vec<(u32, String)>,
    /// Lines the author commented OUT, in markdown's own way: an HTML comment.
    ///
    /// Nothing in one is live. Parking a decision by commenting its section out is ordinary,
    /// and it left the slug defined and the anchor pointing at a section no reader can see.
    pub inert: Vec<u32>,
    /// The frontmatter block at the very top of a markdown file, where there is one.
    ///
    /// `None` where the file opens with no block at all — the common case, and a leading
    /// thematic break is not one. `Some(Err)` where a block is opened and closed and holds a
    /// line the accepted subset does not: a block opened and closed by a line holding only
    /// `---`, holding `key: value` lines with scalar values and nothing else. **The subset is
    /// refused loudly outside itself**, per `design@core@a-failed-parse-is-loud`, because a
    /// frontmatter line nobody can parse is metadata nobody checks.
    ///
    /// The block's lines are prose. Marking them as a fence keeps a value out of the
    /// document's structure and nothing more; see [`Frontmatter`].
    pub frontmatter: Option<Frontmatter>,
    /// Why this parse cannot be trusted, where it cannot.
    ///
    /// **A parse that fails must be loud.** Silently, it removes every citation in the file
    /// from the walk and the run still reports success — which is indistinguishable from a
    /// clean file and is the failure this tool exists to prevent. It is also the guard the
    /// premortem named for a grammar that changes under an upgrade.
    pub trouble: Option<String>,
}

impl Parsed {
    /// The innermost scope holding a line.
    pub fn scope_at(&self, line: u32) -> Option<&Scope> {
        self.scopes
            .iter()
            .filter(|s| s.holds(line))
            .max_by_key(|s| s.depth())
    }
}

/// Parse a file by its suffix.
///
/// A suffix the walk does not cover yields nothing rather than a guess. `walk::LIVE_SUFFIXES`
/// is what decides which those are, and `check::uncovered` asserts that a file outside it may
/// not name a rule — so an empty parse here is never a silent gap.
pub fn parse(path: &Path, text: &str, literals: Literals) -> Parsed {
    match path.extension().and_then(|e| e.to_str()) {
        Some("md") => md::parse(text),
        Some("rs") => rs::parse(text, literals),
        _ => Parsed::default(),
    }
}
