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
//! **Every rule of the regime is enforced.** There is no way for a project to hold one back: the
//! set is compiled in and the manifest declares nothing about it, so conformance means the same
//! thing in every tree the tool checks.

use rules::{norm, RuleNumber};

use crate::finding::Finding;
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
/// The set is compiled in for the reason `design@knowledge@components-carry-the-same-documents` gives about the
/// document set: a project free to declare its own would be conformant with whatever it declared.
/// Each is named so the fixture assertion can say which one has no planted violation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    /// Every marked number names a rule the applicable release holds.
    NumberResolves,
    /// A prose marker owes a verified quote of its rule, in scope and in range.
    QuoteInScope,
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
    /// Two markers could have owned an inline quote, and its text verifies against more
    /// than one of them.
    ///
    /// A quote binds to the NEAREST marker before it. The convention a writer reads puts the
    /// marker in the clause that INTRODUCES the quote, and a clause is not a distance, so a
    /// sentence naming a second rule in between binds the quote to the rule the writer did not
    /// mean. Where the text matches only one candidate, `citations` already reports the
    /// mismatch. Where it matches several the binding is unfalsifiable: 90 rules in the pinned
    /// release have a body contained whole inside another rule's, so the wrong choice verifies
    /// and nothing is reported. That is a silent false negative, which this tool may not have.
    QuoteBindingIsAmbiguous,
    /// A quote whose characters are not the release's, though its text is.
    ///
    /// `norm` folds a curly apostrophe or quotation mark to its ASCII form so that a quote
    /// written either way VERIFIES — which is right for comparing and leaves the tree free to
    /// drift. Root `CLAUDE.md` names adjusting a rule's quote style as the smallest form of
    /// paraphrase, and 1 933 of 3 162 rules carry a character that can be folded, so without
    /// this the convention is stated and unenforced.
    QuoteTypography,
    /// A quotation written in the rule-quote form that no marker claims.
    ///
    /// The form is `*"…"*`, and it declares *this is rule text*. Written with no marker to bind
    /// it, it is verified against nothing — and the missing-marker lint, which would otherwise
    /// catch the bare number beside it, is silenced when that number sits in a code span. So
    /// the shape `Rule `x` states *"…"*` produced no output of any kind.
    QuoteHasNoMarker,
}

impl Rule {
    pub const NAMED: [(&'static str, Rule); 10] = [
        ("number-resolves", Rule::NumberResolves),
        ("quote-in-scope", Rule::QuoteInScope),
        ("identifier-in-prose", Rule::IdentifierInProse),
        ("identifier-full-quote", Rule::IdentifierFullQuote),
        ("omission-marked", Rule::OmissionMarked),
        ("fragment-long-enough", Rule::FragmentLongEnough),
        (
            "parent-rule-is-not-its-subrules",
            Rule::ParentRuleIsNotItsSubrules,
        ),
        ("quote-has-no-marker", Rule::QuoteHasNoMarker),
        ("quote-binding-is-ambiguous", Rule::QuoteBindingIsAmbiguous),
        ("quote-typography", Rule::QuoteTypography),
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

/// One finding, and the rule of the regime it came from.
pub struct Judged {
    pub rule: Rule,
    pub finding: Finding,
}

/// What a run of the regime counted.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub claims: usize,
}

/// Every verified quote in a document: the rule it is offered as, and the line it sits on.
fn verified(doc: &Document, release: &Release) -> Vec<(RuleNumber, u32, String)> {
    super::citations::quotes(doc)
        .into_iter()
        .filter(|q| {
            let text = super::citations::section_body(&q.rule, &q.body);
            // A quote that quotes nothing verifies nothing. An ellipsis-only body survives
            // both arms without this: an empty string is contained in every rule, and a
            // fragment test over no fragments passes vacuously — so `*"…"*` discharged a
            // claim while carrying no evidence at all.
            let whole = norm(&text.replace(['*', '…'], ""));
            if whole.is_empty() {
                return false;
            }
            release
                .rules
                .get(&q.rule)
                .is_some_and(|body| body.contains(&whole))
                || super::citations::fragments(&text).0.iter().all(|f| {
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
    let text = super::citations::section_body(&quote.rule, &quote.body).replace('*', "");
    let whole = norm(&text.replace('…', ""));
    // A quote of the rule entire owes no mark and no floor: there is nothing omitted, and
    // nothing more of the rule to keep. 241 of 3 162 rules have a body under the floor.
    if whole == body {
        return out;
    }
    // **A section is quoted by its heading entire.** The title is the claim of identity, and
    // a piece of one — an elided tail, a fragment over the length floor — reads as the whole
    // to anyone who does not know the title. A quote the title does not hold at all is left
    // to `citations`, which reports what it says against the release.
    if quote.rule.is_section() {
        if body.contains(&whole) || text.contains('…') {
            out.push(Judged {
                rule: Rule::OmissionMarked,
                finding: Finding::at(
                    &doc.rel,
                    quote.line,
                    format!(
                        "the quote of {} is not the section's heading entire",
                        quote.rule
                    ),
                    "a section is quoted by its heading line whole; nothing of the title may \
                     be elided, and the whole line always passes",
                ),
            });
        }
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

/// Whether the quote's CHARACTERS are the release's, not merely its words.
///
/// Verification folds typography on both sides, so `cards' faces` verifies against the release's
/// `cards’ faces`. That fold is deliberate — a quote written either way is checked rather than
/// dropped — and it means nothing else asks whether the document holds what the rule prints.
fn typography(doc: &Document, release: &Release, quote: &crate::quote::Quote) -> Vec<Judged> {
    let Some(raw) = release.rules.raw(&quote.rule) else {
        return Vec::new();
    };
    let ws = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let body = ws(raw);
    let text = super::citations::section_body(&quote.rule, &quote.body).replace('*', "");
    for piece in text.split('…') {
        let piece = ws(piece);
        if piece.chars().count() < MIN_FRAGMENT {
            continue;
        }
        if body.contains(&piece) {
            continue;
        }
        // The fragment verified against the folded body — `citations` reports it otherwise —
        // so a miss here is typography and nothing else.
        if norm(&body).contains(&norm(&piece)) {
            return vec![Judged {
                rule: Rule::QuoteTypography,
                finding: Finding::at(
                    &doc.rel,
                    quote.line,
                    format!(
                        "the quote of {} does not use the release's own characters: {}",
                        quote.rule,
                        clip(&piece, 60)
                    ),
                    "take the text from `cargo knowledge rules show`, which prints the release \
                     verbatim; a straight quote where the rule prints a curly one is the \
                     smallest form of paraphrase",
                ),
            }];
        }
    }
    Vec::new()
}

/// Whether more than one marker could have owned this quote, judged by the corpus.
///
/// **Only the corpus can answer it.** Two markers before a quote is ordinary and correct — a
/// sentence citing two rules in sequence, each with its own quote, is the shape the nearest-marker
/// rule exists to serve. What is not answerable by a reader is a quote whose text belongs to
/// several of the candidates, because then nothing in the file says which was meant and the
/// checker's choice cannot be wrong in a way anything detects.
///
/// The repair is never to retarget the citation: the quote is verbatim and the rule it is bound to
/// holds it. It is to move the other marker out from between, or to use the blockquote form, which
/// binds by the number printed at its head and is not exposed to this at all.
fn ambiguous_binding(
    doc: &Document,
    release: &Release,
    quote: &crate::quote::Quote,
) -> Vec<Judged> {
    if quote.alternatives.is_empty() {
        return Vec::new();
    }
    let holds = |rule: &RuleNumber| {
        let text = super::citations::section_body(rule, &quote.body);
        release
            .rules
            .get(rule)
            .is_some_and(|body| norm(body).contains(&norm(&text.replace(['*', '…'], ""))))
    };
    if !holds(&quote.rule) {
        // The text does not verify as the rule it bound to. `citations` reports that already,
        // and reporting it twice would put two findings on one repair.
        return Vec::new();
    }
    let also: Vec<String> = quote
        .alternatives
        .iter()
        .filter(|r| holds(r))
        .map(|r| r.to_string())
        .collect();
    if also.is_empty() {
        return Vec::new();
    }
    vec![Judged {
        rule: Rule::QuoteBindingIsAmbiguous,
        finding: Finding::at(
            &doc.rel,
            quote.line,
            format!(
                "this quote is bound to {} because that marker is nearest, and its text is also {}",
                quote.rule,
                also.join(" and ")
            ),
            "a quote takes the NEAREST marker before it, not the clause that introduces it; \
             move the other marker out from between, or quote it as a blockquote, which binds \
             by the number at its head",
        ),
    }]
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
    // Rules whose whole body is OWED, because a name cites them. `identifier-full-quote`
    // demands exactly the shape `parent-rule-is-not-its-subrules` forbids, so without this the
    // two rules cannot both be satisfied and a name citing a parent rule has no legal repair
    // but a rename. Three sessions hit it independently and invented three different
    // workarounds, which is what an unsatisfiable pair of instructions produces.
    let named: Vec<(RuleNumber, u32)> = doc
        .observations_of(|o| match o {
            Observation::RuleMarker { number, form } if *form == MarkerForm::Identifier => {
                Some(number.clone())
            }
            _ => None,
        })
        .map(|(line, number)| (number, line))
        .collect();

    for quote in super::citations::quotes(doc) {
        out.extend(completeness(doc, release, &quote));
        out.extend(ambiguous_binding(doc, release, &quote));
        out.extend(typography(doc, release, &quote));
        // A parent rule's body says what it says; its subrules say the rest. A quote of the
        // parent entire passes by equality and can be offered for a claim that belongs to a
        // subrule, which is the one hole the whole-body exception opens.
        let whole = release.rules.get(&quote.rule).map(norm);
        // The elision mark is stripped so a disclosed parent quote is still RECOGNISED as the
        // parent's whole body; whether it discloses is asked separately below.
        let text = norm(&quote.body.replace(['*', '…'], ""));
        let owed_whole = named
            .iter()
            .any(|(rule, at)| *rule == quote.rule && in_range(doc, *at, quote.line));
        // **A trailing elision discloses that the subrules say more, and that is what this
        // rule asks for.** The bargain is `omission-marked`'s: quoting only what the claim
        // needs is allowed, doing it invisibly is not. A parent's body is not the whole of
        // what the rule states — its subrules are the rest — so a quote that stops at the
        // parent and marks the omission is complete and honest, and one that presents the
        // parent as the entire rule is the shape this catches.
        //
        // Without this the rule is UNSATISFIABLE beside `identifier-full-quote` and beside any
        // claim that genuinely rests on a parent's own body: four migrating sessions hit it
        // independently and invented four different forms, which is what a pair of
        // instructions with no legal move produces.
        let discloses_subrules = quote
            .body
            .trim_end()
            .trim_end_matches(['*', '"'])
            .ends_with('…');
        if whole.as_deref() == Some(text.as_str())
            && release.rules.has_subrules(&quote.rule)
            && !owed_whole
            && !discloses_subrules
        {
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
        // Every marker, whatever its form, asserts that this number is a rule — or, in the
        // section form, a section the release prints a heading for.
        if !release.rules.contains(&number) {
            let what = if number.is_section() {
                format!("{number} is cited and the applicable release has no such section")
            } else {
                format!("{number} is cited and the applicable release has no such rule")
            };
            out.push(Judged {
                rule: Rule::NumberResolves,
                finding: Finding::at(
                    &doc.rel,
                    line,
                    what,
                    "check the number against the pinned text; a rule that moved under a \
                     citation is a bump",
                ),
            });
            continue;
        }
        match form {
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

/// Run the regime over every document in a model.
pub fn run(
    model: &Model,
    exempt_files: &[std::path::PathBuf],
    releases: &std::collections::HashMap<Option<String>, Release>,
) -> (Vec<Finding>, Counts) {
    let mut findings = Vec::new();
    let mut counts = Counts::default();

    for doc in model.documents() {
        let Some(release) = releases.get(&doc.pin) else {
            continue;
        };
        // A file the manifest exempts from the lint is exempt here too. It is declared stale
        // and is decomposed as its sections are harvested; judging its claims produces a
        // suppression list longer than the findings, which is the argument the exemption
        // already makes. That list is the only exemption path the regime has, and it names
        // FILES that are leaving the tree — never a rule, and never for the whole tree.
        if exempt_files.contains(&doc.rel) {
            continue;
        }
        let (judged, claims) = check(doc, release);
        counts.claims += claims;
        findings.extend(judged.into_iter().map(|j| j.finding));
    }

    (findings, counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture below is written inline: the checker reads no string literal of its own
    // source, per `design@knowledge@checker-source-literals-are-data`.

    /// A parent with one subrule, for the parent-rule cases.
    const PARENT_CORPUS: &str = concat!(
        "100.8 A parent body long enough to be evidence on its own terms.\n",
        "100.8a A subrule saying its own thing, which the parent does not say.\n",
    );

    /// Two rules sharing a body exactly, and one that shares with nothing.
    ///
    /// Not contrived: 90 rules in the pinned release have a body contained whole inside
    /// another rule's, `103.4` inside `119.1` among them.
    const CORPUS: &str = concat!(
        "100.5 A body long enough to be evidence and shared by two rules exactly.\n",
        "100.6 A body long enough to be evidence and shared by two rules exactly.\n",
        "100.7 A different body, also long enough to be evidence, and shared with nothing.\n",
    );

    fn release() -> Release {
        Release::new(CORPUS, 0)
    }

    fn findings(text: &str) -> Vec<String> {
        let model = crate::model::Model::from_documents(vec![(
            std::path::PathBuf::from("notes/a.md"),
            text.to_string(),
        )]);
        let doc = model.documents()[0].clone();
        check(&doc, &release())
            .0
            .into_iter()
            .filter(|j| j.rule == Rule::QuoteBindingIsAmbiguous)
            .map(|j| j.finding.what)
            .collect()
    }

    fn section_release() -> Release {
        Release::new(&format!("100. A Section Title Long Enough\n{CORPUS}"), 0)
    }

    fn judged(text: &str, release: &Release, rule: Rule) -> Vec<String> {
        let model = crate::model::Model::from_documents(vec![(
            std::path::PathBuf::from("notes/a.md"),
            text.to_string(),
        )]);
        check(&model.documents()[0].clone(), release)
            .0
            .into_iter()
            .filter(|j| j.rule == rule)
            .map(|j| j.finding.what)
            .collect()
    }

    #[test]
    fn a_section_marker_owes_its_heading_quote_in_scope() {
        // The section form owes a verbatim quote of the heading line, under the same scope
        // and distance rules as a subrule quote. Both quote forms discharge it: the
        // blockquote binds by the printed head, the inline form by the nearest marker.
        let release = section_release();
        let bare = "### A section\n\nin the order CR:100 states them\n";
        let owed = judged(bare, &release, Rule::QuoteInScope);
        assert_eq!(owed.len(), 1, "{owed:#?}");
        assert!(owed[0].contains("100"), "{owed:#?}");
        let block = "### A section\n\nin the order CR:100 states them:\n\n\
                     > 100. A Section Title Long Enough\n";
        assert_eq!(
            judged(block, &release, Rule::QuoteInScope),
            Vec::<String>::new()
        );
        let inline =
            "### A section\n\nin the order CR:100, *\"A Section Title Long Enough\"*, states them\n";
        assert_eq!(
            judged(inline, &release, Rule::QuoteInScope),
            Vec::<String>::new()
        );
    }

    #[test]
    fn a_section_is_quoted_by_its_heading_entire_or_reported() {
        // A piece of a title reads as the whole to anyone who does not know the title, and
        // titles long enough to clear the fragment floor were exposed: the pinned release
        // holds nine of thirty characters or more. Mutation checked: removing the section
        // branch from `completeness` passes the elided long title silently.
        let release = Release::new(
            &format!("200. A Title Comfortably Longer Than The Fragment Floor\n{CORPUS}"),
            0,
        );
        let elided = "### A section\n\nper CR:200, *\"A Title Comfortably Longer Than The…\"*\n";
        let found = judged(elided, &release, Rule::OmissionMarked);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("heading entire"), "{found:#?}");
        // An ellipsis-only quote quotes nothing and must not discharge anything.
        let empty = "### A section\n\nper CR:200, *\"…\"*\n";
        assert_eq!(judged(empty, &release, Rule::OmissionMarked).len(), 1);
        // The whole title is complete: no mark owed, in either form, with or without the
        // printed number at its head.
        for clean in [
            "### A section\n\nper CR:200, *\"A Title Comfortably Longer Than The Fragment Floor\"*\n",
            "### A section\n\nper CR:200:\n\n> 200. A Title Comfortably Longer Than The Fragment Floor\n",
            "### A section\n\nper CR:200, *\"200. A Title Comfortably Longer Than The Fragment Floor\"*\n",
        ] {
            assert_eq!(
                judged(clean, &release, Rule::OmissionMarked),
                Vec::<String>::new(),
                "{clean}"
            );
        }
    }

    #[test]
    fn an_ellipsis_only_quote_discharges_nothing() {
        // An empty string is contained in every rule and a fragment test over no fragments
        // passes vacuously, so `*"…"*` used to satisfy `quote-in-scope` while quoting
        // nothing. Mutation checked: removing the emptiness guard from `verified` passes
        // this with zero findings.
        let text = "### A section\n\nper CR:100.5, *\"…\"*\n";
        let found = judged(text, &release(), Rule::QuoteInScope);
        assert_eq!(found.len(), 1, "{found:#?}");
    }

    #[test]
    fn a_section_the_release_does_not_print_is_reported_as_such() {
        // The finding must say SECTION, or the repairer greps the rules body for a dotted
        // number that never existed.
        let text = "### A section\n\nnothing here, per CR:999\n";
        let found = judged(text, &section_release(), Rule::NumberResolves);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no such section"), "{found:#?}");
    }

    #[test]
    fn a_whole_title_quote_owes_no_elision_and_no_subrule_disclosure() {
        // A section's rules are not its subrules: the title is the whole of the section's
        // own text, so quoting it entire is complete — no omission mark owed, no
        // parent-rule finding. Mutation checked: keying the parent test on `major` alone
        // fires it for every section with rules.
        let release = section_release();
        let block = "### A section\n\nper CR:100:\n\n> 100. A Section Title Long Enough\n";
        for rule in [
            Rule::OmissionMarked,
            Rule::FragmentLongEnough,
            Rule::ParentRuleIsNotItsSubrules,
        ] {
            assert_eq!(
                judged(block, &release, rule),
                Vec::<String>::new(),
                "{rule:?}"
            );
        }
    }

    #[test]
    fn a_quote_two_markers_could_own_is_reported_when_both_rules_hold_it() {
        // The silent case. The writer means the first rule and the checker binds the second,
        // and because both bodies hold the text nothing verifies wrongly — so without this
        // rule there is no finding at all, and the generated index records the wrong rule as
        // cited, which is the bump work list.
        let found = findings(
            "per CR:100.5, which CR:100.6 restates, \
             *\"A body long enough to be evidence and shared by two rules exactly.\"*\n",
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("100.6"), "{found:#?}");
        assert!(found[0].contains("100.5"), "{found:#?}");
    }

    #[test]
    fn a_whole_body_quote_a_name_owes_is_not_reported_as_a_parent_rule() {
        // `identifier-full-quote` demands the rule's whole body with no elision, and
        // `parent-rule-is-not-its-subrules` forbids a whole-body quote of a rule with
        // subrules. Together they leave a name citing a parent rule with no legal repair but a
        // rename — which is what three migrating sessions did, three different ways.
        let text = "/// Per CR:100.8:\n\
                    ///\n\
                    /// > 100.8 A parent body long enough to be evidence on its own terms.\n\
                    fn cr_100_8_a_test() {}\n";
        let model = crate::model::Model::from_documents(vec![(
            std::path::PathBuf::from("code/a.rs"),
            text.to_string(),
        )]);
        let doc = model.documents()[0].clone();
        let release = Release::new(PARENT_CORPUS, 0);
        let found: Vec<String> = check(&doc, &release)
            .0
            .into_iter()
            .filter(|j| j.rule == Rule::ParentRuleIsNotItsSubrules)
            .map(|j| j.finding.what)
            .collect();
        assert_eq!(
            found,
            Vec::<String>::new(),
            "the name owes this exact quote"
        );

        // The control: the same whole-body quote with NO name citing it is still reported.
        let plain = crate::model::Model::from_documents(vec![(
            std::path::PathBuf::from("code/b.md"),
            "### A section\n\nPer CR:100.8:\n\n\
             > 100.8 A parent body long enough to be evidence on its own terms.\n"
                .to_string(),
        )]);
        let plain = plain.documents()[0].clone();
        let still: Vec<String> = check(&plain, &release)
            .0
            .into_iter()
            .filter(|j| j.rule == Rule::ParentRuleIsNotItsSubrules)
            .map(|j| j.finding.what)
            .collect();
        assert_eq!(still.len(), 1, "{still:#?}");
    }

    #[test]
    fn a_parent_quoted_whole_is_reported_unless_it_discloses_its_subrules() {
        // The bargain is `omission-marked`'s, applied to the other axis: a parent's body is not
        // the whole of what the rule states, so stopping there is allowed and doing it
        // invisibly is not. Without the disclosure form the rule has no legal repair when the
        // claim rests on the parent's own body, which is most of the time.
        let release = Release::new(PARENT_CORPUS, 0);
        let parent_findings = |text: &str| -> Vec<String> {
            let model = crate::model::Model::from_documents(vec![(
                std::path::PathBuf::from("notes/a.md"),
                text.to_string(),
            )]);
            check(&model.documents()[0].clone(), &release)
                .0
                .into_iter()
                .filter(|j| j.rule == Rule::ParentRuleIsNotItsSubrules)
                .map(|j| j.finding.what)
                .collect()
        };
        let bare = parent_findings(
            "### A section\n\nPer CR:100.8:\n\n\
             > 100.8 A parent body long enough to be evidence on its own terms.\n",
        );
        assert_eq!(bare.len(), 1, "presented as the entire rule: {bare:#?}");
        let disclosed = parent_findings(
            "### A section\n\nPer CR:100.8:\n\n\
             > 100.8 A parent body long enough to be evidence on its own terms. …\n",
        );
        assert_eq!(
            disclosed,
            Vec::<String>::new(),
            "the mark says the subrules state the rest"
        );
    }

    #[test]
    fn a_second_marker_whose_rule_does_not_hold_the_text_is_not_ambiguous() {
        // Every one of the ten lines in this repository carrying two prose markers and an
        // inline quote is this shape, so the rule must stay silent on it or it reports ten
        // correct citations.
        let found = findings(
            "per CR:100.7, and the rule CR:100.5 says \
             *\"A body long enough to be evidence and shared by two rules exactly.\"*\n",
        );
        assert_eq!(found, Vec::<String>::new());
    }

    #[test]
    fn a_quote_that_does_not_verify_as_the_rule_it_bound_to_is_left_to_citations() {
        // The other half of the divergence: the text belongs to the introducing rule and the
        // nearest marker is a rule that does NOT hold it. `citations` reports that as a
        // mismatch, and a second finding here would put two on one repair.
        let found = findings(
            "per CR:100.5, and unlike CR:100.7, the rule says \
             *\"A body long enough to be evidence and shared by two rules exactly.\"*\n",
        );
        assert_eq!(found, Vec::<String>::new());
    }

    #[test]
    fn a_blockquote_is_never_ambiguous_because_it_binds_by_its_own_number() {
        // A block takes the number printed at its head, so no other marker could have owned
        // it however many precede it. This is the repair the finding recommends, and it has to
        // actually be a repair.
        let found = findings(
            "The rule is CR:100.5, which CR:100.6 restates:\n\n\
             > 100.5 A body long enough to be evidence and shared by two rules exactly.\n",
        );
        assert_eq!(found, Vec::<String>::new());
    }
}
