//! The Comprehensive Rules corpus: parse, pin, archive, resolve, diff, watch.
//!
//! This library's truth is the pinned rules text and the releases around it. It knows
//! nothing about this repository's own documents — that is `documentation`, which depends on
//! this and not the other way round. Where a job needs both, the fact about our documents is
//! passed in: [`diff::diff`] takes the cited rules as an argument rather than going to look
//! for them.
//!
//! Nothing here discovers the repository root. A caller supplies it as a [`release::Tree`],
//! so this library never depends on where its own source file happens to sit.

pub mod corpus;
pub mod diff;
pub mod integrity;
pub mod number;
pub mod release;
pub mod text;
pub mod watch;

pub use corpus::Corpus;
pub use number::RuleNumber;
pub use release::Tree;
pub use text::norm;

#[cfg(test)]
pub(crate) mod testing {
    use std::path::{Path, PathBuf};

    /// The checkout this source sits in.
    ///
    /// Tests may use `CARGO_MANIFEST_DIR` where the binary may not: it is baked in at
    /// compile time, which is wrong for a binary that could be copied elsewhere and exactly
    /// right for a test, which only ever runs against the tree it was compiled from.
    pub fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .expect("rules/ sits three levels below the repository root")
            .to_path_buf()
    }

    /// This checkout's corpus, with its layout spelled out here rather than in the library.
    ///
    /// A test about this repository may know where this repository keeps its rules. The
    /// library may not — that is the project's manifest to state, and `documentation` is
    /// what reads it.
    pub fn this_tree() -> crate::release::Tree {
        let root = repo_root();
        let dir = root.join("docs/rules");
        crate::release::Tree::new(
            &root,
            dir.join("MagicCompRules.txt"),
            dir.join("VERSION"),
            dir.join("past"),
            dir.join("past/MANIFEST.tsv"),
        )
    }
}
