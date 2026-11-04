//! `count` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for sylex_semantics_v1::Stat {
    type Target = sylex_core::Stat;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Power => sylex_core::Stat::Power,
            Self::Toughness => sylex_core::Stat::Toughness,
            Self::ManaValue => sylex_core::Stat::ManaValue,
            Self::Loyalty => sylex_core::Stat::Loyalty,
            Self::Defense => sylex_core::Stat::Defense,
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
    fn lowers_stat_power() {
        assert_matches!(
            sylex_semantics_v1::Stat::Power.lower(),
            sylex_core::Stat::Power
        );
    }

    #[test]
    fn lowers_stat_toughness() {
        assert_matches!(
            sylex_semantics_v1::Stat::Toughness.lower(),
            sylex_core::Stat::Toughness
        );
    }

    #[test]
    fn lowers_stat_mana_value() {
        assert_matches!(
            sylex_semantics_v1::Stat::ManaValue.lower(),
            sylex_core::Stat::ManaValue
        );
    }

    #[test]
    fn lowers_stat_loyalty() {
        assert_matches!(
            sylex_semantics_v1::Stat::Loyalty.lower(),
            sylex_core::Stat::Loyalty
        );
    }

    #[test]
    fn lowers_stat_defense() {
        assert_matches!(
            sylex_semantics_v1::Stat::Defense.lower(),
            sylex_core::Stat::Defense
        );
    }

    #[test]
    fn lowers_round_mode_round_up() {
        assert_matches!(
            sylex_semantics_v1::RoundMode::RoundUp.lower(),
            sylex_core::RoundMode::RoundUp
        );
    }

    #[test]
    fn lowers_round_mode_round_down() {
        assert_matches!(
            sylex_semantics_v1::RoundMode::RoundDown.lower(),
            sylex_core::RoundMode::RoundDown
        );
    }

    #[test]
    fn lowers_characteristic_colors() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Colors.lower(),
            sylex_core::Characteristic::Colors
        );
    }

    #[test]
    fn lowers_characteristic_types() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Types.lower(),
            sylex_core::Characteristic::Types
        );
    }

    #[test]
    fn lowers_characteristic_subtypes() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Subtypes.lower(),
            sylex_core::Characteristic::Subtypes
        );
    }

    #[test]
    fn lowers_characteristic_basic_land_types() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::BasicLandTypes.lower(),
            sylex_core::Characteristic::BasicLandTypes
        );
    }

    #[test]
    fn lowers_characteristic_supertypes() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Supertypes.lower(),
            sylex_core::Characteristic::Supertypes
        );
    }

    #[test]
    fn lowers_characteristic_power() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Power.lower(),
            sylex_core::Characteristic::Power
        );
    }

    #[test]
    fn lowers_characteristic_toughness() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Toughness.lower(),
            sylex_core::Characteristic::Toughness
        );
    }

    #[test]
    fn lowers_characteristic_defense() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Defense.lower(),
            sylex_core::Characteristic::Defense
        );
    }

    #[test]
    fn lowers_characteristic_mana_cost() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::ManaCost.lower(),
            sylex_core::Characteristic::ManaCost
        );
    }

    #[test]
    fn lowers_characteristic_name() {
        assert_matches!(
            sylex_semantics_v1::Characteristic::Name.lower(),
            sylex_core::Characteristic::Name
        );
    }

    #[test]
    fn lowers_countable_objects() {
        assert_matches!(
            sylex_semantics_v1::Countable::Objects(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            sylex_core::Countable::Objects(_)
        );
    }

    #[test]
    fn lowers_countable_players() {
        assert_matches!(
            sylex_semantics_v1::Countable::Players(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            sylex_core::Countable::Players(_)
        );
    }

    #[test]
    fn lowers_countable_mana_symbols() {
        assert_matches!(
            sylex_semantics_v1::Countable::ManaSymbols(
                std::sync::Arc::new(minimal_reference()),
                minimal_symbol_pred()
            )
            .lower(),
            sylex_core::Countable::ManaSymbols(_, sylex_core::SymbolPred::AnyColor)
        );
    }

    #[test]
    fn lowers_countable_singleton() {
        assert_matches!(
            sylex_semantics_v1::Countable::Singleton(std::sync::Arc::new(minimal_reference()))
                .lower(),
            sylex_core::Countable::Singleton(_)
        );
    }

    #[test]
    fn lowers_countable_mana_spent_matching() {
        assert_matches!(
            sylex_semantics_v1::Countable::ManaSpentMatching(
                std::sync::Arc::new(minimal_reference()),
                minimal_symbol_pred()
            )
            .lower(),
            sylex_core::Countable::ManaSpentMatching(_, sylex_core::SymbolPred::AnyColor)
        );
    }

    #[test]
    fn lowers_aggregate_op_sum_of() {
        assert_matches!(
            sylex_semantics_v1::AggregateOp::SumOf.lower(),
            sylex_core::AggregateOp::SumOf
        );
    }

    #[test]
    fn lowers_aggregate_op_min_of() {
        assert_matches!(
            sylex_semantics_v1::AggregateOp::MinOf.lower(),
            sylex_core::AggregateOp::MinOf
        );
    }

    #[test]
    fn lowers_aggregate_op_max_of() {
        assert_matches!(
            sylex_semantics_v1::AggregateOp::MaxOf.lower(),
            sylex_core::AggregateOp::MaxOf
        );
    }

    #[test]
    fn lowers_aggregate_op_average_of() {
        assert_matches!(
            sylex_semantics_v1::AggregateOp::AverageOf(minimal_round_mode()).lower(),
            sylex_core::AggregateOp::AverageOf(sylex_core::RoundMode::RoundUp)
        );
    }

    #[test]
    fn lowers_projection() {
        assert_matches!(
            sylex_semantics_v1::Projection {
                of: minimal_countable(),
                by: std::sync::Arc::new(minimal_count())
            }
            .lower(),
            sylex_core::Projection {
                of: sylex_core::Countable::Objects(_),
                by: _
            }
        );
    }

    #[test]
    fn lowers_count_x() {
        use sylex_semantics_v1::Count as SemValue;

        let (params, lowered) =
            crate::region::in_region(crate::region::RegionKind::Spell, 0, || SemValue::X.lower());
        let x = params
            .iter()
            .find(|param| param.provenance == sylex_core::Provenance::AnnouncedX)
            .expect("spell regions declare X");
        assert_eq!(lowered, sylex_core::Count::Reg(x.def.into()));
    }

    #[test]
    fn lowers_count_count_of() {
        assert_matches!(
            sylex_semantics_v1::Count::CountOf(minimal_countable()).lower(),
            sylex_core::Count::CountOf(sylex_core::Countable::Objects(_))
        );
    }

    #[test]
    fn lowers_count_count_distinct() {
        assert_matches!(
            sylex_semantics_v1::Count::CountDistinct(minimal_characteristic(), minimal_countable())
                .lower(),
            sylex_core::Count::CountDistinct(
                sylex_core::Characteristic::Colors,
                sylex_core::Countable::Objects(_)
            )
        );
    }

    #[test]
    fn lowers_count_stat_of() {
        assert_matches!(
            sylex_semantics_v1::Count::StatOf(minimal_reference(), minimal_stat()).lower(),
            sylex_core::Count::StatOf(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Stat::Power
            )
        );
    }

    #[test]
    fn lowers_count_player_stat_of() {
        assert_matches!(
            sylex_semantics_v1::Count::PlayerStatOf(minimal_reference(), minimal_player_attr())
                .lower(),
            sylex_core::Count::PlayerStatOf(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::PlayerAttr::Life
            )
        );
    }

    #[test]
    fn lowers_count_opponents() {
        assert_matches!(
            sylex_semantics_v1::Count::Opponents(minimal_reference()).lower(),
            sylex_core::Count::Opponents(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_count_counter_count() {
        assert_matches!(
            sylex_semantics_v1::Count::CounterCount(
                std::sync::Arc::new(minimal_reference()),
                minimal_counter_ref()
            )
            .lower(),
            sylex_core::Count::CounterCount(_, sylex_core::CounterRef(_))
        );
    }

    #[test]
    fn lowers_count_min() {
        assert_matches!(
            sylex_semantics_v1::Count::Min(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Min(_, _)
        );
    }

    #[test]
    fn lowers_count_max() {
        assert_matches!(
            sylex_semantics_v1::Count::Max(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Max(_, _)
        );
    }

    #[test]
    fn lowers_count_plus() {
        assert_matches!(
            sylex_semantics_v1::Count::Plus(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Plus(_, _)
        );
    }

    #[test]
    fn lowers_count_minus() {
        assert_matches!(
            sylex_semantics_v1::Count::Minus(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Minus(_, _)
        );
    }

    #[test]
    fn lowers_count_times() {
        assert_matches!(
            sylex_semantics_v1::Count::Times(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Times(_, _)
        );
    }

    #[test]
    fn lowers_count_half() {
        assert_matches!(
            sylex_semantics_v1::Count::Half(
                minimal_round_mode(),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Half(sylex_core::RoundMode::RoundUp, _)
        );
    }

    #[test]
    fn lowers_count_divide() {
        assert_matches!(
            sylex_semantics_v1::Count::Divide(
                minimal_round_mode(),
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Divide(sylex_core::RoundMode::RoundUp, _, _)
        );
    }

    #[test]
    fn lowers_count_mod() {
        assert_matches!(
            sylex_semantics_v1::Count::Mod(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Mod(_, _)
        );
    }

    #[test]
    fn lowers_count_pow() {
        assert_matches!(
            sylex_semantics_v1::Count::Pow(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            sylex_core::Count::Pow(_, _)
        );
    }

    #[test]
    fn lowers_count_targets_of() {
        assert_matches!(
            sylex_semantics_v1::Count::TargetsOf(minimal_reference()).lower(),
            sylex_core::Count::TargetsOf(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_count_event_count() {
        assert_matches!(
            sylex_semantics_v1::Count::EventCount(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            sylex_core::Count::EventCount(_, sylex_core::Lookback::ThisTurn)
        );
    }

    #[test]
    fn lowers_count_event_sum() {
        assert_matches!(
            sylex_semantics_v1::Count::EventSum(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            sylex_core::Count::EventSum(_, sylex_core::Lookback::ThisTurn)
        );
    }

    #[test]
    fn lowers_count_noted() {
        use sylex_semantics_v1::Count as SemValue;

        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::bind_named("X".into(), sylex_core::RefId(0));
            SemValue::Noted("X".into()).lower()
        });
        assert_eq!(lowered, sylex_core::Count::Reg(sylex_core::RefId(0)));
    }

    #[test]
    fn lowers_count_times_paid() {
        assert_matches!(
            sylex_semantics_v1::Count::TimesPaid(minimal_cost_tag()).lower(),
            sylex_core::Count::TimesPaid(sylex_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_count_damage() {
        assert_matches!(
            sylex_semantics_v1::Count::Damage(minimal_reference()).lower(),
            sylex_core::Count::Damage(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_count_mana_available() {
        assert_matches!(
            sylex_semantics_v1::Count::ManaAvailable(minimal_reference()).lower(),
            sylex_core::Count::ManaAvailable(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_count_mana_available_kind() {
        assert_matches!(
            sylex_semantics_v1::Count::ManaAvailableKind(
                minimal_reference(),
                sylex_semantics_v1::ColorOrColorless::Colorless,
            )
            .lower(),
            sylex_core::Count::ManaAvailableKind(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::ColorOrColorless::Colorless,
            )
        );
    }

    #[test]
    fn lowers_count_aggregate() {
        assert_matches!(
            sylex_semantics_v1::Count::Aggregate(minimal_aggregate_op(), minimal_projection())
                .lower(),
            sylex_core::Count::Aggregate(
                sylex_core::AggregateOp::SumOf,
                sylex_core::Projection {
                    of: sylex_core::Countable::Objects(_),
                    by: _
                }
            )
        );
    }

    #[test]
    fn lowers_count_expanded() {
        assert_matches!(
            sylex_semantics_v1::Count::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_count())
            })
            .lower(),
            sylex_core::Count::Literal(0)
        );
    }

    #[test]
    fn lowers_count_literal() {
        assert_matches!(
            sylex_semantics_v1::Count::Literal(0).lower(),
            sylex_core::Count::Literal(0)
        );
    }

    /// "That many" names the magnitude a count pinned earlier ([CR#608.2h]).
    /// Re-spelled from `lowers_count_that_many`: `Count::ThatMany` left core
    /// with the discourse channel, so the anaphor's image is the register read
    /// of the pinned amount.
    #[test]
    fn lowers_count_that_many_to_the_pinned_amount_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let amount = crate::region::define(sylex_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                sylex_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(sylex_semantics_v1::Sort::Amount),
                crate::region::Site::Product,
            );
            sylex_semantics_v1::Count::ThatMany.lower()
        });
        assert_eq!(lowered, sylex_core::Count::Reg(sylex_core::RefId(3)));
    }

    /// "That much" is the same register read as "that many" — one amount
    /// anaphor, one channel ([CR#608.2h]). Re-spelled from
    /// `lowers_count_that_much`.
    #[test]
    fn lowers_count_that_much_to_the_pinned_amount_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let amount = crate::region::define(sylex_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                sylex_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(sylex_semantics_v1::Sort::Amount),
                crate::region::Site::Product,
            );
            sylex_semantics_v1::Count::ThatMuch.lower()
        });
        assert_eq!(lowered, sylex_core::Count::Reg(sylex_core::RefId(3)));
    }

    /// A distributed share reads the loop body's allotment parameter
    /// ([CR#601.2d]) rather than a `Count::Allotment` variant. Re-spelled from
    /// `lowers_count_allotment`.
    #[test]
    fn lowers_count_allotment_to_the_loop_bodys_allotment_parameter() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::push_antecedent(
                sylex_core::RefId(1),
                sylex_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(sylex_semantics_v1::Sort::Amount),
                crate::region::Site::Allotment,
            );
            sylex_semantics_v1::Count::Allotment.lower()
        });
        assert_eq!(lowered, sylex_core::Count::Reg(sylex_core::RefId(1)));
    }

    /// The allotment channel is its own site: an ordinary pinned amount in
    /// scope is NOT what "that much" means inside a `Distribute` body
    /// ([CR#601.2d]), so an allotment read never falls back to it.
    #[test]
    #[should_panic(expected = "unbound distribution allotment")]
    fn allotment_does_not_fall_back_to_a_plain_amount_antecedent() {
        let _ = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let amount = crate::region::define(sylex_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                sylex_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(sylex_semantics_v1::Sort::Amount),
                crate::region::Site::Product,
            );
            sylex_semantics_v1::Count::Allotment.lower()
        });
    }
}

impl Lower for sylex_semantics_v1::RoundMode {
    type Target = sylex_core::RoundMode;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::RoundUp => sylex_core::RoundMode::RoundUp,
            Self::RoundDown => sylex_core::RoundMode::RoundDown,
        }
    }
}

impl Lower for sylex_semantics_v1::Characteristic {
    type Target = sylex_core::Characteristic;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Colors => sylex_core::Characteristic::Colors,
            Self::Types => sylex_core::Characteristic::Types,
            Self::Subtypes => sylex_core::Characteristic::Subtypes,
            Self::BasicLandTypes => sylex_core::Characteristic::BasicLandTypes,
            Self::Supertypes => sylex_core::Characteristic::Supertypes,
            Self::Power => sylex_core::Characteristic::Power,
            Self::Toughness => sylex_core::Characteristic::Toughness,
            Self::Defense => sylex_core::Characteristic::Defense,
            Self::ManaCost => sylex_core::Characteristic::ManaCost,
            Self::Name => sylex_core::Characteristic::Name,
        }
    }
}

impl Lower for sylex_semantics_v1::Countable {
    type Target = sylex_core::Countable;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Objects(f0) => sylex_core::Countable::Objects(std::sync::Arc::new(
                crate::region::predicate_region(|| std::sync::Arc::unwrap_or_clone(f0).lower()),
            )),
            Self::Players(f0) => sylex_core::Countable::Players(std::sync::Arc::new(
                crate::region::predicate_region(|| std::sync::Arc::unwrap_or_clone(f0).lower()),
            )),
            Self::ManaSymbols(f0, f1) => sylex_core::Countable::ManaSymbols(f0.lower(), f1.lower()),
            Self::Singleton(f0) => sylex_core::Countable::Singleton(f0.lower()),
            Self::ManaSpentMatching(f0, f1) => {
                sylex_core::Countable::ManaSpentMatching(f0.lower(), f1.lower())
            }
        }
    }
}

impl Lower for sylex_semantics_v1::AggregateOp {
    type Target = sylex_core::AggregateOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SumOf => sylex_core::AggregateOp::SumOf,
            Self::MinOf => sylex_core::AggregateOp::MinOf,
            Self::MaxOf => sylex_core::AggregateOp::MaxOf,
            Self::AverageOf(f0) => sylex_core::AggregateOp::AverageOf(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::Projection {
    type Target = sylex_core::Projection;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::Projection {
            of: self.of.lower(),
            by: std::sync::Arc::new(crate::region::candidate_region(|| {
                std::sync::Arc::unwrap_or_clone(self.by).lower()
            })),
        }
    }
}

impl Lower for sylex_semantics_v1::Count {
    type Target = sylex_core::Count;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::X => sylex_core::Count::Reg(
                crate::region::x().expect("X count outside a region with an announced-X parameter"),
            ),
            Self::CountOf(f0) => sylex_core::Count::CountOf(f0.lower()),
            Self::CountDistinct(f0, f1) => sylex_core::Count::CountDistinct(f0.lower(), f1.lower()),
            Self::StatOf(f0, f1) => sylex_core::Count::StatOf(f0.lower(), f1.lower()),
            Self::PlayerStatOf(f0, f1) => sylex_core::Count::PlayerStatOf(f0.lower(), f1.lower()),
            Self::Opponents(f0) => sylex_core::Count::Opponents(f0.lower()),
            Self::CounterCount(f0, f1) => sylex_core::Count::CounterCount(f0.lower(), f1.lower()),
            Self::Min(f0, f1) => sylex_core::Count::Min(f0.lower(), f1.lower()),
            Self::Max(f0, f1) => sylex_core::Count::Max(f0.lower(), f1.lower()),
            Self::Plus(f0, f1) => sylex_core::Count::Plus(f0.lower(), f1.lower()),
            Self::Minus(f0, f1) => sylex_core::Count::Minus(f0.lower(), f1.lower()),
            Self::Times(f0, f1) => sylex_core::Count::Times(f0.lower(), f1.lower()),
            Self::Half(f0, f1) => sylex_core::Count::Half(f0.lower(), f1.lower()),
            Self::Divide(f0, f1, f2) => {
                sylex_core::Count::Divide(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Mod(f0, f1) => sylex_core::Count::Mod(f0.lower(), f1.lower()),
            Self::Pow(f0, f1) => sylex_core::Count::Pow(f0.lower(), f1.lower()),
            Self::TargetsOf(f0) => sylex_core::Count::TargetsOf(f0.lower()),
            Self::ThatMany | Self::ThatMuch => sylex_core::Count::Reg(
                crate::region::amount().expect("unbound amount anaphor during lowering"),
            ),
            Self::Allotment => sylex_core::Count::Reg(
                crate::region::allotment().expect("unbound distribution allotment during lowering"),
            ),
            Self::EventCount(f0, f1) => sylex_core::Count::EventCount(f0.lower(), f1.lower()),
            Self::EventSum(f0, f1) => sylex_core::Count::EventSum(f0.lower(), f1.lower()),
            // A region-local definition wins; then the card's declared linked
            // memory parameter ([CR#607.1], core law 3). There is no
            // resolution-local name-keyed fallback.
            Self::Noted(name) => crate::region::named(&name)
                .or_else(|| crate::region::cell_read(&name))
                .map_or_else(
                    || {
                        crate::region::refuse(&format!(
                            "noted number `{name}` has no region definition or declared linked cell"
                        ));
                        sylex_core::Count::Reg(sylex_core::RefId(0))
                    },
                    sylex_core::Count::Reg,
                ),
            Self::TimesPaid(f0) => sylex_core::Count::TimesPaid(f0.lower()),
            Self::Damage(f0) => sylex_core::Count::Damage(f0.lower()),
            Self::ManaAvailable(f0) => sylex_core::Count::ManaAvailable(f0.lower()),
            Self::ManaAvailableKind(f0, f1) => {
                sylex_core::Count::ManaAvailableKind(f0.lower(), f1.lower())
            }
            Self::Aggregate(f0, f1) => sylex_core::Count::Aggregate(f0.lower(), f1.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
            Self::Literal(f0) => sylex_core::Count::Literal(f0.lower()),
        }
    }
}
