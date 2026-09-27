//! thaum's rules extension of the knowledge core: every rule quote verified against a pinned
//! corpus, per `design@thaum@knowledge-is-a-generic-core`.
//!
//! `rules_extension::RulesExtension` implements the core's `Extension` over the checks that read
//! the Comprehensive Rules — `citations`, `regime`, `uncovered`, `changes` and `corpus` — and
//! generates the rule index. `rules_scan` is its own scan of a document, over the core's parse,
//! per `design@knowledge@an-extension-builds-its-own-model`.

pub mod check;
pub mod quote;
pub mod rule_index;
pub mod rules_extension;
pub mod rules_scan;

pub use quote::{Quote, QuoteKind};

/// The directory of the Component this library belongs to: the parent of its own crate
/// directory, evaluated at build time.
///
/// A binary that registers this extension hands it to the walk with the core's, so this
/// library's own fixtures are read as data, per
/// `design@knowledge@checker-source-literals-are-data`.
pub fn component_dir() -> std::path::PathBuf {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_dir.parent().unwrap_or(crate_dir).to_path_buf()
}
