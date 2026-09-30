//! Rust: which byte ranges are prose, and which item each one belongs to.
//!
//! The grammar answers both, and that is the whole reason it is here. A line-oriented scanner
//! can only guess from a prefix, and every guess it made is a recorded defect: a formatted
//! return type opening a line with an angle bracket is not a blockquote, a float literal is not
//! a rule number, and a string literal bound to a name is not a citation.
//!
//! **Which string literals are prose is decided twice.** The caller says, through `Literals`,
//! whether the file is the checker's own source, where every literal is a fixture and none is
//! prose, per `design@knowledge@checker-source-literals-are-data`. Everywhere else the grammar says:
//! a literal bound to a name is data, and every other literal is a message written for a
//! human, which cites for real.
//!
//! **A comment's content arrives without its marker.** The grammar makes the marker its own
//! node, so `doc_comment` is the text after `///` or `//!` and nothing has to strip anything.

use tree_sitter::{Node, Parser};

use super::{md, Literals, Parsed, Prose, Scope, ScopeKind};

/// Node kinds that own a scope.
///
/// A quote in any of these discharges a claim made anywhere inside it. Nested items are
/// narrower than the ones holding them, and `Scope::depth` gives every item the same depth, so
/// a nested function's scope is picked over its parent's only because it is listed later —
/// which is why `scope_at` takes the LAST maximum rather than the first.
const ITEMS: [&str; 11] = [
    "function_item",
    "impl_item",
    "mod_item",
    "struct_item",
    "enum_item",
    "trait_item",
    "union_item",
    "const_item",
    "static_item",
    "type_item",
    "macro_definition",
];

/// A Rust file: one prose region per contiguous comment run, and a scope per item.
///
/// A file the grammar cannot parse yields no prose and no scopes, which would make every
/// citation in it invisible — so `Parsed::trouble` says so, and a check reports it.
pub fn parse(text: &str, literals: Literals) -> Parsed {
    let Some(tree) = tree(text) else {
        return Parsed::default();
    };
    let root = tree.root_node();
    let mut comments = Vec::new();
    let mut items = Vec::new();
    let mut names = Vec::new();
    let mut too_deep = false;
    collect(
        root,
        text,
        literals,
        false,
        0,
        &mut comments,
        &mut items,
        &mut names,
        &mut too_deep,
    );
    comments.sort_by_key(|c| c.first);
    items.sort_by_key(|i| (i.0, i.1));
    let items = items;

    // An item's scope reaches UP over the doc comment and the attributes attached to it. The
    // grammar makes those siblings rather than children, so an item's own range opens at its
    // keyword — and a claim in the doc comment would then sit in a different scope from the
    // name it documents, which is the one place they must agree.
    let attached: std::collections::HashSet<u32> = comments
        .iter()
        .filter(|c| c.comment)
        .map(|c| c.first)
        .chain(
            text.lines()
                .enumerate()
                .filter_map(|(i, l)| l.trim_start().starts_with("#[").then_some(i as u32 + 1)),
        )
        .collect();
    let mut prose = runs(comments);
    let mut scopes: Vec<Scope> = Vec::new();
    for (i, (first, last, name)) in items.iter().enumerate() {
        // The walk stops at the line after the previous SIBLING ends. Without that bound two
        // scopes overlap, and `scope_at` takes the last maximum — so the overlapped line is
        // judged against the FOLLOWING item, and a finding names the wrong one. Measured in
        // this repository before the bound: seven overlaps, every one a pair of adjacent
        // two-line statics.
        //
        // `prev_last < first` is what makes it siblings. `items` is sorted by `(first, last)`,
        // so an ENCLOSING item is also earlier in it, and its `last` is past this item's first
        // line — a floor taken over every earlier item therefore lands above `first`, the
        // walk-up never runs, and a nested item's doc comment stays in the enclosing scope
        // while its name sits in the inner one. That is the one thing the walk-up exists to
        // prevent, and it made the identifier form unusable on a test function, which is
        // where that form belongs.
        let floor = items[..i]
            .iter()
            .filter(|(_, prev_last, _)| prev_last < first)
            .map(|(_, prev_last, _)| *prev_last + 1)
            .max()
            .unwrap_or(1);
        let mut top = *first;
        while top > floor && attached.contains(&(top - 1)) {
            top -= 1;
        }
        scopes.push(Scope {
            kind: ScopeKind::Item,
            name: name.clone(),
            first: top,
            last: *last,
        });
    }
    // Every line an item does not hold is still somewhere, and a citation written there — in a
    // module doc, or in a comment between items — needs a scope or it can never be discharged.
    let last_line = text.lines().count().max(1) as u32;
    scopes.insert(
        0,
        Scope {
            kind: ScopeKind::Preamble,
            name: String::new(),
            first: 1,
            last: last_line,
        },
    );

    let mut fenced = Vec::new();
    for region in &mut prose {
        let text = region.text.clone();
        let a = md::analyse(&text, region);
        region.code = a.code;
        fenced.extend(a.fenced);
    }
    fenced.sort_unstable();
    fenced.dedup();
    names.sort();
    names.dedup();
    let trouble = if too_deep {
        Some("the source nests deeper than the parser walks, so part of it was not read".into())
    } else if root.has_error() {
        Some("the Rust grammar could not parse this file, so what it cites may be unread".into())
    } else {
        None
    };
    Parsed {
        prose,
        scopes,
        fenced,
        names,
        inert: Vec::new(),
        // Frontmatter is a markdown file's own opening block. A Rust file has none, and a
        // doc comment that opens with `---` is a thematic break inside prose.
        frontmatter: None,
        trouble,
    }
}

fn tree(text: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .ok()?;
    parser.parse(text, None)
}

/// One line of prose pulled out of a source file, and where it came from.
struct CommentLine {
    first: u32,
    text: String,
    /// A comment, rather than the inside of a string literal.
    ///
    /// Both are prose, and they are NOT interchangeable. A comment above an item documents it;
    /// a string inside its body is a message. Treating a message line as attached
    /// documentation made an item's scope swallow the line above it, and joining the two into
    /// one markdown region let an unclosed code span in a comment reach into the next string.
    comment: bool,
}

/// Node kinds whose initialiser is **named test data** rather than prose.
///
/// A string bound to a name is a fixture: an address, a formatted figure a display test
/// expects, a line of tool output a parser is fed. Measured when reading every literal as prose
/// was tried: six such literals in three crates outside the checker, every one rule-shaped by
/// accident. A string that is NOT bound to a name is almost always a message — an assertion's
/// explanation, a panic, a format — and those cite rules on purpose. Reading both as data lost
/// 32 citations in this tree. The checker's own fixtures are not what this rule is for any
/// more: `Literals::Data` covers those by location, whatever they are bound to.
const BINDINGS: [&str; 3] = ["const_item", "static_item", "let_declaration"];

/// Node kinds that ARE a name.
const NAMES: [&str; 4] = [
    "identifier",
    "type_identifier",
    "field_identifier",
    "shorthand_field_identifier",
];

/// Every name DECLARED under a node, which is not the same as every name mentioned in it.
///
/// **A declaration asserts; a use does not.** Collecting every identifier made a call site, a
/// `use` import and any other mention of a rule-named item a citation of that rule — owing the
/// rule's whole body in the CALLER's scope and a prose marker above the call. There is nowhere
/// to put either, and the orphan lint is not deferred and runs in the write hook, so
/// extracting a rule-named test into a helper and calling it would have failed on every save
/// with advice the author could not follow.
fn declared(node: Node, src: &str, out: &mut Vec<(u32, String)>) {
    if NAMES.contains(&node.kind()) {
        out.push((
            node.start_position().row as u32 + 1,
            src[node.byte_range()].to_string(),
        ));
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        declared(child, src, out);
    }
}

/// How deep the walk may go before it gives up on a file.
///
/// The recursion has no natural bound — the grammar nests once per parenthesis — and a file
/// deep enough overflows the stack, which aborts the WHOLE run with no file named and no
/// finding printed. A bound turns that into one reported file. No real source comes near it;
/// the deepest item in this repository is under 40.
const MAX_DEPTH: usize = 512;

#[allow(clippy::too_many_arguments)]
fn collect(
    node: Node,
    src: &str,
    literals: Literals,
    in_binding: bool,
    depth: usize,
    comments: &mut Vec<CommentLine>,
    items: &mut Vec<(u32, u32, String)>,
    names: &mut Vec<(u32, String)>,
    too_deep: &mut bool,
) {
    if depth > MAX_DEPTH {
        *too_deep = true;
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "string_literal" | "raw_string_literal"
                if literals == Literals::Prose && !in_binding =>
            {
                if let Some(content) = named_child(child, "string_content") {
                    let at = content.start_position().row as u32 + 1;
                    for (i, line) in src[content.byte_range()].split('\n').enumerate() {
                        comments.push(CommentLine {
                            comment: false,
                            first: at + i as u32,
                            text: line.to_string(),
                        });
                    }
                }
            }
            "string_literal" | "raw_string_literal" => {}
            "line_comment" | "block_comment" => {
                // The content node, where there is one, is the text with the marker already
                // removed. A plain `//` comment has no such child, so its own text is taken
                // and the marker trimmed — the one place a prefix is removed by hand, and it
                // is removed from a node the grammar has already identified as a comment.
                let content = child
                    .child_by_field_name("doc")
                    .or_else(|| named_child(child, "doc_comment"));
                // A BLOCK comment's content keeps the `*` that convention puts at the head of
                // every interior line, doc comment or not — the grammar only takes off the
                // opening and closing markers. A `///` line must NOT be treated this way: a
                // `*` there opens a markdown list, which is ordinary in a doc comment.
                let block = child.kind() == "block_comment";
                let (body, at) = match content {
                    Some(n) => (
                        if block {
                            src[n.byte_range()]
                                .split('\n')
                                .map(strip_star)
                                .collect::<Vec<_>>()
                                .join("\n")
                        } else {
                            src[n.byte_range()].to_string()
                        },
                        n.start_position().row as u32 + 1,
                    ),
                    None => (
                        strip_plain(&src[child.byte_range()]),
                        child.start_position().row as u32 + 1,
                    ),
                };
                // The content node carries its own line ending, so splitting on it without
                // trimming yields a phantom empty line at the row below — which then joins the
                // next comment to this one and puts every file line off by the count of them.
                for (i, line) in body.trim_end_matches(['\n', '\r']).split('\n').enumerate() {
                    comments.push(CommentLine {
                        comment: true,
                        first: at + i as u32,
                        // Exactly one space, the one the marker convention puts after `///`.
                        // Trimming further would flatten the indentation markdown reads as
                        // nesting.
                        text: line.strip_prefix(' ').unwrap_or(line).to_string(),
                    });
                }
            }
            kind if ITEMS.contains(&kind) => {
                // An `impl` block has no `identifier` child and no `name` field, so both
                // lookups miss and every one of them was called `impl_item` in a finding.
                let name = named_child(child, "identifier")
                    .or_else(|| child.child_by_field_name("name"))
                    .or_else(|| child.child_by_field_name("type"))
                    .or_else(|| named_child(child, "type_identifier"))
                    .map(|n| src[n.byte_range()].to_string())
                    .unwrap_or_else(|| kind.to_string());
                if let Some(n) = named_child(child, "identifier") {
                    names.push((n.start_position().row as u32 + 1, name.clone()));
                }
                items.push((
                    child.start_position().row as u32 + 1,
                    child.end_position().row as u32 + 1,
                    name,
                ));
                // A `const` or a `static` is BOTH an item and a binding, and the item arm
                // is the one that matches — so the binding has to be carried in here too, or
                // a named fixture is read as prose after all.
                collect(
                    child,
                    src,
                    literals,
                    in_binding || BINDINGS.contains(&kind),
                    depth + 1,
                    comments,
                    items,
                    names,
                    too_deep,
                );
            }
            // An ARGUMENT is never the binding's value, whatever it is bound inside. The
            // shape that forced this: `let x = y.expect("CR:… grants priority")`, where the
            // message is prose sitting inside a `let`, and reading it as data lost three
            // citations that were written for a human to read on failure.
            // A macro's `token_tree` is NOT an argument list and must not be swept in with
            // one: it is the body of every macro, so resetting the flag there made
            // `let x = vec!["…"]` live prose while `let x = ["…"]` stayed data — the same
            // fixture read two ways by which bracket it was written with.
            "arguments" => collect(
                child,
                src,
                literals,
                false,
                depth + 1,
                comments,
                items,
                names,
                too_deep,
            ),
            // A binding and a parameter DECLARE their names, and the convention covers a
            // variable named for a rule as much as a function. Nothing else here is a
            // declaration, so nothing else contributes a name.
            kind @ ("let_declaration" | "parameter" | "closure_parameters") => {
                if let Some(pattern) = child
                    .child_by_field_name("pattern")
                    .or_else(|| child.child_by_field_name("name"))
                {
                    declared(pattern, src, names);
                } else if child.kind() == "closure_parameters" {
                    declared(child, src, names);
                }
                collect(
                    child,
                    src,
                    literals,
                    in_binding || BINDINGS.contains(&kind),
                    depth + 1,
                    comments,
                    items,
                    names,
                    too_deep,
                );
            }
            kind => collect(
                child,
                src,
                literals,
                in_binding || BINDINGS.contains(&kind),
                depth + 1,
                comments,
                items,
                names,
                too_deep,
            ),
        }
    }
}

fn named_child<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    let found = node.children(&mut cursor).find(|c| c.kind() == kind);
    found
}

/// A comment's text, with its markers removed.
///
/// The grammar gives `///` and `//!` their own marker node and hands the content over clean.
/// A BLOCK comment gets no such treatment: `/**`, `/*!` and `/*` open it and `*/` closes it,
/// and the `*` that convention puts at the head of every interior line is part of the content.
/// Left in place it sits in front of a blockquote marker, so a rule quoted inside a block
/// comment is not a quote at all — and the author is told instead that the rule is named with
/// no marker, pointing inside the quote.
fn strip_plain(text: &str) -> String {
    let t = text.trim_start();
    for marker in ["///", "//!", "//", "/*!", "/**", "/*"] {
        if let Some(rest) = t.strip_prefix(marker) {
            let body = rest.trim_end_matches("*/");
            return body
                .split('\n')
                .map(strip_star)
                .collect::<Vec<_>>()
                .join("\n");
        }
    }
    t.to_string()
}

/// One `*` continuation leader, and the single space after it.
///
/// A leading `*` is a leader only when a space or the line end follows. `*"…"*` is the
/// inline-quote convention, and stripping its opening delimiter would destroy the quote.
fn strip_star(line: &str) -> &str {
    let body = line.trim_start();
    match body.strip_prefix('*') {
        Some(rest) if rest.is_empty() || rest.starts_with([' ', '\t']) => {
            rest.strip_prefix([' ', '\t']).unwrap_or(rest)
        }
        _ => line,
    }
}

/// Contiguous comment lines, joined into one markdown region each.
///
/// The run is the unit because a markdown construct spans lines: a blockquote written as three
/// `///` lines is one blockquote, and judging each line alone loses it. A blank line or a line
/// of code ends the run, which is what a reader sees too.
fn runs(comments: Vec<CommentLine>) -> Vec<Prose> {
    let mut out: Vec<Prose> = Vec::new();
    let mut origin: Option<bool> = None;
    for c in comments {
        // A comment and the string on the line below it are two regions, not one. Joined,
        // markdown inline state flows between them — one unmatched backtick in a comment opens
        // a code span that swallows the next line, and every citation in it is then read as
        // data and silently leaves the walk.
        let joins = origin == Some(c.comment);
        origin = Some(c.comment);
        match out.last_mut() {
            Some(p) if joins && p.lines.last() == Some(&(c.first - 1)) => {
                p.text.push('\n');
                p.text.push_str(&c.text);
                p.lines.push(c.first);
            }
            _ => out.push(Prose {
                text: c.text,
                lines: vec![c.first],
                // A source file is not a decision's home, so a slug written in one defines
                // nothing. `design@knowledge@a-slug-belongs-to-a-component` puts that home in a component's
                // design document.
                structural: false,
                code: Vec::new(),
                line_starts: Default::default(),
            }),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every test below reads a file the way the tree outside the checker is read.
    fn parse(text: &str) -> Parsed {
        super::parse(text, Literals::Prose)
    }

    #[test]
    fn in_data_mode_no_string_literal_is_prose_and_every_comment_still_is() {
        // The checker's own source. A literal in a call, in a binding or in a macro is
        // dropped alike, and the comment above stays a region with its scope.
        const SRC: &str = "/// per note token-42\nfn f() {\n    let s = \"token-42\";\n    \
                           assert!(x, \"token-42 orders them\");\n    g(\"token-42\");\n}\n";
        let p = super::parse(SRC, Literals::Data);
        assert_eq!(p.prose.len(), 1, "{:?}", p.prose);
        assert_eq!(p.prose[0].lines, vec![1]);
        assert!(p.prose[0].text.contains(TOKEN));
        assert_eq!(p.scope_at(3).map(|s| s.name.as_str()), Some("f"));
    }

    /// A name is code, and an extension that reads names finds them here, as the rules
    /// extension reads the identifier form of a rule marker. Both fixtures are bound, so they are
    /// data in any reading.
    const NAMED: &str = "fn helper_holds() {}\n";
    const WANTED: &str = "helper_holds";

    #[test]
    fn a_name_is_collected_where_it_is_declared_and_not_where_it_is_used() {
        // A call site has no doc comment to carry what the name claims, and no scope of its
        // own. Collecting it made every caller of a named helper owe what the name owes.
        const SRC: &str = "fn helper_holds() {}\nfn caller() { helper_holds(); }\n";
        const WANTED: &str = "helper_holds";
        let lines: Vec<u32> = parse(SRC)
            .names
            .into_iter()
            .filter(|(_, n)| n == WANTED)
            .map(|(l, _)| l)
            .collect();
        assert_eq!(lines, vec![1], "the declaration only");
    }

    #[test]
    fn a_declared_variable_is_a_name() {
        const SRC: &str = "fn f() {\n    let value_seen = 1;\n}\n";
        const WANTED: &str = "value_seen";
        assert!(
            parse(SRC).names.iter().any(|(l, n)| *l == 2 && n == WANTED),
            "{:?}",
            parse(SRC).names
        );
    }

    #[test]
    fn a_name_is_collected_because_prose_cannot_reach_it() {
        let p = parse(NAMED);
        assert!(
            p.names.iter().any(|(l, n)| *l == 1 && n == WANTED),
            "{:?}",
            p.names
        );
    }

    #[test]
    fn a_doc_comment_arrives_without_its_marker() {
        let p = parse("/// One\n/// Two\nfn f() {}\n");
        assert_eq!(p.prose.len(), 1, "one contiguous run");
        assert_eq!(p.prose[0].text, "One\nTwo");
        assert_eq!(p.prose[0].lines, vec![1, 2]);
    }

    #[test]
    fn a_blank_line_ends_a_comment_run() {
        let p = parse("/// One\n\n/// Two\nfn f() {}\n");
        assert_eq!(p.prose.len(), 2);
        assert_eq!(p.prose[0].lines, vec![1]);
        assert_eq!(p.prose[1].lines, vec![3]);
    }

    #[test]
    fn a_blockquote_across_doc_lines_is_one_region() {
        // The reason a run is the unit: judged line by line this is three fragments.
        const SRC: &str = "/// per the note:\n/// > A line.\n/// > continued here.\nfn f() {}\n";
        let p = parse(SRC);
        assert_eq!(p.prose.len(), 1);
        const WANT: &str = "> A line.\n> continued here.";
        assert!(p.prose[0].text.contains(WANT));
    }

    /// A token the tests look for.
    const TOKEN: &str = "token-42";

    #[test]
    fn a_named_string_is_test_data_and_an_unnamed_one_is_a_message() {
        // Bound to a name: a fixture. Outside the checker that is an address, a formatted
        // figure, a line of output a parser is fed — data, whatever it looks like.
        const FIXTURE: &str = "fn f() {\n    let s = \"token-42\";\n}\n";
        let fixture = parse(FIXTURE);
        assert!(
            fixture.prose.iter().all(|r| !r.text.contains(TOKEN)),
            "{:?}",
            fixture.prose
        );
        // Not bound: an assertion's message, written for a person to read, and prose like
        // any other claim.
        const MESSAGE: &str = "fn f() {\n    assert!(x, \"token-42 orders them\");\n}\n";
        let message = parse(MESSAGE);
        assert!(
            message.prose.iter().any(|r| r.text.contains(TOKEN)),
            "{:?}",
            message.prose
        );
        // An argument is not the binding's value: the message is prose even here.
        const INSIDE: &str = "fn f() {\n    let x = y.expect(\"token-42 orders them\");\n}\n";
        let inside = parse(INSIDE);
        assert!(
            inside.prose.iter().any(|r| r.text.contains(TOKEN)),
            "{:?}",
            inside.prose
        );
    }

    #[test]
    fn code_that_is_not_a_string_is_never_prose() {
        const SRC: &str = "fn f() -> f64 {\n    100.4 * ratio\n}\n";
        let p = parse(SRC);
        assert!(p.prose.is_empty(), "{:?}", p.prose);
    }

    #[test]
    fn a_formatted_return_type_is_not_a_blockquote() {
        // The recorded defect: rustfmt breaks a long signature so a line opens with `>`, and a
        // line-oriented scanner reads it as verbatim rule text.
        const SRC: &str = "fn f()\n-> Result<(usize, usize), E>\n{\n    todo!()\n}\n";
        let p = parse(SRC);
        assert!(p
            .prose
            .iter()
            .all(|r| !r.text.trim_start().starts_with('>')));
    }

    #[test]
    fn an_item_scope_reaches_up_over_its_doc_comment_and_attributes() {
        // The claim in the doc comment and the name below it must land in ONE scope, or a
        // quote written where the convention puts it discharges nothing.
        const SRC: &str = "/// documented\n#[test]\nfn f() {}\n";
        let p = parse(SRC);
        let s = p.scope_at(1).expect("a scope holds the doc line");
        assert_eq!(s.kind, ScopeKind::Item);
        assert_eq!((s.first, s.last), (1, 3));
    }

    #[test]
    fn an_item_scope_does_not_reach_up_into_the_previous_item() {
        // The bound the floor exists for. The walk-up climbs over ATTACHED lines, and every
        // comment line is attached — including one trailing the previous item on its own last
        // line. Without the floor `b` reaches back over it and swallows `a` entirely, and
        // `scope_at` takes the last maximum, so a claim written inside `a` is judged against
        // `b`: the quote and the claim end up in different scopes, which is the failure the
        // regime exists to catch.
        //
        // The failure is CONTAINMENT of a sibling, not a partial overlap. A scan of all 75
        // `.rs` files in this workspace for partial overlaps returns zero with the floor and
        // zero without it, so a scan written that way reports the bound as dead code.
        const SRC: &str = "fn a() {} // trailing\n/// doc for b\nfn b() {}\n";
        let p = parse(SRC);
        assert_eq!(
            p.scope_at(1).map(|s| s.name.as_str()),
            Some("a"),
            "line 1 belongs to the item written on it"
        );
        let b = p
            .scopes
            .iter()
            .find(|s| s.name == "b")
            .expect("b has a scope");
        assert_eq!((b.first, b.last), (2, 3), "b starts at its own doc comment");
    }

    #[test]
    fn a_nested_item_reaches_up_over_its_doc_comment_too() {
        // The enclosing item ENDS after the nested one begins, so a floor taken over every
        // earlier item — rather than over the preceding SIBLINGS — sits above the nested
        // item's own first line and the walk-up never runs. The doc comment then lands in the
        // enclosing scope while the name lands in the inner one, which is the single case the
        // walk-up exists to prevent. Reached constantly in practice: a test function inside
        // `mod tests` is where the identifier form of a rule marker belongs, and that form owes
        // its quote in the doc comment above the name.
        const SRC: &str = "mod tests {\n    /// documented\n    fn f() {}\n}\n";
        let p = parse(SRC);
        let s = p.scope_at(2).expect("a scope holds the doc line");
        assert_eq!(s.kind, ScopeKind::Item);
        assert_eq!(s.name, "f", "the doc line belongs to the item it documents");
        assert_eq!((s.first, s.last), (2, 3));
    }

    #[test]
    fn an_item_scope_spans_the_item_and_nests() {
        let p = parse("fn outer() {\n    fn inner() {}\n}\n");
        let names: Vec<&str> = p
            .scopes
            .iter()
            .filter(|s| s.kind == ScopeKind::Item)
            .map(|s| s.name.as_str())
            .collect();
        assert_eq!(names, vec!["outer", "inner"]);
        assert_eq!(p.scope_at(2).map(|s| s.name.as_str()), Some("inner"));
    }

    #[test]
    fn every_line_has_a_scope_even_outside_an_item() {
        let p = parse("//! A module doc.\n\nfn f() {}\n");
        assert_eq!(p.scope_at(1).map(|s| s.kind), Some(ScopeKind::Preamble));
    }

    #[test]
    fn a_fence_in_a_doc_comment_is_recognised() {
        // The line-oriented scanner tested for three backticks at the start of a RAW line,
        // which a doc-comment fence never is, so every doc example was live text.
        const SRC: &str = "/// ```\n/// 100.1 not a citation\n/// ```\nfn f() {}\n";
        let p = parse(SRC);
        assert_eq!(p.fenced, vec![1, 2, 3]);
    }

    #[test]
    fn a_file_the_grammar_cannot_parse_says_so() {
        assert!(parse("fn f() {}\n").trouble.is_none());
        const BROKEN: &str = "fn f( {{{ unclosed\n";
        assert!(parse(BROKEN).trouble.is_some());
    }

    #[test]
    fn a_file_too_deep_to_walk_says_so_instead_of_aborting() {
        // Unbounded, this overflowed the stack and killed the whole run with no file named.
        let deep = format!(
            "fn f() -> i32 {{ {}1{} }}\n",
            "(".repeat(20_000),
            ")".repeat(20_000)
        );
        assert!(parse(&deep).trouble.is_some());
    }

    #[test]
    fn a_slug_in_a_source_comment_defines_nothing() {
        let p = parse("/// see the decision\nfn f() {}\n");
        assert!(p.prose.iter().all(|r| !r.structural));
    }
}
