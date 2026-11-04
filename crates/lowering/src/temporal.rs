//! `temporal` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Timing {
    type Target = sylex_core::Timing;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::InstantSpeed => sylex_core::Timing::InstantSpeed,
            Self::SorcerySpeed => sylex_core::Timing::SorcerySpeed,
            Self::DuringTurn(f0) => sylex_core::Timing::DuringTurn(f0.lower()),
            Self::DuringStep(f0, f1) => sylex_core::Timing::DuringStep(f0.lower(), f1.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::Lookback {
    type Target = sylex_core::Lookback;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ThisTurn => sylex_core::Lookback::ThisTurn,
            Self::ThisGame => sylex_core::Lookback::ThisGame,
            Self::LastTurn => sylex_core::Lookback::LastTurn,
            Self::ThisCombat => sylex_core::Lookback::ThisCombat,
            Self::ThisStep => sylex_core::Lookback::ThisStep,
            Self::SinceYour(f0) => sylex_core::Lookback::SinceYour(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::TurnMarker {
    type Target = sylex_core::TurnMarker;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::EndOfTurn => sylex_core::TurnMarker::EndOfTurn,
            Self::EndOfCombat => sylex_core::TurnMarker::EndOfCombat,
            Self::YourNextTurn => sylex_core::TurnMarker::YourNextTurn,
        }
    }
}

impl Lower for sylex_semantics_v1::LockPoint {
    type Target = sylex_core::LockPoint;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Announce => sylex_core::LockPoint::Announce,
            Self::StackPlacement => sylex_core::LockPoint::StackPlacement,
            Self::TotalCost => sylex_core::LockPoint::TotalCost,
            Self::Payment => sylex_core::LockPoint::Payment,
            Self::EffectBegin => sylex_core::LockPoint::EffectBegin,
            Self::CopyCreation => sylex_core::LockPoint::CopyCreation,
            Self::Declaration => sylex_core::LockPoint::Declaration,
            Self::PreGame => sylex_core::LockPoint::PreGame,
            Self::Resolution => sylex_core::LockPoint::Resolution,
            Self::Never => sylex_core::LockPoint::Never,
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
    fn lowers_timing_instant_speed() {
        assert_matches!(
            sylex_semantics_v1::Timing::InstantSpeed.lower(),
            sylex_core::Timing::InstantSpeed
        );
    }

    #[test]
    fn lowers_timing_sorcery_speed() {
        assert_matches!(
            sylex_semantics_v1::Timing::SorcerySpeed.lower(),
            sylex_core::Timing::SorcerySpeed
        );
    }

    #[test]
    fn lowers_timing_during_turn() {
        assert_matches!(
            sylex_semantics_v1::Timing::DuringTurn(minimal_whose_turn()).lower(),
            sylex_core::Timing::DuringTurn(sylex_core::WhoseTurn::Your)
        );
    }

    #[test]
    fn lowers_timing_during_step() {
        assert_matches!(
            sylex_semantics_v1::Timing::DuringStep(minimal_phase_step(), minimal_whose_turn())
                .lower(),
            sylex_core::Timing::DuringStep(
                sylex_core::PhaseStep::Beginning(sylex_core::BeginningStep::Untap),
                sylex_core::WhoseTurn::Your
            )
        );
    }

    #[test]
    fn lowers_lookback_this_turn() {
        assert_matches!(
            sylex_semantics_v1::Lookback::ThisTurn.lower(),
            sylex_core::Lookback::ThisTurn
        );
    }

    #[test]
    fn lowers_lookback_this_game() {
        assert_matches!(
            sylex_semantics_v1::Lookback::ThisGame.lower(),
            sylex_core::Lookback::ThisGame
        );
    }

    #[test]
    fn lowers_lookback_last_turn() {
        assert_matches!(
            sylex_semantics_v1::Lookback::LastTurn.lower(),
            sylex_core::Lookback::LastTurn
        );
    }

    #[test]
    fn lowers_lookback_this_combat() {
        assert_matches!(
            sylex_semantics_v1::Lookback::ThisCombat.lower(),
            sylex_core::Lookback::ThisCombat
        );
    }

    #[test]
    fn lowers_lookback_this_step() {
        assert_matches!(
            sylex_semantics_v1::Lookback::ThisStep.lower(),
            sylex_core::Lookback::ThisStep
        );
    }

    #[test]
    fn lowers_lookback_since_your() {
        assert_matches!(
            sylex_semantics_v1::Lookback::SinceYour(minimal_phase_step()).lower(),
            sylex_core::Lookback::SinceYour(sylex_core::PhaseStep::Beginning(
                sylex_core::BeginningStep::Untap
            ))
        );
    }

    #[test]
    fn lowers_turn_marker_end_of_turn() {
        assert_matches!(
            sylex_semantics_v1::TurnMarker::EndOfTurn.lower(),
            sylex_core::TurnMarker::EndOfTurn
        );
    }

    #[test]
    fn lowers_turn_marker_end_of_combat() {
        assert_matches!(
            sylex_semantics_v1::TurnMarker::EndOfCombat.lower(),
            sylex_core::TurnMarker::EndOfCombat
        );
    }

    #[test]
    fn lowers_turn_marker_your_next_turn() {
        assert_matches!(
            sylex_semantics_v1::TurnMarker::YourNextTurn.lower(),
            sylex_core::TurnMarker::YourNextTurn
        );
    }

    #[test]
    fn lowers_lock_point_announce() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::Announce.lower(),
            sylex_core::LockPoint::Announce
        );
    }

    #[test]
    fn lowers_lock_point_stack_placement() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::StackPlacement.lower(),
            sylex_core::LockPoint::StackPlacement
        );
    }

    #[test]
    fn lowers_lock_point_total_cost() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::TotalCost.lower(),
            sylex_core::LockPoint::TotalCost
        );
    }

    #[test]
    fn lowers_lock_point_payment() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::Payment.lower(),
            sylex_core::LockPoint::Payment
        );
    }

    #[test]
    fn lowers_lock_point_effect_begin() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::EffectBegin.lower(),
            sylex_core::LockPoint::EffectBegin
        );
    }

    #[test]
    fn lowers_lock_point_copy_creation() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::CopyCreation.lower(),
            sylex_core::LockPoint::CopyCreation
        );
    }

    #[test]
    fn lowers_lock_point_declaration() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::Declaration.lower(),
            sylex_core::LockPoint::Declaration
        );
    }

    #[test]
    fn lowers_lock_point_pre_game() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::PreGame.lower(),
            sylex_core::LockPoint::PreGame
        );
    }

    #[test]
    fn lowers_lock_point_resolution() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::Resolution.lower(),
            sylex_core::LockPoint::Resolution
        );
    }

    #[test]
    fn lowers_lock_point_never() {
        assert_matches!(
            sylex_semantics_v1::LockPoint::Never.lower(),
            sylex_core::LockPoint::Never
        );
    }
}
