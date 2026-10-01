//! The checker of knowledge-architect, as a library: the binary `klarch` is built on it, and so
//! is a binary that adds a project's own checks to it.
//!
//! The checker reads a project's documents once, through git's listing, into one model. Every
//! check is then a pure function over that model: it touches no file and spawns no process.
//! The project's `knowledge-architect.toml` declares what is checked, and nothing about a
//! project is compiled in. The decisions behind the design, with their arguments, are in the
//! repository, at `path@core@docs/design.md`.
//!
//! # Who uses this library
//!
//! A project that needs no check of its own runs the `klarch` binary and never names this
//! library. The library is for a project with a subject of its own, such as quotes that must
//! match a pinned corpus. Such a project writes an **extension**: a set of checks, the manifest
//! tables they read, and the files they generate. It compiles the extension into a binary of its
//! own, which runs every command of `klarch` unchanged and the extension's checks beside the
//! core's. There is no loading at run time.
//!
//! The public items are grouped by what a consumer is doing:
//!
//! | module | for |
//! | --- | --- |
//! | the crate root | the nouns every consumer meets: [`Manifest`], [`Model`], [`Document`], [`Finding`], [`MANIFEST_NAME`], [`component_dir`] |
//! | [`cli`] | the `main` of a binary: the commands, running one, finding the project, refusing a build from another checkout |
//! | [`extension`] | writing an extension: the two traits, what the core hands them, and what they return |
//! | [`document`] | reading a document's parse: prose regions, scopes, code spans, observations |
//! | [`testing`] | testing an extension over a mock project, as the binary would run it |
//!
//! # A binary that registers an extension
//!
//! The binary flattens the core's [`cli::Command`] into its own command enum, and hands a
//! parsed command to [`cli::run`] with its extensions:
//!
//! ```no_run
//! use std::path::Path;
//! use std::process::ExitCode;
//!
//! use clap::{Parser, Subcommand};
//! use knowledge_architect::cli;
//! use knowledge_architect::extension::Extension;
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[command(subcommand)]
//!     command: Commands,
//! }
//!
//! #[derive(Subcommand)]
//! enum Commands {
//!     #[command(flatten)]
//!     Core(cli::Command),
//!     // The extension's own commands go here.
//! }
//!
//! fn main() -> ExitCode {
//!     let Commands::Core(command) = Cli::parse().command;
//!     // The directory of the binary's own crate: its string literals are read as data.
//!     let own = Path::new(env!("CARGO_MANIFEST_DIR"));
//!     let mut extensions: Vec<Box<dyn Extension>> = Vec::new(); // Box::new(TheExtension), …
//!     let outcome = cli::locate().and_then(|manifest| {
//!         cli::refuse_a_foreign_build(
//!             manifest.root(),
//!             &[cli::this_library()],
//!             &[env!("CARGO_PKG_NAME")],
//!         )?;
//!         cli::run(command, &manifest, &[own], &mut extensions)
//!     });
//!     outcome.unwrap_or_else(|e| {
//!         eprintln!("error: {e}");
//!         ExitCode::from(2)
//!     })
//! }
//! ```
//!
//! [`cli::run`] returns `Err` when the command could not run, and the binary exits 2 with it.
//! Exit 0 and exit 1 are the command's verdict: in order, and not in order.
//!
//! # Writing an extension
//!
//! An extension implements [`extension::Extension`]. The core calls it at fixed points of a run:
//!
//! 1. [`extension::configure`] calls `resolve` once per manifest, before the walk. The
//!    extension reads the tables it claims with [`Manifest::table`], and returns its complaints,
//!    the paths it declares and the files it generates.
//! 2. Once the first three phases found nothing, `prepare` reads what its checks need for one
//!    tree, a [`extension::Tree`]: the working tree, or one commit's tree under `commits`. It
//!    returns an [`extension::Prepared`].
//! 3. `Prepared::check` runs its checks in the last phase. It reads the model and the
//!    [`extension::Inputs`] the core gathered, and returns an [`extension::ExtensionReport`].
//!    `check_message` judges a commit message, and `generated` returns the files it generates.
//!
//! A check is a pure function: what it needs from the filesystem or the network is read in
//! `prepare`. An extension builds its own model from the core's parse. It reads each
//! [`Document`]'s [`document::Parsed`] and scans it again for its subject.
//!
//! # Testing an extension
//!
//! A test runs the core over a mock project as the binary does, with the public items alone:
//!
//! ```no_run
//! # use knowledge_architect::extension::Extension;
//! # fn the_extension() -> Box<dyn Extension> { unimplemented!() }
//! use knowledge_architect::extension::{configure, Purpose, Tree};
//! use knowledge_architect::testing::{foundation, run_with};
//! use knowledge_architect::{cli, Manifest, Model};
//!
//! let mut manifest = Manifest::load("tests/projects/a-mock".as_ref()).unwrap();
//! let mut extensions = vec![the_extension()];
//! configure(&mut manifest, &mut extensions);
//! let model = Model::build(&manifest, &[]).unwrap();
//! let gathered = cli::Gathered::over(&manifest, &model).unwrap();
//! let inputs = gathered.inputs();
//! foundation(&model, &manifest, &inputs).expect("phases 1 to 3 find nothing");
//! let prepared = extensions[0]
//!     .prepare(&manifest, &model, Tree::Checkout(manifest.root()), Purpose::Check)
//!     .unwrap();
//! let report = run_with(&model, &manifest, &inputs, &[(extensions[0].checks(), prepared.as_ref())]);
//! assert!(report.findings.is_empty());
//! ```
//!
//! A mock project is a directory inside the repository with its own `knowledge-architect.toml`.
//! git's listing is what the walk reads, so its files are walked as soon as they exist.
//!
//! # Compatibility
//!
//! A breaking change to this API is a major change of the crate's versioning policy. While the
//! version is 0.x, it bumps 0.MINOR.
//!
//! - [`cli::Command`], [`extension::Inputs`], [`extension::ExtensionReport`] and
//!   [`extension::Resolution`] are `#[non_exhaustive]`. A new command, input or field in them
//!   is not a breaking change.
//! - Every other enum is exhaustive on purpose. A new variant is a compile error at an
//!   extension's exhaustive match, which then has to say what it does with the new case.
//! - A hook added to [`extension::Extension`] or [`extension::Prepared`] has a default body only
//!   when doing nothing is correct for an extension that does not know it. A hook whose absence
//!   would make a verdict wrong has none, and adding it is a breaking change.

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
