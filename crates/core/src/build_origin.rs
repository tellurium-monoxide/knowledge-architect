//! Whether the running binary was built from the tree it checks.
//!
//! Every command judges a tree with the binary the cargo alias builds from the checkout, and
//! one built from another checkout is refused, per `design@core@a-foreign-build-is-refused`. Two checkouts that share one target
//! directory break that: the last build writes the one binary both run, and cargo does not
//! rebuild it for the other checkout, whose own package is still fresh. That checkout then runs
//! another checkout's code, and its tool fixtures, exempted under the other checkout's paths, are
//! read as citations.
//!
//! What can be known at run time is where each library the binary links was compiled, and
//! under which package name. A tree that holds the same package at the same place relative to
//! itself, but not at the compiled directory, is a checkout of the same tool: the binary was
//! built from a different one, and the run is refused.
//!
//! A tree that holds no copy of a library is left alone, which covers a mock project under the
//! tool's own directory and a project that consumes the library as a published crate.

use std::path::{Component, Path, PathBuf};

/// One library a binary links: where it was compiled, and its package name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Library {
    /// The library's own crate directory, its `CARGO_MANIFEST_DIR` at build time.
    pub crate_dir: PathBuf,
    /// Its package name, as its `Cargo.toml` declares it.
    pub package: &'static str,
}

/// This library, as the core's own build saw it.
pub fn this_library() -> Library {
    Library {
        crate_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        package: env!("CARGO_PKG_NAME"),
    }
}

/// The first library the tree holds a second copy of, with the copy's directory.
///
/// Every trailing run of the compiled directory's components, one component long or more, is tried
/// under the root, and a directory there whose `Cargo.toml` declares the same package name, and
/// which is not the compiled directory itself, is the copy. A trailing run is where a checkout
/// of the same repository holds the crate, wherever that checkout sits, a worktree nested in
/// the tree included; trying each one needs no knowledge of where the repository's root was.
/// Both sides are canonicalised before they are compared, so a tree reached through a symlink
/// is not its own copy.
///
/// Two limits follow from reading only names and places. A checkout that moved the crate to
/// another relative path is not seen. A tree holding an unrelated crate of the same name at the
/// same relative path is taken for a copy.
pub(crate) fn foreign_copy(root: &Path, libraries: &[Library]) -> Option<(Library, PathBuf)> {
    let canonical = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let root = canonical(root);
    for library in libraries {
        let compiled = canonical(&library.crate_dir);
        let parts: Vec<&std::ffi::OsStr> = compiled
            .components()
            .filter_map(|c| match c {
                Component::Normal(p) => Some(p),
                _ => None,
            })
            .collect();
        // Not the empty run: it would make the root's own `Cargo.toml` a candidate, and refuse
        // every project whose root package shares a name with a linked crate.
        for start in (0..parts.len()).rev() {
            let candidate: PathBuf = parts[start..]
                .iter()
                .fold(root.clone(), |dir, part| dir.join(part));
            if declares(&candidate.join("Cargo.toml"), library.package)
                && canonical(&candidate) != compiled
            {
                return Some((library.clone(), candidate));
            }
        }
    }
    None
}

/// Refuse to run when the tree holds another copy of a library the binary links.
///
/// `packages` is what the refusal tells the user to clean: the binary's own package and every
/// library's, so that the next run rebuilds all of them from this checkout.
pub fn refuse_a_foreign_build(
    root: &Path,
    libraries: &[Library],
    packages: &[&str],
) -> Result<(), String> {
    let Some((library, copy)) = foreign_copy(root, libraries) else {
        return Ok(());
    };
    // The profile this binary was built in, since `cargo clean -p` clears one profile only.
    let profile = if cfg!(debug_assertions) {
        ""
    } else {
        " --release"
    };
    let clean: String = packages.iter().map(|p| format!(" -p {p}")).collect();
    Err(format!(
        "this binary was built from another checkout: its `{}` was compiled at {}, and the tree \
         being checked holds that package at {}\n       a target directory shared between two \
         checkouts does this; rebuild from this one with `cargo clean{profile}{clean}`",
        library.package,
        library.crate_dir.display(),
        copy.display(),
    ))
}

/// Whether a `Cargo.toml` declares this package name.
fn declares(manifest: &Path, package: &str) -> bool {
    let Ok(text) = std::fs::read_to_string(manifest) else {
        return false;
    };
    let Ok(value) = text.parse::<toml::Table>() else {
        return false;
    };
    value
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        == Some(package)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh directory under the system's temporary directory, removed when dropped.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(tag: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "knowledge-build-origin-{tag}-{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("a scratch directory");
            Scratch(dir)
        }

        fn crate_at(&self, rel: &str, package: &str) -> PathBuf {
            let dir = self.0.join(rel);
            std::fs::create_dir_all(&dir).expect("a crate directory");
            std::fs::write(
                dir.join("Cargo.toml"),
                format!("[package]\nname = \"{package}\"\n"),
            )
            .expect("a crate manifest");
            dir
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn library(crate_dir: PathBuf) -> Library {
        Library {
            crate_dir,
            package: "a-tool",
        }
    }

    #[test]
    fn a_second_checkout_holding_the_same_package_is_a_foreign_copy() {
        let scratch = Scratch::new("second-checkout");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        let copy = scratch.crate_at("two/tools/tool/lib", "a-tool");
        let found = foreign_copy(&scratch.0.join("two"), &[library(built)]);
        assert_eq!(
            found.map(|(_, dir)| dir.canonicalize().expect("the copy")),
            Some(copy.canonicalize().expect("the copy"))
        );
    }

    #[test]
    fn a_library_inside_the_tree_a_tree_inside_it_and_an_unrelated_tree_are_left_alone() {
        let scratch = Scratch::new("left-alone");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        // The ordinary case: the tree holds the compiled directory.
        assert_eq!(
            foreign_copy(&scratch.0.join("one"), &[library(built.clone())]),
            None
        );
        // A mock project under the tool's own directory holds no copy.
        let mock = scratch.0.join("one/tools/tool/lib/tests/projects/mock");
        std::fs::create_dir_all(&mock).expect("a mock");
        assert_eq!(foreign_copy(&mock, &[library(built.clone())]), None);
        // A project consuming the library from elsewhere holds no copy either.
        let other = scratch.0.join("elsewhere");
        std::fs::create_dir_all(&other).expect("a project");
        assert_eq!(foreign_copy(&other, &[library(built.clone())]), None);
        // A crate at the same place under another package name is someone else's.
        scratch.crate_at("three/tools/tool/lib", "another-tool");
        assert_eq!(
            foreign_copy(&scratch.0.join("three"), &[library(built)]),
            None
        );
    }

    #[test]
    fn a_checkout_nested_in_the_tree_a_deep_crate_and_a_crate_at_the_root_are_found() {
        let scratch = Scratch::new("nested");
        // A worktree nested in the tree: its compiled directory sits under the root, and the
        // root holds the crate at the same trailing run.
        let tree = scratch.0.join("tree");
        let nested = scratch.crate_at("tree/.worktrees/inner/tools/tool/lib", "a-tool");
        let copy = scratch.crate_at("tree/tools/tool/lib", "a-tool");
        let found = foreign_copy(&tree, &[library(nested)]);
        assert_eq!(
            found.map(|(_, dir)| dir.canonicalize().expect("the copy")),
            Some(copy.canonicalize().expect("the copy"))
        );
        // Five components deep.
        let deep = scratch.crate_at("one/a/b/c/d/lib", "a-tool");
        scratch.crate_at("two/a/b/c/d/lib", "a-tool");
        assert!(foreign_copy(&scratch.0.join("two"), &[library(deep)]).is_some());
        // A project whose root package shares the name is not taken for a copy: the empty
        // trailing run is not tried.
        let elsewhere = scratch.crate_at("built/a-tool", "a-tool");
        scratch.crate_at("unrelated", "a-tool");
        assert_eq!(
            foreign_copy(&scratch.0.join("unrelated"), &[library(elsewhere)]),
            None
        );
    }

    #[test]
    fn a_crate_compiled_through_a_symlink_is_not_a_copy_of_itself() {
        // `cargo build --manifest-path` through a symlinked path compiles that path in.
        let scratch = Scratch::new("compiled-symlink");
        scratch.crate_at("one/tools/tool/lib", "a-tool");
        let link = scratch.0.join("link");
        std::os::unix::fs::symlink(scratch.0.join("one"), &link).expect("a symlink");
        let through = library(link.join("tools/tool/lib"));
        assert_eq!(foreign_copy(&scratch.0.join("one"), &[through]), None);
    }

    #[test]
    fn a_tree_reached_through_a_symlink_is_not_its_own_copy() {
        let scratch = Scratch::new("symlink");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        let link = scratch.0.join("link");
        std::os::unix::fs::symlink(scratch.0.join("one"), &link).expect("a symlink");
        assert_eq!(foreign_copy(&link, &[library(built)]), None);
        // A symlink inside the tree that resolves to the compiled directory is the same
        // directory, not a copy.
        let real = scratch.crate_at("tree/real/tool/lib", "a-tool");
        std::os::unix::fs::symlink(
            scratch.0.join("tree/real/tool"),
            scratch.0.join("tree/tool"),
        )
        .expect("a symlink");
        assert_eq!(
            foreign_copy(&scratch.0.join("tree"), &[library(real)]),
            None
        );
    }

    #[test]
    fn the_refusal_names_both_directories_and_every_package_to_clean() {
        let scratch = Scratch::new("refusal");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        scratch.crate_at("two/tools/tool/lib", "a-tool");
        let why = refuse_a_foreign_build(
            &scratch.0.join("two"),
            &[library(built.clone())],
            &["a-binary", "a-tool"],
        )
        .expect_err("a refusal");
        assert!(why.contains(&built.display().to_string()), "{why}");
        assert!(why.contains("two"), "{why}");
        let profile = if cfg!(debug_assertions) {
            ""
        } else {
            " --release"
        };
        assert!(
            why.contains(&format!("cargo clean{profile} -p a-binary -p a-tool")),
            "{why}"
        );
        assert_eq!(
            refuse_a_foreign_build(&scratch.0.join("one"), &[library(built)], &["a-tool"]),
            Ok(())
        );
    }
}
