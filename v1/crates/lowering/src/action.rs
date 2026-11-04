//! `action` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for sylex_semantics_v1::Anchor {
    type Target = sylex_core::Anchor;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FromTop(offset) => sylex_core::Anchor::FromTop(offset.lower()),
            Self::FromBottom(offset) => sylex_core::Anchor::FromBottom(offset.lower()),
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
    fn lowers_anchor_from_top() {
        assert_matches!(
            sylex_semantics_v1::Anchor::FromTop(minimal_count()).lower(),
            sylex_core::Anchor::FromTop(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_anchor_from_bottom() {
        assert_matches!(
            sylex_semantics_v1::Anchor::FromBottom(minimal_count()).lower(),
            sylex_core::Anchor::FromBottom(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_destination_zone() {
        assert_matches!(
            sylex_semantics_v1::Destination::Zone(minimal_zone()).lower(),
            sylex_core::Destination::Zone(sylex_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_destination_library() {
        assert_matches!(
            sylex_semantics_v1::Destination::Library(minimal_anchor()).lower(),
            sylex_core::Destination::Library(sylex_core::Anchor::FromTop(
                sylex_core::Count::Literal(0)
            ))
        );
    }

    #[test]
    fn lowers_enter_rider_tapped() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::Tapped.lower(),
            sylex_core::EnterRider::Tapped
        );
    }

    #[test]
    fn lowers_enter_rider_face_down() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::FaceDown.lower(),
            sylex_core::EnterRider::FaceDown
        );
    }

    #[test]
    fn lowers_enter_rider_under_control_of() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::UnderControlOf(minimal_reference()).lower(),
            sylex_core::EnterRider::UnderControlOf(sylex_core::Reference::Reg(sylex_core::RefId(
                0
            )))
        );
    }

    #[test]
    fn lowers_enter_rider_under_owners_control() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::UnderOwnersControl.lower(),
            sylex_core::EnterRider::UnderOwnersControl
        );
    }

    #[test]
    fn lowers_enter_rider_attacking() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::Attacking(None).lower(),
            sylex_core::EnterRider::Attacking(None)
        );
    }

    #[test]
    fn lowers_enter_rider_with_counters() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::WithCounters(minimal_counter_ref(), minimal_count())
                .lower(),
            sylex_core::EnterRider::WithCounters(
                sylex_core::CounterRef(_),
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_enter_rider_as_copy() {
        assert_matches!(
            sylex_semantics_v1::EnterRider::AsCopy(minimal_copy_spec()).lower(),
            sylex_core::EnterRider::AsCopy(sylex_core::CopySpec {
                source: sylex_core::CopySource::Object(sylex_core::Reference::Reg(
                    sylex_core::RefId(0)
                )),
                exceptions: _
            })
        );
    }

    #[test]
    fn lowers_arrangement_chosen_order() {
        assert_matches!(
            sylex_semantics_v1::Arrangement::ChosenOrder(minimal_reference()).lower(),
            sylex_core::Arrangement::ChosenOrder(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_arrangement_any_order() {
        assert_matches!(
            sylex_semantics_v1::Arrangement::AnyOrder.lower(),
            sylex_core::Arrangement::AnyOrder
        );
    }

    #[test]
    fn lowers_arrangement_same_order() {
        assert_matches!(
            sylex_semantics_v1::Arrangement::SameOrder.lower(),
            sylex_core::Arrangement::SameOrder
        );
    }

    #[test]
    fn lowers_arrangement_random_order() {
        assert_matches!(
            sylex_semantics_v1::Arrangement::RandomOrder.lower(),
            sylex_core::Arrangement::RandomOrder
        );
    }

    #[test]
    fn lowers_action_deal_damage() {
        assert_matches!(
            sylex_semantics_v1::Action::DealDamage(
                minimal_reference(),
                minimal_count(),
                minimal_reference()
            )
            .lower(),
            sylex_core::Action::DealDamage(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Count::Literal(0),
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_counter() {
        assert_matches!(
            sylex_semantics_v1::Action::Counter(minimal_reference()).lower(),
            sylex_core::Action::Counter(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_transform() {
        assert_matches!(
            sylex_semantics_v1::Action::Transform(minimal_reference()).lower(),
            sylex_core::Action::Transform(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_cease() {
        assert_matches!(
            sylex_semantics_v1::Action::Cease(minimal_reference()).lower(),
            sylex_core::Action::Cease(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_attach() {
        assert_matches!(
            sylex_semantics_v1::Action::Attach {
                what: minimal_reference(),
                to: minimal_reference()
            }
            .lower(),
            sylex_core::Action::Attach {
                what: sylex_core::Reference::Reg(sylex_core::RefId(0)),
                to: sylex_core::Reference::Reg(sylex_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_action_unattach() {
        assert_matches!(
            sylex_semantics_v1::Action::Unattach(minimal_reference()).lower(),
            sylex_core::Action::Unattach(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_move() {
        assert_matches!(
            sylex_semantics_v1::Action::Move(
                minimal_reference(),
                minimal_destination(),
                [].into(),
                None
            )
            .lower(),
            sylex_core::Action::Move(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Destination::Zone(sylex_core::Zone::Battlefield),
                _,
                None
            )
        );
    }

    #[test]
    fn lowers_action_move_group() {
        assert_matches!(
            sylex_semantics_v1::Action::MoveGroup {
                group: minimal_selection(),
                arrangement: minimal_arrangement(),
                to: minimal_destination(),
                riders: [].into()
            }
            .lower(),
            sylex_core::Action::MoveGroup {
                group: sylex_core::Selection::SelectAll(_),
                arrangement: sylex_core::Arrangement::ChosenOrder(sylex_core::Reference::Reg(
                    sylex_core::RefId(0)
                )),
                to: sylex_core::Destination::Zone(sylex_core::Zone::Battlefield),
                riders: _
            }
        );
    }

    #[test]
    fn lowers_action_gain_control() {
        assert_matches!(
            sylex_semantics_v1::Action::GainControl(minimal_reference(), minimal_reference())
                .lower(),
            sylex_core::Action::GainControl(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_extra_phase() {
        assert_matches!(
            sylex_semantics_v1::Action::ExtraPhase(minimal_phase_kind(), minimal_reference())
                .lower(),
            sylex_core::Action::ExtraPhase(
                sylex_core::PhaseKind::Beginning,
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_move_counters() {
        assert_matches!(
            sylex_semantics_v1::Action::MoveCounters(
                minimal_counter_spec(),
                minimal_reference(),
                minimal_reference()
            )
            .lower(),
            sylex_core::Action::MoveCounters(
                sylex_core::CounterSpec::Named(
                    sylex_core::CounterRef(_),
                    sylex_core::Count::Literal(0)
                ),
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_change_life() {
        assert_matches!(
            sylex_semantics_v1::Action::ChangeLife(minimal_reference(), minimal_life_op()).lower(),
            sylex_core::Action::ChangeLife(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::LifeOp::Set(sylex_core::Count::Literal(0))
            )
        );
    }

    #[test]
    fn lowers_action_add_mana() {
        assert_matches!(
            sylex_semantics_v1::Action::AddMana(
                minimal_reference(),
                minimal_count(),
                minimal_mana_production()
            )
            .lower(),
            sylex_core::Action::AddMana(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Count::Literal(0),
                sylex_core::ManaProduction::WithRiders {
                    mana: sylex_core::ManaSpec::AnyColor,
                    riders: _
                }
            )
        );
    }

    #[test]
    fn lowers_action_create() {
        assert_matches!(
            sylex_semantics_v1::Action::Create {
                agent: minimal_reference(),
                count: minimal_count(),
                token: minimal_token_spec(),
                riders: [].into()
            }
            .lower(),
            sylex_core::Action::Create {
                agent: sylex_core::Reference::Reg(sylex_core::RefId(0)),
                count: sylex_core::Count::Literal(0),
                token: sylex_core::TokenSpec::Token(_),
                riders: _
            }
        );
    }

    #[test]
    fn lowers_action_sacrifice() {
        assert_matches!(
            sylex_semantics_v1::Action::Sacrifice(minimal_reference(), minimal_reference()).lower(),
            sylex_core::Action::Sacrifice(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_draw_card() {
        assert_matches!(
            sylex_semantics_v1::Action::DrawCard(minimal_reference()).lower(),
            sylex_core::Action::DrawCard(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_tap() {
        assert_matches!(
            sylex_semantics_v1::Action::Tap(minimal_reference()).lower(),
            sylex_core::Action::Tap(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_untap() {
        assert_matches!(
            sylex_semantics_v1::Action::Untap(minimal_reference()).lower(),
            sylex_core::Action::Untap(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_get_emblem() {
        assert_matches!(
            sylex_semantics_v1::Action::GetEmblem(minimal_reference(), [].into()).lower(),
            sylex_core::Action::GetEmblem(sylex_core::Reference::Reg(sylex_core::RefId(0)), _)
        );
    }

    #[test]
    fn lowers_action_get_designation() {
        assert_matches!(
            sylex_semantics_v1::Action::GetDesignation(minimal_reference(), "X".into()).lower(),
            sylex_core::Action::GetDesignation(sylex_core::Reference::Reg(sylex_core::RefId(0)), _)
        );
    }

    #[test]
    fn lowers_action_set_game_designation() {
        assert_matches!(
            sylex_semantics_v1::Action::SetGameDesignation("X".into(), "Y".into()).lower(),
            sylex_core::Action::SetGameDesignation(name, value)
                if name.as_ref() == "X" && value.as_ref() == "Y"
        );
    }

    #[test]
    fn lowers_action_choose_value() {
        assert_matches!(
            sylex_semantics_v1::Action::ChooseValue(
                minimal_reference(),
                minimal_chosen_value_kind(),
                "X".into()
            )
            .lower(),
            sylex_core::Action::ChooseValue(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::ChosenValueKind::Color,
                _
            )
        );
    }

    #[test]
    fn lowers_action_copy_spell() {
        assert_matches!(
            sylex_semantics_v1::Action::CopySpell {
                controller: minimal_reference(),
                spec: minimal_copy_spec(),
                retarget: minimal_copy_retarget()
            }
            .lower(),
            sylex_core::Action::CopySpell {
                controller: sylex_core::Reference::Reg(sylex_core::RefId(0)),
                spec: sylex_core::CopySpec {
                    source: sylex_core::CopySource::Object(sylex_core::Reference::Reg(
                        sylex_core::RefId(0)
                    )),
                    exceptions: _
                },
                retarget: sylex_core::CopyRetarget::AsIs
            }
        );
    }

    #[test]
    fn lowers_action_cast_copy() {
        assert_matches!(
            sylex_semantics_v1::Action::CastCopy(minimal_reference(), minimal_copy_spec()).lower(),
            sylex_core::Action::CastCopy(
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
    fn lowers_action_cast() {
        assert_matches!(
            sylex_semantics_v1::Action::Cast(minimal_reference(), minimal_reference(), None)
                .lower(),
            sylex_core::Action::Cast(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                None
            )
        );
    }

    #[test]
    fn lowers_action_retarget() {
        assert_matches!(
            sylex_semantics_v1::Action::Retarget {
                mode: minimal_retarget_mode(),
                of: minimal_reference(),
                by: minimal_reference()
            }
            .lower(),
            sylex_core::Action::Retarget {
                mode: sylex_core::RetargetMode::ChangeAll,
                of: sylex_core::Reference::Reg(sylex_core::RefId(0)),
                by: sylex_core::Reference::Reg(sylex_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_action_flip_coins() {
        assert_matches!(
            sylex_semantics_v1::Action::FlipCoins(minimal_reference(), minimal_count(), false)
                .lower(),
            sylex_core::Action::FlipCoins(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Count::Literal(0),
                false
            )
        );
    }

    #[test]
    fn lowers_action_roll_dice() {
        assert_matches!(
            sylex_semantics_v1::Action::RollDice(minimal_reference(), minimal_count(), 0).lower(),
            sylex_core::Action::RollDice(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Count::Literal(0),
                0
            )
        );
    }

    #[test]
    fn lowers_action_roll_planar_die() {
        assert_matches!(
            sylex_semantics_v1::Action::RollPlanarDie(minimal_reference()).lower(),
            sylex_core::Action::RollPlanarDie(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_put_counters() {
        assert_matches!(
            sylex_semantics_v1::Action::PutCounters(
                minimal_reference(),
                minimal_counter_ref(),
                minimal_count()
            )
            .lower(),
            sylex_core::Action::PutCounters(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::CounterRef(_),
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_action_remove_counters() {
        assert_matches!(
            sylex_semantics_v1::Action::RemoveCounters(
                minimal_reference(),
                minimal_counter_ref(),
                minimal_count()
            )
            .lower(),
            sylex_core::Action::RemoveCounters(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::CounterRef(_),
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_action_win_game() {
        assert_matches!(
            sylex_semantics_v1::Action::WinGame(minimal_reference()).lower(),
            sylex_core::Action::WinGame(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_lose_game() {
        assert_matches!(
            sylex_semantics_v1::Action::LoseGame(minimal_reference()).lower(),
            sylex_core::Action::LoseGame(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_restart_game() {
        assert_matches!(
            sylex_semantics_v1::Action::RestartGame.lower(),
            sylex_core::Action::RestartGame
        );
    }

    #[test]
    fn lowers_action_shuffle() {
        assert_matches!(
            sylex_semantics_v1::Action::Shuffle(minimal_selection()).lower(),
            sylex_core::Action::Shuffle(sylex_core::Selection::SelectAll(_))
        );
    }

    #[test]
    fn lowers_action_reveal() {
        assert_matches!(
            sylex_semantics_v1::Action::Reveal {
                what: minimal_reference(),
                to: None
            }
            .lower(),
            sylex_core::Action::Reveal {
                what: sylex_core::Reference::Reg(sylex_core::RefId(0)),
                to: None
            }
        );
    }

    #[test]
    fn lowers_action_remove_damage() {
        assert_matches!(
            sylex_semantics_v1::Action::RemoveDamage(minimal_reference()).lower(),
            sylex_core::Action::RemoveDamage(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_pay() {
        assert_matches!(
            sylex_semantics_v1::Action::Pay(minimal_cost()).lower(),
            sylex_core::Action::Pay(sylex_core::Cost(_))
        );
    }

    #[test]
    fn lowers_action_expanded() {
        assert_matches!(
            sylex_semantics_v1::Action::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_action())
            })
            .lower(),
            sylex_core::Action::DealDamage(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Count::Literal(0),
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_life_op_set() {
        assert_matches!(
            sylex_semantics_v1::LifeOp::Set(minimal_count()).lower(),
            sylex_core::LifeOp::Set(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_life_op_up() {
        assert_matches!(
            sylex_semantics_v1::LifeOp::Up(minimal_count()).lower(),
            sylex_core::LifeOp::Up(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_life_op_down() {
        assert_matches!(
            sylex_semantics_v1::LifeOp::Down(minimal_count()).lower(),
            sylex_core::LifeOp::Down(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_retarget_mode_change_all() {
        assert_matches!(
            sylex_semantics_v1::RetargetMode::ChangeAll.lower(),
            sylex_core::RetargetMode::ChangeAll
        );
    }

    #[test]
    fn lowers_retarget_mode_change_one() {
        assert_matches!(
            sylex_semantics_v1::RetargetMode::ChangeOne.lower(),
            sylex_core::RetargetMode::ChangeOne
        );
    }

    #[test]
    fn lowers_retarget_mode_change_any() {
        assert_matches!(
            sylex_semantics_v1::RetargetMode::ChangeAny.lower(),
            sylex_core::RetargetMode::ChangeAny
        );
    }

    #[test]
    fn lowers_retarget_mode_choose_new() {
        assert_matches!(
            sylex_semantics_v1::RetargetMode::ChooseNew.lower(),
            sylex_core::RetargetMode::ChooseNew
        );
    }

    #[test]
    fn lowers_copy_retarget_as_is() {
        assert_matches!(
            sylex_semantics_v1::CopyRetarget::AsIs.lower(),
            sylex_core::CopyRetarget::AsIs
        );
    }

    #[test]
    fn lowers_copy_retarget_may_choose_new() {
        assert_matches!(
            sylex_semantics_v1::CopyRetarget::MayChooseNew.lower(),
            sylex_core::CopyRetarget::MayChooseNew
        );
    }

    #[test]
    fn lowers_copy_retarget_targets_that() {
        assert_matches!(
            sylex_semantics_v1::CopyRetarget::TargetsThat(minimal_reference()).lower(),
            sylex_core::CopyRetarget::TargetsThat(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_composite() {
        assert_matches!(
            in_spell_region(|| sylex_semantics_v1::Action::Composite {
                name: minimal_verb_name(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            sylex_core::Action::Composite {
                name: sylex_core::VerbName(_),
                body: _
            }
        );
    }

    #[test]
    fn lowers_action_create_replacement() {
        assert_matches!(
            in_spell_region(|| sylex_semantics_v1::Action::CreateReplacement {
                replacement: std::sync::Arc::new(minimal_replacement()),
                duration: minimal_duration(),
                one_shot: false
            }
            .lower()),
            sylex_core::Action::CreateReplacement {
                subject: sylex_core::Reference::Reg(_),
                replacement: _,
                duration: sylex_core::Duration::FixedUntil(sylex_core::TurnMarker::EndOfTurn),
                one_shot: false
            }
        );
    }
}

impl Lower for sylex_semantics_v1::Destination {
    type Target = sylex_core::Destination;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Zone(zone) => sylex_core::Destination::Zone(zone.lower()),
            Self::Library(anchor) => sylex_core::Destination::Library(anchor.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::EnterRider {
    type Target = sylex_core::EnterRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => sylex_core::EnterRider::Tapped,
            Self::FaceDown => sylex_core::EnterRider::FaceDown,
            Self::UnderControlOf(who) => sylex_core::EnterRider::UnderControlOf(who.lower()),
            Self::UnderOwnersControl => sylex_core::EnterRider::UnderOwnersControl,
            Self::Attacking(who) => sylex_core::EnterRider::Attacking(who.lower()),
            Self::WithCounters(kind, count) => {
                sylex_core::EnterRider::WithCounters(kind.lower(), count.lower())
            }
            Self::AsCopy(spec) => sylex_core::EnterRider::AsCopy(spec.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::Arrangement {
    type Target = sylex_core::Arrangement;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ChosenOrder(by) => sylex_core::Arrangement::ChosenOrder(by.lower()),
            Self::AnyOrder => sylex_core::Arrangement::AnyOrder,
            Self::SameOrder => sylex_core::Arrangement::SameOrder,
            Self::RandomOrder => sylex_core::Arrangement::RandomOrder,
        }
    }
}

impl Lower for sylex_semantics_v1::Action {
    type Target = sylex_core::Action;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::DealDamage(source, amount, target) => {
                sylex_core::Action::DealDamage(source.lower(), amount.lower(), target.lower())
            }
            Self::Counter(what) => sylex_core::Action::Counter(what.lower()),
            Self::Transform(what) => sylex_core::Action::Transform(what.lower()),
            Self::Cease(what) => sylex_core::Action::Cease(what.lower()),
            Self::Attach { what, to } => sylex_core::Action::Attach {
                what: what.lower(),
                to: to.lower(),
            },
            Self::Unattach(what) => sylex_core::Action::Unattach(what.lower()),
            Self::Move(what, to, riders, from) => {
                sylex_core::Action::Move(what.lower(), to.lower(), riders.lower(), from.lower())
            }
            Self::MoveGroup {
                group,
                arrangement,
                to,
                riders,
            } => sylex_core::Action::MoveGroup {
                group: group.lower(),
                arrangement: arrangement.lower(),
                to: to.lower(),
                riders: riders.lower(),
            },
            Self::GainControl(what, to) => {
                sylex_core::Action::GainControl(what.lower(), to.lower())
            }
            Self::ExtraPhase(phase, who) => {
                sylex_core::Action::ExtraPhase(phase.lower(), who.lower())
            }
            Self::MoveCounters(spec, from, to) => {
                sylex_core::Action::MoveCounters(spec.lower(), from.lower(), to.lower())
            }
            Self::CreateReplacement {
                replacement,
                duration,
                one_shot,
            } => sylex_core::Action::CreateReplacement {
                // [CR#614.1]: a replacement effect acts like a shield around
                // WHATEVER IT AFFECTS. English names the affected permanent by
                // discourse ("the next time IT would be destroyed"), so the
                // shield's subject is the region's nearest permanent-sorted
                // antecedent, resolved HERE (reference is resolved once, in lowering) and declared
                // as a register the engine reads by index. R1/R2 refuse an
                // ambiguous one at compile time; a shield with no antecedent
                // at all falls back to the source parameter, which is what a
                // self-regenerating permanent means.
                subject: sylex_core::Reference::Reg(
                    crate::region::that(sylex_semantics_v1::Sort::Permanent)
                        .or_else(crate::region::it)
                        .unwrap_or_else(|| {
                            crate::region::source().expect(
                                "a replacement shield is created inside a region with a source",
                            )
                        }),
                ),
                replacement: replacement.lower(),
                duration: duration.lower(),
                one_shot: one_shot.lower(),
            },
            Self::Composite { name, body } => sylex_core::Action::Composite {
                name: name.lower(),
                body: body.lower(),
            },
            Self::ChangeLife(patient, operation) => {
                sylex_core::Action::ChangeLife(patient.lower(), operation.lower())
            }
            Self::AddMana(recipient, count, production) => {
                sylex_core::Action::AddMana(recipient.lower(), count.lower(), production.lower())
            }
            Self::Create {
                agent,
                count,
                token,
                riders,
            } => sylex_core::Action::Create {
                agent: agent.lower(),
                count: count.lower(),
                token: token.lower(),
                riders: riders.lower(),
            },
            Self::Sacrifice(agent, what) => {
                sylex_core::Action::Sacrifice(agent.lower(), what.lower())
            }
            Self::DrawCard(agent) => sylex_core::Action::DrawCard(agent.lower()),
            Self::Tap(what) => sylex_core::Action::Tap(what.lower()),
            Self::Untap(what) => sylex_core::Action::Untap(what.lower()),
            Self::GetEmblem(recipient, abilities) => {
                sylex_core::Action::GetEmblem(recipient.lower(), abilities.lower())
            }
            Self::GetDesignation(recipient, name) => {
                sylex_core::Action::GetDesignation(recipient.lower(), name.lower())
            }
            Self::SetGameDesignation(name, value) => {
                sylex_core::Action::SetGameDesignation(name.lower(), value.lower())
            }
            Self::ChooseValue(who, kind, note) => {
                sylex_core::Action::ChooseValue(who.lower(), kind.lower(), note.lower())
            }
            Self::CopySpell {
                controller,
                spec,
                retarget,
            } => sylex_core::Action::CopySpell {
                controller: controller.lower(),
                spec: spec.lower(),
                retarget: retarget.lower(),
            },
            Self::CastCopy(agent, spec) => {
                sylex_core::Action::CastCopy(agent.lower(), spec.lower())
            }
            Self::Cast(agent, what, cost) => {
                sylex_core::Action::Cast(agent.lower(), what.lower(), cost.lower())
            }
            Self::Retarget { mode, of, by } => sylex_core::Action::Retarget {
                mode: mode.lower(),
                of: of.lower(),
                by: by.lower(),
            },
            Self::FlipCoins(agent, count, called) => {
                sylex_core::Action::FlipCoins(agent.lower(), count.lower(), called.lower())
            }
            Self::RollDice(agent, count, sides) => {
                sylex_core::Action::RollDice(agent.lower(), count.lower(), sides.lower())
            }
            Self::RollPlanarDie(agent) => sylex_core::Action::RollPlanarDie(agent.lower()),
            Self::PutCounters(what, kind, count) => {
                sylex_core::Action::PutCounters(what.lower(), kind.lower(), count.lower())
            }
            Self::RemoveCounters(what, kind, count) => {
                sylex_core::Action::RemoveCounters(what.lower(), kind.lower(), count.lower())
            }
            Self::WinGame(who) => sylex_core::Action::WinGame(who.lower()),
            Self::LoseGame(who) => sylex_core::Action::LoseGame(who.lower()),
            Self::RestartGame => sylex_core::Action::RestartGame,
            Self::Shuffle(what) => sylex_core::Action::Shuffle(what.lower()),
            Self::Reveal { what, to } => sylex_core::Action::Reveal {
                what: what.lower(),
                to: to.lower(),
            },
            Self::RemoveDamage(what) => sylex_core::Action::RemoveDamage(what.lower()),
            Self::Pay(cost) => sylex_core::Action::Pay(cost.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(expansion) => *expansion.value.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::LifeOp {
    type Target = sylex_core::LifeOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(value) => sylex_core::LifeOp::Set(value.lower()),
            Self::Up(amount) => sylex_core::LifeOp::Up(amount.lower()),
            Self::Down(amount) => sylex_core::LifeOp::Down(amount.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::RetargetMode {
    type Target = sylex_core::RetargetMode;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ChangeAll => sylex_core::RetargetMode::ChangeAll,
            Self::ChangeOne => sylex_core::RetargetMode::ChangeOne,
            Self::ChangeAny => sylex_core::RetargetMode::ChangeAny,
            Self::ChooseNew => sylex_core::RetargetMode::ChooseNew,
        }
    }
}

impl Lower for sylex_semantics_v1::CopyRetarget {
    type Target = sylex_core::CopyRetarget;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AsIs => sylex_core::CopyRetarget::AsIs,
            Self::MayChooseNew => sylex_core::CopyRetarget::MayChooseNew,
            Self::TargetsThat(what) => sylex_core::CopyRetarget::TargetsThat(what.lower()),
        }
    }
}
