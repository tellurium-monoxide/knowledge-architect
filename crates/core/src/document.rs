//! The parse of a document, as an extension reads it, per
//! `design@core@an-extension-builds-its-own-model`.
//!
//! The core's model holds nothing for an extension. An extension reads each [`Document`]'s
//! parse through these types and scans it again for its own subject.
//!
//! [`Document`]: crate::Document

pub use crate::scan::{Located, Observation, RetiredForm, SlugSite};
pub use crate::source::{Frontmatter, Literals, Parsed, Prose, Scope, ScopeKind};

/// The markdown grammar.
pub mod md {
    pub use crate::source::md::parse;
}

/// The Rust grammar.
pub mod rs {
    pub use crate::source::rs::parse;
}
