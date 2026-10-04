//! The checker of knowledge-architect, as a library: the binary `klarch` is built on it, and so
//! is a binary that adds a project's own checks to it.
//!
//! The checker reads a project's documents once, through git's listing, into one model. Every
//! check is then a pure function over that model: it touches no file and spawns no process.
//! The project's `knowledge-architect.toml` declares what is checked, and nothing about a
//! project is compiled in. The decisions behind the design, with their arguments, are in the
//! repository, <https://github.com/tellurium-monoxide/knowledge-architect>, at
//! `path@core@docs/design.md`.
//!
//! # Who uses this library
//!
//! A project that needs no check of its own runs the `klarch` binary. A Rust project that pins the
//! checker in its own workspace builds that binary itself, as the example below with no
//! extension. The library is otherwise for a project with a subject of its own, such as quotes that must
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
//! | [`cli`] | the `main` of a binary: the commands, running one, finding the project, refusing a build from another checkout, gathering a run's inputs |
//! | [`extension`] | writing an extension: the two traits, what the core hands them, and what they return |
//! | [`document`] | reading a document's parse: prose regions, scopes, code spans, observations |
//! | [`testing`] | testing an extension over a mock project, as the binary would run it |
//!
//! # A binary that registers an extension
//!
//! The binary depends on clap 4 with its `derive` feature, the major version this crate uses,
//! because [`cli::Command`] derives clap's `Subcommand`. It flattens [`cli::Command`] into its
//! own command enum, and hands a parsed command to [`cli::run`] with its extensions:
//!
//! ```no_run
//! use std::path::Path;
//! use std::process::ExitCode;
//!
//! use clap::{Parser, Subcommand};
//! use knowledge_architect::cli;
//! use knowledge_architect::extension::Extension;
//!
//! /// What the binary is for: `--help` opens with this.
//! #[derive(Parser)]
//! #[command(version)]
//! struct Cli {
//!     #[command(subcommand)]
//!     command: Commands,
//! }
//!
//! // No doc comment here: clap prints a subcommand enum's doc comment as the binary's own
//! // description, over the one `Cli` gives.
//! #[derive(Subcommand)]
//! enum Commands {
//!     #[command(flatten)]
//!     Core(cli::Command),
//!     // Each command of the extension is one more variant here.
//! }
//!
//! fn main() -> ExitCode {
//!     let parsed = Cli::parse();
//!     // The directory of the binary's own crate: its string literals are read as data.
//!     let own = Path::new(env!("CARGO_MANIFEST_DIR"));
//!     let mut extensions: Vec<Box<dyn Extension>> = Vec::new(); // Box::new(TheExtension), …
//!     let outcome = cli::locate().and_then(|manifest| {
//!         cli::refuse_a_foreign_build(
//!             manifest.root(),
//!             // Every library the binary links, the binary's own crate included, and one
//!             // entry per crate of the extension's library.
//!             &[
//!                 cli::this_library(),
//!                 cli::Library { crate_dir: own.to_path_buf(), package: env!("CARGO_PKG_NAME") },
//!             ],
//!             // The packages of the project's own workspace that the binary links.
//!             &[env!("CARGO_PKG_NAME")],
//!         )?;
//!         // Before any command, the extension's own included.
//!         cli::refuse_another_version(&manifest, &[own])?;
//!         match parsed.command {
//!             Commands::Core(command) => cli::run(command, &manifest, &[own], &mut extensions),
//!             // Each command of the extension is one more arm here.
//!         }
//!     });
//!     outcome.unwrap_or_else(|e| {
//!         eprintln!("error: {e}");
//!         ExitCode::from(2)
//!     })
//! }
//! ```
//!
//! [`cli::run`] returns `Err` when the command could not run, and the binary exits 2 with it.
//! Otherwise it returns the command's verdict: exit 0 when the command ran and found nothing,
//! exit 1 when it ran and found something to report.
//!
//! [`cli::refuse_a_foreign_build`] refuses a binary built from another checkout of the project,
//! because that binary would judge this tree with the other checkout's code. A binary tests the
//! refusal in its own suite, over its own crates.
//!
//! [`cli::refuse_another_version`] refuses to run over a project whose manifest pins another
//! version of the checker in `[project] checker-version`. A mock project inside the extension's
//! crate may say `checker-version = "fixture"`, which this call accepts because the crate's
//! directory is among the directories it is given.
//!
//! # Writing an extension
//!
//! An extension implements [`extension::Extension`]. Three of its methods describe it:
//! `tables` names the top-level manifest tables it reads, `checks` names its checks, and `dump`
//! returns its rows for the `model` command. The core calls the others at fixed points of a
//! run:
//!
//! 1. [`extension::configure`] calls `resolve` once per manifest, before the walk. The
//!    extension reads the tables it claims with [`Manifest::table`], and returns its complaints,
//!    the paths it declares and the files it generates. A table it claims that the manifest
//!    does not hold, and a table no extension claims, are reported in phase 1.
//! 2. Once the first three phases have found nothing, `prepare` reads what its checks need for
//!    one tree, an [`extension::Tree`]: the working tree, or one commit's tree under `commits`.
//!    It returns an [`extension::Prepared`].
//! 3. `Prepared::check` runs its checks in the last phase. It reads the model and the
//!    [`extension::Inputs`] the core gathered, and returns an [`extension::ExtensionReport`].
//!    `check_message` judges a commit message, and `generated` returns the files the
//!    extension generates.
//!
//! A check is a pure function: what it needs from the filesystem or the network is read in
//! `prepare`.
//!
//! [`Manifest::table`] returns a `toml::Value`, so an extension that reads its table depends on
//! the toml crate at the major version this crate uses, 1.
//!
//! # Reading a document
//!
//! The core's model holds nothing for an extension. An extension reads each [`Document`] of
//! [`Model::documents`] and scans its parse again for its own subject:
//!
//! - [`Document::parsed`] is a [`document::Parsed`]: the prose regions as
//!   [`document::Prose`], the code spans inside each, the scopes (a markdown section or a Rust
//!   item), the fenced lines and the Rust names.
//! - [`Document::observations`] are what the core's scanner recorded, each a
//!   [`document::Located`] [`document::Observation`]: headings, slug definitions, reference
//!   candidates and links.
//! - [`document::md::parse`] and [`document::rs::parse`] parse a text in a unit test, without
//!   a project.
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
//! git's listing is what the walk reads, so a mock's files are walked as soon as they exist, and
//! an ignore rule must not cover them. The project's own manifest excludes the mock's directory
//! with a `[walk] exclude` row, or the mock's planted defects are reported as the project's own.
//!
//! # Compatibility
//!
//! Under the crate's versioning policy, a breaking change to this API is major. While the
//! version is 0.x, a major change bumps 0.MINOR, so a break never ships in a patch.
//!
//! - [`cli::Command`], [`extension::Inputs`], [`extension::ExtensionReport`] and
//!   [`extension::Resolution`] are `#[non_exhaustive]`. A new command, input or field in them
//!   is not a breaking change.
//! - Every other enum is exhaustive on purpose. A new variant is a compile error at an
//!   extension's exhaustive match, which then has to say what it does with the new case.
//! - A hook added to [`extension::Extension`] or [`extension::Prepared`] has a default body only
//!   when doing nothing is correct for an extension that does not know it. A hook whose absence
//!   would make a verdict wrong has none, and adding it is a breaking change.

// The compatibility rules above restate `design@core@ne-minimal` and
// `design@core@trait-defaults`.

// A `pub` item no consumer can reach is refused, so an item is public only through the
// re-exports below and the role modules, per `design@core@api-facade`.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

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
