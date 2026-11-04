//! `condition` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Cmp {
    type Target = sylex_core::Cmp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Eq => sylex_core::Cmp::Eq,
            Self::AtLeast => sylex_core::Cmp::AtLeast,
            Self::AtMost => sylex_core::Cmp::AtMost,
            Self::Greater => sylex_core::Cmp::Greater,
            Self::Less => sylex_core::Cmp::Less,
        }
    }
}

impl Lower for sylex_semantics_v1::Condition {
    type Target = sylex_core::Condition;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Compare(f0, f1, f2) => {
                sylex_core::Condition::Compare(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Exists(f0) => sylex_core::Condition::Exists(f0.lower()),
            // [CR#120.1,120.3,702.2c]: `Matches(Source, F)` is not a read of
            // one referent — it asks whether the object under evaluation was
            // dealt damage by a source matching F, existentially over its
            // marks' deal-time abilities. Core names that relation, so the
            // whole condition lowers as a unit onto the same `DealtDamageBy`
            // the explicit spelling below produces.
            Self::Matches(sylex_semantics_v1::Reference::Source, filter) => {
                sylex_core::Condition::DealtDamageBy(
                    sylex_core::Reference::Reg(
                        crate::region::source().unwrap_or(sylex_core::RefId(0)),
                    ),
                    filter.lower(),
                )
            }
            Self::Matches(reference, filter) => {
                sylex_core::Condition::Matches(reference.lower(), filter.lower())
            }
            Self::DealtDamageBy(reference, filter) => {
                sylex_core::Condition::DealtDamageBy(reference.lower(), filter.lower())
            }
            Self::LegallyAttached(f0) => sylex_core::Condition::LegallyAttached(f0.lower()),
            Self::Happened { event, within } => sylex_core::Condition::Happened {
                event: event.lower(),
                within: within.lower(),
            },
            Self::Crossed { value, thresholds } => sylex_core::Condition::Crossed {
                value: value.lower(),
                thresholds: thresholds.lower(),
            },
            Self::PaidCost(f0) => sylex_core::Condition::PaidCost(f0.lower()),
            Self::CastWith(f0) => sylex_core::Condition::CastWith(f0.lower()),
            Self::YourTurn => sylex_core::Condition::YourTurn,
            Self::TurnOf(f0) => sylex_core::Condition::TurnOf(f0.lower()),
            Self::DuringPhase(f0) => sylex_core::Condition::DuringPhase(f0.lower()),
            Self::And(f0) => sylex_core::Condition::And(f0.lower()),
            Self::Or(f0) => sylex_core::Condition::Or(f0.lower()),
            Self::Not(f0) => sylex_core::Condition::Not(f0.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
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
    fn lowers_cmp_eq() {
        assert_matches!(sylex_semantics_v1::Cmp::Eq.lower(), sylex_core::Cmp::Eq);
    }

    #[test]
    fn lowers_cmp_at_least() {
        assert_matches!(
            sylex_semantics_v1::Cmp::AtLeast.lower(),
            sylex_core::Cmp::AtLeast
        );
    }

    #[test]
    fn lowers_cmp_at_most() {
        assert_matches!(
            sylex_semantics_v1::Cmp::AtMost.lower(),
            sylex_core::Cmp::AtMost
        );
    }

    #[test]
    fn lowers_cmp_greater() {
        assert_matches!(
            sylex_semantics_v1::Cmp::Greater.lower(),
            sylex_core::Cmp::Greater
        );
    }

    #[test]
    fn lowers_cmp_less() {
        assert_matches!(sylex_semantics_v1::Cmp::Less.lower(), sylex_core::Cmp::Less);
    }

    #[test]
    fn lowers_condition_compare() {
        assert_matches!(
            sylex_semantics_v1::Condition::Compare(minimal_count(), minimal_cmp(), minimal_count())
                .lower(),
            sylex_core::Condition::Compare(
                sylex_core::Count::Literal(0),
                sylex_core::Cmp::Eq,
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_condition_exists() {
        assert_matches!(
            sylex_semantics_v1::Condition::Exists(minimal_predicate()).lower(),
            sylex_core::Condition::Exists(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_condition_matches() {
        assert_matches!(
            sylex_semantics_v1::Condition::Matches(minimal_reference(), minimal_predicate())
                .lower(),
            sylex_core::Condition::Matches(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            )
        );
    }

    #[test]
    fn lowers_condition_dealt_damage_by() {
        assert_eq!(
            sylex_semantics_v1::Condition::DealtDamageBy(
                sylex_semantics_v1::Reference::This,
                minimal_predicate(),
            )
            .lower(),
            sylex_core::Condition::DealtDamageBy(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
            )
        );
    }

    #[test]
    fn lowers_legacy_matches_source_as_dealt_damage_by() {
        assert_eq!(
            sylex_semantics_v1::Condition::Matches(
                sylex_semantics_v1::Reference::Source,
                minimal_predicate(),
            )
            .lower(),
            sylex_core::Condition::DealtDamageBy(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
            )
        );
    }

    #[test]
    fn lowers_condition_legally_attached() {
        assert_matches!(
            sylex_semantics_v1::Condition::LegallyAttached(minimal_reference()).lower(),
            sylex_core::Condition::LegallyAttached(sylex_core::Reference::Reg(sylex_core::RefId(
                0
            )))
        );
    }

    #[test]
    fn lowers_condition_happened() {
        assert_matches!(
            sylex_semantics_v1::Condition::Happened {
                event: std::sync::Arc::new(minimal_event_filter()),
                within: minimal_lookback()
            }
            .lower(),
            sylex_core::Condition::Happened {
                event: _,
                within: sylex_core::Lookback::ThisTurn
            }
        );
    }

    #[test]
    fn lowers_condition_crossed() {
        assert_matches!(
            sylex_semantics_v1::Condition::Crossed {
                value: minimal_count(),
                thresholds: [].into()
            }
            .lower(),
            sylex_core::Condition::Crossed {
                value: sylex_core::Count::Literal(0),
                thresholds: _
            }
        );
    }

    #[test]
    fn lowers_condition_paid_cost() {
        assert_matches!(
            sylex_semantics_v1::Condition::PaidCost(minimal_cost_tag()).lower(),
            sylex_core::Condition::PaidCost(sylex_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_condition_cast_with() {
        assert_matches!(
            sylex_semantics_v1::Condition::CastWith(minimal_cost_tag()).lower(),
            sylex_core::Condition::CastWith(sylex_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_condition_your_turn() {
        assert_matches!(
            sylex_semantics_v1::Condition::YourTurn.lower(),
            sylex_core::Condition::YourTurn
        );
    }

    #[test]
    fn lowers_condition_turn_of() {
        assert_matches!(
            sylex_semantics_v1::Condition::TurnOf(minimal_predicate()).lower(),
            sylex_core::Condition::TurnOf(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_condition_during_phase() {
        assert_matches!(
            sylex_semantics_v1::Condition::DuringPhase(minimal_phase_step()).lower(),
            sylex_core::Condition::DuringPhase(sylex_core::PhaseStep::Beginning(
                sylex_core::BeginningStep::Untap
            ))
        );
    }

    #[test]
    fn lowers_condition_and() {
        assert_matches!(
            sylex_semantics_v1::Condition::And([].into()).lower(),
            sylex_core::Condition::And(_)
        );
    }

    #[test]
    fn lowers_condition_or() {
        assert_matches!(
            sylex_semantics_v1::Condition::Or([].into()).lower(),
            sylex_core::Condition::Or(_)
        );
    }

    #[test]
    fn lowers_condition_not() {
        assert_matches!(
            sylex_semantics_v1::Condition::Not(std::sync::Arc::new(minimal_condition())).lower(),
            sylex_core::Condition::Not(_)
        );
    }

    #[test]
    fn lowers_condition_expanded() {
        assert_matches!(
            sylex_semantics_v1::Condition::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_condition())
            })
            .lower(),
            sylex_core::Condition::Compare(
                sylex_core::Count::Literal(0),
                sylex_core::Cmp::Eq,
                sylex_core::Count::Literal(0)
            )
        );
    }
}
