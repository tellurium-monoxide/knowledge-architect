//! The citation regime, judged against scopes rather than against lines.
//!
//! Verification — does this quote match the rule — is `citations`. This module asks the
//! questions a quote's correctness cannot answer: is there a quote at all, is it near the
//! claim, does the number exist, and is the marker the right one for what the sentence does.
//!
//! **A quote discharges a claim only from the innermost scope, within
//! [`MAX_DISTANCE`] lines above it.** The unit is the level-three subsection in a document and
//! the item in Rust, and there is no outward search. Sessions reach files by grep and partial
//! read, so a quote a thousand lines above the claim is one the reader never sees — and a
//! reader who cannot see the rule text cannot tell a right citation from a wrong one, which is
//! the whole failure this tool exists to prevent.
//!
//! **A rule still under migration is DEFERRED, never disabled.** Its findings are counted and
//! printed on every run; all a deferral changes is whether the run fails. The set of rules is
//! compiled in, so a project can only say which of them it has not reached — never invent one
//! it meets.

use std::collections::BTreeSet;

use rules::{norm, RuleNumber};

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::{Document, Model};
use crate::scan::{MarkerForm, Observation};
use crate::source::ScopeKind;

use super::citations::{clip, Release};

/// Below this a surviving fragment matches too easily to be evidence.
///
/// Raised from twelve, where a fragment under the floor was COUNTED and never checked — a
/// silent false negative inside the guarantee. Measured over this repository when the bound was
/// chosen: 1 fragment under 12 characters, 8 under 20, 17 under 30. Thirty is about five words,
/// enough to be distinctive without reaching a shape anyone writes on purpose.
pub const MIN_FRAGMENT: usize = 30;

/// How far above a claim its quote may sit, in file lines.
///
/// Measured over this repository when the bound was chosen: innermost markdown sections have a
/// median of 19 lines and a 90th percentile of 42, so the bound binds rarely. When it does, the
/// repair is to quote the rule again — a second claim owes its own quote, and the one-home rule
/// governs decisions rather than rule text.
pub const MAX_DISTANCE: u32 = 60;

/// The rules of the regime that this module enforces.
///
/// Named so a manifest can defer one by name. The set is compiled in for the reason
/// `##components-carry-the-same-documents` gives about the document set: a project free to
/// declare its own would be conformant with whatever it declared.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    /// Every marked number names a rule the applicable release holds.
    NumberResolves,
    /// A prose marker owes a verified quote of its rule, in scope and in range.
    QuoteInScope,
    /// The mention form is retired: a number claiming content takes the prose form and a
    /// quote, and a number that is data goes in a code span, a fence or a string literal.
    MentionRetired,
    /// The identifier form belongs in a name. Written in prose, the prose form fits.
    IdentifierInProse,
    /// A name carrying the identifier form owes the rule's WHOLE body, with no elision.
    IdentifierFullQuote,
    /// Every omission inside a quote carries an elision mark, at the front where the quote
    /// does not begin at the rule's first word and at the end where it does not reach its last.
    OmissionMarked,
    /// A surviving fragment is long enough to be evidence.
    FragmentLongEnough,
    /// A quote of a parent rule's whole body does not stand for what its subrules say.
    ParentRuleIsNotItsSubrules,
    /// A quotation written in the rule-quote form that no marker claims.
    ///
    /// The form is `*"…"*`, and it declares *this is rule text*. Written with no marker to bind
    /// it, it is verified against nothing — and the missing-marker lint, which would otherwise
    /// catch the bare number beside it, is silenced when that number sits in a code span. So
    /// the shape `Rule `x` states *"…"*` produced no output of any kind.
    QuoteHasNoMarker,
}

impl Rule {
    pub const NAMED: [(&'static str, Rule); 9] = [
        ("number-resolves", Rule::NumberResolves),
        ("quote-in-scope", Rule::QuoteInScope),
        ("mention-retired", Rule::MentionRetired),
        ("identifier-in-prose", Rule::IdentifierInProse),
        ("identifier-full-quote", Rule::IdentifierFullQuote),
        ("omission-marked", Rule::OmissionMarked),
        ("fragment-long-enough", Rule::FragmentLongEnough),
        (
            "parent-rule-is-not-its-subrules",
            Rule::ParentRuleIsNotItsSubrules,
        ),
        ("quote-has-no-marker", Rule::QuoteHasNoMarker),
    ];

    pub fn name(self) -> &'static str {
        Self::NAMED
            .iter()
            .find(|(_, r)| *r == self)
            .map(|(n, _)| *n)
            .expect("every rule is named")
    }

    pub fn parse(name: &str) -> Option<Rule> {
        Self::NAMED
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, r)| *r)
    }
}

/// One finding, and the rule it came from, so a deferral can hold it back.
pub struct Judged {
    pub rule: Rule,
    pub finding: Finding,
}

/// What a run of the regime counted.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counts {
    /// Findings held back by a deferral, per rule.
    pub backlog: Vec<(Rule, usize)>,
    /// Deferrals whose backlog is zero, which have outlived their work.
    pub retired: Vec<Rule>,
    pub claims: usize,
}

/// Every verified quote in a document: the rule it is offered as, and the line it sits on.
fn verified(doc: &Document, release: &Release) -> Vec<(RuleNumber, u32, String)> {
    super::citations::quotes(doc)
        .into_iter()
        .filter(|q| {
            release
                .rules
                .get(&q.rule)
                .is_some_and(|body| body.contains(&norm(&q.body.replace(['*', '…'], ""))))
                || super::citations::fragments(&q.body).0.iter().all(|f| {
                    release
                        .rules
                        .get(&q.rule)
                        .is_some_and(|body| body.contains(&norm(f)))
                })
        })
        .map(|q| (q.rule, q.line, q.body))
        .collect()
}

/// Judge one quote's completeness against the rule it is offered as.
///
/// **The failure this exists for is an unmarked truncation.** A quote may stop before the rule
/// does, and containment cannot see the missing tail — so a rule read up to the clause that
/// answered the question, whose continuation reverses it, verifies and tells the next reader
/// nothing. Measured over this repository when the rule was written: of 241 quotes bindable to
/// their body, 84 dropped the tail with no mark and 86 were middle slices with neither end
/// marked.
///
/// It does not stop a writer quoting only the clause they need. It stops them doing it
/// invisibly.
fn completeness(doc: &Document, release: &Release, quote: &crate::quote::Quote) -> Vec<Judged> {
    let mut out = Vec::new();
    let Some(body) = release.rules.get(&quote.rule).map(norm) else {
        return out;
    };
    let text = quote.body.replace('*', "");
    let whole = norm(&text.replace('…', ""));
    // A quote of the rule entire owes no mark and no floor: there is nothing omitted, and
    // nothing more of the rule to keep. 241 of 3 162 rules have a body under the floor.
    if whole == body {
        return out;
    }
    let pieces: Vec<String> = text
        .split('…')
        .map(norm)
        .filter(|p| !p.is_empty())
        .collect();
    let mut at = 0usize;
    let (mut first, mut last) = (None, 0usize);
    for piece in &pieces {
        if piece.chars().count() < MIN_FRAGMENT {
            out.push(Judged {
                rule: Rule::FragmentLongEnough,
                finding: Finding::at(
                    &doc.rel,
                    quote.line,
                    format!(
                        "a fragment quoted as {} is {} characters, under the floor of {MIN_FRAGMENT}: {piece}",
                        quote.rule,
                        piece.chars().count()
                    ),
                    "keep more of the rule; a fragment this short matches too easily to be \
                     evidence, and the whole body always passes",
                ),
            });
            return out;
        }
        let Some(found) = body[at..].find(piece.as_str()).map(|i| at + i) else {
            // Not in the rule AFTER the previous piece. Either it is not in the rule at all —
            // `citations` reports that — or it is, earlier, which means the quote reads the
            // rule out of order. Containment cannot tell those apart, so it verifies both.
            if body.contains(piece.as_str()) {
                out.push(Judged {
                    rule: Rule::OmissionMarked,
                    finding: Finding::at(
                        &doc.rel,
                        quote.line,
                        format!(
                            "the quote of {} takes the rule out of order: {piece}",
                            quote.rule
                        ),
                        "every fragment is verbatim and the sequence is not the rule's; quote \
                         it in the order it is written, or the reader is told the reverse",
                    ),
                });
            }
            return out;
        };
        first.get_or_insert(found);
        at = found + piece.len();
        last = at;
    }
    let opens = first == Some(0);
    let closes = last == body.len();
    let marked_front = text.trim_start().starts_with('…');
    let marked_back = text.trim_end().ends_with('…');
    if !opens && !marked_front {
        out.push(Judged {
            rule: Rule::OmissionMarked,
            finding: Finding::at(
                &doc.rel,
                quote.line,
                format!(
                    "the quote of {} drops the rule's opening with no mark",
                    quote.rule
                ),
                "open the quote with an elision mark, so a reader knows the rule says more \
                 before this",
            ),
        });
    }
    if !closes && !marked_back {
        out.push(Judged {
            rule: Rule::OmissionMarked,
            finding: Finding::at(
                &doc.rel,
                quote.line,
                format!(
                    "the quote of {} drops the rule's tail with no mark",
                    quote.rule
                ),
                "close the quote with an elision mark; an unmarked truncation is the shape \
                 that verifies and still misleads",
            ),
        });
    }
    out
}

/// Judge one document against every rule of the regime.
pub fn check(doc: &Document, release: &Release) -> (Vec<Judged>, usize) {
    let mut out = Vec::new();
    let quotes = verified(doc, release);
    let mut claims = 0;

    for span in doc.unclaimed_quotes() {
        // **The corpus decides.** The emphasised form quotes this project's own documents as
        // often as it quotes a rule, so a span alone declares nothing — but a span whose text
        // is IN THE RELEASE is rule text by definition, and rule text with no marker is
        // verified against nothing. That test costs a substring search and has no judgement
        // in it, which is why it replaces the guess it stands in for.
        if !release.whole.contains(&norm(&span.1)) {
            continue;
        }
        out.push(Judged {
            rule: Rule::QuoteHasNoMarker,
            finding: Finding::at(
                &doc.rel,
                span.0,
                format!(
                    "a quotation is written as rule text and no marker claims it: {}",
                    clip(&span.1, 60)
                ),
                "put `CR:` and the rule number in the clause that introduces it, or write it \
                 as ordinary quoted prose",
            ),
        });
    }
    for quote in super::citations::quotes(doc) {
        out.extend(completeness(doc, release, &quote));
        // A parent rule's body says what it says; its subrules say the rest. A quote of the
        // parent entire passes by equality and can be offered for a claim that belongs to a
        // subrule, which is the one hole the whole-body exception opens.
        let whole = release.rules.get(&quote.rule).map(norm);
        let text = norm(&quote.body.replace('*', ""));
        if whole.as_deref() == Some(text.as_str()) && release.rules.has_subrules(&quote.rule) {
            out.push(Judged {
                rule: Rule::ParentRuleIsNotItsSubrules,
                finding: Finding::at(
                    &doc.rel,
                    quote.line,
                    format!("{} is quoted whole and has subrules of its own", quote.rule),
                    "a parent rule does not stand for what its subrules say; cite the subrule \
                     the claim rests on",
                ),
            });
        }
    }

    for (line, (number, form)) in doc.observations_of(|o| match o {
        Observation::RuleMarker { number, form } => Some((number.clone(), *form)),
        _ => None,
    }) {
        // Every marker, whatever its form, asserts that this number is a rule.
        if !release.rules.contains(&number) {
            out.push(Judged {
                rule: Rule::NumberResolves,
                finding: Finding::at(
                    &doc.rel,
                    line,
                    format!("{number} is cited and the applicable release has no such rule"),
                    "check the number against the pinned text; a rule that moved under a \
                     citation is a bump",
                ),
            });
            continue;
        }
        match form {
            MarkerForm::Mention => out.push(Judged {
                rule: Rule::MentionRetired,
                finding: Finding::at(
                    &doc.rel,
                    line,
                    format!("{number} is named with the retired mention form"),
                    "a claim about the rule takes CR: and a quote; a number that is data goes \
                     in a code span, a fenced block or a string literal",
                ),
            }),
            // A FENCE is where code is shown, and the identifier form IS code, so an
            // illustration of one is not a use of it. The fence suppresses this rule and no
            // other: a fenced sketch in a design document cites its rules for real, which is
            // why fences are not data for the prose forms.
            MarkerForm::IdentifierInProse if doc.parsed.fenced.contains(&line) => {}
            MarkerForm::IdentifierInProse => out.push(Judged {
                rule: Rule::IdentifierInProse,
                finding: Finding::at(
                    &doc.rel,
                    line,
                    format!("the identifier form names {number} in prose"),
                    "the identifier form exists because a NAME cannot hold punctuation; in \
                     prose the prose form fits, and inside backticks it names an identifier",
                ),
            }),
            MarkerForm::Prose => {
                claims += 1;
                if let Some(f) = unquoted(doc, line, &number, &quotes) {
                    out.push(Judged {
                        rule: Rule::QuoteInScope,
                        finding: f,
                    });
                }
            }
            MarkerForm::Identifier => {
                claims += 1;
                let whole = release.rules.get(&number).map(norm).unwrap_or_default();
                let full = quotes.iter().any(|(r, at, body)| {
                    *r == number
                        && in_range(doc, line, *at)
                        && !body.contains('…')
                        && norm(&body.replace('*', "")) == whole
                });
                if !full {
                    out.push(Judged {
                        rule: Rule::IdentifierFullQuote,
                        finding: Finding::at(
                            &doc.rel,
                            line,
                            format!("the name citing {number} does not carry its whole body"),
                            "a name asserts the rule without room to qualify it, so the quote \
                             above it is the rule entire, with no elision",
                        ),
                    });
                }
            }
        }
    }
    (out, claims)
}

/// Whether a quote at `at` can discharge a claim at `line`: same innermost scope, and near it.
///
/// **Either direction.** The conventional shape introduces the rule and then quotes it — `per
/// CR:x:` followed by the block — so a quote usually sits BELOW its marker. Requiring the quote
/// to precede the claim was written into the plan and is wrong; what the distance is for is
/// that the reader sees both at once, and that does not depend on the order.
fn in_range(doc: &Document, line: u32, at: u32) -> bool {
    if line.abs_diff(at) > MAX_DISTANCE {
        return false;
    }
    match (doc.parsed.scope_at(line), doc.parsed.scope_at(at)) {
        (Some(claim), Some(quote)) => {
            claim.first == quote.first && claim.last == quote.last && claim.kind == quote.kind
        }
        // A file with no scope at all — an empty document — holds no claim to discharge.
        _ => false,
    }
}

fn unquoted(
    doc: &Document,
    line: u32,
    number: &RuleNumber,
    quotes: &[(RuleNumber, u32, String)],
) -> Option<Finding> {
    if quotes
        .iter()
        .any(|(r, at, _)| r == number && in_range(doc, line, *at))
    {
        return None;
    }
    let elsewhere = quotes.iter().any(|(r, _, _)| r == number);
    let where_ = match doc.parsed.scope_at(line) {
        Some(s) if s.kind != ScopeKind::Preamble && !s.name.is_empty() => {
            format!(" in `{}`", s.name)
        }
        _ => String::new(),
    };
    Some(Finding::at(
        &doc.rel,
        line,
        format!("{number} is claimed{where_} with no verified quote of it in range"),
        if elsewhere {
            "the quote is outside this scope or further than 60 lines above; a second claim \
             owes its own quote"
        } else {
            "put the rule's text behind the marker, or point at the decision that carries it \
             and drop the number"
        },
    ))
}

/// Run the regime over a model, splitting findings from the backlog a deferral holds back.
pub fn run(
    model: &Model,
    manifest: &Manifest,
    releases: &std::collections::HashMap<Option<String>, Release>,
) -> (Vec<Finding>, Counts) {
    let deferred: BTreeSet<Rule> = manifest
        .migration()
        .deferred
        .iter()
        .filter_map(|n| Rule::parse(n))
        .collect();
    let mut findings = Vec::new();
    let mut backlog: Vec<(Rule, usize)> = Vec::new();
    let mut counts = Counts::default();

    for doc in model.documents() {
        let Some(release) = releases.get(&doc.pin) else {
            continue;
        };
        // A file the manifest exempts from the lint is exempt here too. It is declared stale
        // and is decomposed as its sections are harvested; judging its claims produces a
        // suppression list longer than the findings, which is the argument the exemption
        // already makes. Without this the regime had no exemption path at all, so nothing in
        // the manifest could say so once the deferrals retire.
        let exempt = manifest.lint().exempt_files.contains(&doc.rel);
        if exempt {
            continue;
        }
        let (judged, claims) = check(doc, release);
        counts.claims += claims;
        for j in judged {
            if deferred.contains(&j.rule) {
                match backlog.iter_mut().find(|(r, _)| *r == j.rule) {
                    Some((_, n)) => *n += 1,
                    None => backlog.push((j.rule, 1)),
                }
            } else {
                findings.push(j.finding);
            }
        }
    }

    // A deferral with nothing left to defer has outlived its work, and a list that outlives its
    // work is one nobody deletes.
    for rule in &deferred {
        if !backlog.iter().any(|(r, _)| r == rule) {
            counts.retired.push(*rule);
        }
    }
    backlog.sort();
    counts.backlog = backlog;
    (findings, counts)
}
