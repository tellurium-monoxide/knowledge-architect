//! The maintenance crate's main that the setup skill shows, compiled against this workspace's
//! crates, so a change to an interface it uses fails the build. It is compiled here because xtask
//! depends on the core and on the gates library, and agent-skills, a dependency of the core,
//! cannot. The snippet is included rather than made the example's root because it reads its
//! project's checkout variable, `<PROJECT>_CHECKOUT`, and this root reads this repository's.

// This target links no library of its package, so it reads the variable itself.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

include!("../../../crates/agent-skills/snippets/xtask-main.rs");
