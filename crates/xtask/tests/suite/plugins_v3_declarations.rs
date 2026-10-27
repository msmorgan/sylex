//! Declaration contracts across metadata, typed expansion, and card positions.
use std::path::Path;

use deckmaste_semantics_v3::ast::Ability;
use deckmaste_semantics_v3::ast::AbilityWordLabel;
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Amount;
use deckmaste_semantics_v3::ast::Card;
use deckmaste_semantics_v3::ast::Deed;
use deckmaste_semantics_v3::ast::DeedPermanent;
use deckmaste_semantics_v3::ast::ItalicWord;
use deckmaste_semantics_v3::ast::KeywordLabel;
use deckmaste_semantics_v3::ast::KeywordParam;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::Quantity;
use deckmaste_semantics_v3::ast::Timing;
use deckmaste_semantics_v3::ast::TurnPart;
use deckmaste_semantics_v3::ast::TurnRef;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::bundle::BundleItem;
use deckmaste_semantics_v3::lower::Elaborator;
use deckmaste_semantics_v3::reader::Token;
use deckmaste_semantics_v3::reader::{self};

fn builtin() -> Bundle {
    static BUNDLE: std::sync::OnceLock<Bundle> = std::sync::OnceLock::new();
    BUNDLE
        .get_or_init(|| {
            Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/builtin"))
                .unwrap()
        })
        .clone()
}
fn with_declarations(sources: &[&str]) -> Bundle {
    let mut items: Vec<_> = builtin().items.into_values().collect();
    items.extend(sources.iter().enumerate().map(|(i, source)| BundleItem {
        declaration: reader::parse(source).unwrap(),
        file: format!("fixture/{i}.lean").into(),
    }));
    Bundle::from_items(items).unwrap()
}

#[test]
fn declaration_metadata_is_retained_alongside_expansions() {
    let bundle = builtin();
    let decl = &bundle
        .items
        .values()
        .find(|i| i.declaration.name.as_str() == "destroy")
        .unwrap()
        .declaration;
    let spelling = decl
        .fields
        .iter()
        .find(|(name, _)| name.as_str() == "spelling")
        .unwrap()
        .1;
    assert_eq!(spelling, &vec![Token::String("destroy <subject>".into())]);
    let lexical = decl
        .fields
        .iter()
        .find(|(name, _)| name.as_str() == "lexical")
        .unwrap()
        .1;
    assert!(matches!(lexical.as_slice(),[Token::Group('{',fields)] if !fields.is_empty()));
    assert!(matches!(
        Elaborator::new(&bundle)
            .read::<Action>("destroy this_")
            .unwrap(),
        Action::OwnScope { .. }
    ));
}

#[test]
fn an_ability_word_invocation_expands_at_an_ability_position() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let actual = reader.read::<Ability>("threshold deathtouch").unwrap();
    let expected = Ability::ItalicHead {
        word: ItalicWord::AbilityWord {
            label: AbilityWordLabel::Named {
                name: "threshold".into(),
            },
        },
        ability: Box::new(reader.read::<Ability>("deathtouch").unwrap()),
    };
    assert_eq!(actual, expected);
}

#[test]
fn a_keyword_action_invocation_expands_at_an_action_position() {
    let bundle = with_declarations(&[
        "action drawFor (amount : Amount) := { deed := none, body := draw amount }",
        "card drawingSpell := { name := \"Drawing Spell\", cost := [.blue], types := [instant], text := [spell (drawFor 2)] }",
    ]);
    let card = Elaborator::new(&bundle)
        .read::<Card>("drawingSpell")
        .unwrap();
    let Card::SingleFaced { face } = card else {
        panic!("expected single-faced card")
    };
    assert_eq!(
        face.characteristics.text,
        vec![Ability::Spell {
            timing: None,
            action: Box::new(Action::OwnScope {
                deed: None,
                body: Box::new(Action::Draw {
                    quantity: Box::new(Quantity::ExactlyOf {
                        amount: Box::new(Amount::InCaller {
                            body: Box::new(Amount::Lit { value: 2 })
                        })
                    })
                }),
            })
        }]
    );
}

#[test]
fn a_keyword_declaration_builds_its_wrapper() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    assert_eq!(
        reader.read::<Ability>("toxic 3").unwrap(),
        Ability::Keyword {
            keyword: KeywordLabel::Named {
                name: "toxic".into()
            },
            params: vec![KeywordParam::Number {
                amount: Box::new(Amount::InCaller {
                    body: Box::new(Amount::Lit { value: 3 })
                })
            }],
            body: vec![],
        }
    );
    let Action::OwnScope { deed, body } = reader.read::<Action>("destroy this_").unwrap() else {
        panic!("action must own its scope")
    };
    assert_eq!(
        deed,
        Some(Deed::Action {
            label: "destroy".into(),
            permanent: DeedPermanent::None
        })
    );
    let Action::Move { subject, .. } = body.as_ref() else {
        panic!("destroy moves its subject")
    };
    assert!(
        matches!(subject.as_ref(),NounPhrase::FirstMention { id, phrase } if id.as_str()=="destroy.subject" && matches!(phrase.as_ref(),NounPhrase::InCaller { body } if body.as_ref()==&NounPhrase::This))
    );
    assert!(
        matches!(reader.read::<Action>("shuffle").unwrap(),Action::OwnScope { deed:Some(Deed::Action { label, .. }), .. } if label.as_str()=="shuffle")
    );
}

#[test]
fn a_turn_part_declaration_may_name_its_own_constructor() {
    let bundle = with_declarations(&[
        "primitive fixtureUpkeep : TurnPart := { body := Semantics.TurnPart.upkeep }",
        "template fixtureUpkeepStep : TurnPart := { body := fixtureUpkeep }",
        "card upkeepSpell := { name := \"Upkeep Spell\", cost := [.blue], types := [instant], text := [spell (draw 1) (timing := duringPart fixtureUpkeepStep)] }",
    ]);
    let Card::SingleFaced { face } = Elaborator::new(&bundle)
        .read::<Card>("upkeepSpell")
        .unwrap()
    else {
        panic!("expected single-faced card")
    };
    let [Ability::Spell { timing, .. }] = face.characteristics.text.as_slice() else {
        panic!("expected spell ability")
    };
    assert_eq!(
        timing.as_deref(),
        Some(&Timing::InPart {
            part: TurnPart::Upkeep,
            whose: None,
            which: Box::new(TurnRef::Current)
        })
    );
    let standalone = Bundle::from_items(vec![BundleItem {
        declaration: reader::parse(
            "primitive upkeep : TurnPart := { body := Semantics.TurnPart.upkeep }",
        )
        .unwrap(),
        file: "upkeep.lean".into(),
    }])
    .unwrap();
    assert_eq!(
        Elaborator::new(&standalone)
            .read::<TurnPart>("upkeep")
            .unwrap(),
        TurnPart::Upkeep
    );
}
