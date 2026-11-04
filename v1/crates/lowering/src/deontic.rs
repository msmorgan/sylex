//! `deontic` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::AlternativeCost {
    type Target = sylex_core::AlternativeCost;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Free => sylex_core::AlternativeCost::Free,
            Self::Components(f0) => sylex_core::AlternativeCost::Components(
                crate::cost::lower_scoped_cost_components(&f0),
            ),
        }
    }
}

impl Lower for sylex_semantics_v1::CostPredicate {
    type Target = sylex_core::CostPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IncludesTapSymbol => sylex_core::CostPredicate::IncludesTapSymbol,
        }
    }
}

impl Lower for sylex_semantics_v1::AsThough {
    type Target = sylex_core::AsThough;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Counterfactual { premise, then } => sylex_core::AsThough::Counterfactual {
                premise: premise.lower(),
                then: then.lower(),
            },
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::CountBound {
    type Target = sylex_core::CountBound;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Eq(f0) => sylex_core::CountBound::Eq(f0.lower()),
            Self::AtLeast(f0) => sylex_core::CountBound::AtLeast(f0.lower()),
            Self::AtMost(f0) => sylex_core::CountBound::AtMost(f0.lower()),
            Self::Greater(f0) => sylex_core::CountBound::Greater(f0.lower()),
            Self::Less(f0) => sylex_core::CountBound::Less(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::DeedAgent {
    type Target = sylex_core::DeedAgent;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::DeedAgent {
            stack_object: self.stack_object.lower(),
            source: self.source.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::DeonticAction {
    type Target = sylex_core::DeonticAction;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Attack { by, on } => sylex_core::DeonticAction::Attack {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Block { by, on, count } => sylex_core::DeonticAction::Block {
                by: by.lower(),
                on: on.lower(),
                count: count.lower(),
            },
            Self::Target { by, on } => sylex_core::DeonticAction::Target {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Attach { what, to } => sylex_core::DeonticAction::Attach {
                what: what.lower(),
                to: to.lower(),
            },
            Self::Cast {
                what,
                by,
                from,
                window,
                cost,
                tag,
            } => sylex_core::DeonticAction::Cast {
                what: what.lower(),
                by: by.lower(),
                from: from.lower(),
                window: window.lower(),
                cost: cost.lower(),
                tag: tag.lower(),
            },
            Self::Play { what, by, from } => sylex_core::DeonticAction::Play {
                what: what.lower(),
                by: by.lower(),
                from: from.lower(),
            },
            Self::Activate { what, by, cost } => sylex_core::DeonticAction::Activate {
                what: what.lower(),
                by: by.lower(),
                cost: cost.lower(),
            },
            Self::Regenerate { by, on } => sylex_core::DeonticAction::Regenerate {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Counter { by, on } => sylex_core::DeonticAction::Counter {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Untap { what } => sylex_core::DeonticAction::Untap { what: what.lower() },
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::Deontic {
    type Target = sylex_core::Deontic;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::May(f0) => sylex_core::Deontic::May(f0.lower()),
            Self::Cant(f0) => sylex_core::Deontic::Cant(f0.lower()),
            Self::Must(f0) => sylex_core::Deontic::Must(f0.lower()),
            Self::Gate(f0, f1) => sylex_core::Deontic::Gate(
                f0.lower(),
                crate::cost::lower_scoped_cost_components(&f1),
            ),
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
    fn lowers_alternative_cost_free() {
        assert_matches!(
            sylex_semantics_v1::AlternativeCost::Free.lower(),
            sylex_core::AlternativeCost::Free
        );
    }

    #[test]
    fn lowers_alternative_cost_components() {
        assert_matches!(
            sylex_semantics_v1::AlternativeCost::Components([].into()).lower(),
            sylex_core::AlternativeCost::Components(_)
        );
    }

    #[test]
    fn lowers_cost_predicate_includes_tap_symbol() {
        assert_matches!(
            sylex_semantics_v1::CostPredicate::IncludesTapSymbol.lower(),
            sylex_core::CostPredicate::IncludesTapSymbol
        );
    }

    #[test]
    fn lowers_as_though_counterfactual() {
        assert_matches!(
            sylex_semantics_v1::AsThough::Counterfactual {
                premise: minimal_predicate(),
                then: std::sync::Arc::new(minimal_deontic())
            }
            .lower(),
            sylex_core::AsThough::Counterfactual {
                premise: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                then: _
            }
        );
    }

    #[test]
    fn lowers_as_though_expanded() {
        assert_matches!(
            sylex_semantics_v1::AsThough::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_as_though())
            })
            .lower(),
            sylex_core::AsThough::Counterfactual {
                premise: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                then: _
            }
        );
    }

    #[test]
    fn lowers_count_bound_eq() {
        assert_matches!(
            sylex_semantics_v1::CountBound::Eq(minimal_count()).lower(),
            sylex_core::CountBound::Eq(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_at_least() {
        assert_matches!(
            sylex_semantics_v1::CountBound::AtLeast(minimal_count()).lower(),
            sylex_core::CountBound::AtLeast(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_at_most() {
        assert_matches!(
            sylex_semantics_v1::CountBound::AtMost(minimal_count()).lower(),
            sylex_core::CountBound::AtMost(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_greater() {
        assert_matches!(
            sylex_semantics_v1::CountBound::Greater(minimal_count()).lower(),
            sylex_core::CountBound::Greater(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_less() {
        assert_matches!(
            sylex_semantics_v1::CountBound::Less(minimal_count()).lower(),
            sylex_core::CountBound::Less(sylex_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_deed_agent() {
        assert_matches!(
            sylex_semantics_v1::DeedAgent {
                stack_object: None,
                source: None
            }
            .lower(),
            sylex_core::DeedAgent {
                stack_object: None,
                source: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_attack() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Attack {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            sylex_core::DeonticAction::Attack {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_block() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Block {
                by: minimal_predicate(),
                on: minimal_predicate(),
                count: None
            }
            .lower(),
            sylex_core::DeonticAction::Block {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                count: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_target() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Target {
                by: minimal_deed_agent(),
                on: minimal_predicate()
            }
            .lower(),
            sylex_core::DeonticAction::Target {
                by: sylex_core::DeedAgent {
                    stack_object: None,
                    source: None
                },
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_attach() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Attach {
                what: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            sylex_core::DeonticAction::Attach {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                to: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_cast() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Cast {
                what: minimal_predicate(),
                by: minimal_predicate(),
                from: None,
                window: None,
                cost: None,
                tag: None
            }
            .lower(),
            sylex_core::DeonticAction::Cast {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                from: None,
                window: None,
                cost: None,
                tag: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_play() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Play {
                what: minimal_predicate(),
                by: minimal_predicate(),
                from: None
            }
            .lower(),
            sylex_core::DeonticAction::Play {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                from: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_activate() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Activate {
                what: minimal_predicate(),
                by: minimal_predicate(),
                cost: None
            }
            .lower(),
            sylex_core::DeonticAction::Activate {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                cost: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_regenerate() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Regenerate {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            sylex_core::DeonticAction::Regenerate {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_counter() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Counter {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            sylex_core::DeonticAction::Counter {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_untap() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Untap {
                what: minimal_predicate()
            }
            .lower(),
            sylex_core::DeonticAction::Untap {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_expanded() {
        assert_matches!(
            sylex_semantics_v1::DeonticAction::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic_action())
            })
            .lower(),
            sylex_core::DeonticAction::Attack {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_may() {
        assert_matches!(
            sylex_semantics_v1::Deontic::May(minimal_deontic_action()).lower(),
            sylex_core::Deontic::May(sylex_core::DeonticAction::Attack {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            })
        );
    }

    #[test]
    fn lowers_deontic_cant() {
        assert_matches!(
            sylex_semantics_v1::Deontic::Cant(minimal_deontic_action()).lower(),
            sylex_core::Deontic::Cant(sylex_core::DeonticAction::Attack {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            })
        );
    }

    #[test]
    fn lowers_deontic_must() {
        assert_matches!(
            sylex_semantics_v1::Deontic::Must(minimal_deontic_action()).lower(),
            sylex_core::Deontic::Must(sylex_core::DeonticAction::Attack {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            })
        );
    }

    #[test]
    fn lowers_deontic_gate() {
        assert_matches!(
            sylex_semantics_v1::Deontic::Gate(minimal_deontic_action(), [].into()).lower(),
            sylex_core::Deontic::Gate(
                sylex_core::DeonticAction::Attack {
                    by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                    on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
                },
                _
            )
        );
    }

    #[test]
    fn lowers_deontic_expanded() {
        assert_matches!(
            sylex_semantics_v1::Deontic::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic())
            })
            .lower(),
            sylex_core::Deontic::May(sylex_core::DeonticAction::Attack {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            })
        );
    }
}
