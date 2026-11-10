//! The declaration rules [`check`] enforces across bundles.

use std::path::PathBuf;
use std::sync::Arc;

use sylex_syl::Bundle;
use sylex_syl::FileId;
use sylex_syl::check;
use sylex_syl::check::RefusalKind;

fn bundle(root: &str, first: FileId, sources: &[(&str, &str)]) -> Arc<Bundle> {
    Bundle::from_sources(
        root,
        sources
            .iter()
            .map(|(path, source)| (PathBuf::from(path), (*source).to_owned())),
        first,
    )
    .expect("test sources parse")
}

/// The refusals for one bundle of `sources`, with the text each one spans.
fn refusals(sources: &[(&str, &str)]) -> Vec<(RefusalKind, String)> {
    let bundle = bundle("root", FileId(0), sources);
    let Err(error) = check(&[&bundle]) else {
        return Vec::new();
    };
    error
        .refusals
        .into_iter()
        .map(|refusal| {
            let file = bundle.file(refusal.span.file).expect("refusal in bundle");
            (refusal.kind, refusal.span.text(&file.source).to_owned())
        })
        .collect()
}

#[test]
fn a_primitive_with_a_constructor_clause_and_known_words_passes() {
    let sources = [
        ("power.syl", "primitive power: Stat = Power"),
        ("stat.syl", "primitive stat(stat: Stat): Amount = StatOf"),
        (
            "t.syl",
            "template itsPower: Amount { body = stat(power), lexical = Frame(unknownWord) }",
        ),
        ("o.syl", "template maybe(x: Option(Stat) = none): Amount { body = stat(power) }"),
    ];
    assert_eq!(refusals(&sources), []);
}

#[test]
fn a_primitive_without_a_constructor_clause_is_refused() {
    assert_eq!(
        refusals(&[("power.syl", "primitive power: Stat")]),
        [(
            RefusalKind::MissingConstructor("power".to_owned()),
            "power: Stat".to_owned()
        )]
    );
}

#[test]
fn a_primitive_without_a_sort_is_refused() {
    assert_eq!(
        refusals(&[("power.syl", "primitive power = Power")]),
        [(
            RefusalKind::MissingSort("power".to_owned()),
            "power = Power".to_owned()
        )]
    );
}

#[test]
fn a_nameless_primitive_is_refused() {
    assert_eq!(
        refusals(&[("p.syl", "primitive { body = none }")]),
        [(
            RefusalKind::NamelessPrimitive,
            "primitive { body = none }".to_owned()
        )]
    );
}

#[test]
fn a_constructor_clause_outside_a_primitive_is_refused() {
    assert_eq!(
        refusals(&[("t.syl", "template power: Stat = Power")]),
        [(
            RefusalKind::ConstructorOutsidePrimitive {
                kind: "template".to_owned(),
                name: "power".to_owned(),
                constructor: "Power".to_owned(),
            },
            "Power".to_owned()
        )]
    );
}

#[test]
fn two_primitives_for_one_constructor_are_refused() {
    assert_eq!(
        refusals(&[
            ("a.syl", "primitive power: Stat = Power"),
            ("b.syl", "primitive strength: Stat = Power"),
        ]),
        [(
            RefusalKind::DuplicateConstructor {
                first: "power".to_owned(),
                second: "strength".to_owned(),
                constructor: "Power".to_owned(),
                sort: "Stat".to_owned(),
            },
            "Power".to_owned()
        )]
    );
}

#[test]
fn one_constructor_name_at_two_sorts_is_two_constructors() {
    assert_eq!(
        refusals(&[
            ("a.syl", "primitive up(by: Amount): Delta(Amount) = Up"),
            ("b.syl", "primitive upCost(by: ManaCostTerm): Delta(ManaCostTerm) = Up"),
        ]),
        []
    );
}

#[test]
fn a_word_that_names_no_declaration_is_refused() {
    assert_eq!(
        refusals(&[
            ("stat.syl", "primitive stat(stat: Stat): Amount = StatOf"),
            ("t.syl", "template itsPower: Amount { body = stat(power) }"),
            ("u.syl", "template withNone: Amount { body = none(power) }"),
        ]),
        [
            (RefusalKind::UnknownWord("power".to_owned()), "power".to_owned()),
            (RefusalKind::UnknownWord("none".to_owned()), "none".to_owned()),
            (RefusalKind::UnknownWord("power".to_owned()), "power".to_owned()),
        ]
    );
}

#[test]
fn words_resolve_across_bundles_and_refusals_render_their_location() {
    let builtin = bundle(
        "builtin",
        FileId(0),
        &[
            ("power.syl", "primitive power: Stat = Power"),
            ("singleFaced.syl", "primitive singleFaced(face: Characteristics): Card = SingleFaced"),
        ],
    );
    let canon = bundle(
        "canon",
        builtin.next_file_id(),
        &[("c.syl", "card singleFaced({\n  name = \"C\",\n  power = power,\n  toughness = toughness,\n})")],
    );
    let error = check(&[&builtin, &canon]).expect_err("toughness is not declared");
    assert_eq!(
        error.rendered,
        ["canon/c.syl:4:15: word `toughness` names no declaration"]
    );
}
