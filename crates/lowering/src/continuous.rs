//! `continuous` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for sylex_semantics_v1::Duration {
    type Target = sylex_core::Duration;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FixedUntil(f0) => sylex_core::Duration::FixedUntil(f0.lower()),
            Self::UntilEvent(f0) => sylex_core::Duration::UntilEvent(f0.lower()),
            Self::ForAsLongAs(f0) => sylex_core::Duration::ForAsLongAs(f0.lower()),
            Self::ForThisEvent => sylex_core::Duration::ForThisEvent,
            Self::EndOfGame => sylex_core::Duration::EndOfGame,
        }
    }
}
#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_duration_fixed_until() {
        assert_matches!(
            sylex_semantics_v1::Duration::FixedUntil(minimal_turn_marker()).lower(),
            sylex_core::Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn)
        );
    }

    #[test]
    fn lowers_duration_until_event() {
        assert_matches!(
            sylex_semantics_v1::Duration::UntilEvent(minimal_event_filter()).lower(),
            sylex_core::Duration::UntilEvent(sylex_core::EventFilter::ZoneChange {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            })
        );
    }

    #[test]
    fn lowers_duration_for_as_long_as() {
        assert_matches!(
            sylex_semantics_v1::Duration::ForAsLongAs(minimal_condition()).lower(),
            sylex_core::Duration::ForAsLongAs(sylex_core::Condition::Compare(
                sylex_core::Count::Literal(0),
                sylex_core::Cmp::Eq,
                sylex_core::Count::Literal(0)
            ))
        );
    }

    #[test]
    fn lowers_duration_for_this_event() {
        assert_matches!(
            sylex_semantics_v1::Duration::ForThisEvent.lower(),
            sylex_core::Duration::ForThisEvent
        );
    }

    #[test]
    fn lowers_duration_end_of_game() {
        assert_matches!(
            sylex_semantics_v1::Duration::EndOfGame.lower(),
            sylex_core::Duration::EndOfGame
        );
    }

    #[test]
    fn lowers_numeric_op_set() {
        assert_matches!(
            sylex_semantics_v1::NumericOp::Set(minimal_stat_value()).lower(),
            sylex_core::NumericOp::Set(sylex_core::StatValue::DefinedByAbility)
        );
    }

    #[test]
    fn lowers_numeric_op_up() {
        assert_matches!(
            sylex_semantics_v1::NumericOp::Up(minimal_count()).lower(),
            sylex_core::NumericOp::Up(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_numeric_op_down() {
        assert_matches!(
            sylex_semantics_v1::NumericOp::Down(minimal_count()).lower(),
            sylex_core::NumericOp::Down(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_collection_op_set() {
        assert_matches!(
            sylex_semantics_v1::CollectionOp::<sylex_semantics_v1::Color>::Set([].into()).lower(),
            sylex_core::CollectionOp::Set(_)
        );
    }

    #[test]
    fn lowers_collection_op_add() {
        assert_matches!(
            sylex_semantics_v1::CollectionOp::<sylex_semantics_v1::Color>::Add(minimal_color())
                .lower(),
            sylex_core::CollectionOp::Add(_)
        );
    }

    #[test]
    fn lowers_collection_op_remove() {
        assert_matches!(sylex_semantics_v1::CollectionOp::<sylex_semantics_v1::Color>::Remove(minimal_color()).lower(), sylex_core::CollectionOp::Remove(_));
    }

    #[test]
    fn lowers_modification_power() {
        assert_matches!(
            sylex_semantics_v1::Modification::Power(minimal_numeric_op()).lower(),
            sylex_core::Modification::Power(sylex_core::NumericOp::Set(
                sylex_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_toughness() {
        assert_matches!(
            sylex_semantics_v1::Modification::Toughness(minimal_numeric_op()).lower(),
            sylex_core::Modification::Toughness(sylex_core::NumericOp::Set(
                sylex_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_switch_power_toughness() {
        assert_matches!(
            sylex_semantics_v1::Modification::SwitchPowerToughness.lower(),
            sylex_core::Modification::SwitchPowerToughness
        );
    }

    #[test]
    fn lowers_modification_colors() {
        assert_matches!(
            sylex_semantics_v1::Modification::Colors(minimal_collection_op::<
                sylex_semantics_v1::Color,
            >())
            .lower(),
            sylex_core::Modification::Colors(sylex_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_card_types() {
        assert_matches!(
            sylex_semantics_v1::Modification::CardTypes(minimal_collection_op::<
                sylex_semantics_v1::Ident,
            >())
            .lower(),
            sylex_core::Modification::CardTypes(sylex_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_subtypes() {
        assert_matches!(
            sylex_semantics_v1::Modification::Subtypes(minimal_collection_op::<
                sylex_semantics_v1::SubtypeRef,
            >())
            .lower(),
            sylex_core::Modification::Subtypes(sylex_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_supertypes() {
        assert_matches!(
            sylex_semantics_v1::Modification::Supertypes(minimal_collection_op::<
                sylex_semantics_v1::Supertype,
            >())
            .lower(),
            sylex_core::Modification::Supertypes(sylex_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_gain_ability() {
        assert_matches!(
            sylex_semantics_v1::Modification::GainAbility(std::sync::Arc::new(minimal_ability()))
                .lower(),
            sylex_core::Modification::GainAbility(_)
        );
    }

    #[test]
    fn lowers_modification_lose_ability() {
        assert_matches!(
            sylex_semantics_v1::Modification::LoseAbility("X".into()).lower(),
            sylex_core::Modification::LoseAbility(_)
        );
    }

    #[test]
    fn lowers_modification_lose_all_abilities() {
        assert_matches!(
            sylex_semantics_v1::Modification::LoseAllAbilities.lower(),
            sylex_core::Modification::LoseAllAbilities
        );
    }

    #[test]
    fn lowers_modification_cant_have_ability() {
        assert_matches!(
            sylex_semantics_v1::Modification::CantHaveAbility("X".into()).lower(),
            sylex_core::Modification::CantHaveAbility(_)
        );
    }

    #[test]
    fn lowers_modification_set_controller() {
        assert_matches!(
            sylex_semantics_v1::Modification::SetController(minimal_reference()).lower(),
            sylex_core::Modification::SetController(sylex_core::Reference::Reg(
                sylex_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_modification_set_text() {
        assert_matches!(
            sylex_semantics_v1::Modification::SetText(String::new()).lower(),
            sylex_core::Modification::SetText(_)
        );
    }

    #[test]
    fn lowers_modification_all_creature_types() {
        assert_matches!(
            sylex_semantics_v1::Modification::AllCreatureTypes.lower(),
            sylex_core::Modification::AllCreatureTypes
        );
    }

    #[test]
    fn lowers_modification_base_loyalty() {
        assert_matches!(
            sylex_semantics_v1::Modification::BaseLoyalty(minimal_numeric_op()).lower(),
            sylex_core::Modification::BaseLoyalty(sylex_core::NumericOp::Set(
                sylex_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_base_defense() {
        assert_matches!(
            sylex_semantics_v1::Modification::BaseDefense(minimal_numeric_op()).lower(),
            sylex_core::Modification::BaseDefense(sylex_core::NumericOp::Set(
                sylex_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_become_basic_land_type() {
        assert_matches!(
            sylex_semantics_v1::Modification::BecomeBasicLandType([].into()).lower(),
            sylex_core::Modification::BecomeBasicLandType(_)
        );
    }

    #[test]
    fn lowers_modification_several() {
        assert_matches!(
            sylex_semantics_v1::Modification::Several([].into()).lower(),
            sylex_core::Modification::Several(_)
        );
    }

    #[test]
    fn lowers_modification_expanded() {
        assert_matches!(
            sylex_semantics_v1::Modification::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_modification())
            })
            .lower(),
            sylex_core::Modification::Power(sylex_core::NumericOp::Set(
                sylex_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_cost_change_increase() {
        assert_matches!(
            sylex_semantics_v1::CostChange::Increase([].into()).lower(),
            sylex_core::CostChange::Increase(_)
        );
    }

    #[test]
    fn lowers_cost_change_reduce() {
        assert_matches!(
            sylex_semantics_v1::CostChange::Reduce([].into()).lower(),
            sylex_core::CostChange::Reduce(_)
        );
    }

    #[test]
    fn lowers_cost_change_additional() {
        assert_matches!(
            sylex_semantics_v1::CostChange::Additional {
                components: [].into()
            }
            .lower(),
            sylex_core::CostChange::Additional { components: _ }
        );
    }

    #[test]
    fn lowers_cost_change_scaled() {
        assert_matches!(
            sylex_semantics_v1::CostChange::Scaled {
                change: std::sync::Arc::new(minimal_cost_change()),
                times: minimal_count()
            }
            .lower(),
            sylex_core::CostChange::Scaled {
                change: _,
                times: sylex_core::Count::Literal(0)
            }
        );
    }

    #[test]
    fn lowers_static_effect_modify() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::Modify(minimal_reference(), minimal_modification())
                .lower(),
            sylex_core::StaticSpec::Modify(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Modification::Power(sylex_core::NumericOp::Set(
                    sylex_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_static_effect_becomes_copy() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::BecomesCopy(
                minimal_reference(),
                minimal_copy_spec()
            )
            .lower(),
            sylex_core::StaticSpec::BecomesCopy(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::CopySpec {
                    source: sylex_core::CopySource::Object(sylex_core::Reference::Reg(
                        sylex_core::RefId(0)
                    )),
                    exceptions: _
                }
            )
        );
    }

    #[test]
    fn lowers_static_effect_each() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::Each(
                minimal_selection(),
                std::sync::Arc::new(minimal_static_effect())
            )
            .lower(),
            sylex_core::StaticSpec::Each(sylex_core::Selection::SelectAll(_), _)
        );
    }

    #[test]
    fn lowers_static_effect_conditionally() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::Conditionally(
                minimal_condition(),
                std::sync::Arc::new(minimal_static_effect())
            )
            .lower(),
            sylex_core::StaticSpec::Conditionally(
                sylex_core::Condition::Compare(
                    sylex_core::Count::Literal(0),
                    sylex_core::Cmp::Eq,
                    sylex_core::Count::Literal(0)
                ),
                _
            )
        );
    }

    #[test]
    fn lowers_static_effect_deontic() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::Deontic(minimal_deontic()).lower(),
            sylex_core::StaticSpec::Deontic(sylex_core::Deontic::May(
                sylex_core::DeonticAction::Attack {
                    by: sylex_core::Predicate::Class(
                        sylex_core::ObjectClass::AbilityOnStack
                    ),
                    on: sylex_core::Predicate::Class(
                        sylex_core::ObjectClass::AbilityOnStack
                    )
                }
            ))
        );
    }

    #[test]
    fn lowers_static_effect_cost_modifier() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::CostModifier {
                of: minimal_predicate(),
                change: minimal_cost_change()
            }
            .lower(),
            sylex_core::StaticSpec::CostModifier {
                of: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                change: sylex_core::CostChange::Increase(_)
            }
        );
    }

    #[test]
    fn lowers_static_effect_cost_option() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::CostOption(minimal_optional_cost()).lower(),
            sylex_core::StaticSpec::CostOption(sylex_core::OptionalCost {
                components: _,
                tag: sylex_core::CostTag(_),
                repeatable: false
            })
        );
    }

    #[test]
    fn lowers_static_effect_trigger_multiplier() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::TriggerMultiplier {
                cause: minimal_event_filter(),
                extra: minimal_count(),
                affected: minimal_predicate()
            }
            .lower(),
            sylex_core::StaticSpec::TriggerMultiplier {
                cause: sylex_core::EventFilter::ZoneChange {
                    what: sylex_core::Predicate::Class(
                        sylex_core::ObjectClass::AbilityOnStack
                    ),
                    from: None,
                    to: None,
                    cause: None
                },
                extra: sylex_core::Count::Literal(0),
                affected: sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                )
            }
        );
    }

    #[test]
    fn lowers_static_effect_modify_player() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::ModifyPlayer(
                minimal_reference(),
                minimal_player_mod()
            )
            .lower(),
            sylex_core::StaticSpec::ModifyPlayer(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::PlayerMod::Set(
                    sylex_core::PlayerAttr::Life,
                    sylex_core::Count::Literal(0)
                )
            )
        );
    }

    #[test]
    fn lowers_static_effect_prevention() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::Prevention(
                std::sync::Arc::new(minimal_prevention())
            )
            .lower(),
            sylex_core::StaticSpec::Prevention(_)
        );
    }

    #[test]
    fn lowers_static_effect_cant_prevent() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::CantPrevent {
                from: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            sylex_core::StaticSpec::CantPrevent {
                from: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                to: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_static_effect_spend_as_though() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::SpendAsThough {
                mana_from: minimal_predicate(),
                as_: minimal_symbol_pred()
            }
            .lower(),
            sylex_core::StaticSpec::SpendAsThough {
                mana_from: sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                ),
                as_: sylex_core::SymbolPred::AnyColor
            }
        );
    }

    #[test]
    fn lowers_static_effect_as_though() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::AsThough(minimal_as_though()).lower(),
            sylex_core::StaticSpec::AsThough(sylex_core::AsThough::Counterfactual {
                premise: sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                ),
                then: _
            })
        );
    }

    #[test]
    fn lowers_static_effect_outcome_gate() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::OutcomeGate {
                who: minimal_predicate(),
                gate: minimal_outcome_gate_kind()
            }
            .lower(),
            sylex_core::StaticSpec::OutcomeGate {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                gate: sylex_core::OutcomeGateKind::CantLose
            }
        );
    }

    #[test]
    fn lowers_static_effect_cant_happen() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::CantHappen(minimal_event_filter()).lower(),
            sylex_core::StaticSpec::CantHappen(sylex_core::EventFilter::ZoneChange {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            })
        );
    }

    #[test]
    fn lowers_static_effect_replace_roll() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::ReplaceRoll {
                query: minimal_event_filter(),
                extra: minimal_count(),
                ignore: minimal_ignore_rule()
            }
            .lower(),
            sylex_core::StaticSpec::ReplaceRoll {
                query: sylex_core::EventFilter::ZoneChange {
                    what: sylex_core::Predicate::Class(
                        sylex_core::ObjectClass::AbilityOnStack
                    ),
                    from: None,
                    to: None,
                    cause: None
                },
                extra: sylex_core::Count::Literal(0),
                ignore: sylex_core::IgnoreRule::IgnoreLowest
            }
        );
    }

    #[test]
    fn lowers_static_effect_pay_pips() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::PayPips(minimal_pip_class(), minimal_pay_act())
                .lower(),
            sylex_core::StaticSpec::PayPips(
                sylex_core::PipClass::Generic,
                sylex_core::PayAct::TapToPay(sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                ))
            )
        );
    }

    #[test]
    fn lowers_static_effect_expanded() {
        assert_matches!(
            sylex_semantics_v1::StaticEffect::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_static_effect())
            })
            .lower(),
            sylex_core::StaticSpec::Modify(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Modification::Power(sylex_core::NumericOp::Set(
                    sylex_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_outcome_gate_kind_cant_lose() {
        assert_matches!(
            sylex_semantics_v1::OutcomeGateKind::CantLose.lower(),
            sylex_core::OutcomeGateKind::CantLose
        );
    }

    #[test]
    fn lowers_outcome_gate_kind_cant_win() {
        assert_matches!(
            sylex_semantics_v1::OutcomeGateKind::CantWin.lower(),
            sylex_core::OutcomeGateKind::CantWin
        );
    }

    #[test]
    fn lowers_ignore_rule_ignore_lowest() {
        assert_matches!(
            sylex_semantics_v1::IgnoreRule::IgnoreLowest.lower(),
            sylex_core::IgnoreRule::IgnoreLowest
        );
    }

    #[test]
    fn lowers_ignore_rule_ignore_chosen() {
        assert_matches!(
            sylex_semantics_v1::IgnoreRule::IgnoreChosen(0).lower(),
            sylex_core::IgnoreRule::IgnoreChosen(0)
        );
    }

    #[test]
    fn lowers_pip_class_generic() {
        assert_matches!(
            sylex_semantics_v1::PipClass::Generic.lower(),
            sylex_core::PipClass::Generic
        );
    }

    #[test]
    fn lowers_pip_class_colored() {
        assert_matches!(
            sylex_semantics_v1::PipClass::Colored(minimal_color()).lower(),
            sylex_core::PipClass::Colored(sylex_core::Color::White)
        );
    }

    #[test]
    fn lowers_pay_act_tap_to_pay() {
        assert_matches!(
            sylex_semantics_v1::PayAct::TapToPay(minimal_predicate()).lower(),
            sylex_core::PayAct::TapToPay(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_pay_act_exile_to_pay() {
        assert_matches!(
            sylex_semantics_v1::PayAct::ExileToPay(minimal_predicate()).lower(),
            sylex_core::PayAct::ExileToPay(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_player_attr_life() {
        assert_matches!(
            sylex_semantics_v1::PlayerAttr::Life.lower(),
            sylex_core::PlayerAttr::Life
        );
    }

    #[test]
    fn lowers_player_attr_hand_size() {
        assert_matches!(
            sylex_semantics_v1::PlayerAttr::HandSize.lower(),
            sylex_core::PlayerAttr::HandSize
        );
    }

    #[test]
    fn lowers_player_attr_hand_size_limit() {
        assert_matches!(
            sylex_semantics_v1::PlayerAttr::HandSizeLimit.lower(),
            sylex_core::PlayerAttr::HandSizeLimit
        );
    }

    #[test]
    fn lowers_player_attr_land_plays_per_turn() {
        assert_matches!(
            sylex_semantics_v1::PlayerAttr::LandPlaysPerTurn.lower(),
            sylex_core::PlayerAttr::LandPlaysPerTurn
        );
    }

    #[test]
    fn lowers_player_mod_set_to() {
        assert_matches!(
            sylex_semantics_v1::PlayerMod::SetTo(minimal_player_attr(), minimal_count()).lower(),
            sylex_core::PlayerMod::Set(
                sylex_core::PlayerAttr::Life,
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_player_mod_raise() {
        assert_matches!(
            sylex_semantics_v1::PlayerMod::Raise(minimal_player_attr(), minimal_count()).lower(),
            sylex_core::PlayerMod::Up(
                sylex_core::PlayerAttr::Life,
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_player_mod_lower() {
        assert_matches!(
            sylex_semantics_v1::PlayerMod::Lower(minimal_player_attr(), minimal_count()).lower(),
            sylex_core::PlayerMod::Down(
                sylex_core::PlayerAttr::Life,
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_player_mod_no_max() {
        assert_matches!(
            sylex_semantics_v1::PlayerMod::NoMax(minimal_player_attr()).lower(),
            sylex_core::PlayerMod::NoMax(sylex_core::PlayerAttr::Life)
        );
    }

    #[test]
    fn lowers_static_effect_replacement() {
        assert_matches!(
            in_spell_region(|| sylex_semantics_v1::StaticEffect::Replacement(
                std::sync::Arc::new(minimal_replacement())
            )
            .lower()),
            sylex_core::StaticSpec::Replacement(_)
        );
    }

    #[test]
    fn lowers_static_effect_sba_onto_conditionally_do() {
        assert_matches!(
            in_spell_region(|| sylex_semantics_v1::StaticEffect::Sba {
                when: std::sync::Arc::new(minimal_condition()),
                then: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            sylex_core::StaticSpec::ConditionallyDo { when: _, then: _ }
        );
    }
}

impl Lower for sylex_semantics_v1::NumericOp {
    type Target = sylex_core::NumericOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => sylex_core::NumericOp::Set(f0.lower()),
            Self::Up(f0) => sylex_core::NumericOp::Up(f0.lower()),
            Self::Down(f0) => sylex_core::NumericOp::Down(f0.lower()),
        }
    }
}

impl<T: Lower + Clone> Lower for sylex_semantics_v1::CollectionOp<T> {
    type Target = sylex_core::CollectionOp<<T as Lower>::Target>;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => sylex_core::CollectionOp::Set(f0.lower()),
            Self::Add(f0) => sylex_core::CollectionOp::Add(f0.lower()),
            Self::Remove(f0) => sylex_core::CollectionOp::Remove(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::Modification {
    type Target = sylex_core::Modification;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Power(f0) => sylex_core::Modification::Power(f0.lower()),
            Self::Toughness(f0) => sylex_core::Modification::Toughness(f0.lower()),
            Self::SwitchPowerToughness => sylex_core::Modification::SwitchPowerToughness,
            Self::Colors(f0) => sylex_core::Modification::Colors(f0.lower()),
            Self::CardTypes(f0) => sylex_core::Modification::CardTypes(f0.lower()),
            Self::Subtypes(f0) => sylex_core::Modification::Subtypes(f0.lower()),
            Self::Supertypes(f0) => sylex_core::Modification::Supertypes(f0.lower()),
            Self::GainAbility(f0) => sylex_core::Modification::GainAbility(f0.lower()),
            Self::LoseAbility(f0) => sylex_core::Modification::LoseAbility(f0.lower()),
            Self::LoseAllAbilities => sylex_core::Modification::LoseAllAbilities,
            Self::CantHaveAbility(f0) => sylex_core::Modification::CantHaveAbility(f0.lower()),
            Self::SetController(f0) => sylex_core::Modification::SetController(f0.lower()),
            Self::SetText(f0) => sylex_core::Modification::SetText(f0.lower()),
            Self::AllCreatureTypes => sylex_core::Modification::AllCreatureTypes,
            Self::BaseLoyalty(f0) => sylex_core::Modification::BaseLoyalty(f0.lower()),
            Self::BaseDefense(f0) => sylex_core::Modification::BaseDefense(f0.lower()),
            Self::BecomeBasicLandType(f0) => {
                sylex_core::Modification::BecomeBasicLandType(f0.lower())
            }
            Self::Several(f0) => sylex_core::Modification::Several(f0.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::CostChange {
    type Target = sylex_core::CostChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Increase(f0) => {
                sylex_core::CostChange::Increase(crate::cost::lower_scoped_cost_components(&f0))
            }
            Self::Reduce(f0) => {
                sylex_core::CostChange::Reduce(crate::cost::lower_scoped_cost_components(&f0))
            }
            Self::Additional { components } => sylex_core::CostChange::Additional {
                components: crate::cost::lower_scoped_cost_components(&components),
            },
            Self::Scaled { change, times } => sylex_core::CostChange::Scaled {
                change: change.lower(),
                times: times.lower(),
            },
        }
    }
}

impl Lower for sylex_semantics_v1::StaticEffect {
    type Target = sylex_core::StaticSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Modify(f0, f1) => sylex_core::StaticSpec::Modify(f0.lower(), f1.lower()),
            Self::BecomesCopy(f0, f1) => {
                sylex_core::StaticSpec::BecomesCopy(f0.lower(), f1.lower())
            }
            Self::Each(f0, f1) => sylex_core::StaticSpec::Each(
                f0.lower(),
                std::sync::Arc::new(crate::region::candidate_region(|| {
                    std::sync::Arc::unwrap_or_clone(f1).lower()
                })),
            ),
            Self::Conditionally(f0, f1) => {
                sylex_core::StaticSpec::Conditionally(f0.lower(), f1.lower())
            }
            Self::Deontic(f0) => sylex_core::StaticSpec::Deontic(f0.lower()),
            Self::CostModifier { of, change } => sylex_core::StaticSpec::CostModifier {
                of: of.lower(),
                change: change.lower(),
            },
            Self::CostOption(f0) => sylex_core::StaticSpec::CostOption(f0.lower()),
            Self::TriggerMultiplier {
                cause,
                extra,
                affected,
            } => sylex_core::StaticSpec::TriggerMultiplier {
                cause: cause.lower(),
                extra: extra.lower(),
                affected: affected.lower(),
            },
            Self::ModifyPlayer(f0, f1) => {
                sylex_core::StaticSpec::ModifyPlayer(f0.lower(), f1.lower())
            }
            Self::Replacement(f0) => sylex_core::StaticSpec::Replacement(f0.lower()),
            Self::Prevention(f0) => sylex_core::StaticSpec::Prevention(f0.lower()),
            Self::CantPrevent { from, to } => sylex_core::StaticSpec::CantPrevent {
                from: from.lower(),
                to: to.lower(),
            },
            Self::SpendAsThough { mana_from, as_ } => sylex_core::StaticSpec::SpendAsThough {
                mana_from: mana_from.lower(),
                as_: as_.lower(),
            },
            Self::AsThough(f0) => sylex_core::StaticSpec::AsThough(f0.lower()),
            Self::Sba { when, then } => sylex_core::StaticSpec::ConditionallyDo {
                when: when.lower(),
                then: then.lower(),
            },
            Self::OutcomeGate { who, gate } => sylex_core::StaticSpec::OutcomeGate {
                who: who.lower(),
                gate: gate.lower(),
            },
            Self::CantHappen(f0) => sylex_core::StaticSpec::CantHappen(f0.lower()),
            Self::ReplaceRoll {
                query,
                extra,
                ignore,
            } => sylex_core::StaticSpec::ReplaceRoll {
                query: query.lower(),
                extra: extra.lower(),
                ignore: ignore.lower(),
            },
            Self::PayPips(f0, f1) => sylex_core::StaticSpec::PayPips(f0.lower(), f1.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::OutcomeGateKind {
    type Target = sylex_core::OutcomeGateKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CantLose => sylex_core::OutcomeGateKind::CantLose,
            Self::CantWin => sylex_core::OutcomeGateKind::CantWin,
        }
    }
}

impl Lower for sylex_semantics_v1::IgnoreRule {
    type Target = sylex_core::IgnoreRule;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IgnoreLowest => sylex_core::IgnoreRule::IgnoreLowest,
            Self::IgnoreChosen(f0) => sylex_core::IgnoreRule::IgnoreChosen(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::PipClass {
    type Target = sylex_core::PipClass;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic => sylex_core::PipClass::Generic,
            Self::Colored(f0) => sylex_core::PipClass::Colored(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::PayAct {
    type Target = sylex_core::PayAct;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::TapToPay(f0) => sylex_core::PayAct::TapToPay(f0.lower()),
            Self::ExileToPay(f0) => sylex_core::PayAct::ExileToPay(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::PlayerAttr {
    type Target = sylex_core::PlayerAttr;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Life => sylex_core::PlayerAttr::Life,
            Self::HandSize => sylex_core::PlayerAttr::HandSize,
            Self::HandSizeLimit => sylex_core::PlayerAttr::HandSizeLimit,
            Self::LandPlaysPerTurn => sylex_core::PlayerAttr::LandPlaysPerTurn,
        }
    }
}

impl Lower for sylex_semantics_v1::PlayerMod {
    type Target = sylex_core::PlayerMod;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SetTo(f0, f1) => sylex_core::PlayerMod::Set(f0.lower(), f1.lower()),
            Self::Raise(f0, f1) => sylex_core::PlayerMod::Up(f0.lower(), f1.lower()),
            Self::Lower(f0, f1) => sylex_core::PlayerMod::Down(f0.lower(), f1.lower()),
            Self::NoMax(f0) => sylex_core::PlayerMod::NoMax(f0.lower()),
        }
    }
}
