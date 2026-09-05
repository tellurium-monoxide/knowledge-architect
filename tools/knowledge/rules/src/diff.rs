//! What moved between two releases, filtered to the rules this repository cites.
//!
//! A raw diff between releases is large and mostly irrelevant. What matters is the
//! intersection with our own citations — and the dangerous case is not an edit but a
//! **renumbering**, because every citation of a renumbered rule still verifies while
//! pointing at different text. So a rule is matched by its body, not by its number.

use std::collections::HashMap;
use std::fmt;

use crate::corpus::Corpus;
use crate::number::RuleNumber;

/// What happened to one cited rule between two releases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Change {
    /// The text moved to another number. The worst case: if the old number survives, it now
    /// holds different text and every citation of it reads as verified while pointing
    /// elsewhere.
    Moved {
        rule: RuleNumber,
        to: RuleNumber,
        /// What the old number holds now, or `None` if the number is gone.
        now: Option<String>,
    },
    /// The number is gone and its text did not reappear anywhere identifiable.
    Gone {
        rule: RuleNumber,
        was: String,
        /// Other numbers that carried this exact text in the OLD release. Their existence
        /// is what makes this a deletion rather than a renumbering.
        twins: Vec<RuleNumber>,
        /// Numbers carrying this text in the new release, when there is more than one and
        /// the destination is therefore ambiguous.
        ambiguous: Vec<RuleNumber>,
    },
    /// The number survives and its text was edited.
    Changed {
        rule: RuleNumber,
        was: String,
        now: String,
    },
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Change::Moved { rule, to, now } => write!(
                f,
                "=== MOVED  {rule} -> {to} ===\n  text unchanged; {rule} now holds:\n  {}",
                now.as_deref().unwrap_or("(nothing — the number is gone)")
            ),
            Change::Gone {
                rule,
                was,
                twins,
                ambiguous,
            } => {
                write!(f, "=== GONE  {rule} ===\n  {was}")?;
                if !twins.is_empty() {
                    write!(
                        f,
                        "\n  NOTE: this text is not unique to {rule} — it also sits at {}.\
                         \n  A DELETION, not a renumbering: do not retarget citations at those rules.",
                        join(twins)
                    )?;
                } else if !ambiguous.is_empty() {
                    write!(
                        f,
                        "\n  NOTE: identical text now appears at {}; the destination is\
                         \n  ambiguous, so treat this as a deletion.",
                        join(ambiguous)
                    )?;
                }
                Ok(())
            }
            Change::Changed { rule, was, now } => {
                write!(
                    f,
                    "=== CHANGED  {rule} ===\n  before: {was}\n  after:  {now}"
                )
            }
        }
    }
}

fn join(numbers: &[RuleNumber]) -> String {
    numbers
        .iter()
        .map(RuleNumber::as_str)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Index a release by body text, so a rule can be found by what it says.
fn by_body(corpus: &Corpus) -> HashMap<&str, Vec<&RuleNumber>> {
    let mut out: HashMap<&str, Vec<&RuleNumber>> = HashMap::new();
    for (number, body) in corpus.iter() {
        out.entry(body).or_default().push(number);
    }
    out
}

/// Compare two releases over the rules the repository cites.
///
/// `cited` is supplied by the caller rather than discovered here: which rules this
/// repository cites is a fact about its documents, and this library knows nothing about
/// them. The binary reads it from the document model and passes it in.
pub fn diff(old: &Corpus, new: &Corpus, cited: &[RuleNumber]) -> Vec<Change> {
    let (was, where_now) = (by_body(old), by_body(new));
    let mut out = Vec::new();
    for rule in cited {
        let (Some(a), b) = (old.get(rule), new.get(rule)) else {
            continue; // not in the old release: nothing to have moved
        };
        if Some(a) == b {
            continue;
        }
        // Rule bodies are NOT unique — the pinned text holds byte-identical pairs. Matching
        // a deleted rule's text to the first number that happens to share it would report a
        // DELETION as a renumbering, and a bump would then retarget every citation at an
        // unrelated rule. So a destination counts only where the text identified this rule
        // and no other, in BOTH releases.
        let twins = others(&was, a, rule);
        let dests = if twins.is_empty() {
            others(&where_now, a, rule)
        } else {
            Vec::new()
        };

        out.push(match (dests.as_slice(), b) {
            ([to], _) => Change::Moved {
                rule: rule.clone(),
                to: (*to).clone(),
                now: b.map(str::to_string),
            },
            (_, None) => Change::Gone {
                rule: rule.clone(),
                was: a.to_string(),
                twins: twins.into_iter().cloned().collect(),
                ambiguous: dests.into_iter().cloned().collect(),
            },
            // The Python this replaces attaches a note here for the case where the old text
            // reappears at exactly one other number — which the first arm has already taken,
            // so that note has never been reachable. It is not carried over; if the
            // condition was meant to be different, that is a change to make deliberately.
            (_, Some(now)) => Change::Changed {
                rule: rule.clone(),
                was: a.to_string(),
                now: now.to_string(),
            },
        });
    }
    out
}

fn others<'a>(
    index: &HashMap<&'a str, Vec<&'a RuleNumber>>,
    body: &str,
    except: &RuleNumber,
) -> Vec<&'a RuleNumber> {
    index
        .get(body)
        .map(|v| v.iter().copied().filter(|n| *n != except).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Inputs to a comparison, not claims about content: three numbers the tests move text
    // between.
    const A: &str = "100.1";
    const B: &str = "100.2";
    const C: &str = "100.3";

    fn n(s: &str) -> RuleNumber {
        RuleNumber::parse(s).expect("a rule number")
    }

    fn corpus(rules: &[(&str, &str)]) -> Corpus {
        let mut text = "contents\n".repeat(181);
        for (number, body) in rules {
            text.push_str(&format!("{number} {body}\n"));
        }
        Corpus::parse(&text, 181)
    }

    #[test]
    fn an_untouched_rule_produces_nothing() {
        let c = corpus(&[(A, "Same text.")]);
        assert!(diff(&c, &c, &[n(A)]).is_empty());
    }

    #[test]
    fn an_edited_rule_is_changed() {
        let old = corpus(&[(A, "Before.")]);
        let new = corpus(&[(A, "After.")]);
        assert_eq!(
            diff(&old, &new, &[n(A)]),
            vec![Change::Changed {
                rule: n(A),
                was: "Before.".into(),
                now: "After.".into(),
            }]
        );
    }

    #[test]
    fn text_that_reappears_under_one_other_number_is_a_move() {
        let old = corpus(&[(A, "Travelling text.")]);
        let new = corpus(&[(B, "Travelling text.")]);
        assert_eq!(
            diff(&old, &new, &[n(A)]),
            vec![Change::Moved {
                rule: n(A),
                to: n(B),
                now: None,
            }]
        );
    }

    #[test]
    fn a_move_that_leaves_the_number_reused_reports_what_it_now_holds() {
        // The worst case: citations of the old number still verify, against other text.
        let old = corpus(&[(A, "Travelling text.")]);
        let new = corpus(&[(A, "Something else."), (B, "Travelling text.")]);
        assert_eq!(
            diff(&old, &new, &[n(A)]),
            vec![Change::Moved {
                rule: n(A),
                to: n(B),
                now: Some("Something else.".into()),
            }]
        );
    }

    #[test]
    fn a_deleted_rule_whose_text_was_never_unique_is_gone_not_moved() {
        // Byte-identical twins exist in the real corpus. Reading this as a renumbering
        // would have a bump retarget every citation at an unrelated rule.
        let old = corpus(&[(A, "Shared text."), (B, "Shared text.")]);
        let new = corpus(&[(B, "Shared text.")]);
        let changes = diff(&old, &new, &[n(A)]);
        assert_eq!(
            changes,
            vec![Change::Gone {
                rule: n(A),
                was: "Shared text.".into(),
                twins: vec![n(B)],
                ambiguous: vec![],
            }]
        );
        assert!(changes[0]
            .to_string()
            .contains("A DELETION, not a renumbering"));
    }

    #[test]
    fn a_deleted_rule_with_several_possible_destinations_is_gone_and_says_so() {
        let old = corpus(&[(A, "Copied text.")]);
        let new = corpus(&[(B, "Copied text."), (C, "Copied text.")]);
        let changes = diff(&old, &new, &[n(A)]);
        let Change::Gone { ambiguous, .. } = &changes[0] else {
            panic!("expected a deletion, got {:?}", changes[0]);
        };
        assert_eq!(ambiguous.len(), 2);
        assert!(changes[0].to_string().contains("ambiguous"));
    }

    #[test]
    fn a_rule_absent_from_the_old_release_is_not_reported() {
        let old = corpus(&[(A, "Only here.")]);
        let new = corpus(&[(A, "Only here."), (B, "Brand new.")]);
        assert!(diff(&old, &new, &[n(B)]).is_empty());
    }

    #[test]
    fn only_cited_rules_are_reported() {
        let old = corpus(&[(A, "Before."), (B, "Before too.")]);
        let new = corpus(&[(A, "After."), (B, "After too.")]);
        assert_eq!(diff(&old, &new, &[n(A)]).len(), 1);
    }
}
