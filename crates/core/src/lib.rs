//! A project's documents: the walk, the model, and the core's checks over it.
//!
//! The shape is one model built once, then checks that are pure functions over it. A check
//! never touches the filesystem, spawns a process, or knows how the walk works — which is
//! what makes the single walk a property of the design rather than of anyone's care, and
//! what lets a check be tested against a model built in memory.

pub mod agents;
pub mod build_origin;
pub mod check;
pub mod cli;
pub mod entity;
pub mod extension;
pub mod finding;
pub mod git;
pub mod index;
pub mod manifest;
pub mod model;
pub mod records;
pub mod scan;
pub mod source;
pub mod survey;
pub mod walk;

// The core run against whole mock projects. A unit-test module rather than an integration
// test, because its assertions read the walk, the survey and the entity table, which are not
// public.
#[cfg(test)]
mod mock_projects;

pub use finding::Finding;
pub use manifest::Manifest;
pub use model::{Document, Model};
pub use scan::{Located, Observation, RetiredForm, SlugSite};

/// The directory of the Component this library belongs to: its own crate directory, evaluated at
/// build time.
///
/// A binary built from this library hands it to the walk with the directories of its other
/// libraries, so the tool's own fixtures are read as data, per
/// `design@core@checker-source-literals-are-data`. Once the library is consumed as a
/// published crate, the directory is outside the tree being checked and exempts nothing.
pub fn component_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
