use deckmaste_semantics_v3::{
    bundle::{Bundle, BundleItem},
    reader::{Token, parse, tokenize},
};
use dpsi::Ident;
use std::path::Path;

fn item(source: &str) -> BundleItem {
    BundleItem {
        declaration: parse(source).unwrap(),
        file: "arbitrary.lean".into(),
    }
}
#[test]
fn live_builtin_bundle_loads() {
    let bundle =
        Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/builtin"))
            .unwrap();
    assert_eq!(bundle.order.len(), bundle.items.len());
    assert!(bundle.items.keys().any(|k| k.bare_name.as_str() == "amass"));
}
#[test]
fn nested_metadata_and_spelling_are_preserved() {
    let decl = parse(
        r#"template sample (p : NounPhrase) : Action := {
      lexical := { verbs := [{ lemma := "{not a delimiter}", frames := [Transitive] }] },
      spelling := "do <p>", body := act p -- } ignored
    }"#,
    )
    .unwrap();
    assert_eq!(decl.parameters[0].name, Ident::new("p"));
    assert_eq!(
        decl.fields[&Ident::new("spelling")],
        vec![Token::String("do <p>".into())]
    );
}
#[test]
fn requires_exactly_one_declaration() {
    for source in [
        "",
        "-- only a comment",
        "template a : Action := { body := a } template b : Action := { body := b }",
    ] {
        assert!(parse(source).is_err(), "{source}");
    }
}
#[test]
fn rejects_malformed_delimiters_and_duplicate_fields() {
    for source in ["([)]", "{", "\"open", "/- open"] {
        assert!(tokenize(source).is_err());
    }
    assert!(parse("template a : Action := { body := x, body := y }").is_err());
}
#[test]
fn comments_nest_and_strings_hide_delimiters() {
    assert_eq!(
        tokenize("/- outer /- inner -/ -/ \"[\\\"}\"").unwrap(),
        vec![Token::String("[\"}".into())]
    );
}
#[test]
fn overloaded_names_are_distinct_and_ready_queue_is_lexical() {
    let bundle = Bundle::from_items(vec![
        item("template z : Action := {body := a}"),
        item("template a : Action := {body := raw}"),
        item("template a : Predicate := {body := raw}"),
    ])
    .unwrap();
    let order: Vec<_> = bundle
        .order
        .iter()
        .map(|k| (k.bare_name.as_str(), k.return_type.as_str()))
        .collect();
    assert_eq!(
        order,
        [("a", "Action"), ("a", "Predicate"), ("z", "Action")]
    );
}
#[test]
fn exact_cycles_exclude_downstream_nodes() {
    let error = Bundle::from_items(vec![
        item("template a : Action := {body := b}"),
        item("template b : Action := {body := a}"),
        item("template downstream : Action := {body := a}"),
    ])
    .unwrap_err()
    .to_string();
    assert!(error.contains("a : Action") && error.contains("b : Action"));
    assert!(!error.contains("downstream"));
}
#[test]
fn catalog_metadata_and_parameter_names_create_no_dependencies() {
    let bundle = Bundle::from_items(vec![item("template a (b : Action) : Action := {lexical := {lemma := b}, spelling := \"<b>\", body := b}"), item("template b : Action := {body := a}")]).unwrap();
    assert_eq!(bundle.order[0].bare_name, Ident::new("a"));
}
#[test]
fn duplicate_keys_are_refused() {
    assert!(
        Bundle::from_items(vec![
            item("template a : Action := {body := raw}"),
            item("template a : Action := {body := raw}")
        ])
        .is_err()
    );
}
#[test]
fn card_declarations_are_supported_without_a_canon_directory() {
    let decl =
        parse("card example := { name := \"Example\", types := [creature], text := [] }").unwrap();
    assert_eq!(decl.return_type, Ident::new("Spelled"));
}

#[test]
fn all_command_kinds_read_independently_of_paths() {
    for command in [
        "primitive",
        "action",
        "keyword",
        "ability_word",
        "counter",
        "designation",
        "subtype",
        "card_type",
        "relation",
        "token",
        "sba",
        "conferral",
        "template",
        "card",
    ] {
        let (signature, body) = match command {
            "primitive" | "template" => (": Action", "body := raw"),
            "action" => ("", "body := draw 1"),
            "keyword" => ("", "body := []"),
            "ability_word" => ("(ability : Ability)", "body := ability"),
            "counter" | "relation" => ("", ""),
            "designation" => ("", "scope := player"),
            "subtype" => ("", "kind := creature, name := \"Sample\""),
            "card_type" => ("", "cardClass := some .permanentCard"),
            "token" => ("", "name := \"Sample\", token := {}"),
            "sba" => ("", "scope := creature, when := condition, then_ := action"),
            "conferral" => ("", "scope := creature, confer := property"),
            "card" => ("", "name := \"Sample\", types := [creature]"),
            _ => unreachable!(),
        };
        let source = format!("{command} sample {signature} := {{{body}}}");
        assert_eq!(parse(&source).unwrap().command, Ident::new(command));
    }
}
#[test]
fn generic_binders_defaults_and_composite_result_types_read() {
    let decl =
        parse("template sample {α : Type} (xs : List α) (n : Nat := 1) : List α := { body := xs }")
            .unwrap();
    assert!(decl.parameters[0].implicit);
    assert!(decl.parameters[2].default.is_some());
    assert_eq!(decl.return_type, Ident::new("List α"));
}
#[test]
fn unknown_fields_and_empty_middle_fields_are_refused() {
    for source in [
        "template a : Action := {body := raw, unknown := value}",
        "template a : Action := {body := raw,,}",
        "template a : Action := {spelling := 1,body := raw}",
    ] {
        assert!(parse(source).is_err(), "{source}");
    }
}
#[test]
fn moving_files_between_folders_preserves_bundle_order() {
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();
    std::fs::create_dir(second.path().join("unrelated")).unwrap();
    let sources = [
        "template z : Action := {body := a}",
        "template a : Action := {body := raw}",
    ];
    for (index, source) in sources.iter().enumerate() {
        std::fs::write(first.path().join(format!("{index}.lean")), source).unwrap();
        std::fs::write(
            second
                .path()
                .join(format!("unrelated/reversed-{}.lean", 1 - index)),
            source,
        )
        .unwrap();
    }
    assert_eq!(
        Bundle::load(first.path()).unwrap().order,
        Bundle::load(second.path()).unwrap().order
    );
}

#[test]
fn token_rendering_preserves_numerals_and_compound_operators() {
    let source = "fun n => n + 123 == 456 && n <= 1000";
    let tokens = tokenize(source).unwrap();
    assert_eq!(deckmaste_semantics_v3::reader::render(&tokens), source);
    assert_eq!(tokenize("0xFF").unwrap(), vec![Token::Atom("0xFF".into())]);
}

#[test]
fn same_name_primitive_precedes_its_template() {
    let bundle = Bundle::from_items(vec![
        item("template timesPaid : Amount := { body := timesPaid }"),
        item("primitive timesPaid : LinkedFacet := { body := Semantics.Primitives.LinkedFacet.timesPaid }"),
        item("template phyrexianPaidWithLife : Amount := { body := phyrexianPaidWithLife }"),
        item("primitive phyrexianPaidWithLife : LinkedFacet := { body := Semantics.Primitives.LinkedFacet.phyrexianPaidWithLife }"),
    ])
    .unwrap();
    let order: Vec<_> = bundle
        .order
        .iter()
        .map(|key| (key.bare_name.as_str(), key.return_type.as_str()))
        .collect();
    assert_eq!(
        order,
        [
            ("phyrexianPaidWithLife", "LinkedFacet"),
            ("phyrexianPaidWithLife", "Amount"),
            ("timesPaid", "LinkedFacet"),
            ("timesPaid", "Amount"),
        ]
    );
}

#[test]
fn enum_constructor_names_do_not_create_bundle_dependencies() {
    let bundle = Bundle::from_items(vec![
        item("template exileZone : Location := { body := zone .exile none }"),
        item("action exile := { body := exileZone }"),
        item("primitive zone : Location := { body := Semantics.Primitives.Location.zone }"),
    ])
    .unwrap();
    let names: Vec<_> = bundle
        .order
        .iter()
        .map(|key| key.bare_name.as_str())
        .collect();
    assert_eq!(names, ["zone", "exileZone", "exile"]);
}

#[test]
fn primitive_shadowed_by_a_parameter_is_loaded_before_its_caller() {
    let bundle = Bundle::from_items(vec![
        item("template isNamed (name : String) : CopyExcept := { body := edits [name .sets name] }"),
        item("primitive name (op : QualityOp) (name : String) : CharacteristicEdit := { body := Semantics.Primitives.CharacteristicEdit.name op name }"),
    ])
    .unwrap();
    let names: Vec<_> = bundle
        .order
        .iter()
        .map(|key| key.bare_name.as_str())
        .collect();
    assert_eq!(names, ["name", "isNamed"]);
}
