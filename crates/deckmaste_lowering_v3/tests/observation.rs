//! A move from the battlefield observed on its arrival is refused, as the Lean checker refuses it
//! (`Refusal.arrivalFromBattlefield`, [CR#603.6c,603.10a]); no builtin or canon text writes one.
use std::path::Path;

use deckmaste_lowering_v3::reads::ActionLocalShapeExt;
use deckmaste_lowering_v3::refusal::Refusal;
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Card;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::lower::Elaborator;
use serde_json::Value;

fn plugin(name: &str) -> Bundle {
    Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3").join(name))
        .unwrap()
}

/// Every move in a serialized term that the local shape refuses.
fn refused_moves(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(fields) = map.get("Move")
                && let Ok(action) =
                    serde_json::from_value::<Action>(Value::Object(serde_json::Map::from_iter([(
                        "Move".to_owned(),
                        fields.clone(),
                    )])))
                && !action.local_checks().is_empty()
            {
                found.push(format!("{action:?}"));
            }
            map.values().for_each(|v| refused_moves(v, found));
        }
        Value::Array(items) => items.iter().for_each(|v| refused_moves(v, found)),
        _ => {}
    }
}

#[test]
fn a_move_from_the_battlefield_observed_on_arrival_is_refused() {
    let bundle = plugin("builtin");
    let reader = Elaborator::new(&bundle);
    for (origin, refused) in [
        ("battlefield", true),
        ("(or [battlefield, yourHand])", true),
        ("(inCaller battlefield)", true),
        ("yourHand", false),
        ("(not battlefield)", false),
        ("wherever", false),
    ] {
        for (at, observation_refused) in [(".arrival", refused), (".departure", false)] {
            let action = reader
                .read::<Action>(&format!(
                    "move (target isCreature) {origin} (some graveyard) {at} []"
                ))
                .unwrap();
            assert_eq!(
                action.local_checks(),
                if observation_refused { vec![Refusal::ArrivalFromBattlefield] } else { vec![] },
                "{origin} {at}"
            );
        }
    }
}

#[test]
fn no_builtin_or_canon_text_moves_from_the_battlefield_observed_on_arrival() {
    let builtin = plugin("builtin");
    let canon = plugin("canon");
    let names: Vec<_> = canon.items.keys().map(|key| key.bare_name).collect();
    let registries: Vec<_> = builtin
        .items
        .iter()
        .filter(|(_, item)| {
            ["counter", "designation", "subtype", "card_type", "relation"]
                .contains(&item.declaration.command.as_str())
        })
        .map(|(key, _)| key.clone())
        .collect();
    let bundle = Bundle::from_items(
        builtin
            .items
            .into_values()
            .chain(canon.items.into_values())
            .collect(),
    )
    .unwrap();
    let reader = Elaborator::new(&bundle);
    let mut found = Vec::new();
    for name in &names {
        let card = reader.read::<Card>(name.as_str()).unwrap();
        refused_moves(&serde_json::to_value(card).unwrap(), &mut found);
    }
    for key in &registries {
        let definition = reader.registry_definition(key).unwrap();
        refused_moves(&serde_json::to_value(definition).unwrap(), &mut found);
    }
    assert!(found.is_empty(), "{found:#?}");
}
