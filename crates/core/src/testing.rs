//! What an extension's tests need to run the core over a project, as the binary does.
//!
//! A test configures its extensions with [`crate::extension::configure`], gathers the inputs
//! with [`crate::cli::Gathered`], then calls [`foundation`] for phases 1 to 3 or [`run_with`]
//! for a whole run.

pub use crate::check::{foundation, run_with, Phase, Report, Stop, Structure, CHECKS};
