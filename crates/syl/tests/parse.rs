//! Focused tests of the `.syl` parser.
//!
//! Most tests compare a compact rendering of the tree: each item on its own
//! line or separated by `;`, comments as `#text`, and `^` marking an item
//! with a blank line before it.

use std::fmt::Write as _;

use sylex_syl::Entry;
use sylex_syl::Error;
use sylex_syl::ErrorKind;
use sylex_syl::File;
use sylex_syl::FileId;
use sylex_syl::Item;
use sylex_syl::LineCol;
use sylex_syl::Shape;
use sylex_syl::Sort;
use sylex_syl::Value;
use sylex_syl::ValueKind;
use sylex_syl::parse;

const FILE: FileId = FileId(7);

fn ok(source: &str) -> File {
    parse(FILE, source).unwrap_or_else(|e| panic!("{}", e.render("test", source)))
}

fn err(source: &str) -> Error {
    match parse(FILE, source) {
        Ok(file) => panic!("expected an error, parsed {file:?}"),
        Err(error) => error,
    }
}

fn items<T>(items: &[Item<T>], node: impl Fn(&T) -> String) -> String {
    items
        .iter()
        .map(|item| {
            let blank = if item.blank_line_before { "^" } else { "" };
            match &item.entry {
                Entry::Comment(c) => format!("{blank}#{}", c.text),
                Entry::Node(n) => format!("{blank}{}", node(n)),
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn sort(sort: &Sort) -> String {
    if sort.args.is_empty() {
        sort.head.ident.to_string()
    } else {
        let args: Vec<_> = sort.args.iter().map(self::sort).collect();
        format!("{}({})", sort.head.ident, args.join(", "))
    }
}

fn value(value: &Value) -> String {
    match &value.kind {
        ValueKind::String(s) => format!("{s:?}"),
        ValueKind::Integer(n) => n.to_string(),
        ValueKind::List(list) => format!("[{}]", items(list, self::value)),
        ValueKind::Record(record) => format!(
            "{{{}}}",
            items(&record.fields, |f| format!(
                "{} = {}",
                f.name.ident,
                self::value(&f.value)
            ))
        ),
        ValueKind::Parameter(name) => format!("${}", name.ident),
        ValueKind::Call(call) if call.is_bare() => call.head.ident.to_string(),
        ValueKind::Call(call) => format!(
            "{}({})",
            call.head.ident,
            items(&call.args, |a| match a.name {
                Some(name) => format!("{} = {}", name.ident, self::value(&a.value)),
                None => self::value(&a.value),
            })
        ),
    }
}

fn render(file: &File) -> String {
    file.items
        .iter()
        .map(|item| {
            let blank = if item.blank_line_before { "^" } else { "" };
            let text = match &item.entry {
                Entry::Comment(c) => format!("#{}", c.text),
                Entry::Node(d) => match &d.shape {
                    Shape::Nameless(v) => format!("{} <{}>", d.kind.ident, value(v)),
                    Shape::Named(n) => {
                        let mut out = format!("{} {}", d.kind.ident, n.name.ident);
                        if !n.params.is_empty() {
                            let params = items(&n.params, |p| match &p.default {
                                Some(v) => {
                                    format!("{}: {} = {}", p.name.ident, sort(&p.sort), value(v))
                                }
                                None => format!("{}: {}", p.name.ident, sort(&p.sort)),
                            });
                            let _ = write!(out, "({params})");
                        }
                        if let Some(s) = &n.sort {
                            let _ = write!(out, ": {}", sort(s));
                        }
                        if let Some(c) = &n.constructor {
                            let _ = write!(out, " = {}", c.ident);
                        }
                        if let Some(body) = &n.body {
                            let _ = write!(
                                out,
                                " {{{}}}",
                                items(&body.fields, |f| format!(
                                    "{} = {}",
                                    f.name.ident,
                                    value(&f.value)
                                ))
                            );
                        }
                        out
                    }
                },
            };
            format!("{blank}{text}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn check(source: &str, expected: &str) {
    assert_eq!(render(&ok(source)), expected, "source:\n{source}");
}

fn check_err(source: &str, kind: &ErrorKind, at: &str) {
    let error = err(source);
    assert_eq!(&error.kind, kind, "source:\n{source}");
    assert_eq!(error.span.text(source), at, "source:\n{source}");
    assert_eq!(error.span.file, FILE);
}

// --- named header ---------------------------------------------------------

#[test]
fn named_header_with_sort_and_no_body() {
    check("primitive actor: NounPhrase", "primitive actor: NounPhrase");
}

#[test]
fn named_header_with_every_part() {
    check(
        "primitive actionSearch(scope: Location, for: Option(List(Cost)) = none): Action = Search { lexical = {} }",
        "primitive actionSearch(scope: Location; for: Option(List(Cost)) = none): Action = Search {lexical = {}}",
    );
}

#[test]
fn named_header_with_params_and_body_but_no_sort() {
    check(
        "action proliferate(times: Amount = 1) { body = x }",
        "action proliferate(times: Amount = 1) {body = x}",
    );
}

#[test]
fn named_header_with_body_only() {
    check(
        "counter aegisCounter { lemma = \"aegis\" }",
        "counter aegisCounter {lemma = \"aegis\"}",
    );
}

#[test]
fn named_header_with_constructor_and_no_params() {
    check(
        "primitive itself: NounPhrase = Itself",
        "primitive itself: NounPhrase = Itself",
    );
}

#[test]
fn named_header_with_name_only_before_next_declaration_and_at_eof() {
    check(
        "relation attached\nrelation paired",
        "relation attached\nrelation paired",
    );
}

#[test]
fn shape_does_not_depend_on_kind_word() {
    // A word followed by `(` and `ident :` is a named header for any kind,
    // and a word followed by `(` and anything else is a call value.
    check("card f(x: X)", "card f(x: X)");
    check("template f(x)", "template <f(x)>");
    check("template f(x = y)", "template <f(x = y)>");
}

// --- nameless header ------------------------------------------------------

#[test]
fn nameless_record() {
    check(
        "card {\n  name = \"Ancestral Recall\",\n}",
        "card <{name = \"Ancestral Recall\"}>",
    );
}

#[test]
fn nameless_call() {
    check(
        "card split({ left = { name = \"Fire\" }, right = { name = \"Ice\" } })",
        "card <split({left = {name = \"Fire\"}; right = {name = \"Ice\"}})>",
    );
}

// --- values ---------------------------------------------------------------

#[test]
fn arguments_keep_written_order_and_names() {
    check(
        "template t: X { body = f(a, to = b, c, from = g(h)) }",
        "template t: X {body = f(a; to = b; c; from = g(h))}",
    );
}

#[test]
fn named_arguments_may_stand_anywhere_among_positional_ones() {
    // Binding positional arguments to parameters is resolution's job; the
    // parser only keeps each argument, named or not, where it was written.
    check(
        "template t: X { body = when(does(actor(thisAura), enter), if = is(it, in(battlefield)), sequentially([a])) }",
        "template t: X {body = when(does(actor(thisAura); enter); if = is(it; in(battlefield)); sequentially([a]))}",
    );
    check(
        "template t: X { body = f(to = b, a, from = c, d) }",
        "template t: X {body = f(to = b; a; from = c; d)}",
    );
}

#[test]
fn parameter_references_are_values() {
    check(
        "template t(a: X, b: X = $a): X { body = f($a, to = [$b], c = { d = $_e1 }) }",
        "template t(a: X; b: X = $a): X {body = f($a; to = [$b]; c = {d = $_e1})}",
    );
}

#[test]
fn parameter_reference_span_includes_the_sigil() {
    let source = "template t(a: X): X { body = f( $amount ) }";
    let file = ok(source);
    let Shape::Named(named) = &file.declarations().next().unwrap().shape else {
        panic!("expected a named declaration");
    };
    let body = &named.body.as_ref().unwrap().nodes().next().unwrap().value;
    let ValueKind::Call(call) = &body.kind else {
        panic!("expected a call");
    };
    let arg = &call.args[0].node().unwrap().value;
    let ValueKind::Parameter(name) = &arg.kind else {
        panic!("expected a parameter reference");
    };
    assert_eq!(name.ident.as_str(), "amount");
    assert_eq!(name.span.text(source), "$amount");
    assert_eq!(arg.span.text(source), "$amount");
}

#[test]
fn lone_dollar_is_an_error() {
    check_err("card { a = $ }", &ErrorKind::LoneDollar, "$");
    check_err("card { a = $", &ErrorKind::LoneDollar, "$");
    check_err("card { a = $ name }", &ErrorKind::LoneDollar, "$");
    check_err("card { a = $1 }", &ErrorKind::LoneDollar, "$");
    check_err("card { a = $\"x\" }", &ErrorKind::LoneDollar, "$");
    check_err("card { a = $.x }", &ErrorKind::LoneDollar, "$");
}

#[test]
fn parameter_reference_takes_no_arguments() {
    check_err("card { a = $f(1) }", &ErrorKind::ParameterArguments, "(");
    check_err("card { a = $f() }", &ErrorKind::ParameterArguments, "(");
}

#[test]
fn parameter_reference_follows_the_word_rule() {
    check_err("card { a = $x.y }", &ErrorKind::DottedWord, "$x.y");
}

#[test]
fn labels_field_names_and_headers_are_plain_words() {
    check_err(
        "card { a = f($x = 1) }",
        &ErrorKind::Expected {
            expected: "',' or ')'",
            found: "'='".to_owned(),
        },
        "=",
    );
    check_err(
        "card { $a = 1 }",
        &ErrorKind::Expected {
            expected: "a field name",
            found: "parameter `$a`".to_owned(),
        },
        "$a",
    );
    check_err(
        "template t($a: X): X",
        &ErrorKind::Expected {
            expected: "a parameter name",
            found: "parameter `$a`".to_owned(),
        },
        "$a",
    );
    check_err(
        "template t(a: X, $b: X): X",
        &ErrorKind::Expected {
            expected: "a parameter name",
            found: "parameter `$b`".to_owned(),
        },
        "$b",
    );
}

#[test]
fn true_false_and_none_are_bare_words() {
    let file = ok("template t: X { body = f(true, false, none) }");
    let rendered = render(&file);
    assert_eq!(rendered, "template t: X {body = f(true; false; none)}");
}

#[test]
fn negative_and_extreme_integers() {
    check(
        "template t: X { body = [0, -1, 42, -9223372036854775808, 9223372036854775807] }",
        "template t: X {body = [0; -1; 42; -9223372036854775808; 9223372036854775807]}",
    );
    check_err(
        "template t: X { body = 9223372036854775808 }",
        &ErrorKind::IntegerOutOfRange,
        "9223372036854775808",
    );
    check_err(
        "template t: X { body = 12ab }",
        &ErrorKind::InvalidInteger,
        "12ab",
    );
}

#[test]
fn string_escapes_are_unescaped() {
    let file = ok(r#"template t: X { body = "say \"hi\" \\ bye" }"#);
    let Shape::Named(named) = &file.declarations().next().unwrap().shape else {
        panic!("named");
    };
    let field = named.body.as_ref().unwrap().nodes().next().unwrap();
    assert_eq!(
        field.value.kind,
        ValueKind::String(r#"say "hi" \ bye"#.to_owned())
    );
}

#[test]
fn bad_strings_are_errors() {
    check_err(
        r#"card { name = "a\nb" }"#,
        &ErrorKind::UnsupportedEscape('n'),
        r"\n",
    );
    check_err(
        "card { name = \"abc",
        &ErrorKind::UnterminatedString,
        "\"abc",
    );
}

#[test]
fn trailing_commas_everywhere() {
    check(
        "primitive p(a: Option(X,), b: Y,): Z { f = [1, 2,], g = h(i, j = k,), }",
        "primitive p(a: Option(X); b: Y): Z {f = [1; 2]; g = h(i; j = k)}",
    );
}

#[test]
fn empty_list_and_record() {
    check("card { a = [], b = {} }", "card <{a = []; b = {}}>");
    check("card {}", "card <{}>");
}

#[test]
fn sorts_nest() {
    let file = ok("primitive p(a: Option(List(Cost)), b: Pair(A, B)): X");
    let Shape::Named(named) = &file.declarations().next().unwrap().shape else {
        panic!("named");
    };
    let params: Vec<_> = named.params.iter().filter_map(Item::node).collect();
    assert_eq!(sort(&params[0].sort), "Option(List(Cost))");
    assert_eq!(params[0].sort.args[0].args[0].head.ident, "Cost");
    assert_eq!(sort(&params[1].sort), "Pair(A, B)");
}

// --- errors ---------------------------------------------------------------

#[test]
fn empty_call_is_an_error() {
    check_err("card { a = head() }", &ErrorKind::EmptyCall, "()");
}

#[test]
fn empty_parameter_list_is_an_error() {
    check_err("template name(): X", &ErrorKind::EmptyParameterList, "()");
}

#[test]
fn empty_sort_arguments_are_an_error() {
    check_err(
        "primitive p: Option()",
        &ErrorKind::EmptySortArguments,
        "()",
    );
}

#[test]
fn stray_minus_is_an_error() {
    check_err("card { a = - 1 }", &ErrorKind::StrayMinus, "-");
    check_err("card { a = -x }", &ErrorKind::StrayMinus, "-");
}

#[test]
fn dotted_words_are_errors() {
    check_err("card { a = foo.bar }", &ErrorKind::DottedWord, "foo.bar");
    check_err("primitive p: X = A.b", &ErrorKind::DottedWord, "A.b");
}

#[test]
fn a_constructor_clause_is_a_capitalized_word_without_a_dot() {
    check_err(
        "primitive p: X = .foo",
        &ErrorKind::UnexpectedChar('.'),
        ".",
    );
    check_err(
        "primitive p: X = foo",
        &ErrorKind::UncapitalizedConstructor("foo".to_owned()),
        "foo",
    );
    check_err(
        "primitive p: X = $foo",
        &ErrorKind::Expected {
            expected: "a constructor",
            found: "parameter `$foo`".to_owned(),
        },
        "$foo",
    );
}

#[test]
fn capitalized_words_outside_sorts_and_constructor_clauses_are_errors() {
    let capitalized = |word: &str| ErrorKind::CapitalizedWord(word.to_owned());
    check_err("Card { a = b }", &capitalized("Card"), "Card");
    check_err("primitive Power: Stat = Power", &capitalized("Power"), "Power");
    check_err("template t(X: A): B", &capitalized("X"), "X");
    check_err("template t: B { Body = a }", &capitalized("Body"), "Body");
    check_err("template t: B { body = Power }", &capitalized("Power"), "Power");
    check_err("template t: B { body = f(Power) }", &capitalized("Power"), "Power");
    check_err("template t: B { body = f(X = a) }", &capitalized("X"), "X");
    check_err("template t(x: A): B { body = $X }", &capitalized("X"), "$X");
    check_err("template t(x: A = B): B", &capitalized("B"), "B");
    check_err("card { text = [Activated] }", &capitalized("Activated"), "Activated");
}

#[test]
fn capitalized_words_are_allowed_in_sorts_constructor_clauses_and_metadata() {
    check(
        "primitive p(x: Option(List(Cost))): Ability = Activated",
        "primitive p(x: Option(List(Cost))): Ability = Activated",
    );
    check(
        "template t: B { lexical = { frame = Monotransitive(Verb) }, body = a }",
        "template t: B {lexical = {frame = Monotransitive(Verb)}; body = a}",
    );
    check(
        "template t: B { spelling = [Word, \"x\"], body = f(spelling = a) }",
        "template t: B {spelling = [Word; \"x\"]; body = f(spelling = a)}",
    );
    // The exemption covers a metadata field's value only, not its siblings.
    check_err(
        "template t: B { lexical = A, body = B }",
        &ErrorKind::CapitalizedWord("B".to_owned()),
        "B",
    );
}

#[test]
fn malformed_input_is_located_by_span() {
    let source = "card {\n  name = \"x\",\n  types = [a b],\n}\n";
    let error = err(source);
    assert_eq!(
        error.kind,
        ErrorKind::Expected {
            expected: "',' or ']'",
            found: "word `b`".to_owned()
        }
    );
    assert_eq!(error.span.text(source), "b");
    assert_eq!(
        error.span.start_line_col(source),
        LineCol {
            line: 3,
            column: 14
        }
    );
    assert_eq!(
        error.render("f.syl", source),
        "f.syl:3:14: expected ',' or ']', found word `b`"
    );
}

#[test]
fn unexpected_characters_are_errors() {
    check_err("card { a = b } ;", &ErrorKind::UnexpectedChar(';'), ";");
    check_err("card { a = b → c }", &ErrorKind::UnexpectedChar('→'), "→");
    check_err(
        "card { a = b } }",
        &ErrorKind::Expected {
            expected: "a declaration",
            found: "'}'".to_owned(),
        },
        "}",
    );
}

// --- comments -------------------------------------------------------------

#[test]
fn comments_in_a_list_including_before_the_closing_bracket() {
    check(
        "card { a = [\n  # first\n  x,\n  # middle\n  y,\n  # last\n] }",
        "card <{a = [# first; x; # middle; y; # last]}>",
    );
}

#[test]
fn comments_in_a_record_including_before_the_closing_brace() {
    check(
        "card {\n  # first\n  a = 1,\n  # middle\n  b = 2,\n  # last\n}",
        "card <{# first; a = 1; # middle; b = 2; # last}>",
    );
}

#[test]
fn comments_in_an_argument_list_including_before_the_closing_paren() {
    check(
        "card { a = f(\n  # first\n  x,\n  # middle\n  to = y,\n  # last\n) }",
        "card <{a = f(# first; x; # middle; to = y; # last)}>",
    );
}

#[test]
fn comments_in_a_parameter_list_including_before_the_closing_paren() {
    check(
        "template t(\n  # first\n  a: A,\n  # middle\n  b: B = c,\n  # last\n): X",
        "template t(# first; a: A; # middle; b: B = c; # last): X",
    );
}

#[test]
fn comments_at_file_level_including_end_of_file() {
    check(
        "# first\nrelation a\n# middle\nrelation b\n# last\n",
        "# first\nrelation a\n# middle\nrelation b\n# last",
    );
    check("# only a comment", "# only a comment");
    check("", "");
}

#[test]
fn comment_after_last_item_without_trailing_comma() {
    check("card { a = [x # note\n] }", "card <{a = [x; # note]}>");
}

#[test]
fn same_line_comment_becomes_the_next_item() {
    check(
        "card { a = [x, # note\n  y] }",
        "card <{a = [x; # note; y]}>",
    );
    check(
        "card { a = 1, # note\n  b = 2 }",
        "card <{a = 1; # note; b = 2}>",
    );
}

#[test]
fn comment_between_field_equals_and_value_moves_before_the_field() {
    check(
        "card {\n  a = 1,\n  b = # note\n    f(x),\n}",
        "card <{a = 1; # note; b = f(x)}>",
    );
}

#[test]
fn comment_inside_a_header_moves_before_the_declaration() {
    check(
        "relation a\nprimitive # note\n  p: X",
        "relation a\n# note\nprimitive p: X",
    );
    check("card # note\n{ a = 1 }", "# note\ncard <{a = 1}>");
}

#[test]
fn comment_inside_a_sort_moves_before_its_parameter() {
    check(
        "template t(a: Option( # note\n  X), b: B): Y",
        "template t(# note; a: Option(X); b: B): Y",
    );
}

#[test]
fn comment_inside_a_call_head_moves_before_the_argument() {
    check(
        "card { a = f(g # note\n  (x)) }",
        "card <{a = f(# note; g(x))}>",
    );
}

#[test]
fn blank_line_flag() {
    check(
        "\n# a\n\n# b\nrelation x\n\n\nrelation y\n\n# c\n",
        "^# a\n^# b\nrelation x\n^relation y\n^# c",
    );
    check(
        "card {\n  a = 1,\n\n  b = 2,\n  # c\n\n  # d\n}",
        "card <{a = 1; ^b = 2; # c; ^# d}>",
    );
}

#[test]
fn doc_comment_versus_detached_comment() {
    let source = "# detached\n\n# doc one\n# doc two\nrelation a\n# not doc\n\nrelation b\n\n# para\n\n# doc\nrelation c\n";
    let file = ok(source);
    let doc = |index: usize| -> Vec<&str> {
        file.doc_comment(index)
            .iter()
            .map(|c| c.text.as_str())
            .collect()
    };
    assert_eq!(
        render(&file),
        "# detached\n^# doc one\n# doc two\nrelation a\n# not doc\n^relation b\n^# para\n^# doc\nrelation c"
    );
    assert_eq!(doc(3), [" doc one", " doc two"]);
    assert_eq!(doc(5), Vec::<&str>::new());
    assert_eq!(doc(8), [" doc"]);
    // A comment item has no doc comment.
    assert_eq!(doc(2), Vec::<&str>::new());
}

#[test]
fn comment_text_is_verbatim() {
    let file = ok("#  two spaces [see 1.2] #inner\nrelation a");
    assert_eq!(
        file.items[0].comment().unwrap().text,
        "  two spaces [see 1.2] #inner"
    );
}

// --- spans ----------------------------------------------------------------

#[test]
fn spans_cover_their_nodes() {
    let source = "# doc\ntemplate t(a: List(X) = [1]): Y {\n  body = f(to = \"é\", 2),\n}\n";
    let file = ok(source);
    assert_eq!(file.items[0].comment().unwrap().span.text(source), "# doc");
    let declaration = file.declarations().next().unwrap();
    assert_eq!(declaration.span.file, FILE);
    assert!(declaration.span.text(source).starts_with("template t("));
    assert!(declaration.span.text(source).ends_with("2),\n}"));
    assert_eq!(declaration.kind.span.text(source), "template");
    let Shape::Named(named) = &declaration.shape else { panic!("named") };
    assert_eq!(named.name.span.text(source), "t");
    assert!(named.span.text(source).starts_with("t(a: "));
    assert!(named.span.text(source).ends_with("2),\n}"));
    assert_eq!(named.span.end, declaration.span.end);
    let bare = ok("primitive p(a: A): X = C\n# after\n");
    let Shape::Named(bare) = &bare.declarations().next().unwrap().shape else {
        panic!("named")
    };
    assert_eq!(
        bare.span.text("primitive p(a: A): X = C\n# after\n"),
        "p(a: A): X = C"
    );
    let param = named.params[0].node().unwrap();
    assert_eq!(param.span.text(source), "a: List(X) = [1]");
    assert_eq!(param.sort.span.text(source), "List(X)");
    assert_eq!(param.default.as_ref().unwrap().span.text(source), "[1]");
    assert_eq!(named.sort.as_ref().unwrap().span.text(source), "Y");
    let body = named.body.as_ref().unwrap();
    assert!(body.span.text(source).starts_with('{') && body.span.text(source).ends_with('}'));
    let field = body.nodes().next().unwrap();
    assert_eq!(field.span.text(source), "body = f(to = \"é\", 2)");
    let ValueKind::Call(call) = &field.value.kind else { panic!("call") };
    assert_eq!(call.span.text(source), "f(to = \"é\", 2)");
    let arg = call.args[0].node().unwrap();
    assert_eq!(arg.span.text(source), "to = \"é\"");
    assert_eq!(
        arg.value.span.start_line_col(source),
        LineCol {
            line: 3,
            column: 17
        }
    );
    assert_eq!(
        call.args[1].node().unwrap().span.start_line_col(source),
        LineCol {
            line: 3,
            column: 22
        }
    );
}

#[test]
fn hoisted_comment_takes_the_blank_line_flag_of_its_item() {
    check(
        "primitive a: X\n\nprimitive bar # inner\n  : Y\n",
        "primitive a: X\n^# inner\nprimitive bar: Y",
    );
    check(
        "card {\n  a = 1,\n\n  b = # inner\n    2,\n}",
        "card <{a = 1; ^# inner; b = 2}>",
    );
}

#[test]
fn comments_inside_a_header_are_not_doc_comments() {
    let source = "# doc\nprimitive bar # inner\n  : Y\n";
    let file = ok(source);
    assert_eq!(render(&file), "# doc\n# inner\nprimitive bar: Y");
    let doc: Vec<_> = file
        .doc_comment(2)
        .iter()
        .map(|c| c.text.as_str())
        .collect();
    assert_eq!(doc, [" doc"]);

    // Several hoisted comments, one after a blank line inside the header.
    let source = "# one\n# two\nprimitive # a\n  bar(x: X) # b\n\n  # c\n  : Y = D\n";
    let file = ok(source);
    assert_eq!(
        render(&file),
        "# one\n# two\n# a\n# b\n^# c\nprimitive bar(x: X): Y = D"
    );
    let doc: Vec<_> = file
        .doc_comment(5)
        .iter()
        .map(|c| c.text.as_str())
        .collect();
    assert_eq!(doc, [" one", " two"]);

    // A blank line before the header still detaches the comment above it.
    let source = "# detached\n\nprimitive bar # inner\n  : Y\n";
    let file = ok(source);
    assert_eq!(render(&file), "# detached\n^# inner\nprimitive bar: Y");
    assert_eq!(file.doc_comment(2), Vec::<&sylex_syl::Comment>::new());

    // A blank line inside the header, before the first moved comment, does not
    // detach the doc comment.
    let source = "# doc\nprimitive bar\n\n  # inner\n  : Y\n";
    let file = ok(source);
    let doc: Vec<_> = file
        .doc_comment(2)
        .iter()
        .map(|c| c.text.as_str())
        .collect();
    assert_eq!(doc, [" doc"]);
}

#[test]
fn crlf_line_endings() {
    let source = "# doc\r\nprimitive p: X\r\n\r\n# end\r\n";
    let file = ok(source);
    assert_eq!(render(&file), "# doc\nprimitive p: X\n^# end");
    let doc = file.items[0].comment().unwrap();
    assert_eq!(doc.text, " doc");
    assert_eq!(doc.span.text(source), "# doc");
    let end = file.items[2].comment().unwrap();
    assert_eq!(end.text, " end");
    assert_eq!(end.span.text(source), "# end");
}

#[test]
fn integers_are_canonical() {
    check("card { a = [0, 10, -10] }", "card <{a = [0; 10; -10]}>");
    check_err("card { a = 01 }", &ErrorKind::NonCanonicalInteger, "01");
    check_err("card { a = -01 }", &ErrorKind::NonCanonicalInteger, "-01");
    check_err("card { a = -0 }", &ErrorKind::NonCanonicalInteger, "-0");
    check_err("card { a = 00 }", &ErrorKind::NonCanonicalInteger, "00");
}

#[test]
fn comment_between_argument_name_and_equals_moves_before_the_argument() {
    check(
        "card { a = f(x, to # note\n  = y) }",
        "card <{a = f(x; # note; to = y)}>",
    );
}
