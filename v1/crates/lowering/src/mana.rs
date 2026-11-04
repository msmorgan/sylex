//! `mana` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::PlanarFace {
    type Target = sylex_core::PlanarFace;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Blank => sylex_core::PlanarFace::Blank,
            Self::Chaos => sylex_core::PlanarFace::Chaos,
            Self::Planeswalker => sylex_core::PlanarFace::Planeswalker,
        }
    }
}

impl Lower for sylex_semantics_v1::ManaSpec {
    type Target = sylex_core::ManaSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AnyColor => sylex_core::ManaSpec::AnyColor,
            Self::OneOf(f0) => sylex_core::ManaSpec::OneOf(f0.lower()),
            Self::OneOfRuns(f0) => sylex_core::ManaSpec::OneOfRuns(f0.lower()),
            Self::AmongColorsOf(f0) => sylex_core::ManaSpec::AmongColorsOf(f0.lower()),
            Self::ProducedByEvent => sylex_core::ManaSpec::ProducedByEvent,
            Self::Specific(f0) => sylex_core::ManaSpec::Specific(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::SimpleManaSymbol {
    type Target = sylex_core::SimpleManaSymbol;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic(f0) => sylex_core::SimpleManaSymbol::Generic(f0.lower()),
            Self::Specific(f0) => sylex_core::SimpleManaSymbol::Specific(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::ManaSymbol {
    type Target = sylex_core::ManaSymbol;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Variable => sylex_core::ManaSymbol::Variable,
            Self::Snow => sylex_core::ManaSymbol::Snow,
            Self::Hybrid(f0, f1) => sylex_core::ManaSymbol::Hybrid(f0.lower(), f1.lower()),
            Self::Phyrexian(f0, f1) => sylex_core::ManaSymbol::Phyrexian(f0.lower(), f1.lower()),
            Self::Simple(f0) => sylex_core::ManaSymbol::Simple(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::SymbolPred {
    type Target = sylex_core::SymbolPred;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AnyColor => sylex_core::SymbolPred::AnyColor,
            Self::AnyType => sylex_core::SymbolPred::AnyType,
            Self::CountsAs(f0) => sylex_core::SymbolPred::CountsAs(f0.lower()),
            Self::IsGeneric => sylex_core::SymbolPred::IsGeneric,
            Self::And(f0) => sylex_core::SymbolPred::And(f0.lower()),
            Self::Or(f0) => sylex_core::SymbolPred::Or(f0.lower()),
            Self::Not(f0) => sylex_core::SymbolPred::Not(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::ManaRider {
    type Target = sylex_core::ManaRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SpendOnly(f0) => sylex_core::ManaRider::SpendOnly(f0.lower()),
            Self::GrantOnSpend(f0) => sylex_core::ManaRider::GrantOnSpend(f0.lower()),
            Self::TriggerOnSpend(f0) => sylex_core::ManaRider::TriggerOnSpend(f0.lower()),
            Self::Persistent(f0) => sylex_core::ManaRider::Persistent(f0.lower()),
            Self::Snow => sylex_core::ManaRider::Snow,
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::ManaProduction {
    type Target = sylex_core::ManaProduction;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::WithRiders { mana, riders } => sylex_core::ManaProduction::WithRiders {
                mana: mana.lower(),
                riders: riders.lower(),
            },
            Self::Bare(f0) => sylex_core::ManaProduction::Bare(f0.lower()),
        }
    }
}

/// HAND-WRITTEN, not scaffolded.
///
/// `ManaCost` is a newtype over a PRIVATE `Arc<[ManaSymbol]>` (semantics'
/// `mana.rs`), so `self.0` is not reachable from this crate and the scaffold
/// generator correctly refused to emit an arm for it. This is still an identity
/// mapping — it just travels through the public `From` conversions on either
/// side of the newtype rather than through the field.
///
/// `ParseManaError` gets no arm at all: it is the `FromStr` error type, not
/// grammar, and is unreachable from any container.
impl Lower for sylex_semantics_v1::ManaCost {
    type Target = sylex_core::ManaCost;
    fn lower(self) -> <Self as Lower>::Target {
        let symbols: std::sync::Arc<[sylex_semantics_v1::ManaSymbol]> = self.into();
        symbols.lower().into()
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
    fn lowers_planar_face_blank() {
        assert_matches!(
            sylex_semantics_v1::PlanarFace::Blank.lower(),
            sylex_core::PlanarFace::Blank
        );
    }

    #[test]
    fn lowers_planar_face_chaos() {
        assert_matches!(
            sylex_semantics_v1::PlanarFace::Chaos.lower(),
            sylex_core::PlanarFace::Chaos
        );
    }

    #[test]
    fn lowers_planar_face_planeswalker() {
        assert_matches!(
            sylex_semantics_v1::PlanarFace::Planeswalker.lower(),
            sylex_core::PlanarFace::Planeswalker
        );
    }

    #[test]
    fn lowers_mana_spec_any_color() {
        assert_matches!(
            sylex_semantics_v1::ManaSpec::AnyColor.lower(),
            sylex_core::ManaSpec::AnyColor
        );
    }

    #[test]
    fn lowers_mana_spec_one_of() {
        assert_matches!(
            sylex_semantics_v1::ManaSpec::OneOf([].into()).lower(),
            sylex_core::ManaSpec::OneOf(_)
        );
    }

    #[test]
    fn lowers_mana_spec_one_of_runs() {
        assert_matches!(
            sylex_semantics_v1::ManaSpec::OneOfRuns([].into()).lower(),
            sylex_core::ManaSpec::OneOfRuns(_)
        );
    }

    #[test]
    fn lowers_mana_spec_among_colors_of() {
        assert_matches!(
            sylex_semantics_v1::ManaSpec::AmongColorsOf(minimal_reference()).lower(),
            sylex_core::ManaSpec::AmongColorsOf(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_mana_spec_produced_by_event() {
        assert_matches!(
            sylex_semantics_v1::ManaSpec::ProducedByEvent.lower(),
            sylex_core::ManaSpec::ProducedByEvent
        );
    }

    #[test]
    fn lowers_mana_spec_specific() {
        assert_matches!(
            sylex_semantics_v1::ManaSpec::Specific(minimal_color_or_colorless()).lower(),
            sylex_core::ManaSpec::Specific(sylex_core::ColorOrColorless::Colorless)
        );
    }

    #[test]
    fn lowers_simple_mana_symbol_generic() {
        assert_matches!(
            sylex_semantics_v1::SimpleManaSymbol::Generic(0).lower(),
            sylex_core::SimpleManaSymbol::Generic(0)
        );
    }

    #[test]
    fn lowers_simple_mana_symbol_specific() {
        assert_matches!(
            sylex_semantics_v1::SimpleManaSymbol::Specific(minimal_color_or_colorless()).lower(),
            sylex_core::SimpleManaSymbol::Specific(sylex_core::ColorOrColorless::Colorless)
        );
    }

    #[test]
    fn lowers_mana_symbol_variable() {
        assert_matches!(
            sylex_semantics_v1::ManaSymbol::Variable.lower(),
            sylex_core::ManaSymbol::Variable
        );
    }

    #[test]
    fn lowers_mana_symbol_snow() {
        assert_matches!(
            sylex_semantics_v1::ManaSymbol::Snow.lower(),
            sylex_core::ManaSymbol::Snow
        );
    }

    #[test]
    fn lowers_mana_symbol_hybrid() {
        assert_matches!(
            sylex_semantics_v1::ManaSymbol::Hybrid(minimal_simple_mana_symbol(), minimal_color())
                .lower(),
            sylex_core::ManaSymbol::Hybrid(
                sylex_core::SimpleManaSymbol::Generic(0),
                sylex_core::Color::White
            )
        );
    }

    #[test]
    fn lowers_mana_symbol_phyrexian() {
        assert_matches!(
            sylex_semantics_v1::ManaSymbol::Phyrexian(minimal_color(), None).lower(),
            sylex_core::ManaSymbol::Phyrexian(sylex_core::Color::White, None)
        );
    }

    #[test]
    fn lowers_mana_symbol_simple() {
        assert_matches!(
            sylex_semantics_v1::ManaSymbol::Simple(minimal_simple_mana_symbol()).lower(),
            sylex_core::ManaSymbol::Simple(sylex_core::SimpleManaSymbol::Generic(0))
        );
    }

    #[test]
    fn lowers_symbol_pred_any_color() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::AnyColor.lower(),
            sylex_core::SymbolPred::AnyColor
        );
    }

    #[test]
    fn lowers_symbol_pred_any_type() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::AnyType.lower(),
            sylex_core::SymbolPred::AnyType
        );
    }

    #[test]
    fn lowers_symbol_pred_counts_as() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::CountsAs(minimal_color()).lower(),
            sylex_core::SymbolPred::CountsAs(sylex_core::Color::White)
        );
    }

    #[test]
    fn lowers_symbol_pred_is_generic() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::IsGeneric.lower(),
            sylex_core::SymbolPred::IsGeneric
        );
    }

    #[test]
    fn lowers_symbol_pred_and() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::And([].into()).lower(),
            sylex_core::SymbolPred::And(_)
        );
    }

    #[test]
    fn lowers_symbol_pred_or() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::Or([].into()).lower(),
            sylex_core::SymbolPred::Or(_)
        );
    }

    #[test]
    fn lowers_symbol_pred_not() {
        assert_matches!(
            sylex_semantics_v1::SymbolPred::Not(std::sync::Arc::new(minimal_symbol_pred())).lower(),
            sylex_core::SymbolPred::Not(_)
        );
    }

    #[test]
    fn lowers_mana_rider_spend_only() {
        assert_matches!(
            sylex_semantics_v1::ManaRider::SpendOnly(minimal_predicate()).lower(),
            sylex_core::ManaRider::SpendOnly(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_mana_rider_persistent() {
        assert_matches!(
            sylex_semantics_v1::ManaRider::Persistent(minimal_turn_marker()).lower(),
            sylex_core::ManaRider::Persistent(sylex_core::TurnMarker::EndOfTurn)
        );
    }

    #[test]
    fn lowers_mana_rider_snow() {
        assert_matches!(
            sylex_semantics_v1::ManaRider::Snow.lower(),
            sylex_core::ManaRider::Snow
        );
    }

    #[test]
    fn lowers_mana_rider_expanded() {
        assert_matches!(
            sylex_semantics_v1::ManaRider::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_mana_rider())
            })
            .lower(),
            sylex_core::ManaRider::SpendOnly(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_mana_production_with_riders() {
        assert_matches!(
            sylex_semantics_v1::ManaProduction::WithRiders {
                mana: minimal_mana_spec(),
                riders: [].into()
            }
            .lower(),
            sylex_core::ManaProduction::WithRiders {
                mana: sylex_core::ManaSpec::AnyColor,
                riders: _
            }
        );
    }

    #[test]
    fn lowers_mana_production_bare() {
        assert_matches!(
            sylex_semantics_v1::ManaProduction::Bare(minimal_mana_spec()).lower(),
            sylex_core::ManaProduction::Bare(sylex_core::ManaSpec::AnyColor)
        );
    }

    #[test]
    fn lowers_mana_cost() {
        assert_lowers(sylex_semantics_v1::ManaCost::from(std::sync::Arc::<
            [sylex_semantics_v1::ManaSymbol],
        >::from([])));
    }

    #[test]
    fn lowers_mana_rider_grant_on_spend() {
        assert_matches!(
            in_spell_region(
                || sylex_semantics_v1::ManaRider::GrantOnSpend(std::sync::Arc::new(
                    minimal_one_shot_effect()
                ))
                .lower()
            ),
            sylex_core::ManaRider::GrantOnSpend(_)
        );
    }

    #[test]
    fn lowers_mana_rider_trigger_on_spend() {
        assert_matches!(
            in_spell_region(
                || sylex_semantics_v1::ManaRider::TriggerOnSpend(std::sync::Arc::new(
                    minimal_one_shot_effect()
                ))
                .lower()
            ),
            sylex_core::ManaRider::TriggerOnSpend(_)
        );
    }
}
