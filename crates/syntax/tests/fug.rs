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
        (
            "A: S -> B { colour: [head] }",
            "3:13: unknown setting `colour`",
        ),
        (
            "A: S -> B { cost: 2, cost: 3 }",
            "3:22: `cost` is given twice",
        ),
        (
            "A: S -> B { unclosed: true }",
            "`unclosed` is a flag and takes no value",
        ),
        ("A: S -> B { transparent }", "`transparent` takes a value"),
        (
            "word Of = \"prep:of\" { gap }",
            "this item takes no `{ ... }`",
        ),
        ("category S", "`S` is declared twice"),
        ("root T", "`root` is given twice"),
        ("feature F = A | B | A", "3:21: `A` is listed twice"),
        ("category T [F, G, F]", "`F` is listed twice"),
        (
            "A: S -> B { heavy: [head, head] }",
            "`head` is listed twice",
        ),
        ("A: S -> Shout(B)", "unknown symbol kind `Shout`"),
        ("A: S -> Class(B, C)", "`Class` takes no more arguments"),
        ("A: S -> Lex(Nown)", "unknown variant `Nown`"),
        ("A: S -> head:','", "expected a right-hand-side symbol"),
        (
            "A: S -> B where B = C",
            "expected `$.Feature` or `child.Feature`",
        ),
        ("A: S -> B where $.F := head.G", "malformed equation"),
        ("A: S -> B where $.F = G,", "item ends too early"),
        ("A: S -> 'ab'", "malformed character literal"),
        ("word Of = \"prep:of", "unterminated string"),
        ("A: S -> B ~", "unexpected character `~`"),
        (
            "a: S -> B",
            "`a` is no keyword; a Production's name is capitalized",
        ),
        ("frobnicate S", "unknown keyword `frobnicate`"),
        ("slot Np = S", "unknown variant `Np`"),
        (
            "coordination S { serial: maybe }",
            "expected `true` or `false`",
        ),
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
