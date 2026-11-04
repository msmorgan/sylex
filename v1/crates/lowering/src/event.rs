//! `event` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::PhaseKind {
    type Target = sylex_core::PhaseKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Beginning => sylex_core::PhaseKind::Beginning,
            Self::PrecombatMain => sylex_core::PhaseKind::PrecombatMain,
            Self::Combat => sylex_core::PhaseKind::Combat,
            Self::PostcombatMain => sylex_core::PhaseKind::PostcombatMain,
            Self::Ending => sylex_core::PhaseKind::Ending,
        }
    }
}

impl Lower for sylex_semantics_v1::PhaseStep {
    type Target = sylex_core::PhaseStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Beginning(f0) => sylex_core::PhaseStep::Beginning(f0.lower()),
            Self::PrecombatMain => sylex_core::PhaseStep::PrecombatMain,
            Self::Combat(f0) => sylex_core::PhaseStep::Combat(f0.lower()),
            Self::PostcombatMain => sylex_core::PhaseStep::PostcombatMain,
            Self::Ending(f0) => sylex_core::PhaseStep::Ending(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::BeginningStep {
    type Target = sylex_core::BeginningStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Untap => sylex_core::BeginningStep::Untap,
            Self::Upkeep => sylex_core::BeginningStep::Upkeep,
            Self::Draw => sylex_core::BeginningStep::Draw,
        }
    }
}

impl Lower for sylex_semantics_v1::CombatStep {
    type Target = sylex_core::CombatStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::BeginningOfCombat => sylex_core::CombatStep::BeginningOfCombat,
            Self::DeclareAttackers => sylex_core::CombatStep::DeclareAttackers,
            Self::DeclareBlockers => sylex_core::CombatStep::DeclareBlockers,
            Self::FirstCombatDamage => sylex_core::CombatStep::FirstCombatDamage,
            Self::CombatDamage => sylex_core::CombatStep::CombatDamage,
            Self::EndOfCombat => sylex_core::CombatStep::EndOfCombat,
        }
    }
}

impl Lower for sylex_semantics_v1::EndingStep {
    type Target = sylex_core::EndingStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::End => sylex_core::EndingStep::End,
            Self::Cleanup => sylex_core::EndingStep::Cleanup,
        }
    }
}

impl Lower for sylex_semantics_v1::WhoseTurn {
    type Target = sylex_core::WhoseTurn;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Your => sylex_core::WhoseTurn::Your,
            Self::EachPlayers => sylex_core::WhoseTurn::EachPlayers,
            Self::AnOpponents => sylex_core::WhoseTurn::AnOpponents,
        }
    }
}

impl Lower for sylex_semantics_v1::StateChange {
    type Target = sylex_core::StateChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => sylex_core::StateChange::Tapped,
            Self::Untapped => sylex_core::StateChange::Untapped,
            Self::Phased(f0) => sylex_core::StateChange::Phased(f0.lower()),
            Self::TurnedFace(f0) => sylex_core::StateChange::TurnedFace(f0.lower()),
            Self::Transformed => sylex_core::StateChange::Transformed,
        }
    }
}

impl Lower for sylex_semantics_v1::Agency {
    type Target = sylex_core::Agency;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CostPayment => sylex_core::Agency::CostPayment,
            Self::AttackDeclaration => sylex_core::Agency::AttackDeclaration,
            Self::EffectInstruction => sylex_core::Agency::EffectInstruction,
            Self::TurnBasedAction => sylex_core::Agency::TurnBasedAction,
            Self::StateBasedAction => sylex_core::Agency::StateBasedAction,
            Self::ManaAbilityResolution => sylex_core::Agency::ManaAbilityResolution,
            Self::SpecialAction => sylex_core::Agency::SpecialAction,
        }
    }
}

impl Lower for sylex_semantics_v1::VerbName {
    type Target = sylex_core::VerbName;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::VerbName(self.0.lower())
    }
}

impl Lower for sylex_semantics_v1::CausePattern {
    type Target = sylex_core::CausePattern;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::CausePattern {
            verb: self.verb.lower(),
            agency: self.agency.lower(),
            agent: self.agent.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::Cause {
    type Target = sylex_core::Cause;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Cause(f0) => sylex_core::Cause::Cause(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::EventFilter {
    type Target = sylex_core::EventFilter;
    #[allow(
        clippy::too_many_lines,
        reason = "exhaustive one-arm-per-variant map; splitting it would hide the exhaustiveness check"
    )]
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ZoneChange {
                what,
                from,
                to,
                cause,
            } => sylex_core::EventFilter::ZoneChange {
                what: what.lower(),
                from: from.lower(),
                to: to.lower(),
                cause: cause.lower(),
            },
            Self::Damage {
                source,
                to,
                combat,
                amount,
            } => sylex_core::EventFilter::Damage {
                source: source.lower(),
                to: to.lower(),
                combat: combat.lower(),
                amount: amount.lower(),
            },
            Self::LifeGained { who, amount } => sylex_core::EventFilter::LifeGained {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::LifeLost { who, amount } => sylex_core::EventFilter::LifeLost {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::Drawn { who, amount } => sylex_core::EventFilter::Drawn {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::Act {
                verb,
                who,
                on,
                cause,
            } => sylex_core::EventFilter::Act {
                verb: verb.lower(),
                who: who.lower(),
                on: on.lower(),
                cause: cause.lower(),
            },
            Self::CounterPlaced {
                kind,
                on,
                amount,
                cause,
            } => sylex_core::EventFilter::CounterPlaced {
                kind: kind.lower(),
                on: on.lower(),
                amount: amount.lower(),
                cause: cause.lower(),
            },
            Self::CounterRemoved {
                kind,
                on,
                amount,
                cause,
            } => sylex_core::EventFilter::CounterRemoved {
                kind: kind.lower(),
                on: on.lower(),
                amount: amount.lower(),
                cause: cause.lower(),
            },
            Self::Cast { who, what } => sylex_core::EventFilter::Cast {
                who: who.lower(),
                what: what.lower(),
            },
            Self::Copied { who, what } => sylex_core::EventFilter::Copied {
                who: who.lower(),
                what: what.lower(),
            },
            Self::Played { who, what } => sylex_core::EventFilter::Played {
                who: who.lower(),
                what: what.lower(),
            },
            Self::ActivatedAb { who, what } => sylex_core::EventFilter::ActivatedAb {
                who: who.lower(),
                what: what.lower(),
            },
            Self::ManaAbilityActivated { what, by } => {
                sylex_core::EventFilter::ManaAbilityActivated {
                    what: what.lower(),
                    by: by.lower(),
                }
            }
            Self::ManaProduced { what, by } => sylex_core::EventFilter::ManaProduced {
                what: what.lower(),
                by: by.lower(),
            },
            Self::ManaAdded { what, by } => sylex_core::EventFilter::ManaAdded {
                what: what.lower(),
                by: by.lower(),
            },
            Self::AttackDeclared { by, against } => sylex_core::EventFilter::AttackDeclared {
                by: by.lower(),
                against: against.lower(),
            },
            Self::BlockDeclared { by, of } => sylex_core::EventFilter::BlockDeclared {
                by: by.lower(),
                of: of.lower(),
            },
            Self::Attached { what, to } => sylex_core::EventFilter::Attached {
                what: what.lower(),
                to: to.lower(),
            },
            Self::StateBecame { of, becomes, cause } => sylex_core::EventFilter::StateBecame {
                of: of.lower(),
                becomes: becomes.lower(),
                cause: cause.lower(),
            },
            Self::BecomesTarget { what, by, source } => {
                sylex_core::EventFilter::BecomesTarget {
                    what: what.lower(),
                    by: by.lower(),
                    source: source.lower(),
                }
            }
            Self::StepBegins { at, whose } => sylex_core::EventFilter::StepBegins {
                at: at.lower(),
                whose: whose.lower(),
            },
            Self::ControlChanged { of, to } => sylex_core::EventFilter::ControlChanged {
                of: of.lower(),
                to: to.lower(),
            },
            Self::DesignationChanged { name, of, to } => {
                sylex_core::EventFilter::DesignationChanged {
                    name: name.lower(),
                    of: of.lower(),
                    to: to.lower(),
                }
            }
            Self::TokenCreated { what, by } => sylex_core::EventFilter::TokenCreated {
                what: what.lower(),
                by: by.lower(),
            },
            Self::Shuffled { by } => sylex_core::EventFilter::Shuffled { by: by.lower() },
            Self::Revealed { what } => sylex_core::EventFilter::Revealed { what: what.lower() },
            Self::Used { of } => sylex_core::EventFilter::Used { of: of.lower() },
            Self::CoinFlipped { by, won } => sylex_core::EventFilter::CoinFlipped {
                by: by.lower(),
                won: won.lower(),
            },
            Self::DiceRolled { by } => sylex_core::EventFilter::DiceRolled { by: by.lower() },
            Self::TapForMana { what, by } => sylex_core::EventFilter::TapForMana {
                what: what.lower(),
                by: by.lower(),
            },
            Self::RollPlanarDie { by, face } => sylex_core::EventFilter::RollPlanarDie {
                by: by.lower(),
                face: face.lower(),
            },
            Self::AllOf(f0) => sylex_core::EventFilter::AllOf(f0.lower()),
            Self::OneOf(f0) => sylex_core::EventFilter::OneOf(f0.lower()),
            Self::Not(f0) => sylex_core::EventFilter::Not(f0.lower()),
            Self::OneOrMore(f0) => sylex_core::EventFilter::OneOrMore(f0.lower()),
            Self::Nth { n, of, within } => sylex_core::EventFilter::Nth {
                n: n.lower(),
                of: of.lower(),
                within: within.lower(),
            },
            Self::When(f0, f1) => sylex_core::EventFilter::When(f0.lower(), f1.lower()),
            Self::Within(f0, f1) => sylex_core::EventFilter::Within(f0.lower(), f1.lower()),
            Self::Before(f0) => sylex_core::EventFilter::Before(f0.lower()),
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
    fn lowers_phase_kind_beginning() {
        assert_matches!(
            sylex_semantics_v1::PhaseKind::Beginning.lower(),
            sylex_core::PhaseKind::Beginning
        );
    }

    #[test]
    fn lowers_phase_kind_precombat_main() {
        assert_matches!(
            sylex_semantics_v1::PhaseKind::PrecombatMain.lower(),
            sylex_core::PhaseKind::PrecombatMain
        );
    }

    #[test]
    fn lowers_phase_kind_combat() {
        assert_matches!(
            sylex_semantics_v1::PhaseKind::Combat.lower(),
            sylex_core::PhaseKind::Combat
        );
    }

    #[test]
    fn lowers_phase_kind_postcombat_main() {
        assert_matches!(
            sylex_semantics_v1::PhaseKind::PostcombatMain.lower(),
            sylex_core::PhaseKind::PostcombatMain
        );
    }

    #[test]
    fn lowers_phase_kind_ending() {
        assert_matches!(
            sylex_semantics_v1::PhaseKind::Ending.lower(),
            sylex_core::PhaseKind::Ending
        );
    }

    #[test]
    fn lowers_phase_step_beginning() {
        assert_matches!(
            sylex_semantics_v1::PhaseStep::Beginning(minimal_beginning_step()).lower(),
            sylex_core::PhaseStep::Beginning(sylex_core::BeginningStep::Untap)
        );
    }

    #[test]
    fn lowers_phase_step_precombat_main() {
        assert_matches!(
            sylex_semantics_v1::PhaseStep::PrecombatMain.lower(),
            sylex_core::PhaseStep::PrecombatMain
        );
    }

    #[test]
    fn lowers_phase_step_combat() {
        assert_matches!(
            sylex_semantics_v1::PhaseStep::Combat(minimal_combat_step()).lower(),
            sylex_core::PhaseStep::Combat(sylex_core::CombatStep::BeginningOfCombat)
        );
    }

    #[test]
    fn lowers_phase_step_postcombat_main() {
        assert_matches!(
            sylex_semantics_v1::PhaseStep::PostcombatMain.lower(),
            sylex_core::PhaseStep::PostcombatMain
        );
    }

    #[test]
    fn lowers_phase_step_ending() {
        assert_matches!(
            sylex_semantics_v1::PhaseStep::Ending(minimal_ending_step()).lower(),
            sylex_core::PhaseStep::Ending(sylex_core::EndingStep::End)
        );
    }

    #[test]
    fn lowers_beginning_step_untap() {
        assert_matches!(
            sylex_semantics_v1::BeginningStep::Untap.lower(),
            sylex_core::BeginningStep::Untap
        );
    }

    #[test]
    fn lowers_beginning_step_upkeep() {
        assert_matches!(
            sylex_semantics_v1::BeginningStep::Upkeep.lower(),
            sylex_core::BeginningStep::Upkeep
        );
    }

    #[test]
    fn lowers_beginning_step_draw() {
        assert_matches!(
            sylex_semantics_v1::BeginningStep::Draw.lower(),
            sylex_core::BeginningStep::Draw
        );
    }

    #[test]
    fn lowers_combat_step_beginning_of_combat() {
        assert_matches!(
            sylex_semantics_v1::CombatStep::BeginningOfCombat.lower(),
            sylex_core::CombatStep::BeginningOfCombat
        );
    }

    #[test]
    fn lowers_combat_step_declare_attackers() {
        assert_matches!(
            sylex_semantics_v1::CombatStep::DeclareAttackers.lower(),
            sylex_core::CombatStep::DeclareAttackers
        );
    }

    #[test]
    fn lowers_combat_step_declare_blockers() {
        assert_matches!(
            sylex_semantics_v1::CombatStep::DeclareBlockers.lower(),
            sylex_core::CombatStep::DeclareBlockers
        );
    }

    #[test]
    fn lowers_combat_step_first_combat_damage() {
        assert_matches!(
            sylex_semantics_v1::CombatStep::FirstCombatDamage.lower(),
            sylex_core::CombatStep::FirstCombatDamage
        );
    }

    #[test]
    fn lowers_combat_step_combat_damage() {
        assert_matches!(
            sylex_semantics_v1::CombatStep::CombatDamage.lower(),
            sylex_core::CombatStep::CombatDamage
        );
    }

    #[test]
    fn lowers_combat_step_end_of_combat() {
        assert_matches!(
            sylex_semantics_v1::CombatStep::EndOfCombat.lower(),
            sylex_core::CombatStep::EndOfCombat
        );
    }

    #[test]
    fn lowers_ending_step_end() {
        assert_matches!(
            sylex_semantics_v1::EndingStep::End.lower(),
            sylex_core::EndingStep::End
        );
    }

    #[test]
    fn lowers_ending_step_cleanup() {
        assert_matches!(
            sylex_semantics_v1::EndingStep::Cleanup.lower(),
            sylex_core::EndingStep::Cleanup
        );
    }

    #[test]
    fn lowers_whose_turn_your() {
        assert_matches!(
            sylex_semantics_v1::WhoseTurn::Your.lower(),
            sylex_core::WhoseTurn::Your
        );
    }

    #[test]
    fn lowers_whose_turn_each_players() {
        assert_matches!(
            sylex_semantics_v1::WhoseTurn::EachPlayers.lower(),
            sylex_core::WhoseTurn::EachPlayers
        );
    }

    #[test]
    fn lowers_whose_turn_an_opponents() {
        assert_matches!(
            sylex_semantics_v1::WhoseTurn::AnOpponents.lower(),
            sylex_core::WhoseTurn::AnOpponents
        );
    }

    #[test]
    fn lowers_state_change_tapped() {
        assert_matches!(
            sylex_semantics_v1::StateChange::Tapped.lower(),
            sylex_core::StateChange::Tapped
        );
    }

    #[test]
    fn lowers_state_change_untapped() {
        assert_matches!(
            sylex_semantics_v1::StateChange::Untapped.lower(),
            sylex_core::StateChange::Untapped
        );
    }

    #[test]
    fn lowers_state_change_phased() {
        assert_matches!(
            sylex_semantics_v1::StateChange::Phased(minimal_phasing()).lower(),
            sylex_core::StateChange::Phased(sylex_core::Phasing::In)
        );
    }

    #[test]
    fn lowers_state_change_turned_face() {
        assert_matches!(
            sylex_semantics_v1::StateChange::TurnedFace(minimal_face()).lower(),
            sylex_core::StateChange::TurnedFace(sylex_core::Face::Up)
        );
    }

    #[test]
    fn lowers_state_change_transformed() {
        assert_matches!(
            sylex_semantics_v1::StateChange::Transformed.lower(),
            sylex_core::StateChange::Transformed
        );
    }

    #[test]
    fn lowers_agency_cost_payment() {
        assert_matches!(
            sylex_semantics_v1::Agency::CostPayment.lower(),
            sylex_core::Agency::CostPayment
        );
    }

    #[test]
    fn lowers_agency_attack_declaration() {
        assert_matches!(
            sylex_semantics_v1::Agency::AttackDeclaration.lower(),
            sylex_core::Agency::AttackDeclaration
        );
    }

    #[test]
    fn lowers_agency_effect_instruction() {
        assert_matches!(
            sylex_semantics_v1::Agency::EffectInstruction.lower(),
            sylex_core::Agency::EffectInstruction
        );
    }

    #[test]
    fn lowers_agency_turn_based_action() {
        assert_matches!(
            sylex_semantics_v1::Agency::TurnBasedAction.lower(),
            sylex_core::Agency::TurnBasedAction
        );
    }

    #[test]
    fn lowers_agency_state_based_action() {
        assert_matches!(
            sylex_semantics_v1::Agency::StateBasedAction.lower(),
            sylex_core::Agency::StateBasedAction
        );
    }

    #[test]
    fn lowers_agency_mana_ability_resolution() {
        assert_matches!(
            sylex_semantics_v1::Agency::ManaAbilityResolution.lower(),
            sylex_core::Agency::ManaAbilityResolution
        );
    }

    #[test]
    fn lowers_agency_special_action() {
        assert_matches!(
            sylex_semantics_v1::Agency::SpecialAction.lower(),
            sylex_core::Agency::SpecialAction
        );
    }

    #[test]
    fn lowers_verb_name() {
        assert_matches!(
            sylex_semantics_v1::VerbName("X".into()).lower(),
            sylex_core::VerbName(_)
        );
    }

    #[test]
    fn lowers_cause_pattern() {
        assert_matches!(
            sylex_semantics_v1::CausePattern {
                verb: None,
                agency: None,
                agent: None
            }
            .lower(),
            sylex_core::CausePattern {
                verb: None,
                agency: None,
                agent: None
            }
        );
    }

    #[test]
    fn lowers_cause_cause() {
        assert_matches!(
            sylex_semantics_v1::Cause::Cause(minimal_cause_pattern()).lower(),
            sylex_core::Cause::Cause(sylex_core::CausePattern {
                verb: None,
                agency: None,
                agent: None
            })
        );
    }

    #[test]
    fn lowers_event_filter_zone_change() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::ZoneChange {
                what: minimal_predicate(),
                from: None,
                to: None,
                cause: None
            }
            .lower(),
            sylex_core::EventFilter::ZoneChange {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_damage() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Damage {
                source: minimal_predicate(),
                to: minimal_predicate(),
                combat: None,
                amount: None
            }
            .lower(),
            sylex_core::EventFilter::Damage {
                source: sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                ),
                to: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                combat: None,
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_life_gained() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::LifeGained {
                who: minimal_predicate(),
                amount: None
            }
            .lower(),
            sylex_core::EventFilter::LifeGained {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_life_lost() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::LifeLost {
                who: minimal_predicate(),
                amount: None
            }
            .lower(),
            sylex_core::EventFilter::LifeLost {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_drawn() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Drawn {
                who: minimal_predicate(),
                amount: None
            }
            .lower(),
            sylex_core::EventFilter::Drawn {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_act() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Act {
                verb: minimal_verb_name(),
                who: minimal_predicate(),
                on: minimal_predicate(),
                cause: None
            }
            .lower(),
            sylex_core::EventFilter::Act {
                verb: sylex_core::VerbName(_),
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_counter_placed() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::CounterPlaced {
                kind: None,
                on: minimal_predicate(),
                amount: None,
                cause: None
            }
            .lower(),
            sylex_core::EventFilter::CounterPlaced {
                kind: None,
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                amount: None,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_counter_removed() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::CounterRemoved {
                kind: None,
                on: minimal_predicate(),
                amount: None,
                cause: None
            }
            .lower(),
            sylex_core::EventFilter::CounterRemoved {
                kind: None,
                on: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                amount: None,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_cast() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Cast {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::Cast {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_copied() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Copied {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::Copied {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_played() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Played {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::Played {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_activated_ab() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::ActivatedAb {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::ActivatedAb {
                who: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_attack_declared() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::AttackDeclared {
                by: minimal_predicate(),
                against: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::AttackDeclared {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                against: sylex_core::Predicate::Class(
                    sylex_core::ObjectClass::AbilityOnStack
                )
            }
        );
    }

    #[test]
    fn lowers_event_filter_block_declared() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::BlockDeclared {
                by: minimal_predicate(),
                of: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::BlockDeclared {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                of: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_attached() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Attached {
                what: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::Attached {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                to: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_state_became() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::StateBecame {
                of: minimal_predicate(),
                becomes: minimal_state_change(),
                cause: None
            }
            .lower(),
            sylex_core::EventFilter::StateBecame {
                of: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                becomes: sylex_core::StateChange::Tapped,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_becomes_target() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::BecomesTarget {
                what: minimal_predicate(),
                by: minimal_predicate(),
                source: None
            }
            .lower(),
            sylex_core::EventFilter::BecomesTarget {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                source: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_step_begins() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::StepBegins {
                at: minimal_phase_step(),
                whose: minimal_whose_turn()
            }
            .lower(),
            sylex_core::EventFilter::StepBegins {
                at: sylex_core::PhaseStep::Beginning(sylex_core::BeginningStep::Untap),
                whose: sylex_core::WhoseTurn::Your
            }
        );
    }

    #[test]
    fn lowers_event_filter_control_changed() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::ControlChanged {
                of: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::ControlChanged {
                of: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                to: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_designation_changed() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::DesignationChanged {
                name: "X".into(),
                of: minimal_predicate(),
                to: Some("Y".into()),
            }
            .lower(),
            sylex_core::EventFilter::DesignationChanged {
                name,
                of: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                to: Some(value),
            } if name.as_ref() == "X" && value.as_ref() == "Y"
        );
    }

    #[test]
    fn lowers_event_filter_token_created() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::TokenCreated {
                what: minimal_predicate(),
                by: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::TokenCreated {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_shuffled() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Shuffled {
                by: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::Shuffled {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_revealed() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Revealed {
                what: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::Revealed {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_used() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Used {
                of: minimal_reference()
            }
            .lower(),
            sylex_core::EventFilter::Used {
                of: sylex_core::Reference::Reg(sylex_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_event_filter_coin_flipped() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::CoinFlipped {
                by: minimal_predicate(),
                won: None
            }
            .lower(),
            sylex_core::EventFilter::CoinFlipped {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                won: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_dice_rolled() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::DiceRolled {
                by: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::DiceRolled {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_tap_for_mana() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::TapForMana {
                what: minimal_predicate(),
                by: minimal_predicate()
            }
            .lower(),
            sylex_core::EventFilter::TapForMana {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_roll_planar_die() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::RollPlanarDie {
                by: minimal_predicate(),
                face: None
            }
            .lower(),
            sylex_core::EventFilter::RollPlanarDie {
                by: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                face: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_all_of() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::AllOf([].into()).lower(),
            sylex_core::EventFilter::AllOf(_)
        );
    }

    #[test]
    fn lowers_event_filter_one_of() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::OneOf([].into()).lower(),
            sylex_core::EventFilter::OneOf(_)
        );
    }

    #[test]
    fn lowers_event_filter_not() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Not(std::sync::Arc::new(minimal_event_filter()))
                .lower(),
            sylex_core::EventFilter::Not(_)
        );
    }

    #[test]
    fn lowers_event_filter_one_or_more() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::OneOrMore(
                std::sync::Arc::new(minimal_event_filter())
            )
            .lower(),
            sylex_core::EventFilter::OneOrMore(_)
        );
    }

    #[test]
    fn lowers_event_filter_nth() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Nth {
                n: 0,
                of: std::sync::Arc::new(minimal_event_filter()),
                within: minimal_lookback()
            }
            .lower(),
            sylex_core::EventFilter::Nth {
                n: 0,
                of: _,
                within: sylex_core::Lookback::ThisTurn
            }
        );
    }

    #[test]
    fn lowers_event_filter_when() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::When(
                std::sync::Arc::new(minimal_event_filter()),
                std::sync::Arc::new(minimal_condition())
            )
            .lower(),
            sylex_core::EventFilter::When(_, _)
        );
    }

    #[test]
    fn lowers_event_filter_within() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Within(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            sylex_core::EventFilter::Within(_, sylex_core::Lookback::ThisTurn)
        );
    }

    #[test]
    fn lowers_event_filter_before() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Before(minimal_reference()).lower(),
            sylex_core::EventFilter::Before(sylex_core::Reference::Reg(
                sylex_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_event_filter_expanded() {
        assert_matches!(
            sylex_semantics_v1::EventFilter::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_event_filter())
            })
            .lower(),
            sylex_core::EventFilter::ZoneChange {
                what: sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
    }
}
