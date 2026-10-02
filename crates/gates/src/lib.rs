//! A project's merge gates as one command: every check a branch must pass before it merges, run
//! in order, each one's complete output kept in a log, a distilled extract of what failed, and
//! one verdict taken from the children's exit codes alone.
//!
//! The library is meant for a project's own maintenance binary, conventionally a crate named
//! `xtask` run through a cargo alias. The binary holds the project's gate list, usually built from
//! [`rust_project`], and hands it to [`run`] with the flags it parsed:
//!
//! ```no_run
//! use std::process::ExitCode;
//!
//! use clap::{Parser, Subcommand};
//! use knowledge_architect_gates::{project_root, run, rust_project, Checker, GatesArgs};
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[command(subcommand)]
//!     command: Command,
//! }
//!
//! #[derive(Subcommand)]
//! enum Command {
//!     /// Run every gate a branch must pass before it merges.
//!     Gates(GatesArgs),
//! }
//!
//! fn main() -> ExitCode {
//!     let Command::Gates(args) = Cli::parse().command;
//!     let cwd = std::env::current_dir().expect("a working directory");
//!     let root = project_root(&cwd, "knowledge-architect.toml").expect("inside the project");
//!     let checker = Checker { package: "xtask", prefix: &["klarch"] };
//!     run(&root, &rust_project(checker, "origin/main"), &args)
//! }
//! ```
//!
//! [`process`] is the spawn helper the gates use, for a project's other commands.

mod gates;
pub mod process;

pub use gates::{run, rust_project, Checker, Distiller, Gate, GatesArgs};
pub use process::project_root;
