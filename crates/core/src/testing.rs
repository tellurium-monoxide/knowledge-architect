//! What an extension's tests need to run the core over a project, as the binary does.
//!
//! A test configures its extensions with [`crate::extension::configure`] and gathers the inputs
//! with [`crate::cli::Gathered`]. It calls [`foundation`] for phases 1 to 3. Once that returns
//! `Ok` and each extension is prepared, it calls [`run_with`] for the last phase.

pub use crate::check::{foundation, run_with, Phase, Report, Stop, Structure, CHECKS};
