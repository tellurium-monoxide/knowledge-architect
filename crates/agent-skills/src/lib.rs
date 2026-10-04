//! The files `knowledge-architect` installs into a project's agent configuration: skills,
//! subagent definitions and the primer.
//!
//! Each entry is the path the file is installed at, relative to the project root, and its text.
//! The list is generated from this crate's content/ directory by its build script.

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// Every file the installer writes, as its install path and its text, in path order.
pub static FILES: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/files.rs"));

/// This crate's directory, compiled in, so a binary that links it can refuse a build made from
/// another checkout, per `design@core@a-foreign-build-is-refused`.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, for the same refusal.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");
