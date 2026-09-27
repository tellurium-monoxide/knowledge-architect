//! This repository's own documents: the walk, the model, and every check over it.
//!
//! The shape is one model built once, then checks that are pure functions over it. A check
//! never touches the filesystem, spawns a process, or knows how the walk works — which is
//! what makes the single walk a property of the design rather than of anyone's care, and
//! what lets a check be tested against a model built in memory.

pub mod check;
pub mod cli;
pub mod entity;
pub mod extension;
pub mod finding;
pub mod git;
pub mod index;
pub mod manifest;
pub mod model;
pub mod quote;
pub mod records;
pub mod rules_extension;
pub mod rules_scan;
pub mod scan;
pub mod source;
pub mod survey;
pub mod walk;

pub use finding::Finding;
pub use manifest::Manifest;
pub use model::{Document, Model};
pub use quote::{Quote, QuoteKind};
pub use scan::{Located, Observation, RetiredForm, SlugSite};

/// The directory of the Component this library belongs to: the parent of its own crate
/// directory, evaluated at build time.
///
/// A binary built from this library hands it to the walk with the directories of its other
/// libraries, so the tool's own fixtures are read as data, per
/// `design@knowledge@checker-source-literals-are-data`. Once the library is consumed as a
/// published crate, the directory is outside the tree being checked and exempts nothing.
pub fn component_dir() -> std::path::PathBuf {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_dir.parent().unwrap_or(crate_dir).to_path_buf()
}
