//! A mock source file, wrong on purpose.

/// A name citing a rule owes that rule's whole body. This one quotes part of it.
/// Per CR:100.1: *"The first mock rule says exactly this…"*
fn cr_100_1_a_name_without_the_whole_body() {}

/// A name whose rule is quoted entire, which is what the rule asks for.
/// Per CR:100.4: *"Short."*
fn cr_100_4_a_name_with_the_whole_body() {}

/// A claim in a doc comment with its quote beside it, per CR:100.2:
/// *"The second mock rule says something completely different."*
fn quoted_in_scope() {
    // A claim in the body of the same item, discharged by the quote above.
    let _ = "CR:100.2 is claimed here too";
}

/// A fixture BOUND to a name is data: no citation, no heading, no slug.
const FIXTURE: &str = "100.1 and `##planted-anchor` and CR:100.1";

/// A formatted return type opens a line with an angle bracket and is not a blockquote.
fn long_signature()
-> Result<(usize, usize), std::fmt::Error>
{
    let _ = 100.4_f64 * 2.0;
    Err(std::fmt::Error)
}
