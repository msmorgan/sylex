//! The `.fug` parser's own rejections: each malformed input fails with a
//! `LoadError::Format` that names the defect and its line and column.

use sylex_syntax::Grammar;
use sylex_syntax::LoadError;

/// A well-formed base each case appends one item to.
const BASE: &str = "root S\ncategory S\n";

fn error(item: &str) -> String {
    match Grammar::from_fug_str(&format!("{BASE}{item}\n")) {
        Err(LoadError::Format(message)) => message,
        other => panic!("{item}: expected a format error, got {other:?}"),
    }
}

#[test]
fn malformed_items_are_rejected_naming_the_defect() {
    for (item, expected) in [
        ("A: S -> B { color: [head] }", "3:13: unknown field `color`"),
        (
            "A: S -> B { cost: 2, cost: 3 }",
            "3:22: duplicate field `cost`",
        ),
        ("A: S -> B { transparent }", "`transparent` takes a value"),
        (
            "word Of = \"prep:of\" { gap }",
            "this item takes no `{ ... }`",
        ),
        ("category S", "`S` is declared twice"),
        ("root T", "`root` is given twice"),
        ("feature F = A | B | A", "3:21: `A` is listed twice"),
        (
            "feature F = A | B { holder: A }",
            "a feature's settings are `local`, a license's `head: Value` and `mixed: Value`, each once",
        ),
        (
            "feature F = A | B { head: B, head: A }",
            "a feature's settings are `local`, a license's `head: Value` and `mixed: Value`, each once",
        ),
        ("category T [F, G, F]", "`F` is listed twice"),
        (
            "A: S -> B { heavy: [head, head] }",
            "`head` is listed twice",
        ),
        ("A: S -> Shout(B)", "3:9: unknown variant `Shout`"),
        ("A: S -> Class(B, C)", "3:18: one argument too many"),
        ("A: S -> Lex(Nown)", "unknown variant `Nown`"),
        ("A: S -> head:','", "3:14: punctuation names no child"),
        (
            "A: S -> B where B = C",
            "expected `$.Feature` or `child.Feature`",
        ),
        ("A: S -> B where $.F := head.G", "3:24: expected a name"),
        ("A: S -> B where $.F = G,", "item ends too early"),
        // A name is a name: no path, character or quoted string.
        ("A: S -> B { heavy: [$.F] }", "3:21: a path is no name"),
        ("A: S -> B { heavy: ['x'] }", "3:21: a character is no name"),
        ("A: S -> Class('c')", "3:15: a character is no name"),
        ("feature G = $.A | B", "3:13: a path is no name"),
        ("A: S -> B where $.F = 'x'", "3:23: expected a name"),
        (
            "adjunct X { host: H, side: Before, host_kind: Predicate, delimiter: a }",
            "expected a character",
        ),
        // An equation is written as one, never as its variant.
        (
            "A: S -> B where Require(B, C)",
            "3:17: expected an equation",
        ),
        (
            "A: S -> B where !(B = C & $.F = A)",
            "3:19: expected `$.Feature` or `child.Feature`",
        ),
        (
            "A: S -> B where !($.F -> A)",
            "a joint exclusion is `!(path = A & ...)`",
        ),
        ("A: S -> B where !($.F = head.G)", "3:25: expected a name"),
        ("A: S -> B where $.F != head.G", "3:24: expected a name"),
        // A pair is `A -> B`.
        (
            "valence V { head: Verb, agree: [A = B] }",
            "expected a pair: `A -> B`",
        ),
        // What the item's own syntax fills is no setting.
        (
            "category T { category: P }",
            "`category` is written in the item's own syntax",
        ),
        (
            "A: S -> B { equations: [$.F = A] }",
            "`equations` is written in the item's own syntax",
        ),
        (
            "A: S -> B { name: C }",
            "`name` is written in the item's own syntax",
        ),
        (
            "premodification M { head: Verb, host: H, forms: [PastParticiple] } where $.F = A",
            "this item takes no `where`",
        ),
        (
            "valence V { head: Verb } where $.F = A",
            "this item takes no `where`",
        ),
        ("A: S -> 'ab'", "malformed character literal"),
        ("word Of = \"prep:of", "unterminated string"),
        ("A: S -> B ~", "unexpected character `~`"),
        (
            "a: S -> B",
            "`a` is no keyword; a Production's name is capitalized",
        ),
        ("frobnicate S", "unknown keyword `frobnicate`"),
        ("slot Np = S", "unknown variant `Np`"),
        ("coordination S { serial: maybe }", "expected a boolean"),
        (
            "high_attachment S where $.F != G",
            "a high attachment is `where $.Feature = Value`",
        ),
    ] {
        let message = error(item);
        assert!(message.contains(expected), "{item}: {message}");
    }
}

#[test]
fn items_start_at_the_base_indentation() {
    // An indented grammar parses: its first token sets the base.
    let indented = "    root S\n    category S\n    A: S ->\n        S\n";
    assert!(Grammar::from_fug_str(indented).is_ok());
    // A line indented less than the first item is an error.
    let shallow = "    root S\n  category S\n";
    assert!(matches!(
        Grammar::from_fug_str(shallow),
        Err(LoadError::Format(message)) if message.contains("indented less than the first item")
    ));
    // An identifier at the base column starts the next item, so an item
    // that needs more cannot continue there.
    assert!(matches!(
        Grammar::from_fug_str("root S\ncategory S\nA: S ->\nS\n"),
        Err(LoadError::Format(message)) if message.contains("4:1: a Production is `Name: Lhs -> ...`")
    ));
}

#[test]
fn a_grammar_needs_a_root() {
    assert!(matches!(
        Grammar::from_fug_str("category S\n"),
        Err(LoadError::Format(message)) if message.contains("no `root` item")
    ));
}

/// A Production's cases over `head.F`: one declaration per case, each the
/// Production's with its case's equations and props added.
#[test]
fn cases_are_one_production_each() {
    let grammar = Grammar::from_fug_str(
        "root S\ncategory S\nfeature F = A | B\nfeature P = Marked\n\
         X: S -> head:S { transparent: [head], cases: head.F -> [\n    \
         A -> [head.F = A, $.F = A],\n    \
         B -> { heavy: [head], equations: [head.F := B] },\n] }\n    \
         where $.F = head.F\n\
         Y: S -> head:S { cases: head.P -> [Marked -> [head.P := Marked], Unvalued -> [head.P != Marked]] }\n",
    )
    .unwrap();
    let declarations: Vec<_> = grammar
        .productions
        .iter()
        .map(|p| {
            (
                p.name.to_string(),
                p.equations.len(),
                p.heavy.len(),
                p.transparent.len(),
            )
        })
        .collect();
    assert_eq!(
        declarations,
        [
            ("X/A".to_owned(), 3, 0, 1),
            ("X/B".to_owned(), 2, 1, 1),
            ("Y/Marked".to_owned(), 1, 0, 0),
            ("Y/Unvalued".to_owned(), 1, 0, 0),
        ]
    );
}

#[test]
fn malformed_cases_are_rejected_naming_the_defect() {
    const FEATURES: &str = "feature F = A | B\nfeature P = Marked\n";
    for (item, expected) in [
        (
            "X: S -> head:S { cases: head.F -> [A -> [head.F = A], C -> [head.F = C]] }",
            "case `C` is no value of `F`",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> [head.F = A], A -> [head.F := A], B -> [head.F = B]] }",
            "case `A` is stated twice",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> [head.F = A]] }",
            "cases over `head.F` leave `B` out",
        ),
        (
            "X: S -> head:S { cases: head.P -> [Marked -> [head.P := Marked]] }",
            "cases over `head.P` leave `Unvalued` out",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> [head.F = A], B -> [$.F = B]] }",
            "case `B` states no guard `head.F = B` or `head.F := B`",
        ),
        (
            "X: S -> head:S { cases: head.P -> [Marked -> [head.P := Marked], Unvalued -> [$.P = Marked]] }",
            "case `Unvalued` states no guard `head.P != Marked`",
        ),
        (
            "X: S -> head:S { cases: head.G -> [A -> [head.G = A]] }",
            "cases over `head.G`: no feature `G`",
        ),
        (
            "X: S -> head:S { cases: [A -> [head.F = A]] }",
            "cases are `cases: path -> [Value -> [eq, ...], ...]`",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> { lhs: S }, B -> [head.F = B]] }",
            "`lhs` is the Production's, not a case's",
        ),
        (
            "X: S -> head:S { cost: 2, cases: head.F -> [A -> { cost: 3, equations: [head.F = A] }, B -> [head.F = B]] }",
            "`cost` is set on the Production and on its case",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> { tint: [head], equations: [head.F = A] }, B -> [head.F = B]] }",
            "unknown field `tint`",
        ),
        (
            "X: S -> head:S { cases: head.F -> [head.F -> [head.F = A], B -> [head.F = B]] }",
            "5:36: a case is named by a value of its feature",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> B, B -> [head.F = B]] }",
            "5:41: a case is `[eq, ...]` or `{ ... }`",
        ),
        (
            "X: S -> head:S { cases: head.F -> [A -> { equations }, B -> [head.F = B]] }",
            "`equations` takes a value",
        ),
        (
            "X: S -> head:S { cost: 2, cases: head.F -> [A -> { cost: [3], equations: [head.F = A] }, B -> [head.F = B]] }",
            "`cost` is set on the Production and on its case",
        ),
        (
            "X: S -> head:S { transparent: [head], cases: head.F -> [A -> { transparent: head, equations: [head.F = A] }, B -> [head.F = B]] }",
            "`transparent` is a list on the Production",
        ),
        (
            "X: S -> head:S { transparent: [head], cases: head.F -> [A -> { transparent: [], equations: [head.F = A] }, B -> [head.F = B]] }",
            "the case's `transparent` adds nothing",
        ),
    ] {
        let message = error(&format!("{FEATURES}{item}"));
        assert!(message.contains(expected), "{item}: {message}");
    }
}
