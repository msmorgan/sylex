//! `decision` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::DeciderSpec {
    type Target = sylex_core::DeciderSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Controller => sylex_core::DeciderSpec::Controller,
            Self::ActivePlayer => sylex_core::DeciderSpec::ActivePlayer,
            Self::DefendingPlayer => sylex_core::DeciderSpec::DefendingPlayer,
            Self::Named(f0) => sylex_core::DeciderSpec::Named(f0.lower()),
            Self::EachInTurnOrder => sylex_core::DeciderSpec::EachInTurnOrder,
            Self::PriorityHolder => sylex_core::DeciderSpec::PriorityHolder,
            Self::Rng => sylex_core::DeciderSpec::Rng,
        }
    }
}

impl Lower for sylex_semantics_v1::Visibility {
    type Target = sylex_core::Visibility;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Open => sylex_core::Visibility::Open,
            Self::CommittedHidden => sylex_core::Visibility::CommittedHidden,
        }
    }
}

impl Lower for sylex_semantics_v1::ChosenValueKind {
    type Target = sylex_core::ChosenValueKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Color => sylex_core::ChosenValueKind::Color,
            Self::CardName => sylex_core::ChosenValueKind::CardName,
            Self::Number => sylex_core::ChosenValueKind::Number,
        }
    }
}

impl Lower for sylex_semantics_v1::NotedKind {
    type Target = sylex_core::NotedKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Objects => sylex_core::NotedKind::Objects,
            Self::Piles => sylex_core::NotedKind::Piles,
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
    fn lowers_decider_spec_controller() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::Controller.lower(),
            sylex_core::DeciderSpec::Controller
        );
    }

    #[test]
    fn lowers_decider_spec_active_player() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::ActivePlayer.lower(),
            sylex_core::DeciderSpec::ActivePlayer
        );
    }

    #[test]
    fn lowers_decider_spec_defending_player() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::DefendingPlayer.lower(),
            sylex_core::DeciderSpec::DefendingPlayer
        );
    }

    #[test]
    fn lowers_decider_spec_named() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::Named(minimal_reference()).lower(),
            sylex_core::DeciderSpec::Named(sylex_core::Reference::Reg(sylex_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_decider_spec_each_in_turn_order() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::EachInTurnOrder.lower(),
            sylex_core::DeciderSpec::EachInTurnOrder
        );
    }

    #[test]
    fn lowers_decider_spec_priority_holder() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::PriorityHolder.lower(),
            sylex_core::DeciderSpec::PriorityHolder
        );
    }

    #[test]
    fn lowers_decider_spec_rng() {
        assert_matches!(
            sylex_semantics_v1::DeciderSpec::Rng.lower(),
            sylex_core::DeciderSpec::Rng
        );
    }

    #[test]
    fn lowers_visibility_open() {
        assert_matches!(
            sylex_semantics_v1::Visibility::Open.lower(),
            sylex_core::Visibility::Open
        );
    }

    #[test]
    fn lowers_visibility_committed_hidden() {
        assert_matches!(
            sylex_semantics_v1::Visibility::CommittedHidden.lower(),
            sylex_core::Visibility::CommittedHidden
        );
    }

    #[test]
    fn lowers_chosen_value_kind_color() {
        assert_matches!(
            sylex_semantics_v1::ChosenValueKind::Color.lower(),
            sylex_core::ChosenValueKind::Color
        );
    }

    #[test]
    fn lowers_chosen_value_kind_card_name() {
        assert_matches!(
            sylex_semantics_v1::ChosenValueKind::CardName.lower(),
            sylex_core::ChosenValueKind::CardName
        );
    }

    #[test]
    fn lowers_chosen_value_kind_number() {
        assert_matches!(
            sylex_semantics_v1::ChosenValueKind::Number.lower(),
            sylex_core::ChosenValueKind::Number
        );
    }

    #[test]
    fn lowers_noted_kind_objects() {
        assert_matches!(
            sylex_semantics_v1::NotedKind::Objects.lower(),
            sylex_core::NotedKind::Objects
        );
    }

    #[test]
    fn lowers_noted_kind_piles() {
        assert_matches!(
            sylex_semantics_v1::NotedKind::Piles.lower(),
            sylex_core::NotedKind::Piles
        );
    }
}
