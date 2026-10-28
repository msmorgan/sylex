use std::path::Path;

use deckmaste_semantics_v3::ast::*;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::lower::Elaborator;
use dpsi::Ident;

fn builtin() -> Bundle {
    static BUNDLE: std::sync::OnceLock<Bundle> = std::sync::OnceLock::new();
    BUNDLE
        .get_or_init(|| {
            Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/builtin"))
                .unwrap()
        })
        .clone()
}

#[test]
fn constructors_lower_to_the_expected_ast_type() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader
            .read::<Amount>("Semantics.Primitives.Amount.lit 3")
            .unwrap(),
        Amount::Lit { value: 3 }
    );
    assert_eq!(
        reader.read::<Vec<Color>>("[.white, .blue]").unwrap(),
        vec![Color::White, Color::Blue]
    );
}

#[test]
fn domain_lists_and_mana_coerce_top_down() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<Action>("[]").unwrap(),
        Action::Sequentially { steps: vec![] }
    );
    assert_eq!(
        reader.read::<Cost>("[2, .white]").unwrap(),
        Cost::Mana {
            cost: vec![
                ManaSymbol::Simple {
                    symbol: SimpleManaSymbol::Generic { amount: 2 }
                },
                ManaSymbol::Simple {
                    symbol: SimpleManaSymbol::Specific {
                        color: ColorOrColorless::Of {
                            color: Color::White
                        }
                    }
                },
            ]
        }
    );
}

#[test]
fn registry_declarations_lower_without_expanding_their_rules() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let creature = reader.read::<CardType>("creature").unwrap();
    assert_eq!(creature.name, Ident::new("Creature"));
    assert_eq!(creature.card_class, Some(CardClass::PermanentCard));
    assert!(creature.stats.contains(&Stat::Power));
}

#[test]
fn builtin_card_types_contribute_their_declared_classes() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    for (name, class) in [
        ("artifact", Some(CardClass::PermanentCard)),
        ("battle", Some(CardClass::PermanentCard)),
        ("creature", Some(CardClass::PermanentCard)),
        ("enchantment", Some(CardClass::PermanentCard)),
        ("land", Some(CardClass::PermanentCard)),
        ("planeswalker", Some(CardClass::PermanentCard)),
        ("instant", Some(CardClass::InstantOrSorceryCard)),
        ("sorcery", Some(CardClass::InstantOrSorceryCard)),
        ("kindred", None),
        ("dungeon", None),
    ] {
        assert_eq!(
            reader.read::<CardType>(name).unwrap().card_class,
            class,
            "{name}"
        );
    }
}

#[test]
fn card_type_class_defaults_match_between_registries_and_constructors() {
    let bundle = declarations(&["card_type sample := { name := \"Sample\" }"]);
    let reader = Elaborator::new(&bundle);
    let expected = CardType {
        name: Ident::new("Sample"),
        card_class: None,
        stats: vec![],
    };
    for term in [
        "sample",
        "{ name := \"Sample\" }",
        "Semantics.CardType.mk \"Sample\"",
    ] {
        assert_eq!(reader.read::<CardType>(term).unwrap(), expected, "{term}");
    }
}

#[test]
fn snapshot_samples_preserve_card_type_classes_and_stats() {
    let bundle = declarations(&["primitive example (ty : CardType) : CardType := { body := ty }"]);
    let reader = Elaborator::new(&bundle);
    let samples = reader.sample_invocations(&bundle.order[0]).unwrap();
    let values = samples
        .arguments
        .iter()
        .map(|arguments| {
            reader
                .read::<CardType>(&format!("example {arguments}"))
                .unwrap()
        })
        .collect::<Vec<_>>();
    for class in [
        None,
        Some(CardClass::PermanentCard),
        Some(CardClass::InstantOrSorceryCard),
    ] {
        assert!(
            values.iter().any(|value| value.card_class == class),
            "{class:?}"
        );
    }
    assert!(values.iter().any(|value| value.stats == [Stat::Power]));
    assert!(values.iter().any(|value| value.stats.is_empty()));
}

#[test]
fn snapshot_samples_cover_optional_booleans_and_enums() {
    let bundle = declarations(&[
        "primitive boolean (flag : Option Bool) : Option Bool := { body := flag }",
        "primitive zone (location : Option Zone) : Option Zone := { body := location }",
    ]);
    let reader = Elaborator::new(&bundle);
    for key in &bundle.order {
        let samples = reader.sample_invocations(key).unwrap();
        if key.bare_name.as_str() == "boolean" {
            let values = samples
                .arguments
                .iter()
                .map(|arguments| {
                    reader
                        .read::<Option<bool>>(&format!("boolean {arguments}"))
                        .unwrap()
                })
                .collect::<Vec<_>>();
            assert_eq!(values, [None, Some(false), Some(true)]);
        } else {
            let values = samples
                .arguments
                .iter()
                .map(|arguments| {
                    reader
                        .read::<Option<Zone>>(&format!("zone {arguments}"))
                        .unwrap()
                })
                .collect::<Vec<_>>();
            assert_eq!(
                values,
                [
                    None,
                    Some(Zone::Battlefield),
                    Some(Zone::Graveyard),
                    Some(Zone::Exile),
                    Some(Zone::Hand),
                    Some(Zone::Library),
                    Some(Zone::Stack),
                    Some(Zone::Command),
                ]
            );
        }
    }
}

#[test]
fn live_primitives_expand_through_the_constructor_basis() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<Amount>("amountLit 7").unwrap(),
        Amount::Lit { value: 7 }
    );
    assert_eq!(reader.read::<NounPhrase>("you").unwrap(), NounPhrase::You);
}

#[test]
fn keyword_declarations_build_typed_wrappers() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<Ability>("deathtouch").unwrap(),
        Ability::Keyword {
            keyword: KeywordLabel::Named {
                name: Ident::new("deathtouch")
            },
            params: vec![],
            body: vec![],
        }
    );
    assert!(
        matches!(reader.read::<Ability>("flying").unwrap(), Ability::Keyword { body, .. } if body.len() == 1)
    );
}

fn declarations(sources: &[&str]) -> Bundle {
    Bundle::from_items(
        sources
            .iter()
            .enumerate()
            .map(|(i, source)| deckmaste_semantics_v3::bundle::BundleItem {
                declaration: deckmaste_semantics_v3::reader::parse(source).unwrap(),
                file: format!("{i}.lean").into(),
            })
            .collect(),
    )
    .unwrap()
}

#[test]
fn overloads_dispatch_by_expected_result_before_reading_arguments() {
    let bundle = declarations(&[
        "primitive example : Amount := { body := .lit 5 }",
        "primitive example : NounPhrase := { body := .you }",
        "primitive use (who : NounPhrase) (n : Amount) : Action := { body := .instruct who (.draw (.exactlyOf n)) }",
    ]);
    assert_eq!(
        Elaborator::new(&bundle)
            .read::<Action>("use example example")
            .unwrap(),
        Action::Instruct {
            subj: Box::new(NounPhrase::You),
            action: Box::new(Action::Draw {
                quantity: Box::new(Quantity::ExactlyOf {
                    amount: Box::new(Amount::Lit { value: 5 })
                })
            }),
        }
    );
}

#[test]
fn defaults_and_named_arguments_use_constructor_types() {
    let bundle = declarations(&["primitive value (n : Int := 4) : Amount := { body := .lit n }"]);
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<Amount>("value").unwrap(),
        Amount::Lit { value: 4 }
    );
    assert_eq!(
        reader.read::<Amount>("value (n := 9)").unwrap(),
        Amount::Lit { value: 9 }
    );
    assert!(reader.read::<Amount>("value (wrong := 9)").is_err());
    assert!(reader.read::<Amount>("value (n := 9) (n := 8)").is_err());
}

#[test]
fn definition_parameters_get_caller_scopes_and_stable_mentions() {
    let bundle = declarations(&[
        "primitive take (who : NounPhrase) (n : Amount) : Action := { body := .instruct who (.draw (.exactlyOf n)) }",
        "action example (who : NounPhrase) (n : Amount) := { body := [take who n, take who n] }",
    ]);
    let actual = Elaborator::new(&bundle)
        .read::<Action>("example .you 2")
        .unwrap();
    let Action::OwnScope { body, .. } = actual else {
        panic!("definition must own its scope")
    };
    let Action::Sequentially { steps } = *body else { panic!("expected sequence") };
    let mentions = steps.iter().map(|step| {
        let Action::Instruct { subj: performer, action: body } = step else { panic!("expected instruct") };
        assert!(matches!(body.as_ref(), Action::Draw { quantity } if matches!(quantity.as_ref(), Quantity::ExactlyOf { amount } if matches!(amount.as_ref(), Amount::InCaller { .. }))));
        performer.as_ref()
    }).collect::<Vec<_>>();
    assert!(
        matches!(mentions[0], NounPhrase::FirstMention { id, phrase: body } if *id == Ident::new("example.who") && matches!(body.as_ref(), NounPhrase::InCaller { .. }))
    );
    assert!(
        matches!(mentions[1], NounPhrase::LaterMention { id, .. } if *id == Ident::new("example.who"))
    );
}

#[test]
fn closed_builtin_keywords_and_actions_expand() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let mut failures = Vec::new();
    for item in bundle.items.values() {
        let decl = &item.declaration;
        if !decl.parameters.is_empty() {
            continue;
        }
        let result = match decl.command.as_str() {
            "keyword" => reader.read::<Ability>(decl.name.as_str()).map(|_| ()),
            "action" => reader.read::<Action>(decl.name.as_str()).map(|_| ()),
            _ => continue,
        };
        if let Err(error) = result {
            failures.push(format!("{}: {error}", decl.name));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn card_declarations_lower_into_single_faced_cards() {
    let bundle = declarations(&[
        "card sample := { name := \"Example\", cost := [2, .blue], types := [], text := [] }",
    ]);
    let Card::SingleFaced { face } = Elaborator::new(&bundle).read::<Card>("sample").unwrap()
    else {
        panic!("expected single faced card")
    };
    assert_eq!(face.characteristics.name, Some(Ident::new("Example")));
    assert_eq!(face.characteristics.cost.as_ref().unwrap().len(), 2);
}

#[test]
fn registry_expansion_preserves_conferrals() {
    let bundle = builtin();
    let key = deckmaste_semantics_v3::bundle::BundleKey {
        bare_name: Ident::new("creature"),
        return_type: Ident::new("CardType"),
    };
    let definition = Elaborator::new(&bundle).registry_definition(&key).unwrap();
    let RegistryDefinition::CardType { confers, .. } = definition else {
        panic!("expected card type definition")
    };
    assert_ne!(confers, Vec::<Conferral>::new());
}

#[test]
fn builtin_registries_and_rules_expand() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let mut failures = Vec::new();
    for (key, item) in &bundle.items {
        let decl = &item.declaration;
        let result = match decl.command.as_str() {
            "counter" | "designation" | "subtype" | "card_type" | "relation" => {
                reader.registry_definition(key).map(|_| ())
            }
            "token" => reader
                .read::<PredefinedToken>(decl.name.as_str())
                .map(|_| ()),
            "sba" => reader.read::<SbaRule>(decl.name.as_str()).map(|_| ()),
            "conferral" => reader.read::<ConferralRule>(decl.name.as_str()).map(|_| ()),
            _ => continue,
        };
        if let Err(error) = result {
            failures.push(format!("{}: {error}", decl.name));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn computed_mode_helpers_preserve_costs_and_actions() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let simple = reader
        .read::<Action>("chooseModes (exactly 1) [draw 2, draw 3]")
        .unwrap();
    let Action::ChooseModes { modes, .. } = simple else {
        panic!("expected modal choice")
    };
    assert_eq!(modes.len(), 2);
    assert!(
        modes
            .iter()
            .all(|(cost, (limit, _))| cost.is_none() && limit.is_none())
    );
    assert_ne!(modes[0].1.1, modes[1].1.1);
    let spree = reader
        .read::<Action>("chooseSpree [(some [2, .blue], draw 2), (none, draw 3)]")
        .unwrap();
    let Action::ChooseModes { modes: paid, .. } = spree else {
        panic!("expected spree modes")
    };
    assert!(paid[0].0.is_some());
    assert!(paid[1].0.is_none());
    assert_eq!(paid[0].1.1, modes[0].1.1);
    assert_eq!(paid[1].1.1, modes[1].1.1);
}

#[test]
fn generic_spliced_mode_frames_close_over_the_caller() {
    let bundle = builtin();
    let actual = Elaborator::new(&bundle)
        .read::<Action>(
            "chooseModesOver (α := Amount) (exactly 1) [2, 3] (fun (n : Amount) => draw n)",
        )
        .unwrap();
    let Action::ChooseModes { modes, .. } = actual else {
        panic!("expected modes")
    };
    assert_eq!(modes.len(), 2);
    assert!(matches!(modes[0].1.1.as_ref(), Action::InCaller { .. }));
    assert_ne!(modes[0].1.1, modes[1].1.1);
}

#[test]
fn explicit_caller_scope_opts_out_of_parameter_identity() {
    let bundle = declarations(&[
        "primitive take (who : NounPhrase) : Action := { body := .instruct who [] }",
        "template example (who : NounPhrase) : Action := { body := [take (.inCaller who), take who] }",
    ]);
    let Action::Sequentially { steps } = Elaborator::new(&bundle)
        .read::<Action>("example .you")
        .unwrap()
    else {
        panic!("expected sequence")
    };
    assert!(
        matches!(&steps[0], Action::Instruct { subj, .. } if matches!(subj.as_ref(), NounPhrase::InCaller { body } if body.as_ref() == &NounPhrase::You))
    );
    assert!(
        matches!(&steps[1], Action::Instruct { subj, .. } if subj.as_ref() == &NounPhrase::You)
    );
}

#[test]
fn predicate_alternatives_start_parameter_mentions_independently() {
    let bundle = declarations(&[
        "primitive opponent (who : NounPhrase) : Predicate := { body := .isOpponentOf who }",
        "template example (who : NounPhrase) : Predicate := { body := or [opponent who, opponent who] }",
    ]);
    let Predicate::Coord {
        items: predicates, ..
    } = Elaborator::new(&bundle)
        .read::<Predicate>("example .you")
        .unwrap()
    else {
        panic!("expected disjunction")
    };
    for predicate in predicates {
        let Predicate::IsOpponentOf { who } = predicate else {
            panic!("expected opponent")
        };
        assert!(
            matches!(who.as_ref(), NounPhrase::FirstMention { phrase, .. } if phrase.as_ref() == &NounPhrase::You)
        );
    }
}

#[test]
fn parameterized_builtin_keywords_expand() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let mut failures = Vec::new();
    for item in bundle.items.values() {
        let decl = &item.declaration;
        if decl.command.as_str() != "keyword" || decl.parameters.is_empty() {
            continue;
        }
        let mut call = decl.name.to_string();
        for parameter in &decl.parameters {
            let ty =
                deckmaste_semantics_v3::reader::render(&parameter.ty).replace("Semantics.", "");
            let sample = match ty.as_str() {
                "Amount" => "3",
                "Cost" => "[2, .blue]",
                "Predicate" => "isCreature",
                "Ability" => "activated [2] (draw 1)",
                "DeckCondition" => ".deckSizeOverMinimum 20",
                "NounPhrase" => "you",
                "Action" => "draw 1",
                _ => panic!("add an intentional sample for {ty}"),
            };
            call.push_str(" (");
            call.push_str(sample);
            call.push(')');
        }
        if let Err(error) = reader.read::<Ability>(&call) {
            failures.push(format!("{}: {error}", decl.name));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn keyword_family_computation_joins_static_parts() {
    let bundle = builtin();
    let value = Elaborator::new(&bundle).read::<Ability>("alsoForKeywords (fun (k : KeywordTerm) => static (keywordGrant this_ k isCreature)) [flying, deathtouch]").unwrap();
    let Ability::Static { spec } = value else { panic!("expected static ability") };
    let StaticSpec::Conjunction { parts, .. } = spec.as_ref() else {
        panic!("expected conjunction")
    };
    assert_eq!(parts.len(), 2);
    assert_ne!(parts[0], parts[1]);
}

#[test]
fn parameterized_builtin_actions_expand() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let mut failures = Vec::new();
    for item in bundle.items.values() {
        let decl = &item.declaration;
        if decl.command.as_str() != "action" || decl.parameters.is_empty() {
            continue;
        }
        let mut call = decl.name.to_string();
        for parameter in &decl.parameters {
            let ty =
                deckmaste_semantics_v3::reader::render(&parameter.ty).replace("Semantics.", "");
            let sample = match ty.as_str() {
                "NounPhrase" => "this_",
                "Amount" => "3",
                "Cost" => "[2]",
                "Predicate" => "isCreature",
                "Location" => "battlefield",
                "PlayPayment" => ".itsOwnCost",
                "Disclosure" => ".openly",
                "Ballot" => ".byCandidate you",
                "Subtype" => "zombie",
                "TokenSpec" => "creatureToken 2 2 [.green] [bear]",
                "CounterKind" => "p1p1Counter",
                "Action" => "draw 1",
                "String" => "\"Sample\"",
                ty if ty.starts_with("Option ") => "none",
                ty if ty.starts_with("List ") => "[]",
                _ => panic!("add an intentional sample for {ty}"),
            };
            call.push_str(" (");
            call.push_str(sample);
            call.push(')');
        }
        if let Err(error) = reader.read::<Action>(&call) {
            failures.push(format!("{}: {error}", decl.name));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn every_builtin_declaration_has_a_typed_snapshot() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let mut failures = Vec::new();
    for key in &bundle.order {
        match reader.sample_invocations(key) {
            Ok(samples) => {
                assert_ne!(samples.arguments, Vec::<String>::new());
                for arguments in samples.arguments {
                    if let Err(error) = reader.snapshot(key, &arguments, samples.result_type) {
                        failures.push(format!(
                            "{} : {} {arguments}: {error}",
                            key.bare_name, key.return_type
                        ));
                    }
                }
            }
            Err(error) => {
                failures.push(format!("{} : {}: {error}", key.bare_name, key.return_type));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn list_literals_refuse_missing_elements() {
    let bundle = declarations(&[]);
    let reader = Elaborator::new(&bundle);
    for malformed in ["[, .red]", "[.red,, .blue]"] {
        assert!(reader.read::<Vec<Color>>(malformed).is_err(), "{malformed}");
    }
    assert_eq!(reader.read::<Option<Amount>>(".none").unwrap(), None);
    assert_eq!(
        reader.read::<Option<Amount>>(".some (.lit 2)").unwrap(),
        Some(Amount::Lit { value: 2 })
    );
}

#[test]
fn ability_word_preserves_an_existing_ability_word_heading() {
    let bundle = declarations(&[
        "ability_word heading := { label := \"landfall\", body := .italicHead (.abilityWord (.named \"domain\")) (.spell none []) }",
    ]);
    let actual = Elaborator::new(&bundle).read::<Ability>("heading").unwrap();
    assert_eq!(
        actual,
        Ability::ItalicHead {
            word: ItalicWord::AbilityWord {
                label: AbilityWordLabel::Named {
                    name: "domain".into()
                }
            },
            ability: Box::new(Ability::Spell {
                timing: None,
                action: Box::new(Action::Sequentially { steps: vec![] })
            }),
        }
    );
}

#[test]
fn mutually_exclusive_match_arms_do_not_repeat_a_template_parameter() {
    let bundle = declarations(&[
        "template select (flag : Bool) (who : NounPhrase) : NounPhrase := { body := match flag with | true => who | false => who }",
    ]);
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<NounPhrase>("select true .you").unwrap(),
        NounPhrase::You
    );
    assert_eq!(
        reader.read::<NounPhrase>("select false .you").unwrap(),
        NounPhrase::You
    );
}

#[test]
fn live_canon_cards_elaborate_with_builtin() {
    let canon = Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/canon"))
        .unwrap();
    assert_eq!(canon.items.len(), 908);
    let names: Vec<_> = canon.items.keys().map(|key| key.bare_name).collect();
    let bundle = Bundle::from_items(
        builtin()
            .items
            .into_values()
            .chain(canon.items.into_values())
            .collect(),
    )
    .unwrap();
    let reader = Elaborator::new(&bundle);
    let errors: Vec<_> = names
        .iter()
        .filter_map(|name| {
            reader
                .read::<Card>(name.as_str())
                .err()
                .map(|error| format!("{name}: {error}"))
        })
        .collect();
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn anonymous_structures_and_right_associative_application_elaborate() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<KeywordFamily>("⟨protection, none⟩").unwrap(),
        reader
            .read::<KeywordFamily>("{ word := protection, sort := none }")
            .unwrap()
    );
    assert_eq!(
        reader
            .read::<Option<Ability>>("some <| spell <| draw 1")
            .unwrap(),
        Some(reader.read::<Ability>("spell (draw 1)").unwrap())
    );
}

#[test]
fn qualified_any_constructors_are_distinct_from_the_any_value_helper() {
    let bundle = declarations(&[]);
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader
            .read::<SubtypeScope>("Semantics.SubtypeScope.any")
            .unwrap(),
        SubtypeScope::Any
    );
    assert_eq!(
        reader.read::<SubtypeScope>(".any").unwrap(),
        SubtypeScope::Any
    );
    assert_eq!(reader.read::<NounPhrase>("any").unwrap(), NounPhrase::Any);
}

#[test]
fn snapshot_samples_exercise_defaults_and_every_defined_slot() {
    let bundle = declarations(&[
        "template example (flag : Bool := false) (slots : DefinedSlots) : Nat := { body := if flag then 10 else (match slots with | .powerAlone => 3 | .toughnessAlone => 4 | .bothEach => 5) }",
        "template optional (who : Option NounPhrase := none) : Nat := { body := match who with | none => 0 | some x => 1 }",
        "template listed (items : List Nat := []) : Nat := { body := List.length items }",
        "primitive closed : Nat := { body := 9 }",
    ]);
    let reader = Elaborator::new(&bundle);
    for (name, expected) in [
        ("example", vec!["3", "4", "5", "10"]),
        ("optional", vec!["0", "1"]),
        ("listed", vec!["0", "1"]),
        ("closed", vec!["9"]),
    ] {
        let key = bundle
            .items
            .keys()
            .find(|k| k.bare_name.as_str() == name)
            .unwrap();
        let samples = reader.sample_invocations(key).unwrap();
        let mut values = samples
            .arguments
            .iter()
            .map(|args| reader.snapshot(key, args, samples.result_type).unwrap())
            .collect::<Vec<_>>();
        values.sort();
        values.dedup();
        let mut expected = expected.into_iter().map(str::to_owned).collect::<Vec<_>>();
        expected.sort();
        assert_eq!(values, expected, "{name}");
        if name != "example" {
            assert!(samples.arguments.contains(&String::new()));
        }
    }
}

#[test]
fn snapshot_samples_report_caps_and_recursive_constructors() {
    let bundle = declarations(&[
        "primitive example (a b c d : TurnPart) (body : Ability) : Nat := { body := 3 }",
    ]);
    let key = &bundle.order[0];
    let samples = Elaborator::new(&bundle).sample_invocations(key).unwrap();
    assert_eq!(samples.arguments.len(), 256);
    assert!(
        samples
            .limitations
            .iter()
            .any(|s| s.contains("sample cap 256") && s.contains("omitted"))
    );
    assert!(
        samples
            .limitations
            .iter()
            .any(|s| s.contains("Ability.inCaller omitted") && s.contains("recursive"))
    );
    for parameter in ["a", "b", "c", "d"] {
        assert!(
            samples
                .arguments
                .iter()
                .any(|s| s.contains(&format!("({parameter} := Semantics.TurnPart.cleanup)")))
        );
    }
}
