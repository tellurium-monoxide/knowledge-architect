//! This repository's own documents: the walk, the model, and every check over it.
//!
//! The shape is one model built once, then checks that are pure functions over it. A check
//! never touches the filesystem, spawns a process, or knows how the walk works — which is
//! what makes the single walk a property of the design rather than of anyone's care, and
//! what lets a check be tested against a model built in memory.

pub mod check;
pub mod cli;
pub mod entity;
pub mod finding;
pub mod git;
pub mod index;
pub mod manifest;
pub mod model;
pub mod quote;
pub mod records;
pub mod scan;
pub mod source;
pub mod survey;
pub mod walk;

pub use finding::Finding;
pub use manifest::Manifest;
pub use model::{Document, Model};
pub use quote::{Quote, QuoteKind};
pub use scan::{Located, MarkerForm, Observation, RetiredForm, SlugSite};
