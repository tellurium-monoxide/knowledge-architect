//! A project's documents: the walk, the model, and the core's checks over it.
//!
//! The shape is one model built once, then checks that are pure functions over it. A check
//! never touches the filesystem, spawns a process, or knows how the walk works — which is
//! what makes the single walk a property of the design rather than of anyone's care, and
//! what lets a check be tested against a model built in memory.

// A `pub` item no consumer can reach is refused, so an item is public only through the
// re-exports below and the role modules.
#![warn(unreachable_pub)]

// The implementation modules. They are private: what a consumer uses is re-exported below
// and by the four role modules, so a file can move without changing a public path.
mod agents;
mod build_origin;
mod check;
mod entity;
mod finding;
mod git;
mod index;
mod manifest;
mod model;
mod records;
mod scan;
mod source;
mod survey;
mod walk;

// The role modules: one per kind of consumer.
pub mod cli;
pub mod document;
pub mod extension;
pub mod testing;

// The core run against whole mock projects. A unit-test module rather than an integration
// test, because its assertions read the walk, the survey and the entity table, which are not
// public.
#[cfg(test)]
mod mock_projects;

pub use finding::Finding;
pub use manifest::{Manifest, MANIFEST_NAME};
pub use model::{Document, Model};

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
