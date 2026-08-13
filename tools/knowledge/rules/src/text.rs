//! Normalising rule text so a quote can be compared to it.
//!
//! Every comparison between a quote and the rules text runs through [`norm`], on both
//! sides. Typography is the reason: a document may carry a curly apostrophe where the rules
//! text carries a straight one, or wrap a quote across lines where the rules text has one
//! line, and neither difference is a citation defect.

use unicode_normalization::UnicodeNormalization;

/// Whitespace as Python's `\s` matches it in string patterns.
///
/// It is the Unicode `White_Space` property plus the four separator controls U+001C–U+001F,
/// which Python includes and the property does not. No file in the tree contains one of the
/// four, checked over every walked file and the rules text — so the clause changes no
/// current answer, and it is here because the port is meant to preserve behaviour and not
/// only today's output.
fn is_space(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

/// Fold typographic variants, collapse whitespace runs to one space, and trim.
///
/// The five folds are the ones the rules text and this repository's documents actually
/// differ by: both curly single quotes to `'`, both curly double quotes to `"`, and a
/// non-breaking space to a space. Nothing else is folded — a fold is a place where two
/// different texts start comparing equal, so the set stays as small as the evidence.
pub fn norm(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut pending_space = false;
    for c in s.nfc() {
        let c = match c {
            '\u{2019}' | '\u{2018}' => '\'',
            '\u{201c}' | '\u{201d}' => '"',
            '\u{00a0}' => ' ',
            c => c,
        };
        if is_space(c) {
            pending_space = !out.is_empty();
        } else {
            if pending_space {
                out.push(' ');
                pending_space = false;
            }
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_the_five_typographic_variants() {
        assert_eq!(norm("it\u{2019}s \u{201c}x\u{201d}"), "it's \"x\"");
        assert_eq!(norm("a\u{00a0}b"), "a b");
        assert_eq!(norm("\u{2018}y\u{2019}"), "'y'");
    }

    #[test]
    fn collapses_runs_and_trims_both_ends() {
        assert_eq!(norm("  a \t\n  b  "), "a b");
        assert_eq!(norm("\n\n"), "");
        assert_eq!(norm(""), "");
    }

    #[test]
    fn treats_the_separator_controls_as_whitespace() {
        // Python's `\s` matches these and the Unicode property does not. Nothing in the
        // tree carries one; the case exists so the behaviour is defined rather than
        // accidental.
        assert_eq!(norm("a\u{1c}b"), "a b");
        assert_eq!(norm("a\u{1f}\u{1e}b"), "a b");
    }

    #[test]
    fn leaves_an_em_dash_and_an_ellipsis_alone() {
        // Elision is written with a real ellipsis and rule text carries em dashes. Folding
        // either would make two different texts compare equal.
        assert_eq!(norm("a — b … c"), "a — b … c");
    }
}
